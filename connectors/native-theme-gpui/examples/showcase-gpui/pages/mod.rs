//! One module per page of the content area.

use gpui::{Div, Rems, rems};
use gpui_component::v_flex;
use native_theme::theme::LayoutTheme;
use native_theme_gpui::geometry;

use crate::support::{with_gap, with_padding};

pub(crate) mod basic;
mod buttons;
mod charts;
mod data;
mod feedback;
mod icons;
mod inputs;
mod layout;
mod overlays;
mod theme_map;
mod typography;

/// The space between two cells of the Icons page's two icon grids: the
/// showcase's own, gpui's `gap_2`, as the theme states no icon grid, and
/// the cells are a gallery's, not a row of widgets (spec §3.1). The cells'
/// infos say so.
pub(crate) const ICON_GRID_GAP: Rems = rems(0.5);

/// The space between the Feedback page's Badges: the showcase's own,
/// gpui's `gap_8`, as the theme states no distance for it. Upstream draws a
/// count 3px per digit past its Button's right edge (gpui-component 0.7.3
/// `badge.rs:137-146`), so badges `layout.widget_gap` apart would let the
/// count touch or cover the next one (spec §3.1). The Badges' infos say so.
pub(crate) const BADGE_ROW_GAP: Rems = rems(2.);

/// The room the Layout page leaves on either side of the Carousel: the
/// showcase's own, gpui's `px_16`, as the theme states no distance for it.
/// Upstream places the Previous and Next Buttons outside the frame, a
/// `mr_4`/`ml_4` margin past its edge (gpui-component 0.7.3
/// `carousel/carousel.rs:780-785`), so the page keeps a Button's width free
/// there (spec §3.1). The Carousel's info says so.
pub(crate) const CAROUSEL_SIDE_ROOM: Rems = rems(4.);

/// A page's root, as the Basic page lays out its own: padded by
/// `layout.window_margin`, its sections `layout.section_gap` apart, and
/// neither where the theme states none.
pub(crate) fn page_root(layout: &LayoutTheme) -> Div {
    with_gap(
        with_padding(v_flex(), geometry::window_margin(layout)),
        geometry::section_gap(layout),
    )
}
