//! The field-path registry: each block's and widget's table of its fields, as
//! the `ThemeParts` derive writes it (S6.1).

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
#[derive(Clone, Copy, Debug, PartialEq)]
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
