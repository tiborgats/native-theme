//! Rules' selectors and the conditions they hold under (S7), and the
//! references to widgets, parts and variants they name.

use native_theme_derive::ThemeParts;

use super::key::Key;
use super::presets::PresetId;
use super::roles::{ColorRolesPatch, SurfaceSet};
use super::tokens::TokensPatch;
use super::units::Px;
use super::value::Value;
use super::widgets::{PartPatch, TypedPart, Variant, WidgetKind, WidgetValuesPatch};

/// A rule: where it applies, and what it patches (S7).
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Rule {
    /// Where the rule applies.
    pub selector: Selector,
    /// The target and its patch.
    pub body: RuleBody,
}

impl Rule {
    /// The rule patching `body` where `selector` matches.
    #[must_use]
    pub fn new(selector: Selector, body: RuleBody) -> Rule {
        Rule { selector, body }
    }
}

/// A rule's target and its patch in one, so a mismatch cannot be written
/// (S7). A rule targeting `Colors` or `Tokens` may only have conditions
/// (S12).
#[derive(Clone, Debug, PartialEq)]
#[expect(
    clippy::large_enum_variant,
    reason = "S7's variants as stated; a rule is read when a layer is added, not drawn per frame"
)]
pub enum RuleBody {
    /// The colour roles of the scheme in force.
    Colors(ColorRolesPatch),
    /// The tokens.
    Tokens(TokensPatch),
    /// A part, and its patch.
    Part(PartRef, PartPatch),
    /// A widget's values.
    Values(ValuesPatch),
}

/// A widget's values' patch (S6.1).
#[derive(Clone, Debug, PartialEq)]
pub enum ValuesPatch {
    /// A typed widget's.
    Typed(WidgetValuesPatch),
}

/// Where a rule applies (S7): every part must match; an empty selector
/// matches everywhere.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Selector {
    /// The conditions that hold, and those that do not.
    pub conditions: SetMatch<Condition>,
    /// Ancestors that must each match.
    pub context: Vec<ContextMatch>,
    /// Ancestors none of which may match.
    pub not_context: Vec<ContextMatch>,
    /// The element's place among its siblings.
    pub position: Option<PositionMatch>,
    /// The element's variants.
    pub variants: SetMatch<VariantRef>,
    /// The element's size, density and size classes.
    pub size: SizeMatch,
    /// The element's layout direction.
    pub direction: Option<Direction>,
    /// The element's states.
    pub states: SetMatch<StateFlag>,
    /// Per recency set (S10), the flag that must be its latest (S7.2).
    pub latest: Vec<(Key, StateFlag)>,
}

/// A set test: every one of `all_of` holds, and none of `none_of`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetMatch<T> {
    /// What must all hold.
    pub all_of: Vec<T>,
    /// What none of may hold.
    pub none_of: Vec<T>,
}

impl<T> Default for SetMatch<T> {
    fn default() -> Self {
        SetMatch {
            all_of: Vec::new(),
            none_of: Vec::new(),
        }
    }
}

/// An ancestor a selector matches on (S7).
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ContextMatch {
    /// Which ancestors count.
    pub scope: ContextScope,
    /// The ancestor's widget or part; `None`: any widget (R25.9).
    pub target: Option<WidgetOrPart>,
    /// The ancestor's control size (MAC-MN15).
    pub sizes: SetMatch<ControlSize>,
    /// The surface set the ancestor binds (Q1.37); `None`: not matched on.
    pub surface: Option<SurfaceSet>,
    /// The ancestor's variants.
    pub variants: SetMatch<VariantRef>,
    /// The ancestor's states.
    pub states: SetMatch<StateFlag>,
}

impl ContextMatch {
    /// A match among the ancestors `scope` names, on nothing else yet.
    #[must_use]
    pub fn new(scope: ContextScope) -> ContextMatch {
        ContextMatch {
            scope,
            target: None,
            sizes: SetMatch::default(),
            surface: None,
            variants: SetMatch::default(),
            states: SetMatch::default(),
        }
    }
}

/// Which ancestors a context match looks at.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContextScope {
    /// Any ancestor.
    AnyAncestor,
    /// The nearest ancestor that is one of these widgets.
    NearestAmong(Vec<WidgetRef>),
}

