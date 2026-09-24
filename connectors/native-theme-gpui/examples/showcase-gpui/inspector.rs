//! The inspector (spec §2.6): what the theme sets on the widget the pointer
//! settled on, and what the theme and the window set that no widget carries.
//!
//! Only its TabBar reports itself. Everything below the TabBar is built
//! without `.info()` (spec §4.4), so moving the pointer into the inspector to
//! read or copy leaves the info in place.

use gpui::{
    ClipboardItem, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, WeakEntity, Window, div, prelude::*,
};
use gpui_component::{
    ActiveTheme, Sizable as _, StyledExt,
    button::{Button, ButtonVariants as _},
    h_flex,
    label::Label,
    scroll::ScrollableElement,
    v_flex, window_paddings,
};
use native_theme_gpui::{ActiveNativeTheme as _, geometry, variants};

use crate::app::Showcase;
use crate::demo::TabBarKind;
use crate::info::{InfoRegistry, Note, WidgetInfo, hsla_to_hex};
use crate::support::{NativeStyled, defined_size, with_gap, with_padding};
use crate::{
    INSPECTOR_COPY, INSPECTOR_PANEL, INSPECTOR_TABS, INSPECTOR_TITLE, INSPECTOR_TOKENS_NOTE, demo,
    probe,
};

/// The inspector's two views, in the order its TabBar shows them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum InspectorTab {
    Widget,
    Theme,
}

impl InspectorTab {
    const ALL: [Self; 2] = [Self::Widget, Self::Theme];

    fn index(self) -> usize {
        match self {
            Self::Widget => 0,
            Self::Theme => 1,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Widget => "Widget",
            Self::Theme => "Theme",
        }
    }

    /// The debug selector of the view's tab.
    pub(crate) fn tab(self) -> &'static str {
        match self {
            Self::Widget => "inspector-tab-widget",
            Self::Theme => "inspector-tab-theme",
        }
    }
}

pub(crate) struct Inspector {
    ui: Entity<InfoRegistry>,
    /// Where the Theme tab reads the installed theme's fonts and layout.
    showcase: WeakEntity<Showcase>,
    pub(crate) tab: InspectorTab,
    /// The title of what the last frame drew under the TabBar; `None` for
    /// the hint shown before any hover.
    pub(crate) title_drawn: Option<SharedString>,
    _registry: Subscription,
}

impl Inspector {
    pub(crate) fn new(
        ui: Entity<InfoRegistry>,
        showcase: WeakEntity<Showcase>,
        cx: &mut Context<Self>,
    ) -> Self {
        // The registry notifies when what it shows changes.
        let _registry = cx.observe(&ui, |_: &mut Self, _, cx| cx.notify());
        Self {
            ui,
            showcase,
            tab: InspectorTab::Widget,
            title_drawn: None,
            _registry,
        }
    }

    /// The title of what the Widget tab shows, `None` for the hint: the
    /// status bar names the widget by this, so the two never disagree.
    pub(crate) fn shown_title(&self, cx: &gpui::App) -> Option<String> {
        self.ui.read(cx).shown().map(|info| info.title())
    }

