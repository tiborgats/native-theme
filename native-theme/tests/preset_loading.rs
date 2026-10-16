//! Integration tests for the preset loading system.
//!
//! Validates that all bundled presets parse correctly, have both variants
//! populated with non-empty colors, valid font sizes, and survive TOML
//! round-trip serialization.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::error::Error;
use native_theme::resolve::ResolutionContext;
use native_theme::theme::{ColorMode, Theme};

#[test]
fn all_presets_parse_without_error() {
    for info in Theme::list_presets() {
        let name = info.key;
        let _theme =
            Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed to parse: {e}"));
    }
}

#[test]
fn all_presets_have_both_variants() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        assert!(
            theme.light.is_some(),
            "preset '{name}' missing light variant"
        );
        assert!(theme.dark.is_some(), "preset '{name}' missing dark variant");
    }
}

#[test]
fn all_presets_have_core_colors() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme
            .light
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' missing light variant"));
        let dark = theme
            .dark
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' missing dark variant"));

        assert!(
            light.defaults.accent_color.is_some(),
            "preset '{name}' light missing accent"
        );
        assert!(
            light.defaults.background_color.is_some(),
            "preset '{name}' light missing background"
        );
        assert!(
            light.defaults.text_color.is_some(),
            "preset '{name}' light missing foreground"
        );

        assert!(
            dark.defaults.accent_color.is_some(),
            "preset '{name}' dark missing accent"
        );
        assert!(
            dark.defaults.background_color.is_some(),
            "preset '{name}' dark missing background"
        );
        assert!(
            dark.defaults.text_color.is_some(),
            "preset '{name}' dark missing foreground"
        );
    }
}

#[test]
fn all_presets_have_status_colors() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        assert!(
            light.defaults.danger_color.is_some(),
            "preset '{name}' light missing danger"
        );
        assert!(
            light.defaults.warning_color.is_some(),
            "preset '{name}' light missing warning"
        );
        assert!(
            light.defaults.success_color.is_some(),
            "preset '{name}' light missing success"
        );

        assert!(
            dark.defaults.danger_color.is_some(),
            "preset '{name}' dark missing danger"
        );
        assert!(
            dark.defaults.warning_color.is_some(),
            "preset '{name}' dark missing warning"
        );
        assert!(
            dark.defaults.success_color.is_some(),
            "preset '{name}' dark missing success"
        );
    }
}

#[test]
fn all_presets_have_interactive_colors() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        // A preset may leave the selection to inherit the accent
        // (docs/inheritance-rules.toml, defaults.selection_background).
        assert!(
            light
                .defaults
                .selection_background
                .or(light.defaults.accent_color)
                .is_some(),
            "preset '{name}' light missing selection"
        );
        assert!(
            light.defaults.link_color.is_some(),
            "preset '{name}' light missing link"
        );

        // A preset may leave the selection to inherit the accent
        // (docs/inheritance-rules.toml, defaults.selection_background).
        assert!(
            dark.defaults
                .selection_background
                .or(dark.defaults.accent_color)
                .is_some(),
            "preset '{name}' dark missing selection"
        );
        assert!(
            dark.defaults.link_color.is_some(),
            "preset '{name}' dark missing link"
        );
    }
}

#[test]
fn all_presets_have_valid_fonts() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        for (label, variant) in [
            ("light", theme.light.as_ref()),
            ("dark", theme.dark.as_ref()),
        ] {
            let variant =
                variant.unwrap_or_else(|| panic!("preset '{name}' missing {label} variant"));
            assert!(
                variant.defaults.font.family.is_some(),
                "preset '{name}' {label} missing defaults.font.family"
            );
            let size =
                variant.defaults.font.size.unwrap_or_else(|| {
                    panic!("preset '{name}' {label} missing defaults.font.size")
                });
            assert!(
                size.raw() > 0.0,
                "preset '{name}' {label} font size must be > 0, got {size:?}"
            );
            // font.weight should be set (issue 9a)
            assert!(
                variant.defaults.font.weight.is_some(),
                "preset '{name}' {label} missing defaults.font.weight"
            );
            // mono_font checks (issue 24k)
            assert!(
                variant.defaults.mono_font.family.is_some(),
                "preset '{name}' {label} missing defaults.mono_font.family"
            );
            if let Some(mono_size) = variant.defaults.mono_font.size {
                assert!(
                    mono_size.raw() > 0.0,
                    "preset '{name}' {label} mono font size must be > 0, got {mono_size:?}"
                );
            }
        }
    }
}

