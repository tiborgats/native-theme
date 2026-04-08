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
    v.defaults.accent_color = Some(c(61, 174, 233));
    v.defaults.background_color = Some(c(255, 255, 255));
    v.defaults.text_color = Some(c(35, 38, 41));
    v.defaults.surface_color = Some(c(239, 240, 241));
    v.defaults.border.color = Some(c(188, 190, 191));
    v.defaults.muted_color = Some(c(127, 140, 141));
    v.defaults.shadow_color = Some(Rgba::rgba(0u8.wrapping_add(offset), 0, 0, 64));

    // Status colors
    v.defaults.danger_color = Some(c(218, 68, 83));
    v.defaults.danger_text_color = Some(c(252, 252, 252));
    v.defaults.warning_color = Some(c(246, 116, 0));
    v.defaults.warning_text_color = Some(c(35, 38, 41));
    v.defaults.success_color = Some(c(39, 174, 96));
    v.defaults.success_text_color = Some(c(252, 252, 252));
    v.defaults.info_color = Some(c(61, 174, 233));
    v.defaults.info_text_color = Some(c(252, 252, 252));

    // Interactive
    v.defaults.selection_background = Some(c(61, 174, 233));
    v.defaults.selection_text_color = Some(c(252, 252, 252));
    v.defaults.link_color = Some(c(41, 128, 185));
    v.defaults.focus_ring_color = Some(c(61, 174, 233));

    // Per-widget: button (primary/secondary)
    v.button.primary_background = Some(c(61, 174, 233));
    v.button.primary_text_color = Some(c(252, 252, 252));
    v.button.background_color = Some(c(189, 195, 199));
    v.button.font.get_or_insert_default().color = Some(c(49, 54, 59));

    // Per-widget: sidebar
    v.sidebar.background_color = Some(c(227, 229, 231));
    v.sidebar.font.get_or_insert_default().color = Some(c(35, 38, 41));

    // Per-widget: tooltip
    v.tooltip.background_color = Some(c(35, 38, 41));
    v.tooltip.font.get_or_insert_default().color = Some(c(252, 252, 252));

    // Per-widget: popover
    v.popover.background_color = Some(c(255, 255, 255));
    v.popover.font.get_or_insert_default().color = Some(c(35, 38, 41));

    // Per-widget: input
    v.input.background_color = Some(c(252, 252, 252));
    v.input.font.get_or_insert_default().color = Some(c(35, 38, 41));

    // Per-widget: list
    v.list.alternate_row_background = Some(c(239, 240, 241));

    // Per-widget: separator
    v.separator.line_color = Some(c(188, 190, 191));

    // Disabled foreground
    v.defaults.disabled_text_color = Some(c(189, 195, 199));

    // Fonts (via FontSpec on defaults)
    v.defaults.font.family = Some("Noto Sans".into());
    v.defaults.font.size = Some(FontSize::Px(10.0));
    v.defaults.mono_font.family = Some("Hack".into());
    v.defaults.mono_font.size = Some(FontSize::Px(10.0));

    // Missing fields added for issue 24n
    v.defaults.accent_text_color = Some(c(252, 252, 252));
    v.defaults.selection_inactive_background = Some(c(180, 210, 233));
    v.defaults.line_height = Some(1.2);
    v.defaults.border.corner_radius_lg = Some(8.0);
    v.defaults.border.shadow_enabled = Some(true);

    // Geometry (on defaults)
    v.defaults.border.corner_radius = Some(4.0);
    v.defaults.border.line_width = Some(1.0);
    v.defaults.disabled_opacity = Some(0.4);
    v.defaults.border.opacity = Some(0.6);

    // Scrollbar width (per-widget)
    v.scrollbar.groove_width = Some(12.0);

    // Spacing (on defaults)
    // REMOVED(spacing): v.defaults.spacing.xxs = Some(2.0);
    // REMOVED(spacing): v.defaults.spacing.xs = Some(4.0);
    // REMOVED(spacing): v.defaults.spacing.s = Some(8.0);
    // REMOVED(spacing): v.defaults.spacing.m = Some(12.0);
    // REMOVED(spacing): v.defaults.spacing.l = Some(16.0);
    // REMOVED(spacing): v.defaults.spacing.xl = Some(24.0);
    // REMOVED(spacing): v.defaults.spacing.xxl = Some(32.0);

    v
}

// ---------------------------------------------------------------------------
// Round-trip tests
// ---------------------------------------------------------------------------

#[test]
fn round_trip_full_theme() {
    let mut theme = ThemeSpec::new("Test Theme");
    theme.light = Some(fully_populated_variant(0));
    theme.dark = Some(fully_populated_variant(10));

    let toml_str = toml::to_string_pretty(&theme).unwrap();
    let deserialized: ThemeSpec = toml::from_str(&toml_str).unwrap();

    // Full structural equality (issue 9e/24w)
    assert_eq!(
        theme, deserialized,
        "round-trip should produce structurally identical ThemeSpec"
    );
}

