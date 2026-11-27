//! The compact rule engine: property patches, selectors over the draw-loop
//! query, and the rule set that evaluates them onto a base style.

use std::sync::Arc;

use super::color::Color;
use super::geometry::{Corners, Cursor, Edges, ShadowLayer, Stroke, Transform};
use super::motion::Transition;
use super::paint::{BlendMode, Paint};
use super::query::{
    ButtonKind, ButtonShape, ButtonVariant, ControlSize, GroupPosition, ListKind, ListVariant,
    Orientation, StyleQuery, Surface, WidgetState,
};
use super::style::{
    FocusIndicator, FontStyle, FontWidth, IconRenderingMode, IconStated, IconStyle, PartStated,
    PartStyle, RippleSpec, TextDecoration, TextStated, TextStyle,
};
use super::window::EdgeFlags;

/// One property of a [`PartStyle`], as a rule states it.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum PartProp {
    /// `fill`.
    Fill(Paint),
    /// `stroke`.
    Stroke(Stroke),
    /// `corners`.
    Corners(Corners),
    /// `shadows`.
    Shadows(Option<Arc<[ShadowLayer]>>),
    /// `opacity`.
    Opacity(f32),
    /// `transform`.
    Transform(Transform),
    /// `padding`, all four sides.
    Padding(Edges), // states all four sides
    /// `padding`, only the flagged sides.
    PaddingSides(Edges, EdgeFlags), // states only the flagged sides
    /// `margin`, all four sides.
    Margin(Edges),
    /// `margin`, only the flagged sides.
    MarginSides(Edges, EdgeFlags),
    /// `min_size`, per dimension: a `Some` replaces, a `None` keeps.
    MinSize([Option<f32>; 2]),
    /// `max_size`, per dimension: a `Some` replaces, a `None` keeps.
    MaxSize([Option<f32>; 2]),
    /// `focus`.
    Focus(Option<Arc<FocusIndicator>>),
    /// `ripple`.
    Ripple(Option<Arc<RippleSpec>>),
    /// `transition`.
    Transition(Option<Transition>),
    /// `cursor`.
    Cursor(Cursor),
}

/// One property of a [`TextStyle`], as a rule states it.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TextProp {
    /// `family`.
    Family(Arc<str>),
    /// `color`.
    Color(Color),
    /// `size_px`; it also clears `defined_size` (a rule that sets a pixel
    /// size carries no source unit).
    SizePx(f32), // also clears `defined_size` (§2)
    /// `weight`.
    Weight(u16),
    /// `style`.
    Style(FontStyle),
    /// `width`.
    Width(FontWidth),
    /// `line_height`.
    LineHeight(f32),
    /// `letter_spacing`.
    LetterSpacing(f32),
    /// `optical_size`.
    OpticalSize(Option<f32>),
    /// `decoration`.
    Decoration(TextDecoration),
    /// `blend_mode`.
    BlendMode(BlendMode),
}

/// One property of an [`IconStyle`], as a rule states it.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum IconProp {
    /// `size`.
    Size(f32),
    /// `color`.
    Color(Color),
    /// `opacity`.
    Opacity(f32),
    /// `weight`.
    Weight(u16),
    /// `rendering_mode`.
    RenderingMode(IconRenderingMode),
    /// `stroke_width`.
    StrokeWidth(Option<f32>),
    /// `blend_mode`.
    BlendMode(BlendMode),
}

/// Matches a widget's variant value `V` against a rule's matcher `M`.
pub trait VariantMatcher<V>: Copy + Default + std::fmt::Debug + PartialEq + Eq {
    /// Whether `value` matches.
    fn matches(&self, value: &V) -> bool;
}

impl VariantMatcher<()> for () {
    fn matches(&self, _value: &()) -> bool {
        true
    }
}

/// `None` matches every value, `Some(v)` only `v`.
impl<V: Copy + std::fmt::Debug + PartialEq + Eq> VariantMatcher<V> for Option<V> {
    fn matches(&self, value: &V) -> bool {
        self.is_none_or(|v| v == *value)
    }
}

/// Matches a [`ButtonKind`]; a `None` field matches every value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ButtonMatcher {
    /// The variant to match.
    pub variant: Option<ButtonVariant>,
    /// The shape to match.
    pub shape: Option<ButtonShape>,
}

