//! Typography: the text styles of a theme's content roles and how text is
//! rendered.

use std::sync::Arc;

use super::style::{TextStated, TextStyle};

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

impl Typography {
    /// The text style a slot's text is drawn in: `slot` (one of the optional
    /// slots, `self.caption.as_ref()`) laid over `body`; `body` where `slot`
    /// is `None`.
    ///
    /// A slot states only what its source states for that style (0.7.2's
    /// `text_scale` entry: a size, a weight and a line height). A property it
    /// leaves unstated is `body`'s, property by property, as 0.7.2 drew a
    /// role in `defaults.font`'s family, style and colour. The properties
    /// that belong to a size (`line_height`, `letter_spacing`,
    /// `optical_size`, and `defined_size` with `size_px`) are `body`'s only
    /// where the slot states no size; beside a size of its own they stay the
    /// slot's, stated or not. The returned style's `stated` bits are the
    /// slot's and those taken from `body`; a property neither states stays
    /// unstated, and `from_state` is the slot's. The slot itself is not
    /// changed, and `monospace` is never laid over `body`.
    #[must_use]
    pub fn over_body(&self, slot: Option<&TextStyle>) -> TextStyle {
        let Some(slot) = slot else {
            return self.body.clone();
        };
        let body = &self.body;
        let mut taken = body.stated.difference(slot.stated);
        if slot.stated.contains(TextStated::SIZE_PX) {
            taken = taken.difference(
                TextStated::LINE_HEIGHT | TextStated::LETTER_SPACING | TextStated::OPTICAL_SIZE,
            );
        }
        let mut out = slot.clone();
        let take = |bit: TextStated| taken.contains(bit);
        if take(TextStated::FAMILY) {
            out.family = body.family.clone();
        }
        if take(TextStated::COLOR) {
            out.color = body.color;
        }
        if take(TextStated::SIZE_PX) {
            out.size_px = body.size_px;
            out.defined_size = body.defined_size;
        }
        if take(TextStated::WEIGHT) {
            out.weight = body.weight;
        }
        if take(TextStated::STYLE) {
            out.style = body.style;
        }
        if take(TextStated::WIDTH) {
            out.width = body.width;
        }
        if take(TextStated::LINE_HEIGHT) {
            out.line_height = body.line_height;
        }
        if take(TextStated::LETTER_SPACING) {
            out.letter_spacing = body.letter_spacing;
        }
        if take(TextStated::OPTICAL_SIZE) {
            out.optical_size = body.optical_size;
        }
        if take(TextStated::DECORATION) {
            out.decoration = body.decoration;
        }
        if take(TextStated::BLEND_MODE) {
            out.blend_mode = body.blend_mode;
        }
        out.stated |= taken;
        out
    }
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
    use crate::next::color::Color;
    use crate::next::preset::{Preset, Theme};
    use crate::next::query::ThemeContext;
    use crate::next::style::FontSize;

