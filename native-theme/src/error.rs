// Error enum with Display, std::error::Error, and From conversions
//
// Option F: flat 9-variant Error + ErrorKind + RangeViolation

use std::fmt;

/// A range-violation error for a single theme property.
///
/// Produced during validation when a resolved numeric property falls outside
/// its allowed range.
#[derive(Debug, Clone)]
pub struct RangeViolation {
    /// Dot-separated path of the property (e.g. `"button.min_width"`).
    pub path: String,
    /// The actual value found.
    pub value: f64,
    /// Lower bound (inclusive), or `None` for open-ended.
    pub min: Option<f64>,
    /// Upper bound (inclusive), or `None` for open-ended.
    pub max: Option<f64>,
}

impl fmt::Display for RangeViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lo = self
            .min
            .map_or_else(|| "-inf".to_owned(), |v| v.to_string());
        let hi = self.max.map_or_else(|| "inf".to_owned(), |v| v.to_string());
        write!(
            f,
            "{} must be {}..={}, got {}",
            self.path, lo, hi, self.value
        )
    }
}

/// Coarse error category returned by [`Error::kind()`].
///
/// Follows the `std::io::ErrorKind` precedent: callers can match on the kind
/// for broad dispatch without inspecting each variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Platform feature not available or not supported.
    Platform,
    /// Parsing or lookup failure (TOML, preset name).
    Parse,
    /// Theme resolution failure (missing fields, range violations).
    Resolution,
    /// File I/O error.
    Io,
}

/// Errors that can occur when reading or processing theme data.
///
/// This is a flat enum with 9 variants. Use [`Error::kind()`] for coarse
/// dispatch without matching every variant.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A feature is compiled in but disabled at runtime or not applicable.
    FeatureDisabled {
        /// Feature name (e.g. `"kde"`, `"portal"`).
        name: &'static str,
        /// What the feature is needed for (e.g. `"KDE theme detection"`).
        needed_for: &'static str,
    },

    /// The current platform is not supported.
    PlatformUnsupported {
        /// Platform identifier (e.g. `"wasm"`, `"freebsd"`).
        platform: &'static str,
    },

    /// A preset name was not found in the bundled set.
    UnknownPreset {
        /// The requested preset name.
        name: String,
        /// Available preset names.
        known: &'static [&'static str],
    },

    /// File-system watching is not available.
    WatchUnavailable {
        /// Why watching is unavailable.
        reason: &'static str,
    },

    /// The theme has no variant for the requested color mode.
    ///
    /// Returned by [`Theme::pick_variant()`](crate::Theme::pick_variant) and
    /// [`Theme::into_variant()`](crate::Theme::into_variant) when the theme
    /// has neither a light nor a dark variant set.
    NoVariant {
        /// The color mode that was requested.
        mode: crate::theme::ColorMode,
    },

    /// TOML parsing or serialization error.
    Toml(toml::de::Error),

    /// File I/O error.
    Io(std::io::Error),

    /// Theme resolution found missing fields that could not be inherited.
    ResolutionIncomplete {
        /// Dot-separated paths of fields still `None` after resolution.
        missing: Vec<String>,
    },

    /// Theme resolution found numeric values outside allowed ranges.
    ResolutionInvalid {
        /// One entry per out-of-range property.
        errors: Vec<RangeViolation>,
    },

    /// A platform reader failed with a platform-specific error.
    ReaderFailed {
        /// Reader name (e.g. `"kde"`, `"gnome-portal"`, `"windows"`).
        reader: &'static str,
        /// The underlying error.
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl Error {
    /// Returns the coarse [`ErrorKind`] for this error.
    ///
    /// Useful for broad dispatch (e.g. "is this a platform problem or a
    /// parse problem?") without matching every variant.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::FeatureDisabled { .. } => ErrorKind::Platform,
            Error::PlatformUnsupported { .. } => ErrorKind::Platform,
            Error::WatchUnavailable { .. } => ErrorKind::Platform,
            Error::NoVariant { .. } => ErrorKind::Resolution,
            Error::ReaderFailed { .. } => ErrorKind::Platform,
            Error::UnknownPreset { .. } => ErrorKind::Parse,
            Error::Toml(_) => ErrorKind::Parse,
            Error::Io(_) => ErrorKind::Io,
            Error::ResolutionIncomplete { .. } => ErrorKind::Resolution,
            Error::ResolutionInvalid { .. } => ErrorKind::Resolution,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FeatureDisabled { name, needed_for } => {
                write!(f, "feature \"{name}\" is required for {needed_for}")
            }
            Error::PlatformUnsupported { platform } => {
                write!(f, "platform not supported: {platform}")
            }
            Error::UnknownPreset { name, known } => {
                write!(
                    f,
                    "unknown preset \"{name}\"; available: {}",
                    known.join(", ")
                )
            }
            Error::WatchUnavailable { reason } => {
                write!(f, "theme watching unavailable: {reason}")
            }
            Error::NoVariant { mode } => {
                write!(f, "theme has no variant for {mode:?}")
            }
            Error::Toml(err) => write!(f, "TOML error: {err}"),
            Error::Io(err) => write!(f, "I/O error: {err}"),
            Error::ResolutionIncomplete { missing } => {
                write!(
                    f,
                    "theme resolution failed: {} missing field(s):",
                    missing.len()
                )?;

                // Group fields by category, preserving insertion order within each group.
                let categories: &[&str] =
                    &["root defaults", "text scale", "widget fields", "icon set"];
                for &cat in categories {
                    let fields: Vec<&str> = missing
                        .iter()
                        .filter(|field| field_category(field) == cat)
                        .map(|s| s.as_str())
                        .collect();
                    if fields.is_empty() {
                        continue;
                    }
                    write!(f, "\n  [{cat}]")?;
                    for field in &fields {
                        write!(f, "\n    - {field}")?;
                    }
                }

                // Hint when root defaults are missing (the most common user mistake).
                let has_root = missing
                    .iter()
                    .any(|field| field_category(field) == "root defaults");
                if has_root {
                    write!(
                        f,
                        "\n  hint: root defaults drive widget inheritance; \
                         consider using Theme::preset(name) and then Theme::merge() to inherit from a complete preset"
                    )?;
                }

                Ok(())
            }
            Error::ResolutionInvalid { errors } => {
                write!(
                    f,
                    "theme resolution failed: {} range violation(s):",
                    errors.len()
                )?;
                for violation in errors {
                    write!(f, "\n  - {violation}")?;
                }
                Ok(())
            }
            Error::ReaderFailed { reader, source } => {
                write!(f, "{reader} reader failed: {source}")
            }
        }
    }
}

