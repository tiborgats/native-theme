//! Range (spec §10.4's palette table): `Role::Slider`, `Role::ProgressBar`, `Role::Spinner`.

use native_theme_egui::{Role, RoleVariant, ThemeAtlas};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    _atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let normal = RoleVariant::Normal;
    let range = 0.0..=100.0;

    caption(reg, ui, "Slider (Role::Slider)");
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            demo::scoped(reg, ui, Role::Slider, normal, "Slider (horizontal)", |ui| {
                ui.add(egui::Slider::new(&mut state.slider, range.clone()).show_value(false))
            });
            reg.amend_last(|i| i.notes.push("range 0 to 100".to_string()));
            demo::scoped(
                reg,
                ui,
                Role::Slider,
                normal,
                "Slider (with its value)",
                |ui| ui.add(egui::Slider::new(&mut state.slider, range.clone()).text("value")),
            );
            reg.amend_last(|i| i.notes.push("range 0 to 100".to_string()));
            demo::scoped(
                reg,
                ui,
                Role::Slider,
                normal,
                "Slider (trailing fill)",
                |ui| {
                    ui.add(egui::Slider::new(&mut state.slider, range.clone()).trailing_fill(true))
                },
            );
            let mut disabled = state.slider;
            demo::scoped(
                reg,
                ui,
                Role::Slider,
                RoleVariant::Disabled,
                "Slider (disabled)",
                |ui| ui.add_enabled(false, egui::Slider::new(&mut disabled, range.clone())),
            );
        });
        demo::scoped(reg, ui, Role::Slider, normal, "Slider (vertical)", |ui| {
            ui.add(egui::Slider::new(&mut state.vertical, range.clone()).vertical())
        });
    });

    caption(reg, ui, "ProgressBar (Role::ProgressBar)");
    demo::scoped(
        reg,
        ui,
        Role::ProgressBar,
        normal,
        "ProgressBar (with text)",
        |ui| ui.add(egui::ProgressBar::new(state.progress).show_percentage()),
    );
    demo::scoped(
        reg,
        ui,
        Role::ProgressBar,
        normal,
        "ProgressBar (animated)",
        |ui| ui.add(egui::ProgressBar::new(state.progress).animate(true)),
    );

    caption(reg, ui, "Spinner (Role::Spinner)");
    ui.horizontal(|ui| {
        demo::scoped(reg, ui, Role::Spinner, normal, "Spinner", |ui| {
            ui.add(egui::Spinner::new())
        });
        demo::scoped(reg, ui, Role::Spinner, normal, "ui.spinner", |ui| {
            ui.spinner()
        });
    });
}
