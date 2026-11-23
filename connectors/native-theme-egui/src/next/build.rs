//! The 0.8.0 model's reads for the mapping's shared builders (`crate::style`): each source
//! the 0.7.2 path fills from its fields, filled here at the homes the table of homes
//! (`native-theme/tests/next_homes.toml`) names, by the query its row names. A value the
//! theme states nothing for is `None`, and the builder leaves egui's own there (spec §10,
//! rule 0); a place a note names is the home's.

use native_theme::next::{
    ButtonQuery, Color, IconStated, IconStyle, ListQuery, Orientation, Paint, PartStated,
    PartStyle, ResolvedTheme, Stroke, StyleQuery, TextStated, TextStyle, WidgetState, WindowState,
};

use super::{padding_of, radius_of, stroke_width_of};
use crate::convert::Rgba;
use crate::style::base::{BaseSource, ButtonStates, Read, Sides};
use crate::style::cells::{
    ButtonCell, CellSource, CheckboxCell, ComboBoxCell, DialogCell, ExpanderCell, InputCell,
    ListCell, MenuCell, Outline, PanelCell, PanelEdge, ProgressBarCell, ScrollbarCell,
    SegmentedControlCell, SeparatorCell, SidebarCell, SliderCell, SpinnerCell, SplitterCell,
    SwitchCell, TabCell, Text, ToolbarCell,
};
use crate::style::line::LineAlign;
use crate::style::states::Edge;
use crate::style::variants::{CheckedBox, Disabled, VariantSource};

/// A colour of the 0.8.0 model as the 0.7.2 model's: its 8-bit sRGB channels with straight
/// alpha, which the builders convert as they convert a 0.7.2 colour.
pub(crate) fn rgba(c: Color) -> Rgba {
    let [r, g, b, a] = c.to_srgb8();
    Rgba { r, g, b, a }
}

/// A part's fill, where stated: its one colour, transparent for a fill stated as none, and a
/// layered paint's fallback colour (spec §10, rule 2); `None` for a layered paint without one.
pub(crate) fn fill(part: &PartStyle) -> Option<Rgba> {
    if !part.stated.contains(PartStated::FILL) {
        return None;
    }
    match &part.fill {
        Paint::None => Some(Rgba::TRANSPARENT),
        Paint::Solid(c) => Some(rgba(*c)),
        paint @ Paint::Complex(_) => paint.fallback_color().map(rgba),
    }
}

/// [`fill`] where a rule states it for the queried state (its `from_state` bit).
pub(crate) fn state_fill(part: &PartStyle) -> Option<Rgba> {
    part.from_state
        .contains(PartStated::FILL)
        .then(|| fill(part))
        .flatten()
}

/// [`fill`] with the home a note names it by.
fn fill_at(part: &PartStyle, home: &'static str) -> Read<Rgba> {
    fill(part).map(|c| (c, home))
}

/// [`state_fill`] with the home a note names it by.
fn state_fill_at(part: &PartStyle, home: &'static str) -> Read<Rgba> {
    state_fill(part).map(|c| (c, home))
}

/// A part's stroke colour, where stated as one stroke of one colour in an alignment the
/// connector knows.
pub(crate) fn line(part: &PartStyle) -> Option<Rgba> {
    match &part.stroke {
        Stroke::Uniform { color, align, .. }
            if part.stated.contains(PartStated::STROKE) && LineAlign::of(*align).is_some() =>
        {
            Some(rgba(*color))
        }
        _ => None,
    }
}

/// [`line`] where a rule states it for the queried state (its `from_state` bit).
pub(crate) fn state_line(part: &PartStyle) -> Option<Rgba> {
    part.from_state
        .contains(PartStated::STROKE)
        .then(|| line(part))
        .flatten()
}

/// A part's outline as the builders take it, and where its stroke lies (`Outside` for one
/// the theme states none of, or states as none, which composites nothing).
pub(crate) fn edge(part: &PartStyle) -> (Edge, LineAlign) {
    let align = match &part.stroke {
        Stroke::Uniform { align, .. } => LineAlign::of(*align),
        _ => None,
    };
    (
        Edge {
            color: line(part),
            width: stroke_width_of(part),
            radius: radius_of(part),
        },
        align.unwrap_or(LineAlign::Outside),
    )
}

/// A text style's colour, where stated.
pub(crate) fn text_color(text: &TextStyle) -> Option<Rgba> {
    text.stated
        .contains(TextStated::COLOR)
        .then(|| rgba(text.color))
}

