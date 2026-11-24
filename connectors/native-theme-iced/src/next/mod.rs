//! The connector on the 0.8.0 model ([`native_theme::next`]), beside the
//! 0.7.2 one until the 0.8.0 cut removes the latter: what the connector
//! hands to iced, computed from a [`native_theme::next::ResolvedTheme`].
//!
//! [`to_theme`] builds the iced theme, as [`crate::to_theme_in`] does; the
//! [`styles`] module holds one function for each style function of
//! [`crate::styles`] it has ported, of the same name. Each takes the 0.8.0
//! resolved theme, which carries its name and its scheme, so neither is
//! passed; and each returns [`Mapped`]: the value, or the [`Unmapped`]
//! source it needs that the theme leaves unstated, where the 0.7.2 function,
//! whose fields are always set, returns the value alone. Where the theme
//! states nothing for a value iced has one of its own for, iced's stays
//! (spec §10, rule 0): a value whose home the table of homes allows to be
//! unstated (`unstated_if_unset`, a soft option of a state) keeps iced's,
//! and only a value a resolved theme always states is [`Unmapped`] where it
//! is missing.
//!
//! Each function reads the model where the table of homes
//! (`native-theme/tests/next_homes.toml`) places the 0.7.2 field the old
//! mapping reads, by the query its row names, and hands the values to the
//! function the old mapping hands its own reads to, so the two share
//! everything after the read. `parity` proves the outputs equal to the old
//! mapping's (docs/v0.8.0-plan.md, section 1, rule 2), and lists where they
//! differ with the reason.
//!
//! A 0.7.2 field that is a soft option of a state (`button.active_background`,
//! `scrollbar.thumb_active_color`, `button.checked_background`, ...) is unset
//! where the theme states nothing for the state; a query at a state the theme
//! gives no rule returns the part without that state instead. Such a field is
//! read here only where the property's `from_state` bit is set at the state
//! (spec §2), and is `None` otherwise, so the old path's own fallback for the
//! unset option is taken from both models.
//!
//! A border's stroke is read as stated, with its alignment
//! (`crate::styles::Line`), and the shared style function hands iced its
//! colour as iced on wgpu paints it over the fill that state paints: iced
//! ends a widget's fill at the border's inner edge (`iced_wgpu` 0.14.0,
//! `src/shader/quad/solid.wgsl`), so an `Outside` stroke is drawn as stated;
//! an `Inside` one, which the theme lays over the fill, is the stroke
//! composited over the fill; a `Center` one, over the fill at half its
//! alpha, the band's mean (spec §2, `StrokeAlign`). Under an opaque stroke
//! the three are the stated colour. iced on tiny-skia fills the whole bounds
//! under the border, as gpui does; a style function does not know the
//! renderer, so the connector computes for wgpu and its README records
//! tiny-skia.

// The part readers serve the style functions, which the `widgets` feature
// builds.
#![cfg_attr(not(feature = "widgets"), allow(dead_code))]

#[cfg(feature = "widgets")]
pub(crate) mod borders;
#[cfg(feature = "widgets")]
pub mod styles;

#[cfg(test)]
pub(crate) mod parity;
#[cfg(test)]
mod root_parity;
#[cfg(test)]
mod tests;

use std::fmt;
use std::time::Duration;

use iced_core::Color as IcedColor;
use native_theme::next::{
    Color, DialogButtonOrder, IconSizes, IconStated, InputQuery, Orientation, Paint, PartStated,
    PartStyle, ResolvedTheme, Scheme, Stroke, StyleQuery, TextStated, TextStyle, WidgetState,
};

use crate::ColorMode;
use crate::extended::OverrideColors;
#[cfg(feature = "widgets")]
pub use crate::focus_ring::RingLook;

/// A source a value needs that the theme leaves unstated, or states in a
/// form the connector cannot hand to iced (a gradient fill, a per-side
/// stroke, a concentric radius): its place in the 0.8.0 model, as a query and a
/// property (`button.body(NONE).stroke.width`). The 0.7.2 path has no such
/// case, its resolved fields being always set; the 0.8.0 path reports it
/// rather than inventing the value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unmapped(pub(crate) &'static str);

impl Unmapped {
    /// The source's place in the 0.8.0 model.
    #[must_use]
    pub fn home(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Unmapped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the theme states no value iced can take at {}", self.0)
    }
}

impl std::error::Error for Unmapped {}

/// A value of the 0.8.0 mapping, or the source it lacks.
pub type Mapped<T> = Result<T, Unmapped>;

/// Where a border's stroke lies against the widget's edge, as the 0.8.0
/// model states it (`docs/v0.8.0-spec.md` §2, `StrokeAlign`), in the form
/// the shared style functions take it. The 0.7.2 model states none and its
/// stroke is handed to iced as given, which is what `Outside` gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LineAlign {
    Outside,
    Inside,
    Center,
}

/// A colour of the model as iced's: the same four gamma-encoded channels
/// with straight alpha.
pub(crate) fn to_color(c: Color) -> IcedColor {
    IcedColor {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
    }
}

/// `v`, or `Unmapped(home)` where it is `None`.
pub(crate) fn stated<T>(v: Option<T>, home: &'static str) -> Mapped<T> {
    v.ok_or(Unmapped(home))
}

/// A part's fill, where stated: its one colour, or, for a layered paint,
/// the colour a toolkit that paints one colour renders in its place
/// (spec §10, rule 2; [`Paint::fallback_color`]). A stated `Paint::None` is
/// no fill, here the transparent colour, a fill iced paints nothing of; a
/// state's fill the style lays over the fill at rest is read as a layer
/// instead (`styles::state_layer`), whose no fill paints nothing in that
/// state rather than the fill at rest under a transparent layer.
pub(crate) fn fill(part: &PartStyle, home: &'static str) -> Mapped<IcedColor> {
    stated(fill_of(part), home)
}

/// A part's fill as `fill` reads it, `None` where it is not stated, or
/// stated as a layered paint with no fallback colour: for a reader that
/// shows iced's own there (spec §10, rule 0). A fill stated as none is
/// transparent, and a layered paint gives its fallback colour (spec §10,
/// rule 2).
pub fn fill_of(part: &PartStyle) -> Option<IcedColor> {
    if !part.stated.contains(PartStated::FILL) {
        return None;
    }
    match &part.fill {
        Paint::None => Some(IcedColor::TRANSPARENT),
        Paint::Solid(c) => Some(to_color(*c)),
        paint @ Paint::Complex(_) => paint.fallback_color().map(to_color),
    }
}

