// KDE color group parsing -> populate per-widget fields on ThemeVariant
// Maps semantic color roles from KDE INI color groups directly to ThemeVariant.

use crate::Rgba;

/// Look up a color key from a KDE INI section and parse it as RGB.
fn get_color(ini: &configparser::ini::Ini, section: &str, key: &str) -> Option<Rgba> {
    let value = ini.get(section, key)?;
    super::parse_rgb(&value)
}

/// Populate a ThemeVariant with colors from KDE INI color groups.
///
/// Maps all standard KDE color groups (View, Window, Button, Selection,
/// Tooltip, Complementary, Header, WM) to per-widget fields on the variant.
/// Missing INI keys result in None fields (no hardcoded fallbacks).
pub(crate) fn populate_colors(ini: &configparser::ini::Ini, variant: &mut crate::ThemeVariant) {
    let window_fg = get_color(ini, "Colors:Window", "ForegroundNormal");

    // === defaults-level colors ===
    variant.defaults.accent = get_color(ini, "Colors:View", "DecorationFocus");
    variant.defaults.background = get_color(ini, "Colors:Window", "BackgroundNormal");
    variant.defaults.foreground = window_fg;
    variant.defaults.surface = get_color(ini, "Colors:View", "BackgroundNormal");
    variant.defaults.border = get_color(ini, "Colors:Window", "DecorationFocus");
    variant.defaults.muted = get_color(ini, "Colors:Window", "ForegroundInactive");
    // KDE does not expose shadow color in kdeglobals
    variant.defaults.link = get_color(ini, "Colors:View", "ForegroundLink");
    variant.defaults.focus_ring_color = get_color(ini, "Colors:View", "DecorationFocus");

    // Selection
    variant.defaults.selection = get_color(ini, "Colors:Selection", "BackgroundNormal");
    variant.defaults.selection_foreground = get_color(ini, "Colors:Selection", "ForegroundNormal");

    // Status colors
    variant.defaults.danger = get_color(ini, "Colors:View", "ForegroundNegative");
    variant.defaults.danger_foreground = window_fg;
    variant.defaults.warning = get_color(ini, "Colors:View", "ForegroundNeutral");
    variant.defaults.warning_foreground = window_fg;
    variant.defaults.success = get_color(ini, "Colors:View", "ForegroundPositive");
    variant.defaults.success_foreground = window_fg;
    variant.defaults.info = get_color(ini, "Colors:View", "ForegroundActive");
    variant.defaults.info_foreground = window_fg;

    // Disabled
    variant.defaults.disabled_foreground = get_color(ini, "Colors:View", "ForegroundInactive");

    // === per-widget colors ===

    // Button
    variant.button.background = get_color(ini, "Colors:Button", "BackgroundNormal");
    variant.button.foreground = get_color(ini, "Colors:Button", "ForegroundNormal");

    // Tooltip
    variant.tooltip.background = get_color(ini, "Colors:Tooltip", "BackgroundNormal");
    variant.tooltip.foreground = get_color(ini, "Colors:Tooltip", "ForegroundNormal");

    // Sidebar (from Complementary group)
    variant.sidebar.background = get_color(ini, "Colors:Complementary", "BackgroundNormal");
    variant.sidebar.foreground = get_color(ini, "Colors:Complementary", "ForegroundNormal");

    // Input
    variant.input.background = get_color(ini, "Colors:View", "BackgroundNormal");
    variant.input.foreground = get_color(ini, "Colors:View", "ForegroundNormal");
    // KDE-02: placeholder from View/ForegroundInactive
    variant.input.placeholder = get_color(ini, "Colors:View", "ForegroundInactive");
    // input.caret from View/DecorationFocus (the focus decoration color)
    variant.input.caret = get_color(ini, "Colors:View", "DecorationFocus");

    // Popover (from View)
    variant.popover.background = get_color(ini, "Colors:View", "BackgroundNormal");
    variant.popover.foreground = get_color(ini, "Colors:View", "ForegroundNormal");

    // Separator
    variant.separator.color = get_color(ini, "Colors:Window", "ForegroundInactive");

    // KDE-02: list fields
    variant.list.alternate_row = get_color(ini, "Colors:View", "BackgroundAlternate");
    variant.list.header_background = get_color(ini, "Colors:Header", "BackgroundNormal");
    variant.list.header_foreground = get_color(ini, "Colors:Header", "ForegroundNormal");

    // KDE-02: link.visited
    variant.link.visited = get_color(ini, "Colors:View", "ForegroundVisited");
    // link.color from View/ForegroundLink
    variant.link.color = get_color(ini, "Colors:View", "ForegroundLink");

    // === KDE-01: Window Manager title bar colors ===
    variant.window.title_bar_background = get_color(ini, "WM", "activeBackground");
    variant.window.title_bar_foreground = get_color(ini, "WM", "activeForeground");
    variant.window.inactive_title_bar_background = get_color(ini, "WM", "inactiveBackground");
    variant.window.inactive_title_bar_foreground = get_color(ini, "WM", "inactiveForeground");
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ThemeVariant;

    /// Full Breeze Dark kdeglobals fixture with all 6 color groups + WM + Header.
    const BREEZE_DARK: &str = "\
[Colors:View]
BackgroundNormal=35,38,41
BackgroundAlternate=30,33,36
ForegroundNormal=252,252,252
ForegroundInactive=161,169,177
ForegroundActive=61,174,233
ForegroundLink=29,153,243
ForegroundNegative=218,68,83
ForegroundNeutral=246,116,0
ForegroundPositive=39,174,96
ForegroundVisited=155,89,182
DecorationFocus=61,174,233
DecorationHover=29,153,243

[Colors:Window]
BackgroundNormal=49,54,59
BackgroundAlternate=44,49,54
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177
ForegroundActive=61,174,233
ForegroundLink=29,153,243
ForegroundNegative=218,68,83
ForegroundNeutral=246,116,0
ForegroundPositive=39,174,96
DecorationFocus=61,174,233
DecorationHover=29,153,243

[Colors:Button]
BackgroundNormal=49,54,59
BackgroundAlternate=44,49,54
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177

[Colors:Selection]
BackgroundNormal=61,174,233
BackgroundAlternate=29,153,243
ForegroundNormal=252,252,252
ForegroundInactive=161,169,177

[Colors:Tooltip]
BackgroundNormal=49,54,59
ForegroundNormal=252,252,252

[Colors:Complementary]
BackgroundNormal=42,46,50
ForegroundNormal=239,240,241

[Colors:Header]
BackgroundNormal=35,38,41
ForegroundNormal=252,252,252

[WM]
activeBackground=49,54,59
activeForeground=239,240,241
inactiveBackground=42,46,50
inactiveForeground=161,169,177
";

    fn populate_fixture(content: &str) -> ThemeVariant {
        let mut ini = super::super::create_kde_parser();
        ini.read(content.to_string()).unwrap();
        let mut variant = ThemeVariant::default();
        populate_colors(&ini, &mut variant);
        variant
    }

    // === defaults-level color mapping ===

    #[test]
    fn test_accent_from_view_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.accent, Some(Rgba::rgb(61, 174, 233)));
    }

    #[test]
    fn test_background_from_window() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.background, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_foreground_from_window() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.foreground, Some(Rgba::rgb(239, 240, 241)));
    }

    #[test]
    fn test_surface_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.surface, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_border_from_window_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.border, Some(Rgba::rgb(61, 174, 233)));
    }

    #[test]
    fn test_muted_from_window_foreground_inactive() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.muted, Some(Rgba::rgb(161, 169, 177)));
    }

    // === Status colors ===

    #[test]
    fn test_status_colors() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.danger, Some(Rgba::rgb(218, 68, 83)));
        assert_eq!(v.defaults.warning, Some(Rgba::rgb(246, 116, 0)));
        assert_eq!(v.defaults.success, Some(Rgba::rgb(39, 174, 96)));
        assert_eq!(v.defaults.info, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Selection ===

    #[test]
    fn test_selection_colors() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.selection, Some(Rgba::rgb(61, 174, 233)));
        assert_eq!(
            v.defaults.selection_foreground,
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === Per-widget: Button ===

    #[test]
    fn test_button_background_from_colors_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.button.background, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_button_foreground_from_colors_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.button.foreground, Some(Rgba::rgb(239, 240, 241)));
    }

    // === Per-widget: Tooltip ===

    #[test]
    fn test_tooltip_background_from_colors_tooltip() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.tooltip.background, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_tooltip_foreground_from_colors_tooltip() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.tooltip.foreground, Some(Rgba::rgb(252, 252, 252)));
    }

    // === Per-widget: Sidebar (Complementary) ===

    #[test]
    fn test_sidebar_background_from_complementary() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.sidebar.background, Some(Rgba::rgb(42, 46, 50)));
    }

    #[test]
    fn test_sidebar_foreground_from_complementary() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.sidebar.foreground, Some(Rgba::rgb(239, 240, 241)));
    }

    // === KDE-01: Title bar from WM ===

    #[test]
    fn test_title_bar_background_from_wm_active() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.window.title_bar_background, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_title_bar_foreground_from_wm_active() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.title_bar_foreground,
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    #[test]
    fn test_inactive_title_bar_background_from_wm() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.inactive_title_bar_background,
            Some(Rgba::rgb(42, 46, 50))
        );
    }

    #[test]
    fn test_inactive_title_bar_foreground_from_wm() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.inactive_title_bar_foreground,
            Some(Rgba::rgb(161, 169, 177))
        );
    }

    // === KDE-02: Input placeholder and caret ===

    #[test]
    fn test_input_placeholder_from_view_foreground_inactive() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.input.placeholder, Some(Rgba::rgb(161, 169, 177)));
    }

    #[test]
    fn test_input_caret_from_view_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.input.caret, Some(Rgba::rgb(61, 174, 233)));
    }

    // === KDE-02: List alternate_row ===

    #[test]
    fn test_list_alternate_row_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.alternate_row, Some(Rgba::rgb(30, 33, 36)));
    }

    // === KDE-02: List header from Header group ===

    #[test]
    fn test_list_header_background_from_header() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.header_background, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_list_header_foreground_from_header() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.header_foreground, Some(Rgba::rgb(252, 252, 252)));
    }

    // === KDE-02: Link visited ===

    #[test]
    fn test_link_visited_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.link.visited, Some(Rgba::rgb(155, 89, 182)));
    }

    // === Sidebar None when Complementary missing ===

    #[test]
    fn test_sidebar_none_without_complementary() {
        let content = "\
[Colors:View]
BackgroundNormal=35,38,41
ForegroundNormal=252,252,252
";
        let v = populate_fixture(content);
        assert!(v.sidebar.background.is_none());
        assert!(v.sidebar.foreground.is_none());
    }

    // === Empty INI ===

    #[test]
    fn test_empty_ini() {
        let ini = super::super::create_kde_parser();
        let mut variant = ThemeVariant::default();
        populate_colors(&ini, &mut variant);
        assert!(variant.defaults.accent.is_none());
        assert!(variant.defaults.background.is_none());
        assert!(variant.defaults.foreground.is_none());
        assert!(variant.button.background.is_none());
    }

    // === Partial sections ===

    #[test]
    fn test_partial_sections() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177
