//! # native-theme
//!
//! Cross-platform native theme detection and loading for Rust GUI applications.
//!
//! Any Rust GUI app can look native on any platform by loading a single theme
//! file or reading live OS settings, without coupling to any specific toolkit.
//!
//! A [`Theme`] is sparse and TOML-shaped: every field is an `Option`, and it
//! holds a light and a dark variant. Resolving one variant fills every field
//! the model requires and checks its ranges, giving a [`ResolvedTheme`] that
//! UI code reads without unwrapping. Most applications use a toolkit
//! connector, which does this and converts the result:
//! `native-theme-iced`, `native-theme-gpui` or `native-theme-egui`.
//!
//! # Quick start
//!
//! Read the OS theme and pick its active variant (the OS readers need the
//! `native` feature, or `kde`, `portal`, `macos`, `windows`):
//!
//! ```no_run
//! use native_theme::SystemTheme;
//!
//! let sys = SystemTheme::from_system()?;
//! let active = sys.pick(sys.mode); // &ResolvedTheme
//! let _accent = active.defaults.accent_color;
//! # Ok::<(), native_theme::Error>(())
//! ```
//!
//! Load a bundled preset and resolve one mode (no features needed):
//!
//! ```
//! use native_theme::{ColorMode, Theme};
//!
//! let resolved = Theme::preset("nord")?.resolve(ColorMode::Dark)?;
//! assert_eq!(resolved.mode, ColorMode::Dark);
//! let _background = resolved.variant.defaults.background_color;
//! # Ok::<(), native_theme::Error>(())
//! ```
//!
//! Apply an overlay to the OS theme; the overlay states only what it changes:
//!
//! ```no_run
//! use native_theme::{SystemTheme, Theme};
//!
//! let sys = SystemTheme::from_system()?;
//! let overlay = Theme::from_toml(r##"
//!     [light.defaults]
//!     accent_color = "#ff6600"
//!
//!     [dark.defaults]
//!     accent_color = "#ff6600"
//! "##)?;
//! let customised = sys.with_overlay(&overlay)?;
//! # Ok::<(), native_theme::Error>(())
//! ```
//!
//! Hand the result to a connector: `native-theme-iced`, `native-theme-gpui`
//! and `native-theme-egui` each have `from_system` and `from_preset`, which do
//! the steps above, and `to_theme`, which converts a [`ResolvedTheme`] you
//! resolved yourself into the toolkit's theme.
//!
//! Watch for OS theme changes (the `watch` feature). The callback runs on a
//! background thread and carries no theme: read the OS theme again. Keep the
//! subscription alive; dropping it stops watching:
//!
//! ```no_run
//! # #[cfg(feature = "watch")]
//! # {
//! let _subscription = native_theme::watch::on_theme_change(|_event| {
//!     // Tell the UI thread to call `SystemTheme::from_system()` again.
//! })?;
//! # }
//! # Ok::<(), native_theme::Error>(())
//! ```
//!
//! # Modules
//!
//! - The crate root: [`Theme`], [`ColorMode`], [`SystemTheme`],
//!   [`ResolvedTheme`], [`Resolved`], [`ResolutionContext`],
//!   [`AccessibilityPreferences`], [`Rgba`] and [`Error`].
//! - [`prelude`]: the ten most-used items, for a glob import.
//! - [`theme`]: the data model, the sparse types and their resolved
//!   counterparts, widget by widget.
//! - [`presets`]: [`PresetInfo`](presets::PresetInfo), the description of a
//!   bundled preset that [`Theme::list_presets`] returns; [`Theme::preset`]
//!   loads one.
//! - [`resolve`]: the resolution-time inputs.
//! - [`detect`]: OS detection: dark mode, reduced motion, accessibility
//!   preferences, icon theme and set, desktop environment.
//! - [`icons`]: the per-set icon loaders, the icon data types and the
//!   icon-set choice.
//! - [`fonts`]: font face matching; the platform's own typeface with the
//!   `system-fonts` feature.
//! - [`pipeline`]: the running platform's preset and a diagnosis of its
//!   theme support.
//! - [`error`] and [`color`]: the error and colour types.
//! - `watch` (the `watch` feature): OS theme-change notifications.
//! - `rasterize` (the `svg-rasterize` feature): SVG icons to RGBA.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// Generates `merge()` and `is_empty()` methods for theme structs.
///
/// Three field categories:
/// - `option { field1, field2, ... }` -- `Option<T>` leaf fields
/// - `nested { field1, field2, ... }` -- nested struct fields with their own `merge()`
/// - `optional_nested { field1, field2, ... }` -- `Option<T>` where T has its own `merge()`
///
/// For `option` fields, `Some` values in the overlay replace the
/// corresponding fields in self; `None` fields are left unchanged.
/// For `nested` fields, merge is called recursively.
/// For `optional_nested` fields: if both base and overlay are `Some`, the inner values
/// are merged recursively. If base is `None` and overlay is `Some`, overlay is cloned.
/// If overlay is `None`, the base field is preserved unchanged.
///
/// # Examples
///
/// ```ignore
/// impl_merge!(MyColors {
///     option { accent, background }
/// });
/// ```
macro_rules! impl_merge {
    (
        $struct_name:ident {
            $(option { $($opt_field:ident),* $(,)? })?
            $(nested { $($nest_field:ident),* $(,)? })?
            $(optional_nested { $($on_field:ident),* $(,)? })*
        }
    ) => {
        impl $struct_name {
            /// Merge an overlay into this value. `Some` fields in the overlay
            /// replace the corresponding fields in self; `None` fields are
            /// left unchanged. Nested structs are merged recursively.
            pub fn merge(&mut self, overlay: &Self) {
                $($(
                    if overlay.$opt_field.is_some() {
                        self.$opt_field = overlay.$opt_field.clone();
                    }
                )*)?
                $($(
                    self.$nest_field.merge(&overlay.$nest_field);
                )*)?
                $($(
                    match (&mut self.$on_field, &overlay.$on_field) {
                        (Some(base), Some(over)) => base.merge(over),
                        (None, Some(over)) => self.$on_field = Some(over.clone()),
                        _ => {}
                    }
                )*)*
            }

            /// Returns true if all fields are at their default (None/empty) state.
            pub fn is_empty(&self) -> bool {
                true
                $($(&& self.$opt_field.is_none())*)?
                $($(&& self.$nest_field.is_empty())*)?
                $($(&& self.$on_field.as_ref().map_or(true, |v| v.is_empty()))*)*
            }
        }
    };
}

