// Font specification and text scale types

use crate::Rgba;
use serde::{Deserialize, Serialize};

/// Font style: upright, italic, or oblique.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    /// Normal upright text.
    #[default]
    Normal,
    /// Italic text (true italic glyph).
    Italic,
    /// Oblique text (slanted upright glyph).
    Oblique,
}

/// A font size with an explicit unit.
///
/// In TOML presets, this appears as either `size_pt` (typographic points)
/// or `size_px` (logical pixels). Serde mapping is handled by the parent
/// struct (`FontSpec`, `TextScaleEntry`) — `FontSize` itself has no
/// `Serialize`/`Deserialize` impl.
///
/// During validation, all `FontSize` values are converted to logical pixels
/// via `FontSize::to_px(dpi)`, producing a plain `f32` for the resolved model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontSize {
    /// Typographic points (1/72 inch). Used by platform presets where the OS
    /// reports font sizes in points (KDE, GNOME, Windows).
    /// Converted to px during validation: `px = pt * dpi / 72`.
    Pt(f32),
    /// Logical pixels. Used by community/non-platform presets where font sizes
    /// are hand-authored in pixels.
    Px(f32),
}

impl FontSize {
    /// Convert to logical pixels.
    ///
    /// - `Pt(v)` -> `v * dpi / 72.0`
    /// - `Px(v)` -> `v` (dpi ignored)
    pub fn to_px(self, dpi: f32) -> f32 {
        match self {
            Self::Pt(v) => v * dpi / 72.0,
            Self::Px(v) => v,
        }
    }

    /// Return the raw numeric value regardless of unit.
    /// Used during inheritance to compute derived values (e.g. line_height)
    /// before unit conversion.
    pub fn raw(self) -> f32 {
        match self {
            Self::Pt(v) | Self::Px(v) => v,
        }
    }

    /// True when the value is in typographic points.
    pub fn is_pt(self) -> bool {
        matches!(self, Self::Pt(_))
    }
}

impl Default for FontSize {
    fn default() -> Self {
        Self::Px(0.0)
    }
}

/// Font specification: family name, size, and weight.
///
/// All fields are optional to support partial overlays — a FontSpec with
/// only `size` set will only override the size when merged.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FontSpecRaw", into = "FontSpecRaw")]
pub struct FontSpec {
    /// Font family name (e.g., "Inter", "Noto Sans").
    pub family: Option<String>,
    /// Font size with explicit unit (points or pixels).
    ///
    /// In TOML, set as `size_pt` (typographic points) or `size_px` (logical
    /// pixels). Converted to `f32` logical pixels during validation via
    /// `FontSize::to_px(dpi)`.
    pub size: Option<FontSize>,
    /// CSS font weight (100–900).
    pub weight: Option<u16>,
    /// Font style (normal, italic, oblique).
    pub style: Option<FontStyle>,
    /// Font color.
    pub color: Option<Rgba>,
}

impl FontSpec {
    /// All serialized field names for FontSpec, for TOML linting.
    pub const FIELD_NAMES: &[&str] = &["family", "size_pt", "size_px", "weight", "style", "color"];
}

/// Serde proxy for FontSpec. Maps `FontSize` to two mutually-exclusive keys.
#[serde_with::skip_serializing_none]
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct FontSpecRaw {
    family: Option<String>,
    size_pt: Option<f32>,
    size_px: Option<f32>,
    weight: Option<u16>,
    style: Option<FontStyle>,
    color: Option<Rgba>,
}

impl TryFrom<FontSpecRaw> for FontSpec {
    type Error = String;
    fn try_from(raw: FontSpecRaw) -> Result<Self, Self::Error> {
        let size = match (raw.size_pt, raw.size_px) {
            (Some(v), None) => Some(FontSize::Pt(v)),
            (None, Some(v)) => Some(FontSize::Px(v)),
            (None, None) => None,
            (Some(_), Some(_)) => {
                return Err("font: set `size_pt` or `size_px`, not both".into())
            }
        };
        Ok(FontSpec {
            family: raw.family,
            size,
            weight: raw.weight,
            style: raw.style,
            color: raw.color,
        })
    }
}

impl From<FontSpec> for FontSpecRaw {
    fn from(fs: FontSpec) -> Self {
        let (size_pt, size_px) = match fs.size {
            Some(FontSize::Pt(v)) => (Some(v), None),
            Some(FontSize::Px(v)) => (None, Some(v)),
            None => (None, None),
        };
        FontSpecRaw {
            family: fs.family,
            size_pt,
            size_px,
            weight: fs.weight,
            style: fs.style,
            color: fs.color,
        }
    }
}

impl_merge!(FontSpec {
    option { family, size, weight, style, color }
});

/// A resolved (non-optional) font specification produced after theme resolution.
///
/// Unlike [`FontSpec`], all fields are required (non-optional)
/// because resolution has already filled in all defaults.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResolvedFontSpec {
    /// Font family name.
    pub family: String,
    /// Font size in logical pixels. Converted from platform points during
    /// resolution if `font_dpi` was set on the source `ThemeDefaults`.
    pub size: f32,
    /// CSS font weight (100–900).
    pub weight: u16,
    /// Font style (normal, italic, oblique).
    pub style: FontStyle,
    /// Font color.
    pub color: Rgba,
}

