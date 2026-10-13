// Qt font string parsing with weight extraction -> populate per-widget fonts on ThemeMode

/// Qt5's `QFont::Weight` constants and the OpenType (CSS) weight Qt converts
/// each to (`legacyToOpenTypeMap`, qtbase `src/gui/text/qfont.cpp:150-154` at
/// v6.11.2, ef55f427).
const QT5_WEIGHTS: [(i32, u16); 9] = [
    (0, 100),
    (12, 200),
    (25, 300),
    (50, 400),
    (57, 500),
    (63, 600),
    (75, 700),
    (81, 800),
    (87, 900),
];

/// The Qt6 weight range: `QFont::setWeight` bounds any weight to it, with a
/// warning (`QFONT_WEIGHT_MIN`, `QFONT_WEIGHT_MAX`, qtbase
/// `src/gui/text/qfont_p.h:35-36`; `qfont.cpp:1218-1224` at v6.11.2).
const QT_WEIGHT_MIN: i32 = 1;

/// The top of the Qt6 weight range ([`QT_WEIGHT_MIN`]).
const QT_WEIGHT_MAX: i32 = 1000;

/// Convert a Qt5 weight (0-99 scale) to CSS weight (100-900) as Qt's
/// `qt_legacyToOpenTypeWeight` does (qtbase `src/gui/text/qfont.cpp:147-174`,
/// `:196-200` at v6.11.2): the nearest of [`QT5_WEIGHTS`], the lower one on a
/// tie.
pub(crate) fn qt5_to_css_weight(qt5: i32) -> u16 {
    let mut nearest = (u32::MAX, 100);
    for (legacy, css) in QT5_WEIGHTS {
        let distance = legacy.abs_diff(qt5);
        if distance < nearest.0 {
            nearest = (distance, css);
        } else {
            break;
        }
    }
    nearest.1
}

