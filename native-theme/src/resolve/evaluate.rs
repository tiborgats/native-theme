//! The resolve pass's evaluation (S2.2, S3.3, S8, S9.4): the steps a
//! resolution kept, evaluated into resolved values for one element in one
//! scope.
//!
//! Every block resolves from the steps of its place ([`Resolve`], which the
//! `ThemeParts` derive writes), field by field, each field by its shape: a
//! `Value` is the last step's, its expression evaluated where it is used; a
//! struct block's `Fields` patches apply over the block below; a paint's
//! `Append` adds layers; a map merges by key; a keyed list by its items'
//! keys. A `Fields` patch or an `Append` over a reference composes lazily:
//! the referenced value's own steps and the patches kept on the reference,
//! each at its position ([`compose::composed`]). A reference is followed
//! through the scope's bindings (S3.3), to the tokens, the inputs, the
//! element's other parts and its widget's values; a cycle, or a chain longer
//! than S2.2 allows, is refused at evaluation as S12 refuses it, the value
//! then not drawn (validation keeps both from a stack, S12).

use std::any::{Any, TypeId};
use std::cell::Cell;

use super::compose::{self, Fold, Found, Locate, Target};
use super::{ContentSizeCategory, KdeLists, Pos, ResolvedTheme};
use crate::IconSet;
use crate::blocks::motion::Loop;
use crate::blocks::outline::{DashPattern, Radius, ShapeKind, full};
use crate::blocks::paint::{
    BlendMode, Paint, PaintLayer, PaintPatch, ResolvedPaint, ResolvedPaintKind, ResolvedPaintLayer,
};
use crate::blocks::part_style::{Binding, PartStyle, ResolvedBinding, RoleRef};
use crate::blocks::shadow::{ResolvedShadow, Shadow, ShadowLayer};
use crate::blocks::text::{Font, ResolvedFont, ResolvedTextStyle, TextRole, TextStyle};
use crate::blocks::{Block, BlockPatch, FieldsBlock, KeyedItem, merge_by_key};
use crate::color::Color;
use crate::declare::ScalarUnit;
use crate::expr::{
    Bound, ElementDatum, Env, Expr, FieldType, Literal, MAX_CHAIN, MAX_DEPTH, Name, Ref, Ty, Val,
    bounded,
};
use crate::inputs::InputName;
use crate::key::Key;
use crate::platform::android_non_linear::{
    PROPORTIONAL_LINE_HEIGHT_ABOVE, android_non_linear, android_non_linear_inverse,
};
use crate::platform::dynamic_type::{DEFAULT_CATEGORY, DynamicTypeSize, dynamic_type};
use crate::platform::win_ui::win_ui;
use crate::query::{Element, Scope, ScopeEntry, instance_entries, own_entries};
use crate::roles::SurfaceSet;
use crate::selector::{PartRef, Selector};
use crate::tokens::{KdeColorEffect, KdeContrastEffect, KdeIntensityEffect, TextScaling};
use crate::toml::read::{MapKey, Named};
use crate::units::{Axes, Deg, FontSize, Fraction, Length, Ms, Px, Scalar, Sides};
use crate::validate::{TypeTarget, type_env};
use crate::value::{SurfaceMap, Value};
use crate::widgets::{PartPresence, TypedPart, WidgetKind, WidgetValuesPatch};

/// A struct block resolved from the steps of its place (S8), which the
/// `ThemeParts` derive writes.
pub(crate) trait Resolve: Block {
    /// The block `folds` state, in order: a whole block, and the `Fields`
    /// patches over it. `None` where a field that cannot be absent has no
    /// value, which validation rules out (S2.2: such a field is missing).
    fn resolve<'t: 'f, 'f>(folds: &[Fold<'f>], cx: &mut Cx<'t>) -> Option<Self::Resolved>;
}

/// A block stated whole, resolved: its `Value`s evaluated.
pub(crate) trait ResolveWhole: Block {
    /// `self`, stated by a step at `pos`, resolved.
    fn resolve_whole<'t: 'f, 'f>(&'f self, pos: Pos, cx: &mut Cx<'t>) -> Option<Self::Resolved>;
}

/// A struct block's patch resolved over the block beneath it (a keyframe's
/// part style over its part's, S2.7): each field it sets resolved, the
/// others `None`.
pub(crate) trait ResolvePatch: FieldsBlock {
    /// The patch resolved.
    type ResolvedPatch: Default;

    /// `patch`, its step `fold`, resolved over the steps `beneath`: each
    /// field it sets from the steps beneath the field and its own after
    /// them, `None` where the field beneath resolves `Absent`, and what is
    /// absent within the field beneath kept absent ([`KeepAbsent`]).
    fn resolve_patch<'t: 'f, 'f>(
        patch: &'f Self::Patch,
        fold: Fold<'f>,
        beneath: &[Fold<'f>],
        cx: &mut Cx<'t>,
    ) -> Self::ResolvedPatch;
}

/// A struct block's resolved value a keyframe's patch resolved over the
/// part's (S2.7), which the `ThemeParts` derive writes: a keyframe animates
/// what is drawn and never draws what is not.
pub(crate) trait KeepAbsent {
    /// Every field, at any depth, that `beneath`, the part's own value, does
    /// not draw left not drawn.
    fn keep_absent(&mut self, beneath: &Self);
}

/// Where a keyframe's step lies among a place's steps (S2.7): after every
/// step of the theme, so it applies over the part's resolved style, and
/// before the step a fact states in the facts gate, after every other
/// (S17).
const KEYFRAME: Pos = Pos {
    rule: true,
    layer: usize::MAX,
    index: 0,
};

/// A value a `Value` holds that is no block: a number, a colour, a name, a
/// text (S2.2's operand types).
pub(crate) trait Leaf: Any + Clone {
    /// The value an evaluated expression gives, if it is of this type.
    fn from_val(val: &Val) -> Option<Self>;

    /// The value as an operand.
    fn to_val(&self) -> Val;

    /// The type a call stated on such a field takes, and how the field limits
    /// it; `None` where no call is allowed (S2.2's typing).
    fn field(signed: bool) -> Option<FieldType>;

    /// Whether the value is a keyed scalar, whose call its declaration's
    /// unit types (S4.6).
    const KEYED_SCALAR: bool = false;

    /// A keyed scalar's value as this type; none but `Scalar` is one.
    fn from_scalar(_: Scalar) -> Option<Self> {
        None
    }
}

impl Leaf for Color {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Color(c) => Some(*c),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        Val::Color(*self)
    }

    fn field(_: bool) -> Option<FieldType> {
        Some(FieldType::new(Ty::Color, Bound::Unbounded))
    }
}

impl Leaf for Px {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Length(px) | Val::FontSize(px) => Some(*px),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        Val::Length(*self)
    }

    fn field(signed: bool) -> Option<FieldType> {
        let bound = if signed {
            Bound::Unbounded
        } else {
            Bound::NonNegative
        };
        Some(FieldType::new(Ty::Length, bound))
    }
}

impl Leaf for Ms {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Duration(ms) => Some(*ms),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        Val::Duration(*self)
    }

    fn field(_: bool) -> Option<FieldType> {
        Some(FieldType::new(Ty::Duration, Bound::NonNegative))
    }
}

impl Leaf for Deg {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Angle(deg) => Some(*deg),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        Val::Angle(*self)
    }

    fn field(_: bool) -> Option<FieldType> {
        Some(FieldType::new(Ty::Angle, Bound::Unbounded))
    }
}

/// A plain number's magnitude.
fn number(val: &Val) -> Option<f32> {
    match val {
        Val::Number(x) => Some(*x),
        _ => None,
    }
}

impl Leaf for f32 {
    fn from_val(val: &Val) -> Option<Self> {
        number(val)
    }

    fn to_val(&self) -> Val {
        Val::Number(*self)
    }

    fn field(_: bool) -> Option<FieldType> {
        Some(FieldType::new(Ty::Number, Bound::Unbounded))
    }
}

impl Leaf for Fraction {
    fn from_val(val: &Val) -> Option<Self> {
        number(val).map(Fraction)
    }

    fn to_val(&self) -> Val {
        Val::Number(self.0)
    }

    fn field(_: bool) -> Option<FieldType> {
        Some(FieldType::new(Ty::Number, Bound::Fraction))
    }
}

/// An integer field's value: its bound has floored the number and clamped it
/// to the type's range (S2.2's *Totality*), so the cast keeps it.
impl Leaf for u16 {
    fn from_val(val: &Val) -> Option<Self> {
        number(val).map(|x| x as u16)
    }

    fn to_val(&self) -> Val {
        Val::Number(f32::from(*self))
    }

    fn field(_: bool) -> Option<FieldType> {
        Some(FieldType::new(
            Ty::Number,
            Bound::Integer {
                max: u32::from(u16::MAX),
            },
        ))
    }
}

/// As `u16`'s.
impl Leaf for u32 {
    fn from_val(val: &Val) -> Option<Self> {
        number(val).map(|x| x as u32)
    }

    fn to_val(&self) -> Val {
        Val::Number(*self as f32)
    }

    fn field(_: bool) -> Option<FieldType> {
        Some(FieldType::new(Ty::Number, Bound::Integer { max: u32::MAX }))
    }
}

impl Leaf for bool {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Bool(b) => Some(*b),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        Val::Bool(*self)
    }

    fn field(_: bool) -> Option<FieldType> {
        None
    }
}

impl Leaf for String {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Text(text) => Some(text.clone()),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        Val::Text(self.clone())
    }

    fn field(_: bool) -> Option<FieldType> {
        None
    }
}

impl Leaf for Key {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Name(Name::Key(key)) => Some(key.clone()),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        Val::Name(Name::Key(self.clone()))
    }

    fn field(_: bool) -> Option<FieldType> {
        None
    }
}

impl Leaf for Scalar {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Length(px) => Some(Scalar::Px(*px)),
            Val::Duration(ms) => Some(Scalar::Ms(*ms)),
            Val::Angle(deg) => Some(Scalar::Deg(*deg)),
            Val::Number(x) => Some(Scalar::Number(*x)),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        match self {
            Scalar::Px(px) => Val::Length(*px),
            Scalar::Ms(ms) => Val::Duration(*ms),
            Scalar::Deg(deg) => Val::Angle(*deg),
            Scalar::Fraction(Fraction(x)) | Scalar::Number(x) => Val::Number(*x),
        }
    }

    /// A keyed scalar's unit is its declaration's (S4.6): no call is typed
    /// here.
    fn field(_: bool) -> Option<FieldType> {
        None
    }

    const KEYED_SCALAR: bool = true;

    fn from_scalar(scalar: Scalar) -> Option<Self> {
        Some(scalar)
    }
}

/// A leaf no call or operand reads: a value stated, or reached by a plain
/// reference (S2.2: an enum of data, a set's name).
macro_rules! opaque_leaf {
    ($($ty:ty),*) => {
        $(
            impl Leaf for $ty {
                fn from_val(_: &Val) -> Option<Self> {
                    None
                }

                fn to_val(&self) -> Val {
                    Val::Absent
                }

                fn field(_: bool) -> Option<FieldType> {
                    None
                }
            }
        )*
    };
}

opaque_leaf!(DashPattern, ShapeKind, IconSet);

impl<E: Named + Clone> Leaf for E {
    fn from_val(val: &Val) -> Option<Self> {
        match val {
            Val::Name(Name::Variant(name)) => E::from_name(name.as_str()),
            _ => None,
        }
    }

    fn to_val(&self) -> Val {
        match Key::new(self.name()) {
            Ok(key) => Val::Name(Name::Variant(key)),
            Err(_) => Val::Absent,
        }
    }

    fn field(_: bool) -> Option<FieldType> {
        None
    }
}

/// An `Option` held in a `Value` (`Value<Option<u32>>`, S6.1), reached by a
/// plain reference only: a value of `T` as an operand is `Some`, and no
/// operand is its `None`, which no call reads (S2.2).
impl<T: Leaf> Leaf for Option<T> {
    fn from_val(val: &Val) -> Option<Self> {
        T::from_val(val).map(Some)
    }

    fn to_val(&self) -> Val {
        self.as_ref().map_or(Val::Absent, Leaf::to_val)
    }

    fn field(_: bool) -> Option<FieldType> {
        None
    }
}

/// A place a reference names, or the value it gives itself.
enum Answer {
    /// A place of a target: its steps, the bases' alone where `bases_only`
    /// (`active(…)`, S2.2).
    Place {
        target: Target,
        path: Vec<String>,
        bases_only: bool,
    },
    /// A value the reference gives without a place: a binding's, an
    /// element's datum, a font metric, the scheme.
    Val(Val),
}

/// Why evaluation refused to follow a reference (S12's checks, which keep
/// both from a validated stack).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Refusal {
    /// The reference leads back to a place being evaluated.
    Cycle {
        /// The place's target.
        target: Target,
        /// The place.
        path: String,
    },
    /// A chain of references longer than S2.2 allows, or an expression
    /// nested deeper.
    TooDeep {
        /// The place's target.
        target: Target,
        /// The place.
        path: String,
    },
}

/// A value evaluated, and what evaluation refused on the way.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Evaluated<T> {
    /// The value.
    pub(crate) value: T,
    /// The refusals, in the order met.
    pub(crate) refusals: Vec<Refusal>,
    /// The places whose evaluation read a synthetic layer's step (S17's
    /// synthetic route, ADD-A128); none outside it.
    pub(crate) reached: Vec<Reached>,
}

/// A place whose evaluation read a step of a synthetic layer, the stand-in
/// the synthetic route states a preset's listed path with (S17, ADD-A128):
/// the place evaluated outside every reference followed, and whether the
/// step decided that place itself rather than an operand it reads.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Reached {
    /// The place's target.
    pub(crate) target: Target,
    /// The place, below its target.
    pub(crate) path: Vec<String>,
    /// The step decided the place itself, no reference followed.
    pub(crate) direct: bool,
    /// The step reaches the places below the place too; a step holding a
    /// map merged by key reaches the map's place alone, each entry being
    /// reached by the steps that state it (S6.1, S17 *Where a listed path
    /// is found*, ADD-A144).
    pub(crate) below: bool,
    /// The fewest segments a place deciding by this record has: a step
    /// deciding an entry of a map that a step of another layer holds too
    /// decides no place above the entry, the map being stated by that
    /// layer (ADD-A144).
    pub(crate) decides_from: usize,
}

impl Reached {
    /// Whether the place lies at `path` of `target`, above it or below it
    /// (below it only where the step reaches the places below its own).
    pub(crate) fn meets(&self, target: Target, path: &[String]) -> bool {
        let shared = self.path.len().min(path.len());
        self.target == target
            && self.path.get(..shared) == path.get(..shared)
            && (self.below || path.len() <= self.path.len())
    }

    /// Whether the step decides the place at `path`, one it meets
    /// ([`Reached::meets`]).
    pub(crate) fn decides(&self, path: &[String]) -> bool {
        self.direct && path.len() >= self.decides_from
    }
}

/// The text style a value lies in, for its `font_size` and `line_height`
/// operands (S2.2: the style's own), each with whether its evaluation read
/// a synthetic layer's step.
#[derive(Clone, Debug)]
struct TextFrame {
    font: Option<ResolvedFont>,
    size: Option<Px>,
    line_height: Option<Length>,
    font_reached: bool,
    line_height_reached: bool,
}

