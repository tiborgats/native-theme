//! `TabBar`: a row of tabs on gpui-base's headless `Tabs` and `Tab`, painted
//! from `TabTheme`.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    AnyElement, App, BorderStyle, Bounds, ContentMask, ElementId, Hsla, InteractiveElement as _,
    IntoElement, ParentElement, Pixels, RenderOnce, ScrollHandle, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, canvas, div, point,
    prelude::FluentBuilder as _, px, quad, size, transparent_black,
};
use gpui_base::{Tab as BaseTab, Tabs as BaseTabs};
use gpui_component::StyledExt as _;
use native_theme::theme::{ResolvedTheme, TabIndicatorSide};

use super::{Part, PartBounds, color, font_text, length, outer_edge, over, part_bounds_above};
use crate::ActiveNativeTheme;

/// A tab's side padding where `tab.border.padding` states none:
/// gpui-component's for a tab at the default `Size` (tab/tab.rs:112-118,
/// `TabVariant::inner_paddings`).
const TAB_PADDING: f32 = 12.;

/// What a tab bar paints, from `TabTheme`, the selected tab's shape
/// included: rounded on its top corners and, where the theme states
/// [`outward_radius`](Self::outward_radius), curving out into the bar at its
/// feet.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct TabLook {
    /// `tab.bar_background`: the bar behind the tabs.
    pub bar: Hsla,
    /// `tab.background_color`: an unselected tab.
    pub idle: Hsla,
    /// `tab.font.color`: an unselected tab's label.
    pub idle_text: Hsla,
    /// An unselected tab under the pointer: `tab.hover_background` in place
    /// of its fill, over the bar -- Breeze paints the hover colour at alpha
    /// 0.2 instead of the tab's fill (docs/platform-facts.md §2.11, KDE
    /// `background_color`); the idle fill where the theme states none.
    pub hover: Hsla,
    /// `tab.hover_text_color`.
    pub hover_text: Hsla,
    /// `tab.active_background`: the selected tab.
    pub active: Hsla,
    /// `tab.active_text_color`: the selected tab's label.
    pub active_text: Hsla,
    /// `tab.border.color`: the selected tab's outline.
    pub border: Hsla,
    /// `tab.border.line_width`.
    pub border_width: Pixels,
    /// `tab.border.corner_radius`, on the selected tab's top corners.
    pub radius: Pixels,
    /// `tab.min_width`.
    pub min_width: Pixels,
    /// `tab.min_height`.
    pub min_height: Pixels,
    /// `tab.border.padding` left, gpui-component's 12px where unstated.
    pub padding_left: Pixels,
    /// `tab.border.padding` right, gpui-component's 12px where unstated.
    pub padding_right: Pixels,
    /// `tab.border.padding` top, where stated.
    pub padding_top: Option<Pixels>,
    /// `tab.border.padding` bottom, where stated.
    pub padding_bottom: Option<Pixels>,
    /// `tab.item_gap`: the space between neighbouring tabs; where the theme
    /// states none, gpui-component's for its `TabVariant::Tab`, none
    /// (tab/tab_bar.rs:385-388).
    pub gap: Pixels,
    /// The line that marks the selected tab, where the theme states its
    /// colour, width and side.
    pub indicator: Option<TabIndicator>,
    /// `tab.active_outward_corner_radius`: the selected tab's outward bottom
    /// corners, where the theme states a positive one. The selected tab's
    /// outline then has no bottom edge, and at each of its feet a flare
    /// spreads its fill into the bar's bottom edge, the outline running on
    /// down the curve, as WinUI 3's `TabView` draws its selected tab
    /// (docs/platform-facts.md §2.11, `active_outward_corner_radius`;
    /// [`TabBar`]).
    pub outward_radius: Option<Pixels>,
}

