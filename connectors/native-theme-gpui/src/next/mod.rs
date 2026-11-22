//! The 0.8.0 mapping, under construction: what the connector hands to gpui,
//! computed from the 0.8.0 model (`native_theme::next`) for the first slice
//! of the 0.8.0 work, the globals and `button`, `checkbox`, `list`,
//! `scrollbar` and `tooltip` (docs/v0.8.0-plan.md, M0, step 4).
//!
//! Each function reads the model where the table of homes
//! (`native-theme/tests/next_homes.toml`) places the 0.7.2 field the old
//! mapping reads, by the query its row names, and hands the values to the
//! function the old mapping hands its own reads to, so the two share
//! everything after the read. A leaf of the connector whose source lies
//! outside the slice is not mapped here. `parity` proves the leaves equal to
//! the old mapping's (plan, section 1, rule 2). Not exported; the connector
//! is ported onto it in M4, and the 0.7.2 path is removed at the 0.8.0 cut.
//!
//! A 0.7.2 field that is a soft option of a state (`button.active_background`,
//! `scrollbar.thumb_active_color`, `button.checked_background`, ...) is unset
//! where the theme states nothing for the state; a query at a state the theme
//! gives no rule returns the part without that state instead. Such a field is
//! read here only where the property's `from_state` bit is set at the state
//! (spec §2), and is `None` otherwise, so the old path's own fallback for the
//! unset option is taken from both models.

pub(crate) mod builders;
#[cfg(test)]
mod parity;

use std::rc::Rc;

use gpui::Hsla;
use gpui_component::theme::{Theme as GpuiTheme, ThemeColor, ThemeConfig};
use native_theme::next::{
    ButtonQuery, Color, CornerCurve, CornerRadius, FocusTrigger, FontStyle, ListQuery, Orientation,
    Paint, PartStated, PartStyle, ResolvedTheme, Stroke, StrokeAlign, StyleQuery, TextStated,
    TextStyle, WidgetState,
};

use crate::base_layer::{ScrollbarGeometry, ScrollbarSource, geometry_of};
use crate::colors::{SliceColors, assign_slice, channels_to_hsla, focus_band};
use crate::config::{apply_config_scalars, theme_color_to_config_colors};
use crate::geometry::Font;
use crate::{ThemeScalars, apply_scalars, font_family};

/// A source a leaf needs that the theme leaves unstated, or states in a form
/// no 0.7.2 field holds (a gradient fill, a centred stroke, a pill): its home
/// in the table of homes. The 0.7.2 path has no such case, its resolved
/// fields being always set; the 0.8.0 path reports it rather than inventing
/// the value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Unmapped(pub(crate) &'static str);

/// A leaf of the 0.8.0 mapping, or the source it lacks.
pub(crate) type Mapped<T> = Result<T, Unmapped>;

/// A colour of the model as gpui's: the connector's conversion
/// ([`channels_to_hsla`]).
fn hsla(c: Color) -> Hsla {
    channels_to_hsla([c.r, c.g, c.b, c.a])
}

/// `v`, or `Unmapped(home)` where it is `None`.
fn stated<T>(v: Option<T>, home: &'static str) -> Mapped<T> {
    v.ok_or(Unmapped(home))
}

/// A part's fill, where stated as one colour.
fn fill(part: &PartStyle, home: &'static str) -> Mapped<Hsla> {
    match &part.fill {
        Paint::Solid(c) if part.stated.contains(PartStated::FILL) => Ok(hsla(*c)),
        _ => Err(Unmapped(home)),
    }
}

/// A part's stroke colour, where stated as one stroke inside the edge.
fn stroke_color(part: &PartStyle, home: &'static str) -> Mapped<Hsla> {
    match &part.stroke {
        Stroke::Uniform {
            color,
            align: StrokeAlign::Inside,
            ..
        } if part.stated.contains(PartStated::STROKE) => Ok(hsla(*color)),
        _ => Err(Unmapped(home)),
    }
}

