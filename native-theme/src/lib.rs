//! # native-theme
//!
//! Cross-platform native theme detection and loading for Rust GUI applications.
//!
//! The crate is between two models: today's theme model, its presets and its
//! platform readers are removed, and the 0.8.0 model that replaces them is
//! not yet in place. What stays is the detection, the icons and the fonts.
//!
//! # Feature flags
//!
//! Each OS-specific dependency is compiled only on its own target, so
//! `native` on macOS pulls in only the macOS dependencies.
//!
//! | Feature | Enables |
//! |---|---|
//! | `kde` | The KDE icon theme, read from the `kdeglobals` files as KConfig cascades them (Linux) |
//! | `macos`, `windows` | The running OS version and its generation, in [`pipeline::diagnose_platform_support`] |
//! | `portal`, `watch` | Nothing until the platform readers return |
//! | `linux-kde`, `linux-portal` | Aliases of `kde` and `portal` |
//! | `linux` | `linux-kde` and `linux-portal` |
//! | `native` | `linux`, `macos` and `windows` |
//! | `material-icons`, `lucide-icons` | The bundled Material Symbols and Lucide icon sets |
//! | `system-icons` | The platform icon sets: freedesktop icon themes (Linux), SF Symbols (macOS), Segoe Fluent Icons and stock icons (Windows); also enables `material-icons` |
//! | `svg-rasterize` | `rasterize::rasterize_svg`, SVG icons to RGBA through resvg |
//! | `system-fonts` | `fonts::system_face`, the platform's own typeface for a family; on macOS the system UI font's file through Core Text |
//!
//! The minimum supported Rust version is 1.88.0.
//!
//! # Modules
//!
//! - The crate root: [`Error`] and [`Result`], and from [`pipeline`]
//!   [`VersionMatch`] and [`DiagnosticEntry`].
//! - [`detect`]: OS detection: the Linux desktop environment, and the
//!   detection caches.
//! - [`icons`]: the icon roles and sets, the per-set icon loaders, the icon
//!   data types and the icon-set choice.
//! - [`fonts`]: font face matching; the platform's own typeface with the
//!   `system-fonts` feature.
//! - [`pipeline`]: a diagnosis of the platform's theme support.
//! - `rasterize` (the `svg-rasterize` feature): SVG icons to RGBA.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

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

pub mod detect;
pub mod fonts;
/// Icon loading.
///
/// - The semantic roles, [`IconRole`], the icon sets, [`IconSet`], and the
///   icon data, [`IconData`](icons::IconData).
/// - A typed loader per icon set: [`MaterialLoader`](icons::MaterialLoader)
///   and [`LucideLoader`](icons::LucideLoader) for the bundled sets,
///   [`FreedesktopLoader`](icons::FreedesktopLoader) (Linux),
///   [`SfSymbolsLoader`](icons::SfSymbolsLoader) (macOS) and
///   [`SegoeFluentLoader`](icons::SegoeFluentLoader) (Windows) for the
///   platform sets. Each needs its set's feature, `material-icons`,
///   `lucide-icons` or `system-icons`, and gives `None` without it.
/// - [`load_icon`](icons::load_icon), which loads from whichever set it is
///   given.
/// - The icon-set choice of a settings UI,
///   [`IconSetChoice`](icons::IconSetChoice).
pub mod icons;
/// The KDE icon theme, read from `kdeglobals`.
#[cfg(all(target_os = "linux", feature = "kde"))]
pub(crate) mod kde;
pub mod pipeline;

/// Freedesktop icon theme lookup (Linux).
///
/// Requires the `system-icons` feature, and Linux.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub(crate) mod freedesktop;
/// macOS platform helpers.
#[cfg(target_os = "macos")]
pub(crate) mod macos;
/// SVG-to-RGBA rasterization utilities.
///
/// Requires the `svg-rasterize` feature.
#[cfg(feature = "svg-rasterize")]
pub mod rasterize;
/// SF Symbols icon loader (macOS).
///
/// Requires the `system-icons` feature, and macOS.
#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub(crate) mod sficons;
/// The running Windows version.
///
/// Requires the `windows` feature, and Windows.
#[cfg(all(target_os = "windows", feature = "windows"))]
pub(crate) mod windows;
/// Windows Segoe Fluent / stock icon loader.
///
/// Requires the `system-icons` feature, and Windows.
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub(crate) mod winicons;
#[cfg(all(not(target_os = "windows"), feature = "system-icons"))]
#[allow(dead_code, unused_imports)]
pub(crate) mod winicons;

