//! Maps [`native_theme::ThemeVariant`] colors to an [`iced_core::theme::Palette`].
//!
//! The palette has 6 fields: background, text, primary, success, warning, danger.
//! Each is mapped from the corresponding native-theme semantic color, falling
//! back to a sensible default when the field is `None`.

use native_theme::Rgba;

/// Convert an `Option<Rgba>` to `iced_core::Color`, falling back to `default` when `None`.
pub(crate) fn to_color(rgba: Option<Rgba>, default: iced_core::Color) -> iced_core::Color {
    match rgba {
        Some(c) => {
            let [r, g, b, a] = c.to_f32_array();
            iced_core::Color { r, g, b, a }
        }
        None => default,
    }
}

/// Build an iced [`iced_core::theme::Palette`] from a [`native_theme::ThemeVariant`].
///
/// Maps the 6 palette fields:
/// - `background` <- `colors.background` (fallback: white)
/// - `text` <- `colors.foreground` (fallback: black)
/// - `primary` <- `colors.accent` (fallback: 0x0078d7)
/// - `success` <- `colors.success` (fallback: 0x107c10)
/// - `warning` <- `colors.warning` (fallback: 0xff8c00)
/// - `danger` <- `colors.danger` (fallback: 0xd13438)
pub fn to_palette(variant: &native_theme::ThemeVariant) -> iced_core::theme::Palette {
    let c = &variant.colors;

    iced_core::theme::Palette {
        background: to_color(c.background, iced_core::Color::WHITE),
        text: to_color(c.foreground, iced_core::Color::BLACK),
        primary: to_color(c.accent, iced_core::Color::from_rgb(0.0, 0.47, 0.84)),
        success: to_color(c.success, iced_core::Color::from_rgb(0.063, 0.486, 0.063)),
        warning: to_color(c.warning, iced_core::Color::from_rgb(1.0, 0.549, 0.0)),
        danger: to_color(c.danger, iced_core::Color::from_rgb(0.82, 0.204, 0.22)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use native_theme::{Rgba, ThemeVariant};

    // Helper to compare iced_core::Color fields with tolerance
    fn color_approx_eq(a: iced_core::Color, b: iced_core::Color) -> bool {
        (a.r - b.r).abs() < 0.01
            && (a.g - b.g).abs() < 0.01
            && (a.b - b.b).abs() < 0.01
            && (a.a - b.a).abs() < 0.01
    }

    #[test]
    fn to_color_some_converts_rgba() {
        let result = to_color(Some(Rgba::rgb(255, 0, 0)), iced_core::Color::WHITE);
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
    fn to_color_none_returns_default() {
        let result = to_color(None, iced_core::Color::WHITE);
        assert_eq!(result, iced_core::Color::WHITE);
    }

    #[test]
    fn to_palette_full_variant_maps_all_fields() {
        let mut variant = ThemeVariant::default();
        variant.colors.background = Some(Rgba::rgb(30, 30, 30));
        variant.colors.foreground = Some(Rgba::rgb(220, 220, 220));
        variant.colors.accent = Some(Rgba::rgb(0, 120, 215));
        variant.colors.success = Some(Rgba::rgb(16, 124, 16));
        variant.colors.warning = Some(Rgba::rgb(255, 140, 0));
        variant.colors.danger = Some(Rgba::rgb(209, 52, 56));

        let palette = to_palette(&variant);

        assert!(color_approx_eq(
            palette.background,
            iced_core::Color::from_rgba(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0, 1.0)
        ));
        assert!(color_approx_eq(
            palette.text,
            iced_core::Color::from_rgba(220.0 / 255.0, 220.0 / 255.0, 220.0 / 255.0, 1.0)
        ));
        assert!(color_approx_eq(
            palette.primary,
            iced_core::Color::from_rgba(0.0, 120.0 / 255.0, 215.0 / 255.0, 1.0)
        ));
        assert!(color_approx_eq(
            palette.success,
            iced_core::Color::from_rgba(16.0 / 255.0, 124.0 / 255.0, 16.0 / 255.0, 1.0)
        ));
        assert!(color_approx_eq(
            palette.warning,
            iced_core::Color::from_rgba(255.0 / 255.0, 140.0 / 255.0, 0.0, 1.0)
        ));
        assert!(color_approx_eq(
            palette.danger,
            iced_core::Color::from_rgba(209.0 / 255.0, 52.0 / 255.0, 56.0 / 255.0, 1.0)
        ));
    }

    #[test]
    fn to_palette_empty_variant_returns_defaults() {
        let variant = ThemeVariant::default();
        let palette = to_palette(&variant);

        // Fallback defaults
        assert_eq!(palette.background, iced_core::Color::WHITE);
        assert_eq!(palette.text, iced_core::Color::BLACK);
        // Primary fallback: 0x0078d7
        assert!(color_approx_eq(
            palette.primary,
            iced_core::Color::from_rgb(0.0, 0.47, 0.84)
        ));
        // Success fallback: 0x107c10
        assert!(color_approx_eq(
            palette.success,
            iced_core::Color::from_rgb(0.063, 0.486, 0.063)
        ));
        // Warning fallback: 0xff8c00
        assert!(color_approx_eq(
            palette.warning,
            iced_core::Color::from_rgb(1.0, 0.549, 0.0)
        ));
        // Danger fallback: 0xd13438
        assert!(color_approx_eq(
            palette.danger,
            iced_core::Color::from_rgb(0.82, 0.204, 0.22)
        ));
    }
}
