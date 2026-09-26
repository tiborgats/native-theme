//! Selection (spec §10.4's palette table): `Role::Checkbox` for check boxes and radio
//! buttons, `Role::ComboBox` with its popup through `ComboBox::popup_style`.

use native_theme_egui::{Role, RoleVariant, ThemeAtlas};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

const FRUITS: [&str; 3] = ["Apple", "Banana", "Cherry"];

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    _atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let normal = RoleVariant::Normal;

    caption(reg, ui, "Checkbox (Role::Checkbox)");
    ui.horizontal_wrapped(|ui| {
        let mut unchecked = false;
        demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            normal,
            "checkbox (unchecked)",
            |ui| ui.add(egui::Checkbox::new(&mut unchecked, "Unchecked")),
        );
        let mut checked = true;
        demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            RoleVariant::Selected,
            "checkbox (checked)",
            |ui| ui.add(egui::Checkbox::new(&mut checked, "Checked")),
        );
        // The mixed mark is painted as a checked box's is, so it takes the checked look.
        demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            RoleVariant::Selected,
            "checkbox (indeterminate)",
            |ui| {
                ui.add(
                    egui::Checkbox::new(&mut state.indeterminate, "Indeterminate")
                        .indeterminate(true),
                )
            },
        );
        let mut disabled = true;
        demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            RoleVariant::Disabled,
            "checkbox (disabled)",
            |ui| ui.add_enabled(false, egui::Checkbox::new(&mut disabled, "Disabled")),
        );
        let variant = if state.checked {
            RoleVariant::Selected
        } else {
            normal
        };
        demo::scoped(reg, ui, Role::Checkbox, variant, "ui.checkbox", |ui| {
            ui.checkbox(&mut state.checked, "ui.checkbox")
        });
    });

    // A radio button takes no selected flag of its own either: the selected one is drawn in
    // `RoleVariant::Selected` (§4.4), picked from its state as `ui.checkbox`'s is.
    let variant = |selected: bool| {
        if selected {
            RoleVariant::Selected
        } else {
            normal
        }
    };
    caption(reg, ui, "Radio button (Role::Checkbox)");
    ui.horizontal_wrapped(|ui| {
        let selected = state.radio == 0;
        let r = demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            variant(selected),
            "RadioButton",
            |ui| ui.add(egui::RadioButton::new(selected, "RadioButton")),
        );
        if r.clicked() {
            state.radio = 0;
        }
        let selected = state.radio == 1;
        let r = demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            variant(selected),
            "ui.radio",
            |ui| ui.radio(selected, "ui.radio"),
        );
        if r.clicked() {
            state.radio = 1;
        }
        demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            variant(state.radio == 2),
            "ui.radio_value",
            |ui| ui.radio_value(&mut state.radio, 2, "ui.radio_value"),
        );
    });

    caption(reg, ui, "ComboBox (Role::ComboBox)");
    ui.horizontal_wrapped(|ui| {
        let current = FRUITS.get(state.combo).copied().unwrap_or_default();
        demo::scoped_popup(
            reg,
            ui,
            Role::ComboBox,
            normal,
            "ComboBox",
            |ui, modifier| {
                let mut combo =
                    egui::ComboBox::from_id_salt("selection/combo").selected_text(current);
                if let Some(modifier) = modifier {
                    combo = combo.popup_style(modifier);
                }
                combo
                    .show_ui(ui, |ui| {
                        for (i, fruit) in FRUITS.into_iter().enumerate() {
                            ui.selectable_value(&mut state.combo, i, fruit);
                        }
                    })
                    .response
            },
        );
        demo::scoped_popup(
            reg,
            ui,
            Role::ComboBox,
            RoleVariant::Disabled,
            "ComboBox (disabled)",
            |ui, modifier| {
                let mut combo =
                    egui::ComboBox::from_id_salt("selection/combo-disabled").selected_text(current);
                if let Some(modifier) = modifier {
                    combo = combo.popup_style(modifier);
                }
                ui.add_enabled_ui(false, |ui| combo.show_ui(ui, |_| {}).response)
                    .inner
            },
        );
    });
}
