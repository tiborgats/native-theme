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
use super::layer::Layer;
use super::presets::PresetId;
use super::resolve::ReaderState;
use super::selector::WidgetRef;
pub(crate) use checks::{TypeTarget, type_env};
pub(crate) use stack::Stacked;

/// What a layer is to the stack (S8, S9, S14): which checks apply to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LayerKind {
    /// A preset's own layer: its bases state whole blocks.
    Preset,
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

/// Every structural problem `layer` has against `cx` (S12).
pub(crate) fn check(layer: &Layer, cx: &Context<'_>) -> Result<(), LayerError> {
    let mut problems = checks::layer(layer, cx);
    let mut layers = cx.below.to_vec();
    layers.push(Stacked {
        layer,
        kind: cx.kind,
    });
    add(&mut problems, combinations(&layers, *cx));
    if problems.is_empty() {
        Ok(())
    } else {
        Err(LayerError::Invalid(problems))
    }
}

/// Every problem the checks over combinations find in `layers`, the last
/// the layer `cx` adds (S12), each with whether it names a place of that
/// layer.
fn combinations(layers: &[Stacked<'_>], cx: Context<'_>) -> Vec<(bool, Problem)> {
    let stack = stack::Stack::new(layers, cx);
    let added = layers.len().saturating_sub(1);
    let mut problems: Vec<(bool, Problem)> = combos::check(&stack)
        .into_iter()
        .map(|(layer, problem)| (layer == added, problem))
        .collect();
    if cx.kind == LayerKind::Palette && combos::on_kde_breeze(cx.preset) {
        let palette = combos::mapping_palette(&stack, added);
        problems.extend(palette.into_iter().map(|problem| (true, problem)));
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
/// nothing uses.
pub(crate) fn check_preset(
    declarations: &Declarations,
    layer: &Layer,
    preset: Option<PresetId>,
) -> Result<(), LayerError> {
    check_preset_exempt(declarations, layer, preset, &[])
}

/// [`check_preset`], the listed paths `exempt` (each as
/// [`listed_path`](crate::next::toml::read::listed_path) gives it) left out
/// of its completeness check: the synthetic route's preset, which the
/// layers over it complete ([`listed_unstated`]; S17, ADD-A128).
pub(crate) fn check_preset_exempt(
    declarations: &Declarations,
    layer: &Layer,
    preset: Option<PresetId>,
    exempt: &[String],
) -> Result<(), LayerError> {
    let cx = Context {
        declarations,
        preset,
        kind: LayerKind::Preset,
        reader_accent_left_out: false,
        below: &[],
    };
    let mut problems: Vec<Problem> = checks::layer_exempt(layer, &cx, exempt);
    problems.extend(checks::declarations(declarations));
    problems.extend(checks::unused(declarations, layer, preset));
    let layers = [Stacked {
        layer,
        kind: LayerKind::Preset,
    }];
    add(&mut problems, combinations(&layers, cx));
    if problems.is_empty() {
        Ok(())
    } else {
        Err(LayerError::Invalid(problems))
    }
}

/// Whether `layer`'s bases state the listed path `listed` (as
/// [`listed_path`](crate::next::toml::read::listed_path) gives it), or a
/// value holding it whole (a reference, `Absent`): a preset's own layer
/// stating it lists a path it states (S17, ADD-A128).
pub(crate) fn states(layer: &Layer, listed: &str) -> bool {
    checks::states(layer, listed)
}

/// Each of a preset's listed paths `pending` (as its `[pending]` table
/// spells them) that none of `layers` states, as S12's missing field: the
/// synthetic route's completeness (S17, ADD-A128).
pub(crate) fn listed_unstated(pending: &[(String, String)], layers: &[Layer]) -> Vec<Problem> {
    pending
        .iter()
        .filter(|(path, _)| {
            let listed = crate::next::toml::read::listed_path(path);
            !layers.iter().any(|layer| checks::states(layer, &listed))
        })
        .map(|(path, _)| Problem::new(path, ProblemKind::Missing))
        .collect()
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
