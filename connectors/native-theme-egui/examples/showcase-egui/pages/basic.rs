//! Basic: the controls the three showcases all draw, in the same order, with the same labels,
//! values and states, packed onto one screen, so the gpui, iced and egui captures compare
//! control by control. Each control goes through the seam its palette page gives it.

use egui::Button;
use native_theme_egui::{Role, RoleVariant, Surface, ThemeAtlas, input_frame};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

/// The width of the Basic page's text fields, drop-down, slider and progress bar. The model
/// states no such width; it is the Basic page's own, the gpui showcase's and the iced
/// showcase's `BASIC_WIDTH` too, so the three pages lay the same controls out alike.
const BASIC_WIDTH: f32 = 140.0;

/// The drop-down's rows.
const FRUITS: [&str; 3] = ["Apple", "Banana", "Cherry"];

/// The progress bar's value: the datum on display, not a style value.
const PROGRESS: f32 = 0.4;

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let section_gap = atlas.layout().section_gap;
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| left_column(reg, state, ui));
        if let Some(gap) = section_gap {
            ui.add_space(gap);
        }
        ui.vertical(|ui| right_column(reg, state, atlas, ui));
    });
}

/// Buttons, check boxes, radio buttons, text.
fn left_column(reg: &mut Registry, state: &mut DemoState, ui: &mut egui::Ui) {
    let normal = RoleVariant::Normal;

    caption(reg, ui, "Buttons");
    ui.horizontal(|ui| {
        demo::scoped(reg, ui, Role::Button, normal, "button (enabled)", |ui| {
            ui.add(Button::new("Button"))
        });
        demo::scoped(
            reg,
            ui,
            Role::Button,
            normal,
            "button (suggested action)",
            |ui| ui.add(Button::new("Primary").selected(true)),
        );
        demo::scoped(
            reg,
            ui,
            Role::Button,
            RoleVariant::Disabled,
            "button (disabled)",
            |ui| ui.add_enabled(false, Button::new("Disabled")),
        );
        let owner = demo::scoped(reg, ui, Role::Button, normal, "tooltip button", |ui| {
            ui.add(Button::new("Tooltip"))
        });
        demo::surfaced(
            reg,
            ui,
            Surface::Tooltip,
            false,
            Some((Role::Tooltip, normal)),
            "tooltip",
            |_, chrome, reg| {
                let mut tip = egui::Tooltip::for_enabled(&owner);
                tip.popup = tip.popup.frame(chrome.frame);
                if let Some(modifier) = chrome.modifier {
                    tip.popup = tip.popup.style(modifier);
                }
                tip.show(|ui| {
                    demo::scoped(reg, ui, Role::Tooltip, normal, "tooltip text", |ui| {
                        ui.label("A tooltip")
                    });
                })
                .map(|out| out.response)
            },
        );
    });

    caption(reg, ui, "Checkboxes");
    ui.horizontal(|ui| {
        // Each shows one state and is held in it: a click changes a copy made for the pass.
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
        let mut disabled = true;
        demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            RoleVariant::Disabled,
            "checkbox (disabled)",
            |ui| ui.add_enabled(false, egui::Checkbox::new(&mut disabled, "Disabled")),
        );
    });

    // The selected radio button is drawn in `RoleVariant::Selected`, as on the Selection page.
    caption(reg, ui, "Radio buttons");
    ui.horizontal(|ui| {
        for (i, label) in ["Option A", "Option B"].into_iter().enumerate() {
            let selected = state.basic_radio == i;
            let variant = if selected {
                RoleVariant::Selected
            } else {
                normal
            };
            let r = demo::scoped(reg, ui, Role::Checkbox, variant, "RadioButton", |ui| {
                ui.add(egui::RadioButton::new(selected, label))
            });
            if r.clicked() {
                state.basic_radio = i;
            }
        }
    });

    caption(reg, ui, "Text");
    ui.horizontal(|ui| {
        demo::base(reg, ui, "Label (body text)", |ui| ui.label("Body text"));
        demo::scoped(reg, ui, Role::Link, normal, "Link", |ui| {
            ui.add(egui::Link::new("Link"))
        });
    });
}

/// Text inputs, the drop-down, the slider, the progress bar.
fn right_column(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    caption(reg, ui, "Text inputs");
    ui.horizontal(|ui| {
        let id = ui.make_persistent_id("basic/placeholder");
        let frame = input_frame(ui, id, t);
        demo::scoped(reg, ui, Role::Input, normal, "TextEdit (hint)", |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut state.basic_hint)
                    .hint_text("Placeholder")
                    .desired_width(BASIC_WIDTH)
                    .id(id)
                    .frame(frame),
            )
        });
        reg.amend_last(|i| i.notes.push(("hint text", "\"Placeholder\"".to_string())));
        let id = ui.make_persistent_id("basic/filled");
        let frame = input_frame(ui, id, t);
        demo::scoped(
            reg,
            ui,
            Role::Input,
            normal,
            "TextEdit (single line)",
            |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut state.basic_text)
                        .desired_width(BASIC_WIDTH)
                        .id(id)
                        .frame(frame),
                )
            },
        );
        let id = ui.make_persistent_id("basic/disabled");
        let frame = input_frame(ui, id, t);
        let mut disabled = "Disabled".to_string();
        demo::scoped(
            reg,
            ui,
            Role::Input,
            RoleVariant::Disabled,
            "TextEdit (disabled)",
            |ui| {
                ui.add_enabled(
                    false,
                    egui::TextEdit::singleline(&mut disabled)
                        .desired_width(BASIC_WIDTH)
                        .id(id)
                        .frame(frame),
                )
            },
        );
    });

    caption(reg, ui, "Drop-down");
    let current = FRUITS.get(state.basic_combo).copied().unwrap_or_default();
    demo::scoped_popup(
        reg,
        ui,
        Role::ComboBox,
        normal,
        "ComboBox",
        |ui, modifier, row, reg| {
            let mut combo = egui::ComboBox::from_id_salt("basic/combo")
                .selected_text(current)
                .width(BASIC_WIDTH);
            if let Some(modifier) = modifier {
                combo = combo.popup_style(modifier);
            }
            combo
                .show_ui(ui, |ui| {
                    for (i, fruit) in FRUITS.into_iter().enumerate() {
                        row.add(reg, ui, "ComboBox row", |ui| {
                            ui.selectable_value(&mut state.basic_combo, i, fruit)
                        });
                    }
                })
                .response
        },
    );

    caption(reg, ui, "Slider");
    demo::scoped(reg, ui, Role::Slider, normal, "Slider (horizontal)", |ui| {
        ui.spacing_mut().slider_width = BASIC_WIDTH;
        ui.add(egui::Slider::new(&mut state.basic_slider, 0.0..=100.0).show_value(false))
    });
    reg.amend_last(|i| i.notes.push(("range", "0 to 100".to_string())));

    caption(reg, ui, "Progress bar");
    demo::scoped(reg, ui, Role::ProgressBar, normal, "ProgressBar", |ui| {
        ui.add(egui::ProgressBar::new(PROGRESS).desired_width(BASIC_WIDTH))
    });
}
