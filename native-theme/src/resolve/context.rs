//! Resolution-time inputs (font DPI, button order, icon theme) captured
//! once per theme-build and passed by reference through the pipeline.
//!
//! Per docs/todo_v0.5.7_gaps.md §G7 / doc 2 §J.2 refinement on B5:
//! intentionally NO `impl Default` — runtime-detected types must signal
//! intent at the call site. Use [`ResolutionContext::from_system`] for
//! production, or [`ResolutionContext::for_tests`] for deterministic
//! test values.

use std::borrow::Cow;

use crate::model::DialogButtonOrder;

/// Resolution-time inputs captured once per theme-build.
///
/// Replaces the `font_dpi: Option<f32>` parameter on
/// [`ThemeMode::into_resolved`](crate::theme::ThemeMode::into_resolved) and
/// the corresponding internal `OverlaySource` field.
///
/// Accessibility preferences live on
/// [`SystemTheme`](crate::SystemTheme), NOT here — accessibility is a
/// render-time concern, not a resolve-time concern. See doc 2 §J.2
/// refinement on B4 for the rationale.
///
/// # Examples
///
/// ```
/// use native_theme::resolve::ResolutionContext;
///
/// // Production: auto-detect from OS.
/// let ctx = ResolutionContext::from_system();
/// assert!(ctx.font_dpi > 0.0);
///
/// // Tests: deterministic values.
/// let ctx = ResolutionContext::for_tests();
/// assert_eq!(ctx.font_dpi, 96.0);
/// assert!(ctx.icon_theme.is_none());
/// ```
#[derive(Clone, Debug)]
pub struct ResolutionContext {
    /// Font DPI for pt-to-px conversion. 96.0 on Linux/Windows, 72.0 on
    /// macOS, or reader-supplied for KDE `forceFontDPI`.
    pub font_dpi: f32,
    /// Dialog button ordering (`PrimaryLeft` on KDE, `PrimaryRight`
    /// elsewhere).
    pub button_order: DialogButtonOrder,
    /// Fallback icon theme name used when the preset and per-variant
    /// `icon_theme` fields are both `None`. Three-tier precedence in the
    /// pipeline: per-variant → `Theme`-level shared → this fallback.
    pub icon_theme: Option<Cow<'static, str>>,
}

impl ResolutionContext {
    /// Build the context by auto-detecting from the current OS.
    ///
    /// Calls:
    /// - [`crate::detect::system_font_dpi`] for `font_dpi`
    /// - `crate::resolve::inheritance::platform_button_order` for
    ///   `button_order`
    /// - [`crate::model::icons::system_icon_theme`] for `icon_theme`
    #[must_use]
    pub fn from_system() -> Self {
        Self {
            font_dpi: crate::detect::system_font_dpi(),
            button_order: crate::resolve::inheritance::platform_button_order(),
            icon_theme: Some(Cow::Owned(crate::model::icons::system_icon_theme())),
        }
    }

    /// Deterministic values for tests: 96 DPI, `PrimaryRight` button
    /// order, no `icon_theme` fallback.
    ///
    /// Tests that need a specific DPI (e.g. 72.0 for Apple point→pixel)
    /// can construct the struct via a literal:
    /// `ResolutionContext { font_dpi: 72.0, ..ResolutionContext::for_tests() }`.
    #[must_use]
    pub fn for_tests() -> Self {
        Self {
            font_dpi: 96.0,
            button_order: DialogButtonOrder::PrimaryRight,
            icon_theme: None,
        }
    }
}

// Intentionally NO `impl Default for ResolutionContext`.
// See module-level doc comment for the signal-intent rationale.
