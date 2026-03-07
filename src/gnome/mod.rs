//! GNOME portal reader: reads accent color, color scheme, and contrast
//! from the XDG Desktop Portal Settings interface via ashpd.
//!
//! Uses the bundled Adwaita preset as base, then overlays portal-provided
//! accent color, color scheme (light/dark), and contrast preference.

use ashpd::desktop::settings::{ColorScheme, Contrast};
use ashpd::desktop::Color;

/// Convert an ashpd portal Color to an Rgba, returning None if any
/// component is outside the [0.0, 1.0] range (per XDG spec: out-of-range
/// means "unset").
pub(crate) fn portal_color_to_rgba(color: &Color) -> Option<crate::Rgba> {
    let r = color.red();
    let g = color.green();
    let b = color.blue();

    // Per XDG spec: out-of-range means "accent color not set"
    if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
        return None;
    }

    Some(crate::Rgba::from_f32(r as f32, g as f32, b as f32, 1.0))
}

/// Apply a portal accent color across multiple semantic color roles.
fn apply_accent(variant: &mut crate::ThemeVariant, accent: &crate::Rgba) {
    variant.colors.core.accent = Some(*accent);
    variant.colors.interactive.selection = Some(*accent);
    variant.colors.interactive.focus_ring = Some(*accent);
    variant.colors.primary.background = Some(*accent);
}

/// Adjust theme variant for high contrast preference.
fn apply_high_contrast(variant: &mut crate::ThemeVariant) {
    variant.geometry.border_opacity = Some(1.0);
    variant.geometry.disabled_opacity = Some(0.7);
}

/// Build a NativeTheme from an Adwaita base, applying portal-provided
/// color scheme, accent color, and contrast settings.
///
/// This is the testable core -- no D-Bus required.
pub(crate) fn build_theme(
    base: crate::NativeTheme,
    scheme: ColorScheme,
    accent: Option<Color>,
    contrast: Contrast,
) -> crate::Result<crate::NativeTheme> {
    let is_dark = matches!(scheme, ColorScheme::PreferDark);

    // Pick the appropriate variant from the Adwaita base
    let mut variant = if is_dark {
        base.dark.unwrap_or_default()
    } else {
        base.light.unwrap_or_default()
    };

    // Apply accent color if available and in range
    if let Some(color) = accent {
        if let Some(rgba) = portal_color_to_rgba(&color) {
            apply_accent(&mut variant, &rgba);
        }
    }

    // Apply high contrast adjustments
    if matches!(contrast, Contrast::High) {
        apply_high_contrast(&mut variant);
    }

    // Build NativeTheme with only the selected variant populated
    let theme = if is_dark {
        crate::NativeTheme {
            name: "GNOME".to_string(),
            light: None,
            dark: Some(variant),
        }
    } else {
        crate::NativeTheme {
            name: "GNOME".to_string(),
            light: Some(variant),
            dark: None,
        }
    };

    Ok(theme)
}

/// Read the user's GNOME theme from the XDG Desktop Portal.
///
/// Returns a [`NativeTheme`](crate::NativeTheme) with the active variant
/// (light or dark) populated based on the portal's color scheme preference,
/// with accent color and contrast adjustments applied.
///
/// Falls back to Adwaita defaults if the portal is unavailable.
pub async fn from_gnome() -> crate::Result<crate::NativeTheme> {
    let base = crate::preset("adwaita").expect("adwaita preset must be bundled");
    build_theme(
        base,
        ColorScheme::NoPreference,
        None,
        Contrast::NoPreference,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // === portal_color_to_rgba tests ===

    #[test]
    fn portal_color_valid_converts_to_rgba() {
        let color = Color::new(0.2, 0.4, 0.6);
        let result = portal_color_to_rgba(&color);
        assert!(result.is_some());
        let rgba = result.unwrap();
        assert_eq!(rgba, crate::Rgba::from_f32(0.2, 0.4, 0.6, 1.0));
    }

    #[test]
    fn portal_color_out_of_range_high_returns_none() {
        let color = Color::new(1.5, 0.0, 0.0);
        assert!(portal_color_to_rgba(&color).is_none());
    }

    #[test]
    fn portal_color_out_of_range_negative_returns_none() {
        let color = Color::new(-0.1, 0.5, 0.5);
        assert!(portal_color_to_rgba(&color).is_none());
    }

    // === build_theme color scheme tests ===

    fn adwaita_base() -> crate::NativeTheme {
        crate::preset("adwaita").unwrap()
    }

    #[test]
    fn dark_scheme_produces_dark_variant_only() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::PreferDark,
            None,
            Contrast::NoPreference,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.dark.is_some(), "dark variant should be Some");
        assert!(theme.light.is_none(), "light variant should be None");
    }

    #[test]
    fn light_scheme_produces_light_variant_only() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::PreferLight,
            None,
            Contrast::NoPreference,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_none(), "dark variant should be None");
    }

    #[test]
    fn no_preference_defaults_to_light() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_none(), "dark variant should be None");
    }

    // === accent color tests ===

    #[test]
    fn valid_accent_propagates_to_four_fields() {
        let accent = Color::new(0.2, 0.4, 0.8);
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            Some(accent),
            Contrast::NoPreference,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);

        assert_eq!(variant.colors.core.accent, Some(expected));
        assert_eq!(variant.colors.interactive.selection, Some(expected));
        assert_eq!(variant.colors.interactive.focus_ring, Some(expected));
        assert_eq!(variant.colors.primary.background, Some(expected));
    }

    // === high contrast tests ===

    #[test]
    fn high_contrast_sets_border_and_disabled_opacity() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::High,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.geometry.border_opacity, Some(1.0));
        assert_eq!(variant.geometry.disabled_opacity, Some(0.7));
    }

    #[test]
    fn normal_contrast_preserves_adwaita_geometry() {
        let base = adwaita_base();
        let base_light = base.light.as_ref().unwrap().clone();

        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.geometry, base_light.geometry);
    }

    // === fallback test ===

    #[test]
    fn no_accent_no_preference_no_contrast_returns_adwaita_light() {
        let base = adwaita_base();
        let base_light = base.light.as_ref().unwrap().clone();

        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
        )
        .unwrap();

        assert_eq!(theme.name, "GNOME");
        let variant = theme.light.as_ref().expect("light variant");
        // Colors should match Adwaita light defaults exactly
        assert_eq!(variant.colors, base_light.colors);
        assert_eq!(variant.fonts, base_light.fonts);
        assert_eq!(variant.geometry, base_light.geometry);
        assert_eq!(variant.spacing, base_light.spacing);
    }
}
