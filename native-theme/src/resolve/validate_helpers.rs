// Helper functions, range-check utilities, and ValidateNested trait for validate().
//
// Extracted from validate.rs to keep the orchestration module focused on
// defaults extraction, per-widget dispatch, and construction.
//
// Phase 93-01 (G1): `require<T: Clone>` takes an explicit `fallback: T`
// instead of requiring `T: Default`, breaking the bound chain that forced
// `impl Default for Rgba`. See `fn require` below for details.

use crate::Rgba;
use crate::model::border::{ResolvedBorderSpec, WidgetBorderSpec};
use crate::model::font::FontStyle;
use crate::model::{FontSpec, ResolvedFontSpec};
use std::sync::Arc;

/// Standard screen DPI (96 dots per inch). Used as the font_dpi fallback
/// when no DPI was set on the unresolved variant (e.g. community presets
/// loaded standalone without OS reader). This matches the CSS/Web reference
/// pixel and the Windows default. Validation uses this to convert
/// `FontSize::Pt` values to logical pixels via `FontSize::to_logical_px(dpi)`.
pub(crate) const DEFAULT_FONT_DPI: f32 = 96.0;

/// Empty `Arc<str>` placeholder used as the sentinel for missing font-family
/// fields. Never observed by user code: `validate()` returns `Err` whenever
/// any field was recorded as missing.
fn empty_arc_str() -> Arc<str> {
    Arc::<str>::from("")
}

/// Construct the zero-value sentinel for `ResolvedFontSpec`. Fields mirror
/// the pre-G1 `ResolvedFontSpec::default()` output (empty family, 0.0 size,
/// 0 weight, `FontStyle::Normal`, `Rgba::TRANSPARENT`).
fn resolved_font_spec_sentinel() -> ResolvedFontSpec {
    ResolvedFontSpec {
        family: empty_arc_str(),
        size: 0.0,
        weight: 0,
        style: FontStyle::Normal,
        color: Rgba::TRANSPARENT,
    }
}

/// Construct the zero-value sentinel for `ResolvedBorderSpec`. Fields mirror
/// the pre-G1 `ResolvedBorderSpec::default()` output (TRANSPARENT color,
/// 0.0 geometry, false shadow).
fn resolved_border_spec_sentinel() -> ResolvedBorderSpec {
    ResolvedBorderSpec {
        color: Rgba::TRANSPARENT,
        corner_radius: 0.0,
        corner_radius_lg: 0.0,
        line_width: 0.0,
        opacity: 0.0,
        shadow_enabled: false,
        padding_horizontal: 0.0,
        padding_vertical: 0.0,
    }
}

// --- validate() helpers ---

/// Extract a required field, recording the path if missing.
///
/// Returns the value if present, or the caller-supplied `fallback` as a
/// placeholder if missing. The placeholder is never observed: `validate()`
/// returns `Err` before constructing `ResolvedTheme` whenever any field was
/// recorded as missing.
///
/// Callers supply `fallback` as an explicit sentinel (e.g.
/// [`crate::color::Rgba::TRANSPARENT`], `0.0_f32`, `false`). This lets the
/// function stay bound-free (`T: Clone` only) -- in particular, no `Default`
/// bound is required on `T`, so types like [`crate::color::Rgba`] do not
/// need `impl Default`.
pub(crate) fn require<T: Clone>(
    field: &Option<T>,
    path: &str,
    missing: &mut Vec<String>,
    fallback: T,
) -> T {
    match field {
        Some(val) => val.clone(),
        None => {
            missing.push(path.to_string());
            fallback
        }
    }
}

