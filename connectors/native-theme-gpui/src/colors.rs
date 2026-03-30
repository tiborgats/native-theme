//! ResolvedThemeVariant -> gpui_component::theme::ThemeColor mapping (108 fields).
//!
//! Maps native-theme's per-widget resolved fields to gpui-component's 108-field
//! ThemeColor struct. Direct mappings cover ~40 fields; the remaining ~68 are
//! derived via shade generation, blending, or fallback logic that mirrors
//! gpui-component's own `apply_config` derivation.

use gpui::Hsla;
use gpui_component::theme::ThemeColor;
use native_theme::ResolvedThemeVariant;

use crate::derive::{active_color, hover_color};

/// Convert a `native_theme::Rgba` to `gpui::Hsla`.
fn rgba_to_hsla(rgba: native_theme::Rgba) -> Hsla {
    let [r, g, b, a] = rgba.to_f32_array();
    let gpui_rgba = gpui::Rgba { r, g, b, a };
    gpui_rgba.into()
}

/// Returns true if the background color indicates a dark theme (lightness < 0.5).
#[cfg(test)]
fn is_dark_background(bg: Hsla) -> bool {
    bg.l < 0.5
}

/// Pre-converted HSLA colors extracted from a [`ResolvedThemeVariant`].
///
/// Built once in [`to_theme_color()`] and passed by reference to all
/// assign helper functions, replacing 10-15 parameter signatures.
///
/// Some fields (e.g. `surface`) are not currently consumed by any assign
/// function but are retained for future mapping completeness.
#[allow(dead_code)]
struct ResolvedColors {
    bg: Hsla,
    fg: Hsla,
    accent: Hsla,
    accent_fg: Hsla,
    surface: Hsla,
    border: Hsla,
    muted: Hsla,
    muted_fg: Hsla,
    primary: Hsla,
    primary_fg: Hsla,
    secondary: Hsla,
    secondary_fg: Hsla,
    danger: Hsla,
    danger_fg: Hsla,
    success: Hsla,
    success_fg: Hsla,
    warning: Hsla,
    warning_fg: Hsla,
    info: Hsla,
    info_fg: Hsla,
    selection: Hsla,
    link: Hsla,
    ring: Hsla,
    input: Hsla,
    sidebar: Hsla,
    sidebar_fg: Hsla,
    popover: Hsla,
    popover_fg: Hsla,
    alternate_row: Hsla,
}

/// Build a complete [`ThemeColor`] from a [`ResolvedThemeVariant`].
///
/// Maps all 108 fields: ~40 directly from ResolvedThemeVariant per-widget structs,
/// the rest derived via shade generation following gpui-component's own
/// fallback logic.
///
/// The `is_dark` parameter controls both `ThemeMode` (set in [`crate::to_theme`])
/// and active-state darkening amounts — a single source of truth that prevents
/// the split-brain bug where `ThemeMode` and color derivation could disagree.
pub fn to_theme_color(resolved: &ResolvedThemeVariant, is_dark: bool) -> ThemeColor {
    let d = &resolved.defaults;
    let fg = rgba_to_hsla(d.foreground);

    let c = ResolvedColors {
        bg: rgba_to_hsla(d.background),
        fg,
        accent: rgba_to_hsla(d.accent),
        accent_fg: rgba_to_hsla(d.accent_foreground),
        surface: rgba_to_hsla(d.surface),
        border: rgba_to_hsla(d.border),
        muted: rgba_to_hsla(d.muted),
        muted_fg: rgba_to_hsla(d.muted).blend(fg.opacity(0.7)),
        primary: rgba_to_hsla(resolved.button.primary_bg),
        primary_fg: rgba_to_hsla(resolved.button.primary_fg),
        secondary: rgba_to_hsla(resolved.button.background),
        secondary_fg: rgba_to_hsla(resolved.button.foreground),
        danger: rgba_to_hsla(d.danger),
        danger_fg: rgba_to_hsla(d.danger_foreground),
        success: rgba_to_hsla(d.success),
        success_fg: rgba_to_hsla(d.success_foreground),
        warning: rgba_to_hsla(d.warning),
        warning_fg: rgba_to_hsla(d.warning_foreground),
        info: rgba_to_hsla(d.info),
        info_fg: rgba_to_hsla(d.info_foreground),
        selection: rgba_to_hsla(d.selection),
        link: rgba_to_hsla(d.link),
        ring: rgba_to_hsla(d.focus_ring_color),
        input: rgba_to_hsla(resolved.input.border),
        sidebar: rgba_to_hsla(resolved.sidebar.background),
        sidebar_fg: rgba_to_hsla(resolved.sidebar.foreground),
        popover: rgba_to_hsla(resolved.popover.background),
        popover_fg: rgba_to_hsla(resolved.popover.foreground),
        alternate_row: rgba_to_hsla(resolved.list.alternate_row),
    };

    let mut tc = ThemeColor::default();

    assign_core(&mut tc, &c, is_dark);
    assign_primary(&mut tc, &c, is_dark);
    assign_secondary(&mut tc, &c, is_dark);
    assign_status(&mut tc, &c, is_dark);
    assign_list_table(&mut tc, &c);
    assign_tab_sidebar(&mut tc, &c, resolved);
    assign_charts(&mut tc, &c);
    assign_misc(&mut tc, &c, resolved, is_dark);
    assign_base_colors(&mut tc, &c);

    tc
}

