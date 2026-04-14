//! Bundled theme presets and TOML serialization API.
//!
//! Provides 16 user-facing built-in presets embedded at compile time:
//! 6 platform (kde-breeze, adwaita, windows-11, macos-sonoma, material,
//! ios) and 10 community (Catppuccin 4 flavors, Nord, Dracula, Gruvbox,
//! Solarized, Tokyo Night, One Dark), plus 4 internal live presets
//! (geometry-only, used by the OS-first pipeline) and functions for
//! loading themes from TOML strings and files.

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use crate::{Error, Result, Theme};

/// All preset entries (name + embedded TOML source), parsed once into a HashMap.
///
/// To add a new preset, add it here and (if user-facing) to [`PRESET_NAMES`].
const PRESET_ENTRIES: &[(&str, &str)] = &[
    // Platform presets
    ("kde-breeze", include_str!("presets/kde-breeze.toml")),
    ("adwaita", include_str!("presets/adwaita.toml")),
    ("windows-11", include_str!("presets/windows-11.toml")),
    ("macos-sonoma", include_str!("presets/macos-sonoma.toml")),
    ("material", include_str!("presets/material.toml")),
    ("ios", include_str!("presets/ios.toml")),
    // Community presets
    (
        "catppuccin-latte",
        include_str!("presets/catppuccin-latte.toml"),
    ),
    (
        "catppuccin-frappe",
        include_str!("presets/catppuccin-frappe.toml"),
    ),
    (
        "catppuccin-macchiato",
        include_str!("presets/catppuccin-macchiato.toml"),
    ),
    (
        "catppuccin-mocha",
        include_str!("presets/catppuccin-mocha.toml"),
    ),
    ("nord", include_str!("presets/nord.toml")),
    ("dracula", include_str!("presets/dracula.toml")),
    ("gruvbox", include_str!("presets/gruvbox.toml")),
    ("solarized", include_str!("presets/solarized.toml")),
    ("tokyo-night", include_str!("presets/tokyo-night.toml")),
    ("one-dark", include_str!("presets/one-dark.toml")),
    // Internal live presets (geometry-only, not user-selectable)
    (
        "kde-breeze-live",
        include_str!("presets/kde-breeze-live.toml"),
    ),
    ("adwaita-live", include_str!("presets/adwaita-live.toml")),
    (
        "macos-sonoma-live",
        include_str!("presets/macos-sonoma-live.toml"),
    ),
    (
        "windows-11-live",
        include_str!("presets/windows-11-live.toml"),
    ),
];

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

/// Known display names for bundled presets (borrowed from static strings).
///
/// After TOML deserialization the `Theme.name` field is `Cow::Owned` because
/// TOML parsing cannot borrow into static memory. This map lets the cache
/// replace the owned name with a `Cow::Borrowed` reference, avoiding a
/// per-load `String` allocation for bundled preset names.
const PRESET_DISPLAY_NAMES: &[(&str, &str)] = &[
    ("kde-breeze", "KDE Breeze"),
    ("kde-breeze-live", "KDE Breeze"),
    ("adwaita", "Adwaita"),
    ("adwaita-live", "Adwaita"),
    ("windows-11", "Windows 11"),
    ("windows-11-live", "Windows 11"),
    ("macos-sonoma", "macOS Sonoma"),
    ("macos-sonoma-live", "macOS Sonoma"),
    ("material", "Material"),
    ("ios", "iOS"),
    ("catppuccin-latte", "Catppuccin Latte"),
    ("catppuccin-frappe", "Catppuccin Frappe"),
    ("catppuccin-macchiato", "Catppuccin Macchiato"),
    ("catppuccin-mocha", "Catppuccin Mocha"),
    ("nord", "Nord"),
    ("dracula", "Dracula"),
    ("gruvbox", "Gruvbox"),
    ("solarized", "Solarized"),
    ("tokyo-night", "Tokyo Night"),
    ("one-dark", "One Dark"),
];

type Parsed = std::result::Result<Theme, String>;

fn parse(toml_str: &str) -> Parsed {
    from_toml(toml_str).map_err(|e| e.to_string())
}

static CACHE: LazyLock<HashMap<&str, Parsed>> = LazyLock::new(|| {
    PRESET_ENTRIES
        .iter()
        .map(|(key, toml_str)| {
            let mut result = parse(toml_str);
            // Replace the TOML-deserialized owned name with a borrowed static string
            if let Ok(ref mut theme) = result
                && let Some((_, display_name)) = PRESET_DISPLAY_NAMES.iter().find(|(k, _)| k == key)
            {
                theme.name = Cow::Borrowed(display_name);
            }
            (*key, result)
        })
        .collect()
});

pub(crate) fn preset(name: &str) -> Result<Theme> {
    match CACHE.get(name) {
        None => Err(Error::UnknownPreset {
            name: name.to_string(),
            known: PRESET_NAMES,
        }),
        Some(Ok(theme)) => Ok(theme.clone()),
        Some(Err(msg)) => Err(Error::ReaderFailed {
            reader: "preset_cache",
            source: format!("bundled preset '{name}': {msg}").into(),
        }),
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
        if crate::detect::detect_linux_desktop() == crate::detect::LinuxDesktop::Kde {
            return "linux-kde";
        }
        "linux"
    }
    #[cfg(target_os = "ios")]
    {
        return "ios";
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "windows",
        target_os = "macos",
        target_os = "ios"
    )))]
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

