//! ResolvedTheme -> gpui_component::theme::ThemeColor mapping (108 fields).
//!
//! Maps native-theme's per-widget resolved fields to gpui-component's 108-field
//! ThemeColor struct. Direct mappings cover ~40 fields; the remaining ~68 are
//! derived via shade generation, blending, or fallback logic that mirrors
//! gpui-component's own `apply_config` derivation.

use gpui::Hsla;
use gpui_component::{Colorize, theme::ThemeColor};
use native_theme::ResolvedTheme;

use crate::derive::{active_color, hover_color};

/// Convert a `native_theme::Rgba` to `gpui::Hsla`.
fn rgba_to_hsla(rgba: native_theme::Rgba) -> Hsla {
    let [r, g, b, a] = rgba.to_f32_array();
    let gpui_rgba = gpui::Rgba { r, g, b, a };
    gpui_rgba.into()
}

/// Returns true if the background color indicates a dark theme (lightness < 0.5).
fn is_dark_background(bg: Hsla) -> bool {
    bg.l < 0.5
}

/// Build a complete [`ThemeColor`] from a [`ResolvedTheme`].
///
/// Maps all 108 fields: ~40 directly from ResolvedTheme per-widget structs,
/// the rest derived via shade generation following gpui-component's own
/// fallback logic.
pub fn to_theme_color(resolved: &ResolvedTheme) -> ThemeColor {
    let d = &resolved.defaults;

    // -- Resolve core colors from defaults --
    let bg = rgba_to_hsla(d.background);
    let fg = rgba_to_hsla(d.foreground);
    let is_dark = is_dark_background(bg);
    let accent = rgba_to_hsla(d.accent);
    let surface = rgba_to_hsla(d.surface);
    let border = rgba_to_hsla(d.border);
    let muted = rgba_to_hsla(d.muted);

    // -- Semantic colors from defaults and per-widget structs --
    let primary = rgba_to_hsla(resolved.button.primary_bg);
    let primary_fg = rgba_to_hsla(resolved.button.primary_fg);
    let secondary = rgba_to_hsla(resolved.button.background);
    let secondary_fg = rgba_to_hsla(resolved.button.foreground);
    let danger = rgba_to_hsla(d.danger);
    let danger_fg = rgba_to_hsla(d.danger_foreground);
    let success = rgba_to_hsla(d.success);
    let success_fg = rgba_to_hsla(d.success_foreground);
    let warning = rgba_to_hsla(d.warning);
    let warning_fg = rgba_to_hsla(d.warning_foreground);
    let info = rgba_to_hsla(d.info);
    let info_fg = rgba_to_hsla(d.info_foreground);
    let selection = rgba_to_hsla(d.selection);
    let link = rgba_to_hsla(d.link);
    let ring = rgba_to_hsla(d.focus_ring_color);
    let input = rgba_to_hsla(resolved.input.border);

    let sidebar = rgba_to_hsla(resolved.sidebar.background);
    let sidebar_fg = rgba_to_hsla(resolved.sidebar.foreground);
    let popover = rgba_to_hsla(resolved.popover.background);
    let popover_fg = rgba_to_hsla(resolved.popover.foreground);
    let muted_fg = rgba_to_hsla(d.muted).blend(fg.opacity(0.7));
    let alternate_row = rgba_to_hsla(resolved.list.alternate_row);

    // -- Build ThemeColor from defaults and override everything --
    let mut tc = ThemeColor::default();

    // Core
    assign_core(
        &mut tc, bg, fg, accent, border, muted, muted_fg, input, ring, selection, link,
    );

    // Primary/Secondary
    assign_primary(&mut tc, primary, primary_fg, bg, is_dark);
    assign_secondary(&mut tc, secondary, secondary_fg, bg, is_dark);

    // Status colors (danger, success, warning, info)
    assign_status(
        &mut tc, danger, danger_fg, success, success_fg, warning, warning_fg, info, info_fg, bg,
        is_dark, primary_fg,
    );

    // List/table
    assign_list_table(
        &mut tc,
        bg,
        alternate_row,
        primary,
        secondary,
        border,
        muted_fg,
    );

    // Tab/sidebar
    assign_tab_sidebar(
        &mut tc, resolved, bg, fg, surface, secondary, sidebar, sidebar_fg, accent, primary,
        primary_fg, border,
    );

    // Chart colors
    assign_charts(&mut tc, accent);

    // Misc (overlay, scrollbar, slider, switch, etc.)
    assign_misc(
        &mut tc, resolved, bg, fg, accent, muted, primary, primary_fg, secondary, border, surface,
        is_dark, link, popover, popover_fg,
    );

    // Base named colors
    assign_base_colors(&mut tc, danger, success, info, warning, bg);

    tc
}

// ---------- helper assignment functions ----------

