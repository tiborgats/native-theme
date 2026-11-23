//! The scaffold the platform builders share (spec §6, resolution step 2):
//! the physical sides of a reading direction, the linear text scale, and the
//! motion preference with the system's animation scale.

use crate::next::geometry::Edges;
use crate::next::motion::{DialogEntranceStyle, MotionTokens, ThemeChangePolicy, Transition};
use crate::next::query::{Direction, MotionPreference};
use crate::next::style::FontSize;

/// `edges`, stated for a left-to-right layout, on the physical sides of
/// `direction`: as given from left to right, its left and right sides
/// swapped from right to left (spec §6, resolution step 2: "Emit physical
/// left/right geometry").
pub(crate) const fn physical_edges(edges: Edges, direction: Direction) -> Edges {
    match direction {
        Direction::Ltr => edges,
        Direction::Rtl => Edges::new(edges.top, edges.left, edges.bottom, edges.right),
    }
}

/// A font size in logical pixels on a platform that scales text linearly
/// (GNOME, KDE, Windows and macOS; spec §6, resolution step 2): `size`
/// converted at the font DPI `font_dpi`, times the text scale `text_scale`,
/// then rounded by `whole_pixels`, the platform's own rounding, where it
/// draws fonts at whole pixels.
pub(crate) fn linear_font_px(
    size: FontSize,
    font_dpi: f32,
    text_scale: f32,
    whole_pixels: Option<fn(f32) -> f32>,
) -> f32 {
    let px = size.to_logical_px(font_dpi) * text_scale;
    whole_pixels.map_or(px, |round| round(px))
}

/// How the motion of a resolve runs (spec §6, resolution step 2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Motion {
    /// No motion: every transition takes 0 ms and the dialog appears at once.
    Collapsed,
    /// Every positive duration times this factor, above 0.
    Scaled(f32),
}

/// The motion of `motion` and the system's animation scale `animation_scale`:
/// collapsed under `MotionPreference::None` or a scale of 0, else scaled by
/// the scale where it is given and valid (finite and not below 0, spec §8,
/// `SystemOverrides::animation_scale`), else by 1. `MotionPreference::Reduced`
/// is each platform's policy, which its builder applies; here it runs as
/// `Full`.
pub(crate) fn motion(motion: MotionPreference, animation_scale: Option<f32>) -> Motion {
    let scale = animation_scale.filter(|s| s.is_finite() && *s >= 0.0);
    match (motion, scale) {
        (MotionPreference::None, _) => Motion::Collapsed,
        (_, Some(s)) if s > 0.0 => Motion::Scaled(s),
        (_, Some(_)) => Motion::Collapsed,
        (_, None) => Motion::Scaled(1.0),
    }
}

/// `ms` under `motion`, rounded to the nearest millisecond.
fn scaled_ms(ms: u32, motion: Motion) -> u32 {
    match motion {
        Motion::Collapsed => 0,
        // A float-to-integer `as` saturates: a product beyond `u32::MAX` is `u32::MAX`.
        Motion::Scaled(factor) => (f64::from(ms) * f64::from(factor)).round() as u32,
    }
}

/// `transition` under `motion`: its duration and its delay scaled, both 0
/// where collapsed.
pub(crate) fn transition(transition: Transition, motion: Motion) -> Transition {
    Transition {
        duration_ms: scaled_ms(transition.duration_ms, motion),
        delay_ms: scaled_ms(transition.delay_ms, motion),
        ..transition
    }
}

/// `tokens` under `motion`: each transition as [`transition`] makes it, and a
/// theme change's duration scaled too.
pub(crate) fn motion_tokens(tokens: MotionTokens, motion: Motion) -> MotionTokens {
    let each = |t: Option<Transition>| t.map(|t| transition(t, motion));
    MotionTokens {
        fast: each(tokens.fast),
        standard: each(tokens.standard),
        emphasized: each(tokens.emphasized),
        spring_spatial: each(tokens.spring_spatial),
        spring_effects: each(tokens.spring_effects),
        theme_change: tokens.theme_change.map(|policy| match policy {
            ThemeChangePolicy::Crossfade { duration_ms } => ThemeChangePolicy::Crossfade {
                duration_ms: scaled_ms(duration_ms, motion),
            },
            ThemeChangePolicy::SuspendTransitions { duration_ms } => {
                ThemeChangePolicy::SuspendTransitions {
                    duration_ms: scaled_ms(duration_ms, motion),
                }
            }
            other => other,
        }),
    }
}