#[test]
fn all_presets_have_geometry() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        for (label, variant) in [
            ("light", theme.light.as_ref()),
            ("dark", theme.dark.as_ref()),
        ] {
            let variant =
                variant.unwrap_or_else(|| panic!("preset '{name}' missing {label} variant"));
            assert!(
                variant.defaults.border.corner_radius.is_some(),
                "preset '{name}' {label} missing defaults.border.corner_radius"
            );
            assert!(
                variant.defaults.border.corner_radius_lg.is_some(),
                "preset '{name}' {label} missing defaults.border.corner_radius_lg"
            );
            assert!(
                variant.defaults.border.shadow_enabled.is_some(),
                "preset '{name}' {label} missing defaults.shadow_enabled"
            );
            // Extended checks (issues 24l, 24m)
            assert!(
                variant.defaults.border.line_width.is_some(),
                "preset '{name}' {label} missing defaults.frame_width"
            );
            assert!(
                variant.defaults.line_height.is_some(),
                "preset '{name}' {label} missing defaults.line_height"
            );
        }
    }
}

#[test]
fn all_presets_have_spacing() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        for (label, variant) in [
            ("light", theme.light.as_ref()),
            ("dark", theme.dark.as_ref()),
        ] {
            let variant =
                variant.unwrap_or_else(|| panic!("preset '{name}' missing {label} variant"));
            // REMOVED: spacing checks (ThemeSpacing deleted in Plan 01)
            let _ = variant;
        }
    }
}

#[test]
fn all_presets_round_trip_toml() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let toml_str = theme
            .to_toml()
            .unwrap_or_else(|e| panic!("preset '{name}' to_toml failed: {e}"));
        let reparsed = Theme::from_toml(&toml_str)
            .unwrap_or_else(|e| panic!("preset '{name}' round-trip from_toml failed: {e}"));

        // Core accent must survive the round-trip
        let orig_light = theme.light.as_ref().unwrap();
        let new_light = reparsed
            .light
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' round-trip lost light variant"));
        assert_eq!(
            orig_light.defaults.accent_color, new_light.defaults.accent_color,
            "preset '{name}' light accent changed after round-trip"
        );

        let orig_dark = theme.dark.as_ref().unwrap();
        let new_dark = reparsed
            .dark
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' round-trip lost dark variant"));
        assert_eq!(
            orig_dark.defaults.accent_color, new_dark.defaults.accent_color,
            "preset '{name}' dark accent changed after round-trip"
        );

        // Name must survive the round-trip
        assert_eq!(
            theme.name, reparsed.name,
            "preset '{name}' name changed after round-trip"
        );
    }
}

/// ios-13's caption states `size_pt` and `line_height_pt`; an overlay
/// stating `size_px` leaves a merged entry in mixed units, which `to_toml`
/// writes as `size_px` with `line_height_pt` and `from_toml` must read back.
#[test]
fn merged_mixed_unit_text_scale_round_trips_toml() {
    let mut theme = Theme::preset("ios-13").unwrap();
    let overlay = Theme::from_toml("[light.text_scale.caption]\nsize_px = 12.0\n").unwrap();
    theme.merge(&overlay);

    let toml_str = theme.to_toml().unwrap();
    let reparsed = Theme::from_toml(&toml_str)
        .unwrap_or_else(|e| panic!("merged theme round-trip from_toml failed: {e}"));
    assert_eq!(theme, reparsed);
}

/// A caption stating `size_pt` with `line_height_px` resolves each value in
/// its own unit: at 96 DPI, 12 pt is 16 px and the 20 px line height stays 20.
#[test]
fn mixed_unit_text_scale_resolves_each_value_in_its_own_unit() {
    let mut theme = Theme::preset("ios-13").unwrap();
    let overlay = Theme::from_toml("[light.text_scale.caption]\nline_height_px = 20.0\n").unwrap();
    theme.merge(&overlay);
    let theme = Theme::from_toml(&theme.to_toml().unwrap()).unwrap();

    let ctx = ResolutionContext {
        font_dpi: 96.0,
        ..ResolutionContext::for_tests()
    };
    let caption = theme
        .resolve_with(ColorMode::Light, &ctx)
        .unwrap()
        .variant
        .text_scale
        .caption;
    assert!(
        (caption.size - 16.0).abs() < 0.01,
        "12 pt at 96 DPI should be 16 px, got {}",
        caption.size
    );
    assert!(
        (caption.line_height - 20.0).abs() < 0.01,
        "line_height_px = 20.0 should stay 20 px, got {}",
        caption.line_height
    );
}

