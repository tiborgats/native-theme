//! The error types.

use std::fmt;

use super::expr::ParseError;

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

/// Adding a layer, or reading one, failed (S1, S12).
#[derive(Clone, Debug, PartialEq)]
pub enum LayerError {
    /// Every problem found, not the first only.
    Invalid(Vec<Problem>),
}

impl fmt::Display for LayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayerError::Invalid(problems) => {
                write!(f, "the layer is invalid: {} problem(s)", problems.len())?;
                problems
                    .iter()
                    .try_for_each(|problem| write!(f, "\n  {problem}"))
            }
        }
    }
}

impl std::error::Error for LayerError {}

/// One problem a layer has: where, and what.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Problem {
    /// The model path, or the TOML key path where the layer was read from
    /// TOML (`widgets.button.body.fill`, `rules[2].when.states[0]`).
    pub path: String,
    /// The problem.
    pub kind: ProblemKind,
}

impl Problem {
    pub(crate) fn new(path: &str, kind: ProblemKind) -> Problem {
        Problem {
            path: path.to_owned(),
            kind,
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.kind)
        } else {
            write!(f, "{}: {}", self.path, self.kind)
        }
    }
}

/// What is wrong at a [`Problem`]'s path.
#[derive(Clone, Debug, PartialEq)]
pub enum ProblemKind {
    /// The text is not TOML; the parser's message.
    Toml(String),
    /// A key the model has no field, table or name for here: every unknown
    /// key is refused (S16).
    UnknownKey,
    /// A value of another kind than the place takes: what it takes.
    Mistyped {
        /// What the place takes (`a number`, `a table`).
        expected: &'static str,
    },
    /// A value naming nothing the model has here: an enum variant, a
    /// condition, a state flag, a widget, a part or a preset.
    UnknownName {
        /// The name as written.
        name: String,
    },
    /// A keyed name outside the key grammar.
    NotAKey(KeyError),
    /// An expression that does not parse (S2.2).
    Expression(ParseError),
    /// A field is missing (S2.2): left out of a block stated whole.
    Missing,
    /// A table states one field twice: two keys for one field (S11, S12).
    DuplicateKey {
        /// The field's other key, as written.
        other: String,
    },
    /// A stated number is not finite (S12).
    NotFinite,
    /// A `context`, `not_context`, `previous`, `next` or `drawn_as` table
    /// states `part` without `widget` (S11, S12).
    PartWithoutWidget,
    /// A part's patch sets a field its part does not have (S2.11, S12).
    NotAPartField {
        /// The part, `<widget>.<part>`.
        part: String,
    },
    /// `Append` in a layer's bases, where nothing lies beneath (S6.1, S12).
    AppendInBases,
}

impl fmt::Display for ProblemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProblemKind::Toml(message) => write!(f, "not TOML: {message}"),
            ProblemKind::UnknownKey => f.write_str("unknown key"),
            ProblemKind::Mistyped { expected } => write!(f, "expected {expected}"),
            ProblemKind::UnknownName { name } => write!(f, "unknown name {name:?}"),
            ProblemKind::NotAKey(error) => write!(f, "{error}"),
            ProblemKind::Expression(error) => write!(f, "expression: {error}"),
            ProblemKind::Missing => f.write_str("missing"),
            ProblemKind::DuplicateKey { other } => {
                write!(f, "states the field {other} states too")
            }
            ProblemKind::NotFinite => f.write_str("not a finite number"),
            ProblemKind::PartWithoutWidget => f.write_str("states `part` without `widget`"),
            ProblemKind::NotAPartField { part } => write!(f, "a field `{part}` does not have"),
            ProblemKind::AppendInBases => {
                f.write_str("`append` in a layer's bases, where nothing lies beneath")
            }
        }
    }
}
