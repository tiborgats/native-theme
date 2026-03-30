use serde::Deserialize;
use std::collections::BTreeMap;

/// Single-source table mapping theme TOML names to `IconSet` variant paths.
///
/// Every entry is `(toml_name, "IconSet::Variant")`.
/// All other modules derive their theme lists from this table.
pub(crate) const THEME_TABLE: &[(&str, &str)] = &[
    ("sf-symbols", "IconSet::SfSymbols"),
    ("segoe-fluent", "IconSet::SegoeIcons"),
    ("freedesktop", "IconSet::Freedesktop"),
    ("material", "IconSet::Material"),
    ("lucide", "IconSet::Lucide"),
];

/// Single-source table mapping lowercase DE keys to `LinuxDesktop` variant paths.
///
/// Every entry is `(toml_key, "LinuxDesktop::Variant")`.
/// The special `"default"` key is deliberately absent -- it is handled as a
/// wildcard in codegen and validation.
pub(crate) const DE_TABLE: &[(&str, &str)] = &[
    ("kde", "LinuxDesktop::Kde"),
    ("gnome", "LinuxDesktop::Gnome"),
    ("xfce", "LinuxDesktop::Xfce"),
    ("cinnamon", "LinuxDesktop::Cinnamon"),
    ("mate", "LinuxDesktop::Mate"),
    ("lxqt", "LinuxDesktop::LxQt"),
    ("budgie", "LinuxDesktop::Budgie"),
];

/// Check whether `name` is a known theme name (appears in `THEME_TABLE`).
pub(crate) fn is_known_theme(name: &str) -> bool {
    THEME_TABLE.iter().any(|(k, _)| *k == name)
}

/// Master TOML schema: the top-level icon definition file.
///
/// Example:
/// ```toml
/// name = "app-icon"
/// roles = ["play-pause", "skip-forward"]
/// bundled-themes = ["material"]
/// system-themes = ["sf-symbols"]
/// ```
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[doc(hidden)]
pub struct MasterConfig {
    /// Name for the generated enum (kebab-case, converted to PascalCase).
    pub name: String,
    /// List of role names (kebab-case).
    pub roles: Vec<String>,
    /// Themes with bundled SVGs (e.g., `["material", "lucide"]`).
    #[serde(default, rename = "bundled-themes")]
    pub bundled_themes: Vec<String>,
    /// Themes loaded by OS at runtime (e.g., `["freedesktop", "sf-symbols"]`).
    #[serde(default, rename = "system-themes")]
    pub system_themes: Vec<String>,
}

/// Per-theme mapping: maps role names to theme-specific icon names.
///
/// Deserialized from a flat TOML file like:
/// ```toml
/// play-pause = "play_pause"
/// bluetooth = { kde = "preferences-system-bluetooth", default = "bluetooth" }
/// ```
pub(crate) type ThemeMapping = BTreeMap<String, MappingValue>;

/// A mapping value is either a plain string or a DE-aware inline table.
///
/// - `Simple`: a direct icon name string.
/// - `DeAware`: a table mapping desktop environment names to icon names,
///   with a required `"default"` key.
#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum MappingValue {
    /// A plain icon name string (e.g., `"play_pause"`).
    Simple(String),
    /// A DE-aware mapping table (e.g., `{ kde = "media-playback-start", default = "play" }`).
    DeAware(BTreeMap<String, String>),
}

impl MappingValue {
    /// Returns the default icon name for this mapping value.
    ///
    /// For `Simple`, returns the string itself.
    /// For `DeAware`, returns the value of the `"default"` key, or `None` if missing.
    pub fn default_name(&self) -> Option<&str> {
        match self {
            Self::Simple(s) => Some(s),
            Self::DeAware(m) => m.get("default").map(|s| s.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // === THEME_TABLE consistency tests ===

    #[test]
    fn theme_table_no_duplicate_keys() {
        let mut seen = HashSet::new();
        for (key, _) in THEME_TABLE {
            assert!(
                seen.insert(*key),
                "duplicate theme key in THEME_TABLE: {key}"
            );
        }
    }

    #[test]
    fn theme_table_no_duplicate_variants() {
        let mut seen = HashSet::new();
        for (_, variant) in THEME_TABLE {
            assert!(
                seen.insert(*variant),
                "duplicate variant in THEME_TABLE: {variant}"
            );
        }
    }

    #[test]
    fn theme_table_variants_start_with_icon_set() {
        for (key, variant) in THEME_TABLE {
            assert!(
                variant.starts_with("IconSet::"),
                "THEME_TABLE entry for \"{key}\" should start with \"IconSet::\", got \"{variant}\""
            );
        }
    }

    #[test]
    fn theme_table_keys_are_nonempty() {
        for (key, _) in THEME_TABLE {
            assert!(!key.is_empty(), "THEME_TABLE has an empty key");
        }
    }

    // === DE_TABLE consistency tests ===

    #[test]
    fn de_table_no_duplicate_keys() {
        let mut seen = HashSet::new();
        for (key, _) in DE_TABLE {
            assert!(seen.insert(*key), "duplicate DE key in DE_TABLE: {key}");
        }
    }

    #[test]
    fn de_table_no_duplicate_variants() {
        let mut seen = HashSet::new();
        for (_, variant) in DE_TABLE {
            assert!(
                seen.insert(*variant),
                "duplicate variant in DE_TABLE: {variant}"
            );
        }
    }

    #[test]
    fn de_table_variants_start_with_linux_desktop() {
        for (key, variant) in DE_TABLE {
            assert!(
                variant.starts_with("LinuxDesktop::"),
                "DE_TABLE entry for \"{key}\" should start with \"LinuxDesktop::\", got \"{variant}\""
            );
        }
    }

    #[test]
    fn de_table_does_not_contain_default() {
        // "default" is handled specially, not in the table
        for (key, _) in DE_TABLE {
            assert_ne!(
                *key, "default",
                "DE_TABLE must not contain \"default\" -- it is handled as a wildcard"
            );
        }
    }

    #[test]
    fn de_table_keys_are_nonempty() {
        for (key, _) in DE_TABLE {
            assert!(!key.is_empty(), "DE_TABLE has an empty key");
        }
    }

    // === is_known_theme tests ===

    #[test]
    fn is_known_theme_returns_true_for_known() {
        assert!(is_known_theme("material"));
        assert!(is_known_theme("sf-symbols"));
    }

    #[test]
    fn is_known_theme_returns_false_for_unknown() {
        assert!(!is_known_theme("nonexistent"));
        assert!(!is_known_theme(""));
    }
}