/// The text scaling in force (S9.4): the preset's `token.text.scaling`, with
/// what its curve reads.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Scaling {
    /// A product by the factor.
    Linear(f32),
    /// Android's curve at the factor, the font scale.
    AndroidNonLinear(f32),
    /// Apple's table at the category; the factor is not read.
    DynamicType(ContentSizeCategory),
    /// WinUI's curve at the factor, on a font size alone.
    WinUi(f32),
}

/// What a font size scaled lies in (S9.4): each kind follows its own rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scaled {
    /// A `Font.size`.
    Size,
    /// A text style's `line_height`.
    LineHeight,
    /// A letter or word spacing, or a size read as a value of its own.
    Spacing,
}

/// The query's evaluation context (S8): the resolved theme, the element and
/// the scope, the place being evaluated, and the references being followed.
pub(crate) struct Cx<'t> {
    theme: &'t ResolvedTheme,
    scope: &'t Scope,
    element: Option<&'t Element<'t>>,
    /// Which of a part's kept rules apply to the element (S8 step 3).
    applies: &'t dyn Fn(&Selector) -> bool,
    /// What the place's steps state values of.
    target: Target,
    /// The place being evaluated, below its target.
    path: Vec<String>,
    /// The scope's entries outside the bindings `chain` holds.
    outer: &'t [ScopeEntry],
    /// The scope's entries outside the queried element's own parts: the
    /// scope its parts are queried in below their element's (S2.2's
    /// `widget.<part>`).
    element_outer: &'t [ScopeEntry],
    /// The bindings of the element's parts in scope, outermost first: the
    /// parts enclosing the part whose place is evaluated, then its own
    /// (S3.3).
    chain: Vec<Bindings>,
    /// The part whose scope `chain` is; `None`: no part's.
    chain_part: Option<TypedPart>,
    /// The part whose `bindings` are being evaluated: they are evaluated in
    /// the scope outside it, its own left out (S3.3).
    outside: Option<TypedPart>,
    /// The text styles the place lies in, innermost last.
    text: Vec<TextFrame>,
    /// The `scaling_role`s of the text styles the place lies in, innermost
    /// last, entered before their fonts, each with whether reading it read a
    /// synthetic layer's step; read under `DynamicType` alone (S9.4).
    roles: Vec<(Option<TextRole>, bool)>,
    /// The places whose references are being followed, each with whether
    /// it is read from its bases alone (inside `active(…)`): a role's
    /// active value read while the role itself is evaluated is no cycle
    /// (S12, ADD-A144: a port's `input.inactive.<set>.<key> =
    /// active(role.<set>.<role>)` under the role's inactive rule).
    visiting: Vec<(Target, String, bool)>,
    /// Inside an expression on a `Font.size`: font sizes read converted, not
    /// yet scaled (S9.4).
    unscaled: bool,
    /// Inside `active(…)`: every place read reads its bases alone, and no
    /// `Role` binding is followed (S2.2).
    bases_only: bool,
    /// The DPI and the text scaling, read once outside every chain of
    /// references (S9.4); `None` until read.
    sizes: Option<(Option<f32>, Option<Scaling>)>,
    /// Whether reading the DPI, and the scaling, read a synthetic layer's
    /// step.
    sizes_reached: (bool, bool),
    /// Inside a keyed scalar map whose units a declaration names (a
    /// material's or a keyed kind's `params`): the depth of its place and
    /// the declared units (S2.7, S10).
    params: Option<(usize, &'t [(Key, ScalarUnit)])>,
    /// A step stated after every other, or an input's value in place of the
    /// theme's: a fact's value evaluated where the preset's is (S17).
    extra: Option<Extra<'t>>,
    refusals: Vec<Refusal>,
    /// The places whose evaluation read a synthetic layer's step, in the
    /// order met (S17's synthetic route).
    reached: Vec<Reached>,
    /// The place evaluated outside every reference followed, while one is,
    /// and how many places are being followed where a step read decides it
    /// itself.
    origin: Option<(Target, Vec<String>, usize)>,
    /// Inside a keyframe's patch: a loop resolves to none, as a keyframe
    /// animates its part's style, not the loop it is a frame of (S2.7).
    keyframe: bool,
}

/// What the facts gate evaluates in place of the theme's own (S17): a step
/// after every other step of its target, kept for every element, or an
/// input's value.
#[derive(Clone, Copy)]
pub(crate) enum Extra<'t> {
    /// A step of a target, its fold at the target's root, and the place
    /// below the root its fact's key names.
    Step(Fold<'t>, &'t [String]),
    /// An input and its value under the scheme and contrast in force.
    Input(&'t InputName, &'t Value<Color>),
}

/// Whose declared parameters a keyed scalar map holds (S2.7, S10).
pub(crate) enum ParamsOf<'k> {
    /// A material's: `material_params`.
    Material,
    /// A keyed transition kind's: its `transition_kinds` entry.
    TransitionKind(&'k Key),
    /// A keyed presentation kind's: its `presentation_kinds` entry.
    PresentationKind(&'k Key),
}

/// One part's bindings in scope (S3.3): its surface set and its roles,
/// each role evaluated in the scope outside it.
#[derive(Clone, Debug)]
struct Bindings {
    part: TypedPart,
    surface: Option<SurfaceSet>,
    roles: Vec<(RoleRef, Option<Color>)>,
    /// Their evaluation read a synthetic layer's step.
    reached: bool,
}

impl Bindings {
    /// `part`'s bindings, a later of one role or the surface replacing the
    /// earlier (S6.1); `reached` where their evaluation read a synthetic
    /// layer's step.
    fn of(part: TypedPart, bindings: Vec<ResolvedBinding>, reached: bool) -> Bindings {
        let mut bound = Bindings {
            part,
            surface: None,
            roles: Vec::new(),
            reached,
        };
        for binding in bindings {
            match binding {
                ResolvedBinding::Surface(set) => bound.surface = Some(set),
                ResolvedBinding::Role { role, value } => {
                    bound.roles.retain(|(held, _)| *held != role);
                    bound.roles.push((role, value));
                }
            }
        }
        bound
    }
}

/// How an expression's font size literals scale (S9.4): each as a value of
/// its own, where the place scales.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Literals {
    /// Not scaled: the place does not scale, or nothing does.
    Unscaled,
    /// A product by the factor (`Linear`).
    Factor(f32),
    /// Android's curve at the font scale (`AndroidNonLinear`).
    AndroidNonLinear(f32),
    /// No rule read yet (`DynamicType` away from the default category):
    /// the expression gives no value.
    NoRule,
    /// WinUI's curve at the factor (`WinUi`, at a `Font.size`).
    WinUi(f32),
}

/// An evaluated expression's operands, which typed evaluation reads.
struct Operands<'l> {
    values: Vec<(Ref, Val)>,
    /// KDE's lists under the scheme in force (S9.1).
    kde_lists: &'l KdeLists,
    dpi: Option<f32>,
    literals: Literals,
    /// A font size literal was converted, or `round_pt` read the DPI.
    converted: Cell<bool>,
    /// A font size literal met [`Literals::NoRule`].
    no_rule: Cell<bool>,
}

impl Env for Operands<'_> {
    fn get(&self, reference: &Ref) -> Val {
        match self.values.iter().find(|(held, _)| held == reference) {
            Some((_, value)) => value.clone(),
            None => Val::Absent,
        }
    }

    /// A `<n>pt` or `<n>px` literal converted, and scaled where the place
    /// scales it; without a DPI (validation requires one, S9.4) non-finite,
    /// which evaluation takes as zero (S2.2's *Totality*).
    fn font_size(&self, size: FontSize) -> Px {
        self.converted.set(true);
        match (convert(size, self.dpi), self.literals) {
            (Some(px), Literals::Unscaled) => px,
            (Some(px), Literals::Factor(factor)) => Px(px.0 * factor),
            (Some(px), Literals::AndroidNonLinear(scale)) => Px(android_non_linear(px.0, scale)),
            (Some(px), Literals::WinUi(factor)) => Px(win_ui(px.0, factor)),
            (Some(_), Literals::NoRule) => {
                self.no_rule.set(true);
                Px(f32::NAN)
            }
            (None, _) => Px(f32::NAN),
        }
    }

    fn font_dpi(&self) -> Option<f32> {
        self.converted.set(true);
        self.dpi
    }

    fn kde_lists(&self) -> &KdeLists {
        self.kde_lists
    }
}

/// Where `path` names one of a text style's font sizes (S9.4), its
/// `Font.size`, line height or spacing, how many of its last names lie
/// below the style: the style's place is the rest, whose role and font
/// size the text scaling reads there (`Cx::style_path`).
pub(crate) fn style_font_size(path: &[&str]) -> Option<usize> {
    match path {
        [.., "font", "size"] => Some(2),
        [.., "line_height" | "letter_spacing" | "word_spacing"] => Some(1),
        _ => None,
    }
}

/// Whether the place `path` is one of a text style's font sizes
/// ([`style_font_size`]).
fn is_font_size(path: &[String]) -> bool {
    let names: Vec<&str> = path.iter().map(String::as_str).collect();
    style_font_size(&names).is_some()
}

/// A size in points or pixels in logical pixels (S9.4): `pt × dpi / 72`;
/// `None` for points without a DPI.
fn convert(size: FontSize, dpi: Option<f32>) -> Option<Px> {
    match size {
        FontSize::Pt(points) => dpi.map(|dpi| Px(points * dpi / 72.0)),
        FontSize::Px(px) => Some(Px(px)),
    }
}

/// The references an expression reads, `active(…)`'s whole, and the
/// tokens its calls read themselves (S2.2).
pub(crate) fn references(expr: &Expr, out: &mut Vec<Ref>) {
    match expr {
        Expr::Lit(_) => {}
        Expr::Ref(reference) => {
            if !out.contains(reference) {
                out.push(reference.clone());
            }
        }
        Expr::Call(func, args) => {
            args.iter().for_each(|arg| references(arg, out));
            for token in func.token_operands(args) {
                if !out.contains(token) {
                    out.push(token.clone());
                }
            }
        }
    }
}

/// How deep `expr` nests (S2.2): calls and `active(…)` around its innermost
/// operand.
fn depth(expr: &Expr) -> usize {
    match expr {
        Expr::Lit(_) => 0,
        Expr::Ref(Ref::Active(_)) => 1,
        Expr::Ref(_) => 0,
        Expr::Call(_, args) => args.iter().map(depth).fold(0, usize::max).saturating_add(1),
    }
}

impl From<Target> for TypeTarget {
    fn from(target: Target) -> TypeTarget {
        match target {
            Target::Roles => TypeTarget::Roles,
            Target::Tokens => TypeTarget::Tokens,
            Target::Part(part) => TypeTarget::Part(part),
            Target::Values(widget) => TypeTarget::Values(widget),
        }
    }
}

/// The type a call stated on a keyed scalar of `unit` takes (S2.2, S4.6):
/// a fraction's clamped, no other bound, a keyed scalar's sign no field's.
fn scalar_field(unit: ScalarUnit) -> FieldType {
    match unit {
        ScalarUnit::Px => FieldType::new(Ty::Length, Bound::Unbounded),
        ScalarUnit::Ms => FieldType::new(Ty::Duration, Bound::Unbounded),
        ScalarUnit::Deg => FieldType::new(Ty::Angle, Bound::Unbounded),
        ScalarUnit::Fraction => FieldType::new(Ty::Number, Bound::Fraction),
        ScalarUnit::Number => FieldType::new(Ty::Number, Bound::Unbounded),
    }
}

/// A keyed scalar of `unit` an evaluated call gives.
fn scalar_of(unit: ScalarUnit, val: &Val) -> Option<Scalar> {
    match (unit, val) {
        (ScalarUnit::Px, Val::Length(px)) => Some(Scalar::Px(*px)),
        (ScalarUnit::Ms, Val::Duration(ms)) => Some(Scalar::Ms(*ms)),
        (ScalarUnit::Deg, Val::Angle(deg)) => Some(Scalar::Deg(*deg)),
        (ScalarUnit::Fraction, Val::Number(x)) => Some(Scalar::Fraction(Fraction(*x))),
        (ScalarUnit::Number, Val::Number(x)) => Some(Scalar::Number(*x)),
        _ => None,
    }
}

/// The widget a typed part belongs to.
fn widget_of(part: TypedPart) -> WidgetKind {
    match part {
        TypedPart::Button(_) => WidgetKind::Button,
    }
}

/// A step's `Value`, held directly or as a patch's set member.
fn value_of<'f, X: Any>(fold: Fold<'f>) -> Option<&'f Value<X>> {
    fold.found.get::<Value<X>>().or_else(|| {
        fold.found
            .get::<Option<Value<X>>>()
            .and_then(Option::as_ref)
    })
}

/// The last step that states something at the place.
fn last<'f>(items: &[Fold<'f>]) -> Option<(usize, Fold<'f>)> {
    items
        .iter()
        .enumerate()
        .rev()
        .find(|(_, fold)| !matches!(fold.found, Found::Missing))
        .map(|(index, fold)| (index, *fold))
}

/// One scope entry's bindings, S3.3's lookup walks: its surface set, its
/// roles, and whether they read a synthetic layer's step.
type ScopeBindings<'s> = (Option<SurfaceSet>, &'s [(RoleRef, Option<Color>)], bool);

