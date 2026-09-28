//! The Basic page: the controls the three showcases all draw, in the same
//! order, with the same labels, values and states, packed onto one screen,
//! so the gpui, iced and egui captures compare control by control. Each
//! control is built by the helper its own page builds it with.

use gpui::{Context, IntoElement, ParentElement, Styled, Window, div, prelude::*, px};
use gpui_component::{h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, ButtonKind, ButtonState, DemoButton, InputField, LabelKind};
use crate::support::with_gap;

/// The width, in logical pixels, of the Basic page's text fields, drop-down,
/// slider and progress bar. The model states no such width; it is the Basic
/// page's own, the iced and egui showcases' `BASIC_WIDTH` too, so the three
/// pages lay the same controls out alike.
const BASIC_WIDTH: f32 = 140.0;

/// The progress bar's value, in percent: the datum on display.
const BASIC_PROGRESS: f32 = 40.0;

/// The buttons, as `(id, label, variant, state)`.
const BUTTONS: [(&str, &str, ButtonKind, ButtonState); 3] = [
    (
        "basic-button",
        "Button",
        ButtonKind::Default,
        ButtonState::Idle,
    ),
    (
        "basic-button-primary",
        "Primary",
        ButtonKind::Primary,
        ButtonState::Idle,
    ),
    (
        "basic-button-disabled",
        "Disabled",
        ButtonKind::Default,
        ButtonState::Disabled,
    ),
];

/// A checkbox held in the one state it shows: a click changes nothing.
fn held(_: &bool, _: &mut Window, _: &mut gpui::App) {}

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Basic
    // -----------------------------------------------------------------------
    pub(crate) fn render_basic_page(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        let widget_gap = geometry::widget_gap(&self.layout);
        let on_radio = cx.listener(|this, ix: &usize, _w, _cx| {
            this.basic_radio = Some(*ix);
        });
        // A heading over one row of controls.
        let group = |heading: gpui::Stateful<gpui::Div>, row: gpui::Div| {
            with_gap(v_flex(), widget_gap).child(heading).child(row)
        };
        let row = || with_gap(h_flex(), widget_gap).items_center();
        let width = px(BASIC_WIDTH);

        let left = v_flex()
            .gap_5()
            .child(group(
                demo::heading(ui, cx, "basic-heading-buttons", "Buttons"),
                row()
                    .children(BUTTONS.map(|(id, label, kind, state)| {
                        demo::button(
                            ui,
                            cx,
                            DemoButton {
                                id,
                                label,
                                kind,
                                state,
                                icon: None,
                            },
                        )
                    }))
                    .child(demo::tooltip_button(
                        ui,
                        cx,
                        "basic-button-tooltip",
                        "Tooltip",
                        "A tooltip",
                    )),
            ))
            .child(group(
                demo::heading(ui, cx, "basic-heading-checkboxes", "Checkboxes"),
                row()
                    .child(demo::checkbox(
                        ui,
                        cx,
                        "basic-checkbox-unchecked",
                        "Unchecked",
                        false,
                        Some(held),
                    ))
                    .child(demo::checkbox(
                        ui,
                        cx,
                        "basic-checkbox-checked",
                        "Checked",
                        true,
                        Some(held),
                    ))
                    .child(demo::checkbox(
                        ui,
                        cx,
                        "basic-checkbox-disabled",
                        "Disabled",
                        true,
                        None::<fn(&bool, &mut Window, &mut gpui::App)>,
                    )),
            ))
            .child(group(
                demo::heading(ui, cx, "basic-heading-radio", "Radio buttons"),
                row().child(demo::radio_group(
                    ui,
                    cx,
                    "basic-radio",
                    &["Option A", "Option B"],
                    self.basic_radio,
                    on_radio,
                )),
            ))
            .child(group(
                demo::heading(ui, cx, "basic-heading-text", "Text"),
                row()
                    .child(demo::gallery_label(
                        ui,
                        cx,
                        "basic-body-text",
                        LabelKind::Plain,
                        "Body text",
                    ))
                    .child(demo::link(
                        ui,
                        cx,
                        "basic-link",
                        "Link",
                        "https://github.com/tiborgats/native-theme",
                    )),
            ));

        let right = v_flex()
            .gap_5()
            .child(group(
                demo::heading(ui, cx, "basic-heading-inputs", "Text inputs"),
                row()
                    .child(demo::text_input(
                        ui,
                        cx,
                        "basic-input-placeholder",
                        &self.basic_hint_state,
                        InputField::Refined,
                        false,
                        width,
                    ))
                    .child(demo::text_input(
                        ui,
                        cx,
                        "basic-input-filled",
                        &self.basic_text_state,
                        InputField::Refined,
                        false,
                        width,
                    ))
                    .child(demo::text_input(
                        ui,
                        cx,
                        "basic-input-disabled",
                        &self.basic_disabled_state,
                        InputField::Refined,
                        true,
                        width,
                    )),
            ))
            .child(group(
                demo::heading(ui, cx, "basic-heading-select", "Drop-down"),
                row().child(demo::select(
                    ui,
                    cx,
                    "basic-select",
                    &self.basic_select,
                    "Pick a fruit",
                    width,
                )),
            ))
            .child(group(
                demo::heading(ui, cx, "basic-heading-slider", "Slider"),
                row().child(demo::slider(
                    ui,
                    cx,
                    "basic-slider",
                    &self.basic_slider_state,
                    width,
                )),
            ))
            .child(group(
                demo::heading(ui, cx, "basic-heading-progress", "Progress bar"),
                row().child(div().w(width).child(demo::progress(
                    ui,
                    cx,
                    "basic-progress",
                    "Progress bar",
                    BASIC_PROGRESS,
                ))),
            ));

        h_flex()
            .items_start()
            .gap_5()
            .p_4()
            .flex_1()
            .child(left)
            .child(right)
    }
}
