use super::*;
use crate::Rgba;
use crate::model::font::FontSize;
use crate::model::{DialogButtonOrder, FontSpec, TextScaleEntry};

/// Helper: build a ThemeMode with all defaults.* fields populated.
fn variant_with_defaults() -> ThemeMode {
    let c1 = Rgba::rgb(0, 120, 215); // accent
    let c2 = Rgba::rgb(255, 255, 255); // background
    let c3 = Rgba::rgb(30, 30, 30); // foreground
    let c4 = Rgba::rgb(240, 240, 240); // surface
    let c5 = Rgba::rgb(200, 200, 200); // border
    let c6 = Rgba::rgb(128, 128, 128); // muted
    let c7 = Rgba::rgb(0, 0, 0); // shadow
    let c8 = Rgba::rgb(0, 100, 200); // link
    let c9 = Rgba::rgb(255, 255, 255); // accent_foreground
    let c10 = Rgba::rgb(220, 53, 69); // danger
    let c11 = Rgba::rgb(255, 255, 255); // danger_foreground
    let c12 = Rgba::rgb(240, 173, 78); // warning
    let c13 = Rgba::rgb(30, 30, 30); // warning_foreground
    let c14 = Rgba::rgb(40, 167, 69); // success
    let c15 = Rgba::rgb(255, 255, 255); // success_foreground
    let c16 = Rgba::rgb(0, 120, 215); // info
    let c17 = Rgba::rgb(255, 255, 255); // info_foreground

    let mut v = ThemeMode::default();
    v.defaults.accent_color = Some(c1);
    v.defaults.background_color = Some(c2);
    v.defaults.text_color = Some(c3);
    v.defaults.surface_color = Some(c4);
    v.defaults.border.color = Some(c5);
    v.defaults.muted_color = Some(c6);
    v.defaults.shadow_color = Some(c7);
    v.defaults.link_color = Some(c8);
    v.defaults.accent_text_color = Some(c9);
    v.defaults.selection_text_color = Some(Rgba::rgb(255, 255, 255));
    v.defaults.disabled_text_color = Some(Rgba::rgb(160, 160, 160));
    v.defaults.danger_color = Some(c10);
    v.defaults.danger_text_color = Some(c11);
    v.defaults.warning_color = Some(c12);
    v.defaults.warning_text_color = Some(c13);
    v.defaults.success_color = Some(c14);
    v.defaults.success_text_color = Some(c15);
    v.defaults.info_color = Some(c16);
    v.defaults.info_text_color = Some(c17);

    v.defaults.border.corner_radius = Some(4.0);
    v.defaults.border.corner_radius_lg = Some(8.0);
    v.defaults.border.line_width = Some(1.0);
    v.defaults.disabled_opacity = Some(0.5);
    v.defaults.border.opacity = Some(0.15);
    v.defaults.border.shadow_enabled = Some(true);

    v.defaults.focus_ring_width = Some(2.0);
    v.defaults.focus_ring_offset = Some(1.0);

    v.defaults.font = FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        color: Some(c3), // foreground
        ..Default::default()
    };
    v.defaults.line_height = Some(1.4);
    v.defaults.mono_font = FontSpec {
        family: Some("JetBrains Mono".into()),
        size: Some(FontSize::Px(13.0)),
        weight: Some(400),
        color: Some(c3), // foreground
        ..Default::default()
    };

    // REMOVED: defaults.spacing not in new schema
    // REMOVED: defaults.spacing not in new schema
    // REMOVED: defaults.spacing not in new schema
    // REMOVED: defaults.spacing not in new schema
    // REMOVED: defaults.spacing not in new schema
    // REMOVED: defaults.spacing not in new schema
    // REMOVED: defaults.spacing not in new schema

    v.defaults.icon_sizes.toolbar = Some(24.0);
    v.defaults.icon_sizes.small = Some(16.0);
    v.defaults.icon_sizes.large = Some(32.0);
    v.defaults.icon_sizes.dialog = Some(22.0);
    v.defaults.icon_sizes.panel = Some(20.0);

    v.defaults.text_scaling_factor = Some(1.0);
    v.defaults.reduce_motion = Some(false);
    v.defaults.high_contrast = Some(false);
    v.defaults.reduce_transparency = Some(false);

    v
}

// ===== Phase 1: Defaults internal chains =====

#[test]
fn resolve_phase1_accent_fills_selection_and_focus_ring() {
    let mut v = ThemeMode::default();
    v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
    v.resolve();
    assert_eq!(
        v.defaults.selection_background,
        Some(Rgba::rgb(0, 120, 215))
    );
    assert_eq!(v.defaults.focus_ring_color, Some(Rgba::rgb(0, 120, 215)));
}

#[test]
fn resolve_phase1_selection_fills_selection_inactive() {
    let mut v = ThemeMode::default();
    v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
    v.resolve();
    // selection_inactive should be set from selection (which was set from accent)
    assert_eq!(
        v.defaults.selection_inactive_background,
        Some(Rgba::rgb(0, 120, 215))
    );
}

#[test]
fn resolve_phase1_explicit_selection_preserved() {
    let mut v = ThemeMode::default();
    v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
    v.defaults.selection_background = Some(Rgba::rgb(100, 100, 100));
    v.resolve();
    // Explicit selection preserved
    assert_eq!(
        v.defaults.selection_background,
        Some(Rgba::rgb(100, 100, 100))
    );
    // selection_inactive inherits from the explicit selection
    assert_eq!(
        v.defaults.selection_inactive_background,
        Some(Rgba::rgb(100, 100, 100))
    );
}

#[test]
fn resolve_phase1_font_color_from_text_color() {
    let mut v = ThemeMode::default();
    v.defaults.text_color = Some(Rgba::rgb(30, 30, 30));
    v.resolve();
    assert_eq!(
        v.defaults.font.color,
        Some(Rgba::rgb(30, 30, 30)),
        "defaults.font.color <- defaults.text_color"
    );
    assert_eq!(
        v.defaults.mono_font.color,
        Some(Rgba::rgb(30, 30, 30)),
        "defaults.mono_font.color <- defaults.font.color"
    );
}

#[test]
fn resolve_phase1_font_color_explicit_preserved() {
    let mut v = ThemeMode::default();
    v.defaults.text_color = Some(Rgba::rgb(30, 30, 30));
    v.defaults.font.color = Some(Rgba::rgb(50, 50, 50));
    v.resolve();
    assert_eq!(
        v.defaults.font.color,
        Some(Rgba::rgb(50, 50, 50)),
        "explicit font.color preserved"
    );
    // mono_font inherits from font.color, not text_color
    assert_eq!(
        v.defaults.mono_font.color,
        Some(Rgba::rgb(50, 50, 50)),
        "mono_font.color <- font.color (not text_color)"
    );
}

// ===== Phase 2: Safety nets =====

#[test]
fn resolve_phase2_safety_nets() {
    let mut v = ThemeMode::default();
    v.defaults.text_color = Some(Rgba::rgb(30, 30, 30));
    v.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
    v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
    v.defaults.muted_color = Some(Rgba::rgb(128, 128, 128));
    v.resolve();

    // Removed safety nets: line_height, accent_text_color, shadow_color,
    // disabled_text_color are no longer fabricated -- all 20 presets provide them.
    assert_eq!(
        v.defaults.line_height, None,
        "line_height no longer fabricated"
    );
    assert_eq!(
        v.defaults.accent_text_color, None,
        "accent_text_color no longer fabricated"
    );
    assert_eq!(
        v.defaults.shadow_color, None,
        "shadow_color no longer fabricated"
    );
    assert_eq!(
        v.defaults.disabled_text_color, None,
        "disabled_text_color no longer fabricated"
    );

    // dialog.button_order moved to resolve_platform_defaults (no longer in resolve()):
    assert!(
        v.dialog.button_order.is_none(),
        "dialog.button_order no longer set by resolve() -- moved to resolve_platform_defaults"
    );
    assert_eq!(
        v.input.caret_color,
        Some(Rgba::rgb(30, 30, 30)),
        "input.caret <- foreground"
    );
    assert_eq!(
        v.scrollbar.track_color,
        Some(Rgba::rgb(255, 255, 255)),
        "scrollbar.track <- background"
    );
    assert_eq!(
        v.spinner.fill_color,
        Some(Rgba::rgb(0, 120, 215)),
        "spinner.fill <- accent"
    );
    assert_eq!(
        v.popover.background_color,
        Some(Rgba::rgb(255, 255, 255)),
        "popover.background <- background"
    );
    assert_eq!(
        v.list.background_color,
        Some(Rgba::rgb(255, 255, 255)),
        "list.background <- background"
    );
    assert_eq!(
        v.dialog.background_color,
        Some(Rgba::rgb(255, 255, 255)),
        "dialog.background <- background"
    );
}

