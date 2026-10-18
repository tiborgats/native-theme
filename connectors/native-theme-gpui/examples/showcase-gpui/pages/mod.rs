//! One module per page of the content area.

use gpui::{Div, Pixels, Rems, px, rems};
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
pub(crate) mod layout;
mod overlays;
mod theme_map;
mod typography;

/// The space between two cells of the Icons page's two icon grids: the
/// showcase's own, gpui's `gap_2`, as the theme states no icon grid, and
/// the cells are a gallery's, not a row of widgets (spec §3.1). The cells'
/// infos say so.
pub(crate) const ICON_GRID_GAP: Rems = rems(0.5);

/// How far upstream draws a Badge's count past its Button's right edge, a
/// character at a time: a Medium count, `Badge::new`'s size, is placed
/// `-3px * len` from the right (gpui-component 0.7.3 `badge.rs:137-146`;
/// `Size::Medium` is the default, `sizing.rs:9-10`).
pub(crate) const BADGE_COUNT_OVERHANG: Pixels = px(3.);

/// The highest count a Badge prints as it is: above it, `99+`
/// (gpui-component 0.7.3 `badge.rs:46`, `:130-134`).
const BADGE_MAX: usize = 99;

/// How far the count a Badge shows for `count` stands past its Button's
/// right edge: [`BADGE_COUNT_OVERHANG`] per character upstream prints, and
/// nothing for a dot, which sits inside the Button (`badge.rs:129`), or for
/// a zero count, which upstream does not draw (`badge.rs:103`). The Feedback
/// page adds it to `layout.widget_gap` after each Badge, so a count stands
/// that gap from the next one, as the controls of every other row do.
pub(crate) fn badge_count_overhang(count: Option<usize>) -> Pixels {
    let printed = match count {
        Some(0) | None => 0,
        Some(c) if c > BADGE_MAX => format!("{BADGE_MAX}+").len(),
        Some(c) => c.to_string().len(),
    };
    BADGE_COUNT_OVERHANG * printed as f32
}

/// The room the Layout page leaves on either side of the Carousel: the
/// showcase's own, gpui's `px_16`, as the theme states no distance for it.
/// Upstream places the Previous and Next Buttons outside the frame, a
/// `mr_4`/`ml_4` margin past its edge (gpui-component 0.7.3
/// `carousel/carousel.rs:780-785`), so the page keeps a Button's width free
/// there (spec §3.1). The Carousel's info says so.
pub(crate) const CAROUSEL_SIDE_ROOM: Rems = rems(4.);

/// The space between two swatches of a Theme Map row, and between two
/// rows: the showcase's own, as the theme states no swatch table (spec
/// §3.1). Each swatch's info says so.
pub(crate) const SWATCH_GAP_X: Pixels = px(16.);
pub(crate) const SWATCH_GAP_Y: Pixels = px(4.);

/// The space between a Theme Map swatch's square and its label: the
/// showcase's own, gpui's `gap_2`, as the theme states no swatch (spec
/// §3.1). Each swatch's info says so.
pub(crate) const SWATCH_LABEL_GAP: Rems = rems(0.5);

/// A page's root, as the Basic page lays out its own: padded by
/// `layout.window_margin`, its sections `layout.section_gap` apart, and
/// neither where the theme states none.
pub(crate) fn page_root(layout: &LayoutTheme) -> Div {
    with_gap(
        with_padding(v_flex(), geometry::window_margin(layout)),
        geometry::section_gap(layout),
    )
}
