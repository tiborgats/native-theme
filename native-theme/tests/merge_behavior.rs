//! Integration tests for merge behavior across all structs and trait
//! compile-time assertions.
//!
//! These tests exercise merge() overlay semantics, is_empty() on all structs,
//! and verify that all public types implement Send + Sync + Default + Clone + Debug.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::*;

// ---------------------------------------------------------------------------
// merge() overlay semantics
// ---------------------------------------------------------------------------

#[test]
fn merge_overlay_replaces_none_with_some() {
    let mut base = ThemeVariant::default();
    base.defaults.background_color = Some(Rgba::rgb(255, 255, 255));

    let mut overlay = ThemeVariant::default();
    overlay.defaults.accent_color = Some(Rgba::rgb(61, 174, 233));
    // overlay does NOT set background

    base.merge(&overlay);

    // accent: was None, replaced by Some from overlay
    assert_eq!(
        base.defaults.accent_color,
        Some(Rgba::rgb(61, 174, 233)),
        "overlay should replace None with Some"
    );
    // background: base had Some, overlay had None => preserved
    assert_eq!(
        base.defaults.background_color,
        Some(Rgba::rgb(255, 255, 255)),
        "base value should be preserved when overlay is None"
    );
}

#[test]
fn merge_overlay_replaces_some_with_some() {
    let mut base = ThemeVariant::default();
    base.defaults.accent_color = Some(Rgba::rgb(255, 0, 0)); // red

    let mut overlay = ThemeVariant::default();
    overlay.defaults.accent_color = Some(Rgba::rgb(0, 0, 255)); // blue

    base.merge(&overlay);

    assert_eq!(
        base.defaults.accent_color,
        Some(Rgba::rgb(0, 0, 255)),
        "overlay Some should replace base Some"
    );
}

