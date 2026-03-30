//! # native-theme
//!
//! Cross-platform native theme detection and loading for Rust GUI applications.
//!
//! Any Rust GUI app can look native on any platform by loading a single theme
//! file or reading live OS settings, without coupling to any specific toolkit.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// Generates `merge()` and `is_empty()` methods for theme structs.
///
/// Three field categories:
/// - `option { field1, field2, ... }` -- `Option<T>` leaf fields
/// - `nested { field1, field2, ... }` -- nested struct fields with their own `merge()`
/// - `optional_nested { field1, field2, ... }` -- `Option<T>` where T has its own `merge()`
///
/// For `option` fields, `Some` values in the overlay replace the corresponding
/// fields in self; `None` fields are left unchanged.
/// For `nested` fields, merge is called recursively.
/// For `optional_nested` fields: if both base and overlay are `Some`, the inner values
/// are merged recursively. If base is `None` and overlay is `Some`, overlay is cloned.
/// If overlay is `None`, the base field is preserved unchanged.
///
/// # Examples
///
/// ```ignore
/// impl_merge!(MyColors {
///     option { accent, background }
/// });
/// ```
macro_rules! impl_merge {
    (
        $struct_name:ident {
            $(option { $($opt_field:ident),* $(,)? })?
            $(nested { $($nest_field:ident),* $(,)? })?
            $(optional_nested { $($on_field:ident),* $(,)? })?
        }
    ) => {
        impl $struct_name {
            /// Merge an overlay into this value. `Some` fields in the overlay
            /// replace the corresponding fields in self; `None` fields are
            /// left unchanged. Nested structs are merged recursively.
            pub fn merge(&mut self, overlay: &Self) {
                $($(
                    if overlay.$opt_field.is_some() {
                        self.$opt_field = overlay.$opt_field.clone();
                    }
                )*)?
                $($(
                    self.$nest_field.merge(&overlay.$nest_field);
                )*)?
                $($(
                    match (&mut self.$on_field, &overlay.$on_field) {
                        (Some(base), Some(over)) => base.merge(over),
                        (None, Some(over)) => self.$on_field = Some(over.clone()),
                        _ => {}
                    }
                )*)?
            }

            /// Returns true if all fields are at their default (None/empty) state.
            pub fn is_empty(&self) -> bool {
                true
                $($(&& self.$opt_field.is_none())*)?
                $($(&& self.$nest_field.is_empty())*)?
                $($(&& self.$on_field.is_none())*)?
            }
        }
    };
}

/// Color types and sRGB utilities.
pub mod color;
/// Error types for theme operations.
pub mod error;
/// GNOME portal theme reader.
#[cfg(all(target_os = "linux", feature = "portal"))]
pub mod gnome;
/// KDE theme reader.
#[cfg(all(target_os = "linux", feature = "kde"))]
pub mod kde;
/// Theme data model types.
pub mod model;
/// Bundled theme presets.
pub mod presets;
/// Theme resolution engine (inheritance + validation).
mod resolve;
#[cfg(any(
    feature = "material-icons",
    feature = "lucide-icons",
    feature = "system-icons"
))]
mod spinners;

pub use color::{ParseColorError, Rgba};
pub use error::{Error, ThemeResolutionError};
pub use model::{
    AnimatedIcon, ButtonTheme, CardTheme, CheckboxTheme, ComboBoxTheme, DialogButtonOrder,
    DialogTheme, ExpanderTheme, FontSpec, IconData, IconProvider, IconRole, IconSet, IconSizes,
    InputTheme, LinkTheme, ListTheme, MenuTheme, PopoverTheme, ProgressBarTheme, ResolvedFontSpec,
    ResolvedIconSizes, ResolvedTextScale, ResolvedTextScaleEntry, ResolvedThemeDefaults,
    ResolvedThemeSpacing, ResolvedThemeVariant, ScrollbarTheme, SegmentedControlTheme,
    SeparatorTheme, SidebarTheme, SliderTheme, SpinnerTheme, SplitterTheme, StatusBarTheme,
    SwitchTheme, TabTheme, TextScale, TextScaleEntry, ThemeDefaults, ThemeSpacing, ThemeSpec,
    ThemeVariant, ToolbarTheme, TooltipTheme, TransformAnimation, WindowTheme,
    bundled_icon_by_name, bundled_icon_svg,
};
// icon helper functions re-exported from this module
pub use model::icons::{icon_name, system_icon_set, system_icon_theme};

/// Freedesktop icon theme lookup (Linux).
#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub mod freedesktop;
/// macOS platform helpers.
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(not(target_os = "macos"))]
pub(crate) mod macos;
/// SVG-to-RGBA rasterization utilities.
#[cfg(feature = "svg-rasterize")]
pub mod rasterize;
/// SF Symbols icon loader (macOS).
#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub mod sficons;
/// Windows platform theme reader.
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(not(target_os = "windows"))]
#[allow(dead_code, unused_variables)]
pub(crate) mod windows;
/// Windows Segoe Fluent / stock icon loader.
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub mod winicons;
#[cfg(all(not(target_os = "windows"), feature = "system-icons"))]
#[allow(dead_code, unused_imports)]
pub(crate) mod winicons;

#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub use freedesktop::{load_freedesktop_icon, load_freedesktop_icon_by_name};
#[cfg(all(target_os = "linux", feature = "portal"))]
pub use gnome::from_gnome;
#[cfg(all(target_os = "linux", feature = "portal", feature = "kde"))]
pub use gnome::from_kde_with_portal;
#[cfg(all(target_os = "linux", feature = "kde"))]
pub use kde::from_kde;
#[cfg(all(target_os = "macos", feature = "macos"))]
pub use macos::from_macos;
#[cfg(feature = "svg-rasterize")]
pub use rasterize::rasterize_svg;
#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub use sficons::load_sf_icon;
#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub use sficons::load_sf_icon_by_name;
#[cfg(all(target_os = "windows", feature = "windows"))]
pub use windows::from_windows;
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub use winicons::load_windows_icon;
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub use winicons::load_windows_icon_by_name;

/// Convenience Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Desktop environments recognized on Linux.
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinuxDesktop {
    /// KDE Plasma desktop.
    Kde,
    /// GNOME desktop.
    Gnome,
    /// Xfce desktop.
    Xfce,
    /// Cinnamon desktop (Linux Mint).
    Cinnamon,
    /// MATE desktop.
    Mate,
    /// LXQt desktop.
    LxQt,
    /// Budgie desktop.
    Budgie,
    /// Unrecognized or unset desktop environment.
    Unknown,
}

/// Parse `XDG_CURRENT_DESKTOP` (a colon-separated list) and return
/// the recognized desktop environment.
///
/// Checks components in order; first recognized DE wins. Budgie is checked
/// before GNOME because Budgie sets `Budgie:GNOME`.
#[cfg(target_os = "linux")]
pub fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop {
    for component in xdg_current_desktop.split(':') {
        match component {
            "KDE" => return LinuxDesktop::Kde,
            "Budgie" => return LinuxDesktop::Budgie,
            "GNOME" => return LinuxDesktop::Gnome,
            "XFCE" => return LinuxDesktop::Xfce,
            "X-Cinnamon" | "Cinnamon" => return LinuxDesktop::Cinnamon,
            "MATE" => return LinuxDesktop::Mate,
            "LXQt" => return LinuxDesktop::LxQt,
            _ => {}
        }
    }
    LinuxDesktop::Unknown
}

