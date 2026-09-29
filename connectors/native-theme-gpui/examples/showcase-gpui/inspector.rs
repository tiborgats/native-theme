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
    v_flex, window_paddings,
};
use native_theme_gpui::{ActiveNativeTheme as _, geometry, variants};

use crate::app::Showcase;
use crate::demo::TabBarKind;
use crate::elements;
use crate::info::{InfoRegistry, Note, WidgetInfo, hsla_to_hex, leaves, stated, values};
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
    /// Where its content is scrolled (`support::gutter_scroll`).
    scroll: gpui::ScrollHandle,
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
            scroll: gpui::ScrollHandle::new(),
            _registry,
        }
    }

    /// The title of what the Widget tab shows, `None` for the hint: the
    /// status bar names the widget by this, so the two never disagree.
    pub(crate) fn shown_title(&self, cx: &gpui::App) -> Option<String> {
        self.ui.read(cx).shown().map(|info| shown_title(info))
    }

    /// The Widget tab: the shown info's title, a Copy button and its
    /// sections, or one line of hint before anything was hovered. An element
    /// of docs/showcase-elements.toml, under a native theme, shows in the
    /// list's form, as the iced and egui showcases show it
    /// ([`listed_info`]). With no native theme installed, a note says the
    /// swatches may not be what is painted.
    fn widget_tab(&mut self, gap: Option<gpui::Pixels>, cx: &mut Context<Self>) -> gpui::Div {
        let theme = cx.theme().clone();
        let muted = theme.muted_foreground;
        let shown = self.ui.read(cx).shown().cloned();
        let look = PanelLook::of(&self.showcase, cx);
        let listed = shown.as_ref().and_then(|info| {
            let element = elements::element(info.listed?)?;
            Some((element, info))
        });
        if let (Some(look), Some((element, info))) = (&look, listed) {
            let (panel, title) = listed_info(&self.ui, look, element, info);
            self.title_drawn = Some(title);
            return panel;
        }
        let (title, copied, body) = match shown {
            Some(info) => (info.title(), info.to_text(), info_sections(&info, gap, cx)),
            None => {
                self.title_drawn = None;
                // Clipped to the panel's width, as the other rows are.
                let hint = div()
                    .relative()
                    .w_full()
                    .overflow_hidden()
                    .child(INFO_HINT)
                    .child(elements::record(&self.ui, "chrome.info.hint"));
                return v_flex().items_start().child(match &look {
                    Some(look) => look.text(hint).text_color(look.muted),
                    None => hint.text_sm().text_color(muted),
                });
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
                (
                    "font_family",
                    if theme.font_family.as_ref() == ".SystemUIFont" {
                        format!(
                            "{} (drawn as the system UI font, .SystemUIFont)",
                            font.family
                        )
                    } else {
                        font.family.to_string()
                    },
                ),
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

/// The Widget tab's line before anything was hovered, the iced and egui
/// showcases' too.
pub(crate) const INFO_HINT: &str = "Hover any widget to see what the theme sets on it.";

/// The title Widget Info and the status bar name `info` by: the element's
/// name and state where it is one of docs/showcase-elements.toml, else the
/// widget's own.
pub(crate) fn shown_title(info: &WidgetInfo) -> String {
    info.listed
        .and_then(elements::element)
        .map_or_else(|| info.title(), elements::ShowcaseElement::title)
}

/// The route line of a leaf `info::leaves` has none for:
/// `every_listed_leaf_has_a_route` keeps it off screen.
const NO_ROUTE: &str = "gpui: no route recorded";

/// The approved semibold (R2): the weight of Widget Info's title and
/// section names, in all three showcases.
const SEMIBOLD: gpui::FontWeight = gpui::FontWeight(600.);

/// What Widget Info is drawn in under a native theme, the same in the three
/// showcases (docs/showcase-elements.toml, "The Widget Info format"): every
/// text in `sidebar.font`'s family and size, one line of it tall
/// (`defaults.line_height`), in `sidebar.font.color`, a route in
/// `defaults.muted_color`; `layout.widget_gap` before each row and
/// `layout.section_gap` before each section, none where the layout states
/// none; a swatch framed as `defaults.border` frames a control; the Copy
/// button padded by `button.border`'s sides and rounded by its radius,
/// `button.hover_background` and `active_background` under the pointer.
pub(crate) struct PanelLook {
    family: SharedString,
    size: gpui::Pixels,
    line: gpui::Pixels,
    weight: gpui::FontWeight,
    text: gpui::Hsla,
    pub(crate) muted: gpui::Hsla,
    row_gap: gpui::Pixels,
    section_gap: gpui::Pixels,
    frame: (gpui::Hsla, gpui::Pixels, gpui::Pixels),
    copy_padding: [gpui::Pixels; 4],
    copy_radius: gpui::Pixels,
    copy_hover: Option<gpui::Hsla>,
    copy_active: Option<gpui::Hsla>,
    resolved: serde_json::Value,
    layout: native_theme::theme::LayoutTheme,
    icon_set: Option<native_theme::theme::IconSet>,
}

impl PanelLook {
    /// The look under the installed native theme, with the layout and icon
    /// set `showcase` installed; `None` before `apply` ran.
    fn of(showcase: &WeakEntity<Showcase>, cx: &gpui::App) -> Option<Self> {
        let n = cx.native_theme()?.native(cx)?;
        let r = n.resolved;
        let (layout, icon_set) = showcase
            .upgrade()
            .map(|s| {
                let s = s.read(cx);
                (s.layout.clone(), Some(s.current_icon_set))
            })
            .unwrap_or_default();
        let size = native_theme_gpui::scaled_text_size(r.sidebar.font.size, n.accessibility);
        let side = |v: Option<f32>| v.map_or(gpui::px(0.), gpui::px);
        let p = &r.button.border.padding;
        Some(Self {
            family: native_theme_gpui::font_family(&r.sidebar.font.family),
            size: gpui::px(size),
            line: gpui::px(size * r.defaults.line_height),
            weight: gpui::FontWeight(f32::from(r.sidebar.font.weight)),
            text: stated(r.sidebar.font.color),
            muted: stated(r.defaults.muted_color),
            row_gap: side(layout.widget_gap),
            section_gap: side(layout.section_gap),
            frame: (
                stated(r.defaults.border.color),
                gpui::px(r.defaults.border.line_width),
                gpui::px(r.defaults.border.corner_radius.max(0.)),
            ),
            copy_padding: [side(p.top), side(p.right), side(p.bottom), side(p.left)],
            copy_radius: gpui::px(r.button.border.corner_radius.max(0.)),
            copy_hover: Some(stated(r.button.hover_background)),
            copy_active: r.button.active_background.map(stated),
            resolved: serde_json::to_value(r).unwrap_or_default(),
            layout,
            icon_set,
        })
    }

    /// `text` in the panel's type: `sidebar.font` at its line height.
    pub(crate) fn text(&self, text: gpui::Div) -> gpui::Div {
        text.font_family(self.family.clone())
            .text_size(self.size)
            .line_height(self.line)
            .font_weight(self.weight)
            .text_color(self.text)
            .whitespace_nowrap()
    }
}

/// Widget Info for `element` of docs/showcase-elements.toml, whose widget
/// reported `info`, in the list's form: the title row -- its name and state,
/// Copy at the end -- then the "Theme" section, a row per leaf the list
/// names: a swatch where the value is a colour, `<leaf> <value>`, and under
/// it how gpui applies the leaf (`info::leaves`); then "Not themeable",
/// where the widget's info says what the theme states nothing for. The
/// title, the Copy button, the section name and the first row record where
/// they were laid out. Copy copies the text the panel shows. Also the
/// title.
fn listed_info(
    ui: &Entity<InfoRegistry>,
    look: &PanelLook,
    element: &elements::ShowcaseElement,
    info: &WidgetInfo,
) -> (gpui::Div, SharedString) {
    let title = SharedString::from(element.title());
    let mut copied = format!("{title}\n\nTheme\n");
    let rows: Vec<gpui::AnyElement> = element
        .leaves
        .iter()
        .enumerate()
        .map(|(ix, leaf)| {
            let value = values::leaf_value(leaf, &look.resolved, &look.layout, look.icon_set);
            let route = leaves::how(&element.id, leaf).unwrap_or(NO_ROUTE);
            copied.push_str(&format!("{leaf} {}\n  {route}\n", value.text));
            let first = ix == 0;
            let recorded = |id: &'static str| first.then(|| elements::record(ui, id));
            // Each line its own width, clipped at the row's end.
            let lines = v_flex()
                .flex_1()
                .min_w_0()
                .items_start()
                .child(
                    look.text(div().relative())
                        .max_w_full()
                        .overflow_hidden()
                        .child(format!("{leaf} {}", value.text))
                        .children(recorded("chrome.info.row_1.text")),
                )
                .child(
                    look.text(div().relative())
                        .max_w_full()
                        .overflow_hidden()
                        .text_color(look.muted)
                        .child(route)
                        .children(recorded("chrome.info.row_1.how")),
                );
            let (frame, width, radius) = look.frame;
            let swatch = value.colour.map(|colour| {
                div()
                    .relative()
                    .flex_none()
                    .size(look.line)
                    .bg(colour)
                    .border(width)
                    .border_color(frame)
                    .rounded(radius)
            });
            // The row is the panel's text column; a line longer than it runs
            // on under the panel's clip.
            h_flex()
                .relative()
                .w_full()
                .items_start()
                .gap(look.row_gap)
                .mt(look.row_gap)
                .when_some(swatch, |row, swatch| {
                    row.child(swatch.children(first.then(|| {
                        // Out over the swatch's frame: an absolute child is
                        // laid out inside it.
                        div()
                            .absolute()
                            .top(-width)
                            .left(-width)
                            .right(-width)
                            .bottom(-width)
                            .child(elements::record(ui, "chrome.info.row_1.swatch"))
                    })))
                })
                .child(lines)
                .children(recorded("chrome.info.row_1"))
                .into_any_element()
        })
        .collect();
    let notes: Vec<String> = info
        .not_themeable
        .iter()
        .map(|note| format!("{}: {}", note.what, note.text))
        .collect();
    if !notes.is_empty() {
        copied.push_str("\nNot themeable\n");
        for note in &notes {
            copied.push_str(&format!("{note}\n"));
        }
    }
    let not_themeable: Vec<gpui::AnyElement> = notes
        .into_iter()
        .map(|note| {
            look.text(div())
                .text_color(look.muted)
                .mt(look.row_gap)
                .child(note)
                .into_any_element()
        })
        .collect();
    let copy_text = copied;
    let [top, right, bottom, left] = look.copy_padding;
    let copy = div()
        .id("inspector-copy")
        .relative()
        .cursor_pointer()
        .pt(top)
        .pr(right)
        .pb(bottom)
        .pl(left)
        .rounded(look.copy_radius)
        .when_some(look.copy_hover, |copy, hover| {
            copy.hover(move |style| style.bg(hover))
        })
        .when_some(look.copy_active, |copy, active| {
            copy.active(move |style| style.bg(active))
        })
        .child(look.text(div()).child("Copy"))
        .child(elements::record(ui, "chrome.info.copy"))
        .on_click(move |_, _, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(copy_text.clone()))
        });
    // Every text its own width, the title row the panel's.
    let panel = v_flex()
        .w_full()
        .items_start()
        .overflow_hidden()
        .child(
            h_flex()
                .w_full()
                .justify_between()
                .items_center()
                .child(
                    look.text(div().relative())
                        .font_weight(SEMIBOLD)
                        .debug_selector(|| INSPECTOR_TITLE.into())
                        .child(title.clone())
                        .child(elements::record(ui, "chrome.info.title")),
                )
                .child(probe(INSPECTOR_COPY, copy)),
        )
        .child(
            look.text(div().relative())
                .font_weight(SEMIBOLD)
                .mt(look.section_gap)
                .child("Theme")
                .child(elements::record(ui, "chrome.info.section.theme")),
        )
        .children(rows)
        .when(!not_themeable.is_empty(), |panel| {
            panel
                .child(
                    look.text(div())
                        .font_weight(SEMIBOLD)
                        .mt(look.section_gap)
                        .child("Not themeable"),
                )
                .children(not_themeable)
        });
    (panel, title)
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
        let margin = demo::side_panel_content_margin(cx, geometry::container_margin(&layout));
        let tabs = demo::tab_bar(
            &self.ui,
            cx,
            TabBarKind::Inspector,
            margin,
            InspectorTab::ALL.map(|tab| (tab.label(), tab.tab())),
            self.tab.index(),
            None,
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
                // The bar is drawn over the right edge of the scroll area, as
                // on the content panel (`gutter_scroll`).
                crate::support::gutter_scroll(
                    "inspector-scroll",
                    &self.scroll,
                    div().child(with_padding(v_flex(), margin).child(body)),
                    {
                        let this = cx.entity().downgrade();
                        move |cx: &mut gpui::App| {
                            this.update(cx, |_, cx| cx.notify()).ok();
                        }
                    },
                    cx,
                )
                .flex_1()
                .min_h_0()
                // The room below the tabs, its scroll bar's included.
                .child(elements::record(&self.ui, "chrome.side_panel.inspector")),
            )
    }
}