/// Validate a FontSpec that is stored directly (not wrapped in Option).
/// Checks each sub-field individually. Converts `FontSize` to `f32` px via `to_logical_px(dpi)`.
pub(crate) fn require_font(
    font: &FontSpec,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> ResolvedFontSpec {
    let family = require(
        &font.family,
        &format!("{prefix}.family"),
        missing,
        empty_arc_str(),
    );
    let size = font
        .size
        .map(|fs| fs.to_logical_px(dpi))
        .unwrap_or_else(|| {
            missing.push(format!("{prefix}.size"));
            0.0
        });
    let weight_fallback: u16 = 0;
    let weight = require(
        &font.weight,
        &format!("{prefix}.weight"),
        missing,
        weight_fallback,
    );
    let color = require(
        &font.color,
        &format!("{prefix}.color"),
        missing,
        Rgba::TRANSPARENT,
    );
    ResolvedFontSpec {
        family,
        size,
        weight,
        // `style` is inherently optional: `FontStyle::Normal` is the universally-safe
        // default when a theme omits italic/oblique. Unlike `family`, `size`, `weight`,
        // and `color` -- which have no safe universal default -- `style` can always
        // fall back to Normal without producing an incorrect rendering. This asymmetry
        // is intentional (doc 2 D1).
        style: font.style.unwrap_or(FontStyle::Normal),
        color,
    }
}

/// Validate an `Option<FontSpec>` (widget font fields).
/// If None, records the path as missing. Converts `FontSize` to `f32` px via `to_logical_px(dpi)`.
pub(crate) fn require_font_opt(
    font: &Option<FontSpec>,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> ResolvedFontSpec {
    match font {
        None => {
            missing.push(prefix.to_string());
            resolved_font_spec_sentinel()
        }
        Some(f) => {
            let family = require(
                &f.family,
                &format!("{prefix}.family"),
                missing,
                empty_arc_str(),
            );
            let size = f.size.map(|fs| fs.to_logical_px(dpi)).unwrap_or_else(|| {
                missing.push(format!("{prefix}.size"));
                0.0
            });
            let weight_fallback: u16 = 0;
            let weight = require(
                &f.weight,
                &format!("{prefix}.weight"),
                missing,
                weight_fallback,
            );
            let color = require(
                &f.color,
                &format!("{prefix}.color"),
                missing,
                Rgba::TRANSPARENT,
            );
            ResolvedFontSpec {
                family,
                size,
                weight,
                // `style` is inherently optional: `FontStyle::Normal` is the universally-safe
                // default when a theme omits italic/oblique (see require_font for full rationale).
                style: f.style.unwrap_or(FontStyle::Normal),
                color,
            }
        }
    }
}

/// Validate an `Option<TextScaleEntry>`.
/// Converts `FontSize` to `f32` px via `to_logical_px(dpi)`. Also converts `line_height`
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
            crate::model::resolved::ResolvedTextScaleEntry {
                size: 0.0,
                weight: 0,
                line_height: 0.0,
            }
        }
        Some(e) => {
            let size = e.size.map(|fs| fs.to_logical_px(dpi)).unwrap_or_else(|| {
                missing.push(format!("{prefix}.size"));
                0.0
            });
            let line_height = e
                .line_height
                .map(|fs| fs.to_logical_px(dpi))
                .unwrap_or_else(|| {
                    missing.push(format!("{prefix}.line_height"));
                    0.0
                });
            let weight_fallback: u16 = 0;
            let weight = require(
                &e.weight,
                &format!("{prefix}.weight"),
                missing,
                weight_fallback,
            );
            crate::model::resolved::ResolvedTextScaleEntry {
                size,
                weight,
                line_height,
            }
        }
    }
}

/// Which border sub-fields are required vs optional for a given widget.
///
/// Drives `validate_border()` dispatch. Each widget declares its border kind
/// via `#[theme_layer(border_kind = "...")]` on the struct; the proc-macro
/// emits the corresponding `BorderKind` variant in generated `validate_widget()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BorderKind {
    /// All 4 inherited sub-fields required (color, corner_radius, line_width, shadow_enabled).
    /// Used by: window, button, input, checkbox, tooltip, progress_bar, toolbar,
    /// list, popover, dialog, combo_box, segmented_control, expander.
    Full,
    /// Only color + line_width required; corner_radius and shadow_enabled optional.
    /// Used by: sidebar, status_bar.
    Partial,
    /// All sub-fields optional — no validation errors recorded.
    /// Used by: menu, tab, card.
    None,
}

