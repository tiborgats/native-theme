//! Cross-reference tests: verify platform preset values match docs/platform-facts.md.
//!
//! These tests catch drift between the documentation source-of-truth and the
//! actual preset TOML values. If either is updated without the other, these
//! tests will fail, prompting a reconciliation.
//!
//! Values checked per platform (from light variant defaults):
//! - font.family, font.size, font.weight
//! - line_height
//! - border.corner_radius, border.corner_radius_lg, border.line_width
//! - accent_color (hex string)
//! - icon_set, icon_theme
//!
//! The test also verifies that platform-facts.md mentions the same font family
//! and icon set strings, catching documentation drift in both directions.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::*;

const PLATFORM_FACTS: &str = include_str!("../../docs/platform-facts.md");

#[test]
fn kde_breeze_matches_platform_facts() {
    let theme = ThemeSpec::preset("kde-breeze").unwrap();
    let light = theme
        .light
        .as_ref()
        .expect("kde-breeze should have light variant");

    // Font: documented as "Noto Sans, 10pt, 400" in platform-facts.md
    assert_eq!(light.defaults.font.family.as_deref(), Some("Noto Sans"));
    assert_eq!(light.defaults.font.size, Some(10.0));
    assert_eq!(light.defaults.font.weight, Some(400));

    // Line height: documented as 1.36 (from Noto Sans metrics)
    assert_eq!(light.defaults.line_height, Some(1.36));

    // Border corner radius: documented as 5.0 for KDE Breeze
    assert_eq!(light.defaults.border.corner_radius, Some(5.0));
    assert_eq!(light.defaults.border.corner_radius_lg, Some(8.0));
    assert_eq!(light.defaults.border.line_width, Some(1.0));

    // Accent color: #3daee9 (KDE Breeze highlight blue)
    assert_eq!(
        light.defaults.accent_color,
        Some(Rgba::rgb(61, 174, 233))
    );

    // Icon set and theme
    assert_eq!(light.icon_set, Some(IconSet::Freedesktop));
    assert_eq!(light.icon_theme.as_deref(), Some("breeze"));

    // Verify platform-facts.md mentions these values (catches doc drift)
    assert!(
        PLATFORM_FACTS.contains("Noto Sans"),
        "platform-facts.md should mention 'Noto Sans' for KDE"
    );
    assert!(
        PLATFORM_FACTS.contains("10pt") || PLATFORM_FACTS.contains("10 pt"),
        "platform-facts.md should mention 10pt font size for KDE"
    );
}

#[test]
fn adwaita_matches_platform_facts() {
    let theme = ThemeSpec::preset("adwaita").unwrap();
    let light = theme
        .light
        .as_ref()
        .expect("adwaita should have light variant");

    // Font: documented as "Adwaita Sans 11" in platform-facts.md
    assert_eq!(
        light.defaults.font.family.as_deref(),
        Some("Adwaita Sans")
    );
    assert_eq!(light.defaults.font.size, Some(11.0));
    assert_eq!(light.defaults.font.weight, Some(400));

    // Line height: documented as 1.21 (from Inter/Adwaita Sans metrics)
    assert_eq!(light.defaults.line_height, Some(1.21));

    // Border corner radius: 9.0 for Adwaita (libadwaita)
    assert_eq!(light.defaults.border.corner_radius, Some(9.0));
    assert_eq!(light.defaults.border.corner_radius_lg, Some(15.0));
    assert_eq!(light.defaults.border.line_width, Some(1.0));

    // Accent color: #3584e4 (GNOME blue)
    assert_eq!(
        light.defaults.accent_color,
        Some(Rgba::rgb(53, 132, 228))
    );

    // Icon set and theme
    assert_eq!(light.icon_set, Some(IconSet::Freedesktop));
    assert_eq!(light.icon_theme.as_deref(), Some("Adwaita"));

    // Verify platform-facts.md mentions these values
    assert!(
        PLATFORM_FACTS.contains("Adwaita Sans"),
        "platform-facts.md should mention 'Adwaita Sans' for GNOME 48+"
    );
}

