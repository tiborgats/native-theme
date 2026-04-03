//! iced toolkit connector for native-theme.
//!
//! Maps [`native_theme::ResolvedThemeVariant`] data to iced's theming system.
//!
//! # Quick Start
//!
//! ```ignore
//! use native_theme_iced::from_preset;
//!
//! let (theme, resolved) = from_preset("catppuccin-mocha", true)?;
//! ```
//!
//! Or from the OS-detected theme:
//!
//! ```ignore
//! use native_theme_iced::from_system;
//!
//! let (theme, resolved) = from_system()?;
//! ```
//!
//! # Manual Path
//!
//! For full control over the resolve/validate/convert pipeline:
//!
//! ```rust
//! use native_theme::ThemeSpec;
//! use native_theme_iced::to_theme;
//!
//! let nt = ThemeSpec::preset("catppuccin-mocha").unwrap();
//! let resolved = nt.into_variant(false).unwrap().into_resolved().unwrap();
//! let theme = to_theme(&resolved, "My App");
//! ```
//!
//! # Font Configuration
//!
//! To use theme fonts with iced widgets, leak the family name to obtain
//! the `&'static str` required by [`iced_core::font::Family::Name`]:
//!
//! ```ignore
//! let name: &'static str = Box::leak(
//!     native_theme_iced::font_family(&resolved).to_string().into_boxed_str()
//! );
//! let font = iced_core::Font {
//!     family: iced_core::font::Family::Name(name),
//!     weight: native_theme_iced::to_iced_weight(
//!         native_theme_iced::font_weight(&resolved)
//!     ),
//!     ..Default::default()
//! };
//! ```
//!
//! This is the standard iced pattern for runtime font names. Each leak is
//! ~10-20 bytes and persists for the app lifetime. Call once at theme init,
//! not per-frame.
//!
//! # Theme Field Coverage
//!
//! The connector maps a subset of [`ResolvedThemeVariant`] to iced's theming system:
//!
//! | Target | Fields | Source |
//! |--------|--------|--------|
//! | `Palette` (6 fields) | background, text, primary, success, warning, danger | `defaults.*` |
//! | `Extended` overrides (4) | secondary.base.color/text, background.weak.color/text | button.bg/fg, defaults.surface/foreground |
//! | Widget metrics | button/input padding, border radius, scrollbar width | Per-widget resolved fields |
//! | Typography | font family/size/weight, mono family/size/weight, line height | `defaults.font.*`, `defaults.mono_font.*` |
//!
//! Per-widget geometry beyond padding/radius (e.g., min-width, disabled-opacity)
//! is not mapped because iced applies these via inline widget configuration,
//! not through the theme system. Users can read these directly from the
//! `ResolvedThemeVariant` they pass to [`to_theme()`].

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod extended;
pub mod icons;
pub mod palette;

// Re-export native-theme types that appear in public signatures.
pub use native_theme::{
    AnimatedIcon, Error, IconData, IconProvider, IconRole, IconSet, ResolvedThemeVariant, Result,
    Rgba, SystemTheme, ThemeSpec, ThemeVariant, TransformAnimation,
};

/// Create an iced [`iced_core::theme::Theme`] from a [`native_theme::ResolvedThemeVariant`].
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
///
/// The `name` sets the theme's display name (visible in theme pickers).
/// For the common case, use [`from_preset()`] to derive the name automatically.
#[must_use]
pub fn to_theme(
    resolved: &native_theme::ResolvedThemeVariant,
    name: &str,
) -> iced_core::theme::Theme {
    let pal = palette::to_palette(resolved);

    // Capture only the 4 Rgba values (Copy, 4 bytes each) instead of
    // cloning the entire ResolvedThemeVariant (~2KB with heap data).
    let btn_bg = resolved.button.background;
    let btn_fg = resolved.button.foreground;
    let surface = resolved.defaults.surface;
    let foreground = resolved.defaults.foreground;

    iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);
        ext.secondary.base.color = palette::to_color(btn_bg);
        ext.secondary.base.text = palette::to_color(btn_fg);
        ext.background.weak.color = palette::to_color(surface);
        ext.background.weak.text = palette::to_color(foreground);
        ext
    })
}

/// Load a bundled preset and convert it to an iced [`Theme`](iced_core::theme::Theme) in one call.
///
/// Handles the full pipeline: load preset, pick variant, resolve, validate, convert.
/// The preset name is used as the theme display name.
///
/// # Errors
///
/// Returns an error if the preset name is not recognized or if resolution fails.
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
) -> native_theme::Result<(iced_core::theme::Theme, native_theme::ResolvedThemeVariant)> {
    let spec = native_theme::ThemeSpec::preset(name)?;
    let variant = spec.into_variant(is_dark).ok_or_else(|| {
        native_theme::Error::Format(format!("preset '{name}' has no light or dark variant"))
    })?;
    let resolved = variant.into_resolved()?;
    let theme = to_theme(&resolved, name);
    Ok((theme, resolved))
}

