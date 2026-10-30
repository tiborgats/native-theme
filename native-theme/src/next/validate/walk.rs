//! A walk over a layer's values (S12): every block, patch, `Value` and
//! stated number, each with its path and its field's marks, handed to a
//! [`Visit`]. The `ThemeParts` derive writes the walk of every block and
//! patch; the containers' and leaves' walks are here.

use std::any::Any;
use std::fmt::{self, Display};

use crate::next::blocks::BlockPatch;
use crate::next::blocks::text::LanguageTag;
use crate::next::color::Color;
use crate::next::expr::Expr;
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

/// One step of a path.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Step {
    /// A field, with its owner's name and its marks.
    Field {
        owner: &'static str,
        name: &'static str,
        marks: Marks,
    },
    /// A map's key, or a set's or scheme's name.
    Key(String),
    /// A list's item.
    Index(usize),
}

/// Where a walk is: the path from its root, each field with its marks.
#[derive(Clone, Debug, Default)]
pub(crate) struct Path {
    root: String,
    steps: Vec<Step>,
    /// The marks a part's catalogue entry gives its style's own fields
    /// (S6.3), for the block walked next.
    part: Option<&'static [FieldInfo]>,
    /// An `inner_may_be_absent` place: the fields of the block walked next
    /// may be `Absent`, but those named.
    place: Option<&'static [&'static str]>,
}

impl Path {
    /// A path starting at `root` (`widgets.button.body`, `rules[2].set`).
    pub(crate) fn new(root: &str) -> Path {
        Path {
            root: root.to_owned(),
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
                Step::Key(_) | Step::Index(_) => None,
            })
            .unwrap_or_default()
    }

    /// The fields' names from the root, owner and name, outermost first.
    pub(crate) fn fields(&self) -> impl DoubleEndedIterator<Item = (&'static str, &'static str)> {
        self.steps.iter().filter_map(|step| match step {
            Step::Field { owner, name, .. } => Some((*owner, *name)),
            Step::Key(_) | Step::Index(_) => None,
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
                Step::Key(key) => Some(key.as_str()),
                Step::Field { .. } | Step::Index(_) => None,
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
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.root)?;
        let mut first = self.root.is_empty();
        for step in &self.steps {
            match step {
                Step::Field { name, .. } => {
                    if !first {
                        f.write_str(".")?;
                    }
                    f.write_str(name)?;
                }
                Step::Key(key) => {
                    if !first {
                        f.write_str(".")?;
                    }
                    f.write_str(key)?;
                }
                Step::Index(index) => write!(f, "[{index}]")?,
            }
            first = false;
        }
        Ok(())
    }
}

/// What a walk hands each thing it meets to.
pub(crate) trait Visit {
    /// A block, a patch, an enum or a leaf, before what it holds.
    fn node(&mut self, path: &Path, node: &dyn Any);