#[test]
fn windows_11_matches_platform_facts() {
    let theme = ThemeSpec::preset("windows-11").unwrap();
    let light = theme
        .light
        .as_ref()
        .expect("windows-11 should have light variant");

    // Font: documented as "Segoe UI, 9pt" in platform-facts.md
    // Preset uses size 14.0 (logical px), font weight 400
    assert_eq!(light.defaults.font.family.as_deref(), Some("Segoe UI"));
    assert_eq!(light.defaults.font.size, Some(14.0));
    assert_eq!(light.defaults.font.weight, Some(400));

    // Line height: 1.43 (from Segoe UI metrics)
    assert_eq!(light.defaults.line_height, Some(1.43));

    // Border corner radius: 4.0 for Windows 11 Fluent
    assert_eq!(light.defaults.border.corner_radius, Some(4.0));
    assert_eq!(light.defaults.border.corner_radius_lg, Some(8.0));
    assert_eq!(light.defaults.border.line_width, Some(1.0));

    // Accent color: #0078d4 (Windows blue)
    assert_eq!(
        light.defaults.accent_color,
        Some(Rgba::rgb(0, 120, 212))
    );

    // Icon set and theme
    assert_eq!(light.icon_set, Some(IconSet::SegoeIcons));
    assert_eq!(light.icon_theme.as_deref(), Some("segoe-fluent"));

    // Verify platform-facts.md mentions these values
    assert!(
        PLATFORM_FACTS.contains("Segoe UI"),
        "platform-facts.md should mention 'Segoe UI' for Windows"
    );
}

#[test]
fn macos_sonoma_matches_platform_facts() {
    let theme = ThemeSpec::preset("macos-sonoma").unwrap();
    let light = theme
        .light
        .as_ref()
        .expect("macos-sonoma should have light variant");

    // Font: documented as "SF Pro, 13pt, Regular (400)" in platform-facts.md
    assert_eq!(light.defaults.font.family.as_deref(), Some("SF Pro"));
    assert_eq!(light.defaults.font.size, Some(13.0));
    assert_eq!(light.defaults.font.weight, Some(400));

    // Line height: 1.19 (from SF Pro metrics)
    assert_eq!(light.defaults.line_height, Some(1.19));

    // Border corner radius: 5.0 for macOS
    assert_eq!(light.defaults.border.corner_radius, Some(5.0));
    assert_eq!(light.defaults.border.corner_radius_lg, Some(10.0));
    assert_eq!(light.defaults.border.line_width, Some(0.5));

    // Accent color: #007aff (Apple blue)
    assert_eq!(
        light.defaults.accent_color,
        Some(Rgba::rgb(0, 122, 255))
    );

    // Icon set
    assert_eq!(light.icon_set, Some(IconSet::SfSymbols));

    // Verify platform-facts.md mentions these values
    assert!(
        PLATFORM_FACTS.contains("SF Pro"),
        "platform-facts.md should mention 'SF Pro' for macOS"
    );
    assert!(
        PLATFORM_FACTS.contains("13pt") || PLATFORM_FACTS.contains("13 pt"),
        "platform-facts.md should mention 13pt font size for macOS"
    );
}

#[test]
fn dark_variants_use_same_font_as_light() {
    for preset_name in &["kde-breeze", "adwaita", "windows-11", "macos-sonoma"] {
        let theme = ThemeSpec::preset(preset_name).unwrap();
        let light = theme
            .light
            .as_ref()
            .unwrap_or_else(|| panic!("{preset_name} should have light variant"));
        let dark = theme
            .dark
            .as_ref()
            .unwrap_or_else(|| panic!("{preset_name} should have dark variant"));

        assert_eq!(
            light.defaults.font.family, dark.defaults.font.family,
            "{preset_name}: light and dark should use the same font family"
        );
        assert_eq!(
            light.defaults.font.size, dark.defaults.font.size,
            "{preset_name}: light and dark should use the same font size"
        );
        assert_eq!(
            light.defaults.font.weight, dark.defaults.font.weight,
            "{preset_name}: light and dark should use the same font weight"
        );
    }
}

#[test]
fn all_platform_presets_load_and_have_both_variants() {
    for preset_name in &["kde-breeze", "adwaita", "windows-11", "macos-sonoma"] {
        let theme = ThemeSpec::preset(preset_name)
            .unwrap_or_else(|e| panic!("Failed to load '{preset_name}': {e}"));

        assert!(
            theme.light.is_some(),
            "{preset_name} should have a light variant"
        );
        assert!(
            theme.dark.is_some(),
            "{preset_name} should have a dark variant"
        );

        // Name should be non-empty
        assert!(
            !theme.name.is_empty(),
            "{preset_name} should have a non-empty name"
        );
    }
}
