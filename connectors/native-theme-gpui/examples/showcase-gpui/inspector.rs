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
    v_flex,
};
use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::info::{INFO_SETTLE, InfoRegistry, Note, WidgetInfo};
use crate::support::{NativeStyled, color_swatch, defined_size, with_gap, with_padding};
use crate::{INSPECTOR_COPY, INSPECTOR_PANEL, INSPECTOR_TABS, INSPECTOR_TITLE, demo, probe};

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
}

pub(crate) struct Inspector {
    ui: Entity<InfoRegistry>,
    /// Where the Theme tab reads the installed theme's fonts and layout.
    showcase: WeakEntity<Showcase>,
    pub(crate) tab: InspectorTab,
    /// The text panel of a page that does not report its instances yet (plan
    /// Tasks 14-23), shown in place of an info until an info settles.
    // Task 24: delete (legacy hover_info stopgap)
    legacy: Option<String>,
    /// The text panel waiting out `INFO_SETTLE`, and its ticket.
    // Task 24: delete (legacy hover_info stopgap)
    legacy_pending: Option<(String, u64)>,
    // Task 24: delete (legacy hover_info stopgap)
    legacy_tickets: u64,
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
        // The registry notifies when what it shows changes; an info that
        // settles replaces a page's text panel.
        let _registry = cx.observe(&ui, |this: &mut Self, ui, cx| {
            // Task 24: delete (legacy hover_info stopgap)
            if ui.read(cx).shown().is_some() {
                this.legacy = None;
            }
            cx.notify();
        });
        Self {
            ui,
            showcase,
            tab: InspectorTab::Widget,
            legacy: None,
            legacy_pending: None,
            legacy_tickets: 0,
            title_drawn: None,
            _registry,
        }
    }

    /// A hover over a page's text panel began (`hovered`) or ended. The
    /// panel replaces what is shown only after it stayed hovered for
    /// `INFO_SETTLE`, as an info does (spec §4.2), so crossing a page on the
    /// way to the inspector leaves the inspector as it was.
    // Task 24: delete (legacy hover_info stopgap)
    pub(crate) fn set_legacy(&mut self, text: String, hovered: bool, cx: &mut Context<Self>) {
        if !hovered {
            self.legacy_pending.take_if(|(pending, _)| *pending == text);
            return;
        }
        if self.legacy.as_ref() == Some(&text) {
            self.legacy_pending = None;
            return;
        }
        self.legacy_tickets = self.legacy_tickets.wrapping_add(1);
        let ticket = self.legacy_tickets;
        self.legacy_pending = Some((text, ticket));
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(INFO_SETTLE).await;
            this.update(cx, |this, cx| {
                let Some((text, _)) = this.legacy_pending.take_if(|(_, t)| *t == ticket) else {
                    return;
                };
                this.ui.update(cx, |r, _| r.forget_shown());
                this.legacy = Some(text);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Drop the text panel of a page that is no longer shown.
    // Task 24: delete (legacy hover_info stopgap)
    pub(crate) fn clear_legacy(&mut self, cx: &mut Context<Self>) {
        self.legacy_pending = None;
        if self.legacy.take().is_some() {
            cx.notify();
        }
    }

    /// The Widget tab: the shown info's title, a Copy button and its
    /// sections, or one line of hint before anything was hovered.
    fn widget_tab(&mut self, gap: Option<gpui::Pixels>, cx: &mut Context<Self>) -> gpui::Div {
        let theme = cx.theme().clone();
        let muted = theme.muted_foreground;
        let shown = self.ui.read(cx).shown().cloned();
        let (title, copied, body) = match (&self.legacy, shown) {
            (Some(text), _) => {
                let mut lines = text.lines();
                let title = lines.next().unwrap_or_default().to_string();
                let rest: Vec<&str> = lines.collect();
                (
                    title,
                    text.clone(),
                    div()
                        .text_xs()
                        .child(SharedString::from(rest.join("\n")))
                        .into_any_element(),
                )
            }
            (None, Some(info)) => (
                info.title(),
                info.to_text(),
                info_sections(&info, gap, cx).into_any_element(),
            ),
            (None, None) => {
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
                    .child(probe(
                        INSPECTOR_COPY,
                        Button::new("inspector-copy")
                            .label("Copy")
                            .small()
                            .ghost()
                            .on_click(move |_, _, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(copied.clone()))
                            }),
                    )),
            )
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

        // `Root::new` sets `bordered` (root.rs:117) and `Root::render` wraps
        // everything it holds in `window_border()` (root.rs:605), so the
        // window's frame is a WindowBorder no page draws. Its client-side arm
        // alone sets the client inset (window_border.rs:147-149).
        let decorations = match window.window_decorations() {
            gpui::Decorations::Server => "server-side: the compositor draws the frame".to_string(),
            gpui::Decorations::Client { tiling } => {
                format!("client-side, tiled {tiling:?}")
            }
        };
        let inset = match window.client_inset() {
            Some(inset) => format!("{}px, set by the WindowBorder", inset.as_f32()),
            None => "none: nothing has called set_client_inset".to_string(),
        };
        let window_rows = vec![
            ("decorations", decorations),
            ("client inset", inset),
            (
                "frame",
                "a WindowBorder, which Root draws around everything it holds (root.rs, Root)"
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
            (
                "server-side",
                "nothing is drawn: the compositor owns the frame (window_border.rs, WindowBorder)"
                    .to_string(),
            ),
        ];

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

/// The four sections of `info` (spec §2.6), each only where it has lines.
fn info_sections(info: &WidgetInfo, gap: Option<gpui::Pixels>, cx: &gpui::App) -> gpui::Div {
    let muted = cx.theme().muted_foreground;
    let frame = gpui::StyleRefinement::default().demo_frame(cx);
    let colors = info.colors.iter().map(|c| {
        v_flex()
            .child(color_swatch(
                &format!("{}: {}", c.role, c.field),
                c.value,
                &frame,
            ))
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
            InspectorTab::ALL.map(InspectorTab::label),
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