/// One quad of the selected tab's flares ([`flare_quads`]): an annulus,
/// drawn as a transparent quad's border, clipped to the flare's square.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FlareQuad {
    /// The flare's square, the quad's content mask.
    pub(crate) clip: Bounds<Pixels>,
    /// The annulus's outer bounds, about the curve's centre.
    pub(crate) bounds: Bounds<Pixels>,
    /// The annulus's outer radius, the quad's corner radius.
    pub(crate) radius: Pixels,
    /// The annulus's width, the quad's border.
    pub(crate) width: Pixels,
    /// The annulus's colour.
    pub(crate) color: Hsla,
}

/// The quads of the two flares at the feet of the selected tab whose outer
/// bounds are `tab`, outline included; none where `look` has no
/// [`outward_radius`](TabLook::outward_radius).
///
/// With `r` that radius and `w` the outline's width, each flare lies in an
/// `r` by `r` square at the tab's bottom, set `w` in from its outer edge, as
/// WinUI's outline lies just outside its fill (docs/platform-facts.md
/// §2.11): the left one's right edge at `x0 + w`, the right one's left edge
/// at `x1 - w`. About the square's upper outer corner, clipped to the
/// square, an annulus from `r - w` (or 0) out to `r * sqrt(2) + 1`, past the
/// square's far corner, in `tab.active_background`, and, where `w` is
/// positive, one from `r - w` (or 0) to `r` in `tab.border.color`, the
/// outline continued down the curve on the disc's side to the bar's bottom
/// edge. The outline is painted after the fill and over it, so what shows is
/// the square less the quarter disc of radius `r` in the fill and the band
/// in the outline. The fill reaches under the band, not only to `r`, because
/// gpui snaps each quad's bounds and border width to device pixels but not
/// its corner radius (gpui-pre `src/window.rs:4684-4696`, `snap_bounds`,
/// `snap_border_widths`): two annuli that only met at `r` would part along
/// the curve at a fractional scale, while the overlap stays closed at any.
/// At the square's top the band covers the side edge's own column, so the
/// two are one line; below it, the fill covers the side edge's foot. Each
/// square is the quads' content mask, which gpui widens to whole device
/// pixels (`snapped_content_mask`, `cover_bounds`, `:3312-3331`). The fills
/// come first, then the outlines.
pub(crate) fn flare_quads(tab: Bounds<Pixels>, look: &TabLook) -> Vec<FlareQuad> {
    let Some(r) = look.outward_radius else {
        return Vec::new();
    };
    let r = f32::from(r);
    let w = f32::from(look.border_width);
    let (left, right) = (f32::from(tab.left()), f32::from(tab.right()));
    let top = f32::from(tab.bottom()) - r;
    let far = r * std::f32::consts::SQRT_2 + 1.0;
    // Each foot's square's left edge and its curve's centre.
    let feet = [(left + w - r, left + w - r), (right - w, right - w + r)];
    let ring = |(edge, centre): (f32, f32), outer: f32, width: f32, color: Hsla| FlareQuad {
        clip: Bounds::new(point(px(edge), px(top)), size(px(r), px(r))),
        bounds: Bounds::new(
            point(px(centre - outer), px(top - outer)),
            size(px(outer + outer), px(outer + outer)),
        ),
        radius: px(outer),
        width: px(width),
        color,
    };
    // The outline's inner radius, where the fill begins too.
    let inner = (r - w).max(0.0);
    let fills = feet.map(|foot| ring(foot, far, far - inner, look.active));
    let outlines = (w > 0.0).then(|| feet.map(|foot| ring(foot, r, r - inner, look.border)));
    fills
        .into_iter()
        .chain(outlines.into_iter().flatten())
        .collect()
}

