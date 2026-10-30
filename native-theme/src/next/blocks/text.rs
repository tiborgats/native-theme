//! Text style and font (S2.8).
//!
//! Weight, width and small caps each have one route: [`Font::weight`],
//! [`Font::width`] and [`TextCase::SmallCaps`]; a connector drawing with a
//! variable font maps them onto its `wght` and `wdth` axes itself.

use std::fmt;

use native_theme_derive::ThemeParts;

use super::paint::{Paint, PaintPatch, ResolvedPaint};
use crate::next::selector::{PartRef, VariantRef, WidgetRef};
use crate::next::units::{FontSize, Fraction, Px};
use crate::next::value::Value;

/// A font.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Font {
    /// The family; its fallbacks are the text tokens' (S4.4).
    pub family: Value<String>,
    /// The size, in points or pixels; a point size becomes logical px by the
    /// font DPI, then the text scaling (S9.4).
    pub size: Value<FontSize>,
    /// The weight, the only route for it: `axes` may not hold `wght` (S12).
    pub weight: Value<u16>,
    /// The style.
    pub style: Value<FontStyle>,
    /// The width, in percent of normal as the OpenType `wdth` axis, and the
    /// only route for it: `axes` may not hold `wdth` (S12).
    #[parts(may_be_absent)]
    pub width: Value<f32>,
    /// Variation axes.
    pub axes: Vec<(AxisTag, AxisValue)>,
    /// OpenType features and their values; never `smcp`: small caps is
    /// [`TextCase::SmallCaps`] (S12).
    pub features: Vec<(FeatureTag, u32)>,
}

/// A font's style.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum FontStyle {
    /// Upright.
    Normal,
    /// Italic.
    Italic,
    /// Oblique.
    Oblique,
}

/// A variation axis's value.
#[derive(Clone, Copy, Debug, PartialEq, ThemeParts)]
#[parts(toml)]
pub enum AxisValue {
    /// A fixed value.
    Fixed(f32),
    /// The font's size (SF's and Segoe UI Variable's optical size).
    FollowsSize,
}

/// An OpenType variation axis tag: four bytes, each from `0x20` to `0x7E`
/// (OpenType's `Tag` data type).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct AxisTag([u8; 4]);

/// An OpenType feature tag: four bytes, each from `0x20` to `0x7E`
/// (OpenType's `Tag` data type).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct FeatureTag([u8; 4]);

/// The four bytes of an OpenType tag, if `tag` is one.
fn open_type_tag(tag: &str) -> Option<[u8; 4]> {
    let bytes: [u8; 4] = tag.as_bytes().try_into().ok()?;
    bytes
        .iter()
        .all(|b| (0x20..=0x7E).contains(b))
        .then_some(bytes)
}

fn write_tag(bytes: [u8; 4], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    bytes
        .into_iter()
        .try_for_each(|b| fmt::Write::write_char(f, char::from(b)))
}

impl AxisTag {
    /// The tag `tag` spells (`"wght"`, `"opsz"`), if it is four bytes from
    /// `0x20` to `0x7E`.
    #[must_use]
    pub fn new(tag: &str) -> Option<AxisTag> {
        open_type_tag(tag).map(AxisTag)
    }

    /// The tag's four bytes.
    #[must_use]
    pub const fn bytes(self) -> [u8; 4] {
        self.0
    }
}

impl fmt::Display for AxisTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_tag(self.0, f)
    }
}

impl FeatureTag {
    /// The tag `tag` spells (`"tnum"`), if it is four bytes from `0x20` to
    /// `0x7E`.
    #[must_use]
    pub fn new(tag: &str) -> Option<FeatureTag> {
        open_type_tag(tag).map(FeatureTag)
    }

    /// The tag's four bytes.
    #[must_use]
    pub const fn bytes(self) -> [u8; 4] {
        self.0
    }
}

impl fmt::Display for FeatureTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_tag(self.0, f)
    }
}

