//! The field-path registry: each block's and widget's table of its fields, as
//! the `ThemeParts` derive writes it (S6.1).

use super::blocks::Table;
use super::roles::ColorRoles;
use super::tokens::Tokens;

/// One field of a block or a part: its path, its type and its marks.
#[derive(Clone, Copy, Debug, PartialEq)]
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
                FieldType::Block { ty, form, fields },
                FieldType::Block {
                    ty: other_ty,
                    form: other_form,
                    fields: other_fields,
                },
            ) => ty == other_ty && form == other_form && fields == other_fields,
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

/// A field's info before a place's mark; the placeholder [`absent_but`]
/// overwrites, slot by slot.
const UNSET: FieldInfo = FieldInfo {
    path: "",
    ty: FieldType::Leaf(""),
    may_be_absent: false,
    auto: false,
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
    fields.iter().find_map(|field| {
        let rest = if path == field.path {
            ""
        } else {
            path.strip_prefix(field.path)?.strip_prefix('.')?
        };
        let (ty, below) = match field.ty {
            FieldType::Leaf(ty) => (ty, None),
            FieldType::Block { ty, form, fields } => (ty, Some((form, fields))),
            FieldType::Patch { ty, fields } => (ty, Some((BlockForm::Struct, fields()))),
        };
        if rest.is_empty() {
            return Some(field);
        }
        let mut rest = rest;
        for _ in 0..key_levels(ty) {
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
            Some((_, fields)) => find(fields, rest),
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