impl VariantMatcher<ButtonKind> for ButtonMatcher {
    fn matches(&self, value: &ButtonKind) -> bool {
        self.variant.matches(&value.variant) && self.shape.matches(&value.shape)
    }
}

/// Matches a [`ListKind`]; a `None` field matches every value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ListMatcher {
    /// The list variant to match.
    pub variant: Option<ListVariant>,
    /// The alternate flag to match.
    pub alternate: Option<bool>,
}

impl VariantMatcher<ListKind> for ListMatcher {
    fn matches(&self, value: &ListKind) -> bool {
        self.variant.matches(&value.variant) && self.alternate.matches(&value.alternate)
    }
}

/// When a rule applies: every required state set, no forbidden state set,
/// and each stated axis equal to the query's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct RuleSelector<M = ()> {
    /// States that must all be set.
    pub required_states: WidgetState,
    /// States none of which may be set.
    pub forbidden_states: WidgetState,
    /// The control size, or any.
    pub size: Option<ControlSize>,
    /// The surface, or any.
    pub surface: Option<Surface>,
    /// The group position, or any.
    pub position: Option<GroupPosition>,
    /// The orientation, or any.
    pub orientation: Option<Orientation>,
    /// The variant matcher.
    pub matcher: M,
}

impl<M> RuleSelector<M> {
    /// Whether this selector matches `query` with its state normalised to
    /// `state`.
    fn matches<V>(&self, state: WidgetState, query: &StyleQuery<V>) -> bool
    where
        M: VariantMatcher<V>,
    {
        state.contains(self.required_states)
            && !state.intersects(self.forbidden_states)
            && self.size.is_none_or(|s| s == query.size)
            && self.surface.is_none_or(|s| s == query.surface)
            && self.position.is_none_or(|p| p == query.position)
            && self.orientation.is_none_or(|o| o == query.orientation)
            && self.matcher.matches(&query.variant)
    }
}

/// A rule: its selector and the range of its properties in the rule set's
/// flat property list.
#[derive(Clone, Debug, PartialEq)]
struct RuleEntry<M> {
    selector: RuleSelector<M>,
    start: u32,
    len: u32,
}

/// A base style and the ordered rules that patch it for a query.
///
/// [`eval`](Self::eval) normalises the query's state (`FOCUS_VISIBLE` adds
/// `FOCUSED`; in each exclusive group, in order, only the earliest set flag
/// stays; where that clears `FOCUSED`, it clears `FOCUS_VISIBLE` too),
/// clones the base, and applies the properties of every matching rule in
/// order, the last match winning, setting the `stated` bit of each property
/// applied. A rule whose `required_states` is not empty also sets the
/// property's `from_state` bit; one without required states clears it. Where
/// the query's `asked` is not empty, a rule sets the bit only if its required
/// states contain every asked flag, and one that requires other states leaves
/// the bit as it was; in this test `FOCUS_VISIBLE` is read as `FOCUSED`, in
/// the asked flags and in the rule's. It allocates nothing and never panics.
#[derive(Clone, Debug, PartialEq)]
pub struct RuleSet<T, P, M = ()> {
    base: T,
    exclusive: &'static [&'static [WidgetState]], // e.g. &[&[DISABLED, PRESSED, HOVER]]
    rules: Arc<[RuleEntry<M>]>,                   // RuleSelector<M> + (start: u32, len: u32)
    props: Arc<[P]>, // single flat allocation at resolve(); zero-alloc Clone
}

