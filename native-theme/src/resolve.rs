// Resolution engine: resolve() fills inheritance rules, validate() produces ResolvedThemeVariant.

use crate::error::ThemeResolutionError;
use crate::model::resolved::{
    ResolvedIconSizes, ResolvedTextScale, ResolvedTextScaleEntry, ResolvedThemeDefaults,
    ResolvedThemeSpacing, ResolvedThemeVariant,
};
use crate::model::{FontSpec, ResolvedFontSpec, TextScaleEntry, ThemeVariant};

/// Resolve a per-widget font from defaults.
/// If the widget font is None, clone defaults entirely.
/// If the widget font is Some but has None sub-fields, fill from defaults.
fn resolve_font(widget_font: &mut Option<FontSpec>, defaults_font: &FontSpec) {
    match widget_font {
        None => {
            *widget_font = Some(defaults_font.clone());
        }
        Some(font) => {
            if font.family.is_none() {
                font.family = defaults_font.family.clone();
            }
            if font.size.is_none() {
                font.size = defaults_font.size;
            }
            if font.weight.is_none() {
                font.weight = defaults_font.weight;
            }
        }
    }
}

/// Resolve a text scale entry from defaults.
/// Creates the entry if None, fills sub-fields from defaults.font,
/// computes line_height from defaults.line_height * resolved_size.
fn resolve_text_scale_entry(
    entry: &mut Option<TextScaleEntry>,
    defaults_font: &FontSpec,
    defaults_line_height: Option<f32>,
) {
    let entry = entry.get_or_insert_with(TextScaleEntry::default);
    if entry.size.is_none() {
        entry.size = defaults_font.size;
    }
    if entry.weight.is_none() {
        entry.weight = defaults_font.weight;
    }
    if entry.line_height.is_none()
        && let (Some(lh_mult), Some(size)) = (defaults_line_height, entry.size)
    {
        entry.line_height = Some(lh_mult * size);
    }
}

// --- validate() helpers ---

/// Extract a required field, recording the path if missing.
///
/// Returns the value if present, or `T::default()` as a placeholder if missing.
/// The placeholder is never used: `validate()` returns `Err` before constructing
/// `ResolvedThemeVariant` when any field was recorded as missing.
fn require<T: Clone + Default>(field: &Option<T>, path: &str, missing: &mut Vec<String>) -> T {
    match field {
        Some(val) => val.clone(),
        None => {
            missing.push(path.to_string());
            T::default()
        }
    }
}

/// Validate a FontSpec that is stored directly (not wrapped in Option).
/// Checks each sub-field individually.
fn require_font(font: &FontSpec, prefix: &str, missing: &mut Vec<String>) -> ResolvedFontSpec {
    let family = require(&font.family, &format!("{prefix}.family"), missing);
    let size = require(&font.size, &format!("{prefix}.size"), missing);
    let weight = require(&font.weight, &format!("{prefix}.weight"), missing);
    ResolvedFontSpec {
        family,
        size,
        weight,
    }
}

/// Validate an `Option<FontSpec>` (widget font fields).
/// If None, records the path as missing.
fn require_font_opt(
    font: &Option<FontSpec>,
    prefix: &str,
    missing: &mut Vec<String>,
) -> ResolvedFontSpec {
    match font {
        None => {
            missing.push(prefix.to_string());
            ResolvedFontSpec::default()
        }
        Some(f) => {
            let family = require(&f.family, &format!("{prefix}.family"), missing);
            let size = require(&f.size, &format!("{prefix}.size"), missing);
            let weight = require(&f.weight, &format!("{prefix}.weight"), missing);
            ResolvedFontSpec {
                family,
                size,
                weight,
            }
        }
    }
}

/// Validate an `Option<TextScaleEntry>`.
fn require_text_scale_entry(
    entry: &Option<TextScaleEntry>,
    prefix: &str,
    missing: &mut Vec<String>,
) -> ResolvedTextScaleEntry {
    match entry {
        None => {
            missing.push(prefix.to_string());
            ResolvedTextScaleEntry::default()
        }
        Some(e) => {
            let size = require(&e.size, &format!("{prefix}.size"), missing);
            let weight = require(&e.weight, &format!("{prefix}.weight"), missing);
            let line_height = require(&e.line_height, &format!("{prefix}.line_height"), missing);
            ResolvedTextScaleEntry {
                size,
                weight,
                line_height,
            }
        }
    }
}

// --- Range-check helpers for validate() ---
//
// These push a descriptive message to the `missing` vec (reusing the same
// error-collection pattern as require()) so that all problems — missing
// fields AND out-of-range values — are reported in a single pass.

/// Check that an `f32` value is non-negative (>= 0.0).
fn check_non_negative(value: f32, path: &str, errors: &mut Vec<String>) {
    if value < 0.0 {
        errors.push(format!("{path} must be >= 0, got {value}"));
    }
}

/// Check that an `f32` value is strictly positive (> 0.0).
fn check_positive(value: f32, path: &str, errors: &mut Vec<String>) {
    if value <= 0.0 {
        errors.push(format!("{path} must be > 0, got {value}"));
    }
}

/// Check that an `f32` value falls within an inclusive range.
fn check_range_f32(value: f32, min: f32, max: f32, path: &str, errors: &mut Vec<String>) {
    if value < min || value > max {
        errors.push(format!("{path} must be {min}..={max}, got {value}"));
    }
}

