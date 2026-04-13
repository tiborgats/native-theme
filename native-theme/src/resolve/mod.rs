// Resolution engine: three-stage pipeline.
//
// 1. resolve() -- pure data transform: fills None fields from defaults and
//    related widgets via ~91 inheritance rules. No OS detection, no I/O.
// 2. resolve_platform_defaults() -- fills fields that require OS detection:
//    icon_theme (from system icon settings) and button_order (from detected
//    desktop environment).
// 3. validate() -- extracts Option<T> -> T, producing ResolvedTheme.
//
// Convenience: resolve_all() = 1+2, into_resolved() = 1+2+3.
//
// Split into submodules:
// - inheritance: Phase 1-5 resolution rules (fill None fields from defaults/other widgets)
// - validate: Field extraction, range checks, ResolvedTheme construction

mod inheritance;
pub(crate) mod validate;
pub(crate) mod validate_helpers;

use crate::model::ThemeMode;
use crate::model::resolved::ResolvedTheme;

impl ThemeMode {
    /// Apply all ~91 inheritance rules in 5-phase order (pure data transform).
    ///
    /// After calling resolve(), most Option fields that were None will be filled
    /// from defaults or related widget fields. Calling resolve() twice produces
    /// the same result (idempotent).
    ///
    /// This method is a pure data transform: it does not perform any OS detection
    /// or I/O. For full resolution including platform defaults (icon theme from
    /// the system), use [`resolve_all()`](Self::resolve_all).
    ///
    /// # Phases
    ///
    /// 1. **Defaults internal chains** -- accent derives selection, focus_ring_color;
    ///    selection derives selection_inactive.
    /// 2. **Safety nets** -- platform-divergent fields get a reasonable fallback.
    /// 3. **Widget-from-defaults** -- colors, geometry, fonts, text scale entries
    ///    all inherit from defaults.
    /// 4. **Widget-to-widget** -- inactive title bar fields fall back to active.
    pub fn resolve(&mut self) {
        self.resolve_defaults_internal();
        self.resolve_safety_nets();
        self.resolve_widgets_from_defaults();
        self.resolve_widget_to_widget();
    }

    /// Fill platform-detected defaults that require OS interaction.
    ///
    /// Currently fills:
    /// - `dialog.button_order` from the detected desktop environment if not already set
    ///
    /// This is separated from [`resolve()`](Self::resolve) because it performs
    /// runtime OS detection (reading desktop environment settings), unlike the
    /// pure inheritance rules in resolve().
    ///
    /// Note: `icon_set` and `icon_theme` resolution is handled at the
    /// [`Theme`](crate::Theme) / pipeline level, not per-variant.
    pub fn resolve_platform_defaults(&mut self) {
        if self.dialog.button_order.is_none() {
            self.dialog.button_order = Some(inheritance::platform_button_order());
        }
    }

    /// Apply all inheritance rules and platform defaults.
    ///
    /// Convenience method that calls [`resolve()`](Self::resolve) followed by
    /// [`resolve_platform_defaults()`](Self::resolve_platform_defaults).
    ///
    /// **Note:** this does *not* handle `font_dpi`. Pass the DPI value to
    /// [`validate_with_dpi()`](Self::validate_with_dpi) or use
    /// [`into_resolved()`](Self::into_resolved) which accepts an optional
    /// DPI parameter.
    pub fn resolve_all(&mut self) {
        self.resolve();
        self.resolve_platform_defaults();
    }

    /// Resolve all inheritance rules and validate in one step.
    ///
    /// This is the recommended way to convert a `ThemeMode` into a
    /// [`ResolvedTheme`]. It calls [`resolve_all()`](Self::resolve_all)
    /// followed by [`validate_with_dpi()`](Self::validate_with_dpi),
    /// ensuring no fields are left unresolved.
    ///
    /// # Arguments
    ///
    /// * `font_dpi` -- Font DPI for pt-to-px conversion. Pass `None` to
    ///   auto-detect from the OS (typically 96 on Linux/Windows, 72 on macOS).
    ///   OS readers pass `Some(detected_dpi)` so standalone preset loading
    ///   applies the correct conversion.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::ResolutionIncomplete`] if any fields remain `None`
    /// after resolution, or [`crate::Error::ResolutionInvalid`] if range checks fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::Theme;
    ///
    /// let theme = Theme::preset("dracula")?;
    /// let variant = theme.dark.ok_or("no dark variant")?;
    /// let resolved = variant.into_resolved(None)?;
    /// // All fields are now guaranteed populated
    /// let accent = resolved.defaults.accent_color;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_resolved(mut self, font_dpi: Option<f32>) -> crate::Result<ResolvedTheme> {
        let dpi = font_dpi.unwrap_or_else(crate::detect::system_font_dpi);
        self.resolve_all();
        self.validate_with_dpi(dpi)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests;
