//! # native-theme
//!
//! Cross-platform native theme detection and loading for Rust GUI applications.
//!
//! Any Rust GUI app can look native on any platform by loading a single theme
//! file or reading live OS settings, without coupling to any specific toolkit.

#![warn(missing_docs)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// Generates `merge()` and `is_empty()` methods for theme structs.
///
/// Two field categories:
/// - `option { field1, field2, ... }` -- `Option<T>` leaf fields
/// - `nested { field1, field2, ... }` -- nested struct fields with their own `merge()`
///
/// For `option` fields, `Some` values in the overlay replace the corresponding
/// fields in self; `None` fields are left unchanged.
/// For `nested` fields, merge is called recursively.
///
/// # Examples
///
/// ```
/// use native_theme::impl_merge;
///
/// #[derive(Clone, Debug, Default)]
/// struct MyColors {
///     accent: Option<String>,
///     background: Option<String>,
/// }
///
/// impl_merge!(MyColors {
///     option { accent, background }
/// });
///
/// let mut base = MyColors { accent: Some("blue".into()), background: None };
/// let overlay = MyColors { accent: None, background: Some("white".into()) };
/// base.merge(&overlay);
/// assert_eq!(base.accent.as_deref(), Some("blue"));
/// assert_eq!(base.background.as_deref(), Some("white"));
/// ```
#[macro_export]
macro_rules! impl_merge {
    (
        $struct_name:ident {
            $(option { $($opt_field:ident),* $(,)? })?
            $(nested { $($nest_field:ident),* $(,)? })?
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
            }

            /// Returns true if all fields are at their default (None/empty) state.
            pub fn is_empty(&self) -> bool {
                true
                $($(&& self.$opt_field.is_none())*)?
                $($(&& self.$nest_field.is_empty())*)?
            }
        }
    };
}

pub mod color;
pub mod error;
#[cfg(all(target_os = "linux", feature = "portal"))]
pub mod gnome;
#[cfg(all(target_os = "linux", feature = "kde"))]
pub mod kde;
pub mod model;
pub mod presets;
#[cfg(any(
    feature = "material-icons",
    feature = "lucide-icons",
    feature = "system-icons"
))]
mod spinners;

pub use color::Rgba;
pub use error::Error;
pub use model::{
    AnimatedIcon, IconData, IconProvider, IconRole, IconSet, NativeTheme, Repeat, ThemeColors,
    ThemeFonts, ThemeGeometry, ThemeSpacing, ThemeVariant, TransformAnimation, WidgetMetrics,
    bundled_icon_by_name, bundled_icon_svg,
};
// load_icon re-exported from this module (defined in lib.rs directly)
pub use model::icons::{icon_name, system_icon_set, system_icon_theme};

#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub mod freedesktop;
pub mod macos;
#[cfg(feature = "svg-rasterize")]
pub mod rasterize;
#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub mod sficons;
#[cfg(all(target_os = "windows", feature = "windows"))]
pub mod windows;
#[cfg(feature = "system-icons")]
#[cfg_attr(not(target_os = "windows"), allow(dead_code, unused_imports))]
pub mod winicons;

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
    Kde,
    Gnome,
    Xfce,
    Cinnamon,
    Mate,
    LxQt,
    Budgie,
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
/// The result is cached after the first call using `OnceLock`.
///
/// # Fallback chain
///
/// 1. `gsettings get org.gnome.desktop.interface color-scheme` — works on
///    all DEs that implement the freedesktop color-scheme setting (GNOME,
///    KDE 5.x+, XFCE, etc.).
/// 2. **(with `kde` feature)** `~/.config/kdeglobals` background luminance.
/// 3. Returns `false` (light) if neither source is available.
#[cfg(target_os = "linux")]
#[must_use = "this returns whether the system uses dark mode"]
pub fn system_is_dark() -> bool {
    static CACHED_IS_DARK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHED_IS_DARK.get_or_init(detect_is_dark_inner)
}