// ---------- helper assignment functions ----------

fn assign_core(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
    tc.background = c.bg;
    tc.foreground = c.fg;
    tc.accent = c.accent;
    tc.accent_foreground = c.accent_fg;
    tc.border = c.border;
    tc.muted = c.muted;
    tc.muted_foreground = c.muted_fg;
    tc.input = c.input;
    tc.ring = c.ring;
    tc.selection = c.selection;
    tc.link = c.link;
    tc.link_hover = hover_color(c.link, c.bg);
    tc.link_active = active_color(c.link, is_dark);
}

fn assign_primary(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
    tc.primary = c.primary;
    tc.primary_foreground = c.primary_fg;
    tc.primary_hover = hover_color(c.primary, c.bg);
    tc.primary_active = active_color(c.primary, is_dark);
}

fn assign_secondary(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
    tc.secondary = c.secondary;
    tc.secondary_foreground = c.secondary_fg;
    tc.secondary_hover = hover_color(c.secondary, c.bg);
    tc.secondary_active = active_color(c.secondary, is_dark);
}

fn assign_status(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
    tc.danger = c.danger;
    tc.danger_foreground = c.danger_fg;
    tc.danger_hover = hover_color(c.danger, c.bg);
    tc.danger_active = active_color(c.danger, is_dark);

    tc.success = c.success;
    tc.success_foreground = c.success_fg;
    tc.success_hover = hover_color(c.success, c.bg);
    tc.success_active = active_color(c.success, is_dark);

    tc.warning = c.warning;
    tc.warning_foreground = c.warning_fg;
    tc.warning_hover = hover_color(c.warning, c.bg);
    tc.warning_active = active_color(c.warning, is_dark);

    tc.info = c.info;
    tc.info_foreground = c.info_fg;
    tc.info_hover = hover_color(c.info, c.bg);
    tc.info_active = active_color(c.info, is_dark);

    tc.bullish = c.success;
    tc.bearish = c.danger;
}

fn assign_list_table(tc: &mut ThemeColor, c: &ResolvedColors) {
    tc.list = c.bg;
    tc.list_hover = hover_color(c.secondary, c.bg);
    tc.list_active = c.bg.blend(c.primary.opacity(0.1)).alpha(0.2);
    tc.list_active_border = c.bg.blend(c.primary.opacity(0.6));
    tc.list_even = c.alternate_row;
    tc.list_head = c.bg;

    tc.table = c.bg;
    tc.table_hover = tc.list_hover;
    tc.table_active = tc.list_active;
    tc.table_active_border = tc.list_active_border;
    tc.table_even = tc.list_even;
    tc.table_head = c.bg;
    tc.table_head_foreground = c.muted_fg;
    tc.table_row_border = c.border;
}

