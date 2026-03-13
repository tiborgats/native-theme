//! Shade derivation helpers for hover/active states.
//!
//! Provides lightness adjustment functions used to derive interactive
//! state colors (hover, active/pressed) from base colors. Follows the
//! same approach as gpui-component's internal derivation logic.

use gpui::Hsla;
use gpui_component::Colorize;

/// Lighten an Hsla color by the given factor (0.0 to 1.0).
///
/// Increases lightness multiplicatively: `l = l * (1 + factor)`.
/// Matches [`gpui_component::Colorize::lighten`].
pub fn lighten(color: Hsla, factor: f32) -> Hsla {
    color.lighten(factor)
}

/// Darken an Hsla color by the given factor (0.0 to 1.0).
///
/// Decreases lightness multiplicatively: `l = l * (1 - factor)`.
/// Matches [`gpui_component::Colorize::darken`].
pub fn darken(color: Hsla, factor: f32) -> Hsla {
    color.darken(factor)
}

/// Derive a hover state from a base color.
///
/// For light themes (is_dark=false): blends the background with base at 90% opacity.
/// For dark themes (is_dark=true): blends the background with base at 90% opacity.
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
/// This matches gpui-component's internal `apply_config` active derivation.
pub fn active_color(base: Hsla, is_dark: bool) -> Hsla {
    let factor = if is_dark { 0.2 } else { 0.1 };
    darken(base, factor)
}

/// Set the alpha component of a color.
pub fn with_alpha(color: Hsla, alpha: f32) -> Hsla {
    color.alpha(alpha)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::hsla;

    #[test]
    fn lighten_increases_lightness() {
        let color = hsla(0.5, 0.5, 0.4, 1.0);
        let result = lighten(color, 0.1);
        assert!(
            result.l > color.l,
            "lightened l={} should be > original l={}",
            result.l,
            color.l
        );
    }

    #[test]
    fn darken_decreases_lightness() {
        let color = hsla(0.5, 0.5, 0.6, 1.0);
        let result = darken(color, 0.1);
        assert!(
            result.l < color.l,
            "darkened l={} should be < original l={}",
            result.l,
            color.l
        );
    }

    #[test]
    fn lighten_preserves_hue_and_saturation() {
        let color = hsla(0.3, 0.7, 0.5, 1.0);
        let result = lighten(color, 0.2);
        assert_eq!(result.h, color.h);
        assert_eq!(result.s, color.s);
        assert_eq!(result.a, color.a);
    }

    #[test]
    fn darken_preserves_hue_and_saturation() {
        let color = hsla(0.3, 0.7, 0.5, 1.0);
        let result = darken(color, 0.2);
        assert_eq!(result.h, color.h);
        assert_eq!(result.s, color.s);
        assert_eq!(result.a, color.a);
    }

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

    #[test]
    fn with_alpha_sets_alpha() {
        let color = hsla(0.5, 0.5, 0.5, 1.0);
        let result = with_alpha(color, 0.3);
        assert!((result.a - 0.3).abs() < f32::EPSILON);
        assert_eq!(result.h, color.h);
        assert_eq!(result.s, color.s);
        assert_eq!(result.l, color.l);
    }
}