/// A part's fill where a rule states it for the queried state (its
/// `from_state` bit), as [`fill`] reads it; `None` where none does, a soft
/// option's unset value.
pub(crate) fn state_fill(part: &PartStyle, home: &'static str) -> Mapped<Option<IcedColor>> {
    if part.from_state.contains(PartStated::FILL) {
        fill(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A part's fill as `state_fill` reads it, `None` where no rule states it
/// for the queried state, or it is stated in a form iced cannot paint (a
/// layered paint with no fallback colour). The rule is for some flag of the
/// query, not a given one (spec §2, `from_state`).
pub fn state_fill_of(part: &PartStyle) -> Option<IcedColor> {
    part.from_state
        .contains(PartStated::FILL)
        .then(|| fill_of(part))
        .flatten()
}

/// A part's stroke colour as stated, where stated as one stroke of one
/// colour, in any alignment; `None` where it is not stated, or stated in a
/// form iced cannot draw. The style functions take it with its alignment
/// and composite it over the fill each state paints (the module's doc), so
/// this is the theme's colour, not always the one iced draws.
pub fn stroke_color_of(part: &PartStyle) -> Option<IcedColor> {
    match &part.stroke {
        Stroke::Uniform { color, .. } if part.stated.contains(PartStated::STROKE) => {
            Some(to_color(*color))
        }
        _ => None,
    }
}

/// A part's stroke colour as [`stroke_color_of`] reads it, `None` where no
/// rule states it for the queried state. The rule is for some flag of the
/// query, not a given one (spec §2, `from_state`).
pub fn state_stroke_color_of(part: &PartStyle) -> Option<IcedColor> {
    part.from_state
        .contains(PartStated::STROKE)
        .then(|| stroke_color_of(part))
        .flatten()
}

/// A part's stroke width, where stated as one stroke, in any alignment, or
/// as none, which is no width; `None` where it is not stated, or stated in
/// a form iced cannot draw (a per-side stroke).
pub fn stroke_width_of(part: &PartStyle) -> Option<f32> {
    if !part.stated.contains(PartStated::STROKE) {
        return None;
    }
    match &part.stroke {
        Stroke::None => Some(0.0),
        Stroke::Uniform { width, .. } => Some(*width),
        _ => None,
    }
}

/// A part's corner radius as one radius: one radius stated for every
/// corner, square corners, which are no radius, or a pill, which iced rounds
/// by a radius its renderers clamp to half the shorter side
/// (`crate::styles::pill`); `None` where it is not stated, or stated as
/// per-corner radii that differ, or as a concentric radius, which needs the
/// container's. iced's corner is circular, so a continuous curve is drawn as
/// the circular corner of its radius (spec §10, rule 2).
#[cfg(feature = "widgets")]
pub fn radius_of(part: &PartStyle) -> Option<f32> {
    use native_theme::next::CornerRadius;

    if !part.stated.contains(PartStated::CORNERS) {
        return None;
    }
    match part.corners.radius {
        CornerRadius::None => Some(0.0),
        CornerRadius::Uniform(r) => Some(r),
        CornerRadius::PerCorner {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        } => [top_right, bottom_right, bottom_left]
            .iter()
            .all(|&r| r == top_left)
            .then_some(top_left),
        CornerRadius::Pill => Some(crate::styles::pill().top_left),
        _ => None,
    }
}

/// A part's padding, top, right, bottom, left, each side where stated.
pub fn padding_of(part: &PartStyle) -> [Option<f32>; 4] {
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
pub(crate) fn text_color(text: &TextStyle, home: &'static str) -> Mapped<IcedColor> {
    stated(text_color_of(text), home)
}

/// A text style's colour as `text_color` reads it, `None` where it is not
/// stated.
pub fn text_color_of(text: &TextStyle) -> Option<IcedColor> {
    text.stated
        .contains(TextStated::COLOR)
        .then(|| to_color(text.color))
}

/// A text style's colour where a rule states it for the queried state, as
/// [`text_color`] reads it; `None` where none does.
pub(crate) fn state_text_color(text: &TextStyle, home: &'static str) -> Mapped<Option<IcedColor>> {
    if text.from_state.contains(TextStated::COLOR) {
        text_color(text, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A text style's colour as `state_text_color` reads it, `None` where no
/// rule states it for the queried state or it is not stated. The rule is
/// for some flag of the query, not a given one (spec §2, `from_state`).
pub fn state_text_color_of(text: &TextStyle) -> Option<IcedColor> {
    text.from_state
        .contains(TextStated::COLOR)
        .then(|| text_color_of(text))
        .flatten()
}

/// [`crate::palette::to_palette`]: iced's six-colour palette.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no window background or foreground.
pub(crate) fn to_palette(r: &ResolvedTheme) -> Mapped<iced_core::theme::Palette> {
    let c = &r.colors;
    Ok(iced_core::theme::Palette {
        background: to_color(stated(c.window.background, "colors.window.background")?),
        text: to_color(stated(c.window.foreground, "colors.window.foreground")?),
        primary: to_color(c.accent.fill),
        success: to_color(c.success.fill),
        warning: to_color(c.warning.fill),
        danger: to_color(c.error.fill),
    })
}

/// The colours the connector writes over iced's generated extended palette.
fn override_colors(r: &ResolvedTheme) -> Mapped<OverrideColors> {
    let c = &r.colors;
    Ok(OverrideColors {
        placeholder: text_color(
            &r.input.placeholder(InputQuery::from(WidgetState::NONE)),
            "input.placeholder(NONE).color",
        )?,
        surface: to_color(stated(c.view.background, "colors.view.background")?),
        foreground: to_color(stated(c.window.foreground, "colors.window.foreground")?),
        accent_fg: to_color(c.accent.foreground),
        success_fg: to_color(c.success.foreground),
        danger_fg: to_color(c.error.foreground),
        warning_fg: to_color(c.warning.foreground),
    })
}

/// [`crate::to_theme_in`]: the iced theme, named by the theme's name, its
/// light/dark flag (`Extended::is_dark`) the theme's scheme.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no window background or foreground,
/// no content background or no input placeholder colour.
pub fn to_theme(r: &ResolvedTheme) -> Mapped<iced_core::theme::Theme> {
    let mode = match r.context.scheme {
        Scheme::Light => ColorMode::Light,
        Scheme::Dark => ColorMode::Dark,
    };
    Ok(crate::theme_from(
        &r.name,
        to_palette(r)?,
        override_colors(r)?,
        Some(mode),
    ))
}

// ---------------------------------------------------------------------------
// The crate root's accessors, each read at its home in the table of homes
// ---------------------------------------------------------------------------

/// The border width, in logical pixels (`layout.border_width`), for a
/// border an application draws itself; each style takes its own part's
/// stroke.
#[must_use]
pub fn border_line_width(r: &ResolvedTheme) -> f32 {
    r.layout.border_width
}

/// The share of the text colour already folded into every stated border
/// colour (`layout.border_opacity`).
#[must_use]
pub fn border_opacity(r: &ResolvedTheme) -> f32 {
    r.layout.border_opacity
}

/// Whether shadows are drawn by default (`layout.shadow_enabled`), for a
/// surface an application shadows itself; each style casts the shadows its
/// own part states (`next::styles`).
#[must_use]
pub fn border_shadow_enabled(r: &ResolvedTheme) -> bool {
    r.layout.shadow_enabled
}

/// [`crate::disabled_opacity`]: the whole widget's fade on top of its
/// disabled colours (`colors.disabled_opacity`); 1.0 where the platform dims
/// by colour alone.
#[must_use]
pub fn disabled_opacity(r: &ResolvedTheme) -> f32 {
    r.colors.disabled_opacity
}

/// The text colour of disabled elements (`colors.disabled_text`), for text
/// an application draws itself; each style takes its own part's disabled
/// label.
#[must_use]
pub fn disabled_text_color(r: &ResolvedTheme) -> IcedColor {
    to_color(r.colors.disabled_text)
}

/// The focus ring's stroke width, in logical pixels
/// (`focus_ring.stroke.width`); 0 where the ring has no stroke.
///
/// # Errors
///
/// [`Unmapped`] where the ring's stroke is per side.
pub fn focus_ring_width(r: &ResolvedTheme) -> Mapped<f32> {
    match &r.focus_ring.stroke {
        Stroke::None => Ok(0.0),
        Stroke::Uniform { width, .. } => Ok(*width),
        _ => Err(Unmapped("focus_ring.stroke.width")),
    }
}

/// The gap between an element's edge and its focus ring, in logical pixels
/// (`focus_ring.offset`).
#[must_use]
pub fn focus_ring_offset(r: &ResolvedTheme) -> f32 {
    r.focus_ring.offset
}

/// [`crate::icon_sizes`]: the per-context icon sizes (`icons.sizes`), each
/// `None` where the theme states none.
#[must_use]
pub fn icon_sizes(r: &ResolvedTheme) -> &IconSizes {
    &r.icons.sizes
}

/// The four text roles of 0.7.2's `text_scale`, each the typography's style
/// at its home laid over the body (`Typography::over_body`), or the body's
/// where the theme defines no such role (the table of homes, rows
/// `text_scale.*`, `home_fallback`).
#[derive(Clone, Debug)]
pub struct TextScale {
    /// `typography.caption`.
    pub caption: TextStyle,
    /// `typography.headline`.
    pub section_heading: TextStyle,
    /// `typography.title1`.
    pub dialog_title: TextStyle,
    /// `typography.display`.
    pub display: TextStyle,
}

/// [`crate::text_scale`]: the four text roles 0.7.2 names, each a size,
/// weight and line height as the theme states them, already scaled by the
/// text scale the theme was resolved for, in the body's family, colour and
/// slant where the role states none.
#[must_use]
pub fn text_scale(r: &ResolvedTheme) -> TextScale {
    let t = &r.typography;
    TextScale {
        caption: t.over_body(t.caption.as_ref()),
        section_heading: t.over_body(t.headline.as_ref()),
        dialog_title: t.over_body(t.title1.as_ref()),
        display: t.over_body(t.display.as_ref()),
    }
}

/// [`crate::line_height`]: the line height multiplier
/// (`typography.line_height_multiplier`), for `LineHeight::Relative`.
#[must_use]
pub fn line_height(r: &ResolvedTheme) -> f32 {
    r.typography.line_height_multiplier
}

/// A text style's weight, where stated.
fn text_weight(text: &TextStyle, home: &'static str) -> Mapped<u16> {
    if text.stated.contains(TextStated::WEIGHT) {
        Ok(text.weight)
    } else {
        Err(Unmapped(home))
    }
}

/// [`crate::font_weight`]: the body text's CSS weight
/// (`typography.body.weight`), for [`crate::to_iced_weight`].
///
/// # Errors
///
/// [`Unmapped`] where the theme states no body weight.
pub fn font_weight(r: &ResolvedTheme) -> Mapped<u16> {
    text_weight(&r.typography.body, "typography.body.weight")
}

/// [`crate::mono_font_weight`]: the monospace text's CSS weight
/// (`typography.monospace.weight`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no monospace weight.
pub fn mono_font_weight(r: &ResolvedTheme) -> Mapped<u16> {
    text_weight(&r.typography.monospace, "typography.monospace.weight")
}

/// [`crate::dialog_button_order`]: which end of a dialog's button row its
/// affirmative button sits at (`dialog.button_order`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no order.
pub fn dialog_button_order(r: &ResolvedTheme) -> Mapped<DialogButtonOrder> {
    stated(r.dialog.button_order, "dialog.button_order")
}

/// [`crate::dialog_button_gap`]: the space between a dialog's buttons, in
/// logical pixels (`dialog.button_gap`).
#[must_use]
pub fn dialog_button_gap(r: &ResolvedTheme) -> f32 {
    r.dialog.button_gap
}

/// [`crate::scrollbar_groove_width`]: the vertical scroll bar's groove
/// width, in logical pixels (`scrollbar.track(NONE; orientation=Vertical)
/// .min_size[0]`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no groove width.
pub fn scrollbar_groove_width(r: &ResolvedTheme) -> Mapped<f32> {
    let vertical = StyleQuery::<()>::new(WidgetState::NONE).orientation(Orientation::Vertical);
    stated(
        r.scrollbar.track(vertical).min_size[0],
        "scrollbar.track(NONE; orientation=Vertical).min_size[0]",
    )
}

/// [`crate::list_header_hover_background`]: the fill of a hovered
/// column-header section, where a rule states one (`list.header(HOVER)
/// .fill`); `None` where none does.
///
/// # Errors
///
/// [`Unmapped`] where the fill is stated as a layered paint with no
/// fallback colour.
pub fn list_header_hover_background(r: &ResolvedTheme) -> Mapped<Option<IcedColor>> {
    state_fill(
        &r.list.header(WidgetState::HOVER),
        "list.header(HOVER).fill",
    )
}

/// [`crate::list_header_active_background`]: the fill of a pressed
/// column-header section, where a rule states one (`list.header(PRESSED)
/// .fill`); `None` where none does.
///
/// # Errors
///
/// [`Unmapped`] where the fill is stated as a layered paint with no
/// fallback colour.
pub fn list_header_active_background(r: &ResolvedTheme) -> Mapped<Option<IcedColor>> {
    state_fill(
        &r.list.header(WidgetState::PRESSED),
        "list.header(PRESSED).fill",
    )
}

/// [`crate::tooltip_show_delay`]: the delay between the pointer coming to
/// rest on an element and its tooltip showing (`timings.tooltip_show_ms`),
/// for `Tooltip::delay`; `None` where the theme states none, a `Tooltip`
/// then keeping iced's own.
#[must_use]
pub fn tooltip_show_delay(r: &ResolvedTheme) -> Option<Duration> {
    r.timings
        .tooltip_show_ms
        .map(|ms| Duration::from_millis(u64::from(ms)))
}

// ---------------------------------------------------------------------------
// The crate root's helpers, each read at its home in the table of homes
// ---------------------------------------------------------------------------

/// [`crate::border_color`]: the border and divider colour
/// (`colors.window.border`), into which the model has already folded the
/// border opacity.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no window border colour.
pub fn border_color(r: &ResolvedTheme) -> Mapped<IcedColor> {
    stated(r.colors.window.border, "colors.window.border").map(to_color)
}

/// [`crate::border_corner_radius`]: the default corner radius
/// (`layout.corner_radius`).
#[must_use]
pub fn border_corner_radius(r: &ResolvedTheme) -> f32 {
    r.layout.corner_radius
}

/// [`crate::border_corner_radius_lg`]: the default large corner radius
/// (`layout.corner_radius_lg`).
#[must_use]
pub fn border_corner_radius_lg(r: &ResolvedTheme) -> f32 {
    r.layout.corner_radius_lg
}

/// [`crate::focus_ring_color`]: the focus indicator's colour
/// (`colors.focus_ring`).
#[must_use]
pub fn focus_ring_color(r: &ResolvedTheme) -> IcedColor {
    to_color(r.colors.focus_ring)
}

/// [`crate::link_color`]: the hyperlink colour (`colors.link`).
#[must_use]
pub fn link_color(r: &ResolvedTheme) -> IcedColor {
    to_color(r.colors.link)
}

/// [`crate::selection_background`]: the selection's fill
/// (`colors.selection.background`).
#[must_use]
pub fn selection_background(r: &ResolvedTheme) -> IcedColor {
    to_color(r.colors.selection.background)
}

/// [`crate::info_color`]: the info colour (`colors.info.fill`), which iced's
/// extended palette has no family for.
#[must_use]
pub fn info_color(r: &ResolvedTheme) -> IcedColor {
    to_color(r.colors.info.fill)
}

/// [`crate::info_text_color`]: the text colour on an info fill
/// (`colors.info.foreground`).
#[must_use]
pub fn info_text_color(r: &ResolvedTheme) -> IcedColor {
    to_color(r.colors.info.foreground)
}

/// [`crate::warning_text_color`]: the text colour on a warning fill
/// (`colors.warning.foreground`).
#[must_use]
pub fn warning_text_color(r: &ResolvedTheme) -> IcedColor {
    to_color(r.colors.warning.foreground)
}

/// [`crate::dialog_title_font_color`]: the colour of a dialog's title
/// (`dialog.title.color`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no title colour.
pub fn dialog_title_font_color(r: &ResolvedTheme) -> Mapped<IcedColor> {
    text_color(&r.dialog.title, "dialog.title.color")
}

/// [`crate::expander_arrow_color`]: the colour of an expander's disclosure
/// arrow (`expander.arrow(NONE).color`), and the header label's
/// (`expander.header_label(NONE).color`) where the theme states none.
///
/// # Errors
///
/// [`Unmapped`] where the theme states neither.
pub fn expander_arrow_color(r: &ResolvedTheme) -> Mapped<IcedColor> {
    let x = &r.expander;
    let arrow = x.arrow(WidgetState::NONE);
    if arrow.stated.contains(IconStated::COLOR) {
        return Ok(to_color(arrow.color));
    }
    text_color(
        &x.header_label(WidgetState::NONE),
        "expander.header_label(NONE).color",
    )
}

/// [`crate::font_family`]: the body text's family as stated
/// (`typography.body.family`).
#[must_use]
pub fn font_family(r: &ResolvedTheme) -> &str {
    &r.typography.body.family
}

/// [`crate::mono_font_family`]: the monospace text's family as stated
/// (`typography.monospace.family`).
#[must_use]
pub fn mono_font_family(r: &ResolvedTheme) -> &str {
    &r.typography.monospace.family
}

/// [`crate::font_size`]: the body text's size in logical pixels
/// (`typography.body.size_px`), already scaled by the text scale the theme
/// was resolved for, so no preferences are passed.
#[must_use]
pub fn font_size(r: &ResolvedTheme) -> f32 {
    r.typography.body.size_px
}

/// [`crate::mono_font_size`]: the monospace text's size in logical pixels
/// (`typography.monospace.size_px`), already scaled as [`font_size`]'s.
#[must_use]
pub fn mono_font_size(r: &ResolvedTheme) -> f32 {
    r.typography.monospace.size_px
}

/// [`crate::dialog_max_width`]: a dialog's greatest width in logical pixels
/// (`dialog.panel.max_size[0]`), `None` where the theme states none.
#[must_use]
pub fn dialog_max_width(r: &ResolvedTheme) -> Option<f32> {
    r.dialog.panel.max_size[0]
}

/// [`crate::dialog_max_height`]: a dialog's greatest height in logical
/// pixels (`dialog.panel.max_size[1]`), `None` where the theme states none.
#[must_use]
pub fn dialog_max_height(r: &ResolvedTheme) -> Option<f32> {
    r.dialog.panel.max_size[1]
}

/// [`crate::dialog_min_width`]: a dialog's least width in logical pixels
/// (`dialog.panel.min_size[0]`), `None` where the theme states none.
#[must_use]
pub fn dialog_min_width(r: &ResolvedTheme) -> Option<f32> {
    r.dialog.panel.min_size[0]
}

/// [`crate::dialog_min_height`]: a dialog's least height in logical pixels
/// (`dialog.panel.min_size[1]`), `None` where the theme states none.
#[must_use]
pub fn dialog_min_height(r: &ResolvedTheme) -> Option<f32> {
    r.dialog.panel.min_size[1]
}

/// [`crate::list_header_height`]: a table header row's height in logical
/// pixels (`list.header(NONE).min_size[1]`), `None` where the theme states
/// none.
#[must_use]
pub fn list_header_height(r: &ResolvedTheme) -> Option<f32> {
    r.list.header(WidgetState::NONE).min_size[1]
}

/// [`crate::toolbar_bar_height`]: a toolbar's least outer height in logical
/// pixels (`toolbar.bar(NONE).min_size[1]`), `None` where the theme states
/// none.
#[must_use]
pub fn toolbar_bar_height(r: &ResolvedTheme) -> Option<f32> {
    r.toolbar.bar(WidgetState::NONE).min_size[1]
}

/// [`crate::toolbar_item_gap`]: the space between a toolbar's items in
/// logical pixels (`toolbar.item_gap`).
#[must_use]
pub fn toolbar_item_gap(r: &ResolvedTheme) -> Option<f32> {
    r.toolbar.item_gap
}

/// [`crate::toolbar_border_line_width`]: the height of the rule under a
/// toolbar in logical pixels (`toolbar.bar(NONE).stroke.width`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no stroke for the bar, or a stroke
/// per side.
pub fn toolbar_border_line_width(r: &ResolvedTheme) -> Mapped<f32> {
    let bar = r.toolbar.bar(WidgetState::NONE);
    if !bar.stated.contains(PartStated::STROKE) {
        return Err(Unmapped("toolbar.bar(NONE).stroke.width"));
    }
    line_width(&bar, "toolbar.bar(NONE).stroke.width")
}

/// A part's stroke width as iced outlines it: the stated one, a stated
/// `Stroke::None` no width, and the width of iced's own border,
/// `Border::default()`, where the theme states none (spec §10, rule 0).
fn line_width(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    if !part.stated.contains(PartStated::STROKE) {
        return Ok(iced_core::Border::default().width);
    }
    match &part.stroke {
        Stroke::None => Ok(0.0),
        Stroke::Uniform { width, .. } => Ok(*width),
        _ => Err(Unmapped(home)),
    }
}

/// A part's padding, each side the stated side plus `extra`, and
/// `default`'s side where the theme states none.
fn padding_with(part: &PartStyle, extra: f32, default: iced_core::Padding) -> iced_core::Padding {
    let side = |bit: PartStated, v: f32, own: f32| {
        if part.stated.contains(bit) {
            v + extra
        } else {
            own
        }
    };
    let p = &part.padding;
    iced_core::Padding {
        top: side(PartStated::PADDING_TOP, p.top, default.top),
        right: side(PartStated::PADDING_RIGHT, p.right, default.right),
        bottom: side(PartStated::PADDING_BOTTOM, p.bottom, default.bottom),
        left: side(PartStated::PADDING_LEFT, p.left, default.left),
    }
}

/// [`crate::padding_inside_border`] of a part: each stated side plus the
/// part's stroke width ([`line_width`]), `default`'s side where the theme
/// states none.
fn padding_inside(
    part: &PartStyle,
    default: iced_core::Padding,
    home: &'static str,
) -> Mapped<iced_core::Padding> {
    Ok(padding_with(part, line_width(part, home)?, default))
}

/// [`crate::dialog_padding`]: the padding inside a dialog's border
/// (`dialog.panel`'s padding and stroke width), `default`'s side where the
/// theme states none.
///
/// # Errors
///
/// [`Unmapped`] where the panel's stroke is per side.
pub fn dialog_padding(
    r: &ResolvedTheme,
    default: iced_core::Padding,
) -> Mapped<iced_core::Padding> {
    padding_inside(&r.dialog.panel, default, "dialog.panel.stroke.width")
}

/// [`crate::toolbar_padding`]: the padding of a toolbar's bar
/// (`toolbar.bar(NONE).padding`), `default`'s side where the theme states
/// none; the bar's edge is the rule under it, so no line width is added.
#[must_use]
pub fn toolbar_padding(r: &ResolvedTheme, default: iced_core::Padding) -> iced_core::Padding {
    padding_with(&r.toolbar.bar(WidgetState::NONE), 0.0, default)
}

/// [`crate::button_padding`]: the padding inside a button's border
/// (`button.body(NONE)`), iced's `button::DEFAULT_PADDING` side where the
/// theme states none.
///
/// # Errors
///
/// [`Unmapped`] where the body's stroke is per side.
#[cfg(feature = "widgets")]
pub fn button_padding(r: &ResolvedTheme) -> Mapped<iced_core::Padding> {
    padding_inside(
        &r.button.body(WidgetState::NONE),
        iced_widget::button::DEFAULT_PADDING,
        "button.body(NONE).stroke.width",
    )
}

/// [`crate::input_padding`]: the padding inside a text input's border
/// (`input.field(NONE)`), iced's `text_input::DEFAULT_PADDING` side where
/// the theme states none.
///
/// # Errors
///
/// [`Unmapped`] where the field's stroke is per side.
#[cfg(feature = "widgets")]
pub fn input_padding(r: &ResolvedTheme) -> Mapped<iced_core::Padding> {
    padding_inside(
        &r.input.field(InputQuery::from(WidgetState::NONE)),
        iced_widget::text_input::DEFAULT_PADDING,
        "input.field(NONE).stroke.width",
    )
}

/// [`crate::text_area_padding`]: the padding inside a text editor's border
/// (`text_area.field(NONE)`), the text editor's own padding on a side the
/// theme states none for.
///
/// # Errors
///
/// [`Unmapped`] where the field's stroke is per side.
#[cfg(feature = "widgets")]
pub fn text_area_padding(r: &ResolvedTheme) -> Mapped<iced_core::Padding> {
    padding_inside(
        &r.text_area.field(InputQuery::from(WidgetState::NONE)),
        iced_core::Padding::new(crate::TEXT_EDITOR_PADDING),
        "text_area.field(NONE).stroke.width",
    )
}

/// [`crate::combo_box_padding`]: the padding inside a drop-down's border
/// (`combo_box.field(NONE)`), iced's `button::DEFAULT_PADDING` side where
/// the theme states none, the right side widened to centre the chevron
/// (`combo_box.chevron(NONE).size`) in its column
/// (`combo_box.arrow_area_width`) where the theme states the column, the
/// right side and the chevron's size.
///
/// # Errors
///
/// [`Unmapped`] where the field's stroke is per side.
#[cfg(feature = "widgets")]
pub fn combo_box_padding(r: &ResolvedTheme) -> Mapped<iced_core::Padding> {
    combo_box_padding_with(r, chevron_size(r))
}

/// [`combo_box_padding`] with the chevron `chevron` wide, or none where
/// `None`.
#[cfg(feature = "widgets")]
fn combo_box_padding_with(r: &ResolvedTheme, chevron: Option<f32>) -> Mapped<iced_core::Padding> {
    let c = &r.combo_box;
    let field = c.field(WidgetState::NONE);
    let pad = padding_inside(
        &field,
        iced_widget::button::DEFAULT_PADDING,
        "combo_box.field(NONE).stroke.width",
    )?;
    Ok(
        match (
            field.stated.contains(PartStated::PADDING_RIGHT),
            c.arrow_area_width,
            chevron,
        ) {
            (true, Some(column), Some(chevron)) => {
                pad.right(pad.right + crate::arrow_column_margin(column, chevron))
            }
            _ => pad,
        },
    )
}

/// [`crate::button_content_min_size`]: a button's least size on each axis
/// (`button.body(NONE).min_size`) less its padding ([`button_padding`]),
/// no less than 0, for [`crate::at_least`]'s floor; `None` on an axis the
/// theme states no minimum for (issue #4's `Z1`: a minimum width and no
/// minimum height).
///
/// # Errors
///
/// [`Unmapped`] where the body's stroke is per side.
#[cfg(feature = "widgets")]
pub fn button_content_min_size(r: &ResolvedTheme) -> Mapped<[Option<f32>; 2]> {
    let [width, height] = r.button.body(WidgetState::NONE).min_size;
    let p = button_padding(r)?;
    Ok([
        width.map(|w| (w - p.left - p.right).max(0.0)),
        height.map(|h| (h - p.top - p.bottom).max(0.0)),
    ])
}

// The crate root's widgets, widget parts and fonts, each read at its home
// ---------------------------------------------------------------------------

/// [`crate::control_line_height`]: the line height for a single-line
/// control's text that makes the control at least `min_height` tall inside
/// `padding`: the theme's own line box,
/// `typography.line_height_multiplier` times `text_size`, where that
/// reaches the minimum already, and otherwise the height the minimum leaves
/// inside the padding.
#[must_use]
pub fn control_line_height(
    r: &ResolvedTheme,
    text_size: f32,
    min_height: f32,
    padding: iced_core::Padding,
) -> iced_core::text::LineHeight {
    crate::line_height_from(
        r.typography.line_height_multiplier,
        text_size,
        min_height,
        padding,
    )
}

/// [`crate::SwitchLook`] of the 0.8.0 model: the track's least size
/// (`switch.track(NONE).min_size`), the thumb's diameter on
/// (`switch.thumb(CHECKED).min_size[0]`) and off
/// (`switch.thumb(NONE).min_size[0]`).
#[cfg(feature = "widgets")]
pub(crate) fn switch_look(r: &ResolvedTheme) -> Mapped<crate::SwitchLook> {
    let s = &r.switch;
    switch_look_of(
        s.track(WidgetState::NONE).min_size,
        s.thumb(WidgetState::CHECKED).min_size,
        s.thumb(WidgetState::NONE).min_size,
    )
}

/// [`switch_look`] of the least sizes of the track, the checked thumb and
/// the unchecked one, each `Unmapped` where unstated.
#[cfg(feature = "widgets")]
fn switch_look_of(
    [track_width, track_height]: [Option<f32>; 2],
    [thumb_on, _]: [Option<f32>; 2],
    [thumb_off, _]: [Option<f32>; 2],
) -> Mapped<crate::SwitchLook> {
    Ok(crate::SwitchLook {
        track_width: stated(track_width, "switch.track(NONE).min_size[0]")?,
        track_height: stated(track_height, "switch.track(NONE).min_size[1]")?,
        thumb_diameter: stated(thumb_on, "switch.thumb(CHECKED).min_size[0]")?,
        unchecked_thumb_diameter: stated(thumb_off, "switch.thumb(NONE).min_size[0]")?,
    })
}

/// [`crate::switch`]: a switch at the platform's size, a disabled one faded
/// over the window (`colors.window.background`).
///
/// # Errors
///
/// [`Unmapped`] where the theme states no window background, or no value
/// the switch needs.
#[cfg(feature = "widgets")]
pub fn switch<'a, Message, Renderer>(
    r: &ResolvedTheme,
    is_toggled: bool,
    on_toggle: Option<Message>,
) -> Mapped<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>
where
    Message: Clone + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    let window = stated(r.colors.window.background, "colors.window.background")?;
    switch_over(r, is_toggled, on_toggle, to_color(window))
}

/// [`crate::switch_over`]: [`switch`] drawn on `backdrop`.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no value the switch needs.
#[cfg(feature = "widgets")]
pub fn switch_over<'a, Message, Renderer>(
    r: &ResolvedTheme,
    is_toggled: bool,
    on_toggle: Option<Message>,
    backdrop: IcedColor,
) -> Mapped<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>
where
    Message: Clone + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Ok(crate::switch_from(
        switch_look(r)?,
        styles::toggler_over(r, backdrop)?,
        is_toggled,
        on_toggle,
    ))
}

/// [`crate::RadioLook`] of the 0.8.0 model: the circle's width
/// (`radio.indicator(NONE).min_size[0]`, iced's own where the theme states
/// none: spec §10 rule 0; the table of homes already writes 0.7.2's
/// check-box fallback into the radio's home on the presets), the gap to the label
/// (`radio.label_gap`), and, where the theme states the dot's size
/// (`radio.dot(NONE).min_size[0]`), the dot in its checked fill
/// (`radio.dot(CHECKED).fill`).
#[cfg(feature = "widgets")]
pub(crate) fn radio_look(r: &ResolvedTheme) -> Mapped<crate::RadioLook> {
    radio_look_of(r, &r.radio.indicator(WidgetState::NONE))
}

/// [`radio_look`] of the resting circle `indicator`.
#[cfg(feature = "widgets")]
fn radio_look_of(
    r: &ResolvedTheme,
    indicator: &native_theme::next::PartStyle,
) -> Mapped<crate::RadioLook> {
    let w = &r.radio;
    let [own, _] = indicator.min_size;
    let [dot, _] = w.dot(WidgetState::NONE).min_size;
    let dot = match dot {
        Some(d) => Some((
            d,
            fill(&w.dot(WidgetState::CHECKED), "radio.dot(CHECKED).fill")?,
        )),
        None => None,
    };
    Ok(crate::RadioLook {
        width: own,
        label_gap: w.label_gap,
        dot,
    })
}

/// [`crate::radio`]: a radio button at the platform's size, with the
/// platform's dot where the theme states its size.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no value the radio needs.
#[cfg(feature = "widgets")]
pub fn radio<'a, Message, Renderer>(
    r: &ResolvedTheme,
    radio: iced_widget::Radio<'a, Message, iced_core::Theme, Renderer>,
    is_selected: bool,
) -> Mapped<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>
where
    Message: Clone + 'a,
    Renderer: iced_core::text::Renderer + 'a,
{
    Ok(crate::radio_from(
        radio_look(r)?,
        styles::radio(r)?,
        radio,
        is_selected,
    ))
}

/// The drop-down arrow's size (`combo_box.chevron(NONE).size`), `None`
/// where the theme states none.
#[cfg(feature = "widgets")]
pub(crate) fn chevron_size(r: &ResolvedTheme) -> Option<f32> {
    let chevron = r.combo_box.chevron(WidgetState::NONE);
    chevron
        .stated
        .contains(IconStated::SIZE)
        .then_some(chevron.size)
}

/// [`crate::pick_list_handle`]: the renderer's own downward chevron at the
/// drop-down arrow's size (`combo_box.chevron(NONE).size`), at the size iced
/// gives its own handle where the theme states none.
#[cfg(feature = "widgets")]
#[must_use]
pub fn pick_list_handle<Renderer>(
    r: &ResolvedTheme,
) -> iced_widget::pick_list::Handle<Renderer::Font>
where
    Renderer: iced_core::text::Renderer,
{
    crate::handle_at::<Renderer>(chevron_size(r))
}

/// [`crate::IndicatorLook`] of the 0.8.0 model: the selected tab's
/// indicator (`tab.indicator(SELECTED)`: its fill, its least height as the
/// line's thickness) along `tab.indicator_side`, rounded on top as the
/// selected tab's corners (`tab.tab(SELECTED).corners`) where it runs along
/// the top, square, iced's own, where the theme states no corners; `None`
/// where the theme leaves the fill, the thickness or the side unstated, or
/// states a thickness that is not a positive, finite length.
#[cfg(feature = "widgets")]
pub(crate) fn tab_indicator_look(r: &ResolvedTheme) -> Mapped<Option<crate::IndicatorLook>> {
    use iced_core::alignment::Vertical;
    use native_theme::next::TabIndicatorSide;

    let t = &r.tab;
    let indicator = t.indicator(WidgetState::SELECTED);
    let [_, width] = indicator.min_size;
    let (Some(width), Some(side)) = (width, t.indicator_side) else {
        return Ok(None);
    };
    if !indicator.stated.contains(PartStated::FILL) || !width.is_finite() || width <= 0.0 {
        return Ok(None);
    }
    let color = fill(&indicator, "tab.indicator(SELECTED).fill")?;
    let (align, radius) = match side {
        TabIndicatorSide::Top => {
            let tab = t.tab(WidgetState::SELECTED);
            let radius = if tab.stated.contains(PartStated::CORNERS) {
                crate::top_corners(borders::radius(&tab, "tab.tab(SELECTED).corners")?)
            } else {
                iced_core::border::Radius::default()
            };
            (Vertical::Top, radius)
        }
        TabIndicatorSide::Bottom => (Vertical::Bottom, iced_core::border::Radius::default()),
    };
    Ok(Some(crate::IndicatorLook {
        color,
        width,
        align,
        radius,
    }))
}

/// [`crate::tab_indicator`]: `tab` with the selected tab's indicator line
/// laid over it where `active`; `tab` as it is where not `active`, or where
/// the theme states no indicator.
///
/// # Errors
///
/// [`Unmapped`] where the theme states the indicator in a form iced cannot
/// take.
#[cfg(feature = "widgets")]
pub fn tab_indicator<'a, Message, Renderer>(
    r: &ResolvedTheme,
    tab: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
    active: bool,
) -> Mapped<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>
where
    Message: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Ok(crate::tab_indicator_with(
        tab_indicator_look(r)?,
        tab,
        active,
    ))
}

