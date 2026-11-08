//! What the KDE reader keeps of a kdeglobals font entry: the clean room's
//! Qt font parser, [`qfont_from_string`] (the 0.8.0 plan's D10), mapped onto
//! the text style a text-role slot is patched with (S9.4, S9.5).

use super::qfont_from_string::{QtFontFields, qfont_from_string};

use crate::blocks::text::{FontPatch, FontStyle, TextCase, TextStylePatch};
use crate::units::FontSize;
use crate::value::Value;

/// The points per inch of S9.4's conversion, `pt × dpi / 72`.
const POINTS_PER_INCH: f64 = 72.0;

/// The weights the model's `Font.weight` takes, the CSS and OpenType ones
/// (S2.8, S12).
const WEIGHTS: std::ops::RangeInclusive<i32> = 1..=1000;

/// The text style a kdeglobals font entry `text` (`[General] font`,
/// `fixed`, `smallestReadableFont`, `toolBarFont`, `menuFont`) patches a
/// text-role slot or a keyed text style with, its size in logical pixels
/// converted by `font_dpi` (S9.4); `None` where Qt refuses the string or
/// the call sets none of the fields kept, so the slot keeps the preset's.
///
/// Plasma starts each font from its default and applies the string over it
/// (the clean room's description, *Where KDE uses it*), so a field the
/// string leaves keeps the preset's, which the facts state as that default.
/// Of the fields Qt sets: the family where it is not empty; the size, a
/// pixel size as it is, a point size converted by `font_dpi`, unpatched
/// where it is not above zero and finite or, for a point size, where
/// `font_dpi` is `None`; the weight, unpatched outside 1
/// to 1000; the style (`QFont::Style`: 0 normal, 1 italic, 2 oblique; 3,
/// no member, unpatched); and the capitalization, the style's `case`
/// ([`text_case`]). The stretch, spacings, style name, features and axes
/// are the readers unit's reads (the KDE trace's K3.46), not read here
/// (S9.3's *The KDE reader's readings*, ADD-A140).
pub(super) fn font_patch(text: &str, font_dpi: Option<f32>) -> Option<TextStylePatch> {
    let fields = qfont_from_string(text)?;
    let case = fields.capitalization.and_then(text_case).map(Value::Stated);
    let font = FontPatch {
        family: fields
            .family
            .clone()
            .filter(|family| !family.is_empty())
            .map(Value::Stated),
        size: font_size(&fields, font_dpi).map(Value::Stated),
        weight: fields
            .weight
            .filter(|weight| WEIGHTS.contains(weight))
            .and_then(|weight| u16::try_from(weight).ok())
            .map(Value::Stated),
        style: fields.style.and_then(font_style).map(Value::Stated),
        ..FontPatch::default()
    };
    let font = (font != FontPatch::default()).then_some(font);
    (font.is_some() || case.is_some()).then(|| TextStylePatch {
        font,
        case,
        ..TextStylePatch::default()
    })
}

/// Qt's capitalization integer as the style's `case`: each
/// `QFont::Capitalization` but `MixedCase` the `TextCase` member of the
/// same meaning, `AllUppercase` (1) `Upper`, `AllLowercase` (2) `Lower`,
/// `SmallCaps` (3) and `Capitalize` (4) (`qtbase@6.12.0
/// src/gui/text/qfont.h:99-105`, `qfont.cpp:1742-1746`); `MixedCase` (0),
/// "the normal text rendering option where no capitalization change is
/// applied", leaves the style's `case`, and so does a number no member
/// holds.
fn text_case(capitalization: u8) -> Option<TextCase> {
    match capitalization {
        1 => Some(TextCase::Upper),
        2 => Some(TextCase::Lower),
        3 => Some(TextCase::SmallCaps),
        4 => Some(TextCase::Capitalize),
        _ => None,
    }
}

/// The size Qt set, in logical pixels: a pixel size as it is, a point size
/// converted by `font_dpi`; `None` where neither is set, above zero and
/// finite, or a point size meets no DPI.
fn font_size(fields: &QtFontFields, font_dpi: Option<f32>) -> Option<FontSize> {
    if let Some(px) = fields.pixel_size.filter(|px| *px > 0) {
        return Some(FontSize::Px(px as f32));
    }
    let pt = fields.point_size.filter(|pt| pt.is_finite() && *pt > 0.0)?;
    let px = pt * f64::from(font_dpi?) / POINTS_PER_INCH;
    (px.is_finite() && px > 0.0).then_some(FontSize::Px(px as f32))
}

/// Qt's style integer as the model's style: Normal 0, Italic 1, Oblique 2
/// (the clean room's description, `qfont_from_string`'s step 6).
fn font_style(style: u8) -> Option<FontStyle> {
    match style {
        0 => Some(FontStyle::Normal),
        1 => Some(FontStyle::Italic),
        2 => Some(FontStyle::Oblique),
        _ => None,
    }
}