    /// The Widget tab: the shown info's title, a Copy button and its
    /// sections, or one line of hint before anything was hovered. With no
    /// native theme installed, a note says the swatches may not be what is
    /// painted.
    fn widget_tab(&mut self, gap: Option<gpui::Pixels>, cx: &mut Context<Self>) -> gpui::Div {
        let theme = cx.theme().clone();
        let muted = theme.muted_foreground;
        let shown = self.ui.read(cx).shown().cloned();
        let (title, copied, body) = match shown {
            Some(info) => (info.title(), info.to_text(), info_sections(&info, gap, cx)),
            None => {
                self.title_drawn = None;
                return v_flex().child(
                    Label::new("Hover any widget to see what the theme sets on it.")
                        .text_sm()
                        .text_color(muted),
                );
            }
        };
        let title = SharedString::from(title);
        self.title_drawn = Some(title.clone());
        with_gap(v_flex(), gap)
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .debug_selector(|| INSPECTOR_TITLE.into())
                            .child(Label::new(title).text_sm().font_semibold()),
                    )
                    // The Buttons page's Ghost, as the toolbar's buttons are.
                    .child(probe(
                        INSPECTOR_COPY,
                        Button::new("inspector-copy")
                            .label("Copy")
                            .small()
                            .custom(variants::ghost_button(cx))
                            .on_click(move |_, _, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(copied.clone()))
                            }),
                    )),
            )
            // A claim's value is the ThemeColor field it names, and upstream
            // paints many widgets from `Theme::tokens` instead, which a theme
            // can set apart from its fields (theme/schema.rs:1014, and the
            // test at :1299 asserting a pair that differs). Said only before
            // the connector's `apply` has run, while gpui-component's own
            // theme is up.
            .when(cx.native_theme().is_none(), |tab| {
                tab.child(
                    div().debug_selector(|| INSPECTOR_TOKENS_NOTE.into()).child(
                        Label::new(
                            "No native theme is installed, so the theme is gpui-component's own. \
                             The swatches show its ThemeColor fields, but upstream paints many \
                             widgets from Theme::tokens, which a theme can set apart from those \
                             fields: a swatch may not be the colour on screen.",
                        )
                        .text_sm()
                        .text_color(muted),
                    ),
                )
            })
            .child(body)
    }

    /// The Theme tab: what the Theme Config Inspector showed, and the facts
    /// of the window no hover target carries.
    fn theme_tab(
        &self,
        gap: Option<gpui::Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let theme = cx.theme().clone();
        let fonts = self.showcase.upgrade().map(|showcase| {
            let showcase = showcase.read(cx);
            (
                showcase.original_font.clone(),
                showcase.original_mono_font.clone(),
            )
        });
        let rows = vec![
            ("radius", format!("{}px", theme.radius.as_f32())),
            ("radius_lg", format!("{}px", theme.radius_lg.as_f32())),
            ("shadow", theme.shadow.to_string()),
            ("scrollbar_mode", format!("{:?}", theme.scrollbar_mode)),
        ];
        // In the unit the platform stated, like the Widget Info: these rows
        // report the theme's own definition, not the pixel value gpui lays
        // out with.
        let font_rows = match &fonts {
            Some((font, mono)) => vec![
                ("font_family", font.family.to_string()),
                ("font_size", defined_size(font)),
                ("mono_font_family", mono.family.to_string()),
                ("mono_font_size", defined_size(mono)),
            ],
            None => Vec::new(),
        };

        // The decorations the chrome was drawn for (`Showcase::frame`), so the
        // section and the chrome never disagree.
        let frame = self
            .showcase
            .upgrade()
            .map_or_else(|| window.window_decorations(), |s| s.read(cx).frame(window));
        let window_rows = window_rows(frame, window.client_inset(), window_paddings(window));

        with_gap(v_flex(), gap)
            .child(section("Theme config"))
            .children(rows.into_iter().map(|(what, value)| row(what, value, cx)))
            .child(section("Fonts"))
            .children(
                font_rows
                    .into_iter()
                    .map(|(what, value)| row(what, value, cx)),
            )
            .child(section("Window"))
            .children(
                window_rows
                    .into_iter()
                    .map(|(what, value)| row(what, value, cx)),
            )
    }
}

/// The Theme tab's Window section (spec S8): what draws the window's frame
/// under `window_decorations`, the decorations the window was granted, with
/// the client inset and the frame's insets the window reports. Under
/// server-side decorations the window manager draws the frame; under
/// client-side ones, `Root`'s client frame and the window's TitleBar do.
pub(crate) fn window_rows(
    window_decorations: gpui::Decorations,
    inset: Option<gpui::Pixels>,
    paddings: gpui::Edges<gpui::Pixels>,
) -> Vec<(&'static str, String)> {
    // `Root::new` sets `bordered` (root.rs:117) and `Root::render` wraps
    // everything it holds in `window_border()` (root.rs:605). Its client-side
    // arm alone sets the client inset (window_border.rs:147-149) and draws a
    // frame; the Server arm hands back the bare backdrop (window_border.rs:172)
    // and lays no hit zones (`:270-280`).
    let inset = match inset {
        Some(inset) => format!("{}px, set by the WindowBorder", inset.as_f32()),
        None => "none: nothing has called set_client_inset".to_string(),
    };
    let frame_insets = format!(
        "top {}px, right {}px, bottom {}px, left {}px (window_border.rs, window_paddings)",
        paddings.top.as_f32(),
        paddings.right.as_f32(),
        paddings.bottom.as_f32(),
        paddings.left.as_f32(),
    );
    match window_decorations {
        gpui::Decorations::Server => vec![
            (
                "decorations",
                "server-side: the window manager draws the frame".to_string(),
            ),
            (
                "frame",
                "whatever the window manager draws (KWin: Breeze's title bar, controls, corners and shadow). Root's WindowBorder draws nothing (window_border.rs, WindowBorder)"
                    .to_string(),
            ),
            ("client inset", inset),
            ("frame insets", frame_insets),
            (
                "resize band",
                "the window manager's: the WindowBorder lays none (window_border.rs, WindowBorder)"
                    .to_string(),
            ),
        ],
        gpui::Decorations::Client { tiling } => vec![
            (
                "decorations",
                format!("client-side, tiled {tiling:?}: the window manager leaves the frame to the application"),
            ),
            (
                "frame",
                "Root's client frame: a WindowBorder, which Root draws around everything it holds (root.rs, Root), with the window's TitleBar at its top"
                    .to_string(),
            ),
            ("client inset", inset),
            ("frame insets", frame_insets),
            (
                "resize band",
                "the WindowBorder's, along each edge not tiled (window_border.rs, resize_hit_zones)"
                    .to_string(),
            ),
            (
                "frame fill",
                "none: the WindowBorder's backdrop and frame are transparent (window_border.rs, WindowBorder)"
                    .to_string(),
            ),
            (
                "frame colour",
                "a literal grey, l=0.2 dark / l=0.8 light, that no theme field reaches (window_border.rs, WindowBorder)"
                    .to_string(),
            ),
            (
                "frame shadow",
                "a literal two-layer box shadow (window_border.rs, WindowBorder)".to_string(),
            ),
        ],
    }
}

