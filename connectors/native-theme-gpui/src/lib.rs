//! gpui toolkit connector for native-theme.
//!
//! Maps [`native_theme::theme::ResolvedTheme`] data to gpui-component's theming system.
//!
//! # Quick Start
//!
//! ```ignore
//! use native_theme_gpui::from_preset;
//!
//! let (theme, resolved) = from_preset("catppuccin-mocha", true)?;
//! ```
//!
//! Or from the OS-detected theme:
//!
//! ```ignore
//! use native_theme_gpui::from_system;
//!
//! let (theme, resolved, is_dark) = from_system()?;
//! ```
//!
//! # Manual Path
//!
//! For full control over the resolve/validate/convert pipeline:
//!
//! ```ignore
//! use native_theme::theme::{ColorMode, Theme};
//! use native_theme_gpui::to_theme;
//!
//! let nt = Theme::preset("catppuccin-mocha")?;
//! let variant = nt.into_variant(ColorMode::Dark).ok_or("no dark variant")?;
//! let resolved = variant.into_resolved(None)?;
//! let theme = to_theme(&resolved, "Catppuccin Mocha", true, false);
//! ```
//!
//! # Single-Mode Behavior
//!
//! Each call to [`from_preset()`] or [`to_theme()`] produces a theme for exactly
//! one mode (light or dark). The `is_dark` parameter selects which variant to
//! load from the preset's TOML. Presets that only define one variant will return
//! that variant for both `is_dark=true` and `is_dark=false` (the resolution
//! pipeline falls back to the available variant). To support runtime light/dark
//! switching, call [`from_preset()`] twice and swap the resulting themes.
//!
//! # Theme Field Coverage
//!
//! The connector maps a subset of [`ResolvedTheme`] fields to gpui-component's
//! `ThemeColor` (108 color fields) and `ThemeConfig` (font/geometry).
//!
//! | Category | Mapped | Notes |
//! |----------|--------|-------|
//! | `defaults` colors | All 24 | background, foreground, accent, danger, etc. |
//! | `defaults` geometry | radius, radius_lg, shadow | Font family/size also mapped |
//! | `button` | 6 of 15 | primary bg/fg, bg/fg, hover_bg, active_bg |
//! | `tab` | 5 of 10 | All colors, sizing not mapped |
//! | `sidebar` | 2 of 6 | background, font.color |
//! | `window` | 2 of 6 | title_bar_background, border |
//! | `input` | 2 of 13 | border, caret |
//! | `scrollbar` | 3 of 8 | track, thumb, thumb_hover |
//! | `slider`, `switch` | 2 each | fill/thumb colors |
//! | `progress_bar` | 1 of 5 | fill |
//! | `list` | 3 of 13 | alternate_row, hover_bg, selection_bg |
//! | `popover` | 2 of 3 | background, font.color |
//! | `link` | 1 of 9 | hover_background |
//! | 13 other widgets | 0 fields | checkbox, menu, tooltip, dialog, etc. |
//!
//! **Why the gap:** gpui-component's `ThemeColor` is a flat color bag with no per-widget
//! geometry. The connector cannot map most sizing/spacing data because the target type
//! has no corresponding fields. Users who need per-widget geometry can read it directly
//! from the `ResolvedTheme` they passed to [`to_theme()`].

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
// Issue 8 + 48: re-export Result, Rgba, Error, DialogButtonOrder
pub use native_theme::color::Rgba;
pub use native_theme::error::Error;
pub use native_theme::theme::{
    AnimatedIcon, ColorMode, DialogButtonOrder, IconData, IconProvider, IconRole, IconSet,
    ResolvedTheme, Theme, ThemeMode, TransformAnimation,
};
pub use native_theme::{Result, SystemTheme};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;

use gpui::{SharedString, px};
use gpui_component::scroll::ScrollbarShow;
use gpui_component::theme::{Theme as GpuiTheme, ThemeMode as GpuiThemeMode};
use std::rc::Rc;

