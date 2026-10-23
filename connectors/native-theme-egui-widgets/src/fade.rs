//! [`fade_as_one`]: a disabled control faded as one widget over what lies under it.

use native_theme_egui::egui;

/// Disables `ui` for a control faded as one widget over `ground`, the colour under it, as a
/// platform that dims by opacity fades it: libadwaita's `filter: Opacity(..)` on the whole
/// widget (`docs/platform-facts.md` §2.1.6), which keeps its colours, so its mark is drawn on
/// its fill and the pair fades over the page. egui fades each shape on its own (`Ui::disable`
/// multiplies the painter's opacity by `disabled_alpha`, `egui/src/ui.rs:497-503`), which would
/// fade a mark over the faded fill. Here each colour the scope's resting cells paint is
/// composed over what lies under it -- the fill over `ground`, the stroke and the mark over the
/// fill, the text over `ground` -- and faded by `disabled_alpha` over `ground`; `ui` is then
/// disabled, and its painter keeps the opacity it had. A widget of this crate added to a
/// disabled `Ui` is faded as one too, by that `Ui`'s `disabled_alpha`.
///
/// Call it inside a role scope's `Disabled` or `SelectedDisabled` cell, in place of
/// `Ui::disable`, for a widget egui paints from those cells (a `Checkbox`, a `RadioButton`).
/// Where `disabled_alpha` is 1 the platform dims by colour alone, and it is `Ui::disable`: the
/// colours stay as stated.
pub fn fade_as_one(ui: &mut egui::Ui, ground: egui::Color32) {
    let alpha = ui.visuals().disabled_alpha();
    if alpha >= 1.0 {
        ui.disable();
        return;
    }
    let opacity = ui.opacity();
    fade_cells(ui.visuals_mut(), ground, alpha);
    ui.disable();
    ui.set_opacity(opacity);
}

/// The fade a disabled calling `ui` gave its painter, read before a widget's scope opens:
/// `Ui::disable` multiplied the painter's opacity by `ui`'s `disabled_alpha`
/// (`egui/src/ui.rs:497-502`), as `Ui::add_enabled` does once (`:1588-1592`). Returns that
/// alpha and the opacity before it, which a widget faded as one paints at instead; `None`
/// where `ui` is enabled, or its alpha is not between 0 and 1, which leaves nothing to fade
/// as one. Only one level is undone: a `Ui` disabled twice (a nested
/// `add_enabled_ui(false, ..)`), or restyled after it was disabled, keeps the rest of its fade.
pub(crate) fn outer_fade(ui: &egui::Ui) -> Option<(f32, f32)> {
    let alpha = ui.visuals().disabled_alpha();
    (!ui.is_enabled() && alpha > 0.0 && alpha < 1.0).then(|| (alpha, ui.opacity() / alpha))
}

/// [`fade_as_one`] inside a scope whose calling `Ui` was disabled already, with its
/// [`outer_fade`]: the scope's cells faded by that `alpha`, which becomes the scope's
/// `disabled_alpha`, and the painter set back to `opacity`.
pub(crate) fn fade_as_one_under(
    ui: &mut egui::Ui,
    ground: egui::Color32,
    (alpha, opacity): (f32, f32),
) {
    let visuals = ui.visuals_mut();
    fade_cells(visuals, ground, alpha);
    visuals.disabled_alpha = alpha;
    ui.set_opacity(opacity);
}

/// The resting cells' colours composed over what lies under them and faded by `alpha` over
/// `ground` ([`fade_as_one`]).
fn fade_cells(visuals: &mut egui::Visuals, ground: egui::Color32, alpha: f32) {
    let fade = |colour: egui::Color32| faded(ground, colour, alpha);
    let widgets = &mut visuals.widgets;
    for cell in [&mut widgets.noninteractive, &mut widgets.inactive] {
        let fill = ground.blend(cell.bg_fill);
        cell.bg_fill = fade(fill);
        cell.bg_stroke.color = fade(fill.blend(cell.bg_stroke.color));
        cell.fg_stroke.color = fade(fill.blend(cell.fg_stroke.color));
    }
    visuals.override_text_color = visuals
        .override_text_color
        .map(|colour| fade(ground.blend(colour)));
}

/// An opaque `colour` faded by `alpha` over `ground`: the pixel a control faded as one shows
/// where `colour` is.
pub(crate) fn faded(ground: egui::Color32, colour: egui::Color32, alpha: f32) -> egui::Color32 {
    ground.lerp_to_gamma(colour, alpha)
}
