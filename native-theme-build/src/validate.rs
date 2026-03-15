#[cfg(test)]
use std::collections::BTreeMap;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::Path;

use crate::error::BuildError;
use crate::schema::{MappingValue, MasterConfig, ThemeMapping, KNOWN_THEMES};

/// Validate that all theme names in the config are known.
///
/// Checks both `bundled_themes` and `system_themes` against `KNOWN_THEMES`.
/// Returns a `BuildError::UnknownTheme` for each unrecognized theme name.
pub(crate) fn validate_themes(config: &MasterConfig) -> Vec<BuildError> {
    config
        .bundled_themes
        .iter()
        .chain(config.system_themes.iter())
        .filter(|theme| !KNOWN_THEMES.contains(&theme.as_str()))
        .map(|theme| BuildError::UnknownTheme {
            theme: theme.clone(),
        })
        .collect()
}

/// Validate a theme mapping against the master role list.
///
/// Checks:
/// - Every master role has an entry in the mapping (VAL-01: MissingRole)
/// - Every mapping key is a known master role (VAL-03: UnknownRole)
/// - Every `DeAware` value has a `"default"` key (VAL-04: MissingDefault)
pub(crate) fn validate_mapping(
    master_roles: &[String],
    mapping: &ThemeMapping,
    mapping_path: &str,
) -> Vec<BuildError> {
    let mut errors = Vec::new();

    // VAL-01: Check every master role is present in the mapping
    for role in master_roles {
        if !mapping.contains_key(role) {
            errors.push(BuildError::MissingRole {
                role: role.clone(),
                mapping_file: mapping_path.to_string(),
            });
        }
    }

    let master_set: BTreeSet<&str> = master_roles.iter().map(|s| s.as_str()).collect();

    for (key, value) in mapping {
        // VAL-03: Check every mapping key is a known master role
        if !master_set.contains(key.as_str()) {
            errors.push(BuildError::UnknownRole {
                role: key.clone(),
                mapping_file: mapping_path.to_string(),
            });
        }

        // VAL-04: Check DE-aware values have a "default" key
        if let MappingValue::DeAware(m) = value {
            if !m.contains_key("default") {
                errors.push(BuildError::MissingDefault {
                    role: key.clone(),
                    mapping_file: mapping_path.to_string(),
                });
            }
        }
    }

    errors
}

/// Validate that SVG files exist for all entries in a bundled theme mapping.
///
/// For each mapping entry, constructs the expected path as
/// `theme_dir / {default_name}.svg` and checks if the file exists.
/// Returns `BuildError::MissingSvg` for each missing file.
pub(crate) fn validate_svgs(
    mapping: &ThemeMapping,
    theme_dir: &Path,
    _mapping_path: &str,
) -> Vec<BuildError> {
    let mut errors = Vec::new();

    for (_role, value) in mapping {
        if let Some(name) = value.default_name() {
            let svg_path = theme_dir.join(format!("{name}.svg"));
            if !svg_path.exists() {
                errors.push(BuildError::MissingSvg {
                    path: svg_path.to_string_lossy().into_owned(),
                });
            }
        }
    }

    errors
}

/// Find orphan SVG files not referenced by any mapping entry.
///
/// Lists all `.svg` files in `theme_dir` and checks if each is referenced
/// by at least one mapping entry. Returns warning strings for unreferenced files.
pub(crate) fn check_orphan_svgs(
    mapping: &ThemeMapping,
    theme_dir: &Path,
    theme_name: &str,
) -> Vec<String> {
    // Collect all referenced SVG stems from the mapping
    let referenced: BTreeSet<String> = mapping
        .values()
        .filter_map(|v| v.default_name().map(|s| s.to_string()))
        .collect();

    // List all .svg files in the theme directory
    let entries = match fs::read_dir(theme_dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut warnings = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("svg") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if !referenced.contains(stem) {
                    let file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    warnings.push(format!(
                        "orphan SVG in {theme_name}: {file_name} is not referenced by any mapping"
                    ));
                }
            }
        }
    }

    warnings
}

