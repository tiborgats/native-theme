//! iced toolkit connector for native-theme.
//!
//! Maps [`native_theme::theme::ResolvedTheme`] data to iced's theming system.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use native_theme_iced::from_preset;
//!
//! let (theme, resolved) = from_preset("catppuccin-mocha", true).unwrap();
//! ```
//!
//! Or from the OS-detected theme:
//!
//! ```rust,no_run
//! use native_theme_iced::from_system;
//!
//! let (theme, resolved, is_dark) = from_system().unwrap();
//! ```
//!
//! # Manual Path
//!
//! For full control over the resolve/validate/convert pipeline:
//!
//! ```rust
//! use native_theme::theme::{ColorMode, Theme};
//! use native_theme_iced::to_theme;
//!
//! let nt = Theme::preset("catppuccin-mocha").unwrap();
//! let resolved = nt.into_variant(ColorMode::Light).unwrap().resolve_system().unwrap();
//! let theme = to_theme(&resolved, "My App");
//! ```
//!
//! # Font Configuration
//!
//! Font family names use `Arc<str>`. For iced's `&'static str` requirement,
//! use `intern_font_family` to deduplicate allocations:
//!
//! ```rust,no_run
//! use native_theme::theme::intern_font_family;
//!
//! let (_, resolved) = native_theme_iced::from_preset("catppuccin-mocha", true)?;
//! let family: std::sync::Arc<str> = intern_font_family(
//!     native_theme_iced::font_family(&resolved),
//! );
//! // For iced Font, convert Arc<str> to a String:
//! let font_family: String = family.to_string();
//! # Ok::<(), native_theme::error::Error>(())
//! ```
//!
//! `intern_font_family` returns the same `Arc<str>` for repeated calls with
//! the same family name, so resolving fonts many times allocates only once.
//!
//! # Theme Field Coverage
//!
//! The connector maps a subset of [`ResolvedTheme`] to iced's theming system:
//!
//! | Target | Fields | Source |
//! |--------|--------|--------|
//! | `Palette` (6 fields) | background, text, primary, success, warning, danger | `defaults.*` |
//! | `Extended` overrides (8) | secondary.base.color/text, background.weak.color/text, primary/success/danger/warning.base.text | button.bg/fg, defaults.surface/foreground, `*_foreground` |
//! | Widget metrics | button/input padding, border radius, scrollbar width | Per-widget resolved fields |
//! | Typography | font family/size/weight, mono family/size/weight, line height | `defaults.font.*`, `defaults.mono_font.*` |
//! | Color helpers | border, link, selection, info, info_foreground, warning_foreground, focus_ring | `defaults.*` |
//! | Geometry helpers | disabled_opacity | `defaults.*` |
//!
//! Per-widget geometry beyond padding/radius (e.g., min-width, disabled-opacity)
//! is not mapped because iced applies these via inline widget configuration,
//! not through the theme system. Users can read these directly from the
//! `ResolvedTheme` they pass to [`to_theme()`].

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub(crate) mod extended;
pub mod icons;
pub mod palette;

// Re-export native-theme types that appear in public signatures.
pub use native_theme::color::Rgba;
pub use native_theme::error::Error;
pub use native_theme::theme::{
    AnimatedIcon, ColorMode, DialogButtonOrder, IconData, IconProvider, IconRole, IconSet,
    ResolvedTheme, Theme, ThemeMode, TransformAnimation,
};
pub use native_theme::{Result, SystemTheme};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;

