//! The reader's core (S11): the problems it collects, the keys a field is
//! spelled by, and the reading of every value the model's fields hold. The
//! `ThemeParts` derive writes the blocks' readers over these.

use ::toml::Table;
use ::toml::Value as Item;

use crate::blocks::paint::{Paint, PaintPatch};
use crate::blocks::{BlockPatch, FieldsBlock};
use crate::color::Color;
use crate::error::{Problem, ProblemKind};
use crate::expr::{Expr, Ty};
use crate::inputs::InputName;
use crate::key::Key;
use crate::layer::Shorthands;
use crate::registry::{self, FieldInfo, FieldType};
use crate::roles::SurfaceSet;
use crate::selector::{Scheme, VariantRef};
use crate::units::{Axes, Deg, FontSize, Fraction, Ms, Px, Scalar, Sides};
use crate::value::{SurfaceMap, Value};
use crate::widgets::{TypedPart, WidgetKind};

/// What a reading collects: every problem, not the first only, and what the
/// place being read decides.
#[derive(Debug, Default)]
pub(crate) struct Reader {
    problems: Vec<Problem>,
    /// Reading a preset's own bases, where a table states a whole block, not
    /// a `Fields` patch (S2.2, S11).
    pub(crate) whole: bool,
    /// Reading a layer's bases, where `append` is refused (S6.1).
    pub(crate) bases: bool,
    /// Reading the fields a preset's own bases keep beneath a part's
    /// `absent = true` or `drawn_as`: a block's table there stays a patch,
    /// its completeness S12's where a later `absent = false` draws the part
    /// (S6.3, S11, S12).
    pub(crate) latent: bool,
    /// The typed widget whose table is read: a bare part name names one of
    /// its parts.
    pub(crate) widget: Option<WidgetKind>,
    /// The key each field was first read under, by the field's path in Rust
    /// names: a second key for it is reported naming the first as written.
    keys: Vec<(String, String)>,
    /// The shorthands read.
    pub(crate) shorthands: Shorthands,
    /// A preset's listed paths (S17, ADD-A128), each as [`listed_path`]
    /// gives it: a block built whole may leave exactly these out.
    pub(crate) listed: Vec<String>,
    /// Reading an `Option` of a struct block, stated whole in every
    /// position: a field it leaves out is mistyped, not missing (S11).
    option_block: bool,
}

/// A path as a listed path compares (S11's `[pending]` key): its last
/// key's unit suffix left out, so `delay_ms` and `delay` are one field.
pub(crate) fn listed_path(path: &str) -> String {
    UNIT_SUFFIXES
        .iter()
        .find_map(|suffix| path.strip_suffix(suffix))
        .unwrap_or(path)
        .to_owned()
}

/// The unit suffixes S11 spells a key with.
const UNIT_SUFFIXES: [&str; 4] = ["_px", "_ms", "_deg", "_pt"];

/// The segments of a dotted TOML key, a quoted segment taken as written
/// inside its quotes; `None` for an unclosed quote or an empty segment.
pub(crate) fn key_segments(key: &str) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut chars = key.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => quoted = !quoted,
            '\\' if quoted => current.push(chars.next()?),
            '.' if !quoted => out.push(std::mem::take(&mut current)),
            other => current.push(other),
        }
    }
    out.push(current);
    (!quoted && !out.iter().any(String::is_empty)).then_some(out)
}

/// The input and the scheme a listed path names (S11's `[pending]` key,
/// S17, ADD-A132): `inputs.<name>.<scheme>`, an input listed by the scheme
/// at which its per-scheme value is pending, the last segment the scheme;
/// `None` for any other path, an input's name alone among them.
pub(crate) fn listed_input(path: &str) -> Option<(InputName, Scheme)> {
    let segments = key_segments(path)?;
    let (head, rest) = segments.split_first()?;
    let (scheme, name) = rest.split_last()?;
    if head != "inputs" || name.is_empty() {
        return None;
    }
    let scheme = <Scheme as Named>::from_name(scheme)?;
    Some((InputName::new(&name.join(".")).ok()?, scheme))
}

/// The typed part whose presence a listed path names (S11's `[pending]`
/// key, S17, ADD-A131): `widgets.<widget>.<part>`, a pending presence
/// listed by its part's path; `None` for any other path.
pub(crate) fn listed_part(path: &str) -> Option<TypedPart> {
    let segments = key_segments(path)?;
    let [head, widget, part] = segments.as_slice() else {
        return None;
    };
    if head != "widgets" {
        return None;
    }
    TypedPart::from_names(registry::widget(widget)?.kind, part)
}

/// A pending default size's declaration a listed path names (S11's
/// `[pending]` key, S17's *A pending fact with nothing beneath it*,
/// ADD-A131).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ListedDefault {
    /// The flavour whose section states it; `None`: the widget's own.
    pub(crate) flavor: Option<Key>,
    /// The widget.
    pub(crate) widget: WidgetKind,
    /// Which default of the section.
    pub(crate) at: DefaultAt,
}

/// Which default size of a widget's section a listed path names.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DefaultAt {
    /// `default_size`.
    Widget,
    /// The `sizes_by_variant` entry of this variant.
    Variant(VariantRef),
    /// The `sizes_by_context` entry at this position, from 0.
    Context(usize),
}

