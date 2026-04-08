// ThemeDefaults: global properties shared across widgets

use crate::Rgba;
use crate::model::border::BorderSpec;
use crate::model::{FontSpec, IconSizes};
use serde::{Deserialize, Serialize};

/// Global theme defaults shared across all widgets.
///
/// # Field structure
///
/// This struct uses two patterns for its fields:
///
/// - **`Option<T>` leaf fields** (`accent_color`, `disabled_opacity`, `line_height`, etc.) —
///   `None` means "not set." During merge, an overlay's `Some` value replaces
///   the base wholesale.
///
/// - **Non-Option nested struct fields** (`font`, `mono_font`, `border`,
///   `icon_sizes`) — these support partial field-by-field override during
///   merge. For example, an overlay that sets only `font.size` will inherit
///   the base's `font.family` and `font.weight`. This makes theme merging
///   more flexible: you can fine-tune individual properties without replacing
///   the entire sub-struct.
///
/// This asymmetry is intentional. Checking "is accent_color set?" is
/// `defaults.accent_color.is_some()`, while checking "is font set?" requires
/// inspecting individual fields like `defaults.font.family.is_some()`.
///
/// When resolving a widget's properties, `None` on the widget struct
/// means "inherit from `ThemeDefaults`".
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeDefaults {
    // ---- Base font ----
    /// Primary UI font (family, size, weight).
    #[serde(default, skip_serializing_if = "FontSpec::is_empty")]
    pub font: FontSpec,

    /// Line height multiplier (e.g. 1.4 = 140% of font size).
    pub line_height: Option<f32>,

    /// Monospace font for code/terminal content.
    #[serde(default, skip_serializing_if = "FontSpec::is_empty")]
    pub mono_font: FontSpec,

    // ---- Base colors ----
    /// Main window/surface background color.
    pub background_color: Option<Rgba>,
    /// Default text color.
    pub text_color: Option<Rgba>,
    /// Accent/brand color for interactive elements.
    pub accent_color: Option<Rgba>,
    /// Text color used on accent-colored backgrounds.
    pub accent_text_color: Option<Rgba>,
    /// Elevated surface color (cards, dialogs, popovers).
    pub surface_color: Option<Rgba>,
    /// Secondary/subdued text color.
    pub muted_color: Option<Rgba>,
    /// Drop shadow color (with alpha).
    pub shadow_color: Option<Rgba>,
    /// Hyperlink text color.
    pub link_color: Option<Rgba>,
    /// Selection highlight background.
    pub selection_background: Option<Rgba>,
    /// Text color over selection highlight.
    pub selection_text_color: Option<Rgba>,
    /// Selection background when window is unfocused.
    pub selection_inactive_background: Option<Rgba>,
    /// Text selection background (inline text highlight).
    pub text_selection_background: Option<Rgba>,
    /// Text selection color (inline text highlight).
    pub text_selection_color: Option<Rgba>,
    /// Text color for disabled controls.
    pub disabled_text_color: Option<Rgba>,

    // ---- Status colors ----
    /// Danger/error color.
    pub danger_color: Option<Rgba>,
    /// Text color on danger-colored backgrounds.
    pub danger_text_color: Option<Rgba>,
    /// Warning color.
    pub warning_color: Option<Rgba>,
    /// Text color on warning-colored backgrounds.
    pub warning_text_color: Option<Rgba>,
    /// Success/confirmation color.
    pub success_color: Option<Rgba>,
    /// Text color on success-colored backgrounds.
    pub success_text_color: Option<Rgba>,
    /// Informational color.
    pub info_color: Option<Rgba>,
    /// Text color on info-colored backgrounds.
    pub info_text_color: Option<Rgba>,

    // ---- Global geometry ----
    /// Border sub-struct (color, corner_radius, line_width, etc.).
    #[serde(default, skip_serializing_if = "BorderSpec::is_empty")]
    pub border: BorderSpec,
    /// Opacity for disabled controls (0.0–1.0).
    pub disabled_opacity: Option<f32>,

    // ---- Focus ring ----
    /// Focus indicator outline color.
    pub focus_ring_color: Option<Rgba>,
    /// Focus indicator outline width.
    pub focus_ring_width: Option<f32>,
    /// Gap between element edge and focus indicator.
    pub focus_ring_offset: Option<f32>,

    // ---- Icon sizes ----
    /// Per-context icon sizes.
    #[serde(default, skip_serializing_if = "IconSizes::is_empty")]
    pub icon_sizes: IconSizes,

    // ---- Font DPI ----
    /// Font DPI for pt-to-px conversion. When `Some(dpi)`, font sizes
    /// in this variant are in typographic points and will be converted
    /// during resolution: `px = pt * font_dpi / 72`. When `None`, sizes
    /// are already in logical pixels (no conversion applied).
    ///
    /// OS readers auto-detect this from system settings. Users can
    /// override via TOML overlay or the Rust API.
    pub font_dpi: Option<f32>,

    // ---- Accessibility ----
    /// Text scaling factor (1.0 = no scaling).
    pub text_scaling_factor: Option<f32>,
    /// Whether the user has requested reduced motion.
    pub reduce_motion: Option<bool>,
    /// Whether a high-contrast mode is active.
    pub high_contrast: Option<bool>,
    /// Whether the user has requested reduced transparency.
    pub reduce_transparency: Option<bool>,
}