/// Inner detection logic for [`system_is_dark()`].
///
/// Separated from the public function to allow caching via `OnceLock`.
#[cfg(target_os = "linux")]
fn detect_is_dark_inner() -> bool {
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

/// Query whether the user prefers reduced motion.
///
/// Returns `true` when the OS accessibility setting indicates animations
/// should be reduced or disabled. Returns `false` (allow animations) on
/// unsupported platforms or when the query fails.
///
/// The result is cached after the first call using `OnceLock`.
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

/// Read the current system theme on Linux by detecting the desktop
/// environment and calling the appropriate reader or returning a
/// preset fallback.
#[cfg(target_os = "linux")]
fn from_linux() -> crate::Result<NativeTheme> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    match detect_linux_de(&desktop) {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => crate::kde::from_kde(),
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => NativeTheme::preset("adwaita"),
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => NativeTheme::preset("adwaita"),
        LinuxDesktop::Xfce | LinuxDesktop::Cinnamon | LinuxDesktop::Mate | LinuxDesktop::LxQt => {
            NativeTheme::preset("adwaita")
        }
        LinuxDesktop::Unknown => {
            #[cfg(feature = "kde")]
            {
                let path = crate::kde::kdeglobals_path();
                if path.exists() {
                    return crate::kde::from_kde();
                }
            }
            NativeTheme::preset("adwaita")
        }
    }
}

/// Read the current system theme, auto-detecting the platform and
/// desktop environment.
///
/// # Platform Behavior
///
/// - **macOS:** Calls `from_macos()` when the `macos` feature is enabled.
///   Reads both light and dark variants via NSAppearance.
/// - **Linux (KDE):** Calls `from_kde()` when `XDG_CURRENT_DESKTOP` contains
///   "KDE" and the `kde` feature is enabled.
/// - **Linux (other):** Returns the bundled Adwaita preset. For live GNOME
///   portal data, call `from_gnome()` directly (requires `portal-tokio` or
///   `portal-async-io` feature).
/// - **Windows:** Calls `from_windows()` when the `windows` feature is enabled.
/// - **Other platforms:** Returns `Error::Unsupported`.
///
/// # Errors
///
/// - `Error::Unsupported` if the platform has no reader or the required feature
///   is not enabled.
/// - `Error::Unavailable` if the platform reader cannot access theme data.
#[must_use = "this returns the detected theme; it does not apply it"]
pub fn from_system() -> crate::Result<NativeTheme> {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        return crate::macos::from_macos();

        #[cfg(not(feature = "macos"))]
        return Err(crate::Error::Unsupported);
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        return crate::windows::from_windows();

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

/// Async version of [`from_system()`] that uses D-Bus portal backend
/// detection to improve desktop environment heuristics on Linux.
///
/// When `XDG_CURRENT_DESKTOP` is unset or unrecognized, queries the
/// D-Bus session bus for portal backend activatable names to determine
/// whether KDE or GNOME portal is running, then dispatches to the
/// appropriate reader.
///
/// On non-Linux platforms, behaves identically to [`from_system()`].
#[cfg(target_os = "linux")]
#[must_use = "this returns the detected theme; it does not apply it"]
pub async fn from_system_async() -> crate::Result<NativeTheme> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    match detect_linux_de(&desktop) {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => {
            #[cfg(feature = "portal")]
            return crate::gnome::from_kde_with_portal().await;
            #[cfg(not(feature = "portal"))]
            return crate::kde::from_kde();
        }
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => NativeTheme::preset("adwaita"),
        #[cfg(feature = "portal")]
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => crate::gnome::from_gnome().await,
        #[cfg(not(feature = "portal"))]
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => NativeTheme::preset("adwaita"),
        LinuxDesktop::Xfce | LinuxDesktop::Cinnamon | LinuxDesktop::Mate | LinuxDesktop::LxQt => {
            NativeTheme::preset("adwaita")
        }
        LinuxDesktop::Unknown => {
            // Use D-Bus portal backend detection to refine heuristic
            #[cfg(feature = "portal")]
            {
                if let Some(detected) = crate::gnome::detect_portal_backend().await {
                    return match detected {
                        #[cfg(feature = "kde")]
                        LinuxDesktop::Kde => crate::gnome::from_kde_with_portal().await,
                        #[cfg(not(feature = "kde"))]
                        LinuxDesktop::Kde => NativeTheme::preset("adwaita"),
                        LinuxDesktop::Gnome => crate::gnome::from_gnome().await,
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
                    return crate::kde::from_kde();
                }
            }
            NativeTheme::preset("adwaita")
        }
    }
}

/// Async version of [`from_system()`].
///
/// On non-Linux platforms, this is equivalent to calling [`from_system()`].
#[cfg(not(target_os = "linux"))]
#[must_use = "this returns the detected theme; it does not apply it"]
pub async fn from_system_async() -> crate::Result<NativeTheme> {
    from_system()
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
/// 1. Parse `icon_set` to `IconSet` (unknown names fall back to system set)
/// 2. Platform loader (freedesktop/sf-symbols/segoe-fluent) when `system-icons` enabled
/// 3. Bundled SVGs (material/lucide) when the corresponding feature is enabled
/// 4. Non-matching set: `None` (no cross-set fallback)
///
/// # Examples
///
/// ```
/// use native_theme::{load_icon, IconRole};
///
/// // With material-icons feature enabled
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = load_icon(IconRole::ActionCopy, "material");
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use = "this returns the loaded icon data; it does not display it"]
#[allow(unreachable_patterns, clippy::needless_return, unused_variables)]
pub fn load_icon(role: IconRole, icon_set: &str) -> Option<IconData> {
    let set = IconSet::from_name(icon_set).unwrap_or_else(system_icon_set);

    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => freedesktop::load_freedesktop_icon(role),

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => sficons::load_sf_icon(role),

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => winicons::load_windows_icon(role),

        #[cfg(feature = "material-icons")]
        IconSet::Material => {
            bundled_icon_svg(IconSet::Material, role).map(|b| IconData::Svg(b.to_vec()))
        }

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => {
            bundled_icon_svg(IconSet::Lucide, role).map(|b| IconData::Svg(b.to_vec()))
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
            freedesktop::load_freedesktop_icon_by_name(name, &theme)
        }

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => sficons::load_sf_icon_by_name(name),

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => winicons::load_windows_icon_by_name(name),

        #[cfg(feature = "material-icons")]
        IconSet::Material => {
            bundled_icon_by_name(IconSet::Material, name).map(|b| IconData::Svg(b.to_vec()))
        }

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => {
            bundled_icon_by_name(IconSet::Lucide, name).map(|b| IconData::Svg(b.to_vec()))
        }

        _ => None,
    }
}

