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
//! let (theme, resolved, is_dark) = from_system()?;
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
//! let variant = nt.into_variant(ColorMode::Dark)?;
//! let resolved = variant.resolve_system()?;
//! let prefs = AccessibilityPreferences::from_system();
//! let theme = to_theme(&resolved, "Catppuccin Mocha", true, &prefs);
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
//! The connector maps [`ResolvedTheme`] onto gpui-component's `ThemeColor`
//! (all 139 colour fields), `ThemeConfig` (fonts, radii, shadow, highlighter),
//! the styled `Theme`'s `focus_ring` and `scrollbar_mode`, and gpui-base's
//! scrollbar and resize-handle styles.
//!
//! | Category | Mapped | Notes |
//! |----------|--------|-------|
//! | `defaults` colors | All 24 | background, foreground, accent, danger, etc. |
//! | `defaults` geometry | radius, radius_lg, shadow, focus ring | fonts scaled by the text-scaling factor |
//! | `button` | all 28 `button_*` plus `primary*` / `secondary*` | solid native surfaces (the 0.5.1 semantics) |
//! | `tab` | 5 of 10 colours | geometry is upstream work (`Tab`'s render writes its own height, radius and text size into the style bag the caller's setters fill, `tab/tab.rs:801-808`) |
//! | `sidebar` | 2 of 6 | background, font.color |
//! | `window` | 2 of 6 | title_bar_background, border |
//! | `input` | 2 of 13 colours + geometry | border, caret; height, radius, border, text via `geometry::input` |
//! | `scrollbar` | colours + geometry | track/thumb colours, widths, inset, min length via `base_layer` |
//! | `status_bar` | 2 of 3 | background, border |
//! | `table` | head + foot | `table_foot*` mirror `table_head*` |
//! | `slider`, `switch` | 2 colours each | fill/thumb colours; geometry upstream |
//! | `progress_bar` | fill + geometry | height, radius, min width via `geometry::progress` |
//! | `list` | 3 of 13 colours + geometry | row height, padding, font via `geometry::list_item` |
//! | `popover` | 2 of 3 + geometry | background, font.color; padding, radius via `geometry::popover` |
//! | `link` | 1 of 9 | hover_background |
//! | `splitter` | colours | divider/hover via `base_layer::resizable_theme`; width upstream |
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
//! splitter width, button icon gap, input padding, popup-menu rows), and tab
//! height, radius and text size, which `Tab`'s render writes into the same
//! style bag the caller's setters fill (`tab/tab.rs:801-808`), stay upstream
//! work; §14 of the v0.5.8 specification
//! (<https://github.com/tiborgats/native-theme/blob/main/docs/todo_v0.5.8_gpui-component-0.6-spec.md>)
//! lists each item with the upstream line that makes it unreachable.

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod base_layer;
pub(crate) mod colors;
pub(crate) mod config;
pub(crate) mod derive;
pub mod geometry;
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
pub use native_theme::{AccessibilityPreferences, Result, SystemTheme};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;

use gpui::{App, Global, SharedString, px};
use gpui_component::scroll::ScrollbarMode;
use gpui_component::theme::{Theme as GpuiTheme, ThemeMode as GpuiThemeMode};
use std::rc::Rc;

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
    prefs: &AccessibilityPreferences,
) -> GpuiTheme {
    let s = text_scale_factor(prefs);
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
    theme.font_family = SharedString::from(d.font.family.clone());
    // §3.4: Root sets the window rem to font_size (gpui-component 0.6.0
    // src/root.rs:579), so scaling these two sizes scales every rem-relative
    // size in gpui-component, as the platform toolkit scales its own text.
    theme.font_size = px(d.font.size * s);
    theme.mono_font_family = SharedString::from(d.mono_font.family.clone());
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
/// The preset name is used as the theme display name.
///
/// Pass `&AccessibilityPreferences::default()` for no scaling, or
/// `&AccessibilityPreferences::from_system()` to honour the OS preferences
/// under a preset (spec §7.1).
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
    let spec = Theme::preset(name)?;
    let display_name = spec.name.clone();
    let variant = spec.into_variant(if is_dark {
        ColorMode::Dark
    } else {
        ColorMode::Light
    })?;
    let resolved = variant.resolve_system()?;
    let theme = to_theme(&resolved, &display_name, is_dark, prefs);
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
    let name = sys.name; // K-5: move instead of clone
    let accessibility = sys.accessibility;
    let resolved = if is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name, is_dark, &accessibility);
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
            &self.accessibility,
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

