//! The field-path registry: each block's and widget's table of its fields, as
//! the `ThemeParts` derive writes it (S6.1).

use super::blocks::Table;
use super::expr::Ty;
use super::roles::ColorRoles;
use super::tokens::Tokens;
use super::widgets::{Variant, WidgetKind};

/// One field of a block or a part: its path, its type and its marks.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FieldInfo {
    /// The field's path below its block, in the model's Rust names (S1): a
    /// field's name; for an enum block's variant, the variant in snake_case,
    /// then the field's name (`full.maximum`), or the variant alone for a
    /// tuple variant (`px`).
    pub(crate) path: &'static str,
    /// The field's type.
    pub(crate) ty: FieldType,
    /// The field may be `Absent` (S6.3).
    pub(crate) may_be_absent: bool,
    /// The field may be `Auto` (S6.3), so it resolves to a
    /// [`Length`](super::Length).
    pub(crate) auto: bool,
    /// The type an expression's reference to the field, or to an entry of
    /// its map, has (S2.2); `None` where the place decides it (a key, a
    /// keyed scalar) or nothing can (a list).
    pub(crate) value: fn() -> Option<Ty>,
}

/// Two fields are equal by their paths, types and marks: a function's
/// address is no identity to compare, and the type it gives is the written
/// type's.
impl PartialEq for FieldInfo {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.ty == other.ty
            && self.may_be_absent == other.may_be_absent
            && self.auto == other.auto
    }
}

/// No type for a reference: the place decides it, or nothing can.
pub(crate) fn no_type() -> Option<Ty> {
    None
}

/// A field's type, as the registry walks it.
#[derive(Clone, Copy, Debug)]
pub(crate) enum FieldType {
    /// A value with no fields of its own in the registry: the type as written
    /// (`Value<Px>`, `Vec<(Key,Value<Scalar>)>`).
    Leaf(&'static str),
    /// A block, a list or map of blocks or a block's value, whose own fields
    /// lie below this field's path.
    Block {
        /// The type as written (`Value<Paint>`, `Vec<Stop>`, `Edge`).
        ty: &'static str,
        /// The block's own name (`Paint`, `Stop`, `Edge`; a compound's,
        /// `Sides`).
        block: &'static str,
        /// Whether the block is a struct or an enum.
        form: BlockForm,
        /// The block's own table.
        fields: &'static [FieldInfo],
    },
    /// A struct block's patch (a keyframe's `PartStylePatch`), whose fields
    /// lie below this field's path as the block's do. Its table is reached
    /// through a function, as the block can hold this field itself (a part
    /// style's icon animation's keyframes), which a constant cannot.
    Patch {
        /// The type as written (`PartStylePatch`).
        ty: &'static str,
        /// The block's name (`PartStyle`).
        block: &'static str,
        /// The block's table.
        fields: fn() -> &'static [FieldInfo],
    },
}

/// Two patches are equal by their written type, which names their block, so
/// their tables are one: a function's address is no identity to compare.
impl PartialEq for FieldType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldType::Leaf(a), FieldType::Leaf(b)) => a == b,
            (
                FieldType::Block {
                    ty,
                    block,
                    form,
                    fields,
                },
                FieldType::Block {
                    ty: other_ty,
                    block: other_block,
                    form: other_form,
                    fields: other_fields,
                },
            ) => {
                ty == other_ty
                    && block == other_block
                    && form == other_form
                    && fields == other_fields
            }
            (FieldType::Patch { ty, .. }, FieldType::Patch { ty: other, .. }) => ty == other,
            (FieldType::Leaf(_) | FieldType::Block { .. } | FieldType::Patch { .. }, _) => false,
        }
    }
}

/// Whether a block is a struct or an enum.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum BlockForm {
    /// A struct block: its fields are its table's paths.
    Struct,
    /// An enum block: a variant with data is a table's path prefix
    /// (`full.maximum`, `px`), a unit variant has no fields and is named here
    /// (`follow_part`), each in snake_case, as TOML spells it (S11).
    Enum {
        /// The unit variants, in declaration order.
        unit_variants: &'static [&'static str],
    },
}

/// One part of a typed widget, as its catalogue entry states it (S5, S6.3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PartInfo {
    /// The part's name, as TOML and paths spell it.
    pub(crate) name: &'static str,
    /// The widget's root part: never absent as a whole, never an element.
    pub(crate) root: bool,
    /// Queried as an element of its own (the catalogue's *(element)*).
    pub(crate) element: bool,
    /// The part it nests in, where its bullet names one; else the root.
    pub(crate) parent: Option<&'static str>,
    /// The whole part may be `Absent` (S6.3).
    pub(crate) may_be_absent: bool,
    /// The part's `PartStyle` fields, exactly the catalogue's list, with
    /// their marks there.
    pub(crate) fields: &'static [FieldInfo],
}

