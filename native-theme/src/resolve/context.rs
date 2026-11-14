//! Resolution-time inputs (font DPI, button order, icon theme) captured
//! once per theme-build and passed by reference through the pipeline.
//!
//! Per `docs/archive/v0.5.7_gaps.md` §G7:
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
/// render-time concern, not a resolve-time concern
/// (`docs/archive/v0.5.7_gaps.md` §G7).
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
    /// Font DPI for pt-to-px conversion. It must be finite and positive:
    /// resolution rejects any other value as one
    /// [`Error::ResolutionInvalid`](crate::Error::ResolutionInvalid) on the
    /// path `context.font_dpi`, before any point size is converted.
    ///
    /// [`from_system`](Self::from_system) gives the platform's: 72.0 on macOS,
    /// where a point is a pixel, and 96.0 everywhere else, every Linux desktop
    /// included. Nothing is detected or cached. On Linux the toolkit's scale
    /// factor carries the display's scale (from the compositor on Wayland, from
    /// `Xft.dpi` on X11), so the font DPI carries none of it; GNOME's
    /// text-scaling factor is carried in
    /// [`AccessibilityPreferences`](crate::AccessibilityPreferences). When the OS
    /// theme is read ([`SystemTheme::from_system`](crate::SystemTheme::from_system)),
    /// the reader's own DPI, the same value, replaces it.
    ///
    /// No font DPI is right where the application, not the toolkit, decides the
    /// X11 scale (`GPUI_X11_SCALE_FACTOR`, `WINIT_X11_SCALE_FACTOR`); on X11
    /// without `Xft.dpi`, where gpui and winit scale the whole interface by the
    /// monitor's size (gpui-fast by `GDK_SCALE` where the session sets it) and Qt
    /// does not (text and interface stay in proportion, at the toolkit's size);
    /// on Plasma 6.7 X11 with a global scale other than 1
    /// and a forced font DPI that departs from it, where Qt takes its scale from
    /// `QT_SCREEN_SCALE_FACTORS` and gpui and winit from `Xft.dpi`; and for a
    /// toolkit that does not scale by `Xft.dpi` on X11 (none of the three
    /// connectors'). Such an application builds its own context for the preset
    /// and TOML paths ([`Theme::resolve_with`](crate::Theme::resolve_with),
    /// [`ThemeMode::into_resolved`](crate::theme::ThemeMode::into_resolved));
    /// [`SystemTheme::from_system`](crate::SystemTheme::from_system) takes no font
    /// DPI, so on that path it re-derives its sizes from each resolved font's
    /// [`defined_size`](crate::theme::ResolvedFontSpec::defined_size) (the text
    /// scale's entries carry no `defined_size`, so their sizes cannot be
    /// re-derived).
    pub font_dpi: f32,
    /// Dialog button ordering (`PrimaryLeft` on KDE and Windows,
    /// `PrimaryRight` elsewhere).
    pub button_order: DialogButtonOrder,
    /// Runtime-detected icon theme name, used when the preset and
    /// per-variant `icon_theme` fields are both `None`. Three-tier
    /// precedence in the pipeline: per-variant → `Theme`-level shared →
    /// this runtime detection.
    ///
    /// [`from_system`](Self::from_system) puts the detected system icon
    /// theme here, or `None` where detection failed;
    /// [`system_icon_theme()`](crate::detect::system_icon_theme) gives the
    /// reason.
    pub icon_theme: Option<Cow<'static, str>>,
}

impl ResolutionContext {
    /// Build the context by auto-detecting from the current OS.
    ///
    /// Calls:
    /// - `crate::detect::system_font_dpi` for `font_dpi` (private)
    /// - `crate::resolve::inheritance::platform_button_order` for
    ///   `button_order`
    /// - [`crate::detect::system_icon_theme`] for `icon_theme`, which
    ///   is `None` where detection fails
    #[must_use]
    pub fn from_system() -> Self {
        Self::with_detected_icon_theme(crate::detect::system_icon_theme())
    }

    /// [`from_system`](Self::from_system), with `detected` as the outcome
    /// of the icon theme's detection.
    pub(crate) fn with_detected_icon_theme(detected: crate::Result<String>) -> Self {
        Self::with_font_dpi(crate::detect::system_font_dpi(), detected)
    }

    /// [`from_system`](Self::from_system) with a `font_dpi` already known,
    /// such as the one a platform reader reported.
    /// `detected` is the outcome of the icon theme's detection.
    pub(crate) fn with_font_dpi(font_dpi: f32, detected: crate::Result<String>) -> Self {
        Self {
            font_dpi,
            button_order: crate::resolve::inheritance::platform_button_order(),
            icon_theme: detected.ok().map(Cow::Owned),
        }
    }

    /// Deterministic values for tests: 96 DPI, `PrimaryRight` button
    /// order, no detected `icon_theme`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_font_dpi_is_its_arguments() {
        // The pipeline's constructor when the reader supplied a DPI: the
        // context holds that DPI and the given icon-theme outcome, plus
        // the platform's button order; no DPI detection replaces either.
        let ctx = ResolutionContext::with_font_dpi(
            123.3,
            Err(crate::Error::PlatformUnsupported {
                platform: "the test's failing detection",
            }),
        );
        assert_eq!(ctx.font_dpi, 123.3);
        assert!(ctx.icon_theme.is_none());
        assert_eq!(
            ctx.button_order,
            crate::resolve::inheritance::platform_button_order()
        );

        let ctx = ResolutionContext::with_font_dpi(144.0, Ok("detected-theme".to_string()));
        assert_eq!(ctx.font_dpi, 144.0);
        assert_eq!(ctx.icon_theme.as_deref(), Some("detected-theme"));
    }
}
