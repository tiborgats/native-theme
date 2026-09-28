//! The Basic page: the controls the three showcases all draw, in the same
//! order, with the same labels, values and states, packed onto one screen,
//! so the gpui, iced and egui captures compare control by control. Each
//! control is built by the helper its own page builds it with.

use gpui::{Context, IntoElement, ParentElement, Styled, Window, div, prelude::*, px};
use gpui_component::{h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, ButtonKind, ButtonState, DemoButton, InputField};
use crate::support::{with_gap, with_padding};

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
        // The space between the groups and between the two columns, and round
        // the page: the theme's section gap and window margin, as the iced and
        // egui Basic pages lay them out.
        let section_gap = geometry::section_gap(&self.layout);
        let window_margin = geometry::window_margin(&self.layout);
        let on_radio = cx.listener(|this, ix: &usize, _w, _cx| {
            this.basic_radio = Some(*ix);
        });
        // A heading over one row of controls.
        let group = |heading: gpui::Stateful<gpui::Div>, row: gpui::Div| {
            with_gap(v_flex(), widget_gap).child(heading).child(row)
        };
        let row = || with_gap(h_flex(), widget_gap).items_center();
        let width = px(BASIC_WIDTH);

        let left = with_gap(v_flex(), section_gap)
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
                    // Built by the application, so the tooltip takes the
                    // platform's padding, radius and colours
                    // (`geometry::tooltip`); `Button::tooltip` builds its own.
                    .child(demo::built_tooltip_button(
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
                    .child(demo::body_label(ui, cx, "basic-body-text", "Body text"))
                    .child(demo::link(
                        ui,
                        cx,
                        "basic-link",
                        "Link",
                        "https://github.com/tiborgats/native-theme",
                    )),
            ));

        let right = with_gap(v_flex(), section_gap)
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
                        crate::BASIC_INPUT_DISABLED,
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

        with_padding(with_gap(h_flex(), section_gap), window_margin)
            .items_start()
            .flex_1()
            .child(left)
            .child(right)
    }
}