/// Detect whether the system is using a dark color scheme.
///
/// Uses synchronous, platform-specific checks so the result is available
/// immediately at window creation time (before any async portal response).
///
/// # Caching
///
/// The result is cached after the first call using `OnceLock` and never
/// refreshed. If the user toggles dark mode while the app is running,
/// this function will return stale data.
///
/// For live dark-mode tracking, subscribe to OS appearance-change events
/// (D-Bus `SettingChanged` on Linux, `NSAppearance` KVO on macOS,
/// `UISettings.ColorValuesChanged` on Windows) and call [`SystemTheme::from_system()`]
/// to get a fresh [`SystemTheme`] with updated resolved variants.
///
/// # Platform Behavior
///
/// - **Linux:** Queries `gsettings` for `color-scheme` via subprocess;
///   falls back to KDE `kdeglobals` background luminance (with `kde`
///   feature).
/// - **macOS:** Reads `AppleInterfaceStyle` via `NSUserDefaults` (with
///   `macos` feature) or `defaults` subprocess (without).
/// - **Windows:** Checks foreground color luminance from `UISettings` via
///   BT.601 coefficients (requires `windows` feature).
/// - **Other platforms / missing features:** Returns `false` (light).
#[must_use = "this returns whether the system uses dark mode"]
pub fn system_is_dark() -> bool {
    static CACHED_IS_DARK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHED_IS_DARK.get_or_init(detect_is_dark_inner)
}

/// Inner detection logic for [`system_is_dark()`].
///
/// Separated from the public function to allow caching via `OnceLock`.
#[allow(unreachable_code)]
fn detect_is_dark_inner() -> bool {
    #[cfg(target_os = "linux")]
    {
        // gsettings works across all modern DEs (GNOME, KDE, XFCE, …)
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "color-scheme"])
            .output()
            && output.status.success()
        {
            let val = String::from_utf8_lossy(&output.stdout);
            if val.contains("prefer-dark") {
                return true;
            }
            if val.contains("prefer-light") || val.contains("default") {
                return false;
            }
        }

        // Fallback: read KDE's kdeglobals background luminance
        #[cfg(feature = "kde")]
        {
            let path = crate::kde::kdeglobals_path();
            if let Ok(content) = std::fs::read_to_string(&path) {
                let mut ini = crate::kde::create_kde_parser();
                if ini.read(content).is_ok() {
                    return crate::kde::is_dark_theme(&ini);
                }
            }
        }

        false
    }

    #[cfg(target_os = "macos")]
    {
        // AppleInterfaceStyle is "Dark" when dark mode is active.
        // The key is absent in light mode, so any failure means light.
        #[cfg(feature = "macos")]
        {
            use objc2_foundation::NSUserDefaults;
            let defaults = NSUserDefaults::standardUserDefaults();
            let key = objc2_foundation::ns_string!("AppleInterfaceStyle");
            if let Some(value) = unsafe { defaults.stringForKey(key) } {
                return value.to_string().eq_ignore_ascii_case("dark");
            }
            return false;
        }
        #[cfg(not(feature = "macos"))]
        {
            if let Ok(output) = std::process::Command::new("defaults")
                .args(["read", "-g", "AppleInterfaceStyle"])
                .output()
                && output.status.success()
            {
                let val = String::from_utf8_lossy(&output.stdout);
                return val.trim().eq_ignore_ascii_case("dark");
            }
            return false;
        }
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            // BT.601 luminance: light foreground indicates dark background.
            let Ok(settings) = ::windows::UI::ViewManagement::UISettings::new() else {
                return false;
            };
            let Ok(fg) =
                settings.GetColorValue(::windows::UI::ViewManagement::UIColorType::Foreground)
            else {
                return false;
            };
            let luma = 0.299 * (fg.R as f32) + 0.587 * (fg.G as f32) + 0.114 * (fg.B as f32);
            return luma > 128.0;
        }
        #[cfg(not(feature = "windows"))]
        return false;
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

/// Query whether the user prefers reduced motion.
///
/// Returns `true` when the OS accessibility setting indicates animations
/// should be reduced or disabled. Returns `false` (allow animations) on
/// unsupported platforms or when the query fails.
///
/// # Caching
///
/// The result is cached after the first call using `OnceLock` and never
/// refreshed. For live accessibility-change tracking, subscribe to OS
/// accessibility events and re-query as needed.
///
/// # Platform Behavior
///
/// - **Linux:** Queries `gsettings get org.gnome.desktop.interface enable-animations`.
///   Returns `true` when animations are disabled (`enable-animations` is `false`).
/// - **macOS:** Queries `NSWorkspace.accessibilityDisplayShouldReduceMotion`
///   (requires `macos` feature).
/// - **Windows:** Queries `UISettings.AnimationsEnabled()` (requires `windows` feature).
/// - **Other platforms:** Returns `false`.
///
/// # Examples
///
/// ```
/// let reduced = native_theme::prefers_reduced_motion();
/// // On this platform, the result depends on OS accessibility settings.
/// // The function always returns a bool (false on unsupported platforms).
/// assert!(reduced == true || reduced == false);
/// ```
#[must_use = "this returns whether reduced motion is preferred"]
pub fn prefers_reduced_motion() -> bool {
    static CACHED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHED.get_or_init(detect_reduced_motion_inner)
}

/// Inner detection logic for [`prefers_reduced_motion()`].
///
/// Separated from the public function to allow caching via `OnceLock`.
#[allow(unreachable_code)]
fn detect_reduced_motion_inner() -> bool {
    #[cfg(target_os = "linux")]
    {
        // gsettings boolean output is bare "true\n" or "false\n" (no quotes)
        // enable-animations has INVERTED semantics: false => reduced motion preferred
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "enable-animations"])
            .output()
            && output.status.success()
        {
            let val = String::from_utf8_lossy(&output.stdout);
            return val.trim() == "false";
        }
        false
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            let workspace = objc2_app_kit::NSWorkspace::sharedWorkspace();
            // Direct semantics: true = reduce motion preferred (no inversion needed)
            return workspace.accessibilityDisplayShouldReduceMotion();
        }
        #[cfg(not(feature = "macos"))]
        return false;
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            let Ok(settings) = ::windows::UI::ViewManagement::UISettings::new() else {
                return false;
            };
            // AnimationsEnabled has INVERTED semantics: false => reduced motion preferred
            return match settings.AnimationsEnabled() {
                Ok(enabled) => !enabled,
                Err(_) => false,
            };
        }
        #[cfg(not(feature = "windows"))]
        return false;
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

/// Result of the OS-first pipeline. Holds both resolved variants.
///
/// Produced by [`SystemTheme::from_system()`] and [`SystemTheme::from_system_async()`].
/// Both light and dark are always populated: the OS-active variant
/// comes from the reader + preset + resolve, the inactive variant
/// comes from the preset + resolve.
#[derive(Clone, Debug)]
pub struct SystemTheme {
    /// Theme name (from reader or preset).
    pub name: String,
    /// Whether the OS is currently in dark mode.
    pub is_dark: bool,
    /// Resolved light variant (always populated).
    pub light: ResolvedThemeVariant,
    /// Resolved dark variant (always populated).
    pub dark: ResolvedThemeVariant,
    /// Pre-resolve light variant (retained for overlay support).
    pub(crate) light_variant: ThemeVariant,
    /// Pre-resolve dark variant (retained for overlay support).
    pub(crate) dark_variant: ThemeVariant,
    /// The platform preset used (e.g., "kde-breeze", "adwaita", "macos-sonoma").
    pub preset: String,
    /// The live preset name used internally (e.g., "kde-breeze-live").
    pub(crate) live_preset: String,
}

impl SystemTheme {
    /// Returns the OS-active resolved variant.
    ///
    /// If `is_dark` is true, returns `&self.dark`; otherwise `&self.light`.
    pub fn active(&self) -> &ResolvedThemeVariant {
        if self.is_dark {
            &self.dark
        } else {
            &self.light
        }
    }

    /// Pick a resolved variant by explicit preference.
    ///
    /// `pick(true)` returns `&self.dark`, `pick(false)` returns `&self.light`.
    pub fn pick(&self, is_dark: bool) -> &ResolvedThemeVariant {
        if is_dark { &self.dark } else { &self.light }
    }

