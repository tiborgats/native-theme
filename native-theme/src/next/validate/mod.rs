//! Validation (S12): what adding a layer checks, every problem collected.
//!
//! The structural checks (`checks`) look at each value, patch, rule and
//! declaration on its own; the checks over combinations of the rules'
//! conditions are S12's second half (the M1a design's task 9).

mod checks;
#[cfg(test)]
mod tests;
pub(crate) mod walk;

use super::declare::Declarations;
use super::error::{LayerError, Problem};
use super::layer::Layer;
use super::presets::PresetId;

/// What a layer is to the stack (S8, S9, S14): which checks apply to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "S16's `Theme` adds layers of each kind (M1a task 12)"
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
}

/// Every structural problem `layer` has against `cx` (S12).
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "S16's `Theme::add_layer` validates each layer (M1a task 12)"
    )
)]
pub(crate) fn check(layer: &Layer, cx: &Context<'_>) -> Result<(), LayerError> {
    let problems = checks::layer(layer, cx);
    if problems.is_empty() {
        Ok(())
    } else {
        Err(LayerError::Invalid(problems))
    }
}

/// Every structural problem a preset's declarations and layer have (S10,
/// S12): its layer's, its declarations' own, and each declared keyed name
/// nothing uses.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "S16's `Theme` validates each bundled preset (M1a task 12)"
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
    };
    let mut problems: Vec<Problem> = checks::layer(layer, &cx);
    problems.extend(checks::declarations(declarations));
    problems.extend(checks::unused(declarations, layer, preset));
    if problems.is_empty() {
        Ok(())
    } else {
        Err(LayerError::Invalid(problems))
    }
}
