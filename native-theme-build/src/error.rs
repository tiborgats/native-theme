use std::fmt;

/// Build-time error for icon code generation.
///
/// Each variant provides structured context for actionable error messages
/// suitable for `cargo::error=` output.
pub(crate) enum BuildError {
    /// A role declared in the master TOML is missing from a theme mapping file.
    MissingRole {
        role: String,
        mapping_file: String,
    },
    /// An SVG file referenced by a bundled theme mapping does not exist.
    MissingSvg {
        path: String,
    },
    /// A role in a mapping file is not declared in the master TOML.
    UnknownRole {
        role: String,
        mapping_file: String,
    },
    /// A theme name does not match any known `IconSet` variant.
    UnknownTheme {
        theme: String,
    },
    /// A DE-aware mapping value is missing the required `default` key.
    MissingDefault {
        role: String,
        mapping_file: String,
    },
    /// A role name appears in multiple TOML files.
    DuplicateRole {
        role: String,
        file_a: String,
        file_b: String,
    },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRole { role, mapping_file } => {
                write!(f, "role \"{role}\" is missing from {mapping_file}")
            }
            Self::MissingSvg { path } => {
                write!(f, "SVG file not found: {path}")
            }
            Self::UnknownRole { role, mapping_file } => {
                write!(
                    f,
                    "unknown role \"{role}\" in {mapping_file} (not declared in master TOML)"
                )
            }
            Self::UnknownTheme { theme } => {
                write!(
                    f,
                    "unknown theme \"{theme}\" (expected one of: sf-symbols, segoe-fluent, freedesktop, material, lucide)"
                )
            }
            Self::MissingDefault { role, mapping_file } => {
                write!(
                    f,
                    "DE-aware mapping for \"{role}\" in {mapping_file} is missing the required \"default\" key"
                )
            }
            Self::DuplicateRole {
                role,
                file_a,
                file_b,
            } => {
                write!(
                    f,
                    "role \"{role}\" defined in both {file_a} and {file_b}"
                )
            }
        }
    }
}
