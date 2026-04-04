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
        /// The source file where the unknown theme was found, if known.
        source_file: Option<String>,
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
    /// A file could not be read from disk.
    IoRead {
        /// Filesystem path that could not be read.
        path: String,
        /// The underlying OS error message.
        reason: String,
    },
    /// A file's contents could not be parsed (TOML syntax error, etc.).
    IoParse {
        /// Filesystem path that failed to parse.
        path: String,
        /// The parse error message.
        reason: String,
    },
    /// A required environment variable is missing or invalid.
    IoEnv {
        /// Name of the environment variable (e.g. `CARGO_MANIFEST_DIR`).
        var: String,
        /// The underlying error message.
        reason: String,
    },
    /// An I/O-related error that does not fit the read/parse/env categories.
    IoOther {
        /// Human-readable description of the error.
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
        /// The source file where the collision was detected, if known.
        source_file: Option<String>,
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
        /// The first offending character, if identified.
        offending: Option<char>,
    },
    /// A bundled theme has a DE-aware mapping, creating a semantic mismatch
    /// between `icon_name()` (which returns a DE-specific name) and `icon_svg()`
    /// (which can only embed the default SVG).
    BundledDeAware {
        /// The bundled theme name.
        theme: String,
        /// The role with the DE-aware mapping.
        role: String,
    },
    /// The `crate_path` value is not a valid Rust path.
    InvalidCratePath {
        /// The invalid crate path.
        path: String,
        /// Why it is invalid.
        reason: String,
    },
    /// A `derive` value is not a valid Rust path.
    InvalidDerive {
        /// The invalid derive name.
        name: String,
        /// Why it is invalid.
        reason: String,
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
            Self::UnknownTheme { theme, source_file } => {
                let expected: Vec<&str> = THEME_TABLE.iter().map(|(k, _)| *k).collect();
                let list = expected.join(", ");
                write!(f, "unknown theme \"{theme}\" (expected one of: {list})")?;
                if let Some(file) = source_file {
                    write!(f, " in {file}")?;
                }
                Ok(())
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
            Self::IoRead { path, reason } => {
                write!(f, "failed to read {path}: {reason}")
            }
            Self::IoParse { path, reason } => {
                write!(f, "failed to parse {path}: {reason}")
            }
            Self::IoEnv { var, reason } => {
                write!(f, "environment variable {var} not available: {reason}")
            }
            Self::IoOther { message } => {
                write!(f, "{message}")
            }
            Self::InvalidIdentifier { name, reason } => {
                write!(f, "invalid identifier \"{name}\": {reason}")
            }
            Self::IdentifierCollision {
                role_a,
                role_b,
                pascal,
                source_file,
            } => {
                write!(
                    f,
                    "roles \"{role_a}\" and \"{role_b}\" both produce the same PascalCase variant \"{pascal}\""
                )?;
                if let Some(file) = source_file {
                    write!(f, " (in {file})")?;
                }
                Ok(())
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
                offending,
            } => {
                write!(
                    f,
                    "invalid icon name \"{name}\" for role \"{role}\" in {mapping_file}"
                )?;
                if let Some(ch) = offending {
                    write!(f, " (contains '\\u{{{:04X}}}')", *ch as u32)?;
                }
                write!(
                    f,
                    ": names must be non-empty and free of control characters"
                )
            }
            Self::BundledDeAware { theme, role } => {
                write!(
                    f,
                    "bundled theme \"{theme}\" has DE-aware mapping for role \"{role}\": \
                     bundled themes can only embed one SVG per role, but DE-aware mappings \
                     declare multiple icon names. Use a system theme for DE-aware icons"
                )
            }
            Self::InvalidCratePath { path, reason } => {
                write!(f, "invalid crate_path \"{path}\": {reason}")
            }
            Self::InvalidDerive { name, reason } => {
                write!(f, "invalid derive \"{name}\": {reason}")
            }
        }
    }
}

impl std::error::Error for BuildError {}

/// A collection of [`BuildError`]s from a failed build pipeline.
///
/// Wraps a `Vec<BuildError>` and provides [`emit_cargo_errors()`](Self::emit_cargo_errors)
/// for printing each error as a `cargo::error=` directive. Also carries
/// `rerun_paths` so callers can emit `cargo::rerun-if-changed` directives
/// even on failure.
#[derive(Debug, Clone)]
pub struct BuildErrors {
    errors: Vec<BuildError>,
    /// Paths that cargo should watch for changes, even when the build fails.
    pub rerun_paths: Vec<std::path::PathBuf>,
}

