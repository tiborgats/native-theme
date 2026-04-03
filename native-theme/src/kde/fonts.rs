// Qt font string parsing with weight extraction -> populate per-widget fonts on ThemeVariant

/// Convert a Qt5 weight (0-100 scale) to CSS weight (100-900).
///
/// Mapping from research (KDE Qt5 integer weights to standard CSS ranges):
/// 0-12->100, 13-24->200, 25-37->300, 38-56->400, 57-62->500,
/// 63-74->600, 75-81->700, 82-87->800, 88+->900.
pub(crate) fn qt5_to_css_weight(qt5: i32) -> u16 {
    match qt5 {
        0..=12 => 100,
        13..=24 => 200,
        25..=37 => 300,
        38..=56 => 400,
        57..=62 => 500,
        63..=74 => 600,
        75..=81 => 700,
        82..=87 => 800,
        _ => 900, // 88+
    }
}

/// Parse a Qt `QFont::toString()` string into a FontSpec with weight.
///
/// Handles both Qt5 (<16 fields, field[4] is 0-100 scale) and
/// Qt6 (>=16 fields, field[4] is CSS 100-900 scale) formats.
///
/// Extracts: family (field 0), point size (field 1), weight (field 4).
/// Returns None if fewer than 5 fields, empty family, or invalid/non-positive size.
pub(crate) fn parse_qt_font_with_weight(font_str: &str) -> Option<crate::FontSpec> {
    let fields: Vec<&str> = font_str.split(',').collect();
    if fields.len() < 5 {
        return None;
    }
    let family = fields[0].trim().to_string();
    if family.is_empty() {
        return None;
    }
    let size = fields[1].trim().parse::<f32>().ok()?;
    if size <= 0.0 {
        return None;
    }
    let raw_weight = fields[4].trim().parse::<i32>().ok()?;

    // Reject corrupted font entries with negative weight
    if raw_weight < 0 {
        return None;
    }

    // Qt6 format has >= 16 fields and uses CSS weight scale (100-900) directly.
    // Qt5 format has < 16 fields and uses a 0-100 scale.
    let css_weight = if fields.len() >= 16 {
        raw_weight as u16 // Qt6: already CSS scale
    } else {
        qt5_to_css_weight(raw_weight)
    };

    Some(crate::FontSpec {
        family: Some(family),
        size: Some(size),
        weight: Some(css_weight),
    })
}