/// A part's stroke width, where stated as one stroke inside the edge, or as
/// none, which is no width.
fn stroke_width(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    if !part.stated.contains(PartStated::STROKE) {
        return Err(Unmapped(home));
    }
    match &part.stroke {
        Stroke::None => Ok(0.0),
        Stroke::Uniform {
            width,
            align: StrokeAlign::Inside,
            ..
        } => Ok(*width),
        _ => Err(Unmapped(home)),
    }
}

/// A part's corner radius, where stated as one circular radius, or as square
/// corners, which are no radius.
fn radius(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    if !part.stated.contains(PartStated::CORNERS) {
        return Err(Unmapped(home));
    }
    match (part.corners.radius, part.corners.curve) {
        (CornerRadius::None, _) => Ok(0.0),
        (CornerRadius::Uniform(r), CornerCurve::Circular) => Ok(r),
        _ => Err(Unmapped(home)),
    }
}

/// A part's padding, top, right, bottom, left, each side where stated, as
/// 0.7.2's per-side padding.
fn padding(part: &PartStyle) -> [Option<f32>; 4] {
    let side = |bit: PartStated, v: f32| part.stated.contains(bit).then_some(v);
    let p = &part.padding;
    [
        side(PartStated::PADDING_TOP, p.top),
        side(PartStated::PADDING_RIGHT, p.right),
        side(PartStated::PADDING_BOTTOM, p.bottom),
        side(PartStated::PADDING_LEFT, p.left),
    ]
}

/// A text style's colour, where stated.
fn text_color(text: &TextStyle, home: &'static str) -> Mapped<Hsla> {
    if text.stated.contains(TextStated::COLOR) {
        Ok(hsla(text.color))
    } else {
        Err(Unmapped(home))
    }
}