/// [`crate::tab_indicator_line`]: the line that marks the active tab,
/// alone; `None` where the theme states no indicator.
///
/// # Errors
///
/// [`Unmapped`] where the theme states the indicator in a form iced cannot
/// take.
#[cfg(feature = "widgets")]
pub fn tab_indicator_line<'a, Message, Renderer>(
    r: &ResolvedTheme,
) -> Mapped<Option<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>>
where
    Message: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Ok(tab_indicator_look(r)?.map(crate::indicator_line))
}

/// The flares [`tab_flares`] draws: the outward radius
/// (`tab.active_outward_corner_radius`), the selected tab's fill and
/// outline (`tab.tab(SELECTED)`); `None` where the theme states no positive,
/// finite radius, whatever it states for the tab.
#[cfg(feature = "widgets")]
pub(crate) fn tab_flares_look(r: &ResolvedTheme) -> Mapped<Option<crate::tab_flares::FlareLook>> {
    let t = &r.tab;
    let Some(radius) = t
        .active_outward_corner_radius
        .filter(|radius| radius.is_finite() && *radius > 0.0)
    else {
        return Ok(None);
    };
    let tab = t.tab(WidgetState::SELECTED);
    let selected = fill(&tab, "tab.tab(SELECTED).fill")?;
    let (width, outline) = match &tab.stroke {
        _ if !tab.stated.contains(PartStated::STROKE) => {
            return Err(Unmapped("tab.tab(SELECTED).stroke"));
        }
        Stroke::None => (0.0, IcedColor::TRANSPARENT),
        Stroke::Uniform { width, color, .. } => (*width, to_color(*color)),
        _ => return Err(Unmapped("tab.tab(SELECTED).stroke")),
    };
    let bottom_line = tab_indicator_look(r)?
        .is_some_and(|look| look.align == iced_core::alignment::Vertical::Bottom);
    Ok(crate::tab_flares::flares_look_from(
        Some(radius),
        width,
        selected,
        outline,
        bottom_line,
    ))
}