/// Validate an `Option<WidgetBorderSpec>` according to its `BorderKind`.
///
/// - `Full`: requires color, corner_radius, line_width, shadow_enabled.
/// - `Partial`: requires color and line_width only.
/// - `None`: all sub-fields optional, no missing-field errors.
///
/// Padding sub-fields are sizing fields with no inheritance — the preset
/// value is used when present, otherwise zero. `corner_radius_lg` and
/// `opacity` are defaults-only; always 0.0 at widget level.
pub(crate) fn validate_border(
    border: &Option<WidgetBorderSpec>,
    prefix: &str,
    kind: BorderKind,
    missing: &mut Vec<String>,
) -> ResolvedBorderSpec {
    match kind {
        BorderKind::None => match border {
            Option::None => resolved_border_spec_sentinel(),
            Some(b) => ResolvedBorderSpec {
                // `b.color: Option<Rgba>`. Once `impl Default for Rgba` is
                // removed in Task 2, `Option::unwrap_or_default()` can no
                // longer synthesise an Rgba. Pass TRANSPARENT explicitly
                // (same value as the old `Rgba::default()`).
                color: b.color.unwrap_or(Rgba::TRANSPARENT),
                corner_radius: b.corner_radius.unwrap_or_default(),
                corner_radius_lg: 0.0,
                line_width: b.line_width.unwrap_or_default(),
                opacity: 0.0,
                shadow_enabled: b.shadow_enabled.unwrap_or_default(),
                padding_horizontal: b.padding_horizontal.unwrap_or_default(),
                padding_vertical: b.padding_vertical.unwrap_or_default(),
            },
        },
        BorderKind::Full | BorderKind::Partial => match border {
            Option::None => {
                missing.push(prefix.to_string());
                resolved_border_spec_sentinel()
            }
            Some(b) => {
                let color = require(
                    &b.color,
                    &format!("{prefix}.color"),
                    missing,
                    Rgba::TRANSPARENT,
                );
                let zero_width: f32 = 0.0;
                let line_width = require(
                    &b.line_width,
                    &format!("{prefix}.line_width"),
                    missing,
                    zero_width,
                );

                let (corner_radius, shadow_enabled) = if kind == BorderKind::Full {
                    let zero_radius: f32 = 0.0;
                    (
                        require(
                            &b.corner_radius,
                            &format!("{prefix}.corner_radius"),
                            missing,
                            zero_radius,
                        ),
                        require(
                            &b.shadow_enabled,
                            &format!("{prefix}.shadow_enabled"),
                            missing,
                            false,
                        ),
                    )
                } else {
                    (
                        b.corner_radius.unwrap_or_default(),
                        b.shadow_enabled.unwrap_or_default(),
                    )
                };

                ResolvedBorderSpec {
                    color,
                    corner_radius,
                    corner_radius_lg: 0.0,
                    line_width,
                    opacity: 0.0,
                    shadow_enabled,
                    padding_horizontal: b.padding_horizontal.unwrap_or_default(),
                    padding_vertical: b.padding_vertical.unwrap_or_default(),
                }
            }
        },
    }
}

// --- Range-check helpers for validate() ---
//
// These push `RangeViolation` structs to the `errors` vec so that range
// violations are reported as structured data, separate from missing fields.

/// Check that an `f32` value is finite and non-negative (>= 0.0).
///
/// Path string (`"{prefix}.{field}"`) is only formatted in the error branch,
/// so the happy path (value in range) allocates zero strings.
pub(crate) fn check_non_negative(
    value: f32,
    prefix: &str,
    field: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if !value.is_finite() || value < 0.0 {
        errors.push(crate::error::RangeViolation {
            path: format!("{prefix}.{field}"),
            value: value as f64,
            min: Some(0.0),
            max: None,
        });
    }
}

/// Check that an `f32` value is finite and strictly positive (> 0.0).
///
/// Path string (`"{prefix}.{field}"`) is only formatted in the error branch,
/// so the happy path (value in range) allocates zero strings.
pub(crate) fn check_positive(
    value: f32,
    prefix: &str,
    field: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if !value.is_finite() || value <= 0.0 {
        errors.push(crate::error::RangeViolation {
            path: format!("{prefix}.{field}"),
            value: value as f64,
            min: Some(f64::MIN_POSITIVE),
            max: None,
        });
    }
}

/// Check that an `f32` value is finite and falls within an inclusive range.
///
/// Path string (`"{prefix}.{field}"`) is only formatted in the error branch,
/// so the happy path (value in range) allocates zero strings.
pub(crate) fn check_range_f32(
    value: f32,
    min: f32,
    max: f32,
    prefix: &str,
    field: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if !value.is_finite() || value < min || value > max {
        errors.push(crate::error::RangeViolation {
            path: format!("{prefix}.{field}"),
            value: value as f64,
            min: Some(min as f64),
            max: Some(max as f64),
        });
    }
}

/// Check that a `u16` value falls within an inclusive range.
///
/// Path string (`"{prefix}.{field}"`) is only formatted in the error branch,
/// so the happy path (value in range) allocates zero strings.
pub(crate) fn check_range_u16(
    value: u16,
    min: u16,
    max: u16,
    prefix: &str,
    field: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if value < min || value > max {
        errors.push(crate::error::RangeViolation {
            path: format!("{prefix}.{field}"),
            value: value as f64,
            min: Some(min as f64),
            max: Some(max as f64),
        });
    }
}