/// Populate per-widget font fields on a ThemeVariant from KDE INI.
///
/// Reads font keys from [General] and [WM] sections:
/// - defaults.font from [General] font
/// - defaults.mono_font from [General] fixed
/// - menu.font from [General] menuFont (KDE-03)
/// - toolbar.font from [General] toolBarFont (KDE-03)
/// - window.title_bar_font from [WM] activeFont (KDE-01)
///
/// Missing keys result in None fields (no hardcoded fallbacks).
pub(crate) fn populate_fonts(ini: &configparser::ini::Ini, variant: &mut crate::ThemeVariant) {
    if let Some(font_str) = ini.get("General", "font")
        && let Some(spec) = parse_qt_font_with_weight(&font_str)
    {
        variant.defaults.font = spec;
    }

    if let Some(fixed_str) = ini.get("General", "fixed")
        && let Some(spec) = parse_qt_font_with_weight(&fixed_str)
    {
        variant.defaults.mono_font = spec;
    }

    // KDE-03: Per-widget fonts
    if let Some(menu_str) = ini.get("General", "menuFont")
        && let Some(spec) = parse_qt_font_with_weight(&menu_str)
    {
        variant.menu.font = Some(spec);
    }

    if let Some(toolbar_str) = ini.get("General", "toolBarFont")
        && let Some(spec) = parse_qt_font_with_weight(&toolbar_str)
    {
        variant.toolbar.font = Some(spec);
    }

    // KDE-01: Title bar font from WM section
    if let Some(active_font_str) = ini.get("WM", "activeFont")
        && let Some(spec) = parse_qt_font_with_weight(&active_font_str)
    {
        variant.window.title_bar_font = Some(spec);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ThemeVariant;

    // === qt5_to_css_weight boundary tests ===

    #[test]
    fn qt5_weight_0_maps_to_100() {
        assert_eq!(qt5_to_css_weight(0), 100);
    }

    #[test]
    fn qt5_weight_12_maps_to_100() {
        assert_eq!(qt5_to_css_weight(12), 100);
    }

    #[test]
    fn qt5_weight_50_maps_to_400() {
        assert_eq!(qt5_to_css_weight(50), 400);
    }

    #[test]
    fn qt5_weight_75_maps_to_700_bold() {
        assert_eq!(qt5_to_css_weight(75), 700);
    }

    #[test]
    fn qt5_weight_88_maps_to_900() {
        assert_eq!(qt5_to_css_weight(88), 900);
    }

    #[test]
    fn qt5_weight_100_maps_to_900() {
        assert_eq!(qt5_to_css_weight(100), 900);
    }

    #[test]
    fn qt5_weight_25_maps_to_300() {
        assert_eq!(qt5_to_css_weight(25), 300);
    }

    #[test]
    fn qt5_weight_63_maps_to_600() {
        assert_eq!(qt5_to_css_weight(63), 600);
    }

    // === parse_qt_font_with_weight tests ===

    #[test]
    fn parse_qt5_format_extracts_weight_50_to_400() {
        // Qt5 format: 10 fields, field[4]=50 -> CSS 400
        let result = parse_qt_font_with_weight("Noto Sans,10,-1,5,50,0,0,0,0,0");
        let spec = result.unwrap();
        assert_eq!(spec.family.as_deref(), Some("Noto Sans"));
        assert_eq!(spec.size, Some(10.0));
        assert_eq!(spec.weight, Some(400));
    }

    #[test]
    fn parse_qt6_format_weight_stays_400() {
        // Qt6 format: 16 fields, field[4]=400 stays 400
        let result = parse_qt_font_with_weight("Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1");
        let spec = result.unwrap();
        assert_eq!(spec.weight, Some(400));
    }

    #[test]
    fn parse_qt5_weight_75_converts_to_700() {
        let result = parse_qt_font_with_weight("Noto Sans,10,-1,5,75,0,0,0,0,0");
        let spec = result.unwrap();
        assert_eq!(spec.weight, Some(700));
    }

    #[test]
    fn parse_qt6_weight_700_stays_700() {
        let result = parse_qt_font_with_weight("Noto Sans,10,-1,5,700,0,0,0,0,0,0,0,0,0,0,1");
        let spec = result.unwrap();
        assert_eq!(spec.weight, Some(700));
    }

    #[test]
    fn parse_empty_string_returns_none() {
        assert!(parse_qt_font_with_weight("").is_none());
    }

    #[test]
    fn parse_too_few_fields_returns_none() {
        assert!(parse_qt_font_with_weight("Noto Sans,10,-1,5").is_none());
    }

    #[test]
    fn parse_empty_family_returns_none() {
        assert!(parse_qt_font_with_weight(",10,-1,5,400").is_none());
    }

    #[test]
    fn parse_negative_size_returns_none() {
        assert!(parse_qt_font_with_weight("Noto Sans,-1,-1,5,400,0,0,0,0,0").is_none());
    }

    #[test]
    fn parse_zero_size_returns_none() {
        assert!(parse_qt_font_with_weight("Noto Sans,0,-1,5,400,0,0,0,0,0").is_none());
    }

    #[test]
    fn parse_negative_weight_returns_none() {
        assert!(parse_qt_font_with_weight("Noto Sans,10,-1,5,-1,0,0,0,0,0").is_none());
    }

    // === populate_fonts tests ===

    #[test]
    fn populate_fonts_sets_defaults_font_from_general() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\nfont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeVariant::default();
        populate_fonts(&ini, &mut variant);
        assert_eq!(variant.defaults.font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(variant.defaults.font.size, Some(10.0));
        assert_eq!(variant.defaults.font.weight, Some(400));
    }

    #[test]
    fn populate_fonts_sets_mono_font_from_fixed() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\nfixed=Hack,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeVariant::default();
        populate_fonts(&ini, &mut variant);
        assert_eq!(variant.defaults.mono_font.family.as_deref(), Some("Hack"));
        assert_eq!(variant.defaults.mono_font.size, Some(10.0));
    }

    #[test]
    fn populate_fonts_sets_menu_font_from_menufont_key() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\nmenuFont=Noto Sans,9,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeVariant::default();
        populate_fonts(&ini, &mut variant);
        let menu_font = variant.menu.font.unwrap();
        assert_eq!(menu_font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(menu_font.size, Some(9.0));
    }

    #[test]
    fn populate_fonts_sets_toolbar_font_from_toolbarfont_key() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\ntoolBarFont=Noto Sans,9,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeVariant::default();
        populate_fonts(&ini, &mut variant);
        let toolbar_font = variant.toolbar.font.unwrap();
        assert_eq!(toolbar_font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(toolbar_font.size, Some(9.0));
    }

    #[test]
    fn populate_fonts_sets_title_bar_font_from_wm_activefont() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[WM]\nactiveFont=Noto Sans,10,-1,5,75,0,0,0,0,0\n".to_string())
            .unwrap();
        let mut variant = ThemeVariant::default();
        populate_fonts(&ini, &mut variant);
        let tbf = variant.window.title_bar_font.unwrap();
        assert_eq!(tbf.family.as_deref(), Some("Noto Sans"));
        assert_eq!(tbf.size, Some(10.0));
        assert_eq!(tbf.weight, Some(700)); // Qt5 75 -> CSS 700
    }

    #[test]
    fn populate_fonts_missing_section_leaves_none() {
        let ini = super::super::create_kde_parser();
        let mut variant = ThemeVariant::default();
        populate_fonts(&ini, &mut variant);
        assert!(variant.defaults.font.family.is_none());
        assert!(variant.defaults.font.size.is_none());
        assert!(variant.defaults.mono_font.family.is_none());
        assert!(variant.menu.font.is_none());
        assert!(variant.toolbar.font.is_none());
        assert!(variant.window.title_bar_font.is_none());
    }

    #[test]
    fn populate_fonts_all_keys_present() {
        let mut ini = super::super::create_kde_parser();
        ini.read(
            "[General]\n\
             font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n\
             fixed=Hack,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n\
             menuFont=Noto Sans,9,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n\
             toolBarFont=Noto Sans,8,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n\
             [WM]\n\
             activeFont=Noto Sans,10,-1,5,700,0,0,0,0,0,0,0,0,0,0,1\n"
                .to_string(),
        )
        .unwrap();
        let mut variant = ThemeVariant::default();
        populate_fonts(&ini, &mut variant);

        assert_eq!(variant.defaults.font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(variant.defaults.font.size, Some(10.0));
        assert_eq!(variant.defaults.mono_font.family.as_deref(), Some("Hack"));
        assert_eq!(
            variant.menu.font.as_ref().unwrap().family.as_deref(),
            Some("Noto Sans")
        );
        assert_eq!(variant.menu.font.as_ref().unwrap().size, Some(9.0));
        assert_eq!(variant.toolbar.font.as_ref().unwrap().size, Some(8.0));
        assert_eq!(
            variant.window.title_bar_font.as_ref().unwrap().weight,
            Some(700)
        );
    }
}
