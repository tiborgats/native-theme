//! Bundled theme presets and TOML serialization API.
//!
//! Provides 17 built-in presets embedded at compile time: 3 core (default,
//! kde-breeze, adwaita), 4 platform (windows-11, macos-sonoma, material, ios),
//! and 10 community (Catppuccin 4 flavors, Nord, Dracula, Gruvbox, Solarized,
//! Tokyo Night, One Dark), plus functions for loading themes from TOML strings
//! and files.

use crate::{Error, NativeTheme, Result};
use std::path::Path;

// Embed preset TOML files at compile time
const DEFAULT_TOML: &str = include_str!("presets/default.toml");
const KDE_BREEZE_TOML: &str = include_str!("presets/kde-breeze.toml");
const ADWAITA_TOML: &str = include_str!("presets/adwaita.toml");
const WINDOWS_11_TOML: &str = include_str!("presets/windows-11.toml");
const MACOS_SONOMA_TOML: &str = include_str!("presets/macos-sonoma.toml");
const MATERIAL_TOML: &str = include_str!("presets/material.toml");
const IOS_TOML: &str = include_str!("presets/ios.toml");
const CATPPUCCIN_LATTE_TOML: &str = include_str!("presets/catppuccin-latte.toml");
const CATPPUCCIN_FRAPPE_TOML: &str = include_str!("presets/catppuccin-frappe.toml");
const CATPPUCCIN_MACCHIATO_TOML: &str = include_str!("presets/catppuccin-macchiato.toml");
const CATPPUCCIN_MOCHA_TOML: &str = include_str!("presets/catppuccin-mocha.toml");
const NORD_TOML: &str = include_str!("presets/nord.toml");
const DRACULA_TOML: &str = include_str!("presets/dracula.toml");
const GRUVBOX_TOML: &str = include_str!("presets/gruvbox.toml");
const SOLARIZED_TOML: &str = include_str!("presets/solarized.toml");
const TOKYO_NIGHT_TOML: &str = include_str!("presets/tokyo-night.toml");
const ONE_DARK_TOML: &str = include_str!("presets/one-dark.toml");

/// All available preset names.
const PRESET_NAMES: &[&str] = &[
    "default",
    "kde-breeze",
    "adwaita",
    "windows-11",
    "macos-sonoma",
    "material",
    "ios",
    "catppuccin-latte",
    "catppuccin-frappe",
    "catppuccin-macchiato",
    "catppuccin-mocha",
    "nord",
    "dracula",
    "gruvbox",
    "solarized",
    "tokyo-night",
    "one-dark",
];

pub(crate) fn preset(name: &str) -> Result<NativeTheme> {
    let toml_str = match name {
        "default" => DEFAULT_TOML,
        "kde-breeze" => KDE_BREEZE_TOML,
        "adwaita" => ADWAITA_TOML,
        "windows-11" => WINDOWS_11_TOML,
        "macos-sonoma" => MACOS_SONOMA_TOML,
        "material" => MATERIAL_TOML,
        "ios" => IOS_TOML,
        "catppuccin-latte" => CATPPUCCIN_LATTE_TOML,
        "catppuccin-frappe" => CATPPUCCIN_FRAPPE_TOML,
        "catppuccin-macchiato" => CATPPUCCIN_MACCHIATO_TOML,
        "catppuccin-mocha" => CATPPUCCIN_MOCHA_TOML,
        "nord" => NORD_TOML,
        "dracula" => DRACULA_TOML,
        "gruvbox" => GRUVBOX_TOML,
        "solarized" => SOLARIZED_TOML,
        "tokyo-night" => TOKYO_NIGHT_TOML,
        "one-dark" => ONE_DARK_TOML,
        _ => return Err(Error::Unavailable(format!("unknown preset: {name}"))),
    };
    from_toml(toml_str)
}

pub(crate) fn list_presets() -> &'static [&'static str] {
    PRESET_NAMES
}

pub(crate) fn from_toml(toml_str: &str) -> Result<NativeTheme> {
    let theme: NativeTheme = toml::from_str(toml_str)?;
    Ok(theme)
}

pub(crate) fn from_file(path: impl AsRef<Path>) -> Result<NativeTheme> {
    let contents = std::fs::read_to_string(path)?;
    from_toml(&contents)
}

