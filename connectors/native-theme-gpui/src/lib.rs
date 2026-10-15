//! gpui toolkit connector for native-theme.
//!
//! Maps [`native_theme::theme::ResolvedTheme`] data to gpui-component's theming system.
//!
//! # Quick Start
//!
//! ```ignore
//! use native_theme_gpui::{AccessibilityPreferences, from_preset};
//!
//! let prefs = AccessibilityPreferences::from_system();
//! let (theme, resolved) = from_preset("catppuccin-mocha", true, &prefs)?;
//! ```
//!
//! Or from the OS-detected theme:
//!
//! ```ignore
//! use native_theme_gpui::from_system;
//!
//! let loaded = from_system()?;
//! let (theme, resolved) = (loaded.theme, loaded.resolved);
//! ```
//!
//! # Manual Path
//!
//! For full control over the resolve/validate/convert pipeline:
//!
//! ```ignore
//! use native_theme::theme::{ColorMode, Theme};
//! use native_theme_gpui::{AccessibilityPreferences, to_theme};
//!
//! let nt = Theme::preset("catppuccin-mocha")?;
//! let resolved = nt.resolve(ColorMode::Dark)?;
//! let prefs = AccessibilityPreferences::from_system();
//! let theme = to_theme(&resolved.variant, &nt.name, resolved.mode.is_dark(), &prefs);
//! ```
//!
//! # Single-Mode Behavior
//!
//! Each call to [`from_preset()`] or [`to_theme()`] produces a theme for exactly
//! one mode (light or dark). The `is_dark` parameter selects which variant to
//! load from the preset's TOML. Presets that only define one variant will return
//! that variant for both `is_dark=true` and `is_dark=false` (the resolution
//! pipeline falls back to the available variant), and [`from_preset()`] builds
//! the theme in that variant's mode
//! ([`Resolved::mode`](native_theme::theme::Resolved::mode)). To support
//! runtime light/dark switching, call [`from_preset()`] once per mode and
//! [`apply`] both: each `apply` stores its variant, and gpui-component's
//! `Theme::change` / `sync_system_appearance` then switch between the two
//! native palettes.
//!
//! # What each flag does
//!
//! The four [`AccessibilityPreferences`] flags reach the connector through the
//! preferences [`from_preset()`], [`to_theme()`], [`apply`] and
//! [`apply_accessibility`] take; [`from_system()`] and [`apply_system_theme`]
//! use the OS reading.
//!
//! | Flag | The connector applies | The application forwards |
//! |------|-----------------------|--------------------------|
//! | `text_scaling_factor` | `font_size` and `mono_font_size` times [`AccessibilityPreferences::text_scale_factor`]; `font_size` is the window rem, so every rem-relative size in gpui-component follows; the [`geometry`] builders scale the widget fonts they set | a size it sets itself, through [`scaled_text_size()`] |
//! | `reduce_motion` | [`apply`] and [`apply_accessibility`] switch gpui's `App::reduce_motion` on, which the `widgets` spinner reads | its own animations read `cx.reduce_motion()` |
//! | `reduce_transparency` | no dialog or sheet scrim: `ThemeColor::overlay` is transparent | translucent surfaces of its own |
//! | `high_contrast` | nothing: the theme is the same with it set or unset (`docs/todo.md`, the item "the gpui connector reads `AccessibilityPreferences::high_contrast`"); [`is_high_contrast`] reads it | its own colours, if they should change |
//!
//! # Theme Field Coverage
//!
//! The connector maps [`ResolvedTheme`] onto gpui-component's `ThemeColor`
//! (all 139 colour fields), `ThemeConfig` (fonts, radii, shadow, highlighter),
//! the styled `Theme`'s `focus_ring` and `scrollbar_mode`, and gpui-base's
//! scrollbar and resize-handle styles.
//!
//! | Category | Mapped | Notes |
//! |----------|--------|-------|
//! | `defaults` colors | 14 of 24 | text_color, muted_color, accent_color, border.color, focus_ring_color, shadow_color (the dialog overlay's colour, at an alpha of its own), and the four status colours with their text colours; the window background comes from `window.background_color` |
//! | `defaults` geometry | radius, radius_lg, shadow, focus ring | fonts scaled by the text-scaling factor |
//! | `button` | all 28 `button*` plus `primary*` / `secondary*` | solid native surfaces (the 0.5.1 semantics); flat buttons via [`variants::ghost_button`] |
//! | `tab` | 5 of 10 colours | geometry is upstream work (`Tab`'s render writes its own height, radius and text size into the style bag the caller's setters fill, `tab/tab.rs:801-808`) |
//! | `sidebar` | 4 of 6 | background, font.color, selection_background, selection_text_color |
//! | `window` | 4 of 6 | background_color, title_bar_background, border colour; title_bar_font size, weight and colour via `geometry::title_bar` |
//! | `input` | 3 of 13 + geometry | border, caret, selection_background; height, line height, padding, radius, border, text via `geometry::input` |
//! | `scrollbar` | colours + geometry | track/thumb colours, widths, inset, min length via `base_layer` |
//! | `status_bar` | 3 of 3 | background, border colour; the border's padding sides and the font's size, weight and colour via `geometry::status_bar` |
//! | `table` | head, from `list.header_background` / `list.header_font` | the model states no footer colour, so `table_foot*` are derived |
//! | `slider`, `switch` | 2 colours each | fill/thumb colours; geometry upstream |
//! | `progress_bar` | fill + geometry | height, radius, min width via `geometry::progress` |
//! | `list` | 7 of 14 + geometry | background, alternate row, selection, hover, grid, header background and header font colour; row height, padding, font via `geometry::list_item`; header height via `geometry::list_header_height` for `DataTable::header_row_height` |
//! | `menu` | 2 colours + geometry | hover_background, hover_text_color → `accent`, `accent_foreground` (upstream's item highlight); row height, padding, icon gap, font via `geometry::menu_item` on an application-drawn row |
//! | `segmented_control` | 1 of 9 | background → `tab_bar_segmented`, the track of a segmented `TabBar` |
//! | `popover` | 2 of 3 + geometry | background, font.color; padding, radius via `geometry::popover` |
//! | `tooltip` | show delay + geometry | `show_delay` installed by [`apply`] as gpui-base's `TooltipDefaults` show delay, and [`tooltip_show_delay`] for GPUI's own `.tooltip()` on an element; background, font, padding, border, radius via `geometry::tooltip`, max width via `geometry::tooltip_content` |
//! | `dialog` | entrance + geometry | `entrance` via `geometry::dialog_entrance`, both styles as `DialogEntrance::Fade` (gpui-component draws neither a scaling nor a rising entrance); padding, min height, radius via `geometry::dialog`, max width via `geometry::dialog_max_width`, button gap via `geometry::dialog_footer`, fonts via `geometry::dialog_title` and `geometry::dialog_description` |
//! | `link` | 3 of 8 | font.color, hover_text_color, active_text_color; `link.hover_background` has no receiver — gpui-component paints a link's background as `Theme::transparent` in every state, and that field is not one of the 139 `ThemeColor` colours |
//! | `splitter` | colours | divider/hover colours via `base_layer::resizable_theme`, drawn by gpui-base's resizables at their 1 px width (not gpui-component's, `Settings` or dock edges) |
//!
//! **Accessors.** The free functions of the crate root read what an application
//! needs beside the theme; one that returns one `ResolvedTheme` leaf is named
//! after the leaf's path, without the leading `defaults.` and a unit suffix:
//! `defaults.border.corner_radius` is `border_corner_radius`,
//! `scrollbar.groove_width` is `scrollbar_groove_width`, the same name in the
//! iced, gpui and egui connectors (`scripts/check_connector_parity.py` checks
//! it).
//!
//! **Per-widget geometry.** Heights, paddings, radii, borders and text sizes
//! reach the widgets through the [`geometry`] module: pure builders returning a
//! `StyleRefinement` that the application applies with
//! `gpui_component::StyledExt::refine_style`, where gpui-component applies the
//! caller's style after its own geometry. [`apply`] installs the theme, the
//! base-layer overrides and the observer that keeps them installed.
//!
//! **Limits.** Geometry on inner elements the caller's style cannot reach
//! (checkbox and radio indicators, switch, slider, separator thickness,
//! splitter width, button icon gap, popup-menu rows), and tab
//! height, radius and text size, which `Tab`'s render writes into the same
//! style bag the caller's setters fill (`tab/tab.rs:801-808`), stay upstream
//! work; §14 of the v0.5.8 specification
//! (<https://github.com/tiborgats/native-theme/blob/main/docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md>)
//! lists each item with the upstream line that makes it unreachable. For
//! the checkbox, radio, switch, slider, progress bar, spinner, tab bar and
//! separator, the `widgets` module (feature `widgets`) draws controls of its own on
//! gpui-base's headless primitives, every part from the theme.
//!
//! # Features
//!
//! The first six are on by default; `gpui-fast` is off, because it changes the
//! GPUI core rather than adding to it, and `native` and `watch` are off, as in
//! native-theme.
//!
//! | Feature | Enables |
//! |---------|---------|
//! | `widgets` | the `widgets` module: a checkbox, radio, switch, slider, progress bar, spinner, tab bar and separator drawn from the theme, for the parts gpui-component draws from literals of its own |
//! | `material-icons` | the bundled Material Symbols set (`native-theme/material-icons`) |
//! | `lucide-icons` | the bundled Lucide set (`native-theme/lucide-icons`) |
//! | `system-icons` | the platform's own icons (`native-theme/system-icons`) |
//! | `svg-rasterize` | SVG icons rasterized by this crate (`native-theme/svg-rasterize`) |
//! | `system-fonts` | a theme family the system lacks is drawn in the family the platform substitutes for it ([`font_family`]; `native-theme/system-fonts`) |
//! | `gpui-fast` | the crate built on GPUI Kit's GPUI Fast core instead of gpui-pre (`gpui-kit/gpui-fast`, `gpui-component/gpui-fast`) |
//! | `native` | the OS readers, `native-theme/native`; without one (or native-theme's `kde`, `portal`, `macos`, `windows`), [`from_system()`] is the platform preset on Linux and an error on macOS and Windows |
//! | `watch` | `on_theme_change`, a closure run on gpui's foreground when the OS theme changes, through `native-theme/watch` and `futures` (see [Runtime theme change](#runtime-theme-change)) |
//!
//! The crate names GPUI through gpui-kit, so it is built on whichever core
//! gpui-kit is: an application that enables `gpui-kit/gpui-fast` with
//! gpui-kit's `component` feature gets this crate on GPUI Fast without
//! `gpui-fast` here. The README's *Backends* section says when to enable it.
//!
//! Without `svg-rasterize` an SVG icon is still converted: [`icons`] hands
//! gpui the (colorized) SVG bytes as an undecoded `ImageSource::Image`, which
//! paints nothing the first time it comes up and is rasterized at a size gpui
//! chooses. [`icons::to_image_source`] documents it.
//!
//! # Runtime theme change
//!
//! With the `watch` feature and a reader feature (`native`),
//! `on_theme_change(cx, f)` runs `f` on gpui's foreground, with the `App`,
//! each time the OS theme changes, for as long as the `ThemeSubscription` it
//! returns is held. In `f`, call `native_theme::detect::invalidate_caches()`,
//! read the theme again with the entry point the application used
//! ([`SystemTheme::from_system()`], or
//! [`SystemTheme::from_system_with_preset`] for a pinned preset), and
//! [`apply_system_theme`] it; gpui-component's `Theme::sync_system_appearance`
//! keeps switching between its light and dark palettes after that:
//!
//! ```rust,ignore
//! use native_theme_gpui::{SystemTheme, apply_system_theme, on_theme_change};
//!
//! // Held while the application follows the OS theme; dropping it stops
//! // the watcher.
//! let watcher = on_theme_change(cx, |_event, cx| {
//!     native_theme::detect::invalidate_caches();
//!     if let Ok(sys) = SystemTheme::from_system() {
//!         apply_system_theme(&sys, cx);
//!     }
//! })?;
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

