//! Integration tests for resolve(), validate(), and derivation chains.
//!
//! Tests: idempotency, preset+merge overlay, with_overlay re-derivation,
//! pick_variant/into_variant fallback, ThemeMode/Theme is_empty,
//! Rgba f32 quantization, live preset sync, lint_toml on all presets,
//! and bundled SVG content validation.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::color::Rgba;
use native_theme::error::Error;
use native_theme::theme::*;

// ---------------------------------------------------------------------------
// Issue 2j: resolve() idempotency
// ---------------------------------------------------------------------------

#[test]
fn resolve_idempotency_on_preset() {
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap();
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
// Issues 2h, 9d: preset + merge overlay
// ---------------------------------------------------------------------------

#[test]
fn preset_merge_overlay_merges_correctly() {
    let custom_toml = r##"
name = "Custom"

[light.defaults]
accent_color = "#ff0000"
"##;
    let mut theme = Theme::preset("adwaita").expect("adwaita preset");
    let overlay = Theme::from_toml(custom_toml).expect("overlay parse");
    theme.merge(&overlay);

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

    // Resolved version should validate
    let mut resolved_variant = light.clone();
    resolved_variant.resolve_all();
    resolved_variant.validate().unwrap_or_else(|e| {
        panic!("preset+merge overlay should resolve+validate: {e}");
    });
}

#[test]
fn preset_unknown_preset_returns_err() {
    let result = Theme::preset("no-such-preset");
    assert!(result.is_err(), "unknown preset name should return Err");
    let Error::UnknownPreset { name, .. } = result.unwrap_err() else {
        return;
    };
    assert!(name.contains("no-such-preset"));
}

// ---------------------------------------------------------------------------
// Issue 7f: with_overlay() re-derivation (via Theme merge + resolve)
// ---------------------------------------------------------------------------

#[test]
fn overlay_accent_change_re_derives_widget_fields() {
    let theme = Theme::preset("material").unwrap();
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
        let full = Theme::preset(full_name).unwrap();
        let live = Theme::preset(live_name).unwrap();

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
    for info in Theme::list_presets() {
        let name = info.key;
        let theme = Theme::preset(name).unwrap();
        let toml_str = theme.to_toml().unwrap();
        let warnings = Theme::lint_toml(&toml_str).unwrap();
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
    let mut theme_light_only = Theme {
        name: "Light Only".into(),
        ..Theme::default()
    };
    let mut light = ThemeMode::default();
    light.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
    theme_light_only.light = Some(light);

    // pick_variant(Dark) should fall back to light
    let picked = theme_light_only.pick_variant(ColorMode::Dark);
    assert!(
        picked.is_ok(),
        "pick_variant(Dark) should fall back to light variant"
    );
    assert_eq!(
        picked.unwrap().defaults.accent_color,
        Some(Rgba::rgb(0, 0, 255)),
        "should return the light variant as fallback for dark"
    );

    // Theme with only dark variant
    let mut theme_dark_only = Theme {
        name: "Dark Only".into(),
        ..Theme::default()
    };
    let mut dark = ThemeMode::default();
    dark.defaults.accent_color = Some(Rgba::rgb(255, 0, 0));
    theme_dark_only.dark = Some(dark);

    // pick_variant(Light) should fall back to dark
    let picked = theme_dark_only.pick_variant(ColorMode::Light);
    assert!(
        picked.is_ok(),
        "pick_variant(Light) should fall back to dark variant"
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
    let mut theme = Theme {
        name: "Light Only".into(),
        ..Theme::default()
    };
    let mut light = ThemeMode::default();
    light.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
    theme.light = Some(light);

    let variant = theme.into_variant(ColorMode::Dark);
    assert!(
        variant.is_ok(),
        "into_variant(Dark) should fall back to light"
    );
    assert_eq!(
        variant.unwrap().defaults.accent_color,
        Some(Rgba::rgb(0, 0, 255))
    );
}

#[test]
fn pick_variant_returns_err_for_empty_theme() {
    let theme = Theme {
        name: "Empty".into(),
        ..Theme::default()
    };
    assert!(theme.pick_variant(ColorMode::Dark).is_err());
    assert!(theme.pick_variant(ColorMode::Light).is_err());
}

#[test]
fn into_variant_returns_err_for_empty_theme() {
    let theme1 = Theme {
        name: "Empty".into(),
        ..Theme::default()
    };
    assert!(theme1.into_variant(ColorMode::Dark).is_err());
    let theme2 = Theme {
        name: "Empty".into(),
        ..Theme::default()
    };
    assert!(theme2.into_variant(ColorMode::Light).is_err());
}

// ---------------------------------------------------------------------------
// Issues 21h, 21i: ThemeMode / Theme is_empty
// ---------------------------------------------------------------------------

#[test]
fn default_theme_variant_is_empty() {
    let v = ThemeMode::default();
    assert!(v.is_empty(), "default ThemeMode should be empty");
}

#[test]
fn default_theme_spec_is_empty() {
    let s = Theme::default();
    assert!(s.is_empty(), "default Theme should be empty");
}

#[test]
fn theme_variant_with_one_field_is_not_empty() {
    let mut v = ThemeMode::default();
    v.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
    assert!(
        !v.is_empty(),
        "ThemeMode with accent set should not be empty"
    );
}

#[test]
fn theme_spec_with_one_variant_is_not_empty() {
    let mut s = Theme {
        name: "Test".into(),
        ..Theme::default()
    };
    s.light = Some(ThemeMode::default());
    assert!(
        !s.is_empty(),
        "Theme with a light variant should not be empty"
    );
}

// ---------------------------------------------------------------------------
// Issue 9b: Bundled SVG content validation
// ---------------------------------------------------------------------------

// G3 (Phase 93-03): migrated from the demoted `bundled_icon_svg` (now pub(crate))
// to the public `IconLoader` builder. Bundled icon sets always return
// `IconData::Svg` with `Cow::Borrowed` to static bytes, so `cow.as_ref()` is
// a zero-copy view back to `&'static [u8]`.

#[test]
#[cfg(feature = "material-icons")]
fn bundled_material_svg_starts_with_svg_tag() {
    use native_theme::icons::IconLoader;
    for role in IconRole::ALL {
        if let Some(data) = IconLoader::new(role).set(IconSet::Material).load() {
            let IconData::Svg(cow) = data else { continue };
            let svg_bytes: &[u8] = cow.as_ref();
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
    use native_theme::icons::IconLoader;
    for role in IconRole::ALL {
        if let Some(data) = IconLoader::new(role).set(IconSet::Lucide).load() {
            let IconData::Svg(cow) = data else { continue };
            let svg_bytes: &[u8] = cow.as_ref();
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
// Platform sets return None when the target OS doesn't match
// ---------------------------------------------------------------------------
//
// G3 (Phase 93-03): the original test asserted that `bundled_icon_svg`
// (bundled lookup) has no entries for non-bundled sets. With IconLoader,
// the equivalent observable behaviour is that platform-native sets
// (SfSymbols, SegoeIcons) return None on the wrong OS. Freedesktop is
// no longer asserted None here because on Linux with `system-icons` IconLoader
// intentionally DOES load freedesktop icons via the filesystem.

#[test]
#[cfg(not(target_os = "macos"))]
fn sf_symbols_returns_none_on_non_macos() {
    use native_theme::icons::IconLoader;
    for role in IconRole::ALL {
        assert!(
            IconLoader::new(role)
                .set(IconSet::SfSymbols)
                .load()
                .is_none(),
            "SfSymbols should not load on non-macOS for {role:?}"
        );
    }
}

#[test]
#[cfg(not(target_os = "windows"))]
fn segoe_icons_returns_none_on_non_windows() {
    use native_theme::icons::IconLoader;
    for role in IconRole::ALL {
        assert!(
            IconLoader::new(role)
                .set(IconSet::SegoeIcons)
                .load()
                .is_none(),
            "SegoeIcons should not load on non-Windows for {role:?}"
        );
    }
}
