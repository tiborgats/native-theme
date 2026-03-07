// Error enum -- implemented in Task 3

/// Errors that can occur when reading or processing theme data.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Operation not supported on the current platform.
    Unsupported,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsupported => write!(f, "operation not supported on this platform"),
        }
    }
}

impl std::error::Error for Error {}
