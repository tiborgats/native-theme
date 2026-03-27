//! gpui toolkit connector for native-theme.
//!
//! Maps [`native_theme::ResolvedTheme`] data to gpui-component's theming system.
//!
//! # Overview
//!
//! This crate provides a thin mapping layer that converts native-theme's
//! platform-agnostic color, font, and geometry data into gpui-component's
//! `Theme` type. No intermediate types are introduced -- the mapping goes
//! directly from `ResolvedTheme` fields to gpui-component types.
//!
//! # Usage
//!
//! ```ignore
//! use native_theme::NativeTheme;
//! use native_theme_gpui::to_theme;
//!
//! let nt = NativeTheme::preset("catppuccin-mocha").unwrap();
//! let mut variant = nt.pick_variant(false).unwrap().clone();
//! variant.resolve();
//! let resolved = variant.validate().unwrap();
//! let theme = to_theme(&resolved, "Catppuccin Mocha", false);
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod colors;
pub mod config;
pub mod derive;
pub mod icons;

use gpui_component::theme::{Theme, ThemeMode};
use native_theme::{NativeTheme, ResolvedTheme, ThemeVariant};

/// Pick a theme variant based on the requested mode.
///
/// If `is_dark` is true, returns the dark variant (falling back to light).
/// If `is_dark` is false, returns the light variant (falling back to dark).
#[deprecated(since = "0.3.2", note = "Use NativeTheme::pick_variant() instead")]
#[allow(deprecated)]
pub fn pick_variant(theme: &NativeTheme, is_dark: bool) -> Option<&ThemeVariant> {
    theme.pick_variant(is_dark)
}

/// Convert a [`ResolvedTheme`] into a gpui-component [`Theme`].
///
/// Builds a complete Theme by:
/// 1. Mapping all 108 ThemeColor fields via [`colors::to_theme_color`]
/// 2. Building a ThemeConfig from fonts/geometry via [`config::to_theme_config`]
/// 3. Constructing the Theme from the ThemeColor and applying the config
pub fn to_theme(resolved: &ResolvedTheme, name: &str, is_dark: bool) -> Theme {
    let theme_color = colors::to_theme_color(resolved);
    let mode = if is_dark {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    };
    let theme_config = config::to_theme_config(resolved, name, mode);

    // gpui-component's `apply_config` sets non-color fields we need: font_family,
    // font_size, radius, shadow, mode, light_theme/dark_theme Rc, and highlight_theme.
    // However, `ThemeColor::apply_config` (called internally) overwrites ALL color
    // fields with defaults, since our ThemeConfig has no explicit color overrides.
    // We restore our carefully-mapped colors after. This is a known gpui-component
    // API limitation -- there is no way to apply only non-color config fields.
    let mut theme = Theme::from(&theme_color);
    theme.apply_config(&theme_config.into());
    theme.colors = theme_color;
    theme
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    fn test_resolved() -> ResolvedTheme {
        let nt = NativeTheme::preset("catppuccin-mocha").expect("preset must exist");
        let mut v = nt
            .pick_variant(false)
            .expect("preset must have light variant")
            .clone();
        v.resolve();
        v.validate().expect("resolved preset must validate")
    }

    #[test]
    fn pick_variant_light_first() {
        let nt = NativeTheme::preset("catppuccin-mocha").expect("preset must exist");
        let picked = pick_variant(&nt, false);
        assert!(picked.is_some());
    }

    #[test]
    fn pick_variant_dark_first() {
        let nt = NativeTheme::preset("catppuccin-mocha").expect("preset must exist");
        let picked = pick_variant(&nt, true);
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
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Test", false);

        // Theme should have the correct mode
        assert!(!theme.is_dark());
    }

    #[test]
    fn to_theme_dark_mode() {
        let nt = NativeTheme::preset("catppuccin-mocha").expect("preset must exist");
        let mut v = nt
            .pick_variant(true)
            .expect("preset must have dark variant")
            .clone();
        v.resolve();
        let resolved = v.validate().expect("resolved preset must validate");
        let theme = to_theme(&resolved, "DarkTest", true);

        assert!(theme.is_dark());
    }
}