#[test]
fn merge_preserves_base_when_overlay_empty() {
    let mut base = ThemeVariant::default();
    base.defaults.accent_color = Some(Rgba::rgb(61, 174, 233));
    base.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
    base.defaults.font.family = Some("Noto Sans".into());
    base.defaults.border.corner_radius = Some(4.0);
    // REMOVED(spacing): base.defaults.spacing.m = Some(12.0);

    let overlay = ThemeVariant::default(); // completely empty

    base.merge(&overlay);

    assert_eq!(base.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    assert_eq!(
        base.defaults.background_color,
        Some(Rgba::rgb(255, 255, 255))
    );
    assert_eq!(base.defaults.font.family.as_deref(), Some("Noto Sans"));
    assert_eq!(base.defaults.border.corner_radius, Some(4.0));
    // REMOVED(spacing): assert_eq!(base.defaults.spacing.m, Some(12.0));
}

#[test]
fn merge_native_theme_light_dark() {
    let mut base = ThemeSpec::new("Base");
    let mut base_light = ThemeVariant::default();
    base_light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
    base.light = Some(base_light);
    // base has no dark

    let mut overlay = ThemeSpec::new("Overlay");
    let mut overlay_dark = ThemeVariant::default();
    overlay_dark.defaults.background_color = Some(Rgba::rgb(30, 30, 30));
    overlay.dark = Some(overlay_dark);
    // overlay has no light

    base.merge(&overlay);

    // Name stays as base name
    assert_eq!(base.name, "Base");

    // Light from base is preserved
    assert!(base.light.is_some());
    assert_eq!(
        base.light.as_ref().unwrap().defaults.background_color,
        Some(Rgba::rgb(255, 255, 255))
    );

    // Dark from overlay was adopted
    assert!(base.dark.is_some());
    assert_eq!(
        base.dark.as_ref().unwrap().defaults.background_color,
        Some(Rgba::rgb(30, 30, 30))
    );
}

#[test]
fn merge_native_theme_deep_merge_variants() {
    let mut base = ThemeSpec::new("Base");
    let mut base_light = ThemeVariant::default();
    base_light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
    base.light = Some(base_light);

    let mut overlay = ThemeSpec::new("Overlay");
    let mut overlay_light = ThemeVariant::default();
    overlay_light.defaults.accent_color = Some(Rgba::rgb(61, 174, 233));
    overlay.light = Some(overlay_light);

    base.merge(&overlay);

    let light = base.light.as_ref().unwrap();
    // background from base
    assert_eq!(
        light.defaults.background_color,
        Some(Rgba::rgb(255, 255, 255))
    );
    // accent from overlay
    assert_eq!(light.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
}

#[test]
fn merge_fonts_defaults_spacing() {
    // FontSpec
    let mut base_font = FontSpec {
        size: Some(12.0),
        ..Default::default()
    };
    let overlay_font = FontSpec {
        family: Some("Inter".into()),
        ..Default::default()
    };
    base_font.merge(&overlay_font);
    assert_eq!(
        base_font.family.as_deref(),
        Some("Inter"),
        "overlay family replaces"
    );
    assert_eq!(base_font.size, Some(12.0), "base size preserved");

    // ThemeDefaults (geometry fields via border sub-struct)
    let mut base_defaults = ThemeDefaults {
        border: native_theme::BorderSpec {
            line_width: Some(1.0),
            ..Default::default()
        },
        ..Default::default()
    };
    let overlay_defaults = ThemeDefaults {
        border: native_theme::BorderSpec {
            corner_radius: Some(8.0),
            ..Default::default()
        },
        ..Default::default()
    };
    base_defaults.merge(&overlay_defaults);
    assert_eq!(
        base_defaults.border.corner_radius,
        Some(8.0),
        "overlay corner_radius replaces"
    );
    assert_eq!(
        base_defaults.border.line_width,
        Some(1.0),
        "base line_width preserved"
    );

    // REMOVED: ThemeSpacing deleted in Plan 01
}

#[test]
fn merge_chained_multiple_overlays() {
    let mut base = ThemeVariant::default();
    base.defaults.background_color = Some(Rgba::rgb(255, 255, 255));

    let mut overlay1 = ThemeVariant::default();
    overlay1.defaults.accent_color = Some(Rgba::rgb(255, 0, 0)); // red accent
    overlay1.defaults.font.family = Some("Noto Sans".into());

    let mut overlay2 = ThemeVariant::default();
    overlay2.defaults.accent_color = Some(Rgba::rgb(0, 0, 255)); // blue accent (overwrites)
    overlay2.defaults.border.corner_radius = Some(8.0);

    base.merge(&overlay1);
    base.merge(&overlay2);

    // background from base (neither overlay set it)
    assert_eq!(
        base.defaults.background_color,
        Some(Rgba::rgb(255, 255, 255))
    );
    // accent: overlay2 wins (last-wins)
    assert_eq!(base.defaults.accent_color, Some(Rgba::rgb(0, 0, 255)));
    // font from overlay1 (overlay2 didn't set it)
    assert_eq!(base.defaults.font.family.as_deref(), Some("Noto Sans"));
    // geometry from overlay2
    assert_eq!(base.defaults.border.corner_radius, Some(8.0));
}

// ---------------------------------------------------------------------------
// is_empty() on all structs
// ---------------------------------------------------------------------------

#[test]
fn is_empty_all_structs() {
    // Core structs
    assert!(ThemeSpec::default().is_empty());
    assert!(ThemeVariant::default().is_empty());
    assert!(ThemeDefaults::default().is_empty());
    assert!(FontSpec::default().is_empty());
    // REMOVED(spacing): assert!(ThemeSpacing_DELETED::default().is_empty());
    assert!(IconSizes::default().is_empty());
    assert!(TextScaleEntry::default().is_empty());
    assert!(TextScale::default().is_empty());

    // All 25 widget theme structs
    assert!(WindowTheme::default().is_empty());
    assert!(ButtonTheme::default().is_empty());
    assert!(InputTheme::default().is_empty());
    assert!(CheckboxTheme::default().is_empty());
    assert!(MenuTheme::default().is_empty());
    assert!(TooltipTheme::default().is_empty());
    assert!(ScrollbarTheme::default().is_empty());
    assert!(SliderTheme::default().is_empty());
    assert!(ProgressBarTheme::default().is_empty());
    assert!(TabTheme::default().is_empty());
    assert!(SidebarTheme::default().is_empty());
    assert!(ToolbarTheme::default().is_empty());
    assert!(StatusBarTheme::default().is_empty());
    assert!(ListTheme::default().is_empty());
    assert!(PopoverTheme::default().is_empty());
    assert!(SplitterTheme::default().is_empty());
    assert!(SeparatorTheme::default().is_empty());
    assert!(SwitchTheme::default().is_empty());
    assert!(DialogTheme::default().is_empty());
    assert!(SpinnerTheme::default().is_empty());
    assert!(ComboBoxTheme::default().is_empty());
    assert!(SegmentedControlTheme::default().is_empty());
    assert!(CardTheme::default().is_empty());
    assert!(ExpanderTheme::default().is_empty());
    assert!(LinkTheme::default().is_empty());
}

#[test]
fn is_empty_false_after_setting_field() {
    // Core structs
    let spec = ThemeSpec {
        light: Some(ThemeVariant::default()),
        ..Default::default()
    };
    assert!(!spec.is_empty());

    let mut variant = ThemeVariant::default();
    variant.defaults.background_color = Some(Rgba::rgb(0, 0, 0));
    assert!(!variant.is_empty());

    let defaults = ThemeDefaults {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!defaults.is_empty());

    let font = FontSpec {
        family: Some("Inter".into()),
        ..Default::default()
    };
    assert!(!font.is_empty());

    // REMOVED: ThemeSpacing deleted in Plan 01

    let icons = IconSizes {
        toolbar: Some(24.0),
        ..Default::default()
    };
    assert!(!icons.is_empty());

    let tse = TextScaleEntry {
        size: Some(14.0),
        ..Default::default()
    };
    assert!(!tse.is_empty());

    let ts = TextScale {
        caption: Some(TextScaleEntry::default()),
        ..Default::default()
    };
    assert!(!ts.is_empty());

    // All 25 widget theme structs
    let w = WindowTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = ButtonTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = InputTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = CheckboxTheme {
        checked_background: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = MenuTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = TooltipTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = ScrollbarTheme {
        track_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = SliderTheme {
        fill_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = ProgressBarTheme {
        fill_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = TabTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = SidebarTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = ToolbarTheme {
        bar_height: Some(48.0),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = StatusBarTheme {
        font: Some(FontSpec {
            size: Some(12.0),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = ListTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = PopoverTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = SplitterTheme {
        divider_width: Some(4.0),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = SeparatorTheme {
        line_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = SwitchTheme {
        checked_background: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = DialogTheme {
        min_width: Some(400.0),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = SpinnerTheme {
        fill_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = ComboBoxTheme {
        min_height: Some(32.0),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = SegmentedControlTheme {
        segment_height: Some(28.0),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = CardTheme {
        background_color: Some(Rgba::rgb(0, 0, 0)),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = ExpanderTheme {
        header_height: Some(40.0),
        ..Default::default()
    };
    assert!(!w.is_empty());

    let w = LinkTheme {
        visited_text_color: Some(Rgba::rgb(0, 0, 255)),
        ..Default::default()
    };
    assert!(!w.is_empty());
}

// ---------------------------------------------------------------------------
// Compile-time trait assertions
// ---------------------------------------------------------------------------

#[test]
fn trait_assertions_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<Rgba>();
    assert_send_sync::<ThemeSpec>();
    assert_send_sync::<ThemeVariant>();
    assert_send_sync::<ThemeDefaults>();
    assert_send_sync::<FontSpec>();
    // REMOVED(spacing): assert_send_sync::<ThemeSpacing_DELETED>();
    assert_send_sync::<Error>();
}

#[test]
fn trait_assertions_default_clone_debug() {
    fn assert_default_clone_debug<T: Default + Clone + std::fmt::Debug>() {}

    assert_default_clone_debug::<Rgba>();
    assert_default_clone_debug::<ThemeSpec>();
    assert_default_clone_debug::<ThemeVariant>();
    assert_default_clone_debug::<ThemeDefaults>();
    assert_default_clone_debug::<FontSpec>();
    // REMOVED(spacing): assert_default_clone_debug::<ThemeSpacing_DELETED>();

    // Error is Debug but not Default/Clone -- verify separately
    fn assert_debug<T: std::fmt::Debug>() {}
    assert_debug::<Error>();
}

// ---------------------------------------------------------------------------
// Realistic theme layering scenario
// ---------------------------------------------------------------------------

#[test]
fn realistic_theme_layering_scenario() {
    // Base preset: Breeze Light with many fields set
    let mut base = ThemeSpec::new("Breeze Light");
    let mut light = ThemeVariant::default();

    // Populate base default colors
    light.defaults.accent_color = Some(Rgba::rgb(61, 174, 233));
    light.defaults.background_color = Some(Rgba::rgb(252, 252, 252));
    light.defaults.text_color = Some(Rgba::rgb(35, 38, 41));
    light.defaults.surface_color = Some(Rgba::rgb(239, 240, 241));
    light.defaults.border.color = Some(Rgba::rgb(188, 190, 191));
    light.defaults.danger_color = Some(Rgba::rgb(218, 68, 83));
    light.defaults.success_color = Some(Rgba::rgb(39, 174, 96));
    light.defaults.selection_background = Some(Rgba::rgb(61, 174, 233));
    light.defaults.link_color = Some(Rgba::rgb(41, 128, 185));

    // Populate per-widget colors
    light.sidebar.background_color = Some(Rgba::rgb(227, 229, 231));
    light.button.background_color = Some(Rgba::rgb(239, 240, 241));

    // Populate base fonts
    light.defaults.font.family = Some("Noto Sans".into());
    light.defaults.font.size = Some(10.0);
    light.defaults.mono_font.family = Some("Hack".into());
    light.defaults.mono_font.size = Some(10.0);

    // Populate base geometry
    light.defaults.border.corner_radius = Some(4.0);
    light.defaults.border.line_width = Some(1.0);

    // Populate base spacing
    // REMOVED(spacing): light.defaults.spacing.s = Some(8.0);
    // REMOVED(spacing): light.defaults.spacing.m = Some(12.0);
    // REMOVED(spacing): light.defaults.spacing.l = Some(16.0);

    base.light = Some(light);

    // User override: just accent color and font family
    let mut user_override = ThemeSpec::new("User Override");
    let mut user_light = ThemeVariant::default();
    user_light.defaults.accent_color = Some(Rgba::rgb(156, 39, 176)); // purple accent
    user_light.defaults.font.family = Some("Inter".into()); // different font
    user_override.light = Some(user_light);

    // Apply user override on top of base
    base.merge(&user_override);

    // Verify: name stays as base
    assert_eq!(base.name, "Breeze Light");

    let result = base.light.as_ref().unwrap();

    // Accent changed to purple (from user override)
    assert_eq!(
        result.defaults.accent_color,
        Some(Rgba::rgb(156, 39, 176)),
        "accent should be overridden to purple"
    );

    // Font family changed (from user override)
    assert_eq!(
        result.defaults.font.family.as_deref(),
        Some("Inter"),
        "font family should be overridden to Inter"
    );

    // Everything else from base preserved
    assert_eq!(
        result.defaults.background_color,
        Some(Rgba::rgb(252, 252, 252))
    );
    assert_eq!(result.defaults.text_color, Some(Rgba::rgb(35, 38, 41)));
    assert_eq!(result.defaults.danger_color, Some(Rgba::rgb(218, 68, 83)));
    assert_eq!(result.defaults.link_color, Some(Rgba::rgb(41, 128, 185)));
    assert_eq!(result.defaults.font.size, Some(10.0));
    assert_eq!(result.defaults.mono_font.family.as_deref(), Some("Hack"));
    assert_eq!(result.defaults.border.corner_radius, Some(4.0));
    // REMOVED(spacing): assert_eq!(result.defaults.spacing.m, Some(12.0));

    // Serialize the merged result to TOML and verify it looks reasonable
    let toml_str = toml::to_string_pretty(&base).unwrap();
    assert!(toml_str.contains("Breeze Light"), "name should be in TOML");
    assert!(
        toml_str.contains("#9c27b0"),
        "purple accent hex should appear"
    );
    assert!(toml_str.contains("Inter"), "overridden font should appear");
    assert!(
        toml_str.contains("#fcfcfc"),
        "base background should appear"
    );
}
