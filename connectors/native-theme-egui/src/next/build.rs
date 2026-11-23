//! The 0.8.0 model's reads for the mapping's shared builders (`crate::style`): each source
//! the 0.7.2 path fills from its fields, filled here at the homes the table of homes
//! (`native-theme/tests/next_homes.toml`) names, by the query its row names. A value the
//! theme states nothing for is `None`, and the builder leaves egui's own there (spec §10,
//! rule 0); a place a note names is the home's.

use native_theme::next::{
    Color, ListQuery, Orientation, Paint, PartStated, PartStyle, ResolvedTheme, Stroke, StyleQuery,
    TextStated, TextStyle, WidgetState, WindowState,
};

use super::{padding_of, radius_of, stroke_width_of};
use crate::convert::Rgba;
use crate::style::base::{BaseSource, ButtonStates, Read, Sides};
use crate::style::line::LineAlign;
use crate::style::states::Edge;

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
    let hover = b.body(WidgetState::HOVER);
    let pressed = b.body(WidgetState::PRESSED);
    let (border, align) = edge(&rest);
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
        button: ButtonStates {
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
        },
        thumb: [
            fill_at(
                &sb.thumb(WidgetState::UNFOCUSED_VIEW),
                "scrollbar.thumb(UNFOCUSED_VIEW).fill",
            ),
            state_fill_at(&sb.thumb(WidgetState::HOVER), "scrollbar.thumb(HOVER).fill"),
            state_fill_at(
                &sb.thumb(WidgetState::PRESSED),
                "scrollbar.thumb(PRESSED).fill",
            ),
        ],
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
