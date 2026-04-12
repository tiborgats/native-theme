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
pub(crate) struct MasterConfig {
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

    /// Returns all icon names referenced by this mapping value.
    ///
    /// For `Simple`, returns a single-element vec.
    /// For `DeAware`, returns all values (default + DE-specific).
    pub fn all_names(&self) -> Vec<&str> {
        match self {
            Self::Simple(s) => vec![s.as_str()],
            Self::DeAware(m) => m.values().map(|s| s.as_str()).collect(),
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

    // === Drift detection: THEME_TABLE vs native_theme::theme::IconSet ===

    /// Verify every THEME_TABLE entry resolves to a real IconSet variant
    /// and that the variant path string matches the Debug representation.
    #[test]
    fn theme_table_entries_match_icon_set_variants() {
        for (toml_key, variant_str) in THEME_TABLE {
            let icon_set = native_theme::theme::IconSet::from_name(toml_key).unwrap_or_else(|| {
                panic!(
                    "THEME_TABLE key \"{toml_key}\" does not match any IconSet variant \
                     (IconSet::from_name returned None)"
                )
            });
            let expected = format!("IconSet::{icon_set:?}");
            assert_eq!(
                *variant_str, expected,
                "THEME_TABLE variant string for \"{toml_key}\" is \"{variant_str}\", \
                 but IconSet::from_name produces {expected}"
            );
        }
    }

    /// Verify every IconSet variant has a corresponding THEME_TABLE entry.
    ///
    /// Uses the known set of all IconSet variants. If a new variant is added
    /// to IconSet without updating this list, this test fails -- prompting
    /// the developer to update both THEME_TABLE and this test.
    #[test]
    fn theme_table_covers_all_icon_set_variants() {
        let all_variants: &[native_theme::theme::IconSet] = &[
            native_theme::theme::IconSet::SfSymbols,
            native_theme::theme::IconSet::SegoeIcons,
            native_theme::theme::IconSet::Freedesktop,
            native_theme::theme::IconSet::Material,
            native_theme::theme::IconSet::Lucide,
        ];

        assert_eq!(
            THEME_TABLE.len(),
            all_variants.len(),
            "THEME_TABLE has {} entries but there are {} IconSet variants -- \
             a variant was added or removed without updating the table",
            THEME_TABLE.len(),
            all_variants.len(),
        );

        for variant in all_variants {
            let name = variant.name();
            assert!(
                THEME_TABLE.iter().any(|(k, _)| *k == name),
                "IconSet variant {:?} (name=\"{name}\") has no THEME_TABLE entry",
                variant,
            );
        }
    }

    // === Drift detection: DE_TABLE vs native_theme::LinuxDesktop ===

    /// Verify every DE_TABLE entry resolves to a real LinuxDesktop variant
    /// and that the variant path string matches the Debug representation.
    #[cfg(target_os = "linux")]
    #[test]
    fn de_table_entries_match_linux_desktop_variants() {
        use native_theme::detect::{LinuxDesktop, detect_linux_de};

        // Map from TOML key to the XDG_CURRENT_DESKTOP value that produces
        // each LinuxDesktop variant.
        let key_to_xdg: &[(&str, &str)] = &[
            ("kde", "KDE"),
            ("gnome", "GNOME"),
            ("xfce", "XFCE"),
            ("cinnamon", "Cinnamon"),
            ("mate", "MATE"),
            ("lxqt", "LXQt"),
            ("budgie", "Budgie:GNOME"),
        ];

        for (toml_key, variant_str) in DE_TABLE {
            let xdg = key_to_xdg
                .iter()
                .find(|(k, _)| *k == *toml_key)
                .unwrap_or_else(|| {
                    panic!(
                        "DE_TABLE key \"{toml_key}\" has no XDG mapping in the drift test -- \
                     update key_to_xdg"
                    )
                });

            let desktop = detect_linux_de(xdg.1);
            assert_ne!(
                desktop,
                LinuxDesktop::Unknown,
                "DE_TABLE key \"{toml_key}\" (XDG=\"{}\") resolves to LinuxDesktop::Unknown",
                xdg.1,
            );

            let expected = format!("LinuxDesktop::{desktop:?}");
            assert_eq!(
                *variant_str, expected,
                "DE_TABLE variant string for \"{toml_key}\" is \"{variant_str}\", \
                 but detect_linux_de produces {expected}"
            );
        }
    }

    /// Verify every non-Unknown LinuxDesktop variant has a DE_TABLE entry.
    #[cfg(target_os = "linux")]
    #[test]
    fn de_table_covers_all_linux_desktop_variants() {
        use native_theme::detect::LinuxDesktop;

        // All non-Unknown variants. If a new variant is added to LinuxDesktop,
        // update this list AND DE_TABLE.
        let all_variants: &[(&str, LinuxDesktop)] = &[
            ("kde", LinuxDesktop::Kde),
            ("gnome", LinuxDesktop::Gnome),
            ("xfce", LinuxDesktop::Xfce),
            ("cinnamon", LinuxDesktop::Cinnamon),
            ("mate", LinuxDesktop::Mate),
            ("lxqt", LinuxDesktop::LxQt),
            ("budgie", LinuxDesktop::Budgie),
        ];

        assert_eq!(
            DE_TABLE.len(),
            all_variants.len(),
            "DE_TABLE has {} entries but there are {} non-Unknown LinuxDesktop variants -- \
             a variant was added or removed without updating the table",
            DE_TABLE.len(),
            all_variants.len(),
        );

        for (expected_key, variant) in all_variants {
            assert!(
                DE_TABLE.iter().any(|(k, _)| k == expected_key),
                "LinuxDesktop::{variant:?} (key=\"{expected_key}\") has no DE_TABLE entry",
            );
        }
    }
}
