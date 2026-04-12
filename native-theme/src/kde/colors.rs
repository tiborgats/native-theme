// KDE color group parsing -> populate per-widget fields on ThemeMode
// Maps semantic color roles from KDE INI color groups directly to ThemeMode.

use crate::Rgba;

/// Look up a color key from a KDE INI section and parse it as RGB.
fn get_color(ini: &configparser::ini::Ini, section: &str, key: &str) -> Option<Rgba> {
    let value = ini.get(section, key)?;
    super::parse_rgb(&value)
}

/// Populate a ThemeMode with colors from KDE INI color groups.
///
/// Maps all standard KDE color groups (View, Window, Button, Selection,
/// Tooltip, Complementary, Header, WM) to per-widget fields on the variant.
/// Missing INI keys result in None fields (no hardcoded fallbacks).
pub(crate) fn populate_colors(ini: &configparser::ini::Ini, variant: &mut crate::ThemeMode) {
    let window_fg = get_color(ini, "Colors:Window", "ForegroundNormal");

    // === defaults-level colors ===
    variant.defaults.accent_color = get_color(ini, "Colors:View", "DecorationFocus");
    variant.defaults.background_color = get_color(ini, "Colors:Window", "BackgroundNormal");
    variant.defaults.text_color = window_fg;
    variant.defaults.surface_color = get_color(ini, "Colors:View", "BackgroundNormal");
    // border: not set by reader -- KDE has no native border color API.
    // The preset provides the correct neutral gray (platform-facts: "(preset)").
    variant.defaults.muted_color = get_color(ini, "Colors:Window", "ForegroundInactive");
    // KDE does not expose shadow color in kdeglobals
    variant.defaults.link_color = get_color(ini, "Colors:View", "ForegroundLink");
    variant.defaults.focus_ring_color = get_color(ini, "Colors:View", "DecorationFocus");

    // Selection
    variant.defaults.selection_background = get_color(ini, "Colors:Selection", "BackgroundNormal");
    variant.defaults.selection_text_color = get_color(ini, "Colors:Selection", "ForegroundNormal");
    // accent_text_color: foreground on accent-colored backgrounds (platform-facts 2.1.3)
    variant.defaults.accent_text_color = get_color(ini, "Colors:Selection", "ForegroundNormal");

    // Status colors
    variant.defaults.danger_color = get_color(ini, "Colors:View", "ForegroundNegative");
    variant.defaults.danger_text_color = window_fg;
    variant.defaults.warning_color = get_color(ini, "Colors:View", "ForegroundNeutral");
    variant.defaults.warning_text_color = window_fg;
    variant.defaults.success_color = get_color(ini, "Colors:View", "ForegroundPositive");
    variant.defaults.success_text_color = window_fg;
    variant.defaults.info_color = get_color(ini, "Colors:View", "ForegroundActive");
    variant.defaults.info_text_color = window_fg;

    // Disabled
    variant.defaults.disabled_text_color = get_color(ini, "Colors:View", "ForegroundInactive");

    // === per-widget colors ===

    // Button
    variant.button.background_color = get_color(ini, "Colors:Button", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Button", "ForegroundNormal") {
        variant.button.font.get_or_insert_default().color = Some(color);
    }

    // Tooltip
    variant.tooltip.background_color = get_color(ini, "Colors:Tooltip", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Tooltip", "ForegroundNormal") {
        variant.tooltip.font.get_or_insert_default().color = Some(color);
    }

    // Sidebar (from Complementary group)
    variant.sidebar.background_color = get_color(ini, "Colors:Complementary", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Complementary", "ForegroundNormal") {
        variant.sidebar.font.get_or_insert_default().color = Some(color);
    }

    // Input
    variant.input.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.input.font.get_or_insert_default().color = Some(color);
    }
    // KDE-02: placeholder from View/ForegroundInactive
    variant.input.placeholder_color = get_color(ini, "Colors:View", "ForegroundInactive");
    // input.caret from View/DecorationFocus (the focus decoration color)
    variant.input.caret_color = get_color(ini, "Colors:View", "DecorationFocus");

    // Popover (from View)
    variant.popover.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.popover.font.get_or_insert_default().color = Some(color);
    }

    // Separator
    variant.separator.line_color = get_color(ini, "Colors:Window", "ForegroundInactive");

    // KDE-02: list fields (Colors:View is the native source for list/table content areas)
    variant.list.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.list.item_font.get_or_insert_default().color = Some(color);
    }
    variant.list.alternate_row_background = get_color(ini, "Colors:View", "BackgroundAlternate");
    variant.list.header_background = get_color(ini, "Colors:Header", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Header", "ForegroundNormal") {
        variant.list.header_font.get_or_insert_default().color = Some(color);
    }

    // KDE-02: link.visited
    variant.link.visited_text_color = get_color(ini, "Colors:View", "ForegroundVisited");
    // link.font.color from View/ForegroundLink
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundLink") {
        variant.link.font.get_or_insert_default().color = Some(color);
    }

    // === KDE-01: Window Manager title bar colors ===
    variant.window.title_bar_background = get_color(ini, "WM", "activeBackground");
    if let Some(color) = get_color(ini, "WM", "activeForeground") {
        variant.window.title_bar_font.get_or_insert_default().color = Some(color);
    }
    variant.window.inactive_title_bar_background = get_color(ini, "WM", "inactiveBackground");
    variant.window.inactive_title_bar_text_color = get_color(ini, "WM", "inactiveForeground");
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ThemeMode;

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

    fn populate_fixture(content: &str) -> ThemeMode {
        let mut ini = super::super::create_kde_parser();
        ini.read(content.to_string()).unwrap();
        let mut variant = ThemeMode::default();
        populate_colors(&ini, &mut variant);
        variant
    }

    // === defaults-level color mapping ===

    #[test]
    fn test_accent_from_view_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    }

    #[test]
    fn test_background_from_window() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_text_color_from_window() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.text_color, Some(Rgba::rgb(239, 240, 241)));
    }

    #[test]
    fn test_surface_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.surface_color, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_border_not_set_by_reader() {
        // Border is a preset value, not reader-provided (platform-facts: "(preset)")
        let v = populate_fixture(BREEZE_DARK);
        assert!(v.defaults.border.is_empty());
    }

    #[test]
    fn test_muted_from_window_foreground_inactive() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.muted_color, Some(Rgba::rgb(161, 169, 177)));
    }

    // === Status colors ===

    #[test]
    fn test_status_colors() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.danger_color, Some(Rgba::rgb(218, 68, 83)));
        assert_eq!(v.defaults.warning_color, Some(Rgba::rgb(246, 116, 0)));
        assert_eq!(v.defaults.success_color, Some(Rgba::rgb(39, 174, 96)));
        assert_eq!(v.defaults.info_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Selection ===

    #[test]
    fn test_selection_colors() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.defaults.selection_background,
            Some(Rgba::rgb(61, 174, 233))
        );
        assert_eq!(
            v.defaults.selection_text_color,
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === Per-widget: Button ===

    #[test]
    fn test_button_background_from_colors_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.button.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_button_font_color_from_colors_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.button.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    // === Per-widget: Tooltip ===

    #[test]
    fn test_tooltip_background_from_colors_tooltip() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.tooltip.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_tooltip_font_color_from_colors_tooltip() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.tooltip.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === Per-widget: Sidebar (Complementary) ===

    #[test]
    fn test_sidebar_background_from_complementary() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));
    }

    #[test]
    fn test_sidebar_font_color_from_complementary() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.sidebar.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    // === KDE-01: Title bar from WM ===

    #[test]
    fn test_title_bar_background_from_wm_active() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.window.title_bar_background, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_title_bar_font_color_from_wm_active() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.title_bar_font.as_ref().and_then(|f| f.color),
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
    fn test_inactive_title_bar_text_color_from_wm() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.inactive_title_bar_text_color,
            Some(Rgba::rgb(161, 169, 177))
        );
    }

    // === KDE-02: Input placeholder and caret ===

    #[test]
    fn test_input_placeholder_from_view_foreground_inactive() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.input.placeholder_color, Some(Rgba::rgb(161, 169, 177)));
    }

    #[test]
    fn test_input_caret_from_view_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.input.caret_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === accent_text_color from Selection ===

    #[test]
    fn test_accent_text_color_from_selection() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.accent_text_color, Some(Rgba::rgb(252, 252, 252)));
    }

    // === KDE-02: List background/item_font from View ===

    #[test]
    fn test_list_background_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.background_color, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_list_item_font_color_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.list.item_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === KDE-02: List alternate_row ===

    #[test]
    fn test_list_alternate_row_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.alternate_row_background, Some(Rgba::rgb(30, 33, 36)));
    }

    // === KDE-02: List header from Header group ===

    #[test]
    fn test_list_header_background_from_header() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.header_background, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_list_header_font_color_from_header() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.list.header_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === KDE-02: Link visited ===

    #[test]
    fn test_link_visited_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.link.visited_text_color, Some(Rgba::rgb(155, 89, 182)));
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
        assert!(v.sidebar.background_color.is_none());
        assert!(v.sidebar.font.is_none());
    }

    // === Empty INI ===

    #[test]
    fn test_empty_ini() {
        let ini = super::super::create_kde_parser();
        let mut variant = ThemeMode::default();
        populate_colors(&ini, &mut variant);
        assert!(variant.defaults.accent_color.is_none());
        assert!(variant.defaults.background_color.is_none());
        assert!(variant.defaults.text_color.is_none());
        assert!(variant.button.background_color.is_none());
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
        assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(v.defaults.text_color, Some(Rgba::rgb(239, 240, 241)));
        assert_eq!(v.defaults.muted_color, Some(Rgba::rgb(161, 169, 177)));
        assert!(v.defaults.border.is_empty()); // border not set by reader
        assert!(v.defaults.accent_color.is_none());
        assert!(v.defaults.surface_color.is_none());
        assert!(v.button.background_color.is_none());
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
        assert!(v.defaults.surface_color.is_none()); // BackgroundNormal was malformed
        assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Full mapping: all non-None fields populated ===

    #[test]
    fn test_full_mapping_completeness() {
        let v = populate_fixture(BREEZE_DARK);

        // defaults-level
        assert!(v.defaults.accent_color.is_some(), "accent_color missing");
        assert!(
            v.defaults.background_color.is_some(),
            "background_color missing"
        );
        assert!(v.defaults.text_color.is_some(), "text_color missing");
        assert!(v.defaults.surface_color.is_some(), "surface_color missing");
        // border is not set by reader (preset value)
        assert!(v.defaults.muted_color.is_some(), "muted_color missing");
        assert!(v.defaults.link_color.is_some(), "link_color missing");
        assert!(
            v.defaults.selection_background.is_some(),
            "selection_background missing"
        );
        assert!(
            v.defaults.selection_text_color.is_some(),
            "selection_text_color missing"
        );
        assert!(
            v.defaults.accent_text_color.is_some(),
            "accent_text_color missing"
        );
        assert!(v.defaults.danger_color.is_some(), "danger_color missing");
        assert!(v.defaults.warning_color.is_some(), "warning_color missing");
        assert!(v.defaults.success_color.is_some(), "success_color missing");
        assert!(v.defaults.info_color.is_some(), "info_color missing");

        // per-widget
        assert!(
            v.button.background_color.is_some(),
            "button.background_color missing"
        );
        assert!(
            v.button.font.as_ref().and_then(|f| f.color).is_some(),
            "button.font.color missing"
        );
        assert!(
            v.tooltip.background_color.is_some(),
            "tooltip.background_color missing"
        );
        assert!(
            v.tooltip.font.as_ref().and_then(|f| f.color).is_some(),
            "tooltip.font.color missing"
        );
        assert!(
            v.sidebar.background_color.is_some(),
            "sidebar.background_color missing"
        );
        assert!(
            v.sidebar.font.as_ref().and_then(|f| f.color).is_some(),
            "sidebar.font.color missing"
        );
        assert!(
            v.input.background_color.is_some(),
            "input.background_color missing"
        );
        assert!(
            v.input.font.as_ref().and_then(|f| f.color).is_some(),
            "input.font.color missing"
        );
        assert!(
            v.input.placeholder_color.is_some(),
            "input.placeholder_color missing"
        );
        assert!(v.input.caret_color.is_some(), "input.caret_color missing");
        assert!(
            v.separator.line_color.is_some(),
            "separator.line_color missing"
        );
        assert!(
            v.list.background_color.is_some(),
            "list.background_color missing"
        );
        assert!(
            v.list.item_font.as_ref().and_then(|f| f.color).is_some(),
            "list.item_font.color missing"
        );
        assert!(
            v.list.alternate_row_background.is_some(),
            "list.alternate_row_background missing"
        );
        assert!(
            v.list.header_background.is_some(),
            "list.header_background missing"
        );
        assert!(
            v.list.header_font.as_ref().and_then(|f| f.color).is_some(),
            "list.header_font.color missing"
        );
        assert!(
            v.link.visited_text_color.is_some(),
            "link.visited_text_color missing"
        );

        // WM title bar
        assert!(
            v.window.title_bar_background.is_some(),
            "window.title_bar_background missing"
        );
        assert!(
            v.window
                .title_bar_font
                .as_ref()
                .and_then(|f| f.color)
                .is_some(),
            "window.title_bar_font.color missing"
        );
        assert!(
            v.window.inactive_title_bar_background.is_some(),
            "window.inactive_title_bar_background missing"
        );
        assert!(
            v.window.inactive_title_bar_text_color.is_some(),
            "window.inactive_title_bar_text_color missing"
        );
    }
}