/// The default size a listed path names (S11, S17, ADD-A131):
/// `declare.widget.<widget>.default_size`, a `sizes_by_variant` entry's
/// `declare.widget.<widget>.sizes_by_variant.<variant>.default_size` (the
/// variant as S11 writes it, `@` included), a `sizes_by_context` entry's
/// `declare.widget.<widget>.sizes_by_context.<n>.default_size`, a
/// flavour's under `declare.flavor.<f>.`; `None` for any other path, a
/// component's among them, whose declarations the model does not read yet.
pub(crate) fn listed_default(path: &str) -> Option<ListedDefault> {
    let segments = key_segments(path)?;
    let (head, rest) = segments.split_first()?;
    if head != "declare" {
        return None;
    }
    let (flavor, rest) = match rest {
        [section, flavor, rest @ ..] if section == "flavor" => (Some(Key::new(flavor).ok()?), rest),
        _ => (None, rest),
    };
    let [section, widget, rest @ ..] = rest else {
        return None;
    };
    if section != "widget" {
        return None;
    }
    let widget = registry::widget(widget)?.kind;
    let at = match rest {
        [last] if last == "default_size" => DefaultAt::Widget,
        [list, name, last] if last == "default_size" && list == "sizes_by_variant" => {
            let mut cx = Reader::default();
            let variant = crate::toml::when::variant(name, path, &mut cx);
            DefaultAt::Variant(variant.filter(|_| !cx.failed())?)
        }
        [list, index, last] if last == "default_size" && list == "sizes_by_context" => {
            let digits = !index.is_empty() && index.bytes().all(|b| b.is_ascii_digit());
            DefaultAt::Context(index.parse().ok().filter(|_| digits)?)
        }
        _ => return None,
    };
    Some(ListedDefault { flavor, widget, at })
}

/// What a listed path names (S11's `[pending]` key, S17, ADD-A128): a
/// field of `colors.<scheme>`, of `tokens`, of a typed widget's part
/// (`widgets.<widget>.<part>.…`) or of its values
/// (`widgets.<widget>.values.…`), and how many levels of its map's keys
/// the path names below it ([`registry::find_entry`]), the model's path
/// the reader maps the key to. `None` where it names no such field, or its
/// last key carries a unit suffix its type is not spelled with (S11).
pub(crate) fn listed_field(path: &str) -> Option<(&'static FieldInfo, usize)> {
    let segments = key_segments(path)?;
    let last = segments.last()?;
    let suffixed = UNIT_SUFFIXES.iter().find_map(|suffix| {
        let name = last.strip_suffix(suffix)?;
        let mut stripped = segments.clone();
        if let Some(held) = stripped.last_mut() {
            *held = name.to_owned();
        }
        field_at(&stripped).filter(|(field, levels)| spelled_with(field, *levels, suffix))
    });
    suffixed
        .or_else(|| field_at(&segments).filter(|(field, levels)| spelled_with(field, *levels, "")))
}

/// The block holding the field a listed path names (`Glyph` for
/// `widgets.button.icon.glyph.size_px`, `TextStyle` for
/// `tokens.text.keyed.title.fill`); `None` where a part, a widget's values
/// or the root holds it.
pub(crate) fn listed_owner(path: &str) -> Option<&'static str> {
    let segments = key_segments(path)?;
    let (_, parent) = segments.split_last()?;
    match field_at(parent)?.0.ty {
        FieldType::Block { block, .. } => Some(block),
        FieldType::Leaf(_) | FieldType::Patch { .. } => None,
    }
}

/// The field `segments` name below `colors.<scheme>`, `tokens` or a typed
/// widget's part or values.
fn field_at(segments: &[String]) -> Option<(&'static FieldInfo, usize)> {
    let (head, rest) = segments.split_first()?;
    let (fields, below): (&'static [FieldInfo], &[String]) = match head.as_str() {
        "tokens" => (registry::tokens(), rest),
        "colors" => {
            let (scheme, rest) = rest.split_first()?;
            <Scheme as Named>::from_name(scheme)?;
            (registry::colors(), rest)
        }
        "widgets" => {
            let (widget, rest) = rest.split_first()?;
            let info = registry::widget(widget)?;
            let (part, rest) = rest.split_first()?;
            if part == "values" {
                (info.values, rest)
            } else {
                (info.part(part)?.fields, rest)
            }
        }
        _ => return None,
    };
    registry::find_entry(fields, &below.join("."))
}

/// Whether S11 spells the field `field`, `levels` levels of its map's keys
/// below it, with the unit suffix `suffix` (`""`: none): a duration with
/// `_ms`, a length with `_px`, an angle with `_deg`, a font size with
/// `_pt`, `_px` or none, a corner radius with `_px` or none, any other
/// value, and a map named whole, with none.
fn spelled_with(field: &FieldInfo, levels: usize, suffix: &str) -> bool {
    let ty = match field.ty {
        FieldType::Leaf(ty) | FieldType::Block { ty, .. } | FieldType::Patch { ty, .. } => ty,
    };
    if levels < registry::key_levels(ty) {
        return suffix.is_empty();
    }
    if matches!(
        field.ty,
        FieldType::Block {
            block: "Radius",
            ..
        }
    ) {
        return matches!(suffix, "_px" | "");
    }
    match (field.value)() {
        Some(Ty::Duration) => suffix == "_ms",
        Some(Ty::Length) => suffix == "_px",
        Some(Ty::Angle) => suffix == "_deg",
        Some(Ty::FontSize) => matches!(suffix, "_pt" | "_px" | ""),
        _ => suffix.is_empty(),
    }
}

impl Reader {
    /// Records a problem at `at`.
    pub(crate) fn problem(&mut self, at: &str, kind: ProblemKind) {
        self.problems.push(Problem::new(at, kind));
    }

    /// Records that `at` holds something else than `expected`; `None`, for
    /// the reading that failed.
    pub(crate) fn mistyped<T>(&mut self, at: &str, expected: &'static str) -> Option<T> {
        self.problem(at, ProblemKind::Mistyped { expected });
        None
    }