impl<T, P, M> RuleSet<T, P, M> {
    /// A rule set of `base` (its `stated` bits saying what it states), the
    /// ordered `exclusive` state groups, and `rules` in order, each a
    /// selector and the properties it applies; the tests' shorthand for
    /// [`RuleBuilder`].
    #[cfg(test)]
    pub(crate) fn new(
        base: T,
        exclusive: &'static [&'static [WidgetState]],
        rules: Vec<(RuleSelector<M>, Vec<P>)>,
    ) -> Self
    where
        T: Patched<P>,
    {
        rules
            .into_iter()
            .fold(
                RuleBuilder::with_exclusive(base, exclusive),
                |builder, (selector, props)| builder.rule(selector, props),
            )
            .build()
    }

    /// The indices of the rules that can never match: a rule whose required
    /// states hold two flags of one exclusive group (evaluation keeps only
    /// one of them), or whose required and forbidden states intersect. A
    /// required `FOCUS_VISIBLE` counts as requiring `FOCUSED` too, as
    /// evaluation adds it (step 0) and clears `FOCUS_VISIBLE` with it
    /// (step 1).
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "asserted empty by the presets' tests, from the next task of M0"
        )
    )]
    pub(crate) fn dead_rules(&self) -> Vec<usize> {
        let two_of_a_group = |required: WidgetState| {
            self.exclusive.iter().any(|group| {
                group
                    .iter()
                    .filter(|flag| required.contains(**flag))
                    .nth(1)
                    .is_some()
            })
        };
        self.rules
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                let s = &entry.selector;
                let required = if s.required_states.contains(WidgetState::FOCUS_VISIBLE) {
                    s.required_states | WidgetState::FOCUSED
                } else {
                    s.required_states
                };
                required.intersects(s.forbidden_states) || two_of_a_group(required)
            })
            .map(|(index, _)| index)
            .collect()
    }

    /// The rule set with `rules` appended after its own, in order, each a
    /// selector and the properties it applies (a theme patch's rules, spec
    /// §6, resolution step 3): evaluated after the builder's, so that where
    /// both match, the appended rule wins. The rule and property buffers are
    /// rebuilt, once for the whole batch; with no rule given, nothing is.
    pub(crate) fn append<Q>(&mut self, rules: impl IntoIterator<Item = (RuleSelector<M>, Q)>)
    where
        Q: IntoIterator<Item = P>,
        P: Clone,
        M: Clone,
    {
        let mut rules = rules.into_iter().peekable();
        if rules.peek().is_none() {
            return;
        }
        let mut entries = self.rules.to_vec();
        let mut props = self.props.to_vec();
        for (selector, new) in rules {
            let start = props.len();
            props.extend(new);
            let (start, len) = stored_range(start, props.len().saturating_sub(start));
            entries.push(RuleEntry {
                selector,
                start,
                len,
            });
        }
        self.rules = Arc::from(entries);
        self.props = Arc::from(props);
    }

    /// The rule set with `base` run on its base style and `prop` on every
    /// property of its rules (a theme patch's scales and font families, spec
    /// §6, resolution step 3); the property buffer is rebuilt.
    pub(crate) fn edit(&mut self, base: impl FnOnce(&mut T), prop: impl FnMut(&mut P))
    where
        P: Clone,
    {
        base(&mut self.base);
        let mut props = self.props.to_vec();
        props.iter_mut().for_each(prop);
        self.props = Arc::from(props);
    }

    /// The base style.
    #[cfg(feature = "serde")]
    pub(crate) fn base(&self) -> &T {
        &self.base
    }

    /// The ordered exclusive state groups.
    #[cfg(feature = "serde")]
    pub(crate) fn exclusive(&self) -> &'static [&'static [WidgetState]] {
        self.exclusive
    }

    /// The rules in evaluation order, each its selector and the properties
    /// it applies; a rule whose stored range is not in the property list
    /// applies none.
    #[cfg(feature = "serde")]
    pub(crate) fn entries(&self) -> impl Iterator<Item = (&RuleSelector<M>, &[P])> {
        self.rules.iter().map(|entry| {
            let props = usize::try_from(entry.start)
                .ok()
                .zip(usize::try_from(entry.len).ok())
                .and_then(|(start, len)| self.props.get(start..start.saturating_add(len)))
                .unwrap_or_default();
            (&entry.selector, props)
        })
    }

    /// Steps 0 to 4 of the evaluation contract, applying each property with
    /// `apply`, told what the rule as written does to its `from_state` bit.
    fn eval_with<V>(&self, query: &StyleQuery<V>, apply: fn(&mut T, &P, StateMark)) -> T
    where
        T: Clone,
        M: VariantMatcher<V>,
    {
        let state = normalize(query.state, self.exclusive);
        let asked = focus_folded(query.asked);
        let mut out = self.base.clone();
        for entry in self.rules.iter() {
            if !entry.selector.matches(state, query) {
                continue;
            }
            let (Ok(start), Ok(len)) = (usize::try_from(entry.start), usize::try_from(entry.len))
            else {
                continue;
            };
            let Some(props) = self.props.get(start..start.saturating_add(len)) else {
                continue;
            };
            let required = entry.selector.required_states;
            let mark = if required.is_empty() {
                StateMark::Clear
            } else if asked.is_empty() || focus_folded(required).contains(asked) {
                StateMark::Set
            } else {
                StateMark::Keep
            };
            for prop in props {
                apply(&mut out, prop, mark);
            }
        }
        out
    }
}

