//! The clean room's two GNOME helpers (the 0.8.0 plan's D10), their code
//! placed unchanged in `src/cleanroom_readers/` (their vector tests named
//! `vectors_reader_<helper>` and reading `tests/vectors/`), and what the reader keeps of what
//! they give: Pango's font-description parser, [`pango_font_description`],
//! for the gsettings font strings, and libadwaita's accent classification,
//! [`adw_accent_nearest`], for the portal's accent (S9.3, S9.5).

#[path = "../cleanroom_readers/adw_accent_nearest.rs"]
#[allow(
    clippy::excessive_precision,
    reason = "the clean room's constants, placed unchanged (D10): each decimal as libadwaita writes it, rounded once to a single"
)]
mod adw_accent_nearest;
#[path = "../cleanroom_readers/pango_font_description.rs"]
#[allow(
    clippy::collapsible_if,
    clippy::manual_ignore_case_cmp,
    clippy::type_complexity,
    reason = "the clean room's parser, placed unchanged (D10): style lints only"
)]
mod pango_font_description;

pub(crate) use adw_accent_nearest::{AdwAccent, adw_accent_nearest};
pub(crate) use pango_font_description::{PangoDescription, pango_font_description};

use crate::blocks::text::{FontPatch, FontStyle, TextCase, TextStylePatch};
use crate::color::Color;
use crate::units::FontSize;
use crate::value::Value;

/// The colour of libadwaita's accent `accent`, as `adw_accent_color_to_rgba`
/// gives it (`libadwaita@1.10.0 src/adw-accent-color.c:52-80`), the colour
/// its stylesheet draws that accent with. Blue's is the `adwaita` preset's
/// own accent (`docs/facts/gnome/roles.toml:5565-5583`); the other eight
/// are cited to the source here, their facts pending (ledger queue #171).
pub(crate) fn accent_color(accent: AdwAccent) -> Option<Color> {
    Color::parse(match accent {
        // src/adw-accent-color.c:53-54
        AdwAccent::Blue => "#3584e4",
        // :56-57
        AdwAccent::Teal => "#2190a4",
        // :59-60
        AdwAccent::Green => "#3a944a",
        // :62-63
        AdwAccent::Yellow => "#c88800",
        // :65-66
        AdwAccent::Orange => "#ed5b00",
        // :68-69
        AdwAccent::Red => "#e62d42",
        // :71-72
        AdwAccent::Pink => "#d56199",
        // :74-75
        AdwAccent::Purple => "#9141ac",
        // :77-78
        AdwAccent::Slate => "#6f8396",
    })
}

/// Pango's units per point, or per pixel for an absolute size, `PANGO_SCALE`
/// (`pango@1.58.2 pango/pango-types.h:96`).
const PANGO_SCALE: f64 = 1024.0;

/// The points per inch of S9.4's conversion, `pt × dpi / 72`.
const POINTS_PER_INCH: f64 = 72.0;

/// Pango's width integer per percent of normal: the description holds it
/// in permille (`PangoDescription::width`), the model's `Font.width` in
/// percent (S2.8).
const PERMILLE_PER_PERCENT: f32 = 10.0;

/// The weights the model's `Font.weight` takes, the CSS and OpenType ones
/// (S2.8, S12).
const WEIGHTS: std::ops::RangeInclusive<i32> = 1..=1000;

/// The text style a gsettings font string `text` (`font-name`,
/// `monospace-font-name`) patches a text-role slot with, its size in
/// logical pixels converted by `font_dpi` (S9.4, S9.5); `None` where the
/// string names no family, which names no font, so the slot keeps the
/// preset's.
///
/// Of Pango's description: the family, the first non-empty entry of its
/// comma-separated list (the model's family is one name); the size,
/// unpatched where it is unset or zero (a zero size names no usable font)
/// or, for a point size, where `font_dpi` is `None`; the weight, unpatched
/// outside 1 to 1000; the style (Normal, Oblique, Italic; another
/// integer, which only `style=N` gives, unpatched); the width, permille to
/// percent; small caps as [`TextCase::SmallCaps`]. Its other variants,
/// gravity, colour, variations and features are not read.
pub(crate) fn font_patch(text: &str, font_dpi: Option<f32>) -> Option<TextStylePatch> {
    let desc = pango_font_description(text);
    let family = desc
        .family
        .as_deref()?
        .split(',')
        .find(|family| !family.is_empty())?
        .to_owned();
    let font = FontPatch {
        family: Some(Value::Stated(family)),
        size: font_size(&desc, font_dpi).map(Value::Stated),
        weight: WEIGHTS
            .contains(&desc.weight)
            .then(|| u16::try_from(desc.weight).ok())
            .flatten()
            .map(Value::Stated),
        style: font_style(desc.style).map(Value::Stated),
        width: Some(Value::Stated(desc.width as f32 / PERMILLE_PER_PERCENT)),
        ..FontPatch::default()
    };
    Some(TextStylePatch {
        font: Some(font),
        case: (desc.variant == SMALL_CAPS).then_some(Value::Stated(TextCase::SmallCaps)),
        ..TextStylePatch::default()
    })
}

/// `PANGO_VARIANT_SMALL_CAPS`, the variant integer of the `Small-Caps`
/// word (`pango@1.58.2 pango/pango-font.h:100-108`).
const SMALL_CAPS: i32 = 1;

/// The description's size in logical pixels: an absolute size as it is, a
/// point size converted by `font_dpi`; `None` where the size is unset or
/// zero, or a point size meets no DPI.
fn font_size(desc: &PangoDescription, font_dpi: Option<f32>) -> Option<FontSize> {
    let units = f64::from(desc.size.filter(|size| *size > 0)?);
    let size = units / PANGO_SCALE;
    let px = if desc.absolute {
        size
    } else {
        size * f64::from(font_dpi?) / POINTS_PER_INCH
    };
    Some(FontSize::Px(px as f32))
}

/// Pango's style integer as the model's style: Normal 0, Oblique 1, Italic
/// 2 (`pango@1.58.2 pango/pango-font.h:74-78`).
fn font_style(style: i32) -> Option<FontStyle> {
    match style {
        0 => Some(FontStyle::Normal),
        1 => Some(FontStyle::Oblique),
        2 => Some(FontStyle::Italic),
        _ => None,
    }
}
