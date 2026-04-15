// Font specification and text scale types

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use crate::Rgba;
use serde::{Deserialize, Serialize};

/// Global font family intern cache.
///
/// Stores `Arc<str>` values so that repeated calls with the same family name
/// return clones of the same `Arc`, avoiding redundant allocations.
static FONT_FAMILY_CACHE: std::sync::LazyLock<Mutex<HashSet<Arc<str>>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashSet::new()));

/// Intern a font family name, returning a deduplicated `Arc<str>`.
///
/// If the same family name has been interned before, returns a clone of the
/// existing `Arc<str>` (same allocation, bumped reference count). Otherwise,
/// creates a new `Arc<str>` and caches it for future lookups.
///
/// This is useful for connectors that resolve fonts repeatedly -- calling
/// `intern_font_family("Inter")` 100 times allocates only once.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use native_theme::theme::intern_font_family;
///
/// let a = intern_font_family("Inter");
/// let b = intern_font_family("Inter");
/// assert!(Arc::ptr_eq(&a, &b)); // Same allocation
/// ```
///
/// # Panics
///
/// This function does not panic. If the internal mutex is poisoned (which
/// can only happen if a thread panicked while holding it), a fresh `Arc<str>`
/// is returned without caching.
pub fn intern_font_family(family: &str) -> Arc<str> {
    if let Ok(mut cache) = FONT_FAMILY_CACHE.lock() {
        if let Some(existing) = cache.get(family) {
            return Arc::clone(existing);
        }
        let arc: Arc<str> = Arc::from(family);
        cache.insert(Arc::clone(&arc));
        arc
    } else {
        // Mutex poisoned -- degrade gracefully without caching
        Arc::from(family)
    }
}

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
/// via `FontSize::to_logical_px(dpi)`, producing a plain `f32` for the resolved model.
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
    /// - `Pt(v)` -> `v * dpi / 72.0` (DPI matters)
    /// - `Px(v)` -> `v` (already logical pixels, DPI ignored)
    ///
    /// The name `to_logical_px` makes the asymmetry explicit: `Px` values
    /// are already in logical pixels, so the DPI parameter has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::FontSize;
    ///
    /// // Pt branch: DPI affects the result
    /// let pt_size = FontSize::Pt(10.0);
    /// assert_eq!(pt_size.to_logical_px(96.0), 10.0 * 96.0 / 72.0);
    /// assert_eq!(pt_size.to_logical_px(72.0), 10.0); // identity at 72 DPI
    ///
    /// // Px branch: DPI is ignored, value returned unchanged
    /// let px_size = FontSize::Px(14.0);
    /// assert_eq!(px_size.to_logical_px(96.0), 14.0);
    /// assert_eq!(px_size.to_logical_px(144.0), 14.0); // same regardless of DPI
    /// ```
    pub fn to_logical_px(self, dpi: f32) -> f32 {
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

/// Font specification: family name, size, weight, style, and color.
///
/// All fields are optional to support partial overlays -- a FontSpec with
/// only `size` set will only override the size when merged.
///
/// **Default behavior asymmetry:** During validation, `family`, `size`,
/// `weight`, and `color` are required (missing values produce a validation
/// error). `style` silently defaults to [`FontStyle::Normal`] because
/// Normal is the universally-safe default -- no theme ever intends to
/// leave style undefined in a way that would produce incorrect rendering.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FontSpecRaw", into = "FontSpecRaw")]
pub struct FontSpec {
    /// Font family name (e.g., "Inter", "Noto Sans").
    pub family: Option<Arc<str>>,
    /// Font size with explicit unit (points or pixels).
    ///
    /// In TOML, set as `size_pt` (typographic points) or `size_px` (logical
    /// pixels). Converted to `f32` logical pixels during validation via
    /// `FontSize::to_logical_px(dpi)`.
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
    family: Option<Arc<str>>,
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
            (Some(_), Some(_)) => return Err("font: set `size_pt` or `size_px`, not both".into()),
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
    pub family: Arc<str>,
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
    /// Line height with explicit unit. When `None`, `resolve()` computes it
    /// as `defaults.line_height * size.raw()`, preserving the unit of `size`.
    pub line_height: Option<FontSize>,
}

impl TextScaleEntry {
    /// All serialized field names for TOML linting.
    pub const FIELD_NAMES: &[&str] = &[
        "size_pt",
        "size_px",
        "weight",
        "line_height_pt",
        "line_height_px",
    ];
}

/// Serde proxy for TextScaleEntry. Maps `FontSize` to two mutually-exclusive keys.
#[serde_with::skip_serializing_none]
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct TextScaleEntryRaw {
    size_pt: Option<f32>,
    size_px: Option<f32>,
    weight: Option<u16>,
    line_height_pt: Option<f32>,
    line_height_px: Option<f32>,
}

impl TryFrom<TextScaleEntryRaw> for TextScaleEntry {
    type Error = String;
    fn try_from(raw: TextScaleEntryRaw) -> Result<Self, Self::Error> {
        let size = match (raw.size_pt, raw.size_px) {
            (Some(v), None) => Some(FontSize::Pt(v)),
            (None, Some(v)) => Some(FontSize::Px(v)),
            (None, None) => None,
            (Some(_), Some(_)) => {
                return Err("text_scale: set `size_pt` or `size_px`, not both".into());
            }
        };
        let line_height = match (raw.line_height_pt, raw.line_height_px) {
            (Some(v), None) => Some(FontSize::Pt(v)),
            (None, Some(v)) => Some(FontSize::Px(v)),
            (None, None) => None,
            (Some(_), Some(_)) => {
                return Err(
                    "text_scale: set `line_height_pt` or `line_height_px`, not both".into(),
                );
            }
        };
        if let (Some(s), Some(lh)) = (&size, &line_height)
            && s.is_pt() != lh.is_pt()
        {
            return Err(
                "text_scale: size and line_height must use the same unit suffix (_pt or _px)"
                    .into(),
            );
        }
        Ok(TextScaleEntry {
            size,
            weight: raw.weight,
            line_height,
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
        let (line_height_pt, line_height_px) = match e.line_height {
            Some(FontSize::Pt(v)) => (Some(v), None),
            Some(FontSize::Px(v)) => (None, Some(v)),
            None => (None, None),
        };
        TextScaleEntryRaw {
            size_pt,
            size_px,
            weight: e.weight,
            line_height_pt,
            line_height_px,
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
    use super::FontSize;
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
            size: Some(FontSize::Px(14.0)),
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
            size: Some(FontSize::Px(14.0)),
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
            size: Some(FontSize::Px(12.0)),
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
        assert_eq!(base.size, Some(FontSize::Px(12.0)));
    }

    #[test]
    fn font_spec_merge_none_preserves_base() {
        let mut base = FontSpec {
            family: Some("Noto Sans".into()),
            size: Some(FontSize::Px(12.0)),
            weight: Some(400),
            ..Default::default()
        };
        let overlay = FontSpec::default();
        base.merge(&overlay);
        assert_eq!(base.family.as_deref(), Some("Noto Sans"));
        assert_eq!(base.size, Some(FontSize::Px(12.0)));
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
            size: Some(FontSize::Px(12.0)),
            weight: Some(400),
            line_height: Some(FontSize::Px(1.4)),
        };
        let toml_str = toml::to_string(&entry).unwrap();
        let deserialized: TextScaleEntry = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, entry);
    }

    #[test]
    fn text_scale_entry_merge_overlay_wins() {
        let mut base = TextScaleEntry {
            size: Some(FontSize::Px(12.0)),
            weight: Some(400),
            line_height: None,
        };
        let overlay = TextScaleEntry {
            size: None,
            weight: Some(700),
            line_height: Some(FontSize::Px(1.5)),
        };
        base.merge(&overlay);
        assert_eq!(base.size, Some(FontSize::Px(12.0))); // preserved
        assert_eq!(base.weight, Some(700)); // overlay wins
        assert_eq!(base.line_height, Some(FontSize::Px(1.5))); // overlay sets
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
                size: Some(FontSize::Px(11.0)),
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
                size: Some(FontSize::Px(11.0)),
                weight: Some(400),
                line_height: Some(FontSize::Px(1.3)),
            }),
            section_heading: Some(TextScaleEntry {
                size: Some(FontSize::Px(14.0)),
                weight: Some(600),
                line_height: Some(FontSize::Px(1.4)),
            }),
            dialog_title: Some(TextScaleEntry {
                size: Some(FontSize::Px(16.0)),
                weight: Some(700),
                line_height: Some(FontSize::Px(1.2)),
            }),
            display: Some(TextScaleEntry {
                size: Some(FontSize::Px(24.0)),
                weight: Some(300),
                line_height: Some(FontSize::Px(1.1)),
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
                size: Some(FontSize::Px(11.0)),
                weight: Some(400),
                line_height: None,
            }),
            ..Default::default()
        };
        let overlay = TextScale {
            caption: Some(TextScaleEntry {
                size: None,
                weight: Some(600),
                line_height: Some(FontSize::Px(1.3)),
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let cap = base.caption.as_ref().unwrap();
        assert_eq!(cap.size, Some(FontSize::Px(11.0))); // base preserved
        assert_eq!(cap.weight, Some(600)); // overlay wins
        assert_eq!(cap.line_height, Some(FontSize::Px(1.3))); // overlay sets
    }

    #[test]
    fn text_scale_merge_none_plus_some_clones_overlay() {
        let mut base = TextScale::default();
        let overlay = TextScale {
            section_heading: Some(TextScaleEntry {
                size: Some(FontSize::Px(14.0)),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        assert!(base.section_heading.is_some());
        assert_eq!(base.section_heading.unwrap().size, Some(FontSize::Px(14.0)));
    }

    #[test]
    fn text_scale_merge_none_preserves_base_entry() {
        let mut base = TextScale {
            display: Some(TextScaleEntry {
                size: Some(FontSize::Px(24.0)),
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = TextScale::default();
        base.merge(&overlay);
        assert!(base.display.is_some());
        assert_eq!(base.display.unwrap().size, Some(FontSize::Px(24.0)));
    }

    // === Arc<str> sharing test ===

    #[test]
    fn resolved_font_clone_shares_family_arc() {
        let font = ResolvedFontSpec {
            family: Arc::from("Inter"),
            size: 14.0,
            weight: 400,
            style: FontStyle::Normal,
            color: crate::Rgba::rgb(0, 0, 0),
        };
        let cloned = font.clone();
        // str data pointers are identical -- same Arc allocation, not a deep copy
        assert!(std::ptr::eq(
            font.family.as_ref() as *const str,
            cloned.family.as_ref() as *const str,
        ));
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
            size: Some(FontSize::Px(14.0)),
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

    // === FontSize tests ===

    #[test]
    fn pt_to_logical_px_at_96_dpi() {
        assert_eq!(FontSize::Pt(10.0).to_logical_px(96.0), 10.0 * 96.0 / 72.0);
    }

    #[test]
    fn px_ignores_dpi() {
        assert_eq!(FontSize::Px(14.0).to_logical_px(96.0), 14.0);
        assert_eq!(FontSize::Px(14.0).to_logical_px(144.0), 14.0);
    }

    #[test]
    fn pt_to_logical_px_at_72_dpi_is_identity() {
        assert_eq!(FontSize::Pt(10.0).to_logical_px(72.0), 10.0);
    }

    #[test]
    fn raw_extracts_value() {
        assert_eq!(FontSize::Pt(10.0).raw(), 10.0);
        assert_eq!(FontSize::Px(14.0).raw(), 14.0);
    }

    #[test]
    fn font_size_default_is_px_zero() {
        assert_eq!(FontSize::default(), FontSize::Px(0.0));
    }

    // === Serde round-trip tests ===

    #[test]
    fn fontspec_toml_round_trip_size_pt() {
        let fs = FontSpec {
            family: Some("Inter".into()),
            size: Some(FontSize::Pt(10.0)),
            weight: Some(400),
            ..Default::default()
        };
        let toml_str = toml::to_string(&fs).expect("serialize");
        assert!(
            toml_str.contains("size_pt"),
            "should contain size_pt: {toml_str}"
        );
        assert!(
            !toml_str.contains("size_px"),
            "should not contain size_px: {toml_str}"
        );
        let deserialized: FontSpec = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(deserialized, fs);
    }

    #[test]
    fn fontspec_toml_round_trip_size_px() {
        let fs = FontSpec {
            size: Some(FontSize::Px(14.0)),
            ..Default::default()
        };
        let toml_str = toml::to_string(&fs).expect("serialize");
        assert!(
            toml_str.contains("size_px"),
            "should contain size_px: {toml_str}"
        );
        assert!(
            !toml_str.contains("size_pt"),
            "should not contain size_pt: {toml_str}"
        );
        let deserialized: FontSpec = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(deserialized, fs);
    }

    #[test]
    fn fontspec_toml_rejects_both_pt_and_px() {
        let toml_str = "size_pt = 10.0\nsize_px = 14.0\n";
        assert!(toml::from_str::<FontSpec>(toml_str).is_err());
    }

    #[test]
    fn fontspec_toml_rejects_bare_size() {
        let toml_str = "size = 10.0\n";
        // With #[serde(default)], the bare `size` key is NOT a recognized field
        // in FontSpecRaw. It deserializes to FontSpec with size=None.
        // The TOML linter (lint_toml) catches `size` as unknown separately.
        let result: FontSpec = toml::from_str(toml_str).expect("deserialize");
        assert!(
            result.size.is_none(),
            "bare 'size' should not set FontSpec.size"
        );
    }

    #[test]
    fn fontspec_toml_no_size_is_valid() {
        let fs: FontSpec = toml::from_str(r#"family = "Inter""#).expect("deserialize");
        assert!(fs.size.is_none());
    }

    #[test]
    fn text_scale_entry_toml_round_trip_size_pt() {
        let entry = TextScaleEntry {
            size: Some(FontSize::Pt(9.0)),
            weight: Some(400),
            line_height: Some(FontSize::Pt(12.6)),
        };
        let toml_str = toml::to_string(&entry).expect("serialize");
        assert!(toml_str.contains("size_pt"));
        let deserialized: TextScaleEntry = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(deserialized, entry);
    }

    #[test]
    fn text_scale_entry_toml_round_trip_size_px() {
        let entry = TextScaleEntry {
            size: Some(FontSize::Px(14.0)),
            weight: Some(400),
            line_height: Some(FontSize::Px(18.0)),
        };
        let toml_str = toml::to_string(&entry).expect("serialize");
        assert!(toml_str.contains("size_px"));
        let deserialized: TextScaleEntry = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(deserialized, entry);
    }
}
