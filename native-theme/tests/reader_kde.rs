//! KDE reader fixture-based integration tests.
//!
//! Tests `from_kde_content_pure` with fixture .ini files to verify
//! parsing logic without any KDE desktop or I/O access.

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![cfg(feature = "kde")]

use native_theme::kde::from_kde_content_pure;
use native_theme::model::font::FontSize;
use native_theme::{DialogButtonOrder, Rgba};

// === Breeze Dark (full fixture) ===

#[test]
fn breeze_dark_fixture_colors_and_fonts() {
    let content = include_str!("fixtures/kde/breeze-dark.ini");
    let theme = from_kde_content_pure(content, Some(96.0)).unwrap();

    // Dark theme, not light
    assert!(theme.dark.is_some());
    assert!(theme.light.is_none());
    assert_eq!(theme.name, "BreezeDark");

    let v = theme.dark.as_ref().unwrap();

    // defaults-level colors
    assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
    assert_eq!(v.defaults.text_color, Some(Rgba::rgb(239, 240, 241)));
    assert_eq!(v.defaults.surface_color, Some(Rgba::rgb(35, 38, 41)));
    assert_eq!(v.defaults.focus_ring_color, Some(Rgba::rgb(61, 174, 233)));
    assert_eq!(
        v.defaults.selection_background,
        Some(Rgba::rgb(61, 174, 233))
    );

    // Fonts
    assert_eq!(v.defaults.font.family.as_deref(), Some("Noto Sans"));
    assert_eq!(v.defaults.font.size, Some(FontSize::Pt(10.0)));
    assert_eq!(v.defaults.mono_font.family.as_deref(), Some("Hack"));

    // Accessibility: AnimationDurationFactor=0 -> reduce_motion=true
    assert_eq!(v.defaults.reduce_motion, Some(true));

    // DPI: caller-provided 96.0, not INI's forceFontDPI=120
    assert_eq!(v.defaults.font_dpi, Some(96.0));

    // Icon theme
    assert_eq!(v.icon_theme.as_deref(), Some("breeze-dark"));

    // Icon sizes NOT populated in pure path
    assert!(v.defaults.icon_sizes.small.is_none());

    // Dialog button order: KDE uses PrimaryLeft
    assert_eq!(v.dialog.button_order, Some(DialogButtonOrder::PrimaryLeft));

    // Per-widget: Button
    assert_eq!(v.button.background_color, Some(Rgba::rgb(49, 54, 59)));

    // Per-widget: Tooltip
    assert_eq!(v.tooltip.background_color, Some(Rgba::rgb(49, 54, 59)));

    // Per-widget: Sidebar (Complementary)
    assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));

    // WM: title bar
    assert_eq!(
        v.window.title_bar_background,
        Some(Rgba::rgb(49, 54, 59))
    );

    // Header (list)
    assert_eq!(v.list.header_background, Some(Rgba::rgb(35, 38, 41)));

    // Menu font: Noto Sans 9pt
    let menu_font = v.menu.font.as_ref().unwrap();
    assert_eq!(menu_font.family.as_deref(), Some("Noto Sans"));
    assert_eq!(menu_font.size, Some(FontSize::Pt(9.0)));

    // Toolbar font: 8pt
    let toolbar_font = v.toolbar.font.as_ref().unwrap();
    assert_eq!(toolbar_font.size, Some(FontSize::Pt(8.0)));
}

#[test]
fn breeze_dark_fixture_dpi_from_ini() {
    let content = include_str!("fixtures/kde/breeze-dark.ini");
    // Pass None to let INI extraction (forceFontDPI=120) be used
    let theme = from_kde_content_pure(content, None).unwrap();
    let v = theme.dark.as_ref().unwrap();
    assert_eq!(v.defaults.font_dpi, Some(120.0));
}

// === Breeze Light ===

#[test]
fn breeze_light_fixture() {
    let content = include_str!("fixtures/kde/breeze-light.ini");
    let theme = from_kde_content_pure(content, Some(96.0)).unwrap();

    // Light theme, not dark
    assert!(theme.light.is_some());
    assert!(theme.dark.is_none());
    assert_eq!(theme.name, "BreezeLight");

    let v = theme.light.as_ref().unwrap();

    // Light-variant colors
    assert_eq!(v.defaults.background_color, Some(Rgba::rgb(239, 240, 241)));
    assert_eq!(
        v.defaults.surface_color,
        Some(Rgba::rgb(255, 255, 255))
    );
    assert_eq!(v.defaults.text_color, Some(Rgba::rgb(35, 38, 41)));
    assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));

    // Pitfall 4: Breeze Light's Complementary group has a dark background
    assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));

    // AnimationDurationFactor=1.0 -> reduce_motion=false
    assert_eq!(v.defaults.reduce_motion, Some(false));
}

// === Custom Accent (orange) ===

#[test]
fn custom_accent_fixture() {
    let content = include_str!("fixtures/kde/custom-accent.ini");
    let theme = from_kde_content_pure(content, Some(96.0)).unwrap();
    let v = theme.dark.as_ref().unwrap();

    // Orange accent (246,116,0) replaces default Breeze blue
    assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(246, 116, 0)));
    assert_eq!(
        v.defaults.selection_background,
        Some(Rgba::rgb(246, 116, 0))
    );
    assert_eq!(v.defaults.focus_ring_color, Some(Rgba::rgb(246, 116, 0)));

    // Window background unchanged from Breeze Dark
    assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
}

// === High DPI ===

#[test]
fn high_dpi_fixture() {
    let content = include_str!("fixtures/kde/high-dpi.ini");
    // Pass None to let INI extraction of forceFontDPI=192
    let theme = from_kde_content_pure(content, None).unwrap();
    let v = theme.dark.as_ref().unwrap();

    assert_eq!(v.defaults.font_dpi, Some(192.0));
    // forceFontDPI must NOT set text_scaling_factor (Fix 5 from research)
    assert!(v.defaults.text_scaling_factor.is_none());
    // AnimationDurationFactor=1.0 -> reduce_motion=false
    assert_eq!(v.defaults.reduce_motion, Some(false));
}