/// Round-trip test using an actual preset (issue 24w: ensures all widget sections are covered).
#[test]
fn round_trip_actual_preset() {
    let theme = ThemeSpec::preset("kde-breeze").unwrap();
    let toml_str = toml::to_string_pretty(&theme).unwrap();
    let deserialized: ThemeSpec = toml::from_str(&toml_str).unwrap();
    assert_eq!(
        theme, deserialized,
        "kde-breeze preset round-trip should produce structurally identical ThemeSpec"
    );
}

#[test]
fn round_trip_preserves_all_color_fields() {
    // Construct a variant with many color fields set to unique values
    let mut v = ThemeVariant::default();

    // Defaults core colors
    v.defaults.accent_color = Some(Rgba::rgb(1, 0, 0));
    v.defaults.background_color = Some(Rgba::rgb(2, 0, 0));
    v.defaults.text_color = Some(Rgba::rgb(3, 0, 0));
    v.defaults.surface_color = Some(Rgba::rgb(4, 0, 0));
    v.defaults.border.color = Some(Rgba::rgb(5, 0, 0));
    v.defaults.muted_color = Some(Rgba::rgb(6, 0, 0));
    v.defaults.shadow_color = Some(Rgba::rgb(7, 0, 0));

    // Defaults status colors
    v.defaults.danger_color = Some(Rgba::rgb(12, 0, 0));
    v.defaults.danger_text_color = Some(Rgba::rgb(13, 0, 0));
    v.defaults.warning_color = Some(Rgba::rgb(14, 0, 0));
    v.defaults.warning_text_color = Some(Rgba::rgb(15, 0, 0));
    v.defaults.success_color = Some(Rgba::rgb(16, 0, 0));
    v.defaults.success_text_color = Some(Rgba::rgb(17, 0, 0));
    v.defaults.info_color = Some(Rgba::rgb(18, 0, 0));
    v.defaults.info_text_color = Some(Rgba::rgb(19, 0, 0));

    // Defaults interactive colors
    v.defaults.selection_background = Some(Rgba::rgb(20, 0, 0));
    v.defaults.selection_text_color = Some(Rgba::rgb(21, 0, 0));
    v.defaults.link_color = Some(Rgba::rgb(22, 0, 0));
    v.defaults.focus_ring_color = Some(Rgba::rgb(23, 0, 0));
    v.defaults.disabled_text_color = Some(Rgba::rgb(34, 0, 0));

    // Per-widget: button
    v.button.primary_background = Some(Rgba::rgb(8, 0, 0));
    v.button.primary_text_color = Some(Rgba::rgb(9, 0, 0));
    v.button.background_color = Some(Rgba::rgb(10, 0, 0));
    v.button.font.get_or_insert_default().color = Some(Rgba::rgb(11, 0, 0));

    // Per-widget: sidebar
    v.sidebar.background_color = Some(Rgba::rgb(24, 0, 0));
    v.sidebar.font.get_or_insert_default().color = Some(Rgba::rgb(25, 0, 0));

    // Per-widget: tooltip
    v.tooltip.background_color = Some(Rgba::rgb(26, 0, 0));
    v.tooltip.font.get_or_insert_default().color = Some(Rgba::rgb(27, 0, 0));

    // Per-widget: popover
    v.popover.background_color = Some(Rgba::rgb(28, 0, 0));
    v.popover.font.get_or_insert_default().color = Some(Rgba::rgb(29, 0, 0));

    // Per-widget: input
    v.input.background_color = Some(Rgba::rgb(30, 0, 0));
    v.input.font.get_or_insert_default().color = Some(Rgba::rgb(31, 0, 0));

    // Per-widget: list
    v.list.alternate_row_background = Some(Rgba::rgb(32, 0, 0));

    // Per-widget: separator
    v.separator.line_color = Some(Rgba::rgb(33, 0, 0));

    let mut theme = ThemeSpec::new("All Colors");
    theme.light = Some(v);

    let toml_str = toml::to_string_pretty(&theme).unwrap();
    let de: ThemeSpec = toml::from_str(&toml_str).unwrap();
    let de_v = de.light.as_ref().unwrap();
    let orig_v = theme.light.as_ref().unwrap();

    // Defaults colors
    assert_eq!(de_v.defaults.accent_color, orig_v.defaults.accent_color);
    assert_eq!(
        de_v.defaults.background_color,
        orig_v.defaults.background_color
    );
    assert_eq!(de_v.defaults.text_color, orig_v.defaults.text_color);
    assert_eq!(de_v.defaults.surface_color, orig_v.defaults.surface_color);
    assert_eq!(de_v.defaults.border, orig_v.defaults.border);
    assert_eq!(de_v.defaults.muted_color, orig_v.defaults.muted_color);
    assert_eq!(de_v.defaults.shadow_color, orig_v.defaults.shadow_color);

    assert_eq!(de_v.defaults.danger_color, orig_v.defaults.danger_color);
    assert_eq!(
        de_v.defaults.danger_text_color,
        orig_v.defaults.danger_text_color
    );
    assert_eq!(de_v.defaults.warning_color, orig_v.defaults.warning_color);
    assert_eq!(
        de_v.defaults.warning_text_color,
        orig_v.defaults.warning_text_color
    );
    assert_eq!(de_v.defaults.success_color, orig_v.defaults.success_color);
    assert_eq!(
        de_v.defaults.success_text_color,
        orig_v.defaults.success_text_color
    );
    assert_eq!(de_v.defaults.info_color, orig_v.defaults.info_color);
    assert_eq!(
        de_v.defaults.info_text_color,
        orig_v.defaults.info_text_color
    );

    assert_eq!(
        de_v.defaults.selection_background,
        orig_v.defaults.selection_background
    );
    assert_eq!(
        de_v.defaults.selection_text_color,
        orig_v.defaults.selection_text_color
    );
    assert_eq!(de_v.defaults.link_color, orig_v.defaults.link_color);
    assert_eq!(
        de_v.defaults.focus_ring_color,
        orig_v.defaults.focus_ring_color
    );
    assert_eq!(
        de_v.defaults.disabled_text_color,
        orig_v.defaults.disabled_text_color
    );

    // Per-widget colors
    assert_eq!(
        de_v.button.primary_background,
        orig_v.button.primary_background
    );
    assert_eq!(
        de_v.button.primary_text_color,
        orig_v.button.primary_text_color
    );
    assert_eq!(de_v.button.background_color, orig_v.button.background_color);
    assert_eq!(
        de_v.button.font.as_ref().and_then(|f| f.color),
        orig_v.button.font.as_ref().and_then(|f| f.color)
    );

    assert_eq!(
        de_v.sidebar.background_color,
        orig_v.sidebar.background_color
    );
    assert_eq!(
        de_v.sidebar.font.as_ref().and_then(|f| f.color),
        orig_v.sidebar.font.as_ref().and_then(|f| f.color)
    );

    assert_eq!(
        de_v.tooltip.background_color,
        orig_v.tooltip.background_color
    );
    assert_eq!(
        de_v.tooltip.font.as_ref().and_then(|f| f.color),
        orig_v.tooltip.font.as_ref().and_then(|f| f.color)
    );

    assert_eq!(
        de_v.popover.background_color,
        orig_v.popover.background_color
    );
    assert_eq!(
        de_v.popover.font.as_ref().and_then(|f| f.color),
        orig_v.popover.font.as_ref().and_then(|f| f.color)
    );

    assert_eq!(de_v.input.background_color, orig_v.input.background_color);
    assert_eq!(
        de_v.input.font.as_ref().and_then(|f| f.color),
        orig_v.input.font.as_ref().and_then(|f| f.color)
    );

    assert_eq!(
        de_v.list.alternate_row_background,
        orig_v.list.alternate_row_background
    );
    assert_eq!(de_v.separator.line_color, orig_v.separator.line_color);
}

