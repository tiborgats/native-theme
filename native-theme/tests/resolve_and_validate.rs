//! Integration tests for resolve(), validate(), and derivation chains.
//!
//! Tests: idempotency, from_toml_with_base, with_overlay re-derivation,
//! pick_variant/into_variant fallback, ThemeVariant/ThemeSpec is_empty,
//! Rgba f32 quantization, live preset sync, lint_toml on all presets,
//! and bundled SVG content validation.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::*;

// ---------------------------------------------------------------------------
// Issue 2j: resolve() idempotency
// ---------------------------------------------------------------------------

#[test]
fn resolve_idempotency_on_preset() {
    for name in ThemeSpec::list_presets() {
        let theme = ThemeSpec::preset(name).unwrap();
        let mut variant = theme.light.clone().unwrap();
        variant.resolve_all();
        let first = variant.clone();
        variant.resolve_all();
        assert_eq!(
            variant, first,
            "preset '{name}': second resolve_all() changed the variant"
        );
    }
}

// ---------------------------------------------------------------------------
// Issues 2h, 9d: from_toml_with_base()
// ---------------------------------------------------------------------------

#[test]
fn from_toml_with_base_overlay_merges_correctly() {
    let custom_toml = r##"
name = "Custom"

[light.defaults]
accent_color = "#ff0000"
"##;
    let theme = ThemeSpec::from_toml_with_base(custom_toml, "adwaita").unwrap();

    // Base name preserved
    assert_eq!(theme.name, "Adwaita");

    // Overlay accent applied
    let light = theme.light.as_ref().unwrap();
    assert_eq!(light.defaults.accent_color, Some(Rgba::rgb(255, 0, 0)));

    // Base geometry preserved (adwaita has these)
    assert!(
        light.defaults.border.corner_radius.is_some(),
        "base radius should survive overlay"
    );
    assert!(
        light.defaults.font.family.is_some(),
        "base font.family should survive overlay"
    );
    // REMOVED: spacing assertion (ThemeSpacing deleted in Plan 01)

    // Resolved version should validate
    let mut resolved_variant = light.clone();
    resolved_variant.resolve_all();
    resolved_variant.validate().unwrap_or_else(|e| {
        panic!("from_toml_with_base overlay should resolve+validate: {e}");
    });
}

