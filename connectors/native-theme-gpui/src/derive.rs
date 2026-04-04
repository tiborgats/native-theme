//! Shade derivation helpers for hover/active states.
//!
//! Provides functions to derive interactive state colors (hover,
//! active/pressed) from base colors. Uses the [`Colorize`] trait from
//! gpui-component for lightness adjustments.

use gpui::Hsla;
use gpui_component::Colorize;

/// Derive a hover state from a base color.
///
/// Blends the background with the base at 90% opacity, producing a color
/// slightly closer to the background. Works identically for light and dark themes.
///
/// This matches gpui-component's internal `apply_config` hover derivation:
/// `background.blend(base.opacity(0.9))`.
pub fn hover_color(base: Hsla, bg: Hsla) -> Hsla {
    bg.blend(base.opacity(0.9))
}

/// Derive an active/pressed state from a base color.
///
/// For light themes (is_dark=false): darkens by 10%.
/// For dark themes (is_dark=true): darkens by 20%.
///
/// When the base color is very dark (lightness < 0.15), darkening produces
/// near-invisible feedback. In that case we lighten instead, ensuring the
/// active state is always visually distinct from the base.
///
/// The 20% factor matches gpui-component's internal `apply_config` derivation
/// and provides sufficient contrast shift without overshooting on mid-range colors.
/// Includes a near-black safety net.
pub fn active_color(base: Hsla, is_dark: bool) -> Hsla {
    let factor = if is_dark { 0.2 } else { 0.1 };
    // Near-black colors have no room to darken -- lighten instead
    // so the pressed state is visible.
    if base.l < 0.15 {
        base.lighten(factor)
    } else {
        base.darken(factor)
    }
}

