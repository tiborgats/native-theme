//! gpui toolkit connector for native-theme.
//!
//! Maps [`native_theme::ResolvedThemeVariant`] data to gpui-component's theming system.
//!
//! # Quick Start
//!
//! ```ignore
//! use native_theme_gpui::from_preset;
//!
//! let theme = from_preset("catppuccin-mocha", true)?;
//! ```
//!
//! Or from the OS-detected theme:
//!
//! ```ignore
//! use native_theme_gpui::from_system;
//!
//! let theme = from_system()?;
//! ```
//!
//! # Manual Path
//!
//! For full control over the resolve/validate/convert pipeline:
//!
//! ```ignore
//! use native_theme::ThemeSpec;
//! use native_theme_gpui::to_theme;
//!
//! let nt = ThemeSpec::preset("catppuccin-mocha").unwrap();
//! let mut variant = nt.pick_variant(false).unwrap().clone();
//! variant.resolve();
//! let resolved = variant.validate().unwrap();
//! let theme = to_theme(&resolved, "Catppuccin Mocha", false);
//! ```
//!
//! # Theme Field Coverage
//!
//! The connector maps a subset of [`ResolvedThemeVariant`] fields to gpui-component's
//! `ThemeColor` (108 color fields) and `ThemeConfig` (font/geometry).
//!
//! | Category | Mapped | Notes |
//! |----------|--------|-------|
//! | `defaults` colors | All 20+ | background, foreground, accent, danger, etc. |
//! | `defaults` geometry | radius, radius_lg, shadow | Font family/size also mapped |
//! | `button` | 4 of 14 | primary_bg/fg, background/foreground (colors only) |
//! | `tab` | 5 of 9 | All colors, sizing not mapped |
//! | `sidebar` | 2 of 2 | background, foreground |
//! | `window` | 2 of 10 | title_bar_background, border |
//! | `input` | 2 of 12 | border, caret |
//! | `scrollbar` | 2 of 7 | thumb, thumb_hover |
//! | `slider`, `switch` | 2 each | fill/thumb colors |
//! | `progress_bar` | 1 of 5 | fill |
//! | `list` | 1 of 11 | alternate_row |
//! | `popover` | 2 of 4 | background, foreground |
//! | 14 other widgets | 0 fields | checkbox, menu, tooltip, dialog, etc. |
//!
//! **Why the gap:** gpui-component's `ThemeColor` is a flat color bag with no per-widget
//! geometry. The connector cannot map most sizing/spacing data because the target type
//! has no corresponding fields. Users who need per-widget geometry can read it directly
//! from the `ResolvedThemeVariant` they passed to [`to_theme()`].

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub(crate) mod colors;
pub(crate) mod config;
pub(crate) mod derive;
pub mod icons;

// Re-export native-theme types that appear in public signatures so downstream
// crates don't need native-theme as a direct dependency.
pub use native_theme::{
    AnimatedIcon, IconData, IconProvider, IconRole, IconSet, ResolvedThemeVariant, SystemTheme,
    ThemeSpec, ThemeVariant,
};

use gpui_component::theme::{Theme, ThemeMode};

/// Pick a theme variant based on the requested mode.
///
/// If `is_dark` is true, returns the dark variant (falling back to light).
/// If `is_dark` is false, returns the light variant (falling back to dark).
#[deprecated(since = "0.3.2", note = "Use ThemeSpec::pick_variant() instead")]
#[allow(deprecated)]
pub fn pick_variant(theme: &ThemeSpec, is_dark: bool) -> Option<&ThemeVariant> {
    theme.pick_variant(is_dark)
}