/// Return the loading/spinner animation for the given icon set.
///
/// This is the animated-icon counterpart of [`load_icon()`]. It resolves
/// `icon_set` to an [`IconSet`] via [`IconSet::from_name()`], falling back
/// to [`system_icon_set()`] for unrecognized names, then dispatches to the
/// appropriate bundled spinner data.
///
/// # Dispatch
///
/// - `"material"` -- 12-frame circular arc spinner (83ms per frame)
/// - `"lucide"` -- single loader icon with continuous spin transform (1000ms)
/// - `"freedesktop"` -- bundled Adwaita-style spinner (20 frames, 60ms)
/// - `"sf-symbols"` (macOS) -- macOS-style radial spoke spinner (12 frames)
/// - `"segoe-fluent"` (Windows) -- Windows-style arc spinner (60 frames)
/// - Unknown set -- `None`
///
/// # Examples
///
/// ```
/// // Result depends on enabled features and platform
/// let anim = native_theme::loading_indicator("material");
/// # #[cfg(feature = "material-icons")]
/// # assert!(anim.is_some());
/// ```
#[must_use = "this returns animation data; it does not display anything"]
pub fn loading_indicator(icon_set: &str) -> Option<AnimatedIcon> {
    let set = IconSet::from_name(icon_set).unwrap_or_else(system_icon_set);
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => {
            freedesktop::load_freedesktop_spinner().or_else(|| Some(spinners::adwaita_spinner()))
        }

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => Some(spinners::macos_spinner()),

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => Some(spinners::windows_spinner()),

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
/// The `icon_set` string is parsed via [`IconSet::from_name()`], falling back
/// to [`system_icon_set()`] for unrecognized names.
///
/// # Examples
///
/// ```
/// use native_theme::{load_custom_icon, IconRole};
///
/// // IconRole implements IconProvider, so it works with load_custom_icon
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = load_custom_icon(&IconRole::ActionCopy, "material");
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use = "this returns the loaded icon data; it does not display it"]
pub fn load_custom_icon(
    provider: &(impl IconProvider + ?Sized),
    icon_set: &str,
) -> Option<IconData> {
    let set = IconSet::from_name(icon_set).unwrap_or_else(system_icon_set);

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
    fn from_system_returns_result() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        // On Linux (our test platform), from_system() should return a Result.
        // With GNOME set, it should return the Adwaita preset.
        // SAFETY: ENV_MUTEX serializes env var access across parallel tests
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "GNOME") };
        let result = from_system();
        unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") };

        let theme = result.expect("from_system() should return Ok on Linux");
        assert_eq!(theme.name, "Adwaita");
    }
}

