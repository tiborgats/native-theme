//! The error types.

use std::fmt;

use super::expr::ParseError;
use super::inputs::InputName;
use super::layer::LayerId;
use super::presets::PresetId;

/// [`Key::new`](crate::Key::new)'s error: the text, and the first
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

/// Adding a layer, or reading one, failed (S1, S12), or a change of the
/// stack was refused (S8, S9.2, S16).
#[derive(Clone, Debug, PartialEq)]
pub enum LayerError {
    /// Every problem found, not the first only: S12's checks', and the TOML
    /// reader's own (S11: text that is not TOML, an unknown key, a mistyped
    /// value, an unknown name, a key outside the key grammar, an expression
    /// that does not parse), each a [`ProblemKind`].
    Invalid(Vec<Problem>),
    /// A reader stand-in set while a reader exists (S8, R2.6).
    ReaderPresent,
    /// An accent, a stand-in, or either under a new preset, not stating the
    /// preset's accent inputs as one group (S9.2): at a scheme at which it
    /// states any of them, those lacking their per-scheme value there, each
    /// named once whatever the schemes.
    AccentInputsMissing(Vec<InputName>),
    /// `replace_layer` or `remove_layer` given an id this theme does not
    /// hold: already removed, or another theme's (S16).
    UnknownLayer(LayerId),
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
            LayerError::ReaderPresent => {
                f.write_str("a reader stand-in is refused while a reader exists")
            }
            LayerError::AccentInputsMissing(names) => {
                f.write_str("the preset's accent inputs are not stated as one group, lacking")?;
                names.iter().try_for_each(|name| write!(f, " `{name}`"))
            }
            LayerError::UnknownLayer(id) => write!(f, "the theme holds no {id}"),
        }
    }
}

impl std::error::Error for LayerError {}

/// A theme could not be built (S1, S16).
#[derive(Clone, Debug, PartialEq)]
pub enum ThemeError {
    /// `Theme::from_system` on a platform or desktop this build has no
    /// reader and bundled preset for.
    PlatformUnsupported,
    /// `Theme::from_system` where the platform's reader feature is off.
    FeatureDisabled,
    /// `Theme::from_system(Some(id))` where `id` is not a generation of the
    /// running platform (R1.5).
    PresetNotForPlatform(PresetId),
    /// A failed first read, with the preset `Theme::from_system` chose, for
    /// `Theme::from_preset` (S16).
    Reader {
        /// The preset the system read chose.
        preset: PresetId,
        /// Why the read failed.
        failure: ReaderFailure,
    },
    /// A bundled preset failing validation, which a build test rules out
    /// (S19.1).
    Preset(LayerError),
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeError::PlatformUnsupported => {
                f.write_str("no reader reads this platform's theme in this build")
            }
            ThemeError::FeatureDisabled => f.write_str("the platform's reader feature is disabled"),
            ThemeError::PresetNotForPlatform(id) => write!(
                f,
                "`{}` is not a generation of the running platform",
                id.key()
            ),
            ThemeError::Reader { preset, failure } => write!(
                f,
                "the system read for `{}` failed: {failure}",
                preset.key()
            ),
            ThemeError::Preset(error) => write!(f, "the bundled preset is invalid: {error}"),
        }
    }
}

impl std::error::Error for ThemeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ThemeError::Reader { failure, .. } => Some(failure),
            ThemeError::Preset(error) => Some(error),
            ThemeError::PlatformUnsupported
            | ThemeError::FeatureDisabled
            | ThemeError::PresetNotForPlatform(_) => None,
        }
    }
}

/// How the theme's reader layer was last read (S1, S16).
#[derive(Clone, Debug, PartialEq)]
pub enum ReaderStatus {
    /// The system was read.
    Read,
    /// No reader and no stand-in: S16's no-reader state.
    NoReader,
    /// The application's stand-in stands for the reader.
    StandIn,
    /// Read, but one source failed and another stood in for it, which the
    /// failure names (the KDE scheme file, S9.1, or an accent input the
    /// reader could not read, S9.3; the first failure where several).
    Fallback(ReaderFailure),
    /// A re-read failed; the previous reader layer stays (S8, S16).
    Failed(ReaderFailure),
}