/// What a struct block's step is to its place.
enum Step<'f, B: FieldsBlock> {
    /// Sets the value: a stated block, a reference, `Absent` or `Auto`.
    Set(&'f Value<B>),
    /// The block, whole.
    Whole(&'f B),
    /// Fields over the block below.
    Fields(&'f B::Patch),
    /// A reference above the place.
    Through(&'f Expr, usize),
    /// `Absent` or `Auto` above the place.
    Gone,
}

fn block_step<'f, B: FieldsBlock + Any>(fold: Fold<'f>) -> Option<Step<'f, B>>
where
    B::Patch: Any,
{
    match fold.found {
        Found::Through { expr, rest } => Some(Step::Through(expr, rest)),
        Found::Gone => Some(Step::Gone),
        Found::Missing => None,
        found => {
            if let Some(value) = found.get::<Value<B>>() {
                Some(Step::Set(value))
            } else if let Some(patch) = found.get::<BlockPatch<B>>() {
                Some(match patch {
                    BlockPatch::Set(value) => Step::Set(value),
                    BlockPatch::Fields(fields) => Step::Fields(fields),
                })
            } else if let Some(block) = found.get::<B>() {
                Some(Step::Whole(block))
            } else {
                found.get::<B::Patch>().map(Step::Fields)
            }
        }
    }
}

/// What a paint's step is to its place.
enum PaintStep<'f> {
    /// Sets the paint.
    Set(&'f Value<Paint>),
    /// Layers appended over the paint below.
    Appended(&'f [PaintLayer]),
    /// A reference above the place.
    Through(&'f Expr, usize),
    /// `Absent` or `Auto` above the place.
    Gone,
}

fn paint_step(fold: Fold<'_>) -> Option<PaintStep<'_>> {
    Some(match fold.found {
        Found::Through { expr, rest } => PaintStep::Through(expr, rest),
        Found::Gone => PaintStep::Gone,
        found => match (found.get::<Value<Paint>>(), found.get::<PaintPatch>()) {
            (Some(value), _) | (None, Some(PaintPatch::Set(value))) => PaintStep::Set(value),
            (None, Some(PaintPatch::Append(layers))) => PaintStep::Appended(layers),
            (None, None) => return None,
        },
    })
}

/// What a `Value` of a map's step is to its place (S6.1).
enum MapStep<'f, K, V> {
    /// The value a whole block states: it replaces what lies below.
    Whole(&'f Value<Vec<(K, V)>>),
    /// A patch's: a stated map merges by key, anything else replaces.
    Patch(&'f Value<Vec<(K, V)>>),
    /// A reference above the place.
    Through(&'f Expr, usize),
    /// `Absent` or `Auto` above the place.
    Gone,
}

impl<K, V> MapStep<'_, K, V> {
    /// Whether the step sets the map, rather than merging entries into it.
    fn sets(&self) -> bool {
        !matches!(self, MapStep::Patch(Value::Stated(_)))
    }
}

/// A `Value` of a map's step at `fold`: a patch's held in its `Option`
/// (the derive locates it so), a whole block's held bare.
fn map_step<'f, K: 'static, V: 'static>(fold: Fold<'f>) -> Option<MapStep<'f, K, V>> {
    match fold.found {
        Found::Through { expr, rest } => Some(MapStep::Through(expr, rest)),
        Found::Gone => Some(MapStep::Gone),
        Found::Missing => None,
        found => match found.get::<Option<Value<Vec<(K, V)>>>>() {
            Some(held) => held.as_ref().map(MapStep::Patch),
            None => found.get::<Value<Vec<(K, V)>>>().map(MapStep::Whole),
        },
    }
}

impl<'t> Cx<'t> {
    /// The context of a query in `scope`, for `element` where one is
    /// queried, its part's kept rules those `applies` accepts.
    fn new(
        theme: &'t ResolvedTheme,
        scope: &'t Scope,
        element: Option<&'t Element<'t>>,
        applies: &'t dyn Fn(&Selector) -> bool,
        target: Target,
    ) -> Cx<'t> {
        Cx {
            theme,
            scope,
            element,
            applies,
            target,
            path: Vec::new(),
            outer: &scope.entries,
            element_outer: element.map_or(&scope.entries, |element| {
                let own = own_entries(&element.part, &scope.entries);
                scope
                    .entries
                    .get(..scope.entries.len().saturating_sub(own))
                    .unwrap_or_default()
            }),
            chain: Vec::new(),
            chain_part: None,
            outside: None,
            text: Vec::new(),
            roles: Vec::new(),
            visiting: Vec::new(),
            unscaled: false,
            bases_only: false,
            sizes: None,
            sizes_reached: (false, false),
            params: None,
            extra: None,
            refusals: Vec::new(),
            reached: Vec::new(),
            origin: None,
            keyframe: false,
        }
    }

    /// Records that the place being evaluated read a synthetic layer's
    /// step: `direct` where the step decides the place itself (S17's
    /// synthetic route, ADD-A128).
    fn record(&mut self, direct: bool) {
        let (target, path, depth) = match &self.origin {
            Some((target, path, depth)) => (*target, path.clone(), *depth),
            None => (self.target, self.path.clone(), 0),
        };
        self.reached.push(Reached {
            target,
            path,
            direct: direct && self.visiting.len() == depth,
            below: true,
            decides_from: 0,
        });
    }

    /// Records the step at `pos` read, where it is a synthetic layer's.
    fn touch(&mut self, pos: Pos) {
        if self.theme.synthetic.contains(&pos.layer) {
            self.record(true);
        }
    }

    /// Records the steps at `held`, those holding the map merged by key
    /// being evaluated, read at the map's place (S6.1, S17 *Where a listed
    /// path is found*, ADD-A144): where a synthetic layer's step holds it,
    /// the map's place reached, not the entries below it, which the steps
    /// stating them reach, and decided only where every step holding it is
    /// a synthetic layer's. Whether a step of another layer holds it.
    fn held_map(&mut self, held: &[Pos]) -> bool {
        let synthetic = |pos: &Pos| self.theme.synthetic.contains(&pos.layer);
        let all = held.iter().all(synthetic);
        if held.iter().any(synthetic) {
            self.record(all);
            let (target, path) = (self.target, &self.path);
            if let Some(last) = self.reached.last_mut()
                && last.target == target
                && last.path == *path
            {
                last.below = false;
            }
        }
        !all
    }

    /// Leaves the places recorded from `mark` below the map being
    /// evaluated, its entries' and their fields', deciding no place above
    /// an entry: a step of another layer holds the map, so that layer
    /// states it (S6.1, S17 *Where a listed path is found*, ADD-A144).
    fn entries_from(&mut self, mark: usize) {
        let depth = self.path.len().saturating_add(1);
        let (target, path) = (self.target, &self.path);
        for held in self.reached.iter_mut().skip(mark) {
            if held.target == target && held.path.len() >= depth && held.path.starts_with(path) {
                held.decides_from = held.decides_from.max(depth);
            }
        }
    }

    /// Leaves the places recorded from `mark` on undecided by a synthetic
    /// layer's step, still reached: a step of another layer merged into
    /// the place, so that layer states it too (S6.1, S17's synthetic route,
    /// ADD-A128).
    fn merged_from(&mut self, mark: usize) {
        for held in self.reached.iter_mut().skip(mark) {
            held.direct = false;
        }
    }

    /// Records a value read whose evaluation read a synthetic layer's step,
    /// where `reached`.
    fn reach(&mut self, reached: bool) {
        if reached {
            self.record(false);
        }
    }

    /// The places whose evaluation read a synthetic layer's step so far
    /// (S17's synthetic route, ADD-A128).
    pub(crate) fn reached(&self) -> &[Reached] {
        &self.reached
    }

    /// `f`'s value, and whether its evaluation read a synthetic layer's
    /// step, recorded for no place: a value read once and used by others.
    fn aside<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> (R, bool) {
        let held = self.reached.len();
        let out = f(self);
        let reached = self.reached.len() > held;
        self.reached.truncate(held);
        (out, reached)
    }

    /// `f`'s value, and whether its evaluation read a synthetic layer's
    /// step, recorded at its place too.
    fn noting<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> (R, bool) {
        let held = self.reached.len();
        let out = f(self);
        (out, self.reached.len() > held)
    }

    /// What the place's steps state values of.
    pub(crate) fn target(&self) -> Target {
        self.target
    }

    /// `f` evaluated at the place `name` below the current one.
    pub(crate) fn at<R>(&mut self, name: &str, f: impl FnOnce(&mut Self) -> R) -> R {
        let outside = match self.target {
            Target::Part(part) if name == "bindings" && self.path.is_empty() => Some(part),
            _ => self.outside,
        };
        let saved = std::mem::replace(&mut self.outside, outside);
        self.path.push(name.to_owned());
        let out = f(self);
        self.path.pop();
        self.outside = saved;
        out
    }

    /// `f` evaluated at a `params` map of `of`, whose entries a call types
    /// by their declared units, as validation does (S2.2, S2.7, S10).
    pub(crate) fn params<R>(&mut self, of: ParamsOf<'_>, f: impl FnOnce(&mut Self) -> R) -> R {
        let theme = self.theme;
        let decl = &theme.declarations;
        let kind = |kinds: &'t [(Key, Vec<(Key, ScalarUnit)>)], key: &Key| {
            kinds
                .iter()
                .find(|(held, _)| held == key)
                .map_or(&[][..], |(_, units)| units.as_slice())
        };
        let units = match of {
            ParamsOf::Material => decl.material_params.as_slice(),
            ParamsOf::TransitionKind(key) => kind(&decl.transition_kinds, key),
            ParamsOf::PresentationKind(key) => kind(&decl.presentation_kinds, key),
        };
        let saved = self.params.replace((self.path.len(), units));
        let out = f(self);
        self.params = saved;
        out
    }

    /// `f` evaluated within the text style `folds` state: its own font, size
    /// and line height the `font_size` and `line_height` operands there
    /// (S2.2).
    pub(crate) fn text_style<'f, R>(
        &mut self,
        folds: &[Fold<'f>],
        f: impl FnOnce(&mut Self) -> Option<R>,
    ) -> Option<R>
    where
        't: 'f,
    {
        let role = match self.sizes().1 {
            Some(Scaling::DynamicType(_)) => self.noting(|cx| {
                cx.at("scaling_role", |cx| {
                    cx.value::<Option<TextRole>>(&compose::children(folds, "scaling_role"), false)
                })
                .flatten()
            }),
            Some(Scaling::Linear(_) | Scaling::AndroidNonLinear(_) | Scaling::WinUi(_)) | None => {
                (None, false)
            }
        };
        self.roles.push(role);
        let (font, font_reached) = self.noting(|cx| {
            cx.at("font", |cx| {
                cx.block::<Font>(&compose::children(folds, "font"))
            })
        });
        let size = font.as_ref().and_then(|font| font.size);
        self.text.push(TextFrame {
            font,
            size,
            line_height: None,
            font_reached,
            line_height_reached: false,
        });
        let (line_height, line_height_reached) = self.noting(|cx| {
            cx.at("line_height", |cx| {
                cx.font_auto(&compose::children(folds, "line_height"), false)
            })
        });
        if let Some(frame) = self.text.last_mut() {
            frame.line_height = line_height;
            frame.line_height_reached = line_height_reached;
        }
        let out = f(self);
        self.text.pop();
        self.roles.pop();
        out
    }

    /// The font of the text style being resolved, as its frame resolved it
    /// once (`text_style`).
    pub(crate) fn framed_font(&self) -> Option<ResolvedFont> {
        self.text.last().and_then(|frame| frame.font.clone())
    }

    /// The line height of the text style being resolved, as its frame
    /// resolved it once.
    pub(crate) fn framed_line_height(&self) -> Option<Length> {
        self.text.last().and_then(|frame| frame.line_height)
    }

    /// `f` evaluated with font sizes converted from points and never
    /// text-scaled: a `Value<FontSize>` field outside a text style, a widget
    /// value's (S1, S9.4). No widget value of type `FontSize` exists; one
    /// added later must decide its scaling with S9.4, as read as an operand
    /// ([`Cx::operand`]) it would be text-scaled.
    pub(crate) fn unscaled<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let saved = std::mem::replace(&mut self.unscaled, true);
        let out = f(self);
        self.unscaled = saved;
        out
    }

    /// Whether a part's step applies to the element: a base always, a rule
    /// where `applies` accepts its selector.
    fn kept(&self, selector: Option<&Selector>) -> bool {
        selector.is_none_or(|selector| (self.applies)(selector))
    }

    /// Whether `fold` is the step of a fact the gate evaluates beside the
    /// theme's own ([`Extra::Step`]), or a step below it, read at the place
    /// its key names: a map stated there is the fact's value whole, while a
    /// key below an entry states that leaf alone (S17).
    fn is_fact(&self, fold: &Fold<'_>) -> bool {
        matches!(self.extra, Some(Extra::Step(extra, place))
            if extra.pos == fold.pos && extra.target == fold.target && self.path == place)
    }

    /// The steps of `target`'s root, in the stack's order: a part's those
    /// that apply to the element (S8 step 3), every applying step's fields,
    /// those kept beneath an `Absent` or `DrawnAs` included (S6.3).
    pub(crate) fn roots(&self, target: Target) -> Vec<Fold<'t>> {
        let mut out = self.own_roots(target);
        if let Some(Extra::Step(fold, _)) = self.extra
            && fold.target == target
        {
            out.push(fold);
        }
        out
    }

    /// The theme's own steps of `target`'s root, as [`Cx::roots`] gives
    /// them.
    fn own_roots(&self, target: Target) -> Vec<Fold<'t>> {
        let theme = self.theme;
        let fold = |pos, found| Fold { pos, target, found };
        match target {
            Target::Roles => theme
                .colors
                .iter()
                .map(|step| fold(step.pos, Found::Node(&step.patch)))
                .collect(),
            Target::Tokens => theme
                .tokens
                .iter()
                .map(|step| fold(step.pos, Found::Node(&*step.patch)))
                .collect(),
            Target::Part(part) => theme
                .part_steps(part)
                .iter()
                .filter(|step| self.kept(step.selector.as_ref()))
                .map(|step| fold(step.pos, Found::Node(&*step.patch.style)))
                .collect(),
            Target::Values(widget) => theme
                .values
                .iter()
                .filter(|(held, _)| *held == widget)
                .flat_map(|(_, steps)| steps)
                .filter(|step| self.kept(step.selector.as_ref()))
                .map(|step| match &step.patch {
                    WidgetValuesPatch::Button(values) => fold(step.pos, Found::Node(&**values)),
                })
                .collect(),
        }
    }

    /// The steps of `path` below `target`'s root, the bases' alone where
    /// `bases_only`.
    fn locate(&self, target: Target, path: &[String], bases_only: bool) -> Vec<Fold<'t>> {
        let segments: Vec<&str> = path.iter().map(String::as_str).collect();
        self.roots(target)
            .into_iter()
            .filter(|fold| !bases_only || !fold.pos.rule)
            .map(|fold| fold.below(&segments))
            .filter(|fold| !matches!(fold.found, Found::Missing))
            .collect()
    }

    /// `f` over the steps of the place `path` of `target`, evaluated there;
    /// refused where the place is being evaluated already (a cycle) or the
    /// chain of references would pass [`MAX_CHAIN`].
    fn follow<R>(
        &mut self,
        target: Target,
        path: Vec<String>,
        bases_only: bool,
        f: impl FnOnce(&mut Self, &[Fold<'t>]) -> Option<R>,
    ) -> Option<R> {
        let bases_only = bases_only || self.bases_only;
        let key = (target, path.join("."), bases_only);
        if self.visiting.contains(&key) {
            self.refusals.push(Refusal::Cycle {
                target,
                path: key.1,
            });
            return None;
        }
        if self.visiting.len() >= MAX_CHAIN {
            self.refusals.push(Refusal::TooDeep {
                target,
                path: key.1,
            });
            return None;
        }
        let folds = self.locate(target, &path, bases_only);
        // The place evaluated: the one being evaluated where there is one,
        // else (a reference read alone, a role's) the one followed.
        let entered = self.origin.is_none();
        if entered {
            self.origin = Some(if self.path.is_empty() {
                (target, path.clone(), self.visiting.len().saturating_add(1))
            } else {
                (self.target, self.path.clone(), self.visiting.len())
            });
        }
        self.visiting.push(key);
        let saved_bases = std::mem::replace(&mut self.bases_only, bases_only);
        let saved_target = std::mem::replace(&mut self.target, target);
        let saved_path = std::mem::replace(&mut self.path, path);
        let saved_text = std::mem::take(&mut self.text);
        let saved_roles = std::mem::take(&mut self.roles);
        let saved_params = self.params.take();
        let out = match target {
            // Another part of the element is evaluated in the scope it is
            // itself queried in (S2.2): the element's outer scope and the
            // bindings of the parts enclosing it, then its own.
            Target::Part(part) if self.element.is_some() && self.chain_part != Some(part) => {
                let saved_outer = std::mem::replace(&mut self.outer, self.element_outer);
                let saved_chain = std::mem::take(&mut self.chain);
                let saved_part = self.chain_part;
                let saved_outside = self.outside.take();
                self.chain = self.chain_of(part);
                self.chain_part = Some(part);
                let out = f(self, &folds);
                self.outside = saved_outside;
                self.chain_part = saved_part;
                self.chain = saved_chain;
                self.outer = saved_outer;
                out
            }
            _ => f(self, &folds),
        };
        self.bases_only = saved_bases;
        self.params = saved_params;
        self.text = saved_text;
        self.roles = saved_roles;
        self.path = saved_path;
        self.target = saved_target;
        self.visiting.pop();
        if entered {
            self.origin = None;
        }
        out
    }

    /// The bindings in scope for `part` within the element (S2.2, S3.3):
    /// those of the parts enclosing it, from its element's part inward, and
    /// its own, each evaluated in the scope outside it. A part outside the
    /// element takes only those of the parts below the nearest enclosing it
    /// whose entry the element's outer scope holds, which holds their
    /// bindings already, so none applies twice (S2.11); only the entries
    /// of the element's own widget instance count, never an outer instance
    /// of the same widget's ([`instance_entries`]).
    fn chain_of(&mut self, part: TypedPart) -> Vec<Bindings> {
        let head = self.element.and_then(|element| match &element.part {
            PartRef::Typed(own) => Some(own.element()),
            PartRef::Component { .. } => None,
        });
        let outer = match head {
            Some(head) => instance_entries(head, self.element_outer),
            None => self.element_outer,
        };
        let entered = |held: &TypedPart| {
            outer
                .iter()
                .any(|entry| entry.part == PartRef::Typed(*held))
        };
        let mut enclosing = part.enclosing();
        if let Some(at) = enclosing.iter().position(|held| Some(*held) == head) {
            enclosing.truncate(at.saturating_add(1));
        } else if let Some(at) = enclosing.iter().position(entered) {
            enclosing.truncate(at);
        }
        let mut chain = Vec::new();
        for held in enclosing.into_iter().rev() {
            self.chain = chain.clone();
            self.chain_part = Some(held);
            self.outside = Some(held);
            let path = vec!["bindings".to_owned()];
            let (bindings, reached) = self.aside(|cx| {
                cx.follow(Target::Part(held), path, false, |cx, folds| {
                    cx.keyed::<Binding>(folds)
                })
            });
            chain.push(Bindings::of(held, bindings.unwrap_or_default(), reached));
        }
        self.outside = None;
        chain
    }

    /// `f` over the steps of the place a reference above the current one
    /// leads to: what `expr` names, `rest` segments of the current place
    /// below it, composed with `items[at + 1..]`.
    fn through<'f, R>(
        &mut self,
        expr: &Expr,
        rest: usize,
        items: &[Fold<'f>],
        at: usize,
        origin: Target,
        f: impl FnOnce(&mut Self, &[Fold<'f>]) -> Option<R>,
    ) -> Option<R>
    where
        't: 'f,
    {
        let Expr::Ref(reference) = expr else {
            return None;
        };
        let Some(Answer::Place {
            target,
            mut path,
            bases_only,
        }) = self.reference(reference, origin)
        else {
            return None;
        };
        let tail = self
            .path
            .get(self.path.len().saturating_sub(rest)..)
            .unwrap_or_default()
            .to_vec();
        path.extend(tail);
        self.follow(target, path, bases_only, |cx, referenced| {
            let composed = compose::composed(referenced, items, at);
            f(cx, &composed)
        })
    }

    /// The place or value `reference` names, read from a step of `origin`
    /// (S2.2, S3.3); `None` where it names nothing there.
    fn reference(&mut self, reference: &Ref, origin: Target) -> Option<Answer> {
        let segments = |path: &[Key]| {
            path.iter()
                .map(|key| key.as_str().to_owned())
                .collect::<Vec<_>>()
        };
        let place = |target, path| Answer::Place {
            target,
            path,
            bases_only: false,
        };
        let text = || "text".to_owned();
        match reference {
            Ref::Role { .. } | Ref::GlobalRole(_) | Ref::KeyedRole(_) | Ref::Surface(_) => {
                let (answer, reached) = self.role(reference, false);
                self.reach(reached);
                Some(answer)
            }
            Ref::Active(inner) => {
                let (answer, reached) = self.role(inner, true);
                self.reach(reached);
                Some(answer)
            }
            Ref::Token(path) => Some(place(Target::Tokens, segments(path))),
            Ref::Input(path) => {
                let theme = self.theme;
                let extra = match self.extra {
                    Some(Extra::Input(name, value)) if name.segments() == path.as_slice() => {
                        Some(value)
                    }
                    _ => None,
                };
                // The input's own layer, its place in the stack for the
                // fold; a listed input's synthetic value is told apart by
                // the mark (below; S17, ADD-A132). The value evaluated in
                // its place, none.
                let (value, layer) = match extra {
                    Some(value) => (value, 0),
                    None => {
                        let value = theme
                            .inputs
                            .iter()
                            .find(|(name, _)| name.segments() == path.as_slice())
                            .map(|(_, value)| value)?;
                        let layer = theme
                            .input_layers
                            .iter()
                            .find(|(name, _)| name.segments() == path.as_slice())
                            .map_or(0, |(_, layer)| *layer);
                        (value, layer)
                    }
                };
                let fold = Fold {
                    pos: Pos {
                        rule: false,
                        layer,
                        index: 0,
                    },
                    target: Target::Roles,
                    found: Found::Leaf(value),
                };
                // An input reading itself, directly or through others, is a
                // cycle (S12), refused as any other.
                let key = (
                    Target::Roles,
                    format!("input.{}", segments(path).join(".")),
                    self.bases_only,
                );
                if self.visiting.contains(&key) {
                    self.refusals.push(Refusal::Cycle {
                        target: key.0,
                        path: key.1,
                    });
                    return None;
                }
                self.visiting.push(key);
                // A listed input's folded synthetic value in force (S17,
                // ADD-A132), read as a synthetic layer's step is.
                if extra.is_none()
                    && theme
                        .synthetic_inputs
                        .iter()
                        .any(|name| name.segments() == path.as_slice())
                {
                    self.record(true);
                }
                let value = self.operand(&[fold]);
                self.visiting.pop();
                value.map(Answer::Val)
            }
            Ref::Base => None,
            Ref::Scheme => Key::new(self.theme.scheme.name())
                .ok()
                .map(|key| Answer::Val(Val::Name(Name::Variant(key)))),
            Ref::FontSize => self
                .font_size_operand()
                .map(|px| Answer::Val(Val::Length(px))),
            Ref::LineHeight => self
                .line_height_operand()
                .map(|px| Answer::Val(Val::Length(px))),
            Ref::Element(datum) => self.datum(*datum, origin).map(Answer::Val),
            Ref::BodyFontHeight => {
                let metrics = self.element?.metrics;
                let body = vec![text(), "roles".to_owned(), "body".to_owned()];
                let style = self.follow(Target::Tokens, body, false, |cx, folds| {
                    cx.block::<TextStyle>(folds)
                })?;
                Some(Answer::Val(Val::Length(metrics.height(&style.font))))
            }
            Ref::Part(path) => match origin {
                Target::Part(_) => Some(place(origin, segments(path))),
                Target::Roles | Target::Tokens | Target::Values(_) => None,
            },
            Ref::Widget { part, path } => {
                let widget = match origin {
                    Target::Part(own) => widget_of(own),
                    Target::Values(widget) => widget,
                    Target::Roles | Target::Tokens => return None,
                };
                let other = TypedPart::from_names(widget, part.as_str())?;
                let path = segments(path);
                let names: Vec<&str> = path.iter().map(String::as_str).collect();
                match names.as_slice() {
                    // `.font_size` and `.line_height` are that part's text's
                    // (S2.2), its other font metrics its font's.
                    ["font_size"] => Some(place(
                        Target::Part(other),
                        vec![text(), "font".to_owned(), "size".to_owned()],
                    )),
                    ["line_height"] => {
                        let style = self.part_text(other)?;
                        let px = match style.line_height? {
                            Length::Px(px) => Px(px),
                            Length::Auto => self.element?.metrics.line_spacing(&style.font),
                        };
                        Some(Answer::Val(Val::Length(px)))
                    }
                    [name] if font_metric(name).is_some() => {
                        let datum = font_metric(name)?;
                        let style = self.part_text(other)?;
                        self.metric(datum, &style.font).map(Answer::Val)
                    }
                    _ => Some(place(Target::Part(other), path)),
                }
            }
            Ref::Values(name) => {
                let widget = match origin {
                    Target::Part(own) => widget_of(own),
                    Target::Values(widget) => widget,
                    Target::Roles | Target::Tokens => return None,
                };
                Some(place(
                    Target::Values(widget),
                    vec![name.as_str().to_owned()],
                ))
            }
            Ref::TextScale => Some(Answer::Val(Val::Number(self.theme.text_scaling_factor))),
        }
    }

    /// S3.3's two steps for a role reference: the bindings in scope, the
    /// queried part's own innermost (outside its own `bindings`, whose values
    /// are evaluated in the scope outside it), then its ancestors'; else the
    /// colour roles' value. `active(…)` (`bases`) follows the `Surface(S)`
    /// bindings only, and reads the bases (S2.2). With whether a binding it
    /// takes read a synthetic layer's step (S17's synthetic route).
    fn role(&self, reference: &Ref, bases: bool) -> (Answer, bool) {
        let bases = bases || self.bases_only;
        let joined = |path: &[Key]| path.iter().map(Key::as_str).collect::<Vec<_>>().join(".");
        let mut chain: Vec<ScopeBindings<'_>> = self
            .chain
            .iter()
            .rev()
            .filter(|bound| Some(bound.part) != self.outside)
            .map(|bound| (bound.surface, bound.roles.as_slice(), bound.reached))
            .collect();
        chain.extend(
            self.outer
                .iter()
                .rev()
                .map(|entry| (entry.surface, entry.roles.as_slice(), entry.synthetic)),
        );
        let color = |value: &Option<Color>, reached: bool| {
            let val = match value {
                Some(c) => Val::Color(*c),
                None => Val::Absent,
            };
            (Answer::Val(val), reached)
        };
        let roles_place = |path: Vec<String>| {
            let place = Answer::Place {
                target: Target::Roles,
                path,
                bases_only: bases,
            };
            (place, false)
        };
        let split = |text: &str| text.split('.').map(str::to_owned).collect::<Vec<_>>();
        // Step 2: the nearest `Role` binding of exactly that role, else the
        // colour roles' value; for `active(…)` the value alone.
        let step2 = |held: &dyn Fn(&RoleRef) -> bool, path: Vec<String>| {
            if !bases {
                for (_, roles, reached) in &chain {
                    if let Some((_, value)) = roles.iter().rev().find(|(role, _)| held(role)) {
                        return color(value, *reached);
                    }
                }
            }
            roles_place(path)
        };
        let in_set = |set: SurfaceSet, x: &str| {
            let mut path = vec!["surfaces".to_owned(), set.name().to_owned()];
            path.extend(split(x));
            step2(
                &|role| matches!(role, RoleRef::Set(s, r) if *s == set && r.path() == x),
                path,
            )
        };
        match reference {
            Ref::Role { set, path } => in_set(*set, &joined(path)),
            Ref::GlobalRole(path) => {
                let x = joined(path);
                let mut place = vec!["global".to_owned()];
                place.extend(split(&x));
                step2(
                    &|role| matches!(role, RoleRef::Global(r) if r.path() == x),
                    place,
                )
            }
            Ref::KeyedRole(key) => step2(
                &|role| matches!(role, RoleRef::Keyed(k) if k == key),
                vec!["keyed".to_owned(), key.as_str().to_owned()],
            ),
            // Step 1: the nearest `Role { Surface(x) }` or `Surface(S)`
            // binding decides; `Surface(S)` makes it `role.S.x`, walked again;
            // with no binding, `role.window.x`.
            Ref::Surface(path) => {
                let x = joined(path);
                for (surface, roles, reached) in &chain {
                    if !bases
                        && let Some((_, value)) = roles
                            .iter()
                            .rev()
                            .find(|(role, _)| matches!(role, RoleRef::Surface(r) if r.path() == x))
                    {
                        return color(value, *reached);
                    }
                    if let Some(set) = surface {
                        let (answer, through) = in_set(*set, &x);
                        return (answer, through || *reached);
                    }
                }
                in_set(SurfaceSet::Window, &x)
            }
            Ref::Token(_)
            | Ref::Input(_)
            | Ref::Active(_)
            | Ref::Base
            | Ref::Scheme
            | Ref::FontSize
            | Ref::LineHeight
            | Ref::Element(_)
            | Ref::BodyFontHeight
            | Ref::Part(_)
            | Ref::Widget { .. }
            | Ref::Values(_)
            | Ref::TextScale => (Answer::Val(Val::Absent), false),
        }
    }

    /// The step that decides a place's value (S2.2): the last that states
    /// something there, but for a step after a reference above the place
    /// that patches it below the reference's level, which composes with the
    /// referenced value instead ([`Cx::through`]); an `Option` member a patch
    /// leaves unset states nothing.
    fn decide<'f, X: Any>(&mut self, items: &[Fold<'f>]) -> Option<(usize, Fold<'f>)> {
        let stated = items.iter().enumerate().map(|(index, fold)| {
            let states = match fold.found {
                Found::Missing => false,
                other => !other.get::<Option<Value<X>>>().is_some_and(Option::is_none),
            };
            (index, states)
        });
        let index = self.decider(stated, items)?;
        items.get(index).map(|fold| (index, *fold))
    }

    /// Of the steps `candidates` names (each item's index, and whether it
    /// sets the place rather than patching it), the index of the one that
    /// decides, by [`Cx::decide`]'s rule; positions in `candidates` are
    /// `items`' indices. The step that decides is recorded read (S17's
    /// synthetic route).
    fn decider(
        &mut self,
        candidates: impl Iterator<Item = (usize, bool)>,
        items: &[Fold<'_>],
    ) -> Option<usize> {
        let depth = self.path.len();
        let mut out: Option<usize> = None;
        for (index, sets) in candidates {
            if !sets {
                continue;
            }
            let Some(fold) = items.get(index) else {
                continue;
            };
            let after_reference = out
                .and_then(|held| items.get(held))
                .and_then(|held| match held.found {
                    Found::Through { rest, .. } => Some(depth.saturating_sub(rest)),
                    _ => None,
                });
            if let Some(level) = after_reference
                && !matches!(fold.found, Found::Through { .. } | Found::Gone)
                && self.whole_depth(*fold) > level
            {
                continue;
            }
            out = Some(index);
        }
        if let Some(fold) = out.and_then(|index| items.get(index)) {
            self.touch(fold.pos);
        }
        out
    }

    /// How deep the step of `fold` states the place whole: the depth of the
    /// first node on the way from its target's root to the place that is
    /// whole, the place's own depth where only patches lead there.
    fn whole_depth(&self, fold: Fold<'_>) -> usize {
        let depth = self.path.len();
        let Some(root) = self
            .roots(fold.target)
            .into_iter()
            .find(|root| root.pos == fold.pos)
        else {
            return depth;
        };
        let mut found = root.found;
        for (level, segment) in self.path.iter().enumerate() {
            let Found::Node(node) = found else {
                return depth;
            };
            if node.whole() {
                return level;
            }
            found = node.locate(&[segment.as_str()]);
        }
        depth
    }

    /// The text style of the element's part `part`.
    fn part_text(&mut self, part: TypedPart) -> Option<ResolvedTextStyle> {
        self.follow(
            Target::Part(part),
            vec!["text".to_owned()],
            false,
            |cx, folds| cx.block::<TextStyle>(folds),
        )
    }

    /// The `font_size` operand: the text style's own where the place lies in
    /// one, else the part's text's (S2.2).
    fn font_size_operand(&mut self) -> Option<Px> {
        match self.text.last() {
            Some(frame) => {
                let (size, reached) = (frame.size, frame.font_reached);
                self.reach(reached);
                size
            }
            None => match self.target {
                Target::Part(part) => self.part_text(part)?.font.size,
                Target::Roles | Target::Tokens | Target::Values(_) => None,
            },
        }
    }

    /// The `line_height` operand: under `auto` the font's line spacing
    /// (S2.2).
    fn line_height_operand(&mut self) -> Option<Px> {
        let (line_height, font) = match self.text.last() {
            Some(frame) => {
                let reached = frame.line_height_reached || frame.font_reached;
                let held = (frame.line_height?, frame.font.clone());
                self.reach(reached);
                held
            }
            None => match self.target {
                Target::Part(part) => {
                    let style = self.part_text(part)?;
                    (style.line_height?, Some(style.font))
                }
                Target::Roles | Target::Tokens | Target::Values(_) => return None,
            },
        };
        match line_height {
            Length::Px(px) => Some(Px(px)),
            Length::Auto => Some(self.element?.metrics.line_spacing(&font?)),
        }
    }

    /// An `element.` operand (S8): the element's data, the root's sizes
    /// through the scope, and the part's own font's metrics.
    fn datum(&mut self, datum: ElementDatum, origin: Target) -> Option<Val> {
        let element = self.element?;
        let data = &element.data;
        let scope = self.scope;
        Some(match datum {
            ElementDatum::Width => Val::Length(data.width),
            ElementDatum::Height => Val::Length(data.height),
            ElementDatum::Progress => Val::Number(data.progress.0),
            ElementDatum::ScrollOffset => Val::Length(data.scroll_offset),
            ElementDatum::ScrollProgress => Val::Number(data.scroll_progress.0),
            ElementDatum::ItemIndex => Val::Number(data.item_index as f32),
            ElementDatum::ItemCount => Val::Number(data.item_count as f32),
            ElementDatum::Text => Val::Text(data.text.to_owned()),
            ElementDatum::Color => Val::Color(data.color),
            ElementDatum::ScreenWidth => Val::Length(scope.screen.width),
            ElementDatum::ScreenHeight => Val::Length(scope.screen.height),
            ElementDatum::WindowWidth => Val::Length(scope.window.width),
            ElementDatum::WindowHeight => Val::Length(scope.window.height),
            ElementDatum::MoveDistance => Val::Length(data.move_distance),
            ElementDatum::CaretHeight => Val::Length(data.caret_height),
            ElementDatum::CharWidth
            | ElementDatum::MaxCharWidth
            | ElementDatum::FontXHeight
            | ElementDatum::FontHeight
            | ElementDatum::FontUnderlineOffset
            | ElementDatum::FontUnderlineThickness
            | ElementDatum::FontStrikeoutOffset
            | ElementDatum::FontStrikeoutThickness => {
                let framed = self
                    .text
                    .last()
                    .and_then(|frame| Some((frame.font.clone()?, frame.font_reached)));
                let font = match framed {
                    Some((font, reached)) => {
                        self.reach(reached);
                        font
                    }
                    None => match origin {
                        Target::Part(part) => self.part_text(part)?.font,
                        Target::Roles | Target::Tokens | Target::Values(_) => return None,
                    },
                };
                return self.metric(datum, &font);
            }
        })
    }

    /// A font metric of `font`, by the element's measurement.
    fn metric(&self, datum: ElementDatum, font: &ResolvedFont) -> Option<Val> {
        let metrics = self.element?.metrics;
        let px = match datum {
            ElementDatum::CharWidth => metrics.char_width(font),
            ElementDatum::MaxCharWidth => metrics.max_char_width(font),
            ElementDatum::FontXHeight => metrics.x_height(font),
            ElementDatum::FontHeight => metrics.height(font),
            ElementDatum::FontUnderlineOffset => metrics.underline_offset(font),
            ElementDatum::FontUnderlineThickness => metrics.underline_thickness(font),
            ElementDatum::FontStrikeoutOffset => metrics.strikeout_offset(font),
            ElementDatum::FontStrikeoutThickness => metrics.strikeout_thickness(font),
            _ => return None,
        };
        Some(Val::Length(px))
    }

    /// The DPI points convert by (S9.4): the reader's, else the preset's
    /// `token.text.font_dpi`.
    fn dpi(&mut self) -> Option<f32> {
        let dpi = self.sizes().0;
        self.reach(self.sizes_reached.0);
        dpi
    }

    /// The DPI and the text scaling, read once, outside the chain of
    /// references being followed (S2.2 counts an expression's own
    /// references only); while they are being read, none.
    fn sizes(&mut self) -> (Option<f32>, Option<Scaling>) {
        if let Some(sizes) = self.sizes {
            return sizes;
        }
        self.sizes = Some((None, None));
        let saved = std::mem::take(&mut self.visiting);
        let saved_bases = std::mem::replace(&mut self.bases_only, false);
        let (dpi, dpi_reached) = match self.theme.font_dpi {
            Some(dpi) => (Some(dpi), false),
            None => {
                let path = vec!["text".to_owned(), "font_dpi".to_owned()];
                self.aside(|cx| {
                    cx.follow(Target::Tokens, path, false, |cx, folds| {
                        cx.value::<f32>(folds, false)
                    })
                })
            }
        };
        let path = vec!["text".to_owned(), "scaling".to_owned()];
        let (scaling, scaling_reached) = self.aside(|cx| {
            cx.follow(Target::Tokens, path, false, |cx, folds| {
                cx.plain::<TextScaling>(folds)
            })
        });
        self.sizes_reached = (dpi_reached, scaling_reached);
        let factor = self.theme.text_scaling_factor;
        let scaling = scaling.map(|scaling| match scaling {
            TextScaling::Linear => Scaling::Linear(factor),
            TextScaling::AndroidNonLinear => Scaling::AndroidNonLinear(factor),
            TextScaling::DynamicType => {
                Scaling::DynamicType(self.theme.content_size_category.unwrap_or(DEFAULT_CATEGORY))
            }
            TextScaling::WinUi => Scaling::WinUi(factor),
        });
        self.visiting = saved;
        self.bases_only = saved_bases;
        self.sizes = Some((dpi, scaling));
        (dpi, scaling)
    }

    /// The preset's text scaling (S9.4); `None` where nothing states it.
    fn scaling(&mut self) -> Option<Scaling> {
        let scaling = self.sizes().1;
        self.reach(self.sizes_reached.1);
        scaling
    }

    /// What the font size being resolved at the place lies in: a
    /// `Font.size` where `size` says so or the place is one, a line height
    /// at a `line_height`, else a spacing or a size read as a value of its
    /// own.
    fn scaled_kind(&self, size: bool) -> Scaled {
        let names: Vec<&str> = self.path.iter().map(String::as_str).collect();
        match names.as_slice() {
            _ if size => Scaled::Size,
            [.., "font", "size"] => Scaled::Size,
            [.., "line_height"] => Scaled::LineHeight,
            _ => Scaled::Spacing,
        }
    }

    /// `px`, converted, scaled by the text scaling as its kind is (S9.4):
    /// under `Linear` the product by the factor; under `AndroidNonLinear`
    /// Android's curve, a line height above Compose's threshold kept in
    /// proportion to its font's change
    /// (`androidx@e4d933c9 compose/ui/ui-text/src/androidMain/kotlin/androidx/compose/ui/text/platform/extensions/SpannableExtensions.android.kt:224-250`);
    /// under `DynamicType` nothing at the default category, else a size by
    /// its style's row's ratio of the category's size to the default's, a
    /// line height equal to the row's leading at the default category the
    /// row's leading at the category; under `WinUi` a font size through
    /// WinUI's curve, a line height and a spacing as they are (S9.4: a
    /// spacing on `font_size` follows the scaled size through its operand).
    /// `None` where the kind's rule gives
    /// none: a role without a row, and, until the value phase reads UIKit's
    /// rule from its fonts per category (D-5), a line height other than the
    /// row's and a spacing stated in points or pixels under `DynamicType`.
    fn scale(&mut self, px: Px, kind: Scaled) -> Option<Px> {
        let Some(scaling) = self.scaling() else {
            return Some(px);
        };
        match (scaling, kind) {
            (Scaling::Linear(factor), _) => Some(Px(px.0 * factor)),
            (Scaling::AndroidNonLinear(scale), Scaled::LineHeight)
                if f64::from(scale) > PROPORTIONAL_LINE_HEIGHT_ABOVE =>
            {
                let font = self.style_font_size()?;
                let font_sp = android_non_linear_inverse(font.0, scale);
                Some(Px(px.0 / font_sp * font.0))
            }
            (Scaling::AndroidNonLinear(scale), _) => Some(Px(android_non_linear(px.0, scale))),
            (Scaling::DynamicType(category), _) if category == DEFAULT_CATEGORY => Some(px),
            (Scaling::DynamicType(category), Scaled::Size) => {
                let (at, default) = self.dynamic_type_rows(category)?;
                let size = f64::from(px.0) * f64::from(at.font_size) / f64::from(default.font_size);
                Some(Px(size as f32))
            }
            (Scaling::DynamicType(category), Scaled::LineHeight) => {
                let (at, default) = self.dynamic_type_rows(category)?;
                (px.0 == default.line_height).then_some(Px(at.line_height))
            }
            (Scaling::DynamicType(_), Scaled::Spacing) => None,
            (Scaling::WinUi(factor), Scaled::Size) => Some(Px(win_ui(px.0, factor))),
            (Scaling::WinUi(_), Scaled::LineHeight | Scaled::Spacing) => Some(px),
        }
    }

    /// The table's rows (S9.4's `DynamicType`) of the `scaling_role` of the
    /// text style the place lies in, `body`'s where it states none, at
    /// `category` and at the default category, in logical px.
    fn dynamic_type_rows(
        &mut self,
        category: ContentSizeCategory,
    ) -> Option<(DynamicTypeSize, DynamicTypeSize)> {
        let role = self.style_role().unwrap_or(TextRole::Body);
        let dpi = self.dpi()?;
        Some((
            dynamic_type(role, category, dpi)?,
            dynamic_type(role, DEFAULT_CATEGORY, dpi)?,
        ))
    }

    /// The text style's own path where the place lies in one of its font
    /// sizes, its `Font.size`, line height or spacing.
    fn style_path(&self) -> Option<Vec<String>> {
        let names: Vec<&str> = self.path.iter().map(String::as_str).collect();
        let own = style_font_size(&names)?;
        self.path
            .get(..self.path.len().checked_sub(own)?)
            .map(<[String]>::to_vec)
    }

    /// The `scaling_role` of the text style the place lies in: the style
    /// being resolved, else the one whose font size the place is.
    fn style_role(&mut self) -> Option<TextRole> {
        if let Some(&(role, reached)) = self.roles.last() {
            self.reach(reached);
            return role;
        }
        let mut path = self.style_path()?;
        path.push("scaling_role".to_owned());
        self.follow(self.target, path, false, |cx, folds| {
            cx.value::<Option<TextRole>>(folds, false)
        })
        .flatten()
    }

    /// The scaled font size of the text style the place lies in: the style
    /// being resolved, else the one whose font size the place is.
    fn style_font_size(&mut self) -> Option<Px> {
        if let Some(frame) = self.text.last() {
            let (size, reached) = (frame.size, frame.font_reached);
            self.reach(reached);
            return size;
        }
        let mut path = self.style_path()?;
        path.extend(["font".to_owned(), "size".to_owned()]);
        self.follow(self.target, path, false, |cx, folds| cx.font(folds, true))
    }

    /// A literal's value.
    fn literal(&mut self, literal: &Literal) -> Val {
        match literal {
            Literal::Color(c) => Val::Color(*c),
            Literal::Number(x) => Val::Number(*x),
            Literal::Px(px) => Val::Length(*px),
            Literal::Ms(ms) => Val::Duration(*ms),
            Literal::Deg(deg) => Val::Angle(*deg),
            Literal::Pt(points) => {
                let dpi = self.dpi();
                match convert(FontSize::Pt(*points), dpi) {
                    Some(px) if self.unscaled => Val::FontSize(px),
                    Some(px) => {
                        let kind = self.scaled_kind(false);
                        self.scale(px, kind).map_or(Val::Absent, Val::FontSize)
                    }
                    None => Val::Absent,
                }
            }
            Literal::Name(name) => Val::Name(name.clone()),
            Literal::Bool(b) => Val::Bool(*b),
        }
    }

    /// The value of the operand whose steps are `folds`, of whichever type
    /// its place holds.
    fn operand<'f>(&mut self, folds: &[Fold<'f>]) -> Option<Val>
    where
        't: 'f,
    {
        let (at, fold) = self.decide::<()>(folds)?;
        if let Found::Through { expr, rest } = fold.found {
            return self.through(expr, rest, folds, at, fold.target, |cx, composed| {
                cx.operand(composed)
            });
        }
        if let Some(radius) = fold.found.get::<Radius>() {
            return self.corner(radius, fold).map(Val::Length);
        }
        macro_rules! typed {
            ($($ty:ty),*) => {
                $(
                    if value_of::<$ty>(fold).is_some() {
                        return self.value::<$ty>(folds, true).map(|x| x.to_val());
                    }
                )*
            };
        }
        typed!(
            Color, Px, Ms, Deg, f32, Fraction, u16, u32, bool, String, Key, Scalar
        );
        if value_of::<FontSize>(fold).is_some() {
            let size = self.scaled_kind(false) == Scaled::Size;
            return self.font(folds, size).map(Val::FontSize);
        }
        if value_of::<Paint>(fold).is_some() || fold.found.get::<PaintPatch>().is_some() {
            return self.paint(folds).map(Val::Paint);
        }
        // A plain enum a platform function reads itself (`token.kde`'s
        // effects, S2.2): its variant's name.
        macro_rules! named {
            ($($ty:ty),*) => {
                $(
                    if fold.found.get::<$ty>().is_some() {
                        return self
                            .plain::<$ty>(folds)
                            .and_then(|variant| Key::from_static(variant.name()))
                            .map(|name| Val::Name(Name::Variant(name)));
                    }
                )*
            };
        }
        named!(KdeIntensityEffect, KdeColorEffect, KdeContrastEffect);
        None
    }

    /// A call's value, typed against `field` with the types of the place
    /// `origin`, its operands read first, `base` among them (S2.2).
    fn call(
        &mut self,
        expr: &Expr,
        field: &FieldType,
        origin: Target,
        base: impl FnOnce(&mut Self) -> Option<Val>,
    ) -> Option<Val> {
        if depth(expr) > MAX_DEPTH {
            self.refusals.push(Refusal::TooDeep {
                target: origin,
                path: self.path.join("."),
            });
            return None;
        }
        let theme = self.theme;
        let typed = expr
            .check(field, &type_env(&theme.declarations, origin.into()))
            .ok()?;
        let mut refs = Vec::new();
        references(expr, &mut refs);
        let mut values = Vec::new();
        let mut base = Some(base);
        for reference in refs {
            let value = match &reference {
                Ref::Base => base.take().and_then(|base| base(self)),
                other => self.operand_of(other, origin),
            };
            values.push((reference, value.unwrap_or(Val::Absent)));
        }
        let (dpi, scaling) = self.sizes();
        let literals = match scaling {
            _ if self.unscaled => Literals::Unscaled,
            None => Literals::Unscaled,
            Some(Scaling::Linear(factor)) => Literals::Factor(factor),
            Some(Scaling::AndroidNonLinear(scale)) => Literals::AndroidNonLinear(scale),
            Some(Scaling::DynamicType(category)) if category == DEFAULT_CATEGORY => {
                Literals::Unscaled
            }
            Some(Scaling::DynamicType(_)) => Literals::NoRule,
            Some(Scaling::WinUi(factor)) => match self.scaled_kind(false) {
                Scaled::Size => Literals::WinUi(factor),
                Scaled::LineHeight | Scaled::Spacing => Literals::Unscaled,
            },
        };
        let operands = Operands {
            values,
            kde_lists: &theme.kde_lists,
            dpi,
            literals,
            converted: Cell::new(false),
            no_rule: Cell::new(false),
        };
        let val = typed.eval(&operands);
        if operands.converted.get() {
            let (dpi_reached, scaling_reached) = self.sizes_reached;
            self.reach(dpi_reached || (scaling_reached && !self.unscaled));
        }
        if operands.no_rule.get() {
            return None;
        }
        Some(val)
    }

    /// The value `reference` gives as an operand.
    fn operand_of(&mut self, reference: &Ref, origin: Target) -> Option<Val> {
        match self.reference(reference, origin)? {
            Answer::Val(val) => Some(val),
            Answer::Place {
                target,
                path,
                bases_only,
            } => self.place_operand(target, path, bases_only),
        }
    }

    /// The value the place `path` of `target` gives as an operand: a length's
    /// plain reference to a text style's font size reads it so too,
    /// converted and scaled at its place (S2.2, S9.4).
    fn place_operand(
        &mut self,
        target: Target,
        path: Vec<String>,
        bases_only: bool,
    ) -> Option<Val> {
        self.follow(target, path, bases_only, |cx, folds| cx.operand(folds))
    }

    /// A `Value<X>` field's resolved value (S2.2): the last step's, its
    /// expression evaluated; `None`: absent, or nothing states it.
    pub(crate) fn value<'f, X: Leaf>(&mut self, items: &[Fold<'f>], signed: bool) -> Option<X>
    where
        't: 'f,
    {
        let (at, fold) = self.decide::<X>(items)?;
        match fold.found {
            Found::Gone | Found::Missing => None,
            Found::Through { expr, rest } => {
                self.through(expr, rest, items, at, fold.target, |cx, c| {
                    cx.value::<X>(c, signed)
                })
            }
            Found::Node(_) | Found::Leaf(_) => match value_of::<X>(fold) {
                Some(Value::Stated(x)) => Some(x.clone()),
                Some(Value::Absent | Value::Auto) => None,
                Some(Value::Expr(expr)) => self.expr::<X>(expr, items, at, fold.target, signed),
                None => {
                    let radius = fold.found.get::<Radius>()?;
                    let px = self.corner(radius, fold)?;
                    X::from_val(&Val::Length(px))
                }
            },
        }
    }

    /// An expression stated on a `Value<X>` field at `items[at]`.
    fn expr<'f, X: Leaf>(
        &mut self,
        expr: &Expr,
        items: &[Fold<'f>],
        at: usize,
        origin: Target,
        signed: bool,
    ) -> Option<X>
    where
        't: 'f,
    {
        let below = items.get(..at).unwrap_or_default();
        match expr {
            Expr::Ref(Ref::Base) => self.value::<X>(below, signed),
            Expr::Ref(reference) => {
                let val = match self.reference(reference, origin)? {
                    // A value with no place (an element's datum, a metric)
                    // is limited as a call's result is (S2.2's *Totality*).
                    Answer::Val(val) => val,
                    Answer::Place {
                        target,
                        path,
                        bases_only,
                    } if is_font_size(&path) => self.place_operand(target, path, bases_only)?,
                    Answer::Place {
                        target,
                        path,
                        bases_only,
                    } => {
                        return self.follow(target, path, bases_only, |cx, folds| {
                            cx.value::<X>(folds, signed)
                        });
                    }
                };
                let val = match X::field(signed) {
                    Some(field) => bounded(val, field.bound),
                    None => val,
                };
                X::from_val(&val)
            }
            Expr::Lit(literal) => {
                let val = self.literal(literal);
                X::from_val(&val)
            }
            Expr::Call(..) => {
                let unit = self.scalar_unit::<X>(origin);
                let field = match unit {
                    Some(unit) => scalar_field(unit),
                    None => X::field(signed)?,
                };
                let val = self.call(expr, &field, origin, |cx| {
                    cx.value::<X>(below, signed).map(|x| x.to_val())
                })?;
                match unit {
                    Some(unit) => X::from_scalar(scalar_of(unit, &val)?),
                    None => X::from_val(&val),
                }
            }
        }
    }

    /// Where `X` is a keyed scalar, its declared unit, which types a call
    /// stated on it: a tokens `params` group's (S4.6), a material's or a
    /// keyed kind's parameter's (S2.7, S10), as validation types it.
    fn scalar_unit<X: Leaf>(&self, origin: Target) -> Option<ScalarUnit> {
        if !X::KEYED_SCALAR {
            return None;
        }
        if let Some((depth, units)) = self.params {
            let [.., key] = self.path.as_slice() else {
                return None;
            };
            return (self.path.len() == depth.saturating_add(1))
                .then(|| units.iter().find(|(held, _)| held.as_str() == key))
                .flatten()
                .map(|(_, unit)| *unit);
        }
        if origin != Target::Tokens {
            return None;
        }
        let [params, group, _] = self.path.as_slice() else {
            return None;
        };
        if params != "params" {
            return None;
        }
        self.theme
            .declarations
            .params
            .iter()
            .find(|(held, _)| held.as_str() == group)
            .map(|(_, group)| group.unit)
    }

    /// The colour `reference` names, read from the colour roles (S3.3);
    /// `None`: not drawn.
    pub(crate) fn color_of(&mut self, reference: &Ref) -> Option<Color> {
        match self.reference(reference, Target::Roles)? {
            Answer::Val(val) => Color::from_val(&val),
            Answer::Place {
                target,
                path,
                bases_only,
            } => self.follow(target, path, bases_only, |cx, folds| {
                cx.value::<Color>(folds, false)
            }),
        }
    }

    /// A corner operand's length (S8): a `Px` radius's own; `Full` half the
    /// smaller side of the element's box, at most its `maximum`, a focus
    /// corner's on that box grown by the indicator's `offset` on each side;
    /// none for the corners only the connector computes at layout, which
    /// validation refuses as operands (S12).
    fn corner<'f>(&mut self, radius: &'f Radius, fold: Fold<'f>) -> Option<Px>
    where
        't: 'f,
    {
        match radius {
            Radius::Px(value) => self.value::<Px>(
                &[Fold {
                    found: Found::Leaf(value),
                    ..fold
                }],
                false,
            ),
            Radius::Full { maximum } => {
                let element = self.element?;
                let (mut width, mut height) = (element.data.width.0, element.data.height.0);
                if self.path.first().is_some_and(|first| first == "focus") {
                    let path = vec!["focus".to_owned(), "offset".to_owned()];
                    let offset = self.follow(fold.target, path, false, |cx, folds| {
                        Some(cx.sides::<Px>(folds, true))
                    })?;
                    let side = |px: Option<Px>| px.map_or(0.0, |Px(px)| px);
                    width += side(offset.left) + side(offset.right);
                    height += side(offset.top) + side(offset.bottom);
                }
                let maximum = self.value::<Px>(
                    &[Fold {
                        found: Found::Leaf(maximum),
                        ..fold
                    }],
                    false,
                );
                Some(full(Px(width), Px(height), maximum))
            }
            Radius::Ellipse | Radius::Concentric { .. } | Radius::FollowPart => None,
        }
    }

    /// An `Option<Value<X>>` field's value: `None` where the step states the
    /// `Option`'s own `None` (S1), and where no step states the field, as on
    /// a part whose catalogue entry does not list it (`PartStyle.shown`,
    /// S2.7); elsewhere a field validation refuses unstated (S2.2).
    pub(crate) fn optional<'f, X: Leaf>(&mut self, items: &[Fold<'f>], signed: bool) -> Option<X>
    where
        't: 'f,
    {
        let (held, pos) = items.iter().rev().find_map(|fold| {
            fold.found
                .get::<Option<Value<X>>>()
                .map(|held| (held, fold.pos))
        })?;
        match held {
            None => {
                self.touch(pos);
                None
            }
            Some(_) => self.value::<X>(items, signed),
        }
    }

    /// A plain field's value: the last step's.
    pub(crate) fn plain<'f, T: Any + Clone>(&mut self, items: &[Fold<'f>]) -> Option<T> {
        let (value, pos) = items
            .iter()
            .rev()
            .find_map(|fold| fold.found.get::<T>().map(|value| (value, fold.pos)))?;
        self.touch(pos);
        Some(value.clone())
    }

    /// A `Value<Px>` field marked `auto` (S6.1): `Length::Auto` where the last
    /// step, or a plain reference, brings `Auto`.
    pub(crate) fn auto<'f>(&mut self, items: &[Fold<'f>], signed: bool) -> Option<Length>
    where
        't: 'f,
    {
        let (at, fold) = self.decide::<Px>(items)?;
        if let Found::Through { expr, rest } = fold.found {
            return self.through(expr, rest, items, at, fold.target, |cx, c| {
                cx.auto(c, signed)
            });
        }
        match value_of::<Px>(fold)? {
            Value::Auto => Some(Length::Auto),
            Value::Expr(Expr::Ref(Ref::Base)) => {
                self.auto(items.get(..at).unwrap_or_default(), signed)
            }
            Value::Expr(Expr::Ref(reference)) => {
                let val = match self.reference(reference, fold.target)? {
                    Answer::Val(val) => val,
                    Answer::Place {
                        target,
                        path,
                        bases_only,
                    } if is_font_size(&path) => self.place_operand(target, path, bases_only)?,
                    Answer::Place {
                        target,
                        path,
                        bases_only,
                    } => {
                        return self
                            .follow(target, path, bases_only, |cx, folds| cx.auto(folds, signed));
                    }
                };
                let bound = Px::field(signed).map_or(Bound::Unbounded, |field| field.bound);
                Px::from_val(&bounded(val, bound)).map(|px| Length::Px(px.0))
            }
            Value::Stated(_) | Value::Absent | Value::Expr(_) => {
                self.value::<Px>(items, signed).map(|px| Length::Px(px.0))
            }
        }
    }

    /// A `Value<FontSize>` field's value in logical pixels (S9.4): converted
    /// and scaled once; `size` where the field is a `Font.size`, whose
    /// expression reads font sizes converted and scales its result.
    pub(crate) fn font<'f>(&mut self, items: &[Fold<'f>], size: bool) -> Option<Px>
    where
        't: 'f,
    {
        let (px, scaled) = self.font_value(items, size)?;
        if scaled || self.unscaled {
            return Some(px);
        }
        let kind = self.scaled_kind(size);
        self.scale(px, kind)
    }

    /// A `Value<FontSize>` field marked `auto` (a line height).
    pub(crate) fn font_auto<'f>(&mut self, items: &[Fold<'f>], size: bool) -> Option<Length>
    where
        't: 'f,
    {
        let (_, fold) = self.decide::<FontSize>(items)?;
        if matches!(value_of::<FontSize>(fold), Some(Value::Auto)) {
            return Some(Length::Auto);
        }
        self.font(items, size).map(|px| Length::Px(px.0))
    }

    /// A font size converted, and whether it is scaled already.
    fn font_value<'f>(&mut self, items: &[Fold<'f>], size: bool) -> Option<(Px, bool)>
    where
        't: 'f,
    {
        let (at, fold) = self.decide::<FontSize>(items)?;
        let below = items.get(..at).unwrap_or_default();
        if let Found::Through { expr, rest } = fold.found {
            return self.through(expr, rest, items, at, fold.target, |cx, c| {
                cx.font_value(c, size)
            });
        }
        let Some(value) = value_of::<FontSize>(fold) else {
            // A length where a font size is expected reads as px, not
            // text-scaled (S2.2).
            return self.value::<Px>(items, true).map(|px| (px, true));
        };
        match value {
            Value::Stated(stated) => {
                let dpi = self.dpi();
                Some((convert(*stated, dpi)?, false))
            }
            Value::Absent | Value::Auto => None,
            Value::Expr(Expr::Ref(Ref::Base)) => self.font_value(below, size),
            Value::Expr(Expr::Ref(reference)) => match self.reference(reference, fold.target)? {
                Answer::Val(val) => {
                    Px::from_val(&bounded(val, Bound::Unbounded)).map(|px| (px, !self.unscaled))
                }
                Answer::Place {
                    target,
                    path,
                    bases_only,
                } => self.follow(target, path, bases_only, |cx, folds| {
                    cx.font_value(folds, size)
                }),
            },
            Value::Expr(Expr::Lit(Literal::Pt(points))) => {
                let dpi = self.dpi();
                Some((convert(FontSize::Pt(*points), dpi)?, false))
            }
            Value::Expr(Expr::Lit(Literal::Px(px))) => Some((*px, false)),
            Value::Expr(expr) => {
                let field = FieldType::new(Ty::FontSize, Bound::Unbounded);
                // Inside an expression on a `Font.size`, every font size reads
                // converted and not yet scaled, however deep (S9.4).
                let saved = self.unscaled;
                let unscaled = size || saved;
                self.unscaled = unscaled;
                let val = self.call(expr, &field, fold.target, |cx| {
                    cx.font_value(below, size).map(|(px, _)| Val::FontSize(px))
                });
                self.unscaled = saved;
                let px = Px::from_val(&val?)?;
                Some((px, !unscaled))
            }
        }
    }

    /// A `Value` struct block field's resolved value (S2.2): `None` where no
    /// step states it, as a `Value` nothing states; else [`Cx::block`]'s.
    pub(crate) fn value_block<'f, B>(&mut self, items: &[Fold<'f>]) -> Option<B::Resolved>
    where
        't: 'f,
        B: Resolve + FieldsBlock + Locate,
        B::Patch: Locate,
    {
        if !items.iter().any(|fold| block_step::<B>(*fold).is_some()) {
            return None;
        }
        self.block::<B>(items)
    }

    /// A struct block's resolved value (S2.2): the last step that sets it,
    /// the `Fields` patches after it applied over it, a reference composed
    /// lazily with them; `None`: absent, or a field that cannot be absent
    /// has no value.
    pub(crate) fn block<'f, B>(&mut self, items: &[Fold<'f>]) -> Option<B::Resolved>
    where
        't: 'f,
        B: Resolve + FieldsBlock + Locate,
        B::Patch: Locate,
    {
        if self.keyframe && TypeId::of::<B>() == TypeId::of::<Loop>() {
            return None;
        }
        let steps: Vec<(usize, Step<'f, B>)> = items
            .iter()
            .enumerate()
            .filter_map(|(index, fold)| block_step::<B>(*fold).map(|step| (index, step)))
            .collect();
        let fields_after = |from: usize| -> Vec<Fold<'f>> {
            steps
                .iter()
                .skip(from)
                .filter_map(|(index, step)| match step {
                    Step::Fields(patch) => items.get(*index).map(|fold| Fold {
                        found: Found::Node(*patch),
                        ..*fold
                    }),
                    Step::Set(_) | Step::Whole(_) | Step::Through(..) | Step::Gone => None,
                })
                .collect()
        };
        let Some(set) = self.decider(
            steps
                .iter()
                .map(|(index, step)| (*index, !matches!(step, Step::Fields(_)))),
            items,
        ) else {
            return B::resolve(&fields_after(0), self);
        };
        let (at, step) = steps.get(set)?;
        let at = *at;
        let fold = *items.get(at)?;
        let whole = |block: &'f B| -> Vec<Fold<'f>> {
            let mut folds = vec![Fold {
                found: Found::Node(block),
                ..fold
            }];
            folds.extend(fields_after(set.saturating_add(1)));
            folds
        };
        match step {
            Step::Whole(block) => B::resolve(&whole(block), self),
            Step::Set(Value::Stated(block)) => B::resolve(&whole(block), self),
            Step::Set(Value::Absent | Value::Auto) | Step::Gone => None,
            Step::Set(Value::Expr(Expr::Ref(Ref::Base))) => {
                self.block::<B>(items.get(..at).unwrap_or_default())
            }
            Step::Set(Value::Expr(Expr::Ref(reference))) => {
                match self.reference(reference, fold.target)? {
                    Answer::Place {
                        target,
                        path,
                        bases_only,
                    } => self.follow(target, path, bases_only, |cx, referenced| {
                        let composed = compose::composed(referenced, items, at);
                        cx.block::<B>(&composed)
                    }),
                    Answer::Val(_) => None,
                }
            }
            Step::Through(expr, rest) => {
                self.through(expr, *rest, items, at, fold.target, |cx, c| {
                    cx.block::<B>(c)
                })
            }
            Step::Set(Value::Expr(_)) | Step::Fields(_) => None,
        }
    }

    /// A block set whole (an enum block, S6.1): the last step's, a reference
    /// followed.
    pub(crate) fn set<'f, B>(&mut self, items: &[Fold<'f>]) -> Option<B::Resolved>
    where
        't: 'f,
        B: ResolveWhole + Locate,
    {
        let (at, fold) = self.decide::<()>(items)?;
        if let Found::Through { expr, rest } = fold.found {
            return self.through(expr, rest, items, at, fold.target, |cx, c| cx.set::<B>(c));
        }
        if let Some(block) = fold.found.get::<B>() {
            return block.resolve_whole(fold.pos, self);
        }
        match value_of::<B>(fold)? {
            Value::Stated(block) => block.resolve_whole(fold.pos, self),
            Value::Absent | Value::Auto => None,
            Value::Expr(Expr::Ref(Ref::Base)) => self.set::<B>(items.get(..at).unwrap_or_default()),
            Value::Expr(Expr::Ref(reference)) => match self.reference(reference, fold.target)? {
                Answer::Place {
                    target,
                    path,
                    bases_only,
                } => self.follow(target, path, bases_only, |cx, folds| cx.set::<B>(folds)),
                Answer::Val(_) => None,
            },
            Value::Expr(_) => None,
        }
    }

    /// An append block's resolved value (S6.1): a paint's.
    pub(crate) fn append<'f, B: Appended>(&mut self, items: &[Fold<'f>]) -> Option<B::Resolved>
    where
        't: 'f,
    {
        B::append(items, self)
    }

    /// A paint (S2.3, S6.1): the last step that sets it, the layers each
    /// `Append` after it adds over it (over `Absent`, or nothing, none: the
    /// paint stays not drawn), a reference composed lazily with them, a
    /// colour expression one `Color` layer; `None`: not drawn.
    fn paint<'f>(&mut self, items: &[Fold<'f>]) -> Option<ResolvedPaint>
    where
        't: 'f,
    {
        let steps: Vec<(usize, PaintStep<'f>)> = items
            .iter()
            .enumerate()
            .filter_map(|(index, fold)| paint_step(*fold).map(|step| (index, step)))
            .collect();
        let set = self.decider(
            steps
                .iter()
                .map(|(index, step)| (*index, !matches!(step, PaintStep::Appended(_)))),
            items,
        );
        let mut layers: Vec<ResolvedPaintLayer> = Vec::new();
        let mut drawn = false;
        let from = match set {
            Some(set) => {
                let (at, step) = steps.get(set)?;
                let at = *at;
                let fold = *items.get(at)?;
                match step {
                    PaintStep::Set(Value::Stated(paint)) => {
                        drawn = true;
                        for layer in &paint.layers {
                            layers.push(layer.resolve_whole(fold.pos, self)?);
                        }
                    }
                    PaintStep::Set(Value::Absent | Value::Auto) | PaintStep::Gone => {}
                    PaintStep::Set(Value::Expr(Expr::Ref(Ref::Base))) => {
                        if let Some(below) = self.paint(items.get(..at).unwrap_or_default()) {
                            drawn = true;
                            layers = below.layers;
                        }
                    }
                    PaintStep::Set(Value::Expr(Expr::Ref(reference))) => {
                        // A paint's place composes with the appends kept on
                        // the reference; a colour's is one `Color` layer
                        // (S2.2), the appends after it over it.
                        let color = match self.reference(reference, fold.target)? {
                            Answer::Place {
                                target,
                                path,
                                bases_only,
                            } => {
                                let colour = self.locate(target, &path, bases_only);
                                if last(&colour)
                                    .is_some_and(|(_, f)| value_of::<Color>(f).is_some())
                                {
                                    self.follow(target, path, bases_only, |cx, folds| {
                                        cx.value::<Color>(folds, false)
                                    })
                                } else {
                                    return self.follow(
                                        target,
                                        path,
                                        bases_only,
                                        |cx, referenced| {
                                            let composed = compose::composed(referenced, items, at);
                                            cx.paint(&composed)
                                        },
                                    );
                                }
                            }
                            Answer::Val(val) => Color::from_val(&bounded(val, Bound::Unbounded)),
                        };
                        if let Some(color) = color {
                            drawn = true;
                            layers = vec![color_layer(color)];
                        }
                    }
                    PaintStep::Through(expr, rest) => {
                        return self
                            .through(expr, *rest, items, at, fold.target, |cx, c| cx.paint(c));
                    }
                    PaintStep::Set(Value::Expr(expr)) => {
                        let field = FieldType::new(Ty::Paint, Bound::Unbounded);
                        if let Some(Val::Paint(paint)) =
                            self.call(expr, &field, fold.target, |_| None)
                        {
                            drawn = true;
                            layers = paint.layers;
                        }
                    }
                    PaintStep::Appended(_) => {}
                }
                set.saturating_add(1)
            }
            None => 0,
        };
        if !drawn {
            return None;
        }
        for (at, step) in steps.iter().skip(from) {
            if let PaintStep::Appended(appended) = step {
                let pos = items.get(*at)?.pos;
                self.touch(pos);
                for layer in *appended {
                    layers.push(layer.resolve_whole(pos, self)?);
                }
            }
        }
        Some(ResolvedPaint { layers })
    }

    /// A list replaced whole: the last step's, each item resolved; empty
    /// where no step states it (a part's field its catalogue entry does not
    /// list, S5).
    pub(crate) fn list<'f, B: ResolveWhole + Any>(
        &mut self,
        items: &[Fold<'f>],
    ) -> Option<Vec<B::Resolved>>
    where
        't: 'f,
    {
        let Some(fold) = items
            .iter()
            .rev()
            .find(|fold| fold.found.get::<Vec<B>>().is_some())
        else {
            return Some(Vec::new());
        };
        let list = fold.found.get::<Vec<B>>()?;
        self.touch(fold.pos);
        list.iter()
            .map(|item| item.resolve_whole(fold.pos, self))
            .collect()
    }

    /// A list merged by its items' keys (S6.1), each item resolved; empty
    /// where no step states it, as [`Cx::list`].
    pub(crate) fn keyed<'f, B>(&mut self, items: &[Fold<'f>]) -> Option<Vec<B::Resolved>>
    where
        't: 'f,
        B: ResolveWhole + KeyedItem + Clone + Any,
    {
        let mark = self.reached.len();
        let mut merged: Vec<B> = Vec::new();
        let mut pos = None;
        let mut others = false;
        for fold in items {
            if let Some(list) = fold.found.get::<Vec<B>>() {
                B::merge_keyed(&mut merged, list.clone());
                pos = Some(fold.pos);
                others |= !self.theme.synthetic.contains(&fold.pos.layer);
                self.touch(fold.pos);
            }
        }
        let Some(pos) = pos else {
            return Some(Vec::new());
        };
        let out = merged
            .iter()
            .map(|item| item.resolve_whole(pos, self))
            .collect();
        // A synthetic layer's list with another layer's merged into it is
        // that layer's too: decided by a synthetic layer only where every
        // step merged into it is one (S6.1, S17 *Where a listed path is
        // found*, ADD-A144).
        if others {
            self.merged_from(mark);
        }
        out
    }

    /// An `Option` of a block, set whole.
    pub(crate) fn option<'f, B: ResolveWhole + Any>(
        &mut self,
        items: &[Fold<'f>],
    ) -> Option<Option<B::Resolved>>
    where
        't: 'f,
    {
        let fold = items
            .iter()
            .rev()
            .find(|fold| fold.found.get::<Option<B>>().is_some())?;
        self.touch(fold.pos);
        match fold.found.get::<Option<B>>()? {
            None => Some(None),
            Some(block) => block.resolve_whole(fold.pos, self).map(Some),
        }
    }

    /// A map merged by key (S6.1): its keys in the order first stated, each
    /// entry resolved by `entry` from the steps that state it; a whole map's
    /// entries located by `whole`, a patch's by `patch`. Every step holding
    /// the map is read, an empty one too, at the map's place alone
    /// ([`Cx::held_map`]).
    pub(crate) fn map<'f, K, V, P, Out>(
        &mut self,
        items: &[Fold<'f>],
        whole: impl Fn(&'f V, &[&str]) -> Found<'f>,
        patch: impl Fn(&'f P, &[&str]) -> Found<'f>,
        entry: impl Fn(&mut Self, &[Fold<'f>]) -> Option<Out>,
    ) -> Option<Vec<(K, Out)>>
    where
        't: 'f,
        K: MapKey + Clone + 'static,
        V: 'static,
        P: 'static,
    {
        // A fact the gate evaluates is the map's value, not a patch (S17):
        // the map it states replaces what lies below.
        let fact = items.iter().position(|fold| {
            self.is_fact(fold)
                && (fold.found.get::<Vec<(K, V)>>().is_some()
                    || fold.found.get::<Vec<(K, P)>>().is_some())
        });
        let items = match fact {
            Some(at) => items.get(at..).unwrap_or_default(),
            None => items,
        };
        let mut keys: Vec<(K, String)> = Vec::new();
        let mut add = |key: &K| {
            let name = key.key_name();
            if !keys.iter().any(|(_, held)| *held == name) {
                keys.push((key.clone(), name));
            }
        };
        let mut held = Vec::new();
        for fold in items {
            if let Some(map) = fold.found.get::<Vec<(K, V)>>() {
                map.iter().for_each(|(key, _)| add(key));
            } else if let Some(map) = fold.found.get::<Vec<(K, P)>>() {
                map.iter().for_each(|(key, _)| add(key));
            } else {
                continue;
            }
            held.push(fold.pos);
        }
        let mark = self.reached.len();
        let others = self.held_map(&held);
        let mut out = Vec::new();
        for (key, name) in keys {
            let named = |held: &K| held.key_name() == name;
            let folds: Vec<Fold<'f>> = items
                .iter()
                .filter_map(|fold| {
                    let found = match fold.found.get::<Vec<(K, V)>>() {
                        Some(map) => map
                            .iter()
                            .rev()
                            .find(|(held, _)| named(held))
                            .map(|(_, value)| whole(value, &[]))?,
                        None => fold
                            .found
                            .get::<Vec<(K, P)>>()?
                            .iter()
                            .rev()
                            .find(|(held, _)| named(held))
                            .map(|(_, value)| patch(value, &[]))?,
                    };
                    Some(Fold { found, ..*fold })
                })
                .collect();
            let value = self.at(&name, |cx| entry(cx, &folds));
            out.push(value.map(|value| (key, value)));
        }
        if others {
            self.entries_from(mark);
        }
        out.into_iter().collect()
    }

    /// A `Value` of a map's resolved value (`Font.axes`, S6.1): the last step
    /// that sets it, each stated map a patch states after it merged in by
    /// key, a reference composed lazily with them (S2.2); `None`: absent, or
    /// nothing states it.
    pub(crate) fn value_map<'f, K, V>(&mut self, items: &[Fold<'f>]) -> Option<Vec<(K, V)>>
    where
        't: 'f,
        K: PartialEq + Clone + 'static,
        V: Clone + 'static,
    {
        // A fact the gate evaluates is the map's value, not a patch (S17):
        // its stated map replaces what lies below.
        let steps: Vec<(usize, MapStep<'f, K, V>)> = items
            .iter()
            .enumerate()
            .filter_map(|(index, fold)| {
                map_step(*fold).map(|step| match step {
                    MapStep::Patch(value) if self.is_fact(fold) => (index, MapStep::Whole(value)),
                    step => (index, step),
                })
            })
            .collect();
        let mark = self.reached.len();
        let set = self.decider(
            steps.iter().map(|(index, step)| (*index, step.sets())),
            items,
        );
        let mut others = set
            .and_then(|at| items.get(at))
            .is_some_and(|fold| !self.theme.synthetic.contains(&fold.pos.layer));
        let (mut map, from) = match set {
            None => (None, 0),
            Some(at) => {
                let fold = *items.get(at)?;
                let (_, step) = steps.iter().find(|(index, _)| *index == at)?;
                let value = match step {
                    MapStep::Through(expr, rest) => {
                        return self.through(expr, *rest, items, at, fold.target, |cx, c| {
                            cx.value_map::<K, V>(c)
                        });
                    }
                    MapStep::Gone => return None,
                    MapStep::Whole(value) | MapStep::Patch(value) => value,
                };
                let map = match value {
                    Value::Stated(map) => map.clone(),
                    Value::Absent | Value::Auto => return None,
                    Value::Expr(Expr::Ref(Ref::Base)) => {
                        self.value_map::<K, V>(items.get(..at).unwrap_or_default())?
                    }
                    Value::Expr(Expr::Ref(reference)) => {
                        return match self.reference(reference, fold.target)? {
                            Answer::Place {
                                target,
                                path,
                                bases_only,
                            } => self.follow(target, path, bases_only, |cx, referenced| {
                                let composed = compose::composed(referenced, items, at);
                                cx.value_map::<K, V>(&composed)
                            }),
                            Answer::Val(_) => None,
                        };
                    }
                    Value::Expr(_) => return None,
                };
                (Some(map), at.saturating_add(1))
            }
        };
        for (at, step) in &steps {
            if *at < from {
                continue;
            }
            if let MapStep::Patch(Value::Stated(entries)) = step {
                let fold = items.get(*at)?;
                others |= !self.theme.synthetic.contains(&fold.pos.layer);
                self.touch(fold.pos);
                merge_by_key(map.get_or_insert_with(Vec::new), entries.clone());
            }
        }
        // A synthetic layer's map with another layer's merged into it is
        // that layer's too: the place is stated beside the synthetic value,
        // no listed path there (S6.1, S17, ADD-A128).
        if others {
            self.merged_from(mark);
        }
        map
    }

    /// A map of a struct block per surface set (S3), each set's entry
    /// resolved as a block. Every step holding the map is read, an empty
    /// one too, at the map's place alone ([`Cx::held_map`]).
    pub(crate) fn surfaces<'f, B>(&mut self, items: &[Fold<'f>]) -> Option<SurfaceMap<B::Resolved>>
    where
        't: 'f,
        B: Resolve + FieldsBlock + Locate,
        B::Patch: Locate,
    {
        let mut sets: Vec<SurfaceSet> = Vec::new();
        let mut held = Vec::new();
        for fold in items {
            if let Some(map) = fold.found.get::<SurfaceMap<B>>() {
                sets.extend(map.iter().map(|(set, _)| set));
            } else if let Some(map) = fold.found.get::<SurfaceMap<BlockPatch<B>>>() {
                sets.extend(map.iter().map(|(set, _)| set));
            } else {
                continue;
            }
            held.push(fold.pos);
        }
        let mark = self.reached.len();
        let others = self.held_map(&held);
        let mut out = SurfaceMap::default();
        for set in sets {
            if out.get(set).is_some() {
                continue;
            }
            let name = set.name();
            let folds = compose::children(items, name);
            if let Some(resolved) = self.at(name, |cx| cx.block::<B>(&folds)) {
                out.insert(set, resolved);
            }
        }
        if others {
            self.entries_from(mark);
        }
        Some(out)
    }

    /// A `Sides` of `Value`s, member by member.
    pub(crate) fn sides<'f, X: Leaf>(
        &mut self,
        items: &[Fold<'f>],
        signed: bool,
    ) -> Sides<Option<X>>
    where
        't: 'f,
    {
        let mut member = |name: &str| {
            let folds = compose::children(items, name);
            self.at(name, |cx| cx.value::<X>(&folds, signed))
        };
        Sides {
            top: member("top"),
            right: member("right"),
            bottom: member("bottom"),
            left: member("left"),
        }
    }

    /// A plain member's value: the last step's, a patch's unset member left
    /// out.
    fn member<T: Any + Clone>(&mut self, items: &[Fold<'_>], name: &str) -> Option<T> {
        let (value, pos) = compose::children(items, name)
            .iter()
            .rev()
            .find_map(|fold| {
                fold.found
                    .get::<T>()
                    .or_else(|| fold.found.get::<Option<T>>().and_then(Option::as_ref))
                    .map(|value| (value.clone(), fold.pos))
            })?;
        self.touch(pos);
        Some(value)
    }

    /// An `Axes` of plain values.
    pub(crate) fn axes_plain<T: Any + Clone>(&mut self, items: &[Fold<'_>]) -> Option<Axes<T>> {
        Some(Axes {
            horizontal: self.member(items, "horizontal")?,
            vertical: self.member(items, "vertical")?,
        })
    }

    /// A keyframe's patch (S2.7), `owner` its keyframe's steps: resolved
    /// over the style of the part its `part` names (`None`: the part the loop
    /// is on, the one whose place is evaluated), in that part's scope
    /// ([`ResolvePatch`]); every field `None` where that part is not drawn
    /// or is drawn as another (S6.3), or is no typed widget's part, its `at`
    /// and `timing` the keyframe's own.
    pub(crate) fn patch<'f, B>(
        &mut self,
        items: &[Fold<'f>],
        owner: &[Fold<'f>],
    ) -> Option<B::ResolvedPatch>
    where
        't: 'f,
        B: ResolvePatch,
        B::Patch: Any,
    {
        let (patch, fold) = items
            .iter()
            .rev()
            .find_map(|fold| fold.found.get::<B::Patch>().map(|patch| (patch, *fold)))?;
        self.touch(fold.pos);
        let named = self
            .plain::<Option<PartRef>>(&compose::children(owner, "part"))
            .flatten();
        let part = match named {
            Some(PartRef::Typed(part)) => Some(part),
            Some(PartRef::Component { .. }) => None,
            None => match self
                .origin
                .as_ref()
                .map_or(self.target, |(target, ..)| *target)
            {
                Target::Part(part) => Some(part),
                Target::Roles | Target::Tokens | Target::Values(_) => None,
            },
        };
        let Some(part) = part.filter(|part| self.drawn(*part)) else {
            return Some(B::ResolvedPatch::default());
        };
        let fold = Fold {
            pos: KEYFRAME,
            ..fold
        };
        let keyframe = std::mem::replace(&mut self.keyframe, true);
        let resolved = self.follow(Target::Part(part), Vec::new(), false, |cx, beneath| {
            Some(B::resolve_patch(patch, fold, beneath, cx))
        });
        self.keyframe = keyframe;
        Some(resolved.unwrap_or_default())
    }

    /// Whether `part` is drawn as itself: no kept step's presence, or the
    /// last one's `Present` (S6.3).
    fn drawn(&self, part: TypedPart) -> bool {
        self.theme
            .part_steps(part)
            .iter()
            .rev()
            .filter(|step| self.kept(step.selector.as_ref()))
            .find_map(|step| step.patch.presence.as_ref())
            .is_none_or(|presence| *presence == PartPresence::Present)
    }

    /// The element's part's style (S8 step 3): `None` where its last patch
    /// that states a presence makes it `Absent` or drawn as another widget
    /// (S6.3); its own bindings evaluated first, in the scope outside it.
    pub(crate) fn part(&mut self, part: TypedPart) -> Option<<PartStyle as Block>::Resolved> {
        if !self.drawn(part) {
            return None;
        }
        let folds = self.roots(Target::Part(part));
        self.target = Target::Part(part);
        self.chain_part = Some(part);
        let (bindings, reached) = self.noting(|cx| {
            cx.at("bindings", |cx| {
                cx.keyed::<Binding>(&compose::children(&folds, "bindings"))
            })
        });
        self.chain
            .push(Bindings::of(part, bindings.unwrap_or_default(), reached));
        PartStyle::resolve(&folds, self)
    }

    /// The element's part's own bindings, as [`Cx::part`] evaluates them
    /// first, in the scope outside it; `None` where it is not drawn as
    /// itself.
    fn part_bindings(&mut self, part: TypedPart) -> Option<Vec<ResolvedBinding>> {
        if !self.drawn(part) {
            return None;
        }
        let folds = self.roots(Target::Part(part));
        self.target = Target::Part(part);
        self.chain_part = Some(part);
        Some(
            self.at("bindings", |cx| {
                cx.keyed::<Binding>(&compose::children(&folds, "bindings"))
            })
            .unwrap_or_default(),
        )
    }
}

/// One `Color` layer drawn normally: a colour stated on a paint (S2.2).
fn color_layer(color: Color) -> ResolvedPaintLayer {
    ResolvedPaintLayer {
        kind: ResolvedPaintKind::Color(Some(color)),
        blend: BlendMode::Normal,
    }
}

/// An append block (S6.1): a paint.
pub(crate) trait Appended: Block {
    /// The block from the steps of its place.
    fn append<'t: 'f, 'f>(items: &[Fold<'f>], cx: &mut Cx<'t>) -> Option<Self::Resolved>;
}

impl Appended for Paint {
    fn append<'t: 'f, 'f>(items: &[Fold<'f>], cx: &mut Cx<'t>) -> Option<ResolvedPaint> {
        cx.paint(items)
    }
}