    /// Records an unknown name at `at`; `None`, for the reading that failed.
    pub(crate) fn unknown<T>(&mut self, at: &str, name: &str) -> Option<T> {
        self.problem(
            at,
            ProblemKind::UnknownName {
                name: name.to_owned(),
            },
        );
        None
    }

    /// Whether a problem has been recorded at the field `at` (or below it,
    /// or at its key's suffixed spelling): its value stated, but not read.
    fn reported_at(&self, at: &str) -> bool {
        self.problems.iter().any(|problem| {
            problem.path.strip_prefix(at).is_some_and(|rest| {
                rest.is_empty()
                    || rest.starts_with(['.', '['])
                    || ["_px", "_ms", "_deg", "_pt"].contains(&rest)
            })
        })
    }

    /// Whether a problem has been recorded.
    pub(crate) fn failed(&self) -> bool {
        !self.problems.is_empty()
    }

    /// Records that the key at `written` stands for the place `place`
    /// (S11's shorthands).
    pub(crate) fn shorthand(&mut self, place: String, written: &str) {
        self.shorthands.add(place, written);
    }

    /// The problems recorded, in reading order.
    pub(crate) fn into_problems(self) -> Vec<Problem> {
        self.problems
    }
}

/// What `value`, one TOML value, reads as a `T` keyed `spelling` at `at`
/// (S11): the type's own reader, no widget in between, for the tests of a
/// block no enabled part lists (a part's `transitions`, the motion unit's,
/// D6 outcome 3) or of a staged token; its problems as `path: kind` lines.
#[cfg(test)]
pub(crate) fn read_as<T: FromToml>(
    value: &str,
    spelling: Spelling,
    at: &str,
) -> Result<T, Vec<String>> {
    read_in(value, spelling, at, None)
}

/// What `value`, one TOML table, reads as a map of `V` keyed by `K` at
/// `at` (S11), as [`read_as`] reads a value.
#[cfg(test)]
pub(crate) fn read_map_as<K: MapKey + PartialEq, V: FromToml>(
    value: &str,
    at: &str,
) -> Result<Vec<(K, V)>, Vec<String>> {
    let table: Table =
        ::toml::from_str(&format!("value = {value}")).map_err(|error| vec![error.to_string()])?;
    let item = table
        .get("value")
        .ok_or_else(|| vec!["no value".to_owned()])?;
    let mut cx = Reader::default();
    let read = read_map::<K, V>(item, at, &mut cx);
    let problems: Vec<String> = cx.into_problems().iter().map(Problem::to_string).collect();
    match read {
        Some(read) if problems.is_empty() => Ok(read),
        Some(_) | None => Err(problems),
    }
}

/// [`read_as`] inside the table of the typed widget `widget`, where a bare
/// part name names one of its parts.
#[cfg(test)]
pub(crate) fn read_in<T: FromToml>(
    value: &str,
    spelling: Spelling,
    at: &str,
    widget: Option<WidgetKind>,
) -> Result<T, Vec<String>> {
    let table: Table =
        ::toml::from_str(&format!("value = {value}")).map_err(|error| vec![error.to_string()])?;
    let item = table
        .get("value")
        .ok_or_else(|| vec!["no value".to_owned()])?;
    let mut cx = Reader {
        widget,
        ..Reader::default()
    };
    let read = T::read(item, spelling, at, &mut cx);
    let problems: Vec<String> = cx.into_problems().iter().map(Problem::to_string).collect();
    match read {
        Some(read) if problems.is_empty() => Ok(read),
        Some(_) | None => Err(problems),
    }
}

/// `key` as a TOML key: bare where TOML allows it, else quoted.
pub(crate) fn toml_key(key: &str) -> String {
    let bare = !key.is_empty()
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    if bare {
        return key.to_owned();
    }
    // A TOML basic string: `"` and `\` escaped, the control characters by
    // their short escapes or `\uXXXX` (TOML 1.0, *String*).
    let mut out = String::with_capacity(key.len().saturating_add(2));
    out.push('"');
    for c in key.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\u{c}' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            c if c.is_control() && u32::from(c) <= 0x7f => {
                out.push_str(&format!("\\u{:04X}", u32::from(c)));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// `at.key`, or `key` at the root; a key that is no bare TOML key is quoted.
/// `read` in a keyframe's patch (S2.7, S11): a patch wherever the keyframe
/// stands, so neither the bases' refusal of `append` nor a preset's whole
/// blocks hold there, a table at a struct block a `Fields` patch and
/// `{ replace = { … } }` its `Set`.
pub(crate) fn in_patch<R>(cx: &mut Reader, read: impl FnOnce(&mut Reader) -> R) -> R {
    let held = (cx.whole, cx.bases);
    cx.whole = false;
    cx.bases = false;
    let out = read(cx);
    (cx.whole, cx.bases) = held;
    out
}

pub(crate) fn join(at: &str, key: &str) -> String {
    let key = toml_key(key);
    if at.is_empty() {
        key
    } else {
        format!("{at}.{key}")
    }
}

/// `at[index]`: an array's item.
pub(crate) fn item(at: &str, index: usize) -> String {
    format!("{at}[{index}]")
}

/// How a field's key is spelled: its name, or its name and its unit's
/// suffix (S1, S11).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Spelling {
    /// The field's name alone.
    Bare,
    /// `<field>_px`.
    Px,
    /// `<field>_ms`.
    Ms,
    /// `<field>_deg`.
    Deg,
    /// `<field>_pt`: a font size in points.
    Pt,
    /// No key: a list's item, a variant's payload or a tuple's member, read
    /// in its type's own unit.
    Item,
}