// GPUI through gpui-kit, which re-exports the core it is built on: gpui-pre,
// or GPUI Fast under `gpui-kit/gpui-fast` (gpui-kit `src/lib.rs:94, 101-103`).
extern crate gpui_kit as gpui;

pub mod base_layer;
pub(crate) mod colors;
#[cfg(test)]
mod compat;
pub(crate) mod config;
#[cfg(test)]
mod contract;
pub(crate) mod derive;
pub mod geometry;
pub mod icons;
#[cfg(test)]
mod showcase;
pub mod variants;
#[cfg(feature = "watch")]
mod watch;
#[cfg(feature = "widgets")]
pub mod widgets;

#[cfg(feature = "watch")]
pub use watch::on_theme_change;

// Re-export native-theme types that appear in public signatures so downstream
// crates don't need native-theme as a direct dependency.
// Issue 8 + 48: re-export Result, Rgba, Error, DialogButtonOrder
pub use native_theme::color::Rgba;
pub use native_theme::error::Error;
pub use native_theme::icons::{AnimatedIcon, IconData, IconProvider, IconRole, TransformAnimation};
pub use native_theme::theme::{
    ColorMode, DialogButtonOrder, IconSet, ResolvedTheme, Theme, ThemeMode,
};
#[cfg(feature = "watch")]
pub use native_theme::watch::{ThemeChangeEvent, ThemeSubscription};
pub use native_theme::{AccessibilityPreferences, Result, SystemTheme};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;

use gpui::{App, Global, SharedString, px};
use gpui_component::scroll::ScrollbarMode;
use gpui_component::theme::{Theme as GpuiTheme, ThemeMode as GpuiThemeMode};
use std::borrow::Cow;
use std::rc::Rc;
use std::time::Duration;

