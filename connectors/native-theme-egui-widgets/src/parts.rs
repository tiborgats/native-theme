//! [`Parts`]: where a widget of this crate painted its parts.

use std::borrow::Cow;

use native_theme_egui::egui;
use native_theme_egui::egui::util::id_type_map::RawKey;

/// The rectangles a widget of this crate painted its named parts in, in the coordinates of the
/// `Ui` it was added to, as the pass that drew it last painted them: what a test or a layout
/// inspector reads to find a switch's thumb or a slider's fill without re-deriving the widget's
/// geometry.
///
/// | widget | parts |
/// |---|---|
/// | [`Switch`](crate::switch::Switch) | `track`, `thumb`, `label` (when labelled) |
/// | [`Slider`](crate::slider::Slider) | `track`, `fill`, `thumb` |
/// | [`RadioButton`](crate::radio_button::RadioButton) | `indicator`, `dot` (when checked), `label` |
/// | [`ProgressBar`](crate::progress_bar::ProgressBar) | `fill` |
/// | [`SegmentedControl`](crate::segmented_control::SegmentedControl) | `segment_<n>` from 0, `divider_<n>` between segment `n` and the next in its row |
/// | [`Expander`](crate::expander::Expander) | `header`, `arrow`, `title`, `body` (when open), `frame` (when `frame_enabled` draws one) |
/// | [`ComboBox`](crate::combo_box::ComboBox) | `text`, `arrow` |
///
/// A widget drawn with egui's own counterpart, where no atlas is installed or a size is not
/// finite, records none.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Parts(Vec<(Cow<'static, str>, egui::Rect)>);

impl Parts {
    /// The parts the widget answering with `response` painted in the pass that drew it, in
    /// whichever viewport drew it; `None` for a response no widget of this crate recorded parts
    /// for, and for one not drawn in the last two passes of its viewport that drew a widget of
    /// this crate.
    pub fn of(response: &egui::Response) -> Option<Self> {
        Self::of_id(&response.ctx, response.id)
    }

    /// [`Parts::of`] the widget answering with `id`, read before it answers: what a widget
    /// reads of the last pass that drew it.
    pub(crate) fn of_id(ctx: &egui::Context, id: egui::Id) -> Option<Self> {
        let viewport = ctx.viewport_id();
        ctx.data(|d| {
            // Borrowed, not cloned as `get_temp` clones (`egui/src/util/id_type_map.rs:448-450`).
            let store = d
                .get_temp_raw(RawKey::new::<PartsStore>(store_key()))?
                .downcast_ref::<PartsStore>()?;
            // The viewport being drawn first; the others' ids are their own, as each viewport's
            // root `Ui` is salted with its id (`egui/src/context.rs:802`).
            let others = store.0.iter().filter(|(v, _)| **v != viewport);
            store
                .0
                .get(&viewport)
                .into_iter()
                .chain(others.map(|(_, generations)| generations))
                .find_map(|generations| generations.get(id))
                .cloned()
        })
    }

    /// The part named `name`, where the widget painted one.
    pub fn get(&self, name: &str) -> Option<egui::Rect> {
        self.0
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, rect)| *rect)
    }

    /// Every part, in the order the widget painted them.
    pub fn iter(&self) -> impl Iterator<Item = (&str, egui::Rect)> + '_ {
        self.0.iter().map(|(n, rect)| (n.as_ref(), *rect))
    }

    /// Add the part `name` at `rect`.
    pub(crate) fn push(&mut self, name: impl Into<Cow<'static, str>>, rect: egui::Rect) {
        self.0.push((name.into(), rect));
    }

    /// Keep these as the parts of the widget answering with `id` in this pass, for
    /// [`Parts::of`].
    pub(crate) fn store(self, ctx: &egui::Context, id: egui::Id) {
        // Outside the accessor: each takes the `Context` lock itself.
        let (viewport, pass) = (ctx.viewport_id(), ctx.cumulative_pass_nr());
        let others = ctx.data_mut(|d| {
            let store = d.get_temp_mut_or_default::<PartsStore>(store_key());
            let generations = store.0.entry(viewport).or_default();
            let first = generations.current.pass != pass;
            if first {
                std::mem::swap(&mut generations.current, &mut generations.previous);
                generations.current.pass = pass;
                generations.current.parts.clear();
            }
            generations.current.parts.insert(id, self);
            // On the pass's first record, the other viewports whose parts are kept.
            first.then(|| {
                store
                    .0
                    .keys()
                    .copied()
                    .filter(|&v| v != viewport && v != egui::ViewportId::ROOT)
                    .collect::<Vec<_>>()
            })
        });
        if let Some(others) = others.filter(|others| !others.is_empty()) {
            forget_closed_viewports(ctx, others);
        }
    }
}