impl fmt::Display for BuildErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} build error(s):", self.errors.len())?;
        for e in &self.errors {
            write!(f, "\n  - {e}")?;
        }
        Ok(())
    }
}

impl std::error::Error for BuildErrors {}

impl BuildErrors {
    /// Create a `BuildErrors` from a `Vec<BuildError>`.
    pub(crate) fn new(errors: Vec<BuildError>) -> Self {
        debug_assert!(
            !errors.is_empty(),
            "BuildErrors created with empty error list"
        );
        Self {
            errors,
            rerun_paths: Vec::new(),
        }
    }

    /// Create a `BuildErrors` with rerun paths.
    pub(crate) fn with_rerun_paths(
        errors: Vec<BuildError>,
        rerun_paths: Vec<std::path::PathBuf>,
    ) -> Self {
        Self {
            errors,
            rerun_paths,
        }
    }

    /// Create a single-error `BuildErrors` from a file read failure.
    pub(crate) fn io_read(path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            errors: vec![BuildError::IoRead {
                path: path.into(),
                reason: reason.into(),
            }],
            rerun_paths: Vec::new(),
        }
    }

    /// Create a single-error `BuildErrors` from a file parse failure.
    pub(crate) fn io_parse(path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            errors: vec![BuildError::IoParse {
                path: path.into(),
                reason: reason.into(),
            }],
            rerun_paths: Vec::new(),
        }
    }

    /// Create a single-error `BuildErrors` from a missing environment variable.
    pub(crate) fn io_env(var: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            errors: vec![BuildError::IoEnv {
                var: var.into(),
                reason: reason.into(),
            }],
            rerun_paths: Vec::new(),
        }
    }

    /// Create a single-error `BuildErrors` from an I/O error that doesn't
    /// fit read/parse/env categories.
    pub(crate) fn io_other(message: impl Into<String>) -> Self {
        Self {
            errors: vec![BuildError::IoOther {
                message: message.into(),
            }],
            rerun_paths: Vec::new(),
        }
    }

    /// Return a borrowed slice of the contained errors.
    pub fn errors(&self) -> &[BuildError] {
        &self.errors
    }

    /// Consume this collection and return the inner `Vec<BuildError>`.
    pub fn into_errors(self) -> Vec<BuildError> {
        self.errors
    }

    /// Returns `true` if there are no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Print each error as a `cargo::error=` directive to stdout.
    ///
    /// Also prints `cargo::rerun-if-changed` for all tracked paths so cargo
    /// re-checks when the user fixes the files.
    pub fn emit_cargo_errors(&self) {
        for path in &self.rerun_paths {
            println!("cargo::rerun-if-changed={}", path.display());
        }
        for e in &self.errors {
            println!("cargo::error={e}");
        }
    }
}

impl IntoIterator for BuildErrors {
    type Item = BuildError;
    type IntoIter = std::vec::IntoIter<BuildError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a> IntoIterator for &'a BuildErrors {
    type Item = &'a BuildError;
    type IntoIter = std::slice::Iter<'a, BuildError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_errors_is_empty_and_len() {
        let errors = BuildErrors::new(vec![BuildError::IoOther {
            message: "test".into(),
        }]);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn build_errors_len_multiple() {
        let errors = BuildErrors::new(vec![
            BuildError::IoRead {
                path: "file.toml".into(),
                reason: "first".into(),
            },
            BuildError::IoParse {
                path: "file.toml".into(),
                reason: "second".into(),
            },
        ]);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn build_errors_display_shows_count() {
        let errors = BuildErrors::new(vec![BuildError::IoOther {
            message: "oops".into(),
        }]);
        let display = format!("{errors}");
        assert!(display.contains("1 build error(s)"));
        assert!(display.contains("oops"));
    }

    #[test]
    fn build_errors_into_iter() {
        let errors = BuildErrors::new(vec![BuildError::IoOther {
            message: "iter".into(),
        }]);
        let collected: Vec<BuildError> = errors.into_iter().collect();
        assert_eq!(collected.len(), 1);
    }

    #[test]
    fn build_errors_ref_iter() {
        let errors = BuildErrors::new(vec![BuildError::IoOther {
            message: "ref".into(),
        }]);
        let collected: Vec<&BuildError> = (&errors).into_iter().collect();
        assert_eq!(collected.len(), 1);
    }
}