/// Convert a [`ResolvedThemeVariant`] into a gpui-component [`Theme`].
///
/// Builds a complete Theme by:
/// 1. Mapping all 108 ThemeColor fields via [`colors::to_theme_color`]
/// 2. Building a ThemeConfig from fonts/geometry via [`config::to_theme_config`]
/// 3. Constructing the Theme from the ThemeColor and applying the config
pub fn to_theme(resolved: &ResolvedThemeVariant, name: &str, is_dark: bool) -> Theme {
    let theme_color = colors::to_theme_color(resolved, is_dark);
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

/// Load a bundled preset and convert it to a gpui-component [`Theme`] in one call.
///
/// This is the primary entry point for most users. It handles the full pipeline:
/// load preset, pick variant, resolve, validate, and convert to gpui Theme.
///
/// The preset name is used as the theme display name.
///
/// # Errors
///
/// Returns an error if the preset name is not recognized or if resolution fails.
///
/// # Examples
///
/// ```ignore
/// let dark_theme = native_theme_gpui::from_preset("dracula", true)?;
/// let light_theme = native_theme_gpui::from_preset("catppuccin-latte", false)?;
/// ```
pub fn from_preset(name: &str, is_dark: bool) -> native_theme::Result<Theme> {
    let spec = ThemeSpec::preset(name)?;
    let variant = spec
        .pick_variant(is_dark)
        .ok_or_else(|| native_theme::Error::Format(format!("preset '{name}' has no variants")))?;
    let resolved = variant.clone().into_resolved()?;
    Ok(to_theme(&resolved, name, is_dark))
}

/// Detect the OS theme and convert it to a gpui-component [`Theme`] in one call.
///
/// Combines [`SystemTheme::from_system()`](native_theme::SystemTheme::from_system)
/// with [`to_theme()`] using the system-detected name and dark-mode preference.
///
/// # Errors
///
/// Returns an error if the platform theme cannot be read (e.g., unsupported platform,
/// missing desktop environment).
///
/// # Examples
///
/// ```ignore
/// let theme = native_theme_gpui::from_system()?;
/// ```
pub fn from_system() -> native_theme::Result<Theme> {
    let sys = SystemTheme::from_system()?;
    Ok(to_theme(sys.active(), &sys.name, sys.is_dark))
}

/// Extension trait for converting a [`SystemTheme`] to a gpui-component [`Theme`].
///
/// Useful when you already have a `SystemTheme` and want method syntax:
///
/// ```ignore
/// use native_theme_gpui::SystemThemeExt;
///
/// let sys = native_theme::SystemTheme::from_system()?;
/// let theme = sys.to_gpui_theme();
/// ```
pub trait SystemThemeExt {
    /// Convert this system theme to a gpui-component [`Theme`].
    ///
    /// Uses the active variant (based on `is_dark`), the theme name,
    /// and the dark-mode flag from the `SystemTheme`.
    fn to_gpui_theme(&self) -> Theme;
}

impl SystemThemeExt for SystemTheme {
    fn to_gpui_theme(&self) -> Theme {
        to_theme(self.active(), &self.name, self.is_dark)
    }
}

#[cfg(test)]
#[allow(deprecated)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn test_resolved() -> ResolvedThemeVariant {
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let mut v = nt
            .pick_variant(false)
            .expect("preset must have light variant")
            .clone();
        v.resolve();
        v.validate().expect("resolved preset must validate")
    }

    #[test]
    fn pick_variant_light_first() {
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let picked = pick_variant(&nt, false);
        assert!(picked.is_some());
    }

    #[test]
    fn pick_variant_dark_first() {
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let picked = pick_variant(&nt, true);
        assert!(picked.is_some());
    }

    #[test]
    fn pick_variant_empty_returns_none() {
        let theme = ThemeSpec::new("Empty");
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
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let mut v = nt
            .pick_variant(true)
            .expect("preset must have dark variant")
            .clone();
        v.resolve();
        let resolved = v.validate().expect("resolved preset must validate");
        let theme = to_theme(&resolved, "DarkTest", true);

        assert!(theme.is_dark());
    }

    // -- from_preset tests --

    #[test]
    fn from_preset_valid_light() {
        let theme = from_preset("catppuccin-mocha", false).expect("preset should load");
        assert!(!theme.is_dark());
    }

    #[test]
    fn from_preset_valid_dark() {
        let theme = from_preset("catppuccin-mocha", true).expect("preset should load");
        assert!(theme.is_dark());
    }

    #[test]
    fn from_preset_invalid_name() {
        let result = from_preset("nonexistent-preset", false);
        assert!(result.is_err(), "invalid preset should return Err");
    }

    // -- SystemThemeExt + from_system tests --
    // SystemTheme has pub(crate) fields, so it can only be obtained via
    // SystemTheme::from_system(). These tests verify the trait and function
    // when a system theme is available, and gracefully skip when not.

    #[test]
    fn system_theme_ext_to_gpui_theme() {
        // from_system() may fail on CI (no desktop env) — skip gracefully
        let Ok(sys) = SystemTheme::from_system() else {
            return;
        };
        let theme = sys.to_gpui_theme();
        assert_eq!(
            theme.is_dark(),
            sys.is_dark,
            "to_gpui_theme() is_dark should match SystemTheme.is_dark"
        );
    }

    #[test]
    fn from_system_does_not_panic() {
        // Just verify no panic — result may be Err on CI
        let _ = from_system();
    }

    #[test]
    fn from_system_matches_manual_path() {
        let Ok(sys) = SystemTheme::from_system() else {
            return;
        };
        let via_convenience = sys.to_gpui_theme();
        let via_manual = to_theme(sys.active(), &sys.name, sys.is_dark);
        // Both paths should produce identical results
        assert_eq!(
            via_convenience.is_dark(),
            via_manual.is_dark(),
            "convenience and manual paths should agree on is_dark"
        );
    }
}
