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

use std::any::Any;

use super::compose::{self, Fold, Found, Locate, Target};
use super::{Pos, ResolvedTheme};
use crate::IconSet;
use crate::next::blocks::outline::{DashPattern, Radius, ShapeKind, full};
use crate::next::blocks::paint::{
    BlendMode, Paint, PaintLayer, PaintPatch, ResolvedPaint, ResolvedPaintKind, ResolvedPaintLayer,
};
use crate::next::blocks::part_style::{Binding, PartStyle, ResolvedBinding, RoleRef};
use crate::next::blocks::shadow::{ResolvedShadow, Shadow, ShadowLayer};
use crate::next::blocks::text::{Font, ResolvedFont, ResolvedTextStyle, TextStyle};
use crate::next::blocks::{Block, BlockPatch, FieldsBlock, KeyedItem};
use crate::next::color::Color;
use crate::next::declare::ScalarUnit;
use crate::next::expr::{
    Bound, ElementDatum, Env, Expr, FieldType, Literal, MAX_CHAIN, MAX_DEPTH, Name, Ref, Ty, Val,
    bounded,
};
use crate::next::key::Key;
use crate::next::query::{Element, Scope, ScopeEntry, own_entries};
use crate::next::roles::SurfaceSet;
use crate::next::selector::{PartRef, Selector};
use crate::next::tokens::TextScaling;
use crate::next::toml::read::{MapKey, Named};
use crate::next::units::{Axes, Deg, FontSize, Fraction, Length, Ms, Px, Scalar, Sides};
use crate::next::validate::{TypeTarget, type_env};
use crate::next::value::{SurfaceMap, Value};
use crate::next::widgets::{PartPatch, TypedPart, WidgetKind, WidgetValuesPatch};

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

/// A struct block's patch resolved alone (a keyframe's part style, S2.7):
/// each field it sets resolved, the others `None`.
pub(crate) trait ResolvePatch: FieldsBlock {
    /// The patch resolved.
    type ResolvedPatch;

    /// `patch`, stated by a step at `pos`, resolved.
    fn resolve_patch<'t: 'f, 'f>(
        patch: &'f Self::Patch,
        pos: Pos,
        cx: &mut Cx<'t>,
    ) -> Self::ResolvedPatch;
}

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
}

/// The text style a value lies in, for its `font_size` and `line_height`
/// operands (S2.2: the style's own).
#[derive(Clone, Debug)]
struct TextFrame {
    font: Option<ResolvedFont>,
    size: Option<Px>,
    line_height: Option<Length>,
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
    /// The places whose references are being followed.
    visiting: Vec<(Target, String)>,
    /// Inside an expression on a `Font.size`: font sizes read converted, not
    /// yet scaled (S9.4).
    unscaled: bool,
    /// Inside `active(…)`: every place read reads its bases alone, and no
    /// `Role` binding is followed (S2.2).
    bases_only: bool,
    /// The DPI and the scaling factor, read once outside every chain of
    /// references (S9.4); `None` until read.
    sizes: Option<(Option<f32>, Option<f32>)>,
    refusals: Vec<Refusal>,
}

/// One part's bindings in scope (S3.3): its surface set and its roles,
/// each role evaluated in the scope outside it.
#[derive(Clone, Debug)]
struct Bindings {
    part: TypedPart,
    surface: Option<SurfaceSet>,
    roles: Vec<(RoleRef, Option<Color>)>,
}

