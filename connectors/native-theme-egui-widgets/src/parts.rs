//! [`Parts`]: where a widget of this crate painted its parts.

use native_theme_egui::egui;

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
/// | [`Expander`](crate::expander::Expander) | `header`, `arrow`, `title`, `body` (when open) |
/// | [`ComboBox`](crate::combo_box::ComboBox) | `text`, `arrow` |
///
/// A widget drawn with egui's own counterpart, where no atlas is installed or a size is not
/// finite, records none.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Parts(Vec<(String, egui::Rect)>);

impl Parts {
    /// The parts the widget answering with `response` painted in the pass that drew it; `None`
    /// for a response no widget of this crate recorded parts for.
    pub fn of(response: &egui::Response) -> Option<Self> {
        response.ctx.data(|d| d.get_temp::<Self>(key(response.id)))
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

    /// Keep these as the parts of the widget answering with `id`, for [`Parts::of`].
    pub(crate) fn store(self, ctx: &egui::Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_temp(key(id), self));
    }
}

/// Where a widget's parts are kept in its `Context`'s data.
fn key(id: egui::Id) -> egui::Id {
    id.with("native-theme-egui-widgets::parts")
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