/// The dialog's entrance `entrance` under `motion`: `DialogEntranceStyle::None`
/// where collapsed, else as given.
pub(crate) fn dialog_entrance(
    entrance: Option<DialogEntranceStyle>,
    motion: Motion,
) -> Option<DialogEntranceStyle> {
    match motion {
        Motion::Collapsed => Some(DialogEntranceStyle::None),
        Motion::Scaled(_) => entrance,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::motion::Timing;

    #[test]
    fn right_to_left_swaps_the_left_and_right_sides_only() {
        let edges = Edges::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(physical_edges(edges, Direction::Ltr), edges);
        assert_eq!(
            physical_edges(edges, Direction::Rtl),
            Edges::new(1.0, 4.0, 3.0, 2.0)
        );
    }

    #[test]
    fn a_font_size_is_scaled_after_the_dpi_conversion_and_rounded_after_the_scale() {
        // 10 pt at 96 DPI is 13.333 px; times 1.5 is 20 px, which no rounding of 13.333 px
        // first gives (13 × 1.5 = 19.5).
        let ten_pt = FontSize::Pt(10.0);
        let round: fn(f32) -> f32 = f32::round;
        assert_eq!(linear_font_px(ten_pt, 96.0, 1.5, Some(round)), 20.0);
        assert_eq!(linear_font_px(ten_pt, 96.0, 1.0, Some(round)), 13.0);
        assert_eq!(linear_font_px(FontSize::Px(9.0), 96.0, 2.0, None), 18.0);
    }

    #[test]
    fn no_motion_or_a_zero_scale_collapses_and_a_scale_multiplies() {
        let t = Transition {
            duration_ms: 200,
            delay_ms: 50,
            timing: Timing::Linear,
        };
        for collapsed in [
            motion(MotionPreference::None, None),
            motion(MotionPreference::None, Some(2.0)),
            motion(MotionPreference::Full, Some(0.0)),
        ] {
            assert_eq!(collapsed, Motion::Collapsed);
            let t = transition(t, collapsed);
            assert_eq!((t.duration_ms, t.delay_ms), (0, 0));
            assert_eq!(
                dialog_entrance(Some(DialogEntranceStyle::FadeRise), collapsed),
                Some(DialogEntranceStyle::None)
            );
        }
        let half = motion(MotionPreference::Full, Some(0.5));
        let scaled = transition(t, half);
        assert_eq!((scaled.duration_ms, scaled.delay_ms), (100, 25));
        assert_eq!(
            dialog_entrance(Some(DialogEntranceStyle::FadeRise), half),
            Some(DialogEntranceStyle::FadeRise)
        );
        for unscaled in [
            motion(MotionPreference::Full, None),
            motion(MotionPreference::Reduced, None),
            motion(MotionPreference::Full, Some(f32::NAN)),
            motion(MotionPreference::Full, Some(-1.0)),
        ] {
            assert_eq!(unscaled, Motion::Scaled(1.0));
            assert_eq!(transition(t, unscaled), t);
        }
    }

    #[test]
    fn a_theme_change_s_duration_is_scaled_with_the_transitions() {
        let tokens = MotionTokens {
            theme_change: Some(ThemeChangePolicy::Crossfade { duration_ms: 300 }),
            ..MotionTokens::default()
        };
        assert_eq!(
            motion_tokens(tokens, Motion::Scaled(2.0)).theme_change,
            Some(ThemeChangePolicy::Crossfade { duration_ms: 600 })
        );
        assert_eq!(
            motion_tokens(tokens, Motion::Collapsed).theme_change,
            Some(ThemeChangePolicy::Crossfade { duration_ms: 0 })
        );
    }
}