impl Spelling {
    fn suffix(self) -> &'static str {
        match self {
            Spelling::Bare | Spelling::Item => "",
            Spelling::Px => "_px",
            Spelling::Ms => "_ms",
            Spelling::Deg => "_deg",
            Spelling::Pt => "_pt",
        }
    }
}

/// The spelling `key` is of the field `field`, among `spellings`.
pub(crate) fn spelled(field: &str, key: &str, spellings: &[Spelling]) -> Option<Spelling> {
    let rest = key.strip_prefix(field)?;
    spellings
        .iter()
        .copied()
        .find(|spelling| rest == spelling.suffix())
}

/// A map entry's key and its spelling, among `spellings`: the key less the
/// suffix (`button_ms` of a duration map is `button`), a suffixed spelling
/// tried before the bare one.
pub(crate) fn entry_spelled<'k>(
    key: &'k str,
    spellings: &[Spelling],
) -> Option<(&'k str, Spelling)> {
    let suffixed = spellings
        .iter()
        .copied()
        .filter(|spelling| *spelling != Spelling::Bare)
        .find_map(|spelling| {
            key.strip_suffix(spelling.suffix())
                .filter(|name| !name.is_empty())
                .map(|name| (name, spelling))
        });
    suffixed.or_else(|| {
        spellings
            .contains(&Spelling::Bare)
            .then_some((key, Spelling::Bare))
    })
}

/// A value read from TOML (S11).
pub(crate) trait FromToml: Sized {
    /// The spellings a field of this type is keyed by.
    const SPELLINGS: &'static [Spelling] = &[Spelling::Bare];

    /// Whether a `Value` of this type may be spelled `"auto"`: a length's
    /// (S6.3; whether the field allows it is S12's check). Elsewhere `"auto"`
    /// is what the type spells by it.
    const AUTO: bool = false;

    /// Whether the type is a struct block, read whole as a table holding
    /// every field (the `ThemeParts` derive's).
    const STRUCT_BLOCK: bool = false;

    /// Whether, under `spelling`, a `Value` of this type is also stated as
    /// an expression, `"auto"` or `"absent"`: a font size's number takes a
    /// suffixed key, its other forms the bare one (S11).
    fn forms(spelling: Spelling) -> bool {
        let _ = spelling;
        true
    }

    /// The value `item` states under a key spelled `spelling`; each problem
    /// recorded at `at`, and `None` where the value cannot be read.
    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self>;

    /// Two keys for one field (`corners_px` beside `corners.curve`): `later`
    /// merged into `self` where they state different fields; `false` where
    /// they cannot be, a duplicate key.
    fn combine(&mut self, later: Self) -> bool {
        let _ = later;
        false
    }

    /// The value once the table holding it is read: in a preset's own
    /// bases, a table read as a `Fields` patch becomes the whole block
    /// (S2.2); a block that cannot be built stays as read, its problems
    /// recorded.
    fn finish(self, at: &str, cx: &mut Reader) -> Self {
        let _ = (at, cx);
        self
    }

    /// [`FromToml::read`] into a box: a large patch the derive writes is
    /// read in place on the heap ([`read_fields_boxed`]), so its reading
    /// holds no copy of it on the stack.
    fn read_boxed(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Box<Self>> {
        Self::read(item, spelling, at, cx).map(Box::new)
    }

    /// [`FromToml::finish`] of a boxed value, in its box where finishing
    /// leaves it as read.
    fn finish_boxed(self: Box<Self>, at: &str, cx: &mut Reader) -> Box<Self> {
        Box::new((*self).finish(at, cx))
    }
}

/// A struct block's patch, read key by key: the `ThemeParts` derive writes
/// it.
pub(crate) trait Fields: Default {
    /// Reads `key`'s `item` of the table at `at` into its field; `false`
    /// where the key spells none of the block's fields.
    fn read_entry(&mut self, key: &str, item: &Item, at: &str, cx: &mut Reader) -> bool;

    /// The field `key` spells, by its Rust name.
    fn field_of(key: &str) -> Option<&'static str>;

    /// Each field read finished, once the whole table is read.
    fn finish_fields(&mut self, at: &str, cx: &mut Reader);

    /// The other patch's fields merged in, where the two set none in common.
    fn combine_fields(&mut self, later: Self) -> bool;
}

/// A struct block built whole from its patch: the `ThemeParts` derive writes
/// it (S1's `T::try_from`).
pub(crate) trait Build: Sized {
    /// The block's patch.
    type Patch;

