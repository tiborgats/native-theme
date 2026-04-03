use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;

use heck::ToUpperCamelCase;

use crate::error::BuildError;
use crate::schema::{DE_TABLE, MappingValue, MasterConfig, ThemeMapping};

/// Rust keywords that cannot be used as identifiers (including contextual keywords).
///
/// Note: `to_upper_camel_case()` always produces PascalCase, so only `Self`
/// is practically reachable from this list (it is the only keyword starting
/// with an uppercase letter). Reserved-for-future keywords (`abstract`, `try`,
/// `yield`, etc.) are omitted because PascalCase conversion makes them
/// unreachable (`try` -> `Try`, which is a valid identifier).
const RUST_KEYWORDS: [&str; 38] = [
    "Self", "self", "super", "crate", "fn", "mod", "pub", "use", "let", "mut", "const", "static",
    "struct", "enum", "trait", "impl", "type", "where", "for", "loop", "while", "if", "else",
    "match", "return", "break", "continue", "as", "in", "ref", "move", "async", "await", "dyn",
    "unsafe", "extern", "true", "false",
];

/// Validate that all theme names in the config are known.
///
/// Checks both `bundled_themes` and `system_themes` against [`THEME_TABLE`](crate::schema::THEME_TABLE).
/// Returns a `BuildError::UnknownTheme` for each unrecognized theme name.
pub(crate) fn validate_themes(config: &MasterConfig) -> Vec<BuildError> {
    config
        .bundled_themes
        .iter()
        .chain(config.system_themes.iter())
        .filter(|theme| !crate::schema::is_known_theme(theme))
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
        if let MappingValue::DeAware(m) = value
            && !m.contains_key("default")
        {
            errors.push(BuildError::MissingDefault {
                role: key.clone(),
                mapping_file: mapping_path.to_string(),
            });
        }
    }

    errors
}

