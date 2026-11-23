//! The widgets on the 0.8.0 model ([`native_theme::next`](native_theme_egui::native_theme::next)),
//! beside the 0.7.2 path until the 0.8.0 cut removes it: the reads each widget's `next_of`
//! builds its look from, at the homes the table of homes (`native-theme/tests/next_homes.toml`)
//! names for the 0.7.2 fields the widget's `of` reads.
//!
//! Each read gives a value where the theme states it, `None` where it does not, so the widget
//! keeps egui's own there (spec §10, rule 0); a fill stated as none is transparent and a stroke
//! stated as none is no outline (spec §10, rule 2).

#[cfg(test)]
pub(crate) mod parity;

use native_theme_egui::native_theme::color::Rgba;
use native_theme_egui::native_theme::next::{
    Color, CornerCurve, CornerRadius, IconStated, IconStyle, Paint, PartStated, PartStyle,
    ResolvedTheme, Stroke, TextStated, TextStyle,
};

/// A colour of the 0.8.0 model as the 0.7.2 model's: its 8-bit sRGB channels with straight
/// alpha.
pub(crate) fn rgba(c: Color) -> Rgba {
    let [r, g, b, a] = c.to_srgb8();
    Rgba::new(r, g, b, a)
}

/// A part's fill, where stated: its one colour, transparent for a fill stated as none, and a
/// layered paint's fallback colour (spec §10, rule 2); `None` for a layered paint without one.
pub(crate) fn fill(part: &PartStyle) -> Option<Rgba> {
    if !part.stated.contains(PartStated::FILL) {
        return None;
    }
    match &part.fill {
        Paint::None => Some(Rgba::new(0, 0, 0, 0)),
        Paint::Solid(c) => Some(rgba(*c)),
        paint @ Paint::Complex(_) => paint.fallback_color().map(rgba),
    }
}

/// [`fill`] of `part` where `ruled`, the part queried with the state's flag alone, has its
/// fill from a rule for that state (spec §2, `from_state`): a state's fill laid over another
/// state (`HOVER|CHECKED`), read only where the theme states one for the state.
pub(crate) fn ruled_fill(ruled: &PartStyle, part: &PartStyle) -> Option<Rgba> {
    ruled
        .from_state
        .contains(PartStated::FILL)
        .then(|| fill(part))
        .flatten()
}

/// A part's outline colour, where stated as one stroke of one colour; `None` where it is not
/// stated, stated as none (no outline), or per side.
pub(crate) fn stroke(part: &PartStyle) -> Option<Rgba> {
    match &part.stroke {
        Stroke::Uniform { color, .. } if part.stated.contains(PartStated::STROKE) => {
            Some(rgba(*color))
        }
        _ => None,
    }
}

/// [`stroke`] of `part` where `ruled`, the part queried with the state's flag alone, has its
/// stroke from a rule for that state.
pub(crate) fn ruled_stroke(ruled: &PartStyle, part: &PartStyle) -> Option<Rgba> {
    ruled
        .from_state
        .contains(PartStated::STROKE)
        .then(|| stroke(part))
        .flatten()
}

/// A part's outline width, where stated as one stroke; `None` where it is not stated, stated
/// as none (no outline), or per side.
pub(crate) fn stroke_width(part: &PartStyle) -> Option<f32> {
    match &part.stroke {
        Stroke::Uniform { width, .. } if part.stated.contains(PartStated::STROKE) => Some(*width),
        _ => None,
    }
}

/// A part's corner radius on a box `width` × `height`, where stated as one circular radius,
/// square corners (no radius) or a pill (half the shorter side, of a box whose size is known);
/// `None` where it is not stated or stated in a form egui cannot draw as one radius.
pub(crate) fn radius(part: &PartStyle, width: Option<f32>, height: Option<f32>) -> Option<f32> {
    if !part.stated.contains(PartStated::CORNERS) {
        return None;
    }
    match (part.corners.radius, part.corners.curve) {
        (CornerRadius::None, _) => Some(0.0),
        (CornerRadius::Uniform(r), CornerCurve::Circular) => Some(r),
        (CornerRadius::Pill, CornerCurve::Circular) => Some(0.5 * width?.min(height?)),
        _ => None,
    }
}

/// A text style's colour, where stated.
pub(crate) fn text(t: &TextStyle) -> Option<Rgba> {
    t.stated.contains(TextStated::COLOR).then(|| rgba(t.color))
}

/// An icon's colour, where stated.
pub(crate) fn icon(i: &IconStyle) -> Option<Rgba> {
    i.stated.contains(IconStated::COLOR).then(|| rgba(i.color))
}

/// [`text`] where a rule states it for the queried state.
pub(crate) fn state_text(t: &TextStyle) -> Option<Rgba> {
    t.from_state
        .contains(TextStated::COLOR)
        .then(|| text(t))
        .flatten()
}

/// The window's background, `colors.window.background`, where stated: the ground a disabled
/// widget is faded over as one.
pub(crate) fn window_background(r: &ResolvedTheme) -> Option<Rgba> {
    r.colors.window.background.map(rgba)
}

/// The body text's colour, `typography.body.color`, where stated.
pub(crate) fn body_text(r: &ResolvedTheme) -> Option<Rgba> {
    text(&r.typography.body)
}