    /// The block `patch` states, each field it leaves unset recorded
    /// missing at its path below `at`; an `Option` field left unset is
    /// `None` where `options_none` (a list's item or a variant's payload,
    /// S11), else missing too.
    fn build(patch: Self::Patch, at: &str, options_none: bool, cx: &mut Reader) -> Option<Self>;
}

/// [`read_fields`] into a box, the patch read in place on the heap.
pub(crate) fn read_fields_boxed<P: Fields>(table: &Table, at: &str, cx: &mut Reader) -> Box<P> {
    let mut patch = Box::<P>::default();
    for (key, item) in table {
        if !patch.read_entry(key, item, at, cx) {
            cx.problem(&join(at, key), ProblemKind::UnknownKey);
        }
    }
    patch.finish_fields(at, cx);
    patch
}

/// `table`'s keys read into a patch of `P`, every unknown key recorded.
pub(crate) fn read_fields<P: Fields>(table: &Table, at: &str, cx: &mut Reader) -> P {
    let mut patch = P::default();
    for (key, item) in table {
        if !patch.read_entry(key, item, at, cx) {
            cx.problem(&join(at, key), ProblemKind::UnknownKey);
        }
    }
    patch.finish_fields(at, cx);
    patch
}

/// Puts a field's value read under `key` of the table at `table` into its
/// slot; a second key for the field is combined where it can be, else a
/// duplicate key, naming the first key as written.
pub(crate) fn put<P>(
    slot: &mut Option<P>,
    value: P,
    combine: fn(&mut P, P) -> bool,
    table: &str,
    key: &str,
    field: &str,
    cx: &mut Reader,
) {
    let field_at = join(table, field);
    match slot {
        Some(held) => {
            if !combine(held, value) {
                let other = cx
                    .keys
                    .iter()
                    .find(|(at, _)| *at == field_at)
                    .map_or_else(|| field.to_owned(), |(_, first)| first.clone());
                cx.problem(&join(table, key), ProblemKind::DuplicateKey { other });
            }
        }
        None => {
            cx.keys.push((field_at, key.to_owned()));
            *slot = Some(value);
        }
    }
}

/// The key a field left out of a block stated whole is reported at: its
/// name with the suffix of its type's first spelling (`top_px`, `size_pt`),
/// as a file spells it.
pub(crate) fn missing_key(name: &str, spellings: &[Spelling]) -> String {
    let suffix = spellings.first().map_or("", |spelling| spelling.suffix());
    format!("{name}{suffix}")
}

/// Never combines: a second key is a duplicate.
pub(crate) fn no_combine<P>(_held: &mut P, _later: P) -> bool {
    false
}

/// A field of a block built whole, or a key a table must state: its value,
/// else missing, unless its value was stated and its problem recorded.
pub(crate) fn required<T>(value: Option<T>, at: &str, cx: &mut Reader) -> Option<T> {
    if value.is_none() && cx.whole && cx.listed.contains(&listed_path(at)) {
        // A preset's listed path left out of its own bases: the block stays
        // a patch, the field unstated for S12 to report (S17, ADD-A128).
        return value;
    }
    if value.is_none() && !cx.reported_at(at) {
        if cx.option_block {
            cx.problem(
                at,
                ProblemKind::Mistyped {
                    expected: OPTION_BLOCK,
                },
            );
        } else {
            cx.problem(at, ProblemKind::Missing);
        }
    }
    value
}

/// What an `Option` of a struct block is stated as (S11).
const OPTION_BLOCK: &str = "a field of a block an `Option` holds: the block is \"none\" or a \
                            table holding every field";

/// An `Option` field of a block built whole: left out, `None` where
/// `options_none`, else missing.
pub(crate) fn optional<T>(
    value: Option<Option<T>>,
    options_none: bool,
    at: &str,
    cx: &mut Reader,
) -> Option<Option<T>> {
    match value {
        Some(value) => Some(value),
        None if options_none => Some(None),
        None => required(None, at, cx),
    }
}

/// `item` as a table.
pub(crate) fn table<'i>(item: &'i Item, at: &str, cx: &mut Reader) -> Option<&'i Table> {
    match item {
        Item::Table(table) => Some(table),
        _ => cx.mistyped(at, "a table"),
    }
}

/// `item` as a string.
pub(crate) fn text<'i>(item: &'i Item, at: &str, cx: &mut Reader) -> Option<&'i str> {
    match item {
        Item::String(text) => Some(text),
        _ => cx.mistyped(at, "a string"),
    }
}

/// `item` as an array.
pub(crate) fn array<'i>(item: &'i Item, at: &str, cx: &mut Reader) -> Option<&'i [Item]> {
    match item {
        Item::Array(items) => Some(items),
        _ => cx.mistyped(at, "an array"),
    }
}

/// `item` as an array of exactly `N` items (a tuple's members).
pub(crate) fn members<'i, const N: usize>(
    item: &'i Item,
    expected: &'static str,
    at: &str,
    cx: &mut Reader,
) -> Option<&'i [Item; N]> {
    match item {
        Item::Array(items) => match <&[Item; N]>::try_from(items.as_slice()) {
            Ok(members) => Some(members),
            Err(_) => cx.mistyped(at, expected),
        },
        _ => cx.mistyped(at, expected),
    }
}

/// An enum's spelling (S11): a unit variant's name, or a one-key table of a
/// variant with data and its payload.
pub(crate) fn variant<'i>(
    item: &'i Item,
    at: &str,
    cx: &mut Reader,
) -> Option<(&'i str, Option<&'i Item>)> {
    match item {
        Item::String(name) => Some((name, None)),
        Item::Table(table) if table.len() == 1 => table
            .iter()
            .next()
            .map(|(name, payload)| (name.as_str(), Some(payload))),
        _ => cx.mistyped(
            at,
            "a variant's name, or a one-key table of a variant and its data",
        ),
    }
}

/// A keyed name, `@<key>` (S2.2's names, S10).
pub(crate) fn keyed(name: &str, at: &str, cx: &mut Reader) -> Option<Key> {
    let key = name.strip_prefix('@')?;
    key_named(key, at, cx)
}

/// The key `name` spells.
pub(crate) fn key_named(name: &str, at: &str, cx: &mut Reader) -> Option<Key> {
    match Key::new(name) {
        Ok(key) => Some(key),
        Err(error) => {
            cx.problem(at, ProblemKind::NotAKey(error));
            None
        }
    }
}