/// A BCP 47 language tag, in RFC 5646's syntax (section 2.1's ABNF): a
/// `langtag` (language with up to three extended language subtags, script,
/// region, variants, extensions, private use), a private-use tag, or one of
/// the grandfathered tags, letters in either case, held in RFC 5646's
/// canonical case (section 2.1.1), so tags differing only in case are one
/// key: `zh-hant` is `zh-Hant`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct LanguageTag(String);

impl LanguageTag {
    /// The tag `tag` spells (`"ar"`, `"zh-Hant"`, `"sl-rozaj-biske"`), if it
    /// has that syntax, in canonical case.
    #[must_use]
    pub fn new(tag: &str) -> Option<LanguageTag> {
        is_language_tag(tag).then(|| LanguageTag(canonical_case(tag)))
    }

    /// The tag in canonical case.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// `tag` in RFC 5646's canonical case (section 2.1.1): a two-letter subtag
/// uppercase (a region), a four-letter one titlecase (a script) and every
/// other lowercase; the first subtag and every subtag after a singleton (an
/// extension's or private use's) lowercase.
fn canonical_case(tag: &str) -> String {
    let mut out = String::with_capacity(tag.len());
    let mut after_singleton = false;
    for (index, subtag) in tag.split('-').enumerate() {
        if index > 0 {
            out.push('-');
        }
        let first = index == 0 || after_singleton;
        match subtag.len() {
            2 if !first => out.push_str(&subtag.to_ascii_uppercase()),
            4 if !first => {
                let mut chars = subtag.chars();
                if let Some(head) = chars.next() {
                    out.push(head.to_ascii_uppercase());
                }
                out.push_str(&chars.as_str().to_ascii_lowercase());
            }
            _ => out.push_str(&subtag.to_ascii_lowercase()),
        }
        if subtag.len() == 1 {
            after_singleton = true;
        }
    }
    out
}

/// RFC 5646's `grandfathered` tags, `irregular` then `regular`.
const GRANDFATHERED: [&str; 26] = [
    "en-GB-oed",
    "i-ami",
    "i-bnn",
    "i-default",
    "i-enochian",
    "i-hak",
    "i-klingon",
    "i-lux",
    "i-mingo",
    "i-navajo",
    "i-pwn",
    "i-tao",
    "i-tay",
    "i-tsu",
    "sgn-BE-FR",
    "sgn-BE-NL",
    "sgn-CH-DE",
    "art-lojban",
    "cel-gaulish",
    "no-bok",
    "no-nyn",
    "zh-guoyu",
    "zh-hakka",
    "zh-min",
    "zh-min-nan",
    "zh-xiang",
];

/// Whether `tag` is RFC 5646's `Language-Tag`: `langtag / privateuse /
/// grandfathered`.
fn is_language_tag(tag: &str) -> bool {
    if GRANDFATHERED.iter().any(|g| g.eq_ignore_ascii_case(tag)) {
        return true;
    }
    let subtags: Vec<&str> = tag.split('-').collect();
    match subtags.split_first() {
        Some((first, rest)) if first.eq_ignore_ascii_case("x") => is_private_use(rest),
        _ => is_langtag(&subtags),
    }
}

/// `subtag` has `len` characters, each passing `class`.
fn subtag(subtag: &str, len: std::ops::RangeInclusive<usize>, class: fn(&u8) -> bool) -> bool {
    len.contains(&subtag.len()) && subtag.bytes().all(|b| class(&b))
}

fn alpha(s: &str, len: std::ops::RangeInclusive<usize>) -> bool {
    subtag(s, len, u8::is_ascii_alphabetic)
}

fn alphanum(s: &str, len: std::ops::RangeInclusive<usize>) -> bool {
    subtag(s, len, u8::is_ascii_alphanumeric)
}

/// `variant = 5*8alphanum / (DIGIT 3alphanum)`.
fn is_variant(s: &str) -> bool {
    alphanum(s, 5..=8)
        || (alphanum(s, 4..=4) && s.bytes().next().is_some_and(|b| b.is_ascii_digit()))
}

/// `singleton`: one letter or digit but `x`.
fn is_singleton(s: &str) -> bool {
    alphanum(s, 1..=1) && !s.eq_ignore_ascii_case("x")
}

/// `1*("-" (1*8alphanum))`, after the `x` of `privateuse`.
fn is_private_use(rest: &[&str]) -> bool {
    !rest.is_empty() && rest.iter().all(|s| alphanum(s, 1..=8))
}

/// `langtag = language ["-" script] ["-" region] *("-" variant)
/// *("-" extension) ["-" privateuse]`.
fn is_langtag(subtags: &[&str]) -> bool {
    let Some((language, mut rest)) = subtags.split_first() else {
        return false;
    };
    // language = 2*3ALPHA ["-" extlang] / 4ALPHA / 5*8ALPHA,
    // extlang = 3ALPHA *2("-" 3ALPHA)
    if !alpha(language, 2..=8) {
        return false;
    }
    if language.len() <= 3 {
        for _ in 0..3 {
            match rest.split_first() {
                Some((s, after)) if alpha(s, 3..=3) => rest = after,
                _ => break,
            }
        }
    }
    // script = 4ALPHA
    if let Some((s, after)) = rest.split_first()
        && alpha(s, 4..=4)
    {
        rest = after;
    }
    // region = 2ALPHA / 3DIGIT
    if let Some((s, after)) = rest.split_first()
        && (alpha(s, 2..=2) || subtag(s, 3..=3, u8::is_ascii_digit))
    {
        rest = after;
    }
    while let Some((s, after)) = rest.split_first()
        && is_variant(s)
    {
        rest = after;
    }
    // extension = singleton 1*("-" (2*8alphanum))
    while let Some((s, after)) = rest.split_first()
        && is_singleton(s)
    {
        rest = after;
        let mut any = false;
        while let Some((s, after)) = rest.split_first()
            && alphanum(s, 2..=8)
        {
            rest = after;
            any = true;
        }
        if !any {
            return false;
        }
    }
    match rest.split_first() {
        None => true,
        Some((x, after)) => x.eq_ignore_ascii_case("x") && is_private_use(after),
    }
}

/// The nineteen semantic slots of the text ramp (S4.4), one variant per
/// `TextRoles` field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum TextRole {
    /// `body`.
    Body,
    /// `body_emphasized`.
    BodyEmphasized,
    /// `caption`.
    Caption,
    /// `caption_emphasized`.
    CaptionEmphasized,
    /// `caption2`.
    Caption2,
    /// `footnote`.
    Footnote,
    /// `subheadline`.
    Subheadline,
    /// `callout`.
    Callout,
    /// `headline`.
    Headline,
    /// `title1`.
    Title1,
    /// `title2`.
    Title2,
    /// `title3`.
    Title3,
    /// `large_title`.
    LargeTitle,
    /// `display_large`.
    DisplayLarge,
    /// `display_medium`.
    DisplayMedium,
    /// `display_small`.
    DisplaySmall,
    /// `label`.
    Label,
    /// `monospace`.
    Monospace,
    /// `numeric`.
    Numeric,
}

