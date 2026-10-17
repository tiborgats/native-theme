//! Bundled theme presets and TOML serialization API.
//!
//! Provides 17 user-facing built-in presets embedded at compile time:
//! 7 platform (kde-breeze, adwaita, windows-11, macos-monterey, macos-tahoe,
//! material-3, ios-13) and 10 community (Catppuccin 4 flavors, Nord, Dracula, Gruvbox,
//! Solarized, Tokyo Night, One Dark), and functions for
//! loading themes from TOML strings and files.

use std::borrow::Cow;
use std::path::Path;
use std::sync::OnceLock;

use crate::{Error, Result, Theme};

/// Structured information about a bundled preset.
///
/// Returned by [`Theme::list_presets()`] and [`Theme::list_presets_for_platform()`]
/// to provide richer metadata than bare preset names.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PresetInfo {
    /// Machine-readable preset key (e.g. `"catppuccin-mocha"`, `"kde-breeze"`).
    /// Pass this to [`Theme::preset()`] to load the preset.
    pub key: &'static str,
    /// Human-readable display name (e.g. `"Catppuccin Mocha"`, `"KDE Breeze"`).
    pub display_name: &'static str,
    /// Target platform tags (e.g. `&["linux-kde"]`). Empty for presets offered
    /// on every platform (`material-3` and the community themes).
    pub platforms: &'static [&'static str],
}

/// Builds [`PRESET_INFOS`], [`PRESET_NAMES`] and [`PRESET_ENTRIES`] from one
/// table, in the same order, so the three cannot disagree.
macro_rules! bundled_presets {
    ($($key:literal => $display_name:literal, $platforms:expr;)*) => {
        /// Structured metadata for every bundled preset.
        const PRESET_INFOS: &[PresetInfo] = &[$(PresetInfo {
            key: $key,
            display_name: $display_name,
            platforms: $platforms,
        }),*];

        /// Every bundled preset's key.
        const PRESET_NAMES: &[&str] = &[$($key),*];

        /// Every bundled preset's key and embedded TOML source,
        /// `presets/<key>.toml`.
        const PRESET_ENTRIES: &[(&str, &str)] =
            &[$(($key, include_str!(concat!("presets/", $key, ".toml")))),*];
    };
}

// The bundled presets: key => display name, platform tags. To add a preset,
// add its line here and its `presets/<key>.toml`, whose `name` is the display
// name (a test checks it).
bundled_presets! {
    // Platform presets
    "kde-breeze" => "KDE Breeze", &["linux-kde"];
    "adwaita" => "Adwaita", &["linux"];
    "windows-11" => "Windows 11", &["windows"];
    "macos-monterey" => "macOS Sonoma", &["macos"];
    "macos-tahoe" => "macOS Tahoe", &["macos"];
    "material-3" => "Material", &[];
    "ios-13" => "iOS", &["macos", "ios"];
    // Community presets
    "catppuccin-latte" => "Catppuccin Latte", &[];
    "catppuccin-frappe" => "Catppuccin Frappe", &[];
    "catppuccin-macchiato" => "Catppuccin Macchiato", &[];
    "catppuccin-mocha" => "Catppuccin Mocha", &[];
    "nord" => "Nord", &[];
    "dracula" => "Dracula", &[];
    "gruvbox" => "Gruvbox", &[];
    "solarized" => "Solarized", &[];
    "tokyo-night" => "Tokyo Night", &[];
    "one-dark" => "One Dark", &[];
}

/// Cached parse result. Uses `String` for the error type (not `Error`) because:
/// - `Error` does not implement `Clone`, which is needed for cache retrieval
/// - Preset parse errors are rare (bundled TOML is validated at dev time)
/// - The string-erased error message is sufficient for user diagnostics
type Parsed = std::result::Result<Theme, String>;

fn parse(toml_str: &str) -> Parsed {
    from_toml(toml_str).map_err(|e| e.to_string())
}

/// One cell per [`PRESET_ENTRIES`] entry, at the same position, so a preset's
/// TOML is parsed on its first use only.
static CACHE: [OnceLock<Parsed>; PRESET_ENTRIES.len()] =
    [const { OnceLock::new() }; PRESET_ENTRIES.len()];