/// Why a read failed, or what a fallback stood in for (S1).
#[derive(Clone, Debug, PartialEq)]
pub enum ReaderFailure {
    /// The platform's settings could not be read: what failed.
    Platform(String),
    /// The reader's layer or state failed validation over the stack.
    Invalid(LayerError),
}

impl fmt::Display for ReaderFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReaderFailure::Platform(what) => f.write_str(what),
            ReaderFailure::Invalid(error) => write!(f, "the reader's layer is invalid: {error}"),
        }
    }
}

impl std::error::Error for ReaderFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReaderFailure::Platform(_) => None,
            ReaderFailure::Invalid(error) => Some(error),
        }
    }
}

/// Why [`Palette::from_port`](crate::Palette::from_port) refused a KDE
/// colour scheme file (S1, S14). A group or key the file lacks and a line
/// KConfig skips are no error: Plasma applies such a file, and so does the
/// conversion (S9.1, S9.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortError {
    /// The file has no colour group of a set `kde-breeze` declares,
    /// `[Colors:<Set>]` or a subgroup of one such as
    /// `[Colors:Header][Inactive]`: it would convert to KColorScheme's
    /// defaults, Breeze Light's colours, under whichever scheme it is named
    /// for (ADD-A142).
    NoColorGroup,
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortError::NoColorGroup => f.write_str("the colour scheme file has no colour group"),
        }
    }
}

impl std::error::Error for PortError {}