/// A finite number, from a TOML integer or float.
pub(crate) fn number(item: &Item, at: &str, cx: &mut Reader) -> Option<f32> {
    let n = match item {
        Item::Integer(n) => *n as f32,
        Item::Float(n) => *n as f32,
        _ => return cx.mistyped(at, "a number"),
    };
    if n.is_finite() {
        Some(n)
    } else {
        cx.problem(at, ProblemKind::NotFinite);
        None
    }
}

/// An enum with unit variants only, by its variants' names: the `ThemeParts`
/// derive writes it (`#[parts(toml)]`).
pub(crate) trait Named: Sized + 'static {
    /// The variant's name, in snake_case (S11).
    fn name(&self) -> &'static str;

    /// The variant `name` names.
    fn from_name(name: &str) -> Option<Self>;
}

/// A unit-only enum's variant named by `item`.
pub(crate) fn named<T: Named>(item: &Item, at: &str, cx: &mut Reader) -> Option<T> {
    let name = text(item, at, cx)?;
    T::from_name(name).or_else(|| cx.unknown(at, name))
}

impl FromToml for bool {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        match item {
            Item::Boolean(b) => Some(*b),
            _ => cx.mistyped(at, "a boolean"),
        }
    }
}

impl FromToml for f32 {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        number(item, at, cx)
    }
}

impl FromToml for u16 {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        match item {
            Item::Integer(n) => u16::try_from(*n)
                .ok()
                .or_else(|| cx.mistyped(at, "an integer from 0 to 65535")),
            _ => cx.mistyped(at, "an integer from 0 to 65535"),
        }
    }
}

impl FromToml for u32 {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        match item {
            Item::Integer(n) => u32::try_from(*n)
                .ok()
                .or_else(|| cx.mistyped(at, "an integer from 0 to 4294967295")),
            _ => cx.mistyped(at, "an integer from 0 to 4294967295"),
        }
    }
}

impl FromToml for String {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        text(item, at, cx).map(str::to_owned)
    }
}

impl FromToml for Key {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        key_named(name, at, cx)
    }
}

impl FromToml for Color {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let literal = text(item, at, cx)?;
        Color::parse(literal).or_else(|| cx.mistyped(at, "a colour (S2.1)"))
    }
}

impl FromToml for Px {
    const SPELLINGS: &'static [Spelling] = &[Spelling::Px];
    const AUTO: bool = true;

    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        number(item, at, cx).map(Px)
    }
}

impl FromToml for Ms {
    const SPELLINGS: &'static [Spelling] = &[Spelling::Ms];

    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        number(item, at, cx).map(Ms)
    }
}

impl FromToml for Deg {
    const SPELLINGS: &'static [Spelling] = &[Spelling::Deg];

    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        number(item, at, cx).map(Deg)
    }
}

impl FromToml for Fraction {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        number(item, at, cx).map(Fraction)
    }
}

/// A keyed scalar's key takes no unit suffix: its number is in the unit its
/// declaration names (S10, S11), a `Number` here, which the declaration's
/// unit decides when the layer is added (S12).
impl FromToml for Scalar {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        number(item, at, cx).map(Scalar::Number)
    }
}

/// A font size's number is keyed by its unit, `_pt` or `_px`; its other
/// forms, an expression, `"auto"` and `"absent"`, by the bare name (S11).
impl FromToml for FontSize {
    const SPELLINGS: &'static [Spelling] = &[Spelling::Pt, Spelling::Px, Spelling::Bare];
    const AUTO: bool = true;

    fn forms(spelling: Spelling) -> bool {
        spelling == Spelling::Bare
    }

    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        match spelling {
            Spelling::Pt => number(item, at, cx).map(FontSize::Pt),
            Spelling::Px => number(item, at, cx).map(FontSize::Px),
            Spelling::Bare | Spelling::Ms | Spelling::Deg | Spelling::Item => cx.mistyped(
                at,
                "an expression, \"auto\" or \"absent\"; a number under the key's `_pt` or `_px` \
                 form",
            ),
        }
    }
}

impl FromToml for (f32, f32) {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let [x, y] = members::<2>(item, "an array of two numbers", at, cx)?;
        let x = number(x, &self::item(at, 0), cx);
        let y = number(y, &self::item(at, 1), cx);
        Some((x?, y?))
    }
}

/// A value (S2.2): `"= <expression>"`, `"auto"` (a length's), `"absent"`,
/// or the value stated.
impl<T: FromToml> FromToml for Value<T> {
    const SPELLINGS: &'static [Spelling] = T::SPELLINGS;
    const AUTO: bool = T::AUTO;

    fn forms(spelling: Spelling) -> bool {
        T::forms(spelling)
    }

    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        if T::forms(spelling)
            && let Some(read) = unstated(item, T::AUTO, at, cx)
        {
            return read;
        }
        T::read(item, spelling, at, cx).map(Value::Stated)
    }

    fn combine(&mut self, later: Self) -> bool {
        match (self, later) {
            (Value::Stated(held), Value::Stated(later)) => held.combine(later),
            _ => false,
        }
    }

    fn finish(self, at: &str, cx: &mut Reader) -> Self {
        match self {
            Value::Stated(value) => Value::Stated(value.finish(at, cx)),
            other => other,
        }
    }
}

/// A `Value` that `item` states other than by its value: `"= <expression>"`,
/// `"auto"` where `auto` (a length's) or `"absent"`; `None` for any other
/// item, the value stated, and `Some(None)` for an expression that does not
/// parse, its problem recorded.
fn unstated<T>(item: &Item, auto: bool, at: &str, cx: &mut Reader) -> Option<Option<Value<T>>> {
    let Item::String(text) = item else {
        return None;
    };
    if let Some(expression) = text.strip_prefix('=') {
        return Some(match Expr::parse(expression) {
            Ok(expr) => Some(Value::Expr(expr)),
            Err(error) => {
                cx.problem(at, ProblemKind::Expression(error));
                None
            }
        });
    }
    match text.as_str() {
        "auto" if auto => Some(Some(Value::Auto)),
        "absent" => Some(Some(Value::Absent)),
        _ => None,
    }
}