/// Color types and sRGB utilities.
pub mod color;
/// OS detection: dark mode, reduced motion, accessibility preferences, icon
/// theme and set, desktop environment.
pub mod detect;
/// Error types for theme operations.
pub mod error;
pub mod fonts;
/// GNOME portal theme reader.
///
/// Requires the `portal` feature, and Linux.
#[cfg(all(target_os = "linux", feature = "portal"))]
pub mod gnome;
/// Icon loading and dispatch.
pub mod icons;
/// KDE theme reader.
///
/// Requires the `kde` feature, and Linux.
#[cfg(all(target_os = "linux", feature = "kde"))]
pub mod kde;
/// Breeze widget sizing constants. Compiled on every target and feature set,
/// so the preset gate sees the values the KDE reader states.
#[path = "kde/metrics.rs"]
pub(crate) mod kde_metrics;
/// Theme data model types, public through [`theme`].
mod model;
/// Theme pipeline: reader -> preset merge -> resolve -> validate.
pub mod pipeline;
/// Bundled theme presets.
pub mod presets;
/// Internal `ThemeReader` trait (see module docs for Option A rationale).
///
/// Consumed by `pipeline::select_reader` as `Box<dyn ThemeReader>`; the
/// trait and every impl are `pub(crate)` — not part of the public API.
mod reader;
/// Theme resolution engine (inheritance + validation).
///
/// Public surface: [`resolve::ResolutionContext`] — the resolution-time
/// inputs struct consumed by
/// [`ThemeMode::into_resolved`](crate::theme::ThemeMode::into_resolved).
/// Inheritance rules, validation machinery, and the range-check helpers
/// are all crate-internal (no stability guarantee).
pub mod resolve;
#[cfg(any(
    feature = "material-icons",
    feature = "lucide-icons",
    feature = "system-icons"
))]
mod spinners;
#[cfg(feature = "watch")]
pub mod watch;

/// Convenience re-exports for common usage.
///
/// `use native_theme::prelude::*` imports:
/// [`Theme`], [`ColorMode`], [`Resolved`], [`ResolvedTheme`], [`SystemTheme`],
/// [`AccessibilityPreferences`], [`ResolutionContext`], [`Rgba`], [`Error`],
/// and [`Result`].
pub mod prelude;

/// Theme data model: types, defaults, fonts, borders, widgets.
///
/// Core types: [`Theme`], [`ThemeMode`],
/// [`ResolvedTheme`], [`ResolvedDefaults`].
pub mod theme {
    pub use crate::model::widgets::*;
    pub use crate::model::{
        ArrowSide, ColorMode, DefaultsBorderSpec, DialogButtonOrder, FontSize, FontSpec, FontStyle,
        IconSet, IconSizes, Resolved, ResolvedDefaults, ResolvedDefaultsBorder, ResolvedFontSpec,
        ResolvedIconSizes, ResolvedPadding, ResolvedTextScale, ResolvedTextScaleEntry,
        ResolvedTheme, ResolvedWidgetBorder, TabIndicatorSide, TextScale, TextScaleEntry, Theme,
        ThemeDefaults, ThemeMode, WidgetBorderSpec, intern_font_family,
    };
    pub use crate::presets::PresetInfo;
}

/// Freedesktop icon theme lookup (Linux).
///
/// Requires the `system-icons` feature, and Linux.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub(crate) mod freedesktop;
/// macOS platform helpers.
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
/// Windows platform theme reader.
#[cfg(target_os = "windows")]
#[cfg_attr(not(feature = "windows"), allow(dead_code))]
pub(crate) mod windows;
#[cfg(not(target_os = "windows"))]
#[allow(dead_code, unused_variables)]
pub(crate) mod windows;
/// Windows Segoe Fluent / stock icon loader.
///
/// Requires the `system-icons` feature, and Windows.
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub(crate) mod winicons;
#[cfg(all(not(target_os = "windows"), feature = "system-icons"))]
#[allow(dead_code, unused_imports)]
pub(crate) mod winicons;

use std::borrow::Cow;

/// Convenience Result type alias for this crate.
pub type Result<T> = std::result::Result<T, error::Error>;

// Internal re-exports: keep crate::Type paths working for internal modules
// without exposing them in the public API. External users access types via
// native_theme::theme::*, native_theme::icons::*, native_theme::detect::*, etc.
#[allow(unused_imports)]
pub(crate) use color::ParseColorError;
pub use color::Rgba;
#[cfg(target_os = "linux")]
#[allow(unused_imports)]
pub(crate) use detect::LinuxDesktop;
#[cfg(target_os = "linux")]
#[allow(unused_imports)]
pub(crate) use detect::detect_linux_desktop;
#[allow(unused_imports)]
pub(crate) use detect::{detect_icon_theme, system_icon_set, system_icon_theme};
#[allow(unused_imports)]
pub(crate) use detect::{
    detect_is_dark, detect_reduced_motion, invalidate_caches, prefers_reduced_motion,
    system_is_dark,
};
pub use error::Error;
#[allow(unused_imports)]
pub(crate) use error::{ErrorKind, RangeViolation};
#[allow(unused_imports)]
pub(crate) use icons::{
    FreedesktopLoader, IconId, LucideLoader, MaterialLoader, SegoeFluentLoader, SfSymbolsLoader,
    is_freedesktop_theme_available, load_icon, load_icon_indicator,
};
#[allow(unused_imports)]
pub(crate) use model::icons::icon_name;
#[allow(unused_imports)]
pub(crate) use model::{
    AnimatedIcon, ButtonTheme, CardTheme, CheckboxTheme, ComboBoxTheme, DefaultsBorderSpec,
    DialogButtonOrder, DialogTheme, ExpanderTheme, FontSize, FontSpec, FontStyle, IconData,
    IconProvider, IconRole, IconSet, IconSizes, InputTheme, LayoutTheme, LinkTheme, ListTheme,
    MenuTheme, PopoverTheme, ProgressBarTheme, ResolvedDefaults, ResolvedDefaultsBorder,
    ResolvedFontSpec, ResolvedIconSizes, ResolvedPadding, ResolvedTextScale,
    ResolvedTextScaleEntry, ResolvedWidgetBorder, ScrollbarTheme, SegmentedControlTheme,
    SeparatorTheme, SidebarTheme, SliderTheme, SpinnerTheme, SplitterTheme, StatusBarTheme,
    SwitchTheme, TabTheme, TextScale, TextScaleEntry, ThemeDefaults, ThemeMode, ToolbarTheme,
    TooltipTheme, TransformAnimation, WidgetBorderSpec, WindowTheme, bundled_icon_by_name,
    bundled_icon_svg,
};
pub use pipeline::{DiagnosticEntry, PlatformPreset, VersionMatch};
#[allow(unused_imports)]
pub(crate) use pipeline::{diagnose_platform_support, platform_preset_name};
pub use resolve::ResolutionContext;
pub use theme::{ColorMode, Resolved, ResolvedTheme, Theme};