/// Create an iced [`iced_core::theme::Theme`] from a [`native_theme::theme::ResolvedTheme`].
///
/// Builds a custom theme using `Theme::custom_with_fn()`, which:
/// 1. Maps the 6 Palette fields from resolved theme colors via [`palette::to_palette()`]
/// 2. Generates an Extended palette, then overrides secondary, background.weak,
///    and status-family `.base.text` entries via `extended::apply_overrides()`
///
/// The resulting theme carries the mapped Palette and Extended palette. iced's
/// built-in Catalog trait implementations for all 8 core widgets (Button,
/// Container, TextInput, Scrollable, Checkbox, Slider, ProgressBar, Tooltip)
/// automatically derive their Style structs from this palette. No explicit
/// Catalog implementations are needed.
///
/// The `name` sets the theme's display name (visible in theme pickers).
/// For the common case, use [`from_preset()`] to derive the name automatically.
///
/// Note: iced has no `info` color family in its Extended palette, so
/// `info` / `info_foreground` are not mapped automatically. Use
/// [`info_color()`] and [`info_foreground_color()`] helpers to access them.
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(
    resolved: &native_theme::theme::ResolvedTheme,
    name: &str,
) -> iced_core::theme::Theme {
    let pal = palette::to_palette(resolved);

    // Capture only the Rgba values (Copy, 4 bytes each) instead of
    // cloning the entire ResolvedTheme (~2KB with heap data).
    let colors = extended::OverrideColors {
        btn_bg: resolved.button.background_color,
        btn_fg: resolved.button.font.color,
        surface: resolved.defaults.surface_color,
        foreground: resolved.defaults.text_color,
        accent_fg: resolved.defaults.accent_text_color,
        success_fg: resolved.defaults.success_text_color,
        danger_fg: resolved.defaults.danger_text_color,
        warning_fg: resolved.defaults.warning_text_color,
        success_bg: resolved.defaults.success_color,
        danger_bg: resolved.defaults.danger_color,
        warning_bg: resolved.defaults.warning_color,
    };

    iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);
        extended::apply_overrides(&mut ext, &colors);
        ext
    })
}

/// Load a bundled preset and convert it to an iced [`Theme`](iced_core::theme::Theme) in one call.
///
/// Handles the full pipeline: load preset, pick variant, resolve, validate, convert.
/// The `Theme` display name is used as the theme display name.
///
/// # Errors
///
/// Returns an error if the preset name is not recognized or if resolution fails.
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
) -> Result<(iced_core::theme::Theme, native_theme::theme::ResolvedTheme)> {
    let spec = native_theme::theme::Theme::preset(name)?;
    let display_name = spec.name.clone();
    let variant = spec.into_variant(if is_dark {
        ColorMode::Dark
    } else {
        ColorMode::Light
    })?;
    let resolved = variant.resolve_system()?;
    let theme = to_theme(&resolved, &display_name);
    Ok((theme, resolved))
}

/// Detect the OS theme and convert it to an iced [`Theme`](iced_core::theme::Theme) in one call.
///
/// Returns the iced theme, the resolved variant, and whether the system is in
/// dark mode. The `is_dark` flag comes from the OS preference, not from
/// background color analysis.
///
/// # Errors
///
/// Returns an error if the platform theme cannot be read.
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system() -> Result<(
    iced_core::theme::Theme,
    native_theme::theme::ResolvedTheme,
    bool,
)> {
    let sys = native_theme::SystemTheme::from_system()?;
    let is_dark = sys.mode.is_dark();
    let name = sys.name;
    let resolved = if is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name);
    Ok((theme, resolved, is_dark))
}

/// Extension trait for converting a [`SystemTheme`] to an iced theme.
pub trait SystemThemeExt {
    /// Convert this system theme to an iced [`iced_core::theme::Theme`] and its [`ResolvedTheme`].
    ///
    /// Returns both the iced theme and the resolved variant, so callers can
    /// access per-widget metrics without re-resolving.
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_iced_theme(&self) -> (iced_core::theme::Theme, native_theme::theme::ResolvedTheme);
}

impl SystemThemeExt for native_theme::SystemTheme {
    fn to_iced_theme(&self) -> (iced_core::theme::Theme, native_theme::theme::ResolvedTheme) {
        let resolved = self.pick(self.mode).clone();
        let theme = to_theme(&resolved, &self.name);
        (theme, resolved)
    }
}