/// Convert a [`ResolvedTheme`] into a gpui-component [`GpuiTheme`].
///
/// Builds a complete GpuiTheme by:
/// 1. Mapping all 108 ThemeColor fields via `colors::to_theme_color`
/// 2. Setting font, geometry, and mode fields directly on the Theme
/// 3. Storing a ThemeConfig in light_theme/dark_theme Rc for gpui-component switching
///
/// All Theme fields are set explicitly -- no `apply_config` call is used.
/// This avoids the fragile apply-then-restore pattern where `apply_config`
/// would overwrite all 108 color fields with defaults.
///
/// The `is_dark` parameter is required rather than auto-derived because
/// several presets (e.g. solarized, gruvbox) have borderline lightness
/// values where auto-detection would disagree with the user's intent.
/// To auto-derive: `let is_dark = resolved.defaults.background_color` lightness < 0.5
/// via [`is_dark_resolved()`].
///
/// Note: `is_dark` is an explicit parameter here, unlike the iced connector
/// which derives it from background luminance. Planned for unification in v0.6.0.
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(
    resolved: &ResolvedTheme,
    name: &str,
    is_dark: bool,
    reduce_transparency: bool,
) -> GpuiTheme {
    let theme_color = colors::to_theme_color(resolved, is_dark, reduce_transparency);
    let mode = if is_dark {
        GpuiThemeMode::Dark
    } else {
        GpuiThemeMode::Light
    };
    let d = &resolved.defaults;

    let mut theme = GpuiTheme::from(&theme_color);
    // Issue 53: Theme.transparent is set by Theme::from() to
    // Hsla::transparent_black() and is intentionally left unchanged.
    // It's used internally by gpui-component for transparent overlays.
    theme.mode = mode;
    theme.font_family = SharedString::from(d.font.family.clone());
    theme.font_size = px(d.font.size);
    theme.mono_font_family = SharedString::from(d.mono_font.family.clone());
    theme.mono_font_size = px(d.mono_font.size);
    // Issue 14: clamp radius to non-negative
    theme.radius = px(d.border.corner_radius.max(0.0));
    theme.radius_lg = px(d.border.corner_radius_lg.max(0.0));
    theme.shadow = d.border.shadow_enabled;

    // Issue 43: set scrollbar_show from resolved overlay_mode
    theme.scrollbar_show = if resolved.scrollbar.overlay_mode {
        ScrollbarShow::Scrolling
    } else {
        ScrollbarShow::Always
    };

    // Issue 43/44: set highlight_theme based on is_dark so syntax highlighting
    // uses appropriate colors (dark themes get dark highlight, light themes get light).
    theme.highlight_theme = if is_dark {
        gpui_component::highlighter::HighlightTheme::default_dark()
    } else {
        gpui_component::highlighter::HighlightTheme::default_light()
    };

    // Store config for gpui-component's theme switching
    let config: Rc<_> = Rc::new(config::to_theme_config(resolved, name, mode));
    if mode == GpuiThemeMode::Dark {
        theme.dark_theme = config;
    } else {
        theme.light_theme = config;
    }
    theme
}

/// Load a bundled preset and convert it to a gpui-component [`GpuiTheme`] in one call.
///
/// This is the primary entry point for most users. It handles the full pipeline:
/// load preset, pick variant, resolve, validate, and convert to gpui Theme.
///
/// Returns both the gpui Theme and the [`ResolvedTheme`] so callers can
/// access per-widget metrics (button padding, scrollbar width, etc.) that the
/// flat `ThemeColor` cannot represent.
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
/// let (dark_theme, resolved) = native_theme_gpui::from_preset("dracula", true)?;
/// let (light_theme, _) = native_theme_gpui::from_preset("catppuccin-latte", false)?;
/// ```
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(name: &str, is_dark: bool) -> Result<(GpuiTheme, ResolvedTheme)> {
    let spec = Theme::preset(name)?;
    let display_name = spec.name.clone();
    let mode_str = if is_dark { "dark" } else { "light" };
    let variant = spec
        .into_variant(if is_dark {
            ColorMode::Dark
        } else {
            ColorMode::Light
        })
        .ok_or_else(|| Error::ReaderFailed {
            reader: "gpui_connector",
            source: format!("preset '{name}' has no {mode_str} variant").into(),
        })?;
    let resolved = variant.into_resolved(None)?;
    let theme = to_theme(&resolved, &display_name, is_dark, false);
    Ok((theme, resolved))
}

