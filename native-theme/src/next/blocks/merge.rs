//! Merging patches (S6.1): a later patch over an earlier one, and a patch
//! over a value.

use super::{BlockPatch, FieldsBlock};
use crate::next::value::Value;

/// What stops a merge: a `Fields` patch or an `Append` over a value it
/// cannot be applied to now.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MergeConflict {
    /// The field's path below the merged patch or block, in Rust names.
    pub(crate) path: String,
    /// What lies below the patch there.
    pub(crate) below: Below,
}

/// The value a `Fields` patch or an `Append` met.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Below {
    /// A reference: the patch composes lazily, applied where the reference
    /// is evaluated (S2.2), which the resolver does.
    Reference,
    /// `Absent`: refused (S12).
    Absent,
    /// `Auto`: refused (S12).
    Auto,
}

impl MergeConflict {
    fn new(below: Below) -> Self {
        Self {
            path: String::new(),
            below,
        }
    }

    /// The conflict seen from the block holding `field`.
    pub(crate) fn under(mut self, field: &str) -> Self {
        self.path = if self.path.is_empty() {
            field.to_owned()
        } else {
            format!("{field}.{}", self.path)
        };
        self
    }

    /// The conflict a `Fields` patch or an `Append` meets over `value`;
    /// `None` over a stated value.
    pub(crate) fn over<T>(value: &Value<T>) -> Option<Self> {
        match value {
            Value::Stated(_) => None,
            Value::Expr(_) => Some(Self::new(Below::Reference)),
            Value::Absent => Some(Self::new(Below::Absent)),
            Value::Auto => Some(Self::new(Below::Auto)),
        }
    }
}

/// A later patch merged over an earlier one. Where applying the two in order
/// over a value succeeds, applying the merged patch gives the same value. The
/// merge does not see that value, so it can succeed where the ordered
/// application conflicts: a later `Set` drops an earlier `Fields` or `Append`
/// unchecked, which over an `Absent` or `Auto` value would have conflicted;
/// such a conflict is the earlier patch's, refused where the layer is
/// validated (S12). On a conflict `self` is left partly merged and is to be
/// discarded.
pub(crate) trait Merge: Sized {
    fn merge(&mut self, later: Self) -> Result<(), MergeConflict>;
}

/// A patch applied over a value. On a conflict `self` is left partly patched
/// and is to be discarded.
pub(crate) trait Apply<P> {
    fn apply(&mut self, patch: P) -> Result<(), MergeConflict>;
}

impl<T> Merge for BlockPatch<T>
where
    T: FieldsBlock + Apply<T::Patch>,
    T::Patch: Merge,
{
    fn merge(&mut self, later: Self) -> Result<(), MergeConflict> {
        match (self, later) {
            (this, BlockPatch::Set(value)) => {
                *this = BlockPatch::Set(value);
                Ok(())
            }
            (BlockPatch::Fields(earlier), BlockPatch::Fields(later)) => earlier.merge(later),
            (BlockPatch::Set(value), BlockPatch::Fields(later)) => {
                value.apply(BlockPatch::Fields(later))
            }
        }
    }
}

impl<T> Apply<BlockPatch<T>> for Value<T>
where
    T: FieldsBlock + Apply<T::Patch>,
{
    fn apply(&mut self, patch: BlockPatch<T>) -> Result<(), MergeConflict> {
        match patch {
            BlockPatch::Set(value) => {
                *self = value;
                Ok(())
            }
            BlockPatch::Fields(fields) => match self {
                Value::Stated(block) => block.apply(fields),
                other => MergeConflict::over(other).map_or(Ok(()), Err),
            },
        }
    }
}

/// A later setting replaces the earlier.
pub(crate) fn replace_slot<T>(earlier: &mut Option<T>, later: Option<T>) {
    if later.is_some() {
        *earlier = later;
    }
}

/// A later map merges into the earlier by key.
pub(crate) fn merge_slot_by_key<K: PartialEq, V>(
    earlier: &mut Option<Vec<(K, V)>>,
    later: Option<Vec<(K, V)>>,
) {
    let Some(later) = later else { return };
    if let Some(held) = earlier.as_mut() {
        merge_by_key(held, later);
    } else {
        *earlier = Some(later);
    }
}

/// A later patch merges over the earlier.
pub(crate) fn merge_slot<T: Merge>(
    earlier: &mut Option<T>,
    later: Option<T>,
) -> Result<(), MergeConflict> {
    let Some(later) = later else { return Ok(()) };
    if let Some(held) = earlier.as_mut() {
        held.merge(later)
    } else {
        *earlier = Some(later);
        Ok(())
    }
}

/// Each later entry replaces the earlier entry of its key, or is added after
/// the earlier entries.
pub(crate) fn merge_by_key<K: PartialEq, V>(earlier: &mut Vec<(K, V)>, later: Vec<(K, V)>) {
    for (key, value) in later {
        match earlier.iter_mut().find(|(held, _)| *held == key) {
            Some(entry) => entry.1 = value,
            None => earlier.push((key, value)),
        }
    }
}