/// An absolute, empty canvas that paints the selected tab's flares
/// ([`flare_quads`]) about the outer bounds `tab` holds once the frame has
/// laid the tab out, each quad under the content mask of its square, which
/// gpui intersects with the row's own clip.
fn flares(tab: Rc<Cell<Option<Bounds<Pixels>>>>, look: TabLook) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |_, (), window, _| {
            let Some(tab) = tab.get() else {
                return;
            };
            for flare in flare_quads(tab, &look) {
                window.with_content_mask(Some(ContentMask { bounds: flare.clip }), |window| {
                    window.paint_quad(quad(
                        flare.bounds,
                        flare.radius,
                        transparent_black(),
                        flare.width,
                        flare.color,
                        BorderStyle::default(),
                    ));
                });
            }
        },
    )
    .absolute()
}

/// The line along one edge of the selected tab: `tab.active_indicator_color`,
/// `tab.active_indicator_width` thick, along `tab.active_indicator_side`
/// (docs/platform-facts.md §2.11: Breeze's `Highlight` strip on top,
/// libadwaita's accent and Material's `primary` at the bottom).
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct TabIndicator {
    /// `tab.active_indicator_color`.
    pub color: Hsla,
    /// `tab.active_indicator_width`: the line's thickness.
    pub width: Pixels,
    /// `tab.active_indicator_side`.
    pub side: TabIndicatorSide,
}

/// The space between tabs where `tab.item_gap` is unstated: gpui-component
/// lays the tabs of its default variant, `TabVariant::Tab`, with no gap
/// (tab/tab_bar.rs:385-388).
const TAB_GAP: f32 = 0.;

/// A stated side, `None` where it is unstated, or `Err` where it is not a
/// finite, non-negative length.
fn side(v: Option<f32>) -> Result<Option<Pixels>, ()> {
    v.map_or(Ok(None), |v| length(v).map(Some).ok_or(()))
}

impl TabLook {
    /// The look of a tab bar, or `None` when a length the theme gives is not
    /// finite.
    #[must_use]
    pub fn of(resolved: &ResolvedTheme) -> Option<Self> {
        let t = &resolved.tab;
        let b = &t.border;
        let bar = color(t.bar_background);
        let idle = color(t.background_color);
        let unstated = px(TAB_PADDING);
        Some(Self {
            bar,
            idle,
            idle_text: color(t.font.color),
            hover: t.hover_background.map_or(idle, |h| over(bar, color(h))),
            hover_text: color(t.hover_text_color),
            active: color(t.active_background),
            active_text: color(t.active_text_color),
            border: color(b.color),
            border_width: length(b.line_width)?,
            radius: length(b.corner_radius)?,
            min_width: length(t.min_width)?,
            min_height: length(t.min_height)?,
            padding_left: side(b.padding.left).ok()?.unwrap_or(unstated),
            padding_right: side(b.padding.right).ok()?.unwrap_or(unstated),
            padding_top: side(b.padding.top).ok()?,
            padding_bottom: side(b.padding.bottom).ok()?,
            gap: side(t.item_gap).ok()?.unwrap_or(px(TAB_GAP)),
            indicator: match (
                t.active_indicator_color,
                side(t.active_indicator_width).ok()?,
                t.active_indicator_side,
            ) {
                (Some(c), Some(width), Some(side)) => Some(TabIndicator {
                    color: color(c),
                    width,
                    side,
                }),
                _ => None,
            },
            outward_radius: side(t.active_outward_corner_radius)
                .ok()?
                .filter(|r| *r > px(0.)),
        })
    }
}

/// The selected tab's indicator: a strip `line.width` thick across the tab's
/// outer edge on `line.side`, over its outline, as Breeze fills its strip
/// from the tab's frame rectangle (breezehelper.cpp:1471-1487); on top,
/// rounded as the tab's top corners are. `bottom` is the width of the tab's
/// bottom border, which it has none of where the tab curves out at its feet.
fn indicator(line: TabIndicator, look: &TabLook, bottom: Pixels) -> gpui::Div {
    let outside = px(-f32::from(look.border_width));
    let strip = div()
        .absolute()
        .left(outside)
        .right(outside)
        .h(line.width)
        .bg(line.color)
        .debug_selector(|| "native-tab-indicator".into());
    match line.side {
        TabIndicatorSide::Top => strip
            .top(outside)
            .rounded_tl(look.radius)
            .rounded_tr(look.radius),
        TabIndicatorSide::Bottom => strip.bottom(px(-f32::from(bottom))),
    }
}