/// Parse a Qt `QFont::toString()` string into a FontSpec with weight.
///
/// Qt writes family (field 0), point size (1), pixel size (2), style hint
/// (3), weight (4) and style (5) first (qtbase `src/gui/text/qfont.cpp:2155-2163`
/// at v6.11.2, ef55f427). A Qt6 string has >= 16 fields and its weight on
/// the OpenType scale, a Qt5 one fewer and a 0-100 weight (`:2259-2262`).
///
/// - size: the point size when positive, replaced by the pixel size when
///   that is positive, as `QFont::fromString` sets them (`:2252-2257`):
///   `FontSize::Pt` or `FontSize::Px`; a pixel size is at most a `u16`
///   (`setPixelSize`, `:1052-1054`)
/// - weight: Qt6's bounded to 1..=1000 as `QFont::setWeight` bounds any
///   weight ([`QT_WEIGHT_MIN`], [`QT_WEIGHT_MAX`]), then clamped to the CSS
///   100..=900 that `check_font` accepts (`resolve/validate_helpers.rs:492`);
///   Qt5's through [`qt5_to_css_weight`]; a field that is not an integer
///   leaves it `None`
/// - style: `QFont::Style`'s `StyleNormal`, `StyleItalic`, `StyleOblique`
///   (`src/gui/text/qfont.h:77-81`) as [`FontStyle`](crate::model::font::FontStyle);
///   another value leaves it `None`
///
/// Returns None if fewer than 5 fields, an empty family, no positive size,
/// or a negative Qt5 weight.
pub(crate) fn parse_qt_font_with_weight(font_str: &str) -> Option<crate::FontSpec> {
    use crate::model::font::{FontSize, FontStyle};
    let fields: Vec<&str> = font_str.split(',').collect();
    let family_str = fields.first()?.trim();
    if family_str.is_empty() {
        return None;
    }
    let point_size = fields
        .get(1)?
        .trim()
        .parse::<f32>()
        .ok()
        .filter(|size| size.is_finite() && *size > 0.0)
        .map(FontSize::Pt);
    let pixel_size = fields
        .get(2)
        .and_then(|field| field.trim().parse::<u16>().ok())
        .filter(|size| *size > 0)
        .map(|size| FontSize::Px(f32::from(size)));
    let size = pixel_size.or(point_size)?;
    let raw_weight = fields.get(4)?.trim().parse::<i32>().ok();

    // Qt6 format has >= 16 fields and uses the OpenType scale (1-1000).
    // Qt5 format has < 16 fields and uses a 0-99 scale.
    let css_weight = match raw_weight {
        None => None,
        Some(raw) if fields.len() >= 16 => {
            u16::try_from(raw.clamp(QT_WEIGHT_MIN, QT_WEIGHT_MAX).clamp(100, 900)).ok()
        }
        // Reject corrupted font entries with negative weight
        Some(raw) if raw < 0 => return None,
        Some(raw) => Some(qt5_to_css_weight(raw)),
    };

    let style = match fields.get(5).map(|field| field.trim()) {
        Some("0") => Some(FontStyle::Normal),
        Some("1") => Some(FontStyle::Italic),
        Some("2") => Some(FontStyle::Oblique),
        _ => None,
    };

    Some(crate::FontSpec {
        family: Some(family_str.into()),
        size: Some(size),
        weight: css_weight,
        style,
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
        // Kirigami scales the body font's pointSize (kirigami `8319acc`,
        // `src/controls/Heading.qml:32`), which Qt Quick's font value type
        // gives a pixel-sized font as pixelSize * 72 / qt_defaultDpi()
        // (qtdeclarative `src/quick/util/qquickvaluetypes.cpp:1165-1171`,
        // `qquickvaluetypes_p.h:441-442` at v6.11.2, 4e3399c2). A pixel body's heading
        // is therefore its pixel size times the factor, which equals
        // Kirigami's when its DPI and the point-to-pixel conversion agree.
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
    // Merge into existing FontSpec to preserve .color set by populate_colors.
    if let Some(menu_str) = ini.get("General", "menuFont")
        && let Some(spec) = parse_qt_font_with_weight(&menu_str)
    {
        let font = variant.menu.font.get_or_insert_default();
        font.family = spec.family;
        font.size = spec.size;
        font.weight = spec.weight;
        font.style = spec.style;
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
    fn qt5_weight_12_maps_to_200() {
        assert_eq!(qt5_to_css_weight(12), 200);
    }

    /// Each Qt5 `QFont::Weight` constant maps to its Qt6 one, as
    /// `qt_legacyToOpenTypeWeight` maps it.
    #[test]
    fn qt5_weight_constants_map_to_qt6_constants() {
        for (qt5, qt6) in [
            (0, 100),
            (12, 200),
            (25, 300),
            (50, 400),
            (57, 500),
            (63, 600),
            (75, 700),
            (81, 800),
            (87, 900),
        ] {
            assert_eq!(qt5_to_css_weight(qt5), qt6, "Qt5 weight {qt5}");
        }
    }

    /// Between two constants the nearer one wins, the lower one on a tie.
    #[test]
    fn qt5_weight_between_constants_takes_the_nearest() {
        for (qt5, qt6) in [
            (6, 100),
            (7, 200),
            (18, 200),
            (19, 300),
            (37, 300),
            (38, 400),
            (53, 400),
            (54, 500),
            (60, 500),
            (61, 600),
            (69, 600),
            (70, 700),
            (78, 700),
            (79, 800),
            (84, 800),
            (85, 900),
            (99, 900),
        ] {
            assert_eq!(qt5_to_css_weight(qt5), qt6, "Qt5 weight {qt5}");
        }
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
    fn parse_non_finite_size_returns_none() {
        for size in ["NaN", "nan", "inf", "-inf", "infinity"] {
            let font = format!("Noto Sans,{size},-1,5,400,0,0,0,0,0,0,0,0,0,0,1");
            assert!(parse_qt_font_with_weight(&font).is_none(), "{size}");
        }
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

    // === Qt6 weight range, pixel size, style ===

    #[test]
    fn parse_qt6_weight_outside_css_range_is_clamped() {
        for (weight, css) in [
            ("-1", 100),
            ("0", 100),
            ("1", 100),
            ("99", 100),
            ("950", 900),
            ("1000", 900),
            ("1001", 900),
            ("70000", 900),
        ] {
            let font = format!("Noto Sans,10,-1,5,{weight},0,0,0,0,0,0,0,0,0,0,1");
            let spec = parse_qt_font_with_weight(&font).unwrap();
            assert_eq!(spec.weight, Some(css), "weight {weight}");
        }
    }

    #[test]
    fn parse_non_integer_weight_keeps_the_font_without_weight() {
        for weight in ["bold", "400.5", "", "99999999999"] {
            for tail in ["0,0,0,0,0,0,0,0,0,0,1", "0,0,0,0,0"] {
                let font = format!("Noto Sans,10,-1,5,{weight},{tail}");
                let spec = parse_qt_font_with_weight(&font).unwrap();
                assert_eq!(spec.weight, None, "{font}");
                assert_eq!(spec.family.as_deref(), Some("Noto Sans"), "{font}");
                assert_eq!(spec.size, Some(FontSize::Pt(10.0)), "{font}");
            }
        }
    }

    #[test]
    fn parse_qt6_weight_range_ends_are_kept() {
        for weight in [100, 900] {
            let font = format!("Noto Sans,10,-1,5,{weight},0,0,0,0,0,0,0,0,0,0,1");
            assert_eq!(
                parse_qt_font_with_weight(&font).and_then(|spec| spec.weight),
                Some(weight)
            );
        }
    }

    #[test]
    fn parse_pixel_size_font_is_px() {
        let spec =
            parse_qt_font_with_weight("Noto Sans,-1,13,5,400,0,0,0,0,0,0,0,0,0,0,1").unwrap();
        assert_eq!(spec.size, Some(FontSize::Px(13.0)));
        assert_eq!(spec.weight, Some(400));
    }

    #[test]
    fn parse_pixel_size_above_u16_returns_none() {
        assert!(
            parse_qt_font_with_weight("Noto Sans,-1,65536,5,400,0,0,0,0,0,0,0,0,0,0,1").is_none()
        );
    }

    /// A pixel-sized body's headings are its pixel size times Kirigami's
    /// level factors.
    #[test]
    fn populate_fonts_derives_pixel_headings_from_a_pixel_body() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[General]\nfont=Noto Sans,-1,13,5,400,0,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        assert_eq!(variant.defaults.font.size, Some(FontSize::Px(13.0)));
        let ts = &variant.text_scale;
        for (entry, factor) in [
            (&ts.section_heading, HEADING_LEVEL_2_FACTOR),
            (&ts.dialog_title, HEADING_LEVEL_1_FACTOR),
        ] {
            assert_eq!(
                entry.as_ref().and_then(|e| e.size),
                Some(FontSize::Px(13.0 * factor))
            );
        }
    }

    #[test]
    fn parse_pixel_size_zero_or_negative_returns_none() {
        for pixels in ["0", "-1", "-13"] {
            let font = format!("Noto Sans,-1,{pixels},5,400,0,0,0,0,0,0,0,0,0,0,1");
            assert!(
                parse_qt_font_with_weight(&font).is_none(),
                "pixel size {pixels}"
            );
        }
    }

    #[test]
    fn parse_style_field_maps_to_font_style() {
        use crate::model::font::FontStyle;
        for (field, style) in [
            ("0", Some(FontStyle::Normal)),
            ("1", Some(FontStyle::Italic)),
            ("2", Some(FontStyle::Oblique)),
            ("3", None),
            ("x", None),
        ] {
            let font = format!("Noto Sans,10,-1,5,400,{field},0,0,0,0,0,0,0,0,0,1");
            let spec = parse_qt_font_with_weight(&font).unwrap();
            assert_eq!(spec.style, style, "style field {field}");
        }
    }

    #[test]
    fn populate_fonts_sets_title_bar_font_style() {
        let mut ini = super::super::create_kde_parser();
        ini.read("[WM]\nactiveFont=Noto Sans,10,-1,5,700,1,0,0,0,0,0,0,0,0,0,1\n".to_string())
            .unwrap();
        let mut variant = ThemeMode::default();
        populate_fonts(&ini, &mut variant);
        let tbf = variant.window.title_bar_font.unwrap();
        assert_eq!(tbf.style, Some(crate::model::font::FontStyle::Italic));
    }
}