impl<M> RuleSet<PartStyle, PartProp, M> {
    /// The part's style for `query`.
    #[must_use]
    pub fn eval<V>(&self, query: &StyleQuery<V>) -> PartStyle
    where
        M: VariantMatcher<V>,
    {
        self.eval_with(query, apply_part)
    }
}

impl<M> RuleSet<TextStyle, TextProp, M> {
    /// The text's style for `query`.
    #[must_use]
    pub fn eval<V>(&self, query: &StyleQuery<V>) -> TextStyle
    where
        M: VariantMatcher<V>,
    {
        self.eval_with(query, apply_text)
    }
}

impl<M> RuleSet<IconStyle, IconProp, M> {
    /// The icon's style for `query`.
    #[must_use]
    pub fn eval<V>(&self, query: &StyleQuery<V>) -> IconStyle
    where
        M: VariantMatcher<V>,
    {
        self.eval_with(query, apply_icon)
    }
}

/// A style onto which a property of kind `P` applies, setting its `stated`
/// bit; as no state rule writes it, its `from_state` bit is cleared.
pub(crate) trait Patched<P> {
    /// Applies `prop`.
    fn apply(&mut self, prop: &P);

    /// Clears every `from_state` bit.
    fn clear_from_state(&mut self);
}

impl Patched<PartProp> for PartStyle {
    fn apply(&mut self, prop: &PartProp) {
        apply_part(self, prop, StateMark::Clear);
    }

    fn clear_from_state(&mut self) {
        self.from_state = PartStated::default();
    }
}

impl Patched<TextProp> for TextStyle {
    fn apply(&mut self, prop: &TextProp) {
        apply_text(self, prop, StateMark::Clear);
    }

    fn clear_from_state(&mut self) {
        self.from_state = TextStated::default();
    }
}

impl Patched<IconProp> for IconStyle {
    fn apply(&mut self, prop: &IconProp) {
        apply_icon(self, prop, StateMark::Clear);
    }

    fn clear_from_state(&mut self) {
        self.from_state = IconStated::default();
    }
}

/// Selectors as a preset writes them: `RuleSelector::on(HOVER)`, then the
/// axes it is limited to.
impl<M: Default> RuleSelector<M> {
    /// The selector of every query whose state holds all of `states`.
    pub(crate) fn on(states: WidgetState) -> Self {
        Self {
            required_states: states,
            ..Self::default()
        }
    }
}

impl<M> RuleSelector<M> {
    /// This selector, also requiring that none of `states` is set.
    pub(crate) fn except(mut self, states: WidgetState) -> Self {
        self.forbidden_states |= states;
        self
    }

    /// This selector, limited to `size`.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builders of presets with control sizes (plan, M1)"
        )
    )]
    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    /// This selector, limited to `surface`.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builders of presets with surface rules (plan, M1)"
        )
    )]
    pub(crate) fn surface(mut self, surface: Surface) -> Self {
        self.surface = Some(surface);
        self
    }

    /// This selector, limited to `position`.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builders of presets with grouped controls (plan, M1)"
        )
    )]
    pub(crate) fn position(mut self, position: GroupPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// This selector, limited to `orientation`.
    pub(crate) fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// This selector, limited to the variants `matcher` matches.
    pub(crate) fn matcher(mut self, matcher: M) -> Self {
        self.matcher = matcher;
        self
    }
}

