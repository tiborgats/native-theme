//! The Basic page: the controls the three showcases all draw, in the same
//! order, with the same labels, values and states, packed onto one screen,
//! so the gpui, iced and egui captures compare control by control. Each
//! control is built by the helper its own page builds it with.
//!
//! Four equal columns, each a stack of groups; a group is its heading over
//! its rows of controls.

use gpui::{
    AnyElement, AvailableSpace, Context, IntoElement, ParentElement, Pixels, Styled, Window, div,
    prelude::*, px, size,
};
use gpui_component::{h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{
    self, ButtonKind, ButtonState, DemoButton, InputField, SeparatorKind, SpinnerKind,
};
use crate::support::{native_value, with_gap, with_padding};

/// The width, in logical pixels, of the Basic page's text fields, drop-down,
/// slider and progress bar. The model states no such width; it is the Basic
/// page's own, the iced and egui showcases' `BASIC_WIDTH` too, so the three
/// pages lay the same controls out alike.
const BASIC_WIDTH: f32 = 140.0;

/// The width, in logical pixels, of the Basic page's text area, list,
/// expander, card and separator: the Basic page's own, as `BASIC_WIDTH` is,
/// and the iced and egui showcases' `BASIC_WIDE` too.
const BASIC_WIDE: f32 = 200.0;

/// The progress bar's value, in percent: the datum on display.
const BASIC_PROGRESS: f32 = 40.0;

/// How many of the List's rows show at once: fewer than it has, so its
/// scrollbar shows.
const BASIC_LIST_VISIBLE: f32 = 4.0;

/// A Button of the page, as `(id, label, variant, state)`.
type BasicButton = (&'static str, &'static str, ButtonKind, ButtonState);

/// The first row of buttons.
const BUTTONS: [BasicButton; 2] = [
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
];

/// The second row's Button, beside the tooltip's.
const DISABLED_BUTTON: BasicButton = (
    "basic-button-disabled",
    "Disabled",
    ButtonKind::Default,
    ButtonState::Disabled,
);

/// A group of the page, as `(heading id, heading)`.
pub(crate) type BasicGroup = (&'static str, &'static str);

/// The groups of each column, top to bottom.
pub(crate) const BASIC_COLUMN_1: [BasicGroup; 4] = [
    ("basic-heading-buttons", "Buttons"),
    ("basic-heading-checkboxes", "Checkboxes"),
    ("basic-heading-radio", "Radio buttons"),
    ("basic-heading-switches", "Switches"),
];
pub(crate) const BASIC_COLUMN_2: [BasicGroup; 4] = [
    ("basic-heading-inputs", "Text inputs"),
    ("basic-heading-textarea", "Text area"),
    ("basic-heading-select", "Drop-down"),
    ("basic-heading-text", "Text"),
];
pub(crate) const BASIC_COLUMN_3: [BasicGroup; 5] = [
    ("basic-heading-slider", "Slider"),
    ("basic-heading-progress", "Progress bar"),
    ("basic-heading-spinner", "Spinner"),
    ("basic-heading-tabs", "Tabs"),
    ("basic-heading-segmented", "Segmented control"),
];
pub(crate) const BASIC_COLUMN_4: [BasicGroup; 4] = [
    ("basic-heading-list", "List"),
    ("basic-heading-expander", "Expander"),
    ("basic-heading-card", "Card"),
    ("basic-heading-separator", "Separator"),
];

/// A control held in the one state it shows: a click changes nothing.
fn held(_: &bool, _: &mut Window, _: &mut gpui::App) {}

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Basic
    // -----------------------------------------------------------------------
    pub(crate) fn render_basic_page(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let wide = px(BASIC_WIDE);
        let list_height = self.basic_list_height(window, cx, wide);
        let ui = &self.info_ui;
        let widget_gap = geometry::widget_gap(&self.layout);
        // The space between the columns, between the groups, and round the
        // page: the theme's section gap and window margin, as the iced and
        // egui Basic pages lay them out.
        let section_gap = geometry::section_gap(&self.layout);
        let window_margin = geometry::window_margin(&self.layout);
        let width = px(BASIC_WIDTH);

        let on_radio = cx.listener(|this, ix: &usize, _w, _cx| {
            this.basic_radio = Some(*ix);
        });
        let on_tab = cx.listener(|this, ix: &usize, _w, cx| {
            this.basic_tab = *ix;
            cx.notify();
        });
        let on_segment = cx.listener(|this, ix: &usize, _w, cx| {
            this.basic_segment = *ix;
            cx.notify();
        });
        let on_expander = cx.listener(|this, open: &[bool; 2], _w, cx| {
            this.basic_expanded = *open;
            cx.notify();
        });

        // A heading over its rows, `widget_gap` apart.
        let group = |(id, text): BasicGroup, rows: Vec<AnyElement>| {
            with_gap(v_flex(), widget_gap)
                .items_start()
                .child(demo::heading(ui, cx, id, text))
                .children(rows)
        };
        // Controls side by side, `widget_gap` apart.
        let row = || with_gap(h_flex(), widget_gap).items_center();
        let button = |(id, label, kind, state): BasicButton| {
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
        };
        let [buttons, checkboxes, radios, switches] = BASIC_COLUMN_1;
        let [inputs, textarea, select, text] = BASIC_COLUMN_2;
        let [slider, progress, spinner, tabs, segmented] = BASIC_COLUMN_3;
        let [list, expander, card, separator] = BASIC_COLUMN_4;
        let no_click = None::<fn(&bool, &mut Window, &mut gpui::App)>;

        let column1 = vec![
            group(
                buttons,
                vec![
                    row().children(BUTTONS.map(button)).into_any_element(),
                    row()
                        .child(button(DISABLED_BUTTON))
                        // Built by the application, so the tooltip takes the
                        // platform's padding, radius and colours
                        // (`geometry::tooltip`); `Button::tooltip` builds
                        // its own.
                        .child(demo::built_tooltip_button(
                            ui,
                            cx,
                            "basic-button-tooltip",
                            "Tooltip",
                            "A tooltip",
                        ))
                        .into_any_element(),
                ],
            ),
            group(
                checkboxes,
                vec![
                    demo::checkbox(
                        ui,
                        cx,
                        "basic-checkbox-unchecked",
                        "Unchecked",
                        false,
                        Some(held),
                    )
                    .into_any_element(),
                    demo::checkbox(
                        ui,
                        cx,
                        "basic-checkbox-checked",
                        "Checked",
                        true,
                        Some(held),
                    )
                    .into_any_element(),
                    demo::checkbox(
                        ui,
                        cx,
                        "basic-checkbox-disabled",
                        "Disabled",
                        true,
                        no_click,
                    )
                    .into_any_element(),
                ],
            ),
            group(
                radios,
                vec![
                    demo::radio_column(
                        ui,
                        cx,
                        "basic-radio",
                        &["Option A", "Option B"],
                        widget_gap,
                        self.basic_radio,
                        on_radio,
                    )
                    .into_any_element(),
                ],
            ),
            group(
                switches,
                vec![
                    demo::switch(ui, cx, "basic-switch-off", "Off", false, Some(held))
                        .into_any_element(),
                    demo::switch(ui, cx, "basic-switch-on", "On", true, Some(held))
                        .into_any_element(),
                    demo::switch(ui, cx, "basic-switch-disabled", "Disabled", true, no_click)
                        .into_any_element(),
                ],
            ),
        ];

        let column2 = vec![
            group(
                inputs,
                vec![
                    demo::text_input(
                        ui,
                        cx,
                        "basic-input-placeholder",
                        &self.basic_hint_state,
                        InputField::Refined,
                        false,
                        width,
                    )
                    .into_any_element(),
                    demo::text_input(
                        ui,
                        cx,
                        "basic-input-filled",
                        &self.basic_text_state,
                        InputField::Refined,
                        false,
                        width,
                    )
                    .into_any_element(),
                    demo::text_input(
                        ui,
                        cx,
                        crate::BASIC_INPUT_DISABLED,
                        &self.basic_disabled_state,
                        InputField::Refined,
                        true,
                        width,
                    )
                    .into_any_element(),
                ],
            ),
            group(
                textarea,
                vec![
                    demo::rows_textarea(ui, cx, "basic-textarea", &self.basic_textarea, wide)
                        .into_any_element(),
                ],
            ),
            group(
                select,
                vec![
                    demo::select(
                        ui,
                        cx,
                        "basic-select",
                        &self.basic_select,
                        "Pick a fruit",
                        width,
                    )
                    .into_any_element(),
                ],
            ),
            group(
                text,
                vec![
                    demo::body_label(ui, cx, "basic-body-text", "Body text").into_any_element(),
                    demo::link(
                        ui,
                        cx,
                        "basic-link",
                        "Link",
                        "https://github.com/tiborgats/native-theme",
                    )
                    .into_any_element(),
                ],
            ),
        ];

        let column3 = vec![
            group(
                slider,
                vec![
                    demo::slider(ui, cx, "basic-slider", &self.basic_slider_state, width)
                        .into_any_element(),
                ],
            ),
            group(
                progress,
                vec![
                    div()
                        .w(width)
                        .child(demo::progress(
                            ui,
                            cx,
                            "basic-progress",
                            "Progress bar",
                            BASIC_PROGRESS,
                        ))
                        .into_any_element(),
                ],
            ),
            group(
                spinner,
                vec![
                    demo::spinner(ui, cx, "basic-spinner", SpinnerKind::Medium).into_any_element(),
                ],
            ),
            group(
                tabs,
                vec![
                    demo::tab_row(
                        ui,
                        cx,
                        "basic-tabs",
                        &["One", "Two", "Three"],
                        self.basic_tab,
                        on_tab,
                    )
                    .into_any_element(),
                ],
            ),
            group(
                segmented,
                vec![
                    demo::segmented(
                        ui,
                        cx,
                        "basic-segmented",
                        &["Day", "Week", "Month"],
                        self.basic_segment,
                        on_segment,
                    )
                    .into_any_element(),
                ],
            ),
        ];

        let column4 = vec![
            group(
                list,
                vec![
                    demo::list(
                        ui,
                        cx,
                        "basic-list",
                        "basic-list",
                        &self.basic_list,
                        wide,
                        list_height,
                    )
                    .into_any_element(),
                ],
            ),
            group(
                expander,
                vec![
                    demo::expander(
                        ui,
                        cx,
                        "basic-expander",
                        [
                            ("Details", "basic-expander-body", "Expanded content"),
                            ("More", "basic-expander-more-body", "More content"),
                        ],
                        self.basic_expanded,
                        wide,
                        on_expander,
                    )
                    .into_any_element(),
                ],
            ),
            group(
                card,
                vec![
                    demo::card(
                        ui,
                        cx,
                        "basic-card",
                        "basic-card-text",
                        "Card content",
                        wide,
                    )
                    .into_any_element(),
                ],
            ),
            group(
                separator,
                vec![
                    div()
                        .w(wide)
                        .child(demo::separator(
                            ui,
                            cx,
                            "basic-separator",
                            SeparatorKind::Horizontal,
                        ))
                        .into_any_element(),
                ],
            ),
        ];

        let column = |groups: Vec<gpui::Div>| {
            with_gap(v_flex(), section_gap)
                .flex_1()
                .min_w_0()
                .items_start()
                .children(groups)
        };
        with_padding(with_gap(h_flex(), section_gap), window_margin)
            .items_start()
            .flex_1()
            .child(column(column1))
            .child(column(column2))
            .child(column(column3))
            .child(column(column4))
    }

    /// The Basic List's height: `BASIC_LIST_VISIBLE` rows and the List's
    /// frame. A row is measured as the List measures its own
    /// (list/list.rs, `ListState::prepare_items_if_needed`), `wide` wide, so
    /// the height is the row the theme gives, stated or not; the frame is
    /// `list.border.line_width` above and below.
    fn basic_list_height(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
        wide: Pixels,
    ) -> Pixels {
        let mut row = demo::ListRow::new(&self.info_ui, "basic-list-measure", 0, "Item 1".into())
            .into_any_element();
        let measured = row.layout_as_root(
            size(AvailableSpace::Definite(wide), AvailableSpace::MinContent),
            window,
            cx,
        );
        let frame = px(native_value(cx, |n| n.resolved.list.border.line_width).unwrap_or_default());
        measured.height * BASIC_LIST_VISIBLE + frame + frame
    }
}