DecorationFocus=61,174,233
";
        let v = populate_fixture(content);
        assert_eq!(v.defaults.background, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(v.defaults.foreground, Some(Rgba::rgb(239, 240, 241)));
        assert_eq!(v.defaults.muted, Some(Rgba::rgb(161, 169, 177)));
        assert_eq!(v.defaults.border, Some(Rgba::rgb(61, 174, 233)));
        assert!(v.defaults.accent.is_none());
        assert!(v.defaults.surface.is_none());
        assert!(v.button.background.is_none());
    }

    // === Malformed values ===

    #[test]
    fn test_malformed_values() {
        let content = "\
[Colors:View]
BackgroundNormal=abc,def,ghi
ForegroundNormal=252,252,252
DecorationFocus=61,174,233
";
        let v = populate_fixture(content);
        assert!(v.defaults.surface.is_none()); // BackgroundNormal was malformed
        assert_eq!(v.defaults.accent, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Full mapping: all non-None fields populated ===

    #[test]
    fn test_full_mapping_completeness() {
        let v = populate_fixture(BREEZE_DARK);

        // defaults-level
        assert!(v.defaults.accent.is_some(), "accent missing");
        assert!(v.defaults.background.is_some(), "background missing");
        assert!(v.defaults.foreground.is_some(), "foreground missing");
        assert!(v.defaults.surface.is_some(), "surface missing");
        assert!(v.defaults.border.is_some(), "border missing");
        assert!(v.defaults.muted.is_some(), "muted missing");
        assert!(v.defaults.link.is_some(), "link missing");
        assert!(v.defaults.selection.is_some(), "selection missing");
        assert!(
            v.defaults.selection_foreground.is_some(),
            "selection_foreground missing"
        );
        assert!(v.defaults.danger.is_some(), "danger missing");
        assert!(v.defaults.warning.is_some(), "warning missing");
        assert!(v.defaults.success.is_some(), "success missing");
        assert!(v.defaults.info.is_some(), "info missing");

        // per-widget
        assert!(v.button.background.is_some(), "button.background missing");
        assert!(v.button.foreground.is_some(), "button.foreground missing");
        assert!(v.tooltip.background.is_some(), "tooltip.background missing");
        assert!(v.tooltip.foreground.is_some(), "tooltip.foreground missing");
        assert!(v.sidebar.background.is_some(), "sidebar.background missing");
        assert!(v.sidebar.foreground.is_some(), "sidebar.foreground missing");
        assert!(v.input.background.is_some(), "input.background missing");
        assert!(v.input.foreground.is_some(), "input.foreground missing");
        assert!(v.input.placeholder.is_some(), "input.placeholder missing");
        assert!(v.input.caret.is_some(), "input.caret missing");
        assert!(v.separator.color.is_some(), "separator.color missing");
        assert!(v.list.alternate_row.is_some(), "list.alternate_row missing");
        assert!(
            v.list.header_background.is_some(),
            "list.header_background missing"
        );
        assert!(
            v.list.header_foreground.is_some(),
            "list.header_foreground missing"
        );
        assert!(v.link.visited.is_some(), "link.visited missing");

        // WM title bar
        assert!(
            v.window.title_bar_background.is_some(),
            "window.title_bar_background missing"
        );
        assert!(
            v.window.title_bar_foreground.is_some(),
            "window.title_bar_foreground missing"
        );
        assert!(
            v.window.inactive_title_bar_background.is_some(),
            "window.inactive_title_bar_background missing"
        );
        assert!(
            v.window.inactive_title_bar_foreground.is_some(),
            "window.inactive_title_bar_foreground missing"
        );
    }
}
