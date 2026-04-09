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
pub(crate) fn require<T: Clone + Default>(
    field: &Option<T>,
    path: &str,
    missing: &mut Vec<String>,
) -> T {
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
            let line_height = e.line_height.map(|fs| fs.to_px(dpi)).unwrap_or_else(|| {
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
pub(crate) fn border_all_optional(border: &Option<BorderSpec>) -> ResolvedBorderSpec {
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
pub(crate) fn require_border_partial(
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

/// Trait for nested types that can be validated from an Option wrapper.
/// Used by `define_widget_pair!` generated `validate_widget()` methods
/// to dispatch to the correct extraction function without knowing the
/// concrete type at macro expansion time.
pub(crate) trait ValidateNested {
    /// The resolved (non-Option) output type.
    type Resolved;

    /// Extract from `Option<Self>`, recording missing fields in `missing`.
    fn validate_nested(
        source: &Option<Self>,
        prefix: &str,
        dpi: f32,
        missing: &mut Vec<String>,
    ) -> Self::Resolved
    where
        Self: Sized;
}

impl ValidateNested for FontSpec {
    type Resolved = ResolvedFontSpec;
    fn validate_nested(
        source: &Option<Self>,
        prefix: &str,
        dpi: f32,
        missing: &mut Vec<String>,
    ) -> ResolvedFontSpec {
        require_font_opt(source, prefix, dpi, missing)
    }
}

impl ValidateNested for BorderSpec {
    type Resolved = ResolvedBorderSpec;
    fn validate_nested(
        source: &Option<Self>,
        prefix: &str,
        _dpi: f32,
        missing: &mut Vec<String>,
    ) -> ResolvedBorderSpec {
        require_border(source, prefix, missing)
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
    // Per-widget extraction is generated by define_widget_pair! validate_widget().
    // Only defaults, text_scale, and range checks remain hand-written.
    #[must_use = "this returns the validation result; handle the Result or propagate with ?"]
    pub fn validate(&self) -> crate::Result<ResolvedThemeVariant> {
        let mut missing = Vec::new();

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

        // --- per-widget extraction (generated by define_widget_pair!) ---

        use crate::model::widgets::*;

        let window =
            ResolvedWindowTheme::validate_widget(&self.window, "window", dpi, &mut missing);
        let button =
            ResolvedButtonTheme::validate_widget(&self.button, "button", dpi, &mut missing);
        let input = ResolvedInputTheme::validate_widget(&self.input, "input", dpi, &mut missing);
        let checkbox =
            ResolvedCheckboxTheme::validate_widget(&self.checkbox, "checkbox", dpi, &mut missing);
        let menu = ResolvedMenuTheme::validate_widget(&self.menu, "menu", dpi, &mut missing);
        let tooltip =
            ResolvedTooltipTheme::validate_widget(&self.tooltip, "tooltip", dpi, &mut missing);
        let scrollbar = ResolvedScrollbarTheme::validate_widget(
            &self.scrollbar,
            "scrollbar",
            dpi,
            &mut missing,
        );
        let slider =
            ResolvedSliderTheme::validate_widget(&self.slider, "slider", dpi, &mut missing);
        let progress_bar = ResolvedProgressBarTheme::validate_widget(
            &self.progress_bar,
            "progress_bar",
            dpi,
            &mut missing,
        );
        let tab = ResolvedTabTheme::validate_widget(&self.tab, "tab", dpi, &mut missing);
        let sidebar =
            ResolvedSidebarTheme::validate_widget(&self.sidebar, "sidebar", dpi, &mut missing);
        let toolbar =
            ResolvedToolbarTheme::validate_widget(&self.toolbar, "toolbar", dpi, &mut missing);
        let status_bar = ResolvedStatusBarTheme::validate_widget(
            &self.status_bar,
            "status_bar",
            dpi,
            &mut missing,
        );
        let list = ResolvedListTheme::validate_widget(&self.list, "list", dpi, &mut missing);
        let popover =
            ResolvedPopoverTheme::validate_widget(&self.popover, "popover", dpi, &mut missing);
        let splitter =
            ResolvedSplitterTheme::validate_widget(&self.splitter, "splitter", dpi, &mut missing);
        let separator = ResolvedSeparatorTheme::validate_widget(
            &self.separator,
            "separator",
            dpi,
            &mut missing,
        );
        let switch =
            ResolvedSwitchTheme::validate_widget(&self.switch, "switch", dpi, &mut missing);
        let dialog =
            ResolvedDialogTheme::validate_widget(&self.dialog, "dialog", dpi, &mut missing);
        let spinner =
            ResolvedSpinnerTheme::validate_widget(&self.spinner, "spinner", dpi, &mut missing);
        let combo_box =
            ResolvedComboBoxTheme::validate_widget(&self.combo_box, "combo_box", dpi, &mut missing);
        let segmented_control = ResolvedSegmentedControlTheme::validate_widget(
            &self.segmented_control,
            "segmented_control",
            dpi,
            &mut missing,
        );
        let card = ResolvedCardTheme::validate_widget(&self.card, "card", dpi, &mut missing);
        let expander =
            ResolvedExpanderTheme::validate_widget(&self.expander, "expander", dpi, &mut missing);
        let link = ResolvedLinkTheme::validate_widget(&self.link, "link", dpi, &mut missing);

        let icon_set = require(&self.icon_set, "icon_set", &mut missing);
        let icon_theme = require(&self.icon_theme, "icon_theme", &mut missing);

        // --- range validation ---
        //
        // Operate on the already-extracted values from validate_widget()/require().
        // If a field was missing, require() returned T::default() as placeholder —
        // range-checking that placeholder is harmless because the missing-field error
        // already captured the real problem.

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
            window.title_bar_font.size,
            "window.title_bar_font.size",
            &mut missing,
        );
        check_range_u16(
            window.title_bar_font.weight,
            100,
            900,
            "window.title_bar_font.weight",
            &mut missing,
        );

        // button: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
        check_non_negative(button.min_width, "button.min_width", &mut missing);
        check_non_negative(button.min_height, "button.min_height", &mut missing);
        check_non_negative(button.icon_text_gap, "button.icon_text_gap", &mut missing);
        check_range_f32(
            button.disabled_opacity,
            0.0,
            1.0,
            "button.disabled_opacity",
            &mut missing,
        );
        check_positive(button.font.size, "button.font.size", &mut missing);
        check_range_u16(
            button.font.weight,
            100,
            900,
            "button.font.weight",
            &mut missing,
        );

        // input: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
        check_non_negative(input.min_height, "input.min_height", &mut missing);
        check_range_f32(
            input.disabled_opacity,
            0.0,
            1.0,
            "input.disabled_opacity",
            &mut missing,
        );
        check_positive(input.font.size, "input.font.size", &mut missing);
        check_range_u16(
            input.font.weight,
            100,
            900,
            "input.font.weight",
            &mut missing,
        );

        // checkbox: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
        check_non_negative(
            checkbox.indicator_width,
            "checkbox.indicator_width",
            &mut missing,
        );
        check_non_negative(checkbox.label_gap, "checkbox.label_gap", &mut missing);
        check_range_f32(
            checkbox.disabled_opacity,
            0.0,
            1.0,
            "checkbox.disabled_opacity",
            &mut missing,
        );
        check_positive(checkbox.font.size, "checkbox.font.size", &mut missing);
        check_range_u16(
            checkbox.font.weight,
            100,
            900,
            "checkbox.font.weight",
            &mut missing,
        );

        // menu: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(menu.row_height, "menu.row_height", &mut missing);
        check_non_negative(menu.icon_text_gap, "menu.icon_text_gap", &mut missing);
        check_non_negative(menu.icon_size, "menu.icon_size", &mut missing);
        check_positive(menu.font.size, "menu.font.size", &mut missing);
        check_range_u16(menu.font.weight, 100, 900, "menu.font.weight", &mut missing);

        // tooltip: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(tooltip.max_width, "tooltip.max_width", &mut missing);
        check_positive(tooltip.font.size, "tooltip.font.size", &mut missing);
        check_range_u16(
            tooltip.font.weight,
            100,
            900,
            "tooltip.font.weight",
            &mut missing,
        );

        // scrollbar: geometry >= 0
        check_non_negative(
            scrollbar.groove_width,
            "scrollbar.groove_width",
            &mut missing,
        );
        check_non_negative(
            scrollbar.min_thumb_length,
            "scrollbar.min_thumb_length",
            &mut missing,
        );
        check_non_negative(scrollbar.thumb_width, "scrollbar.thumb_width", &mut missing);

        // slider: geometry >= 0, opacity 0..=1
        check_non_negative(slider.track_height, "slider.track_height", &mut missing);
        check_non_negative(slider.thumb_diameter, "slider.thumb_diameter", &mut missing);
        check_non_negative(
            slider.tick_mark_length,
            "slider.tick_mark_length",
            &mut missing,
        );
        check_range_f32(
            slider.disabled_opacity,
            0.0,
            1.0,
            "slider.disabled_opacity",
            &mut missing,
        );

        // progress_bar: geometry >= 0
        check_non_negative(
            progress_bar.track_height,
            "progress_bar.track_height",
            &mut missing,
        );
        check_non_negative(
            progress_bar.min_width,
            "progress_bar.min_width",
            &mut missing,
        );

        // tab: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(tab.min_width, "tab.min_width", &mut missing);
        check_non_negative(tab.min_height, "tab.min_height", &mut missing);
        check_positive(tab.font.size, "tab.font.size", &mut missing);
        check_range_u16(tab.font.weight, 100, 900, "tab.font.weight", &mut missing);

        // sidebar: font size > 0, font weight 100..=900
        check_positive(sidebar.font.size, "sidebar.font.size", &mut missing);
        check_range_u16(
            sidebar.font.weight,
            100,
            900,
            "sidebar.font.weight",
            &mut missing,
        );

        // toolbar: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(toolbar.bar_height, "toolbar.bar_height", &mut missing);
        check_non_negative(toolbar.item_gap, "toolbar.item_gap", &mut missing);
        check_non_negative(toolbar.icon_size, "toolbar.icon_size", &mut missing);
        check_positive(toolbar.font.size, "toolbar.font.size", &mut missing);
        check_range_u16(
            toolbar.font.weight,
            100,
            900,
            "toolbar.font.weight",
            &mut missing,
        );

        // status_bar: font size > 0, font weight 100..=900
        check_positive(status_bar.font.size, "status_bar.font.size", &mut missing);
        check_range_u16(
            status_bar.font.weight,
            100,
            900,
            "status_bar.font.weight",
            &mut missing,
        );

        // list: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(list.row_height, "list.row_height", &mut missing);
        check_positive(list.item_font.size, "list.item_font.size", &mut missing);
        check_range_u16(
            list.item_font.weight,
            100,
            900,
            "list.item_font.weight",
            &mut missing,
        );
        check_positive(list.header_font.size, "list.header_font.size", &mut missing);
        check_range_u16(
            list.header_font.weight,
            100,
            900,
            "list.header_font.weight",
            &mut missing,
        );

        // popover: font size > 0, font weight 100..=900
        check_positive(popover.font.size, "popover.font.size", &mut missing);
        check_range_u16(
            popover.font.weight,
            100,
            900,
            "popover.font.weight",
            &mut missing,
        );

        // splitter: width >= 0
        check_non_negative(
            splitter.divider_width,
            "splitter.divider_width",
            &mut missing,
        );

        // separator: line_width >= 0
        check_non_negative(separator.line_width, "separator.line_width", &mut missing);

        // switch: geometry >= 0, opacity 0..=1
        check_non_negative(switch.track_width, "switch.track_width", &mut missing);
        check_non_negative(switch.track_height, "switch.track_height", &mut missing);
        check_non_negative(switch.thumb_diameter, "switch.thumb_diameter", &mut missing);
        check_non_negative(switch.track_radius, "switch.track_radius", &mut missing);
        check_range_f32(
            switch.disabled_opacity,
            0.0,
            1.0,
            "switch.disabled_opacity",
            &mut missing,
        );

        // dialog: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(dialog.min_width, "dialog.min_width", &mut missing);
        check_non_negative(dialog.max_width, "dialog.max_width", &mut missing);
        check_non_negative(dialog.min_height, "dialog.min_height", &mut missing);
        check_non_negative(dialog.max_height, "dialog.max_height", &mut missing);
        check_non_negative(dialog.button_gap, "dialog.button_gap", &mut missing);
        check_non_negative(dialog.icon_size, "dialog.icon_size", &mut missing);
        check_positive(
            dialog.title_font.size,
            "dialog.title_font.size",
            &mut missing,
        );
        check_range_u16(
            dialog.title_font.weight,
            100,
            900,
            "dialog.title_font.weight",
            &mut missing,
        );

        // dialog: body_font size > 0, weight 100..=900
        check_positive(dialog.body_font.size, "dialog.body_font.size", &mut missing);
        check_range_u16(
            dialog.body_font.weight,
            100,
            900,
            "dialog.body_font.weight",
            &mut missing,
        );

        // dialog: cross-field min/max constraints
        check_min_max(
            dialog.min_width,
            dialog.max_width,
            "dialog.min_width",
            "dialog.max_width",
            &mut missing,
        );
        check_min_max(
            dialog.min_height,
            dialog.max_height,
            "dialog.min_height",
            "dialog.max_height",
            &mut missing,
        );

        // spinner: geometry >= 0
        check_non_negative(spinner.diameter, "spinner.diameter", &mut missing);
        check_non_negative(spinner.min_diameter, "spinner.min_diameter", &mut missing);
        check_non_negative(spinner.stroke_width, "spinner.stroke_width", &mut missing);

        // link: font size > 0, font weight 100..=900
        check_positive(link.font.size, "link.font.size", &mut missing);
        check_range_u16(link.font.weight, 100, 900, "link.font.weight", &mut missing);

        // combo_box: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
        check_non_negative(combo_box.min_height, "combo_box.min_height", &mut missing);
        check_non_negative(combo_box.min_width, "combo_box.min_width", &mut missing);
        check_non_negative(
            combo_box.arrow_icon_size,
            "combo_box.arrow_icon_size",
            &mut missing,
        );
        check_non_negative(
            combo_box.arrow_area_width,
            "combo_box.arrow_area_width",
            &mut missing,
        );
        check_range_f32(
            combo_box.disabled_opacity,
            0.0,
            1.0,
            "combo_box.disabled_opacity",
            &mut missing,
        );
        check_positive(combo_box.font.size, "combo_box.font.size", &mut missing);
        check_range_u16(
            combo_box.font.weight,
            100,
            900,
            "combo_box.font.weight",
            &mut missing,
        );

        // segmented_control: geometry >= 0, opacity 0..=1, font size > 0, font weight 100..=900
        check_non_negative(
            segmented_control.segment_height,
            "segmented_control.segment_height",
            &mut missing,
        );
        check_non_negative(
            segmented_control.separator_width,
            "segmented_control.separator_width",
            &mut missing,
        );
        check_range_f32(
            segmented_control.disabled_opacity,
            0.0,
            1.0,
            "segmented_control.disabled_opacity",
            &mut missing,
        );
        check_positive(
            segmented_control.font.size,
            "segmented_control.font.size",
            &mut missing,
        );
        check_range_u16(
            segmented_control.font.weight,
            100,
            900,
            "segmented_control.font.weight",
            &mut missing,
        );

        // expander: geometry >= 0, font size > 0, font weight 100..=900
        check_non_negative(
            expander.header_height,
            "expander.header_height",
            &mut missing,
        );
        check_non_negative(
            expander.arrow_icon_size,
            "expander.arrow_icon_size",
            &mut missing,
        );
        check_positive(expander.font.size, "expander.font.size", &mut missing);
        check_range_u16(
            expander.font.weight,
            100,
            900,
            "expander.font.weight",
            &mut missing,
        );

        // --- check for missing fields and range errors ---

        if !missing.is_empty() {
            return Err(crate::Error::Resolution(ThemeResolutionError {
                missing_fields: missing,
            }));
        }

        // All fields present -- construct ResolvedThemeVariant.

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
            window,
            button,
            input,
            checkbox,
            menu,
            tooltip,
            scrollbar,
            slider,
            progress_bar,
            tab,
            sidebar,
            toolbar,
            status_bar,
            list,
            popover,
            splitter,
            separator,
            switch,
            dialog,
            spinner,
            combo_box,
            segmented_control,
            card,
            expander,
            link,
            icon_set,
            icon_theme,
        })
    }
}
