//! Part, text and icon styles, the bit sets saying which of their properties
//! a theme states, and the focus indicator and ripple a part may carry.

use std::sync::Arc;

use super::color::Color;
use super::geometry::{Corners, Cursor, Edges, ShadowLayer, Stroke, Transform};
use super::motion::Transition;
use super::paint::{BlendMode, Paint};

/// How a part shows that it has the focus.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct FocusIndicator {
    /// The indicator's shape.
    pub kind: FocusKind,
    /// Its stroke.
    pub stroke: Stroke,
    /// A second stroke inside the first.
    pub inner_stroke: Stroke, // e.g. WinUI dual white/black focus ring; Stroke::None when unused
    /// Its distance from the part's edge, in logical pixels.
    pub offset: f32, // positive = outside part, negative = inside
    /// Its corners.
    pub corners: Option<Corners>, // None = concentric with part corners
    /// When it is shown.
    pub trigger: Option<FocusTrigger>, // None where the theme states no keyboard-only vs always rule (0.7.2's `focus_ring_keyboard_only: Option<bool>`)
}

impl FocusIndicator {
    /// An indicator of these fields.
    #[must_use]
    pub fn new(
        kind: FocusKind,
        stroke: Stroke,
        inner_stroke: Stroke,
        offset: f32,
        corners: Option<Corners>,
        trigger: Option<FocusTrigger>,
    ) -> Self {
        Self {
            kind,
            stroke,
            inner_stroke,
            offset,
            corners,
            trigger,
        }
    }
}

/// The shape of a [`FocusIndicator`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FocusKind {
    /// A ring around the part.
    #[default]
    Ring,
    /// A fill of the part.
    Fill,
    /// A line under the part.
    Underline,
}

/// When a [`FocusIndicator`] is shown.
///
/// A connector draws a stated indicator when `trigger == Some(KeyboardOnly)`
/// and the state contains `FOCUS_VISIBLE`, when `trigger == Some(Always)` and
/// the state intersects `FOCUSED | FOCUS_VISIBLE`, and when `trigger` is
/// `None` by the toolkit's own rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// no `Default`: the type stands only in `Option` fields, where a default would be an invented value
#[non_exhaustive]
pub enum FocusTrigger {
    /// Only when the focus came from the keyboard.
    KeyboardOnly,
    /// Whenever the part has the focus.
    Always,
}

/// A touch ripple.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct RippleSpec {
    /// The ripple's colour.
    pub color: Color,
    /// Whether it is clipped to the part.
    pub bounded: bool,
    /// Its entrance duration, in milliseconds.
    pub enter_ms: u32,
    /// Its exit duration, in milliseconds.
    pub exit_ms: u32,
}

impl RippleSpec {
    /// A ripple of these fields.
    #[must_use]
    pub fn new(color: Color, bounded: bool, enter_ms: u32, exit_ms: u32) -> Self {
        Self {
            color,
            bounded,
            enter_ms,
            exit_ms,
        }
    }
}

/// The style of a drawn part of a widget.
///
/// A property whose bit in [`stated`](Self::stated) is clear is not stated by
/// the theme: it holds its neutral default and a connector keeps the
/// toolkit's own value.
///
/// A property whose bit in [`from_state`](Self::from_state) is set got its
/// value from a rule whose `required_states` is not empty, i.e. a value the
/// theme states *for this state*. A query returns the resting value for a
/// state without a rule, and a value stated equal to the resting one looks
/// the same, so the bit says which it is. Where it is clear for a state the
/// toolkit has its own treatment of (a pressed fill, a hover outline), the
/// theme states nothing for that state and a connector keeps the toolkit's
/// own treatment, exactly as for an unstated property; where it is set, the
/// theme's value is drawn. A rule that matches by variant, size, surface,
/// position or orientation alone sets no bit.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PartStyle {
    /// The fill.
    pub fill: Paint, // 24 B
    /// The outline.
    pub stroke: Stroke, // 24 B
    /// The corners.
    pub corners: Corners, // 24 B
    /// The shadows; `None` with the bit set states that there are none.
    pub shadows: Option<Arc<[ShadowLayer]>>, // 16 B
    /// The opacity, `0.0..=1.0`.
    pub opacity: f32, // 4 B  (default 1.0)
    /// The transform.
    pub transform: Transform, // 28 B
    /// The space between the stroke's inner edge and the content.
    pub padding: Edges, // 16 B  between the stroke's inner edge and the content, as 0.7.2's padding
    /// The space around the part.
    pub margin: Edges, // 16 B
    /// The smallest `[width, height]`; `None` where no floor is stated.
    pub min_size: [Option<f32>; 2], // 16 B [width, height]
    /// The largest `[width, height]`; `None` where no ceiling is stated.
    pub max_size: [Option<f32>; 2], // 16 B [width, height]
    /// The focus indicator.
    pub focus: Option<Arc<FocusIndicator>>, // 8 B
    /// The touch ripple.
    pub ripple: Option<Arc<RippleSpec>>, // 8 B
    /// The transition between states.
    pub transition: Option<Transition>, // 28 B
    /// The pointer cursor.
    pub cursor: Cursor, // 1 B
    /// Which properties the theme states.
    pub stated: PartStated, // 4 B  which properties the theme states
    /// Which properties got their value from a rule that requires a state
    /// flag of the query.
    pub from_state: PartStated, // 4 B
} // Total <= 256 B (240 B as laid out by rustc 1.100, `from_state` included); zero heap allocation on Clone.

