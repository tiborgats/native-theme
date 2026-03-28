//! Integration tests for TOML serialization round-trip, sparse deserialization,
//! and serialization skip behavior.
//!
//! These tests exercise the public API exactly as downstream consumers will use
//! it: `use native_theme::*` plus `toml` for (de)serialization.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::*;

// ---------------------------------------------------------------------------
// Helper: build a fully populated ThemeVariant with distinct values
// ---------------------------------------------------------------------------

fn fully_populated_variant(offset: u8) -> ThemeVariant {
    let c = |r: u8, g: u8, b: u8| Rgba::rgb(r.wrapping_add(offset), g, b);

    let mut v = ThemeVariant::default();

    // Core defaults (colors)
    v.defaults.accent = Some(c(61, 174, 233));
    v.defaults.background = Some(c(255, 255, 255));
    v.defaults.foreground = Some(c(35, 38, 41));
    v.defaults.surface = Some(c(239, 240, 241));
    v.defaults.border = Some(c(188, 190, 191));
    v.defaults.muted = Some(c(127, 140, 141));
    v.defaults.shadow = Some(Rgba::rgba(0u8.wrapping_add(offset), 0, 0, 64));

    // Status colors
    v.defaults.danger = Some(c(218, 68, 83));
    v.defaults.danger_foreground = Some(c(252, 252, 252));
    v.defaults.warning = Some(c(246, 116, 0));
    v.defaults.warning_foreground = Some(c(35, 38, 41));
    v.defaults.success = Some(c(39, 174, 96));
    v.defaults.success_foreground = Some(c(252, 252, 252));
    v.defaults.info = Some(c(61, 174, 233));
    v.defaults.info_foreground = Some(c(252, 252, 252));

    // Interactive
    v.defaults.selection = Some(c(61, 174, 233));
    v.defaults.selection_foreground = Some(c(252, 252, 252));
    v.defaults.link = Some(c(41, 128, 185));
    v.defaults.focus_ring_color = Some(c(61, 174, 233));

    // Per-widget: button (primary/secondary)
    v.button.primary_bg = Some(c(61, 174, 233));
    v.button.primary_fg = Some(c(252, 252, 252));
    v.button.background = Some(c(189, 195, 199));
    v.button.foreground = Some(c(49, 54, 59));

    // Per-widget: sidebar
    v.sidebar.background = Some(c(227, 229, 231));
    v.sidebar.foreground = Some(c(35, 38, 41));

    // Per-widget: tooltip
    v.tooltip.background = Some(c(35, 38, 41));
    v.tooltip.foreground = Some(c(252, 252, 252));

    // Per-widget: popover
    v.popover.background = Some(c(255, 255, 255));
    v.popover.foreground = Some(c(35, 38, 41));

    // Per-widget: input
    v.input.background = Some(c(252, 252, 252));
    v.input.foreground = Some(c(35, 38, 41));

    // Per-widget: list
    v.list.alternate_row = Some(c(239, 240, 241));

    // Per-widget: separator
    v.separator.color = Some(c(188, 190, 191));

    // Disabled foreground
    v.defaults.disabled_foreground = Some(c(189, 195, 199));

    // Fonts (via FontSpec on defaults)
    v.defaults.font.family = Some("Noto Sans".into());
    v.defaults.font.size = Some(10.0);
    v.defaults.mono_font.family = Some("Hack".into());
    v.defaults.mono_font.size = Some(10.0);

    // Geometry (on defaults)
    v.defaults.radius = Some(4.0);
    v.defaults.frame_width = Some(1.0);
    v.defaults.disabled_opacity = Some(0.4);
    v.defaults.border_opacity = Some(0.6);

    // Scrollbar width (per-widget)
    v.scrollbar.width = Some(12.0);

    // Spacing (on defaults)
    v.defaults.spacing.xxs = Some(2.0);
    v.defaults.spacing.xs = Some(4.0);
    v.defaults.spacing.s = Some(8.0);
    v.defaults.spacing.m = Some(12.0);
    v.defaults.spacing.l = Some(16.0);
    v.defaults.spacing.xl = Some(24.0);
    v.defaults.spacing.xxl = Some(32.0);

    v
}

// ---------------------------------------------------------------------------
// Round-trip tests
// ---------------------------------------------------------------------------