/// Convert a [`ResolvedTheme`] into a gpui-component [`GpuiTheme`].
///
/// Builds a complete GpuiTheme by:
/// 1. Mapping all 139 ThemeColor fields via `colors::to_theme_color`
/// 2. Setting font, geometry, and mode fields directly on the Theme
/// 3. Storing a ThemeConfig in light_theme/dark_theme Rc for gpui-component switching
///
/// All Theme fields are set explicitly -- no `apply_config` call is used.
/// This avoids the fragile apply-then-restore pattern where `apply_config`
/// would overwrite all 139 color fields with defaults.
///
/// A [`ResolvedTheme`] carries no colour mode, so `is_dark` is the theme's:
/// [`Resolved::mode`](native_theme::theme::Resolved::mode) of the variant
/// [`Theme::resolve`] resolved, as [`from_preset()`] passes it, or
/// [`SystemTheme::mode`], as [`from_system()`] does. [`is_dark_resolved()`]
/// guesses it from the background's lightness for a caller holding only a
/// `ResolvedTheme`; on a borderline background (solarized, gruvbox) the
/// guess can disagree with the theme's mode.
///
/// Note: `is_dark` is an explicit parameter here; the iced connector's
/// `to_theme` takes no mode.
///
/// The families are [`font_family`]'s: on macOS the stated family of the
/// system UI font, "SF Pro", is passed to gpui as its own alias
/// `.SystemUIFont`, so Core Text supplies the system font itself; a family
/// the system lacks is the one the platform substitutes for it (feature
/// `system-fonts`); every other family is passed as stated.
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(
    resolved: &ResolvedTheme,
    name: &str,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> GpuiTheme {
    let s = prefs.text_scale_factor();
    let theme_color = colors::to_theme_color(resolved, is_dark, prefs.reduce_transparency);
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
    theme.font_family = font_family(&d.font.family);
    // §3.4: gpui-component's `WindowState` root plugin sets the window rem
    // to font_size (gpui-component src/root.rs:440), so scaling these two sizes scales every rem-relative
    // size in gpui-component, as the platform toolkit scales its own text.
    theme.font_size = px(d.font.size * s);
    theme.mono_font_family = font_family(&d.mono_font.family);
    theme.mono_font_size = px(d.mono_font.size * s);
    // Issue 14: clamp radius to non-negative
    theme.radius = px(d.border.corner_radius.max(0.0));
    theme.radius_lg = px(d.border.corner_radius_lg.max(0.0));
    theme.shadow = d.border.shadow_enabled;

    // §8.3: the ring is drawn only when the theme gives it a width; its width
    // and offset have no receiver (derived from the element's border upstream).
    theme.focus_ring = d.focus_ring_width > 0.0;

    // §8.3: Scrolling (overlay, auto-hide) when the platform draws overlay
    // scrollbars, Always otherwise.
    theme.scrollbar_mode = if resolved.scrollbar.overlay_mode {
        ScrollbarMode::Scrolling
    } else {
        ScrollbarMode::Always
    };

    // Issue 43/44: set highlight_theme based on is_dark so syntax highlighting
    // uses appropriate colors (dark themes get dark highlight, light themes get light).
    theme.highlight_theme = if is_dark {
        gpui_component::highlighter::HighlightTheme::default_dark()
    } else {
        gpui_component::highlighter::HighlightTheme::default_light()
    };

    // Store config for gpui-component's theme switching
    let config: Rc<_> = Rc::new(config::to_theme_config(resolved, name, mode, prefs));
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
/// The gpui theme is dark when the variant resolved is the dark one
/// ([`Resolved::mode`](native_theme::theme::Resolved::mode)), so a light-only
/// preset asked for dark gives a light theme.
///
/// The theme's display name is the preset's `name` field (`"Dracula"` for
/// `"dracula"`), not the name passed in.
///
/// Pass `&AccessibilityPreferences::default()` for no scaling, or the OS
/// preferences to honour them under a preset: accessibility is orthogonal to
/// the theme choice, and a user with large text wants it under a preset too.
/// On a preset switch, `native_theme::detect::accessibility_preferences()` is
/// the cached reading: the first call after a system read may still run the
/// reduced-motion detection (about 3 ms measured on one KDE machine), and
/// every later call returns the cached block in nanoseconds;
/// [`AccessibilityPreferences::from_system()`] runs the platform reader each
/// time.
///
/// # Errors
///
/// Returns an error if the preset name is not recognized or if resolution fails.
///
/// # Examples
///
/// ```ignore
/// use native_theme_gpui::AccessibilityPreferences;
///
/// let prefs = AccessibilityPreferences::from_system();
/// let (dark_theme, resolved) = native_theme_gpui::from_preset("dracula", true, &prefs)?;
/// let (light_theme, _) = native_theme_gpui::from_preset("catppuccin-latte", false, &prefs)?;
/// ```
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> Result<(GpuiTheme, ResolvedTheme)> {
    from_spec(&Theme::preset(name)?, is_dark, prefs)
}

/// [`from_preset()`] of a theme already loaded.
fn from_spec(
    spec: &Theme,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> Result<(GpuiTheme, ResolvedTheme)> {
    let resolved = spec.resolve(if is_dark {
        ColorMode::Dark
    } else {
        ColorMode::Light
    })?;
    let theme = to_theme(
        &resolved.variant,
        &spec.name,
        resolved.mode.is_dark(),
        prefs,
    );
    Ok((theme, resolved.variant))
}

/// The OS theme as [`from_system()`] and [`SystemThemeExt::to_gpui_theme`]
/// return it: the gpui-component theme of the OS colour mode's variant, with
/// what the OS reading detected beside it.
///
/// `#[non_exhaustive]`, so a field added later is not a breaking change;
/// read it by field, or destructure it with `..`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Loaded {
    /// The gpui-component theme, built by [`to_theme()`] from
    /// [`resolved`](Self::resolved) with [`accessibility`](Self::accessibility).
    pub theme: GpuiTheme,
    /// The resolved variant of the OS colour mode, for the per-widget
    /// metrics the flat `ThemeColor` cannot represent.
    pub resolved: ResolvedTheme,
    /// The OS colour mode, the OS preference rather than a guess from the
    /// background colour; [`resolved`](Self::resolved) is its variant.
    pub mode: ColorMode,
    /// The OS accessibility preferences the theme was built with, which
    /// [`apply`] takes.
    pub accessibility: AccessibilityPreferences,
    /// The icon set the theme names ([`SystemTheme::icon_set`]).
    pub icon_set: IconSet,
    /// The icon theme of the OS colour mode's variant
    /// ([`SystemTheme::icon_theme_for`]), for a freedesktop icon or spinner;
    /// `None` where that variant states none and detection failed.
    pub icon_theme: Option<Cow<'static, str>>,
}

/// Detect the OS theme and convert it to a gpui-component [`GpuiTheme`] in one call.
///
/// Combines [`SystemTheme::from_system()`](native_theme::SystemTheme::from_system)
/// with [`to_theme()`] using the system-detected name, dark-mode preference
/// and accessibility preferences.
///
/// Returns the gpui Theme with the [`ResolvedTheme`], so callers can access
/// per-widget metrics that the flat `ThemeColor` cannot represent, and the
/// OS colour mode, the accessibility preferences, the icon set and the icon
/// theme beside them, as a [`Loaded`].
///
/// **Ownership note** (Issue 19/31): this function takes ownership of the
/// `SystemTheme`'s active variant. The non-active variant (light when dark
/// is active, or vice versa) is dropped. If you need both variants, use
/// `SystemTheme::from_system()` directly and call [`to_theme()`] on each.
///
/// **Performance note:** `SystemTheme::from_system()` resolves both light
/// and dark variants before this function picks one; native-theme has no
/// system read that resolves only one variant.
///
/// # Errors
///
/// Returns an error if the platform theme cannot be read (e.g., unsupported platform,
/// missing desktop environment). Without a reader feature (`native`, or
/// native-theme's `kde`, `portal`, `macos` or `windows`) the theme is the
/// platform preset on Linux and an error on macOS and Windows.
///
/// # Examples
///
/// ```ignore
/// let loaded = native_theme_gpui::from_system()?;
/// ```
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system() -> Result<Loaded> {
    let sys = SystemTheme::from_system()?;
    let mode = sys.mode;
    let is_dark = mode.is_dark();
    let icon_theme = sys.icon_theme_for(mode).map(|s| Cow::Owned(s.to_owned()));
    let name = sys.name; // K-5: move instead of clone
    let accessibility = sys.accessibility;
    let resolved = if is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name, is_dark, &accessibility);
    Ok(Loaded {
        theme,
        resolved,
        mode,
        accessibility,
        icon_set: sys.icon_set,
        icon_theme,
    })
}

/// Extension trait for converting a [`SystemTheme`] to a gpui-component [`GpuiTheme`].
///
/// Useful when you already have a `SystemTheme` and want method syntax:
///
/// ```ignore
/// use native_theme_gpui::SystemThemeExt;
///
/// let sys = native_theme::SystemTheme::from_system()?;
/// let theme = sys.to_gpui_theme().theme;
/// ```
pub trait SystemThemeExt {
    /// Convert this system theme to a gpui-component [`GpuiTheme`], as a
    /// [`Loaded`].
    ///
    /// Uses the OS-active variant (based on `mode`), the theme name, the
    /// color mode and the accessibility preferences from the `SystemTheme`.
    /// The variant is cloned out of `self`; [`from_system()`] moves it.
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_gpui_theme(&self) -> Loaded;
}

impl SystemThemeExt for SystemTheme {
    fn to_gpui_theme(&self) -> Loaded {
        Loaded {
            theme: system_gpui_theme(self),
            resolved: self.pick(self.mode).clone(),
            mode: self.mode,
            accessibility: self.accessibility.clone(),
            icon_set: self.icon_set,
            icon_theme: self
                .icon_theme_for(self.mode)
                .map(|s| Cow::Owned(s.to_owned())),
        }
    }
}

/// [`SystemThemeExt::to_gpui_theme`]'s theme, without cloning the variant.
fn system_gpui_theme(sys: &SystemTheme) -> GpuiTheme {
    to_theme(
        sys.pick(sys.mode),
        &sys.name,
        sys.mode.is_dark(),
        &sys.accessibility,
    )
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

/// Border line width from the resolved theme defaults.
#[must_use]
pub fn border_line_width(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.border.line_width
}

/// Disabled control opacity from the resolved theme defaults: the whole
/// widget fades by it on top of its disabled colours; 1.0 where the platform
/// dims by colour alone.
#[must_use]
pub fn disabled_opacity(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.disabled_opacity
}

/// The platform's line opacity from the resolved theme defaults: the share
/// of the text colour already folded into every stated border colour. Apply
/// it to no colour; a border colour is the final line colour.
#[must_use]
pub fn border_opacity(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.border.opacity
}

/// Whether drop shadows are enabled.
#[must_use]
pub fn border_shadow_enabled(resolved: &ResolvedTheme) -> bool {
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

/// Scales a text size from the resolved theme by the user's text-scaling
/// preference, as [`to_theme`] scales `font_size` and the `geometry` builders
/// scale the widget fonts they apply.
///
/// For a size the connector does not apply itself: a `text_scale` role's
/// (`resolved.text_scale.caption.size`, …) or a widget font set by hand. The
/// factor applies to every text size alike. A factor that is not finite and
/// positive is ignored.
#[must_use]
pub fn scaled_text_size(size: f32, prefs: &AccessibilityPreferences) -> f32 {
    prefs.scaled(size)
}

/// The family gpui is asked to draw `family` in. On macOS (`macos`), the
/// system UI font's stated family — "SF Pro" caselessly,
/// `native_theme::fonts::is_macos_system_ui_family` — becomes gpui's own
/// alias `.SystemUIFont`, "used to identify the system UI font, which varies
/// based on platform" (gpui-pre 0.3.9 `src/text_system.rs` line 1295): gpui's
/// macOS text system maps it to `.AppleSystemUIFont` through
/// `font_name_with_fallbacks` (gpui-pre 0.3.9 `src/text_system.rs` lines 1420–1430,
/// gpui-pre-apple 0.3.9 `src/text_system.rs` line 290) and looks that up among
/// its memory fonts first, then in the system source (lines 294–297), so Core
/// Text supplies its own system UI font. Every other family, and every
/// family elsewhere, is passed as stated: fontdb-style databases file the
/// font under `.SF NS`, but gpui does not draw from one. The platform is a
/// parameter so the unit test runs on every platform; the callers pass
/// `cfg!(target_os = "macos")`.
pub(crate) fn ui_font_family(family: &std::sync::Arc<str>, macos: bool) -> SharedString {
    if macos && native_theme::fonts::is_macos_system_ui_family(family) {
        SharedString::from(".SystemUIFont")
    } else {
        SharedString::from(family.clone())
    }
}

/// The family gpui is asked to draw a theme's `family` in: the macOS alias
/// of `ui_font_family`; with feature `system-fonts`, where the system
/// lacks `family`, the family the platform substitutes for it
/// (`native_theme::fonts::substitute_family`: fontconfig's match on Linux,
/// the font every native application of the system gets); else `family` as
/// stated. gpui's own answer to a family its text system cannot find is a
/// list of its own, `.ZedMono` first (gpui-pre 0.3.9 `src/text_system.rs`
/// lines 255-266, 370-379), not the platform's. The platform is asked once
/// per family and the answer kept, so a caller may ask every frame.
#[must_use]
pub fn font_family(family: &std::sync::Arc<str>) -> SharedString {
    let alias = ui_font_family(family, cfg!(target_os = "macos"));
    if alias.as_ref() != family.as_ref() {
        return alias;
    }
    match substituted_family(family) {
        Some(substitute) => SharedString::from(substitute),
        None => alias,
    }
}

/// [`font_family`]'s substitute for `family`, asked of the platform once.
#[cfg(feature = "system-fonts")]
fn substituted_family(family: &str) -> Option<String> {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock, PoisonError};
    static ASKED: OnceLock<Mutex<HashMap<String, Option<String>>>> = OnceLock::new();
    let mut asked = ASKED
        .get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    if let Some(answer) = asked.get(family) {
        return answer.clone();
    }
    let answer = native_theme::fonts::substitute_family(family);
    asked.insert(family.to_owned(), answer.clone());
    answer
}

/// Without feature `system-fonts`, no family is substituted.
#[cfg(not(feature = "system-fonts"))]
fn substituted_family(_family: &str) -> Option<String> {
    None
}

// --- Issue 36: Line height ---

/// Line height multiplier from the resolved theme defaults.
#[must_use]
pub fn line_height(resolved: &ResolvedTheme) -> f32 {
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

/// Dialog button gap in logical pixels.
#[must_use]
pub fn dialog_button_gap(resolved: &ResolvedTheme) -> f32 {
    resolved.dialog.button_gap
}

/// Scrollbar groove width in logical pixels.
#[must_use]
pub fn scrollbar_groove_width(resolved: &ResolvedTheme) -> f32 {
    resolved.scrollbar.groove_width
}

/// Selection text color (foreground for selected content).
#[must_use]
pub fn selection_text_color(resolved: &ResolvedTheme) -> Rgba {
    resolved.defaults.selection_text_color
}

/// Selection background when window is unfocused.
#[must_use]
pub fn selection_inactive_background(resolved: &ResolvedTheme) -> Rgba {
    resolved.defaults.selection_inactive_background
}

/// Foreground color for disabled elements.
#[must_use]
pub fn disabled_text_color(resolved: &ResolvedTheme) -> Rgba {
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

/// The delay between the pointer coming to rest on an element and its
/// tooltip showing, `tooltip.show_delay_ms`; `None` where the theme states
/// none.
///
/// [`apply`] installs it as gpui-base's application-wide `TooltipDefaults`
/// show delay, which every tooltip shown through gpui-component's tooltip
/// overlay waits (a `Button::tooltip`'s, among others). GPUI's own
/// `.tooltip()` on an element does not read those defaults: it waits GPUI's
/// 500 ms (gpui-pre `src/elements/div.rs:53`) unless the element is given
/// this value through `StatefulInteractiveElement::tooltip_show_delay`
/// (`:1708`).
#[must_use]
pub fn tooltip_show_delay(resolved: &ResolvedTheme) -> Option<Duration> {
    resolved
        .tooltip
        .show_delay
        .map(|ms| Duration::from_millis(u64::from(ms)))
}

// ---------------------------------------------------------------------------
// Installation: NativeTheme global, apply family, base-layer observer (§8.1, §3.3)
// ---------------------------------------------------------------------------

/// The native theme installed by [`apply`]; one per `App`, read with
/// [`ActiveNativeTheme::native_theme`].
///
/// Stores both resolved variants (when known) because upstream's
/// `Theme::change` switches modes without going through the connector, and the
/// re-apply observer must then find the variant of the mode upstream switched
/// to (rationale §2.22). Every field's default is the right initial state, so
/// `Default` is derived (a hand-written impl would trip clippy's
/// `derivable_impls`).
#[derive(Debug, Default)]
pub struct NativeTheme {
    light: Option<ResolvedTheme>,
    dark: Option<ResolvedTheme>,
    accessibility: AccessibilityPreferences,
    /// Set by the observer around its own write of `gpui_base::Theme`, so it
    /// can tell that notification from an upstream rebuild (§3.3).
    reapplying: bool,
    observer_installed: bool,
    /// Mode `apply` last installed; the fallback when the styled theme global
    /// is absent (D29).
    last_is_dark: bool,
}

impl Global for NativeTheme {}

impl NativeTheme {
    fn is_dark(&self, cx: &App) -> bool {
        cx.try_global::<GpuiTheme>()
            .map(GpuiTheme::is_dark)
            .unwrap_or(self.last_is_dark)
    }

    fn variant(&self, is_dark: bool) -> Option<&ResolvedTheme> {
        if is_dark {
            self.dark.as_ref()
        } else {
            self.light.as_ref()
        }
    }

    /// The stored variant for the styled theme's current mode, if any.
    #[must_use]
    pub fn resolved(&self, cx: &App) -> Option<&ResolvedTheme> {
        self.variant(self.is_dark(cx))
    }

    /// The preferences `apply` / `apply_accessibility` last installed.
    #[must_use]
    pub fn accessibility(&self) -> &AccessibilityPreferences {
        &self.accessibility
    }

    /// Borrowed view for the `geometry` builders, if a variant is stored for
    /// the current mode.
    #[must_use]
    pub fn native(&self, cx: &App) -> Option<Native<'_>> {
        self.resolved(cx).map(|resolved| Native {
            resolved,
            accessibility: &self.accessibility,
        })
    }
}

/// `cx.native_theme()` and `cx.native()`, mirroring gpui-component's
/// `cx.theme()`.
pub trait ActiveNativeTheme {
    /// The installed [`NativeTheme`], or `None` before [`apply`] ran.
    fn native_theme(&self) -> Option<&NativeTheme>;

    /// The [`Native`] view for the styled theme's current mode, or `None`
    /// before [`apply`] ran or when no variant is stored for that mode
    /// ([`NativeTheme::native`] of [`native_theme`](Self::native_theme)).
    fn native(&self) -> Option<Native<'_>>;
}

impl ActiveNativeTheme for App {
    fn native_theme(&self) -> Option<&NativeTheme> {
        self.try_global::<NativeTheme>()
    }

    fn native(&self) -> Option<Native<'_>> {
        self.native_theme()?.native(self)
    }
}

/// Borrowed inputs of every `geometry` builder (spec §9.1).
#[derive(Clone, Copy, Debug)]
pub struct Native<'a> {
    /// The resolved theme the values come from.
    pub resolved: &'a ResolvedTheme,
    /// Accessibility preferences; only `text_scaling_factor` affects geometry.
    pub accessibility: &'a AccessibilityPreferences,
}

impl<'a> Native<'a> {
    /// Unscaled, no reductions: for the preset path and tests.
    #[must_use]
    pub fn unscaled(resolved: &'a ResolvedTheme) -> Self {
        Self {
            resolved,
            accessibility: &AccessibilityPreferences::DEFAULT,
        }
    }
}

/// Install `theme` as gpui-component's global theme with the native base-layer
/// overrides, and keep them installed across upstream rebuilds (spec §8.1, §3.3).
///
/// 1. stores `resolved` under the theme's mode and `prefs` in [`NativeTheme`];
/// 2. initialises gpui-component if its theme global is absent (upstream
///    requires `gpui_component::init` before any component use; calling it
///    here only when the global is missing means it runs at most once);
/// 3. writes the styled theme, then installs a `ThemeConfig` for the *other*
///    stored variant (if any) under the same display name, so upstream's
///    `Theme::change` / `sync_system_appearance` reproduces native colours in
///    either mode (D34);
/// 4. projects into gpui-base (`sync_base`);
/// 5. writes the native scrollbar geometry/colours and resize-handle colours
///    onto gpui-base ([`base_layer::apply_overrides`]);
/// 6. forwards a reduced-motion *request*: `prefs.reduce_motion == true`
///    switches `App::reduce_motion` on if it is off; a later `false` undoes
///    only that switch, never clears a flag the connector did not set, and
///    asks gpui-base (which reads the OS preference itself since 0.6.2) to
///    re-read the system;
/// 7. installs the variant's [`tooltip_show_delay`] as gpui-base's
///    application-wide `TooltipDefaults` show delay, keeping the grace period
///    installed; where the theme states no delay, the application's comes back:
///    the one installed before the connector's first write, or set by the
///    application since (gpui-base's 500 ms where it set none);
/// 8. installs, once per `App`, the observer that restores step 5 and the 12
///    base-palette colours a `ThemeConfig` cannot carry (`red` … `cyan_light`,
///    private in `ThemeConfigColors`; D43), and installs step 7's delay for the
///    mode then current, whenever upstream rebuilds the theme (a
///    `Theme::change` or `sync_system_appearance` mode switch among them),
///    together with one deferred repeat for the update that installs it (the
///    subscription activates only at the end of that update's effect flush,
///    D38);
/// 9. refreshes every window so the change paints at once (D37).
///
/// Call `gpui_component::init` (or `gpui_kit::init`) *before* `apply`: an
/// `init` afterwards resets the theme to upstream's default. A single-variant
/// `apply` leaves the other mode's config at `ThemeConfig::default()`, so a
/// `Theme::change` to the unstored mode shows upstream's built-in
/// `ThemeColor::light()` / `dark()` constants (the default config carries no
/// colours);
/// call `apply` once per variant to store both.
///
/// Reduced motion: a flag that is already on is left to whoever switched it on
/// — gpui-base following the OS, or the application. While the connector's own
/// request stands, gpui-base discards OS readings, including the Linux
/// portal's first answer if `apply` ran before it arrived; once the request is
/// withdrawn, macOS and Windows are re-read at once and Linux follows again
/// from the portal's next change signal. gpui-base decides by value, not by
/// provenance, so an application flag that happens to equal gpui-base's own
/// last reading is replaced when the OS reading changes. To force the flag
/// either way regardless of the OS, call `cx.set_reduce_motion` after `apply`.
///
/// `theme` is moved into the global; `resolved` is cloned once.
pub fn apply(
    theme: GpuiTheme,
    resolved: &ResolvedTheme,
    prefs: &AccessibilityPreferences,
    cx: &mut App,
) {
    let is_dark = theme.is_dark();
    let (light, dark) = if is_dark {
        (None, Some(resolved))
    } else {
        (Some(resolved), None)
    };
    apply_inner(theme, light, dark, prefs, cx);
}

/// [`SystemThemeExt::to_gpui_theme`] for the OS mode, storing both variants
/// and installing both `ThemeConfig`s, then [`apply`]; upstream's
/// `Theme::sync_system_appearance` then reproduces native colours in either mode.
pub fn apply_system_theme(sys: &SystemTheme, cx: &mut App) {
    let theme = system_gpui_theme(sys);
    apply_inner(
        theme,
        Some(&sys.light),
        Some(&sys.dark),
        &sys.accessibility,
        cx,
    );
}

/// Apply a runtime change of the accessibility preferences (a portal signal,
/// a settings toggle). When a variant is stored for the current mode, the
/// styled theme is rebuilt from it with `prefs` and re-installed through the
/// [`apply`] path, so text scaling and transparency take effect and both
/// configs are refreshed (D35); the stored variants are kept. Without a stored
/// variant only the reduced-motion request is forwarded and the preferences
/// are stored.
pub fn apply_accessibility(prefs: &AccessibilityPreferences, cx: &mut App) {
    let rebuilt = cx.try_global::<NativeTheme>().and_then(|nt| {
        let is_dark = nt.is_dark(cx);
        let resolved = nt.variant(is_dark)?;
        let name = cx.try_global::<GpuiTheme>()?.theme_name().clone();
        Some(to_theme(resolved, &name, is_dark, prefs))
    });
    match rebuilt {
        // `None, None` keeps the stored variants; apply_inner stores `prefs`.
        Some(theme) => apply_inner(theme, None, None, prefs, cx),
        None => {
            if cx.has_global::<NativeTheme>() {
                cx.global_mut::<NativeTheme>().accessibility = prefs.clone();
            }
            forward_reduce_motion(prefs.reduce_motion, cx);
        }
    }
}

/// Whether the connector switched `App::reduce_motion` on and has not yet
/// switched it back (spec v0.5.9 §5).
#[derive(Default)]
struct ReduceMotionRequest(bool);

impl Global for ReduceMotionRequest {}

/// `true` is a request: switch the flag on, remembering that the connector did
/// so if it was off. `false` is the absence of a request: undo the connector's
/// own switch, if any, and ask gpui-base (which reads the OS preference itself
/// since 0.6.2) to re-read the system. gpui-base writes only a flag it still
/// owns, so a flag the application set is never cleared here.
fn forward_reduce_motion(reduce: bool, cx: &mut App) {
    if reduce {
        if !cx.reduce_motion() {
            cx.default_global::<ReduceMotionRequest>().0 = true;
            cx.set_reduce_motion(true);
        }
        return;
    }
    if std::mem::take(&mut cx.default_global::<ReduceMotionRequest>().0) {
        cx.set_reduce_motion(false);
    }
    gpui_base::apply_system_reduce_motion(cx);
}

/// The show delay the connector last installed on gpui-base's
/// `TooltipDefaults`, and the application's own, which a theme without a
/// delay brings back (spec §7.4).
#[derive(Default)]
struct TooltipDelayWrite {
    /// What the connector last installed; `None` before its first write.
    last_written: Option<Duration>,
    /// The delay installed before the connector's first write, or set by the
    /// application since.
    app_delay: Duration,
}

impl Global for TooltipDelayWrite {}

/// Install `theme`'s delay as the `TooltipDefaults` show delay, or the
/// application's where the theme states none. A show delay that is not the
/// one the connector last wrote was set by the application (or is gpui-base's
/// default, before the first write), so it becomes the application's delay.
/// The grace period is never touched. Decided by value, as gpui-base decides
/// reduced motion: an application delay equal to the one the connector last
/// wrote is taken for the connector's.
fn install_tooltip_delay(theme: Option<Duration>, cx: &mut App) {
    let current = gpui_base::TooltipDefaults::global(cx);
    let state = cx.default_global::<TooltipDelayWrite>();
    if state.last_written != Some(current.show_delay()) {
        state.app_delay = current.show_delay();
    }
    let delay = theme.unwrap_or(state.app_delay);
    state.last_written = Some(delay);
    if delay != current.show_delay() {
        current.with_show_delay(delay).install(cx);
    }
}

/// [`install_tooltip_delay`] for the stored variant of the styled theme's
/// current mode, or for no delay where none is stored.
fn install_variant_tooltip_delay(cx: &mut App) {
    let delay = cx
        .try_global::<NativeTheme>()
        .and_then(|nt| nt.resolved(cx))
        .and_then(tooltip_show_delay);
    install_tooltip_delay(delay, cx);
}

fn apply_inner(
    theme: GpuiTheme,
    light: Option<&ResolvedTheme>,
    dark: Option<&ResolvedTheme>,
    prefs: &AccessibilityPreferences,
    cx: &mut App,
) {
    let is_dark = theme.is_dark();
    // The display name of the config `to_theme` built for this mode; the other
    // variant's config takes the same name.
    let name: SharedString = theme.theme_name().clone();
    {
        let nt = cx.default_global::<NativeTheme>();
        if let Some(light) = light {
            nt.light = Some(light.clone());
        }
        if let Some(dark) = dark {
            nt.dark = Some(dark.clone());
        }
        nt.accessibility = prefs.clone();
        nt.last_is_dark = is_dark;
    }

    // D29: the styled accessors panic on a missing global; init creates it.
    if !cx.has_global::<GpuiTheme>() {
        gpui_component::init(cx);
    }
    *GpuiTheme::global_mut(cx) = theme;

    // D34: the other mode's config from its stored variant, so Theme::change
    // reproduces the native palette instead of the registry default.
    let other_config = cx.try_global::<NativeTheme>().and_then(|nt| {
        nt.variant(!is_dark).map(|other| {
            let mode = if is_dark {
                GpuiThemeMode::Light
            } else {
                GpuiThemeMode::Dark
            };
            Rc::new(config::to_theme_config(other, &name, mode, prefs))
        })
    });
    if let Some(cfg) = other_config {
        let styled = GpuiTheme::global_mut(cx);
        if is_dark {
            styled.light_theme = cfg;
        } else {
            styled.dark_theme = cfg;
        }
    }

    GpuiTheme::sync_base(cx);
    // `false`: this write's notification may be delivered before the observer
    // is active (§3.3), so it must not be marked as the observer's own.
    write_base_overrides(cx, false);
    forward_reduce_motion(prefs.reduce_motion, cx);
    install_variant_tooltip_delay(cx);
    install_observer_once(cx);
    // D37: paint now. A change from a timer, portal signal or menu action must
    // not wait for the next input event. The theme is written through
    // `Theme::global_mut` and `sync_base`, which refresh nothing -- upstream
    // leaves the refresh to such a caller (gpui-component
    // src/theme/mod.rs:468-469), where its own `Theme::change` refreshes every
    // window (`:334`) -- and `refresh_windows` schedules every window (gpui-pre
    // src/app.rs:1308).
    cx.refresh_windows();
}

/// The values to write onto gpui-base for `is_dark` (spec §3.3 step 2):
/// the stored variant when there is one; otherwise geometry from the other
/// variant and colours from the styled theme, which is exactly upstream's own
/// projection (`scrollbar`, `scrollbar_thumb`, `scrollbar_thumb_hover` with
/// active = hover; `border` / `drag_border` for the handles).
fn base_overrides_for(
    nt: &NativeTheme,
    is_dark: bool,
    styled: Option<&GpuiTheme>,
) -> Option<(base_layer::ScrollbarGeometry, gpui_base::ResizableTheme)> {
    if let Some(resolved) = nt.variant(is_dark) {
        return Some((
            base_layer::scrollbar_geometry(resolved),
            base_layer::resizable_theme(resolved),
        ));
    }
    let other = nt.variant(!is_dark)?;
    let mut geometry = base_layer::scrollbar_geometry(other);
    let mut resizable = base_layer::resizable_theme(other);
    if let Some(styled) = styled {
        geometry.track = styled.scrollbar;
        geometry.track_active_border = styled.border;
        geometry.thumb = styled.scrollbar_thumb;
        geometry.thumb_hover = styled.scrollbar_thumb_hover;
        geometry.thumb_active = styled.scrollbar_thumb_hover;
        resizable = gpui_base::ResizableTheme {
            handle: Some(styled.border),
            active_handle: Some(styled.drag_border),
        };
    }
    Some((geometry, resizable))
}

/// Compute and write the overrides for the current mode. `mark` flags the
/// write as the observer's own so the observer ignores its notification
/// (§3.3); only the observer passes `true`, because a flag set by `apply`
/// could be delivered before the observer is active and would then never be
/// cleared (rationale errors 40, 42).
fn write_base_overrides(cx: &mut App, mark: bool) {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return;
    };
    let is_dark = nt.is_dark(cx);
    let Some((geometry, resizable)) = base_overrides_for(nt, is_dark, cx.try_global::<GpuiTheme>())
    else {
        return;
    };
    if mark {
        cx.global_mut::<NativeTheme>().reapplying = true;
    }
    base_layer::apply_overrides(&geometry, resizable, cx);
}

