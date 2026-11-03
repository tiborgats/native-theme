//! The typed widgets (S5, S6.1): the units `scripts/widget_units.toml`
//! enables, which `scripts/generate_widgets.py` writes from the catalogue
//! into the `gen_*.rs` files (each widget's parts, values and own types,
//! and the index over them), and the part patches and slots written here by
//! hand.

use super::blocks::part_style::PartStylePatch;
use super::blocks::{Apply, Merge, MergeConflict, merge_slot};
use super::registry::{self, FieldInfo, PartInfo};
use super::selector::{VariantRef, WidgetOrPart};

mod gen_button;
mod gen_index;

pub use gen_index::*;

/// A part's patch (S6.1): what it says of the part's presence, and the
/// style's fields it sets. A patch stating no presence sets fields only and
/// never brings an absent or redrawn part back (S6.3).
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct PartPatch {
    /// The part's presence; `None`: this patch says nothing of it (S6.3).
    pub presence: Option<PartPresence>,
    /// The fields it sets, boxed: a widget's patch holds one per part and a
    /// layer one per widget, so inline they would make a layer's value grow
    /// with the catalogue.
    pub style: Box<PartStylePatch>,
}

/// A part's presence, as the last of its patches stating one decides it
/// (S6.3).
#[derive(Clone, Debug, PartialEq)]
pub enum PartPresence {
    /// The whole part is not drawn; the query answers `NotDrawn`.
    Absent,
    /// Drawn again, with its own style, over an `Absent` or `DrawnAs`
    /// below.
    Present,
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

#[cfg(test)]
impl PartPatch {
    /// A patch of the style's fields `style` alone.
    pub(crate) fn of_style(style: PartStylePatch) -> PartPatch {
        PartPatch {
            presence: None,
            style: Box::new(style),
        }
    }

    /// A patch of the presence `presence` alone.
    pub(crate) fn of_presence(presence: PartPresence) -> PartPatch {
        PartPatch {
            presence: Some(presence),
            style: Box::default(),
        }
    }
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

impl From<&PartPresence> for Shown {
    fn from(presence: &PartPresence) -> Shown {
        match presence {
            PartPresence::Absent => Shown::Absent,
            PartPresence::Present => Shown::Drawn,
            PartPresence::DrawnAs { target, variants } => Shown::DrawnAs {
                target: target.clone(),
                variants: variants.clone(),
            },
        }
    }
}

/// A part's patches applied in order (S6.1, S6.3): what the part is drawn
/// as, by the last patch stating a presence, and its style's fields stated
/// so far, which an `Absent` or a `DrawnAs` keeps beneath it, so a later
/// `Present` draws the part with them.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PartSlot {
    /// What the part is drawn as.
    pub(crate) shown: Shown,
    /// The style's fields stated so far; `None`: none yet.
    pub(crate) style: Option<PartStylePatch>,
}

impl Apply<PartPatch> for PartSlot {
    fn apply(&mut self, patch: PartPatch) -> Result<(), MergeConflict> {
        if let Some(presence) = &patch.presence {
            self.shown = Shown::from(presence);
        }
        if *patch.style == PartStylePatch::default() {
            return Ok(());
        }
        merge_slot(&mut self.style, Some(*patch.style))
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

impl TypedPart {
    /// The part's catalogue entry.
    pub(crate) fn info(self) -> Option<&'static PartInfo> {
        let widget = self.widget();
        registry::widgets()
            .iter()
            .find(|info| info.kind == widget)
            .and_then(|info| info.part(self.name()))
    }

    /// Whether the part starts an element of its own: the widget's root
    /// part, or an *(element)* part (S8).
    pub(crate) fn starts_element(self) -> bool {
        self.info().is_none_or(|info| info.root || info.element)
    }

    /// The part it nests in: its bullet's parent, else the root; `None` for
    /// the root.
    pub(crate) fn parent(self) -> Option<TypedPart> {
        let info = self.info()?;
        if info.root {
            return None;
        }
        let widget = self.widget();
        let parent = match info.parent {
            Some(parent) => parent,
            None => {
                registry::widgets()
                    .iter()
                    .find(|held| held.kind == widget)?
                    .parts
                    .iter()
                    .find(|part| part.root)?
                    .name
            }
        };
        TypedPart::from_names(widget, parent)
    }

    /// The part whose element it belongs to (S8): itself where it starts
    /// one, else the nearest enclosing part that does.
    pub(crate) fn element(self) -> TypedPart {
        self.enclosing()
            .into_iter()
            .find(|part| part.starts_element())
            .unwrap_or(self)
    }

    /// The part and the parts enclosing it, innermost first, the root last;
    /// each once, as the catalogue nests no part in itself.
    pub(crate) fn enclosing(self) -> Vec<TypedPart> {
        let mut out = vec![self];
        let mut current = self;
        while let Some(parent) = current.parent() {
            if out.contains(&parent) {
                break;
            }
            out.push(parent);
            current = parent;
        }
        out
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
