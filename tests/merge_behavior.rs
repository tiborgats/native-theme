//! Integration tests for merge behavior across all structs and trait
//! compile-time assertions.
//!
//! These tests exercise merge() overlay semantics, is_empty() on all structs,
//! and verify that all public types implement Send + Sync + Default + Clone + Debug.

use native_theme::*;

// ---------------------------------------------------------------------------
// merge() overlay semantics
// ---------------------------------------------------------------------------

#[test]
fn merge_overlay_replaces_none_with_some() {
    let mut base = ThemeVariant::default();
    base.colors.core.background = Some(Rgba::rgb(255, 255, 255));

    let mut overlay = ThemeVariant::default();
    overlay.colors.core.accent = Some(Rgba::rgb(61, 174, 233));
    // overlay does NOT set background

    base.merge(&overlay);

    // accent: was None, replaced by Some from overlay
    assert_eq!(
        base.colors.core.accent,
        Some(Rgba::rgb(61, 174, 233)),
        "overlay should replace None with Some"
    );
    // background: base had Some, overlay had None => preserved
    assert_eq!(
        base.colors.core.background,
        Some(Rgba::rgb(255, 255, 255)),
        "base value should be preserved when overlay is None"
    );
}

#[test]
fn merge_overlay_replaces_some_with_some() {
    let mut base = ThemeVariant::default();
    base.colors.core.accent = Some(Rgba::rgb(255, 0, 0)); // red

    let mut overlay = ThemeVariant::default();
    overlay.colors.core.accent = Some(Rgba::rgb(0, 0, 255)); // blue

    base.merge(&overlay);

    assert_eq!(
        base.colors.core.accent,
        Some(Rgba::rgb(0, 0, 255)),
        "overlay Some should replace base Some"
    );
}

#[test]
fn merge_preserves_base_when_overlay_empty() {
    let mut base = ThemeVariant::default();
    base.colors.core.accent = Some(Rgba::rgb(61, 174, 233));
    base.colors.core.background = Some(Rgba::rgb(255, 255, 255));
    base.fonts.family = Some("Noto Sans".into());
    base.geometry.radius = Some(4.0);
    base.spacing.m = Some(12.0);

    let overlay = ThemeVariant::default(); // completely empty

    base.merge(&overlay);

    assert_eq!(base.colors.core.accent, Some(Rgba::rgb(61, 174, 233)));
    assert_eq!(base.colors.core.background, Some(Rgba::rgb(255, 255, 255)));
    assert_eq!(base.fonts.family.as_deref(), Some("Noto Sans"));
    assert_eq!(base.geometry.radius, Some(4.0));
    assert_eq!(base.spacing.m, Some(12.0));
}

#[test]
fn merge_native_theme_light_dark() {
    let mut base = NativeTheme::new("Base");
    let mut base_light = ThemeVariant::default();
    base_light.colors.core.background = Some(Rgba::rgb(255, 255, 255));
    base.light = Some(base_light);
    // base has no dark

    let mut overlay = NativeTheme::new("Overlay");
    let mut overlay_dark = ThemeVariant::default();
    overlay_dark.colors.core.background = Some(Rgba::rgb(30, 30, 30));
    overlay.dark = Some(overlay_dark);
    // overlay has no light

    base.merge(&overlay);

    // Name stays as base name
    assert_eq!(base.name, "Base");

    // Light from base is preserved
    assert!(base.light.is_some());
    assert_eq!(
        base.light.as_ref().unwrap().colors.core.background,
        Some(Rgba::rgb(255, 255, 255))
    );

    // Dark from overlay was adopted
    assert!(base.dark.is_some());
    assert_eq!(
        base.dark.as_ref().unwrap().colors.core.background,
        Some(Rgba::rgb(30, 30, 30))
    );
}

#[test]
fn merge_native_theme_deep_merge_variants() {
    let mut base = NativeTheme::new("Base");
    let mut base_light = ThemeVariant::default();
    base_light.colors.core.background = Some(Rgba::rgb(255, 255, 255));
    base.light = Some(base_light);

    let mut overlay = NativeTheme::new("Overlay");
    let mut overlay_light = ThemeVariant::default();
    overlay_light.colors.core.accent = Some(Rgba::rgb(61, 174, 233));
    overlay.light = Some(overlay_light);

    base.merge(&overlay);

    let light = base.light.as_ref().unwrap();
    // background from base
    assert_eq!(light.colors.core.background, Some(Rgba::rgb(255, 255, 255)));
    // accent from overlay
    assert_eq!(light.colors.core.accent, Some(Rgba::rgb(61, 174, 233)));
}