/// Validate that no role name appears in multiple config files.
///
/// Given a list of `(file_path, MasterConfig)` pairs, checks for role name
/// collisions across files. Returns `BuildError::DuplicateRole` for each
/// collision found.
pub(crate) fn validate_no_duplicate_roles(
    configs: &[(String, MasterConfig)],
) -> Vec<BuildError> {
    // Map from role name to the file that first declared it
    let mut seen: HashMap<&str, &str> = HashMap::new();
    let mut errors = Vec::new();

    for (file_path, config) in configs {
        for role in &config.roles {
            if let Some(&first_file) = seen.get(role.as_str()) {
                errors.push(BuildError::DuplicateRole {
                    role: role.clone(),
                    file_a: first_file.to_string(),
                    file_b: file_path.clone(),
                });
            } else {
                seen.insert(role.as_str(), file_path.as_str());
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper to build a MasterConfig for testing
    fn make_config(
        name: &str,
        roles: &[&str],
        bundled: &[&str],
        system: &[&str],
    ) -> MasterConfig {
        MasterConfig {
            name: name.to_string(),
            roles: roles.iter().map(|s| s.to_string()).collect(),
            bundled_themes: bundled.iter().map(|s| s.to_string()).collect(),
            system_themes: system.iter().map(|s| s.to_string()).collect(),
        }
    }

    // Helper to build a ThemeMapping from (key, MappingValue) pairs
    fn make_mapping(entries: Vec<(&str, MappingValue)>) -> ThemeMapping {
        entries
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    }

    // === validate_themes tests ===

    #[test]
    fn validate_themes_all_known() {
        let config = make_config("x", &["a"], &["material"], &["sf-symbols"]);
        let errors = validate_themes(&config);
        assert!(errors.is_empty(), "all themes are known, no errors expected");
    }

    #[test]
    fn validate_themes_unknown_bundled() {
        let config = make_config("x", &["a"], &["material", "typo-theme"], &[]);
        let errors = validate_themes(&config);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("typo-theme"), "should mention the unknown theme");
    }

    #[test]
    fn validate_themes_unknown_system() {
        let config = make_config("x", &["a"], &[], &["bogus-os"]);
        let errors = validate_themes(&config);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("bogus-os"), "should mention the unknown theme");
    }

    #[test]
    fn validate_themes_multiple_unknown() {
        let config = make_config("x", &["a"], &["nope1"], &["nope2"]);
        let errors = validate_themes(&config);
        assert_eq!(errors.len(), 2);
    }

    // === validate_mapping tests (VAL-01, VAL-03, VAL-04) ===

    #[test]
    fn val01_missing_role() {
        let roles = vec!["play-pause".to_string(), "skip-forward".to_string()];
        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("play_pause".into()),
        )]);
        let errors = validate_mapping(&roles, &mapping, "icons/material/mapping.toml");
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("skip-forward"), "should mention the missing role");
        assert!(
            msg.contains("icons/material/mapping.toml"),
            "should mention the mapping file"
        );
    }

    #[test]
    fn val03_unknown_role() {
        let roles = vec!["play-pause".to_string()];
        let mapping = make_mapping(vec![
            ("play-pause", MappingValue::Simple("play_pause".into())),
            ("bluetooth", MappingValue::Simple("bluetooth".into())),
        ]);
        let errors = validate_mapping(&roles, &mapping, "icons/material/mapping.toml");
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("bluetooth"), "should mention the unknown role");
        assert!(
            msg.contains("icons/material/mapping.toml"),
            "should mention the mapping file"
        );
    }

    #[test]
    fn val04_missing_default() {
        let roles = vec!["play-pause".to_string()];
        let mut de_map = BTreeMap::new();
        de_map.insert("kde".to_string(), "media-playback-start".to_string());
        // No "default" key
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let errors = validate_mapping(&roles, &mapping, "icons/freedesktop/mapping.toml");
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("play-pause"), "should mention the role");
        assert!(
            msg.contains("icons/freedesktop/mapping.toml"),
            "should mention the mapping file"
        );
        assert!(
            msg.contains("default"),
            "should mention the missing default key"
        );
    }

    #[test]
    fn validate_mapping_all_valid() {
        let roles = vec!["play-pause".to_string(), "skip-forward".to_string()];
        let mapping = make_mapping(vec![
            ("play-pause", MappingValue::Simple("play_pause".into())),
            ("skip-forward", MappingValue::Simple("skip_next".into())),
        ]);
        let errors = validate_mapping(&roles, &mapping, "mapping.toml");
        assert!(errors.is_empty(), "no errors expected for valid mapping");
    }

    #[test]
    fn validate_mapping_de_aware_with_default_ok() {
        let roles = vec!["play-pause".to_string()];
        let mut de_map = BTreeMap::new();
        de_map.insert("kde".to_string(), "media-playback-start".to_string());
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let errors = validate_mapping(&roles, &mapping, "mapping.toml");
        assert!(errors.is_empty(), "DE-aware with default should be valid");
    }

    // === validate_svgs tests (VAL-02) ===

    #[test]
    fn val02_missing_svg() {
        let dir = std::env::temp_dir().join("native_theme_test_val02_missing");
        let _ = fs::create_dir_all(&dir);
        // Create one SVG, leave the other missing
        fs::write(dir.join("play_pause.svg"), "<svg/>").unwrap();

        let mapping = make_mapping(vec![
            ("play-pause", MappingValue::Simple("play_pause".into())),
            ("skip-forward", MappingValue::Simple("skip_next".into())),
        ]);
        let errors = validate_svgs(&mapping, &dir, "icons/material/mapping.toml");
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("skip_next.svg"), "should mention the missing SVG file");

        // Cleanup
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn val02_all_svgs_present() {
        let dir = std::env::temp_dir().join("native_theme_test_val02_all");
        let _ = fs::create_dir_all(&dir);
        fs::write(dir.join("play_pause.svg"), "<svg/>").unwrap();
        fs::write(dir.join("skip_next.svg"), "<svg/>").unwrap();

        let mapping = make_mapping(vec![
            ("play-pause", MappingValue::Simple("play_pause".into())),
            ("skip-forward", MappingValue::Simple("skip_next".into())),
        ]);
        let errors = validate_svgs(&mapping, &dir, "mapping.toml");
        assert!(errors.is_empty(), "all SVGs present, no errors expected");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn val02_de_aware_uses_default_name() {
        let dir = std::env::temp_dir().join("native_theme_test_val02_deaware");
        let _ = fs::create_dir_all(&dir);
        // The default name is "play", so expect play.svg
        fs::write(dir.join("play.svg"), "<svg/>").unwrap();

        let mut de_map = BTreeMap::new();
        de_map.insert("kde".to_string(), "media-playback-start".to_string());
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let errors = validate_svgs(&mapping, &dir, "mapping.toml");
        assert!(errors.is_empty(), "SVG for default name exists");

        let _ = fs::remove_dir_all(&dir);
    }

    // === check_orphan_svgs tests (VAL-05) ===

    #[test]
    fn val05_orphan_svg() {
        let dir = std::env::temp_dir().join("native_theme_test_val05_orphan");
        let _ = fs::create_dir_all(&dir);
        fs::write(dir.join("play_pause.svg"), "<svg/>").unwrap();
        fs::write(dir.join("unused_icon.svg"), "<svg/>").unwrap();

        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("play_pause".into()),
        )]);
        let warnings = check_orphan_svgs(&mapping, &dir, "material");
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].contains("unused_icon.svg"),
            "should mention the orphan file"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn val05_no_orphans() {
        let dir = std::env::temp_dir().join("native_theme_test_val05_none");
        let _ = fs::create_dir_all(&dir);
        fs::write(dir.join("play_pause.svg"), "<svg/>").unwrap();

        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("play_pause".into()),
        )]);
        let warnings = check_orphan_svgs(&mapping, &dir, "material");
        assert!(warnings.is_empty(), "no orphans expected");

        let _ = fs::remove_dir_all(&dir);
    }

    // === validate_no_duplicate_roles tests (VAL-06) ===

    #[test]
    fn val06_duplicate_role() {
        let config_a = make_config("a", &["play-pause", "skip-forward"], &[], &[]);
        let config_b = make_config("b", &["play-pause", "volume-up"], &[], &[]);
        let configs = vec![
            ("icons/a.toml".to_string(), config_a),
            ("icons/b.toml".to_string(), config_b),
        ];
        let errors = validate_no_duplicate_roles(&configs);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("play-pause"), "should mention the duplicate role");
        assert!(msg.contains("icons/a.toml"), "should mention first file");
        assert!(msg.contains("icons/b.toml"), "should mention second file");
    }

    #[test]
    fn val06_no_duplicates() {
        let config_a = make_config("a", &["play-pause"], &[], &[]);
        let config_b = make_config("b", &["volume-up"], &[], &[]);
        let configs = vec![
            ("a.toml".to_string(), config_a),
            ("b.toml".to_string(), config_b),
        ];
        let errors = validate_no_duplicate_roles(&configs);
        assert!(errors.is_empty(), "no duplicates expected");
    }

    #[test]
    fn val06_three_files_duplicate() {
        let config_a = make_config("a", &["play-pause"], &[], &[]);
        let config_b = make_config("b", &["skip-forward"], &[], &[]);
        let config_c = make_config("c", &["play-pause"], &[], &[]);
        let configs = vec![
            ("a.toml".to_string(), config_a),
            ("b.toml".to_string(), config_b),
            ("c.toml".to_string(), config_c),
        ];
        let errors = validate_no_duplicate_roles(&configs);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("play-pause"));
    }
}