/// A shadow resolves an elevation to that entry of the ramp's layers, and
/// an empty one to `None`, as `Absent` does (S2.5).
impl ResolveWhole for Shadow {
    fn resolve_whole<'t: 'f, 'f>(&'f self, pos: Pos, cx: &mut Cx<'t>) -> Option<ResolvedShadow> {
        let layers = match self {
            Shadow::Layers(layers) => layers
                .iter()
                .map(|layer| layer.resolve_whole(pos, cx))
                .collect::<Option<Vec<_>>>()?,
            Shadow::Elevation(key) => {
                let path = vec!["elevation".to_owned(), key.as_str().to_owned()];
                cx.follow(Target::Tokens, path, false, |cx, folds| {
                    cx.list::<ShadowLayer>(folds)
                })?
            }
        };
        (!layers.is_empty()).then_some(layers)
    }
}

/// A font metric `element.<name>` names, also read as
/// `widget.<part>.<name>`.
fn font_metric(name: &str) -> Option<ElementDatum> {
    ElementDatum::from_name(name).filter(|datum| {
        matches!(
            datum,
            ElementDatum::CharWidth
                | ElementDatum::MaxCharWidth
                | ElementDatum::FontXHeight
                | ElementDatum::FontHeight
                | ElementDatum::FontUnderlineOffset
                | ElementDatum::FontUnderlineThickness
                | ElementDatum::FontStrikeoutOffset
                | ElementDatum::FontStrikeoutThickness
        )
    })
}

