//! Icon style and glyph (S2.9).
//!
//! Icon lookup takes the element's direction and language, from the scope
//! its part's query returned; mirroring per icon role is
//! `token.icons.mirroring`, stated per preset.

use native_theme_derive::ThemeParts;

use super::motion::{Loop, ResolvedLoop};
use super::outline::{LineCap, LineJoin};
use super::paint::{Paint, PaintPatch, ResolvedPaint};
use super::text::{AxisTag, AxisValue, LanguageTag};
use crate::IconRole;
use crate::next::key::Key;
use crate::next::selector::PartRef;
use crate::next::units::{Fraction, Px, ViewBox};
use crate::next::value::Value;

/// How a part draws its icon.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct IconStyle {
    /// The icon's size.
    pub size: Value<Px>,
    /// The paint the platform's recolouring gives the icon (a symbolic icon,
    /// a template image, a symbol in its rendering mode's colours); absent
    /// only where `rendering_mode` is `Original`, which recolours nothing
    /// (S12).
    #[parts(block = "append", may_be_absent)]
    pub fill: Value<Paint>,
    /// How the icon is recoloured.
    pub rendering_mode: Value<RenderingMode>,
    /// The weight, the only route for it: `axes` may not hold `wght` (S12).
    #[parts(may_be_absent)]
    pub weight: Value<u16>,
    /// SF Symbols' named scale, declared in `symbol_scales` (S10).
    #[parts(may_be_absent)]
    pub scale: Value<Key>,
    /// Variation axes (Material Symbols' fill, grade, optical size).
    pub axes: Vec<(AxisTag, AxisValue)>,
    /// KDE's icon effect for the state the rule selects.
    #[parts(may_be_absent)]
    pub state_effect: Value<Key>,
    /// SF Symbols' variants (`.fill`, `.circle`, …).
    pub symbol_variants: Vec<Key>,
    /// A symbolic icon's status colours (success, warning, error).
    #[parts(block = "append")]
    pub status_colors: Vec<(Key, Value<Paint>)>,
    /// A repeating icon animation.
    #[parts(block = "fields", may_be_absent)]
    pub animation: Value<Loop>,
}

/// How an icon is recoloured.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum RenderingMode {
    /// A template: its alpha in the fill.
    Template,
    /// One colour.
    Monochrome,
    /// One colour in layered opacities.
    Hierarchical,
    /// A palette of colours.
    Palette,
    /// The symbol's own colours.
    Multicolor,
    /// The icon as drawn: nothing recoloured.
    Original,
}

/// A glyph a part draws: a mark, an arrow, a spinner's arc.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Glyph {
    /// What the glyph is drawn from.
    pub source: GlyphSource,
    /// The glyph's size.
    pub size: Value<Px>,
    /// The glyph's fill; absent only on a stroked-only path (a spinner's
    /// arc, whose paint is its stroke's; S12).
    #[parts(block = "append", may_be_absent)]
    pub fill: Value<Paint>,
    /// The glyph's stroke; absent where the glyph is not stroked.
    #[parts(block = "fields", may_be_absent)]
    pub stroke: Value<GlyphStroke>,
    /// A symbol font's variation axes.
    pub axes: Vec<(AxisTag, AxisValue)>,
    /// The fraction of the path drawn (Breeze's check mark).
    #[parts(may_be_absent)]
    pub draw_extent: Value<Fraction>,
    /// Where along the path drawing starts (a spinner's arc).
    #[parts(may_be_absent)]
    pub draw_start: Value<Fraction>,
    /// How the glyph mirrors under right-to-left.
    pub mirroring: GlyphMirroring,
}

/// What a glyph is drawn from.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(toml)]
pub enum GlyphSource {
    /// A code point of a font. The family's fallbacks are the text tokens'
    /// `fallbacks.per_family` chain, followed by `fallbacks.families` (S4.4).
    FontCodePoint {
        /// The font's family.
        family: String,
        /// The code point.
        code_point: u32,
    },
    /// An icon of the icon theme.
    ThemeIcon {
        /// The icon's name.
        name: String,
        /// The names tried, in order, where the theme lacks it.
        fallbacks: Vec<String>,
    },
    /// An SVG path.
    Path {
        /// The path, in SVG path syntax, in the source's own coordinates.
        data: String,
        /// The rectangle of those coordinates mapped onto the glyph's box at
        /// its size, as SVG's `viewBox`.
        view_box: ViewBox,
    },
    /// An image the platform names for that purpose.
    NamedSystemImage {
        /// The image's name.
        name: String,
    },
    /// The system's own rendering of a control (S15).
    SystemControl {
        /// The control.
        control: SystemControl,
        /// The parts the rendering replaces (a checkbox's box and mark
        /// together).
        replaces: Vec<PartRef>,
    },
}

/// A system control whose rendering a glyph takes (S15).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum SystemControl {
    /// AppKit's checkbox: the box and mark.
    AppKitCheckbox,
    /// AppKit's radio button: the box and mark.
    AppKitRadio,
    /// AppKit's disclosure triangle.
    AppKitDisclosure,
    /// AppKit's push disclosure button: the bezel and chevron.
    AppKitPushDisclosure,
    /// AppKit's pop-up button: the whole button at its width.
    AppKitPopUp,
}

/// A stroked path's outline.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct GlyphStroke {
    /// The stroke's paint.
    #[parts(block = "append")]
    pub paint: Value<Paint>,
    /// The stroke's width.
    pub width: Value<Px>,
    /// The line's cap.
    pub cap: LineCap,
    /// How the line's segments join.
    pub join: LineJoin,
}

/// How an icon role mirrors under right-to-left.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Mirroring {
    /// Never mirrored.
    Never,
    /// Always mirrored.
    Always,
    /// Mirrored except in these languages.
    ExceptLanguages(Vec<LanguageTag>),
    /// The role takes another role's icon (KStandardAction's Back and
    /// Forward).
    SwapWith(IconRole),
}

/// How a glyph mirrors under right-to-left; a glyph names no icon role, so
/// it takes no other role's icon.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum GlyphMirroring {
    /// Never mirrored.
    Never,
    /// Always mirrored.
    Always,
    /// Mirrored except in these languages.
    ExceptLanguages(Vec<LanguageTag>),
}
