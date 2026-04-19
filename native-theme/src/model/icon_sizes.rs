// Icon size configuration

use native_theme_derive::ThemeFields;
use serde::{Deserialize, Serialize};

/// Per-context icon sizes in logical pixels.
///
/// Defines the expected icon size for each visual context. All fields are
/// optional to support partial overlays.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ThemeFields)]
#[serde(default)]
pub struct IconSizes {
    /// Icon size for toolbar buttons (e.g., 24px).
    #[serde(rename = "toolbar_px")]
    pub toolbar: Option<f32>,
    /// Small icon size for inline use (e.g., 16px).
    #[serde(rename = "small_px")]
    pub small: Option<f32>,
    /// Large icon size for menus/lists (e.g., 32px).
    #[serde(rename = "large_px")]
    pub large: Option<f32>,
    /// Icon size for dialog buttons (e.g., 22px).
    #[serde(rename = "dialog_px")]
    pub dialog: Option<f32>,
    /// Icon size for panel headers (e.g., 20px).
    #[serde(rename = "panel_px")]
    pub panel: Option<f32>,
}

impl_merge!(IconSizes {
    option { toolbar, small, large, dialog, panel }
});

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn icon_sizes_default_is_empty() {
        assert!(IconSizes::default().is_empty());
    }

    #[test]
    fn icon_sizes_not_empty_when_toolbar_set() {
        let s = IconSizes {
            toolbar: Some(24.0),
            ..Default::default()
        };
        assert!(!s.is_empty());
    }

    #[test]
    fn icon_sizes_not_empty_when_any_field_set() {
        for sizes in [
            IconSizes {
                toolbar: Some(24.0),
                ..Default::default()
            },
            IconSizes {
                small: Some(16.0),
                ..Default::default()
            },
            IconSizes {
                large: Some(32.0),
                ..Default::default()
            },
            IconSizes {
                dialog: Some(22.0),
                ..Default::default()
            },
            IconSizes {
                panel: Some(20.0),
                ..Default::default()
            },
        ] {
            assert!(!sizes.is_empty());
        }
    }

    #[test]
    fn icon_sizes_merge_overlay_wins() {
        let mut base = IconSizes {
            toolbar: Some(24.0),
            small: Some(16.0),
            large: None,
            dialog: None,
            panel: None,
        };
        let overlay = IconSizes {
            toolbar: None,
            small: Some(18.0),
            large: Some(32.0),
            dialog: None,
            panel: None,
        };
        base.merge(&overlay);
        assert_eq!(base.toolbar, Some(24.0)); // preserved
        assert_eq!(base.small, Some(18.0)); // overlay wins
        assert_eq!(base.large, Some(32.0)); // overlay sets
        assert_eq!(base.dialog, None);
        assert_eq!(base.panel, None);
    }

    #[test]
    fn icon_sizes_merge_none_preserves_base() {
        let mut base = IconSizes {
            toolbar: Some(24.0),
            small: Some(16.0),
            large: Some(32.0),
            dialog: Some(22.0),
            panel: Some(20.0),
        };
        let overlay = IconSizes::default();
        base.merge(&overlay);
        assert_eq!(base.toolbar, Some(24.0));
        assert_eq!(base.small, Some(16.0));
        assert_eq!(base.large, Some(32.0));
        assert_eq!(base.dialog, Some(22.0));
        assert_eq!(base.panel, Some(20.0));
    }

    #[test]
    fn icon_sizes_toml_round_trip() {
        let sizes = IconSizes {
            toolbar: Some(24.0),
            small: Some(16.0),
            large: Some(32.0),
            dialog: Some(22.0),
            panel: Some(20.0),
        };
        let toml_str = toml::to_string(&sizes).unwrap();
        let deserialized: IconSizes = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, sizes);
    }

    #[test]
    fn icon_sizes_toml_partial_round_trip() {
        let sizes = IconSizes {
            toolbar: Some(24.0),
            ..Default::default()
        };
        let toml_str = toml::to_string(&sizes).unwrap();
        let deserialized: IconSizes = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, sizes);
        assert!(deserialized.small.is_none());
    }
}