/// [`crate::tab_flares()`]: `tabs` with the selected tab's outward bottom
/// corners drawn over it where the theme states
/// `tab.active_outward_corner_radius`; `tabs` as it is where it states none.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no fill or outline of the selected
/// tab the flares need.
#[cfg(feature = "widgets")]
pub fn tab_flares<'a, Message, Renderer>(
    r: &ResolvedTheme,
    tabs: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
    selected: &[usize],
) -> Mapped<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>
where
    Message: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Ok(crate::tab_flares_with(tab_flares_look(r)?, tabs, selected))
}

/// A focus ring's stroke (`focus_ring.stroke`, or its `inner_stroke` at
/// `home`): its width and colour, `None` where the theme states none.
pub(crate) fn ring_stroke(stroke: &Stroke, home: &'static str) -> Mapped<Option<(f32, IcedColor)>> {
    match stroke {
        Stroke::None => Ok(None),
        Stroke::Uniform { width, color, .. } => Ok(Some((*width, to_color(*color)))),
        _ => Err(Unmapped(home)),
    }
}

/// The ring [`focus_ring`] draws round a field of corner `radius`: the
/// ring's stroke (`focus_ring.stroke`), its offset (`focus_ring.offset`),
/// whether it shows for keyboard focus only (`focus_ring.trigger`), a
/// second ring inside it (`focus_ring.inner_stroke`) and the corners of the
/// line the offset places, between the two (`focus_ring.corners`, the
/// field's grown by the offset where the theme states none; spec §2,
/// `FocusIndicator`); `None` where the theme states no stroke. The ring is
/// painted over the field: one inset at the field's edge (offset minus its
/// width, libadwaita's) is the band over the field's outermost width, which
/// the field styles leave to it ([`styles::text_input`]).
///
/// # Errors
///
/// [`Unmapped`] where the indicator is not a ring (`focus_ring.kind`), or a
/// stroke is per side or the corners concentric ([`borders::corner_radii`]).
#[cfg(feature = "widgets")]
pub fn ring_look(r: &ResolvedTheme, radius: f32) -> Mapped<Option<RingLook>> {
    use native_theme::next::{FocusKind, FocusTrigger};

    let f = &r.focus_ring;
    if f.kind != FocusKind::Ring {
        return Err(Unmapped("focus_ring.kind"));
    }
    let Some((width, color)) = ring_stroke(&f.stroke, "focus_ring.stroke")? else {
        return Ok(None);
    };
    let inner = ring_stroke(&f.inner_stroke, "focus_ring.inner_stroke")?
        .map(|(width, color)| (color, width));
    let corners = match &f.corners {
        Some(corners) => Some(borders::corner_radii(corners, "focus_ring.corners")?),
        None => None,
    };
    Ok(crate::focus_ring::ring_look_from(
        color,
        width,
        f.offset,
        radius,
        f.trigger == Some(FocusTrigger::KeyboardOnly),
    )
    .map(|look| look.with(inner, corners)))
}

