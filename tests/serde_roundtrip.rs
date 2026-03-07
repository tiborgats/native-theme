//! Integration tests for TOML serialization round-trip, sparse deserialization,
//! and serialization skip behavior.
//!
//! These tests exercise the public API exactly as downstream consumers will use
//! it: `use native_theme::*` plus `toml` for (de)serialization.

use native_theme::*;

// ---------------------------------------------------------------------------
// Helper: build a fully populated ThemeVariant with distinct values
// ---------------------------------------------------------------------------

fn fully_populated_variant(offset: u8) -> ThemeVariant {
    let c = |r: u8, g: u8, b: u8| Rgba::rgb(r.wrapping_add(offset), g, b);

    let mut v = ThemeVariant::default();

    // core (7 fields)
    v.colors.core.accent = Some(c(61, 174, 233));
    v.colors.core.background = Some(c(255, 255, 255));
    v.colors.core.foreground = Some(c(35, 38, 41));
    v.colors.core.surface = Some(c(239, 240, 241));
    v.colors.core.border = Some(c(188, 190, 191));
    v.colors.core.muted = Some(c(127, 140, 141));
    v.colors.core.shadow = Some(Rgba::rgba(0u8.wrapping_add(offset), 0, 0, 64));

    // primary (2 fields)
    v.colors.primary.background = Some(c(61, 174, 233));
    v.colors.primary.foreground = Some(c(252, 252, 252));

    // secondary (2 fields)
    v.colors.secondary.background = Some(c(189, 195, 199));
    v.colors.secondary.foreground = Some(c(49, 54, 59));

    // status (8 fields)
    v.colors.status.danger = Some(c(218, 68, 83));
    v.colors.status.danger_foreground = Some(c(252, 252, 252));
    v.colors.status.warning = Some(c(246, 116, 0));
    v.colors.status.warning_foreground = Some(c(35, 38, 41));
    v.colors.status.success = Some(c(39, 174, 96));
    v.colors.status.success_foreground = Some(c(252, 252, 252));
    v.colors.status.info = Some(c(61, 174, 233));
    v.colors.status.info_foreground = Some(c(252, 252, 252));

    // interactive (4 fields)
    v.colors.interactive.selection = Some(c(61, 174, 233));
    v.colors.interactive.selection_foreground = Some(c(252, 252, 252));
    v.colors.interactive.link = Some(c(41, 128, 185));
    v.colors.interactive.focus_ring = Some(c(61, 174, 233));

    // panel (6 fields)
    v.colors.panel.sidebar = Some(c(227, 229, 231));
    v.colors.panel.sidebar_foreground = Some(c(35, 38, 41));
    v.colors.panel.tooltip = Some(c(35, 38, 41));
    v.colors.panel.tooltip_foreground = Some(c(252, 252, 252));
    v.colors.panel.popover = Some(c(255, 255, 255));
    v.colors.panel.popover_foreground = Some(c(35, 38, 41));

    // component (7 fields)
    v.colors.component.button = Some(c(239, 240, 241));
    v.colors.component.button_foreground = Some(c(35, 38, 41));
    v.colors.component.input = Some(c(252, 252, 252));
    v.colors.component.input_foreground = Some(c(35, 38, 41));
    v.colors.component.disabled = Some(c(189, 195, 199));
    v.colors.component.separator = Some(c(188, 190, 191));
    v.colors.component.alternate_row = Some(c(239, 240, 241));

    // fonts
    v.fonts.family = Some("Noto Sans".into());
    v.fonts.size = Some(10.0);
    v.fonts.mono_family = Some("Hack".into());
    v.fonts.mono_size = Some(10.0);

    // geometry
    v.geometry.radius = Some(4.0);
    v.geometry.frame_width = Some(1.0);
    v.geometry.disabled_opacity = Some(0.4);
    v.geometry.border_opacity = Some(0.6);
    v.geometry.scroll_width = Some(12.0);

    // spacing
    v.spacing.xxs = Some(2.0);
    v.spacing.xs = Some(4.0);
    v.spacing.s = Some(8.0);
    v.spacing.m = Some(12.0);
    v.spacing.l = Some(16.0);
    v.spacing.xl = Some(24.0);
    v.spacing.xxl = Some(32.0);

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

    // Colors
    assert_eq!(de_light.colors.core.accent, orig_light.colors.core.accent);
    assert_eq!(de_light.colors.core.background, orig_light.colors.core.background);
    assert_eq!(de_light.colors.core.foreground, orig_light.colors.core.foreground);
    assert_eq!(de_light.colors.primary.background, orig_light.colors.primary.background);
    assert_eq!(de_light.colors.secondary.background, orig_light.colors.secondary.background);
    assert_eq!(de_light.colors.status.danger, orig_light.colors.status.danger);
    assert_eq!(de_light.colors.status.success, orig_light.colors.status.success);
    assert_eq!(de_light.colors.interactive.selection, orig_light.colors.interactive.selection);
    assert_eq!(de_light.colors.interactive.link, orig_light.colors.interactive.link);
    assert_eq!(de_light.colors.panel.sidebar, orig_light.colors.panel.sidebar);
    assert_eq!(de_light.colors.panel.tooltip, orig_light.colors.panel.tooltip);
    assert_eq!(de_light.colors.component.button, orig_light.colors.component.button);
    assert_eq!(de_light.colors.component.input, orig_light.colors.component.input);

    // Fonts
    assert_eq!(de_light.fonts.family, orig_light.fonts.family);
    assert_eq!(de_light.fonts.size, orig_light.fonts.size);
    assert_eq!(de_light.fonts.mono_family, orig_light.fonts.mono_family);
    assert_eq!(de_light.fonts.mono_size, orig_light.fonts.mono_size);

    // Geometry
    assert_eq!(de_light.geometry.radius, orig_light.geometry.radius);
    assert_eq!(de_light.geometry.frame_width, orig_light.geometry.frame_width);

    // Spacing
    assert_eq!(de_light.spacing.m, orig_light.spacing.m);
    assert_eq!(de_light.spacing.l, orig_light.spacing.l);

    // Dark variant spot-checks
    let orig_dark = theme.dark.as_ref().unwrap();
    let de_dark = deserialized.dark.as_ref().unwrap();
    assert_eq!(de_dark.colors.core.accent, orig_dark.colors.core.accent);
    assert_eq!(de_dark.fonts.family, orig_dark.fonts.family);
    assert_eq!(de_dark.geometry.radius, orig_dark.geometry.radius);
    assert_eq!(de_dark.spacing.xxl, orig_dark.spacing.xxl);
}

