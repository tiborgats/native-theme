// Error enum with Display, std::error::Error, and From conversions

/// Error returned when theme resolution finds missing fields.
///
/// Contains a list of field paths (e.g., `"defaults.accent"`, `"button.font.family"`)
/// that were still `None` after resolution. Used by `validate()` to report exactly
/// which fields need to be supplied before a [`crate::model::ResolvedTheme`] can be built.
#[derive(Debug, Clone)]
pub struct ThemeResolutionError {
    /// Dot-separated paths of fields that remained `None` after resolution.
    pub missing_fields: Vec<String>,
}

impl std::fmt::Display for ThemeResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "theme resolution failed: {} missing field(s):",
            self.missing_fields.len()
        )?;
        for field in &self.missing_fields {
            write!(f, "\n  - {field}")?;
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
    Unsupported,

    /// Data source exists but cannot be read right now.
    Unavailable(String),

    /// TOML parsing or serialization error.
    Format(String),

    /// Wrapped platform-specific error.
    Platform(Box<dyn std::error::Error + Send + Sync>),

    /// Theme resolution/validation found missing fields.
    Resolution(ThemeResolutionError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsupported => write!(f, "operation not supported on this platform"),
            Error::Unavailable(msg) => write!(f, "theme data unavailable: {msg}"),
            Error::Format(msg) => write!(f, "theme format error: {msg}"),
            Error::Platform(err) => write!(f, "platform error: {err}"),
            Error::Resolution(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Platform(err) => Some(&**err),
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
        Error::Unavailable(err.to_string())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_display() {
        let err = Error::Unsupported;
        let msg = err.to_string();
        assert!(msg.contains("not supported"), "got: {msg}");
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
        assert!(std::error::Error::source(&Error::Unsupported).is_none());
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
            Error::Unavailable(msg) => assert!(msg.contains("missing file")),
            other => panic!("expected Unavailable variant, got: {other:?}"),
        }
    }

    // === ThemeResolutionError tests ===

    #[test]
    fn theme_resolution_error_construction() {
        let e = ThemeResolutionError {
            missing_fields: vec![
                "defaults.accent".into(),
                "button.font.family".into(),
            ],
        };
        assert_eq!(e.missing_fields.len(), 2);
        assert_eq!(e.missing_fields[0], "defaults.accent");
        assert_eq!(e.missing_fields[1], "button.font.family");
    }

    #[test]
    fn theme_resolution_error_display_lists_count_and_fields() {
        let e = ThemeResolutionError {
            missing_fields: vec![
                "defaults.accent".into(),
                "button.foreground".into(),
                "window.radius".into(),
            ],
        };
        let msg = e.to_string();
        assert!(msg.contains("3 missing field(s)"), "got: {msg}");
        assert!(msg.contains("defaults.accent"), "got: {msg}");
        assert!(msg.contains("button.foreground"), "got: {msg}");
        assert!(msg.contains("window.radius"), "got: {msg}");
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
        assert!(source.is_some(), "source() should return Some for Resolution variant");
        let source_msg = source.unwrap().to_string();
        assert!(source_msg.contains("1 missing field(s)"), "got: {source_msg}");
    }
}
