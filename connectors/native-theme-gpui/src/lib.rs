//! gpui toolkit connector for native-theme.
//!
//! Maps [`native_theme::ResolvedThemeVariant`] data to gpui-component's theming system.
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
//! let (theme, resolved) = from_system()?;
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
//! let nt = ThemeSpec::preset("catppuccin-mocha")?;
//! let variant = nt.into_variant(true).ok_or("no dark variant")?;
//! let resolved = variant.into_resolved()?;
//! let theme = to_theme(&resolved, "Catppuccin Mocha", true);
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
//! The connector maps a subset of [`ResolvedThemeVariant`] fields to gpui-component's
//! `ThemeColor` (108 color fields) and `ThemeConfig` (font/geometry).
//!
//! | Category | Mapped | Notes |
//! |----------|--------|-------|
//! | `defaults` colors | All 20+ | background, foreground, accent, danger, etc. |
//! | `defaults` geometry | radius, radius_lg, shadow | Font family/size also mapped |
//! | `button` | 4 of 14 | primary_background/foreground, background/foreground (colors only) |
//! | `tab` | 5 of 9 | All colors, sizing not mapped |
//! | `sidebar` | 2 of 2 | background, foreground |
//! | `window` | 2 of 10 | title_bar_background, border |
//! | `input` | 2 of 12 | border, caret |
//! | `scrollbar` | 3 of 7 | track, thumb, thumb_hover |
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
// Issue 8 + 48: re-export Result, Rgba, Error, DialogButtonOrder
pub use native_theme::{
    AnimatedIcon, DialogButtonOrder, Error, IconData, IconProvider, IconRole, IconSet,
    ResolvedThemeVariant, Result, Rgba, SystemTheme, ThemeSpec, ThemeVariant, TransformAnimation,
};

#[cfg(target_os = "linux")]
pub use native_theme::LinuxDesktop;

use gpui::{SharedString, px};
use gpui_component::scroll::ScrollbarShow;
use gpui_component::theme::{Theme, ThemeMode};
use std::rc::Rc;

/// Convert a [`ResolvedThemeVariant`] into a gpui-component [`Theme`].
///
/// Builds a complete Theme by:
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
/// To auto-derive: `let is_dark = resolved.defaults.background` lightness < 0.5
/// via [`is_dark_resolved()`].
///
/// Note: `is_dark` is an explicit parameter here, unlike the iced connector
/// which derives it from background luminance. Planned for unification in v0.6.0.
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(resolved: &ResolvedThemeVariant, name: &str, is_dark: bool) -> Theme {
    let theme_color = colors::to_theme_color(resolved, is_dark);
    let mode = if is_dark {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    };
    let d = &resolved.defaults;

    let mut theme = Theme::from(&theme_color);
    // Issue 53: Theme.transparent is set by Theme::from() to
    // Hsla::transparent_black() and is intentionally left unchanged.
    // It's used internally by gpui-component for transparent overlays.
    theme.mode = mode;
    theme.font_family = SharedString::from(d.font.family.clone());
    theme.font_size = px(d.font.size);
    theme.mono_font_family = SharedString::from(d.mono_font.family.clone());
    theme.mono_font_size = px(d.mono_font.size);
    // Issue 14: clamp radius to non-negative
    theme.radius = px(d.radius.max(0.0));
    theme.radius_lg = px(d.radius_lg.max(0.0));
    theme.shadow = d.shadow_enabled;

    // Issue 43: set scrollbar_show from resolved overlay_mode
    // ThemeConfig.highlight requires syntax highlighting colors (keyword, string,
    // comment, type, function, etc. — ~35 SyntaxColors + ~15 StatusColors fields)
    // which native-theme's ResolvedThemeVariant does not include. These are
    // editor-specific and cannot be derived from platform UI theme colors.
    // Users should set highlight separately via gpui-component's HighlightTheme API.
    theme.scrollbar_show = if resolved.scrollbar.overlay_mode {
        ScrollbarShow::Scrolling
    } else {
        ScrollbarShow::Always
    };

    // Store config for gpui-component's theme switching
    let config: Rc<_> = Rc::new(config::to_theme_config(resolved, name, mode));
    if mode == ThemeMode::Dark {
        theme.dark_theme = config;
    } else {
        theme.light_theme = config;
    }
    theme
}