/// The writer of a part's rule set: a base style and the exclusive state
/// groups, given once where the builder is created, then the rules in
/// order, their properties pushed into one flat list that
/// [`build`](Self::build) turns into the rule set's single allocation.
///
/// ```text
/// RuleBuilder::with_exclusive(PartStyle::default(), &[&[DISABLED, PRESSED, HOVER]])
///     .base([Fill(face), Stroke(frame), Corners(radius), Padding(edges), MinSize(size)])
///     .rule(RuleSelector::on(HOVER), [Fill(hover_face), Stroke(hover_frame)])
///     .rule(RuleSelector::on(NONE).orientation(Vertical), [MinSize(width)])
///     .rules([RuleSelector::on(CHECKED), RuleSelector::on(PRESSED)], [Fill(sunken)])
///     .build()
/// ```
#[derive(Debug)]
pub(crate) struct RuleBuilder<T, P, M = ()> {
    base: T,
    exclusive: &'static [&'static [WidgetState]],
    rules: Vec<RuleEntry<M>>,
    props: Vec<P>,
}

impl<T, P, M> RuleBuilder<T, P, M> {
    /// A builder whose base is `base`, with the `stated` bits it carries,
    /// and no exclusive state group.
    pub(crate) fn new(base: T) -> Self {
        Self::with_exclusive(base, &[])
    }

    /// A builder whose base is `base`, with the `stated` bits it carries,
    /// and the ordered exclusive state groups `exclusive` (evaluation
    /// contract, step 1).
    pub(crate) fn with_exclusive(base: T, exclusive: &'static [&'static [WidgetState]]) -> Self {
        Self {
            base,
            exclusive,
            rules: Vec::new(),
            props: Vec::new(),
        }
    }

    /// The base with `props` applied and stated.
    pub(crate) fn base(mut self, props: impl IntoIterator<Item = P>) -> Self
    where
        T: Patched<P>,
    {
        for prop in props {
            self.base.apply(&prop);
        }
        self
    }

    /// A rule: where `selector` matches, `props` apply.
    pub(crate) fn rule(
        self,
        selector: RuleSelector<M>,
        props: impl IntoIterator<Item = P>,
    ) -> Self {
        self.rules([selector], props)
    }

    /// One rule per selector, in order, each applying the same `props`,
    /// which are stored once; with no selector, nothing is stored.
    pub(crate) fn rules(
        mut self,
        selectors: impl IntoIterator<Item = RuleSelector<M>>,
        props: impl IntoIterator<Item = P>,
    ) -> Self {
        let mut selectors = selectors.into_iter().peekable();
        if selectors.peek().is_none() {
            return self;
        }
        let start = self.props.len();
        self.props.extend(props);
        let (start, len) = stored_range(start, self.props.len().saturating_sub(start));
        self.rules.extend(selectors.map(|selector| RuleEntry {
            selector,
            start,
            len,
        }));
        self
    }

    /// The rule set. Its base's `from_state` is empty, whatever bits the
    /// base given to [`new`](Self::new) or
    /// [`with_exclusive`](Self::with_exclusive) carried (evaluation
    /// contract, step 3).
    pub(crate) fn build(self) -> RuleSet<T, P, M>
    where
        T: Patched<P>,
    {
        let mut base = self.base;
        base.clear_from_state();
        RuleSet {
            base,
            exclusive: self.exclusive,
            rules: Arc::from(self.rules),
            props: Arc::from(self.props),
        }
    }
}

/// A rule's property range as stored, `(start, len)`; `(0, 0)` where either
/// does not fit in a `u32` (beyond `u32::MAX` properties), so that the rule
/// applies nothing rather than a range it does not own or a truncated one.
fn stored_range(start: usize, len: usize) -> (u32, u32) {
    match (u32::try_from(start), u32::try_from(len)) {
        (Ok(start), Ok(len)) => (start, len),
        _ => (0, 0),
    }
}

/// Steps 0 and 1 of the evaluation contract: `FOCUS_VISIBLE` adds `FOCUSED`;
/// then the exclusive groups in their order, each on the result of the one
/// before: in a group in which any flag is set, only the earliest set flag
/// stays. Where that clears `FOCUSED`, `FOCUS_VISIBLE` is cleared too.
fn normalize(state: WidgetState, exclusive: &[&[WidgetState]]) -> WidgetState {
    let mut state = if state.contains(WidgetState::FOCUS_VISIBLE) {
        state | WidgetState::FOCUSED
    } else {
        state
    };
    for group in exclusive {
        let Some(first) = group.iter().copied().find(|flag| state.intersects(*flag)) else {
            continue;
        };
        let keep = state & first;
        let all = group
            .iter()
            .fold(WidgetState::NONE, |acc, flag| acc | *flag);
        state = state.difference(all) | keep;
    }
    if state.contains(WidgetState::FOCUSED) {
        state
    } else {
        state.difference(WidgetState::FOCUS_VISIBLE)
    }
}