/// A part's text style.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct TextStyle {
    /// The font.
    #[parts(block = "fields")]
    pub font: Font,
    /// The text role whose Dynamic Type curve scales it (S9.4); `None`:
    /// `body`'s.
    pub scaling_role: Option<TextRole>,
    /// The letter spacing, in points or pixels, converted and scaled as a
    /// line height, or an expression on `font_size`; may be negative.
    #[parts(may_be_absent)]
    pub letter_spacing: Value<FontSize>,
    /// The word spacing, as the letter spacing.
    #[parts(may_be_absent)]
    pub word_spacing: Value<FontSize>,
    /// The line height, in points or pixels (converted and scaled as a font
    /// size, S9.4), an expression on `font_size`, or `Auto`: the font's own
    /// line spacing.
    #[parts(auto)]
    pub line_height: Value<FontSize>,
    /// The decoration.
    #[parts(block = "fields", may_be_absent)]
    pub decoration: Value<Decoration>,
    /// The mnemonic's underline.
    #[parts(block = "fields", may_be_absent)]
    pub mnemonic_decoration: Value<Decoration>,
    /// The case transform.
    pub case: TextCase,
    /// How text that does not fit is cut.
    #[parts(block = "set")]
    pub elision: Elision,
    /// How text wraps.
    pub wrap: Wrap,
    /// Wrap to at most this many lines, then elide; `None`: no limit.
    pub max_lines: Option<u32>,
    /// The text's paint.
    #[parts(block = "append")]
    pub fill: Value<Paint>,
}

