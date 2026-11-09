//! Validation (S12): what adding a layer checks, every problem collected.
//!
//! The structural checks (`checks`) look at each value, patch, rule and
//! declaration on its own; the checks over combinations (`combos`) at what
//! each place can resolve to under the combinations of the rules' selectors,
//! over the stack flattened (`stack`).

mod checks;
mod combos;
#[cfg(test)]
mod combos_tests;
mod outcomes;
#[cfg(test)]
mod outcomes_tests;
mod stack;
#[cfg(test)]
mod tests;
pub(crate) mod walk;

use super::declare::Declarations;
use super::error::{LayerError, Problem, ProblemKind};
use super::inputs::Contrast;
use super::layer::{Bases, Layer};
use super::presets::PresetId;
use super::resolve::ReaderState;
use super::selector::{Scheme, WidgetRef};
use super::toml::read::Named;
pub(crate) use checks::{TypeTarget, type_env};
pub(crate) use combos::Reused;
pub(crate) use stack::Stacked;

/// What a layer is to the stack (S8, S9, S14): which checks apply to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LayerKind {
    /// A preset's own layer: its bases state whole blocks.
    Preset,
    /// The accent-derivation layer `Theme` derives directly above a preset
    /// that derives its accent-dependent roles (S8, S9.2; ADD-A132): its
    /// bases state those roles, no accent input.
    AccentDerivation,
    /// A reader's layer, or its stand-in (S9).
    Reader,
    /// A palette (S14).
    Palette,
    /// Any other application layer.
    Application,
    /// The accent layer `Theme` derives from the accent (S8, S9.2): its
    /// bases state the accent inputs.
    Accent,
}

/// What a layer is checked against: the current preset's declarations,
/// the preset (a `for_preset` part is validated while its preset is
/// current), and what the layer is.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Context<'a> {
    /// The current preset's `[declare]` (S10).
    pub(crate) declarations: &'a Declarations,
    /// The current preset; `None` while no preset is bundled.
    pub(crate) preset: Option<PresetId>,
    /// What the layer is.
    pub(crate) kind: LayerKind,
    /// The layers below it, bottom first: the stack the checks over
    /// combinations read with it (S8, S12).
    pub(crate) below: &'a [Stacked<'a>],
    /// The reader's common accent is left out of the stack (S8, S9.5), so
    /// the reader layer's `accent` input is not in force.
    pub(crate) reader_accent_left_out: bool,
}