/// The element's place among its siblings (S7).
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct PositionMatch {
    /// Which siblings count (R25.9).
    pub among: SiblingScope,
    /// Whether it is the first.
    pub first: Option<bool>,
    /// Whether it is the last.
    pub last: Option<bool>,
    /// Whether it is the only one.
    pub only: Option<bool>,
    /// Its parity, counted from 1.
    pub parity: Option<Parity>,
    /// The adjacent sibling before it, among those `among` names.
    pub previous: Option<SiblingMatch>,
    /// The adjacent sibling after it, among those `among` names.
    pub next: Option<SiblingMatch>,
}

/// Which siblings a position counts among.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum SiblingScope {
    /// Siblings of the element's own widget and part.
    #[default]
    SameKind,
    /// Siblings of any kind.
    Any,
    /// Siblings of these widgets.
    Kinds(Vec<WidgetRef>),
}

/// A parity, counted from 1 among the siblings that count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Parity {
    /// Odd.
    Odd,
    /// Even.
    Even,
}

/// An adjacent sibling a position matches on.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct SiblingMatch {
    /// The sibling's widget or part; `None`: any.
    pub target: Option<WidgetOrPart>,
    /// The sibling's variants.
    pub variants: SetMatch<VariantRef>,
    /// The sibling's states.
    pub states: SetMatch<StateFlag>,
}

/// The element's control size, density and size classes a selector matches
/// on (S7.3).
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct SizeMatch {
    /// The control size.
    pub sizes: SetMatch<ControlSize>,
    /// The density.
    pub densities: SetMatch<Density>,
    /// The size classes.
    pub size_classes: SizeClassMatch,
}

/// The size classes a selector matches on, per axis (S7.3).
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct SizeClassMatch {
    /// The horizontal size class.
    pub horizontal: SetMatch<SizeClass>,
    /// The vertical size class.
    pub vertical: SetMatch<SizeClass>,
}

/// A condition a rule holds under (S7.1): the system's, by the reader; the
/// window's, by the connector; the application's.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    /// The scheme.
    Scheme(Scheme),
    /// Increased contrast.
    ContrastIncreased,
    /// High contrast.
    ContrastHigh,
    /// Reduced transparency.
    ReducedTransparency,
    /// Reduced motion.
    ReducedMotion,
    /// Animations off.
    AnimationsOff,
    /// Bold text.
    BoldText,
    /// Button shapes.
    ButtonShapes,
    /// Differentiate without colour.
    DifferentiateWithoutColor,
    /// Tablet mode.
    TabletMode,
    /// Plasma Mobile's shell, which is not a device idiom.
    MobileFormFactor,
    /// Battery saver.
    BatterySaver,
    /// A container's size the preset declares (macOS's sidebar icon size).
    ContainerSize {
        /// The container.
        container: WidgetRef,
        /// Its size.
        size: Key,
    },
    /// Overlay scroll bars.
    OverlayScrollbars,
    /// On/off labels.
    OnOffLabels,
    /// Full keyboard access.
    FullKeyboardAccess,
    /// Prefer cross-fade transitions.
    PreferCrossFade,
    /// The content-size category is one of the five accessibility
    /// categories.
    AccessibilityTextSize,
    /// UIKit's and Material's device idiom.
    Idiom(Idiom),
    /// A platform switch the reader reads, declared (S10).
    SystemSetting(Key),
    /// The device model, where a measured value differs per model.
    DeviceModel(Key),
    /// The window is inactive.
    WindowInactive,
    /// The window is not the main window.
    NotMain,
    /// The application is inactive.
    AppInactive,
    /// The window is maximised.
    Maximized,
    /// The window is full screen.
    FullScreen,
    /// The window is tiled to every edge these name, at least.
    Tiled(TiledEdges),
    /// The window is elevated.
    Elevated,
    /// The kind of the last input.
    Input(InputKind),
    /// The window is not composited.
    NotComposited,
    /// The window is shaded.
    Shaded,
    /// The window flashes for attention.
    AttentionFlash,
    /// The window is dimmed by a tint.
    TintDimmed,
    /// Mnemonics are visible.
    MnemonicsVisible,
    /// Android's desktop windowing.
    Freeform,
    /// A foldable's posture.
    Posture(Posture),
    /// The device pixel ratio is 1.
    UnitScale,
    /// The flavour in force.
    Flavor(Key),
    /// The current preset.
    Preset(PresetId),
}

/// The kind of the last input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum InputKind {
    /// A mouse.
    Mouse,
    /// Touch.
    Touch,
    /// A pen.
    Pen,
    /// A game controller.
    GameController,
}