/// Check that a `u16` value falls within an inclusive range.
fn check_range_u16(value: u16, min: u16, max: u16, path: &str, errors: &mut Vec<String>) {
    if value < min || value > max {
        errors.push(format!("{path} must be {min}..={max}, got {value}"));
    }
}

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
    /// 2. **Safety nets** -- platform-divergent fields get a reasonable fallback.
    /// 3. **Widget-from-defaults** -- colors, geometry, fonts, text scale entries
    ///    all inherit from defaults.
    /// 4. **Widget-to-widget** -- inactive title bar fields fall back to active.
    /// 5. **Icon set** -- fills icon_set from the compile-time system default.
    pub fn resolve(&mut self) {
        self.resolve_defaults_internal();
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
    /// let accent = resolved.defaults.accent;
    /// ```
    #[must_use = "this returns the resolved theme; it does not modify self"]
    pub fn into_resolved(mut self) -> crate::Result<ResolvedThemeVariant> {
        self.resolve_all();
        self.validate()
    }

    // --- Phase 1: Defaults internal chains ---

    fn resolve_defaults_internal(&mut self) {
        let d = &mut self.defaults;

        // selection <- accent
        if d.selection.is_none() {
            d.selection = d.accent;
        }
        // focus_ring_color <- accent
        if d.focus_ring_color.is_none() {
            d.focus_ring_color = d.accent;
        }
        // selection_inactive <- selection (MUST run after selection is set)
        if d.selection_inactive.is_none() {
            d.selection_inactive = d.selection;
        }
    }

    // --- Phase 2: Safety nets ---

    fn resolve_safety_nets(&mut self) {
        // input.caret <- defaults.foreground
        if self.input.caret.is_none() {
            self.input.caret = self.defaults.foreground;
        }
        // scrollbar.track <- defaults.background
        if self.scrollbar.track.is_none() {
            self.scrollbar.track = self.defaults.background;
        }
        // spinner.fill <- defaults.foreground
        if self.spinner.fill.is_none() {
            self.spinner.fill = self.defaults.foreground;
        }
        // popover.background <- defaults.background
        if self.popover.background.is_none() {
            self.popover.background = self.defaults.background;
        }
        // list.background <- defaults.background
        if self.list.background.is_none() {
            self.list.background = self.defaults.background;
        }
    }

    // --- Phase 3: Widget-from-defaults ---

    fn resolve_widgets_from_defaults(&mut self) {
        self.resolve_color_inheritance();
        self.resolve_font_inheritance();
        self.resolve_text_scale();
    }

    fn resolve_color_inheritance(&mut self) {
        let d = &self.defaults;

        // --- window ---
        if self.window.background.is_none() {
            self.window.background = d.background;
        }
        if self.window.foreground.is_none() {
            self.window.foreground = d.foreground;
        }
        if self.window.border.is_none() {
            self.window.border = d.border;
        }
        if self.window.title_bar_background.is_none() {
            self.window.title_bar_background = d.surface;
        }
        if self.window.title_bar_foreground.is_none() {
            self.window.title_bar_foreground = d.foreground;
        }
        if self.window.radius.is_none() {
            self.window.radius = d.radius_lg;
        }
        if self.window.shadow.is_none() {
            self.window.shadow = d.shadow_enabled;
        }

        // --- button ---
        if self.button.background.is_none() {
            self.button.background = d.background;
        }
        if self.button.foreground.is_none() {
            self.button.foreground = d.foreground;
        }
        if self.button.border.is_none() {
            self.button.border = d.border;
        }
        if self.button.primary_background.is_none() {
            self.button.primary_background = d.accent;
        }
        if self.button.primary_foreground.is_none() {
            self.button.primary_foreground = d.accent_foreground;
        }
        if self.button.radius.is_none() {
            self.button.radius = d.radius;
        }
        if self.button.disabled_opacity.is_none() {
            self.button.disabled_opacity = d.disabled_opacity;
        }
        if self.button.shadow.is_none() {
            self.button.shadow = d.shadow_enabled;
        }

        // --- input ---
        if self.input.background.is_none() {
            self.input.background = d.background;
        }
        if self.input.foreground.is_none() {
            self.input.foreground = d.foreground;
        }
        if self.input.border.is_none() {
            self.input.border = d.border;
        }
        if self.input.placeholder.is_none() {
            self.input.placeholder = d.muted;
        }
        if self.input.selection.is_none() {
            self.input.selection = d.selection;
        }
        if self.input.selection_foreground.is_none() {
            self.input.selection_foreground = d.selection_foreground;
        }
        if self.input.radius.is_none() {
            self.input.radius = d.radius;
        }
        if self.input.border_width.is_none() {
            self.input.border_width = d.frame_width;
        }

        // --- checkbox ---
        if self.checkbox.checked_bg.is_none() {
            self.checkbox.checked_bg = d.accent;
        }
        if self.checkbox.radius.is_none() {
            self.checkbox.radius = d.radius;
        }
        if self.checkbox.border_width.is_none() {
            self.checkbox.border_width = d.frame_width;
        }

        // --- menu ---
        if self.menu.background.is_none() {
            self.menu.background = d.background;
        }
        if self.menu.foreground.is_none() {
            self.menu.foreground = d.foreground;
        }
        if self.menu.separator.is_none() {
            self.menu.separator = d.border;
        }

        // --- tooltip ---
        if self.tooltip.background.is_none() {
            self.tooltip.background = d.background;
        }
        if self.tooltip.foreground.is_none() {
            self.tooltip.foreground = d.foreground;
        }
        if self.tooltip.radius.is_none() {
            self.tooltip.radius = d.radius;
        }

        // --- scrollbar ---
        if self.scrollbar.thumb.is_none() {
            self.scrollbar.thumb = d.muted;
        }
        if self.scrollbar.thumb_hover.is_none() {
            self.scrollbar.thumb_hover = d.muted;
        }

        // --- slider ---
        if self.slider.fill.is_none() {
            self.slider.fill = d.accent;
        }
        if self.slider.track.is_none() {
            self.slider.track = d.muted;
        }
        if self.slider.thumb.is_none() {
            self.slider.thumb = d.surface;
        }

        // --- progress_bar ---
        if self.progress_bar.fill.is_none() {
            self.progress_bar.fill = d.accent;
        }
        if self.progress_bar.track.is_none() {
            self.progress_bar.track = d.muted;
        }
        if self.progress_bar.radius.is_none() {
            self.progress_bar.radius = d.radius;
        }

        // --- tab ---
        if self.tab.background.is_none() {
            self.tab.background = d.background;
        }
        if self.tab.foreground.is_none() {
            self.tab.foreground = d.foreground;
        }
        if self.tab.active_background.is_none() {
            self.tab.active_background = d.background;
        }
        if self.tab.active_foreground.is_none() {
            self.tab.active_foreground = d.foreground;
        }
        if self.tab.bar_background.is_none() {
            self.tab.bar_background = d.background;
        }

        // --- sidebar ---
        if self.sidebar.background.is_none() {
            self.sidebar.background = d.background;
        }
        if self.sidebar.foreground.is_none() {
            self.sidebar.foreground = d.foreground;
        }

        // --- list ---
        if self.list.foreground.is_none() {
            self.list.foreground = d.foreground;
        }
        if self.list.alternate_row.is_none() {
            self.list.alternate_row = self.list.background;
        }
        if self.list.selection.is_none() {
            self.list.selection = d.selection;
        }
        if self.list.selection_foreground.is_none() {
            self.list.selection_foreground = d.selection_foreground;
        }
        if self.list.header_background.is_none() {
            self.list.header_background = d.surface;
        }
        if self.list.header_foreground.is_none() {
            self.list.header_foreground = d.foreground;
        }
        if self.list.grid_color.is_none() {
            self.list.grid_color = d.border;
        }

        // --- popover ---
        if self.popover.foreground.is_none() {
            self.popover.foreground = d.foreground;
        }
        if self.popover.border.is_none() {
            self.popover.border = d.border;
        }
        if self.popover.radius.is_none() {
            self.popover.radius = d.radius_lg;
        }

        // --- separator ---
        if self.separator.color.is_none() {
            self.separator.color = d.border;
        }

        // --- switch ---
        if self.switch.checked_background.is_none() {
            self.switch.checked_background = d.accent;
        }
        if self.switch.unchecked_background.is_none() {
            self.switch.unchecked_background = d.muted;
        }
        if self.switch.thumb_background.is_none() {
            self.switch.thumb_background = d.surface;
        }

        // --- dialog ---
        if self.dialog.radius.is_none() {
            self.dialog.radius = d.radius_lg;
        }

        // --- combo_box ---
        if self.combo_box.radius.is_none() {
            self.combo_box.radius = d.radius;
        }

        // --- segmented_control ---
        if self.segmented_control.radius.is_none() {
            self.segmented_control.radius = d.radius;
        }

        // --- card ---
        if self.card.background.is_none() {
            self.card.background = d.surface;
        }
        if self.card.border.is_none() {
            self.card.border = d.border;
        }
        if self.card.radius.is_none() {
            self.card.radius = d.radius_lg;
        }
        if self.card.shadow.is_none() {
            self.card.shadow = d.shadow_enabled;
        }

        // --- expander ---
        if self.expander.radius.is_none() {
            self.expander.radius = d.radius;
        }

        // --- link ---
        if self.link.color.is_none() {
            self.link.color = d.link;
        }
        if self.link.visited.is_none() {
            self.link.visited = d.link;
        }
    }

    fn resolve_font_inheritance(&mut self) {
        let defaults_font = &self.defaults.font;
        resolve_font(&mut self.window.title_bar_font, defaults_font);
        resolve_font(&mut self.button.font, defaults_font);
        resolve_font(&mut self.input.font, defaults_font);
        resolve_font(&mut self.menu.font, defaults_font);
        resolve_font(&mut self.tooltip.font, defaults_font);
        resolve_font(&mut self.toolbar.font, defaults_font);
        resolve_font(&mut self.status_bar.font, defaults_font);
        resolve_font(&mut self.dialog.title_font, defaults_font);
    }

    fn resolve_text_scale(&mut self) {
        let defaults_font = &self.defaults.font;
        let defaults_lh = self.defaults.line_height;
        resolve_text_scale_entry(&mut self.text_scale.caption, defaults_font, defaults_lh);
        resolve_text_scale_entry(
            &mut self.text_scale.section_heading,
            defaults_font,
            defaults_lh,
        );
        resolve_text_scale_entry(
            &mut self.text_scale.dialog_title,
            defaults_font,
            defaults_lh,
        );
        resolve_text_scale_entry(&mut self.text_scale.display, defaults_font, defaults_lh);
    }

    // --- Phase 4: Widget-to-widget chains ---

    fn resolve_widget_to_widget(&mut self) {
        // inactive title bar <- active title bar
        if self.window.inactive_title_bar_background.is_none() {
            self.window.inactive_title_bar_background = self.window.title_bar_background;
        }
        if self.window.inactive_title_bar_foreground.is_none() {
            self.window.inactive_title_bar_foreground = self.window.title_bar_foreground;
        }
    }

    // --- validate() ---

    /// Convert this ThemeVariant into a [`ResolvedThemeVariant`] with all fields guaranteed.
    ///
    /// Should be called after [`resolve()`](ThemeVariant::resolve). Walks every field
    /// and collects missing (None) field paths. Returns `Ok(ResolvedThemeVariant)` if all fields
    /// are populated, or `Err(Error::Resolution(...))` listing every missing field.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Resolution`] containing a [`ThemeResolutionError`]
    /// with all missing field paths if any fields remain None.
    #[must_use = "this returns the resolved theme; it does not modify self"]
    pub fn validate(&self) -> crate::Result<ResolvedThemeVariant> {
        let mut missing = Vec::new();

        // --- defaults ---

        let defaults_font = require_font(&self.defaults.font, "defaults.font", &mut missing);
        let defaults_line_height = require(
            &self.defaults.line_height,
            "defaults.line_height",
            &mut missing,
        );
        let defaults_mono_font =
            require_font(&self.defaults.mono_font, "defaults.mono_font", &mut missing);

        let defaults_background = require(
            &self.defaults.background,
            "defaults.background",
            &mut missing,
        );
        let defaults_foreground = require(
            &self.defaults.foreground,
            "defaults.foreground",
            &mut missing,
        );
        let defaults_accent = require(&self.defaults.accent, "defaults.accent", &mut missing);
        let defaults_accent_foreground = require(
            &self.defaults.accent_foreground,
            "defaults.accent_foreground",
            &mut missing,
        );
        let defaults_surface = require(&self.defaults.surface, "defaults.surface", &mut missing);
        let defaults_border = require(&self.defaults.border, "defaults.border", &mut missing);
        let defaults_muted = require(&self.defaults.muted, "defaults.muted", &mut missing);
        let defaults_shadow = require(&self.defaults.shadow, "defaults.shadow", &mut missing);
        let defaults_link = require(&self.defaults.link, "defaults.link", &mut missing);
        let defaults_selection =
            require(&self.defaults.selection, "defaults.selection", &mut missing);
        let defaults_selection_foreground = require(
            &self.defaults.selection_foreground,
            "defaults.selection_foreground",
            &mut missing,
        );
        let defaults_selection_inactive = require(
            &self.defaults.selection_inactive,
            "defaults.selection_inactive",
            &mut missing,
        );
        let defaults_disabled_foreground = require(
            &self.defaults.disabled_foreground,
            "defaults.disabled_foreground",
            &mut missing,
        );

        let defaults_danger = require(&self.defaults.danger, "defaults.danger", &mut missing);
        let defaults_danger_foreground = require(
            &self.defaults.danger_foreground,
            "defaults.danger_foreground",
            &mut missing,
        );
        let defaults_warning = require(&self.defaults.warning, "defaults.warning", &mut missing);
        let defaults_warning_foreground = require(
            &self.defaults.warning_foreground,
            "defaults.warning_foreground",
            &mut missing,
        );
        let defaults_success = require(&self.defaults.success, "defaults.success", &mut missing);
        let defaults_success_foreground = require(
            &self.defaults.success_foreground,
            "defaults.success_foreground",
            &mut missing,
        );
        let defaults_info = require(&self.defaults.info, "defaults.info", &mut missing);
        let defaults_info_foreground = require(
            &self.defaults.info_foreground,
            "defaults.info_foreground",
            &mut missing,
        );

        let defaults_radius = require(&self.defaults.radius, "defaults.radius", &mut missing);
        let defaults_radius_lg =
            require(&self.defaults.radius_lg, "defaults.radius_lg", &mut missing);
        let defaults_frame_width = require(
            &self.defaults.frame_width,
            "defaults.frame_width",
            &mut missing,
        );
        let defaults_disabled_opacity = require(
            &self.defaults.disabled_opacity,
            "defaults.disabled_opacity",
            &mut missing,
        );
        let defaults_border_opacity = require(
            &self.defaults.border_opacity,
            "defaults.border_opacity",
            &mut missing,
        );
        let defaults_shadow_enabled = require(
            &self.defaults.shadow_enabled,
            "defaults.shadow_enabled",
            &mut missing,
        );

        let defaults_focus_ring_color = require(
            &self.defaults.focus_ring_color,
            "defaults.focus_ring_color",
            &mut missing,
        );
        let defaults_focus_ring_width = require(
            &self.defaults.focus_ring_width,
            "defaults.focus_ring_width",
            &mut missing,
        );
        let defaults_focus_ring_offset = require(
            &self.defaults.focus_ring_offset,
            "defaults.focus_ring_offset",
            &mut missing,
        );

        let defaults_spacing_xxs = require(
            &self.defaults.spacing.xxs,
            "defaults.spacing.xxs",
            &mut missing,
        );
        let defaults_spacing_xs = require(
            &self.defaults.spacing.xs,
            "defaults.spacing.xs",
            &mut missing,
        );
        let defaults_spacing_s =
            require(&self.defaults.spacing.s, "defaults.spacing.s", &mut missing);
        let defaults_spacing_m =
            require(&self.defaults.spacing.m, "defaults.spacing.m", &mut missing);
        let defaults_spacing_l =
            require(&self.defaults.spacing.l, "defaults.spacing.l", &mut missing);
        let defaults_spacing_xl = require(
            &self.defaults.spacing.xl,
            "defaults.spacing.xl",
            &mut missing,
        );
        let defaults_spacing_xxl = require(
            &self.defaults.spacing.xxl,
            "defaults.spacing.xxl",
            &mut missing,
        );

        let defaults_icon_sizes_toolbar = require(
            &self.defaults.icon_sizes.toolbar,
            "defaults.icon_sizes.toolbar",
            &mut missing,
        );
        let defaults_icon_sizes_small = require(
            &self.defaults.icon_sizes.small,
            "defaults.icon_sizes.small",
            &mut missing,
        );
        let defaults_icon_sizes_large = require(
            &self.defaults.icon_sizes.large,
            "defaults.icon_sizes.large",
            &mut missing,
        );
        let defaults_icon_sizes_dialog = require(
            &self.defaults.icon_sizes.dialog,
            "defaults.icon_sizes.dialog",
            &mut missing,
        );
        let defaults_icon_sizes_panel = require(
            &self.defaults.icon_sizes.panel,
            "defaults.icon_sizes.panel",
            &mut missing,
        );

        let defaults_text_scaling_factor = require(
            &self.defaults.text_scaling_factor,
            "defaults.text_scaling_factor",
            &mut missing,
        );
        let defaults_reduce_motion = require(
            &self.defaults.reduce_motion,
            "defaults.reduce_motion",
            &mut missing,
        );
        let defaults_high_contrast = require(
            &self.defaults.high_contrast,
            "defaults.high_contrast",
            &mut missing,
        );
        let defaults_reduce_transparency = require(
            &self.defaults.reduce_transparency,
            "defaults.reduce_transparency",
            &mut missing,
        );

        // --- text_scale ---

        let ts_caption =
            require_text_scale_entry(&self.text_scale.caption, "text_scale.caption", &mut missing);
        let ts_section_heading = require_text_scale_entry(
            &self.text_scale.section_heading,
            "text_scale.section_heading",
            &mut missing,
        );
        let ts_dialog_title = require_text_scale_entry(
            &self.text_scale.dialog_title,
            "text_scale.dialog_title",
            &mut missing,
        );
        let ts_display =
            require_text_scale_entry(&self.text_scale.display, "text_scale.display", &mut missing);

        // --- window ---

        let window_background = require(&self.window.background, "window.background", &mut missing);
        let window_foreground = require(&self.window.foreground, "window.foreground", &mut missing);
        let window_border = require(&self.window.border, "window.border", &mut missing);
        let window_title_bar_background = require(
            &self.window.title_bar_background,
            "window.title_bar_background",
            &mut missing,
        );
        let window_title_bar_foreground = require(
            &self.window.title_bar_foreground,
            "window.title_bar_foreground",
            &mut missing,
        );
        let window_inactive_title_bar_background = require(
            &self.window.inactive_title_bar_background,
            "window.inactive_title_bar_background",
            &mut missing,
        );
        let window_inactive_title_bar_foreground = require(
            &self.window.inactive_title_bar_foreground,
            "window.inactive_title_bar_foreground",
            &mut missing,
        );
        let window_radius = require(&self.window.radius, "window.radius", &mut missing);
        let window_shadow = require(&self.window.shadow, "window.shadow", &mut missing);
        let window_title_bar_font = require_font_opt(
            &self.window.title_bar_font,
            "window.title_bar_font",
            &mut missing,
        );

        // --- button ---

        let button_background = require(&self.button.background, "button.background", &mut missing);
        let button_foreground = require(&self.button.foreground, "button.foreground", &mut missing);
        let button_border = require(&self.button.border, "button.border", &mut missing);
        let button_primary_background = require(
            &self.button.primary_background,
            "button.primary_background",
            &mut missing,
        );
        let button_primary_foreground = require(
            &self.button.primary_foreground,
            "button.primary_foreground",
            &mut missing,
        );
        let button_min_width = require(&self.button.min_width, "button.min_width", &mut missing);
        let button_min_height = require(&self.button.min_height, "button.min_height", &mut missing);
        let button_padding_horizontal = require(
            &self.button.padding_horizontal,
            "button.padding_horizontal",
            &mut missing,
        );
        let button_padding_vertical = require(
            &self.button.padding_vertical,
            "button.padding_vertical",
            &mut missing,
        );
        let button_radius = require(&self.button.radius, "button.radius", &mut missing);
        let button_icon_spacing = require(
            &self.button.icon_spacing,
            "button.icon_spacing",
            &mut missing,
        );
        let button_disabled_opacity = require(
            &self.button.disabled_opacity,
            "button.disabled_opacity",
            &mut missing,
        );
        let button_shadow = require(&self.button.shadow, "button.shadow", &mut missing);
        let button_font = require_font_opt(&self.button.font, "button.font", &mut missing);

        // --- input ---

        let input_background = require(&self.input.background, "input.background", &mut missing);
        let input_foreground = require(&self.input.foreground, "input.foreground", &mut missing);
        let input_border = require(&self.input.border, "input.border", &mut missing);
        let input_placeholder = require(&self.input.placeholder, "input.placeholder", &mut missing);
        let input_caret = require(&self.input.caret, "input.caret", &mut missing);
        let input_selection = require(&self.input.selection, "input.selection", &mut missing);
        let input_selection_foreground = require(
            &self.input.selection_foreground,
            "input.selection_foreground",
            &mut missing,
        );
        let input_min_height = require(&self.input.min_height, "input.min_height", &mut missing);
        let input_padding_horizontal = require(
            &self.input.padding_horizontal,
            "input.padding_horizontal",
            &mut missing,
        );
        let input_padding_vertical = require(
            &self.input.padding_vertical,
            "input.padding_vertical",
            &mut missing,
        );
        let input_radius = require(&self.input.radius, "input.radius", &mut missing);
        let input_border_width =
            require(&self.input.border_width, "input.border_width", &mut missing);
        let input_font = require_font_opt(&self.input.font, "input.font", &mut missing);

        // --- checkbox ---

        let checkbox_checked_bg = require(
            &self.checkbox.checked_bg,
            "checkbox.checked_bg",
            &mut missing,
        );
        let checkbox_indicator_size = require(
            &self.checkbox.indicator_size,
            "checkbox.indicator_size",
            &mut missing,
        );
        let checkbox_spacing = require(&self.checkbox.spacing, "checkbox.spacing", &mut missing);
        let checkbox_radius = require(&self.checkbox.radius, "checkbox.radius", &mut missing);
        let checkbox_border_width = require(
            &self.checkbox.border_width,
            "checkbox.border_width",
            &mut missing,
        );

        // --- menu ---

        let menu_background = require(&self.menu.background, "menu.background", &mut missing);
        let menu_foreground = require(&self.menu.foreground, "menu.foreground", &mut missing);
        let menu_separator = require(&self.menu.separator, "menu.separator", &mut missing);
        let menu_item_height = require(&self.menu.item_height, "menu.item_height", &mut missing);
        let menu_padding_horizontal = require(
            &self.menu.padding_horizontal,
            "menu.padding_horizontal",
            &mut missing,
        );
        let menu_padding_vertical = require(
            &self.menu.padding_vertical,
            "menu.padding_vertical",
            &mut missing,
        );
        let menu_icon_spacing = require(&self.menu.icon_spacing, "menu.icon_spacing", &mut missing);
        let menu_font = require_font_opt(&self.menu.font, "menu.font", &mut missing);

        // --- tooltip ---

        let tooltip_background =
            require(&self.tooltip.background, "tooltip.background", &mut missing);
        let tooltip_foreground =
            require(&self.tooltip.foreground, "tooltip.foreground", &mut missing);
        let tooltip_padding_horizontal = require(
            &self.tooltip.padding_horizontal,
            "tooltip.padding_horizontal",
            &mut missing,
        );
        let tooltip_padding_vertical = require(
            &self.tooltip.padding_vertical,
            "tooltip.padding_vertical",
            &mut missing,
        );
        let tooltip_max_width = require(&self.tooltip.max_width, "tooltip.max_width", &mut missing);
        let tooltip_radius = require(&self.tooltip.radius, "tooltip.radius", &mut missing);
        let tooltip_font = require_font_opt(&self.tooltip.font, "tooltip.font", &mut missing);

        // --- scrollbar ---

        let scrollbar_track = require(&self.scrollbar.track, "scrollbar.track", &mut missing);
        let scrollbar_thumb = require(&self.scrollbar.thumb, "scrollbar.thumb", &mut missing);
        let scrollbar_thumb_hover = require(
            &self.scrollbar.thumb_hover,
            "scrollbar.thumb_hover",
            &mut missing,
        );
        let scrollbar_width = require(&self.scrollbar.width, "scrollbar.width", &mut missing);
        let scrollbar_min_thumb_height = require(
            &self.scrollbar.min_thumb_height,
            "scrollbar.min_thumb_height",
            &mut missing,
        );
        let scrollbar_slider_width = require(
            &self.scrollbar.slider_width,
            "scrollbar.slider_width",
            &mut missing,
        );
        let scrollbar_overlay_mode = require(
            &self.scrollbar.overlay_mode,
            "scrollbar.overlay_mode",
            &mut missing,
        );

        // --- slider ---

        let slider_fill = require(&self.slider.fill, "slider.fill", &mut missing);
        let slider_track = require(&self.slider.track, "slider.track", &mut missing);
        let slider_thumb = require(&self.slider.thumb, "slider.thumb", &mut missing);
        let slider_track_height = require(
            &self.slider.track_height,
            "slider.track_height",
            &mut missing,
        );
        let slider_thumb_size = require(&self.slider.thumb_size, "slider.thumb_size", &mut missing);
        let slider_tick_length =
            require(&self.slider.tick_length, "slider.tick_length", &mut missing);

        // --- progress_bar ---

        let progress_bar_fill = require(&self.progress_bar.fill, "progress_bar.fill", &mut missing);
        let progress_bar_track =
            require(&self.progress_bar.track, "progress_bar.track", &mut missing);
        let progress_bar_height = require(
            &self.progress_bar.height,
            "progress_bar.height",
            &mut missing,
        );
        let progress_bar_min_width = require(
            &self.progress_bar.min_width,
            "progress_bar.min_width",
            &mut missing,
        );
        let progress_bar_radius = require(
            &self.progress_bar.radius,
            "progress_bar.radius",
            &mut missing,
        );

        // --- tab ---

        let tab_background = require(&self.tab.background, "tab.background", &mut missing);
        let tab_foreground = require(&self.tab.foreground, "tab.foreground", &mut missing);
        let tab_active_background = require(
            &self.tab.active_background,
            "tab.active_background",
            &mut missing,
        );
        let tab_active_foreground = require(
            &self.tab.active_foreground,
            "tab.active_foreground",
            &mut missing,
        );
        let tab_bar_background =
            require(&self.tab.bar_background, "tab.bar_background", &mut missing);
        let tab_min_width = require(&self.tab.min_width, "tab.min_width", &mut missing);
        let tab_min_height = require(&self.tab.min_height, "tab.min_height", &mut missing);
        let tab_padding_horizontal = require(
            &self.tab.padding_horizontal,
            "tab.padding_horizontal",
            &mut missing,
        );
        let tab_padding_vertical = require(
            &self.tab.padding_vertical,
            "tab.padding_vertical",
            &mut missing,
        );

        // --- sidebar ---

        let sidebar_background =
            require(&self.sidebar.background, "sidebar.background", &mut missing);
        let sidebar_foreground =
            require(&self.sidebar.foreground, "sidebar.foreground", &mut missing);

        // --- toolbar ---

        let toolbar_height = require(&self.toolbar.height, "toolbar.height", &mut missing);
        let toolbar_item_spacing = require(
            &self.toolbar.item_spacing,
            "toolbar.item_spacing",
            &mut missing,
        );
        let toolbar_padding = require(&self.toolbar.padding, "toolbar.padding", &mut missing);
        let toolbar_font = require_font_opt(&self.toolbar.font, "toolbar.font", &mut missing);

        // --- status_bar ---

        let status_bar_font =
            require_font_opt(&self.status_bar.font, "status_bar.font", &mut missing);

        // --- list ---

        let list_background = require(&self.list.background, "list.background", &mut missing);
        let list_foreground = require(&self.list.foreground, "list.foreground", &mut missing);
        let list_alternate_row =
            require(&self.list.alternate_row, "list.alternate_row", &mut missing);
        let list_selection = require(&self.list.selection, "list.selection", &mut missing);
        let list_selection_foreground = require(
            &self.list.selection_foreground,
            "list.selection_foreground",
            &mut missing,
        );
        let list_header_background = require(
            &self.list.header_background,
            "list.header_background",
            &mut missing,
        );
        let list_header_foreground = require(
            &self.list.header_foreground,
            "list.header_foreground",
            &mut missing,
        );
        let list_grid_color = require(&self.list.grid_color, "list.grid_color", &mut missing);
        let list_item_height = require(&self.list.item_height, "list.item_height", &mut missing);
        let list_padding_horizontal = require(
            &self.list.padding_horizontal,
            "list.padding_horizontal",
            &mut missing,
        );
        let list_padding_vertical = require(
            &self.list.padding_vertical,
            "list.padding_vertical",
            &mut missing,
        );

        // --- popover ---

        let popover_background =
            require(&self.popover.background, "popover.background", &mut missing);
        let popover_foreground =
            require(&self.popover.foreground, "popover.foreground", &mut missing);
        let popover_border = require(&self.popover.border, "popover.border", &mut missing);
        let popover_radius = require(&self.popover.radius, "popover.radius", &mut missing);

        // --- splitter ---

        let splitter_width = require(&self.splitter.width, "splitter.width", &mut missing);

        // --- separator ---

        let separator_color = require(&self.separator.color, "separator.color", &mut missing);

        // --- switch ---

        let switch_checked_background = require(
            &self.switch.checked_background,
            "switch.checked_background",
            &mut missing,
        );
        let switch_unchecked_background = require(
            &self.switch.unchecked_background,
            "switch.unchecked_background",
            &mut missing,
        );
        let switch_thumb_background = require(
            &self.switch.thumb_background,
            "switch.thumb_background",
            &mut missing,
        );
        let switch_track_width =
            require(&self.switch.track_width, "switch.track_width", &mut missing);
        let switch_track_height = require(
            &self.switch.track_height,
            "switch.track_height",
            &mut missing,
        );
        let switch_thumb_size = require(&self.switch.thumb_size, "switch.thumb_size", &mut missing);
        let switch_track_radius = require(
            &self.switch.track_radius,
            "switch.track_radius",
            &mut missing,
        );

        // --- dialog ---

        let dialog_min_width = require(&self.dialog.min_width, "dialog.min_width", &mut missing);
        let dialog_max_width = require(&self.dialog.max_width, "dialog.max_width", &mut missing);
        let dialog_min_height = require(&self.dialog.min_height, "dialog.min_height", &mut missing);
        let dialog_max_height = require(&self.dialog.max_height, "dialog.max_height", &mut missing);
        let dialog_content_padding = require(
            &self.dialog.content_padding,
            "dialog.content_padding",
            &mut missing,
        );
        let dialog_button_spacing = require(
            &self.dialog.button_spacing,
            "dialog.button_spacing",
            &mut missing,
        );
        let dialog_radius = require(&self.dialog.radius, "dialog.radius", &mut missing);
        let dialog_icon_size = require(&self.dialog.icon_size, "dialog.icon_size", &mut missing);
        let dialog_button_order = require(
            &self.dialog.button_order,
            "dialog.button_order",
            &mut missing,
        );
        let dialog_title_font =
            require_font_opt(&self.dialog.title_font, "dialog.title_font", &mut missing);

        // --- spinner ---

        let spinner_fill = require(&self.spinner.fill, "spinner.fill", &mut missing);
        let spinner_diameter = require(&self.spinner.diameter, "spinner.diameter", &mut missing);
        let spinner_min_size = require(&self.spinner.min_size, "spinner.min_size", &mut missing);
        let spinner_stroke_width = require(
            &self.spinner.stroke_width,
            "spinner.stroke_width",
            &mut missing,
        );

        // --- combo_box ---

        let combo_box_min_height = require(
            &self.combo_box.min_height,
            "combo_box.min_height",
            &mut missing,
        );
        let combo_box_min_width = require(
            &self.combo_box.min_width,
            "combo_box.min_width",
            &mut missing,
        );
        let combo_box_padding_horizontal = require(
            &self.combo_box.padding_horizontal,
            "combo_box.padding_horizontal",
            &mut missing,
        );
        let combo_box_arrow_size = require(
            &self.combo_box.arrow_size,
            "combo_box.arrow_size",
            &mut missing,
        );
        let combo_box_arrow_area_width = require(
            &self.combo_box.arrow_area_width,
            "combo_box.arrow_area_width",
            &mut missing,
        );
        let combo_box_radius = require(&self.combo_box.radius, "combo_box.radius", &mut missing);

        // --- segmented_control ---

        let segmented_control_segment_height = require(
            &self.segmented_control.segment_height,
            "segmented_control.segment_height",
            &mut missing,
        );
        let segmented_control_separator_width = require(
            &self.segmented_control.separator_width,
            "segmented_control.separator_width",
            &mut missing,
        );
        let segmented_control_padding_horizontal = require(
            &self.segmented_control.padding_horizontal,
            "segmented_control.padding_horizontal",
            &mut missing,
        );
        let segmented_control_radius = require(
            &self.segmented_control.radius,
            "segmented_control.radius",
            &mut missing,
        );

        // --- card ---

        let card_background = require(&self.card.background, "card.background", &mut missing);
        let card_border = require(&self.card.border, "card.border", &mut missing);
        let card_radius = require(&self.card.radius, "card.radius", &mut missing);
        let card_padding = require(&self.card.padding, "card.padding", &mut missing);
        let card_shadow = require(&self.card.shadow, "card.shadow", &mut missing);

        // --- expander ---

        let expander_header_height = require(
            &self.expander.header_height,
            "expander.header_height",
            &mut missing,
        );
        let expander_arrow_size = require(
            &self.expander.arrow_size,
            "expander.arrow_size",
            &mut missing,
        );
        let expander_content_padding = require(
            &self.expander.content_padding,
            "expander.content_padding",
            &mut missing,
        );
        let expander_radius = require(&self.expander.radius, "expander.radius", &mut missing);

        // --- link ---

        let link_color = require(&self.link.color, "link.color", &mut missing);
        let link_visited = require(&self.link.visited, "link.visited", &mut missing);
        let link_background = require(&self.link.background, "link.background", &mut missing);
        let link_hover_bg = require(&self.link.hover_bg, "link.hover_bg", &mut missing);
        let link_underline = require(&self.link.underline, "link.underline", &mut missing);

        // --- icon_set / icon_theme ---

        let icon_set = require(&self.icon_set, "icon_set", &mut missing);
        let icon_theme = require(&self.icon_theme, "icon_theme", &mut missing);

        // --- range validation ---
        //
        // Operate on the already-extracted values from require(). If a field was
        // missing, require() returned T::default() as placeholder — range-checking
        // that placeholder is harmless because the missing-field error already
        // captured the real problem.

        // Fonts: size > 0, weight 100..=900
        check_positive(defaults_font.size, "defaults.font.size", &mut missing);
        check_range_u16(
            defaults_font.weight,
            100,
            900,
            "defaults.font.weight",
            &mut missing,
        );
        check_positive(
            defaults_mono_font.size,
            "defaults.mono_font.size",
            &mut missing,
        );
        check_range_u16(
            defaults_mono_font.weight,
            100,
            900,
            "defaults.mono_font.weight",
            &mut missing,
        );

        // defaults: line_height > 0, text_scaling_factor > 0
        check_positive(defaults_line_height, "defaults.line_height", &mut missing);
        check_positive(
            defaults_text_scaling_factor,
            "defaults.text_scaling_factor",
            &mut missing,
        );

        // defaults: radius, geometry >= 0
        check_non_negative(defaults_radius, "defaults.radius", &mut missing);
        check_non_negative(defaults_radius_lg, "defaults.radius_lg", &mut missing);
        check_non_negative(defaults_frame_width, "defaults.frame_width", &mut missing);
        check_non_negative(
            defaults_focus_ring_width,
            "defaults.focus_ring_width",
            &mut missing,
        );
        // Note: focus_ring_offset is intentionally NOT range-checked — negative values
        // mean an inset focus ring (e.g., adwaita uses -2.0, macOS uses -1.0).

        // defaults: opacity 0..=1
        check_range_f32(
            defaults_disabled_opacity,
            0.0,
            1.0,
            "defaults.disabled_opacity",
            &mut missing,
        );
        check_range_f32(
            defaults_border_opacity,
            0.0,
            1.0,
            "defaults.border_opacity",
            &mut missing,
        );

        // defaults: spacing >= 0
        check_non_negative(defaults_spacing_xxs, "defaults.spacing.xxs", &mut missing);
        check_non_negative(defaults_spacing_xs, "defaults.spacing.xs", &mut missing);
        check_non_negative(defaults_spacing_s, "defaults.spacing.s", &mut missing);
        check_non_negative(defaults_spacing_m, "defaults.spacing.m", &mut missing);
        check_non_negative(defaults_spacing_l, "defaults.spacing.l", &mut missing);
        check_non_negative(defaults_spacing_xl, "defaults.spacing.xl", &mut missing);
        check_non_negative(defaults_spacing_xxl, "defaults.spacing.xxl", &mut missing);

        // defaults: icon sizes >= 0
        check_non_negative(
            defaults_icon_sizes_toolbar,
            "defaults.icon_sizes.toolbar",
            &mut missing,
        );
        check_non_negative(
            defaults_icon_sizes_small,
            "defaults.icon_sizes.small",
            &mut missing,
        );
        check_non_negative(
            defaults_icon_sizes_large,
            "defaults.icon_sizes.large",
            &mut missing,
        );
        check_non_negative(
            defaults_icon_sizes_dialog,
            "defaults.icon_sizes.dialog",
            &mut missing,
        );
        check_non_negative(
            defaults_icon_sizes_panel,
            "defaults.icon_sizes.panel",
            &mut missing,
        );

        // text_scale: entry sizes > 0, line_height > 0
        check_positive(ts_caption.size, "text_scale.caption.size", &mut missing);
        check_positive(
            ts_caption.line_height,
            "text_scale.caption.line_height",
            &mut missing,
        );
        check_range_u16(
            ts_caption.weight,
            100,
            900,
            "text_scale.caption.weight",
            &mut missing,
        );
        check_positive(
            ts_section_heading.size,
            "text_scale.section_heading.size",
            &mut missing,
        );
        check_positive(
            ts_section_heading.line_height,
            "text_scale.section_heading.line_height",
            &mut missing,
        );
        check_range_u16(
            ts_section_heading.weight,
            100,
            900,
            "text_scale.section_heading.weight",
            &mut missing,
        );
        check_positive(
            ts_dialog_title.size,
            "text_scale.dialog_title.size",
            &mut missing,
        );
        check_positive(
            ts_dialog_title.line_height,
            "text_scale.dialog_title.line_height",
            &mut missing,
        );
        check_range_u16(
            ts_dialog_title.weight,
            100,
            900,
            "text_scale.dialog_title.weight",
            &mut missing,
        );
        check_positive(ts_display.size, "text_scale.display.size", &mut missing);
        check_positive(
            ts_display.line_height,
            "text_scale.display.line_height",
            &mut missing,
        );
        check_range_u16(
            ts_display.weight,
            100,
            900,
            "text_scale.display.weight",
            &mut missing,
        );

        // window: radius >= 0
        check_non_negative(window_radius, "window.radius", &mut missing);

        // window font: size > 0, weight 100..=900
        check_positive(
            window_title_bar_font.size,
            "window.title_bar_font.size",
            &mut missing,
        );
        check_range_u16(
            window_title_bar_font.weight,
            100,
            900,
            "window.title_bar_font.weight",
            &mut missing,
        );

        // button: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
        check_non_negative(button_min_width, "button.min_width", &mut missing);
        check_non_negative(button_min_height, "button.min_height", &mut missing);
        check_non_negative(
            button_padding_horizontal,
            "button.padding_horizontal",
            &mut missing,
        );
        check_non_negative(
            button_padding_vertical,
            "button.padding_vertical",
            &mut missing,
        );
        check_non_negative(button_radius, "button.radius", &mut missing);
        check_non_negative(button_icon_spacing, "button.icon_spacing", &mut missing);
        check_range_f32(
            button_disabled_opacity,
            0.0,
            1.0,
            "button.disabled_opacity",
            &mut missing,
        );
        check_positive(button_font.size, "button.font.size", &mut missing);
        check_range_u16(
            button_font.weight,
            100,
            900,
            "button.font.weight",
            &mut missing,
        );

        // input: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(input_min_height, "input.min_height", &mut missing);
        check_non_negative(
            input_padding_horizontal,
            "input.padding_horizontal",
            &mut missing,
        );
        check_non_negative(
            input_padding_vertical,
            "input.padding_vertical",
            &mut missing,
        );
        check_non_negative(input_radius, "input.radius", &mut missing);
        check_non_negative(input_border_width, "input.border_width", &mut missing);
        check_positive(input_font.size, "input.font.size", &mut missing);
        check_range_u16(
            input_font.weight,
            100,
            900,
            "input.font.weight",
            &mut missing,
        );

        // checkbox: geometry >= 0
        check_non_negative(
            checkbox_indicator_size,
            "checkbox.indicator_size",
            &mut missing,
        );
        check_non_negative(checkbox_spacing, "checkbox.spacing", &mut missing);
        check_non_negative(checkbox_radius, "checkbox.radius", &mut missing);
        check_non_negative(checkbox_border_width, "checkbox.border_width", &mut missing);

        // menu: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(menu_item_height, "menu.item_height", &mut missing);
        check_non_negative(
            menu_padding_horizontal,
            "menu.padding_horizontal",
            &mut missing,
        );
        check_non_negative(menu_padding_vertical, "menu.padding_vertical", &mut missing);
        check_non_negative(menu_icon_spacing, "menu.icon_spacing", &mut missing);
        check_positive(menu_font.size, "menu.font.size", &mut missing);
        check_range_u16(menu_font.weight, 100, 900, "menu.font.weight", &mut missing);

        // tooltip: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(
            tooltip_padding_horizontal,
            "tooltip.padding_horizontal",
            &mut missing,
        );
        check_non_negative(
            tooltip_padding_vertical,
            "tooltip.padding_vertical",
            &mut missing,
        );
        check_non_negative(tooltip_max_width, "tooltip.max_width", &mut missing);
        check_non_negative(tooltip_radius, "tooltip.radius", &mut missing);
        check_positive(tooltip_font.size, "tooltip.font.size", &mut missing);
        check_range_u16(
            tooltip_font.weight,
            100,
            900,
            "tooltip.font.weight",
            &mut missing,
        );

        // scrollbar: geometry >= 0
        check_non_negative(scrollbar_width, "scrollbar.width", &mut missing);
        check_non_negative(
            scrollbar_min_thumb_height,
            "scrollbar.min_thumb_height",
            &mut missing,
        );
        check_non_negative(
            scrollbar_slider_width,
            "scrollbar.slider_width",
            &mut missing,
        );

        // slider: geometry >= 0
        check_non_negative(slider_track_height, "slider.track_height", &mut missing);
        check_non_negative(slider_thumb_size, "slider.thumb_size", &mut missing);
        check_non_negative(slider_tick_length, "slider.tick_length", &mut missing);

        // progress_bar: geometry >= 0
        check_non_negative(progress_bar_height, "progress_bar.height", &mut missing);
        check_non_negative(
            progress_bar_min_width,
            "progress_bar.min_width",
            &mut missing,
        );
        check_non_negative(progress_bar_radius, "progress_bar.radius", &mut missing);

        // tab: geometry >= 0
        check_non_negative(tab_min_width, "tab.min_width", &mut missing);
        check_non_negative(tab_min_height, "tab.min_height", &mut missing);
        check_non_negative(
            tab_padding_horizontal,
            "tab.padding_horizontal",
            &mut missing,
        );
        check_non_negative(tab_padding_vertical, "tab.padding_vertical", &mut missing);

        // toolbar: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(toolbar_height, "toolbar.height", &mut missing);
        check_non_negative(toolbar_item_spacing, "toolbar.item_spacing", &mut missing);
        check_non_negative(toolbar_padding, "toolbar.padding", &mut missing);
        check_positive(toolbar_font.size, "toolbar.font.size", &mut missing);
        check_range_u16(
            toolbar_font.weight,
            100,
            900,
            "toolbar.font.weight",
            &mut missing,
        );

        // status_bar: font size > 0, font weight 100..=900
        check_positive(status_bar_font.size, "status_bar.font.size", &mut missing);
        check_range_u16(
            status_bar_font.weight,
            100,
            900,
            "status_bar.font.weight",
            &mut missing,
        );

        // list: geometry >= 0
        check_non_negative(list_item_height, "list.item_height", &mut missing);
        check_non_negative(
            list_padding_horizontal,
            "list.padding_horizontal",
            &mut missing,
        );
        check_non_negative(list_padding_vertical, "list.padding_vertical", &mut missing);

        // popover: radius >= 0
        check_non_negative(popover_radius, "popover.radius", &mut missing);

        // splitter: width >= 0
        check_non_negative(splitter_width, "splitter.width", &mut missing);

        // switch: geometry >= 0
        check_non_negative(switch_track_width, "switch.track_width", &mut missing);
        check_non_negative(switch_track_height, "switch.track_height", &mut missing);
        check_non_negative(switch_thumb_size, "switch.thumb_size", &mut missing);
        check_non_negative(switch_track_radius, "switch.track_radius", &mut missing);

        // dialog: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(dialog_min_width, "dialog.min_width", &mut missing);
        check_non_negative(dialog_max_width, "dialog.max_width", &mut missing);
        check_non_negative(dialog_min_height, "dialog.min_height", &mut missing);
        check_non_negative(dialog_max_height, "dialog.max_height", &mut missing);
        check_non_negative(
            dialog_content_padding,
            "dialog.content_padding",
            &mut missing,
        );
        check_non_negative(dialog_button_spacing, "dialog.button_spacing", &mut missing);
        check_non_negative(dialog_radius, "dialog.radius", &mut missing);
        check_non_negative(dialog_icon_size, "dialog.icon_size", &mut missing);
        check_positive(
            dialog_title_font.size,
            "dialog.title_font.size",
            &mut missing,
        );
        check_range_u16(
            dialog_title_font.weight,
            100,
            900,
            "dialog.title_font.weight",
            &mut missing,
        );

        // spinner: geometry >= 0
        check_non_negative(spinner_diameter, "spinner.diameter", &mut missing);
        check_non_negative(spinner_min_size, "spinner.min_size", &mut missing);
        check_non_negative(spinner_stroke_width, "spinner.stroke_width", &mut missing);

        // combo_box: geometry >= 0
        check_non_negative(combo_box_min_height, "combo_box.min_height", &mut missing);
        check_non_negative(combo_box_min_width, "combo_box.min_width", &mut missing);
        check_non_negative(
            combo_box_padding_horizontal,
            "combo_box.padding_horizontal",
            &mut missing,
        );
        check_non_negative(combo_box_arrow_size, "combo_box.arrow_size", &mut missing);
        check_non_negative(
            combo_box_arrow_area_width,
            "combo_box.arrow_area_width",
            &mut missing,
        );
        check_non_negative(combo_box_radius, "combo_box.radius", &mut missing);

        // segmented_control: geometry >= 0
        check_non_negative(
            segmented_control_segment_height,
            "segmented_control.segment_height",
            &mut missing,
        );
        check_non_negative(
            segmented_control_separator_width,
            "segmented_control.separator_width",
            &mut missing,
        );
        check_non_negative(
            segmented_control_padding_horizontal,
            "segmented_control.padding_horizontal",
            &mut missing,
        );
        check_non_negative(
            segmented_control_radius,
            "segmented_control.radius",
            &mut missing,
        );

        // card: geometry >= 0
        check_non_negative(card_radius, "card.radius", &mut missing);
        check_non_negative(card_padding, "card.padding", &mut missing);

        // expander: geometry >= 0
        check_non_negative(
            expander_header_height,
            "expander.header_height",
            &mut missing,
        );
        check_non_negative(expander_arrow_size, "expander.arrow_size", &mut missing);
        check_non_negative(
            expander_content_padding,
            "expander.content_padding",
            &mut missing,
        );
        check_non_negative(expander_radius, "expander.radius", &mut missing);

        // --- check for missing fields and range errors ---

        if !missing.is_empty() {
            return Err(crate::Error::Resolution(ThemeResolutionError {
                missing_fields: missing,
            }));
        }

        // All fields present -- construct ResolvedThemeVariant.
        // require() returns T directly (using T::default() as placeholder for missing),
        // so no unwrap() is needed. The defaults are never used: we returned Err above.
        Ok(ResolvedThemeVariant {
            defaults: ResolvedThemeDefaults {
                font: defaults_font,
                line_height: defaults_line_height,
                mono_font: defaults_mono_font,
                background: defaults_background,
                foreground: defaults_foreground,
                accent: defaults_accent,
                accent_foreground: defaults_accent_foreground,
                surface: defaults_surface,
                border: defaults_border,
                muted: defaults_muted,
                shadow: defaults_shadow,
                link: defaults_link,
                selection: defaults_selection,
                selection_foreground: defaults_selection_foreground,
                selection_inactive: defaults_selection_inactive,
                disabled_foreground: defaults_disabled_foreground,
                danger: defaults_danger,
                danger_foreground: defaults_danger_foreground,
                warning: defaults_warning,
                warning_foreground: defaults_warning_foreground,
                success: defaults_success,
                success_foreground: defaults_success_foreground,
                info: defaults_info,
                info_foreground: defaults_info_foreground,
                radius: defaults_radius,
                radius_lg: defaults_radius_lg,
                frame_width: defaults_frame_width,
                disabled_opacity: defaults_disabled_opacity,
                border_opacity: defaults_border_opacity,
                shadow_enabled: defaults_shadow_enabled,
                focus_ring_color: defaults_focus_ring_color,
                focus_ring_width: defaults_focus_ring_width,
                focus_ring_offset: defaults_focus_ring_offset,
                spacing: ResolvedThemeSpacing {
                    xxs: defaults_spacing_xxs,
                    xs: defaults_spacing_xs,
                    s: defaults_spacing_s,
                    m: defaults_spacing_m,
                    l: defaults_spacing_l,
                    xl: defaults_spacing_xl,
                    xxl: defaults_spacing_xxl,
                },
                icon_sizes: ResolvedIconSizes {
                    toolbar: defaults_icon_sizes_toolbar,
                    small: defaults_icon_sizes_small,
                    large: defaults_icon_sizes_large,
                    dialog: defaults_icon_sizes_dialog,
                    panel: defaults_icon_sizes_panel,
                },
                text_scaling_factor: defaults_text_scaling_factor,
                reduce_motion: defaults_reduce_motion,
                high_contrast: defaults_high_contrast,
                reduce_transparency: defaults_reduce_transparency,
            },
            text_scale: ResolvedTextScale {
                caption: ts_caption,
                section_heading: ts_section_heading,
                dialog_title: ts_dialog_title,
                display: ts_display,
            },
            window: crate::model::widgets::ResolvedWindowTheme {
                background: window_background,
                foreground: window_foreground,
                border: window_border,
                title_bar_background: window_title_bar_background,
                title_bar_foreground: window_title_bar_foreground,
                inactive_title_bar_background: window_inactive_title_bar_background,
                inactive_title_bar_foreground: window_inactive_title_bar_foreground,
                radius: window_radius,
                shadow: window_shadow,
                title_bar_font: window_title_bar_font,
            },
            button: crate::model::widgets::ResolvedButtonTheme {
                background: button_background,
                foreground: button_foreground,
                border: button_border,
                primary_background: button_primary_background,
                primary_foreground: button_primary_foreground,
                min_width: button_min_width,
                min_height: button_min_height,
                padding_horizontal: button_padding_horizontal,
                padding_vertical: button_padding_vertical,
                radius: button_radius,
                icon_spacing: button_icon_spacing,
                disabled_opacity: button_disabled_opacity,
                shadow: button_shadow,
                font: button_font,
            },
            input: crate::model::widgets::ResolvedInputTheme {
                background: input_background,
                foreground: input_foreground,
                border: input_border,
                placeholder: input_placeholder,
                caret: input_caret,
                selection: input_selection,
                selection_foreground: input_selection_foreground,
                min_height: input_min_height,
                padding_horizontal: input_padding_horizontal,
                padding_vertical: input_padding_vertical,
                radius: input_radius,
                border_width: input_border_width,
                font: input_font,
            },
            checkbox: crate::model::widgets::ResolvedCheckboxTheme {
                checked_bg: checkbox_checked_bg,
                indicator_size: checkbox_indicator_size,
                spacing: checkbox_spacing,
                radius: checkbox_radius,
                border_width: checkbox_border_width,
            },
            menu: crate::model::widgets::ResolvedMenuTheme {
                background: menu_background,
                foreground: menu_foreground,
                separator: menu_separator,
                item_height: menu_item_height,
                padding_horizontal: menu_padding_horizontal,
                padding_vertical: menu_padding_vertical,
                icon_spacing: menu_icon_spacing,
                font: menu_font,
            },
            tooltip: crate::model::widgets::ResolvedTooltipTheme {
                background: tooltip_background,
                foreground: tooltip_foreground,
                padding_horizontal: tooltip_padding_horizontal,
                padding_vertical: tooltip_padding_vertical,
                max_width: tooltip_max_width,
                radius: tooltip_radius,
                font: tooltip_font,
            },
            scrollbar: crate::model::widgets::ResolvedScrollbarTheme {
                track: scrollbar_track,
                thumb: scrollbar_thumb,
                thumb_hover: scrollbar_thumb_hover,
                width: scrollbar_width,
                min_thumb_height: scrollbar_min_thumb_height,
                slider_width: scrollbar_slider_width,
                overlay_mode: scrollbar_overlay_mode,
            },
            slider: crate::model::widgets::ResolvedSliderTheme {
                fill: slider_fill,
                track: slider_track,
                thumb: slider_thumb,
                track_height: slider_track_height,
                thumb_size: slider_thumb_size,
                tick_length: slider_tick_length,
            },
            progress_bar: crate::model::widgets::ResolvedProgressBarTheme {
                fill: progress_bar_fill,
                track: progress_bar_track,
                height: progress_bar_height,
                min_width: progress_bar_min_width,
                radius: progress_bar_radius,
            },
            tab: crate::model::widgets::ResolvedTabTheme {
                background: tab_background,
                foreground: tab_foreground,
                active_background: tab_active_background,
                active_foreground: tab_active_foreground,
                bar_background: tab_bar_background,
                min_width: tab_min_width,
                min_height: tab_min_height,
                padding_horizontal: tab_padding_horizontal,
                padding_vertical: tab_padding_vertical,
            },
            sidebar: crate::model::widgets::ResolvedSidebarTheme {
                background: sidebar_background,
                foreground: sidebar_foreground,
            },
            toolbar: crate::model::widgets::ResolvedToolbarTheme {
                height: toolbar_height,
                item_spacing: toolbar_item_spacing,
                padding: toolbar_padding,
                font: toolbar_font,
            },
            status_bar: crate::model::widgets::ResolvedStatusBarTheme {
                font: status_bar_font,
            },
            list: crate::model::widgets::ResolvedListTheme {
                background: list_background,
                foreground: list_foreground,
                alternate_row: list_alternate_row,
                selection: list_selection,
                selection_foreground: list_selection_foreground,
                header_background: list_header_background,
                header_foreground: list_header_foreground,
                grid_color: list_grid_color,
                item_height: list_item_height,
                padding_horizontal: list_padding_horizontal,
                padding_vertical: list_padding_vertical,
            },
            popover: crate::model::widgets::ResolvedPopoverTheme {
                background: popover_background,
                foreground: popover_foreground,
                border: popover_border,
                radius: popover_radius,
            },
            splitter: crate::model::widgets::ResolvedSplitterTheme {
                width: splitter_width,
            },
            separator: crate::model::widgets::ResolvedSeparatorTheme {
                color: separator_color,
            },
            switch: crate::model::widgets::ResolvedSwitchTheme {
                checked_background: switch_checked_background,
                unchecked_background: switch_unchecked_background,
                thumb_background: switch_thumb_background,
                track_width: switch_track_width,
                track_height: switch_track_height,
                thumb_size: switch_thumb_size,
                track_radius: switch_track_radius,
            },
            dialog: crate::model::widgets::ResolvedDialogTheme {
                min_width: dialog_min_width,
                max_width: dialog_max_width,
                min_height: dialog_min_height,
                max_height: dialog_max_height,
                content_padding: dialog_content_padding,
                button_spacing: dialog_button_spacing,
                radius: dialog_radius,
                icon_size: dialog_icon_size,
                button_order: dialog_button_order,
                title_font: dialog_title_font,
            },
            spinner: crate::model::widgets::ResolvedSpinnerTheme {
                fill: spinner_fill,
                diameter: spinner_diameter,
                min_size: spinner_min_size,
                stroke_width: spinner_stroke_width,
            },
            combo_box: crate::model::widgets::ResolvedComboBoxTheme {
                min_height: combo_box_min_height,
                min_width: combo_box_min_width,
                padding_horizontal: combo_box_padding_horizontal,
                arrow_size: combo_box_arrow_size,
                arrow_area_width: combo_box_arrow_area_width,
                radius: combo_box_radius,
            },
            segmented_control: crate::model::widgets::ResolvedSegmentedControlTheme {
                segment_height: segmented_control_segment_height,
                separator_width: segmented_control_separator_width,
                padding_horizontal: segmented_control_padding_horizontal,
                radius: segmented_control_radius,
            },
            card: crate::model::widgets::ResolvedCardTheme {
                background: card_background,
                border: card_border,
                radius: card_radius,
                padding: card_padding,
                shadow: card_shadow,
            },
            expander: crate::model::widgets::ResolvedExpanderTheme {
                header_height: expander_header_height,
                arrow_size: expander_arrow_size,
                content_padding: expander_content_padding,
                radius: expander_radius,
            },
            link: crate::model::widgets::ResolvedLinkTheme {
                color: link_color,
                visited: link_visited,
                background: link_background,
                hover_bg: link_hover_bg,
                underline: link_underline,
            },
            icon_set,
            icon_theme,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::Rgba;
    use crate::model::{DialogButtonOrder, FontSpec};

    /// Helper: build a ThemeVariant with all defaults.* fields populated.
    fn variant_with_defaults() -> ThemeVariant {
        let c1 = Rgba::rgb(0, 120, 215); // accent
        let c2 = Rgba::rgb(255, 255, 255); // background
        let c3 = Rgba::rgb(30, 30, 30); // foreground
        let c4 = Rgba::rgb(240, 240, 240); // surface
        let c5 = Rgba::rgb(200, 200, 200); // border
        let c6 = Rgba::rgb(128, 128, 128); // muted
        let c7 = Rgba::rgb(0, 0, 0); // shadow
        let c8 = Rgba::rgb(0, 100, 200); // link
        let c9 = Rgba::rgb(255, 255, 255); // accent_foreground
        let c10 = Rgba::rgb(220, 53, 69); // danger
        let c11 = Rgba::rgb(255, 255, 255); // danger_foreground
        let c12 = Rgba::rgb(240, 173, 78); // warning
        let c13 = Rgba::rgb(30, 30, 30); // warning_foreground
        let c14 = Rgba::rgb(40, 167, 69); // success
        let c15 = Rgba::rgb(255, 255, 255); // success_foreground
        let c16 = Rgba::rgb(0, 120, 215); // info
        let c17 = Rgba::rgb(255, 255, 255); // info_foreground

        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(c1);
        v.defaults.background = Some(c2);
        v.defaults.foreground = Some(c3);
        v.defaults.surface = Some(c4);
        v.defaults.border = Some(c5);
        v.defaults.muted = Some(c6);
        v.defaults.shadow = Some(c7);
        v.defaults.link = Some(c8);
        v.defaults.accent_foreground = Some(c9);
        v.defaults.selection_foreground = Some(Rgba::rgb(255, 255, 255));
        v.defaults.disabled_foreground = Some(Rgba::rgb(160, 160, 160));
        v.defaults.danger = Some(c10);
        v.defaults.danger_foreground = Some(c11);
        v.defaults.warning = Some(c12);
        v.defaults.warning_foreground = Some(c13);
        v.defaults.success = Some(c14);
        v.defaults.success_foreground = Some(c15);
        v.defaults.info = Some(c16);
        v.defaults.info_foreground = Some(c17);

        v.defaults.radius = Some(4.0);
        v.defaults.radius_lg = Some(8.0);
        v.defaults.frame_width = Some(1.0);
        v.defaults.disabled_opacity = Some(0.5);
        v.defaults.border_opacity = Some(0.15);
        v.defaults.shadow_enabled = Some(true);

        v.defaults.focus_ring_width = Some(2.0);
        v.defaults.focus_ring_offset = Some(1.0);

        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        v.defaults.line_height = Some(1.4);
        v.defaults.mono_font = FontSpec {
            family: Some("JetBrains Mono".into()),
            size: Some(13.0),
            weight: Some(400),
        };

        v.defaults.spacing.xxs = Some(2.0);
        v.defaults.spacing.xs = Some(4.0);
        v.defaults.spacing.s = Some(6.0);
        v.defaults.spacing.m = Some(12.0);
        v.defaults.spacing.l = Some(18.0);
        v.defaults.spacing.xl = Some(24.0);
        v.defaults.spacing.xxl = Some(36.0);

        v.defaults.icon_sizes.toolbar = Some(24.0);
        v.defaults.icon_sizes.small = Some(16.0);
        v.defaults.icon_sizes.large = Some(32.0);
        v.defaults.icon_sizes.dialog = Some(22.0);
        v.defaults.icon_sizes.panel = Some(20.0);

        v.defaults.text_scaling_factor = Some(1.0);
        v.defaults.reduce_motion = Some(false);
        v.defaults.high_contrast = Some(false);
        v.defaults.reduce_transparency = Some(false);

        v
    }

    // ===== Phase 1: Defaults internal chains =====

    #[test]
    fn resolve_phase1_accent_fills_selection_and_focus_ring() {
        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        v.resolve();
        assert_eq!(v.defaults.selection, Some(Rgba::rgb(0, 120, 215)));
        assert_eq!(v.defaults.focus_ring_color, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn resolve_phase1_selection_fills_selection_inactive() {
        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        v.resolve();
        // selection_inactive should be set from selection (which was set from accent)
        assert_eq!(v.defaults.selection_inactive, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn resolve_phase1_explicit_selection_preserved() {
        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        v.defaults.selection = Some(Rgba::rgb(100, 100, 100));
        v.resolve();
        // Explicit selection preserved
        assert_eq!(v.defaults.selection, Some(Rgba::rgb(100, 100, 100)));
        // selection_inactive inherits from the explicit selection
        assert_eq!(
            v.defaults.selection_inactive,
            Some(Rgba::rgb(100, 100, 100))
        );
    }

    // ===== Phase 2: Safety nets =====

    #[test]
    fn resolve_phase2_safety_nets() {
        let mut v = ThemeVariant::default();
        v.defaults.foreground = Some(Rgba::rgb(30, 30, 30));
        v.defaults.background = Some(Rgba::rgb(255, 255, 255));
        v.resolve();

        assert_eq!(
            v.input.caret,
            Some(Rgba::rgb(30, 30, 30)),
            "input.caret <- foreground"
        );
        assert_eq!(
            v.scrollbar.track,
            Some(Rgba::rgb(255, 255, 255)),
            "scrollbar.track <- background"
        );
        assert_eq!(
            v.spinner.fill,
            Some(Rgba::rgb(30, 30, 30)),
            "spinner.fill <- foreground"
        );
        assert_eq!(
            v.popover.background,
            Some(Rgba::rgb(255, 255, 255)),
            "popover.background <- background"
        );
        assert_eq!(
            v.list.background,
            Some(Rgba::rgb(255, 255, 255)),
            "list.background <- background"
        );
    }

    // ===== Phase 3: Accent propagation (RESOLVE-06) =====

    #[test]
    fn resolve_phase3_accent_propagation() {
        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        v.resolve();

        assert_eq!(
            v.button.primary_background,
            Some(Rgba::rgb(0, 120, 215)),
            "button.primary_background <- accent"
        );
        assert_eq!(
            v.checkbox.checked_bg,
            Some(Rgba::rgb(0, 120, 215)),
            "checkbox.checked_bg <- accent"
        );
        assert_eq!(
            v.slider.fill,
            Some(Rgba::rgb(0, 120, 215)),
            "slider.fill <- accent"
        );
        assert_eq!(
            v.progress_bar.fill,
            Some(Rgba::rgb(0, 120, 215)),
            "progress_bar.fill <- accent"
        );
        assert_eq!(
            v.switch.checked_background,
            Some(Rgba::rgb(0, 120, 215)),
            "switch.checked_background <- accent"
        );
    }

    // ===== Phase 3: Font sub-field inheritance (RESOLVE-04) =====

    #[test]
    fn resolve_phase3_font_subfield_inheritance() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        // Menu has a font with only size set
        v.menu.font = Some(FontSpec {
            family: None,
            size: Some(12.0),
            weight: None,
        });
        v.resolve();

        let menu_font = v.menu.font.as_ref().unwrap();
        assert_eq!(
            menu_font.family.as_deref(),
            Some("Inter"),
            "family from defaults"
        );
        assert_eq!(menu_font.size, Some(12.0), "explicit size preserved");
        assert_eq!(menu_font.weight, Some(400), "weight from defaults");
    }

    #[test]
    fn resolve_phase3_font_entire_inheritance() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        // button.font is None, should inherit entire defaults.font
        assert!(v.button.font.is_none());
        v.resolve();

        let button_font = v.button.font.as_ref().unwrap();
        assert_eq!(button_font.family.as_deref(), Some("Inter"));
        assert_eq!(button_font.size, Some(14.0));
        assert_eq!(button_font.weight, Some(400));
    }

    // ===== Phase 3: Text scale inheritance (RESOLVE-05) =====

    #[test]
    fn resolve_phase3_text_scale_inheritance() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        v.defaults.line_height = Some(1.4);
        // Leave text_scale.caption as None
        v.resolve();

        let caption = v.text_scale.caption.as_ref().unwrap();
        assert_eq!(caption.size, Some(14.0), "size from defaults.font.size");
        assert_eq!(
            caption.weight,
            Some(400),
            "weight from defaults.font.weight"
        );
        // line_height = defaults.line_height * resolved_size = 1.4 * 14.0 = 19.6
        assert!(
            (caption.line_height.unwrap() - 19.6).abs() < 0.001,
            "line_height computed"
        );
    }

    // ===== Phase 3: Color inheritance =====

    #[test]
    fn resolve_phase3_color_inheritance() {
        let mut v = variant_with_defaults();
        v.resolve();

        // window
        assert_eq!(v.window.background, Some(Rgba::rgb(255, 255, 255)));
        assert_eq!(v.window.border, v.defaults.border);
        // button
        assert_eq!(v.button.border, v.defaults.border);
        // tooltip
        assert_eq!(v.tooltip.radius, v.defaults.radius);
    }

    // ===== Phase 4: Widget-to-widget =====

    #[test]
    fn resolve_phase4_inactive_title_bar_from_active() {
        let mut v = ThemeVariant::default();
        v.defaults.surface = Some(Rgba::rgb(240, 240, 240));
        v.defaults.foreground = Some(Rgba::rgb(30, 30, 30));
        v.resolve();

        // title_bar_background was set to surface in Phase 3
        // inactive should inherit from active
        assert_eq!(
            v.window.inactive_title_bar_background,
            v.window.title_bar_background
        );
        assert_eq!(
            v.window.inactive_title_bar_foreground,
            v.window.title_bar_foreground
        );
    }

    // ===== Preserve explicit values =====

    #[test]
    fn resolve_does_not_overwrite_existing_some_values() {
        let mut v = variant_with_defaults();
        let explicit = Rgba::rgb(255, 0, 0);
        v.window.background = Some(explicit);
        v.button.primary_background = Some(explicit);
        v.resolve();

        assert_eq!(
            v.window.background,
            Some(explicit),
            "window.background preserved"
        );
        assert_eq!(
            v.button.primary_background,
            Some(explicit),
            "button.primary_background preserved"
        );
    }

    // ===== Idempotent =====

    #[test]
    fn resolve_is_idempotent() {
        let mut v = variant_with_defaults();
        v.resolve();
        let after_first = v.clone();
        v.resolve();
        assert_eq!(v, after_first, "second resolve() produces same result");
    }

    // ===== All 8 font-carrying widgets get resolved fonts =====

    #[test]
    fn resolve_all_font_carrying_widgets_get_resolved_fonts() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        v.resolve();

        // All 8 should now have Some(FontSpec)
        assert!(v.window.title_bar_font.is_some(), "window.title_bar_font");
        assert!(v.button.font.is_some(), "button.font");
        assert!(v.input.font.is_some(), "input.font");
        assert!(v.menu.font.is_some(), "menu.font");
        assert!(v.tooltip.font.is_some(), "tooltip.font");
        assert!(v.toolbar.font.is_some(), "toolbar.font");
        assert!(v.status_bar.font.is_some(), "status_bar.font");
        assert!(v.dialog.title_font.is_some(), "dialog.title_font");

        // Each should have the defaults values
        for (name, font) in [
            ("window.title_bar_font", &v.window.title_bar_font),
            ("button.font", &v.button.font),
            ("input.font", &v.input.font),
            ("menu.font", &v.menu.font),
            ("tooltip.font", &v.tooltip.font),
            ("toolbar.font", &v.toolbar.font),
            ("status_bar.font", &v.status_bar.font),
            ("dialog.title_font", &v.dialog.title_font),
        ] {
            let f = font.as_ref().unwrap();
            assert_eq!(f.family.as_deref(), Some("Inter"), "{name} family");
            assert_eq!(f.size, Some(14.0), "{name} size");
            assert_eq!(f.weight, Some(400), "{name} weight");
        }
    }

    // ===== validate() tests =====

    /// Build a fully-populated ThemeVariant (all fields Some) for validate() testing.
    fn fully_populated_variant() -> ThemeVariant {
        let mut v = variant_with_defaults();
        let c = Rgba::rgb(128, 128, 128);

        // Ensure derived defaults are set (variant_with_defaults doesn't set these)
        v.defaults.selection = Some(Rgba::rgb(0, 120, 215));
        v.defaults.selection_foreground = Some(Rgba::rgb(255, 255, 255));
        v.defaults.selection_inactive = Some(Rgba::rgb(0, 120, 215));
        v.defaults.focus_ring_color = Some(Rgba::rgb(0, 120, 215));

        // icon_set / icon_theme
        v.icon_set = Some(crate::IconSet::Freedesktop);
        v.icon_theme = Some("breeze".into());

        // window
        v.window.background = Some(c);
        v.window.foreground = Some(c);
        v.window.border = Some(c);
        v.window.title_bar_background = Some(c);
        v.window.title_bar_foreground = Some(c);
        v.window.inactive_title_bar_background = Some(c);
        v.window.inactive_title_bar_foreground = Some(c);
        v.window.radius = Some(8.0);
        v.window.shadow = Some(true);
        v.window.title_bar_font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        });

        // button
        v.button.background = Some(c);
        v.button.foreground = Some(c);
        v.button.border = Some(c);
        v.button.primary_background = Some(c);
        v.button.primary_foreground = Some(c);
        v.button.min_width = Some(64.0);
        v.button.min_height = Some(28.0);
        v.button.padding_horizontal = Some(12.0);
        v.button.padding_vertical = Some(6.0);
        v.button.radius = Some(4.0);
        v.button.icon_spacing = Some(6.0);
        v.button.disabled_opacity = Some(0.5);
        v.button.shadow = Some(false);
        v.button.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        });

        // input
        v.input.background = Some(c);
        v.input.foreground = Some(c);
        v.input.border = Some(c);
        v.input.placeholder = Some(c);
        v.input.caret = Some(c);
        v.input.selection = Some(c);
        v.input.selection_foreground = Some(c);
        v.input.min_height = Some(28.0);
        v.input.padding_horizontal = Some(8.0);
        v.input.padding_vertical = Some(4.0);
        v.input.radius = Some(4.0);
        v.input.border_width = Some(1.0);
        v.input.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        });

        // checkbox
        v.checkbox.checked_bg = Some(c);
        v.checkbox.indicator_size = Some(18.0);
        v.checkbox.spacing = Some(6.0);
        v.checkbox.radius = Some(2.0);
        v.checkbox.border_width = Some(1.0);

        // menu
        v.menu.background = Some(c);
        v.menu.foreground = Some(c);
        v.menu.separator = Some(c);
        v.menu.item_height = Some(28.0);
        v.menu.padding_horizontal = Some(8.0);
        v.menu.padding_vertical = Some(4.0);
        v.menu.icon_spacing = Some(6.0);
        v.menu.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        });

        // tooltip
        v.tooltip.background = Some(c);
        v.tooltip.foreground = Some(c);
        v.tooltip.padding_horizontal = Some(6.0);
        v.tooltip.padding_vertical = Some(4.0);
        v.tooltip.max_width = Some(300.0);
        v.tooltip.radius = Some(4.0);
        v.tooltip.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        });

        // scrollbar
        v.scrollbar.track = Some(c);
        v.scrollbar.thumb = Some(c);
        v.scrollbar.thumb_hover = Some(c);
        v.scrollbar.width = Some(14.0);
        v.scrollbar.min_thumb_height = Some(20.0);
        v.scrollbar.slider_width = Some(8.0);
        v.scrollbar.overlay_mode = Some(false);

        // slider
        v.slider.fill = Some(c);
        v.slider.track = Some(c);
        v.slider.thumb = Some(c);
        v.slider.track_height = Some(4.0);
        v.slider.thumb_size = Some(16.0);
        v.slider.tick_length = Some(6.0);

        // progress_bar
        v.progress_bar.fill = Some(c);
        v.progress_bar.track = Some(c);
        v.progress_bar.height = Some(6.0);
        v.progress_bar.min_width = Some(100.0);
        v.progress_bar.radius = Some(3.0);

        // tab
        v.tab.background = Some(c);
        v.tab.foreground = Some(c);
        v.tab.active_background = Some(c);
        v.tab.active_foreground = Some(c);
        v.tab.bar_background = Some(c);
        v.tab.min_width = Some(60.0);
        v.tab.min_height = Some(32.0);
        v.tab.padding_horizontal = Some(12.0);
        v.tab.padding_vertical = Some(6.0);

        // sidebar
        v.sidebar.background = Some(c);
        v.sidebar.foreground = Some(c);

        // toolbar
        v.toolbar.height = Some(40.0);
        v.toolbar.item_spacing = Some(4.0);
        v.toolbar.padding = Some(4.0);
        v.toolbar.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        });

        // status_bar
        v.status_bar.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        });

        // list
        v.list.background = Some(c);
        v.list.foreground = Some(c);
        v.list.alternate_row = Some(c);
        v.list.selection = Some(c);
        v.list.selection_foreground = Some(c);
        v.list.header_background = Some(c);
        v.list.header_foreground = Some(c);
        v.list.grid_color = Some(c);
        v.list.item_height = Some(28.0);
        v.list.padding_horizontal = Some(8.0);
        v.list.padding_vertical = Some(4.0);

        // popover
        v.popover.background = Some(c);
        v.popover.foreground = Some(c);
        v.popover.border = Some(c);
        v.popover.radius = Some(6.0);

        // splitter
        v.splitter.width = Some(4.0);

        // separator
        v.separator.color = Some(c);

        // switch
        v.switch.checked_background = Some(c);
        v.switch.unchecked_background = Some(c);
        v.switch.thumb_background = Some(c);
        v.switch.track_width = Some(40.0);
        v.switch.track_height = Some(20.0);
        v.switch.thumb_size = Some(14.0);
        v.switch.track_radius = Some(10.0);

        // dialog
        v.dialog.min_width = Some(320.0);
        v.dialog.max_width = Some(600.0);
        v.dialog.min_height = Some(200.0);
        v.dialog.max_height = Some(800.0);
        v.dialog.content_padding = Some(16.0);
        v.dialog.button_spacing = Some(8.0);
        v.dialog.radius = Some(8.0);
        v.dialog.icon_size = Some(22.0);
        v.dialog.button_order = Some(DialogButtonOrder::TrailingAffirmative);
        v.dialog.title_font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(16.0),
            weight: Some(700),
        });

        // spinner
        v.spinner.fill = Some(c);
        v.spinner.diameter = Some(24.0);
        v.spinner.min_size = Some(16.0);
        v.spinner.stroke_width = Some(2.0);

        // combo_box
        v.combo_box.min_height = Some(28.0);
        v.combo_box.min_width = Some(80.0);
        v.combo_box.padding_horizontal = Some(8.0);
        v.combo_box.arrow_size = Some(12.0);
        v.combo_box.arrow_area_width = Some(20.0);
        v.combo_box.radius = Some(4.0);

        // segmented_control
        v.segmented_control.segment_height = Some(28.0);
        v.segmented_control.separator_width = Some(1.0);
        v.segmented_control.padding_horizontal = Some(12.0);
        v.segmented_control.radius = Some(4.0);

        // card
        v.card.background = Some(c);
        v.card.border = Some(c);
        v.card.radius = Some(8.0);
        v.card.padding = Some(12.0);
        v.card.shadow = Some(true);

        // expander
        v.expander.header_height = Some(32.0);
        v.expander.arrow_size = Some(12.0);
        v.expander.content_padding = Some(8.0);
        v.expander.radius = Some(4.0);

        // link
        v.link.color = Some(c);
        v.link.visited = Some(c);
        v.link.background = Some(c);
        v.link.hover_bg = Some(c);
        v.link.underline = Some(true);

        // text_scale (all 4 entries fully populated)
        v.text_scale.caption = Some(crate::model::TextScaleEntry {
            size: Some(11.0),
            weight: Some(400),
            line_height: Some(15.4),
        });
        v.text_scale.section_heading = Some(crate::model::TextScaleEntry {
            size: Some(14.0),
            weight: Some(600),
            line_height: Some(19.6),
        });
        v.text_scale.dialog_title = Some(crate::model::TextScaleEntry {
            size: Some(16.0),
            weight: Some(700),
            line_height: Some(22.4),
        });
        v.text_scale.display = Some(crate::model::TextScaleEntry {
            size: Some(24.0),
            weight: Some(300),
            line_height: Some(33.6),
        });

        v
    }

    #[test]
    fn validate_fully_populated_returns_ok() {
        let v = fully_populated_variant();
        let result = v.validate();
        assert!(
            result.is_ok(),
            "validate() should succeed on fully populated variant, got: {:?}",
            result.err()
        );
        let resolved = result.unwrap();
        assert_eq!(resolved.defaults.font.family, "Inter");
        assert_eq!(resolved.icon_set, crate::IconSet::Freedesktop);
    }

    #[test]
    fn validate_missing_3_fields_returns_all_paths() {
        let mut v = fully_populated_variant();
        // Remove 3 specific fields (non-cascading)
        v.defaults.muted = None;
        v.window.radius = None;
        v.icon_set = None;

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert_eq!(
            err.missing_fields.len(),
            3,
            "should report exactly 3 missing fields, got: {:?}",
            err.missing_fields
        );
        assert!(err.missing_fields.contains(&"defaults.muted".to_string()));
        assert!(err.missing_fields.contains(&"window.radius".to_string()));
        assert!(err.missing_fields.contains(&"icon_set".to_string()));
    }

    #[test]
    fn validate_error_message_includes_count_and_paths() {
        let mut v = fully_populated_variant();
        v.defaults.muted = None;
        v.button.min_height = None;

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        let msg = err.to_string();
        assert!(msg.contains("2 missing field(s)"), "got: {msg}");
        assert!(msg.contains("defaults.muted"), "got: {msg}");
        assert!(msg.contains("button.min_height"), "got: {msg}");
    }

    #[test]
    fn validate_checks_all_defaults_fields() {
        // Default variant has ALL fields None, so validate should report many missing
        let v = ThemeVariant::default();
        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        // Should include defaults fields
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.starts_with("defaults.")),
            "should include defaults.* fields in missing"
        );
        // Check a representative set of defaults fields
        assert!(
            err.missing_fields
                .contains(&"defaults.font.family".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"defaults.background".to_string())
        );
        assert!(err.missing_fields.contains(&"defaults.accent".to_string()));
        assert!(err.missing_fields.contains(&"defaults.radius".to_string()));
        assert!(
            err.missing_fields
                .contains(&"defaults.spacing.m".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"defaults.icon_sizes.toolbar".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"defaults.text_scaling_factor".to_string())
        );
    }

    #[test]
    fn validate_checks_all_widget_structs() {
        let v = ThemeVariant::default();
        let result = v.validate();
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        // Every widget should have at least one field reported
        for prefix in [
            "window.",
            "button.",
            "input.",
            "checkbox.",
            "menu.",
            "tooltip.",
            "scrollbar.",
            "slider.",
            "progress_bar.",
            "tab.",
            "sidebar.",
            "toolbar.",
            "status_bar.",
            "list.",
            "popover.",
            "splitter.",
            "separator.",
            "switch.",
            "dialog.",
            "spinner.",
            "combo_box.",
            "segmented_control.",
            "card.",
            "expander.",
            "link.",
        ] {
            assert!(
                err.missing_fields.iter().any(|f| f.starts_with(prefix)),
                "missing fields should include {prefix}* but got: {:?}",
                err.missing_fields
                    .iter()
                    .filter(|f| f.starts_with(prefix))
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn validate_checks_text_scale_entries() {
        let v = ThemeVariant::default();
        let result = v.validate();
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .contains(&"text_scale.caption".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"text_scale.section_heading".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"text_scale.dialog_title".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"text_scale.display".to_string())
        );
    }

    #[test]
    fn validate_checks_icon_set() {
        let mut v = fully_populated_variant();
        v.icon_set = None;

        let result = v.validate();
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(err.missing_fields.contains(&"icon_set".to_string()));
    }

    #[test]
    fn validate_after_resolve_succeeds_for_derivable_fields() {
        // Start with defaults populated but widgets empty
        let mut v = variant_with_defaults();
        // Add non-derivable widget sizing fields
        v.icon_set = Some(crate::IconSet::Freedesktop);

        // Non-derivable fields that resolve() cannot fill:
        // button sizing
        v.button.min_width = Some(64.0);
        v.button.min_height = Some(28.0);
        v.button.padding_horizontal = Some(12.0);
        v.button.padding_vertical = Some(6.0);
        v.button.icon_spacing = Some(6.0);
        // input sizing
        v.input.min_height = Some(28.0);
        v.input.padding_horizontal = Some(8.0);
        v.input.padding_vertical = Some(4.0);
        // checkbox sizing
        v.checkbox.indicator_size = Some(18.0);
        v.checkbox.spacing = Some(6.0);
        // menu sizing
        v.menu.item_height = Some(28.0);
        v.menu.padding_horizontal = Some(8.0);
        v.menu.padding_vertical = Some(4.0);
        v.menu.icon_spacing = Some(6.0);
        // tooltip sizing
        v.tooltip.padding_horizontal = Some(6.0);
        v.tooltip.padding_vertical = Some(4.0);
        v.tooltip.max_width = Some(300.0);
        // scrollbar sizing
        v.scrollbar.width = Some(14.0);
        v.scrollbar.min_thumb_height = Some(20.0);
        v.scrollbar.slider_width = Some(8.0);
        v.scrollbar.overlay_mode = Some(false);
        // slider sizing
        v.slider.track_height = Some(4.0);
        v.slider.thumb_size = Some(16.0);
        v.slider.tick_length = Some(6.0);
        // progress_bar sizing
        v.progress_bar.height = Some(6.0);
        v.progress_bar.min_width = Some(100.0);
        // tab sizing
        v.tab.min_width = Some(60.0);
        v.tab.min_height = Some(32.0);
        v.tab.padding_horizontal = Some(12.0);
        v.tab.padding_vertical = Some(6.0);
        // toolbar sizing
        v.toolbar.height = Some(40.0);
        v.toolbar.item_spacing = Some(4.0);
        v.toolbar.padding = Some(4.0);
        // list sizing
        v.list.item_height = Some(28.0);
        v.list.padding_horizontal = Some(8.0);
        v.list.padding_vertical = Some(4.0);
        // splitter
        v.splitter.width = Some(4.0);
        // switch sizing
        v.switch.unchecked_background = Some(Rgba::rgb(180, 180, 180));
        v.switch.track_width = Some(40.0);
        v.switch.track_height = Some(20.0);
        v.switch.thumb_size = Some(14.0);
        v.switch.track_radius = Some(10.0);
        // dialog sizing
        v.dialog.min_width = Some(320.0);
        v.dialog.max_width = Some(600.0);
        v.dialog.min_height = Some(200.0);
        v.dialog.max_height = Some(800.0);
        v.dialog.content_padding = Some(16.0);
        v.dialog.button_spacing = Some(8.0);
        v.dialog.icon_size = Some(22.0);
        v.dialog.button_order = Some(DialogButtonOrder::TrailingAffirmative);
        // spinner sizing
        v.spinner.diameter = Some(24.0);
        v.spinner.min_size = Some(16.0);
        v.spinner.stroke_width = Some(2.0);
        // combo_box sizing
        v.combo_box.min_height = Some(28.0);
        v.combo_box.min_width = Some(80.0);
        v.combo_box.padding_horizontal = Some(8.0);
        v.combo_box.arrow_size = Some(12.0);
        v.combo_box.arrow_area_width = Some(20.0);
        // segmented_control sizing
        v.segmented_control.segment_height = Some(28.0);
        v.segmented_control.separator_width = Some(1.0);
        v.segmented_control.padding_horizontal = Some(12.0);
        // card
        v.card.padding = Some(12.0);
        // expander
        v.expander.header_height = Some(32.0);
        v.expander.arrow_size = Some(12.0);
        v.expander.content_padding = Some(8.0);
        // link
        v.link.background = Some(Rgba::rgb(255, 255, 255));
        v.link.hover_bg = Some(Rgba::rgb(230, 230, 255));
        v.link.underline = Some(true);

        v.resolve_all();
        let result = v.validate();
        assert!(
            result.is_ok(),
            "validate() should succeed after resolve_all() with all non-derivable fields set, got: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_gnome_resolve_validate() {
        // Simulate GNOME reader pipeline: adwaita base + GNOME reader overlay.
        // On a non-GNOME system, build_gnome_variant() only sets dialog.button_order
        // and icon_set (gsettings calls return None). We simulate the full merge.
        let adwaita = crate::ThemeSpec::preset("adwaita").unwrap();

        // Pick dark variant from adwaita (matches GNOME PreferDark path).
        let mut variant = adwaita
            .dark
            .clone()
            .expect("adwaita should have dark variant");

        // Apply what build_gnome_variant() would set.
        variant.dialog.button_order = Some(DialogButtonOrder::TrailingAffirmative);
        // icon_set comes from gsettings icon-theme; simulate typical GNOME value.
        variant.icon_set = Some(crate::IconSet::Freedesktop);

        // Simulate GNOME reader font output (gsettings font-name on a GNOME system).
        variant.defaults.font = FontSpec {
            family: Some("Cantarell".to_string()),
            size: Some(11.0),
            weight: Some(400),
        };

        variant.resolve_all();
        let resolved = variant.validate().unwrap_or_else(|e| {
            panic!("GNOME resolve/validate pipeline failed: {e}");
        });

        // Spot-check: adwaita-base fields present.
        // Adwaita dark accent is #3584e4 = rgb(53, 132, 228)
        assert_eq!(
            resolved.defaults.accent,
            Rgba::rgb(53, 132, 228),
            "accent should be from adwaita preset"
        );
        assert_eq!(
            resolved.defaults.font.family, "Cantarell",
            "font family should be from GNOME reader overlay"
        );
        assert_eq!(
            resolved.dialog.button_order,
            DialogButtonOrder::TrailingAffirmative,
            "dialog button order should be trailing affirmative for GNOME"
        );
        assert_eq!(
            resolved.icon_set,
            crate::IconSet::Freedesktop,
            "icon_set should be from GNOME reader"
        );
    }

    // ===== Range validation tests =====

    #[test]
    fn validate_catches_negative_radius() {
        let mut v = fully_populated_variant();
        v.defaults.radius = Some(-5.0);
        v.button.radius = Some(-1.0);
        v.window.radius = Some(-3.0);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.radius") && f.contains("-5")),
            "should report negative defaults.radius, got: {:?}",
            err.missing_fields
        );
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("button.radius") && f.contains("-1")),
            "should report negative button.radius, got: {:?}",
            err.missing_fields
        );
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("window.radius") && f.contains("-3")),
            "should report negative window.radius, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_zero_font_size() {
        let mut v = fully_populated_variant();
        v.defaults.font.size = Some(0.0);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.font.size") && f.contains("> 0")),
            "should report zero defaults.font.size, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_opacity_out_of_range() {
        let mut v = fully_populated_variant();
        v.defaults.disabled_opacity = Some(1.5);
        v.defaults.border_opacity = Some(-0.1);
        v.button.disabled_opacity = Some(3.0);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.disabled_opacity")),
            "should report out-of-range disabled_opacity, got: {:?}",
            err.missing_fields
        );
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.border_opacity")),
            "should report out-of-range border_opacity, got: {:?}",
            err.missing_fields
        );
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("button.disabled_opacity")),
            "should report out-of-range button.disabled_opacity, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_invalid_font_weight() {
        let mut v = fully_populated_variant();
        v.defaults.font.weight = Some(50); // below 100
        v.defaults.mono_font.weight = Some(1000); // above 900

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.font.weight") && f.contains("50")),
            "should report out-of-range font weight 50, got: {:?}",
            err.missing_fields
        );
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.mono_font.weight") && f.contains("1000")),
            "should report out-of-range mono_font weight 1000, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_reports_multiple_range_errors_together() {
        let mut v = fully_populated_variant();
        v.defaults.radius = Some(-1.0);
        v.defaults.disabled_opacity = Some(2.0);
        v.defaults.font.size = Some(0.0);
        v.defaults.font.weight = Some(50);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        // All 4 range errors should be reported in one batch
        assert!(
            err.missing_fields.len() >= 4,
            "should report at least 4 range errors, got {}: {:?}",
            err.missing_fields.len(),
            err.missing_fields
        );
    }

    #[test]
    fn validate_allows_zero_radius_and_frame_width() {
        // Zero is valid for these fields (flat design, no border)
        let mut v = fully_populated_variant();
        v.defaults.radius = Some(0.0);
        v.defaults.radius_lg = Some(0.0);
        v.defaults.frame_width = Some(0.0);
        v.button.radius = Some(0.0);
        v.defaults.disabled_opacity = Some(0.0);
        v.defaults.border_opacity = Some(0.0);

        let result = v.validate();
        assert!(
            result.is_ok(),
            "zero values should be valid for radius/frame_width/opacity, got: {:?}",
            result.err()
        );
    }

    // ===== Resolve completeness test =====

    /// Verify that resolve() has rules for every derived field.
    ///
    /// Constructs a ThemeVariant with ONLY root fields (the ~46 defaults
    /// fields + ~65 widget geometry/behavior fields that have no derivation
    /// path in resolve()). Derived fields (widget colors, widget fonts, text
    /// scale entries, widget-to-widget chains) are left as None.
    ///
    /// If any derived field lacks a resolve rule, it stays None and
    /// validate() reports it as missing -- catching the bug.
    #[test]
    fn resolve_completeness_minimal_variant() {
        // Start with all defaults root fields populated
        let mut v = variant_with_defaults();

        // icon_set is required but not derived from anything -- it's a root field
        v.icon_set = Some(crate::IconSet::Freedesktop);

        // --- Non-derivable widget geometry/behavior fields ---
        // These have no resolve() rule; they MUST be set explicitly.

        // button
        v.button.min_width = Some(64.0);
        v.button.min_height = Some(28.0);
        v.button.padding_horizontal = Some(12.0);
        v.button.padding_vertical = Some(6.0);
        v.button.icon_spacing = Some(6.0);

        // input
        v.input.min_height = Some(28.0);
        v.input.padding_horizontal = Some(8.0);
        v.input.padding_vertical = Some(4.0);

        // checkbox
        v.checkbox.indicator_size = Some(18.0);
        v.checkbox.spacing = Some(6.0);

        // menu
        v.menu.item_height = Some(28.0);
        v.menu.padding_horizontal = Some(8.0);
        v.menu.padding_vertical = Some(4.0);
        v.menu.icon_spacing = Some(6.0);

        // tooltip
        v.tooltip.padding_horizontal = Some(6.0);
        v.tooltip.padding_vertical = Some(4.0);
        v.tooltip.max_width = Some(300.0);

        // scrollbar
        v.scrollbar.width = Some(14.0);
        v.scrollbar.min_thumb_height = Some(20.0);
        v.scrollbar.slider_width = Some(8.0);
        v.scrollbar.overlay_mode = Some(false);

        // slider
        v.slider.track_height = Some(4.0);
        v.slider.thumb_size = Some(16.0);
        v.slider.tick_length = Some(6.0);

        // progress_bar
        v.progress_bar.height = Some(6.0);
        v.progress_bar.min_width = Some(100.0);

        // tab
        v.tab.min_width = Some(60.0);
        v.tab.min_height = Some(32.0);
        v.tab.padding_horizontal = Some(12.0);
        v.tab.padding_vertical = Some(6.0);

        // toolbar
        v.toolbar.height = Some(40.0);
        v.toolbar.item_spacing = Some(4.0);
        v.toolbar.padding = Some(4.0);

        // list
        v.list.item_height = Some(28.0);
        v.list.padding_horizontal = Some(8.0);
        v.list.padding_vertical = Some(4.0);

        // splitter
        v.splitter.width = Some(4.0);

        // switch
        v.switch.track_width = Some(40.0);
        v.switch.track_height = Some(20.0);
        v.switch.thumb_size = Some(14.0);
        v.switch.track_radius = Some(10.0);

        // dialog
        v.dialog.min_width = Some(320.0);
        v.dialog.max_width = Some(600.0);
        v.dialog.min_height = Some(200.0);
        v.dialog.max_height = Some(800.0);
        v.dialog.content_padding = Some(16.0);
        v.dialog.button_spacing = Some(8.0);
        v.dialog.icon_size = Some(22.0);
        v.dialog.button_order = Some(DialogButtonOrder::TrailingAffirmative);

        // spinner
        v.spinner.diameter = Some(24.0);
        v.spinner.min_size = Some(16.0);
        v.spinner.stroke_width = Some(2.0);

        // combo_box
        v.combo_box.min_height = Some(28.0);
        v.combo_box.min_width = Some(80.0);
        v.combo_box.padding_horizontal = Some(8.0);
        v.combo_box.arrow_size = Some(12.0);
        v.combo_box.arrow_area_width = Some(20.0);

        // segmented_control
        v.segmented_control.segment_height = Some(28.0);
        v.segmented_control.separator_width = Some(1.0);
        v.segmented_control.padding_horizontal = Some(12.0);

        // card
        v.card.padding = Some(12.0);

        // expander
        v.expander.header_height = Some(32.0);
        v.expander.arrow_size = Some(12.0);
        v.expander.content_padding = Some(8.0);

        // link: background and hover_bg have no derivation path
        v.link.background = Some(Rgba::rgb(255, 255, 255));
        v.link.hover_bg = Some(Rgba::rgb(230, 230, 255));
        v.link.underline = Some(true);

        // --- Verify: NO derived color/font/text_scale fields are set ---
        // These should all be None at this point (resolve must fill them):
        assert!(
            v.window.background.is_none(),
            "window.background should be None before resolve"
        );
        assert!(
            v.button.background.is_none(),
            "button.background should be None before resolve"
        );
        assert!(
            v.button.font.is_none(),
            "button.font should be None before resolve"
        );
        assert!(
            v.text_scale.caption.is_none(),
            "text_scale.caption should be None before resolve"
        );

        // --- Resolve and validate ---
        v.resolve_all();
        let result = v.validate();
        assert!(
            result.is_ok(),
            "Resolve completeness failed -- some derived fields lack resolve rules: {:?}",
            result.err()
        );
    }

    /// Cross-check: verify completeness by stripping derived fields from a preset.
    ///
    /// Loads a known preset, clears all derived color/font/text_scale fields,
    /// then verifies resolve() can reconstruct them. This is the complementary
    /// approach to `resolve_completeness_minimal_variant` -- it starts from a
    /// known-good preset instead of building from scratch.
    #[test]
    fn resolve_completeness_from_preset() {
        let spec = crate::ThemeSpec::preset("material").unwrap();
        let mut v = spec.dark.expect("material should have dark variant");

        // Clear all derived color fields -- these should all be refilled by resolve()
        // window colors
        v.window.background = None;
        v.window.foreground = None;
        v.window.border = None;
        v.window.title_bar_background = None;
        v.window.title_bar_foreground = None;
        v.window.inactive_title_bar_background = None;
        v.window.inactive_title_bar_foreground = None;
        v.window.radius = None;
        v.window.shadow = None;

        // button colors
        v.button.background = None;
        v.button.foreground = None;
        v.button.border = None;
        v.button.primary_background = None;
        v.button.primary_foreground = None;
        v.button.radius = None;
        v.button.disabled_opacity = None;
        v.button.shadow = None;

        // input colors
        v.input.background = None;
        v.input.foreground = None;
        v.input.border = None;
        v.input.placeholder = None;
        v.input.caret = None;
        v.input.selection = None;
        v.input.selection_foreground = None;
        v.input.radius = None;
        v.input.border_width = None;

        // checkbox colors
        v.checkbox.checked_bg = None;
        v.checkbox.radius = None;
        v.checkbox.border_width = None;

        // menu colors
        v.menu.background = None;
        v.menu.foreground = None;
        v.menu.separator = None;

        // tooltip colors
        v.tooltip.background = None;
        v.tooltip.foreground = None;
        v.tooltip.radius = None;

        // scrollbar colors
        v.scrollbar.track = None;
        v.scrollbar.thumb = None;
        v.scrollbar.thumb_hover = None;

        // slider colors
        v.slider.fill = None;
        v.slider.track = None;
        v.slider.thumb = None;

        // progress_bar colors
        v.progress_bar.fill = None;
        v.progress_bar.track = None;
        v.progress_bar.radius = None;

        // tab colors
        v.tab.background = None;
        v.tab.foreground = None;
        v.tab.active_background = None;
        v.tab.active_foreground = None;
        v.tab.bar_background = None;

        // sidebar colors
        v.sidebar.background = None;
        v.sidebar.foreground = None;

        // list colors
        v.list.background = None;
        v.list.foreground = None;
        v.list.alternate_row = None;
        v.list.selection = None;
        v.list.selection_foreground = None;
        v.list.header_background = None;
        v.list.header_foreground = None;
        v.list.grid_color = None;

        // popover colors
        v.popover.background = None;
        v.popover.foreground = None;
        v.popover.border = None;
        v.popover.radius = None;

        // separator
        v.separator.color = None;

        // switch colors
        v.switch.checked_background = None;
        v.switch.unchecked_background = None;
        v.switch.thumb_background = None;

        // dialog radius
        v.dialog.radius = None;

        // combo_box radius
        v.combo_box.radius = None;

        // segmented_control radius
        v.segmented_control.radius = None;

        // card colors
        v.card.background = None;
        v.card.border = None;
        v.card.radius = None;
        v.card.shadow = None;

        // expander radius
        v.expander.radius = None;

        // link colors
        v.link.color = None;
        v.link.visited = None;

        // spinner
        v.spinner.fill = None;

        // Clear all widget fonts (these are derived from defaults.font)
        v.window.title_bar_font = None;
        v.button.font = None;
        v.input.font = None;
        v.menu.font = None;
        v.tooltip.font = None;
        v.toolbar.font = None;
        v.status_bar.font = None;
        v.dialog.title_font = None;

        // Clear text_scale entries (derived from defaults.font + defaults.line_height)
        v.text_scale.caption = None;
        v.text_scale.section_heading = None;
        v.text_scale.dialog_title = None;
        v.text_scale.display = None;

        // Clear defaults internal chains (derived from accent/selection)
        v.defaults.selection = None;
        v.defaults.focus_ring_color = None;
        v.defaults.selection_inactive = None;

        // Resolve should fill everything back
        v.resolve_all();
        let result = v.validate();
        assert!(
            result.is_ok(),
            "Resolve completeness from preset failed -- some derived fields lack resolve rules: {:?}",
            result.err()
        );
    }

    #[test]
    fn validate_all_presets_pass_range_checks() {
        // Verify no false positives: all 16 presets pass validation including range checks
        let names = crate::ThemeSpec::list_presets();
        assert!(names.len() >= 16, "expected at least 16 presets");

        for name in names {
            let spec = crate::ThemeSpec::preset(name).unwrap();
            if let Some(light) = spec.light {
                let resolved = light.into_resolved();
                assert!(
                    resolved.is_ok(),
                    "preset '{name}' light variant failed: {:?}",
                    resolved.err()
                );
            }
            if let Some(dark) = spec.dark {
                let resolved = dark.into_resolved();
                assert!(
                    resolved.is_ok(),
                    "preset '{name}' dark variant failed: {:?}",
                    resolved.err()
                );
            }
        }
    }
}