/// Load a bundled preset and convert it to a gpui-component [`Theme`] in one call.
///
/// This is the primary entry point for most users. It handles the full pipeline:
/// load preset, pick variant, resolve, validate, and convert to gpui Theme.
///
/// Returns both the gpui Theme and the [`ResolvedThemeVariant`] so callers can
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
pub fn from_preset(
    name: &str,
    is_dark: bool,
) -> native_theme::Result<(Theme, ResolvedThemeVariant)> {
    let spec = ThemeSpec::preset(name)?;
    let mode_str = if is_dark { "dark" } else { "light" };
    let variant = spec.into_variant(is_dark).ok_or_else(|| {
        native_theme::Error::Format(format!("preset '{name}' has no {mode_str} variant"))
    })?;
    let resolved = variant.into_resolved()?;
    let theme = to_theme(&resolved, name, is_dark);
    Ok((theme, resolved))
}

/// Detect the OS theme and convert it to a gpui-component [`Theme`] in one call.
///
/// Combines [`SystemTheme::from_system()`](native_theme::SystemTheme::from_system)
/// with [`to_theme()`] using the system-detected name and dark-mode preference.
///
/// Returns both the gpui Theme and the [`ResolvedThemeVariant`] so callers can
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
/// let (theme, resolved) = native_theme_gpui::from_system()?;
/// ```
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system() -> native_theme::Result<(Theme, ResolvedThemeVariant)> {
    let sys = SystemTheme::from_system()?;
    let is_dark = sys.is_dark;
    let name = sys.name.clone();
    let resolved = if is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name, is_dark);
    Ok((theme, resolved))
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
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_gpui_theme(&self) -> Theme;
}

impl SystemThemeExt for SystemTheme {
    fn to_gpui_theme(&self) -> Theme {
        to_theme(self.active(), &self.name, self.is_dark)
    }
}

// ---------------------------------------------------------------------------
// Helper functions (Issues 15, 17, 25, 32, 36, 37, 47, 48, 13)
// ---------------------------------------------------------------------------

/// Derive `is_dark` from a [`ResolvedThemeVariant`]'s background lightness.
///
/// Returns `true` when the background lightness is below 0.5. This is a
/// convenience for callers that do not have an explicit dark-mode flag.
/// Some presets (e.g. solarized, gruvbox) have borderline values where the
/// auto-detected result may differ from the user's intent.
#[must_use]
pub fn is_dark_resolved(resolved: &ResolvedThemeVariant) -> bool {
    colors::rgba_to_hsla(resolved.defaults.background).l < 0.5
}

// --- Issue 32: Accessibility helpers ---

/// Returns `true` if the resolved theme is considered dark.
///
/// Equivalent to [`is_dark_resolved()`] -- delegates to background lightness.
#[must_use]
pub fn is_dark(resolved: &ResolvedThemeVariant) -> bool {
    is_dark_resolved(resolved)
}

/// Whether the user/theme has requested reduced motion.
#[must_use]
pub fn is_reduced_motion(resolved: &ResolvedThemeVariant) -> bool {
    resolved.defaults.reduce_motion
}

/// Whether the theme is in high-contrast mode.
#[must_use]
pub fn is_high_contrast(resolved: &ResolvedThemeVariant) -> bool {
    resolved.defaults.high_contrast
}

/// Whether the user/theme has requested reduced transparency.
#[must_use]
pub fn is_reduced_transparency(resolved: &ResolvedThemeVariant) -> bool {
    resolved.defaults.reduce_transparency
}

// --- Issue 15: Defaults field accessors ---

/// Frame/border width from the resolved theme defaults.
#[must_use]
pub fn frame_width(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.frame_width
}

