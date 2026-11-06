//! GNOME's Pango font-description parser, unlinked (the 0.8.0 plan's D10,
//! decision F7): the clean room redoes it at M1b. Kept as it was at the
//! switch's parent commit, outside the module tree, so it is not compiled.

use crate::model::FontSpec;
use crate::model::font::{FontSize, FontStyle};

/// Pango's weight words and their `PangoWeight` (`weight_map`, pango 1.58.2
/// `pango/fonts.c:1311-1333`; the values `pango/pango-font.h:131-142`).
const PANGO_WEIGHTS: &[(&str, i32)] = &[
    ("Thin", 100),
    ("Ultra-Light", 200),
    ("Extra-Light", 200),
    ("Light", 300),
    ("Semi-Light", 350),
    ("Demi-Light", 350),
    ("Book", 380),
    ("Regular", 400),
    ("Medium", 500),
    ("Semi-Bold", 600),
    ("Demi-Bold", 600),
    ("Bold", 700),
    ("Ultra-Bold", 800),
    ("Extra-Bold", 800),
    ("Heavy", 900),
    ("Black", 900),
    ("Ultra-Heavy", 1000),
    ("Extra-Heavy", 1000),
    ("Ultra-Black", 1000),
    ("Extra-Black", 1000),
];

/// `PANGO_WEIGHT_NORMAL`, the weight of a description without a weight word
/// (`pango/pango-font.h:136`; `pfd_defaults`, `pango/fonts.c:63-82`).
const PANGO_WEIGHT_NORMAL: i32 = 400;

/// Pango's style words (`style_map`, `pango/fonts.c:1294-1299`).
const PANGO_STYLES: &[(&str, FontStyle)] = &[
    ("Roman", FontStyle::Normal),
    ("Oblique", FontStyle::Oblique),
    ("Italic", FontStyle::Italic),
];

/// Pango's stretch, variant, gravity and colour words (`width_map`,
/// `variant_map`, `gravity_map`, `color_map`, `pango/fonts.c:1301-1309`,
/// `:1335-1361`), each with the name of its `{field}=` prefix. The model's
/// font has no field for any of them, so they are read only to know where the
/// family ends, and dropped.
const PANGO_DROPPED_FIELDS: &[(&str, &[(&str, ())])] = &[
    (
        "width",
        &[
            ("Ultra-Condensed", ()),
            ("Extra-Condensed", ()),
            ("Condensed", ()),
            ("Semi-Condensed", ()),
            ("Semi-Expanded", ()),
            ("Expanded", ()),
            ("Extra-Expanded", ()),
            ("Ultra-Expanded", ()),
        ],
    ),
    (
        "variant",
        &[
            ("Small-Caps", ()),
            ("All-Small-Caps", ()),
            ("Petite-Caps", ()),
            ("All-Petite-Caps", ()),
            ("Unicase", ()),
            ("Title-Caps", ()),
        ],
    ),
    (
        "gravity",
        &[
            ("Not-Rotated", ()),
            ("South", ()),
            ("Upside-Down", ()),
            ("North", ()),
            ("Rotated-Left", ()),
            ("East", ()),
            ("Rotated-Right", ()),
            ("West", ()),
        ],
    ),
    ("color", &[("With-Color", ()), ("Without-Color", ())]),
];

/// The largest size Pango reads (`parse_size`, `pango/fonts.c:1495-1519`).
const PANGO_MAX_SIZE: f64 = 1_000_000.0;


    // `apply_accent`, which a KDE read shares, where the info colour is
    // `[Colors:View] ForegroundActive`.
    variant.defaults.info_color = Some(*accent);
    let ring = accent::focus_ring(*accent, dark, high_contrast);
    variant.defaults.focus_ring_color = Some(ring);
    variant.input.focus_border_color = Some(ring);
}