// ===== Font DPI conversion (via validate) =====
//
// These tests verify that FontSize::Pt values are correctly converted to
// logical pixels during validate(). This replaces the old Phase 1.5
// resolve_font_dpi_conversion() tests -- conversion now happens in validate
// via FontSize::to_px(dpi), not in-place mutation during resolve.

/// Standard screen DPI (CSS reference pixel).
const TEST_DPI_STANDARD: f32 = 96.0;
/// Apple coordinate system DPI (1pt = 1px identity).
const TEST_DPI_APPLE: f32 = 72.0;

#[test]
fn validate_converts_pt_to_px_at_96_dpi() {
    // 10pt at 96 DPI -> 10 * 96/72 = 13.333...px
    let mut v = fully_populated_variant();
    v.defaults.font_dpi = Some(TEST_DPI_STANDARD);
    v.defaults.font.size = Some(FontSize::Pt(10.0));
    v.defaults.mono_font.size = Some(FontSize::Pt(10.0));
    let resolved = v.validate().expect("should validate");
    let size = resolved.defaults.font.size;
    assert!(
        (size - 13.333).abs() < 0.01,
        "10pt at 96 DPI should be ~13.333px, got {size}"
    );
}

#[test]
fn validate_px_ignores_dpi() {
    // FontSize::Px(14.0) stays 14.0 regardless of dpi
    let mut v = fully_populated_variant();
    v.defaults.font_dpi = Some(TEST_DPI_STANDARD);
    v.defaults.font.size = Some(FontSize::Px(14.0));
    v.defaults.mono_font.size = Some(FontSize::Px(13.0));
    let resolved = v.validate().expect("should validate");
    assert_eq!(resolved.defaults.font.size, 14.0);
}

#[test]
fn validate_pt_at_72_dpi_is_identity() {
    // 72 DPI (macOS): 13pt * 72/72 = 13px (identity)
    let mut v = fully_populated_variant();
    v.defaults.font_dpi = Some(TEST_DPI_APPLE);
    v.defaults.font.size = Some(FontSize::Pt(13.0));
    v.defaults.mono_font.size = Some(FontSize::Pt(13.0));
    let resolved = v.validate().expect("should validate");
    assert!(
        (resolved.defaults.font.size - 13.0).abs() < 0.01,
        "13pt at 72 DPI should stay 13.0px"
    );
}

#[test]
fn validate_text_scale_pt_converted() {
    // text_scale caption: size_pt + line_height both converted
    let mut v = fully_populated_variant();
    v.defaults.font_dpi = Some(TEST_DPI_STANDARD);
    v.defaults.font.size = Some(FontSize::Pt(10.0));
    v.defaults.mono_font.size = Some(FontSize::Pt(10.0));
    v.text_scale.caption = Some(TextScaleEntry {
        size: Some(FontSize::Pt(9.0)),
        weight: Some(400),
        line_height: Some(FontSize::Pt(12.6)),
    });
    let resolved = v.validate().expect("should validate");
    let cap = &resolved.text_scale.caption;
    assert!(
        (cap.size - 12.0).abs() < 0.01,
        "9pt at 96 DPI should be 12.0px, got {}",
        cap.size
    );
    assert!(
        (cap.line_height - 16.8).abs() < 0.01,
        "12.6 line_height at 96 DPI should be 16.8px, got {}",
        cap.line_height
    );
}

#[test]
fn validate_per_widget_font_pt_converted() {
    // Per-widget font with FontSize::Pt gets converted in validate
    let mut v = fully_populated_variant();
    v.defaults.font_dpi = Some(TEST_DPI_STANDARD);
    v.defaults.font.size = Some(FontSize::Pt(10.0));
    v.defaults.mono_font.size = Some(FontSize::Pt(10.0));
    // Override only the size on the existing button font (preserve family/weight/color)
    if let Some(ref mut bf) = v.button.font {
        bf.size = Some(FontSize::Pt(11.0));
    }
    let resolved = v.validate().expect("should validate");
    let btn_size = resolved.button.font.size;
    assert!(
        (btn_size - 14.666).abs() < 0.01,
        "11pt at 96 DPI should be ~14.666px, got {btn_size}"
    );
}

#[test]
fn validate_no_dpi_uses_default_96() {
    // font_dpi=None -> DEFAULT_FONT_DPI (96) is used as fallback in validate
    let mut v = fully_populated_variant();
    v.defaults.font_dpi = None;
    v.defaults.font.size = Some(FontSize::Pt(10.0));
    v.defaults.mono_font.size = Some(FontSize::Pt(10.0));
    let resolved = v.validate().expect("should validate");
    let size = resolved.defaults.font.size;
    // With DEFAULT_FONT_DPI=96: 10 * 96/72 = 13.333
    assert!(
        (size - 13.333).abs() < 0.01,
        "10pt with default 96 DPI should be ~13.333px, got {size}"
    );
}

// ===== Phase 3: Accent propagation (RESOLVE-06) =====

#[test]
fn resolve_phase3_accent_propagation() {
    let mut v = ThemeMode::default();
    v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
    v.resolve();

    assert_eq!(
        v.button.primary_background,
        Some(Rgba::rgb(0, 120, 215)),
        "button.primary_background <- accent"
    );
    assert_eq!(
        v.checkbox.checked_background,
        Some(Rgba::rgb(0, 120, 215)),
        "checkbox.checked_background <- accent"
    );
    assert_eq!(
        v.slider.fill_color,
        Some(Rgba::rgb(0, 120, 215)),
        "slider.fill <- accent"
    );
    assert_eq!(
        v.progress_bar.fill_color,
        Some(Rgba::rgb(0, 120, 215)),
        "progress_bar.fill <- accent"
    );
    assert_eq!(
        v.switch.checked_background,
        Some(Rgba::rgb(0, 120, 215)),
        "switch.checked_background <- accent"
    );
}

// ===== Phase 3: Font sub-field inheritance (RESOLVE-04) =====

#[test]
fn resolve_phase3_font_subfield_inheritance() {
    let mut v = ThemeMode::default();
    v.defaults.font = FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    };
    // Menu has a font with only size set
    v.menu.font = Some(FontSpec {
        family: None,
        size: Some(FontSize::Px(12.0)),
        weight: None,
        ..Default::default()
    });
    v.resolve();

    let menu_font = v.menu.font.as_ref().unwrap();
    assert_eq!(
        menu_font.family.as_deref(),
        Some("Inter"),
        "family from defaults"
    );
    assert_eq!(
        menu_font.size,
        Some(FontSize::Px(12.0)),
        "explicit size preserved"
    );
    assert_eq!(menu_font.weight, Some(400), "weight from defaults");
}

#[test]
fn resolve_phase3_font_entire_inheritance() {
    let mut v = ThemeMode::default();
    v.defaults.font = FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    };
    // button.font is None, should inherit entire defaults.font
    assert!(v.button.font.is_none());
    v.resolve();

    let button_font = v.button.font.as_ref().unwrap();
    assert_eq!(button_font.family.as_deref(), Some("Inter"));
    assert_eq!(button_font.size, Some(FontSize::Px(14.0)));
    assert_eq!(button_font.weight, Some(400));
}

// ===== Phase 3: Text scale inheritance (RESOLVE-05) =====

#[test]
fn resolve_phase3_text_scale_inheritance() {
    let mut v = ThemeMode::default();
    v.defaults.font = FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    };
    v.defaults.line_height = Some(1.4);
    // Leave text_scale entries as None -- resolve fills from defaults.font
    v.resolve();

    // All entries inherit size = defaults.font.size (14.0),
    // weight = defaults.font.weight (400),
    // line_height = defaults.line_height * size = 1.4 * 14.0 = 19.6

    // caption
    assert!(v.text_scale.caption.is_some(), "caption present");
    if let Some(ref cap) = v.text_scale.caption {
        assert_eq!(
            cap.size,
            Some(FontSize::Px(14.0)),
            "caption size inherits defaults.font.size"
        );
        assert_eq!(
            cap.weight,
            Some(400),
            "caption weight inherits defaults.font.weight"
        );
        assert_eq!(
            cap.line_height,
            Some(FontSize::Px(19.6)),
            "caption line_height = lh * size"
        );
    }

    // section_heading
    assert!(
        v.text_scale.section_heading.is_some(),
        "section_heading present"
    );
    if let Some(ref sh) = v.text_scale.section_heading {
        assert_eq!(
            sh.size,
            Some(FontSize::Px(14.0)),
            "section_heading size inherits defaults.font.size"
        );
        assert_eq!(
            sh.weight,
            Some(400),
            "section_heading weight inherits defaults.font.weight"
        );
    }

    // dialog_title
    assert!(v.text_scale.dialog_title.is_some(), "dialog_title present");
    if let Some(ref dt) = v.text_scale.dialog_title {
        assert_eq!(
            dt.size,
            Some(FontSize::Px(14.0)),
            "dialog_title size inherits defaults.font.size"
        );
        assert_eq!(
            dt.weight,
            Some(400),
            "dialog_title weight inherits defaults.font.weight"
        );
    }

    // display
    assert!(v.text_scale.display.is_some(), "display present");
    if let Some(ref disp) = v.text_scale.display {
        assert_eq!(
            disp.size,
            Some(FontSize::Px(14.0)),
            "display size inherits defaults.font.size"
        );
        assert_eq!(
            disp.weight,
            Some(400),
            "display weight inherits defaults.font.weight"
        );
    }
}

