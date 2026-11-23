//! The 0.8.0 model's reads for the mapping's shared builders (`crate::style`): each source
//! the 0.7.2 path fills from its fields, filled here at the homes the table of homes
//! (`native-theme/tests/next_homes.toml`) names, by the query its row names. A value the
//! theme states nothing for is `None`, and the builder leaves egui's own there (spec §10,
//! rule 0); a place a note names is the home's.

use native_theme::next::{
    ButtonQuery, Color, FocusKind, FocusTrigger, IconStated, IconStyle, ListQuery, Orientation,
    PartStated, PartStyle, Preset, ResolvedTheme, Stroke, StyleQuery, TextStated, TextStyle,
    WidgetState, WindowState,
};

use super::{Mapped, Unmapped, padding_of, radius_of, reads, stroke_width_of};
use crate::convert::Rgba;
use crate::plugin::FocusRingSource;
use crate::style::Sources;
use crate::style::base::{BaseSource, ButtonStates, DropShadow, Read, Sides};
use crate::style::cells::{
    ButtonCell, CellSource, CheckboxCell, ComboBoxCell, DialogCell, ExpanderCell, InputCell,
    ListCell, MenuCell, Outline, PanelCell, PanelEdge, ProgressBarCell, ScrollbarCell,
    SegmentedControlCell, SeparatorCell, SidebarCell, SliderCell, SpinnerCell, SplitterCell,
    SwitchCell, TabCell, Text, ToolbarCell,
};
use crate::style::frames::{FrameSource, SurfaceSource};
use crate::style::line::LineAlign;
use crate::style::states::{Edge, StateLine};
use crate::style::variants::{CheckedBox, Disabled, PrimaryDisabled, VariantSource};

/// A colour of the 0.8.0 model as the 0.7.2 model's: its 8-bit sRGB channels with straight
/// alpha, which the builders convert as they convert a 0.7.2 colour.
pub(crate) fn rgba(c: Color) -> Rgba {
    let [r, g, b, a] = c.to_srgb8();
    Rgba { r, g, b, a }
}

/// A part's fill, where stated ([`reads::fill`]).
pub(crate) fn fill(part: &PartStyle) -> Option<Rgba> {
    reads::fill(part).map(rgba)
}

/// [`fill`] where a rule states it for the queried state ([`reads::state_fill`]).
pub(crate) fn state_fill(part: &PartStyle) -> Option<Rgba> {
    reads::state_fill(part).map(rgba)
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
        Stroke::Uniform { align, .. } if LineAlign::of(*align).is_some() => {
            reads::stroke(part).map(rgba)
        }
        _ => None,
    }
}

/// The border a rule states for the queried state (its `from_state` bit), as [`line_of`] reads
/// it; `None` where no rule states the stroke.
pub(crate) fn state_line(part: &PartStyle, home: &'static str) -> Option<StateLine> {
    part.from_state
        .contains(PartStated::STROKE)
        .then(|| line_of(part, home))
}

/// A part's border as a state's: its colour as [`line()`] reads it, transparent for a stroke
/// stated as none (spec §10, rule 2), and its width, with `home`, the width's.
fn line_of(part: &PartStyle, home: &'static str) -> StateLine {
    let color = match part.stroke {
        Stroke::None if part.stated.contains(PartStated::STROKE) => Some(rgba(Color::TRANSPARENT)),
        _ => line(part),
    };
    StateLine {
        color,
        width: stroke_width_of(part).map(|w| (w, home)),
    }
}

/// [`line()`] where a rule states it for the queried state (its `from_state` bit).
pub(crate) fn state_line_color(part: &PartStyle) -> Option<Rgba> {
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
    reads::text(text).map(rgba)
}

