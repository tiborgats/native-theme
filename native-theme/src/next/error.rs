//! The error types.

use std::fmt;

/// [`Key::new`](crate::next::Key::new)'s error: the text, and the first
/// character outside the key grammar `[a-z0-9][a-z0-9_]*`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct KeyError {
    /// The text that is not a key.
    pub text: String,
    /// The first character outside the grammar; `None` for the empty text.
    pub character: Option<char>,
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.character {
            Some(character) => write!(
                f,
                "{:?} is not a key: {character:?} is outside `[a-z0-9][a-z0-9_]*`",
                self.text
            ),
            None => f.write_str("the empty text is not a key"),
        }
    }
}

impl std::error::Error for KeyError {}
