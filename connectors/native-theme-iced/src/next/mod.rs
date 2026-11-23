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
//! ([`crate::styles::Line`]), and the shared style function hands iced its
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

use std::fmt;
use std::time::Duration;

use iced_core::Color as IcedColor;
use native_theme::next::{
    Color, DialogButtonOrder, IconSizes, InputQuery, Orientation, Paint, PartStated, PartStyle,
    ResolvedTheme, Scheme, Stroke, StyleQuery, TextStated, TextStyle, WidgetState,
};

use crate::ColorMode;
use crate::extended::OverrideColors;

/// A source a value needs that the theme leaves unstated, or states in a
/// form the connector cannot hand to iced (a gradient fill, a per-side
/// stroke, a squircle): its place in the 0.8.0 model, as a query and a
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
    if !part.stated.contains(PartStated::FILL) {
        return Err(Unmapped(home));
    }
    match &part.fill {
        Paint::None => Ok(IcedColor::TRANSPARENT),
        Paint::Solid(c) => Ok(to_color(*c)),
        paint @ Paint::Complex(_) => paint.fallback_color().map(to_color).ok_or(Unmapped(home)),
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

/// A text style's colour, where stated.
pub(crate) fn text_color(text: &TextStyle, home: &'static str) -> Mapped<IcedColor> {
    if text.stated.contains(TextStated::COLOR) {
        Ok(to_color(text.color))
    } else {
        Err(Unmapped(home))
    }
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
/// at its home, or the body's where the theme defines no such role (the
/// table of homes, rows `text_scale.*`, `home_fallback`).
#[derive(Clone, Copy, Debug)]
pub struct TextScale<'a> {
    /// `typography.caption`.
    pub caption: &'a TextStyle,
    /// `typography.headline`.
    pub section_heading: &'a TextStyle,
    /// `typography.title1`.
    pub dialog_title: &'a TextStyle,
    /// `typography.display`.
    pub display: &'a TextStyle,
}

/// [`crate::text_scale`]: the four text roles 0.7.2 names, each a size,
/// weight and line height as the theme states them, already scaled by the
/// text scale the theme was resolved for.
#[must_use]
pub fn text_scale(r: &ResolvedTheme) -> TextScale<'_> {
    let t = &r.typography;
    TextScale {
        caption: t.caption.as_ref().unwrap_or(&t.body),
        section_heading: t.headline.as_ref().unwrap_or(&t.body),
        dialog_title: t.title1.as_ref().unwrap_or(&t.body),
        display: t.display.as_ref().unwrap_or(&t.body),
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