#[test]
fn merge_fonts_geometry_spacing() {
    // ThemeFonts
    let mut base_fonts = ThemeFonts::default();
    base_fonts.size = Some(12.0);
    let mut overlay_fonts = ThemeFonts::default();
    overlay_fonts.family = Some("Inter".into());
    base_fonts.merge(&overlay_fonts);
    assert_eq!(base_fonts.family.as_deref(), Some("Inter"), "overlay family replaces");
    assert_eq!(base_fonts.size, Some(12.0), "base size preserved");

    // ThemeGeometry
    let mut base_geom = ThemeGeometry::default();
    base_geom.frame_width = Some(1.0);
    let mut overlay_geom = ThemeGeometry::default();
    overlay_geom.radius = Some(8.0);
    base_geom.merge(&overlay_geom);
    assert_eq!(base_geom.radius, Some(8.0), "overlay radius replaces");
    assert_eq!(base_geom.frame_width, Some(1.0), "base frame_width preserved");

    // ThemeSpacing
    let mut base_spacing = ThemeSpacing::default();
    base_spacing.l = Some(16.0);
    let mut overlay_spacing = ThemeSpacing::default();
    overlay_spacing.m = Some(12.0);
    base_spacing.merge(&overlay_spacing);
    assert_eq!(base_spacing.m, Some(12.0), "overlay m replaces");
    assert_eq!(base_spacing.l, Some(16.0), "base l preserved");
}

#[test]
fn merge_chained_multiple_overlays() {
    let mut base = ThemeVariant::default();
    base.colors.core.background = Some(Rgba::rgb(255, 255, 255));

    let mut overlay1 = ThemeVariant::default();
    overlay1.colors.core.accent = Some(Rgba::rgb(255, 0, 0)); // red accent
    overlay1.fonts.family = Some("Noto Sans".into());

    let mut overlay2 = ThemeVariant::default();
    overlay2.colors.core.accent = Some(Rgba::rgb(0, 0, 255)); // blue accent (overwrites)
    overlay2.geometry.radius = Some(8.0);

    base.merge(&overlay1);
    base.merge(&overlay2);

    // background from base (neither overlay set it)
    assert_eq!(base.colors.core.background, Some(Rgba::rgb(255, 255, 255)));
    // accent: overlay2 wins (last-wins)
    assert_eq!(base.colors.core.accent, Some(Rgba::rgb(0, 0, 255)));
    // font from overlay1 (overlay2 didn't set it)
    assert_eq!(base.fonts.family.as_deref(), Some("Noto Sans"));
    // geometry from overlay2
    assert_eq!(base.geometry.radius, Some(8.0));
}

// ---------------------------------------------------------------------------
// is_empty() on all structs
// ---------------------------------------------------------------------------

#[test]
fn is_empty_all_structs() {
    assert!(ThemeColors::default().is_empty());
    assert!(CoreColors::default().is_empty());
    assert!(ActionColors::default().is_empty());
    assert!(StatusColors::default().is_empty());
    assert!(InteractiveColors::default().is_empty());
    assert!(PanelColors::default().is_empty());
    assert!(ComponentColors::default().is_empty());
    assert!(ThemeFonts::default().is_empty());
    assert!(ThemeGeometry::default().is_empty());
    assert!(ThemeSpacing::default().is_empty());
    assert!(ThemeVariant::default().is_empty());
}

// ---------------------------------------------------------------------------
// Compile-time trait assertions
// ---------------------------------------------------------------------------

#[test]
fn trait_assertions_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<Rgba>();
    assert_send_sync::<NativeTheme>();
    assert_send_sync::<ThemeVariant>();
    assert_send_sync::<ThemeColors>();
    assert_send_sync::<CoreColors>();
    assert_send_sync::<ActionColors>();
    assert_send_sync::<StatusColors>();
    assert_send_sync::<InteractiveColors>();
    assert_send_sync::<PanelColors>();
    assert_send_sync::<ComponentColors>();
    assert_send_sync::<ThemeFonts>();
    assert_send_sync::<ThemeGeometry>();
    assert_send_sync::<ThemeSpacing>();
    assert_send_sync::<Error>();
}