/// Parse a GNOME font string (gsettings `font-name`, `monospace-font-name`,
/// `titlebar-font`) as Pango's `pango_font_description_from_string` does
/// (pango 1.58.2 `pango/fonts.c:1613-1729`): `[FAMILY-LIST] [STYLE-OPTIONS]
/// [SIZE] [VARIATIONS] [FEATURES]`, such as `'Cantarell Bold Italic 11'`,
/// `'Cantarell 15px'`, `'Cantarell 11 @wght=200'`.
///
/// 1. A trailing `#features` and `@variations` word, in either order, are
///    skipped: the model's font holds neither.
/// 2. The size ([`pango_size`]): points, or pixels with a `px` suffix.
/// 3. Style-option words, read right to left until one is not Pango's
///    ([`PangoStyleOptions::apply`]): weight and style as the model holds
///    them; stretch, variant, gravity and colour dropped.
/// 4. The rest is the family list. Pango keeps the whole comma-separated list
///    as the family (`"Cantarell,Sans"`) for fontconfig to try in turn; the
///    model's family is one name, so it is the list's first non-empty one.
///
/// Weight and style default to Pango's normal ones, as Pango sets both on
/// every description. Returns `None` for a string without a family or without
/// a positive, finite size: Pango reads `'Cantarell NaN'` as the family
/// `Cantarell NaN` with no size, which names no font, so that font keeps the
/// preset's.
pub(crate) fn parse_gnome_font_to_fontspec(s: &str) -> Option<FontSpec> {
    let mut rest = s.trim().trim_matches('\'');

    // The last word runs back to whitespace, the one before it to a comma too.
    let (before, word) = pango_last_word(rest, "");
    let tail = ['#', '@'].into_iter().find(|mark| word.starts_with(*mark));
    if tail.is_some() {
        rest = before;
    }
    let (before, word) = pango_last_word(rest, ",");
    if ['#', '@']
        .into_iter()
        .any(|mark| Some(mark) != tail && word.starts_with(mark))
    {
        rest = before;
    }

    let (before, word) = pango_last_word(rest, ",");
    let size = pango_size(word)?;
    rest = before;

    let mut options = PangoStyleOptions {
        weight: PANGO_WEIGHT_NORMAL,
        style: Some(FontStyle::Normal),
    };
    loop {
        let (before, word) = pango_last_word(rest, ",");
        if word.is_empty() || !options.apply(word) {
            break;
        }
        rest = before;
    }

    let family = rest
        .split(',')
        .map(|family| family.trim_matches(|c: char| c.is_ascii_whitespace()))
        .find(|family| !family.is_empty())?;

    Some(FontSpec {
        family: Some(family.into()),
        size: Some(size),
        // PangoWeight runs to 1000; `check_font` accepts CSS 100..=900
        // (`resolve/validate_helpers.rs:492`).
        weight: u16::try_from(options.weight.clamp(100, 900)).ok(),
        style: options.style,
        ..Default::default()
    })
}

/// Pango's `getword` (`pango/fonts.c:1478-1493`): after trailing whitespace,
/// the last word of `s`, running back to whitespace or to a character of
/// `stop`. Returns the text before the word, and the word. Whitespace is
/// `g_ascii_isspace`'s (glib 2.88.3 `glib/gstrfuncs.c:262-267`), the set of
/// [`char::is_ascii_whitespace`].
fn pango_last_word<'a>(s: &'a str, stop: &str) -> (&'a str, &'a str) {
    let s = s.trim_end_matches(|c: char| c.is_ascii_whitespace());
    let word = s
        .rsplit(|c: char| c.is_ascii_whitespace() || stop.contains(c))
        .next()
        .unwrap_or_default();
    (s.strip_suffix(word).unwrap_or_default(), word)
}

/// Pango's `parse_size` (`pango/fonts.c:1495-1519`): a number of points, or
/// of pixels when `px` follows it, up to [`PANGO_MAX_SIZE`]. A size of 0, which
/// Pango reads as no size, and a NaN or infinite one, which is not a size to
/// Pango, give `None`.
fn pango_size(word: &str) -> Option<FontSize> {
    let (number, pixels) = match word.strip_suffix("px") {
        Some(number) => (number, true),
        None => (word, false),
    };
    let size = number
        .parse::<f64>()
        .ok()
        .filter(|size| *size > 0.0 && *size <= PANGO_MAX_SIZE)? as f32;
    Some(if pixels {
        FontSize::Px(size)
    } else {
        FontSize::Pt(size)
    })
}

/// A Pango style-option word's value: a word of the field's table, or the
/// integer after a `{field}=` prefix.
enum PangoField<T> {
    Word(T),
    Int(i32),
}

/// Pango's `find_field` (`pango/fonts.c:1411-1447`): `word` as one of `map`'s
/// words, or, after a `{what}=` prefix, as one of them or a non-negative
/// integer (`parse_int`, `:1391-1409`).
fn pango_find_field<T: Copy>(what: &str, map: &[(&str, T)], word: &str) -> Option<PangoField<T>> {
    let (word, had_prefix) = match word.strip_prefix(what).and_then(|w| w.strip_prefix('=')) {
        Some(value) => (value, true),
        None => (word, false),
    };
    if let Some(&(_, value)) = map.iter().find(|(name, _)| pango_word_matches(name, word)) {
        return Some(PangoField::Word(value));
    }
    if had_prefix {
        return word
            .parse::<i32>()
            .ok()
            .filter(|n| *n >= 0)
            .map(PangoField::Int);
    }
    None
}

