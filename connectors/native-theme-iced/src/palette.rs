//! Maps [`native_theme::ResolvedThemeVariant`] colors to an [`iced_core::theme::Palette`].
//!
//! The palette has 6 fields: background, text, primary, success, warning, danger.
//! Each is mapped directly from the corresponding resolved theme color -- no
//! fallbacks needed since all fields are guaranteed populated.

use native_theme::Rgba;

/// Convert an `Rgba` to `iced_core::Color`.
pub(crate) fn to_color(rgba: Rgba) -> iced_core::Color {
    let [r, g, b, a] = rgba.to_f32_array();
    iced_core::Color { r, g, b, a }
}

/// Build an iced [`iced_core::theme::Palette`] from a [`native_theme::ResolvedThemeVariant`].
///
/// Maps the 6 palette fields directly from resolved defaults:
/// - `background` <- `resolved.defaults.background`
/// - `text` <- `resolved.defaults.foreground`
/// - `primary` <- `resolved.defaults.accent`
/// - `success` <- `resolved.defaults.success`
/// - `warning` <- `resolved.defaults.warning`
/// - `danger` <- `resolved.defaults.danger`
pub fn to_palette(resolved: &native_theme::ResolvedThemeVariant) -> iced_core::theme::Palette {
    let d = &resolved.defaults;

    iced_core::theme::Palette {
        background: to_color(d.background),
        text: to_color(d.foreground),
        primary: to_color(d.accent),
        success: to_color(d.success),
        warning: to_color(d.warning),
        danger: to_color(d.danger),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::ThemeSpec;

    // Helper to compare iced_core::Color fields with tolerance
    fn color_approx_eq(a: iced_core::Color, b: iced_core::Color) -> bool {
        (a.r - b.r).abs() < 0.01
            && (a.g - b.g).abs() < 0.01
            && (a.b - b.b).abs() < 0.01
            && (a.a - b.a).abs() < 0.01
    }

    #[test]
    fn to_color_converts_rgba() {
        let result = to_color(Rgba::rgb(255, 0, 0));
        assert!(
            color_approx_eq(
                result,
                iced_core::Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0
                }
            ),
            "expected red, got {:?}",
            result
        );
    }

    #[test]
    fn to_palette_maps_all_fields_from_resolved() {
        let nt = ThemeSpec::preset("catppuccin-mocha").unwrap();
        let mut variant = nt.pick_variant(false).unwrap().clone();
        variant.resolve();
        let resolved = variant.validate().unwrap();

        let palette = to_palette(&resolved);

        // All fields should be populated (non-zero)
        // Catppuccin Mocha light has a white-ish background
        assert!(palette.background.r > 0.9, "background should be light");
        // Text should be dark
        assert!(palette.text.r < 0.3, "text should be dark");
        // Primary should be non-zero (accent color)
        assert!(
            palette.primary.r > 0.0 || palette.primary.g > 0.0 || palette.primary.b > 0.0,
            "primary should be non-zero"
        );
    }

    #[test]
    fn to_palette_dark_variant_has_dark_background() {
        let nt = ThemeSpec::preset("catppuccin-mocha").unwrap();
        let mut variant = nt.pick_variant(true).unwrap().clone();
        variant.resolve();
        let resolved = variant.validate().unwrap();

        let palette = to_palette(&resolved);

        // Dark background should be dark
        assert!(palette.background.r < 0.3, "dark background should be dark");
    }
}
