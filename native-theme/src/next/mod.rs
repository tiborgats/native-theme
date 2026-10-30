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
mod widgets;

pub mod corners {
    //! The pure functions a connector computes a `Full`, `Concentric` or
    //! `FollowPart` corner with at layout, from its own layout's numbers
    //! (S2.4).

    pub use super::blocks::outline::{concentric, follow_part, full};
}

pub mod shapes {
    //! Shapes replacing a part's corners' outline, as code (S2.4).

    pub use super::blocks::outline::{
        MaterialShape, Outline, OutlineCorners, Path, PathCommand, ShapeKind, outline,
    };
}

pub mod timing {
    //! Springs stated by their damping ratio (S2.7).

    pub use super::blocks::motion::{damping_coefficient, damping_ratio};
}

pub use blocks::focus::{
    FocusIndicator, FocusIndicatorPatch, FocusKind, FocusVisibleWhen, ResolvedFocusIndicator,
    ResolvedFocusVisibleWhen,
};
pub use blocks::icon::{
    Glyph, GlyphMirroring, GlyphPatch, GlyphSource, GlyphStroke, GlyphStrokePatch, IconStyle,
    IconStylePatch, Mirroring, RenderingMode, ResolvedGlyph, ResolvedGlyphStroke,
    ResolvedIconStyle, SystemControl,
};
pub use blocks::motion::{
    EdgeSide, Keyframe, KeyframePatch, Loop, LoopPatch, PlatformCurve, PresentationEvent,
    PresentationKind, PresentationStep, PresentationStepPatch, PresentationTransition,
    PresentationTransitionPatch, PropertyGroup, ResolvedKeyframe, ResolvedLoop,
    ResolvedPresentationKind, ResolvedPresentationStep, ResolvedPresentationTransition,
    ResolvedTiming, ResolvedTransition, ResolvedTransitionKind, RevealOrigin, StepJump, Timing,
    Transition, TransitionKind, TransitionPatch,
};
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
pub use blocks::part_style::{
    Binding, CursorShape, PartStyle, PartStylePatch, ResolvedBinding, ResolvedPartStyle, RoleRef,
};
pub use blocks::shadow::{
    ResolvedShadow, ResolvedShadowLayer, Shadow, ShadowLayer, ShadowLayerPatch,
};
pub use blocks::text::{
    AxisTag, AxisValue, Decoration, DecorationLine, DecorationPatch, DecorationStyle, Elision,
    FadeLength, FadeUnder, FeatureTag, Font, FontPatch, FontStyle, LanguageTag, ResolvedDecoration,
    ResolvedElision, ResolvedFadeLength, ResolvedFont, ResolvedTextStyle, TextCase, TextRole,
    TextStyle, TextStylePatch, Wrap,
};
pub use blocks::transform::{ResolvedTransform, Transform, TransformPatch};
pub use blocks::{Block, BlockPatch, FieldsBlock};
pub use color::Color;
pub use error::KeyError;
pub use expr::{Expr, ParseError};
pub use key::Key;
pub use roles::{GlobalRole, SurfaceRole, SurfaceSet};
pub use selector::{PartRef, Scheme, VariantRef, WidgetRef};
pub use units::{
    Axes, Deg, FontSize, Fraction, Length, Ms, Point, Px, Rect, Scalar, Sides, Size, ViewBox,
};
pub use value::{SchemeMap, SurfaceMap, Value};
pub use widgets::{TypedPart, Variant, WidgetKind};
