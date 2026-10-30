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

use super::registry::FieldInfo;
use super::value::Value;

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "patches are merged by S8's resolution (M1a task 10)"
    )
)]
mod merge;
pub mod outline;
pub mod paint;
pub mod shadow;
pub mod transform;

pub(crate) use merge::{
    Apply, Merge, MergeConflict, merge_by_key, merge_slot, merge_slot_by_key, replace_slot,
};

/// A block: its resolved type and its field table, which the `ThemeParts`
/// derive generates.
pub trait Block {
    /// The block with each `Value` evaluated (S6.1).
    type Resolved;
    /// The block's fields, for the field-path registry.
    const FIELDS: &'static [FieldInfo];
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
mod tests;
