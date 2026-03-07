//! # native-theme
//!
//! Cross-platform native theme detection and loading for Rust GUI applications.
//!
//! Any Rust GUI app can look native on any platform by loading a single theme
//! file or reading live OS settings, without coupling to any specific toolkit.

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
    ActionColors, ComponentColors, CoreColors, InteractiveColors, NativeTheme, PanelColors,
    StatusColors, ThemeColors, ThemeFonts, ThemeGeometry, ThemeSpacing, ThemeVariant,
};
pub use presets::{from_file, from_toml, list_presets, preset, to_toml};

#[cfg(feature = "windows")]
pub mod windows;

#[cfg(feature = "portal")]
pub use gnome::from_gnome;
#[cfg(feature = "kde")]
pub use kde::from_kde;
#[cfg(feature = "windows")]
pub use windows::from_windows;

/// Convenience Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

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
        // Temporarily set XDG_CURRENT_DESKTOP to GNOME so from_linux()
        // takes the preset fallback path.
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "GNOME") };
        let result = from_linux();
        unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") };

        let theme = result.expect("from_linux() should return Ok for non-KDE desktop");
        assert_eq!(theme.name, "Adwaita");
    }

    // -- from_system() smoke test --

    #[test]
    fn from_system_returns_result() {
        // On Linux (our test platform), from_system() should return a Result.
        // With GNOME set, it should return the Adwaita preset.
        unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", "GNOME") };
        let result = from_system();
        unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP") };

        let theme = result.expect("from_system() should return Ok on Linux");
        assert_eq!(theme.name, "Adwaita");
    }
}
