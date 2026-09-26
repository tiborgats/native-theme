//! The install plugin (§10.3) and the focus-ring bookkeeping (§6.18). This half records, per
//! pass and per viewport, the `Ui`s a role scope styled and the radius each took; the plugin
//! reads the record at the end of the pass.

use egui::{Context, CornerRadius, Id, Rect};

/// The one temporary `Context` data entry per viewport holding this pass's scope records and
/// registered focus shapes (§6.18; §4.3). One entry, not one per `Ui`: egui's garbage
/// collection gathers only the elements it can serialize and skips a temporary value
/// (`egui/src/util/id_type_map.rs:712-722`); its `gc` scope walks that gathered map (`:728-729`).
#[derive(Clone, Default)]
pub(crate) struct FocusEntry {
    /// `Context::cumulative_pass_nr` of the pass the lists belong to.
    pub(crate) pass_nr: u64,
    /// Each styled `Ui`'s `unique_id` with its role style's `widgets.active.corner_radius`.
    pub(crate) scopes: Vec<(Id, CornerRadius)>,
    /// `register_focus_shape`'s outlines: widget id, rect, radius.
    pub(crate) shapes: Vec<(Id, Rect, CornerRadius)>,
}

impl FocusEntry {
    /// Empties both lists when the entry belongs to an earlier pass (§6.18).
    pub(crate) fn begin(&mut self, pass_nr: u64) {
        if self.pass_nr != pass_nr {
            self.scopes.clear();
            self.shapes.clear();
            self.pass_nr = pass_nr;
        }
    }
}

/// The entry's key for the current viewport (§4.3): per viewport, because an immediate
/// viewport runs a pass nested inside its parent's and `cumulative_pass_nr` is counted per
/// viewport (`egui/src/context.rs:1781-1793`).
pub(crate) fn focus_key(ctx: &Context) -> Id {
    Id::new(("native-theme-egui/focus", ctx.viewport_id()))
}

/// Record that `ui_id` was styled with a role whose active corner radius is `radius`, for this
/// pass. A second record for the same `Ui` replaces the first (§6.18). The pass number and the
/// key are read before `data_mut` is entered: no `Context` accessor inside another (§10.3).
pub(crate) fn record_scope(ctx: &Context, ui_id: Id, radius: CornerRadius) {
    let pass_nr = ctx.cumulative_pass_nr();
    let key = focus_key(ctx);
    ctx.data_mut(|data| {
        let entry = data.get_temp_mut_or_insert_with(key, FocusEntry::default);
        entry.begin(pass_nr);
        match entry.scopes.iter_mut().find(|(id, _)| *id == ui_id) {
            Some(slot) => slot.1 = radius,
            None => entry.scopes.push((ui_id, radius)),
        }
    });
}