/// [`crate::focus_ring()`]: `field`, and round it while it holds the keyboard
/// focus, the theme's focus ring, its curves the field's `corner_radius`
/// grown by the ring's offset, or the ring's own corners where the theme
/// states them, and the second ring inside it where the theme states one;
/// painted over the field, a ring inset at its edge included.
///
/// # Errors
///
/// [`Unmapped`] where the indicator is not a ring, a stroke is per side or
/// the corners concentric.
#[cfg(feature = "widgets")]
pub fn focus_ring<'a, Message, Renderer>(
    r: &ResolvedTheme,
    corner_radius: f32,
    field: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
) -> Mapped<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>
where
    Message: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Ok(focus_ring_with(ring_look(r, corner_radius)?, field))
}

/// [`focus_ring`] of a ring [`ring_look`] mapped: `field`, and round it
/// while it holds the keyboard focus, `look`; the field with no ring where
/// `look` is `None`. A caller that draws the field alone where the ring is
/// [`Unmapped`] asks [`ring_look`] once and hands the field here, which
/// [`focus_ring`] cannot do, as it keeps the field it is given.
#[cfg(feature = "widgets")]
pub fn focus_ring_with<'a, Message, Renderer>(
    look: Option<RingLook>,
    field: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
) -> iced_core::Element<'a, Message, iced_core::Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    crate::focus_ring::FocusRing::new(field.into(), look).into()
}