    /// The optional slots of `t`, by name.
    fn slots(t: &Typography) -> [(&'static str, Option<&TextStyle>); 9] {
        [
            ("display", t.display.as_ref()),
            ("title1", t.title1.as_ref()),
            ("title2", t.title2.as_ref()),
            ("title3", t.title3.as_ref()),
            ("title4", t.title4.as_ref()),
            ("headline", t.headline.as_ref()),
            ("callout", t.callout.as_ref()),
            ("subheadline", t.subheadline.as_ref()),
            ("caption", t.caption.as_ref()),
        ]
    }

    fn typographies() -> Vec<(String, Typography)> {
        let mut out = Vec::new();
        for &preset in Preset::ALL {
            for ctx in [ThemeContext::light(), ThemeContext::dark()] {
                let label = format!("{preset:?} {:?}", ctx.scheme);
                out.push((label, Theme::preset(preset).resolve(&ctx).typography));
            }
        }
        out
    }

    #[test]
    fn a_preset_slot_keeps_its_size_and_takes_the_bodys_family_colour_and_style() {
        let mut checked = 0;
        for (label, t) in typographies() {
            for (name, slot) in slots(&t) {
                let Some(slot) = slot else { continue };
                let at = format!("{label} {name}");
                assert!(!slot.stated.contains(TextStated::FAMILY), "{at}");
                let drawn = t.over_body(Some(slot));
                assert_eq!(
                    (drawn.size_px, drawn.weight, drawn.line_height),
                    (slot.size_px, slot.weight, slot.line_height),
                    "{at}"
                );
                assert_eq!(drawn.defined_size, slot.defined_size, "{at}");
                assert_eq!(drawn.family, t.body.family, "{at}");
                assert_eq!(drawn.color, t.body.color, "{at}");
                assert_eq!(drawn.style, t.body.style, "{at}");
                assert_eq!(
                    drawn.stated,
                    slot.stated | TextStated::FAMILY | TextStated::COLOR | TextStated::STYLE,
                    "{at}"
                );
                assert_eq!(drawn.from_state, slot.from_state, "{at}");
                checked += 1;
            }
        }
        assert!(checked > 0);
    }

    #[test]
    fn no_slot_is_the_body() {
        for (label, t) in typographies() {
            assert_eq!(t.over_body(None), t.body, "{label}");
        }
    }

    /// Hand-built text: test inputs, not theme values.
    const BODY_SIZE: f32 = 13.0;
    const BODY_POINTS: f32 = 10.0;
    const BODY_LINE: f32 = 18.0;
    const BODY_SPACING: f32 = 0.5;
    const SLOT_SIZE: f32 = 20.0;
    const SLOT_WEIGHT: u16 = 700;

    fn hand_built() -> Typography {
        let mut t = Theme::preset(Preset::KdeBreeze)
            .resolve(&ThemeContext::light())
            .typography;
        t.body = TextStyle {
            family: Arc::from("Body Sans"),
            color: Color::rgb(0.0, 0.0, 1.0),
            size_px: BODY_SIZE,
            defined_size: Some(FontSize::Pt(BODY_POINTS)),
            line_height: BODY_LINE,
            letter_spacing: BODY_SPACING,
            stated: TextStated::FAMILY
                | TextStated::COLOR
                | TextStated::SIZE_PX
                | TextStated::WEIGHT
                | TextStated::LINE_HEIGHT
                | TextStated::LETTER_SPACING,
            ..TextStyle::default()
        };
        t
    }

    #[test]
    fn a_slot_without_a_size_takes_the_bodys_size_and_what_belongs_to_it() {
        let t = hand_built();
        let slot = TextStyle {
            weight: SLOT_WEIGHT,
            stated: TextStated::WEIGHT,
            ..TextStyle::default()
        };
        let drawn = t.over_body(Some(&slot));
        assert_eq!(drawn.weight, SLOT_WEIGHT);
        assert_eq!(drawn.size_px, BODY_SIZE);
        assert_eq!(drawn.defined_size, Some(FontSize::Pt(BODY_POINTS)));
        assert_eq!(drawn.line_height, BODY_LINE);
        assert_eq!(drawn.letter_spacing, BODY_SPACING);
        assert_eq!(&*drawn.family, "Body Sans");
        assert_eq!(drawn.stated, t.body.stated);
    }

    #[test]
    fn a_slot_with_a_size_keeps_what_belongs_to_its_size() {
        let t = hand_built();
        let slot = TextStyle {
            size_px: SLOT_SIZE,
            stated: TextStated::SIZE_PX,
            ..TextStyle::default()
        };
        let drawn = t.over_body(Some(&slot));
        assert_eq!(drawn.size_px, SLOT_SIZE);
        assert_eq!(drawn.defined_size, None);
        assert!(t.body.stated.contains(TextStated::LINE_HEIGHT));
        assert!(!drawn.stated.contains(TextStated::LINE_HEIGHT));
        assert_eq!(drawn.line_height, slot.line_height);
        assert!(t.body.stated.contains(TextStated::LETTER_SPACING));
        assert!(!drawn.stated.contains(TextStated::LETTER_SPACING));
        assert_eq!(drawn.letter_spacing, slot.letter_spacing);
        assert_eq!(&*drawn.family, "Body Sans");
        assert_eq!(drawn.weight, t.body.weight);
        assert_eq!(
            drawn.stated,
            TextStated::SIZE_PX | TextStated::FAMILY | TextStated::COLOR | TextStated::WEIGHT
        );
    }

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