#[test]
fn round_trip_full_theme() {
    let mut theme = NativeTheme::new("Test Theme");
    theme.light = Some(fully_populated_variant(0));
    theme.dark = Some(fully_populated_variant(10));

    let toml_str = toml::to_string_pretty(&theme).unwrap();
    let deserialized: NativeTheme = toml::from_str(&toml_str).unwrap();

    assert_eq!(deserialized.name, "Test Theme");

    let orig_light = theme.light.as_ref().unwrap();
    let de_light = deserialized.light.as_ref().unwrap();

    // Defaults colors
    assert_eq!(de_light.defaults.accent, orig_light.defaults.accent);
    assert_eq!(de_light.defaults.background, orig_light.defaults.background);
    assert_eq!(de_light.defaults.foreground, orig_light.defaults.foreground);
    assert_eq!(de_light.defaults.danger, orig_light.defaults.danger);
    assert_eq!(de_light.defaults.success, orig_light.defaults.success);
    assert_eq!(de_light.defaults.selection, orig_light.defaults.selection);
    assert_eq!(de_light.defaults.link, orig_light.defaults.link);

    // Per-widget colors
    assert_eq!(de_light.button.primary_bg, orig_light.button.primary_bg);
    assert_eq!(de_light.button.background, orig_light.button.background);
    assert_eq!(de_light.sidebar.background, orig_light.sidebar.background);
    assert_eq!(de_light.tooltip.background, orig_light.tooltip.background);
    assert_eq!(de_light.input.background, orig_light.input.background);

    // Fonts
    assert_eq!(
        de_light.defaults.font.family,
        orig_light.defaults.font.family
    );
    assert_eq!(de_light.defaults.font.size, orig_light.defaults.font.size);
    assert_eq!(
        de_light.defaults.mono_font.family,
        orig_light.defaults.mono_font.family
    );
    assert_eq!(
        de_light.defaults.mono_font.size,
        orig_light.defaults.mono_font.size
    );

    // Geometry
    assert_eq!(de_light.defaults.radius, orig_light.defaults.radius);
    assert_eq!(
        de_light.defaults.frame_width,
        orig_light.defaults.frame_width
    );

    // Spacing
    assert_eq!(de_light.defaults.spacing.m, orig_light.defaults.spacing.m);
    assert_eq!(de_light.defaults.spacing.l, orig_light.defaults.spacing.l);

    // Dark variant spot-checks
    let orig_dark = theme.dark.as_ref().unwrap();
    let de_dark = deserialized.dark.as_ref().unwrap();
    assert_eq!(de_dark.defaults.accent, orig_dark.defaults.accent);
    assert_eq!(de_dark.defaults.font.family, orig_dark.defaults.font.family);
    assert_eq!(de_dark.defaults.radius, orig_dark.defaults.radius);
    assert_eq!(de_dark.defaults.spacing.xxl, orig_dark.defaults.spacing.xxl);
}

