//! The Basic page: the controls the three showcases all draw, in the same
//! order, with the same labels, values and states, packed onto one screen,
//! so the gpui, iced and egui captures compare control by control. Each
//! control is built by the helper its own page builds it with.
//!
//! Equal columns, five down to one as the theme's widest fixed-width
//! control and the Segmented control leave room for
//! (`basic_column_count`), each a stack of groups; a
//! group is its heading over its rows of controls, a row wrapping where its
//! column is too narrow for it. The page scrolls in the content panel where
//! it is taller than the panel. The groups and their elements are
//! docs/showcase-elements.toml's; the page, its columns and every element
//! record where they were laid out (`crate::elements`).

use gpui::{
    AnyElement, AvailableSpace, Axis, Context, IntoElement, ParentElement, Pixels, Styled, Window,
    div, prelude::*, px, size,
};
use gpui_component::{ActiveTheme as _, IconName, h_flex, v_flex};

use native_theme::icons::IconRole;
use native_theme_gpui::next;

use crate::app::Showcase;
use crate::demo::{
    self, ButtonKind, ButtonState, DemoButton, ExpanderArrows, ExpanderSpec, InputField,
    SeparatorKind, SpinnerKind, StepIcons, TypeRole,
};
use crate::elements;
use crate::support::{next_metric, next_value, with_gap, with_padding};

/// The width, in logical pixels, of the Basic page's text fields, drop-down,
/// number input, slider and progress bar. The model states no such width;
/// it is the Basic page's own, the iced and egui showcases' `BASIC_WIDTH`
/// too, so the three pages lay the same controls out alike.
const BASIC_WIDTH: f32 = 140.0;

/// The width, in logical pixels, of the Basic page's text area, list,
/// expander, card, separator and table: the Basic page's own, as
/// `BASIC_WIDTH` is, and the iced and egui showcases' `BASIC_WIDE` too.
const BASIC_WIDE: f32 = 170.0;

/// The progress bar's value, in percent: the datum on display.
const BASIC_PROGRESS: f32 = 40.0;

/// How many of the List's rows show at once: fewer than it has, so its
/// scrollbar shows.
const BASIC_LIST_VISIBLE: f32 = 3.0;

/// A Button of the page, as `(id, label, variant, state)`.
type BasicButton = (&'static str, &'static str, ButtonKind, ButtonState);

