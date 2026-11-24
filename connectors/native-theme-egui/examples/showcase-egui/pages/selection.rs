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
        // The first four each show one state and are held in it: a click on them changes a
        // copy made for the pass. `ui.checkbox`, last, is the one that toggles.
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
        // The mixed box: the unchecked box with what the theme states for the mixed state
        // (`MixedLook`), not the checked look.
        demo::scoped(
            reg,
            ui,
            Role::Checkbox,
            normal,
            "checkbox (indeterminate)",
            |ui| {
                if let Some(look) = demo::next_value(ui, MixedLook::of) {
                    look.apply(ui.visuals_mut());
                }
                ui.add(
                    egui::Checkbox::new(&mut state.indeterminate, "Indeterminate")
                        .indeterminate(true),
                )
            },
        );
        reg.note_last("mixed state", MIXED_NOTE);
        // Unchecked and disabled, the `Disabled` cell; the Basic page draws a checked one in
        // `SelectedDisabled`.
        let mut disabled = false;
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
            |ui, modifier, row, reg| {
                let mut combo =
                    egui::ComboBox::from_id_salt("selection/combo").selected_text(current);
                if let Some(modifier) = modifier {
                    combo = combo.popup_style(modifier);
                }
                combo
                    .show_ui(ui, |ui| {
                        for (i, fruit) in FRUITS.into_iter().enumerate() {
                            row.add(reg, ui, "ComboBox row", |ui| {
                                ui.selectable_value(&mut state.combo, i, fruit)
                            });
                        }
                    })
                    .response
                    .accessible_name("ComboBox")
            },
        );
        demo::scoped_popup(
            reg,
            ui,
            Role::ComboBox,
            RoleVariant::Disabled,
            "ComboBox (disabled)",
            |ui, modifier, _, _| {
                let mut combo =
                    egui::ComboBox::from_id_salt("selection/combo-disabled").selected_text(current);
                if let Some(modifier) = modifier {
                    combo = combo.popup_style(modifier);
                }
                ui.add_enabled_ui(false, |ui| combo.show_ui(ui, |_| {}).response)
                    .inner
                    .accessible_name("ComboBox (disabled)")
            },
        );
    });
}

/// The *Not themeable* line of the mixed box: what it draws where the theme states nothing for
/// the mixed state.
pub(crate) const MIXED_NOTE: &str = "egui draws a mixed box as its unchecked one with a dash in \
     the mark's stroke (egui 0.37.0 src/widgets/checkbox.rs:135-141); the fill, outline and mark \
     the theme states for INDETERMINATE are laid over it per call (pages::selection::MixedLook), \
     each unstated one the unchecked box's";

/// What the installed 0.8.0 theme states for the mixed check box, per interaction entry: at
/// rest `checkbox.indicator(INDETERMINATE)` and `checkbox.checkmark(INDETERMINATE)`, under the
/// pointer and pressed the query with `HOVER` or `PRESSED` too; each property where a rule for a
/// flag of the query states it (spec §2, `from_state`), else `None`, and the entry keeps the
/// `Normal` cell's (spec §10, rule 0): egui draws a mixed box as its unchecked one, its visuals
/// picked by the interaction alone (egui 0.37.0 `src/theme/mod.rs:31-38`,
/// `src/theme/default_style.rs:176`), its frame from `bg_fill` and `bg_stroke` (`:193`, `:198`)
/// and its dash in `fg_stroke` (`src/widgets/checkbox.rs:135-141`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct MixedLook {
    /// The resting, the hovered and the pressed entry's.
    pub(crate) entries: [MixedEntry; 3],
}

/// One interaction entry of [`MixedLook`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct MixedEntry {
    pub(crate) fill: Option<egui::Color32>,
    pub(crate) stroke_width: Option<f32>,
    pub(crate) stroke_color: Option<egui::Color32>,
    pub(crate) mark: Option<egui::Color32>,
    pub(crate) mark_width: Option<f32>,
}

impl MixedLook {
    /// The mixed box's look on the 0.8.0 theme `r`.
    pub(crate) fn of(r: &native_theme::next::ResolvedTheme) -> Self {
        use native_theme::next::{IconStated, PartStated, WidgetState as S};
        use native_theme_egui::next;
        let entry = |state: S| {
            let q = state | S::INDETERMINATE;
            let part = r.checkbox.indicator(q);
            let mark = r.checkbox.checkmark(q);
            let stroke = part.from_state.contains(PartStated::STROKE);
            MixedEntry {
                fill: next::state_fill_of(&part),
                stroke_width: stroke.then(|| next::stroke_width_of(&part)).flatten(),
                stroke_color: next::state_stroke_color_of(&part),
                mark: mark
                    .from_state
                    .contains(IconStated::COLOR)
                    .then(|| next::to_color32(mark.color)),
                mark_width: mark
                    .from_state
                    .contains(IconStated::STROKE_WIDTH)
                    .then_some(mark.stroke_width)
                    .flatten(),
            }
        };
        Self {
            entries: [S::NONE, S::HOVER, S::PRESSED].map(entry),
        }
    }

    /// Lay the look over `v`, the mixed box's scope's visuals. Where it states a mark, the label
    /// is first held to the colour it has at rest, as the connector's checked-disabled cell
    /// holds it: egui letters a check box's label in the mark's `fg_stroke` where
    /// `override_text_color` is unset (egui 0.37.0 `src/theme/default_style.rs:182-186`,
    /// `src/widget_text.rs:831-835`).
    pub(crate) fn apply(&self, v: &mut egui::Visuals) {
        if v.override_text_color.is_none() && self.entries.iter().any(|e| e.mark.is_some()) {
            v.override_text_color = Some(v.widgets.inactive.fg_stroke.color);
        }
        let w = &mut v.widgets;
        for (entry, look) in [&mut w.inactive, &mut w.hovered, &mut w.active]
            .into_iter()
            .zip(self.entries)
        {
            if let Some(fill) = look.fill {
                entry.bg_fill = fill;
            }
            if let Some(width) = look.stroke_width {
                entry.bg_stroke.width = width;
            }
            if let Some(colour) = look.stroke_color {
                entry.bg_stroke.color = colour;
            }
            if let Some(mark) = look.mark {
                entry.fg_stroke.color = mark;
            }
            if let Some(width) = look.mark_width {
                entry.fg_stroke.width = width;
            }
        }
    }
}