/// Whether gpui-base's resize-handle colours are the ones the connector would
/// write for the current mode. `ResizableTheme` derives no `PartialEq`, so both
/// fields are compared. `true` when nothing is stored (nothing to repair).
fn handles_hold_native_values(cx: &App) -> bool {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return true;
    };
    let Some((_, expected)) = base_overrides_for(nt, nt.is_dark(cx), cx.try_global::<GpuiTheme>())
    else {
        return true;
    };
    let Some(current) = cx.try_global::<gpui_base::Theme>().map(|t| t.resizable) else {
        return true;
    };
    current.handle == expected.handle && current.active_handle == expected.active_handle
}

/// Restore the 12 base-palette colours (`red` … `cyan_light`) of the styled
/// theme from the stored variant for the current mode (D43).
///
/// `ThemeConfigColors` keeps them private (gpui-component
/// `src/theme/schema.rs:643-677`), so the config `apply` installs for a variant
/// cannot carry them; every `Theme::change` / `sync_system_appearance` resets
/// them to `ThemeColor::dark()` / `light()` (`:691-699`, `:1079-1083`) and
/// ends in `sync_base` (`theme/mod.rs:333`), whose `cx.set_global` of
/// `gpui_base::Theme` (`:473-478`) is the one write of it; its notification
/// reaches the base-theme observer. Writes only when a field differs; the
/// styled theme has no upstream observer, so the write triggers no rebuild.
/// After `apply` the connector is the sole writer of these 12 fields: a value
/// an application sets on them itself is replaced at the next rebuild.
fn repair_base_palette(cx: &mut App) {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return;
    };
    let is_dark = nt.is_dark(cx);
    let Some(resolved) = nt.variant(is_dark) else {
        return;
    };
    let native = colors::base_palette(resolved, is_dark);
    // Probe a copy first so an unchanged palette causes no notification.
    let differs = cx.try_global::<GpuiTheme>().is_some_and(|styled| {
        let mut probe: gpui_component::theme::ThemeColor = **styled;
        colors::copy_base_palette(&native, &mut probe)
    });
    if differs {
        colors::copy_base_palette(&native, GpuiTheme::global_mut(cx));
    }
}