/// One tab of a [`TabBar`]: its label.
pub struct Tab {
    label: SharedString,
    selector: Option<String>,
}

impl Tab {
    /// A tab labelled `label`.
    #[must_use]
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            selector: None,
        }
    }

    /// The key a test looks the tab's bounds up by, as gpui's
    /// `InteractiveElement::debug_selector`.
    #[must_use]
    pub fn debug_selector(mut self, f: impl FnOnce() -> String) -> Self {
        self.selector = Some(f());
        self
    }
}

/// What a click on the tab at an index runs.
type TabClick = Rc<dyn Fn(&usize, &mut Window, &mut App)>;

/// A row of tabs drawn as `tab.*` states them, on gpui-base's headless
/// `Tabs` (role `TabList`) and `Tab` (role `Tab`, selected state, position
/// in the set, activation by click).
///
/// The bar is `bar_background`. An unselected tab is filled
/// `background_color` and lettered in `font.color`; under the pointer it is
/// `hover_background` in place of that fill, lettered in `hover_text_color`.
/// The selected tab is filled `active_background`, lettered in
/// `active_text_color` and outlined by `tab.border` -- the outline on the
/// selected tab only and rounded on its top corners, as Breeze and Windows
/// draw it (docs/platform-facts.md §2.11, `border.color`,
/// `border.corner_radius`) -- and, where the theme states
/// `active_indicator_color`, `active_indicator_width` and
/// `active_indicator_side`, by a line that thick along that edge
/// ([`TabIndicator`]). Where the theme states `active_outward_corner_radius`,
/// the selected tab curves out into the bar's bottom edge at both feet, as
/// WinUI 3's `TabView` draws it (docs/platform-facts.md §2.11,
/// `active_outward_corner_radius`): its outline has no bottom edge, and at
/// each foot a flare -- the square of that radius beside the foot less the
/// quarter disc of the same radius, filled `active_background` -- spreads
/// its fill into the bar, the outline running on down the curve to the bar's
/// bottom edge. The flares are painted after every tab, over a hovered
/// neighbour too, and inside the row's clip, so a tab scrolled away takes
/// its flares with it; the row keeps that radius free at both ends, so the
/// end tabs' flares show. Nothing else marks it: gpui-component's own
/// variants add a primary underline or a frame in `border`
/// (tab/tab.rs, `TabVariant`), whatever the platform draws. Every tab is at
/// least `min_width` by `min_height`, padded by the stated `border.padding`
/// sides (gpui-component's 12px on a side left unstated), in `tab.font`'s
/// family, size, weight and style, `item_gap` from its neighbours (none
/// where unstated, as gpui-component's). The tabs scroll sideways where they
/// do not fit, a clicked tab into view
/// ([`track_scroll`](Self::track_scroll)), or wrap onto a further row with
/// [`wrap`](Self::wrap); a [`suffix`](Self::suffix) follows them. A vertical
/// mouse wheel scrolls them sideways too: gpui turns it so on a row that
/// scrolls on one axis alone (gpui-pre `src/elements/div.rs:3354-3362`), as
/// gpui-component 0.8.0's `TabBar`, whose row locks its axis, does by a wheel
/// mask (`tab/tab_bar.rs:691`, `:697`).
///
/// The bar's own style (`Styled`) is the caller's: a rule under it, its
/// inset. Without a native theme, or with a length that is not finite, it
/// renders gpui-component's `TabBar`.
pub struct TabBar {
    id: ElementId,
    tabs: Vec<Tab>,
    selected: usize,
    on_click: Option<TabClick>,
    suffix: Option<AnyElement>,
    style: StyleRefinement,
    observer: Option<PartBounds>,
    wrap: bool,
    scroll: Option<ScrollHandle>,
}