impl Default for PartStyle {
    fn default() -> Self {
        Self {
            fill: Paint::None,
            stroke: Stroke::None,
            corners: Corners::NONE,
            shadows: None,
            opacity: 1.0,
            transform: Transform::IDENTITY,
            padding: Edges::ZERO,
            margin: Edges::ZERO,
            min_size: [None, None],
            max_size: [None, None],
            focus: None,
            ripple: None,
            transition: None,
            cursor: Cursor::Default,
            stated: PartStated::default(),
            from_state: PartStated::default(),
        }
    }
}

/// One bit per `PartStyle` property the theme states; `padding` and `margin` have one bit per side.
/// `min_size` and `max_size` need none: each dimension is its own `Option`. A size its source gives as exact
/// (a check box's indicator, a row's height) is stated as `min_size` and `max_size` equal in that dimension;
/// a floor is `min_size` alone.
/// For the other `Option`-typed properties (`shadows`, `focus`, `ripple`, `transition`) the bit tells two
/// meanings of `None` apart: bit clear, the theme states nothing and the toolkit's own applies; bit set, the
/// theme states that the part has none. An empty shadow list is not used. The same holds for the `Option`
/// fields of `TextStyle` and `IconStyle`.
///
/// In [`PartStyle::from_state`] the type has two bits more,
/// [`MIN_SIZE`](Self::MIN_SIZE) and [`MAX_SIZE`](Self::MAX_SIZE), one each
/// for both dimensions: a rule that requires a state wrote that size (a
/// hover thickness). They are never set in [`PartStyle::stated`], where each
/// dimension's `Option` says whether it is stated.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PartStated(u32);

impl PartStated {
    /// `fill` is stated.
    pub const FILL: Self = Self(1 << 0);
    /// `stroke` is stated.
    pub const STROKE: Self = Self(1 << 1);
    /// `corners` is stated.
    pub const CORNERS: Self = Self(1 << 2);
    /// `shadows` is stated.
    pub const SHADOWS: Self = Self(1 << 3);
    /// `opacity` is stated.
    pub const OPACITY: Self = Self(1 << 4);
    /// `transform` is stated.
    pub const TRANSFORM: Self = Self(1 << 5);
    /// `padding.top` is stated.
    pub const PADDING_TOP: Self = Self(1 << 6);
    /// `padding.right` is stated.
    pub const PADDING_RIGHT: Self = Self(1 << 7);
    /// `padding.bottom` is stated.
    pub const PADDING_BOTTOM: Self = Self(1 << 8);
    /// `padding.left` is stated.
    pub const PADDING_LEFT: Self = Self(1 << 9);
    /// `margin.top` is stated.
    pub const MARGIN_TOP: Self = Self(1 << 10);
    /// `margin.right` is stated.
    pub const MARGIN_RIGHT: Self = Self(1 << 11);
    /// `margin.bottom` is stated.
    pub const MARGIN_BOTTOM: Self = Self(1 << 12);
    /// `margin.left` is stated.
    pub const MARGIN_LEFT: Self = Self(1 << 13);
    /// `focus` is stated.
    pub const FOCUS: Self = Self(1 << 14);
    /// `ripple` is stated.
    pub const RIPPLE: Self = Self(1 << 15);
    /// `transition` is stated.
    pub const TRANSITION: Self = Self(1 << 16);
    /// `cursor` is stated.
    pub const CURSOR: Self = Self(1 << 17);
    /// `min_size` was written by a rule that requires a state, either
    /// dimension: set only in `from_state`, never in `stated`.
    pub const MIN_SIZE: Self = Self(1 << 18);
    /// `max_size` was written by a rule that requires a state, either
    /// dimension: set only in `from_state`, never in `stated`.
    pub const MAX_SIZE: Self = Self(1 << 19);
    /// Every bit, [`MIN_SIZE`](Self::MIN_SIZE) and
    /// [`MAX_SIZE`](Self::MAX_SIZE) included.
    pub const ALL: Self = Self((1 << 20) - 1);
}