fn assign_tab_sidebar(tc: &mut ThemeColor, c: &ResolvedColors, resolved: &ResolvedThemeVariant) {
    // Tab: use per-widget resolved tab colors
    tc.tab = rgba_to_hsla(resolved.tab.background);
    tc.tab_active = rgba_to_hsla(resolved.tab.active_background);
    tc.tab_active_foreground = rgba_to_hsla(resolved.tab.active_foreground);
    tc.tab_bar = rgba_to_hsla(resolved.tab.bar_background);
    tc.tab_bar_segmented = c.secondary;
    tc.tab_foreground = rgba_to_hsla(resolved.tab.foreground);

    tc.sidebar = c.sidebar;
    tc.sidebar_foreground = c.sidebar_fg;
    tc.sidebar_accent = c.accent;
    tc.sidebar_accent_foreground = c.accent_fg;
    tc.sidebar_border = c.border;
    tc.sidebar_primary = c.primary;
    tc.sidebar_primary_foreground = c.primary_fg;

    // Title bar: use per-widget resolved window colors
    tc.title_bar = rgba_to_hsla(resolved.window.title_bar_background);
    tc.title_bar_border = rgba_to_hsla(resolved.window.border);
    tc.window_border = rgba_to_hsla(resolved.window.border);
}

fn assign_charts(tc: &mut ThemeColor, c: &ResolvedColors) {
    // Distribute 5 chart colors evenly around the hue wheel (~72° apart).
    // Preserves accent's saturation and lightness for palette coherence.
    tc.chart_1 = c.accent;
    tc.chart_2 = Hsla { h: (c.accent.h + 0.2) % 1.0, ..c.accent };
    tc.chart_3 = Hsla { h: (c.accent.h + 0.4) % 1.0, ..c.accent };
    tc.chart_4 = Hsla { h: (c.accent.h + 0.6) % 1.0, ..c.accent };
    tc.chart_5 = Hsla { h: (c.accent.h + 0.8) % 1.0, ..c.accent };
}

fn assign_misc(
    tc: &mut ThemeColor,
    c: &ResolvedColors,
    resolved: &ResolvedThemeVariant,
    is_dark: bool,
) {
    tc.popover = c.popover;
    tc.popover_foreground = c.popover_fg;

    tc.accordion = c.bg;
    tc.accordion_hover = c.accent.opacity(0.8);

    tc.group_box = c.bg.blend(c.secondary.opacity(if is_dark { 0.3 } else { 0.4 }));
    tc.group_box_foreground = c.fg;

    tc.description_list_label = c.bg.blend(c.border.opacity(0.2));
    tc.description_list_label_foreground = tc.muted_foreground;

    // Derive overlay from the theme's shadow color instead of hardcoded black
    let shadow = rgba_to_hsla(resolved.defaults.shadow);
    tc.overlay = Hsla {
        h: shadow.h,
        s: shadow.s,
        l: shadow.l,
        a: if is_dark { 0.5 } else { 0.4 },
    };

    // Per-widget resolved scrollbar colors
    tc.scrollbar = c.bg;
    tc.scrollbar_thumb = rgba_to_hsla(resolved.scrollbar.thumb);
    tc.scrollbar_thumb_hover = rgba_to_hsla(resolved.scrollbar.thumb_hover);

    // Per-widget resolved slider colors
    tc.slider_bar = rgba_to_hsla(resolved.slider.fill);
    tc.slider_thumb = rgba_to_hsla(resolved.slider.thumb);

    // Per-widget resolved switch colors
    tc.switch = rgba_to_hsla(resolved.switch.unchecked_bg);
    tc.switch_thumb = rgba_to_hsla(resolved.switch.thumb_bg);

    // Per-widget resolved progress bar color
    tc.progress_bar = rgba_to_hsla(resolved.progress_bar.fill);

    // Per-widget resolved caret from input
    tc.caret = rgba_to_hsla(resolved.input.caret);

    tc.skeleton = c.secondary;

    tc.tiles = c.bg;

    tc.drag_border = c.primary.opacity(0.65);
    tc.drop_target = c.primary.opacity(0.2);
}