/// A typed widget's tables: its parts, its values, its typed variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WidgetInfo {
    /// The widget.
    pub(crate) kind: WidgetKind,
    /// The widget's name, as TOML and paths spell it.
    pub(crate) name: &'static str,
    /// The parts, in the catalogue's order, the root first.
    pub(crate) parts: &'static [PartInfo],
    /// The widget values' table: `<widget>.values.…`.
    pub(crate) values: &'static [FieldInfo],
    /// The catalogue's typed variants of the widget; a preset declares the
    /// ones it draws (S10).
    pub(crate) variants: &'static [Variant],
    /// The content variants the connector sets from the element's content
    /// (the catalogue's pairs, *Entry format*, ADD-A61, ADD-A123), each with
    /// the *(element)* part its pair names (`None`: the widget's own):
    /// typed, and keyed with their `@` (`@custom_child`).
    pub(crate) content_variants: &'static [(Option<&'static str>, &'static str)],
}

impl WidgetInfo {
    /// The part named `name`.
    pub(crate) fn part(&self, name: &str) -> Option<&'static PartInfo> {
        self.parts.iter().find(|part| part.name == name)
    }
}

/// A field's info before a place's mark; the placeholder [`absent_but`]
/// overwrites, slot by slot.
const UNSET: FieldInfo = FieldInfo {
    path: "",
    ty: FieldType::Leaf(""),
    may_be_absent: false,
    auto: false,
    value: no_type,
};

/// `fields` with every `Value` field not named in `required` marked may be
/// `Absent`: a mark holding in one place only (S6.3: `IconStyle`'s fields in
/// an icon-size context), which the `ThemeParts` derive writes for a field
/// marked `inner_may_be_absent`, `N` being `fields.len()`. A field's own
/// fields keep their marks.
pub(crate) const fn absent_but<const N: usize>(
    fields: &[FieldInfo],
    required: &[&str],
) -> [FieldInfo; N] {
    let mut out = [UNSET; N];
    let mut rest = fields;
    let mut slots: &mut [FieldInfo] = &mut out;
    while let (Some((field, tail)), Some((slot, slots_tail))) =
        (rest.split_first(), slots.split_first_mut())
    {
        *slot = *field;
        let written = match field.ty {
            FieldType::Leaf(ty) | FieldType::Block { ty, .. } | FieldType::Patch { ty, .. } => ty,
        };
        if starts_with(written, "Value<") && !names(required, field.path) {
            slot.may_be_absent = true;
        }
        rest = tail;
        slots = slots_tail;
    }
    out
}

/// Whether every name in `required` is a `Value` field of `fields`: the
/// derive asserts it at compile time beside each [`absent_but`], so a
/// misspelled `required` name does not compile.
pub(crate) const fn names_values(fields: &[FieldInfo], required: &[&str]) -> bool {
    let mut rest = required;
    while let Some((name, tail)) = rest.split_first() {
        let mut found = false;
        let mut held = fields;
        while let Some((field, others)) = held.split_first() {
            let written = match field.ty {
                FieldType::Leaf(ty) | FieldType::Block { ty, .. } | FieldType::Patch { ty, .. } => {
                    ty
                }
            };
            if field.path.len() == name.len()
                && starts_with(field.path, name)
                && starts_with(written, "Value<")
            {
                found = true;
            }
            held = others;
        }
        if !found {
            return false;
        }
        rest = tail;
    }
    true
}

/// The fields of `fields` named in `names`, in that order, each named in
/// `absent` marked may be `Absent` (a part's field list and its catalogue
/// marks, S6.3), `N` being `names.len()`; the derive asserts beside it, by
/// [`names_fields`], that every name is a field.
pub(crate) const fn select<const N: usize>(
    fields: &[FieldInfo],
    names: &[&str],
    absent: &[&str],
) -> [FieldInfo; N] {
    let mut out = [UNSET; N];
    let mut rest = names;
    let mut slots: &mut [FieldInfo] = &mut out;
    while let (Some((name, tail)), Some((slot, slots_tail))) =
        (rest.split_first(), slots.split_first_mut())
    {
        let mut held = fields;
        while let Some((field, others)) = held.split_first() {
            if same(field.path, name) {
                *slot = *field;
                if self::names(absent, name) {
                    slot.may_be_absent = true;
                }
            }
            held = others;
        }
        rest = tail;
        slots = slots_tail;
    }
    out
}

/// Whether every name in `names` is a field of `fields`.
pub(crate) const fn names_fields(fields: &[FieldInfo], names: &[&str]) -> bool {
    let mut rest = names;
    while let Some((name, tail)) = rest.split_first() {
        let mut found = false;
        let mut held = fields;
        while let Some((field, others)) = held.split_first() {
            if same(field.path, name) {
                found = true;
            }
            held = others;
        }
        if !found {
            return false;
        }
        rest = tail;
    }
    true
}

/// Whether `a` and `b` are the same text, at compile time.
const fn same(a: &str, b: &str) -> bool {
    a.len() == b.len() && starts_with(a, b)
}

/// Whether `text` starts with `prefix`, byte by byte so it runs at compile
/// time.
const fn starts_with(text: &str, prefix: &str) -> bool {
    let mut text = text.as_bytes();
    let mut prefix = prefix.as_bytes();
    while let Some((p, prefix_tail)) = prefix.split_first() {
        let Some((t, text_tail)) = text.split_first() else {
            return false;
        };
        if *t != *p {
            return false;
        }
        text = text_tail;
        prefix = prefix_tail;
    }
    true
}

