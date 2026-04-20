// Resolution engine: three-stage pipeline.
//
// 1. resolve() -- pure data transform: fills None fields from defaults and
//    related widgets via ~91 inheritance rules. No OS detection, no I/O.
// 2. resolve_platform_defaults() -- fills fields that require OS detection:
//    button_order (from detected desktop environment).
// 3. validate() -- extracts Option<T> -> T, producing ResolvedTheme.
//
// Convenience: resolve_all() = 1+2, into_resolved() = 1+2+3.
//
// Split into submodules:
// - inheritance: Phase 1-5 resolution rules (fill None fields from defaults/other widgets)
// - validate: Field extraction, range checks, ResolvedTheme construction

pub(crate) mod context;
pub(crate) mod inheritance;
pub(crate) mod validate;
pub(crate) mod validate_helpers;

pub use context::ResolutionContext;

/// Widget field metadata for TOML linting. Populated by `#[derive(ThemeWidget)]`.
pub(crate) struct WidgetFieldInfo {
    /// Snake_case widget name (e.g., "button", "segmented_control").
    pub widget_name: &'static str,
    /// All serialized TOML field names for this widget.
    pub field_names: &'static [&'static str],
}
inventory::collect!(WidgetFieldInfo);

/// Non-widget (plain struct) field metadata for TOML linting.
///
/// Populated by `#[derive(ThemeFields)]` on plain structs like `FontSpec`,
/// `IconSizes`, `ThemeDefaults`, etc. Consumed by `lint_toml` to detect
/// unknown keys in sub-tables. See `docs/todo_v0.5.7_gaps.md` §G5 for
/// rationale — this sister registry to `WidgetFieldInfo` eliminates the
/// hand-authored `FIELD_NAMES` constants that previously duplicated
/// struct field lists.
pub(crate) struct FieldInfo {
    /// Pascal-case type name (e.g. "FontSpec", "IconSizes", "ThemeDefaults").
    pub struct_name: &'static str,
    /// All serialized TOML field names for this struct.
    pub field_names: &'static [&'static str],
}
inventory::collect!(FieldInfo);

/// Border inheritance registry (Phase 94-01 G6).
///
/// Populated by `#[derive(ThemeWidget)]` for widgets that declare
/// `#[theme_inherit(border_kind = "full" | "full_lg" | "partial")]`. Each
/// declaration produces one row in this registry.
///
/// Consumed by the inverted drift test
/// `border_inheritance_toml_matches_macro_emit` in `inheritance.rs::tests`
/// which asserts `docs/inheritance-rules.toml [border_inheritance]` matches
/// the registry byte-for-byte (macro is the source of truth post-G6;
/// TOML is a generated-documentation output).
///
/// Sister registry to [`WidgetFieldInfo`] / [`FieldInfo`] — same
/// inventory-crate pattern (`inventory::submit!` from the derive output).
///
/// Fields carry `#[cfg_attr(not(test), allow(dead_code))]` because they are
/// consumed only by the drift tests in `inheritance.rs::tests`
/// (introspection via `inventory::iter`). A non-test build compiles the
/// registry rows but never reads them — matching Phase 93-09's conditional
/// `allow(dead_code)` pattern for self-unmasking (if a future non-test
/// consumer lands, the allow stops firing and real dead-code regressions
/// surface).
pub(crate) struct BorderInheritanceInfo {
    /// Snake_case widget name (e.g., "button", "segmented_control").
    #[cfg_attr(not(test), allow(dead_code))]
    pub widget_name: &'static str,
    /// Inheritance kind: `"full"`, `"full_lg"`, or `"partial"`.
    ///
    /// Uses a plain `&'static str` instead of an enum to keep the
    /// inventory schema dependency-free — `BorderInheritanceKind` lives
    /// in `native-theme-derive` which the runtime crate cannot import.
    #[cfg_attr(not(test), allow(dead_code))]
    pub kind: &'static str,
}
inventory::collect!(BorderInheritanceInfo);