// ===== Phase 3: Color inheritance =====

#[test]
fn resolve_phase3_color_inheritance() {
    let mut v = variant_with_defaults();
    v.resolve();

    // window
    assert_eq!(v.window.background_color, Some(Rgba::rgb(255, 255, 255)));
    assert_eq!(
        v.window.border.as_ref().and_then(|b| b.color),
        v.defaults.border.color
    );
    // button
    assert_eq!(
        v.button.border.as_ref().and_then(|b| b.color),
        v.defaults.border.color
    );
    // tooltip
    assert_eq!(
        v.tooltip.border.as_ref().and_then(|b| b.corner_radius),
        v.defaults.border.corner_radius
    );
}

// ===== Phase 4: Widget-to-widget =====

#[test]
fn resolve_phase4_inactive_title_bar_from_active() {
    let mut v = ThemeMode::default();
    v.defaults.surface_color = Some(Rgba::rgb(240, 240, 240));
    v.defaults.text_color = Some(Rgba::rgb(30, 30, 30));
    v.resolve();

    // title_bar_background was set to surface in Phase 3
    // inactive should inherit from active
    assert_eq!(
        v.window.inactive_title_bar_background,
        v.window.title_bar_background
    );
    assert_eq!(
        v.window.inactive_title_bar_text_color,
        v.window.title_bar_font.as_ref().and_then(|f| f.color)
    );
}

// ===== Preserve explicit values =====

#[test]
fn resolve_does_not_overwrite_existing_some_values() {
    let mut v = variant_with_defaults();
    let explicit = Rgba::rgb(255, 0, 0);
    v.window.background_color = Some(explicit);
    v.button.primary_background = Some(explicit);
    v.resolve();

    assert_eq!(
        v.window.background_color,
        Some(explicit),
        "window.background preserved"
    );
    assert_eq!(
        v.button.primary_background,
        Some(explicit),
        "button.primary_background preserved"
    );
}

// ===== Idempotent =====

#[test]
fn resolve_is_idempotent() {
    let mut v = variant_with_defaults();
    v.resolve();
    let after_first = v.clone();
    v.resolve();
    assert_eq!(v, after_first, "second resolve() produces same result");
}

#[test]
fn scrollbar_thumb_hover_inherits_muted_color() {
    let mut v = variant_with_defaults();
    let muted = v.defaults.muted_color;
    // Ensure thumb and thumb_hover are not pre-set so resolve derives them
    v.scrollbar.thumb_color = None;
    v.scrollbar.thumb_hover_color = None;
    v.resolve();
    assert!(
        v.scrollbar.thumb_hover_color.is_some(),
        "thumb_hover should be resolved"
    );
    assert_eq!(
        v.scrollbar.thumb_hover_color, muted,
        "thumb_hover_color should inherit from defaults.muted_color"
    );
    // Both thumb_color and thumb_hover_color inherit from muted_color
    assert_eq!(
        v.scrollbar.thumb_color, muted,
        "thumb_color should also inherit from defaults.muted_color"
    );
}

// ===== All 8 font-carrying widgets get resolved fonts =====

#[test]
fn resolve_all_font_carrying_widgets_get_resolved_fonts() {
    let mut v = ThemeMode::default();
    v.defaults.font = FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    };
    v.resolve();

    // All 8 should now have Some(FontSpec)
    assert!(v.window.title_bar_font.is_some(), "window.title_bar_font");
    assert!(v.button.font.is_some(), "button.font");
    assert!(v.input.font.is_some(), "input.font");
    assert!(v.menu.font.is_some(), "menu.font");
    assert!(v.tooltip.font.is_some(), "tooltip.font");
    assert!(v.toolbar.font.is_some(), "toolbar.font");
    assert!(v.status_bar.font.is_some(), "status_bar.font");
    assert!(v.dialog.title_font.is_some(), "dialog.title_font");

    // Each should have the defaults values
    for (name, font) in [
        ("window.title_bar_font", &v.window.title_bar_font),
        ("button.font", &v.button.font),
        ("input.font", &v.input.font),
        ("menu.font", &v.menu.font),
        ("tooltip.font", &v.tooltip.font),
        ("toolbar.font", &v.toolbar.font),
        ("status_bar.font", &v.status_bar.font),
        ("dialog.title_font", &v.dialog.title_font),
    ] {
        let f = font.as_ref().unwrap();
        assert_eq!(f.family.as_deref(), Some("Inter"), "{name} family");
        assert_eq!(f.size, Some(FontSize::Px(14.0)), "{name} size");
        assert_eq!(f.weight, Some(400), "{name} weight");
    }
}

// ===== validate() tests =====