/// OS-detected accessibility preferences.
///
/// A single copy lives on [`SystemTheme`], shared across light and dark
/// variants. These are runtime values detected from the OS -- not stored
/// in TOML presets. Outside this crate, start from
/// [`AccessibilityPreferences::DEFAULT`] (or `default()`) and set fields.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AccessibilityPreferences {
    /// Text scaling factor (1.0 = no scaling). Multiply font sizes by
    /// this factor when honoring the user's preference for larger text.
    pub text_scaling_factor: f32,
    /// Whether the user has requested reduced motion.
    pub reduce_motion: bool,
    /// Whether a high-contrast mode is active.
    pub high_contrast: bool,
    /// Whether the user has requested reduced transparency.
    pub reduce_transparency: bool,
}

impl Default for AccessibilityPreferences {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl AccessibilityPreferences {
    /// No text scaling and no flag set: what [`Default::default`] returns,
    /// usable in a `const` or `static`.
    pub const DEFAULT: Self = Self {
        text_scaling_factor: 1.0,
        reduce_motion: false,
        high_contrast: false,
        reduce_transparency: false,
    };

    /// Read the OS accessibility preferences without resolving a theme.
    ///
    /// Runs the same platform reader [`SystemTheme::from_system`] runs (KDE
    /// `kdeglobals`, GNOME portal + gsettings, macOS `NSWorkspace`, Windows
    /// `UISettings`) and takes its accessibility block, blocking on it with
    /// `pollster::block_on`; `reduce_motion` is additionally read through
    /// [`crate::detect::detect_reduced_motion`] on every platform. Fields no
    /// reader supplies keep their defaults. Never fails: with no reader or a
    /// failing reader the defaults are returned.
    ///
    /// Use it on the preset path, where accessibility is orthogonal to the
    /// theme choice (a user with large text wants it under a preset too).
    #[must_use]
    pub fn from_system() -> Self {
        pollster::block_on(pipeline::accessibility_from_system_inner())
    }

    /// The text-scaling multiplier: [`text_scaling_factor`](Self::text_scaling_factor)
    /// when it is finite and positive, else `1.0`, so a factor no platform
    /// could mean (NaN, zero, negative, infinite) leaves text at its size.
    #[must_use]
    pub fn text_scale_factor(&self) -> f32 {
        let s = self.text_scaling_factor;
        if s.is_finite() && s > 0.0 { s } else { 1.0 }
    }