pub(crate) fn from_toml(toml_str: &str) -> Result<Theme> {
    let theme: Theme = toml::from_str(toml_str)?;
    Ok(theme)
}

pub(crate) fn from_file(path: impl AsRef<Path>) -> Result<Theme> {
    let contents = std::fs::read_to_string(path)?;
    from_toml(&contents)
}

pub(crate) fn to_toml(theme: &Theme) -> Result<String> {
    let s = toml::to_string_pretty(theme)?;
    Ok(s)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // NOTE: all_presets_loadable_via_preset_fn and all_presets_have_nonempty_core_colors
    // are covered by tests/preset_loading.rs (all_presets_parse_without_error,
    // all_presets_have_both_variants, all_presets_have_core_colors).

    #[test]
    fn preset_unknown_name_returns_unknown_preset() {
        let err = preset("nonexistent").unwrap_err();
        let Error::UnknownPreset { name, .. } = err else {
            return;
        };
        assert!(name.contains("nonexistent"));
    }

    // NOTE: list_presets_returns_all_sixteen is covered by
    // tests/preset_loading.rs::list_presets_returns_sixteen_entries.

    #[test]
    fn from_toml_minimal_valid() {
        let toml_str = r##"
name = "Minimal"

[light.defaults]
accent_color = "#ff0000"
"##;
        let theme = from_toml(toml_str).unwrap();
        assert_eq!(theme.name, "Minimal");
        assert!(theme.light.is_some());
        let light = theme.light.unwrap();
        assert_eq!(
            light.defaults.accent_color,
            Some(crate::Rgba::rgb(255, 0, 0))
        );
    }

    #[test]
    fn from_toml_invalid_returns_toml_error() {
        let err = from_toml("{{{{invalid toml").unwrap_err();
        assert!(
            matches!(err, Error::Toml(_)),
            "expected Toml variant, got: {err:?}"
        );
    }

    #[test]
    fn to_toml_produces_valid_round_trip() {
        let theme = preset("catppuccin-mocha").unwrap();
        let toml_str = to_toml(&theme).unwrap();

        // Must be parseable back into a Theme
        let reparsed = from_toml(&toml_str).unwrap();
        assert_eq!(reparsed.name, theme.name);
        assert!(reparsed.light.is_some());
        assert!(reparsed.dark.is_some());

        // Core colors should survive the round-trip
        let orig_light = theme.light.as_ref().unwrap();
        let new_light = reparsed.light.as_ref().unwrap();
        assert_eq!(
            orig_light.defaults.accent_color,
            new_light.defaults.accent_color
        );
    }

    #[test]
    fn from_file_with_tempfile() {
        let dir = std::env::temp_dir();
        let path = dir.join("native_theme_test_preset.toml");
        let toml_str = r##"
name = "File Test"

[light.defaults]
accent_color = "#00ff00"
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
        use crate::IconSet;
        let cases: &[(&str, IconSet)] = &[
            ("windows-11", IconSet::SegoeIcons),
            ("macos-sonoma", IconSet::SfSymbols),
            ("ios", IconSet::SfSymbols),
            ("adwaita", IconSet::Freedesktop),
            ("kde-breeze", IconSet::Freedesktop),
            ("material", IconSet::Material),
        ];
        for (name, expected) in cases {
            let theme = preset(name).unwrap();
            assert_eq!(
                theme.icon_set,
                Some(*expected),
                "preset '{name}' icon_set should be Some({expected:?})"
            );
        }
    }

    #[test]
    fn icon_set_community_presets_have_lucide() {
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
            assert_eq!(
                theme.icon_set,
                Some(crate::IconSet::Lucide),
                "preset '{name}' icon_set should be Lucide"
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

    // NOTE: presets_have_correct_names is tested via tests/preset_loading.rs.
    // NOTE: all_presets_with_fonts_have_valid_sizes is covered by
    // tests/preset_loading.rs::all_presets_have_valid_fonts.

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
                // button.primary_background is derived from accent - should not be in presets
                assert!(
                    variant.button.primary_background.is_none(),
                    "preset '{name}' {label}.button.primary_background should be None (derived)"
                );
                // checkbox.checked_background is derived from accent
                assert!(
                    variant.checkbox.checked_background.is_none(),
                    "preset '{name}' {label}.checkbox.checked_background should be None (derived)"
                );
                // slider.fill is derived from accent
                assert!(
                    variant.slider.fill_color.is_none(),
                    "preset '{name}' {label}.slider.fill_color should be None (derived)"
                );
                // progress_bar.fill is derived from accent
                assert!(
                    variant.progress_bar.fill_color.is_none(),
                    "preset '{name}' {label}.progress_bar.fill_color should be None (derived)"
                );
                // switch.checked_background is derived from accent
                assert!(
                    variant.switch.checked_background.is_none(),
                    "preset '{name}' {label}.switch.checked_background should be None (derived)"
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
                light.resolve_all();
                light.validate().unwrap_or_else(|e| {
                    panic!("preset {name} light variant failed validation: {e}");
                });
            }
            if let Some(mut dark) = theme.dark.clone() {
                dark.resolve_all();
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
            light.button.primary_background.is_none(),
            "primary_background should be None pre-resolve"
        );
        assert!(
            light.checkbox.checked_background.is_none(),
            "checkbox.checked_background should be None pre-resolve"
        );
        assert!(
            light.slider.fill_color.is_none(),
            "slider.fill should be None pre-resolve"
        );
        assert!(
            light.progress_bar.fill_color.is_none(),
            "progress_bar.fill should be None pre-resolve"
        );
        assert!(
            light.switch.checked_background.is_none(),
            "switch.checked_background should be None pre-resolve"
        );

        light.resolve();

        // After resolve: all accent-derived fields should equal accent
        let accent = light.defaults.accent_color.unwrap();
        assert_eq!(
            light.button.primary_background,
            Some(accent),
            "button.primary_background should match accent"
        );
        assert_eq!(
            light.checkbox.checked_background,
            Some(accent),
            "checkbox.checked_background should match accent"
        );
        assert_eq!(
            light.slider.fill_color,
            Some(accent),
            "slider.fill should match accent"
        );
        assert_eq!(
            light.progress_bar.fill_color,
            Some(accent),
            "progress_bar.fill should match accent"
        );
        assert_eq!(
            light.switch.checked_background,
            Some(accent),
            "switch.checked_background should match accent"
        );
    }

    #[test]
    fn resolve_then_validate_produces_complete_theme() {
        let theme = preset("catppuccin-mocha").unwrap();
        let mut light = theme.light.clone().unwrap();
        light.resolve_all();
        let resolved = light.validate().unwrap();

        assert_eq!(resolved.defaults.font.family.as_ref(), "Inter");
        assert_eq!(resolved.defaults.font.size, 14.0);
        assert_eq!(resolved.defaults.font.weight, 400);
        assert_eq!(resolved.defaults.line_height, 1.2);
        assert_eq!(resolved.defaults.border.corner_radius, 8.0);
        assert_eq!(resolved.defaults.focus_ring_width, 2.0);
        assert_eq!(resolved.defaults.icon_sizes.toolbar, 24.0);
        assert_eq!(resolved.defaults.icon_sizes.toolbar, 24.0);
        // Window inherits from defaults
        assert_eq!(
            resolved.window.background_color,
            resolved.defaults.background_color
        );
        // icon_set is now on Theme, not on ResolvedTheme
        assert_eq!(theme.icon_set, Some(crate::IconSet::Lucide));
    }

    #[test]
    fn font_subfield_inheritance_integration() {
        // Load a preset, set menu.font to only have size=12.0 (clear family/weight),
        // resolve, and verify family/weight are inherited from defaults.
        let theme = preset("catppuccin-mocha").unwrap();
        let mut light = theme.light.clone().unwrap();

        // Set partial font on menu
        use crate::model::FontSpec;
        use crate::model::font::FontSize;
        light.menu.font = Some(FontSpec {
            family: None,
            size: Some(FontSize::Px(12.0)),
            weight: None,
            ..Default::default()
        });

        light.resolve_all();
        let resolved = light.validate().unwrap();

        // menu font should have inherited family/weight from defaults
        assert_eq!(
            resolved.menu.font.family.as_ref(),
            "Inter",
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

        light.resolve_all();
        let resolved = light.validate().unwrap();

        // caption should have been populated from defaults.font (no ratio)
        // defaults.font.size = 14.0, so caption.size = 14.0
        let expected_size = 14.0;
        assert!(
            (resolved.text_scale.caption.size - expected_size).abs() < 0.01,
            "caption size = defaults.font.size, got {}",
            resolved.text_scale.caption.size
        );
        assert_eq!(
            resolved.text_scale.caption.weight, 400,
            "caption weight from defaults.font.weight"
        );
        // line_height = defaults.line_height * caption_size = 1.2 * 14.0 = 16.8
        let expected_lh = 1.2 * expected_size;
        assert!(
            (resolved.text_scale.caption.line_height - expected_lh).abs() < 0.01,
            "caption line_height should be line_height_multiplier * caption_size = {expected_lh}, got {}",
            resolved.text_scale.caption.line_height
        );
    }

    // NOTE: all_presets_round_trip_exact is covered by
    // tests/preset_loading.rs::all_presets_round_trip_toml.

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
                light.defaults.accent_color.is_none(),
                "live preset '{name}' light should have no accent"
            );
            assert!(
                light.defaults.background_color.is_none(),
                "live preset '{name}' light should have no background"
            );
            assert!(
                light.defaults.text_color.is_none(),
                "live preset '{name}' light should have no foreground"
            );
            assert!(
                dark.defaults.accent_color.is_none(),
                "live preset '{name}' dark should have no accent"
            );
            assert!(
                dark.defaults.background_color.is_none(),
                "live preset '{name}' dark should have no background"
            );
            assert!(
                dark.defaults.text_color.is_none(),
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