/// UIKit's and Material's device idiom.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Idiom {
    /// A phone.
    Phone,
    /// A pad.
    Pad,
}

/// A foldable's posture (Jetpack WindowManager).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Posture {
    /// Flat.
    Flat,
    /// Tabletop.
    Tabletop,
    /// Book.
    Book,
}

/// The edges a window is tiled to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TiledEdges {
    /// The top edge.
    pub top: bool,
    /// The right edge.
    pub right: bool,
    /// The bottom edge.
    pub bottom: bool,
    /// The left edge.
    pub left: bool,
}

/// A state flag (S7.2).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateFlag {
    /// Hovered.
    Hover,
    /// Pressed.
    Pressed,
    /// Focused.
    Focused,
    /// Focus is within.
    FocusWithin,
    /// Focus is visible.
    FocusVisible,
    /// Disabled.
    Disabled,
    /// Toggled on.
    Checked,
    /// The current choice among siblings.
    Selected,
    /// Indeterminate.
    Indeterminate,
    /// Open.
    Open,
    /// The default.
    Default,
    /// Dragged.
    Dragged,
    /// A drop target.
    DropTarget,
    /// Read-only.
    ReadOnly,
    /// Invalid.
    Invalid,
    /// Visited.
    Visited,
    /// Autofilled.
    Autofilled,
    /// Scrolled.
    Scrolled,
    /// Over dark content.
    OverDarkContent,
    /// Opened by touch.
    OpenedByTouch,
    /// Overflowing.
    Overflowing,
    /// Redacted.
    Redacted,
    /// A flag two or more platforms draw on a typed widget.
    Shared(SharedFlag),
    /// A one-platform flag the preset declares (S10).
    Widget(Key),
}

impl StateFlag {
    /// The typed flags, each with its name as S11 writes it.
    const TYPED: [(&'static str, StateFlag); 22] = [
        ("hover", StateFlag::Hover),
        ("pressed", StateFlag::Pressed),
        ("focused", StateFlag::Focused),
        ("focus_within", StateFlag::FocusWithin),
        ("focus_visible", StateFlag::FocusVisible),
        ("disabled", StateFlag::Disabled),
        ("checked", StateFlag::Checked),
        ("selected", StateFlag::Selected),
        ("indeterminate", StateFlag::Indeterminate),
        ("open", StateFlag::Open),
        ("default", StateFlag::Default),
        ("dragged", StateFlag::Dragged),
        ("drop_target", StateFlag::DropTarget),
        ("read_only", StateFlag::ReadOnly),
        ("invalid", StateFlag::Invalid),
        ("visited", StateFlag::Visited),
        ("autofilled", StateFlag::Autofilled),
        ("scrolled", StateFlag::Scrolled),
        ("over_dark_content", StateFlag::OverDarkContent),
        ("opened_by_touch", StateFlag::OpenedByTouch),
        ("overflowing", StateFlag::Overflowing),
        ("redacted", StateFlag::Redacted),
    ];

    /// The typed flag `name` names.
    pub(crate) fn typed(name: &str) -> Option<StateFlag> {
        Self::TYPED
            .iter()
            .find(|(held, _)| *held == name)
            .map(|(_, flag)| flag.clone())
    }

    /// The flag's name as S11 writes it: a typed or shared flag's own, a
    /// declared one's `@<key>`.
    pub(crate) fn name(&self) -> String {
        match self {
            StateFlag::Shared(flag) => crate::next::toml::read::Named::name(flag).to_owned(),
            StateFlag::Widget(key) => format!("@{key}"),
            StateFlag::Hover => "hover".to_owned(),
            StateFlag::Pressed => "pressed".to_owned(),
            StateFlag::Focused => "focused".to_owned(),
            StateFlag::FocusWithin => "focus_within".to_owned(),
            StateFlag::FocusVisible => "focus_visible".to_owned(),
            StateFlag::Disabled => "disabled".to_owned(),
            StateFlag::Checked => "checked".to_owned(),
            StateFlag::Selected => "selected".to_owned(),
            StateFlag::Indeterminate => "indeterminate".to_owned(),
            StateFlag::Open => "open".to_owned(),
            StateFlag::Default => "default".to_owned(),
            StateFlag::Dragged => "dragged".to_owned(),
            StateFlag::DropTarget => "drop_target".to_owned(),
            StateFlag::ReadOnly => "read_only".to_owned(),
            StateFlag::Invalid => "invalid".to_owned(),
            StateFlag::Visited => "visited".to_owned(),
            StateFlag::Autofilled => "autofilled".to_owned(),
            StateFlag::Scrolled => "scrolled".to_owned(),
            StateFlag::OverDarkContent => "over_dark_content".to_owned(),
            StateFlag::OpenedByTouch => "opened_by_touch".to_owned(),
            StateFlag::Overflowing => "overflowing".to_owned(),
            StateFlag::Redacted => "redacted".to_owned(),
        }
    }
}

/// A set of state flags (S1): an element's, a sibling's, or those a
/// transition's `from` is matched against.
#[derive(Clone, Debug, Default, Eq)]
#[non_exhaustive]
pub struct StateFlags {
    flags: Vec<StateFlag>,
}

/// Two sets are equal holding the same flags, in whatever order they were
/// set (S1: a set).
impl PartialEq for StateFlags {
    fn eq(&self, other: &Self) -> bool {
        self.flags.len() == other.flags.len()
            && self.flags.iter().all(|flag| other.flags.contains(flag))
    }
}

impl StateFlags {
    /// The empty set.
    #[must_use]
    pub fn new() -> StateFlags {
        StateFlags::default()
    }