/// `state` with `FOCUS_VISIBLE` read as `FOCUSED`, as the `asked` test reads
/// the asked flags and a rule's required states.
fn focus_folded(state: WidgetState) -> WidgetState {
    if state.contains(WidgetState::FOCUS_VISIBLE) {
        state.difference(WidgetState::FOCUS_VISIBLE) | WidgetState::FOCUSED
    } else {
        state
    }
}

/// What a matching rule does to the `from_state` bits of the properties it
/// applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StateMark {
    /// Sets them: the rule requires a state (every asked flag, where the
    /// query asks).
    Set,
    /// Clears them: the rule requires no state.
    Clear,
    /// Leaves them: the rule requires states, not every asked flag.
    Keep,
}

/// The sides of `edges` flagged in `mask` written into `target`; returns
/// their bits (`[top, right, bottom, left]`).
fn apply_sides(
    target: &mut Edges,
    edges: &Edges,
    mask: EdgeFlags,
    bits: [PartStated; 4],
) -> PartStated {
    let [top, right, bottom, left] = bits;
    let mut written = PartStated::default();
    if mask.contains(EdgeFlags::TOP) {
        target.top = edges.top;
        written |= top;
    }
    if mask.contains(EdgeFlags::RIGHT) {
        target.right = edges.right;
        written |= right;
    }
    if mask.contains(EdgeFlags::BOTTOM) {
        target.bottom = edges.bottom;
        written |= bottom;
    }
    if mask.contains(EdgeFlags::LEFT) {
        target.left = edges.left;
        written |= left;
    }
    written
}

/// Each `Some` dimension of `size` written into `target`; returns `bit`
/// where a dimension was written, else nothing.
fn merge_size(
    target: &mut [Option<f32>; 2],
    size: &[Option<f32>; 2],
    bit: PartStated,
) -> PartStated {
    let [width, height] = target;
    let [new_width, new_height] = size;
    if new_width.is_some() {
        *width = *new_width;
    }
    if new_height.is_some() {
        *height = *new_height;
    }
    if new_width.is_some() || new_height.is_some() {
        bit
    } else {
        PartStated::default()
    }
}

/// The bits a size writes, which go to `from_state` alone: in `stated` each
/// dimension's `Option` says it.
const SIZE_BITS: PartStated = PartStated::MIN_SIZE.union(PartStated::MAX_SIZE);

const PADDING_BITS: [PartStated; 4] = [
    PartStated::PADDING_TOP,
    PartStated::PADDING_RIGHT,
    PartStated::PADDING_BOTTOM,
    PartStated::PADDING_LEFT,
];

const MARGIN_BITS: [PartStated; 4] = [
    PartStated::MARGIN_TOP,
    PartStated::MARGIN_RIGHT,
    PartStated::MARGIN_BOTTOM,
    PartStated::MARGIN_LEFT,
];

