// Helper functions, range-check utilities, and ValidateNested trait for validate().
//
// Extracted from validate.rs to keep the orchestration module focused on
// defaults extraction, per-widget dispatch, and construction.

use crate::model::border::{BorderSpec, ResolvedBorderSpec};
use crate::model::{FontSpec, ResolvedFontSpec};

/// Standard screen DPI (96 dots per inch). Used as the font_dpi fallback
/// when no DPI was set on the unresolved variant (e.g. community presets
/// loaded standalone without OS reader). This matches the CSS/Web reference
/// pixel and the Windows default. Validation uses this to convert
/// `FontSize::Pt` values to logical pixels via `FontSize::to_px(dpi)`.
pub(crate) const DEFAULT_FONT_DPI: f32 = 96.0;

// --- validate() helpers ---

/// Extract a required field, recording the path if missing.
///
/// Returns the value if present, or `T::default()` as a placeholder if missing.
/// The placeholder is never used: `validate()` returns `Err` before constructing
/// `ResolvedTheme` when any field was recorded as missing.
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
pub(crate) fn require_font(
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
pub(crate) fn require_font_opt(
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
pub(crate) fn require_text_scale_entry(
    entry: &Option<crate::model::TextScaleEntry>,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> crate::model::resolved::ResolvedTextScaleEntry {
    match entry {
        None => {
            missing.push(prefix.to_string());
            crate::model::resolved::ResolvedTextScaleEntry::default()
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
            crate::model::resolved::ResolvedTextScaleEntry {
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
pub(crate) fn require_border(
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
// These push `RangeViolation` structs to the `errors` vec so that range
// violations are reported as structured data, separate from missing fields.

/// Check that an `f32` value is finite and non-negative (>= 0.0).
pub(crate) fn check_non_negative(
    value: f32,
    path: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if !value.is_finite() || value < 0.0 {
        errors.push(crate::error::RangeViolation {
            path: path.to_string(),
            value: value as f64,
            min: Some(0.0),
            max: None,
        });
    }
}

/// Check that an `f32` value is finite and strictly positive (> 0.0).
pub(crate) fn check_positive(
    value: f32,
    path: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if !value.is_finite() || value <= 0.0 {
        errors.push(crate::error::RangeViolation {
            path: path.to_string(),
            value: value as f64,
            min: Some(f64::MIN_POSITIVE),
            max: None,
        });
    }
}

/// Check that an `f32` value is finite and falls within an inclusive range.
pub(crate) fn check_range_f32(
    value: f32,
    min: f32,
    max: f32,
    path: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if !value.is_finite() || value < min || value > max {
        errors.push(crate::error::RangeViolation {
            path: path.to_string(),
            value: value as f64,
            min: Some(min as f64),
            max: Some(max as f64),
        });
    }
}

/// Check that a `u16` value falls within an inclusive range.
pub(crate) fn check_range_u16(
    value: u16,
    min: u16,
    max: u16,
    path: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if value < min || value > max {
        errors.push(crate::error::RangeViolation {
            path: path.to_string(),
            value: value as f64,
            min: Some(min as f64),
            max: Some(max as f64),
        });
    }
}

/// Check that a min value does not exceed its corresponding max value.
pub(crate) fn check_min_max(
    min_val: f32,
    max_val: f32,
    min_name: &str,
    _max_name: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if min_val > max_val {
        errors.push(crate::error::RangeViolation {
            path: min_name.to_string(),
            value: min_val as f64,
            min: None,
            max: Some(max_val as f64),
        });
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

/// Validate defaults and text_scale range checks.
///
/// This centralizes all the range-check calls for the global defaults
/// and the text_scale entries that were previously inline in validate().
pub(crate) fn check_defaults_ranges(
    defaults: &crate::model::resolved::ResolvedDefaults,
    text_scale: &crate::model::resolved::ResolvedTextScale,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    // Fonts: size > 0, weight 100..=900
    check_positive(defaults.font.size, "defaults.font.size", errors);
    check_range_u16(
        defaults.font.weight,
        100,
        900,
        "defaults.font.weight",
        errors,
    );
    check_positive(defaults.mono_font.size, "defaults.mono_font.size", errors);
    check_range_u16(
        defaults.mono_font.weight,
        100,
        900,
        "defaults.mono_font.weight",
        errors,
    );

    // defaults: line_height > 0
    check_positive(defaults.line_height, "defaults.line_height", errors);

    // defaults: radius, geometry >= 0
    check_non_negative(
        defaults.border.corner_radius,
        "defaults.border.corner_radius",
        errors,
    );
    check_non_negative(
        defaults.border.corner_radius_lg,
        "defaults.border.corner_radius_lg",
        errors,
    );
    check_non_negative(
        defaults.border.line_width,
        "defaults.border.line_width",
        errors,
    );
    check_non_negative(
        defaults.focus_ring_width,
        "defaults.focus_ring_width",
        errors,
    );
    // Note: focus_ring_offset is intentionally NOT range-checked -- negative values
    // mean an inset focus ring (e.g., adwaita uses -2.0, macOS uses -1.0).

    // defaults: opacity 0..=1
    check_range_f32(
        defaults.disabled_opacity,
        0.0,
        1.0,
        "defaults.disabled_opacity",
        errors,
    );
    check_range_f32(
        defaults.border.opacity,
        0.0,
        1.0,
        "defaults.border.opacity",
        errors,
    );

    // defaults: border padding >= 0
    check_non_negative(
        defaults.border.padding_horizontal,
        "defaults.border.padding_horizontal",
        errors,
    );
    check_non_negative(
        defaults.border.padding_vertical,
        "defaults.border.padding_vertical",
        errors,
    );

    // defaults: icon sizes >= 0
    check_non_negative(
        defaults.icon_sizes.toolbar,
        "defaults.icon_sizes.toolbar",
        errors,
    );
    check_non_negative(
        defaults.icon_sizes.small,
        "defaults.icon_sizes.small",
        errors,
    );
    check_non_negative(
        defaults.icon_sizes.large,
        "defaults.icon_sizes.large",
        errors,
    );
    check_non_negative(
        defaults.icon_sizes.dialog,
        "defaults.icon_sizes.dialog",
        errors,
    );
    check_non_negative(
        defaults.icon_sizes.panel,
        "defaults.icon_sizes.panel",
        errors,
    );

    // text_scale: entry sizes > 0, line_height > 0
    check_positive(text_scale.caption.size, "text_scale.caption.size", errors);
    check_positive(
        text_scale.caption.line_height,
        "text_scale.caption.line_height",
        errors,
    );
    check_range_u16(
        text_scale.caption.weight,
        100,
        900,
        "text_scale.caption.weight",
        errors,
    );
    check_positive(
        text_scale.section_heading.size,
        "text_scale.section_heading.size",
        errors,
    );
    check_positive(
        text_scale.section_heading.line_height,
        "text_scale.section_heading.line_height",
        errors,
    );
    check_range_u16(
        text_scale.section_heading.weight,
        100,
        900,
        "text_scale.section_heading.weight",
        errors,
    );
    check_positive(
        text_scale.dialog_title.size,
        "text_scale.dialog_title.size",
        errors,
    );
    check_positive(
        text_scale.dialog_title.line_height,
        "text_scale.dialog_title.line_height",
        errors,
    );
    check_range_u16(
        text_scale.dialog_title.weight,
        100,
        900,
        "text_scale.dialog_title.weight",
        errors,
    );
    check_positive(text_scale.display.size, "text_scale.display.size", errors);
    check_positive(
        text_scale.display.line_height,
        "text_scale.display.line_height",
        errors,
    );
    check_range_u16(
        text_scale.display.weight,
        100,
        900,
        "text_scale.display.weight",
        errors,
    );
}