/// Build a fully-populated ThemeMode (all fields Some) for validate() testing.
fn fully_populated_variant() -> ThemeMode {
    let mut v = variant_with_defaults();
    let c = Rgba::rgb(128, 128, 128);

    // Ensure derived defaults are set (variant_with_defaults doesn't set these)
    v.defaults.selection_background = Some(Rgba::rgb(0, 120, 215));
    v.defaults.selection_text_color = Some(Rgba::rgb(255, 255, 255));
    v.defaults.selection_inactive_background = Some(Rgba::rgb(0, 120, 215));
    v.defaults.text_selection_background = Some(Rgba::rgb(0, 120, 215));
    v.defaults.text_selection_color = Some(Rgba::rgb(255, 255, 255));
    v.defaults.focus_ring_color = Some(Rgba::rgb(0, 120, 215));
    v.defaults.border.padding_horizontal = Some(0.0);
    v.defaults.border.padding_vertical = Some(0.0);

    // icon_set / icon_theme
    v.icon_set = Some(crate::IconSet::Freedesktop);
    v.icon_theme = Some("breeze".into());

    // window
    v.window.background_color = Some(c);
    v.window.title_bar_font.get_or_insert_default().color = Some(c);
    v.window.border.get_or_insert_default().color = Some(c);
    v.window.title_bar_background = Some(c);
    v.window.title_bar_font.get_or_insert_default().color = Some(c);
    v.window.inactive_title_bar_background = Some(c);
    v.window.inactive_title_bar_text_color = Some(c);
    v.window.border.get_or_insert_default().corner_radius = Some(8.0);
    v.window.border.get_or_insert_default().line_width = Some(1.0);
    v.window.border.get_or_insert_default().shadow_enabled = Some(true);
    v.window.title_bar_font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // button
    v.button.background_color = Some(c);
    v.button.font.get_or_insert_default().color = Some(c);
    v.button.border.get_or_insert_default().color = Some(c);
    v.button.primary_background = Some(c);
    v.button.primary_text_color = Some(c);
    v.button.min_width = Some(64.0);
    v.button.min_height = Some(28.0);
    v.button.border.get_or_insert_default().padding_horizontal = Some(12.0);
    v.button.border.get_or_insert_default().padding_vertical = Some(6.0);
    v.button.border.get_or_insert_default().corner_radius = Some(4.0);
    v.button.border.get_or_insert_default().line_width = Some(1.0);
    v.button.icon_text_gap = Some(6.0);
    v.button.disabled_opacity = Some(0.5);
    v.button.hover_background = Some(c);
    v.button.hover_text_color = Some(c);
    v.button.active_text_color = Some(c);
    v.button.disabled_text_color = Some(c);
    v.button.active_background = Some(c);
    v.button.disabled_background = Some(c);
    v.button.border.get_or_insert_default().shadow_enabled = Some(false);
    v.button.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // input
    v.input.background_color = Some(c);
    v.input.font.get_or_insert_default().color = Some(c);
    v.input.border.get_or_insert_default().color = Some(c);
    v.input.placeholder_color = Some(c);
    v.input.caret_color = Some(c);
    v.input.selection_background = Some(c);
    v.input.selection_text_color = Some(c);
    v.input.min_height = Some(28.0);
    v.input.border.get_or_insert_default().padding_horizontal = Some(8.0);
    v.input.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.input.disabled_opacity = Some(0.5);
    v.input.disabled_text_color = Some(c);
    v.input.hover_border_color = Some(c);
    v.input.focus_border_color = Some(c);
    v.input.disabled_background = Some(c);
    v.input.border.get_or_insert_default().corner_radius = Some(4.0);
    v.input.border.get_or_insert_default().line_width = Some(1.0);
    v.input.border.get_or_insert_default().shadow_enabled = Some(false);
    v.input.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // checkbox
    v.checkbox.background_color = Some(c);
    v.checkbox.checked_background = Some(c);
    v.checkbox.indicator_color = Some(c);
    v.checkbox.indicator_width = Some(18.0);
    v.checkbox.label_gap = Some(6.0);
    v.checkbox.disabled_opacity = Some(0.5);
    v.checkbox.disabled_text_color = Some(c);
    v.checkbox.hover_background = Some(c);
    v.checkbox.disabled_background = Some(c);
    v.checkbox.unchecked_background = Some(c);
    v.checkbox.unchecked_border_color = Some(c);
    v.checkbox.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.checkbox.border.get_or_insert_default().corner_radius = Some(2.0);
    v.checkbox.border.get_or_insert_default().line_width = Some(1.0);
    v.checkbox.border.get_or_insert_default().shadow_enabled = Some(false);

    // menu
    v.menu.background_color = Some(c);
    v.menu.font.get_or_insert_default().color = Some(c);
    v.menu.separator_color = Some(c);
    v.menu.row_height = Some(28.0);
    v.menu.border.get_or_insert_default().padding_horizontal = Some(8.0);
    v.menu.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.menu.icon_text_gap = Some(6.0);
    v.menu.icon_size = Some(16.0);
    v.menu.hover_background = Some(c);
    v.menu.hover_text_color = Some(c);
    v.menu.disabled_text_color = Some(c);
    v.menu.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // tooltip
    v.tooltip.background_color = Some(c);
    v.tooltip.font.get_or_insert_default().color = Some(c);
    v.tooltip.border.get_or_insert_default().padding_horizontal = Some(6.0);
    v.tooltip.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.tooltip.max_width = Some(300.0);
    v.tooltip.border.get_or_insert_default().corner_radius = Some(4.0);
    v.tooltip.border.get_or_insert_default().line_width = Some(1.0);
    v.tooltip.border.get_or_insert_default().shadow_enabled = Some(false);
    v.tooltip.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // scrollbar
    v.scrollbar.track_color = Some(c);
    v.scrollbar.thumb_color = Some(c);
    v.scrollbar.thumb_hover_color = Some(c);
    v.scrollbar.groove_width = Some(14.0);
    v.scrollbar.min_thumb_length = Some(20.0);
    v.scrollbar.thumb_width = Some(8.0);
    v.scrollbar.overlay_mode = Some(false);
    v.scrollbar.thumb_active_color = Some(c);

    // slider
    v.slider.fill_color = Some(c);
    v.slider.track_color = Some(c);
    v.slider.thumb_color = Some(c);
    v.slider.track_height = Some(4.0);
    v.slider.thumb_diameter = Some(16.0);
    v.slider.tick_mark_length = Some(6.0);
    v.slider.disabled_opacity = Some(0.5);
    v.slider.thumb_hover_color = Some(c);
    v.slider.disabled_fill_color = Some(c);
    v.slider.disabled_track_color = Some(c);
    v.slider.disabled_thumb_color = Some(c);

    // progress_bar
    v.progress_bar.fill_color = Some(c);
    v.progress_bar.track_color = Some(c);
    v.progress_bar.track_height = Some(6.0);
    v.progress_bar.min_width = Some(100.0);
    v.progress_bar.border.get_or_insert_default().corner_radius = Some(3.0);
    v.progress_bar.border.get_or_insert_default().color = Some(c);
    v.progress_bar.border.get_or_insert_default().line_width = Some(1.0);
    v.progress_bar.border.get_or_insert_default().shadow_enabled = Some(false);

    // tab
    v.tab.background_color = Some(c);
    v.tab.active_background = Some(c);
    v.tab.active_text_color = Some(c);
    v.tab.bar_background = Some(c);
    v.tab.min_width = Some(60.0);
    v.tab.min_height = Some(32.0);
    v.tab.hover_text_color = Some(c);
    v.tab.hover_background = Some(c);
    v.tab.border.get_or_insert_default().padding_horizontal = Some(12.0);
    v.tab.border.get_or_insert_default().padding_vertical = Some(6.0);
    v.tab.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // sidebar
    v.sidebar.background_color = Some(c);
    v.sidebar.selection_background = Some(c);
    v.sidebar.selection_text_color = Some(c);
    v.sidebar.hover_background = Some(c);
    v.sidebar.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.sidebar.border.get_or_insert_default().color = Some(c);
    v.sidebar.border.get_or_insert_default().line_width = Some(1.0);

    // toolbar
    v.toolbar.background_color = Some(c);
    v.toolbar.bar_height = Some(40.0);
    v.toolbar.item_gap = Some(4.0);
    v.toolbar.icon_size = Some(24.0);
    v.toolbar.border.get_or_insert_default().color = Some(c);
    v.toolbar.border.get_or_insert_default().corner_radius = Some(4.0);
    v.toolbar.border.get_or_insert_default().line_width = Some(1.0);
    v.toolbar.border.get_or_insert_default().shadow_enabled = Some(false);
    v.toolbar.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // status_bar
    v.status_bar.background_color = Some(c);
    v.status_bar.border.get_or_insert_default().color = Some(c);
    v.status_bar.border.get_or_insert_default().line_width = Some(1.0);
    v.status_bar.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // list
    v.list.background_color = Some(c);
    v.list.alternate_row_background = Some(c);
    v.list.selection_background = Some(c);
    v.list.selection_text_color = Some(c);
    v.list.header_background = Some(c);
    v.list.grid_color = Some(c);
    v.list.row_height = Some(28.0);
    v.list.hover_background = Some(c);
    v.list.hover_text_color = Some(c);
    v.list.disabled_text_color = Some(c);
    v.list.item_font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.list.header_font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.list.border.get_or_insert_default().color = Some(c);
    v.list.border.get_or_insert_default().corner_radius = Some(4.0);
    v.list.border.get_or_insert_default().line_width = Some(1.0);
    v.list.border.get_or_insert_default().shadow_enabled = Some(false);

    // popover
    v.popover.background_color = Some(c);
    v.popover.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.popover.border.get_or_insert_default().color = Some(c);
    v.popover.border.get_or_insert_default().corner_radius = Some(6.0);
    v.popover.border.get_or_insert_default().line_width = Some(1.0);
    v.popover.border.get_or_insert_default().shadow_enabled = Some(false);

    // splitter
    v.splitter.divider_width = Some(4.0);
    v.splitter.divider_color = Some(c);
    v.splitter.hover_color = Some(c);

    // separator
    v.separator.line_color = Some(c);
    v.separator.line_width = Some(1.0);

    // switch
    v.switch.checked_background = Some(c);
    v.switch.unchecked_background = Some(c);
    v.switch.thumb_background = Some(c);
    v.switch.track_width = Some(40.0);
    v.switch.track_height = Some(20.0);
    v.switch.thumb_diameter = Some(14.0);
    v.switch.track_radius = Some(10.0);
    v.switch.disabled_opacity = Some(0.5);
    v.switch.hover_checked_background = Some(c);
    v.switch.hover_unchecked_background = Some(c);
    v.switch.disabled_checked_background = Some(c);
    v.switch.disabled_unchecked_background = Some(c);
    v.switch.disabled_thumb_color = Some(c);

    // dialog
    v.dialog.background_color = Some(c);
    v.dialog.min_width = Some(320.0);
    v.dialog.max_width = Some(600.0);
    v.dialog.min_height = Some(200.0);
    v.dialog.max_height = Some(800.0);
    v.dialog.button_gap = Some(8.0);
    v.dialog.border.get_or_insert_default().color = Some(c);
    v.dialog.border.get_or_insert_default().corner_radius = Some(8.0);
    v.dialog.border.get_or_insert_default().line_width = Some(1.0);
    v.dialog.border.get_or_insert_default().shadow_enabled = Some(true);
    v.dialog.icon_size = Some(22.0);
    v.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);
    v.dialog.title_font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(16.0)),
        weight: Some(700),
        ..Default::default()
    });
    v.dialog.body_font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });

    // spinner
    v.spinner.fill_color = Some(c);
    v.spinner.diameter = Some(24.0);
    v.spinner.min_diameter = Some(16.0);
    v.spinner.stroke_width = Some(2.0);

    // combo_box
    v.combo_box.background_color = Some(c);
    v.combo_box.min_height = Some(28.0);
    v.combo_box.min_width = Some(80.0);
    v.combo_box.arrow_icon_size = Some(12.0);
    v.combo_box.arrow_area_width = Some(20.0);
    v.combo_box.disabled_opacity = Some(0.5);
    v.combo_box.disabled_text_color = Some(c);
    v.combo_box.hover_background = Some(c);
    v.combo_box.disabled_background = Some(c);
    v.combo_box.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.combo_box.border.get_or_insert_default().color = Some(c);
    v.combo_box.border.get_or_insert_default().corner_radius = Some(4.0);
    v.combo_box.border.get_or_insert_default().line_width = Some(1.0);
    v.combo_box.border.get_or_insert_default().shadow_enabled = Some(false);

    // segmented_control
    v.segmented_control.background_color = Some(c);
    v.segmented_control.active_background = Some(c);
    v.segmented_control.active_text_color = Some(c);
    v.segmented_control.segment_height = Some(28.0);
    v.segmented_control.separator_width = Some(1.0);
    v.segmented_control.disabled_opacity = Some(0.5);
    v.segmented_control.hover_background = Some(c);
    v.segmented_control.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.segmented_control.border.get_or_insert_default().color = Some(c);
    v.segmented_control
        .border
        .get_or_insert_default()
        .corner_radius = Some(4.0);
    v.segmented_control
        .border
        .get_or_insert_default()
        .line_width = Some(1.0);
    v.segmented_control
        .border
        .get_or_insert_default()
        .shadow_enabled = Some(false);

    // card
    v.card.background_color = Some(c);
    v.card.border.get_or_insert_default().color = Some(c);
    v.card.border.get_or_insert_default().corner_radius = Some(8.0);
    v.card.border.get_or_insert_default().shadow_enabled = Some(true);

    // expander
    v.expander.header_height = Some(32.0);
    v.expander.arrow_icon_size = Some(12.0);
    v.expander.hover_background = Some(c);
    v.expander.arrow_color = Some(c);
    v.expander.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.expander.border.get_or_insert_default().color = Some(c);
    v.expander.border.get_or_insert_default().corner_radius = Some(4.0);
    v.expander.border.get_or_insert_default().line_width = Some(1.0);
    v.expander.border.get_or_insert_default().shadow_enabled = Some(false);

    // link
    v.link.font = Some(FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Px(14.0)),
        weight: Some(400),
        ..Default::default()
    });
    v.link.visited_text_color = Some(c);
    v.link.background_color = Some(c);
    v.link.hover_background = Some(c);
    v.link.hover_text_color = Some(c);
    v.link.active_text_color = Some(c);
    v.link.disabled_text_color = Some(c);
    v.link.underline_enabled = Some(true);

    // text_scale (all 4 entries fully populated)
    v.text_scale.caption = Some(crate::model::TextScaleEntry {
        size: Some(FontSize::Px(11.0)),
        weight: Some(400),
        line_height: Some(FontSize::Px(15.4)),
    });
    v.text_scale.section_heading = Some(crate::model::TextScaleEntry {
        size: Some(FontSize::Px(14.0)),
        weight: Some(600),
        line_height: Some(FontSize::Px(19.6)),
    });
    v.text_scale.dialog_title = Some(crate::model::TextScaleEntry {
        size: Some(FontSize::Px(16.0)),
        weight: Some(700),
        line_height: Some(FontSize::Px(22.4)),
    });
    v.text_scale.display = Some(crate::model::TextScaleEntry {
        size: Some(FontSize::Px(24.0)),
        weight: Some(300),
        line_height: Some(FontSize::Px(33.6)),
    });

    // Font color on all widget fonts (require_font/require_font_opt now requires color)
    v.window.title_bar_font.get_or_insert_default().color = Some(c);
    v.button.font.get_or_insert_default().color = Some(c);
    v.input.font.get_or_insert_default().color = Some(c);
    v.checkbox.font.get_or_insert_default().color = Some(c);
    v.menu.font.get_or_insert_default().color = Some(c);
    v.tooltip.font.get_or_insert_default().color = Some(c);
    v.tab.font.get_or_insert_default().color = Some(c);
    v.sidebar.font.get_or_insert_default().color = Some(c);
    v.toolbar.font.get_or_insert_default().color = Some(c);
    v.status_bar.font.get_or_insert_default().color = Some(c);
    v.list.item_font.get_or_insert_default().color = Some(c);
    v.list.header_font.get_or_insert_default().color = Some(c);
    v.popover.font.get_or_insert_default().color = Some(c);
    v.dialog.title_font.get_or_insert_default().color = Some(c);
    v.dialog.body_font.get_or_insert_default().color = Some(c);
    v.link.font.get_or_insert_default().color = Some(c);
    v.combo_box.font.get_or_insert_default().color = Some(c);
    v.segmented_control.font.get_or_insert_default().color = Some(c);
    v.expander.font.get_or_insert_default().color = Some(c);

    // Border sub-fields that require_border needs but weren't set above
    v.checkbox.border.get_or_insert_default().color = Some(c);
    v.checkbox.border.get_or_insert_default().shadow_enabled = Some(false);
    v.tooltip.border.get_or_insert_default().color = Some(c);

    v
}

