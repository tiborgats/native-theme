// Font specification and text scale types

use serde::{Deserialize, Serialize};

/// Font specification: family name, size, and weight.
///
/// All fields are optional to support partial overlays — a FontSpec with
/// only `size` set will only override the size when merged.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FontSpec {
    /// Font family name (e.g., "Inter", "Noto Sans").
    pub family: Option<String>,
    /// Font size in logical pixels.
    pub size: Option<f32>,
    /// CSS font weight (100–900).
    pub weight: Option<u16>,
}

impl_merge!(FontSpec {
    option { family, size, weight }
});

/// A single entry in a text scale: size, weight, and line height.
///
/// Used to define typographic roles (caption, heading, etc.) with
/// consistent sizing and spacing.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TextScaleEntry {
    /// Font size in logical pixels.
    pub size: Option<f32>,
    /// CSS font weight (100–900).
    pub weight: Option<u16>,
    /// Line height in logical pixels. When `None`, `resolve()` computes it
    /// as `defaults.line_height × size`.
    pub line_height: Option<f32>,
}

impl_merge!(TextScaleEntry {
    option { size, weight, line_height }
});

/// A named text scale with four typographic roles.
///
/// Each field is an optional `TextScaleEntry` so that a partial overlay
/// can override only specific roles.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TextScale {
    /// Caption / small label text.
    pub caption: Option<TextScaleEntry>,
    /// Section heading text.
    pub section_heading: Option<TextScaleEntry>,
    /// Dialog title text.
    pub dialog_title: Option<TextScaleEntry>,
    /// Large display / hero text.
    pub display: Option<TextScaleEntry>,
}

impl_merge!(TextScale {
    optional_nested { caption, section_heading, dialog_title, display }
});

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === FontSpec tests ===

    #[test]
    fn font_spec_default_is_empty() {
        assert!(FontSpec::default().is_empty());
    }

    #[test]
    fn font_spec_not_empty_when_family_set() {
        let fs = FontSpec {
            family: Some("Inter".into()),
            ..Default::default()
        };
        assert!(!fs.is_empty());
    }

    #[test]
    fn font_spec_not_empty_when_size_set() {
        let fs = FontSpec {
            size: Some(14.0),
            ..Default::default()
        };
        assert!(!fs.is_empty());
    }

    #[test]
    fn font_spec_not_empty_when_weight_set() {
        let fs = FontSpec {
            weight: Some(700),
            ..Default::default()
        };
        assert!(!fs.is_empty());
    }

    #[test]
    fn font_spec_toml_round_trip() {
        let fs = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        let toml_str = toml::to_string(&fs).unwrap();
        let deserialized: FontSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, fs);
    }

    #[test]
    fn font_spec_toml_round_trip_partial() {
        let fs = FontSpec {
            family: Some("Inter".into()),
            size: None,
            weight: None,
        };
        let toml_str = toml::to_string(&fs).unwrap();
        let deserialized: FontSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, fs);
        assert!(deserialized.size.is_none());
        assert!(deserialized.weight.is_none());
    }

    #[test]
    fn font_spec_merge_overlay_family_replaces_base() {
        let mut base = FontSpec {
            family: Some("Noto Sans".into()),
            size: Some(12.0),
            weight: None,
        };
        let overlay = FontSpec {
            family: Some("Inter".into()),
            size: None,
            weight: None,
        };
        base.merge(&overlay);
        assert_eq!(base.family.as_deref(), Some("Inter"));
        // base size preserved since overlay size is None
        assert_eq!(base.size, Some(12.0));
    }

    #[test]
    fn font_spec_merge_none_preserves_base() {
        let mut base = FontSpec {
            family: Some("Noto Sans".into()),
            size: Some(12.0),
            weight: Some(400),
        };
        let overlay = FontSpec::default();
        base.merge(&overlay);
        assert_eq!(base.family.as_deref(), Some("Noto Sans"));
        assert_eq!(base.size, Some(12.0));
        assert_eq!(base.weight, Some(400));
    }

    // === TextScaleEntry tests ===

    #[test]
    fn text_scale_entry_default_is_empty() {
        assert!(TextScaleEntry::default().is_empty());
    }

    #[test]
    fn text_scale_entry_toml_round_trip() {
        let entry = TextScaleEntry {
            size: Some(12.0),
            weight: Some(400),
            line_height: Some(1.4),
        };
        let toml_str = toml::to_string(&entry).unwrap();
        let deserialized: TextScaleEntry = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, entry);
    }

    #[test]
    fn text_scale_entry_merge_overlay_wins() {
        let mut base = TextScaleEntry {
            size: Some(12.0),
            weight: Some(400),
            line_height: None,
        };
        let overlay = TextScaleEntry {
            size: None,
            weight: Some(700),
            line_height: Some(1.5),
        };
        base.merge(&overlay);
        assert_eq!(base.size, Some(12.0)); // preserved
        assert_eq!(base.weight, Some(700)); // overlay wins
        assert_eq!(base.line_height, Some(1.5)); // overlay sets
    }

    // === TextScale tests ===

    #[test]
    fn text_scale_default_is_empty() {
        assert!(TextScale::default().is_empty());
    }

    #[test]
    fn text_scale_not_empty_when_entry_set() {
        let ts = TextScale {
            caption: Some(TextScaleEntry {
                size: Some(11.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!ts.is_empty());
    }

    #[test]
    fn text_scale_toml_round_trip() {
        let ts = TextScale {
            caption: Some(TextScaleEntry {
                size: Some(11.0),
                weight: Some(400),
                line_height: Some(1.3),
            }),
            section_heading: Some(TextScaleEntry {
                size: Some(14.0),
                weight: Some(600),
                line_height: Some(1.4),
            }),
            dialog_title: Some(TextScaleEntry {
                size: Some(16.0),
                weight: Some(700),
                line_height: Some(1.2),
            }),
            display: Some(TextScaleEntry {
                size: Some(24.0),
                weight: Some(300),
                line_height: Some(1.1),
            }),
        };
        let toml_str = toml::to_string(&ts).unwrap();
        let deserialized: TextScale = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, ts);
    }

    #[test]
    fn text_scale_merge_some_plus_some_merges_inner() {
        let mut base = TextScale {
            caption: Some(TextScaleEntry {
                size: Some(11.0),
                weight: Some(400),
                line_height: None,
            }),
            ..Default::default()
        };
        let overlay = TextScale {
            caption: Some(TextScaleEntry {
                size: None,
                weight: Some(600),
                line_height: Some(1.3),
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let cap = base.caption.as_ref().unwrap();
        assert_eq!(cap.size, Some(11.0)); // base preserved
        assert_eq!(cap.weight, Some(600)); // overlay wins
        assert_eq!(cap.line_height, Some(1.3)); // overlay sets
    }

    #[test]
    fn text_scale_merge_none_plus_some_clones_overlay() {
        let mut base = TextScale::default();
        let overlay = TextScale {
            section_heading: Some(TextScaleEntry {
                size: Some(14.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        assert!(base.section_heading.is_some());
        assert_eq!(base.section_heading.unwrap().size, Some(14.0));
    }

    #[test]
    fn text_scale_merge_none_preserves_base_entry() {
        let mut base = TextScale {
            display: Some(TextScaleEntry {
                size: Some(24.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = TextScale::default();
        base.merge(&overlay);
        assert!(base.display.is_some());
        assert_eq!(base.display.unwrap().size, Some(24.0));
    }
}