/// Pango's `field_matches` (`pango/fonts.c:1363-1389`): `word` is `name`
/// ignoring ASCII case, any of `name`'s hyphens optional (`SemiBold` is
/// `Semi-Bold`).
fn pango_word_matches(name: &str, word: &str) -> bool {
    let mut name = name.bytes();
    for w in word.bytes() {
        loop {
            match name.next() {
                Some(n) if n.eq_ignore_ascii_case(&w) => break,
                Some(b'-') => {}
                _ => return false,
            }
        }
    }
    name.next().is_none()
}

/// What Pango's style-option words set, as the model's font holds it.
struct PangoStyleOptions {
    /// A `PangoWeight`.
    weight: i32,
    /// `None` for a `style=` integer that is no `PangoStyle`.
    style: Option<FontStyle>,
}

impl PangoStyleOptions {
    /// Read one word as Pango's `find_field_any` does (`pango/fonts.c:1449-1476`):
    /// `Normal`, then the weight, style, stretch, variant, gravity and colour
    /// fields in turn. Returns `false` when the word is none of them.
    fn apply(&mut self, word: &str) -> bool {
        if pango_word_matches("Normal", word) {
            return true;
        }
        if let Some(weight) = pango_find_field("weight", PANGO_WEIGHTS, word) {
            self.weight = match weight {
                PangoField::Word(weight) | PangoField::Int(weight) => weight,
            };
            return true;
        }
        if let Some(style) = pango_find_field("style", PANGO_STYLES, word) {
            self.style = match style {
                PangoField::Word(style) => Some(style),
                // `PangoStyle`'s values (`pango/pango-font.h:74-78`).
                PangoField::Int(0) => Some(FontStyle::Normal),
                PangoField::Int(1) => Some(FontStyle::Oblique),
                PangoField::Int(2) => Some(FontStyle::Italic),
                PangoField::Int(_) => None,
            };
            return true;
        }
        PANGO_DROPPED_FIELDS
            .iter()
            .any(|(what, map)| pango_find_field(what, map, word).is_some())
    }
}


