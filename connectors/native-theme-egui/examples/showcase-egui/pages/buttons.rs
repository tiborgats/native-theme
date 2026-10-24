//! Buttons (spec §10.4's palette table): `Role::Button`, the base style's selectables,
//! `AtomLayout`, and the companion crate's segmented control and switch.

use egui::Button;
use native_theme::icons::IconRole;
use native_theme::theme::IconSet;
use native_theme_egui::icons::{IconContext, icon_size};
use native_theme_egui::{Role, RoleVariant, ThemeAtlas};
use native_theme_egui_widgets::segmented_control::SegmentedControl;
use native_theme_egui_widgets::switch::Switch;

use super::{DemoState, button_min_size, caption};
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
    let min = button_min_size(t);

    caption(reg, ui, "Button (Role::Button)");
    ui.horizontal_wrapped(|ui| {
        demo::scoped(reg, ui, Role::Button, normal, "button (enabled)", |ui| {
            ui.add(Button::new("Button").min_size(min))
        });
        let image = demo::role_image(ui, IconRole::ActionSave, *set, icon_theme.as_deref(), size);
        demo::scoped(
            reg,
            ui,
            Role::Button,
            normal,
            "button (icon and text)",
            |ui| match image {
                Some(image) => ui.add(Button::new((image, "Save")).min_size(min)),
                None => ui.add(Button::new("Save (icon not in this set)").min_size(min)),
            },
        );
        demo::scoped(reg, ui, Role::Button, normal, "button (small)", |ui| {
            ui.add(Button::new("Small").small())
        });
        // egui's own small size, as the gpui showcase's small button keeps gpui-component's.
        reg.note_last(
            "Button::small",
            "egui's own size, its label's with egui's horizontal button padding and no \
             vertical padding (egui 0.36.2 src/widgets/button.rs:306-309, :339-342); \
             button.min_width and button.min_height are not applied",
        );
        demo::scoped(reg, ui, Role::Button, normal, "button (frameless)", |ui| {
            ui.add(Button::new("Frameless").frame(false).min_size(min))
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
            |ui| ui.add(Button::new("Save").shortcut_text(shortcut).min_size(min)),
        );
        demo::scoped(
            reg,
            ui,
            Role::Button,
            normal,
            "button (suggested action)",
            |ui| ui.add(Button::new("Suggested").selected(true).min_size(min)),
        );
        demo::scoped(
            reg,
            ui,
            Role::Button,
            RoleVariant::Disabled,
            "button (disabled)",
            |ui| ui.add_enabled(false, Button::new("Button").min_size(min)),
        );
    });
    ui.horizontal_wrapped(|ui| {
        // egui's own kinds, at egui's own size: `Ui::button`, `Ui::small_button` and
        // `Ui::toggle_value` take no `min_size` (egui 0.36.2 src/ui.rs:1848-1850, :1858-1860,
        // :1875-1882).
        demo::scoped(reg, ui, Role::Button, normal, "ui.button", |ui| {
            ui.button("ui.button")
        });
        reg.note_last(
            "ui.button",
            "egui's own width, its label's and padding; button.min_width is not applied",
        );
        demo::scoped(reg, ui, Role::Button, normal, "ui.small_button", |ui| {
            ui.small_button("ui.small_button")
        });
        reg.note_last(
            "ui.small_button",
            "egui's own size, its label's with egui's horizontal button padding and no \
             vertical padding; button.min_width and button.min_height are not applied",
        );
        demo::scoped(reg, ui, Role::Button, normal, "ui.toggle_value", |ui| {
            ui.toggle_value(&mut state.toggle, "ui.toggle_value")
        });
        reg.note_last(
            "ui.toggle_value",
            "egui's own width, its label's and padding; button.min_width is not applied",
        );
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

    // The companion crate's segmented control (docs/archive/todo_egui-widgets-spec.md §4.4).
    caption(reg, ui, "Segmented control (Role::SegmentedControl)");
    demo::widget(
        reg,
        ui,
        Role::SegmentedControl,
        normal,
        "segmented control",
        |ui| {
            ui.add(SegmentedControl::new(
                &mut state.segment,
                ["Day", "Week", "Month"],
            ))
        },
    );

    caption(reg, ui, "AtomLayout (the base style)");
    demo::base(reg, ui, "AtomLayout", |ui| {
        ui.add(egui::AtomLayout::new(("AtomLayout", "of two atoms")))
    });

    // The companion crate's switch (docs/archive/todo_egui-widgets-spec.md §4.1).
    caption(reg, ui, "Switch (Role::Switch)");
    ui.horizontal_wrapped(|ui| {
        // Held off, as the disabled one is held on: each shows one state; the next one toggles.
        demo::widget(reg, ui, Role::Switch, normal, "switch (off)", |ui| {
            ui.add(Switch::new(&mut false).label("Off"))
        });
        demo::widget(reg, ui, Role::Switch, normal, "switch", |ui| {
            ui.add(Switch::new(&mut state.switch).label("Switch"))
        });
        demo::widget(
            reg,
            ui,
            Role::Switch,
            RoleVariant::Disabled,
            "switch (disabled)",
            |ui| ui.add(Switch::new(&mut true).label("Disabled").enabled(false)),
        );
    });
}