    /// The set with `flag` added.
    #[must_use]
    pub fn with(mut self, flag: StateFlag) -> StateFlags {
        self.insert(flag);
        self
    }

    /// Adds `flag`.
    pub fn insert(&mut self, flag: StateFlag) {
        if !self.flags.contains(&flag) {
            self.flags.push(flag);
        }
    }

    /// Removes `flag`.
    pub fn remove(&mut self, flag: &StateFlag) {
        self.flags.retain(|held| held != flag);
    }

    /// Whether `flag` is in the set.
    #[must_use]
    pub fn contains(&self, flag: &StateFlag) -> bool {
        self.flags.contains(flag)
    }

    /// The flags, in the order they were added.
    pub fn iter(&self) -> impl Iterator<Item = &StateFlag> {
        self.flags.iter()
    }

    /// Whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
}

impl FromIterator<StateFlag> for StateFlags {
    fn from_iter<I: IntoIterator<Item = StateFlag>>(iter: I) -> StateFlags {
        let mut out = StateFlags::new();
        for flag in iter {
            out.insert(flag);
        }
        out
    }
}

/// A flag two or more platforms draw on a typed widget (the catalogue's
/// *Shared flags*).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum SharedFlag {
    /// The element holds no text or value.
    Empty,
    /// A tick or stop indicator inside the value fill.
    InFill,
    /// The day is today.
    Today,
    /// A day outside the shown month.
    OtherMonth,
    /// The current item of a sequence.
    Current,
    /// The element's text wraps to more than one line.
    Multiline,
    /// A rating star filled.
    Filled,
    /// A rating star half filled.
    Half,
    /// The hover preview of a value.
    Preview,
    /// A placeholder in place of the value.
    Placeholder,
    /// Pull-to-refresh pulling.
    Pulling,
    /// Pull-to-refresh refreshing.
    Refreshing,
    /// A popup placed above its anchor.
    PlacedAbove,
    /// A popup placed below its anchor.
    PlacedBelow,
    /// An output route is active.
    RouteActive,
    /// In edit mode.
    Editing,
    /// Swipe actions revealed.
    Swiped,
    /// The context menu or popup is open.
    ContextMenuOpen,
    /// A sorted column header.
    Sorted,
    /// Sorted descending.
    Descending,
    /// Overwrite mode.
    Overwrite,
    /// An input-method composition in progress.
    Composing,
    /// The element in its minimised form.
    Minimized,
    /// The element's text has a selection.
    TextSelected,
    /// A drop would copy.
    DropCopy,
    /// A drop would move.
    DropMove,
}

/// A control size slot (S7.3); a preset declares the slots it fills.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum ControlSize {
    /// Extra small.
    ExtraSmall,
    /// Mini.
    Mini,
    /// Small.
    Small,
    /// Regular.
    Regular,
    /// Large.
    Large,
    /// Extra large.
    ExtraLarge,
}

/// A density (S7.3).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Density {
    /// The default density.
    Default,
    /// Compact.
    Compact,
    /// A density step the preset declares (Material's).
    Keyed(Key),
}