#[test]
fn round_trip_preserves_all_color_fields() {
    // Construct a variant with many color fields set to unique values
    let mut v = ThemeVariant::default();

    // Defaults core colors
    v.defaults.accent = Some(Rgba::rgb(1, 0, 0));
    v.defaults.background = Some(Rgba::rgb(2, 0, 0));
    v.defaults.foreground = Some(Rgba::rgb(3, 0, 0));
    v.defaults.surface = Some(Rgba::rgb(4, 0, 0));
    v.defaults.border = Some(Rgba::rgb(5, 0, 0));
    v.defaults.muted = Some(Rgba::rgb(6, 0, 0));
    v.defaults.shadow = Some(Rgba::rgb(7, 0, 0));

    // Defaults status colors
    v.defaults.danger = Some(Rgba::rgb(12, 0, 0));
    v.defaults.danger_foreground = Some(Rgba::rgb(13, 0, 0));
    v.defaults.warning = Some(Rgba::rgb(14, 0, 0));
    v.defaults.warning_foreground = Some(Rgba::rgb(15, 0, 0));
    v.defaults.success = Some(Rgba::rgb(16, 0, 0));
    v.defaults.success_foreground = Some(Rgba::rgb(17, 0, 0));
    v.defaults.info = Some(Rgba::rgb(18, 0, 0));
    v.defaults.info_foreground = Some(Rgba::rgb(19, 0, 0));

    // Defaults interactive colors
    v.defaults.selection = Some(Rgba::rgb(20, 0, 0));
    v.defaults.selection_foreground = Some(Rgba::rgb(21, 0, 0));
    v.defaults.link = Some(Rgba::rgb(22, 0, 0));
    v.defaults.focus_ring_color = Some(Rgba::rgb(23, 0, 0));
    v.defaults.disabled_foreground = Some(Rgba::rgb(34, 0, 0));

    // Per-widget: button
    v.button.primary_bg = Some(Rgba::rgb(8, 0, 0));
    v.button.primary_fg = Some(Rgba::rgb(9, 0, 0));
    v.button.background = Some(Rgba::rgb(10, 0, 0));
    v.button.foreground = Some(Rgba::rgb(11, 0, 0));

    // Per-widget: sidebar
    v.sidebar.background = Some(Rgba::rgb(24, 0, 0));
    v.sidebar.foreground = Some(Rgba::rgb(25, 0, 0));

    // Per-widget: tooltip
    v.tooltip.background = Some(Rgba::rgb(26, 0, 0));
    v.tooltip.foreground = Some(Rgba::rgb(27, 0, 0));

    // Per-widget: popover
    v.popover.background = Some(Rgba::rgb(28, 0, 0));
    v.popover.foreground = Some(Rgba::rgb(29, 0, 0));

    // Per-widget: input
    v.input.background = Some(Rgba::rgb(30, 0, 0));
    v.input.foreground = Some(Rgba::rgb(31, 0, 0));

    // Per-widget: list
    v.list.alternate_row = Some(Rgba::rgb(32, 0, 0));

    // Per-widget: separator
    v.separator.color = Some(Rgba::rgb(33, 0, 0));

    let mut theme = NativeTheme::new("All Colors");
    theme.light = Some(v);

    let toml_str = toml::to_string_pretty(&theme).unwrap();
    let de: NativeTheme = toml::from_str(&toml_str).unwrap();
    let de_v = de.light.as_ref().unwrap();
    let orig_v = theme.light.as_ref().unwrap();

    // Defaults colors
    assert_eq!(de_v.defaults.accent, orig_v.defaults.accent);
    assert_eq!(de_v.defaults.background, orig_v.defaults.background);
    assert_eq!(de_v.defaults.foreground, orig_v.defaults.foreground);
    assert_eq!(de_v.defaults.surface, orig_v.defaults.surface);
    assert_eq!(de_v.defaults.border, orig_v.defaults.border);
    assert_eq!(de_v.defaults.muted, orig_v.defaults.muted);
    assert_eq!(de_v.defaults.shadow, orig_v.defaults.shadow);

    assert_eq!(de_v.defaults.danger, orig_v.defaults.danger);
    assert_eq!(
        de_v.defaults.danger_foreground,
        orig_v.defaults.danger_foreground
    );
    assert_eq!(de_v.defaults.warning, orig_v.defaults.warning);
    assert_eq!(
        de_v.defaults.warning_foreground,
        orig_v.defaults.warning_foreground
    );
    assert_eq!(de_v.defaults.success, orig_v.defaults.success);
    assert_eq!(
        de_v.defaults.success_foreground,
        orig_v.defaults.success_foreground
    );
    assert_eq!(de_v.defaults.info, orig_v.defaults.info);
    assert_eq!(
        de_v.defaults.info_foreground,
        orig_v.defaults.info_foreground
    );

    assert_eq!(de_v.defaults.selection, orig_v.defaults.selection);
    assert_eq!(
        de_v.defaults.selection_foreground,
        orig_v.defaults.selection_foreground
    );
    assert_eq!(de_v.defaults.link, orig_v.defaults.link);
    assert_eq!(
        de_v.defaults.focus_ring_color,
        orig_v.defaults.focus_ring_color
    );
    assert_eq!(
        de_v.defaults.disabled_foreground,
        orig_v.defaults.disabled_foreground
    );

    // Per-widget colors
    assert_eq!(de_v.button.primary_bg, orig_v.button.primary_bg);
    assert_eq!(de_v.button.primary_fg, orig_v.button.primary_fg);
    assert_eq!(de_v.button.background, orig_v.button.background);
    assert_eq!(de_v.button.foreground, orig_v.button.foreground);

    assert_eq!(de_v.sidebar.background, orig_v.sidebar.background);
    assert_eq!(de_v.sidebar.foreground, orig_v.sidebar.foreground);

    assert_eq!(de_v.tooltip.background, orig_v.tooltip.background);
    assert_eq!(de_v.tooltip.foreground, orig_v.tooltip.foreground);

    assert_eq!(de_v.popover.background, orig_v.popover.background);
    assert_eq!(de_v.popover.foreground, orig_v.popover.foreground);

    assert_eq!(de_v.input.background, orig_v.input.background);
    assert_eq!(de_v.input.foreground, orig_v.input.foreground);

    assert_eq!(de_v.list.alternate_row, orig_v.list.alternate_row);
    assert_eq!(de_v.separator.color, orig_v.separator.color);
}

// ---------------------------------------------------------------------------
// Sparse deserialization tests
// ---------------------------------------------------------------------------