impl TabBar {
    /// A new, empty bar.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            selected: 0,
            on_click: None,
            suffix: None,
            style: StyleRefinement::default(),
            observer: None,
            wrap: false,
            scroll: None,
        }
    }

    /// Hands `observer` the bounds of each tab, as [`Part::Tab`] with its
    /// index, as a frame lays them out ([`PartBounds`]).
    #[must_use]
    pub fn on_part_bounds(mut self, observer: PartBounds) -> Self {
        self.observer = Some(observer);
        self
    }

    /// Adds a tab.
    #[must_use]
    pub fn child(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    /// Adds the tabs, in order.
    #[must_use]
    pub fn children(mut self, tabs: impl IntoIterator<Item = Tab>) -> Self {
        self.tabs.extend(tabs);
        self
    }

    /// The index of the selected tab.
    #[must_use]
    pub fn selected_index(mut self, ix: usize) -> Self {
        self.selected = ix;
        self
    }

    /// Called with the index of the tab clicked, after the bar has asked for
    /// that tab to be scrolled into view on the next frame
    /// ([`track_scroll`](Self::track_scroll)).
    #[must_use]
    pub fn on_click(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// Tracks the tabs' sideways scroll with `handle`. Without one, the bar
    /// keeps a handle of its own.
    ///
    /// Where an [`on_click`](Self::on_click) handler is set, a click on a tab
    /// asks the handle to scroll the tab at that index into view
    /// (`ScrollHandle::scroll_to_item`) before the handler runs. The scroll
    /// lands when the next frame lays the tabs out: the handler still sees
    /// the old offset and the tab's old bounds, a reveal it asks for itself
    /// replaces the bar's, and a handler that reorders or removes tabs
    /// reveals whichever tab is then at that index. Under
    /// [`wrap`](Self::wrap) the tabs do not scroll, so a click reveals
    /// nothing. A change of the selection, the tabs or the layout scrolls
    /// nothing, as in gpui-component's `TabBar::track_scroll`
    /// (tab/tab_bar.rs:138-142); use `handle` to reveal a tab otherwise.
    #[must_use]
    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll = Some(handle.clone());
        self
    }

    /// Wraps the tabs onto a further row where the bar is too narrow for
    /// them, instead of scrolling them sideways.
    #[must_use]
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// An element after the tabs, at the bar's end.
    #[must_use]
    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    fn fallback(self) -> AnyElement {
        let on_click = self.on_click;
        gpui_component::tab::TabBar::new(self.id)
            .children(self.tabs.into_iter().map(|tab| {
                let item = gpui_component::tab::Tab::new().label(tab.label);
                match tab.selector {
                    Some(selector) => item.debug_selector(|| selector),
                    None => item,
                }
            }))
            .selected_index(self.selected)
            .when_some(self.scroll.as_ref(), |bar, handle| bar.track_scroll(handle))
            .when_some(on_click, |bar, on_click| {
                bar.on_click(move |ix, window, cx| on_click(ix, window, cx))
            })
            .when_some(self.suffix, |bar, suffix| bar.suffix(suffix))
            .refine_style(&self.style)
            .into_any_element()
    }
}

