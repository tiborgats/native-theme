//! # native-theme
//!
//! Cross-platform native theme detection and loading for Rust GUI applications.
//!
//! Any Rust GUI app can look native on any platform by loading a single theme
//! file or reading live OS settings, without coupling to any specific toolkit.

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
#[cfg(feature = "portal")]
pub mod gnome;
#[cfg(feature = "kde")]
pub mod kde;
pub mod model;
pub mod presets;

pub use color::Rgba;
pub use error::Error;
pub use model::{
    IconData, IconRole, IconSet, NativeTheme, ThemeColors, ThemeFonts, ThemeGeometry, ThemeSpacing,
    ThemeVariant, WidgetMetrics,
};
pub use model::icons::{icon_name, system_icon_set};

pub mod macos;
#[cfg(feature = "windows")]
pub mod windows;

#[cfg(feature = "portal")]
pub use gnome::from_gnome;
#[cfg(all(feature = "portal", feature = "kde"))]
pub use gnome::from_kde_with_portal;
#[cfg(feature = "kde")]
pub use kde::from_kde;
#[cfg(feature = "macos")]
pub use macos::from_macos;
#[cfg(feature = "windows")]
pub use windows::from_windows;

/// Convenience Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Desktop environments recognized on Linux.
#[cfg(target_os = "linux")]
#[derive(Debug, PartialEq)]
enum LinuxDesktop {
    Kde,
    Gnome,
    Unknown,
}

/// Parse `XDG_CURRENT_DESKTOP` (a colon-separated list) and return
/// the recognized desktop environment.
#[cfg(target_os = "linux")]
fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop {
    for component in xdg_current_desktop.split(':') {
        match component {
            "KDE" => return LinuxDesktop::Kde,
            "GNOME" => return LinuxDesktop::Gnome,
            _ => {}
        }
    }
    LinuxDesktop::Unknown
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
        LinuxDesktop::Gnome => NativeTheme::preset("adwaita"),
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
        LinuxDesktop::Gnome => crate::gnome::from_gnome().await,
        #[cfg(not(feature = "portal"))]
        LinuxDesktop::Gnome => NativeTheme::preset("adwaita"),
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
                        LinuxDesktop::Unknown => {
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
pub async fn from_system_async() -> crate::Result<NativeTheme> {
    from_system()
}

/// Mutex to serialize tests that manipulate environment variables.
/// Env vars are process-global state, so tests that call set_var/remove_var
/// must hold this lock to avoid races with parallel test execution.
#[cfg(test)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
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
    fn detect_unknown_xfce() {
        assert_eq!(detect_linux_de("XFCE"), LinuxDesktop::Unknown);
    }

    #[test]
    fn detect_unknown_cinnamon() {
        assert_eq!(detect_linux_de("Cinnamon"), LinuxDesktop::Unknown);
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
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "XFCE") };

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
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "XFCE") };

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