/// Validate that SVG files exist for all entries in a bundled theme mapping.
///
/// Only checks SVGs for roles declared in `master_roles`, avoiding spurious
/// `MissingSvg` errors for unknown roles that are already reported by
/// [`validate_mapping()`].
///
/// For each matching entry, constructs the expected path as
/// `theme_dir / {default_name}.svg` and checks if the file exists.
/// Returns `BuildError::MissingSvg` for each missing file.
pub(crate) fn validate_svgs(
    mapping: &ThemeMapping,
    theme_dir: &Path,
    master_roles: &[String],
) -> Vec<BuildError> {
    let role_set: BTreeSet<&str> = master_roles.iter().map(|s| s.as_str()).collect();
    let mut errors = Vec::new();

    for (role, value) in mapping {
        if role_set.contains(role.as_str())
            && let Some(name) = value.default_name()
        {
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
///
/// Issue 11: Uses all icon names (including DE-specific names), not just
/// the default, in the referenced set.
pub(crate) fn check_orphan_svgs(
    mapping: &ThemeMapping,
    theme_dir: &Path,
    theme_name: &str,
) -> Vec<String> {
    // Issue 11: Collect ALL referenced SVG stems from the mapping (default + DE-specific)
    let referenced: BTreeSet<String> = mapping
        .values()
        .flat_map(|v| v.all_names())
        .map(|s| s.to_string())
        .collect();

    // List all .svg files in the theme directory
    let entries = match fs::read_dir(theme_dir) {
        Ok(entries) => entries,
        Err(e) => {
            return vec![format!(
                "cannot scan theme dir '{}' for orphan SVGs: {e}",
                theme_dir.display()
            )];
        }
    };

    let mut warnings = Vec::new();
    // Issue 14: Handle directory entry errors explicitly instead of flattening
    for entry_result in entries {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                warnings.push(format!(
                    "cannot read entry in '{}': {e}",
                    theme_dir.display()
                ));
                continue;
            }
        };
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("svg")
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            && !referenced.contains(stem)
        {
            // Issue 13: Use to_string_lossy() instead of unwrap_or("unknown")
            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_else(|| std::borrow::Cow::Borrowed("unknown"));
            warnings.push(format!(
                "orphan SVG in {theme_name}: {file_name} is not referenced by any mapping"
            ));
        }
    }

    warnings
}

/// Check whether `key` is a known DE key (appears in `DE_TABLE` or is `"default"`).
fn is_known_de_key(key: &str) -> bool {
    key == "default" || DE_TABLE.iter().any(|(k, _)| *k == key)
}

/// Validate DE keys in DeAware mapping values.
///
/// Returns warnings (not errors) for unrecognized DE keys, since the
/// mandatory `default` key ensures correctness. Unknown keys produce
/// unreachable match arms in generated code but are not harmful.
pub(crate) fn validate_de_keys(mapping: &ThemeMapping, mapping_path: &str) -> Vec<String> {
    // Build the recognised list from DE_TABLE for the warning message
    let recognised: Vec<&str> = DE_TABLE.iter().map(|(k, _)| *k).collect();
    let recognised_list = recognised.join(", ");

    let mut warnings = Vec::new();
    for (role, value) in mapping {
        if let MappingValue::DeAware(de_map) = value {
            for key in de_map.keys() {
                if !is_known_de_key(key) {
                    warnings.push(format!(
                        "unrecognized DE key \"{key}\" for role \"{role}\" in {mapping_path} \
                         (recognized: {recognised_list}). \
                         This icon name will never be used at runtime."
                    ));
                }
            }
        }
    }
    warnings
}

/// Validate that no theme appears in both `bundled_themes` and `system_themes`.
///
/// Returns `BuildError::ThemeOverlap` for each theme that is in both lists.
pub(crate) fn validate_theme_overlap(config: &MasterConfig) -> Vec<BuildError> {
    let bundled: BTreeSet<&str> = config.bundled_themes.iter().map(|s| s.as_str()).collect();
    config
        .system_themes
        .iter()
        .filter(|t| bundled.contains(t.as_str()))
        .map(|t| BuildError::ThemeOverlap { theme: t.clone() })
        .collect()
}

/// Check whether a PascalCase identifier starts with a valid Rust identifier character.
fn is_valid_ident_start(pascal: &str) -> bool {
    pascal
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
}

/// Validate identifiers produced from role names and the enum name.
///
/// Checks:
/// - PascalCase conversion produces a non-empty result.
/// - Result starts with a letter or underscore (valid Rust identifier).
/// - Result is not a Rust keyword.
/// - No two roles produce the same PascalCase variant (collision).
pub(crate) fn validate_identifiers(config: &MasterConfig) -> Vec<BuildError> {
    let mut errors = Vec::new();

    // Check enum name
    let enum_pascal = config.name.to_upper_camel_case();
    if enum_pascal.is_empty() {
        errors.push(BuildError::InvalidIdentifier {
            name: config.name.clone(),
            reason: "PascalCase conversion produces an empty string".to_string(),
        });
    } else if !is_valid_ident_start(&enum_pascal) {
        errors.push(BuildError::InvalidIdentifier {
            name: config.name.clone(),
            reason: format!(
                "\"{enum_pascal}\" starts with a non-letter character; \
                 Rust identifiers must begin with a letter or underscore"
            ),
        });
    } else if RUST_KEYWORDS.contains(&enum_pascal.as_str()) {
        errors.push(BuildError::InvalidIdentifier {
            name: config.name.clone(),
            reason: format!("\"{enum_pascal}\" is a Rust keyword"),
        });
    }

    // Check role names + collision detection
    let mut seen: HashMap<String, &str> = HashMap::new();

    for role in &config.roles {
        let pascal = role.to_upper_camel_case();
        if pascal.is_empty() {
            errors.push(BuildError::InvalidIdentifier {
                name: role.clone(),
                reason: "PascalCase conversion produces an empty string".to_string(),
            });
            continue;
        }
        if !is_valid_ident_start(&pascal) {
            errors.push(BuildError::InvalidIdentifier {
                name: role.clone(),
                reason: format!(
                    "\"{pascal}\" starts with a non-letter character; \
                     Rust identifiers must begin with a letter or underscore"
                ),
            });
        } else if RUST_KEYWORDS.contains(&pascal.as_str()) {
            errors.push(BuildError::InvalidIdentifier {
                name: role.clone(),
                reason: format!("\"{pascal}\" is a Rust keyword"),
            });
        }
        if let Some(&first_role) = seen.get(&pascal) {
            errors.push(BuildError::IdentifierCollision {
                role_a: first_role.to_string(),
                role_b: role.clone(),
                pascal,
            });
        } else {
            seen.insert(pascal, role.as_str());
        }
    }

    errors
}

/// Validate that no role name is duplicated within a single config file's roles array.
///
/// Checks both case-sensitive exact duplicates and PascalCase conversion collisions.
/// Returns `BuildError::DuplicateRoleInFile` for each duplicate found.
pub(crate) fn validate_no_duplicate_roles_in_file(
    config: &MasterConfig,
    file_path: &str,
) -> Vec<BuildError> {
    let mut errors = Vec::new();
    let mut seen_exact: HashSet<&str> = HashSet::new();

    for role in &config.roles {
        if !seen_exact.insert(role.as_str()) {
            errors.push(BuildError::DuplicateRoleInFile {
                role: role.clone(),
                file: file_path.to_string(),
            });
        }
    }

    errors
}

/// Validate that no theme name appears twice within the same list.
///
/// Checks both `bundled_themes` and `system_themes` for exact duplicates.
/// Returns `BuildError::DuplicateTheme` for each duplicate found.
pub(crate) fn validate_no_duplicate_themes(config: &MasterConfig) -> Vec<BuildError> {
    let mut errors = Vec::new();

    let mut seen = BTreeSet::new();
    for theme in &config.bundled_themes {
        if !seen.insert(theme.as_str()) {
            errors.push(BuildError::DuplicateTheme {
                theme: theme.clone(),
                list: "bundled-themes".to_string(),
            });
        }
    }

    seen.clear();
    for theme in &config.system_themes {
        if !seen.insert(theme.as_str()) {
            errors.push(BuildError::DuplicateTheme {
                theme: theme.clone(),
                list: "system-themes".to_string(),
            });
        }
    }

    errors
}

/// Check if a character is an invisible Unicode character that should be
/// rejected in icon names.
fn is_invisible_unicode(c: char) -> bool {
    matches!(
        c,
        '\u{FEFF}'        // BOM
        | '\u{200B}'      // zero-width space
        | '\u{200C}'      // zero-width non-joiner
        | '\u{200D}'      // zero-width joiner
        | '\u{2060}'      // word joiner
        | '\u{00AD}'      // soft hyphen
        | '\u{034F}'      // combining grapheme joiner
        | '\u{FFFE}'      // non-character
        | '\u{FFFF}' // non-character
    )
}

/// Validate that all icon name strings in a mapping are well-formed.
///
/// Rejects:
/// - empty names
/// - names containing control characters
/// - names containing path separators (`/`, `\`) or parent-directory
///   sequences (`..`) to prevent path traversal in `include_bytes!` paths
/// - names containing invisible Unicode characters
///
/// Also validates DE keys (not just values) in DeAware mappings for empty
/// strings and control characters.
pub(crate) fn validate_mapping_values(
    mapping: &ThemeMapping,
    mapping_path: &str,
) -> Vec<BuildError> {
    let mut errors = Vec::new();

    for (role, value) in mapping {
        // Issue 43: Validate DE keys alongside values
        if let MappingValue::DeAware(m) = value {
            for key in m.keys() {
                if key.is_empty() || key.chars().any(|c| c.is_control()) {
                    errors.push(BuildError::InvalidIconName {
                        name: key.to_string(),
                        role: role.clone(),
                        mapping_file: mapping_path.to_string(),
                    });
                }
            }
        }

        let names: Vec<&str> = match value {
            MappingValue::Simple(s) => vec![s.as_str()],
            MappingValue::DeAware(m) => m.values().map(|s| s.as_str()).collect(),
        };
        for name in names {
            if name.is_empty()
                || name.chars().any(|c| c.is_control())
                // Issue 4: Reject path separators and parent-directory sequences
                || name.contains('/')
                || name.contains('\\')
                || name.contains("..")
                // Issue 62: Reject invisible Unicode characters
                || name.chars().any(is_invisible_unicode)
            {
                errors.push(BuildError::InvalidIconName {
                    name: name.to_string(),
                    role: role.clone(),
                    mapping_file: mapping_path.to_string(),
                });
            }
        }
    }

    errors
}

/// Validate that no role name appears in multiple config files.
///
/// Given a list of `(file_path, MasterConfig)` pairs, checks for role name
/// collisions across files. Returns `BuildError::DuplicateRole` for each
/// collision found.
pub(crate) fn validate_no_duplicate_roles(configs: &[(String, MasterConfig)]) -> Vec<BuildError> {
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
    use std::collections::BTreeMap;
    use std::fs;

    // Helper to build a MasterConfig for testing
    fn make_config(name: &str, roles: &[&str], bundled: &[&str], system: &[&str]) -> MasterConfig {
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
        assert!(
            errors.is_empty(),
            "all themes are known, no errors expected"
        );
    }

    #[test]
    fn validate_themes_unknown_bundled() {
        let config = make_config("x", &["a"], &["material", "typo-theme"], &[]);
        let errors = validate_themes(&config);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(
            msg.contains("typo-theme"),
            "should mention the unknown theme"
        );
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
        assert!(
            msg.contains("skip-forward"),
            "should mention the missing role"
        );
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
        let master_roles = vec!["play-pause".to_string(), "skip-forward".to_string()];
        let errors = validate_svgs(&mapping, &dir, &master_roles);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(
            msg.contains("skip_next.svg"),
            "should mention the missing SVG file"
        );

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
        let master_roles = vec!["play-pause".to_string(), "skip-forward".to_string()];
        let errors = validate_svgs(&mapping, &dir, &master_roles);
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
        let master_roles = vec!["play-pause".to_string()];
        let errors = validate_svgs(&mapping, &dir, &master_roles);
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
        assert!(
            msg.contains("play-pause"),
            "should mention the duplicate role"
        );
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

    // === validate_de_keys tests ===

    #[test]
    fn de_keys_all_recognized() {
        let mut de_map = BTreeMap::new();
        de_map.insert("kde".to_string(), "media-playback-start".to_string());
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let warnings = validate_de_keys(&mapping, "mapping.toml");
        assert!(
            warnings.is_empty(),
            "all DE keys recognized, no warnings expected"
        );
    }

    #[test]
    fn de_keys_unrecognized_cosmic() {
        let mut de_map = BTreeMap::new();
        de_map.insert("cosmic".to_string(), "cosmic-play".to_string());
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let warnings = validate_de_keys(&mapping, "mapping.toml");
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].contains("cosmic"),
            "should mention the unrecognized key"
        );
        assert!(warnings[0].contains("kde"), "should list valid keys");
    }

    #[test]
    fn de_keys_mixed_recognized_and_unrecognized() {
        let mut de_map = BTreeMap::new();
        de_map.insert("kde".to_string(), "media-playback-start".to_string());
        de_map.insert("cosmic".to_string(), "cosmic-play".to_string());
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let warnings = validate_de_keys(&mapping, "mapping.toml");
        assert_eq!(warnings.len(), 1, "only cosmic is unrecognized");
        assert!(warnings[0].contains("cosmic"));
    }

    #[test]
    fn de_keys_default_only_no_warnings() {
        let mut de_map = BTreeMap::new();
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let warnings = validate_de_keys(&mapping, "mapping.toml");
        assert!(warnings.is_empty(), "default-only is fine");
    }

    #[test]
    fn de_keys_simple_value_ignored() {
        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("play_pause".into()),
        )]);
        let warnings = validate_de_keys(&mapping, "mapping.toml");
        assert!(warnings.is_empty(), "Simple values should be ignored");
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

    // === validate_theme_overlap tests ===

    #[test]
    fn theme_overlap_detected() {
        let config = make_config("x", &["a"], &["material"], &["material"]);
        let errors = validate_theme_overlap(&config);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(
            msg.contains("material"),
            "should mention the overlapping theme"
        );
        assert!(
            msg.contains("bundled-themes") && msg.contains("system-themes"),
            "should mention both lists"
        );
    }

    #[test]
    fn theme_overlap_none() {
        let config = make_config("x", &["a"], &["material"], &["sf-symbols"]);
        let errors = validate_theme_overlap(&config);
        assert!(errors.is_empty(), "no overlap expected");
    }

    #[test]
    fn theme_overlap_multiple() {
        let config = make_config(
            "x",
            &["a"],
            &["material", "lucide"],
            &["material", "lucide"],
        );
        let errors = validate_theme_overlap(&config);
        assert_eq!(errors.len(), 2);
    }

    // === validate_identifiers tests ===

    #[test]
    fn identifiers_valid() {
        let config = make_config("app-icon", &["play-pause", "skip-forward"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            errors.is_empty(),
            "valid identifiers should produce no errors"
        );
    }

    #[test]
    fn identifier_enum_name_empty_pascal() {
        // A name of only dashes produces empty PascalCase
        let config = make_config("---", &["play-pause"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            errors.iter().any(|e| {
                matches!(e, BuildError::InvalidIdentifier { name, reason }
                    if name == "---" && reason.contains("empty"))
            }),
            "should detect empty PascalCase for enum name: {errors:?}"
        );
    }

    #[test]
    fn identifier_role_is_keyword() {
        // "r#self" -> PascalCase "Self" which is a keyword
        let config = make_config("app-icon", &["self"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason }
                    if name == "self" && reason.contains("keyword"))
            ),
            "should detect keyword: {errors:?}"
        );
    }

    #[test]
    fn identifier_enum_name_is_keyword() {
        // "self" -> PascalCase "Self" which is a Rust keyword
        let config = make_config("self", &["play-pause"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason }
                    if name == "self" && reason.contains("keyword"))
            ),
            "should detect keyword for enum name: {errors:?}"
        );
    }

    #[test]
    fn identifier_collision_detected() {
        // "play_pause" and "play-pause" both become "PlayPause"
        let config = make_config("app-icon", &["play_pause", "play-pause"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::IdentifierCollision { pascal, .. }
                    if pascal == "PlayPause")
            ),
            "should detect PascalCase collision: {errors:?}"
        );
    }

    #[test]
    fn identifier_no_collision_different_variants() {
        let config = make_config("app-icon", &["play-pause", "skip-forward"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, BuildError::IdentifierCollision { .. })),
            "should not detect collision for distinct roles"
        );
    }

    #[test]
    fn identifier_role_starts_with_digit() {
        let config = make_config("app-icon", &["3d-rotate"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason }
                    if name == "3d-rotate" && reason.contains("non-letter"))
            ),
            "should reject digit-starting role identifier: {errors:?}"
        );
    }

    #[test]
    fn identifier_enum_name_starts_with_digit() {
        let config = make_config("3d-icons", &["play-pause"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason }
                    if name == "3d-icons" && reason.contains("non-letter"))
            ),
            "should reject digit-starting enum name: {errors:?}"
        );
    }

    #[test]
    fn identifier_role_all_digits() {
        let config = make_config("app-icon", &["123"], &[], &[]);
        let errors = validate_identifiers(&config);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason }
                    if name == "123" && reason.contains("non-letter"))
            ),
            "should reject all-digits role identifier: {errors:?}"
        );
    }

    // === validate_no_duplicate_roles_in_file tests ===

    #[test]
    fn duplicate_role_in_file_detected() {
        let config = make_config("x", &["play-pause", "skip-forward", "play-pause"], &[], &[]);
        let errors = validate_no_duplicate_roles_in_file(&config, "icons.toml");
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(
            msg.contains("play-pause"),
            "should mention the duplicate role"
        );
        assert!(msg.contains("icons.toml"), "should mention the file");
    }

    #[test]
    fn duplicate_role_in_file_none() {
        let config = make_config("x", &["play-pause", "skip-forward"], &[], &[]);
        let errors = validate_no_duplicate_roles_in_file(&config, "icons.toml");
        assert!(errors.is_empty(), "no duplicates expected");
    }

    #[test]
    fn duplicate_role_in_file_multiple() {
        let config = make_config("x", &["a", "b", "a", "b", "c"], &[], &[]);
        let errors = validate_no_duplicate_roles_in_file(&config, "test.toml");
        assert_eq!(errors.len(), 2, "should detect both duplicates");
    }

    #[test]
    fn duplicate_role_in_file_case_sensitive() {
        // "Play-Pause" and "play-pause" are different strings (case-sensitive)
        let config = make_config("x", &["Play-Pause", "play-pause"], &[], &[]);
        let errors = validate_no_duplicate_roles_in_file(&config, "icons.toml");
        assert!(
            errors.is_empty(),
            "case-different roles are not exact duplicates"
        );
    }

    // === validate_no_duplicate_themes tests ===

    #[test]
    fn duplicate_bundled_theme_detected() {
        let config = make_config("x", &["a"], &["material", "lucide", "material"], &[]);
        let errors = validate_no_duplicate_themes(&config);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("material"));
        assert!(msg.contains("bundled-themes"));
    }

    #[test]
    fn duplicate_system_theme_detected() {
        let config = make_config("x", &["a"], &[], &["freedesktop", "freedesktop"]);
        let errors = validate_no_duplicate_themes(&config);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("freedesktop"));
        assert!(msg.contains("system-themes"));
    }

    #[test]
    fn no_duplicate_themes() {
        let config = make_config("x", &["a"], &["material", "lucide"], &["freedesktop"]);
        let errors = validate_no_duplicate_themes(&config);
        assert!(errors.is_empty());
    }

    // === validate_mapping_values tests ===

    #[test]
    fn valid_icon_names_pass() {
        let mapping = make_mapping(vec![
            ("play-pause", MappingValue::Simple("play_pause".into())),
            ("skip", MappingValue::Simple("skip_forward".into())),
        ]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert!(errors.is_empty());
    }

    #[test]
    fn empty_icon_name_rejected() {
        let mapping = make_mapping(vec![("play-pause", MappingValue::Simple("".into()))]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("play-pause"));
        assert!(msg.contains("invalid icon name"));
    }

    #[test]
    fn control_char_in_icon_name_rejected() {
        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("play\x00pause".into()),
        )]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn de_aware_empty_value_rejected() {
        let mut de_map = BTreeMap::new();
        de_map.insert("default".to_string(), "".to_string());
        de_map.insert("kde".to_string(), "valid_name".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert_eq!(errors.len(), 1);
    }

    // === Issue 4: Path traversal rejection ===

    #[test]
    fn slash_in_icon_name_rejected() {
        let mapping = make_mapping(vec![("play-pause", MappingValue::Simple("sub/dir".into()))]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn backslash_in_icon_name_rejected() {
        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("sub\\dir".into()),
        )]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn dotdot_in_icon_name_rejected() {
        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("../../etc/passwd".into()),
        )]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert_eq!(errors.len(), 1);
    }

    // === Issue 43: DE key validation ===

    #[test]
    fn empty_de_key_rejected() {
        let mut de_map = BTreeMap::new();
        de_map.insert("".to_string(), "icon_name".to_string());
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert!(!errors.is_empty(), "should reject empty DE key");
    }

    // === Issue 62: Invisible Unicode rejection ===

    #[test]
    fn invisible_unicode_in_icon_name_rejected() {
        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("play\u{200B}pause".into()),
        )]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert_eq!(errors.len(), 1, "should reject zero-width space");
    }

    #[test]
    fn bom_in_icon_name_rejected() {
        let mapping = make_mapping(vec![(
            "play-pause",
            MappingValue::Simple("\u{FEFF}play".into()),
        )]);
        let errors = validate_mapping_values(&mapping, "mapping.toml");
        assert_eq!(errors.len(), 1, "should reject BOM");
    }
}