/// A `Value` of a map (`Font.axes`, S6.1, S11): `"= <reference>"`,
/// `"absent"`, or the map as a table, `{}` an empty one.
pub(crate) fn read_value_map<K: MapKey + PartialEq, V: FromToml>(
    item: &Item,
    at: &str,
    cx: &mut Reader,
) -> Option<Value<Vec<(K, V)>>> {
    match unstated(item, false, at, cx) {
        Some(read) => read,
        None => read_map::<K, V>(item, at, cx).map(Value::Stated),
    }
}

/// A boxed value, read as the value it holds.
impl<T: FromToml> FromToml for Box<T> {
    const SPELLINGS: &'static [Spelling] = T::SPELLINGS;
    const AUTO: bool = T::AUTO;
    const STRUCT_BLOCK: bool = T::STRUCT_BLOCK;

    fn forms(spelling: Spelling) -> bool {
        T::forms(spelling)
    }

    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        T::read_boxed(item, spelling, at, cx)
    }

    fn combine(&mut self, later: Self) -> bool {
        (**self).combine(*later)
    }

    fn finish(self, at: &str, cx: &mut Reader) -> Self {
        T::finish_boxed(self, at, cx)
    }
}

/// An `Option` with a meaning of its own for `None`: `"none"` (S11).
impl<T: FromToml> FromToml for Option<T> {
    const SPELLINGS: &'static [Spelling] = T::SPELLINGS;

    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        if let Item::String(text) = item
            && text == "none"
        {
            return Some(None);
        }
        if !T::STRUCT_BLOCK {
            return T::read(item, spelling, at, cx).map(Some);
        }
        let outer = std::mem::replace(&mut cx.option_block, true);
        let read = T::read(item, spelling, at, cx).map(Some);
        cx.option_block = outer;
        read
    }

    fn finish(self, at: &str, cx: &mut Reader) -> Self {
        self.map(|value| value.finish(at, cx))
    }
}

/// A list: a TOML array, every item read (S11).
impl<T: FromToml> FromToml for Vec<T> {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let items = array(item, at, cx)?;
        let read: Vec<Option<T>> = items
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let at = self::item(at, index);
                T::read(entry, Spelling::Item, &at, cx).map(|value| value.finish(&at, cx))
            })
            .collect();
        read.into_iter().collect()
    }
}

/// A map's key, from the TOML key it is stated under.
pub(crate) trait MapKey: Sized {
    /// The key `name` spells.
    fn map_key(name: &str, at: &str, cx: &mut Reader) -> Option<Self>;

    /// The key as TOML spells it.
    fn key_name(&self) -> String;
}

impl MapKey for Key {
    fn map_key(name: &str, at: &str, cx: &mut Reader) -> Option<Self> {
        key_named(name, at, cx)
    }

    fn key_name(&self) -> String {
        self.as_str().to_owned()
    }
}

impl MapKey for String {
    fn map_key(name: &str, _: &str, _: &mut Reader) -> Option<Self> {
        Some(name.to_owned())
    }

    fn key_name(&self) -> String {
        self.clone()
    }
}

impl<T: Named> MapKey for T {
    fn map_key(name: &str, at: &str, cx: &mut Reader) -> Option<Self> {
        T::from_name(name).or_else(|| cx.unknown(at, name))
    }

    fn key_name(&self) -> String {
        self.name().to_owned()
    }
}

/// A map, `Vec<(K, V)>`: a TOML table, one entry per key; a value with a unit
/// keyed by its key and the unit's suffix (`button_ms`, S1).
pub(crate) fn read_map<K: MapKey + PartialEq, V: FromToml>(
    item: &Item,
    at: &str,
    cx: &mut Reader,
) -> Option<Vec<(K, V)>> {
    let table = table(item, at, cx)?;
    let mut out: Vec<(K, V)> = Vec::new();
    let mut written: Vec<&String> = Vec::new();
    let mut whole = true;
    for (name, entry) in table {
        let at = join(at, name);
        let Some((key, spelling)) = entry_spelled(name, V::SPELLINGS) else {
            cx.problem(&at, ProblemKind::UnknownKey);
            whole = false;
            continue;
        };
        let key = K::map_key(key, &at, cx);
        let value = V::read(entry, spelling, &at, cx).map(|value| value.finish(&at, cx));
        match (key, value) {
            (Some(key), Some(value)) => {
                let first = out.iter().position(|(held, _)| *held == key);
                if let Some(first) = first {
                    let other = written.get(first).map_or(name, |first| *first);
                    cx.problem(
                        &at,
                        ProblemKind::DuplicateKey {
                            other: other.clone(),
                        },
                    );
                    whole = false;
                } else {
                    out.push((key, value));
                    written.push(name);
                }
            }
            _ => whole = false,
        }
    }
    whole.then_some(out)
}

/// A map whose values are maps.
type NestedMap<K, L, V> = Vec<(K, Vec<(L, V)>)>;

/// A map of maps, each level a TOML table (S6.1).
pub(crate) fn read_nested_map<K, L, V>(
    item: &Item,
    at: &str,
    cx: &mut Reader,
) -> Option<NestedMap<K, L, V>>
where
    K: MapKey + PartialEq,
    L: MapKey + PartialEq,
    V: FromToml,
{
    let table = table(item, at, cx)?;
    let mut out = Vec::new();
    let mut whole = true;
    for (name, entry) in table {
        let at = join(at, name);
        let key = K::map_key(name, &at, cx);
        let inner = read_map::<L, V>(entry, &at, cx);
        match (key, inner) {
            (Some(key), Some(inner)) => out.push((key, inner)),
            _ => whole = false,
        }
    }
    whole.then_some(out)
}

