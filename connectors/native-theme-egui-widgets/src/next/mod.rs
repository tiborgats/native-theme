//! The widgets on the 0.8.0 model ([`native_theme::next`](native_theme_egui::native_theme::next)),
//! beside the 0.7.2 path until the 0.8.0 cut removes it: the reads each widget's `next_of`
//! builds its look from, at the homes the table of homes (`native-theme/tests/next_homes.toml`)
//! names for the 0.7.2 fields the widget's `of` reads.
//!
//! A part's or a text style's value is read through the connector's own readers
//! ([`native_theme_egui::next`]), so the widgets draw a stated value as the connector hands it
//! to egui: each gives a value where the theme states it, `None` where it does not, so the
//! widget keeps egui's own there (spec §10, rule 0); a fill stated as none is transparent and a
//! stroke stated as none is no outline (spec §10, rule 2). Kept here are only the reads the
//! connector has no reader for: a state's value read where a rule for the state states it, the
//! corners of a pill, and the colours no part's `stated` bits govern.

#[cfg(test)]
pub(crate) mod parity;

pub(crate) use native_theme_egui::next::{
    disabled_text_color, fill_of, radius_of, state_text_color_of, stroke_color_of, stroke_width_of,
    text_color_of, to_color32,
};

use native_theme_egui::egui::Color32;
use native_theme_egui::native_theme::next::{
    CornerRadius, IconStated, IconStyle, PartStated, PartStyle, ResolvedTheme,
};

/// [`fill_of`] `part` where `ruled`, the part queried with the state's flag alone, has its
/// fill from a rule for that state (spec §2, `from_state`): a state's fill laid over another
/// state (`HOVER|CHECKED`), read only where the theme states one for the state.
pub(crate) fn ruled_fill(ruled: &PartStyle, part: &PartStyle) -> Option<Color32> {
    ruled
        .from_state
        .contains(PartStated::FILL)
        .then(|| fill_of(part))
        .flatten()
}

/// [`stroke_color_of`] `part` where `ruled`, the part queried with the state's flag alone, has
/// its stroke from a rule for that state.
pub(crate) fn ruled_stroke(ruled: &PartStyle, part: &PartStyle) -> Option<Color32> {
    ruled
        .from_state
        .contains(PartStated::STROKE)
        .then(|| stroke_color_of(part))
        .flatten()
}

/// A part's corner radius on a box `width` × `height`: [`radius_of`], or for a pill, half the
/// shorter side of a box whose size is known (on another, [`radius_of`]'s pill, which epaint
/// clamps to half the shorter side); `None` where it is not stated or stated in a form egui
/// cannot draw as one radius.
pub(crate) fn radius(part: &PartStyle, width: Option<f32>, height: Option<f32>) -> Option<f32> {
    match (part.corners.radius, width.zip(height)) {
        (CornerRadius::Pill, Some((width, height)))
            if part.stated.contains(PartStated::CORNERS) =>
        {
            Some(0.5 * width.min(height))
        }
        _ => radius_of(part),
    }
}

/// An icon's colour, where stated.
pub(crate) fn icon(i: &IconStyle) -> Option<Color32> {
    i.stated
        .contains(IconStated::COLOR)
        .then(|| to_color32(i.color))
}

/// The window's background, `colors.window.background`, where stated: the ground a disabled
/// widget is faded over as one.
pub(crate) fn window_background(r: &ResolvedTheme) -> Option<Color32> {
    r.colors.window.background.map(to_color32)
}

/// The body text's colour, `typography.body.color`, where stated.
pub(crate) fn body_text(r: &ResolvedTheme) -> Option<Color32> {
    text_color_of(&r.typography.body)
}
