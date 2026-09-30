//! The Basic page: the controls the three showcases all draw, in the same
//! order, with the same labels, values and states, packed onto one screen,
//! so the gpui, iced and egui captures compare control by control. Each
//! control is built by the helper its own page builds it with.
//!
//! Equal columns, five, four or three as the theme's widest fixed-width
//! control leaves room for (`basic_column_count`), each a stack of groups; a
//! group is its heading over its rows of controls, a row wrapping where its
//! column is too narrow for it. The page scrolls in the content panel where
//! it is taller than the panel. The groups and their elements are
//! docs/showcase-elements.toml's; the page, its columns and every element
//! record where they were laid out (`crate::elements`).

use gpui::{
    AnyElement, AvailableSpace, Context, IntoElement, ParentElement, Pixels, Styled, Window, div,
    prelude::*, px, size,
};
use gpui_component::{h_flex, v_flex};

use native_theme::theme::IconRole;
use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{
    self, ButtonKind, ButtonState, DemoButton, InputField, SeparatorKind, SpinnerKind, TypeRole,
};
use crate::elements;
use crate::support::{native_value, with_gap, with_padding};

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

/// The toggle button that is off: a plain Button.
const TOGGLE_OFF: BasicButton = (
    "basic-toggle-off",
    "Off",
    ButtonKind::Default,
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
const TYPE_LINES_BELOW_LINK: [(&str, TypeRole, &str); 4] = [
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
    ("basic-type-display", TypeRole::Display, "Display"),
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
    fn(native_theme_gpui::Native<'_>) -> gpui_component::Size,
    &'static str,
);
const ICONS: [SizedIcon; 3] = [
    ("basic-icon-small", geometry::icon_size_small, "Small"),
    ("basic-icon-toolbar", geometry::icon_size_toolbar, "Toolbar"),
    ("basic-icon-large", geometry::icon_size_large, "Large"),
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
/// the page's five, four and three columns (docs/showcase-elements.toml, the
/// Basic page): four columns end the other four with the fifth's groups,
/// three put the fourth's too, balanced by the groups' heights under the
/// material preset, the tallest.
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

/// The fewest and the most columns the page lays its groups out in.
const MIN_COLUMNS: usize = 3;
const MAX_COLUMNS: usize = 5;

/// The groups of each column for a page of `columns` columns.
pub(crate) fn basic_arrangement(columns: usize) -> &'static [&'static [usize]] {
    match columns {
        5 => &BASIC_FIVE,
        4 => &BASIC_FOUR,
        _ => &BASIC_THREE,
    }
}

/// How many columns the page lays its groups out in, from the theme alone,
/// as the iced and egui Basic pages count them: the most, of five, four and
/// three, whose width, `(page - (n - 1) * layout.section_gap) / n`, holds the
/// widest fixed-width control the theme states on the page --
/// `combo_box.min_width`, the two tabs at `tab.min_width` `tab.item_gap`
/// apart, `BASIC_WIDE` and `BASIC_WIDTH` -- where `page` is
/// `content_width` less `layout.window_margin` on both sides and, where
/// `scrollbar.overlay_mode` is false, the `scrollbar.groove_width` a
/// scrolling page keeps free. Three where not even three do: the page is then
/// wider than its area.
pub(crate) fn basic_column_count(
    resolved: &native_theme::theme::ResolvedTheme,
    layout: &native_theme::theme::LayoutTheme,
    content_width: f32,
) -> usize {
    let margin = layout.window_margin.unwrap_or_default();
    let gap = layout.section_gap.unwrap_or_default();
    let bar = &resolved.scrollbar;
    let gutter = if bar.overlay_mode {
        0.0
    } else {
        bar.groove_width
    };
    let page = content_width - margin - margin - gutter;
    let t = &resolved.tab;
    let tabs = t.min_width + t.min_width + t.item_gap.unwrap_or_default();
    let widest = resolved
        .combo_box
        .min_width
        .max(tabs)
        .max(BASIC_WIDE)
        .max(BASIC_WIDTH);
    (MIN_COLUMNS..=MAX_COLUMNS)
        .rev()
        .find(|&n| {
            let n = n as f32;
            (page - (n - 1.0) * gap) / n >= widest
        })
        .unwrap_or(MIN_COLUMNS)
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
        let widget_gap = geometry::widget_gap(&self.layout);
        // The space between the columns, between the groups, and round the
        // page: the theme's section gap and window margin, as the iced and
        // egui Basic pages lay them out.
        let section_gap = geometry::section_gap(&self.layout);
        let window_margin = geometry::window_margin(&self.layout);
        let width = px(BASIC_WIDTH);
        let tool_gap = native_value(cx, |n| n.resolved.toolbar.item_gap)
            .flatten()
            .map(px);

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
                        ))
                        .into_any_element(),
                    row()
                        .child(button(TOGGLE_OFF))
                        .child(demo::toggle_button(ui, cx, "basic-toggle-on", "On"))
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
                        width,
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
                    demo::slider(ui, cx, "basic-slider", &self.basic_slider_state, width)
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
                    &self.basic_number_state,
                    width,
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
                    &["Day", "Week", "Month"],
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
                        BASIC_PROGRESS,
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
        let g_icons = group(
            icons,
            vec![
                // As the toolbar's buttons are spaced: `toolbar.item_gap`,
                // `layout.widget_gap` where the theme states none.
                with_gap(h_flex(), tool_gap.or(widget_gap))
                    .flex_wrap()
                    .items_center()
                    .children(ICON_BUTTONS.map(|(id, role, name, listed)| {
                        demo::icon_button(ui, cx, id, name, &self.role_chrome_icon(role), listed)
                    }))
                    .into_any_element(),
                row()
                    .children(
                        ICONS.map(|(id, size, what)| {
                            demo::sized_icon(ui, cx, id, &folder, size, what)
                        }),
                    )
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
                    geometry::container_margin(&self.layout),
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
                    "basic-expander",
                    [
                        ("Details", "basic-expander-body", "Expanded content"),
                        ("More", "basic-expander-more-body", "More content"),
                    ],
                    self.basic_expanded,
                    wide,
                    widget_gap,
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
                    TABLE_SELECTED,
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
        let count = native_value(cx, |n| {
            basic_column_count(n.resolved, &self.layout, f32::from(content_width))
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
