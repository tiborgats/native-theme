//! Keyed names: a platform's own colour role, variant, component, text style
//! or other name a preset declares.

use std::borrow::Cow;
use std::fmt;

use super::error::KeyError;

/// A keyed name, `[a-z0-9][a-z0-9_]*`.
///
/// Built from a runtime text by [`Key::new`], or from a literal by
/// [`key!`](crate::key), which checks the literal at compile time.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key(Cow<'static, str>);

impl Key {
    /// The key `name` names, or the first character outside the grammar.
    ///
    /// ```
    /// use native_theme::next::Key;
    ///
    /// assert_eq!(Key::new("shade_light").map(|k| k.to_string()), Ok("shade_light".to_owned()));
    /// assert_eq!(Key::new("Shade").map_err(|e| e.character), Err(Some('S')));
    /// ```
    pub fn new(name: &str) -> Result<Key, KeyError> {
        let mut chars = name.chars();
        let outside = match chars.next() {
            None => {
                return Err(KeyError {
                    text: String::new(),
                    character: None,
                });
            }
            Some(first) if is_first(first) => chars.find(|c| !is_rest(*c)),
            Some(first) => Some(first),
        };
        match outside {
            None => Ok(Key(Cow::Owned(name.to_owned()))),
            Some(character) => Err(KeyError {
                text: name.to_owned(),
                character: Some(character),
            }),
        }
    }

    /// The key a static text names, or `None` where it is not one; usable in
    /// a constant, as [`key!`](crate::key) uses it.
    #[must_use]
    pub const fn from_static(name: &'static str) -> Option<Key> {
        if is_key(name) {
            Some(Key(Cow::Borrowed(name)))
        } else {
            None
        }
    }

    /// The name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn is_first(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit()
}

fn is_rest(c: char) -> bool {
    is_first(c) || c == '_'
}

/// Whether `name` is in the key grammar, checked byte by byte so it runs at
/// compile time: a non-ASCII byte is outside the grammar, as its character is.
const fn is_key(name: &str) -> bool {
    let Some((first, mut rest)) = name.as_bytes().split_first() else {
        return false;
    };
    if !matches!(*first, b'a'..=b'z' | b'0'..=b'9') {
        return false;
    }
    while let Some((byte, tail)) = rest.split_first() {
        if !matches!(*byte, b'a'..=b'z' | b'0'..=b'9' | b'_') {
            return false;
        }
        rest = tail;
    }
    true
}

/// A [`Key`](crate::next::Key) from a literal, checked against the key
/// grammar `[a-z0-9][a-z0-9_]*` at compile time.
///
/// ```
/// use native_theme::key;
///
/// assert_eq!(key!("shade_light").as_str(), "shade_light");
/// assert_eq!(key!("128").as_str(), "128");
/// ```
///
/// A literal outside the grammar does not compile:
///
/// ```compile_fail,E0080
/// let _ = native_theme::key!("Shade");
/// ```
#[cfg_attr(not(nt_next_docs), doc(hidden))]
#[macro_export]
macro_rules! key {
    ($name:literal) => {{
        const KEY: $crate::next::Key =
            $crate::next::Key::from_static($name).expect(::core::concat!(
                "key!(",
                ::core::stringify!($name),
                "): not a key, `[a-z0-9][a-z0-9_]*`"
            ));
        KEY
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_grammar() {
        for name in ["a", "0", "128", "body", "shade_light", "a_", "x9_y"] {
            assert!(is_key(name), "{name}");
            assert_eq!(Key::new(name).map(|k| k.to_string()), Ok(name.to_owned()));
            assert_eq!(
                Key::from_static(name).map(|k| k.to_string()),
                Some(name.to_owned())
            );
        }
        for (name, character) in [
            ("", None),
            ("_a", Some('_')),
            ("Body", Some('B')),
            ("body-large", Some('-')),
            ("a.b", Some('.')),
            ("a b", Some(' ')),
            ("ab\u{e9}", Some('\u{e9}')),
            ("bodY", Some('Y')),
        ] {
            assert!(!is_key(name), "{name}");
            assert_eq!(
                Key::new(name),
                Err(KeyError {
                    text: name.to_owned(),
                    character,
                })
            );
            assert_eq!(Key::from_static(name), None);
        }
        assert_eq!(Ok(crate::key!("shade_light")), Key::new("shade_light"));
    }
}
