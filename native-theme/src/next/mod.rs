//! The 0.8.0 model, built beside today's until it replaces it.
//!
//! Its re-exports are the shape the crate root takes when this module moves
//! into place: the units, keys, colours and values every block is made of,
//! and the expressions a value may derive by.

mod blocks;
pub mod color;
mod error;
pub mod expr;
mod key;
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "read by S11's reader and the facts gate (M1a tasks 7 and 14)"
    )
)]
mod registry;
mod roles;
mod selector;
mod units;
mod value;

pub use blocks::outline::{
    CornerCurve, Corners, CornersPatch, DashPattern, Edge, EdgePatch, FillClip, LineCap, LineJoin,
    PixelSnap, Radius, ResolvedCorners, ResolvedEdge, ResolvedRadius, ResolvedStroke, Stroke,
    StrokePatch, StrokePosition,
};
pub use blocks::paint::{
    BlendMode, Material, MaterialPatch, Paint, PaintKind, PaintLayer, PaintLayerPatch, PaintPatch,
    Pattern, QtBrushStyle, ResolvedMaterial, ResolvedPaint, ResolvedPaintKind, ResolvedPaintLayer,
    ResolvedStop, Stop, StopPatch,
};
pub use blocks::shadow::{
    ResolvedShadow, ResolvedShadowLayer, Shadow, ShadowLayer, ShadowLayerPatch,
};
pub use blocks::transform::{ResolvedTransform, Transform, TransformPatch};
pub use blocks::{Block, BlockPatch, FieldsBlock};
pub use color::Color;
pub use error::KeyError;
pub use expr::{Expr, ParseError};
pub use key::Key;
pub use roles::SurfaceSet;
pub use selector::Scheme;
pub use units::{
    Axes, Deg, FontSize, Fraction, Length, Ms, Point, Px, Rect, Scalar, Sides, Size, ViewBox,
};
pub use value::{SchemeMap, SurfaceMap, Value};