/// What [`spinner`] reads: its diameter, stroke width, colour and period
/// (`spinner`), and the window's text colour (`colors.window.foreground`)
/// for a freedesktop indicator's `currentColor`.
#[cfg(feature = "spinner")]
pub(crate) fn spinner_look(r: &ResolvedTheme) -> Mapped<crate::spinner::Look> {
    let s = &r.spinner;
    let text = stated(r.colors.window.foreground, "colors.window.foreground")?;
    let period_ms = match s.period_ms {
        Some(ms) => Some(std::num::NonZeroU32::new(ms).ok_or(Unmapped("spinner.period_ms"))?),
        None => None,
    };
    Ok(crate::spinner::Look {
        diameter: s.diameter,
        stroke_width: s.stroke_width,
        color: to_color(s.color),
        current_color: to_color(text),
        period_ms,
    })
}

/// [`crate::Spinner::new`]: the spinner of `icon_set` (for
/// `IconSet::Freedesktop` the indicator of `icon_theme`, the system's icon
/// theme where `None`), or an arc at the theme's stroke, turning once per
/// `spinner.period_ms` where the theme states it, else a turn a second.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no window text colour, or a period
/// of 0.
#[cfg(feature = "spinner")]
pub fn spinner(
    r: &ResolvedTheme,
    icon_set: crate::IconSet,
    icon_theme: Option<&str>,
) -> Mapped<crate::Spinner> {
    Ok(crate::Spinner::of_look(
        spinner_look(r)?,
        icon_set,
        icon_theme,
    ))
}