    /// Apply an app-level TOML overlay and re-resolve.
    ///
    /// Merges the overlay onto the pre-resolve [`ThemeVariant`] (not the
    /// already-resolved [`ResolvedThemeVariant`]) so that changed source fields
    /// propagate correctly through `resolve()`. For example, changing
    /// `defaults.accent` in the overlay will cause `button.primary_bg`,
    /// `checkbox.checked_bg`, `slider.fill`, etc. to be re-derived from
    /// the new accent color.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let system = native_theme::SystemTheme::from_system().unwrap();
    /// let overlay = native_theme::ThemeSpec::from_toml(r##"
    ///     [light.defaults]
    ///     accent = "#ff6600"
    ///     [dark.defaults]
    ///     accent = "#ff6600"
    /// "##).unwrap();
    /// let customized = system.with_overlay(&overlay).unwrap();
    /// // customized.active().defaults.accent is now #ff6600
    /// // and all accent-derived fields are updated
    /// ```
    pub fn with_overlay(&self, overlay: &ThemeSpec) -> crate::Result<Self> {
        // Start from pre-resolve variants (avoids double-resolve idempotency issue)
        let mut light = self.light_variant.clone();
        let mut dark = self.dark_variant.clone();

        // Merge overlay onto pre-resolve variants (overlay values win)
        if let Some(over) = &overlay.light {
            light.merge(over);
        }
        if let Some(over) = &overlay.dark {
            dark.merge(over);
        }

        // Resolve and validate both
        let resolved_light = light.clone().into_resolved()?;
        let resolved_dark = dark.clone().into_resolved()?;

        Ok(SystemTheme {
            name: self.name.clone(),
            is_dark: self.is_dark,
            light: resolved_light,
            dark: resolved_dark,
            light_variant: light,
            dark_variant: dark,
            live_preset: self.live_preset.clone(),
            preset: self.preset.clone(),
        })
    }

    /// Apply an app overlay from a TOML string.
    ///
    /// Parses the TOML as a [`ThemeSpec`] and calls [`with_overlay`](Self::with_overlay).
    pub fn with_overlay_toml(&self, toml: &str) -> crate::Result<Self> {
        let overlay = ThemeSpec::from_toml(toml)?;
        self.with_overlay(&overlay)
    }

    /// Load the OS theme synchronously.
    ///
    /// Detects the platform and desktop environment, reads the current theme
    /// settings, merges with a platform preset, and returns a fully resolved
    /// [`SystemTheme`] with both light and dark variants.
    ///
    /// The return value goes through the full pipeline: reader output →
    /// resolve → validate → [`SystemTheme`] with both light and dark
    /// [`ResolvedThemeVariant`] variants.
    ///
    /// # Platform Behavior
    ///
    /// - **macOS:** Calls `from_macos()` when the `macos` feature is enabled.
    ///   Reads both light and dark variants via NSAppearance, merges with
    ///   `macos-sonoma` preset.
    /// - **Linux (KDE):** Calls `from_kde()` when `XDG_CURRENT_DESKTOP` contains
    ///   "KDE" and the `kde` feature is enabled, merges with `kde-breeze` preset.
    /// - **Linux (other):** Uses the `adwaita` preset. For live GNOME portal
    ///   data, use [`from_system_async()`](Self::from_system_async) (requires
    ///   `portal-tokio` or `portal-async-io` feature).
    /// - **Windows:** Calls `from_windows()` when the `windows` feature is enabled,
    ///   merges with `windows-11` preset.
    /// - **Other platforms:** Returns `Error::Unsupported`.
    ///
    /// # Errors
    ///
    /// - `Error::Unsupported` if the platform has no reader or the required feature
    ///   is not enabled.
    /// - `Error::Unavailable` if the platform reader cannot access theme data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let system = native_theme::SystemTheme::from_system().unwrap();
    /// let active = system.active();
    /// ```
    #[must_use = "this returns the detected theme; it does not apply it"]
    pub fn from_system() -> crate::Result<Self> {
        from_system_inner()
    }

    /// Async version of [`from_system()`](Self::from_system) that uses D-Bus
    /// portal backend detection to improve desktop environment heuristics on
    /// Linux.
    ///
    /// When `XDG_CURRENT_DESKTOP` is unset or unrecognized, queries the
    /// D-Bus session bus for portal backend activatable names to determine
    /// whether KDE or GNOME portal is running, then dispatches to the
    /// appropriate reader.
    ///
    /// Returns a [`SystemTheme`] with both resolved light and dark variants,
    /// same as [`from_system()`](Self::from_system).
    ///
    /// On non-Linux platforms, behaves identically to
    /// [`from_system()`](Self::from_system).
    #[cfg(target_os = "linux")]
    #[must_use = "this returns the detected theme; it does not apply it"]
    pub async fn from_system_async() -> crate::Result<Self> {
        from_system_async_inner().await
    }

    /// Async version of [`from_system()`](Self::from_system).
    ///
    /// On non-Linux platforms, this is equivalent to calling
    /// [`from_system()`](Self::from_system).
    #[cfg(not(target_os = "linux"))]
    #[must_use = "this returns the detected theme; it does not apply it"]
    pub async fn from_system_async() -> crate::Result<Self> {
        from_system_inner()
    }
}

/// Run the OS-first pipeline: merge reader output onto a platform
/// preset, resolve both light and dark variants, validate.
///
/// For the variant the reader supplied, the merged (reader + live preset)
/// version is used. For the variant the reader did NOT supply, the full
/// platform preset (with colors/fonts) is used as fallback.
fn run_pipeline(
    reader_output: ThemeSpec,
    preset_name: &str,
    is_dark: bool,
) -> crate::Result<SystemTheme> {
    let live_preset = ThemeSpec::preset(preset_name)?;

    // For the inactive variant, load the full preset (with colors)
    let full_preset_name = preset_name.strip_suffix("-live").unwrap_or(preset_name);
    let full_preset = ThemeSpec::preset(full_preset_name)?;

    // Merge: full preset provides color/font defaults, live preset overrides
    // geometry, reader output provides live OS data on top.
    let mut merged = full_preset.clone();
    merged.merge(&live_preset);
    merged.merge(&reader_output);

    // Keep reader name if non-empty, else use preset name
    let name = if reader_output.name.is_empty() {
        merged.name.clone()
    } else {
        reader_output.name.clone()
    };

    // For the variant the reader provided: use merged (live geometry + reader colors)
    // For the variant the reader didn't provide: use FULL preset (has colors)
    let light_variant = if reader_output.light.is_some() {
        merged.light.unwrap_or_default()
    } else {
        full_preset.light.unwrap_or_default()
    };

    let dark_variant = if reader_output.dark.is_some() {
        merged.dark.unwrap_or_default()
    } else {
        full_preset.dark.unwrap_or_default()
    };

    // Clone pre-resolve variants for overlay support (Plan 02)
    let light_variant_pre = light_variant.clone();
    let dark_variant_pre = dark_variant.clone();

    let light = light_variant.into_resolved()?;
    let dark = dark_variant.into_resolved()?;

    Ok(SystemTheme {
        name,
        is_dark,
        light,
        dark,
        light_variant: light_variant_pre,
        dark_variant: dark_variant_pre,
        preset: full_preset_name.to_string(),
        live_preset: preset_name.to_string(),
    })
}

/// Map the current platform to its matching live preset name.
///
/// Live presets contain only geometry/metrics (no colors, fonts, or icons)
/// and are used as the merge base in the OS-first pipeline.
///
/// - macOS -> `"macos-sonoma-live"`
/// - Windows -> `"windows-11-live"`
/// - Linux KDE -> `"kde-breeze-live"`
/// - Linux other/GNOME -> `"adwaita-live"`
/// - Unknown platform -> `"adwaita-live"`
///
/// Returns the live preset name for the current platform.
///
/// This is the public API for what [`SystemTheme::from_system()`] uses internally.
/// Showcase UIs use this to build the "default (...)" label.
#[allow(unreachable_code)]
pub fn platform_preset_name() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        return "macos-sonoma-live";
    }
    #[cfg(target_os = "windows")]
    {
        return "windows-11-live";
    }
    #[cfg(target_os = "linux")]
    {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        match detect_linux_de(&desktop) {
            LinuxDesktop::Kde => "kde-breeze-live",
            _ => "adwaita-live",
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        "adwaita-live"
    }
}

