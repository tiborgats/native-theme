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
use native_theme_showcase_support::info::ListedInfo;
use native_theme_showcase_support::status::{DrawnFamily, defined_size};
use native_theme_showcase_support::values;
use std::borrow::Cow;
use std::sync::Arc;

use crate::app::Showcase;
use crate::demo::TabBarKind;
use crate::elements;
use crate::info::{InfoRegistry, Note, WidgetInfo, as_rgba, leaves, stated};
use crate::support::{NativeStyled, frame_drawer, with_gap, with_padding};
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
    /// Where the inspector reads the installed theme's fonts and layout, and
    /// the look the Widget tab is drawn in (`Showcase::panel_look`).
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
        let showcase = self.showcase.upgrade();
        let look = showcase
            .as_ref()
            .and_then(|showcase| showcase.read(cx).panel_look.as_ref());
        let shown = self.ui.read(cx).shown();
        let listed = shown.and_then(|info| {
            let element = elements::element(info.listed?)?;
            Some((element, info))
        });
        if let (Some(look), Some((element, info))) = (look, listed) {
            let (panel, title) = listed_info(&self.ui, look, element, info);
            self.title_drawn = Some(title);
            return panel;
        }
        let muted = cx.theme().muted_foreground;
        let (title, copied, body) = match shown {
            Some(info) => (info.title(), info.to_text(), info_sections(info, gap, cx)),
            None => {
                self.title_drawn = None;
                // Clipped to the panel's width, as the other rows are.
                let hint = div()
                    .relative()
                    .w_full()
                    .overflow_hidden()
                    .child(INFO_HINT)
                    .child(elements::record(&self.ui, "chrome.info.hint"));
                return v_flex().items_start().child(match look {
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
                        demo::labelled(
                            &self.ui,
                            cx,
                            Button::new("inspector-copy"),
                            "inspector-copy",
                            "Copy",
                        )
                        .small()
                        .custom(variants::ghost_button(cx))
                        .on_click(move |_, _, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(copied.clone()))
                        }),
                    )),
            )
            // A claim's value is the ThemeColor field it names, and upstream
            // paints many widgets from `Theme::tokens` instead, which a theme
            // can set apart from its fields (theme/schema.rs:1018, and the
            // test at :1308 asserting a pair that differs). Said only before
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
        let theme = cx.theme();
        let rows = vec![
            ("radius", format!("{}px", theme.radius.as_f32())),
            ("radius_lg", format!("{}px", theme.radius_lg.as_f32())),
            ("shadow", theme.shadow.to_string()),
            ("scrollbar_mode", format!("{:?}", theme.scrollbar_mode)),
        ];
        let font_rows = self
            .showcase
            .upgrade()
            .map(|showcase| font_rows(showcase.read(cx), cx))
            .unwrap_or_default();

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

/// The Theme tab's Fonts section: the installed theme's `defaults.font` and
/// `defaults.mono_font` of `app`, each family by the family gpui draws
/// ([`DrawnFamilies::label`] of the stated family and `Theme::font_family`
/// or `mono_font_family`, the connector's `font_family` of it, once a native
/// theme is installed; the placeholder alone before that), each size in the
/// unit the platform stated, like the Widget Info: these rows report the
/// theme's own definition, not the pixel value gpui lays out with.
pub(crate) fn font_rows(app: &Showcase, cx: &gpui::App) -> Vec<(&'static str, String)> {
    let (font, mono) = (&app.original_font, &app.original_mono_font);
    let theme = cx.theme();
    let families = DrawnFamilies::installed(app, cx);
    let label = |stated: &str, asked: &SharedString| {
        families
            .as_deref()
            .map_or_else(|| stated.to_string(), |f| f.label(stated, asked))
    };
    vec![
        ("font_family", label(&font.family, &theme.font_family)),
        ("font_size", defined_size(font)),
        (
            "mono_font_family",
            label(&mono.family, &theme.mono_font_family),
        ),
        ("mono_font_size", defined_size(mono)),
    ]
}