/// Check that a min value does not exceed its corresponding max value.
///
/// Path string (`"{prefix}.{min_field}"`) is only formatted in the error branch,
/// so the happy path (min <= max) allocates zero strings.
pub(crate) fn check_min_max(
    min_val: f32,
    max_val: f32,
    prefix: &str,
    min_field: &str,
    _max_field: &str,
    errors: &mut Vec<crate::error::RangeViolation>,
) {
    if min_val > max_val {
        errors.push(crate::error::RangeViolation {
            path: format!("{prefix}.{min_field}"),
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

impl ValidateNested for WidgetBorderSpec {
    type Resolved = ResolvedBorderSpec;
    fn validate_nested(
        source: &Option<Self>,
        prefix: &str,
        _dpi: f32,
        missing: &mut Vec<String>,
    ) -> ResolvedBorderSpec {
        validate_border(source, prefix, BorderKind::Full, missing)
    }
}

// --- Declarative macros for defaults/text_scale extraction ---

/// Extract all defaults fields and construct `ResolvedDefaults` in one invocation.
///
/// Handles five extraction patterns:
/// - **font**: non-Option `FontSpec` fields via `require_font()`.
/// - **option_color**: `Option<Rgba>` fields via `require()` with
///   `Rgba::TRANSPARENT` fallback.
/// - **option_f32**: `Option<f32>` fields via `require()` with `0.0_f32`
///   fallback.
/// - **border_required**: `Option<T>` fields nested under `defaults.border`
///   with per-field fallback expressions supplied at the call site
///   (Rgba / f32 / bool).
/// - **icon_sizes**: `Option<f32>` fields nested under `defaults.icon_sizes`
///   via `require()` with `0.0_f32` fallback.
///
/// The split between `option_color` and `option_f32` is intentional: after
/// G1 removes `impl Default for Rgba`, there is no uniform way for the
/// macro to synthesise a per-field fallback without a sealed
/// `Default`-equivalent trait. Encoding the type group at the macro call
/// site keeps the fallback sentinel construction local and explicit.
///
/// Border padding fields (`padding_horizontal`, `padding_vertical`) default to `0.0`
/// and are not extracted via `require()`.
macro_rules! validate_defaults {
    (
        $src:expr, $dpi:expr, $missing:expr;
        font { $($font_field:ident),* $(,)? }
        option_color { $($color_field:ident),* $(,)? }
        option_f32 { $($f32_field:ident),* $(,)? }
        border_required { $($br_field:ident : $br_fallback:expr),* $(,)? }
        icon_sizes { $($is_field:ident),* $(,)? }
    ) => {{
        use $crate::resolve::validate_helpers::{require, require_font};
        // Pattern B: font fields (non-Option FontSpec -> ResolvedFontSpec)
        $(
            let $font_field = require_font(
                &$src.defaults.$font_field,
                concat!("defaults.", stringify!($font_field)),
                $dpi,
                $missing,
            );
        )*
        // Pattern A1: Option<Rgba> fields
        $(
            let $color_field = require(
                &$src.defaults.$color_field,
                concat!("defaults.", stringify!($color_field)),
                $missing,
                $crate::color::Rgba::TRANSPARENT,
            );
        )*
        // Pattern A2: Option<f32> fields
        $(
            let $f32_field = require(
                &$src.defaults.$f32_field,
                concat!("defaults.", stringify!($f32_field)),
                $missing,
                0.0_f32,
            );
        )*
        // Pattern C: border sub-fields (require'd, with explicit per-field fallback)
        $(
            let $br_field = require(
                &$src.defaults.border.$br_field,
                concat!("defaults.border.", stringify!($br_field)),
                $missing,
                $br_fallback,
            );
        )*
        // Pattern D: icon_sizes sub-fields (Option<f32>)
        $(
            let $is_field = require(
                &$src.defaults.icon_sizes.$is_field,
                concat!("defaults.icon_sizes.", stringify!($is_field)),
                $missing,
                0.0_f32,
            );
        )*

        use $crate::model::border::ResolvedBorderSpec;
        use $crate::model::resolved::{ResolvedDefaults, ResolvedIconSizes};
        ResolvedDefaults {
            $($font_field,)*
            $($color_field,)*
            $($f32_field,)*
            border: ResolvedBorderSpec {
                $($br_field,)*
                padding_horizontal: 0.0,
                padding_vertical: 0.0,
            },
            icon_sizes: ResolvedIconSizes {
                $($is_field,)*
            },
        }
    }};
}
pub(crate) use validate_defaults;

/// Extract all text_scale fields and construct `ResolvedTextScale` in one invocation.
///
/// Each field is an `Option<TextScaleEntry>` extracted via `require_text_scale_entry()`.
macro_rules! validate_text_scale {
    ($src:expr, $dpi:expr, $missing:expr; $($field:ident),* $(,)?) => {{
        use $crate::resolve::validate_helpers::require_text_scale_entry;
        $(
            let $field = require_text_scale_entry(
                &$src.text_scale.$field,
                concat!("text_scale.", stringify!($field)),
                $dpi,
                $missing,
            );
        )*
        $crate::model::resolved::ResolvedTextScale { $($field),* }
    }};
}
pub(crate) use validate_text_scale;

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
    check_positive(defaults.font.size, "defaults.font", "size", errors);
    check_range_u16(
        defaults.font.weight,
        100,
        900,
        "defaults.font",
        "weight",
        errors,
    );
    check_positive(
        defaults.mono_font.size,
        "defaults.mono_font",
        "size",
        errors,
    );
    check_range_u16(
        defaults.mono_font.weight,
        100,
        900,
        "defaults.mono_font",
        "weight",
        errors,
    );

    // defaults: line_height > 0
    check_positive(defaults.line_height, "defaults", "line_height", errors);

    // defaults: radius, geometry >= 0
    check_non_negative(
        defaults.border.corner_radius,
        "defaults.border",
        "corner_radius",
        errors,
    );
    check_non_negative(
        defaults.border.corner_radius_lg,
        "defaults.border",
        "corner_radius_lg",
        errors,
    );
    check_non_negative(
        defaults.border.line_width,
        "defaults.border",
        "line_width",
        errors,
    );
    check_non_negative(
        defaults.focus_ring_width,
        "defaults",
        "focus_ring_width",
        errors,
    );
    // Note: focus_ring_offset is intentionally NOT range-checked -- negative values
    // mean an inset focus ring (e.g., adwaita uses -2.0, macOS uses -1.0).

    // defaults: opacity 0..=1
    check_range_f32(
        defaults.disabled_opacity,
        0.0,
        1.0,
        "defaults",
        "disabled_opacity",
        errors,
    );
    check_range_f32(
        defaults.border.opacity,
        0.0,
        1.0,
        "defaults.border",
        "opacity",
        errors,
    );

    // defaults: border padding >= 0
    check_non_negative(
        defaults.border.padding_horizontal,
        "defaults.border",
        "padding_horizontal",
        errors,
    );
    check_non_negative(
        defaults.border.padding_vertical,
        "defaults.border",
        "padding_vertical",
        errors,
    );

    // defaults: icon sizes >= 0
    check_non_negative(
        defaults.icon_sizes.toolbar,
        "defaults.icon_sizes",
        "toolbar",
        errors,
    );
    check_non_negative(
        defaults.icon_sizes.small,
        "defaults.icon_sizes",
        "small",
        errors,
    );
    check_non_negative(
        defaults.icon_sizes.large,
        "defaults.icon_sizes",
        "large",
        errors,
    );
    check_non_negative(
        defaults.icon_sizes.dialog,
        "defaults.icon_sizes",
        "dialog",
        errors,
    );
    check_non_negative(
        defaults.icon_sizes.panel,
        "defaults.icon_sizes",
        "panel",
        errors,
    );

    // text_scale: entry sizes > 0, line_height > 0
    check_positive(
        text_scale.caption.size,
        "text_scale.caption",
        "size",
        errors,
    );
    check_positive(
        text_scale.caption.line_height,
        "text_scale.caption",
        "line_height",
        errors,
    );
    check_range_u16(
        text_scale.caption.weight,
        100,
        900,
        "text_scale.caption",
        "weight",
        errors,
    );
    check_positive(
        text_scale.section_heading.size,
        "text_scale.section_heading",
        "size",
        errors,
    );
    check_positive(
        text_scale.section_heading.line_height,
        "text_scale.section_heading",
        "line_height",
        errors,
    );
    check_range_u16(
        text_scale.section_heading.weight,
        100,
        900,
        "text_scale.section_heading",
        "weight",
        errors,
    );
    check_positive(
        text_scale.dialog_title.size,
        "text_scale.dialog_title",
        "size",
        errors,
    );
    check_positive(
        text_scale.dialog_title.line_height,
        "text_scale.dialog_title",
        "line_height",
        errors,
    );
    check_range_u16(
        text_scale.dialog_title.weight,
        100,
        900,
        "text_scale.dialog_title",
        "weight",
        errors,
    );
    check_positive(
        text_scale.display.size,
        "text_scale.display",
        "size",
        errors,
    );
    check_positive(
        text_scale.display.line_height,
        "text_scale.display",
        "line_height",
        errors,
    );
    check_range_u16(
        text_scale.display.weight,
        100,
        900,
        "text_scale.display",
        "weight",
        errors,
    );
}
