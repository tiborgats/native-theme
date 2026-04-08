// Theme validation: require fields, range-check values, produce ResolvedThemeVariant.

use crate::error::ThemeResolutionError;
use crate::model::border::{BorderSpec, ResolvedBorderSpec};

/// Standard screen DPI (96 dots per inch). Used as the font_dpi fallback
/// when no DPI was set on the unresolved variant (e.g. community presets
/// loaded standalone without OS reader). This matches the CSS/Web reference
/// pixel and the Windows default. Validation uses this to convert
/// `FontSize::Pt` values to logical pixels via `FontSize::to_px(dpi)`.
const DEFAULT_FONT_DPI: f32 = 96.0;
use crate::model::resolved::{
    ResolvedIconSizes, ResolvedTextScale, ResolvedTextScaleEntry, ResolvedThemeDefaults,
    ResolvedThemeVariant,
};
use crate::model::{FontSpec, ResolvedFontSpec, TextScaleEntry, ThemeVariant};

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
/// Checks each sub-field individually. Converts `FontSize` to `f32` px via `to_px(dpi)`.
fn require_font(
    font: &FontSpec,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> ResolvedFontSpec {
    let family = require(&font.family, &format!("{prefix}.family"), missing);
    let size = font.size.map(|fs| fs.to_px(dpi)).unwrap_or_else(|| {
        missing.push(format!("{prefix}.size"));
        0.0
    });
    let weight = require(&font.weight, &format!("{prefix}.weight"), missing);
    let color = require(&font.color, &format!("{prefix}.color"), missing);
    ResolvedFontSpec {
        family,
        size,
        weight,
        style: font.style.unwrap_or_default(),
        color,
    }
}

/// Validate an `Option<FontSpec>` (widget font fields).
/// If None, records the path as missing. Converts `FontSize` to `f32` px via `to_px(dpi)`.
fn require_font_opt(
    font: &Option<FontSpec>,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> ResolvedFontSpec {
    match font {
        None => {
            missing.push(prefix.to_string());
            ResolvedFontSpec::default()
        }
        Some(f) => {
            let family = require(&f.family, &format!("{prefix}.family"), missing);
            let size = f.size.map(|fs| fs.to_px(dpi)).unwrap_or_else(|| {
                missing.push(format!("{prefix}.size"));
                0.0
            });
            let weight = require(&f.weight, &format!("{prefix}.weight"), missing);
            let color = require(&f.color, &format!("{prefix}.color"), missing);
            ResolvedFontSpec {
                family,
                size,
                weight,
                style: f.style.unwrap_or_default(),
                color,
            }
        }
    }
}

/// Validate an `Option<TextScaleEntry>`.
/// Converts `FontSize` to `f32` px via `to_px(dpi)`. Also converts `line_height`
/// when the sibling `size` is in points (same unit, same DPI factor).
fn require_text_scale_entry(
    entry: &Option<TextScaleEntry>,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> ResolvedTextScaleEntry {
    match entry {
        None => {
            missing.push(prefix.to_string());
            ResolvedTextScaleEntry::default()
        }
        Some(e) => {
            let size = e.size.map(|fs| fs.to_px(dpi)).unwrap_or_else(|| {
                missing.push(format!("{prefix}.size"));
                0.0
            });
            let needs_pt_conversion = e.size.is_some_and(|fs| fs.is_pt());
            let line_height = e
                .line_height
                .map(|lh| {
                    if needs_pt_conversion {
                        lh * dpi / 72.0
                    } else {
                        lh
                    }
                })
                .unwrap_or_else(|| {
                    missing.push(format!("{prefix}.line_height"));
                    0.0
                });
            let weight = require(&e.weight, &format!("{prefix}.weight"), missing);
            ResolvedTextScaleEntry {
                size,
                weight,
                line_height,
            }
        }
    }
}

/// Validate an `Option<BorderSpec>` (widget border fields).
/// If None, records the path as missing. Requires the 4 sub-fields filled by
/// border_inheritance (color, corner_radius, line_width, shadow_enabled).
/// Padding sub-fields are sizing fields with no inheritance -- they use
/// the preset value if present, otherwise default to `T::default()`.
fn require_border(
    border: &Option<BorderSpec>,
    prefix: &str,
    missing: &mut Vec<String>,
) -> ResolvedBorderSpec {
    match border {
        None => {
            missing.push(prefix.to_string());
            ResolvedBorderSpec::default()
        }
        Some(b) => {
            let color = require(&b.color, &format!("{prefix}.color"), missing);
            let corner_radius = require(
                &b.corner_radius,
                &format!("{prefix}.corner_radius"),
                missing,
            );
            let line_width = require(&b.line_width, &format!("{prefix}.line_width"), missing);
            let shadow_enabled = require(
                &b.shadow_enabled,
                &format!("{prefix}.shadow_enabled"),
                missing,
            );
            ResolvedBorderSpec {
                color,
                corner_radius,
                corner_radius_lg: b.corner_radius_lg.unwrap_or_default(),
                line_width,
                opacity: b.opacity.unwrap_or_default(),
                shadow_enabled,
                padding_horizontal: b.padding_horizontal.unwrap_or_default(),
                padding_vertical: b.padding_vertical.unwrap_or_default(),
            }
        }
    }
}

/// Resolve a border for widgets excluded from border_inheritance (menu, tab, card).
/// These widgets have no inheritance for any border sub-field; all sub-fields
/// use the preset value if present, otherwise `T::default()`. No validation
/// errors are recorded -- the border is entirely optional.
fn border_all_optional(border: &Option<BorderSpec>) -> ResolvedBorderSpec {
    match border {
        None => ResolvedBorderSpec::default(),
        Some(b) => ResolvedBorderSpec {
            color: b.color.unwrap_or_default(),
            corner_radius: b.corner_radius.unwrap_or_default(),
            corner_radius_lg: b.corner_radius_lg.unwrap_or_default(),
            line_width: b.line_width.unwrap_or_default(),
            opacity: b.opacity.unwrap_or_default(),
            shadow_enabled: b.shadow_enabled.unwrap_or_default(),
            padding_horizontal: b.padding_horizontal.unwrap_or_default(),
            padding_vertical: b.padding_vertical.unwrap_or_default(),
        },
    }
}

/// Validate a border for widgets with partial border inheritance (sidebar, status_bar).
/// Only color + line_width are inherited; other sub-fields use defaults if not in preset.
fn require_border_partial(
    border: &Option<BorderSpec>,
    prefix: &str,
    missing: &mut Vec<String>,
) -> ResolvedBorderSpec {
    match border {
        None => {
            missing.push(prefix.to_string());
            ResolvedBorderSpec::default()
        }
        Some(b) => {
            let color = require(&b.color, &format!("{prefix}.color"), missing);
            let line_width = require(&b.line_width, &format!("{prefix}.line_width"), missing);
            ResolvedBorderSpec {
                color,
                corner_radius: b.corner_radius.unwrap_or_default(),
                corner_radius_lg: b.corner_radius_lg.unwrap_or_default(),
                line_width,
                opacity: b.opacity.unwrap_or_default(),
                shadow_enabled: b.shadow_enabled.unwrap_or_default(),
                padding_horizontal: b.padding_horizontal.unwrap_or_default(),
                padding_vertical: b.padding_vertical.unwrap_or_default(),
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

impl ThemeVariant {
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
    #[must_use = "this returns the validation result; handle the Result or propagate with ?"]
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

        // Extract DPI for FontSize::to_px conversion throughout validate().
        let dpi = self.defaults.font_dpi.unwrap_or(DEFAULT_FONT_DPI);

        let defaults_font = require_font(&self.defaults.font, "defaults.font", dpi, &mut missing);
        let defaults_line_height = require(
            &self.defaults.line_height,
            "defaults.line_height",
            &mut missing,
        );
        let defaults_mono_font = require_font(
            &self.defaults.mono_font,
            "defaults.mono_font",
            dpi,
            &mut missing,
        );

        let defaults_background = require(
            &self.defaults.background_color,
            "defaults.background_color",
            &mut missing,
        );
        let defaults_foreground = require(
            &self.defaults.text_color,
            "defaults.text_color",
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

        // font_dpi records the DPI used for pt-to-px conversion (or the default).
        let defaults_font_dpi = dpi;

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

        let ts_caption = require_text_scale_entry(
            &self.text_scale.caption,
            "text_scale.caption",
            dpi,
            &mut missing,
        );
        let ts_section_heading = require_text_scale_entry(
            &self.text_scale.section_heading,
            "text_scale.section_heading",
            dpi,
            &mut missing,
        );
        let ts_dialog_title = require_text_scale_entry(
            &self.text_scale.dialog_title,
            "text_scale.dialog_title",
            dpi,
            &mut missing,
        );
        let ts_display = require_text_scale_entry(
            &self.text_scale.display,
            "text_scale.display",
            dpi,
            &mut missing,
        );

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
            dpi,
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
        let button_font = require_font_opt(&self.button.font, "button.font", dpi, &mut missing);

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
        let input_font = require_font_opt(&self.input.font, "input.font", dpi, &mut missing);

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
        let menu_font = require_font_opt(&self.menu.font, "menu.font", dpi, &mut missing);

        // --- tooltip ---

        let tooltip_background = require(
            &self.tooltip.background_color,
            "tooltip.background_color",
            &mut missing,
        );
        let tooltip_max_width = require(&self.tooltip.max_width, "tooltip.max_width", &mut missing);
        let tooltip_font = require_font_opt(&self.tooltip.font, "tooltip.font", dpi, &mut missing);

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
        let tab_hover_text_color = require(
            &self.tab.hover_text_color,
            "tab.hover_text_color",
            &mut missing,
        );

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
        let toolbar_font = require_font_opt(&self.toolbar.font, "toolbar.font", dpi, &mut missing);

        // --- status_bar ---

        let status_bar_font =
            require_font_opt(&self.status_bar.font, "status_bar.font", dpi, &mut missing);

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
        let dialog_title_font = require_font_opt(
            &self.dialog.title_font,
            "dialog.title_font",
            dpi,
            &mut missing,
        );

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

        // --- require() calls for fields previously using placeholder bindings ---

        // window
        let window_border_spec = require_border(&self.window.border, "window.border", &mut missing);

        // button
        let button_hover_background = require(
            &self.button.hover_background,
            "button.hover_background",
            &mut missing,
        );
        let button_hover_text_color = require(
            &self.button.hover_text_color,
            "button.hover_text_color",
            &mut missing,
        );
        let button_border_spec = require_border(&self.button.border, "button.border", &mut missing);
        let button_active_text_color = require(
            &self.button.active_text_color,
            "button.active_text_color",
            &mut missing,
        );
        let button_disabled_text_color = require(
            &self.button.disabled_text_color,
            "button.disabled_text_color",
            &mut missing,
        );

        // input
        let input_disabled_opacity = require(
            &self.input.disabled_opacity,
            "input.disabled_opacity",
            &mut missing,
        );
        let input_border_spec = require_border(&self.input.border, "input.border", &mut missing);
        let input_disabled_text_color = require(
            &self.input.disabled_text_color,
            "input.disabled_text_color",
            &mut missing,
        );

        // checkbox
        let checkbox_background_color = require(
            &self.checkbox.background_color,
            "checkbox.background_color",
            &mut missing,
        );
        let checkbox_indicator_color = require(
            &self.checkbox.indicator_color,
            "checkbox.indicator_color",
            &mut missing,
        );
        let checkbox_disabled_opacity = require(
            &self.checkbox.disabled_opacity,
            "checkbox.disabled_opacity",
            &mut missing,
        );
        let checkbox_font =
            require_font_opt(&self.checkbox.font, "checkbox.font", dpi, &mut missing);
        let checkbox_border_spec =
            require_border(&self.checkbox.border, "checkbox.border", &mut missing);
        let checkbox_disabled_text_color = require(
            &self.checkbox.disabled_text_color,
            "checkbox.disabled_text_color",
            &mut missing,
        );

        // menu (excluded from border_inheritance -- border belongs to popup container)
        let menu_icon_size = require(&self.menu.icon_size, "menu.icon_size", &mut missing);
        let menu_hover_background = require(
            &self.menu.hover_background,
            "menu.hover_background",
            &mut missing,
        );
        let menu_hover_text_color = require(
            &self.menu.hover_text_color,
            "menu.hover_text_color",
            &mut missing,
        );
        let menu_disabled_text_color = require(
            &self.menu.disabled_text_color,
            "menu.disabled_text_color",
            &mut missing,
        );
        // menu border: no inheritance, all sub-fields optional (preset provides if available)
        let menu_border_spec = border_all_optional(&self.menu.border);

        // tooltip
        let tooltip_border_spec =
            require_border(&self.tooltip.border, "tooltip.border", &mut missing);

        // slider
        let slider_disabled_opacity = require(
            &self.slider.disabled_opacity,
            "slider.disabled_opacity",
            &mut missing,
        );

        // progress_bar
        let progress_bar_border_spec = require_border(
            &self.progress_bar.border,
            "progress_bar.border",
            &mut missing,
        );

        // tab (excluded from border_inheritance -- all border sub-fields are platform-specific)
        let tab_font = require_font_opt(&self.tab.font, "tab.font", dpi, &mut missing);
        // tab border: no inheritance, all sub-fields optional (preset provides if available)
        let tab_border_spec = border_all_optional(&self.tab.border);

        // sidebar (partial border inheritance: color + line_width only)
        let sidebar_selection_background = require(
            &self.sidebar.selection_background,
            "sidebar.selection_background",
            &mut missing,
        );
        let sidebar_selection_text_color = require(
            &self.sidebar.selection_text_color,
            "sidebar.selection_text_color",
            &mut missing,
        );
        let sidebar_hover_background = require(
            &self.sidebar.hover_background,
            "sidebar.hover_background",
            &mut missing,
        );
        let sidebar_font = require_font_opt(&self.sidebar.font, "sidebar.font", dpi, &mut missing);
        let sidebar_border_spec =
            require_border_partial(&self.sidebar.border, "sidebar.border", &mut missing);

        // toolbar
        let toolbar_background_color = require(
            &self.toolbar.background_color,
            "toolbar.background_color",
            &mut missing,
        );
        let toolbar_icon_size = require(&self.toolbar.icon_size, "toolbar.icon_size", &mut missing);
        let toolbar_border_spec =
            require_border(&self.toolbar.border, "toolbar.border", &mut missing);

        // status_bar (partial border inheritance: color + line_width only)
        let status_bar_background_color = require(
            &self.status_bar.background_color,
            "status_bar.background_color",
            &mut missing,
        );
        let status_bar_border_spec =
            require_border_partial(&self.status_bar.border, "status_bar.border", &mut missing);

        // list
        let list_item_font =
            require_font_opt(&self.list.item_font, "list.item_font", dpi, &mut missing);
        let list_header_font = require_font_opt(
            &self.list.header_font,
            "list.header_font",
            dpi,
            &mut missing,
        );
        let list_hover_background = require(
            &self.list.hover_background,
            "list.hover_background",
            &mut missing,
        );
        let list_hover_text_color = require(
            &self.list.hover_text_color,
            "list.hover_text_color",
            &mut missing,
        );
        let list_disabled_text_color = require(
            &self.list.disabled_text_color,
            "list.disabled_text_color",
            &mut missing,
        );
        let list_border_spec = require_border(&self.list.border, "list.border", &mut missing);

        // popover
        let popover_font = require_font_opt(&self.popover.font, "popover.font", dpi, &mut missing);
        let popover_border_spec =
            require_border(&self.popover.border, "popover.border", &mut missing);

        // splitter
        let splitter_divider_color = require(
            &self.splitter.divider_color,
            "splitter.divider_color",
            &mut missing,
        );
        let splitter_hover_color = require(
            &self.splitter.hover_color,
            "splitter.hover_color",
            &mut missing,
        );

        // separator
        let separator_line_width = require(
            &self.separator.line_width,
            "separator.line_width",
            &mut missing,
        );

        // switch
        let switch_disabled_opacity = require(
            &self.switch.disabled_opacity,
            "switch.disabled_opacity",
            &mut missing,
        );

        // dialog
        let dialog_background_color = require(
            &self.dialog.background_color,
            "dialog.background_color",
            &mut missing,
        );
        let dialog_body_font = require_font_opt(
            &self.dialog.body_font,
            "dialog.body_font",
            dpi,
            &mut missing,
        );
        let dialog_border_spec = require_border(&self.dialog.border, "dialog.border", &mut missing);

        // link
        let link_font = require_font_opt(&self.link.font, "link.font", dpi, &mut missing);
        let link_hover_text_color = require(
            &self.link.hover_text_color,
            "link.hover_text_color",
            &mut missing,
        );
        let link_active_text_color = require(
            &self.link.active_text_color,
            "link.active_text_color",
            &mut missing,
        );
        let link_disabled_text_color = require(
            &self.link.disabled_text_color,
            "link.disabled_text_color",
            &mut missing,
        );

        // combo_box
        let combo_box_background_color = require(
            &self.combo_box.background_color,
            "combo_box.background_color",
            &mut missing,
        );
        let combo_box_disabled_opacity = require(
            &self.combo_box.disabled_opacity,
            "combo_box.disabled_opacity",
            &mut missing,
        );
        let combo_box_disabled_text_color = require(
            &self.combo_box.disabled_text_color,
            "combo_box.disabled_text_color",
            &mut missing,
        );
        let combo_box_font =
            require_font_opt(&self.combo_box.font, "combo_box.font", dpi, &mut missing);
        let combo_box_border_spec =
            require_border(&self.combo_box.border, "combo_box.border", &mut missing);

        // segmented_control
        let segmented_control_background_color = require(
            &self.segmented_control.background_color,
            "segmented_control.background_color",
            &mut missing,
        );
        let segmented_control_active_background = require(
            &self.segmented_control.active_background,
            "segmented_control.active_background",
            &mut missing,
        );
        let segmented_control_active_text_color = require(
            &self.segmented_control.active_text_color,
            "segmented_control.active_text_color",
            &mut missing,
        );
        let segmented_control_disabled_opacity = require(
            &self.segmented_control.disabled_opacity,
            "segmented_control.disabled_opacity",
            &mut missing,
        );
        let segmented_control_font = require_font_opt(
            &self.segmented_control.font,
            "segmented_control.font",
            dpi,
            &mut missing,
        );
        let segmented_control_border_spec = require_border(
            &self.segmented_control.border,
            "segmented_control.border",
            &mut missing,
        );

        // card (excluded from border_inheritance -- all sub-fields platform-specific or none)
        let card_border_spec = border_all_optional(&self.card.border);

        // expander
        let expander_font =
            require_font_opt(&self.expander.font, "expander.font", dpi, &mut missing);
        let expander_border_spec =
            require_border(&self.expander.border, "expander.border", &mut missing);

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

        // input: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
        check_non_negative(input_min_height, "input.min_height", &mut missing);
        check_range_f32(
            input_disabled_opacity,
            0.0,
            1.0,
            "input.disabled_opacity",
            &mut missing,
        );
        check_positive(input_font.size, "input.font.size", &mut missing);
        check_range_u16(
            input_font.weight,
            100,
            900,
            "input.font.weight",
            &mut missing,
        );

        // checkbox: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
        check_non_negative(
            checkbox_indicator_size,
            "checkbox.indicator_width",
            &mut missing,
        );
        check_non_negative(checkbox_spacing, "checkbox.label_gap", &mut missing);
        check_range_f32(
            checkbox_disabled_opacity,
            0.0,
            1.0,
            "checkbox.disabled_opacity",
            &mut missing,
        );
        check_positive(checkbox_font.size, "checkbox.font.size", &mut missing);
        check_range_u16(
            checkbox_font.weight,
            100,
            900,
            "checkbox.font.weight",
            &mut missing,
        );

        // menu: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(menu_item_height, "menu.row_height", &mut missing);
        check_non_negative(menu_icon_spacing, "menu.icon_text_gap", &mut missing);
        check_non_negative(menu_icon_size, "menu.icon_size", &mut missing);
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

        // slider: geometry >= 0, opacity 0..=1
        check_non_negative(slider_track_height, "slider.track_height", &mut missing);
        check_non_negative(slider_thumb_size, "slider.thumb_diameter", &mut missing);
        check_non_negative(slider_tick_length, "slider.tick_mark_length", &mut missing);
        check_range_f32(
            slider_disabled_opacity,
            0.0,
            1.0,
            "slider.disabled_opacity",
            &mut missing,
        );

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

        // tab: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(tab_min_width, "tab.min_width", &mut missing);
        check_non_negative(tab_min_height, "tab.min_height", &mut missing);
        check_positive(tab_font.size, "tab.font.size", &mut missing);
        check_range_u16(tab_font.weight, 100, 900, "tab.font.weight", &mut missing);

        // sidebar: font size > 0, font weight 100..=900
        check_positive(sidebar_font.size, "sidebar.font.size", &mut missing);
        check_range_u16(
            sidebar_font.weight,
            100,
            900,
            "sidebar.font.weight",
            &mut missing,
        );

        // toolbar: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(toolbar_height, "toolbar.bar_height", &mut missing);
        check_non_negative(toolbar_item_spacing, "toolbar.item_gap", &mut missing);
        check_non_negative(toolbar_icon_size, "toolbar.icon_size", &mut missing);
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

        // list: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(list_item_height, "list.row_height", &mut missing);
        check_positive(list_item_font.size, "list.item_font.size", &mut missing);
        check_range_u16(
            list_item_font.weight,
            100,
            900,
            "list.item_font.weight",
            &mut missing,
        );
        check_positive(list_header_font.size, "list.header_font.size", &mut missing);
        check_range_u16(
            list_header_font.weight,
            100,
            900,
            "list.header_font.weight",
            &mut missing,
        );

        // popover: font size > 0, font weight 100..=900
        check_positive(popover_font.size, "popover.font.size", &mut missing);
        check_range_u16(
            popover_font.weight,
            100,
            900,
            "popover.font.weight",
            &mut missing,
        );

        // splitter: width >= 0
        check_non_negative(splitter_width, "splitter.divider_width", &mut missing);

        // separator: line_width >= 0
        check_non_negative(separator_line_width, "separator.line_width", &mut missing);

        // switch: geometry >= 0, opacity 0..=1
        check_non_negative(switch_track_width, "switch.track_width", &mut missing);
        check_non_negative(switch_track_height, "switch.track_height", &mut missing);
        check_non_negative(switch_thumb_size, "switch.thumb_diameter", &mut missing);
        check_non_negative(switch_track_radius, "switch.track_radius", &mut missing);
        check_range_f32(
            switch_disabled_opacity,
            0.0,
            1.0,
            "switch.disabled_opacity",
            &mut missing,
        );

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

        // dialog: body_font size > 0, weight 100..=900
        check_positive(dialog_body_font.size, "dialog.body_font.size", &mut missing);
        check_range_u16(
            dialog_body_font.weight,
            100,
            900,
            "dialog.body_font.weight",
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

        // link: font size > 0, font weight 100..=900
        check_positive(link_font.size, "link.font.size", &mut missing);
        check_range_u16(link_font.weight, 100, 900, "link.font.weight", &mut missing);

        // combo_box: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
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
        check_range_f32(
            combo_box_disabled_opacity,
            0.0,
            1.0,
            "combo_box.disabled_opacity",
            &mut missing,
        );
        check_positive(combo_box_font.size, "combo_box.font.size", &mut missing);
        check_range_u16(
            combo_box_font.weight,
            100,
            900,
            "combo_box.font.weight",
            &mut missing,
        );

        // segmented_control: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
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
        check_range_f32(
            segmented_control_disabled_opacity,
            0.0,
            1.0,
            "segmented_control.disabled_opacity",
            &mut missing,
        );
        check_positive(
            segmented_control_font.size,
            "segmented_control.font.size",
            &mut missing,
        );
        check_range_u16(
            segmented_control_font.weight,
            100,
            900,
            "segmented_control.font.weight",
            &mut missing,
        );

        // expander: geometry >= 0, font size > 0, font weight 100..=900
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
        check_positive(expander_font.size, "expander.font.size", &mut missing);
        check_range_u16(
            expander_font.weight,
            100,
            900,
            "expander.font.weight",
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
                font_dpi: defaults_font_dpi,
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
                active_text_color: button_active_text_color,
                disabled_text_color: button_disabled_text_color,
                // soft_option fields (Option in Resolved, no require):
                active_background: self.button.active_background,
                disabled_background: self.button.disabled_background,
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
                disabled_text_color: input_disabled_text_color,
                // soft_option fields (Option in Resolved, no require):
                hover_border_color: self.input.hover_border_color,
                focus_border_color: self.input.focus_border_color,
                disabled_background: self.input.disabled_background,
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
                disabled_text_color: checkbox_disabled_text_color,
                // soft_option fields (Option in Resolved, no require):
                hover_background: self.checkbox.hover_background,
                disabled_background: self.checkbox.disabled_background,
                unchecked_background: self.checkbox.unchecked_background,
                unchecked_border_color: self.checkbox.unchecked_border_color,
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
                // soft_option field (Option in Resolved, no require):
                thumb_active_color: self.scrollbar.thumb_active_color,
            },
            slider: crate::model::widgets::ResolvedSliderTheme {
                fill_color: slider_fill,
                track_color: slider_track,
                thumb_color: slider_thumb,
                track_height: slider_track_height,
                thumb_diameter: slider_thumb_size,
                tick_mark_length: slider_tick_length,
                disabled_opacity: slider_disabled_opacity,
                // soft_option fields (Option in Resolved, no require):
                thumb_hover_color: self.slider.thumb_hover_color,
                disabled_fill_color: self.slider.disabled_fill_color,
                disabled_track_color: self.slider.disabled_track_color,
                disabled_thumb_color: self.slider.disabled_thumb_color,
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
                hover_text_color: tab_hover_text_color,
                hover_background: self.tab.hover_background,
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
                hover_text_color: list_hover_text_color,
                disabled_text_color: list_disabled_text_color,
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
                hover_color: splitter_hover_color,
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
                hover_checked_background: self.switch.hover_checked_background,
                hover_unchecked_background: self.switch.hover_unchecked_background,
                disabled_checked_background: self.switch.disabled_checked_background,
                disabled_unchecked_background: self.switch.disabled_unchecked_background,
                disabled_thumb_color: self.switch.disabled_thumb_color,
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
                disabled_text_color: combo_box_disabled_text_color,
                hover_background: self.combo_box.hover_background,
                disabled_background: self.combo_box.disabled_background,
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
                hover_background: self.segmented_control.hover_background,
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
                hover_background: self.expander.hover_background,
                arrow_color: self.expander.arrow_color,
                font: expander_font,
                border: expander_border_spec,
            },
            link: crate::model::widgets::ResolvedLinkTheme {
                visited_text_color: link_visited,
                underline_enabled: link_underline,
                background_color: link_background,
                hover_background: link_hover_bg,
                hover_text_color: link_hover_text_color,
                active_text_color: link_active_text_color,
                disabled_text_color: link_disabled_text_color,
                font: link_font,
            },
            icon_set,
            icon_theme,
        })
    }
}