/// What gpui draws each family it is asked for under the installed theme
/// in, as the status bar, the Theme tab and Widget Info name it (the shared
/// crate's `status::DrawnFamily`): the family gpui's text system resolves it
/// to, which is the family itself where the text system finds a face of it
/// and otherwise the first family of gpui's fallback stack it finds
/// (`TextSystem::resolve_font`: gpui-pre 0.3.9 `src/text_system.rs:370-388`
/// over the stack of `:255-266`; gpui-fast 0.1.10 `src/text_system.rs:372-374`,
/// `src/fast/text.rs:990-1058`, over `:256-267`), read back from the face's
/// id (`TextSystem::get_font_for_id`, gpui-pre `:354-362`, gpui-fast
/// `:356-364`): each face gpui's platform text systems load gets an id of its
/// own for each family it is asked for under (gpui-pre-wgpu 0.3.9 and
/// gpui-fast-wgpu 0.1.10 `src/cosmic_text_system.rs:44-47`, `:400-443`;
/// gpui-pre-apple `src/text_system.rs:382`, gpui-fast-apple `:385`;
/// gpui-pre-windows and gpui-fast-windows `src/direct_write.rs:332`), so the
/// id names that family.
/// Built once an install, with the panel's look (`PanelLook::of`), as the
/// text system is read through the app.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DrawnFamilies(Vec<(SharedString, Option<SharedString>)>);

impl DrawnFamilies {
    /// The family the text system of `cx` resolves each of `asked` to, each
    /// asked once. A text system that lists no font
    /// (`TextSystem::all_font_names`, gpui-pre `:284-289`, gpui-fast
    /// `:285-290`) is not asked, and the answer is `None`: the test
    /// platform's `NoopTextSystem` lists none and resolves every family to
    /// one face, whose id then names no family of its own (gpui-pre
    /// `src/platform.rs:1547-1553`). `resolve_font` panics where its text
    /// system finds no face of the family nor of any family of the stack, as
    /// gpui's own layout of that family's text, which goes through it
    /// (gpui-pre `src/text_system.rs:797`, `:911`), already does.
    pub(crate) fn of(asked: impl IntoIterator<Item = SharedString>, cx: &gpui::App) -> Self {
        let text_system = cx.text_system();
        let listed = !text_system.all_font_names().is_empty();
        let mut drawn: Vec<(SharedString, Option<SharedString>)> = Vec::new();
        for family in asked {
            if drawn.iter().any(|(known, _)| *known == family) {
                continue;
            }
            let resolved = if listed {
                let id = text_system.resolve_font(&gpui::font(family.clone()));
                text_system.get_font_for_id(id).map(|font| font.family)
            } else {
                None
            };
            drawn.push((family, resolved));
        }
        Self(drawn)
    }

    /// The installed theme's: kept with the panel's look
    /// (`Showcase::panel_look`), or, where the frame that keeps it has not
    /// run since the install, built now for `Theme::font_family` and
    /// `mono_font_family`; `None` where no native theme is installed.
    pub(crate) fn installed<'a>(app: &'a Showcase, cx: &gpui::App) -> Option<Cow<'a, Self>> {
        if let Some(look) = &app.panel_look {
            return Some(Cow::Borrowed(&look.families));
        }
        cx.native_theme()?;
        let theme = cx.theme();
        Some(Cow::Owned(Self::of(
            [theme.font_family.clone(), theme.mono_font_family.clone()],
            cx,
        )))
    }

    /// What gpui draws text of a theme font whose family is `stated` in,
    /// gpui having been asked for `asked` (the connector's `font_family` of
    /// it): where the text system resolves `asked` to another family, the
    /// first family of gpui's fallback stack it finds, with `asked` named
    /// where it is not `stated`; else `asked` itself
    /// (`DrawnFamily::Named`, the stated name alone where `asked` is it), and
    /// so where the text system was not asked or does not say.
    pub(crate) fn drawn<'a>(&'a self, stated: &str, asked: &'a str) -> DrawnFamily<'a> {
        let resolved = self
            .0
            .iter()
            .find(|(known, _)| known.as_ref() == asked)
            .and_then(|(_, resolved)| resolved.as_deref());
        match resolved {
            Some(family) if family != asked => DrawnFamily::FallbackStack {
                toolkit: "gpui",
                asked: (asked != stated).then_some(asked),
                family,
            },
            _ => DrawnFamily::from(Some(asked)),
        }
    }

    /// `stated` as a label that says what gpui draws ([`Self::drawn`]).
    pub(crate) fn label(&self, stated: &str, asked: &str) -> String {
        self.drawn(stated, asked).label(stated)
    }
}

