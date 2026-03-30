//! Bundled theme presets and TOML serialization API.
//!
//! Provides 16 user-facing built-in presets embedded at compile time:
//! 2 core platform (kde-breeze, adwaita), 4 platform (windows-11,
//! macos-sonoma, material, ios), and 10 community (Catppuccin 4 flavors,
//! Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark), plus
//! 4 internal live presets (geometry-only, used by the OS-first pipeline)
//! and functions for loading themes from TOML strings and files.

use crate::{Error, Result, ThemeSpec};
use std::path::Path;
use std::sync::LazyLock;

// Embed preset TOML files at compile time
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

// Live presets: geometry/metrics only (internal, not user-selectable)
const KDE_BREEZE_LIVE_TOML: &str = include_str!("presets/kde-breeze-live.toml");
const ADWAITA_LIVE_TOML: &str = include_str!("presets/adwaita-live.toml");
const MACOS_SONOMA_LIVE_TOML: &str = include_str!("presets/macos-sonoma-live.toml");
const WINDOWS_11_LIVE_TOML: &str = include_str!("presets/windows-11-live.toml");

/// All available user-facing preset names (excludes internal live presets).
const PRESET_NAMES: &[&str] = &[
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

// Cached presets: each parsed at most once for the process lifetime.
// Errors are stored as String (Error is not Clone) and propagated to callers.
mod cached {
    use super::*;

    type Parsed = std::result::Result<ThemeSpec, String>;

    fn parse(toml: &str) -> Parsed {
        from_toml(toml).map_err(|e| e.to_string())
    }

    static KDE_BREEZE: LazyLock<Parsed> = LazyLock::new(|| parse(KDE_BREEZE_TOML));
    static ADWAITA: LazyLock<Parsed> = LazyLock::new(|| parse(ADWAITA_TOML));
    static WINDOWS_11: LazyLock<Parsed> = LazyLock::new(|| parse(WINDOWS_11_TOML));
    static MACOS_SONOMA: LazyLock<Parsed> = LazyLock::new(|| parse(MACOS_SONOMA_TOML));
    static MATERIAL: LazyLock<Parsed> = LazyLock::new(|| parse(MATERIAL_TOML));
    static IOS: LazyLock<Parsed> = LazyLock::new(|| parse(IOS_TOML));
    static CATPPUCCIN_LATTE: LazyLock<Parsed> = LazyLock::new(|| parse(CATPPUCCIN_LATTE_TOML));
    static CATPPUCCIN_FRAPPE: LazyLock<Parsed> = LazyLock::new(|| parse(CATPPUCCIN_FRAPPE_TOML));
    static CATPPUCCIN_MACCHIATO: LazyLock<Parsed> =
        LazyLock::new(|| parse(CATPPUCCIN_MACCHIATO_TOML));
    static CATPPUCCIN_MOCHA: LazyLock<Parsed> = LazyLock::new(|| parse(CATPPUCCIN_MOCHA_TOML));
    static NORD: LazyLock<Parsed> = LazyLock::new(|| parse(NORD_TOML));
    static DRACULA: LazyLock<Parsed> = LazyLock::new(|| parse(DRACULA_TOML));
    static GRUVBOX: LazyLock<Parsed> = LazyLock::new(|| parse(GRUVBOX_TOML));
    static SOLARIZED: LazyLock<Parsed> = LazyLock::new(|| parse(SOLARIZED_TOML));
    static TOKYO_NIGHT: LazyLock<Parsed> = LazyLock::new(|| parse(TOKYO_NIGHT_TOML));
    static ONE_DARK: LazyLock<Parsed> = LazyLock::new(|| parse(ONE_DARK_TOML));
    // Internal live presets
    static KDE_BREEZE_LIVE: LazyLock<Parsed> = LazyLock::new(|| parse(KDE_BREEZE_LIVE_TOML));
    static ADWAITA_LIVE: LazyLock<Parsed> = LazyLock::new(|| parse(ADWAITA_LIVE_TOML));
    static MACOS_SONOMA_LIVE: LazyLock<Parsed> = LazyLock::new(|| parse(MACOS_SONOMA_LIVE_TOML));
    static WINDOWS_11_LIVE: LazyLock<Parsed> = LazyLock::new(|| parse(WINDOWS_11_LIVE_TOML));

    pub(crate) fn get(name: &str) -> Option<&'static Parsed> {
        match name {
            "kde-breeze" => Some(&KDE_BREEZE),
            "adwaita" => Some(&ADWAITA),
            "windows-11" => Some(&WINDOWS_11),
            "macos-sonoma" => Some(&MACOS_SONOMA),
            "material" => Some(&MATERIAL),
            "ios" => Some(&IOS),
            "catppuccin-latte" => Some(&CATPPUCCIN_LATTE),
            "catppuccin-frappe" => Some(&CATPPUCCIN_FRAPPE),
            "catppuccin-macchiato" => Some(&CATPPUCCIN_MACCHIATO),
            "catppuccin-mocha" => Some(&CATPPUCCIN_MOCHA),
            "nord" => Some(&NORD),
            "dracula" => Some(&DRACULA),
            "gruvbox" => Some(&GRUVBOX),
            "solarized" => Some(&SOLARIZED),
            "tokyo-night" => Some(&TOKYO_NIGHT),
            "one-dark" => Some(&ONE_DARK),
            "kde-breeze-live" => Some(&KDE_BREEZE_LIVE),
            "adwaita-live" => Some(&ADWAITA_LIVE),
            "macos-sonoma-live" => Some(&MACOS_SONOMA_LIVE),
            "windows-11-live" => Some(&WINDOWS_11_LIVE),
            _ => None,
        }
    }
}

