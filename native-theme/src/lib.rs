//! # native-theme
//!
//! Cross-platform native theme detection and loading for Rust GUI applications.
//!
//! Any Rust GUI app can look native on any platform by loading a single theme
//! file or reading live OS settings, without coupling to any specific toolkit.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// Generates `merge()` and `is_empty()` methods for theme structs.
///
/// Four field categories:
/// - `option { field1, field2, ... }` -- `Option<T>` leaf fields
/// - `soft_option { field1, field2, ... }` -- `Option<T>` leaf fields (same merge/is_empty as option)
/// - `nested { field1, field2, ... }` -- nested struct fields with their own `merge()`
/// - `optional_nested { field1, field2, ... }` -- `Option<T>` where T has its own `merge()`
///
/// For `option` and `soft_option` fields, `Some` values in the overlay replace the
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
            $(soft_option { $($so_field:ident),* $(,)? })?
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
                    if overlay.$so_field.is_some() {
                        self.$so_field = overlay.$so_field.clone();
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
                $($(&& self.$so_field.is_none())*)?
                $($(&& self.$nest_field.is_empty())*)?
                $($(&& self.$on_field.as_ref().map_or(true, |v| v.is_empty()))*)*
            }
        }
    };
}

/// Color types and sRGB utilities.
pub mod color;
/// OS detection: dark mode, reduced motion, DPI, desktop environment.
pub mod detect;
/// Error types for theme operations.
pub mod error;
/// GNOME portal theme reader.
#[cfg(all(target_os = "linux", feature = "portal"))]
pub mod gnome;
/// Icon loading and dispatch.
pub mod icons;
/// KDE theme reader.
#[cfg(all(target_os = "linux", feature = "kde"))]
pub mod kde;
/// Theme data model types.
pub mod model;
/// Theme pipeline: reader -> preset merge -> resolve -> validate.
pub mod pipeline;
/// Bundled theme presets.
pub mod presets;
/// Theme resolution engine (inheritance + validation).
mod resolve;
#[cfg(any(
    feature = "material-icons",
    feature = "lucide-icons",
    feature = "system-icons"
))]
mod spinners;
/// Runtime theme change watching.
#[cfg(feature = "watch")]
pub mod watch;

/// Convenience re-exports for common usage.
///
/// `use native_theme::prelude::*` imports:
/// [`Theme`](theme::Theme), [`ResolvedTheme`](theme::ResolvedTheme),
/// [`SystemTheme`], [`AccessibilityPreferences`],
/// [`Rgba`](color::Rgba), [`Error`](error::Error), and [`Result`].
pub mod prelude;

/// Theme data model: types, defaults, fonts, borders, widgets.
///
/// Core types: [`Theme`], [`ThemeMode`],
/// [`ResolvedTheme`], [`ResolvedDefaults`].
///
/// Re-exports from the internal model module.
pub mod theme {
    pub use crate::model::*;
    pub use crate::presets::PresetInfo;
}

/// Freedesktop icon theme lookup (Linux).
#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub mod freedesktop;
/// macOS platform helpers.
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(not(target_os = "macos"))]
pub(crate) mod macos;
/// SVG-to-RGBA rasterization utilities.
#[cfg(feature = "svg-rasterize")]
pub mod rasterize;
/// SF Symbols icon loader (macOS).
#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub mod sficons;
/// Windows platform theme reader.
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(not(target_os = "windows"))]
#[allow(dead_code, unused_variables)]
pub(crate) mod windows;
/// Windows Segoe Fluent / stock icon loader.
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub mod winicons;
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
pub(crate) use color::{ParseColorError, Rgba};
#[cfg(target_os = "linux")]
#[allow(unused_imports)]
pub(crate) use detect::LinuxDesktop;
#[cfg(target_os = "linux")]
#[allow(unused_imports)]
pub(crate) use detect::detect_linux_desktop;
#[allow(unused_imports)]
pub(crate) use detect::{
    detect_is_dark, detect_reduced_motion, invalidate_caches, prefers_reduced_motion,
    system_is_dark,
};
#[allow(unused_imports)]
pub(crate) use error::{Error, ErrorKind, RangeViolation};
#[allow(unused_imports)]
pub(crate) use icons::{IconId, IconLoader, is_freedesktop_theme_available};
pub use icons::{IconSetChoice, default_icon_choice, list_freedesktop_themes};
#[allow(unused_imports)]
pub(crate) use model::icons::{detect_icon_theme, icon_name, system_icon_set, system_icon_theme};
#[allow(unused_imports)]
pub(crate) use model::{
    AnimatedIcon, ButtonTheme, CardTheme, CheckboxTheme, ColorMode, ComboBoxTheme,
    DefaultsBorderSpec, DialogButtonOrder, DialogTheme, ExpanderTheme, FontSize, FontSpec,
    FontStyle, IconData, IconProvider, IconRole, IconSet, IconSizes, InputTheme, LayoutTheme,
    LinkTheme, ListTheme, MenuTheme, PopoverTheme, ProgressBarTheme, ResolvedBorderSpec,
    ResolvedDefaults, ResolvedFontSpec, ResolvedIconSizes, ResolvedTextScale,
    ResolvedTextScaleEntry, ResolvedTheme, ScrollbarTheme, SegmentedControlTheme, SeparatorTheme,
    SidebarTheme, SliderTheme, SpinnerTheme, SplitterTheme, StatusBarTheme, SwitchTheme, TabTheme,
    TextScale, TextScaleEntry, Theme, ThemeDefaults, ThemeMode, ToolbarTheme, TooltipTheme,
    TransformAnimation, WidgetBorderSpec, WindowTheme, bundled_icon_by_name, bundled_icon_svg,
};
pub use pipeline::{DiagnosticEntry, PlatformPreset};
#[allow(unused_imports)]
pub(crate) use pipeline::{diagnose_platform_support, platform_preset_name};