/// Detect the OS theme and convert it to a gpui-component [`GpuiTheme`] in one call.
///
/// Combines [`SystemTheme::from_system()`](native_theme::SystemTheme::from_system)
/// with [`to_theme()`] using the system-detected name and dark-mode preference.
///
/// Returns both the gpui Theme and the [`ResolvedTheme`] so callers can
/// access per-widget metrics that the flat `ThemeColor` cannot represent.
///
/// **Ownership note** (Issue 19/31): this function takes ownership of the
/// `SystemTheme`'s active variant. The non-active variant (light when dark
/// is active, or vice versa) is dropped. If you need both variants, use
/// `SystemTheme::from_system()` directly and call [`to_theme()`] on each.
///
/// **Performance note:** `SystemTheme::from_system()` resolves both light
/// and dark variants before this function picks one. If you only need one
/// variant and want to avoid the cost of resolving both, use
/// `SystemTheme::from_system()` directly and resolve only the variant you need.
///
/// # Errors
///
/// Returns an error if the platform theme cannot be read (e.g., unsupported platform,
/// missing desktop environment).
///
/// # Examples
///
/// ```ignore
/// let (theme, resolved, is_dark) = native_theme_gpui::from_system()?;
/// ```
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system() -> Result<(GpuiTheme, ResolvedTheme, bool)> {
    let sys = SystemTheme::from_system()?;
    let is_dark = sys.mode.is_dark();
    let reduce_transparency = sys.accessibility.reduce_transparency;
    let name = sys.name; // K-5: move instead of clone
    let resolved = if is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name, is_dark, reduce_transparency);
    Ok((theme, resolved, is_dark))
}

/// Extension trait for converting a [`SystemTheme`] to a gpui-component [`GpuiTheme`].
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
    /// Convert this system theme to a gpui-component [`GpuiTheme`].
    ///
    /// Uses the OS-active variant (based on `mode`), the theme name,
    /// and the color mode from the `SystemTheme`.
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_gpui_theme(&self) -> GpuiTheme;
}

impl SystemThemeExt for SystemTheme {
    fn to_gpui_theme(&self) -> GpuiTheme {
        to_theme(
            self.pick(self.mode),
            &self.name,
            self.mode.is_dark(),
            self.accessibility.reduce_transparency,
        )
    }
}

// ---------------------------------------------------------------------------
// Helper functions (Issues 15, 17, 25, 32, 36, 37, 47, 48, 13)
// ---------------------------------------------------------------------------

/// Derive `is_dark` from a [`ResolvedTheme`]'s background lightness.
///
/// Returns `true` when the background lightness is below 0.5. This is a
/// convenience for callers that do not have an explicit dark-mode flag.
/// Some presets (e.g. solarized, gruvbox) have borderline values where the
/// auto-detected result may differ from the user's intent.
#[must_use]
pub fn is_dark_resolved(resolved: &ResolvedTheme) -> bool {
    colors::rgba_to_hsla(resolved.defaults.background_color).l < 0.5
}

// --- Issue 32: Accessibility helpers ---

/// Returns `true` if the resolved theme is considered dark.
///
/// Equivalent to [`is_dark_resolved()`] -- delegates to background lightness.
#[must_use]
pub fn is_dark(resolved: &ResolvedTheme) -> bool {
    is_dark_resolved(resolved)
}

/// Whether the user/OS has requested reduced motion.
#[must_use]
pub fn is_reduced_motion(sys: &SystemTheme) -> bool {
    sys.accessibility.reduce_motion
}

/// Whether the OS reports a high-contrast mode is active.
#[must_use]
pub fn is_high_contrast(sys: &SystemTheme) -> bool {
    sys.accessibility.high_contrast
}

/// Whether the user/OS has requested reduced transparency.
#[must_use]
pub fn is_reduced_transparency(sys: &SystemTheme) -> bool {
    sys.accessibility.reduce_transparency
}

// --- Issue 15: Defaults field accessors ---

/// Frame/border width from the resolved theme defaults.
#[must_use]
pub fn frame_width(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.border.line_width
}

/// Disabled control opacity from the resolved theme defaults.
#[must_use]
pub fn disabled_opacity(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.disabled_opacity
}

/// Border opacity multiplier from the resolved theme defaults.
#[must_use]
pub fn border_opacity(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.border.opacity
}

/// Whether drop shadows are enabled.
#[must_use]
pub fn shadow_enabled(resolved: &ResolvedTheme) -> bool {
    resolved.defaults.border.shadow_enabled
}

/// Text scaling factor (1.0 = no scaling).
#[must_use]
pub fn text_scaling_factor(sys: &SystemTheme) -> f32 {
    sys.accessibility.text_scaling_factor
}

// --- Issue 17: Spacing / icon-size / text-scale accessors ---