/// Categorize a field path into a human-readable group name.
fn field_category(field: &str) -> &'static str {
    match field.split('.').next() {
        Some("defaults") => "root defaults",
        Some("text_scale") => "text scale",
        _ => "widget fields",
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Toml(err) => Some(err),
            Error::Io(err) => Some(err),
            Error::ReaderFailed { source, .. } => Some(source.as_ref()),
            Error::FeatureDisabled { .. }
            | Error::PlatformUnsupported { .. }
            | Error::UnknownPreset { .. }
            | Error::WatchUnavailable { .. }
            | Error::NoVariant { .. }
            | Error::ResolutionIncomplete { .. }
            | Error::ResolutionInvalid { .. } => None,
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        // Serialization errors are stringified because Error::Toml wraps only
        // toml::de::Error. This preserves the existing From<toml::ser::Error>
        // conversion used by presets::to_toml().
        Error::ReaderFailed {
            reader: "toml-serializer",
            source: Box::new(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === ErrorKind dispatch tests ===

    #[test]
    fn kind_platform_for_feature_disabled() {
        let err = Error::FeatureDisabled {
            name: "kde",
            needed_for: "KDE theme detection",
        };
        assert_eq!(err.kind(), ErrorKind::Platform);
    }

    #[test]
    fn kind_platform_for_platform_unsupported() {
        let err = Error::PlatformUnsupported { platform: "wasm" };
        assert_eq!(err.kind(), ErrorKind::Platform);
    }

    #[test]
    fn kind_platform_for_watch_unavailable() {
        let err = Error::WatchUnavailable {
            reason: "notify crate not compiled",
        };
        assert_eq!(err.kind(), ErrorKind::Platform);
    }

    #[test]
    fn kind_platform_for_reader_failed() {
        let err = Error::ReaderFailed {
            reader: "kde",
            source: Box::new(std::io::Error::other("dbus down")),
        };
        assert_eq!(err.kind(), ErrorKind::Platform);
    }

    #[test]
    fn kind_parse_for_unknown_preset() {
        let err = Error::UnknownPreset {
            name: "foobar".into(),
            known: &["adwaita", "kde-breeze"],
        };
        assert_eq!(err.kind(), ErrorKind::Parse);
    }

    #[test]
    fn kind_parse_for_toml() {
        let toml_err: Result<toml::Value, toml::de::Error> = toml::from_str("=invalid");
        let err = Error::Toml(toml_err.unwrap_err());
        assert_eq!(err.kind(), ErrorKind::Parse);
    }

    #[test]
    fn kind_resolution_for_incomplete() {
        let err = Error::ResolutionIncomplete {
            missing: vec!["defaults.accent_color".into()],
        };
        assert_eq!(err.kind(), ErrorKind::Resolution);
    }

    #[test]
    fn kind_resolution_for_invalid() {
        let err = Error::ResolutionInvalid {
            errors: vec![RangeViolation {
                path: "button.min_width".into(),
                value: -5.0,
                min: Some(0.0),
                max: None,
            }],
        };
        assert_eq!(err.kind(), ErrorKind::Resolution);
    }

    #[test]
    fn kind_io_for_io() {
        let err = Error::Io(std::io::Error::other("disk failure"));
        assert_eq!(err.kind(), ErrorKind::Io);
    }

    // === Send + Sync ===

    #[test]
    fn error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    // === Display tests ===

    #[test]
    fn display_feature_disabled_includes_name_and_needed_for() {
        let err = Error::FeatureDisabled {
            name: "kde",
            needed_for: "KDE theme detection",
        };
        let msg = err.to_string();
        assert!(msg.contains("kde"), "got: {msg}");
        assert!(msg.contains("KDE theme detection"), "got: {msg}");
    }

    #[test]
    fn display_resolution_incomplete_categorizes_fields() {
        let err = Error::ResolutionIncomplete {
            missing: vec![
                "defaults.accent_color".into(),
                "button.font.color".into(),
                "window.border.corner_radius".into(),
            ],
        };
        let msg = err.to_string();
        assert!(msg.contains("3 missing field(s)"), "got: {msg}");
        assert!(msg.contains("[root defaults]"), "got: {msg}");
        assert!(msg.contains("defaults.accent_color"), "got: {msg}");
        assert!(msg.contains("[widget fields]"), "got: {msg}");
        assert!(msg.contains("button.font.color"), "got: {msg}");
        assert!(msg.contains("window.border.corner_radius"), "got: {msg}");
        assert!(msg.contains("hint:"), "got: {msg}");
        assert!(msg.contains("preset"), "got: {msg}");
    }

    #[test]
    fn display_resolution_incomplete_no_hint_without_root_defaults() {
        let err = Error::ResolutionIncomplete {
            missing: vec!["button.font.color".into()],
        };
        let msg = err.to_string();
        assert!(msg.contains("[widget fields]"), "got: {msg}");
        assert!(!msg.contains("hint:"), "got: {msg}");
    }

    #[test]
    fn display_resolution_incomplete_groups_text_scale() {
        let err = Error::ResolutionIncomplete {
            missing: vec!["text_scale.caption".into(), "defaults.font.family".into()],
        };
        let msg = err.to_string();
        assert!(msg.contains("[text scale]"), "got: {msg}");
        assert!(msg.contains("[root defaults]"), "got: {msg}");
    }

    #[test]
    fn display_resolution_incomplete_widget_category() {
        // icon_set is no longer validated per-variant; test a widget field instead
        let err = Error::ResolutionIncomplete {
            missing: vec!["button.font.color".into()],
        };
        let msg = err.to_string();
        assert!(msg.contains("[widget fields]"), "got: {msg}");
        assert!(!msg.contains("hint:"), "got: {msg}");
    }

    #[test]
    fn display_resolution_invalid_lists_violations() {
        let err = Error::ResolutionInvalid {
            errors: vec![
                RangeViolation {
                    path: "button.min_width".into(),
                    value: -5.0,
                    min: Some(0.0),
                    max: None,
                },
                RangeViolation {
                    path: "slider.track_height".into(),
                    value: 200.0,
                    min: Some(1.0),
                    max: Some(100.0),
                },
            ],
        };
        let msg = err.to_string();
        assert!(msg.contains("2 range violation(s)"), "got: {msg}");
        assert!(msg.contains("button.min_width"), "got: {msg}");
        assert!(msg.contains("-5"), "got: {msg}");
        assert!(msg.contains("slider.track_height"), "got: {msg}");
        assert!(msg.contains("200"), "got: {msg}");
    }

    #[test]
    fn display_unknown_preset_shows_name_and_available() {
        let err = Error::UnknownPreset {
            name: "foobar".into(),
            known: &["adwaita", "kde-breeze"],
        };
        let msg = err.to_string();
        assert!(msg.contains("foobar"), "got: {msg}");
        assert!(msg.contains("adwaita"), "got: {msg}");
        assert!(msg.contains("kde-breeze"), "got: {msg}");
    }

    // === source() tests ===

    #[test]
    fn source_some_for_io() {
        let err = Error::Io(std::io::Error::other("disk failure"));
        assert!(
            std::error::Error::source(&err).is_some(),
            "Io should return Some from source()"
        );
    }

    #[test]
    fn source_some_for_reader_failed() {
        let err = Error::ReaderFailed {
            reader: "kde",
            source: Box::new(std::io::Error::other("dbus down")),
        };
        let source = std::error::Error::source(&err);
        assert!(
            source.is_some(),
            "ReaderFailed should return Some from source()"
        );
        assert!(source.unwrap().to_string().contains("dbus down"));
    }

    #[test]
    fn source_some_for_toml() {
        let toml_err: Result<toml::Value, toml::de::Error> = toml::from_str("=invalid");
        let err = Error::Toml(toml_err.unwrap_err());
        assert!(
            std::error::Error::source(&err).is_some(),
            "Toml should return Some from source()"
        );
    }

    #[test]
    fn source_none_for_feature_disabled() {
        let err = Error::FeatureDisabled {
            name: "kde",
            needed_for: "detection",
        };
        assert!(std::error::Error::source(&err).is_none());
    }

    #[test]
    fn source_none_for_platform_unsupported() {
        let err = Error::PlatformUnsupported { platform: "wasm" };
        assert!(std::error::Error::source(&err).is_none());
    }

    #[test]
    fn source_none_for_watch_unavailable() {
        let err = Error::WatchUnavailable {
            reason: "not compiled",
        };
        assert!(std::error::Error::source(&err).is_none());
    }

    #[test]
    fn source_none_for_unknown_preset() {
        let err = Error::UnknownPreset {
            name: "x".into(),
            known: &[],
        };
        assert!(std::error::Error::source(&err).is_none());
    }

    #[test]
    fn source_none_for_resolution_incomplete() {
        let err = Error::ResolutionIncomplete {
            missing: vec!["a".into()],
        };
        assert!(std::error::Error::source(&err).is_none());
    }

    #[test]
    fn source_none_for_resolution_invalid() {
        let err = Error::ResolutionInvalid {
            errors: vec![RangeViolation {
                path: "x".into(),
                value: 0.0,
                min: None,
                max: None,
            }],
        };
        assert!(std::error::Error::source(&err).is_none());
    }

    // === From impls ===

    #[test]
    fn from_toml_de_error_produces_toml_variant() {
        let toml_err: Result<toml::Value, toml::de::Error> = toml::from_str("=invalid");
        let err: Error = toml_err.unwrap_err().into();
        match &err {
            Error::Toml(_) => {} // correct
            other => panic!("expected Toml variant, got: {other:?}"),
        }
    }

    #[test]
    fn from_io_error_produces_io_variant_no_arc() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing file");
        let err: Error = io_err.into();
        match &err {
            Error::Io(e) => {
                assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
                assert!(e.to_string().contains("missing file"));
            }
            other => panic!("expected Io variant, got: {other:?}"),
        }
    }

    // === Derive checks ===

    #[test]
    fn error_kind_derives() {
        // Debug
        let k = ErrorKind::Platform;
        let dbg = format!("{k:?}");
        assert!(dbg.contains("Platform"));

        // Clone + Copy
        let k2 = k;
        let k3 = k2;
        assert_eq!(k, k3);

        // PartialEq + Eq
        assert_eq!(ErrorKind::Parse, ErrorKind::Parse);
        assert_ne!(ErrorKind::Io, ErrorKind::Resolution);
    }

    #[test]
    fn range_violation_derives_debug_clone() {
        let v = RangeViolation {
            path: "x".into(),
            value: 1.0,
            min: Some(0.0),
            max: Some(10.0),
        };
        // Debug
        let dbg = format!("{v:?}");
        assert!(dbg.contains("RangeViolation"));

        // Clone
        let v2 = v.clone();
        assert_eq!(v2.path, "x");
        assert!((v2.value - 1.0).abs() < f64::EPSILON);
    }

    // === RangeViolation Display ===

    #[test]
    fn range_violation_display_both_bounds() {
        let v = RangeViolation {
            path: "button.min_width".into(),
            value: -5.0,
            min: Some(0.0),
            max: Some(1000.0),
        };
        let msg = v.to_string();
        assert!(msg.contains("button.min_width"), "got: {msg}");
        assert!(msg.contains("0..=1000"), "got: {msg}");
        assert!(msg.contains("-5"), "got: {msg}");
    }

    #[test]
    fn range_violation_display_open_max() {
        let v = RangeViolation {
            path: "x".into(),
            value: -1.0,
            min: Some(0.0),
            max: None,
        };
        let msg = v.to_string();
        assert!(msg.contains("0..=inf"), "got: {msg}");
    }

    #[test]
    fn range_violation_display_open_min() {
        let v = RangeViolation {
            path: "x".into(),
            value: 999.0,
            min: None,
            max: Some(100.0),
        };
        let msg = v.to_string();
        assert!(msg.contains("-inf..=100"), "got: {msg}");
    }
}