/// Whether `list` holds `name`.
const fn names(list: &[&str], name: &str) -> bool {
    let mut rest = list;
    while let Some((held, tail)) = rest.split_first() {
        if held.len() == name.len() && starts_with(held, name) {
            return true;
        }
        rest = tail;
    }
    false
}

/// The typed widgets' tables (S5, S6.1): `widgets.<widget>.<part>.…` and
/// `widgets.<widget>.values.…` in TOML.
pub(crate) fn widgets() -> &'static [WidgetInfo] {
    super::widgets::WIDGETS
}

/// The typed widget named `name`.
pub(crate) fn widget(name: &str) -> Option<&'static WidgetInfo> {
    widgets().iter().find(|widget| widget.name == name)
}

/// The colour roles' table (S3): `colors.<scheme>.…` in TOML.
pub(crate) fn colors() -> &'static [FieldInfo] {
    <ColorRoles as Table>::FIELDS
}

/// The tokens' table (S4): `tokens.…` in TOML, `token.…` in expressions.
pub(crate) fn tokens() -> &'static [FieldInfo] {
    <Tokens as Table>::FIELDS
}

/// How many key segments a map written as `ty` takes before its values: one
/// per `Vec` of pairs or `SurfaceMap` it nests (`Vec<(Key,Vec<(Key,…)>)>`
/// two), none for any other type.
fn key_levels(ty: &str) -> usize {
    let mut levels = 0_usize;
    let mut rest = ty;
    loop {
        if let Some(inner) = rest.strip_prefix("SurfaceMap<") {
            levels = levels.saturating_add(1);
            rest = inner;
        } else if let Some(inner) = rest.strip_prefix("Vec<(") {
            levels = levels.saturating_add(1);
            rest = inner.split_once(',').map_or("", |(_, value)| value);
        } else {
            return levels;
        }
    }
}

/// The field a path names below `fields`, in the model's Rust names, a map's
/// key a segment of its own (`surfaces.window.background`,
/// `params.opacity.disabled`); a unit variant of an enum block names the
/// field holding it.
pub(crate) fn find(fields: &'static [FieldInfo], path: &str) -> Option<&'static FieldInfo> {
    find_in(fields, path, false)
}

/// The field whose value a path names below `fields`, as [`find`] does,
/// but `None` for a map named whole, its keys left out: a reference names
/// one entry's value (S2.2), never the map.
pub(crate) fn find_value(fields: &'static [FieldInfo], path: &str) -> Option<&'static FieldInfo> {
    find_in(fields, path, true)
}

/// [`find`]; `value`: a map named whole names nothing.
fn find_in(fields: &'static [FieldInfo], path: &str, value: bool) -> Option<&'static FieldInfo> {
    fields.iter().find_map(|field| {
        let rest = if path == field.path {
            ""
        } else {
            path.strip_prefix(field.path)?.strip_prefix('.')?
        };
        let (ty, below) = match field.ty {
            FieldType::Leaf(ty) => (ty, None),
            FieldType::Block {
                ty, form, fields, ..
            } => (ty, Some((form, fields))),
            FieldType::Patch { ty, fields, .. } => (ty, Some((BlockForm::Struct, fields()))),
        };
        let levels = key_levels(ty);
        if rest.is_empty() {
            return (!value || levels == 0).then_some(field);
        }
        let mut rest = rest;
        for _ in 0..levels {
            if rest.is_empty() {
                return None;
            }
            rest = rest.split_once('.').map_or("", |(_, after)| after);
        }
        if rest.is_empty() {
            return Some(field);
        }
        match below {
            Some((BlockForm::Enum { unit_variants }, _)) if unit_variants.contains(&rest) => {
                Some(field)
            }
            Some((_, fields)) => find_in(fields, rest, value),
            None => None,
        }
    })
}

/// Every leaf path below `fields`, each prefixed by `prefix`, a map's key
/// left out; an enum block with unit variants is a leaf itself too, as a
/// unit variant is stated at its path (`corners.top_left = "follow_part"`,
/// S11), and so is a block's patch, whose block can hold it.
#[cfg(test)]
pub(crate) fn leaves(prefix: &str, fields: &'static [FieldInfo], out: &mut Vec<String>) {
    for field in fields {
        let path = join(prefix, field.path);
        match field.ty {
            FieldType::Block { form, fields, .. } => {
                if matches!(form, BlockForm::Enum { unit_variants } if !unit_variants.is_empty())
                    || fields.is_empty()
                {
                    out.push(path.clone());
                }
                leaves(&path, fields, out);
            }
            FieldType::Leaf(_) | FieldType::Patch { .. } => out.push(path),
        }
    }
}

/// `prefix.path`, or `path` alone below the root.
#[cfg(test)]
pub(crate) fn join(prefix: &str, path: &str) -> String {
    if prefix.is_empty() {
        path.to_owned()
    } else {
        format!("{prefix}.{path}")
    }
}