/// Text-scaling multiplier from the preferences: the factor when it is finite
/// and positive, else `1.0` (spec §7.2). Named `text_scale_factor` because
/// [`text_scale()`], the public typography-scale accessor, already owns the
/// shorter name.
pub(crate) fn text_scale_factor(prefs: &AccessibilityPreferences) -> f32 {
    let s = prefs.text_scaling_factor;
    if s.is_finite() && s > 0.0 { s } else { 1.0 }
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
#[derive(Default)]
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

/// `cx.native_theme()`, mirroring gpui-component's `cx.theme()`.
pub trait ActiveNativeTheme {
    /// The installed [`NativeTheme`], or `None` before [`apply`] ran.
    fn native_theme(&self) -> Option<&NativeTheme>;
}

impl ActiveNativeTheme for App {
    fn native_theme(&self) -> Option<&NativeTheme> {
        self.try_global::<NativeTheme>()
    }
}

/// Borrowed inputs of every `geometry` builder (spec §9.1).
#[derive(Clone, Copy)]
pub struct Native<'a> {
    /// The resolved theme the values come from.
    pub resolved: &'a ResolvedTheme,
    /// Accessibility preferences; only `text_scaling_factor` affects geometry.
    pub accessibility: &'a AccessibilityPreferences,
}

static UNSCALED: AccessibilityPreferences = AccessibilityPreferences {
    text_scaling_factor: 1.0,
    reduce_motion: false,
    high_contrast: false,
    reduce_transparency: false,
};

impl<'a> Native<'a> {
    /// Unscaled, no reductions: for the preset path and tests.
    #[must_use]
    pub fn unscaled(resolved: &'a ResolvedTheme) -> Self {
        Self {
            resolved,
            accessibility: &UNSCALED,
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
/// 6. forwards `prefs.reduce_motion` to GPUI;
/// 7. installs, once per `App`, the observer that restores step 5 and the 12
///    base-palette colours a `ThemeConfig` cannot carry (`red` … `cyan_light`,
///    private in `ThemeConfigColors`; D43) whenever upstream rebuilds the
///    theme, together with one deferred repeat for the update that installs it
///    (the subscription activates only at the end of that update's effect
///    flush, D38);
/// 8. refreshes every window so the change paints at once (D37).
///
/// Call `gpui_component::init` (or `gpui_kit::init`) *before* `apply`: an
/// `init` afterwards resets the theme to upstream's default. A single-variant
/// `apply` leaves the other mode's config at `ThemeConfig::default()`, so a
/// `Theme::change` to the unstored mode shows upstream's built-in
/// `ThemeColor::light()` / `dark()` constants (the default config carries no
/// colours);
/// call `apply` once per variant to store both.
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
    let theme = sys.to_gpui_theme();
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
/// variant only `reduce_motion` is forwarded and the preferences are stored.
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
            cx.set_reduce_motion(prefs.reduce_motion);
        }
    }
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
    cx.set_reduce_motion(prefs.reduce_motion);
    install_observer_once(cx);
    // D37: paint now. A change from a timer, portal signal or menu action must
    // not wait for the next input event; upstream refreshes only the window
    // passed to Theme::change (gpui-pre 0.3.3 src/app.rs:1074).
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
/// `ThemeConfigColors` keeps them private (gpui-component 0.6.0
/// `src/theme/schema.rs:657-668`), so the config `apply` installs for a variant
/// cannot carry them; every `Theme::change` / `sync_system_appearance` resets
/// them to `ThemeColor::dark()` / `light()` (`:687-695`, `:1074-1078`) and
/// ends in `cx.set_global` of `gpui_base::Theme` (`theme/mod.rs:247-256`,
/// `:321-325`), whose notification reaches the base-theme observer. Writes
/// only when a field differs; the styled theme has no upstream observer, so
/// the write triggers no rebuild. After `apply` the connector is the sole
/// writer of these 12 fields: a value an application sets on them itself is
/// replaced at the next rebuild.
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
/// deduplicated notification (gpui-pre 0.3.3 `src/app.rs:1662-1664`), delivered
/// after the pending mark is removed (`:1817-1821`): the observer's own write
/// yields exactly one further delivery, absorbed by `reapplying`.
///
/// The subscription activates through a deferred effect (`src/app.rs:2087-2099`)
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

    fn scaled(factor: f32) -> AccessibilityPreferences {
        AccessibilityPreferences {
            text_scaling_factor: factor,
            ..AccessibilityPreferences::default()
        }
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

    fn prefs(reduce_motion: bool) -> AccessibilityPreferences {
        AccessibilityPreferences {
            reduce_motion,
            ..AccessibilityPreferences::default()
        }
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
        let scaled = AccessibilityPreferences {
            text_scaling_factor: 1.5,
            reduce_motion: true,
            ..AccessibilityPreferences::default()
        };
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
        assert_eq!(text_scale_factor(n.accessibility), 1.0);
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