/// Convenience Result type alias for this crate.
pub type Result<T> = std::result::Result<T, error::Error>;

// Internal re-exports: keep crate::Type paths working for internal modules
// without exposing them in the public API. External users access the icon
// types via native_theme::icons::*.
pub(crate) use icons::{IconRole, IconSet};
pub use pipeline::{DiagnosticEntry, VersionMatch};

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
    IndicationBox, IndicationBoxPatch, Interaction, InteractionPatch, ResolvedIndicationBox,
    ResolvedInteraction, ResolvedRippleStyle, ResolvedStateLayerStyle, RippleEffect, RippleStyle,
    RippleStylePatch, StateLayerStyle, StateLayerStylePatch,
};
pub use blocks::motion::{
    AdwEasing, AnimatedVisual, AnimatedVisualPatch, BreezeTest, BreezeTestPatch, EdgeSide,
    Keyframe, KeyframePatch, Loop, LoopPatch, PlatformCurve, PresentationEvent, PresentationKind,
    PresentationStep, PresentationStepPatch, PresentationTransition, PresentationTransitionPatch,
    PropertyGroup, QtEasing, ResolvedAnimatedVisual, ResolvedEngine, ResolvedKeyframe,
    ResolvedLoop, ResolvedPlatformCurve, ResolvedPresentationKind, ResolvedPresentationStep,
    ResolvedPresentationTransition, ResolvedTiming, ResolvedTransition, ResolvedTransitionKind,
    RevealOrigin, StepJump, Timing, Transition, TransitionEngine, TransitionKind, TransitionPatch,
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
pub use error::{Error, KeyError, LayerError, Problem, ProblemKind, ThemeError};
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
    Antialiasing, Behavior, BehaviorPatch, BlinkStyle, Caret, CaretPatch, ColorInterpolation,
    FontFallbacks, FontFallbacksPatch, Hinting, IconSizeTokens, IconSizeTokensPatch, IconTokens,
    IconTokensPatch, KdeColorEffect, KdeContrastEffect, KdeEffects, KdeEffectsPatch, KdeGroup,
    KdeIntensityEffect, KdeRoleKind, KdeTokens, KdeTokensPatch, KeyRepeat, KeyRepeatPatch,
    MaterialScheme, MaterialSchemePatch, MaterialVariant, Mnemonics, MotionTokens,
    MotionTokensPatch, PointerTokens, PointerTokensPatch, ResolvedBehavior, ResolvedCaret,
    ResolvedFontFallbacks, ResolvedIconSizeTokens, ResolvedIconTokens, ResolvedKdeEffects,
    ResolvedKdeTokens, ResolvedKeyRepeat, ResolvedMaterialScheme, ResolvedMnemonics,
    ResolvedMotionTokens, ResolvedPointerTokens, ResolvedSizeClassThreshold, ResolvedTextRendering,
    ResolvedTextRoles, ResolvedTextTokens, ResolvedThemeChange, ResolvedTokens,
    ResolvedTouchBehavior, SizeClassThreshold, SizeClassThresholdPatch, SubpixelOrder,
    TextRendering, TextRenderingPatch, TextRoles, TextRolesPatch, TextScaling, TextTokens,
    TextTokensPatch, ThemeChange, Tokens, TokensPatch, TouchBehavior, TouchBehaviorPatch,
};
pub use units::{
    Axes, Deg, FontSize, Fraction, Length, Ms, Point, Px, Rect, Scalar, Sides, Size, ViewBox,
};
pub use value::{SchemeMap, SurfaceMap, Value};
pub use widgets::*;