fn assign_base_colors(tc: &mut ThemeColor, c: &ResolvedColors) {
    tc.red = c.danger;
    tc.red_light = c.bg.blend(c.danger.opacity(0.8));
    tc.green = c.success;
    tc.green_light = c.bg.blend(c.success.opacity(0.8));
    tc.blue = c.info;
    tc.blue_light = c.bg.blend(c.info.opacity(0.8));
    tc.yellow = c.warning;
    tc.yellow_light = c.bg.blend(c.warning.opacity(0.8));
    // Magenta: fixed hue, but saturation and lightness from accent
    let magenta = Hsla {
        h: 0.833,
        s: c.accent.s.min(0.85),
        l: c.accent.l,
        a: 1.0,
    };
    tc.magenta = magenta;
    tc.magenta_light = c.bg.blend(magenta.opacity(0.8));
    // Cyan: fixed hue, but saturation and lightness from info
    let cyan = Hsla {
        h: 0.5,
        s: c.info.s.min(0.85),
        l: c.info.l,
        a: 1.0,
    };
    tc.cyan = cyan;
    tc.cyan_light = c.bg.blend(cyan.opacity(0.8));
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::ThemeSpec;

    /// Create a ResolvedThemeVariant via the preset resolve+validate pipeline.
    fn test_resolved() -> ResolvedThemeVariant {
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let mut v = nt
            .pick_variant(false)
            .expect("preset must have light variant")
            .clone();
        v.resolve();
        v.validate().expect("resolved preset must validate")
    }

    #[test]
    fn rgba_to_hsla_converts_red() {
        let red = native_theme::Rgba::rgb(255, 0, 0);
        let result = rgba_to_hsla(red);
        // Red should have hue ~0 (or near 0/1), high saturation
        assert!(
            result.h < 0.05 || result.h > 0.95,
            "red hue={} should be near 0",
            result.h
        );
        assert!(result.s > 0.9, "red saturation={} should be high", result.s);
    }

    #[test]
    fn rgba_to_hsla_converts_green() {
        let green = native_theme::Rgba::rgb(0, 255, 0);
        let result = rgba_to_hsla(green);
        // Green hue ~0.333
        assert!(
            (result.h - 0.333).abs() < 0.05,
            "green hue={} should be near 0.333",
            result.h
        );
    }

    #[test]
    fn rgba_to_hsla_converts_blue() {
        let blue = native_theme::Rgba::rgb(0, 0, 255);
        let result = rgba_to_hsla(blue);
        // Blue hue ~0.667
        assert!(
            (result.h - 0.667).abs() < 0.05,
            "blue hue={} should be near 0.667",
            result.h
        );
    }

    #[test]
    fn to_theme_color_produces_nondefault() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);
        let default = ThemeColor::default();

        // Direct-mapped fields should differ from default
        assert_ne!(
            tc.background, default.background,
            "background should be set"
        );
        assert_ne!(
            tc.foreground, default.foreground,
            "foreground should be set"
        );
        assert_ne!(tc.primary, default.primary, "primary should be set");
        assert_ne!(tc.danger, default.danger, "danger should be set");
        assert_ne!(tc.border, default.border, "border should be set");
    }

    #[test]
    fn is_dark_detects_dark_background() {
        assert!(is_dark_background(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.1,
            a: 1.0
        }));
        assert!(!is_dark_background(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0
        }));
    }

    #[test]
    fn hover_states_differ_from_base() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);

        assert_ne!(
            tc.primary_hover, tc.primary,
            "primary_hover should differ from primary"
        );
        assert_ne!(
            tc.danger_hover, tc.danger,
            "danger_hover should differ from danger"
        );
    }

    #[test]
    fn per_widget_fields_used() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);

        // Scrollbar thumb should match resolved scrollbar
        let expected_thumb = rgba_to_hsla(resolved.scrollbar.thumb);
        assert_eq!(
            tc.scrollbar_thumb, expected_thumb,
            "scrollbar thumb should come from resolved.scrollbar.thumb"
        );

        // Slider bar should match resolved slider fill
        let expected_slider = rgba_to_hsla(resolved.slider.fill);
        assert_eq!(
            tc.slider_bar, expected_slider,
            "slider bar should come from resolved.slider.fill"
        );

        // Progress bar should match resolved progress_bar fill
        let expected_progress = rgba_to_hsla(resolved.progress_bar.fill);
        assert_eq!(
            tc.progress_bar, expected_progress,
            "progress bar should come from resolved.progress_bar.fill"
        );

        // Title bar should match resolved window title_bar_background
        let expected_title = rgba_to_hsla(resolved.window.title_bar_background);
        assert_eq!(
            tc.title_bar, expected_title,
            "title bar should come from resolved.window.title_bar_background"
        );

        // Switch should match resolved switch unchecked_bg
        let expected_switch = rgba_to_hsla(resolved.switch.unchecked_bg);
        assert_eq!(
            tc.switch, expected_switch,
            "switch should come from resolved.switch.unchecked_bg"
        );

        // Caret should match resolved input caret
        let expected_caret = rgba_to_hsla(resolved.input.caret);
        assert_eq!(
            tc.caret, expected_caret,
            "caret should come from resolved.input.caret"
        );
    }

    #[test]
    fn accent_foreground_uses_theme_value() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);

        // accent_foreground should come from d.accent_foreground, not from fg
        let expected = rgba_to_hsla(resolved.defaults.accent_foreground);
        assert_eq!(
            tc.accent_foreground, expected,
            "accent_foreground should map from d.accent_foreground"
        );
        // Verify it's also used for sidebar_accent_foreground
        assert_eq!(
            tc.sidebar_accent_foreground, expected,
            "sidebar_accent_foreground should map from d.accent_foreground"
        );
    }

    #[test]
    fn is_dark_passed_not_derived() {
        let resolved = test_resolved();
        let tc_light = to_theme_color(&resolved, false);
        let tc_dark = to_theme_color(&resolved, true);

        // active_color uses different darkening for light vs dark:
        // light darkens by 10%, dark darkens by 20%.
        // So primary_active should differ between the two.
        assert_ne!(
            tc_light.primary_active, tc_dark.primary_active,
            "primary_active should differ between is_dark=false and is_dark=true"
        );
    }

    #[test]
    fn link_hover_differs_from_link() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);

        assert_ne!(
            tc.link_hover, tc.link,
            "link_hover should differ from link (uses hover_color)"
        );
        assert_ne!(
            tc.link_active, tc.link,
            "link_active should differ from link (uses active_color)"
        );
    }

    #[test]
    fn selection_not_clamped() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);

        // The theme's selection color should be used as-is, not alpha-clamped to 0.3
        let expected = rgba_to_hsla(resolved.defaults.selection);
        assert_eq!(
            tc.selection, expected,
            "selection should use theme value without alpha clamping"
        );
    }

    #[test]
    fn chart_colors_have_hue_separation() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);

        let hues = [tc.chart_1.h, tc.chart_2.h, tc.chart_3.h, tc.chart_4.h, tc.chart_5.h];
        // All 5 hues should be distinct
        for i in 0..5 {
            for j in (i + 1)..5 {
                assert!(
                    (hues[i] - hues[j]).abs() > 0.05,
                    "chart_{} (h={:.3}) and chart_{} (h={:.3}) should have distinct hues",
                    i + 1, hues[i], j + 1, hues[j]
                );
            }
        }
        // chart_1 should be the accent color itself
        let accent = rgba_to_hsla(resolved.defaults.accent);
        assert!(
            (tc.chart_1.h - accent.h).abs() < 0.001,
            "chart_1 hue should match accent hue"
        );
    }

    #[test]
    fn magenta_uses_theme_saturation() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);

        let accent = rgba_to_hsla(resolved.defaults.accent);
        let expected_s = accent.s.min(0.85);
        assert!(
            (tc.magenta.s - expected_s).abs() < 0.001,
            "magenta saturation {:.3} should be min(accent.s, 0.85) = {:.3}",
            tc.magenta.s, expected_s
        );
        assert!(
            (tc.magenta.l - accent.l).abs() < 0.001,
            "magenta lightness {:.3} should match accent lightness {:.3}",
            tc.magenta.l, accent.l
        );
    }

    #[test]
    fn overlay_uses_shadow_color() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, false);

        let shadow = rgba_to_hsla(resolved.defaults.shadow);
        assert!(
            (tc.overlay.h - shadow.h).abs() < 0.001,
            "overlay hue {:.3} should match shadow hue {:.3}",
            tc.overlay.h, shadow.h
        );
        assert!(
            (tc.overlay.s - shadow.s).abs() < 0.001,
            "overlay saturation {:.3} should match shadow saturation {:.3}",
            tc.overlay.s, shadow.s
        );
    }

    #[test]
    fn theme_color_field_count_tripwire() {
        // ThemeColor has N Hsla fields (each 16 bytes = 4x f32).
        // If this fails, gpui-component added/removed fields — update the color mapping.
        let size = std::mem::size_of::<ThemeColor>();
        let hsla_size = std::mem::size_of::<Hsla>();
        let field_count = size / hsla_size;
        assert_eq!(
            field_count, 108,
            "ThemeColor field count changed (got {field_count}) — update color mapping in to_theme_color()"
        );
    }
}