pub(crate) fn to_toml(theme: &NativeTheme) -> Result<String> {
    let s = toml::to_string_pretty(theme)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_presets_loadable_via_preset_fn() {
        for name in list_presets() {
            let theme =
                preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed to parse: {e}"));
            assert!(
                theme.light.is_some(),
                "preset '{name}' missing light variant"
            );
            assert!(theme.dark.is_some(), "preset '{name}' missing dark variant");
        }
    }

    #[test]
    fn all_presets_have_nonempty_core_colors() {
        for name in list_presets() {
            let theme = preset(name).unwrap();
            let light = theme.light.as_ref().unwrap();
            let dark = theme.dark.as_ref().unwrap();

            assert!(
                light.colors.accent.is_some(),
                "preset '{name}' light missing accent"
            );
            assert!(
                light.colors.background.is_some(),
                "preset '{name}' light missing background"
            );
            assert!(
                light.colors.foreground.is_some(),
                "preset '{name}' light missing foreground"
            );
            assert!(
                dark.colors.accent.is_some(),
                "preset '{name}' dark missing accent"
            );
            assert!(
                dark.colors.background.is_some(),
                "preset '{name}' dark missing background"
            );
            assert!(
                dark.colors.foreground.is_some(),
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
    fn list_presets_returns_all_seventeen() {
        let names = list_presets();
        assert_eq!(names.len(), 17);
        assert!(names.contains(&"default"));
        assert!(names.contains(&"kde-breeze"));
        assert!(names.contains(&"adwaita"));
        assert!(names.contains(&"windows-11"));
        assert!(names.contains(&"macos-sonoma"));
        assert!(names.contains(&"material"));
        assert!(names.contains(&"ios"));
        assert!(names.contains(&"catppuccin-latte"));
        assert!(names.contains(&"catppuccin-frappe"));
        assert!(names.contains(&"catppuccin-macchiato"));
        assert!(names.contains(&"catppuccin-mocha"));
        assert!(names.contains(&"nord"));
        assert!(names.contains(&"dracula"));
        assert!(names.contains(&"gruvbox"));
        assert!(names.contains(&"solarized"));
        assert!(names.contains(&"tokyo-night"));
        assert!(names.contains(&"one-dark"));
    }

    #[test]
    fn from_toml_minimal_valid() {
        let toml_str = r##"
name = "Minimal"

[light.colors]
accent = "#ff0000"
"##;
        let theme = from_toml(toml_str).unwrap();
        assert_eq!(theme.name, "Minimal");
        assert!(theme.light.is_some());
        let light = theme.light.unwrap();
        assert_eq!(light.colors.accent, Some(crate::Rgba::rgb(255, 0, 0)));
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
        assert_eq!(orig_light.colors.accent, new_light.colors.accent);
    }

    #[test]
    fn from_file_with_tempfile() {
        let dir = std::env::temp_dir();
        let path = dir.join("native_theme_test_preset.toml");
        let toml_str = r##"
name = "File Test"

[light.colors]
accent = "#00ff00"
"##;
        std::fs::write(&path, toml_str).unwrap();

        let theme = from_file(&path).unwrap();
        assert_eq!(theme.name, "File Test");
        assert!(theme.light.is_some());

        // Clean up
        let _ = std::fs::remove_file(&path);
    }

    // === icon_set preset tests ===

    #[test]
    fn icon_set_native_presets_have_correct_values() {
        let cases: &[(&str, &str)] = &[
            ("windows-11", "segoe-fluent"),
            ("macos-sonoma", "sf-symbols"),
            ("ios", "sf-symbols"),
            ("adwaita", "freedesktop"),
            ("kde-breeze", "freedesktop"),
            ("material", "material"),
        ];
        for (name, expected) in cases {
            let theme = preset(name).unwrap();
            let light = theme.light.as_ref().unwrap();
            assert_eq!(
                light.icon_set.as_deref(),
                Some(*expected),
                "preset '{name}' light.icon_set should be Some(\"{expected}\")"
            );
            let dark = theme.dark.as_ref().unwrap();
            assert_eq!(
                dark.icon_set.as_deref(),
                Some(*expected),
                "preset '{name}' dark.icon_set should be Some(\"{expected}\")"
            );
        }
    }

    #[test]
    fn icon_set_community_presets_are_none() {
        let community = &[
            "catppuccin-latte",
            "catppuccin-frappe",
            "catppuccin-macchiato",
            "catppuccin-mocha",
            "nord",
            "dracula",
            "gruvbox",
            "solarized",
            "tokyo-night",
            "one-dark",
            "default",
        ];
        for name in community {
            let theme = preset(name).unwrap();
            let light = theme.light.as_ref().unwrap();
            assert!(
                light.icon_set.is_none(),
                "preset '{name}' light.icon_set should be None"
            );
            let dark = theme.dark.as_ref().unwrap();
            assert!(
                dark.icon_set.is_none(),
                "preset '{name}' dark.icon_set should be None"
            );
        }
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
        assert_eq!(preset("windows-11").unwrap().name, "Windows 11");
        assert_eq!(preset("macos-sonoma").unwrap().name, "macOS Sonoma");
        assert_eq!(preset("material").unwrap().name, "Material");
        assert_eq!(preset("ios").unwrap().name, "iOS");
        assert_eq!(preset("catppuccin-latte").unwrap().name, "Catppuccin Latte");
        assert_eq!(
            preset("catppuccin-frappe").unwrap().name,
            "Catppuccin Frappe"
        );
        assert_eq!(
            preset("catppuccin-macchiato").unwrap().name,
            "Catppuccin Macchiato"
        );
        assert_eq!(preset("catppuccin-mocha").unwrap().name, "Catppuccin Mocha");
        assert_eq!(preset("nord").unwrap().name, "Nord");
        assert_eq!(preset("dracula").unwrap().name, "Dracula");
        assert_eq!(preset("gruvbox").unwrap().name, "Gruvbox");
        assert_eq!(preset("solarized").unwrap().name, "Solarized");
        assert_eq!(preset("tokyo-night").unwrap().name, "Tokyo Night");
        assert_eq!(preset("one-dark").unwrap().name, "One Dark");
    }

    #[test]
    fn all_presets_with_fonts_have_valid_sizes() {
        for name in list_presets() {
            let theme = preset(name).unwrap();
            for (label, variant) in [
                ("light", theme.light.as_ref()),
                ("dark", theme.dark.as_ref()),
            ] {
                let variant = variant.unwrap();
                // Community color themes may omit fonts entirely — skip those.
                if let Some(size) = variant.fonts.size {
                    assert!(
                        size > 0.0,
                        "preset '{name}' {label} font size must be positive, got {size}"
                    );
                }
                if let Some(mono_size) = variant.fonts.mono_size {
                    assert!(
                        mono_size > 0.0,
                        "preset '{name}' {label} mono font size must be positive, got {mono_size}"
                    );
                }
            }
        }
    }
}