#[test]
fn sparse_toml_deserializes() {
    let toml_str = r##"
name = "Minimal"

[light.defaults]
accent = "#3daee9"
"##;

    let theme: NativeTheme = toml::from_str(toml_str).unwrap();

    assert_eq!(theme.name, "Minimal");
    assert_eq!(
        theme.light.as_ref().unwrap().defaults.accent,
        Some(Rgba::rgb(61, 174, 233))
    );
    // All other fields are None/default
    assert!(theme.light.as_ref().unwrap().defaults.background.is_none());
    assert!(theme.light.as_ref().unwrap().defaults.danger.is_none());
    assert!(theme.light.as_ref().unwrap().defaults.font.family.is_none());
    assert!(theme.dark.is_none());
}

#[test]
fn very_sparse_toml_name_only() {
    let toml_str = r#"name = "Empty""#;

    let theme: NativeTheme = toml::from_str(toml_str).unwrap();

    assert_eq!(theme.name, "Empty");
    assert!(theme.light.is_none());
    assert!(theme.dark.is_none());
}

// ---------------------------------------------------------------------------
// Serialization skip tests
// ---------------------------------------------------------------------------

#[test]
fn serialization_skips_none_fields() {
    let mut theme = NativeTheme::new("Skip Test");
    let mut light = ThemeVariant::default();
    light.defaults.accent = Some(Rgba::rgb(61, 174, 233));
    theme.light = Some(light);

    let toml_str = toml::to_string_pretty(&theme).unwrap();

    // accent IS present
    assert!(toml_str.contains("accent"), "TOML should contain 'accent'");

    // None fields are skipped
    assert!(
        !toml_str.contains("background"),
        "TOML should NOT contain 'background' (None)"
    );

    // Empty sub-structs omitted
    assert!(
        !toml_str.contains("[light.button]"),
        "TOML should NOT contain '[light.button]' (empty)"
    );
    assert!(
        !toml_str.contains("[light.sidebar]"),
        "TOML should NOT contain '[light.sidebar]' (empty)"
    );
}

// ---------------------------------------------------------------------------
// TOML structure readability
// ---------------------------------------------------------------------------

#[test]
fn toml_structure_is_human_readable() {
    let mut theme = NativeTheme::new("Readable");
    let mut light = ThemeVariant::default();
    light.defaults.accent = Some(Rgba::rgb(61, 174, 233));
    light.defaults.background = Some(Rgba::rgb(255, 255, 255));
    light.defaults.font.family = Some("Noto Sans".into());
    light.defaults.radius = Some(4.0);
    theme.light = Some(light);

    let toml_str = toml::to_string_pretty(&theme).unwrap();

    // Print for manual inspection during development
    println!("--- Human-readable TOML ---\n{toml_str}---");

    // Verify section header for defaults (where colors, fonts, geometry now live)
    assert!(
        toml_str.contains("[light.defaults]"),
        "expected [light.defaults] section header"
    );

    // Font sub-section under defaults
    assert!(
        toml_str.contains("[light.defaults.font]"),
        "expected [light.defaults.font] section header"
    );

    // Fields appear under correct section (accent under defaults)
    let defaults_section_start = toml_str.find("[light.defaults]").unwrap();
    let accent_pos = toml_str.find("accent").unwrap();
    assert!(
        accent_pos > defaults_section_start,
        "accent should appear after [light.defaults]"
    );
}

// ---------------------------------------------------------------------------
// Rgba hex in TOML
// ---------------------------------------------------------------------------

#[test]
fn rgba_hex_in_toml() {
    let toml_str = r##"
name = "Hex Test"

[light.defaults]
accent = "#3daee9"
shadow = "#00000040"
"##;

    let theme: NativeTheme = toml::from_str(toml_str).unwrap();
    let light = theme.light.as_ref().unwrap();

    // accent: #3daee9 -> r=61, g=174, b=233, a=255 (no alpha => opaque)
    assert_eq!(light.defaults.accent, Some(Rgba::rgb(61, 174, 233)));

    // shadow: #00000040 -> r=0, g=0, b=0, a=0x40=64
    assert_eq!(light.defaults.shadow, Some(Rgba::rgba(0, 0, 0, 64)));

    // Serialize back and verify hex strings are lowercase
    let re_serialized = toml::to_string_pretty(&theme).unwrap();
    assert!(
        re_serialized.contains("#3daee9"),
        "serialized accent should be lowercase hex"
    );
    assert!(
        re_serialized.contains("#00000040"),
        "serialized shadow should be lowercase hex with alpha"
    );
}
