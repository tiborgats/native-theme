// KDE color group parsing and mapping to ThemeColors
// Maps 36 semantic color roles from KDE INI color groups to ThemeColors.

use crate::Rgba;
use crate::model::colors::ThemeColors;

/// Look up a color key from a KDE INI section and parse it as RGB.
fn get_color(ini: &configparser::ini::Ini, section: &str, key: &str) -> Option<Rgba> {
    let value = ini.get(section, key)?;
    super::parse_rgb(&value)
}

/// Parse KDE color groups into a unified `ThemeColors`.
///
/// Maps all 35 populated color roles (shadow is always None) from the
/// standard KDE color groups: View, Window, Button, Selection, Tooltip,
/// and the optional Complementary group.
pub(crate) fn parse_colors(ini: &configparser::ini::Ini) -> ThemeColors {
    let window_fg = get_color(ini, "Colors:Window", "ForegroundNormal");

    ThemeColors {
        accent: get_color(ini, "Colors:View", "DecorationFocus"),
        background: get_color(ini, "Colors:Window", "BackgroundNormal"),
        foreground: window_fg,
        surface: get_color(ini, "Colors:View", "BackgroundNormal"),
        border: get_color(ini, "Colors:Window", "DecorationFocus"),
        muted: get_color(ini, "Colors:Window", "ForegroundInactive"),
        shadow: None, // KDE does not expose shadow color in kdeglobals
        primary_background: get_color(ini, "Colors:Selection", "BackgroundNormal"),
        primary_foreground: get_color(ini, "Colors:Selection", "ForegroundNormal"),
        secondary_background: get_color(ini, "Colors:Button", "BackgroundNormal"),
        secondary_foreground: get_color(ini, "Colors:Button", "ForegroundNormal"),
        danger: get_color(ini, "Colors:View", "ForegroundNegative"),
        danger_foreground: window_fg,
        warning: get_color(ini, "Colors:View", "ForegroundNeutral"),
        warning_foreground: window_fg,
        success: get_color(ini, "Colors:View", "ForegroundPositive"),
        success_foreground: window_fg,
        info: get_color(ini, "Colors:View", "ForegroundActive"),
        info_foreground: window_fg,
        selection: get_color(ini, "Colors:Selection", "BackgroundNormal"),
        selection_foreground: get_color(ini, "Colors:Selection", "ForegroundNormal"),
        link: get_color(ini, "Colors:View", "ForegroundLink"),
        focus_ring: get_color(ini, "Colors:View", "DecorationFocus"),
        sidebar: get_color(ini, "Colors:Complementary", "BackgroundNormal"),
        sidebar_foreground: get_color(ini, "Colors:Complementary", "ForegroundNormal"),
        tooltip: get_color(ini, "Colors:Tooltip", "BackgroundNormal"),
        tooltip_foreground: get_color(ini, "Colors:Tooltip", "ForegroundNormal"),
        popover: get_color(ini, "Colors:View", "BackgroundNormal"),
        popover_foreground: get_color(ini, "Colors:View", "ForegroundNormal"),
        button: get_color(ini, "Colors:Button", "BackgroundNormal"),
        button_foreground: get_color(ini, "Colors:Button", "ForegroundNormal"),
        input: get_color(ini, "Colors:View", "BackgroundNormal"),
        input_foreground: get_color(ini, "Colors:View", "ForegroundNormal"),
        disabled: get_color(ini, "Colors:View", "ForegroundInactive"),
        separator: get_color(ini, "Colors:Window", "ForegroundInactive"),
        alternate_row: get_color(ini, "Colors:View", "BackgroundAlternate"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Full Breeze Dark kdeglobals fixture with all 6 color groups.
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
";

    fn parse_fixture(content: &str) -> ThemeColors {
        let mut ini = super::super::create_kde_parser();
        ini.read(content.to_string()).unwrap();
        parse_colors(&ini)
    }

    // === Full mapping test ===

    #[test]
    fn test_full_mapping() {
        let colors = parse_fixture(BREEZE_DARK);

        // All 35 non-shadow fields should be Some
        assert!(colors.accent.is_some(), "accent missing");
        assert!(colors.background.is_some(), "background missing");
        assert!(colors.foreground.is_some(), "foreground missing");
        assert!(colors.surface.is_some(), "surface missing");
        assert!(colors.border.is_some(), "border missing");
        assert!(colors.muted.is_some(), "muted missing");
        assert!(colors.shadow.is_none(), "shadow should be None");

        assert!(
            colors.primary_background.is_some(),
            "primary_background missing"
        );
        assert!(
            colors.primary_foreground.is_some(),
            "primary_foreground missing"
        );
        assert!(
            colors.secondary_background.is_some(),
            "secondary_background missing"
        );
        assert!(
            colors.secondary_foreground.is_some(),
            "secondary_foreground missing"
        );

        assert!(colors.danger.is_some(), "danger missing");
        assert!(
            colors.danger_foreground.is_some(),
            "danger_foreground missing"
        );
        assert!(colors.warning.is_some(), "warning missing");
        assert!(
            colors.warning_foreground.is_some(),
            "warning_foreground missing"
        );
        assert!(colors.success.is_some(), "success missing");
        assert!(
            colors.success_foreground.is_some(),
            "success_foreground missing"
        );
        assert!(colors.info.is_some(), "info missing");
        assert!(colors.info_foreground.is_some(), "info_foreground missing");

        assert!(colors.selection.is_some(), "selection missing");
        assert!(
            colors.selection_foreground.is_some(),
            "selection_foreground missing"
        );
        assert!(colors.link.is_some(), "link missing");
        assert!(colors.focus_ring.is_some(), "focus_ring missing");

        assert!(colors.sidebar.is_some(), "sidebar missing");
        assert!(
            colors.sidebar_foreground.is_some(),
            "sidebar_foreground missing"
        );
        assert!(colors.tooltip.is_some(), "tooltip missing");
        assert!(
            colors.tooltip_foreground.is_some(),
            "tooltip_foreground missing"
        );
        assert!(colors.popover.is_some(), "popover missing");
        assert!(
            colors.popover_foreground.is_some(),
            "popover_foreground missing"
        );

        assert!(colors.button.is_some(), "button missing");
        assert!(
            colors.button_foreground.is_some(),
            "button_foreground missing"
        );
        assert!(colors.input.is_some(), "input missing");
        assert!(
            colors.input_foreground.is_some(),
            "input_foreground missing"
        );
        assert!(colors.disabled.is_some(), "disabled missing");
        assert!(colors.separator.is_some(), "separator missing");
        assert!(colors.alternate_row.is_some(), "alternate_row missing");
    }

    // === Core color mapping ===

    #[test]
    fn test_core_colors_mapping() {
        let colors = parse_fixture(BREEZE_DARK);
        // accent <- Colors:View/DecorationFocus = 61,174,233
        assert_eq!(colors.accent, Some(Rgba::rgb(61, 174, 233)));
        // background <- Colors:Window/BackgroundNormal = 49,54,59
        assert_eq!(colors.background, Some(Rgba::rgb(49, 54, 59)));
        // foreground <- Colors:Window/ForegroundNormal = 239,240,241
        assert_eq!(colors.foreground, Some(Rgba::rgb(239, 240, 241)));
        // surface <- Colors:View/BackgroundNormal = 35,38,41
        assert_eq!(colors.surface, Some(Rgba::rgb(35, 38, 41)));
        // border <- Colors:Window/DecorationFocus = 61,174,233
        assert_eq!(colors.border, Some(Rgba::rgb(61, 174, 233)));
        // muted <- Colors:Window/ForegroundInactive = 161,169,177
        assert_eq!(colors.muted, Some(Rgba::rgb(161, 169, 177)));
    }

    // === Status color mapping ===

    #[test]
    fn test_status_colors_mapping() {
        let colors = parse_fixture(BREEZE_DARK);
        // danger <- Colors:View/ForegroundNegative = 218,68,83
        assert_eq!(colors.danger, Some(Rgba::rgb(218, 68, 83)));
        // warning <- Colors:View/ForegroundNeutral = 246,116,0
        assert_eq!(colors.warning, Some(Rgba::rgb(246, 116, 0)));
        // success <- Colors:View/ForegroundPositive = 39,174,96
        assert_eq!(colors.success, Some(Rgba::rgb(39, 174, 96)));
        // info <- Colors:View/ForegroundActive = 61,174,233
        assert_eq!(colors.info, Some(Rgba::rgb(61, 174, 233)));
        // All _foreground fields <- Colors:Window/ForegroundNormal = 239,240,241
        let fg = Some(Rgba::rgb(239, 240, 241));
        assert_eq!(colors.danger_foreground, fg);
        assert_eq!(colors.warning_foreground, fg);
        assert_eq!(colors.success_foreground, fg);
        assert_eq!(colors.info_foreground, fg);
    }

    // === Interactive color mapping ===

    #[test]
    fn test_interactive_colors_mapping() {
        let colors = parse_fixture(BREEZE_DARK);
        // selection <- Colors:Selection/BackgroundNormal = 61,174,233
        assert_eq!(colors.selection, Some(Rgba::rgb(61, 174, 233)));
        // selection_foreground <- Colors:Selection/ForegroundNormal = 252,252,252
        assert_eq!(colors.selection_foreground, Some(Rgba::rgb(252, 252, 252)));
        // link <- Colors:View/ForegroundLink = 29,153,243
        assert_eq!(colors.link, Some(Rgba::rgb(29, 153, 243)));
        // focus_ring <- Colors:View/DecorationFocus = 61,174,233
        assert_eq!(colors.focus_ring, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Panel sidebar from Complementary ===

    #[test]
    fn test_panel_sidebar_from_complementary() {
        let colors = parse_fixture(BREEZE_DARK);
        // sidebar <- Colors:Complementary/BackgroundNormal = 42,46,50
        assert_eq!(colors.sidebar, Some(Rgba::rgb(42, 46, 50)));
        // sidebar_foreground <- Colors:Complementary/ForegroundNormal = 239,240,241
        assert_eq!(colors.sidebar_foreground, Some(Rgba::rgb(239, 240, 241)));
        // tooltip <- Colors:Tooltip/BackgroundNormal = 49,54,59
        assert_eq!(colors.tooltip, Some(Rgba::rgb(49, 54, 59)));
        // tooltip_foreground <- Colors:Tooltip/ForegroundNormal = 252,252,252
        assert_eq!(colors.tooltip_foreground, Some(Rgba::rgb(252, 252, 252)));
    }

    // === Component color mapping ===

    #[test]
    fn test_component_colors_mapping() {
        let colors = parse_fixture(BREEZE_DARK);
        // button <- Colors:Button/BackgroundNormal = 49,54,59
        assert_eq!(colors.button, Some(Rgba::rgb(49, 54, 59)));
        // button_foreground <- Colors:Button/ForegroundNormal = 239,240,241
        assert_eq!(colors.button_foreground, Some(Rgba::rgb(239, 240, 241)));
        // input <- Colors:View/BackgroundNormal = 35,38,41
        assert_eq!(colors.input, Some(Rgba::rgb(35, 38, 41)));
        // input_foreground <- Colors:View/ForegroundNormal = 252,252,252
        assert_eq!(colors.input_foreground, Some(Rgba::rgb(252, 252, 252)));
        // disabled <- Colors:View/ForegroundInactive = 161,169,177
        assert_eq!(colors.disabled, Some(Rgba::rgb(161, 169, 177)));
        // separator <- Colors:Window/ForegroundInactive = 161,169,177
        assert_eq!(colors.separator, Some(Rgba::rgb(161, 169, 177)));
        // alternate_row <- Colors:View/BackgroundAlternate = 30,33,36
        assert_eq!(colors.alternate_row, Some(Rgba::rgb(30, 33, 36)));
    }

    // === Sidebar None when Complementary missing ===

    #[test]
    fn test_sidebar_none_without_complementary() {
        let content = "\
[Colors:View]
BackgroundNormal=35,38,41
ForegroundNormal=252,252,252
";
        let colors = parse_fixture(content);
        assert!(colors.sidebar.is_none());
        assert!(colors.sidebar_foreground.is_none());
    }

    // === Empty INI ===

    #[test]
    fn test_empty_ini() {
        let ini = super::super::create_kde_parser();
        let colors = parse_colors(&ini);
        assert!(colors.accent.is_none());
        assert!(colors.background.is_none());
        assert!(colors.foreground.is_none());
        assert!(colors.primary_background.is_none());
        assert!(colors.button.is_none());
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
        let colors = parse_fixture(content);
        // Window-sourced fields should be populated
        assert_eq!(colors.background, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(colors.foreground, Some(Rgba::rgb(239, 240, 241)));
        assert_eq!(colors.muted, Some(Rgba::rgb(161, 169, 177)));
        assert_eq!(colors.border, Some(Rgba::rgb(61, 174, 233)));
        // Non-Window fields should be None
        assert!(colors.accent.is_none()); // View/DecorationFocus
        assert!(colors.surface.is_none()); // View/BackgroundNormal
        assert!(colors.primary_background.is_none()); // Selection
        assert!(colors.button.is_none()); // Button
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
        let colors = parse_fixture(content);
        // Malformed value -> None
        assert!(colors.surface.is_none()); // BackgroundNormal was malformed
        // Other values still parse
        assert_eq!(colors.accent, Some(Rgba::rgb(61, 174, 233))); // DecorationFocus ok
    }
}
