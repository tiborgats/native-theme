//! iced toolkit connector for native-theme.
//!
//! Maps [`native_theme::NativeTheme`] data to iced's theming system.
//!
//! # Overview
//!
//! This crate provides a thin mapping layer from `native_theme::ThemeVariant`
//! to `iced_core::theme::Theme`. The main entry point is [`to_theme()`], which
//! produces a valid iced `Theme` with correct colors for all built-in widget
//! styles via iced's Catalog system.
//!
//! Widget metrics (padding, border radius, scrollbar width) are exposed as
//! helper functions rather than through the Catalog, since iced applies these
//! on widget instances.
//!
//! # Example
//!
//! ```rust
//! use native_theme::NativeTheme;
//! use native_theme_iced::{pick_variant, to_theme};
//!
//! let nt = NativeTheme::preset("default").unwrap();
//! if let Some(variant) = pick_variant(&nt, false) {
//!     let theme = to_theme(variant, "My App");
//!     // Use `theme` as your iced application theme
//! }
//! ```

pub mod extended;
pub mod palette;

/// Select light or dark variant from a [`NativeTheme`], with cross-fallback.
///
/// When `is_dark` is true, prefers `theme.dark` and falls back to `theme.light`.
/// When `is_dark` is false, prefers `theme.light` and falls back to `theme.dark`.
///
/// Returns `None` only if the theme has no variants at all.
pub fn pick_variant(
    theme: &native_theme::NativeTheme,
    is_dark: bool,
) -> Option<&native_theme::ThemeVariant> {
    if is_dark {
        theme.dark.as_ref().or(theme.light.as_ref())
    } else {
        theme.light.as_ref().or(theme.dark.as_ref())
    }
}

/// Create an iced [`Theme`] from a [`ThemeVariant`].
///
/// Builds a custom theme using `Theme::custom_with_fn()`, which:
/// 1. Maps the 6 Palette fields from native-theme colors via [`palette::to_palette()`]
/// 2. Generates an Extended palette, then overrides secondary and background.weak
///    entries via [`extended::apply_overrides()`]
///
/// The resulting theme carries the mapped Palette and Extended palette. iced's
/// built-in Catalog trait implementations for all 8 core widgets (Button,
/// Container, TextInput, Scrollable, Checkbox, Slider, ProgressBar, Tooltip)
/// automatically derive their Style structs from this palette. No explicit
/// Catalog implementations are needed.
pub fn to_theme(
    variant: &native_theme::ThemeVariant,
    name: &str,
) -> iced_core::theme::Theme {
    let pal = palette::to_palette(variant);

    // Clone the variant reference data we need into the closure.
    // The closure only needs the colors for extended palette overrides.
    let colors = variant.colors.clone();

    iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);

        // Build a temporary ThemeVariant with just the colors for apply_overrides
        let mut tmp = native_theme::ThemeVariant::default();
        tmp.colors = colors;
        extended::apply_overrides(&mut ext, &tmp);

        ext
    })
}

/// Returns button padding as `[horizontal, vertical]` from widget metrics.
///
/// Returns `None` if the variant has no widget metrics or if both padding
/// fields are `None`.
pub fn button_padding(variant: &native_theme::ThemeVariant) -> Option<[f32; 2]> {
    let bm = &variant.widget_metrics.as_ref()?.button;
    let h = bm.padding_horizontal?;
    let v = bm.padding_vertical.unwrap_or(h * 0.5);
    Some([h, v])
}

/// Returns text input padding as `[horizontal, vertical]` from widget metrics.
///
/// Returns `None` if the variant has no widget metrics or if the horizontal
/// padding field is `None`.
pub fn input_padding(variant: &native_theme::ThemeVariant) -> Option<[f32; 2]> {
    let im = &variant.widget_metrics.as_ref()?.input;
    let h = im.padding_horizontal?;
    let v = im.padding_vertical.unwrap_or(h * 0.5);
    Some([h, v])
}

/// Returns the standard border radius from geometry, defaulting to 4.0.
pub fn border_radius(variant: &native_theme::ThemeVariant) -> f32 {
    variant.geometry.radius.unwrap_or(4.0)
}

/// Returns the large border radius from geometry, defaulting to 8.0.
pub fn border_radius_lg(variant: &native_theme::ThemeVariant) -> f32 {
    variant.geometry.radius_lg.unwrap_or(8.0)
}

