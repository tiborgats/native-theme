//! Paint (S2.3): colours, gradients, patterns and materials in layers.

use native_theme_derive::ThemeParts;

use crate::next::color::Color;
use crate::next::error::{LayerError, Problem, ProblemKind};
use crate::next::key::Key;
use crate::next::units::{Deg, Fraction, Px, Scalar};
use crate::next::value::Value;

/// Layers drawn bottom first. A paint has at least one layer (S12); a paint
/// that paints nothing is `Value::Absent`.
///
/// A rule's patch sets a paint or appends layers over the one below
/// ([`PaintPatch`]). A paint is drawn in the box's own coordinates and is
/// not mirrored in right-to-left.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, append)]
#[non_exhaustive]
pub struct Paint {
    /// The layers, bottom first.
    #[parts(block = "fields")]
    pub layers: Vec<PaintLayer>,
}

/// A paint of `layers`, bottom first (S1): refused with no layer, or with
/// a `Mask` at the bottom, which masks nothing (S2.3, S12).
impl TryFrom<Vec<PaintLayer>> for Paint {
    type Error = LayerError;

    fn try_from(layers: Vec<PaintLayer>) -> Result<Paint, LayerError> {
        let problem = match layers.first() {
            None => Some(Problem::new("layers", ProblemKind::NoLayer)),
            Some(bottom) if bottom.blend == BlendMode::Mask => {
                Some(Problem::new("layers[0].blend", ProblemKind::MaskAtBottom))
            }
            Some(_) => None,
        };
        match problem {
            Some(problem) => Err(LayerError::Invalid(vec![problem])),
            None => Ok(Paint { layers }),
        }
    }
}

/// One layer of a paint.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, own_toml)]
#[non_exhaustive]
pub struct PaintLayer {
    /// What the layer draws.
    #[parts(block = "set")]
    pub kind: PaintKind,
    /// How the layer combines with the layers beneath it.
    pub blend: BlendMode,
}

/// What a paint layer draws.
///
/// Angles follow CSS: [`PaintKind::Linear`]'s `angle` is
/// `linear-gradient()`'s, `0deg` pointing toward the top of the box and
/// increasing clockwise; [`PaintKind::Conic`]'s `start_angle` is
/// `conic-gradient()`'s `from` angle, its stops running clockwise from it. A
/// gradient has at least two stops (S12).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum PaintKind {
    /// One colour.
    Color(Value<Color>),
    /// A linear gradient.
    Linear {
        /// The gradient's angle.
        angle: Deg,
        /// The colour stops.
        #[parts(block = "fields")]
        stops: Vec<Stop>,
        /// The stops repeat beyond the last.
        repeat: bool,
        /// The stops run over this many px from the start edge; `None`: over
        /// the box.
        extent: Option<Value<Px>>,
        /// The stops shifted along the axis by this many px, toward its end
        /// for a positive offset; `None`: not shifted.
        #[parts(signed)]
        offset: Option<Value<Px>>,
    },
    /// A radial gradient.
    Radial {
        /// The centre, in fractions of the box.
        center: (f32, f32),
        /// The radius.
        radius: Value<Px>,
        /// The colour stops.
        #[parts(block = "fields")]
        stops: Vec<Stop>,
        /// The stops repeat beyond the last.
        repeat: bool,
    },
    /// A conic gradient.
    Conic {
        /// The centre, in fractions of the box.
        center: (f32, f32),
        /// The angle the stops start from.
        start_angle: Deg,
        /// The colour stops.
        #[parts(block = "fields")]
        stops: Vec<Stop>,
    },
    /// A pattern, its lines or cells in the foreground.
    Pattern {
        /// The pattern.
        pattern: Pattern,
        /// The pattern's colour; a grid's lines.
        foreground: Value<Color>,
        /// The colour between.
        background: Value<Color>,
        /// The pattern's cell.
        cell: Value<Px>,
    },
    /// A platform material.
    Material(#[parts(block = "fields")] Material),
}

/// A pattern, each drawn as its platform's source draws it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Pattern {
    /// A checkerboard.
    Checkerboard,
    /// A grid of lines.
    Grid,
    /// One of Qt's brush patterns.
    QtBrush(QtBrushStyle),
}

/// Qt's `Qt::BrushStyle` patterns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum QtBrushStyle {
    /// `Qt::Dense1Pattern`.
    Dense1,
    /// `Qt::Dense2Pattern`.
    Dense2,
    /// `Qt::Dense3Pattern`.
    Dense3,
    /// `Qt::Dense4Pattern`.
    Dense4,
    /// `Qt::Dense5Pattern`.
    Dense5,
    /// `Qt::Dense6Pattern`.
    Dense6,
    /// `Qt::Dense7Pattern`.
    Dense7,
    /// `Qt::HorPattern`.
    Hor,
    /// `Qt::VerPattern`.
    Ver,
    /// `Qt::CrossPattern`.
    Cross,
    /// `Qt::BDiagPattern`.
    BDiag,
    /// `Qt::FDiagPattern`.
    FDiag,
    /// `Qt::DiagCrossPattern`.
    DiagCross,
}

/// A gradient's colour stop.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Stop {
    /// Where the stop lies along the gradient.
    pub position: f32,
    /// The colour there.
    pub color: Value<Color>,
}

/// How a layer combines with the layers beneath it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum BlendMode {
    /// Drawn over the layers beneath.
    Normal,
    /// Not drawn: its alpha, at each point, multiplies the alpha of the
    /// layers beneath it in the same paint (Porter-Duff destination-in). A
    /// paint whose bottom layer is a mask is refused (S12).
    Mask,
    /// A platform's blending mode, as the facts name it (macOS vibrancy).
    Keyed(Key),
}

/// A platform material: a blurred, tinted backdrop.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Material {
    /// The platform's material, declared by the preset (S10).
    pub identity: Key,
    /// The backdrop's blur radius.
    #[parts(may_be_absent)]
    pub blur_radius: Value<Px>,
    /// The tint.
    #[parts(may_be_absent)]
    pub tint: Value<Color>,
    /// A saturation multiplier, at least zero (CSS `saturate()`).
    #[parts(may_be_absent)]
    pub saturation: Value<f32>,
    /// The luminosity.
    #[parts(may_be_absent)]
    pub luminosity: Value<Fraction>,
    /// The noise's opacity.
    #[parts(may_be_absent)]
    pub noise_opacity: Value<Fraction>,
    /// What a toolkit that cannot draw the material draws.
    pub fallback: Value<Color>,
    /// What a platform's material states beyond these.
    pub params: Vec<(Key, Value<Scalar>)>,
}
