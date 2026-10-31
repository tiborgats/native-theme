//! The 0.8.0 model, built beside today's until it replaces it.
//!
//! Its re-exports are the shape the crate root takes when this module moves
//! into place: the units, keys, colours and values every block is made of,
//! and the expressions a value may derive by.

mod blocks;
pub mod color;
mod declare;
mod error;
pub mod expr;
#[doc(hidden)]
pub mod gate;
mod inputs;
mod key;
mod layer;
pub mod placement;
#[doc(hidden)]
pub mod platform;
mod presets;
mod query;
mod registry;
mod resolve;
pub mod ripple;
mod roles;
#[cfg(test)]
mod roles_tokens_tests;
mod selector;
mod theme;
mod tokens;
mod toml;
mod units;
mod validate;
mod value;
mod widgets;

pub mod corners {
    //! The pure functions a connector computes a `Full`, `Ellipse`,
    //! `Concentric` or `FollowPart` corner with at layout, from its own
    //! layout's numbers (S2.4).

    pub use super::blocks::outline::{concentric, ellipse, follow_part, full};
}

pub mod shapes {
    //! Shapes replacing a part's corners' outline, as code (S2.4).

    pub use super::blocks::outline::{
        MaterialShape, Outline, OutlineCorners, Path, PathCommand, ShapeKind, outline,
    };
}

pub mod transitions {
    //! Which of a style's transitions governs a change (S2.7).

    pub use super::blocks::motion::governing;
}

pub mod timing {
    //! A timing's value at an elapsed fraction, the platforms' own curves
    //! among them, and a spring's damping coefficient from its damping ratio
    //! (S2.7).

    pub use super::blocks::motion::{damping_coefficient, evaluate, spring_duration};
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
pub use blocks::interaction::{
    Interaction, InteractionPatch, ResolvedInteraction, ResolvedRippleStyle,
    ResolvedStateLayerStyle, RippleEffect, RippleStyle, RippleStylePatch, StateLayerStyle,
    StateLayerStylePatch,
};
pub use blocks::motion::{
    AdwEasing, EdgeSide, Keyframe, KeyframePatch, Loop, LoopPatch, PlatformCurve,
    PresentationEvent, PresentationKind, PresentationStep, PresentationStepPatch,
    PresentationTransition, PresentationTransitionPatch, PropertyGroup, QtEasing, ResolvedKeyframe,
    ResolvedLoop, ResolvedPlatformCurve, ResolvedPresentationKind, ResolvedPresentationStep,
    ResolvedPresentationTransition, ResolvedTiming, ResolvedTransition, ResolvedTransitionKind,
    RevealOrigin, StepJump, Timing, Transition, TransitionKind, TransitionPatch,
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
    Binding, CursorShape, PartStyle, PartStylePatch, ResolvedBinding, ResolvedPartStyle,
    ResolvedPartStylePatch, RoleRef,
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
pub use declare::{DataName, FontMetric};
pub use error::{KeyError, LayerError, Problem, ProblemKind, ThemeError};
pub use expr::{Expr, ParseError};
pub use inputs::{Accent, Contrast, InputBases, InputName, InputValue, KdeSchemeKey, KdeWmKey};
pub use key::Key;
pub use layer::{Bases, Layer, LayerId};
pub use presets::PresetId;
pub use query::{
    DataNeeds, Element, ElementData, FontMetrics, RootInputs, Scope, Sibling, Styled, Unavailable,
    Widget,
};
pub use resolve::{
    Change, ContentSizeCategory, ReaderState, Resolution, ResolvedTheme, WindowConditions,
    WindowState,
};
pub use roles::{
    ColorRoles, ColorRolesPatch, DisabledRoles, DisabledRolesPatch, GlobalRole, GlobalRoles,
    GlobalRolesPatch, ResolvedColorRoles, ResolvedDisabledRoles, ResolvedGlobalRoles,
    ResolvedStatusColors, ResolvedStatusRoles, ResolvedSurfaceRoles, StatusColors,
    StatusColorsPatch, StatusRoles, StatusRolesPatch, SurfaceRole, SurfaceRoles, SurfaceRolesPatch,
    SurfaceSet,
};
pub use selector::{
    Breakpoint, Combine, Comparison, Condition, ContextMatch, ContextScope, ControlSize, Density,
    Direction, Idiom, InputKind, Measured, Parity, PartRef, PositionMatch, Posture, Rule, RuleBody,
    Scheme, Selector, SetMatch, SharedFlag, SiblingMatch, SiblingScope, SizeClass, SizeClassMatch,
    SizeClasses, SizeMatch, StateFlag, StateFlags, TiledEdges, ValuesPatch, VariantRef,
    WidgetOrPart, WidgetRef,
};
pub use theme::Theme;
pub use tokens::{
    Antialiasing, Behavior, BehaviorPatch, BlinkStyle, Caret, CaretPatch, FontFallbacks,
    FontFallbacksPatch, Hinting, IconSizeTokens, IconSizeTokensPatch, IconTokens, IconTokensPatch,
    KdeColorEffect, KdeContrastEffect, KdeEffects, KdeEffectsPatch, KdeGroup, KdeIntensityEffect,
    KdeRoleKind, KdeTokens, KdeTokensPatch, KeyRepeat, KeyRepeatPatch, MaterialScheme,
    MaterialSchemePatch, MaterialVariant, Mnemonics, MotionTokens, MotionTokensPatch,
    PointerTokens, PointerTokensPatch, ResolvedBehavior, ResolvedCaret, ResolvedFontFallbacks,
    ResolvedIconSizeTokens, ResolvedIconTokens, ResolvedKdeEffects, ResolvedKdeTokens,
    ResolvedKeyRepeat, ResolvedMaterialScheme, ResolvedMnemonics, ResolvedMotionTokens,
    ResolvedPointerTokens, ResolvedSizeClassThreshold, ResolvedTextRendering, ResolvedTextRoles,
    ResolvedTextTokens, ResolvedThemeChange, ResolvedTokens, ResolvedTouchBehavior,
    SizeClassThreshold, SizeClassThresholdPatch, SubpixelOrder, TextRendering, TextRenderingPatch,
    TextRoles, TextRolesPatch, TextScaling, TextTokens, TextTokensPatch, ThemeChange, Tokens,
    TokensPatch, TouchBehavior, TouchBehaviorPatch,
};
pub use units::{
    Axes, Deg, FontSize, Fraction, Length, Ms, Point, Px, Rect, Scalar, Sides, Size, ViewBox,
};
pub use value::{SchemeMap, SurfaceMap, Value};
pub use widgets::*;
