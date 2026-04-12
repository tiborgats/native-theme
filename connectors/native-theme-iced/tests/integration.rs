#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::theme::{ColorMode, Theme};
use native_theme_iced::{from_preset, to_theme};

#[test]
fn all_presets_produce_valid_light_themes() {
    for name in Theme::list_presets() {
        let result = from_preset(name, false);
        assert!(
            result.is_ok(),
            "preset '{name}' should produce a valid light theme: {:?}",
            result.err()
        );
    }
}

#[test]
fn all_presets_produce_valid_dark_themes() {
    for name in Theme::list_presets() {
        let result = from_preset(name, true);
        assert!(
            result.is_ok(),
            "preset '{name}' should produce a valid dark theme: {:?}",
            result.err()
        );
    }
}

#[test]
fn from_system_returns_result() {
    // from_system() may return Err on CI (no OS theme), but must not panic.
    let _ = native_theme_iced::from_system();
}

#[test]
fn to_theme_produces_non_default_palette() {
    let nt = Theme::preset("catppuccin-mocha").unwrap();
    let resolved = nt.into_variant(ColorMode::Dark).unwrap().into_resolved().unwrap();
    let theme = to_theme(&resolved, "catppuccin-mocha");

    let palette = theme.palette();
    let default_palette = iced_core::theme::Palette::DARK;

    // At minimum, background or primary should differ from iced's built-in dark palette
    assert!(
        palette.background != default_palette.background
            || palette.primary != default_palette.primary,
        "to_theme should produce a palette distinct from iced's default"
    );
}

#[test]
fn light_and_dark_palettes_differ() {
    let (light_theme, _) = from_preset("adwaita", false).unwrap();
    let (dark_theme, _) = from_preset("adwaita", true).unwrap();

    assert_ne!(
        light_theme.palette().background,
        dark_theme.palette().background,
        "light and dark adwaita should have different backgrounds"
    );
}
