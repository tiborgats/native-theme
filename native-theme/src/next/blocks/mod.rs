//! Blocks (S2): the records a part's fields are made of, their patches and
//! their resolved forms, which the `ThemeParts` derive generates (S6.1).
//!
//! Merging follows S6.1: a field a later patch sets replaces the earlier
//! value; a map (every `Vec` of pairs) merges by key, the later entry
//! replacing the earlier; any other list is replaced whole; a struct block's
//! `Fields` patch applies its fields over the block below, and a paint's
//! `Append` adds layers over the paint below, over an `Absent` one giving a
//! paint of the appended layers.

use std::fmt::Debug;

use super::registry::{BlockForm, FieldInfo};
use super::value::Value;

pub mod focus;
pub mod icon;
pub mod interaction;
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "patches are merged by S8's resolution (M1a task 10); an `allow`, as Rust 1.88 (the MSRV) counts what dead code under an `expect` uses as used, and stable does not"
    )
)]
mod merge;
pub mod motion;
pub mod outline;
pub mod paint;
pub mod part_style;
pub mod shadow;
pub mod text;
pub mod transform;

pub(crate) use merge::{
    Apply, Below, Create, Entry, KeyedItem, Merge, MergeConflict, apply_entries, apply_surfaces,
    created_value, merge_by_key, merge_by_key_nested, merge_keyed_by, merge_slot,
    merge_slot_by_key, merge_slot_by_key_nested, merge_slot_entries, merge_slot_keyed,
    merge_slot_surfaces, replace_slot,
};

/// Seals [`Block`]: only the `ThemeParts` derive implements it.
pub(crate) mod sealed {
    /// The supertrait no type outside the crate can implement.
    pub trait Sealed {}
}

/// A block: its resolved type, which the `ThemeParts` derive generates.
pub trait Block: sealed::Sealed {
    /// The block with each `Value` evaluated (S6.1).
    type Resolved;
}

/// A block's field table, which the `ThemeParts` derive generates, for the
/// field-path registry.
pub(crate) trait Table {
    /// The block's fields.
    const FIELDS: &'static [FieldInfo];
    /// Whether the block is a struct or an enum, and an enum's unit variants.
    const FORM: BlockForm;
}

/// A struct block, which a patch sets whole or field by field (S2.2).
pub trait FieldsBlock: Block + Clone + Debug + PartialEq {
    /// The block's patch: each field `None` where the patch leaves it as it
    /// is.
    type Patch: Clone + Debug + Default + PartialEq;
}

/// The patch of a field whose value is a struct block (S2.2).
#[derive(Clone, Debug, PartialEq)]
pub enum BlockPatch<T: FieldsBlock> {
    /// Sets the field: a whole block, a reference or `Absent`.
    Set(Value<T>),
    /// Applies these fields over the block below; refused over an `Absent`
    /// or `Auto` value (S12), and composed lazily over a reference (S2.2).
    Fields(T::Patch),
}

#[cfg(test)]
mod part_tests;
#[cfg(test)]
mod tests;
