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

use std::fmt;

use iced_core::Color as IcedColor;
use native_theme::next::{
    Color, InputQuery, Paint, PartStated, PartStyle, ResolvedTheme, Scheme, TextStated, TextStyle,
    WidgetState,
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

/// The theme's colour mode.
fn mode_of(r: &ResolvedTheme) -> ColorMode {
    match r.context.scheme {
        Scheme::Light => ColorMode::Light,
        Scheme::Dark => ColorMode::Dark,
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
    Ok(crate::theme_from(
        &r.name,
        to_palette(r)?,
        override_colors(r)?,
        Some(mode_of(r)),
    ))
}
