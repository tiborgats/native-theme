// Theme spacing scale

use serde::{Deserialize, Serialize};

/// Named spacing scale from extra-extra-small to extra-extra-large.
///
/// All values are in logical pixels. The scale provides a consistent
/// spacing vocabulary across platforms.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeSpacing {
    /// Extra-extra-small spacing (e.g., 2px).
    pub xxs: Option<f32>,

    /// Extra-small spacing (e.g., 4px).
    pub xs: Option<f32>,

    /// Small spacing (e.g., 8px).
    pub s: Option<f32>,

    /// Medium spacing (e.g., 12px).
    pub m: Option<f32>,

    /// Large spacing (e.g., 16px).
    pub l: Option<f32>,

    /// Extra-large spacing (e.g., 24px).
    pub xl: Option<f32>,

    /// Extra-extra-large spacing (e.g., 32px).
    pub xxl: Option<f32>,
}

impl_merge!(ThemeSpacing {
    option { xxs, xs, s, m, l, xl, xxl }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        assert!(ThemeSpacing::default().is_empty());
    }

    #[test]
    fn not_empty_when_field_set() {
        let s = ThemeSpacing {
            m: Some(12.0),
            ..Default::default()
        };
        assert!(!s.is_empty());
    }

    #[test]
    fn merge_some_replaces_none() {
        let mut base = ThemeSpacing::default();
        let overlay = ThemeSpacing {
            s: Some(8.0),
            m: Some(12.0),
            l: Some(16.0),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.s, Some(8.0));
        assert_eq!(base.m, Some(12.0));
        assert_eq!(base.l, Some(16.0));
    }

    #[test]
    fn merge_none_preserves_base() {
        let mut base = ThemeSpacing {
            xxs: Some(2.0),
            xs: Some(4.0),
            ..Default::default()
        };
        let overlay = ThemeSpacing::default();
        base.merge(&overlay);
        assert_eq!(base.xxs, Some(2.0));
        assert_eq!(base.xs, Some(4.0));
    }

    #[test]
    fn serde_toml_round_trip() {
        let spacing = ThemeSpacing {
            xxs: Some(2.0),
            xs: Some(4.0),
            s: Some(8.0),
            m: Some(12.0),
            l: Some(16.0),
            xl: Some(24.0),
            xxl: Some(32.0),
        };
        let toml_str = toml::to_string(&spacing).unwrap();
        let deserialized: ThemeSpacing = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, spacing);
    }
}
