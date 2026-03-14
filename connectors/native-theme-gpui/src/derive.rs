//! Shade derivation helpers for hover/active states.
//!
//! Provides functions to derive interactive state colors (hover,
//! active/pressed) from base colors. Uses the [`Colorize`] trait from
//! gpui-component for lightness adjustments.

use gpui::Hsla;
use gpui_component::Colorize;

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
    base.darken(factor)
}

#[cfg(test)]
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

}