/// Every font family `value`, a resolved theme's JSON, states: the `family`
/// string of each object it holds, each once.
fn stated_families(value: &serde_json::Value, families: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(family) = map.get("family").and_then(serde_json::Value::as_str)
                && !families.iter().any(|known| known == family)
            {
                families.push(family.to_string());
            }
            for value in map.values() {
                stated_families(value, families);
            }
        }
        serde_json::Value::Array(values) => {
            for value in values {
                stated_families(value, families);
            }
        }
        _ => {}
    }
}

/// Who draws the frame of a window granted server-side decorations, on the
/// platform `os` names (`std::env::consts::OS`): the window manager on
/// Linux, the system on macOS and Windows, as the iced showcase's
/// `window_decorations` says.
pub(crate) fn window_decorations(os: &str) -> String {
    format!("{} draws the frame", frame_drawer(os))
}

/// The frame of a window granted server-side decorations, on the platform
/// `os` names (`std::env::consts::OS`), as the iced showcase's
/// `window_frame` names it. gpui's default options keep the system's title
/// bar on macOS and Windows (`window_options`); on Linux the window manager
/// draws the frame where it grants server-side decorations, and where a
/// Wayland compositor grants none the window is client-side, which
/// `window_rows`' other arm says.
pub(crate) fn window_frame(os: &str) -> &'static str {
    match os {
        "macos" => "the system's title bar: the window's own, with its traffic-light controls",
        "windows" => "the system's title bar: the window's own caption, controls and border",
        _ => {
            "the window manager's (KWin: Breeze's title bar, controls, corners and shadow; \
             Mutter under X11: the desktop's decoration)"
        }
    }
}

/// The resize band of a window granted server-side decorations, on the
/// platform `os` names (`std::env::consts::OS`): its owner's, the one that
/// draws the frame ([`frame_drawer`]), since the WindowBorder lays none.
pub(crate) fn resize_band(os: &str) -> String {
    format!(
        "{}'s: the WindowBorder lays none (window_border.rs, WindowBorder)",
        frame_drawer(os)
    )
}

