// Border specification sub-structs for defaults-level and widget-level border properties

use crate::Rgba;
use serde::{Deserialize, Serialize};

/// Defaults-level border specification: color, geometry, and opacity.
///
/// Used on [`ThemeDefaults`](crate::model::ThemeDefaults) for global border
/// properties that are inherited by per-widget borders. Does not include
/// padding fields -- those live on [`WidgetBorderSpec`].
///
/// All fields are optional to support partial overlays -- a DefaultsBorderSpec
/// with only `color` set will only override the color when merged.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DefaultsBorderSpec {
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
    /// Border alpha multiplier 0.0-1.0 (defaults only).
    pub opacity: Option<f32>,
    /// Whether the bordered element has a drop shadow.
    pub shadow_enabled: Option<bool>,
}

impl DefaultsBorderSpec {
    /// All serialized field names for DefaultsBorderSpec, for TOML linting.
    pub const FIELD_NAMES: &[&str] = &[
        "color",
        "corner_radius_px",
        "corner_radius_lg_px",
        "line_width_px",
        "opacity",
        "shadow_enabled",
    ];
}

impl_merge!(DefaultsBorderSpec {
    option { color, corner_radius, corner_radius_lg, line_width, opacity, shadow_enabled }
});

/// Widget-level border specification: color, geometry, and padding.
///
/// Used on per-widget structs for border properties specific to individual
/// widgets. Unlike [`DefaultsBorderSpec`], does not include `corner_radius_lg`
/// or `opacity` (those are defaults-only).
///
/// All fields are optional to support partial overlays.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WidgetBorderSpec {
    /// Border color.
    pub color: Option<Rgba>,
    /// Corner radius in logical pixels.
    #[serde(rename = "corner_radius_px")]
    pub corner_radius: Option<f32>,
    /// Border stroke width in logical pixels.
    #[serde(rename = "line_width_px")]
    pub line_width: Option<f32>,
    /// Whether the bordered element has a drop shadow.
    pub shadow_enabled: Option<bool>,
    /// Horizontal padding inside the border in logical pixels.
    #[serde(rename = "padding_horizontal_px")]
    pub padding_horizontal: Option<f32>,
    /// Vertical padding inside the border in logical pixels.
    #[serde(rename = "padding_vertical_px")]
    pub padding_vertical: Option<f32>,
}

impl WidgetBorderSpec {
    /// All serialized field names for WidgetBorderSpec, for TOML linting.
    pub const FIELD_NAMES: &[&str] = &[
        "color",
        "corner_radius_px",
        "line_width_px",
        "shadow_enabled",
        "padding_horizontal_px",
        "padding_vertical_px",
    ];
}

impl_merge!(WidgetBorderSpec {
    option { color, corner_radius, line_width, shadow_enabled, padding_horizontal, padding_vertical }
});

/// A resolved (non-optional) border specification produced after theme resolution.
///
/// Unlike [`DefaultsBorderSpec`] and [`WidgetBorderSpec`], all fields are
/// required (non-optional) because resolution has already filled in all defaults.
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
    /// Border alpha multiplier 0.0-1.0 (defaults only).
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

    // === DefaultsBorderSpec tests ===

    #[test]
    fn defaults_border_spec_default_is_empty() {
        assert!(DefaultsBorderSpec::default().is_empty());
    }

    #[test]
    fn defaults_border_spec_not_empty_when_color_set() {
        let bs = DefaultsBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            ..Default::default()
        };
        assert!(!bs.is_empty());
    }

    #[test]
    fn defaults_border_spec_toml_round_trip_full() {
        let bs = DefaultsBorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            corner_radius: Some(4.0),
            corner_radius_lg: Some(8.0),
            line_width: Some(1.0),
            opacity: Some(0.15),
            shadow_enabled: Some(true),
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: DefaultsBorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
    }

    #[test]
    fn defaults_border_spec_toml_round_trip_partial() {
        let bs = DefaultsBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(8.0),
            corner_radius_lg: None,
            line_width: None,
            opacity: None,
            shadow_enabled: None,
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: DefaultsBorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
        assert!(deserialized.corner_radius_lg.is_none());
        assert!(deserialized.line_width.is_none());
        assert!(deserialized.opacity.is_none());
        assert!(deserialized.shadow_enabled.is_none());
    }

    #[test]
    fn defaults_border_spec_merge_overlay_wins() {
        let mut base = DefaultsBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(4.0),
            ..Default::default()
        };
        let overlay = DefaultsBorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.color, Some(Rgba::rgb(200, 200, 200)));
        // base corner_radius preserved since overlay corner_radius is None
        assert_eq!(base.corner_radius, Some(4.0));
    }

    // === WidgetBorderSpec tests ===

    #[test]
    fn widget_border_spec_default_is_empty() {
        assert!(WidgetBorderSpec::default().is_empty());
    }

    #[test]
    fn widget_border_spec_not_empty_when_color_set() {
        let bs = WidgetBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            ..Default::default()
        };
        assert!(!bs.is_empty());
    }

    #[test]
    fn widget_border_spec_toml_round_trip_full() {
        let bs = WidgetBorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            corner_radius: Some(4.0),
            line_width: Some(1.0),
            shadow_enabled: Some(true),
            padding_horizontal: Some(8.0),
            padding_vertical: Some(6.0),
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: WidgetBorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
    }

    #[test]
    fn widget_border_spec_toml_round_trip_partial() {
        let bs = WidgetBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(8.0),
            line_width: None,
            shadow_enabled: None,
            padding_horizontal: None,
            padding_vertical: None,
        };
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: WidgetBorderSpec = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, bs);
        assert!(deserialized.line_width.is_none());
        assert!(deserialized.shadow_enabled.is_none());
        assert!(deserialized.padding_horizontal.is_none());
        assert!(deserialized.padding_vertical.is_none());
    }

    #[test]
    fn widget_border_spec_merge_overlay_wins() {
        let mut base = WidgetBorderSpec {
            color: Some(Rgba::rgb(100, 100, 100)),
            corner_radius: Some(4.0),
            ..Default::default()
        };
        let overlay = WidgetBorderSpec {
            color: Some(Rgba::rgb(200, 200, 200)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.color, Some(Rgba::rgb(200, 200, 200)));
        // base corner_radius preserved since overlay corner_radius is None
        assert_eq!(base.corner_radius, Some(4.0));
    }

    // === ResolvedBorderSpec tests (unchanged) ===

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