impl ThemeDefaults {
    /// All serialized field names for ThemeDefaults, for TOML linting.
    pub const FIELD_NAMES: &[&str] = &[
        "font",
        "line_height",
        "mono_font",
        "background_color",
        "text_color",
        "accent_color",
        "accent_text_color",
        "surface_color",
        "muted_color",
        "shadow_color",
        "link_color",
        "selection_background",
        "selection_text_color",
        "selection_inactive_background",
        "text_selection_background",
        "text_selection_color",
        "disabled_text_color",
        "danger_color",
        "danger_text_color",
        "warning_color",
        "warning_text_color",
        "success_color",
        "success_text_color",
        "info_color",
        "info_text_color",
        "border",
        "disabled_opacity",
        "focus_ring_color",
        "focus_ring_width",
        "focus_ring_offset",
        "icon_sizes",
        "font_dpi",
        "text_scaling_factor",
        "reduce_motion",
        "high_contrast",
        "reduce_transparency",
    ];
}

impl_merge!(ThemeDefaults {
    option {
        line_height,
        background_color, text_color, accent_color, accent_text_color,
        surface_color, muted_color, shadow_color, link_color,
        selection_background, selection_text_color,
        selection_inactive_background,
        text_selection_background, text_selection_color,
        disabled_text_color,
        danger_color, danger_text_color, warning_color, warning_text_color,
        success_color, success_text_color, info_color, info_text_color,
        disabled_opacity, focus_ring_color, focus_ring_width, focus_ring_offset,
        font_dpi,
        text_scaling_factor, reduce_motion, high_contrast, reduce_transparency
    }
    nested { font, mono_font, border, icon_sizes }
});

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::Rgba;
    use crate::model::border::BorderSpec;
    use crate::model::{FontSpec, IconSizes};

    // === default / is_empty ===

    #[test]
    fn default_has_all_none_options() {
        let d = ThemeDefaults::default();
        assert!(d.background_color.is_none());
        assert!(d.text_color.is_none());
        assert!(d.accent_color.is_none());
        assert!(d.accent_text_color.is_none());
        assert!(d.surface_color.is_none());
        assert!(d.muted_color.is_none());
        assert!(d.shadow_color.is_none());
        assert!(d.link_color.is_none());
        assert!(d.selection_background.is_none());
        assert!(d.selection_text_color.is_none());
        assert!(d.selection_inactive_background.is_none());
        assert!(d.text_selection_background.is_none());
        assert!(d.text_selection_color.is_none());
        assert!(d.disabled_text_color.is_none());
        assert!(d.danger_color.is_none());
        assert!(d.danger_text_color.is_none());
        assert!(d.warning_color.is_none());
        assert!(d.warning_text_color.is_none());
        assert!(d.success_color.is_none());
        assert!(d.success_text_color.is_none());
        assert!(d.info_color.is_none());
        assert!(d.info_text_color.is_none());
        assert!(d.disabled_opacity.is_none());
        assert!(d.focus_ring_color.is_none());
        assert!(d.focus_ring_width.is_none());
        assert!(d.focus_ring_offset.is_none());
        assert!(d.text_scaling_factor.is_none());
        assert!(d.reduce_motion.is_none());
        assert!(d.high_contrast.is_none());
        assert!(d.reduce_transparency.is_none());
        assert!(d.line_height.is_none());
        assert!(d.font_dpi.is_none());
    }

    #[test]
    fn default_nested_structs_are_all_empty() {
        let d = ThemeDefaults::default();
        assert!(d.font.is_empty());
        assert!(d.mono_font.is_empty());
        assert!(d.border.is_empty());
        assert!(d.icon_sizes.is_empty());
    }

    #[test]
    fn default_is_empty() {
        assert!(ThemeDefaults::default().is_empty());
    }

    #[test]
    fn not_empty_when_accent_color_set() {
        let d = ThemeDefaults {
            accent_color: Some(Rgba::rgb(0, 120, 215)),
            ..Default::default()
        };
        assert!(!d.is_empty());
    }

    #[test]
    fn not_empty_when_font_family_set() {
        let d = ThemeDefaults {
            font: FontSpec {
                family: Some("Inter".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(!d.is_empty());
    }

    #[test]
    fn not_empty_when_border_set() {
        let d = ThemeDefaults {
            border: BorderSpec {
                corner_radius: Some(4.0),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(!d.is_empty());
    }

    // === font and mono_font are plain FontSpec (not Option) ===

    #[test]
    fn font_is_plain_fontspec_not_option() {
        let d = ThemeDefaults::default();
        // If this compiles, font is FontSpec (not Option<FontSpec>)
        let _ = d.font.family;
        let _ = d.font.size;
        let _ = d.font.weight;
    }

    #[test]
    fn mono_font_is_plain_fontspec_not_option() {
        let d = ThemeDefaults::default();
        let _ = d.mono_font.family;
    }

    // === merge ===

    #[test]
    fn merge_option_overlay_wins() {
        let mut base = ThemeDefaults {
            accent_color: Some(Rgba::rgb(100, 100, 100)),
            ..Default::default()
        };
        let overlay = ThemeDefaults {
            accent_color: Some(Rgba::rgb(0, 120, 215)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.accent_color, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn merge_none_preserves_base() {
        let mut base = ThemeDefaults {
            accent_color: Some(Rgba::rgb(0, 120, 215)),
            ..Default::default()
        };
        let overlay = ThemeDefaults::default();
        base.merge(&overlay);
        assert_eq!(base.accent_color, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn merge_font_family_preserved_when_overlay_family_none() {
        let mut base = ThemeDefaults {
            font: FontSpec {
                family: Some("Noto Sans".into()),
                size: Some(11.0),
                weight: None,
                ..Default::default()
            },
            ..Default::default()
        };
        let overlay = ThemeDefaults {
            font: FontSpec {
                family: None,
                size: None,
                weight: Some(700),
                ..Default::default()
            },
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.font.family.as_deref(), Some("Noto Sans")); // preserved
        assert_eq!(base.font.size, Some(11.0)); // preserved
        assert_eq!(base.font.weight, Some(700)); // overlay wins
    }

    #[test]
    fn merge_border_nested_merges_recursively() {
        let mut base = ThemeDefaults {
            border: BorderSpec {
                corner_radius: Some(4.0),
                ..Default::default()
            },
            ..Default::default()
        };
        let overlay = ThemeDefaults {
            border: BorderSpec {
                line_width: Some(1.0),
                ..Default::default()
            },
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.border.corner_radius, Some(4.0)); // preserved
        assert_eq!(base.border.line_width, Some(1.0)); // overlay wins
    }

    #[test]
    fn merge_icon_sizes_nested_merges_recursively() {
        let mut base = ThemeDefaults {
            icon_sizes: IconSizes {
                toolbar: Some(22.0),
                ..Default::default()
            },
            ..Default::default()
        };
        let overlay = ThemeDefaults {
            icon_sizes: IconSizes {
                small: Some(16.0),
                ..Default::default()
            },
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.icon_sizes.toolbar, Some(22.0)); // preserved
        assert_eq!(base.icon_sizes.small, Some(16.0)); // overlay wins
    }

    // === TOML round-trip ===

    #[test]
    fn toml_round_trip_accent_color_and_font_family() {
        let d = ThemeDefaults {
            accent_color: Some(Rgba::rgb(0, 120, 215)),
            font: FontSpec {
                family: Some("Inter".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        let toml_str = toml::to_string(&d).unwrap();
        // Font section should appear
        assert!(
            toml_str.contains("[font]"),
            "Expected [font] section, got: {toml_str}"
        );
        // accent_color should appear as hex
        assert!(
            toml_str.contains("accent_color"),
            "Expected accent_color field, got: {toml_str}"
        );
        // Round-trip
        let d2: ThemeDefaults = toml::from_str(&toml_str).unwrap();
        assert_eq!(d, d2);
    }

    #[test]
    fn toml_empty_sections_suppressed() {
        // An all-default ThemeDefaults should produce minimal or empty TOML
        let d = ThemeDefaults::default();
        let toml_str = toml::to_string(&d).unwrap();
        // No sub-tables should appear for empty nested structs
        assert!(
            !toml_str.contains("[font]"),
            "Empty font should be suppressed: {toml_str}"
        );
        assert!(
            !toml_str.contains("[mono_font]"),
            "Empty mono_font should be suppressed: {toml_str}"
        );
        assert!(
            !toml_str.contains("[border]"),
            "Empty border should be suppressed: {toml_str}"
        );
        assert!(
            !toml_str.contains("[icon_sizes]"),
            "Empty icon_sizes should be suppressed: {toml_str}"
        );
    }

    #[test]
    fn toml_mono_font_sub_table() {
        let d = ThemeDefaults {
            mono_font: FontSpec {
                family: Some("JetBrains Mono".into()),
                size: Some(12.0),
                ..Default::default()
            },
            ..Default::default()
        };
        let toml_str = toml::to_string(&d).unwrap();
        assert!(
            toml_str.contains("[mono_font]"),
            "Expected [mono_font] section, got: {toml_str}"
        );
        let d2: ThemeDefaults = toml::from_str(&toml_str).unwrap();
        assert_eq!(d, d2);
    }

    #[test]
    fn toml_border_sub_table() {
        let d = ThemeDefaults {
            border: BorderSpec {
                corner_radius: Some(4.0),
                line_width: Some(1.0),
                ..Default::default()
            },
            ..Default::default()
        };
        let toml_str = toml::to_string(&d).unwrap();
        assert!(
            toml_str.contains("[border]"),
            "Expected [border] section, got: {toml_str}"
        );
        let d2: ThemeDefaults = toml::from_str(&toml_str).unwrap();
        assert_eq!(d, d2);
    }

    #[test]
    fn accessibility_fields_round_trip() {
        let d = ThemeDefaults {
            text_scaling_factor: Some(1.25),
            reduce_motion: Some(true),
            high_contrast: Some(false),
            reduce_transparency: Some(true),
            ..Default::default()
        };
        let toml_str = toml::to_string(&d).unwrap();
        let d2: ThemeDefaults = toml::from_str(&toml_str).unwrap();
        assert_eq!(d, d2);
    }
}