/// One problem a layer has: where, and what.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Problem {
    /// The model path, or the TOML key path where the layer was read from
    /// TOML (`widgets.button.body.fill`, `rules[2].when.states[0]`).
    ///
    /// A path starts at the layer. A base's place starts at its field of
    /// [`Bases`](crate::Bases) (`widgets.button.body.fill`,
    /// `tokens.motion.durations`), a preset's own bases' at
    /// `for_preset.<id>`, a rule's at `rules[<index>]`. Below that, a layer
    /// built in Rust names each place by the model's fields
    /// (`rules[2].selector.states.none_of[0]`, `rules[2].body.fill`,
    /// `inputs.entries.accent.per_scheme.light`), and a layer read from
    /// TOML by its keys as written (`rules[2].when.not_states[0]`,
    /// `rules[2].set.fill`, `inputs.accent.light`, a shorthand's own key:
    /// `corners_px`). A layer keeps the spelling it was made with: one read
    /// from TOML and changed through its fields afterwards is still named
    /// by its TOML keys, a rule's shorthand only while the rule read is
    /// still at its index.
    ///
    /// A reader's or stand-in's state is named from its
    /// [`ReaderState`](crate::ReaderState)'s fields
    /// (`text_scaling_factor`, `font_dpi`, `container_sizes[2]`), as it is
    /// no layer's; a flavour the preset does not declare has the empty path,
    /// as it is no value's.
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
    /// A `FocusIndicator.transition` whose `properties` list is not empty:
    /// it times the whole indicator, so it names no group (S2.10, S12).
    FocusTransitionProperties,
    /// A `Transition.from` that is an empty list: it would apply from no
    /// state (S2.7, S12).
    FromNoState,
    /// A `WinUiAnimatedIcon` transition naming a `visual` that
    /// `token.icons.animated_visuals` does not hold, under some combination
    /// (S2.7, S4.5, S12).
    UnknownVisual {
        /// The visual as written.
        visual: String,
    },
    /// A transition's or presentation step's `timing` that can be `Absent`
    /// where its `duration` is not a stated zero, but a transition's under
    /// `WinUiAnimatedIcon`, under some combination (S2.7, S12).
    TimingAbsent,
    /// A stated zero duration's `timing` that can be stated, under some
    /// combination: a zero runs no curve, so its timing is `Absent` (S2.7,
    /// S12).
    TimingOnZero,
    /// A transition's or presentation step's `duration` that can be
    /// `Absent` where its timing, followed through references, is no
    /// spring, but a transition's under `WinUiAnimatedIcon`, under some
    /// combination (S2.7, S6.3, S12).
    DurationAbsent,
    /// A `PresentationKind::Instant` step whose `duration` can be other
    /// than a stated zero, under some combination (S2.7, S12).
    InstantNotZero,
    /// A presentation step of another kind than `Instant` that can state a
    /// zero `duration`, under some combination: an instant step is
    /// `Instant`, its one spelling (S2.7, S12).
    ZeroNotInstant,
    /// A transition's stated timing, followed through references, that its
    /// engine's code does not compute, under some combination (S2.7, S12).
    TimingNotComputed {
        /// The engine, as S11 spells it.
        engine: &'static str,
    },
    /// A transition its engine does not run: a `WinUiAnimatedIcon` one
    /// stating a duration, a timing or a delay, or of another kind than
    /// `Keyed`; a `Breeze` or `BreezeTabBar` one with no duration, a delay
    /// or another kind than `Interpolate`; a `GtkSwitch` one not the switch
    /// thumb's `Geometry` one, `Interpolate`, with no delay; a
    /// `ComposeHeld` one not the `Geometry` one of a segmented control's
    /// segment; a `Keyed` one under a value engine; a `Keyframes` one under
    /// another engine than `CssAnimation`, or a `CssAnimation` one of
    /// another kind (S2.7, S12; ADD-A126, ADD-A134).
    EngineMismatch {
        /// The engine, as S11 spells it.
        engine: &'static str,
        /// What the engine takes.
        takes: &'static str,
    },
    /// A Breeze engine's test that holds nowhere or reads what the engine
    /// cannot: empty on a transition that is not a stated zero, not empty
    /// on a stated zero, a flag in both `any` and `none`, a condition that
    /// is not the window's, or a focus indicator's test naming no
    /// `focus_visible` in `any` (S2.7, S12; ADD-A126).
    BreezeTestInvalid {
        /// What the test is to name.
        expected: &'static str,
    },
    /// A `FocusMove` transition: no engine runs one yet, as an engine runs
    /// per element part and the move crosses two (S2.7, S2.10, S12;
    /// ADD-A126).
    FocusMoveNotRun,
    /// A `WinUiAnimatedIcon` transition's `Keyed` key whose state name, its
    /// UpperCamelCase form, no marker of its visual holds, under some
    /// combination (S2.7, S12).
    UnknownState {
        /// The key as written.
        state: String,
        /// The visual.
        visual: String,
    },
    /// A `GtkCss` part's transitions in effect together under some
    /// combination naming several curves, followed through references,
    /// while one of them lists a group other than `Shadow` and `Corners`,
    /// the groups GTK's engine takes a curve for by property id (S2.7, S12;
    /// ADD-A131).
    SeveralCurves,
    /// A stated zero with no delay that overrides nothing: no other
    /// transition of one of its groups (the focus indicator's, for a
    /// `FocusIndicator.transition`), earlier in its list, in the base or in
    /// an earlier rule of its part, can apply where its rule does (S2.7,
    /// S12; ADD-A126).
    OverridesNothing,
    /// A recency set with fewer than two flags (S10, S12).
    RecencyTooSmall,
    /// A recency set ordering two flags oppositely to another of the
    /// widget's sets, which it names (S10, S12).
    RecencyOrder {
        /// The other set.
        set: String,
    },
    /// A `latest` term naming a flag that is not a member of its set (S7.2,
    /// S12).
    NotInRecencySet {
        /// The flag as written.
        flag: String,
        /// The set.
        set: String,
    },
    /// A `StateLayer` transition on a part whose fields lack `interaction`
    /// (S2.7, S2.12, S12).
    StateLayerWithoutInteraction,
    /// An `IndicationBox.part` naming a part of another widget than the
    /// ripple's (S2.12, S12).
    IndicationBoxOtherWidget {
        /// The part named, `<widget>.<part>`.
        part: String,
    },
    /// An `IndicationBox.part` naming a part that can be `Absent` where
    /// the ripple can be drawn, under some combination (S2.12, S12).
    IndicationBoxPartAbsent {
        /// The part named.
        part: String,
    },
    /// A loop's or a `Keyframes` transition's keyframes' `at` do not
    /// increase, or repeat a value (S2.7, S12).
    KeyframesNotIncreasing,
    /// A keyframe of a loop stated as a widget's or a component's value
    /// leaves its `part` out: such a loop is on no part (S2.7, S12).
    KeyframeWithoutPart,
    /// A `Keyframes` transition with no keyframe (S2.7, S12; ADD-A134).
    NoKeyframe,
    /// A `Keyframes` transition's keyframe naming a `part`: each styles the
    /// part or block the transition times (S2.7, S12; ADD-A134).
    KeyframeNamesPart,
    /// A `Keyframes` transition's keyframe patch stating a field its
    /// transition's groups do not time; in a `FocusIndicator.transition`,
    /// any field but `focus`'s (S2.7, S12; ADD-A134).
    KeyframeOutsideGroups,
    /// A keyframe's patch stating `icon.animation`, which has no effect: a
    /// keyframe resolves its part's icon without the icon's own animation
    /// (S2.7, S12).
    KeyframeAnimation,
    /// A `KirigamiSmoothstep` shadow layer's `spread` that can be other
    /// than zero, or its `inset` true: Kirigami's shadow has no spread and
    /// is never inset (S2.5, S12; ADD-A134).
    SmoothstepSpreadOrInset,
    /// `Radius::FollowPart` outside a `FocusIndicator.corners` (S2.4, S12).
    FollowPartOutsideFocus,
    /// `round_pt` called outside an expression stated on a `Font.size`, or
    /// with an operand that is not a reference to a font size (S2.2, S12).
    RoundPtMisused,
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
    /// An expression that does not type against its field (S2.2, S12): it
    /// mistypes, names nothing, or calls a platform function not
    /// implemented yet.
    IllTyped {
        /// Why, as typing says it.
        reason: String,
    },
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
    /// A default size that is not one of the sizes it goes with: a
    /// declaration's, a `sizes_by_variant` entry's, or the one in force in a
    /// `sizes_by_context` entry's context (S10, S12).
    SizeNotTaken {
        /// The size.
        size: String,
    },
    /// The missing default size (S10, S12): sizes taken and no
    /// `default_size` in force, or a pending one outside the synthetic
    /// route (S10's *A pending default*, S17).
    DefaultSizeMissing,
    /// An element taking sizes outside a `sizes_by_context` entry's context
    /// and none inside it (S10, S12).
    NoSizeInContext {
        /// The `sizes_by_variant` entry's variant the element holds; `None`:
        /// an element holding none.
        variant: Option<String>,
    },
    /// A `sizes_by_context` entry's selector stating `states`,
    /// `not_states`, `sizes`, `not_sizes` or `surface`, which the sizes an
    /// element takes do not change with (S10, S12).
    SizeContextKey {
        /// The key.
        key: &'static str,
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
    /// `DrawnAs` patches forming a cycle under some combination: the part,
    /// followed through `DrawnAs`, is drawn again (S6.1, S12).
    DrawnAsCycle,
    /// `text_scale` read where `token.text.scaling` can be `DynamicType`,
    /// whose lengths scale by `uifm_scaled` (S2.2, S9.4, S12).
    TextScaleUnderDynamicType,
    /// An expression's call meeting an operand that can resolve `Absent`
    /// (S2.2, S12).
    AbsentOperand {
        /// The operand as written.
        operand: String,
    },
    /// An expression's call meeting an operand that can resolve `Auto`, or
    /// a `TextStyle` token reading `line_height` while that style's line
    /// height can be `Auto` (S2.2, S12).
    AutoOperand {
        /// The operand as written.
        operand: String,
    },
    /// An expression reading itself through references, under some
    /// combination (S2.2, S12).
    ExpressionCycle,
    /// A chain of references longer than 32 (S2.2, S12).
    ChainTooLong,
    /// A `widget.<part>` reference naming a part that can resolve `Absent`
    /// as a whole or `DrawnAs` (S2.2, S6.3, S12).
    PartNotDrawn {
        /// The part named.
        part: String,
    },
    /// An expression naming a corner that can resolve `Concentric` or
    /// `FollowPart`, or `Full` on a part other than the element's own part:
    /// the connector computes it at layout (S2.4, S8, S12).
    CornerAtLayout {
        /// The operand as written.
        operand: String,
    },
    /// A focus indicator's `FollowPart` corner whose two sides' offsets can
    /// differ (S2.4, S2.10, S12).
    UnequalOffsets {
        /// The corner.
        corner: String,
    },
    /// A `Corners` value stating `Ellipse` on some of its corners but not
    /// all four, or with a `Continuous` curve, under some combination (S2.4,
    /// S12, ADD-A124).
    EllipseCorners,
    /// A `Concentric` corner whose two insets can differ, until the value
    /// phase states UIKit's rule for them (S2.4, S12, ADD-A122).
    UnequalInsets {
        /// The corner.
        corner: String,
    },
    /// Two `TextRoles` slots referencing one keyed text style directly
    /// (S4.4, S12).
    TextRoleShared {
        /// The keyed style.
        style: String,
    },
    /// A `TextRoles` slot referencing a slot later in S4.4's owner order
    /// (S4.4, S12).
    TextRoleOrder {
        /// The slot referenced.
        slot: String,
    },
    /// A part's `shape` stated while its corners' curve can be
    /// `Continuous`, or a corner `Ellipse` (S2.4, S12).
    ShapeOverCorners,
    /// A map entry's patch that can create its entry with a field left
    /// unset that may not be `Absent` (S6.1, S12, ADD-A122).
    EntryCreatedUnset {
        /// The field left unset; `paint` for a paint map's entry, the
        /// paint itself, an `Append` creates (S6.1, ADD-A131).
        field: String,
    },
    /// A field laying the element out or sizing its content reading the
    /// element's laid-out size, the element's own part no fixed box (S8,
    /// S12).
    ElementSizeInLayout {
        /// The operand as written.
        datum: String,
    },
    /// On `kde-breeze`, a mapping palette leaving an accent-dependent role
    /// or an inactive input role to the preset's Breeze value (S9.1, S9.2,
    /// S14, S12).
    LeftToPreset {
        /// The role or input, as its TOML path names it.
        role: String,
    },
    /// On `kde-breeze`, a reader's or stand-in's layer stating a surface
    /// set whole, a block and not field by field, where S8 leaves its
    /// accent-dependent roles out beneath the accent-derivation layer: the
    /// leave-out is per role and reaches no role of a block stated whole
    /// (S8, S9.2, S12).
    SetStatedWhole {
        /// The surface set.
        set: &'static str,
    },
    /// A declared keyed name nothing uses: no rule selects on it, no base
    /// states it, or no `presentations` list names the event (a typo's
    /// sign, S12).
    Unused {
        /// The name as declared.
        name: String,
    },
    /// A value one of the synthetic route's layers states as an
    /// expression, where they state constant values (S17, ADD-A131).
    SyntheticExpression,
    /// A place one of the synthetic route's layers states at, above or
    /// below a place an earlier one states, where they state disjoint places
    /// (S17, ADD-A131).
    SyntheticOverlap,
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
            ProblemKind::FocusTransitionProperties => {
                f.write_str("a focus indicator's transition names no property group")
            }
            ProblemKind::FromNoState => f.write_str("a transition applying from no state"),
            ProblemKind::UnknownVisual { visual } => {
                write!(
                    f,
                    "`{visual}` can name no visual `token.icons.animated_visuals` holds"
                )
            }
            ProblemKind::TimingAbsent => {
                f.write_str("a timing absent where the duration is not a stated zero")
            }
            ProblemKind::TimingOnZero => {
                f.write_str("a timing stated where the duration is a stated zero")
            }
            ProblemKind::DurationAbsent => {
                f.write_str("a duration absent where the timing is no spring")
            }
            ProblemKind::InstantNotZero => {
                f.write_str("an `instant` step whose duration is not a stated zero")
            }
            ProblemKind::ZeroNotInstant => {
                f.write_str("a stated zero duration on a step that is not `instant`")
            }
            ProblemKind::TimingNotComputed { engine } => {
                write!(f, "a timing the `{engine}` engine does not compute")
            }
            ProblemKind::EngineMismatch { engine, takes } => {
                write!(f, "the `{engine}` engine takes {takes}")
            }
            ProblemKind::BreezeTestInvalid { expected } => write!(f, "a Breeze test {expected}"),
            ProblemKind::FocusMoveNotRun => {
                f.write_str("a `focus_move` transition, which no engine runs yet")
            }
            ProblemKind::UnknownState { state, visual } => write!(
                f,
                "`{state}` names a state no marker of the visual `{visual}` holds"
            ),
            ProblemKind::SeveralCurves => f.write_str(
                "a `gtk_css` style's transitions name several curves on a group other than \
                 `shadow` and `corners`",
            ),
            ProblemKind::OverridesNothing => {
                f.write_str("a stated zero with no delay that overrides no transition")
            }
            ProblemKind::RecencyTooSmall => f.write_str("a recency set of fewer than two flags"),
            ProblemKind::RecencyOrder { set } => {
                write!(f, "orders two flags oppositely to the recency set `{set}`")
            }
            ProblemKind::NotInRecencySet { flag, set } => {
                write!(f, "`{flag}` is not a member of the recency set `{set}`")
            }
            ProblemKind::StateLayerWithoutInteraction => {
                f.write_str("a `state_layer` transition on a part without `interaction`")
            }
            ProblemKind::IndicationBoxOtherWidget { part } => {
                write!(f, "the indication's box is `{part}`, another widget's part")
            }
            ProblemKind::IndicationBoxPartAbsent { part } => write!(
                f,
                "the indication's box is `{part}`, which can be absent where the ripple is drawn"
            ),
            ProblemKind::KeyframesNotIncreasing => {
                f.write_str("the keyframes' `at` do not increase")
            }
            ProblemKind::KeyframeWithoutPart => {
                f.write_str("a widget's or component's loop's keyframe names no part")
            }
            ProblemKind::NoKeyframe => f.write_str("a `keyframes` transition with no keyframe"),
            ProblemKind::KeyframeNamesPart => {
                f.write_str("a `keyframes` transition's keyframe names a part")
            }
            ProblemKind::KeyframeOutsideGroups => {
                f.write_str("a field the keyframes' transition does not time")
            }
            ProblemKind::KeyframeAnimation => {
                f.write_str("a keyframe's `icon.animation`, which has no effect")
            }
            ProblemKind::SmoothstepSpreadOrInset => {
                f.write_str("a `kirigami_smoothstep` shadow layer's spread or inset")
            }
            ProblemKind::FollowPartOutsideFocus => {
                f.write_str("`follow_part` outside a focus indicator's corners")
            }
            ProblemKind::RoundPtMisused => f.write_str(
                "`round_pt` takes a reference to a font size, in an expression on a `Font.size`",
            ),
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
            ProblemKind::IllTyped { reason } => write!(f, "the expression does not type: {reason}"),
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
                write!(
                    f,
                    "the default size `{size}` is not one of the sizes taken there"
                )
            }
            ProblemKind::DefaultSizeMissing => f.write_str("no default size where sizes are taken"),
            ProblemKind::NoSizeInContext { variant } => match variant {
                Some(variant) => write!(
                    f,
                    "an element holding `{variant}` takes sizes outside the context and none in it"
                ),
                None => f.write_str(
                    "an element holding no variant of an entry takes sizes outside the context and \
                     none in it",
                ),
            },
            ProblemKind::SizeContextKey { key } => {
                write!(f, "a `sizes_by_context` selector may not state `{key}`")
            }
            ProblemKind::NotABlock => f.write_str(
                "an entry of a map of blocks holds a block, never a reference, `absent` or `auto`",
            ),
            ProblemKind::UnitMismatch { unit } => write!(f, "declared in `{unit}`"),
            ProblemKind::Unused { name } => write!(f, "`{name}` is declared but unused"),
            ProblemKind::DrawnAsCycle => f.write_str("drawn as itself through `drawn_as`"),
            ProblemKind::TextScaleUnderDynamicType => {
                f.write_str("`text_scale` where the text scaling can be `dynamic_type`")
            }
            ProblemKind::AbsentOperand { operand } => {
                write!(f, "`{operand}` can be `absent` here")
            }
            ProblemKind::AutoOperand { operand } => write!(f, "`{operand}` can be `auto` here"),
            ProblemKind::ExpressionCycle => f.write_str("the expression reads itself"),
            ProblemKind::ChainTooLong => f.write_str("a chain of references longer than 32"),
            ProblemKind::PartNotDrawn { part } => {
                write!(
                    f,
                    "`widget.{part}` can be absent or drawn as another widget"
                )
            }
            ProblemKind::CornerAtLayout { operand } => {
                write!(f, "`{operand}` is a corner computed at layout")
            }
            ProblemKind::UnequalOffsets { corner } => {
                write!(
                    f,
                    "`follow_part` on `{corner}`, whose sides' offsets can differ"
                )
            }
            ProblemKind::EllipseCorners => f.write_str(
                "`ellipse` on a corner, not on all four corners with a `circular` curve",
            ),
            ProblemKind::UnequalInsets { corner } => {
                write!(f, "`concentric` on `{corner}`, whose insets can differ")
            }
            ProblemKind::TextRoleShared { style } => {
                write!(f, "another slot references `text.keyed.{style}` directly")
            }
            ProblemKind::TextRoleOrder { slot } => {
                write!(f, "references `{slot}`, a slot later in the owner order")
            }
            ProblemKind::ShapeOverCorners => f.write_str(
                "a shape where the corners' curve can be `continuous` or a corner `ellipse`",
            ),
            ProblemKind::EntryCreatedUnset { field } => {
                write!(f, "can create the entry with `{field}` unset")
            }
            ProblemKind::ElementSizeInLayout { datum } => {
                write!(f, "`{datum}` in a field laying the element out")
            }
            ProblemKind::LeftToPreset { role } => {
                write!(f, "`{role}` left to the preset's value under the palette")
            }
            ProblemKind::SetStatedWhole { set } => write!(
                f,
                "the surface set `{set}` stated whole where its accent-dependent roles are left out; state its roles one by one"
            ),
            ProblemKind::SyntheticExpression => {
                f.write_str("an expression in a synthetic layer, which states constant values")
            }
            ProblemKind::SyntheticOverlap => f.write_str(
                "a place an earlier synthetic layer states too, or one above or below it",
            ),
        }
    }
}

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
        #[cfg_attr(
            not(all(feature = "watch", feature = "kde", target_os = "linux")),
            allow(dead_code)
        )]
        KDE_WATCHER = "kde-watcher";
        #[cfg_attr(
            not(all(feature = "watch", feature = "portal", target_os = "linux")),
            allow(dead_code)
        )]
        GNOME_WATCHER = "gnome-watcher";
        #[cfg_attr(
            not(all(feature = "watch", feature = "windows", target_os = "windows")),
            allow(dead_code)
        )]
        WINDOWS_WATCHER = "windows-watcher";
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
