//! Integration tests for the preset loading system.
//!
//! Validates that all bundled presets parse correctly, have both variants
//! populated with non-empty colors, valid font sizes, and survive TOML
//! round-trip serialization.

use native_theme::{list_presets, preset, from_toml, to_toml};

#[test]
fn all_presets_parse_without_error() {
    for name in list_presets() {
        preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed to parse: {e}"));
    }
}

#[test]
fn all_presets_have_both_variants() {
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
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
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme
            .light
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' missing light variant"));
        let dark = theme
            .dark
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' missing dark variant"));

        assert!(
            light.colors.core.accent.is_some(),
            "preset '{name}' light missing core.accent"
        );
        assert!(
            light.colors.core.background.is_some(),
            "preset '{name}' light missing core.background"
        );
        assert!(
            light.colors.core.foreground.is_some(),
            "preset '{name}' light missing core.foreground"
        );

        assert!(
            dark.colors.core.accent.is_some(),
            "preset '{name}' dark missing core.accent"
        );
        assert!(
            dark.colors.core.background.is_some(),
            "preset '{name}' dark missing core.background"
        );
        assert!(
            dark.colors.core.foreground.is_some(),
            "preset '{name}' dark missing core.foreground"
        );
    }
}

#[test]
fn all_presets_have_status_colors() {
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        assert!(
            light.colors.status.danger.is_some(),
            "preset '{name}' light missing status.danger"
        );
        assert!(
            light.colors.status.warning.is_some(),
            "preset '{name}' light missing status.warning"
        );
        assert!(
            light.colors.status.success.is_some(),
            "preset '{name}' light missing status.success"
        );

        assert!(
            dark.colors.status.danger.is_some(),
            "preset '{name}' dark missing status.danger"
        );
        assert!(
            dark.colors.status.warning.is_some(),
            "preset '{name}' dark missing status.warning"
        );
        assert!(
            dark.colors.status.success.is_some(),
            "preset '{name}' dark missing status.success"
        );
    }
}

#[test]
fn all_presets_have_interactive_colors() {
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        assert!(
            light.colors.interactive.selection.is_some(),
            "preset '{name}' light missing interactive.selection"
        );
        assert!(
            light.colors.interactive.link.is_some(),
            "preset '{name}' light missing interactive.link"
        );

        assert!(
            dark.colors.interactive.selection.is_some(),
            "preset '{name}' dark missing interactive.selection"
        );
        assert!(
            dark.colors.interactive.link.is_some(),
            "preset '{name}' dark missing interactive.link"
        );
    }
}

#[test]
fn all_presets_have_valid_fonts() {
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
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
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
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
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
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
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let toml_str = to_toml(&theme)
            .unwrap_or_else(|e| panic!("preset '{name}' to_toml failed: {e}"));
        let reparsed = from_toml(&toml_str)
            .unwrap_or_else(|e| panic!("preset '{name}' round-trip from_toml failed: {e}"));

        // Core accent must survive the round-trip
        let orig_light = theme.light.as_ref().unwrap();
        let new_light = reparsed
            .light
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' round-trip lost light variant"));
        assert_eq!(
            orig_light.colors.core.accent, new_light.colors.core.accent,
            "preset '{name}' light core.accent changed after round-trip"
        );

        let orig_dark = theme.dark.as_ref().unwrap();
        let new_dark = reparsed
            .dark
            .as_ref()
            .unwrap_or_else(|| panic!("preset '{name}' round-trip lost dark variant"));
        assert_eq!(
            orig_dark.colors.core.accent, new_dark.colors.core.accent,
            "preset '{name}' dark core.accent changed after round-trip"
        );

        // Name must survive the round-trip
        assert_eq!(
            theme.name, reparsed.name,
            "preset '{name}' name changed after round-trip"
        );
    }
}

#[test]
fn list_presets_returns_three_entries() {
    assert_eq!(
        list_presets().len(),
        3,
        "expected 3 presets, got {}",
        list_presets().len()
    );
}

#[test]
fn dark_backgrounds_are_darker() {
    for name in list_presets() {
        let theme = preset(name).unwrap_or_else(|e| panic!("preset '{name}' failed: {e}"));
        let light_bg = theme
            .light
            .as_ref()
            .unwrap()
            .colors
            .core
            .background
            .unwrap_or_else(|| panic!("preset '{name}' light missing core.background"));
        let dark_bg = theme
            .dark
            .as_ref()
            .unwrap()
            .colors
            .core
            .background
            .unwrap_or_else(|| panic!("preset '{name}' dark missing core.background"));

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
    let names = list_presets();
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
}