impl Bindings {
    /// `part`'s bindings, a later of one role or the surface replacing the
    /// earlier (S6.1).
    fn of(part: TypedPart, bindings: Vec<ResolvedBinding>) -> Bindings {
        let mut bound = Bindings {
            part,
            surface: None,
            roles: Vec::new(),
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

/// An evaluated expression's operands, which typed evaluation reads.
struct Operands {
    values: Vec<(Ref, Val)>,
    dpi: Option<f32>,
    factor: Option<f32>,
}

impl Env for Operands {
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
        match (convert(size, self.dpi), self.factor) {
            (Some(px), Some(factor)) => Px(px.0 * factor),
            (Some(px), None) => px,
            (None, _) => Px(f32::NAN),
        }
    }
}

/// A size in points or pixels in logical pixels (S9.4): `pt × dpi / 72`;
/// `None` for points without a DPI.
fn convert(size: FontSize, dpi: Option<f32>) -> Option<Px> {
    match size {
        FontSize::Pt(points) => dpi.map(|dpi| Px(points * dpi / 72.0)),
        FontSize::Px(px) => Some(Px(px)),
    }
}

/// The references an expression reads, `active(…)`'s whole.
pub(crate) fn references(expr: &Expr, out: &mut Vec<Ref>) {
    match expr {
        Expr::Lit(_) => {}
        Expr::Ref(reference) => {
            if !out.contains(reference) {
                out.push(reference.clone());
            }
        }
        Expr::Call(_, args) => args.iter().for_each(|arg| references(arg, out)),
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

/// One scope entry's bindings, S3.3's lookup walks: its surface set and its
/// roles.
type ScopeBindings<'s> = (Option<SurfaceSet>, &'s [(RoleRef, Option<Color>)]);

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
            visiting: Vec::new(),
            unscaled: false,
            bases_only: false,
            sizes: None,
            refusals: Vec::new(),
        }
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
        let font = self.at("font", |cx| {
            cx.block::<Font>(&compose::children(folds, "font"))
        });
        let size = font.as_ref().and_then(|font| font.size);
        self.text.push(TextFrame {
            font,
            size,
            line_height: None,
        });
        let line_height = self.at("line_height", |cx| {
            cx.font_auto(&compose::children(folds, "line_height"), false)
        });
        if let Some(frame) = self.text.last_mut() {
            frame.line_height = line_height;
        }
        let out = f(self);
        self.text.pop();
        out
    }

    /// Whether a part's step applies to the element: a base always, a rule
    /// where `applies` accepts its selector.
    fn kept(&self, selector: Option<&Selector>) -> bool {
        selector.is_none_or(|selector| (self.applies)(selector))
    }