#[test]
fn validate_fully_populated_returns_ok() {
    let v = fully_populated_variant();
    let result = v.validate();
    assert!(
        result.is_ok(),
        "validate() should succeed on fully populated variant, got: {:?}",
        result.err()
    );
    let resolved = result.unwrap();
    assert_eq!(resolved.defaults.font.family, "Inter");
    assert_eq!(resolved.icon_set, crate::IconSet::Freedesktop);
}

#[test]
fn validate_missing_3_fields_returns_all_paths() {
    let mut v = fully_populated_variant();
    // Remove 3 specific scalar fields (non-cascading)
    v.defaults.muted_color = None;
    v.defaults.link_color = None;
    v.icon_set = None;

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionIncomplete { missing } = result.unwrap_err() else {
        // validate() always returns ResolutionIncomplete for missing fields
        return;
    };
    assert_eq!(
        missing.len(),
        3,
        "should report exactly 3 missing fields, got: {:?}",
        missing
    );
    assert!(missing.contains(&"defaults.muted_color".to_string()));
    assert!(missing.contains(&"defaults.link_color".to_string()));
    assert!(missing.contains(&"icon_set".to_string()));
}

#[test]
fn validate_error_message_includes_count_and_paths() {
    let mut v = fully_populated_variant();
    v.defaults.muted_color = None;
    v.button.min_height = None;

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionIncomplete { missing } = result.unwrap_err() else {
        return;
    };
    // ResolutionIncomplete Display includes count and paths
    let err = crate::Error::ResolutionIncomplete {
        missing: missing.clone(),
    };
    let msg = err.to_string();
    assert!(msg.contains("2 missing field(s)"), "got: {msg}");
    assert!(msg.contains("defaults.muted_color"), "got: {msg}");
    assert!(msg.contains("button.min_height"), "got: {msg}");
}

#[test]
fn validate_checks_all_defaults_fields() {
    // Default variant has ALL fields None, so validate should report many missing
    let v = ThemeMode::default();
    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionIncomplete { missing } = result.unwrap_err() else {
        return;
    };
    // Should include defaults fields
    assert!(
        missing.iter().any(|f| f.starts_with("defaults.")),
        "should include defaults.* fields in missing"
    );
    // Check a representative set of defaults fields
    assert!(missing.contains(&"defaults.font.family".to_string()));
    assert!(missing.contains(&"defaults.background_color".to_string()));
    assert!(missing.contains(&"defaults.accent_color".to_string()));
    assert!(missing.contains(&"defaults.border.corner_radius".to_string()));
    assert!(missing.contains(&"defaults.text_selection_background".to_string()));
    assert!(missing.contains(&"defaults.icon_sizes.toolbar".to_string()));
    assert!(missing.contains(&"defaults.text_scaling_factor".to_string()));
}