/// Infer dark-mode preference from the reader's output.
///
/// Returns `true` if the reader populated only the dark variant,
/// `false` if it populated only light or both variants.
/// On platforms that produce both variants (macOS), this defaults to
/// `false` (light); callers can use [`SystemTheme::pick()`] for
/// explicit variant selection regardless of this default.
#[allow(dead_code)]
fn reader_is_dark(reader: &ThemeSpec) -> bool {
    reader.dark.is_some() && reader.light.is_none()
}

/// Read the current system theme on Linux by detecting the desktop
/// environment and calling the appropriate reader or returning a
/// preset fallback.
///
/// Runs the full OS-first pipeline: reader -> preset merge -> resolve -> validate.
#[cfg(target_os = "linux")]
fn from_linux() -> crate::Result<SystemTheme> {
    let is_dark = system_is_dark();
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    match detect_linux_de(&desktop) {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => {
            let reader = crate::kde::from_kde()?;
            run_pipeline(reader, "kde-breeze-live", is_dark)
        }
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark),
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            // GNOME sync path: no portal, just adwaita preset
            run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark)
        }
        LinuxDesktop::Xfce | LinuxDesktop::Cinnamon | LinuxDesktop::Mate | LinuxDesktop::LxQt => {
            run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark)
        }
        LinuxDesktop::Unknown => {
            #[cfg(feature = "kde")]
            {
                let path = crate::kde::kdeglobals_path();
                if path.exists() {
                    let reader = crate::kde::from_kde()?;
                    return run_pipeline(reader, "kde-breeze-live", is_dark);
                }
            }
            run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark)
        }
    }
}

fn from_system_inner() -> crate::Result<SystemTheme> {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        {
            let reader = crate::macos::from_macos()?;
            let is_dark = reader_is_dark(&reader);
            return run_pipeline(reader, "macos-sonoma-live", is_dark);
        }

        #[cfg(not(feature = "macos"))]
        return Err(crate::Error::Unsupported);
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        {
            let reader = crate::windows::from_windows()?;
            let is_dark = reader_is_dark(&reader);
            return run_pipeline(reader, "windows-11-live", is_dark);
        }

        #[cfg(not(feature = "windows"))]
        return Err(crate::Error::Unsupported);
    }

    #[cfg(target_os = "linux")]
    {
        from_linux()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Err(crate::Error::Unsupported)
    }
}

#[cfg(target_os = "linux")]
async fn from_system_async_inner() -> crate::Result<SystemTheme> {
    let is_dark = system_is_dark();
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    match detect_linux_de(&desktop) {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => {
            #[cfg(feature = "portal")]
            {
                let reader = crate::gnome::from_kde_with_portal().await?;
                run_pipeline(reader, "kde-breeze-live", is_dark)
            }
            #[cfg(not(feature = "portal"))]
            {
                let reader = crate::kde::from_kde()?;
                run_pipeline(reader, "kde-breeze-live", is_dark)
            }
        }
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark),
        #[cfg(feature = "portal")]
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            let reader = crate::gnome::from_gnome().await?;
            run_pipeline(reader, "adwaita-live", is_dark)
        }
        #[cfg(not(feature = "portal"))]
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => {
            run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark)
        }
        LinuxDesktop::Xfce | LinuxDesktop::Cinnamon | LinuxDesktop::Mate | LinuxDesktop::LxQt => {
            run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark)
        }
        LinuxDesktop::Unknown => {
            // Use D-Bus portal backend detection to refine heuristic
            #[cfg(feature = "portal")]
            {
                if let Some(detected) = crate::gnome::detect_portal_backend().await {
                    return match detected {
                        #[cfg(feature = "kde")]
                        LinuxDesktop::Kde => {
                            let reader = crate::gnome::from_kde_with_portal().await?;
                            run_pipeline(reader, "kde-breeze-live", is_dark)
                        }
                        #[cfg(not(feature = "kde"))]
                        LinuxDesktop::Kde => {
                            run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark)
                        }
                        LinuxDesktop::Gnome => {
                            let reader = crate::gnome::from_gnome().await?;
                            run_pipeline(reader, "adwaita-live", is_dark)
                        }
                        _ => {
                            unreachable!("detect_portal_backend only returns Kde or Gnome")
                        }
                    };
                }
            }
            // Sync fallback: try kdeglobals, then Adwaita
            #[cfg(feature = "kde")]
            {
                let path = crate::kde::kdeglobals_path();
                if path.exists() {
                    let reader = crate::kde::from_kde()?;
                    return run_pipeline(reader, "kde-breeze-live", is_dark);
                }
            }
            run_pipeline(ThemeSpec::preset("adwaita")?, "adwaita-live", is_dark)
        }
    }
}

/// Load an icon for the given role using the specified icon set.
///
/// Resolves `icon_set` to an [`IconSet`] via [`IconSet::from_name()`],
/// falling back to [`system_icon_set()`] if the set string is not
/// recognized. Then dispatches to the appropriate platform loader or
/// bundled icon set.
///
/// # Dispatch
///
/// 1. Platform loader (freedesktop/sf-symbols/segoe-fluent) when `system-icons` enabled
/// 2. Bundled SVGs (material/lucide) when the corresponding feature is enabled
/// 3. Non-matching set: `None` (no cross-set fallback)
///
/// Freedesktop icons are loaded at 24 px (the standard toolbar size). For
/// custom sizes, call `freedesktop::load_freedesktop_icon()` directly.
///
/// # Examples
///
/// ```
/// use native_theme::{load_icon, IconRole, IconSet};
///
/// // With material-icons feature enabled
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = load_icon(IconRole::ActionCopy, IconSet::Material);
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use = "this returns the loaded icon data; it does not display it"]
#[allow(unreachable_patterns, clippy::needless_return, unused_variables)]
pub fn load_icon(role: IconRole, set: IconSet) -> Option<IconData> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => freedesktop::load_freedesktop_icon(role, 24),

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => sficons::load_sf_icon(role),

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => winicons::load_windows_icon(role),

        #[cfg(feature = "material-icons")]
        IconSet::Material => {
            bundled_icon_svg(role, IconSet::Material).map(|b| IconData::Svg(b.to_vec()))
        }

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => {
            bundled_icon_svg(role, IconSet::Lucide).map(|b| IconData::Svg(b.to_vec()))
        }

        // Non-matching platform or unknown set: no cross-set fallback
        _ => None,
    }
}

/// Load a system icon by its platform-specific name string.
///
/// Dispatches to the appropriate platform loader based on the icon set:
/// - [`IconSet::Freedesktop`] -- freedesktop icon theme lookup (auto-detects theme)
/// - [`IconSet::SfSymbols`] -- macOS SF Symbols
/// - [`IconSet::SegoeIcons`] -- Windows Segoe Fluent / stock icons
/// - [`IconSet::Material`] / [`IconSet::Lucide`] -- bundled SVG lookup by name
///
/// Returns `None` if the icon is not found on the current platform or
/// the icon set is not available.
///
/// # Examples
///
/// ```
/// use native_theme::{load_system_icon_by_name, IconSet};
///
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = load_system_icon_by_name("content_copy", IconSet::Material);
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use = "this returns the loaded icon data; it does not display it"]
#[allow(unreachable_patterns, unused_variables)]
pub fn load_system_icon_by_name(name: &str, set: IconSet) -> Option<IconData> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => {
            let theme = system_icon_theme();
            freedesktop::load_freedesktop_icon_by_name(name, theme, 24)
        }

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => sficons::load_sf_icon_by_name(name),

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => winicons::load_windows_icon_by_name(name),

        #[cfg(feature = "material-icons")]
        IconSet::Material => {
            bundled_icon_by_name(name, IconSet::Material).map(|b| IconData::Svg(b.to_vec()))
        }

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => {
            bundled_icon_by_name(name, IconSet::Lucide).map(|b| IconData::Svg(b.to_vec()))
        }

        _ => None,
    }
}