/// [`text_color`] where a rule states it for the queried state (its `from_state` bit).
pub(crate) fn state_text_color(text: &TextStyle) -> Option<Rgba> {
    text.from_state
        .contains(TextStated::COLOR)
        .then(|| text_color(text))
        .flatten()
}

/// A text style's size in logical pixels, where stated, with its home.
fn size(text: &TextStyle, home: &'static str) -> Read<f32> {
    text.stated
        .contains(TextStated::SIZE_PX)
        .then_some((text.size_px, home))
}

/// A part's padding with the homes of its four sides, `<part>.padding.<side>`.
fn sides(part: &PartStyle, paths: [&'static str; 4]) -> Sides {
    Sides {
        values: padding_of(part),
        paths,
    }
}

/// Whether a part is drawn with a shadow: unless the theme states it has none (spec §5,
/// `0.7.2`'s `border.shadow_enabled`: `false` is `SHADOWS` stated with no layer).
fn shadowed(part: &PartStyle) -> bool {
    !(part.stated.contains(PartStated::SHADOWS)
        && part.shadows.as_ref().is_none_or(|layers| layers.is_empty()))
}

/// [`BaseSource`] of the 0.8.0 model. The scroll bar's thumb at rest is the handle of a view
/// without the focus, where 0.7.2's `scrollbar.thumb_color` lives on kde-breeze; a theme with
/// no rule for that state returns its plain thumb there. A caption or headline style the
/// typography does not define is the body's (the table's `home_fallback`).
pub(crate) fn base_source(r: &ResolvedTheme) -> BaseSource {
    let c = &r.colors;
    let t = &r.typography;
    let b = &r.button;
    let rest = b.body(WidgetState::NONE);
    let sb = &r.scrollbar;
    let vertical = StyleQuery::new(WidgetState::NONE).orientation(Orientation::Vertical);
    let horizontal = StyleQuery::new(WidgetState::NONE).orientation(Orientation::Horizontal);
    let frame = r.window.frame(WindowState::default());
    BaseSource {
        sizes: [
            size(
                t.caption.as_ref().unwrap_or(&t.body),
                "typography.caption.size_px",
            ),
            size(&t.body, "typography.body.size_px"),
            size(&t.monospace, "typography.monospace.size_px"),
            size(&b.label(WidgetState::NONE), "button.label(NONE).size_px"),
            size(
                t.headline.as_ref().unwrap_or(&t.body),
                "typography.headline.size_px",
            ),
        ],
        window_fill: c
            .window
            .background
            .map(|v| (rgba(v), "colors.window.background")),
        text: c.window.foreground.map(rgba),
        muted: c.window.foreground_muted.map(rgba),
        shadow: Some(rgba(c.shadow)),
        link: Some(rgba(c.link)),
        selection: Some(rgba(c.selection.background)),
        selection_text: Some(rgba(c.selection.foreground)),
        danger: Some(rgba(c.error.fill)),
        warning: Some(rgba(c.warning.fill)),
        border: Edge {
            color: c.window.border.map(rgba),
            width: Some(r.layout.border_width),
            radius: Some(r.layout.corner_radius),
        },
        border_paths: ["layout.corner_radius", "layout.border_width"],
        radius_lg: Some((r.layout.corner_radius_lg, "layout.corner_radius_lg")),
        disabled_opacity: Some((c.disabled_opacity, "colors.disabled_opacity")),
        menu_fill: fill(&r.menu.panel(WidgetState::NONE)),
        overlay_scrollbar: sb.overlay_mode,
        track: fill(&sb.track(WidgetState::NONE)),
        input_fill: fill(&r.input.field(WidgetState::NONE)),
        alternate_row: fill(
            &r.list
                .row(ListQuery::new(WidgetState::NONE).alternate(true)),
        ),
        caret: Some(rgba(c.caret)),
        blink_on_ms: r.caret.blink_on_ms,
        blink_off_ms: r.caret.blink_off_ms,
        window_radius: radius_of(&frame).map(|v| (v, "window.frame(normal).corners")),
        window_padding: sides(
            &frame,
            [
                "window.frame(normal).padding.top",
                "window.frame(normal).padding.right",
                "window.frame(normal).padding.bottom",
                "window.frame(normal).padding.left",
            ],
        ),
        min_height: rest.min_size[1].map(|v| (v, "button.body(NONE).min_size[1]")),
        button_padding: sides(
            &rest,
            [
                "button.body(NONE).padding.top",
                "button.body(NONE).padding.right",
                "button.body(NONE).padding.bottom",
                "button.body(NONE).padding.left",
            ],
        ),
        indicator_width: r.checkbox.indicator(WidgetState::NONE).min_size[0]
            .map(|v| (v, "checkbox.indicator(NONE).min_size[0]")),
        label_gap: Some((r.checkbox.label_gap, "checkbox.label_gap")),
        rail_height: r.slider.track(horizontal).min_size[1]
            .map(|v| (v, "slider.track(NONE; orientation=Horizontal).min_size[1]")),
        combo_width: r.combo_box.field(WidgetState::NONE).min_size[0]
            .map(|v| (v, "combo_box.field(NONE).min_size[0]")),
        tooltip_width: r.tooltip.panel.max_size[0].map(|v| (v, "tooltip.panel.max_size[0]")),
        min_thumb: Some((sb.min_thumb_length, "scrollbar.min_thumb_length")),
        widget_gap: r.layout.widget_gap,
        // the theme's milliseconds as egui's seconds (`egui/src/style.rs:928`)
        tooltip_delay: r
            .timings
            .tooltip_show_ms
            .map(|ms| (f64::from(ms) / 1000.0) as f32),
        button: button_states(r),
        thumb: thumb(r),
        title_bar_fill: fill(&r.window.title_bar(WindowState::default())),
        line_height: Some((
            t.line_height_multiplier,
            "typography.line_height_multiplier",
        )),
        groove_width: sb.track(vertical).min_size[0]
            .map(|v| (v, "scrollbar.track(NONE; orientation=Vertical).min_size[0]")),
        thumb_width: sb.thumb(vertical).min_size[0]
            .map(|v| (v, "scrollbar.thumb(NONE; orientation=Vertical).min_size[0]")),
        window_shadow: shadowed(&frame),
        popup_shadow: r.layout.shadow_enabled,
    }
}

/// The button's states the base style and the Button cell take.
fn button_states(r: &ResolvedTheme) -> ButtonStates {
    let b = &r.button;
    let rest = b.body(WidgetState::NONE);
    let hover = b.body(WidgetState::HOVER);
    let pressed = b.body(WidgetState::PRESSED);
    let (border, align) = edge(&rest);
    ButtonStates {
        fill: [
            fill_at(&rest, "button.body(NONE).fill"),
            state_fill_at(&hover, "button.body(HOVER).fill"),
            state_fill_at(&pressed, "button.body(PRESSED).fill"),
        ],
        label: [
            text_color(&b.label(WidgetState::NONE)),
            state_text_color(&b.label(WidgetState::HOVER)),
        ],
        border,
        border_paths: [
            "button.body(NONE).corners",
            "button.body(NONE).stroke.width",
        ],
        align,
        // egui's `active` is press and keyboard focus alike: the focus ring's border where
        // the theme states one, else the pressed one
        state_border: [
            state_line(&hover),
            state_line(&b.body(WidgetState::FOCUS_VISIBLE)).or_else(|| state_line(&pressed)),
        ],
    }
}

/// The scroll bar's thumb at rest (see [`base_source`]), hovered and pressed.
fn thumb(r: &ResolvedTheme) -> [Read<Rgba>; 3] {
    let sb = &r.scrollbar;
    [
        fill_at(
            &sb.thumb(WidgetState::UNFOCUSED_VIEW),
            "scrollbar.thumb(UNFOCUSED_VIEW).fill",
        ),
        state_fill_at(&sb.thumb(WidgetState::HOVER), "scrollbar.thumb(HOVER).fill"),
        state_fill_at(
            &sb.thumb(WidgetState::PRESSED),
            "scrollbar.thumb(PRESSED).fill",
        ),
    ]
}

/// A part's outline with the homes of its corners and stroke width.
fn outline(part: &PartStyle, paths: [&'static str; 2]) -> Outline {
    let (edge, align) = edge(part);
    Outline { edge, paths, align }
}

/// The menu item's outline: its corners where stated, else the panel's, the home the table of
/// homes names for `menu.border.corner_radius` on a theme that rounds the panel alone
/// (material-3), as iced's menu reads them.
fn menu_outline(item: &PartStyle, panel: &PartStyle) -> Outline {
    let mut outline = outline(
        item,
        ["menu.item(NONE).corners", "menu.item(NONE).stroke.width"],
    );
    if outline.edge.radius.is_none() {
        outline.edge.radius = radius_of(panel);
        outline.paths[0] = "menu.panel(NONE).corners";
    }
    outline
}

/// A panel part's separator line, with the home of its width.
fn panel_edge(part: &PartStyle, width_path: &'static str) -> PanelEdge {
    PanelEdge {
        color: line(part),
        width: stroke_width_of(part).map(|w| (w, width_path)),
    }
}

/// A text style's colour and size, with the home of the size.
fn text(t: &TextStyle, home: &'static str) -> Text {
    Text {
        color: text_color(t),
        size: size(t, home),
    }
}

/// An icon's colour, where stated.
fn icon_color(icon: &IconStyle) -> Option<Rgba> {
    icon.stated
        .contains(IconStated::COLOR)
        .then(|| rgba(icon.color))
}

/// An icon's size, where stated, with its home.
fn icon_size(icon: &IconStyle, home: &'static str) -> Read<f32> {
    icon.stated
        .contains(IconStated::SIZE)
        .then_some((icon.size, home))
}

/// A widget's disabled opacity, where stated, with its home.
fn opacity_at(v: Option<f32>, home: &'static str) -> Read<f32> {
    v.map(|v| (v, home))
}

/// [`CellSource`] of the 0.8.0 model. A checked, selected or open state's value is the one a
/// rule states for that state; a hover's or a press's as [`state_fill`] reads it.
pub(crate) fn cell_source(r: &ResolvedTheme) -> CellSource {
    use WidgetState as S;
    let b = &r.button;
    let rest = b.body(S::NONE);
    let primary = ButtonQuery::primary();
    let m = &r.menu;
    let item = m.item(S::NONE);
    let [bar_top, _, _, bar_left] = padding_of(&r.menu_bar.item(S::NONE));
    let i = &r.input;
    let field = i.field(S::NONE);
    let k = &r.checkbox;
    let indicator = k.indicator(S::NONE);
    let sb = &r.scrollbar;
    let sl = &r.slider;
    let p = &r.progress_bar;
    let track = p.track(S::NONE);
    let [_, _, _, track_left] = padding_of(&track);
    let tb = &r.tab;
    let tab = tb.tab(S::NONE);
    let side = &r.sidebar;
    let side_box = side.container(S::NONE);
    let tool = &r.toolbar;
    let bar = tool.bar(S::NONE);
    let l = &r.list;
    let list_box = l.container(S::NONE);
    let sw = &r.switch;
    let sw_track = sw.track(S::NONE);
    let cb = &r.combo_box;
    let cb_field = cb.field(S::NONE);
    let sc = &r.segmented_control;
    let segment = sc.segment(S::NONE);
    let sc_track = sc.track(S::NONE);
    let x = &r.expander;
    let header = x.header(S::NONE);
    CellSource {
        button: ButtonCell {
            states: button_states(r),
            pressed_label: state_text_color(&b.label(S::PRESSED)),
            primary_fill: fill(&b.body(primary)),
            primary_text: text_color(&b.label(primary)),
            min_height: rest.min_size[1].map(|v| (v, "button.body(NONE).min_size[1]")),
            icon_gap: Some((b.icon_text_gap, "button.icon_text_gap")),
            padding: sides(
                &rest,
                [
                    "button.body(NONE).padding.top",
                    "button.body(NONE).padding.right",
                    "button.body(NONE).padding.bottom",
                    "button.body(NONE).padding.left",
                ],
            ),
            disabled_opacity: opacity_at(b.disabled_opacity, "button.disabled_opacity"),
            size: size(&b.label(S::NONE), "button.label(NONE).size_px"),
        },
        menu: MenuCell {
            hover_fill: state_fill_at(&m.item(S::HOVER), "menu.item(HOVER).fill"),
            text: [
                text_color(&m.item_label(S::NONE)),
                state_text_color(&m.item_label(S::HOVER)),
            ],
            outline: menu_outline(&item, &m.panel(S::NONE)),
            hover_border: state_line(&m.item(S::HOVER)),
            bar_padding: [
                bar_left.map(|v| (v, "menu_bar.item(NONE).padding.left")),
                bar_top.map(|v| (v, "menu_bar.item(NONE).padding.top")),
            ],
            separator: fill(&m.separator(S::NONE)),
            padding: sides(
                &item,
                [
                    "menu.item(NONE).padding.top",
                    "menu.item(NONE).padding.right",
                    "menu.item(NONE).padding.bottom",
                    "menu.item(NONE).padding.left",
                ],
            ),
            // a height the item is held to is a row height; a least height alone, a floor
            row_height: item.max_size[1].map(|v| (v, "menu.item(NONE).max_size[1]")),
            min_row_height: item.min_size[1].map(|v| (v, "menu.item(NONE).min_size[1]")),
            panel_padding: sides(
                &m.panel(S::NONE),
                [
                    "menu.panel(NONE).padding.top",
                    "menu.panel(NONE).padding.right",
                    "menu.panel(NONE).padding.bottom",
                    "menu.panel(NONE).padding.left",
                ],
            ),
            icon_gap: Some((m.icon_text_gap, "menu.icon_text_gap")),
            size: size(&m.item_label(S::NONE), "menu.item_label(NONE).size_px"),
        },
        input: InputCell {
            text: text_color(&i.text(S::NONE)),
            outline: outline(
                &field,
                [
                    "input.field(NONE).corners",
                    "input.field(NONE).stroke.width",
                ],
            ),
            state_border: [
                state_line(&i.field(S::HOVER)),
                state_line(&i.field(S::PRESSED)),
            ],
            placeholder: text_color(&i.placeholder(S::NONE)),
            selection: Some(rgba(r.colors.text_selection.background)),
            selection_text: Some(rgba(r.colors.text_selection.foreground)),
            padding: sides(
                &field,
                [
                    "input.field(NONE).padding.top",
                    "input.field(NONE).padding.right",
                    "input.field(NONE).padding.bottom",
                    "input.field(NONE).padding.left",
                ],
            ),
            size: size(&i.text(S::NONE), "input.text(NONE).size_px"),
            disabled_opacity: opacity_at(i.disabled_opacity, "input.disabled_opacity"),
        },
        checkbox: CheckboxCell {
            fill: [
                fill_at(&indicator, "checkbox.indicator(NONE).fill"),
                state_fill_at(&k.indicator(S::HOVER), "checkbox.indicator(HOVER).fill"),
            ],
            mark: icon_color(&k.checkmark(S::CHECKED)),
            outline: outline(
                &indicator,
                [
                    "checkbox.indicator(NONE).corners",
                    "checkbox.indicator(NONE).stroke.width",
                ],
            ),
            state_border: [
                state_line(&k.indicator(S::HOVER)),
                state_line(&k.indicator(S::PRESSED)),
            ],
            mark_width: k
                .checkmark(S::NONE)
                .stroke_width
                .map(|v| (v, "checkbox.checkmark(NONE).stroke_width")),
            label: text(&k.label(S::NONE), "checkbox.label(NONE).size_px"),
            indicator: indicator.min_size[0].map(|v| (v, "checkbox.indicator(NONE).min_size[0]")),
            label_gap: Some((k.label_gap, "checkbox.label_gap")),
            disabled_opacity: opacity_at(k.disabled_opacity, "checkbox.disabled_opacity"),
            padding: sides(
                &indicator,
                [
                    "checkbox.indicator(NONE).padding.top",
                    "checkbox.indicator(NONE).padding.right",
                    "checkbox.indicator(NONE).padding.bottom",
                    "checkbox.indicator(NONE).padding.left",
                ],
            ),
        },
        tooltip: text(&r.tooltip.text, "tooltip.text.size_px"),
        popover: text(&r.popover.text, "popover.text.size_px"),
        dialog: DialogCell {
            body: text(&r.dialog.body, "dialog.body.size_px"),
            title_size: size(&r.dialog.title, "dialog.title.size_px"),
            button_gap: Some((r.dialog.button_gap, "dialog.button_gap")),
        },
        scrollbar: ScrollbarCell {
            track: fill(&sb.track(S::NONE)),
            thumb: thumb(r),
            min_thumb: Some((sb.min_thumb_length, "scrollbar.min_thumb_length")),
            radius: Some(r.layout.corner_radius),
        },
        slider: SliderCell {
            fill: fill(&sl.active_track(S::NONE)),
            track: fill_at(&sl.track(S::NONE), "slider.track(NONE).fill"),
            disabled_opacity: opacity_at(sl.disabled_opacity, "slider.disabled_opacity"),
            thumb_diameter: sl.thumb(S::NONE).min_size[0]
                .map(|v| (v, "slider.thumb(NONE).min_size[0]")),
            thumb: [
                fill_at(&sl.thumb(S::NONE), "slider.thumb(NONE).fill"),
                state_fill_at(&sl.thumb(S::HOVER), "slider.thumb(HOVER).fill"),
            ],
            radius: Some(r.layout.corner_radius),
        },
        progress_bar: ProgressBarCell {
            fill: fill(&p.fill(S::NONE)),
            track: fill(&track),
            height: track.min_size[1].map(|v| (v, "progress_bar.track(NONE).min_size[1]")),
            gap: track_left.map(|v| (v, "progress_bar.track(NONE).padding.left")),
        },
        tab: TabCell {
            fill: [
                fill_at(&tab, "tab.tab(NONE).fill"),
                state_fill_at(&tb.tab(S::HOVER), "tab.tab(HOVER).fill"),
            ],
            text: [
                text_color(&tb.label(S::NONE)),
                state_text_color(&tb.label(S::HOVER)),
            ],
            // the selected tab's outline is the tab's (the table of homes' `tab.border` rows)
            outline: outline(
                &tb.tab(S::SELECTED),
                [
                    "tab.tab(SELECTED).corners",
                    "tab.tab(SELECTED).stroke.width",
                ],
            ),
            selected_fill: state_fill(&tb.tab(S::SELECTED)),
            selected_text: state_text_color(&tb.label(S::SELECTED)),
            bar_fill: fill(&tb.bar(S::NONE)),
            min_height: tab.min_size[1].map(|v| (v, "tab.tab(NONE).min_size[1]")),
            item_gap: tb.item_gap.map(|v| (v, "tab.item_gap")),
            padding: sides(
                &tab,
                [
                    "tab.tab(NONE).padding.top",
                    "tab.tab(NONE).padding.right",
                    "tab.tab(NONE).padding.bottom",
                    "tab.tab(NONE).padding.left",
                ],
            ),
            size: size(&tb.label(S::NONE), "tab.label(NONE).size_px"),
        },
        sidebar: SidebarCell {
            panel: PanelCell {
                fill: fill(&side_box),
                text: text(
                    &side.item_label(S::NONE),
                    "sidebar.item_label(NONE).size_px",
                ),
                edge: panel_edge(&side_box, "sidebar.container(NONE).stroke.width"),
            },
            selection: state_fill(&side.item(S::SELECTED)),
            selection_text: state_text_color(&side.item_label(S::SELECTED)),
            hover: state_fill_at(&side.item(S::HOVER), "sidebar.item(HOVER).fill"),
        },
        toolbar: ToolbarCell {
            panel: PanelCell {
                fill: fill(&bar),
                text: text(
                    &tool.button_label(S::NONE),
                    "toolbar.button_label(NONE).size_px",
                ),
                edge: panel_edge(&bar, "toolbar.bar(NONE).stroke.width"),
            },
            bar_height: bar.min_size[1].map(|v| (v, "toolbar.bar(NONE).min_size[1]")),
            item_gap: tool.item_gap.map(|v| (v, "toolbar.item_gap")),
            button_fill: [
                state_fill(&tool.button(S::HOVER)),
                state_fill(&tool.button(S::PRESSED)),
            ],
            button_border: state_line(&tool.button(S::HOVER)),
            padding: sides(
                &bar,
                [
                    "toolbar.bar(NONE).padding.top",
                    "toolbar.bar(NONE).padding.right",
                    "toolbar.bar(NONE).padding.bottom",
                    "toolbar.bar(NONE).padding.left",
                ],
            ),
        },
        status_bar: PanelCell {
            fill: fill(&r.status_bar.bar),
            text: text(&r.status_bar.label, "status_bar.label.size_px"),
            edge: panel_edge(&r.status_bar.bar, "status_bar.bar.stroke.width"),
        },
        list: ListCell {
            selection: state_fill(&l.highlight(S::SELECTED)),
            selection_text: state_text_color(&l.item_label(S::SELECTED)),
            grid: Some(rgba(l.grid_color)),
            width: stroke_width_of(&list_box).map(|v| (v, "list.container(NONE).stroke.width")),
            radius: radius_of(&list_box).map(|v| (v, "list.container(NONE).corners")),
            row_height: l.row(S::NONE).min_size[1].map(|v| (v, "list.row(NONE).min_size[1]")),
            hover: state_fill_at(&l.highlight(S::HOVER), "list.highlight(HOVER).fill"),
            item: text(&l.item_label(S::NONE), "list.item_label(NONE).size_px"),
            header_text: text_color(&l.header_label(S::NONE)),
        },
        separator: SeparatorCell {
            color: Some(rgba(r.separator.color)),
            width: Some((r.separator.thickness, "separator.thickness")),
        },
        switch: SwitchCell {
            checked: state_fill(&sw.track(S::CHECKED)),
            fill: [
                fill_at(&sw_track, "switch.track(NONE).fill"),
                state_fill_at(&sw.track(S::HOVER), "switch.track(HOVER).fill"),
            ],
            radius: radius_of(&sw_track).map(|v| (v, "switch.track(NONE).corners")),
            height: sw_track.min_size[1].map(|v| (v, "switch.track(NONE).min_size[1]")),
            disabled_opacity: opacity_at(sw.disabled_opacity, "switch.disabled_opacity"),
        },
        spinner: SpinnerCell {
            color: Some(rgba(r.spinner.color)),
            diameter: Some((r.spinner.diameter, "spinner.diameter")),
            min_diameter: Some((r.spinner.min_diameter, "spinner.min_diameter")),
        },
        combo_box: ComboBoxCell {
            fill: [
                fill_at(&cb_field, "combo_box.field(NONE).fill"),
                state_fill_at(&cb.field(S::HOVER), "combo_box.field(HOVER).fill"),
            ],
            text: text(&cb.label(S::NONE), "combo_box.label(NONE).size_px"),
            outline: outline(
                &cb_field,
                [
                    "combo_box.field(NONE).corners",
                    "combo_box.field(NONE).stroke.width",
                ],
            ),
            state_border: [
                state_line(&cb.field(S::HOVER)),
                state_line(&cb.field(S::PRESSED)),
            ],
            open_fill: state_fill(&cb.field(S::OPEN)),
            open_border: state_line(&cb.field(S::OPEN)),
            min_height: cb_field.min_size[1].map(|v| (v, "combo_box.field(NONE).min_size[1]")),
            arrow: icon_size(&cb.chevron(S::NONE), "combo_box.chevron(NONE).size"),
            arrow_area: cb
                .arrow_area_width
                .map(|v| (v, "combo_box.arrow_area_width")),
            padding: sides(
                &cb_field,
                [
                    "combo_box.field(NONE).padding.top",
                    "combo_box.field(NONE).padding.right",
                    "combo_box.field(NONE).padding.bottom",
                    "combo_box.field(NONE).padding.left",
                ],
            ),
            disabled_opacity: opacity_at(cb.disabled_opacity, "combo_box.disabled_opacity"),
        },
        segmented_control: SegmentedControlCell {
            fill: [
                fill_at(&sc_track, "segmented_control.track(NONE).fill"),
                state_fill_at(
                    &sc.segment(S::HOVER),
                    "segmented_control.segment(HOVER).fill",
                ),
            ],
            text: text(&sc.label(S::NONE), "segmented_control.label(NONE).size_px"),
            outline: outline(
                &sc_track,
                [
                    "segmented_control.track(NONE).corners",
                    "segmented_control.track(NONE).stroke.width",
                ],
            ),
            selected_fill: state_fill(&sc.segment(S::SELECTED)),
            selected_text: state_text_color(&sc.label(S::SELECTED)),
            height: segment.min_size[1].map(|v| (v, "segmented_control.segment(NONE).min_size[1]")),
            separator: Some((sc.separator_width, "segmented_control.separator_width")),
            padding: sides(
                &segment,
                [
                    "segmented_control.segment(NONE).padding.top",
                    "segmented_control.segment(NONE).padding.right",
                    "segmented_control.segment(NONE).padding.bottom",
                    "segmented_control.segment(NONE).padding.left",
                ],
            ),
            disabled_opacity: opacity_at(sc.disabled_opacity, "segmented_control.disabled_opacity"),
        },
        expander: ExpanderCell {
            hover: state_fill_at(&x.header(S::HOVER), "expander.header(HOVER).fill"),
            text: text(
                &x.header_label(S::NONE),
                "expander.header_label(NONE).size_px",
            ),
            outline: outline(
                &x.container(S::NONE),
                [
                    "expander.container(NONE).corners",
                    "expander.container(NONE).stroke.width",
                ],
            ),
            header_height: header.min_size[1].map(|v| (v, "expander.header(NONE).min_size[1]")),
            padding: sides(
                &header,
                [
                    "expander.header(NONE).padding.top",
                    "expander.header(NONE).padding.right",
                    "expander.header(NONE).padding.bottom",
                    "expander.header(NONE).padding.left",
                ],
            ),
            framed: x.frame_enabled == Some(true),
            arrow: icon_size(&x.arrow(S::NONE), "expander.arrow(NONE).size"),
        },
        link: text(&r.link.label(S::NONE), "link.label(NONE).size_px"),
        splitter: SplitterCell {
            color: fill(&r.splitter.handle(S::NONE)),
            hover: state_fill(&r.splitter.handle(S::HOVER)),
            width: Some((r.splitter.divider_width, "splitter.divider_width")),
        },
        scaled_up: r.context.text_scale > 1.0,
    }
}

/// A role's disabled look: the label's and the fill's values a rule states for the disabled
/// state, and the role's resting outline that fill is drawn under.
fn disabled(label: &TextStyle, part: &PartStyle, home: &'static str, rest: &PartStyle) -> Disabled {
    let (outline, align) = edge(rest);
    Disabled {
        text: state_text_color(label),
        fill: state_fill_at(part, home),
        outline: outline.color,
        align,
    }
}

/// [`VariantSource`] of the 0.8.0 model. A disabled or checked value is the one a rule states
/// for that state. The checked box under the pointer or pressed keeps egui's own treatment of
/// the hover or the press unless the theme states one for it (spec §2, `from_state`: the
/// state asked alone), and is then the combined state's value; a checked box's disabled fill
/// likewise.
pub(crate) fn variant_source(r: &ResolvedTheme) -> VariantSource {
    use WidgetState as S;
    let b = &r.button;
    let cb = &r.combo_box;
    let k = &r.checkbox;
    let i = &r.input;
    let checked = k.indicator(S::CHECKED);
    let (checked_outline, checked_align) = edge(&checked);
    // the combined state's outline where a rule states one for `state` alone
    let checked_state_line = |state: WidgetState| {
        state_line(&k.indicator(state)).and_then(|_| line(&k.indicator(state | S::CHECKED)))
    };
    VariantSource {
        button: disabled(
            &b.label(S::DISABLED),
            &b.body(S::DISABLED),
            "button.body(DISABLED).fill",
            &b.body(S::NONE),
        ),
        primary: state_fill(&b.body(S::DISABLED)),
        combo_box: disabled(
            &cb.label(S::DISABLED),
            &cb.field(S::DISABLED),
            "combo_box.field(DISABLED).fill",
            &cb.field(S::NONE),
        ),
        checkbox: disabled(
            &k.label(S::DISABLED),
            &k.indicator(S::DISABLED),
            "checkbox.indicator(DISABLED).fill",
            &k.indicator(S::NONE),
        ),
        checked: CheckedBox {
            fill: state_fill_at(&checked, "checkbox.indicator(CHECKED).fill"),
            outline: checked_outline.color,
            align: checked_align,
            state_border: [checked_state_line(S::HOVER), checked_state_line(S::PRESSED)],
            disabled_fill: state_fill(&k.indicator(S::DISABLED)).and_then(|_| {
                fill_at(
                    &k.indicator(S::DISABLED | S::CHECKED),
                    "checkbox.indicator(DISABLED|CHECKED).fill",
                )
            }),
        },
        input: Disabled {
            text: state_text_color(&i.text(S::DISABLED)),
            fill: state_fill_at(&i.field(S::DISABLED), "input.field(DISABLED).fill"),
            outline: None,
            align: LineAlign::Outside,
        },
        slider_track: state_fill_at(&r.slider.track(S::DISABLED), "slider.track(DISABLED).fill"),
        slider_fill: state_fill(&r.slider.active_track(S::DISABLED)),
        switch: [
            state_fill(&r.switch.track(S::DISABLED)),
            state_fill(&r.switch.track(S::DISABLED | S::CHECKED)),
        ],
        menu: state_text_color(&r.menu.item_label(S::DISABLED)),
        list: state_text_color(&r.list.item_label(S::DISABLED)),
        link: state_text_color(&r.link.label(S::DISABLED)),
    }
}
