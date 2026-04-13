//! KDE reader fixture-based integration tests.
//!
//! Tests `from_kde_content_pure` with fixture .ini files to verify
//! parsing logic without any KDE desktop or I/O access.

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![cfg(feature = "kde")]

use native_theme::color::Rgba;
use native_theme::kde::from_kde_content_pure;
use native_theme::model::font::FontSize;
use native_theme::theme::DialogButtonOrder;

// === Breeze Dark (full fixture) ===

#[test]
fn breeze_dark_fixture_colors_and_fonts() {
    let content = include_str!("fixtures/kde/breeze-dark.ini");
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, Some(96.0)).unwrap();

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
    assert!(accessibility.reduce_motion);

    // DPI: caller-provided 96.0, not INI's forceFontDPI=120
    assert_eq!(font_dpi, Some(96.0));

    // Icon theme (per-variant on defaults)
    assert_eq!(v.defaults.icon_theme.as_deref(), Some("breeze-dark"));

    // Icon sizes NOT populated in pure path
    assert!(v.defaults.icon_sizes.small.is_none());

    // Dialog button order: not set by reader (resolver handles it)
    assert!(v.dialog.button_order.is_none());

    // Per-widget: Button
    assert_eq!(v.button.background_color, Some(Rgba::rgb(49, 54, 59)));

    // Per-widget: Tooltip
    assert_eq!(v.tooltip.background_color, Some(Rgba::rgb(49, 54, 59)));

    // Per-widget: Sidebar (Complementary)
    assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));

    // WM: title bar
    assert_eq!(v.window.title_bar_background, Some(Rgba::rgb(49, 54, 59)));

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
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, None).unwrap();
    let v = theme.dark.as_ref().unwrap();
    assert_eq!(font_dpi, Some(120.0));
}

// === Breeze Light ===

#[test]
fn breeze_light_fixture() {
    let content = include_str!("fixtures/kde/breeze-light.ini");
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, Some(96.0)).unwrap();

    // Light theme, not dark
    assert!(theme.light.is_some());
    assert!(theme.dark.is_none());
    assert_eq!(theme.name, "BreezeLight");

    let v = theme.light.as_ref().unwrap();

    // Light-variant colors
    assert_eq!(v.defaults.background_color, Some(Rgba::rgb(239, 240, 241)));
    assert_eq!(v.defaults.surface_color, Some(Rgba::rgb(255, 255, 255)));
    assert_eq!(v.defaults.text_color, Some(Rgba::rgb(35, 38, 41)));
    assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));

    // Pitfall 4: Breeze Light's Complementary group has a dark background
    assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));

    // AnimationDurationFactor=1.0 -> reduce_motion=false
    assert!(!accessibility.reduce_motion);
}

// === Custom Accent (orange) ===

#[test]
fn custom_accent_fixture() {
    let content = include_str!("fixtures/kde/custom-accent.ini");
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, Some(96.0)).unwrap();
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
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, None).unwrap();
    let v = theme.dark.as_ref().unwrap();

    assert_eq!(font_dpi, Some(192.0));
    // forceFontDPI must NOT set text_scaling_factor (Fix 5 from research)
    assert_eq!(accessibility.text_scaling_factor, 1.0);
    // AnimationDurationFactor=1.0 -> reduce_motion=false
    assert!(!accessibility.reduce_motion);
}

// === Minimal Config (only Colors:Window) ===

#[test]
fn minimal_config_fixture() {
    let content = include_str!("fixtures/kde/minimal.ini");
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, Some(96.0)).unwrap();

    // Dark theme (BackgroundNormal=49,54,59 is dark)
    assert!(theme.dark.is_some());
    // No ColorScheme key -> falls back to "KDE"
    assert_eq!(theme.name, "KDE");

    let v = theme.dark.as_ref().unwrap();

    // Only Window fields populated
    assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
    assert_eq!(v.defaults.text_color, Some(Rgba::rgb(239, 240, 241)));

    // No View section -> accent, surface are None
    assert!(v.defaults.accent_color.is_none());
    assert!(v.defaults.surface_color.is_none());

    // No Button section
    assert!(v.button.background_color.is_none());

    // No Tooltip section
    assert!(v.tooltip.background_color.is_none());

    // No Complementary section
    assert!(v.sidebar.background_color.is_none());

    // No General font
    assert!(v.defaults.font.family.is_none());

    // No Icons section (icon_theme is on variant defaults)
    assert!(v.defaults.icon_theme.is_none());

    // No KDE section -> reduce_motion not set
    assert!(!accessibility.reduce_motion);
}

// === Missing Groups (Window + View + Button only) ===

#[test]
fn missing_groups_fixture() {
    let content = include_str!("fixtures/kde/missing-groups.ini");
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, Some(96.0)).unwrap();
    let v = theme.dark.as_ref().unwrap();

    // Present groups work
    assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
    assert_eq!(v.defaults.surface_color, Some(Rgba::rgb(35, 38, 41)));
    assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    assert_eq!(v.button.background_color, Some(Rgba::rgb(49, 54, 59)));

    // Missing WM section
    assert!(v.window.title_bar_background.is_none());
    assert!(v.window.inactive_title_bar_background.is_none());

    // Missing Tooltip section
    assert!(v.tooltip.background_color.is_none());

    // Missing Complementary section
    assert!(v.sidebar.background_color.is_none());

    // Missing Header section
    assert!(v.list.header_background.is_none());

    // Missing Selection section
    assert!(v.defaults.selection_background.is_none());
}

// === Malformed Values (mix of valid and invalid RGB) ===

#[test]
fn malformed_values_fixture() {
    let content = include_str!("fixtures/kde/malformed-values.ini");
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, Some(96.0)).unwrap();
    let v = theme.dark.as_ref().unwrap();

    // Valid Window BackgroundNormal parses
    assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));

    // Malformed Window ForegroundNormal="abc,def,ghi" -> None
    assert!(v.defaults.text_color.is_none());

    // Valid View BackgroundNormal=35,38,41
    assert_eq!(v.defaults.surface_color, Some(Rgba::rgb(35, 38, 41)));

    // View ForegroundNormal="252,252" (2 components) -> None (affects input font, list item font)
    assert!(
        v.input.font.as_ref().and_then(|f| f.color).is_none(),
        "input font color should be None for 2-component ForegroundNormal"
    );

    // Valid View DecorationFocus=61,174,233
    assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));

    // Button BackgroundNormal="" (empty) -> None
    assert!(v.button.background_color.is_none());

    // Button ForegroundNormal="256,0,0" (out of u8 range) -> None
    assert!(v.button.font.is_none());
}

#[test]
fn malformed_values_fixture_dpi_fallback() {
    let content = include_str!("fixtures/kde/malformed-values.ini");
    // Pass None: forceFontDPI="not_a_number" can't parse -> font_dpi is None
    let (theme, font_dpi, accessibility) = from_kde_content_pure(content, None).unwrap();
    let v = theme.dark.as_ref().unwrap();
    assert!(font_dpi.is_none());
}