#[test]
fn validate_checks_all_widget_structs() {
    let v = ThemeMode::default();
    let result = v.validate();
    let crate::Error::ResolutionIncomplete { missing } = result.unwrap_err() else {
        return;
    };
    // Every widget should have at least one field reported
    for prefix in [
        "window.",
        "button.",
        "input.",
        "checkbox.",
        "menu.",
        "tooltip.",
        "scrollbar.",
        "slider.",
        "progress_bar.",
        "tab.",
        "sidebar.",
        "toolbar.",
        "status_bar.",
        "list.",
        "popover.",
        "splitter.",
        "separator.",
        "switch.",
        "dialog.",
        "spinner.",
        "combo_box.",
        "segmented_control.",
        "card.",
        "expander.",
        "link.",
    ] {
        assert!(
            missing.iter().any(|f| f.starts_with(prefix)),
            "missing fields should include {prefix}* but got: {:?}",
            missing
                .iter()
                .filter(|f| f.starts_with(prefix))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn validate_checks_text_scale_entries() {
    let v = ThemeMode::default();
    let result = v.validate();
    let crate::Error::ResolutionIncomplete { missing } = result.unwrap_err() else {
        return;
    };
    assert!(missing.contains(&"text_scale.caption".to_string()));
    assert!(missing.contains(&"text_scale.section_heading".to_string()));
    assert!(missing.contains(&"text_scale.dialog_title".to_string()));
    assert!(missing.contains(&"text_scale.display".to_string()));
}

#[test]
fn validate_checks_icon_set() {
    let mut v = fully_populated_variant();
    v.icon_set = None;

    let result = v.validate();
    let crate::Error::ResolutionIncomplete { missing } = result.unwrap_err() else {
        return;
    };
    assert!(missing.contains(&"icon_set".to_string()));
}

#[test]
fn validate_after_resolve_succeeds_for_derivable_fields() {
    // Start with defaults populated but widgets empty
    let mut v = variant_with_defaults();
    // Add non-derivable widget sizing fields
    v.icon_set = Some(crate::IconSet::Freedesktop);

    // Non-derivable fields that resolve() cannot fill:
    // button sizing
    v.button.min_width = Some(64.0);
    v.button.min_height = Some(28.0);
    v.button.border.get_or_insert_default().padding_horizontal = Some(12.0);
    v.button.border.get_or_insert_default().padding_vertical = Some(6.0);
    v.button.icon_text_gap = Some(6.0);
    // input sizing
    v.input.min_height = Some(28.0);
    v.input.border.get_or_insert_default().padding_horizontal = Some(8.0);
    v.input.border.get_or_insert_default().padding_vertical = Some(4.0);
    // checkbox sizing
    v.checkbox.indicator_width = Some(18.0);
    v.checkbox.label_gap = Some(6.0);
    // menu sizing
    v.menu.row_height = Some(28.0);
    v.menu.border.get_or_insert_default().padding_horizontal = Some(8.0);
    v.menu.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.menu.icon_text_gap = Some(6.0);
    // tooltip sizing
    v.tooltip.border.get_or_insert_default().padding_horizontal = Some(6.0);
    v.tooltip.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.tooltip.max_width = Some(300.0);
    // scrollbar sizing
    v.scrollbar.groove_width = Some(14.0);
    v.scrollbar.min_thumb_length = Some(20.0);
    v.scrollbar.thumb_width = Some(8.0);
    v.scrollbar.overlay_mode = Some(false);
    // slider sizing
    v.slider.track_height = Some(4.0);
    v.slider.thumb_diameter = Some(16.0);
    v.slider.tick_mark_length = Some(6.0);
    // progress_bar sizing
    v.progress_bar.track_height = Some(6.0);
    v.progress_bar.min_width = Some(100.0);
    // tab sizing
    v.tab.min_width = Some(60.0);
    v.tab.min_height = Some(32.0);
    v.tab.border.get_or_insert_default().padding_horizontal = Some(12.0);
    v.tab.border.get_or_insert_default().padding_vertical = Some(6.0);
    // toolbar sizing
    v.toolbar.bar_height = Some(40.0);
    v.toolbar.item_gap = Some(4.0);
    // REMOVED: toolbar.padding not in new schema
    // list sizing + alternate_row (no longer derived — must be explicit)
    v.list.row_height = Some(28.0);
    v.list.border.get_or_insert_default().padding_horizontal = Some(8.0);
    v.list.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.list.alternate_row_background = Some(Rgba::rgb(245, 245, 245));
    // splitter
    v.splitter.divider_width = Some(4.0);
    // switch sizing
    v.switch.unchecked_background = Some(Rgba::rgb(180, 180, 180));
    v.switch.track_width = Some(40.0);
    v.switch.track_height = Some(20.0);
    v.switch.thumb_diameter = Some(14.0);
    v.switch.track_radius = Some(10.0);
    // dialog sizing
    v.dialog.min_width = Some(320.0);
    v.dialog.max_width = Some(600.0);
    v.dialog.min_height = Some(200.0);
    v.dialog.max_height = Some(800.0);
    // REMOVED: content_padding not in new schema
    v.dialog.button_gap = Some(8.0);
    v.dialog.icon_size = Some(22.0);
    v.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);
    // spinner sizing
    v.spinner.diameter = Some(24.0);
    v.spinner.min_diameter = Some(16.0);
    v.spinner.stroke_width = Some(2.0);
    // combo_box sizing
    v.combo_box.min_height = Some(28.0);
    v.combo_box.min_width = Some(80.0);
    v.combo_box
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(8.0);
    v.combo_box.arrow_icon_size = Some(12.0);
    v.combo_box.arrow_area_width = Some(20.0);
    // segmented_control sizing
    v.segmented_control.segment_height = Some(28.0);
    v.segmented_control.separator_width = Some(1.0);
    v.segmented_control
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(12.0);
    // card
    // REMOVED: card.padding not in new schema
    // expander
    v.expander.header_height = Some(32.0);
    v.expander.arrow_icon_size = Some(12.0);
    // REMOVED: content_padding not in new schema
    // link
    v.link.background_color = Some(Rgba::rgb(255, 255, 255));
    v.link.hover_background = Some(Rgba::rgb(230, 230, 255));
    v.link.underline_enabled = Some(true);

    v.resolve_all();
    let result = v.validate();
    assert!(
        result.is_ok(),
        "validate() should succeed after resolve_all() with all non-derivable fields set, got: {:?}",
        result.err()
    );
}

#[test]
fn test_gnome_resolve_validate() {
    // Simulate GNOME reader pipeline: adwaita base + GNOME reader overlay.
    // On a non-GNOME system, build_gnome_variant() only sets dialog.button_order
    // and icon_set (gsettings calls return None). We simulate the full merge.
    let adwaita = crate::Theme::preset("adwaita").unwrap();

    // Pick dark variant from adwaita (matches GNOME PreferDark path).
    let mut variant = adwaita
        .dark
        .clone()
        .expect("adwaita should have dark variant");

    // Apply what build_gnome_variant() would set.
    variant.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);
    // icon_set comes from gsettings icon-theme; simulate typical GNOME value.
    variant.icon_set = Some(crate::IconSet::Freedesktop);

    // Simulate GNOME reader font output (gsettings font-name on a GNOME system).
    variant.defaults.font = FontSpec {
        family: Some("Cantarell".to_string()),
        size: Some(FontSize::Px(11.0)),
        weight: Some(400),
        ..Default::default()
    };

    variant.resolve_all();
    let resolved = variant.validate().unwrap_or_else(|e| {
        panic!("GNOME resolve/validate pipeline failed: {e}");
    });

    // Spot-check: adwaita-base fields present.
    // Adwaita dark accent is #3584e4 = rgb(53, 132, 228)
    assert_eq!(
        resolved.defaults.accent_color,
        Rgba::rgb(53, 132, 228),
        "accent should be from adwaita preset"
    );
    assert_eq!(
        resolved.defaults.font.family, "Cantarell",
        "font family should be from GNOME reader overlay"
    );
    assert_eq!(
        resolved.dialog.button_order,
        DialogButtonOrder::PrimaryRight,
        "dialog button order should be trailing affirmative for GNOME"
    );
    assert_eq!(
        resolved.icon_set,
        crate::IconSet::Freedesktop,
        "icon_set should be from GNOME reader"
    );
}

// ===== Range validation tests =====

#[test]
fn validate_catches_negative_radius() {
    let mut v = fully_populated_variant();
    v.defaults.border.corner_radius = Some(-5.0);

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.border.corner_radius")),
        "should report negative defaults.border.corner_radius, got: {:?}",
        errors
    );
}

#[test]
fn validate_catches_zero_font_size() {
    let mut v = fully_populated_variant();
    v.defaults.font.size = Some(FontSize::Px(0.0));

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.font.size")),
        "should report zero defaults.font.size, got: {:?}",
        errors
    );
}

#[test]
fn validate_catches_opacity_out_of_range() {
    let mut v = fully_populated_variant();
    v.defaults.disabled_opacity = Some(1.5);
    v.defaults.border.opacity = Some(-0.1);
    v.button.disabled_opacity = Some(3.0);

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.disabled_opacity")),
        "should report out-of-range disabled_opacity, got: {:?}",
        errors
    );
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.border.opacity")),
        "should report out-of-range border_opacity, got: {:?}",
        errors
    );
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("button.disabled_opacity")),
        "should report out-of-range button.disabled_opacity, got: {:?}",
        errors
    );
}

#[test]
fn validate_catches_invalid_font_weight() {
    let mut v = fully_populated_variant();
    v.defaults.font.weight = Some(50); // below 100
    v.defaults.mono_font.weight = Some(1000); // above 900

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.font.weight")
                && (rv.value - 50.0).abs() < f64::EPSILON),
        "should report out-of-range font weight 50, got: {:?}",
        errors
    );
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.mono_font.weight")
                && (rv.value - 1000.0).abs() < f64::EPSILON),
        "should report out-of-range mono_font weight 1000, got: {:?}",
        errors
    );
}

