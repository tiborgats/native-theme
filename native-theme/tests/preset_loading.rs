//! Integration tests for the preset loading system.
//!
//! Validates that all bundled presets parse correctly, have both variants
//! populated with non-empty colors, valid font sizes, and survive TOML
//! round-trip serialization.

use native_theme::NativeTheme;

#[test]
fn all_presets_parse_without_error() {
    for name in NativeTheme::list_presets() {
        NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed to parse: {e}"));
    }
}

#[test]
fn all_presets_have_both_variants() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
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
fn all_presets_have_core_colors() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme
            .light
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' missing light variant"));
        let dark = theme
            .dark
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' missing dark variant"));

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
fn all_presets_have_status_colors() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        assert!(
            light.colors.danger.is_some(),
            "preset '{name}' light missing danger"
        );
        assert!(
            light.colors.warning.is_some(),
            "preset '{name}' light missing warning"
        );
        assert!(
            light.colors.success.is_some(),
            "preset '{name}' light missing success"
        );

        assert!(
            dark.colors.danger.is_some(),
            "preset '{name}' dark missing danger"
        );
        assert!(
            dark.colors.warning.is_some(),
            "preset '{name}' dark missing warning"
        );
        assert!(
            dark.colors.success.is_some(),
            "preset '{name}' dark missing success"
        );
    }
}

#[test]
fn all_presets_have_interactive_colors() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        assert!(
            light.colors.selection.is_some(),
            "preset '{name}' light missing selection"
        );
        assert!(
            light.colors.link.is_some(),
            "preset '{name}' light missing link"
        );

        assert!(
            dark.colors.selection.is_some(),
            "preset '{name}' dark missing selection"
        );
        assert!(
            dark.colors.link.is_some(),
            "preset '{name}' dark missing link"
        );
    }
}

#[test]
fn all_presets_have_valid_fonts() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        for (label, variant) in [
            ("light", theme.light.as_ref()),
            ("dark", theme.dark.as_ref()),
        ] {
            let variant =
                variant.unwrap_or_else(|| panic!("preset '{name}' missing {label} variant"));
            assert!(
                variant.fonts.family.is_some(),
                "preset '{name}' {label} missing fonts.family"
            );
            let size = variant.fonts.size.unwrap_or_else(|| {
                panic!("preset '{name}' {label} missing fonts.size")
            });
            assert!(
                size > 0.0,
                "preset '{name}' {label} font size must be > 0, got {size}"
            );
        }
    }
}

#[test]
fn all_presets_have_geometry() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        for (label, variant) in [
            ("light", theme.light.as_ref()),
            ("dark", theme.dark.as_ref()),
        ] {
            let variant =
                variant.unwrap_or_else(|| panic!("preset '{name}' missing {label} variant"));
            assert!(
                variant.geometry.radius.is_some(),
                "preset '{name}' {label} missing geometry.radius"
            );
        }
    }
}

#[test]
fn all_presets_have_spacing() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        for (label, variant) in [
            ("light", theme.light.as_ref()),
            ("dark", theme.dark.as_ref()),
        ] {
            let variant =
                variant.unwrap_or_else(|| panic!("preset '{name}' missing {label} variant"));
            assert!(
                variant.spacing.m.is_some(),
                "preset '{name}' {label} missing spacing.m"
            );
        }
    }
}

#[test]
fn all_presets_round_trip_toml() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let toml_str = theme.to_toml()
            .unwrap_or_else(|e| panic!("preset '{name}' to_toml failed: {e}"));
        let reparsed = NativeTheme::from_toml(&toml_str)
            .unwrap_or_else(|e| panic!("preset '{name}' round-trip from_toml failed: {e}"));

        // Core accent must survive the round-trip
        let orig_light = theme.light.as_ref().unwrap();
        let new_light = reparsed
            .light
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' round-trip lost light variant"));
        assert_eq!(
            orig_light.colors.accent, new_light.colors.accent,
            "preset '{name}' light accent changed after round-trip"
        );

        let orig_dark = theme.dark.as_ref().unwrap();
        let new_dark = reparsed
            .dark
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' round-trip lost dark variant"));
        assert_eq!(
            orig_dark.colors.accent, new_dark.colors.accent,
            "preset '{name}' dark accent changed after round-trip"
        );

        // Name must survive the round-trip
        assert_eq!(
            theme.name, reparsed.name,
            "preset '{name}' name changed after round-trip"
        );
    }
}

#[test]
fn list_presets_returns_seventeen_entries() {
    assert_eq!(
        NativeTheme::list_presets().len(),
        17,
        "expected 17 presets, got {}",
        NativeTheme::list_presets().len()
    );
}

#[test]
fn dark_backgrounds_are_darker() {
    for name in NativeTheme::list_presets() {
        let theme = NativeTheme::preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light_bg = theme
            .light
            .as_ref()
            .unwrap()
            .colors
            .background
            .unwrap_or_else(|| panic!("preset '{name}' light missing background"));
        let dark_bg = theme
            .dark
            .as_ref()
            .unwrap()
            .colors
            .background
            .unwrap_or_else(|| panic!("preset '{name}' dark missing background"));

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
    let names = NativeTheme::list_presets();
    assert!(
        names.contains(&"default"),
        "list_presets() missing 'default'"
    );
    assert!(
        names.contains(&"kde-breeze"),
        "list_presets() missing 'kde-breeze'"
    );
    assert!(
        names.contains(&"adwaita"),
        "list_presets() missing 'adwaita'"
    );
    assert!(
        names.contains(&"windows-11"),
        "list_presets() missing 'windows-11'"
    );
    assert!(
        names.contains(&"macos-sonoma"),
        "list_presets() missing 'macos-sonoma'"
    );
    assert!(
        names.contains(&"material"),
        "list_presets() missing 'material'"
    );
    assert!(
        names.contains(&"ios"),
        "list_presets() missing 'ios'"
    );
    assert!(
        names.contains(&"catppuccin-latte"),
        "list_presets() missing 'catppuccin-latte'"
    );
    assert!(
        names.contains(&"catppuccin-frappe"),
        "list_presets() missing 'catppuccin-frappe'"
    );
    assert!(
        names.contains(&"catppuccin-macchiato"),
        "list_presets() missing 'catppuccin-macchiato'"
    );
    assert!(
        names.contains(&"catppuccin-mocha"),
        "list_presets() missing 'catppuccin-mocha'"
    );
    assert!(
        names.contains(&"nord"),
        "list_presets() missing 'nord'"
    );
    assert!(
        names.contains(&"dracula"),
        "list_presets() missing 'dracula'"
    );
    assert!(
        names.contains(&"gruvbox"),
        "list_presets() missing 'gruvbox'"
    );
    assert!(
        names.contains(&"solarized"),
        "list_presets() missing 'solarized'"
    );
    assert!(
        names.contains(&"tokyo-night"),
        "list_presets() missing 'tokyo-night'"
    );
    assert!(
        names.contains(&"one-dark"),
        "list_presets() missing 'one-dark'"
    );
}
