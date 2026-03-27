//! iced toolkit connector for native-theme.
//!
//! Maps [`native_theme::ResolvedTheme`] data to iced's theming system.
//!
//! # Overview
//!
//! This crate provides a thin mapping layer from `native_theme::ResolvedTheme`
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
//! use native_theme_iced::to_theme;
//!
//! let nt = NativeTheme::preset("catppuccin-mocha").unwrap();
//! let mut variant = nt.pick_variant(false).unwrap().clone();
//! variant.resolve();
//! let resolved = variant.validate().unwrap();
//! let theme = to_theme(&resolved, "My App");
//! // Use `theme` as your iced application theme
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod extended;
pub mod icons;
pub mod palette;

/// Select light or dark variant from a [`native_theme::NativeTheme`], with cross-fallback.
///
/// When `is_dark` is true, prefers `theme.dark` and falls back to `theme.light`.
/// When `is_dark` is false, prefers `theme.light` and falls back to `theme.dark`.
///
/// Returns `None` only if the theme has no variants at all.
#[deprecated(since = "0.3.2", note = "Use NativeTheme::pick_variant() instead")]
#[allow(deprecated)]
pub fn pick_variant(
    theme: &native_theme::NativeTheme,
    is_dark: bool,
) -> Option<&native_theme::ThemeVariant> {
    theme.pick_variant(is_dark)
}

/// Create an iced [`iced_core::theme::Theme`] from a [`native_theme::ResolvedTheme`].
///
/// Builds a custom theme using `Theme::custom_with_fn()`, which:
/// 1. Maps the 6 Palette fields from resolved theme colors via [`palette::to_palette()`]
/// 2. Generates an Extended palette, then overrides secondary and background.weak
///    entries via [`extended::apply_overrides()`]
///
/// The resulting theme carries the mapped Palette and Extended palette. iced's
/// built-in Catalog trait implementations for all 8 core widgets (Button,
/// Container, TextInput, Scrollable, Checkbox, Slider, ProgressBar, Tooltip)
/// automatically derive their Style structs from this palette. No explicit
/// Catalog implementations are needed.
pub fn to_theme(resolved: &native_theme::ResolvedTheme, name: &str) -> iced_core::theme::Theme {
    let pal = palette::to_palette(resolved);

    // Clone the resolved fields we need into the closure.
    let button_bg = resolved.button.background;
    let button_fg = resolved.button.foreground;
    let surface = resolved.defaults.surface;
    let foreground = resolved.defaults.foreground;

    iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);

        ext.secondary.base.color = palette::to_color(button_bg);
        ext.secondary.base.text = palette::to_color(button_fg);
        ext.background.weak.color = palette::to_color(surface);
        ext.background.weak.text = palette::to_color(foreground);

        ext
    })
}

/// Returns button padding as `[horizontal, vertical]` from the resolved theme.
pub fn button_padding(resolved: &native_theme::ResolvedTheme) -> [f32; 2] {
    [resolved.button.padding_horizontal, resolved.button.padding_vertical]
}

/// Returns text input padding as `[horizontal, vertical]` from the resolved theme.
pub fn input_padding(resolved: &native_theme::ResolvedTheme) -> [f32; 2] {
    [resolved.input.padding_horizontal, resolved.input.padding_vertical]
}

/// Returns the standard border radius from the resolved theme.
pub fn border_radius(resolved: &native_theme::ResolvedTheme) -> f32 {
    resolved.defaults.radius
}

/// Returns the large border radius from the resolved theme.
pub fn border_radius_lg(resolved: &native_theme::ResolvedTheme) -> f32 {
    resolved.defaults.radius_lg
}

/// Returns the scrollbar width from the resolved theme.
pub fn scrollbar_width(resolved: &native_theme::ResolvedTheme) -> f32 {
    resolved.scrollbar.width
}

/// Returns the primary UI font family name from the resolved theme.
pub fn font_family(resolved: &native_theme::ResolvedTheme) -> &str {
    &resolved.defaults.font.family
}

/// Returns the primary UI font size in logical pixels from the resolved theme.
///
/// ResolvedFontSpec.size is already in logical pixels -- no pt-to-px conversion
/// is applied.
pub fn font_size(resolved: &native_theme::ResolvedTheme) -> f32 {
    resolved.defaults.font.size
}

/// Returns the monospace font family name from the resolved theme.
pub fn mono_font_family(resolved: &native_theme::ResolvedTheme) -> &str {
    &resolved.defaults.mono_font.family
}

/// Returns the monospace font size in logical pixels from the resolved theme.
///
/// ResolvedFontSpec.size is already in logical pixels -- no pt-to-px conversion
/// is applied.
pub fn mono_font_size(resolved: &native_theme::ResolvedTheme) -> f32 {
    resolved.defaults.mono_font.size
}