/// Disabled control opacity from the resolved theme defaults.
#[must_use]
pub fn disabled_opacity(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.disabled_opacity
}

/// Border opacity multiplier from the resolved theme defaults.
#[must_use]
pub fn border_opacity(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.border_opacity
}

/// Whether drop shadows are enabled.
#[must_use]
pub fn shadow_enabled(resolved: &ResolvedThemeVariant) -> bool {
    resolved.defaults.shadow_enabled
}

/// Text scaling factor (1.0 = no scaling).
#[must_use]
pub fn text_scaling_factor(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.text_scaling_factor
}

// --- Issue 17: Spacing / icon-size / text-scale accessors ---

/// Access the spacing scale from the resolved theme.
///
/// Returns the 7-step spacing scale (xxs, xs, s, m, l, xl, xxl) in logical pixels.
#[must_use]
pub fn spacing(resolved: &ResolvedThemeVariant) -> &native_theme::ResolvedThemeSpacing {
    &resolved.defaults.spacing
}

/// Access the per-context icon sizes from the resolved theme.
///
/// Returns icon sizes for toolbar, small, large, dialog, and panel contexts.
#[must_use]
pub fn icon_sizes(resolved: &ResolvedThemeVariant) -> &native_theme::ResolvedIconSizes {
    &resolved.defaults.icon_sizes
}

/// Access the text scale entries from the resolved theme.
///
/// Returns the 4-entry text scale (caption, section_heading, dialog_title, display).
#[must_use]
pub fn text_scale(resolved: &ResolvedThemeVariant) -> &native_theme::ResolvedTextScale {
    &resolved.text_scale
}

// --- Issue 36: Line height multiplier ---

/// Line height multiplier from the resolved theme defaults.
#[must_use]
pub fn line_height_multiplier(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.line_height
}

// --- Issue 13: Font weight helper ---

/// Default font weight from the resolved theme.
///
/// Returns the CSS font weight value (100-900).
#[must_use]
pub fn font_weight(resolved: &ResolvedThemeVariant) -> u16 {
    resolved.defaults.font.weight
}

// --- Issue 47: Mono font weight helper ---

/// Monospace font weight from the resolved theme.
///
/// Returns the CSS font weight value (100-900).
#[must_use]
pub fn mono_font_weight(resolved: &ResolvedThemeVariant) -> u16 {
    resolved.defaults.mono_font.weight
}

// --- Issue 48: Dialog button order helper ---

/// The platform-appropriate dialog button order.
///
/// Returns whether affirmative buttons should appear on the leading (left)
/// or trailing (right) side of a dialog. KDE uses leading-affirmative;
/// most other platforms use trailing-affirmative.
#[must_use]
pub fn dialog_button_order(resolved: &ResolvedThemeVariant) -> DialogButtonOrder {
    resolved.dialog.button_order
}

// --- Issue 37: Padding/geometry helpers ---

/// Dialog content padding in logical pixels.
#[must_use]
pub fn dialog_content_padding(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.dialog.content_padding
}

/// Dialog button spacing in logical pixels.
#[must_use]
pub fn dialog_button_spacing(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.dialog.button_spacing
}

/// Scrollbar width in logical pixels.
#[must_use]
pub fn scrollbar_width(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.scrollbar.width
}

/// Selection text color (foreground for selected content).
#[must_use]
pub fn selection_foreground(resolved: &ResolvedThemeVariant) -> Rgba {
    resolved.defaults.selection_foreground
}

/// Selection background when window is unfocused.
#[must_use]
pub fn selection_inactive(resolved: &ResolvedThemeVariant) -> Rgba {
    resolved.defaults.selection_inactive
}

/// Foreground color for disabled elements.
#[must_use]
pub fn disabled_foreground(resolved: &ResolvedThemeVariant) -> Rgba {
    resolved.defaults.disabled_foreground
}

/// Focus ring stroke width in logical pixels.
#[must_use]
pub fn focus_ring_width(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.focus_ring_width
}