/// [`text_color`] where a rule states it for the queried state ([`reads::state_text`]).
pub(crate) fn state_text_color(text: &TextStyle) -> Option<Rgba> {
    reads::state_text(text).map(rgba)
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
/// `0.7.2`'s `border.shadow_enabled`: `false` is `SHADOWS` stated with no layer), or none egui
/// draws (only inset layers).
fn shadowed(part: &PartStyle) -> bool {
    !part.stated.contains(PartStated::SHADOWS) || drop_shadow(part, "").is_some()
}

/// A part's primary drop shadow layer, its first outset one (spec §10, rule 2), where the theme
/// states its shadows, with `home`, the shadows'.
fn drop_shadow(part: &PartStyle, home: &'static str) -> Option<DropShadow> {
    if !part.stated.contains(PartStated::SHADOWS) {
        return None;
    }
    let layer = part.shadows.as_deref()?.iter().find(|layer| !layer.inset)?;
    Some(DropShadow {
        offset: [layer.offset_x, layer.offset_y],
        blur: layer.blur,
        spread: layer.spread,
        color: rgba(layer.color),
        home,
    })
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
    let menu_panel = r.menu.panel(WidgetState::NONE);
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
        window_drop: drop_shadow(&frame, "window.frame(normal).shadows"),
        // egui's menus and popups: the menu's panel where it states its shadows (spec §5, notes
        // to the table, the menu's popup frame), else the popover's
        popup_drop: if menu_panel.stated.contains(PartStated::SHADOWS) {
            drop_shadow(&menu_panel, "menu.panel(NONE).shadows")
        } else {
            drop_shadow(&r.popover.panel, "popover.panel.shadows")
        },
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
            state_line(&hover, "button.body(HOVER).stroke.width"),
            state_line(
                &b.body(WidgetState::FOCUS_VISIBLE),
                "button.body(FOCUS_VISIBLE).stroke.width",
            )
            .or_else(|| state_line(&pressed, "button.body(PRESSED).stroke.width")),
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

/// The menu item's outline: its corners where stated, else, on the preset the table of homes
/// names `menu.panel(NONE).corners` the home of `menu.border.corner_radius` on (material-3,
/// whose menu rounds the panel alone), the panel's.
fn menu_outline(r: &ResolvedTheme) -> Outline {
    let m = &r.menu;
    let mut outline = outline(
        &m.item(WidgetState::NONE),
        ["menu.item(NONE).corners", "menu.item(NONE).stroke.width"],
    );
    if outline.edge.radius.is_none() && r.preset == Preset::Material3 {
        outline.edge.radius = radius_of(&m.panel(WidgetState::NONE));
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
    reads::icon(icon).map(rgba)
}

/// [`icon_color`] where a rule states it for the queried state ([`reads::state_icon`]).
fn state_icon_color(icon: &IconStyle) -> Option<Rgba> {
    reads::state_icon(icon).map(rgba)
}

/// A text style's hovered and pressed colours where a rule states them for the state, the
/// text style `at` gives for a state.
fn hover_press_text(at: impl Fn(WidgetState) -> TextStyle) -> [Option<Rgba>; 2] {
    [
        state_text_color(&at(WidgetState::HOVER)),
        state_text_color(&at(WidgetState::PRESSED)),
    ]
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
            outline: menu_outline(r),
            hover_border: state_line(&m.item(S::HOVER), "menu.item(HOVER).stroke.width"),
            bar_padding: [
                bar_left.map(|v| (v, "menu_bar.item(NONE).padding.left")),
                bar_top.map(|v| (v, "menu_bar.item(NONE).padding.top")),
            ],
            bar_fill: [
                state_fill(&r.menu_bar.item(S::HOVER)),
                state_fill(&r.menu_bar.item(S::OPEN)),
            ],
            bar_text: [
                state_text_color(&r.menu_bar.item_label(S::HOVER)),
                state_text_color(&r.menu_bar.item_label(S::OPEN)),
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
            text: {
                let [hover, press] = hover_press_text(|s| i.text(s));
                [text_color(&i.text(S::NONE)), hover, press]
            },
            outline: outline(
                &field,
                [
                    "input.field(NONE).corners",
                    "input.field(NONE).stroke.width",
                ],
            ),
            state_border: [
                state_line(&i.field(S::HOVER), "input.field(HOVER).stroke.width"),
                state_line(&i.field(S::PRESSED), "input.field(PRESSED).stroke.width"),
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
                state_fill_at(&k.indicator(S::PRESSED), "checkbox.indicator(PRESSED).fill"),
            ],
            mark: [
                icon_color(&k.checkmark(S::CHECKED)),
                state_icon_color(&k.checkmark(S::HOVER)),
                state_icon_color(&k.checkmark(S::PRESSED)),
            ],
            outline: outline(
                &indicator,
                [
                    "checkbox.indicator(NONE).corners",
                    "checkbox.indicator(NONE).stroke.width",
                ],
            ),
            state_border: [
                state_line(
                    &k.indicator(S::HOVER),
                    "checkbox.indicator(HOVER).stroke.width",
                ),
                state_line(
                    &k.indicator(S::PRESSED),
                    "checkbox.indicator(PRESSED).stroke.width",
                ),
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
                state_fill_at(&sl.thumb(S::PRESSED), "slider.thumb(PRESSED).fill"),
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
                state_fill_at(&tb.tab(S::PRESSED), "tab.tab(PRESSED).fill"),
            ],
            text: [
                text_color(&tb.label(S::NONE)),
                state_text_color(&tb.label(S::HOVER)),
                state_text_color(&tb.label(S::PRESSED)),
            ],
            // the selected tab's outline is the tab's (the table of homes' `tab.border` rows)
            outline: outline(
                &tb.tab(S::SELECTED),
                [
                    "tab.tab(SELECTED).corners",
                    "tab.tab(SELECTED).stroke.width",
                ],
            ),
            state_border: [
                state_line(&tb.tab(S::HOVER), "tab.tab(HOVER).stroke.width"),
                state_line(&tb.tab(S::PRESSED), "tab.tab(PRESSED).stroke.width"),
            ],
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
            item_fill: [
                state_fill_at(&side.item(S::HOVER), "sidebar.item(HOVER).fill"),
                state_fill_at(&side.item(S::PRESSED), "sidebar.item(PRESSED).fill"),
            ],
            item_text: hover_press_text(|s| side.item_label(s)),
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
            button_border: [
                state_line(&tool.button(S::HOVER), "toolbar.button(HOVER).stroke.width"),
                state_line(
                    &tool.button(S::PRESSED),
                    "toolbar.button(PRESSED).stroke.width",
                ),
            ],
            button_text: hover_press_text(|s| tool.button_label(s)),
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
            highlight: [
                state_fill_at(&l.highlight(S::HOVER), "list.highlight(HOVER).fill"),
                state_fill_at(&l.highlight(S::PRESSED), "list.highlight(PRESSED).fill"),
            ],
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
                state_fill_at(&sw.track(S::PRESSED), "switch.track(PRESSED).fill"),
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
                state_fill_at(&cb.field(S::PRESSED), "combo_box.field(PRESSED).fill"),
            ],
            text: text(&cb.label(S::NONE), "combo_box.label(NONE).size_px"),
            text_states: hover_press_text(|s| cb.label(s)),
            outline: outline(
                &cb_field,
                [
                    "combo_box.field(NONE).corners",
                    "combo_box.field(NONE).stroke.width",
                ],
            ),
            state_border: [
                state_line(&cb.field(S::HOVER), "combo_box.field(HOVER).stroke.width"),
                state_line(
                    &cb.field(S::PRESSED),
                    "combo_box.field(PRESSED).stroke.width",
                ),
            ],
            open_fill: state_fill(&cb.field(S::OPEN)),
            open_border: state_line_color(&cb.field(S::OPEN)),
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
                state_fill_at(
                    &sc.segment(S::PRESSED),
                    "segmented_control.segment(PRESSED).fill",
                ),
            ],
            text: text(&sc.label(S::NONE), "segmented_control.label(NONE).size_px"),
            text_states: hover_press_text(|s| sc.label(s)),
            outline: outline(
                &sc_track,
                [
                    "segmented_control.track(NONE).corners",
                    "segmented_control.track(NONE).stroke.width",
                ],
            ),
            state_border: [
                state_line(
                    &sc.segment(S::HOVER),
                    "segmented_control.segment(HOVER).stroke.width",
                ),
                state_line(
                    &sc.segment(S::PRESSED),
                    "segmented_control.segment(PRESSED).stroke.width",
                ),
            ],
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
            fill: [
                state_fill_at(&x.header(S::HOVER), "expander.header(HOVER).fill"),
                state_fill_at(&x.header(S::PRESSED), "expander.header(PRESSED).fill"),
            ],
            text: text(
                &x.header_label(S::NONE),
                "expander.header_label(NONE).size_px",
            ),
            text_states: hover_press_text(|s| x.header_label(s)),
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
            pressed: state_fill(&r.splitter.handle(S::PRESSED)),
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
/// for that state. The checked box's fill and outline under the pointer or pressed keep egui's
/// own treatment of the hover or the press, each property apart, unless the theme states one
/// for it (spec §2, `from_state`: the state asked alone), and are then the combined state's
/// value; a checked box's disabled fill likewise.
pub(crate) fn variant_source(r: &ResolvedTheme) -> VariantSource {
    use WidgetState as S;
    let b = &r.button;
    let cb = &r.combo_box;
    let k = &r.checkbox;
    let i = &r.input;
    let checked = k.indicator(S::CHECKED);
    let (checked_outline, checked_align) = edge(&checked);
    let primary_disabled = ButtonQuery::primary().with_state(S::DISABLED);
    // the combined state's outline where a rule states one for `state` alone
    let checked_state_line = |state: WidgetState, home: &'static str| {
        k.indicator(state)
            .from_state
            .contains(PartStated::STROKE)
            .then(|| line_of(&k.indicator(state | S::CHECKED), home))
    };
    // the combined state's fill where a rule states one for `state` alone
    let checked_state_fill = |state: WidgetState, home: &'static str| {
        state_fill(&k.indicator(state))
            .and_then(|_| fill_at(&k.indicator(state | S::CHECKED), home))
    };
    VariantSource {
        button: disabled(
            &b.label(S::DISABLED),
            &b.body(S::DISABLED),
            "button.body(DISABLED).fill",
            &b.body(S::NONE),
        ),
        primary: PrimaryDisabled {
            fill: state_fill(&b.body(primary_disabled)),
            text: state_text_color(&b.label(primary_disabled)),
        },
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
            state_fill: [
                checked_state_fill(S::HOVER, "checkbox.indicator(HOVER|CHECKED).fill"),
                checked_state_fill(S::PRESSED, "checkbox.indicator(PRESSED|CHECKED).fill"),
            ],
            state_border: [
                checked_state_line(S::HOVER, "checkbox.indicator(HOVER|CHECKED).stroke.width"),
                checked_state_line(
                    S::PRESSED,
                    "checkbox.indicator(PRESSED|CHECKED).stroke.width",
                ),
            ],
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

/// A surface's frame reads from its part, each with its home: `<home>.stroke.width`,
/// `<home>.corners` and `<home>.padding.<side>`.
fn surface(part: &PartStyle, homes: [&'static str; 7]) -> SurfaceSource {
    let [width, corners, top, right, bottom, left, shadows] = homes;
    let (edge, align) = edge(part);
    SurfaceSource {
        fill: fill(part),
        stroke: edge.color,
        width: edge.width.map(|v| (v, width)),
        align,
        radius: edge.radius.map(|v| (v, corners)),
        padding: sides(part, [top, right, bottom, left]),
        shadow: shadowed(part),
        drop: drop_shadow(part, shadows),
    }
}

/// The seven homes [`surface`] names, of the part whose path is `$p`.
macro_rules! homes {
    ($p:literal) => {
        [
            concat!($p, ".stroke.width"),
            concat!($p, ".corners"),
            concat!($p, ".padding.top"),
            concat!($p, ".padding.right"),
            concat!($p, ".padding.bottom"),
            concat!($p, ".padding.left"),
            concat!($p, ".shadows"),
        ]
    };
}

/// [`FrameSource`] of the 0.8.0 model; `inactive` is the same theme resolved for a window that
/// is not active (`ThemeContext::window_active` false), whose title bar the
/// `WindowTitleBar` frame fills from (the table of homes' `window.inactive_title_bar_background`
/// row).
pub(crate) fn frame_source(r: &ResolvedTheme, inactive: &ResolvedTheme) -> FrameSource {
    use WidgetState as S;
    let window = WindowState::default();
    FrameSource {
        window: surface(&r.window.frame(window), homes!("window.frame(normal)")),
        title_bar: fill(&inactive.window.title_bar(window)),
        dialog: surface(&r.dialog.panel, homes!("dialog.panel")),
        popover: surface(&r.popover.panel, homes!("popover.panel")),
        tooltip: surface(&r.tooltip.panel, homes!("tooltip.panel")),
        card: surface(&r.card.container(S::NONE), homes!("card.container(NONE)")),
        sidebar: surface(
            &r.sidebar.container(S::NONE),
            homes!("sidebar.container(NONE)"),
        ),
        toolbar: surface(&r.toolbar.bar(S::NONE), homes!("toolbar.bar(NONE)")),
        status_bar: surface(&r.status_bar.bar, homes!("status_bar.bar")),
        shadow_color: rgba(r.colors.shadow),
        window_margin: r.layout.window_margin,
    }
}

/// Every source of one scheme's styles from the 0.8.0 model, an unstated hover or press taking
/// egui's own treatment (spec §10, rule 0); `inactive` as [`frame_source`] takes it.
///
/// # Errors
///
/// [`focus_ring_source`]'s.
pub(crate) fn sources(r: &ResolvedTheme, inactive: &ResolvedTheme) -> Mapped<Sources> {
    Ok(Sources {
        base: base_source(r),
        cells: cell_source(r),
        variants: variant_source(r),
        frames: frame_source(r, inactive),
        focus_ring: focus_ring_source(r)?,
        rule_0: true,
    })
}

/// [`FocusRingSource`] of the 0.8.0 model: the ring's stroke where it is one stroke of one
/// colour (a stroke stated as none is a zero width, no ring; a per-side one gives no ring, as
/// [`super::focus_ring_width`] cannot map it), its offset, its trigger, the second ring inside
/// it (`inner_stroke`) and its outer corners (`corners`) where the theme states them. The
/// plugin strokes the ring outside the widget's edge grown by the offset, whatever alignment
/// the stroke states (README, what egui cannot draw).
///
/// # Errors
///
/// [`Unmapped`] `focus_ring.kind` where the indicator is not a ring (a fill, an underline),
/// which the plugin's ring cannot draw, `focus_ring.inner_stroke` where the second ring is
/// stated per side, and `focus_ring.corners` where the corners are continuous or concentric.
pub(crate) fn focus_ring_source(r: &ResolvedTheme) -> Mapped<FocusRingSource> {
    use crate::plugin::RingCorners;
    use native_theme::next::{CornerCurve, CornerRadius};

    let ring = &r.focus_ring;
    if ring.kind != FocusKind::Ring {
        return Err(Unmapped("focus_ring.kind"));
    }
    let (color, width) = match &ring.stroke {
        Stroke::None => (None, Some(0.0)),
        Stroke::Uniform { color, width, .. } => (Some(rgba(*color)), Some(*width)),
        _ => (None, None),
    };
    let inner = match &ring.inner_stroke {
        Stroke::None => None,
        Stroke::Uniform { color, width, .. } => {
            Some((rgba(*color), (*width, "focus_ring.inner_stroke.width")))
        }
        _ => return Err(Unmapped("focus_ring.inner_stroke")),
    };
    let corners = match ring.corners.map(|c| (c.radius, c.curve)) {
        None => None,
        Some((CornerRadius::None, _)) => Some(RingCorners::Radii([0.0; 4])),
        Some((CornerRadius::Uniform(v), CornerCurve::Circular)) => Some(RingCorners::Radii([v; 4])),
        Some((
            CornerRadius::PerCorner {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            },
            CornerCurve::Circular,
        )) => Some(RingCorners::Radii([
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        ])),
        Some((CornerRadius::Pill, CornerCurve::Circular)) => Some(RingCorners::Pill),
        Some(_) => return Err(Unmapped("focus_ring.corners")),
    };
    Ok(FocusRingSource {
        color,
        width: width.map(|v| (v, "focus_ring.stroke.width")),
        offset: Some((ring.offset, "focus_ring.offset")),
        keyboard_only: ring.trigger == Some(FocusTrigger::KeyboardOnly),
        inner,
        corners,
    })
}