bit_set!(PartStated, u32);

/// The style of a part's text.
///
/// A property whose bit in [`stated`](Self::stated) is clear is not stated by
/// the theme, and one whose bit in [`from_state`](Self::from_state) is set
/// got its value from a rule that requires a state, as for [`PartStyle`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct TextStyle {
    /// The font family.
    pub family: Arc<str>,
    /// The text colour.
    pub color: Color,
    /// The font size, in logical pixels.
    pub size_px: f32,
    /// The size in its source's unit, before `font_dpi`, whole-pixel
    /// rounding and `text_scale` (0.7.2's `defined_size`); `None` where the
    /// source gives none.
    ///
    /// It is what a display of the size shows and what an application
    /// re-derives the size from at its own DPI: `size_px` cannot give it back
    /// after rounding, nor tell a pixel size from a point size. It belongs to
    /// `size_px`: it has no [`TextStated`] bit and no [`TextProp`] of its own,
    /// and [`TextProp::SizePx`] clears it.
    ///
    /// [`TextProp`]: crate::next::TextProp
    /// [`TextProp::SizePx`]: crate::next::TextProp::SizePx
    pub defined_size: Option<FontSize>,
    /// The weight.
    pub weight: u16, // CSS 100..=900
    /// The slant.
    pub style: FontStyle, // Normal | Italic | Oblique
    /// The width.
    pub width: FontWidth, // OS/2 1..=9 (Normal = 5)
    /// The line height.
    pub line_height: f32, // logical px
    /// The letter spacing.
    pub letter_spacing: f32, // logical px
    /// The optical size axis value.
    pub optical_size: Option<f32>,
    /// The decoration line.
    pub decoration: TextDecoration,
    /// How the text blends with what lies below.
    pub blend_mode: BlendMode,
    /// Which properties the theme states.
    pub stated: TextStated,
    /// Which properties got their value from a rule that requires a state
    /// flag of the query.
    pub from_state: TextStated,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            family: Arc::from(""),
            color: Color::BLACK,
            size_px: 0.0,
            defined_size: None,
            weight: 400,
            style: FontStyle::Normal,
            width: FontWidth::NORMAL,
            line_height: 0.0,
            letter_spacing: 0.0,
            optical_size: None,
            decoration: TextDecoration::default(),
            blend_mode: BlendMode::Normal,
            stated: TextStated::default(),
            from_state: TextStated::default(),
        }
    }
}

/// One bit per `TextStyle` property the theme states.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextStated(u16);

impl TextStated {
    /// `family` is stated.
    pub const FAMILY: Self = Self(1 << 0);
    /// `color` is stated.
    pub const COLOR: Self = Self(1 << 1);
    /// `size_px` is stated.
    pub const SIZE_PX: Self = Self(1 << 2);
    /// `weight` is stated.
    pub const WEIGHT: Self = Self(1 << 3);
    /// `style` is stated.
    pub const STYLE: Self = Self(1 << 4);
    /// `width` is stated.
    pub const WIDTH: Self = Self(1 << 5);
    /// `line_height` is stated.
    pub const LINE_HEIGHT: Self = Self(1 << 6);
    /// `letter_spacing` is stated.
    pub const LETTER_SPACING: Self = Self(1 << 7);
    /// `optical_size` is stated.
    pub const OPTICAL_SIZE: Self = Self(1 << 8);
    /// `decoration` is stated.
    pub const DECORATION: Self = Self(1 << 9);
    /// `blend_mode` is stated.
    pub const BLEND_MODE: Self = Self(1 << 10);
    /// Every property.
    pub const ALL: Self = Self((1 << 11) - 1);
}

bit_set!(TextStated, u16);

/// A line drawn with text.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextDecoration {
    /// Which line.
    pub kind: DecorationKind,
    /// Its colour.
    pub color: Option<Color>, // None = inherit text color
    /// Its thickness, in logical pixels.
    pub thickness: Option<f32>, // None = font metric default
}

/// The line of a [`TextDecoration`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DecorationKind {
    /// No line.
    #[default]
    None,
    /// A line under the text.
    Underline,
    /// A line through the text.
    LineThrough,
    /// A wavy line under the text.
    WavyUnderline,
}

