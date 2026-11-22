//! The model's error type.

use std::fmt;

/// Coarse category of an [`Error`], returned by [`Error::kind`], for broad
/// dispatch without matching every variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A platform reader or watcher is unavailable or failed, or a needed
    /// Cargo feature is off.
    Platform,
    /// Text that names or states something could not be read: a colour
    /// literal, a preset or palette name.
    Parse,
    /// A theme was read but its content is not valid.
    Validation,
    /// A file could not be read.
    Io,
}

/// An error of the theme model.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A Cargo feature the call needs is not enabled.
    FeatureDisabled {
        /// The feature's name.
        feature: &'static str,
        /// What the feature is needed for.
        needed_for: &'static str,
    },
    /// The current platform is not supported.
    PlatformUnsupported {
        /// The platform's identifier, as `std::env::consts::OS` gives it.
        platform: &'static str,
    },
    /// A platform reader failed.
    ReaderFailed {
        /// The reader's name.
        reader: &'static str,
        /// What failed.
        message: String,
    },
    /// Theme-change watching is not available.
    WatchUnavailable {
        /// Why watching is unavailable.
        reason: String,
    },
    /// No bundled preset has this name.
    UnknownPreset {
        /// The requested name.
        name: String,
    },
    /// No bundled palette has this name.
    UnknownPalette {
        /// The requested name.
        name: String,
    },
    /// The text is not one of the colour literal forms.
    InvalidColor {
        /// The text that was given.
        input: String,
    },
    /// A theme's content is not valid.
    InvalidTheme {
        /// Where in the theme the invalid value lies.
        path: String,
        /// What is wrong with it.
        reason: String,
    },
    /// A file could not be read.
    Io(std::io::Error),
}

/// `Result` with the model's [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// The coarse category of this error.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::FeatureDisabled { .. }
            | Error::PlatformUnsupported { .. }
            | Error::ReaderFailed { .. }
            | Error::WatchUnavailable { .. } => ErrorKind::Platform,
            Error::UnknownPreset { .. }
            | Error::UnknownPalette { .. }
            | Error::InvalidColor { .. } => ErrorKind::Parse,
            Error::InvalidTheme { .. } => ErrorKind::Validation,
            Error::Io(_) => ErrorKind::Io,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FeatureDisabled {
                feature,
                needed_for,
            } => write!(f, "feature \"{feature}\" is required for {needed_for}"),
            Error::PlatformUnsupported { platform } => {
                write!(f, "platform not supported: {platform}")
            }
            Error::ReaderFailed { reader, message } => {
                write!(f, "{reader} reader failed: {message}")
            }
            Error::WatchUnavailable { reason } => {
                write!(f, "theme watching unavailable: {reason}")
            }
            Error::UnknownPreset { name } => write!(f, "unknown preset \"{name}\""),
            Error::UnknownPalette { name } => write!(f, "unknown palette \"{name}\""),
            Error::InvalidColor { input } => write!(f, "not a colour: \"{input}\""),
            Error::InvalidTheme { path, reason } => {
                write!(f, "invalid theme value {path}: {reason}")
            }
            Error::Io(err) => write!(f, "I/O error: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_and_messages() {
        let color = Error::InvalidColor {
            input: "#zz".to_owned(),
        };
        assert_eq!(color.kind(), ErrorKind::Parse);
        assert_eq!(color.to_string(), "not a colour: \"#zz\"");
        let io = Error::from(std::io::Error::other("gone"));
        assert_eq!(io.kind(), ErrorKind::Io);
        assert!(std::error::Error::source(&io).is_some());
        let theme = Error::InvalidTheme {
            path: "button.face".to_owned(),
            reason: "negative".to_owned(),
        };
        assert_eq!(theme.kind(), ErrorKind::Validation);
        assert_eq!(
            Error::PlatformUnsupported { platform: "plan9" }.kind(),
            ErrorKind::Platform
        );
    }
}