pub(crate) fn preset(name: &str) -> Result<Theme> {
    let Some(((_, toml_str), (info, cell))) = PRESET_ENTRIES
        .iter()
        .zip(PRESET_INFOS.iter().zip(&CACHE))
        .find(|((key, _), _)| *key == name)
    else {
        return Err(Error::UnknownPreset {
            name: name.to_string(),
            known: PRESET_NAMES,
        });
    };
    let parsed = cell.get_or_init(|| {
        let mut result = parse(toml_str);
        // The TOML name equals the display name (tested); borrowing the
        // static one saves an owned `String` per load
        if let Ok(ref mut theme) = result {
            theme.name = Cow::Borrowed(info.display_name);
        }
        result
    });
    match parsed {
        Ok(theme) => Ok(theme.clone()),
        Err(msg) => Err(Error::ReaderFailed {
            reader: crate::error::reader_names::PRESET_CACHE,
            source: format!("bundled preset '{name}': {msg}").into(),
        }),
    }
}

pub(crate) fn list_presets() -> &'static [PresetInfo] {
    PRESET_INFOS
}

/// Detect the current platform tag for preset filtering.
///
/// Returns a string like "linux-kde", "linux", "windows", or "macos".
fn detect_platform() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(target_os = "windows")]
    {
        "windows"
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

/// Returns preset info appropriate for the current platform.
///
/// Platform-specific presets (kde-breeze, adwaita, windows-11, macos-monterey, macos-tahoe, ios-13)
/// are only included on their native platform. Community themes are always included.
pub(crate) fn list_presets_for_platform() -> Vec<PresetInfo> {
    let platform = detect_platform();

    PRESET_INFOS
        .iter()
        .filter(|info| {
            info.platforms.is_empty() || info.platforms.iter().any(|p| platform.starts_with(p))
        })
        .cloned()
        .collect()
}

pub(crate) fn from_toml(toml_str: &str) -> Result<Theme> {
    let theme: Theme = toml::from_str(toml_str)?;
    Ok(theme)
}

pub(crate) fn from_file(path: impl AsRef<Path>) -> Result<Theme> {
    let path = path.as_ref();
    let contents = std::fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })?;
    from_toml(&contents)
}

pub(crate) fn to_toml(theme: &Theme) -> Result<String> {
    let s = toml::to_string_pretty(theme)?;
    Ok(s)
}

