//! What the role cells (`roles`, `derived::apply_role`) read, from either model: each value
//! `None` where the theme states none, so that the cell keeps what it holds there (spec §10,
//! rule 0). [`CellSource::old`] fills it from the 0.7.2 model, every value stated;
//! `next::build::cell_source` from the 0.8.0 model's homes.

use super::base::{ButtonStates, Read, Sides};
use super::line::LineAlign;
use super::states::{Edge, StateLine};
use crate::accessors::scaled_text_size;
use crate::convert::{Rgba, padding_with_line};
use crate::{AccessibilityPreferences, ResolvedTheme};

/// A border with the places a note names its corner radius and line width by, and where its
/// stroke lies.
pub(crate) struct Outline {
    pub edge: Edge,
    pub paths: [&'static str; 2],
    pub align: LineAlign,
}

/// A panel's separator line (`noninteractive.bg_stroke`): its colour and width, with the place
/// a note names the width by.
pub(crate) struct PanelEdge {
    pub color: Option<Rgba>,
    pub width: Read<f32>,
}

/// A text colour and its size, scaled by the text scale.
pub(crate) struct Text {
    pub color: Option<Rgba>,
    pub size: Read<f32>,
}

pub(crate) struct ButtonCell {
    pub states: ButtonStates,
    pub pressed_label: Option<Rgba>,
    pub primary_fill: Option<Rgba>,
    pub primary_text: Option<Rgba>,
    pub min_height: Read<f32>,
    pub icon_gap: Read<f32>,
    pub padding: Sides,
    pub disabled_opacity: Read<f32>,
    pub size: Read<f32>,
}

pub(crate) struct MenuCell {
    pub hover_fill: Read<Rgba>,
    /// The item label at rest and hovered.
    pub text: [Option<Rgba>; 2],
    pub outline: Outline,
    /// The hovered item's outline.
    pub hover_border: Option<StateLine>,
    /// The menu bar title's horizontal and vertical padding, reported here where not finite.
    pub bar_padding: [Read<f32>; 2],
    /// The menu bar title's fill under the pointer and open.
    pub bar_fill: [Option<Rgba>; 2],
    /// The menu bar title's text under the pointer and open.
    pub bar_text: [Option<Rgba>; 2],
    pub separator: Option<Rgba>,
    pub padding: Sides,
    pub row_height: Read<f32>,
    pub min_row_height: Read<f32>,
    pub panel_padding: Sides,
    pub icon_gap: Read<f32>,
    pub size: Read<f32>,
}

pub(crate) struct InputCell {
    /// The text at rest, hovered and pressed.
    pub text: [Option<Rgba>; 3],
    pub outline: Outline,
    /// The hovered and the pressed field's border.
    pub state_border: [Option<StateLine>; 2],
    pub placeholder: Option<Rgba>,
    pub selection: Option<Rgba>,
    pub selection_text: Option<Rgba>,
    pub padding: Sides,
    pub size: Read<f32>,
    pub disabled_opacity: Read<f32>,
}

pub(crate) struct CheckboxCell {
    /// The unchecked box at rest, hovered and pressed.
    pub fill: [Read<Rgba>; 3],
    /// The check mark at rest, hovered and pressed.
    pub mark: [Option<Rgba>; 3],
    pub outline: Outline,
    /// The hovered and the pressed box's outline.
    pub state_border: [Option<StateLine>; 2],
    pub mark_width: Read<f32>,
    pub label: Text,
    pub indicator: Read<f32>,
    pub label_gap: Read<f32>,
    pub disabled_opacity: Read<f32>,
    /// The box's padding, the mark's inset.
    pub padding: Sides,
}

pub(crate) struct DialogCell {
    pub body: Text,
    pub title_size: Read<f32>,
    pub button_gap: Read<f32>,
}

pub(crate) struct ScrollbarCell {
    pub track: Option<Rgba>,
    /// The thumb at rest, hovered and pressed.
    pub thumb: [Read<Rgba>; 3],
    pub min_thumb: Read<f32>,
    /// The handle's and the trough's radius (§6.8).
    pub radius: Option<f32>,
}

pub(crate) struct SliderCell {
    pub fill: Option<Rgba>,
    pub track: Read<Rgba>,
    pub disabled_opacity: Read<f32>,
    pub thumb_diameter: Read<f32>,
    /// The thumb at rest, hovered and pressed.
    pub thumb: [Read<Rgba>; 3],
    /// The rail's radius (§6.8).
    pub radius: Option<f32>,
}

pub(crate) struct ProgressBarCell {
    pub fill: Option<Rgba>,
    pub track: Option<Rgba>,
    pub height: Read<f32>,
    pub gap: Read<f32>,
}

pub(crate) struct TabCell {
    /// The tab at rest, hovered and pressed.
    pub fill: [Read<Rgba>; 3],
    /// The label at rest, hovered and pressed.
    pub text: [Option<Rgba>; 3],
    pub outline: Outline,
    /// The hovered and pressed tab's outline where stated.
    pub state_border: [Option<StateLine>; 2],
    pub selected_fill: Option<Rgba>,
    pub selected_text: Option<Rgba>,
    pub bar_fill: Option<Rgba>,
    pub min_height: Read<f32>,
    pub item_gap: Read<f32>,
    pub padding: Sides,
    pub size: Read<f32>,
}

pub(crate) struct PanelCell {
    pub fill: Option<Rgba>,
    pub text: Text,
    pub edge: PanelEdge,
}

pub(crate) struct SidebarCell {
    pub panel: PanelCell,
    pub selection: Option<Rgba>,
    pub selection_text: Option<Rgba>,
    /// An item's hovered and pressed fill.
    pub item_fill: [Read<Rgba>; 2],
    /// An item's hovered and pressed label.
    pub item_text: [Option<Rgba>; 2],
}

pub(crate) struct ToolbarCell {
    pub panel: PanelCell,
    pub bar_height: Read<f32>,
    pub item_gap: Read<f32>,
    /// A tool button's hovered and pressed fill.
    pub button_fill: [Option<Rgba>; 2],
    /// A tool button's hovered and pressed outline.
    pub button_border: [Option<StateLine>; 2],
    /// A tool button's hovered and pressed label.
    pub button_text: [Option<Rgba>; 2],
    pub padding: Sides,
}

pub(crate) struct ListCell {
    pub selection: Option<Rgba>,
    pub selection_text: Option<Rgba>,
    pub grid: Option<Rgba>,
    pub width: Read<f32>,
    pub radius: Read<f32>,
    pub row_height: Read<f32>,
    /// A row's hovered and pressed highlight.
    pub highlight: [Read<Rgba>; 2],
    pub item: Text,
    pub header_text: Option<Rgba>,
}

pub(crate) struct SeparatorCell {
    pub color: Option<Rgba>,
    pub width: Read<f32>,
}

pub(crate) struct SwitchCell {
    pub checked: Option<Rgba>,
    /// The unchecked track at rest, hovered and pressed.
    pub fill: [Read<Rgba>; 3],
    pub radius: Read<f32>,
    pub height: Read<f32>,
    pub disabled_opacity: Read<f32>,
}

pub(crate) struct SpinnerCell {
    pub color: Option<Rgba>,
    pub diameter: Read<f32>,
    pub min_diameter: Read<f32>,
}

pub(crate) struct ComboBoxCell {
    /// The field at rest, hovered and pressed.
    pub fill: [Read<Rgba>; 3],
    pub text: Text,
    /// The label hovered and pressed.
    pub text_states: [Option<Rgba>; 2],
    pub outline: Outline,
    /// The hovered and the pressed field's border.
    pub state_border: [Option<StateLine>; 2],
    pub open_fill: Option<Rgba>,
    pub open_border: Option<Rgba>,
    pub min_height: Read<f32>,
    pub arrow: Read<f32>,
    pub arrow_area: Read<f32>,
    pub padding: Sides,
    pub disabled_opacity: Read<f32>,
}

pub(crate) struct SegmentedControlCell {
    /// The segment at rest, hovered and pressed.
    pub fill: [Read<Rgba>; 3],
    pub text: Text,
    /// The label hovered and pressed.
    pub text_states: [Option<Rgba>; 2],
    pub outline: Outline,
    /// The hovered and pressed segment's outline where stated.
    pub state_border: [Option<StateLine>; 2],
    pub selected_fill: Option<Rgba>,
    pub selected_text: Option<Rgba>,
    pub height: Read<f32>,
    pub separator: Read<f32>,
    pub padding: Sides,
    pub disabled_opacity: Read<f32>,
}

pub(crate) struct ExpanderCell {
    /// The header's hovered and pressed fill.
    pub fill: [Read<Rgba>; 2],
    pub text: Text,
    /// The header's label hovered and pressed.
    pub text_states: [Option<Rgba>; 2],
    pub outline: Outline,
    pub header_height: Read<f32>,
    pub padding: Sides,
    /// Whether a frame holds header and body together, the padding then lying inside its line.
    pub framed: bool,
    pub arrow: Read<f32>,
}

impl ExpanderCell {
    /// The header's padding measured from the outside of its border: each stated, finite side
    /// plus the border's width where a frame is drawn, the side alone where none is (the header
    /// draws no outline of its own, ledger M9-178; `convert::expander_padding_border`).
    pub(crate) fn outside_padding(&self) -> native_theme::theme::ResolvedPadding {
        let [top, right, bottom, left] = self.padding.values;
        let padding = native_theme::theme::ResolvedPadding {
            top,
            right,
            bottom,
            left,
        };
        match self.outline.edge.width {
            Some(width) if self.framed => padding_with_line(&padding, width),
            _ => padding_with_line(&padding, 0.0),
        }
    }
}

pub(crate) struct SplitterCell {
    pub color: Option<Rgba>,
    pub hover: Option<Rgba>,
    /// The drag line's colour.
    pub pressed: Option<Rgba>,
    pub width: Read<f32>,
}

/// Every role cell's reads, the text sizes scaled by the text scale.
pub(crate) struct CellSource {
    pub button: ButtonCell,
    pub menu: MenuCell,
    pub input: InputCell,
    pub checkbox: CheckboxCell,
    pub tooltip: Text,
    pub popover: Text,
    pub dialog: DialogCell,
    pub scrollbar: ScrollbarCell,
    pub slider: SliderCell,
    pub progress_bar: ProgressBarCell,
    pub tab: TabCell,
    pub sidebar: SidebarCell,
    pub toolbar: ToolbarCell,
    pub status_bar: PanelCell,
    pub list: ListCell,
    pub separator: SeparatorCell,
    pub switch: SwitchCell,
    pub spinner: SpinnerCell,
    pub combo_box: ComboBoxCell,
    pub segmented_control: SegmentedControlCell,
    pub expander: ExpanderCell,
    pub link: Text,
    pub splitter: SplitterCell,
    /// Whether the text scale is above 1, where a stated row height fits no padding.
    pub scaled_up: bool,
}

/// The four `<prefix>.border.padding.<side>` leaves of a 0.7.2 border.
fn old_sides(p: &native_theme::theme::ResolvedPadding, paths: [&'static str; 4]) -> Sides {
    Sides {
        values: [p.top, p.right, p.bottom, p.left],
        paths,
    }
}

/// A 0.7.2 border with its corner-radius and line-width leaves, its stroke `Outside`.
fn old_outline(b: &native_theme::theme::ResolvedWidgetBorder, paths: [&'static str; 2]) -> Outline {
    Outline {
        edge: Edge::old(b),
        paths,
        align: LineAlign::Outside,
    }
}

fn old_edge(b: &native_theme::theme::ResolvedWidgetBorder, width_path: &'static str) -> PanelEdge {
    PanelEdge {
        color: Some(b.color),
        width: Some((b.line_width, width_path)),
    }
}

impl CellSource {
    /// The 0.7.2 model's reads, every value stated but those the 0.7.2 model leaves optional.
    pub(crate) fn old(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> Self {
        let scaled = |size: f32, path| Some((scaled_text_size(size, prefs), path));
        let text = |f: &native_theme::theme::ResolvedFontSpec, path| Text {
            color: Some(f.color),
            size: scaled(f.size, path),
        };
        let b = &t.button;
        let m = &t.menu;
        let i = &t.input;
        let k = &t.checkbox;
        let sb = &t.scrollbar;
        let sl = &t.slider;
        let tb = &t.tab;
        let cb = &t.combo_box;
        let sc = &t.segmented_control;
        let x = &t.expander;
        let radius = Some(t.defaults.border.corner_radius);
        Self {
            button: ButtonCell {
                states: ButtonStates::old(t),
                pressed_label: Some(b.active_text_color),
                primary_fill: Some(b.primary_background),
                primary_text: Some(b.primary_text_color),
                min_height: Some((b.min_height, "button.min_height")),
                icon_gap: Some((b.icon_text_gap, "button.icon_text_gap")),
                padding: old_sides(
                    &b.border.padding,
                    [
                        "button.border.padding.top",
                        "button.border.padding.right",
                        "button.border.padding.bottom",
                        "button.border.padding.left",
                    ],
                ),
                disabled_opacity: Some((b.disabled_opacity, "button.disabled_opacity")),
                size: scaled(b.font.size, "button.font.size"),
            },
            menu: MenuCell {
                hover_fill: Some((m.hover_background, "menu.hover_background")),
                text: [Some(m.font.color), Some(m.hover_text_color)],
                outline: old_outline(
                    &m.border,
                    ["menu.border.corner_radius", "menu.border.line_width"],
                ),
                hover_border: StateLine::colour(m.hover_border_color),
                bar_padding: [
                    m.bar_padding_horizontal
                        .map(|v| (v, "menu.bar_padding_horizontal")),
                    m.bar_padding_vertical
                        .map(|v| (v, "menu.bar_padding_vertical")),
                ],
                bar_fill: [
                    m.bar_hover_background,
                    m.bar_open_background.or(m.bar_hover_background),
                ],
                bar_text: [m.bar_hover_text_color; 2],
                separator: Some(m.separator_color),
                padding: old_sides(
                    &m.border.padding,
                    [
                        "menu.border.padding.top",
                        "menu.border.padding.right",
                        "menu.border.padding.bottom",
                        "menu.border.padding.left",
                    ],
                ),
                row_height: m.row_height.map(|v| (v, "menu.row_height")),
                min_row_height: m.min_row_height.map(|v| (v, "menu.min_row_height")),
                panel_padding: Sides {
                    values: [
                        m.panel_padding_vertical,
                        m.panel_padding_horizontal,
                        m.panel_padding_vertical,
                        m.panel_padding_horizontal,
                    ],
                    paths: [
                        "menu.panel_padding_vertical",
                        "menu.panel_padding_horizontal",
                        "menu.panel_padding_vertical",
                        "menu.panel_padding_horizontal",
                    ],
                },
                icon_gap: Some((m.icon_text_gap, "menu.icon_text_gap")),
                size: scaled(m.font.size, "menu.font.size"),
            },
            input: InputCell {
                text: [Some(i.font.color), None, None],
                outline: old_outline(
                    &i.border,
                    ["input.border.corner_radius", "input.border.line_width"],
                ),
                // §6.4: `None` copies `border.color`; the pressed field keeps the hovered one
                state_border: [StateLine::colour(Some(
                    i.hover_border_color.unwrap_or(i.border.color),
                )); 2],
                placeholder: Some(i.placeholder_color),
                selection: Some(i.selection_background),
                selection_text: Some(i.selection_text_color),
                padding: old_sides(
                    &i.border.padding,
                    [
                        "input.border.padding.top",
                        "input.border.padding.right",
                        "input.border.padding.bottom",
                        "input.border.padding.left",
                    ],
                ),
                size: scaled(i.font.size, "input.font.size"),
                disabled_opacity: Some((i.disabled_opacity, "input.disabled_opacity")),
            },
            checkbox: CheckboxCell {
                // §6.4: the unchecked box and its outline fall back to the widget's own colours
                fill: [
                    Some(match k.unchecked_background {
                        Some(c) => (c, "checkbox.unchecked_background"),
                        None => (k.background_color, "checkbox.background_color"),
                    }),
                    k.hover_background.map(|h| (h, "checkbox.hover_background")),
                    None,
                ],
                mark: [Some(k.indicator_color), None, None],
                outline: Outline {
                    edge: Edge {
                        color: Some(k.unchecked_border_color.unwrap_or(k.border.color)),
                        ..Edge::old(&k.border)
                    },
                    paths: [
                        "checkbox.border.corner_radius",
                        "checkbox.border.line_width",
                    ],
                    align: LineAlign::Outside,
                },
                state_border: [StateLine::colour(k.hover_border_color); 2],
                mark_width: k
                    .check_mark_stroke_width
                    .map(|v| (v, "checkbox.check_mark_stroke_width")),
                label: text(&k.font, "checkbox.font.size"),
                indicator: Some((k.indicator_width, "checkbox.indicator_width")),
                label_gap: Some((k.label_gap, "checkbox.label_gap")),
                disabled_opacity: Some((k.disabled_opacity, "checkbox.disabled_opacity")),
                padding: old_sides(
                    &k.border.padding,
                    [
                        "checkbox.border.padding.top",
                        "checkbox.border.padding.right",
                        "checkbox.border.padding.bottom",
                        "checkbox.border.padding.left",
                    ],
                ),
            },
            tooltip: text(&t.tooltip.font, "tooltip.font.size"),
            popover: text(&t.popover.font, "popover.font.size"),
            dialog: DialogCell {
                body: text(&t.dialog.body_font, "dialog.body_font.size"),
                title_size: scaled(t.dialog.title_font.size, "dialog.title_font.size"),
                button_gap: Some((t.dialog.button_gap, "dialog.button_gap")),
            },
            scrollbar: ScrollbarCell {
                track: Some(sb.track_color),
                thumb: [
                    Some((sb.thumb_color, "scrollbar.thumb_color")),
                    Some((sb.thumb_hover_color, "scrollbar.thumb_hover_color")),
                    Some(match sb.thumb_active_color {
                        Some(c) => (c, "scrollbar.thumb_active_color"),
                        None => (sb.thumb_hover_color, "scrollbar.thumb_hover_color"),
                    }),
                ],
                min_thumb: Some((sb.min_thumb_length, "scrollbar.min_thumb_length")),
                radius,
            },
            slider: SliderCell {
                fill: Some(sl.fill_color),
                track: Some((sl.track_color, "slider.track_color")),
                disabled_opacity: Some((sl.disabled_opacity, "slider.disabled_opacity")),
                thumb_diameter: Some((sl.thumb_diameter, "slider.thumb_diameter")),
                thumb: [
                    Some((sl.thumb_color, "slider.thumb_color")),
                    sl.thumb_hover_color
                        .map(|c| (c, "slider.thumb_hover_color")),
                    None,
                ],
                radius,
            },
            progress_bar: ProgressBarCell {
                fill: Some(t.progress_bar.fill_color),
                track: Some(t.progress_bar.track_color),
                height: Some((t.progress_bar.track_height, "progress_bar.track_height")),
                gap: t
                    .progress_bar
                    .border
                    .padding
                    .left
                    .map(|v| (v, "progress_bar.border.padding.left")),
            },
            tab: TabCell {
                fill: [
                    Some((tb.background_color, "tab.background_color")),
                    tb.hover_background.map(|h| (h, "tab.hover_background")),
                    None,
                ],
                text: [Some(tb.font.color), Some(tb.hover_text_color), None],
                outline: old_outline(
                    &tb.border,
                    ["tab.border.corner_radius", "tab.border.line_width"],
                ),
                state_border: [None; 2],
                selected_fill: Some(tb.active_background),
                selected_text: Some(tb.active_text_color),
                bar_fill: Some(tb.bar_background),
                min_height: Some((tb.min_height, "tab.min_height")),
                item_gap: tb.item_gap.map(|v| (v, "tab.item_gap")),
                padding: old_sides(
                    &tb.border.padding,
                    [
                        "tab.border.padding.top",
                        "tab.border.padding.right",
                        "tab.border.padding.bottom",
                        "tab.border.padding.left",
                    ],
                ),
                size: scaled(tb.font.size, "tab.font.size"),
            },
            sidebar: SidebarCell {
                panel: PanelCell {
                    fill: Some(t.sidebar.background_color),
                    text: text(&t.sidebar.font, "sidebar.font.size"),
                    edge: old_edge(&t.sidebar.border, "sidebar.border.line_width"),
                },
                selection: Some(t.sidebar.selection_background),
                selection_text: Some(t.sidebar.selection_text_color),
                item_fill: [
                    Some((t.sidebar.hover_background, "sidebar.hover_background")),
                    None,
                ],
                item_text: [None; 2],
            },
            toolbar: ToolbarCell {
                panel: PanelCell {
                    fill: Some(t.toolbar.background_color),
                    text: text(&t.toolbar.font, "toolbar.font.size"),
                    edge: old_edge(&t.toolbar.border, "toolbar.border.line_width"),
                },
                bar_height: t.toolbar.bar_height.map(|v| (v, "toolbar.bar_height")),
                item_gap: t.toolbar.item_gap.map(|v| (v, "toolbar.item_gap")),
                button_fill: [
                    t.toolbar.button_hover_background,
                    t.toolbar.button_active_background,
                ],
                button_border: [StateLine::colour(t.toolbar.button_hover_border_color), None],
                button_text: [None; 2],
                padding: old_sides(
                    &t.toolbar.border.padding,
                    [
                        "toolbar.border.padding.top",
                        "toolbar.border.padding.right",
                        "toolbar.border.padding.bottom",
                        "toolbar.border.padding.left",
                    ],
                ),
            },
            status_bar: PanelCell {
                fill: Some(t.status_bar.background_color),
                text: text(&t.status_bar.font, "status_bar.font.size"),
                edge: old_edge(&t.status_bar.border, "status_bar.border.line_width"),
            },
            list: ListCell {
                selection: Some(t.list.selection_background),
                selection_text: Some(t.list.selection_text_color),
                grid: Some(t.list.grid_color),
                width: Some((t.list.border.line_width, "list.border.line_width")),
                radius: Some((t.list.border.corner_radius, "list.border.corner_radius")),
                row_height: t.list.row_height.map(|v| (v, "list.row_height")),
                highlight: [
                    Some((t.list.hover_background, "list.hover_background")),
                    None,
                ],
                item: text(&t.list.item_font, "list.item_font.size"),
                header_text: Some(t.list.header_font.color),
            },
            separator: SeparatorCell {
                color: Some(t.separator.line_color),
                width: Some((t.separator.line_width, "separator.line_width")),
            },
            switch: SwitchCell {
                checked: Some(t.switch.checked_background),
                fill: [
                    Some((t.switch.unchecked_background, "switch.unchecked_background")),
                    t.switch
                        .hover_unchecked_background
                        .map(|h| (h, "switch.hover_unchecked_background")),
                    None,
                ],
                radius: Some((t.switch.track_radius, "switch.track_radius")),
                height: Some((t.switch.track_height, "switch.track_height")),
                disabled_opacity: Some((t.switch.disabled_opacity, "switch.disabled_opacity")),
            },
            spinner: SpinnerCell {
                color: Some(t.spinner.fill_color),
                diameter: Some((t.spinner.diameter, "spinner.diameter")),
                min_diameter: Some((t.spinner.min_diameter, "spinner.min_diameter")),
            },
            combo_box: ComboBoxCell {
                fill: [
                    Some((cb.background_color, "combo_box.background_color")),
                    cb.hover_background
                        .map(|h| (h, "combo_box.hover_background")),
                    None,
                ],
                text: text(&cb.font, "combo_box.font.size"),
                text_states: [None; 2],
                outline: old_outline(
                    &cb.border,
                    [
                        "combo_box.border.corner_radius",
                        "combo_box.border.line_width",
                    ],
                ),
                state_border: [StateLine::colour(cb.hover_border_color); 2],
                open_fill: cb.open_background,
                open_border: cb.open_border_color,
                min_height: Some((cb.min_height, "combo_box.min_height")),
                arrow: Some((cb.arrow_icon_size, "combo_box.arrow_icon_size")),
                arrow_area: cb
                    .arrow_area_width
                    .map(|v| (v, "combo_box.arrow_area_width")),
                padding: old_sides(
                    &cb.border.padding,
                    [
                        "combo_box.border.padding.top",
                        "combo_box.border.padding.right",
                        "combo_box.border.padding.bottom",
                        "combo_box.border.padding.left",
                    ],
                ),
                disabled_opacity: Some((cb.disabled_opacity, "combo_box.disabled_opacity")),
            },
            segmented_control: SegmentedControlCell {
                fill: [
                    Some((sc.background_color, "segmented_control.background_color")),
                    sc.hover_background
                        .map(|h| (h, "segmented_control.hover_background")),
                    None,
                ],
                text: text(&sc.font, "segmented_control.font.size"),
                text_states: [None; 2],
                outline: old_outline(
                    &sc.border,
                    [
                        "segmented_control.border.corner_radius",
                        "segmented_control.border.line_width",
                    ],
                ),
                state_border: [None; 2],
                selected_fill: Some(sc.active_background),
                selected_text: Some(sc.active_text_color),
                height: Some((sc.segment_height, "segmented_control.segment_height")),
                separator: Some((sc.separator_width, "segmented_control.separator_width")),
                padding: old_sides(
                    &sc.border.padding,
                    [
                        "segmented_control.border.padding.top",
                        "segmented_control.border.padding.right",
                        "segmented_control.border.padding.bottom",
                        "segmented_control.border.padding.left",
                    ],
                ),
                disabled_opacity: Some((sc.disabled_opacity, "segmented_control.disabled_opacity")),
            },
            expander: ExpanderCell {
                fill: [
                    x.hover_background.map(|h| (h, "expander.hover_background")),
                    None,
                ],
                text: text(&x.font, "expander.font.size"),
                text_states: [None; 2],
                outline: old_outline(
                    &x.border,
                    [
                        "expander.border.corner_radius",
                        "expander.border.line_width",
                    ],
                ),
                header_height: Some((x.header_height, "expander.header_height")),
                padding: old_sides(
                    &x.border.padding,
                    [
                        "expander.border.padding.top",
                        "expander.border.padding.right",
                        "expander.border.padding.bottom",
                        "expander.border.padding.left",
                    ],
                ),
                framed: x.frame_enabled == Some(true),
                arrow: Some((x.arrow_icon_size, "expander.arrow_icon_size")),
            },
            link: text(&t.link.font, "link.font.size"),
            splitter: SplitterCell {
                color: Some(t.splitter.divider_color),
                hover: Some(t.splitter.hover_color),
                pressed: None,
                width: Some((t.splitter.divider_width, "splitter.divider_width")),
            },
            scaled_up: prefs.text_scale_factor() > 1.0,
        }
    }
}