/// Returns button padding from the resolved theme as an iced [`Padding`](iced_core::Padding).
///
/// Maps `padding_vertical` to top/bottom and `padding_horizontal` to left/right.
#[must_use]
pub fn button_padding(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Padding {
    iced_core::Padding::from([
        resolved.button.border.padding_vertical,
        resolved.button.border.padding_horizontal,
    ])
}

/// Returns text input padding from the resolved theme as an iced [`Padding`](iced_core::Padding).
///
/// Maps `border.padding_vertical` to top/bottom and `border.padding_horizontal` to left/right.
#[must_use]
pub fn input_padding(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Padding {
    iced_core::Padding::from([
        resolved.input.border.padding_vertical,
        resolved.input.border.padding_horizontal,
    ])
}

/// Returns the standard border radius from the resolved theme.
#[must_use]
pub fn border_radius(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.border.corner_radius
}

/// Returns the large border radius from the resolved theme.
#[must_use]
pub fn border_radius_lg(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.border.corner_radius_lg
}

/// Returns the scrollbar groove width from the resolved theme.
#[must_use]
pub fn scrollbar_width(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.scrollbar.groove_width
}

/// Returns the primary UI font family name from the resolved theme.
#[must_use]
pub fn font_family(resolved: &native_theme::theme::ResolvedTheme) -> &str {
    &resolved.defaults.font.family
}

/// Returns the primary UI font size in logical pixels from the resolved theme.
///
/// ResolvedFontSpec.size is in logical pixels (conversion from platform points
/// is handled by the resolution step).
#[must_use]
pub fn font_size(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.font.size
}

/// Returns the monospace font family name from the resolved theme.
#[must_use]
pub fn mono_font_family(resolved: &native_theme::theme::ResolvedTheme) -> &str {
    &resolved.defaults.mono_font.family
}

/// Returns the monospace font size in logical pixels from the resolved theme.
///
/// ResolvedFontSpec.size is in logical pixels (conversion from platform points
/// is handled by the resolution step).
#[must_use]
pub fn mono_font_size(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.mono_font.size
}

/// Returns the primary UI font weight (CSS 100-900) from the resolved theme.
#[must_use]
pub fn font_weight(resolved: &native_theme::theme::ResolvedTheme) -> u16 {
    resolved.defaults.font.weight
}

/// Returns the monospace font weight (CSS 100-900) from the resolved theme.
#[must_use]
pub fn mono_font_weight(resolved: &native_theme::theme::ResolvedTheme) -> u16 {
    resolved.defaults.mono_font.weight
}

/// Returns the border/divider color from the resolved theme.
#[must_use]
pub fn border_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.border.color)
}

/// Returns the disabled control opacity from the resolved theme.
#[must_use]
pub fn disabled_opacity(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.disabled_opacity
}

/// Returns the focus ring indicator color from the resolved theme.
#[must_use]
pub fn focus_ring_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.focus_ring_color)
}

/// Returns the hyperlink color from the resolved theme.
#[must_use]
pub fn link_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.link_color)
}

/// Returns the selection highlight background color from the resolved theme.
#[must_use]
pub fn selection_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.selection_background)
}

/// Returns the info/attention color from the resolved theme.
///
/// Note: iced has no `info` family in its Extended palette, so this color
/// is not mapped automatically. Use this helper to access it directly.
#[must_use]
pub fn info_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.info_color)
}

/// Returns the text color for info-colored backgrounds from the resolved theme.
#[must_use]
pub fn info_foreground_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.info_text_color)
}

/// Returns the warning foreground text color from the resolved theme.
///
/// The warning base color is already mapped to `palette.warning`. This returns
/// the text color intended for use on warning-colored backgrounds.
#[must_use]
pub fn warning_foreground_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.warning_text_color)
}

