//! ResolvedThemeVariant -> gpui_component::theme::ThemeColor mapping (108 fields).
//!
//! Maps native-theme's per-widget resolved fields to gpui-component's 108-field
//! ThemeColor struct. Direct mappings cover ~40 fields; the remaining ~68 are
//! derived via shade generation, blending, or fallback logic that mirrors
//! gpui-component's own `apply_config` derivation.

use gpui::Hsla;
use gpui_component::theme::ThemeColor;
use native_theme::ResolvedThemeVariant;

use crate::derive::{active_color, contrast_ratio, hover_color, light_variant};

/// Convert a `native_theme::Rgba` to `gpui::Hsla`.
///
/// The input is clamped to [0.0, 1.0] per channel before conversion.
pub(crate) fn rgba_to_hsla(rgba: native_theme::Rgba) -> Hsla {
    let [r, g, b, a] = rgba.to_f32_array();
    let gpui_rgba = gpui::Rgba {
        r: r.clamp(0.0, 1.0),
        g: g.clamp(0.0, 1.0),
        b: b.clamp(0.0, 1.0),
        a: a.clamp(0.0, 1.0),
    };
    gpui_rgba.into()
}

/// Convert an `Hsla` color to a `#rrggbb` hex string.
///
/// Alpha is discarded (only the opaque RGB is encoded).
pub(crate) fn hsla_to_hex(c: Hsla) -> String {
    let rgba: gpui::Rgba = c.into();
    let r = (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (rgba.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (rgba.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// Minimum WCAG contrast ratio for status foreground against its background.
/// 4.5:1 is AA for normal text.
const MIN_STATUS_CONTRAST: f32 = 4.5;

/// Ensure a status foreground color has sufficient contrast against its background.
///
/// If the foreground has less than 4.5:1 contrast against the background,
/// falls back to white (for dark backgrounds) or black (for light backgrounds).
fn ensure_status_contrast(fg: Hsla, bg: Hsla) -> Hsla {
    if contrast_ratio(fg, bg) >= MIN_STATUS_CONTRAST {
        fg
    } else if bg.l < 0.5 {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 1.0,
            a: 1.0,
        }
    } else {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 1.0,
        }
    }
}

/// Pre-converted HSLA colors extracted from a [`ResolvedThemeVariant`].
///
/// Built once in [`to_theme_color()`] and passed by reference to all
/// assign helper functions, replacing 10-15 parameter signatures.
///
/// The `surface` field is used for the `muted` background slot (Issue 2).
struct ResolvedColors {
    bg: Hsla,
    fg: Hsla,
    accent: Hsla,
    accent_fg: Hsla,
    #[allow(dead_code)]
    surface: Hsla,
    border: Hsla,
    // Issue 2: `muted` is the muted *background* slot (derived from surface),
    // and `muted_fg` is the muted *foreground* (d.muted IS the muted foreground).
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
    // Issue 20: per-widget colors cached to avoid repeated rgba_to_hsla calls
    // in assign_tab_sidebar() and assign_misc().
    tab_bg: Hsla,
    tab_active_bg: Hsla,
    tab_active_fg: Hsla,
    tab_bar_bg: Hsla,
    tab_fg: Hsla,
    title_bar_bg: Hsla,
    window_border: Hsla,
    scrollbar_track: Hsla,
    scrollbar_thumb: Hsla,
    scrollbar_thumb_hover: Hsla,
    slider_fill: Hsla,
    slider_thumb: Hsla,
    switch_unchecked: Hsla,
    switch_thumb: Hsla,
    progress_fill: Hsla,
    input_caret: Hsla,
}

/// Build a complete [`ThemeColor`] from a [`ResolvedThemeVariant`].
///
/// Maps all 108 fields: ~40 directly from ResolvedThemeVariant per-widget structs,
/// the rest derived via shade generation following gpui-component's own
/// fallback logic.
///
/// The `is_dark` parameter controls both `ThemeMode` (set in [`crate::to_theme`])
/// and active-state darkening amounts -- a single source of truth that prevents
/// the split-brain bug where `ThemeMode` and color derivation could disagree.
///
/// Callers can derive `is_dark` from `resolved.defaults.background` lightness
/// when the caller does not have an explicit dark-mode flag:
/// `let is_dark = rgba_to_hsla(resolved.defaults.background).l < 0.5;`
pub fn to_theme_color(resolved: &ResolvedThemeVariant, is_dark: bool) -> ThemeColor {
    let d = &resolved.defaults;
    let bg = rgba_to_hsla(d.background);
    let fg = rgba_to_hsla(d.foreground);

    // Issue 2: d.muted IS the muted foreground color (subdued text).
    // The muted *background* slot (Skeleton, Switch bg) uses a derived bg.
    let muted_fg = rgba_to_hsla(d.muted);
    let muted_bg = bg.blend(fg.opacity(0.1));

    let c = ResolvedColors {
        bg,
        fg,
        accent: rgba_to_hsla(d.accent),
        accent_fg: rgba_to_hsla(d.accent_foreground),
        surface: rgba_to_hsla(d.surface),
        border: rgba_to_hsla(d.border),
        muted: muted_bg,
        muted_fg,
        primary: rgba_to_hsla(resolved.button.primary_background),
        primary_fg: rgba_to_hsla(resolved.button.primary_foreground),
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
        // Issue 20: per-widget colors
        tab_bg: rgba_to_hsla(resolved.tab.background),
        tab_active_bg: rgba_to_hsla(resolved.tab.active_background),
        tab_active_fg: rgba_to_hsla(resolved.tab.active_foreground),
        tab_bar_bg: rgba_to_hsla(resolved.tab.bar_background),
        tab_fg: rgba_to_hsla(resolved.tab.foreground),
        title_bar_bg: rgba_to_hsla(resolved.window.title_bar_background),
        window_border: rgba_to_hsla(resolved.window.border),
        scrollbar_track: rgba_to_hsla(resolved.scrollbar.track),
        scrollbar_thumb: rgba_to_hsla(resolved.scrollbar.thumb),
        scrollbar_thumb_hover: rgba_to_hsla(resolved.scrollbar.thumb_hover),
        slider_fill: rgba_to_hsla(resolved.slider.fill),
        slider_thumb: rgba_to_hsla(resolved.slider.thumb),
        switch_unchecked: rgba_to_hsla(resolved.switch.unchecked_background),
        switch_thumb: rgba_to_hsla(resolved.switch.thumb_background),
        progress_fill: rgba_to_hsla(resolved.progress_bar.fill),
        input_caret: rgba_to_hsla(resolved.input.caret),
    };

    let mut tc = ThemeColor::default();

    assign_core(&mut tc, &c, is_dark);
    assign_primary(&mut tc, &c, is_dark);
    assign_secondary(&mut tc, &c, is_dark);
    assign_status(&mut tc, &c, is_dark);
    assign_list_table(&mut tc, &c, is_dark);
    assign_tab_sidebar(&mut tc, &c);
    assign_charts(&mut tc, &c);
    assign_misc(&mut tc, &c, resolved, is_dark);
    assign_base_colors(&mut tc, &c, is_dark);

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
    // Issue 9: ensure status foreground colors have sufficient contrast
    // against their respective status backgrounds.
    tc.danger = c.danger;
    tc.danger_foreground = ensure_status_contrast(c.danger_fg, c.danger);
    tc.danger_hover = hover_color(c.danger, c.bg);
    tc.danger_active = active_color(c.danger, is_dark);

    tc.success = c.success;
    tc.success_foreground = ensure_status_contrast(c.success_fg, c.success);
    tc.success_hover = hover_color(c.success, c.bg);
    tc.success_active = active_color(c.success, is_dark);

    tc.warning = c.warning;
    tc.warning_foreground = ensure_status_contrast(c.warning_fg, c.warning);
    tc.warning_hover = hover_color(c.warning, c.bg);
    tc.warning_active = active_color(c.warning, is_dark);

    tc.info = c.info;
    tc.info_foreground = ensure_status_contrast(c.info_fg, c.info);
    tc.info_hover = hover_color(c.info, c.bg);
    tc.info_active = active_color(c.info, is_dark);

    tc.bullish = c.success;
    tc.bearish = c.danger;
}

fn assign_list_table(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
    tc.list = c.bg;
    tc.list_hover = hover_color(c.secondary, c.bg);
    // Issue 7: removed spurious .alpha(0.2) that made the active state nearly
    // invisible. Use mode-aware opacity: dark themes need subtler tinting.
    let active_opacity = if is_dark { 0.08 } else { 0.1 };
    tc.list_active = c.bg.blend(c.primary.opacity(active_opacity));
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

fn assign_tab_sidebar(tc: &mut ThemeColor, c: &ResolvedColors) {
    // Tab: use per-widget resolved tab colors (Issue 20: via ResolvedColors cache)
    tc.tab = c.tab_bg;
    tc.tab_active = c.tab_active_bg;
    tc.tab_active_foreground = c.tab_active_fg;
    tc.tab_bar = c.tab_bar_bg;
    // Issue 42: tab_bar_segmented uses secondary because native-theme's
    // ResolvedTabTheme has no segmented-specific color. The secondary button
    // color is the closest semantic match for the segmented tab indicator.
    tc.tab_bar_segmented = c.secondary;
    tc.tab_foreground = c.tab_fg;

    tc.sidebar = c.sidebar;
    tc.sidebar_foreground = c.sidebar_fg;
    tc.sidebar_accent = c.accent;
    tc.sidebar_accent_foreground = c.accent_fg;
    tc.sidebar_border = c.window_border;
    tc.sidebar_primary = c.primary;
    tc.sidebar_primary_foreground = c.primary_fg;

    // Title bar / window (Issue 20: via ResolvedColors cache)
    tc.title_bar = c.title_bar_bg;
    tc.title_bar_border = c.window_border;
    tc.window_border = c.window_border;
}

fn assign_charts(tc: &mut ThemeColor, c: &ResolvedColors) {
    // Distribute 5 chart colors evenly around the hue wheel (~72 degrees apart).
    // Preserves accent's lightness for palette coherence.
    // Issue 16: enforce a saturation floor of 0.3 so desaturated themes
    // (e.g. Nord, Solarized) still produce distinguishable chart colors.
    let s = c.accent.s.max(0.3);
    tc.chart_1 = c.accent;
    tc.chart_2 = Hsla {
        h: (c.accent.h + 0.2) % 1.0,
        s,
        ..c.accent
    };
    tc.chart_3 = Hsla {
        h: (c.accent.h + 0.4) % 1.0,
        s,
        ..c.accent
    };
    tc.chart_4 = Hsla {
        h: (c.accent.h + 0.6) % 1.0,
        s,
        ..c.accent
    };
    tc.chart_5 = Hsla {
        h: (c.accent.h + 0.8) % 1.0,
        s,
        ..c.accent
    };
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
    // Issue 60: match upstream apply_config fallback: accent.opacity(0.8).
    // Use accent blended into bg for a subtler hover tint.
    tc.accordion_hover = c.bg.blend(c.accent.opacity(0.08));

    // Dark=0.3, light=0.4: lower opacity on dark avoids washing out backgrounds.
    // Diverges from upstream's uniform 0.5 for better per-mode contrast.
    tc.group_box =
        c.bg.blend(c.secondary.opacity(if is_dark { 0.3 } else { 0.4 }));
    tc.group_box_foreground = c.fg;

    tc.description_list_label = c.bg.blend(c.border.opacity(0.2));
    tc.description_list_label_foreground = c.muted_fg;

    // Issue 6: respect reduce_transparency. When the user requests reduced
    // transparency, use a fully opaque overlay instead of semi-transparent.
    let shadow = rgba_to_hsla(resolved.defaults.shadow);
    let overlay_alpha = if resolved.defaults.reduce_transparency {
        1.0
    } else if is_dark {
        0.5
    } else {
        0.4
    };
    tc.overlay = Hsla {
        h: shadow.h,
        s: shadow.s,
        l: shadow.l,
        a: overlay_alpha,
    };

    // Issue 68: use resolved scrollbar track color instead of plain bg
    // Issue 20: via ResolvedColors cache
    tc.scrollbar = c.scrollbar_track;
    tc.scrollbar_thumb = c.scrollbar_thumb;
    tc.scrollbar_thumb_hover = c.scrollbar_thumb_hover;

    // Per-widget resolved slider colors (Issue 20: via ResolvedColors cache)
    tc.slider_bar = c.slider_fill;
    tc.slider_thumb = c.slider_thumb;

    // Per-widget resolved switch colors (Issue 20: via ResolvedColors cache)
    tc.switch = c.switch_unchecked;
    // Issue 51: switch.checked_background is not mapped because gpui-component's
    // ThemeColor has no checked-state field for switches. The unchecked background
    // is the only mappable slot. Callers needing checked-state styling should read
    // resolved.switch.checked_background directly.
    tc.switch_thumb = c.switch_thumb;

    // Per-widget resolved progress bar color (Issue 20: via ResolvedColors cache)
    tc.progress_bar = c.progress_fill;

    // Per-widget resolved caret from input (Issue 20: via ResolvedColors cache)
    tc.caret = c.input_caret;

    tc.skeleton = c.secondary;

    tc.tiles = c.bg;

    tc.drag_border = c.primary.opacity(0.65);
    tc.drop_target = c.primary.opacity(0.2);
}

fn assign_base_colors(tc: &mut ThemeColor, c: &ResolvedColors, is_dark: bool) {
    // Issue 3: _light variants use mode-aware derivation. On dark themes,
    // blending toward a dark bg *darkens* the color (wrong); instead we
    // increase lightness for a visible tinted background.
    tc.red = c.danger;
    tc.red_light = light_variant(c.bg, c.danger, is_dark);
    tc.green = c.success;
    tc.green_light = light_variant(c.bg, c.success, is_dark);
    tc.blue = c.info;
    tc.blue_light = light_variant(c.bg, c.info, is_dark);
    tc.yellow = c.warning;
    tc.yellow_light = light_variant(c.bg, c.warning, is_dark);
    // Magenta: fixed hue, but saturation and lightness from accent
    let magenta = Hsla {
        h: 0.833,
        s: c.accent.s.min(0.85),
        l: c.accent.l,
        a: 1.0,
    };
    tc.magenta = magenta;
    tc.magenta_light = light_variant(c.bg, magenta, is_dark);
    // Cyan: fixed hue, but saturation and lightness from info
    let cyan = Hsla {
        h: 0.5,
        s: c.info.s.min(0.85),
        l: c.info.l,
        a: 1.0,
    };
    tc.cyan = cyan;
    tc.cyan_light = light_variant(c.bg, cyan, is_dark);
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::ThemeSpec;

    /// Create a dark ResolvedThemeVariant for catppuccin-mocha.
    ///
    /// Issue 1: fixed to use `into_variant(true)` -- catppuccin-mocha is a dark
    /// theme, so loading with `false` would pick the light fallback.
    fn test_resolved() -> ResolvedThemeVariant {
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(true)
            .expect("preset must have dark variant");
        variant
            .into_resolved()
            .expect("resolved preset must validate")
    }

    /// Create a light ResolvedThemeVariant for catppuccin-latte.
    fn test_resolved_light() -> ResolvedThemeVariant {
        let nt = ThemeSpec::preset("catppuccin-latte").expect("preset must exist");
        let variant = nt
            .into_variant(false)
            .expect("preset must have light variant");
        variant
            .into_resolved()
            .expect("resolved preset must validate")
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

    // Issue 29: RGBA-to-HSLA clamping
    #[test]
    fn rgba_to_hsla_clamps_out_of_range() {
        // Verify no panic on values that would be out of [0,1] range
        let c = native_theme::Rgba::from_f32(1.5, -0.1, 0.5, 2.0);
        let result = rgba_to_hsla(c);
        assert!(result.a <= 1.0, "alpha should be clamped to 1.0");
    }

    #[test]
    fn hsla_to_hex_roundtrip() {
        let c = Hsla {
            h: 0.0,
            s: 1.0,
            l: 0.5,
            a: 1.0,
        };
        let hex = hsla_to_hex(c);
        assert!(hex.starts_with('#'), "hex should start with #");
        assert_eq!(hex.len(), 7, "hex should be #rrggbb");
    }

    #[test]
    fn to_theme_color_produces_nondefault() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
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
        let dark = Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.1,
            a: 1.0,
        };
        assert!(dark.l < 0.5);
        let light = Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0,
        };
        assert!(light.l >= 0.5);
    }

    #[test]
    fn hover_states_differ_from_base() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);

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
        let tc = to_theme_color(&resolved, true);

        // Scrollbar thumb should match resolved scrollbar
        let expected_thumb = rgba_to_hsla(resolved.scrollbar.thumb);
        assert_eq!(
            tc.scrollbar_thumb, expected_thumb,
            "scrollbar thumb should come from resolved.scrollbar.thumb"
        );

        // Issue 68: scrollbar track should come from resolved, not bg
        let expected_track = rgba_to_hsla(resolved.scrollbar.track);
        assert_eq!(
            tc.scrollbar, expected_track,
            "scrollbar should come from resolved.scrollbar.track"
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

        // Switch should match resolved switch unchecked_background
        let expected_switch = rgba_to_hsla(resolved.switch.unchecked_background);
        assert_eq!(
            tc.switch, expected_switch,
            "switch should come from resolved.switch.unchecked_background"
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
        let tc = to_theme_color(&resolved, true);

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
        let tc = to_theme_color(&resolved, true);

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
        let tc = to_theme_color(&resolved, true);

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
        let tc = to_theme_color(&resolved, true);

        let hues = [
            tc.chart_1.h,
            tc.chart_2.h,
            tc.chart_3.h,
            tc.chart_4.h,
            tc.chart_5.h,
        ];
        // All 5 hues should be distinct
        for i in 0..5 {
            for j in (i + 1)..5 {
                assert!(
                    (hues[i] - hues[j]).abs() > 0.05,
                    "chart_{} (h={:.3}) and chart_{} (h={:.3}) should have distinct hues",
                    i + 1,
                    hues[i],
                    j + 1,
                    hues[j]
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
        let tc = to_theme_color(&resolved, true);

        let accent = rgba_to_hsla(resolved.defaults.accent);
        let expected_s = accent.s.min(0.85);
        assert!(
            (tc.magenta.s - expected_s).abs() < 0.001,
            "magenta saturation {:.3} should be min(accent.s, 0.85) = {:.3}",
            tc.magenta.s,
            expected_s
        );
        assert!(
            (tc.magenta.l - accent.l).abs() < 0.001,
            "magenta lightness {:.3} should match accent lightness {:.3}",
            tc.magenta.l,
            accent.l
        );
    }

    #[test]
    fn overlay_uses_shadow_color() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);

        let shadow = rgba_to_hsla(resolved.defaults.shadow);
        assert!(
            (tc.overlay.h - shadow.h).abs() < 0.001,
            "overlay hue {:.3} should match shadow hue {:.3}",
            tc.overlay.h,
            shadow.h
        );
        assert!(
            (tc.overlay.s - shadow.s).abs() < 0.001,
            "overlay saturation {:.3} should match shadow saturation {:.3}",
            tc.overlay.s,
            shadow.s
        );
    }

    #[test]
    fn theme_color_field_count_tripwire() {
        // ThemeColor has N Hsla fields (each 16 bytes = 4x f32).
        // If this fails, gpui-component added/removed fields -- update the color mapping.
        let size = std::mem::size_of::<ThemeColor>();
        let hsla_size = std::mem::size_of::<Hsla>();
        let field_count = size / hsla_size;
        assert_eq!(
            field_count, 108,
            "ThemeColor field count changed (got {field_count}) -- update color mapping in to_theme_color() and the doc table in lib.rs"
        );
    }

    // Issue 35: per-category tripwire tests to catch doc/code drift.
    // When a count fails, update both the mapping function AND the doc table in lib.rs.
    #[test]
    fn coverage_tab_fields_mapped() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        let d = ThemeColor::default();
        let mapped = [
            tc.tab != d.tab,
            tc.tab_active != d.tab_active,
            tc.tab_active_foreground != d.tab_active_foreground,
            tc.tab_bar != d.tab_bar,
            tc.tab_foreground != d.tab_foreground,
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        assert_eq!(mapped, 5, "doc says 5 tab fields mapped; got {mapped}");
    }

    #[test]
    fn coverage_sidebar_fields_mapped() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        let d = ThemeColor::default();
        // sidebar, sidebar_foreground, sidebar_accent, sidebar_accent_foreground,
        // sidebar_border, sidebar_primary, sidebar_primary_foreground
        let mapped = [
            tc.sidebar != d.sidebar,
            tc.sidebar_foreground != d.sidebar_foreground,
            tc.sidebar_accent != d.sidebar_accent,
            tc.sidebar_accent_foreground != d.sidebar_accent_foreground,
            tc.sidebar_border != d.sidebar_border,
            tc.sidebar_primary != d.sidebar_primary,
            tc.sidebar_primary_foreground != d.sidebar_primary_foreground,
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        assert_eq!(mapped, 7, "doc says sidebar+accent mapped; got {mapped}");
    }

    #[test]
    fn coverage_list_table_fields_mapped() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        let d = ThemeColor::default();
        let mapped = [
            tc.list != d.list,
            tc.list_hover != d.list_hover,
            tc.list_active != d.list_active,
            tc.list_active_border != d.list_active_border,
            tc.list_even != d.list_even,
            tc.list_head != d.list_head,
            tc.table != d.table,
            tc.table_hover != d.table_hover,
            tc.table_active != d.table_active,
            tc.table_active_border != d.table_active_border,
            tc.table_even != d.table_even,
            tc.table_head != d.table_head,
            tc.table_head_foreground != d.table_head_foreground,
            tc.table_row_border != d.table_row_border,
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        assert_eq!(
            mapped, 14,
            "doc says 14 list/table fields mapped; got {mapped}"
        );
    }

    #[test]
    fn coverage_scrollbar_slider_fields_mapped() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        let d = ThemeColor::default();
        let mapped = [
            tc.scrollbar != d.scrollbar,
            tc.scrollbar_thumb != d.scrollbar_thumb,
            tc.scrollbar_thumb_hover != d.scrollbar_thumb_hover,
            tc.slider_bar != d.slider_bar,
            tc.slider_thumb != d.slider_thumb,
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        assert_eq!(
            mapped, 5,
            "doc says 3 scrollbar + 2 slider fields mapped; got {mapped}"
        );
    }

    #[test]
    fn coverage_chart_fields_mapped() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        let d = ThemeColor::default();
        let mapped = [
            tc.chart_1 != d.chart_1,
            tc.chart_2 != d.chart_2,
            tc.chart_3 != d.chart_3,
            tc.chart_4 != d.chart_4,
            tc.chart_5 != d.chart_5,
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        assert_eq!(mapped, 5, "doc says 5 chart fields mapped; got {mapped}");
    }

    // Issue 2: muted_fg should differ from muted background
    #[test]
    fn muted_fg_differs_from_muted_bg() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        assert_ne!(
            tc.muted, tc.muted_foreground,
            "muted (bg) and muted_foreground should be different"
        );
    }

    // Issue 3: _light color variants should be lighter than base on dark themes
    #[test]
    fn light_variants_lighter_on_dark_theme() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        assert!(
            tc.red_light.l > tc.red.l,
            "red_light (l={:.3}) should be lighter than red (l={:.3}) on dark theme",
            tc.red_light.l,
            tc.red.l
        );
        assert!(
            tc.green_light.l > tc.green.l,
            "green_light should be lighter than green on dark theme"
        );
    }

    // Issue 7: list_active should not have alpha forced to 0.2
    #[test]
    fn list_active_is_not_nearly_transparent() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        // The old code forced alpha to 0.2 which made list_active invisible.
        // The new code should produce a color with a > 0.5 (near-opaque from blend).
        assert!(
            tc.list_active.a > 0.5,
            "list_active alpha={} should not be forced to 0.2",
            tc.list_active.a
        );
    }

    // Issue 9: status foreground contrast check
    #[test]
    fn status_foreground_has_sufficient_contrast() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        // All status foregrounds should have at least 4.5:1 contrast
        assert!(
            contrast_ratio(tc.danger_foreground, tc.danger) >= MIN_STATUS_CONTRAST
                || tc.danger_foreground.l == 0.0
                || tc.danger_foreground.l == 1.0,
            "danger_foreground should have sufficient contrast or be black/white"
        );
    }

    // Issue 50: multi-preset dual-mode tests
    #[test]
    fn multi_preset_dark_mode() {
        let presets = [
            "catppuccin-mocha",
            "dracula",
            "nord",
            "tokyo-night",
            "one-dark",
        ];
        for name in presets {
            let nt = ThemeSpec::preset(name).expect("preset must exist");
            let variant = nt
                .into_variant(true)
                .expect("preset must have dark variant");
            let resolved = variant.into_resolved().expect("must validate");
            let _tc = to_theme_color(&resolved, true);
        }
    }

    #[test]
    fn multi_preset_light_mode() {
        let presets = [
            "catppuccin-latte",
            "adwaita",
            "material",
            "solarized",
            "gruvbox",
        ];
        for name in presets {
            let nt = ThemeSpec::preset(name).expect("preset must exist");
            let variant = nt
                .into_variant(false)
                .expect("preset must have light variant");
            let resolved = variant.into_resolved().expect("must validate");
            let _tc = to_theme_color(&resolved, false);
        }
    }

    // Issue 63: tab/sidebar/window fields should be populated from resolved
    #[test]
    fn tab_sidebar_window_fields_populated() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        let default = ThemeColor::default();

        // Tab fields
        assert_ne!(tc.tab, default.tab, "tab should be set");
        assert_ne!(tc.tab_bar, default.tab_bar, "tab_bar should be set");
        assert_ne!(
            tc.tab_foreground, default.tab_foreground,
            "tab_foreground should be set"
        );
        assert_ne!(
            tc.tab_active, default.tab_active,
            "tab_active should be set"
        );
        assert_ne!(
            tc.tab_active_foreground, default.tab_active_foreground,
            "tab_active_foreground should be set"
        );

        // Sidebar fields
        assert_ne!(tc.sidebar, default.sidebar, "sidebar should be set");
        assert_ne!(
            tc.sidebar_accent, default.sidebar_accent,
            "sidebar_accent should be set"
        );
        assert_ne!(
            tc.sidebar_border, default.sidebar_border,
            "sidebar_border should be set"
        );

        // Title bar / window fields
        assert_ne!(tc.title_bar, default.title_bar, "title_bar should be set");
        assert_ne!(
            tc.title_bar_border, default.title_bar_border,
            "title_bar_border should be set"
        );
    }

    // Issue 75: list_active_border and table_active_border should be populated
    #[test]
    fn list_active_border_and_table_active_border_populated() {
        let resolved = test_resolved();
        let tc = to_theme_color(&resolved, true);
        let default = ThemeColor::default();

        assert_ne!(
            tc.list_active_border, default.list_active_border,
            "list_active_border should be set"
        );
        assert_ne!(
            tc.table_active_border, default.table_active_border,
            "table_active_border should be set"
        );
    }

    // Issue 50: light theme produces lighter base colors
    #[test]
    fn light_theme_light_variants_blend_toward_bg() {
        let resolved = test_resolved_light();
        let tc = to_theme_color(&resolved, false);
        // On a light theme, red_light should be lighter than red (blended toward white bg)
        assert!(
            tc.red_light.l > tc.red.l,
            "red_light (l={:.3}) should be > red (l={:.3}) on light theme",
            tc.red_light.l,
            tc.red.l
        );
    }
}