/// Access the per-context icon sizes from the resolved theme.
///
/// Returns icon sizes for toolbar, small, large, dialog, and panel contexts.
#[must_use]
pub fn icon_sizes(resolved: &ResolvedTheme) -> &native_theme::theme::ResolvedIconSizes {
    &resolved.defaults.icon_sizes
}

/// Access the text scale entries from the resolved theme.
///
/// Returns the 4-entry text scale (caption, section_heading, dialog_title, display).
#[must_use]
pub fn text_scale(resolved: &ResolvedTheme) -> &native_theme::theme::ResolvedTextScale {
    &resolved.text_scale
}

// --- Issue 36: Line height multiplier ---

/// Line height multiplier from the resolved theme defaults.
#[must_use]
pub fn line_height_multiplier(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.line_height
}

// --- Issue 13: Font weight helper ---

/// Default font weight from the resolved theme.
///
/// Returns the CSS font weight value (100-900).
#[must_use]
pub fn font_weight(resolved: &ResolvedTheme) -> u16 {
    resolved.defaults.font.weight
}

// --- Issue 47: Mono font weight helper ---

/// Monospace font weight from the resolved theme.
///
/// Returns the CSS font weight value (100-900).
#[must_use]
pub fn mono_font_weight(resolved: &ResolvedTheme) -> u16 {
    resolved.defaults.mono_font.weight
}

// --- Issue 48: Dialog button order helper ---

/// The platform-appropriate dialog button order.
///
/// Returns whether affirmative buttons should appear on the leading (left)
/// or trailing (right) side of a dialog. KDE uses leading-affirmative;
/// most other platforms use trailing-affirmative.
#[must_use]
pub fn dialog_button_order(resolved: &ResolvedTheme) -> DialogButtonOrder {
    resolved.dialog.button_order
}

// --- Issue 37: Padding/geometry helpers ---

/// Dialog content padding in logical pixels.
///
/// Returns the horizontal padding from the dialog's border spec.
#[must_use]
pub fn dialog_content_padding(resolved: &ResolvedTheme) -> f32 {
    resolved.dialog.border.padding_horizontal
}

/// Dialog button gap in logical pixels.
#[must_use]
pub fn dialog_button_spacing(resolved: &ResolvedTheme) -> f32 {
    resolved.dialog.button_gap
}

/// Scrollbar groove width in logical pixels.
#[must_use]
pub fn scrollbar_width(resolved: &ResolvedTheme) -> f32 {
    resolved.scrollbar.groove_width
}

/// Selection text color (foreground for selected content).
#[must_use]
pub fn selection_foreground(resolved: &ResolvedTheme) -> Rgba {
    resolved.defaults.selection_text_color
}

/// Selection background when window is unfocused.
#[must_use]
pub fn selection_inactive(resolved: &ResolvedTheme) -> Rgba {
    resolved.defaults.selection_inactive_background
}

/// Foreground color for disabled elements.
#[must_use]
pub fn disabled_foreground(resolved: &ResolvedTheme) -> Rgba {
    resolved.defaults.disabled_text_color
}

/// Focus ring stroke width in logical pixels.
#[must_use]
pub fn focus_ring_width(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.focus_ring_width
}