pub(crate) fn preset(name: &str) -> Result<ThemeSpec> {
    match cached::get(name) {
        None => Err(Error::Unavailable(format!("unknown preset: {name}"))),
        Some(Ok(theme)) => Ok(theme.clone()),
        Some(Err(msg)) => Err(Error::Format(format!("bundled preset '{name}': {msg}"))),
    }
}

pub(crate) fn list_presets() -> &'static [&'static str] {
    PRESET_NAMES
}

/// Platform-specific preset names that should only appear on their native platform.
const PLATFORM_SPECIFIC: &[(&str, &[&str])] = &[
    ("kde-breeze", &["linux-kde"]),
    ("adwaita", &["linux"]),
    ("windows-11", &["windows"]),
    ("macos-sonoma", &["macos"]),
    ("ios", &["macos", "ios"]),
];

/// Detect the current platform tag for preset filtering.
///
/// Returns a string like "linux-kde", "linux", "windows", or "macos".
#[allow(unreachable_code)]
fn detect_platform() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        return "macos";
    }
    #[cfg(target_os = "windows")]
    {
        return "windows";
    }
    #[cfg(target_os = "linux")]
    {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        for component in desktop.split(':') {
            if component == "KDE" {
                return "linux-kde";
            }
        }
        "linux"
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        "linux"
    }
}

/// Returns preset names appropriate for the current platform.
///
/// Platform-specific presets (kde-breeze, adwaita, windows-11, macos-sonoma, ios)
/// are only included on their native platform. Community themes are always included.
pub(crate) fn list_presets_for_platform() -> Vec<&'static str> {
    let platform = detect_platform();

    PRESET_NAMES
        .iter()
        .filter(|name| {
            if let Some((_, platforms)) = PLATFORM_SPECIFIC.iter().find(|(n, _)| n == *name) {
                platforms.iter().any(|p| platform.starts_with(p))
            } else {
                true // Community themes always visible
            }
        })
        .copied()
        .collect()
}

pub(crate) fn from_toml(toml_str: &str) -> Result<ThemeSpec> {
    let theme: ThemeSpec = toml::from_str(toml_str)?;
    Ok(theme)
}

pub(crate) fn from_file(path: impl AsRef<Path>) -> Result<ThemeSpec> {
    let contents = std::fs::read_to_string(path)?;
    from_toml(&contents)
}

