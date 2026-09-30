//! native-theme-iced — comprehensive widget showcase and designer reference.
//!
//! Demonstrates every styled iced widget with live theme switching across all
//! bundled `native-theme` presets (system / light / dark), every `native-theme`
//! metric helper, an icon gallery with icon-theme switching and source
//! tracking, and a theme map showing all palette colors. The chrome is the
//! gpui and egui showcases', element by element, drawn from the theme data:
//! a menu bar, a toolbar, a side panel with the theme settings over a
//! hover-driven inspector, a splitter, the page tabs, a status bar, and the
//! command palette, Preferences and About dialogs.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-iced --example showcase-iced
//! ```
//!
//! # What to look for
//!
//! - Sidebar switches theme presets, color modes, and icon sets at runtime —
//!   no restart needed. Watch how every widget re-themes together.
//! - Hover the Widget Info inspector to see which `ResolvedTheme` fields
//!   drive the widget currently under the mouse.
//! - The Theme Map tab exposes iced's 6-field `Palette` plus `Extended`
//!   palette values produced by `native-theme-iced`.
//! - The Icons tab demonstrates icon loading across Material, Lucide, and
//!   freedesktop sets, plus animated spinner playback with `Rotation::Floating`.
//!
//! # How this file is organised
//!
//! The source is split into section-divider blocks (`// ───────`) — one per
//! widget category, tab, or view. Search for the dividers to jump between
//! sections.

use iced::advanced::graphics::text::cosmic_text::{self, Fallback, fontdb};
use iced::widget::{
    button, canvas, center, checkbox, column, combo_box, container, grid, markdown, mouse_area,
    opaque, pane_grid, pick_list, progress_bar, qr_code, radio, rich_text, row, rule, scrollable,
    slider, space, span, stack, svg, table, text, text_editor, text_input, toggler, tooltip,
    vertical_slider,
};
use iced::{Color, Element, Fill, Length, Padding, Theme};
// The window's menu bar is iced_aw's, with or without the connector's
// `iced_aw` feature: the showcase depends on iced_aw's menu itself.
use iced_aw::menu::{Item, Menu, MenuBar};
#[cfg(feature = "iced_aw")]
use iced_aw::sidebar::Sidebar;
#[cfg(feature = "iced_aw")]
use iced_aw::{Card, ContextMenu, SelectionList, Spinner, TabBar, TabLabel, Tabs};

use iced::Subscription;
use native_theme::detect::prefers_reduced_motion;
use native_theme::icons::{
    FreedesktopLoader, IconSetChoice, LucideLoader, MaterialLoader, SegoeIconsLoader,
    SfSymbolsLoader, default_icon_choice, list_freedesktop_themes, load_icon_indicator,
};
use native_theme::theme::{
    AnimatedIcon, ArrowSide, IconData, IconRole, IconSet, LayoutTheme, ResolvedFontSpec,
    ResolvedTextScale, ResolvedTextScaleEntry, ResolvedTheme, TransformAnimation,
};
use native_theme_iced::icons::{
    AnimatedSvgHandles, animated_frames_to_svg_handles, spin_rotation_radians, to_svg_handle,
};
use native_theme_iced::palette::to_color;
use native_theme_iced::styles;
use native_theme_iced::{
    AccessibilityPreferences, at_least, combo_box_padding, control_line_height, scaled_text_size,
};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// UI spacing scale (local constants, not a theme property)
// ---------------------------------------------------------------------------

/// Fixed spacing scale for showcase UI layout.
///
/// These are UI constants used by this example application for layout spacing.
/// They are NOT theme-driven values -- they are design choices for this showcase.
struct Spacing {
    xxs: f32,
    xs: f32,
    s: f32,
    m: f32,
    l: f32,
    xl: f32,
}

impl Spacing {
    const fn new() -> Self {
        Self {
            xxs: 2.0,
            xs: 4.0,
            s: 8.0,
            m: 12.0,
            l: 16.0,
            xl: 24.0,
        }
    }
}

/// Showcase UI spacing constants.
const SP: Spacing = Spacing::new();

/// The window `main` opens, and the viewport `every_tab_renders` lays out in:
/// 1280 × 720 logical pixels, the maintainer's default for every showcase, as
/// the egui and gpui showcases' `WINDOW_SIZE`.
const WINDOW_SIZE: (f32, f32) = (1280.0, 720.0);

/// Tags a widget so the self-tests can find it, and does nothing otherwise.
///
/// `iced_selector` sees only the widgets that implement `Widget::operate`, and
/// `slider`, `radio`, `toggler`, `pick_list`, `combo_box` and `text_editor`
/// implement none, so no selector can reach them. `container` is the one
/// widget in the showcase's set that carries a `widget::Id` and reports it
/// with its own bounds (`container.rs:109`, `:280-283`), which is what
/// `iced_test`'s `id` selector matches.
///
/// Outside `cargo test` the wrapper is not built at all, so the interface the
/// showcase draws -- and the screenshots taken from it -- are exactly what
/// they were.
#[cfg(test)]
fn probe<'a>(
    id: &'static str,
    width: Length,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content).id(id).width(width).into()
}

#[cfg(not(test))]
fn probe<'a>(
    _id: &'static str,
    _width: Length,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    content.into()
}

/// The [`probe`] tags, shared by the render code and by
/// `interactive_controls_respond`.
mod probes {
    pub const THEME: &str = "probe-theme";
    pub const COLOR_MODE: &str = "probe-color-mode";
    pub const ICON_THEME: &str = "probe-icon-theme";
    pub const SIDE_PANEL: &str = "probe-side-panel";
    pub const TEXT_EDITOR: &str = "probe-text-editor";
    pub const RADIO_APPLE: &str = "probe-radio-apple";
    pub const RADIO_BANANA: &str = "probe-radio-banana";
    pub const RADIO_CHERRY: &str = "probe-radio-cherry";
    pub const BASIC_RADIO_B: &str = "probe-basic-radio-b";
    pub const BASIC_BUTTON: &str = "probe-basic-button";
    pub const BASIC_TEXT_INPUT: &str = "probe-basic-text-input";
    pub const BASIC_PICK_LIST: &str = "probe-basic-pick-list";
    pub const BASIC_TEXT_AREA: &str = "probe-basic-text-area";
    pub const BASIC_SEGMENTED: &str = "probe-basic-segmented";
    pub const BASIC_EXPANDER: &str = "probe-basic-expander";
    pub const BASIC_LIST: &str = "probe-basic-list";
    pub const BASIC_SPINNER: &str = "probe-basic-spinner";
    pub const BASIC_SWITCH_OFF: &str = "probe-basic-switch-off";
    pub const BASIC_SWITCH_ON: &str = "probe-basic-switch-on";
    pub const BASIC_SWITCH_DISABLED: &str = "probe-basic-switch-disabled";
    pub const BASIC_CARD: &str = "probe-basic-card";
    pub const TOGGLER: &str = "probe-toggler";
    pub const PICK_LIST: &str = "probe-pick-list";
    pub const COMBO_BOX: &str = "probe-combo-box";
    pub const SLIDER: &str = "probe-slider";
    pub const VERTICAL_SLIDER: &str = "probe-vertical-slider";
    #[cfg(feature = "iced_aw")]
    pub const SELECTION_LIST: &str = "probe-selection-list";
}

/// The `widget::Id` of the single-line `text_input`.
///
/// `text_input` carries an id of its own (`text_input.rs:157`), so it needs no
/// [`probe`]; the two inputs share one value, and this one is the one the
/// self-tests type into.
const TEXT_INPUT_ID: &str = "showcase-text-input";

/// The `widget::Id`s of the page and inspector tab strips' `scrollable`s,
/// which report their own bounds and their content's to a selector
/// (`scrollable.rs:548-560`).
const TAB_STRIP_ID: &str = "showcase-tab-strip";
const INSPECTOR_TABS_ID: &str = "showcase-inspector-tabs";

/// The four layout distances the platform itself states.
///
/// `LayoutTheme` is the one theme struct that is not per-variant, so it lives
/// on the native `Theme` and on `SystemTheme` rather than on `ResolvedTheme`
/// (`native-theme/src/model/widgets/mod.rs:884`). The showcase therefore keeps
/// a copy of it beside the resolved theme and refreshes it whenever the theme
/// is rebuilt.
///
/// Every field of `LayoutTheme` is an `Option`: `None` is the platform saying
/// it states no such distance, and nothing is invented for it -- the
/// showcase's own [`Spacing`] constant stands in, and the inspector's Theme
/// tab says which of the two is on screen.
struct Gaps {
    /// Space between adjacent widgets, `layout.widget_gap`.
    widget: f32,
    /// Padding inside a container, `layout.container_margin`.
    container: f32,
    /// Padding inside the main window, `layout.window_margin`.
    window: f32,
    /// Space between major content sections, `layout.section_gap`.
    section: f32,
}

impl Gaps {
    fn from_layout(layout: &LayoutTheme) -> Self {
        Self {
            widget: layout.widget_gap.unwrap_or(SP.s),
            container: layout.container_margin.unwrap_or(SP.l),
            window: layout.window_margin.unwrap_or(SP.l),
            section: layout.section_gap.unwrap_or(SP.xl),
        }
    }
}

/// How one layout distance reads in the inspector's Theme tab: the
/// platform's value, or the showcase constant that stood in for it.
fn layout_value(stated: Option<f32>, fallback: f32) -> String {
    match stated {
        Some(v) => format!("{v:.0}px"),
        None => format!("{fallback:.0}px (showcase)"),
    }
}

// ---------------------------------------------------------------------------
// The shared element list (docs/showcase-elements.toml)
// ---------------------------------------------------------------------------

/// One element of `docs/showcase-elements.toml`, the list of what the gpui,
/// iced and egui showcases all draw. Widget Info takes an element's title and
/// rows from it, and the layout dump (`--dump-layout`) its ids.
#[derive(Debug, Clone)]
struct ShowcaseElement {
    id: String,
    name: String,
    /// Read by the self-tests, which check the dump against the list.
    #[cfg_attr(not(test), allow(dead_code))]
    parent: String,
    leaves: Vec<String>,
    states: Vec<String>,
    part: bool,
    /// Read by the self-tests: an element on screen only while this holds.
    #[cfg_attr(not(test), allow(dead_code))]
    when: Option<String>,
}

/// The list, as the three showcases read it: at build time, so the showcase
/// needs no file beside it when it runs.
const SHOWCASE_ELEMENTS: &str = include_str!("../../../docs/showcase-elements.toml");

/// `docs/showcase-elements.toml` parsed into its elements, walked by hand as
/// the list's header shows.
fn showcase_elements() -> Result<Vec<ShowcaseElement>, String> {
    let table: toml::Table = toml::from_str(SHOWCASE_ELEMENTS).map_err(|e| e.to_string())?;
    let rows = table
        .get("element")
        .and_then(toml::Value::as_array)
        .ok_or("docs/showcase-elements.toml: no [[element]] entries")?;
    rows.iter()
        .map(|row| {
            let string = |key: &str| {
                row.get(key)
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| format!("an element has no `{key}`"))
            };
            let list = |key: &str| {
                row.get(key)
                    .and_then(toml::Value::as_array)
                    .ok_or_else(|| format!("an element has no `{key}`"))?
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .map(str::to_owned)
                            .ok_or_else(|| format!("`{key}` holds a value that is not a string"))
                    })
                    .collect::<Result<Vec<_>, _>>()
            };
            Ok(ShowcaseElement {
                id: string("id")?,
                name: string("name")?,
                parent: string("parent")?,
                leaves: list("leaves")?,
                states: list("states")?,
                part: row
                    .get("part")
                    .and_then(toml::Value::as_bool)
                    .unwrap_or(false),
                when: row
                    .get("when")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
            })
        })
        .collect()
}

/// The list, parsed once. `main` stops the showcase where it does not parse,
/// so an empty list is only ever seen by a test that broke it.
static ELEMENTS: OnceLock<Result<Vec<ShowcaseElement>, String>> = OnceLock::new();

fn elements() -> &'static [ShowcaseElement] {
    match ELEMENTS.get_or_init(showcase_elements) {
        Ok(list) => list,
        Err(_) => &[],
    }
}

/// The element of the list with this id.
fn listed(id: &str) -> Option<&'static ShowcaseElement> {
    elements().iter().find(|element| element.id == id)
}

/// The id of the list's part `<id>.<part>`, where the list has one.
fn listed_part(id: &str, part: &str) -> Option<&'static str> {
    listed(&format!("{id}.{part}")).map(|element| element.id.as_str())
}

/// The listed elements that are layout boxes rather than something drawn to
/// be looked at: the pointer over the space between controls keeps what
/// Widget Info shows, as it does over a part of the page no element covers.
const LAYOUT_BOXES: [&str; 8] = [
    "chrome.window",
    "chrome.content",
    "basic.page",
    "basic.column_1",
    "basic.column_2",
    "basic.column_3",
    "basic.column_4",
    "basic.column_5",
];

/// Whether the pointer over `id` makes Widget Info show it: every listed
/// element but a part (hovered as its control, which holds it), a layout box
/// ([`LAYOUT_BOXES`]), and Widget Info itself, whose own lines would replace
/// what it shows on the way to its Copy button.
fn hover_target(id: &str) -> bool {
    listed(id).is_some_and(|element| !element.part)
        && !LAYOUT_BOXES.contains(&id)
        && id != "chrome.side_panel.inspector"
        && !id.starts_with("chrome.info.")
}

// ---------------------------------------------------------------------------
// Element tags: what the layout dump records, and what the pointer hovers
// ---------------------------------------------------------------------------

/// Where a part of a tagged element lies, found from the element's layout.
type PartRect<'a> = Box<dyn Fn(iced::advanced::Layout<'_>) -> Option<iced::Rectangle> + 'a>;

/// `content`, one element of the list: while it is drawn its rectangle, and
/// its parts', are recorded under their ids for the layout dump; while the
/// pointer is over it, it is the hover target Widget Info shows, unless a
/// smaller one under the pointer is ([`hover_target`]).
///
/// The wrapper is invisible to the widget tree: it hands its content the
/// tree, the layout and every event as they are, so wrapping a widget keeps
/// its state and its place. The one element tagged as the `root` (the
/// window) starts each frame's record and reports the hover target once the
/// whole tree has seen a pointer move.
struct Tagged<'a> {
    id: &'static str,
    content: Element<'a, Message>,
    parts: Vec<(&'static str, PartRect<'a>)>,
    texts: Vec<(&'static str, MeasuredText, PartOrigin<'a>)>,
    root: bool,
    loose: bool,
    /// The element that clips this one, where one does: the rectangle is
    /// recorded as the part of it inside that element's.
    within: Option<&'static str>,
}

/// The box a measured text part sits in, found from the element's layout.
type PartOrigin<'a> = Box<dyn Fn(iced::advanced::Layout<'_>) -> Option<iced::Rectangle> + 'a>;

/// A text a widget draws inside itself, which the layout dump records at its
/// own extent -- the list's "a label's is its text box" -- measured as iced
/// shapes it: its widest line, by its lines on the theme's line box `line`
/// (the font size times `defaults.line_height`).
struct MeasuredText {
    content: String,
    size: f32,
    line: f32,
    font: iced::Font,
}

impl MeasuredText {
    fn extent(&self) -> iced::Size {
        use iced::advanced::text::Paragraph as _;
        let paragraph = <iced::Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
            iced::advanced::Text {
                content: self.content.as_str(),
                bounds: iced::Size::INFINITE,
                size: iced::Pixels(self.size),
                line_height: text::LineHeight::Absolute(iced::Pixels(self.line)),
                font: self.font,
                align_x: text::Alignment::Default,
                align_y: iced::alignment::Vertical::Top,
                shaping: text::Shaping::Advanced,
                wrapping: text::Wrapping::None,
            },
        );
        paragraph.min_bounds()
    }

    /// The text's rectangle in `room`: at its left edge, centred
    /// vertically, as the widgets set their text.
    fn placed(&self, room: iced::Rectangle) -> iced::Rectangle {
        let size = self.extent();
        iced::Rectangle::new(
            iced::Point::new(room.x, room.center_y() - size.height / 2.0),
            size,
        )
    }
}

/// Tags `content` as the list's element `id`.
fn tagged<'a>(id: &'static str, content: impl Into<Element<'a, Message>>) -> Tagged<'a> {
    Tagged {
        id,
        content: content.into(),
        parts: Vec::new(),
        texts: Vec::new(),
        root: false,
        loose: false,
        within: None,
    }
}

impl<'a> Tagged<'a> {
    /// A text part of the element, `id`: `text` as iced shapes it, in the
    /// box `origin` finds ([`MeasuredText::placed`]).
    fn text_part(
        mut self,
        id: &'static str,
        text: MeasuredText,
        origin: impl Fn(iced::advanced::Layout<'_>) -> Option<iced::Rectangle> + 'a,
    ) -> Self {
        self.texts.push((id, text, Box::new(origin)));
        self
    }

    /// The element laid out at its own width even where that is wider than
    /// the room it is given, so it shows past its column as the overflow it
    /// is, rather than squeezed.
    fn loose(mut self) -> Self {
        self.loose = true;
        self
    }

    /// The element recorded as the part of it inside the element `outer`,
    /// which clips it: `outer` encloses it, so it is recorded first, in
    /// the same frame and the same survey.
    fn within(mut self, outer: &'static str) -> Self {
        self.within = Some(outer);
        self
    }

    /// A part of the element, `id`, at the rectangle `rect` finds.
    fn part(
        mut self,
        id: &'static str,
        rect: impl Fn(iced::advanced::Layout<'_>) -> Option<iced::Rectangle> + 'a,
    ) -> Self {
        self.parts.push((id, Box::new(rect)));
        self
    }

    /// A part of the element, `id`, that is the node of its layout reached
    /// by taking the child `path` names at each level.
    fn node(self, id: &'static str, path: &'static [usize]) -> Self {
        self.part(id, move |layout| node_at(layout, path).map(|n| n.bounds()))
    }

    /// The window: the element that starts each frame's record and reports
    /// the hover target.
    fn root(mut self) -> Self {
        self.root = true;
        self
    }

    /// The element's rectangle and its parts', into `rects`.
    fn record(&self, layout: iced::advanced::Layout<'_>, rects: &mut ElementRects) {
        let bounds = layout.bounds();
        let shown = match self.within.and_then(|outer| rects.get(outer)) {
            Some(outer) => bounds.intersection(outer).unwrap_or(iced::Rectangle {
                width: 0.0,
                ..bounds
            }),
            None => bounds,
        };
        rects.insert(self.id, shown);
        for (id, rect) in &self.parts {
            if let Some(bounds) = rect(layout) {
                rects.insert(id, bounds);
            }
        }
        for (id, measured, origin) in &self.texts {
            if let Some(room) = origin(layout) {
                rects.insert(id, measured.placed(room));
            }
        }
    }
}

impl<'a> From<Tagged<'a>> for Element<'a, Message> {
    fn from(tagged: Tagged<'a>) -> Self {
        Element::new(tagged)
    }
}

/// The node `path` reaches from `layout`, a child index at each level.
fn node_at<'l>(
    layout: iced::advanced::Layout<'l>,
    path: &[usize],
) -> Option<iced::advanced::Layout<'l>> {
    path.iter()
        .try_fold(layout, |node, &index| node.children().nth(index))
}

/// The rectangles of an element tree, by element id.
type ElementRects = BTreeMap<&'static str, iced::Rectangle>;

thread_local! {
    /// The rectangles of the frame being drawn.
    static DRAWING: std::cell::RefCell<ElementRects> =
        const { std::cell::RefCell::new(BTreeMap::new()) };
    /// The rectangles of the last frame drawn whole, its overlays included.
    static DRAWN: std::cell::RefCell<ElementRects> =
        const { std::cell::RefCell::new(BTreeMap::new()) };
    /// The smallest hover target under the pointer, and its area, while a
    /// pointer move goes through the tree.
    static UNDER_POINTER: std::cell::Cell<Option<(&'static str, f32)>> =
        const { std::cell::Cell::new(None) };
    /// The hover target last reported, `None` once the pointer left them all.
    static REPORTED: std::cell::Cell<Option<&'static str>> = const { std::cell::Cell::new(None) };
    /// When the window first asked to be drawn.
    static FIRST_FRAME: std::cell::Cell<Option<Instant>> = const { std::cell::Cell::new(None) };
    /// The rectangles the layout dump last wrote.
    static DUMPED: std::cell::RefCell<Option<ElementRects>> =
        const { std::cell::RefCell::new(None) };
    /// The rectangles of every element laid out, drawn or not, as the last
    /// operation over the tree found them ([`Survey`]).
    static SURVEYED: std::cell::RefCell<ElementRects> =
        const { std::cell::RefCell::new(BTreeMap::new()) };
}

/// The operation the layout dump runs over the whole tree: it does nothing
/// but go everywhere, so each [`Tagged`] records itself as it is operated on
/// (`Tagged::operate`), in view or scrolled off.
struct Survey;

impl iced::advanced::widget::Operation for Survey {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn iced::advanced::widget::Operation)) {
        operate(self);
    }

    fn finish(&self) -> iced::advanced::widget::operation::Outcome<()> {
        iced::advanced::widget::operation::Outcome::Some(())
    }
}

/// The rectangles of every element the last [`Survey`] found.
fn surveyed_layout() -> ElementRects {
    SURVEYED.with_borrow(Clone::clone)
}

/// How long after the window's first frame the layout dump waits before it
/// writes: the showcase's fonts, icons and theme are all loaded before its
/// first frame, and a second is many frames for anything laid out late to
/// settle. The capture scripts capture no sooner than four seconds after
/// the start (`HELD_PRESS_AFTER`'s note), so the dump is written before.
const LAYOUT_SETTLES_AFTER: Duration = Duration::from_secs(1);

/// The rectangles of the last frame drawn whole.
fn drawn_layout() -> ElementRects {
    DRAWN.with_borrow(Clone::clone)
}

impl iced::advanced::Widget<Message, Theme, iced::Renderer> for Tagged<'_> {
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        self.content.as_widget().diff(tree);
    }

    fn size(&self) -> iced::Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> iced::Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &iced::Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        if self.loose {
            let room = iced::advanced::layout::Limits::new(
                iced::Size::ZERO,
                iced::Size::new(f32::INFINITY, limits.max().height),
            );
            return self.content.as_widget_mut().layout(tree, renderer, &room);
        }
        self.content.as_widget_mut().layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        if self.root {
            let frame = DRAWING.take();
            if !frame.is_empty() {
                DRAWN.set(frame);
            }
        }
        DRAWING.with_borrow_mut(|drawing| self.record(layout, drawing));
        self.content
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    /// Every operation goes through the whole tree, whether a widget is in
    /// view or not, so the element records itself for the layout dump here
    /// too: the dump's [`Survey`] finds what a scrolled-off part of the page
    /// lays out, which iced does not draw.
    fn operate(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        if self.root {
            SURVEYED.with_borrow_mut(BTreeMap::clear);
        }
        SURVEYED.with_borrow_mut(|surveyed| self.record(layout, surveyed));
        self.content
            .as_widget_mut()
            .operate(tree, layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        let moved = matches!(
            event,
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { .. })
        );
        if self.root && moved {
            UNDER_POINTER.set(None);
        }
        self.content.as_widget_mut().update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
        let bounds = layout.bounds();
        if moved && cursor.is_over(bounds) && hover_target(self.id) {
            let area = bounds.width * bounds.height;
            if UNDER_POINTER
                .get()
                .is_none_or(|(_, smallest)| area < smallest)
            {
                UNDER_POINTER.set(Some((self.id, area)));
            }
        }
        if !self.root {
            return;
        }
        if moved {
            let under = UNDER_POINTER.get().map(|(id, _)| id);
            if under != REPORTED.get() {
                REPORTED.set(under);
                if let Some(id) = under {
                    shell.publish(Message::ElementHovered(id));
                }
            }
        }
        if let iced::Event::Window(iced::window::Event::RedrawRequested(now)) = event
            && CLI_ARGS.get().is_some_and(|cli| cli.dump_layout.is_some())
        {
            let first = *FIRST_FRAME.get().get_or_insert(*now);
            FIRST_FRAME.set(Some(first));
            if now.saturating_duration_since(first) < LAYOUT_SETTLES_AFTER {
                shell.request_redraw();
                return;
            }
            let frame = drawn_layout();
            let changed = DUMPED.with_borrow(|dumped| dumped.as_ref() != Some(&frame));
            if changed && !frame.is_empty() {
                DUMPED.set(Some(frame));
                shell.publish(Message::LayoutSettled);
            } else if DUMPED.with_borrow(Option::is_none) {
                shell.request_redraw();
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &iced::Renderer,
    ) -> iced::mouse::Interaction {
        self.content
            .as_widget()
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &iced::Rectangle,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, iced::Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(tree, layout, renderer, viewport, translation)
    }
}

/// The layout dump `--dump-layout` writes: the rectangles the last
/// [`Survey`] found, every element laid out, the ones scrolled out of view
/// too, in logical pixels of the window's content, by element id,
/// with the theme and the scale factor, in the list's schema
/// (docs/showcase-elements.toml, "The layout dump").
fn write_layout_dump(state: &State, path: &str, scale: f32) -> Result<(), String> {
    let elements: serde_json::Map<String, serde_json::Value> = surveyed_layout()
        .into_iter()
        .map(|(id, r)| {
            (
                id.to_string(),
                serde_json::json!({"x": r.x, "y": r.y, "w": r.width, "h": r.height}),
            )
        })
        .collect();
    let preset = match &state.current_choice {
        ThemeChoice::OsTheme(_) => state.default_label.clone(),
        ThemeChoice::Preset(key) => key.clone(),
    };
    let dump = serde_json::json!({
        "kind": "iced",
        "preset": preset,
        "variant": if state.is_dark { "dark" } else { "light" },
        "scale": scale,
        "elements": elements,
    });
    let text = serde_json::to_string_pretty(&dump).map_err(|e| e.to_string())?;
    std::fs::write(path, text).map_err(|e| format!("{path}: {e}"))
}

// ---------------------------------------------------------------------------
// CLI argument parsing
// ---------------------------------------------------------------------------

/// Optional CLI arguments for launching the showcase in a specific state.
///
/// Parsed from `std::env::args()` -- no external crate dependency.
/// When no arguments are provided the showcase behaves identically to before.
#[derive(Default)]
struct CliArgs {
    theme: Option<String>,
    variant: Option<String>,
    tab: Option<String>,
    icon_set: Option<String>,
    screenshot: Option<String>,
    /// `--capture`: a tool outside the showcase captures its window, which
    /// opens as a `--screenshot` run's does (`capture_window_settings`).
    capture: bool,
    /// `--pointer X,Y`: the pointer is held at that point of the window's
    /// content, in logical pixels, so a capture shows the control under it
    /// hovered ([`HeldPointer`]). For captures where no pointer can be
    /// driven, as in a nested compositor.
    pointer: Option<(u16, u16)>,
    /// A `--pointer` value that is not `X,Y`, or its missing value: `main`
    /// reports it and exits with 1, as a failed `--screenshot` does, rather
    /// than capture the window at rest.
    bad_pointer: Option<String>,
    /// `--press`: with `--pointer`, the primary button is held down there,
    /// so a capture shows the control pressed.
    press: bool,
    /// `--dump-layout FILE`: once the layout has settled, the rectangle of
    /// every element of docs/showcase-elements.toml the showcase draws is
    /// written to FILE as JSON ([`write_layout_dump`]), and again whenever it
    /// changes; without `--capture` the showcase then exits.
    dump_layout: Option<String>,
    /// `--open-menu NAME`: the menu bar's menu `NAME` is open from the first
    /// frame and held open ([`HeldMenu`]), so a capture and the layout dump
    /// show it with its rows. `theme`, the Theme menu, is the one menu the
    /// three showcases share (docs/showcase-elements.toml).
    open_menu: Option<String>,
}

/// `X,Y` as two whole logical pixels, or `None`.
fn parse_point(value: &str) -> Option<(u16, u16)> {
    let (x, y) = value.split_once(',')?;
    Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
}

/// Global CLI args, set once in `main()` before the iced application starts.
static CLI_ARGS: OnceLock<CliArgs> = OnceLock::new();

impl CliArgs {
    fn parse() -> Self {
        let mut args = Self::default();
        let argv: Vec<String> = std::env::args().collect();
        let mut i = 1; // skip binary name
        while i < argv.len() {
            match argv[i].as_str() {
                "--theme" => {
                    i += 1;
                    if i < argv.len() {
                        args.theme = Some(argv[i].clone());
                    }
                }
                "--variant" => {
                    i += 1;
                    if i < argv.len() {
                        args.variant = Some(argv[i].to_lowercase());
                    }
                }
                "--tab" => {
                    i += 1;
                    if i < argv.len() {
                        args.tab = Some(argv[i].to_lowercase());
                    }
                }
                "--icon-set" => {
                    i += 1;
                    if i < argv.len() {
                        args.icon_set = Some(argv[i].clone());
                    }
                }
                "--screenshot" => {
                    i += 1;
                    if i < argv.len() {
                        args.screenshot = Some(argv[i].clone());
                    }
                }
                "--capture" => args.capture = true,
                "--dump-layout" => {
                    i += 1;
                    if i < argv.len() {
                        args.dump_layout = Some(argv[i].clone());
                    }
                }
                "--press" => args.press = true,
                "--open-menu" => {
                    i += 1;
                    args.open_menu = Some(argv.get(i).map_or("", String::as_str).to_lowercase());
                }
                "--pointer" => {
                    i += 1;
                    let value = argv.get(i).map_or("", String::as_str);
                    match parse_point(value) {
                        Some(point) => args.pointer = Some(point),
                        None => args.bad_pointer = Some(value.to_string()),
                    }
                }
                _ => {} // ignore unknown args
            }
            i += 1;
        }
        args
    }

    /// The colour mode `--variant` names: `light`, `dark`, or `system`,
    /// which follows the OS.
    fn color_mode(variant: &str) -> Result<AppColorMode, String> {
        match variant {
            "light" => Ok(AppColorMode::Light),
            "dark" => Ok(AppColorMode::Dark),
            "system" => Ok(AppColorMode::System),
            other => Err(format!("--variant {other}: not light, dark or system")),
        }
    }

    /// The theme `--theme` names: `default`, the OS theme, or a preset the
    /// theme picker offers on this platform (`theme_choices`), which offers
    /// only the platform's own presets and the community ones.
    fn theme_choice(name: &str, default_label: &str) -> Result<ThemeChoice, String> {
        if name == "default" {
            return Ok(ThemeChoice::OsTheme(default_label.to_string()));
        }
        let offered = theme_choices(default_label);
        let choice = ThemeChoice::Preset(name.to_string());
        if offered.contains(&choice) {
            return Ok(choice);
        }
        let names: Vec<String> = offered
            .iter()
            .map(|choice| match choice {
                ThemeChoice::OsTheme(_) => "default".to_string(),
                ThemeChoice::Preset(key) => key.clone(),
            })
            .collect();
        let names = names.join(", ");
        if native_theme::theme::Theme::list_presets()
            .iter()
            .any(|info| info.key == name)
        {
            Err(format!(
                "--theme {name}: a preset of another platform; only this platform's \
                 presets run here: {names}"
            ))
        } else {
            Err(format!(
                "--theme {name}: no such preset; the presets are: {names}"
            ))
        }
    }

    /// The icon set `--icon-set` names, which the icon-theme picker offers
    /// (`build_icon_choices`): a bundled set, `system` or `freedesktop` (the
    /// system icon theme), or one of `installed_themes`, the freedesktop
    /// themes the picker lists.
    fn icon_set_choice(name: &str, installed_themes: &[String]) -> Result<IconSetChoice, String> {
        match name {
            "material" => Ok(IconSetChoice::Material),
            "lucide" => Ok(IconSetChoice::Lucide),
            "system" | "freedesktop" => Ok(IconSetChoice::System),
            theme if installed_themes.iter().any(|installed| installed == theme) => {
                Ok(IconSetChoice::Freedesktop(theme.to_string()))
            }
            other => Err(format!(
                "--icon-set {other}: not material, lucide, system, freedesktop or an \
                 installed icon theme; {}",
                if installed_themes.is_empty() {
                    "none are installed".to_string()
                } else {
                    format!(
                        "the installed icon themes are: {}",
                        installed_themes.join(", ")
                    )
                }
            )),
        }
    }

    /// The tab `--tab` names: a tab's [`Tab::flag`], or `textinputs` or
    /// `thememap` for the two tabs whose name has a hyphen.
    fn parse_tab(name: &str) -> Result<Tab, String> {
        let flag = match name {
            "textinputs" => "text-inputs",
            "thememap" => "theme-map",
            other => other,
        };
        Tab::ALL
            .iter()
            .copied()
            .find(|tab| tab.flag() == flag)
            .ok_or_else(|| {
                let tabs: Vec<&str> = Tab::ALL.iter().map(|tab| tab.flag()).collect();
                format!(
                    "--tab {name}: no such tab; the tabs are: {}",
                    tabs.join(", ")
                )
            })
    }
}

// ---------------------------------------------------------------------------
// Tab identifiers (right panel)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    /// The controls all three showcases draw, alike, on one screen: the tab
    /// the showcase opens on.
    Basic,
    Buttons,
    TextInputs,
    Selection,
    Range,
    Display,
    Layout,
    Graphics,
    /// The `iced_aw` widgets. Only when the connector's `iced_aw` feature is
    /// on, because `styles::aw` and the widgets themselves are behind it.
    #[cfg(feature = "iced_aw")]
    Extra,
    Icons,
    ThemeMap,
}

impl Tab {
    /// Every tab, in declaration order.
    ///
    /// `every_tab_renders` (spec §6.2) iterates this list, so a tab missing
    /// from it is a tab nothing renders under test. The `const` block below
    /// rejects a build where the list is out of order or stops short of the
    /// last declared variant; what no stable construct catches is a variant
    /// appended *after* `ThemeMap` and left out of the list, which `label` and
    /// `view`'s exhaustive matches still force the author to write.
    const ALL: &[Tab] = &[
        Tab::Basic,
        Tab::Buttons,
        Tab::TextInputs,
        Tab::Selection,
        Tab::Range,
        Tab::Display,
        Tab::Layout,
        Tab::Graphics,
        #[cfg(feature = "iced_aw")]
        Tab::Extra,
        Tab::Icons,
        Tab::ThemeMap,
    ];

    /// The name `--tab` takes for the tab.
    fn flag(self) -> &'static str {
        match self {
            Tab::Basic => "basic",
            Tab::Buttons => "buttons",
            Tab::TextInputs => "text-inputs",
            Tab::Selection => "selection",
            Tab::Range => "range",
            Tab::Display => "display",
            Tab::Layout => "layout",
            Tab::Graphics => "graphics",
            #[cfg(feature = "iced_aw")]
            Tab::Extra => "extra",
            Tab::Icons => "icons",
            Tab::ThemeMap => "theme-map",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Tab::Basic => "Basic",
            Tab::Buttons => "Buttons",
            Tab::TextInputs => "Text Inputs",
            Tab::Selection => "Selection",
            Tab::Range => "Range",
            Tab::Display => "Display",
            Tab::Layout => "Layout",
            Tab::Graphics => "Graphics",
            #[cfg(feature = "iced_aw")]
            Tab::Extra => "Extra widgets (iced_aw)",
            Tab::Icons => "Icons",
            Tab::ThemeMap => "Theme Map",
        }
    }
}

const _: () = {
    assert!(
        Tab::ALL.len() == Tab::ThemeMap as usize + 1,
        "Tab::ALL is missing a variant"
    );
    let mut i = 0;
    while i < Tab::ALL.len() {
        assert!(
            Tab::ALL[i] as usize == i,
            "Tab::ALL must list the variants in declaration order"
        );
        i += 1;
    }
};

// ---------------------------------------------------------------------------
// ThemeChoice enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum ThemeChoice {
    OsTheme(String),
    Preset(String),
}

impl std::fmt::Display for ThemeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The preset's display name, as the gpui and egui showcases' preset
        // switches show it; its key where no preset has that key.
        match self {
            ThemeChoice::OsTheme(label) => write!(f, "{label}"),
            ThemeChoice::Preset(key) => write!(f, "{}", preset_display_name(key)),
        }
    }
}

impl PartialEq for ThemeChoice {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ThemeChoice::OsTheme(_), ThemeChoice::OsTheme(_)) => true,
            (ThemeChoice::Preset(a), ThemeChoice::Preset(b)) => a == b,
            // Different ThemeChoice variants are never equal
            _ => false,
        }
    }
}

impl Eq for ThemeChoice {}

/// The bundled adwaita preset, resolved, as the last-resort fallback: what
/// is drawn, with the icon set and icon theme the preset states.
struct AdwaitaFallback {
    resolved: native_theme::theme::ResolvedTheme,
    theme: Theme,
    layout: LayoutTheme,
    icon_set: IconSet,
    icon_theme: Option<String>,
}

/// Load the bundled adwaita preset as a last-resort fallback.
///
/// Returns `None` if any step fails (should not happen for bundled data,
/// but we never panic).
fn load_adwaita_fallback(is_dark: bool) -> Option<AdwaitaFallback> {
    let nt = native_theme::theme::Theme::preset("adwaita").ok()?;
    let r = nt
        .resolve(if is_dark {
            native_theme_iced::ColorMode::Dark
        } else {
            native_theme_iced::ColorMode::Light
        })
        .ok()?;
    Some(AdwaitaFallback {
        theme: native_theme_iced::to_theme(&r.variant, &nt.name),
        resolved: r.variant,
        layout: nt.layout.clone(),
        icon_set: r.icon_set,
        icon_theme: r.icon_theme.map(Cow::into_owned),
    })
}

/// The display name of the preset `key`, or `key` where no preset has it.
fn preset_display_name(key: &str) -> &str {
    native_theme::theme::Theme::list_presets()
        .iter()
        .find(|info| info.key == key)
        .map_or(key, |info| info.display_name)
}

fn theme_choices(default_label: &str) -> Vec<ThemeChoice> {
    let mut choices = vec![ThemeChoice::OsTheme(default_label.to_string())];
    choices.extend(
        native_theme::theme::Theme::list_presets_for_platform()
            .iter()
            .map(|info| ThemeChoice::Preset(info.key.to_string())),
    );
    choices
}

// ---------------------------------------------------------------------------
// Color mode (System / Light / Dark)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppColorMode {
    System,
    Light,
    Dark,
}

impl AppColorMode {
    const ALL: &[AppColorMode] = &[
        AppColorMode::System,
        AppColorMode::Light,
        AppColorMode::Dark,
    ];

    fn is_dark(self) -> bool {
        match self {
            AppColorMode::Light => false,
            AppColorMode::Dark => true,
            AppColorMode::System => native_theme::detect::system_is_dark(),
        }
    }
}

impl AppColorMode {
    /// The mode as the command palette names it: `System` with the mode the
    /// OS is in.
    fn palette_label(self) -> String {
        match self {
            AppColorMode::System => {
                let actual = if self.is_dark() { "Dark" } else { "Light" };
                format!("System ({actual})")
            }
            other => other.to_string(),
        }
    }
}

/// The mode as the Mode drop-down and the Theme menu name it, as the gpui
/// showcase's `AppColorMode::short_label` does.
impl std::fmt::Display for AppColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppColorMode::System => write!(f, "System"),
            AppColorMode::Light => write!(f, "Light"),
            AppColorMode::Dark => write!(f, "Dark"),
        }
    }
}

// ---------------------------------------------------------------------------
// Icon set choice (uses library type: native_theme::icons::IconSetChoice)
// ---------------------------------------------------------------------------

/// Build the available icon set choices for the dropdown.
///
/// Includes `Default(X)` (if icon_theme is specified), `System`, all installed
/// freedesktop themes, and the bundled sets, Lucide then Material, in the
/// gpui showcase's order.
fn build_icon_choices(
    icon_set: IconSet,
    icon_theme: Option<&str>,
    installed_themes: &[String],
) -> Vec<IconSetChoice> {
    let mut items = Vec::new();
    if let choice @ IconSetChoice::Default(_) = default_icon_choice(icon_set, icon_theme) {
        items.push(choice);
    }
    items.push(IconSetChoice::System);
    for name in installed_themes {
        items.push(IconSetChoice::Freedesktop(name.clone()));
    }
    items.push(IconSetChoice::Lucide);
    items.push(IconSetChoice::Material);
    items
}

// ---------------------------------------------------------------------------
// Radio demo choice
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Cherry,
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Apple => write!(f, "Apple"),
            Fruit::Banana => write!(f, "Banana"),
            Fruit::Cherry => write!(f, "Cherry"),
        }
    }
}

// ---------------------------------------------------------------------------
// Icon source tracking (matches gpui showcase)
// ---------------------------------------------------------------------------

/// Where an icon was loaded from.
#[derive(Clone, Copy, PartialEq)]
enum IconSource {
    /// Loaded from the OS/desktop icon theme (e.g. Breeze, Adwaita, SF Symbols).
    System,
    /// Bundled icon set (material or lucide) used directly.
    Bundled,
    /// System lookup failed; fell back to bundled Material SVGs.
    Fallback,
    /// No icon data available at all.
    NotFound,
}

impl IconSource {
    fn label(self) -> &'static str {
        match self {
            IconSource::System => "System",
            IconSource::Bundled => "Bundled",
            IconSource::Fallback => "Fallback",
            IconSource::NotFound => "Not Found",
        }
    }
}

struct LoadedIcon {
    role: IconRole,
    data: Option<IconData>,
    name: Option<&'static str>,
    source: IconSource,
}

/// Pre-load all 42 icons for the given choice, tracking source.
fn load_all_icons(
    choice: &IconSetChoice,
    resolved: &native_theme::theme::ResolvedTheme,
    theme_icon_set: IconSet,
) -> Vec<LoadedIcon> {
    let set = choice.effective_icon_set(theme_icon_set);
    let theme = choice.freedesktop_theme();
    let is_system_set = matches!(
        set,
        IconSet::Freedesktop | IconSet::SfSymbols | IconSet::SegoeIcons
    );

    // Foreground color for GTK symbolic icons (Adwaita, Yaru, etc.), SF Symbols, Segoe glyphs
    let tc = resolved.defaults.text_color;
    let fg = Some([tc.r, tc.g, tc.b]);

    // For system icon sets, pre-load the Material set so we can detect fallbacks
    let material_icons: Vec<Option<IconData>> = if is_system_set {
        IconRole::ALL
            .iter()
            .map(|role| MaterialLoader::new(*role).load())
            .collect()
    } else {
        vec![]
    };

    IconRole::ALL
        .iter()
        .enumerate()
        .map(|(i, &role)| {
            let data = match set {
                IconSet::Freedesktop => {
                    let mut l = FreedesktopLoader::new(role).color_opt(fg);
                    if let Some(t) = theme {
                        l = l.theme(t);
                    }
                    l.load()
                }
                IconSet::Material => MaterialLoader::new(role).load(),
                IconSet::Lucide => LucideLoader::new(role).load(),
                IconSet::SfSymbols => SfSymbolsLoader::new(role).color_opt(fg).load(),
                IconSet::SegoeIcons => SegoeIconsLoader::new(role).color_opt(fg).load(),
                _ => None,
            };
            let name = native_theme::theme::icon_name(role, set);
            let source = match (&data, is_system_set) {
                (None, _) => IconSource::NotFound,
                (Some(_), false) => IconSource::Bundled,
                (Some(IconData::Svg(loaded)), true) => {
                    // Compare with Material to detect fallback
                    if let Some(Some(IconData::Svg(mat))) = material_icons.get(i) {
                        if loaded == mat {
                            IconSource::Fallback
                        } else {
                            IconSource::System
                        }
                    } else {
                        IconSource::System
                    }
                }
                (Some(_), true) => {
                    // RGBA or other data comes from native APIs, always system
                    IconSource::System
                }
            };
            LoadedIcon {
                role,
                data,
                name,
                source,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Animated icon cache builder
// ---------------------------------------------------------------------------

/// Build the animation caches for the spinner of `icon_set`: for a
/// freedesktop set, the spinner of `freedesktop_theme` (the system's where
/// `None`), so the page never shows another theme's spinner; none where the
/// theme has none.
///
/// Returns the full set of animation state fields that go into `State`.
#[allow(clippy::type_complexity)]
fn build_animation_caches(
    icon_set: native_theme::theme::IconSet,
    freedesktop_theme: Option<&str>,
) -> (
    Vec<(String, AnimatedSvgHandles)>,          // animated_frames
    Vec<usize>,                                 // animated_frame_indices
    Vec<Duration>,                              // animated_frame_elapsed
    Vec<(String, iced_core::svg::Handle, u32)>, // animated_spins
    Instant,                                    // animation_start
    bool,                                       // reduced_motion
    Vec<(String, iced_core::svg::Handle)>,      // animated_static
) {
    let mut animated_frames = Vec::new();
    let mut animated_spins = Vec::new();
    let mut animated_static = Vec::new();

    let set_name = icon_set.name().to_string();
    {
        let indicator = match icon_set {
            IconSet::Freedesktop => FreedesktopLoader::load_indicator(freedesktop_theme),
            other => load_icon_indicator(other),
        };
        if let Some(anim) = indicator {
            // Cache static first-frame for reduced motion
            if let Some(handle) = to_svg_handle(anim.first_frame(), None) {
                animated_static.push((set_name.clone(), handle));
            }

            match &anim {
                AnimatedIcon::Frames(_) => {
                    if let Some(anim_handles) = animated_frames_to_svg_handles(&anim, None) {
                        animated_frames.push((set_name.clone(), anim_handles));
                    }
                }
                AnimatedIcon::Transform(data) => {
                    if let TransformAnimation::Spin { duration_ms } = data.animation()
                        && let Some(handle) = to_svg_handle(data.icon(), None)
                    {
                        animated_spins.push((set_name.clone(), handle, duration_ms.get()));
                    }
                }
                _ => {} // Future AnimatedIcon variants
            }
        }
    }

    let animated_frame_indices = vec![0; animated_frames.len()];
    let animated_frame_elapsed = vec![Duration::ZERO; animated_frames.len()];
    let animation_start = Instant::now();
    let reduced_motion = prefers_reduced_motion();

    (
        animated_frames,
        animated_frame_indices,
        animated_frame_elapsed,
        animated_spins,
        animation_start,
        reduced_motion,
        animated_static,
    )
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

struct State {
    // Theme
    current_choice: ThemeChoice,
    current_theme: Theme,
    color_mode: AppColorMode,
    is_dark: bool,
    current_resolved: native_theme::theme::ResolvedTheme,
    /// The current theme's layout spacing. Not part of `ResolvedTheme`: it is
    /// shared between the light and the dark variant, so it sits on the
    /// native `Theme` and on `SystemTheme` instead.
    layout: LayoutTheme,
    /// OS accessibility preferences (from SystemTheme, not ResolvedTheme).
    accessibility: native_theme_iced::AccessibilityPreferences,
    /// Icon set for the current theme (from Theme or SystemTheme, not ResolvedTheme).
    current_icon_set: IconSet,
    /// Icon theme name for the current theme; `None` where the theme names
    /// none and the system's cannot be detected.
    current_icon_theme: Option<String>,
    /// Dynamic label for the default theme entry, updated on color mode change.
    default_label: String,

    // Navigation
    active_tab: Tab,

    // Widget Info (hover-driven)
    widget_info: String,
    /// The element of docs/showcase-elements.toml the pointer was last over,
    /// which Widget Info shows in place of `widget_info`; `None` after a
    /// hover of another page's widget.
    hovered_element: Option<&'static str>,

    // Chrome
    /// The inspector's open tab.
    inspector_tab: InspectorTab,
    /// Whether the side panel is shown, and how wide the splitter left it.
    side_panel_visible: bool,
    side_panel_width: f32,
    /// Whether the splitter is being dragged, and whether the pointer is on
    /// it.
    splitter_dragging: bool,
    splitter_hovered: bool,
    /// The menu bar's title whose menu is open, by its index among the
    /// bar's titles.
    menu_title_open: Option<usize>,
    /// The window's logical width, which the splitter leaves the page
    /// [`PANEL_MIN_WIDTH`] of.
    window_width: f32,
    /// The dialog open over the window, if any.
    overlay: Option<Overlay>,
    /// The command palette's query.
    palette_query: String,
    /// The chrome's icons, of the chosen icon theme.
    chrome_icons: ChromeIcons,

    // Basic tab
    /// The Basic tab's radio button, text fields, drop-down and slider.
    basic_radio: usize,
    basic_hint: String,
    basic_text: String,
    basic_fruit: Fruit,
    basic_slider: f32,
    /// The Basic tab's text area.
    basic_text_area: text_editor::Content,
    /// The Basic tab's chosen segment, and whether its two expanders are
    /// open.
    basic_segment: usize,
    basic_details_open: bool,
    basic_more_open: bool,
    /// The Basic tab's tab bar and list: their selection.
    #[cfg(feature = "iced_aw")]
    basic_tab: usize,
    basic_list_selected: Option<usize>,
    /// The Basic tab's number input and focused input.
    #[cfg(feature = "iced_aw")]
    basic_number: i32,
    #[cfg(not(feature = "iced_aw"))]
    basic_number_text: String,
    basic_focused: String,

    // Button tab
    button_press_count: u32,

    // Text input tab
    text_input_value: String,
    text_editor_content: text_editor::Content,

    // Selection tab
    checkbox_a: bool,
    checkbox_b: bool,
    checkbox_c: bool,
    selected_fruit: Option<Fruit>,
    toggler_enabled: bool,
    pick_list_selected: Option<String>,
    combo_state: combo_box::State<String>,
    combo_selected: Option<String>,

    // Range tab
    slider_value: f32,
    slider_step: f32,
    vslider_value: f32,
    progress_value: f32,

    // Layout tab
    /// The `pane_grid`'s own layout state: splits, ratios and pane order.
    panes: pane_grid::State<usize>,
    /// The number the next pane created by a split is labelled with.
    pane_count: usize,
    /// The pane last clicked, dragged or created.
    focused_pane: Option<pane_grid::Pane>,

    // Graphics tab
    /// The parsed Markdown document the `markdown` section renders.
    markdown_content: markdown::Content,
    /// The last Markdown link clicked, echoed under the document.
    markdown_link: Option<String>,
    /// The encoded QR payload. `None` when the encoder rejects the data,
    /// which the section then says instead of showing a code.
    qr_data: Option<qr_code::Data>,

    // Extra widgets tab (iced_aw)
    /// The selected tab of the stand-alone `TabBar`.
    #[cfg(feature = "iced_aw")]
    aw_tab_bar_active: usize,
    /// The selected tab of the `Tabs` container, which owns its content.
    #[cfg(feature = "iced_aw")]
    aw_tabs_active: usize,
    /// The selected item of the `Sidebar`.
    #[cfg(feature = "iced_aw")]
    aw_sidebar_active: usize,
    /// The `SelectionList`'s options. It borrows them, so they live here.
    #[cfg(feature = "iced_aw")]
    aw_list_options: Vec<String>,
    /// The index the `SelectionList` has selected, if any.
    #[cfg(feature = "iced_aw")]
    aw_list_selected: Option<usize>,
    /// Whether the `Card` is on screen; its close button clears this.
    #[cfg(feature = "iced_aw")]
    aw_card_open: bool,
    /// The last menu or context-menu entry chosen, echoed in the tab.
    #[cfg(feature = "iced_aw")]
    aw_last_action: String,

    // Icons tab
    icon_set_choice: IconSetChoice,
    /// Whether `icon_set_choice` is re-derived from each theme installed:
    /// until the user picks an icon theme, and after a pick of the
    /// `default` row. Not `IconSetChoice::follows_preset`: a choice that
    /// followed the preset onto `system`, where the preset's own icon theme
    /// is not installed (`default_icon_choice`), keeps following it.
    icon_choice_follows_preset: bool,
    icon_set_choices: Vec<IconSetChoice>,
    loaded_icons: Vec<LoadedIcon>,
    /// Cached list of installed freedesktop icon themes (populated once at init).
    installed_themes: Vec<String>,

    // Animated Icons state
    /// Cached SVG handles for frame-based animations: (set_name, AnimatedSvgHandles).
    animated_frames: Vec<(String, AnimatedSvgHandles)>,
    /// Current frame index per frame-based animation.
    animated_frame_indices: Vec<usize>,
    /// Elapsed time tracker per frame-based animation (for correct per-animation timing).
    animated_frame_elapsed: Vec<Duration>,
    /// Cached SVG handle + duration for transform (spin) animations: (set_name, handle, duration_ms).
    animated_spins: Vec<(String, iced_core::svg::Handle, u32)>,
    /// Start time for spin animations (used with spin_rotation_radians).
    animation_start: Instant,
    /// Whether reduced motion is active (cached at init).
    reduced_motion: bool,
    /// Static first-frame SVG handles for reduced motion: (set_name, handle).
    animated_static: Vec<(String, iced_core::svg::Handle)>,
    /// The Basic page's spinner: the icon set's indicator, or the arc.
    basic_spinner: native_theme_iced::Spinner,

    // Screenshot mode
    screenshot_path: Option<String>,
    screenshot_countdown: u8,

    /// Error message from theme loading, displayed as a banner in the UI.
    error_message: Option<String>,
    /// Reads the OS theme at startup and when the `default` entry is
    /// installed after it: `SystemTheme::from_system`, which a test replaces
    /// with a read that fails.
    read_system_theme: fn() -> native_theme::Result<native_theme::SystemTheme>,
    /// Detects the system icon theme the Icons page names:
    /// `system_icon_theme`, which a test replaces with a detection that
    /// fails.
    detect_icon_theme: fn() -> native_theme::Result<String>,

    // Theme watcher (runtime dark/light toggle detection)
    /// Flag set by the ThemeSubscription background thread when the OS theme changes.
    theme_change_flag: Arc<AtomicBool>,
    /// RAII guard keeping the theme watcher background thread alive.
    _theme_watcher: Option<native_theme::watch::ThemeSubscription>,
}

impl Default for State {
    fn default() -> Self {
        Self::starting_with(native_theme::SystemTheme::from_system)
    }
}

impl State {
    /// The showcase at startup, with the OS theme `read_system_theme` reads
    /// installed, or adwaita where that read fails.
    fn starting_with(
        read_system_theme: fn() -> native_theme::Result<native_theme::SystemTheme>,
    ) -> Self {
        let color_mode = AppColorMode::System;
        let is_dark = color_mode.is_dark();
        let (
            resolved,
            theme,
            initial_error,
            system_preset,
            init_icon_set,
            init_icon_theme,
            accessibility,
            layout,
            fallback_choice,
        ) = match read_system_theme() {
            Ok(system) => {
                let r = system
                    .pick(if is_dark {
                        native_theme_iced::ColorMode::Dark
                    } else {
                        native_theme_iced::ColorMode::Light
                    })
                    .clone();
                let t = native_theme_iced::to_theme(&r, &system.name);
                let preset = system.preset.clone();
                let is = system.icon_set;
                let it = system.icon_theme.map(Cow::into_owned);
                let acc = system.accessibility;
                let lay = system.layout.clone();
                (r, t, None, preset, is, it, acc, lay, None)
            }
            Err(e) => {
                // Fallback: load adwaita preset through resolve pipeline. The
                // theme picker names what is drawn, so a mode change installs
                // adwaita again instead of reading the OS theme.
                match load_adwaita_fallback(is_dark) {
                    Some(fallback) => (
                        fallback.resolved,
                        fallback.theme,
                        Some(format!("OS theme failed: {e}. Using adwaita fallback.")),
                        "adwaita".to_string(),
                        fallback.icon_set,
                        fallback.icon_theme,
                        native_theme_iced::AccessibilityPreferences::default(),
                        fallback.layout,
                        Some(ThemeChoice::Preset("adwaita".to_string())),
                    ),
                    None => {
                        // This is the only safe fallback when both OS theme
                        // detection and the bundled adwaita preset fail.
                        // With bundled data this case is near-impossible.
                        // process::exit avoids constructing a dummy
                        // ResolvedTheme (30+ required fields).
                        eprintln!(
                            "Fatal: OS theme failed ({e}) and adwaita fallback \
                                 also failed. Cannot start."
                        );
                        std::process::exit(1);
                    }
                }
            }
        };

        let languages = vec![
            "Rust".to_string(),
            "Python".to_string(),
            "JavaScript".to_string(),
            "TypeScript".to_string(),
            "Go".to_string(),
            "C++".to_string(),
            "Java".to_string(),
            "Swift".to_string(),
            "Kotlin".to_string(),
            "Zig".to_string(),
        ];

        // Populate installed freedesktop themes once at init.
        let installed_themes = list_freedesktop_themes();
        let init_icon_theme_opt: Option<&str> = init_icon_theme.as_deref();
        let icon_set_choice = default_icon_choice(init_icon_set, init_icon_theme_opt);
        let icon_set_choices =
            build_icon_choices(init_icon_set, init_icon_theme_opt, &installed_themes);
        let loaded_icons = load_all_icons(&icon_set_choice, &resolved, init_icon_set);
        let chrome_icons = load_chrome_icons(&icon_set_choice, &resolved, init_icon_set);

        let anim_set = icon_set_choice.effective_icon_set(init_icon_set);
        let (
            animated_frames,
            animated_frame_indices,
            animated_frame_elapsed,
            animated_spins,
            animation_start,
            reduced_motion,
            animated_static,
        ) = build_animation_caches(anim_set, icon_set_choice.freedesktop_theme());
        let basic_spinner = native_theme_iced::Spinner::new(
            &resolved,
            anim_set,
            icon_set_choice.freedesktop_theme(),
        );

        let default_label = format!("default ({})", system_preset);

        // Two panes to start with, so the vertical split between them is
        // there to be dragged before anything is clicked.
        let (mut panes, first_pane) = pane_grid::State::new(1);
        let pane_count = if panes
            .split(pane_grid::Axis::Vertical, first_pane, 2)
            .is_some()
        {
            2
        } else {
            1
        };

        // Start theme watcher for runtime dark/light toggle detection.
        // Skip in screenshot mode — the watcher's background thread cleanup
        // races with the Cocoa runtime on macOS CI, causing SIGTRAP on exit.
        // Skip under `cargo test` too: a file watch on the desktop's own
        // configuration is host I/O, and nothing the self-tests assert
        // depends on it.
        let theme_change_flag = Arc::new(AtomicBool::new(false));
        let is_screenshot = CLI_ARGS.get().is_some_and(|cli| cli.screenshot.is_some());
        let _theme_watcher = if is_screenshot || cfg!(test) {
            None
        } else {
            let flag_clone = theme_change_flag.clone();
            native_theme::watch::on_theme_change(move |_event| {
                flag_clone.store(true, Ordering::Release);
            })
            .ok()
        };

        let mut state = Self {
            current_choice: fallback_choice
                .unwrap_or_else(|| ThemeChoice::OsTheme(default_label.clone())),
            current_theme: theme,
            color_mode,
            is_dark,
            current_resolved: resolved,
            layout,
            accessibility,
            current_icon_set: init_icon_set,
            current_icon_theme: init_icon_theme,
            default_label,
            active_tab: Tab::Basic,
            widget_info: String::new(),
            hovered_element: None,
            inspector_tab: InspectorTab::Widget,
            side_panel_visible: true,
            side_panel_width: LEFT_PANEL_WIDTH,
            splitter_dragging: false,
            splitter_hovered: false,
            menu_title_open: None,
            window_width: WINDOW_SIZE.0,
            overlay: None,
            palette_query: String::new(),
            chrome_icons,
            basic_radio: 0,
            basic_hint: String::new(),
            basic_text: "Text".to_string(),
            basic_fruit: Fruit::Apple,
            basic_slider: BASIC_SLIDER,
            basic_text_area: text_editor::Content::with_text(BASIC_TEXT_AREA),
            basic_segment: BASIC_SEGMENT,
            basic_details_open: true,
            basic_more_open: false,
            #[cfg(feature = "iced_aw")]
            basic_tab: 0,
            basic_list_selected: Some(BASIC_LIST_SELECTED),
            #[cfg(feature = "iced_aw")]
            basic_number: BASIC_NUMBER,
            #[cfg(not(feature = "iced_aw"))]
            basic_number_text: BASIC_NUMBER.to_string(),
            basic_focused: "Focused".to_string(),
            button_press_count: 0,
            text_input_value: String::new(),
            text_editor_content: text_editor::Content::with_text(
                "This is a multi-line text editor.\nEdit this text freely.\n\nIt supports:\n  - Multiple lines\n  - Scrolling\n  - Selection",
            ),
            checkbox_a: true,
            checkbox_b: false,
            checkbox_c: true,
            selected_fruit: Some(Fruit::Apple),
            toggler_enabled: false,
            pick_list_selected: Some("Rust".to_string()),
            combo_state: combo_box::State::new(languages),
            combo_selected: None,
            slider_value: 65.0,
            slider_step: 25.0,
            vslider_value: 50.0,
            progress_value: 72.0,
            panes,
            pane_count,
            focused_pane: Some(first_pane),
            markdown_content: markdown::Content::parse(MARKDOWN_SAMPLE),
            markdown_link: None,
            qr_data: qr_code::Data::new(QR_PAYLOAD).ok(),
            #[cfg(feature = "iced_aw")]
            aw_tab_bar_active: 0,
            #[cfg(feature = "iced_aw")]
            aw_tabs_active: 0,
            #[cfg(feature = "iced_aw")]
            aw_sidebar_active: 0,
            #[cfg(feature = "iced_aw")]
            aw_list_options: vec![
                "Adwaita".to_string(),
                "Breeze".to_string(),
                "Catppuccin".to_string(),
                "Dracula".to_string(),
                "Gruvbox".to_string(),
                "Nord".to_string(),
                "Solarized".to_string(),
                "Tokyo Night".to_string(),
            ],
            #[cfg(feature = "iced_aw")]
            aw_list_selected: Some(0),
            #[cfg(feature = "iced_aw")]
            aw_card_open: true,
            #[cfg(feature = "iced_aw")]
            aw_last_action: String::new(),
            icon_set_choice,
            icon_choice_follows_preset: true,
            icon_set_choices,
            loaded_icons,
            installed_themes,
            animated_frames,
            animated_frame_indices,
            animated_frame_elapsed,
            animated_spins,
            animation_start,
            reduced_motion,
            animated_static,
            basic_spinner,
            screenshot_path: None,
            screenshot_countdown: 0,
            error_message: initial_error,
            read_system_theme,
            detect_icon_theme: native_theme::theme::system_icon_theme,
            theme_change_flag,
            _theme_watcher,
        };

        if let Some(cli) = CLI_ARGS.get() {
            apply_cli_args(&mut state, cli);
        }

        state
    }
}

/// Put the showcase into the state the command line asks for: its colour
/// mode, theme, tab and icons.
///
/// Each flag takes exactly what the matching picker offers. A value the
/// showcase cannot honour is reported on stderr and ignored: the setting
/// stays what it would have been without the flag. `--variant` installs
/// the theme in the mode it names: the one `--theme` names, or, where
/// `--theme` is absent or rejected here, the one installed. A `--theme`
/// accepted here that then fails to load changes neither the theme nor the
/// mode, `--variant`'s included: the install is all or nothing
/// (`State::install_in_mode`).
fn apply_cli_args(state: &mut State, cli: &CliArgs) {
    let mode = cli
        .variant
        .as_deref()
        .and_then(|variant| reported(CliArgs::color_mode(variant)));
    let choice = cli
        .theme
        .as_deref()
        .and_then(|name| reported(CliArgs::theme_choice(name, &state.default_label)));
    if mode.is_some() || choice.is_some() {
        let choice = choice.unwrap_or_else(|| state.current_choice.clone());
        let mode = mode.unwrap_or(state.color_mode);
        state.install_in_mode(choice, mode);
    }

    // Override tab
    if let Some(tab) = cli
        .tab
        .as_deref()
        .and_then(|name| reported(CliArgs::parse_tab(name)))
    {
        state.active_tab = tab;
    }

    // `--icon-set` names a set, which stays chosen across theme switches as
    // a pick in the icon-theme picker does.
    if let Some(choice) = cli
        .icon_set
        .as_deref()
        .and_then(|name| reported(CliArgs::icon_set_choice(name, &state.installed_themes)))
    {
        state.choose_icon_set(choice);
    }

    // Apply screenshot settings
    if let Some(ref path) = cli.screenshot {
        state.screenshot_path = Some(path.clone());
        state.screenshot_countdown = 60; // 60 ticks × 50ms = 3s render delay
    }
}

/// The value of `result`; its error reported on stderr, and `None`.
fn reported<T>(result: Result<T, String>) -> Option<T> {
    result.map_err(|error| eprintln!("{error}; ignored")).ok()
}

impl State {
    /// Install `choice` in `mode`: every theme and colour-mode switch
    /// installs through here. The pickers then show `choice` and `mode`.
    ///
    /// A theme that fails to load leaves the installed one on screen, and
    /// the pickers showing it and the mode it is drawn in, with the error in
    /// the banner; false is returned.
    fn install_in_mode(&mut self, choice: ThemeChoice, mode: AppColorMode) -> bool {
        let is_dark = mode.is_dark();
        let icon_theme = match self.load_theme(&choice, is_dark) {
            Ok(icon_theme) => icon_theme,
            Err(error) => {
                self.error_message = Some(error);
                return false;
            }
        };
        self.current_choice = match choice {
            ThemeChoice::OsTheme(_) => ThemeChoice::OsTheme(self.default_label.clone()),
            preset @ ThemeChoice::Preset(_) => preset,
        };
        self.color_mode = mode;
        self.is_dark = is_dark;

        // Only a choice that follows the preset is re-derived; the user's
        // pick of an icon theme stays chosen across theme switches.
        let it_opt = icon_theme.as_deref();
        if self.icon_choice_follows_preset {
            self.icon_set_choice = default_icon_choice(self.current_icon_set, it_opt);
        }
        // Always rebuild the choices list (theme name in "default (X)" may have changed).
        self.icon_set_choices =
            build_icon_choices(self.current_icon_set, it_opt, &self.installed_themes);

        // Always reload icons — the resolved text_color may have changed (light↔dark)
        // even when the icon set choice is the same.
        self.reload_icons();
        true
    }

    /// Install the current theme again in the current mode.
    fn rebuild_theme(&mut self) {
        self.install_in_mode(self.current_choice.clone(), self.color_mode);
    }

    /// Load `choice` in the mode `is_dark` names into the theme fields.
    ///
    /// `Ok` holds the icon theme the theme states (`None` where its TOML
    /// states none); `Err` the error, with the theme fields untouched.
    fn load_theme(
        &mut self,
        choice: &ThemeChoice,
        is_dark: bool,
    ) -> Result<Option<String>, String> {
        let mode = if is_dark {
            native_theme_iced::ColorMode::Dark
        } else {
            native_theme_iced::ColorMode::Light
        };
        match choice {
            ThemeChoice::OsTheme(_) => match (self.read_system_theme)() {
                Ok(system) => {
                    // Platform presets always specify icon_theme.
                    self.current_icon_set = system.icon_set;
                    self.accessibility = system.accessibility.clone();
                    self.layout = system.layout.clone();
                    self.current_icon_theme = system.icon_theme.clone().map(Cow::into_owned);
                    self.current_resolved = system.pick(mode).clone();
                    self.current_theme =
                        native_theme_iced::to_theme(&self.current_resolved, &system.name);
                    self.default_label = format!("default ({})", system.preset);
                    self.error_message = None;
                    Ok(self.current_icon_theme.clone())
                }
                // Only startup falls back to adwaita, with nothing installed
                // yet; here the installed theme stays.
                Err(e) => Err(format!("Failed to load OS theme: {e}")),
            },
            ThemeChoice::Preset(name) => {
                let nt = native_theme::theme::Theme::preset(name)
                    .map_err(|e| format!("Failed to load preset '{name}': {e}"))?;
                let r = nt
                    .resolve(mode)
                    .map_err(|e| format!("Theme '{name}' resolution failed: {e}"))?;
                let icon_theme_explicit = r.icon_theme_explicit;
                self.current_icon_theme = r.icon_theme.map(Cow::into_owned);
                let icon_theme = self
                    .current_icon_theme
                    .clone()
                    .filter(|_| icon_theme_explicit);
                self.current_icon_set = r.icon_set;
                self.current_resolved = r.variant;
                self.layout = nt.layout.clone();
                self.current_theme = native_theme_iced::to_theme(&self.current_resolved, &nt.name);
                self.error_message = None;
                Ok(icon_theme)
            }
        }
    }

    /// The Icons page's line for the system icon theme: its name, or why
    /// none was detected -- then no system icon loads, and no other theme
    /// is named in its place.
    fn system_icon_theme_label(&self) -> String {
        match (self.detect_icon_theme)() {
            Ok(theme) => format!("System icon theme: {theme}"),
            Err(e) => format!("System icon theme: unavailable ({e})"),
        }
    }

    /// Load the icons and the spinner of the chosen icon set, recoloured for
    /// the installed theme.
    fn reload_icons(&mut self) {
        self.loaded_icons = load_all_icons(
            &self.icon_set_choice,
            &self.current_resolved,
            self.current_icon_set,
        );
        self.chrome_icons = load_chrome_icons(
            &self.icon_set_choice,
            &self.current_resolved,
            self.current_icon_set,
        );
        let anim_set = self
            .icon_set_choice
            .effective_icon_set(self.current_icon_set);
        let (af, afi, afe, asp, astart, rm, ast) =
            build_animation_caches(anim_set, self.icon_set_choice.freedesktop_theme());
        self.animated_frames = af;
        self.animated_frame_indices = afi;
        self.animated_frame_elapsed = afe;
        self.animated_spins = asp;
        self.animation_start = astart;
        self.reduced_motion = rm;
        self.animated_static = ast;
        self.basic_spinner = native_theme_iced::Spinner::new(
            &self.current_resolved,
            anim_set,
            self.icon_set_choice.freedesktop_theme(),
        );
    }

    /// Choose `choice`, the user's pick of an icon theme: only its
    /// `default` row follows the preset from now on.
    fn choose_icon_set(&mut self, choice: IconSetChoice) {
        self.icon_choice_follows_preset = choice.follows_preset();
        self.icon_set_choice = choice;
        self.reload_icons();
    }

    /// Whether animations stop: the OS asks for reduced motion, or the
    /// Preferences dialog's Reduce motion is on.
    fn motion_reduced(&self) -> bool {
        self.reduced_motion || self.accessibility.reduce_motion
    }
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Message {
    // Navigation
    TabSelected(Tab),

    // Theme
    ThemeSelected(ThemeChoice),
    ColorModeSelected(AppColorMode),

    // Widget Info hover
    WidgetHovered(String),
    WidgetUnhovered,
    /// The pointer moved onto an element of docs/showcase-elements.toml
    /// ([`Tagged`]): Widget Info shows it.
    ElementHovered(&'static str),

    // Layout dump (`--dump-layout`)
    /// The layout has settled, or changed since it was last written.
    LayoutSettled,
    /// The window's scale factor, read for the layout dump.
    LayoutScaled(f32),

    // Chrome: the menus', the toolbar's and the key bindings' actions
    /// A click on a menu's title: `iced_aw` opens its menu, and nothing else
    /// changes.
    MenuOpened,
    /// The menu bar's title whose menu is open, by its index, or none
    /// ([`MenuTitles`]).
    MenuTitleOpened(Option<usize>),
    Quit,
    ToggleSidePanel,
    /// The desktop's settings are read again and the current theme is
    /// installed from them, as a theme change the watcher saw is.
    ReloadSystemTheme,
    Open(Overlay),
    CloseOverlay,
    InspectorTabSelected(InspectorTab),
    /// Text for the clipboard: Widget Info's, or the About dialog's address.
    Copy(String),
    SplitterPressed,
    SplitterDragged(iced::Point),
    SplitterReleased,
    SplitterHovered(bool),
    PaletteQueryChanged(String),
    /// Enter in the command palette's query: its first entry runs.
    PaletteSubmitted,
    /// A command palette entry: the action it runs, then the palette closes.
    PaletteRun(Box<Message>),
    TextScaleSelected(TextScale),
    ReduceMotionToggled(bool),
    HighContrastToggled(bool),
    ReduceTransparencyToggled(bool),
    /// The window's new logical size, which bounds the splitter.
    WindowResized(iced::Size),

    // Basic tab
    /// A click on a checkbox held in the one state it shows: nothing changes.
    BasicHeld,
    BasicRadioSelected(usize),
    BasicHintChanged(String),
    BasicTextChanged(String),
    BasicFruitSelected(Fruit),
    BasicSliderChanged(f32),
    BasicTextAreaAction(text_editor::Action),
    BasicSegmentSelected(usize),
    BasicDetailsToggled,
    BasicMoreToggled,
    #[cfg(feature = "iced_aw")]
    BasicTabSelected(usize),
    BasicListSelected(usize),
    #[cfg(feature = "iced_aw")]
    BasicNumberChanged(i32),
    BasicFocusedChanged(String),

    // Button tab
    ButtonPressed,

    // Text input tab
    TextInputChanged(String),
    EditorAction(text_editor::Action),

    // Selection tab
    CheckboxAToggled(bool),
    CheckboxBToggled(bool),
    CheckboxCToggled(bool),
    FruitSelected(Fruit),
    TogglerToggled(bool),
    PickListSelected(String),
    ComboBoxSelected(String),

    // Range tab
    SliderChanged(f32),
    StepSliderChanged(f32),
    VSliderChanged(f32),
    ProgressChanged(f32),

    // Layout tab
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    PaneSplit(pane_grid::Axis, pane_grid::Pane),
    PaneClosed(pane_grid::Pane),

    // Graphics tab
    MarkdownLinkClicked(markdown::Uri),

    // Extra widgets tab (iced_aw)
    #[cfg(feature = "iced_aw")]
    AwTabBarSelected(usize),
    #[cfg(feature = "iced_aw")]
    AwTabsSelected(usize),
    #[cfg(feature = "iced_aw")]
    AwSidebarSelected(usize),
    #[cfg(feature = "iced_aw")]
    AwListSelected(usize, String),
    #[cfg(feature = "iced_aw")]
    AwCardToggled,
    #[cfg(feature = "iced_aw")]
    AwActionChosen(String),

    // Icons tab
    IconSetSelected(IconSetChoice),

    // Animated Icons
    AnimationTick,

    // Screenshot
    ScreenshotTick,
    /// iced's frame of the window's content: RGBA bytes, width, height and
    /// the scale factor it was drawn at.
    ScreenshotCaptured(Vec<u8>, u32, u32, f32),
    /// The window's logical content size and scale factor, read before an
    /// OS capture of the window with its frame.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    CaptureFrame(iced::Size, f32),

    // Theme watcher
    ThemeWatcherTick,
}

// ---------------------------------------------------------------------------
// Self-capture screenshot (macOS only)
// ---------------------------------------------------------------------------

/// Capture the iced window including decorations using macOS `screencapture -l`.
///
/// Gets the CGWindowID via NSApplication -> mainWindow -> windowNumber, then
/// shells out to `screencapture -l <id> -o <path>`.  This is the same approach
/// used by the gpui showcase capture.
#[cfg(target_os = "macos")]
fn capture_own_window_macos(output_path: &str) -> Result<(), String> {
    use std::process::Command;

    let Some(ns_app_class) = objc2::runtime::AnyClass::get(c"NSApplication") else {
        return Err("NSApplication class not found".into());
    };
    let window_id: i64 = unsafe {
        let ns_app: *mut objc2::runtime::AnyObject =
            objc2::msg_send![ns_app_class, sharedApplication];
        // Ensure the app is front-most (the second invocation on CI may
        // launch behind the terminal).
        let _: () = objc2::msg_send![ns_app, activateIgnoringOtherApps: true];

        // Try mainWindow first, then keyWindow, then the first window in
        // the windows array.  On CI the second run may not become main.
        let mut win: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, mainWindow];
        if win.is_null() {
            win = objc2::msg_send![ns_app, keyWindow];
        }
        if win.is_null() {
            let windows: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, windows];
            let count: usize = objc2::msg_send![windows, count];
            if count > 0 {
                win = objc2::msg_send![windows, objectAtIndex: 0usize];
            }
        }
        if win.is_null() {
            return Err("No window found".into());
        }
        objc2::msg_send![win, windowNumber]
    };

    let status = Command::new("screencapture")
        .args(["-l", &format!("{window_id}"), "-o", output_path])
        .status()
        .map_err(|e| format!("Failed to run screencapture: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("screencapture exited with {status}"))
    }
}

// ---------------------------------------------------------------------------
// Self-capture screenshot (Windows only)
// ---------------------------------------------------------------------------

/// Capture the iced window including decorations using Windows BitBlt.
///
/// Uses `FindWindowW` with the known window title to locate the correct HWND
/// (more reliable than `GetForegroundWindow` which may return a console or
/// other window on CI), then `BitBlt` + `GetDIBits` to extract pixel data.
#[cfg(target_os = "windows")]
fn capture_own_window_windows(output_path: &str) -> Result<(), String> {
    use windows::Win32::Foundation::*;
    use windows::Win32::Graphics::Dwm::*;
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::PCWSTR;

    unsafe {
        let title_w: Vec<u16> = WINDOW_TITLE
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let hwnd = FindWindowW(None, PCWSTR(title_w.as_ptr()))
            .map_err(|e| format!("FindWindowW failed: {e}"))?;

        // DWMWA_EXTENDED_FRAME_BOUNDS gives visible bounds in physical
        // screen pixels (excluding the invisible DWM border), matching
        // the screen DC coordinate space.  Fall back to GetWindowRect.
        let mut rect = RECT::default();
        if DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut _ as *mut std::ffi::c_void,
            std::mem::size_of::<RECT>() as u32,
        )
        .is_err()
        {
            GetWindowRect(hwnd, &mut rect).map_err(|e| format!("GetWindowRect failed: {e}"))?;
        }

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        if width <= 0 || height <= 0 {
            return Err(format!("Invalid window dimensions: {width}x{height}"));
        }
        eprintln!(
            "windows capture: rect=({},{},{},{}), size={}x{}",
            rect.left, rect.top, rect.right, rect.bottom, width, height
        );

        let screen_dc = GetDC(None);
        let mem_dc = CreateCompatibleDC(Some(screen_dc));
        let bitmap = CreateCompatibleBitmap(screen_dc, width, height);
        let old_obj = SelectObject(mem_dc, bitmap.into());

        let blt_result = BitBlt(
            mem_dc,
            0,
            0,
            width,
            height,
            Some(screen_dc),
            rect.left,
            rect.top,
            SRCCOPY | CAPTUREBLT,
        );

        if blt_result.is_err() {
            SelectObject(mem_dc, old_obj);
            let _ = DeleteObject(bitmap.into());
            let _ = DeleteDC(mem_dc);
            ReleaseDC(None, screen_dc);
            return Err("BitBlt failed".into());
        }

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0 as u32,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut pixels = vec![0u8; (width * height * 4) as usize];
        let lines = GetDIBits(
            mem_dc,
            bitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        SelectObject(mem_dc, old_obj);
        let _ = DeleteObject(bitmap.into());
        let _ = DeleteDC(mem_dc);
        ReleaseDC(None, screen_dc);

        if lines == 0 {
            return Err("GetDIBits returned 0 lines".into());
        }

        for chunk in pixels.as_chunks_mut::<4>().0 {
            chunk.swap(0, 2); // BGRA -> RGBA
            chunk[3] = 255; // force opaque
        }

        image::save_buffer(
            output_path,
            &pixels,
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        )
        .map_err(|e| format!("Failed to save PNG: {e}"))
    }
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::ScreenshotTick => {
            if state.screenshot_countdown > 0 {
                state.screenshot_countdown -= 1;
                if state.screenshot_countdown == 0 {
                    // macOS and Windows: an OS capture of the window with its
                    // decorations, once the content's size is known.
                    #[cfg(any(target_os = "macos", target_os = "windows"))]
                    {
                        return iced::window::latest().then(|opt_id| match opt_id {
                            Some(id) => iced::window::size(id).then(move |size| {
                                iced::window::scale_factor(id)
                                    .map(move |scale| Message::CaptureFrame(size, scale))
                            }),
                            None => failed_capture("the showcase has no window"),
                        });
                    }
                    // Linux (and other platforms): iced internal framebuffer
                    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                    {
                        return iced::window::latest().then(|opt_id| {
                            if let Some(id) = opt_id {
                                iced::window::screenshot(id).map(|s| {
                                    let bytes = s.rgba.to_vec();
                                    let w = s.size.width;
                                    let h = s.size.height;
                                    Message::ScreenshotCaptured(bytes, w, h, s.scale_factor)
                                })
                            } else {
                                failed_capture("the showcase has no window")
                            }
                        });
                    }
                }
            }
        }
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        Message::CaptureFrame(size, scale) => {
            if let Some(ref path) = state.screenshot_path {
                #[cfg(target_os = "macos")]
                let captured = capture_own_window_macos(path);
                #[cfg(target_os = "windows")]
                let captured = capture_own_window_windows(path);
                let checked = captured
                    .and_then(|()| image::image_dimensions(path).map_err(|e| e.to_string()))
                    .and_then(|(width, height)| {
                        let scale = f64::from(scale);
                        check_frame_capture(
                            (i64::from(width), i64::from(height)),
                            (
                                (f64::from(size.width) * scale).round() as i64,
                                (f64::from(size.height) * scale).round() as i64,
                            ),
                            scale,
                        )
                    });
                return match checked {
                    Ok(()) => {
                        eprintln!("Screenshot saved to {path}");
                        iced::exit()
                    }
                    Err(e) => failed_capture(&format!("{path}: {e}")),
                };
            }
        }
        Message::ScreenshotCaptured(bytes, width, height, scale) => {
            if let Some(ref path) = state.screenshot_path {
                let saved = check_content_capture((width, height), scale).and_then(|()| {
                    image::save_buffer(path, &bytes, width, height, image::ColorType::Rgba8)
                        .map_err(|e| e.to_string())
                });
                return match saved {
                    Ok(()) => {
                        eprintln!("Screenshot saved to {path}");
                        iced::exit()
                    }
                    Err(e) => failed_capture(&format!("{path}: {e}")),
                };
            }
            return iced::exit();
        }
        Message::Quit => return iced::exit(),
        // The whole tree surveyed, then the scale factor read, then written.
        Message::LayoutSettled => {
            return iced::advanced::widget::operate(Survey)
                .then(|()| iced::window::latest().and_then(iced::window::scale_factor))
                .map(Message::LayoutScaled);
        }
        Message::LayoutScaled(scale) => {
            let Some(cli) = CLI_ARGS.get() else {
                return iced::Task::none();
            };
            if let Some(path) = &cli.dump_layout {
                if let Err(error) = write_layout_dump(state, path, scale) {
                    eprintln!("ERROR: --dump-layout: {error}");
                    std::process::exit(1);
                }
                if !cli.capture {
                    return iced::exit();
                }
            }
        }
        // The query takes the focus, so typing reaches it at once.
        Message::Open(Overlay::CommandPalette) if state.overlay.is_none() => {
            update_inner(state, Message::Open(Overlay::CommandPalette));
            return iced::widget::operation::focus(PALETTE_QUERY_ID);
        }
        Message::Copy(contents) => return iced::clipboard::write(contents),
        Message::PaletteRun(action) => {
            state.overlay = None;
            return update(state, *action);
        }
        Message::PaletteSubmitted => {
            let first = palette_groups(state)
                .into_iter()
                .flat_map(|(_, entries)| entries)
                .next();
            if let Some(entry) = first {
                state.overlay = None;
                return update(state, entry.action);
            }
        }
        other => {
            update_inner(state, other);
        }
    }
    iced::Task::none()
}

fn update_inner(state: &mut State, message: Message) {
    match message {
        Message::TabSelected(tab) => {
            state.active_tab = tab;
        }
        Message::ThemeSelected(choice) => {
            let mode = state.color_mode;
            state.install_in_mode(choice, mode);
        }
        Message::ColorModeSelected(mode) => {
            let choice = state.current_choice.clone();
            state.install_in_mode(choice, mode);
        }
        Message::WidgetHovered(info) => {
            state.widget_info = info;
            state.hovered_element = None;
        }
        Message::ElementHovered(id) => {
            state.hovered_element = Some(id);
        }
        Message::WidgetUnhovered => {
            // Keep last info visible (like gpui showcase)
        }
        Message::ToggleSidePanel => state.side_panel_visible = !state.side_panel_visible,
        Message::ReloadSystemTheme => {
            native_theme::detect::invalidate_caches();
            state.rebuild_theme();
        }
        // One dialog at a time: a second shortcut leaves the open one be.
        Message::Open(overlay) => {
            if state.overlay.is_none() {
                state.overlay = Some(overlay);
                state.palette_query.clear();
            }
        }
        Message::CloseOverlay => state.overlay = None,
        Message::InspectorTabSelected(tab) => state.inspector_tab = tab,
        Message::SplitterPressed => state.splitter_dragging = true,
        Message::SplitterDragged(at) => {
            if state.splitter_dragging {
                state.side_panel_width = splitter_width(at.x, state.window_width);
            }
        }
        Message::SplitterReleased => state.splitter_dragging = false,
        Message::SplitterHovered(on) => state.splitter_hovered = on,
        Message::WindowResized(size) => {
            state.window_width = size.width;
            state.side_panel_width = splitter_width(state.side_panel_width, size.width);
        }
        Message::PaletteQueryChanged(query) => state.palette_query = query,
        Message::TextScaleSelected(scale) => state.accessibility.text_scaling_factor = scale.0,
        Message::ReduceMotionToggled(on) => state.accessibility.reduce_motion = on,
        Message::HighContrastToggled(on) => state.accessibility.high_contrast = on,
        Message::ReduceTransparencyToggled(on) => state.accessibility.reduce_transparency = on,
        Message::BasicHeld | Message::MenuOpened => {}
        Message::MenuTitleOpened(title) => state.menu_title_open = title,
        Message::BasicRadioSelected(i) => state.basic_radio = i,
        Message::BasicHintChanged(value) => state.basic_hint = value,
        Message::BasicTextChanged(value) => state.basic_text = value,
        Message::BasicFruitSelected(fruit) => state.basic_fruit = fruit,
        Message::BasicSliderChanged(v) => state.basic_slider = v,
        Message::BasicTextAreaAction(action) => state.basic_text_area.perform(action),
        Message::BasicSegmentSelected(i) => state.basic_segment = i,
        Message::BasicDetailsToggled => state.basic_details_open = !state.basic_details_open,
        Message::BasicMoreToggled => state.basic_more_open = !state.basic_more_open,
        #[cfg(feature = "iced_aw")]
        Message::BasicTabSelected(i) => state.basic_tab = i,
        Message::BasicListSelected(i) => state.basic_list_selected = Some(i),
        #[cfg(feature = "iced_aw")]
        Message::BasicNumberChanged(value) => state.basic_number = value,
        Message::BasicFocusedChanged(value) => state.basic_focused = value,
        Message::ButtonPressed => {
            state.button_press_count = state.button_press_count.saturating_add(1);
        }
        Message::TextInputChanged(value) => {
            state.text_input_value = value;
        }
        Message::EditorAction(action) => {
            state.text_editor_content.perform(action);
        }
        Message::CheckboxAToggled(v) => state.checkbox_a = v,
        Message::CheckboxBToggled(v) => state.checkbox_b = v,
        Message::CheckboxCToggled(v) => state.checkbox_c = v,
        Message::FruitSelected(fruit) => state.selected_fruit = Some(fruit),
        Message::TogglerToggled(v) => state.toggler_enabled = v,
        Message::PickListSelected(v) => state.pick_list_selected = Some(v),
        Message::ComboBoxSelected(v) => state.combo_selected = Some(v),
        Message::SliderChanged(v) => state.slider_value = v,
        Message::StepSliderChanged(v) => state.slider_step = v,
        Message::VSliderChanged(v) => state.vslider_value = v,
        Message::ProgressChanged(v) => state.progress_value = v,
        Message::PaneClicked(pane) => state.focused_pane = Some(pane),
        Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            state.panes.drop(pane, target);
            state.focused_pane = Some(pane);
        }
        // Picked and Canceled leave the layout as it was.
        Message::PaneDragged(_) => {}
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            state.panes.resize(split, ratio);
        }
        Message::PaneSplit(axis, pane) => {
            let label = state.pane_count.saturating_add(1);
            if let Some((new_pane, _)) = state.panes.split(axis, pane, label) {
                state.pane_count = label;
                state.focused_pane = Some(new_pane);
            }
        }
        Message::PaneClosed(pane) => {
            if let Some((_, sibling)) = state.panes.close(pane) {
                state.focused_pane = Some(sibling);
            }
        }
        Message::MarkdownLinkClicked(uri) => state.markdown_link = Some(uri),
        #[cfg(feature = "iced_aw")]
        Message::AwTabBarSelected(i) => state.aw_tab_bar_active = i,
        #[cfg(feature = "iced_aw")]
        Message::AwTabsSelected(i) => state.aw_tabs_active = i,
        #[cfg(feature = "iced_aw")]
        Message::AwSidebarSelected(i) => state.aw_sidebar_active = i,
        #[cfg(feature = "iced_aw")]
        Message::AwListSelected(index, value) => {
            state.aw_list_selected = Some(index);
            state.aw_last_action = format!("SelectionList: {value}");
        }
        #[cfg(feature = "iced_aw")]
        Message::AwCardToggled => state.aw_card_open = !state.aw_card_open,
        #[cfg(feature = "iced_aw")]
        Message::AwActionChosen(what) => state.aw_last_action = what,
        Message::IconSetSelected(choice) => state.choose_icon_set(choice),
        Message::ThemeWatcherTick if state.theme_change_flag.swap(false, Ordering::AcqRel) => {
            native_theme::detect::invalidate_caches();
            state.rebuild_theme();
        }
        Message::AnimationTick => {
            let tick_duration = Duration::from_millis(50);
            for (i, (_, anim_handles)) in state.animated_frames.iter().enumerate() {
                state.animated_frame_elapsed[i] += tick_duration;
                let frame_dur = Duration::from_millis(anim_handles.frame_duration_ms as u64);
                if state.animated_frame_elapsed[i] >= frame_dur {
                    state.animated_frame_elapsed[i] -= frame_dur;
                    state.animated_frame_indices[i] =
                        (state.animated_frame_indices[i] + 1) % anim_handles.handles.len();
                }
            }
        }
        _ => {} // Screenshot messages handled by update()
    }
}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

fn view(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let btn_pad = native_theme_iced::button_padding(resolved);
    let inp_pad = native_theme_iced::input_padding(resolved);

    let content = content_panel(state, btn_pad, inp_pad);
    let content = tagged("chrome.content", content);
    let body: Element<'_, Message> = if state.side_panel_visible {
        let body = row![
            tagged(
                "chrome.side_panel",
                container(probe(probes::SIDE_PANEL, Fill, side_panel(state)))
                    .width(Length::Fixed(state.side_panel_width))
                    .height(Fill)
            ),
            splitter(state),
            content,
        ]
        .height(Fill);
        // While the splitter is dragged, the pointer anywhere over the body
        // moves it, and letting go, or leaving the body, ends the drag.
        let area = mouse_area(body);
        if state.splitter_dragging {
            area.on_move(Message::SplitterDragged)
                .on_release(Message::SplitterReleased)
                .on_exit(Message::SplitterReleased)
                .interaction(iced::mouse::Interaction::ResizingHorizontally)
                .into()
        } else {
            area.into()
        }
    } else {
        content.into()
    };

    let window = column![menu_bar(state), toolbar(state)]
        .push(container(body).height(Fill))
        .push(status_bar(state));
    // The window's fill, `window.background_color`, under everything.
    let window = tagged(
        "chrome.window",
        container(window).width(Fill).height(Fill).style(surface(
            to_color(resolved.window.background_color),
            to_color(resolved.defaults.text_color),
        )),
    )
    .root();
    let page: Element<'_, Message> = match state.overlay {
        // No backdrop: the model states none, so the window stays as it is
        // under the dialog, and `opaque` keeps the pointer off it.
        Some(overlay) => stack![window, opaque(center(dialog(state, overlay)))].into(),
        None => window.into(),
    };
    match CLI_ARGS.get() {
        Some(cli) => held_page(
            page,
            cli.pointer,
            cli.press,
            cli.capture || cli.screenshot.is_some(),
        ),
        None => page,
    }
}

/// The content panel (spec S3): the page tab row at its top, under it the
/// theme-error banner where a theme failed to load, then the page, which
/// scrolls, inside `layout.window_margin`.
fn content_panel(state: &State, btn_pad: Padding, inp_pad: Padding) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let tab_content: Element<'_, Message> = match state.active_tab {
        Tab::Basic => view_basic(state, btn_pad, inp_pad),
        Tab::Buttons => view_buttons(state, btn_pad),
        Tab::TextInputs => view_text_inputs(state, inp_pad),
        Tab::Selection => view_selection(state),
        Tab::Range => view_range(state),
        Tab::Display => view_display(state),
        Tab::Layout => view_layout(state),
        Tab::Graphics => view_graphics(state),
        #[cfg(feature = "iced_aw")]
        Tab::Extra => view_extra(state),
        Tab::Icons => view_icons(state),
        Tab::ThemeMap => view_theme_map(state),
    };

    let mut panel = column![
        page_tabs(state),
        tagged("chrome.page_tabs.rule", separator_line(resolved))
    ]
    .width(Fill)
    .height(Fill);
    // The banner: the platform's own error colour, not the palette slot iced
    // derives from it, inside the window margin the page has.
    if let Some(ref msg) = state.error_message {
        let danger = to_color(resolved.defaults.danger_color);
        panel = panel.push(
            container(
                text(msg.as_str())
                    .color(danger)
                    .role(&ts.caption, resolved, a11y),
            )
            .padding(Padding::from(gap.window).bottom(0.0))
            .width(Fill),
        );
    }
    panel
        .push(
            scrollable(
                container(tab_content)
                    .padding(Padding::from(gap.window))
                    .width(Fill),
            )
            .direction(scrollable::Direction::Vertical(styles::scrollbar(resolved)))
            .style(styles::scrollable(resolved))
            .height(Fill),
        )
        .into()
}

/// The page as a capture shows it. With a `pointer` (`--pointer X,Y`), the
/// pointer held there, and with `press` the primary button held down
/// ([`HeldPointer`]). Otherwise, while `capturing` (`--capture`,
/// `--screenshot`), no pointer at all: the real one, wherever the desktop
/// left it, hovers nothing, opens no tooltip and fills no Widget Info, so a
/// capture shows every control at rest. Neither: the page as it is.
fn held_page<'a>(
    page: Element<'a, Message>,
    pointer: Option<(u16, u16)>,
    press: bool,
    capturing: bool,
) -> Element<'a, Message> {
    match (pointer, capturing) {
        (Some((x, y)), _) => Element::new(HeldPointer {
            content: page,
            at: Some(iced::Point::new(f32::from(x), f32::from(y))),
            press,
        }),
        (None, true) => Element::new(HeldPointer {
            content: page,
            at: None,
            press: false,
        }),
        (None, false) => page,
    }
}

// ---------------------------------------------------------------------------
// Held pointer (`--pointer`, `--press`)
// ---------------------------------------------------------------------------

/// How long [`HeldPointer`] holds the pointer before the press: long enough
/// for the window to open at its size and the page to be laid out under the
/// pointer. No repository script passes `--press`. The capture scripts
/// capture a showcase at least four seconds after starting it (`DELAY=3` in
/// `scripts/generate_screenshots_*.sh`, then `capture_showcase`'s `sleep 1` in
/// `scripts/capture_window.sh`), so a press three seconds after the first
/// frame lands about a second before such a capture; a capture with `--press`
/// should wait longer.
const HELD_PRESS_AFTER: Duration = Duration::from_secs(3);

/// The view with the pointer held at `at`, and with `press` the primary
/// button held down there: for a capture of a control hovered or pressed
/// where nothing can move the real pointer, as in a nested compositor.
///
/// iced takes the pointer from the window and offers no way to set it, so the
/// view is wrapped in this widget, which hands its content every event with
/// the cursor at `at` in place of the window's own and drops the window's own
/// mouse events. On the first redraw it delivers a cursor move to `at`, and
/// with `press`, once `HELD_PRESS_AFTER` has passed, a press of the primary
/// button there: the events iced's widgets read hover and press from
/// (iced_widget 0.14.2 `src/button.rs`, `Button::update`). With `at` `None`
/// the cursor is `Cursor::Unavailable` and nothing is delivered: the content
/// sees no pointer at all.
struct HeldPointer<'a> {
    content: Element<'a, Message>,
    at: Option<iced::Point>,
    press: bool,
}

impl HeldPointer<'_> {
    /// The cursor the content is drawn and updated with.
    fn cursor(&self) -> iced::mouse::Cursor {
        self.at.map_or(
            iced::mouse::Cursor::Unavailable,
            iced::mouse::Cursor::Available,
        )
    }
}

/// What [`HeldPointer`] has delivered, and when it first drew.
#[derive(Default)]
struct HeldState {
    started: Option<Instant>,
    moved: bool,
    pressed: bool,
}

impl iced::advanced::Widget<Message, Theme, iced::Renderer> for HeldPointer<'_> {
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<HeldState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(HeldState::default())
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        vec![iced::advanced::widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> iced::Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> iced::Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &iced::Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        match tree.children.first_mut() {
            Some(child) => self.content.as_widget_mut().layout(child, renderer, limits),
            None => iced::advanced::layout::Node::new(iced::Size::ZERO),
        }
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        if let Some(child) = tree.children.first() {
            self.content.as_widget().draw(
                child,
                renderer,
                theme,
                style,
                layout,
                self.cursor(),
                viewport,
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        if let Some(child) = tree.children.first_mut() {
            self.content
                .as_widget_mut()
                .operate(child, layout, renderer, operation);
        }
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        let cursor = self.cursor();
        // `State::downcast_mut` panics on a state of another type; this cannot.
        let iced::advanced::widget::tree::State::Some(any) = &mut tree.state else {
            return;
        };
        let Some(state) = any.downcast_mut::<HeldState>() else {
            return;
        };
        let Some(child) = tree.children.first_mut() else {
            return;
        };
        let content = self.content.as_widget_mut();
        if let iced::Event::Window(iced::window::Event::RedrawRequested(now)) = event {
            let started = *state.started.get_or_insert(*now);
            let mut held = Vec::new();
            if let Some(position) = self.at
                && !state.moved
            {
                held.push(iced::mouse::Event::CursorMoved { position });
                state.moved = true;
            }
            if self.press
                && self.at.is_some()
                && !state.pressed
                && now.saturating_duration_since(started) >= HELD_PRESS_AFTER
            {
                held.push(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left));
                state.pressed = true;
            }
            for e in held {
                content.update(
                    child,
                    &iced::Event::Mouse(e),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                );
            }
            if self.press && !state.pressed {
                shell.request_redraw();
            }
        }
        // The window's own pointer events are replaced by the held pointer.
        if !matches!(event, iced::Event::Mouse(_)) {
            content.update(
                child, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &iced::Renderer,
    ) -> iced::mouse::Interaction {
        match tree.children.first() {
            Some(child) => self.content.as_widget().mouse_interaction(
                child,
                layout,
                self.cursor(),
                viewport,
                renderer,
            ),
            None => iced::mouse::Interaction::None,
        }
    }

    /// The content's overlays, unwrapped: iced hands an overlay its events
    /// and draws it with the window's own cursor, not the held one, so a
    /// held pointer hovers and presses nothing inside an open menu or a
    /// tooltip bubble.
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &iced::Rectangle,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let child = tree.children.first_mut()?;
        self.content
            .as_widget_mut()
            .overlay(child, layout, renderer, viewport, translation)
    }
}

// ---------------------------------------------------------------------------
// Held menu (`--open-menu`)
// ---------------------------------------------------------------------------

/// The one menu `--open-menu` opens: the Theme menu's name on the command
/// line.
const OPEN_MENU_THEME: &str = "theme";

/// A menu bar with one of its menus opened on the first frame and held open,
/// for `--open-menu`: a capture and the layout dump show the menu with its
/// rows.
///
/// `iced_aw`'s `MenuBar` opens a menu only on a click of its title and
/// offers no way to open one otherwise (iced_aw 0.14.1
/// `src/widget/menu/menu_bar.rs:430-470`, `MenuBarTask::OpenOnClick`), so on
/// the first redraw this delivers a press and a release of the primary button
/// at the centre of the title `root` (the bar's roots are its layout's first
/// child's children, `menu_bar.rs:400`). The open menu is an overlay, which
/// iced hands the window's own pointer and events; [`HeldOverlay`] hands it
/// none, so the real pointer, wherever it is, neither closes it nor hovers a
/// row.
struct HeldMenu<'a> {
    content: Element<'a, Message>,
    /// The index of the menu's title among the bar's.
    root: usize,
    /// The list's id of the menu's panel.
    popup: &'static str,
    /// The panel's padding round its rows, as the menu is given it.
    padding: Padding,
}

/// Whether [`HeldMenu`] has opened its menu.
#[derive(Default)]
struct HeldMenuState {
    opened: bool,
}

impl iced::advanced::Widget<Message, Theme, iced::Renderer> for HeldMenu<'_> {
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<HeldMenuState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(HeldMenuState::default())
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        vec![iced::advanced::widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> iced::Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> iced::Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &iced::Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        match tree.children.first_mut() {
            Some(child) => self.content.as_widget_mut().layout(child, renderer, limits),
            None => iced::advanced::layout::Node::new(iced::Size::ZERO),
        }
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        if let Some(child) = tree.children.first() {
            self.content
                .as_widget()
                .draw(child, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn operate(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        if let Some(child) = tree.children.first_mut() {
            self.content
                .as_widget_mut()
                .operate(child, layout, renderer, operation);
        }
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        // `State::downcast_mut` panics on a state of another type; this cannot.
        let iced::advanced::widget::tree::State::Some(any) = &mut tree.state else {
            return;
        };
        let Some(state) = any.downcast_mut::<HeldMenuState>() else {
            return;
        };
        let Some(child) = tree.children.first_mut() else {
            return;
        };
        let content = self.content.as_widget_mut();
        if !state.opened
            && matches!(
                event,
                iced::Event::Window(iced::window::Event::RedrawRequested(_))
            )
            && let Some(title) = node_at(layout, &[0, self.root])
        {
            let at = iced::mouse::Cursor::Available(title.bounds().center());
            for click in [
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left),
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left),
            ] {
                uncaptured(shell, |held| {
                    content.update(
                        child,
                        &iced::Event::Mouse(click),
                        layout,
                        at,
                        renderer,
                        clipboard,
                        held,
                        viewport,
                    );
                });
            }
            state.opened = true;
            shell.request_redraw();
        }
        content.update(
            child, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &iced::Renderer,
    ) -> iced::mouse::Interaction {
        match tree.children.first() {
            Some(child) => self
                .content
                .as_widget()
                .mouse_interaction(child, layout, cursor, viewport, renderer),
            None => iced::mouse::Interaction::None,
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &iced::Rectangle,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let child = tree.children.first_mut()?;
        let inner =
            self.content
                .as_widget_mut()
                .overlay(child, layout, renderer, viewport, translation)?;
        Some(iced::advanced::overlay::Element::new(Box::new(
            HeldOverlay {
                inner,
                popup: self.popup,
                padding: self.padding,
            },
        )))
    }
}

/// The overlay of a [`HeldMenu`]: the open menu, drawn and updated with no
/// pointer and handed no pointer event and no resize, so it stays open and no
/// row shows hovered. What it does with any other event it does, but it
/// captures none
/// of them: `iced_aw`'s open menu captures every event while the pointer is
/// off it (iced_aw 0.14.1 `src/widget/menu/menu_tree.rs:606-611`), and iced
/// hands the rest of the window no event an overlay captured (iced_runtime
/// 0.14 `src/user_interface.rs:310-316`), so the page would see no redraw.
///
/// It records the menu's panel under the list's id `popup`: the rows
/// `padding` out on each side, where `iced_aw` fills and frames it
/// (`pad_rectangle(items_bounds, padding)`, iced_aw 0.14.1
/// `src/widget/menu/menu_tree.rs:764-780`); the open menu's layout is the
/// bar's, its titles', then its menus', each of which is its rows' slice
/// and then the rows' bounds (`src/widget/menu/menu_bar_overlay.rs:171-181`,
/// `menu_tree.rs:363-372`).
struct HeldOverlay<'a> {
    inner: iced::advanced::overlay::Element<'a, Message, Theme, iced::Renderer>,
    popup: &'static str,
    padding: Padding,
}

impl HeldOverlay<'_> {
    /// The menu's panel, where the menu is open.
    fn panel(&self, layout: iced::advanced::Layout<'_>) -> Option<iced::Rectangle> {
        let rows = node_at(layout, &[2, 0, 1])?.bounds();
        Some(iced::Rectangle {
            x: rows.x - self.padding.left,
            y: rows.y - self.padding.top,
            width: rows.width + self.padding.x(),
            height: rows.height + self.padding.y(),
        })
    }
}

impl iced::advanced::Overlay<Message, Theme, iced::Renderer> for HeldOverlay<'_> {
    fn layout(
        &mut self,
        renderer: &iced::Renderer,
        bounds: iced::Size,
    ) -> iced::advanced::layout::Node {
        self.inner.as_overlay_mut().layout(renderer, bounds)
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::mouse::Cursor,
    ) {
        if let Some(panel) = self.panel(layout) {
            DRAWING.with_borrow_mut(|drawing| drawing.insert(self.popup, panel));
        }
        self.inner.as_overlay().draw(
            renderer,
            theme,
            style,
            layout,
            iced::mouse::Cursor::Unavailable,
        );
    }

    fn operate(
        &mut self,
        layout: iced::advanced::Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        if let Some(panel) = self.panel(layout) {
            SURVEYED.with_borrow_mut(|surveyed| surveyed.insert(self.popup, panel));
        }
        self.inner
            .as_overlay_mut()
            .operate(layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
    ) {
        // No pointer event, and no resize, on which `iced_aw` closes the
        // menu (iced_aw 0.14.1 `src/widget/menu/menu_bar_overlay.rs:239-242`):
        // the window is resized as the compositor maps it.
        if matches!(
            event,
            iced::Event::Mouse(_) | iced::Event::Window(iced::window::Event::Resized(_))
        ) {
            return;
        }
        let inner = self.inner.as_overlay_mut();
        uncaptured(shell, |held| {
            inner.update(
                event,
                layout,
                iced::mouse::Cursor::Unavailable,
                renderer,
                clipboard,
                held,
            );
        });
    }

    fn index(&self) -> f32 {
        self.inner.as_overlay().index()
    }
}

// ---------------------------------------------------------------------------
// The open menu's title
// ---------------------------------------------------------------------------

/// A menu bar that tells the page which of its titles has its menu open
/// ([`Message::MenuTitleOpened`]), for that title to be drawn as the bar's
/// current item ([`menu_title`]).
///
/// `iced_aw`'s `MenuBar` keeps its open title to itself (`MenuBarState` is
/// private, iced_aw 0.14.1 `src/widget/menu/menu_bar.rs:51`) and styles only
/// the fill under it (`Style::path`); the title's label is the title's own
/// widget. This follows the bar by the bar's own rules: it is open while it
/// has an overlay (`menu_bar.rs:639-651`); the release that opens it, and a
/// pointer move over the bar while it is open, make the title under the
/// pointer the open one (`menu_bar.rs:452-481`, `try_open_menu` in
/// `src/widget/menu/common.rs:212-228`); the bar's titles are its layout's
/// first child's children (`menu_bar.rs:400`).
struct MenuTitles<'a> {
    content: Element<'a, Message>,
}

/// The title [`MenuTitles`] last reported open.
#[derive(Default)]
struct MenuTitlesState {
    open: Option<usize>,
}

impl iced::advanced::Widget<Message, Theme, iced::Renderer> for MenuTitles<'_> {
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<MenuTitlesState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(MenuTitlesState::default())
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        vec![iced::advanced::widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> iced::Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> iced::Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &iced::Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        match tree.children.first_mut() {
            Some(child) => self.content.as_widget_mut().layout(child, renderer, limits),
            None => iced::advanced::layout::Node::new(iced::Size::ZERO),
        }
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        if let Some(child) = tree.children.first() {
            self.content
                .as_widget()
                .draw(child, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn operate(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        if let Some(child) = tree.children.first_mut() {
            self.content
                .as_widget_mut()
                .operate(child, layout, renderer, operation);
        }
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        // `State::downcast_mut` panics on a state of another type; this cannot.
        let iced::advanced::widget::tree::State::Some(any) = &mut tree.state else {
            return;
        };
        let Some(state) = any.downcast_mut::<MenuTitlesState>() else {
            return;
        };
        let Some(child) = tree.children.first_mut() else {
            return;
        };
        let content = self.content.as_widget_mut();
        content.update(
            child, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
        let bar_open = content
            .overlay(child, layout, renderer, viewport, iced::Vector::ZERO)
            .is_some();
        let pointed = || {
            node_at(layout, &[0])
                .and_then(|titles| titles.children().position(|t| cursor.is_over(t.bounds())))
        };
        let open = match event {
            _ if !bar_open => None,
            iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left))
                if state.open.is_none() =>
            {
                pointed()
            }
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { .. }) => pointed().or(state.open),
            _ => state.open,
        };
        if open != state.open {
            state.open = open;
            shell.publish(Message::MenuTitleOpened(open));
        }
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &iced::Renderer,
    ) -> iced::mouse::Interaction {
        match tree.children.first() {
            Some(child) => self
                .content
                .as_widget()
                .mouse_interaction(child, layout, cursor, viewport, renderer),
            None => iced::mouse::Interaction::None,
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &iced::Rectangle,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let child = tree.children.first_mut()?;
        self.content
            .as_widget_mut()
            .overlay(child, layout, renderer, viewport, translation)
    }
}

/// Runs `update` on a shell of its own and hands `shell` what it asked for
/// -- its messages, a redraw, a new layout, a rebuild of the widgets -- but
/// not the capture of the event, so the event still reaches the rest of the
/// window.
fn uncaptured(
    shell: &mut iced::advanced::Shell<'_, Message>,
    update: impl FnOnce(&mut iced::advanced::Shell<'_, Message>),
) {
    let mut messages = Vec::new();
    let mut held = iced::advanced::Shell::new(&mut messages);
    update(&mut held);
    let redraw = held.redraw_request();
    let relayout = held.is_layout_invalid();
    let rebuild = held.are_widgets_invalid();
    drop(held);
    for message in messages {
        shell.publish(message);
    }
    shell.request_redraw_at(redraw);
    if relayout {
        shell.invalidate_layout();
    }
    if rebuild {
        shell.invalidate_widgets();
    }
}

// ---------------------------------------------------------------------------
// Chrome: the menu bar, toolbar, side panel, splitter, page tabs, status bar
// and dialogs, laid out as the gpui and egui showcases lay theirs out
// (parity inventory items 1-21, 25-27), every visual property from the theme
// ---------------------------------------------------------------------------

/// The window's title: this crate's name and version, built as the gpui and
/// egui showcases' `WINDOW_TITLE` is.
const WINDOW_TITLE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    " showcase"
);

/// This crate's name and version, as the About dialog states them.
const ABOUT_NAME_VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

/// The connector README's Compatibility table, at the tag of this version,
/// as the gpui showcase's `COMPATIBILITY_URL`: `#compatibility` is the anchor
/// of the README's `## Compatibility` heading.
const COMPATIBILITY_URL: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    "/blob/v",
    env!("CARGO_PKG_VERSION"),
    "/connectors/native-theme-iced/README.md#compatibility"
);

/// The side panel's width when the showcase opens, the gpui and egui
/// showcases' `LEFT_PANEL_WIDTH`. The model states no side-panel width; the
/// splitter changes it.
const LEFT_PANEL_WIDTH: f32 = 300.0;

/// The narrowest the splitter leaves the side panel, and the page beside it:
/// gpui-base's `PANEL_MIN_SIZE` (gpui-base 0.6.6 `src/resizable/mod.rs:14`),
/// the floor of the gpui showcase's resizable body. The model states none.
const PANEL_MIN_WIDTH: f32 = 100.0;

/// The weight of the inspector's title and section headings: semibold, the
/// gpui showcase's `font_semibold()`, which the egui showcase draws too
/// (parity decision 3). The model states no inspector heading.
const HEADING_WEIGHT: u16 = 600;

/// The side of a Widget Info colour swatch, the gpui and egui showcases'
/// `SWATCH_SIZE`. The model states none.
const SWATCH_SIZE: f32 = 16.0;

/// The `widget::Id` of the command palette's query, which takes the focus
/// when the palette opens.
const PALETTE_QUERY_ID: &str = "showcase-palette-query";

/// The side-panel width the splitter leaves at `at`, the pointer's distance
/// from the body's left edge: at least [`PANEL_MIN_WIDTH`], and at most the
/// window's `width` less [`PANEL_MIN_WIDTH`] for the page.
fn splitter_width(at: f32, width: f32) -> f32 {
    at.min(width - PANEL_MIN_WIDTH).max(PANEL_MIN_WIDTH)
}

/// The inspector's two views, in the order its tabs show them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InspectorTab {
    Widget,
    Theme,
}

impl InspectorTab {
    const ALL: [Self; 2] = [Self::Widget, Self::Theme];

    fn label(self) -> &'static str {
        match self {
            Self::Widget => "Widget",
            Self::Theme => "Theme",
        }
    }
}

/// A dialog over the window (spec §2.8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Overlay {
    CommandPalette,
    Preferences,
    About,
}

/// A text-scaling factor the Preferences dialog offers.
#[derive(Debug, Clone, Copy, PartialEq)]
struct TextScale(f32);

impl std::fmt::Display for TextScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "×{}", self.0)
    }
}

/// The factors the Preferences dialog offers: the gpui showcase's number
/// input's range and step, 1.0 to 2.25 by 0.25
/// (`showcase-gpui/demo.rs`, `TEXT_SCALE_MIN`, `TEXT_SCALE_MAX`,
/// `TEXT_SCALE_STEP`).
const TEXT_SCALES: [TextScale; 6] = [
    TextScale(1.0),
    TextScale(1.25),
    TextScale(1.5),
    TextScale(1.75),
    TextScale(2.0),
    TextScale(2.25),
];

/// The message a key binding sends: Ctrl (Cmd on macOS) with Q, B, K or the
/// comma, as the gpui showcase binds `secondary-q`, `-b`, `-k` and `-,`
/// (`showcase-gpui/app.rs`), and Escape, which closes a dialog. Only keys no
/// widget took reach here (`keyboard::listen`).
fn shortcut(event: iced::keyboard::Event) -> Option<Message> {
    use iced::keyboard::{Event, Key, key::Named};
    let Event::KeyPressed { key, modifiers, .. } = event else {
        return None;
    };
    match key.as_ref() {
        Key::Named(Named::Escape) => Some(Message::CloseOverlay),
        Key::Character(c) if modifiers.command() => match c {
            "q" => Some(Message::Quit),
            "b" => Some(Message::ToggleSidePanel),
            "k" => Some(Message::Open(Overlay::CommandPalette)),
            "," => Some(Message::Open(Overlay::Preferences)),
            _ => None,
        },
        _ => None,
    }
}

/// A key binding as the menus and tooltips spell it: Ctrl and the key, ⌘ on
/// macOS.
fn binding(key: &str) -> String {
    if cfg!(target_os = "macos") {
        format!("⌘{key}")
    } else {
        format!("Ctrl+{key}")
    }
}

/// The chrome's icons of the chosen icon theme, loaded by the names the gpui
/// showcase gives its gpui-component icons in each set
/// (native-theme-gpui `src/icons.rs`), or by `IconRole` where the gpui
/// showcase loads by role. `None` where the set has no such icon: the control
/// then shows its text, never another set's icon.
#[derive(Default)]
struct ChromeIcons {
    /// `SquareTerminal`: the command palette.
    palette: Option<IconData>,
    /// `RotateCw`: Reload System Theme.
    reload: Option<IconData>,
    /// `IconRole::ActionSettings`: Preferences.
    preferences: Option<IconData>,
    /// `PanelLeft`: the side-panel toggle.
    panel: Option<IconData>,
    /// `IconRole::WindowClose`: a dialog's close button.
    close: Option<IconData>,
    /// `IconRole::ActionCopy`, `ActionPaste`, `ActionDelete`: the Basic
    /// page's icon buttons.
    copy: Option<IconData>,
    paste: Option<IconData>,
    delete: Option<IconData>,
    /// `IconRole::FolderOpen`: the Basic page's icons at the theme's sizes.
    folder: Option<IconData>,
    /// Whether they are an OS icon theme's, drawn in their own colours;
    /// otherwise a bundled set's, drawn in their label's colour.
    system: bool,
}

/// `PanelLeft`'s freedesktop name: `sidebar-show` on the GTK desktops,
/// `sidebar-expand-left` on the others (native-theme-gpui `src/icons.rs`,
/// `freedesktop_name_for_gpui_icon`).
#[cfg(target_os = "linux")]
fn panel_left_name() -> &'static str {
    use native_theme::detect::LinuxDesktop;
    match native_theme::detect::detect_linux_desktop() {
        LinuxDesktop::Gnome
        | LinuxDesktop::Budgie
        | LinuxDesktop::Cinnamon
        | LinuxDesktop::Mate
        | LinuxDesktop::Xfce => "sidebar-show",
        _ => "sidebar-expand-left",
    }
}

#[cfg(not(target_os = "linux"))]
fn panel_left_name() -> &'static str {
    "sidebar-expand-left"
}

/// Load the chrome's icons from `choice`, as `load_all_icons` loads the Icons
/// page's: a freedesktop theme's recoloured where they are symbolic.
fn load_chrome_icons(
    choice: &IconSetChoice,
    resolved: &ResolvedTheme,
    theme_icon_set: IconSet,
) -> ChromeIcons {
    let set = choice.effective_icon_set(theme_icon_set);
    let theme = choice.freedesktop_theme();
    let tc = resolved.defaults.text_color;
    let fg = Some([tc.r, tc.g, tc.b]);
    let freedesktop = |loader: FreedesktopLoader<'_>| {
        let loader = loader.color_opt(fg);
        match theme {
            Some(t) => loader.theme(t).load(),
            None => loader.load(),
        }
    };
    // By gpui-component name: its Lucide, Material and freedesktop names.
    let named = |lucide: &str, material: &str, desktop: &str| match set {
        IconSet::Lucide => LucideLoader::new(lucide).load(),
        IconSet::Material => MaterialLoader::new(material).load(),
        IconSet::Freedesktop => freedesktop(FreedesktopLoader::new(desktop)),
        _ => None,
    };
    let by_role = |role: IconRole| match set {
        IconSet::Freedesktop => freedesktop(FreedesktopLoader::new(role)),
        IconSet::Material => MaterialLoader::new(role).load(),
        IconSet::Lucide => LucideLoader::new(role).load(),
        IconSet::SfSymbols => SfSymbolsLoader::new(role).color_opt(fg).load(),
        IconSet::SegoeIcons => SegoeIconsLoader::new(role).color_opt(fg).load(),
        _ => None,
    };
    ChromeIcons {
        palette: named("square-terminal", "terminal", "utilities-terminal"),
        reload: named("rotate-cw", "rotate_right", "object-rotate-right"),
        preferences: by_role(IconRole::ActionSettings),
        panel: named("panel-left", "side_navigation", panel_left_name()),
        close: by_role(IconRole::WindowClose),
        copy: by_role(IconRole::ActionCopy),
        paste: by_role(IconRole::ActionPaste),
        delete: by_role(IconRole::ActionDelete),
        folder: by_role(IconRole::FolderOpen),
        system: matches!(
            set,
            IconSet::Freedesktop | IconSet::SfSymbols | IconSet::SegoeIcons
        ),
    }
}

/// `icon` at `size`: an OS theme's in its own colours, a bundled set's in
/// `color`, its label's colour.
fn chrome_icon<'a>(
    icon: &IconData,
    size: f32,
    system: bool,
    color: Color,
) -> Option<Element<'a, Message>> {
    match icon {
        IconData::Svg(_) => {
            native_theme_iced::icons::to_svg_handle(icon, (!system).then_some(color)).map(
                |handle| {
                    svg(handle)
                        .width(Length::Fixed(size))
                        .height(Length::Fixed(size))
                        .into()
                },
            )
        }
        IconData::Rgba { .. } => native_theme_iced::icons::to_image_handle(icon).map(|handle| {
            iced::widget::image(handle)
                .width(Length::Fixed(size))
                .height(Length::Fixed(size))
                .into()
        }),
        _ => None,
    }
}

/// A flat button in the platform's button state colours, built from the
/// leaves the gpui connector's `variants::ghost_button` takes: no fill and no
/// frame at rest, its label `button.font.color`; `button.hover_background`
/// and `.hover_text_color` under the pointer; `button.active_background`
/// (the hover fill where the theme states none) and `.active_text_color`
/// while pressed, and while `selected`, as a toggle that is on;
/// `button.disabled_text_color` disabled; rounded by
/// `button.border.corner_radius`. The model states no flat button, so it
/// takes the button's own states.
fn ghost_button(
    resolved: &ResolvedTheme,
    selected: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style + use<> {
    let b = &resolved.button;
    let label = to_color(b.font.color);
    let hover = to_color(b.hover_background);
    let hover_label = to_color(b.hover_text_color);
    let pressed = to_color(b.active_background.unwrap_or(b.hover_background));
    let pressed_label = to_color(b.active_text_color);
    let disabled_label = to_color(b.disabled_text_color);
    let radius = b.border.corner_radius;
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Disabled => (None, disabled_label),
            _ if selected => (Some(pressed), pressed_label),
            button::Status::Pressed => (Some(pressed), pressed_label),
            button::Status::Hovered => (Some(hover), hover_label),
            button::Status::Active => (None, label),
        };
        button::Style {
            background: background.map(iced::Background::Color),
            text_color,
            border: iced::Border::default().rounded(radius),
            ..button::Style::default()
        }
    }
}

/// The padding of a flat button: the sides `button.border.padding` states,
/// iced's own button padding where it states none. No line width is added:
/// a flat button draws no border.
fn ghost_padding(resolved: &ResolvedTheme) -> Padding {
    native_theme_iced::padding_or(&resolved.button.border.padding, button::DEFAULT_PADDING)
}

/// A menu's title and its rows, as the model's menu item (platform-facts
/// §2.6): no fill at rest, its label `menu.font.color`;
/// `menu.hover_background` and `.hover_text_color` under the pointer and
/// while pressed; `menu.disabled_text_color` disabled; rounded by
/// `menu.border.corner_radius`, 0 where the items are rectangular.
fn menu_row(resolved: &ResolvedTheme) -> impl Fn(&Theme, button::Status) -> button::Style + use<> {
    let m = &resolved.menu;
    let label = to_color(m.font.color);
    let hover = to_color(m.hover_background);
    let hover_label = to_color(m.hover_text_color);
    let disabled_label = to_color(m.disabled_text_color);
    let radius = m.border.corner_radius;
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Active => (None, label),
            button::Status::Hovered | button::Status::Pressed => (Some(hover), hover_label),
            button::Status::Disabled => (None, disabled_label),
        };
        button::Style {
            background: background.map(iced::Background::Color),
            text_color,
            border: iced::Border::default().rounded(radius),
            ..button::Style::default()
        }
    }
}

/// A menu bar's title: a menu item ([`menu_row`]); while its menu is `open`,
/// the bar's current item, drawn as a hovered title whether or not the
/// pointer is on it, as a desktop menu bar marks the item whose menu is
/// open: `menu.hover_background` under `menu.hover_text_color`. `iced_aw`
/// marks it only by handing the titles a pointer at the open one's centre
/// as its menu takes an event (`DrawPath::FakeHovering`, the bar's
/// default, iced_aw 0.14.1 `src/widget/menu/menu_bar.rs:143`,
/// `src/widget/menu/menu_bar_overlay.rs:382-422`), which a held menu
/// ([`HeldOverlay`]) is never handed.
fn menu_title(
    resolved: &ResolvedTheme,
    open: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style + use<> {
    let row = menu_row(resolved);
    move |theme, status| {
        row(
            theme,
            match status {
                button::Status::Active if open => button::Status::Hovered,
                other => other,
            },
        )
    }
}

/// A tab of the page and inspector tab rows, as the model's tab
/// (platform-facts §2.11): `tab.background_color` and `tab.font.color` at
/// rest; `tab.active_background` and `.active_text_color` while it is the
/// open one; `tab.hover_background` (the rest fill where the theme states
/// none) and `.hover_text_color` under the pointer. `tab.border` frames the
/// open tab only, rounding its top corners, as `styles::aw::tab_bar` does
/// (platform-facts §2.11): the model states no outline for another tab, so
/// it has none, and square corners.
fn tab_style(
    resolved: &ResolvedTheme,
    selected: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style + use<> {
    let t = &resolved.tab;
    let rest = to_color(t.background_color);
    let label = to_color(t.font.color);
    let active = to_color(t.active_background);
    let active_label = to_color(t.active_text_color);
    let hover = to_color(t.hover_background.unwrap_or(t.background_color));
    let hover_label = to_color(t.hover_text_color);
    let border = if selected {
        iced::Border {
            color: to_color(t.border.color),
            width: t.border.line_width,
            radius: iced::border::Radius::new(0.0).top(t.border.corner_radius),
        }
    } else {
        iced::Border::default()
    };
    move |_theme, status| {
        let (background, text_color) = match status {
            _ if selected => (active, active_label),
            button::Status::Hovered | button::Status::Pressed => (hover, hover_label),
            button::Status::Active | button::Status::Disabled => (rest, label),
        };
        button::Style {
            background: Some(iced::Background::Color(background)),
            text_color,
            border,
            ..button::Style::default()
        }
    }
}

/// A list row, as the model's list (platform-facts §2.15): no fill of its
/// own over `list.background_color`, its label `list.item_font.color`;
/// `list.selection_background` and `.selection_text_color` while selected;
/// `list.hover_background` and `.hover_text_color` under the pointer;
/// `list.disabled_text_color` disabled. Square: the model rounds the list's
/// frame, not its rows.
fn list_row(
    resolved: &ResolvedTheme,
    selected: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style + use<> {
    let l = &resolved.list;
    let label = to_color(l.item_font.color);
    let selection = to_color(l.selection_background);
    let selection_label = to_color(l.selection_text_color);
    let hover = to_color(l.hover_background);
    let hover_label = to_color(l.hover_text_color);
    let disabled_label = to_color(l.disabled_text_color);
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Disabled => (None, disabled_label),
            _ if selected => (Some(selection), selection_label),
            button::Status::Hovered | button::Status::Pressed => (Some(hover), hover_label),
            button::Status::Active => (None, label),
        };
        button::Style {
            background: background.map(iced::Background::Color),
            text_color,
            ..button::Style::default()
        }
    }
}

/// A horizontal separator: `separator.line_width` thick, in
/// `separator.line_color` (`styles::rule`).
fn separator_line<'a>(resolved: &ResolvedTheme) -> Element<'a, Message> {
    rule::horizontal(resolved.separator.line_width)
        .style(styles::rule(resolved))
        .into()
}

/// The separator's style (`styles::rule`) in `color`: a line a widget's own
/// leaf colours, as the status bar's edge and a menu's separator.
fn line_style(resolved: &ResolvedTheme, color: Color) -> impl Fn(&Theme) -> rule::Style + use<> {
    let separator = styles::rule(resolved);
    move |theme| rule::Style {
        color,
        ..separator(theme)
    }
}

/// A container filled with `background`, its text in `text_color`.
fn surface(background: Color, text_color: Color) -> impl Fn(&Theme) -> container::Style + use<> {
    move |_theme| container::Style {
        text_color: Some(text_color),
        background: Some(iced::Background::Color(background)),
        ..container::Style::default()
    }
}

/// `content` with a tooltip reading `label` and, where it has one, the key
/// binding of `key`, in the platform's tooltip (`styles::tooltip`,
/// `tooltip.font`, `tooltip.max_width`, and `tooltip.border.padding` where
/// iced can carry it, [`tooltip_padding`]).
fn chrome_tooltip<'a>(
    state: &'a State,
    content: impl Into<Element<'a, Message>>,
    label: &'static str,
    key: Option<&'static str>,
) -> Element<'a, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let font = &resolved.tooltip.font;
    let mut tip = row![text(label).themed(font, resolved, a11y)].spacing(gap.widget);
    if let Some(key) = key {
        tip = tip.push(text(binding(key)).themed(font, resolved, a11y));
    }
    let tip = tooltip(
        content,
        container(tip).max_width(resolved.tooltip.max_width),
        tooltip::Position::Bottom,
    )
    .gap(gap.widget)
    .style(styles::tooltip(resolved));
    match tooltip_padding(resolved) {
        Some(padding) => tip.padding(padding).into(),
        None => tip.into(),
    }
}

/// A flat button showing `icon` at `size`, or `label` in `font` where the
/// icon theme has no icon for it, with a tooltip reading `label` and the key
/// binding of `key`. `padding` is the button's; `selected` shows it on.
struct IconButton<'a> {
    icon: &'a Option<IconData>,
    size: f32,
    label: &'static str,
    key: Option<&'static str>,
    font: &'a ResolvedFontSpec,
    padding: Padding,
    selected: bool,
    action: Message,
    /// The list's ids of the button and of its icon, where the list names
    /// the button.
    tags: Option<(&'static str, &'static str)>,
}

fn icon_button<'a>(state: &'a State, spec: IconButton<'a>) -> Element<'a, Message> {
    let resolved = &state.current_resolved;
    let b = &resolved.button;
    let color = to_color(if spec.selected {
        b.active_text_color
    } else {
        b.font.color
    });
    let button = match spec
        .icon
        .as_ref()
        .and_then(|icon| chrome_icon(icon, spec.size, state.chrome_icons.system, color))
    {
        Some(icon) => {
            let icon: Element<'a, Message> = match spec.tags {
                Some((_, icon_id)) => tagged(icon_id, icon).into(),
                None => icon,
            };
            button(icon)
                .padding(spec.padding)
                .style(ghost_button(resolved, spec.selected))
                .on_press(spec.action)
        }
        None => button(text(spec.label).themed(spec.font, resolved, &state.accessibility))
            .padding(spec.padding)
            .style(ghost_button(resolved, spec.selected))
            .on_press(spec.action),
    };
    let button: Element<'a, Message> = match spec.tags {
        Some((button_id, _)) => tagged(button_id, button).into(),
        None => button.into(),
    };
    chrome_tooltip(state, button, spec.label, spec.key)
}

/// The menu-bar row (spec S8), the gpui showcase's menus with the same items,
/// separators and key bindings, in `iced_aw`'s `MenuBar`: each title and row
/// a button in [`menu_row`], in `menu.font`, padded by `menu.border.padding`
/// (iced's button padding on a side it leaves unstated), `menu.row_height`
/// tall where the theme states one; the rows' separators
/// `separator.line_width` thick in `menu.separator_color`. The row has the
/// window's `window.background_color` and no frame; a menu's panel is
/// `menu.background_color`, framed by the popover's border (platform-facts
/// §2.6: the popup border is §2.16's); its open title the bar's current
/// item ([`menu_title`]). The row's sides are `layout.container_margin`, as
/// the gpui showcase's: the model states no menu-bar inset.
fn menu_bar(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let m = &resolved.menu;
    let pad = native_theme_iced::padding_or(&m.border.padding, button::DEFAULT_PADDING);
    // The bar's `index`th title.
    let title = |index: usize, id: &'static str, label: &'static str| {
        tagged(
            id,
            button(text(label).themed(&m.font, resolved, a11y))
                .padding(pad)
                .style(menu_title(resolved, state.menu_title_open == Some(index)))
                .on_press(Message::MenuOpened),
        )
    };
    // A row of a menu, tagged `id` where the list names it (the Theme
    // menu's rows, the one menu the three showcases share), its key binding
    // tagged `<id>.shortcut` where it has one.
    // The label and the shortcut are one line each: the menu is as wide as
    // its widest row ([`menu_width`]), so a row never wraps.
    let tagged_entry = |id: Option<&'static str>,
                        label: &'static str,
                        key: Option<&'static str>,
                        action: Message| {
        let shortcut = key.map(|key| {
            let shortcut = text(binding(key))
                .themed(&m.font, resolved, a11y)
                .wrapping(text::Wrapping::None);
            match id {
                Some("chrome.menu.theme.preferences") => {
                    Element::from(tagged("chrome.menu.theme.preferences.shortcut", shortcut))
                }
                _ => shortcut.into(),
            }
        });
        // A row `menu.row_height` tall holds its line centred in it, as the
        // gpui and egui showcases' rows do, where iced's button would lay it
        // out from the top: the model states the row's height ("height of a
        // single item row", docs/platform-facts.md §2 dimension rules), and
        // a menu item's label sits in the middle of it.
        let line_height = match m.row_height {
            Some(_) => Fill,
            None => Length::Shrink,
        };
        // The shortcut flush right, at least `layout.widget_gap` from the
        // label: the space between them fills the rest of the row, which is
        // that gap in the widest row.
        let entry = button(
            row![
                text(label)
                    .themed(&m.font, resolved, a11y)
                    .wrapping(text::Wrapping::None),
                space().width(Fill),
                shortcut,
            ]
            .align_y(iced::Center)
            .height(line_height),
        )
        .padding(pad)
        .width(Fill)
        .style(menu_row(resolved))
        .on_press(action);
        let entry = match m.row_height {
            Some(h) => entry.height(Length::Fixed(h)),
            None => entry,
        };
        Item::new(match id {
            Some(id) => Element::from(tagged(id, entry)),
            None => entry.into(),
        })
    };
    let separator = |id: Option<&'static str>| {
        let line = rule::horizontal(resolved.separator.line_width)
            .style(line_style(resolved, to_color(m.separator_color)));
        Item::new(match id {
            Some(id) => Element::from(tagged(id, line)),
            None => line.into(),
        })
    };
    let popup_pad = menu_popup_padding(resolved);
    // A menu of `entries`, as wide as its widest row, its rows
    // `popup_pad.top` under the title, so the panel, which iced_aw pads out
    // from the rows, starts at the title's foot.
    let drop = |entries: Vec<MenuEntry>| {
        let width = menu_width(
            entries.iter().filter_map(MenuEntry::texts),
            theme_font(&m.font),
            scaled_text_size(m.font.size, a11y),
            pad,
            gap.widget,
        );
        let items = entries
            .into_iter()
            .map(|entry| match entry {
                MenuEntry::Row {
                    id,
                    label,
                    key,
                    action,
                } => tagged_entry(id, label, key, action),
                MenuEntry::Separator(id) => separator(id),
            })
            .collect();
        Menu::new(items)
            .width(width)
            .offset(popup_pad.top)
            .padding(popup_pad)
    };
    let entry = |label, key, action| MenuEntry::Row {
        id: None,
        label,
        key,
        action,
    };
    let theme_row = |id, label, action| MenuEntry::Row {
        id: Some(id),
        label,
        key: None,
        action,
    };

    let mut pages: Vec<MenuEntry> = Tab::ALL
        .iter()
        .map(|&tab| entry(tab.label(), None, Message::TabSelected(tab)))
        .collect();
    pages.push(MenuEntry::Separator(None));
    pages.push(entry(
        "Toggle Side Panel",
        Some("B"),
        Message::ToggleSidePanel,
    ));
    pages.push(entry(
        "Command Palette",
        Some("K"),
        Message::Open(Overlay::CommandPalette),
    ));

    let bar = MenuBar::new(vec![
        Item::with_menu(
            title(0, "chrome.menu_bar.file", "File"),
            drop(vec![entry("Quit", Some("Q"), Message::Quit)]),
        ),
        Item::with_menu(title(1, "chrome.menu_bar.view", "View"), drop(pages)),
        Item::with_menu(
            title(2, "chrome.menu_bar.theme", "Theme"),
            drop(vec![
                theme_row(
                    "chrome.menu.theme.reload",
                    "Reload System Theme",
                    Message::ReloadSystemTheme,
                ),
                MenuEntry::Separator(Some("chrome.menu.theme.separator_1")),
                theme_row(
                    "chrome.menu.theme.system",
                    "System",
                    Message::ColorModeSelected(AppColorMode::System),
                ),
                theme_row(
                    "chrome.menu.theme.light",
                    "Light",
                    Message::ColorModeSelected(AppColorMode::Light),
                ),
                theme_row(
                    "chrome.menu.theme.dark",
                    "Dark",
                    Message::ColorModeSelected(AppColorMode::Dark),
                ),
                MenuEntry::Separator(Some("chrome.menu.theme.separator_2")),
                MenuEntry::Row {
                    id: Some("chrome.menu.theme.preferences"),
                    label: "Preferences…",
                    key: Some(","),
                    action: Message::Open(Overlay::Preferences),
                },
            ]),
        ),
        Item::with_menu(
            title(3, "chrome.menu_bar.help", "Help"),
            drop(vec![entry("About", None, Message::Open(Overlay::About))]),
        ),
    ])
    .style(menu_bar_style(resolved));
    let bar = Element::new(MenuTitles {
        content: bar.into(),
    });
    // `--open-menu theme`: the Theme menu, the bar's third, held open.
    let bar: Element<'_, Message> = match CLI_ARGS.get().and_then(|cli| cli.open_menu.as_deref()) {
        Some(OPEN_MENU_THEME) => Element::new(HeldMenu {
            content: bar,
            root: 2,
            popup: "chrome.menu.theme",
            padding: popup_pad,
        }),
        _ => bar,
    };
    tagged(
        "chrome.menu_bar",
        container(bar)
            .padding(Padding::ZERO.left(gap.container).right(gap.container))
            .width(Fill),
    )
    .into()
}

/// The window's toolbar (spec §2.3, platform-facts §2.13): three flat
/// buttons in the gpui showcase's order -- the command palette, Reload
/// System Theme and Preferences -- their icons of the chosen icon theme at
/// `toolbar.icon_size`, in a row filled with `toolbar.background_color`,
/// padded by `toolbar.border.padding` where the theme states a side and by
/// `layout.container_margin` where it does not, its items `toolbar.item_gap`
/// apart, or `layout.widget_gap` apart where that is unstated, at least
/// `toolbar.bar_height` tall where the theme states one. Under its padding a
/// `toolbar.border.line_width` line in `toolbar.border.color`, as the
/// resolved theme states it. §2.13 has no row for that border: on the
/// presets it is `defaults.border`'s, which docs/inheritance-rules.toml hands
/// the toolbar, where Breeze draws no toolbar frame (`ToolBar_FrameWidth` 0,
/// cited in §2.13's padding row); the showcase draws what the theme holds.
fn toolbar(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let gap = Gaps::from_layout(&state.layout);
    let t = &resolved.toolbar;
    let icons = &state.chrome_icons;
    let pad = ghost_padding(resolved);
    let item = |tags, icon, label, key, action| {
        icon_button(
            state,
            IconButton {
                icon,
                size: t.icon_size,
                label,
                key,
                font: &t.font,
                padding: pad,
                selected: false,
                action,
                tags: Some(tags),
            },
        )
    };
    let items = row![
        item(
            (
                "chrome.toolbar.command_palette",
                "chrome.toolbar.command_palette.icon"
            ),
            &icons.palette,
            "Command Palette",
            Some("K"),
            Message::Open(Overlay::CommandPalette),
        ),
        item(
            (
                "chrome.toolbar.reload_theme",
                "chrome.toolbar.reload_theme.icon"
            ),
            &icons.reload,
            "Reload System Theme",
            None,
            Message::ReloadSystemTheme,
        ),
        item(
            (
                "chrome.toolbar.preferences",
                "chrome.toolbar.preferences.icon"
            ),
            &icons.preferences,
            "Preferences",
            Some(","),
            Message::Open(Overlay::Preferences),
        ),
    ]
    .spacing(t.item_gap.unwrap_or(gap.widget))
    .align_y(iced::Center);
    let bar = container(items)
        .padding(native_theme_iced::padding_or(
            &t.border.padding,
            Padding::from(gap.container),
        ))
        .width(Fill)
        .align_y(iced::Center)
        .style(surface(
            to_color(t.background_color),
            to_color(t.font.color),
        ));
    // `toolbar.bar_height` is the outer height, the line included.
    let bar = match t.bar_height {
        Some(h) => bar.height(Length::Fixed((h - t.border.line_width).max(0.0))),
        None => bar,
    };
    tagged(
        "chrome.toolbar",
        column![
            bar,
            rule::horizontal(t.border.line_width)
                .style(line_style(resolved, to_color(t.border.color)))
        ],
    )
    .into()
}

/// One of the theme settings: `label`, in `sidebar.font`, above `control`,
/// `layout.widget_gap` apart; `tags` are the list's ids of the label and of
/// the control.
fn setting<'a>(
    state: &'a State,
    tags: (&'static str, &'static str),
    label: &'static str,
    control: Element<'a, Message>,
) -> Element<'a, Message> {
    let gap = Gaps::from_layout(&state.layout);
    column![
        tagged(
            tags.0,
            text(label).themed(
                &state.current_resolved.sidebar.font,
                &state.current_resolved,
                &state.accessibility
            )
        ),
        tagged(tags.1, control),
    ]
    .spacing(gap.widget)
    .into()
}

/// A drop-down of the theme settings: as wide as the side panel's content,
/// in the model's combo box (`styles::pick_list`, `combo_box.font`,
/// `combo_box_padding`, at least `combo_box.min_height` tall, its arrow
/// `combo_box.arrow_icon_size`).
fn setting_picker<'a, T>(
    state: &'a State,
    options: Vec<T>,
    selected: Option<T>,
    on_select: fn(T) -> Message,
) -> Element<'a, Message>
where
    T: ToString + PartialEq + Clone + 'a,
{
    let resolved = &state.current_resolved;
    let size = scaled_text_size(resolved.combo_box.font.size, &state.accessibility);
    let pad = combo_box_padding(resolved);
    pick_list(options, selected, on_select)
        .handle(arrow_handle(resolved))
        .padding(pad)
        .text_line_height(control_line_height(
            resolved,
            size,
            resolved.combo_box.min_height,
            pad,
        ))
        .text_size(size)
        .font(theme_font(&resolved.combo_box.font))
        .style(styles::pick_list(resolved))
        .menu_style(styles::menu(resolved))
        .width(Fill)
        .into()
}

/// The side panel (spec S2): the theme settings -- Theme, Mode and Icon
/// theme, each labelled above its drop-down -- padded by
/// `layout.container_margin`; a separator from edge to edge; then the
/// inspector's tabs and its content, which fills the rest of the panel and
/// scrolls. Filled with `sidebar.background_color`, lettered in
/// `sidebar.font.color`.
///
/// The panel is as wide as the splitter leaves it, whatever it holds: its
/// scrollbar, `scrollbar.groove_width` wide where the platform's does not
/// overlay, is laid out inside that width and moves nothing beside it.
fn side_panel(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let gap = Gaps::from_layout(&state.layout);
    let s = &resolved.sidebar;
    let settings = column![
        setting(
            state,
            (
                "chrome.side_panel.settings.theme_label",
                "chrome.side_panel.settings.theme"
            ),
            "Theme",
            probe(
                probes::THEME,
                Fill,
                setting_picker(
                    state,
                    theme_choices(&state.default_label),
                    Some(state.current_choice.clone()),
                    Message::ThemeSelected,
                ),
            ),
        ),
        setting(
            state,
            (
                "chrome.side_panel.settings.mode_label",
                "chrome.side_panel.settings.mode"
            ),
            "Mode",
            probe(
                probes::COLOR_MODE,
                Fill,
                setting_picker(
                    state,
                    AppColorMode::ALL.to_vec(),
                    Some(state.color_mode),
                    Message::ColorModeSelected,
                ),
            ),
        ),
        setting(
            state,
            (
                "chrome.side_panel.settings.icon_theme_label",
                "chrome.side_panel.settings.icon_theme"
            ),
            "Icon theme",
            probe(
                probes::ICON_THEME,
                Fill,
                setting_picker(
                    state,
                    state.icon_set_choices.clone(),
                    Some(state.icon_set_choice.clone()),
                    Message::IconSetSelected,
                ),
            ),
        ),
    ]
    .spacing(gap.widget);
    let settings = container(tagged("chrome.side_panel.settings", settings))
        .padding(Padding::from(gap.container))
        .width(Fill);
    let body = match state.inspector_tab {
        InspectorTab::Widget => inspector_widget(state),
        InspectorTab::Theme => inspector_theme(state),
    };
    // The inspector's two tabs, Widget and Theme, first and second.
    const INSPECTOR_TAB_IDS: [&str; 2] = [
        "chrome.side_panel.inspector_tabs.widget",
        "chrome.side_panel.inspector_tabs.theme",
    ];
    container(
        column![
            settings,
            tagged("chrome.side_panel.separator", separator_line(resolved)),
            tab_row(
                state,
                ("chrome.side_panel.inspector_tabs", INSPECTOR_TABS_ID),
                InspectorTab::ALL
                    .iter()
                    .enumerate()
                    .map(|(i, &tab)| TabSpec {
                        label: tab.label(),
                        open: tab == state.inspector_tab,
                        message: Message::InspectorTabSelected(tab),
                        tag: INSPECTOR_TAB_IDS.get(i).copied(),
                    })
                    .collect(),
                None,
            ),
            tagged(
                "chrome.side_panel.inspector",
                scrollable(
                    container(body)
                        .padding(Padding::from(gap.container))
                        .width(Fill),
                )
                .direction(scrollable::Direction::Vertical(styles::scrollbar(resolved)))
                .style(styles::scrollable(resolved))
                .width(Fill)
                .height(Fill)
            ),
        ]
        .width(Fill)
        .height(Fill),
    )
    .style(surface(
        to_color(s.background_color),
        to_color(s.font.color),
    ))
    .width(Fill)
    .height(Fill)
    .into()
}

/// One tab of a [`tab_row`]: its label, whether it is the open one, the
/// message a press sends, and its id in the list where the list names it.
struct TabSpec {
    label: &'static str,
    open: bool,
    message: Message,
    tag: Option<&'static str>,
}

/// A tab row, the page tabs' and the inspector's: each tab a button in
/// [`tab_style`], labelled in `tab.font`, padded by `tab.border.padding`
/// inside its border (iced's button padding on a side it leaves unstated),
/// at least `tab.min_width` by `tab.min_height`, on a strip of
/// `tab.bar_background` whose sides are `layout.container_margin`, as the
/// gpui showcase's tab bars are inset. More tabs than the strip is wide
/// scroll sideways, with no bar of their own; `trailing`, where given, stays
/// at the strip's right end. `tag` is the strip's id in the list, `id` its
/// scrollable's widget id.
fn tab_row<'a>(
    state: &'a State,
    (tag, id): (&'static str, &'static str),
    tabs: Vec<TabSpec>,
    trailing: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let t = &resolved.tab;
    let pad = native_theme_iced::padding_inside_border(&t.border, button::DEFAULT_PADDING);
    let min = iced::Size::new(
        (t.min_width - pad.x()).max(0.0),
        (t.min_height - pad.y()).max(0.0),
    );
    let tabs = tabs.into_iter().map(|spec| {
        let tab = button(at_least(
            text(spec.label).themed(&t.font, resolved, a11y),
            min,
        ))
        .padding(pad)
        .style(tab_style(resolved, spec.open))
        .on_press(spec.message);
        // The open tab's line, where the theme states one.
        let tab = native_theme_iced::tab_indicator(resolved, tab, spec.open);
        match spec.tag {
            Some(tag) => tagged(tag, tab).into(),
            None => tab,
        }
    });
    // The strip scrolls sideways with no bar: a bar laid out under tabs no
    // taller than their labels would cover them, and the page tabs' menu
    // reaches every tab.
    // tab.item_gap between the tabs, where stated; a row's own none otherwise.
    let strip = scrollable(match t.item_gap {
        Some(gap) => row(tabs).spacing(gap),
        None => row(tabs),
    })
    .id(id)
    .direction(scrollable::Direction::Horizontal(
        styles::scrollbar(resolved).width(0.0).scroller_width(0.0),
    ))
    .style(styles::scrollable(resolved))
    .width(Fill);
    let mut bar = row![strip].align_y(iced::Center);
    if let Some(trailing) = trailing {
        bar = bar.push(trailing);
    }
    tagged(
        tag,
        container(bar)
            .padding(Padding::ZERO.left(gap.container).right(gap.container))
            .width(Fill)
            .style(surface(to_color(t.bar_background), to_color(t.font.color))),
    )
    .into()
}

/// The content panel's tab row (spec S3): a tab per page, the shown one
/// open, and at its right end a flat button opening a menu of every page
/// (`iced_aw`'s `MenuBar`), as the gpui showcase's tab bar has.
fn page_tabs(state: &State) -> Element<'_, Message> {
    let tabs = Tab::ALL
        .iter()
        .map(|&tab| TabSpec {
            label: tab.label(),
            open: tab == state.active_tab,
            message: Message::TabSelected(tab),
            // The list names the two pages the three showcases open first.
            tag: match tab {
                Tab::Basic => Some("chrome.page_tabs.basic"),
                Tab::Buttons => Some("chrome.page_tabs.buttons"),
                _ => None,
            },
        })
        .collect();
    tab_row(
        state,
        ("chrome.page_tabs", TAB_STRIP_ID),
        tabs,
        Some(page_menu(state)),
    )
}

/// The page tabs' menu: a flat button showing the chosen icon theme's
/// `IconRole::NavDown` at `defaults.icon_sizes.small`, or "Pages" where the
/// set has none, opening a menu of every page in [`menu_row`]s. Unlike the
/// gpui showcase's, the shown page carries no check mark: the model's menu
/// states no checked row.
fn page_menu(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let m = &resolved.menu;
    let size = resolved.defaults.icon_sizes.small;
    let caret = state
        .loaded_icons
        .iter()
        .find(|loaded| loaded.role == IconRole::NavDown)
        .and_then(|loaded| loaded.data.as_ref())
        .and_then(|icon| {
            chrome_icon(
                icon,
                size,
                state.chrome_icons.system,
                to_color(resolved.button.font.color),
            )
        })
        .map(|icon| Element::from(tagged("chrome.page_tabs.menu.icon", icon)))
        .unwrap_or_else(|| {
            text("Pages")
                .themed(&resolved.tab.font, resolved, a11y)
                .into()
        });
    let pad = native_theme_iced::padding_or(&m.border.padding, button::DEFAULT_PADDING);
    let rows = Tab::ALL
        .iter()
        .map(|&tab| {
            Item::new(
                button(
                    text(tab.label())
                        .themed(&m.font, resolved, a11y)
                        .wrapping(text::Wrapping::None),
                )
                .padding(pad)
                .width(Fill)
                .style(menu_row(resolved))
                .on_press(Message::TabSelected(tab)),
            )
        })
        .collect();
    let width = menu_width(
        Tab::ALL.iter().map(|tab| (tab.label(), None)),
        theme_font(&m.font),
        scaled_text_size(m.font.size, a11y),
        pad,
        Gaps::from_layout(&state.layout).widget,
    );
    MenuBar::new(vec![Item::with_menu(
        tagged(
            "chrome.page_tabs.menu",
            button(caret)
                .padding(ghost_padding(resolved))
                .style(ghost_button(resolved, false))
                .on_press(Message::MenuOpened),
        ),
        Menu::new(rows)
            .width(width)
            .offset(menu_popup_padding(resolved).top)
            .padding(menu_popup_padding(resolved)),
    )])
    .style(menu_bar_style(resolved))
    .into()
}

/// The style of the window's menu bar and of the page tabs' menu, from the
/// model's leaves: the bar is the window's `window.background_color` with no
/// frame; a menu's panel `menu.background_color`, framed by the popover's
/// border (platform-facts §2.6: the popup border is §2.16's); the open
/// title's highlight `menu.hover_background`. The panel casts a shadow only
/// where `popover.border.shadow_enabled` states one; its geometry, the
/// bar's shadow and the highlight's border have no source -- the model has
/// a shadow colour but no shadow geometry, and no border round a highlighted
/// row -- and are `iced_aw`'s own (`menu_bar::primary`).
///
/// Not the connector's `styles::aw::menu`, which the Extra page shows: that
/// one fills the bar as a menu and frames the popup with `menu.border`,
/// which platform-facts §2.6 says is not the popup's.
fn menu_bar_style(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_aw::style::Status) -> iced_aw::style::menu_bar::Style + use<> {
    let bar = to_color(resolved.window.background_color);
    let panel = to_color(resolved.menu.background_color);
    let highlight = to_color(resolved.menu.hover_background);
    let popup = &resolved.popover.border;
    let frame = iced::Border {
        color: to_color(popup.color),
        width: popup.line_width,
        radius: popup.corner_radius.into(),
    };
    let shadow = popup.shadow_enabled;
    move |theme, status| {
        let iced = iced_aw::style::menu_bar::primary(theme, status);
        iced_aw::style::menu_bar::Style {
            bar_background: iced::Background::Color(bar),
            bar_border: iced::Border::default(),
            bar_shadow: iced.bar_shadow,
            menu_background: iced::Background::Color(panel),
            menu_border: frame,
            menu_shadow: if shadow {
                iced.menu_shadow
            } else {
                iced::Shadow::default()
            },
            path: iced::Background::Color(highlight),
            path_border: iced.path_border,
        }
    }
}

/// The splitter between the side panel and the page (platform-facts §2.17):
/// a line `splitter.divider_width` wide in `splitter.divider_color`, in
/// `splitter.hover_color` under the pointer and while dragged. A press on it
/// starts a drag, which the body follows ([`view`]).
fn splitter(state: &State) -> Element<'_, Message> {
    let sp = &state.current_resolved.splitter;
    let color = to_color(if state.splitter_hovered || state.splitter_dragging {
        sp.hover_color
    } else {
        sp.divider_color
    });
    let line = container(space())
        .width(Length::Fixed(sp.divider_width))
        .height(Fill)
        .style(move |_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(color)),
            ..container::Style::default()
        });
    tagged(
        "chrome.splitter",
        mouse_area(line)
            .on_press(Message::SplitterPressed)
            .on_enter(Message::SplitterHovered(true))
            .on_exit(Message::SplitterHovered(false))
            .interaction(iced::mouse::Interaction::ResizingHorizontally),
    )
    .into()
}

/// The desktop `native_theme::detect` recognises in `XDG_CURRENT_DESKTOP`, as
/// the gpui and egui showcases name it.
#[cfg(target_os = "linux")]
fn desktop() -> String {
    format!("{:?}", native_theme::detect::detect_linux_desktop())
}

/// The operating system: `native_theme::detect` names a desktop on Linux
/// only.
#[cfg(not(target_os = "linux"))]
fn desktop() -> String {
    std::env::consts::OS.to_string()
}

/// A font's size in the unit its source stated, as the gpui showcase's
/// `defined_size`.
fn defined_size(font: &ResolvedFontSpec) -> String {
    match font.defined_size {
        Some(native_theme::theme::FontSize::Pt(v)) => format!("{v}pt"),
        Some(native_theme::theme::FontSize::Px(v)) => format!("{v}px"),
        None => "(size not stated)".to_string(),
    }
}

/// The status bar's environment (spec §2.7), the gpui showcase's items: the
/// desktop, the preset and colour mode, the theme's `defaults.font` in the
/// unit its source stated, the text-scaling factor, and each accessibility
/// preference that is set, by its field name.
fn status_environment(state: &State) -> Vec<String> {
    let preset = match &state.current_choice {
        ThemeChoice::OsTheme(_) => state.default_label.clone(),
        ThemeChoice::Preset(key) => key.clone(),
    };
    let mode = if state.is_dark { "dark" } else { "light" };
    let font = &state.current_resolved.defaults.font;
    let prefs = &state.accessibility;
    let mut items = vec![
        desktop(),
        format!("{preset} {mode}"),
        format!("{} {}", font.family, defined_size(font)),
        format!("text ×{}", prefs.text_scaling_factor),
    ];
    let flags = [
        ("reduce_motion", prefs.reduce_motion),
        ("high_contrast", prefs.high_contrast),
        ("reduce_transparency", prefs.reduce_transparency),
    ];
    items.extend(
        flags
            .into_iter()
            .filter(|(_, set)| *set)
            .map(|(name, _)| name.to_string()),
    );
    items
}

/// The title Widget Info shows: its text's first line, `None` before any
/// hover.
fn info_title(info: &str) -> Option<&str> {
    info.lines().find(|line| !line.trim().is_empty())
}

/// The window's status bar (spec §2.7, platform-facts §2.14): at its left
/// the side-panel toggle, then the environment joined by " · "; at its right
/// the title of what Widget Info shows. Filled with
/// `status_bar.background_color`, lettered in `status_bar.font`, padded by
/// `status_bar.border.padding` (`layout.container_margin` on a side it
/// leaves unstated), its top edge a `status_bar.border.line_width` line in
/// `status_bar.border.color` outside that padding: the bar is its line, its
/// padding and its content, the model's box (padding lies inside the
/// border).
fn status_bar(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let s = &resolved.status_bar;
    // A flat button padded as the toolbar's are, by the button's own sides.
    let pad = ghost_padding(resolved);
    let toggle = icon_button(
        state,
        IconButton {
            icon: &state.chrome_icons.panel,
            size: resolved.defaults.icon_sizes.small,
            label: "Toggle Side Panel",
            key: Some("B"),
            font: &s.font,
            padding: pad,
            selected: state.side_panel_visible,
            action: Message::ToggleSidePanel,
            tags: Some(("chrome.status_bar.toggle", "chrome.status_bar.toggle.icon")),
        },
    );
    let mut items = row![
        toggle,
        tagged(
            "chrome.status_bar.environment",
            text(status_environment(state).join(" · ")).themed(&s.font, resolved, a11y)
        ),
        space().width(Fill),
    ]
    .spacing(gap.widget)
    .align_y(iced::Center);
    if let Some(title) = shown_title(state) {
        items = items.push(tagged(
            "chrome.status_bar.shown",
            text(title).themed(&s.font, resolved, a11y),
        ));
    }
    let bar = container(items)
        .padding(native_theme_iced::padding_or(
            &s.border.padding,
            Padding::from(gap.container),
        ))
        .width(Fill)
        .style(surface(
            to_color(s.background_color),
            to_color(s.font.color),
        ));
    tagged(
        "chrome.status_bar",
        column![
            rule::horizontal(s.border.line_width)
                .style(line_style(resolved, to_color(s.border.color))),
            bar,
        ],
    )
    .into()
}

/// The title of what Widget Info shows, for the status bar: the hovered
/// element's (`name · state`), or the first line of another page's widget
/// info; `None` before any hover.
fn shown_title(state: &State) -> Option<String> {
    match state.hovered_element.and_then(listed) {
        Some(element) => Some(element_title(element)),
        None => info_title(&state.widget_info).map(str::to_string),
    }
}

/// Text in `text_scale.caption`, the inspector's size (the gpui showcase's
/// `text_sm` and `text_xs`, parity rule R1), in `color`.
fn caption_text<'a>(
    state: &State,
    content: impl text::IntoFragment<'a>,
    color: Color,
) -> text::Text<'a> {
    let resolved = &state.current_resolved;
    text(content)
        .role(&resolved.text_scale.caption, resolved, &state.accessibility)
        .color(color)
}

/// An inspector heading: `text_scale.caption` at [`HEADING_WEIGHT`], in
/// `sidebar.font.color`.
fn inspector_heading<'a>(state: &State, content: impl text::IntoFragment<'a>) -> text::Text<'a> {
    let resolved = &state.current_resolved;
    let heading = ResolvedTextScaleEntry {
        weight: HEADING_WEIGHT,
        ..resolved.text_scale.caption.clone()
    };
    text(content)
        .role(&heading, resolved, &state.accessibility)
        .color(to_color(resolved.sidebar.font.color))
}

/// A name over its value, the gpui showcase's inspector row: the name in
/// `defaults.muted_color`, the value in `sidebar.font.color`, both in
/// `text_scale.caption`.
fn inspector_row<'a>(state: &State, name: String, value: String) -> Element<'a, Message> {
    let resolved = &state.current_resolved;
    column![
        caption_text(state, name, to_color(resolved.defaults.muted_color)),
        caption_text(state, value, to_color(resolved.sidebar.font.color)),
    ]
    .into()
}

/// A colour line of Widget Info, the gpui showcase's `swatch`: a
/// [`SWATCH_SIZE`] square of `color`, framed in `defaults.border`'s colour,
/// width and radius, beside `label`.
fn inspector_swatch<'a>(state: &State, label: String, color: Color) -> Element<'a, Message> {
    let resolved = &state.current_resolved;
    let gap = Gaps::from_layout(&state.layout);
    let d = &resolved.defaults.border;
    let frame = iced::Border {
        color: to_color(d.color),
        width: d.line_width,
        radius: d.corner_radius.into(),
    };
    row![
        container(space())
            .width(Length::Fixed(SWATCH_SIZE))
            .height(Length::Fixed(SWATCH_SIZE))
            .style(move |_theme: &Theme| container::Style {
                background: Some(iced::Background::Color(color)),
                border: frame,
                ..container::Style::default()
            }),
        caption_text(state, label, to_color(resolved.sidebar.font.color)),
    ]
    .spacing(gap.widget)
    .align_y(iced::Center)
    .into()
}

/// A colour a Widget Info line ends in, as `color_to_hex` writes it:
/// `#rrggbb` or `#rrggbbaa`.
fn hex_color(line: &str) -> Option<Color> {
    let hex = line.rsplit(' ').next()?.strip_prefix('#')?;
    let channel = |at: usize| {
        hex.get(at..at + 2)
            .and_then(|pair| u8::from_str_radix(pair, 16).ok())
    };
    let (r, g, b) = (channel(0)?, channel(2)?, channel(4)?);
    let a = match hex.len() {
        6 => u8::MAX,
        8 => channel(6)?,
        _ => return None,
    };
    Some(Color::from_rgba8(
        r,
        g,
        b,
        f32::from(a) / f32::from(u8::MAX),
    ))
}

// ---------------------------------------------------------------------------
// Widget Info of a listed element (docs/showcase-elements.toml, section B)
// ---------------------------------------------------------------------------

/// What a leaf whose value the theme leaves unset reads as.
const NOT_STATED: &str = "not stated";

/// One "Theme" row of Widget Info: the leaf as the list spells it, its value
/// as the resolved theme holds it, its colour where it is one, and how iced
/// applies it.
struct InfoRow {
    leaf: &'static str,
    value: String,
    colour: Option<Color>,
    how: String,
}

/// Widget Info of one element of the list: its title, a row per leaf in the
/// list's order, and what iced draws for it that the theme states nothing
/// for.
struct ElementInfo {
    title: String,
    rows: Vec<InfoRow>,
    not_themeable: Vec<&'static str>,
}

impl ElementInfo {
    fn of(state: &State, element: &'static ShowcaseElement) -> Self {
        let values = toml::Value::try_from(&state.current_resolved)
            .unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));
        let rows = element
            .leaves
            .iter()
            .map(|leaf| {
                let (value, colour) = leaf_value(state, &values, leaf);
                InfoRow {
                    leaf: leaf.as_str(),
                    value,
                    colour,
                    how: iced_route(element, leaf),
                }
            })
            .collect();
        Self {
            title: element_title(element),
            rows,
            not_themeable: iced_not_themeable(element),
        }
    }

    /// The panel's text, line for line, as the Copy button copies it.
    fn copied(&self) -> String {
        let mut text = format!("{}\n\nTheme\n", self.title);
        for row in &self.rows {
            text.push_str(&format!("{} {}\n  {}\n", row.leaf, row.value, row.how));
        }
        if !self.not_themeable.is_empty() {
            text.push_str("\nNot themeable\n");
            for line in &self.not_themeable {
                text.push_str(line);
                text.push('\n');
            }
        }
        text
    }
}

/// The title Widget Info and the status bar show for an element: its name
/// and the state or variant the page shows, " · " between them.
fn element_title(element: &ShowcaseElement) -> String {
    match element.states.first() {
        Some(state) => format!("{} · {state}", element.name),
        None => element.name.clone(),
    }
}

/// A leaf's value as the theme holds it, and its colour where it is one. The
/// leaf is the registry's path (`button.border.padding_top_px`); the resolved
/// theme spells the same field without the unit and with the padding's
/// sides nested (`button.border.padding.top`). The layout distances, the
/// icon set and the icon theme are not in `ResolvedTheme`: they are read
/// where the showcase keeps them.
fn leaf_value(state: &State, values: &toml::Value, leaf: &str) -> (String, Option<Color>) {
    let px = leaf.ends_with("_px");
    if let Some(field) = leaf.strip_prefix("layout.") {
        let l = &state.layout;
        let stated = match field {
            "widget_gap_px" => l.widget_gap,
            "container_margin_px" => l.container_margin,
            "window_margin_px" => l.window_margin,
            "section_gap_px" => l.section_gap,
            _ => None,
        };
        return (
            stated.map_or(NOT_STATED.to_string(), |v| format!("{v} px")),
            None,
        );
    }
    match leaf {
        "theme_variant.icon_set" => return (state.current_icon_set.to_string(), None),
        "defaults.icon_theme" => {
            return (
                state
                    .current_icon_theme
                    .clone()
                    .unwrap_or_else(|| NOT_STATED.to_string()),
                None,
            );
        }
        _ => {}
    }
    let found = leaf.split('.').try_fold(values, |node, segment| {
        let segment = segment.strip_suffix("_px").unwrap_or(segment);
        match segment.strip_prefix("padding_") {
            Some(side) => node.get("padding").and_then(|p| p.get(side)),
            None => node.get(segment),
        }
    });
    match found {
        None => (NOT_STATED.to_string(), None),
        Some(value) => value_text(value, px),
    }
}

/// A value of the resolved theme as Widget Info writes it: a colour as its
/// `#rrggbb[aa]`, a size in px, a font as its family, the size its source
/// stated and its weight, a text-scale entry as its size and weight.
fn value_text(value: &toml::Value, px: bool) -> (String, Option<Color>) {
    let number = |v: f32| {
        if px {
            format!("{v} px")
        } else {
            format!("{v}")
        }
    };
    match value {
        toml::Value::String(s) => (s.clone(), hex_color(s)),
        toml::Value::Float(f) => (number(*f as f32), None),
        toml::Value::Integer(i) => (if px { format!("{i} px") } else { i.to_string() }, None),
        toml::Value::Boolean(b) => (b.to_string(), None),
        toml::Value::Table(t) => (compound_text(t), None),
        other => (other.to_string(), None),
    }
}

/// A font (`family`, `defined_size`, `weight`, `style`) or a text-scale entry
/// (`size`, `weight`) as one line: `Noto Sans 10 pt 400`, `16 px 400`.
fn compound_text(t: &toml::Table) -> String {
    let number_at = |table: &toml::Table, key: &str| {
        table
            .get(key)
            .and_then(toml::Value::as_float)
            .map(|v| v as f32)
    };
    let weight = t.get("weight").and_then(toml::Value::as_integer);
    let size = match t.get("defined_size").and_then(toml::Value::as_table) {
        Some(defined) => number_at(defined, "Pt")
            .map(|v| format!("{v} pt"))
            .or_else(|| number_at(defined, "Px").map(|v| format!("{v} px"))),
        None => number_at(t, "size").map(|v| format!("{v} px")),
    };
    let mut line = Vec::new();
    if let Some(family) = t.get("family").and_then(toml::Value::as_str) {
        line.push(family.to_string());
    }
    line.push(size.unwrap_or_else(|| NOT_STATED.to_string()));
    if let Some(weight) = weight {
        line.push(weight.to_string());
    }
    if let Some(style) = t.get("style").and_then(toml::Value::as_str)
        && style != "normal"
    {
        line.push(style.to_string());
    }
    line.join(" ")
}

/// Text in the side panel's font, `sidebar.font` (size, family), at
/// `weight`, on the theme's line height, on one line: Widget Info's text,
/// the same in the three showcases (docs/showcase-elements.toml, section B).
fn info_text<'a>(
    state: &State,
    content: impl text::IntoFragment<'a>,
    weight: u16,
    color: Color,
) -> text::Text<'a> {
    let resolved = &state.current_resolved;
    let font = ResolvedFontSpec {
        weight,
        ..resolved.sidebar.font.clone()
    };
    text(content)
        .themed(&font, resolved, &state.accessibility)
        .color(color)
        .wrapping(text::Wrapping::None)
}

/// A layout gap of Widget Info: the theme's, or none where it states none,
/// as the gpui and egui showcases' Widget Info lay it out.
fn info_gap(stated: Option<f32>) -> f32 {
    stated.unwrap_or_default()
}

/// Widget Info of the element `element`: the title (`name · state`) flush
/// left and Copy flush right; `layout.section_gap` below them the "Theme"
/// section, a row per leaf `layout.widget_gap` apart -- a swatch of a colour
/// one line tall, then `<leaf> <value>` over the muted line that says how
/// iced applies it -- and, where iced draws something the theme states
/// nothing for, the "Not themeable" section. Every line in `sidebar.font`, the
/// title and the section names at [`HEADING_WEIGHT`] ([`info_gap`] for the
/// gaps). The layout the gpui and egui showcases' Widget Info has.
fn element_info_view<'a>(
    state: &'a State,
    element: &'static ShowcaseElement,
) -> Element<'a, Message> {
    let resolved = &state.current_resolved;
    let widget_gap = info_gap(state.layout.widget_gap);
    let section_gap = info_gap(state.layout.section_gap);
    let ink = to_color(resolved.sidebar.font.color);
    let muted = to_color(resolved.defaults.muted_color);
    let regular = resolved.sidebar.font.weight;
    let line = scaled_text_size(resolved.sidebar.font.size, &state.accessibility)
        * resolved.defaults.line_height;
    let info = ElementInfo::of(state, element);
    let b = &resolved.button;
    let copy = button(text("Copy").themed(&resolved.sidebar.font, resolved, &state.accessibility))
        .padding(native_theme_iced::padding_or(
            &b.border.padding,
            Padding::ZERO,
        ))
        .style(copy_button(ghost_button(resolved, false), resolved))
        .on_press(Message::Copy(info.copied()));
    let mut body = column![
        row![
            tagged(
                "chrome.info.title",
                info_text(state, info.title.clone(), HEADING_WEIGHT, ink)
            ),
            space().width(Fill),
            tagged("chrome.info.copy", copy),
        ]
        .align_y(iced::Center),
        space().height(section_gap),
        tagged(
            "chrome.info.section.theme",
            info_text(state, "Theme", HEADING_WEIGHT, ink)
        ),
    ];
    let d = &resolved.defaults.border;
    let frame = iced::Border {
        color: to_color(d.color),
        width: d.line_width,
        radius: d.corner_radius.into(),
    };
    for (index, row) in info.rows.into_iter().enumerate() {
        let first = index == 0;
        let leaf = info_text(state, format!("{} {}", row.leaf, row.value), regular, ink);
        let how = info_text(state, row.how, regular, muted);
        let (leaf, how): (Element<'a, Message>, Element<'a, Message>) = if first {
            (
                // At the lines' own width, past the panel's edge, which
                // clips them: each recorded as the part of it inside the
                // row, what the panel shows of it.
                tagged("chrome.info.row_1.text", leaf)
                    .loose()
                    .within("chrome.info.row_1")
                    .into(),
                tagged("chrome.info.row_1.how", how)
                    .loose()
                    .within("chrome.info.row_1")
                    .into(),
            )
        } else {
            (leaf.into(), how.into())
        };
        let lines = column![leaf, how].width(Fill);
        let content: Element<'a, Message> = match row.colour {
            Some(colour) => {
                let swatch = container(space())
                    .width(Length::Fixed(line))
                    .height(Length::Fixed(line))
                    .style(move |_theme: &Theme| container::Style {
                        background: Some(iced::Background::Color(colour)),
                        border: frame,
                        ..container::Style::default()
                    });
                let swatch: Element<'a, Message> = if first {
                    tagged("chrome.info.row_1.swatch", swatch).into()
                } else {
                    swatch.into()
                };
                row![swatch, lines].spacing(widget_gap).width(Fill).into()
            }
            None => lines.into(),
        };
        body = body.push(space().height(widget_gap));
        body = body.push(if first {
            tagged("chrome.info.row_1", content).into()
        } else {
            content
        });
    }
    if !info.not_themeable.is_empty() {
        body = body.push(space().height(section_gap)).push(info_text(
            state,
            "Not themeable",
            HEADING_WEIGHT,
            ink,
        ));
        for line in info.not_themeable {
            body = body
                .push(space().height(widget_gap))
                .push(info_text(state, line, regular, muted));
        }
    }
    body.into()
}

/// Widget Info's Copy button: flat, `ghost` (a [`ghost_button`]), its label
/// the side panel's text colour at rest, `sidebar.font.color`, where the
/// other flat buttons' is `button.font.color`.
fn copy_button(
    ghost: impl Fn(&Theme, button::Status) -> button::Style,
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    let label = to_color(resolved.sidebar.font.color);
    move |theme, status| {
        let style = ghost(theme, status);
        match status {
            button::Status::Active => button::Style {
                text_color: label,
                ..style
            },
            _ => style,
        }
    }
}

/// How iced applies `leaf` to `element`: the connector's style function or
/// helper and the field or setter it reaches, or why it cannot. One muted
/// line of Widget Info.
fn iced_route(element: &ShowcaseElement, leaf: &str) -> String {
    let id = element.id.as_str();
    let under = |prefix: &str| id.starts_with(prefix);
    // The flat buttons, drawn in `ghost_button`: the toolbar's, the Basic
    // page's icon buttons, the page tabs' menu, the side-panel toggle and
    // Widget Info's Copy.
    let ghost = under("chrome.toolbar.")
        || under("basic.icons.copy")
        || under("basic.icons.paste")
        || under("basic.icons.delete")
        || under("chrome.page_tabs.menu")
        || under("chrome.status_bar.toggle")
        || id == "chrome.info.copy";
    let toggle_on = under("basic.buttons.toggle_on");
    let class = if ghost {
        "ghost_button"
    } else if toggle_on {
        "toggle_on (styles::button's Pressed look)"
    } else if under("basic.buttons.primary") {
        "styles::button_primary"
    } else {
        "styles::button"
    };
    let (table, field) = leaf.split_once('.').unwrap_or((leaf, ""));
    let side = |field: &str| {
        ["top", "right", "bottom", "left"]
            .into_iter()
            .find(|s| field == format!("border.padding_{s}_px"))
    };
    let line: Cow<'static, str> = match (table, field) {
        // ---- button ----
        ("button", "background_color") => {
            format!("{class}: button::Style::background at rest").into()
        }
        ("button", "font") => "the label's Text::size and theme_font (family, weight)".into(),
        ("button", "font.color") if ghost => {
            "the icon's colour where the set is a bundled one (chrome_icon); an OS icon theme's keep theirs"
                .into()
        }
        ("button", "font.color") => format!("{class}: button::Style::text_color at rest").into(),
        ("button", "min_width_px" | "min_height_px") => {
            "at_least(label, button_content_min_size): the minimum less button_padding".into()
        }
        ("button", "hover_background") if ghost => "ghost_button: the Hovered fill".into(),
        ("button", "hover_background") => {
            format!("{class}: the Hovered fill, composited over the rest fill").into()
        }
        ("button", "hover_text_color") => format!("{class}: the Hovered text_color").into(),
        ("button", "checked_background") => {
            "toggle_on: the fill at rest, the Pressed fill where unstated".into()
        }
        ("button", "checked_text_color") => {
            "toggle_on: text_color at rest, the Pressed one where unstated".into()
        }
        ("button", "active_background") if toggle_on => {
            "toggle_on: the fill at rest where checked_background is unstated, composited over button.background_color".into()
        }
        ("button", "active_background") if ghost => {
            "ghost_button: the Pressed fill, and the fill while selected".into()
        }
        ("button", "active_background") => {
            format!("{class}: the Pressed fill, composited over the rest fill").into()
        }
        ("button", "active_text_color") if toggle_on => {
            "toggle_on: text_color at rest where checked_text_color is unstated".into()
        }
        ("button", "active_text_color") => format!("{class}: the Pressed text_color").into(),
        ("button", "primary_background") => {
            "styles::button_primary: button::Style::background at rest".into()
        }
        ("button", "primary_text_color") => {
            "styles::button_primary: button::Style::text_color at rest".into()
        }
        ("button", "disabled_background") => {
            "styles::button: the Disabled fill (a button with no on_press)".into()
        }
        ("button", "disabled_text_color") => "styles::button: the Disabled text_color".into(),
        ("button", "disabled_opacity") => {
            "styles::button: the Disabled colours' alpha times it (iced has no widget opacity)"
                .into()
        }
        ("button", "border.color") => format!("{class}: button::Style::border.color").into(),
        ("button", "border.corner_radius_px") => {
            format!("{class}: button::Style::border.radius").into()
        }
        ("button", "border.line_width_px") => {
            format!("{class}: button::Style::border.width, painted over the padding").into()
        }
        ("button", f) if side(f).is_some() && id == "chrome.info.copy" => {
            "Button::padding, the side as stated (none where unstated)".into()
        }
        ("button", f) if side(f).is_some() && ghost => {
            "ghost_padding: Button::padding, the side as stated (a flat button draws no border)"
                .into()
        }
        ("button", f) if side(f).is_some() => {
            "button_padding: Button::padding, the side plus button.border.line_width".into()
        }
        // ---- input, text area ----
        ("input", "background_color") => "styles::text_input: text_input::Style::background".into(),
        ("input", "font") => "Text input size and theme_font (family, weight)".into(),
        ("input", "font.color") => "styles::text_input: text_input::Style::value".into(),
        ("input", "placeholder_color") => "styles::text_input: text_input::Style::placeholder".into(),
        ("input", "caret_color") => {
            "not reachable: text_input::Style has no caret colour; iced draws it in the value colour"
                .into()
        }
        ("input", "min_height_px") => {
            "control_line_height: the line box that fills it inside the padding".into()
        }
        ("input", "hover_border_color") => "styles::text_input: the Hovered border.color".into(),
        ("input", "focus_border_color") => "styles::text_input: the Focused border.color".into(),
        ("input", "disabled_background") => {
            "styles::text_input: the Disabled background (a field with no on_input)".into()
        }
        ("input", "disabled_text_color") => "styles::text_input: the Disabled value colour".into(),
        ("input", "disabled_opacity") => {
            "styles::text_input: the Disabled colours' alpha times it".into()
        }
        ("input", "border.color") => "styles::text_input: text_input::Style::border.color".into(),
        ("input", "border.corner_radius_px") => {
            "styles::text_input: text_input::Style::border.radius".into()
        }
        ("input", "border.line_width_px") => {
            "styles::text_input: text_input::Style::border.width".into()
        }
        ("input", f) if side(f).is_some() => {
            "input_padding: TextInput::padding, the side plus the line width".into()
        }
        ("text_area", "border.color") => "styles::text_editor: text_editor::Style::border.color".into(),
        ("text_area", "border.corner_radius_px") => {
            "styles::text_editor: text_editor::Style::border.radius".into()
        }
        ("text_area", "border.line_width_px") => {
            "styles::text_editor: text_editor::Style::border.width".into()
        }
        ("text_area", f) if side(f).is_some() => {
            "text_area_padding: TextEditor::padding, the side plus the line width".into()
        }
        (
            "button" | "input" | "text_area" | "combo_box" | "tooltip" | "card",
            "border.shadow_enabled",
        ) => {
            "not reachable: the model states no shadow geometry; iced draws no shadow here".into()
        }
        // ---- checkbox and radio ----
        ("checkbox", "background_color") if under("basic.radios.") => {
            "styles::radio: radio::Style::background".into()
        }
        ("checkbox", "background_color") => "styles::checkbox: checkbox::Style::background".into(),
        ("checkbox", "font") => "Checkbox/Radio text_size and font (theme_font)".into(),
        ("checkbox", "font.color") => "styles::checkbox / styles::radio: the label's text_color".into(),
        ("checkbox", "indicator_color") if under("basic.radios.") => {
            "native_theme_iced::radio: the dot's fill".into()
        }
        ("checkbox", "indicator_color") => "the check mark's stroke colour (CheckMark canvas)".into(),
        ("checkbox", "indicator_width_px") => "Checkbox/Radio::size".into(),
        ("checkbox", "check_mark_stroke_width_px") => {
            "CheckMark: the mark's stroke width, over iced's glyph made transparent".into()
        }
        ("checkbox", "radio_dot_diameter_px") => {
            "native_theme_iced::radio: the dot's diameter, over iced's own dot".into()
        }
        ("checkbox", "radio_indicator_width_px") => {
            "native_theme_iced::radio: Radio::size, indicator_width where unstated".into()
        }
        ("checkbox", "label_gap_px") => "Checkbox/Radio::spacing".into(),
        ("checkbox", "hover_background") => {
            "styles::checkbox / styles::radio: the Hovered unchecked fill".into()
        }
        ("checkbox", "unchecked_background") => {
            "styles::checkbox / styles::radio: the unchecked fill".into()
        }
        ("checkbox", "unchecked_border_color") => {
            "styles::checkbox / styles::radio: the unchecked border colour".into()
        }
        ("checkbox", "checked_background") => {
            "styles::checkbox / styles::radio: the checked fill".into()
        }
        ("checkbox", "disabled_background") => "styles::checkbox: the Disabled fill".into(),
        ("checkbox", "disabled_text_color") => "styles::checkbox: the Disabled label and mark".into(),
        ("checkbox", "disabled_opacity") => {
            "styles::checkbox: the Disabled colours' alpha times it; a drawn mark fades with its box's fill".into()
        }
        ("checkbox", "border.color") => {
            "styles::checkbox / styles::radio: the checked border colour".into()
        }
        ("checkbox", "border.corner_radius_px") => "styles::checkbox: checkbox::Style::border.radius".into(),
        ("checkbox", "border.line_width_px") => {
            "styles::checkbox / styles::radio: the border width".into()
        }
        ("checkbox", f) if side(f).is_some() => {
            "CheckMark: the mark's box, inside the border and this side".into()
        }
        // ---- switch ----
        ("switch", "track_width_px" | "track_height_px") => {
            "native_theme_iced::switch: the track button's width and height".into()
        }
        ("switch", "thumb_diameter_px") => {
            "native_theme_iced::switch: the thumb container, inset by half the height difference"
                .into()
        }
        ("switch", "track_radius_px") => "styles::toggler: border_radius of track and thumb".into(),
        ("switch", "checked_background" | "unchecked_background") => {
            "styles::toggler: the track's background".into()
        }
        ("switch", "hover_checked_background" | "hover_unchecked_background") => {
            "styles::toggler: the Hovered track background".into()
        }
        ("switch", "thumb_background") => "styles::toggler: the thumb's foreground".into(),
        ("switch", "unchecked_thumb_diameter_px") => {
            "native_theme_iced::switch: the off thumb container, thumb_diameter where unstated"
                .into()
        }
        ("switch", "unchecked_thumb_background") => {
            "styles::toggler: the off thumb's foreground, thumb_background where unstated".into()
        }
        ("switch", "disabled_checked_background") => {
            "styles::toggler: the Disabled track background".into()
        }
        ("switch", "disabled_thumb_color") => "styles::toggler: the Disabled foreground".into(),
        ("switch", "disabled_opacity") => {
            "styles::toggler: the Disabled colours' alpha times it; the label faded alike".into()
        }
        // ---- combo box (drop-down) ----
        ("combo_box", "background_color") => "styles::pick_list: pick_list::Style::background".into(),
        ("combo_box", "font") => "PickList text_size and font (theme_font)".into(),
        ("combo_box", "font.color") if under("basic.drop_down.trigger.arrow") => {
            "styles::pick_list: pick_list::Style::handle_color".into()
        }
        ("combo_box", "font.color") => "styles::pick_list: pick_list::Style::text_color".into(),
        ("combo_box", "min_height_px") => {
            "control_line_height: the line box that fills it inside combo_box_padding".into()
        }
        ("combo_box", "min_width_px") => {
            "not reachable: PickList takes a width, not a minimum; the page's own width".into()
        }
        ("combo_box", "arrow_icon_size_px") => {
            "pick_list_handle: the chevron glyph's size".into()
        }
        ("combo_box", "arrow_area_width_px") => {
            "combo_box_padding: the right padding holds half the column the arrow leaves".into()
        }
        ("combo_box", "hover_background") => "styles::pick_list: the Hovered background".into(),
        ("combo_box", "border.color") => "styles::pick_list: pick_list::Style::border.color".into(),
        ("combo_box", "border.corner_radius_px") => {
            "styles::pick_list: pick_list::Style::border.radius".into()
        }
        ("combo_box", "border.line_width_px") => {
            "styles::pick_list: pick_list::Style::border.width".into()
        }
        ("combo_box", f) if side(f).is_some() => {
            "combo_box_padding: PickList::padding, the side plus the line width".into()
        }
        // ---- menu and popover ----
        ("menu", "background_color") => "menu_bar_style: menu_background".into(),
        ("menu", "font") => "the row's text size and theme_font".into(),
        ("menu", "font.color") => "menu_row: text_color at rest".into(),
        ("menu", "hover_background") => "menu_row: the Hovered fill; menu_bar_style: path".into(),
        ("menu", "hover_text_color") => "menu_row: the Hovered text_color".into(),
        ("menu", "row_height_px") => "the row button's height, where stated".into(),
        ("menu", "separator_color") => "line_style: the rule's colour".into(),
        ("menu", f) if side(f).is_some() => "Button::padding, iced's own on an unstated side".into(),
        ("popover", "border.color") => "menu_bar_style: menu_border.color".into(),
        ("popover", "border.corner_radius_px") => "menu_bar_style: menu_border.radius".into(),
        ("popover", "border.line_width_px") => "menu_bar_style: menu_border.width".into(),
        ("popover", "border.shadow_enabled") => {
            "not reachable: the model states no shadow geometry; iced_aw's menu_shadow".into()
        }
        ("popover", f) if side(f).is_some() => {
            "not reachable: iced_aw's Menu has no padding around its rows".into()
        }
        // ---- toolbar ----
        ("toolbar", "bar_height_px") => "the bar container's height, where stated".into(),
        ("toolbar", "item_gap_px") => {
            "the buttons' Row::spacing; layout.widget_gap where unstated".into()
        }
        ("toolbar", "background_color") => "surface: the bar container's background".into(),
        ("toolbar", "border.color") => "line_style: the bar's bottom line's colour".into(),
        ("toolbar", "border.line_width_px") => {
            "the bottom line rule's height, under the bar's padding".into()
        }
        ("toolbar", f) if side(f).is_some() => {
            "the bar container's padding; layout.container_margin where unstated".into()
        }
        ("toolbar", "font") => "the button's text, where the icon set has no icon".into(),
        ("toolbar", "icon_size_px") => "the svg or image's width and height".into(),
        // ---- side panel, status bar, splitter, separator ----
        ("sidebar", "background_color") => "surface: the panel container's background".into(),
        ("sidebar", f) if side(f).is_some() => {
            "not drawn: the panel's sections are padded by layout.container_margin".into()
        }
        ("sidebar", "font") => "the text's size and theme_font".into(),
        ("sidebar", "font.color") => "the text's colour".into(),
        ("status_bar", "background_color") => "surface: the bar container's background".into(),
        ("status_bar", "border.color") => "line_style: the top edge's colour".into(),
        ("status_bar", "border.line_width_px") => {
            "the top edge rule's height, above the bar's padding".into()
        }
        ("status_bar", f) if side(f).is_some() => {
            "the bar container's padding; layout.container_margin where unstated".into()
        }
        ("status_bar", "font") => "the text's size and theme_font".into(),
        ("status_bar", "font.color") => "surface: the bar's text_color".into(),
        ("splitter", "divider_width_px") => "the line container's width".into(),
        ("splitter", "divider_color") => "the line container's background".into(),
        ("splitter", "hover_color") => "the line's background under the pointer and while dragged".into(),
        ("separator", "line_color") if under("chrome.menu.") => "not drawn here".into(),
        ("separator", "line_color") => "styles::rule: rule::Style::color".into(),
        ("separator", "line_width_px") => "rule::horizontal's thickness".into(),
        // ---- tabs ----
        ("tab", "bar_background") if under("basic.tabs.") => {
            "styles::aw::tab_bar: tab_bar::Style::background".into()
        }
        ("tab", "bar_background") => "surface: the strip container's background".into(),
        ("tab", "item_gap_px") => "the tabs' spacing, where stated".into(),
        ("tab", "active_indicator_color" | "active_indicator_width_px" | "active_indicator_side")
            if under("basic.tabs.") =>
        {
            "native_theme_iced::tab_indicator_line over the open tab's place in iced_aw's bar"
                .into()
        }
        ("tab", "active_indicator_color" | "active_indicator_width_px" | "active_indicator_side") => {
            "native_theme_iced::tab_indicator: the line stacked over the open tab".into()
        }
        ("tab", "active_background") => "the open tab's background".into(),
        ("tab", "active_text_color") => "the open tab's text colour".into(),
        ("tab", "background_color") => "a tab's background at rest".into(),
        ("tab", "hover_background") => "a tab's background under the pointer".into(),
        ("tab", "hover_text_color") => "a tab's text colour under the pointer".into(),
        ("tab", "font") => "the label's size and theme_font".into(),
        ("tab", "font.color") => "a tab's text colour at rest".into(),
        ("tab", "min_width_px" | "min_height_px") if under("basic.tabs.") => {
            "TabBar::tab_width and height, fixed: iced_aw takes no minimum".into()
        }
        ("tab", "min_width_px" | "min_height_px") => {
            "at_least(label, ..): the minimum less the padding".into()
        }
        ("tab", "border.color") => "the open tab's border colour".into(),
        ("tab", "border.corner_radius_px") => "the open tab's top corners".into(),
        ("tab", "border.line_width_px") => "the open tab's border width".into(),
        ("tab", f) if side(f).is_some() && under("basic.tabs.") => {
            "TabBar::padding, where every side is stated".into()
        }
        ("tab", f) if side(f).is_some() => {
            "padding_inside_border: the button's padding, the side plus the line width".into()
        }
        // ---- slider, progress bar, spinner ----
        ("slider", "fill_color") => "styles::slider: the rail's filled colour".into(),
        ("slider", "track_color") => "styles::slider: the rail's remaining colour".into(),
        ("slider", "thumb_color") => "styles::slider: the handle's background".into(),
        ("slider", "thumb_hover_color") => "styles::slider: the Hovered handle".into(),
        ("slider", "track_height_px") => "styles::slider: the rail's width".into(),
        ("slider", "thumb_diameter_px") => {
            "styles::slider: a circle handle of half of it; Slider::height".into()
        }
        ("progress_bar", "fill_color") => "styles::progress_bar: progress_bar::Style::bar".into(),
        ("progress_bar", "track_color") => {
            "styles::progress_bar: progress_bar::Style::background".into()
        }
        ("progress_bar", "track_height_px") => "ProgressBar::girth".into(),
        ("progress_bar", "min_width_px") => {
            "not reachable: ProgressBar takes a length, not a minimum; the page's own width".into()
        }
        ("progress_bar", "border.color") => "styles::progress_bar: border.color".into(),
        ("progress_bar", "border.corner_radius_px") => "styles::progress_bar: border.radius".into(),
        ("progress_bar", "border.line_width_px") => "styles::progress_bar: border.width".into(),
        ("spinner", "diameter_px") => "native_theme_iced::Spinner: its width and height".into(),
        ("spinner", "min_diameter_px") => {
            "not reachable: the spinner is drawn at spinner.diameter".into()
        }
        ("spinner", "stroke_width_px") => {
            "native_theme_iced::Spinner: the arc's stroke, where the icon set has no indicator"
                .into()
        }
        ("spinner", "fill_color") => {
            "native_theme_iced::Spinner: the arc, or a bundled set's indicator".into()
        }
        // ---- segmented control ----
        ("segmented_control", "background_color") => "styles::segment: a segment's fill".into(),
        ("segmented_control", "font") => "the label's size and theme_font".into(),
        ("segmented_control", "font.color") => "styles::segment: text_color at rest".into(),
        ("segmented_control", "active_background") => {
            "styles::segment: the selected segment's fill".into()
        }
        ("segmented_control", "active_text_color") => {
            "styles::segment: the selected segment's text_color".into()
        }
        ("segmented_control", "hover_background") => "styles::segment: the Hovered fill".into(),
        ("segmented_control", "segment_height_px") => {
            "at_least(label, ..): each segment's outer height less its padding".into()
        }
        ("segmented_control", "separator_width_px") => {
            "the segments' Row::spacing, over the outline colour".into()
        }
        ("segmented_control", "border.color") => {
            "styles::segmented_control: the outline, showing between segments".into()
        }
        ("segmented_control", "border.corner_radius_px") => {
            "styles::segmented_control and styles::segment: the outer corners".into()
        }
        ("segmented_control", "border.line_width_px") => {
            "styles::segmented_control: the outline, the container's padding".into()
        }
        ("segmented_control", f) if side(f).is_some() => {
            "Button::padding of each segment, iced's own on an unstated side".into()
        }
        // ---- list and table ----
        ("list", "background_color") if under("basic.table.") => {
            "the table container's background".into()
        }
        ("list", "background_color") => "the list container's background".into(),
        ("list", "item_font") => "the row label's size and theme_font".into(),
        ("list", "item_font.color") if under("basic.table.") => "a cell's text colour".into(),
        ("list", "item_font.color") => "list_row: text_color at rest".into(),
        ("list", "alternate_row_background") => "the alternate row container's background".into(),
        ("list", "selection_background") if under("basic.table.") => {
            "the selected row container's background".into()
        }
        ("list", "selection_background") => "list_row: the selected row's fill".into(),
        ("list", "selection_text_color") if under("basic.table.") => {
            "the selected row's text colour".into()
        }
        ("list", "selection_text_color") => "list_row: the selected row's text_color".into(),
        ("list", "header_font") => "the header cells' size and theme_font".into(),
        ("list", "header_font.color") => "the header cells' text colour".into(),
        ("list", "header_background") => "the header row container's background".into(),
        ("list", "grid_color") => "the grid lines' colour (rules between the cells)".into(),
        ("list", "row_height_px") => {
            "a row's height; the label's line box and padding where unstated".into()
        }
        ("list", "hover_background") if under("basic.table.") => {
            "not reachable: the table's rows are containers, not hoverable".into()
        }
        ("list", "hover_text_color") if under("basic.table.") => {
            "not reachable: the table's rows are containers, not hoverable".into()
        }
        ("list", "hover_background") => "list_row: the Hovered fill".into(),
        ("list", "hover_text_color") => "list_row: the Hovered text_color".into(),
        ("list", "border.color") => "the frame container's border.color".into(),
        ("list", "border.corner_radius_px") => "the frame container's border.radius".into(),
        ("list", "border.line_width_px") => {
            "the frame container's border.width and padding".into()
        }
        ("list", f) if side(f).is_some() && under("basic.table.") => {
            "a cell's padding; none where unstated".into()
        }
        ("list", f) if side(f).is_some() => {
            "a row's padding; iced_aw's list's 5 px where unstated".into()
        }
        // ---- scrollbar ----
        ("scrollbar", "track_color") => "styles::scrollable: the rail's background".into(),
        ("scrollbar", "thumb_color") => "styles::scrollable: the scroller's background".into(),
        ("scrollbar", "thumb_hover_color") => "styles::scrollable: the Hovered scroller".into(),
        ("scrollbar", "thumb_active_color") => "styles::scrollable: the Dragged scroller".into(),
        ("scrollbar", "groove_width_px") => "styles::scrollbar: Scrollbar::width".into(),
        ("scrollbar", "thumb_width_px") => "styles::scrollbar: Scrollbar::scroller_width".into(),
        ("scrollbar", "min_thumb_length_px") => {
            "not reachable: iced's scroller has no minimum length setter".into()
        }
        ("scrollbar", "overlay_mode") => "styles::scrollbar: no spacing beside the content".into(),
        // ---- expander ----
        ("expander", "font") => "the title's size and theme_font".into(),
        ("expander", "font.color") => "styles::expander: text_color".into(),
        ("expander", "header_height_px") => {
            "at_least(title row, ..): the height less the padding".into()
        }
        ("expander", "arrow_icon_size_px") => "the DisclosureArrow canvas's size".into(),
        ("expander", "hover_background") => "styles::expander: the Hovered fill".into(),
        ("expander", "arrow_color") => {
            "expander_arrow_color: the DisclosureArrow's fill".into()
        }
        ("expander", "arrow_side") => "the arrow before or after the title".into(),
        ("expander", "arrow_gap_px") => {
            "the header row's spacing; layout.widget_gap where unstated".into()
        }
        ("expander", "content_indent_px") => {
            "the body's left padding; under the title where unstated".into()
        }
        ("expander", "frame_enabled") => {
            "one frame round header and body, or none; each header framed where unstated".into()
        }
        ("expander", "border.color") => "styles::expander / the frame: border.color".into(),
        ("expander", "border.corner_radius_px") => "styles::expander / the frame: border.radius".into(),
        ("expander", "border.line_width_px") => "styles::expander / the frame: border.width".into(),
        ("expander", f) if side(f).is_some() => {
            "padding_inside_border: the header's padding, the side plus the line width".into()
        }
        // ---- card ----
        ("card", "background_color") => "styles::container_card: container::Style::background".into(),
        ("card", "border.color") => "styles::container_card: border.color".into(),
        ("card", "border.corner_radius_px") => "styles::container_card: border.radius".into(),
        ("card", "border.line_width_px") => "styles::container_card: border.width".into(),
        ("card", f) if side(f).is_some() => {
            "the card container's padding; layout.container_margin where unstated".into()
        }
        // ---- tooltip ----
        ("tooltip", "background_color") => "styles::tooltip: container::Style::background".into(),
        ("tooltip", "font") => "the tip's text size and theme_font".into(),
        ("tooltip", "font.color") => "styles::tooltip: container::Style::text_color".into(),
        ("tooltip", "max_width_px") => "the tip container's max_width".into(),
        ("tooltip", "border.color") => "styles::tooltip: border.color".into(),
        ("tooltip", "border.corner_radius_px") => "styles::tooltip: border.radius".into(),
        ("tooltip", "border.line_width_px") => "styles::tooltip: border.width".into(),
        ("tooltip", f) if side(f).is_some() => {
            "Tooltip::padding where all four sides are equal; iced's own otherwise".into()
        }
        // ---- link ----
        ("link", "font") => "the span's size and theme_font".into(),
        ("link", "font.color") => "styles::button_link: text_color at rest".into(),
        ("link", "underline_enabled") => "the span's underline".into(),
        ("link", "background_color") => "styles::button_link: the background".into(),
        ("link", "hover_background") => "styles::button_link: the Hovered background".into(),
        ("link", "hover_text_color") => "styles::button_link: the Hovered text_color".into(),
        ("link", "active_text_color") => "styles::button_link: the Pressed text_color".into(),
        // ---- window, layout ----
        ("window", "background_color") if id == "chrome.window" => {
            "surface: the window container's background".into()
        }
        ("window", "background_color") => "the window's fill, which shows through".into(),
        ("layout", "widget_gap_px") => "Row/Column::spacing between the items".into(),
        ("layout", "container_margin_px") => "the container's padding".into(),
        ("layout", "window_margin_px") => "the page's padding".into(),
        ("layout", "section_gap_px") => "Column::spacing between the groups".into(),
        // ---- defaults, text scale, icons ----
        ("defaults", "line_height") => {
            "the text's LineHeight::Relative (line_height_multiplier)".into()
        }
        ("defaults", "font") => "the text's size and theme_font".into(),
        ("defaults", "font.color") => "the text's colour: the palette's text".into(),
        ("defaults", "font.family") => "theme_font: the role's family".into(),
        ("defaults", "mono_font") => "theme_mono_font: the text's size and family".into(),
        ("defaults", "mono_font.color") => "the text's colour".into(),
        ("defaults", "muted_color") => "the text's colour".into(),
        ("defaults", "disabled_text_color") => {
            "the label's colour, times switch.disabled_opacity".into()
        }
        ("defaults", "focus_ring_color" | "focus_ring_width_px" | "focus_ring_offset_px") => {
            "not reachable: iced draws no focus ring; the Focused border is the input's".into()
        }
        ("defaults", "border.color") => "the swatch container's border.color".into(),
        ("defaults", "border.corner_radius_px") => "the swatch container's border.radius".into(),
        ("defaults", "border.line_width_px") => "the swatch container's border.width".into(),
        ("defaults", "icon_sizes.small_px" | "icon_sizes.toolbar_px" | "icon_sizes.large_px") => {
            "the svg or image's width and height".into()
        }
        ("defaults", "icon_theme") => {
            "the icon loaders' theme: native_theme's FreedesktopLoader".into()
        }
        ("theme_variant", "icon_set") => {
            "the icon loader of the set: IconSetChoice::effective_icon_set".into()
        }
        ("text_scale", _) => "Typeset::role: the text's size, line height and weight".into(),
        _ => Cow::Owned(format!("{leaf}: no route recorded")),
    };
    if line.starts_with("not reachable") || line.starts_with("not drawn") {
        line.into_owned()
    } else {
        format!("iced: {line}")
    }
}

/// What iced draws for `element` that the theme states nothing for: the
/// "Not themeable" lines of its Widget Info.
fn iced_not_themeable(element: &ShowcaseElement) -> Vec<&'static str> {
    match element.name.as_str() {
        "Switch" => vec!["label gap: iced's toggler's, Toggler::DEFAULT_SIZE / 2"],
        "Spinner" => vec!["sweep and speed of the arc: egui's Spinner's, 240° a turn a second"],
        "Expander" => vec!["arrow shape: a filled triangle on a canvas"],
        "Menu" => vec![
            "menu width: its widest row, the label and the shortcut \
             layout.widget_gap apart inside the row's padding",
            "shadows: iced_aw's own",
        ],
        "Menu shortcut" => vec!["shortcut colour: the row's label colour"],
        "Tooltip" => vec!["gap to its control: the showcase's"],
        "Number input" => vec!["step buttons: iced_aw's, in the field's text colour"],
        _ => Vec::new(),
    }
}

/// The inspector's Widget tab: the hint before any hover; after a hover of
/// an element of the list, [`element_info_view`]; after one of another
/// page's widget, the title with a flat Copy button flush right, then the
/// text's sections as `widget_tooltip` writes them -- a heading per section,
/// a swatch line per colour, a name over its value per other line.
fn inspector_widget(state: &State) -> Element<'_, Message> {
    if let Some(element) = state.hovered_element.and_then(listed) {
        return element_info_view(state, element);
    }
    let resolved = &state.current_resolved;
    let gap = Gaps::from_layout(&state.layout);
    let muted = to_color(resolved.defaults.muted_color);
    let text_color = to_color(resolved.sidebar.font.color);
    let Some(title) = info_title(&state.widget_info) else {
        return tagged(
            "chrome.info.hint",
            info_text(
                state,
                "Hover any widget to see what the theme sets on it.",
                resolved.sidebar.font.weight,
                muted,
            ),
        )
        .into();
    };
    let pad = ghost_padding(resolved);
    let copy =
        button(text("Copy").role(&resolved.text_scale.caption, resolved, &state.accessibility))
            .padding(Padding::ZERO.left(pad.left).right(pad.right))
            .style(ghost_button(resolved, false))
            .on_press(Message::Copy(state.widget_info.clone()));
    let mut body = column![
        row![
            inspector_heading(state, title.to_string()),
            space().width(Fill),
            copy
        ]
        .align_y(iced::Center)
    ]
    .spacing(gap.widget);
    let mut colours = false;
    for line in state
        .widget_info
        .lines()
        .skip_while(|l| l.trim().is_empty())
        .skip(1)
    {
        if line.trim().is_empty() {
            continue;
        }
        if !line.starts_with(' ')
            && let Some(heading) = line.strip_suffix(':')
        {
            colours = heading == "Theme colors";
            body = body.push(inspector_heading(state, heading.to_string()));
            continue;
        }
        let line = line.trim();
        body = body.push(match (colours, hex_color(line)) {
            (true, Some(color)) => inspector_swatch(state, line.to_string(), color),
            _ => match line.split_once(": ") {
                Some((name, value)) => inspector_row(state, name.to_string(), value.to_string()),
                None => caption_text(state, line.to_string(), text_color).into(),
            },
        });
    }
    body.into()
}

/// The inspector's Theme tab: what the theme and the window set that no
/// widget carries, as the gpui showcase's -- the theme's metrics, its fonts
/// in the unit their source stated, and the window.
fn inspector_theme(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let gap = Gaps::from_layout(&state.layout);
    let px = |v: f32| format!("{v}px");
    let sides = |p: Padding| {
        format!(
            "top {}px, right {}px, bottom {}px, left {}px",
            p.top, p.right, p.bottom, p.left
        )
    };
    let layout = &state.layout;
    let config = [
        ("radius", px(native_theme_iced::border_radius(resolved))),
        (
            "radius_lg",
            px(native_theme_iced::border_radius_lg(resolved)),
        ),
        (
            "scrollbar",
            px(native_theme_iced::scrollbar_width(resolved)),
        ),
        (
            "scrollbar overlay",
            resolved.scrollbar.overlay_mode.to_string(),
        ),
        (
            "button padding",
            sides(native_theme_iced::button_padding(resolved)),
        ),
        (
            "input padding",
            sides(native_theme_iced::input_padding(resolved)),
        ),
        ("widget_gap", layout_value(layout.widget_gap, SP.s)),
        (
            "container_margin",
            layout_value(layout.container_margin, SP.l),
        ),
        ("window_margin", layout_value(layout.window_margin, SP.l)),
        ("section_gap", layout_value(layout.section_gap, SP.xl)),
    ];
    let d = &resolved.defaults;
    let fonts = [
        (
            "font_family",
            family_label(
                &d.font.family,
                resolved_family(&d.font),
                &font_drawn(&d.font),
            ),
        ),
        ("font_size", defined_size(&d.font)),
        (
            "mono_font_family",
            family_label(
                &d.mono_font.family,
                resolved_family(&d.mono_font),
                &mono_drawn(resolved),
            ),
        ),
        ("mono_font_size", defined_size(&d.mono_font)),
    ];
    let window = [
        (
            "decorations",
            "the window manager's, where it draws them: iced asks winit for a decorated window"
                .to_string(),
        ),
        (
            "frame",
            "whatever the window manager draws (KWin: Breeze's title bar, controls, corners and shadow)"
                .to_string(),
        ),
        ("title", WINDOW_TITLE.to_string()),
    ];
    let section = |title: &'static str, rows: Vec<(&'static str, String)>| {
        column![inspector_heading(state, title)]
            .extend(
                rows.into_iter()
                    .map(|(name, value)| inspector_row(state, name.to_string(), value)),
            )
            .spacing(gap.widget)
    };
    column![
        section("Theme config", config.to_vec()),
        section("Fonts", fonts.to_vec()),
        section("Window", window.to_vec()),
    ]
    .spacing(gap.widget)
    .into()
}

/// One entry of the command palette: its label, the words it is also found
/// by, and the message it runs.
struct PaletteEntry {
    label: String,
    keywords: Vec<String>,
    action: Message,
}

/// The command palette's entries (spec §2.8), as `(group, entries)`: every
/// page, every preset the preset switch offers, found by its key too, and
/// the three colour modes, each sending the message its tab, drop-down or
/// menu row sends; only those the query matches, ignoring case.
fn palette_groups(state: &State) -> Vec<(&'static str, Vec<PaletteEntry>)> {
    let query = state.palette_query.trim().to_lowercase();
    let matches = |entry: &PaletteEntry| {
        query.is_empty()
            || entry.label.to_lowercase().contains(&query)
            || entry
                .keywords
                .iter()
                .any(|word| word.to_lowercase().contains(&query))
    };
    let pages = Tab::ALL
        .iter()
        .map(|&tab| PaletteEntry {
            label: tab.label().to_string(),
            keywords: Vec::new(),
            action: Message::TabSelected(tab),
        })
        .collect::<Vec<_>>();
    let presets = theme_choices(&state.default_label)
        .into_iter()
        .map(|choice| PaletteEntry {
            label: choice.to_string(),
            keywords: vec![match &choice {
                ThemeChoice::OsTheme(_) => "default".to_string(),
                ThemeChoice::Preset(key) => key.clone(),
            }],
            action: Message::ThemeSelected(choice),
        })
        .collect::<Vec<_>>();
    let modes = AppColorMode::ALL
        .iter()
        .map(|&mode| PaletteEntry {
            label: mode.palette_label(),
            keywords: Vec::new(),
            action: Message::ColorModeSelected(mode),
        })
        .collect::<Vec<_>>();
    [
        ("Pages", pages),
        ("Presets", presets),
        ("Colour mode", modes),
    ]
    .into_iter()
    .map(|(group, entries)| (group, entries.into_iter().filter(matches).collect()))
    .collect()
}

/// A dialog over the window (spec §2.8), as the model's dialog
/// (platform-facts §2.22): its title in `dialog.title_font` with a flat close
/// button at its right, the chosen icon theme's `IconRole::WindowClose` at
/// `defaults.icon_sizes.small`; its body in `dialog.body_font`'s colour; filled
/// with `dialog.background_color`, framed by `dialog.border`, padded by
/// `dialog.border.padding` inside the border (`layout.container_margin` on a
/// side it leaves unstated), `dialog.max_width` wide and at least
/// `dialog.min_height` tall.
fn dialog(state: &State, overlay: Overlay) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let d = &resolved.dialog;
    let (title, body) = match overlay {
        Overlay::CommandPalette => ("Command Palette", command_palette(state)),
        Overlay::Preferences => ("Preferences", preferences(state)),
        Overlay::About => ("About", about(state)),
    };
    let pad = ghost_padding(resolved);
    let close = icon_button(
        state,
        IconButton {
            icon: &state.chrome_icons.close,
            size: resolved.defaults.icon_sizes.small,
            label: "Close",
            key: None,
            font: &resolved.button.font,
            padding: Padding::ZERO.left(pad.left).right(pad.right),
            selected: false,
            action: Message::CloseOverlay,
            tags: None,
        },
    );
    let head = row![
        text(title)
            .themed(&d.title_font, resolved, a11y)
            .color(to_color(d.title_font.color)),
        space().width(Fill),
        close,
    ]
    .align_y(iced::Center);
    let padding = native_theme_iced::padding_inside_border(&d.border, Padding::from(gap.container));
    let min_height = (d.min_height - padding.y()).max(0.0);
    let border = iced::Border {
        color: to_color(d.border.color),
        width: d.border.line_width,
        radius: d.border.corner_radius.into(),
    };
    let fill = to_color(d.background_color);
    let body_color = to_color(d.body_font.color);
    // A row is as tall as its tallest child, and keeps the content at its
    // top: the space beside the content is the dialog's minimum height.
    container(row![
        column![head, body].spacing(gap.widget).width(Fill),
        space().height(Length::Fixed(min_height)),
    ])
    .padding(padding)
    .width(Fill)
    .max_width(d.max_width)
    .max_height(d.max_height)
    .style(move |_theme: &Theme| container::Style {
        text_color: Some(body_color),
        background: Some(iced::Background::Color(fill)),
        border,
        ..container::Style::default()
    })
    .into()
}

/// The command palette: a query, in the model's text input, over the
/// entries it matches in their groups, each group headed in
/// `text_scale.caption` and `defaults.muted_color`, each entry a list row
/// ([`list_row`]) in `list.item_font`, padded by `list.border.padding`.
/// Enter runs the first entry; running one closes the palette.
fn command_palette(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let size = scaled_text_size(resolved.input.font.size, a11y);
    let inp_pad = native_theme_iced::input_padding(resolved);
    let query = text_input("A page, a preset or a colour mode…", &state.palette_query)
        .id(PALETTE_QUERY_ID)
        .on_input(Message::PaletteQueryChanged)
        .on_submit(Message::PaletteSubmitted)
        .size(size)
        .line_height(control_line_height(
            resolved,
            size,
            resolved.input.min_height,
            inp_pad,
        ))
        .font(theme_font(&resolved.input.font))
        .style(styles::text_input(resolved))
        .padding(inp_pad);
    let l = &resolved.list;
    let row_pad = native_theme_iced::padding_or(&l.border.padding, Padding::from(AW_LIST_PADDING));
    let muted = to_color(resolved.defaults.muted_color);
    let mut groups = column![].spacing(gap.widget);
    for (group, entries) in palette_groups(state) {
        if entries.is_empty() {
            continue;
        }
        groups = groups.push(caption_text(state, group, muted));
        groups = groups.push(column(entries.into_iter().map(|entry| {
            button(text(entry.label).themed(&l.item_font, resolved, a11y))
                .padding(row_pad)
                .width(Fill)
                .style(list_row(resolved, false))
                .on_press(Message::PaletteRun(Box::new(entry.action)))
                .into()
        })));
    }
    column![
        query,
        scrollable(groups)
            .direction(scrollable::Direction::Vertical(styles::scrollbar(resolved)))
            .style(styles::scrollable(resolved))
            .height(Length::Shrink),
    ]
    .spacing(gap.widget)
    .into()
}

/// One row of the Preferences dialog: `title` in the dialog's body font and
/// `description` in `text_scale.caption` and `defaults.muted_color` on the
/// left, `control` on the right.
fn preference<'a>(
    state: &'a State,
    title: &'static str,
    description: &'static str,
    control: Element<'a, Message>,
) -> Element<'a, Message> {
    let resolved = &state.current_resolved;
    let gap = Gaps::from_layout(&state.layout);
    row![
        column![
            text(title).themed(&resolved.dialog.body_font, resolved, &state.accessibility),
            caption_text(state, description, to_color(resolved.defaults.muted_color)),
        ]
        .width(Fill),
        control,
    ]
    .spacing(gap.section)
    .align_y(iced::Center)
    .into()
}

/// The Preferences dialog: the accessibility preferences the showcase
/// applies, as the gpui showcase's Settings page lists them. Each row says
/// what this showcase does with its preference, and no more.
fn preferences(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let switch = |on: bool, message: fn(bool) -> Message| -> Element<'_, Message> {
        native_theme_iced::switch(resolved, on, Some(message(!on)))
    };
    let size = scaled_text_size(resolved.combo_box.font.size, a11y);
    let pad = combo_box_padding(resolved);
    let scale = pick_list(
        TEXT_SCALES,
        Some(TextScale(a11y.text_scaling_factor)),
        Message::TextScaleSelected,
    )
    .handle(arrow_handle(resolved))
    .padding(pad)
    .text_line_height(control_line_height(
        resolved,
        size,
        resolved.combo_box.min_height,
        pad,
    ))
    .text_size(size)
    .font(theme_font(&resolved.combo_box.font))
    .style(styles::pick_list(resolved))
    .menu_style(styles::menu(resolved));
    column![
        text("Accessibility").role(section_title(&resolved.text_scale), resolved, a11y),
        caption_text(
            state,
            "Read from the OS with its theme; the showcase applies each one itself",
            to_color(resolved.defaults.muted_color),
        ),
        preference(
            state,
            "Text scale",
            "text_scaling_factor: every text size the showcase sets is multiplied by it",
            scale.into(),
        ),
        preference(
            state,
            "Reduce motion",
            "reduce_motion: the spinner and the Icons page's animations stop",
            switch(a11y.reduce_motion, Message::ReduceMotionToggled),
        ),
        preference(
            state,
            "High contrast",
            "high_contrast: stored with the theme, which the connector builds no differently for it",
            switch(a11y.high_contrast, Message::HighContrastToggled),
        ),
        preference(
            state,
            "Reduce transparency",
            "reduce_transparency: stored with the theme; no dialog here draws a backdrop to leave out",
            switch(a11y.reduce_transparency, Message::ReduceTransparencyToggled),
        ),
    ]
    .spacing(gap.widget)
    .into()
}

/// The About dialog: this crate's name and version, and where the upstream
/// versions it requires are, in `dialog.body_font`; the link, in the
/// platform's link (`styles::button_link`, `link.font`), copies the README's
/// address, since iced opens no browser.
fn about(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let gap = Gaps::from_layout(&state.layout);
    let body = &resolved.dialog.body_font;
    let link = &resolved.link;
    let label = format!(
        "the README's Compatibility table at v{}",
        env!("CARGO_PKG_VERSION")
    );
    let link_button = button(
        rich_text([span::<(), _>(label).underline(link.underline_enabled)])
            .size(scaled_text_size(link.font.size, a11y))
            .font(theme_font(&link.font)),
    )
    .on_press(Message::Copy(COMPATIBILITY_URL.to_string()))
    .style(styles::button_link(resolved))
    .padding(LINK_PADDING);
    column![
        text(ABOUT_NAME_VERSION).themed(body, resolved, a11y),
        text(
            "The iced and iced_aw versions it requires, and those it was verified \
             against, are in"
        )
        .themed(body, resolved, a11y),
        chrome_tooltip(state, link_button, "Copies the address", None),
    ]
    .spacing(gap.widget)
    .into()
}

// ---------------------------------------------------------------------------
// Hover helper: wraps a widget in mouse_area for Widget Info updates
// ---------------------------------------------------------------------------

fn hoverable<'a>(info: String, content: Element<'a, Message>) -> Element<'a, Message> {
    mouse_area(content)
        .on_enter(Message::WidgetHovered(info))
        .on_exit(Message::WidgetUnhovered)
        .into()
}

// ---------------------------------------------------------------------------
// Widget Info builder
// ---------------------------------------------------------------------------

/// Build a multi-line info string for the Widget Info panel, with three of
/// the sections the gpui showcase's `WidgetInfo::to_text` writes (that one
/// adds a "This instance" section and a citation after each colour):
/// - Theme colors: (role, field_name, live hex color)
/// - Theme config: (what, live_value_string)
/// - Not themeable: (what, reason why)
fn widget_tooltip(
    name: &str,
    colors: &[(&str, &str, Color)],
    config: &[(&str, &str)],
    not_themeable: &[(&str, &str)],
) -> String {
    let mut s = format!("{name}\n");

    if !colors.is_empty() {
        s.push_str("\nTheme colors:\n");
        for (role, field, val) in colors {
            s.push_str(&format!("  {role}: {field} {}\n", color_to_hex(*val)));
        }
    }

    if !config.is_empty() {
        s.push_str("\nTheme config:\n");
        for (what, val) in config {
            s.push_str(&format!("  {what}: {val}\n"));
        }
    }

    if !not_themeable.is_empty() {
        s.push_str("\nNot themeable:\n");
        for (what, why) in not_themeable {
            s.push_str(&format!("  {what}: {why}\n"));
        }
    }

    s
}

/// Like [`widget_tooltip`] but appends the active theme font settings.
fn widget_tooltip_themed(
    state: &State,
    name: &str,
    colors: &[(&str, &str, Color)],
    config: &[(&str, &str)],
    not_themeable: &[(&str, &str)],
) -> String {
    let mut s = widget_tooltip(name, colors, config, not_themeable);
    let resolved = &state.current_resolved;
    let ff = family_label(
        &resolved.defaults.font.family,
        resolved_family(&resolved.defaults.font),
        &font_drawn(&resolved.defaults.font),
    );
    let fs = format!("{:.0}px", state.current_resolved.defaults.font.size);
    let mf = family_label(
        &resolved.defaults.mono_font.family,
        resolved_family(&resolved.defaults.mono_font),
        &mono_drawn(resolved),
    );
    let ms = format!("{:.0}px", state.current_resolved.defaults.mono_font.size);
    s.push_str(&format!(
        "\nTheme fonts:\n  Font: {ff} {fs}\n  Mono: {mf} {ms}\n"
    ));
    s
}

// ---------------------------------------------------------------------------
// Tab: Basic
// ---------------------------------------------------------------------------

/// The width of the Basic tab's text fields, drop-down, slider and progress
/// bar. The model states no such width; it is the Basic page's own, the gpui
/// and egui showcases' `BASIC_WIDTH` too, so the three pages lay the same
/// controls out alike.
const BASIC_WIDTH: f32 = 140.0;

/// The Basic tab's progress bar value, on 0 to 100: the datum on display.
const BASIC_PROGRESS: f32 = 40.0;

/// The padding of the Basic tab's link: none, because the theme states none
/// for a link (`docs/property-registry.toml` `[link]` has no padding; a link
/// is inline text), where iced's button, which stands in for one, has its
/// own default.
const LINK_PADDING: Padding = Padding::ZERO;

/// The rows of the Basic tab's drop-down.
const BASIC_FRUITS: [Fruit; 3] = [Fruit::Apple, Fruit::Banana, Fruit::Cherry];

/// The Basic tab's slider value, on 0 to 100: the datum on display.
const BASIC_SLIDER: f32 = 40.0;

/// The width of the Basic tab's text area, list, expanders, card, separator
/// and table. The model states no such width; it is the Basic page's own, the
/// gpui and egui showcases' `BASIC_WIDE` too (Basic v4: 170, so five columns
/// fit the page).
const BASIC_WIDE: f32 = 170.0;

/// The Basic tab's text area: its text, and how many lines tall it is.
const BASIC_TEXT_AREA: &str = "Line one\nLine two\nLine three";
const BASIC_TEXT_AREA_LINES: f32 = 3.0;

/// The Basic tab's segmented control: its segments, and the one selected.
const BASIC_SEGMENTS: [&str; 3] = ["Day", "Week", "Month"];
const BASIC_SEGMENT: usize = 1;

/// The Basic tab's tab bar: two tabs, the first selected.
const BASIC_TABS: [&str; 2] = ["One", "Two"];

/// The Basic tab's list: how many rows it has (`Item 1` to `Item 8`), the one
/// selected (`Item 2`), and how many rows it shows, fewer than it has, so its
/// scrollbar is part of the page.
const BASIC_LIST_ITEMS: usize = 8;
const BASIC_LIST_SELECTED: usize = 1;
const BASIC_LIST_VISIBLE: f32 = 3.0;

/// The Basic tab's number input: its value, stepped by 1 (R11 §D).
const BASIC_NUMBER: i32 = 42;

/// The `widget::Id` of the Basic tab's focused input, which `boot` focuses.
const FOCUSED_INPUT_ID: &str = "showcase-basic-focused-input";

/// The Basic tab's table (R11 §D): each row's id in the list, its name and
/// its size; the row selected.
const BASIC_TABLE_ROWS: [(&str, &str, &str); 2] = [
    ("basic.table.row_1", "a.txt", "1 KB"),
    ("basic.table.row_2", "b.png", "20 KB"),
];
const BASIC_TABLE_SELECTED: usize = 1;

/// The padding iced gives a tooltip's bubble where the showcase sets none:
/// `Tooltip::DEFAULT_PADDING` (iced_widget 0.14.2 `src/tooltip.rs:89`),
/// which iced keeps private. The layout dump reads the bubble's rectangle
/// with it.
const ICED_TOOLTIP_PADDING: f32 = 5.0;

/// How a push button of the Basic page is dressed: the plain button, the
/// primary one, or a toggle button that is on.
#[derive(Clone, Copy)]
enum Push {
    Plain,
    Primary,
    On,
}

/// A toggle button that is on, at rest and under the pointer:
/// `button.checked_background` and `.checked_text_color`
/// (docs/platform-facts.md §2.3), and where the theme states none the
/// button's pressed look, `style`'s `Pressed` (`button.active_background` and
/// `.active_text_color`). Disabled stays `style`'s.
fn toggle_on(
    resolved: &ResolvedTheme,
    style: impl Fn(&Theme, button::Status) -> button::Style,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    let b = &resolved.button;
    let fill = b.checked_background.map(to_color);
    let label = b.checked_text_color.map(to_color);
    move |theme, status| match status {
        button::Status::Disabled => style(theme, status),
        _ => {
            let pressed = style(theme, button::Status::Pressed);
            button::Style {
                background: fill.map(iced::Background::Color).or(pressed.background),
                text_color: label.unwrap_or(pressed.text_color),
                ..pressed
            }
        }
    }
}

/// The list's ids of a checkbox or a radio button and of its parts: the
/// indicator, the label, and the mark (a check mark, a radio dot) where the
/// list names one.
struct CheckIds {
    id: &'static str,
    indicator: &'static str,
    label: &'static str,
    mark: Option<&'static str>,
}

/// The step buttons of the Basic page's number input: no fill of their own,
/// their arrows in the field's text colour, `input.font.color`, and in
/// `input.disabled_text_color` faded by `input.disabled_opacity` disabled.
/// The model states no number input (docs/property-registry.toml), so its
/// buttons take the field's own colours, where iced_aw's default class paints
/// them in the palette's primary (iced_aw 0.14.1
/// `src/style/number_input.rs`, `primary`).
#[cfg(feature = "iced_aw")]
fn number_steps(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_aw::style::Status) -> iced_aw::style::number_input::Style + use<> {
    let i = &resolved.input;
    let ink = to_color(i.font.color);
    let mut disabled = to_color(i.disabled_text_color);
    disabled.a *= i.disabled_opacity;
    move |_theme, status| iced_aw::style::number_input::Style {
        button_background: None,
        icon_color: match status {
            iced_aw::style::Status::Disabled => disabled,
            _ => ink,
        },
    }
}

/// A list row's padding on a side `list.border.padding` leaves unstated:
/// iced has no list, and this is the row padding `iced_aw` gives its
/// `SelectionList` built without one, `padding: 5.0.into()` (iced_aw 0.14.1
/// `src/widget/selection_list.rs:77`), which it keeps in no constant.
const AW_LIST_PADDING: f32 = 5.0;

/// The gap between a Basic page switch and its label: the model states none
/// (`SwitchTheme` has no label gap), and this is the one iced's `Toggler`
/// leaves, `Self::DEFAULT_SIZE / 2.0` (iced_widget 0.14.2 `src/toggler.rs`,
/// `Toggler::new`), which the page's switches drew before they took the
/// connector's `switch`.
const TOGGLER_LABEL_GAP: f32 = iced::widget::Toggler::<'static, Message>::DEFAULT_SIZE / 2.0;

/// The tooltip's padding, where iced can carry it: `Tooltip::padding` is one
/// number for all four sides (iced_widget 0.14.2 `src/tooltip.rs`,
/// `Tooltip::padding`), laid out in from the bubble's edge with the border
/// painted over it, so a theme that states all four sides of
/// `tooltip.border.padding` alike gets that side plus
/// `tooltip.border.line_width`, as `native_theme_iced::padding_inside_border`
/// computes it. Unequal or unstated sides (adwaita's 10 and 6, windows-11's
/// 9, 6 and 8) have no receiver: `None`, and iced's own padding stands.
fn tooltip_padding(resolved: &ResolvedTheme) -> Option<f32> {
    let b = &resolved.tooltip.border;
    let p = &b.padding;
    let side = p.top?;
    [p.right, p.bottom, p.left]
        .iter()
        .all(|s| *s == Some(side))
        .then_some(side + b.line_width)
}

/// The controls the three showcases all draw, in the same order, with the
/// same labels, values and states, packed onto one screen so the gpui, iced
/// and egui captures compare control by control, in five equal columns
/// (Basic v4): buttons (the toggle buttons their third row), checkboxes,
/// radio buttons and switches; text inputs (the focused one their fourth
/// row), a text area and a drop-down; a number input, a slider, a progress
/// bar, a spinner, tabs and a segmented control; typography (the link among
/// its lines), icons (the icon buttons their first row), a card and a
/// separator; a list, expanders and a table. Each control is one hover
/// target, with its widget's info.
///
/// iced has no segmented control and no expander: they are built from
/// buttons in the connector's `styles::segmented_control`, `styles::segment`
/// and `styles::expander`. The card is the Layout page's, a container in
/// `styles::container_card`. The list is list rows in a scrollable and the
/// spinner an arc on a canvas, each drawn from its `list.*` and `spinner.*`
/// leaves. The tab bar is `iced_aw`'s; without the `iced_aw` feature it is
/// drawn as buttons.
fn view_basic<'a>(state: &'a State, btn_pad: Padding, inp_pad: Padding) -> Element<'a, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let c = &resolved.checkbox;
    // A group: its heading, tagged `<group>.heading`, over its controls,
    // each of which carries its own tags.
    let group = |id: &'static str, label: &'a str, controls: Element<'a, Message>| {
        let heading = text(label).role(section_title(ts), resolved, a11y);
        Element::from(column![tagged(id, heading), controls].spacing(gap.widget))
    };
    let font = &resolved.button.font;
    // `button.min_width` and `button.min_height`, as a floor under the label.
    let btn_min = native_theme_iced::button_content_min_size(resolved);
    // A push button of the page: the label tagged `<id>.label`, the button
    // `id`.
    let push = |id: &'static str,
                label_id: &'static str,
                label: &'a str,
                kind: Push,
                on_press: Option<Message>| {
        let pushed = match kind {
            Push::Plain => button(at_least(
                tagged(label_id, text(label).themed(font, resolved, a11y)),
                btn_min,
            ))
            .style(styles::button(resolved))
            .padding(btn_pad),
            Push::Primary => button(at_least(
                tagged(label_id, text(label).themed(font, resolved, a11y)),
                btn_min,
            ))
            .style(styles::button_primary(resolved))
            .padding(btn_pad),
            Push::On => button(at_least(
                tagged(label_id, text(label).themed(font, resolved, a11y)),
                btn_min,
            ))
            .style(toggle_on(resolved, styles::button(resolved)))
            .padding(btn_pad),
        };
        // At its own width even where its row runs past the column, as the
        // gpui and egui buttons are, rather than squeezed under its label.
        tagged(id, pushed.on_press_maybe(on_press)).loose()
    };
    // The tip's padding: `tooltip_padding` where iced can carry it, iced's
    // own otherwise.
    let tip_padding = tooltip_padding(resolved);
    let tip = tooltip(
        push(
            "basic.buttons.tooltip",
            "basic.buttons.tooltip.label",
            "Tooltip",
            Push::Plain,
            Some(Message::ButtonPressed),
        ),
        // `tooltip.max_width` is no `Style` field: iced wraps the tip's own
        // element, so the element is given the width.
        tagged(
            "basic.buttons.tooltip.bubble.text",
            container(text("A tooltip").themed(&resolved.tooltip.font, resolved, a11y))
                .max_width(resolved.tooltip.max_width),
        )
        .part("basic.buttons.tooltip.bubble", move |layout| {
            Some(
                layout
                    .bounds()
                    .expand(tip_padding.unwrap_or(ICED_TOOLTIP_PADDING)),
            )
        }),
        tooltip::Position::Bottom,
    )
    .gap(sp.xs)
    .style(styles::tooltip(resolved));
    let tip = match tip_padding {
        Some(padding) => tip.padding(padding),
        None => tip,
    };

    let buttons = group(
        "basic.buttons.heading",
        "Buttons",
        column![
            row![
                probe(
                    probes::BASIC_BUTTON,
                    Length::Shrink,
                    push(
                        "basic.buttons.default",
                        "basic.buttons.default.label",
                        "Button",
                        Push::Plain,
                        Some(Message::ButtonPressed),
                    ),
                ),
                push(
                    "basic.buttons.primary",
                    "basic.buttons.primary.label",
                    "Primary",
                    Push::Primary,
                    Some(Message::ButtonPressed),
                ),
            ]
            .spacing(gap.widget),
            row![
                push(
                    "basic.buttons.disabled",
                    "basic.buttons.disabled.label",
                    "Disabled",
                    Push::Plain,
                    None,
                ),
                tip,
            ]
            .spacing(gap.widget),
            // The toggle buttons, off and on (Basic v4's third row).
            row![
                push(
                    "basic.buttons.toggle_off",
                    "basic.buttons.toggle_off.label",
                    "Off",
                    Push::Plain,
                    Some(Message::ButtonPressed),
                ),
                push(
                    "basic.buttons.toggle_on",
                    "basic.buttons.toggle_on.label",
                    "On",
                    Push::On,
                    Some(Message::ButtonPressed),
                ),
            ]
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let check =
        |ids: CheckIds, checked: bool, label: &'a str, enabled: bool| -> Element<'a, Message> {
            let stated = c.check_mark_stroke_width;
            let boxed = checkbox(checked)
                .label(label)
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .text_size(scaled_text_size(c.font.size, a11y))
                .text_line_height(native_theme_iced::line_height_multiplier(resolved))
                .font(theme_font(&c.font))
                .style(glyphless_when_stated(styles::checkbox(resolved), stated));
            // A box with no `on_toggle` is a disabled one (checkbox.rs:154).
            let boxed = if enabled {
                boxed.on_toggle(|_| Message::BasicHeld)
            } else {
                boxed
            };
            // The mark box: inside the padding where the theme states the
            // padding (GNOME's 3, docs/platform-facts.md §2.5: the 20 px
            // indicator is the 14 px box plus 3 on each side, its outline an
            // inset box-shadow that takes no room), and egui's own share of
            // the box, centred, where it does not.
            let inset = |side: Option<f32>| match side {
                Some(pad) => pad.max(0.0),
                None => c.indicator_width * (1.0 - EGUI_ICON_WIDTH_INNER / EGUI_ICON_WIDTH) / 2.0,
            };
            let (inset_x, inset_y) = (inset(c.border.padding.left), inset(c.border.padding.top));
            // The box and the label are the checkbox's two layout nodes
            // (`layout::next_to_each_other`, iced_widget 0.14.2
            // `src/checkbox.rs:280`), under the stack's first layer where the
            // mark is laid over it.
            let tag =
                |content: Element<'a, Message>, at: &'static [usize], label: &'static [usize]| {
                    let tagged = tagged(ids.id, content)
                        .node(ids.indicator, at)
                        .node(ids.label, label);
                    match ids.mark {
                        Some(mark) => tagged.part(mark, move |layout| {
                            node_at(layout, at)
                                .map(|b| b.bounds().shrink(Padding::from([inset_y, inset_x])))
                        }),
                        None => tagged,
                    }
                };
            let Some(stroke) = stated else {
                return tag(boxed.into(), &[0], &[1]).into();
            };
            // The mark the theme states a line for, over iced's glyph made
            // transparent: laid out whether checked or not, so the box keeps its
            // place in the widget tree as it toggles.
            let status = if enabled {
                checkbox::Status::Active {
                    is_checked: checked,
                }
            } else {
                checkbox::Status::Disabled {
                    is_checked: checked,
                }
            };
            // The style's mark colour over the box's fill over the page, one
            // opaque colour: a translucent one comes out of iced's mesh
            // pipeline darker than the same colour on a quad, where the box's
            // fill blends as the connector composites
            // (`styles::composite_over`). A disabled check fades as a whole,
            // its mark over its fill and the pair at
            // `checkbox.disabled_opacity` over the page (docs/platform-facts.md
            // §2.1.6: GNOME's `filter: Opacity` on the whole widget, which
            // keeps its normal colours), where the style fades each colour on
            // its own: each is taken back to its unfaded self first.
            let style = styles::checkbox(resolved)(&Theme::Light, status);
            let page = to_color(resolved.window.background_color);
            let opacity = if enabled || !c.disabled_opacity.is_finite() {
                1.0
            } else {
                c.disabled_opacity.clamp(0.0, 1.0)
            };
            let unfaded = |colour: Color| Color {
                a: if opacity > 0.0 {
                    (colour.a / opacity).min(1.0)
                } else {
                    0.0
                },
                ..colour
            };
            let colour = match style.background {
                iced::Background::Color(fill) => {
                    let whole = over(unfaded(style.icon_color), over(unfaded(fill), page));
                    over(
                        Color {
                            a: opacity,
                            ..whole
                        },
                        page,
                    )
                }
                _ => style.icon_color,
            };
            let mark = canvas(CheckMark {
                checked,
                color: colour,
                stroke,
                inset: (inset_x, inset_y),
            })
            .width(Length::Fixed(c.indicator_width))
            .height(Length::Fixed(c.indicator_width));
            let seat = container(mark)
                .height(Length::Fill)
                .align_y(iced::alignment::Vertical::Center);
            tag(
                iced::widget::Stack::new().push(boxed).push(seat).into(),
                &[0, 0],
                &[0, 1],
            )
            .into()
        };
    let checkboxes = group(
        "basic.checkboxes.heading",
        "Checkboxes",
        column![
            check(
                CheckIds {
                    id: "basic.checkboxes.unchecked",
                    indicator: "basic.checkboxes.unchecked.indicator",
                    label: "basic.checkboxes.unchecked.label",
                    mark: None,
                },
                false,
                "Unchecked",
                true
            ),
            check(
                CheckIds {
                    id: "basic.checkboxes.checked",
                    indicator: "basic.checkboxes.checked.indicator",
                    label: "basic.checkboxes.checked.label",
                    mark: Some("basic.checkboxes.checked.mark"),
                },
                true,
                "Checked",
                true
            ),
            check(
                CheckIds {
                    id: "basic.checkboxes.disabled",
                    indicator: "basic.checkboxes.disabled.indicator",
                    label: "basic.checkboxes.disabled.label",
                    mark: Some("basic.checkboxes.disabled.mark"),
                },
                true,
                "Disabled",
                false
            ),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // `native_theme_iced::radio` lays its dot over the radio in a stack
    // where the theme states the dot's size, and leaves the radio as it is
    // where it does not (src/lib.rs, `radio`): the radio's circle and label
    // are its two layout nodes (`layout::next_to_each_other`, iced_widget
    // 0.14.2 `src/radio.rs`), under the stack's first layer.
    let dot = c.radio_dot_diameter.filter(|d| d.is_finite() && *d >= 0.0);
    let radio_nodes: (&'static [usize], &'static [usize]) = match dot {
        Some(_) => (&[0, 0], &[0, 1]),
        None => (&[0], &[1]),
    };
    let option = |ids: CheckIds, label: &'a str, value: usize| {
        let (circle, text_node) = radio_nodes;
        let option = tagged(
            ids.id,
            native_theme_iced::radio(
                resolved,
                radio(
                    label,
                    value,
                    Some(state.basic_radio),
                    Message::BasicRadioSelected,
                )
                .text_size(scaled_text_size(c.font.size, a11y))
                .text_line_height(native_theme_iced::line_height_multiplier(resolved))
                .font(theme_font(&c.font)),
                state.basic_radio == value,
            ),
        )
        .node(ids.indicator, circle)
        .node(ids.label, text_node);
        match ids.mark {
            // The dot, centred in the circle: the theme's diameter, or iced's
            // own, half the circle (iced_widget 0.14.2 `src/radio.rs:409-433`).
            Some(mark) => option.part(mark, move |layout| {
                node_at(layout, circle).map(|b| {
                    let side = dot.unwrap_or(b.bounds().width / 2.0);
                    let centre = b.bounds().center();
                    iced::Rectangle::new(
                        iced::Point::new(centre.x - side / 2.0, centre.y - side / 2.0),
                        iced::Size::new(side, side),
                    )
                })
            }),
            None => option,
        }
    };
    let radios = group(
        "basic.radios.heading",
        "Radio buttons",
        column![
            option(
                CheckIds {
                    id: "basic.radios.option_a",
                    indicator: "basic.radios.option_a.indicator",
                    label: "basic.radios.option_a.label",
                    mark: Some("basic.radios.option_a.dot"),
                },
                "Option A",
                0
            ),
            probe(
                probes::BASIC_RADIO_B,
                Length::Shrink,
                option(
                    CheckIds {
                        id: "basic.radios.option_b",
                        indicator: "basic.radios.option_b.indicator",
                        label: "basic.radios.option_b.label",
                        mark: None,
                    },
                    "Option B",
                    1
                )
            ),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let input_size = scaled_text_size(resolved.input.font.size, a11y);
    // `input.min_height`, as the line box that fills it inside the padding.
    let input_line = control_line_height(resolved, input_size, resolved.input.min_height, inp_pad);
    let field = |placeholder: &'a str, value: &'a str| {
        text_input(placeholder, value)
            .size(input_size)
            .line_height(input_line)
            .font(theme_font(&resolved.input.font))
            .style(styles::text_input(resolved))
            .padding(inp_pad)
            .width(Length::Fixed(BASIC_WIDTH))
    };
    // The text a field shows, `shown` (its value, or its placeholder while
    // empty), as the input's font sets it.
    let input_text = |shown: &str| MeasuredText {
        content: shown.to_string(),
        size: input_size,
        line: input_size * resolved.defaults.line_height,
        font: theme_font(&resolved.input.font),
    };
    // A field and its text: the text sits in the field's first layout node,
    // the line inside its padding (iced_widget 0.14.2 `src/text_input.rs`,
    // `layout`).
    let fielded = |id: &'static str, field: text_input::TextInput<'a, Message>, shown: &str| {
        let field = tagged(id, field);
        match listed_part(id, "text") {
            Some(text_id) => field.text_part(text_id, input_text(shown), |layout| {
                node_at(layout, &[0]).map(|node| node.bounds())
            }),
            None => field,
        }
    };
    // A field with no `on_input` is a disabled one (text_input.rs:170).
    let inputs = group(
        "basic.text_inputs.heading",
        "Text inputs",
        column![
            probe(
                probes::BASIC_TEXT_INPUT,
                Length::Shrink,
                fielded(
                    "basic.text_inputs.placeholder",
                    field("Placeholder", &state.basic_hint).on_input(Message::BasicHintChanged),
                    if state.basic_hint.is_empty() {
                        "Placeholder"
                    } else {
                        &state.basic_hint
                    },
                ),
            ),
            fielded(
                "basic.text_inputs.filled",
                field("", &state.basic_text).on_input(Message::BasicTextChanged),
                &state.basic_text,
            ),
            fielded(
                "basic.text_inputs.disabled",
                field("", "Disabled"),
                "Disabled"
            ),
            // A field holding the keyboard focus from the start (`boot`
            // focuses it), so its Focused border shows (Basic v4's fourth
            // row).
            fielded(
                "basic.text_inputs.focused",
                field("", &state.basic_focused)
                    .id(FOCUSED_INPUT_ID)
                    .on_input(Message::BasicFocusedChanged),
                &state.basic_focused,
            ),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // The number input: the field and its text only (the list's note), the
    // field iced_aw's `NumberInput` in the input's style, its text the
    // field's text node (`NumberInput::layout` lays the text input out
    // first, iced_aw 0.14.1 `src/widget/number_input.rs:584`).
    #[cfg(feature = "iced_aw")]
    let number: Element<'a, Message> = tagged(
        "basic.number_input.field",
        iced_aw::NumberInput::new(&state.basic_number, .., Message::BasicNumberChanged)
            .step(1)
            .set_size(input_size)
            .line_height(input_line)
            .font(theme_font(&resolved.input.font))
            .padding(inp_pad)
            .input_style(styles::text_input(resolved))
            .style(number_steps(resolved))
            .width(Length::Fixed(BASIC_WIDTH)),
    )
    .text_part(
        "basic.number_input.field.text",
        input_text(&state.basic_number.to_string()),
        |layout| node_at(layout, &[0, 0]).map(|node| node.bounds()),
    )
    .into();
    // Without iced_aw, the value in a plain field.
    #[cfg(not(feature = "iced_aw"))]
    let number: Element<'a, Message> = fielded(
        "basic.number_input.field",
        field("", &state.basic_number_text),
        &state.basic_number_text,
    )
    .into();
    let number = group("basic.number_input.heading", "Number input", number);

    let combo_size = scaled_text_size(resolved.combo_box.font.size, a11y);
    let combo_pad = combo_box_padding(resolved);
    let arrow_size = resolved.combo_box.arrow_icon_size;
    let drop_down = group(
        "basic.drop_down.heading",
        "Drop-down",
        probe(
            probes::BASIC_PICK_LIST,
            Length::Shrink,
            tagged(
                "basic.drop_down.trigger",
                pick_list(
                    BASIC_FRUITS,
                    Some(state.basic_fruit),
                    Message::BasicFruitSelected,
                )
                .handle(arrow_handle(resolved))
                .padding(combo_pad)
                // `combo_box.min_height`, as the line box that fills it inside the padding.
                .text_line_height(control_line_height(
                    resolved,
                    combo_size,
                    resolved.combo_box.min_height,
                    combo_pad,
                ))
                .text_size(combo_size)
                .font(theme_font(&resolved.combo_box.font))
                .style(styles::pick_list(resolved))
                .menu_style(styles::menu(resolved))
                .width(Length::Fixed(BASIC_WIDTH)),
            )
            // The text inside the padding, left of the arrow; the arrow
            // right-aligned at the right padding's inner edge, centred
            // (iced_widget 0.14.2 `src/pick_list.rs:636-660`).
            .text_part(
                "basic.drop_down.trigger.text",
                MeasuredText {
                    content: state.basic_fruit.to_string(),
                    size: combo_size,
                    line: combo_size * resolved.defaults.line_height,
                    font: theme_font(&resolved.combo_box.font),
                },
                move |layout| Some(layout.bounds().shrink(combo_pad)),
            )
            .part("basic.drop_down.trigger.arrow", move |layout| {
                let inner = layout.bounds().shrink(combo_pad);
                Some(iced::Rectangle::new(
                    iced::Point::new(
                        inner.x + inner.width - arrow_size,
                        inner.center_y() - arrow_size / 2.0,
                    ),
                    iced::Size::new(arrow_size, arrow_size),
                ))
            }),
        ),
    );

    // The slider's rail and handle as iced draws them (iced_widget 0.14.2
    // `src/slider.rs:446-515`): the rail `slider.track_height` tall across
    // the width, centred; the handle a circle `slider.thumb_diameter` across
    // at the value's offset; the filled rail from the start to the handle's
    // centre.
    let sl = &resolved.slider;
    let (rail, handle) = (sl.track_height, sl.thumb_diameter);
    let slider_value = state.basic_slider / 100.0;
    let rail_rect = move |b: iced::Rectangle| {
        iced::Rectangle::new(
            iced::Point::new(b.x, b.center_y() - rail / 2.0),
            iced::Size::new(b.width, rail),
        )
    };
    let handle_at = move |b: iced::Rectangle| (b.width - handle) * slider_value;
    let slider_group = group(
        "basic.slider.heading",
        "Slider",
        tagged(
            "basic.slider.control",
            // As tall as what it paints: the thumb, or the track where that is
            // taller; iced's own height (16) would leave a larger thumb
            // outside the control.
            slider(0.0..=100.0, state.basic_slider, Message::BasicSliderChanged)
                .style(styles::slider(resolved))
                .height(handle.max(rail))
                .width(Length::Fixed(BASIC_WIDTH)),
        )
        .part("basic.slider.control.track", move |l| {
            Some(rail_rect(l.bounds()))
        })
        .part("basic.slider.control.fill", move |l| {
            let b = l.bounds();
            Some(iced::Rectangle {
                width: handle_at(b) + handle / 2.0,
                ..rail_rect(b)
            })
        })
        .part("basic.slider.control.thumb", move |l| {
            let b = l.bounds();
            Some(iced::Rectangle::new(
                iced::Point::new(b.x + handle_at(b), b.center_y() - handle / 2.0),
                iced::Size::new(handle, handle),
            ))
        })
        .into(),
    );

    let progress = group(
        "basic.progress_bar.heading",
        "Progress bar",
        tagged(
            "basic.progress_bar.bar",
            progress_bar(0.0..=100.0, BASIC_PROGRESS)
                .length(Length::Fixed(BASIC_WIDTH))
                .girth(Length::Fixed(resolved.progress_bar.track_height))
                .style(styles::progress_bar(resolved)),
        )
        .part("basic.progress_bar.bar.fill", |l| {
            let b = l.bounds();
            Some(iced::Rectangle {
                width: b.width * BASIC_PROGRESS / 100.0,
                ..b
            })
        })
        .into(),
    );

    // The connector's switch, whose track takes `switch.track_width`: iced's
    // toggler draws every track twice its height. The model states no font
    // and no label gap for a switch: the label is body text, at the gap
    // iced's toggler leaves. A disabled switch's label is the disabled text
    // colour, faded with the switch by `switch.disabled_opacity`, as the gpui
    // and egui switches draw it (docs/platform-facts.md §2.1.6).
    let disabled_label = {
        let mut c = to_color(resolved.defaults.disabled_text_color);
        c.a *= resolved.switch.disabled_opacity;
        c
    };
    // The thumb inside the track as `native_theme_iced::switch` seats it:
    // inset by half the difference of the two heights, at the track's right
    // end while on.
    let sw = &resolved.switch;
    let (thumb, thumb_inset) = (
        sw.thumb_diameter,
        ((sw.track_height - sw.thumb_diameter) / 2.0).max(0.0),
    );
    let switch = |id: &'static str,
                  probe_id: &'static str,
                  on: bool,
                  label: &'a str,
                  enabled: bool|
     -> Element<'a, Message> {
        let label = text(label).body(resolved, a11y);
        let parts = |name: &str| listed_part(id, name);
        let mut control = tagged(
            id,
            row![
                probe(
                    probe_id,
                    Length::Shrink,
                    native_theme_iced::switch(resolved, on, enabled.then_some(Message::BasicHeld)),
                ),
                if enabled {
                    label
                } else {
                    label.color(disabled_label)
                },
            ]
            .spacing(TOGGLER_LABEL_GAP)
            .align_y(iced::Alignment::Center),
        );
        if let Some(track) = parts("track") {
            control = control.node(track, &[0]);
        }
        if let Some(label) = parts("label") {
            control = control.node(label, &[1]);
        }
        if let Some(thumb_id) = parts("thumb") {
            control = control.part(thumb_id, move |layout| {
                node_at(layout, &[0]).map(|track| {
                    let t = track.bounds();
                    let x = if on {
                        t.x + t.width - thumb_inset - thumb
                    } else {
                        t.x + thumb_inset
                    };
                    iced::Rectangle::new(
                        iced::Point::new(x, t.center_y() - thumb / 2.0),
                        iced::Size::new(thumb, thumb),
                    )
                })
            });
        }
        control.into()
    };
    let switches = group(
        "basic.switches.heading",
        "Switches",
        column![
            switch(
                "basic.switches.off",
                probes::BASIC_SWITCH_OFF,
                false,
                "Off",
                true
            ),
            switch(
                "basic.switches.on",
                probes::BASIC_SWITCH_ON,
                true,
                "On",
                true
            ),
            switch(
                "basic.switches.disabled",
                probes::BASIC_SWITCH_DISABLED,
                true,
                "Disabled",
                false
            ),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // Three of the theme's text lines, inside the multi-line field's own
    // padding (docs/platform-facts.md §2.29).
    let area_line = input_size * resolved.defaults.line_height;
    let area_pad = native_theme_iced::text_area_padding(resolved);
    let text_area = group(
        "basic.text_area.heading",
        "Text area",
        probe(
            probes::BASIC_TEXT_AREA,
            Length::Shrink,
            tagged(
                "basic.text_area.field",
                text_editor(&state.basic_text_area)
                    .on_action(Message::BasicTextAreaAction)
                    .size(input_size)
                    .line_height(iced::Pixels(area_line))
                    .font(theme_font(&resolved.input.font))
                    .style(styles::text_editor(resolved))
                    .padding(area_pad)
                    .width(BASIC_WIDE)
                    .height(Length::Fixed(
                        BASIC_TEXT_AREA_LINES * area_line + area_pad.y(),
                    )),
            )
            // The text: the three lines inside the padding.
            .text_part(
                "basic.text_area.field.text",
                MeasuredText {
                    content: state.basic_text_area.text(),
                    size: input_size,
                    line: area_line,
                    font: theme_font(&resolved.input.font),
                },
                move |l| Some(l.bounds().shrink(area_pad)),
            ),
        ),
    );

    let spinner = probe(
        probes::BASIC_SPINNER,
        Length::Shrink,
        tagged(
            "basic.spinner.indicator",
            state
                .basic_spinner
                .view(state.animation_start.elapsed(), state.motion_reduced()),
        ),
    );
    let spinner_group = group("basic.spinner.heading", "Spinner", spinner);

    let tab_t = &resolved.tab;
    #[cfg(not(feature = "iced_aw"))]
    let tab_pad = native_theme_iced::padding_or(&tab_t.border.padding, button::DEFAULT_PADDING);
    #[cfg(feature = "iced_aw")]
    let tab_row: Element<'a, Message> = {
        let [one, two] = BASIC_TABS;
        let bar = TabBar::new(Message::BasicTabSelected)
            .push(0, TabLabel::Text(one.to_string()))
            .push(1, TabLabel::Text(two.to_string()))
            .set_active_tab(&state.basic_tab)
            .text_size(scaled_text_size(tab_t.font.size, a11y))
            .text_font(theme_font(&tab_t.font))
            .tab_width(Length::Fixed(tab_t.min_width))
            .height(Length::Fixed(tab_t.min_height))
            .width(Length::Shrink)
            .style(styles::aw::tab_bar(resolved));
        // tab.item_gap between the tabs, where stated; iced_aw's own none
        // otherwise (widget/tab_bar.rs:43, `DEFAULT_SPACING`).
        let bar = match tab_t.item_gap {
            Some(gap) => bar.spacing(gap),
            None => bar,
        };
        // iced_aw keeps a tab's default padding private (widget/tab_bar.rs:41)
        // and takes a padding whole.
        let bar: Element<'a, Message> =
            match native_theme_iced::stated_padding(&tab_t.border.padding) {
                Some(padding) => bar.padding(padding).into(),
                None => bar.into(),
            };
        // The tabs are the bar's row's nodes, one per label
        // (iced_aw 0.14.1 `src/widget/tab_bar.rs:406-409`).
        let bar: Element<'a, Message> = tagged("basic.tabs.bar", bar)
            .loose()
            .node("basic.tabs.one", &[0])
            .node("basic.tabs.two", &[1])
            .into();
        // The active tab's line, where the theme states one, over that tab's
        // place: iced_aw's tabs are `tab_width` wide, `spacing` apart
        // (`src/widget/tab_bar.rs:490-510`), and its style has no line.
        let slots = (0..BASIC_TABS.len()).map(|i| {
            let line = (i == state.basic_tab)
                .then(|| native_theme_iced::tab_indicator_line(resolved))
                .flatten();
            container(line.unwrap_or_else(|| iced::widget::Space::new().into()))
                .width(Length::Fixed(tab_t.min_width))
                .height(Fill)
                .into()
        });
        // tab.item_gap apart, as the bar's tabs are; a row's own none, as
        // iced_aw's `DEFAULT_SPACING` is, where unstated.
        let slots = match tab_t.item_gap {
            Some(gap) => row(slots).spacing(gap),
            None => row(slots),
        };
        iced::widget::Stack::new()
            .push(bar)
            .push(slots.height(Fill))
            .into()
    };
    // Without iced_aw, the page tab strip's tabs: buttons padded like the
    // platform's tabs, the open one the call to action.
    #[cfg(not(feature = "iced_aw"))]
    let tab_row: Element<'a, Message> = tagged(
        "basic.tabs.bar",
        row(BASIC_TABS.iter().enumerate().map(|(i, label)| {
            let tab = button(text(*label).themed(&tab_t.font, resolved, a11y))
                .padding(tab_pad)
                .on_press(Message::ButtonPressed);
            if i == 0 {
                native_theme_iced::tab_indicator(
                    resolved,
                    tab.style(styles::button_primary(resolved)),
                    true,
                )
            } else {
                tab.style(styles::button(resolved)).into()
            }
        }))
        .spacing(tab_t.item_gap.unwrap_or(sp.xs)),
    )
    .loose()
    .node("basic.tabs.one", &[0])
    .node("basic.tabs.two", &[1])
    .into();
    let tabs = group("basic.tabs.heading", "Tabs", tab_row);

    let sc = &resolved.segmented_control;
    // A segment has no border of its own (the control's outline is the
    // container's), so its padding is the stated sides as they are.
    let seg_pad = native_theme_iced::padding_or(&sc.border.padding, button::DEFAULT_PADDING);
    // `segment_height` is each segment's outer height ("height of each
    // segment button", docs/platform-facts.md §2 dimension rules): a segment
    // has no border of its own, so its content is that less its padding, and
    // the control is the segments inside its outline.
    let seg_min = iced::Size::new(0.0, (sc.segment_height - seg_pad.y()).max(0.0));
    let segments = BASIC_SEGMENTS.iter().enumerate().map(|(i, label)| {
        button(at_least(
            text(*label).themed(&sc.font, resolved, a11y),
            seg_min,
        ))
        .padding(seg_pad)
        .style(styles::segment(
            resolved,
            i == state.basic_segment,
            styles::SegmentPosition::of(i, BASIC_SEGMENTS.len()),
        ))
        .on_press(Message::BasicSegmentSelected(i))
        .into()
    });
    // A divider: the gap between two segments, where the outline shows
    // through, `segmented_control.separator_width` wide.
    let divider = |before: usize| {
        move |layout: iced::advanced::Layout<'_>| {
            node_at(layout, &[0, before]).map(|segment| {
                let s = segment.bounds();
                iced::Rectangle {
                    x: s.x + s.width,
                    width: sc.separator_width,
                    ..s
                }
            })
        }
    };
    let segmented = group(
        "basic.segmented.heading",
        "Segmented control",
        probe(
            probes::BASIC_SEGMENTED,
            Length::Shrink,
            tagged(
                "basic.segmented.control",
                container(row(segments).spacing(sc.separator_width))
                    .padding(sc.border.line_width)
                    .style(styles::segmented_control(resolved)),
            )
            .node("basic.segmented.day", &[0, 0])
            .node("basic.segmented.week", &[0, 1])
            .node("basic.segmented.month", &[0, 2])
            .part("basic.segmented.divider_1", divider(0))
            .part("basic.segmented.divider_2", divider(1)),
        ),
    );

    // The list is built from iced's own widgets: `iced_aw`'s `SelectionList`
    // draws its rows' text at their left edge whatever padding it is given
    // (iced_aw 0.14.1 `src/widget/selection_list/list.rs:263-276`), rounds
    // nothing and styles no scrollbar, so it could not reach `list.*`.
    let list_t = &resolved.list;
    let label = scaled_text_size(list_t.item_font.size, a11y);
    let row_pad =
        native_theme_iced::padding_or(&list_t.border.padding, Padding::from(AW_LIST_PADDING));
    // KDE's rows size to their content: the label's line box and the padding
    // the theme states above and below it.
    let label_line = label * resolved.defaults.line_height;
    let row_height = list_t.row_height.unwrap_or(label_line + row_pad.y());
    const LIST_ROW_IDS: [&str; 3] = ["basic.list.row_1", "basic.list.row_2", "basic.list.row_3"];
    let rows = (0..BASIC_LIST_ITEMS).map(|i| {
        let item = button(
            text(format!("Item {}", i + 1))
                .typeset(&list_t.item_font, a11y)
                .line_height(iced::Pixels(label_line)),
        )
        .padding(row_pad)
        .width(Fill)
        .height(Length::Fixed(row_height))
        .style(list_row(resolved, state.basic_list_selected == Some(i)))
        .on_press(Message::BasicListSelected(i));
        match LIST_ROW_IDS.get(i) {
            Some(id) => tagged(id, item).into(),
            None => item.into(),
        }
    });
    // The scrollbar and its thumb as iced lays them out (iced_widget 0.14.2
    // `src/scrollable.rs:1957-2015`): the groove `scrollbar.groove_width`
    // wide at the right edge, the thumb `scrollbar.thumb_width` wide centred
    // in it, as tall as the part of the rows shown, at the top.
    let (groove, scroller) = (
        resolved.scrollbar.groove_width,
        resolved.scrollbar.thumb_width,
    );
    let shown = BASIC_LIST_VISIBLE / BASIC_LIST_ITEMS as f32;
    let groove_rect = move |b: iced::Rectangle| {
        let total = groove.max(scroller);
        iced::Rectangle {
            x: b.x + b.width - total / 2.0 - groove / 2.0,
            width: groove,
            ..b
        }
    };
    let frame = iced::Border {
        color: to_color(list_t.border.color),
        width: list_t.border.line_width,
        radius: list_t.border.corner_radius.into(),
    };
    let list_fill = to_color(list_t.background_color);
    let list: Element<'a, Message> = probe(
        probes::BASIC_LIST,
        Length::Shrink,
        // The rows inside the frame's line, which iced paints over the
        // container's padding.
        tagged(
            "basic.list.frame",
            container(
                scrollable(column(rows))
                    .direction(scrollable::Direction::Vertical(styles::scrollbar(resolved)))
                    .style(styles::scrollable(resolved))
                    .height(Length::Fixed(BASIC_LIST_VISIBLE * row_height)),
            )
            .padding(list_t.border.line_width)
            .width(Length::Fixed(BASIC_WIDE))
            .style(move |_theme: &Theme| container::Style {
                background: Some(iced::Background::Color(list_fill)),
                border: frame,
                ..container::Style::default()
            }),
        )
        .part("basic.list.scrollbar", move |l| {
            node_at(l, &[0]).map(|s| groove_rect(s.bounds()))
        })
        .part("basic.list.scrollbar.thumb", move |l| {
            node_at(l, &[0]).map(|s| {
                let g = groove_rect(s.bounds());
                iced::Rectangle {
                    x: g.center_x() - scroller / 2.0,
                    width: scroller,
                    height: (g.height * shown).max(2.0),
                    ..g
                }
            })
        }),
    );
    let list_group = group("basic.list.heading", "List", list);

    let x = &resolved.expander;
    // The header's padding: inside its own border where it draws one; where
    // the theme states whether the expander is framed the header draws none
    // ([`unframed_when_stated`]) -- a framed one sits inside the item's
    // frame, `expander.border.line_width` in -- so the stated sides are its
    // padding as they are.
    let x_pad = match x.frame_enabled {
        Some(_) => native_theme_iced::padding_or(&x.border.padding, button::DEFAULT_PADDING),
        None => native_theme_iced::padding_inside_border(&x.border, button::DEFAULT_PADDING),
    };
    let x_min = iced::Size::new(0.0, (x.header_height - x_pad.y()).max(0.0));
    let arrow_color = native_theme_iced::expander_arrow_color(resolved);
    // expander.arrow_side, arrow_gap, content_indent and frame_enabled
    // (docs/platform-facts.md §2.27); where one is unstated the showcase's
    // own stands: the arrow before the title, the widget gap, the body
    // under the title, and each header framed by `styles::expander`.
    let trailing = x.arrow_side == Some(ArrowSide::Trailing);
    let arrow_gap = x.arrow_gap.unwrap_or(gap.widget);
    // A header, tagged `<id>.header`, its arrow `<id>.arrow` and its title
    // `<id>.title`.
    let header = move |id: &'static str, title: &'a str, expanded: bool, toggle: Message| {
        let arrow = || {
            let arrow = canvas(DisclosureArrow {
                expanded,
                trailing,
                color: arrow_color,
            })
            .width(Length::Fixed(x.arrow_icon_size))
            .height(Length::Fixed(x.arrow_icon_size));
            match listed_part(id, "arrow") {
                Some(arrow_id) => Element::from(tagged(arrow_id, arrow)),
                None => arrow.into(),
            }
        };
        let tag_title = |title: text::Text<'a>| -> Element<'a, Message> {
            match listed_part(id, "title") {
                Some(title_id) => tagged(title_id, title).into(),
                None => title.into(),
            }
        };
        let head = button(at_least(
            row![]
                .push((!trailing).then(arrow))
                .push(tag_title(text(title).themed(&x.font, resolved, a11y)))
                // A trailing arrow at the header's far end.
                .push(trailing.then(|| space().width(Fill)))
                .push(trailing.then(arrow))
                .spacing(arrow_gap)
                .align_y(iced::Center),
            x_min,
        ))
        .padding(x_pad)
        .width(Fill)
        .style(unframed_when_stated(
            styles::expander(resolved),
            x.frame_enabled,
        ))
        .on_press(toggle);
        match listed_part(id, "header") {
            Some(header_id) => Element::from(tagged(header_id, head)),
            None => head.into(),
        }
    };
    // The body sits content_indent in, or under the title, past the arrow
    // and its gap, where the theme states no indent.
    let body_inset = x
        .content_indent
        .unwrap_or(x_pad.left + x.arrow_icon_size + arrow_gap);
    let frame = iced::Border {
        color: to_color(x.border.color),
        width: x.border.line_width,
        radius: x.border.corner_radius.into(),
    };
    let item = move |id: &'static str,
                     head: Element<'a, Message>,
                     body: Option<&'a str>|
          -> Element<'a, Message> {
        let mut item = column![head].spacing(gap.widget);
        if let Some(body) = body {
            let body = text(body).body(resolved, a11y);
            let body: Element<'a, Message> = match listed_part(id, "body") {
                Some(body_id) => tagged(body_id, body).into(),
                None => body.into(),
            };
            item = item.push(container(body).padding(Padding::ZERO.left(body_inset)));
        }
        let item: Element<'a, Message> = match x.frame_enabled {
            Some(true) => container(item)
                .padding(x.border.line_width)
                .width(Length::Fixed(BASIC_WIDE))
                .style(move |_: &Theme| container::Style {
                    border: frame,
                    ..container::Style::default()
                })
                .into(),
            _ => container(item).width(Length::Fixed(BASIC_WIDE)).into(),
        };
        tagged(id, item).into()
    };
    let details = item(
        "basic.expander.details",
        probe(
            probes::BASIC_EXPANDER,
            Fill,
            header(
                "basic.expander.details",
                "Details",
                state.basic_details_open,
                Message::BasicDetailsToggled,
            ),
        ),
        state.basic_details_open.then_some("Expanded content"),
    );
    let more = item(
        "basic.expander.more",
        header(
            "basic.expander.more",
            "More",
            state.basic_more_open,
            Message::BasicMoreToggled,
        ),
        state.basic_more_open.then_some("More content"),
    );
    let expanders = group(
        "basic.expander.heading",
        "Expander",
        column![details, more].spacing(gap.widget).into(),
    );

    // The Layout page's card, a container in `styles::container_card`, not
    // `iced_aw`'s `Card`: a card with no title has no head, and iced_aw's
    // squares a card's lower corners under its head or body fill
    // (`widget/card.rs`, `draw_head`, `draw_body`), where the theme rounds all
    // four.
    let card_group = group(
        "basic.card.heading",
        "Card",
        // A side `card.border.padding` leaves unstated -- every side on the
        // Linux presets, whose cards leave the padding to their content
        // (platform-facts §2.26) -- is `layout.container_margin`, the
        // theme's padding inside a container.
        probe(
            probes::BASIC_CARD,
            Length::Shrink,
            tagged(
                "basic.card.frame",
                container(tagged(
                    "basic.card.text",
                    text("Card content").body(resolved, a11y),
                ))
                .padding(native_theme_iced::padding_or(
                    &resolved.card.border.padding,
                    Padding::from(gap.container),
                ))
                .width(Length::Fixed(BASIC_WIDE))
                .style(styles::container_card(resolved)),
            ),
        ),
    );

    let separator = group(
        "basic.separator.heading",
        "Separator",
        container(tagged(
            "basic.separator.line",
            rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ))
        .width(Length::Fixed(BASIC_WIDE))
        .into(),
    );

    // Three icon-only tool buttons, as the toolbar's: flat, their icons the
    // shown set's Copy, Paste and Delete at `toolbar.icon_size`, the
    // toolbar's item gap apart.
    let icons = &state.chrome_icons;
    let tool = |tags, icon, label| {
        icon_button(
            state,
            IconButton {
                icon,
                size: resolved.toolbar.icon_size,
                label,
                key: None,
                font: &resolved.toolbar.font,
                padding: ghost_padding(resolved),
                selected: false,
                action: Message::ButtonPressed,
                tags: Some(tags),
            },
        )
    };
    let icon_buttons = row![
        tool(
            ("basic.icons.copy", "basic.icons.copy.icon"),
            &icons.copy,
            "Copy"
        ),
        tool(
            ("basic.icons.paste", "basic.icons.paste.icon"),
            &icons.paste,
            "Paste"
        ),
        tool(
            ("basic.icons.delete", "basic.icons.delete.icon"),
            &icons.delete,
            "Delete"
        ),
    ]
    .spacing(resolved.toolbar.item_gap.unwrap_or(gap.widget))
    .align_y(iced::Center);

    // The text scale, a line per role, each in its own size, line height
    // and weight, in the body font's family, the link after the body text
    // (Basic v4); the monospace font last.
    let mono = &resolved.defaults.mono_font;
    let link = &resolved.link;
    let typography = group(
        "basic.typography.heading",
        "Typography",
        column![
            tagged(
                "basic.typography.caption",
                text("Caption").role(&ts.caption, resolved, a11y)
            ),
            tagged("basic.typography.body", text("Body").body(resolved, a11y)),
            // `link.underline_enabled`, which a text button cannot carry: the
            // label is a span, which iced underlines in its own colour, the
            // colour `styles::button_link` gives the button's text.
            tagged(
                "basic.typography.link",
                button(
                    rich_text([span::<(), _>("Link").underline(link.underline_enabled)])
                        .size(scaled_text_size(link.font.size, a11y))
                        .line_height(native_theme_iced::line_height_multiplier(resolved))
                        .font(theme_font(&link.font))
                )
                .on_press(Message::ButtonPressed)
                .style(styles::button_link(resolved))
                .padding(LINK_PADDING)
            ),
            tagged(
                "basic.typography.section_heading",
                text("Section heading").role(&ts.section_heading, resolved, a11y)
            ),
            tagged(
                "basic.typography.dialog_title",
                text("Dialog title").role(&ts.dialog_title, resolved, a11y)
            ),
            tagged(
                "basic.typography.display",
                text("Display").role(&ts.display, resolved, a11y)
            ),
            tagged(
                "basic.typography.monospace",
                text("Monospace")
                    .themed(mono, resolved, a11y)
                    .font(theme_mono_font(resolved))
            ),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let table = group("basic.table.heading", "Table", basic_table(state));

    // One icon of the shown set, `IconRole::FolderOpen`, at each size the
    // theme states for small, toolbar and large icons; none where the set
    // has no such icon.
    let folder = |id: &'static str, size: f32| -> Element<'a, Message> {
        icons
            .folder
            .as_ref()
            .and_then(|icon| {
                chrome_icon(
                    icon,
                    size,
                    icons.system,
                    to_color(resolved.defaults.text_color),
                )
            })
            .map_or_else(|| space().into(), |icon| tagged(id, icon).into())
    };
    let sizes = &resolved.defaults.icon_sizes;
    // The icon buttons over the icons at their sizes (Basic v4).
    let icon_group = group(
        "basic.icons.heading",
        "Icons",
        column![
            icon_buttons,
            row![
                folder("basic.icons.small", sizes.small),
                folder("basic.icons.toolbar", sizes.toolbar),
                folder("basic.icons.large", sizes.large),
            ]
            .spacing(gap.widget)
            .align_y(iced::Center),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let column_of = |id: &'static str, groups: Vec<Element<'a, Message>>| {
        tagged(id, column(groups).spacing(gap.section).width(Fill))
    };
    tagged(
        "basic.page",
        row![
            column_of(
                "basic.column_1",
                vec![buttons, checkboxes, radios, drop_down]
            ),
            column_of("basic.column_2", vec![inputs, text_area, slider_group]),
            column_of(
                "basic.column_3",
                vec![switches, number, spinner_group, segmented, card_group]
            ),
            column_of(
                "basic.column_4",
                vec![typography, separator, progress, list_group]
            ),
            column_of("basic.column_5", vec![icon_group, tabs, expanders, table]),
        ]
        .spacing(gap.section),
    )
    .into()
}

/// The Basic page's table (Basic v3), built from containers: iced_widget's
/// `table` has no style setter (iced_widget 0.14.2 `src/table.rs:149-196`),
/// so its separators, header and rows could not take `list.*`. A header row
/// `Name | Size` on `list.header_background` in `list.header_font`, two
/// rows in `list.item_font` -- the second selected, in
/// `list.selection_background` and `.selection_text_color` -- each
/// `list.row_height` tall, or its
/// line box and padding where unstated; the two columns half the width
/// inside the frame each (the model states no column width); the cells
/// padded by `list.border.padding`, none on a side it leaves unstated; a
/// `list.grid_color` line between the columns and under the header,
/// `separator.line_width` thick (the theme states no grid width), inside the
/// cell it closes; the frame `list.border` on `list.background_color`. The
/// table the gpui and egui showcases build.
fn basic_table(state: &State) -> Element<'_, Message> {
    let resolved = &state.current_resolved;
    let a11y = &state.accessibility;
    let l = &resolved.list;
    let line = resolved.separator.line_width;
    let pad = native_theme_iced::padding_or(&l.border.padding, Padding::ZERO);
    let height = |font: &ResolvedFontSpec| {
        l.row_height
            .unwrap_or(scaled_text_size(font.size, a11y) * resolved.defaults.line_height + pad.y())
    };
    let grid = to_color(l.grid_color);
    let grid_line = move |w: Length, h: Length| {
        container(space())
            .width(w)
            .height(h)
            .style(move |_: &Theme| container::Style {
                background: Some(iced::Background::Color(grid)),
                ..container::Style::default()
            })
    };
    // A cell: its text, vertically centred inside the padding, `h` tall;
    // the first column's cell with the column line inside its right edge,
    // a header cell with the header's line inside its bottom edge.
    let cell = move |content: &'static str,
                     font: &ResolvedFontSpec,
                     color: Color,
                     first: bool,
                     header: bool,
                     h: f32|
          -> Element<'_, Message> {
        let text_h = if header { (h - line).max(0.0) } else { h };
        let mut body = column![
            container(
                text(content)
                    .themed(font, resolved, a11y)
                    .color(color)
                    .wrapping(text::Wrapping::None),
            )
            .padding(pad)
            .width(Fill)
            .height(Length::Fixed(text_h))
            .align_y(iced::Center)
        ];
        if header {
            body = body.push(grid_line(Fill, Length::Fixed(line)));
        }
        let cell = row![body.width(Fill)];
        let cell = if first {
            cell.push(grid_line(Length::Fixed(line), Length::Fixed(h)))
        } else {
            cell
        };
        cell.width(Length::FillPortion(1)).into()
    };
    let head_h = height(&l.header_font);
    let head_ink = to_color(l.header_font.color);
    let head_fill = to_color(l.header_background);
    let header = tagged(
        "basic.table.header",
        container(row![
            cell("Name", &l.header_font, head_ink, true, true, head_h),
            cell("Size", &l.header_font, head_ink, false, true, head_h),
        ])
        .style(move |_: &Theme| container::Style {
            background: Some(iced::Background::Color(head_fill)),
            ..container::Style::default()
        }),
    )
    .node("basic.table.header.name", &[0, 0])
    .node("basic.table.header.size", &[0, 1]);
    let row_h = height(&l.item_font);
    let rows = BASIC_TABLE_ROWS
        .iter()
        .enumerate()
        .map(|(i, (id, name, size))| {
            let selected = i == BASIC_TABLE_SELECTED;
            let (fill, ink) = if selected {
                (
                    Some(to_color(l.selection_background)),
                    to_color(l.selection_text_color),
                )
            } else {
                (None, to_color(l.item_font.color))
            };
            tagged(
                id,
                container(row![
                    cell(name, &l.item_font, ink, true, false, row_h),
                    cell(size, &l.item_font, ink, false, false, row_h),
                ])
                .style(move |_: &Theme| container::Style {
                    background: fill.map(iced::Background::Color),
                    ..container::Style::default()
                }),
            )
            .into()
        });
    let frame = iced::Border {
        color: to_color(l.border.color),
        width: l.border.line_width,
        radius: l.border.corner_radius.into(),
    };
    let fill = to_color(l.background_color);
    tagged(
        "basic.table.frame",
        container(column![header].extend(rows))
            .padding(l.border.line_width)
            .width(Length::Fixed(BASIC_WIDE))
            .style(move |_: &Theme| container::Style {
                background: Some(iced::Background::Color(fill)),
                border: frame,
                ..container::Style::default()
            }),
    )
    .into()
}

/// `style` with no border where the theme states whether the expander is
/// framed (`frame`): a framed expander's frame holds its header and its body
/// together, and an unframed one has none (docs/platform-facts.md §2.27), so
/// the header draws no border of its own. Where the theme states nothing the
/// header keeps `styles::expander`'s border.
fn unframed_when_stated(
    style: impl Fn(&Theme, button::Status) -> button::Style + Clone,
    frame: Option<bool>,
) -> impl Fn(&Theme, button::Status) -> button::Style + Clone {
    move |theme, status| {
        let s = style(theme, status);
        match frame {
            Some(_) => button::Style {
                border: iced::Border {
                    width: 0.0,
                    ..s.border
                },
                ..s
            },
            None => s,
        }
    }
}

/// `style` with a transparent glyph where the theme states the check mark's
/// line (`stroke`): the showcase paints that mark itself ([`CheckMark`]).
/// Where the theme states none, iced's own glyph stands.
fn glyphless_when_stated(
    style: impl Fn(&Theme, checkbox::Status) -> checkbox::Style + Clone,
    stroke: Option<f32>,
) -> impl Fn(&Theme, checkbox::Status) -> checkbox::Style + Clone {
    move |theme, status| {
        let s = style(theme, status);
        match stroke {
            Some(_) => checkbox::Style {
                icon_color: Color::TRANSPARENT,
                ..s
            },
            None => s,
        }
    }
}

/// egui's check box and the box its tick fills: `Spacing::icon_width` 14 and
/// `icon_width_inner` 8 (egui 0.36.2 `src/style.rs:1466-1467`), the tick's box
/// centred in the check box (`:478`). [`CheckMark`] draws egui's tick, so where
/// the theme states no padding round the mark it takes egui's share of the box.
const EGUI_ICON_WIDTH: f32 = 14.0;
const EGUI_ICON_WIDTH_INNER: f32 = 8.0;

/// A checked box's mark where the theme states `checkbox.check_mark_stroke_width`:
/// iced draws its check as a glyph of its icon font (`checkbox.rs`, `Icon`), whose
/// line no style reaches, so the showcase paints the mark over the box at the
/// stated width, in the colour the style gives the glyph. The model states the
/// line, not the shape: the tick is egui's `Checkbox`'s (egui 0.36.2
/// `src/widgets/checkbox.rs:151-158`), from the middle of the mark box's left
/// edge through the middle of its bottom edge to its top-right corner, the mark
/// box being the indicator inside `inset` on each side: its border and the
/// stated padding, or egui's share of the box where the theme states none.
/// `top` over an opaque `bottom`, blended in sRGB as the connector blends a
/// translucent layer (`styles::composite_over`) and as iced's quads show one:
/// an opaque colour.
fn over(top: Color, bottom: Color) -> Color {
    let mix = |t: f32, b: f32| t * top.a + b * (1.0 - top.a);
    Color::from_rgb(
        mix(top.r, bottom.r),
        mix(top.g, bottom.g),
        mix(top.b, bottom.b),
    )
}

struct CheckMark {
    checked: bool,
    color: Color,
    stroke: f32,
    /// Horizontal and vertical inset of the mark box.
    inset: (f32, f32),
}

impl<Message> canvas::Program<Message> for CheckMark {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        if self.checked {
            let (x, y) = self.inset;
            let (left, right) = (x, frame.width() - x);
            let (top, bottom) = (y, frame.height() - y);
            let tick = canvas::Path::new(|path| {
                path.move_to(iced::Point::new(left, (top + bottom) / 2.0));
                path.line_to(iced::Point::new((left + right) / 2.0, bottom));
                path.line_to(iced::Point::new(right, top));
            });
            frame.stroke(
                &tick,
                canvas::Stroke::default()
                    .with_color(self.color)
                    .with_width(self.stroke),
            );
        }
        vec![frame.into_geometry()]
    }
}

/// The disclosure arrow of the Basic tab's expanders, a filled triangle as
/// wide and as tall as `expander.arrow_icon_size`, in
/// `native_theme_iced::expander_arrow_color`: before the title it points at
/// the title while collapsed and down while expanded, and after it
/// (`expander.arrow_side` trailing) down while collapsed and up while
/// expanded, as libadwaita's and WinUI's trailing chevrons turn
/// (docs/platform-facts.md §2.27). The model states the arrow's size, colour
/// and side, and no shape; iced has no expander, so the arrow is drawn here.
struct DisclosureArrow {
    expanded: bool,
    trailing: bool,
    color: Color,
}

impl<Message> canvas::Program<Message> for DisclosureArrow {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let (w, h) = (frame.width(), frame.height());
        let arrow = canvas::Path::new(|path| match (self.trailing, self.expanded) {
            // up
            (true, true) => {
                path.move_to(iced::Point::new(0.0, h));
                path.line_to(iced::Point::new(w, h));
                path.line_to(iced::Point::new(w / 2.0, 0.0));
                path.close();
            }
            // down
            (true, false) | (false, true) => {
                path.move_to(iced::Point::ORIGIN);
                path.line_to(iced::Point::new(w, 0.0));
                path.line_to(iced::Point::new(w / 2.0, h));
                path.close();
            }
            // right
            (false, false) => {
                path.move_to(iced::Point::ORIGIN);
                path.line_to(iced::Point::new(w, h / 2.0));
                path.line_to(iced::Point::new(0.0, h));
                path.close();
            }
        });
        frame.fill(&arrow, self.color);
        vec![frame.into_geometry()]
    }
}

/// The Widget Info of the Basic page's switches.
fn switch_info(resolved: &ResolvedTheme) -> String {
    let sw = &resolved.switch;
    widget_tooltip(
        "Switch (native_theme_iced::switch)",
        &[
            (
                "on track",
                "switch.checked_background",
                to_color(sw.checked_background),
            ),
            (
                "off track",
                "switch.unchecked_background",
                to_color(sw.unchecked_background),
            ),
            (
                "thumb",
                "switch.thumb_background",
                to_color(sw.thumb_background),
            ),
        ],
        &[
            ("track", "switch.track_width x switch.track_height"),
            (
                "thumb",
                "switch.thumb_diameter, inset by half the difference of the heights",
            ),
            ("track radius", "switch.track_radius"),
            (
                "label",
                format!(
                    "{} -- the model states no font for a switch",
                    font_row("defaults.font", &resolved.defaults.font)
                )
                .as_str(),
            ),
        ],
        &[
            (
                "widget",
                "iced's toggler lays its track out 2 x its height (toggler.rs:287): \
                 a button and two containers instead, in styles::toggler's colours",
            ),
            ("label gap", "the model states none: iced's toggler's"),
        ],
    )
}

// ---------------------------------------------------------------------------
// Tab: Buttons
// ---------------------------------------------------------------------------

/// The Widget Info of a button, the Buttons page's primary row and the
/// Basic page's buttons. `sized` is whether the buttons take the theme's
/// minimum size (`at_least`), as the Basic page's do.
fn button_info(state: &State, sized: bool) -> String {
    let resolved = &state.current_resolved;
    let ext = state.current_theme.extended_palette();
    let radius_s = format!("{:.0}px", resolved.button.border.corner_radius);
    let label = font_row("button.font", &resolved.button.font);
    let mut config = vec![
        ("border-radius", radius_s.as_str()),
        (
            "padding",
            "button_padding — each side button.border.padding states, plus \
             button.border.line_width (the theme's padding lies inside the border, \
             which iced paints over its padding); iced's button::DEFAULT_PADDING \
             for the others",
        ),
        ("shadow", "iced's own — the model has no shadow geometry"),
        ("label", label.as_str()),
    ];
    let mut not_themeable = Vec::new();
    if sized {
        config.push((
            "minimum size",
            "button.min_width × button.min_height, outer: the label in \
             at_least(.., button_content_min_size(..)), the minimum less \
             button_padding",
        ));
    } else {
        not_themeable.push(("min-height", "hardcoded by iced"));
    }
    widget_tooltip_themed(
        state,
        "Button (Primary)",
        &[
            (
                "bg",
                "button.primary_background",
                to_color(resolved.button.primary_background),
            ),
            (
                "text",
                "button.primary_text_color",
                to_color(resolved.button.primary_text_color),
            ),
            (
                "hover bg",
                "iced's primary.strong",
                ext.primary.strong.color,
            ),
        ],
        &config,
        &not_themeable,
    )
}

fn view_buttons<'a>(state: &'a State, btn_pad: Padding) -> Element<'a, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;

    let apply_pad =
        |b: button::Button<'a, Message>| -> button::Button<'a, Message> { b.padding(btn_pad) };

    let header = section_header(
        "Buttons",
        "Interactive button styles from the resolved theme",
        resolved,
        ts,
        a11y,
        sp,
    );

    let primary_row = hoverable(
        button_info(state, false),
        column![
            text("Primary Actions").role(section_title(ts), resolved, a11y),
            row![
                apply_pad(
                    button(text("Primary").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_primary(resolved))
                ),
                apply_pad(
                    button(text("Secondary").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button(resolved))
                ),
                apply_pad(
                    button(text("Success").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_success(resolved))
                ),
                apply_pad(
                    button(text("Warning").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_warning(resolved))
                ),
                apply_pad(
                    button(text("Danger").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_danger(resolved))
                ),
                apply_pad(
                    button(text("Text Style").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_link(resolved))
                ),
            ]
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let disabled_row = hoverable(
        widget_tooltip(
            "Disabled Buttons",
            &[
                (
                    "bg",
                    "button.disabled_background",
                    to_color(
                        resolved
                            .button
                            .disabled_background
                            .unwrap_or(resolved.button.background_color),
                    ),
                ),
                (
                    "text",
                    "button.disabled_text_color",
                    to_color(resolved.button.disabled_text_color),
                ),
            ],
            &[],
            &[
                ("cursor", "not interactive"),
                ("theme", "one disabled pair for every button class"),
            ],
        ),
        column![
            text("Disabled State").role(section_title(ts), resolved, a11y),
            text("Buttons without on_press are rendered as disabled:").body(resolved, a11y),
            row![
                apply_pad(
                    button(text("Disabled Primary").typeset(&resolved.button.font, a11y))
                        .style(styles::button_primary(resolved))
                ),
                apply_pad(
                    button(text("Disabled Secondary").typeset(&resolved.button.font, a11y))
                        .style(styles::button(resolved))
                ),
                apply_pad(
                    button(text("Disabled Danger").typeset(&resolved.button.font, a11y))
                        .style(styles::button_danger(resolved))
                ),
            ]
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let counter_text = format!("Button presses this session: {}", state.button_press_count);

    let interactive = column![
        text("Interactive Demo").role(section_title(ts), resolved, a11y),
        row![
            apply_pad(
                button(text("Click me!").typeset(&resolved.button.font, a11y))
                    .on_press(Message::ButtonPressed)
                    .style(styles::button_primary(resolved))
            ),
            text(counter_text).body(resolved, a11y),
        ]
        .spacing(sp.m)
        .align_y(iced::Center),
    ]
    .spacing(gap.widget);

    column![
        header,
        primary_row,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        disabled_row,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        interactive,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Text Inputs
// ---------------------------------------------------------------------------

/// The Widget Info of a single-line text field, the Text Inputs page's and
/// the Basic page's. `sized` is whether the field takes `input.min_height`
/// (`control_line_height`), as the Basic page's do.
fn text_input_info(state: &State, sized: bool) -> String {
    let resolved = &state.current_resolved;
    let i = &resolved.input;
    let radius_s = format!("{:.0}px", i.border.corner_radius);
    let text = font_row("input.font", &resolved.input.font);
    let mut config = vec![
        ("border-radius", radius_s.as_str()),
        (
            "padding",
            "input_padding — each side input.border.padding states, plus \
             input.border.line_width (the theme's padding lies inside the border, \
             which iced paints over its padding); iced's \
             text_input::DEFAULT_PADDING for the others",
        ),
        ("text", text.as_str()),
    ];
    let mut not_themeable = Vec::new();
    if sized {
        config.push((
            "height",
            "input.min_height: control_line_height gives the text the line box \
             that fills it inside input_padding",
        ));
    } else {
        not_themeable.push((
            "height",
            "iced's: a line of the text size, plus the padding",
        ));
    }
    not_themeable.push(("icon color", "no native source — iced's own"));
    widget_tooltip_themed(
        state,
        "TextInput",
        &[
            ("border", "input.border.color", to_color(i.border.color)),
            ("bg", "input.background_color", to_color(i.background_color)),
            ("text", "input.font.color", to_color(i.font.color)),
            (
                "placeholder",
                "input.placeholder_color",
                to_color(i.placeholder_color),
            ),
            (
                "selection",
                "input.selection_background",
                to_color(i.selection_background),
            ),
        ],
        &config,
        &not_themeable,
    )
}

fn view_text_inputs<'a>(state: &'a State, inp_pad: Padding) -> Element<'a, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let i = &resolved.input;
    let radius = i.border.corner_radius;
    let radius_s = format!("{radius:.0}px");

    let header = section_header(
        "Text Inputs",
        "Single-line TextInput and multi-line TextEditor",
        resolved,
        ts,
        a11y,
        sp,
    );

    let single_line = {
        let mut input = text_input("Type something here...", &state.text_input_value)
            .id(TEXT_INPUT_ID)
            .on_input(Message::TextInputChanged)
            .size(scaled_text_size(resolved.input.font.size, a11y))
            .font(theme_font(&resolved.input.font))
            .style(styles::text_input(resolved));
        {
            input = input.padding(inp_pad);
        }

        hoverable(
            text_input_info(state, false),
            column![
                text("TextInput (single line)").role(section_title(ts), resolved, a11y),
                input,
                text(format!(
                    "Characters: {}  |  input.border.corner_radius: {radius:.0}px",
                    state.text_input_value.len()
                ))
                .role(&ts.caption, resolved, a11y),
            ]
            .spacing(gap.widget)
            .into(),
        )
    };

    let secure_input = {
        let mut input = text_input("Password field...", &state.text_input_value)
            .on_input(Message::TextInputChanged)
            .secure(true)
            .size(scaled_text_size(resolved.input.font.size, a11y))
            .font(theme_font(&resolved.input.font))
            .style(styles::text_input(resolved));
        {
            input = input.padding(inp_pad);
        }

        hoverable(
            widget_tooltip(
                "TextInput (secure)",
                &[
                    ("border", "input.border.color", to_color(i.border.color)),
                    ("bg", "input.background_color", to_color(i.background_color)),
                ],
                &[
                    ("border-radius", &radius_s),
                    (
                        "text",
                        font_row("input.font", &resolved.input.font).as_str(),
                    ),
                ],
                &[("mode", "password / secure — dots replace chars")],
            ),
            column![
                text("TextInput (secure / password)").role(section_title(ts), resolved, a11y),
                input,
            ]
            .spacing(gap.widget)
            .into(),
        )
    };

    let multi_line = hoverable(
        widget_tooltip_themed(
            state,
            "TextEditor (multi-line)",
            &[
                ("bg", "input.background_color", to_color(i.background_color)),
                ("text", "input.font.color", to_color(i.font.color)),
                (
                    "selection",
                    "input.selection_background",
                    to_color(i.selection_background),
                ),
            ],
            &[
                ("border-radius", &radius_s),
                (
                    "text",
                    font_row("input.font", &resolved.input.font).as_str(),
                ),
            ],
            &[
                ("line numbers", "not built-in"),
                ("syntax highlighting", "requires iced_highlighter"),
            ],
        ),
        column![
            text("TextEditor (multi-line)").role(section_title(ts), resolved, a11y),
            probe(
                probes::TEXT_EDITOR,
                Fill,
                text_editor(&state.text_editor_content)
                    .on_action(Message::EditorAction)
                    .size(scaled_text_size(resolved.input.font.size, a11y))
                    .font(theme_font(&resolved.input.font))
                    .style(styles::text_editor(resolved))
                    .padding(native_theme_iced::text_area_padding(resolved))
                    .height(Length::Fixed(180.0)),
            ),
            text("Supports multi-line editing, selection, and scrolling").role(
                &ts.caption,
                resolved,
                a11y
            ),
        ]
        .spacing(gap.widget)
        .into(),
    );

    column![
        header,
        single_line,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        secure_input,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        multi_line,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Selection
// ---------------------------------------------------------------------------

/// The Widget Info of a checkbox, the Selection page's and the Basic page's.
fn checkbox_info(resolved: &ResolvedTheme) -> String {
    let c = &resolved.checkbox;
    let checkbox_radius_s = format!("{:.0}px", c.border.corner_radius);
    let label_gap_s = format!("{:.0}px", c.label_gap);
    // `checkbox.indicator_width` is the indicator's side length, square for a
    // checkbox and a diameter for a radio (platform-facts.md:980), and both
    // `Checkbox::size` (checkbox.rs:176, laid out at :287) and `Radio::size`
    // (radio.rs:200, :300) take exactly that.
    let indicator_width_s = format!("{:.0}px", c.indicator_width);
    widget_tooltip(
        "Checkbox",
        &[
            (
                "checked bg",
                "checkbox.checked_background",
                to_color(c.checked_background),
            ),
            (
                "checkmark",
                "checkbox.indicator_color",
                to_color(c.indicator_color),
            ),
            (
                "unchecked border",
                "checkbox.unchecked_border_color",
                to_color(c.unchecked_border_color.unwrap_or(c.border.color)),
            ),
            (
                "bg",
                "checkbox.unchecked_background",
                to_color(c.unchecked_background.unwrap_or(c.background_color)),
            ),
        ],
        &[
            ("border-radius", &checkbox_radius_s),
            ("label gap", &label_gap_s),
            ("box size", &indicator_width_s),
            (
                "label",
                font_row("checkbox.font", &resolved.checkbox.font).as_str(),
            ),
        ],
        &[(
            "check mark",
            "Checkbox::icon takes an Icon — a font glyph — so the mark's own \
             shape is the font's, not the theme's (checkbox.rs:229, :493-504)",
        )],
    )
}

/// The Widget Info of a radio button, the Selection page's and the Basic
/// page's.
fn radio_info(resolved: &ResolvedTheme) -> String {
    let c = &resolved.checkbox;
    let label_gap_s = format!("{:.0}px", c.label_gap);
    // `checkbox.radio_indicator_width` where the theme sizes the radio apart
    // (platform-facts.md:1222), the checkbox's `indicator_width` where not.
    let indicator_width_s = format!(
        "{:.0}px",
        c.radio_indicator_width.unwrap_or(c.indicator_width)
    );
    // `checkbox.radio_dot_diameter` (platform-facts.md:1220), which
    // `native_theme_iced::radio` draws over iced's radio; unstated, the dot is
    // iced's own, half the circle (radio.rs:409).
    let dot_s = c.radio_dot_diameter.map(|d| format!("{d:.0}px"));
    let mut sizes = vec![
        ("label gap", label_gap_s.as_str()),
        ("indicator diameter", indicator_width_s.as_str()),
    ];
    let mut unthemed = vec![
        ("border-radius", "radio::Style carries no corner radius"),
        ("disabled", "radio::Status has no disabled value"),
    ];
    match &dot_s {
        Some(dot) => sizes.push(("dot diameter", dot.as_str())),
        None => unthemed.push((
            "dot diameter",
            "the theme states none: iced's own, half the circle (radio.rs:409)",
        )),
    }
    let label_s = font_row("checkbox.font", &resolved.checkbox.font);
    sizes.push(("label", label_s.as_str()));
    widget_tooltip(
        "Radio",
        &[
            (
                "selected",
                "checkbox.checked_background",
                to_color(c.checked_background),
            ),
            (
                "dot",
                "checkbox.indicator_color",
                to_color(c.indicator_color),
            ),
            (
                "unselected border",
                "checkbox.unchecked_border_color",
                to_color(c.unchecked_border_color.unwrap_or(c.border.color)),
            ),
            (
                "bg",
                "checkbox.unchecked_background",
                to_color(c.unchecked_background.unwrap_or(c.background_color)),
            ),
        ],
        &sizes,
        &unthemed,
    )
}

/// The Widget Info of a pick list, the Selection page's and the Basic page's
/// drop-down. `sized` is whether it takes `combo_box_padding` and
/// `combo_box.min_height` (`control_line_height`), as the Basic page's does.
fn pick_list_info(resolved: &ResolvedTheme, sized: bool) -> String {
    let cb = &resolved.combo_box;
    let combo_radius_s = format!("{:.0}px", cb.border.corner_radius);
    let arrow_size_s = format!("{:.0}px", cb.arrow_icon_size);
    let rows = font_row("combo_box.font", &resolved.combo_box.font);
    let mut config = vec![
        ("border-radius", combo_radius_s.as_str()),
        ("arrow size", arrow_size_s.as_str()),
        ("label and menu rows", rows.as_str()),
    ];
    if sized {
        config.push((
            "padding",
            "combo_box_padding — each side combo_box.border.padding states, plus \
             combo_box.border.line_width (the theme's padding lies inside the \
             border, which iced paints over its padding); iced's pick-list \
             padding for the others",
        ));
        config.push((
            "height",
            "combo_box.min_height: control_line_height gives the label the line \
             box that fills it inside combo_box_padding",
        ));
    }
    widget_tooltip(
        "PickList (dropdown)",
        &[
            (
                "bg",
                "combo_box.background_color",
                to_color(cb.background_color),
            ),
            ("text", "combo_box.font.color", to_color(cb.font.color)),
            (
                "border",
                "combo_box.border.color",
                to_color(cb.border.color),
            ),
            (
                "menu bg",
                "menu.background_color",
                to_color(resolved.menu.background_color),
            ),
            (
                "menu selected",
                "menu.hover_background",
                to_color(resolved.menu.hover_background),
            ),
        ],
        &config,
        &[
            (
                "dropdown arrow",
                "an open chevron, the platforms' glyph, drawn as iced's own \
                 Iced-Icons U+E803 (native_theme_iced::pick_list_handle); \
                 its stroke is the font's, the theme states none",
            ),
            ("arrow color", "ComboBoxTheme carries no arrow color"),
            ("arrow area width", "no receiver in iced"),
        ],
    )
}

fn view_selection(state: &State) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let c = &resolved.checkbox;
    let sw = &resolved.switch;
    let cb = &resolved.combo_box;
    let track_radius_s = format!("{:.0}px", sw.track_radius);
    let track_height_s = format!("{:.0}px", sw.track_height);
    let thumb_diameter_s = format!("{:.0}px", sw.thumb_diameter);
    let input_radius_s = format!("{:.0}px", resolved.input.border.corner_radius);

    let header = section_header(
        "Selection Widgets",
        "Checkbox, Radio, Toggler, PickList, and ComboBox",
        resolved,
        ts,
        a11y,
        sp,
    );

    let checkboxes = hoverable(
        checkbox_info(resolved),
        column![
            text("Checkboxes").role(section_title(ts), resolved, a11y),
            checkbox(state.checkbox_a)
                .label("Enable notifications")
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .text_size(scaled_text_size(c.font.size, a11y))
                .font(theme_font(&c.font))
                .style(styles::checkbox(resolved))
                .on_toggle(Message::CheckboxAToggled),
            checkbox(state.checkbox_b)
                .label("Dark mode auto-detect")
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .text_size(scaled_text_size(c.font.size, a11y))
                .font(theme_font(&c.font))
                .style(styles::checkbox(resolved))
                .on_toggle(Message::CheckboxBToggled),
            checkbox(state.checkbox_c)
                .label("Remember preferences")
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .text_size(scaled_text_size(c.font.size, a11y))
                .font(theme_font(&c.font))
                .style(styles::checkbox(resolved))
                .on_toggle(Message::CheckboxCToggled),
            text(format!(
                "Checked: {}",
                [
                    state.checkbox_a.then_some("notifications"),
                    state.checkbox_b.then_some("auto-detect"),
                    state.checkbox_c.then_some("remember"),
                ]
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>()
                .join(", ")
            ))
            .role(&ts.caption, resolved, a11y),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let radios = hoverable(
        radio_info(resolved),
        column![
            text("Radio Buttons").role(section_title(ts), resolved, a11y),
            probe(
                probes::RADIO_APPLE,
                Length::Shrink,
                native_theme_iced::radio(
                    resolved,
                    radio(
                        "Apple",
                        Fruit::Apple,
                        state.selected_fruit,
                        Message::FruitSelected
                    )
                    .text_size(scaled_text_size(c.font.size, a11y))
                    .font(theme_font(&c.font)),
                    state.selected_fruit == Some(Fruit::Apple),
                )
            ),
            probe(
                probes::RADIO_BANANA,
                Length::Shrink,
                native_theme_iced::radio(
                    resolved,
                    radio(
                        "Banana",
                        Fruit::Banana,
                        state.selected_fruit,
                        Message::FruitSelected
                    )
                    .text_size(scaled_text_size(c.font.size, a11y))
                    .font(theme_font(&c.font)),
                    state.selected_fruit == Some(Fruit::Banana),
                )
            ),
            probe(
                probes::RADIO_CHERRY,
                Length::Shrink,
                native_theme_iced::radio(
                    resolved,
                    radio(
                        "Cherry",
                        Fruit::Cherry,
                        state.selected_fruit,
                        Message::FruitSelected
                    )
                    .text_size(scaled_text_size(c.font.size, a11y))
                    .font(theme_font(&c.font)),
                    state.selected_fruit == Some(Fruit::Cherry),
                )
            ),
            text(format!(
                "Selected: {}",
                state
                    .selected_fruit
                    .map(|f| f.to_string())
                    .unwrap_or_else(|| "None".to_string())
            ))
            .role(&ts.caption, resolved, a11y),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // The page's switch is the connector's, whose track takes
    // `switch.track_width`, labelled as the Basic page's are. iced's own
    // toggler stays shown below it, so the widget iced offers is on show
    // under the theme too, its track twice its height.
    let switch = hoverable(
        switch_info(resolved),
        row![
            probe(
                probes::TOGGLER,
                Length::Shrink,
                native_theme_iced::switch(
                    resolved,
                    state.toggler_enabled,
                    Some(Message::TogglerToggled(!state.toggler_enabled)),
                ),
            ),
            text("Feature flag enabled").body(resolved, a11y),
        ]
        .spacing(TOGGLER_LABEL_GAP)
        .align_y(iced::Alignment::Center)
        .into(),
    );
    let iced_toggler = hoverable(
        widget_tooltip(
            "iced's Toggler",
            &[
                (
                    "active track",
                    "switch.checked_background",
                    to_color(sw.checked_background),
                ),
                (
                    "inactive track",
                    "switch.unchecked_background",
                    to_color(sw.unchecked_background),
                ),
                (
                    "thumb",
                    "switch.thumb_background",
                    to_color(sw.thumb_background),
                ),
            ],
            &[
                ("border-radius", &track_radius_s),
                ("track height", &track_height_s),
                ("thumb diameter", &thumb_diameter_s),
                (
                    "label",
                    format!(
                        "{} — the model states no font for a switch",
                        font_row("defaults.font", &resolved.defaults.font)
                    )
                    .as_str(),
                ),
            ],
            &[
                ("track width", "iced lays the track out as 2 x its height"),
                ("border", "SwitchTheme carries none — iced's own"),
                ("animation timing", "hardcoded"),
            ],
        ),
        toggler(state.toggler_enabled)
            .label("iced's Toggler")
            .size(sw.track_height)
            .text_size(scaled_text_size(resolved.defaults.font.size, a11y))
            .font(theme_font(&resolved.defaults.font))
            .style(styles::toggler(resolved))
            .on_toggle(Message::TogglerToggled)
            .into(),
    );
    let togglers: Element<'_, Message> = column![
        text("Switch").role(section_title(ts), resolved, a11y),
        switch,
        text(format!(
            "State: {}",
            if state.toggler_enabled { "ON" } else { "OFF" }
        ))
        .role(&ts.caption, resolved, a11y),
        iced_toggler,
    ]
    .spacing(gap.widget)
    .into();

    let languages: Vec<String> = vec![
        "Rust",
        "Python",
        "JavaScript",
        "TypeScript",
        "Go",
        "C++",
        "Java",
        "Swift",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let pickers = hoverable(
        pick_list_info(resolved, false),
        column![
            text("PickList (dropdown)").role(section_title(ts), resolved, a11y),
            probe(
                probes::PICK_LIST,
                Length::Shrink,
                pick_list(
                    languages,
                    state.pick_list_selected.as_ref(),
                    Message::PickListSelected,
                )
                .handle(arrow_handle(resolved))
                .text_size(scaled_text_size(resolved.combo_box.font.size, a11y))
                .font(theme_font(&resolved.combo_box.font))
                .style(styles::pick_list(resolved))
                .menu_style(styles::menu(resolved))
                .width(Length::Fixed(250.0))
            ),
            text(format!(
                "Selected: {}",
                state.pick_list_selected.as_deref().unwrap_or("None")
            ))
            .role(&ts.caption, resolved, a11y),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let combos = hoverable(
        widget_tooltip(
            "ComboBox (searchable dropdown)",
            &[
                (
                    "bg",
                    "input.background_color",
                    to_color(resolved.input.background_color),
                ),
                (
                    "text",
                    "input.font.color",
                    to_color(resolved.input.font.color),
                ),
                (
                    "border",
                    "input.border.color",
                    to_color(resolved.input.border.color),
                ),
                (
                    "menu bg",
                    "menu.background_color",
                    to_color(resolved.menu.background_color),
                ),
            ],
            &[
                ("border-radius", &input_radius_s),
                (
                    "text and menu rows",
                    font_row("combo_box.font", &resolved.combo_box.font).as_str(),
                ),
            ],
            &[
                ("search", "built-in text filter"),
                ("field style", "a ComboBox takes a text_input style"),
            ],
        ),
        column![
            text("ComboBox (searchable dropdown)").role(section_title(ts), resolved, a11y),
            probe(
                probes::COMBO_BOX,
                Length::Shrink,
                combo_box(
                    &state.combo_state,
                    "Search a language...",
                    state.combo_selected.as_ref(),
                    Message::ComboBoxSelected,
                )
                .size(scaled_text_size(cb.font.size, a11y))
                .font(theme_font(&cb.font))
                .input_style(styles::text_input(resolved))
                .menu_style(styles::menu(resolved))
                .width(Length::Fixed(250.0))
            ),
            text(format!(
                "Selected: {}",
                state.combo_selected.as_deref().unwrap_or("None")
            ))
            .role(&ts.caption, resolved, a11y),
        ]
        .spacing(gap.widget)
        .into(),
    );

    column![
        header,
        row![
            column![
                checkboxes,
                rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                togglers,
            ]
            .spacing(gap.section)
            .width(Fill),
            rule::vertical(resolved.separator.line_width).style(styles::rule(resolved)),
            column![
                radios,
                rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                pickers,
                rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                combos,
            ]
            .spacing(gap.section)
            .width(Fill),
        ]
        .spacing(gap.section),
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Range
// ---------------------------------------------------------------------------

/// The Widget Info of a horizontal slider, the Range page's and the Basic
/// page's.
fn slider_info(resolved: &ResolvedTheme) -> String {
    let sl = &resolved.slider;
    let rail_s = format!("{:.0}px", sl.track_height);
    let thumb_s = format!("{:.0}px", sl.thumb_diameter);
    widget_tooltip(
        "Horizontal Slider",
        &[
            ("active track", "slider.fill_color", to_color(sl.fill_color)),
            (
                "inactive track",
                "slider.track_color",
                to_color(sl.track_color),
            ),
            ("handle", "slider.thumb_color", to_color(sl.thumb_color)),
            (
                "hovered handle",
                "slider.thumb_hover_color",
                to_color(sl.thumb_hover_color.unwrap_or(sl.thumb_color)),
            ),
        ],
        &[("rail width", &rail_s), ("thumb diameter", &thumb_s)],
        &[
            ("widget height", "no native source — iced's own"),
            ("dragged handle fill", "no native source — iced's own"),
        ],
    )
}

/// The Widget Info of a progress bar, the Range page's and the Basic page's.
fn progress_bar_info(resolved: &ResolvedTheme) -> String {
    let pb = &resolved.progress_bar;
    let bar_girth_s = format!("{:.0}px", pb.track_height);
    widget_tooltip(
        "Progress Bar",
        &[
            ("fill", "progress_bar.fill_color", to_color(pb.fill_color)),
            (
                "track bg",
                "progress_bar.track_color",
                to_color(pb.track_color),
            ),
            (
                "border",
                "progress_bar.border.color",
                to_color(pb.border.color),
            ),
        ],
        &[("girth", &bar_girth_s)],
        &[
            ("min width", "progress_bar.min_width has no receiver"),
            ("animation", "none — immediate"),
        ],
    )
}

fn view_range(state: &State) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let sl = &resolved.slider;
    let pb = &resolved.progress_bar;
    let rail_s = format!("{:.0}px", sl.track_height);
    let thumb_s = format!("{:.0}px", sl.thumb_diameter);

    let header = section_header(
        "Range Widgets",
        "Slider, VerticalSlider, and ProgressBar",
        resolved,
        ts,
        a11y,
        sp,
    );

    let horiz_slider =
        hoverable(
            slider_info(resolved),
            column![
                text("Horizontal Slider").role(section_title(ts), resolved, a11y),
                row![
                    probe(
                        probes::SLIDER,
                        Fill,
                        slider(0.0..=100.0, state.slider_value, Message::SliderChanged)
                            .style(styles::slider(resolved))
                            .width(Fill)
                    ),
                    text(format!("{:.1}", state.slider_value))
                        .body(resolved, a11y)
                        .width(Length::Fixed(50.0)),
                ]
                .spacing(sp.m)
                .align_y(iced::Center),
                text("Drag to change value. This slider drives the first progress bar below.")
                    .role(&ts.caption, resolved, a11y),
            ]
            .spacing(gap.widget)
            .into(),
        );

    let step_slider = hoverable(
        widget_tooltip(
            "Slider (stepped)",
            &[("track", "slider.fill_color", to_color(sl.fill_color))],
            &[("step", "5.0")],
            &[("snap behavior", "hardcoded step increments")],
        ),
        column![
            text("Slider with Step (5-unit increments)").role(section_title(ts), resolved, a11y),
            row![
                slider(0.0..=100.0, state.slider_step, Message::StepSliderChanged)
                    .step(5.0_f32)
                    .style(styles::slider(resolved))
                    .width(Fill),
                text(format!("{:.0}", state.slider_step))
                    .body(resolved, a11y)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(sp.m)
            .align_y(iced::Center),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let vert_slider = hoverable(
        widget_tooltip(
            "Vertical Slider",
            &[("track", "slider.fill_color", to_color(sl.fill_color))],
            &[
                ("orientation", "vertical"),
                ("rail width", &rail_s),
                ("thumb diameter", &thumb_s),
            ],
            &[("widget width", "no native source — iced's own")],
        ),
        column![
            text("Vertical Slider").role(section_title(ts), resolved, a11y),
            row![
                probe(
                    probes::VERTICAL_SLIDER,
                    Length::Shrink,
                    container(
                        vertical_slider(0.0..=100.0, state.vslider_value, Message::VSliderChanged)
                            .style(styles::slider(resolved))
                            .height(Length::Fixed(200.0))
                    )
                    .center_x(Length::Fixed(60.0))
                ),
                column![
                    text(format!("Value: {:.1}", state.vslider_value)).body(resolved, a11y),
                    space().height(Length::Fixed(8.0)),
                    text("Vertical sliders are useful\nfor volume controls,\nequalizers, etc.")
                        .role(&ts.caption, resolved, a11y),
                ]
                .spacing(sp.xs),
            ]
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let progress = hoverable(
        progress_bar_info(resolved),
        column![
            text("Progress Bars").role(section_title(ts), resolved, a11y),
            text("Driven by horizontal slider value:").body(resolved, a11y),
            progress_bar(0.0..=100.0, state.slider_value)
                .girth(Length::Fixed(pb.track_height))
                .style(styles::progress_bar(resolved)),
            space().height(Length::Fixed(4.0)),
            text("Separate progress control:").body(resolved, a11y),
            row![
                slider(0.0..=100.0, state.progress_value, Message::ProgressChanged)
                    .style(styles::slider(resolved))
                    .width(Fill),
                text(format!("{:.0}%", state.progress_value))
                    .body(resolved, a11y)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(sp.m)
            .align_y(iced::Center),
            progress_bar(0.0..=100.0, state.progress_value)
                .girth(Length::Fixed(pb.track_height))
                .style(styles::progress_bar(resolved)),
        ]
        .spacing(gap.widget)
        .into(),
    );

    column![
        header,
        horiz_slider,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        step_slider,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        vert_slider,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        progress,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Display
// ---------------------------------------------------------------------------

fn view_display(state: &State) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let card = &resolved.card;
    let tip = &resolved.tooltip;
    let sep = &resolved.separator;
    let card_radius = card.border.corner_radius;
    // A container is unpadded unless given a padding (container.rs:95), so a
    // side the card's theme leaves unstated stays at zero.
    let card_pad = native_theme_iced::padding_or(&card.border.padding, Padding::ZERO);
    let card_radius_s = format!("{card_radius:.0}px");
    let tip_radius_s = format!("{:.0}px", tip.border.corner_radius);
    let line_width_s = format!("{:.0}px", sep.line_width);

    let header = section_header(
        "Display Widgets",
        "Container, Rule, Tooltip, and layout helpers",
        resolved,
        ts,
        a11y,
        sp,
    );

    let containers = hoverable(
        widget_tooltip(
            "Styled Containers (card)",
            &[
                (
                    "bg",
                    "card.background_color",
                    to_color(card.background_color),
                ),
                ("border", "card.border.color", to_color(card.border.color)),
            ],
            &[
                ("border-radius", &card_radius_s),
                (
                    "padding",
                    "card.border.padding's stated sides, a container's own \
                     Padding::ZERO for the others",
                ),
            ],
            &[(
                "text",
                "CardTheme carries no font, so the labels are set explicitly: the \
                 title in defaults.font, the notes in the caption role",
            )],
        ),
        column![
            text("Styled Containers").role(section_title(ts), resolved, a11y),
            container(
                column![
                    text("Container (card fill)").body(resolved, a11y),
                    text(format!(
                        "This container uses styles::container_card. \
                         card.border.corner_radius: {card_radius:.0}px."
                    ))
                    .role(&ts.caption, resolved, a11y),
                ]
                .spacing(sp.xs),
            )
            .padding(card_pad)
            .style(styles::container_card(resolved))
            .width(Fill),
            container(
                text(
                    "A second container dressed as a card. Containers take their \
                      background, border and padding from the resolved card theme."
                )
                .role(&ts.caption, resolved, a11y),
            )
            .padding(card_pad)
            .style(styles::container_card(resolved))
            .width(Fill),
        ]
        .spacing(sp.m)
        .into(),
    );

    let rules = column![
        text("Divider Rules").role(section_title(ts), resolved, a11y),
        text(format!(
            "iced takes a rule's thickness as the constructor's argument, and the \
             platform states exactly one: separator.line_width ({line_width_s}). \
             Three rules at that width would be three copies of the same line, so \
             here is the one:"
        ))
        .body(resolved, a11y),
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        text(
            "Its colour is separator.line_color; its radius and its fill mode have \
             no native source and are iced's own."
        )
        .role(&ts.caption, resolved, a11y),
    ]
    .spacing(gap.widget);

    let tooltips = hoverable(
        widget_tooltip(
            "Tooltip",
            &[
                (
                    "bg",
                    "tooltip.background_color",
                    to_color(tip.background_color),
                ),
                ("text", "tooltip.font.color", to_color(tip.font.color)),
                ("border", "tooltip.border.color", to_color(tip.border.color)),
            ],
            &[
                ("positions", "Top / Bottom / Left / Right"),
                ("border-radius", &tip_radius_s),
                (
                    "label",
                    font_row("tooltip.font", &resolved.tooltip.font).as_str(),
                ),
            ],
            &[
                ("gap", "set per widget instance"),
                ("delay", "none"),
                ("max width", "iced wraps the tip's own content element"),
            ],
        ),
        column![
            text("Tooltips").role(section_title(ts), resolved, a11y),
            row![
                tooltip(
                    button(text("Hover: Top").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_primary(resolved))
                        .padding(native_theme_iced::button_padding(resolved)),
                    text("Tooltip on top!").typeset(&resolved.tooltip.font, a11y),
                    tooltip::Position::Top,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button(text("Hover: Bottom").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button(resolved))
                        .padding(native_theme_iced::button_padding(resolved)),
                    text("Tooltip on bottom!").typeset(&resolved.tooltip.font, a11y),
                    tooltip::Position::Bottom,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button(text("Hover: Left").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_success(resolved))
                        .padding(native_theme_iced::button_padding(resolved)),
                    text("Tooltip on left!").typeset(&resolved.tooltip.font, a11y),
                    tooltip::Position::Left,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button(text("Hover: Right").typeset(&resolved.button.font, a11y))
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_danger(resolved))
                        .padding(native_theme_iced::button_padding(resolved)),
                    text("Tooltip on right!").typeset(&resolved.tooltip.font, a11y),
                    tooltip::Position::Right,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
            ]
            .spacing(sp.m),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let theme_info_text = format!(
        "Active theme: {}  |  Mode: {}",
        state.current_theme,
        if state.is_dark { "Dark" } else { "Light" },
    );

    // The sizes the showcase draws at: the theme's, times the OS text-scaling
    // factor, which every text here takes alike. Where the factor is not 1 the
    // theme's own size is shown beside it.
    let font_info = {
        let r = &state.current_resolved;
        let ff = family_label(
            native_theme_iced::font_family(r),
            resolved_family(&r.defaults.font),
            &font_drawn(&r.defaults.font),
        );
        let mf = family_label(
            native_theme_iced::mono_font_family(r),
            resolved_family(&r.defaults.mono_font),
            &mono_drawn(r),
        );
        let drawn = format!(
            "Font: {ff} @ {:.1}px  |  Mono: {mf} @ {:.1}px",
            native_theme_iced::font_size(r, a11y),
            native_theme_iced::mono_font_size(r, a11y)
        );
        if native_theme_iced::font_size(r, a11y) != r.defaults.font.size {
            format!(
                "{drawn}  |  OS text scaling {:.2}: the theme states {:.1}px and {:.1}px",
                a11y.text_scaling_factor, r.defaults.font.size, r.defaults.mono_font.size
            )
        } else {
            drawn
        }
    };

    let info_box = container(
        column![
            text("Theme Information").role(section_title(ts), resolved, a11y),
            text(theme_info_text).role(&ts.caption, resolved, a11y),
            text(font_info).role(&ts.caption, resolved, a11y),
            text(format!(
                "Available presets: {} | All presets have both light and dark variants.",
                native_theme::theme::Theme::list_presets().len(),
            ))
            .role(&ts.caption, resolved, a11y),
        ]
        .spacing(sp.xs),
    )
    .padding(Padding::from(gap.container))
    .style(styles::container_card(resolved))
    .width(Fill);

    let spacing_demo = column![
        text("Spacing & Layout").role(section_title(ts), resolved, a11y),
        row![
            container(text("A").body(resolved, a11y))
                .padding(Padding::from(sp.m))
                .style(styles::container_card(resolved))
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            container(text("B").body(resolved, a11y))
                .padding(Padding::from(sp.m))
                .style(styles::container_card(resolved))
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            container(text("C").body(resolved, a11y))
                .padding(Padding::from(sp.m))
                .style(styles::container_card(resolved))
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            space().width(Fill),
            container(text("Right-aligned").role(&ts.caption, resolved, a11y))
                .padding(Padding::from(sp.m))
                .style(styles::container_card(resolved)),
        ]
        .spacing(gap.widget)
        .align_y(iced::Center),
    ]
    .spacing(gap.widget);

    column![
        header,
        containers,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        rules,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        tooltips,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        spacing_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        info_box,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Layout
// ---------------------------------------------------------------------------

/// One row of the text-scale `table`: role, size, weight, line height.
///
/// `table::column`'s view function takes its row by value, so the row type is
/// `Clone` and carries the formatted strings rather than a borrow.
type ScaleRow = (&'static str, String, String, String);

/// Grid, PaneGrid and Table: the three `iced_widget` modules that arrange
/// other widgets rather than paint a control.
fn view_layout(state: &State) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let sep = &resolved.separator;
    let sp_th = &resolved.splitter;
    let line_width_s = format!("{:.0}px", sep.line_width);
    let divider_width_s = format!("{:.0}px", sp_th.divider_width);

    let header = section_header(
        "Layout Widgets",
        "Grid, PaneGrid and Table: the modules that arrange other widgets",
        resolved,
        ts,
        a11y,
        sp,
    );

    // ---- Grid: the platform's own colors, four to a row ----

    let swatch_border = native_theme_iced::border_color(resolved);
    let swatch_bw = resolved.defaults.border.line_width;
    let swatch_r = native_theme_iced::border_radius(resolved);
    let swatch_frame = iced::Border {
        color: swatch_border,
        width: swatch_bw,
        radius: swatch_r.into(),
    };
    let caption = &ts.caption;
    let xxs_sp = sp.xxs;
    let cell = |label: &'static str, color: Color| -> Element<'_, Message> {
        color_swatch(label, color, swatch_frame, caption, resolved, a11y, xxs_sp)
    };
    let d = &resolved.defaults;

    let grid_demo = hoverable(
        widget_tooltip(
            "Grid",
            &[
                (
                    "cell fill",
                    "each cell's own color",
                    to_color(d.accent_color),
                ),
                (
                    "cell border",
                    "defaults.border.color",
                    to_color(d.border.color),
                ),
            ],
            &[
                ("columns", "4"),
                ("cell radius", "defaults.border.corner_radius"),
            ],
            &[
                (
                    "Style",
                    "grid has no Catalog and no Style — it only places its children \
                     (grid.rs:13-19)",
                ),
                ("spacing", "the showcase's own scale"),
            ],
        ),
        column![
            text("Grid (cells in columns)").role(section_title(ts), resolved, a11y),
            text(
                "iced_widget::grid distributes its children over a fixed number of \
                 columns. Here: the eight colors ResolvedDefaults names."
            )
            .body(resolved, a11y),
            grid([
                cell("accent_color", to_color(d.accent_color)),
                cell("danger_color", to_color(d.danger_color)),
                cell("warning_color", to_color(d.warning_color)),
                cell("success_color", to_color(d.success_color)),
                cell("info_color", to_color(d.info_color)),
                cell("link_color", to_color(d.link_color)),
                cell("muted_color", to_color(d.muted_color)),
                cell("surface_color", to_color(d.surface_color)),
            ])
            .columns(4)
            .spacing(gap.widget)
            // `Grid::height` sets the height of the whole grid, not of a cell
            // (`grid.rs:78-84`, `Sizing::EvenlyDistribute`): a fixed number
            // here divides that number between the rows, and the swatch plus
            // its caption did not fit in the share. `Shrink` is the one
            // `Sizing` that lets each row be as tall as its content
            // (`grid.rs:207-211`).
            .height(Length::Shrink),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- PaneGrid: splits the platform's splitter theme paints ----

    let pane_style = {
        let hovered = to_color(sp_th.hover_color);
        let divider_width = sp_th.divider_width;
        move |theme: &Theme| {
            let iced = pane_grid::default(theme);
            pane_grid::Style {
                // The model states no drop-target highlight, so the region
                // iced paints under a dragged pane stays iced's own.
                hovered_region: iced.hovered_region,
                // A picked split is a split being dragged; SplitterTheme
                // states an idle and a hovered color and no dragged one, so
                // that one line is iced's (spec §3.2).
                picked_split: pane_grid::Line {
                    color: iced.picked_split.color,
                    width: divider_width,
                },
                hovered_split: pane_grid::Line {
                    color: hovered,
                    width: divider_width,
                },
            }
        }
    };

    let panes = pane_grid(&state.panes, |pane, label, _is_maximized| {
        let focused = state.focused_pane == Some(pane);
        let title = format!("Pane {label}");
        let controls = row![
            button(text("split |").typeset(&resolved.button.font, a11y))
                .on_press(Message::PaneSplit(pane_grid::Axis::Vertical, pane))
                .style(styles::button(resolved))
                .padding(native_theme_iced::button_padding(resolved)),
            button(text("split —").typeset(&resolved.button.font, a11y))
                .on_press(Message::PaneSplit(pane_grid::Axis::Horizontal, pane))
                .style(styles::button(resolved))
                .padding(native_theme_iced::button_padding(resolved)),
            button(text("close").typeset(&resolved.button.font, a11y))
                .on_press(Message::PaneClosed(pane))
                .style(styles::button_danger(resolved))
                .padding(native_theme_iced::button_padding(resolved)),
        ]
        .spacing(sp.xs);

        let title_bar = pane_grid::TitleBar::new(text(title).body(resolved, a11y))
            .controls(Element::from(controls))
            .always_show_controls()
            .padding(Padding::from([sp.xs, sp.s]))
            .style(styles::container_card(resolved));

        let body = column![
            text(if focused {
                "Focused. Drag the title bar onto another pane to move it."
            } else {
                "Drag the split between the panes to resize."
            })
            .role(&ts.caption, resolved, a11y),
        ]
        .spacing(sp.xs)
        .padding(Padding::from(sp.s));

        pane_grid::Content::new(body)
            .title_bar(title_bar)
            .style(styles::container_card(resolved))
    })
    .width(Fill)
    .height(Length::Fixed(220.0))
    .spacing(sp_th.divider_width)
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(sp_th.divider_width, Message::PaneResized)
    .style(pane_style);

    let pane_demo = hoverable(
        widget_tooltip(
            "PaneGrid",
            &[(
                "hovered split",
                "splitter.hover_color",
                to_color(sp_th.hover_color),
            )],
            &[
                ("split width", &divider_width_s),
                ("pane surface", "styles::container_card"),
                ("title bar", "styles::container_card"),
                (
                    "title",
                    font_row("defaults.font", &resolved.defaults.font).as_str(),
                ),
                (
                    "controls",
                    font_row("button.font", &resolved.button.font).as_str(),
                ),
            ],
            &[
                (
                    "drop region",
                    "the model states no drop-target highlight — iced's own",
                ),
                (
                    "picked split",
                    "SplitterTheme states no dragged color — iced's own, at the \
                     platform's width",
                ),
                (
                    "splitter.divider_color",
                    "no receiver: iced leaves an idle split unpainted",
                ),
            ],
        ),
        column![
            text("PaneGrid (split, drag and resize)").role(section_title(ts), resolved, a11y),
            text(
                "Split a pane, drag its title bar onto another one, or drag the \
                 divider between two panes."
            )
            .body(resolved, a11y),
            panes,
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- Table: the resolved text scale, as data ----

    let scale_rows: Vec<ScaleRow> = vec![
        (
            "caption",
            format!("{:.0}px", ts.caption.size),
            weight_label(ts.caption.weight, &role_drawn(&ts.caption, resolved)),
            format!("{:.1}px", ts.caption.line_height),
        ),
        (
            "section_heading",
            format!("{:.0}px", ts.section_heading.size),
            weight_label(
                ts.section_heading.weight,
                &role_drawn(&ts.section_heading, resolved),
            ),
            format!("{:.1}px", ts.section_heading.line_height),
        ),
        (
            "dialog_title",
            format!("{:.0}px", ts.dialog_title.size),
            weight_label(
                ts.dialog_title.weight,
                &role_drawn(&ts.dialog_title, resolved),
            ),
            format!("{:.1}px", ts.dialog_title.line_height),
        ),
        (
            "display",
            format!("{:.0}px", ts.display.size),
            weight_label(ts.display.weight, &role_drawn(&ts.display, resolved)),
            format!("{:.1}px", ts.display.line_height),
        ),
    ];

    let scale_table = table(
        [
            table::column(
                text("text_scale role").typeset(&resolved.list.header_font, a11y),
                {
                    let cell = resolved.list.item_font.clone();
                    move |r: ScaleRow| text(r.0).typeset(&cell, a11y)
                },
            )
            .width(Length::Fixed(160.0)),
            table::column(text("size").typeset(&resolved.list.header_font, a11y), {
                let cell = resolved.list.item_font.clone();
                move |r: ScaleRow| text(r.1).typeset(&cell, a11y)
            })
            .width(Length::Fixed(90.0)),
            table::column(text("weight").typeset(&resolved.list.header_font, a11y), {
                let cell = resolved.list.item_font.clone();
                move |r: ScaleRow| text(r.2).typeset(&cell, a11y)
            })
            .width(Length::Fixed(90.0)),
            table::column(
                text("line height").typeset(&resolved.list.header_font, a11y),
                {
                    let cell = resolved.list.item_font.clone();
                    move |r: ScaleRow| text(r.3).typeset(&cell, a11y)
                },
            )
            .width(Length::Fixed(110.0)),
        ],
        scale_rows,
    )
    .padding_x(sp.s)
    .padding_y(sp.xs)
    .separator_x(sep.line_width)
    .separator_y(sep.line_width)
    .width(Fill);

    let table_demo = hoverable(
        widget_tooltip(
            "Table",
            &[(
                "separators",
                "iced's background.strong, read live",
                state
                    .current_theme
                    .extended_palette()
                    .background
                    .strong
                    .color,
            )],
            &[
                ("separator_x / separator_y", &line_width_s),
                ("cell padding", "the showcase's own scale"),
                (
                    "header",
                    font_row("list.header_font", &resolved.list.header_font).as_str(),
                ),
                (
                    "cells",
                    font_row("list.item_font", &resolved.list.item_font).as_str(),
                ),
            ],
            &[
                (
                    "Style",
                    "iced_widget 0.14.2 gives Table a Catalog and a Style but no \
                     `.style(..)` or `.class(..)` setter (table.rs:149-196), so its \
                     separator colors stay table::default(theme), which reads \
                     palette.background.strong (table.rs:717-724)",
                ),
                (
                    "separator.line_color",
                    "has no receiver here; the hex above is the one on screen, not \
                     the platform's separator colour",
                ),
            ],
        ),
        column![
            text("Table (columns and rows)").role(section_title(ts), resolved, a11y),
            text("The four typographic roles this theme resolves:").body(resolved, a11y),
            scale_table,
        ]
        .spacing(gap.widget)
        .into(),
    );

    column![
        header,
        grid_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        pane_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        table_demo,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Graphics
// ---------------------------------------------------------------------------

/// The document the `markdown` section parses once, at start-up.
const MARKDOWN_SAMPLE: &str = "\
# Markdown under a native theme

`iced_widget::markdown` parses its input once and renders the items every
frame with a `Settings` value. The showcase builds that value from the
resolved theme: heading sizes from `text_scale`, the base size from
`defaults.font`, code size from `defaults.mono_font` and the link colour
below from `link.font.color`.

- headings, lists and rules come from the parser
- [a link, in the platform's own link colour](https://github.com/tiborgats/native-theme)
- inline `code` and code blocks share the monospace weight

```rust
let settings = markdown::Settings::with_text_size(size, style);
```
";

/// A `markdown` viewer whose `h1` and `h2` take their roles' weights.
///
/// `markdown::Settings` carries a size per heading level but one font for all
/// of the text (`markdown.rs:1033-1054`), and iced's `heading` draws a
/// heading's spans in that font (`:1273-1313`). So each of the two swaps in a
/// font at its role's weight before handing the heading to iced's `heading`.
struct RoleHeadings {
    h1: iced::Font,
    h2: iced::Font,
}

impl<'a> markdown::Viewer<'a, Message> for RoleHeadings {
    fn on_link_click(url: markdown::Uri) -> Message {
        Message::MarkdownLinkClicked(url)
    }

    fn heading(
        &self,
        settings: markdown::Settings,
        level: &'a markdown::HeadingLevel,
        text: &'a markdown::Text,
        index: usize,
    ) -> Element<'a, Message> {
        let font = match level {
            markdown::HeadingLevel::H1 => self.h1,
            markdown::HeadingLevel::H2 => self.h2,
            _ => settings.style.font,
        };
        let settings = markdown::Settings {
            style: markdown::Style {
                font,
                ..settings.style
            },
            ..settings
        };
        markdown::heading(settings, level, text, index, Self::on_link_click)
    }
}

/// What the QR code encodes.
const QR_PAYLOAD: &str = "https://github.com/tiborgats/native-theme";

/// A small drawing whose every colour and width is the platform's.
///
/// `canvas` has no `Catalog` and no `Style` of its own: a [`canvas::Program`]
/// is handed the theme and paints whatever it likes (`canvas/program.rs:49`).
/// So the values are captured from the resolved theme when the program is
/// built, which `view` does once per frame.
struct ThemeSketch {
    surface: Color,
    outline: Color,
    outline_width: f32,
    corner_radius: f32,
    accent: Color,
    ink: Color,
    ink_width: f32,
    dot_radius: f32,
}

impl<Message> canvas::Program<Message> for ThemeSketch {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let panel = canvas::Path::rounded_rectangle(
            iced::Point::ORIGIN,
            frame.size(),
            self.corner_radius.into(),
        );
        frame.fill(&panel, self.surface);
        frame.stroke(
            &panel,
            canvas::Stroke::default()
                .with_color(self.outline)
                .with_width(self.outline_width),
        );

        let center = frame.center();
        let baseline = canvas::Path::line(
            iced::Point::new(0.0, center.y),
            iced::Point::new(frame.width(), center.y),
        );
        frame.stroke(
            &baseline,
            canvas::Stroke::default()
                .with_color(self.ink)
                .with_width(self.ink_width),
        );

        frame.fill(&canvas::Path::circle(center, self.dot_radius), self.accent);

        vec![frame.into_geometry()]
    }
}

/// Canvas, QRCode and Markdown: the three modules that paint content the
/// application supplies rather than a control.
fn view_graphics(state: &State) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let sep = &resolved.separator;
    let d = &resolved.defaults;
    let line_width_s = format!("{:.0}px", sep.line_width);
    let dot_s = format!("{:.0}px", d.icon_sizes.large);

    let header = section_header(
        "Graphics Widgets",
        "Canvas, QRCode and Markdown: content the application paints itself",
        resolved,
        ts,
        a11y,
        sp,
    );

    // ---- Canvas ----

    let sketch = ThemeSketch {
        surface: to_color(resolved.card.background_color),
        outline: to_color(sep.line_color),
        outline_width: sep.line_width,
        corner_radius: resolved.card.border.corner_radius,
        accent: to_color(d.accent_color),
        ink: to_color(d.text_color),
        ink_width: sep.line_width,
        // iced states a circle by its radius, the model by its diameter.
        dot_radius: d.icon_sizes.large / 2.0,
    };

    let canvas_demo = hoverable(
        widget_tooltip(
            "Canvas",
            &[
                (
                    "panel fill",
                    "card.background_color",
                    to_color(resolved.card.background_color),
                ),
                ("outline", "separator.line_color", to_color(sep.line_color)),
                ("baseline", "defaults.text_color", to_color(d.text_color)),
                ("dot", "defaults.accent_color", to_color(d.accent_color)),
            ],
            &[
                ("stroke width", &line_width_s),
                ("panel radius", "card.border.corner_radius"),
                ("dot diameter", &dot_s),
            ],
            &[(
                "Style",
                "canvas has no Catalog and no Style: a Program paints with \
                 whatever it is given (canvas/program.rs:49-56)",
            )],
        ),
        column![
            text("Canvas (a drawing from the theme)").role(section_title(ts), resolved, a11y),
            text(
                "Every colour and every width below is a ResolvedTheme field, \
                 captured when the program is built."
            )
            .body(resolved, a11y),
            canvas(sketch)
                .width(Length::Fixed(280.0))
                .height(Length::Fixed(120.0)),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- QR code ----

    let qr_cell = to_color(d.text_color);
    let qr_background = to_color(d.background_color);
    let qr_demo: Element<'_, Message> = match &state.qr_data {
        Some(data) => qr_code(data)
            .style(move |_theme: &Theme| qr_code::Style {
                cell: qr_cell,
                background: qr_background,
            })
            .into(),
        None => text("The payload could not be encoded as a QR code.")
            .role(&ts.caption, resolved, a11y)
            .into(),
    };

    let qr_section = hoverable(
        widget_tooltip(
            "QRCode",
            &[
                ("cell", "defaults.text_color", qr_cell),
                ("background", "defaults.background_color", qr_background),
            ],
            &[("payload", QR_PAYLOAD)],
            &[(
                "cell size",
                "no native source — iced's own DEFAULT_CELL_SIZE (qr_code.rs)",
            )],
        ),
        column![
            text("QRCode").role(section_title(ts), resolved, a11y),
            text("Its two-colour Style is the platform's foreground on its background:")
                .body(resolved, a11y),
            qr_demo,
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- Markdown ----

    let md_settings = {
        // Every field the model does not carry is iced's own, read at run
        // time from the theme the connector produced.
        let iced_style = markdown::Style::from(&state.current_theme);
        let style = markdown::Style {
            font: theme_font(&resolved.defaults.font),
            inline_code_highlight: iced_style.inline_code_highlight,
            inline_code_padding: iced_style.inline_code_padding,
            inline_code_color: iced_style.inline_code_color,
            inline_code_font: theme_mono_font(resolved),
            code_block_font: theme_mono_font(resolved),
            link_color: to_color(resolved.link.font.color),
        };
        let body = native_theme_iced::font_size(resolved, a11y);
        let mut settings = markdown::Settings::with_text_size(body, style);
        settings.h1_size = scaled_text_size(page_title(ts).size, a11y).into();
        settings.h2_size = scaled_text_size(section_title(ts).size, a11y).into();
        settings.h3_size = body.into();
        settings.h4_size = body.into();
        settings.code_size = native_theme_iced::mono_font_size(resolved, a11y).into();
        settings
    };

    let link_line = match &state.markdown_link {
        Some(uri) => format!("Last link clicked: {uri}"),
        None => "Click the link above and it is echoed here.".to_string(),
    };

    let markdown_demo = hoverable(
        widget_tooltip(
            "Markdown",
            &[
                (
                    "link",
                    "link.font.color",
                    to_color(resolved.link.font.color),
                ),
                // Markdown states no colour of its own, so iced paints it with
                // the slot `Base::base` hands every inheriting widget
                // (`iced_core` theme.rs:334); the connector sets that slot from
                // `defaults.text_color`, so reading it live shows what is on
                // screen rather than the palette input behind it.
                (
                    "text",
                    "extended.background.base.text, from defaults.text_color",
                    state.current_theme.extended_palette().background.base.text,
                ),
            ],
            &[
                (
                    "base size",
                    "defaults.font.size, times the text-scaling factor",
                ),
                (
                    "h1 / h2",
                    format!(
                        "text_scale dialog_title / section_heading, size (times the \
                         text-scaling factor); defaults.font's family, {}; weight {} / {}",
                        family_label(
                            &resolved.defaults.font.family,
                            role_family(section_title(ts), resolved),
                            &role_drawn(section_title(ts), resolved)
                        ),
                        weight_label(page_title(ts).weight, &role_drawn(page_title(ts), resolved)),
                        weight_label(
                            section_title(ts).weight,
                            &role_drawn(section_title(ts), resolved)
                        ),
                    )
                    .as_str(),
                ),
                (
                    "h3 – h6",
                    "defaults.font: the model names no role between section_heading \
                     and body text",
                ),
                (
                    "code size",
                    "defaults.mono_font.size, times the text-scaling factor",
                ),
            ],
            &[
                (
                    "block spacing",
                    "Settings::spacing is iced's own, derived from the native base \
                     size. It is not layout.widget_gap: iced reuses the one number \
                     as the gap between blocks, as 0.6 and 0.75 of itself inside \
                     lists, and as a list's horizontal indent (markdown.rs:1230, \
                     :1364-1374)",
                ),
                (
                    "inline code chip",
                    "the model states no inline-code surface; its fill and its \
                     foreground are iced's, as a pair",
                ),
                (
                    "font weight",
                    "iced 0.14's text engine takes a face only at the weight \
                     asked for, and files a variable font at its default weight \
                     alone; the showcase files the weights a variable face's wght \
                     axis covers (register_weight), and a weight no face has is \
                     drawn at the family's nearest face, as the h1 / h2 row says",
                ),
            ],
        ),
        column![
            text("Markdown").role(section_title(ts), resolved, a11y),
            container(markdown::view_with(
                state.markdown_content.items(),
                md_settings,
                &RoleHeadings {
                    h1: role_font(page_title(ts), resolved),
                    h2: role_font(section_title(ts), resolved),
                },
            ))
            .padding(Padding::from(gap.container))
            .style(styles::container_card(resolved))
            .width(Fill),
            text(link_line).role(&ts.caption, resolved, a11y),
        ]
        .spacing(gap.widget)
        .into(),
    );

    column![
        header,
        canvas_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        qr_section,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        markdown_demo,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Extra widgets (iced_aw)
// ---------------------------------------------------------------------------

/// An entry of a window menu ([`menu_bar`]): a row, tagged `id` where the
/// element list names it, with its label, its key binding and what it does;
/// or a separator.
enum MenuEntry {
    Row {
        id: Option<&'static str>,
        label: &'static str,
        key: Option<&'static str>,
        action: Message,
    },
    Separator(Option<&'static str>),
}

impl MenuEntry {
    /// A row's label and key binding as the row spells them; none for a
    /// separator.
    fn texts(&self) -> Option<(&'static str, Option<String>)> {
        match self {
            Self::Row { label, key, .. } => Some((label, key.map(binding))),
            Self::Separator(_) => None,
        }
    }
}

/// How wide a drop-down menu of a `MenuBar` is, as the gpui and egui
/// showcases size theirs: its widest row -- the label, then `gap` and the
/// shortcut where the row has one -- inside the row's padding `pad`. Each
/// text is measured as iced shapes it in `font` at `size`.
///
/// `MenuTheme` states no menu width, and iced_aw lays a menu's rows out at
/// one width, the `Menu`'s (`Menu::new` fills the width its limits give,
/// iced_aw 0.14.1 `src/widget/menu/menu_tree.rs:151-165`), which the
/// showcase sets to this.
fn menu_width<'a>(
    rows: impl IntoIterator<Item = (&'a str, Option<String>)>,
    font: iced::Font,
    size: f32,
    pad: Padding,
    gap: f32,
) -> f32 {
    let width = |content: &str| {
        MeasuredText {
            content: content.to_owned(),
            size,
            line: size,
            font,
        }
        .extent()
        .width
    };
    let widest = rows
        .into_iter()
        .map(|(label, shortcut)| match shortcut {
            Some(shortcut) => width(label) + gap + width(&shortcut),
            None => width(label),
        })
        .fold(0.0, f32::max);
    pad.left + widest + pad.right
}

/// The padding `iced_aw` gives a menu's panel round its rows where the
/// application sets none: `Padding::new(5.0)` in `Menu::new` (iced_aw 0.14.1
/// `src/widget/menu/menu_tree.rs:161`), which it keeps in no constant.
const AW_MENU_PADDING: f32 = 5.0;

/// The padding of a menu's panel round its rows: the popup's,
/// `popover.border.padding` (platform-facts §2.6: the popup border is
/// §2.16's), inside `popover.border.line_width`, which iced_aw paints over
/// the panel's padding (`pad_rectangle` round the rows, one quad with its
/// border, iced_aw 0.14.1 `src/widget/menu/menu_tree.rs:764-780`);
/// [`AW_MENU_PADDING`] on a side the theme leaves unstated.
fn menu_popup_padding(resolved: &ResolvedTheme) -> Padding {
    native_theme_iced::padding_inside_border(
        &resolved.popover.border,
        Padding::new(AW_MENU_PADDING),
    )
}

/// A drop-down of the `MenuBar`, `width` wide ([`menu_width`]), with the
/// gap `iced_aw` leaves around it.
#[cfg(feature = "iced_aw")]
fn aw_menu<'a>(
    items: Vec<Item<'a, Message, Theme, iced::Renderer>>,
    width: f32,
    gap: f32,
) -> Menu<'a, Message, Theme, iced::Renderer> {
    Menu::new(items).width(width).offset(gap).spacing(gap)
}

/// The six `styles::aw::*` functions, on the eight `iced_aw` widgets the
/// connector's feature enables.
#[cfg(feature = "iced_aw")]
fn view_extra(state: &State) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let sep = &resolved.separator;
    let card_t = &resolved.card;
    let menu_t = &resolved.menu;
    let tab_t = &resolved.tab;
    let side_t = &resolved.sidebar;
    let list_t = &resolved.list;
    let spin_t = &resolved.spinner;

    let header = section_header(
        "Extra widgets (iced_aw)",
        "The six styles::aw functions, each through the setter its widget offers",
        resolved,
        ts,
        a11y,
        sp,
    );

    // ---- Card ----

    let card_section: Element<'_, Message> = if state.aw_card_open {
        let card = Card::new(
            text("Card").role(section_title(ts), resolved, a11y),
            column![
                text(
                    "CardTheme states one fill and one border, so the head, the body \
                     and the foot are the same surface, and the three labels are \
                     defaults.text_color."
                )
                .role(&ts.caption, resolved, a11y),
            ]
            .spacing(sp.xs),
        )
        .foot(Element::from(
            row![
                button(text("Dismiss").typeset(&resolved.button.font, a11y))
                    .on_press(Message::AwCardToggled)
                    .style(styles::button(resolved))
                    .padding(native_theme_iced::button_padding(resolved)),
            ]
            .spacing(sp.xs),
        ))
        .width(Length::Fixed(420.0))
        .style(styles::aw::card(resolved));
        // `Card::padding` sets the head, the body and the foot alike, and
        // iced_aw keeps the default it replaces private (widget/card.rs:21),
        // so a theme that leaves a side unstated leaves the card iced_aw's.
        match native_theme_iced::stated_padding(&card_t.border.padding) {
            Some(padding) => card.padding(padding),
            None => card,
        }
        .into()
    } else {
        button(text("Show the card again").typeset(&resolved.button.font, a11y))
            .on_press(Message::AwCardToggled)
            .style(styles::button_primary(resolved))
            .padding(native_theme_iced::button_padding(resolved))
            .into()
    };

    let card_demo = hoverable(
        widget_tooltip(
            "Card",
            &[
                (
                    "surface",
                    "card.background_color",
                    to_color(card_t.background_color),
                ),
                ("border", "card.border.color", to_color(card_t.border.color)),
                (
                    "labels",
                    "defaults.text_color",
                    to_color(resolved.defaults.text_color),
                ),
            ],
            &[
                ("radius", "card.border.corner_radius"),
                (
                    "section padding",
                    "card.border.padding, on the head, the body and the foot \
                     alike, where the theme states every side; otherwise \
                     iced_aw's own, which it keeps private (widget/card.rs:21)",
                ),
                ("Dismiss padding", "button_padding"),
                (
                    "Dismiss label",
                    font_row("button.font", &resolved.button.font).as_str(),
                ),
            ],
            &[(
                "close icon",
                "not drawn: iced_aw 0.14.1 styles Card::on_close's button \
                     with its default class, whose icon is white whatever \
                     styles::aw::card sets (widget/card.rs:193-206), so the \
                     themed Dismiss button closes the card instead",
            )],
        ),
        column![
            text("Card").role(section_title(ts), resolved, a11y),
            card_section,
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- MenuBar and Menu ----

    // The items are buttons padded like the platform's menu items: the sides
    // menu.border.padding states, a button's own elsewhere. A theme that
    // states no row height (KDE's items size to their font) leaves the item
    // its own height.
    let item_pad = native_theme_iced::padding_or(&menu_t.border.padding, button::DEFAULT_PADDING);
    let menu_entry = move |label: &'static str| -> Element<'_, Message> {
        let entry = button(
            text(label)
                .typeset(&menu_t.font, a11y)
                .wrapping(text::Wrapping::None),
        )
        .on_press(Message::AwActionChosen(format!("Menu: {label}")))
        .style(styles::button(resolved))
        .width(Fill)
        .padding(item_pad);
        match menu_t.row_height {
            Some(h) => entry.height(Length::Fixed(h)),
            None => entry,
        }
        .into()
    };
    let menu_root = move |label: &'static str| {
        button(text(label).typeset(&menu_t.font, a11y))
            .on_press(Message::AwActionChosen(format!("Menu: {label}")))
            .style(styles::button(resolved))
            .padding(item_pad)
    };
    // A menu of the rows `entries`, each with the menu it opens where it
    // opens one, as wide as its widest row.
    let drop = |entries: Vec<(&'static str, Option<_>)>| {
        let width = menu_width(
            entries.iter().map(|&(label, _)| (label, None)),
            theme_font(&menu_t.font),
            scaled_text_size(menu_t.font.size, a11y),
            item_pad,
            gap.widget,
        );
        let items = entries
            .into_iter()
            .map(|(label, menu)| match menu {
                Some(menu) => Item::with_menu(menu_entry(label), menu),
                None => Item::new(menu_entry(label)),
            })
            .collect();
        aw_menu(items, width, sp.xxs)
    };

    let menu_bar = MenuBar::new(vec![
        Item::with_menu(
            menu_root("File"),
            drop(vec![
                ("New window", None),
                ("Open preset", None),
                (
                    "Recent",
                    Some(drop(vec![
                        ("adwaita.toml", None),
                        ("kde-breeze.toml", None),
                    ])),
                ),
            ]),
        ),
        Item::with_menu(
            menu_root("View"),
            drop(vec![
                ("Light", None),
                ("Dark", None),
                ("Follow the system", None),
            ]),
        ),
    ])
    .padding(Padding::from(sp.xxs))
    .spacing(sp.xs)
    .style(styles::aw::menu(resolved));

    let menu_demo = hoverable(
        widget_tooltip(
            "MenuBar / Menu",
            &[
                (
                    "bar and panel",
                    "menu.background_color",
                    to_color(menu_t.background_color),
                ),
                (
                    "open path",
                    "menu.hover_background",
                    to_color(menu_t.hover_background),
                ),
                ("border", "menu.border.color", to_color(menu_t.border.color)),
            ],
            &[
                (
                    "item height",
                    "menu.row_height where the theme states one, on the item button",
                ),
                (
                    "item padding",
                    "menu.border.padding's stated sides, iced's \
                     button::DEFAULT_PADDING for the others, on the item button",
                ),
                (
                    "item label",
                    format!(
                        "{}, the context menu's too",
                        font_row("menu.font", &resolved.menu.font)
                    )
                    .as_str(),
                ),
            ],
            &[
                (
                    "label colour",
                    "menu_bar::Style has no text colour; the items are our own \
                     widgets, so they take styles::button",
                ),
                (
                    "shadows",
                    "the model has no shadow geometry — iced_aw's own",
                ),
                (
                    "menu width",
                    "MenuTheme states none — its widest row inside the item padding",
                ),
            ],
        ),
        column![
            text("MenuBar and its Menus").role(section_title(ts), resolved, a11y),
            menu_bar,
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- ContextMenu ----

    let context_underlay = container(
        column![
            text("Right-click inside this panel").body(resolved, a11y),
            text(
                "ContextMenu gets no styles::aw function: its own Style is a one-field \
                 backdrop scrim and its default class already emits alpha 0 \
                 (style/context_menu.rs:48-59). The popup below is our own element; \
                 its entries are padded like the menu bar's items, by \
                 menu.border.padding."
            )
            .role(&ts.caption, resolved, a11y),
        ]
        .spacing(sp.xs),
    )
    .padding(Padding::from(gap.container))
    .style(styles::container_card(resolved))
    .width(Fill);

    let context_demo = ContextMenu::new(context_underlay, move || {
        let entry = |label: &'static str| -> Element<'_, Message> {
            button(text(label).typeset(&menu_t.font, a11y))
                .on_press(Message::AwActionChosen(format!("Context menu: {label}")))
                .style(styles::button(resolved))
                .width(Fill)
                .padding(item_pad)
                .into()
        };
        container(column![entry("Copy"), entry("Paste"), entry("Select all")].spacing(sp.xxs))
            .padding(Padding::from(sp.xs))
            .style(styles::container_card(resolved))
            .width(Length::Fixed(180.0))
            .into()
    });

    // ---- TabBar and Tabs ----

    let tab_bar = TabBar::new(Message::AwTabBarSelected)
        .push(0usize, TabLabel::Text("Overview".to_string()))
        .push(1usize, TabLabel::Text("Details".to_string()))
        .push(2usize, TabLabel::Text("About".to_string()))
        .set_active_tab(&state.aw_tab_bar_active)
        .text_size(scaled_text_size(tab_t.font.size, a11y))
        .text_font(theme_font(&tab_t.font))
        .tab_width(Length::Fixed(tab_t.min_width))
        .height(Length::Fixed(tab_t.min_height))
        .spacing(sp.xxs);
    // iced_aw keeps a tab's default padding private (widget/tab_bar.rs:41) and
    // takes a padding whole, so only a theme that states every side replaces it.
    let tab_padding = native_theme_iced::stated_padding(&tab_t.border.padding);
    let tab_bar = match tab_padding {
        Some(padding) => tab_bar.padding(padding),
        None => tab_bar,
    };

    let tab_bar_body = text(match state.aw_tab_bar_active {
        0 => "A stand-alone TabBar reports the selection and shows nothing itself.",
        1 => "The second tab. Its label colour is tab.active_text_color.",
        _ => "An unselected tab is Status::Disabled to iced_aw, not a dead one.",
    })
    .role(&ts.caption, resolved, a11y);

    let tabs = Tabs::new(Message::AwTabsSelected)
        .push(
            0usize,
            TabLabel::Text("Colours".to_string()),
            container(
                text("Tabs owns its content and forwards the bar's style to the TabBar it holds.")
                    .role(&ts.caption, resolved, a11y),
            )
            .padding(Padding::from(sp.s)),
        )
        .push(
            1usize,
            TabLabel::Text("Sizes".to_string()),
            container(
                text(format!(
                    "tab.min_width {:.0}px · tab.min_height {:.0}px",
                    tab_t.min_width, tab_t.min_height
                ))
                .role(&ts.caption, resolved, a11y),
            )
            .padding(Padding::from(sp.s)),
        )
        .set_active_tab(&state.aw_tabs_active)
        .text_size(scaled_text_size(tab_t.font.size, a11y))
        .text_font(theme_font(&tab_t.font))
        .tab_bar_height(Length::Fixed(tab_t.min_height))
        .tab_bar_style(styles::aw::tab_bar(resolved))
        .height(Length::Shrink);
    let tabs = match tab_padding {
        Some(padding) => tabs.tab_label_padding(padding),
        None => tabs,
    };

    let tab_demo = hoverable(
        widget_tooltip(
            "TabBar / Tabs",
            &[
                (
                    "strip",
                    "tab.bar_background",
                    to_color(tab_t.bar_background),
                ),
                (
                    "selected tab",
                    "tab.active_background",
                    to_color(tab_t.active_background),
                ),
                (
                    "selected label",
                    "tab.active_text_color",
                    to_color(tab_t.active_text_color),
                ),
                (
                    "hovered tab",
                    "tab.hover_background",
                    to_color(tab_t.hover_background.unwrap_or(tab_t.background_color)),
                ),
            ],
            &[
                ("label", font_row("tab.font", &resolved.tab.font).as_str()),
                ("tab width", "tab.min_width"),
                ("bar height", "tab.min_height"),
                (
                    "tab padding",
                    "tab.border.padding where the theme states every side; \
                     otherwise iced_aw's own, which it keeps private \
                     (widget/tab_bar.rs:41)",
                ),
            ],
            &[(
                "min_width / min_height",
                "the platform states minima and iced_aw takes fixed lengths, so a \
                 tab here is exactly its minimum",
            )],
        ),
        column![
            text("TabBar (stand-alone) and Tabs (with content)").role(
                section_title(ts),
                resolved,
                a11y
            ),
            tab_bar.style(styles::aw::tab_bar(resolved)),
            tab_bar_body,
            tabs,
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- Sidebar ----

    let side_bar = Sidebar::new(Message::AwSidebarSelected)
        // A `Sidebar` declares a `TabLabel` of its own (sidebar/sidebar.rs:51).
        .push(
            0usize,
            iced_aw::sidebar::TabLabel::Text("General".to_string()),
        )
        .push(
            1usize,
            iced_aw::sidebar::TabLabel::Text("Appearance".to_string()),
        )
        .push(
            2usize,
            iced_aw::sidebar::TabLabel::Text("Icons".to_string()),
        )
        .set_active_tab(&state.aw_sidebar_active)
        .text_size(scaled_text_size(side_t.font.size, a11y))
        .text_font(theme_font(&side_t.font))
        .width(Length::Fixed(200.0))
        .height(Length::Shrink)
        .style(styles::aw::sidebar(resolved));

    let side_demo = hoverable(
        widget_tooltip(
            "Sidebar",
            &[
                (
                    "panel",
                    "sidebar.background_color",
                    to_color(side_t.background_color),
                ),
                (
                    "selected item",
                    "sidebar.selection_background",
                    to_color(side_t.selection_background),
                ),
                (
                    "selected label",
                    "sidebar.selection_text_color",
                    to_color(side_t.selection_text_color),
                ),
                (
                    "hovered item",
                    "sidebar.hover_background",
                    to_color(side_t.hover_background),
                ),
            ],
            &[(
                "label",
                font_row("sidebar.font", &resolved.sidebar.font).as_str(),
            )],
            &[(
                "corner radius",
                "sidebar::Style carries none but the close icon's; iced_aw rounds \
                 the panel with a hardcoded 0",
            )],
        ),
        column![
            text("Sidebar").role(section_title(ts), resolved, a11y),
            row![
                side_bar,
                container(
                    text(match state.aw_sidebar_active {
                        0 =>
                            "General: an unselected item shows the panel itself — the \
                              platform states no fill of its own for one.",
                        1 => "Appearance: the selected item is sidebar.selection_background.",
                        _ => "Icons: hovering an item paints sidebar.hover_background.",
                    })
                    .role(&ts.caption, resolved, a11y),
                )
                .padding(Padding::from(gap.container))
                .style(styles::container_card(resolved))
                .width(Fill),
            ]
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- Spinner ----

    let spinner_demo = hoverable(
        widget_tooltip(
            "Spinner",
            &[(
                "orbiting dot",
                "spinner.fill_color",
                to_color(spin_t.fill_color),
            )],
            &[("orbit diameter", "spinner.diameter")],
            &[
                (
                    "Style",
                    "iced_aw 0.14.1 gives Spinner none at all: it paints in the \
                     inherited text colour, which the wrapping container states",
                ),
                (
                    "spinner.stroke_width",
                    "no receiver: iced_aw paints no ring to stroke — one dot \
                     circling the centre (spinner.rs:151), whose radius is \
                     circle_radius, not a stroke width",
                ),
            ],
        ),
        column![
            text("Spinner (styled through its container)").role(section_title(ts), resolved, a11y),
            container(
                Spinner::new()
                    .width(Length::Fixed(spin_t.diameter))
                    .height(Length::Fixed(spin_t.diameter)),
            )
            .style(styles::aw::spinner(resolved)),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- SelectionList ----

    // `iced_aw` lays a row out as `text_size + padding.y()`
    // (`selection_list/list.rs:118`, `:209`), so `list.row_height` is reachable
    // after all -- not through a setter of its own, but as the vertical padding
    // that makes the sum come out. `padding.y()` is top plus bottom, so each
    // side takes half of what the label leaves. A theme that states no row
    // height, or one that does not clear its own label size, leaves the
    // showcase's own padding standing, rather than a negative inset.
    let row_height_s = match list_t.row_height {
        Some(h) => format!("{h:.0}px"),
        None => "not stated: the showcase's own padding".to_string(),
    };
    let row_label = scaled_text_size(list_t.item_font.size, a11y);
    let row_inset = list_t.row_height.map_or(0.0, |h| h - row_label);
    let list_padding = if row_inset > 0.0 {
        // Only the vertical half is the platform's: the label is drawn at the
        // row's own `bounds.x` (`selection_list/list.rs:274`), so horizontal
        // padding never reaches it, and the list's intrinsic width does read
        // `padding.x()` (`selection_list.rs:237`, `:244`).
        Padding::ZERO
            .top(row_inset / 2.0)
            .bottom(row_inset / 2.0)
            .left(sp.xs)
            .right(sp.xs)
    } else {
        Padding::from(sp.xs)
    };

    // `SelectionList::new_with` is the only constructor that gives the rows a
    // class as well as the list, and it demands a `Clone` style function --
    // which every `styles::aw::*` closure is.
    let selection_list = SelectionList::new_with(
        &state.aw_list_options,
        Message::AwListSelected,
        row_label,
        list_padding,
        styles::aw::selection_list(resolved),
        state.aw_list_selected,
        theme_font(&list_t.item_font),
    )
    .width(Length::Fixed(260.0))
    .height(Length::Fixed(180.0));

    let list_demo = hoverable(
        widget_tooltip(
            "SelectionList",
            &[
                (
                    "list",
                    "list.background_color",
                    to_color(list_t.background_color),
                ),
                (
                    "selected row",
                    "list.selection_background",
                    to_color(list_t.selection_background),
                ),
                (
                    "hovered row",
                    "list.hover_background",
                    to_color(list_t.hover_background),
                ),
                (
                    "row label",
                    "list.item_font.color",
                    to_color(list_t.item_font.color),
                ),
            ],
            &[
                (
                    "row label",
                    font_row("list.item_font", &resolved.list.item_font).as_str(),
                ),
                ("row height", &row_height_s),
            ],
            &[(
                "list.row_height",
                "reachable only indirectly: iced_aw has no item_height setter, so \
                 a row is text_size + padding.y() (selection_list/list.rs:118, \
                 :209) and the showcase gives new_with the padding that makes the \
                 sum the platform's height",
            )],
        ),
        column![
            text("SelectionList").role(section_title(ts), resolved, a11y),
            probe(probes::SELECTION_LIST, Length::Shrink, selection_list),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let action_line = if state.aw_last_action.is_empty() {
        "Nothing chosen yet. Use the menu bar, the context menu or the list.".to_string()
    } else {
        state.aw_last_action.clone()
    };

    column![
        header,
        text(action_line).role(&ts.caption, resolved, a11y),
        card_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        menu_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        column![
            text("ContextMenu").role(section_title(ts), resolved, a11y),
            context_demo,
        ]
        .spacing(gap.widget),
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        tab_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        side_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        spinner_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        list_demo,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Icons
// ---------------------------------------------------------------------------

fn view_icons(state: &State) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let loaded_count = state
        .loaded_icons
        .iter()
        .filter(|i| i.data.is_some())
        .count();
    let system_count = state
        .loaded_icons
        .iter()
        .filter(|i| i.source == IconSource::System)
        .count();
    let fallback_count = state
        .loaded_icons
        .iter()
        .filter(|i| i.source == IconSource::Fallback)
        .count();
    let total_count = state.loaded_icons.len();

    let header = column![
        text("Icons").role(page_title(ts), resolved, a11y),
        text(format!(
            "All {total_count} IconRole variants — \
             {loaded_count} loaded, {system_count} system, {fallback_count} fallback"
        ))
        .body(resolved, a11y),
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
    ]
    .spacing(sp.xs);

    let icon_set_info = column![
        text(format!("Active icon set: {}", state.icon_set_choice)).body(resolved, a11y),
        text(state.system_icon_theme_label()).role(&ts.caption, resolved, a11y),
    ]
    .spacing(sp.xs);

    // Use the theme's foreground color for colorizing icons
    let fg_color = state.current_theme.palette().text;

    // Build grid rows of 6 icons each
    let icons_per_row = 6;
    let mut grid_rows: Vec<Element<'_, Message>> = Vec::new();
    let mut idx = 0;
    while idx < state.loaded_icons.len() {
        let end = (idx + icons_per_row).min(state.loaded_icons.len());
        let row_icons: Vec<Element<'_, Message>> = state.loaded_icons[idx..end]
            .iter()
            .map(|loaded| build_icon_cell(loaded, resolved, fg_color, &ts.caption, a11y, sp.xxs))
            .collect();
        grid_rows.push(row(row_icons).spacing(gap.widget).into());
        idx = end;
    }

    let animated_section = view_animated_icons(state, fg_color);
    let mut content = column![
        header,
        icon_set_info,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        animated_section,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved))
    ]
    // Header, icon-set summary and animated section are major sections of the
    // tab, the same role the other tab roots give `gap.section`.
    .spacing(gap.section);
    for r in grid_rows {
        content = content.push(r);
    }

    content.width(Fill).into()
}

fn view_animated_icons<'a>(state: &'a State, fg_color: Color) -> Element<'a, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let icon_px = resolved.defaults.icon_sizes.large;
    let title = text("Animated Icons").role(section_title(ts), resolved, a11y);
    let divider = rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved));

    // Collect spinner columns into a row
    let mut spinners: Vec<Element<'a, Message>> = Vec::new();

    if state.motion_reduced() {
        // Reduced motion: show static first-frame for each animated icon
        for (set_name, handle) in &state.animated_static {
            let icon = svg(handle.clone())
                .width(Length::Fixed(icon_px))
                .height(Length::Fixed(icon_px))
                .style(move |_theme, _status| iced::widget::svg::Style {
                    color: Some(fg_color),
                });
            let label = text(format!("{} - Static (reduced motion)", set_name)).role(
                &ts.caption,
                resolved,
                a11y,
            );
            spinners.push(
                column![icon, label]
                    .spacing(sp.xs)
                    .align_x(iced::Center)
                    .into(),
            );
        }
    } else {
        // Frame-based animations
        for (i, (set_name, anim_handles)) in state.animated_frames.iter().enumerate() {
            let frame_idx = state.animated_frame_indices[i];
            let icon = svg(anim_handles.handles[frame_idx].clone())
                .width(Length::Fixed(icon_px))
                .height(Length::Fixed(icon_px))
                .style(move |_theme, _status| iced::widget::svg::Style {
                    color: Some(fg_color),
                });
            let label = text(format!(
                "{} - Frames: {} ({}ms)",
                set_name,
                anim_handles.handles.len(),
                anim_handles.frame_duration_ms,
            ))
            .role(&ts.caption, resolved, a11y);
            spinners.push(
                column![icon, label]
                    .spacing(sp.xs)
                    .align_x(iced::Center)
                    .into(),
            );
        }

        // Spin-based animations
        for (set_name, handle, duration_ms) in &state.animated_spins {
            let angle = spin_rotation_radians(state.animation_start.elapsed(), *duration_ms);
            let icon = svg(handle.clone())
                .width(Length::Fixed(icon_px))
                .height(Length::Fixed(icon_px))
                .rotation(iced::Rotation::Floating(angle))
                .style(move |_theme, _status| iced::widget::svg::Style {
                    color: Some(fg_color),
                });
            let label = text(format!("{} - Spin ({}ms)", set_name, duration_ms)).role(
                &ts.caption,
                resolved,
                a11y,
            );
            spinners.push(
                column![icon, label]
                    .spacing(sp.xs)
                    .align_x(iced::Center)
                    .into(),
            );
        }
    }

    let mut content = column![title, divider].spacing(gap.widget);

    if state.motion_reduced() {
        content = content.push(text("prefers-reduced-motion: showing static frames").role(
            &ts.caption,
            resolved,
            a11y,
        ));
    }

    if spinners.is_empty() {
        content = content.push(
            text("No animated icons available for this configuration.").role(
                &ts.caption,
                resolved,
                a11y,
            ),
        );
    } else {
        content = content.push(row(spinners).spacing(gap.widget));
    }

    content.into()
}

fn build_icon_cell<'a>(
    loaded: &LoadedIcon,
    resolved: &ResolvedTheme,
    fg_color: Color,
    caption: &ResolvedTextScaleEntry,
    a11y: &AccessibilityPreferences,
    xxs_spacing: f32,
) -> Element<'a, Message> {
    let role_name = format!("{:?}", loaded.role);
    let icon_name_str = loaded.name.unwrap_or("(unmapped)");
    let source_label = loaded.source.label();
    let icon_px = resolved.defaults.icon_sizes.toolbar;
    let cell_px = resolved.defaults.icon_sizes.large;

    let icon_element: Element<'a, Message> = match &loaded.data {
        Some(data @ IconData::Svg(_)) => {
            if loaded.source == IconSource::System {
                // System icons: render as-is without colorization
                match native_theme_iced::icons::to_svg_handle(data, None) {
                    Some(handle) => svg(handle)
                        .width(Length::Fixed(icon_px))
                        .height(Length::Fixed(icon_px))
                        .into(),
                    None => placeholder_icon(icon_px),
                }
            } else {
                // Bundled/fallback: colorize with theme foreground
                match native_theme_iced::icons::to_svg_handle(data, Some(fg_color)) {
                    Some(handle) => svg(handle)
                        .width(Length::Fixed(icon_px))
                        .height(Length::Fixed(icon_px))
                        .into(),
                    None => placeholder_icon(icon_px),
                }
            }
        }
        Some(data @ IconData::Rgba { .. }) => {
            match native_theme_iced::icons::to_image_handle(data) {
                Some(handle) => iced::widget::image(handle)
                    .width(Length::Fixed(icon_px))
                    .height(Length::Fixed(icon_px))
                    .into(),
                None => placeholder_icon(icon_px),
            }
        }
        _ => placeholder_icon(icon_px),
    };

    let info = format!("{role_name}\nicon: {icon_name_str}\nsource: {source_label}");

    // Wrap in mouse_area for Widget Info hover
    mouse_area(
        container(
            column![
                container(icon_element)
                    .center_x(Length::Fixed(cell_px))
                    .center_y(Length::Fixed(cell_px)),
                text(role_name.clone()).role(caption, resolved, a11y),
                text(source_label).role(caption, resolved, a11y),
            ]
            .spacing(xxs_spacing)
            .align_x(iced::Center),
        )
        .padding(Padding::from(SP.xs))
        .style(styles::container_card(resolved))
        .width(Length::Fixed(100.0)),
    )
    .on_enter(Message::WidgetHovered(info))
    .on_exit(Message::WidgetUnhovered)
    .into()
}

/// A "?" in the slot of an icon that did not load, drawn at the icon's own
/// size: it stands in for an icon, not for text in any role.
fn placeholder_icon<'a>(icon_px: f32) -> Element<'a, Message> {
    container(
        text("?")
            .size(icon_px)
            .line_height(text::LineHeight::Absolute(icon_px.into())),
    )
    .center_x(Length::Fixed(icon_px))
    .center_y(Length::Fixed(icon_px))
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Theme Map
// ---------------------------------------------------------------------------

fn view_theme_map(state: &State) -> Element<'_, Message> {
    let a11y = &state.accessibility;
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let header = section_header(
        "Theme Map",
        "All palette and extended palette colors from the current theme",
        resolved,
        ts,
        a11y,
        sp,
    );

    let palette = state.current_theme.palette();
    let extended = state.current_theme.extended_palette();
    let swatch_border = native_theme_iced::border_color(&state.current_resolved);
    let swatch_bw = state.current_resolved.defaults.border.line_width;
    let swatch_r = native_theme_iced::border_radius(&state.current_resolved);
    let swatch_frame = iced::Border {
        color: swatch_border,
        width: swatch_bw,
        radius: swatch_r.into(),
    };
    // Local closure for concise swatch calls
    let caption = &ts.caption;
    let xxs_sp = sp.xxs;
    let cs = |label: &'static str, color: Color| -> Element<'_, Message> {
        color_swatch(label, color, swatch_frame, caption, resolved, a11y, xxs_sp)
    };

    let swatch_style = SwatchStyle {
        border_color: swatch_border,
        border_width: swatch_bw,
        radius: swatch_r,
        heading: section_title(ts),
        caption: &ts.caption,
        resolved,
        a11y,
        xxs_spacing: sp.xxs,
        swatch_spacing: sp.m,
        column_spacing: sp.s,
    };

    // Base palette (6 colors)
    let base_palette = hoverable(
        widget_tooltip(
            "Base Palette (6 fields)",
            &[
                ("background", "background", palette.background),
                ("text", "text", palette.text),
                ("primary", "primary", palette.primary),
                ("success", "success", palette.success),
                ("warning", "warning", palette.warning),
                ("danger", "danger", palette.danger),
            ],
            &[],
            &[],
        ),
        column![
            text("Base Palette (6 fields)").role(section_title(ts), resolved, a11y),
            row![
                cs("background", palette.background),
                cs("text", palette.text),
                cs("primary", palette.primary),
                cs("success", palette.success),
                cs("warning", palette.warning),
                cs("danger", palette.danger),
            ]
            .spacing(sp.m),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // Extended palette sections (all 6 families use hoverable_ext_section)
    let ext_background = hoverable_ext_section(
        "Background (Extended)",
        extended.background.base,
        extended.background.weak,
        extended.background.strong,
        &swatch_style,
    );
    let ext_primary = hoverable_ext_section(
        "Primary (Extended)",
        extended.primary.base,
        extended.primary.weak,
        extended.primary.strong,
        &swatch_style,
    );
    let ext_secondary = hoverable_ext_section(
        "Secondary (Extended)",
        extended.secondary.base,
        extended.secondary.weak,
        extended.secondary.strong,
        &swatch_style,
    );
    let ext_success = hoverable_ext_section(
        "Success (Extended)",
        extended.success.base,
        extended.success.weak,
        extended.success.strong,
        &swatch_style,
    );
    let ext_warning = hoverable_ext_section(
        "Warning (Extended)",
        extended.warning.base,
        extended.warning.weak,
        extended.warning.strong,
        &swatch_style,
    );
    let ext_danger = hoverable_ext_section(
        "Danger (Extended)",
        extended.danger.base,
        extended.danger.weak,
        extended.danger.strong,
        &swatch_style,
    );

    // Resolved theme colors (defaults + selected per-widget)
    let native_colors = {
        let d = &state.current_resolved.defaults;
        let r = &state.current_resolved;
        let pairs: Vec<(&str, native_theme::color::Rgba)> = vec![
            ("accent", d.accent_color),
            ("background", d.background_color),
            ("text", d.text_color),
            ("surface", d.surface_color),
            ("border", d.border.color),
            ("muted", d.muted_color),
            ("shadow", d.shadow_color),
            ("accent_fg", d.accent_text_color),
            ("btn_bg", r.button.background_color),
            ("btn_fg", r.button.font.color),
            ("btn_primary", r.button.primary_background),
            ("danger", d.danger_color),
            ("danger_fg", d.danger_text_color),
            ("warning", d.warning_color),
            ("warning_fg", d.warning_text_color),
            ("success", d.success_color),
            ("success_fg", d.success_text_color),
            ("info", d.info_color),
            ("info_fg", d.info_text_color),
            ("selection", d.selection_background),
            ("selection_fg", d.selection_text_color),
            ("link", d.link_color),
            ("focus_ring", d.focus_ring_color),
            ("sidebar_bg", r.sidebar.background_color),
            ("sidebar_fg", r.sidebar.font.color),
            ("tooltip_bg", r.tooltip.background_color),
            ("tooltip_fg", r.tooltip.font.color),
            ("popover_bg", r.popover.background_color),
            ("popover_fg", r.popover.font.color),
            ("input_bg", r.input.background_color),
            ("input_fg", r.input.font.color),
            ("disabled_fg", d.disabled_text_color),
            ("separator", r.separator.line_color),
            ("alt_row", r.list.alternate_row_background),
            ("sel_inactive", d.selection_inactive_background),
            ("card_bg", r.card.background_color),
        ];

        // Wrap into rows of 6
        let mut rows: Vec<Element<'_, Message>> = Vec::new();
        let mut idx = 0;
        while idx < pairs.len() {
            let end = (idx + 6).min(pairs.len());
            let row_items: Vec<Element<'_, Message>> = pairs[idx..end]
                .iter()
                .map(|(name, rgba)| {
                    let [cr, cg, cb, ca] = rgba.to_f32_array();
                    color_swatch(
                        name,
                        Color::from_rgba(cr, cg, cb, ca),
                        swatch_frame,
                        &ts.caption,
                        resolved,
                        a11y,
                        sp.xxs,
                    )
                })
                .collect();
            rows.push(row(row_items).spacing(sp.m).into());
            idx = end;
        }

        let mut col = column![text("Resolved Theme Colors (defaults + per-widget)").role(
            section_title(ts),
            resolved,
            a11y
        )]
        .spacing(gap.widget);
        for r in rows {
            col = col.push(r);
        }
        col
    };

    column![
        header,
        base_palette,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_background,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_primary,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_secondary,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_success,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_warning,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_danger,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        native_colors,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

fn color_to_hex(c: Color) -> String {
    let r = (c.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (c.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (c.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = c.a.clamp(0.0, 1.0);
    if (a - 1.0).abs() < f32::EPSILON {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        let a8 = (a * 255.0).round() as u8;
        format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a8)
    }
}

fn color_swatch<'a>(
    label: &'a str,
    color: Color,
    border: iced::Border,
    caption: &ResolvedTextScaleEntry,
    resolved: &ResolvedTheme,
    a11y: &AccessibilityPreferences,
    xxs_spacing: f32,
) -> Element<'a, Message> {
    let hex = color_to_hex(color);
    // Determine if text should be light or dark for contrast
    let luminance = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
    let text_color = if luminance > 0.5 {
        Color::BLACK
    } else {
        Color::WHITE
    };

    // Derive swatch padding from the theme spacing scale (xxs ~= 2px).
    // Vertical uses 3×xxs, horizontal uses 2×xxs — keeps the swatch compact
    // while scaling proportionally with the theme.
    let swatch_pad = Padding::new(0.0)
        .top(xxs_spacing * 3.0)
        .bottom(xxs_spacing * 3.0)
        .left(xxs_spacing * 2.0)
        .right(xxs_spacing * 2.0);

    column![
        container(
            text(hex.clone())
                .role(caption, resolved, a11y)
                .color(text_color)
        )
        .padding(swatch_pad)
        .style(move |_theme: &Theme| container::Style {
            background: Some(color.into()),
            border,
            ..Default::default()
        })
        .center_x(Length::Fixed(80.0))
        .center_y(Length::Fixed(32.0)),
        text(label).role(caption, resolved, a11y),
    ]
    .spacing(xxs_spacing)
    .align_x(iced::Center)
    .into()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Swatch rendering parameters shared across extended palette sections.
struct SwatchStyle<'s> {
    border_color: Color,
    border_width: f32,
    radius: f32,
    heading: &'s ResolvedTextScaleEntry,
    caption: &'s ResolvedTextScaleEntry,
    resolved: &'s ResolvedTheme,
    a11y: &'s AccessibilityPreferences,
    xxs_spacing: f32,
    swatch_spacing: f32,
    column_spacing: f32,
}

/// Build an extended palette section wrapped in a hoverable tooltip.
///
/// Combines `widget_tooltip` + `ext_palette_section` + `hoverable` to eliminate
/// repetition in `view_theme_map`. Each call produces a complete hoverable section
/// with all 6 swatches (base/weak/strong x color/text) and a Widget Info tooltip.
fn hoverable_ext_section<'a>(
    label: &'a str,
    base: iced_core::theme::palette::Pair,
    weak: iced_core::theme::palette::Pair,
    strong: iced_core::theme::palette::Pair,
    style: &SwatchStyle<'_>,
) -> Element<'a, Message> {
    let SwatchStyle {
        border_color,
        border_width,
        radius,
        heading,
        caption,
        resolved,
        a11y,
        xxs_spacing,
        swatch_spacing,
        column_spacing,
    } = *style;
    let cs = |field: &'static str, color: Color| -> Element<'_, Message> {
        color_swatch(
            field,
            color,
            iced::Border {
                color: border_color,
                width: border_width,
                radius: radius.into(),
            },
            caption,
            resolved,
            a11y,
            xxs_spacing,
        )
    };
    let info = widget_tooltip(
        label,
        &[
            ("base.color", "base.color", base.color),
            ("base.text", "base.text", base.text),
            ("weak.color", "weak.color", weak.color),
            ("weak.text", "weak.text", weak.text),
            ("strong.color", "strong.color", strong.color),
            ("strong.text", "strong.text", strong.text),
        ],
        &[],
        &[],
    );
    let content: Element<'a, Message> = column![
        text(label).role(heading, resolved, a11y),
        row![
            cs("base.color", base.color),
            cs("base.text", base.text),
            cs("weak.color", weak.color),
            cs("weak.text", weak.text),
            cs("strong.color", strong.color),
            cs("strong.text", strong.text),
        ]
        .spacing(swatch_spacing),
    ]
    .spacing(column_spacing)
    .into();
    hoverable(info, content)
}

/// The drop-down arrow: the connector's open chevron at the platform's own
/// icon size (`native_theme_iced::pick_list_handle`).
///
/// `combo_box.arrow_icon_size` is not a `pick_list::Style` field: iced carries
/// the arrow in the `Handle` the widget is built with
/// (`pick_list.rs:794-801`), so it is set here rather than in
/// `styles::pick_list`.
fn arrow_handle(resolved: &ResolvedTheme) -> pick_list::Handle<iced::Font> {
    native_theme_iced::pick_list_handle::<iced::Renderer>(resolved)
}

/// The role a page's title is set in: `dialog_title`, the dialog or page title
/// (`docs/platform-facts.md` §2.19). Not `display`, the hero text of
/// onboarding and banners, which KDE does not have.
fn page_title(ts: &ResolvedTextScale) -> &ResolvedTextScaleEntry {
    &ts.dialog_title
}

/// The role a section title within a page is set in: `section_heading`, the
/// section divider (`docs/platform-facts.md` §2.19).
fn section_title(ts: &ResolvedTextScale) -> &ResolvedTextScaleEntry {
    &ts.section_heading
}

/// Sets text in a `text_scale` role, in a widget's font, or in the theme's
/// body font: the size, scaled by the user's text-scaling factor, and the
/// font that comes with it, [`theme_font`].
///
/// Every text the showcase draws is sized here or through its widget's own
/// `text_size`/`size` and `font`, and every size is scaled by the same
/// factor: iced fixes its default text size when the renderer is created, so
/// text given no size would stay at iced's default whatever theme is
/// installed, and a factor applied to some text but not the rest would mix
/// two scales on one screen.
trait Typeset {
    fn role(
        self,
        entry: &ResolvedTextScaleEntry,
        resolved: &ResolvedTheme,
        a11y: &AccessibilityPreferences,
    ) -> Self;
    fn typeset(self, font: &ResolvedFontSpec, a11y: &AccessibilityPreferences) -> Self;
    fn body(self, resolved: &ResolvedTheme, a11y: &AccessibilityPreferences) -> Self;
    /// `typeset`, on the theme's line height, `defaults.line_height`.
    fn themed(
        self,
        font: &ResolvedFontSpec,
        resolved: &ResolvedTheme,
        a11y: &AccessibilityPreferences,
    ) -> Self;
}

impl Typeset for iced::widget::Text<'_> {
    fn role(
        self,
        entry: &ResolvedTextScaleEntry,
        resolved: &ResolvedTheme,
        a11y: &AccessibilityPreferences,
    ) -> Self {
        // The role's own line box, as the theme states it, scaled with its size.
        self.size(scaled_text_size(entry.size, a11y))
            .line_height(iced::Pixels(scaled_text_size(entry.line_height, a11y)))
            .font(role_font(entry, resolved))
    }

    fn typeset(self, font: &ResolvedFontSpec, a11y: &AccessibilityPreferences) -> Self {
        self.size(scaled_text_size(font.size, a11y))
            .font(theme_font(font))
    }

    fn body(self, resolved: &ResolvedTheme, a11y: &AccessibilityPreferences) -> Self {
        self.themed(&resolved.defaults.font, resolved, a11y)
    }

    fn themed(
        self,
        font: &ResolvedFontSpec,
        resolved: &ResolvedTheme,
        a11y: &AccessibilityPreferences,
    ) -> Self {
        // The theme's line height, where iced's own is 1.3 times the size
        // (`LineHeight::default`).
        self.typeset(font, a11y)
            .line_height(native_theme_iced::line_height_multiplier(resolved))
    }
}

/// A theme font, its family and weight, as iced can draw it
/// ([`font_from_database`]).
fn theme_font(font: &ResolvedFontSpec) -> iced::Font {
    font_drawn(font).font
}

/// A `text_scale` role's font: the role's weight in the body font's family,
/// which a role does not state for itself.
fn role_font(entry: &ResolvedTextScaleEntry, resolved: &ResolvedTheme) -> iced::Font {
    role_drawn(entry, resolved).font
}

/// The theme's monospace font as iced can draw it.
fn theme_mono_font(resolved: &ResolvedTheme) -> iced::Font {
    mono_drawn(resolved).font
}

/// How a theme font is drawn.
fn font_drawn(font: &ResolvedFontSpec) -> Drawn<'static> {
    font_from_database(resolved_family(font), font.weight, false)
}

/// How the theme's monospace font is drawn.
fn mono_drawn(resolved: &ResolvedTheme) -> Drawn<'static> {
    let mono = &resolved.defaults.mono_font;
    font_from_database(resolved_family(mono), mono.weight, true)
}

/// How a `text_scale` role is drawn: its weight in the body font's family.
fn role_drawn(entry: &ResolvedTextScaleEntry, resolved: &ResolvedTheme) -> Drawn<'static> {
    font_from_database(role_family(entry, resolved), entry.weight, false)
}

/// A Widget Info row for text in a theme font: the field it comes from,
/// and the weight it is drawn at where that is not the weight stated.
fn font_row(field: &str, font: &ResolvedFontSpec) -> String {
    font_row_drawn(field, font, &font_drawn(font))
}

/// [`font_row`], given how the font is drawn.
fn font_row_drawn(field: &str, font: &ResolvedFontSpec, drawn: &Drawn<'_>) -> String {
    let family = family_label(&font.family, resolved_family(font), drawn);
    let weight = weight_label(font.weight, drawn);
    match (family == *font.family, weight == font.weight.to_string()) {
        (true, true) => format!("{field}, family, size and weight"),
        (true, false) => format!("{field}, family and size; weight {weight}"),
        (false, true) => format!("{field}, size and weight; family {family}"),
        (false, false) => format!("{field}, size; family {family}; weight {weight}"),
    }
}

/// A theme family as the showcase shows it: the stated name alone where
/// iced draws text in the family asked for it — the one the font database
/// holds for the face, `.SF NS` for "SF Pro" on macOS — and otherwise the
/// generic family drawn instead and the family that resolves to.
fn family_label(stated: &str, asked: &str, drawn: &Drawn<'_>) -> String {
    let generic = match drawn.font.family {
        iced::font::Family::Name(name) if name == asked => return stated.to_string(),
        iced::font::Family::Monospace => "generic monospace",
        _ => "generic sans-serif",
    };
    match drawn.family {
        Some(resolved) => format!("{stated} (not found; drawn in {generic}: {resolved})"),
        None => format!("{stated} (not found; drawn in {generic})"),
    }
}

/// A theme weight as the showcase shows it: the number alone where iced
/// draws it, and otherwise what is drawn instead and why.
fn weight_label(stated: u16, drawn: &Drawn<'_>) -> String {
    let asked = css_weight(native_theme_iced::to_iced_weight(stated));
    let at = css_weight(drawn.font.weight);
    if at == stated {
        stated.to_string()
    } else if at == asked {
        format!("{stated} (drawn at {at}, iced's nearest weight)")
    } else {
        let family = drawn.family.unwrap_or("the generic family");
        format!("{stated} (drawn at {at}: {family} has no {asked} face)")
    }
}

/// The CSS weight iced asks cosmic-text for at one of its nine weights
/// (`iced_graphics` `text.rs:278-290`).
fn css_weight(weight: iced::font::Weight) -> u16 {
    use cosmic_text::Weight as Css;
    use iced::font::Weight;
    match weight {
        Weight::Thin => Css::THIN.0,
        Weight::ExtraLight => Css::EXTRA_LIGHT.0,
        Weight::Light => Css::LIGHT.0,
        Weight::Normal => Css::NORMAL.0,
        Weight::Medium => Css::MEDIUM.0,
        Weight::Semibold => Css::SEMIBOLD.0,
        Weight::Bold => Css::BOLD.0,
        Weight::ExtraBold => Css::EXTRA_BOLD.0,
        Weight::Black => Css::BLACK.0,
    }
}

/// A font as iced draws it, and the family that draws it where the font
/// database holds one: the theme's, or the one iced's generic family
/// resolves to.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Drawn<'a> {
    font: iced::Font,
    family: Option<&'a str>,
}

/// The fonts built so far, by family, weight and whether monospace, and the
/// family names they hold.
///
/// iced's `Family::Name` takes a `&'static str` (`iced_core` `font.rs:46`),
/// so each family name is leaked once and reused; the showcase meets a few
/// dozen at most. A font is worked out once, the first time a view asks for
/// it, which is at startup and after a theme change for that theme's fonts.
static FONTS: Mutex<FontBook> = Mutex::new(FontBook {
    names: BTreeSet::new(),
    fonts: BTreeMap::new(),
    resolved: BTreeMap::new(),
});

struct FontBook {
    names: BTreeSet<&'static str>,
    fonts: BTreeMap<(&'static str, u16, bool), Drawn<'static>>,
    /// The family the database holds per stated family, weight and style.
    resolved: BTreeMap<(&'static str, u16, u8), &'static str>,
}

impl FontBook {
    fn intern(&mut self, name: &str) -> &'static str {
        match self.names.get(name) {
            Some(&held) => held,
            None => {
                let held: &'static str = Box::leak(name.to_owned().into_boxed_str());
                self.names.insert(held);
                held
            }
        }
    }
}

/// The family iced's font database holds for a theme font, worked out once
/// per family, weight and style — `system_font_family` copies the chosen
/// face's bytes on every call — and interned, as the drawn fonts are.
fn resolved_family(font: &ResolvedFontSpec) -> &'static str {
    let mut book = FONTS.lock().unwrap_or_else(PoisonError::into_inner);
    let stated = book.intern(&font.family);
    let key = (stated, font.weight, style_key(font.style));
    if let Some(&held) = book.resolved.get(&key) {
        return held;
    }
    let resolved = native_theme_iced::system_font_family(font);
    let held = book.intern(&resolved);
    book.resolved.insert(key, held);
    held
}

/// A `FontStyle` as a map key.
fn style_key(style: native_theme::theme::FontStyle) -> u8 {
    match style {
        native_theme::theme::FontStyle::Normal => 0,
        native_theme::theme::FontStyle::Italic => 1,
        native_theme::theme::FontStyle::Oblique => 2,
    }
}

/// The family a `text_scale` role is drawn in: the body font's, at the
/// role's weight.
fn role_family(entry: &ResolvedTextScaleEntry, resolved: &ResolvedTheme) -> &'static str {
    let at_weight = ResolvedFontSpec {
        weight: entry.weight,
        ..resolved.defaults.font.clone()
    };
    resolved_family(&at_weight)
}

/// [`drawable_font`] over the database iced draws text from, and the
/// platform fallback list cosmic-text walks, after [`register_weight`] has
/// filed the weight a variable face of the drawing family covers.
fn font_from_database(family: &str, weight: u16, mono: bool) -> Drawn<'static> {
    let mut book = FONTS.lock().unwrap_or_else(PoisonError::into_inner);
    let name = book.intern(family);
    if let Some(&drawn) = book.fonts.get(&(name, weight, mono)) {
        return drawn;
    }
    let (font, drawing) = {
        let mut system = iced::advanced::graphics::text::font_system()
            .write()
            .unwrap_or_else(PoisonError::into_inner);
        let raw = system.raw();
        let platform = cosmic_text::PlatformFallback;
        let fallbacks = platform.common_fallback();
        let asked = css_weight(native_theme_iced::to_iced_weight(weight));
        // The theme's family first: one the database holds only at weights
        // iced cannot ask for is passed over for the generic family unless
        // a variable face of it covers the weight.
        let filed_theme =
            lacks(raw.db(), name, asked) && register_weight(raw.db_mut(), name, asked, weight_axis);
        let first = drawable_font(raw.db(), fallbacks, name, weight, mono);
        let unmatched = first
            .family
            .filter(|drawing| lacks(raw.db(), drawing, asked))
            .map(str::to_owned);
        let first = (first.font, first.family.map(str::to_owned));
        let filed = unmatched
            .is_some_and(|drawing| register_weight(raw.db_mut(), &drawing, asked, weight_axis));
        if filed_theme || filed {
            let drawn = drawable_font(raw.db(), fallbacks, name, weight, mono);
            (drawn.font, drawn.family.map(str::to_owned))
        } else {
            first
        }
    };
    let drawn = Drawn {
        font,
        family: drawing.map(|drawing| book.intern(&drawing)),
    };
    book.fonts.insert((name, weight, mono), drawn);
    drawn
}

/// The font iced draws `family` at the CSS `weight` in, given the font
/// database and the platform's fallback families: the family where the
/// database holds it, at the weight iced asks for where the family has a
/// face of it, and otherwise at the weight of its nearest face.
///
/// iced 0.14 draws text with cosmic-text 0.15, which takes a face only at
/// the weight asked for (`font/fallback/mod.rs:279-287`, `:299-303`,
/// `:446-456`), and fontdb 0.23 files a face at the one weight its OS/2
/// table states, a variable font too (`lib.rs:1034-1037`, `:1161`). A
/// weight its family has no face of therefore falls through to the next
/// fallback family that has one: on macOS cosmic-text's fallback list puts
/// the system family, `.SF NS`, first and the monospace Menlo second
/// (`font/fallback/macos.rs:30-38`), and bold section titles were drawn in
/// a monospace bold. [`register_weight`] files the weights a variable face
/// covers; where none does, the text stays in its family at the nearest
/// face iced can ask for, one of its nine weights, chosen as fontdb's CSS
/// matching chooses (`lib.rs:1278-1336`).
///
/// iced asks for one of nine weights (`iced_graphics` `text.rs:278-290`),
/// so a weight between them is asked for at the nearest of them.
///
/// A family the database does not hold is drawn in iced's generic family,
/// which cosmic-text resolves to the database's sans-serif family
/// (`font/system.rs:158-160`) or, where the database does not hold that,
/// to the first fallback family it holds; that family decides the weight.
/// A monospace one keeps its weight: cosmic-text takes a monospaced face at
/// any weight, the nearest first (`font/fallback/mod.rs:299-303`,
/// `:375-413`).
fn drawable_font<'a>(
    db: &'a fontdb::Database,
    fallbacks: &[&'a str],
    family: &'static str,
    weight: u16,
    mono: bool,
) -> Drawn<'a> {
    let (drawn, deciding) = if holds(db, family) {
        (iced::font::Family::Name(family), Some(family))
    } else if mono {
        (iced::font::Family::Monospace, None)
    } else {
        let generic = db.family_name(&fontdb::Family::SansSerif);
        let resolved = std::iter::once(generic)
            .chain(fallbacks.iter().copied())
            .find(|name| holds(db, name));
        (iced::font::Family::SansSerif, resolved)
    };
    let asked = native_theme_iced::to_iced_weight(weight);
    let weight = deciding
        .and_then(|name| {
            let faces: Vec<u16> = upright_faces(db, name)
                .map(|face| face.weight.0)
                .filter(|&held| askable(held))
                .collect();
            nearest_weight(css_weight(asked), &faces)
        })
        .map_or(asked, native_theme_iced::to_iced_weight);
    Drawn {
        font: iced::Font {
            family: drawn,
            weight,
            ..iced::Font::DEFAULT
        },
        family: deciding,
    }
}

/// Whether the database holds an upright face of `family` at a weight iced
/// can ask for, one of its nine (`iced_graphics` `text.rs:278-290`).
fn holds(db: &fontdb::Database, family: &str) -> bool {
    upright_faces(db, family).any(|face| askable(face.weight.0))
}

/// Whether iced can ask cosmic-text for the CSS `weight`: whether it is one
/// of iced's nine.
fn askable(weight: u16) -> bool {
    css_weight(native_theme_iced::to_iced_weight(weight)) == weight
}

/// Whether the database holds upright faces of `family`, none of them at
/// the CSS `weight`: whether a face at `weight` is worth filing.
fn lacks(db: &fontdb::Database, family: &str, weight: u16) -> bool {
    let mut faces = upright_faces(db, family).peekable();
    faces.peek().is_some() && faces.all(|face| face.weight.0 != weight)
}

/// The upright faces of `family`: the only ones cosmic-text matches text
/// that asks for no style or stretch against (`attrs.rs:323-327`).
fn upright_faces<'a>(
    db: &'a fontdb::Database,
    family: &'a str,
) -> impl Iterator<Item = &'a fontdb::FaceInfo> + 'a {
    db.faces().filter(move |face| {
        face.style == fontdb::Style::Normal
            && face.stretch == fontdb::Stretch::Normal
            && face.families.iter().any(|(held, _)| held == family)
    })
}

/// The weight CSS font matching takes from `held` for `asked`, as fontdb
/// implements it (`lib.rs:1278-1336`): the weight itself; for 400, 500 and
/// for 500, 400 first; up to 500, the nearest lighter weight, else the
/// nearest heavier; above 500, the nearest heavier, else the nearest
/// lighter.
fn nearest_weight(asked: u16, held: &[u16]) -> Option<u16> {
    let has = |weight: u16| held.contains(&weight);
    let lighter = || held.iter().copied().filter(|&w| w < asked).max();
    let heavier = || held.iter().copied().filter(|&w| w > asked).min();
    let normal = cosmic_text::Weight::NORMAL.0;
    let medium = cosmic_text::Weight::MEDIUM.0;
    if has(asked) {
        Some(asked)
    } else if asked == normal && has(medium) {
        Some(medium)
    } else if asked == medium && has(normal) {
        Some(normal)
    } else if asked <= medium {
        lighter().or_else(heavier)
    } else {
        heavier().or_else(lighter)
    }
}

/// The range of a face's `wght` axis, where its font varies in weight.
fn weight_axis(db: &fontdb::Database, id: fontdb::ID) -> Option<(f32, f32)> {
    use cosmic_text::skrifa::MetadataProvider as _;
    db.with_face_data(id, |data, index| {
        let font = cosmic_text::skrifa::FontRef::from_index(data, index).ok()?;
        let axis = font
            .axes()
            .get_by_tag(cosmic_text::skrifa::Tag::new(b"wght"))?;
        Some((axis.min_value(), axis.max_value()))
    })
    .flatten()
}

/// Files a face of `family` at the CSS `weight` where the family has none
/// but a variable face of it covers that weight on its `wght` axis: a copy
/// of that face's entry at `weight`, over the same font data. Returns
/// whether it filed one.
///
/// cosmic-text 0.15 matches only the weight fontdb filed a face at, but it
/// draws the face it matched at the weight asked for: it shapes with the
/// font's `wght` axis set to it (`font/mod.rs:139-142`) and rasterises each
/// glyph, which carries the text's weight (`shape.rs:231`, `:528`), with
/// the axis set to it too (`swash.rs:20-40`), so the filed face draws the
/// platform's own font at the true weight. cosmic-text 0.19 matches a
/// variable face this way itself (`variable_weight_match`,
/// `font/system.rs:38-44`). `axis` reads a face's `wght` range,
/// [`weight_axis`] outside the tests.
fn register_weight(
    db: &mut fontdb::Database,
    family: &str,
    weight: u16,
    axis: impl Fn(&fontdb::Database, fontdb::ID) -> Option<(f32, f32)>,
) -> bool {
    if upright_faces(db, family).any(|face| face.weight.0 == weight) {
        return false;
    }
    let wanted = f32::from(weight);
    let variable = upright_faces(db, family)
        .find(|face| axis(db, face.id).is_some_and(|(min, max)| min <= wanted && wanted <= max))
        .cloned();
    match variable {
        Some(mut face) => {
            face.weight = fontdb::Weight(weight);
            db.push_face_info(face);
            true
        }
        None => false,
    }
}

fn section_header<'a>(
    title: &'a str,
    description: &'a str,
    resolved: &ResolvedTheme,
    ts: &ResolvedTextScale,
    a11y: &AccessibilityPreferences,
    sp: &Spacing,
) -> Element<'a, Message> {
    column![
        text(title).role(page_title(ts), resolved, a11y),
        text(description).body(resolved, a11y),
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
    ]
    .spacing(sp.xs)
    .into()
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

fn theme(state: &State) -> Theme {
    state.current_theme.clone()
}

// ---------------------------------------------------------------------------
// Subscription (animation tick)
// ---------------------------------------------------------------------------

fn subscription(state: &State) -> Subscription<Message> {
    let mut subs = vec![];

    // Animation tick: the Icons page's animated icons, and the Basic page's
    // spinner, which animates with the time since `animation_start`.
    let icons_animate = state.active_tab == Tab::Icons
        && (!state.animated_frames.is_empty() || !state.animated_spins.is_empty());
    if (icons_animate || state.active_tab == Tab::Basic) && !state.motion_reduced() {
        subs.push(iced::time::every(Duration::from_millis(50)).map(|_| Message::AnimationTick));
    }

    // The key bindings the menus show, and Escape, which closes a dialog.
    subs.push(iced::keyboard::listen().filter_map(shortcut));
    subs.push(iced::window::resize_events().map(|(_, size)| Message::WindowResized(size)));

    // Theme watcher: poll the atomic flag set by on_theme_change() callback.
    // Only active when color mode is System and the watcher started successfully.
    if matches!(state.color_mode, AppColorMode::System) && state._theme_watcher.is_some() {
        subs.push(iced::time::every(Duration::from_millis(500)).map(|_| Message::ThemeWatcherTick));
    }

    // Screenshot countdown timer
    if state.screenshot_path.is_some() && state.screenshot_countdown > 0 {
        subs.push(iced::time::every(Duration::from_millis(50)).map(|_| Message::ScreenshotTick));
    }

    Subscription::batch(subs)
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> iced::Result {
    // Parse CLI args and store globally before the iced application starts.
    // State::default() reads from CLI_ARGS to apply overrides.
    let cli = CliArgs::parse();
    if let Some(value) = &cli.bad_pointer {
        eprintln!("ERROR: --pointer {value:?}: not X,Y in whole logical pixels");
        std::process::exit(1);
    }
    if let Some(menu) = &cli.open_menu
        && menu != OPEN_MENU_THEME
    {
        eprintln!("ERROR: --open-menu {menu:?}: the menu that opens is {OPEN_MENU_THEME:?}");
        std::process::exit(1);
    }
    if let Err(error) = ELEMENTS.get_or_init(showcase_elements) {
        eprintln!("ERROR: docs/showcase-elements.toml: {error}");
        std::process::exit(1);
    }
    let _ = CLI_ARGS.set(cli);
    let capturing = CLI_ARGS
        .get()
        .is_some_and(|cli| cli.capture || cli.screenshot.is_some());

    // iced_aw draws its glyphs (the number input's step arrows) in a font of
    // its own, which the application loads (iced_aw 0.14.1 `src/lib.rs:174`).
    let application = iced::application(boot, update, view)
        .font(iced_aw::ICED_AW_FONT_BYTES)
        .title(|_: &State| WINDOW_TITLE.to_string())
        .theme(theme)
        .subscription(subscription)
        .window_size(WINDOW_SIZE)
        .centered();
    if capturing {
        application.window(capture_window_settings()).run()
    } else {
        application.run()
    }
}

/// The showcase as it starts: its state, and the Basic page's focused input
/// holding the keyboard focus, so its Focused border shows from the first
/// frame.
fn boot() -> (State, iced::Task<Message>) {
    (
        State::default(),
        iced::widget::operation::focus(FOCUSED_INPUT_ID),
    )
}

/// The window settings of a run that is captured (`--screenshot` or
/// `--capture`): `main`'s own, `WINDOW_SIZE` centred, whatever size a desktop
/// stored for the showcase's window. iced itself stores none; a desktop can,
/// by the window's app id (a KWin script that remembers window geometry,
/// say), so on Linux the window takes an app id of this process's own, which
/// nothing stored.
fn capture_window_settings() -> iced::window::Settings {
    iced::window::Settings {
        size: WINDOW_SIZE.into(),
        position: iced::window::Position::Centered,
        #[cfg(target_os = "linux")]
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: format!("showcase-iced-capture-{}", std::process::id()),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// A `--screenshot` that failed: reported on stderr, and the process exits
/// with 1, so a capture step fails.
fn failed_capture(error: &str) -> ! {
    eprintln!("ERROR: screenshot capture failed: {error}");
    std::process::exit(1)
}

/// Whether a frame the showcase captured of its own content, `captured`
/// physical pixels, is `WINDOW_SIZE` at the display's scale factor `scale`.
/// Any other size is a window that did not open at its default size -- on a
/// display too small for it, say.
#[cfg_attr(any(target_os = "macos", target_os = "windows"), allow(dead_code))]
fn check_content_capture(captured: (u32, u32), scale: f32) -> Result<(), String> {
    check_frame_capture(
        (i64::from(captured.0), i64::from(captured.1)),
        (i64::from(captured.0), i64::from(captured.1)),
        f64::from(scale),
    )
}

/// Whether an OS capture of the window with its frame, `captured` pixels, is
/// the frame of a window whose content is `WINDOW_SIZE` at the display's
/// scale factor `scale`: the capture less the content the window has now,
/// `content` pixels, is the frame, and around a `WINDOW_SIZE` content it
/// makes the size the capture must be. Any other size is a window that did
/// not open at its default size.
fn check_frame_capture(
    captured: (i64, i64),
    content: (i64, i64),
    scale: f64,
) -> Result<(), String> {
    let default = (
        (f64::from(WINDOW_SIZE.0) * scale).round() as i64,
        (f64::from(WINDOW_SIZE.1) * scale).round() as i64,
    );
    let frame = (captured.0 - content.0, captured.1 - content.1);
    let expected = (default.0 + frame.0, default.1 + frame.1);
    if captured == expected {
        Ok(())
    } else {
        Err(format!(
            "the capture is {}x{} px, expected {}x{}: a {}x{} content area \
             ({}x{} at scale {scale}) in the frame's {}x{} px, but the content \
             is {}x{} px, so the window did not open at its default size",
            captured.0,
            captured.1,
            expected.0,
            expected.1,
            default.0,
            default.1,
            WINDOW_SIZE.0,
            WINDOW_SIZE.1,
            frame.0,
            frame.1,
            content.0,
            content.1,
        ))
    }
}

// ---------------------------------------------------------------------------
// Self-tests (spec §6.2)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use iced::mouse;
    use iced::{Event, Point, Rectangle, Settings, Size};
    use iced_test::Simulator;
    use iced_test::selector::{self, Candidate};

    /// The macOS system UI font, stated "SF Pro", is a family iced's font
    /// database holds under the name `system_font_family` gives, and the
    /// inspector shows the stated name alone (egui spec §8.8). Runs on the
    /// screenshot workflow's `macos-latest` runner (plan Task 39).
    #[test]
    #[ignore = "needs macOS: run with --ignored on the macos-latest runner"]
    fn macos_system_font_is_in_the_font_database() {
        if !cfg!(target_os = "macos") {
            eprintln!("macos_system_font_is_in_the_font_database: not macOS, nothing to check");
            return;
        }
        let (_, resolved) =
            native_theme_iced::from_preset("macos-sonoma", false).expect("macos-sonoma resolves");
        let font = &resolved.defaults.font;
        let family = native_theme_iced::system_font_family(font);
        let held = {
            let mut system = iced::advanced::graphics::text::font_system()
                .write()
                .unwrap_or_else(PoisonError::into_inner);
            holds(system.raw().db(), &family)
        };
        assert!(
            held,
            "iced's font database holds {family:?} for the stated {:?}",
            font.family
        );
        assert_eq!(
            family_label(&font.family, resolved_family(font), &font_drawn(font)),
            font.family.as_ref(),
            "the inspector shows the stated name alone"
        );
    }

    /// Whether the same holds for the monospace font, "SF Mono": printed,
    /// not asserted — UNVERIFIED (egui spec §15) whether any font database
    /// holds that name, so the outcome is recorded, not required.
    #[test]
    #[ignore = "needs macOS: run with --ignored on the macos-latest runner"]
    fn macos_mono_font_in_the_font_database() {
        if !cfg!(target_os = "macos") {
            eprintln!("macos_mono_font_in_the_font_database: not macOS, nothing to check");
            return;
        }
        let (_, resolved) =
            native_theme_iced::from_preset("macos-sonoma", false).expect("macos-sonoma resolves");
        let mono = &resolved.defaults.mono_font;
        let family = native_theme_iced::system_font_family(mono);
        let held = {
            let mut system = iced::advanced::graphics::text::font_system()
                .write()
                .unwrap_or_else(PoisonError::into_inner);
            holds(system.raw().db(), &family)
        };
        println!(
            "SF Mono: stated {:?}, system_font_family gives {family:?}, in iced's font database: {held}",
            mono.family
        );
    }

    /// The viewport the interaction test lays the interface out in.
    ///
    /// Not the window `main` opens: an element scrolled out of the content
    /// `scrollable`'s viewport has no visible bounds and cannot be clicked,
    /// and the tab strip scrolls sideways once the tabs are wider than the
    /// panel. `every_tab_renders` uses the real [`WINDOW_SIZE`] instead, which
    /// is what makes the scroll containers part of what it renders.
    const TALL_WINDOW: Size = Size::new(1400.0, 6000.0);

    /// The real interface, in [`TALL_WINDOW`].
    ///
    /// `Settings::default()` leaves the font as `Font::DEFAULT`, which the
    /// simulator resolves to the Fira Sans it bundles
    /// (`iced_test` `simulator.rs:67-70`), so text measures the same on every
    /// host.
    fn interface(state: &State) -> Simulator<'_, Message> {
        Simulator::with_size(Settings::default(), TALL_WINDOW, view(state))
    }

    /// Builds the real interface, runs `interaction` on it, then feeds every
    /// message the interaction produced through the real `update`.
    ///
    /// Every message reaches `update`; the ones handed back leave the Widget
    /// Info panel's out. Pointing the cursor at a widget is what that panel
    /// listens for, so any click no widget captures — a disabled button's,
    /// say — also reports the hover, which says nothing about the control
    /// under test.
    fn drive(
        state: &mut State,
        interaction: impl FnOnce(&mut Simulator<'_, Message>),
    ) -> Vec<Message> {
        let messages: Vec<Message> = {
            let mut ui = interface(state);
            interaction(&mut ui);
            ui.into_messages().collect()
        };
        for message in messages.iter().cloned() {
            let _ = update(state, message);
        }
        messages
            .into_iter()
            .filter(|message| {
                !matches!(
                    message,
                    Message::WidgetHovered(_)
                        | Message::WidgetUnhovered
                        | Message::ElementHovered(_)
                )
            })
            .collect()
    }

    /// Clicks the widget whose own text is `label`.
    fn click_text(ui: &mut Simulator<'_, Message>, label: &str) {
        if let Err(error) = ui.click(label) {
            panic!("{label}: {error}");
        }
    }

    /// Clicks the last text reading `label`, in the order the interface lays
    /// its widgets out: the page's, where the chrome reads it too.
    fn click_last_text(ui: &mut Simulator<'_, Message>, label: &str) {
        let at = texts(ui)
            .into_iter()
            .filter(|(content, _)| content == label)
            .map(|(_, bounds)| bounds.center())
            .next_back();
        assert!(at.is_some(), "{label}: no such text");
        let Some(at) = at else {
            return;
        };
        ui.point_at(at);
        let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
        let _ = ui.simulate(iced_test::simulator::click());
    }

    /// Clicks the widget tagged with the given [`probe`] id.
    fn click_probe(ui: &mut Simulator<'_, Message>, id: &'static str) {
        if let Err(error) = ui.click(selector::id(id)) {
            panic!("{id}: {error}");
        }
    }

    /// Opens the `pick_list` tagged `id` and chooses its `nth` option.
    ///
    /// A `pick_list` pads its menu with `button::DEFAULT_PADDING`
    /// (`pick_list.rs:204`); `text_size` is the one the showcase gives it.
    fn pick_list_option(
        ui: &mut Simulator<'_, Message>,
        id: &'static str,
        nth: usize,
        text_size: f32,
    ) {
        pick_option(
            ui,
            id,
            nth,
            text_size,
            iced::widget::button::DEFAULT_PADDING,
        );
    }

    /// Opens the `combo_box` tagged `id` and chooses its `nth` option.
    ///
    /// A `combo_box` pads its menu with `text_input::DEFAULT_PADDING`
    /// (`combo_box.rs:190`), which is as tall as a `pick_list`'s today but is
    /// not the same constant and need not stay equal; `text_size` is the one
    /// the showcase gives it.
    fn combo_box_option(
        ui: &mut Simulator<'_, Message>,
        id: &'static str,
        nth: usize,
        text_size: f32,
    ) {
        pick_option(
            ui,
            id,
            nth,
            text_size,
            iced::widget::text_input::DEFAULT_PADDING,
        );
    }

    /// Opens the picker tagged `id` and chooses its `nth` option, counting
    /// from the top of the menu.
    ///
    /// The menu is an overlay whose list implements no `Widget::operate`
    /// (`overlay/menu.rs`), so no selector reaches its rows. What iced does
    /// state publicly is where they are: the menu is laid out directly under
    /// the widget that opened it (`overlay/menu.rs:241-262`), one row per
    /// option, each `line_height + padding.y()` tall
    /// (`overlay/menu.rs:375-397`), where the padding is the one the picker
    /// gives it and the line is the picker's text size: the theme's
    /// `combo_box.font.size` times the text-scaling factor, which the showcase
    /// gives every picker. The
    /// padding is iced's own default, which the showcase overrides for none of
    /// its pickers, read rather than copied as a number.
    fn pick_option(
        ui: &mut Simulator<'_, Message>,
        id: &'static str,
        nth: usize,
        text_size: f32,
        menu_padding: Padding,
    ) {
        let row = line_of(text_size) + menu_padding.y();
        let field = probe_bounds(ui, id);
        let at = Point::new(
            field.center().x,
            field.y + field.height + row * (nth as f32 + 0.5),
        );

        click_probe(ui, id);
        ui.point_at(at);
        let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
        let _ = ui.simulate(iced_test::simulator::click());
    }

    /// The bounds of the widget tagged with the given [`probe`] id.
    fn probe_bounds(ui: &mut Simulator<'_, Message>, id: &'static str) -> Rectangle {
        match ui.find(selector::id(id)) {
            Ok(target) => match target.visible_bounds() {
                Some(bounds) => bounds,
                None => panic!("{id}: found, but not visible"),
            },
            Err(error) => panic!("{id}: {error}"),
        }
    }

    /// Clicks `label`, and asserts the one message it produced.
    fn press(state: &mut State, label: &'static str, expected: &str) {
        let messages = drive(state, |ui| click_text(ui, label));
        let printed: Vec<String> = messages.iter().map(|m| format!("{m:?}")).collect();
        assert_eq!(
            printed,
            vec![expected.to_string()],
            "{label}: the click produced the wrong messages"
        );
    }

    // -----------------------------------------------------------------------
    // every_tab_renders
    // -----------------------------------------------------------------------

    /// Every tab lays out and draws, at the size the application opens.
    ///
    /// What taking the snapshot catches is a panic in `view`, in layout or in
    /// drawing. It has no failure path of its own: `Simulator::snapshot` in
    /// `iced_test` 0.14.0 always returns `Ok` (`simulator.rs:199-241`), so the
    /// `Err` arm below is unreachable today and is written out rather than
    /// discarded so that the day it stops being so does not pass unnoticed.
    ///
    /// The snapshot is deliberately not compared with a baseline — that is
    /// Layer 4 and out of scope (spec §6.2). What is compared is that no text
    /// the tab lays out was squeezed to nothing: a widget with no room left is
    /// a widget nobody sees, which a drawing pass that merely returns `Ok`
    /// would not report.
    #[test]
    fn every_tab_renders() {
        for tab in Tab::ALL {
            let state = State {
                active_tab: *tab,
                ..State::default()
            };
            let theme = theme(&state);

            let mut ui: Simulator<'_, Message> =
                Simulator::with_size(Settings::default(), WINDOW_SIZE, view(&state));

            match ui.snapshot(&theme) {
                Ok(_) => {}
                Err(error) => panic!("{}: the interface did not draw: {error}", tab.label()),
            }

            let mut starved: Vec<String> = Vec::new();
            {
                let sink = &mut starved;
                let _ = ui.find(move |candidate: Candidate<'_>| -> Option<()> {
                    if let Candidate::Text {
                        content, bounds, ..
                    } = candidate
                        && (bounds.width <= 0.0 || bounds.height <= 0.0)
                    {
                        sink.push(format!("{content:?} at {bounds:?}"));
                    }
                    None
                });
            }
            assert!(
                starved.is_empty(),
                "{}: laid out with no room to draw: {starved:?}",
                tab.label()
            );
        }
    }

    // -----------------------------------------------------------------------
    // the_basic_radio_draws_the_themes_dot
    // -----------------------------------------------------------------------

    /// The Basic page's selected radio, Option A, is drawn by
    /// `native_theme_iced::radio`: under kde-breeze its dot is
    /// `checkbox.radio_dot_diameter`, 6px (platform-facts.md:1220), centred
    /// in the circle, in the column of Option B's circle and above it.
    #[test]
    fn the_basic_radio_draws_the_themes_dot() {
        let mut state = State::default();
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset("kde-breeze".to_string())),
        );
        assert_eq!(state.basic_radio, 0, "Option A is the selected one");
        let c = state.current_resolved.checkbox.clone();
        assert_eq!(c.radio_dot_diameter, Some(6.0));
        let mut ui = interface(&state);
        let option_b = probe_bounds(&mut ui, probes::BASIC_RADIO_B);
        let found = ui.find(|candidate: Candidate<'_>| -> Option<Rectangle> {
            match candidate {
                Candidate::Container {
                    visible_bounds: Some(bounds),
                    ..
                } if bounds.width == 6.0 && bounds.height == 6.0 => Some(bounds),
                _ => None,
            }
        });
        let dot = match found {
            Ok(bounds) => bounds,
            Err(error) => panic!("no 6px dot: {error}"),
        };
        let column = option_b.x + c.indicator_width / 2.0;
        assert_eq!(
            dot.center().x,
            column,
            "the dot is not in the circles' column"
        );
        assert!(dot.y < option_b.y, "the dot is not Option A's");
    }

    // -----------------------------------------------------------------------
    // interactive_controls_respond
    // -----------------------------------------------------------------------

    /// Every control the showcase advertises answers a click, and the answer
    /// reaches the state through the real `update`.
    #[test]
    fn interactive_controls_respond() {
        let mut state = State::default();

        // ---- the tab strip ----
        assert_eq!(state.active_tab, Tab::Basic, "the showcase opens on Basic");

        // ---- Basic tab: its button, its held checkbox and its radio ----
        press(&mut state, "Primary", "ButtonPressed");
        assert_eq!(state.button_press_count, 1, "Primary: the press was lost");
        // The first "Disabled" in the tab is its disabled button.
        let messages = drive(&mut state, |ui| click_text(ui, "Disabled"));
        assert!(
            messages.is_empty(),
            "Disabled: a button with no on_press must stay silent, got {messages:?}"
        );
        press(&mut state, "Checked", "BasicHeld");
        let messages = drive(&mut state, |ui| click_probe(ui, probes::BASIC_RADIO_B));
        assert!(
            matches!(messages.as_slice(), [Message::BasicRadioSelected(1)]),
            "Option B: {messages:?}"
        );
        assert_eq!(state.basic_radio, 1, "Option B: the choice was lost");
        let _ = update(&mut state, Message::TabSelected(Tab::Buttons));

        // ---- Buttons tab: one message per class, and none from a disabled one ----
        for (label, count) in [
            ("Primary", 2),
            ("Secondary", 3),
            ("Success", 4),
            ("Warning", 5),
            ("Danger", 6),
            ("Text Style", 7),
            ("Click me!", 8),
        ] {
            press(&mut state, label, "ButtonPressed");
            assert_eq!(
                state.button_press_count, count,
                "{label}: the press did not reach the counter"
            );
        }
        // Spec §6.2 words this as "its click fails". That is not what
        // `iced_test` does: `click` errs only when the selector finds nothing
        // (`simulator.rs:121-128`) or the target has no visible bounds
        // (`:151-154`), and a disabled button is both found and visible. The
        // click resolves; what says the button is disabled is that it
        // published nothing and the counter did not move, which is the
        // stronger claim.
        for label in ["Disabled Primary", "Disabled Secondary", "Disabled Danger"] {
            let messages = drive(&mut state, |ui| click_text(ui, label));
            assert!(
                messages.is_empty(),
                "{label}: a button with no on_press must stay silent, got {messages:?}"
            );
            assert_eq!(state.button_press_count, 8, "{label}: the counter moved");
        }

        // ---- the tab strip carries the view to the next tab ----
        let messages = drive(&mut state, |ui| click_text(ui, "Text Inputs"));
        assert!(
            matches!(messages.as_slice(), [Message::TabSelected(Tab::TextInputs)]),
            "tab strip: {messages:?}"
        );
        assert_eq!(
            state.active_tab,
            Tab::TextInputs,
            "the tab strip did not switch tabs"
        );

        // ---- TextInput: click to focus, then type ----
        let messages = drive(&mut state, |ui| {
            click_probe(ui, TEXT_INPUT_ID);
            let _ = ui.typewrite("q");
        });
        assert!(
            matches!(messages.as_slice(), [Message::TextInputChanged(value)] if value == "q"),
            "text_input: {messages:?}"
        );
        assert_eq!(
            state.text_input_value, "q",
            "text_input: the value did not reach the state"
        );

        // ---- TextEditor: the same, through its own Content ----
        let before = state.text_editor_content.text();
        let messages = drive(&mut state, |ui| {
            click_probe(ui, probes::TEXT_EDITOR);
            let _ = ui.typewrite("q");
        });
        assert!(
            messages
                .iter()
                .any(|m| matches!(m, Message::EditorAction(_))),
            "text_editor: {messages:?}"
        );
        assert_ne!(
            state.text_editor_content.text(),
            before,
            "text_editor: the action did not reach the document"
        );

        // ---- Selection tab ----
        let _ = drive(&mut state, |ui| click_text(ui, "Selection"));
        assert_eq!(state.active_tab, Tab::Selection);

        assert!(state.checkbox_a, "the first checkbox starts checked");
        let messages = drive(&mut state, |ui| click_text(ui, "Enable notifications"));
        assert!(
            matches!(messages.as_slice(), [Message::CheckboxAToggled(false)]),
            "checkbox: {messages:?}"
        );
        assert!(
            !state.checkbox_a,
            "checkbox: the toggle did not reach the state"
        );

        assert_eq!(
            state.selected_fruit,
            Some(Fruit::Apple),
            "the radios start on Apple"
        );
        let messages = drive(&mut state, |ui| click_probe(ui, probes::RADIO_BANANA));
        assert!(
            matches!(messages.as_slice(), [Message::FruitSelected(Fruit::Banana)]),
            "radio: {messages:?}"
        );
        assert_eq!(
            state.selected_fruit,
            Some(Fruit::Banana),
            "radio: the selection did not reach the state"
        );

        assert!(!state.toggler_enabled, "the toggler starts off");
        let messages = drive(&mut state, |ui| click_probe(ui, probes::TOGGLER));
        assert!(
            matches!(messages.as_slice(), [Message::TogglerToggled(true)]),
            "toggler: {messages:?}"
        );
        assert!(
            state.toggler_enabled,
            "toggler: the toggle did not reach the state"
        );

        // The menu is an overlay: it exists only while the same interface is
        // open, so the click that opens it and the click that picks an entry
        // share one simulator.
        assert_eq!(state.pick_list_selected.as_deref(), Some("Rust"));
        let size = scaled_text_size(
            state.current_resolved.combo_box.font.size,
            &state.accessibility,
        );
        let messages = drive(&mut state, |ui| {
            pick_list_option(ui, probes::PICK_LIST, 1, size)
        });
        let picked = match messages.as_slice() {
            [Message::PickListSelected(value)] => value.clone(),
            other => panic!("pick_list: {other:?}"),
        };
        assert_ne!(picked, "Rust", "pick_list: the second row is the first one");
        assert_eq!(
            state.pick_list_selected.as_deref(),
            Some(picked.as_str()),
            "pick_list: the choice did not reach the state"
        );

        assert_eq!(state.combo_selected, None);
        let size = scaled_text_size(
            state.current_resolved.combo_box.font.size,
            &state.accessibility,
        );
        let messages = drive(&mut state, |ui| {
            combo_box_option(ui, probes::COMBO_BOX, 1, size)
        });
        let picked = match messages.as_slice() {
            [Message::ComboBoxSelected(value)] => value.clone(),
            other => panic!("combo_box: {other:?}"),
        };
        assert_eq!(
            state.combo_selected.as_deref(),
            Some(picked.as_str()),
            "combo_box: the choice did not reach the state"
        );

        // ---- Range tab ----
        let _ = drive(&mut state, |ui| click_text(ui, "Range"));
        assert_eq!(state.active_tab, Tab::Range);

        let before = state.slider_value;
        let messages = drive(&mut state, |ui| click_probe(ui, probes::SLIDER));
        let picked = match messages.as_slice() {
            [Message::SliderChanged(value)] => *value,
            other => panic!("slider: {other:?}"),
        };
        assert_ne!(
            picked, before,
            "slider: the click landed on the current value"
        );
        assert_eq!(
            state.slider_value, picked,
            "slider: the value did not reach the state"
        );

        // The middle of the vertical rail is the value the slider already
        // holds, and a slider publishes nothing when the value does not move
        // (`vertical_slider.rs`, `change`), so the click lands a quarter of
        // the way down the rail instead of at its centre.
        let before = state.vslider_value;
        let messages = drive(&mut state, |ui| {
            let rail = probe_bounds(ui, probes::VERTICAL_SLIDER);
            let quarter = rail.y + rail.height / 4.0;
            ui.point_at(Point::new(rail.center().x, quarter));
            let _ = ui.simulate(iced_test::simulator::click());
        });
        let picked = match messages.as_slice() {
            [Message::VSliderChanged(value)] => *value,
            other => panic!("vertical_slider: {other:?}"),
        };
        assert_ne!(
            picked, before,
            "vertical_slider: the click landed on the current value"
        );
        assert_eq!(
            state.vslider_value, picked,
            "vertical_slider: the value did not reach the state"
        );

        // ---- Layout tab: the pane grid splits and closes ----
        let _ = drive(&mut state, |ui| click_text(ui, "Layout"));
        assert_eq!(state.active_tab, Tab::Layout);

        // A click inside a pane focuses it as well as pressing the button
        // under the cursor, so both messages are expected here.
        let panes_before = state.panes.iter().count();
        let messages = drive(&mut state, |ui| click_text(ui, "split |"));
        assert!(
            messages
                .iter()
                .any(|message| matches!(message, Message::PaneSplit(pane_grid::Axis::Vertical, _))),
            "pane_grid split: {messages:?}"
        );
        assert_eq!(
            state.panes.iter().count(),
            panes_before + 1,
            "pane_grid: the split did not add a pane"
        );

        let panes_before = state.panes.iter().count();
        let messages = drive(&mut state, |ui| click_text(ui, "close"));
        assert!(
            messages
                .iter()
                .any(|message| matches!(message, Message::PaneClosed(_))),
            "pane_grid close: {messages:?}"
        );
        assert_eq!(
            state.panes.iter().count(),
            panes_before - 1,
            "pane_grid: the close did not remove a pane"
        );

        // ---- Graphics tab: the Markdown link ----
        let _ = drive(&mut state, |ui| click_text(ui, "Graphics"));
        assert_eq!(state.active_tab, Tab::Graphics);

        let (nth_item, url) = markdown_link();
        assert_eq!(state.markdown_link, None, "nothing has been clicked yet");
        let messages = drive(&mut state, |ui| {
            let bounds = markdown_item_bounds(ui, nth_item);
            ui.point_at(bounds.center());
            let _ = ui.simulate(iced_test::simulator::click());
        });
        assert!(
            matches!(messages.as_slice(), [Message::MarkdownLinkClicked(uri)] if uri == url),
            "markdown link: {messages:?}"
        );
        assert_eq!(
            state.markdown_link.as_deref(),
            Some(url),
            "markdown link: the uri did not reach the state"
        );

        #[cfg(feature = "iced_aw")]
        aw_controls_respond(&mut state);

        // ---- The sidebar selectors, last: they replace the theme ----
        theme_selectors_respond(&mut state);
    }

    /// The `iced_aw` widgets of the Extra tab.
    #[cfg(feature = "iced_aw")]
    fn aw_controls_respond(state: &mut State) {
        let _ = drive(state, |ui| click_text(ui, "Extra widgets (iced_aw)"));
        assert_eq!(state.active_tab, Tab::Extra);

        // TabBar (stand-alone).
        assert_eq!(state.aw_tab_bar_active, 0);
        let messages = drive(state, |ui| click_text(ui, "Details"));
        assert!(
            matches!(messages.as_slice(), [Message::AwTabBarSelected(1)]),
            "TabBar: {messages:?}"
        );
        assert_eq!(
            state.aw_tab_bar_active, 1,
            "TabBar: the selection did not reach the state"
        );

        // Sidebar.
        assert_eq!(state.aw_sidebar_active, 0);
        let messages = drive(state, |ui| click_text(ui, "Appearance"));
        assert!(
            matches!(messages.as_slice(), [Message::AwSidebarSelected(1)]),
            "Sidebar: {messages:?}"
        );
        assert_eq!(
            state.aw_sidebar_active, 1,
            "Sidebar: the selection did not reach the state"
        );

        // MenuBar: a root item is one of our own buttons. The window's own
        // menu bar has a File too, above the page: the page's is the last.
        let messages = drive(state, |ui| click_last_text(ui, "File"));
        assert!(
            matches!(messages.as_slice(), [Message::AwActionChosen(what)] if what == "Menu: File"),
            "MenuBar: {messages:?}"
        );
        assert_eq!(
            state.aw_last_action, "Menu: File",
            "MenuBar: the choice did not reach the state"
        );

        // ContextMenu: it opens on the right button only, and its popup is
        // placed at the cursor (`overlay/context_menu.rs:84-105`). The entry
        // is then clicked without looking it up: while the menu is open,
        // `ContextMenu::operate` hands the popup the *underlay's* layout
        // (`widget/context_menu.rs:186-192`), and every button in it panics on
        // a layout node with no children (`button.rs:265-268`), so any
        // selector run over that tab would take the whole test down. Clicking
        // needs no selector -- `simulate` never calls `operate` -- and the
        // point is the popup's own padding plus its first entry's.
        let messages = drive(state, |ui| {
            let underlay = match ui.find("Right-click inside this panel") {
                Ok(target) => match target.visible_bounds() {
                    Some(bounds) => bounds,
                    None => panic!("ContextMenu: the underlay is not visible"),
                },
                Err(error) => panic!("ContextMenu: {error}"),
            };
            let at = underlay.center();
            ui.point_at(at);
            let _ = ui.simulate([
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)),
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)),
            ]);
            let entry = Point::new(at.x + SP.xs + SP.s, at.y + SP.xs + SP.xxs);
            ui.point_at(entry);
            let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: entry })]);
            let _ = ui.simulate(iced_test::simulator::click());
        });
        assert!(
            matches!(messages.as_slice(), [Message::AwActionChosen(what)] if what == "Context menu: Copy"),
            "ContextMenu: {messages:?}"
        );
        assert_eq!(
            state.aw_last_action, "Context menu: Copy",
            "ContextMenu: the choice did not reach the state"
        );

        // SelectionList: `iced_aw` reports its rows in the list's own
        // coordinates and with no visible bounds
        // (`selection_list/list.rs:281-310`), so the click point is the
        // list's own origin plus the row's offset inside it.
        assert_eq!(
            state.aw_list_selected,
            Some(0),
            "the list starts on its first row"
        );
        let messages = drive(state, |ui| {
            let list = probe_bounds(ui, probes::SELECTION_LIST);
            let row = match ui.find("Breeze") {
                Ok(target) => target.bounds(),
                Err(error) => panic!("SelectionList: {error}"),
            };
            ui.point_at(Point::new(list.x + row.center().x, list.y + row.center().y));
            let _ = ui.simulate(iced_test::simulator::click());
        });
        assert!(
            matches!(messages.as_slice(), [Message::AwListSelected(1, value)] if value == "Breeze"),
            "SelectionList: {messages:?}"
        );
        assert_eq!(
            state.aw_list_selected,
            Some(1),
            "SelectionList: the row did not reach the state"
        );

        // Card: its foot button closes it, and the button that replaces it
        // brings it back.
        assert!(state.aw_card_open, "the card starts open");
        let messages = drive(state, |ui| click_text(ui, "Dismiss"));
        assert!(
            matches!(messages.as_slice(), [Message::AwCardToggled]),
            "Card close: {messages:?}"
        );
        assert!(
            !state.aw_card_open,
            "Card: the close did not reach the state"
        );

        let messages = drive(state, |ui| click_text(ui, "Show the card again"));
        assert!(
            matches!(messages.as_slice(), [Message::AwCardToggled]),
            "Card reopen: {messages:?}"
        );
        assert!(
            state.aw_card_open,
            "Card: the reopen did not reach the state"
        );
    }

    /// The two sidebar selectors, asserted against what the state held before
    /// the click rather than against this host's desktop.
    fn theme_selectors_respond(state: &mut State) {
        // Colour mode. `AppColorMode::System` resolves to whichever mode this
        // desktop is in, so the test asks for the opposite of what is on
        // screen and checks that both the flag and the installed palette
        // followed.
        let was_dark = state.is_dark;
        let before = theme(state).extended_palette().background.base.color;
        let wanted = if was_dark {
            AppColorMode::Light
        } else {
            AppColorMode::Dark
        };
        let nth = match AppColorMode::ALL.iter().position(|mode| *mode == wanted) {
            Some(index) => index,
            None => panic!("{wanted} is not offered by the colour mode picker"),
        };
        let size = scaled_text_size(
            state.current_resolved.combo_box.font.size,
            &state.accessibility,
        );
        let messages = drive(state, |ui| {
            pick_list_option(ui, probes::COLOR_MODE, nth, size)
        });
        assert!(
            matches!(messages.as_slice(), [Message::ColorModeSelected(mode)] if *mode == wanted),
            "colour mode: {messages:?}"
        );
        assert_eq!(
            state.is_dark, !was_dark,
            "colour mode: the mode did not change"
        );
        assert_ne!(
            theme(state).extended_palette().background.base.color,
            before,
            "colour mode: the installed theme did not follow the mode"
        );

        // Theme preset. The entry is the first preset this platform offers
        // that would put a different background on screen, so the assertion
        // is about the installed theme moving and not about which desktop
        // this runs on.
        let mode = if state.is_dark {
            native_theme_iced::ColorMode::Dark
        } else {
            native_theme_iced::ColorMode::Light
        };
        let before = theme(state).extended_palette().background.base.color;
        let chosen = native_theme::theme::Theme::list_presets_for_platform()
            .iter()
            .enumerate()
            .find_map(|(index, info)| {
                let native = native_theme::theme::Theme::preset(info.key).ok()?;
                let resolved = native.resolve(mode).ok()?;
                let built = native_theme_iced::to_theme(&resolved.variant, &native.name);
                (built.extended_palette().background.base.color != before)
                    .then(|| (index + 1, info.key.to_string(), built))
            });
        let (nth, preset, expected) = match chosen {
            Some(chosen) => chosen,
            None => panic!("no preset of this platform resolves to another background"),
        };

        let size = scaled_text_size(
            state.current_resolved.combo_box.font.size,
            &state.accessibility,
        );
        let messages = drive(state, |ui| pick_list_option(ui, probes::THEME, nth, size));
        assert!(
            matches!(messages.as_slice(), [Message::ThemeSelected(ThemeChoice::Preset(name))] if *name == preset),
            "theme preset: {messages:?}"
        );
        assert_eq!(
            state.current_choice,
            ThemeChoice::Preset(preset.clone()),
            "theme preset: the choice did not reach the state"
        );
        assert!(
            state.error_message.is_none(),
            "theme preset: {:?}",
            state.error_message
        );
        assert_eq!(
            theme(state).to_string(),
            expected.to_string(),
            "theme preset: another theme is installed"
        );
        assert_eq!(
            theme(state).extended_palette().background.base.color,
            expected.extended_palette().background.base.color,
            "theme preset: the installed theme did not follow the choice"
        );
    }

    // -----------------------------------------------------------------------
    // Theme installs and the command line
    // -----------------------------------------------------------------------

    /// The colour mode the state is not drawn in: installing in it changes
    /// what is on screen, so an install that failed and kept it is caught.
    fn other_mode(state: &State) -> AppColorMode {
        if state.is_dark {
            AppColorMode::Light
        } else {
            AppColorMode::Dark
        }
    }

    /// What the theme and colour-mode pickers show, and what is drawn.
    fn shown(state: &State) -> (ThemeChoice, AppColorMode, bool, String, Color) {
        let drawn = theme(state);
        (
            state.current_choice.clone(),
            state.color_mode,
            state.is_dark,
            drawn.to_string(),
            drawn.extended_palette().background.base.color,
        )
    }

    /// The command line of `args`, each flag with the value `main` would
    /// have parsed for it.
    fn command_line(args: &[(&str, &str)]) -> CliArgs {
        let mut cli = CliArgs::default();
        for (flag, value) in args {
            let value = Some(value.to_string());
            match *flag {
                "--theme" => cli.theme = value,
                "--variant" => cli.variant = value,
                "--icon-set" => cli.icon_set = value,
                other => panic!("the tests do not pass {other}"),
            }
        }
        cli
    }

    /// The icon choice a preset's install derives where the choice follows
    /// the preset (`default_icon_choice`).
    fn preset_icon_choice(key: &str, is_dark: bool) -> Option<IconSetChoice> {
        let mode = if is_dark {
            native_theme_iced::ColorMode::Dark
        } else {
            native_theme_iced::ColorMode::Light
        };
        let resolved = native_theme::theme::Theme::preset(key)
            .ok()?
            .resolve(mode)
            .ok()?;
        let icon_theme = resolved.icon_theme.map(Cow::into_owned);
        Some(default_icon_choice(
            resolved.icon_set,
            icon_theme
                .as_deref()
                .filter(|_| resolved.icon_theme_explicit),
        ))
    }

    /// A theme that fails to load leaves the installed one on screen, and
    /// the theme and colour-mode pickers showing it and the mode it is drawn
    /// in, never a choice nothing on screen follows.
    #[test]
    fn a_failed_install_keeps_the_theme_and_mode_shown() {
        let mut state = State::default();
        let before = shown(&state);
        let failing = ThemeChoice::Preset("no-such-preset".to_string());

        let mode = other_mode(&state);
        assert!(
            !state.install_in_mode(failing.clone(), mode),
            "no-such-preset installed"
        );
        assert_eq!(
            shown(&state),
            before,
            "a failed install in {mode} changed what the pickers show or what is drawn"
        );
        assert!(
            state.error_message.is_some(),
            "a failed install reported nothing"
        );

        let _ = update(&mut state, Message::ThemeSelected(failing));
        assert_eq!(
            shown(&state),
            before,
            "the theme picker shows a theme that failed to load"
        );
    }

    /// A system icon theme that cannot be detected is shown as such: the
    /// Icons page gives the reason where it would name the theme, never a
    /// theme nothing detected.
    #[test]
    fn a_failed_icon_theme_detection_is_shown_as_a_failure() {
        let state = State {
            detect_icon_theme: || {
                Err(native_theme::error::Error::PlatformUnsupported {
                    platform: "the test's failing detection",
                })
            },
            ..State::default()
        };
        assert_eq!(
            state.system_icon_theme_label(),
            "System icon theme: unavailable (platform not supported: the test's failing detection)"
        );
    }

    /// After startup, an OS theme that fails to read is a failed install
    /// like any other: the installed theme stays drawn and named, never
    /// adwaita under the `default` entry. Only at startup, with nothing
    /// installed yet, is adwaita the fallback.
    #[test]
    fn a_failed_os_theme_read_keeps_the_installed_theme() {
        let mut state = State::default();
        let preset = match native_theme::theme::Theme::list_presets_for_platform().first() {
            Some(info) => info.key,
            None => panic!("this platform offers no preset"),
        };
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset(preset.to_string())),
        );
        state.read_system_theme = || {
            Err(native_theme::error::Error::PlatformUnsupported {
                platform: "the test's failing read",
            })
        };
        let before = shown(&state);

        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::OsTheme(String::new())),
        );
        assert_eq!(
            shown(&state),
            before,
            "a failed OS-theme read replaced {preset}"
        );
        assert!(
            state.error_message.is_some(),
            "a failed OS-theme read reported nothing"
        );
    }

    /// An OS theme that fails to read at startup leaves adwaita drawn, and
    /// the theme picker naming adwaita, with the failed read in the banner:
    /// a colour-mode change, from the picker or `--variant`, and a rebuild
    /// by the theme watcher install adwaita again, in the mode chosen.
    #[test]
    fn the_startup_fallback_is_adwaita_in_every_mode() {
        fn failing_read() -> native_theme::Result<native_theme::SystemTheme> {
            Err(native_theme::error::Error::PlatformUnsupported {
                platform: "the test's failing read",
            })
        }
        fn installed(state: &State) -> (ThemeChoice, AppColorMode, Option<String>) {
            (
                state.current_choice.clone(),
                state.color_mode,
                state.error_message.clone(),
            )
        }
        let adwaita = ThemeChoice::Preset("adwaita".to_string());
        let mut state = State::starting_with(failing_read);
        assert_eq!(state.current_choice, adwaita, "the picker at startup");
        assert!(
            state.error_message.is_some(),
            "the failed OS-theme read at startup reported nothing"
        );
        let preset = native_theme::theme::Theme::preset("adwaita")
            .and_then(|t| t.resolve(native_theme_iced::ColorMode::Light))
            .map(|r| (r.icon_set, r.icon_theme.map(Cow::into_owned)))
            .ok();
        assert_eq!(
            Some((state.current_icon_set, state.current_icon_theme.clone())),
            preset,
            "the startup fallback's icons are not the adwaita preset's"
        );

        let (other, other_name) = if state.is_dark {
            (AppColorMode::Light, "light")
        } else {
            (AppColorMode::Dark, "dark")
        };
        let _ = update(&mut state, Message::ColorModeSelected(other));
        assert_eq!(
            installed(&state),
            (adwaita.clone(), other, None),
            "a colour-mode change after the startup fallback"
        );

        let mut state = State::starting_with(failing_read);
        apply_cli_args(&mut state, &command_line(&[("--variant", other_name)]));
        assert_eq!(
            installed(&state),
            (adwaita.clone(), other, None),
            "--variant {other_name} after the startup fallback"
        );

        state.error_message = Some("the banner before the rebuild".to_string());
        state.rebuild_theme();
        assert_eq!(
            installed(&state),
            (adwaita, other, None),
            "the theme watcher's rebuild after the startup fallback"
        );
    }

    /// `--icon-set` takes exactly what the icon-theme picker offers: a
    /// bundled set, `system` or `freedesktop` (the system icon theme), or an
    /// installed theme the picker lists (`installed_themes`). Anything else
    /// is reported and ignored, also a theme with an `index.theme` the
    /// picker does not list, such as `hicolor`. The picker's list is set
    /// here, so the test does not depend on the themes of the host.
    #[test]
    fn the_icon_set_flag_takes_what_the_picker_offers() {
        let mut state = State::default();
        let listed = "a-listed-icon-theme";
        state.installed_themes = vec![listed.to_string()];

        apply_cli_args(&mut state, &command_line(&[("--icon-set", "freedesktop")]));
        assert_eq!(
            state.icon_set_choice,
            IconSetChoice::System,
            "--icon-set freedesktop is not the system icon theme"
        );

        apply_cli_args(&mut state, &command_line(&[("--icon-set", "material")]));
        assert_eq!(state.icon_set_choice, IconSetChoice::Material);
        for unlisted in ["no-such-icon-theme", "hicolor"] {
            apply_cli_args(&mut state, &command_line(&[("--icon-set", unlisted)]));
            assert_eq!(
                state.icon_set_choice,
                IconSetChoice::Material,
                "--icon-set {unlisted}, which the picker does not offer, replaced the icon set"
            );
        }

        apply_cli_args(&mut state, &command_line(&[("--icon-set", listed)]));
        assert_eq!(
            state.icon_set_choice,
            IconSetChoice::Freedesktop(listed.to_string()),
            "--icon-set {listed}, which the picker offers, is not chosen"
        );
    }

    /// `--tab` takes each tab's name, and `textinputs` and `thememap` for
    /// two of them; another name is reported with the tabs it could have
    /// been.
    #[test]
    fn the_tab_flag_names_every_tab() {
        for &tab in Tab::ALL {
            assert_eq!(CliArgs::parse_tab(tab.flag()), Ok(tab));
        }
        assert_eq!(CliArgs::parse_tab("textinputs"), Ok(Tab::TextInputs));
        assert_eq!(CliArgs::parse_tab("thememap"), Ok(Tab::ThemeMap));
        assert_eq!(CliArgs::parse_tab("basic"), Ok(Tab::Basic));
        assert_eq!(
            Tab::ALL.first(),
            Some(&Tab::Basic),
            "Basic is the first tab"
        );
        match CliArgs::parse_tab("no-such-tab") {
            Ok(tab) => panic!("--tab no-such-tab opened {tab:?}"),
            Err(error) => {
                for tab in Tab::ALL {
                    assert!(error.contains(tab.flag()), "{error}");
                }
            }
        }
    }

    /// Where no freedesktop icon theme is installed -- off Linux, always --
    /// `--icon-set` says so, rather than end its report on an empty list.
    #[test]
    fn the_icon_set_flag_says_when_no_icon_theme_is_installed() {
        match CliArgs::icon_set_choice("no-such-icon-theme", &[]) {
            Ok(choice) => panic!("--icon-set no-such-icon-theme chose {choice:?}"),
            Err(error) => assert!(error.ends_with("; none are installed"), "{error}"),
        }
    }

    /// A freedesktop icon theme's spinner is that theme's own, and a theme
    /// that has none shows none: never the system theme's.
    #[test]
    fn the_spinner_comes_from_the_chosen_icon_theme() {
        let mut state = State::default();
        if state.installed_themes.is_empty() {
            eprintln!(
                "the_spinner_comes_from_the_chosen_icon_theme: this host has no freedesktop \
                 icon theme, so no theme's spinner was checked"
            );
        }
        for name in state.installed_themes.clone() {
            let _ = update(
                &mut state,
                Message::IconSetSelected(IconSetChoice::Freedesktop(name.clone())),
            );
            let expected = FreedesktopLoader::load_indicator(Some(&name))
                .and_then(|anim| to_svg_handle(anim.first_frame(), None));
            let spinners = state.animated_frames.len() + state.animated_spins.len();
            assert_eq!(
                state.animated_static.first().map(|(_, handle)| handle),
                expected.as_ref(),
                "{name}: the spinner is not the one {name} has"
            );
            assert_eq!(
                state.basic_spinner.is_indicator(),
                expected.is_some(),
                "{name}: the Basic page's spinner is not {name}'s indicator"
            );
            assert_eq!(
                spinners,
                usize::from(expected.is_some()),
                "{name}: {spinners} spinners animate"
            );
        }
    }

    /// A freedesktop theme without a spinner gets none, never the system
    /// theme's: checked with a theme that does not exist, so on every host.
    #[test]
    fn a_freedesktop_theme_without_a_spinner_gets_none() {
        let theme = "no-such-icon-theme";
        let (frames, _, _, spins, _, _, statics) =
            build_animation_caches(IconSet::Freedesktop, Some(theme));
        assert!(
            frames.is_empty() && spins.is_empty() && statics.is_empty(),
            "{theme} has a spinner: {} frame animations, {} spins, {} static frames",
            frames.len(),
            spins.len(),
            statics.len()
        );
    }

    /// A choice that followed the preset keeps following it, also where it
    /// followed it onto `system` (`default_icon_choice`, where the preset's
    /// own icon theme is not installed); the user's pick, `system` too,
    /// stays chosen across a preset change.
    #[test]
    fn an_icon_choice_that_followed_the_preset_keeps_following_it() {
        let mut state = State::default();
        let is_dark = state.is_dark;
        let followed = native_theme::theme::Theme::list_presets_for_platform()
            .iter()
            .find_map(|info| match preset_icon_choice(info.key, is_dark) {
                Some(choice @ IconSetChoice::Default(_)) => Some((info.key.to_string(), choice)),
                _ => None,
            });
        let (preset, expected) = match followed {
            Some(followed) => followed,
            None => panic!("no preset of this platform names an available icon theme"),
        };

        state.icon_set_choice = IconSetChoice::System;
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset(preset.clone())),
        );
        assert_eq!(
            state.icon_set_choice, expected,
            "a choice that followed the preset onto system did not follow {preset}"
        );

        let _ = update(&mut state, Message::IconSetSelected(IconSetChoice::System));
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::OsTheme(String::new())),
        );
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset(preset.clone())),
        );
        assert_eq!(
            state.icon_set_choice,
            IconSetChoice::System,
            "the user's pick of system did not stay chosen across a preset change"
        );
    }

    /// `--pointer X,Y` is two whole logical pixels, and anything else holds
    /// no pointer.
    #[test]
    fn the_pointer_flag_is_a_point() {
        assert_eq!(parse_point("259,96"), Some((259, 96)));
        assert_eq!(parse_point(" 259 , 96 "), Some((259, 96)));
        for bad in ["259", "x,1", "1,-2", ""] {
            assert_eq!(parse_point(bad), None, "{bad:?}");
        }
    }

    /// `--variant` and `--theme` values the showcase cannot honour are
    /// reported and ignored: the state stays what it would have been
    /// without the flag. `--variant system` follows the OS, `--theme
    /// default` is the OS theme, and a preset of another platform is not
    /// installed.
    #[test]
    fn the_command_line_rejects_what_it_cannot_honour() {
        let mut state = State::default();
        let before = shown(&state);
        apply_cli_args(&mut state, &command_line(&[("--variant", "sideways")]));
        assert_eq!(
            shown(&state),
            before,
            "--variant sideways changed the state"
        );

        let other = other_mode(&state);
        let other_name = if other == AppColorMode::Dark {
            "dark"
        } else {
            "light"
        };
        apply_cli_args(&mut state, &command_line(&[("--variant", other_name)]));
        assert_eq!(state.color_mode, other, "--variant {other_name}");
        apply_cli_args(&mut state, &command_line(&[("--variant", "system")]));
        assert_eq!(state.color_mode, AppColorMode::System, "--variant system");

        let offered = native_theme::theme::Theme::list_presets_for_platform();
        let foreign = native_theme::theme::Theme::list_presets()
            .iter()
            .find(|info| !offered.iter().any(|o| o.key == info.key))
            .map(|info| info.key);
        let foreign = match foreign {
            Some(key) => key,
            None => panic!("every bundled preset is offered on this platform"),
        };
        let before = shown(&state);
        apply_cli_args(&mut state, &command_line(&[("--theme", foreign)]));
        assert_eq!(
            shown(&state),
            before,
            "--theme {foreign} installed a preset of another platform"
        );
        apply_cli_args(&mut state, &command_line(&[("--theme", "no-such-preset")]));
        assert_eq!(
            shown(&state),
            before,
            "--theme no-such-preset changed the state"
        );

        apply_cli_args(
            &mut state,
            &command_line(&[("--theme", foreign), ("--variant", other_name)]),
        );
        assert_eq!(
            (&state.current_choice, state.color_mode),
            (&before.0, other),
            "--theme {foreign} --variant {other_name}: not the variant alone"
        );

        let preset = match offered.first() {
            Some(info) => info.key,
            None => panic!("this platform offers no preset"),
        };
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset(preset.to_string())),
        );
        apply_cli_args(&mut state, &command_line(&[("--theme", "default")]));
        assert!(
            matches!(state.current_choice, ThemeChoice::OsTheme(_)),
            "--theme default: {:?}",
            state.current_choice
        );
        assert!(
            state.error_message.is_none(),
            "--theme default: {:?}",
            state.error_message
        );
    }

    /// Which list item of [`MARKDOWN_SAMPLE`] holds the link, and where it
    /// points — read from the document itself rather than copied out of it.
    fn markdown_link() -> (usize, &'static str) {
        let items: Vec<&'static str> = MARKDOWN_SAMPLE
            .lines()
            .filter(|line| line.starts_with("- "))
            .collect();
        let nth = match items.iter().position(|line| line.contains("](")) {
            Some(index) => index + 1,
            None => panic!("MARKDOWN_SAMPLE has no link in a list item"),
        };
        let line = items[nth - 1];
        let (start, end) = match (line.find("]("), line.rfind(')')) {
            (Some(start), Some(end)) => (start + 2, end),
            _ => panic!("MARKDOWN_SAMPLE's link is not an inline link"),
        };
        (nth, &line[start..end])
    }

    /// The bounds of the `nth` list item `markdown::view` lays out.
    ///
    /// A link lives in a `rich_text` span, and `rich_text` implements no
    /// `Widget::operate`, so no selector can see it. What `markdown` does
    /// expose is the row it puts each list item in: a bullet `text` followed
    /// by the item's own container. Counting the bullets picks the item out.
    fn markdown_item_bounds(ui: &mut Simulator<'_, Message>, nth: usize) -> Rectangle {
        let mut bullets = 0usize;
        let found = ui.find(move |candidate: Candidate<'_>| -> Option<Rectangle> {
            match candidate {
                Candidate::Text {
                    content: "\u{2022}",
                    ..
                } => {
                    bullets += 1;
                    None
                }
                Candidate::Container {
                    visible_bounds: Some(bounds),
                    ..
                } if bullets == nth => Some(bounds),
                _ => None,
            }
        });
        match found {
            Ok(bounds) => bounds,
            Err(error) => panic!("markdown list item {nth}: {error}"),
        }
    }

    // -----------------------------------------------------------------------
    // styles_cover_every_widget_shown
    // -----------------------------------------------------------------------

    /// The showcase's own source, comments and string literals removed.
    const SHOWCASE: &str = include_str!("showcase-iced.rs");

    /// One row of the coverage table: a widget constructor, and the
    /// `styles::*` functions that dress what it builds.
    struct Dressed {
        ctor: &'static str,
        /// One entry per style the constructor has to be given; a site clears
        /// an entry by carrying any one of its alternatives. A widget with
        /// two dressable parts — a picker and the menu it drops — therefore
        /// has two entries, and losing either is a failure.
        styles: &'static [&'static [&'static str]],
    }

    /// Every constructor in the showcase that has a `styles::*` function.
    ///
    /// `container(` is deliberately absent (spec §6.2): most of them are
    /// layout and carry no class at all. `styles::container_card`'s own call
    /// sites are covered by the second half of the test, which asks every
    /// public style function for at least one caller.
    ///
    /// A button in a role the connector has no class for -- a flat button, a
    /// menu row, a tab, a list row -- wears the showcase's own style
    /// function for that role, built from that role's theme leaves alone:
    /// `ghost_button`, `menu_row` (`menu_title`, a menu bar's title, is a
    /// menu row), `tab_style`, `list_row`. A line in a
    /// widget's own colour wears `line_style`, the separator's style in that
    /// colour.
    const DRESSED: &[Dressed] = &[
        Dressed {
            ctor: "button",
            styles: &[&[
                "styles::button",
                "styles::button_primary",
                "styles::button_danger",
                "styles::button_success",
                "styles::button_warning",
                "styles::button_link",
                "styles::segment",
                "styles::expander",
                "ghost_button",
                "menu_row",
                "menu_title",
                "tab_style",
                "list_row",
            ]],
        },
        Dressed {
            ctor: "text_input",
            styles: &[&["styles::text_input"]],
        },
        Dressed {
            ctor: "text_editor",
            styles: &[&["styles::text_editor"]],
        },
        Dressed {
            ctor: "checkbox",
            styles: &[&["styles::checkbox"]],
        },
        Dressed {
            ctor: "radio",
            styles: &[&["styles::radio"]],
        },
        Dressed {
            ctor: "toggler",
            styles: &[&["styles::toggler"]],
        },
        Dressed {
            ctor: "pick_list",
            styles: &[&["styles::pick_list"], &["styles::menu"]],
        },
        Dressed {
            ctor: "combo_box",
            styles: &[&["styles::text_input"], &["styles::menu"]],
        },
        Dressed {
            ctor: "scrollable",
            styles: &[&["styles::scrollable"], &["styles::scrollbar"]],
        },
        Dressed {
            ctor: "slider",
            styles: &[&["styles::slider"]],
        },
        Dressed {
            ctor: "vertical_slider",
            styles: &[&["styles::slider"]],
        },
        Dressed {
            ctor: "progress_bar",
            styles: &[&["styles::progress_bar"]],
        },
        Dressed {
            ctor: "tooltip",
            styles: &[&["styles::tooltip"]],
        },
        Dressed {
            ctor: "rule::horizontal",
            styles: &[&["styles::rule", "line_style"]],
        },
        Dressed {
            ctor: "rule::vertical",
            styles: &[&["styles::rule"]],
        },
    ];

    /// The iced classes the connector replaces. A widget wearing one of these
    /// is a widget the platform's theme never reached.
    const REPLACED: &[&str] = &[
        "button::primary",
        "button::secondary",
        "button::danger",
        "button::success",
        "button::text",
        "container::rounded_box",
    ];

    /// Constructors a connector function dresses itself, as
    /// `(constructor, the style it applies, the function)`:
    /// `native_theme_iced::radio(resolved, radio(..), ..)` applies
    /// `styles::radio` with the platform's geometry and dot (`src/lib.rs`).
    const CONNECTOR_DRESSED: &[(&str, &str, &str)] =
        &[("radio", "styles::radio", "native_theme_iced::radio")];

    /// Is the constructor at `site` the widget argument of the connector
    /// function that dresses it -- `<function>(resolved, <site>..`?
    fn dressed_by_connector(source: &str, site: usize, ctor: &str) -> bool {
        CONNECTOR_DRESSED
            .iter()
            .filter(|(c, _, _)| *c == ctor)
            .any(|(_, _, wrapper)| {
                source[..site]
                    .trim_end()
                    .strip_suffix(',')
                    .map(str::trim_end)
                    .and_then(|s| s.strip_suffix("resolved"))
                    .map(str::trim_end)
                    .and_then(|s| s.strip_suffix('('))
                    .is_some_and(|s| s.ends_with(wrapper))
            })
    }

    /// Every widget the showcase renders wears the platform's own style.
    #[test]
    fn styles_cover_every_widget_shown() {
        let source = strip_comments_and_strings(SHOWCASE);

        for replaced in REPLACED {
            if let Some(at) = source.find(replaced) {
                panic!(
                    "{replaced} is still used at showcase-iced.rs:{}",
                    line_at(&source, at)
                );
            }
        }

        let mut total = 0usize;
        for row in DRESSED {
            let sites = call_sites(&source, row.ctor);
            assert!(
                !sites.is_empty(),
                "{}( has no call site left in the showcase",
                row.ctor
            );
            // The invariant `call_sites` rests on: the showcase imports its
            // constructors and calls them bare.
            let qualified = format!("widget::{}(", row.ctor);
            assert!(
                !source.contains(&qualified),
                "{qualified} is a call site this test cannot see; the showcase \
                 calls its constructors by their imported names only"
            );
            total += sites.len();

            for required in row.styles {
                let bare: Vec<String> = sites
                    .iter()
                    .filter(|site| {
                        !is_dressed(&source, **site, required)
                            && !dressed_by_connector(&source, **site, row.ctor)
                    })
                    .map(|site| format!("showcase-iced.rs:{}", line_at(&source, *site)))
                    .collect();
                assert!(
                    bare.is_empty(),
                    "{}( is built with no {} at {}",
                    row.ctor,
                    required.join(" / "),
                    bare.join(", ")
                );
            }
        }
        assert!(total > 0, "the coverage table found nothing");

        // Every public style function the connector ships is demonstrated,
        // itself or through the connector function that applies it.
        for function in public_functions(include_str!("../src/styles.rs")) {
            let call = format!("styles::{function}");
            let applied_by = CONNECTOR_DRESSED
                .iter()
                .filter(|(_, style, _)| *style == call)
                .any(|(_, _, wrapper)| !call_sites(&source, wrapper).is_empty());
            assert!(
                !call_sites(&source, &call).is_empty() || applied_by,
                "styles::{function} has no call site in the showcase"
            );
        }
        if cfg!(feature = "iced_aw") {
            for function in public_functions(include_str!("../src/styles/aw.rs")) {
                let call = format!("styles::aw::{function}");
                assert!(
                    !call_sites(&source, &call).is_empty(),
                    "styles::aw::{function} has no call site in the showcase"
                );
            }
        }
    }

    /// The connector's padding helpers: each returns a padding whose stated
    /// sides are the theme's.
    const PADDING_HELPERS: &[&str] = &["button_padding(", "padding_or(", "stated_padding("];

    /// Every button dressed in a `styles::button*` class takes the theme's
    /// padding through one of the connector's padding helpers, rather than a
    /// spacing of the showcase's own or iced's default -- the push buttons
    /// `button.border.padding`, and the buttons that stand in for a menu item
    /// or a tab that widget's padding. A button that stands in for a link
    /// takes `LINK_PADDING`, none, because the theme states none for a link
    /// (`docs/property-registry.toml` `[link]` has no padding).
    #[test]
    fn themed_buttons_take_the_themes_padding() {
        let source = strip_comments_and_strings(SHOWCASE);
        let classes = [
            "styles::button",
            "styles::button_primary",
            "styles::button_danger",
            "styles::button_success",
            "styles::button_warning",
            "styles::button_link",
        ];
        let derived = theme_padding_bindings(&source);
        let from_theme = |args: &str| {
            PADDING_HELPERS.iter().any(|helper| args.contains(helper))
                || derived.iter().any(|name| {
                    args.match_indices(name).any(|(at, _)| {
                        let joined = |c: char| c.is_alphanumeric() || c == '_';
                        !args[..at].chars().next_back().is_some_and(joined)
                            && !args[at + name.len()..].chars().next().is_some_and(joined)
                    })
                })
        };

        let mut checked = 0usize;
        let mut links = 0usize;
        let mut unpadded = Vec::new();
        for site in call_sites(&source, "button") {
            if !is_dressed(&source, site, &classes) {
                continue;
            }
            checked += 1;
            if source[..site].trim_end().ends_with("apply_pad(") {
                continue;
            }
            // A link has no padding: docs/property-registry.toml `[link]`
            // states none, and `LINK_PADDING` is that none.
            let unpadded_link = is_dressed(&source, site, &["styles::button_link"])
                && close_of_call(&source, site).is_some_and(|end| {
                    method_calls(&source, end)
                        .iter()
                        .any(|(name, args)| *name == "padding" && args.trim() == "(LINK_PADDING)")
                });
            if unpadded_link {
                links += 1;
                continue;
            }
            let padded = close_of_call(&source, site).is_some_and(|end| {
                method_calls(&source, end)
                    .iter()
                    .any(|(name, args)| *name == "padding" && from_theme(args))
            });
            if !padded {
                unpadded.push(format!("showcase-iced.rs:{}", line_at(&source, site)));
            }
        }
        assert!(checked > 0, "no themed button was found");
        assert!(
            links > 0,
            "no link-styled button takes LINK_PADDING, so the link exemption \
             describes nothing"
        );
        assert!(
            unpadded.is_empty(),
            "themed buttons not padded from the theme: {}",
            unpadded.join(", ")
        );
    }

    /// The card surfaces and the `iced_aw` tabs take the padding the theme
    /// states for them, not a spacing of the showcase's own: the `Card` and
    /// the `TabBar`/`Tabs` where every side is stated (`iced_aw` keeps their
    /// defaults private), the containers dressed as a card side by side.
    #[test]
    fn card_and_tab_padding_come_from_the_theme() {
        let source = strip_comments_and_strings(SHOWCASE);
        for per_section in [".padding_head(", ".padding_body(", ".padding_foot("] {
            if let Some(at) = source.find(per_section) {
                panic!(
                    "the card's padding is set section by section at \
                     showcase-iced.rs:{}, not from card.border.padding",
                    line_at(&source, at)
                );
            }
        }
        for call in [
            "stated_padding(&card_t.border.padding)",
            "stated_padding(&tab_t.border.padding)",
            "padding_or(&card.border.padding, Padding::ZERO)",
        ] {
            assert!(
                source.contains(call),
                "{call} has no call site in the showcase"
            );
        }
    }

    /// The `Card` is dismissed by a themed button of its own, never by
    /// `Card::on_close`: `iced_aw` 0.14.1 styles that close button with its
    /// default class (`widget/card.rs:193-206`), so the icon is white whatever
    /// `styles::aw::card` says.
    #[test]
    fn the_card_has_no_iced_aw_close_button() {
        let source = strip_comments_and_strings(SHOWCASE);
        if let Some(at) = source.find(".on_close(") {
            panic!(
                "Card::on_close is called at showcase-iced.rs:{}",
                line_at(&source, at)
            );
        }
    }

    /// A page's title outranks its section titles, and a section title the
    /// body text, on the native presets in both modes: each takes the role
    /// `docs/platform-facts.md` §2.19 defines for it -- `dialog_title` for a
    /// page, `section_heading` for a section -- and body text is the theme's
    /// font. KDE has no `display` role; set in it, a page title fell to body
    /// size, below its own description.
    #[test]
    fn page_and_section_titles_take_their_defined_roles() {
        for preset in ["kde-breeze", "adwaita", "macos-sonoma", "windows-11"] {
            for is_dark in [false, true] {
                let at = format!("{preset} (dark: {is_dark})");
                let resolved = match native_theme_iced::from_preset(preset, is_dark) {
                    Ok((_, resolved)) => resolved,
                    Err(error) => panic!("{at}: {error}"),
                };
                let ts = &resolved.text_scale;
                let page = page_title(ts).size;
                let section = section_title(ts).size;
                let body = resolved.defaults.font.size;
                assert!(
                    page >= section && section >= body,
                    "{at}: page title {page}px, section title {section}px, body {body}px"
                );
                assert!(
                    std::ptr::eq(page_title(ts), &ts.dialog_title),
                    "{at}: a page title is not set in dialog_title"
                );
                assert!(
                    std::ptr::eq(section_title(ts), &ts.section_heading),
                    "{at}: a section title is not set in section_heading"
                );
            }
        }
    }

    /// Text set in a `text_scale` role takes the role's weight with its size,
    /// which `.role(..)` applies together, so no role's size or weight is read
    /// on its own outside the text-scale table's rows, `scale_rows`.
    #[test]
    fn text_roles_carry_their_weight() {
        let source = strip_comments_and_strings(SHOWCASE);
        let table = match source.find("let scale_rows") {
            Some(start) => match source[start..].find("];") {
                Some(len) => start..start + len,
                None => panic!("scale_rows has no end"),
            },
            None => panic!("scale_rows is gone"),
        };
        for role in ["caption", "section_heading", "dialog_title", "display"] {
            for field in ["size", "weight"] {
                let read = format!("ts.{role}.{field}");
                for (at, _) in source.match_indices(read.as_str()) {
                    assert!(
                        table.contains(&at),
                        "{read} is read on its own at showcase-iced.rs:{}",
                        line_at(&source, at)
                    );
                }
            }
        }
        let header = match source.find("fn section_header") {
            Some(start) => match source[start..].find("\n}") {
                Some(len) => &source[start..start + len],
                None => &source[start..],
            },
            None => panic!("section_header is gone"),
        };
        assert!(
            header.contains("page_title(ts)"),
            "section_header does not set its title in page_title"
        );
    }

    /// The height iced lays one line of text out in, at `size`.
    fn line_of(size: f32) -> f32 {
        f32::from(iced_core::text::LineHeight::default().to_absolute(iced::Pixels(size)))
    }

    /// The height of the widget found by `selector`.
    fn height_of(ui: &mut Simulator<'_, Message>, selector: &'static str) -> f32 {
        match ui.find(selector) {
            Ok(target) => target.bounds().height,
            Err(error) => panic!("{selector}: {error}"),
        }
    }

    /// Button labels, inputs and body text are drawn at the theme's sizes.
    /// iced fixes its default text size when the renderer is created, so text
    /// given no size is drawn at iced's default under every theme.
    #[test]
    fn text_bearing_widgets_take_the_theme_sizes() {
        for preset in ["kde-breeze", "adwaita"] {
            let (theme, resolved) = match native_theme_iced::from_preset(preset, false) {
                Ok(installed) => installed,
                Err(error) => panic!("{preset}: {error}"),
            };
            let button_line = line_of(resolved.button.font.size);
            // Body text is laid out at the theme's line height.
            let body_line = resolved.defaults.font.size * resolved.defaults.line_height;
            let input_height =
                line_of(resolved.input.font.size) + native_theme_iced::input_padding(&resolved).y();
            // The OS's text-scaling factor is left out: the sizes compared are
            // the theme's own.
            let mut state = State {
                current_theme: theme,
                current_resolved: resolved,
                accessibility: native_theme_iced::AccessibilityPreferences::default(),
                active_tab: Tab::Buttons,
                ..State::default()
            };

            let mut ui = interface(&state);
            let label = height_of(&mut ui, "Primary");
            assert!(
                (label - button_line).abs() < 0.01,
                "{preset}: a button label is {label}px tall, button.font gives {button_line}px"
            );
            let body = height_of(&mut ui, "Interactive button styles from the resolved theme");
            assert!(
                (body - body_line).abs() < 0.01,
                "{preset}: a page description is {body}px tall, defaults.font gives {body_line}px"
            );
            drop(ui);

            state.active_tab = Tab::TextInputs;
            let mut ui = interface(&state);
            let input = match ui.find(selector::id(TEXT_INPUT_ID)) {
                Ok(target) => target.bounds().height,
                Err(error) => panic!("{preset}: {TEXT_INPUT_ID}: {error}"),
            };
            assert!(
                (input - input_height).abs() < 0.01,
                "{preset}: a text input is {input}px tall, input.font and its padding give \
                 {input_height}px"
            );
        }
    }

    /// Text in a `text_scale` role and body text are laid out in the theme's
    /// line boxes (the role's `line_height`, `defaults.line_height` times the
    /// size), not iced's own 1.3: the Basic tab's section title and its body
    /// text.
    #[test]
    fn text_takes_the_themes_line_height() {
        let (theme, resolved) = match native_theme_iced::from_preset("kde-breeze", false) {
            Ok(installed) => installed,
            Err(error) => panic!("kde-breeze: {error}"),
        };
        let heading = resolved.text_scale.section_heading.line_height;
        let body = resolved.defaults.font.size * resolved.defaults.line_height;
        let state = State {
            current_theme: theme,
            current_resolved: resolved,
            accessibility: native_theme_iced::AccessibilityPreferences::default(),
            active_tab: Tab::Basic,
            ..State::default()
        };
        let mut ui = interface(&state);
        let title = height_of(&mut ui, "Checkboxes");
        assert!(
            (title - heading).abs() < 0.01,
            "a section title is {title}px tall, section_heading.line_height is {heading}px"
        );
        let text = height_of(&mut ui, "Body");
        assert!(
            (text - body).abs() < 0.01,
            "body text is {text}px tall, the theme's line box is {body}px"
        );
    }

    /// A link is underlined exactly where the theme says the platform
    /// underlines links: every underline the showcase asks iced for is
    /// `link.underline_enabled`, and the Basic tab's link asks for one.
    #[test]
    fn a_link_is_underlined_by_link_underline_enabled() {
        let source = strip_comments_and_strings(SHOWCASE);
        let basic = match (
            source.find("fn view_basic<"),
            source.find("fn button_info("),
        ) {
            (Some(start), Some(end)) if start < end => &source[start..end],
            _ => panic!("view_basic not found"),
        };
        let calls: Vec<&str> = source
            .match_indices(".underline(")
            .map(|(at, _)| {
                let open = at + ".underline".len();
                close_of_call(&source, open).map_or("", |end| &source[open..end])
            })
            .collect();
        assert!(
            basic.contains(".underline("),
            "the Basic tab's link asks for no underline"
        );
        assert!(
            calls.iter().all(|args| *args == "(link.underline_enabled)"),
            "an underline not taken from link.underline_enabled: {calls:?}"
        );
    }

    /// The Basic tab's tooltip takes `tooltip.border.padding` plus the border
    /// line where the theme states all four sides alike (kde-breeze's 3,
    /// macos-sonoma's 4), and iced's own padding where it does not: iced's
    /// `Tooltip::padding` is one number, and adwaita states 10 and 6,
    /// windows-11 9, 6 and 8.
    #[test]
    fn the_basic_tooltip_takes_a_uniform_stated_padding() {
        for (preset, expected) in [
            ("kde-breeze", Some(3.0 + 1.0)),
            ("macos-sonoma", Some(4.0 + 0.5)),
            ("adwaita", None),
            ("windows-11", None),
        ] {
            let resolved = match native_theme_iced::from_preset(preset, false) {
                Ok((_, resolved)) => resolved,
                Err(error) => panic!("{preset}: {error}"),
            };
            let b = &resolved.tooltip.border;
            if let Some(side) = expected {
                assert_eq!(
                    b.padding.top.map(|t| t + b.line_width),
                    Some(side),
                    "{preset}"
                );
            }
            assert_eq!(tooltip_padding(&resolved), expected, "{preset}");
        }
    }

    /// The Basic tab's push button, text field and drop-down are laid out at
    /// the minimum sizes the theme states (`button.min_width` and
    /// `.min_height`, `input.min_height`, `combo_box.min_height`), or at their
    /// own size where their content, padded, is larger — as gpui-component's
    /// are through `native_theme_gpui::geometry` and the platform's own are.
    #[test]
    fn basic_controls_take_the_stated_minimum_sizes() {
        for preset in ["kde-breeze", "adwaita", "material"] {
            let (theme, resolved) = match native_theme_iced::from_preset(preset, false) {
                Ok(installed) => installed,
                Err(error) => panic!("{preset}: {error}"),
            };
            let r = resolved.clone();
            let own = |size: f32, pad: Padding| size * r.defaults.line_height + pad.y();
            let state = State {
                current_theme: theme,
                current_resolved: resolved,
                accessibility: native_theme_iced::AccessibilityPreferences::default(),
                active_tab: Tab::Basic,
                ..State::default()
            };
            let mut ui = interface(&state);

            let button = probe_bounds(&mut ui, probes::BASIC_BUTTON);
            let label = height_of(&mut ui, "Button");
            let pad = native_theme_iced::button_padding(&r);
            assert!(
                button.width >= r.button.min_width - 0.01,
                "{preset}: the button is {}px wide, button.min_width is {}px",
                button.width,
                r.button.min_width
            );
            let expected = r.button.min_height.max(label + pad.y());
            assert!(
                (button.height - expected).abs() < 0.01,
                "{preset}: the button is {}px tall, expected {expected}px",
                button.height
            );

            let input = probe_bounds(&mut ui, probes::BASIC_TEXT_INPUT);
            let expected = r
                .input
                .min_height
                .max(own(r.input.font.size, native_theme_iced::input_padding(&r)));
            assert!(
                (input.height - expected).abs() < 0.01,
                "{preset}: the text input is {}px tall, expected {expected}px",
                input.height
            );

            let pick = probe_bounds(&mut ui, probes::BASIC_PICK_LIST);
            let expected = r.combo_box.min_height.max(own(
                r.combo_box.font.size,
                native_theme_iced::combo_box_padding(&r),
            ));
            assert!(
                (pick.height - expected).abs() < 0.01,
                "{preset}: the drop-down is {}px tall, expected {expected}px",
                pick.height
            );
        }
    }

    /// Every text the Basic page lays out, with its bounds.
    fn texts(ui: &mut Simulator<'_, Message>) -> Vec<(String, Rectangle)> {
        let mut found = Vec::new();
        {
            let sink = &mut found;
            let _ = ui.find(move |candidate: Candidate<'_>| -> Option<()> {
                if let Candidate::Text {
                    content, bounds, ..
                } = candidate
                {
                    sink.push((content.to_string(), bounds));
                }
                None
            });
        }
        found
    }

    /// The Basic page is Basic v5: its five columns hold their groups in
    /// order, each group under its heading, and every control label the spec
    /// names is on the page.
    #[test]
    fn the_basic_page_has_every_group_in_its_column() {
        const COLUMNS: [&[&str]; 5] = [
            &["Buttons", "Checkboxes", "Radio buttons", "Drop-down"],
            &["Text inputs", "Text area", "Slider"],
            &[
                "Switches",
                "Number input",
                "Spinner",
                "Segmented control",
                "Card",
            ],
            &["Typography", "Separator", "Progress bar", "List"],
            &["Icons", "Tabs", "Expander", "Table"],
        ];
        let state = State::default();
        let mut ui = interface(&state);
        let texts = texts(&mut ui);
        // A heading shares its words with a page tab ("Buttons"), which sits
        // above the page: the lowest text with the words is the heading.
        let heading = |label: &str| -> Rectangle {
            match texts
                .iter()
                .filter(|(content, _)| content == label)
                .map(|(_, bounds)| *bounds)
                .max_by(|a, b| a.y.total_cmp(&b.y))
            {
                Some(bounds) => bounds,
                None => panic!("the Basic page has no {label:?} group"),
            }
        };
        let mut previous_x = f32::NEG_INFINITY;
        for column in COLUMNS {
            let first = heading(column[0]);
            assert!(
                first.x > previous_x,
                "{:?} is not right of the column before it",
                column[0]
            );
            previous_x = first.x;
            let mut previous_y = f32::NEG_INFINITY;
            for label in column {
                let bounds = heading(label);
                assert!(
                    (bounds.x - first.x).abs() < 0.01,
                    "{label:?} is not in {:?}'s column",
                    column[0]
                );
                assert!(bounds.y > previous_y, "{label:?} is out of order");
                previous_y = bounds.y;
            }
        }
        // Not the radio buttons', the switches' and the drop-down's labels,
        // nor the text area's lines: `radio`, `toggler`, `pick_list` and
        // `text_editor` implement no `Widget::operate`, so no selector sees
        // them ([`probe`]); their probes are found instead. Nor the link, a
        // rich text, which no text selector matches; the layout dump's test
        // finds it.
        for id in [
            probes::BASIC_RADIO_B,
            probes::BASIC_PICK_LIST,
            probes::BASIC_TEXT_AREA,
        ] {
            let _ = probe_bounds(&mut ui, id);
        }
        let mut labels = vec![
            "Button",
            "Primary",
            "Disabled",
            "Tooltip",
            "Unchecked",
            "Checked",
            "Off",
            "On",
            "Caption",
            "Body",
            "Section heading",
            "Dialog title",
            "Display",
            "Monospace",
            "Day",
            "Week",
            "Month",
            "Details",
            "Expanded content",
            "More",
            "Card content",
        ];
        if cfg!(feature = "iced_aw") {
            labels.extend(["One", "Two", "Item 1", "Item 2"]);
        }
        for label in labels {
            let _ = ui
                .find(label)
                .unwrap_or_else(|error| panic!("{label:?} is not on the Basic page: {error}"));
        }
    }

    /// The Basic page's text area, segmented control, expander and list take
    /// the sizes the theme states: three of the theme's lines inside the
    /// field's padding; `segmented_control.segment_height` and
    /// `expander.header_height` as outer heights, where the label fits;
    /// three rows of the list, each `list.row_height` or its content.
    #[test]
    fn basic_composites_take_the_stated_sizes() {
        for preset in ["kde-breeze", "material", "adwaita"] {
            let (theme, resolved) = match native_theme_iced::from_preset(preset, false) {
                Ok(installed) => installed,
                Err(error) => panic!("{preset}: {error}"),
            };
            let r = resolved.clone();
            let state = State {
                current_theme: theme,
                current_resolved: resolved,
                accessibility: native_theme_iced::AccessibilityPreferences::default(),
                active_tab: Tab::Basic,
                basic_spinner: native_theme_iced::Spinner::new(&r, IconSet::Material, None),
                ..State::default()
            };
            let mut ui = interface(&state);

            let area = probe_bounds(&mut ui, probes::BASIC_TEXT_AREA);
            let pad = native_theme_iced::text_area_padding(&r);
            let expected = 3.0 * r.input.font.size * r.defaults.line_height + pad.y();
            assert!(
                (area.height - expected).abs() < 0.01 && (area.width - BASIC_WIDE).abs() < 0.01,
                "{preset}: the text area is {area:?}, expected {BASIC_WIDE} x {expected}"
            );

            let sc = &r.segmented_control;
            let segmented = probe_bounds(&mut ui, probes::BASIC_SEGMENTED);
            let seg_pad = native_theme_iced::padding_or(
                &sc.border.padding,
                iced::widget::button::DEFAULT_PADDING,
            );
            // Each segment `segment_height` tall (its outer height; a segment
            // has no border), or its label's line and padding where taller,
            // inside the control's outline.
            let expected = sc
                .segment_height
                .max(sc.font.size * r.defaults.line_height + seg_pad.y())
                + 2.0 * sc.border.line_width;
            assert!(
                (segmented.height - expected).abs() < 0.01,
                "{preset}: the segmented control is {}px tall, expected {expected}px",
                segmented.height
            );

            let x = &r.expander;
            let header = probe_bounds(&mut ui, probes::BASIC_EXPANDER);
            let x_pad = native_theme_iced::padding_inside_border(
                &x.border,
                iced::widget::button::DEFAULT_PADDING,
            );
            let expected = x.header_height.max(line_of(x.font.size) + x_pad.y());
            assert!(
                (header.height - expected).abs() < 0.01,
                "{preset}: the expander header is {}px tall, expected {expected}px",
                header.height
            );

            let list = probe_bounds(&mut ui, probes::BASIC_LIST);
            let l = &r.list;
            let stated =
                native_theme_iced::padding_or(&l.border.padding, Padding::from(AW_LIST_PADDING));
            let row = l
                .row_height
                .unwrap_or(l.item_font.size * r.defaults.line_height + stated.y());
            let expected = BASIC_LIST_VISIBLE * row + 2.0 * l.border.line_width;
            assert!(
                (list.height - expected).abs() < 0.01,
                "{preset}: the list is {}px tall, expected {expected}px",
                list.height
            );

            let s = &r.spinner;
            let spinner = probe_bounds(&mut ui, probes::BASIC_SPINNER);
            assert!(
                (spinner.width - s.diameter).abs() < 0.01
                    && (spinner.height - s.diameter).abs() < 0.01,
                "{preset}: the spinner is {spinner:?}, expected {} across",
                s.diameter
            );

            let sw = &r.switch;
            for id in [
                probes::BASIC_SWITCH_OFF,
                probes::BASIC_SWITCH_ON,
                probes::BASIC_SWITCH_DISABLED,
            ] {
                let track = probe_bounds(&mut ui, id);
                assert!(
                    (track.width - sw.track_width).abs() < 0.01
                        && (track.height - sw.track_height).abs() < 0.01,
                    "{preset}: {id} is {track:?}, expected {} x {}",
                    sw.track_width,
                    sw.track_height
                );
            }

            // "Card content" sits the card's padding in from its edges:
            // `layout.container_margin` on every Linux preset, whose cards
            // state none.
            let card = probe_bounds(&mut ui, probes::BASIC_CARD);
            let margin = Gaps::from_layout(&state.layout).container;
            let label = texts(&mut ui)
                .into_iter()
                .find(|(content, _)| content == "Card content")
                .map(|(_, bounds)| bounds);
            assert!(
                label.is_some_and(|label| (label.x - card.x - margin).abs() < 0.01
                    && (label.y - card.y - margin).abs() < 0.01),
                "{preset}: the card's label is at {label:?} in {card:?}, {margin}px in expected"
            );
        }
    }

    /// The page and inspector tab rows frame the open tab only, with
    /// `tab.border`, its top corners rounded: the other tabs have no outline
    /// and square corners, in every status.
    #[test]
    fn only_the_open_tab_is_framed() {
        for (preset, is_dark) in [
            ("kde-breeze", false),
            ("kde-breeze", true),
            ("material", true),
        ] {
            let resolved = match native_theme_iced::from_preset(preset, is_dark) {
                Ok((_, resolved)) => resolved,
                Err(error) => panic!("{preset}: {error}"),
            };
            let b = &resolved.tab.border;
            for status in [
                button::Status::Active,
                button::Status::Hovered,
                button::Status::Pressed,
                button::Status::Disabled,
            ] {
                let open = tab_style(&resolved, true)(&Theme::Light, status).border;
                assert_eq!(
                    (open.color, open.width, open.radius),
                    (
                        to_color(b.color),
                        b.line_width,
                        iced::border::Radius::new(0.0).top(b.corner_radius)
                    ),
                    "{preset} {is_dark}: the open tab's frame ({status:?})"
                );
                let other = tab_style(&resolved, false)(&Theme::Light, status).border;
                assert_eq!(
                    (other.width, other.radius),
                    (0.0, iced::border::Radius::new(0.0)),
                    "{preset} {is_dark}: another tab's frame ({status:?})"
                );
            }
        }
    }

    /// Where the icon set has no indicator, the spinner draws an arc across
    /// its circle at every moment of its turn, the moments the captures take
    /// (about 2π seconds after the start) among them: the ink a snapshot of
    /// it shows reaches across `spinner.diameter` (its two farthest pixels at
    /// least the diameter less a stroke apart), and its body is
    /// `spinner.fill_color`.
    #[test]
    fn the_spinner_arc_reaches_across_its_diameter() {
        let root =
            std::env::temp_dir().join(format!("showcase-iced-spinner-{}", std::process::id()));
        let pi = std::f32::consts::PI;
        for preset in ["kde-breeze", "material", "adwaita"] {
            let resolved = match native_theme_iced::from_preset(preset, false) {
                Ok((_, resolved)) => resolved,
                Err(error) => panic!("{preset}: {error}"),
            };
            let s = &resolved.spinner;
            let fill = to_color(s.fill_color).into_rgba8();
            let spinner = native_theme_iced::Spinner::new(&resolved, IconSet::SegoeIcons, None);
            assert!(!spinner.is_indicator(), "{preset}: Segoe has no indicator");
            for (n, elapsed) in [0.0, 0.25, 0.5, 1.0, pi, 2.0 * pi, 6.3, 7.0, 3.0 * pi]
                .into_iter()
                .enumerate()
            {
                let mut ui: Simulator<'_, Message> = Simulator::with_size(
                    Settings::default(),
                    Size::new(s.diameter, s.diameter),
                    spinner.view(Duration::from_secs_f32(elapsed), false),
                );
                let snapshot = match ui.snapshot(&Theme::Light) {
                    Ok(snapshot) => snapshot,
                    Err(error) => panic!("{preset} at {elapsed}s: {error}"),
                };
                let dir = root.join(format!("{preset}-{n}"));
                let image = match snapshot
                    .matches_image(dir.join("arc.png"))
                    .map_err(|error| error.to_string())
                    .and_then(|_| {
                        std::fs::read_dir(&dir)
                            .and_then(|mut entries| {
                                entries.next().unwrap_or_else(|| {
                                    Err(std::io::Error::other("no snapshot written"))
                                })
                            })
                            .map_err(|error| error.to_string())
                    })
                    .and_then(|entry| {
                        image::open(entry.path())
                            .map(|image| image.to_rgba8())
                            .map_err(|error| error.to_string())
                    }) {
                    Ok(image) => image,
                    Err(error) => panic!("{preset} at {elapsed}s: {error}"),
                };
                let scale = image.width() as f32 / s.diameter;
                let background = image.get_pixel(0, 0).0;
                let ink: Vec<(f32, f32)> = image
                    .enumerate_pixels()
                    .filter(|(_, _, pixel)| {
                        pixel
                            .0
                            .iter()
                            .zip(background)
                            .map(|(a, b)| a.abs_diff(b) as u32)
                            .sum::<u32>()
                            > 96
                    })
                    .map(|(x, y, _)| ((x as f32 + 0.5) / scale, (y as f32 + 0.5) / scale))
                    .collect();
                let span = ink
                    .iter()
                    .flat_map(|a| ink.iter().map(move |b| (a.0 - b.0).hypot(a.1 - b.1)))
                    .fold(0.0_f32, f32::max);
                assert!(
                    span >= s.diameter - s.stroke_width,
                    "{preset} at {elapsed}s: the arc's ink spans {span}px, the spinner is {}px \
                     across",
                    s.diameter
                );
                assert!(
                    image.pixels().any(|pixel| pixel
                        .0
                        .iter()
                        .zip(fill)
                        .all(|(a, b)| a.abs_diff(b) <= 2)),
                    "{preset} at {elapsed}s: no pixel of the arc is spinner.fill_color"
                );
            }
        }
        let _ = std::fs::remove_dir_all(&root);
    }

    /// docs/showcase-elements.toml parses, and Widget Info has a route for
    /// every leaf the list names: a line saying how iced applies it, or why
    /// it cannot.
    #[test]
    fn the_element_list_parses_and_every_leaf_has_a_route() {
        let list = match showcase_elements() {
            Ok(list) => list,
            Err(error) => panic!("docs/showcase-elements.toml: {error}"),
        };
        assert!(!list.is_empty(), "the element list is empty");
        let mut unrouted = Vec::new();
        for element in &list {
            for leaf in &element.leaves {
                let route = iced_route(element, leaf);
                if route.contains("no route recorded") {
                    unrouted.push(format!("{}: {leaf}", element.id));
                }
            }
        }
        assert!(unrouted.is_empty(), "leaves with no route: {unrouted:#?}");
    }

    /// A window menu is as wide as its widest row, as the gpui and egui
    /// showcases' are: opened, the Theme menu's rows are each the widest
    /// row's label, and its shortcut `layout.widget_gap` after it, inside
    /// the row's padding, measured as iced shapes them; Preferences' shortcut
    /// sits flush right in its row, inside the padding.
    #[test]
    fn a_menu_is_as_wide_as_its_widest_row() {
        let state = State::default();
        let theme = theme(&state);
        let resolved = &state.current_resolved;
        let m = &resolved.menu;
        let pad = native_theme_iced::padding_or(&m.border.padding, button::DEFAULT_PADDING);
        let font = theme_font(&m.font);
        let size = scaled_text_size(m.font.size, &state.accessibility);
        let measured = |content: &str| {
            MeasuredText {
                content: content.to_owned(),
                size,
                line: size,
                font,
            }
            .extent()
            .width
        };
        let widest = ["Reload System Theme", "System", "Light", "Dark"]
            .into_iter()
            .map(measured)
            .chain([measured("Preferences…")
                + Gaps::from_layout(&state.layout).widget
                + measured(&binding(","))])
            .fold(0.0, f32::max);
        let mut ui: Simulator<'_, Message> =
            Simulator::with_size(Settings::default(), WINDOW_SIZE, view(&state));
        for _ in 0..2 {
            let drew = ui.snapshot(&theme);
            assert!(drew.is_ok(), "the interface did not draw: {drew:?}");
        }
        let title = match drawn_layout().get("chrome.menu_bar.theme") {
            Some(rect) => rect.center(),
            None => panic!("the Theme menu's title is not drawn"),
        };
        ui.point_at(title);
        let _ = ui.simulate([
            Event::Mouse(mouse::Event::CursorMoved { position: title }),
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
        ]);
        for _ in 0..2 {
            if let Err(error) = ui.snapshot(&theme) {
                panic!("the interface did not draw: {error}");
            }
        }
        let drawn = drawn_layout();
        let rect = |id: &str| match drawn.get(id) {
            Some(rect) => *rect,
            None => panic!("{id} is not drawn with the Theme menu open"),
        };
        let expected = pad.left + widest + pad.right;
        for id in [
            "chrome.menu.theme.reload",
            "chrome.menu.theme.system",
            "chrome.menu.theme.light",
            "chrome.menu.theme.dark",
            "chrome.menu.theme.preferences",
        ] {
            let width = rect(id).width;
            assert!(
                (width - expected).abs() < 0.01,
                "{id}: {width} wide, the widest row is {expected}"
            );
        }
        let row = rect("chrome.menu.theme.preferences");
        let shortcut = rect("chrome.menu.theme.preferences.shortcut");
        assert!(
            (shortcut.x + shortcut.width - (row.x + row.width - pad.right)).abs() < 0.01,
            "the shortcut {shortcut:?} is not flush right in {row:?} inside {pad:?}"
        );
    }

    /// The layout dump holds what the Basic page and the chrome draw: every
    /// id it records is an element of the list, a part's with it, and every
    /// element of the list on screen at rest is recorded -- all but the ones
    /// shown only under a condition (`when`) and the icons, which the icon
    /// theme on the test machine may not have.
    #[test]
    fn the_layout_dump_records_the_listed_elements() {
        let state = State::default();
        let theme = theme(&state);
        let mut ui: Simulator<'_, Message> =
            Simulator::with_size(Settings::default(), WINDOW_SIZE, view(&state));
        for _ in 0..2 {
            if let Err(error) = ui.snapshot(&theme) {
                panic!("the interface did not draw: {error}");
            }
        }
        let on_screen = drawn_layout();
        let unknown: Vec<_> = on_screen.keys().filter(|id| listed(id).is_none()).collect();
        assert!(
            unknown.is_empty(),
            "drawn ids the list has not: {unknown:?}"
        );
        // A selector goes through the whole tree, as the dump's `Survey` does.
        let _ = texts(&mut ui);
        let drawn = surveyed_layout();
        let unknown: Vec<_> = drawn.keys().filter(|id| listed(id).is_none()).collect();
        assert!(
            unknown.is_empty(),
            "recorded ids the list has not: {unknown:?}"
        );
        let missing: Vec<_> = elements()
            .iter()
            .filter(|e| e.when.is_none())
            .filter(|e| {
                !e.id.ends_with(".icon")
                    && ![
                        "basic.icons.small",
                        "basic.icons.toolbar",
                        "basic.icons.large",
                    ]
                    .contains(&e.id.as_str())
            })
            .filter(|e| !drawn.contains_key(e.id.as_str()))
            .map(|e| e.id.as_str())
            .collect();
        assert!(
            missing.is_empty(),
            "listed elements not recorded: {missing:?}"
        );
        for (id, rect) in &drawn {
            let parent = listed(id).map(|e| e.parent.as_str());
            assert!(
                parent == Some("window") || parent.is_some_and(|p| listed(p).is_some()),
                "{id}: its parent is not in the list"
            );
            assert!(
                rect.width >= 0.0 && rect.height >= 0.0,
                "{id}: a negative size {rect:?}"
            );
        }
    }

    /// The pointer over an element shows that element in Widget Info: over
    /// the Basic page's plain button its title is "Button · Normal", not the
    /// primary button's; over the primary one, "Button · Primary". Its rows
    /// are the list's leaves in the list's order, each with a value, and the
    /// status bar shows the same title.
    #[test]
    fn widget_info_names_the_hovered_element() {
        for (id, title) in [
            ("basic.buttons.default", "Button · Normal"),
            ("basic.buttons.primary", "Button · Primary"),
            ("basic.checkboxes.checked", "Checkbox · Checked"),
        ] {
            let mut state = State::default();
            let at = {
                let theme = theme(&state);
                let mut ui: Simulator<'_, Message> =
                    Simulator::with_size(Settings::default(), WINDOW_SIZE, view(&state));
                if let Err(error) = ui.snapshot(&theme) {
                    panic!("the interface did not draw: {error}");
                }
                let _ = ui.snapshot(&theme);
                match drawn_layout().get(id) {
                    Some(rect) => rect.center(),
                    None => panic!("{id} is not drawn"),
                }
            };
            let messages = {
                let mut ui: Simulator<'_, Message> =
                    Simulator::with_size(Settings::default(), WINDOW_SIZE, view(&state));
                ui.point_at(at);
                let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
                ui.into_messages().collect::<Vec<_>>()
            };
            for message in messages {
                let _ = update(&mut state, message);
            }
            assert_eq!(state.hovered_element, Some(id), "the pointer at {at:?}");
            let element = match listed(id) {
                Some(element) => element,
                None => panic!("{id} is not in the list"),
            };
            let info = ElementInfo::of(&state, element);
            assert_eq!(info.title, title);
            assert_eq!(shown_title(&state).as_deref(), Some(title));
            let leaves: Vec<&str> = info.rows.iter().map(|row| row.leaf).collect();
            let listed_leaves: Vec<&str> = element.leaves.iter().map(String::as_str).collect();
            assert_eq!(leaves, listed_leaves, "{id}: rows out of the list's order");
            assert!(
                info.rows.iter().all(|row| !row.value.is_empty()),
                "{id}: a row with no value"
            );
        }
    }

    /// A leaf's value reads as the resolved theme holds it, in the
    /// registry's spelling: a padding side in px, a font as its family, the
    /// size its source stated and its weight, a colour as its hex, a layout
    /// gap from the theme's layout, and what the theme leaves unset as not
    /// stated.
    #[test]
    fn leaf_values_read_the_resolved_theme() {
        let (theme, resolved) = match native_theme_iced::from_preset("kde-breeze", false) {
            Ok(installed) => installed,
            Err(error) => panic!("kde-breeze: {error}"),
        };
        let layout = match native_theme::theme::Theme::preset("kde-breeze") {
            Ok(preset) => preset.layout,
            Err(error) => panic!("kde-breeze: {error}"),
        };
        let r = resolved.clone();
        let state = State {
            current_theme: theme,
            current_resolved: resolved,
            layout: layout.clone(),
            ..State::default()
        };
        let values = match toml::Value::try_from(&state.current_resolved) {
            Ok(values) => values,
            Err(error) => panic!("the resolved theme as TOML: {error}"),
        };
        let value = |leaf: &str| leaf_value(&state, &values, leaf);
        let side = r.button.border.padding.top.map(|v| format!("{v} px"));
        assert_eq!(Some(value("button.border.padding_top_px").0), side);
        let (colour, swatch) = value("button.background_color");
        assert_eq!(colour, r.button.background_color.to_string());
        assert_eq!(swatch, Some(to_color(r.button.background_color)));
        let font = &r.button.font;
        let size = match font.defined_size {
            Some(native_theme::theme::FontSize::Pt(v)) => format!("{v} pt"),
            Some(native_theme::theme::FontSize::Px(v)) => format!("{v} px"),
            None => format!("{} px", font.size),
        };
        assert_eq!(
            value("button.font").0,
            format!("{} {size} {}", font.family, font.weight)
        );
        assert_eq!(
            value("layout.widget_gap_px").0,
            layout
                .widget_gap
                .map_or(NOT_STATED.to_string(), |v| format!("{v} px"))
        );
        let heading = &r.text_scale.section_heading;
        assert_eq!(
            value("text_scale.section_heading").0,
            format!("{} px {}", heading.size, heading.weight)
        );
        assert_eq!(
            value("defaults.line_height").0,
            format!("{}", r.defaults.line_height)
        );
    }

    /// The Basic page fits the window without scrolling under every preset
    /// and mode the captures take (kde-breeze, material, catppuccin-mocha and
    /// adwaita, light and dark): its content, laid out in the page area of a
    /// 1280 x 720 window with the preset's own layout gaps, ends inside the
    /// window.
    #[test]
    fn the_basic_page_fits_the_window() {
        for preset in ["kde-breeze", "material", "catppuccin-mocha", "adwaita"] {
            for dark in [false, true] {
                let mut state = State {
                    accessibility: native_theme_iced::AccessibilityPreferences::default(),
                    active_tab: Tab::Basic,
                    ..State::default()
                };
                let loaded = state.load_theme(&ThemeChoice::Preset(preset.to_string()), dark);
                assert!(loaded.is_ok(), "{preset}: {loaded:?}");
                let mut ui: Simulator<'_, Message> =
                    Simulator::with_size(Settings::default(), WINDOW_SIZE, view(&state));
                // The page's scrollable is the rightmost one as tall as half
                // the window: the side panel's are left of it, the list's and
                // the tab strip's are short.
                let mut scrollables: Vec<(Rectangle, Rectangle)> = Vec::new();
                {
                    let sink = &mut scrollables;
                    let _ = ui.find(move |candidate: Candidate<'_>| -> Option<()> {
                        if let Candidate::Scrollable {
                            bounds,
                            content_bounds,
                            ..
                        } = candidate
                        {
                            sink.push((bounds, content_bounds));
                        }
                        None
                    });
                }
                let page = scrollables
                    .into_iter()
                    .filter(|(bounds, _)| bounds.height > WINDOW_SIZE.1 / 2.0)
                    .max_by(|a, b| a.0.x.total_cmp(&b.0.x));
                assert!(
                    page.is_some(),
                    "{preset} (dark: {dark}): no page scrollable found"
                );
                if let Some((bounds, content)) = page {
                    assert!(
                        content.height <= bounds.height + 0.01,
                        "{preset} (dark: {dark}): the Basic page is {}px tall, its area {}px",
                        content.height,
                        bounds.height
                    );
                }
            }
        }
    }

    /// In a capture the pointer hovers nothing: the page `held_page` wraps
    /// for `--capture` and `--screenshot` hands its content no pointer, so the
    /// real one, wherever the desktop left it, shows no control hovered and
    /// fills no Widget Info. The page as it is does report the hover, which
    /// is what makes the first half mean something.
    #[test]
    fn a_capture_is_not_hovered_by_the_pointer() {
        let state = State::default();
        let at = {
            let mut ui = interface(&state);
            probe_bounds(&mut ui, probes::BASIC_BUTTON).center()
        };
        let hover = |page: Element<'_, Message>| -> Vec<Message> {
            let mut ui = Simulator::with_size(Settings::default(), TALL_WINDOW, page);
            ui.point_at(at);
            let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
            ui.into_messages().collect()
        };
        let live = hover(view(&state));
        assert!(
            live.iter()
                .any(|m| matches!(m, Message::ElementHovered("basic.buttons.default"))),
            "the page as it is reports no hover over the Basic button, so the \
             capture half of this test proves nothing: {live:?}"
        );
        let captured = hover(held_page(view(&state), None, false, true));
        assert!(
            captured.is_empty(),
            "a capture reacted to the pointer: {captured:?}"
        );
    }

    /// The user's text-scaling factor reaches every text alike: at 1.5, a
    /// button label and body text are both drawn at 1.5 times the theme's size.
    #[test]
    fn text_scales_by_the_text_scaling_factor() {
        let factor = 1.5;
        let (theme, resolved) = match native_theme_iced::from_preset("kde-breeze", false) {
            Ok(installed) => installed,
            Err(error) => panic!("kde-breeze: {error}"),
        };
        let button_line = line_of(resolved.button.font.size * factor);
        let body_line = resolved.defaults.font.size * factor * resolved.defaults.line_height;
        let state = State {
            current_theme: theme,
            current_resolved: resolved,
            accessibility: native_theme_iced::AccessibilityPreferences {
                text_scaling_factor: factor,
                ..native_theme_iced::AccessibilityPreferences::default()
            },
            active_tab: Tab::Buttons,
            ..State::default()
        };
        let mut ui = interface(&state);
        let label = height_of(&mut ui, "Primary");
        assert!(
            (label - button_line).abs() < 0.01,
            "at {factor}: a button label is {label}px tall, button.font scaled gives {button_line}px"
        );
        let body = height_of(&mut ui, "Interactive button styles from the resolved theme");
        assert!(
            (body - body_line).abs() < 0.01,
            "at {factor}: a page description is {body}px tall, defaults.font scaled gives {body_line}px"
        );
    }

    /// A font database of upright faces, each a family at a CSS weight and
    /// whether it is monospaced. The faces carry no data: choosing one reads
    /// only what fontdb files about it.
    fn database(faces: &[(&str, u16, bool)]) -> fontdb::Database {
        let mut db = fontdb::Database::new();
        for &(family, weight, monospaced) in faces {
            db.push_face_info(fontdb::FaceInfo {
                id: fontdb::ID::dummy(),
                source: fontdb::Source::Binary(Arc::new(Vec::<u8>::new())),
                index: 0,
                families: vec![(family.to_string(), fontdb::Language::English_UnitedStates)],
                post_script_name: format!("{family}-{weight}"),
                style: fontdb::Style::Normal,
                weight: fontdb::Weight(weight),
                stretch: fontdb::Stretch::Normal,
                monospaced,
            });
        }
        db
    }

    /// A family the font database holds is drawn in, at the theme's weight
    /// where the family has a face of it.
    #[test]
    fn a_held_family_keeps_the_theme_family_and_weight() {
        let db = database(&[("Theme Sans", 400, false), ("Theme Sans", 700, false)]);
        let font = drawable_font(&db, &[], "Theme Sans", 700, false).font;
        assert_eq!(
            (font.family, font.weight),
            (
                iced::font::Family::Name("Theme Sans"),
                iced::font::Weight::Bold
            )
        );
    }

    /// A weight the family has no face of is drawn at the family's nearest
    /// one, never in a later fallback family that has it: that is how
    /// macOS's system family lost its bold to Menlo Bold.
    #[test]
    fn a_weight_the_family_lacks_stays_in_the_family() {
        let db = database(&[("Theme Sans", 400, false), ("Mono Fallback", 700, true)]);
        let font = drawable_font(&db, &["Mono Fallback"], "Theme Sans", 700, false).font;
        assert_eq!(
            (font.family, font.weight),
            (
                iced::font::Family::Name("Theme Sans"),
                iced::font::Weight::Normal
            )
        );
    }

    /// A family the database does not hold is drawn in iced's generic one,
    /// and the family that resolves to decides the weight: the database's
    /// sans-serif family when it holds it, else the first platform fallback
    /// it holds.
    #[test]
    fn an_absent_family_is_drawn_in_the_generic_one() {
        let fallbacks = ["System Sans", "Mono Fallback"];
        let db = database(&[
            ("System Sans", 400, false),
            ("Mono Fallback", 400, true),
            ("Mono Fallback", 700, true),
        ]);
        let font = drawable_font(&db, &fallbacks, "Absent Sans", 700, false).font;
        assert_eq!(
            (font.family, font.weight),
            (iced::font::Family::SansSerif, iced::font::Weight::Normal)
        );

        let mut db = database(&[("Generic Sans", 400, false), ("Generic Sans", 700, false)]);
        db.set_sans_serif_family("Generic Sans");
        let font = drawable_font(&db, &fallbacks, "Absent Sans", 700, false).font;
        assert_eq!(
            (font.family, font.weight),
            (iced::font::Family::SansSerif, iced::font::Weight::Bold)
        );
    }

    /// A monospace family the database does not hold is drawn in iced's
    /// generic monospace one at the theme's weight: cosmic-text takes a
    /// monospaced face at any weight, the nearest first.
    #[test]
    fn an_absent_mono_family_keeps_its_weight() {
        let db = database(&[("Mono Fallback", 400, true)]);
        let font = drawable_font(&db, &[], "Absent Mono", 700, true).font;
        assert_eq!(
            (font.family, font.weight),
            (iced::font::Family::Monospace, iced::font::Weight::Bold)
        );
    }

    /// A weight between iced's nine is asked for at the nearest of them, and
    /// a face iced cannot ask for, at a weight between them, is passed over
    /// for the nearest face it can.
    #[test]
    fn weights_are_ones_iced_can_ask_for() {
        let db = database(&[("Theme Sans", 400, false), ("Theme Sans", 700, false)]);
        let font = drawable_font(&db, &[], "Theme Sans", 350, false).font;
        assert_eq!(font.weight, iced::font::Weight::Normal);

        let db = database(&[("Theme Sans", 350, false), ("Theme Sans", 600, false)]);
        let font = drawable_font(&db, &[], "Theme Sans", 400, false).font;
        assert_eq!(font.weight, iced::font::Weight::Semibold);
    }

    /// A variable face whose `wght` axis covers a weight its family has no
    /// face of gains one at that weight, over the same font data.
    #[test]
    fn a_variable_face_gains_the_weights_its_axis_covers() {
        let mut db = database(&[("Theme Sans", 400, false)]);
        let filed = register_weight(&mut db, "Theme Sans", 700, |_, _| Some((100.0, 900.0)));
        assert!(filed, "no face was filed");
        let font = drawable_font(&db, &[], "Theme Sans", 700, false).font;
        assert_eq!(
            (font.family, font.weight),
            (
                iced::font::Family::Name("Theme Sans"),
                iced::font::Weight::Bold
            )
        );
        let sources: Vec<_> = upright_faces(&db, "Theme Sans")
            .map(|face| (face.post_script_name.as_str(), face.index))
            .collect();
        assert_eq!(sources, [("Theme Sans-400", 0), ("Theme Sans-400", 0)]);
    }

    /// An axis that stops short of the weight files nothing, and the text
    /// stays at the family's nearest face.
    #[test]
    fn an_axis_short_of_the_weight_files_nothing() {
        let mut db = database(&[("Theme Sans", 400, false)]);
        let filed = register_weight(&mut db, "Theme Sans", 700, |_, _| Some((400.0, 600.0)));
        assert!(!filed, "a face was filed beyond the axis");
        assert_eq!(upright_faces(&db, "Theme Sans").count(), 1);
    }

    /// A static font files nothing: the axis is read from real font data,
    /// the Fira Sans Regular that `iced_test` has iced bundle (`iced_graphics`
    /// `text.rs:111-113`), which parses as a font and has no `wght` axis.
    #[test]
    fn a_static_face_files_nothing() {
        let data = iced::advanced::graphics::text::FIRA_SANS_REGULAR;
        assert!(
            cosmic_text::skrifa::FontRef::new(data).is_ok(),
            "the fixture does not parse as a font"
        );
        let mut db = fontdb::Database::new();
        db.load_font_data(data.to_vec());
        let family = match db.faces().next().and_then(|face| face.families.first()) {
            Some((name, _)) => name.clone(),
            None => panic!("the fixture holds no named face"),
        };
        let weights = |db: &fontdb::Database| -> Vec<u16> {
            upright_faces(db, &family)
                .map(|face| face.weight.0)
                .collect()
        };
        let before = weights(&db);
        assert!(
            !before.contains(&700),
            "the fixture already has a bold face"
        );
        let filed = register_weight(&mut db, &family, 700, weight_axis);
        assert!(!filed, "a static face gained a weight");
        assert_eq!(weights(&db), before);
    }

    /// A family the database holds only at weights iced cannot ask for is
    /// passed over for the generic family, unless a variable face of it
    /// covers the weight asked for.
    #[test]
    fn a_family_held_only_between_iced_weights_is_drawn_in_the_generic_one() {
        let fallbacks = ["System Sans"];
        let mut db = database(&[("Theme Sans", 350, false), ("System Sans", 400, false)]);
        let drawn = drawable_font(&db, &fallbacks, "Theme Sans", 700, false);
        assert_eq!(
            (drawn.font.family, drawn.font.weight, drawn.family),
            (
                iced::font::Family::SansSerif,
                iced::font::Weight::Normal,
                Some("System Sans")
            )
        );
        assert!(register_weight(&mut db, "Theme Sans", 700, |_, _| Some((
            100.0, 900.0
        ))));
        let font = drawable_font(&db, &fallbacks, "Theme Sans", 700, false).font;
        assert_eq!(
            (font.family, font.weight),
            (
                iced::font::Family::Name("Theme Sans"),
                iced::font::Weight::Bold
            )
        );
    }

    /// A Widget Info row claims the family, and the weight, only where text
    /// is drawn in them, and otherwise says what is drawn instead.
    #[test]
    fn a_font_row_claims_only_what_is_drawn() {
        let mut font = match native_theme_iced::from_preset("kde-breeze", false) {
            Ok((_, resolved)) => resolved.button.font.clone(),
            Err(error) => panic!("kde-breeze: {error}"),
        };
        font.family = "Theme Sans".into();
        font.weight = 700;
        let drawn = |family, weight, resolved| Drawn {
            font: iced::Font {
                family,
                weight,
                ..iced::Font::DEFAULT
            },
            family: resolved,
        };
        let held = drawn(
            iced::font::Family::Name("Theme Sans"),
            iced::font::Weight::Bold,
            Some("Theme Sans"),
        );
        assert_eq!(
            font_row_drawn("button.font", &font, &held),
            "button.font, family, size and weight"
        );
        let generic = drawn(
            iced::font::Family::SansSerif,
            iced::font::Weight::Normal,
            Some("System Sans"),
        );
        assert_eq!(
            font_row_drawn("button.font", &font, &generic),
            "button.font, size; family Theme Sans (not found; drawn in generic \
             sans-serif: System Sans); weight 700 (drawn at 400: System Sans has no 700 face)"
        );
    }

    /// A family is shown as the name alone where text is drawn in it, and
    /// with the generic family drawn instead, and what that resolves to,
    /// where it is not.
    #[test]
    fn a_substituted_family_says_so() {
        let drawn = |family, resolved| Drawn {
            font: iced::Font {
                family,
                ..iced::Font::DEFAULT
            },
            family: resolved,
        };
        let held = drawn(iced::font::Family::Name("Theme Sans"), Some("Theme Sans"));
        assert_eq!(
            family_label("Theme Sans", "Theme Sans", &held),
            "Theme Sans"
        );
        let generic = drawn(iced::font::Family::SansSerif, Some("System Sans"));
        assert_eq!(
            family_label("Theme Sans", "Theme Sans", &generic),
            "Theme Sans (not found; drawn in generic sans-serif: System Sans)"
        );
        let mono = drawn(iced::font::Family::Monospace, None);
        assert_eq!(
            family_label("Theme Mono", "Theme Mono", &mono),
            "Theme Mono (not found; drawn in generic monospace)"
        );
        let alias = drawn(iced::font::Family::Name(".SF NS"), Some(".SF NS"));
        assert_eq!(family_label("SF Pro", ".SF NS", &alias), "SF Pro");
    }

    /// A weight is shown as the number alone where it is drawn, and with
    /// what is drawn instead, and why, where it is not.
    #[test]
    fn a_substituted_weight_says_so() {
        let drawn = |weight| Drawn {
            font: iced::Font {
                weight,
                ..iced::Font::DEFAULT
            },
            family: Some("Theme Sans"),
        };
        assert_eq!(weight_label(700, &drawn(iced::font::Weight::Bold)), "700");
        assert_eq!(
            weight_label(700, &drawn(iced::font::Weight::Normal)),
            "700 (drawn at 400: Theme Sans has no 700 face)"
        );
        assert_eq!(
            weight_label(350, &drawn(iced::font::Weight::Normal)),
            "350 (drawn at 400, iced's nearest weight)"
        );
    }

    /// Every font the showcase builds comes from `drawable_font`, so role and
    /// widget text carries the theme's family, never iced's generic one
    /// picked outside it. `family_label` only reads the family a font has.
    #[test]
    fn fonts_are_built_from_the_theme_family() {
        let source = strip_comments_and_strings(SHOWCASE);
        let app = match source.find("mod tests {") {
            Some(end) => &source[..end],
            None => panic!("the test module is gone"),
        };
        let body = |name: &str| match app.find(name) {
            Some(start) => match app[start..].find("\n}") {
                Some(len) => start..start + len,
                None => panic!("{name} has no end"),
            },
            None => panic!("{name} is gone"),
        };
        let helpers = [body("fn drawable_font"), body("fn family_label")];
        let mut generic = Vec::new();
        for token in [
            "Font::DEFAULT",
            "Font::MONOSPACE",
            "Family::",
            "Font {",
            "weighted(",
        ] {
            for (at, _) in app.match_indices(token) {
                // A function returning a font opens its body after the type.
                let return_type = app[..at].trim_end_matches("iced::").ends_with("-> ");
                if !helpers.iter().any(|helper| helper.contains(&at)) && !return_type {
                    generic.push(format!("{token} at :{}", line_at(&source, at)));
                }
            }
        }
        assert!(
            generic.is_empty(),
            "a font built outside drawable_font: {generic:#?}"
        );
    }

    /// A captured window opens at `WINDOW_SIZE`, centred, as `main`'s own
    /// does, and on Linux under an app id no desktop stored a geometry for.
    #[test]
    fn a_capture_opens_at_the_default_size() {
        let settings = capture_window_settings();
        assert_eq!(settings.size, Size::new(WINDOW_SIZE.0, WINDOW_SIZE.1));
        assert!(matches!(
            settings.position,
            iced::window::Position::Centered
        ));
        #[cfg(target_os = "linux")]
        assert_eq!(
            settings.platform_specific.application_id,
            format!("showcase-iced-capture-{}", std::process::id())
        );
    }

    /// A capture passes only when the window's content is `WINDOW_SIZE` at
    /// the display's scale factor, with or without the frame around it.
    #[test]
    fn a_capture_of_another_size_fails() {
        assert_eq!(check_content_capture((1280, 720), 1.0), Ok(()));
        assert_eq!(check_content_capture((2560, 1440), 2.0), Ok(()));
        assert!(check_content_capture((1060, 750), 1.0).is_err());
        // A frame 2px wider and 32px taller than the content, as the Windows
        // runner's captures measured at the old default size (1062 x 782
        // around a 1060 x 750 content).
        assert_eq!(check_frame_capture((1282, 752), (1280, 720), 1.0), Ok(()));
        // A 1024px-wide display clamped the window.
        let clamped = check_frame_capture((1024, 674), (1024, 646), 1.0);
        assert!(
            clamped
                .as_ref()
                .is_err_and(|e| e.contains("1024x674") && e.contains("1280x748")),
            "{clamped:?}"
        );
    }

    /// The page tabs scroll sideways with no bar of their own, so no bar
    /// covers their labels wherever the tabs overflow the strip: the strip is
    /// exactly as tall as its tabs. Every tab set overflows a window 512px
    /// wide, so the check always runs.
    #[test]
    fn the_page_tabs_scroll_with_no_bar_over_their_labels() {
        let mut scrolled = 0;
        for preset in ["macos-sonoma", "kde-breeze"] {
            for width in [1024.0, 512.0] {
                let (theme, resolved) = match native_theme_iced::from_preset(preset, false) {
                    Ok(installed) => installed,
                    Err(error) => panic!("{preset}: {error}"),
                };
                let state = State {
                    current_theme: theme,
                    current_resolved: resolved,
                    active_tab: Tab::Buttons,
                    side_panel_visible: false,
                    ..State::default()
                };
                let mut ui: Simulator<'_, Message> = Simulator::with_size(
                    Settings::default(),
                    Size::new(width, WINDOW_SIZE.1),
                    view(&state),
                );
                let (strip, tabs) = match ui.find(selector::id(TAB_STRIP_ID)) {
                    Ok(selector::Target::Scrollable {
                        bounds,
                        content_bounds,
                        ..
                    }) => (bounds, content_bounds),
                    Ok(other) => {
                        panic!("{preset}: {TAB_STRIP_ID} is not a scrollable: {other:?}")
                    }
                    Err(error) => panic!("{preset}: {TAB_STRIP_ID}: {error}"),
                };
                if tabs.width <= strip.width {
                    continue;
                }
                scrolled += 1;
                assert!(
                    (strip.height - tabs.height).abs() < 0.01,
                    "{preset} at {width}px: the strip is {}px tall around {}px of tabs",
                    strip.height,
                    tabs.height
                );
            }
        }
        assert!(
            scrolled > 0,
            "the tabs overflowed no strip, so none scrolled"
        );
    }

    /// The side panel keeps its width, and the page beside it its place,
    /// whatever Widget Info holds: on kde-breeze, whose scrollbar does not
    /// overlay, the panel's bar is laid out inside the panel, where round 2
    /// found it widening the panel by `scrollbar.groove_width` and moving the
    /// page.
    #[test]
    fn the_side_panel_scrollbar_moves_nothing() {
        let (theme, resolved) = match native_theme_iced::from_preset("kde-breeze", false) {
            Ok(installed) => installed,
            Err(error) => panic!("kde-breeze: {error}"),
        };
        assert!(
            !resolved.scrollbar.overlay_mode,
            "kde-breeze's scrollbar no longer takes room of its own; pick a preset whose does"
        );
        let long: String = std::iter::once("Tall info".to_string())
            .chain((0..200).map(|i| format!("  row {i}: a value")))
            .collect::<Vec<_>>()
            .join("\n");
        let mut drawn = Vec::new();
        for info in [String::new(), long] {
            let state = State {
                current_theme: theme.clone(),
                current_resolved: resolved.clone(),
                widget_info: info,
                ..State::default()
            };
            let mut ui: Simulator<'_, Message> =
                Simulator::with_size(Settings::default(), Size::from(WINDOW_SIZE), view(&state));
            drawn.push((
                probe_bounds(&mut ui, probes::SIDE_PANEL),
                probe_bounds(&mut ui, probes::BASIC_BUTTON),
            ));
        }
        for (panel, _) in &drawn {
            assert_eq!(panel.width, LEFT_PANEL_WIDTH, "the side panel's width");
        }
        assert_eq!(
            drawn[0].1.x, drawn[1].1.x,
            "the page moved when Widget Info filled"
        );
    }

    /// A key press: `character` with Ctrl (Cmd on macOS), or Escape where
    /// it is `None`.
    fn key_press(character: Option<&str>) -> iced::keyboard::Event {
        use iced::keyboard::{Key, Location, Modifiers, key};
        let (key, modifiers) = match character {
            Some(c) => (Key::Character(c.into()), Modifiers::COMMAND),
            None => (Key::Named(key::Named::Escape), Modifiers::empty()),
        };
        iced::keyboard::Event::KeyPressed {
            key: key.clone(),
            modified_key: key,
            physical_key: key::Physical::Unidentified(key::NativeCode::Unidentified),
            location: Location::Standard,
            modifiers,
            text: None,
            repeat: false,
        }
    }

    /// The chrome does what the gpui showcase's does: the key bindings the
    /// menus show send their actions, the status bar's toggle hides and shows
    /// the side panel, the inspector's tabs switch its view, the command
    /// palette finds a page by its name and shows it, closing itself, and
    /// Escape closes a dialog.
    #[test]
    fn the_chrome_responds() {
        assert!(matches!(
            shortcut(key_press(Some("q"))),
            Some(Message::Quit)
        ));
        assert!(matches!(
            shortcut(key_press(Some("b"))),
            Some(Message::ToggleSidePanel)
        ));
        assert!(matches!(
            shortcut(key_press(Some("k"))),
            Some(Message::Open(Overlay::CommandPalette))
        ));
        assert!(matches!(
            shortcut(key_press(Some(","))),
            Some(Message::Open(Overlay::Preferences))
        ));
        assert!(matches!(
            shortcut(key_press(None)),
            Some(Message::CloseOverlay)
        ));
        assert!(shortcut(key_press(Some("x"))).is_none());

        let mut state = State::default();
        let _ = update(&mut state, Message::ToggleSidePanel);
        assert!(!state.side_panel_visible, "the side panel is still shown");
        let _ = update(&mut state, Message::ToggleSidePanel);
        assert!(state.side_panel_visible, "the side panel is still hidden");

        // The last "Theme" on screen is the inspector's tab: the menu bar's
        // title and the settings' label come before it.
        let messages = drive(&mut state, |ui| click_last_text(ui, "Theme"));
        assert!(
            matches!(
                messages.as_slice(),
                [Message::InspectorTabSelected(InspectorTab::Theme)]
            ),
            "inspector tabs: {messages:?}"
        );
        assert_eq!(state.inspector_tab, InspectorTab::Theme);

        let _ = update(&mut state, Message::Open(Overlay::CommandPalette));
        assert_eq!(state.overlay, Some(Overlay::CommandPalette));
        let _ = drive(&mut state, |ui| {
            click_probe(ui, PALETTE_QUERY_ID);
            let _ = ui.typewrite("Icons");
        });
        assert_eq!(
            state.palette_query, "Icons",
            "the query did not reach the state"
        );
        // The palette is laid over the window, so its entry comes last.
        let messages = drive(&mut state, |ui| click_last_text(ui, "Icons"));
        assert!(
            matches!(
                messages.as_slice(),
                [Message::PaletteRun(action)] if matches!(**action, Message::TabSelected(Tab::Icons))
            ),
            "command palette: {messages:?}"
        );
        assert_eq!(state.active_tab, Tab::Icons, "the palette showed no page");
        assert_eq!(state.overlay, None, "the palette stayed open");

        let _ = update(&mut state, Message::Open(Overlay::About));
        let _ = update(&mut state, Message::CloseOverlay);
        assert_eq!(state.overlay, None, "Escape left the dialog open");
    }

    /// Every text-bearing widget is given its size from the theme, in the
    /// font at the weight that comes with it: a `text` its role, its body font
    /// or its widget's font; a `button` a `text` of its own, never a bare
    /// label; the labelled controls and the fields their widget's font. The
    /// one exception is the "?" that stands in for an icon, at the icon's size.
    #[test]
    fn text_bearing_widgets_are_sized_from_the_theme() {
        let source = strip_comments_and_strings(SHOWCASE);
        let mut by_iced = Vec::new();
        let chained = |site: usize| -> Vec<&str> {
            match close_of_call(&source, site) {
                Some(end) => method_calls(&source, end)
                    .into_iter()
                    .map(|(name, _)| name)
                    .collect(),
                None => Vec::new(),
            }
        };
        for site in call_sites(&source, "text") {
            let calls = match close_of_call(&source, site) {
                Some(end) => method_calls(&source, end),
                None => Vec::new(),
            };
            // The one glyph in an icon's slot, `placeholder_icon`'s "?", is
            // sized as the icon.
            let sized = calls.iter().any(|(name, args)| {
                ["role", "body", "typeset", "themed"].contains(name)
                    || (*name == "size" && *args == "(icon_px)")
            });
            if !sized {
                by_iced.push(format!("text at :{}", line_at(&source, site)));
            }
        }
        for site in call_sites(&source, "button") {
            let label = match close_of_call(&source, site) {
                Some(end) => source[site + "button(".len()..end].trim_start(),
                None => "",
            };
            // A label under the button's minimum size is still a text, and a
            // link's underlined label is rich text, sized below. An expander
            // header's label is a row of its arrow and its title, a text the
            // loop above sizes.
            let label = label
                .strip_prefix("at_least(")
                .unwrap_or(label)
                .trim_start();
            // A label tagged for the layout dump (`tagged(id, ..)`) is the
            // label it wraps.
            let label = label
                .strip_prefix("tagged(")
                .and_then(|rest| rest.split_once(','))
                .map_or(label, |(_, wrapped)| wrapped.trim_start());
            let composite = label.starts_with("row![") && label.contains("text(");
            // A chrome icon button's content is its icon (`chrome_icon`), and
            // the page tabs' menu button's is its caret icon, or where the
            // icon theme has none a text the loop above sizes.
            let icon = label == "icon)" || label == "caret)";
            if !label.starts_with("text(")
                && !label.starts_with("rich_text(")
                && !composite
                && !icon
            {
                by_iced.push(format!("button at :{}", line_at(&source, site)));
            }
        }
        let receivers: [(&str, &[&str]); 11] = [
            ("rich_text", &["size", "font"]),
            ("checkbox", &["text_size", "font"]),
            ("radio", &["text_size", "font"]),
            ("toggler", &["text_size", "font"]),
            ("pick_list", &["text_size", "font"]),
            ("text_input", &["size", "font"]),
            ("combo_box", &["size", "font"]),
            ("text_editor", &["size", "font"]),
            ("TabBar::new", &["text_size", "text_font"]),
            ("Tabs::new", &["text_size", "text_font"]),
            ("Sidebar::new", &["text_size", "text_font"]),
        ];
        for (ctor, setters) in receivers {
            for site in call_sites(&source, ctor) {
                let calls = chained(site);
                // A switch with no label, as the Preferences dialog's, whose
                // row titles it, draws no text.
                if ctor == "toggler" && !calls.contains(&"label") {
                    continue;
                }
                for setter in setters {
                    if !calls.contains(setter) {
                        by_iced.push(format!(
                            "{ctor} at :{} has no {setter}",
                            line_at(&source, site)
                        ));
                    }
                }
            }
        }
        assert!(
            by_iced.is_empty(),
            "sized by iced, not the theme: {by_iced:#?}"
        );
    }

    /// The names `let` binds to a padding one of `PADDING_HELPERS` returns.
    fn theme_padding_bindings(source: &str) -> Vec<&str> {
        let mut names = Vec::new();
        for (at, _) in source.match_indices("let ") {
            let joined_before = source[..at]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
            if joined_before {
                continue;
            }
            let rest = &source[at + 4..];
            let rest = rest.strip_prefix("mut ").unwrap_or(rest);
            let name_len = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .count();
            if name_len == 0 {
                continue;
            }
            let name_at = source.len() - rest.len();
            let mut depth = 0i32;
            let mut end = source.len();
            for (offset, c) in source[name_at..].char_indices() {
                match c {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => {
                        depth -= 1;
                        if depth < 0 {
                            end = name_at + offset;
                            break;
                        }
                    }
                    ';' if depth == 0 => {
                        end = name_at + offset;
                        break;
                    }
                    _ => {}
                }
            }
            let statement = &source[name_at..end];
            if PADDING_HELPERS
                .iter()
                .any(|helper| statement.contains(helper))
            {
                names.push(&rest[..name_len]);
            }
        }
        names
    }

    /// The names of the `pub fn` items a module declares.
    fn public_functions(source: &str) -> Vec<String> {
        strip_comments_and_strings(source)
            .lines()
            .filter_map(|line| line.strip_prefix("pub fn "))
            .filter_map(|rest| rest.split(['(', '<']).next())
            .map(str::to_string)
            .collect()
    }

    /// Rust source with `//` comments, `/* */` comments and string, raw
    /// string and character literals removed, so that a widget named in prose
    /// is not counted as one built.
    ///
    /// A `'` opens a literal only when what follows it closes one; otherwise
    /// it is a lifetime or a loop label (`'a`, `'_`, `'static`), which is code
    /// and stays. Byte literals need no state of their own: `b` is an
    /// identifier character, and what follows it is lexed like any other
    /// string or character. Every removed character becomes a space, which
    /// keeps line numbers and offsets the same as the original's.
    fn strip_comments_and_strings(source: &str) -> String {
        #[derive(Clone, Copy, PartialEq)]
        enum Mode {
            Code,
            Line,
            Block(usize),
            Text,
            Raw(usize),
        }

        let bytes: Vec<char> = source.chars().collect();
        let mut out = String::with_capacity(source.len());
        let mut mode = Mode::Code;
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i];
            let next = bytes.get(i + 1).copied();
            match mode {
                Mode::Code => {
                    if c == '/' && next == Some('/') {
                        mode = Mode::Line;
                        out.push(' ');
                    } else if c == '/' && next == Some('*') {
                        mode = Mode::Block(1);
                        out.push(' ');
                    } else if let Some(hashes) = raw_string_open(&bytes, i) {
                        mode = Mode::Raw(hashes);
                        let opener = hashes + 2;
                        out.extend(std::iter::repeat_n(' ', opener));
                        i += opener;
                        continue;
                    } else if c == '"' {
                        mode = Mode::Text;
                        out.push(' ');
                    } else if let Some(len) = char_literal_len(&bytes, i) {
                        out.extend(std::iter::repeat_n(' ', len));
                        i += len;
                        continue;
                    } else {
                        out.push(c);
                    }
                }
                Mode::Line => {
                    if c == '\n' {
                        mode = Mode::Code;
                        out.push(c);
                    } else {
                        out.push(' ');
                    }
                }
                Mode::Block(depth) => {
                    if c == '/' && next == Some('*') {
                        mode = Mode::Block(depth + 1);
                        out.push(' ');
                        out.push(' ');
                        i += 2;
                        continue;
                    } else if c == '*' && next == Some('/') {
                        mode = if depth == 1 {
                            Mode::Code
                        } else {
                            Mode::Block(depth - 1)
                        };
                        out.push(' ');
                        out.push(' ');
                        i += 2;
                        continue;
                    }
                    out.push(if c == '\n' { c } else { ' ' });
                }
                Mode::Text => {
                    if c == '\\' {
                        out.push(' ');
                        out.push(if next == Some('\n') { '\n' } else { ' ' });
                        i += 2;
                        continue;
                    } else if c == '"' {
                        mode = Mode::Code;
                    }
                    out.push(if c == '\n' { c } else { ' ' });
                }
                Mode::Raw(hashes) => {
                    if c == '"' && (1..=hashes).all(|k| bytes.get(i + k) == Some(&'#')) {
                        mode = Mode::Code;
                        let closer = hashes + 1;
                        out.extend(std::iter::repeat_n(' ', closer));
                        i += closer;
                        continue;
                    }
                    out.push(if c == '\n' { c } else { ' ' });
                }
            }
            i += 1;
        }
        out
    }

    /// The number of `#`s of the raw string opener at `at`, if one starts
    /// there.
    ///
    /// `at` is the `r`; a `b` in front of it is a prefix of the same literal
    /// and not the tail of an identifier, so `br"..."` is a raw string while
    /// `str"` — which no Rust ever writes — is not.
    fn raw_string_open(source: &[char], at: usize) -> Option<usize> {
        if source.get(at) != Some(&'r') {
            return None;
        }
        let before = at.checked_sub(1).and_then(|k| source.get(k)).copied();
        let head = if before == Some('b') {
            at.checked_sub(2).and_then(|k| source.get(k)).copied()
        } else {
            before
        };
        if head.is_some_and(|c| c.is_alphanumeric() || c == '_') {
            return None;
        }
        let mut hashes = 0;
        while source.get(at + 1 + hashes) == Some(&'#') {
            hashes += 1;
        }
        (source.get(at + 1 + hashes) == Some(&'"')).then_some(hashes)
    }

    /// The length of the character literal at `at`, if one starts there.
    ///
    /// A `'` that opens no literal is a lifetime or a loop label, which the
    /// stripper leaves alone. The escaped form is scanned to its closing
    /// quote, which is at most twelve characters away — `'\u{10FFFF}'` is the
    /// longest a character literal gets.
    fn char_literal_len(source: &[char], at: usize) -> Option<usize> {
        if source.get(at) != Some(&'\'') {
            return None;
        }
        match source.get(at + 1)? {
            '\\' => {
                let end = (at + 3..(at + 12).min(source.len()))
                    .find(|k| source.get(*k) == Some(&'\''))?;
                Some(end + 1 - at)
            }
            _ => (source.get(at + 2) == Some(&'\'')).then_some(3),
        }
    }

    /// The stripper, on a fixture of everything that has ever confused one.
    ///
    /// The fixture is a raw string, so the stripper's own source carries the
    /// forms it is asked about; that a `thing(` inside it is *not* counted is
    /// half of what this asserts.
    #[test]
    fn the_stripper_blanks_every_literal_and_keeps_every_lifetime() {
        let fixture = r##"
let quote = '"';
let escaped = '\'';
let slash = '\\';
let newline = '\n';
fn f<'a>(s: &'a str) -> &'a str { s }   // a " inside a line comment
let text = "a // b /* c */ d";
let continued = "first line \
                 second line";
/* outer /* inner */ still a comment */
'outer: loop { break 'outer; }
let raw = r"first";
let hashed = r#"a "quoted" thing(  "#;
thing(0);
"##;
        let stripped = strip_comments_and_strings(fixture);

        assert_eq!(
            stripped.chars().count(),
            fixture.chars().count(),
            "a removed character must become a space, or offsets move"
        );
        assert_eq!(
            stripped.lines().count(),
            fixture.lines().count(),
            "a removed newline moves every line number after it"
        );

        for code in [
            "let quote =",
            "let escaped =",
            "let slash =",
            "let newline =",
            "fn f<'a>(s: &'a str) -> &'a str { s }",
            "'outer: loop { break 'outer; }",
            "let hashed =",
            "let continued =",
        ] {
            assert!(stripped.contains(code), "the stripper ate code: {code}");
        }
        assert_eq!(
            stripped.matches('\'').count(),
            5,
            "the only quotes left are the three lifetimes and the two labels"
        );

        for gone in [
            "\"",
            "//",
            "/*",
            "*/",
            "quoted",
            "inside a line comment",
            "still a comment",
            "second line",
        ] {
            assert!(
                !stripped.contains(gone),
                "the stripper left a literal or a comment behind: {gone}"
            );
        }

        let sites = call_sites(&stripped, "thing");
        assert_eq!(
            sites.len(),
            1,
            "one `thing(` is code and one is inside a raw string, got {sites:?}"
        );
    }

    /// Every place `name(` is called, as an offset into `source`.
    ///
    /// A call is only a call when nothing joins it to what precedes it, so
    /// `styles::slider(` is not a `slider(` site and `vertical_slider(` is not
    /// one either.
    ///
    /// That rule is what the coverage count rests on: a constructor is seen
    /// only under its bare imported name, and a site written
    /// `iced::widget::button(` would be counted by nothing.
    /// `styles_cover_every_widget_shown` asserts that the showcase contains no
    /// such qualified form, so the invariant fails loudly instead of quietly
    /// shrinking the count.
    fn call_sites(source: &str, name: &str) -> Vec<usize> {
        let mut sites = Vec::new();
        for (at, _) in source.match_indices(name) {
            let joined_before = source[..at]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == ':' || c == '.');
            if joined_before {
                continue;
            }
            let rest = &source[at + name.len()..];
            if rest.starts_with('(') {
                sites.push(at);
            }
        }
        sites
    }

    /// Does the widget built at `site` get one of `styles` put on it?
    ///
    /// The style normally sits in the method chain that follows the
    /// constructor. The tab strip builds its button once and dresses it in the
    /// two arms of an `if`, so a constructor bound to a name is followed
    /// through that name as well — which is why a file-wide count of
    /// `button(` against `styles::button*` does not balance (14 against 15)
    /// while every one of the 14 is dressed.
    fn is_dressed(source: &str, site: usize, styles: &[&str]) -> bool {
        let after = match close_of_call(source, site) {
            Some(end) => end,
            None => return false,
        };
        if styles.iter().any(|style| {
            method_chain(source, after)
                .iter()
                .any(|call| !call_sites(call, style).is_empty())
        }) {
            return true;
        }
        match binding_of(source, site) {
            Some(name) => {
                let block = &source[site..block_end(source, site)];
                let handle = format!("{name}.");
                block.match_indices(&handle).any(|(at, _)| {
                    let window = &block[at..block.len().min(at + 200)];
                    styles
                        .iter()
                        .any(|style| !call_sites(window, style).is_empty())
                })
            }
            None => false,
        }
    }

    /// The offset just past the `)` that closes the call starting at `site`.
    fn close_of_call(source: &str, site: usize) -> Option<usize> {
        let open = source[site..].find('(')? + site;
        let mut depth = 0usize;
        for (offset, c) in source[open..].char_indices() {
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(open + offset + 1);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// The arguments of every `.method(..)` chained onto the value that ends
    /// at `from`.
    fn method_chain(source: &str, from: usize) -> Vec<&str> {
        method_calls(source, from)
            .into_iter()
            .map(|(_, args)| args)
            .collect()
    }

    /// Every `.method(..)` chained onto the value that ends at `from`: its
    /// name, and its arguments with their parentheses.
    fn method_calls(source: &str, from: usize) -> Vec<(&str, &str)> {
        let mut calls = Vec::new();
        let mut at = from;
        loop {
            let rest = &source[at..];
            let skipped = rest.len() - rest.trim_start().len();
            at += skipped;
            if !source[at..].starts_with('.') {
                return calls;
            }
            let name_start = at + 1;
            let name_len = source[name_start..]
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .count();
            let open = name_start + name_len;
            if name_len == 0 || !source[open..].starts_with('(') {
                return calls;
            }
            match close_of_call(source, open) {
                Some(end) => {
                    calls.push((&source[name_start..open], &source[open..end]));
                    at = end;
                }
                None => return calls,
            }
        }
    }

    /// The name a `let` binds the call at `site` to, if it binds one.
    fn binding_of(source: &str, site: usize) -> Option<&str> {
        let head = &source[..site];
        let start = head.rfind(['{', '}', ';']).map(|at| at + 1).unwrap_or(0);
        let statement = head[start..].trim_start();
        let rest = statement.strip_prefix("let ")?;
        let rest = rest.strip_prefix("mut ").unwrap_or(rest);
        let name_len = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .count();
        let name = &rest[..name_len];
        let tail = rest[name_len..].trim_start();
        if name.is_empty() || !tail.starts_with('=') {
            return None;
        }
        Some(name)
    }

    /// Where the block that encloses `site` ends.
    fn block_end(source: &str, site: usize) -> usize {
        let mut depth = 0i32;
        for (offset, c) in source[site..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    if depth == 0 {
                        return site + offset;
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }
        source.len()
    }

    /// The 1-based line the given offset sits on.
    fn line_at(source: &str, at: usize) -> usize {
        source[..at].lines().count().max(1)
    }
}