/// Detect the OS theme and convert it to an iced [`Theme`](iced_core::theme::Theme) in one call.
///
/// # Errors
///
/// Returns an error if the platform theme cannot be read.
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system()
-> native_theme::Result<(iced_core::theme::Theme, native_theme::ResolvedThemeVariant)> {
    let sys = native_theme::SystemTheme::from_system()?;
    let name = sys.name;
    let resolved = if sys.is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name);
    Ok((theme, resolved))
}

/// Extension trait for converting a [`SystemTheme`] to an iced theme.
pub trait SystemThemeExt {
    /// Convert this system theme to an iced [`Theme`](iced_core::theme::Theme).
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_iced_theme(&self) -> iced_core::theme::Theme;
}

impl SystemThemeExt for native_theme::SystemTheme {
    fn to_iced_theme(&self) -> iced_core::theme::Theme {
        to_theme(self.active(), &self.name)
    }
}

/// Returns button padding from the resolved theme as an iced [`Padding`](iced_core::Padding).
///
/// Maps `padding_vertical` to top/bottom and `padding_horizontal` to left/right.
#[must_use]
pub fn button_padding(resolved: &native_theme::ResolvedThemeVariant) -> iced_core::Padding {
    iced_core::Padding::from([
        resolved.button.padding_vertical,
        resolved.button.padding_horizontal,
    ])
}

/// Returns text input padding from the resolved theme as an iced [`Padding`](iced_core::Padding).
///
/// Maps `padding_vertical` to top/bottom and `padding_horizontal` to left/right.
#[must_use]
pub fn input_padding(resolved: &native_theme::ResolvedThemeVariant) -> iced_core::Padding {
    iced_core::Padding::from([
        resolved.input.padding_vertical,
        resolved.input.padding_horizontal,
    ])
}

/// Returns the standard border radius from the resolved theme.
#[must_use]
pub fn border_radius(resolved: &native_theme::ResolvedThemeVariant) -> f32 {
    resolved.defaults.radius
}

/// Returns the large border radius from the resolved theme.
#[must_use]
pub fn border_radius_lg(resolved: &native_theme::ResolvedThemeVariant) -> f32 {
    resolved.defaults.radius_lg
}

/// Returns the scrollbar width from the resolved theme.
#[must_use]
pub fn scrollbar_width(resolved: &native_theme::ResolvedThemeVariant) -> f32 {
    resolved.scrollbar.width
}

/// Returns the primary UI font family name from the resolved theme.
#[must_use]
pub fn font_family(resolved: &native_theme::ResolvedThemeVariant) -> &str {
    &resolved.defaults.font.family
}

/// Returns the primary UI font size in logical pixels from the resolved theme.
///
/// ResolvedFontSpec.size is already in logical pixels -- no pt-to-px conversion
/// is applied.
#[must_use]
pub fn font_size(resolved: &native_theme::ResolvedThemeVariant) -> f32 {
    resolved.defaults.font.size
}

/// Returns the monospace font family name from the resolved theme.
#[must_use]
pub fn mono_font_family(resolved: &native_theme::ResolvedThemeVariant) -> &str {
    &resolved.defaults.mono_font.family
}

/// Returns the monospace font size in logical pixels from the resolved theme.
///
/// ResolvedFontSpec.size is already in logical pixels -- no pt-to-px conversion
/// is applied.
#[must_use]
pub fn mono_font_size(resolved: &native_theme::ResolvedThemeVariant) -> f32 {
    resolved.defaults.mono_font.size
}

/// Returns the primary UI font weight (CSS 100-900) from the resolved theme.
#[must_use]
pub fn font_weight(resolved: &native_theme::ResolvedThemeVariant) -> u16 {
    resolved.defaults.font.weight
}

/// Returns the monospace font weight (CSS 100-900) from the resolved theme.
#[must_use]
pub fn mono_font_weight(resolved: &native_theme::ResolvedThemeVariant) -> u16 {
    resolved.defaults.mono_font.weight
}

/// Returns the line height multiplier from the resolved theme.
///
/// The raw multiplier (e.g., 1.4). Use with iced's
/// `LineHeight::Relative(native_theme_iced::line_height_multiplier(&r))`
/// for Text widgets. Font-size agnostic -- works correctly for both
/// the primary UI font and monospace text.
///
/// For absolute pixels (layout math), multiply by the appropriate
/// font size: `line_height_multiplier(&r) * font_size(&r)`.
#[must_use]
pub fn line_height_multiplier(resolved: &native_theme::ResolvedThemeVariant) -> f32 {
    resolved.defaults.line_height
}

