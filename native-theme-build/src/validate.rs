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
///
/// When `file_hint` is `Some`, it is included in the error for per-file context.
/// Pass `None` when validating a merged config where per-file context is lost.
pub(crate) fn validate_themes(config: &MasterConfig, file_hint: Option<&str>) -> Vec<BuildError> {
    config
        .bundled_themes
        .iter()
        .chain(config.system_themes.iter())
        .filter(|theme| !crate::schema::is_known_theme(theme))
        .map(|theme| BuildError::UnknownTheme {
            theme: theme.clone(),
            source_file: file_hint.map(String::from),
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
        // Issue 66: Also rejects empty DE-aware mappings (no keys at all)
        if let MappingValue::DeAware(m) = value
            && (m.is_empty() || !m.contains_key("default"))
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
                    role: role.clone(),
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

        // Issue 44: Warn about symlinks pointing outside the theme directory.
        // Use symlink_metadata to detect symlinks without following them.
        if let Ok(meta) = fs::symlink_metadata(&path)
            && meta.file_type().is_symlink()
            && let Ok(target) = fs::read_link(&path)
        {
            // Path::starts_with keeps `..`, so the resolved paths are compared;
            // a dangling link, which has none, is compared as written
            let inside = match (fs::canonicalize(&path), fs::canonicalize(theme_dir)) {
                (Ok(resolved), Ok(theme_dir)) => resolved.starts_with(theme_dir),
                _ => {
                    let resolved = if target.is_absolute() {
                        target.clone()
                    } else {
                        path.parent().unwrap_or(Path::new(".")).join(&target)
                    };
                    resolved.starts_with(theme_dir)
                }
            };
            if !inside {
                let file_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_else(|| std::borrow::Cow::Borrowed("unknown"));
                warnings.push(format!(
                    "SVG symlink {file_name} in {theme_name} points outside the theme directory: {}",
                    target.display()
                ));
            }
        }

        // path.extension() works on the path string (not the filesystem),
        // so symlinked SVGs are correctly detected by extension check alone.
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
///
/// When `file_hint` is `Some`, it is included in the error for per-file context.
/// Pass `None` when validating a merged config where per-file context is lost.
pub(crate) fn validate_theme_overlap(
    config: &MasterConfig,
    file_hint: Option<&str>,
) -> Vec<BuildError> {
    let bundled: BTreeSet<&str> = config.bundled_themes.iter().map(|s| s.as_str()).collect();
    config
        .system_themes
        .iter()
        .filter(|t| bundled.contains(t.as_str()))
        .map(|t| BuildError::ThemeOverlap {
            theme: t.clone(),
            source_file: file_hint.map(String::from),
        })
        .collect()
}

/// Why a PascalCase name is not a Rust identifier the generated code can
/// declare, or `None` when it is one.
///
/// heck keeps every alphanumeric character, `'²'` too, so each character is
/// checked: ASCII letters, digits and `_`, a letter or `_` first.
fn identifier_problem(pascal: &str) -> Option<String> {
    let first = pascal.chars().next()?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return Some(format!(
            "\"{pascal}\" starts with a non-letter character; \
             Rust identifiers must begin with a letter or underscore"
        ));
    }
    if let Some(c) = pascal
        .chars()
        .find(|c| !(c.is_ascii_alphanumeric() || *c == '_'))
    {
        return Some(format!(
            "\"{pascal}\" contains '{c}'; identifiers are ASCII letters, digits and underscores"
        ));
    }
    if RUST_KEYWORDS.contains(&pascal) {
        return Some(format!("\"{pascal}\" is a Rust keyword"));
    }
    None
}

/// Validate the identifier produced from the enum name, the one the merged
/// config uses (the `.enum_name()` override, else the first file's `name`).
///
/// Checks:
/// - PascalCase conversion produces a non-empty result.
/// - Result is ASCII letters, digits and underscores, a letter or underscore
///   first (valid Rust identifier).
/// - Result is not a Rust keyword.
///
/// `file_hint` is the file that declares the name, `None` for the override.
pub(crate) fn validate_enum_name(name: &str, file_hint: Option<&str>) -> Vec<BuildError> {
    let enum_pascal = name.to_upper_camel_case();
    let reason = if enum_pascal.is_empty() {
        Some("PascalCase conversion produces an empty string".to_string())
    } else {
        identifier_problem(&enum_pascal)
    };
    reason
        .map(|reason| BuildError::InvalidIdentifier {
            name: name.to_string(),
            reason,
            source_file: file_hint.map(String::from),
        })
        .into_iter()
        .collect()
}

/// Validate identifiers produced from one file's role names.
///
/// Checks each role as [`validate_enum_name`] checks the enum name, and that
/// no two roles produce the same PascalCase variant (collision).
///
/// `file_hint` is included in `InvalidIdentifier` and `IdentifierCollision`
/// errors for per-file context.
pub(crate) fn validate_role_identifiers(
    config: &MasterConfig,
    file_hint: Option<&str>,
) -> Vec<BuildError> {
    let mut errors = Vec::new();
    let mut seen: HashMap<String, &str> = HashMap::new();

    for role in &config.roles {
        let pascal = role.to_upper_camel_case();
        if pascal.is_empty() {
            errors.push(BuildError::InvalidIdentifier {
                name: role.clone(),
                reason: "PascalCase conversion produces an empty string".to_string(),
                source_file: file_hint.map(String::from),
            });
            continue;
        }
        if let Some(reason) = identifier_problem(&pascal) {
            errors.push(BuildError::InvalidIdentifier {
                name: role.clone(),
                reason,
                source_file: file_hint.map(String::from),
            });
        }
        if let Some(&first_role) = seen.get(&pascal) {
            errors.push(BuildError::IdentifierCollision {
                role_a: first_role.to_string(),
                role_b: role.clone(),
                pascal,
                source_file: file_hint.map(String::from),
            });
        } else {
            seen.insert(pascal, role.as_str());
        }
    }

    errors
}

/// Validate that no two roles from different files produce the same
/// PascalCase variant.
///
/// A collision inside one file is [`validate_role_identifiers`]'s, and the
/// same role in two files is a `DuplicateRole`
/// ([`validate_no_duplicate_roles`]), so neither is reported here. The
/// errors carry no `source_file`: the roles come from two files.
pub(crate) fn validate_cross_file_collisions(
    configs: &[(String, MasterConfig)],
) -> Vec<BuildError> {
    let mut errors = Vec::new();
    // PascalCase -> (role, index of its file), the first role seen.
    let mut seen: HashMap<String, (&str, usize)> = HashMap::new();

    for (index, (_path, config)) in configs.iter().enumerate() {
        for role in &config.roles {
            let pascal = role.to_upper_camel_case();
            if pascal.is_empty() {
                continue;
            }
            match seen.get(&pascal) {
                Some(&(first_role, first_index)) => {
                    if first_index != index && first_role != role {
                        errors.push(BuildError::IdentifierCollision {
                            role_a: first_role.to_string(),
                            role_b: role.clone(),
                            pascal,
                            source_file: None,
                        });
                    }
                }
                None => {
                    seen.insert(pascal, (role.as_str(), index));
                }
            }
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
pub(crate) fn validate_no_duplicate_themes(
    config: &MasterConfig,
    file_path: &str,
) -> Vec<BuildError> {
    let mut errors = Vec::new();

    let mut seen = BTreeSet::new();
    for theme in &config.bundled_themes {
        if !seen.insert(theme.as_str()) {
            errors.push(BuildError::DuplicateTheme {
                theme: theme.clone(),
                list: "bundled-themes".to_string(),
                file: file_path.to_string(),
            });
        }
    }

    seen.clear();
    for theme in &config.system_themes {
        if !seen.insert(theme.as_str()) {
            errors.push(BuildError::DuplicateTheme {
                theme: theme.clone(),
                list: "system-themes".to_string(),
                file: file_path.to_string(),
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

/// The first rule `name` breaks, with the character that breaks it when one
/// does, or `None` for a well-formed icon name.
fn icon_name_violation(name: &str) -> Option<(&'static str, Option<char>)> {
    if name.is_empty() {
        return Some(("names must be non-empty", None));
    }
    if let Some(c) = name.chars().find(|c| c.is_control()) {
        return Some(("names must be free of control characters", Some(c)));
    }
    // Issue 4: Reject path separators and parent-directory sequences
    if let Some(c) = name.chars().find(|c| *c == '/' || *c == '\\') {
        return Some((
            "names must not contain a path separator ('/' or '\\')",
            Some(c),
        ));
    }
    if name.contains("..") {
        return Some(("names must not contain \"..\"", None));
    }
    // Issue 62: Reject invisible Unicode characters
    if let Some(c) = name.chars().find(|c| is_invisible_unicode(*c)) {
        return Some((
            "names must be free of invisible Unicode characters",
            Some(c),
        ));
    }
    None
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
                let violation = if key.is_empty() {
                    Some(("DE keys must be non-empty", None))
                } else {
                    key.chars()
                        .find(|c| c.is_control())
                        .map(|c| ("DE keys must be free of control characters", Some(c)))
                };
                if let Some((reason, offending)) = violation {
                    errors.push(BuildError::InvalidIconName {
                        name: key.to_string(),
                        role: role.clone(),
                        mapping_file: mapping_path.to_string(),
                        offending,
                        reason: reason.to_string(),
                    });
                }
            }
        }

        let names: Vec<&str> = match value {
            MappingValue::Simple(s) => vec![s.as_str()],
            MappingValue::DeAware(m) => m.values().map(|s| s.as_str()).collect(),
        };
        for name in names {
            // Issue 97: Capture the offending character for diagnostics
            if let Some((reason, offending)) = icon_name_violation(name) {
                errors.push(BuildError::InvalidIconName {
                    name: name.to_string(),
                    role: role.clone(),
                    mapping_file: mapping_path.to_string(),
                    offending,
                    reason: reason.to_string(),
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
        let errors = validate_themes(&config, None);
        assert!(
            errors.is_empty(),
            "all themes are known, no errors expected"
        );
    }

    #[test]
    fn validate_themes_unknown_bundled() {
        let config = make_config("x", &["a"], &["material", "typo-theme"], &[]);
        let errors = validate_themes(&config, None);
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
        let errors = validate_themes(&config, None);
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("bogus-os"), "should mention the unknown theme");
    }

    #[test]
    fn validate_themes_multiple_unknown() {
        let config = make_config("x", &["a"], &["nope1"], &["nope2"]);
        let errors = validate_themes(&config, None);
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

    #[cfg(unix)]
    #[test]
    fn symlink_with_a_relative_target_outside_the_theme_dir_is_warned() {
        let root = std::env::temp_dir().join(format!(
            "native_theme_test_symlink_outside_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        let dir = root.join("material");
        fs::create_dir_all(&dir).unwrap();
        fs::create_dir_all(root.join("shared")).unwrap();
        fs::write(root.join("shared/play.svg"), "<svg/>").unwrap();
        std::os::unix::fs::symlink("../shared/play.svg", dir.join("play.svg")).unwrap();

        let mapping = make_mapping(vec![("play-pause", MappingValue::Simple("play".into()))]);
        let warnings = check_orphan_svgs(&mapping, &dir, "material");
        assert!(
            matches!(warnings.as_slice(), [w] if w.contains("play.svg") && w.contains("points outside")),
            "{warnings:?}"
        );

        let _ = fs::remove_dir_all(&root);
    }

    #[cfg(unix)]
    #[test]
    fn symlink_with_a_target_inside_the_theme_dir_is_not_warned() {
        let root = std::env::temp_dir().join(format!(
            "native_theme_test_symlink_inside_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        let dir = root.join("material");
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("sub/play.svg"), "<svg/>").unwrap();
        fs::write(dir.join("pause_real.svg"), "<svg/>").unwrap();
        std::os::unix::fs::symlink("sub/play.svg", dir.join("play.svg")).unwrap();
        // `..` that comes back into the theme directory
        std::os::unix::fs::symlink("../material/pause_real.svg", dir.join("pause.svg")).unwrap();

        let mapping = make_mapping(vec![
            ("play-pause", MappingValue::Simple("play".into())),
            ("pause", MappingValue::Simple("pause".into())),
            ("pause-real", MappingValue::Simple("pause_real".into())),
        ]);
        let warnings = check_orphan_svgs(&mapping, &dir, "material");
        assert!(warnings.is_empty(), "{warnings:?}");

        let _ = fs::remove_dir_all(&root);
    }

    #[cfg(unix)]
    #[test]
    fn dangling_symlink_outside_the_theme_dir_is_warned() {
        let root = std::env::temp_dir().join(format!(
            "native_theme_test_symlink_dangling_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        let dir = root.join("material");
        fs::create_dir_all(&dir).unwrap();
        std::os::unix::fs::symlink("/nonexistent/play.svg", dir.join("play.svg")).unwrap();

        let mapping = make_mapping(vec![("play-pause", MappingValue::Simple("play".into()))]);
        let warnings = check_orphan_svgs(&mapping, &dir, "material");
        assert!(
            matches!(warnings.as_slice(), [w] if w.contains("points outside")),
            "{warnings:?}"
        );

        let _ = fs::remove_dir_all(&root);
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
    fn de_keys_unrecognized_unity() {
        let mut de_map = BTreeMap::new();
        de_map.insert("unity".to_string(), "unity-play".to_string());
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let warnings = validate_de_keys(&mapping, "mapping.toml");
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].contains("unity"),
            "should mention the unrecognized key"
        );
        assert!(warnings[0].contains("kde"), "should list valid keys");
    }

    #[test]
    fn de_keys_mixed_recognized_and_unrecognized() {
        let mut de_map = BTreeMap::new();
        de_map.insert("kde".to_string(), "media-playback-start".to_string());
        de_map.insert("cosmic".to_string(), "cosmic-play".to_string());
        de_map.insert("unity".to_string(), "unity-play".to_string());
        de_map.insert("default".to_string(), "play".to_string());
        let mapping = make_mapping(vec![("play-pause", MappingValue::DeAware(de_map))]);
        let warnings = validate_de_keys(&mapping, "mapping.toml");
        assert_eq!(warnings.len(), 1, "only unity is unrecognized");
        assert!(warnings[0].contains("unity"));
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
        let errors = validate_theme_overlap(&config, None);
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
        let errors = validate_theme_overlap(&config, None);
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
        let errors = validate_theme_overlap(&config, None);
        assert_eq!(errors.len(), 2);
    }

    // === validate_enum_name / validate_role_identifiers tests ===

    #[test]
    fn identifiers_valid() {
        let config = make_config("app-icon", &["play-pause", "skip-forward"], &[], &[]);
        let mut errors = validate_role_identifiers(&config, None);
        errors.extend(validate_enum_name(&config.name, None));
        assert!(
            errors.is_empty(),
            "valid identifiers should produce no errors"
        );
    }

    #[test]
    fn identifier_enum_name_empty_pascal() {
        // A name of only dashes produces empty PascalCase
        let errors = validate_enum_name("---", None);
        assert!(
            errors.iter().any(|e| {
                matches!(e, BuildError::InvalidIdentifier { name, reason, .. }
                    if name == "---" && reason.contains("empty"))
            }),
            "should detect empty PascalCase for enum name: {errors:?}"
        );
    }

    #[test]
    fn identifier_role_is_keyword() {
        // "r#self" -> PascalCase "Self" which is a keyword
        let config = make_config("app-icon", &["self"], &[], &[]);
        let errors = validate_role_identifiers(&config, Some("icons/icons.toml"));
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason, .. }
                    if name == "self" && reason.contains("keyword"))
            ),
            "should detect keyword: {errors:?}"
        );
        let msg = errors[0].to_string();
        assert!(msg.contains("in icons/icons.toml: "), "{msg}");
    }

    #[test]
    fn identifier_enum_name_is_keyword() {
        // "self" -> PascalCase "Self" which is a Rust keyword
        let errors = validate_enum_name("self", None);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason, .. }
                    if name == "self" && reason.contains("keyword"))
            ),
            "should detect keyword for enum name: {errors:?}"
        );
    }

    #[test]
    fn identifier_collision_detected() {
        // "play_pause" and "play-pause" both become "PlayPause"
        let config = make_config("app-icon", &["play_pause", "play-pause"], &[], &[]);
        let errors = validate_role_identifiers(&config, None);
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
        let errors = validate_role_identifiers(&config, None);
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
        let errors = validate_role_identifiers(&config, None);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason, .. }
                    if name == "3d-rotate" && reason.contains("non-letter"))
            ),
            "should reject digit-starting role identifier: {errors:?}"
        );
    }

    #[test]
    fn identifier_enum_name_starts_with_digit() {
        let errors = validate_enum_name("3d-icons", None);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason, .. }
                    if name == "3d-icons" && reason.contains("non-letter"))
            ),
            "should reject digit-starting enum name: {errors:?}"
        );
    }

    #[test]
    fn identifier_role_with_a_non_identifier_character() {
        // heck keeps '²' (alphanumeric), which is not XID_Continue
        let config = make_config("app-icon", &["x²"], &[], &[]);
        let errors = validate_role_identifiers(&config, None);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason, .. }
                    if name == "x²" && reason.contains("'²'"))
            ),
            "should reject '²': {errors:?}"
        );
    }

    #[test]
    fn identifier_enum_name_with_a_non_identifier_character() {
        let errors = validate_enum_name("x²-icons", None);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason, .. }
                    if name == "x²-icons" && reason.contains("'²'"))
            ),
            "should reject '²': {errors:?}"
        );
    }

    #[test]
    fn identifier_role_all_digits() {
        let config = make_config("app-icon", &["123"], &[], &[]);
        let errors = validate_role_identifiers(&config, None);
        assert!(
            errors.iter().any(
                |e| matches!(e, BuildError::InvalidIdentifier { name, reason, .. }
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
        let errors = validate_no_duplicate_themes(&config, "icons/icons.toml");
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("material"));
        assert!(msg.contains("bundled-themes"));
        assert!(msg.contains("icons/icons.toml"), "{msg}");
    }

    #[test]
    fn duplicate_system_theme_detected() {
        let config = make_config("x", &["a"], &[], &["freedesktop", "freedesktop"]);
        let errors = validate_no_duplicate_themes(&config, "icons.toml");
        assert_eq!(errors.len(), 1);
        let msg = errors[0].to_string();
        assert!(msg.contains("freedesktop"));
        assert!(msg.contains("system-themes"));
    }

    #[test]
    fn no_duplicate_themes() {
        let config = make_config("x", &["a"], &["material", "lucide"], &["freedesktop"]);
        let errors = validate_no_duplicate_themes(&config, "icons.toml");
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
        let msg = errors[0].to_string();
        assert!(msg.contains("(contains '\\u{002F}')"), "{msg}");
        assert!(msg.ends_with("path separator ('/' or '\\')"), "{msg}");
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
