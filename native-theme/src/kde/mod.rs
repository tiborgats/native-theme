// KDE theme reader -- reads kdeglobals INI file and maps to NativeTheme

pub mod colors;
pub mod fonts;
pub mod metrics;

use crate::Rgba;

/// Parse a KDE kdeglobals content string into a NativeTheme.
///
/// Internal helper that encapsulates all parsing logic for testability
/// without requiring filesystem access.
pub(crate) fn from_kde_content(content: &str) -> crate::Result<crate::NativeTheme> {
    let mut ini = create_kde_parser();
    ini.read(content.to_string())
        .map_err(|e| crate::Error::Format(e))?;

    let theme_colors = colors::parse_colors(&ini);
    let theme_fonts = fonts::parse_fonts(&ini);
    let dark = is_dark_theme(&ini);

    let name = ini
        .get("General", "ColorScheme")
        .unwrap_or_else(|| "KDE".to_string());

    let variant = crate::ThemeVariant {
        colors: theme_colors,
        fonts: theme_fonts,
        geometry: Default::default(),
        spacing: Default::default(),
        widget_metrics: Some(metrics::breeze_widget_metrics()),
        icon_set: None,
    };

    let theme = if dark {
        crate::NativeTheme {
            name,
            light: None,
            dark: Some(variant),
        }
    } else {
        crate::NativeTheme {
            name,
            light: Some(variant),
            dark: None,
        }
    };

    Ok(theme)
}

/// Read the current KDE theme from kdeglobals.
///
/// Parses `~/.config/kdeglobals` (respecting `XDG_CONFIG_HOME`) and maps
/// KDE color groups and font strings to a `NativeTheme`.
pub fn from_kde() -> crate::Result<crate::NativeTheme> {
    let path = kdeglobals_path();
    let content = std::fs::read_to_string(&path)
        .map_err(|e| crate::Error::Unavailable(format!("cannot read {}: {e}", path.display())))?;
    from_kde_content(&content)
}

/// Create a configparser Ini instance configured for KDE files.
///
/// Uses case-sensitive mode and equals-only delimiter to correctly
/// handle KDE's PascalCase keys and colon-containing section names.
pub(crate) fn create_kde_parser() -> configparser::ini::Ini {
    let tmp = configparser::ini::Ini::new_cs();
    let mut defaults = tmp.defaults();
    defaults.delimiters = vec!['='];
    configparser::ini::Ini::new_from_defaults(defaults)
}

/// Parse a KDE "R,G,B" color string into an Rgba (opaque).
///
/// Returns None for malformed values (never panics).
/// Exactly 3 comma-separated u8 components required.
pub(crate) fn parse_rgb(value: &str) -> Option<Rgba> {
    let parts: Vec<&str> = value.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    Some(Rgba::rgb(r, g, b))
}

/// Resolve the path to the kdeglobals file.
///
/// Checks XDG_CONFIG_HOME (non-empty), then $HOME/.config/kdeglobals,
/// then /etc/xdg/kdeglobals as last resort.
pub(crate) fn kdeglobals_path() -> std::path::PathBuf {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        if !config_home.is_empty() {
            return std::path::PathBuf::from(config_home).join("kdeglobals");
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home)
            .join(".config")
            .join("kdeglobals");
    }
    // Last resort fallback
    std::path::PathBuf::from("/etc/xdg/kdeglobals")
}