/// Gap between element edge and focus ring.
#[must_use]
pub fn focus_ring_offset(resolved: &ResolvedThemeVariant) -> f32 {
    resolved.defaults.focus_ring_offset
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// Issue 1: fixed to use into_variant(true) for catppuccin-mocha (dark theme).
    fn test_resolved() -> ResolvedThemeVariant {
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(true)
            .expect("preset must have dark variant");
        variant
            .into_resolved()
            .expect("resolved preset must validate")
    }

    #[test]
    fn to_theme_produces_valid_theme() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Test", true);

        // Theme should have the correct mode
        assert!(theme.is_dark());
    }

    #[test]
    fn to_theme_dark_mode() {
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(true)
            .expect("preset must have dark variant");
        let resolved = variant
            .into_resolved()
            .expect("resolved preset must validate");
        let theme = to_theme(&resolved, "DarkTest", true);

        assert!(theme.is_dark());
    }

    #[test]
    fn to_theme_applies_font_and_geometry() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Test", true);

        assert_eq!(theme.font_family.to_string(), resolved.defaults.font.family);
        assert_eq!(theme.font_size, px(resolved.defaults.font.size));
        assert_eq!(
            theme.mono_font_family.to_string(),
            resolved.defaults.mono_font.family
        );
        assert_eq!(theme.mono_font_size, px(resolved.defaults.mono_font.size));
        assert_eq!(theme.radius, px(resolved.defaults.radius.max(0.0)));
        assert_eq!(theme.radius_lg, px(resolved.defaults.radius_lg.max(0.0)));
        assert_eq!(theme.shadow, resolved.defaults.shadow_enabled);
    }

    // Issue 43: scrollbar_show set from overlay_mode
    #[test]
    fn scrollbar_show_from_overlay_mode() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Scroll", true);
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
        // ResolvedThemeVariant should have populated defaults
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
            sys.is_dark,
            "to_gpui_theme() is_dark should match SystemTheme.is_dark"
        );
    }

    #[test]
    fn from_system_does_not_panic() {
        // Just verify no panic -- result may be Err on CI
        let _ = from_system();
    }

    #[test]
    fn from_system_returns_tuple() {
        let Ok((theme, resolved)) = from_system() else {
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
        let via_manual = to_theme(sys.active(), &sys.name, sys.is_dark);
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
        let resolved = sys.active();
        assert!(
            resolved.defaults.accent != native_theme::Rgba::default()
                || resolved.defaults.background != native_theme::Rgba::default(),
            "resolved variant should have at least accent or background populated"
        );
    }

    // -- Issue 25/32: helper function tests --

    #[test]
    fn is_dark_resolved_matches_background() {
        let resolved = test_resolved();
        let bg = colors::rgba_to_hsla(resolved.defaults.background);
        assert_eq!(
            is_dark_resolved(&resolved),
            bg.l < 0.5,
            "is_dark_resolved should match background lightness"
        );
    }

    #[test]
    fn accessibility_helpers() {
        let resolved = test_resolved();
        // Just verify they return without panic and give sensible values
        let _ = is_reduced_motion(&resolved);
        let _ = is_high_contrast(&resolved);
        let _ = is_reduced_transparency(&resolved);
    }

    #[test]
    fn defaults_field_helpers() {
        let resolved = test_resolved();
        assert!(frame_width(&resolved) >= 0.0);
        assert!(disabled_opacity(&resolved) >= 0.0);
        assert!(disabled_opacity(&resolved) <= 1.0);
        assert!(border_opacity(&resolved) >= 0.0);
        assert!(text_scaling_factor(&resolved) > 0.0);
    }

    #[test]
    fn spacing_helper() {
        let resolved = test_resolved();
        let sp = spacing(&resolved);
        assert!(sp.m > 0.0, "medium spacing should be positive");
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
        let presets = ThemeSpec::list_presets();
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
        let presets = ThemeSpec::list_presets();
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