impl MapKey for SurfaceSet {
    fn map_key(name: &str, at: &str, cx: &mut Reader) -> Option<Self> {
        SurfaceSet::from_name(name).or_else(|| cx.unknown(at, name))
    }

    fn key_name(&self) -> String {
        self.name().to_owned()
    }
}

/// A map per surface set: a TOML table keyed by the sets' names (S3).
pub(crate) fn read_surfaces<V: FromToml>(
    item: &Item,
    at: &str,
    cx: &mut Reader,
) -> Option<SurfaceMap<V>> {
    let entries = read_map::<SurfaceSet, V>(item, at, cx)?;
    let mut map = SurfaceMap::default();
    for (set, value) in entries {
        map.insert(set, value);
    }
    Some(map)
}

/// A compound's members, each keyed as a field of its own with its unit's
/// suffix (`padding.top_px`, S1); a member left out is left as it is.
fn read_members<V: FromToml, const N: usize>(
    item: &Item,
    names: [&str; N],
    at: &str,
    cx: &mut Reader,
) -> Option<[Option<V>; N]> {
    let table = table(item, at, cx)?;
    let mut out: [Option<V>; N] = std::array::from_fn(|_| None);
    let mut written: [Option<&String>; N] = [None; N];
    for (key, entry) in table {
        let at = join(at, key);
        let found = names
            .iter()
            .zip(out.iter_mut().zip(written.iter_mut()))
            .find_map(|(name, slot)| {
                spelled(name, key, V::SPELLINGS).map(|spelling| (spelling, slot))
            });
        match found {
            Some((_, (_, Some(first)))) => cx.problem(
                &at,
                ProblemKind::DuplicateKey {
                    other: (*first).clone(),
                },
            ),
            Some((spelling, (slot, first))) => {
                *first = Some(key);
                *slot = V::read(entry, spelling, &at, cx);
            }
            None => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    Some(out)
}

/// A `Sides` patch: a table of the members it states (S1).
impl<V: FromToml> FromToml for Sides<Option<V>> {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let [top, right, bottom, left] =
            read_members::<V, 4>(item, ["top", "right", "bottom", "left"], at, cx)?;
        Some(Sides {
            top,
            right,
            bottom,
            left,
        })
    }
}

/// An `Axes` patch: a table of the members it states (S1).
impl<V: FromToml> FromToml for Axes<Option<V>> {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let [horizontal, vertical] =
            read_members::<V, 2>(item, ["horizontal", "vertical"], at, cx)?;
        Some(Axes {
            horizontal,
            vertical,
        })
    }
}

/// A `Value<B>` field's patch built whole: set, or its fields built.
pub(crate) fn build_value<B>(
    patch: BlockPatch<B>,
    at: &str,
    options_none: bool,
    cx: &mut Reader,
) -> Option<Value<B>>
where
    B: FieldsBlock + Build<Patch = <B as FieldsBlock>::Patch>,
{
    match patch {
        BlockPatch::Set(value) => Some(value),
        BlockPatch::Fields(fields) => B::build(fields, at, options_none, cx).map(Value::Stated),
    }
}

/// A map of bare blocks' entry built whole: a stated block, or its fields
/// built; a reference, `Absent` or `Auto` the entry cannot hold.
pub(crate) fn build_bare<B>(
    patch: BlockPatch<B>,
    at: &str,
    options_none: bool,
    cx: &mut Reader,
) -> Option<B>
where
    B: FieldsBlock + Build<Patch = <B as FieldsBlock>::Patch>,
{
    match patch {
        BlockPatch::Set(Value::Stated(block)) => Some(block),
        BlockPatch::Set(_) => cx.mistyped(at, "a table: the entry is a block, never a value"),
        BlockPatch::Fields(fields) => B::build(fields, at, options_none, cx),
    }
}

/// A paint's patch built whole: set, or its appended layers over nothing,
/// a paint of those layers (S6.1).
pub(crate) fn build_paint(patch: PaintPatch) -> Value<Paint> {
    match patch {
        PaintPatch::Set(value) => value,
        PaintPatch::Append(layers) => Value::Stated(Paint { layers }),
    }
}

/// A map's entries built whole, each at its key's path.
pub(crate) fn build_entries<K: MapKey, P, V>(
    entries: Vec<(K, P)>,
    at: &str,
    cx: &mut Reader,
    mut build: impl FnMut(P, &str, &mut Reader) -> Option<V>,
) -> Option<Vec<(K, V)>> {
    let built: Vec<Option<(K, V)>> = entries
        .into_iter()
        .map(|(key, patch)| {
            let at = join(at, &key.key_name());
            build(patch, &at, cx).map(|value| (key, value))
        })
        .collect();
    built.into_iter().collect()
}

/// A map per surface set built whole, set by set.
pub(crate) fn build_surfaces<P, V>(
    sets: SurfaceMap<P>,
    at: &str,
    cx: &mut Reader,
    build: impl FnMut(P, &str, &mut Reader) -> Option<V>,
) -> Option<SurfaceMap<V>> {
    let entries = build_entries(sets.into_entries().collect(), at, cx, build)?;
    let mut map = SurfaceMap::default();
    for (set, value) in entries {
        map.insert(set, value);
    }
    Some(map)
}