/// Return the loading/spinner animation for the given icon set.
///
/// This is the animated-icon counterpart of [`load_icon()`].
///
/// # Dispatch
///
/// - [`IconSet::Material`] -- `progress_activity.svg` with continuous spin transform (1000ms)
/// - [`IconSet::Lucide`] -- `loader.svg` with continuous spin transform (1000ms)
/// - [`IconSet::Freedesktop`] -- loads `process-working` sprite sheet from active icon theme
/// - Other sets -- `None`
///
/// # Examples
///
/// ```
/// // Result depends on enabled features and platform
/// let anim = native_theme::loading_indicator(native_theme::IconSet::Lucide);
/// # #[cfg(feature = "lucide-icons")]
/// # assert!(anim.is_some());
/// ```
#[must_use = "this returns animation data; it does not display anything"]
pub fn loading_indicator(set: IconSet) -> Option<AnimatedIcon> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => freedesktop::load_freedesktop_spinner(),

        #[cfg(feature = "material-icons")]
        IconSet::Material => Some(spinners::material_spinner()),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => Some(spinners::lucide_spinner()),

        _ => None,
    }
}

/// Load an icon from any [`IconProvider`], dispatching through the standard
/// platform loading chain.
///
/// # Fallback chain
///
/// 1. Provider's [`icon_name()`](IconProvider::icon_name) -- passed to platform
///    system loader via [`load_system_icon_by_name()`]
/// 2. Provider's [`icon_svg()`](IconProvider::icon_svg) -- bundled SVG data
/// 3. `None` -- **no cross-set fallback** (mixing icon sets is forbidden)
///
/// # Examples
///
/// ```
/// use native_theme::{load_custom_icon, IconRole, IconSet};
///
/// // IconRole implements IconProvider, so it works with load_custom_icon
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = load_custom_icon(&IconRole::ActionCopy, IconSet::Material);
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use = "this returns the loaded icon data; it does not display it"]
pub fn load_custom_icon(provider: &(impl IconProvider + ?Sized), set: IconSet) -> Option<IconData> {
    // Step 1: Try system loader with provider's name mapping
    if let Some(name) = provider.icon_name(set)
        && let Some(data) = load_system_icon_by_name(name, set)
    {
        return Some(data);
    }

    // Step 2: Try bundled SVG from provider
    if let Some(svg) = provider.icon_svg(set) {
        return Some(IconData::Svg(svg.to_vec()));
    }

    // No cross-set fallback -- return None
    None
}