/// A single entry in a text scale: size, weight, and line height.
///
/// Used to define typographic roles (caption, heading, etc.) with
/// consistent sizing and spacing.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TextScaleEntryRaw", into = "TextScaleEntryRaw")]
pub struct TextScaleEntry {
    /// Font size with explicit unit (points or pixels).
    ///
    /// Same semantics as `FontSpec.size` -- in TOML, set as `size_pt` or
    /// `size_px`. Converted to `f32` logical pixels during validation.
    pub size: Option<FontSize>,
    /// CSS font weight (100–900).
    pub weight: Option<u16>,
    /// Line height in the same unit as the sibling `size`. When `None`,
    /// `resolve()` computes it as `defaults.line_height * size.raw()`.
    /// Converted alongside `size` during validation when the unit is points.
    pub line_height: Option<f32>,
}

impl TextScaleEntry {
    /// All serialized field names for TOML linting.
    pub const FIELD_NAMES: &[&str] = &["size_pt", "size_px", "weight", "line_height"];
}

/// Serde proxy for TextScaleEntry. Maps `FontSize` to two mutually-exclusive keys.
#[serde_with::skip_serializing_none]
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct TextScaleEntryRaw {
    size_pt: Option<f32>,
    size_px: Option<f32>,
    weight: Option<u16>,
    line_height: Option<f32>,
}

impl TryFrom<TextScaleEntryRaw> for TextScaleEntry {
    type Error = String;
    fn try_from(raw: TextScaleEntryRaw) -> Result<Self, Self::Error> {
        let size = match (raw.size_pt, raw.size_px) {
            (Some(v), None) => Some(FontSize::Pt(v)),
            (None, Some(v)) => Some(FontSize::Px(v)),
            (None, None) => None,
            (Some(_), Some(_)) => {
                return Err("text_scale: set `size_pt` or `size_px`, not both".into())
            }
        };
        Ok(TextScaleEntry {
            size,
            weight: raw.weight,
            line_height: raw.line_height,
        })
    }
}

impl From<TextScaleEntry> for TextScaleEntryRaw {
    fn from(e: TextScaleEntry) -> Self {
        let (size_pt, size_px) = match e.size {
            Some(FontSize::Pt(v)) => (Some(v), None),
            Some(FontSize::Px(v)) => (None, Some(v)),
            None => (None, None),
        };
        TextScaleEntryRaw {
            size_pt,
            size_px,
            weight: e.weight,
            line_height: e.line_height,
        }
    }
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

impl TextScale {
    /// All serialized field names for TOML linting (issue 3b).
    pub const FIELD_NAMES: &[&str] = &["caption", "section_heading", "dialog_title", "display"];
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        };
        let overlay = FontSpec {
            family: Some("Inter".into()),
            size: None,
            weight: None,
            ..Default::default()
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
            ..Default::default()
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

    // === FontStyle tests ===

    #[test]
    fn font_style_default_is_normal() {
        assert_eq!(FontStyle::default(), FontStyle::Normal);
    }

    #[test]
    fn font_style_serde_round_trip() {
        // TOML cannot serialize a bare enum as a top-level value; use a wrapper struct.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        struct Wrapper {
            style: FontStyle,
        }

        for (variant, expected_str) in [
            (FontStyle::Normal, "normal"),
            (FontStyle::Italic, "italic"),
            (FontStyle::Oblique, "oblique"),
        ] {
            let original = Wrapper { style: variant };
            let serialized = toml::to_string(&original).unwrap();
            assert!(serialized.contains(expected_str), "got: {serialized}");
            let deserialized: Wrapper = toml::from_str(&serialized).unwrap();
            assert_eq!(deserialized, original);
        }
    }

    #[test]
    fn font_spec_with_style_and_color_round_trip() {
        let fs = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
            style: Some(FontStyle::Italic),
            color: Some(crate::Rgba::rgb(255, 0, 0)),
        };
        let toml_str = toml::to_string(&fs).unwrap();
        let deserialized: FontSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, fs);
    }

    #[test]
    fn font_spec_style_none_preserved() {
        let fs = FontSpec {
            family: Some("Inter".into()),
            style: None,
            ..Default::default()
        };
        let toml_str = toml::to_string(&fs).unwrap();
        let deserialized: FontSpec = toml::from_str(&toml_str).unwrap();
        assert!(deserialized.style.is_none());
    }

    #[test]
    fn font_spec_merge_includes_style_and_color() {
        let mut base = FontSpec {
            family: Some("Noto Sans".into()),
            style: Some(FontStyle::Normal),
            color: Some(crate::Rgba::rgb(0, 0, 0)),
            ..Default::default()
        };
        let overlay = FontSpec {
            style: Some(FontStyle::Italic),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.style, Some(FontStyle::Italic)); // overlay wins
        assert_eq!(base.color, Some(crate::Rgba::rgb(0, 0, 0))); // base preserved
        assert_eq!(base.family.as_deref(), Some("Noto Sans")); // base preserved
    }
}
