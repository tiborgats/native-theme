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
/// disabled, and its painter keeps the opacity it had.
///
/// Call it inside a role scope's `Disabled` cell, in place of `Ui::disable`, for a widget egui
/// paints from those cells (a `Checkbox`, a `RadioButton`). Where `disabled_alpha` is 1 the
/// platform dims by colour alone, and it is `Ui::disable`: the colours stay as stated.
pub fn fade_as_one(ui: &mut egui::Ui, ground: egui::Color32) {
    let alpha = ui.visuals().disabled_alpha();
    if alpha >= 1.0 {
        ui.disable();
        return;
    }
    let opacity = ui.opacity();
    let fade = |colour: egui::Color32| faded(ground, colour, alpha);
    let text = ui.visuals().override_text_color;
    let visuals = ui.visuals_mut();
    let widgets = &mut visuals.widgets;
    for cell in [&mut widgets.noninteractive, &mut widgets.inactive] {
        let fill = ground.blend(cell.bg_fill);
        cell.bg_fill = fade(fill);
        cell.bg_stroke.color = fade(fill.blend(cell.bg_stroke.color));
        cell.fg_stroke.color = fade(fill.blend(cell.fg_stroke.color));
    }
    visuals.override_text_color = text.map(|colour| fade(ground.blend(colour)));
    ui.disable();
    ui.set_opacity(opacity);
}

/// An opaque `colour` faded by `alpha` over `ground`: the pixel a control faded as one shows
/// where `colour` is.
pub(crate) fn faded(ground: egui::Color32, colour: egui::Color32, alpha: f32) -> egui::Color32 {
    ground.lerp_to_gamma(colour, alpha)
}
