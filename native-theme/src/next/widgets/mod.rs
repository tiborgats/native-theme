//! The typed widgets (S5, S6.1): the units `scripts/widget_units.toml`
//! enables, which `scripts/generate_widgets.py` writes from the catalogue
//! into the `gen_*.rs` files (each widget's parts, values and own types,
//! and the index over them), and the part patches and slots written here by
//! hand.

use super::blocks::part_style::PartStylePatch;
use super::blocks::{Apply, Merge, MergeConflict, merge_slot};
use super::registry::{FieldInfo, PartInfo};
use super::selector::{VariantRef, WidgetOrPart};

mod gen_button;
mod gen_index;

pub use gen_index::*;

/// A part's patch (S6.1): its style's fields, or that the whole part is not
/// drawn, or that another widget, or a part of one, draws it.
#[derive(Clone, Debug, PartialEq)]
#[expect(
    clippy::large_enum_variant,
    reason = "S6.1's variants as stated; a part patch is read when a layer is added and resolved, not drawn per frame"
)]
pub enum PartPatch {
    /// The part's style's fields.
    Style(PartStylePatch),
    /// The whole part is not drawn (S6.3); the query answers `NotDrawn`.
    Absent,
    /// Another widget, or a part of one, draws the part, in the host's
    /// context; the query answers `DrawnAs` and the connector builds that
    /// widget.
    DrawnAs {
        /// The widget or part that draws it.
        target: WidgetOrPart,
        /// Its variants.
        variants: Vec<VariantRef>,
    },
}

/// What a part is drawn as, by the last of its patches that says so.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum Shown {
    /// Drawn with its style.
    #[default]
    Drawn,
    /// Not drawn.
    Absent,
    /// Drawn by another widget or part.
    DrawnAs {
        /// The widget or part that draws it.
        target: WidgetOrPart,
        /// Its variants.
        variants: Vec<VariantRef>,
    },
}

/// A part's patches applied in order (S6.1, S6.3): what the part is drawn
/// as, and its style's fields stated so far, which an `Absent` or a
/// `DrawnAs` leaves in place, so a later `Style` brings the part back with
/// the fields stated below them.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PartSlot {
    /// What the part is drawn as.
    pub(crate) shown: Shown,
    /// The style's fields stated so far; `None`: none yet.
    pub(crate) style: Option<PartStylePatch>,
}

impl Apply<PartPatch> for PartSlot {
    fn apply(&mut self, patch: PartPatch) -> Result<(), MergeConflict> {
        match patch {
            PartPatch::Style(style) => {
                self.shown = Shown::Drawn;
                merge_slot(&mut self.style, Some(style))
            }
            PartPatch::Absent => {
                self.shown = Shown::Absent;
                Ok(())
            }
            PartPatch::DrawnAs { target, variants } => {
                self.shown = Shown::DrawnAs { target, variants };
                Ok(())
            }
        }
    }
}

/// One typed widget's patches applied in order: each part's slot, in the
/// order its first patch came, and its values' patches merged.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WidgetSlot<P, V> {
    /// The parts' slots.
    pub(crate) parts: Vec<(P, PartSlot)>,
    /// The values' patches merged; `None`: none yet.
    pub(crate) values: Option<V>,
}

impl<P, V> Default for WidgetSlot<P, V> {
    fn default() -> Self {
        Self {
            parts: Vec::new(),
            values: None,
        }
    }
}

impl<P: PartialEq, V: Merge> WidgetSlot<P, V> {
    /// Applies `patch` over `part`'s slot.
    pub(crate) fn apply_part(&mut self, part: P, patch: PartPatch) -> Result<(), MergeConflict> {
        if let Some((_, slot)) = self.parts.iter_mut().find(|(held, _)| *held == part) {
            return slot.apply(patch);
        }
        let mut slot = PartSlot::default();
        slot.apply(patch)?;
        self.parts.push((part, slot));
        Ok(())
    }

    /// Merges `patch` over the values' patches.
    pub(crate) fn apply_values(&mut self, patch: V) -> Result<(), MergeConflict> {
        merge_slot(&mut self.values, Some(patch))
    }

    /// Applies a widget's patch, split into its parts' patches and its
    /// values' patch.
    pub(crate) fn apply_widget(
        &mut self,
        (parts, values): (Vec<(P, PartPatch)>, Option<V>),
    ) -> Result<(), MergeConflict> {
        for (part, patch) in parts {
            self.apply_part(part, patch)?;
        }
        merge_slot(&mut self.values, values)
    }
}

/// A typed widget's tables, which the `ThemeParts` derive writes from the
/// widget's generated input (S6.1).
pub(crate) trait WidgetTable {
    /// The parts, in the catalogue's order, the root first.
    const PARTS: &'static [PartInfo];
    /// The widget values' table.
    const VALUES: &'static [FieldInfo];
}

#[cfg(test)]
mod tests;
