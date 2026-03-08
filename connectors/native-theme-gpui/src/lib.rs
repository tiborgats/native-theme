//! gpui toolkit connector for native-theme.
//!
//! Maps [`native_theme::NativeTheme`] data to gpui-component's theming system.
//!
//! # Overview
//!
//! This crate provides a thin mapping layer that converts native-theme's
//! platform-agnostic color, font, and geometry data into gpui-component's
//! `Theme` type. No intermediate types are introduced -- the mapping goes
//! directly from `ThemeVariant` fields to gpui-component types.
//!
//! # Usage
//!
//! ```ignore
//! use native_theme::NativeTheme;
//! use native_theme_gpui::{pick_variant, to_theme};
//!
//! let nt = NativeTheme::preset("default").unwrap();
//! let variant = pick_variant(&nt, false).unwrap();
//! let theme = to_theme(variant, "Default");
//! ```

pub mod colors;
pub mod config;
pub mod derive;

use gpui_component::theme::{Theme, ThemeMode};
use native_theme::{NativeTheme, ThemeVariant};

/// Pick a theme variant based on the requested mode.
///
/// If `is_dark` is true, returns the dark variant (falling back to light).
/// If `is_dark` is false, returns the light variant (falling back to dark).
pub fn pick_variant(theme: &NativeTheme, is_dark: bool) -> Option<&ThemeVariant> {
    if is_dark {
        theme.dark.as_ref().or(theme.light.as_ref())
    } else {
        theme.light.as_ref().or(theme.dark.as_ref())
    }
}

/// Convert a [`ThemeVariant`] into a gpui-component [`Theme`].
///
/// Builds a complete Theme by:
/// 1. Mapping all 108 ThemeColor fields via [`colors::to_theme_color`]
/// 2. Building a ThemeConfig from fonts/geometry via [`config::to_theme_config`]
/// 3. Constructing the Theme from the ThemeColor and applying the config
pub fn to_theme(variant: &ThemeVariant, name: &str, is_dark: bool) -> Theme {
    let theme_color = colors::to_theme_color(variant);
    let mode = if is_dark { ThemeMode::Dark } else { ThemeMode::Light };
    let theme_config = config::to_theme_config(variant, name, mode);

    // Create a base Theme from the ThemeColor, then apply config overrides.
    // Note: apply_config sets fonts/radius/shadow/mode but also overwrites all
    // color fields with gpui-component defaults (since ThemeConfig.colors is empty).
    // We must restore our mapped colors afterwards.
    let mut theme = Theme::from(&theme_color);
    theme.apply_config(&theme_config.into());
    theme.colors = theme_color;
    theme
}

#[cfg(test)]
mod tests {
    use super::*;
    use native_theme::Rgba;

    #[test]
    fn pick_variant_light_first() {
        let mut theme = NativeTheme::new("Test");
        let mut light = ThemeVariant::default();
        light.colors.background = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);

        let picked = pick_variant(&theme, false);
        assert!(picked.is_some());
        assert_eq!(
            picked.unwrap().colors.background,
            Some(Rgba::rgb(255, 255, 255))
        );
    }

    #[test]
    fn pick_variant_dark_first() {
        let mut theme = NativeTheme::new("Test");
        let mut dark = ThemeVariant::default();
        dark.colors.background = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = pick_variant(&theme, true);
        assert!(picked.is_some());
        assert_eq!(
            picked.unwrap().colors.background,
            Some(Rgba::rgb(30, 30, 30))
        );
    }

    #[test]
    fn pick_variant_fallback() {
        let mut theme = NativeTheme::new("Test");
        let mut light = ThemeVariant::default();
        light.colors.background = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        // No dark variant -- requesting dark should fall back to light
        let picked = pick_variant(&theme, true);
        assert!(picked.is_some());
    }

    #[test]
    fn pick_variant_empty_returns_none() {
        let theme = NativeTheme::new("Empty");
        assert!(pick_variant(&theme, false).is_none());
        assert!(pick_variant(&theme, true).is_none());
    }

    #[test]
    fn to_theme_produces_valid_theme() {
        let mut variant = ThemeVariant::default();
        variant.colors.background = Some(Rgba::rgb(255, 255, 255));
        variant.colors.foreground = Some(Rgba::rgb(0, 0, 0));
        variant.colors.accent = Some(Rgba::rgb(0, 120, 215));
        variant.fonts.family = Some("Inter".into());
        variant.fonts.size = Some(14.0);
        variant.geometry.radius = Some(4.0);

        let theme = to_theme(&variant, "Test", false);

        // Theme should have the correct mode
        assert!(!theme.is_dark());
    }
}
