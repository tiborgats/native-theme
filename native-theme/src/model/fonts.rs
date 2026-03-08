// Theme font configuration

use serde::{Deserialize, Serialize};

/// Font family and size settings for UI text.
///
/// All fields are optional, allowing partial theme definitions
/// that can be merged with other themes.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeFonts {
    /// Primary UI font family name.
    pub family: Option<String>,

    /// Primary UI font size in points.
    pub size: Option<f32>,

    /// Monospace font family name (for code, terminal, etc.).
    pub mono_family: Option<String>,

    /// Monospace font size in points.
    pub mono_size: Option<f32>,
}

impl_merge!(ThemeFonts {
    option { family, size, mono_family, mono_size }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        assert!(ThemeFonts::default().is_empty());
    }

    #[test]
    fn not_empty_when_family_set() {
        let f = ThemeFonts {
            family: Some("Noto Sans".into()),
            ..Default::default()
        };
        assert!(!f.is_empty());
    }

    #[test]
    fn merge_some_replaces_none() {
        let mut base = ThemeFonts::default();
        let overlay = ThemeFonts {
            family: Some("Inter".into()),
            size: Some(14.0),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.family.as_deref(), Some("Inter"));
        assert_eq!(base.size, Some(14.0));
    }

    #[test]
    fn merge_none_preserves_base() {
        let mut base = ThemeFonts {
            family: Some("Noto Sans".into()),
            size: Some(12.0),
            ..Default::default()
        };
        let overlay = ThemeFonts::default();
        base.merge(&overlay);
        assert_eq!(base.family.as_deref(), Some("Noto Sans"));
        assert_eq!(base.size, Some(12.0));
    }

    #[test]
    fn serde_toml_round_trip() {
        let fonts = ThemeFonts {
            family: Some("Inter".into()),
            size: Some(14.0),
            mono_family: Some("JetBrains Mono".into()),
            mono_size: Some(13.0),
        };
        let toml_str = toml::to_string(&fonts).unwrap();
        let deserialized: ThemeFonts = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, fonts);
    }

    #[test]
    fn serde_skips_none_fields() {
        let fonts = ThemeFonts {
            family: Some("Inter".into()),
            ..Default::default()
        };
        let toml_str = toml::to_string(&fonts).unwrap();
        assert!(toml_str.contains("family"));
        assert!(!toml_str.contains("size"));
        assert!(!toml_str.contains("mono_family"));
        assert!(!toml_str.contains("mono_size"));
    }
}