    /// A `Value`'s form, the expression form its type takes.
    fn value(&mut self, path: &Path, form: Form<'_>, expr: ExprForm);

    /// A stated number.
    fn number(&mut self, path: &Path, n: f32, number: Number);

    /// A map's key stated twice, `key` as the map spells it.
    fn duplicate(&mut self, path: &Path, key: &str);

    /// A map's key, `key` as the map spells it, at the map's path.
    fn map_key(&mut self, path: &Path, key: &str);

    /// A patch's fields: every one its block has, and those it sets.
    fn patch(&mut self, path: &Path, all: &'static [&'static str], set: &[&'static str]);
}

/// A walk in progress.
pub(crate) struct Walker<'v> {
    path: Path,
    visit: &'v mut dyn Visit,
}

impl<'v> Walker<'v> {
    /// A walk from `root`, handing what it meets to `visit`.
    pub(crate) fn new(root: &str, visit: &'v mut dyn Visit) -> Walker<'v> {
        Walker {
            path: Path::new(root),
            visit,
        }
    }

    /// Walks `value` from this walk's root.
    pub(crate) fn walk<T: Walk + ?Sized>(&mut self, value: &T) {
        value.walk(self);
    }

    /// Hands `node` to the visit.
    pub(crate) fn node(&mut self, node: &dyn Any) {
        self.visit.node(&self.path, node);
    }

    /// Hands a `Value`'s form to the visit.
    pub(crate) fn value(&mut self, form: Form<'_>, expr: ExprForm) {
        self.visit.value(&self.path, form, expr);
    }

    /// Hands a patch's fields to the visit.
    pub(crate) fn patch(&mut self, all: &'static [&'static str], set: &[&'static str]) {
        self.visit.patch(&self.path, all, set);
    }

    /// Hands a stated number to the visit.
    pub(crate) fn number(&mut self, n: f32, number: Number) {
        self.visit.number(&self.path, n, number);
    }

    /// Walks the field `name` of the block `owner`, of marks `marks`, by
    /// `walk`; the place's marks (a part's catalogue entry, an
    /// `inner_may_be_absent` field) added to the field's own.
    pub(crate) fn field(
        &mut self,
        owner: &'static str,
        name: &'static str,
        mut marks: Marks,
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
        self.path.steps.push(Step::Field { owner, name, marks });
        walk(self);
        self.path.steps.pop();
        self.path.part = part;
        self.path.place = place;
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

    /// Walks below the key `key`.
    pub(crate) fn key(&mut self, key: String, walk: impl FnOnce(&mut Walker<'_>)) {
        let held = (self.path.part.take(), self.path.place.take());
        self.path.steps.push(Step::Key(key));
        walk(self);
        self.path.steps.pop();
        (self.path.part, self.path.place) = held;
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
pub(crate) trait Walk: Any {
    /// What an expression stated on a `Value` of this type may be.
    const EXPR: ExprForm = ExprForm::ReferenceOnly;

    /// Hands `self`, and what it holds, to the walk.
    fn walk(&self, w: &mut Walker<'_>);

    /// A patch's fields it sets, by their names; none for any other value.
    fn set_fields(&self) -> Vec<&'static str> {
        Vec::new()
    }
}

/// A map, `Vec<(K, V)>`: each key stated once (S12), each value walked
/// below its key.
pub(crate) fn walk_map<K: MapKey + PartialEq, V: Walk>(map: &[(K, V)], w: &mut Walker<'_>) {
    for (index, (key, value)) in map.iter().enumerate() {
        let name = key.key_name();
        if map.iter().take(index).any(|(earlier, _)| earlier == key) {
            w.visit.duplicate(&w.path, &name);
        }
        w.visit.map_key(&w.path, &name);
        w.key(name, |w| value.walk(w));
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
            w.visit.duplicate(&w.path, &name);
        }
        w.visit.map_key(&w.path, &name);
        w.key(name, |w| walk_map(inner, w));
    }
}

impl<T: Walk> Walk for Value<T> {
    const EXPR: ExprForm = T::EXPR;

    fn walk(&self, w: &mut Walker<'_>) {
        match self {
            Value::Stated(value) => {
                w.value(Form::Stated, T::EXPR);
                value.walk(w);
            }
            Value::Expr(expr) => w.value(Form::Expr(expr), T::EXPR),
            Value::Absent => w.value(Form::Absent, T::EXPR),
            Value::Auto => w.value(Form::Auto, T::EXPR),
        }
    }
}

impl<T: Walk> Walk for Option<T> {
    const EXPR: ExprForm = T::EXPR;

    fn walk(&self, w: &mut Walker<'_>) {
        if let Some(value) = self {
            value.walk(w);
        }
    }
}

impl<T: Walk> Walk for Vec<T> {
    fn walk(&self, w: &mut Walker<'_>) {
        w.node(self);
        for (index, item) in self.iter().enumerate() {
            w.index(index, |w| item.walk(w));
        }
    }
}

impl<T: Walk> Walk for Box<T> {
    fn walk(&self, w: &mut Walker<'_>) {
        T::walk(&**self, w);
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
            w.field("Sides", name, marks, |w| member.walk(w));
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
            w.field("Axes", name, marks, |w| member.walk(w));
        }
    }
}

impl<T: Walk> Walk for SurfaceMap<T> {
    fn walk(&self, w: &mut Walker<'_>) {
        w.node(self);
        for (set, value) in self.iter() {
            w.key(set.name().to_owned(), |w| value.walk(w));
        }
    }
}

impl<T: Walk> Walk for SchemeMap<T> {
    fn walk(&self, w: &mut Walker<'_>) {
        for (name, value) in [("light", &self.light), ("dark", &self.dark)] {
            if let Some(value) = value {
                w.key(name.to_owned(), |w| value.walk(w));
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
}

/// A leaf with a number of a kind.
macro_rules! numbers {
    ($($ty:ty => $number:expr;)*) => {
        $(
            impl Walk for $ty {
                const EXPR: ExprForm = ExprForm::Calls;

                fn walk(&self, w: &mut Walker<'_>) {
                    w.node(self);
                    w.number(self.0, $number);
                }
            }
        )*
    };
}

numbers! {
    Px => Number::Length;
    Ms => Number::Duration;
    Deg => Number::Angle;
    Fraction => Number::Fraction;
}

impl Walk for f32 {
    const EXPR: ExprForm = ExprForm::Calls;

    fn walk(&self, w: &mut Walker<'_>) {
        w.node(self);
        w.number(*self, Number::Plain);
    }
}

impl Walk for FontSize {
    const EXPR: ExprForm = ExprForm::Calls;

    fn walk(&self, w: &mut Walker<'_>) {
        w.node(self);
        match self {
            FontSize::Pt(n) | FontSize::Px(n) => w.number(*n, Number::FontSize),
        }
    }
}

impl Walk for Scalar {
    const EXPR: ExprForm = ExprForm::Calls;

    fn walk(&self, w: &mut Walker<'_>) {
        w.node(self);
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

    fn walk(&self, w: &mut Walker<'_>) {
        w.node(self);
        for channel in [self.r, self.g, self.b, self.a] {
            w.number(channel, Number::Channel);
        }
    }
}

impl Walk for (f32, f32) {
    fn walk(&self, w: &mut Walker<'_>) {
        w.node(self);
        w.number(self.0, Number::Plain);
        w.number(self.1, Number::Plain);
    }
}

/// A leaf holding no number: walked as a node only.
macro_rules! leaves {
    ($form:expr => $($ty:ty),* $(,)?) => {
        $(
            impl Walk for $ty {
                const EXPR: ExprForm = $form;

                fn walk(&self, w: &mut Walker<'_>) {
                    w.node(self);
                }
            }
        )*
    };
}

leaves!(ExprForm::Calls => u16, u32);
leaves!(ExprForm::NoCalls => bool, String, Key, LanguageTag, IconRole, IconSet);
leaves!(ExprForm::NoCalls => SurfaceSet, SurfaceRole, GlobalRole, PartRef, VariantRef, WidgetRef);

impl Walk for ViewBox {
    fn walk(&self, w: &mut Walker<'_>) {
        w.node(self);
        for n in [self.x, self.y, self.width, self.height] {
            w.number(n, Number::Plain);
        }
    }
}