/// The Theme tab's Window section (spec S8): what draws the window's frame
/// under `decorations`, the decorations the window was granted, with
/// the client inset and the frame's insets the window reports. Under
/// server-side decorations the platform draws the frame (the system on macOS
/// and Windows, the window manager on Linux); under client-side ones,
/// gpui-component's client frame and the window's TitleBar do.
pub(crate) fn window_rows(
    decorations: gpui::Decorations,
    inset: Option<gpui::Pixels>,
    paddings: gpui::Edges<gpui::Pixels>,
) -> Vec<(&'static str, String)> {
    // gpui-component's `WindowState` root plugin wraps everything the window
    // holds in `window_border()` (root.rs:552-560, `WindowState::decorate`).
    // Its client-side arm alone sets the client inset (window_border.rs:155),
    // lays hit zones and draws a frame; under server-side decorations it
    // hands back the content itself (window_border.rs:134-139).
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
    match decorations {
        gpui::Decorations::Server => vec![
            (
                "decorations",
                format!("server-side: {}", window_decorations(std::env::consts::OS)),
            ),
            (
                "frame",
                format!(
                    "{}. The WindowBorder gpui-component wraps the window in draws nothing (window_border.rs, WindowBorder)",
                    window_frame(std::env::consts::OS)
                ),
            ),
            ("client inset", inset),
            ("frame insets", frame_insets),
            ("resize band", resize_band(std::env::consts::OS)),
        ],
        gpui::Decorations::Client { tiling } => vec![
            (
                "decorations",
                format!("client-side, tiled {tiling:?}: the window manager leaves the frame to the application"),
            ),
            (
                "frame",
                "the client frame gpui-component draws: a WindowBorder, which its WindowState root plugin wraps around everything the window holds (root.rs, WindowState::decorate), with the window's TitleBar at its top"
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
pub(crate) const SWATCH_SIZE: gpui::Pixels = gpui::px(16.);

/// The gap between a swatch's square and its label, gpui's `gap_2`: the
/// showcase's own choice, as [`SWATCH_SIZE`] is. The side panel's info,
/// which the inspector's content reports through, says both.
pub(crate) const SWATCH_GAP: gpui::Rems = gpui::rems(0.5);

/// A colour swatch labelled `name` and the colour's hex value, its square
/// in `frame`, the showcase's frame, built once by the caller.
fn swatch(name: &str, color: gpui::Hsla, frame: &gpui::StyleRefinement) -> gpui::Div {
    let label = SharedString::from(format!("{name} {}", as_rgba(color)));
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
/// filled with `button.hover_background` and lettered in
/// `hover_text_color` under the pointer, `active_background` (where stated)
/// and `active_text_color` while pressed.
/// With it, what the rows' values are read from: the resolved theme's JSON,
/// and the layout, icon set and icon theme the showcase installed, which the
/// JSON does not hold (`values::leaf_value`).
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
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
    /// The Copy box's fill and text colour under the pointer.
    copy_hover: (gpui::Hsla, gpui::Hsla),
    /// The Copy box's fill, where stated, and text colour while pressed.
    copy_active: (Option<gpui::Hsla>, gpui::Hsla),
    resolved: serde_json::Value,
    layout: native_theme::theme::LayoutTheme,
    icon_set: native_theme::theme::IconSet,
    icon_theme: Option<String>,
    /// What gpui draws each family of `resolved` in, and `Theme::font_family`
    /// and `mono_font_family`.
    families: DrawnFamilies,
}

impl PanelLook {
    /// The look under the installed native theme, with the layout, icon set
    /// and icon theme `showcase` installed; `None` before `apply` ran. The
    /// showcase keeps it from one install to the next
    /// (`Showcase::panel_look`).
    pub(crate) fn of(showcase: &Showcase, cx: &gpui::App) -> Option<Self> {
        let n = cx.native()?;
        let r = n.resolved;
        let layout = showcase.layout.clone();
        let size = native_theme_gpui::scaled_text_size(r.sidebar.font.size, n.accessibility);
        let side = |v: Option<f32>| v.map_or(gpui::px(0.), gpui::px);
        let p = &r.button.border.padding;
        let resolved = theme_json(r);
        let mut named = Vec::new();
        stated_families(&resolved, &mut named);
        let theme = cx.theme();
        let asked = named
            .iter()
            .map(|family| native_theme_gpui::font_family(&Arc::from(family.as_str())))
            .chain([theme.font_family.clone(), theme.mono_font_family.clone()]);
        let families = DrawnFamilies::of(asked, cx);
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
            copy_hover: (
                stated(r.button.hover_background),
                stated(r.button.hover_text_color),
            ),
            copy_active: (
                r.button.active_background.map(stated),
                stated(r.button.active_text_color),
            ),
            resolved,
            layout,
            icon_set: showcase.current_icon_set,
            icon_theme: showcase.current_icon_theme.clone(),
            families,
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

/// The resolved theme `r` as JSON (`values::theme_json`), which the panel's
/// rows read their values from. Counted in the self-tests
/// (`THEME_SERIALISATIONS`).
fn theme_json(r: &native_theme::theme::ResolvedTheme) -> serde_json::Value {
    #[cfg(test)]
    THEME_SERIALISATIONS.with(|count| count.set(count.get().saturating_add(1)));
    values::theme_json(r).unwrap_or_default()
}

#[cfg(test)]
thread_local! {
    /// How often the inspector serialised a resolved theme ([`theme_json`])
    /// on this thread; a test reads how much an action adds to it.
    pub(crate) static THEME_SERIALISATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

/// The panel's text for `element` of docs/showcase-elements.toml, whose
/// widget reported `info`, under `look`: a row per leaf the list names, with
/// its value and how gpui applies it (`info::leaves`), each font's family by
/// the family gpui draws ([`DrawnFamilies::label`] of the stated family and
/// the connector's `font_family` of it, as the Theme tab's [`font_rows`]),
/// and the widget's "Not themeable" notes.
pub(crate) fn listed(
    look: &PanelLook,
    element: &elements::ShowcaseElement,
    info: &WidgetInfo,
) -> ListedInfo {
    ListedInfo::new(
        element,
        &look.resolved,
        &look.layout,
        Some(look.icon_set),
        look.icon_theme.as_deref(),
        |_, font| {
            look.families
                .label(&font.family, &native_theme_gpui::font_family(&font.family))
        },
        |leaf| {
            leaves::how(&element.id, leaf)
                .unwrap_or(NO_ROUTE)
                .to_string()
        },
        info.not_themeable
            .iter()
            .map(|note| format!("{}: {}", note.what, note.text))
            .collect(),
    )
}

/// Widget Info for `element` of docs/showcase-elements.toml, whose widget
/// reported `info`, in the list's form (the shared crate's `ListedInfo`): the
/// title row -- its name and state, Copy at the end -- then the "Theme"
/// section, a row per leaf the list names: a swatch where the value is a
/// colour, `<leaf> <value>`, and under it how gpui applies the leaf
/// (`info::leaves`); then "Not themeable", where the widget's info says what
/// the theme states nothing for. The title, the Copy button, the section
/// name and the first row record where they were laid out. Copy copies the
/// text the panel shows (`ListedInfo::copy_text`). Also the title.
fn listed_info(
    ui: &Entity<InfoRegistry>,
    look: &PanelLook,
    element: &elements::ShowcaseElement,
    info: &WidgetInfo,
) -> (gpui::Div, SharedString) {
    let listed = listed(look, element, info);
    let title = SharedString::from(listed.title.clone());
    let rows: Vec<gpui::AnyElement> = listed
        .rows
        .iter()
        .enumerate()
        .map(|(ix, row)| {
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
                        .child(format!("{} {}", row.leaf, row.value))
                        .children(recorded("chrome.info.row_1.text")),
                )
                .child(
                    look.text(div().relative())
                        .max_w_full()
                        .overflow_hidden()
                        .text_color(look.muted)
                        .child(row.how.clone())
                        .children(recorded("chrome.info.row_1.how")),
                );
            let (frame, width, radius) = look.frame;
            let swatch = row.colour.map(|colour| {
                div()
                    .relative()
                    .flex_none()
                    .size(look.line)
                    .bg(stated(colour))
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
                        let out = gpui::px(-f32::from(width));
                        div()
                            .absolute()
                            .top(out)
                            .left(out)
                            .right(out)
                            .bottom(out)
                            .child(elements::record(ui, "chrome.info.row_1.swatch"))
                    })))
                })
                .child(lines)
                .children(recorded("chrome.info.row_1"))
                .into_any_element()
        })
        .collect();
    let not_themeable: Vec<gpui::AnyElement> = listed
        .not_themeable
        .iter()
        .map(|note| {
            look.text(div())
                .text_color(look.muted)
                .mt(look.row_gap)
                .child(note.clone())
                .into_any_element()
        })
        .collect();
    let copy_text = listed.copy_text();
    let [top, right, bottom, left] = look.copy_padding;
    let ((hover_fill, hover_text), (active_fill, active_text)) =
        (look.copy_hover, look.copy_active);
    // The label is the box's own text, so the box's hover and pressed text
    // colours letter it; a child setting its own colour would keep it.
    let copy = look
        .text(div())
        .id("inspector-copy")
        .relative()
        .cursor_pointer()
        .pt(top)
        .pr(right)
        .pb(bottom)
        .pl(left)
        .rounded(look.copy_radius)
        .hover(move |style| style.bg(hover_fill).text_color(hover_text))
        .active(move |style| {
            let style = style.text_color(active_text);
            match active_fill {
                Some(fill) => style.bg(fill),
                None => style,
            }
        })
        .child("Copy")
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
        let (gap, container_margin) = self
            .showcase
            .upgrade()
            .map(|showcase| {
                let layout = &showcase.read(cx).layout;
                (
                    geometry::widget_gap(layout),
                    geometry::container_margin(layout),
                )
            })
            .unwrap_or_default();
        let margin = demo::side_panel_content_margin(cx, container_margin);
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