/// Mutex to serialize tests that manipulate environment variables.
/// Env vars are process-global state, so tests that call set_var/remove_var
/// must hold this lock to avoid races with parallel test execution.
#[cfg(test)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(all(test, target_os = "linux"))]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod dispatch_tests {
    use super::*;

    // -- detect_linux_de() pure function tests --

    #[test]
    fn detect_kde_simple() {
        assert_eq!(detect_linux_de("KDE"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_kde_colon_separated_after() {
        assert_eq!(detect_linux_de("ubuntu:KDE"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_kde_colon_separated_before() {
        assert_eq!(detect_linux_de("KDE:plasma"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_gnome_simple() {
        assert_eq!(detect_linux_de("GNOME"), LinuxDesktop::Gnome);
    }

    #[test]
    fn detect_gnome_ubuntu() {
        assert_eq!(detect_linux_de("ubuntu:GNOME"), LinuxDesktop::Gnome);
    }

    #[test]
    fn detect_xfce() {
        assert_eq!(detect_linux_de("XFCE"), LinuxDesktop::Xfce);
    }

    #[test]
    fn detect_cinnamon() {
        assert_eq!(detect_linux_de("X-Cinnamon"), LinuxDesktop::Cinnamon);
    }

    #[test]
    fn detect_cinnamon_short() {
        assert_eq!(detect_linux_de("Cinnamon"), LinuxDesktop::Cinnamon);
    }

    #[test]
    fn detect_mate() {
        assert_eq!(detect_linux_de("MATE"), LinuxDesktop::Mate);
    }

    #[test]
    fn detect_lxqt() {
        assert_eq!(detect_linux_de("LXQt"), LinuxDesktop::LxQt);
    }

    #[test]
    fn detect_budgie() {
        assert_eq!(detect_linux_de("Budgie:GNOME"), LinuxDesktop::Budgie);
    }

    #[test]
    fn detect_empty_string() {
        assert_eq!(detect_linux_de(""), LinuxDesktop::Unknown);
    }

    // -- from_linux() fallback test --

    #[test]
    #[allow(unsafe_code)]
    fn from_linux_non_kde_returns_adwaita() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        // Temporarily set XDG_CURRENT_DESKTOP to GNOME so from_linux()
        // takes the preset fallback path.
        // SAFETY: ENV_MUTEX serializes env var access across parallel tests
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "GNOME") };
        let result = from_linux();
        unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") };

        let theme = result.expect("from_linux() should return Ok for non-KDE desktop");
        assert_eq!(theme.name, "Adwaita");
    }

    // -- from_linux() kdeglobals fallback tests --

    #[test]
    #[cfg(feature = "kde")]
    #[allow(unsafe_code)]
    fn from_linux_unknown_de_with_kdeglobals_fallback() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        use std::io::Write;

        // Create a temp dir with a minimal kdeglobals file
        let tmp_dir = std::env::temp_dir().join("native_theme_test_kde_fallback");
        std::fs::create_dir_all(&tmp_dir).unwrap();
        let kdeglobals = tmp_dir.join("kdeglobals");
        let mut f = std::fs::File::create(&kdeglobals).unwrap();
        writeln!(
            f,
            "[General]\nColorScheme=TestTheme\n\n[Colors:Window]\nBackgroundNormal=239,240,241\n"
        )
        .unwrap();

        // SAFETY: ENV_MUTEX serializes env var access across parallel tests
        let orig_xdg = std::env::var("XDG_CONFIG_HOME").ok();
        let orig_desktop = std::env::var("XDG_CURRENT_DESKTOP").ok();

        unsafe { std::env::set_var("XDG_CONFIG_HOME", &tmp_dir) };
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "SomeUnknownDE") };

        let result = from_linux();

        // Restore env
        match orig_xdg {
            Some(val) => unsafe { std::env::set_var("XDG_CONFIG_HOME", val) },
            None => unsafe { std::env::remove_var("XDG_CONFIG_HOME") },
        }
        match orig_desktop {
            Some(val) => unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", val) },
            None => unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") },
        }

        // Cleanup
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let theme = result.expect("from_linux() should return Ok with kdeglobals fallback");
        assert_eq!(
            theme.name, "TestTheme",
            "should use KDE theme name from kdeglobals"
        );
    }

    #[test]
    #[allow(unsafe_code)]
    fn from_linux_unknown_de_without_kdeglobals_returns_adwaita() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        // SAFETY: ENV_MUTEX serializes env var access across parallel tests
        let orig_xdg = std::env::var("XDG_CONFIG_HOME").ok();
        let orig_desktop = std::env::var("XDG_CURRENT_DESKTOP").ok();

        unsafe {
            std::env::set_var(
                "XDG_CONFIG_HOME",
                "/tmp/nonexistent_native_theme_test_no_kde",
            )
        };
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "SomeUnknownDE") };

        let result = from_linux();

        // Restore env
        match orig_xdg {
            Some(val) => unsafe { std::env::set_var("XDG_CONFIG_HOME", val) },
            None => unsafe { std::env::remove_var("XDG_CONFIG_HOME") },
        }
        match orig_desktop {
            Some(val) => unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", val) },
            None => unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") },
        }

        let theme = result.expect("from_linux() should return Ok (adwaita fallback)");
        assert_eq!(
            theme.name, "Adwaita",
            "should fall back to Adwaita without kdeglobals"
        );
    }

    // -- LNXDE-03: Hyprland, Sway, COSMIC map to Unknown --

    #[test]
    fn detect_hyprland_returns_unknown() {
        assert_eq!(detect_linux_de("Hyprland"), LinuxDesktop::Unknown);
    }

    #[test]
    fn detect_sway_returns_unknown() {
        assert_eq!(detect_linux_de("sway"), LinuxDesktop::Unknown);
    }

    #[test]
    fn detect_cosmic_returns_unknown() {
        assert_eq!(detect_linux_de("COSMIC"), LinuxDesktop::Unknown);
    }

    // -- from_system() smoke test --

    #[test]
    #[allow(unsafe_code)]
    fn from_system_returns_result() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        // On Linux (our test platform), from_system() should return a Result.
        // With GNOME set, it should return the Adwaita preset.
        // SAFETY: ENV_MUTEX serializes env var access across parallel tests
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "GNOME") };
        let result = SystemTheme::from_system();
        unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") };

        let theme = result.expect("from_system() should return Ok on Linux");
        assert_eq!(theme.name, "Adwaita");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_icon_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_material_returns_svg() {
        let result = load_icon(IconRole::ActionCopy, IconSet::Material);
        assert!(result.is_some(), "material ActionCopy should return Some");
        match result.unwrap() {
            IconData::Svg(bytes) => {
                let content = std::str::from_utf8(&bytes).expect("should be valid UTF-8");
                assert!(content.contains("<svg"), "should contain SVG data");
            }
            _ => panic!("expected IconData::Svg for bundled material icon"),
        }
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn load_icon_lucide_returns_svg() {
        let result = load_icon(IconRole::ActionCopy, IconSet::Lucide);
        assert!(result.is_some(), "lucide ActionCopy should return Some");
        match result.unwrap() {
            IconData::Svg(bytes) => {
                let content = std::str::from_utf8(&bytes).expect("should be valid UTF-8");
                assert!(content.contains("<svg"), "should contain SVG data");
            }
            _ => panic!("expected IconData::Svg for bundled lucide icon"),
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_unknown_theme_no_cross_set_fallback() {
        // On Linux (test platform), unknown theme resolves to system_icon_set() = Freedesktop.
        // Without system-icons feature, Freedesktop falls through to wildcard -> None.
        // No cross-set Material fallback.
        let result = load_icon(IconRole::ActionCopy, IconSet::Freedesktop);
        // Without system-icons, this falls to wildcard which returns None
        // With system-icons, this dispatches to load_freedesktop_icon which may return Some
        // Either way, no panic
        let _ = result;
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_all_roles_material() {
        // Material has 42 of 42 roles mapped, all return Some
        let mut some_count = 0;
        for role in IconRole::ALL {
            if load_icon(role, IconSet::Material).is_some() {
                some_count += 1;
            }
        }
        // bundled_icon_svg covers all 42 roles for Material
        assert_eq!(
            some_count, 42,
            "Material should cover all 42 roles via bundled SVGs"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn load_icon_all_roles_lucide() {
        let mut some_count = 0;
        for role in IconRole::ALL {
            if load_icon(role, IconSet::Lucide).is_some() {
                some_count += 1;
            }
        }
        // bundled_icon_svg covers all 42 roles for Lucide
        assert_eq!(
            some_count, 42,
            "Lucide should cover all 42 roles via bundled SVGs"
        );
    }

    #[test]
    fn load_icon_unrecognized_set_no_features() {
        // SfSymbols on Linux without system-icons: falls through to wildcard -> None
        let _result = load_icon(IconRole::ActionCopy, IconSet::SfSymbols);
        // Just verifying it doesn't panic
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_system_icon_by_name_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn system_icon_by_name_material() {
        let result = load_system_icon_by_name("content_copy", IconSet::Material);
        assert!(
            result.is_some(),
            "content_copy should be found in Material set"
        );
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn system_icon_by_name_lucide() {
        let result = load_system_icon_by_name("copy", IconSet::Lucide);
        assert!(result.is_some(), "copy should be found in Lucide set");
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn system_icon_by_name_unknown_returns_none() {
        let result = load_system_icon_by_name("nonexistent_xyz", IconSet::Material);
        assert!(result.is_none(), "nonexistent name should return None");
    }

    #[test]
    fn system_icon_by_name_sf_on_linux_returns_none() {
        // On Linux, SfSymbols set is not available (cfg-gated to macOS)
        #[cfg(not(target_os = "macos"))]
        {
            let result = load_system_icon_by_name("doc.on.doc", IconSet::SfSymbols);
            assert!(
                result.is_none(),
                "SF Symbols should return None on non-macOS"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_custom_icon_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_with_icon_role_material() {
        let result = load_custom_icon(&IconRole::ActionCopy, IconSet::Material);
        assert!(
            result.is_some(),
            "IconRole::ActionCopy should load via material"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn custom_icon_with_icon_role_lucide() {
        let result = load_custom_icon(&IconRole::ActionCopy, IconSet::Lucide);
        assert!(
            result.is_some(),
            "IconRole::ActionCopy should load via lucide"
        );
    }

    #[test]
    fn custom_icon_no_cross_set_fallback() {
        // Provider that returns None for all sets -- should NOT fall back
        #[derive(Debug)]
        struct NullProvider;
        impl IconProvider for NullProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<&'static [u8]> {
                None
            }
        }

        let result = load_custom_icon(&NullProvider, IconSet::Material);
        assert!(
            result.is_none(),
            "NullProvider should return None (no cross-set fallback)"
        );
    }

    #[test]
    fn custom_icon_unknown_set_uses_system() {
        // "unknown-set" is not a known IconSet name, falls through to system_icon_set()
        #[derive(Debug)]
        struct NullProvider;
        impl IconProvider for NullProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<&'static [u8]> {
                None
            }
        }

        // Just verify it doesn't panic -- the actual set chosen depends on platform
        let _result = load_custom_icon(&NullProvider, IconSet::Freedesktop);
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_via_dyn_dispatch() {
        let boxed: Box<dyn IconProvider> = Box::new(IconRole::ActionCopy);
        let result = load_custom_icon(&*boxed, IconSet::Material);
        assert!(
            result.is_some(),
            "dyn dispatch through Box<dyn IconProvider> should work"
        );
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_bundled_svg_fallback() {
        // Provider that returns None from icon_name but Some from icon_svg
        #[derive(Debug)]
        struct SvgOnlyProvider;
        impl IconProvider for SvgOnlyProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<&'static [u8]> {
                Some(b"<svg>test</svg>")
            }
        }

        let result = load_custom_icon(&SvgOnlyProvider, IconSet::Material);
        assert!(
            result.is_some(),
            "provider with icon_svg should return Some"
        );
        match result.unwrap() {
            IconData::Svg(bytes) => {
                assert_eq!(bytes, b"<svg>test</svg>");
            }
            _ => panic!("expected IconData::Svg"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod loading_indicator_tests {
    use super::*;

    // === Dispatch tests (through loading_indicator public API) ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn loading_indicator_lucide_returns_transform_spin() {
        let anim = loading_indicator(IconSet::Lucide);
        assert!(anim.is_some(), "lucide should return Some");
        let anim = anim.unwrap();
        assert!(
            matches!(
                anim,
                AnimatedIcon::Transform {
                    animation: TransformAnimation::Spin { duration_ms: 1000 },
                    ..
                }
            ),
            "lucide should be Transform::Spin at 1000ms"
        );
    }

    /// Freedesktop loading_indicator returns Some if the active icon theme
    /// has a `process-working` sprite sheet (e.g. Breeze), None otherwise.
    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn loading_indicator_freedesktop_depends_on_theme() {
        let anim = loading_indicator(IconSet::Freedesktop);
        // Result depends on installed icon theme -- Some if process-working exists
        if let Some(anim) = anim {
            match anim {
                AnimatedIcon::Frames { frames, .. } => {
                    assert!(
                        !frames.is_empty(),
                        "Frames variant should have at least one frame"
                    );
                }
                AnimatedIcon::Transform { .. } => {
                    // Single-frame theme icon with Spin -- valid result
                }
            }
        }
    }

    /// Freedesktop spinner depends on platform and icon theme.
    #[test]
    fn loading_indicator_freedesktop_does_not_panic() {
        let _result = loading_indicator(IconSet::Freedesktop);
    }

    // === Direct spinner construction tests (any platform) ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_spinner_is_transform() {
        let anim = spinners::lucide_spinner();
        assert!(matches!(
            anim,
            AnimatedIcon::Transform {
                animation: TransformAnimation::Spin { duration_ms: 1000 },
                ..
            }
        ));
    }
}

#[cfg(all(test, feature = "svg-rasterize"))]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod spinner_rasterize_tests {
    use super::*;

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_spinner_icon_rasterizes() {
        let anim = spinners::lucide_spinner();
        if let AnimatedIcon::Transform { icon, .. } = &anim {
            if let IconData::Svg(bytes) = icon {
                let result = crate::rasterize::rasterize_svg(bytes, 24);
                assert!(result.is_some(), "lucide loader should rasterize");
                if let Some(IconData::Rgba { data, .. }) = &result {
                    assert!(
                        data.iter().any(|&b| b != 0),
                        "lucide loader rasterized to empty image"
                    );
                }
            } else {
                panic!("lucide spinner icon should be Svg");
            }
        } else {
            panic!("lucide spinner should be Transform");
        }
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::field_reassign_with_default
)]
mod system_theme_tests {
    use super::*;

    // --- SystemTheme::active() / pick() tests ---

    #[test]
    fn test_system_theme_active_dark() {
        let preset = ThemeSpec::preset("catppuccin-mocha").unwrap();
        let mut light_v = preset.light.clone().unwrap();
        let mut dark_v = preset.dark.clone().unwrap();
        // Give them distinct accents so we can tell them apart
        light_v.defaults.accent = Some(Rgba::rgb(0, 0, 255));
        dark_v.defaults.accent = Some(Rgba::rgb(255, 0, 0));
        light_v.resolve();
        dark_v.resolve();
        let light_resolved = light_v.validate().unwrap();
        let dark_resolved = dark_v.validate().unwrap();

        let st = SystemTheme {
            name: "test".into(),
            is_dark: true,
            light: light_resolved.clone(),
            dark: dark_resolved.clone(),
            light_variant: preset.light.unwrap(),
            dark_variant: preset.dark.unwrap(),
            live_preset: "catppuccin-mocha".into(),
            preset: "catppuccin-mocha".into(),
        };
        assert_eq!(st.active().defaults.accent, dark_resolved.defaults.accent);
    }

    #[test]
    fn test_system_theme_active_light() {
        let preset = ThemeSpec::preset("catppuccin-mocha").unwrap();
        let mut light_v = preset.light.clone().unwrap();
        let mut dark_v = preset.dark.clone().unwrap();
        light_v.defaults.accent = Some(Rgba::rgb(0, 0, 255));
        dark_v.defaults.accent = Some(Rgba::rgb(255, 0, 0));
        light_v.resolve();
        dark_v.resolve();
        let light_resolved = light_v.validate().unwrap();
        let dark_resolved = dark_v.validate().unwrap();

        let st = SystemTheme {
            name: "test".into(),
            is_dark: false,
            light: light_resolved.clone(),
            dark: dark_resolved.clone(),
            light_variant: preset.light.unwrap(),
            dark_variant: preset.dark.unwrap(),
            live_preset: "catppuccin-mocha".into(),
            preset: "catppuccin-mocha".into(),
        };
        assert_eq!(st.active().defaults.accent, light_resolved.defaults.accent);
    }

    #[test]
    fn test_system_theme_pick() {
        let preset = ThemeSpec::preset("catppuccin-mocha").unwrap();
        let mut light_v = preset.light.clone().unwrap();
        let mut dark_v = preset.dark.clone().unwrap();
        light_v.defaults.accent = Some(Rgba::rgb(0, 0, 255));
        dark_v.defaults.accent = Some(Rgba::rgb(255, 0, 0));
        light_v.resolve();
        dark_v.resolve();
        let light_resolved = light_v.validate().unwrap();
        let dark_resolved = dark_v.validate().unwrap();

        let st = SystemTheme {
            name: "test".into(),
            is_dark: false,
            light: light_resolved.clone(),
            dark: dark_resolved.clone(),
            light_variant: preset.light.unwrap(),
            dark_variant: preset.dark.unwrap(),
            live_preset: "catppuccin-mocha".into(),
            preset: "catppuccin-mocha".into(),
        };
        assert_eq!(st.pick(true).defaults.accent, dark_resolved.defaults.accent);
        assert_eq!(
            st.pick(false).defaults.accent,
            light_resolved.defaults.accent
        );
    }

    // --- platform_preset_name() tests ---

    #[test]
    #[cfg(target_os = "linux")]
    #[allow(unsafe_code)]
    fn test_platform_preset_name_kde() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "KDE") };
        let name = platform_preset_name();
        unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") };
        assert_eq!(name, "kde-breeze-live");
    }

    #[test]
    #[cfg(target_os = "linux")]
    #[allow(unsafe_code)]
    fn test_platform_preset_name_gnome() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "GNOME") };
        let name = platform_preset_name();
        unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") };
        assert_eq!(name, "adwaita-live");
    }

    // --- run_pipeline() tests ---

    #[test]
    fn test_run_pipeline_produces_both_variants() {
        let reader = ThemeSpec::preset("catppuccin-mocha").unwrap();
        let result = run_pipeline(reader, "catppuccin-mocha", false);
        assert!(result.is_ok(), "run_pipeline should succeed");
        let st = result.unwrap();
        // Both light and dark exist as ResolvedThemeVariant (non-Option)
        assert!(!st.name.is_empty(), "name should be populated");
        // If we get here, both variants validated successfully
    }

    #[test]
    fn test_run_pipeline_reader_values_win() {
        // Create a reader with a custom accent color
        let custom_accent = Rgba::rgb(42, 100, 200);
        let mut reader = ThemeSpec::default();
        reader.name = "CustomTheme".into();
        let mut variant = ThemeVariant::default();
        variant.defaults.accent = Some(custom_accent);
        reader.light = Some(variant);

        let result = run_pipeline(reader, "catppuccin-mocha", false);
        assert!(result.is_ok(), "run_pipeline should succeed");
        let st = result.unwrap();
        // The reader's accent should win over the preset's accent
        assert_eq!(
            st.light.defaults.accent, custom_accent,
            "reader accent should win over preset accent"
        );
        assert_eq!(st.name, "CustomTheme", "reader name should win");
    }

    #[test]
    fn test_run_pipeline_single_variant() {
        // Simulate a real OS reader that provides a complete dark variant
        // (like KDE's from_kde() would) but no light variant.
        // Use a live preset so the inactive light variant gets the full preset.
        let full = ThemeSpec::preset("kde-breeze").unwrap();
        let mut reader = ThemeSpec::default();
        let mut dark_v = full.dark.clone().unwrap();
        // Override accent to prove reader values win
        dark_v.defaults.accent = Some(Rgba::rgb(200, 50, 50));
        reader.dark = Some(dark_v);
        reader.light = None;

        let result = run_pipeline(reader, "kde-breeze-live", true);
        assert!(
            result.is_ok(),
            "run_pipeline should succeed with single variant"
        );
        let st = result.unwrap();
        // Dark should have the reader's overridden accent
        assert_eq!(
            st.dark.defaults.accent,
            Rgba::rgb(200, 50, 50),
            "dark variant should have reader accent"
        );
        // Light should still exist (from full preset, which has colors)
        // If we get here, both variants validated successfully
        assert_eq!(st.live_preset, "kde-breeze-live");
        assert_eq!(st.preset, "kde-breeze");
    }

    #[test]
    fn test_run_pipeline_inactive_variant_from_full_preset() {
        // When reader provides only dark, light must come from the full preset
        // (not the live preset, which has no colors and would fail validation).
        let full = ThemeSpec::preset("kde-breeze").unwrap();
        let mut reader = ThemeSpec::default();
        reader.dark = Some(full.dark.clone().unwrap());
        reader.light = None;

        let st = run_pipeline(reader, "kde-breeze-live", true).unwrap();

        // The light variant should have colors from the full "kde-breeze" preset
        let full_light = full.light.unwrap();
        assert_eq!(
            st.light.defaults.accent,
            full_light.defaults.accent.unwrap(),
            "inactive light variant should get accent from full preset"
        );
        assert_eq!(
            st.light.defaults.background,
            full_light.defaults.background.unwrap(),
            "inactive light variant should get background from full preset"
        );
    }

    // --- run_pipeline with preset-as-reader (GNOME double-merge test) ---

    #[test]
    fn test_run_pipeline_with_preset_as_reader() {
        // Simulates GNOME sync fallback: adwaita used as both reader and preset.
        // Double-merge is harmless: merge is idempotent for matching values.
        let reader = ThemeSpec::preset("adwaita").unwrap();
        let result = run_pipeline(reader, "adwaita", false);
        assert!(
            result.is_ok(),
            "double-merge with same preset should succeed"
        );
        let st = result.unwrap();
        assert_eq!(st.name, "Adwaita");
    }

    // --- reader_is_dark() tests ---

    #[test]
    fn test_reader_is_dark_only_dark() {
        let mut theme = ThemeSpec::default();
        theme.dark = Some(ThemeVariant::default());
        theme.light = None;
        assert!(
            reader_is_dark(&theme),
            "should be true when only dark is set"
        );
    }

    #[test]
    fn test_reader_is_dark_only_light() {
        let mut theme = ThemeSpec::default();
        theme.light = Some(ThemeVariant::default());
        theme.dark = None;
        assert!(
            !reader_is_dark(&theme),
            "should be false when only light is set"
        );
    }

    #[test]
    fn test_reader_is_dark_both() {
        let mut theme = ThemeSpec::default();
        theme.light = Some(ThemeVariant::default());
        theme.dark = Some(ThemeVariant::default());
        assert!(
            !reader_is_dark(&theme),
            "should be false when both are set (macOS case)"
        );
    }

    #[test]
    fn test_reader_is_dark_neither() {
        let theme = ThemeSpec::default();
        assert!(
            !reader_is_dark(&theme),
            "should be false when neither is set"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod reduced_motion_tests {
    use super::*;

    #[test]
    fn prefers_reduced_motion_smoke_test() {
        // Smoke test: function should not panic on any platform.
        // Cannot assert a specific value because OnceLock caches the first call
        // and CI environments have varying accessibility settings.
        let _result = prefers_reduced_motion();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn detect_reduced_motion_inner_linux() {
        // Bypass OnceLock to test actual detection logic.
        // On CI without gsettings, returns false (animations enabled).
        // On developer machines, depends on accessibility settings.
        let result = detect_reduced_motion_inner();
        // Just verify it returns a bool without panicking.
        let _ = result;
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn detect_reduced_motion_inner_macos() {
        let result = detect_reduced_motion_inner();
        let _ = result;
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn detect_reduced_motion_inner_windows() {
        let result = detect_reduced_motion_inner();
        let _ = result;
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod overlay_tests {
    use super::*;

    /// Helper: build a SystemTheme from a preset via run_pipeline.
    fn default_system_theme() -> SystemTheme {
        let reader = ThemeSpec::preset("catppuccin-mocha").unwrap();
        run_pipeline(reader, "catppuccin-mocha", false).unwrap()
    }

    #[test]
    fn test_overlay_accent_propagates() {
        let st = default_system_theme();
        let new_accent = Rgba::rgb(255, 0, 0);

        // Build overlay with accent on both light and dark
        let mut overlay = ThemeSpec::default();
        let mut light_v = ThemeVariant::default();
        light_v.defaults.accent = Some(new_accent);
        let mut dark_v = ThemeVariant::default();
        dark_v.defaults.accent = Some(new_accent);
        overlay.light = Some(light_v);
        overlay.dark = Some(dark_v);

        let result = st.with_overlay(&overlay).unwrap();

        // Accent itself
        assert_eq!(result.light.defaults.accent, new_accent);
        // Accent-derived widget fields
        assert_eq!(result.light.button.primary_bg, new_accent);
        assert_eq!(result.light.checkbox.checked_bg, new_accent);
        assert_eq!(result.light.slider.fill, new_accent);
        assert_eq!(result.light.progress_bar.fill, new_accent);
        assert_eq!(result.light.switch.checked_bg, new_accent);
    }

    #[test]
    fn test_overlay_preserves_unrelated_fields() {
        let st = default_system_theme();
        let original_bg = st.light.defaults.background;

        // Apply overlay changing only accent
        let mut overlay = ThemeSpec::default();
        let mut light_v = ThemeVariant::default();
        light_v.defaults.accent = Some(Rgba::rgb(255, 0, 0));
        overlay.light = Some(light_v);

        let result = st.with_overlay(&overlay).unwrap();
        assert_eq!(
            result.light.defaults.background, original_bg,
            "background should be unchanged"
        );
    }

    #[test]
    fn test_overlay_empty_noop() {
        let st = default_system_theme();
        let original_light_accent = st.light.defaults.accent;
        let original_dark_accent = st.dark.defaults.accent;
        let original_light_bg = st.light.defaults.background;

        // Empty overlay
        let overlay = ThemeSpec::default();
        let result = st.with_overlay(&overlay).unwrap();

        assert_eq!(result.light.defaults.accent, original_light_accent);
        assert_eq!(result.dark.defaults.accent, original_dark_accent);
        assert_eq!(result.light.defaults.background, original_light_bg);
    }

    #[test]
    fn test_overlay_both_variants() {
        let st = default_system_theme();
        let red = Rgba::rgb(255, 0, 0);
        let green = Rgba::rgb(0, 255, 0);

        let mut overlay = ThemeSpec::default();
        let mut light_v = ThemeVariant::default();
        light_v.defaults.accent = Some(red);
        let mut dark_v = ThemeVariant::default();
        dark_v.defaults.accent = Some(green);
        overlay.light = Some(light_v);
        overlay.dark = Some(dark_v);

        let result = st.with_overlay(&overlay).unwrap();
        assert_eq!(result.light.defaults.accent, red, "light accent = red");
        assert_eq!(result.dark.defaults.accent, green, "dark accent = green");
    }

    #[test]
    fn test_overlay_font_family() {
        let st = default_system_theme();

        let mut overlay = ThemeSpec::default();
        let mut light_v = ThemeVariant::default();
        light_v.defaults.font.family = Some("Comic Sans".into());
        overlay.light = Some(light_v);

        let result = st.with_overlay(&overlay).unwrap();
        assert_eq!(result.light.defaults.font.family, "Comic Sans");
    }

    #[test]
    fn test_overlay_toml_convenience() {
        let st = default_system_theme();
        let result = st
            .with_overlay_toml(
                r##"
            name = "overlay"
            [light.defaults]
            accent = "#ff0000"
        "##,
            )
            .unwrap();
        assert_eq!(result.light.defaults.accent, Rgba::rgb(255, 0, 0));
    }
}