/// A case transform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum TextCase {
    /// As written.
    Mixed,
    /// Upper case.
    Upper,
    /// Lower case.
    Lower,
    /// Small capitals, the only route for them.
    SmallCaps,
    /// Each word capitalised.
    Capitalize,
}

/// A line drawn with text. Where the platform draws it from the font's own
/// metrics, the preset states `thickness` and `offset` as references to the
/// part's font metrics (`= element.font_underline_thickness`); a number only
/// where the platform draws a fixed line.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Decoration {
    /// Which line.
    pub line: DecorationLine,
    /// Its pattern.
    pub style: DecorationStyle,
    /// Drawn under words only, not the spaces between (AppKit's
    /// `NSUnderlineStyle.byWord`).
    #[parts(may_be_absent)]
    pub by_word: Value<bool>,
    /// The line's height.
    pub thickness: Value<Px>,
    /// The distance from the baseline to the line's top edge, positive
    /// downward (an overline's and a strike line's are negative).
    pub offset: Value<Px>,
    /// The line's paint.
    #[parts(block = "append")]
    pub paint: Value<Paint>,
}

/// A decoration's line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum DecorationLine {
    /// Under the text.
    Underline,
    /// Over the text.
    Overline,
    /// Through the text.
    Strike,
}

/// A decoration's pattern (AppKit's patterns).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum DecorationStyle {
    /// A solid line.
    Solid,
    /// Two lines.
    Double,
    /// Dots.
    Dotted,
    /// Dashes.
    Dashed,
    /// Dashes and dots.
    DashDot,
    /// Dashes and pairs of dots.
    DashDotDot,
    /// A wavy line.
    Wavy,
}

/// How text that does not fit is cut.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum Elision {
    /// No ellipsis.
    None,
    /// An ellipsis at the end.
    End,
    /// An ellipsis in the middle.
    Middle,
    /// An ellipsis at the start.
    Start,
    /// A fade at either or both edges.
    Fade {
        /// The fade at the start edge; `None`: no fade there.
        #[parts(block = "set")]
        start: Option<FadeLength>,
        /// The fade at the end edge; `None`: no fade there.
        #[parts(block = "set")]
        end: Option<FadeLength>,
        /// What the fade ends under; `None`: the text's own edge.
        under: Option<FadeUnder>,
    },
}

/// A fade's length.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum FadeLength {
    /// In logical pixels.
    Px(Value<Px>),
    /// A fraction of the box.
    Fraction(Value<Fraction>),
}

/// How text wraps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Wrap {
    /// It does not wrap.
    None,
    /// At word boundaries.
    Word,
    /// At any character.
    Char,
}

/// What a fade ends under.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum FadeUnder {
    /// A part.
    Part(PartRef),
    /// A nested element (a tab's close button).
    Element {
        /// The element's widget.
        widget: WidgetRef,
        /// Its variant, where one is named.
        variant: Option<VariantRef>,
    },
}