#[cfg(test)]
mod load_icon_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_material_returns_svg() {
        let result = load_icon(IconRole::ActionCopy, "material");
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
        let result = load_icon(IconRole::ActionCopy, "lucide");
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
        let result = load_icon(IconRole::ActionCopy, "unknown-theme");
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
            if load_icon(role, "material").is_some() {
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
            if load_icon(role, "lucide").is_some() {
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
        let _result = load_icon(IconRole::ActionCopy, "sf-symbols");
        // Just verifying it doesn't panic
    }
}

#[cfg(test)]
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
mod load_custom_icon_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_with_icon_role_material() {
        let result = load_custom_icon(&IconRole::ActionCopy, "material");
        assert!(
            result.is_some(),
            "IconRole::ActionCopy should load via material"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn custom_icon_with_icon_role_lucide() {
        let result = load_custom_icon(&IconRole::ActionCopy, "lucide");
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

        let result = load_custom_icon(&NullProvider, "material");
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
        let _result = load_custom_icon(&NullProvider, "unknown-set");
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_via_dyn_dispatch() {
        let boxed: Box<dyn IconProvider> = Box::new(IconRole::ActionCopy);
        let result = load_custom_icon(&*boxed, "material");
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

        let result = load_custom_icon(&SvgOnlyProvider, "material");
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
mod loading_indicator_tests {
    use super::*;

    // === Dispatch tests (through loading_indicator public API) ===

    #[test]
    #[cfg(feature = "material-icons")]
    fn loading_indicator_material_returns_frames() {
        let anim = loading_indicator("material");
        assert!(anim.is_some(), "material should return Some");
        let anim = anim.unwrap();
        match &anim {
            AnimatedIcon::Frames {
                frames,
                frame_duration_ms,
                repeat,
            } => {
                assert_eq!(frames.len(), 12, "material has 12 frames");
                assert_eq!(*frame_duration_ms, 83);
                assert_eq!(*repeat, Repeat::Infinite);
            }
            _ => panic!("material should be AnimatedIcon::Frames"),
        }
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn loading_indicator_lucide_returns_transform_spin() {
        let anim = loading_indicator("lucide");
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

    #[test]
    #[cfg(all(target_os = "macos", feature = "system-icons"))]
    fn loading_indicator_macos_returns_frames() {
        let anim = loading_indicator("macos");
        assert!(anim.is_some(), "macos should return Some on macOS");
        if let AnimatedIcon::Frames {
            frames,
            frame_duration_ms,
            ..
        } = anim.unwrap()
        {
            assert_eq!(frames.len(), 12);
            assert_eq!(frame_duration_ms, 83);
        } else {
            panic!("macos should be AnimatedIcon::Frames");
        }
    }

    #[test]
    #[cfg(all(target_os = "windows", feature = "system-icons"))]
    fn loading_indicator_windows_returns_frames() {
        let anim = loading_indicator("windows");
        assert!(anim.is_some(), "windows should return Some on Windows");
        if let AnimatedIcon::Frames {
            frames,
            frame_duration_ms,
            ..
        } = anim.unwrap()
        {
            assert_eq!(frames.len(), 60);
            assert_eq!(frame_duration_ms, 33);
        } else {
            panic!("windows should be AnimatedIcon::Frames");
        }
    }

    /// Freedesktop loading_indicator returns Some -- either theme-native
    /// sprite sheet frames (e.g. Breeze 15 frames) or bundled Adwaita (20 frames).
    /// The result is theme-dependent so we only assert Some + AnimatedIcon variant.
    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn loading_indicator_freedesktop_returns_some() {
        let anim = loading_indicator("freedesktop");
        assert!(
            anim.is_some(),
            "freedesktop should return Some (theme-native or adwaita fallback) on Linux"
        );
        // Accept either Frames (sprite sheet or bundled Adwaita) or Transform (single-frame spin)
        match anim.unwrap() {
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

    /// The or_else fallback guarantees loading_indicator("freedesktop") never returns None.
    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn loading_indicator_freedesktop_always_returns_some() {
        let result = loading_indicator("freedesktop");
        assert!(
            result.is_some(),
            "freedesktop should always return Some due to or_else adwaita fallback"
        );
    }

    #[test]
    #[cfg(all(target_os = "macos", feature = "system-icons"))]
    fn loading_indicator_sf_symbols_returns_frames() {
        let anim = loading_indicator("sf-symbols");
        assert!(anim.is_some(), "sf-symbols should return Some on macOS");
        if let AnimatedIcon::Frames {
            frames,
            frame_duration_ms,
            ..
        } = anim.unwrap()
        {
            assert_eq!(frames.len(), 12);
            assert_eq!(frame_duration_ms, 83);
        } else {
            panic!("sf-symbols should be AnimatedIcon::Frames");
        }
    }

    #[test]
    #[cfg(all(target_os = "windows", feature = "system-icons"))]
    fn loading_indicator_segoe_returns_frames() {
        let anim = loading_indicator("segoe-fluent");
        assert!(anim.is_some(), "segoe-fluent should return Some on Windows");
        if let AnimatedIcon::Frames {
            frames,
            frame_duration_ms,
            ..
        } = anim.unwrap()
        {
            assert_eq!(frames.len(), 60);
            assert_eq!(frame_duration_ms, 33);
        } else {
            panic!("segoe-fluent should be AnimatedIcon::Frames");
        }
    }

    /// Unknown icon set name falls back to system_icon_set().
    /// On Linux with system-icons, that's Freedesktop (returns adwaita spinner).
    /// Without system-icons, it falls through to None.
    #[test]
    fn loading_indicator_unknown_falls_back_to_system() {
        let result = loading_indicator("unknown");
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        assert!(
            result.is_some(),
            "on Linux+system-icons, unknown -> Freedesktop -> Some"
        );
        #[cfg(not(all(target_os = "linux", feature = "system-icons")))]
        {
            // On non-Linux or without system-icons, depends on platform.
            // Without any matching feature, falls to wildcard -> None.
            let _ = result;
        }
    }

    #[test]
    fn loading_indicator_empty_string_falls_back_to_system() {
        let result = loading_indicator("");
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        assert!(
            result.is_some(),
            "on Linux+system-icons, empty -> Freedesktop -> Some"
        );
        #[cfg(not(all(target_os = "linux", feature = "system-icons")))]
        {
            let _ = result;
        }
    }

    // === Direct spinner construction tests (any platform) ===

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_spinner_frame_count() {
        let anim = spinners::material_spinner();
        if let AnimatedIcon::Frames { frames, .. } = &anim {
            assert_eq!(frames.len(), 12);
            for frame in frames {
                assert!(
                    matches!(frame, IconData::Svg(_)),
                    "each frame should be Svg"
                );
            }
        } else {
            panic!("material_spinner should be Frames");
        }
    }

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

    #[test]
    #[cfg(feature = "system-icons")]
    fn macos_spinner_frame_count() {
        let anim = spinners::macos_spinner();
        if let AnimatedIcon::Frames { frames, .. } = &anim {
            assert_eq!(frames.len(), 12);
        } else {
            panic!("macos_spinner should be Frames");
        }
    }

    #[test]
    #[cfg(feature = "system-icons")]
    fn windows_spinner_frame_count() {
        let anim = spinners::windows_spinner();
        if let AnimatedIcon::Frames { frames, .. } = &anim {
            assert_eq!(frames.len(), 60);
        } else {
            panic!("windows_spinner should be Frames");
        }
    }

    #[test]
    #[cfg(feature = "system-icons")]
    fn adwaita_spinner_frame_count() {
        let anim = spinners::adwaita_spinner();
        if let AnimatedIcon::Frames {
            frames,
            frame_duration_ms,
            ..
        } = &anim
        {
            assert_eq!(frames.len(), 20);
            assert_eq!(*frame_duration_ms, 60);
        } else {
            panic!("adwaita_spinner should be Frames");
        }
    }
}

#[cfg(all(test, feature = "svg-rasterize"))]
mod spinner_rasterize_tests {
    use super::*;

    fn assert_frames_rasterize(spinner: &AnimatedIcon, set_name: &str) {
        if let AnimatedIcon::Frames { frames, .. } = spinner {
            for (i, frame) in frames.iter().enumerate() {
                if let IconData::Svg(bytes) = frame {
                    let result = crate::rasterize::rasterize_svg(bytes, 24);
                    assert!(result.is_some(), "{set_name} frame {i} failed to rasterize");
                    if let Some(IconData::Rgba { data, .. }) = &result {
                        assert!(
                            data.iter().any(|&b| b != 0),
                            "{set_name} frame {i} rasterized to empty image"
                        );
                    }
                } else {
                    panic!("{set_name} frame {i} should be IconData::Svg");
                }
            }
        } else {
            panic!("{set_name} should be AnimatedIcon::Frames");
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_spinner_frames_rasterize() {
        let anim = spinners::material_spinner();
        assert_frames_rasterize(&anim, "material");
    }

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

    #[test]
    #[cfg(feature = "system-icons")]
    fn macos_spinner_frames_rasterize() {
        let anim = spinners::macos_spinner();
        assert_frames_rasterize(&anim, "macos");
    }

    #[test]
    #[cfg(feature = "system-icons")]
    fn windows_spinner_frames_rasterize() {
        let anim = spinners::windows_spinner();
        assert_frames_rasterize(&anim, "windows");
    }

    #[test]
    #[cfg(feature = "system-icons")]
    fn adwaita_spinner_frames_rasterize() {
        let anim = spinners::adwaita_spinner();
        assert_frames_rasterize(&anim, "adwaita");
    }
}