/// The slant of a font.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FontStyle {
    /// Upright.
    #[default]
    Normal,
    /// Italic.
    Italic,
    /// Oblique.
    Oblique,
}

/// A font size in the unit its source states (0.7.2's `FontSize`): a
/// preset's point or pixel constant, a `QFont` string's point or pixel size.
#[derive(Clone, Copy, Debug, PartialEq)]
// no `Default`: the type stands only in `Option` fields, where a default would be an invented value
pub enum FontSize {
    /// Typographic points (1/72 inch).
    Pt(f32),
    /// Logical pixels.
    Px(f32),
}

/// Points per inch: a point is 1/72 inch (0.7.2's `FontSize::to_logical_px`).
const POINTS_PER_INCH: f32 = 72.0;

impl FontSize {
    /// The size in logical pixels at `font_dpi`: `Pt(v)` is
    /// `v * font_dpi / 72`, `Px(v)` is `v` whatever the DPI.
    #[must_use]
    pub fn to_logical_px(self, font_dpi: f32) -> f32 {
        match self {
            Self::Pt(v) => v * font_dpi / POINTS_PER_INCH,
            Self::Px(v) => v,
        }
    }
}

/// The width class of a font, OpenType OS/2 `usWidthClass` `1..=9`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FontWidth(u16); // clamped to 1..=9; manual Default => Self::NORMAL (5)

/// The narrowest width class (OpenType OS/2 `usWidthClass`, 1 = ultra-condensed).
const FONT_WIDTH_MIN: u16 = 1;
/// The widest width class (OpenType OS/2 `usWidthClass`, 9 = ultra-expanded).
const FONT_WIDTH_MAX: u16 = 9;

impl FontWidth {
    /// The normal width class, 5 (OpenType OS/2 `usWidthClass`, medium).
    pub const NORMAL: Self = Self(5);

    /// The width class `width`, clamped to `1..=9`.
    #[must_use]
    pub const fn new(width: u16) -> Self {
        if width < FONT_WIDTH_MIN {
            Self(FONT_WIDTH_MIN)
        } else if width > FONT_WIDTH_MAX {
            Self(FONT_WIDTH_MAX)
        } else {
            Self(width)
        }
    }

    /// The width class, `1..=9`.
    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl Default for FontWidth {
    fn default() -> Self {
        Self::NORMAL
    }
}

/// The style of a part's icon.
///
/// A property whose bit in [`stated`](Self::stated) is clear is not stated by
/// the theme, and one whose bit in [`from_state`](Self::from_state) is set
/// got its value from a rule that requires a state, as for [`PartStyle`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct IconStyle {
    /// The size, in logical pixels.
    pub size: f32,
    /// The colour.
    pub color: Color,
    /// The opacity, `0.0..=1.0`.
    pub opacity: f32,
    /// The symbol weight.
    pub weight: u16, // SF Symbols / Material Symbols weight (400 default)
    /// The rendering mode.
    pub rendering_mode: IconRenderingMode,
    /// The stroke width of a vector glyph.
    pub stroke_width: Option<f32>, // vector glyph stroke (e.g., checkmark)
    /// How the icon blends with what lies below.
    pub blend_mode: BlendMode,
    /// Which properties the theme states.
    pub stated: IconStated, // e.g. `SIZE` clear where no source states the glyph's size (issue #4 `Z3`)
    /// Which properties got their value from a rule that requires a state
    /// flag of the query.
    pub from_state: IconStated,
}

impl Default for IconStyle {
    fn default() -> Self {
        Self {
            size: 0.0,
            color: Color::default(),
            opacity: 1.0,
            weight: 400,
            rendering_mode: IconRenderingMode::Monochrome,
            stroke_width: None,
            blend_mode: BlendMode::Normal,
            stated: IconStated::default(),
            from_state: IconStated::default(),
        }
    }
}

/// One bit per `IconStyle` property the theme states.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct IconStated(u8);

impl IconStated {
    /// `size` is stated.
    pub const SIZE: Self = Self(1 << 0);
    /// `color` is stated.
    pub const COLOR: Self = Self(1 << 1);
    /// `opacity` is stated.
    pub const OPACITY: Self = Self(1 << 2);
    /// `weight` is stated.
    pub const WEIGHT: Self = Self(1 << 3);
    /// `rendering_mode` is stated.
    pub const RENDERING_MODE: Self = Self(1 << 4);
    /// `stroke_width` is stated.
    pub const STROKE_WIDTH: Self = Self(1 << 5);
    /// `blend_mode` is stated.
    pub const BLEND_MODE: Self = Self(1 << 6);
    /// Every property.
    pub const ALL: Self = Self((1 << 7) - 1);
}

