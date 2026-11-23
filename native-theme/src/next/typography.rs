//! Typography: the text styles of a theme's content roles and how text is
//! rendered.

use std::sync::Arc;

use super::style::TextStyle;

/// The text styles of a resolved theme.
///
/// `body` and `monospace` are always present. Every other slot is `None` on a
/// platform that defines no such style: no size is invented for it, and a
/// caller falls back to `body`. The slots are content roles, not a platform's
/// own style names. 0.7.2's four `text_scale` roles (`platform-facts.md`
/// §2.19): `caption` is `caption`, `section_heading` is `headline`,
/// `dialog_title` is `title1`, `display` is `display` (so libadwaita's
/// `.title-2` is `title1` and its `.title-1` is `display`). A slot's
/// `line_height`, where its source states none, is `line_height_multiplier`
/// times the slot's size before whole-pixel rounding (0.7.2's rule for a
/// `text_scale` entry); `body` follows it too.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Typography {
    /// The largest display text.
    pub display: Option<TextStyle>,
    /// A first-level title (0.7.2's `text_scale.dialog_title`).
    pub title1: Option<TextStyle>,
    /// A second-level title.
    pub title2: Option<TextStyle>,
    /// A third-level title.
    pub title3: Option<TextStyle>,
    /// A fourth-level title.
    pub title4: Option<TextStyle>,
    /// A headline (0.7.2's `text_scale.section_heading`).
    pub headline: Option<TextStyle>,
    /// Body text; always present.
    pub body: TextStyle,
    /// A callout.
    pub callout: Option<TextStyle>,
    /// A subheadline.
    pub subheadline: Option<TextStyle>,
    /// A caption (0.7.2's `text_scale.caption`).
    pub caption: Option<TextStyle>,
    /// Monospaced text; always present.
    pub monospace: TextStyle,
    /// The global default line-height multiplier (0.7.2's
    /// `defaults.line_height`).
    pub line_height_multiplier: f32, // global default multiplier (0.7.2's `defaults.line_height`)
    /// The fallback font families, in order.
    pub fallbacks: Arc<[Arc<str>]>,
    /// The emoji font family; 0.7.2 has none.
    pub emoji_family: Option<Arc<str>>, // 0.7.2 has none
    /// How text is rendered.
    pub rendering: TextRendering,
    /// The font DPI: 72.0 on Apple platforms, 96.0 by default on Windows and
    /// Linux, or KDE's live `Xft.dpi`.
    pub font_dpi: f32, // 72.0 Apple, 96.0 default Windows/Linux, or live KDE Xft.dpi
    /// Whether font sizes are rounded to whole pixels: `Some(true)` on
    /// Qt/KDE; `None` where unstated (as in 0.7.2).
    pub font_whole_pixels: Option<bool>, // Some(true) on Qt/KDE; None where unstated (as in 0.7.2)
}

/// How text is rendered; each `None` where neither the preset nor the live
/// system states it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TextRendering {
    /// The antialiasing.
    pub antialiasing: Option<Antialiasing>,
    /// The hinting.
    pub hinting: Option<FontHinting>,
    /// The subpixel order.
    pub subpixel_order: Option<SubpixelOrder>,
    /// Font smoothing (macOS `AppleFontSmoothing`).
    pub smoothing: Option<bool>, // macOS AppleFontSmoothing
}

impl TextRendering {
    /// Text rendering of these fields.
    #[must_use]
    pub fn new(
        antialiasing: Option<Antialiasing>,
        hinting: Option<FontHinting>,
        subpixel_order: Option<SubpixelOrder>,
        smoothing: Option<bool>,
    ) -> Self {
        Self {
            antialiasing,
            hinting,
            subpixel_order,
            smoothing,
        }
    }
}

/// Font antialiasing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// no `Default`: the type stands only in `Option` fields, where a default would be an invented value
#[non_exhaustive]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Antialiasing {
    /// None.
    None,
    /// Greyscale.
    Grayscale,
    /// Subpixel.
    Subpixel,
}

/// Font hinting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// no `Default`: the type stands only in `Option` fields, where a default would be an invented value
#[non_exhaustive]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum FontHinting {
    /// None.
    None,
    /// Slight.
    Slight,
    /// Medium.
    Medium,
    /// Full.
    Full,
}

/// The order of a display's subpixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// no `Default`: the type stands only in `Option` fields, where a default would be an invented value
#[non_exhaustive]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum SubpixelOrder {
    /// No subpixel order.
    None,
    /// Red, green, blue, horizontally.
    Rgb,
    /// Blue, green, red, horizontally.
    Bgr,
    /// Red, green, blue, vertically.
    Vrgb,
    /// Blue, green, red, vertically.
    Vbgr,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_rendering_constructor_and_default() {
        let d = TextRendering::default();
        assert_eq!(d, TextRendering::new(None, None, None, None));
        let r = TextRendering::new(
            Some(Antialiasing::Subpixel),
            Some(FontHinting::Slight),
            Some(SubpixelOrder::Rgb),
            None,
        );
        assert_eq!(r.antialiasing, Some(Antialiasing::Subpixel));
        assert_eq!(r.hinting, Some(FontHinting::Slight));
        assert_eq!(r.subpixel_order, Some(SubpixelOrder::Rgb));
        assert_eq!(r.smoothing, None);
    }
}
