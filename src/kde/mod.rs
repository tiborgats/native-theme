// KDE theme reader -- reads kdeglobals INI file and maps to NativeTheme

pub mod colors;
pub mod fonts;

use crate::Rgba;

/// Read the current KDE theme from kdeglobals.
///
/// Parses `~/.config/kdeglobals` (respecting `XDG_CONFIG_HOME`) and maps
/// KDE color groups and font strings to a `NativeTheme`.
pub fn from_kde() -> crate::Result<crate::NativeTheme> {
    todo!("Plan 02 implements the full body")
}

/// Create a configparser Ini instance configured for KDE files.
///
/// Uses case-sensitive mode and equals-only delimiter to correctly
/// handle KDE's PascalCase keys and colon-containing section names.
pub(crate) fn create_kde_parser() -> configparser::ini::Ini {
    todo!()
}

/// Parse a KDE "R,G,B" color string into an Rgba (opaque).
///
/// Returns None for malformed values (never panics).
/// Exactly 3 comma-separated u8 components required.
pub(crate) fn parse_rgb(_value: &str) -> Option<Rgba> {
    todo!()
}

/// Resolve the path to the kdeglobals file.
///
/// Checks XDG_CONFIG_HOME (non-empty), then $HOME/.config/kdeglobals,
/// then /etc/xdg/kdeglobals as last resort.
pub(crate) fn kdeglobals_path() -> std::path::PathBuf {
    todo!()
}

/// Detect whether the active KDE theme is dark based on background luminance.
///
/// Uses BT.601 luminance coefficients on Colors:Window/BackgroundNormal.
/// Defaults to false (light) if the section/key is missing.
pub(crate) fn is_dark_theme(_ini: &configparser::ini::Ini) -> bool {
    todo!()
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
    fn kdeglobals_path_respects_xdg_config_home() {
        // SAFETY: test runs single-threaded (--test-threads=1)
        unsafe { std::env::set_var("XDG_CONFIG_HOME", "/tmp/test") };
        let path = kdeglobals_path();
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
        assert_eq!(path, std::path::PathBuf::from("/tmp/test/kdeglobals"));
    }

    #[test]
    fn kdeglobals_path_falls_back_to_home_config() {
        // SAFETY: test runs single-threaded (--test-threads=1)
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
        let path = kdeglobals_path();
        let home = std::env::var("HOME").expect("HOME must be set for this test");
        assert_eq!(
            path,
            std::path::PathBuf::from(home).join(".config").join("kdeglobals")
        );
    }

    // === is_dark_theme tests ===

    #[test]
    fn is_dark_theme_detects_breeze_dark() {
        let mut ini = create_kde_parser();
        ini.read(
            "[Colors:Window]\nBackgroundNormal=20,22,24\n".to_string(),
        )
        .unwrap();
        assert!(is_dark_theme(&ini));
    }

    #[test]
    fn is_dark_theme_detects_breeze_light() {
        let mut ini = create_kde_parser();
        ini.read(
            "[Colors:Window]\nBackgroundNormal=239,240,241\n".to_string(),
        )
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
        ini.read(
            "[Colors:View]\nBackgroundNormal=255,255,255\n".to_string(),
        )
        .unwrap();
        // Case-sensitive: PascalCase key should be retrievable as-is
        assert!(ini.get("Colors:View", "BackgroundNormal").is_some());
    }

    #[test]
    fn create_kde_parser_preserves_section_colons() {
        let mut ini = create_kde_parser();
        ini.read(
            "[Colors:Window]\nForegroundNormal=0,0,0\n".to_string(),
        )
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
}
