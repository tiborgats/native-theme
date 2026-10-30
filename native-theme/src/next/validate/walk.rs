//! A walk over a layer's values (S12): every block, patch, `Value` and
//! stated number, each with its path and its field's marks, handed to a
//! [`Visit`]. The `ThemeParts` derive writes the walk of every block and
//! patch; the containers' and leaves' walks are here.
//!
//! A path names its place as the layer was written (S1's `Problem.path`): in
//! the model's Rust names for a layer built in Rust, an enum's variant a step
//! of its own (`top_left.concentric.minimum`); by the TOML keys as S11 spells
//! them for a layer read from TOML (`min_width_px`, `top_left_px`,
//! `durations.button_ms`).

use std::fmt::{self, Display};

use crate::next::blocks::BlockPatch;
use crate::next::blocks::icon::{IconStyle, IconStylePatch};
use crate::next::blocks::motion::{
    Loop, LoopPatch, PresentationKind, Timing, Transition, TransitionKind, TransitionPatch,
};
use crate::next::blocks::outline::Radius;
use crate::next::blocks::paint::{Paint, PaintKind, PaintPatch};
use crate::next::blocks::shadow::Shadow;
use crate::next::blocks::text::{Font, FontPatch, LanguageTag};
use crate::next::color::Color;
use crate::next::expr::{EnumTy, Expr, SURFACE_SET, Ty};
use crate::next::key::Key;
use crate::next::registry::FieldInfo;
use crate::next::roles::{GlobalRole, SurfaceRole, SurfaceSet};
use crate::next::selector::{PartRef, VariantRef, WidgetRef};
use crate::next::toml::read::MapKey;
use crate::next::units::{Axes, Deg, FontSize, Fraction, Ms, Px, Scalar, Sides, ViewBox};
use crate::next::value::{SchemeMap, SurfaceMap, Value};
use crate::{IconRole, IconSet};

/// A field's marks (S6.3), and whether its length may be negative (S1).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Marks {
    /// The field may be `Absent`.
    pub(crate) may_be_absent: bool,
    /// The field may be `Auto`.
    pub(crate) auto: bool,
    /// The field's length may be negative (offsets, translations, letter and
    /// word spacing, margins).
    pub(crate) signed: bool,
}

/// What an expression stated on a `Value` of a type may be (S2.2, S12).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ExprForm {
    /// A literal, a reference or a call: a scalar, a colour or a paint.
    Calls,
    /// A literal or a reference, no call: a bool, a text, a key or an enum.
    NoCalls,
    /// A reference only: a block.
    ReferenceOnly,
}

/// A `Value`'s form.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Form<'a> {
    /// `Absent`.
    Absent,
    /// `Auto`.
    Auto,
    /// An expression.
    Expr(&'a Expr),
    /// A stated value, walked next.
    Stated,
}

/// What a stated number is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Number {
    /// A length.
    Length,
    /// A font size, a line height or a spacing.
    FontSize,
    /// A duration.
    Duration,
    /// An angle.
    Angle,
    /// A `Fraction`, `0..=1`.
    Fraction,
    /// A plain number, or a keyed scalar declared `number` or `fraction`.
    Plain,
    /// A colour's channel.
    Channel,
    /// A keyed scalar's length, duration or angle, whose sign no field
    /// states (S4.6).
    Keyed,
}

