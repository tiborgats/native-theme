// Qt font string parsing with weight extraction -> populate per-widget fonts on ThemeMode

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
    let family_str = fields.first()?.trim();
    if family_str.is_empty() {
        return None;
    }
    let size = fields.get(1)?.trim().parse::<f32>().ok()?;
    if size <= 0.0 {
        return None;
    }
    let raw_weight = fields.get(4)?.trim().parse::<i32>().ok()?;

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
        family: Some(family_str.into()),
        size: Some(crate::model::font::FontSize::Pt(size)),
        weight: Some(css_weight),
        ..Default::default()
    })
}

/// Kirigami `Heading` level 1 (the default level): the body font's point size
/// times this (kirigami `8319acc`, `src/controls/Heading.qml:17-18,32`).
const HEADING_LEVEL_1_FACTOR: f32 = 1.35;

/// Kirigami `Heading` level 2: the body font's point size times this
/// (kirigami `8319acc`, `src/controls/Heading.qml:20-21,32`).
const HEADING_LEVEL_2_FACTOR: f32 = 1.20;

/// Kirigami `Heading`'s weight for its default `type: Heading.Type.Normal`
/// (kirigami `8319acc`, `src/templates/Heading.qml:89`): `Font.Normal`, CSS
/// 400 (`src/controls/Heading.qml:35`).
const HEADING_WEIGHT: u16 = 400;

/// A Kirigami heading of the body font `body` at a level's factor.
fn heading(body: crate::model::font::FontSize, factor: f32) -> crate::TextScaleEntry {
    use crate::model::font::FontSize;
    let size = match body {
        FontSize::Pt(v) => FontSize::Pt(v * factor),
        FontSize::Px(v) => FontSize::Px(v * factor),
    };
    crate::TextScaleEntry {
        size: Some(size),
        weight: Some(HEADING_WEIGHT),
        line_height: None,
    }
}