/// A part's fill where a rule states it for the queried state (its
/// `from_state` bit), as [`fill`] reads it; `None` where none does, a soft
/// option's unset value.
fn state_fill(part: &PartStyle, home: &'static str) -> Mapped<Option<Hsla>> {
    if part.from_state.contains(PartStated::FILL) {
        fill(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A part's stroke colour where a rule states it for the queried state, as
/// [`stroke_color`] reads it; `None` where none does.
fn state_stroke_color(part: &PartStyle, home: &'static str) -> Mapped<Option<Hsla>> {
    if part.from_state.contains(PartStated::STROKE) {
        stroke_color(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A text style's colour where a rule states it for the queried state, as
/// [`text_color`] reads it; `None` where none does.
fn state_text_color(text: &TextStyle, home: &'static str) -> Mapped<Option<Hsla>> {
    if text.from_state.contains(TextStated::COLOR) {
        text_color(text, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A text style's size, where stated.
fn text_size(text: &TextStyle, home: &'static str) -> Mapped<f32> {
    if text.stated.contains(TextStated::SIZE_PX) {
        Ok(text.size_px)
    } else {
        Err(Unmapped(home))
    }
}

/// A slant of the model as gpui's.
fn font_style(style: FontStyle) -> Option<gpui::FontStyle> {
    match style {
        FontStyle::Normal => Some(gpui::FontStyle::Normal),
        FontStyle::Italic => Some(gpui::FontStyle::Italic),
        FontStyle::Oblique => Some(gpui::FontStyle::Oblique),
        _ => None,
    }
}

/// A text style as the builders' [`Font`], where its family, size, weight,
/// slant and colour are stated: for the builders that set the colour. The
/// size is the model's, which the theme's text scale has already scaled.
fn font<'a>(text: &'a TextStyle, home: &'static str) -> Mapped<Font<'a>> {
    font_of(text, home, TextStated::COLOR)
}

/// A text style as the builders' [`Font`], where its family, size, weight
/// and slant are stated: for the builders that hand it to
/// `geometry::text`, which sets no colour and does not read the font's. The
/// colour carried is the style's, stated or not.
fn uncoloured_font<'a>(text: &'a TextStyle, home: &'static str) -> Mapped<Font<'a>> {
    font_of(text, home, TextStated::default())
}

/// [`font`] and [`uncoloured_font`]: the four properties every builder reads
/// and `also`.
fn font_of<'a>(text: &'a TextStyle, home: &'static str, also: TextStated) -> Mapped<Font<'a>> {
    let needed =
        TextStated::FAMILY | TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::STYLE | also;
    if !text.stated.contains(needed) {
        return Err(Unmapped(home));
    }
    Ok(Font {
        family: &text.family,
        size: text.size_px,
        weight: text.weight,
        style: stated(font_style(text.style), home)?,
        color: hsla(text.color),
    })
}

/// Whether gpui-component's focus band is on for any focus: the focus
/// indicator's stroke width (no stroke is no width) and its trigger
/// (0.7.2's `focus_ring_width` and `focus_ring_keyboard_only`).
fn focus_band_of(r: &ResolvedTheme) -> Mapped<bool> {
    let ring = &r.focus_ring;
    let width = match &ring.stroke {
        Stroke::None => 0.0,
        Stroke::Uniform { width, .. } => *width,
        _ => return Err(Unmapped("focus_ring.stroke.width")),
    };
    Ok(focus_band(
        width,
        ring.trigger == Some(FocusTrigger::KeyboardOnly),
    ))
}

/// The scroll bar's thumb at rest: the handle of a view without the focus,
/// where 0.7.2's `scrollbar.thumb_color` lives on kde-breeze (spec §5, S3);
/// a theme with no rule for that state returns the plain thumb, the field's
/// home elsewhere.
///
/// That makes `thumb(UNFOCUSED_VIEW)` the one query that returns 0.7.2's
/// `thumb_color`, the plain handle, on every preset, whichever of the two
/// the preset states it at: kde-breeze's rest state is the handle of the
/// view that has the focus, and the plain handle is its `UNFOCUSED_VIEW`
/// rule; a preset without an `UNFOCUSED_VIEW` rule returns its base there.
/// When the widgets are ported, the connector sets the bit from the view's
/// real focus instead.
fn thumb_at_rest(r: &ResolvedTheme) -> Mapped<Hsla> {
    fill(
        &r.scrollbar.thumb(WidgetState::UNFOCUSED_VIEW),
        "scrollbar.thumb(UNFOCUSED_VIEW).fill",
    )
}

/// The slice's colours of a 0.8.0 resolved theme.
pub(crate) fn slice_colors(r: &ResolvedTheme) -> Mapped<SliceColors> {
    let c = &r.colors;
    let b = &r.button;
    let l = &r.list;
    let s = &r.scrollbar;
    Ok(SliceColors {
        fg: hsla(stated(c.window.foreground, "colors.window.foreground")?),
        accent: hsla(c.accent.fill),
        border: hsla(stated(c.window.border, "colors.window.border")?),
        muted_fg: hsla(stated(
            c.window.foreground_muted,
            "colors.window.foreground_muted",
        )?),
        primary: fill(
            &b.body(ButtonQuery::primary()),
            "button.body(NONE; variant=Primary).fill",
        )?,
        primary_fg: text_color(
            &b.label(ButtonQuery::primary()),
            "button.label(NONE; variant=Primary).color",
        )?,
        secondary: fill(&b.body(WidgetState::NONE), "button.body(NONE).fill")?,
        secondary_fg: text_color(&b.label(WidgetState::NONE), "button.label(NONE).color")?,
        danger: hsla(c.error.fill),
        danger_fg: hsla(c.error.foreground),
        success: hsla(c.success.fill),
        success_fg: hsla(c.success.foreground),
        warning: hsla(c.warning.fill),
        warning_fg: hsla(c.warning.foreground),
        info: hsla(c.info.fill),
        info_fg: hsla(c.info.foreground),
        focus_ring: hsla(c.focus_ring),
        focus_band: focus_band_of(r)?,
        list_bg: fill(&l.container(WidgetState::NONE), "list.container(NONE).fill")?,
        list_grid: hsla(l.grid_color),
        list_header_bg: fill(&l.header(WidgetState::NONE), "list.header(NONE).fill")?,
        list_header_fg: text_color(
            &l.header_label(WidgetState::NONE),
            "list.header_label(NONE).color",
        )?,
        alternate_row: fill(
            &l.row(ListQuery::new(WidgetState::NONE).alternate(true)),
            "list.row(NONE; alternate=true).fill",
        )?,
        scrollbar_track: fill(&s.track(WidgetState::NONE), "scrollbar.track(NONE).fill")?,
        scrollbar_thumb: thumb_at_rest(r)?,
        scrollbar_thumb_hover: fill(&s.thumb(WidgetState::HOVER), "scrollbar.thumb(HOVER).fill")?,
        button_hover_bg: fill(&b.body(WidgetState::HOVER), "button.body(HOVER).fill")?,
        button_active_bg: state_fill(&b.body(WidgetState::PRESSED), "button.body(PRESSED).fill")?,
        list_hover_bg: fill(&l.row(WidgetState::HOVER), "list.row(HOVER).fill")?,
        list_selection_bg: fill(&l.row(WidgetState::SELECTED), "list.row(SELECTED).fill")?,
        shadow: hsla(c.shadow),
    })
}

/// The `ThemeColor` fields the slice feeds, written by the old mapping's
/// [`assign_slice`]; every other field is `ThemeColor::default()`'s.
pub(crate) fn theme_color(r: &ResolvedTheme) -> Mapped<ThemeColor> {
    let mut tc = ThemeColor::default();
    assign_slice(
        &mut tc,
        &slice_colors(r)?,
        r.is_dark(),
        r.context.reduced_transparency,
    );
    Ok(tc)
}

/// The global theme's values the slice feeds. The sizes are the model's,
/// which its text scale has already scaled.
pub(crate) fn theme_scalars(r: &ResolvedTheme) -> Mapped<ThemeScalars> {
    let t = &r.typography;
    let body = &t.body;
    let mono = &t.monospace;
    if !body.stated.contains(TextStated::FAMILY) {
        return Err(Unmapped("typography.body.family"));
    }
    if !mono.stated.contains(TextStated::FAMILY) {
        return Err(Unmapped("typography.monospace.family"));
    }
    Ok(ThemeScalars {
        is_dark: r.is_dark(),
        font_family: font_family(&body.family),
        font_size: text_size(body, "typography.body.size_px")?,
        mono_font_family: font_family(&mono.family),
        mono_font_size: text_size(mono, "typography.monospace.size_px")?,
        radius: r.layout.corner_radius,
        radius_lg: r.layout.corner_radius_lg,
        shadow: r.layout.shadow_enabled,
        focus_band: focus_band_of(r)?,
        overlay_scrollbars: r.scrollbar.overlay_mode,
    })
}

/// The gpui-component theme of the slice: [`theme_color`]'s colours and the
/// tokens gpui-component derives from them, [`theme_scalars`]' values, and,
/// in the slot of its mode, a `ThemeConfig` of the same values and colours.
/// Every leaf the slice does not feed is gpui-component's default.
pub(crate) fn to_theme(r: &ResolvedTheme) -> Mapped<GpuiTheme> {
    let tc = theme_color(r)?;
    let scalars = theme_scalars(r)?;
    let mut theme = GpuiTheme::from(&tc);
    apply_scalars(&mut theme, &scalars);
    let mut config = ThemeConfig::default();
    config.mode = theme.mode;
    apply_config_scalars(&mut config, &scalars);
    config.colors = theme_color_to_config_colors(&tc);
    if r.is_dark() {
        theme.dark_theme = Rc::new(config);
    } else {
        theme.light_theme = Rc::new(config);
    }
    Ok(theme)
}

/// The scroll bar's values for gpui-base.
pub(crate) fn scrollbar_geometry(r: &ResolvedTheme) -> Mapped<ScrollbarGeometry> {
    let s = &r.scrollbar;
    let vertical = StyleQuery::<()>::new(WidgetState::NONE).orientation(Orientation::Vertical);
    Ok(geometry_of(&ScrollbarSource {
        groove_width: stated(
            s.track(vertical).min_size[0],
            "scrollbar.track(NONE; orientation=Vertical).min_size[0]",
        )?,
        thumb_width: stated(
            s.thumb(vertical).min_size[0],
            "scrollbar.thumb(NONE; orientation=Vertical).min_size[0]",
        )?,
        corner_radius: r.layout.corner_radius,
        min_thumb_length: s.min_thumb_length,
        track: fill(&s.track(WidgetState::NONE), "scrollbar.track(NONE).fill")?,
        border: hsla(stated(r.colors.window.border, "colors.window.border")?),
        thumb: thumb_at_rest(r)?,
        thumb_hover: fill(&s.thumb(WidgetState::HOVER), "scrollbar.thumb(HOVER).fill")?,
        thumb_active: state_fill(
            &s.thumb(WidgetState::PRESSED),
            "scrollbar.thumb(PRESSED).fill",
        )?,
    }))
}

#[cfg(test)]
mod tests {
    use gpui::{StyleRefinement, Styled as _};

    use super::*;

    const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

    /// A part whose fill is `c`, stated, and from a state rule where
    /// `for_state`.
    fn filled(c: Color, for_state: bool) -> PartStyle {
        let mut part = PartStyle::default();
        part.fill = Paint::Solid(c);
        part.stated = PartStated::FILL;
        if for_state {
            part.from_state = PartStated::FILL;
        }
        part
    }

    /// `button.active_background`: the pressed body's fill without a rule
    /// for the state is unset, so `assign_secondary` derives
    /// `secondary_active` as for 0.7.2's `None`.
    #[test]
    fn a_pressed_button_fill_without_a_state_rule_is_unset() -> Mapped<()> {
        assert_eq!(state_fill(&filled(RED, false), "pressed")?, None);
        assert_eq!(state_fill(&filled(RED, true), "pressed")?, Some(hsla(RED)));
        Ok(())
    }

    /// `scrollbar.thumb_active_color`: without a rule for the state the
    /// pressed thumb is the hovered one, 0.7.2's fallback.
    #[test]
    fn a_pressed_thumb_without_a_state_rule_falls_back_to_the_hover_colour() -> Mapped<()> {
        let source = |pressed: &PartStyle| -> Mapped<ScrollbarSource> {
            Ok(ScrollbarSource {
                groove_width: 10.0,
                thumb_width: 6.0,
                corner_radius: 3.0,
                min_thumb_length: 20.0,
                track: hsla(BLUE),
                border: hsla(BLUE),
                thumb: hsla(BLUE),
                thumb_hover: hsla(GREEN),
                thumb_active: state_fill(pressed, "scrollbar.thumb(PRESSED).fill")?,
            })
        };
        let unset = geometry_of(&source(&filled(RED, false))?);
        assert_eq!(unset.thumb_active, hsla(GREEN));
        let stated = geometry_of(&source(&filled(RED, true))?);
        assert_eq!(stated.thumb_active, hsla(RED));
        Ok(())
    }

    /// `button.checked_text_color` and `button.checked_background`: without
    /// a rule for `CHECKED` the pressed pair stands in.
    #[test]
    fn a_checked_pair_without_a_state_rule_is_the_pressed_pair() -> Mapped<()> {
        let mut label = TextStyle::default();
        label.color = RED;
        label.stated = TextStated::COLOR;
        assert_eq!(state_text_color(&label, "checked label")?, None);
        let checked = crate::geometry::button_checked_from(
            state_text_color(&label, "checked label")?,
            hsla(GREEN),
            state_fill(&filled(RED, false), "checked body")?,
            state_fill(&filled(BLUE, true), "pressed body")?,
        );
        assert_eq!(
            checked,
            StyleRefinement::default()
                .text_color(hsla(GREEN))
                .bg(hsla(BLUE))
        );
        label.from_state = TextStated::COLOR;
        assert_eq!(state_text_color(&label, "checked label")?, Some(hsla(RED)));
        Ok(())
    }

    /// `button.hover_border_color`: without a rule for `HOVER` it is unset,
    /// and no bordered button is built.
    #[test]
    fn a_hover_stroke_without_a_state_rule_is_unset() -> Mapped<()> {
        let mut hovered = PartStyle::default();
        hovered.stroke = Stroke::uniform(1.0, RED);
        hovered.stated = PartStated::STROKE;
        assert_eq!(state_stroke_color(&hovered, "hover")?, None);
        hovered.from_state = PartStated::STROKE;
        assert_eq!(state_stroke_color(&hovered, "hover")?, Some(hsla(RED)));
        Ok(())
    }
}