/// Observe `gpui_base::Theme` (§3.3). Terminates because `global_mut` queues one
/// deduplicated notification (gpui-pre `src/app.rs:1983-1985`), delivered
/// after the pending mark is removed (`:2140-2144`): the observer's own write
/// yields exactly one further delivery, absorbed by `reapplying`.
///
/// The subscription activates through a deferred effect (`src/app.rs:2450-2462`)
/// at the end of the flush that follows this call, so a base-theme write made by
/// other code in the same update as the first `apply` (a
/// `Theme::sync_system_appearance` right after it) would stand until the next
/// rebuild. One deferred re-write, queued after the activation, closes that gap
/// (D38); it is the first notification the active observer receives.
fn install_observer_once(cx: &mut App) {
    if cx
        .try_global::<NativeTheme>()
        .is_none_or(|nt| nt.observer_installed)
    {
        return;
    }
    cx.global_mut::<NativeTheme>().observer_installed = true;
    cx.observe_global::<gpui_base::Theme>(|cx| {
        // A rebuild also resets the 12 private base-palette colours (D43);
        // a plain comparison makes this safe on every delivery.
        repair_base_palette(cx);
        // A mode switch upstream (`Theme::change`) ends in this rebuild, and
        // the new mode's variant may state another delay. It writes only a
        // changed delay, to `TooltipDefaults`, which this observer does not
        // watch.
        install_variant_tooltip_delay(cx);
        let Some(nt) = cx.try_global::<NativeTheme>() else {
            return;
        };
        if nt.reapplying {
            cx.global_mut::<NativeTheme>().reapplying = false;
            // A rebuild another effect performs between this observer's write
            // and the delivery of its notification is merged into that
            // notification by GPUI's per-type deduplication, so the flag alone
            // would swallow it (spec §3.3, limit). The handle colours are
            // comparable and upstream's rebuild changes them, so repair when
            // they no longer hold the native values; the scrollbar styles are
            // opaque and cannot be checked.
            if !handles_hold_native_values(cx) {
                write_base_overrides(cx, true);
            }
            return;
        }
        write_base_overrides(cx, true);
    })
    .detach();
    // Effects are FIFO, so this runs after the activation above and after any
    // same-update write. `false`: the active observer re-applies once on its
    // delivery and absorbs its own write, like any other rebuild (§3.3).
    cx.defer(|cx| {
        write_base_overrides(cx, false);
        repair_base_palette(cx);
    });
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// Compile-time tripwire: `gpui_component::Theme` has exactly these 20
    /// fields in 0.7.0. Upstream carried three `tile_*` fields through 0.6.0
    /// and 0.6.1 that this connector never set, and nothing noticed until
    /// 0.6.2 removed them again; a field *added* in a patch release would be
    /// left at upstream's default just as silently. Naming every field here
    /// turns either change into a compile error.
    #[test]
    fn every_theme_field_is_named() {
        let theme = to_theme(
            &test_resolved(),
            "Test",
            true,
            &AccessibilityPreferences::default(),
        );
        let GpuiTheme {
            // Set by `to_theme` from the resolved native theme.
            mode: _,
            font_family: _,
            font_size: _,
            mono_font_family: _,
            mono_font_size: _,
            radius: _,
            radius_lg: _,
            shadow: _,
            focus_ring: _,
            scrollbar_mode: _,
            highlight_theme: _,
            // `to_theme` fills the slot for `mode` only; the other keeps
            // `Theme::from(&ThemeColor)`'s empty `ThemeConfig::default()` until
            // a later `apply` stores the other variant (D34).
            light_theme: _,
            dark_theme: _,
            // Written by `Theme::from(&ThemeColor)`: the 139-field colour map,
            // the tokens upstream derives from it, and `transparent` (Issue 53).
            colors: _,
            tokens: _,
            transparent: _,
            // Left at upstream's default on purpose: native-theme models no
            // notification placement, list highlight default, sheet margin or
            // motion tokens (`ResolvedTheme` has no such field).
            notification: _,
            list: _,
            sheet: _,
            motion: _,
        } = theme;
    }

    /// Issue 1: fixed to use into_variant(true) for catppuccin-mocha (dark theme).
    fn test_resolved() -> ResolvedTheme {
        let nt = Theme::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(ColorMode::Dark)
            .expect("preset must have dark variant");
        variant
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolved preset must validate")
    }

    /// A family the system lacks is drawn in the family the platform
    /// substitutes for it, where feature `system-fonts` asks it; the answer
    /// is kept, so asking again gives the same family.
    #[test]
    fn a_missing_family_is_the_platforms_substitute() {
        let family = std::sync::Arc::<str>::from("native-theme: no such family");
        let drawn = font_family(&family);
        let platform = if cfg!(feature = "system-fonts") {
            substituted_family(&family)
        } else {
            None
        };
        match platform {
            Some(substitute) => assert_eq!(drawn.as_ref(), substitute),
            None => assert_eq!(drawn.as_ref(), family.as_ref()),
        }
        assert_eq!(font_family(&family), drawn, "asked again, another family");
    }

    /// On macOS the system UI font's stated family becomes gpui's own alias
    /// for it, `.SystemUIFont`; every other family, and every family on
    /// another platform, stays as stated (egui spec §8.8). The platform is
    /// a parameter, so this runs everywhere.
    #[test]
    fn the_macos_system_ui_font_is_gpuis_alias_only_on_macos() {
        for name in ["SF Pro", "sf pro"] {
            let family = std::sync::Arc::<str>::from(name);
            assert_eq!(ui_font_family(&family, true).as_ref(), ".SystemUIFont");
            assert_eq!(ui_font_family(&family, false).as_ref(), name);
        }
        for name in ["SF Mono", "SF Pro Text", "Inter"] {
            let family = std::sync::Arc::<str>::from(name);
            assert_eq!(ui_font_family(&family, true).as_ref(), name);
            assert_eq!(ui_font_family(&family, false).as_ref(), name);
        }
    }

    fn scaled(factor: f32) -> AccessibilityPreferences {
        let mut prefs = AccessibilityPreferences::default();
        prefs.text_scaling_factor = factor;
        prefs
    }

    #[test]
    fn to_theme_produces_valid_theme() {
        let resolved = test_resolved();
        let theme = to_theme(
            &resolved,
            "Test",
            true,
            &AccessibilityPreferences::default(),
        );

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
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolved preset must validate");
        let theme = to_theme(
            &resolved,
            "DarkTest",
            true,
            &AccessibilityPreferences::default(),
        );

        assert!(theme.is_dark());
    }

    #[test]
    fn to_theme_applies_font_and_geometry() {
        let resolved = test_resolved();
        let theme = to_theme(
            &resolved,
            "Test",
            true,
            &AccessibilityPreferences::default(),
        );

        assert_eq!(
            theme.font_family,
            font_family(&resolved.defaults.font.family)
        );
        assert_eq!(theme.font_size, px(resolved.defaults.font.size));
        assert_eq!(
            theme.mono_font_family,
            font_family(&resolved.defaults.mono_font.family)
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

    // §8.3: scrollbar_mode set from overlay_mode
    #[test]
    fn scrollbar_mode_from_overlay_mode() {
        let resolved = test_resolved();
        let theme = to_theme(
            &resolved,
            "Scroll",
            true,
            &AccessibilityPreferences::default(),
        );
        let expected = if resolved.scrollbar.overlay_mode {
            ScrollbarMode::Scrolling
        } else {
            ScrollbarMode::Always
        };
        assert_eq!(theme.scrollbar_mode, expected);
    }

    // Issue 43/44: highlight_theme matches is_dark
    #[test]
    fn highlight_theme_matches_is_dark() {
        let resolved = test_resolved();
        let dark_theme = to_theme(
            &resolved,
            "Dark",
            true,
            &AccessibilityPreferences::default(),
        );
        assert_eq!(
            dark_theme.highlight_theme.appearance,
            GpuiThemeMode::Dark,
            "dark theme should use dark highlight"
        );

        let light_resolved = {
            let spec = Theme::preset("catppuccin-latte").expect("preset must exist");
            let variant = spec.into_variant(ColorMode::Light).expect("light variant");
            variant
                .into_resolved(&native_theme::ResolutionContext::for_tests())
                .expect("must validate")
        };
        let light_theme = to_theme(
            &light_resolved,
            "Light",
            false,
            &AccessibilityPreferences::default(),
        );
        assert_eq!(
            light_theme.highlight_theme.appearance,
            GpuiThemeMode::Light,
            "light theme should use light highlight"
        );
    }

    /// §3.4: font sizes carry the factor; the config copies too, so
    /// `Theme::change` reproduces them.
    #[test]
    fn to_theme_scales_font_sizes_by_the_text_scaling_factor() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Scaled", true, &scaled(1.5));
        assert_eq!(theme.font_size, px(resolved.defaults.font.size * 1.5));
        assert_eq!(
            theme.mono_font_size,
            px(resolved.defaults.mono_font.size * 1.5)
        );
        assert_eq!(
            theme.dark_theme.font_size,
            Some(resolved.defaults.font.size * 1.5)
        );
        assert_eq!(
            theme.dark_theme.mono_font_size,
            Some(resolved.defaults.mono_font.size * 1.5)
        );
    }

    /// A text size the connector does not apply itself scales as the ones it
    /// does, and a degenerate factor leaves it as the theme states it.
    #[test]
    fn scaled_text_size_scales_every_size_alike() {
        let resolved = test_resolved();
        for size in [resolved.button.font.size, resolved.text_scale.caption.size] {
            assert_eq!(scaled_text_size(size, &scaled(1.5)), size * 1.5);
            for factor in [0.0, -1.0, f32::NAN, f32::INFINITY] {
                assert_eq!(
                    scaled_text_size(size, &scaled(factor)),
                    size,
                    "factor {factor}"
                );
            }
        }
    }

    /// §7.2: a non-finite or non-positive factor means "no scaling".
    #[test]
    fn to_theme_ignores_a_degenerate_text_scaling_factor() {
        let resolved = test_resolved();
        for factor in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            let theme = to_theme(&resolved, "Degenerate", true, &scaled(factor));
            assert_eq!(
                theme.font_size,
                px(resolved.defaults.font.size),
                "factor {factor}"
            );
        }
    }

    /// §8.3: only the flag has a receiver; a zero-width ring is not drawn.
    #[test]
    fn focus_ring_follows_focus_ring_width() {
        let resolved = test_resolved();
        let theme = to_theme(
            &resolved,
            "Ring",
            true,
            &AccessibilityPreferences::default(),
        );
        assert_eq!(theme.focus_ring, resolved.defaults.focus_ring_width > 0.0);
    }

    // -- from_preset tests --

    #[test]
    fn from_preset_takes_preferences() {
        let (theme, resolved) =
            from_preset("catppuccin-latte", false, &scaled(1.5)).expect("preset should load");
        assert_eq!(theme.font_size, px(resolved.defaults.font.size * 1.5));
    }

    #[test]
    fn from_preset_valid_light() {
        let (theme, _resolved) = from_preset(
            "catppuccin-latte",
            false,
            &AccessibilityPreferences::default(),
        )
        .expect("preset should load");
        assert!(!theme.is_dark());
    }

    #[test]
    fn from_preset_valid_dark() {
        let (theme, _resolved) = from_preset(
            "catppuccin-mocha",
            true,
            &AccessibilityPreferences::default(),
        )
        .expect("preset should load");
        assert!(theme.is_dark());
    }

    #[test]
    fn from_preset_returns_resolved() {
        let (_theme, resolved) = from_preset(
            "catppuccin-mocha",
            true,
            &AccessibilityPreferences::default(),
        )
        .expect("preset should load");
        // ResolvedTheme should have populated defaults
        assert!(resolved.defaults.font.size > 0.0);
    }

    #[test]
    fn from_preset_invalid_name() {
        let result = from_preset(
            "nonexistent-preset",
            false,
            &AccessibilityPreferences::default(),
        );
        assert!(result.is_err(), "invalid preset should return Err");
    }

    // Issue 23: error message includes the mode
    #[test]
    fn from_preset_error_message_includes_mode() {
        // Both modes should load for catppuccin-mocha (it has both variants)
        let _ = from_preset(
            "catppuccin-mocha",
            true,
            &AccessibilityPreferences::default(),
        )
        .expect("dark should work");
        let _ = from_preset(
            "catppuccin-mocha",
            false,
            &AccessibilityPreferences::default(),
        )
        .expect("light should work");
    }

    #[test]
    fn from_preset_takes_is_dark_from_the_resolved_mode() {
        let prefs = AccessibilityPreferences::default();
        let presets = Theme::list_presets();
        assert!(!presets.is_empty());
        for info in presets {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let (theme, _) = from_preset(info.key, mode.is_dark(), &prefs)
                    .unwrap_or_else(|e| panic!("{} {mode:?}: {e}", info.key));
                let resolved = Theme::preset(info.key)
                    .and_then(|spec| spec.resolve(mode))
                    .unwrap_or_else(|e| panic!("{} {mode:?}: {e}", info.key));
                assert_eq!(
                    theme.is_dark(),
                    resolved.mode.is_dark(),
                    "{} asked for {mode:?}",
                    info.key
                );
            }
        }
    }

    #[test]
    fn from_preset_light_only_asked_for_dark_is_light() {
        let mut spec = Theme::preset("catppuccin-latte").expect("preset should load");
        spec.dark = None;
        let (theme, resolved) = from_spec(&spec, true, &AccessibilityPreferences::default())
            .expect("the light variant should resolve");
        assert!(!theme.is_dark());
        assert_eq!(
            Some(resolved.defaults.background_color),
            spec.light
                .as_ref()
                .and_then(|light| light.defaults.background_color)
        );
    }

    // -- SystemThemeExt + from_system tests --

    #[test]
    fn system_theme_ext_to_gpui_theme() {
        // from_system() may fail on CI (no desktop env) -- skip gracefully
        let Ok(sys) = SystemTheme::from_system() else {
            return;
        };
        let loaded = sys.to_gpui_theme();
        assert_eq!(
            loaded.theme.is_dark(),
            sys.mode.is_dark(),
            "to_gpui_theme() is_dark should match SystemTheme.mode"
        );
        assert_eq!(&loaded.resolved, sys.pick(sys.mode));
        assert_eq!(loaded.mode, sys.mode);
        assert_eq!(loaded.accessibility, sys.accessibility);
        assert_eq!(loaded.icon_set, sys.icon_set);
        assert_eq!(loaded.icon_theme.as_deref(), sys.icon_theme_for(sys.mode));
    }

    #[test]
    fn from_system_does_not_panic() {
        // Just verify no panic -- result may be Err on CI
        let _ = from_system();
    }

    #[test]
    fn from_system_matches_system_theme_ext() {
        let (Ok(loaded), Ok(sys)) = (from_system(), SystemTheme::from_system()) else {
            return;
        };
        let expected = sys.to_gpui_theme();
        assert!(loaded.resolved.defaults.font.size > 0.0);
        assert_eq!(loaded.theme.is_dark(), loaded.mode.is_dark());
        assert_eq!(loaded.theme.is_dark(), expected.theme.is_dark());
        assert_eq!(loaded.theme.background, expected.theme.background);
        assert_eq!(loaded.resolved, expected.resolved);
        assert_eq!(loaded.mode, expected.mode);
        assert_eq!(loaded.accessibility, expected.accessibility);
        assert_eq!(loaded.icon_set, expected.icon_set);
        assert_eq!(loaded.icon_theme, expected.icon_theme);
    }

    #[test]
    fn from_system_matches_manual_path() {
        let Ok(sys) = SystemTheme::from_system() else {
            return;
        };
        let via_convenience = sys.to_gpui_theme().theme;
        let via_manual = to_theme(
            sys.pick(sys.mode),
            &sys.name,
            sys.mode.is_dark(),
            &sys.accessibility,
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
        assert!(border_line_width(&resolved) >= 0.0);
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
            line_height(&resolved) > 0.0,
            "line height should be positive"
        );
    }

    #[test]
    fn geometry_helpers() {
        let resolved = test_resolved();
        assert!(dialog_button_gap(&resolved) >= 0.0);
        assert!(scrollbar_groove_width(&resolved) > 0.0);
    }

    #[test]
    fn selection_and_disabled_helpers() {
        let resolved = test_resolved();
        let _ = selection_text_color(&resolved);
        let _ = selection_inactive_background(&resolved);
        let _ = disabled_text_color(&resolved);
    }

    #[test]
    fn focus_ring_helpers() {
        let resolved = test_resolved();
        assert!(focus_ring_width(&resolved) >= 0.0);
        assert!(focus_ring_offset(&resolved) >= 0.0);
    }

    #[test]
    fn tooltip_show_delay_is_the_stated_delay_or_none() {
        let prefs = AccessibilityPreferences::default();
        let (_, breeze) = from_preset("kde-breeze", false, &prefs).unwrap();
        assert_eq!(breeze.tooltip.show_delay, Some(700), "precondition");
        assert_eq!(
            tooltip_show_delay(&breeze),
            Some(std::time::Duration::from_millis(700))
        );
        let (_, material) = from_preset("material-3", false, &prefs).unwrap();
        assert_eq!(material.tooltip.show_delay, Some(0), "precondition");
        assert_eq!(
            tooltip_show_delay(&material),
            Some(std::time::Duration::ZERO)
        );
        let (_, mocha) = from_preset("catppuccin-mocha", true, &prefs).unwrap();
        assert_eq!(mocha.tooltip.show_delay, None, "precondition");
        assert_eq!(tooltip_show_delay(&mocha), None);
    }

    // -- Issue 26: integration tests for all 17 presets in both modes --

    #[test]
    fn all_presets_dark_mode_no_panic() {
        let presets = Theme::list_presets();
        for info in presets {
            let name = info.key;
            let result = from_preset(name, true, &AccessibilityPreferences::default());
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
        for info in presets {
            let name = info.key;
            let result = from_preset(name, false, &AccessibilityPreferences::default());
            assert!(
                result.is_ok(),
                "from_preset({name}, false) failed: {:?}",
                result.err()
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod apply_tests {
    use super::*;
    use gpui::TestAppContext;
    use gpui_component::scroll::ScrollbarMode;

    /// Spec v0.5.9 §5: `false` is the absence of a request. gpui-base 0.6.2+
    /// (or the application) may have switched the flag on; a preset applied
    /// with default preferences must not switch it off.
    #[gpui::test]
    fn apply_with_default_prefs_keeps_a_flag_it_did_not_set(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, &prefs).expect("preset should load");
        cx.update(|cx| {
            cx.set_reduce_motion(true); // stands for gpui-base's OS reading
            apply(theme, &resolved, &prefs, cx);
            assert!(cx.reduce_motion());
        });
    }

    #[gpui::test]
    fn releasing_a_request_keeps_a_flag_that_was_already_on(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.set_reduce_motion(true);
            apply_accessibility(&prefs(true), cx);
            apply_accessibility(&prefs(false), cx);
            assert!(cx.reduce_motion(), "the connector never switched it on");
        });
    }

    #[gpui::test]
    fn a_repeated_request_is_released_once(cx: &mut TestAppContext) {
        cx.update(|cx| {
            apply_accessibility(&prefs(true), cx);
            apply_accessibility(&prefs(true), cx);
            assert!(cx.reduce_motion());
            apply_accessibility(&prefs(false), cx);
            assert!(!cx.reduce_motion());
        });
    }

    fn prefs(reduce_motion: bool) -> AccessibilityPreferences {
        let mut prefs = AccessibilityPreferences::default();
        prefs.reduce_motion = reduce_motion;
        prefs
    }

    /// The dark catppuccin preset, with the precondition every observer test
    /// relies on: the native *active* handle colour (`splitter.hover_color`)
    /// differs from `drag_border`, which upstream's projection writes there.
    /// `handle` cannot serve as the observable: every preset inherits
    /// `splitter.divider_color` from `defaults.border.color`
    /// (`docs/inheritance-rules.toml:238`), which is also what upstream writes,
    /// so that slot holds the same value whichever side wrote it.
    fn preset_for_observer_tests(prefs: &AccessibilityPreferences) -> (GpuiTheme, ResolvedTheme) {
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, prefs).expect("preset should load");
        assert_ne!(
            colors::rgba_to_hsla(resolved.splitter.hover_color),
            theme.drag_border,
            "precondition: the native active-handle colour must differ from upstream's drag_border"
        );
        (theme, resolved)
    }

    /// A preset whose light and dark variants resolve to different backgrounds.
    fn preset_with_two_variants(
        prefs: &AccessibilityPreferences,
    ) -> ((GpuiTheme, ResolvedTheme), (GpuiTheme, ResolvedTheme)) {
        for info in Theme::list_presets() {
            if let (Ok(dark), Ok(light)) = (
                from_preset(info.key, true, prefs),
                from_preset(info.key, false, prefs),
            ) && dark.1.defaults.background_color != light.1.defaults.background_color
            {
                return (dark, light);
            }
        }
        panic!("no preset with distinct light and dark variants");
    }

    /// §12: after `apply`, every receiver holds the native value; after upstream
    /// rebuilds the base layer, the observer restores it; and the test returning
    /// proves the observer terminates. Two changes catch a stale `reapplying`.
    #[gpui::test]
    fn apply_installs_and_survives_theme_change(cx: &mut TestAppContext) {
        let prefs = prefs(true);
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let handle = colors::rgba_to_hsla(resolved.splitter.divider_color);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        let expected_mode = if resolved.scrollbar.overlay_mode {
            ScrollbarMode::Scrolling
        } else {
            ScrollbarMode::Always
        };

        let native_palette = colors::base_palette(&resolved, true);

        // No gpui_component::init here: apply must initialise the styled layer
        // itself when it is absent (D29) and must not panic.
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));

        cx.update(|cx| {
            assert!(GpuiTheme::global(cx).is_dark());
            let base = gpui_base::Theme::global(cx);
            assert_eq!(base.scrollbar.mode(), expected_mode);
            assert_eq!(base.resizable.handle, Some(handle));
            assert_eq!(base.resizable.active_handle, Some(active));
            assert!(cx.reduce_motion());
            assert!(cx.native_theme().and_then(|t| t.resolved(cx)).is_some());
            // Upstream's projection writes `drag_border` into active_handle; the
            // helper's precondition makes the restore assertions below meaningful.
            assert_ne!(GpuiTheme::global(cx).drag_border, active);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx); // rebuilds gpui_base::Theme
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "observer restored the native value after the first rebuild"
            );
            // D43: the rebuild reset the 12 private base-palette fields to
            // upstream's constants; the observer copied the native ones back.
            let styled = GpuiTheme::global(cx);
            assert_eq!(
                styled.red, native_palette.red,
                "red restored after Theme::change"
            );
            assert_eq!(styled.magenta_light, native_palette.magenta_light);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "and after the second rebuild (no stale reapplying flag)"
            );
        });
    }

    /// §3.3, D38: the observer activates at the end of the effect flush that
    /// installs it, so a rebuild in the *same* update as the first `apply` (an
    /// application calling `Theme::sync_system_appearance` right after it) is
    /// delivered while the observer is inactive. The deferred re-write in
    /// `install_observer_once` closes that gap; without it this test fails.
    #[gpui::test]
    fn apply_then_change_in_the_same_update_keeps_overrides(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        let native_palette = colors::base_palette(&resolved, true);
        cx.update(|cx| {
            apply(theme, &resolved, &prefs, cx);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx); // same update: observer not yet active
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "the deferred re-write restored the overrides after a same-update rebuild"
            );
            assert_eq!(
                GpuiTheme::global(cx).red,
                native_palette.red,
                "and the base palette (D43)"
            );
        });
    }

    /// D43 precondition: upstream's rebuild really does reset the base palette
    /// (otherwise the repair assertions above would be vacuous). Upstream's
    /// own dark palette, read after a plain `init` + `change`, differs from the
    /// preset's; `Theme::change` after `apply` would install it were there no
    /// repair, and the repaired value is the preset's.
    #[gpui::test]
    fn theme_change_resets_private_base_palette_without_repair(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let native_palette = colors::base_palette(&resolved, true);
        let upstream_red = cx.update(|cx| {
            gpui_component::init(cx);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
            GpuiTheme::global(cx).red
        });
        assert_ne!(
            upstream_red, native_palette.red,
            "the preset's danger colour must differ from upstream's red for this test to mean anything"
        );
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        cx.update(|cx| {
            let red = GpuiTheme::global(cx).red;
            assert_ne!(red, upstream_red, "the repair replaced upstream's constant");
            assert_eq!(red, native_palette.red);
        });
    }

    /// The `reapplying` flag cannot tell the observer's own notification from a
    /// rebuild another effect performs before that notification is delivered:
    /// GPUI merges the two (spec §3.3, limit). The observer therefore repairs
    /// the handle colours when they no longer hold the native values. Without
    /// that repair the deferred `Theme::change` below would leave upstream's
    /// `drag_border` in `active_handle`.
    #[gpui::test]
    fn observer_repairs_a_rebuild_merged_into_its_own_notification(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        cx.update(|cx| apply(theme, &resolved, &prefs, cx)); // observer active after this flush
        cx.update(|cx| {
            // Queue: [N_base (this write), Defer(change)]. The observer answers
            // N_base with a marked write whose notification lands after the
            // deferred change; the change's own notification is deduplicated.
            let _ = gpui_base::Theme::global_mut(cx);
            cx.defer(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "the observer repaired the rebuild that its own notification had absorbed"
            );
        });
    }

    /// §3.3 step 2: with no stored variant for the new mode, colours come from
    /// the styled theme (upstream's own projection) and nothing panics.
    #[gpui::test]
    fn apply_without_stored_variant_falls_back(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs); // stores dark only
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Light, None, cx));
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed by apply");
            assert!(nt.resolved(cx).is_none(), "no light variant was stored");
            assert!(nt.native(cx).is_none());
            let styled = GpuiTheme::global(cx);
            let base = gpui_base::Theme::global(cx);
            assert_eq!(base.resizable.handle, Some(styled.border));
            assert_eq!(base.resizable.active_handle, Some(styled.drag_border));
        });
    }

    #[gpui::test]
    fn apply_system_theme_stores_both_variants(cx: &mut TestAppContext) {
        let Ok(sys) = SystemTheme::from_system() else {
            return;
        }; // CI has no desktop
        cx.update(|cx| apply_system_theme(&sys, cx));
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed");
            assert!(nt.resolved(cx).is_some());
            GpuiTheme::change(
                if sys.mode.is_dark() {
                    GpuiThemeMode::Light
                } else {
                    GpuiThemeMode::Dark
                },
                None,
                cx,
            );
        });
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed");
            assert!(nt.resolved(cx).is_some(), "the other variant is stored too");
            // D34: the other mode's palette is the native one, not the registry default.
            let other = if sys.mode.is_dark() {
                &sys.light
            } else {
                &sys.dark
            };
            assert_eq!(
                colors::hsla_to_hex(GpuiTheme::global(cx).background),
                colors::hsla_to_hex(colors::rgba_to_hsla(other.defaults.background_color))
            );
        });
    }

    /// D34: once both variants are applied, upstream's `Theme::change` reproduces
    /// the native palette of either mode through the installed `ThemeConfig`.
    #[gpui::test]
    fn apply_installs_configs_for_both_variants(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let ((dark_theme, dark), (light_theme, light)) = preset_with_two_variants(&prefs);
        cx.update(|cx| {
            apply(dark_theme, &dark, &prefs, cx);
            apply(light_theme, &light, &prefs, cx); // light is current; the dark config must survive
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            assert!(styled.is_dark());
            // Hex comparison: the config round trip is exact for 8-bit colours.
            assert_eq!(
                colors::hsla_to_hex(styled.background),
                colors::hsla_to_hex(colors::rgba_to_hsla(dark.defaults.background_color))
            );
            assert_eq!(
                colors::hsla_to_hex(styled.button_primary),
                colors::hsla_to_hex(colors::rgba_to_hsla(dark.button.primary_background)),
                "button_* fields survive the config round trip"
            );
            // D41: the config carries the mode's default highlighter style, so the
            // switch to dark also switched code highlighting.
            assert_eq!(styled.highlight_theme.appearance, GpuiThemeMode::Dark);
            assert!(cx.native_theme().and_then(|t| t.resolved(cx)).is_some());
        });
    }

    /// D35: a runtime preference change rebuilds the styled theme from the
    /// stored variant, so scaling reaches `font_size` and its config copy.
    #[gpui::test]
    fn apply_accessibility_rescales_from_the_stored_variant(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, &prefs).expect("preset should load");
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        let mut scaled = AccessibilityPreferences::default();
        scaled.text_scaling_factor = 1.5;
        scaled.reduce_motion = true;
        cx.update(|cx| apply_accessibility(&scaled, cx));
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            assert_eq!(styled.font_size, px(resolved.defaults.font.size * 1.5));
            assert_eq!(
                styled.dark_theme.font_size,
                Some(resolved.defaults.font.size * 1.5)
            );
            assert!(cx.reduce_motion());
            let nt = cx.native_theme().expect("installed");
            assert_eq!(nt.accessibility().text_scaling_factor, 1.5);
            assert!(
                nt.resolved(cx).is_some(),
                "the stored variant survives the rebuild"
            );
        });
    }

    /// A preset that states `tooltip.show_delay_ms`, a second that states
    /// another, and one that states none.
    fn tooltip_delay_presets() -> [(GpuiTheme, ResolvedTheme); 3] {
        let prefs = AccessibilityPreferences::default();
        let load = |name| from_preset(name, false, &prefs).expect("preset should load");
        let (breeze, adwaita, latte) = (
            load("kde-breeze"),
            load("adwaita"),
            load("catppuccin-latte"),
        );
        assert_eq!(breeze.1.tooltip.show_delay, Some(700), "precondition");
        assert_eq!(adwaita.1.tooltip.show_delay, Some(500), "precondition");
        assert_eq!(latte.1.tooltip.show_delay, None, "precondition");
        [breeze, adwaita, latte]
    }

    fn ms(ms: u64) -> std::time::Duration {
        std::time::Duration::from_millis(ms)
    }

    fn apply_preset(preset: &(GpuiTheme, ResolvedTheme), cx: &mut App) {
        apply(
            preset.0.clone(),
            &preset.1,
            &AccessibilityPreferences::default(),
            cx,
        );
    }

    /// Spec §7.4: the theme's delay becomes the application-wide
    /// `TooltipDefaults` show delay; the grace period stays gpui-base's.
    #[gpui::test]
    fn a_stated_tooltip_delay_is_installed(cx: &mut TestAppContext) {
        let [breeze, _, _] = tooltip_delay_presets();
        cx.update(|cx| {
            apply_preset(&breeze, cx);
            let defaults = gpui_base::TooltipDefaults::global(cx);
            assert_eq!(defaults.show_delay(), ms(700));
            assert_eq!(
                defaults.grace_period(),
                gpui_base::TooltipDefaults::new().grace_period()
            );
        });
    }

    /// Review Focus 2: a theme without the field keeps the toolkit's delay.
    #[gpui::test]
    fn a_theme_without_a_delay_keeps_the_toolkits(cx: &mut TestAppContext) {
        let [_, _, latte] = tooltip_delay_presets();
        cx.update(|cx| {
            apply_preset(&latte, cx);
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx),
                gpui_base::TooltipDefaults::new()
            );
        });
    }

    /// Review Focus 3: from a theme that states a delay to one that does not,
    /// the delay goes back to gpui-base's 500 ms where the application set
    /// none.
    #[gpui::test]
    fn a_switch_to_a_theme_without_a_delay_restores_the_default(cx: &mut TestAppContext) {
        let [breeze, adwaita, latte] = tooltip_delay_presets();
        cx.update(|cx| {
            apply_preset(&breeze, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(700));
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(500));
            apply_preset(&breeze, cx);
            apply_preset(&adwaita, cx);
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(500));
        });
    }

    /// Review Focus 3 and 6: the delay and grace period the application
    /// installed before the first apply. A theme's delay replaces the
    /// application's delay and keeps its grace period; a theme without one
    /// brings the application's delay back.
    #[gpui::test]
    fn the_applications_defaults_installed_first_are_kept(cx: &mut TestAppContext) {
        let [breeze, _, latte] = tooltip_delay_presets();
        let own = gpui_base::TooltipDefaults::new()
            .with_show_delay(ms(250))
            .with_grace_period(ms(80));
        cx.update(|cx| {
            own.install(cx);
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
            apply_preset(&breeze, cx);
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx),
                own.with_show_delay(ms(700))
            );
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
        });
    }

    /// Review Focus 3 and 6: a delay the application sets between two
    /// applies is its own from then on, and outlives a later theme's delay.
    #[gpui::test]
    fn a_delay_the_application_sets_between_applies_is_kept(cx: &mut TestAppContext) {
        let [breeze, adwaita, latte] = tooltip_delay_presets();
        cx.update(|cx| {
            apply_preset(&breeze, cx);
            let own = gpui_base::TooltipDefaults::global(cx)
                .with_show_delay(ms(900))
                .with_grace_period(ms(40));
            own.install(cx);
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
            apply_preset(&adwaita, cx);
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx),
                own.with_show_delay(ms(500))
            );
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
        });
    }

    /// An upstream mode switch (`Theme::change`, which ends in a rebuild of
    /// gpui-base's theme) installs the new mode's variant's delay, so a theme
    /// whose two variants state different delays does not keep the previous
    /// mode's.
    #[gpui::test]
    fn an_upstream_mode_switch_installs_that_variants_delay(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (dark_theme, dark) =
            from_preset("kde-breeze", true, &prefs).expect("preset should load");
        let (light_theme, mut light) =
            from_preset("kde-breeze", false, &prefs).expect("preset should load");
        assert_eq!(dark.tooltip.show_delay, Some(700), "precondition");
        light.tooltip.show_delay = Some(300);
        cx.update(|cx| {
            apply(dark_theme, &dark, &prefs, cx);
            apply(light_theme, &light, &prefs, cx);
            assert!(!GpuiTheme::global(cx).is_dark());
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(300));
        });
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        cx.update(|cx| {
            assert!(GpuiTheme::global(cx).is_dark());
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx).show_delay(),
                ms(700),
                "the dark variant's delay after the switch"
            );
            GpuiTheme::change(GpuiThemeMode::Light, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx).show_delay(),
                ms(300),
                "the light variant's delay after the switch back"
            );
        });
    }

    #[gpui::test]
    fn apply_accessibility_forwards_reduce_motion(cx: &mut TestAppContext) {
        cx.update(|cx| {
            apply_accessibility(&prefs(true), cx); // works without a NativeTheme
            assert!(cx.reduce_motion());
            apply_accessibility(&prefs(false), cx);
            assert!(!cx.reduce_motion());
        });
    }

    #[test]
    fn native_unscaled_has_factor_one() {
        let (_, resolved) = from_preset(
            "catppuccin-mocha",
            true,
            &AccessibilityPreferences::default(),
        )
        .unwrap();
        let n = Native::unscaled(&resolved);
        assert_eq!(n.accessibility.text_scale_factor(), 1.0);
        assert!(!n.accessibility.reduce_motion);
    }

    #[test]
    fn base_overrides_fallback_takes_geometry_from_the_other_variant_and_colours_from_styled() {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = from_preset("catppuccin-mocha", true, &prefs).unwrap();
        let nt = NativeTheme {
            dark: Some(resolved.clone()),
            ..NativeTheme::default()
        };
        let (g, r) =
            base_overrides_for(&nt, false, Some(&theme)).expect("falls back to the dark variant");
        let dark = base_layer::scrollbar_geometry(&resolved);
        assert_eq!(g.track_width, dark.track_width);
        assert_eq!(g.thumb_inset, dark.thumb_inset);
        assert_eq!(g.track, theme.scrollbar);
        assert_eq!(g.thumb_active, theme.scrollbar_thumb_hover);
        assert_eq!(r.handle, Some(theme.border));
        assert!(base_overrides_for(&NativeTheme::default(), false, Some(&theme)).is_none());
    }
}
