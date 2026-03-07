// Theme geometry configuration (border radius, widths, opacities)

use serde::{Deserialize, Serialize};

/// Geometric properties for UI elements.
///
/// Controls border radius, frame widths, and opacity values
/// that vary across platform themes.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeGeometry {
    /// Corner radius for rounded elements (in logical pixels).
    pub radius: Option<f32>,

    /// Window/widget frame border width (in logical pixels).
    pub frame_width: Option<f32>,

    /// Opacity multiplier for disabled elements (0.0 = invisible, 1.0 = fully opaque).
    pub disabled_opacity: Option<f32>,

    /// Opacity multiplier for borders (0.0 = invisible, 1.0 = fully opaque).
    pub border_opacity: Option<f32>,

    /// Scrollbar track width (in logical pixels).
    pub scroll_width: Option<f32>,
}

impl_merge!(ThemeGeometry {
    option { radius, frame_width, disabled_opacity, border_opacity, scroll_width }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        assert!(ThemeGeometry::default().is_empty());
    }

    #[test]
    fn not_empty_when_field_set() {
        let g = ThemeGeometry {
            radius: Some(4.0),
            ..Default::default()
        };
        assert!(!g.is_empty());
    }

    #[test]
    fn merge_some_replaces_none() {
        let mut base = ThemeGeometry::default();
        let overlay = ThemeGeometry {
            radius: Some(8.0),
            frame_width: Some(1.0),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.radius, Some(8.0));
        assert_eq!(base.frame_width, Some(1.0));
    }

    #[test]
    fn merge_none_preserves_base() {
        let mut base = ThemeGeometry {
            radius: Some(4.0),
            disabled_opacity: Some(0.5),
            ..Default::default()
        };
        let overlay = ThemeGeometry::default();
        base.merge(&overlay);
        assert_eq!(base.radius, Some(4.0));
        assert_eq!(base.disabled_opacity, Some(0.5));
    }

    #[test]
    fn serde_toml_round_trip() {
        let geom = ThemeGeometry {
            radius: Some(8.0),
            frame_width: Some(1.0),
            disabled_opacity: Some(0.4),
            border_opacity: Some(0.6),
            scroll_width: Some(12.0),
        };
        let toml_str = toml::to_string(&geom).unwrap();
        let deserialized: ThemeGeometry = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, geom);
    }
}
