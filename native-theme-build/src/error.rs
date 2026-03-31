use std::fmt;

use crate::schema::THEME_TABLE;

/// Build-time error for icon code generation.
///
/// Each variant provides structured context for actionable error messages
/// suitable for `cargo::error=` output.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum BuildError {
    /// A role declared in the master TOML is missing from a theme mapping file.
    MissingRole {
        /// The role name that is missing.
        role: String,
        /// Path to the mapping file where the role is expected.
        mapping_file: String,
    },
    /// An SVG file referenced by a bundled theme mapping does not exist.
    MissingSvg {
        /// Filesystem path to the missing SVG.
        path: String,
    },
    /// A role in a mapping file is not declared in the master TOML.
    UnknownRole {
        /// The unexpected role name found in the mapping.
        role: String,
        /// Path to the mapping file containing the unknown role.
        mapping_file: String,
    },
    /// A theme name does not match any known `IconSet` variant.
    UnknownTheme {
        /// The unrecognized theme name.
        theme: String,
    },
    /// A DE-aware mapping value is missing the required `default` key.
    MissingDefault {
        /// The role whose DE-aware mapping lacks a default.
        role: String,
        /// Path to the mapping file.
        mapping_file: String,
    },
    /// A role name appears in multiple TOML files.
    DuplicateRole {
        /// The duplicated role name.
        role: String,
        /// Path to the first file declaring the role.
        file_a: String,
        /// Path to the second file declaring the role.
        file_b: String,
    },
    /// A TOML or SVG file could not be read or parsed.
    Io {
        /// Human-readable description of the I/O error.
        message: String,
    },
    /// A role or enum name produces an invalid Rust identifier.
    InvalidIdentifier {
        /// The original name that failed validation.
        name: String,
        /// Why the identifier is invalid.
        reason: String,
    },
    /// Two different role names produce the same PascalCase enum variant.
    IdentifierCollision {
        /// The first role name.
        role_a: String,
        /// The second role name.
        role_b: String,
        /// The PascalCase variant they both produce.
        pascal: String,
    },
    /// A theme name appears in both `bundled_themes` and `system_themes`.
    ThemeOverlap {
        /// The overlapping theme name.
        theme: String,
    },
    /// A role name appears more than once in a single config file.
    DuplicateRoleInFile {
        /// The duplicated role name.
        role: String,
        /// Path to the file containing the duplicate.
        file: String,
    },
    /// A theme name appears more than once in the same list.
    DuplicateTheme {
        /// The duplicated theme name.
        theme: String,
        /// Which list contains the duplicate (`"bundled-themes"` or `"system-themes"`).
        list: String,
    },
    /// A mapping value contains characters that are invalid for an icon name.
    InvalidIconName {
        /// The offending icon name.
        name: String,
        /// The role it belongs to.
        role: String,
        /// Path to the mapping file.
        mapping_file: String,
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
                let expected: Vec<&str> = THEME_TABLE.iter().map(|(k, _)| *k).collect();
                let list = expected.join(", ");
                write!(f, "unknown theme \"{theme}\" (expected one of: {list})")
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
                write!(f, "role \"{role}\" defined in both {file_a} and {file_b}")
            }
            Self::Io { message } => {
                write!(f, "{message}")
            }
            Self::InvalidIdentifier { name, reason } => {
                write!(f, "invalid identifier \"{name}\": {reason}")
            }
            Self::IdentifierCollision {
                role_a,
                role_b,
                pascal,
            } => {
                write!(
                    f,
                    "roles \"{role_a}\" and \"{role_b}\" both produce the same PascalCase variant \"{pascal}\""
                )
            }
            Self::ThemeOverlap { theme } => {
                write!(
                    f,
                    "theme \"{theme}\" appears in both bundled-themes and system-themes"
                )
            }
            Self::DuplicateRoleInFile { role, file } => {
                write!(f, "role \"{role}\" appears more than once in {file}")
            }
            Self::DuplicateTheme { theme, list } => {
                write!(f, "theme \"{theme}\" appears more than once in {list}")
            }
            Self::InvalidIconName {
                name,
                role,
                mapping_file,
            } => {
                write!(
                    f,
                    "invalid icon name \"{name}\" for role \"{role}\" in {mapping_file}: \
                     names must be non-empty and free of control characters"
                )
            }
        }
    }
}

impl std::error::Error for BuildError {}

/// A collection of [`BuildError`]s from a failed build pipeline.
///
/// Wraps a `Vec<BuildError>` and provides [`emit_cargo_errors()`](Self::emit_cargo_errors)
/// for printing each error as a `cargo::error=` directive.
#[derive(Debug, Clone)]
pub struct BuildErrors(Vec<BuildError>);

impl fmt::Display for BuildErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} build error(s):", self.0.len())?;
        for e in &self.0 {
            write!(f, "\n  - {e}")?;
        }
        Ok(())
    }
}

impl std::error::Error for BuildErrors {}

impl BuildErrors {
    /// Create a `BuildErrors` from a `Vec<BuildError>`.
    pub(crate) fn new(errors: Vec<BuildError>) -> Self {
        Self(errors)
    }

    /// Create a single-error `BuildErrors` from an I/O error message.
    pub(crate) fn io(message: impl Into<String>) -> Self {
        Self(vec![BuildError::Io {
            message: message.into(),
        }])
    }

    /// Return a borrowed slice of the contained errors.
    pub fn errors(&self) -> &[BuildError] {
        &self.0
    }

    /// Consume this collection and return the inner `Vec<BuildError>`.
    pub fn into_errors(self) -> Vec<BuildError> {
        self.0
    }

    /// Returns `true` if there are no errors.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of errors.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Print each error as a `cargo::error=` directive to stdout.
    pub fn emit_cargo_errors(&self) {
        for e in &self.0 {
            println!("cargo::error={e}");
        }
    }
}

impl IntoIterator for BuildErrors {
    type Item = BuildError;
    type IntoIter = std::vec::IntoIter<BuildError>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a BuildErrors {
    type Item = &'a BuildError;
    type IntoIter = std::slice::Iter<'a, BuildError>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