/// Every problem the layers `added`, each of the kind `cx.kind`, have
/// against `cx` (S12), the layers added at once and validated with those
/// below them: in their order, each one's structural checks, then what the
/// checks over combinations find at the places it and the layers below it
/// state. Those checks run once, over `cx.below` and all of `added`, each
/// problem going to the layer stating the place it names. With one layer
/// added, that is its check over the layers below it; with several, it is
/// each one's check over the layers below it only where no layer's value
/// reads another's or is masked by it, as on the synthetic route, whose
/// layers `Stack::synthetic` refuses unless they state constant values at
/// disjoint places ([`synthetic_shape`]; S17, ADD-A131). The checks take
/// from `reused` what the check of the layers below found that the layers
/// leave as it was.
pub(crate) fn check(added: &[&Layer], cx: &Context<'_>, reused: &Reused) -> Result<(), LayerError> {
    let mut layers = cx.below.to_vec();
    let first = layers.len();
    layers.extend(added.iter().map(|layer| Stacked {
        layer,
        kind: cx.kind,
    }));
    let found = combinations(&layers, *cx, reused);
    let mut problems: Vec<Problem> = Vec::new();
    for (offset, layer) in added.iter().enumerate() {
        let at = first.saturating_add(offset);
        let mut own = checks::layer(layer, cx);
        add(
            &mut own,
            found
                .iter()
                .filter(|(stating, _)| *stating <= at)
                .map(|(stating, problem)| (*stating == at, problem.clone()))
                .collect(),
        );
        for problem in own {
            if !problems.contains(&problem) {
                problems.push(problem);
            }
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(LayerError::Invalid(problems))
    }
}

/// Every problem the checks over combinations find in `layers`, the last
/// the layer `cx` adds (S12), each with the layer stating the place it
/// names; what the check of the layers below found is taken from `reused`
/// where the added layer leaves it as it was.
fn combinations(layers: &[Stacked<'_>], cx: Context<'_>, reused: &Reused) -> Vec<(usize, Problem)> {
    let stack = stack::Stack::new(layers, cx);
    let added = layers.len().saturating_sub(1);
    let mut problems = combos::check(&stack, Some(reused));
    if cx.kind == LayerKind::Palette && combos::on_kde_breeze(cx.preset) {
        let palette = combos::mapping_palette(&stack, added);
        problems.extend(palette.into_iter().map(|problem| (added, problem)));
    }
    problems
}

/// Adds the problems the checks over combinations `found` at places the
/// structural checks, which check the added layer, found none at, or below:
/// one problem per fault, a place's own fault (an undeclared key, an
/// ill-typed expression) the one reported there; a place a layer below
/// states is never the added layer's.
fn add(problems: &mut Vec<Problem>, found: Vec<(bool, Problem)>) {
    let structural = problems.len();
    for (own, problem) in found {
        let at = &problem.path;
        let taken = own
            && problems.iter().take(structural).any(|held| {
                *at == held.path
                    || at
                        .strip_prefix(held.path.as_str())
                        .is_some_and(|rest| rest.starts_with('.') || rest.starts_with('['))
            });
        if !taken && !problems.contains(&problem) {
            problems.push(problem);
        }
    }
}

/// Every structural problem a preset's declarations and layer have (S10,
/// S12): its layer's, its declarations' own, and each declared keyed name
/// nothing uses. Its listed paths `pending` are left out of its
/// completeness check, as the layers above it state them
/// ([`listed_unstated`]; S17, ADD-A131); `synthetic`: the synthetic
/// route's layers over the preset, whose pending default sizes are waived
/// (S10's *A pending default*, ADD-A131) and whose bases state keyed
/// names as the preset's do (ADD-A133); `reused` keeps what its checks
/// over combinations found for the checks of the layers above it.
pub(crate) fn check_preset(
    declarations: &Declarations,
    layer: &Layer,
    preset: Option<PresetId>,
    pending: &[crate::toml::layer::Listed],
    synthetic: Option<&[Layer]>,
    reused: &Reused,
) -> Result<(), LayerError> {
    let exempt: Vec<String> = pending
        .iter()
        .map(|(path, _)| crate::toml::read::listed_path(path))
        .collect();
    let cx = Context {
        declarations,
        preset,
        kind: LayerKind::Preset,
        reader_accent_left_out: false,
        below: &[],
    };
    let mut problems: Vec<Problem> = checks::layer_exempt(layer, &cx, &exempt);
    problems.extend(checks::declarations(declarations, synthetic.is_some()));
    problems.extend(checks::unused(
        declarations,
        layer,
        preset,
        synthetic.unwrap_or_default(),
    ));
    let layers = [Stacked {
        layer,
        kind: LayerKind::Preset,
    }];
    // A listed path a redrawn part leaves unstated is the layers' over it
    // too (S6.3, S17).
    let found = combinations(&layers, cx, reused)
        .into_iter()
        .map(|(stating, problem)| (stating == 0, problem))
        .flat_map(|(own, problem)| {
            if problem.kind == ProblemKind::Missing {
                checks::unlisted(&problem.path, &exempt)
                    .into_iter()
                    .map(|at| (own, Problem::new(&at, ProblemKind::Missing)))
                    .collect()
            } else {
                vec![(own, problem)]
            }
        })
        .collect();
    add(&mut problems, found);
    if problems.is_empty() {
        Ok(())
    } else {
        Err(LayerError::Invalid(problems))
    }
}

/// Every problem keeping the synthetic route's layers `layers`, over the
/// preset `preset` and its declarations, from the one pass [`check`] makes
/// over them (S17, ADD-A131): a value one states as an expression
/// ([`ProblemKind::SyntheticExpression`]), and a place one states where an
/// earlier one states it, or a place above or below it, a list taken whole
/// ([`ProblemKind::SyntheticOverlap`]). Without either, no layer's value
/// reads another's or is masked by it, so the pass finds what each one's
/// check over the layers below it finds.
pub(crate) fn synthetic_shape(
    layers: &[Layer],
    declarations: &Declarations,
    preset: Option<PresetId>,
) -> Vec<Problem> {
    let stacked: Vec<Stacked<'_>> = layers
        .iter()
        .map(|layer| Stacked {
            layer,
            kind: LayerKind::Application,
        })
        .collect();
    let cx = Context {
        declarations,
        preset,
        kind: LayerKind::Application,
        reader_accent_left_out: false,
        below: &[],
    };
    let stack = stack::Stack::new(&stacked, cx);
    let mut problems = Vec::new();
    // Each place stated: its layer, what it states values of (`None`: an
    // input), the place with every list's items taken as the list, and
    // where the layer states it.
    let mut stated: Vec<(usize, Option<stack::Target>, String, String)> = Vec::new();
    for source in &stack.sources {
        for op in &source.ops {
            match &op.kind {
                stack::OpKind::Fields { .. } => continue,
                stack::OpKind::Set(stack::Val::Expr(_)) => {
                    problems.push(Problem::new(&op.at, ProblemKind::SyntheticExpression));
                }
                _ => {}
            }
            let place = whole_lists(&op.path);
            stated.push((source.layer, Some(source.target), place, op.at.clone()));
        }
    }
    for (index, layer) in layers.iter().enumerate() {
        for bases in applying_bases(layer, preset) {
            let inputs = &bases.inputs;
            for (name, value) in &inputs.entries {
                let per_scheme = Scheme::ALL.into_iter().map(|scheme| {
                    let at = format!("inputs.{name}.{}", scheme.name());
                    (at, value.per_scheme.get(scheme))
                });
                let per_contrast = value.per_contrast.iter().flat_map(|(contrast, schemes)| {
                    Scheme::ALL.into_iter().map(move |scheme| {
                        let at = format!(
                            "inputs.{name}.contrast.{}.{}",
                            contrast.name(),
                            scheme.name()
                        );
                        (at, schemes.get(scheme))
                    })
                });
                for (at, value) in per_scheme.chain(per_contrast) {
                    let Some(value) = value else {
                        continue;
                    };
                    if matches!(value, super::value::Value::Expr(_)) {
                        problems.push(Problem::new(&at, ProblemKind::SyntheticExpression));
                    }
                    stated.push((index, None, at.clone(), at));
                }
            }
            for scheme in Scheme::ALL {
                let lists = [
                    ("kde_groups", inputs.kde_groups.get(scheme).is_some()),
                    (
                        "kde_inactive_groups",
                        inputs.kde_inactive_groups.get(scheme).is_some(),
                    ),
                    (
                        "kde_unset_keys",
                        inputs.kde_unset_keys.get(scheme).is_some(),
                    ),
                ];
                for (list, _) in lists.into_iter().filter(|(_, held)| *held) {
                    let at = format!("inputs.{list}.{}", scheme.name());
                    stated.push((index, None, at.clone(), at));
                }
            }
        }
    }
    for (layer, target, place, at) in &stated {
        let shared = stated.iter().any(|(earlier, held_target, held, _)| {
            earlier < layer
                && held_target == target
                && (held == place || stack::is_above(held, place) || stack::is_above(place, held))
        });
        let problem = Problem::new(at, ProblemKind::SyntheticOverlap);
        if shared && !problems.contains(&problem) {
            problems.push(problem);
        }
    }
    problems
}

/// `path` with every list's item taken as the list: each `[…]` left out.
fn whole_lists(path: &str) -> String {
    let mut out = String::new();
    let mut depth = 0_usize;
    for c in path.chars() {
        match c {
            '[' => depth = depth.saturating_add(1),
            ']' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out
}

/// Whether `layer`'s bases state the listed path `listed` (as
/// [`listed_path`](crate::toml::read::listed_path) gives it), or a
/// value holding it whole (a reference, `Absent`), as [`checks::states`]
/// judges it: a listed input (`inputs.<name>.<scheme>`) by its per-scheme
/// value at that scheme, a listed presence (`widgets.<widget>.<part>`) by
/// the part stated `Absent` or drawn as another. A preset's own layer
/// stating it lists a path it states (S17, ADD-A128, ADD-A131, ADD-A132).
pub(crate) fn states(layer: &Layer, listed: &str) -> bool {
    checks::states(&layer.bases, layer, listed)
}

/// Each of a preset's listed paths `pending` (as its `[pending]` table
/// spells them) that no layer of `layers`, the stack bottom first, states,
/// as S12's missing field at the model's path: a path a layer states is
/// stated, on every theme, the preset's own bases and a layer's part for
/// `preset` included (S12, S17, ADD-A131). A listed input
/// (`inputs.<name>.<scheme>`) is a declared input, which
/// [`inputs_missing`] checks at every slot (ADD-A132). A listed presence counts as stated only
/// where the preset's or an application layer's bases state the part's
/// presence, and on the synthetic route (`synthetic`) is drawn, never
/// refused. A listed default size is no layer's to state: it is S12's
/// missing default size on every route but the synthetic one, which
/// waives the checks that read it ([`check_preset`]).
pub(crate) fn listed_unstated(
    pending: &[crate::toml::layer::Listed],
    layers: &[Stacked<'_>],
    preset: Option<PresetId>,
    synthetic: bool,
) -> Vec<Problem> {
    let mut problems = Vec::new();
    for (path, _) in pending {
        if crate::toml::read::listed_default(path).is_some()
            || crate::toml::read::listed_input(path).is_some()
        {
            continue;
        }
        let presence = crate::toml::read::listed_part(path).is_some();
        if presence && synthetic {
            continue;
        }
        let listed = crate::toml::read::listed_path(path);
        let stated = layers.iter().any(|stacked| {
            if presence && !matches!(stacked.kind, LayerKind::Preset | LayerKind::Application) {
                return false;
            }
            applying_bases(stacked.layer, preset)
                .any(|bases| checks::states(bases, stacked.layer, &listed))
        });
        if !stated {
            problems.push(Problem::new(&listed, ProblemKind::Missing));
        }
    }
    problems
}

/// Each declared input missing at a slot (S9, S12, ADD-A132): at a scheme
/// with no contrast or one, no layer of `layers`, the stack bottom first,
/// whose inputs apply there gives it a value, its contrast base for that
/// contrast at that scheme or its per-scheme value at that scheme (a
/// layer's common bases and its part for `preset`; the reader's `accent`
/// not counting where `reader_accent_left_out`, S8, S9.5). Its problem is
/// S12's missing field, one per scheme at which it is missing with no
/// contrast, at `inputs.<name>.<scheme>`, and, at a scheme where it has a
/// value with no contrast, one per contrast under which it is missing, at
/// `inputs.<name>.contrast.<contrast>.<scheme>` (S11's spelling): so no
/// reference evaluates an input to nothing at any scheme or contrast.
pub(crate) fn inputs_missing(
    declarations: &Declarations,
    layers: &[Stacked<'_>],
    preset: Option<PresetId>,
    reader_accent_left_out: bool,
) -> Vec<Problem> {
    let mut problems = Vec::new();
    for name in &declarations.inputs {
        let left_out = |stacked: &Stacked<'_>| {
            reader_accent_left_out
                && stacked.kind == LayerKind::Reader
                && crate::resolve::is_accent(name)
        };
        let given = |scheme: Scheme, contrast: Option<Contrast>| {
            layers
                .iter()
                .filter(|stacked| !left_out(stacked))
                .flat_map(|stacked| applying_bases(stacked.layer, preset))
                .flat_map(|bases| &bases.inputs.entries)
                .any(|(held, value)| {
                    held == name && crate::resolve::input_value(value, scheme, contrast).is_some()
                })
        };
        for scheme in Scheme::ALL {
            if !given(scheme, None) {
                let at = format!("inputs.{name}.{}", scheme.name());
                problems.push(Problem::new(&at, ProblemKind::Missing));
                continue;
            }
            // A layer giving a per-scheme value gives it under every
            // contrast too (`input_value` falls back to it), so this fires
            // only once a layer applies at a scheme but not under a contrast
            // (S8's palette skipped under a contrast it states no colours
            // for), which `given` must then leave out.
            for contrast in [Contrast::Increased, Contrast::High] {
                if !given(scheme, Some(contrast)) {
                    let at = format!(
                        "inputs.{name}.contrast.{}.{}",
                        contrast.name(),
                        scheme.name()
                    );
                    problems.push(Problem::new(&at, ProblemKind::Missing));
                }
            }
        }
    }
    problems
}

/// The bases of `layer` that apply under the preset `preset`, in their
/// order: its common bases, then its part for `preset` (S8, S11's
/// `[for_preset.<id>]`).
pub(crate) fn applying_bases(
    layer: &Layer,
    preset: Option<PresetId>,
) -> impl Iterator<Item = &Bases> {
    std::iter::once(&layer.bases).chain(
        layer
            .for_preset
            .iter()
            .filter(move |(held, _)| Some(*held) == preset)
            .map(|(_, bases)| bases),
    )
}

/// Every problem a reader's or stand-in's state has (S9.4, S12): a text
/// scaling factor, or a font DPI it reports, not finite and above zero; a
/// container named twice in `container_sizes`, which reports one size at a
/// time. Each path starts at the state.
pub(crate) fn reader_state(state: &ReaderState) -> Vec<Problem> {
    let mut problems = Vec::new();
    let factors = [
        ("text_scaling_factor", Some(state.text_scaling_factor)),
        ("font_dpi", state.font_dpi),
    ];
    for (at, factor) in factors {
        match factor {
            Some(n) if !n.is_finite() => problems.push(Problem::new(at, ProblemKind::NotFinite)),
            Some(n) if n <= 0.0 => problems.push(Problem::new(
                at,
                ProblemKind::OutOfRange {
                    expected: "above zero",
                },
            )),
            _ => {}
        }
    }
    let sizes = &state.container_sizes;
    for (index, (container, _)) in sizes.iter().enumerate() {
        if sizes
            .iter()
            .take(index)
            .any(|(earlier, _)| earlier == container)
        {
            let name = match container {
                WidgetRef::Typed(widget) => checks::widget_name(*widget).to_owned(),
                WidgetRef::Component(key) => key.to_string(),
            };
            let at = format!("container_sizes[{index}]");
            problems.push(Problem::new(&at, ProblemKind::ListedTwice { name }));
        }
    }
    problems
}