impl ResolvedTheme {
    /// The style of `element`'s part `part` (S8), the part's kept rules
    /// those `applies` accepts; `None` where the part is not drawn as itself.
    pub(crate) fn evaluate_part<'t>(
        &'t self,
        part: TypedPart,
        element: &'t Element<'t>,
        applies: &'t dyn Fn(&Selector) -> bool,
    ) -> Evaluated<Option<<PartStyle as Block>::Resolved>> {
        let mut cx = Cx::new(
            self,
            element.scope,
            Some(element),
            applies,
            Target::Part(part),
        );
        let value = cx.part(part);
        Evaluated {
            value,
            refusals: cx.refusals,
            reached: cx.reached,
        }
    }

    /// The own bindings of `element`'s part `part` (S3.3), its kept rules
    /// those `applies` accepts, as [`ResolvedTheme::evaluate_part`]
    /// evaluates them before its fields; `None` where the part is not
    /// drawn as itself.
    pub(crate) fn evaluate_bindings<'t>(
        &'t self,
        part: TypedPart,
        element: &'t Element<'t>,
        applies: &'t dyn Fn(&Selector) -> bool,
    ) -> Option<Vec<ResolvedBinding>> {
        let mut cx = Cx::new(
            self,
            element.scope,
            Some(element),
            applies,
            Target::Part(part),
        );
        cx.part_bindings(part)
    }

    /// The colour `reference` names in `scope` (S3.3); `None`: not drawn.
    pub(crate) fn evaluate_role<'t>(
        &'t self,
        scope: &'t Scope,
        reference: &Ref,
    ) -> Evaluated<Option<Color>> {
        let mut cx = Cx::new(self, scope, None, &|_| true, Target::Roles);
        let value = cx.color_of(reference);
        Evaluated {
            value,
            refusals: cx.refusals,
            reached: cx.reached,
        }
    }

    /// `target`'s root, a block of type `V` with no selector on its steps,
    /// evaluated in `scope` with no element (the tokens, S8).
    pub(crate) fn evaluate_target<'t, V: Resolve>(
        &'t self,
        scope: &'t Scope,
        target: Target,
    ) -> Evaluated<Option<V::Resolved>> {
        let mut cx = Cx::new(self, scope, None, &|_| true, target);
        let folds = cx.roots(target);
        let value = V::resolve(&folds, &mut cx);
        Evaluated {
            value,
            refusals: cx.refusals,
            reached: cx.reached,
        }
    }

    /// The bound a size class's threshold states on `axis` (`horizontal`
    /// or `vertical`), evaluated in `scope` (R16); `None` where it states
    /// none.
    pub(crate) fn evaluate_threshold<'t>(
        &'t self,
        scope: &'t Scope,
        class: &str,
        axis: &str,
    ) -> Option<Px> {
        let mut cx = Cx::new(self, scope, None, &|_| true, Target::Tokens);
        let path = vec!["size_classes".to_owned(), class.to_owned(), axis.to_owned()];
        cx.follow(Target::Tokens, path, false, |cx, folds| {
            cx.optional::<Px>(folds, false)
        })
    }

    /// `f` over an evaluation context in `scope` for `element`, its kept
    /// rules those `applies` accepts, `extra` evaluated beside the theme's
    /// own steps (S17's gate).
    pub(crate) fn evaluate_in<'t, R>(
        &'t self,
        scope: &'t Scope,
        element: Option<&'t Element<'t>>,
        applies: &'t dyn Fn(&Selector) -> bool,
        target: Target,
        extra: Option<Extra<'t>>,
        f: impl FnOnce(&mut Cx<'t>) -> R,
    ) -> R {
        let mut cx = Cx::new(self, scope, element, applies, target);
        cx.extra = extra;
        f(&mut cx)
    }

    /// The logical px a point resolves to in `scope` (S9.4): a point
    /// converted as a font size is, the DPI over 72, scaled where text
    /// scales linearly; `None` without a DPI.
    pub(crate) fn px_per_point(&self, scope: &Scope) -> Option<f32> {
        let mut cx = Cx::new(self, scope, None, &|_| true, Target::Tokens);
        let (dpi, scaling) = cx.sizes();
        let px = convert(FontSize::Pt(1.0), dpi)?;
        Some(match scaling {
            Some(Scaling::Linear(factor)) => px.0 * factor,
            Some(Scaling::AndroidNonLinear(_) | Scaling::DynamicType(_) | Scaling::WinUi(_))
            | None => px.0,
        })
    }

    /// The keyed scalar `token.params.<group>.<key>` in `scope`, typed by
    /// its group's declared unit (S4.6); `None` where it is not stated or
    /// is `Absent`.
    pub(crate) fn evaluate_param<'t>(
        &'t self,
        scope: &'t Scope,
        group: &str,
        key: &str,
    ) -> Option<Scalar> {
        let mut cx = Cx::new(self, scope, None, &|_| true, Target::Tokens);
        let path = vec!["params".to_owned(), group.to_owned(), key.to_owned()];
        cx.follow(Target::Tokens, path, false, |cx, folds| {
            cx.value::<Scalar>(folds, false)
        })
    }

    /// The values of `element`'s widget `widget` (S8), their kept rules
    /// those `applies` accepts.
    pub(crate) fn evaluate_values<'t, V: Resolve>(
        &'t self,
        widget: WidgetKind,
        element: &'t Element<'t>,
        applies: &'t dyn Fn(&Selector) -> bool,
    ) -> Evaluated<Option<V::Resolved>> {
        let mut cx = Cx::new(
            self,
            element.scope,
            Some(element),
            applies,
            Target::Values(widget),
        );
        let folds = cx.roots(Target::Values(widget));
        let value = V::resolve(&folds, &mut cx);
        Evaluated {
            value,
            refusals: cx.refusals,
            reached: cx.reached,
        }
    }
}

mod engine;
#[cfg(test)]
mod tests;