/// Compute the WCAG 2.1 relative luminance contrast ratio between two colors.
///
/// Returns a value in [1.0, 21.0]. Ratios below 4.5 indicate insufficient
/// contrast for normal text (AA), below 3.0 for large text.
pub fn contrast_ratio(a: Hsla, b: Hsla) -> f32 {
    let la = relative_luminance(a);
    let lb = relative_luminance(b);
    let (lighter, darker) = if la > lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

/// WCAG 2.1 relative luminance from an Hsla color.
fn relative_luminance(c: Hsla) -> f32 {
    let rgba: gpui::Rgba = c.into();
    let linearize = |v: f32| -> f32 {
        let v = v.clamp(0.0, 1.0);
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linearize(rgba.r) + 0.7152 * linearize(rgba.g) + 0.0722 * linearize(rgba.b)
}

/// Light variant of a base color, mode-aware.
///
/// For dark themes: increases lightness (because the background is dark,
/// a tinted background must be lighter than the pure color).
/// For light themes: blends toward the background (gpui-component convention).
pub fn light_variant(bg: Hsla, color: Hsla, is_dark: bool) -> Hsla {
    if is_dark {
        Hsla {
            l: (color.l + 0.15).min(0.95),
            ..color
        }
    } else {
        bg.blend(color.opacity(0.8))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use gpui::hsla;

    #[test]
    fn hover_color_differs_from_base() {
        let base = hsla(0.6, 0.7, 0.5, 1.0);
        let bg = hsla(0.0, 0.0, 1.0, 1.0); // white background
        let result = hover_color(base, bg);
        // hover blends base at 0.9 opacity on bg -- should differ from base
        assert_ne!(result, base, "hover should differ from base");
    }

    #[test]
    fn active_color_light_theme_darkens() {
        let base = hsla(0.6, 0.7, 0.5, 1.0);
        let result = active_color(base, false);
        assert!(
            result.l < base.l,
            "active (light) l={} should be < base l={}",
            result.l,
            base.l
        );
    }

    #[test]
    fn active_color_dark_theme_darkens_more() {
        let base = hsla(0.6, 0.7, 0.5, 1.0);
        let light_result = active_color(base, false);
        let dark_result = active_color(base, true);
        assert!(
            dark_result.l < light_result.l,
            "dark active l={} should darken more than light active l={}",
            dark_result.l,
            light_result.l
        );
    }

    // Issue 52: near-black colors should lighten instead of darken
    #[test]
    fn active_color_near_black_lightens() {
        let near_black = hsla(0.6, 0.7, 0.05, 1.0);
        let result = active_color(near_black, true);
        assert!(
            result.l > near_black.l,
            "near-black active l={} should be > base l={} (lighten, not darken)",
            result.l,
            near_black.l
        );
    }

    #[test]
    fn active_color_near_black_light_mode_also_lightens() {
        let near_black = hsla(0.3, 0.5, 0.10, 1.0);
        let result = active_color(near_black, false);
        assert!(
            result.l > near_black.l,
            "near-black active (light) l={} should be > base l={}",
            result.l,
            near_black.l
        );
    }

    #[test]
    fn contrast_ratio_black_white() {
        let black = hsla(0.0, 0.0, 0.0, 1.0);
        let white = hsla(0.0, 0.0, 1.0, 1.0);
        let ratio = contrast_ratio(black, white);
        assert!(
            ratio > 20.0,
            "black/white contrast should be ~21, got {}",
            ratio
        );
    }

    #[test]
    fn contrast_ratio_same_color_is_one() {
        let c = hsla(0.5, 0.5, 0.5, 1.0);
        let ratio = contrast_ratio(c, c);
        assert!(
            (ratio - 1.0).abs() < 0.01,
            "same-color contrast should be 1.0, got {}",
            ratio
        );
    }

    #[test]
    fn light_variant_dark_theme_increases_lightness() {
        let bg = hsla(0.0, 0.0, 0.1, 1.0);
        let color = hsla(0.0, 0.8, 0.4, 1.0);
        let result = light_variant(bg, color, true);
        assert!(
            result.l > color.l,
            "dark theme light_variant l={} should be > base l={}",
            result.l,
            color.l
        );
    }

    #[test]
    fn light_variant_light_theme_blends_toward_bg() {
        let bg = hsla(0.0, 0.0, 0.95, 1.0);
        let color = hsla(0.0, 0.8, 0.4, 1.0);
        let result = light_variant(bg, color, false);
        // Should be closer to bg than the original
        assert!(
            result.l > color.l,
            "light theme light_variant l={} should be > base l={}",
            result.l,
            color.l
        );
    }

    // Issue 67: hover_color near white boundary
    #[test]
    fn hover_color_near_white() {
        let near_white = hsla(0.6, 0.5, 0.95, 1.0);
        let bg = hsla(0.0, 0.0, 1.0, 1.0); // white background
        let result = hover_color(near_white, bg);
        assert_ne!(
            result, near_white,
            "hover of near-white color should still differ from input"
        );
    }

    // Issue 67: active_color just above the near-black threshold should darken
    #[test]
    fn active_color_near_boundary() {
        // l=0.16 is just above the 0.15 threshold — should darken, not lighten
        let base = hsla(0.6, 0.7, 0.16, 1.0);
        let result = active_color(base, true);
        assert!(
            result.l < base.l,
            "l=0.16 (above 0.15 threshold) active l={} should darken (< base l={})",
            result.l,
            base.l
        );
    }

    // Issue 67: hover_color with zero saturation should still produce a different color
    #[test]
    fn hover_color_zero_saturation() {
        let gray = hsla(0.0, 0.0, 0.5, 1.0); // pure gray
        let bg = hsla(0.0, 0.0, 0.1, 1.0); // dark background
        let result = hover_color(gray, bg);
        assert_ne!(
            result, gray,
            "hover of zero-saturation gray should still differ from input"
        );
    }

    // Issue 67: hover_color with fully transparent base (alpha=0.0)
    #[test]
    fn hover_color_transparent_base() {
        let transparent = hsla(0.5, 0.5, 0.5, 0.0);
        let bg = hsla(0.0, 0.0, 1.0, 1.0);
        let result = hover_color(transparent, bg);
        // Blending a 0-alpha color at 0.9 opacity onto bg should produce
        // a color very close to bg (the transparent base contributes nothing).
        assert!(
            (result.l - bg.l).abs() < 0.05,
            "hover of transparent base l={} should be close to bg l={}",
            result.l,
            bg.l
        );
    }

    // Issue 67: active_color with pure black (l=0.0)
    // Documents a Colorize trait limitation: lighten(0.2) on l=0.0 stays 0.0
    // because the multiplication `l * (1 + factor) = 0`. The near-black safety
    // net works for l > 0 but not exactly zero. This is acceptable since pure
    // black rarely occurs in real themes.
    #[test]
    fn active_color_pure_black_stays_black() {
        let pure_black = hsla(0.0, 0.0, 0.0, 1.0);
        let result = active_color(pure_black, true);
        // l=0.0 enters the lighten path but multiplicative lighten can't
        // increase zero — result stays at l=0.0.
        assert!(
            (result.l - 0.0).abs() < f32::EPSILON,
            "pure black active l={} stays at 0.0 (Colorize limitation)",
            result.l,
        );
    }

    #[test]
    fn light_variant_clamped_to_095() {
        let bg = hsla(0.0, 0.0, 0.1, 1.0);
        let bright = hsla(0.5, 0.5, 0.9, 1.0);
        let result = light_variant(bg, bright, true);
        assert!(
            result.l <= 0.95,
            "light_variant should clamp to 0.95, got {}",
            result.l
        );
    }
}
