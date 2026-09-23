//! The Inputs page.

use gpui::{Context, IntoElement, ParentElement, SharedString, Styled, Window, prelude::*, px};
use gpui_component::{h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, InputField, InputGroupStates};
use crate::support::with_gap;
use crate::{
    INPUTS_CHECKBOX_AUTOSAVE, INPUTS_CHECKBOX_NOTIFICATIONS, INPUTS_FIELD,
    INPUTS_FIELD_HEIGHT_ONLY, PROBE_RATING, probe,
};

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Inputs
    // -----------------------------------------------------------------------
    pub(crate) fn render_inputs_page(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        let widget_gap = geometry::widget_gap(&self.layout);
        let on_checkbox_a = cx.listener(|this, val: &bool, _w, _cx| {
            this.checkbox_a = *val;
        });
        let on_checkbox_b = cx.listener(|this, val: &bool, _w, _cx| {
            this.checkbox_b = *val;
        });
        let on_radio = cx.listener(|this, ix: &usize, _w, _cx| {
            this.radio_index = Some(*ix);
        });
        let on_switch = cx.listener(|this, val: &bool, _w, _cx| {
            this.switch_on = *val;
        });
        let on_rating = cx.listener(|this, value: &usize, _w, cx| {
            this.rating_value = *value;
            cx.notify();
        });

        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-text-input",
                "Text Input",
            ))
            .child(
                with_gap(v_flex(), widget_gap)
                    .child(
                        demo::text_input(
                            ui,
                            cx,
                            INPUTS_FIELD,
                            &self.input_state,
                            InputField::Refined,
                            px(360.0),
                        )
                        .self_start(),
                    )
                    // The same control height without the rest of the
                    // refinement: what `geometry::input_height` is for, a
                    // field that must line up with the one above without
                    // taking its border or text size.
                    .child(
                        demo::text_input(
                            ui,
                            cx,
                            INPUTS_FIELD_HEIGHT_ONLY,
                            &self.input_height_state,
                            InputField::HeightOnly,
                            px(360.0),
                        )
                        .self_start(),
                    ),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-textarea",
                "Textarea (multi-line, the same input surface)",
            ))
            .child(
                demo::textarea(
                    ui,
                    cx,
                    "inputs-textarea",
                    &self.textarea_demo,
                    px(360.0),
                    px(90.0),
                )
                .self_start(),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-input-group",
                "InputGroup",
            ))
            .child(
                demo::input_groups(
                    ui,
                    cx,
                    "inputs-input-groups",
                    InputGroupStates {
                        search: &self.input_group_state,
                        copy: &self.input_group_button_state,
                        notes: &self.input_group_textarea_state,
                    },
                    px(360.0),
                )
                .self_start(),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-number-input",
                "Number Input",
            ))
            .child(
                demo::number_input(
                    ui,
                    cx,
                    "inputs-number-input",
                    &self.number_input_state,
                    px(200.0),
                )
                .self_start(),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-checkboxes",
                "Checkboxes",
            ))
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        demo::checkbox(
                            ui,
                            cx,
                            INPUTS_CHECKBOX_NOTIFICATIONS,
                            "Enable notifications",
                            self.checkbox_a,
                            Some(on_checkbox_a),
                        )
                        .self_start(),
                    )
                    .child(
                        demo::checkbox(
                            ui,
                            cx,
                            INPUTS_CHECKBOX_AUTOSAVE,
                            "Auto-save drafts",
                            self.checkbox_b,
                            Some(on_checkbox_b),
                        )
                        .self_start(),
                    )
                    .child(
                        demo::checkbox(
                            ui,
                            cx,
                            "inputs-checkbox-disabled",
                            "Disabled checkbox",
                            self.checkbox_c,
                            None::<fn(&bool, &mut Window, &mut gpui::App)>,
                        )
                        .self_start(),
                    ),
            )
            .child(demo::heading(ui, cx, "inputs-heading-radio", "Radio Group"))
            .child(demo::radio_group(
                ui,
                cx,
                "inputs-radio-group",
                &["Option A", "Option B", "Option C"],
                self.radio_index,
                on_radio,
            ))
            .child(demo::heading(ui, cx, "inputs-heading-switch", "Switch"))
            .child(
                h_flex()
                    .gap_6()
                    .child(demo::switch(
                        ui,
                        cx,
                        "inputs-switch-feature",
                        "Feature toggle",
                        self.switch_on,
                        Some(on_switch),
                    ))
                    .child(demo::switch(
                        ui,
                        cx,
                        "inputs-switch-disabled",
                        "Disabled",
                        true,
                        None::<fn(&bool, &mut Window, &mut gpui::App)>,
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-slider",
                format!("Slider (value: {:.0})", self.slider_value),
            ))
            .child(
                demo::slider(ui, cx, "inputs-slider", &self.slider_state, px(360.0)).self_start(),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-rating",
                format!("Rating ({} of 5 stars)", self.rating_value),
            ))
            .child(
                with_gap(h_flex(), widget_gap)
                    .items_center()
                    .child(probe(
                        PROBE_RATING,
                        demo::rating(ui, cx, "inputs-rating", self.rating_value, Some(on_rating)),
                    ))
                    .child(demo::caption(
                        ui,
                        cx,
                        "inputs-rating-value",
                        SharedString::from(format!("value: {}", self.rating_value)),
                    ))
                    .child(demo::rating(
                        ui,
                        cx,
                        "inputs-rating-disabled",
                        2,
                        None::<fn(&usize, &mut Window, &mut gpui::App)>,
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-otp",
                "OTP Input (6 digits)",
            ))
            .child(demo::otp_input(ui, cx, "inputs-otp", &self.otp_state, 2).self_start())
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-select",
                "Select (the same trigger as a Combobox, with the carried font colour)",
            ))
            .child(
                demo::select(
                    ui,
                    cx,
                    "inputs-select",
                    &self.select_demo,
                    "Pick an icon theme…",
                    px(260.0),
                )
                .self_start(),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-color-picker",
                "ColorPicker",
            ))
            .child(
                demo::color_picker(
                    ui,
                    cx,
                    "inputs-color-picker",
                    &self.color_picker_state,
                    "Pick a color",
                )
                .self_start(),
            )
            .child(demo::heading(
                ui,
                cx,
                "inputs-heading-date-picker",
                "DatePicker",
            ))
            .child(demo::date_picker(
                ui,
                cx,
                "inputs-date-picker",
                &self.date_picker_state,
                "Select a date",
            ))
            .child(demo::heading(ui, cx, "inputs-heading-calendar", "Calendar"))
            .child(demo::calendar(ui, cx, "inputs-calendar", &self.calendar_state).self_start())
    }
}
