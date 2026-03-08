// Error enum with Display, std::error::Error, and From conversions

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
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsupported => write!(f, "operation not supported on this platform"),
            Error::Unavailable(msg) => write!(f, "theme data unavailable: {msg}"),
            Error::Format(msg) => write!(f, "theme format error: {msg}"),
            Error::Platform(err) => write!(f, "platform error: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Platform(err) => Some(&**err),
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
        let inner = std::io::Error::new(std::io::ErrorKind::Other, "dbus failure");
        let err = Error::Platform(Box::new(inner));
        let msg = err.to_string();
        assert!(msg.contains("platform error"), "got: {msg}");
        assert!(msg.contains("dbus failure"), "got: {msg}");
    }

    #[test]
    fn platform_source_returns_inner() {
        let inner = std::io::Error::new(std::io::ErrorKind::Other, "inner error");
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
}
