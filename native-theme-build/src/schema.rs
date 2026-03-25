use serde::Deserialize;
use std::collections::BTreeMap;

/// The 5 known theme name strings matching `IconSet` kebab-case identifiers.
pub(crate) const KNOWN_THEMES: [&str; 5] = [
    "sf-symbols",
    "segoe-fluent",
    "freedesktop",
    "material",
    "lucide",
];

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
