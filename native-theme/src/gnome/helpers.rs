//! The clean room's two GNOME helpers (the 0.8.0 plan's D10), their code
//! placed in `src/cleanroom_readers/`, unchanged but for its panic-free
//! arithmetic (their vector tests named
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
    reason = "the clean room's parser, its style kept as written (D10): style lints only"
)]
mod pango_font_description;

pub(crate) use adw_accent_nearest::{AdwAccent, adw_accent_nearest};
pub(crate) use pango_font_description::{PangoDescription, pango_font_description};

use crate::blocks::text::{FontPatch, TextStylePatch};
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

/// The text style a gsettings font string `text` (`font-name`,
/// `monospace-font-name`) patches a text-role slot with, its size in
/// logical pixels converted by `font_dpi` (S9.4, S9.5); `None` where the
/// string names no family, which names no font, so the slot keeps the
/// preset's.
///
/// Of Pango's description: the family, the first non-empty entry of its
/// comma-separated list (the model's family is one name), and the size,
/// unpatched where it is unset or zero (a zero size names no usable font)
/// or, for a point size, where `font_dpi` is `None`; nothing else. GTK 4
/// takes only the family and the size from `gtk-font-name`, never its
/// weight (`gtk@4.24.1 gtk/gtksettings.c:1356-1390`), so the slot's weight,
/// style, width and case stay the preset's (S9.3's *The GNOME reader's
/// readings*, ADD-A141); `monospace` takes the same two fields.
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
        ..FontPatch::default()
    };
    Some(TextStylePatch {
        font: Some(font),
        ..TextStylePatch::default()
    })
}

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