#[allow(clippy::too_many_arguments)]
fn assign_core(
    tc: &mut ThemeColor,
    bg: Hsla,
    fg: Hsla,
    accent: Hsla,
    border: Hsla,
    muted: Hsla,
    muted_fg: Hsla,
    input: Hsla,
    ring: Hsla,
    selection: Hsla,
    link: Hsla,
) {
    tc.background = bg;
    tc.foreground = fg;
    tc.accent = accent;
    tc.accent_foreground = fg;
    tc.border = border;
    tc.muted = muted;
    tc.muted_foreground = muted_fg;
    tc.input = input;
    tc.ring = ring;
    tc.selection = selection.alpha(selection.a.min(0.3));
    tc.link = link;
    tc.link_hover = link;
    tc.link_active = link;
}

fn assign_primary(tc: &mut ThemeColor, primary: Hsla, primary_fg: Hsla, bg: Hsla, is_dark: bool) {
    tc.primary = primary;
    tc.primary_foreground = primary_fg;
    tc.primary_hover = hover_color(primary, bg);
    tc.primary_active = active_color(primary, is_dark);
}

fn assign_secondary(
    tc: &mut ThemeColor,
    secondary: Hsla,
    secondary_fg: Hsla,
    bg: Hsla,
    is_dark: bool,
) {
    tc.secondary = secondary;
    tc.secondary_foreground = secondary_fg;
    tc.secondary_hover = hover_color(secondary, bg);
    tc.secondary_active = active_color(secondary, is_dark);
}

#[allow(clippy::too_many_arguments)]
fn assign_status(
    tc: &mut ThemeColor,
    danger: Hsla,
    danger_fg: Hsla,
    success: Hsla,
    success_fg: Hsla,
    warning: Hsla,
    warning_fg: Hsla,
    info: Hsla,
    info_fg: Hsla,
    bg: Hsla,
    is_dark: bool,
    _primary_fg: Hsla,
) {
    tc.danger = danger;
    tc.danger_foreground = danger_fg;
    tc.danger_hover = hover_color(danger, bg);
    tc.danger_active = active_color(danger, is_dark);

    tc.success = success;
    tc.success_foreground = success_fg;
    tc.success_hover = hover_color(success, bg);
    tc.success_active = active_color(success, is_dark);

    tc.warning = warning;
    tc.warning_foreground = warning_fg;
    tc.warning_hover = hover_color(warning, bg);
    tc.warning_active = active_color(warning, is_dark);

    tc.info = info;
    tc.info_foreground = info_fg;
    tc.info_hover = hover_color(info, bg);
    tc.info_active = active_color(info, is_dark);

    tc.bullish = success;
    tc.bearish = danger;
}

fn assign_list_table(
    tc: &mut ThemeColor,
    bg: Hsla,
    alternate_row: Hsla,
    primary: Hsla,
    secondary: Hsla,
    border: Hsla,
    muted_fg: Hsla,
) {
    tc.list = bg;
    tc.list_hover = hover_color(secondary, bg);
    tc.list_active = bg.blend(primary.opacity(0.1)).alpha(0.2);
    tc.list_active_border = bg.blend(primary.opacity(0.6));
    tc.list_even = alternate_row;
    tc.list_head = bg;

    tc.table = bg;
    tc.table_hover = tc.list_hover;
    tc.table_active = tc.list_active;
    tc.table_active_border = tc.list_active_border;
    tc.table_even = tc.list_even;
    tc.table_head = bg;
    tc.table_head_foreground = muted_fg;
    tc.table_row_border = border;
}

#[allow(clippy::too_many_arguments)]
fn assign_tab_sidebar(
    tc: &mut ThemeColor,
    resolved: &ResolvedTheme,
    _bg: Hsla,
    fg: Hsla,
    _surface: Hsla,
    secondary: Hsla,
    sidebar: Hsla,
    sidebar_fg: Hsla,
    accent: Hsla,
    primary: Hsla,
    primary_fg: Hsla,
    border: Hsla,
) {
    // Tab: use per-widget resolved tab colors
    tc.tab = rgba_to_hsla(resolved.tab.background);
    tc.tab_active = rgba_to_hsla(resolved.tab.active_background);
    tc.tab_active_foreground = rgba_to_hsla(resolved.tab.active_foreground);
    tc.tab_bar = rgba_to_hsla(resolved.tab.bar_background);
    tc.tab_bar_segmented = secondary;
    tc.tab_foreground = rgba_to_hsla(resolved.tab.foreground);

    tc.sidebar = sidebar;
    tc.sidebar_foreground = sidebar_fg;
    tc.sidebar_accent = accent;
    tc.sidebar_accent_foreground = fg;
    tc.sidebar_border = border;
    tc.sidebar_primary = primary;
    tc.sidebar_primary_foreground = primary_fg;

    // Title bar: use per-widget resolved window colors
    tc.title_bar = rgba_to_hsla(resolved.window.title_bar_background);
    tc.title_bar_border = rgba_to_hsla(resolved.window.border);
    tc.window_border = rgba_to_hsla(resolved.window.border);
}

