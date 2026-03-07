//! Bundled theme presets and TOML serialization API.
//!
//! Provides three built-in presets (default, kde-breeze, adwaita) embedded
//! at compile time, plus functions for loading themes from TOML strings
//! and files.

use crate::{Error, NativeTheme, Result};
use std::path::Path;

// Embed preset TOML files at compile time
const DEFAULT_TOML: &str = include_str!("presets/default.toml");
const KDE_BREEZE_TOML: &str = include_str!("presets/kde-breeze.toml");
const ADWAITA_TOML: &str = include_str!("presets/adwaita.toml");

/// All available preset names.
const PRESET_NAMES: &[&str] = &["default", "kde-breeze", "adwaita"];

/// Load a bundled theme preset by name.
///
/// Returns the preset as a fully populated [`NativeTheme`] with both
/// light and dark variants.
///
/// # Errors
///
/// Returns [`Error::Unavailable`] if the preset name is not recognized.
///
/// # Examples
///
/// ```
/// let theme = native_theme::preset("default").unwrap();
/// assert!(theme.light.is_some());
/// assert!(theme.dark.is_some());
/// ```
pub fn preset(name: &str) -> Result<NativeTheme> {
    let toml_str = match name {
        "default" => DEFAULT_TOML,
        "kde-breeze" => KDE_BREEZE_TOML,
        "adwaita" => ADWAITA_TOML,
        _ => return Err(Error::Unavailable(format!("unknown preset: {name}"))),
    };
    from_toml(toml_str)
}

/// List all available bundled preset names.
///
/// # Examples
///
/// ```
/// let names = native_theme::list_presets();
/// assert!(names.contains(&"default"));
/// assert!(names.contains(&"kde-breeze"));
/// assert!(names.contains(&"adwaita"));
/// ```
pub fn list_presets() -> &'static [&'static str] {
    PRESET_NAMES
}

/// Parse a TOML string into a [`NativeTheme`].
///
/// # Errors
///
/// Returns [`Error::Format`] if the TOML is invalid or doesn't
/// match the [`NativeTheme`] schema.
///
/// # Examples
///
/// ```
/// let toml = r##"
/// name = "My Theme"
/// [light.colors.core]
/// accent = "#ff0000"
/// "##;
/// let theme = native_theme::from_toml(toml).unwrap();
/// assert_eq!(theme.name, "My Theme");
/// ```
pub fn from_toml(toml_str: &str) -> Result<NativeTheme> {
    let theme: NativeTheme = toml::from_str(toml_str)?;
    Ok(theme)
}

/// Load a [`NativeTheme`] from a TOML file at the given path.
///
/// # Errors
///
/// Returns [`Error::Unavailable`] if the file cannot be read, or
/// [`Error::Format`] if the contents are not valid theme TOML.
///
/// # Examples
///
/// ```no_run
/// let theme = native_theme::from_file("my-theme.toml").unwrap();
/// ```
pub fn from_file(path: impl AsRef<Path>) -> Result<NativeTheme> {
    let contents = std::fs::read_to_string(path)?;
    from_toml(&contents)
}