pub(crate) fn to_toml(theme: &ThemeSpec) -> Result<String> {
    let s = toml::to_string_pretty(theme)?;
    Ok(s)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
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
                light.defaults.accent.is_some(),
                "preset '{name}' light missing accent"
            );
            assert!(
                light.defaults.background.is_some(),
                "preset '{name}' light missing background"
            );
            assert!(
                light.defaults.foreground.is_some(),
                "preset '{name}' light missing foreground"
            );
            assert!(
                dark.defaults.accent.is_some(),
                "preset '{name}' dark missing accent"
            );
            assert!(
                dark.defaults.background.is_some(),
                "preset '{name}' dark missing background"
            );
            assert!(
                dark.defaults.foreground.is_some(),
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
    fn list_presets_returns_all_sixteen() {
        let names = list_presets();
        assert_eq!(names.len(), 16);
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

[light.defaults]
accent = "#ff0000"
"##;
        let theme = from_toml(toml_str).unwrap();
        assert_eq!(theme.name, "Minimal");
        assert!(theme.light.is_some());
        let light = theme.light.unwrap();
        assert_eq!(light.defaults.accent, Some(crate::Rgba::rgb(255, 0, 0)));
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
        let theme = preset("catppuccin-mocha").unwrap();
        let toml_str = to_toml(&theme).unwrap();

        // Must be parseable back into a ThemeSpec
        let reparsed = from_toml(&toml_str).unwrap();
        assert_eq!(reparsed.name, theme.name);
        assert!(reparsed.light.is_some());
        assert!(reparsed.dark.is_some());

        // Core colors should survive the round-trip
        let orig_light = theme.light.as_ref().unwrap();
        let new_light = reparsed.light.as_ref().unwrap();
        assert_eq!(orig_light.defaults.accent, new_light.defaults.accent);
    }

    #[test]
    fn from_file_with_tempfile() {
        let dir = std::env::temp_dir();
        let path = dir.join("native_theme_test_preset.toml");
        let toml_str = r##"
name = "File Test"

[light.defaults]
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
    fn icon_set_community_presets_are_freedesktop() {
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
        ];
        for name in community {
            let theme = preset(name).unwrap();
            let light = theme.light.as_ref().unwrap();
            assert_eq!(
                light.icon_set.as_deref(),
                Some("freedesktop"),
                "preset '{name}' light.icon_set should be Some(\"freedesktop\")"
            );
            let dark = theme.dark.as_ref().unwrap();
            assert_eq!(
                dark.icon_set.as_deref(),
                Some("freedesktop"),
                "preset '{name}' dark.icon_set should be Some(\"freedesktop\")"
            );
        }
    }

    #[test]
    fn from_file_nonexistent_returns_error() {
        let err = from_file("/tmp/nonexistent_theme_file_12345.toml").unwrap_err();
        match err {
            Error::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
            other => panic!("expected Io, got: {other:?}"),
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
                if let Some(size) = variant.defaults.font.size {
                    assert!(
                        size > 0.0,
                        "preset '{name}' {label} font size must be positive, got {size}"
                    );
                }
                if let Some(mono_size) = variant.defaults.mono_font.size {
                    assert!(
                        mono_size > 0.0,
                        "preset '{name}' {label} mono font size must be positive, got {mono_size}"
                    );
                }
            }
        }
    }

    #[test]
    fn platform_presets_no_derived_fields() {
        // Platform presets must not contain fields that are derived by resolve()
        let platform_presets = &["kde-breeze", "adwaita", "windows-11", "macos-sonoma"];
        for name in platform_presets {
            let theme = preset(name).unwrap();
            for (label, variant_opt) in [
                ("light", theme.light.as_ref()),
                ("dark", theme.dark.as_ref()),
            ] {
                let variant = variant_opt.unwrap();
                // button.primary_bg is derived from accent - should not be in presets
                assert!(
                    variant.button.primary_bg.is_none(),
                    "preset '{name}' {label}.button.primary_bg should be None (derived)"
                );
                // checkbox.checked_bg is derived from accent
                assert!(
                    variant.checkbox.checked_bg.is_none(),
                    "preset '{name}' {label}.checkbox.checked_bg should be None (derived)"
                );
                // slider.fill is derived from accent
                assert!(
                    variant.slider.fill.is_none(),
                    "preset '{name}' {label}.slider.fill should be None (derived)"
                );
                // progress_bar.fill is derived from accent
                assert!(
                    variant.progress_bar.fill.is_none(),
                    "preset '{name}' {label}.progress_bar.fill should be None (derived)"
                );
                // switch.checked_bg is derived from accent
                assert!(
                    variant.switch.checked_bg.is_none(),
                    "preset '{name}' {label}.switch.checked_bg should be None (derived)"
                );
            }
        }
    }

    // === resolve()/validate() integration tests (PRESET-03) ===

    #[test]
    fn all_presets_resolve_validate() {
        for name in list_presets() {
            let theme = preset(name).unwrap();
            if let Some(mut light) = theme.light.clone() {
                light.resolve();
                light.validate().unwrap_or_else(|e| {
                    panic!("preset {name} light variant failed validation: {e}");
                });
            }
            if let Some(mut dark) = theme.dark.clone() {
                dark.resolve();
                dark.validate().unwrap_or_else(|e| {
                    panic!("preset {name} dark variant failed validation: {e}");
                });
            }
        }
    }

    #[test]
    fn resolve_fills_accent_derived_fields() {
        // Load a preset that only has accent set (not explicit widget accent-derived fields).
        // After resolve(), the accent-derived fields should be populated.
        let theme = preset("catppuccin-mocha").unwrap();
        let mut light = theme.light.clone().unwrap();

        // Before resolve: accent-derived fields should be None (not in preset TOML)
        assert!(
            light.button.primary_bg.is_none(),
            "primary_bg should be None pre-resolve"
        );
        assert!(
            light.checkbox.checked_bg.is_none(),
            "checkbox.checked_bg should be None pre-resolve"
        );
        assert!(
            light.slider.fill.is_none(),
            "slider.fill should be None pre-resolve"
        );
        assert!(
            light.progress_bar.fill.is_none(),
            "progress_bar.fill should be None pre-resolve"
        );
        assert!(
            light.switch.checked_bg.is_none(),
            "switch.checked_bg should be None pre-resolve"
        );

        light.resolve();

        // After resolve: all accent-derived fields should equal accent
        let accent = light.defaults.accent.unwrap();
        assert_eq!(
            light.button.primary_bg,
            Some(accent),
            "button.primary_bg should match accent"
        );
        assert_eq!(
            light.checkbox.checked_bg,
            Some(accent),
            "checkbox.checked_bg should match accent"
        );
        assert_eq!(
            light.slider.fill,
            Some(accent),
            "slider.fill should match accent"
        );
        assert_eq!(
            light.progress_bar.fill,
            Some(accent),
            "progress_bar.fill should match accent"
        );
        assert_eq!(
            light.switch.checked_bg,
            Some(accent),
            "switch.checked_bg should match accent"
        );
    }

    #[test]
    fn resolve_then_validate_produces_complete_theme() {
        let theme = preset("catppuccin-mocha").unwrap();
        let mut light = theme.light.clone().unwrap();
        light.resolve();
        let resolved = light.validate().unwrap();

        assert_eq!(resolved.defaults.font.family, "Inter");
        assert_eq!(resolved.defaults.font.size, 14.0);
        assert_eq!(resolved.defaults.font.weight, 400);
        assert_eq!(resolved.defaults.line_height, 1.4);
        assert_eq!(resolved.defaults.radius, 8.0);
        assert_eq!(resolved.defaults.focus_ring_width, 2.0);
        assert_eq!(resolved.defaults.icon_sizes.toolbar, 24.0);
        assert_eq!(resolved.defaults.text_scaling_factor, 1.0);
        assert!(!resolved.defaults.reduce_motion);
        // Window inherits from defaults
        assert_eq!(resolved.window.background, resolved.defaults.background);
        // icon_set should be populated
        assert_eq!(resolved.icon_set, "freedesktop");
    }

    #[test]
    fn font_subfield_inheritance_integration() {
        // Load a preset, set menu.font to only have size=12.0 (clear family/weight),
        // resolve, and verify family/weight are inherited from defaults.
        let theme = preset("catppuccin-mocha").unwrap();
        let mut light = theme.light.clone().unwrap();

        // Set partial font on menu
        use crate::model::FontSpec;
        light.menu.font = Some(FontSpec {
            family: None,
            size: Some(12.0),
            weight: None,
        });

        light.resolve();
        let resolved = light.validate().unwrap();

        // menu font should have inherited family/weight from defaults
        assert_eq!(
            resolved.menu.font.family, "Inter",
            "menu font family should inherit from defaults"
        );
        assert_eq!(
            resolved.menu.font.size, 12.0,
            "menu font size should be the explicit value"
        );
        assert_eq!(
            resolved.menu.font.weight, 400,
            "menu font weight should inherit from defaults"
        );
    }

    #[test]
    fn text_scale_inheritance_integration() {
        // Load a preset, ensure text_scale.caption gets populated from defaults.
        let theme = preset("catppuccin-mocha").unwrap();
        let mut light = theme.light.clone().unwrap();

        // Clear caption to test inheritance
        light.text_scale.caption = None;

        light.resolve();
        let resolved = light.validate().unwrap();

        // caption should have been populated from defaults.font
        assert_eq!(
            resolved.text_scale.caption.size, 14.0,
            "caption size from defaults.font.size"
        );
        assert_eq!(
            resolved.text_scale.caption.weight, 400,
            "caption weight from defaults.font.weight"
        );
        // line_height = defaults.line_height * size = 1.4 * 14.0 = 19.6
        assert!(
            (resolved.text_scale.caption.line_height - 19.6).abs() < 0.01,
            "caption line_height should be line_height_multiplier * size = 19.6, got {}",
            resolved.text_scale.caption.line_height
        );
    }

    #[test]
    fn all_presets_round_trip_exact() {
        // All 16 presets must survive a serde round-trip
        for name in list_presets() {
            let theme1 =
                preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed to parse: {e}"));
            let toml_str = to_toml(&theme1)
                .unwrap_or_else(|e| panic!("preset '{name}' failed to serialize: {e}"));
            let theme2 = from_toml(&toml_str)
                .unwrap_or_else(|e| panic!("preset '{name}' failed to re-parse: {e}"));
            assert_eq!(
                theme1, theme2,
                "preset '{name}' round-trip produced different value"
            );
        }
    }

    // === Live preset tests ===

    #[test]
    fn live_presets_loadable() {
        let live_names = &[
            "kde-breeze-live",
            "adwaita-live",
            "macos-sonoma-live",
            "windows-11-live",
        ];
        for name in live_names {
            let theme = preset(name)
                .unwrap_or_else(|e| panic!("live preset '{name}' failed to parse: {e}"));

            // Both variants must exist
            assert!(
                theme.light.is_some(),
                "live preset '{name}' missing light variant"
            );
            assert!(
                theme.dark.is_some(),
                "live preset '{name}' missing dark variant"
            );

            let light = theme.light.as_ref().unwrap();
            let dark = theme.dark.as_ref().unwrap();

            // No colors
            assert!(
                light.defaults.accent.is_none(),
                "live preset '{name}' light should have no accent"
            );
            assert!(
                light.defaults.background.is_none(),
                "live preset '{name}' light should have no background"
            );
            assert!(
                light.defaults.foreground.is_none(),
                "live preset '{name}' light should have no foreground"
            );
            assert!(
                dark.defaults.accent.is_none(),
                "live preset '{name}' dark should have no accent"
            );
            assert!(
                dark.defaults.background.is_none(),
                "live preset '{name}' dark should have no background"
            );
            assert!(
                dark.defaults.foreground.is_none(),
                "live preset '{name}' dark should have no foreground"
            );

            // No fonts
            assert!(
                light.defaults.font.family.is_none(),
                "live preset '{name}' light should have no font family"
            );
            assert!(
                light.defaults.font.size.is_none(),
                "live preset '{name}' light should have no font size"
            );
            assert!(
                light.defaults.font.weight.is_none(),
                "live preset '{name}' light should have no font weight"
            );
            assert!(
                dark.defaults.font.family.is_none(),
                "live preset '{name}' dark should have no font family"
            );
            assert!(
                dark.defaults.font.size.is_none(),
                "live preset '{name}' dark should have no font size"
            );
            assert!(
                dark.defaults.font.weight.is_none(),
                "live preset '{name}' dark should have no font weight"
            );
        }
    }

    #[test]
    fn list_presets_for_platform_returns_subset() {
        let all = list_presets();
        let filtered = list_presets_for_platform();
        // Filtered list should be a subset of all presets
        for name in &filtered {
            assert!(
                all.contains(name),
                "filtered preset '{name}' not in full list"
            );
        }
        // Community themes should always be present
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
        ];
        for name in community {
            assert!(
                filtered.contains(name),
                "community preset '{name}' should always be in filtered list"
            );
        }
        // material is cross-platform, always present
        assert!(
            filtered.contains(&"material"),
            "material should always be in filtered list"
        );
    }

    #[test]
    fn live_presets_fail_validate_standalone() {
        let live_names = &[
            "kde-breeze-live",
            "adwaita-live",
            "macos-sonoma-live",
            "windows-11-live",
        ];
        for name in live_names {
            let theme = preset(name).unwrap();
            let mut light = theme.light.clone().unwrap();
            light.resolve();
            let result = light.validate();
            assert!(
                result.is_err(),
                "live preset '{name}' light should fail validation standalone (missing colors/fonts)"
            );

            let mut dark = theme.dark.clone().unwrap();
            dark.resolve();
            let result = dark.validate();
            assert!(
                result.is_err(),
                "live preset '{name}' dark should fail validation standalone (missing colors/fonts)"
            );
        }
    }
}
