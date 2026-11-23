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
//! (spec §10, rule 0).
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
//! A border's colour is the stroke as iced on wgpu paints it, which ends a
//! widget's fill at the border's inner edge (`iced_wgpu` 0.14.0,
//! `src/shader/quad/solid.wgsl`): an `Outside` stroke is drawn as stated; an
//! `Inside` one, which the theme lays over the fill, is the stroke composited
//! over the fill; a `Center` one, over the fill at half its alpha, the band's
//! mean (spec §2, `StrokeAlign`). Under an opaque stroke the three are the
//! stated colour. iced on tiny-skia fills the whole bounds under the border,
//! as gpui does; a style function does not know the renderer, so the
//! connector computes for wgpu and its README records tiny-skia.

// The part readers serve the style functions, which the `widgets` feature
// builds.
#![cfg_attr(not(feature = "widgets"), allow(dead_code))]

#[cfg(feature = "widgets")]
pub mod styles;

#[cfg(test)]
mod parity;

use std::fmt;

use iced_core::border::Radius;
use iced_core::{Border, Color as IcedColor};
use native_theme::next::{
    Color, CornerCurve, CornerRadius, InputQuery, Paint, PartStated, PartStyle, ResolvedTheme,
    Scheme, Stroke, StrokeAlign, TextStated, TextStyle, WidgetState,
};

use crate::ColorMode;
use crate::extended::OverrideColors;

/// `layer` composited over `base`, as [`crate::styles::composite_over`]
/// composites them.
fn composite_over(layer: IcedColor, base: IcedColor) -> IcedColor {
    let [r, g, b, a] = native_theme::color::composite_over(
        [layer.r, layer.g, layer.b, layer.a],
        [base.r, base.g, base.b, base.a],
    );
    IcedColor { r, g, b, a }
}

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

/// A colour of the model as iced's: the same four gamma-encoded channels
/// with straight alpha.
#[must_use]
pub fn to_color(c: Color) -> IcedColor {
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

/// A part's fill, where stated as one colour.
pub(crate) fn fill(part: &PartStyle, home: &'static str) -> Mapped<IcedColor> {
    match &part.fill {
        Paint::Solid(c) if part.stated.contains(PartStated::FILL) => Ok(to_color(*c)),
        _ => Err(Unmapped(home)),
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

/// A part's stroke colour as iced on wgpu shows it (the module's doc), where
/// stated as one stroke of one colour: as stated where it is opaque or
/// `Outside`; otherwise composited over the part's own fill, which must then
/// be stated as one colour too.
pub(crate) fn stroke_color(part: &PartStyle, home: &'static str) -> Mapped<IcedColor> {
    let (color, align) = match &part.stroke {
        Stroke::Uniform { color, align, .. } if part.stated.contains(PartStated::STROKE) => {
            (to_color(*color), *align)
        }
        _ => return Err(Unmapped(home)),
    };
    if color.a >= 1.0 {
        return Ok(color);
    }
    match align {
        StrokeAlign::Outside => Ok(color),
        StrokeAlign::Inside => Ok(composite_over(color, fill(part, home)?)),
        StrokeAlign::Center => {
            let under = fill(part, home)?;
            Ok(composite_over(
                color,
                IcedColor {
                    a: under.a / 2.0,
                    ..under
                },
            ))
        }
        _ => Err(Unmapped(home)),
    }
}

/// A part's stroke colour where a rule states it for the queried state, as
/// [`stroke_color`] reads it; `None` where none does.
pub(crate) fn state_stroke_color(
    part: &PartStyle,
    home: &'static str,
) -> Mapped<Option<IcedColor>> {
    if part.from_state.contains(PartStated::STROKE) {
        stroke_color(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// A part's stroke width, where stated as one stroke, in any alignment, or
/// as none, which is no width.
pub(crate) fn stroke_width(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    if !part.stated.contains(PartStated::STROKE) {
        return Err(Unmapped(home));
    }
    match &part.stroke {
        Stroke::None => Ok(0.0),
        Stroke::Uniform { width, .. } => Ok(*width),
        _ => Err(Unmapped(home)),
    }
}

/// A part's corner radius, where stated as one circular radius, or as square
/// corners, which are no radius.
pub(crate) fn radius(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    if !part.stated.contains(PartStated::CORNERS) {
        return Err(Unmapped(home));
    }
    match (part.corners.radius, part.corners.curve) {
        (CornerRadius::None, _) => Ok(0.0),
        (CornerRadius::Uniform(r), CornerCurve::Circular) => Ok(r),
        _ => Err(Unmapped(home)),
    }
}

/// A part's border as iced outlines a widget, [`crate::styles::border_of`]'s
/// three values: the stroke's colour ([`stroke_color`]) and width where the
/// stroke is stated (a stated `Stroke::None` is no width, its colour
/// iced's), the corner radius where the corners are, and iced's own
/// (`Border::default()`, no line and square corners) where they are not
/// (spec §10, rule 0).
pub(crate) fn border_of(
    part: &PartStyle,
    color_home: &'static str,
    width_home: &'static str,
    corners_home: &'static str,
) -> Mapped<Border> {
    let mut border = Border::default();
    if part.stated.contains(PartStated::STROKE) {
        border.width = stroke_width(part, width_home)?;
        if !matches!(part.stroke, Stroke::None) {
            border.color = stroke_color(part, color_home)?;
        }
    }
    if part.stated.contains(PartStated::CORNERS) {
        border.radius = Radius::new(radius(part, corners_home)?);
    }
    Ok(border)
}

/// [`border_of`] the part at `$home`, each value's home under it.
macro_rules! border {
    ($part:expr, $home:literal) => {
        $crate::next::border_of(
            &$part,
            concat!($home, ".stroke.color"),
            concat!($home, ".stroke.width"),
            concat!($home, ".corners"),
        )
    };
}
pub(crate) use border;

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
        Scheme::Dark => ColorMode::Dark,
        _ => ColorMode::Light,
    }
}

/// [`crate::palette::to_palette`]: iced's six-colour palette.
///
/// # Errors
///
/// [`Unmapped`] where the theme states no window background or foreground.
pub fn to_palette(r: &ResolvedTheme) -> Mapped<iced_core::theme::Palette> {
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
