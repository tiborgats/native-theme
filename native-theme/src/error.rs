// Error enum with Display and std::error::Error

use std::fmt;

/// Errors of the platform APIs: detection, watching and the platform readers.
#[derive(Debug)]
pub enum Error {
    /// The current platform is not supported.
    PlatformUnsupported {
        /// Platform identifier, `std::env::consts::OS` (e.g. `"freebsd"`).
        platform: &'static str,
    },

    /// Theme-change watching is not available: no backend for this desktop
    /// environment, or the platform's feature is disabled.
    WatchUnavailable {
        /// Why watching is unavailable.
        reason: &'static str,
    },

    /// A platform reader failed with a platform-specific error.
    ReaderFailed {
        /// Reader name (e.g. `"icon-theme"`, `"svg-rasterizer"`).
        reader: &'static str,
        /// The underlying error.
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PlatformUnsupported { platform } => {
                write!(f, "platform not supported: {platform}")
            }
            Error::WatchUnavailable { reason } => {
                write!(f, "theme watching unavailable: {reason}")
            }
            Error::ReaderFailed { reader, source } => {
                write!(f, "{reader} reader failed: {source}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ReaderFailed { source, .. } => Some(source.as_ref()),
            Error::PlatformUnsupported { .. } | Error::WatchUnavailable { .. } => None,
        }
    }
}

/// Declares the reader-name consts and, for the test, the list of them all.
macro_rules! reader_names {
    ($($(#[$meta:meta])* $name:ident = $value:literal;)*) => {
        $($(#[$meta])* pub(crate) const $name: &str = $value;)*

        /// Every name declared above, for the test that keeps them one style.
        #[cfg(test)]
        pub(crate) const ALL: &[&str] = &[$($value),*];
    };
}

/// The `reader` name of every [`Error::ReaderFailed`], lowercase kebab-case.
pub(crate) mod reader_names {
    reader_names! {
        ICON_THEME = "icon-theme";
        #[cfg_attr(not(feature = "svg-rasterize"), allow(dead_code))]
        SVG_RASTERIZER = "svg-rasterizer";
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn reader_names_are_lowercase_kebab_case_and_unique() {
        for name in reader_names::ALL {
            assert!(
                !name.is_empty()
                    && !name.starts_with('-')
                    && !name.ends_with('-')
                    && name
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{name:?} is not lowercase kebab-case"
            );
        }
        let mut sorted = reader_names::ALL.to_vec();
        sorted.sort_unstable();
        assert!(
            sorted.windows(2).all(|w| matches!(w, [a, b] if a != b)),
            "duplicate name"
        );
    }

    // === Send + Sync ===

    #[test]
    fn error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    // === Display tests ===

    #[test]
    fn display_names_each_variants_detail() {
        let unsupported = Error::PlatformUnsupported { platform: "wasm" };
        assert_eq!(unsupported.to_string(), "platform not supported: wasm");
        let unavailable = Error::WatchUnavailable {
            reason: "not compiled",
        };
        assert_eq!(
            unavailable.to_string(),
            "theme watching unavailable: not compiled"
        );
        let failed = Error::ReaderFailed {
            reader: reader_names::ICON_THEME,
            source: Box::new(std::io::Error::other("dbus down")),
        };
        assert_eq!(failed.to_string(), "icon-theme reader failed: dbus down");
    }

    // === source() tests ===

    #[test]
    fn source_some_for_reader_failed() {
        let err = Error::ReaderFailed {
            reader: reader_names::ICON_THEME,
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
}
