//! [`Parts`]: where a widget of this crate painted its parts.

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
/// | [`SegmentedControl`](crate::segmented_control::SegmentedControl) | `segment_<n>` from 0, `divider_<n>` between segment `n` and the next |
/// | [`Expander`](crate::expander::Expander) | `header`, `arrow`, `title`, `body` (when open), `frame` (when `frame_enabled` draws one) |
/// | [`ComboBox`](crate::combo_box::ComboBox) | `text`, `arrow` |
///
/// A widget drawn with egui's own counterpart, where no atlas is installed or a size is not
/// finite, records none.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Parts(Vec<(String, egui::Rect)>);

impl Parts {
    /// The parts the widget answering with `response` painted in the pass that drew it; `None`
    /// for a response no widget of this crate recorded parts for, and for one not drawn in
    /// the last two passes that drew a widget of this crate.
    pub fn of(response: &egui::Response) -> Option<Self> {
        let ctx = &response.ctx;
        let key = store_key(ctx);
        ctx.data(|d| {
            // Borrowed, not cloned as `get_temp` clones (`egui/src/util/id_type_map.rs:448-450`).
            let store = d
                .get_temp_raw(RawKey::new::<PartsStore>(key))?
                .downcast_ref::<PartsStore>()?;
            store
                .current
                .parts
                .get(&response.id)
                .or_else(|| store.previous.parts.get(&response.id))
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
        self.0.iter().map(|(n, rect)| (n.as_str(), *rect))
    }

    /// Add the part `name` at `rect`.
    pub(crate) fn push(&mut self, name: impl Into<String>, rect: egui::Rect) {
        self.0.push((name.into(), rect));
    }

    /// Keep these as the parts of the widget answering with `id` in this pass, for
    /// [`Parts::of`].
    pub(crate) fn store(self, ctx: &egui::Context, id: egui::Id) {
        // Outside the accessor: each takes the `Context` lock itself.
        let (key, pass) = (store_key(ctx), ctx.cumulative_pass_nr());
        ctx.data_mut(|d| {
            let store = d.get_temp_mut_or_default::<PartsStore>(key);
            if store.current.pass != pass {
                std::mem::swap(&mut store.current, &mut store.previous);
                store.current.pass = pass;
                store.current.parts.clear();
            }
            store.current.parts.insert(id, self);
        });
    }
}

/// The parts of the widgets of this crate drawn in a viewport, in one entry of the `Context`'s
/// data, which egui never clears of temporary values (`egui/src/util/id_type_map.rs:375`): the
/// generation of the latest pass a widget recorded parts in, and of the one before it, which
/// the next pass's first record drops.
#[derive(Clone, Default)]
struct PartsStore {
    current: Generation,
    previous: Generation,
}

/// The parts the widgets drawn in the pass numbered `pass` recorded, by widget id.
#[derive(Clone, Default)]
struct Generation {
    pass: u64,
    parts: egui::IdMap<Parts>,
}

/// Where the parts of the widgets drawn in `ctx`'s viewport are kept: one store per viewport,
/// as egui counts passes per viewport (`Context::cumulative_pass_nr`).
fn store_key(ctx: &egui::Context) -> egui::Id {
    egui::Id::new(("native-theme-egui-widgets::parts", ctx.viewport_id()))
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

/// How many widgets' parts the store of `ctx`'s viewport keeps.
#[cfg(test)]
pub(crate) fn stored(ctx: &egui::Context) -> usize {
    let key = store_key(ctx);
    ctx.data_mut(|d| {
        let store = d.get_temp_mut_or_default::<PartsStore>(key);
        store.current.parts.len() + store.previous.parts.len()
    })
}