    /// A text size times [`text_scale_factor`](Self::text_scale_factor).
    ///
    /// For every text size alike: a widget's font, a `text_scale` role's
    /// size, the default fonts. No other length is scaled.
    #[must_use]
    pub fn scaled(&self, size: f32) -> f32 {
        size * self.text_scale_factor()
    }
}

/// Complete reader result for the pipeline.
///
/// Bundles the type-safe [`ReaderOutput`] with reader metadata
/// (name, icon_set, layout, font_dpi, accessibility) so that
/// `run_pipeline` accepts a single struct instead of many arguments.
#[derive(Clone, Debug)]
pub(crate) struct ReaderResult {
    /// The reader's variant data.
    pub(crate) output: ReaderOutput,
    /// Theme name from reader (e.g. "BreezeDark", "GNOME", "macOS").
    pub(crate) name: Cow<'static, str>,
    /// Shared icon_set from reader.
    pub(crate) icon_set: Option<IconSet>,
    /// Shared layout from reader.
    pub(crate) layout: LayoutTheme,
    /// Font DPI captured at detection time (None = auto-detect).
    pub(crate) font_dpi: Option<f32>,
    /// OS-detected accessibility preferences.
    pub(crate) accessibility: AccessibilityPreferences,
}

/// Output contract for platform readers.
///
/// Expresses single-vs-dual variant semantics explicitly:
/// - `Single`: KDE, GNOME, and Windows readers report only the OS-active mode.
///   The pipeline fills the inactive variant from the platform preset.
/// - `Dual`: macOS reads both light and dark appearances in a single call.
///   The pipeline uses both reader-provided variants directly.
///
/// Both carry `is_dark`, the OS-active mode, from which the pipeline sets
/// [`SystemTheme::mode`].
#[derive(Clone, Debug)]
pub(crate) enum ReaderOutput {
    /// Reader provides only the OS-active variant. The pipeline fills the
    /// inactive variant from the platform preset.
    #[cfg_attr(all(target_os = "windows", not(feature = "windows")), allow(dead_code))]
    Single {
        /// The reader-provided variant (OS-active).
        mode: Box<ThemeMode>,
        /// Which color mode this variant represents.
        is_dark: bool,
    },
    /// Reader provides both light and dark variants (macOS).
    #[allow(dead_code)]
    Dual {
        /// The light variant from the reader.
        light: Box<ThemeMode>,
        /// The dark variant from the reader.
        dark: Box<ThemeMode>,
        /// Whether the OS-active appearance is the dark one.
        is_dark: bool,
    },
}

impl ReaderOutput {
    /// Reconstruct a [`Theme`] from this reader output (for overlay replay
    /// and merge compatibility).
    pub(crate) fn to_theme(
        &self,
        name: &str,
        icon_set: Option<IconSet>,
        layout: &LayoutTheme,
    ) -> Theme {
        let (light, dark) = match self {
            ReaderOutput::Single { mode, is_dark } => {
                if *is_dark {
                    (None, Some(ThemeMode::clone(mode)))
                } else {
                    (Some(ThemeMode::clone(mode)), None)
                }
            }
            ReaderOutput::Dual { light, dark, .. } => {
                (Some(ThemeMode::clone(light)), Some(ThemeMode::clone(dark)))
            }
        };
        Theme {
            name: std::borrow::Cow::Owned(name.to_string()),
            light,
            dark,
            layout: layout.clone(),
            icon_set,
            // Readers keep Theme-level icon_theme = None and rely on either
            // the preset's value (tier 2) or system detect (tier 3). Readers
            // that need to override per color mode use ThemeDefaults::icon_theme
            // on the variant (tier 1, e.g. the KDE reader for breeze/breeze-dark).
            icon_theme: None,
        }
    }
}

/// Data needed to replay the merge+resolve pipeline for overlay support.
///
/// Stores the reader's output, the preset name and every overlay applied so
/// far (merged into one [`Theme`]), so that [`SystemTheme::with_overlay()`]
/// rebuilds the pre-resolve variants (preset, then reader, then overlays)
/// and resolves them again.
#[derive(Clone, Debug)]
pub(crate) struct OverlaySource {
    /// The reader's variant data for replay.
    pub(crate) reader_output: ReaderOutput,
    /// Theme name from reader.
    pub(crate) name: Cow<'static, str>,
    /// Shared icon_set from reader.
    pub(crate) icon_set: Option<IconSet>,
    /// Shared layout from reader.
    pub(crate) layout: LayoutTheme,
    /// The platform preset name (e.g. "kde-breeze").
    pub(crate) preset_name: String,
    /// Resolution-time inputs captured at detection time. Replaces the
    /// old `font_dpi: Option<f32>` field; the context bundles
    /// `font_dpi` + `button_order` + detected `icon_theme` and is cloned
    /// into `with_overlay` replays so resolution is deterministic across
    /// overlay applications.
    pub(crate) context: crate::resolve::ResolutionContext,
    /// Every overlay applied so far, merged in order; `None` before the
    /// first [`SystemTheme::with_overlay()`].
    pub(crate) overlay: Option<Theme>,
}

/// Result of the OS-first pipeline. Holds both resolved variants.
///
/// Produced by [`SystemTheme::from_system()`] and [`SystemTheme::from_system_async()`].
/// Both light and dark are always populated: the OS-active variant
/// comes from the reader + preset + resolve, the inactive variant
/// comes from the preset + resolve.
#[derive(Clone, Debug)]
pub struct SystemTheme {
    /// Theme name (from reader or preset).
    ///
    /// # Ownership type
    ///
    /// This field uses `Cow<'static, str>`, not `Arc<str>`. The v0.5.7 API
    /// review recommended uniform `Arc<str>` across `name`, `icon_theme`,
    /// `ReaderOutput::name`, and `ResolvedFontSpec::family`. The audit in
    /// `docs/archive/v0.5.7_gaps.md` §G9 concluded that the
    /// uniform recommendation should be adopted ONLY for
    /// [`ResolvedFontSpec::family`](crate::model::font::ResolvedFontSpec)
    /// (where 26 widgets × connectors genuinely share font families), and
    /// should be REVERSED for `name` / `icon_theme` because:
    ///
    /// - Each resolved theme carries exactly ONE `name` — no dedup benefit.
    /// - Bundled preset names are `&'static str` literals; `Cow::Borrowed(static_lit)`
    ///   is zero allocation, zero refcount. `Arc<str>` would require at least one
    ///   allocation per unique string at construction time, paying allocation cost
    ///   for a dedup benefit that is structurally absent.
    ///
    /// The same reasoning applies symmetrically to
    /// [`SystemTheme::icon_theme`](Self::icon_theme),
    /// [`Theme::name`](crate::theme::Theme), and
    /// [`ThemeDefaults::icon_theme`](crate::model::defaults::ThemeDefaults).
    ///
    /// See `docs/archive/v0.5.7_gaps.md` §G9 for the full audit.
    pub name: Cow<'static, str>,
    /// The OS color mode preference (light or dark).
    pub mode: ColorMode,
    /// Resolved light variant (always populated).
    pub light: ResolvedTheme,
    /// Resolved dark variant (always populated).
    pub dark: ResolvedTheme,
    /// Data for replaying the pipeline on overlay (replaces light_variant/dark_variant).
    pub(crate) overlay_source: OverlaySource,
    /// The platform preset used (e.g., "kde-breeze", "adwaita", "macos-tahoe").
    pub preset: String,
    /// How the running OS version relates to the generation of
    /// [`preset`](Self::preset): `None` where the platform has no generation
    /// table (Linux, whose readers read the installed style) or no reader ran.
    /// On macOS 27 under `macos-tahoe`, which no preset of its own covers yet,
    /// it is `Some(VersionMatch::PresetOlder)`; on Windows 10 under
    /// `windows-11`, `Some(VersionMatch::PresetNewer)`. Early macOS 26 betas
    /// reported version 16 to a binary built against the macOS 15 SDK; a
    /// major version 16 gets `macos-monterey` with
    /// `Some(VersionMatch::PresetOlder)` (sources in the macOS generation
    /// table, `pipeline.rs`).
    pub version_match: Option<VersionMatch>,
    /// Which icon loading mechanism to use for this theme.
    pub icon_set: IconSet,
    /// The name of the visual icon theme (e.g. `"breeze"`, `"Adwaita"`):
    /// the active variant's, else the theme's, else the detected system
    /// icon theme. `None` where the theme states none and detection fails;
    /// [`system_icon_theme()`](crate::detect::system_icon_theme) gives the
    /// reason.
    ///
    /// # Ownership type
    ///
    /// `Cow<'static, str>` is used here per the same principled deviation
    /// documented on [`SystemTheme::name`](Self::name) — see `docs/archive/v0.5.7_gaps.md`
    /// §G9. Each resolved theme carries a single icon-theme name (KDE has
    /// exactly two across light/dark variants — `"breeze"` / `"breeze-dark"`;
    /// other platforms have one), so the `Arc<str>` dedup benefit does not apply.
    pub icon_theme: Option<Cow<'static, str>>,
    /// Each variant's icon theme, resolved by the same three tiers as
    /// [`icon_theme`](Self::icon_theme), kept by variant rather than relative
    /// to [`mode`](Self::mode), a public field a caller may change. Read
    /// through [`icon_theme_for`](Self::icon_theme_for).
    pub(crate) light_icon_theme: Option<Cow<'static, str>>,
    /// The dark variant's counterpart of [`light_icon_theme`](Self::light_icon_theme).
    pub(crate) dark_icon_theme: Option<Cow<'static, str>>,
    /// Layout spacing shared by both variants: the platform reader's values
    /// merged field-wise over the preset's, the same precedence the pipeline
    /// uses for colours. `None` in a field means neither the platform nor the
    /// preset specifies it (platform-facts §2.20); nothing is invented.
    pub layout: LayoutTheme,
    /// OS-detected accessibility preferences (shared across variants).
    pub accessibility: AccessibilityPreferences,
}

impl SystemTheme {
    /// Pick a resolved variant by color mode.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use native_theme::theme::ColorMode;
    ///
    /// let sys = native_theme::SystemTheme::from_system()?;
    /// let dark = sys.pick(ColorMode::Dark);
    /// let active = sys.pick(sys.mode);
    /// # Ok::<(), native_theme::error::Error>(())
    /// ```
    #[must_use]
    pub fn pick(&self, mode: ColorMode) -> &ResolvedTheme {
        match mode {
            ColorMode::Light => &self.light,
            ColorMode::Dark => &self.dark,
        }
    }

