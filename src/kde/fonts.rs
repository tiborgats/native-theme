// Qt font string parsing -> ThemeFonts

use crate::ThemeFonts;

/// Parse a Qt `QFont::toString()` string into family and point size.
///
/// Handles both Qt4 (10 fields) and Qt5/6 (16+ fields) formats.
/// Only extracts family (field 0) and point size (field 1).
/// Returns None if fewer than 2 fields, empty family, or invalid/non-positive size.
fn parse_qt_font(_font_str: &str) -> Option<(String, f32)> {
    todo!()
}

/// Parse font settings from KDE's [General] section.
///
/// Reads the `font` key for the primary UI font and `fixed` key for
/// the monospace font. Returns a `ThemeFonts` with all fields `None`
/// if the keys are missing or unparseable.
pub(crate) fn parse_fonts(_ini: &configparser::ini::Ini) -> ThemeFonts {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // === parse_qt_font tests ===

    #[test]
    fn test_qt5_font_16_fields() {
        let result = parse_qt_font("Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1");
        assert_eq!(result, Some(("Noto Sans".to_string(), 10.0)));
    }

    #[test]
    fn test_qt4_font_10_fields() {
        let result = parse_qt_font("Noto Sans,10,-1,5,50,0,0,0,0,0");
        assert_eq!(result, Some(("Noto Sans".to_string(), 10.0)));
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(parse_qt_font(""), None);
    }

    #[test]
    fn test_single_field() {
        assert_eq!(parse_qt_font("Noto Sans"), None);
    }

    #[test]
    fn test_empty_family() {
        assert_eq!(parse_qt_font(",10,-1,5"), None);
    }

    #[test]
    fn test_negative_size() {
        assert_eq!(
            parse_qt_font("Noto Sans,-1,-1,5,400,0,0,0,0,0"),
            None
        );
    }

    #[test]
    fn test_zero_size() {
        assert_eq!(
            parse_qt_font("Noto Sans,0,-1,5,400,0,0,0,0,0"),
            None
        );
    }

    // === parse_fonts tests ===

    #[test]
    fn test_parse_fonts_with_both_keys() {
        let mut ini = super::super::create_kde_parser();
        ini.read(
            "[General]\nfont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\nfixed=Hack,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n"
                .to_string(),
        )
        .unwrap();

        let fonts = parse_fonts(&ini);
        assert_eq!(fonts.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fonts.size, Some(10.0));
        assert_eq!(fonts.mono_family.as_deref(), Some("Hack"));
        assert_eq!(fonts.mono_size, Some(10.0));
    }

    #[test]
    fn test_parse_fonts_missing_section() {
        let ini = super::super::create_kde_parser();
        let fonts = parse_fonts(&ini);
        assert_eq!(fonts.family, None);
        assert_eq!(fonts.size, None);
        assert_eq!(fonts.mono_family, None);
        assert_eq!(fonts.mono_size, None);
    }

    #[test]
    fn test_parse_fonts_missing_keys() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\nColorScheme=BreezeDark\n".to_string())
            .unwrap();
        let fonts = parse_fonts(&ini);
        assert_eq!(fonts.family, None);
        assert_eq!(fonts.size, None);
        assert_eq!(fonts.mono_family, None);
        assert_eq!(fonts.mono_size, None);
    }
}