/// Convert a CSS font weight (100-900) to an iced [`Weight`](iced_core::font::Weight) enum.
///
/// Non-standard weights are rounded to the nearest standard value
/// (e.g., 350 -> Normal, 550 -> Semibold).
///
/// # Example
///
/// ```ignore
/// let weight = native_theme_iced::to_iced_weight(
///     native_theme_iced::font_weight(&resolved),
/// );
/// ```
#[must_use]
pub fn to_iced_weight(css_weight: u16) -> iced_core::font::Weight {
    use iced_core::font::Weight;
    match css_weight {
        0..=149 => Weight::Thin,
        150..=249 => Weight::ExtraLight,
        250..=349 => Weight::Light,
        350..=449 => Weight::Normal,
        450..=549 => Weight::Medium,
        550..=649 => Weight::Semibold,
        650..=749 => Weight::Bold,
        750..=849 => Weight::ExtraBold,
        850.. => Weight::Black,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::ThemeSpec;

    fn make_resolved(is_dark: bool) -> native_theme::ResolvedThemeVariant {
        ThemeSpec::preset("catppuccin-mocha")
            .unwrap()
            .into_variant(is_dark)
            .unwrap()
            .into_resolved()
            .unwrap()
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
        assert!(
            r >= border_radius(&resolved),
            "radius_lg should be >= radius"
        );
    }

    #[test]
    fn scrollbar_width_returns_resolved_value() {
        let resolved = make_resolved(false);
        let w = scrollbar_width(&resolved);
        assert!(w > 0.0, "scrollbar width should be > 0");
    }

    #[test]
    fn button_padding_returns_iced_padding() {
        let resolved = make_resolved(false);
        let pad = button_padding(&resolved);
        assert!(pad.top > 0.0, "button vertical (top) padding should be > 0");
        assert!(
            pad.right > 0.0,
            "button horizontal (right) padding should be > 0"
        );
        // vertical maps to top+bottom, horizontal maps to left+right
        assert_eq!(pad.top, pad.bottom, "top and bottom should be equal");
        assert_eq!(pad.left, pad.right, "left and right should be equal");
    }

    #[test]
    fn input_padding_returns_iced_padding() {
        let resolved = make_resolved(false);
        let pad = input_padding(&resolved);
        assert!(pad.top > 0.0, "input vertical (top) padding should be > 0");
        assert!(
            pad.right > 0.0,
            "input horizontal (right) padding should be > 0"
        );
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

    #[test]
    fn font_weight_returns_concrete_value() {
        let resolved = make_resolved(false);
        let w = font_weight(&resolved);
        assert!(
            (100..=900).contains(&w),
            "font weight should be 100-900, got {}",
            w
        );
    }

    #[test]
    fn mono_font_weight_returns_concrete_value() {
        let resolved = make_resolved(false);
        let w = mono_font_weight(&resolved);
        assert!(
            (100..=900).contains(&w),
            "mono font weight should be 100-900, got {}",
            w
        );
    }

    #[test]
    fn line_height_multiplier_returns_concrete_value() {
        let resolved = make_resolved(false);
        let lh = line_height_multiplier(&resolved);
        assert!(lh > 0.0, "line height multiplier should be > 0");
        assert!(
            lh < 5.0,
            "line height multiplier should be a multiplier (e.g. 1.4), got {}",
            lh
        );
    }

    #[test]
    fn to_iced_weight_standard_weights() {
        use iced_core::font::Weight;
        assert_eq!(to_iced_weight(100), Weight::Thin);
        assert_eq!(to_iced_weight(200), Weight::ExtraLight);
        assert_eq!(to_iced_weight(300), Weight::Light);
        assert_eq!(to_iced_weight(400), Weight::Normal);
        assert_eq!(to_iced_weight(500), Weight::Medium);
        assert_eq!(to_iced_weight(600), Weight::Semibold);
        assert_eq!(to_iced_weight(700), Weight::Bold);
        assert_eq!(to_iced_weight(800), Weight::ExtraBold);
        assert_eq!(to_iced_weight(900), Weight::Black);
    }

    #[test]
    fn to_iced_weight_non_standard_rounds_correctly() {
        use iced_core::font::Weight;
        assert_eq!(to_iced_weight(350), Weight::Normal);
        assert_eq!(to_iced_weight(450), Weight::Medium);
        assert_eq!(to_iced_weight(550), Weight::Semibold);
        assert_eq!(to_iced_weight(0), Weight::Thin);
        assert_eq!(to_iced_weight(1000), Weight::Black);
    }

    // === Convenience API tests ===

    #[test]
    fn from_preset_valid_light() {
        let (theme, resolved) = from_preset("catppuccin-mocha", false).expect("preset should load");
        // Should produce a valid custom theme (not Light or Dark built-in)
        assert_ne!(theme, iced_core::theme::Theme::Light);
        // Should also return the resolved variant
        assert!(!resolved.defaults.font.family.is_empty());
    }

    #[test]
    fn from_preset_valid_dark() {
        let (theme, _resolved) = from_preset("catppuccin-mocha", true).expect("preset should load");
        assert_ne!(theme, iced_core::theme::Theme::Dark);
    }

    #[test]
    fn from_preset_invalid_name() {
        let result = from_preset("nonexistent-preset", false);
        assert!(result.is_err(), "invalid preset should return Err");
    }

    #[test]
    fn system_theme_ext_to_iced_theme() {
        // May fail on CI — skip gracefully
        let Ok(sys) = native_theme::SystemTheme::from_system() else {
            return;
        };
        let _theme = sys.to_iced_theme();
    }

    #[test]
    fn from_system_does_not_panic() {
        let _ = from_system();
    }
}