impl Styled for TabBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TabBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(look) = cx.native().and_then(|n| TabLook::of(n.resolved)) else {
            return self.fallback();
        };
        // Without a tracked handle the bar keeps one, to reveal a clicked
        // tab, as gpui-component's does (tab/tab_bar.rs:458-466).
        let scroll = self.scroll.clone().unwrap_or_else(|| {
            window
                .use_keyed_state(format!("{}-scroll", self.id), cx, |_, _| {
                    ScrollHandle::new()
                })
                .read(cx)
                .clone()
        });
        let view = window.current_view();
        // Read again: the keyed handle took `cx` mutably.
        let Some(n) = cx.native() else {
            return self.fallback();
        };
        let total = self.tabs.len();
        let selected = self.selected;
        let observer = self.observer.clone();
        let outward = look.outward_radius;
        // The selected tab's outer bounds, recorded while the frame is laid
        // out, for the flares painted after the tabs.
        let selected_bounds = Rc::new(Cell::new(None));
        let tabs = self.tabs.into_iter().enumerate().map(|(ix, tab)| {
            let on_click = self.on_click.clone();
            let scroll = scroll.clone();
            let is_selected = ix == selected;
            // A selected tab that curves out at its feet has no bottom edge,
            // so nothing is inset from its bottom side.
            let curves_out = is_selected && outward.is_some();
            let bottom = if curves_out {
                px(0.)
            } else {
                look.border_width
            };
            let recorded = selected_bounds.clone();
            BaseTab::new(ix)
                .selected(is_selected)
                .set_position(ix.saturating_add(1), total)
                .accessibility_label(tab.label.clone())
                .flex_none()
                .min_w(look.min_width)
                .min_h(look.min_height)
                .pl(look.padding_left)
                .pr(look.padding_right)
                .when_some(look.padding_top, |tab, top| tab.pt(top))
                .when_some(look.padding_bottom, |tab, bottom| tab.pb(bottom))
                // Every tab keeps the outline's width, so a label sits where
                // it sits whichever tab is selected; only the selected one's
                // shows.
                .border(look.border_width)
                .map(|tab| {
                    if is_selected {
                        tab.bg(look.active)
                            .text_color(look.active_text)
                            .border_color(look.border)
                            .rounded_tl(look.radius)
                            .rounded_tr(look.radius)
                            // The label stays where the bottom edge held it.
                            .when(curves_out, |tab| {
                                tab.border_b(px(0.))
                                    .pb(px(f32::from(look.padding_bottom.unwrap_or(px(0.)))
                                        + f32::from(look.border_width)))
                            })
                    } else {
                        tab.bg(look.idle)
                            .text_color(look.idle_text)
                            .border_color(transparent_black())
                            .hover(move |style| style.bg(look.hover).text_color(look.hover_text))
                    }
                })
                .child(font_text(&n.resolved.tab.font, n).child(tab.label))
                .when_some(look.indicator.filter(|_| is_selected), |tab, line| {
                    tab.relative().child(indicator(line, &look, bottom))
                })
                .children(
                    observer
                        .as_ref()
                        .map(|o| part_bounds_above(Part::Tab(ix), o, look.border_width, bottom)),
                )
                .when(curves_out, |tab| {
                    tab.child(outer_edge(look.border_width, px(0.), move |b, _, _| {
                        recorded.set(Some(b));
                    }))
                })
                .when_some(on_click, |tab, on_click| {
                    tab.on_click(move |_, window, cx| {
                        scroll.scroll_to_item(ix);
                        cx.notify(view);
                        on_click(&ix, window, cx)
                    })
                })
                .when_some(tab.selector, |tab, selector| {
                    tab.debug_selector(|| selector)
                })
        });
        // Ids within the bar's own: the tabs within their row.
        let row = BaseTabs::new("tabs")
            .flex()
            .flex_row()
            .items_end()
            .flex_auto()
            .min_w_0()
            .map(|row| {
                if self.wrap {
                    row.flex_wrap()
                } else {
                    row.overflow_x_scroll()
                }
            })
            // Room for the end tabs' flares, inside the row's clip, as
            // gpui-component pads its Folder row (tab/tab_bar.rs:686-687).
            .when_some(outward, |row, r| row.pl(r).pr(r))
            .gap(look.gap)
            .track_scroll(&scroll)
            .children(tabs)
            .children(outward.map(|_| flares(selected_bounds, look)));
        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_end()
            .bg(look.bar)
            .refine_style(&self.style)
            .debug_selector(|| "native-tab-bar".into())
            .child(row)
            .children(
                self.suffix
                    .map(|suffix| div().flex_none().self_center().child(suffix)),
            )
            .into_any_element()
    }
}
