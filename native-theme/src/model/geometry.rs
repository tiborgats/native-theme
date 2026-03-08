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

    /// Larger corner radius for dialogs, cards, and panels (in logical pixels).
    pub radius_lg: Option<f32>,

    /// Window/widget frame border width (in logical pixels).
    pub frame_width: Option<f32>,

    /// Opacity multiplier for disabled elements (0.0 = invisible, 1.0 = fully opaque).
    pub disabled_opacity: Option<f32>,

    /// Opacity multiplier for borders (0.0 = invisible, 1.0 = fully opaque).
    pub border_opacity: Option<f32>,

    /// Scrollbar track width (in logical pixels).
    pub scroll_width: Option<f32>,

    /// Whether the platform uses drop shadows on windows and popups.
    pub shadow: Option<bool>,
}

impl_merge!(ThemeGeometry {
    option { radius, radius_lg, frame_width, disabled_opacity, border_opacity, scroll_width, shadow }
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
            radius_lg: Some(12.0),
            frame_width: Some(1.0),
            disabled_opacity: Some(0.4),
            border_opacity: Some(0.6),
            scroll_width: Some(12.0),
            shadow: Some(true),
        };
        let toml_str = toml::to_string(&geom).unwrap();
        let deserialized: ThemeGeometry = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, geom);
    }

    #[test]
    fn new_fields_default_to_none() {
        let g = ThemeGeometry::default();
        assert!(g.radius_lg.is_none());
        assert!(g.shadow.is_none());
    }

    #[test]
    fn merge_new_fields() {
        let mut base = ThemeGeometry::default();
        let overlay = ThemeGeometry {
            radius_lg: Some(16.0),
            shadow: Some(true),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.radius_lg, Some(16.0));
        assert_eq!(base.shadow, Some(true));

        // Verify overlay None doesn't overwrite existing
        let mut base2 = ThemeGeometry {
            radius_lg: Some(12.0),
            shadow: Some(false),
            ..Default::default()
        };
        let overlay2 = ThemeGeometry::default();
        base2.merge(&overlay2);
        assert_eq!(base2.radius_lg, Some(12.0));
        assert_eq!(base2.shadow, Some(false));
    }
}
