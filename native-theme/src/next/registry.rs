//! The field-path registry: each block's and widget's table of its fields, as
//! the `ThemeParts` derive writes it (S6.1).

/// One field of a block or a part: its path, its type and its marks.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct FieldInfo {
    /// The field's path below its block, in the model's Rust names (S1): a
    /// field's name; for an enum block's variant, the variant in snake_case,
    /// then the field's name (`full.maximum`), or the variant alone for a
    /// tuple variant's one field (`px`).
    pub path: &'static str,
    /// The field's type.
    pub ty: FieldType,
    /// The field may be `Absent` (S6.3).
    pub may_be_absent: bool,
    /// The field may be `Auto` (S6.3), so it resolves to a
    /// [`Length`](super::Length).
    pub auto: bool,
}

/// A field's type, as the registry walks it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldType {
    /// A value with no fields of its own in the registry: the type as written
    /// (`Value<Px>`, `Vec<(Key,Value<Scalar>)>`).
    Leaf(&'static str),
    /// A block, a list of blocks or a block's value, whose own fields lie
    /// below this field's path.
    Block {
        /// The type as written (`Value<Paint>`, `Vec<Stop>`, `Edge`).
        ty: &'static str,
        /// The block's own table.
        fields: &'static [FieldInfo],
    },
}