#[test]
fn round_trip_preserves_all_36_color_fields() {
    // Construct a variant with ALL 36 color fields set to unique values
    let mut v = ThemeVariant::default();

    // Core (7)
    v.colors.core.accent = Some(Rgba::rgb(1, 0, 0));
    v.colors.core.background = Some(Rgba::rgb(2, 0, 0));
    v.colors.core.foreground = Some(Rgba::rgb(3, 0, 0));
    v.colors.core.surface = Some(Rgba::rgb(4, 0, 0));
    v.colors.core.border = Some(Rgba::rgb(5, 0, 0));
    v.colors.core.muted = Some(Rgba::rgb(6, 0, 0));
    v.colors.core.shadow = Some(Rgba::rgb(7, 0, 0));

    // Primary (2)
    v.colors.primary.background = Some(Rgba::rgb(8, 0, 0));
    v.colors.primary.foreground = Some(Rgba::rgb(9, 0, 0));

    // Secondary (2)
    v.colors.secondary.background = Some(Rgba::rgb(10, 0, 0));
    v.colors.secondary.foreground = Some(Rgba::rgb(11, 0, 0));

    // Status (8)
    v.colors.status.danger = Some(Rgba::rgb(12, 0, 0));
    v.colors.status.danger_foreground = Some(Rgba::rgb(13, 0, 0));
    v.colors.status.warning = Some(Rgba::rgb(14, 0, 0));
    v.colors.status.warning_foreground = Some(Rgba::rgb(15, 0, 0));
    v.colors.status.success = Some(Rgba::rgb(16, 0, 0));
    v.colors.status.success_foreground = Some(Rgba::rgb(17, 0, 0));
    v.colors.status.info = Some(Rgba::rgb(18, 0, 0));
    v.colors.status.info_foreground = Some(Rgba::rgb(19, 0, 0));

    // Interactive (4)
    v.colors.interactive.selection = Some(Rgba::rgb(20, 0, 0));
    v.colors.interactive.selection_foreground = Some(Rgba::rgb(21, 0, 0));
    v.colors.interactive.link = Some(Rgba::rgb(22, 0, 0));
    v.colors.interactive.focus_ring = Some(Rgba::rgb(23, 0, 0));

    // Panel (6)
    v.colors.panel.sidebar = Some(Rgba::rgb(24, 0, 0));
    v.colors.panel.sidebar_foreground = Some(Rgba::rgb(25, 0, 0));
    v.colors.panel.tooltip = Some(Rgba::rgb(26, 0, 0));
    v.colors.panel.tooltip_foreground = Some(Rgba::rgb(27, 0, 0));
    v.colors.panel.popover = Some(Rgba::rgb(28, 0, 0));
    v.colors.panel.popover_foreground = Some(Rgba::rgb(29, 0, 0));

    // Component (7)
    v.colors.component.button = Some(Rgba::rgb(30, 0, 0));
    v.colors.component.button_foreground = Some(Rgba::rgb(31, 0, 0));
    v.colors.component.input = Some(Rgba::rgb(32, 0, 0));
    v.colors.component.input_foreground = Some(Rgba::rgb(33, 0, 0));
    v.colors.component.disabled = Some(Rgba::rgb(34, 0, 0));
    v.colors.component.separator = Some(Rgba::rgb(35, 0, 0));
    v.colors.component.alternate_row = Some(Rgba::rgb(36, 0, 0));

    let mut theme = NativeTheme::new("36 Colors");
    theme.light = Some(v);

    let toml_str = toml::to_string_pretty(&theme).unwrap();
    let de: NativeTheme = toml::from_str(&toml_str).unwrap();
    let de_v = de.light.as_ref().unwrap();
    let orig_v = theme.light.as_ref().unwrap();

    // Verify every single one of the 36 color fields
    assert_eq!(de_v.colors.core.accent, orig_v.colors.core.accent);
    assert_eq!(de_v.colors.core.background, orig_v.colors.core.background);
    assert_eq!(de_v.colors.core.foreground, orig_v.colors.core.foreground);
    assert_eq!(de_v.colors.core.surface, orig_v.colors.core.surface);
    assert_eq!(de_v.colors.core.border, orig_v.colors.core.border);
    assert_eq!(de_v.colors.core.muted, orig_v.colors.core.muted);
    assert_eq!(de_v.colors.core.shadow, orig_v.colors.core.shadow);

    assert_eq!(de_v.colors.primary.background, orig_v.colors.primary.background);
    assert_eq!(de_v.colors.primary.foreground, orig_v.colors.primary.foreground);

    assert_eq!(de_v.colors.secondary.background, orig_v.colors.secondary.background);
    assert_eq!(de_v.colors.secondary.foreground, orig_v.colors.secondary.foreground);

    assert_eq!(de_v.colors.status.danger, orig_v.colors.status.danger);
    assert_eq!(de_v.colors.status.danger_foreground, orig_v.colors.status.danger_foreground);
    assert_eq!(de_v.colors.status.warning, orig_v.colors.status.warning);
    assert_eq!(de_v.colors.status.warning_foreground, orig_v.colors.status.warning_foreground);
    assert_eq!(de_v.colors.status.success, orig_v.colors.status.success);
    assert_eq!(de_v.colors.status.success_foreground, orig_v.colors.status.success_foreground);
    assert_eq!(de_v.colors.status.info, orig_v.colors.status.info);
    assert_eq!(de_v.colors.status.info_foreground, orig_v.colors.status.info_foreground);

    assert_eq!(de_v.colors.interactive.selection, orig_v.colors.interactive.selection);
    assert_eq!(de_v.colors.interactive.selection_foreground, orig_v.colors.interactive.selection_foreground);
    assert_eq!(de_v.colors.interactive.link, orig_v.colors.interactive.link);
    assert_eq!(de_v.colors.interactive.focus_ring, orig_v.colors.interactive.focus_ring);

    assert_eq!(de_v.colors.panel.sidebar, orig_v.colors.panel.sidebar);
    assert_eq!(de_v.colors.panel.sidebar_foreground, orig_v.colors.panel.sidebar_foreground);
    assert_eq!(de_v.colors.panel.tooltip, orig_v.colors.panel.tooltip);
    assert_eq!(de_v.colors.panel.tooltip_foreground, orig_v.colors.panel.tooltip_foreground);
    assert_eq!(de_v.colors.panel.popover, orig_v.colors.panel.popover);
    assert_eq!(de_v.colors.panel.popover_foreground, orig_v.colors.panel.popover_foreground);

    assert_eq!(de_v.colors.component.button, orig_v.colors.component.button);
    assert_eq!(de_v.colors.component.button_foreground, orig_v.colors.component.button_foreground);
    assert_eq!(de_v.colors.component.input, orig_v.colors.component.input);
    assert_eq!(de_v.colors.component.input_foreground, orig_v.colors.component.input_foreground);
    assert_eq!(de_v.colors.component.disabled, orig_v.colors.component.disabled);
    assert_eq!(de_v.colors.component.separator, orig_v.colors.component.separator);
    assert_eq!(de_v.colors.component.alternate_row, orig_v.colors.component.alternate_row);
}

