//! Merging patches (S6.1): a later patch over an earlier one, and a patch
//! over a value.

use super::{BlockPatch, FieldsBlock};
use crate::next::value::{SurfaceMap, Value};

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
    /// Nothing: a map entry's fields patched over a map lacking the entry,
    /// which no block can be built for from the patch alone, as a field the
    /// patch leaves unset may not be `Absent` (S12 refuses it; no value is
    /// invented for the field).
    Missing,
    /// A bare block: a map of bare blocks' entry set to a reference,
    /// `Absent` or `Auto`, which the entry cannot hold. That is no merge
    /// conflict but an ill-typed patch, refused whatever lies below it when
    /// the layer is added (S6.1, S12), so no validated layer brings one here.
    Block,
}

impl MergeConflict {
    pub(crate) fn new(below: Below) -> Self {
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

/// A map entry's value and its patch (S6.1): the patch applied over the
/// entry the map holds, or building the entry the map lacks.
pub(crate) trait Entry<P>: Sized {
    /// Applies `patch` over the held entry.
    fn apply_entry(&mut self, patch: P) -> Result<(), MergeConflict>;
    /// The entry `patch` builds where the map lacks it.
    fn create(patch: P) -> Result<Self, MergeConflict>;
}

/// A struct block built from its patch alone, for a map entry the value
/// lacks (S6.1): only where every field the patch leaves unset may be
/// `Absent` (S6.3's marks, their conditions S12's), those fields then
/// `Absent`; a field left unset that may not be (a `Value` without the mark,
/// a plain value, an `Option`, a list) is a [`Below::Missing`] conflict, the
/// merge conflict S12 refuses, as no value is invented for it.
pub(crate) trait Create: FieldsBlock {
    /// The block `patch` states in a place whose marks `inner` gives: `None`,
    /// the block's own; `Some(required)`, every `Value` field but those
    /// named may also be `Absent` there (an `inner_may_be_absent` place).
    /// The first field it leaves unset that may not be `Absent` is the
    /// error.
    fn create_in(patch: Self::Patch, inner: Option<&[&str]>) -> Result<Self, MergeConflict>;

    /// The block `patch` states, under the block's own marks.
    fn create(patch: Self::Patch) -> Result<Self, MergeConflict> {
        Self::create_in(patch, None)
    }
}

/// A created block's `Value` field: as the patch states it, else `Absent`
/// where the field may be (`marked`, or the place's `inner` marks not
/// requiring it), else a [`Below::Missing`] conflict at `field`.
pub(crate) fn created_value<T>(
    value: Option<Value<T>>,
    marked: bool,
    inner: Option<&[&str]>,
    field: &str,
) -> Result<Value<T>, MergeConflict> {
    match value {
        Some(value) => Ok(value),
        None if marked || inner.is_some_and(|required| !required.contains(&field)) => {
            Ok(Value::Absent)
        }
        None => Err(MergeConflict::new(Below::Missing).under(field)),
    }
}

/// An entry of a map of bare struct blocks: `Set` replaces it with a stated
/// block, `Fields` applies over it, and over a missing entry builds it.
impl<B> Entry<BlockPatch<B>> for B
where
    B: Create + Apply<B::Patch>,
{
    fn apply_entry(&mut self, patch: BlockPatch<B>) -> Result<(), MergeConflict> {
        match patch {
            BlockPatch::Set(Value::Stated(block)) => {
                *self = block;
                Ok(())
            }
            BlockPatch::Set(_) => Err(MergeConflict::new(Below::Block)),
            BlockPatch::Fields(fields) => self.apply(fields),
        }
    }

    fn create(patch: BlockPatch<B>) -> Result<Self, MergeConflict> {
        match patch {
            BlockPatch::Set(Value::Stated(block)) => Ok(block),
            BlockPatch::Set(_) => Err(MergeConflict::new(Below::Block)),
            BlockPatch::Fields(fields) => B::create(fields),
        }
    }
}

/// An entry of a map of a struct block's values: `Set` replaces it, `Fields`
/// applies over it, and over a missing entry builds a stated block.
impl<B> Entry<BlockPatch<B>> for Value<B>
where
    B: Create + Apply<B::Patch>,
{
    fn apply_entry(&mut self, patch: BlockPatch<B>) -> Result<(), MergeConflict> {
        self.apply(patch)
    }

    fn create(patch: BlockPatch<B>) -> Result<Self, MergeConflict> {
        match patch {
            BlockPatch::Set(value) => Ok(value),
            BlockPatch::Fields(fields) => B::create(fields).map(Value::Stated),
        }
    }
}

/// A later map of entry patches merges into the earlier by key, an entry
/// held on both sides merged as its patch merges.
pub(crate) fn merge_slot_entries<K: PartialEq, P: Merge>(
    earlier: &mut Option<Vec<(K, P)>>,
    later: Option<Vec<(K, P)>>,
) -> Result<(), MergeConflict> {
    let Some(later) = later else { return Ok(()) };
    let Some(held) = earlier.as_mut() else {
        *earlier = Some(later);
        return Ok(());
    };
    for (key, patch) in later {
        match held.iter_mut().find(|(k, _)| *k == key) {
            Some((_, below)) => below.merge(patch)?,
            None => held.push((key, patch)),
        }
    }
    Ok(())
}

/// Entry patches applied over a map by key, an entry the map lacks built
/// from its patch and added after the held entries.
pub(crate) fn apply_entries<K: PartialEq, V: Entry<P>, P>(
    value: &mut Vec<(K, V)>,
    patch: Vec<(K, P)>,
) -> Result<(), MergeConflict> {
    for (key, entry) in patch {
        match value.iter_mut().find(|(k, _)| *k == key) {
            Some((_, held)) => held.apply_entry(entry)?,
            None => value.push((key, V::create(entry)?)),
        }
    }
    Ok(())
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

/// A map whose values are maps.
type MapOfMaps<K, L, V> = Vec<(K, Vec<(L, V)>)>;

/// A later map of maps merges into the earlier by key, an entry held on both
/// sides merged by key in turn.
pub(crate) fn merge_slot_by_key_nested<K: PartialEq, L: PartialEq, V>(
    earlier: &mut Option<MapOfMaps<K, L, V>>,
    later: Option<MapOfMaps<K, L, V>>,
) {
    let Some(later) = later else { return };
    if let Some(held) = earlier.as_mut() {
        merge_by_key_nested(held, later);
    } else {
        *earlier = Some(later);
    }
}

/// Each later entry is merged by key into the earlier entry of its key, or
/// is added after the earlier entries.
pub(crate) fn merge_by_key_nested<K: PartialEq, L: PartialEq, V>(
    earlier: &mut MapOfMaps<K, L, V>,
    later: MapOfMaps<K, L, V>,
) {
    for (key, entries) in later {
        match earlier.iter_mut().find(|(held, _)| *held == key) {
            Some((_, held)) => merge_by_key(held, entries),
            None => earlier.push((key, entries)),
        }
    }
}

/// A later map of patches per surface set merges into the earlier, set by
/// set.
pub(crate) fn merge_slot_surfaces<P: Merge>(
    earlier: &mut Option<SurfaceMap<P>>,
    later: Option<SurfaceMap<P>>,
) -> Result<(), MergeConflict> {
    let Some(later) = later else { return Ok(()) };
    let Some(held) = earlier.as_mut() else {
        *earlier = Some(later);
        return Ok(());
    };
    for (set, patch) in later.into_entries() {
        match held.get_mut(set) {
            Some(below) => below.merge(patch).map_err(|c| c.under(set.name()))?,
            None => {
                held.insert(set, patch);
            }
        }
    }
    Ok(())
}

/// Patches per surface set applied over the sets' records, a record the
/// map lacks built from its patch.
pub(crate) fn apply_surfaces<B: Entry<P>, P>(
    value: &mut SurfaceMap<B>,
    patch: SurfaceMap<P>,
) -> Result<(), MergeConflict> {
    for (set, entry) in patch.into_entries() {
        match value.get_mut(set) {
            Some(record) => record.apply_entry(entry).map_err(|c| c.under(set.name()))?,
            None => {
                let record = B::create(entry).map_err(|c| c.under(set.name()))?;
                value.insert(set, record);
            }
        }
    }
    Ok(())
}

/// Each later entry replaces the earlier entry of its key, or is added after
/// the earlier entries.
pub(crate) fn merge_by_key<K: PartialEq, V>(earlier: &mut Vec<(K, V)>, later: Vec<(K, V)>) {
    merge_by(earlier, later, |a, b| a.0 == b.0);
}

/// Each later item replaces the earlier item `same` pairs it with, or is
/// added after the earlier items.
pub(crate) fn merge_by<T>(earlier: &mut Vec<T>, later: Vec<T>, same: impl Fn(&T, &T) -> bool) {
    for item in later {
        match earlier.iter_mut().find(|held| same(held, &item)) {
            Some(held) => *held = item,
            None => earlier.push(item),
        }
    }
}

/// A keyed list's merge (S6.1): each later item replaces the earlier item
/// `same` pairs it with, and the merged list holds the earlier items not
/// replaced, in their order, then the later items, in theirs, so a
/// replacing item takes the later list's position.
pub(crate) fn merge_keyed_by<T>(
    earlier: &mut Vec<T>,
    later: Vec<T>,
    same: impl Fn(&T, &T) -> bool,
) {
    for item in later {
        earlier.retain(|held| !same(held, &item));
        earlier.push(item);
    }
}

/// A list item that merges by a key of its own (S6.1): a binding by its role
/// or surface, a transition by its property group, a presentation by its
/// event.
pub(crate) trait KeyedItem: Sized {
    /// Merges `later` into `earlier` by the items' keys.
    fn merge_keyed(earlier: &mut Vec<Self>, later: Vec<Self>);
}

/// A later keyed list merges into the earlier by its items' keys.
/// A first one is merged into an empty list, so its items are keyed (a
/// transition listing several groups split) as a later one's are.
pub(crate) fn merge_slot_keyed<T: KeyedItem>(earlier: &mut Option<Vec<T>>, later: Option<Vec<T>>) {
    let Some(later) = later else { return };
    T::merge_keyed(earlier.get_or_insert_with(Vec::new), later);
}
