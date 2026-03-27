// ThemeDefaults: global properties shared across widgets

use crate::Rgba;
use crate::model::{FontSpec, IconSizes};
use crate::model::spacing::ThemeSpacing;
use serde::{Deserialize, Serialize};

/// Global theme defaults shared across all widgets.
///
/// All `Option<T>` fields default to `None`. Plain nested structs
/// (`font`, `mono_font`, `spacing`, `icon_sizes`) default to their
/// all-None state and are suppressed in serialization when empty.
///
/// When resolving a widget's properties, `None` on the widget struct
/// means "inherit from `ThemeDefaults`".
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
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
    pub background: Option<Rgba>,
    /// Default text color.
    pub foreground: Option<Rgba>,
    /// Accent/brand color for interactive elements.
    pub accent: Option<Rgba>,
    /// Text color used on accent-colored backgrounds.
    pub accent_foreground: Option<Rgba>,
    /// Elevated surface color (cards, dialogs, popovers).
    pub surface: Option<Rgba>,
    /// Border/divider color.
    pub border: Option<Rgba>,
    /// Secondary/subdued text color.
    pub muted: Option<Rgba>,
    /// Drop shadow color (with alpha).
    pub shadow: Option<Rgba>,
    /// Hyperlink text color.
    pub link: Option<Rgba>,
    /// Selection highlight background.
    pub selection: Option<Rgba>,
    /// Text color over selection highlight.
    pub selection_foreground: Option<Rgba>,
    /// Selection background when window is unfocused.
    pub selection_inactive: Option<Rgba>,
    /// Text color for disabled controls.
    pub disabled_foreground: Option<Rgba>,

    // ---- Status colors ----
    /// Danger/error color.
    pub danger: Option<Rgba>,
    /// Text color on danger-colored backgrounds.
    pub danger_foreground: Option<Rgba>,
    /// Warning color.
    pub warning: Option<Rgba>,
    /// Text color on warning-colored backgrounds.
    pub warning_foreground: Option<Rgba>,
    /// Success/confirmation color.
    pub success: Option<Rgba>,
    /// Text color on success-colored backgrounds.
    pub success_foreground: Option<Rgba>,
    /// Informational color.
    pub info: Option<Rgba>,
    /// Text color on info-colored backgrounds.
    pub info_foreground: Option<Rgba>,

    // ---- Global geometry ----
    /// Default corner radius in logical pixels.
    pub radius: Option<f32>,
    /// Large corner radius (dialogs, floating panels).
    pub radius_lg: Option<f32>,
    /// Border/frame width in logical pixels.
    pub frame_width: Option<f32>,
    /// Opacity for disabled controls (0.0–1.0).
    pub disabled_opacity: Option<f32>,
    /// Border alpha multiplier (0.0–1.0).
    pub border_opacity: Option<f32>,
    /// Whether drop shadows are enabled.
    pub shadow_enabled: Option<bool>,

    // ---- Focus ring ----
    /// Focus indicator outline color.
    pub focus_ring_color: Option<Rgba>,
    /// Focus indicator outline width.
    pub focus_ring_width: Option<f32>,
    /// Gap between element edge and focus indicator.
    pub focus_ring_offset: Option<f32>,

    // ---- Spacing scale ----
    /// Logical spacing scale (xxs through xxl).
    #[serde(default, skip_serializing_if = "ThemeSpacing::is_empty")]
    pub spacing: ThemeSpacing,

    // ---- Icon sizes ----
    /// Per-context icon sizes.
    #[serde(default, skip_serializing_if = "IconSizes::is_empty")]
    pub icon_sizes: IconSizes,

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

