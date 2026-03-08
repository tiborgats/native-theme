// KDE color group parsing and mapping to ThemeColors
// Maps 36 semantic color roles from KDE INI color groups to ThemeColors.

use crate::model::colors::*;
use crate::Rgba;

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
        core: CoreColors {
            accent: get_color(ini, "Colors:View", "DecorationFocus"),
            background: get_color(ini, "Colors:Window", "BackgroundNormal"),
            foreground: window_fg,
            surface: get_color(ini, "Colors:View", "BackgroundNormal"),
            border: get_color(ini, "Colors:Window", "DecorationFocus"),
            muted: get_color(ini, "Colors:Window", "ForegroundInactive"),
            shadow: None, // KDE does not expose shadow color in kdeglobals
        },
        primary: ActionColors {
            background: get_color(ini, "Colors:Selection", "BackgroundNormal"),
            foreground: get_color(ini, "Colors:Selection", "ForegroundNormal"),
        },
        secondary: ActionColors {
            background: get_color(ini, "Colors:Button", "BackgroundNormal"),
            foreground: get_color(ini, "Colors:Button", "ForegroundNormal"),
        },
        status: StatusColors {
            danger: get_color(ini, "Colors:View", "ForegroundNegative"),
            danger_foreground: window_fg,
            warning: get_color(ini, "Colors:View", "ForegroundNeutral"),
            warning_foreground: window_fg,
            success: get_color(ini, "Colors:View", "ForegroundPositive"),
            success_foreground: window_fg,
            info: get_color(ini, "Colors:View", "ForegroundActive"),
            info_foreground: window_fg,
        },
        interactive: InteractiveColors {
            selection: get_color(ini, "Colors:Selection", "BackgroundNormal"),
            selection_foreground: get_color(ini, "Colors:Selection", "ForegroundNormal"),
            link: get_color(ini, "Colors:View", "ForegroundLink"),
            focus_ring: get_color(ini, "Colors:View", "DecorationFocus"),
        },
        panel: PanelColors {
            sidebar: get_color(ini, "Colors:Complementary", "BackgroundNormal"),
            sidebar_foreground: get_color(ini, "Colors:Complementary", "ForegroundNormal"),
            tooltip: get_color(ini, "Colors:Tooltip", "BackgroundNormal"),
            tooltip_foreground: get_color(ini, "Colors:Tooltip", "ForegroundNormal"),
            popover: get_color(ini, "Colors:View", "BackgroundNormal"),
            popover_foreground: get_color(ini, "Colors:View", "ForegroundNormal"),
        },
        component: ComponentColors {
            button: get_color(ini, "Colors:Button", "BackgroundNormal"),
            button_foreground: get_color(ini, "Colors:Button", "ForegroundNormal"),
            input: get_color(ini, "Colors:View", "BackgroundNormal"),
            input_foreground: get_color(ini, "Colors:View", "ForegroundNormal"),
            disabled: get_color(ini, "Colors:View", "ForegroundInactive"),
            separator: get_color(ini, "Colors:Window", "ForegroundInactive"),
            alternate_row: get_color(ini, "Colors:View", "BackgroundAlternate"),
        },
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
        assert!(colors.core.accent.is_some(), "core.accent missing");
        assert!(colors.core.background.is_some(), "core.background missing");
        assert!(colors.core.foreground.is_some(), "core.foreground missing");
        assert!(colors.core.surface.is_some(), "core.surface missing");
        assert!(colors.core.border.is_some(), "core.border missing");
        assert!(colors.core.muted.is_some(), "core.muted missing");
        assert!(colors.core.shadow.is_none(), "core.shadow should be None");

        assert!(colors.primary.background.is_some(), "primary.background missing");
        assert!(colors.primary.foreground.is_some(), "primary.foreground missing");
        assert!(colors.secondary.background.is_some(), "secondary.background missing");
        assert!(colors.secondary.foreground.is_some(), "secondary.foreground missing");

        assert!(colors.status.danger.is_some(), "status.danger missing");
        assert!(colors.status.danger_foreground.is_some(), "status.danger_foreground missing");
        assert!(colors.status.warning.is_some(), "status.warning missing");
        assert!(colors.status.warning_foreground.is_some(), "status.warning_foreground missing");
        assert!(colors.status.success.is_some(), "status.success missing");
        assert!(colors.status.success_foreground.is_some(), "status.success_foreground missing");
        assert!(colors.status.info.is_some(), "status.info missing");
        assert!(colors.status.info_foreground.is_some(), "status.info_foreground missing");

        assert!(colors.interactive.selection.is_some(), "interactive.selection missing");
        assert!(colors.interactive.selection_foreground.is_some(), "interactive.selection_foreground missing");
        assert!(colors.interactive.link.is_some(), "interactive.link missing");
        assert!(colors.interactive.focus_ring.is_some(), "interactive.focus_ring missing");

        assert!(colors.panel.sidebar.is_some(), "panel.sidebar missing");
        assert!(colors.panel.sidebar_foreground.is_some(), "panel.sidebar_foreground missing");
        assert!(colors.panel.tooltip.is_some(), "panel.tooltip missing");
        assert!(colors.panel.tooltip_foreground.is_some(), "panel.tooltip_foreground missing");
        assert!(colors.panel.popover.is_some(), "panel.popover missing");
        assert!(colors.panel.popover_foreground.is_some(), "panel.popover_foreground missing");

        assert!(colors.component.button.is_some(), "component.button missing");
        assert!(colors.component.button_foreground.is_some(), "component.button_foreground missing");
        assert!(colors.component.input.is_some(), "component.input missing");
        assert!(colors.component.input_foreground.is_some(), "component.input_foreground missing");
        assert!(colors.component.disabled.is_some(), "component.disabled missing");
        assert!(colors.component.separator.is_some(), "component.separator missing");
        assert!(colors.component.alternate_row.is_some(), "component.alternate_row missing");
    }

    // === Core color mapping ===

    #[test]
    fn test_core_colors_mapping() {
        let colors = parse_fixture(BREEZE_DARK);
        // accent <- Colors:View/DecorationFocus = 61,174,233
        assert_eq!(colors.core.accent, Some(Rgba::rgb(61, 174, 233)));
        // background <- Colors:Window/BackgroundNormal = 49,54,59
        assert_eq!(colors.core.background, Some(Rgba::rgb(49, 54, 59)));
        // foreground <- Colors:Window/ForegroundNormal = 239,240,241
        assert_eq!(colors.core.foreground, Some(Rgba::rgb(239, 240, 241)));
        // surface <- Colors:View/BackgroundNormal = 35,38,41
        assert_eq!(colors.core.surface, Some(Rgba::rgb(35, 38, 41)));
        // border <- Colors:Window/DecorationFocus = 61,174,233
        assert_eq!(colors.core.border, Some(Rgba::rgb(61, 174, 233)));
        // muted <- Colors:Window/ForegroundInactive = 161,169,177
        assert_eq!(colors.core.muted, Some(Rgba::rgb(161, 169, 177)));
    }

    // === Status color mapping ===

    #[test]
    fn test_status_colors_mapping() {
        let colors = parse_fixture(BREEZE_DARK);
        // danger <- Colors:View/ForegroundNegative = 218,68,83
        assert_eq!(colors.status.danger, Some(Rgba::rgb(218, 68, 83)));
        // warning <- Colors:View/ForegroundNeutral = 246,116,0
        assert_eq!(colors.status.warning, Some(Rgba::rgb(246, 116, 0)));
        // success <- Colors:View/ForegroundPositive = 39,174,96
        assert_eq!(colors.status.success, Some(Rgba::rgb(39, 174, 96)));
        // info <- Colors:View/ForegroundActive = 61,174,233
        assert_eq!(colors.status.info, Some(Rgba::rgb(61, 174, 233)));
        // All _foreground fields <- Colors:Window/ForegroundNormal = 239,240,241
        let fg = Some(Rgba::rgb(239, 240, 241));
        assert_eq!(colors.status.danger_foreground, fg);
        assert_eq!(colors.status.warning_foreground, fg);
        assert_eq!(colors.status.success_foreground, fg);
        assert_eq!(colors.status.info_foreground, fg);
    }

    // === Interactive color mapping ===

    #[test]
    fn test_interactive_colors_mapping() {
        let colors = parse_fixture(BREEZE_DARK);
        // selection <- Colors:Selection/BackgroundNormal = 61,174,233
        assert_eq!(colors.interactive.selection, Some(Rgba::rgb(61, 174, 233)));
        // selection_foreground <- Colors:Selection/ForegroundNormal = 252,252,252
        assert_eq!(colors.interactive.selection_foreground, Some(Rgba::rgb(252, 252, 252)));
        // link <- Colors:View/ForegroundLink = 29,153,243
        assert_eq!(colors.interactive.link, Some(Rgba::rgb(29, 153, 243)));
        // focus_ring <- Colors:View/DecorationFocus = 61,174,233
        assert_eq!(colors.interactive.focus_ring, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Panel sidebar from Complementary ===

    #[test]
    fn test_panel_sidebar_from_complementary() {
        let colors = parse_fixture(BREEZE_DARK);
        // sidebar <- Colors:Complementary/BackgroundNormal = 42,46,50
        assert_eq!(colors.panel.sidebar, Some(Rgba::rgb(42, 46, 50)));
        // sidebar_foreground <- Colors:Complementary/ForegroundNormal = 239,240,241
        assert_eq!(colors.panel.sidebar_foreground, Some(Rgba::rgb(239, 240, 241)));
        // tooltip <- Colors:Tooltip/BackgroundNormal = 49,54,59
        assert_eq!(colors.panel.tooltip, Some(Rgba::rgb(49, 54, 59)));
        // tooltip_foreground <- Colors:Tooltip/ForegroundNormal = 252,252,252
        assert_eq!(colors.panel.tooltip_foreground, Some(Rgba::rgb(252, 252, 252)));
    }

    // === Component color mapping ===

    #[test]
    fn test_component_colors_mapping() {
        let colors = parse_fixture(BREEZE_DARK);
        // button <- Colors:Button/BackgroundNormal = 49,54,59
        assert_eq!(colors.component.button, Some(Rgba::rgb(49, 54, 59)));
        // button_foreground <- Colors:Button/ForegroundNormal = 239,240,241
        assert_eq!(colors.component.button_foreground, Some(Rgba::rgb(239, 240, 241)));
        // input <- Colors:View/BackgroundNormal = 35,38,41
        assert_eq!(colors.component.input, Some(Rgba::rgb(35, 38, 41)));
        // input_foreground <- Colors:View/ForegroundNormal = 252,252,252
        assert_eq!(colors.component.input_foreground, Some(Rgba::rgb(252, 252, 252)));
        // disabled <- Colors:View/ForegroundInactive = 161,169,177
        assert_eq!(colors.component.disabled, Some(Rgba::rgb(161, 169, 177)));
        // separator <- Colors:Window/ForegroundInactive = 161,169,177
        assert_eq!(colors.component.separator, Some(Rgba::rgb(161, 169, 177)));
        // alternate_row <- Colors:View/BackgroundAlternate = 30,33,36
        assert_eq!(colors.component.alternate_row, Some(Rgba::rgb(30, 33, 36)));
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
        assert!(colors.panel.sidebar.is_none());
        assert!(colors.panel.sidebar_foreground.is_none());
    }

    // === Empty INI ===

    #[test]
    fn test_empty_ini() {
        let ini = super::super::create_kde_parser();
        let colors = parse_colors(&ini);
        assert!(colors.core.accent.is_none());
        assert!(colors.core.background.is_none());
        assert!(colors.core.foreground.is_none());
        assert!(colors.primary.background.is_none());
        assert!(colors.component.button.is_none());
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
        assert_eq!(colors.core.background, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(colors.core.foreground, Some(Rgba::rgb(239, 240, 241)));
        assert_eq!(colors.core.muted, Some(Rgba::rgb(161, 169, 177)));
        assert_eq!(colors.core.border, Some(Rgba::rgb(61, 174, 233)));
        // Non-Window fields should be None
        assert!(colors.core.accent.is_none()); // View/DecorationFocus
        assert!(colors.core.surface.is_none()); // View/BackgroundNormal
        assert!(colors.primary.background.is_none()); // Selection
        assert!(colors.component.button.is_none()); // Button
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
        assert!(colors.core.surface.is_none()); // BackgroundNormal was malformed
        // Other values still parse
        assert_eq!(colors.core.accent, Some(Rgba::rgb(61, 174, 233))); // DecorationFocus ok
    }
}