    /// The icon-theme name for one colour mode, resolved for that mode's
    /// variant by the same three tiers as [`icon_theme`](Self::icon_theme)
    /// (its `defaults.icon_theme`, then the theme's, then the detected system
    /// theme); for the mode the theme was built in it equals `icon_theme` as
    /// built. KDE names one theme per variant (`breeze`, `breeze-dark`); a
    /// toolkit that keeps a style per colour scheme needs both. `None` where
    /// that variant states none and detection failed; nothing is invented.
    #[must_use]
    pub fn icon_theme_for(&self, mode: ColorMode) -> Option<&str> {
        match mode {
            ColorMode::Light => self.light_icon_theme.as_deref(),
            ColorMode::Dark => self.dark_icon_theme.as_deref(),
        }
    }

    /// Apply an app-level TOML overlay and re-resolve.
    ///
    /// Merges the overlay onto the pre-resolve [`ThemeMode`] (not the
    /// already-resolved [`ResolvedTheme`]) so that changed source fields
    /// propagate correctly through `resolve()`. For example, changing
    /// `defaults.accent_color` in the overlay will cause `button.primary_background`,
    /// `checkbox.checked_background`, `slider.fill`, etc. to be re-derived from
    /// the new accent color.
    ///
    /// Overlays stack: each call applies its overlay over the ones applied
    /// before it, so `sys.with_overlay(&a)?.with_overlay(&b)?` keeps what `a`
    /// states unless `b` states it too.
    ///
    /// The overlay's `icon_set`, `icon_theme` and `[layout]` apply too, by
    /// the precedence [`from_system()`](Self::from_system) uses: a variant's
    /// `defaults.icon_theme`, then the theme's `icon_theme`, then the
    /// detected one. So an overlay's top-level `icon_theme` loses to a
    /// variant's own, such as the KDE reader's `breeze` and `breeze-dark`;
    /// to replace those, state `[light.defaults] icon_theme` and
    /// `[dark.defaults] icon_theme`.
    ///
    /// The result's `light`, `dark`, `layout`, `icon_set` and `icon_theme`
    /// (with [`icon_theme_for`](Self::icon_theme_for)'s per-variant names)
    /// are rebuilt from the theme's sources and the overlays, not copied
    /// from `self`, so a change the caller made to one of those public fields
    /// is lost. `name`, `mode`, `preset`, `version_match` and `accessibility`
    /// are kept from `self`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ResolutionInvalid`] when a value the overlay states,
    /// or one that inherits it, is out of range. Each path is the TOML key,
    /// as [`Error::ResolutionIncomplete`] describes: a variant's value under
    /// its variant table (`light.button.min_height_px`), a `[layout]` value
    /// with no variant (`layout.widget_gap_px`).
    ///
    /// # Examples
    ///
    /// An overlay is a theme file that states only what it changes; it needs
    /// no `name`:
    ///
    /// ```
    /// let overlay = native_theme::Theme::from_toml(r##"
    ///     [light.defaults]
    ///     accent_color = "#ff6600"
    ///     [dark.defaults]
    ///     accent_color = "#ff6600"
    /// "##)?;
    /// # Ok::<(), native_theme::Error>(())
    /// ```
    ///
    /// Then apply it to the live OS theme:
    ///
    /// ```no_run
    /// # let overlay = native_theme::Theme::default();
    /// let system = native_theme::SystemTheme::from_system()?;
    /// let customized = system.with_overlay(&overlay)?;
    /// // customized.pick(customized.mode).defaults.accent_color is now #ff6600
    /// // and all accent-derived fields are updated
    /// # Ok::<(), native_theme::Error>(())
    /// ```
    pub fn with_overlay(&self, overlay: &Theme) -> crate::Result<Self> {
        let mut source = self.overlay_source.clone();
        // The earlier overlays go under this one
        source.overlay = Some(match source.overlay.take() {
            Some(mut stacked) => {
                stacked.merge(overlay);
                stacked
            }
            None => overlay.clone(),
        });
        Ok(SystemTheme {
            name: self.name.clone(),
            preset: self.preset.clone(),
            version_match: self.version_match,
            ..pipeline::replay(source, self.mode, self.accessibility.clone())?
        })
    }

    /// Load the OS theme synchronously.
    ///
    /// Detects the platform and desktop environment, reads the current theme
    /// settings, merges with a platform preset, and returns a fully resolved
    /// [`SystemTheme`] with both light and dark variants.
    ///
    /// The return value goes through the full pipeline: reader output ->
    /// resolve -> validate -> [`SystemTheme`] with both light and dark
    /// [`ResolvedTheme`] variants.
    ///
    /// On every platform `pollster::block_on` drives the async implementation
    /// that [`from_system_async`](Self::from_system_async) awaits.
    ///
    /// # Platform Behavior
    ///
    /// - **macOS:** with the `macos` feature, reads both light and dark
    ///   variants via NSAppearance and merges them with the preset of the
    ///   running version's generation, `macos-monterey` on 12 to 15 and
    ///   `macos-tahoe` on 26 and later ([`version_match`](Self::version_match)
    ///   reports macOS 27 as `PresetOlder`); the active mode comes from
    ///   [`detect::detect_is_dark`].
    /// - **Linux:** the async implementation handles portal D-Bus calls when
    ///   the `portal` feature is enabled. Without a reader for the detected
    ///   desktop (feature disabled, or a desktop with no reader), returns the
    ///   desktop's platform preset with no live reading (`kde-breeze` on KDE,
    ///   `adwaita` elsewhere, as [`pipeline::platform_preset_name`] names
    ///   it), resolved in the light/dark mode [`detect::detect_is_dark`]
    ///   reads.
    /// - **Windows:** with the `windows` feature, reads the active variant and
    ///   merges it with the `windows-11` preset, the one Windows generation
    ///   ([`version_match`](Self::version_match) reports Windows 10 as
    ///   `PresetNewer`).
    /// - **Other platforms:** Returns `Error::PlatformUnsupported`.
    ///
    /// # Errors
    ///
    /// - `Error::FeatureDisabled` on macOS or Windows when the `macos` /
    ///   `windows` feature is not enabled.
    /// - `Error::PlatformUnsupported` if the platform has no reader at all.
    /// - `Error::ReaderFailed` if the platform reader cannot access theme data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let sys = native_theme::SystemTheme::from_system()?;
    /// let theme = sys.pick(sys.mode);
    /// // Icon set and theme are on SystemTheme, shared across variants
    /// let _icon_set = sys.icon_set;
    /// let _icon_theme = &sys.icon_theme;
    /// # Ok::<(), native_theme::error::Error>(())
    /// ```
    pub fn from_system() -> crate::Result<Self> {
        pollster::block_on(pipeline::from_system_inner())
    }

