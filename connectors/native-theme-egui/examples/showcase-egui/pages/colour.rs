//! Colour (spec §10.4's palette table): the eight colour-edit buttons, in the base style —
//! native-theme models no colour picker.

use native_theme_egui::ThemeAtlas;

use super::{DemoState, caption};
use crate::demo::{self, Registry};

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    _atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    caption(reg, ui, "Colour-edit buttons (the base style)");
    egui::Grid::new("colour/buttons").show(ui, |ui| {
        demo::base(reg, ui, "caption", |ui| ui.label("srgba"));
        demo::base(reg, ui, "ui.color_edit_button_srgba", |ui| {
            ui.color_edit_button_srgba(&mut state.colour)
        });
        ui.end_row();
        demo::base(reg, ui, "caption", |ui| ui.label("hsva"));
        demo::base(reg, ui, "ui.color_edit_button_hsva", |ui| {
            ui.color_edit_button_hsva(&mut state.hsva)
        });
        ui.end_row();
        demo::base(reg, ui, "caption", |ui| ui.label("srgb"));
        demo::base(reg, ui, "ui.color_edit_button_srgb", |ui| {
            ui.color_edit_button_srgb(&mut state.srgb)
        });
        ui.end_row();
        demo::base(reg, ui, "caption", |ui| ui.label("rgb"));
        demo::base(reg, ui, "ui.color_edit_button_rgb", |ui| {
            ui.color_edit_button_rgb(&mut state.rgb)
        });
        ui.end_row();
        demo::base(reg, ui, "caption", |ui| ui.label("srgba, premultiplied"));
        demo::base(reg, ui, "ui.color_edit_button_srgba_premultiplied", |ui| {
            ui.color_edit_button_srgba_premultiplied(&mut state.srgba_premultiplied)
        });
        ui.end_row();
        demo::base(reg, ui, "caption", |ui| ui.label("srgba, unmultiplied"));
        demo::base(reg, ui, "ui.color_edit_button_srgba_unmultiplied", |ui| {
            ui.color_edit_button_srgba_unmultiplied(&mut state.srgba_unmultiplied)
        });
        ui.end_row();
        demo::base(reg, ui, "caption", |ui| ui.label("rgba, premultiplied"));
        demo::base(reg, ui, "ui.color_edit_button_rgba_premultiplied", |ui| {
            ui.color_edit_button_rgba_premultiplied(&mut state.rgba_premultiplied)
        });
        ui.end_row();
        demo::base(reg, ui, "caption", |ui| ui.label("rgba, unmultiplied"));
        demo::base(reg, ui, "ui.color_edit_button_rgba_unmultiplied", |ui| {
            ui.color_edit_button_rgba_unmultiplied(&mut state.rgba_unmultiplied)
        });
        ui.end_row();
    });
}
