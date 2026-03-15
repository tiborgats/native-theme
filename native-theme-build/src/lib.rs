mod error;
mod schema;

use std::path::Path;

pub(crate) use error::BuildError;
pub(crate) use schema::{MappingValue, MasterConfig, ThemeMapping, KNOWN_THEMES};

/// Simple API: generate icon code from a single TOML file.
///
/// Reads the master TOML at `toml_path`, validates all referenced themes
/// and SVG files, and writes generated Rust code to `OUT_DIR`.
pub fn generate_icons(_toml_path: impl AsRef<Path>) {
    todo!("will be implemented in Plan 04")
}

/// Builder API for composing multiple TOML icon definitions.
pub struct IconGenerator;

impl IconGenerator {
    /// Create a new builder.
    pub fn new() -> Self {
        todo!("will be implemented in Plan 04")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    // === MasterConfig tests ===

    #[test]
    fn master_config_deserializes_full() {
        let toml_str = r#"
name = "app-icon"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
system-themes = ["sf-symbols"]
"#;
        let config: MasterConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.name, "app-icon");
        assert_eq!(config.roles, vec!["play-pause", "skip-forward"]);
        assert_eq!(config.bundled_themes, vec!["material"]);
        assert_eq!(config.system_themes, vec!["sf-symbols"]);
    }

    #[test]
    fn master_config_empty_optional_fields() {
        let toml_str = r#"
name = "x"
roles = ["a"]
"#;
        let config: MasterConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.name, "x");
        assert_eq!(config.roles, vec!["a"]);
        assert!(config.bundled_themes.is_empty());
        assert!(config.system_themes.is_empty());
    }

    #[test]
    fn master_config_rejects_unknown_fields() {
        let toml_str = r#"
name = "x"
roles = ["a"]
bogus = "nope"
"#;
        let result = toml::from_str::<MasterConfig>(toml_str);
        assert!(result.is_err());
    }

    // === MappingValue tests ===

    #[test]
    fn mapping_value_simple() {
        let toml_str = r#"play-pause = "play_pause""#;
        let mapping: BTreeMap<String, MappingValue> = toml::from_str(toml_str).unwrap();
        match &mapping["play-pause"] {
            MappingValue::Simple(s) => assert_eq!(s, "play_pause"),
            _ => panic!("expected Simple variant"),
        }
    }

    #[test]
    fn mapping_value_de_aware() {
        let toml_str =
            r#"play-pause = { kde = "media-playback-start", default = "play" }"#;
        let mapping: BTreeMap<String, MappingValue> = toml::from_str(toml_str).unwrap();
        match &mapping["play-pause"] {
            MappingValue::DeAware(m) => {
                assert_eq!(m["kde"], "media-playback-start");
                assert_eq!(m["default"], "play");
            }
            _ => panic!("expected DeAware variant"),
        }
    }

    #[test]
    fn theme_mapping_mixed_values() {
        let toml_str = r#"
play-pause = "play_pause"
bluetooth = { kde = "preferences-system-bluetooth", default = "bluetooth" }
skip-forward = "skip_next"
"#;
        let mapping: ThemeMapping = toml::from_str(toml_str).unwrap();
        assert_eq!(mapping.len(), 3);
        assert!(matches!(&mapping["play-pause"], MappingValue::Simple(_)));
        assert!(matches!(&mapping["bluetooth"], MappingValue::DeAware(_)));
        assert!(matches!(&mapping["skip-forward"], MappingValue::Simple(_)));
    }

    // === MappingValue::default_name tests ===

    #[test]
    fn mapping_value_default_name_simple() {
        let val = MappingValue::Simple("play_pause".to_string());
        assert_eq!(val.default_name(), Some("play_pause"));
    }

    #[test]
    fn mapping_value_default_name_de_aware() {
        let mut m = BTreeMap::new();
        m.insert("kde".to_string(), "media-playback-start".to_string());
        m.insert("default".to_string(), "play".to_string());
        let val = MappingValue::DeAware(m);
        assert_eq!(val.default_name(), Some("play"));
    }

    #[test]
    fn mapping_value_default_name_de_aware_missing_default() {
        let mut m = BTreeMap::new();
        m.insert("kde".to_string(), "media-playback-start".to_string());
        let val = MappingValue::DeAware(m);
        assert_eq!(val.default_name(), None);
    }

    // === BuildError Display tests ===

    #[test]
    fn build_error_missing_role_format() {
        let err = BuildError::MissingRole {
            role: "play-pause".into(),
            mapping_file: "icons/material/mapping.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("play-pause"), "should contain role name");
        assert!(
            msg.contains("icons/material/mapping.toml"),
            "should contain file path"
        );
    }

    #[test]
    fn build_error_missing_svg_format() {
        let err = BuildError::MissingSvg {
            path: "icons/material/play.svg".into(),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("icons/material/play.svg"),
            "should contain SVG path"
        );
    }

    #[test]
    fn build_error_unknown_role_format() {
        let err = BuildError::UnknownRole {
            role: "bogus".into(),
            mapping_file: "icons/material/mapping.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("bogus"), "should contain role name");
        assert!(
            msg.contains("icons/material/mapping.toml"),
            "should contain file path"
        );
    }

    #[test]
    fn build_error_unknown_theme_format() {
        let err = BuildError::UnknownTheme {
            theme: "nonexistent".into(),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("nonexistent"),
            "should contain theme name"
        );
    }

    #[test]
    fn build_error_missing_default_format() {
        let err = BuildError::MissingDefault {
            role: "bluetooth".into(),
            mapping_file: "icons/freedesktop/mapping.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("bluetooth"), "should contain role name");
        assert!(
            msg.contains("icons/freedesktop/mapping.toml"),
            "should contain file path"
        );
    }

    #[test]
    fn build_error_duplicate_role_format() {
        let err = BuildError::DuplicateRole {
            role: "play-pause".into(),
            file_a: "icons/a.toml".into(),
            file_b: "icons/b.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("play-pause"), "should contain role name");
        assert!(
            msg.contains("icons/a.toml"),
            "should contain first file path"
        );
        assert!(
            msg.contains("icons/b.toml"),
            "should contain second file path"
        );
    }

    // === KNOWN_THEMES tests ===

    #[test]
    fn known_themes_has_all_five() {
        assert_eq!(KNOWN_THEMES.len(), 5);
        assert!(KNOWN_THEMES.contains(&"sf-symbols"));
        assert!(KNOWN_THEMES.contains(&"segoe-fluent"));
        assert!(KNOWN_THEMES.contains(&"freedesktop"));
        assert!(KNOWN_THEMES.contains(&"material"));
        assert!(KNOWN_THEMES.contains(&"lucide"));
    }
}
