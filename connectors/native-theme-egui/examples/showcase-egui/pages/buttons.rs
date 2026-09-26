//! Buttons (spec §10.4's palette table): `Role::Button`, the base style's selectables,
//! a segmented control, `AtomLayout` and the switch substitute.

use egui::Button;
use native_theme::theme::{IconRole, IconSet};
use native_theme_egui::icons::{IconContext, icon_size};
use native_theme_egui::{Role, RoleVariant, ThemeAtlas};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let (set, icon_theme) = chosen;
    let size = icon_size(t, IconContext::Small);
    let normal = RoleVariant::Normal;

    caption(reg, ui, "Button (Role::Button)");
    ui.horizontal_wrapped(|ui| {
        demo::scoped(reg, ui, Role::Button, normal, "button (enabled)", |ui| {
            ui.add(Button::new("Button"))
        });
        let image = demo::role_image(ui, IconRole::ActionSave, *set, icon_theme.as_deref(), size);
        demo::scoped(
            reg,
            ui,
            Role::Button,
            normal,
            "button (icon and text)",
            |ui| match image {
                Some(image) => ui.add(Button::new((image, "Save"))),
                None => ui.add(Button::new("Save (icon not in this set)")),
            },
        );
        demo::scoped(reg, ui, Role::Button, normal, "button (small)", |ui| {
            ui.add(Button::new("Small").small())
        });
        demo::scoped(reg, ui, Role::Button, normal, "button (frameless)", |ui| {
            ui.add(Button::new("Frameless").frame(false))
        });
        let shortcut = ui.ctx().format_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::S,
        ));
        demo::scoped(
            reg,
            ui,
            Role::Button,
            normal,
            "button (shortcut text)",
            |ui| ui.add(Button::new("Save").shortcut_text(shortcut)),
        );
        demo::scoped(
            reg,
            ui,
            Role::Button,
            normal,
            "button (suggested action)",
            |ui| ui.add(Button::new("Suggested").selected(true)),
        );
        demo::scoped(
            reg,
            ui,
            Role::Button,
            RoleVariant::Disabled,
            "button (disabled)",
            |ui| ui.add_enabled(false, Button::new("Button")),
        );
    });
    ui.horizontal_wrapped(|ui| {
        demo::scoped(reg, ui, Role::Button, normal, "ui.button", |ui| {
            ui.button("ui.button")
        });
        demo::scoped(reg, ui, Role::Button, normal, "ui.small_button", |ui| {
            ui.small_button("ui.small_button")
        });
        demo::scoped(reg, ui, Role::Button, normal, "ui.toggle_value", |ui| {
            ui.toggle_value(&mut state.toggle, "ui.toggle_value")
        });
    });

    caption(reg, ui, "Selectable (the base style)");
    ui.horizontal_wrapped(|ui| {
        let selected = state.selectable == 0;
        let r = demo::base(reg, ui, "Button::selectable", |ui| {
            ui.add(Button::selectable(selected, "Button::selectable"))
        });
        if r.clicked() {
            state.selectable = 0;
        }
        let selected = state.selectable == 1;
        let r = demo::base(reg, ui, "ui.selectable_label", |ui| {
            ui.selectable_label(selected, "ui.selectable_label")
        });
        if r.clicked() {
            state.selectable = 1;
        }
        demo::base(reg, ui, "ui.selectable_value", |ui| {
            ui.selectable_value(&mut state.selectable, 2, "ui.selectable_value")
        });
    });

    caption(reg, ui, "Segmented control (Role::SegmentedControl)");
    demo::scoped(
        reg,
        ui,
        Role::SegmentedControl,
        normal,
        "segmented control",
        |ui| {
            ui.horizontal(|ui| {
                for (i, label) in ["Day", "Week", "Month"].into_iter().enumerate() {
                    if ui
                        .add(Button::new(label).selected(i == state.segment))
                        .clicked()
                    {
                        state.segment = i;
                    }
                }
            })
            .response
        },
    );

    caption(reg, ui, "AtomLayout (the base style)");
    demo::base(reg, ui, "AtomLayout", |ui| {
        ui.add(egui::AtomLayout::new(("AtomLayout", "of two atoms")))
    });

    caption(reg, ui, "Switch (Role::Switch)");
    ui.horizontal_wrapped(|ui| {
        let mut off = false;
        demo::scoped(reg, ui, Role::Switch, normal, "switch (off)", |ui| {
            let r = ui.add(Button::new("Off").selected(off));
            if r.clicked() {
                off = !off;
            }
            r
        });
        demo::scoped(reg, ui, Role::Switch, normal, "switch", |ui| {
            let r =
                ui.add(Button::new(if state.switch { "On" } else { "Off" }).selected(state.switch));
            if r.clicked() {
                state.switch = !state.switch;
            }
            r
        });
        demo::scoped(
            reg,
            ui,
            Role::Switch,
            RoleVariant::Disabled,
            "switch (disabled)",
            |ui| ui.add_enabled(false, Button::new("Disabled").selected(true)),
        );
    });
}