/// `prop` written into `style`, its `stated` bits set, and its `from_state`
/// bits set, cleared or left as `mark` says. A size has no `stated` bit; its
/// `from_state` bit ([`PartStated::MIN_SIZE`], [`PartStated::MAX_SIZE`])
/// follows the last rule that wrote either dimension and did not leave it.
fn apply_part(style: &mut PartStyle, prop: &PartProp, mark: StateMark) {
    let bits = match prop {
        PartProp::Fill(v) => {
            style.fill = v.clone();
            PartStated::FILL
        }
        PartProp::Stroke(v) => {
            style.stroke = v.clone();
            PartStated::STROKE
        }
        PartProp::Corners(v) => {
            style.corners = *v;
            PartStated::CORNERS
        }
        PartProp::Shadows(v) => {
            style.shadows = v.clone();
            PartStated::SHADOWS
        }
        PartProp::Opacity(v) => {
            style.opacity = *v;
            PartStated::OPACITY
        }
        PartProp::Transform(v) => {
            style.transform = *v;
            PartStated::TRANSFORM
        }
        PartProp::Padding(v) => apply_sides(&mut style.padding, v, EdgeFlags::ALL, PADDING_BITS),
        PartProp::PaddingSides(v, mask) => apply_sides(&mut style.padding, v, *mask, PADDING_BITS),
        PartProp::Margin(v) => apply_sides(&mut style.margin, v, EdgeFlags::ALL, MARGIN_BITS),
        PartProp::MarginSides(v, mask) => apply_sides(&mut style.margin, v, *mask, MARGIN_BITS),
        PartProp::MinSize(v) => merge_size(&mut style.min_size, v, PartStated::MIN_SIZE),
        PartProp::MaxSize(v) => merge_size(&mut style.max_size, v, PartStated::MAX_SIZE),
        PartProp::Focus(v) => {
            style.focus = v.clone();
            PartStated::FOCUS
        }
        PartProp::Ripple(v) => {
            style.ripple = v.clone();
            PartStated::RIPPLE
        }
        PartProp::Transition(v) => {
            style.transition = *v;
            PartStated::TRANSITION
        }
        PartProp::Cursor(v) => {
            style.cursor = *v;
            PartStated::CURSOR
        }
    };
    style.stated |= bits.difference(SIZE_BITS);
    style.from_state = match mark {
        StateMark::Set => style.from_state | bits,
        StateMark::Clear => style.from_state.difference(bits),
        StateMark::Keep => style.from_state,
    };
}

/// As [`apply_part`], for a text property.
fn apply_text(style: &mut TextStyle, prop: &TextProp, mark: StateMark) {
    let bits = match prop {
        TextProp::Family(v) => {
            style.family = v.clone();
            TextStated::FAMILY
        }
        TextProp::Color(v) => {
            style.color = *v;
            TextStated::COLOR
        }
        TextProp::SizePx(v) => {
            style.size_px = *v;
            style.defined_size = None;
            TextStated::SIZE_PX
        }
        TextProp::Weight(v) => {
            style.weight = *v;
            TextStated::WEIGHT
        }
        TextProp::Style(v) => {
            style.style = *v;
            TextStated::STYLE
        }
        TextProp::Width(v) => {
            style.width = *v;
            TextStated::WIDTH
        }
        TextProp::LineHeight(v) => {
            style.line_height = *v;
            TextStated::LINE_HEIGHT
        }
        TextProp::LetterSpacing(v) => {
            style.letter_spacing = *v;
            TextStated::LETTER_SPACING
        }
        TextProp::OpticalSize(v) => {
            style.optical_size = *v;
            TextStated::OPTICAL_SIZE
        }
        TextProp::Decoration(v) => {
            style.decoration = *v;
            TextStated::DECORATION
        }
        TextProp::BlendMode(v) => {
            style.blend_mode = *v;
            TextStated::BLEND_MODE
        }
    };
    style.stated |= bits;
    style.from_state = match mark {
        StateMark::Set => style.from_state | bits,
        StateMark::Clear => style.from_state.difference(bits),
        StateMark::Keep => style.from_state,
    };
}

/// As [`apply_part`], for an icon property.
fn apply_icon(style: &mut IconStyle, prop: &IconProp, mark: StateMark) {
    let bits = match prop {
        IconProp::Size(v) => {
            style.size = *v;
            IconStated::SIZE
        }
        IconProp::Color(v) => {
            style.color = *v;
            IconStated::COLOR
        }
        IconProp::Opacity(v) => {
            style.opacity = *v;
            IconStated::OPACITY
        }
        IconProp::Weight(v) => {
            style.weight = *v;
            IconStated::WEIGHT
        }
        IconProp::RenderingMode(v) => {
            style.rendering_mode = *v;
            IconStated::RENDERING_MODE
        }
        IconProp::StrokeWidth(v) => {
            style.stroke_width = *v;
            IconStated::STROKE_WIDTH
        }
        IconProp::BlendMode(v) => {
            style.blend_mode = *v;
            IconStated::BLEND_MODE
        }
    };
    style.stated |= bits;
    style.from_state = match mark {
        StateMark::Set => style.from_state | bits,
        StateMark::Clear => style.from_state.difference(bits),
        StateMark::Keep => style.from_state,
    };
}

#[cfg(test)]
mod tests;