#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;


    // === parse_gnome_font_to_fontspec with weight extraction ===

    #[test]
    fn fontspec_bold_weight() {
        let fs = parse_gnome_font_to_fontspec("Cantarell Bold 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(700));
    }

    #[test]
    fn fontspec_light_weight() {
        let fs = parse_gnome_font_to_fontspec("Inter Light 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Inter"));
        assert_eq!(fs.size, Some(FontSize::Pt(10.0)));
        assert_eq!(fs.weight, Some(300));
    }

    #[test]
    fn fontspec_semi_bold_weight() {
        let fs = parse_gnome_font_to_fontspec("Noto Sans Semi-Bold 12").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(12.0)));
        assert_eq!(fs.weight, Some(600));
    }

    #[test]
    fn fontspec_no_modifier_defaults_to_regular() {
        let fs = parse_gnome_font_to_fontspec("Cantarell 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(400));
    }

    #[test]
    fn fontspec_medium_weight() {
        let fs = parse_gnome_font_to_fontspec("Inter Medium 10.5").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Inter"));
        assert_eq!(fs.size, Some(FontSize::Pt(10.5)));
        assert_eq!(fs.weight, Some(500));
    }

    #[test]
    fn fontspec_heavy_weight() {
        let fs = parse_gnome_font_to_fontspec("'Fira Sans Heavy 14'").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Fira Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(14.0)));
        assert_eq!(fs.weight, Some(900));
    }

    #[test]
    fn fontspec_thin_weight() {
        let fs = parse_gnome_font_to_fontspec("Roboto Thin 12").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Roboto"));
        assert_eq!(fs.size, Some(FontSize::Pt(12.0)));
        assert_eq!(fs.weight, Some(100));
    }

    #[test]
    fn fontspec_extra_bold_weight() {
        let fs = parse_gnome_font_to_fontspec("Source Sans Extra-Bold 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Source Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(800));
    }

    #[test]
    fn fontspec_ultra_light_weight() {
        let fs = parse_gnome_font_to_fontspec("Noto Sans Ultra-Light 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(10.0)));
        assert_eq!(fs.weight, Some(200));
    }

    #[test]
    fn fontspec_empty_returns_none() {
        assert!(parse_gnome_font_to_fontspec("").is_none());
    }

    #[test]
    fn fontspec_with_quotes() {
        let fs = parse_gnome_font_to_fontspec("'Cantarell Bold 11'").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.weight, Some(700));
    }

    #[test]
    fn fontspec_non_finite_size_is_dropped() {
        assert!(parse_gnome_font_to_fontspec("Cantarell NaN").is_none());
        assert!(parse_gnome_font_to_fontspec("Cantarell inf").is_none());
        assert!(parse_gnome_font_to_fontspec("Cantarell 0").is_none());
        assert!(parse_gnome_font_to_fontspec("Cantarell -11").is_none());
    }

    #[test]
    fn fontspec_style_word() {
        let fs = parse_gnome_font_to_fontspec("Cantarell Italic 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(400));
        assert_eq!(fs.style, Some(FontStyle::Italic));
    }

    #[test]
    fn fontspec_weight_and_style_words() {
        let fs = parse_gnome_font_to_fontspec("Noto Sans Bold Italic 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(700));
        assert_eq!(fs.style, Some(FontStyle::Italic));
    }

    #[test]
    fn fontspec_pixel_size() {
        let fs = parse_gnome_font_to_fontspec("Cantarell 15px").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Px(15.0)));
    }

    #[test]
    fn fontspec_variations_and_features_are_skipped() {
        let fs = parse_gnome_font_to_fontspec("Cantarell 11 @wght=200").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        let fs =
            parse_gnome_font_to_fontspec("Cantarell Italic Light 15 @wght=200 #tnum=1").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(15.0)));
        assert_eq!(fs.weight, Some(300));
        assert_eq!(fs.style, Some(FontStyle::Italic));
    }

    #[test]
    fn fontspec_first_family_of_a_list() {
        let fs = parse_gnome_font_to_fontspec("Cantarell, Sans 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        let fs = parse_gnome_font_to_fontspec("Cantarell,Bold 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.weight, Some(700));
    }

    /// Pango's words match ignoring ASCII case, and a hyphen of its table's
    /// word may be left out (`field_matches`, pango 1.58.2 `pango/fonts.c:1363-1389`).
    #[test]
    fn fontspec_words_ignore_case_and_hyphens() {
        let fs = parse_gnome_font_to_fontspec("Inter semibold OBLIQUE 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Inter"));
        assert_eq!(fs.weight, Some(600));
        assert_eq!(fs.style, Some(FontStyle::Oblique));
    }

    /// Pango's Semi-Light, Book and Ultra-Black (`pango/fonts.c:1311-1333`) are
    /// `PANGO_WEIGHT_SEMILIGHT` 350, `PANGO_WEIGHT_BOOK` 380 and
    /// `PANGO_WEIGHT_ULTRAHEAVY` 1000 (pango 1.58.2 `pango/pango-font.h:131-142`),
    /// the last clamped to the 900 that `check_font` accepts.
    #[test]
    fn fontspec_pango_only_weights() {
        let weight = |s| parse_gnome_font_to_fontspec(s).unwrap().weight;
        assert_eq!(weight("Inter Semi-Light 10"), Some(350));
        assert_eq!(weight("Inter Book 10"), Some(380));
        assert_eq!(weight("Inter Ultra-Black 10"), Some(900));
        assert_eq!(weight("Inter weight=650 10"), Some(650));
    }

    /// Stretch, variant and gravity words are Pango's, so they leave the family,
    /// and the model has no field for them.
    #[test]
    fn fontspec_stretch_variant_gravity_words_leave_the_family() {
        let fs =
            parse_gnome_font_to_fontspec("Noto Sans Condensed Small-Caps Rotated-Left 9").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(9.0)));
        assert_eq!(fs.weight, Some(400));
        assert_eq!(fs.style, Some(FontStyle::Normal));
    }

    /// Pango reads the words right to left and stops at the first that is not
    /// one of its own: the rest is the family, and of two weights the leftmost
    /// wins.
    #[test]
    fn fontspec_family_ends_at_the_first_unknown_word() {
        let fs = parse_gnome_font_to_fontspec("DejaVu Sans Mono Book 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("DejaVu Sans Mono"));
        assert_eq!(fs.weight, Some(380));
        let fs = parse_gnome_font_to_fontspec("Noto Sans Bold Mono 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans Bold Mono"));
        assert_eq!(fs.weight, Some(400));
        let fs = parse_gnome_font_to_fontspec("Inter Bold Light 10").unwrap();
        assert_eq!(fs.weight, Some(700));
    }
}