fn assign_charts(tc: &mut ThemeColor, accent: Hsla) {
    // Distribute chart colors around the hue wheel from the accent
    tc.chart_1 = accent.lighten(0.4);
    tc.chart_2 = accent.lighten(0.2);
    tc.chart_3 = accent;
    tc.chart_4 = accent.darken(0.2);
    tc.chart_5 = accent.darken(0.4);
}

#[allow(clippy::too_many_arguments)]
fn assign_misc(
    tc: &mut ThemeColor,
    resolved: &ResolvedTheme,
    bg: Hsla,
    fg: Hsla,
    accent: Hsla,
    _muted: Hsla,
    primary: Hsla,
    _primary_fg: Hsla,
    secondary: Hsla,
    border: Hsla,
    _surface: Hsla,
    is_dark: bool,
    link: Hsla,
    popover: Hsla,
    popover_fg: Hsla,
) {
    tc.popover = popover;
    tc.popover_foreground = popover_fg;

    tc.accordion = bg;
    tc.accordion_hover = accent.opacity(0.8);

    tc.group_box = bg.blend(secondary.opacity(if is_dark { 0.3 } else { 0.4 }));
    tc.group_box_foreground = fg;

    tc.description_list_label = bg.blend(border.opacity(0.2));
    tc.description_list_label_foreground = tc.muted_foreground;

    tc.overlay = if is_dark {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.5,
        }
    } else {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.4,
        }
    };

    // Per-widget resolved scrollbar colors
    tc.scrollbar = bg;
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

    tc.skeleton = secondary;

    tc.tiles = bg;

    tc.drag_border = primary.opacity(0.65);
    tc.drop_target = primary.opacity(0.2);

    tc.link_hover = link;
    tc.link_active = link;
}

fn assign_base_colors(
    tc: &mut ThemeColor,
    danger: Hsla,
    success: Hsla,
    info: Hsla,
    warning: Hsla,
    bg: Hsla,
) {
    tc.red = danger;
    tc.red_light = bg.blend(danger.opacity(0.8));
    tc.green = success;
    tc.green_light = bg.blend(success.opacity(0.8));
    tc.blue = info;
    tc.blue_light = bg.blend(info.opacity(0.8));
    tc.yellow = warning;
    tc.yellow_light = bg.blend(warning.opacity(0.8));
    // Magenta: derive from accent hue-shifted
    let magenta = Hsla {
        h: 0.833,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    };
    tc.magenta = magenta;
    tc.magenta_light = bg.blend(magenta.opacity(0.8));
    tc.cyan = info;
    tc.cyan_light = bg.blend(info.opacity(0.8));
}

#[cfg(test)]
mod tests {
    use super::*;
    use native_theme::NativeTheme;

    /// Create a ResolvedTheme via the preset resolve+validate pipeline.
    fn test_resolved() -> ResolvedTheme {
        let nt = NativeTheme::preset("catppuccin-mocha").expect("preset must exist");
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
        let tc = to_theme_color(&resolved);
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
        let tc = to_theme_color(&resolved);

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
        let tc = to_theme_color(&resolved);

        // Scrollbar thumb should match resolved scrollbar
        let expected_thumb = rgba_to_hsla(resolved.scrollbar.thumb);
        assert_eq!(tc.scrollbar_thumb, expected_thumb, "scrollbar thumb should come from resolved.scrollbar.thumb");

        // Slider bar should match resolved slider fill
        let expected_slider = rgba_to_hsla(resolved.slider.fill);
        assert_eq!(tc.slider_bar, expected_slider, "slider bar should come from resolved.slider.fill");

        // Progress bar should match resolved progress_bar fill
        let expected_progress = rgba_to_hsla(resolved.progress_bar.fill);
        assert_eq!(tc.progress_bar, expected_progress, "progress bar should come from resolved.progress_bar.fill");

        // Title bar should match resolved window title_bar_background
        let expected_title = rgba_to_hsla(resolved.window.title_bar_background);
        assert_eq!(tc.title_bar, expected_title, "title bar should come from resolved.window.title_bar_background");

        // Switch should match resolved switch unchecked_bg
        let expected_switch = rgba_to_hsla(resolved.switch.unchecked_bg);
        assert_eq!(tc.switch, expected_switch, "switch should come from resolved.switch.unchecked_bg");

        // Caret should match resolved input caret
        let expected_caret = rgba_to_hsla(resolved.input.caret);
        assert_eq!(tc.caret, expected_caret, "caret should come from resolved.input.caret");
    }
}
