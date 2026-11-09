//! # native-theme
//!
//! A theme model for Rust GUI toolkits: a bundled platform preset with the
//! application's layers over it, resolved for each window into the style of
//! each part of each widget, which a connector hands its toolkit.
//!
//! - [`Theme`] owns the stack: a preset ([`PresetId`]), the application's
//!   [`Layer`]s, the accent and the application's conditions.
//!   [`Theme::from_preset`] builds one under a [`Scheme`]; every change
//!   validates the whole stack and is refused whole ([`LayerError`]), the
//!   theme staying as it was.
//! - [`Layer::from_toml`] reads a layer in the presets' format; a layer
//!   built in Rust is its [`Bases`] and its [`Rule`]s, each a [`Selector`]
//!   and what it sets.
//! - [`Theme::resolve`] resolves the theme for one window, which its
//!   [`WindowState`] records, under its [`WindowConditions`]: a
//!   [`Resolution`], whose [`ResolvedTheme`] the connector queries:
//!   [`ResolvedTheme::style`] of an [`Element`]'s part,
//!   [`ResolvedTheme::values`] of a widget, [`ResolvedTheme::tokens`],
//!   [`ResolvedTheme::role`] and [`ResolvedTheme::size_classes`]. A query
//!   answers [`Styled`]: the style, or why there is none to draw.
//! - The blocks a style is made of ([`Paint`], [`Stroke`], [`Corners`],
//!   [`Shadow`], [`TextStyle`], [`IconStyle`], [`Transition`], …), their
//!   units ([`Px`], [`Ms`], [`Deg`], [`Fraction`]) and colours ([`Color`],
//!   extended sRGB); the typed widgets (the button: [`ButtonPart`],
//!   [`ButtonValues`]).
//! - What a connector computes itself, at layout or per frame: [`corners`],
//!   [`shapes`], [`transitions`], [`timing`], [`ripple`] and
//!   [`color::composite_over`]; [`expr`], the expressions a value may derive
//!   by; [`placement`], where a popup goes against its anchor.
//!
//! ```no_run
//! use native_theme::{InputKind, Layer, PresetId, Scheme, Theme, WindowConditions, WindowState};
//!
//! let mut theme = Theme::from_preset(PresetId::Adwaita, Scheme::Light)?;
//! theme.add_layer(Layer::from_toml(
//!     r#"
//!     [[rules]]
//!     when = { states = ["hover"] }
//!     target = "button.body"
//!     set = { opacity = 0.5 }
//!     "#,
//! )?)?;
//! let mut window = WindowState::default();
//! let resolved = theme
//!     .resolve(&mut window, &WindowConditions::new(InputKind::Mouse))
//!     .theme;
//! # let _ = resolved;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! The model is being built: the eight bundled presets ([`PresetId::all`])
//! still list fields whose facts are pending, and [`Theme::from_preset`]
//! refuses a preset with a pending field ([`ThemeError::Preset`]), so none
//! builds a theme yet; the one typed widget is the button; the platform
//! readers are back ([`Theme::from_system`]): KDE's (the `kde` feature),
//! GNOME's (the `portal` feature), with the portal-only read of the Linux
//! desktops that have no reader of their own, macOS's (the `macos` feature)
//! and Windows' (the `windows` feature).
//!
//! # Feature flags
//!
//! Each OS-specific dependency is compiled only on its own target, so
//! `native` on macOS pulls in only the macOS dependencies. The model and its
//! bundled presets need no feature.
//!
//! | Feature | Enables |
//! |---|---|
//! | `kde` | The KDE Plasma reader, [`Theme::from_system`]: `kdeglobals` as KConfig cascades it, the colour scheme file it names and, on X11, `Xft.dpi`; and the KDE icon theme (Linux) |
//! | `macos` | The macOS reader, [`Theme::from_system`]: AppKit's appearances, colours and settings; the running macOS version and its generation, in [`pipeline::diagnose_platform_support`] |
//! | `windows` | The Windows reader, [`Theme::from_system`]: `UISettings`' colours, text scale and effects, high contrast and its system colours; the running Windows version and its generation, in [`pipeline::diagnose_platform_support`] |
//! | `portal` | The GNOME and Budgie reader, and the portal-only read of the Linux desktops with no reader of their own, [`Theme::from_system`]: the XDG Desktop Portal and gsettings (Linux) |
//! | `watch` | `watch::on_theme_change`, OS theme-change notifications: KDE (with `kde`), GNOME and the portal-only desktops (with `portal`), macOS (with `macos`) and Windows (with `windows`) |
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
//! # Modules beside the model
//!
//! - The crate root: [`Error`] and [`Result`], the platform APIs' errors, and
//!   from [`pipeline`] [`VersionMatch`] and [`DiagnosticEntry`].
//! - [`detect`]: OS detection: the Linux desktop environment, and the
//!   detection caches.
//! - [`icons`]: the icon roles and sets, the per-set icon loaders, the icon
//!   data types and the icon-set choice.
//! - [`fonts`]: font face matching; the platform's own typeface with the
//!   `system-fonts` feature.
//! - [`pipeline`]: a diagnosis of the platform's theme support.
//! - `watch` (the `watch` feature): OS theme-change notifications.
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
mod kde_config;
mod key;
mod layer;
mod palette;
pub mod placement;
#[doc(hidden)]
pub mod platform;
mod presets;
mod query;
mod reader;
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
/// The GNOME reader and the portal-only read: the XDG Desktop Portal and
/// gsettings (Linux).
#[cfg(all(target_os = "linux", feature = "portal"))]
mod gnome;
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
/// The KDE reader: kdeglobals, the scheme file it names and, on X11,
/// `Xft.dpi` (Linux); the KDE icon theme `detect` reports.
#[cfg(all(target_os = "linux", feature = "kde"))]
pub(crate) mod kde;
pub mod pipeline;
#[cfg(feature = "watch")]
pub mod watch;

/// Freedesktop icon theme lookup (Linux).
///
/// Requires the `system-icons` feature, and Linux.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub(crate) mod freedesktop;
/// The macOS reader and platform helpers; the reader's build from what
/// AppKit gave is compiled on every platform, for its tests.
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
/// The Windows reader and the running Windows version; the reader's build
/// from what Windows gave is compiled on every platform, for its tests.
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
    ResolvedShadow, ResolvedShadowLayer, Shadow, ShadowFalloff, ShadowLayer, ShadowLayerPatch,
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
pub use error::{
    Error, KeyError, LayerError, PortError, Problem, ProblemKind, ReaderFailure, ReaderStatus,
    ThemeError,
};
pub use expr::{Expr, ParseError};
pub use inputs::{Accent, Contrast, InputBases, InputName, InputValue, KdeSchemeKey, KdeWmKey};
pub use key::Key;
pub use layer::{Bases, Layer, LayerId};
pub use palette::Palette;
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