/// Serialize a [`NativeTheme`] to a TOML string.
///
/// # Errors
///
/// Returns [`Error::Format`] if serialization fails.
///
/// # Examples
///
/// ```
/// let theme = native_theme::preset("default").unwrap();
/// let toml_str = native_theme::to_toml(&theme).unwrap();
/// assert!(toml_str.contains("name = \"Default\""));
/// ```
pub fn to_toml(theme: &NativeTheme) -> Result<String> {
    let s = toml::to_string_pretty(theme)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_presets_loadable_via_preset_fn() {
        for name in list_presets() {
            let theme = preset(name)
                .unwrap_or_else(|e| panic!("preset '{name}' failed to parse: {e}"));
            assert!(
                theme.light.is_some(),
                "preset '{name}' missing light variant"
            );
            assert!(
                theme.dark.is_some(),
                "preset '{name}' missing dark variant"
            );
        }
    }

    #[test]
    fn all_presets_have_nonempty_core_colors() {
        for name in list_presets() {
            let theme = preset(name).unwrap();
            let light = theme.light.as_ref().unwrap();
            let dark = theme.dark.as_ref().unwrap();

            assert!(
                light.colors.core.accent.is_some(),
                "preset '{name}' light missing accent"
            );
            assert!(
                light.colors.core.background.is_some(),
                "preset '{name}' light missing background"
            );
            assert!(
                light.colors.core.foreground.is_some(),
                "preset '{name}' light missing foreground"
            );
            assert!(
                dark.colors.core.accent.is_some(),
                "preset '{name}' dark missing accent"
            );
            assert!(
                dark.colors.core.background.is_some(),
                "preset '{name}' dark missing background"
            );
            assert!(
                dark.colors.core.foreground.is_some(),
                "preset '{name}' dark missing foreground"
            );
        }
    }

    #[test]
    fn preset_unknown_name_returns_unavailable() {
        let err = preset("nonexistent").unwrap_err();
        match err {
            Error::Unavailable(msg) => assert!(msg.contains("nonexistent")),
            other => panic!("expected Unavailable, got: {other:?}"),
        }
    }

    #[test]
    fn list_presets_returns_all_three() {
        let names = list_presets();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"default"));
        assert!(names.contains(&"kde-breeze"));
        assert!(names.contains(&"adwaita"));
    }

    #[test]
    fn from_toml_minimal_valid() {
        let toml_str = r##"
name = "Minimal"

[light.colors.core]
accent = "#ff0000"
"##;
        let theme = from_toml(toml_str).unwrap();
        assert_eq!(theme.name, "Minimal");
        assert!(theme.light.is_some());
        let light = theme.light.unwrap();
        assert_eq!(
            light.colors.core.accent,
            Some(crate::Rgba::rgb(255, 0, 0))
        );
    }

    #[test]
    fn from_toml_invalid_returns_format_error() {
        let err = from_toml("{{{{invalid toml").unwrap_err();
        match err {
            Error::Format(_) => {}
            other => panic!("expected Format, got: {other:?}"),
        }
    }

    #[test]
    fn to_toml_produces_valid_round_trip() {
        let theme = preset("default").unwrap();
        let toml_str = to_toml(&theme).unwrap();

        // Must be parseable back into a NativeTheme
        let reparsed = from_toml(&toml_str).unwrap();
        assert_eq!(reparsed.name, theme.name);
        assert!(reparsed.light.is_some());
        assert!(reparsed.dark.is_some());

        // Core colors should survive the round-trip
        let orig_light = theme.light.as_ref().unwrap();
        let new_light = reparsed.light.as_ref().unwrap();
        assert_eq!(
            orig_light.colors.core.accent,
            new_light.colors.core.accent
        );
    }

    #[test]
    fn from_file_with_tempfile() {
        let dir = std::env::temp_dir();
        let path = dir.join("native_theme_test_preset.toml");
        let toml_str = r##"
name = "File Test"

[light.colors.core]
accent = "#00ff00"
"##;
        std::fs::write(&path, toml_str).unwrap();

        let theme = from_file(&path).unwrap();
        assert_eq!(theme.name, "File Test");
        assert!(theme.light.is_some());

        // Clean up
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn from_file_nonexistent_returns_error() {
        let err = from_file("/tmp/nonexistent_theme_file_12345.toml").unwrap_err();
        match err {
            Error::Unavailable(_) => {}
            other => panic!("expected Unavailable, got: {other:?}"),
        }
    }

    #[test]
    fn preset_names_match_list() {
        // Every name in list_presets() must be loadable via preset()
        for name in list_presets() {
            assert!(preset(name).is_ok(), "preset '{name}' not loadable");
        }
    }

    #[test]
    fn presets_have_correct_names() {
        assert_eq!(preset("default").unwrap().name, "Default");
        assert_eq!(preset("kde-breeze").unwrap().name, "KDE Breeze");
        assert_eq!(preset("adwaita").unwrap().name, "Adwaita");
    }

    #[test]
    fn all_presets_have_valid_font_sizes() {
        for name in list_presets() {
            let theme = preset(name).unwrap();
            for (label, variant) in [
                ("light", theme.light.as_ref()),
                ("dark", theme.dark.as_ref()),
            ] {
                let variant = variant.unwrap();
                let size = variant.fonts.size.unwrap();
                assert!(
                    size > 0.0,
                    "preset '{name}' {label} font size must be positive, got {size}"
                );
                let mono_size = variant.fonts.mono_size.unwrap();
                assert!(
                    mono_size > 0.0,
                    "preset '{name}' {label} mono font size must be positive, got {mono_size}"
                );
            }
        }
    }
}
