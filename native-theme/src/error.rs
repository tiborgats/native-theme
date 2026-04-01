// Error enum with Display, std::error::Error, and From conversions

/// Error returned when theme resolution finds missing fields.
///
/// Contains a list of field paths (e.g., `"defaults.accent"`, `"button.font.family"`)
/// that were still `None` after resolution. Used by `validate()` to report exactly
/// which fields need to be supplied before a [`crate::model::ResolvedThemeVariant`] can be built.
#[derive(Debug, Clone)]
pub struct ThemeResolutionError {
    /// Dot-separated paths of fields that remained `None` after resolution.
    pub missing_fields: Vec<String>,
}

impl ThemeResolutionError {
    /// Categorize a field path into a human-readable group name.
    fn field_category(field: &str) -> &'static str {
        if field == "icon_set" {
            return "icon set";
        }
        match field.split('.').next() {
            Some("defaults") => "root defaults",
            Some("text_scale") => "text scale",
            _ => "widget fields",
        }
    }
}

impl std::fmt::Display for ThemeResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "theme resolution failed: {} missing field(s):",
            self.missing_fields.len()
        )?;

        // Group fields by category, preserving insertion order within each group.
        let categories: &[&str] = &["root defaults", "text scale", "widget fields", "icon set"];
        for &cat in categories {
            let fields: Vec<&str> = self
                .missing_fields
                .iter()
                .filter(|f| Self::field_category(f) == cat)
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
        let has_root = self
            .missing_fields
            .iter()
            .any(|f| Self::field_category(f) == "root defaults");
        if has_root {
            write!(
                f,
                "\n  hint: root defaults drive widget inheritance; \
                 consider using ThemeSpec::from_toml_with_base() to inherit from a complete preset"
            )?;
        }

        Ok(())
    }
}

impl std::error::Error for ThemeResolutionError {}