/// OS-detected accessibility preferences.
///
/// A single copy lives on [`SystemTheme`], shared across light and dark
/// variants. These are runtime values detected from the OS -- not stored
/// in TOML presets.
#[derive(Clone, Debug, PartialEq)]
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
        Self {
            text_scaling_factor: 1.0,
            reduce_motion: false,
            high_contrast: false,
            reduce_transparency: false,
        }
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
#[derive(Clone, Debug)]
pub(crate) enum ReaderOutput {
    /// Reader provides only the OS-active variant. The pipeline fills the
    /// inactive variant from the platform preset.
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
            ReaderOutput::Dual { light, dark } => {
                (Some(ThemeMode::clone(light)), Some(ThemeMode::clone(dark)))
            }
        };
        Theme {
            name: std::borrow::Cow::Owned(name.to_string()),
            light,
            dark,
            layout: layout.clone(),
            icon_set,
        }
    }
}

/// Data needed to replay the merge+resolve pipeline for overlay support.
///
/// Stores the original reader output and preset name so that
/// [`SystemTheme::with_overlay()`] can reconstruct pre-resolve variants
/// on demand instead of storing ~2KB of ThemeMode clones.
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
    /// The live preset name (e.g. "kde-breeze-live").
    pub(crate) preset_name: String,
    /// Font DPI captured at detection time (None = auto-detect).
    pub(crate) font_dpi: Option<f32>,
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
    pub name: Cow<'static, str>,
    /// The OS color mode preference (light or dark).
    pub mode: ColorMode,
    /// Resolved light variant (always populated).
    pub light: ResolvedTheme,
    /// Resolved dark variant (always populated).
    pub dark: ResolvedTheme,
    /// Data for replaying the pipeline on overlay (replaces light_variant/dark_variant).
    pub(crate) overlay_source: OverlaySource,
    /// The platform preset used (e.g., "kde-breeze", "adwaita", "macos-sonoma").
    pub preset: String,
    /// The live preset name used internally (e.g., "kde-breeze-live").
    pub(crate) live_preset: String,
    /// Which icon loading mechanism to use for this theme.
    pub icon_set: IconSet,
    /// The name of the visual icon theme (e.g. `"breeze"`, `"Adwaita"`).
    pub icon_theme: Cow<'static, str>,
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

    /// Apply an app-level TOML overlay and re-resolve.
    ///
    /// Merges the overlay onto the pre-resolve [`ThemeMode`] (not the
    /// already-resolved [`ResolvedTheme`]) so that changed source fields
    /// propagate correctly through `resolve()`. For example, changing
    /// `defaults.accent_color` in the overlay will cause `button.primary_background`,
    /// `checkbox.checked_background`, `slider.fill`, etc. to be re-derived from
    /// the new accent color.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let system = native_theme::SystemTheme::from_system()?;
    /// let overlay = native_theme::theme::Theme::from_toml(r##"
    ///     [light.defaults]
    ///     accent_color = "#ff6600"
    ///     [dark.defaults]
    ///     accent_color = "#ff6600"
    /// "##)?;
    /// let customized = system.with_overlay(&overlay)?;
    /// // customized.pick(customized.mode).defaults.accent_color is now #ff6600
    /// // and all accent-derived fields are updated
    /// # Ok::<(), native_theme::error::Error>(())
    /// ```
    pub fn with_overlay(&self, overlay: &Theme) -> crate::Result<Self> {
        // Reconstruct pre-resolve variants from overlay_source
        let src = &self.overlay_source;
        let live_preset = Theme::preset(&src.preset_name)?;
        let full_preset_name = src
            .preset_name
            .strip_suffix("-live")
            .unwrap_or(&src.preset_name);
        let full_preset = Theme::preset(full_preset_name)?;

        // Reconstruct a Theme from the type-safe ReaderOutput for merge
        let reader_as_theme = src
            .reader_output
            .to_theme(&src.name, src.icon_set, &src.layout);

        let mut merged = full_preset.clone();
        merged.merge(&live_preset);
        merged.merge(&reader_as_theme);

        // Match on ReaderOutput for type-safe variant selection
        let (mut light, mut dark) = match &src.reader_output {
            ReaderOutput::Single { is_dark, .. } => {
                if *is_dark {
                    (
                        full_preset.light.unwrap_or_default(),
                        merged.dark.unwrap_or_default(),
                    )
                } else {
                    (
                        merged.light.unwrap_or_default(),
                        full_preset.dark.unwrap_or_default(),
                    )
                }
            }
            ReaderOutput::Dual { .. } => (
                merged.light.unwrap_or_default(),
                merged.dark.unwrap_or_default(),
            ),
        };

        // Apply the user overlay on top
        if let Some(over) = &overlay.light {
            light.merge(over);
        }
        if let Some(over) = &overlay.dark {
            dark.merge(over);
        }

        // Re-resolve both variants
        let resolved_light = light.into_resolved(src.font_dpi)?;
        let resolved_dark = dark.into_resolved(src.font_dpi)?;

        Ok(SystemTheme {
            name: self.name.clone(),
            mode: self.mode,
            light: resolved_light,
            dark: resolved_dark,
            overlay_source: self.overlay_source.clone(),
            live_preset: self.live_preset.clone(),
            preset: self.preset.clone(),
            icon_set: self.icon_set,
            icon_theme: self.icon_theme.clone(),
            accessibility: self.accessibility.clone(),
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
    /// # Platform Behavior
    ///
    /// - **macOS:** Calls `from_macos()` when the `macos` feature is enabled.
    ///   Reads both light and dark variants via NSAppearance, merges with
    ///   `macos-sonoma` preset.
    /// - **Linux:** Uses `pollster::block_on` to drive the async inner
    ///   implementation, which handles portal D-Bus calls when the `portal`
    ///   feature is enabled.
    /// - **Windows:** Calls `from_windows()` when the `windows` feature is enabled,
    ///   merges with `windows-11` preset.
    /// - **Other platforms:** Returns `Error::PlatformUnsupported`.
    ///
    /// # Errors
    ///
    /// - `Error::FeatureDisabled` if the platform has a reader but the required feature
    ///   is not enabled.
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
    #[cfg(target_os = "linux")]
    pub fn from_system() -> crate::Result<Self> {
        pollster::block_on(pipeline::from_system_inner())
    }

    /// Load the OS theme synchronously (non-Linux).
    ///
    /// On macOS and Windows the async inner has zero `.await` points, so a
    /// noop-waker single-poll is sufficient -- no async runtime needed.
    #[cfg(not(target_os = "linux"))]
    pub fn from_system() -> crate::Result<Self> {
        let waker = std::task::Waker::noop();
        let mut cx = std::task::Context::from_waker(&waker);
        let mut fut = std::pin::pin!(pipeline::from_system_inner());
        match fut.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(result) => result,
            std::task::Poll::Pending => Err(crate::Error::PlatformUnsupported {
                platform: "unexpected async suspension",
            }),
        }
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
                },
                name: Cow::Borrowed(""),
                icon_set: None,
                layout: LayoutTheme::default(),
                preset_name: "catppuccin-mocha".into(),
                font_dpi: None,
            },
            live_preset: "catppuccin-mocha".into(),
            preset: "catppuccin-mocha".into(),
            icon_set: IconSet::Lucide,
            icon_theme: "lucide".into(),
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
                },
                name: Cow::Borrowed(""),
                icon_set: None,
                layout: LayoutTheme::default(),
                preset_name: "catppuccin-mocha".into(),
                font_dpi: None,
            },
            live_preset: "catppuccin-mocha".into(),
            preset: "catppuccin-mocha".into(),
            icon_set: IconSet::Lucide,
            icon_theme: "lucide".into(),
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
                },
                name: Cow::Borrowed(""),
                icon_set: None,
                layout: LayoutTheme::default(),
                preset_name: "catppuccin-mocha".into(),
                font_dpi: None,
            },
            live_preset: "catppuccin-mocha".into(),
            preset: "catppuccin-mocha".into(),
            icon_set: IconSet::Lucide,
            icon_theme: "lucide".into(),
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
        assert!(preset.is_live);
        assert_eq!(preset.live_name(), "kde-breeze-live");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_platform_preset_name_gnome() {
        let preset = pipeline::linux_preset_for_de(detect::parse_linux_desktop("GNOME"));
        assert_eq!(preset.name, "adwaita");
        assert!(preset.is_live);
        assert_eq!(preset.live_name(), "adwaita-live");
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
            },
            name: preset.name,
            icon_set: preset.icon_set,
            layout: preset.layout,
            font_dpi: None,
            accessibility: AccessibilityPreferences::default(),
        };
        pipeline::run_pipeline(reader, "catppuccin-mocha", ColorMode::Light)
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