    /// Async version of [`from_system()`](Self::from_system).
    ///
    /// On Linux, this enables portal D-Bus calls (e.g. GNOME settings portal,
    /// KDE portal backend detection) via `.await`. On macOS and Windows, the
    /// future completes immediately -- no actual async operations occur.
    ///
    /// Returns a [`SystemTheme`] with both resolved light and dark variants,
    /// same as [`from_system()`](Self::from_system).
    pub async fn from_system_async() -> crate::Result<Self> {
        pipeline::from_system_inner().await
    }

    /// Load the OS theme over a chosen generation's preset: the platform
    /// reader runs as in [`from_system()`](Self::from_system), and its values
    /// merge over `preset` instead of the running version's generation.
    ///
    /// `preset` must be a generation of the running platform: `macos-monterey`
    /// or `macos-tahoe` on macOS, `windows-11` on Windows. Any other name, and
    /// any name on a platform without a generation table (Linux, whose readers
    /// read the installed style, BSD and other platforms), is
    /// `Error::UnknownPreset`, where [`from_system()`](Self::from_system) may
    /// return `Error::PlatformUnsupported` (BSD and other platforms).
    ///
    /// The result is a hybrid, not the older OS's look: the chosen
    /// generation's geometry and design colours under the running OS's
    /// colours, fonts and accessibility settings, which the reader reads live
    /// (on macOS 26 with `macos-monterey`, `NSColor` gives macOS 26's
    /// colours). [`version_match`](Self::version_match) reports the chosen
    /// generation against the running OS: `PresetOlder` for an older one,
    /// `PresetNewer` for a newer one.
    ///
    /// # Errors
    ///
    /// - `Error::UnknownPreset` for a name that is not a generation of the
    ///   running platform, and for any name on a platform without a
    ///   generation table.
    /// - `Error::FeatureDisabled` on macOS or Windows when the `macos` /
    ///   `windows` feature is not enabled.
    /// - `Error::ReaderFailed` if the platform reader cannot access theme data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Keep the Monterey-to-Sequoia look on a Mac running macOS 26.
    /// let sys = native_theme::SystemTheme::from_system_with_preset("macos-monterey")?;
    /// assert_eq!(sys.preset, "macos-monterey");
    /// # Ok::<(), native_theme::error::Error>(())
    /// ```
    pub fn from_system_with_preset(preset: &str) -> crate::Result<Self> {
        pollster::block_on(pipeline::from_system_with_preset_inner(preset))
    }

    /// Async version of
    /// [`from_system_with_preset()`](Self::from_system_with_preset).
    pub async fn from_system_with_preset_async(preset: &str) -> crate::Result<Self> {
        pipeline::from_system_with_preset_inner(preset).await
    }
}

// =============================================================================
// Tests -- SystemTheme public API (active, pick, platform_preset_name)
// =============================================================================

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::field_reassign_with_default
)]
mod system_theme_tests {
    use super::*;

    // --- SystemTheme::active() / pick() tests ---

    #[test]
    fn test_system_theme_pick_dark_mode() {
        let preset = Theme::preset("catppuccin-mocha").unwrap();
        let mut light_v = preset.light.clone().unwrap();
        let mut dark_v = preset.dark.clone().unwrap();
        // Give them distinct accents so we can tell them apart
        // (test fixture values -- not production hardcoded colors)
        light_v.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
        dark_v.defaults.accent_color = Some(Rgba::rgb(255, 0, 0));
        light_v.resolve_all();
        dark_v.resolve_all();
        let light_resolved = light_v.validate().unwrap();
        let dark_resolved = dark_v.validate().unwrap();

        let st = SystemTheme {
            name: "test".into(),
            mode: ColorMode::Dark,
            light: light_resolved.clone(),
            dark: dark_resolved.clone(),
            overlay_source: OverlaySource {
                reader_output: ReaderOutput::Dual {
                    light: Box::new(ThemeMode::default()),
                    dark: Box::new(ThemeMode::default()),
                    is_dark: false,
                },
                name: Cow::Borrowed(""),
                icon_set: None,
                layout: LayoutTheme::default(),
                preset_name: "catppuccin-mocha".into(),
                context: crate::resolve::ResolutionContext::for_tests(),
                overlay: None,
            },
            preset: "catppuccin-mocha".into(),
            version_match: None,
            icon_set: IconSet::Lucide,
            icon_theme: Some("lucide".into()),
            light_icon_theme: None,
            dark_icon_theme: Some("lucide".into()),
            layout: LayoutTheme::default(),
            accessibility: AccessibilityPreferences::default(),
        };
        assert_eq!(
            st.pick(st.mode).defaults.accent_color,
            dark_resolved.defaults.accent_color
        );
    }

    #[test]
    fn test_system_theme_pick_light_mode() {
        let preset = Theme::preset("catppuccin-mocha").unwrap();
        let mut light_v = preset.light.clone().unwrap();
        let mut dark_v = preset.dark.clone().unwrap();
        light_v.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
        dark_v.defaults.accent_color = Some(Rgba::rgb(255, 0, 0));
        light_v.resolve_all();
        dark_v.resolve_all();
        let light_resolved = light_v.validate().unwrap();
        let dark_resolved = dark_v.validate().unwrap();

        let st = SystemTheme {
            name: "test".into(),
            mode: ColorMode::Light,
            light: light_resolved.clone(),
            dark: dark_resolved.clone(),
            overlay_source: OverlaySource {
                reader_output: ReaderOutput::Dual {
                    light: Box::new(ThemeMode::default()),
                    dark: Box::new(ThemeMode::default()),
                    is_dark: false,
                },
                name: Cow::Borrowed(""),
                icon_set: None,
                layout: LayoutTheme::default(),
                preset_name: "catppuccin-mocha".into(),
                context: crate::resolve::ResolutionContext::for_tests(),
                overlay: None,
            },
            preset: "catppuccin-mocha".into(),
            version_match: None,
            icon_set: IconSet::Lucide,
            icon_theme: Some("lucide".into()),
            light_icon_theme: Some("lucide".into()),
            dark_icon_theme: None,
            layout: LayoutTheme::default(),
            accessibility: AccessibilityPreferences::default(),
        };
        assert_eq!(
            st.pick(st.mode).defaults.accent_color,
            light_resolved.defaults.accent_color
        );
    }

