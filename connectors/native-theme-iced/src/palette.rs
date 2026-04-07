//! Maps [`native_theme::ResolvedThemeVariant`] colors to an [`iced_core::theme::Palette`].
//!
//! The palette has 6 fields: background, text, primary, success, warning, danger.
//! Each is mapped directly from the corresponding resolved theme color -- no
//! fallbacks needed since all fields are guaranteed populated.
//!
//! Note: `to_color()` preserves the alpha channel. The 6 palette colors are
//! always fully opaque in resolved themes (`a = 1.0`). Other resolved fields
//! (e.g., `shadow`, `selection_inactive`) may carry meaningful alpha values;
//! use [`to_color()`] to convert them when needed.

use native_theme::Rgba;

/// Convert a [`native_theme::Rgba`] to [`iced_core::Color`].
///
/// Useful for power users who need to map arbitrary `ResolvedThemeVariant`
/// fields to iced colors beyond what [`to_palette()`] covers. The alpha
/// channel is preserved faithfully.
#[must_use]
pub fn to_color(rgba: Rgba) -> iced_core::Color {
    let [r, g, b, a] = rgba.to_f32_array();
    iced_core::Color { r, g, b, a }
}

/// Build an iced [`iced_core::theme::Palette`] from a [`native_theme::ResolvedThemeVariant`].
///
/// Maps the 6 palette fields directly from resolved defaults:
/// - `background` <- `resolved.defaults.background_color`
/// - `text` <- `resolved.defaults.text_color`
/// - `primary` <- `resolved.defaults.accent_color`
/// - `success` <- `resolved.defaults.success_color`
/// - `warning` <- `resolved.defaults.warning_color`
/// - `danger` <- `resolved.defaults.danger_color`
#[must_use]
pub fn to_palette(resolved: &native_theme::ResolvedThemeVariant) -> iced_core::theme::Palette {
    let d = &resolved.defaults;

    iced_core::theme::Palette {
        background: to_color(d.background_color),
        text: to_color(d.text_color),
        primary: to_color(d.accent_color),
        success: to_color(d.success_color),
        warning: to_color(d.warning_color),
        danger: to_color(d.danger_color),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::ThemeSpec;

    #[test]
    fn to_color_converts_rgba() {
        let result = to_color(Rgba::rgb(255, 0, 0));
        // 255/255.0 = 1.0 exactly in f32, so exact comparison is safe
        assert_eq!(
            result,
            iced_core::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            },
            "expected exact red, got {:?}",
            result
        );
    }

    #[test]
    fn to_color_preserves_alpha() {
        let result = to_color(Rgba::rgba(0, 0, 0, 128));
        let expected_alpha = 128.0 / 255.0;
        assert!(
            (result.a - expected_alpha).abs() < 1e-6,
            "alpha should be ~{expected_alpha}, got {}",
            result.a
        );
        assert!((result.r).abs() < 1e-6, "r should be ~0, got {}", result.r);
    }

    #[test]
    fn to_palette_maps_all_fields_from_resolved() {
        let resolved = ThemeSpec::preset("catppuccin-mocha")
            .unwrap()
            .into_variant(false)
            .unwrap()
            .into_resolved()
            .unwrap();

        let palette = to_palette(&resolved);

        // Exact match against resolved values
        assert_eq!(
            palette.background,
            to_color(resolved.defaults.background_color)
        );
        assert_eq!(palette.text, to_color(resolved.defaults.text_color));
        assert_eq!(palette.primary, to_color(resolved.defaults.accent_color));
        assert_eq!(palette.success, to_color(resolved.defaults.success_color));
        assert_eq!(palette.warning, to_color(resolved.defaults.warning_color));
        assert_eq!(palette.danger, to_color(resolved.defaults.danger_color));
    }

    #[test]
    fn to_palette_dark_variant_has_dark_background() {
        let resolved = ThemeSpec::preset("catppuccin-mocha")
            .unwrap()
            .into_variant(true)
            .unwrap()
            .into_resolved()
            .unwrap();

        let palette = to_palette(&resolved);

        // Dark background should be dark
        assert!(palette.background.r < 0.3, "dark background should be dark");
    }

    #[test]
    fn to_palette_multiple_presets() {
        for name in ["catppuccin-mocha", "dracula", "nord", "gruvbox"] {
            let resolved = ThemeSpec::preset(name)
                .unwrap()
                .into_variant(true)
                .unwrap()
                .into_resolved()
                .unwrap();
            let palette = to_palette(&resolved);
            // Exact match for all presets
            assert_eq!(
                palette.primary,
                to_color(resolved.defaults.accent_color),
                "{name}: primary should match accent"
            );
        }
    }
}