/// A section's heading.
fn section(title: &'static str) -> Label {
    Label::new(title).text_sm().font_semibold()
}

/// One `what: value` row, the name muted.
fn row(what: &'static str, value: String, cx: &gpui::App) -> gpui::Div {
    v_flex()
        .child(
            Label::new(what)
                .text_xs()
                .text_color(cx.theme().muted_foreground),
        )
        .child(Label::new(value).text_xs())
}

/// The side of a swatch's square. A swatch is the inspector's own element,
/// which the model states nothing of, so this is the showcase's own choice,
/// not a platform's.
const SWATCH_SIZE: gpui::Pixels = gpui::px(16.);

/// The gap between a swatch's square and its label, gpui's `gap_2`: the
/// showcase's own choice, as [`SWATCH_SIZE`] is.
const SWATCH_GAP: gpui::Rems = gpui::rems(0.5);

/// A colour swatch labelled `name` and the colour's hex value, its square
/// in `frame`, the showcase's frame, built once by the caller.
fn swatch(name: &str, color: gpui::Hsla, frame: &gpui::StyleRefinement) -> gpui::Div {
    let label = SharedString::from(format!("{name} {}", hsla_to_hex(color)));
    h_flex()
        .gap(SWATCH_GAP)
        .items_center()
        .child(div().size(SWATCH_SIZE).bg(color).refine_style(frame))
        .child(Label::new(label).text_sm())
}

/// The four sections of `info` (spec §2.6), each only where it has lines.
fn info_sections(info: &WidgetInfo, gap: Option<gpui::Pixels>, cx: &gpui::App) -> gpui::Div {
    let muted = cx.theme().muted_foreground;
    let frame = gpui::StyleRefinement::default().demo_frame(cx);
    let colors = info.colors.iter().map(|c| {
        v_flex()
            .child(swatch(&format!("{}: {}", c.role, c.field), c.value, &frame))
            .child(Label::new(c.cited_at).text_xs().text_color(muted))
    });
    let notes = |title: &'static str, notes: &[Note]| {
        (!notes.is_empty()).then(|| {
            v_flex()
                .child(section(title))
                .children(notes.iter().map(|n| {
                    v_flex()
                        .child(Label::new(n.what).text_xs().text_color(muted))
                        .child(Label::new(n.text.clone()).text_xs())
                }))
        })
    };
    with_gap(v_flex(), gap)
        .when(!info.colors.is_empty(), |this| {
            this.child(section("Theme colors")).children(colors)
        })
        .children(notes("Theme config", &info.config))
        .children(notes("Not themeable", &info.not_themeable))
        .children(notes("This instance", &info.instance))
}

impl Render for Inspector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let layout = self
            .showcase
            .upgrade()
            .map(|showcase| showcase.read(cx).layout.clone())
            .unwrap_or_default();
        let gap = geometry::widget_gap(&layout);
        let margin = geometry::container_margin(&layout);
        let tabs = demo::tab_bar(
            &self.ui,
            cx,
            TabBarKind::Inspector,
            margin,
            InspectorTab::ALL.map(|tab| (tab.label(), tab.tab())),
            self.tab.index(),
            cx.listener(|this, ix: &usize, _window, cx| {
                if let Some(&tab) = InspectorTab::ALL.get(*ix) {
                    this.tab = tab;
                    cx.notify();
                }
            }),
        )
        .debug_selector(|| INSPECTOR_TABS.into());
        let body = match self.tab {
            InspectorTab::Widget => self.widget_tab(gap, cx),
            InspectorTab::Theme => self.theme_tab(gap, window, cx),
        };
        v_flex()
            .size_full()
            .debug_selector(|| INSPECTOR_PANEL.into())
            .child(tabs)
            .child(
                div()
                    .id("inspector-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    // The bar is drawn over the right edge of the scroll
                    // area, as on the content panel.
                    .native(cx, geometry::scrollbar_gutter)
                    .child(with_padding(v_flex(), margin).child(body)),
            )
    }
}