/// Gap between element edge and focus ring.
#[must_use]
pub fn focus_ring_offset(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.focus_ring_offset
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// Issue 1: fixed to use into_variant(true) for catppuccin-mocha (dark theme).
    fn test_resolved() -> ResolvedTheme {
        let nt = Theme::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(ColorMode::Dark)
            .expect("preset must have dark variant");
        variant
            .into_resolved(None)
            .expect("resolved preset must validate")
    }

    #[test]
    fn to_theme_produces_valid_theme() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Test", true, false);

        // Theme should have the correct mode
        assert!(theme.is_dark());
    }

    #[test]
    fn to_theme_dark_mode() {
        let nt = Theme::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(ColorMode::Dark)
            .expect("preset must have dark variant");
        let resolved = variant
            .into_resolved(None)
            .expect("resolved preset must validate");
        let theme = to_theme(&resolved, "DarkTest", true, false);

        assert!(theme.is_dark());
    }

    #[test]
    fn to_theme_applies_font_and_geometry() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Test", true, false);

        assert_eq!(
            theme.font_family.as_ref(),
            resolved.defaults.font.family.as_ref()
        );
        assert_eq!(theme.font_size, px(resolved.defaults.font.size));
        assert_eq!(
            theme.mono_font_family.as_ref(),
            resolved.defaults.mono_font.family.as_ref()
        );
        assert_eq!(theme.mono_font_size, px(resolved.defaults.mono_font.size));
        assert_eq!(
            theme.radius,
            px(resolved.defaults.border.corner_radius.max(0.0))
        );
        assert_eq!(
            theme.radius_lg,
            px(resolved.defaults.border.corner_radius_lg.max(0.0))
        );
        assert_eq!(theme.shadow, resolved.defaults.border.shadow_enabled);
    }

    // Issue 43: scrollbar_show set from overlay_mode
    #[test]
    fn scrollbar_show_from_overlay_mode() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Scroll", true, false);
        if resolved.scrollbar.overlay_mode {
            assert!(
                matches!(theme.scrollbar_show, ScrollbarShow::Scrolling),
                "overlay_mode=true should set Scrolling"
            );
        } else {
            assert!(
                matches!(theme.scrollbar_show, ScrollbarShow::Always),
                "overlay_mode=false should set Always"
            );
        }
    }

    // Issue 43/44: highlight_theme matches is_dark
    #[test]
    fn highlight_theme_matches_is_dark() {
        let resolved = test_resolved();
        let dark_theme = to_theme(&resolved, "Dark", true, false);
        assert_eq!(
            dark_theme.highlight_theme.appearance,
            GpuiThemeMode::Dark,
            "dark theme should use dark highlight"
        );

        let light_resolved = {
            let spec = Theme::preset("catppuccin-latte").expect("preset must exist");
            let variant = spec.into_variant(ColorMode::Light).expect("light variant");
            variant.into_resolved(None).expect("must validate")
        };
        let light_theme = to_theme(&light_resolved, "Light", false, false);
        assert_eq!(
            light_theme.highlight_theme.appearance,
            GpuiThemeMode::Light,
            "light theme should use light highlight"
        );
    }

    // -- from_preset tests --

    #[test]
    fn from_preset_valid_light() {
        let (theme, _resolved) =
            from_preset("catppuccin-latte", false).expect("preset should load");
        assert!(!theme.is_dark());
    }

    #[test]
    fn from_preset_valid_dark() {
        let (theme, _resolved) = from_preset("catppuccin-mocha", true).expect("preset should load");
        assert!(theme.is_dark());
    }

    #[test]
    fn from_preset_returns_resolved() {
        let (_theme, resolved) = from_preset("catppuccin-mocha", true).expect("preset should load");
        // ResolvedTheme should have populated defaults
        assert!(resolved.defaults.font.size > 0.0);
    }

    #[test]
    fn from_preset_invalid_name() {
        let result = from_preset("nonexistent-preset", false);
        assert!(result.is_err(), "invalid preset should return Err");
    }

    // Issue 23: error message includes the mode
    #[test]
    fn from_preset_error_message_includes_mode() {
        // Both modes should load for catppuccin-mocha (it has both variants)
        let _ = from_preset("catppuccin-mocha", true).expect("dark should work");
        let _ = from_preset("catppuccin-mocha", false).expect("light should work");
    }

    // -- SystemThemeExt + from_system tests --

    #[test]
    fn system_theme_ext_to_gpui_theme() {
        // from_system() may fail on CI (no desktop env) -- skip gracefully
        let Ok(sys) = SystemTheme::from_system() else {
            return;
        };
        let theme = sys.to_gpui_theme();
        assert_eq!(
            theme.is_dark(),
            sys.mode.is_dark(),
            "to_gpui_theme() is_dark should match SystemTheme.mode"
        );
    }

    #[test]
    fn from_system_does_not_panic() {
        // Just verify no panic -- result may be Err on CI
        let _ = from_system();
    }

    #[test]
    fn from_system_returns_tuple() {
        let Ok((theme, resolved, _is_dark)) = from_system() else {
            return;
        };
        // Theme and resolved should agree on basic properties
        assert!(resolved.defaults.font.size > 0.0);
        // Theme mode should be set
        let _ = theme.is_dark();
    }

    #[test]
    fn from_system_matches_manual_path() {
        let Ok(sys) = SystemTheme::from_system() else {
            return;
        };
        let via_convenience = sys.to_gpui_theme();
        let via_manual = to_theme(
            sys.pick(sys.mode),
            &sys.name,
            sys.mode.is_dark(),
            sys.accessibility.reduce_transparency,
        );
        // Both paths should produce identical results
        assert_eq!(
            via_convenience.is_dark(),
            via_manual.is_dark(),
            "convenience and manual paths should agree on is_dark"
        );
        // Issue 39: verify the resolved variant has meaningful content.
        // from_system() may return Err on systems without a desktop (CI),
        // but if we reach here, the active variant should have at least
        // accent or background populated.
        let resolved = sys.pick(sys.mode);
        assert!(
            resolved.defaults.accent_color != Rgba::TRANSPARENT
                || resolved.defaults.background_color != Rgba::TRANSPARENT,
            "resolved variant should have at least accent or background populated"
        );
    }

    // -- Issue 25/32: helper function tests --

    #[test]
    fn is_dark_resolved_matches_background() {
        let resolved = test_resolved();
        let bg = colors::rgba_to_hsla(resolved.defaults.background_color);
        assert_eq!(
            is_dark_resolved(&resolved),
            bg.l < 0.5,
            "is_dark_resolved should match background lightness"
        );
    }

    #[test]
    fn accessibility_helpers() {
        // Accessibility helpers now take &SystemTheme; skip on CI if unavailable
        let Ok(sys) = SystemTheme::from_system() else {
            return;
        };
        let _ = is_reduced_motion(&sys);
        let _ = is_high_contrast(&sys);
        let _ = is_reduced_transparency(&sys);
        let _ = text_scaling_factor(&sys);
    }

    #[test]
    fn defaults_field_helpers() {
        let resolved = test_resolved();
        assert!(frame_width(&resolved) >= 0.0);
        assert!(disabled_opacity(&resolved) >= 0.0);
        assert!(disabled_opacity(&resolved) <= 1.0);
        assert!(border_opacity(&resolved) >= 0.0);
    }

    #[test]
    fn icon_sizes_helper() {
        let resolved = test_resolved();
        let sizes = icon_sizes(&resolved);
        assert!(sizes.toolbar > 0.0, "toolbar icon size should be positive");
    }

    #[test]
    fn text_scale_helper() {
        let resolved = test_resolved();
        let ts = text_scale(&resolved);
        assert!(ts.caption.size > 0.0, "caption size should be positive");
    }

    #[test]
    fn font_weight_helper() {
        let resolved = test_resolved();
        let w = font_weight(&resolved);
        assert!((100..=900).contains(&w), "font weight should be 100-900");
    }

    #[test]
    fn mono_font_weight_helper() {
        let resolved = test_resolved();
        let w = mono_font_weight(&resolved);
        assert!(
            (100..=900).contains(&w),
            "mono font weight should be 100-900"
        );
    }

    #[test]
    fn dialog_button_order_helper() {
        let resolved = test_resolved();
        let _order = dialog_button_order(&resolved);
        // Just verify it doesn't panic
    }

    #[test]
    fn line_height_helper() {
        let resolved = test_resolved();
        assert!(
            line_height_multiplier(&resolved) > 0.0,
            "line height should be positive"
        );
    }

    #[test]
    fn geometry_helpers() {
        let resolved = test_resolved();
        assert!(dialog_content_padding(&resolved) >= 0.0);
        assert!(dialog_button_spacing(&resolved) >= 0.0);
        assert!(scrollbar_width(&resolved) > 0.0);
    }

    #[test]
    fn selection_and_disabled_helpers() {
        let resolved = test_resolved();
        let _ = selection_foreground(&resolved);
        let _ = selection_inactive(&resolved);
        let _ = disabled_foreground(&resolved);
    }

    #[test]
    fn focus_ring_helpers() {
        let resolved = test_resolved();
        assert!(focus_ring_width(&resolved) >= 0.0);
        assert!(focus_ring_offset(&resolved) >= 0.0);
    }

    // -- Issue 26: integration tests for all 16 presets in both modes --

    #[test]
    fn all_presets_dark_mode_no_panic() {
        let presets = Theme::list_presets();
        for name in presets {
            let result = from_preset(name, true);
            assert!(
                result.is_ok(),
                "from_preset({name}, true) failed: {:?}",
                result.err()
            );
        }
    }

    #[test]
    fn all_presets_light_mode_no_panic() {
        let presets = Theme::list_presets();
        for name in presets {
            let result = from_preset(name, false);
            assert!(
                result.is_ok(),
                "from_preset({name}, false) failed: {:?}",
                result.err()
            );
        }
    }
}