#[test]
fn validate_reports_multiple_range_errors_together() {
    let mut v = fully_populated_variant();
    v.defaults.border.corner_radius = Some(-1.0);
    v.defaults.disabled_opacity = Some(2.0);
    v.defaults.font.size = Some(FontSize::Px(0.0));
    v.defaults.font.weight = Some(50);

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    // All 4 range errors should be reported in one batch
    assert!(
        errors.len() >= 4,
        "should report at least 4 range errors, got {}: {:?}",
        errors.len(),
        errors
    );
}

#[test]
fn validate_allows_zero_radius_and_frame_width() {
    // Zero is valid for these fields (flat design, no border)
    let mut v = fully_populated_variant();
    v.defaults.border.corner_radius = Some(0.0);
    v.defaults.border.corner_radius_lg = Some(0.0);
    v.defaults.border.line_width = Some(0.0);
    v.button.border.get_or_insert_default().corner_radius = Some(0.0);
    v.defaults.disabled_opacity = Some(0.0);
    v.defaults.border.opacity = Some(0.0);

    let result = v.validate();
    assert!(
        result.is_ok(),
        "zero values should be valid for radius/frame_width/opacity, got: {:?}",
        result.err()
    );
}

// ===== Additional range-check negative tests (issue 2a) =====

#[test]
fn validate_catches_negative_font_size() {
    let mut v = fully_populated_variant();
    v.defaults.font.size = Some(FontSize::Px(-1.0));

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.font.size")),
        "should report negative font.size, got: {:?}",
        errors
    );
}

#[test]
fn validate_catches_disabled_opacity_above_one() {
    let mut v = fully_populated_variant();
    v.defaults.disabled_opacity = Some(2.0);

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.disabled_opacity")),
        "should report disabled_opacity=2.0 out of 0..=1 range, got: {:?}",
        errors
    );
}

#[test]
fn validate_catches_font_weight_zero() {
    let mut v = fully_populated_variant();
    v.defaults.font.weight = Some(0);

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.font.weight")),
        "should report font.weight=0 out of 100..=900 range, got: {:?}",
        errors
    );
}

#[test]
fn validate_catches_nan_values() {
    let mut v = fully_populated_variant();
    v.defaults.border.corner_radius = Some(f32::NAN);

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.border.corner_radius")),
        "should report NaN defaults.border.corner_radius, got: {:?}",
        errors
    );
}

#[test]
fn validate_catches_infinity() {
    let mut v = fully_populated_variant();
    v.defaults.font.size = Some(FontSize::Px(f32::INFINITY));

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.font.size")),
        "should report infinite font.size, got: {:?}",
        errors
    );
}

#[test]
fn validate_catches_negative_infinity() {
    let mut v = fully_populated_variant();
    // Set a field to negative infinity to trigger range check failure
    v.defaults.border.line_width = Some(f32::NEG_INFINITY);

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.border.line_width")),
        "should report -inf border.line_width, got: {:?}",
        errors
    );
}

// ===== Validation split tests (BUG-01, BUG-02) =====

#[test]
fn validate_missing_field_short_circuits_before_range_checks() {
    // BUG-01 fix: when a field is missing AND another field is out-of-range,
    // validate() must return ResolutionIncomplete (not ResolutionInvalid)
    // because the short-circuit fires before check_ranges runs.
    let mut v = fully_populated_variant();
    v.defaults.accent_color = None; // missing field
    v.defaults.font.weight = Some(0); // out of range (100..=900)

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionIncomplete { missing } = result.unwrap_err() else {
        return;
    };
    assert!(
        missing.contains(&"defaults.accent_color".to_string()),
        "missing should contain defaults.accent_color, got: {missing:?}"
    );
}

#[test]
fn validate_range_only_errors_produce_resolution_invalid() {
    // BUG-02 fix: when all fields are present but one is out-of-range,
    // validate() returns ResolutionInvalid with structured RangeViolation.
    let mut v = fully_populated_variant();
    v.defaults.font.weight = Some(0); // out of range (100..=900)

    let result = v.validate();
    assert!(result.is_err());
    let crate::Error::ResolutionInvalid { errors } = result.unwrap_err() else {
        return;
    };
    assert!(
        errors
            .iter()
            .any(|rv| rv.path.contains("defaults.font.weight")),
        "should contain RangeViolation for defaults.font.weight, got: {errors:?}"
    );
}

// ===== Derivation chain tests (issues 17a, 17b, 17c) =====

#[test]
fn merge_preserves_base_name_when_overlay_name_empty() {
    let mut base = crate::Theme::new("My Base");
    let overlay = crate::Theme::new("");
    base.merge(&overlay);
    assert_eq!(base.name, "My Base", "base name should be preserved");
}

#[test]
fn merge_preserves_empty_base_name_over_nonempty_overlay() {
    // Issue 17a edge case: merge() never touches self.name, so an empty
    // base name is kept even when the overlay has a non-empty name.
    let mut base = crate::Theme::new("");
    let overlay = crate::Theme::new("Overlay Name");
    base.merge(&overlay);
    assert_eq!(
        base.name, "",
        "empty base name should be preserved (merge never touches name)"
    );
}

#[test]
fn accent_derives_selection_then_selection_inactive() {
    let mut v = ThemeMode::default();
    let accent = Rgba::rgb(0, 120, 215);
    v.defaults.accent_color = Some(accent);
    v.resolve();

    // accent -> selection -> selection_inactive chain
    assert_eq!(
        v.defaults.selection_background,
        Some(accent),
        "selection should derive from accent"
    );
    assert_eq!(
        v.defaults.selection_inactive_background,
        Some(accent),
        "selection_inactive should derive from selection"
    );
}

#[test]
fn title_bar_background_inherits_from_surface_not_background() {
    let mut v = ThemeMode::default();
    v.defaults.surface_color = Some(Rgba::rgb(240, 240, 240));
    v.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
    v.resolve();

    assert_eq!(
        v.window.title_bar_background,
        Some(Rgba::rgb(240, 240, 240)),
        "title_bar_background should inherit from surface, not background"
    );
    assert_ne!(
        v.window.title_bar_background, v.defaults.background_color,
        "title_bar_background must not equal background"
    );
}

// ===== Resolve completeness test =====

/// Verify that resolve() has rules for every derived field.
///
/// Constructs a ThemeMode with ONLY root fields (the ~46 defaults
/// Helper: populate all non-derivable widget geometry/behavior fields.
///
/// These fields have no resolve() rule; they MUST be set explicitly.
fn set_widget_geometry(v: &mut ThemeMode) {
    v.icon_set = Some(crate::IconSet::Freedesktop);
    // button
    v.button.min_width = Some(64.0);
    v.button.min_height = Some(28.0);
    v.button.border.get_or_insert_default().padding_horizontal = Some(12.0);
    v.button.border.get_or_insert_default().padding_vertical = Some(6.0);
    v.button.icon_text_gap = Some(6.0);
    // input
    v.input.min_height = Some(28.0);
    v.input.border.get_or_insert_default().padding_horizontal = Some(8.0);
    v.input.border.get_or_insert_default().padding_vertical = Some(4.0);
    // checkbox
    v.checkbox.indicator_width = Some(18.0);
    v.checkbox.label_gap = Some(6.0);
    // menu
    v.menu.row_height = Some(28.0);
    v.menu.border.get_or_insert_default().padding_horizontal = Some(8.0);
    v.menu.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.menu.icon_text_gap = Some(6.0);
    // tooltip
    v.tooltip.border.get_or_insert_default().padding_horizontal = Some(6.0);
    v.tooltip.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.tooltip.max_width = Some(300.0);
    // scrollbar
    v.scrollbar.groove_width = Some(14.0);
    v.scrollbar.min_thumb_length = Some(20.0);
    v.scrollbar.thumb_width = Some(8.0);
    v.scrollbar.overlay_mode = Some(false);
    // slider
    v.slider.track_height = Some(4.0);
    v.slider.thumb_diameter = Some(16.0);
    v.slider.tick_mark_length = Some(6.0);
    // progress_bar
    v.progress_bar.track_height = Some(6.0);
    v.progress_bar.min_width = Some(100.0);
    // tab
    v.tab.min_width = Some(60.0);
    v.tab.min_height = Some(32.0);
    v.tab.border.get_or_insert_default().padding_horizontal = Some(12.0);
    v.tab.border.get_or_insert_default().padding_vertical = Some(6.0);
    // toolbar
    v.toolbar.bar_height = Some(40.0);
    v.toolbar.item_gap = Some(4.0);
    // REMOVED: toolbar.padding not in new schema
    // list (alternate_row_background has no inheritance -- must be preset-provided)
    v.list.row_height = Some(28.0);
    v.list.border.get_or_insert_default().padding_horizontal = Some(8.0);
    v.list.border.get_or_insert_default().padding_vertical = Some(4.0);
    v.list.alternate_row_background = Some(Rgba::rgb(245, 245, 245));
    // splitter
    v.splitter.divider_width = Some(4.0);
    // switch (unchecked_background has no inheritance -- must be preset-provided)
    v.switch.unchecked_background = Some(Rgba::rgb(180, 180, 180));
    v.switch.track_width = Some(40.0);
    v.switch.track_height = Some(20.0);
    v.switch.thumb_diameter = Some(14.0);
    v.switch.track_radius = Some(10.0);
    // card (border sub-fields have no inheritance -- must be preset-provided)
    v.card.border.get_or_insert_default().color = Some(Rgba::rgb(200, 200, 200));
    v.card.border.get_or_insert_default().corner_radius = Some(8.0);
    v.card.border.get_or_insert_default().shadow_enabled = Some(true);
    // dialog
    v.dialog.min_width = Some(320.0);
    v.dialog.max_width = Some(600.0);
    v.dialog.min_height = Some(200.0);
    v.dialog.max_height = Some(800.0);
    // REMOVED: content_padding not in new schema
    v.dialog.button_gap = Some(8.0);
    v.dialog.icon_size = Some(22.0);
    v.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);
    // spinner
    v.spinner.diameter = Some(24.0);
    v.spinner.min_diameter = Some(16.0);
    v.spinner.stroke_width = Some(2.0);
    // combo_box
    v.combo_box.min_height = Some(28.0);
    v.combo_box.min_width = Some(80.0);
    v.combo_box
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(8.0);
    v.combo_box.arrow_icon_size = Some(12.0);
    v.combo_box.arrow_area_width = Some(20.0);
    // segmented_control
    v.segmented_control.segment_height = Some(28.0);
    v.segmented_control.separator_width = Some(1.0);
    v.segmented_control
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(12.0);
    // card
    // REMOVED: card.padding not in new schema
    // expander
    v.expander.header_height = Some(32.0);
    v.expander.arrow_icon_size = Some(12.0);
    // REMOVED: content_padding not in new schema
    // link (background and hover_bg have no derivation path)
    v.link.background_color = Some(Rgba::rgb(255, 255, 255));
    v.link.hover_background = Some(Rgba::rgb(230, 230, 255));
    v.link.underline_enabled = Some(true);
}