/// A text style's slant as the 0.7.2 model's, which the font database
/// selects a face by, where the style states its family, weight and slant.
#[cfg(feature = "system-fonts")]
fn face_style(text: &TextStyle) -> Mapped<native_theme::theme::FontStyle> {
    use native_theme::next::FontStyle;
    use native_theme::theme::FontStyle as Slant;

    for (bit, home) in [
        (TextStated::FAMILY, "TextStyle.family"),
        (TextStated::WEIGHT, "TextStyle.weight"),
        (TextStated::STYLE, "TextStyle.style"),
    ] {
        if !text.stated.contains(bit) {
            return Err(Unmapped(home));
        }
    }
    match text.style {
        FontStyle::Normal => Ok(Slant::Normal),
        FontStyle::Italic => Ok(Slant::Italic),
        FontStyle::Oblique => Ok(Slant::Oblique),
        _ => Err(Unmapped("TextStyle.style")),
    }
}

/// [`crate::system_font_family`] of a text style (`typography.body`,
/// `button.label(NONE)`, ...): the family iced draws its family, weight and
/// slant in, the platform's substitute where the system lacks the family.
/// A width class the style states does not choose the family: iced picks
/// the face of that width within the family by the `Font`'s stretch
/// (iced_graphics 0.14.0 `src/text.rs`, `to_stretch`), which [`font`] sets.
///
/// # Errors
///
/// [`Unmapped`] where the style states no family, weight or slant, or a
/// slant iced has no face for.
#[cfg(feature = "system-fonts")]
pub fn system_font_family(text: &TextStyle) -> Mapped<std::sync::Arc<str>> {
    let style = face_style(text)?;
    Ok(crate::family_drawn(
        &text.family,
        text.weight,
        style,
        native_theme::fonts::substitute_family_cached,
    ))
}