// ---------------------------------------------------------------------------
// Sparse deserialization tests
// ---------------------------------------------------------------------------

#[test]
fn sparse_toml_deserializes() {
    let toml_str = r##"
name = "Minimal"

[light.colors.core]
accent = "#3daee9"
"##;

    let theme: NativeTheme = toml::from_str(toml_str).unwrap();

    assert_eq!(theme.name, "Minimal");
    assert_eq!(
        theme.light.as_ref().unwrap().colors.core.accent,
        Some(Rgba::rgb(61, 174, 233))
    );
    // All other fields are None/default
    assert!(theme.light.as_ref().unwrap().colors.core.background.is_none());
    assert!(theme.light.as_ref().unwrap().colors.status.danger.is_none());
    assert!(theme.light.as_ref().unwrap().fonts.family.is_none());
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
    light.colors.core.accent = Some(Rgba::rgb(61, 174, 233));
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
        !toml_str.contains("[light.colors.status]"),
        "TOML should NOT contain '[light.colors.status]' (empty)"
    );
    assert!(
        !toml_str.contains("[light.fonts]"),
        "TOML should NOT contain '[light.fonts]' (empty)"
    );
    assert!(
        !toml_str.contains("[light.geometry]"),
        "TOML should NOT contain '[light.geometry]' (empty)"
    );
    assert!(
        !toml_str.contains("[light.spacing]"),
        "TOML should NOT contain '[light.spacing]' (empty)"
    );
}