/// Font inheritance registry (Phase 94-01 G6).
///
/// Populated by `#[derive(ThemeWidget)]` for widgets that declare one or
/// more `#[theme_inherit(font = "<field>")]` attributes. Each attribute
/// produces one row in this registry; widgets like `list` (item_font +
/// header_font) and `dialog` (title_font + body_font) contribute two rows.
///
/// Consumed by the inverted drift test
/// `font_inheritance_toml_matches_macro_emit` in `inheritance.rs::tests`
/// which asserts `docs/inheritance-rules.toml [font_inheritance]` matches
/// the registry byte-for-byte.
///
/// Fields carry `#[cfg_attr(not(test), allow(dead_code))]` — see
/// [`BorderInheritanceInfo`] for rationale.
pub(crate) struct FontInheritanceInfo {
    /// Snake_case widget name (e.g., "button", "list", "dialog", "link").
    #[cfg_attr(not(test), allow(dead_code))]
    pub widget_name: &'static str,
    /// Name of the font field on the widget Option struct
    /// (e.g., `"font"`, `"title_bar_font"`, `"item_font"`, `"header_font"`,
    /// `"title_font"`, `"body_font"`).
    #[cfg_attr(not(test), allow(dead_code))]
    pub font_field: &'static str,
}
inventory::collect!(FontInheritanceInfo);

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
    #[doc(hidden)]
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
    /// Note: `icon_set` resolution is handled at the
    /// [`Theme`](crate::Theme) / pipeline level. `icon_theme` is per-variant
    /// on [`ThemeDefaults`](crate::ThemeDefaults) and resolved in the pipeline.
    #[doc(hidden)]
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
    #[doc(hidden)]
    pub fn resolve_all(&mut self) {
        self.resolve();
        self.resolve_platform_defaults();
    }

    /// Apply inheritance rules using a pre-built [`ResolutionContext`].
    ///
    /// Replacement for [`resolve_all()`](Self::resolve_all) that reads
    /// `ctx.button_order` instead of calling
    /// `inheritance::platform_button_order` inline. Used by
    /// [`into_resolved`](Self::into_resolved) so that platform detection
    /// happens once per theme-build (in
    /// [`ResolutionContext::from_system`](crate::resolve::ResolutionContext::from_system))
    /// rather than inside each variant resolution.
    ///
    /// Note: `icon_theme` resolution lives in
    /// [`pipeline::run_pipeline`](crate::pipeline) with three-tier
    /// precedence. This method intentionally does not read
    /// `ctx.icon_theme`.
    #[doc(hidden)]
    pub fn resolve_all_with_context(&mut self, ctx: &ResolutionContext) {
        self.resolve();
        if self.dialog.button_order.is_none() {
            self.dialog.button_order = Some(ctx.button_order);
        }
    }

    /// Resolve all inheritance rules and validate in one step.
    ///
    /// This is the recommended way to convert a `ThemeMode` into a
    /// [`ResolvedTheme`]. It applies every inheritance rule (including
    /// the `ctx.button_order` platform default) followed by
    /// [`validate_with_dpi()`](Self::validate_with_dpi), ensuring no
    /// fields are left unresolved.
    ///
    /// # Arguments
    ///
    /// * `ctx` -- resolution-time inputs captured once per theme-build.
    ///   Use [`ResolutionContext::from_system`](crate::resolve::ResolutionContext::from_system)
    ///   for OS-detected values (Linux/Windows ≈ 96 DPI, macOS = 72 DPI,
    ///   button order per desktop environment), or
    ///   [`ResolutionContext::for_tests`](crate::resolve::ResolutionContext::for_tests)
    ///   for deterministic test values (96 DPI, `PrimaryRight`,
    ///   no `icon_theme`). The [`resolve_system()`](Self::resolve_system)
    ///   shortcut wraps the `from_system()` case.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::ResolutionIncomplete`] if any fields
    /// remain `None` after resolution, or
    /// [`crate::Error::ResolutionInvalid`] if range checks fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::resolve::ResolutionContext;
    /// use native_theme::theme::Theme;
    ///
    /// let theme = Theme::preset("dracula")?;
    /// let variant = theme.dark.ok_or("no dark variant")?;
    /// let resolved = variant.into_resolved(&ResolutionContext::from_system())?;
    /// // Or use the zero-argument shortcut:
    /// // let resolved = variant.resolve_system()?;
    /// let _accent = resolved.defaults.accent_color;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_resolved(mut self, ctx: &ResolutionContext) -> crate::Result<ResolvedTheme> {
        self.resolve_all_with_context(ctx);
        self.validate_with_dpi(ctx.font_dpi)
    }

    /// Resolve using the OS-detected context.
    ///
    /// Equivalent to
    /// `self.into_resolved(&ResolutionContext::from_system())`.
    ///
    /// Placed on `ThemeMode` (not `Theme`) because `Theme` has both
    /// light and dark variants — variant selection must be explicit via
    /// [`Theme::into_variant`](crate::theme::Theme::into_variant). See
    /// plan 94-02 objective for the deviation rationale from gap doc §G7
    /// step 4.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::ResolutionIncomplete`] if any fields
    /// remain `None` after resolution, or
    /// [`crate::Error::ResolutionInvalid`] if range checks fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::{ColorMode, Theme};
    ///
    /// let theme = Theme::preset("dracula")?;
    /// let variant = theme.into_variant(ColorMode::Dark)?;
    /// let resolved = variant.resolve_system()?;
    /// let _accent = resolved.defaults.accent_color;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn resolve_system(self) -> crate::Result<ResolvedTheme> {
        self.into_resolved(&ResolutionContext::from_system())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests;