    /// The steps of `target`'s root, in the stack's order: a part's those
    /// that apply to the element (S8 step 3), its style patches only.
    fn roots(&self, target: Target) -> Vec<Fold<'t>> {
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
                .map(|step| fold(step.pos, Found::Node(&step.patch)))
                .collect(),
            Target::Part(part) => theme
                .part_steps(part)
                .iter()
                .filter(|step| self.kept(step.selector.as_ref()))
                .filter_map(|step| match &step.patch {
                    PartPatch::Style(style) => Some(fold(step.pos, Found::Node(style))),
                    PartPatch::Absent | PartPatch::DrawnAs { .. } => None,
                })
                .collect(),
            Target::Values(widget) => theme
                .values
                .iter()
                .filter(|(held, _)| *held == widget)
                .flat_map(|(_, steps)| steps)
                .filter(|step| self.kept(step.selector.as_ref()))
                .map(|step| match &step.patch {
                    WidgetValuesPatch::Button(values) => fold(step.pos, Found::Node(values)),
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
        let key = (target, path.join("."));
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
        let bases_only = bases_only || self.bases_only;
        let folds = self.locate(target, &path, bases_only);
        self.visiting.push(key);
        let saved_bases = std::mem::replace(&mut self.bases_only, bases_only);
        let saved_target = std::mem::replace(&mut self.target, target);
        let saved_path = std::mem::replace(&mut self.path, path);
        let saved_text = std::mem::take(&mut self.text);
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
        self.text = saved_text;
        self.path = saved_path;
        self.target = saved_target;
        self.visiting.pop();
        out
    }

    /// The bindings in scope for `part` within the element (S2.2, S3.3):
    /// those of the parts enclosing it, from its element's part inward, and
    /// its own, each evaluated in the scope outside it.
    fn chain_of(&mut self, part: TypedPart) -> Vec<Bindings> {
        let head = self.element.and_then(|element| match &element.part {
            PartRef::Typed(own) => Some(own.element()),
            PartRef::Component { .. } => None,
        });
        let mut enclosing = part.enclosing();
        if let Some(at) = enclosing.iter().position(|held| Some(*held) == head) {
            enclosing.truncate(at.saturating_add(1));
        }
        let mut chain = Vec::new();
        for held in enclosing.into_iter().rev() {
            self.chain = chain.clone();
            self.chain_part = Some(held);
            self.outside = Some(held);
            let path = vec!["bindings".to_owned()];
            let bindings = self.follow(Target::Part(held), path, false, |cx, folds| {
                cx.keyed::<Binding>(folds)
            });
            chain.push(Bindings::of(held, bindings.unwrap_or_default()));
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
                Some(self.role(reference, false))
            }
            Ref::Active(inner) => Some(self.role(inner, true)),
            Ref::Token(path) => Some(place(Target::Tokens, segments(path))),
            Ref::Input(path) => {
                let theme = self.theme;
                let value = theme
                    .inputs
                    .iter()
                    .find(|(name, _)| name.segments() == path.as_slice())
                    .map(|(_, value)| value)?;
                let fold = Fold {
                    pos: Pos {
                        rule: false,
                        layer: 0,
                        index: 0,
                    },
                    target: Target::Roles,
                    found: Found::Leaf(value),
                };
                // An input reading itself, directly or through others, is a
                // cycle (S12), refused as any other.
                let key = (Target::Roles, format!("input.{}", segments(path).join(".")));
                if self.visiting.contains(&key) {
                    self.refusals.push(Refusal::Cycle {
                        target: key.0,
                        path: key.1,
                    });
                    return None;
                }
                self.visiting.push(key);
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
    /// bindings only, and reads the bases (S2.2).
    fn role(&self, reference: &Ref, bases: bool) -> Answer {
        let bases = bases || self.bases_only;
        let joined = |path: &[Key]| path.iter().map(Key::as_str).collect::<Vec<_>>().join(".");
        let mut chain: Vec<ScopeBindings<'_>> = self
            .chain
            .iter()
            .rev()
            .filter(|bound| Some(bound.part) != self.outside)
            .map(|bound| (bound.surface, bound.roles.as_slice()))
            .collect();
        chain.extend(
            self.outer
                .iter()
                .rev()
                .map(|entry| (entry.surface, entry.roles.as_slice())),
        );
        let color = |value: &Option<Color>| {
            Answer::Val(match value {
                Some(c) => Val::Color(*c),
                None => Val::Absent,
            })
        };
        let roles_place = |path: Vec<String>| Answer::Place {
            target: Target::Roles,
            path,
            bases_only: bases,
        };
        let split = |text: &str| text.split('.').map(str::to_owned).collect::<Vec<_>>();
        // Step 2: the nearest `Role` binding of exactly that role, else the
        // colour roles' value; for `active(…)` the value alone.
        let step2 = |held: &dyn Fn(&RoleRef) -> bool, path: Vec<String>| {
            if !bases {
                for (_, roles) in &chain {
                    if let Some((_, value)) = roles.iter().rev().find(|(role, _)| held(role)) {
                        return color(value);
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
                for (surface, roles) in &chain {
                    if !bases
                        && let Some((_, value)) = roles
                            .iter()
                            .rev()
                            .find(|(role, _)| matches!(role, RoleRef::Surface(r) if r.path() == x))
                    {
                        return color(value);
                    }
                    if let Some(set) = surface {
                        return in_set(*set, &x);
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
            | Ref::TextScale => Answer::Val(Val::Absent),
        }
    }

    /// The step that decides a place's value (S2.2): the last that states
    /// something there, but for a step after a reference above the place
    /// that patches it below the reference's level, which composes with the
    /// referenced value instead ([`Cx::through`]); an `Option` member a patch
    /// leaves unset states nothing.
    fn decide<'f, X: Any>(&self, items: &[Fold<'f>]) -> Option<(usize, Fold<'f>)> {
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
    /// `items`' indices.
    fn decider(
        &self,
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
            Some(frame) => frame.size,
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
            Some(frame) => (frame.line_height?, frame.font.clone()),
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
            ElementDatum::CharWidth
            | ElementDatum::MaxCharWidth
            | ElementDatum::FontXHeight
            | ElementDatum::FontHeight
            | ElementDatum::FontUnderlineOffset
            | ElementDatum::FontUnderlineThickness
            | ElementDatum::FontStrikeoutOffset
            | ElementDatum::FontStrikeoutThickness => {
                let font = match self.text.last().and_then(|frame| frame.font.clone()) {
                    Some(font) => font,
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
        self.sizes().0
    }

    /// The DPI and the scaling factor, read once, outside the chain of
    /// references being followed (S2.2 counts an expression's own
    /// references only); while they are being read, none.
    fn sizes(&mut self) -> (Option<f32>, Option<f32>) {
        if let Some(sizes) = self.sizes {
            return sizes;
        }
        self.sizes = Some((None, None));
        let saved = std::mem::take(&mut self.visiting);
        let saved_bases = std::mem::replace(&mut self.bases_only, false);
        let dpi = match self.theme.font_dpi {
            Some(dpi) => Some(dpi),
            None => {
                let path = vec!["text".to_owned(), "font_dpi".to_owned()];
                self.follow(Target::Tokens, path, false, |cx, folds| {
                    cx.value::<f32>(folds, false)
                })
            }
        };
        let path = vec!["text".to_owned(), "scaling".to_owned()];
        let scaling = self.follow(Target::Tokens, path, false, |cx, folds| {
            cx.plain::<TextScaling>(folds)
        });
        let factor = match scaling {
            Some(TextScaling::Linear) => Some(self.theme.text_scaling_factor),
            Some(TextScaling::AndroidNonLinear | TextScaling::DynamicType) | None => None,
        };
        self.visiting = saved;
        self.bases_only = saved_bases;
        self.sizes = Some((dpi, factor));
        (dpi, factor)
    }

    /// The factor font sizes scale by under the preset's
    /// `token.text.scaling` (S9.4): `Linear`'s product; `None` for the
    /// non-linear kinds, whose curves are not implemented yet.
    fn factor(&mut self) -> Option<f32> {
        self.sizes().1
    }

    /// `px` scaled by the text scaling (S9.4).
    fn scale(&mut self, px: Px) -> Px {
        match self.factor() {
            Some(factor) => Px(px.0 * factor),
            None => px,
        }
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
                    Some(px) => Val::FontSize(self.scale(px)),
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
            return self.font(folds, false).map(Val::FontSize);
        }
        if value_of::<Paint>(fold).is_some() || fold.found.get::<PaintPatch>().is_some() {
            return self.paint(folds).map(Val::Paint);
        }
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
        let dpi = self.dpi();
        let factor = if self.unscaled { None } else { self.factor() };
        Some(typed.eval(&Operands {
            values,
            dpi,
            factor,
        }))
    }

    /// The value `reference` gives as an operand.
    fn operand_of(&mut self, reference: &Ref, origin: Target) -> Option<Val> {
        match self.reference(reference, origin)? {
            Answer::Val(val) => Some(val),
            Answer::Place {
                target,
                path,
                bases_only,
            } => self.follow(target, path, bases_only, |cx, folds| cx.operand(folds)),
        }
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
            Expr::Ref(reference) => match self.reference(reference, origin)? {
                // A value with no place (an element's datum, a metric) is
                // limited as a call's result is (S2.2's *Totality*).
                Answer::Val(val) => {
                    let val = match X::field(signed) {
                        Some(field) => bounded(val, field.bound),
                        None => val,
                    };
                    X::from_val(&val)
                }
                Answer::Place {
                    target,
                    path,
                    bases_only,
                } => self.follow(target, path, bases_only, |cx, folds| {
                    cx.value::<X>(folds, signed)
                }),
            },
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

    /// Where `X` is a keyed scalar stated in the tokens' `params`, its
    /// group's declared unit (S4.6), which types a call stated on it.
    fn scalar_unit<X: Leaf>(&self, origin: Target) -> Option<ScalarUnit> {
        if !X::KEYED_SCALAR || origin != Target::Tokens {
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

    /// An `Option<Value<X>>` field's value: `Some(None)` where the step states
    /// the `Option`'s own `None` (S1).
    pub(crate) fn optional<'f, X: Leaf>(
        &mut self,
        items: &[Fold<'f>],
        signed: bool,
    ) -> Option<Option<X>>
    where
        't: 'f,
    {
        let held = items
            .iter()
            .rev()
            .find_map(|fold| fold.found.get::<Option<Value<X>>>())?;
        match held {
            None => Some(None),
            Some(_) => Some(self.value::<X>(items, signed)),
        }
    }

    /// A plain field's value: the last step's.
    pub(crate) fn plain<'f, T: Any + Clone>(&mut self, items: &[Fold<'f>]) -> Option<T> {
        items
            .iter()
            .rev()
            .find_map(|fold| fold.found.get::<T>())
            .cloned()
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
            Value::Expr(Expr::Ref(reference)) => match self.reference(reference, fold.target)? {
                Answer::Val(val) => {
                    let bound = Px::field(signed).map_or(Bound::Unbounded, |field| field.bound);
                    Px::from_val(&bounded(val, bound)).map(|px| Length::Px(px.0))
                }
                Answer::Place {
                    target,
                    path,
                    bases_only,
                } => self.follow(target, path, bases_only, |cx, folds| cx.auto(folds, signed)),
            },
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
        Some(if scaled || self.unscaled {
            px
        } else {
            self.scale(px)
        })
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
    /// `Append` after it adds over it (over `Absent`, the appended layers
    /// alone), a reference composed lazily with them, a colour expression
    /// one `Color` layer; `None`: not drawn.
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
        for (at, step) in steps.iter().skip(from) {
            if let PaintStep::Appended(appended) = step {
                let pos = items.get(*at)?.pos;
                for layer in *appended {
                    drawn = true;
                    layers.push(layer.resolve_whole(pos, self)?);
                }
            }
        }
        drawn.then_some(ResolvedPaint { layers })
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
        let mut merged: Vec<B> = Vec::new();
        let mut pos = None;
        for fold in items {
            if let Some(list) = fold.found.get::<Vec<B>>() {
                B::merge_keyed(&mut merged, list.clone());
                pos = Some(fold.pos);
            }
        }
        let Some(pos) = pos else {
            return Some(Vec::new());
        };
        merged
            .iter()
            .map(|item| item.resolve_whole(pos, self))
            .collect()
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
        match fold.found.get::<Option<B>>()? {
            None => Some(None),
            Some(block) => block.resolve_whole(fold.pos, self).map(Some),
        }
    }

    /// A map merged by key (S6.1): its keys in the order first stated, each
    /// entry resolved by `entry` from the steps that state it; a whole map's
    /// entries located by `whole`, a patch's by `patch`.
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
        let mut keys: Vec<(K, String)> = Vec::new();
        let mut add = |key: &K| {
            let name = key.key_name();
            if !keys.iter().any(|(_, held)| *held == name) {
                keys.push((key.clone(), name));
            }
        };
        for fold in items {
            if let Some(map) = fold.found.get::<Vec<(K, V)>>() {
                map.iter().for_each(|(key, _)| add(key));
            } else if let Some(map) = fold.found.get::<Vec<(K, P)>>() {
                map.iter().for_each(|(key, _)| add(key));
            }
        }
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
            let value = self.at(&name, |cx| entry(cx, &folds))?;
            out.push((key, value));
        }
        Some(out)
    }

    /// A map of a struct block per surface set (S3), each set's entry
    /// resolved as a block.
    pub(crate) fn surfaces<'f, B>(&mut self, items: &[Fold<'f>]) -> Option<SurfaceMap<B::Resolved>>
    where
        't: 'f,
        B: Resolve + FieldsBlock + Locate,
        B::Patch: Locate,
    {
        let mut sets: Vec<SurfaceSet> = Vec::new();
        for fold in items {
            if let Some(map) = fold.found.get::<SurfaceMap<B>>() {
                sets.extend(map.iter().map(|(set, _)| set));
            } else if let Some(map) = fold.found.get::<SurfaceMap<BlockPatch<B>>>() {
                sets.extend(map.iter().map(|(set, _)| set));
            }
        }
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
    fn member<T: Any + Clone>(items: &[Fold<'_>], name: &str) -> Option<T> {
        compose::children(items, name)
            .iter()
            .rev()
            .find_map(|fold| {
                fold.found
                    .get::<T>()
                    .or_else(|| fold.found.get::<Option<T>>().and_then(Option::as_ref))
                    .cloned()
            })
    }

    /// An `Axes` of plain values.
    pub(crate) fn axes_plain<T: Any + Clone>(&mut self, items: &[Fold<'_>]) -> Option<Axes<T>> {
        Some(Axes {
            horizontal: Self::member(items, "horizontal")?,
            vertical: Self::member(items, "vertical")?,
        })
    }

    /// A field holding a block's patch, resolved alone (a keyframe's).
    pub(crate) fn patch<'f, B>(&mut self, items: &[Fold<'f>]) -> Option<B::ResolvedPatch>
    where
        't: 'f,
        B: ResolvePatch,
        B::Patch: Any,
    {
        let (patch, pos) = items
            .iter()
            .rev()
            .find_map(|fold| fold.found.get::<B::Patch>().map(|patch| (patch, fold.pos)))?;
        Some(B::resolve_patch(patch, pos, self))
    }

    /// The element's part's style (S8 step 3): `None` where its last patch
    /// that says what it is makes it `Absent` or drawn as another widget
    /// (S6.3); its own bindings evaluated first, in the scope outside it.
    fn part(&mut self, part: TypedPart) -> Option<<PartStyle as Block>::Resolved> {
        let shown = self
            .theme
            .part_steps(part)
            .iter()
            .rev()
            .find(|step| self.kept(step.selector.as_ref()))
            .map(|step| matches!(step.patch, PartPatch::Style(_)));
        if shown == Some(false) {
            return None;
        }
        let folds = self.roots(Target::Part(part));
        self.target = Target::Part(part);
        self.chain_part = Some(part);
        let bindings = self.at("bindings", |cx| {
            cx.keyed::<Binding>(&compose::children(&folds, "bindings"))
        });
        self.chain
            .push(Bindings::of(part, bindings.unwrap_or_default()));
        PartStyle::resolve(&folds, self)
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
        }
    }

    /// The colour `reference` names in `scope` (S3.3); `None`: not drawn.
    pub(crate) fn evaluate_role<'t>(
        &'t self,
        scope: &'t Scope,
        reference: &Ref,
    ) -> Evaluated<Option<Color>> {
        let mut cx = Cx::new(self, scope, None, &|_| true, Target::Roles);
        let value = match cx.reference(reference, Target::Roles) {
            Some(Answer::Val(val)) => Color::from_val(&val),
            Some(Answer::Place {
                target,
                path,
                bases_only,
            }) => cx.follow(target, path, bases_only, |cx, folds| {
                cx.value::<Color>(folds, false)
            }),
            None => None,
        };
        Evaluated {
            value,
            refusals: cx.refusals,
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
        .flatten()
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
        }
    }
}

#[cfg(test)]
mod tests;
