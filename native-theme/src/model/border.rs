// Border specification sub-struct for widget border properties

use crate::Rgba;
use serde::{Deserialize, Serialize};

/// Border specification: color, geometry, and padding.
///
/// All fields are optional to support partial overlays -- a BorderSpec with
/// only `color` set will only override the color when merged.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BorderSpec {
    /// Border color.
    pub color: Option<Rgba>,
    /// Corner radius in logical pixels.
    #[serde(rename = "corner_radius_px")]
    pub corner_radius: Option<f32>,
    /// Large corner radius in logical pixels (defaults only).
    #[serde(rename = "corner_radius_lg_px")]
    pub corner_radius_lg: Option<f32>,
    /// Border stroke width in logical pixels.
    #[serde(rename = "line_width_px")]
    pub line_width: Option<f32>,
    /// Border alpha multiplier 0.0–1.0 (defaults only).
    pub opacity: Option<f32>,
    /// Whether the bordered element has a drop shadow.
    pub shadow_enabled: Option<bool>,
    /// Horizontal padding inside the border in logical pixels.
    #[serde(rename = "padding_horizontal_px")]
    pub padding_horizontal: Option<f32>,
    /// Vertical padding inside the border in logical pixels.
    #[serde(rename = "padding_vertical_px")]
    pub padding_vertical: Option<f32>,
}

impl BorderSpec {
    /// All serialized field names for BorderSpec, for TOML linting.
    pub const FIELD_NAMES: &[&str] = &[
        "color",
        "corner_radius_px",
        "corner_radius_lg_px",
        "line_width_px",
        "opacity",
        "shadow_enabled",
        "padding_horizontal_px",
        "padding_vertical_px",
    ];
}

impl_merge!(BorderSpec {
    option { color, corner_radius, corner_radius_lg, line_width, opacity, shadow_enabled, padding_horizontal, padding_vertical }
});

/// A resolved (non-optional) border specification produced after theme resolution.
///
/// Unlike [`BorderSpec`], all fields are required (non-optional)
/// because resolution has already filled in all defaults.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResolvedBorderSpec {
    /// Border color.
    pub color: Rgba,
    /// Corner radius in logical pixels.
    pub corner_radius: f32,
    /// Large corner radius in logical pixels (defaults only).
    pub corner_radius_lg: f32,
    /// Border stroke width in logical pixels.
    pub line_width: f32,
    /// Border alpha multiplier 0.0–1.0 (defaults only).
    pub opacity: f32,
    /// Whether the bordered element has a drop shadow.
    pub shadow_enabled: bool,
    /// Horizontal padding inside the border in logical pixels.
    pub padding_horizontal: f32,
    /// Vertical padding inside the border in logical pixels.
    pub padding_vertical: f32,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn border_spec_default_is_empty() {
        assert!(BorderSpec::default().is_empty());
    }

    #[test]
    fn border_spec_not_empty_when_color_set() {
        let bs = BorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            ..Default::default()
        };
        assert!(!bs.is_empty());
    }

    #[test]
    fn border_spec_toml_round_trip_full() {
        let bs = BorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            corner_radius: Some(4.0),
            corner_radius_lg: Some(8.0),
            line_width: Some(1.0),
            opacity: Some(0.15),
            shadow_enabled: Some(true),
            padding_horizontal: Some(8.0),
            padding_vertical: Some(6.0),
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: BorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
    }

    #[test]
    fn border_spec_toml_round_trip_partial() {
        let bs = BorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(8.0),
            corner_radius_lg: None,
            line_width: None,
            opacity: None,
            shadow_enabled: None,
            padding_horizontal: None,
            padding_vertical: None,
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: BorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
        assert!(deserialized.corner_radius_lg.is_none());
        assert!(deserialized.line_width.is_none());
        assert!(deserialized.opacity.is_none());
        assert!(deserialized.shadow_enabled.is_none());
        assert!(deserialized.padding_horizontal.is_none());
        assert!(deserialized.padding_vertical.is_none());
    }

    #[test]
    fn border_spec_merge_overlay_wins() {
        let mut base = BorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(4.0),
            ..Default::default()
        };
        let overlay = BorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.color, Some(Rgba::rgb(200, 200, 200)));
        // base corner_radius preserved since overlay corner_radius is None
        assert_eq!(base.corner_radius, Some(4.0));
    }

    #[test]
    fn resolved_border_spec_default() {
        let rbs = ResolvedBorderSpec::default();
        assert_eq!(rbs.padding_horizontal, 0.0);
        assert_eq!(rbs.padding_vertical, 0.0);
        assert_eq!(rbs.corner_radius, 0.0);
        assert_eq!(rbs.corner_radius_lg, 0.0);
        assert_eq!(rbs.line_width, 0.0);
        assert_eq!(rbs.opacity, 0.0);
        assert!(!rbs.shadow_enabled);
    }
}