/// [`crate::font`] of a text style (`typography.body`,
/// `button.label(NONE)`, ...): the iced `Font` its family, weight and slant
/// are drawn in, at its width class (`stretch`) where the style states
/// one, iced's normal stretch where it states none.
///
/// # Errors
///
/// [`Unmapped`] where the style states no family, weight or slant, or a
/// slant iced has no face for.
#[cfg(feature = "system-fonts")]
pub fn font(text: &TextStyle) -> Mapped<iced_core::Font> {
    let style = face_style(text)?;
    let font = crate::font_of(&text.family, text.weight, style);
    if !text.stated.contains(TextStated::WIDTH) {
        return Ok(font);
    }
    Ok(iced_core::Font {
        stretch: stretch(text.width)?,
        ..font
    })
}

/// iced's stretch of a width class: OpenType's `usWidthClass` 1 to 9,
/// ultra-condensed to ultra-expanded (the OpenType `OS/2` table's
/// `usWidthClass`, as `FontWidth` states), each the variant of
/// `iced_core::font::Stretch` (0.14.0 `src/font.rs`) of that name.
#[cfg(feature = "system-fonts")]
fn stretch(width: native_theme::next::FontWidth) -> Mapped<iced_core::font::Stretch> {
    use iced_core::font::Stretch;

    Ok(match width.value() {
        1 => Stretch::UltraCondensed,
        2 => Stretch::ExtraCondensed,
        3 => Stretch::Condensed,
        4 => Stretch::SemiCondensed,
        5 => Stretch::Normal,
        6 => Stretch::SemiExpanded,
        7 => Stretch::Expanded,
        8 => Stretch::ExtraExpanded,
        9 => Stretch::UltraExpanded,
        _ => return Err(Unmapped("TextStyle.width")),
    })
}