/// Populate per-widget font fields on a ThemeMode from KDE INI.
///
/// Reads font keys from [General] and [WM] sections:
/// - defaults.font from [General] font, and from its size the Kirigami
///   heading sizes of text_scale.section_heading (level 2) and
///   text_scale.dialog_title (level 1), `docs/platform-facts.md:1436-1437`
/// - defaults.mono_font from [General] fixed
/// - text_scale.caption from [General] smallestReadableFont
///   (`docs/platform-facts.md:1435`)
/// - menu.font from [General] menuFont (KDE-03)
/// - toolbar.font from [General] toolBarFont (KDE-03)
/// - window.title_bar_font from [WM] activeFont (KDE-01)
///
/// Missing keys result in None fields (no hardcoded fallbacks).
pub(crate) fn populate_fonts(ini: &configparser::ini::Ini, variant: &mut crate::ThemeMode) {
    if let Some(font_str) = ini.get("General", "font")
        && let Some(spec) = parse_qt_font_with_weight(&font_str)
    {
        if let Some(body) = spec.size {
            variant.text_scale.section_heading = Some(heading(body, HEADING_LEVEL_2_FACTOR));
            variant.text_scale.dialog_title = Some(heading(body, HEADING_LEVEL_1_FACTOR));
        }
        variant.defaults.font = spec;
    }

    if let Some(smallest_str) = ini.get("General", "smallestReadableFont")
        && let Some(spec) = parse_qt_font_with_weight(&smallest_str)
    {
        variant.text_scale.caption = Some(crate::TextScaleEntry {
            size: spec.size,
            weight: spec.weight,
            line_height: None,
        });
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
    // Merge into existing FontSpec to preserve .color set by populate_colors.
    if let Some(active_font_str) = ini.get("WM", "activeFont")
        && let Some(spec) = parse_qt_font_with_weight(&active_font_str)
    {
        let font = variant.window.title_bar_font.get_or_insert_default();
        font.family = spec.family;
        font.size = spec.size;
        font.weight = spec.weight;
        font.style = spec.style;
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ThemeMode;
    use crate::model::font::FontSize;

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
        assert_eq!(spec.size, Some(FontSize::Pt(10.0)));
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
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        assert_eq!(variant.defaults.font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(variant.defaults.font.size, Some(FontSize::Pt(10.0)));
        assert_eq!(variant.defaults.font.weight, Some(400));
    }

    #[test]
    fn populate_fonts_sets_mono_font_from_fixed() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\nfixed=Hack,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        assert_eq!(variant.defaults.mono_font.family.as_deref(), Some("Hack"));
        assert_eq!(variant.defaults.mono_font.size, Some(FontSize::Pt(10.0)));
    }

    #[test]
    fn populate_fonts_sets_menu_font_from_menufont_key() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\nmenuFont=Noto Sans,9,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        let menu_font = variant.menu.font.unwrap();
        assert_eq!(menu_font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(menu_font.size, Some(FontSize::Pt(9.0)));
    }

    #[test]
    fn populate_fonts_sets_toolbar_font_from_toolbarfont_key() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\ntoolBarFont=Noto Sans,9,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        let toolbar_font = variant.toolbar.font.unwrap();
        assert_eq!(toolbar_font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(toolbar_font.size, Some(FontSize::Pt(9.0)));
    }

    #[test]
    fn populate_fonts_sets_title_bar_font_from_wm_activefont() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[WM]\nactiveFont=Noto Sans,10,-1,5,75,0,0,0,0,0\n".to_string())
            .unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        let tbf = variant.window.title_bar_font.unwrap();
        assert_eq!(tbf.family.as_deref(), Some("Noto Sans"));
        assert_eq!(tbf.size, Some(FontSize::Pt(10.0)));
        assert_eq!(tbf.weight, Some(700)); // Qt5 75 -> CSS 700
    }

    /// A non-default body (11pt) and smallest readable font (9pt, Qt6 weight 300).
    const NON_DEFAULT_FONTS: &str = "[General]\n\
         font=Noto Sans,11,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n\
         smallestReadableFont=Noto Sans,9,-1,5,300,0,0,0,0,0,0,0,0,0,0,1\n";

    fn size_pt(entry: Option<&crate::TextScaleEntry>) -> Option<f32> {
        match entry?.size? {
            FontSize::Pt(v) => Some(v),
            FontSize::Px(_) => None,
        }
    }

    #[test]
    fn populate_fonts_sets_caption_from_smallest_readable_font() {
        let mut ini = super::super::create_kde_parser();
        ini.read(NON_DEFAULT_FONTS.to_string()).unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        let caption = variant.text_scale.caption.as_ref();
        assert_eq!(size_pt(caption), Some(9.0));
        assert_eq!(caption.and_then(|c| c.weight), Some(300));
        assert_eq!(caption.and_then(|c| c.line_height), None);
    }

    /// Kirigami's heading levels of an 11pt body: level 2 is 13.2pt, level 1
    /// 14.85pt, both `Font.Normal`.
    #[test]
    fn populate_fonts_derives_headings_from_body_font() {
        let mut ini = super::super::create_kde_parser();
        ini.read(NON_DEFAULT_FONTS.to_string()).unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        let ts = &variant.text_scale;
        let section = size_pt(ts.section_heading.as_ref()).unwrap();
        let title = size_pt(ts.dialog_title.as_ref()).unwrap();
        assert!((section - 13.2).abs() < 1e-4, "section_heading {section}");
        assert!((title - 14.85).abs() < 1e-4, "dialog_title {title}");
        for entry in [&ts.section_heading, &ts.dialog_title] {
            let entry = entry.as_ref().unwrap();
            assert_eq!(entry.weight, Some(400));
            assert_eq!(entry.line_height, None);
        }
        assert!(ts.display.is_none());
    }

    #[test]
    fn populate_fonts_without_fonts_states_no_text_scale() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\nColorScheme=BreezeLight\n".to_string())
            .unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        assert_eq!(variant.text_scale, crate::TextScale::default());
    }

    #[test]
    fn populate_fonts_missing_section_leaves_none() {
        let ini = super::super::create_kde_parser();
        let mut variant = ThemeMode::default();
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
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);

        assert_eq!(variant.defaults.font.family.as_deref(), Some("Noto Sans"));
        assert_eq!(variant.defaults.font.size, Some(FontSize::Pt(10.0)));
        assert_eq!(variant.defaults.mono_font.family.as_deref(), Some("Hack"));
        assert_eq!(
            variant.menu.font.as_ref().unwrap().family.as_deref(),
            Some("Noto Sans")
        );
        assert_eq!(
            variant.menu.font.as_ref().unwrap().size,
            Some(FontSize::Pt(9.0))
        );
        assert_eq!(
            variant.toolbar.font.as_ref().unwrap().size,
            Some(FontSize::Pt(8.0))
        );
        assert_eq!(
            variant.window.title_bar_font.as_ref().unwrap().weight,
            Some(700)
        );
    }
}