// ---------------------------------------------------------------------------
// Sparse deserialization tests
// ---------------------------------------------------------------------------

#[test]
fn sparse_toml_deserializes() {
    let toml_str = r##"
name = "Minimal"

[light.defaults]
accent_color = "#3daee9"
"##;

    let theme: ThemeSpec = toml::from_str(toml_str).unwrap();

    assert_eq!(theme.name, "Minimal");
    assert_eq!(
        theme.light.as_ref().unwrap().defaults.accent_color,
        Some(Rgba::rgb(61, 174, 233))
    );
    // All other fields are None/default
    assert!(
        theme
            .light
            .as_ref()
            .unwrap()
            .defaults
            .background_color
            .is_none()
    );
    assert!(
        theme
            .light
            .as_ref()
            .unwrap()
            .defaults
            .danger_color
            .is_none()
    );
    assert!(theme.light.as_ref().unwrap().defaults.font.family.is_none());
    assert!(theme.dark.is_none());
}

#[test]
fn very_sparse_toml_name_only() {
    let toml_str = r#"name = "Empty""#;

    let theme: ThemeSpec = toml::from_str(toml_str).unwrap();

    assert_eq!(theme.name, "Empty");
    assert!(theme.light.is_none());
    assert!(theme.dark.is_none());
}

// ---------------------------------------------------------------------------
// Serialization skip tests
// ---------------------------------------------------------------------------

#[test]
fn serialization_skips_none_fields() {
    let mut theme = ThemeSpec::new("Skip Test");
    let mut light = ThemeVariant::default();
    light.defaults.accent_color = Some(Rgba::rgb(61, 174, 233));
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
    let mut theme = ThemeSpec::new("Readable");
    let mut light = ThemeVariant::default();
    light.defaults.accent_color = Some(Rgba::rgb(61, 174, 233));
    light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
    light.defaults.font.family = Some("Noto Sans".into());
    light.defaults.border.corner_radius = Some(4.0);
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
accent_color = "#3daee9"
shadow_color = "#00000040"
"##;

    let theme: ThemeSpec = toml::from_str(toml_str).unwrap();
    let light = theme.light.as_ref().unwrap();

    // accent: #3daee9 -> r=61, g=174, b=233, a=255 (no alpha => opaque)
    assert_eq!(light.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));

    // shadow: #00000040 -> r=0, g=0, b=0, a=0x40=64
    assert_eq!(light.defaults.shadow_color, Some(Rgba::rgba(0, 0, 0, 64)));

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