/// Drop the parts of the viewports of `others` that egui no longer has, as egui drops its own
/// state of a closed viewport (`egui/src/context.rs:2756-2790`, `:2829-2835`).
///
/// egui counts a viewport it has no longer as having completed no pass
/// (`Context::cumulative_pass_nr_for`, `egui/src/context.rs:1788-1793`). A viewport that kept
/// parts has begun a pass, so one still open has completed it unless that pass is running now,
/// which only the viewport being drawn and the viewports it is drawn inside can be: its parent
/// is kept too. (A viewport two immediate levels up, in its very first pass, would lose that
/// pass's parts; its next pass records them again.)
fn forget_closed_viewports(ctx: &egui::Context, others: Vec<egui::ViewportId>) {
    let parent = ctx.parent_viewport_id();
    let closed: Vec<egui::ViewportId> = others
        .into_iter()
        .filter(|&v| v != parent && ctx.cumulative_pass_nr_for(v) == 0)
        .collect();
    if closed.is_empty() {
        return;
    }
    ctx.data_mut(|d| {
        let store = d.get_temp_mut_or_default::<PartsStore>(store_key());
        for v in &closed {
            store.0.remove(v);
        }
    });
}

/// The parts of the widgets of this crate drawn in each viewport, in one entry of the
/// `Context`'s data, which egui never clears of temporary values
/// (`egui/src/util/id_type_map.rs:375`); per viewport, as egui counts passes per viewport
/// (`Context::cumulative_pass_nr`).
#[derive(Clone, Default)]
struct PartsStore(egui::ViewportIdMap<Generations>);

/// A viewport's parts: the generation of its latest pass a widget recorded parts in, and of
/// the one before it, which the next pass's first record drops.
#[derive(Clone, Default)]
struct Generations {
    current: Generation,
    previous: Generation,
}

impl Generations {
    /// The parts of the widget answering with `id`, from the later generation first.
    fn get(&self, id: egui::Id) -> Option<&Parts> {
        self.current
            .parts
            .get(&id)
            .or_else(|| self.previous.parts.get(&id))
    }
}

/// The parts the widgets drawn in the pass numbered `pass` recorded, by widget id.
#[derive(Clone, Default)]
struct Generation {
    pass: u64,
    parts: egui::IdMap<Parts>,
}

/// Where the parts are kept in the `Context`'s data.
fn store_key() -> egui::Id {
    egui::Id::new("native-theme-egui-widgets::parts")
}

/// The rectangle of a text atom laid out after an atom whose cell is `before`, `gap` apart, as
/// `egui::AtomLayout` places it: at the start of the cell after the gap, centred on the row
/// (`egui/src/atomics/atom_layout.rs:629-640`; an atom's `align` is `CENTER_CENTER`,
/// `egui/src/atomics/atom.rs:63`).
pub(crate) fn text_after(before: egui::Rect, gap: f32, size: egui::Vec2) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(before.right() + gap, before.center().y - 0.5 * size.y),
        size,
    )
}

/// How many widgets' parts the store keeps for `ctx`'s viewport.
#[cfg(test)]
pub(crate) fn stored(ctx: &egui::Context) -> usize {
    let viewport = ctx.viewport_id();
    ctx.data_mut(|d| {
        let store = d.get_temp_mut_or_default::<PartsStore>(store_key());
        store.0.get(&viewport).map_or(0, |generations| {
            generations.current.parts.len() + generations.previous.parts.len()
        })
    })
}

/// The viewports the store keeps parts for.
#[cfg(test)]
pub(crate) fn stored_viewports(ctx: &egui::Context) -> Vec<egui::ViewportId> {
    ctx.data_mut(|d| {
        let store = d.get_temp_mut_or_default::<PartsStore>(store_key());
        store.0.keys().copied().collect()
    })
}