/// A breakpoint (S7.3, R16): while it holds, the connector adds its variant
/// to the element's variants; the platform's own conditions, not inverted.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Breakpoint {
    /// The size it measures.
    pub measured: Measured,
    /// The width test; `None`: none.
    pub width: Option<(Comparison, Value<Px>)>,
    /// The height test; `None`: none.
    pub height: Option<(Comparison, Value<Px>)>,
    /// How the two tests combine.
    pub combine: Combine,
    /// The variant added while it holds.
    pub variant: VariantRef,
}

impl Breakpoint {
    /// The breakpoint adding `variant` while `measured`'s tests, combined by
    /// `combine`, hold; its tests then assigned.
    #[must_use]
    pub fn new(measured: Measured, combine: Combine, variant: VariantRef) -> Breakpoint {
        Breakpoint {
            measured,
            width: None,
            height: None,
            combine,
            variant,
        }
    }
}

/// A size comparison: `≥`, `≤`, `>`, `<`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Comparison {
    /// At least.
    AtLeast,
    /// At most.
    AtMost,
    /// Above.
    Above,
    /// Below.
    Below,
}

/// How a breakpoint's two tests combine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Combine {
    /// Both stated tests must hold.
    All,
    /// Either holds.
    Any,
}

/// The size a breakpoint measures: never a box its own variant sizes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Measured {
    /// The size the parent offers the element.
    Allocation,
    /// The window's.
    Window,
    /// The allocation of a part of the widget that holds the content.
    Part(PartRef),
}

/// A window's size classes (R25.10); `None`: the platform states no class on
/// that axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SizeClasses {
    /// The horizontal class.
    pub horizontal: Option<SizeClass>,
    /// The vertical class.
    pub vertical: Option<SizeClass>,
}

/// A widget: a typed one, or a keyed component a preset declares (S5.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WidgetRef {
    /// A typed widget.
    Typed(WidgetKind),
    /// A keyed component.
    Component(Key),
}

/// A part of a widget: a typed widget's, or a keyed component's (S1).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PartRef {
    /// A typed widget's part, naming its widget.
    Typed(TypedPart),
    /// A keyed component's part.
    Component {
        /// The component.
        key: Key,
        /// Its part.
        part: Key,
    },
}

/// A widget, or one of its parts: the one field where a selector or a
/// substitution names either (S1); a part names its widget, so the two
/// cannot disagree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WidgetOrPart {
    /// A whole widget.
    Widget(WidgetRef),
    /// A part of a widget.
    Part(PartRef),
}

/// A variant: the catalogue's typed one, or a keyed one a preset declares.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VariantRef {
    /// A typed variant.
    Typed(Variant),
    /// A keyed variant.
    Keyed(Key),
}

/// A window's size class on one axis (S7.3), closed by the catalogue:
/// Material's window size classes, and Apple's `regular`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum SizeClass {
    /// Compact.
    Compact,
    /// Medium.
    Medium,
    /// Expanded.
    Expanded,
    /// Large.
    Large,
    /// Extra large.
    ExtraLarge,
    /// Regular.
    Regular,
}

/// A layout direction (S7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Direction {
    /// Left to right.
    Ltr,
    /// Right to left.
    Rtl,
}

/// A colour scheme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ThemeParts)]
#[parts(toml)]
pub enum Scheme {
    /// The light scheme.
    Light,
    /// The dark scheme.
    Dark,
}

impl Scheme {
    /// Every scheme, in declaration order.
    pub const ALL: [Scheme; 2] = [Scheme::Light, Scheme::Dark];

    /// Every scheme's name, in [`Scheme::ALL`]'s order.
    pub const NAMES: [&'static str; 2] = ["light", "dark"];

    /// The scheme's name in TOML and in expressions, its variant in
    /// snake_case.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Scheme::Light => "light",
            Scheme::Dark => "dark",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheme_names_match_variants() {
        assert_eq!(Scheme::ALL.map(Scheme::name), Scheme::NAMES);
    }
}

#[cfg(test)]
mod state_flags_tests {
    use super::{StateFlag, StateFlags};

    /// A set of flags compares as a set (S1), and every typed flag's name
    /// reads back as that flag.
    #[test]
    fn state_flags_compare_as_sets() {
        let a = StateFlags::new()
            .with(StateFlag::Hover)
            .with(StateFlag::Pressed);
        let b = StateFlags::new()
            .with(StateFlag::Pressed)
            .with(StateFlag::Hover);
        assert_eq!(a, b);
        assert_ne!(a, StateFlags::new().with(StateFlag::Hover));
        for (name, flag) in StateFlag::TYPED {
            assert_eq!(flag.name(), name);
        }
    }
}
