//! `TabBar`: a row of tabs on gpui-base's headless `Tabs` and `Tab`, painted
//! from `TabTheme`.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, FontWeight, Hsla, InteractiveElement as _, IntoElement,
    ParentElement, Pixels, RenderOnce, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px, transparent_black,
};
use gpui_base::{Tab as BaseTab, Tabs as BaseTabs};
use gpui_component::StyledExt as _;
use native_theme::theme::ResolvedTheme;

use super::{color, length, native, over, text_size};

/// A tab's side padding where `tab.border.padding` states none:
/// gpui-component's for a tab at the default `Size` (tab/tab.rs:72-80,
/// `TabVariant::inner_paddings`).
const TAB_PADDING: f32 = 12.;

/// What a tab bar paints, from `TabTheme`.
#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// (tab/tab_bar.rs:366-369, [`TAB_GAP`]).
    pub gap: Pixels,
}

/// The space between tabs where `tab.item_gap` is unstated: gpui-component
/// lays the tabs of its default variant, `TabVariant::Tab`, with no gap
/// (tab/tab_bar.rs:366-369).
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
        })
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
/// `border.corner_radius`). Nothing else marks it: gpui-component's own
/// variants add a primary underline or a frame in `border`
/// (tab/tab.rs, `TabVariant`), and `tab.*` states neither. Every tab is at
/// least `min_width` by `min_height`, padded by the stated `border.padding`
/// sides (gpui-component's 12px on a side left unstated), in `tab.font`'s
/// size and weight, `item_gap` from its neighbours (none where unstated, as
/// gpui-component's). The tabs scroll sideways where they do not fit; a
/// [`suffix`](Self::suffix) follows them.
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
        }
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

    /// Called with the index of the tab clicked.
    #[must_use]
    pub fn on_click(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(n) = native(cx) else {
            return self.fallback();
        };
        let Some(look) = TabLook::of(n.resolved) else {
            return self.fallback();
        };
        let font = &n.resolved.tab.font;
        let (size, weight) = (text_size(font.size, n), FontWeight(f32::from(font.weight)));
        let total = self.tabs.len();
        let selected = self.selected;
        let tabs = self.tabs.into_iter().enumerate().map(|(ix, tab)| {
            let on_click = self.on_click.clone();
            let is_selected = ix == selected;
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
                    } else {
                        tab.bg(look.idle)
                            .text_color(look.idle_text)
                            .border_color(transparent_black())
                            .hover(move |style| style.bg(look.hover).text_color(look.hover_text))
                    }
                })
                .child(div().text_size(size).font_weight(weight).child(tab.label))
                .when_some(on_click, |tab, on_click| {
                    tab.on_click(move |_, window, cx| on_click(&ix, window, cx))
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
            .overflow_x_scroll()
            .gap(look.gap)
            .children(tabs);
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