// ---------------------------------------------------------------------------
// TOML structure readability
// ---------------------------------------------------------------------------

#[test]
fn toml_structure_is_human_readable() {
    let mut theme = NativeTheme::new("Readable");
    let mut light = ThemeVariant::default();
    light.colors.core.accent = Some(Rgba::rgb(61, 174, 233));
    light.colors.core.background = Some(Rgba::rgb(255, 255, 255));
    light.fonts.family = Some("Noto Sans".into());
    light.geometry.radius = Some(4.0);
    theme.light = Some(light);

    let toml_str = toml::to_string_pretty(&theme).unwrap();

    // Print for manual inspection during development
    println!("--- Human-readable TOML ---\n{toml_str}---");

    // Verify section headers
    assert!(
        toml_str.contains("[light.colors.core]"),
        "expected [light.colors.core] section header"
    );
    assert!(
        toml_str.contains("[light.fonts]"),
        "expected [light.fonts] section header"
    );
    assert!(
        toml_str.contains("[light.geometry]"),
        "expected [light.geometry] section header"
    );

    // Fields appear under correct sections (accent under colors.core, not elsewhere)
    let core_section_start = toml_str.find("[light.colors.core]").unwrap();
    let accent_pos = toml_str.find("accent").unwrap();
    assert!(
        accent_pos > core_section_start,
        "accent should appear after [light.colors.core]"
    );
}

// ---------------------------------------------------------------------------
// Rgba hex in TOML
// ---------------------------------------------------------------------------

#[test]
fn rgba_hex_in_toml() {
    let toml_str = r##"
name = "Hex Test"

[light.colors.core]
accent = "#3daee9"
shadow = "#00000040"
"##;

    let theme: NativeTheme = toml::from_str(toml_str).unwrap();
    let light = theme.light.as_ref().unwrap();

    // accent: #3daee9 -> r=61, g=174, b=233, a=255 (no alpha => opaque)
    assert_eq!(
        light.colors.core.accent,
        Some(Rgba::rgb(61, 174, 233))
    );

    // shadow: #00000040 -> r=0, g=0, b=0, a=0x40=64
    assert_eq!(
        light.colors.core.shadow,
        Some(Rgba::rgba(0, 0, 0, 64))
    );

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