/// Returns the scrollbar width, checking geometry first, then widget metrics.
///
/// Falls back to 10.0 if neither source provides a value.
pub fn scrollbar_width(variant: &native_theme::ThemeVariant) -> f32 {
    // Prefer geometry.scroll_width, then widget_metrics.scrollbar.width
    variant
        .geometry
        .scroll_width
        .or_else(|| {
            variant
                .widget_metrics
                .as_ref()
                .and_then(|wm| wm.scrollbar.width)
        })
        .unwrap_or(10.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use native_theme::{NativeTheme, Rgba, ThemeVariant};

    // === pick_variant tests ===

    #[test]
    fn pick_variant_light_preferred_returns_light() {
        let mut theme = NativeTheme::new("Test");
        theme.light = Some(ThemeVariant::default());
        theme.dark = Some(ThemeVariant::default());

        let result = pick_variant(&theme, false);
        assert!(result.is_some());
        // Should return the light variant (which is the same as dark here,
        // but logically we check it's the light ref)
        assert!(std::ptr::eq(
            result.unwrap(),
            theme.light.as_ref().unwrap()
        ));
    }

    #[test]
    fn pick_variant_dark_preferred_returns_dark() {
        let mut theme = NativeTheme::new("Test");
        theme.light = Some(ThemeVariant::default());
        theme.dark = Some(ThemeVariant::default());

        let result = pick_variant(&theme, true);
        assert!(result.is_some());
        assert!(std::ptr::eq(
            result.unwrap(),
            theme.dark.as_ref().unwrap()
        ));
    }

    #[test]
    fn pick_variant_falls_back_to_light_when_no_dark() {
        let mut theme = NativeTheme::new("Test");
        theme.light = Some(ThemeVariant::default());
        // dark is None

        let result = pick_variant(&theme, true);
        assert!(result.is_some());
        assert!(std::ptr::eq(
            result.unwrap(),
            theme.light.as_ref().unwrap()
        ));
    }

    #[test]
    fn pick_variant_falls_back_to_dark_when_no_light() {
        let mut theme = NativeTheme::new("Test");
        // light is None
        theme.dark = Some(ThemeVariant::default());

        let result = pick_variant(&theme, false);
        assert!(result.is_some());
        assert!(std::ptr::eq(
            result.unwrap(),
            theme.dark.as_ref().unwrap()
        ));
    }

    #[test]
    fn pick_variant_returns_none_when_empty() {
        let theme = NativeTheme::new("Test");
        assert!(pick_variant(&theme, false).is_none());
        assert!(pick_variant(&theme, true).is_none());
    }

    // === to_theme tests ===

    #[test]
    fn to_theme_produces_non_default_theme() {
        let mut variant = ThemeVariant::default();
        variant.colors.accent = Some(Rgba::rgb(0, 120, 215));
        variant.colors.background = Some(Rgba::rgb(30, 30, 30));
        variant.colors.foreground = Some(Rgba::rgb(220, 220, 220));

        let theme = to_theme(&variant, "Test Theme");

        // The theme should not be equal to Light or Dark builtins
        assert_ne!(theme, iced_core::theme::Theme::Light);
        assert_ne!(theme, iced_core::theme::Theme::Dark);

        // Verify the palette was applied
        let palette = theme.palette();
        assert!(
            (palette.primary.r - 0.0).abs() < 0.01,
            "primary.r should be ~0.0, got {}",
            palette.primary.r
        );
    }

    #[test]
    fn to_theme_from_preset() {
        let nt = NativeTheme::preset("default").unwrap();
        let variant = pick_variant(&nt, false).unwrap();
        let theme = to_theme(variant, "Default");

        // Should be a valid custom theme
        let palette = theme.palette();
        // Default preset has white-ish background for light
        assert!(palette.background.r > 0.9);
    }

    // === Widget metric helper tests ===

    #[test]
    fn border_radius_returns_geometry_value() {
        let mut variant = ThemeVariant::default();
        variant.geometry.radius = Some(6.0);

        assert_eq!(border_radius(&variant), 6.0);
    }

    #[test]
    fn border_radius_returns_default_when_none() {
        let variant = ThemeVariant::default();
        assert_eq!(border_radius(&variant), 4.0);
    }

    #[test]
    fn border_radius_lg_returns_geometry_value() {
        let mut variant = ThemeVariant::default();
        variant.geometry.radius_lg = Some(12.0);

        assert_eq!(border_radius_lg(&variant), 12.0);
    }

    #[test]
    fn border_radius_lg_returns_default_when_none() {
        let variant = ThemeVariant::default();
        assert_eq!(border_radius_lg(&variant), 8.0);
    }

    #[test]
    fn scrollbar_width_prefers_geometry() {
        let mut variant = ThemeVariant::default();
        variant.geometry.scroll_width = Some(14.0);

        assert_eq!(scrollbar_width(&variant), 14.0);
    }

    #[test]
    fn scrollbar_width_falls_back_to_widget_metrics() {
        let mut variant = ThemeVariant::default();
        let mut wm = native_theme::WidgetMetrics::default();
        wm.scrollbar.width = Some(12.0);
        variant.widget_metrics = Some(wm);

        assert_eq!(scrollbar_width(&variant), 12.0);
    }

    #[test]
    fn scrollbar_width_returns_default_when_none() {
        let variant = ThemeVariant::default();
        assert_eq!(scrollbar_width(&variant), 10.0);
    }

    #[test]
    fn button_padding_returns_values_from_metrics() {
        let mut variant = ThemeVariant::default();
        let mut wm = native_theme::WidgetMetrics::default();
        wm.button.padding_horizontal = Some(12.0);
        wm.button.padding_vertical = Some(6.0);
        variant.widget_metrics = Some(wm);

        let result = button_padding(&variant).unwrap();
        assert_eq!(result, [12.0, 6.0]);
    }

    #[test]
    fn button_padding_returns_none_without_metrics() {
        let variant = ThemeVariant::default();
        assert!(button_padding(&variant).is_none());
    }

    #[test]
    fn input_padding_returns_values_from_metrics() {
        let mut variant = ThemeVariant::default();
        let mut wm = native_theme::WidgetMetrics::default();
        wm.input.padding_horizontal = Some(8.0);
        wm.input.padding_vertical = Some(4.0);
        variant.widget_metrics = Some(wm);

        let result = input_padding(&variant).unwrap();
        assert_eq!(result, [8.0, 4.0]);
    }

    #[test]
    fn input_padding_returns_none_without_metrics() {
        let variant = ThemeVariant::default();
        assert!(input_padding(&variant).is_none());
    }
}