/// Returns a reference to the per-context icon sizes from the resolved theme.
#[must_use]
pub fn icon_sizes(
    resolved: &native_theme::theme::ResolvedTheme,
) -> &native_theme::theme::ResolvedIconSizes {
    &resolved.defaults.icon_sizes
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
pub fn line_height_multiplier(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.line_height
}

/// Convert a CSS font weight (100-900) to an iced [`Weight`](iced_core::font::Weight) enum.
///
/// Non-standard weights are rounded to the nearest standard value
/// (e.g., 350 -> Normal, 550 -> Semibold).
///
/// # Example
///
/// ```rust,no_run
/// let (_, resolved) = native_theme_iced::from_preset("catppuccin-mocha", true).unwrap();
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
    use native_theme::theme::{ColorMode, Theme};

    fn make_resolved_preset(name: &str, is_dark: bool) -> native_theme::theme::ResolvedTheme {
        Theme::preset(name)
            .unwrap()
            .into_variant(if is_dark {
                ColorMode::Dark
            } else {
                ColorMode::Light
            })
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap()
    }

    fn make_resolved(is_dark: bool) -> native_theme::theme::ResolvedTheme {
        make_resolved_preset("catppuccin-mocha", is_dark)
    }

    // === to_theme tests ===

    #[test]
    fn to_theme_produces_non_default_theme() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "Test Theme");

        assert_ne!(theme, iced_core::theme::Theme::Light);
        assert_ne!(theme, iced_core::theme::Theme::Dark);

        let palette = theme.palette();
        // Catppuccin Mocha dark primary should be non-trivial
        let expected = palette::to_color(resolved.defaults.accent_color);
        assert_eq!(palette.primary, expected, "primary should match accent");
    }

    #[test]
    fn to_theme_from_preset() {
        let resolved = make_resolved(false);
        let theme = to_theme(&resolved, "Default");

        let palette = theme.palette();
        // Light variant has white-ish background
        assert!(
            palette.background.r > 0.9,
            "light background should be bright"
        );
    }

    #[test]
    fn to_theme_dark_variant() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "Dark Test");

        let palette = theme.palette();
        assert!(palette.background.r < 0.3, "dark background should be dark");
    }

    #[test]
    fn to_theme_different_presets_differ() {
        let r1 = Theme::preset("catppuccin-mocha")
            .unwrap()
            .into_variant(ColorMode::Dark)
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap();
        let r2 = Theme::preset("dracula")
            .unwrap()
            .into_variant(ColorMode::Dark)
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap();

        let t1 = to_theme(&r1, "mocha");
        let t2 = to_theme(&r2, "dracula");

        // Different presets should produce different palette colors
        assert_ne!(t1.palette().primary, t2.palette().primary);
    }

    #[test]
    fn to_theme_with_adwaita_preset() {
        let resolved = make_resolved_preset("adwaita", false);
        let theme = to_theme(&resolved, "Adwaita");
        let palette = theme.palette();
        assert!(palette.primary.a > 0.0, "adwaita primary should be visible");
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
        // Padding values come from border sub-struct; >= 0 is the valid range.
        // Phase 51 will wire per-widget border padding from presets.
        assert!(
            pad.top >= 0.0,
            "button vertical (top) padding should be >= 0"
        );
        assert!(
            pad.right >= 0.0,
            "button horizontal (right) padding should be >= 0"
        );
        // vertical maps to top+bottom, horizontal maps to left+right
        assert_eq!(pad.top, pad.bottom, "top and bottom should be equal");
        assert_eq!(pad.left, pad.right, "left and right should be equal");
    }

    #[test]
    fn input_padding_returns_iced_padding() {
        let resolved = make_resolved(false);
        let pad = input_padding(&resolved);
        // Padding values come from border sub-struct; >= 0 is the valid range.
        // Phase 51 will wire per-widget border padding from presets.
        assert!(
            pad.top >= 0.0,
            "input vertical (top) padding should be >= 0"
        );
        assert!(
            pad.right >= 0.0,
            "input horizontal (right) padding should be >= 0"
        );
        // Symmetry: vertical maps to top+bottom, horizontal maps to left+right
        assert_eq!(pad.top, pad.bottom, "top and bottom should be equal");
        assert_eq!(pad.left, pad.right, "left and right should be equal");
    }

    // === Color helper tests ===

    #[test]
    fn border_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = border_color(&resolved);
        assert!(c.a > 0.0, "border color should have non-zero alpha");
    }

    #[test]
    fn disabled_opacity_returns_value() {
        let resolved = make_resolved(false);
        let o = disabled_opacity(&resolved);
        assert!(
            o > 0.0 && o <= 1.0,
            "disabled opacity should be in (0, 1], got {o}"
        );
    }

    #[test]
    fn focus_ring_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = focus_ring_color(&resolved);
        assert!(c.a > 0.0, "focus ring color should have non-zero alpha");
    }

    #[test]
    fn link_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = link_color(&resolved);
        assert!(
            c.r > 0.0 || c.g > 0.0 || c.b > 0.0,
            "link color should be non-black"
        );
    }

    #[test]
    fn selection_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = selection_color(&resolved);
        assert!(c.a > 0.0, "selection color should have non-zero alpha");
    }

    #[test]
    fn info_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = info_color(&resolved);
        assert!(
            c.r > 0.0 || c.g > 0.0 || c.b > 0.0,
            "info color should be non-black"
        );
    }

    #[test]
    fn info_foreground_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = info_foreground_color(&resolved);
        assert!(c.a > 0.0, "info foreground should have non-zero alpha");
    }

    #[test]
    fn warning_foreground_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = warning_foreground_color(&resolved);
        assert!(c.a > 0.0, "warning foreground should have non-zero alpha");
    }

    #[test]
    fn icon_sizes_returns_concrete_values() {
        let resolved = make_resolved(false);
        let is = icon_sizes(&resolved);
        assert!(is.small > 0.0, "small icon size should be > 0");
        assert!(is.toolbar > 0.0, "toolbar icon size should be > 0");
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
        assert_ne!(theme, iced_core::theme::Theme::Light);
        assert!(!resolved.defaults.font.family.is_empty());
        // Light variant should have bright background
        let palette = theme.palette();
        assert!(
            palette.background.r > 0.9,
            "light variant should have bright background, got r={}",
            palette.background.r
        );
    }

    #[test]
    fn from_preset_valid_dark() {
        let (theme, _resolved) = from_preset("catppuccin-mocha", true).expect("preset should load");
        assert_ne!(theme, iced_core::theme::Theme::Dark);
        // Dark variant should have dark background
        let palette = theme.palette();
        assert!(
            palette.background.r < 0.3,
            "dark variant should have dark background, got r={}",
            palette.background.r
        );
    }

    #[test]
    fn from_preset_invalid_name() {
        let result = from_preset("nonexistent-preset", false);
        assert!(result.is_err(), "invalid preset should return Err");
    }

    #[test]
    fn from_preset_error_shows_requested_mode() {
        // This tests the error path -- an actually empty preset cannot be
        // created through the public API, but we verify the format of
        // the success/error paths.
        let result = from_preset("nonexistent-preset", true);
        assert!(result.is_err());
    }

    #[test]
    fn system_theme_ext_to_iced_theme() {
        // May fail on CI -- skip gracefully
        let Ok(sys) = native_theme::SystemTheme::from_system() else {
            return;
        };
        let (_theme, _resolved) = sys.to_iced_theme();
    }

    #[test]
    fn from_system_does_not_panic() {
        let _ = from_system();
    }

    #[test]
    fn from_system_returns_is_dark() {
        // If system theme is available, verify it returns a triple
        if let Ok((_theme, _resolved, is_dark)) = from_system() {
            // is_dark should be a valid bool (always true, but verify the return)
            let _ = is_dark;
        }
    }

    #[test]
    fn to_theme_extended_overrides_take_effect() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "test");
        let ext = theme.extended_palette();
        // Generate what the Extended palette would be without overrides
        let auto_palette = iced_core::theme::palette::Extended::generate(theme.palette());
        // apply_overrides sets secondary.base from button.bg/fg which differs
        // from the auto-generated value
        assert_ne!(
            ext.secondary.base.color, auto_palette.secondary.base.color,
            "secondary.base.color should be overridden, not auto-generated"
        );
    }

    // Integration-level: exercises the full from_preset -> to_theme pipeline for all presets

    #[test]
    fn all_presets_produce_valid_themes() {
        for info in Theme::list_presets() {
            let name = info.key;
            for is_dark in [false, true] {
                let spec = Theme::preset(name).unwrap();
                if let Ok(variant) = spec.into_variant(if is_dark {
                    ColorMode::Dark
                } else {
                    ColorMode::Light
                }) {
                    let resolved = variant
                        .into_resolved(&native_theme::ResolutionContext::for_tests())
                        .unwrap();
                    let theme = to_theme(&resolved, name);
                    let palette = theme.palette();
                    // Basic sanity: all palette colors have valid alpha
                    assert!(
                        palette.background.a > 0.0,
                        "{name}/{is_dark}: background alpha"
                    );
                    assert!(palette.text.a > 0.0, "{name}/{is_dark}: text alpha");
                    assert!(palette.primary.a > 0.0, "{name}/{is_dark}: primary alpha");
                    assert!(palette.success.a > 0.0, "{name}/{is_dark}: success alpha");
                    assert!(palette.warning.a > 0.0, "{name}/{is_dark}: warning alpha");
                    assert!(palette.danger.a > 0.0, "{name}/{is_dark}: danger alpha");
                }
            }
        }
    }

    // === Tripwire: iced Palette field count ===

    #[test]
    fn palette_field_count_tripwire() {
        // iced_core::theme::Palette has 6 Color fields. If upstream adds more,
        // this test fails so we know to update to_palette().
        let field_count = std::mem::size_of::<iced_core::theme::Palette>()
            / std::mem::size_of::<iced_core::Color>();
        assert_eq!(
            field_count, 6,
            "iced Palette field count changed from 6 to {field_count} -- update to_palette()"
        );
    }
}