/// A block or a leaf with constraints of its own (S12), handed to the visit
/// where the walk meets it. The `ThemeParts` derive hands a block marked
/// `#[parts(check)]` over as its variant here, so a marked block without a
/// variant, or a variant no block is handed over as, does not compile.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Node<'a> {
    /// A paint.
    Paint(&'a Paint),
    /// A paint's patch.
    PaintPatch(&'a PaintPatch),
    /// A paint layer's kind.
    PaintKind(&'a PaintKind),
    /// A shadow.
    Shadow(&'a Shadow),
    /// A transition.
    Transition(&'a Transition),
    /// A transition's patch.
    TransitionPatch(&'a TransitionPatch),
    /// A transition's kind.
    TransitionKind(&'a TransitionKind),
    /// A presentation step's kind.
    PresentationKind(&'a PresentationKind),
    /// A timing.
    Timing(&'a Timing),
    /// A loop.
    Loop(&'a Loop),
    /// A loop's patch.
    LoopPatch(&'a LoopPatch),
    /// A corner's radius.
    Radius(&'a Radius),
    /// A font.
    Font(&'a Font),
    /// A font's patch.
    FontPatch(&'a FontPatch),
    /// An icon style.
    IconStyle(&'a IconStyle),
    /// An icon style's patch.
    IconStylePatch(&'a IconStylePatch),
    /// A keyed scalar.
    Scalar(&'a Scalar),
    /// A key.
    Key(&'a Key),
    /// A surface set: a value, or a map's key.
    SurfaceSet(SurfaceSet),
}

/// One step of a path.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Step {
    /// A field, with its owner's name, its marks and the unit suffix its key
    /// takes in TOML.
    Field {
        owner: &'static str,
        name: &'static str,
        marks: Marks,
        suffix: &'static str,
    },
    /// An enum's variant holding data, with the enum's name; `folded`: TOML
    /// spells it by its holder's key suffix (`top_left_px`, S11), not as a
    /// key of its own.
    Variant {
        owner: &'static str,
        name: &'static str,
        folded: bool,
    },
    /// A map's key, or a set's or scheme's name, and the unit suffix the
    /// key takes in TOML (`button_ms`, S11).
    Key { key: String, suffix: &'static str },
    /// A list's item.
    Index(usize),
}

/// Where a walk is: the path from its root, each field with its marks.
#[derive(Clone, Debug, Default)]
pub(crate) struct Path {
    root: String,
    /// Spelled by the TOML keys a layer was read under, not the model's
    /// names.
    toml: bool,
    steps: Vec<Step>,
    /// The marks a part's catalogue entry gives its style's own fields
    /// (S6.3), for the block walked next.
    part: Option<&'static [FieldInfo]>,
    /// An `inner_may_be_absent` place: the fields of the block walked next
    /// may be `Absent`, but those named.
    place: Option<&'static [&'static str]>,
}

/// `key` as a TOML key: bare where TOML allows it, else quoted.
pub(crate) fn toml_key(key: &str) -> String {
    let bare = !key.is_empty()
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    if bare {
        key.to_owned()
    } else {
        format!("{key:?}")
    }
}

impl Path {
    /// A path starting at `root` (`widgets.button.body`, `rules[2].set`),
    /// spelled by TOML's keys where `toml`.
    pub(crate) fn new(root: &str, toml: bool) -> Path {
        Path {
            root: root.to_owned(),
            toml,
            ..Path::default()
        }
    }

    /// The marks of the field the walk is in; none at the root.
    pub(crate) fn marks(&self) -> Marks {
        self.steps
            .iter()
            .rev()
            .find_map(|step| match step {
                Step::Field { marks, .. } => Some(*marks),
                Step::Variant { .. } | Step::Key { .. } | Step::Index(_) => None,
            })
            .unwrap_or_default()
    }

    /// The fields' names from the root, owner and name, outermost first.
    pub(crate) fn fields(&self) -> impl DoubleEndedIterator<Item = (&'static str, &'static str)> {
        self.steps.iter().filter_map(|step| match step {
            Step::Field { owner, name, .. } => Some((*owner, *name)),
            Step::Variant { .. } | Step::Key { .. } | Step::Index(_) => None,
        })
    }

    /// The innermost field, owner and name.
    pub(crate) fn last_field(&self) -> Option<(&'static str, &'static str)> {
        self.fields().next_back()
    }

    /// The outermost field below the root, owner and name.
    pub(crate) fn first_field(&self) -> Option<(&'static str, &'static str)> {
        self.fields().next()
    }

    /// The innermost field or variant holding the walk, its owner and its
    /// name: the slot a leaf or a map's key is in (`IconStyle.scale`,
    /// `BlendMode.keyed`, `MotionTokens.durations`).
    pub(crate) fn slot(&self) -> Option<(&'static str, &'static str)> {
        self.steps.iter().rev().find_map(|step| match step {
            Step::Field { owner, name, .. } | Step::Variant { owner, name, .. } => {
                Some((*owner, *name))
            }
            Step::Key { .. } | Step::Index(_) => None,
        })
    }

    /// The map field whose entry the walk is at: its last step a key, the
    /// one before it the map's field, owner and name.
    pub(crate) fn map_entry(&self) -> Option<(&'static str, &'static str)> {
        match self.steps.as_slice() {
            [.., Step::Field { owner, name, .. }, Step::Key { .. }] => Some((*owner, *name)),
            _ => None,
        }
    }

    /// The keys stated after the innermost field (a map of maps' outer key).
    pub(crate) fn keys(&self) -> Vec<&str> {
        let tail = self
            .steps
            .iter()
            .rposition(|step| matches!(step, Step::Field { .. }))
            .and_then(|last| self.steps.get(last..))
            .unwrap_or(&self.steps);
        tail.iter()
            .filter_map(|step| match step {
                Step::Key { key, .. } => Some(key.as_str()),
                Step::Field { .. } | Step::Variant { .. } | Step::Index(_) => None,
            })
            .collect()
    }

    /// The catalogue entry's fields of the part whose style is walked next.
    pub(crate) fn part_fields(&self) -> Option<&'static [FieldInfo]> {
        self.part
    }

    /// Whether the walk is below the field `name` of the block `owner`.
    pub(crate) fn within(&self, owner: &str, name: &str) -> bool {
        self.fields().any(|(o, n)| o == owner && n == name)
    }

    /// The path below this one by `key`, a field of no unit (`properties`,
    /// `layers[0].blend`), as text.
    pub(crate) fn join(&self, key: &str) -> String {
        let at = self.to_string();
        if at.is_empty() {
            key.to_owned()
        } else {
            format!("{at}.{key}")
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.root)?;
        let mut first = self.root.is_empty();
        for step in &self.steps {
            let (name, suffix) = match step {
                Step::Field { name, suffix, .. } => (Some((*name).to_owned()), *suffix),
                Step::Variant { name, folded, .. } => {
                    if self.toml && *folded {
                        continue;
                    }
                    (Some((*name).to_owned()), "")
                }
                Step::Key { key, suffix } => (Some(toml_key(key)), *suffix),
                Step::Index(index) => {
                    write!(f, "[{index}]")?;
                    first = false;
                    continue;
                }
            };
            if let Some(name) = name {
                if !first {
                    f.write_str(".")?;
                }
                f.write_str(&name)?;
                if self.toml {
                    f.write_str(suffix)?;
                }
            }
            first = false;
        }
        Ok(())
    }
}

/// What a walk hands each thing it meets to.
pub(crate) trait Visit {
    /// A block or a leaf with constraints of its own, before what it holds.
    fn check(&mut self, path: &Path, node: Node<'_>);

    /// A `Value`'s form, the expression form its type takes and its type
    /// (S2.2), `None` where the place decides it.
    fn value(&mut self, path: &Path, form: Form<'_>, expr: ExprForm, ty: Option<Ty>);

    /// A stated number.
    fn number(&mut self, path: &Path, n: f32, number: Number);

    /// A map's key stated twice, `key` as the map spells it.
    fn duplicate(&mut self, path: &Path, key: &str, suffix: &'static str);

    /// A map's key, `key` as the map spells it, at the map's path.
    fn map_key(&mut self, path: &Path, key: &str, suffix: &'static str);

    /// A patch's fields: every one its block has, and those it sets.
    fn patch(&mut self, path: &Path, all: &'static [&'static str], set: &[&'static str]);
}

/// A walk in progress.
pub(crate) struct Walker<'v> {
    path: Path,
    visit: &'v mut dyn Visit,
}

impl<'v> Walker<'v> {
    /// A walk from `root`, spelled by TOML's keys where `toml`, handing what
    /// it meets to `visit`.
    pub(crate) fn new(root: &str, toml: bool, visit: &'v mut dyn Visit) -> Walker<'v> {
        Walker {
            path: Path::new(root, toml),
            visit,
        }
    }

    /// Walks `value` from this walk's root.
    pub(crate) fn walk<T: Walk + ?Sized>(&mut self, value: &T) {
        value.walk(self);
    }

    /// Hands a block or leaf with constraints of its own to the visit.
    pub(crate) fn check(&mut self, node: Node<'_>) {
        self.visit.check(&self.path, node);
    }

    /// Hands a `Value`'s form to the visit.
    pub(crate) fn value(&mut self, form: Form<'_>, expr: ExprForm, ty: Option<Ty>) {
        self.visit.value(&self.path, form, expr, ty);
    }

    /// Hands a patch's fields to the visit.
    pub(crate) fn patch(&mut self, all: &'static [&'static str], set: &[&'static str]) {
        self.visit.patch(&self.path, all, set);
    }

    /// Hands a stated number to the visit.
    pub(crate) fn number(&mut self, n: f32, number: Number) {
        self.visit.number(&self.path, n, number);
    }

    /// Walks the field `name` of the block `owner`, of marks `marks`, its
    /// TOML key taking `suffix`, by `walk`; the place's marks (a part's
    /// catalogue entry, an `inner_may_be_absent` field) added to the field's
    /// own.
    pub(crate) fn field(
        &mut self,
        owner: &'static str,
        name: &'static str,
        mut marks: Marks,
        suffix: &'static str,
        walk: impl FnOnce(&mut Walker<'_>),
    ) {
        if let Some(fields) = self.path.part
            && let Some(info) = fields.iter().find(|info| info.path == name)
        {
            marks.may_be_absent |= info.may_be_absent;
            marks.auto |= info.auto;
        }
        if let Some(required) = self.path.place {
            marks.may_be_absent |= !required.contains(&name);
        }
        let (part, place) = (self.path.part.take(), self.path.place.take());
        self.path.steps.push(Step::Field {
            owner,
            name,
            marks,
            suffix,
        });
        walk(self);
        self.path.steps.pop();
        self.path.part = part;
        self.path.place = place;
    }

    /// Walks the variant `name` of the enum `owner` by `walk`; `folded`: TOML
    /// spells it by its holder's key suffix.
    pub(crate) fn variant(
        &mut self,
        owner: &'static str,
        name: &'static str,
        folded: bool,
        walk: impl FnOnce(&mut Walker<'_>),
    ) {
        let held = (self.path.part.take(), self.path.place.take());
        self.path.steps.push(Step::Variant {
            owner,
            name,
            folded,
        });
        walk(self);
        self.path.steps.pop();
        (self.path.part, self.path.place) = held;
    }

    /// Walks the block `walk` walks as the style of a part whose catalogue
    /// entry is `fields`.
    pub(crate) fn part(
        &mut self,
        fields: &'static [FieldInfo],
        walk: impl FnOnce(&mut Walker<'_>),
    ) {
        let outer = self.path.part.replace(fields);
        walk(self);
        self.path.part = outer;
    }

    /// Walks the block `walk` walks in an `inner_may_be_absent` place,
    /// `required` the fields it leaves out.
    pub(crate) fn place(
        &mut self,
        required: &'static [&'static str],
        walk: impl FnOnce(&mut Walker<'_>),
    ) {
        let outer = self.path.place.replace(required);
        walk(self);
        self.path.place = outer;
    }

    /// Walks below the key `key`, whose TOML spelling takes `suffix`; an
    /// `inner_may_be_absent` place holding a map (`IconSizeTokens.keyed`)
    /// is kept for each entry's value.
    pub(crate) fn key(
        &mut self,
        key: String,
        suffix: &'static str,
        walk: impl FnOnce(&mut Walker<'_>),
    ) {
        let part = self.path.part.take();
        self.path.steps.push(Step::Key { key, suffix });
        walk(self);
        self.path.steps.pop();
        self.path.part = part;
    }

    /// Walks below the list item `index`.
    pub(crate) fn index(&mut self, index: usize, walk: impl FnOnce(&mut Walker<'_>)) {
        let held = (self.path.part.take(), self.path.place.take());
        self.path.steps.push(Step::Index(index));
        walk(self);
        self.path.steps.pop();
        (self.path.part, self.path.place) = held;
    }
}

/// A value a walk goes through.
pub(crate) trait Walk {
    /// What an expression stated on a `Value` of this type may be.
    const EXPR: ExprForm = ExprForm::ReferenceOnly;

    /// The unit suffix a key holding this type's expression, `"auto"` or
    /// `"absent"` takes in TOML (S11): a length's `_px` for every form, a
    /// font size's none.
    const SUFFIX: &'static str = "";

    /// Hands `self`, and what it holds, to the walk.
    fn walk(&self, w: &mut Walker<'_>);

    /// The unit suffix a key holding this value takes in TOML: a stated
    /// font size's by its variant (`size_pt`), every other value its type's.
    fn suffix(&self) -> &'static str {
        Self::SUFFIX
    }

    /// A patch's fields it sets, by their names; none for any other value.
    fn set_fields(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// The type an expression stated on a `Value` of this type has, and a
    /// reference to such a value (S2.2); `None` where the place decides it
    /// (a key, a keyed scalar) or nothing can (a list).
    fn ty() -> Option<Ty> {
        None
    }
}

/// A map, `Vec<(K, V)>`: each key stated once (S12), each value walked
/// below its key.
pub(crate) fn walk_map<K: MapKey + PartialEq, V: Walk>(map: &[(K, V)], w: &mut Walker<'_>) {
    for (index, (key, value)) in map.iter().enumerate() {
        let name = key.key_name();
        let suffix = value.suffix();
        if map.iter().take(index).any(|(earlier, _)| earlier == key) {
            w.visit.duplicate(&w.path, &name, suffix);
        }
        w.visit.map_key(&w.path, &name, suffix);
        w.key(name, suffix, |w| value.walk(w));
    }
}

/// A map of maps: each level a map.
pub(crate) fn walk_nested_map<K: MapKey + PartialEq, L: MapKey + PartialEq, V: Walk>(
    map: &[(K, Vec<(L, V)>)],
    w: &mut Walker<'_>,
) {
    for (index, (key, inner)) in map.iter().enumerate() {
        let name = key.key_name();
        if map.iter().take(index).any(|(earlier, _)| earlier == key) {
            w.visit.duplicate(&w.path, &name, "");
        }
        w.visit.map_key(&w.path, &name, "");
        w.key(name, "", |w| walk_map(inner, w));
    }
}

impl<T: Walk> Walk for Value<T> {
    const EXPR: ExprForm = T::EXPR;
    const SUFFIX: &'static str = T::SUFFIX;

    fn walk(&self, w: &mut Walker<'_>) {
        match self {
            Value::Stated(value) => {
                w.value(Form::Stated, T::EXPR, None);
                value.walk(w);
            }
            Value::Expr(expr) => w.value(Form::Expr(expr), T::EXPR, T::ty()),
            Value::Absent => w.value(Form::Absent, T::EXPR, None),
            Value::Auto => w.value(Form::Auto, T::EXPR, None),
        }
    }

    fn suffix(&self) -> &'static str {
        match self {
            Value::Stated(value) => value.suffix(),
            Value::Expr(_) | Value::Absent | Value::Auto => T::SUFFIX,
        }
    }

    fn ty() -> Option<Ty> {
        T::ty()
    }
}

impl<T: Walk> Walk for Option<T> {
    const EXPR: ExprForm = T::EXPR;
    const SUFFIX: &'static str = T::SUFFIX;

    fn walk(&self, w: &mut Walker<'_>) {
        if let Some(value) = self {
            value.walk(w);
        }
    }

    fn suffix(&self) -> &'static str {
        self.as_ref().map_or(T::SUFFIX, Walk::suffix)
    }

    fn ty() -> Option<Ty> {
        T::ty()
    }
}

impl<T: Walk> Walk for Vec<T> {
    fn walk(&self, w: &mut Walker<'_>) {
        for (index, item) in self.iter().enumerate() {
            w.index(index, |w| item.walk(w));
        }
    }
}

impl<T: Walk> Walk for Box<T> {
    const EXPR: ExprForm = T::EXPR;
    const SUFFIX: &'static str = T::SUFFIX;

    fn walk(&self, w: &mut Walker<'_>) {
        T::walk(&**self, w);
    }

    fn suffix(&self) -> &'static str {
        T::suffix(&**self)
    }

    fn ty() -> Option<Ty> {
        T::ty()
    }
}

impl<T: Walk> Walk for Sides<T> {
    fn walk(&self, w: &mut Walker<'_>) {
        let marks = w.path.marks();
        for (name, member) in [
            ("top", &self.top),
            ("right", &self.right),
            ("bottom", &self.bottom),
            ("left", &self.left),
        ] {
            w.field("Sides", name, marks, member.suffix(), |w| member.walk(w));
        }
    }
}

impl<T: Walk> Walk for Axes<T> {
    fn walk(&self, w: &mut Walker<'_>) {
        let marks = w.path.marks();
        for (name, member) in [
            ("horizontal", &self.horizontal),
            ("vertical", &self.vertical),
        ] {
            w.field("Axes", name, marks, member.suffix(), |w| member.walk(w));
        }
    }
}

impl<T: Walk> Walk for SurfaceMap<T> {
    fn ty() -> Option<Ty> {
        T::ty()
    }

    fn walk(&self, w: &mut Walker<'_>) {
        for (set, value) in self.iter() {
            w.key(set.name().to_owned(), value.suffix(), |w| {
                w.check(Node::SurfaceSet(set));
                value.walk(w);
            });
        }
    }
}

impl<T: Walk> Walk for SchemeMap<T> {
    fn walk(&self, w: &mut Walker<'_>) {
        for (name, value) in [("light", &self.light), ("dark", &self.dark)] {
            if let Some(value) = value {
                w.key(name.to_owned(), value.suffix(), |w| value.walk(w));
            }
        }
    }
}

impl<T: Walk + crate::next::blocks::FieldsBlock> Walk for BlockPatch<T>
where
    T::Patch: Walk,
{
    fn walk(&self, w: &mut Walker<'_>) {
        match self {
            BlockPatch::Set(value) => value.walk(w),
            BlockPatch::Fields(patch) => patch.walk(w),
        }
    }

    fn ty() -> Option<Ty> {
        T::ty()
    }
}

/// A leaf with a number of a kind, keyed by its unit's suffix.
macro_rules! numbers {
    ($($ty:ty => $number:expr, $suffix:expr, $typed:expr;)*) => {
        $(
            impl Walk for $ty {
                const EXPR: ExprForm = ExprForm::Calls;
                const SUFFIX: &'static str = $suffix;

                fn walk(&self, w: &mut Walker<'_>) {
                    w.number(self.0, $number);
                }

                fn ty() -> Option<Ty> {
                    Some($typed)
                }
            }
        )*
    };
}

numbers! {
    Px => Number::Length, "_px", Ty::Length;
    Ms => Number::Duration, "_ms", Ty::Duration;
    Deg => Number::Angle, "_deg", Ty::Angle;
    Fraction => Number::Fraction, "", Ty::Number;
}

impl Walk for f32 {
    const EXPR: ExprForm = ExprForm::Calls;

    fn walk(&self, w: &mut Walker<'_>) {
        w.number(*self, Number::Plain);
    }

    fn ty() -> Option<Ty> {
        Some(Ty::Number)
    }
}

/// A font size's number is keyed by its unit, its other forms by the bare
/// name (S11).
impl Walk for FontSize {
    const EXPR: ExprForm = ExprForm::Calls;

    fn walk(&self, w: &mut Walker<'_>) {
        match self {
            FontSize::Pt(n) | FontSize::Px(n) => w.number(*n, Number::FontSize),
        }
    }

    fn suffix(&self) -> &'static str {
        match self {
            FontSize::Pt(_) => "_pt",
            FontSize::Px(_) => "_px",
        }
    }

    fn ty() -> Option<Ty> {
        Some(Ty::FontSize)
    }
}

/// A keyed scalar's key takes no unit suffix in TOML (S11).
impl Walk for Scalar {
    const EXPR: ExprForm = ExprForm::Calls;

    fn walk(&self, w: &mut Walker<'_>) {
        w.check(Node::Scalar(self));
        match self {
            Scalar::Px(Px(n)) | Scalar::Ms(Ms(n)) | Scalar::Deg(Deg(n)) => {
                w.number(*n, Number::Keyed);
            }
            Scalar::Fraction(Fraction(n)) => w.number(*n, Number::Fraction),
            Scalar::Number(n) => w.number(*n, Number::Plain),
        }
    }
}

impl Walk for Color {
    const EXPR: ExprForm = ExprForm::Calls;

    fn ty() -> Option<Ty> {
        Some(Ty::Color)
    }

    fn walk(&self, w: &mut Walker<'_>) {
        for channel in [self.r, self.g, self.b, self.a] {
            w.number(channel, Number::Channel);
        }
    }
}

impl Walk for (f32, f32) {
    fn walk(&self, w: &mut Walker<'_>) {
        w.number(self.0, Number::Plain);
        w.number(self.1, Number::Plain);
    }
}

/// A leaf holding no number and no constraint of its own, of a type.
macro_rules! leaves {
    ($form:expr => $($ty:ty: $typed:expr),* $(,)?) => {
        $(
            impl Walk for $ty {
                const EXPR: ExprForm = $form;

                fn walk(&self, _: &mut Walker<'_>) {}

                fn ty() -> Option<Ty> {
                    $typed
                }
            }
        )*
    };
}

leaves!(ExprForm::Calls => u16: Some(Ty::Number), u32: Some(Ty::Number));
leaves!(ExprForm::NoCalls => bool: Some(Ty::Bool), String: Some(Ty::Text));
leaves!(ExprForm::NoCalls => IconSet: Some(Ty::Enum(&ICON_SET)));
leaves!(ExprForm::NoCalls => LanguageTag: None, IconRole: None, SurfaceRole: None);
leaves!(ExprForm::NoCalls => PartRef: None, VariantRef: None, WidgetRef: None);

/// The declaration a key's type names until its place names the one it is
/// a key of (S10): a field's, or a widget value's `keys` entry.
pub(crate) const KEY_OF_PLACE: &str = "";

/// `IconSet` (S4.5), its names as TOML writes them: in snake_case.
static ICON_SET: EnumTy = EnumTy {
    name: "IconSet",
    variants: &[
        "sf_symbols",
        "segoe_fluent",
        "freedesktop",
        "material",
        "lucide",
    ],
};

/// A global role: an `accent_ramp` entry's key is a keyed name (S10).
impl Walk for GlobalRole {
    const EXPR: ExprForm = ExprForm::NoCalls;

    fn walk(&self, w: &mut Walker<'_>) {
        if let GlobalRole::AccentRamp(key) = self {
            w.variant("GlobalRole", "accent_ramp", false, |w| key.walk(w));
        }
    }
}

impl Walk for Key {
    const EXPR: ExprForm = ExprForm::NoCalls;

    /// A key of a declaration the place names: [`KEY_OF_PLACE`].
    fn ty() -> Option<Ty> {
        Some(Ty::Key(std::borrow::Cow::Borrowed(KEY_OF_PLACE)))
    }

    fn walk(&self, w: &mut Walker<'_>) {
        w.check(Node::Key(self));
    }
}

impl Walk for SurfaceSet {
    const EXPR: ExprForm = ExprForm::NoCalls;

    fn ty() -> Option<Ty> {
        Some(Ty::Enum(&SURFACE_SET))
    }

    fn walk(&self, w: &mut Walker<'_>) {
        w.check(Node::SurfaceSet(*self));
    }
}

impl Walk for ViewBox {
    fn walk(&self, w: &mut Walker<'_>) {
        for n in [self.x, self.y, self.width, self.height] {
            w.number(n, Number::Plain);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_spells_the_model_or_the_toml() {
        let steps = vec![
            Step::Field {
                owner: "PartStyle",
                name: "corners",
                marks: Marks::default(),
                suffix: "",
            },
            Step::Field {
                owner: "Corners",
                name: "top_left",
                marks: Marks::default(),
                suffix: "_px",
            },
            Step::Variant {
                owner: "Radius",
                name: "px",
                folded: true,
            },
        ];
        let mut path = Path::new("widgets.button.body", false);
        path.steps = steps.clone();
        assert_eq!(path.to_string(), "widgets.button.body.corners.top_left.px");
        path.toml = true;
        assert_eq!(path.to_string(), "widgets.button.body.corners.top_left_px");
        path.steps = vec![Step::Key {
            key: "Segoe Fluent Icons".to_owned(),
            suffix: "",
        }];
        assert_eq!(
            path.to_string(),
            "widgets.button.body.\"Segoe Fluent Icons\""
        );
    }

    /// `ICON_SET` names each icon set as the reader takes it: its name in
    /// snake_case.
    #[test]
    fn icon_set_names_are_the_readers() {
        let sets = [
            IconSet::SfSymbols,
            IconSet::SegoeFluent,
            IconSet::Freedesktop,
            IconSet::Material,
            IconSet::Lucide,
        ];
        let names: Vec<String> = sets
            .iter()
            .map(|set| set.name().replace('-', "_"))
            .collect();
        assert_eq!(names, ICON_SET.variants);
    }
}
