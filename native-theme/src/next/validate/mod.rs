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
use super::error::{LayerError, Problem};
use super::layer::Layer;
use super::presets::PresetId;
pub(crate) use checks::{TypeTarget, type_env};
pub(crate) use stack::Stacked;

/// What a layer is to the stack (S8, S9, S14): which checks apply to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "S16's `Theme` adds layers of each kind (M1a task 12); an `allow`, as Rust 1.88 (the MSRV) counts what dead code under an `expect` uses as used, and stable does not"
    )
)]
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
}

/// Every structural problem `layer` has against `cx` (S12).
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "S16's `Theme::add_layer` validates each layer (M1a task 12); an `allow`, as Rust 1.88 (the MSRV) counts what dead code under an `expect` uses as used, and stable does not"
    )
)]
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
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "S16's `Theme` validates each bundled preset (M1a task 12); an `allow`, as Rust 1.88 (the MSRV) counts what dead code under an `expect` uses as used, and stable does not"
    )
)]
pub(crate) fn check_preset(
    declarations: &Declarations,
    layer: &Layer,
    preset: Option<PresetId>,
) -> Result<(), LayerError> {
    let cx = Context {
        declarations,
        preset,
        kind: LayerKind::Preset,
        below: &[],
    };
    let mut problems: Vec<Problem> = checks::layer(layer, &cx);
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