impl_merge!(ThemeDefaults {
    option {
        line_height,
        background, foreground, accent, accent_foreground,
        surface, border, muted, shadow, link, selection, selection_foreground,
        selection_inactive, disabled_foreground,
        danger, danger_foreground, warning, warning_foreground,
        success, success_foreground, info, info_foreground,
        radius, radius_lg, frame_width, disabled_opacity, border_opacity,
        shadow_enabled, focus_ring_color, focus_ring_width, focus_ring_offset,
        text_scaling_factor, reduce_motion, high_contrast, reduce_transparency
    }
    nested { font, mono_font, spacing, icon_sizes }
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;
    use crate::model::{FontSpec, IconSizes};
    use crate::model::spacing::ThemeSpacing;

    // === default / is_empty ===

    #[test]
    fn default_has_all_none_options() {
        let d = ThemeDefaults::default();
        assert!(d.background.is_none());
        assert!(d.foreground.is_none());
        assert!(d.accent.is_none());
        assert!(d.accent_foreground.is_none());
        assert!(d.surface.is_none());
        assert!(d.border.is_none());
        assert!(d.muted.is_none());
        assert!(d.shadow.is_none());
        assert!(d.link.is_none());
        assert!(d.selection.is_none());
        assert!(d.selection_foreground.is_none());
        assert!(d.selection_inactive.is_none());
        assert!(d.disabled_foreground.is_none());
        assert!(d.danger.is_none());
        assert!(d.danger_foreground.is_none());
        assert!(d.warning.is_none());
        assert!(d.warning_foreground.is_none());
        assert!(d.success.is_none());
        assert!(d.success_foreground.is_none());
        assert!(d.info.is_none());
        assert!(d.info_foreground.is_none());
        assert!(d.radius.is_none());
        assert!(d.radius_lg.is_none());
        assert!(d.frame_width.is_none());
        assert!(d.disabled_opacity.is_none());
        assert!(d.border_opacity.is_none());
        assert!(d.shadow_enabled.is_none());
        assert!(d.focus_ring_color.is_none());
        assert!(d.focus_ring_width.is_none());
        assert!(d.focus_ring_offset.is_none());
        assert!(d.text_scaling_factor.is_none());
        assert!(d.reduce_motion.is_none());
        assert!(d.high_contrast.is_none());
        assert!(d.reduce_transparency.is_none());
        assert!(d.line_height.is_none());
    }

    #[test]
    fn default_nested_structs_are_all_empty() {
        let d = ThemeDefaults::default();
        assert!(d.font.is_empty());
        assert!(d.mono_font.is_empty());
        assert!(d.spacing.is_empty());
        assert!(d.icon_sizes.is_empty());
    }

    #[test]
    fn default_is_empty() {
        assert!(ThemeDefaults::default().is_empty());
    }

    #[test]
    fn not_empty_when_accent_set() {
        let d = ThemeDefaults {
            accent: Some(Rgba::rgb(0, 120, 215)),
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
    fn not_empty_when_spacing_set() {
        let d = ThemeDefaults {
            spacing: ThemeSpacing {
                m: Some(12.0),
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
            accent: Some(Rgba::rgb(100, 100, 100)),
            ..Default::default()
        };
        let overlay = ThemeDefaults {
            accent: Some(Rgba::rgb(0, 120, 215)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.accent, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn merge_none_preserves_base() {
        let mut base = ThemeDefaults {
            accent: Some(Rgba::rgb(0, 120, 215)),
            ..Default::default()
        };
        let overlay = ThemeDefaults::default();
        base.merge(&overlay);
        assert_eq!(base.accent, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn merge_font_family_preserved_when_overlay_family_none() {
        let mut base = ThemeDefaults {
            font: FontSpec {
                family: Some("Noto Sans".into()),
                size: Some(11.0),
                weight: None,
            },
            ..Default::default()
        };
        let overlay = ThemeDefaults {
            font: FontSpec {
                family: None,
                size: None,
                weight: Some(700),
            },
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.font.family.as_deref(), Some("Noto Sans")); // preserved
        assert_eq!(base.font.size, Some(11.0));                     // preserved
        assert_eq!(base.font.weight, Some(700));                    // overlay wins
    }

    #[test]
    fn merge_spacing_nested_merges_recursively() {
        let mut base = ThemeDefaults {
            spacing: ThemeSpacing {
                m: Some(12.0),
                ..Default::default()
            },
            ..Default::default()
        };
        let overlay = ThemeDefaults {
            spacing: ThemeSpacing {
                s: Some(6.0),
                ..Default::default()
            },
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.spacing.m, Some(12.0)); // preserved
        assert_eq!(base.spacing.s, Some(6.0));  // overlay wins
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
        assert_eq!(base.icon_sizes.small, Some(16.0));   // overlay wins
    }

    // === TOML round-trip ===

    #[test]
    fn toml_round_trip_accent_and_font_family() {
        let d = ThemeDefaults {
            accent: Some(Rgba::rgb(0, 120, 215)),
            font: FontSpec {
                family: Some("Inter".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        let toml_str = toml::to_string(&d).unwrap();
        // Font section should appear
        assert!(toml_str.contains("[font]"), "Expected [font] section, got: {toml_str}");
        // accent should appear as hex
        assert!(toml_str.contains("accent"), "Expected accent field, got: {toml_str}");
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
        assert!(!toml_str.contains("[font]"), "Empty font should be suppressed: {toml_str}");
        assert!(!toml_str.contains("[mono_font]"), "Empty mono_font should be suppressed: {toml_str}");
        assert!(!toml_str.contains("[spacing]"), "Empty spacing should be suppressed: {toml_str}");
        assert!(!toml_str.contains("[icon_sizes]"), "Empty icon_sizes should be suppressed: {toml_str}");
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
        assert!(toml_str.contains("[mono_font]"), "Expected [mono_font] section, got: {toml_str}");
        let d2: ThemeDefaults = toml::from_str(&toml_str).unwrap();
        assert_eq!(d, d2);
    }

    #[test]
    fn toml_spacing_sub_table() {
        let d = ThemeDefaults {
            spacing: ThemeSpacing {
                m: Some(12.0),
                l: Some(18.0),
                ..Default::default()
            },
            ..Default::default()
        };
        let toml_str = toml::to_string(&d).unwrap();
        assert!(toml_str.contains("[spacing]"), "Expected [spacing] section, got: {toml_str}");
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
