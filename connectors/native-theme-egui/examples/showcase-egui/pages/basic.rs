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
        ui.vertical(|ui| left_column(reg, state, atlas, ui));
        if let Some(gap) = section_gap {
            ui.add_space(gap);
        }
        ui.vertical(|ui| right_column(reg, state, atlas, ui));
    });
}

/// Buttons, check boxes, radio buttons, text.
fn left_column(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;
    // `button.min_width`, which egui's `Button` never reads from the style: it raises only its
    // height, to `interact_size.y` (`button.min_height` in the button scope), so the width is
    // the application's, per call (connector spec §5.3, `Button::min_size`).
    let button_min = egui::vec2(t.button.min_width, t.button.min_height);

    let read_min = |i: &mut demo::InstanceInfo| {
        i.read.push(("button.min_width", button_min.x.to_string()));
        i.read.push(("button.min_height", button_min.y.to_string()));
    };

    caption(reg, ui, "Buttons");
    ui.horizontal(|ui| {
        demo::scoped(reg, ui, Role::Button, normal, "button (enabled)", |ui| {
            ui.add(Button::new("Button").min_size(button_min))
        });
        reg.amend_last(read_min);
        demo::scoped(
            reg,
            ui,
            Role::Button,
            normal,
            "button (suggested action)",
            |ui| ui.add(Button::new("Primary").selected(true).min_size(button_min)),
        );
        reg.amend_last(read_min);
        demo::scoped(
            reg,
            ui,
            Role::Button,
            RoleVariant::Disabled,
            "button (disabled)",
            |ui| ui.add_enabled(false, Button::new("Disabled").min_size(button_min)),
        );
        reg.amend_last(read_min);
        let owner = demo::scoped(reg, ui, Role::Button, normal, "tooltip button", |ui| {
            ui.add(Button::new("Tooltip").min_size(button_min))
        });
        reg.amend_last(read_min);
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
        // `link.underline_enabled`, which egui's `Link` never reads: it underlines only on hover
        // or focus (`egui/src/widgets/hyperlink.rs:50-54`). An underline in the text's own format
        // is painted at rest, in the text's colour, so the text takes `link.font.color` too, the
        // `hyperlink_color` of the link scope (connector spec §5.3).
        let mut text = egui::RichText::new("Link")
            .color(native_theme_egui::convert::to_color32(t.link.font.color));
        if t.link.underline_enabled {
            text = text.underline();
        }
        demo::scoped(reg, ui, Role::Link, normal, "Link", |ui| {
            ui.add(egui::Link::new(text))
        });
        reg.amend_last(|i| {
            i.read.push((
                "link.underline_enabled",
                t.link.underline_enabled.to_string(),
            ));
            i.read
                .push(("link.font.color", format!("{:?}", t.link.font.color)));
        });
    });
}

/// A text field of the Basic page, built inside its `Role::Input` scope (`ui` is the scope's), so
/// its frame is the style it paints in: the disabled field takes the disabled cell's
/// `input.disabled_background`. `input.min_height` is a per-instance `min_size`, which a
/// `TextEdit` never reads from the style: it is one row plus its margin tall, at least its
/// `min_size` (connector spec §5.4); the row is centred in the height the minimum adds, as the
/// platform centres a field's text.
fn field<'a>(
    text: &'a mut String,
    id: egui::Id,
    ui: &egui::Ui,
    t: &native_theme_egui::ResolvedTheme,
) -> egui::TextEdit<'a> {
    egui::TextEdit::singleline(text)
        .desired_width(BASIC_WIDTH)
        .min_size(egui::vec2(BASIC_WIDTH, t.input.min_height))
        .vertical_align(egui::Align::Center)
        .id(id)
        .frame(input_frame(ui, id, t))
}

/// Text inputs, the drop-down, the slider, the progress bar.
fn right_column(reg: &mut Registry, state: &mut DemoState, atlas: &ThemeAtlas, ui: &mut egui::Ui) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    let read_min = |i: &mut demo::InstanceInfo| {
        i.read
            .push(("input.min_height", t.input.min_height.to_string()));
    };

    caption(reg, ui, "Text inputs");
    ui.horizontal(|ui| {
        let id = ui.make_persistent_id("basic/placeholder");
        demo::scoped(reg, ui, Role::Input, normal, "TextEdit (hint)", |ui| {
            let edit = field(&mut state.basic_hint, id, ui, t).hint_text("Placeholder");
            ui.add(edit)
        });
        reg.amend_last(|i| {
            read_min(i);
            i.notes.push(("hint text", "\"Placeholder\"".to_string()));
        });
        let id = ui.make_persistent_id("basic/filled");
        demo::scoped(
            reg,
            ui,
            Role::Input,
            normal,
            "TextEdit (single line)",
            |ui| {
                let edit = field(&mut state.basic_text, id, ui, t);
                ui.add(edit)
            },
        );
        reg.amend_last(read_min);
        let id = ui.make_persistent_id("basic/disabled");
        let mut disabled = "Disabled".to_string();
        demo::scoped(
            reg,
            ui,
            Role::Input,
            RoleVariant::Disabled,
            "TextEdit (disabled)",
            |ui| {
                let edit = field(&mut disabled, id, ui, t);
                ui.add_enabled(false, edit)
            },
        );
        reg.amend_last(read_min);
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
