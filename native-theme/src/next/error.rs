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
    /// `Absent` where neither the core's marks nor the catalogue's allow it
    /// (S6.3, S12); a widget's root part made absent as a whole included.
    AbsentNotAllowed,
    /// `Auto` on a field marked `auto` neither by the catalogue for that
    /// part nor by the core (S6.3, S12).
    AutoNotAllowed,
    /// A call stated on a `Value<bool>`, `Value<Key>`, `Value<String>` or
    /// enum field, which takes a literal or a reference (S12).
    CallNotAllowed,
    /// A stated number outside its type's range: what the place takes (S12).
    OutOfRange {
        /// The range or the condition the place takes (`0..=1`, `above
        /// zero`).
        expected: &'static str,
    },
    /// A paint or a `Shadow::Layers` with no layer (S2.3, S2.5, S12).
    NoLayer,
    /// A paint whose bottom layer is a `Mask`, which masks nothing (S2.3,
    /// S12).
    MaskAtBottom,
    /// A gradient with fewer than two stops (S2.3, S12).
    TooFewStops,
    /// A `Transition` with an empty `properties` list: it would animate
    /// nothing (S2.7, S12).
    NoProperties,
    /// A `ThemeChange::Crossfade` or `Suspend` stating a zero duration,
    /// which is `Instant` (S4.3, S12).
    ZeroDuration,
    /// A loop's keyframes' `at` do not increase, or repeat a value (S2.7,
    /// S12).
    KeyframesNotIncreasing,
    /// A keyframe of a loop stated as a widget's or a component's value
    /// leaves its `part` out: such a loop is on no part (S2.7, S12).
    KeyframeWithoutPart,
    /// `Radius::FollowPart` outside a `FocusIndicator.corners` (S2.4, S12).
    FollowPartOutsideFocus,
    /// An axis or feature duplicating a field's route (`wght` in
    /// `Font.axes`, S2.8, S12).
    RouteDuplicated {
        /// The tag.
        tag: String,
    },
    /// `kde_accent` called with group `disabled`, or with a set that is not
    /// one of KDE's seven colour sets (S2.2, S12).
    KdeAccentArgument {
        /// The argument as written.
        name: String,
    },
    /// `pick` with no colours (S2.2, S12).
    PickWithoutColors,
    /// An expression nesting deeper than 32 (S2.2, S12).
    TooDeep,
    /// `base` in a layer's bases, where nothing lies beneath (S2.2, S12).
    BaseInBases,
    /// A token or colour-role expression naming an operand only an element
    /// has (S3.3, S12).
    ElementOperand {
        /// The operand as written.
        operand: String,
    },
    /// A colour-role definition naming `surface.`: a set's roles name
    /// `role.<set>.<role>` (S3.3, S12).
    SurfaceInRoles,
    /// A `widget.<part>` reference naming a part of another element than
    /// the one its part is queried with (S2.2, S12).
    OtherElementsPart {
        /// The part named.
        part: String,
    },
    /// An expression naming element data its part does not declare in the
    /// preset's `[declare]` (S10, S12).
    UndeclaredData {
        /// The operand as written.
        datum: String,
    },
    /// A name the preset does not declare (S10, S12).
    Undeclared {
        /// The name as written.
        name: String,
        /// The declaration that does not list it (`color_roles`,
        /// `widget.button.variants`).
        declaration: String,
    },
    /// A `Colors` or `Tokens` rule with selector parts that are not
    /// conditions (S7, S12).
    NotConditions,
    /// A `DrawnAs` naming a widget the preset does not make available
    /// (S6.1, S12).
    WidgetUnavailable {
        /// The widget.
        widget: &'static str,
    },
    /// An application layer that is neither a palette nor a reader stand-in
    /// stating an accent input: the accent is `Theme::set_accent`'s (S9.2,
    /// S12).
    AccentInput,
    /// A list naming one thing twice (S9.1, S10, S12).
    ListedTwice {
        /// The name as written.
        name: String,
    },
    /// A `default_size_by_variant` entry's size its variant does not take
    /// (S10, S12).
    SizeNotTaken {
        /// The size.
        size: String,
    },
    /// An entry of a map of bare blocks (`token.text.keyed`,
    /// `token.icons.sizes.keyed`, `ColorRoles.surfaces`) set to a reference,
    /// `Absent` or `Auto`, which the entry cannot hold: an ill-typed patch
    /// (S6.1, S12).
    NotABlock,
    /// A keyed scalar stated in another unit than its declaration's (S4.6,
    /// S10, S12).
    UnitMismatch {
        /// The declared unit.
        unit: &'static str,
    },
    /// A declared keyed name nothing uses: no rule selects on it, no base
    /// states it, or no `presentations` list names the event (a typo's
    /// sign, S12).
    Unused {
        /// The name as declared.
        name: String,
    },
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
            ProblemKind::AbsentNotAllowed => f.write_str("may not be absent"),
            ProblemKind::AutoNotAllowed => f.write_str("may not be auto"),
            ProblemKind::CallNotAllowed => {
                f.write_str("takes a literal or a reference, not a call")
            }
            ProblemKind::OutOfRange { expected } => write!(f, "out of range: {expected}"),
            ProblemKind::NoLayer => f.write_str("has no layer"),
            ProblemKind::MaskAtBottom => f.write_str("its bottom layer is a mask"),
            ProblemKind::TooFewStops => f.write_str("a gradient has at least two stops"),
            ProblemKind::NoProperties => f.write_str("a transition of no property group"),
            ProblemKind::ZeroDuration => f.write_str("a zero duration is `instant`"),
            ProblemKind::KeyframesNotIncreasing => {
                f.write_str("the keyframes' `at` do not increase")
            }
            ProblemKind::KeyframeWithoutPart => {
                f.write_str("a widget's or component's loop's keyframe names no part")
            }
            ProblemKind::FollowPartOutsideFocus => {
                f.write_str("`follow_part` outside a focus indicator's corners")
            }
            ProblemKind::RouteDuplicated { tag } => {
                write!(f, "`{tag}` duplicates a field's route")
            }
            ProblemKind::KdeAccentArgument { name } => {
                write!(f, "`kde_accent` does not take `{name}`")
            }
            ProblemKind::PickWithoutColors => f.write_str("`pick` with no colours"),
            ProblemKind::TooDeep => f.write_str("an expression nested deeper than 32"),
            ProblemKind::BaseInBases => {
                f.write_str("`base` in a layer's bases, where nothing lies beneath")
            }
            ProblemKind::ElementOperand { operand } => {
                write!(f, "`{operand}` is an element's, not a token's or role's")
            }
            ProblemKind::SurfaceInRoles => {
                f.write_str("a colour role names `surface.`, not `role.<set>.`")
            }
            ProblemKind::OtherElementsPart { part } => {
                write!(f, "`widget.{part}` is another element's part")
            }
            ProblemKind::UndeclaredData { datum } => {
                write!(f, "`{datum}` is element data the part does not declare")
            }
            ProblemKind::Undeclared { name, declaration } => {
                write!(f, "`{name}` is not declared in `{declaration}`")
            }
            ProblemKind::NotConditions => {
                f.write_str("a colours or tokens rule selects on conditions only")
            }
            ProblemKind::WidgetUnavailable { widget } => {
                write!(f, "`{widget}` is not available on the preset")
            }
            ProblemKind::AccentInput => f.write_str("the accent is `Theme::set_accent`'s"),
            ProblemKind::ListedTwice { name } => write!(f, "`{name}` listed twice"),
            ProblemKind::SizeNotTaken { size } => {
                write!(f, "the variant does not take the size `{size}`")
            }
            ProblemKind::NotABlock => f.write_str(
                "an entry of a map of blocks holds a block, never a reference, `absent` or `auto`",
            ),
            ProblemKind::UnitMismatch { unit } => write!(f, "declared in `{unit}`"),
            ProblemKind::Unused { name } => write!(f, "`{name}` is declared but unused"),
        }
    }
}