/// Errors that can occur when reading or processing theme data.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Operation not supported on the current platform.
    Unsupported(&'static str),

    /// Data source exists but cannot be read right now.
    Unavailable(String),

    /// TOML parsing or serialization error.
    Format(String),

    /// Wrapped platform-specific error.
    Platform(Box<dyn std::error::Error + Send + Sync>),

    /// File I/O error (preserves the original `std::io::Error`).
    Io(std::io::Error),

    /// Theme resolution/validation found missing fields.
    Resolution(ThemeResolutionError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsupported(reason) => write!(f, "not supported: {reason}"),
            Error::Unavailable(msg) => write!(f, "theme data unavailable: {msg}"),
            Error::Format(msg) => write!(f, "theme format error: {msg}"),
            Error::Platform(err) => write!(f, "platform error: {err}"),
            Error::Io(err) => write!(f, "I/O error: {err}"),
            Error::Resolution(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Platform(err) => Some(&**err),
            Error::Io(err) => Some(err),
            Error::Resolution(e) => Some(e),
            _ => None,
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Format(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::Format(err.to_string())
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

    #[test]
    fn unsupported_display() {
        let err = Error::Unsupported("KDE theme detection requires the `kde` feature");
        let msg = err.to_string();
        assert!(msg.contains("not supported"), "got: {msg}");
        assert!(msg.contains("kde"), "got: {msg}");
    }

    #[test]
    fn unavailable_display() {
        let err = Error::Unavailable("file not found".into());
        let msg = err.to_string();
        assert!(msg.contains("unavailable"), "got: {msg}");
        assert!(msg.contains("file not found"), "got: {msg}");
    }

    #[test]
    fn format_display() {
        let err = Error::Format("invalid TOML".into());
        let msg = err.to_string();
        assert!(msg.contains("format error"), "got: {msg}");
        assert!(msg.contains("invalid TOML"), "got: {msg}");
    }

    #[test]
    fn platform_display() {
        let inner = std::io::Error::other("dbus failure");
        let err = Error::Platform(Box::new(inner));
        let msg = err.to_string();
        assert!(msg.contains("platform error"), "got: {msg}");
        assert!(msg.contains("dbus failure"), "got: {msg}");
    }

    #[test]
    fn platform_source_returns_inner() {
        let inner = std::io::Error::other("inner error");
        let err = Error::Platform(Box::new(inner));
        let source = std::error::Error::source(&err);
        assert!(source.is_some());
        assert!(source.unwrap().to_string().contains("inner error"));
    }

    #[test]
    fn non_platform_source_is_none() {
        assert!(std::error::Error::source(&Error::Unsupported("test")).is_none());
        assert!(std::error::Error::source(&Error::Unavailable("x".into())).is_none());
        assert!(std::error::Error::source(&Error::Format("x".into())).is_none());
    }

    #[test]
    fn error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    #[test]
    fn from_toml_de_error() {
        // Create a toml deserialization error by parsing invalid TOML
        let toml_err: Result<toml::Value, toml::de::Error> = toml::from_str("=invalid");
        let err: Error = toml_err.unwrap_err().into();
        match &err {
            Error::Format(msg) => assert!(!msg.is_empty()),
            other => panic!("expected Format variant, got: {other:?}"),
        }
    }

    #[test]
    fn from_io_error() {
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

    // === ThemeResolutionError tests ===

    #[test]
    fn theme_resolution_error_construction() {
        let e = ThemeResolutionError {
            missing_fields: vec!["defaults.accent".into(), "button.font.family".into()],
        };
        assert_eq!(e.missing_fields.len(), 2);
        assert_eq!(e.missing_fields[0], "defaults.accent");
        assert_eq!(e.missing_fields[1], "button.font.family");
    }

    #[test]
    fn theme_resolution_error_display_categorizes_fields() {
        let e = ThemeResolutionError {
            missing_fields: vec![
                "defaults.accent".into(),
                "button.foreground".into(),
                "window.radius".into(),
            ],
        };
        let msg = e.to_string();
        assert!(msg.contains("3 missing field(s)"), "got: {msg}");
        assert!(msg.contains("[root defaults]"), "got: {msg}");
        assert!(msg.contains("defaults.accent"), "got: {msg}");
        assert!(msg.contains("[widget fields]"), "got: {msg}");
        assert!(msg.contains("button.foreground"), "got: {msg}");
        assert!(msg.contains("window.radius"), "got: {msg}");
        assert!(msg.contains("hint:"), "got: {msg}");
        assert!(msg.contains("from_toml_with_base"), "got: {msg}");
    }

    #[test]
    fn theme_resolution_error_display_no_hint_without_root_defaults() {
        let e = ThemeResolutionError {
            missing_fields: vec!["button.foreground".into()],
        };
        let msg = e.to_string();
        assert!(msg.contains("[widget fields]"), "got: {msg}");
        assert!(!msg.contains("hint:"), "got: {msg}");
    }

    #[test]
    fn theme_resolution_error_display_groups_text_scale() {
        let e = ThemeResolutionError {
            missing_fields: vec!["text_scale.caption".into(), "defaults.font.family".into()],
        };
        let msg = e.to_string();
        assert!(msg.contains("[text scale]"), "got: {msg}");
        assert!(msg.contains("[root defaults]"), "got: {msg}");
    }

    #[test]
    fn theme_resolution_error_display_icon_set_category() {
        let e = ThemeResolutionError {
            missing_fields: vec!["icon_set".into()],
        };
        let msg = e.to_string();
        assert!(msg.contains("[icon set]"), "got: {msg}");
        assert!(!msg.contains("hint:"), "got: {msg}");
    }

    #[test]
    fn theme_resolution_error_implements_std_error() {
        let e = ThemeResolutionError {
            missing_fields: vec!["defaults.accent".into()],
        };
        // This compiles only if ThemeResolutionError: std::error::Error
        let _: &dyn std::error::Error = &e;
    }

    #[test]
    fn theme_resolution_error_is_clone() {
        let e = ThemeResolutionError {
            missing_fields: vec!["defaults.accent".into()],
        };
        let e2 = e.clone();
        assert_eq!(e.missing_fields, e2.missing_fields);
    }

    // === Error::Resolution variant tests ===

    #[test]
    fn error_resolution_variant_wraps_theme_resolution_error() {
        let inner = ThemeResolutionError {
            missing_fields: vec!["defaults.accent".into()],
        };
        let err = Error::Resolution(inner);
        match &err {
            Error::Resolution(e) => assert_eq!(e.missing_fields.len(), 1),
            other => panic!("expected Resolution variant, got: {other:?}"),
        }
    }

    #[test]
    fn error_resolution_display_delegates_to_inner() {
        let inner = ThemeResolutionError {
            missing_fields: vec!["defaults.accent".into(), "window.radius".into()],
        };
        let err = Error::Resolution(inner);
        let msg = err.to_string();
        assert!(msg.contains("2 missing field(s)"), "got: {msg}");
        assert!(msg.contains("defaults.accent"), "got: {msg}");
        assert!(msg.contains("window.radius"), "got: {msg}");
    }

    #[test]
    fn error_resolution_source_returns_inner() {
        let inner = ThemeResolutionError {
            missing_fields: vec!["defaults.accent".into()],
        };
        let err = Error::Resolution(inner);
        let source = std::error::Error::source(&err);
        assert!(
            source.is_some(),
            "source() should return Some for Resolution variant"
        );
        let source_msg = source.unwrap().to_string();
        assert!(
            source_msg.contains("1 missing field(s)"),
            "got: {source_msg}"
        );
    }
}