    #[test]
    fn test_system_theme_pick_explicit() {
        let preset = Theme::preset("catppuccin-mocha").unwrap();
        let mut light_v = preset.light.clone().unwrap();
        let mut dark_v = preset.dark.clone().unwrap();
        light_v.defaults.accent_color = Some(Rgba::rgb(0, 0, 255));
        dark_v.defaults.accent_color = Some(Rgba::rgb(255, 0, 0));
        light_v.resolve_all();
        dark_v.resolve_all();
        let light_resolved = light_v.validate().unwrap();
        let dark_resolved = dark_v.validate().unwrap();

        let st = SystemTheme {
            name: "test".into(),
            mode: ColorMode::Light,
            light: light_resolved.clone(),
            dark: dark_resolved.clone(),
            overlay_source: OverlaySource {
                reader_output: ReaderOutput::Dual {
                    light: Box::new(ThemeMode::default()),
                    dark: Box::new(ThemeMode::default()),
                    is_dark: false,
                },
                name: Cow::Borrowed(""),
                icon_set: None,
                layout: LayoutTheme::default(),
                preset_name: "catppuccin-mocha".into(),
                context: crate::resolve::ResolutionContext::for_tests(),
                overlay: None,
            },
            preset: "catppuccin-mocha".into(),
            version_match: None,
            icon_set: IconSet::Lucide,
            icon_theme: Some("lucide".into()),
            light_icon_theme: Some("lucide".into()),
            dark_icon_theme: None,
            layout: LayoutTheme::default(),
            accessibility: AccessibilityPreferences::default(),
        };
        assert_eq!(
            st.pick(ColorMode::Dark).defaults.accent_color,
            dark_resolved.defaults.accent_color
        );
        assert_eq!(
            st.pick(ColorMode::Light).defaults.accent_color,
            light_resolved.defaults.accent_color
        );
    }

    // --- platform_preset_name() pure tests ---
    // Tests the same logic path (parse_linux_desktop -> linux_preset_for_de) without env var mocking.