bit_set!(IconStated, u8);

/// How a symbol icon's layers are coloured.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconRenderingMode {
    /// One colour.
    #[default]
    Monochrome,
    /// One colour at layered opacities.
    Hierarchical,
    /// A colour per layer.
    Palette,
    /// The symbol's own colours.
    Multicolor,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_stated_bit_operators_and_not_masking() {
        let s = PartStated::FILL | PartStated::PADDING_LEFT;
        assert!(s.contains(PartStated::FILL));
        assert!(!s.contains(PartStated::FILL | PartStated::STROKE));
        assert!(s.intersects(PartStated::FILL | PartStated::STROKE));
        assert_eq!(s & PartStated::FILL, PartStated::FILL);
        assert_eq!(!s & s, PartStated::default());
        assert_eq!((!s).union(s), PartStated::ALL);
        assert_eq!((!PartStated::default()).bits(), PartStated::ALL.bits());
        assert_eq!((!PartStated::default()).bits() >> 20, 0);
        assert_eq!(PartStated::from_bits_truncate(u32::MAX), PartStated::ALL);
        let mut t = PartStated::default();
        t |= PartStated::CURSOR;
        t &= PartStated::CURSOR | PartStated::FOCUS;
        assert_eq!(t, PartStated::CURSOR);
        assert_eq!(s.difference(PartStated::FILL), PartStated::PADDING_LEFT);
        assert!(PartStated::default().is_empty());
        let all = [
            PartStated::FILL,
            PartStated::STROKE,
            PartStated::CORNERS,
            PartStated::SHADOWS,
            PartStated::OPACITY,
            PartStated::TRANSFORM,
            PartStated::PADDING_TOP,
            PartStated::PADDING_RIGHT,
            PartStated::PADDING_BOTTOM,
            PartStated::PADDING_LEFT,
            PartStated::MARGIN_TOP,
            PartStated::MARGIN_RIGHT,
            PartStated::MARGIN_BOTTOM,
            PartStated::MARGIN_LEFT,
            PartStated::FOCUS,
            PartStated::RIPPLE,
            PartStated::TRANSITION,
            PartStated::CURSOR,
            PartStated::MIN_SIZE,
            PartStated::MAX_SIZE,
        ]
        .into_iter()
        .fold(PartStated::default(), |acc, b| acc | b);
        assert_eq!(all, PartStated::ALL);
    }

    #[test]
    fn text_and_icon_stated_not_masking() {
        assert_eq!(!TextStated::default(), TextStated::ALL);
        assert_eq!((!TextStated::BLEND_MODE).bits() >> 11, 0);
        assert_eq!(!IconStated::default(), IconStated::ALL);
        assert_eq!((!IconStated::SIZE).bits() >> 7, 0);
    }

    #[test]
    fn manual_defaults() {
        let part = PartStyle::default();
        assert_eq!(part.opacity, 1.0);
        assert_eq!(part.transform, Transform::IDENTITY);
        assert!(part.stated.is_empty());
        assert!(part.from_state.is_empty());
        let text = TextStyle::default();
        assert_eq!(text.weight, 400);
        assert_eq!(text.width, FontWidth::NORMAL);
        assert_eq!(text.color, Color::BLACK);
        assert_eq!(text.defined_size, None);
        let icon = IconStyle::default();
        assert_eq!(icon.opacity, 1.0);
        assert_eq!(icon.weight, 400);
        assert_eq!(FontWidth::default().value(), 5);
    }

    #[test]
    fn font_size_to_logical_px() {
        // 10 pt at 96 DPI: 10 × 96 / 72 = 13.333…; at 72 DPI the point is the pixel.
        assert!((FontSize::Pt(10.0).to_logical_px(96.0) - 40.0 / 3.0).abs() < 1e-5);
        assert_eq!(FontSize::Pt(10.0).to_logical_px(72.0), 10.0);
        assert_eq!(FontSize::Px(14.0).to_logical_px(96.0), 14.0);
        assert_eq!(FontSize::Px(14.0).to_logical_px(144.0), 14.0);
    }

    #[test]
    fn font_width_clamps() {
        assert_eq!(FontWidth::new(0).value(), 1);
        assert_eq!(FontWidth::new(1).value(), 1);
        assert_eq!(FontWidth::new(7).value(), 7);
        assert_eq!(FontWidth::new(9).value(), 9);
        assert_eq!(FontWidth::new(400).value(), 9);
    }
}