#[cfg(test)]
#[allow(deprecated)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::{NativeTheme, ThemeVariant};

    fn make_resolved(is_dark: bool) -> native_theme::ResolvedTheme {
        let nt = NativeTheme::preset("catppuccin-mocha").unwrap();
        let mut variant = nt.pick_variant(is_dark).unwrap().clone();
        variant.resolve();
        variant.validate().unwrap()
    }

    // === pick_variant tests ===

    #[test]
    fn pick_variant_light_preferred_returns_light() {
        let mut theme = NativeTheme::new("Test");
        theme.light = Some(ThemeVariant::default());
        theme.dark = Some(ThemeVariant::default());

        let result = pick_variant(&theme, false);
        assert!(result.is_some());
        assert!(std::ptr::eq(result.unwrap(), theme.light.as_ref().unwrap()));
    }

    #[test]
    fn pick_variant_dark_preferred_returns_dark() {
        let mut theme = NativeTheme::new("Test");
        theme.light = Some(ThemeVariant::default());
        theme.dark = Some(ThemeVariant::default());

        let result = pick_variant(&theme, true);
        assert!(result.is_some());
        assert!(std::ptr::eq(result.unwrap(), theme.dark.as_ref().unwrap()));
    }

    #[test]
    fn pick_variant_falls_back_to_light_when_no_dark() {
        let mut theme = NativeTheme::new("Test");
        theme.light = Some(ThemeVariant::default());

        let result = pick_variant(&theme, true);
        assert!(result.is_some());
        assert!(std::ptr::eq(result.unwrap(), theme.light.as_ref().unwrap()));
    }

    #[test]
    fn pick_variant_falls_back_to_dark_when_no_light() {
        let mut theme = NativeTheme::new("Test");
        theme.dark = Some(ThemeVariant::default());

        let result = pick_variant(&theme, false);
        assert!(result.is_some());
        assert!(std::ptr::eq(result.unwrap(), theme.dark.as_ref().unwrap()));
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
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "Test Theme");

        assert_ne!(theme, iced_core::theme::Theme::Light);
        assert_ne!(theme, iced_core::theme::Theme::Dark);

        let palette = theme.palette();
        // Verify palette was applied from resolved theme
        assert!(
            palette.primary.r > 0.0 || palette.primary.g > 0.0 || palette.primary.b > 0.0,
            "primary should be non-zero"
        );
    }

    #[test]
    fn to_theme_from_preset() {
        let resolved = make_resolved(false);
        let theme = to_theme(&resolved, "Default");

        let palette = theme.palette();
        // Default preset has white-ish background for light
        assert!(palette.background.r > 0.9);
    }

    // === Widget metric helper tests ===

    #[test]
    fn border_radius_returns_resolved_value() {
        let resolved = make_resolved(false);
        let r = border_radius(&resolved);
        assert!(r > 0.0, "resolved radius should be > 0");
    }

    #[test]
    fn border_radius_lg_returns_resolved_value() {
        let resolved = make_resolved(false);
        let r = border_radius_lg(&resolved);
        assert!(r > 0.0, "resolved radius_lg should be > 0");
        assert!(r >= border_radius(&resolved), "radius_lg should be >= radius");
    }

    #[test]
    fn scrollbar_width_returns_resolved_value() {
        let resolved = make_resolved(false);
        let w = scrollbar_width(&resolved);
        assert!(w > 0.0, "scrollbar width should be > 0");
    }

    #[test]
    fn button_padding_returns_resolved_values() {
        let resolved = make_resolved(false);
        let [h, v] = button_padding(&resolved);
        assert!(h > 0.0, "button horizontal padding should be > 0");
        assert!(v > 0.0, "button vertical padding should be > 0");
    }

    #[test]
    fn input_padding_returns_resolved_values() {
        let resolved = make_resolved(false);
        let [h, v] = input_padding(&resolved);
        assert!(h > 0.0, "input horizontal padding should be > 0");
        assert!(v > 0.0, "input vertical padding should be > 0");
    }

    // === Font helper tests ===

    #[test]
    fn font_family_returns_concrete_value() {
        let resolved = make_resolved(false);
        let ff = font_family(&resolved);
        assert!(!ff.is_empty(), "font family should not be empty");
    }

    #[test]
    fn font_size_returns_concrete_value() {
        let resolved = make_resolved(false);
        let fs = font_size(&resolved);
        assert!(fs > 0.0, "font size should be > 0");
    }

    #[test]
    fn mono_font_family_returns_concrete_value() {
        let resolved = make_resolved(false);
        let mf = mono_font_family(&resolved);
        assert!(!mf.is_empty(), "mono font family should not be empty");
    }

    #[test]
    fn mono_font_size_returns_concrete_value() {
        let resolved = make_resolved(false);
        let ms = mono_font_size(&resolved);
        assert!(ms > 0.0, "mono font size should be > 0");
    }
}
