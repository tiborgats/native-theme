// Resolution engine: resolve() fills inheritance rules, validate() produces ResolvedThemeVariant.
//
// Split into submodules:
// - inheritance: Phase 1-5 resolution rules (fill None fields from defaults/other widgets)
// - validate: Field extraction, range checks, ResolvedThemeVariant construction

mod inheritance;
mod validate;

use crate::model::ThemeVariant;
use crate::model::resolved::ResolvedThemeVariant;

impl ThemeVariant {
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
    /// 1.5. **Font DPI conversion** -- pt->px using font_dpi.
    /// 2. **Safety nets** -- platform-divergent fields get a reasonable fallback.
    /// 3. **Widget-from-defaults** -- colors, geometry, fonts, text scale entries
    ///    all inherit from defaults.
    /// 4. **Widget-to-widget** -- inactive title bar fields fall back to active.
    /// 5. **Icon set** -- fills icon_set from the compile-time system default.
    pub fn resolve(&mut self) {
        self.resolve_defaults_internal();
        self.resolve_font_dpi_conversion(); // Phase 1.5: pt->px using font_dpi
        self.resolve_safety_nets();
        self.resolve_widgets_from_defaults();
        self.resolve_widget_to_widget();

        // Phase 5: icon_set fallback — fill from system default if not set
        if self.icon_set.is_none() {
            self.icon_set = Some(crate::model::icons::system_icon_set());
        }
    }

    /// Fill platform-detected defaults that require OS interaction.
    ///
    /// Currently fills `icon_theme` from the system icon theme if not already set.
    /// This is separated from [`resolve()`](Self::resolve) because it performs
    /// runtime OS detection (reading desktop environment settings), unlike the
    /// pure inheritance rules in resolve().
    pub fn resolve_platform_defaults(&mut self) {
        if self.icon_theme.is_none() {
            self.icon_theme = Some(crate::model::icons::system_icon_theme().to_string());
        }
    }

    /// Apply all inheritance rules and platform defaults.
    ///
    /// Convenience method that calls [`resolve()`](Self::resolve) followed by
    /// [`resolve_platform_defaults()`](Self::resolve_platform_defaults).
    /// This is equivalent to the full resolution that
    /// [`into_resolved()`](Self::into_resolved) performs before validation.
    pub fn resolve_all(&mut self) {
        self.resolve();
        self.resolve_platform_defaults();
    }

    /// Resolve all inheritance rules and validate in one step.
    ///
    /// This is the recommended way to convert a `ThemeVariant` into a
    /// [`ResolvedThemeVariant`]. It calls [`resolve_all()`](Self::resolve_all)
    /// followed by [`validate()`](Self::validate), ensuring no fields are left
    /// unresolved.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Resolution`] if any fields remain `None` after
    /// resolution (e.g., when accent color is missing and cannot be derived).
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::ThemeSpec;
    ///
    /// let theme = ThemeSpec::preset("dracula").unwrap();
    /// let variant = theme.dark.unwrap();
    /// let resolved = variant.into_resolved().unwrap();
    /// // All fields are now guaranteed populated
    /// let accent = resolved.defaults.accent_color;
    /// ```
    #[must_use = "this returns the resolved theme and consumes self"]
    pub fn into_resolved(mut self) -> crate::Result<ResolvedThemeVariant> {
        self.resolve_all();
        self.validate()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests;