/// Detect whether the active KDE theme is dark based on background luminance.
///
/// Uses BT.601 luminance coefficients on Colors:Window/BackgroundNormal.
/// Defaults to false (light) if the section/key is missing.
pub(crate) fn is_dark_theme(ini: &configparser::ini::Ini) -> bool {
    if let Some(bg_str) = ini.get("Colors:Window", "BackgroundNormal") {
        if let Some(bg) = parse_rgb(&bg_str) {
            let luma = 0.299 * (bg.r as f32) + 0.587 * (bg.g as f32) + 0.114 * (bg.b as f32);
            return luma < 128.0;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;

    // === parse_rgb tests ===

    #[test]
    fn parse_rgb_valid_breeze_accent() {
        assert_eq!(parse_rgb("61,174,233"), Some(Rgba::rgb(61, 174, 233)));
    }

    #[test]
    fn parse_rgb_handles_whitespace() {
        assert_eq!(parse_rgb("0, 0, 0"), Some(Rgba::rgb(0, 0, 0)));
    }

    #[test]
    fn parse_rgb_white() {
        assert_eq!(parse_rgb("255,255,255"), Some(Rgba::rgb(255, 255, 255)));
    }

    #[test]
    fn parse_rgb_invalid_text() {
        assert_eq!(parse_rgb("invalid"), None);
    }

    #[test]
    fn parse_rgb_too_few_components() {
        assert_eq!(parse_rgb("1,2"), None);
    }

    #[test]
    fn parse_rgb_too_many_components() {
        assert_eq!(parse_rgb("1,2,3,4"), None);
    }

    #[test]
    fn parse_rgb_out_of_u8_range() {
        assert_eq!(parse_rgb("256,0,0"), None);
    }

    #[test]
    fn parse_rgb_empty_string() {
        assert_eq!(parse_rgb(""), None);
    }

    // === kdeglobals_path tests ===

    #[test]
    #[allow(unsafe_code)]
    fn kdeglobals_path_respects_xdg_config_home() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        // SAFETY: ENV_MUTEX serializes env var access across parallel tests
        unsafe { std::env::set_var("XDG_CONFIG_HOME", "/tmp/test") };
        let path = kdeglobals_path();
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
        assert_eq!(path, std::path::PathBuf::from("/tmp/test/kdeglobals"));
    }

    #[test]
    #[allow(unsafe_code)]
    fn kdeglobals_path_falls_back_to_home_config() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        // SAFETY: ENV_MUTEX serializes env var access across parallel tests
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
        let path = kdeglobals_path();
        let home = std::env::var("HOME").expect("HOME must be set for this test");
        assert_eq!(
            path,
            std::path::PathBuf::from(home)
                .join(".config")
                .join("kdeglobals")
        );
    }

    // === is_dark_theme tests ===

    #[test]
    fn is_dark_theme_detects_breeze_dark() {
        let mut ini = create_kde_parser();
        ini.read("[Colors:Window]\nBackgroundNormal=20,22,24\n".to_string())
            .unwrap();
        assert!(is_dark_theme(&ini));
    }

    #[test]
    fn is_dark_theme_detects_breeze_light() {
        let mut ini = create_kde_parser();
        ini.read("[Colors:Window]\nBackgroundNormal=239,240,241\n".to_string())
            .unwrap();
        assert!(!is_dark_theme(&ini));
    }

    #[test]
    fn is_dark_theme_defaults_false_when_missing() {
        let ini = create_kde_parser();
        assert!(!is_dark_theme(&ini));
    }

    // === create_kde_parser tests ===

    #[test]
    fn create_kde_parser_is_case_sensitive() {
        let mut ini = create_kde_parser();
        ini.read("[Colors:View]\nBackgroundNormal=255,255,255\n".to_string())
            .unwrap();
        // Case-sensitive: PascalCase key should be retrievable as-is
        assert!(ini.get("Colors:View", "BackgroundNormal").is_some());
    }

    #[test]
    fn create_kde_parser_preserves_section_colons() {
        let mut ini = create_kde_parser();
        ini.read("[Colors:Window]\nForegroundNormal=0,0,0\n".to_string())
            .unwrap();
        // Section name with colon must be preserved
        assert!(ini.get("Colors:Window", "ForegroundNormal").is_some());
    }

    #[test]
    fn create_kde_parser_equals_only_delimiter() {
        let mut ini = create_kde_parser();
        // ':' should NOT be a delimiter -- this line should be parsed
        // as section [Test] with no key-value pairs containing ':'
        ini.read("[Test]\nsome:value=actual\n".to_string()).unwrap();
        // If ':' were a delimiter, 'some' would be a key with value 'value=actual'
        // With '=' only, 'some:value' is the key and 'actual' is the value
        assert_eq!(ini.get("Test", "some:value"), Some("actual".to_string()));
    }

    // === from_kde_content / from_kde integration tests ===

    /// Full Breeze Dark kdeglobals fixture with all sections.
    const BREEZE_DARK_FULL: &str = "\
[General]
ColorScheme=BreezeDark
font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
fixed=Hack,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1

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

    /// Light theme fixture with light background colors.
    const BREEZE_LIGHT_FULL: &str = "\
[General]
ColorScheme=BreezeLight

[Colors:View]
BackgroundNormal=255,255,255
ForegroundNormal=35,38,41
DecorationFocus=61,174,233

[Colors:Window]
BackgroundNormal=239,240,241
ForegroundNormal=35,38,41
ForegroundInactive=127,140,141
DecorationFocus=61,174,233

[Colors:Button]
BackgroundNormal=239,240,241
ForegroundNormal=35,38,41

[Colors:Selection]
BackgroundNormal=61,174,233
ForegroundNormal=255,255,255

[Colors:Tooltip]
BackgroundNormal=247,247,247
ForegroundNormal=35,38,41
";

    /// Minimal fixture -- only Colors:Window section.
    const MINIMAL_FIXTURE: &str = "\
[Colors:Window]
BackgroundNormal=49,54,59
";

    #[test]
    fn test_dark_theme_detection() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        assert!(theme.dark.is_some(), "dark variant should be populated");
        assert!(
            theme.light.is_none(),
            "light variant should be None for dark theme"
        );
    }

    #[test]
    fn test_light_theme_detection() {
        let theme = from_kde_content(BREEZE_LIGHT_FULL).unwrap();
        assert!(theme.light.is_some(), "light variant should be populated");
        assert!(
            theme.dark.is_none(),
            "dark variant should be None for light theme"
        );
    }

    #[test]
    fn test_theme_name_from_colorscheme() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        assert_eq!(theme.name, "BreezeDark");
    }

    #[test]
    fn test_theme_name_fallback() {
        let content = "[Colors:Window]\nBackgroundNormal=49,54,59\n";
        let theme = from_kde_content(content).unwrap();
        assert_eq!(theme.name, "KDE");
    }

    #[test]
    fn test_colors_populated() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = theme.dark.as_ref().unwrap();
        assert!(variant.colors.accent.is_some());
        assert!(variant.colors.background.is_some());
        assert!(variant.colors.foreground.is_some());
    }

    #[test]
    fn test_fonts_populated() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = theme.dark.as_ref().unwrap();
        assert_eq!(variant.fonts.family.as_deref(), Some("Noto Sans"));
        assert_eq!(variant.fonts.size, Some(10.0));
        assert_eq!(variant.fonts.mono_family.as_deref(), Some("Hack"));
        assert_eq!(variant.fonts.mono_size, Some(10.0));
    }

    #[test]
    fn test_minimal_fixture_no_panic() {
        let result = from_kde_content(MINIMAL_FIXTURE);
        assert!(
            result.is_ok(),
            "minimal fixture should not panic: {result:?}"
        );
        let theme = result.unwrap();
        // Should have a dark variant (49,54,59 is dark)
        assert!(theme.dark.is_some());
    }

    #[test]
    fn test_from_kde_content_populates_widget_metrics() {
        let theme = from_kde_content(BREEZE_DARK_FULL).unwrap();
        let variant = theme.dark.as_ref().unwrap();
        assert!(
            variant.widget_metrics.is_some(),
            "widget_metrics should be populated from breeze constants"
        );
        let wm = variant.widget_metrics.as_ref().unwrap();
        assert_eq!(wm.button.min_width, Some(80.0), "Button_MinWidth");
    }

    #[test]
    fn test_empty_content() {
        let result = from_kde_content("");
        // configparser accepts empty input as empty ini
        assert!(
            result.is_ok(),
            "empty content should produce Ok: {result:?}"
        );
        let theme = result.unwrap();
        assert_eq!(theme.name, "KDE"); // fallback name
    }

    #[test]
    #[allow(unsafe_code)]
    fn test_missing_file() {
        let _guard = crate::ENV_MUTEX.lock().unwrap();
        // SAFETY: ENV_MUTEX serializes env var access across parallel tests
        let original_xdg = std::env::var("XDG_CONFIG_HOME").ok();
        unsafe { std::env::set_var("XDG_CONFIG_HOME", "/tmp/nonexistent_kde_test_dir_12345") };

        let result = from_kde();
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::Unavailable(msg) => {
                assert!(
                    msg.contains("kdeglobals") || msg.contains("cannot read"),
                    "unexpected error message: {msg}"
                );
            }
            other => panic!("expected Error::Unavailable, got: {other:?}"),
        }

        // Restore
        match original_xdg {
            Some(val) => unsafe { std::env::set_var("XDG_CONFIG_HOME", val) },
            None => unsafe { std::env::remove_var("XDG_CONFIG_HOME") },
        }
    }
}
