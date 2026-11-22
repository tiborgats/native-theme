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
/// applied. It allocates nothing and never panics.
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
    /// selector and the properties it applies.
    ///
    /// The provisional constructor of the 0.8.0 work, until its rule
    /// builder is written.
    #[must_use]
    pub fn new(
        base: T,
        exclusive: &'static [&'static [WidgetState]],
        rules: Vec<(RuleSelector<M>, Vec<P>)>,
    ) -> Self {
        let mut entries = Vec::with_capacity(rules.len());
        let mut props = Vec::new();
        for (selector, rule_props) in rules {
            let (start, len) = stored_range(props.len(), rule_props.len());
            props.extend(rule_props);
            entries.push(RuleEntry {
                selector,
                start,
                len,
            });
        }
        Self {
            base,
            exclusive,
            rules: Arc::from(entries),
            props: Arc::from(props),
        }
    }

    /// Steps 0 to 4 of the evaluation contract, applying each property with
    /// `apply`.
    fn eval_with<V>(&self, query: &StyleQuery<V>, apply: fn(&mut T, &P)) -> T
    where
        T: Clone,
        M: VariantMatcher<V>,
    {
        let state = normalize(query.state, self.exclusive);
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
            for prop in props {
                apply(&mut out, prop);
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

/// The sides of `edges` flagged in `mask` written into `target`, and their
/// bits (`[top, right, bottom, left]`) set in `stated`.
fn apply_sides(
    target: &mut Edges,
    stated: &mut PartStated,
    edges: &Edges,
    mask: EdgeFlags,
    bits: [PartStated; 4],
) {
    let [top, right, bottom, left] = bits;
    if mask.contains(EdgeFlags::TOP) {
        target.top = edges.top;
        *stated |= top;
    }
    if mask.contains(EdgeFlags::RIGHT) {
        target.right = edges.right;
        *stated |= right;
    }
    if mask.contains(EdgeFlags::BOTTOM) {
        target.bottom = edges.bottom;
        *stated |= bottom;
    }
    if mask.contains(EdgeFlags::LEFT) {
        target.left = edges.left;
        *stated |= left;
    }
}

/// Each `Some` dimension of `size` written into `target`.
fn merge_size(target: &mut [Option<f32>; 2], size: &[Option<f32>; 2]) {
    let [width, height] = target;
    let [new_width, new_height] = size;
    if new_width.is_some() {
        *width = *new_width;
    }
    if new_height.is_some() {
        *height = *new_height;
    }
}

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

fn apply_part(style: &mut PartStyle, prop: &PartProp) {
    match prop {
        PartProp::Fill(v) => {
            style.fill = v.clone();
            style.stated |= PartStated::FILL;
        }
        PartProp::Stroke(v) => {
            style.stroke = v.clone();
            style.stated |= PartStated::STROKE;
        }
        PartProp::Corners(v) => {
            style.corners = *v;
            style.stated |= PartStated::CORNERS;
        }
        PartProp::Shadows(v) => {
            style.shadows = v.clone();
            style.stated |= PartStated::SHADOWS;
        }
        PartProp::Opacity(v) => {
            style.opacity = *v;
            style.stated |= PartStated::OPACITY;
        }
        PartProp::Transform(v) => {
            style.transform = *v;
            style.stated |= PartStated::TRANSFORM;
        }
        PartProp::Padding(v) => {
            apply_sides(
                &mut style.padding,
                &mut style.stated,
                v,
                EdgeFlags::ALL,
                PADDING_BITS,
            );
        }
        PartProp::PaddingSides(v, mask) => {
            apply_sides(
                &mut style.padding,
                &mut style.stated,
                v,
                *mask,
                PADDING_BITS,
            );
        }
        PartProp::Margin(v) => {
            apply_sides(
                &mut style.margin,
                &mut style.stated,
                v,
                EdgeFlags::ALL,
                MARGIN_BITS,
            );
        }
        PartProp::MarginSides(v, mask) => {
            apply_sides(&mut style.margin, &mut style.stated, v, *mask, MARGIN_BITS);
        }
        PartProp::MinSize(v) => merge_size(&mut style.min_size, v),
        PartProp::MaxSize(v) => merge_size(&mut style.max_size, v),
        PartProp::Focus(v) => {
            style.focus = v.clone();
            style.stated |= PartStated::FOCUS;
        }
        PartProp::Ripple(v) => {
            style.ripple = v.clone();
            style.stated |= PartStated::RIPPLE;
        }
        PartProp::Transition(v) => {
            style.transition = *v;
            style.stated |= PartStated::TRANSITION;
        }
        PartProp::Cursor(v) => {
            style.cursor = *v;
            style.stated |= PartStated::CURSOR;
        }
    }
}

fn apply_text(style: &mut TextStyle, prop: &TextProp) {
    match prop {
        TextProp::Family(v) => {
            style.family = v.clone();
            style.stated |= TextStated::FAMILY;
        }
        TextProp::Color(v) => {
            style.color = *v;
            style.stated |= TextStated::COLOR;
        }
        TextProp::SizePx(v) => {
            style.size_px = *v;
            style.defined_size = None;
            style.stated |= TextStated::SIZE_PX;
        }
        TextProp::Weight(v) => {
            style.weight = *v;
            style.stated |= TextStated::WEIGHT;
        }
        TextProp::Style(v) => {
            style.style = *v;
            style.stated |= TextStated::STYLE;
        }
        TextProp::Width(v) => {
            style.width = *v;
            style.stated |= TextStated::WIDTH;
        }
        TextProp::LineHeight(v) => {
            style.line_height = *v;
            style.stated |= TextStated::LINE_HEIGHT;
        }
        TextProp::LetterSpacing(v) => {
            style.letter_spacing = *v;
            style.stated |= TextStated::LETTER_SPACING;
        }
        TextProp::OpticalSize(v) => {
            style.optical_size = *v;
            style.stated |= TextStated::OPTICAL_SIZE;
        }
        TextProp::Decoration(v) => {
            style.decoration = *v;
            style.stated |= TextStated::DECORATION;
        }
        TextProp::BlendMode(v) => {
            style.blend_mode = *v;
            style.stated |= TextStated::BLEND_MODE;
        }
    }
}

fn apply_icon(style: &mut IconStyle, prop: &IconProp) {
    match prop {
        IconProp::Size(v) => {
            style.size = *v;
            style.stated |= IconStated::SIZE;
        }
        IconProp::Color(v) => {
            style.color = *v;
            style.stated |= IconStated::COLOR;
        }
        IconProp::Opacity(v) => {
            style.opacity = *v;
            style.stated |= IconStated::OPACITY;
        }
        IconProp::Weight(v) => {
            style.weight = *v;
            style.stated |= IconStated::WEIGHT;
        }
        IconProp::RenderingMode(v) => {
            style.rendering_mode = *v;
            style.stated |= IconStated::RENDERING_MODE;
        }
        IconProp::StrokeWidth(v) => {
            style.stroke_width = *v;
            style.stated |= IconStated::STROKE_WIDTH;
        }
        IconProp::BlendMode(v) => {
            style.blend_mode = *v;
            style.stated |= IconStated::BLEND_MODE;
        }
    }
}

#[cfg(test)]
mod tests;