/// Helper: clear all derived color/font/text_scale fields on a variant.
///
/// After calling this, resolve_all() must be able to reconstruct every
/// cleared field from the remaining defaults.
fn clear_derived_fields(v: &mut ThemeMode) {
    // Widget colors and radii (derived from defaults)
    v.window.background_color = None;
    v.window.title_bar_font = None;
    v.window.border = None;
    v.window.title_bar_background = None;
    v.window.title_bar_font = None;
    v.window.inactive_title_bar_background = None;
    v.window.inactive_title_bar_text_color = None;
    v.window.border = None;
    v.window.border = None;
    v.button.background_color = None;
    v.button.font = None;
    v.button.border = None;
    v.button.primary_background = None;
    v.button.primary_text_color = None;
    v.button.border = None;
    v.button.disabled_opacity = None;
    v.button.active_text_color = None;
    v.button.disabled_text_color = None;
    v.button.border = None;
    v.input.background_color = None;
    v.input.font = None;
    v.input.border = None;
    v.input.placeholder_color = None;
    v.input.caret_color = None;
    v.input.selection_background = None;
    v.input.selection_text_color = None;
    v.input.disabled_text_color = None;
    v.input.border = None;
    v.checkbox.checked_background = None;
    v.checkbox.disabled_text_color = None;
    v.checkbox.border = None;
    v.menu.background_color = None;
    v.menu.font = None;
    v.menu.separator_color = None;
    v.tooltip.background_color = None;
    v.tooltip.font = None;
    v.tooltip.border = None;
    v.scrollbar.track_color = None;
    v.scrollbar.thumb_color = None;
    v.scrollbar.thumb_hover_color = None;
    v.slider.fill_color = None;
    v.slider.track_color = None;
    v.slider.thumb_color = None;
    v.progress_bar.fill_color = None;
    v.progress_bar.track_color = None;
    v.progress_bar.border = None;
    v.tab.background_color = None;
    v.tab.font = None;
    v.tab.active_background = None;
    v.tab.active_text_color = None;
    v.tab.bar_background = None;
    v.tab.hover_text_color = None;
    v.sidebar.background_color = None;
    v.sidebar.font = None;
    v.list.background_color = None;
    v.list.item_font = None;
    // list.alternate_row_background: NOT cleared — it is no longer derived,
    // so presets must provide it explicitly.
    v.list.selection_background = None;
    v.list.selection_text_color = None;
    v.list.header_background = None;
    v.list.header_font = None;
    v.list.grid_color = None;
    v.list.hover_text_color = None;
    v.list.disabled_text_color = None;
    v.popover.background_color = None;
    v.popover.font = None;
    v.popover.border = None;
    v.separator.line_color = None;
    v.splitter.hover_color = None;
    v.switch.checked_background = None;
    // switch.unchecked_background: NOT cleared -- no inheritance, must come from preset
    v.switch.thumb_background = None;
    v.dialog.border = None;
    v.combo_box.border = None;
    v.combo_box.disabled_text_color = None;
    v.segmented_control.border = None;
    v.card.background_color = None;
    // card.border: NOT cleared -- no inheritance from defaults (INH-3), must come from preset
    v.expander.border = None;
    v.link.font = None;
    v.link.visited_text_color = None;
    v.link.hover_text_color = None;
    v.link.active_text_color = None;
    v.link.disabled_text_color = None;
    v.spinner.fill_color = None;
    // Widget fonts (derived from defaults.font)
    v.window.title_bar_font = None;
    v.button.font = None;
    v.input.font = None;
    v.menu.font = None;
    v.tooltip.font = None;
    v.toolbar.font = None;
    v.status_bar.font = None;
    v.dialog.title_font = None;
    // Text scale (derived from defaults.font + defaults.line_height)
    v.text_scale.caption = None;
    v.text_scale.section_heading = None;
    v.text_scale.dialog_title = None;
    v.text_scale.display = None;
    // Defaults internal chains (derived from accent/selection)
    v.defaults.selection_background = None;
    v.defaults.focus_ring_color = None;
    v.defaults.selection_inactive_background = None;
}

/// fields + ~65 widget geometry/behavior fields that have no derivation
/// path in resolve()). Derived fields (widget colors, widget fonts, text
/// scale entries, widget-to-widget chains) are left as None.
///
/// If any derived field lacks a resolve rule, it stays None and
/// validate() reports it as missing -- catching the bug.
#[test]
fn resolve_completeness_minimal_variant() {
    let mut v = variant_with_defaults();
    set_widget_geometry(&mut v);

    // Verify: NO derived color/font/text_scale fields are set
    assert!(
        v.window.background_color.is_none(),
        "window.background should be None before resolve"
    );
    assert!(
        v.button.background_color.is_none(),
        "button.background should be None before resolve"
    );
    assert!(
        v.button.font.is_none(),
        "button.font should be None before resolve"
    );
    assert!(
        v.text_scale.caption.is_none(),
        "text_scale.caption should be None before resolve"
    );

    v.resolve_all();
    let result = v.validate();
    assert!(
        result.is_ok(),
        "Resolve completeness failed -- some derived fields lack resolve rules: {:?}",
        result.err()
    );
}

/// Cross-check: verify completeness by stripping derived fields from a preset.
///
/// Loads a known preset, clears all derived color/font/text_scale fields,
/// then verifies resolve() can reconstruct them.
#[test]
fn resolve_completeness_from_preset() {
    let spec = crate::Theme::preset("material").unwrap();
    let mut v = spec.dark.expect("material should have dark variant");

    clear_derived_fields(&mut v);

    v.resolve_all();
    let result = v.validate();
    assert!(
        result.is_ok(),
        "Resolve completeness from preset failed -- some derived fields lack resolve rules: {:?}",
        result.err()
    );
}

#[test]
fn validate_all_presets_pass_range_checks() {
    // Verify no false positives: all 16 presets pass validation including range checks
    let names = crate::Theme::list_presets();
    assert!(names.len() >= 16, "expected at least 16 presets");

    for name in names {
        let spec = crate::Theme::preset(name).unwrap();
        if let Some(light) = spec.light {
            let resolved = light.into_resolved();
            assert!(
                resolved.is_ok(),
                "preset '{name}' light variant failed: {:?}",
                resolved.err()
            );
        }
        if let Some(dark) = spec.dark {
            let resolved = dark.into_resolved();
            assert!(
                resolved.is_ok(),
                "preset '{name}' dark variant failed: {:?}",
                resolved.err()
            );
        }
    }
}
