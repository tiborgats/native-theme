// Resolution engine: resolve() fills inheritance rules, validate() produces ResolvedThemeVariant.

use crate::Rgba;
use crate::error::ThemeResolutionError;
use crate::model::border::{BorderSpec, ResolvedBorderSpec};
use crate::model::resolved::{
    ResolvedIconSizes, ResolvedTextScale, ResolvedTextScaleEntry, ResolvedThemeDefaults,
    ResolvedThemeVariant,
};
use crate::model::{DialogButtonOrder, FontSpec, ResolvedFontSpec, TextScaleEntry, ThemeVariant};

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
/// applies a size ratio and default weight, then computes line_height
/// from defaults.line_height * resolved_size.
///
/// `size_ratio` scales the default font size (e.g. 0.82 for caption).
/// `default_weight` is the fallback weight when none is set (e.g. 700 for headings).
fn resolve_text_scale_entry(
    entry: &mut Option<TextScaleEntry>,
    defaults_font: &FontSpec,
    defaults_line_height: Option<f32>,
    size_ratio: f32,
    default_weight: u16,
) {
    let entry = entry.get_or_insert_with(TextScaleEntry::default);
    if entry.size.is_none() {
        entry.size = defaults_font.size.map(|s| s * size_ratio);
    }
    if entry.weight.is_none() {
        entry.weight = Some(default_weight);
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
        style: font.style.unwrap_or_default(),
        color: font.color.unwrap_or(Rgba::rgb(0, 0, 0)),
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
                style: f.style.unwrap_or_default(),
                color: f.color.unwrap_or(Rgba::rgb(0, 0, 0)),
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

/// Check that an `f32` value is finite and non-negative (>= 0.0).
fn check_non_negative(value: f32, path: &str, errors: &mut Vec<String>) {
    if !value.is_finite() || value < 0.0 {
        errors.push(format!(
            "{path} must be a finite non-negative number, got {value}"
        ));
    }
}

/// Check that an `f32` value is finite and strictly positive (> 0.0).
fn check_positive(value: f32, path: &str, errors: &mut Vec<String>) {
    if !value.is_finite() || value <= 0.0 {
        errors.push(format!(
            "{path} must be a finite positive number, got {value}"
        ));
    }
}

/// Check that an `f32` value is finite and falls within an inclusive range.
fn check_range_f32(value: f32, min: f32, max: f32, path: &str, errors: &mut Vec<String>) {
    if !value.is_finite() || value < min || value > max {
        errors.push(format!(
            "{path} must be a finite number between {min} and {max}, got {value}"
        ));
    }
}

/// Check that a `u16` value falls within an inclusive range.
fn check_range_u16(value: u16, min: u16, max: u16, path: &str, errors: &mut Vec<String>) {
    if value < min || value > max {
        errors.push(format!("{path} must be {min}..={max}, got {value}"));
    }
}

/// Check that a min value does not exceed its corresponding max value.
fn check_min_max(
    min_val: f32,
    max_val: f32,
    min_name: &str,
    max_name: &str,
    errors: &mut Vec<String>,
) {
    if min_val > max_val {
        errors.push(format!(
            "{min_name} ({min_val}) must not exceed {max_name} ({max_val})"
        ));
    }
}

/// Return the platform-appropriate dialog button order.
///
/// On Linux/KDE, the convention is leading-affirmative (OK on the left).
/// On Windows, GNOME, macOS, iOS the convention is trailing-affirmative.
fn platform_button_order() -> DialogButtonOrder {
    #[cfg(target_os = "linux")]
    {
        if crate::detect_linux_de(&crate::xdg_current_desktop()) == crate::LinuxDesktop::Kde {
            return DialogButtonOrder::PrimaryLeft;
        }
    }
    // Windows, GNOME, macOS, iOS all use primary-right
    DialogButtonOrder::PrimaryRight
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
    /// let accent = resolved.defaults.accent_color;
    /// ```
    #[must_use = "this returns the resolved theme; it does not modify self"]
    pub fn into_resolved(mut self) -> crate::Result<ResolvedThemeVariant> {
        self.resolve_all();
        self.validate()
    }

    // --- Phase 1: Defaults internal chains ---

    fn resolve_defaults_internal(&mut self) {
        let d = &mut self.defaults;

        // selection_background <- accent_color
        if d.selection_background.is_none() {
            d.selection_background = d.accent_color;
        }
        // focus_ring_color <- accent_color
        if d.focus_ring_color.is_none() {
            d.focus_ring_color = d.accent_color;
        }
        // selection_inactive_background <- selection_background (MUST run after selection_background is set)
        if d.selection_inactive_background.is_none() {
            d.selection_inactive_background = d.selection_background;
        }
        // text_selection_background <- selection_background (inline text highlight defaults to selection)
        if d.text_selection_background.is_none() {
            d.text_selection_background = d.selection_background;
        }
        // text_selection_color <- selection_text_color
        if d.text_selection_color.is_none() {
            d.text_selection_color = d.selection_text_color;
        }
        // defaults.font.color <- defaults.text_color
        if d.font.color.is_none() {
            d.font.color = d.text_color;
        }
        // defaults.mono_font.color <- defaults.font.color (MUST run AFTER font.color is set)
        if d.mono_font.color.is_none() {
            d.mono_font.color = d.font.color;
        }
        // defaults.border padding -- defaults-level border carries no meaningful
        // padding; per-widget border specs carry the actual padding values.
        // Derive from line_width presence to ensure the field is populated.
        if d.border.padding_horizontal.is_none() {
            d.border.padding_horizontal = d.border.line_width.map(|_| 0.0_f32);
            if d.border.padding_horizontal.is_none() {
                d.border.padding_horizontal = d.border.corner_radius.map(|_| 0.0_f32);
            }
        }
        if d.border.padding_vertical.is_none() {
            d.border.padding_vertical = d.border.line_width.map(|_| 0.0_f32);
            if d.border.padding_vertical.is_none() {
                d.border.padding_vertical = d.border.corner_radius.map(|_| 0.0_f32);
            }
        }
    }

    // --- Phase 2: Safety nets ---

    fn resolve_safety_nets(&mut self) {
        // dialog.button_order <- platform convention
        if self.dialog.button_order.is_none() {
            self.dialog.button_order = Some(platform_button_order());
        }
        // input.caret_color <- defaults.text_color
        if self.input.caret_color.is_none() {
            self.input.caret_color = self.defaults.text_color;
        }
        // scrollbar.track <- defaults.background_color
        if self.scrollbar.track_color.is_none() {
            self.scrollbar.track_color = self.defaults.background_color;
        }
        // spinner.fill <- defaults.accent (all platforms use accent)
        if self.spinner.fill_color.is_none() {
            self.spinner.fill_color = self.defaults.accent_color;
        }
        // popover.background <- defaults.background_color
        if self.popover.background_color.is_none() {
            self.popover.background_color = self.defaults.background_color;
        }
        // list.background <- defaults.background_color
        if self.list.background_color.is_none() {
            self.list.background_color = self.defaults.background_color;
        }
        // dialog.background <- defaults.background_color (per_platform fallback)
        if self.dialog.background_color.is_none() {
            self.dialog.background_color = self.defaults.background_color;
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
        if self.window.background_color.is_none() {
            self.window.background_color = d.background_color;
        }
        {
            let font = self
                .window
                .title_bar_font
                .get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        {
            let border = self.window.border.get_or_insert_with(BorderSpec::default);
            if border.color.is_none() {
                border.color = d.border.color;
            }
        }
        if self.window.title_bar_background.is_none() {
            self.window.title_bar_background = d.surface_color;
        }
        {
            let border = self.window.border.get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius_lg;
            }
            if border.shadow_enabled.is_none() {
                border.shadow_enabled = d.border.shadow_enabled;
            }
        }

        // --- button ---
        if self.button.background_color.is_none() {
            self.button.background_color = d.background_color;
        }
        {
            let font = self.button.font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        {
            let border = self.button.border.get_or_insert_with(BorderSpec::default);
            if border.color.is_none() {
                border.color = d.border.color;
            }
        }
        if self.button.primary_background.is_none() {
            self.button.primary_background = d.accent_color;
        }
        if self.button.primary_text_color.is_none() {
            self.button.primary_text_color = d.accent_text_color;
        }
        {
            let border = self.button.border.get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius;
            }
        }
        if self.button.disabled_opacity.is_none() {
            self.button.disabled_opacity = d.disabled_opacity;
        }
        {
            let border = self.button.border.get_or_insert_with(BorderSpec::default);
            if border.shadow_enabled.is_none() {
                border.shadow_enabled = d.border.shadow_enabled;
            }
        }

        // --- input ---
        if self.input.background_color.is_none() {
            self.input.background_color = d.background_color;
        }
        {
            let font = self.input.font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        {
            let border = self.input.border.get_or_insert_with(BorderSpec::default);
            if border.color.is_none() {
                border.color = d.border.color;
            }
        }
        if self.input.placeholder_color.is_none() {
            self.input.placeholder_color = d.muted_color;
        }
        if self.input.selection_background.is_none() {
            self.input.selection_background = d.text_selection_background;
        }
        if self.input.selection_text_color.is_none() {
            self.input.selection_text_color = d.text_selection_color;
        }
        {
            let border = self.input.border.get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius;
            }
            if border.line_width.is_none() {
                border.line_width = d.border.line_width;
            }
        }

        // --- checkbox ---
        if self.checkbox.checked_background.is_none() {
            self.checkbox.checked_background = d.accent_color;
        }
        {
            let border = self.checkbox.border.get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius;
            }
            if border.line_width.is_none() {
                border.line_width = d.border.line_width;
            }
        }

        // --- menu ---
        if self.menu.background_color.is_none() {
            self.menu.background_color = d.background_color;
        }
        {
            let font = self.menu.font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        if self.menu.separator_color.is_none() {
            self.menu.separator_color = d.border.color;
        }

        // --- tooltip ---
        if self.tooltip.background_color.is_none() {
            self.tooltip.background_color = d.background_color;
        }
        {
            let font = self.tooltip.font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        {
            let border = self.tooltip.border.get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius;
            }
        }

        // --- scrollbar ---
        if self.scrollbar.thumb_color.is_none() {
            self.scrollbar.thumb_color = d.muted_color;
        }
        // scrollbar.thumb_hover_color <- defaults.muted_color (per inheritance-rules.toml line 154)
        // Both thumb_color and thumb_hover_color inherit from defaults.muted_color independently.
        if self.scrollbar.thumb_hover_color.is_none() {
            self.scrollbar.thumb_hover_color = d.muted_color;
        }

        // --- slider ---
        if self.slider.fill_color.is_none() {
            self.slider.fill_color = d.accent_color;
        }
        if self.slider.track_color.is_none() {
            self.slider.track_color = d.muted_color;
        }
        if self.slider.thumb_color.is_none() {
            self.slider.thumb_color = d.surface_color;
        }

        // --- progress_bar ---
        if self.progress_bar.fill_color.is_none() {
            self.progress_bar.fill_color = d.accent_color;
        }
        if self.progress_bar.track_color.is_none() {
            self.progress_bar.track_color = d.muted_color;
        }
        if self.progress_bar.border.is_none() {
            self.progress_bar.border = Some(crate::model::border::BorderSpec {
                corner_radius: d.border.corner_radius,
                ..Default::default()
            });
        } else if let Some(ref mut b) = self.progress_bar.border
            && b.corner_radius.is_none()
        {
            b.corner_radius = d.border.corner_radius;
        }

        // --- tab ---
        if self.tab.background_color.is_none() {
            self.tab.background_color = d.background_color;
        }
        {
            let font = self.tab.font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        if self.tab.active_background.is_none() {
            self.tab.active_background = d.background_color;
        }
        if self.tab.active_text_color.is_none() {
            self.tab.active_text_color = d.text_color;
        }
        if self.tab.bar_background.is_none() {
            self.tab.bar_background = d.background_color;
        }

        // --- sidebar ---
        if self.sidebar.background_color.is_none() {
            self.sidebar.background_color = d.background_color;
        }
        {
            let font = self.sidebar.font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }

        // --- list ---
        {
            let font = self.list.item_font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        if self.list.alternate_row_background.is_none() {
            self.list.alternate_row_background = self.list.background_color;
        }
        if self.list.selection_background.is_none() {
            self.list.selection_background = d.selection_background;
        }
        if self.list.selection_text_color.is_none() {
            self.list.selection_text_color = d.selection_text_color;
        }
        if self.list.header_background.is_none() {
            self.list.header_background = d.surface_color;
        }
        {
            let font = self.list.header_font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        if self.list.grid_color.is_none() {
            self.list.grid_color = d.border.color;
        }

        // --- popover ---
        {
            let font = self.popover.font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.text_color;
            }
        }
        {
            let border = self.popover.border.get_or_insert_with(BorderSpec::default);
            if border.color.is_none() {
                border.color = d.border.color;
            }
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius_lg;
            }
        }

        // --- separator ---
        if self.separator.line_color.is_none() {
            self.separator.line_color = d.border.color;
        }

        // --- switch ---
        if self.switch.checked_background.is_none() {
            self.switch.checked_background = d.accent_color;
        }
        // switch.unchecked_background: no inheritance -- each platform has
        // distinct off-track color (SPEC-3 fix, per no_inheritance spec)
        if self.switch.thumb_background.is_none() {
            self.switch.thumb_background = d.surface_color;
        }

        // --- dialog ---
        {
            let border = self.dialog.border.get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius_lg;
            }
        }

        // --- combo_box ---
        {
            let border = self
                .combo_box
                .border
                .get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius;
            }
        }

        // --- segmented_control ---
        {
            let border = self
                .segmented_control
                .border
                .get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius;
            }
        }

        // --- card ---
        if self.card.background_color.is_none() {
            self.card.background_color = d.surface_color;
        }
        // card.border: all sub-fields are platform-specific or (none) per §2.26
        // -- no inheritance from defaults.border (INH-3 fix)

        // --- expander ---
        {
            let border = self.expander.border.get_or_insert_with(BorderSpec::default);
            if border.corner_radius.is_none() {
                border.corner_radius = d.border.corner_radius;
            }
        }

        // --- link ---
        {
            let font = self.link.font.get_or_insert_with(FontSpec::default);
            if font.color.is_none() {
                font.color = d.link_color;
            }
        }
        if self.link.visited_text_color.is_none() {
            self.link.visited_text_color = d.link_color;
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
        let body_weight = defaults_font.weight.unwrap_or(400);

        // caption: 0.82x body size, body weight
        resolve_text_scale_entry(
            &mut self.text_scale.caption,
            defaults_font,
            defaults_lh,
            0.82,
            body_weight,
        );
        // section_heading: 1.0x body size, bold (700)
        resolve_text_scale_entry(
            &mut self.text_scale.section_heading,
            defaults_font,
            defaults_lh,
            1.0,
            700,
        );
        // dialog_title: 1.2x body size, bold (700)
        resolve_text_scale_entry(
            &mut self.text_scale.dialog_title,
            defaults_font,
            defaults_lh,
            1.2,
            700,
        );
        // display: 2.0x body size, bold (700)
        resolve_text_scale_entry(
            &mut self.text_scale.display,
            defaults_font,
            defaults_lh,
            2.0,
            700,
        );
    }

    // --- Phase 4: Widget-to-widget chains ---

    fn resolve_widget_to_widget(&mut self) {
        // inactive title bar <- active title bar
        if self.window.inactive_title_bar_background.is_none() {
            self.window.inactive_title_bar_background = self.window.title_bar_background;
        }
        if self.window.inactive_title_bar_text_color.is_none() {
            self.window.inactive_title_bar_text_color =
                self.window.title_bar_font.as_ref().and_then(|f| f.color);
        }
    }

    // --- validate() ---

    /// Convert this ThemeVariant into a [`ResolvedThemeVariant`] with all fields guaranteed.
    ///
    /// Should be called after [`resolve()`](ThemeVariant::resolve). Walks every field
    /// and collects missing (None) field paths, then validates that numeric values
    /// are within legal ranges (e.g., spacing >= 0, opacity 0..=1, font weight
    /// 100..=900). Returns `Ok(ResolvedThemeVariant)` if all fields are populated
    /// and in range.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Resolution`] containing a [`ThemeResolutionError`]
    /// with all missing field paths and out-of-range diagnostics.
    // Validation is kept in a single function for field-level traceability.
    // Future extraction into per-widget validators is planned for v0.6.0.
    #[must_use = "this returns the resolved theme; it does not modify self"]
    pub fn validate(&self) -> crate::Result<ResolvedThemeVariant> {
        let mut missing = Vec::new();

        // ┌──────────────────────────────────────────────────────────────┐
        // │ Adding a new widget or field? Follow these steps:           │
        // │ 1. Add require() calls in the extraction section below      │
        // │ 2. Add range checks in the "range validation" section       │
        // │ 3. Add the field to the ResolvedThemeVariant construction   │
        // │ Each section is marked with // --- widget_name ---          │
        // └──────────────────────────────────────────────────────────────┘

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
            &self.defaults.background_color,
            "defaults.background_color",
            &mut missing,
        );
        let defaults_foreground = require(
            &self.defaults.text_color,
            "defaults.foreground",
            &mut missing,
        );
        let defaults_accent = require(
            &self.defaults.accent_color,
            "defaults.accent_color",
            &mut missing,
        );
        let defaults_accent_foreground = require(
            &self.defaults.accent_text_color,
            "defaults.accent_text_color",
            &mut missing,
        );
        let defaults_surface = require(
            &self.defaults.surface_color,
            "defaults.surface_color",
            &mut missing,
        );
        let defaults_border = require(
            &self.defaults.border.color,
            "defaults.border.color",
            &mut missing,
        );
        let defaults_muted = require(
            &self.defaults.muted_color,
            "defaults.muted_color",
            &mut missing,
        );
        let defaults_shadow = require(
            &self.defaults.shadow_color,
            "defaults.shadow_color",
            &mut missing,
        );
        let defaults_link = require(
            &self.defaults.link_color,
            "defaults.link_color",
            &mut missing,
        );
        let defaults_selection = require(
            &self.defaults.selection_background,
            "defaults.selection_background",
            &mut missing,
        );
        let defaults_selection_foreground = require(
            &self.defaults.selection_text_color,
            "defaults.selection_text_color",
            &mut missing,
        );
        let defaults_selection_inactive = require(
            &self.defaults.selection_inactive_background,
            "defaults.selection_inactive_background",
            &mut missing,
        );
        let defaults_disabled_foreground = require(
            &self.defaults.disabled_text_color,
            "defaults.disabled_text_color",
            &mut missing,
        );

        let defaults_danger = require(
            &self.defaults.danger_color,
            "defaults.danger_color",
            &mut missing,
        );
        let defaults_danger_foreground = require(
            &self.defaults.danger_text_color,
            "defaults.danger_text_color",
            &mut missing,
        );
        let defaults_warning = require(
            &self.defaults.warning_color,
            "defaults.warning_color",
            &mut missing,
        );
        let defaults_warning_foreground = require(
            &self.defaults.warning_text_color,
            "defaults.warning_text_color",
            &mut missing,
        );
        let defaults_success = require(
            &self.defaults.success_color,
            "defaults.success_color",
            &mut missing,
        );
        let defaults_success_foreground = require(
            &self.defaults.success_text_color,
            "defaults.success_text_color",
            &mut missing,
        );
        let defaults_info = require(
            &self.defaults.info_color,
            "defaults.info_color",
            &mut missing,
        );
        let defaults_info_foreground = require(
            &self.defaults.info_text_color,
            "defaults.info_text_color",
            &mut missing,
        );

        let defaults_radius = require(
            &self.defaults.border.corner_radius,
            "defaults.border.corner_radius",
            &mut missing,
        );
        let defaults_radius_lg = require(
            &self.defaults.border.corner_radius_lg,
            "defaults.border.corner_radius_lg",
            &mut missing,
        );
        let defaults_frame_width = require(
            &self.defaults.border.line_width,
            "defaults.border.line_width",
            &mut missing,
        );
        let defaults_disabled_opacity = require(
            &self.defaults.disabled_opacity,
            "defaults.disabled_opacity",
            &mut missing,
        );
        let defaults_border_opacity = require(
            &self.defaults.border.opacity,
            "defaults.border.opacity",
            &mut missing,
        );
        let defaults_shadow_enabled = require(
            &self.defaults.border.shadow_enabled,
            "defaults.border.shadow_enabled",
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

        let defaults_border_padding_h = require(
            &self.defaults.border.padding_horizontal,
            "defaults.border.padding_horizontal",
            &mut missing,
        );
        let defaults_border_padding_v = require(
            &self.defaults.border.padding_vertical,
            "defaults.border.padding_vertical",
            &mut missing,
        );
        let defaults_text_selection_background = require(
            &self.defaults.text_selection_background,
            "defaults.text_selection_background",
            &mut missing,
        );
        let defaults_text_selection_color = require(
            &self.defaults.text_selection_color,
            "defaults.text_selection_color",
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

        let window_background = require(
            &self.window.background_color,
            "window.background_color",
            &mut missing,
        );
        let window_title_bar_background = require(
            &self.window.title_bar_background,
            "window.title_bar_background",
            &mut missing,
        );
        let window_inactive_title_bar_background = require(
            &self.window.inactive_title_bar_background,
            "window.inactive_title_bar_background",
            &mut missing,
        );
        let window_inactive_title_bar_foreground = require(
            &self.window.inactive_title_bar_text_color,
            "window.inactive_title_bar_text_color",
            &mut missing,
        );
        let window_title_bar_font = require_font_opt(
            &self.window.title_bar_font,
            "window.title_bar_font",
            &mut missing,
        );

        // --- button ---

        let button_background = require(
            &self.button.background_color,
            "button.background_color",
            &mut missing,
        );
        let button_primary_background = require(
            &self.button.primary_background,
            "button.primary_background",
            &mut missing,
        );
        let button_primary_foreground = require(
            &self.button.primary_text_color,
            "button.primary_text_color",
            &mut missing,
        );
        let button_min_width = require(&self.button.min_width, "button.min_width", &mut missing);
        let button_min_height = require(&self.button.min_height, "button.min_height", &mut missing);
        let button_icon_spacing = require(
            &self.button.icon_text_gap,
            "button.icon_text_gap",
            &mut missing,
        );
        let button_disabled_opacity = require(
            &self.button.disabled_opacity,
            "button.disabled_opacity",
            &mut missing,
        );
        let button_font = require_font_opt(&self.button.font, "button.font", &mut missing);

        // --- input ---

        let input_background = require(
            &self.input.background_color,
            "input.background_color",
            &mut missing,
        );
        let input_placeholder = require(
            &self.input.placeholder_color,
            "input.placeholder_color",
            &mut missing,
        );
        let input_caret = require(&self.input.caret_color, "input.caret_color", &mut missing);
        let input_selection = require(
            &self.input.selection_background,
            "input.selection_background",
            &mut missing,
        );
        let input_selection_foreground = require(
            &self.input.selection_text_color,
            "input.selection_text_color",
            &mut missing,
        );
        let input_min_height = require(&self.input.min_height, "input.min_height", &mut missing);
        let input_font = require_font_opt(&self.input.font, "input.font", &mut missing);

        // --- checkbox ---

        let checkbox_checked_background = require(
            &self.checkbox.checked_background,
            "checkbox.checked_background",
            &mut missing,
        );
        let checkbox_indicator_size = require(
            &self.checkbox.indicator_width,
            "checkbox.indicator_width",
            &mut missing,
        );
        let checkbox_spacing =
            require(&self.checkbox.label_gap, "checkbox.label_gap", &mut missing);

        // --- menu ---

        let menu_background = require(
            &self.menu.background_color,
            "menu.background_color",
            &mut missing,
        );
        let menu_separator = require(
            &self.menu.separator_color,
            "menu.separator_color",
            &mut missing,
        );
        let menu_item_height = require(&self.menu.row_height, "menu.row_height", &mut missing);
        let menu_icon_spacing =
            require(&self.menu.icon_text_gap, "menu.icon_text_gap", &mut missing);
        let menu_font = require_font_opt(&self.menu.font, "menu.font", &mut missing);

        // --- tooltip ---

        let tooltip_background = require(
            &self.tooltip.background_color,
            "tooltip.background_color",
            &mut missing,
        );
        let tooltip_max_width = require(&self.tooltip.max_width, "tooltip.max_width", &mut missing);
        let tooltip_font = require_font_opt(&self.tooltip.font, "tooltip.font", &mut missing);

        // --- scrollbar ---

        let scrollbar_track = require(
            &self.scrollbar.track_color,
            "scrollbar.track_color",
            &mut missing,
        );
        let scrollbar_thumb = require(
            &self.scrollbar.thumb_color,
            "scrollbar.thumb_color",
            &mut missing,
        );
        let scrollbar_thumb_hover = require(
            &self.scrollbar.thumb_hover_color,
            "scrollbar.thumb_hover_color",
            &mut missing,
        );
        let scrollbar_width = require(
            &self.scrollbar.groove_width,
            "scrollbar.groove_width",
            &mut missing,
        );
        let scrollbar_min_thumb_height = require(
            &self.scrollbar.min_thumb_length,
            "scrollbar.min_thumb_length",
            &mut missing,
        );
        let scrollbar_slider_width = require(
            &self.scrollbar.thumb_width,
            "scrollbar.thumb_width",
            &mut missing,
        );
        let scrollbar_overlay_mode = require(
            &self.scrollbar.overlay_mode,
            "scrollbar.overlay_mode",
            &mut missing,
        );

        // --- slider ---

        let slider_fill = require(&self.slider.fill_color, "slider.fill_color", &mut missing);
        let slider_track = require(&self.slider.track_color, "slider.track_color", &mut missing);
        let slider_thumb = require(&self.slider.thumb_color, "slider.thumb_color", &mut missing);
        let slider_track_height = require(
            &self.slider.track_height,
            "slider.track_height",
            &mut missing,
        );
        let slider_thumb_size = require(
            &self.slider.thumb_diameter,
            "slider.thumb_diameter",
            &mut missing,
        );
        let slider_tick_length = require(
            &self.slider.tick_mark_length,
            "slider.tick_mark_length",
            &mut missing,
        );

        // --- progress_bar ---

        let progress_bar_fill = require(
            &self.progress_bar.fill_color,
            "progress_bar.fill_color",
            &mut missing,
        );
        let progress_bar_track = require(
            &self.progress_bar.track_color,
            "progress_bar.track_color",
            &mut missing,
        );
        let progress_bar_height = require(
            &self.progress_bar.track_height,
            "progress_bar.track_height",
            &mut missing,
        );
        let progress_bar_min_width = require(
            &self.progress_bar.min_width,
            "progress_bar.min_width",
            &mut missing,
        );
        // progress_bar.border.corner_radius -- handled by placeholder border spec

        // --- tab ---

        let tab_background = require(
            &self.tab.background_color,
            "tab.background_color",
            &mut missing,
        );
        let tab_active_background = require(
            &self.tab.active_background,
            "tab.active_background",
            &mut missing,
        );
        let tab_active_foreground = require(
            &self.tab.active_text_color,
            "tab.active_text_color",
            &mut missing,
        );
        let tab_bar_background =
            require(&self.tab.bar_background, "tab.bar_background", &mut missing);
        let tab_min_width = require(&self.tab.min_width, "tab.min_width", &mut missing);
        let tab_min_height = require(&self.tab.min_height, "tab.min_height", &mut missing);

        // --- sidebar ---

        let sidebar_background = require(
            &self.sidebar.background_color,
            "sidebar.background_color",
            &mut missing,
        );

        // --- toolbar ---

        let toolbar_height = require(&self.toolbar.bar_height, "toolbar.bar_height", &mut missing);
        let toolbar_item_spacing =
            require(&self.toolbar.item_gap, "toolbar.item_gap", &mut missing);
        let toolbar_font = require_font_opt(&self.toolbar.font, "toolbar.font", &mut missing);

        // --- status_bar ---

        let status_bar_font =
            require_font_opt(&self.status_bar.font, "status_bar.font", &mut missing);

        // --- list ---

        let list_background = require(
            &self.list.background_color,
            "list.background_color",
            &mut missing,
        );
        let list_alternate_row = require(
            &self.list.alternate_row_background,
            "list.alternate_row_background",
            &mut missing,
        );
        let list_selection = require(
            &self.list.selection_background,
            "list.selection_background",
            &mut missing,
        );
        let list_selection_foreground = require(
            &self.list.selection_text_color,
            "list.selection_text_color",
            &mut missing,
        );
        let list_header_background = require(
            &self.list.header_background,
            "list.header_background",
            &mut missing,
        );
        let list_grid_color = require(&self.list.grid_color, "list.grid_color", &mut missing);
        let list_item_height = require(&self.list.row_height, "list.row_height", &mut missing);

        // --- popover ---

        let popover_background = require(
            &self.popover.background_color,
            "popover.background_color",
            &mut missing,
        );

        // --- splitter ---

        let splitter_width = require(
            &self.splitter.divider_width,
            "splitter.divider_width",
            &mut missing,
        );

        // --- separator ---

        let separator_color = require(
            &self.separator.line_color,
            "separator.line_color",
            &mut missing,
        );

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
        let switch_thumb_size = require(
            &self.switch.thumb_diameter,
            "switch.thumb_diameter",
            &mut missing,
        );
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
        let dialog_button_spacing =
            require(&self.dialog.button_gap, "dialog.button_gap", &mut missing);
        let dialog_icon_size = require(&self.dialog.icon_size, "dialog.icon_size", &mut missing);
        let dialog_button_order = require(
            &self.dialog.button_order,
            "dialog.button_order",
            &mut missing,
        );
        let dialog_title_font =
            require_font_opt(&self.dialog.title_font, "dialog.title_font", &mut missing);

        // --- spinner ---

        let spinner_fill = require(&self.spinner.fill_color, "spinner.fill_color", &mut missing);
        let spinner_diameter = require(&self.spinner.diameter, "spinner.diameter", &mut missing);
        let spinner_min_size = require(
            &self.spinner.min_diameter,
            "spinner.min_diameter",
            &mut missing,
        );
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
        let combo_box_arrow_size = require(
            &self.combo_box.arrow_icon_size,
            "combo_box.arrow_icon_size",
            &mut missing,
        );
        let combo_box_arrow_area_width = require(
            &self.combo_box.arrow_area_width,
            "combo_box.arrow_area_width",
            &mut missing,
        );

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

        // --- card ---

        let card_background = require(
            &self.card.background_color,
            "card.background_color",
            &mut missing,
        );

        // --- expander ---

        let expander_header_height = require(
            &self.expander.header_height,
            "expander.header_height",
            &mut missing,
        );
        let expander_arrow_size = require(
            &self.expander.arrow_icon_size,
            "expander.arrow_icon_size",
            &mut missing,
        );

        // --- link ---

        let link_visited = require(
            &self.link.visited_text_color,
            "link.visited_text_color",
            &mut missing,
        );
        let link_background = require(
            &self.link.background_color,
            "link.background_color",
            &mut missing,
        );
        let link_hover_bg = require(
            &self.link.hover_background,
            "link.hover_background",
            &mut missing,
        );
        let link_underline = require(&self.link.underline_enabled, "link.underline", &mut missing);

        // --- icon_set / icon_theme ---

        let icon_set = require(&self.icon_set, "icon_set", &mut missing);
        let icon_theme = require(&self.icon_theme, "icon_theme", &mut missing);

        // NEW WIDGET: add require() calls above this line, then add
        // range checks below and construction fields at the bottom.

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
        check_non_negative(
            defaults_radius,
            "defaults.border.corner_radius",
            &mut missing,
        );
        check_non_negative(
            defaults_radius_lg,
            "defaults.border.corner_radius_lg",
            &mut missing,
        );
        check_non_negative(
            defaults_frame_width,
            "defaults.border.line_width",
            &mut missing,
        );
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
            "defaults.border.opacity",
            &mut missing,
        );

        // defaults: border padding >= 0
        check_non_negative(
            defaults_border_padding_h,
            "defaults.border.padding_horizontal",
            &mut missing,
        );
        check_non_negative(
            defaults_border_padding_v,
            "defaults.border.padding_vertical",
            &mut missing,
        );

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
        check_non_negative(button_icon_spacing, "button.icon_text_gap", &mut missing);
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
            "checkbox.indicator_width",
            &mut missing,
        );
        check_non_negative(checkbox_spacing, "checkbox.label_gap", &mut missing);

        // menu: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(menu_item_height, "menu.row_height", &mut missing);
        check_non_negative(menu_icon_spacing, "menu.icon_text_gap", &mut missing);
        check_positive(menu_font.size, "menu.font.size", &mut missing);
        check_range_u16(menu_font.weight, 100, 900, "menu.font.weight", &mut missing);

        // tooltip: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(tooltip_max_width, "tooltip.max_width", &mut missing);
        check_positive(tooltip_font.size, "tooltip.font.size", &mut missing);
        check_range_u16(
            tooltip_font.weight,
            100,
            900,
            "tooltip.font.weight",
            &mut missing,
        );

        // scrollbar: geometry >= 0
        check_non_negative(scrollbar_width, "scrollbar.groove_width", &mut missing);
        check_non_negative(
            scrollbar_min_thumb_height,
            "scrollbar.min_thumb_length",
            &mut missing,
        );
        check_non_negative(
            scrollbar_slider_width,
            "scrollbar.thumb_width",
            &mut missing,
        );

        // slider: geometry >= 0
        check_non_negative(slider_track_height, "slider.track_height", &mut missing);
        check_non_negative(slider_thumb_size, "slider.thumb_diameter", &mut missing);
        check_non_negative(slider_tick_length, "slider.tick_mark_length", &mut missing);

        // progress_bar: geometry >= 0
        check_non_negative(
            progress_bar_height,
            "progress_bar.track_height",
            &mut missing,
        );
        check_non_negative(
            progress_bar_min_width,
            "progress_bar.min_width",
            &mut missing,
        );
        // progress_bar.border.corner_radius check handled by border spec validation

        // tab: geometry >= 0
        check_non_negative(tab_min_width, "tab.min_width", &mut missing);
        check_non_negative(tab_min_height, "tab.min_height", &mut missing);

        // toolbar: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(toolbar_height, "toolbar.bar_height", &mut missing);
        check_non_negative(toolbar_item_spacing, "toolbar.item_gap", &mut missing);
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
        check_non_negative(list_item_height, "list.row_height", &mut missing);

        // splitter: width >= 0
        check_non_negative(splitter_width, "splitter.divider_width", &mut missing);

        // switch: geometry >= 0
        check_non_negative(switch_track_width, "switch.track_width", &mut missing);
        check_non_negative(switch_track_height, "switch.track_height", &mut missing);
        check_non_negative(switch_thumb_size, "switch.thumb_diameter", &mut missing);
        check_non_negative(switch_track_radius, "switch.track_radius", &mut missing);

        // dialog: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(dialog_min_width, "dialog.min_width", &mut missing);
        check_non_negative(dialog_max_width, "dialog.max_width", &mut missing);
        check_non_negative(dialog_min_height, "dialog.min_height", &mut missing);
        check_non_negative(dialog_max_height, "dialog.max_height", &mut missing);
        check_non_negative(dialog_button_spacing, "dialog.button_gap", &mut missing);
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

        // dialog: cross-field min/max constraints
        check_min_max(
            dialog_min_width,
            dialog_max_width,
            "dialog.min_width",
            "dialog.max_width",
            &mut missing,
        );
        check_min_max(
            dialog_min_height,
            dialog_max_height,
            "dialog.min_height",
            "dialog.max_height",
            &mut missing,
        );

        // spinner: geometry >= 0
        check_non_negative(spinner_diameter, "spinner.diameter", &mut missing);
        check_non_negative(spinner_min_size, "spinner.min_diameter", &mut missing);
        check_non_negative(spinner_stroke_width, "spinner.stroke_width", &mut missing);

        // combo_box: geometry >= 0
        check_non_negative(combo_box_min_height, "combo_box.min_height", &mut missing);
        check_non_negative(combo_box_min_width, "combo_box.min_width", &mut missing);
        check_non_negative(
            combo_box_arrow_size,
            "combo_box.arrow_icon_size",
            &mut missing,
        );
        check_non_negative(
            combo_box_arrow_area_width,
            "combo_box.arrow_area_width",
            &mut missing,
        );

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

        // expander: geometry >= 0
        check_non_negative(
            expander_header_height,
            "expander.header_height",
            &mut missing,
        );
        check_non_negative(
            expander_arrow_size,
            "expander.arrow_icon_size",
            &mut missing,
        );

        // NEW WIDGET: add range checks above this line.

        // --- check for missing fields and range errors ---

        if !missing.is_empty() {
            return Err(crate::Error::Resolution(ThemeResolutionError {
                missing_fields: missing,
            }));
        }

        // All fields present -- construct ResolvedThemeVariant.
        // require() returns T directly (using T::default() as placeholder for missing),
        // so no unwrap() is needed. The defaults are never used: we returned Err above.

        // Placeholder bindings for new fields added in Plan 01 that do not yet have
        // require() calls. Phase 51 will wire these properly with inheritance.
        let default_border_spec = ResolvedBorderSpec::default();
        let default_font = defaults_font.clone();
        let window_border_spec = default_border_spec.clone();
        let button_hover_background = defaults_background;
        let button_hover_text_color = defaults_foreground;
        let button_border_spec = default_border_spec.clone();
        let input_disabled_opacity = defaults_disabled_opacity;
        let input_border_spec = default_border_spec.clone();
        let checkbox_background_color = defaults_background;
        let checkbox_indicator_color = defaults_foreground;
        let checkbox_disabled_opacity = defaults_disabled_opacity;
        let checkbox_font = default_font.clone();
        let checkbox_border_spec = default_border_spec.clone();
        let menu_icon_size = defaults_icon_sizes_toolbar;
        let menu_hover_background = defaults_selection;
        let menu_hover_text_color = defaults_selection_foreground;
        let menu_disabled_text_color = defaults_disabled_foreground;
        let menu_border_spec = default_border_spec.clone();
        let tooltip_border_spec = default_border_spec.clone();
        let slider_disabled_opacity = defaults_disabled_opacity;
        let progress_bar_border_spec = default_border_spec.clone();
        let tab_font = default_font.clone();
        let tab_border_spec = default_border_spec.clone();
        let sidebar_selection_background = defaults_selection;
        let sidebar_selection_text_color = defaults_selection_foreground;
        let sidebar_hover_background = defaults_background;
        let sidebar_font = default_font.clone();
        let sidebar_border_spec = default_border_spec.clone();
        let toolbar_background_color = defaults_background;
        let toolbar_icon_size = defaults_icon_sizes_toolbar;
        let toolbar_border_spec = default_border_spec.clone();
        let status_bar_background_color = defaults_background;
        let status_bar_border_spec = default_border_spec.clone();
        let list_item_font = default_font.clone();
        let list_header_font = default_font.clone();
        let list_hover_background = defaults_background;
        let list_border_spec = default_border_spec.clone();
        let popover_font = default_font.clone();
        let popover_border_spec = default_border_spec.clone();
        let splitter_divider_color = defaults_border;
        let separator_line_width = defaults_frame_width;
        let switch_disabled_opacity = defaults_disabled_opacity;
        let dialog_background_color = defaults_surface;
        let dialog_body_font = default_font.clone();
        let dialog_border_spec = default_border_spec.clone();
        let link_font = default_font.clone();
        let combo_box_background_color = defaults_background;
        let combo_box_disabled_opacity = defaults_disabled_opacity;
        let combo_box_font = default_font.clone();
        let combo_box_border_spec = default_border_spec.clone();
        let segmented_control_background_color = defaults_background;
        let segmented_control_active_background = defaults_accent;
        let segmented_control_active_text_color = defaults_accent_foreground;
        let segmented_control_disabled_opacity = defaults_disabled_opacity;
        let segmented_control_font = default_font.clone();
        let segmented_control_border_spec = default_border_spec.clone();
        let card_border_spec = default_border_spec.clone();
        let expander_font = default_font.clone();
        let expander_border_spec = default_border_spec;

        Ok(ResolvedThemeVariant {
            defaults: ResolvedThemeDefaults {
                font: defaults_font,
                line_height: defaults_line_height,
                mono_font: defaults_mono_font,
                background_color: defaults_background,
                text_color: defaults_foreground,
                accent_color: defaults_accent,
                accent_text_color: defaults_accent_foreground,
                surface_color: defaults_surface,
                border: ResolvedBorderSpec {
                    color: defaults_border,
                    corner_radius: defaults_radius,
                    corner_radius_lg: defaults_radius_lg,
                    line_width: defaults_frame_width,
                    opacity: defaults_border_opacity,
                    shadow_enabled: defaults_shadow_enabled,
                    padding_horizontal: defaults_border_padding_h,
                    padding_vertical: defaults_border_padding_v,
                },
                muted_color: defaults_muted,
                shadow_color: defaults_shadow,
                link_color: defaults_link,
                selection_background: defaults_selection,
                selection_text_color: defaults_selection_foreground,
                selection_inactive_background: defaults_selection_inactive,
                text_selection_background: defaults_text_selection_background,
                text_selection_color: defaults_text_selection_color,
                disabled_text_color: defaults_disabled_foreground,
                danger_color: defaults_danger,
                danger_text_color: defaults_danger_foreground,
                warning_color: defaults_warning,
                warning_text_color: defaults_warning_foreground,
                success_color: defaults_success,
                success_text_color: defaults_success_foreground,
                info_color: defaults_info,
                info_text_color: defaults_info_foreground,
                disabled_opacity: defaults_disabled_opacity,
                focus_ring_color: defaults_focus_ring_color,
                focus_ring_width: defaults_focus_ring_width,
                focus_ring_offset: defaults_focus_ring_offset,
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
                background_color: window_background,
                title_bar_background: window_title_bar_background,
                inactive_title_bar_background: window_inactive_title_bar_background,
                inactive_title_bar_text_color: window_inactive_title_bar_foreground,
                title_bar_font: window_title_bar_font,
                border: window_border_spec,
            },
            button: crate::model::widgets::ResolvedButtonTheme {
                background_color: button_background,
                primary_background: button_primary_background,
                primary_text_color: button_primary_foreground,
                min_width: button_min_width,
                min_height: button_min_height,
                icon_text_gap: button_icon_spacing,
                disabled_opacity: button_disabled_opacity,
                hover_background: button_hover_background,
                hover_text_color: button_hover_text_color,
                font: button_font,
                border: button_border_spec,
            },
            input: crate::model::widgets::ResolvedInputTheme {
                background_color: input_background,
                placeholder_color: input_placeholder,
                caret_color: input_caret,
                selection_background: input_selection,
                selection_text_color: input_selection_foreground,
                min_height: input_min_height,
                disabled_opacity: input_disabled_opacity,
                font: input_font,
                border: input_border_spec,
            },
            checkbox: crate::model::widgets::ResolvedCheckboxTheme {
                background_color: checkbox_background_color,
                checked_background: checkbox_checked_background,
                indicator_color: checkbox_indicator_color,
                indicator_width: checkbox_indicator_size,
                label_gap: checkbox_spacing,
                disabled_opacity: checkbox_disabled_opacity,
                font: checkbox_font,
                border: checkbox_border_spec,
            },
            menu: crate::model::widgets::ResolvedMenuTheme {
                background_color: menu_background,
                separator_color: menu_separator,
                row_height: menu_item_height,
                icon_text_gap: menu_icon_spacing,
                icon_size: menu_icon_size,
                hover_background: menu_hover_background,
                hover_text_color: menu_hover_text_color,
                disabled_text_color: menu_disabled_text_color,
                font: menu_font,
                border: menu_border_spec,
            },
            tooltip: crate::model::widgets::ResolvedTooltipTheme {
                background_color: tooltip_background,
                max_width: tooltip_max_width,
                font: tooltip_font,
                border: tooltip_border_spec,
            },
            scrollbar: crate::model::widgets::ResolvedScrollbarTheme {
                track_color: scrollbar_track,
                thumb_color: scrollbar_thumb,
                thumb_hover_color: scrollbar_thumb_hover,
                groove_width: scrollbar_width,
                min_thumb_length: scrollbar_min_thumb_height,
                thumb_width: scrollbar_slider_width,
                overlay_mode: scrollbar_overlay_mode,
            },
            slider: crate::model::widgets::ResolvedSliderTheme {
                fill_color: slider_fill,
                track_color: slider_track,
                thumb_color: slider_thumb,
                track_height: slider_track_height,
                thumb_diameter: slider_thumb_size,
                tick_mark_length: slider_tick_length,
                disabled_opacity: slider_disabled_opacity,
            },
            progress_bar: crate::model::widgets::ResolvedProgressBarTheme {
                fill_color: progress_bar_fill,
                track_color: progress_bar_track,
                track_height: progress_bar_height,
                min_width: progress_bar_min_width,
                border: progress_bar_border_spec,
            },
            tab: crate::model::widgets::ResolvedTabTheme {
                background_color: tab_background,
                active_background: tab_active_background,
                active_text_color: tab_active_foreground,
                bar_background: tab_bar_background,
                min_width: tab_min_width,
                min_height: tab_min_height,
                font: tab_font,
                border: tab_border_spec,
            },
            sidebar: crate::model::widgets::ResolvedSidebarTheme {
                background_color: sidebar_background,
                selection_background: sidebar_selection_background,
                selection_text_color: sidebar_selection_text_color,
                hover_background: sidebar_hover_background,
                font: sidebar_font,
                border: sidebar_border_spec,
            },
            toolbar: crate::model::widgets::ResolvedToolbarTheme {
                background_color: toolbar_background_color,
                bar_height: toolbar_height,
                item_gap: toolbar_item_spacing,
                icon_size: toolbar_icon_size,
                font: toolbar_font,
                border: toolbar_border_spec,
            },
            status_bar: crate::model::widgets::ResolvedStatusBarTheme {
                background_color: status_bar_background_color,
                font: status_bar_font,
                border: status_bar_border_spec,
            },
            list: crate::model::widgets::ResolvedListTheme {
                background_color: list_background,
                alternate_row_background: list_alternate_row,
                selection_background: list_selection,
                selection_text_color: list_selection_foreground,
                header_background: list_header_background,
                grid_color: list_grid_color,
                row_height: list_item_height,
                hover_background: list_hover_background,
                item_font: list_item_font,
                header_font: list_header_font,
                border: list_border_spec,
            },
            popover: crate::model::widgets::ResolvedPopoverTheme {
                background_color: popover_background,
                font: popover_font,
                border: popover_border_spec,
            },
            splitter: crate::model::widgets::ResolvedSplitterTheme {
                divider_width: splitter_width,
                divider_color: splitter_divider_color,
            },
            separator: crate::model::widgets::ResolvedSeparatorTheme {
                line_color: separator_color,
                line_width: separator_line_width,
            },
            switch: crate::model::widgets::ResolvedSwitchTheme {
                checked_background: switch_checked_background,
                unchecked_background: switch_unchecked_background,
                thumb_background: switch_thumb_background,
                track_width: switch_track_width,
                track_height: switch_track_height,
                thumb_diameter: switch_thumb_size,
                track_radius: switch_track_radius,
                disabled_opacity: switch_disabled_opacity,
            },
            dialog: crate::model::widgets::ResolvedDialogTheme {
                background_color: dialog_background_color,
                min_width: dialog_min_width,
                max_width: dialog_max_width,
                min_height: dialog_min_height,
                max_height: dialog_max_height,
                button_gap: dialog_button_spacing,
                icon_size: dialog_icon_size,
                button_order: dialog_button_order,
                title_font: dialog_title_font,
                body_font: dialog_body_font,
                border: dialog_border_spec,
            },
            spinner: crate::model::widgets::ResolvedSpinnerTheme {
                fill_color: spinner_fill,
                diameter: spinner_diameter,
                min_diameter: spinner_min_size,
                stroke_width: spinner_stroke_width,
            },
            combo_box: crate::model::widgets::ResolvedComboBoxTheme {
                background_color: combo_box_background_color,
                min_height: combo_box_min_height,
                min_width: combo_box_min_width,
                arrow_icon_size: combo_box_arrow_size,
                arrow_area_width: combo_box_arrow_area_width,
                disabled_opacity: combo_box_disabled_opacity,
                font: combo_box_font,
                border: combo_box_border_spec,
            },
            segmented_control: crate::model::widgets::ResolvedSegmentedControlTheme {
                background_color: segmented_control_background_color,
                active_background: segmented_control_active_background,
                active_text_color: segmented_control_active_text_color,
                segment_height: segmented_control_segment_height,
                separator_width: segmented_control_separator_width,
                disabled_opacity: segmented_control_disabled_opacity,
                font: segmented_control_font,
                border: segmented_control_border_spec,
            },
            card: crate::model::widgets::ResolvedCardTheme {
                background_color: card_background,
                border: card_border_spec,
            },
            expander: crate::model::widgets::ResolvedExpanderTheme {
                header_height: expander_header_height,
                arrow_icon_size: expander_arrow_size,
                font: expander_font,
                border: expander_border_spec,
            },
            link: crate::model::widgets::ResolvedLinkTheme {
                visited_text_color: link_visited,
                underline_enabled: link_underline,
                background_color: link_background,
                hover_background: link_hover_bg,
                font: link_font,
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
        v.defaults.accent_color = Some(c1);
        v.defaults.background_color = Some(c2);
        v.defaults.text_color = Some(c3);
        v.defaults.surface_color = Some(c4);
        v.defaults.border.color = Some(c5);
        v.defaults.muted_color = Some(c6);
        v.defaults.shadow_color = Some(c7);
        v.defaults.link_color = Some(c8);
        v.defaults.accent_text_color = Some(c9);
        v.defaults.selection_text_color = Some(Rgba::rgb(255, 255, 255));
        v.defaults.disabled_text_color = Some(Rgba::rgb(160, 160, 160));
        v.defaults.danger_color = Some(c10);
        v.defaults.danger_text_color = Some(c11);
        v.defaults.warning_color = Some(c12);
        v.defaults.warning_text_color = Some(c13);
        v.defaults.success_color = Some(c14);
        v.defaults.success_text_color = Some(c15);
        v.defaults.info_color = Some(c16);
        v.defaults.info_text_color = Some(c17);

        v.defaults.border.corner_radius = Some(4.0);
        v.defaults.border.corner_radius_lg = Some(8.0);
        v.defaults.border.line_width = Some(1.0);
        v.defaults.disabled_opacity = Some(0.5);
        v.defaults.border.opacity = Some(0.15);
        v.defaults.border.shadow_enabled = Some(true);

        v.defaults.focus_ring_width = Some(2.0);
        v.defaults.focus_ring_offset = Some(1.0);

        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
        };
        v.defaults.line_height = Some(1.4);
        v.defaults.mono_font = FontSpec {
            family: Some("JetBrains Mono".into()),
            size: Some(13.0),
            weight: Some(400),
            ..Default::default()
        };

        // REMOVED: defaults.spacing not in new schema
        // REMOVED: defaults.spacing not in new schema
        // REMOVED: defaults.spacing not in new schema
        // REMOVED: defaults.spacing not in new schema
        // REMOVED: defaults.spacing not in new schema
        // REMOVED: defaults.spacing not in new schema
        // REMOVED: defaults.spacing not in new schema

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
        v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        v.resolve();
        assert_eq!(
            v.defaults.selection_background,
            Some(Rgba::rgb(0, 120, 215))
        );
        assert_eq!(v.defaults.focus_ring_color, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn resolve_phase1_selection_fills_selection_inactive() {
        let mut v = ThemeVariant::default();
        v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        v.resolve();
        // selection_inactive should be set from selection (which was set from accent)
        assert_eq!(
            v.defaults.selection_inactive_background,
            Some(Rgba::rgb(0, 120, 215))
        );
    }

    #[test]
    fn resolve_phase1_explicit_selection_preserved() {
        let mut v = ThemeVariant::default();
        v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        v.defaults.selection_background = Some(Rgba::rgb(100, 100, 100));
        v.resolve();
        // Explicit selection preserved
        assert_eq!(
            v.defaults.selection_background,
            Some(Rgba::rgb(100, 100, 100))
        );
        // selection_inactive inherits from the explicit selection
        assert_eq!(
            v.defaults.selection_inactive_background,
            Some(Rgba::rgb(100, 100, 100))
        );
    }

    #[test]
    fn resolve_phase1_font_color_from_text_color() {
        let mut v = ThemeVariant::default();
        v.defaults.text_color = Some(Rgba::rgb(30, 30, 30));
        v.resolve();
        assert_eq!(
            v.defaults.font.color,
            Some(Rgba::rgb(30, 30, 30)),
            "defaults.font.color <- defaults.text_color"
        );
        assert_eq!(
            v.defaults.mono_font.color,
            Some(Rgba::rgb(30, 30, 30)),
            "defaults.mono_font.color <- defaults.font.color"
        );
    }

    #[test]
    fn resolve_phase1_font_color_explicit_preserved() {
        let mut v = ThemeVariant::default();
        v.defaults.text_color = Some(Rgba::rgb(30, 30, 30));
        v.defaults.font.color = Some(Rgba::rgb(50, 50, 50));
        v.resolve();
        assert_eq!(
            v.defaults.font.color,
            Some(Rgba::rgb(50, 50, 50)),
            "explicit font.color preserved"
        );
        // mono_font inherits from font.color, not text_color
        assert_eq!(
            v.defaults.mono_font.color,
            Some(Rgba::rgb(50, 50, 50)),
            "mono_font.color <- font.color (not text_color)"
        );
    }

    // ===== Phase 2: Safety nets =====

    #[test]
    fn resolve_phase2_safety_nets() {
        let mut v = ThemeVariant::default();
        v.defaults.text_color = Some(Rgba::rgb(30, 30, 30));
        v.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        v.defaults.muted_color = Some(Rgba::rgb(128, 128, 128));
        v.resolve();

        // Removed safety nets: line_height, accent_text_color, shadow_color,
        // disabled_text_color are no longer fabricated -- all 20 presets provide them.
        assert_eq!(
            v.defaults.line_height, None,
            "line_height no longer fabricated"
        );
        assert_eq!(
            v.defaults.accent_text_color, None,
            "accent_text_color no longer fabricated"
        );
        assert_eq!(
            v.defaults.shadow_color, None,
            "shadow_color no longer fabricated"
        );
        assert_eq!(
            v.defaults.disabled_text_color, None,
            "disabled_text_color no longer fabricated"
        );

        // Kept safety nets (per_platform rules, not invented values):
        assert!(
            v.dialog.button_order.is_some(),
            "dialog.button_order safety net"
        );
        assert_eq!(
            v.input.caret_color,
            Some(Rgba::rgb(30, 30, 30)),
            "input.caret <- foreground"
        );
        assert_eq!(
            v.scrollbar.track_color,
            Some(Rgba::rgb(255, 255, 255)),
            "scrollbar.track <- background"
        );
        assert_eq!(
            v.spinner.fill_color,
            Some(Rgba::rgb(0, 120, 215)),
            "spinner.fill <- accent"
        );
        assert_eq!(
            v.popover.background_color,
            Some(Rgba::rgb(255, 255, 255)),
            "popover.background <- background"
        );
        assert_eq!(
            v.list.background_color,
            Some(Rgba::rgb(255, 255, 255)),
            "list.background <- background"
        );
        assert_eq!(
            v.dialog.background_color,
            Some(Rgba::rgb(255, 255, 255)),
            "dialog.background <- background"
        );
    }

    // ===== Phase 3: Accent propagation (RESOLVE-06) =====

    #[test]
    fn resolve_phase3_accent_propagation() {
        let mut v = ThemeVariant::default();
        v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        v.resolve();

        assert_eq!(
            v.button.primary_background,
            Some(Rgba::rgb(0, 120, 215)),
            "button.primary_background <- accent"
        );
        assert_eq!(
            v.checkbox.checked_background,
            Some(Rgba::rgb(0, 120, 215)),
            "checkbox.checked_background <- accent"
        );
        assert_eq!(
            v.slider.fill_color,
            Some(Rgba::rgb(0, 120, 215)),
            "slider.fill <- accent"
        );
        assert_eq!(
            v.progress_bar.fill_color,
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
            ..Default::default()
        };
        // Menu has a font with only size set
        v.menu.font = Some(FontSpec {
            family: None,
            size: Some(12.0),
            weight: None,
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        };
        v.defaults.line_height = Some(1.4);
        // Leave text_scale entries as None
        v.resolve();

        // caption: 0.82x body size = 14.0 * 0.82 = 11.48, body weight (400)
        let caption = v.text_scale.caption.as_ref().unwrap();
        let expected_caption_size = 14.0 * 0.82;
        assert!(
            (caption.size.unwrap() - expected_caption_size).abs() < 0.001,
            "caption size = 0.82 * body size, got {:?}",
            caption.size
        );
        assert_eq!(caption.weight, Some(400), "caption weight from body weight");
        // line_height = defaults.line_height * caption_size = 1.4 * 11.48 = 16.072
        let expected_caption_lh = 1.4 * expected_caption_size;
        assert!(
            (caption.line_height.unwrap() - expected_caption_lh).abs() < 0.001,
            "caption line_height computed"
        );

        // section_heading: 1.0x body size = 14.0, bold (700)
        let sh = v.text_scale.section_heading.as_ref().unwrap();
        assert!(
            (sh.size.unwrap() - 14.0).abs() < 0.001,
            "section_heading size = 1.0 * body size"
        );
        assert_eq!(
            sh.weight,
            Some(700),
            "section_heading weight defaults to bold"
        );

        // dialog_title: 1.2x body size = 16.8, bold (700)
        let dt = v.text_scale.dialog_title.as_ref().unwrap();
        let expected_dt_size = 14.0 * 1.2;
        assert!(
            (dt.size.unwrap() - expected_dt_size).abs() < 0.001,
            "dialog_title size = 1.2 * body size"
        );
        assert_eq!(dt.weight, Some(700), "dialog_title weight defaults to bold");

        // display: 2.0x body size = 28.0, bold (700)
        let disp = v.text_scale.display.as_ref().unwrap();
        let expected_disp_size = 14.0 * 2.0;
        assert!(
            (disp.size.unwrap() - expected_disp_size).abs() < 0.001,
            "display size = 2.0 * body size"
        );
        assert_eq!(disp.weight, Some(700), "display weight defaults to bold");
    }

    // ===== Phase 3: Color inheritance =====

    #[test]
    fn resolve_phase3_color_inheritance() {
        let mut v = variant_with_defaults();
        v.resolve();

        // window
        assert_eq!(v.window.background_color, Some(Rgba::rgb(255, 255, 255)));
        assert_eq!(
            v.window.border.as_ref().and_then(|b| b.color),
            v.defaults.border.color
        );
        // button
        assert_eq!(
            v.button.border.as_ref().and_then(|b| b.color),
            v.defaults.border.color
        );
        // tooltip
        assert_eq!(
            v.tooltip.border.as_ref().and_then(|b| b.corner_radius),
            v.defaults.border.corner_radius
        );
    }

    // ===== Phase 4: Widget-to-widget =====

    #[test]
    fn resolve_phase4_inactive_title_bar_from_active() {
        let mut v = ThemeVariant::default();
        v.defaults.surface_color = Some(Rgba::rgb(240, 240, 240));
        v.defaults.text_color = Some(Rgba::rgb(30, 30, 30));
        v.resolve();

        // title_bar_background was set to surface in Phase 3
        // inactive should inherit from active
        assert_eq!(
            v.window.inactive_title_bar_background,
            v.window.title_bar_background
        );
        assert_eq!(
            v.window.inactive_title_bar_text_color,
            v.window.title_bar_font.as_ref().and_then(|f| f.color)
        );
    }

    // ===== Preserve explicit values =====

    #[test]
    fn resolve_does_not_overwrite_existing_some_values() {
        let mut v = variant_with_defaults();
        let explicit = Rgba::rgb(255, 0, 0);
        v.window.background_color = Some(explicit);
        v.button.primary_background = Some(explicit);
        v.resolve();

        assert_eq!(
            v.window.background_color,
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

    #[test]
    fn scrollbar_thumb_hover_inherits_muted_color() {
        let mut v = variant_with_defaults();
        let muted = v.defaults.muted_color;
        // Ensure thumb and thumb_hover are not pre-set so resolve derives them
        v.scrollbar.thumb_color = None;
        v.scrollbar.thumb_hover_color = None;
        v.resolve();
        assert!(
            v.scrollbar.thumb_hover_color.is_some(),
            "thumb_hover should be resolved"
        );
        assert_eq!(
            v.scrollbar.thumb_hover_color, muted,
            "thumb_hover_color should inherit from defaults.muted_color"
        );
        // Both thumb_color and thumb_hover_color inherit from muted_color
        assert_eq!(
            v.scrollbar.thumb_color, muted,
            "thumb_color should also inherit from defaults.muted_color"
        );
    }

    // ===== All 8 font-carrying widgets get resolved fonts =====

    #[test]
    fn resolve_all_font_carrying_widgets_get_resolved_fonts() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
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
        v.defaults.selection_background = Some(Rgba::rgb(0, 120, 215));
        v.defaults.selection_text_color = Some(Rgba::rgb(255, 255, 255));
        v.defaults.selection_inactive_background = Some(Rgba::rgb(0, 120, 215));
        v.defaults.text_selection_background = Some(Rgba::rgb(0, 120, 215));
        v.defaults.text_selection_color = Some(Rgba::rgb(255, 255, 255));
        v.defaults.focus_ring_color = Some(Rgba::rgb(0, 120, 215));
        v.defaults.border.padding_horizontal = Some(0.0);
        v.defaults.border.padding_vertical = Some(0.0);

        // icon_set / icon_theme
        v.icon_set = Some(crate::IconSet::Freedesktop);
        v.icon_theme = Some("breeze".into());

        // window
        v.window.background_color = Some(c);
        v.window.title_bar_font.get_or_insert_default().color = Some(c);
        v.window.border.get_or_insert_default().color = Some(c);
        v.window.title_bar_background = Some(c);
        v.window.title_bar_font.get_or_insert_default().color = Some(c);
        v.window.inactive_title_bar_background = Some(c);
        v.window.inactive_title_bar_text_color = Some(c);
        v.window.border.get_or_insert_default().corner_radius = Some(8.0);
        v.window.border.get_or_insert_default().shadow_enabled = Some(true);
        v.window.title_bar_font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
        });

        // button
        v.button.background_color = Some(c);
        v.button.font.get_or_insert_default().color = Some(c);
        v.button.border.get_or_insert_default().color = Some(c);
        v.button.primary_background = Some(c);
        v.button.primary_text_color = Some(c);
        v.button.min_width = Some(64.0);
        v.button.min_height = Some(28.0);
        v.button.border.get_or_insert_default().padding_horizontal = Some(12.0);
        v.button.border.get_or_insert_default().padding_vertical = Some(6.0);
        v.button.border.get_or_insert_default().corner_radius = Some(4.0);
        v.button.icon_text_gap = Some(6.0);
        v.button.disabled_opacity = Some(0.5);
        v.button.border.get_or_insert_default().shadow_enabled = Some(false);
        v.button.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
        });

        // input
        v.input.background_color = Some(c);
        v.input.font.get_or_insert_default().color = Some(c);
        v.input.border.get_or_insert_default().color = Some(c);
        v.input.placeholder_color = Some(c);
        v.input.caret_color = Some(c);
        v.input.selection_background = Some(c);
        v.input.selection_text_color = Some(c);
        v.input.min_height = Some(28.0);
        v.input.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.input.border.get_or_insert_default().padding_vertical = Some(4.0);
        v.input.border.get_or_insert_default().corner_radius = Some(4.0);
        v.input.border.get_or_insert_default().line_width = Some(1.0);
        v.input.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
        });

        // checkbox
        v.checkbox.checked_background = Some(c);
        v.checkbox.indicator_width = Some(18.0);
        v.checkbox.label_gap = Some(6.0);
        v.checkbox.border.get_or_insert_default().corner_radius = Some(2.0);
        v.checkbox.border.get_or_insert_default().line_width = Some(1.0);

        // menu
        v.menu.background_color = Some(c);
        v.menu.font.get_or_insert_default().color = Some(c);
        v.menu.separator_color = Some(c);
        v.menu.row_height = Some(28.0);
        v.menu.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.menu.border.get_or_insert_default().padding_vertical = Some(4.0);
        v.menu.icon_text_gap = Some(6.0);
        v.menu.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
        });

        // tooltip
        v.tooltip.background_color = Some(c);
        v.tooltip.font.get_or_insert_default().color = Some(c);
        v.tooltip.border.get_or_insert_default().padding_horizontal = Some(6.0);
        v.tooltip.border.get_or_insert_default().padding_vertical = Some(4.0);
        v.tooltip.max_width = Some(300.0);
        v.tooltip.border.get_or_insert_default().corner_radius = Some(4.0);
        v.tooltip.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
        });

        // scrollbar
        v.scrollbar.track_color = Some(c);
        v.scrollbar.thumb_color = Some(c);
        v.scrollbar.thumb_hover_color = Some(c);
        v.scrollbar.groove_width = Some(14.0);
        v.scrollbar.min_thumb_length = Some(20.0);
        v.scrollbar.thumb_width = Some(8.0);
        v.scrollbar.overlay_mode = Some(false);

        // slider
        v.slider.fill_color = Some(c);
        v.slider.track_color = Some(c);
        v.slider.thumb_color = Some(c);
        v.slider.track_height = Some(4.0);
        v.slider.thumb_diameter = Some(16.0);
        v.slider.tick_mark_length = Some(6.0);

        // progress_bar
        v.progress_bar.fill_color = Some(c);
        v.progress_bar.track_color = Some(c);
        v.progress_bar.track_height = Some(6.0);
        v.progress_bar.min_width = Some(100.0);
        v.progress_bar.border.get_or_insert_default().corner_radius = Some(3.0);

        // tab
        v.tab.background_color = Some(c);
        v.tab.font.get_or_insert_default().color = Some(c);
        v.tab.active_background = Some(c);
        v.tab.active_text_color = Some(c);
        v.tab.bar_background = Some(c);
        v.tab.min_width = Some(60.0);
        v.tab.min_height = Some(32.0);
        v.tab.border.get_or_insert_default().padding_horizontal = Some(12.0);
        v.tab.border.get_or_insert_default().padding_vertical = Some(6.0);

        // sidebar
        v.sidebar.background_color = Some(c);
        v.sidebar.font.get_or_insert_default().color = Some(c);

        // toolbar
        v.toolbar.bar_height = Some(40.0);
        v.toolbar.item_gap = Some(4.0);
        // REMOVED: toolbar.padding not in new schema
        v.toolbar.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
        });

        // status_bar
        v.status_bar.font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            ..Default::default()
        });

        // list
        v.list.background_color = Some(c);
        v.list.item_font.get_or_insert_default().color = Some(c);
        v.list.alternate_row_background = Some(c);
        v.list.selection_background = Some(c);
        v.list.selection_text_color = Some(c);
        v.list.header_background = Some(c);
        v.list.header_font.get_or_insert_default().color = Some(c);
        v.list.grid_color = Some(c);
        v.list.row_height = Some(28.0);
        v.list.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.list.border.get_or_insert_default().padding_vertical = Some(4.0);

        // popover
        v.popover.background_color = Some(c);
        v.popover.font.get_or_insert_default().color = Some(c);
        v.popover.border.get_or_insert_default().color = Some(c);
        v.popover.border.get_or_insert_default().corner_radius = Some(6.0);

        // splitter
        v.splitter.divider_width = Some(4.0);

        // separator
        v.separator.line_color = Some(c);

        // switch
        v.switch.checked_background = Some(c);
        v.switch.unchecked_background = Some(c);
        v.switch.thumb_background = Some(c);
        v.switch.track_width = Some(40.0);
        v.switch.track_height = Some(20.0);
        v.switch.thumb_diameter = Some(14.0);
        v.switch.track_radius = Some(10.0);

        // dialog
        v.dialog.min_width = Some(320.0);
        v.dialog.max_width = Some(600.0);
        v.dialog.min_height = Some(200.0);
        v.dialog.max_height = Some(800.0);
        // REMOVED: content_padding not in new schema
        v.dialog.button_gap = Some(8.0);
        v.dialog.border.get_or_insert_default().corner_radius = Some(8.0);
        v.dialog.icon_size = Some(22.0);
        v.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);
        v.dialog.title_font = Some(FontSpec {
            family: Some("Inter".into()),
            size: Some(16.0),
            weight: Some(700),
            ..Default::default()
        });

        // spinner
        v.spinner.fill_color = Some(c);
        v.spinner.diameter = Some(24.0);
        v.spinner.min_diameter = Some(16.0);
        v.spinner.stroke_width = Some(2.0);

        // combo_box
        v.combo_box.min_height = Some(28.0);
        v.combo_box.min_width = Some(80.0);
        v.combo_box
            .border
            .get_or_insert_default()
            .padding_horizontal = Some(8.0);
        v.combo_box.arrow_icon_size = Some(12.0);
        v.combo_box.arrow_area_width = Some(20.0);
        v.combo_box.border.get_or_insert_default().corner_radius = Some(4.0);

        // segmented_control
        v.segmented_control.segment_height = Some(28.0);
        v.segmented_control.separator_width = Some(1.0);
        v.segmented_control
            .border
            .get_or_insert_default()
            .padding_horizontal = Some(12.0);
        v.segmented_control
            .border
            .get_or_insert_default()
            .corner_radius = Some(4.0);

        // card
        v.card.background_color = Some(c);
        v.card.border.get_or_insert_default().color = Some(c);
        v.card.border.get_or_insert_default().corner_radius = Some(8.0);
        // REMOVED: card.padding not in new schema
        v.card.border.get_or_insert_default().shadow_enabled = Some(true);

        // expander
        v.expander.header_height = Some(32.0);
        v.expander.arrow_icon_size = Some(12.0);
        // REMOVED: content_padding not in new schema
        v.expander.border.get_or_insert_default().corner_radius = Some(4.0);

        // link
        v.link.font.get_or_insert_default().color = Some(c);
        v.link.visited_text_color = Some(c);
        v.link.background_color = Some(c);
        v.link.hover_background = Some(c);
        v.link.underline_enabled = Some(true);

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
        // Remove 3 specific scalar fields (non-cascading)
        v.defaults.muted_color = None;
        v.defaults.link_color = None;
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
        assert!(
            err.missing_fields
                .contains(&"defaults.muted_color".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"defaults.link_color".to_string())
        );
        assert!(err.missing_fields.contains(&"icon_set".to_string()));
    }

    #[test]
    fn validate_error_message_includes_count_and_paths() {
        let mut v = fully_populated_variant();
        v.defaults.muted_color = None;
        v.button.min_height = None;

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        let msg = err.to_string();
        assert!(msg.contains("2 missing field(s)"), "got: {msg}");
        assert!(msg.contains("defaults.muted_color"), "got: {msg}");
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
                .contains(&"defaults.background_color".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"defaults.accent_color".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"defaults.border.corner_radius".to_string())
        );
        assert!(
            err.missing_fields
                .contains(&"defaults.text_selection_background".to_string())
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
        v.button.border.get_or_insert_default().padding_horizontal = Some(12.0);
        v.button.border.get_or_insert_default().padding_vertical = Some(6.0);
        v.button.icon_text_gap = Some(6.0);
        // input sizing
        v.input.min_height = Some(28.0);
        v.input.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.input.border.get_or_insert_default().padding_vertical = Some(4.0);
        // checkbox sizing
        v.checkbox.indicator_width = Some(18.0);
        v.checkbox.label_gap = Some(6.0);
        // menu sizing
        v.menu.row_height = Some(28.0);
        v.menu.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.menu.border.get_or_insert_default().padding_vertical = Some(4.0);
        v.menu.icon_text_gap = Some(6.0);
        // tooltip sizing
        v.tooltip.border.get_or_insert_default().padding_horizontal = Some(6.0);
        v.tooltip.border.get_or_insert_default().padding_vertical = Some(4.0);
        v.tooltip.max_width = Some(300.0);
        // scrollbar sizing
        v.scrollbar.groove_width = Some(14.0);
        v.scrollbar.min_thumb_length = Some(20.0);
        v.scrollbar.thumb_width = Some(8.0);
        v.scrollbar.overlay_mode = Some(false);
        // slider sizing
        v.slider.track_height = Some(4.0);
        v.slider.thumb_diameter = Some(16.0);
        v.slider.tick_mark_length = Some(6.0);
        // progress_bar sizing
        v.progress_bar.track_height = Some(6.0);
        v.progress_bar.min_width = Some(100.0);
        // tab sizing
        v.tab.min_width = Some(60.0);
        v.tab.min_height = Some(32.0);
        v.tab.border.get_or_insert_default().padding_horizontal = Some(12.0);
        v.tab.border.get_or_insert_default().padding_vertical = Some(6.0);
        // toolbar sizing
        v.toolbar.bar_height = Some(40.0);
        v.toolbar.item_gap = Some(4.0);
        // REMOVED: toolbar.padding not in new schema
        // list sizing
        v.list.row_height = Some(28.0);
        v.list.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.list.border.get_or_insert_default().padding_vertical = Some(4.0);
        // splitter
        v.splitter.divider_width = Some(4.0);
        // switch sizing
        v.switch.unchecked_background = Some(Rgba::rgb(180, 180, 180));
        v.switch.track_width = Some(40.0);
        v.switch.track_height = Some(20.0);
        v.switch.thumb_diameter = Some(14.0);
        v.switch.track_radius = Some(10.0);
        // dialog sizing
        v.dialog.min_width = Some(320.0);
        v.dialog.max_width = Some(600.0);
        v.dialog.min_height = Some(200.0);
        v.dialog.max_height = Some(800.0);
        // REMOVED: content_padding not in new schema
        v.dialog.button_gap = Some(8.0);
        v.dialog.icon_size = Some(22.0);
        v.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);
        // spinner sizing
        v.spinner.diameter = Some(24.0);
        v.spinner.min_diameter = Some(16.0);
        v.spinner.stroke_width = Some(2.0);
        // combo_box sizing
        v.combo_box.min_height = Some(28.0);
        v.combo_box.min_width = Some(80.0);
        v.combo_box
            .border
            .get_or_insert_default()
            .padding_horizontal = Some(8.0);
        v.combo_box.arrow_icon_size = Some(12.0);
        v.combo_box.arrow_area_width = Some(20.0);
        // segmented_control sizing
        v.segmented_control.segment_height = Some(28.0);
        v.segmented_control.separator_width = Some(1.0);
        v.segmented_control
            .border
            .get_or_insert_default()
            .padding_horizontal = Some(12.0);
        // card
        // REMOVED: card.padding not in new schema
        // expander
        v.expander.header_height = Some(32.0);
        v.expander.arrow_icon_size = Some(12.0);
        // REMOVED: content_padding not in new schema
        // link
        v.link.background_color = Some(Rgba::rgb(255, 255, 255));
        v.link.hover_background = Some(Rgba::rgb(230, 230, 255));
        v.link.underline_enabled = Some(true);

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
        variant.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);
        // icon_set comes from gsettings icon-theme; simulate typical GNOME value.
        variant.icon_set = Some(crate::IconSet::Freedesktop);

        // Simulate GNOME reader font output (gsettings font-name on a GNOME system).
        variant.defaults.font = FontSpec {
            family: Some("Cantarell".to_string()),
            size: Some(11.0),
            weight: Some(400),
            ..Default::default()
        };

        variant.resolve_all();
        let resolved = variant.validate().unwrap_or_else(|e| {
            panic!("GNOME resolve/validate pipeline failed: {e}");
        });

        // Spot-check: adwaita-base fields present.
        // Adwaita dark accent is #3584e4 = rgb(53, 132, 228)
        assert_eq!(
            resolved.defaults.accent_color,
            Rgba::rgb(53, 132, 228),
            "accent should be from adwaita preset"
        );
        assert_eq!(
            resolved.defaults.font.family, "Cantarell",
            "font family should be from GNOME reader overlay"
        );
        assert_eq!(
            resolved.dialog.button_order,
            DialogButtonOrder::PrimaryRight,
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
        v.defaults.border.corner_radius = Some(-5.0);

        let result = v.validate();
        assert!(result.is_err());
        let crate::Error::Resolution(err) = result.unwrap_err() else {
            // validate() always returns Resolution errors
            return;
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.border.corner_radius") && f.contains("-5")),
            "should report negative defaults.border.corner_radius, got: {:?}",
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
                .any(|f| f.contains("defaults.font.size") && f.contains("positive")),
            "should report zero defaults.font.size, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_opacity_out_of_range() {
        let mut v = fully_populated_variant();
        v.defaults.disabled_opacity = Some(1.5);
        v.defaults.border.opacity = Some(-0.1);
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
                .any(|f| f.contains("defaults.border.opacity")),
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
        v.defaults.border.corner_radius = Some(-1.0);
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
        v.defaults.border.corner_radius = Some(0.0);
        v.defaults.border.corner_radius_lg = Some(0.0);
        v.defaults.border.line_width = Some(0.0);
        v.button.border.get_or_insert_default().corner_radius = Some(0.0);
        v.defaults.disabled_opacity = Some(0.0);
        v.defaults.border.opacity = Some(0.0);

        let result = v.validate();
        assert!(
            result.is_ok(),
            "zero values should be valid for radius/frame_width/opacity, got: {:?}",
            result.err()
        );
    }

    // ===== Additional range-check negative tests (issue 2a) =====

    #[test]
    fn validate_catches_negative_font_size() {
        let mut v = fully_populated_variant();
        v.defaults.font.size = Some(-1.0);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.font.size")),
            "should report negative font.size, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_disabled_opacity_above_one() {
        let mut v = fully_populated_variant();
        v.defaults.disabled_opacity = Some(2.0);

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
            "should report disabled_opacity=2.0 out of 0..=1 range, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_font_weight_zero() {
        let mut v = fully_populated_variant();
        v.defaults.font.weight = Some(0);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.font.weight")),
            "should report font.weight=0 out of 100..=900 range, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_nan_values() {
        let mut v = fully_populated_variant();
        v.defaults.border.corner_radius = Some(f32::NAN);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.border.corner_radius")),
            "should report NaN defaults.radius, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_infinity() {
        let mut v = fully_populated_variant();
        v.defaults.font.size = Some(f32::INFINITY);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.font.size")),
            "should report infinite font.size, got: {:?}",
            err.missing_fields
        );
    }

    #[test]
    fn validate_catches_negative_infinity() {
        let mut v = fully_populated_variant();
        // Set a field to negative infinity to trigger range check failure
        v.defaults.border.line_width = Some(f32::NEG_INFINITY);

        let result = v.validate();
        assert!(result.is_err());
        let err = match result.unwrap_err() {
            crate::Error::Resolution(e) => e,
            other => panic!("expected Resolution error, got: {other:?}"),
        };
        assert!(
            err.missing_fields
                .iter()
                .any(|f| f.contains("defaults.border.line_width")),
            "should report -inf border.line_width, got: {:?}",
            err.missing_fields
        );
    }

    // ===== Derivation chain tests (issues 17a, 17b, 17c) =====

    #[test]
    fn merge_preserves_base_name_when_overlay_name_empty() {
        let mut base = crate::ThemeSpec::new("My Base");
        let overlay = crate::ThemeSpec::new("");
        base.merge(&overlay);
        assert_eq!(base.name, "My Base", "base name should be preserved");
    }

    #[test]
    fn merge_preserves_empty_base_name_over_nonempty_overlay() {
        // Issue 17a edge case: merge() never touches self.name, so an empty
        // base name is kept even when the overlay has a non-empty name.
        let mut base = crate::ThemeSpec::new("");
        let overlay = crate::ThemeSpec::new("Overlay Name");
        base.merge(&overlay);
        assert_eq!(
            base.name, "",
            "empty base name should be preserved (merge never touches name)"
        );
    }

    #[test]
    fn accent_derives_selection_then_selection_inactive() {
        let mut v = ThemeVariant::default();
        let accent = Rgba::rgb(0, 120, 215);
        v.defaults.accent_color = Some(accent);
        v.resolve();

        // accent -> selection -> selection_inactive chain
        assert_eq!(
            v.defaults.selection_background,
            Some(accent),
            "selection should derive from accent"
        );
        assert_eq!(
            v.defaults.selection_inactive_background,
            Some(accent),
            "selection_inactive should derive from selection"
        );
    }

    #[test]
    fn title_bar_background_inherits_from_surface_not_background() {
        let mut v = ThemeVariant::default();
        v.defaults.surface_color = Some(Rgba::rgb(240, 240, 240));
        v.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        v.resolve();

        assert_eq!(
            v.window.title_bar_background,
            Some(Rgba::rgb(240, 240, 240)),
            "title_bar_background should inherit from surface, not background"
        );
        assert_ne!(
            v.window.title_bar_background, v.defaults.background_color,
            "title_bar_background must not equal background"
        );
    }

    // ===== Resolve completeness test =====

    /// Verify that resolve() has rules for every derived field.
    ///
    /// Constructs a ThemeVariant with ONLY root fields (the ~46 defaults
    /// Helper: populate all non-derivable widget geometry/behavior fields.
    ///
    /// These fields have no resolve() rule; they MUST be set explicitly.
    fn set_widget_geometry(v: &mut ThemeVariant) {
        v.icon_set = Some(crate::IconSet::Freedesktop);
        // button
        v.button.min_width = Some(64.0);
        v.button.min_height = Some(28.0);
        v.button.border.get_or_insert_default().padding_horizontal = Some(12.0);
        v.button.border.get_or_insert_default().padding_vertical = Some(6.0);
        v.button.icon_text_gap = Some(6.0);
        // input
        v.input.min_height = Some(28.0);
        v.input.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.input.border.get_or_insert_default().padding_vertical = Some(4.0);
        // checkbox
        v.checkbox.indicator_width = Some(18.0);
        v.checkbox.label_gap = Some(6.0);
        // menu
        v.menu.row_height = Some(28.0);
        v.menu.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.menu.border.get_or_insert_default().padding_vertical = Some(4.0);
        v.menu.icon_text_gap = Some(6.0);
        // tooltip
        v.tooltip.border.get_or_insert_default().padding_horizontal = Some(6.0);
        v.tooltip.border.get_or_insert_default().padding_vertical = Some(4.0);
        v.tooltip.max_width = Some(300.0);
        // scrollbar
        v.scrollbar.groove_width = Some(14.0);
        v.scrollbar.min_thumb_length = Some(20.0);
        v.scrollbar.thumb_width = Some(8.0);
        v.scrollbar.overlay_mode = Some(false);
        // slider
        v.slider.track_height = Some(4.0);
        v.slider.thumb_diameter = Some(16.0);
        v.slider.tick_mark_length = Some(6.0);
        // progress_bar
        v.progress_bar.track_height = Some(6.0);
        v.progress_bar.min_width = Some(100.0);
        // tab
        v.tab.min_width = Some(60.0);
        v.tab.min_height = Some(32.0);
        v.tab.border.get_or_insert_default().padding_horizontal = Some(12.0);
        v.tab.border.get_or_insert_default().padding_vertical = Some(6.0);
        // toolbar
        v.toolbar.bar_height = Some(40.0);
        v.toolbar.item_gap = Some(4.0);
        // REMOVED: toolbar.padding not in new schema
        // list
        v.list.row_height = Some(28.0);
        v.list.border.get_or_insert_default().padding_horizontal = Some(8.0);
        v.list.border.get_or_insert_default().padding_vertical = Some(4.0);
        // splitter
        v.splitter.divider_width = Some(4.0);
        // switch (unchecked_background has no inheritance -- must be preset-provided)
        v.switch.unchecked_background = Some(Rgba::rgb(180, 180, 180));
        v.switch.track_width = Some(40.0);
        v.switch.track_height = Some(20.0);
        v.switch.thumb_diameter = Some(14.0);
        v.switch.track_radius = Some(10.0);
        // card (border sub-fields have no inheritance -- must be preset-provided)
        v.card.border.get_or_insert_default().color = Some(Rgba::rgb(200, 200, 200));
        v.card.border.get_or_insert_default().corner_radius = Some(8.0);
        v.card.border.get_or_insert_default().shadow_enabled = Some(true);
        // dialog
        v.dialog.min_width = Some(320.0);
        v.dialog.max_width = Some(600.0);
        v.dialog.min_height = Some(200.0);
        v.dialog.max_height = Some(800.0);
        // REMOVED: content_padding not in new schema
        v.dialog.button_gap = Some(8.0);
        v.dialog.icon_size = Some(22.0);
        v.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);
        // spinner
        v.spinner.diameter = Some(24.0);
        v.spinner.min_diameter = Some(16.0);
        v.spinner.stroke_width = Some(2.0);
        // combo_box
        v.combo_box.min_height = Some(28.0);
        v.combo_box.min_width = Some(80.0);
        v.combo_box
            .border
            .get_or_insert_default()
            .padding_horizontal = Some(8.0);
        v.combo_box.arrow_icon_size = Some(12.0);
        v.combo_box.arrow_area_width = Some(20.0);
        // segmented_control
        v.segmented_control.segment_height = Some(28.0);
        v.segmented_control.separator_width = Some(1.0);
        v.segmented_control
            .border
            .get_or_insert_default()
            .padding_horizontal = Some(12.0);
        // card
        // REMOVED: card.padding not in new schema
        // expander
        v.expander.header_height = Some(32.0);
        v.expander.arrow_icon_size = Some(12.0);
        // REMOVED: content_padding not in new schema
        // link (background and hover_bg have no derivation path)
        v.link.background_color = Some(Rgba::rgb(255, 255, 255));
        v.link.hover_background = Some(Rgba::rgb(230, 230, 255));
        v.link.underline_enabled = Some(true);
    }

    /// Helper: clear all derived color/font/text_scale fields on a variant.
    ///
    /// After calling this, resolve_all() must be able to reconstruct every
    /// cleared field from the remaining defaults.
    fn clear_derived_fields(v: &mut ThemeVariant) {
        // Widget colors and radii (derived from defaults)
        v.window.background_color = None;
        v.window.title_bar_font = None;
        v.window.border = None;
        v.window.title_bar_background = None;
        v.window.title_bar_font = None;
        v.window.inactive_title_bar_background = None;
        v.window.inactive_title_bar_text_color = None;
        v.window.border = None;
        v.window.border = None;
        v.button.background_color = None;
        v.button.font = None;
        v.button.border = None;
        v.button.primary_background = None;
        v.button.primary_text_color = None;
        v.button.border = None;
        v.button.disabled_opacity = None;
        v.button.border = None;
        v.input.background_color = None;
        v.input.font = None;
        v.input.border = None;
        v.input.placeholder_color = None;
        v.input.caret_color = None;
        v.input.selection_background = None;
        v.input.selection_text_color = None;
        v.input.border = None;
        v.checkbox.checked_background = None;
        v.checkbox.border = None;
        v.menu.background_color = None;
        v.menu.font = None;
        v.menu.separator_color = None;
        v.tooltip.background_color = None;
        v.tooltip.font = None;
        v.tooltip.border = None;
        v.scrollbar.track_color = None;
        v.scrollbar.thumb_color = None;
        v.scrollbar.thumb_hover_color = None;
        v.slider.fill_color = None;
        v.slider.track_color = None;
        v.slider.thumb_color = None;
        v.progress_bar.fill_color = None;
        v.progress_bar.track_color = None;
        v.progress_bar.border = None;
        v.tab.background_color = None;
        v.tab.font = None;
        v.tab.active_background = None;
        v.tab.active_text_color = None;
        v.tab.bar_background = None;
        v.sidebar.background_color = None;
        v.sidebar.font = None;
        v.list.background_color = None;
        v.list.item_font = None;
        v.list.alternate_row_background = None;
        v.list.selection_background = None;
        v.list.selection_text_color = None;
        v.list.header_background = None;
        v.list.header_font = None;
        v.list.grid_color = None;
        v.popover.background_color = None;
        v.popover.font = None;
        v.popover.border = None;
        v.separator.line_color = None;
        v.switch.checked_background = None;
        // switch.unchecked_background: NOT cleared -- no inheritance, must come from preset
        v.switch.thumb_background = None;
        v.dialog.border = None;
        v.combo_box.border = None;
        v.segmented_control.border = None;
        v.card.background_color = None;
        // card.border: NOT cleared -- no inheritance from defaults (INH-3), must come from preset
        v.expander.border = None;
        v.link.font = None;
        v.link.visited_text_color = None;
        v.spinner.fill_color = None;
        // Widget fonts (derived from defaults.font)
        v.window.title_bar_font = None;
        v.button.font = None;
        v.input.font = None;
        v.menu.font = None;
        v.tooltip.font = None;
        v.toolbar.font = None;
        v.status_bar.font = None;
        v.dialog.title_font = None;
        // Text scale (derived from defaults.font + defaults.line_height)
        v.text_scale.caption = None;
        v.text_scale.section_heading = None;
        v.text_scale.dialog_title = None;
        v.text_scale.display = None;
        // Defaults internal chains (derived from accent/selection)
        v.defaults.selection_background = None;
        v.defaults.focus_ring_color = None;
        v.defaults.selection_inactive_background = None;
    }

    /// fields + ~65 widget geometry/behavior fields that have no derivation
    /// path in resolve()). Derived fields (widget colors, widget fonts, text
    /// scale entries, widget-to-widget chains) are left as None.
    ///
    /// If any derived field lacks a resolve rule, it stays None and
    /// validate() reports it as missing -- catching the bug.
    #[test]
    fn resolve_completeness_minimal_variant() {
        let mut v = variant_with_defaults();
        set_widget_geometry(&mut v);

        // Verify: NO derived color/font/text_scale fields are set
        assert!(
            v.window.background_color.is_none(),
            "window.background should be None before resolve"
        );
        assert!(
            v.button.background_color.is_none(),
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
    /// then verifies resolve() can reconstruct them.
    #[test]
    fn resolve_completeness_from_preset() {
        let spec = crate::ThemeSpec::preset("material").unwrap();
        let mut v = spec.dark.expect("material should have dark variant");

        clear_derived_fields(&mut v);

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