/// The first row of buttons.
const BUTTONS: [BasicButton; 2] = [
    (
        "basic-button",
        "Button",
        ButtonKind::Bordered,
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

/// The toggle button that is off: a plain Button.
const TOGGLE_OFF: BasicButton = (
    "basic-toggle-off",
    "Off",
    ButtonKind::Bordered,
    ButtonState::Idle,
);

/// The icon buttons, as (id, the role whose icon it shows, its name, the
/// element of docs/showcase-elements.toml its icon is).
const ICON_BUTTONS: [(&str, IconRole, &str, &str); 3] = [
    (
        "basic-icon-copy",
        IconRole::ActionCopy,
        "Copy",
        "basic.icons.copy.icon",
    ),
    (
        "basic-icon-paste",
        IconRole::ActionPaste,
        "Paste",
        "basic.icons.paste.icon",
    ),
    (
        "basic-icon-delete",
        IconRole::ActionDelete,
        "Delete",
        "basic.icons.delete.icon",
    ),
];

/// The Typography group's lines above its Link, and those below it, as (id,
/// role, text).
const TYPE_LINES_ABOVE_LINK: [(&str, TypeRole, &str); 2] = [
    ("basic-type-caption", TypeRole::Caption, "Caption"),
    ("basic-type-body", TypeRole::Body, "Body"),
];
const TYPE_LINES_BELOW_LINK: [(&str, TypeRole, &str); 3] = [
    (
        "basic-type-section-heading",
        TypeRole::SectionHeading,
        "Section heading",
    ),
    (
        "basic-type-dialog-title",
        TypeRole::DialogTitle,
        "Dialog title",
    ),
    ("basic-type-monospace", TypeRole::Monospace, "Monospace"),
];

/// The Table's header, as (text, the element of docs/showcase-elements.toml
/// its cell is), and its rows; the second row is selected.
const TABLE_HEAD: [(&str, &str); 2] = [
    ("Name", "basic.table.header.name"),
    ("Size", "basic.table.header.size"),
];
const TABLE_ROWS: [[&str; 2]; 2] = [["a.txt", "1 KB"], ["b.png", "20 KB"]];
const TABLE_SELECTED: usize = 1;

/// The Icons group's icons, as (id, the size builder, what it is), each
/// native-theme's `IconRole::FolderOpen` of the shown set, the iced and
/// egui showcases' too.
type SizedIcon = (
    &'static str,
    fn(&native_theme::next::ResolvedTheme) -> next::Mapped<gpui_component::Size>,
    &'static str,
);
const ICONS: [SizedIcon; 3] = [
    ("basic-icon-small", next::geometry::icon_size_small, "Small"),
    (
        "basic-icon-toolbar",
        next::geometry::icon_size_toolbar,
        "Toolbar",
    ),
    ("basic-icon-large", next::geometry::icon_size_large, "Large"),
];

/// A group of the page, as `(heading id, heading)`.
pub(crate) type BasicGroup = (&'static str, &'static str);

/// The page's groups in reading order: the five columns' groups, left to
/// right, each column top to bottom.
pub(crate) const BASIC_GROUPS: [BasicGroup; 20] = [
    ("basic-heading-buttons", "Buttons"),
    ("basic-heading-checkboxes", "Checkboxes"),
    ("basic-heading-radio", "Radio buttons"),
    ("basic-heading-select", "Drop-down"),
    ("basic-heading-inputs", "Text inputs"),
    ("basic-heading-textarea", "Text area"),
    ("basic-heading-slider", "Slider"),
    ("basic-heading-switches", "Switches"),
    ("basic-heading-number", "Number input"),
    ("basic-heading-spinner", "Spinner"),
    ("basic-heading-segmented", "Segmented control"),
    ("basic-heading-card", "Card"),
    ("basic-heading-typography", "Typography"),
    ("basic-heading-separator", "Separator"),
    ("basic-heading-progress", "Progress bar"),
    ("basic-heading-list", "List"),
    ("basic-heading-icons", "Icons"),
    ("basic-heading-tabs", "Tabs"),
    ("basic-heading-expander", "Expander"),
    ("basic-heading-table", "Table"),
];

/// Each column's groups, top to bottom, as indices of [`BASIC_GROUPS`], for
/// the page's five, four, three, two and one columns
/// (docs/showcase-elements.toml, the Basic page): four columns end the other
/// four with the fifth's groups, three put the fourth's too and two the
/// third's, balanced by the groups' heights under the material-3 preset, the
/// tallest; one column holds every group in reading order. A column's groups
/// are in reading order.
pub(crate) const BASIC_FIVE: [&[usize]; 5] = [
    &[0, 1, 2, 3],
    &[4, 5, 6],
    &[7, 8, 9, 10, 11],
    &[12, 13, 14, 15],
    &[16, 17, 18, 19],
];
pub(crate) const BASIC_FOUR: [&[usize]; 4] = [
    &[0, 1, 2, 3, 18],
    &[4, 5, 6, 16],
    &[7, 8, 9, 10, 11, 17],
    &[12, 13, 14, 15, 19],
];
pub(crate) const BASIC_THREE: [&[usize]; 3] = [
    &[0, 1, 2, 3, 15, 18],
    &[4, 5, 6, 13, 14, 16, 19],
    &[7, 8, 9, 10, 11, 12, 17],
];
pub(crate) const BASIC_TWO: [&[usize]; 2] = [
    &[0, 1, 2, 3, 7, 12, 15, 17, 18],
    &[4, 5, 6, 8, 9, 10, 11, 13, 14, 16, 19],
];
pub(crate) const BASIC_ONE: [&[usize]; 1] = [&[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
]];

/// The fewest and the most columns the page lays its groups out in.
const MIN_COLUMNS: usize = 1;
const MAX_COLUMNS: usize = 5;

/// The Segmented control's group in [`BASIC_GROUPS`].
const SEGMENTED_GROUP: usize = 10;

/// The Segmented control's segments, which the column count measures.
pub(crate) const BASIC_SEGMENTS: [&str; 3] = ["Day", "Week", "Month"];

/// The groups of each column for a page of `columns` columns.
pub(crate) fn basic_arrangement(columns: usize) -> &'static [&'static [usize]] {
    match columns {
        5 => &BASIC_FIVE,
        4 => &BASIC_FOUR,
        3 => &BASIC_THREE,
        2 => &BASIC_TWO,
        _ => &BASIC_ONE,
    }
}

/// Whether a page of `columns` columns lays the Segmented control out in its
/// last column, which the page's right margin follows.
fn segmented_in_last_column(columns: usize) -> bool {
    basic_arrangement(columns)
        .last()
        .is_some_and(|column| column.contains(&SEGMENTED_GROUP))
}

/// How many columns the page lays its groups out in, from the installed
/// 0.8.0 theme `r` alone, as the iced and egui Basic pages count them: the
/// most, of five down to one, whose width,
/// `(page - (n - 1) * layout.section_gap) / n`, holds the widest fixed-width
/// control the theme states on the page --
/// `combo_box.field(NONE).min_size[0]`, the two tabs at
/// `tab.tab(NONE).min_size[0]` `tab.item_gap` apart with room for both of
/// the selected tab's flares, `2 * tab.active_outward_corner_radius` where
/// `widgets::TabBar` draws them (`TabLook::outward_radius`), `BASIC_WIDE`
/// and `BASIC_WIDTH` -- where `page` is `content_width` less
/// `layout.window_margin` on both sides and, where `scrollbar.overlay_mode`
/// is false, the groove a scrolling page keeps free,
/// `scrollbar.track(NONE; orientation=Vertical).min_size[0]`
/// (`next::scrollbar_groove_width`). A width or a metric the theme leaves
/// unstated takes no room: gpui-component's tabs and drop-down have no
/// minimum width of their own, and the page sets no gap, margin or gutter
/// the theme does not state (`support::with_gap`, `next_metric`). The
/// column also holds `segmented`, the
/// Segmented control's own width (`demo::segmented_width`, as gpui measures
/// its labels), whose segments stay on one row: the control may reach into
/// the section gap after its column, but keeps `layout.widget_gap`, the gap
/// the page's rows keep between neighbouring widgets (Button and Primary),
/// from the next column's left edge (M9-166); in the last column it may
/// reach into the page's right margin, `layout.window_margin`, keeping
/// `layout.widget_gap` from the scroll area's edge or its scroll bar: where
/// `scrollbar.overlay_mode` is true the bar floats over that margin, its
/// groove width taken off the room. One
/// where not even one does: the page is then wider than its area.
pub(crate) fn basic_column_count(
    r: &native_theme::next::ResolvedTheme,
    content_width: f32,
    segmented: f32,
) -> usize {
    use native_theme::next::WidgetState;
    let metric = |build: fn(&native_theme::next::ResolvedTheme) -> next::Mapped<Option<Pixels>>| {
        build(r).ok().flatten().map(f32::from).unwrap_or_default()
    };
    let margin = metric(next::geometry::window_margin);
    let gap = metric(next::geometry::section_gap);
    let groove = next::scrollbar_groove_width(r).unwrap_or_default();
    let overlay_mode = r.scrollbar.overlay_mode;
    let gutter = if overlay_mode { 0.0 } else { groove };
    // a floating bar takes no width of the page, and is drawn over its right margin
    let overlay = if overlay_mode { groove } else { 0.0 };
    let page = content_width - margin - margin - gutter;
    let t = &r.tab;
    let tab_width = t.tab(WidgetState::NONE).min_size[0].unwrap_or_default();
    let flare = outward_radius(r);
    let tabs = tab_width + tab_width + t.item_gap.unwrap_or_default() + flare + flare;
    let widest = r.combo_box.field(WidgetState::NONE).min_size[0]
        .unwrap_or_default()
        .max(tabs)
        .max(BASIC_WIDE)
        .max(BASIC_WIDTH);
    let widget_gap = metric(next::geometry::widget_gap);
    (MIN_COLUMNS..=MAX_COLUMNS)
        .rev()
        .find(|&n| {
            let column = (page - (n as f32 - 1.0) * gap) / n as f32;
            // past the column into the section gap, or in the last column into the page's
            // right margin, keeping `layout.widget_gap` from what lies beyond either: the
            // scroll area's edge, or a floating bar drawn over the margin
            let room = if segmented_in_last_column(n) {
                column + margin - overlay - widget_gap
            } else {
                column + gap - widget_gap
            };
            column >= widest && room >= segmented
        })
        .unwrap_or(MIN_COLUMNS)
}

/// The radius of the selected tab's flares as `widgets::TabBar` draws them
/// (`TabLook::next_of`), 0 where it draws none: a radius the theme leaves
/// unstated, states as 0, or states as a length the bar does not take, a
/// theme the bar cannot be drawn from, or a build without the `widgets`
/// feature, whose Basic tabs are gpui-component's.
fn outward_radius(r: &native_theme::next::ResolvedTheme) -> f32 {
    #[cfg(feature = "widgets")]
    {
        native_theme_gpui::widgets::TabLook::next_of(r)
            .ok()
            .and_then(|look| look.outward_radius)
            .map(f32::from)
            .unwrap_or_default()
    }
    #[cfg(not(feature = "widgets"))]
    {
        let _ = r;
        0.0
    }
}

/// The elements of docs/showcase-elements.toml the page and its columns
/// are.
const PAGE: &str = "basic.page";
const COLUMNS: [&str; 5] = [
    "basic.column_1",
    "basic.column_2",
    "basic.column_3",
    "basic.column_4",
    "basic.column_5",
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
        let widget_gap = next_metric(cx, next::geometry::widget_gap);
        // The space between the columns, between the groups, and round the
        // page: the theme's section gap and window margin, as the iced and
        // egui Basic pages lay them out.
        let section_gap = next_metric(cx, next::geometry::section_gap);
        let window_margin = next_metric(cx, next::geometry::window_margin);
        let width = px(BASIC_WIDTH);
        let tool_gap = next_value(cx, |r| r.toolbar.item_gap).flatten().map(px);

        let on_radio = cx.listener(|this, ix: &usize, _w, cx| {
            this.basic_radio = Some(*ix);
            cx.notify();
        });
        let on_edit = cx.listener(|this, _, _, cx| {
            this.basic_edit = !this.basic_edit;
            cx.notify();
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
        // Controls side by side, `widget_gap` apart, wrapping onto the next
        // line where their column is too narrow for them, as the iced
        // (`Row::wrap`) and egui (`horizontal_wrapped`) pages wrap them.
        let row = || with_gap(h_flex(), widget_gap).flex_wrap().items_center();
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
        let [
            buttons,
            checkboxes,
            radios,
            select,
            inputs,
            textarea,
            slider,
            switches,
            number,
            spinner,
            segmented,
            card,
            typography,
            separator,
            progress,
            list,
            icons,
            tabs,
            expander,
            table,
        ] = BASIC_GROUPS;
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
                            ButtonKind::Bordered,
                        ))
                        .into_any_element(),
                    row()
                        .child(button(TOGGLE_OFF))
                        .child(demo::toggle_button(ui, cx, "basic-toggle-on", "On"))
                        .child(demo::dialog_default_button(
                            ui,
                            cx,
                            "basic-button-dialog-default",
                            "OK",
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
                select,
                vec![
                    demo::select(
                        ui,
                        cx,
                        "basic-select",
                        &self.basic_select,
                        "Pick a fruit",
                        // At least `combo_box.field(NONE).min_size[0]`, which
                        // `next::geometry::select` sets on the trigger inside
                        // where the theme states it: the box round it too, as
                        // the iced and egui pages size theirs.
                        next_value(cx, |r| {
                            r.combo_box
                                .field(native_theme::next::WidgetState::NONE)
                                .min_size[0]
                        })
                        .flatten()
                        .map_or(width, |min| px(BASIC_WIDTH.max(min))),
                    )
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
                    demo::text_input(
                        ui,
                        cx,
                        "basic-input-focused",
                        &self.basic_focused_state,
                        InputField::Refined,
                        false,
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
                slider,
                vec![
                    demo::slider(
                        ui,
                        cx,
                        "basic-slider",
                        "Sample value",
                        &self.basic_slider_state,
                        Axis::Horizontal,
                        width,
                    )
                    .into_any_element(),
                ],
            ),
        ];

        let g_switches = group(
            switches,
            vec![
                demo::switch(ui, cx, "basic-switch-off", "Off", false, Some(held))
                    .into_any_element(),
                demo::switch(ui, cx, "basic-switch-on", "On", true, Some(held)).into_any_element(),
                demo::switch(ui, cx, "basic-switch-disabled", "Disabled", true, no_click)
                    .into_any_element(),
            ],
        );
        let g_number = group(
            number,
            vec![
                demo::number_input(
                    ui,
                    cx,
                    window,
                    "basic-number",
                    "Sample number",
                    &self.basic_number_state,
                    width,
                    &StepIcons {
                        up: self.sample_icon(IconName::ChevronUp),
                        down: self.sample_icon(IconName::ChevronDown),
                    },
                )
                .into_any_element(),
            ],
        );
        let g_spinner = group(
            spinner,
            vec![
                demo::spinner(
                    ui,
                    cx,
                    "basic-spinner",
                    SpinnerKind::Medium,
                    &self.spinner_icons(),
                )
                .into_any_element(),
            ],
        );
        let g_tabs = group(
            tabs,
            vec![
                demo::tab_row(
                    ui,
                    cx,
                    "basic-tabs",
                    &["One", "Two"],
                    self.basic_tab,
                    on_tab,
                )
                .into_any_element(),
            ],
        );
        let g_segmented = group(
            segmented,
            vec![
                demo::segmented(
                    ui,
                    cx,
                    "basic-segmented",
                    &BASIC_SEGMENTS,
                    self.basic_segment,
                    on_segment,
                )
                .into_any_element(),
            ],
        );

        let type_line =
            |(id, role, text)| demo::type_line(ui, cx, id, role, text).into_any_element();
        let folder = self.role_chrome_icon(IconRole::FolderOpen);
        let g_progress = group(
            progress,
            vec![
                div()
                    .w(width)
                    .child(demo::progress(
                        ui,
                        cx,
                        "basic-progress",
                        "Progress bar",
                        Some(BASIC_PROGRESS),
                    ))
                    .into_any_element(),
            ],
        );
        let g_typography = group(
            typography,
            TYPE_LINES_ABOVE_LINK
                .map(type_line)
                .into_iter()
                .chain([demo::link(
                    ui,
                    cx,
                    "basic-link",
                    "Link",
                    "https://github.com/tiborgats/native-theme",
                )
                .into_any_element()])
                .chain(TYPE_LINES_BELOW_LINK.map(type_line))
                .collect(),
        );
        let g_icons =
            group(
                icons,
                vec![
                    // As the toolbar's buttons are spaced: `toolbar.item_gap`,
                    // `layout.widget_gap` where the theme states none.
                    with_gap(h_flex(), tool_gap.or(widget_gap))
                        .flex_wrap()
                        .items_center()
                        .children(ICON_BUTTONS.map(|(id, role, name, listed)| {
                            demo::icon_button(
                                ui,
                                cx,
                                demo::IconButton {
                                    id,
                                    name,
                                    drawn: &self.role_chrome_icon(role),
                                    listed,
                                    selected: false,
                                },
                                None::<fn(&gpui::ClickEvent, &mut Window, &mut gpui::App)>,
                            )
                        }))
                        // The edit-mode switch, on at start: the one toolbar
                        // button that is ever checked.
                        .child(demo::icon_button(
                            ui,
                            cx,
                            demo::IconButton {
                                id: "basic-icon-edit",
                                name: "Edit",
                                drawn: &self.role_chrome_icon(IconRole::ActionEdit),
                                listed: "basic.icons.edit.icon",
                                selected: self.basic_edit,
                            },
                            Some(on_edit),
                        ))
                        .into_any_element(),
                    row()
                        .children(ICONS.map(|(id, size, what)| {
                            demo::sized_icon(ui, cx, id, &folder, size, what)
                        }))
                        .into_any_element(),
                ],
            );
        let g_separator = group(
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
        );

        let g_card = group(
            card,
            vec![
                demo::card(
                    ui,
                    cx,
                    "basic-card",
                    "basic-card-text",
                    "Card content",
                    wide,
                    next_metric(cx, next::geometry::container_margin),
                )
                .into_any_element(),
            ],
        );
        let g_list = group(
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
        );
        let g_expander = group(
            expander,
            vec![
                demo::expander(
                    ui,
                    cx,
                    ExpanderSpec {
                        id: "basic-expander",
                        items: [
                            ("Details", "basic-expander-body", "Expanded content"),
                            ("More", "basic-expander-more-body", "More content"),
                        ],
                        open: self.basic_expanded,
                        width: wide,
                        gap: widget_gap,
                        arrows: ExpanderArrows {
                            leading: self.sample_icon(IconName::ChevronRight),
                            trailing: self.sample_icon(IconName::ChevronDown),
                            leading_open: self.sample_icon(IconName::ChevronDown),
                            trailing_open: self.sample_icon(IconName::ChevronUp),
                        },
                    },
                    on_expander,
                )
                .into_any_element(),
            ],
        );
        let g_table = group(
            table,
            vec![
                demo::files_table(
                    ui,
                    cx,
                    "basic-table",
                    TABLE_HEAD,
                    &TABLE_ROWS,
                    demo::TableSelection {
                        row: TABLE_SELECTED,
                        window_active: window.is_window_active(),
                    },
                    wide,
                )
                .into_any_element(),
            ],
        );

        // Every group in reading order, each taken once into its column.
        let mut groups: Vec<Option<gpui::Div>> = column1
            .into_iter()
            .chain(column2)
            .chain([
                g_switches,
                g_number,
                g_spinner,
                g_segmented,
                g_card,
                g_typography,
                g_separator,
                g_progress,
                g_list,
                g_icons,
                g_tabs,
                g_expander,
                g_table,
            ])
            .map(Some)
            .collect();
        let content_width = self.content_width(window, cx);
        let count = next_value(cx, |r| {
            let segmented = demo::segmented_width(window, cx.theme(), r, &BASIC_SEGMENTS);
            basic_column_count(r, f32::from(content_width), segmented)
        })
        .unwrap_or(MAX_COLUMNS);
        // The width is the last frame's: where this frame lays the content
        // panel out at another (a first frame, a moved splitter), the next
        // frame counts the columns again.
        let watched = self.content_scroll.clone();
        let this = cx.entity().downgrade();
        let recount = gpui::canvas(
            move |_, _, cx| {
                if watched.bounds().size.width != content_width {
                    this.update(cx, |_, cx| cx.notify()).ok();
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .size_0();
        let columns = basic_arrangement(count)
            .iter()
            .zip(COLUMNS)
            .map(|(members, listed)| {
                with_gap(v_flex(), section_gap)
                    .relative()
                    .flex_1()
                    .min_w_0()
                    .items_start()
                    .child(elements::record(ui, listed))
                    .children(
                        members
                            .iter()
                            .filter_map(|&ix| groups.get_mut(ix).and_then(Option::take)),
                    )
            })
            .collect::<Vec<_>>();
        // The page is the columns' row, inside the window margin.
        with_padding(div(), window_margin).flex_1().child(
            with_gap(h_flex(), section_gap)
                .relative()
                .w_full()
                .items_start()
                .child(elements::record(ui, PAGE))
                .child(recount)
                .children(columns),
        )
    }

    /// The Basic List's height: `BASIC_LIST_VISIBLE` rows and the List's
    /// frame. A row is measured as the List measures its own
    /// (list/list.rs, `ListState::prepare_items_if_needed`), `wide` wide, so
    /// the height is the row the theme gives, stated or not; the frame is
    /// `list.container(NONE).stroke.width` above and below, which
    /// `next::geometry::list` draws, none where the theme states no width
    /// gpui can take.
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
        let frame = px(next_value(cx, |r| {
            crate::info::stroke_width_of(&r.list.container(native_theme::next::WidgetState::NONE))
        })
        .flatten()
        .unwrap_or_default());
        px(f32::from(measured.height) * BASIC_LIST_VISIBLE + f32::from(frame) + f32::from(frame))
    }
}