    /// Prove that the sync `from_system()` API works without any async runtime.
    /// On Linux with KDE feature: exercises pollster::block_on(from_system_inner()).
    /// This acts as a compile-time and runtime gate that the sync path works.
    #[test]
    #[cfg(target_os = "linux")]
    #[cfg(feature = "kde")]
    fn sync_consumer_no_async_runtime() {
        // Call the actual from_system() entry point.
        // This exercises the pollster::block_on(pipeline::from_system_inner()) path.
        // We don't assert Ok because the test environment may lack KDE config files,
        // but the call must not panic and must return a Result (not hang or deadlock).
        let _result = SystemTheme::from_system();
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_platform_preset_name_kde() {
        let preset = pipeline::linux_preset_for_de(detect::parse_linux_desktop("KDE"));
        assert_eq!(preset.name, "kde-breeze");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_platform_preset_name_gnome() {
        let preset = pipeline::linux_preset_for_de(detect::parse_linux_desktop("GNOME"));
        assert_eq!(preset.name, "adwaita");
    }

    /// §11.2: `from_system()` is an extraction of the reader path, so it must
    /// agree with `SystemTheme::from_system()` wherever both succeed, and it
    /// must never return a non-finite or non-positive text scale.
    #[test]
    fn accessibility_preferences_from_system_is_consistent_with_system_theme() {
        let prefs = AccessibilityPreferences::from_system();
        assert!(prefs.text_scaling_factor.is_finite());
        assert!(prefs.text_scaling_factor > 0.0);

        // CI has no desktop; only compare when the full pipeline also works.
        if let Ok(sys) = SystemTheme::from_system() {
            assert_eq!(
                prefs.text_scaling_factor,
                sys.accessibility.text_scaling_factor
            );
            assert_eq!(prefs.high_contrast, sys.accessibility.high_contrast);
            assert_eq!(
                prefs.reduce_transparency,
                sys.accessibility.reduce_transparency
            );
            // reduce_motion may additionally be true via detect::detect_reduced_motion().
            assert!(prefs.reduce_motion || !sys.accessibility.reduce_motion);
        }
    }
}

// =============================================================================
// Tests -- with_overlay
// =============================================================================

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod overlay_tests {
    use super::*;

    /// Helper: build a SystemTheme from a preset via pipeline::run_pipeline.
    /// Uses test-only Result handling (module has #[allow(clippy::unwrap_used)]).
    fn default_system_theme() -> crate::Result<SystemTheme> {
        let preset = Theme::preset("catppuccin-mocha")?;
        let reader = ReaderResult {
            output: ReaderOutput::Dual {
                light: Box::new(preset.light.clone().unwrap_or_default()),
                dark: Box::new(preset.dark.clone().unwrap_or_default()),
                is_dark: false,
            },
            name: preset.name,
            icon_set: preset.icon_set,
            layout: preset.layout,
            font_dpi: None,
            accessibility: AccessibilityPreferences::default(),
        };
        pipeline::run_pipeline(reader, "catppuccin-mocha", ColorMode::Light)
    }

    /// The range-violation paths of `result`'s error.
    fn violation_paths(result: crate::Result<SystemTheme>) -> Vec<String> {
        let Err(crate::Error::ResolutionInvalid { errors }) = result else {
            panic!("expected ResolutionInvalid, got {:?}", result.err());
        };
        errors.into_iter().map(|rv| rv.path).collect()
    }

    #[test]
    fn test_overlay_error_names_the_variant() -> crate::Result<()> {
        let st = default_system_theme()?;
        let overlay = Theme::from_toml("name = \"overlay\"\n[dark.button]\nmin_height_px = -1.0")?;
        assert_eq!(
            violation_paths(st.with_overlay(&overlay)),
            ["dark.button.min_height_px"]
        );
        Ok(())
    }

    #[test]
    fn test_pipeline_error_names_the_variant() -> crate::Result<()> {
        let preset = Theme::preset("catppuccin-mocha")?;
        let mut light = preset.light.clone().unwrap_or_default();
        light.button.min_height = Some(-1.0);
        let reader = ReaderResult {
            output: ReaderOutput::Dual {
                light: Box::new(light),
                dark: Box::new(preset.dark.clone().unwrap_or_default()),
                is_dark: true,
            },
            name: preset.name,
            icon_set: preset.icon_set,
            layout: preset.layout,
            font_dpi: None,
            accessibility: AccessibilityPreferences::default(),
        };
        assert_eq!(
            violation_paths(pipeline::run_pipeline(
                reader,
                "catppuccin-mocha",
                ColorMode::Dark
            )),
            ["light.button.min_height_px"]
        );
        Ok(())
    }

    #[test]
    fn test_overlay_accent_propagates() -> crate::Result<()> {
        let st = default_system_theme()?;
        let new_accent = Rgba::rgb(255, 0, 0);

        // Build overlay with accent on both light and dark
        let mut overlay = Theme::default();
        let mut light_v = ThemeMode::default();
        light_v.defaults.accent_color = Some(new_accent);
        let mut dark_v = ThemeMode::default();
        dark_v.defaults.accent_color = Some(new_accent);
        overlay.light = Some(light_v);
        overlay.dark = Some(dark_v);

        let result = st.with_overlay(&overlay)?;

        // Accent itself
        assert_eq!(result.light.defaults.accent_color, new_accent);
        // Accent-derived widget fields
        assert_eq!(result.light.button.primary_background, new_accent);
        assert_eq!(result.light.checkbox.checked_background, new_accent);
        assert_eq!(result.light.slider.fill_color, new_accent);
        assert_eq!(result.light.progress_bar.fill_color, new_accent);
        assert_eq!(result.light.switch.checked_background, new_accent);
        // Additional accent-derived fields re-resolved via safety nets
        assert_eq!(
            result.light.spinner.fill_color, new_accent,
            "spinner.fill should re-derive from new accent"
        );
        Ok(())
    }

    #[test]
    fn test_overlay_preserves_unrelated_fields() -> crate::Result<()> {
        let st = default_system_theme()?;
        let original_bg = st.light.defaults.background_color;

        // Apply overlay changing only accent
        let mut overlay = Theme::default();
        let mut light_v = ThemeMode::default();
        light_v.defaults.accent_color = Some(Rgba::rgb(255, 0, 0));
        overlay.light = Some(light_v);

        let result = st.with_overlay(&overlay)?;
        assert_eq!(
            result.light.defaults.background_color, original_bg,
            "background should be unchanged"
        );
        Ok(())
    }

    #[test]
    fn test_overlay_empty_noop() -> crate::Result<()> {
        let st = default_system_theme()?;
        let original_light_accent = st.light.defaults.accent_color;
        let original_dark_accent = st.dark.defaults.accent_color;
        let original_light_bg = st.light.defaults.background_color;

        // Empty overlay
        let overlay = Theme::default();
        let result = st.with_overlay(&overlay)?;

        assert_eq!(result.light.defaults.accent_color, original_light_accent);
        assert_eq!(result.dark.defaults.accent_color, original_dark_accent);
        assert_eq!(result.light.defaults.background_color, original_light_bg);
        Ok(())
    }

    #[test]
    fn test_overlay_both_variants() -> crate::Result<()> {
        let st = default_system_theme()?;
        let red = Rgba::rgb(255, 0, 0);
        let green = Rgba::rgb(0, 255, 0);

        let mut overlay = Theme::default();
        let mut light_v = ThemeMode::default();
        light_v.defaults.accent_color = Some(red);
        let mut dark_v = ThemeMode::default();
        dark_v.defaults.accent_color = Some(green);
        overlay.light = Some(light_v);
        overlay.dark = Some(dark_v);

        let result = st.with_overlay(&overlay)?;
        assert_eq!(
            result.light.defaults.accent_color, red,
            "light accent = red"
        );
        assert_eq!(
            result.dark.defaults.accent_color, green,
            "dark accent = green"
        );
        Ok(())
    }

    #[test]
    fn test_overlay_font_family() -> crate::Result<()> {
        let st = default_system_theme()?;

        let mut overlay = Theme::default();
        let mut light_v = ThemeMode::default();
        light_v.defaults.font.family = Some("Comic Sans".into());
        overlay.light = Some(light_v);

        let result = st.with_overlay(&overlay)?;
        assert_eq!(result.light.defaults.font.family.as_ref(), "Comic Sans");
        Ok(())
    }

    #[test]
    fn test_overlay_roundtrip_via_overlay_source() -> crate::Result<()> {
        let st = default_system_theme()?;
        // Apply overlay and verify accent propagates
        let new_accent = Rgba::rgb(255, 0, 0);
        let mut overlay = Theme::default();
        let mut light_v = ThemeMode::default();
        light_v.defaults.accent_color = Some(new_accent);
        overlay.light = Some(light_v);

        let result = st.with_overlay(&overlay)?;
        assert_eq!(result.light.defaults.accent_color, new_accent);
        // The dark variant should be unchanged from original
        assert_eq!(
            result.dark.defaults.accent_color,
            st.dark.defaults.accent_color
        );
        Ok(())
    }

    #[test]
    fn test_overlay_source_no_variant_fields() -> crate::Result<()> {
        // Verify overlay_source exists on SystemTheme (compile-time structural check).
        // If light_variant or dark_variant fields still existed, this test would
        // need updating -- documenting the structural change.
        let st = default_system_theme()?;
        let _ = &st.overlay_source; // overlay_source exists
        Ok(())
    }
}

// =============================================================================
// Tests -- AccessibilityPreferences text scaling
// =============================================================================

#[cfg(test)]
mod accessibility_tests {
    use super::*;

    fn prefs(text_scaling_factor: f32) -> AccessibilityPreferences {
        AccessibilityPreferences {
            text_scaling_factor,
            ..AccessibilityPreferences::default()
        }
    }

    #[test]
    fn text_scale_factor_keeps_a_finite_positive_factor() {
        assert_eq!(prefs(2.0).text_scale_factor(), 2.0);
        assert_eq!(prefs(2.0).scaled(10.0), 20.0);
        assert_eq!(AccessibilityPreferences::default().text_scale_factor(), 1.0);
    }

    #[test]
    fn text_scale_factor_ignores_a_factor_that_is_not_finite_and_positive() {
        for f in [f32::NAN, 0.0, -0.0, -1.0, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(prefs(f).text_scale_factor(), 1.0, "{f}");
            assert_eq!(prefs(f).scaled(10.0), 10.0, "{f}");
        }
    }
}