#[test]
fn trait_assertions_default_clone_debug() {
    fn assert_default_clone_debug<T: Default + Clone + std::fmt::Debug>() {}

    assert_default_clone_debug::<Rgba>();
    assert_default_clone_debug::<NativeTheme>();
    assert_default_clone_debug::<ThemeVariant>();
    assert_default_clone_debug::<ThemeColors>();
    assert_default_clone_debug::<CoreColors>();
    assert_default_clone_debug::<ActionColors>();
    assert_default_clone_debug::<StatusColors>();
    assert_default_clone_debug::<InteractiveColors>();
    assert_default_clone_debug::<PanelColors>();
    assert_default_clone_debug::<ComponentColors>();
    assert_default_clone_debug::<ThemeFonts>();
    assert_default_clone_debug::<ThemeGeometry>();
    assert_default_clone_debug::<ThemeSpacing>();

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
    let mut base = NativeTheme::new("Breeze Light");
    let mut light = ThemeVariant::default();

    // Populate base colors
    light.colors.core.accent = Some(Rgba::rgb(61, 174, 233));
    light.colors.core.background = Some(Rgba::rgb(252, 252, 252));
    light.colors.core.foreground = Some(Rgba::rgb(35, 38, 41));
    light.colors.core.surface = Some(Rgba::rgb(239, 240, 241));
    light.colors.core.border = Some(Rgba::rgb(188, 190, 191));
    light.colors.status.danger = Some(Rgba::rgb(218, 68, 83));
    light.colors.status.success = Some(Rgba::rgb(39, 174, 96));
    light.colors.interactive.selection = Some(Rgba::rgb(61, 174, 233));
    light.colors.interactive.link = Some(Rgba::rgb(41, 128, 185));
    light.colors.panel.sidebar = Some(Rgba::rgb(227, 229, 231));
    light.colors.component.button = Some(Rgba::rgb(239, 240, 241));

    // Populate base fonts
    light.fonts.family = Some("Noto Sans".into());
    light.fonts.size = Some(10.0);
    light.fonts.mono_family = Some("Hack".into());
    light.fonts.mono_size = Some(10.0);

    // Populate base geometry
    light.geometry.radius = Some(4.0);
    light.geometry.frame_width = Some(1.0);

    // Populate base spacing
    light.spacing.s = Some(8.0);
    light.spacing.m = Some(12.0);
    light.spacing.l = Some(16.0);

    base.light = Some(light);

    // User override: just accent color and font family
    let mut user_override = NativeTheme::new("User Override");
    let mut user_light = ThemeVariant::default();
    user_light.colors.core.accent = Some(Rgba::rgb(156, 39, 176)); // purple accent
    user_light.fonts.family = Some("Inter".into()); // different font
    user_override.light = Some(user_light);

    // Apply user override on top of base
    base.merge(&user_override);

    // Verify: name stays as base
    assert_eq!(base.name, "Breeze Light");

    let result = base.light.as_ref().unwrap();

    // Accent changed to purple (from user override)
    assert_eq!(
        result.colors.core.accent,
        Some(Rgba::rgb(156, 39, 176)),
        "accent should be overridden to purple"
    );

    // Font family changed (from user override)
    assert_eq!(
        result.fonts.family.as_deref(),
        Some("Inter"),
        "font family should be overridden to Inter"
    );

    // Everything else from base preserved
    assert_eq!(result.colors.core.background, Some(Rgba::rgb(252, 252, 252)));
    assert_eq!(result.colors.core.foreground, Some(Rgba::rgb(35, 38, 41)));
    assert_eq!(result.colors.status.danger, Some(Rgba::rgb(218, 68, 83)));
    assert_eq!(result.colors.interactive.link, Some(Rgba::rgb(41, 128, 185)));
    assert_eq!(result.fonts.size, Some(10.0));
    assert_eq!(result.fonts.mono_family.as_deref(), Some("Hack"));
    assert_eq!(result.geometry.radius, Some(4.0));
    assert_eq!(result.spacing.m, Some(12.0));

    // Serialize the merged result to TOML and verify it looks reasonable
    let toml_str = toml::to_string_pretty(&base).unwrap();
    assert!(toml_str.contains("Breeze Light"), "name should be in TOML");
    assert!(toml_str.contains("#9c27b0"), "purple accent hex should appear");
    assert!(toml_str.contains("Inter"), "overridden font should appear");
    assert!(toml_str.contains("#fcfcfc"), "base background should appear");
}
