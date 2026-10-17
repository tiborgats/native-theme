#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::theme::{ColorMode, Theme};
use native_theme_iced::{from_preset, to_theme};

#[test]
fn all_presets_produce_valid_light_themes() {
    for info in Theme::list_presets() {
        let name = info.key;
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
    for info in Theme::list_presets() {
        let name = info.key;
        let result = from_preset(name, true);
        assert!(
            result.is_ok(),
            "preset '{name}' should produce a valid dark theme: {:?}",
            result.err()
        );
    }
}

/// `from_system` never fails on Linux: a desktop with no reader falls back to
/// the platform preset (native-theme `pipeline.rs`, `linux_fallback`). Off
/// Linux a build without a reader feature has no OS theme, and the error must
/// come back as one, not as a panic.
#[test]
fn from_system_is_ok_on_linux() {
    let loaded = native_theme_iced::from_system();
    if cfg!(target_os = "linux") {
        assert!(
            loaded.is_ok(),
            "Linux falls back to the platform preset: {:?}",
            loaded.err()
        );
    }
}

#[test]
fn to_theme_produces_non_default_palette() {
    let nt = Theme::preset("catppuccin-mocha").unwrap();
    let resolved = nt
        .into_variant(ColorMode::Dark)
        .unwrap()
        .into_resolved(&native_theme::ResolutionContext::for_tests())
        .unwrap();
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

/// The connector re-exports native-theme, so an application names its API
/// through `native_theme_iced::native_theme` with no dependency of its own,
/// and it is the copy the connector's own types come from.
#[test]
fn native_theme_is_reached_through_the_connector() {
    let _: fn() = native_theme_iced::native_theme::detect::invalidate_caches;
    let theme: native_theme_iced::native_theme::theme::Theme = Theme::preset("adwaita").unwrap();
    let resolved = theme.resolve(ColorMode::Light).unwrap();
    let _ = to_theme(&resolved.variant, &theme.name);
}