#[test]
fn list_presets_returns_seventeen_entries() {
    assert_eq!(
        Theme::list_presets().len(),
        17,
        "expected 17 presets, got {}",
        Theme::list_presets().len()
    );
}

#[test]
fn dark_backgrounds_are_darker() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light_bg = theme
            .light
            .as_ref()
            .unwrap()
            .defaults
            .background_color
            .unwrap_or_else(|| panic!("preset '{name}' light missing background"));
        let dark_bg = theme
            .dark
            .as_ref()
            .unwrap()
            .defaults
            .background_color
            .unwrap_or_else(|| panic!("preset '{name}' dark missing background"));

        // Naive RGB sum (r+g+b < threshold) is sufficient for binary dark/light
        // classification. BT.601 weighted luma would be more accurate but unnecessary here.
        let light_sum = light_bg.r as u16 + light_bg.g as u16 + light_bg.b as u16;
        let dark_sum = dark_bg.r as u16 + dark_bg.g as u16 + dark_bg.b as u16;

        assert!(
            light_sum > dark_sum,
            "preset '{name}' dark background ({dark_bg}) is not darker than light background ({light_bg}): \
             light RGB sum {light_sum} should be > dark RGB sum {dark_sum}"
        );
    }
}

#[test]
fn preset_names_are_correct() {
    let presets = Theme::list_presets();
    let has_key = |k: &str| presets.iter().any(|info| info.key == k);
    assert!(has_key("kde-breeze"), "list_presets() missing 'kde-breeze'");
    assert!(has_key("adwaita"), "list_presets() missing 'adwaita'");
    assert!(has_key("windows-11"), "list_presets() missing 'windows-11'");
    assert!(
        has_key("macos-monterey"),
        "list_presets() missing 'macos-monterey'"
    );
    assert!(
        has_key("macos-tahoe"),
        "list_presets() missing 'macos-tahoe'"
    );
    assert!(has_key("material-3"), "list_presets() missing 'material-3'");
    assert!(has_key("ios-13"), "list_presets() missing 'ios-13'");
    assert!(
        has_key("catppuccin-latte"),
        "list_presets() missing 'catppuccin-latte'"
    );
    assert!(
        has_key("catppuccin-frappe"),
        "list_presets() missing 'catppuccin-frappe'"
    );
    assert!(
        has_key("catppuccin-macchiato"),
        "list_presets() missing 'catppuccin-macchiato'"
    );
    assert!(
        has_key("catppuccin-mocha"),
        "list_presets() missing 'catppuccin-mocha'"
    );
    assert!(has_key("nord"), "list_presets() missing 'nord'");
    assert!(has_key("dracula"), "list_presets() missing 'dracula'");
    assert!(has_key("gruvbox"), "list_presets() missing 'gruvbox'");
    assert!(has_key("solarized"), "list_presets() missing 'solarized'");
    assert!(
        has_key("tokyo-night"),
        "list_presets() missing 'tokyo-night'"
    );
    assert!(has_key("one-dark"), "list_presets() missing 'one-dark'");
}

#[test]
fn material_preset_is_named_by_its_generation() {
    assert!(Theme::preset("material-3").is_ok());
    assert!(Theme::preset("material").is_err());
    assert!(Theme::list_presets().iter().any(|p| p.key == "material-3"));
}

#[test]
fn macos_preset_is_named_by_its_generation() {
    assert!(Theme::preset("macos-monterey").is_ok());
    assert!(Theme::preset("macos-tahoe").is_ok());
    assert!(matches!(
        Theme::preset("macos-sonoma"),
        Err(Error::UnknownPreset { .. })
    ));
    assert!(
        Theme::list_presets()
            .iter()
            .any(|p| p.key == "macos-monterey")
    );
}

#[test]
fn ios_preset_is_named_by_its_generation() {
    assert!(Theme::preset("ios-13").is_ok());
    assert!(matches!(
        Theme::preset("ios"),
        Err(Error::UnknownPreset { .. })
    ));
    assert!(Theme::list_presets().iter().any(|p| p.key == "ios-13"));
}

#[test]
fn live_preset_tier_is_gone() {
    for name in [
        "kde-breeze-live",
        "adwaita-live",
        "macos-sonoma-live",
        "windows-11-live",
    ] {
        assert!(Theme::preset(name).is_err(), "{name} still loads");
    }
}