#[cfg(test)]
mod documented_sizes;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // NOTE: all_presets_loadable_via_preset_fn and all_presets_have_nonempty_core_colors
    // are covered by tests/preset_loading.rs (all_presets_parse_without_error,
    // all_presets_have_both_variants, all_presets_have_core_colors).

    /// Every bundled preset's own source lints clean, shorthand keys and
    /// all. `lint_toml_all_presets_clean` lints `to_toml()` output, which
    /// writes padding sides only, so it cannot see a shorthand key.
    #[test]
    fn every_bundled_preset_source_lints_clean() {
        for (name, source) in PRESET_ENTRIES {
            let warnings = crate::Theme::lint_toml(source)
                .unwrap_or_else(|e| panic!("preset {name} should lint: {e}"));
            assert!(
                warnings.is_empty(),
                "preset {name}'s source has lint warnings: {warnings:?}"
            );
        }
        assert!(
            PRESET_ENTRIES
                .iter()
                .any(|(_, s)| s.contains("padding_horizontal_px")),
            "no bundled preset uses the shorthand any more, so this test no \
             longer proves the linter accepts it"
        );
    }

    /// The cache swaps each preset's TOML `name` for its display name, so
    /// the two must be equal, or the swap would hide the TOML's.
    #[test]
    fn every_bundled_preset_toml_name_is_its_display_name() {
        assert_eq!(PRESET_ENTRIES.len(), PRESET_INFOS.len());
        for ((key, source), info) in PRESET_ENTRIES.iter().zip(PRESET_INFOS) {
            let theme = from_toml(source).unwrap();
            assert_eq!(
                theme.name, info.display_name,
                "preset {key}'s TOML name differs from its display name"
            );
        }
    }

    #[test]
    fn preset_unknown_name_returns_unknown_preset() {
        let err = preset("nonexistent").unwrap_err();
        assert!(
            matches!(err, Error::UnknownPreset { ref name, .. } if name == "nonexistent"),
            "expected UnknownPreset, got: {err:?}"
        );
    }

    // NOTE: list_presets_returns_all_seventeen is covered by
    // tests/preset_loading.rs::list_presets_returns_seventeen_entries.

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
            ("windows-11", IconSet::SegoeFluent),
            ("macos-monterey", IconSet::SfSymbols),
            ("macos-tahoe", IconSet::SfSymbols),
            ("ios-13", IconSet::SfSymbols),
            ("adwaita", IconSet::Freedesktop),
            ("kde-breeze", IconSet::Freedesktop),
            ("material-3", IconSet::Material),
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
            Error::Io { source, .. } => assert_eq!(source.kind(), std::io::ErrorKind::NotFound),
            other => panic!("expected Io, got: {other:?}"),
        }
    }

    #[test]
    fn from_file_error_names_the_path() {
        let path = "/nonexistent/native_theme_missing_12345.toml";
        let err = Theme::from_file(path).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains(path),
            "error message should name the file: {msg}"
        );
    }

    #[test]
    fn preset_names_match_list() {
        // Every name in list_presets() must be loadable via preset()
        for info in list_presets() {
            assert!(
                preset(info.key).is_ok(),
                "preset '{}' not loadable",
                info.key
            );
        }
    }

    // NOTE: presets_have_correct_names is tested via tests/preset_loading.rs.
    // NOTE: all_presets_with_fonts_have_valid_sizes is covered by
    // tests/preset_loading.rs::all_presets_have_valid_fonts.

    #[test]
    fn platform_presets_no_derived_fields() {
        // Platform presets must not contain fields that are derived by resolve()
        let platform_presets = &[
            "kde-breeze",
            "adwaita",
            "windows-11",
            "macos-monterey",
            "macos-tahoe",
        ];
        for name in platform_presets {
            let theme = preset(name).unwrap();
            for (label, variant_opt) in [
                ("light", theme.light.as_ref()),
                ("dark", theme.dark.as_ref()),
            ] {
                let variant = variant_opt.unwrap();
                // button.primary_background is derived from accent, except on
                // Windows, where WinUI 3 paints AccentFillColorDefault instead
                // (docs/platform-facts.md:1175, §2.3)
                assert_eq!(
                    variant.button.primary_background.is_some(),
                    *name == "windows-11",
                    "preset '{name}' {label}.button.primary_background: stated only where \
                     platform-facts does not derive it from the accent"
                );
                // checkbox.checked_background is derived from accent, except on
                // KDE, where Breeze tints the button colour instead
                // (docs/platform-facts.md:1226, §2.5)
                assert_eq!(
                    variant.checkbox.checked_background.is_some(),
                    *name == "kde-breeze",
                    "preset '{name}' {label}.checkbox.checked_background: stated only where \
                     platform-facts does not derive it from the accent"
                );
                // slider.fill is derived from accent
                assert!(
                    variant.slider.fill_color.is_none(),
                    "preset '{name}' {label}.slider.fill_color should be None (derived)"
                );
                // progress_bar.fill is derived from accent, except on KDE,
                // where Breeze lays the accent at 0.7 over the window
                // (docs/platform-facts.md:1320, §2.10)
                assert_eq!(
                    variant.progress_bar.fill_color.is_some(),
                    *name == "kde-breeze",
                    "preset '{name}' {label}.progress_bar.fill_color: stated only where \
                     platform-facts does not derive it from the accent"
                );
                // switch.checked_background is derived from accent, except on
                // KDE, where qqc2-desktop-style lays the highlight at 0.5 over
                // the off track (docs/platform-facts.md §2.21 checked_background)
                assert_eq!(
                    variant.switch.checked_background.is_some(),
                    *name == "kde-breeze",
                    "preset '{name}' {label}.switch.checked_background: stated only where \
                     platform-facts does not derive it from the accent"
                );
            }
        }
    }

    // === resolve()/validate() integration tests (PRESET-03) ===

    #[test]
    fn all_presets_resolve_validate() {
        for info in list_presets() {
            let name = info.key;
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
    fn all_presets_indicator_color_inherits_accent_text_color() {
        // platform-facts.md §2.5 records the check mark / radio dot as the
        // on-accent colour on all four platforms, and it is painted on
        // checkbox.checked_background <- defaults.accent_color.
        for info in list_presets() {
            let name = info.key;
            let theme = preset(name).unwrap();
            for (label, variant_opt) in
                [("light", theme.light.clone()), ("dark", theme.dark.clone())]
            {
                let Some(mut variant) = variant_opt else {
                    continue;
                };
                if variant.checkbox.indicator_color.is_some() {
                    continue;
                }
                variant.resolve_all();
                assert_eq!(
                    variant.checkbox.indicator_color, variant.defaults.accent_text_color,
                    "preset '{name}' {label}.checkbox.indicator_color should inherit \
                     defaults.accent_text_color"
                );
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

    #[test]
    fn list_presets_for_platform_returns_subset() {
        let all = list_presets();
        let filtered = list_presets_for_platform();
        // Filtered list should be a subset of all presets
        for info in &filtered {
            assert!(
                all.iter().any(|a| a.key == info.key),
                "filtered preset '{}' not in full list",
                info.key
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
                filtered.iter().any(|info| info.key == *name),
                "community preset '{name}' should always be in filtered list"
            );
        }
        // material-3 is cross-platform, always present
        assert!(
            filtered.iter().any(|info| info.key == "material-3"),
            "material-3 should always be in filtered list"
        );
    }
}