#[test]
fn from_toml_with_base_invalid_base_returns_err() {
    let result = ThemeSpec::from_toml_with_base("name = \"X\"", "no-such-preset");
    assert!(result.is_err(), "invalid base name should return Err");
    match result.unwrap_err() {
        Error::Unavailable(msg) => assert!(msg.contains("no-such-preset")),
        other => panic!("expected Unavailable, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Issue 7f: with_overlay() re-derivation (via ThemeSpec merge + resolve)
// ---------------------------------------------------------------------------

#[test]
fn overlay_accent_change_re_derives_widget_fields() {
    let theme = ThemeSpec::preset("material").unwrap();
    let mut variant = theme.dark.clone().unwrap();
    let original_accent = variant.defaults.accent_color.unwrap();

    // Resolve the original
    variant.resolve_all();
    let original_resolved = variant.validate().unwrap();

    // Now create a new variant from scratch with a different accent
    let mut variant2 = theme.dark.clone().unwrap();
    let new_accent = Rgba::rgb(255, 0, 0);
    assert_ne!(new_accent, original_accent);
    variant2.defaults.accent_color = Some(new_accent);
    variant2.resolve_all();
    let new_resolved = variant2.validate().unwrap();

    // Accent-derived fields should differ
    assert_eq!(
        new_resolved.defaults.accent_color, new_accent,
        "accent should be the new value"
    );
    assert_ne!(
        new_resolved.button.primary_background, original_resolved.button.primary_background,
        "button.primary_background should change with new accent"
    );
    assert_ne!(
        new_resolved.slider.fill_color, original_resolved.slider.fill_color,
        "slider.fill should change with new accent"
    );
    assert_ne!(
        new_resolved.checkbox.checked_background, original_resolved.checkbox.checked_background,
        "checkbox.checked_background should change with new accent"
    );
}

// ---------------------------------------------------------------------------
// Issue 7g: Rgba f32 quantization
// ---------------------------------------------------------------------------

#[test]
fn rgba_f32_quantization_behavior() {
    let rgba = Rgba::from_f32(0.5, 0.5, 0.5, 1.0);
    let arr = rgba.to_f32_array();
    // 0.5 * 255 = 127.5 -> round to 128 -> 128/255 = 0.50196...
    assert!(
        (arr[0] - 0.50196).abs() < 0.001,
        "expected ~0.50196 after u8 quantization, got {}",
        arr[0]
    );
    assert!(
        (arr[1] - 0.50196).abs() < 0.001,
        "expected ~0.50196 for green, got {}",
        arr[1]
    );
    assert!(
        (arr[2] - 0.50196).abs() < 0.001,
        "expected ~0.50196 for blue, got {}",
        arr[2]
    );
    assert_eq!(arr[3], 1.0, "alpha 1.0 should round-trip exactly");

    // Exact values (0.0, 1.0) round-trip losslessly
    let exact = Rgba::from_f32(0.0, 1.0, 0.0, 0.0);
    let arr2 = exact.to_f32_array();
    assert_eq!(arr2[0], 0.0, "0.0 should round-trip exactly");
    assert_eq!(arr2[1], 1.0, "1.0 should round-trip exactly");
}

// ---------------------------------------------------------------------------
// Issue 8a: Live preset sync test
// ---------------------------------------------------------------------------

#[test]
fn live_presets_geometry_matches_full_presets() {
    let pairs = [
        ("adwaita", "adwaita-live"),
        ("kde-breeze", "kde-breeze-live"),
        ("macos-sonoma", "macos-sonoma-live"),
        ("windows-11", "windows-11-live"),
    ];

    for (full_name, live_name) in pairs {
        let full = ThemeSpec::preset(full_name).unwrap();
        let live = ThemeSpec::preset(live_name).unwrap();

        for (label, full_var, live_var) in [
            (
                "light",
                full.light.as_ref().unwrap(),
                live.light.as_ref().unwrap(),
            ),
            (
                "dark",
                full.dark.as_ref().unwrap(),
                live.dark.as_ref().unwrap(),
            ),
        ] {
            // Geometry fields: radius, radius_lg, frame_width
            assert_eq!(
                full_var.defaults.border.corner_radius, live_var.defaults.border.corner_radius,
                "{full_name} {label} radius mismatch with live"
            );
            assert_eq!(
                full_var.defaults.border.corner_radius_lg,
                live_var.defaults.border.corner_radius_lg,
                "{full_name} {label} radius_lg mismatch with live"
            );
            assert_eq!(
                full_var.defaults.border.line_width, live_var.defaults.border.line_width,
                "{full_name} {label} frame_width mismatch with live"
            );
            // REMOVED: spacing comparison (ThemeSpacing deleted in Plan 01)
            // Button geometry
            assert_eq!(
                full_var.button.min_height, live_var.button.min_height,
                "{full_name} {label} button.min_height mismatch with live"
            );
            assert_eq!(
                full_var
                    .button
                    .border
                    .as_ref()
                    .and_then(|b| b.padding_horizontal),
                live_var
                    .button
                    .border
                    .as_ref()
                    .and_then(|b| b.padding_horizontal),
                "{full_name} {label} button.padding_horizontal mismatch with live"
            );
            assert_eq!(
                full_var
                    .button
                    .border
                    .as_ref()
                    .and_then(|b| b.padding_vertical),
                live_var
                    .button
                    .border
                    .as_ref()
                    .and_then(|b| b.padding_vertical),
                "{full_name} {label} button.padding_vertical mismatch with live"
            );
            // Input geometry
            assert_eq!(
                full_var.input.min_height, live_var.input.min_height,
                "{full_name} {label} input.min_height mismatch with live"
            );
            // Scrollbar
            assert_eq!(
                full_var.scrollbar.groove_width, live_var.scrollbar.groove_width,
                "{full_name} {label} scrollbar.width mismatch with live"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Issue 13a: lint_toml on all presets
// ---------------------------------------------------------------------------

#[test]
fn lint_toml_all_presets_no_warnings() {
    for name in ThemeSpec::list_presets() {
        let theme = ThemeSpec::preset(name).unwrap();
        let toml_str = theme.to_toml().unwrap();
        let warnings = ThemeSpec::lint_toml(&toml_str).unwrap();
        assert!(
            warnings.is_empty(),
            "preset '{name}' lint_toml() produced warnings: {warnings:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Issues 13b, 13c: pick_variant / into_variant fallback
// ---------------------------------------------------------------------------

#[test]
fn pick_variant_falls_back_to_other() {
    // Theme with only light variant
    let mut theme_light_only = ThemeSpec::new("Light Only");
    let mut light = ThemeVariant::default();
    light.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
    theme_light_only.light = Some(light);

    // pick_variant(true) should fall back to light
    let picked = theme_light_only.pick_variant(true);
    assert!(
        picked.is_some(),
        "pick_variant(true) should fall back to light variant"
    );
    assert_eq!(
        picked.unwrap().defaults.accent_color,
        Some(Rgba::rgb(0, 0, 255)),
        "should return the light variant as fallback for dark"
    );

    // Theme with only dark variant
    let mut theme_dark_only = ThemeSpec::new("Dark Only");
    let mut dark = ThemeVariant::default();
    dark.defaults.accent_color = Some(Rgba::rgb(255, 0, 0));
    theme_dark_only.dark = Some(dark);

    // pick_variant(false) should fall back to dark
    let picked = theme_dark_only.pick_variant(false);
    assert!(
        picked.is_some(),
        "pick_variant(false) should fall back to dark variant"
    );
    assert_eq!(
        picked.unwrap().defaults.accent_color,
        Some(Rgba::rgb(255, 0, 0)),
        "should return the dark variant as fallback for light"
    );
}

#[test]
fn into_variant_falls_back_to_other() {
    // Theme with only light variant
    let mut theme = ThemeSpec::new("Light Only");
    let mut light = ThemeVariant::default();
    light.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
    theme.light = Some(light);

    let variant = theme.into_variant(true);
    assert!(
        variant.is_some(),
        "into_variant(true) should fall back to light"
    );
    assert_eq!(
        variant.unwrap().defaults.accent_color,
        Some(Rgba::rgb(0, 0, 255))
    );
}

#[test]
fn pick_variant_returns_none_for_empty_theme() {
    let theme = ThemeSpec::new("Empty");
    assert!(theme.pick_variant(true).is_none());
    assert!(theme.pick_variant(false).is_none());
}

#[test]
fn into_variant_returns_none_for_empty_theme() {
    let theme1 = ThemeSpec::new("Empty");
    assert!(theme1.into_variant(true).is_none());
    let theme2 = ThemeSpec::new("Empty");
    assert!(theme2.into_variant(false).is_none());
}

// ---------------------------------------------------------------------------
// Issues 21h, 21i: ThemeVariant / ThemeSpec is_empty
// ---------------------------------------------------------------------------

#[test]
fn default_theme_variant_is_empty() {
    let v = ThemeVariant::default();
    assert!(v.is_empty(), "default ThemeVariant should be empty");
}

#[test]
fn default_theme_spec_is_empty() {
    let s = ThemeSpec::default();
    assert!(s.is_empty(), "default ThemeSpec should be empty");
}

#[test]
fn theme_variant_with_one_field_is_not_empty() {
    let mut v = ThemeVariant::default();
    v.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
    assert!(
        !v.is_empty(),
        "ThemeVariant with accent set should not be empty"
    );
}

#[test]
fn theme_spec_with_one_variant_is_not_empty() {
    let mut s = ThemeSpec::new("Test");
    s.light = Some(ThemeVariant::default());
    assert!(
        !s.is_empty(),
        "ThemeSpec with a light variant should not be empty"
    );
}

// ---------------------------------------------------------------------------
// Issue 9b: Bundled SVG content validation
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "material-icons")]
fn bundled_material_svg_starts_with_svg_tag() {
    for role in IconRole::ALL {
        if let Some(svg_bytes) = bundled_icon_svg(role, IconSet::Material) {
            let content = std::str::from_utf8(svg_bytes).unwrap_or_else(|_| {
                panic!("Material SVG for {role:?} is not valid UTF-8");
            });
            assert!(
                content.starts_with("<svg") || content.starts_with("<?xml"),
                "Material SVG for {role:?} should start with <svg or <?xml, got: {}...",
                &content[..content.len().min(40)]
            );
        }
    }
}

#[test]
#[cfg(feature = "lucide-icons")]
fn bundled_lucide_svg_starts_with_svg_tag() {
    for role in IconRole::ALL {
        if let Some(svg_bytes) = bundled_icon_svg(role, IconSet::Lucide) {
            let content = std::str::from_utf8(svg_bytes).unwrap_or_else(|_| {
                panic!("Lucide SVG for {role:?} is not valid UTF-8");
            });
            assert!(
                content.starts_with("<svg") || content.starts_with("<?xml"),
                "Lucide SVG for {role:?} should start with <svg or <?xml, got: {}...",
                &content[..content.len().min(40)]
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Bundled icon SVG returns None for non-bundled sets
// ---------------------------------------------------------------------------

#[test]
fn bundled_icon_svg_returns_none_for_non_bundled_sets() {
    for role in IconRole::ALL {
        assert!(
            bundled_icon_svg(role, IconSet::SfSymbols).is_none(),
            "SfSymbols should not have bundled SVGs for {role:?}"
        );
        assert!(
            bundled_icon_svg(role, IconSet::SegoeIcons).is_none(),
            "SegoeIcons should not have bundled SVGs for {role:?}"
        );
        assert!(
            bundled_icon_svg(role, IconSet::Freedesktop).is_none(),
            "Freedesktop should not have bundled SVGs for {role:?}"
        );
    }
}
