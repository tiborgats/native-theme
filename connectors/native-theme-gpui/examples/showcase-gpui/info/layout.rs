//! What the Layout page's widgets report about themselves (spec §3.4).

use gpui::Pixels;
use gpui_component::theme::Theme;
use native_theme::next::{ResolvedTheme as NextTheme, WidgetState};
#[cfg(feature = "widgets")]
use native_theme::theme::TabIndicatorSide;
use native_theme_gpui::next::{
    fill_of, padding_of, state_fill_of, state_text_color_of, text_color_of,
};
#[cfg(feature = "widgets")]
use native_theme_gpui::widgets::{SeparatorLook, TabLook};

use super::{
    WidgetInfo,
    chrome::{GhostContent, chrome_icon_note, ghost_colours},
    claim, edge_claim, icon_color_of, px_text, stated_claim, stroke_color_of, stroke_width_of,
};
use crate::Page;
use crate::demo::{
    ExpanderLook, FOLDER_TAB_HEIGHT, GroupBoxKind, SEGMENT_PADDING, SegmentedLook, SeparatorKind,
    SpacingBox, StepperKind,
};
use crate::support::{ChromeIcon, GRID_LAYOUT_ITEMS, SampleIcon, layout_value};

/// One of the Layout page's two spacing boxes, `kind`, padded by `padding`
/// and spacing its children by `gap`, each `None` where the platform states
/// none. Their geometry lines are recorded where `demo::spacing_box` applies
/// the values.
pub fn spacing_box(
    t: &Theme,
    kind: SpacingBox,
    padding: Option<Pixels>,
    gap: Option<Pixels>,
) -> WidgetInfo {
    let info = WidgetInfo::new("Layout spacing");
    let info = match kind {
        SpacingBox::Window => info
            .variant("window margin, section gap")
            .color(claim("frame", "border", t.border, "showcase"))
            .config("window_margin", layout_value(padding))
            .config("section_gap", layout_value(gap)),
        SpacingBox::Container => info
            .variant("container margin, widget gap")
            .color(claim("frame", "border", t.border, "showcase"))
            .config("container_margin", layout_value(padding))
            .config("widget_gap", layout_value(gap)),
    };
    let info = info.not_themeable("receivers", "none a theme can write: Theme::spacing_tokens() returns SpacingTokens::default() with no field behind it (gpui-component/theme/mod.rs, spacing_tokens), and its readers get that default: a Dialog's viewport margin (gpui-component/dialog/dialog.rs, Dialog), a GridLayout's gap (gpui-component/grid_layout.rs, grid_layout) and, through Theme::semantic_tokens, a Questionnaire's metrics (gpui-component/questionnaire/components.rs, QuestionnaireMetrics). So these four are the application's to apply (geometry.rs §9.5)");
    match kind {
        SpacingBox::Window => info.instance(
            "holds",
            "the row below and the summary under it: the window margin is the padding inside this frame, and the section gap the space between the two",
        ),
        SpacingBox::Container => info.instance(
            "holds",
            "three Default Buttons, which report themselves: the container margin is the padding inside this frame, and the widget gap the space between the Buttons",
        ),
    }
    .instance(
        "frame",
        "the showcase's own frame, not a widget of gpui-component's: Theme::border at the platform's border line width, rounded with Theme::radius",
    )
}

/// The room above and below a Separator sample, which is
/// `SEPARATOR_SAMPLE_ROOM`.
fn sample_room(t: &Theme) -> String {
    format!(
        "{} above and below, SEPARATOR_SAMPLE_ROOM, the showcase's own choice rather than a platform's, as the theme states none: the sample's box is as tall as its line or label, and the room leaves something to point at (spec §3.1)",
        super::rems_text(crate::demo::SEPARATOR_SAMPLE_ROOM, t)
    )
}

/// A plain line drawn by `native_theme_gpui::widgets::Separator` in `look`,
/// the installed 0.8.0 theme's (`SeparatorLook::next_of`), with `room` above
/// and below where `demo::separator` gives it some.
#[cfg(feature = "widgets")]
pub fn native_separator(t: &Theme, look: &SeparatorLook, room: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Separator");
    let info = if room {
        info.not_themeable("room", sample_room(t))
    } else {
        info
    };
    info
        .variant(SeparatorKind::Horizontal.name())
        .config("drawn by", "native_theme_gpui::widgets::Separator, a box filled in the line's colour, as thick as the theme states and one device pixel at least: gpui-component's Separator draws its line an absolute child at px(1.), which no caller style reaches (separator.rs, Separator::render_base)")
        .color(claim(
            "line",
            "separator.color",
            look.color,
            "native-theme-gpui/widgets/separator.rs:36",
        ))
        .config(
            "thickness",
            format!(
                "separator.thickness: {}px",
                px_text(look.width.as_f32())
            ),
        )
}

/// A `Separator` of `kind`, its line `native`'s `separator.color` where a
/// 0.8.0 theme is installed (`demo::separator`).
pub fn separator(
    t: &Theme,
    kind: SeparatorKind,
    native: Option<&native_theme::next::ResolvedTheme>,
) -> WidgetInfo {
    let info = WidgetInfo::new("Separator").variant(kind.name());
    let info = match native {
        Some(r) => info.color(claim(
            "line",
            "separator.color",
            super::next_stated(r.separator.color),
            "showcase",
        )),
        None => info.color(claim(
            "line",
            "border",
            t.border,
            "gpui-component/separator.rs:128",
        )),
    };
    let info = info
        .not_themeable(
            "thickness",
            "Tier U, not an absence: the platform states separator.thickness and the model carries it. Upstream draws the line on an inner absolutely-positioned div at px(1.) and applies the caller's refinement to the outer container instead, so nothing reaches the line (separator.rs, Separator::render_base)",
        );
    let info = match kind {
        SeparatorKind::Vertical => info,
        _ => info.not_themeable("room", sample_room(t)),
    };
    match kind {
        SeparatorKind::Horizontal => info,
        SeparatorKind::Labelled(label) => info
            .color(claim(
                "label bg",
                "background",
                t.background,
                "gpui-component/separator.rs:149",
            ))
            .color(claim(
                "label text",
                "muted_foreground",
                t.muted_foreground,
                "gpui-component/separator.rs:150",
            ))
            .not_themeable(
                "label",
                "text_xs with px_2 / py_1 -- rems, so the platform's font -- on a box the caller's refinement does not reach: the Separator refines its outer container and builds the label as a child after it (gpui-component/separator.rs, Separator::render)",
            )
            .instance("label", label),
        SeparatorKind::Dashed => info.not_themeable(
            "dashes",
            "4px dashes with 2px gaps, stroked 1px wide -- literals (separator.rs, Separator::render_dashed)",
        ),
        SeparatorKind::Vertical => info.not_themeable(
            "length",
            "the height of the row it stands in: a vertical Separator is h_full (gpui-component/separator.rs, Separator::vertical)",
        ),
    }
}

/// Each side of a GroupBox's content padding, named by where it comes from:
/// the side
/// `card.container(NONE)` states (`next::geometry::group_box_content`),
/// else `margin`, the installed theme's `layout.container_margin`, which the
/// showcase pads an unstated side with (`demo::next_padded`), else `own`, the
/// variant's own.
fn next_content_padding(native: Option<&NextTheme>, margin: Option<Pixels>, own: &str) -> String {
    let side = |name: &str, stated: Option<f32>| match (native, stated, margin) {
        (Some(_), Some(v), _) => format!("{name} {}px, card.container(NONE)'s", px_text(v)),
        (Some(_), None, Some(m)) => {
            format!("{name} {}px, layout.container_margin", px_text(m.as_f32()))
        }
        _ => format!("{name} {own}"),
    };
    let [top, right, bottom, left] = native
        .map(|r| padding_of(&r.card.container(WidgetState::NONE)))
        .unwrap_or_default();
    format!(
        "{}; {}; {}; {}. A side card.container(NONE) leaves without padding takes layout.container_margin, the padding inside containers (platform-facts §2.20), where the installed theme states one, else the variant's own (group_box.rs, GroupBox)",
        side("top", top),
        side("right", right),
        side("bottom", bottom),
        side("left", left),
    )
}

/// `info` with the edge
/// `next::geometry::group_box_content` draws on a GroupBox's content of
/// `kind`, `card.container(NONE)`'s stroke, and its line: none where the
/// stroke is 0 wide, which an Outline box's own edge gives way to; where the
/// theme states no stroke, the builder sets none, and the box keeps
/// upstream's edge, an Outline box's `border_1` in `border` and none on a
/// Normal or Fill one.
fn next_card_edge(info: WidgetInfo, t: &Theme, r: &NextTheme, kind: GroupBoxKind) -> WidgetInfo {
    let container = r.card.container(WidgetState::NONE);
    let outline = kind == GroupBoxKind::Outline;
    let Some(width) = stroke_width_of(&container) else {
        return if outline {
            info.color(claim(
                "border",
                "border",
                t.border,
                "gpui-component/group_box.rs:146",
            ))
            .config("edge", "the Outline variant's own border_1 in border: the theme states no card.container(NONE) stroke, so geometry::group_box_content sets none on the content (group_box.rs, GroupBox)")
        } else {
            info.config("edge", "none: the theme states no card.container(NONE) stroke, so geometry::group_box_content sets none on the content, and a Normal or Fill box draws none of its own (group_box.rs, GroupBox)")
        };
    };
    info.colors(edge_claim(
        "edge",
        "card.container(NONE).stroke.color",
        stroke_color_of(&r.card.container(WidgetState::NONE)),
        Some(width),
        "native-theme-gpui/next/builders.rs:748",
    ))
    .config("edge", match (width > 0.0, outline) {
        (true, _) => format!("card.container(NONE)'s stroke, {}px, on every variant: geometry::group_box_content sets it on the content, which the GroupBox refines last (group_box.rs, GroupBox) -- over an Outline's own border colour, and on a Normal or Fill box, which draws no edge of its own", px_text(width)),
        (false, true) => "none: card.container(NONE)'s stroke is 0 wide, and geometry::group_box_content sets that width on the content, which the GroupBox refines last, so the Outline box's own border_1 gives way to it (group_box.rs, GroupBox)".to_string(),
        (false, false) => "none: card.container(NONE)'s stroke is 0 wide, so geometry::group_box_content draws no edge on the content, and a Normal or Fill box draws none of its own (group_box.rs, GroupBox)".to_string(),
    })
}

/// A `GroupBox` of `kind` titled `title`, on the 0.8.0 model.
/// `native` is the installed 0.8.0 theme `next::geometry::group_box_content`
/// refined its content from, where it did (`demo::group_box`), whose line
/// is recorded where `demo::group_box` applies the builder, and `margin`
/// that theme's `layout.container_margin`, which pads the sides the theme
/// leaves unstated. The caller adds what the content is.
pub fn next_group_box(
    t: &Theme,
    kind: GroupBoxKind,
    native: Option<&NextTheme>,
    margin: Option<Pixels>,
    title: &str,
) -> WidgetInfo {
    let info = WidgetInfo::new("GroupBox")
        .variant(kind.name())
        .color(claim(
            "title",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/group_box.rs:158",
        ))
        // A Label would paint foreground over this on its own element
        // (label.rs:213), so the content's text is plain text.
        .color(claim(
            "content text",
            "group_box_foreground",
            t.group_box_foreground,
            "gpui-component/group_box.rs:173",
        ));
    // (fill, edge) per variant (group_box.rs:143-147), under the card's
    // fill and edge where the builder refined the content.
    let info = match (kind, native) {
        (_, Some(r)) => next_card_edge(
            info.colors(stated_claim(
                "fill",
                "card.container(NONE).fill",
                fill_of(&r.card.container(WidgetState::NONE)),
                "native-theme-gpui/next/builders.rs:750",
            )),
            t,
            r,
            kind,
        ),
        (GroupBoxKind::Normal, None) => info,
        (GroupBoxKind::Fill, None) => info.color(claim(
            "bg",
            "group_box",
            t.group_box,
            "gpui-component/group_box.rs:145",
        )),
        (GroupBoxKind::Outline, None) => info.color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/group_box.rs:146",
        )),
    };
    let info = match (kind, native) {
        (_, Some(_)) => info,
        (GroupBoxKind::Outline, None) => {
            info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        }
        (GroupBoxKind::Normal | GroupBoxKind::Fill, None) => info
            .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
            .not_themeable(
                "edge",
                "none: only the Outline variant draws a border (group_box.rs, GroupBox)",
            ),
    };
    let info = match (kind, native) {
        (_, Some(_)) => info.config(
            "fill",
            "card.container(NONE).fill, on every variant: geometry::group_box_content sets it on the content, which the GroupBox refines last (group_box.rs, GroupBox) -- over a Fill box's group_box token, and on a Normal or Outline box, which paints no background of its own",
        ),
        (GroupBoxKind::Normal, None) => info.not_themeable(
            "fill",
            "none: the Normal variant paints no background (group_box.rs, GroupBox)",
        ),
        (GroupBoxKind::Fill | GroupBoxKind::Outline, None) => info,
    };
    let own = match kind {
        GroupBoxKind::Normal => "none, the Normal variant's own",
        GroupBoxKind::Fill => "p_4, the Fill variant's own",
        GroupBoxKind::Outline => "p_4, the Outline variant's own",
    };
    let info = match native {
        Some(_) => info.config("padding", next_content_padding(native, margin, own)),
        None => info.not_themeable("padding", next_content_padding(None, None, own)),
    };
    info.not_themeable(
        "gaps",
        "gap_3 between the title and the content on a Fill or Outline box and gap_4 on a Normal one -- rems, so the platform's font -- and settable: the GroupBox refines its outer box after them (group_box.rs, GroupBox)",
    )
    .instance("title", title.to_string())
}

/// The `Accordion`, drawn while gpui's `reduce_motion` is as given.
/// `refined` is `None` where no 0.8.0 theme is installed, else whether
/// `next::geometry::accordion_title` refined the titles and
/// `next::geometry::accordion` coloured the edges, whose geometry lines --
/// recorded where `demo::accordion` applies the builders -- then name the
/// colours; where a builder lacks a value, its part keeps upstream's colours
/// and its line says at which home (spec §10, rule 0).
pub fn next_accordion(
    t: &Theme,
    reduce_motion: bool,
    items: usize,
    refined: Option<(bool, bool)>,
) -> WidgetInfo {
    upstream_accordion(WidgetInfo::new("Accordion"), t, reduce_motion, refined)
        .instance("items", format!("{items}, the first open. Accordion::item takes a closure over an AccordionItem that the Accordion builds and renders itself (gpui-component/accordion.rs, Accordion::item), so an item cannot be wrapped: the Accordion reports for its items and their titles. Each answer is a Label that reports itself"))
}

/// The Basic page's two expanders where no native theme is installed:
/// upstream's Accordion of items titled `titles`, open as `open` says,
/// drawn while gpui's `reduce_motion` is as given (`demo::expander`).
pub fn expander(
    t: &Theme,
    reduce_motion: bool,
    titles: [&'static str; 2],
    open: [bool; 2],
) -> WidgetInfo {
    let state = |open: bool| if open { "open" } else { "closed" };
    let [first, second] = titles;
    let [first_open, second_open] = open;
    upstream_accordion(
        WidgetInfo::new("Accordion").variant("expanders"),
        t,
        reduce_motion,
        None,
    )
    .instance(
        "items",
        format!(
            "{first} ({}), {second} ({}); Accordion::item takes a closure over an AccordionItem the Accordion renders itself (gpui-component/accordion.rs, Accordion::item), so the Accordion reports for its items; each body is a Label that reports itself",
            state(first_open),
            state(second_open)
        ),
    )
    .instance("click", "a title opens its item and closes the other; the showcase keeps the state")
}

/// What upstream's Accordion draws of its own, onto `info`: the Layout
/// page's and, without a native theme, the Basic page's expanders.
/// `refined` as [`next_accordion`] takes it; `None` for the expanders.
fn upstream_accordion(
    info: WidgetInfo,
    t: &Theme,
    reduce_motion: bool,
    refined: Option<(bool, bool)>,
) -> WidgetInfo {
    let (titles, edges) = refined.unwrap_or_default();
    let info = info
        .not_themeable(
            "own icons",
            super::own_icons("ChevronDown on each item's header, turned while the item is open (gpui-component/accordion.rs, AccordionItem::render)"),
        )
        .color(claim(
            "bg",
            "accordion",
            t.accordion,
            "gpui-component/accordion.rs:381",
        ));
    let info = if edges {
        info
    } else {
        info.color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/accordion.rs:111",
        ))
        .color(claim(
            "line between items",
            "border",
            t.border,
            "gpui-component/accordion.rs:384",
        ))
    };
    let info = if titles {
        info
    } else {
        info.color(claim(
            "open item's title",
            "foreground",
            t.foreground,
            "gpui-component/accordion.rs:308",
        ))
        // A closed item's title row sets no colour (accordion.rs:306 is
        // the open one's), so it takes the colour the showcase sets on its
        // window.
        .color(claim(
            "closed item's title, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
    };
    let info = match refined {
        Some(_) => info,
        None => info.not_themeable("hover", "none: no native theme is installed, so the showcase gives the items no hover style (gpui-component/accordion.rs, AccordionItem::hover)"),
    };
    let styled = titles;
    info.color(claim(
            "chevron",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/accordion.rs:333",
        ))
        .config(
            "border-radius",
            format!("radius_lg: {}px", t.radius_lg.as_f32()),
        )
        .not_themeable("padding", if styled {
            "per-Size literals, py_2 / px_3 on the title row and pb_2 / px_3 under the content at the default Size -- rems -- each followed by the caller's title_style or content_style (gpui-component/accordion.rs, AccordionItem::render), so both are reachable. geometry::accordion_title sets only the title row's height and colour"
        } else {
            "per-Size literals, py_2 / px_3 on the title row and pb_2 / px_3 under the content at the default Size -- rems -- each followed by the caller's title_style or content_style (gpui-component/accordion.rs, AccordionItem::render), so both are reachable"
        })
        .not_themeable("chevron", "a ChevronDown built inline at XSmall in muted_foreground, with no setter (gpui-component/accordion.rs, AccordionItem::render), so expander.arrow(NONE)'s size and colour have no receiver -- Tier U")
        .not_themeable("animation", "reads the theme's spring_control (gpui-component/accordion.rs, AccordionItem). Theme::motion is a writable field (theme/mod.rs, MotionTokens) the connector leaves at its default, because native-theme models no motion -- our model's gap, not upstream's")
        .instance("animation", if reduce_motion {
            "none: reduced motion is on, so an item's content opens and closes at once (gpui-base/motion.rs, spring)"
        } else {
            "an item's content springs open and shut on spring_control; the chevron turns at once (gpui-component/accordion.rs, AccordionItem::render)"
        })
}

/// A card: a Fill `GroupBox` with no title, on the 0.8.0 model. `native` is
/// the installed 0.8.0 theme `next::geometry::group_box_content` refined its
/// content from, where it did (`demo::card`): the content then takes
/// `card.container(NONE)`'s fill and edge. `container_margin` is that
/// theme's `layout.container_margin`, which pads the sides the theme leaves
/// unstated. Its geometry line is recorded where the helper applies the
/// builder.
pub fn card(t: &Theme, native: Option<&NextTheme>, container_margin: Option<Pixels>) -> WidgetInfo {
    // No content text: the content is a Label, which paints foreground on
    // its own element over the group box's colour (label.rs:213) and
    // reports itself.
    let info = WidgetInfo::new("GroupBox").variant("Fill, a card");
    let info = match native {
        Some(r) => next_card_edge(
            info.colors(stated_claim(
                "fill",
                "card.container(NONE).fill",
                fill_of(&r.card.container(WidgetState::NONE)),
                "native-theme-gpui/next/builders.rs:750",
            )),
            t,
            r,
            GroupBoxKind::Fill,
        ),
        None => info.color(claim(
            "fill",
            "group_box",
            t.group_box,
            "gpui-component/group_box.rs:145",
        )),
    };
    let own = "p_4, the Fill variant's own";
    let info = match native {
        Some(_) => info.config(
            "padding",
            next_content_padding(native, container_margin, own),
        ),
        None => info.not_themeable("padding", next_content_padding(None, None, own)),
    };
    info.instance("content", "a body-text Label, which reports itself")
}

/// The ring gpui-component's own tab draws while it has the focus from the
/// keyboard.
const UPSTREAM_TAB_RING: &str = "on keyboard focus only: ring, the theme's focus ring colour (without its alpha where the theme says keyboard only or its ring has no width), as an inset shadow 2px wide inside the tab, a literal the theme's defaults.focus_ring_width and focus_ring_offset do not reach (gpui-component/tab/tab.rs:933, ring; :962-973, focus_visible)";

/// A `TabBar` of the Tab variant over `labels`, the one at `selected`
/// shown, its per-call seams taken from `native` where a 0.8.0 theme is
/// installed (`demo::tab_row`, `demo::next_tab`).
pub fn tab_row(
    t: &Theme,
    labels: &[&'static str],
    selected: usize,
    native: Option<&NextTheme>,
) -> WidgetInfo {
    let info = match native {
        Some(r) => upstream_tab_row(t, WidgetInfo::new("TabBar").variant("Tab"))
            .config(
                "font",
                folder_tab_font(&r.typography, &r.tab.label(WidgetState::NONE)),
            )
            .config(
                "size",
                folder_tab_size(r.tab.tab(WidgetState::NONE).min_size),
            ),
        None => upstream_tab_row(t, WidgetInfo::new("TabBar").variant("Tab")).not_themeable(
            "font",
            "text_sm, the Tab's own: no native theme is installed (tab/tab.rs, RenderOnce for Tab)",
        ),
    };
    let shown = labels.get(selected).copied().unwrap_or("none");
    info.instance("tabs", labels.join(", "))
        .instance("selected", shown)
        .instance("click", "selects the tab; the showcase keeps the state. A tab the bar clips is scrolled into view, the next one peeking past it (tab/tab_bar.rs, TabReveal)")
}

/// `info` with what gpui-component's `TabBar` of the Tab variant paints on
/// `t` and what of it no theme reaches.
fn upstream_tab_row(t: &Theme, info: WidgetInfo) -> WidgetInfo {
    info
        .color(claim(
                "bar",
                "tab_bar",
                t.tab_bar,
                "gpui-component/tab/tab_bar.rs:388",
            ))
            .color(claim(
                "label",
                "tab_foreground",
                t.tab_foreground,
                "gpui-component/tab/tab.rs:173",
            ))
            .color(claim(
                "selected label",
                "tab_active_foreground",
                t.tab_active_foreground,
                "gpui-component/tab/tab.rs:277",
            ))
            .color(claim(
                "selected tab",
                "tab_active",
                t.tab_active,
                "gpui-component/tab/tab.rs:278",
            ))
            .color(claim(
                "selected tab's sides",
                "border",
                t.border,
                "gpui-component/tab/tab.rs:284",
            ))
            .color(claim(
                "rule under the bar",
                "border",
                t.border,
                "gpui-component/tab/tab_bar.rs:676",
            ))
            .not_themeable("hover", "transparent: a Tab-variant tab paints no hover fill, and the Tab sets its hover style itself, which replaces one a caller gives (tab/tab.rs, TabVariant::hovered and RenderOnce for Tab), so tab.hover_background has no receiver")
            .not_themeable("edges", "the selected tab's 1px sides and the bar's 1px bottom rule in border, px literals set inside render (tab/tab.rs, TabVariant::selected; tab/tab_bar.rs, RenderOnce for TabBar), while the model's tab.border states the tab's own edge")
            .not_themeable("padding", "the tab's inner padding is a px literal per variant and Size on a child the Tab builds (tab/tab.rs, TabVariant::inner_paddings), so a caller's padding adds to it rather than replacing it, and the model's tab.border.padding is not applied")
            .not_themeable("focus ring", UPSTREAM_TAB_RING)
}

/// A `TabBar` of the Folder variant over `labels`, the one at `selected`
/// shown and the one at `disabled` taking no click, above the showcase's
/// panel, its per-call seams taken from `native` where a 0.8.0 theme is
/// installed (`demo::folder_tab_row`).
pub fn next_folder_tab_row(
    t: &Theme,
    labels: &[&'static str],
    selected: usize,
    disabled: usize,
    native: Option<&NextTheme>,
) -> WidgetInfo {
    let from = |field: &str| match native {
        Some(_) => format!(", from layout.{field}"),
        None => String::new(),
    };
    let info = WidgetInfo::new("TabBar")
        .variant("Folder")
        .color(claim(
            "bar",
            "tab_bar",
            t.tab_bar,
            "gpui-component/tab/tab_bar.rs:424",
        ))
        .color(claim(
            "label",
            "tab_foreground",
            t.tab_foreground,
            "gpui-component/tab/tab.rs:212",
        ))
        .color(claim(
            "hovered label",
            "tab_active_foreground",
            t.tab_active_foreground,
            "gpui-component/tab/tab.rs:267",
        ))
        .color(claim(
            "selected label",
            "tab_active_foreground",
            t.tab_active_foreground,
            "gpui-component/tab/tab.rs:318",
        ))
        .color(claim(
            "disabled label",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/tab/tab.rs:391",
        ))
        .color(claim(
            "selected tab",
            "tab_active",
            t.tab_active,
            "gpui-component/tab/tab.rs:1026",
        ))
        .color(claim(
            "hovered tab",
            "secondary",
            t.secondary,
            "gpui-component/tab/tab.rs:1040",
        ))
        .color(claim(
            "separator",
            "border",
            t.border,
            "gpui-component/tab/tab.rs:1054",
        ))
        .color(claim("panel", "tab_active", t.tab_active, "showcase"))
        .color(claim(
            "panel text",
            "tab_active_foreground",
            t.tab_active_foreground,
            "showcase",
        ))
        .config("corner radius", format!("{}px drawn: radius_lg, {}px{}, at the default size this bar takes, capped at a third of its {}px tab height; Theme::radius, {}px{}, at XSmall and Small, capped the same way (tab/tab.rs, FolderTabMetrics::new)", px_text(t.radius_lg.as_f32().min(FOLDER_TAB_HEIGHT.as_f32() / 3.0)), px_text(t.radius_lg.as_f32()), from("corner_radius_lg"), px_text(FOLDER_TAB_HEIGHT.as_f32()), px_text(t.radius.as_f32()), from("corner_radius")))
        .not_themeable("outward corners", "the theme's tab.active_outward_corner_radius: upstream's Folder curves at Theme::radius / radius_lg, capped at a third of the height (tab/tab.rs, FolderTabMetrics::new); widgets::TabBar draws the theme's")
        .not_themeable("one radius", "the selected tab's top corners and its curves out into the panel take the same radius, so a theme cannot give them two (tab/tab.rs, FolderTabMetrics); nor can it reach the curves alone: upstream's Folder has no seam for a theme's tab shape. Only on a tab narrower than three radii do the two differ: its top corners shrink to a third of its width, keeping a third of the top flat (tab/tab.rs, folder_tab_shape)")
        .not_themeable("hover", "an inset fill in secondary, a button token, which the connector maps from button.background_color, so tab.hover_background has no receiver (tab/tab.rs, RenderOnce for Tab)")
        .not_themeable("outline", "none: the selected tab's shape is a filled path, stroked by nothing (tab/tab.rs, folder_tab_shape), so the model's tab.border is not drawn")
        .not_themeable("separator", "1px wide in border, a px literal set inside render (tab/tab.rs, RenderOnce for Tab), drawn between two tabs neither of which is selected or hovered (tab/tab_bar.rs, RenderOnce for TabBar)")
        .not_themeable("padding", "px literals per Size (tab/tab.rs, FolderTabMetrics::new): the bar's space above the tabs is 2, 4, 4 and 8px at XSmall, Small, the default size and Large, the tab's side padding 8, 8, 12 and 16px and the separator 12, 12, 16 and 16px tall, so the model's tab.border.padding is not applied")
        .not_themeable("gap", "4px between two tabs, holding their separator: FOLDER_TAB_GAP, a px literal (tab/tab.rs)")
        .not_themeable("height", "32px at the default size (20, 24 and 36px at XSmall, Small and Large), its content 24px tall (16, 18 and 28px), as Segmented's: px literals per Size (tab/tab.rs, TabVariant::height and TabVariant::inner_height)")
        .not_themeable("focus ring", UPSTREAM_TAB_RING);
    let info = match native {
        Some(r) => info
            .config(
                "font",
                folder_tab_font(&r.typography, &r.tab.label(WidgetState::NONE)),
            )
            .config(
                "size",
                folder_tab_size(r.tab.tab(WidgetState::NONE).min_size),
            ),
        None => info.not_themeable(
            "font",
            "text_sm, the Tab's own: no native theme is installed (tab/tab.rs, RenderOnce for Tab)",
        ),
    };
    let shown = labels.get(selected).copied().unwrap_or("none");
    let inert = labels.get(disabled).copied().unwrap_or("none");
    info.instance("tabs", labels.join(", "))
        .instance("selected", shown)
        .instance("disabled", inert)
        .instance("click", "selects the tab; the showcase keeps the state. The disabled tab takes none (gpui-base tabs.rs, RenderOnce for Tab)")
        .instance("panel", "the showcase's own, under the bar, filled with tab_active, the colour of the selected tab's shape, so the tab joins it, and labelled in tab_active_foreground, the selected label's colour")
}

/// The font line of a Folder tab's label of the installed 0.8.0 theme,
/// whose `typography` and `tab.label(NONE)`, `label`, it is lettered in
/// (`demo::next_text_in`) where `label` gives a font gpui can letter it in;
/// else upstream's own label.
pub fn folder_tab_font(
    typography: &native_theme::next::Typography,
    label: &native_theme::next::TextStyle,
) -> String {
    match native_theme_gpui::next::label_font_unmapped(label, "tab.label(NONE)") {
        None => format!(
            "tab.label(NONE): {}, on each label, a child over the text_sm the Tab sets on itself (tab/tab.rs, RenderOnce for Tab)",
            super::typography::text_style_font(typography, label)
        ),
        Some(unstated) => format!(
            "text_sm, the Tab's own: the theme states no value gpui can take at {}, so each label is upstream's (tab/tab.rs, RenderOnce for Tab; spec §10, rule 0)",
            unstated.home()
        ),
    }
}

/// The size line of a Folder tab of the installed 0.8.0 theme, whose
/// `tab.tab(NONE)` states `min_size` (`[width, height]`, each `None` where
/// unstated): each stated least the Tab's style takes, and upstream's own
/// where none is stated.
pub fn folder_tab_size(min_size: [Option<f32>; 2]) -> String {
    let least = |name: &str, home: &str, v: Option<f32>| match v {
        Some(v) => format!("{name} at least {home}, {}px", px_text(v)),
        None => format!("{name} upstream's: the theme states no {home}"),
    };
    let [w, h] = min_size;
    format!(
        "{}; {}; a stated least through the Tab's style, which its own h does not clear (tab/tab.rs, RenderOnce for Tab)",
        least("width", "tab.tab(NONE).min_size[0]", w),
        least("height", "tab.tab(NONE).min_size[1]", h)
    )
}

/// The expanders the showcase draws as `r`'s `expander.*` states them
/// (`demo::native_expander`, `demo::ExpanderLook`), on `t`, the
/// gpui-component theme whose own values stand where `r` states none,
/// reading `titles`, each `open` or not, their arrow `arrow`, of the chosen
/// icon theme -- `turned`, the chevron an open item shows where the set
/// gives `arrow` as pixels, or as an SVG in several colours, drawn as an
/// image, which does not turn -- and `gap`, the `layout.widget_gap` that
/// parts the items where the theme states the frame.
pub fn native_expander(
    r: &NextTheme,
    t: &Theme,
    titles: [&'static str; 2],
    open: [bool; 2],
    [arrow, turned]: [&SampleIcon; 2],
    gap: Option<Pixels>,
) -> WidgetInfo {
    let look = ExpanderLook::next_of(r);
    let e = &look;
    // The frame is drawn at the stated stroke's width, upstream's border_1
    // where it states none, unless frame_enabled is false: KDE's expander
    // draws none (§2.27).
    let frame_width = Some(match e.frame_enabled {
        Some(false) => 0.0,
        _ => e
            .line_width
            .unwrap_or(f32::from(crate::demo::EXPANDER_LINE_WIDTH)),
    });
    let edge = match (e.frame_enabled == Some(true), e.line.is_some()) {
        (true, true) => edge_claim(
            "each item's frame",
            "expander.container(NONE).stroke.color",
            stroke_color_of(&r.expander.container(WidgetState::NONE)),
            frame_width,
            "showcase",
        ),
        (true, false) => edge_claim(
            "each item's frame",
            "border",
            Some(t.border),
            frame_width,
            "showcase",
        ),
        (false, true) => edge_claim(
            "frame and the line between items",
            "expander.container(NONE).stroke.color",
            stroke_color_of(&r.expander.container(WidgetState::NONE)),
            frame_width,
            "showcase",
        ),
        (false, false) => edge_claim(
            "frame and the line between items",
            "border",
            Some(t.border),
            frame_width,
            "showcase",
        ),
    };
    let title = stated_claim(
        "title",
        "expander.header_label(NONE).color",
        text_color_of(&r.expander.header_label(WidgetState::NONE)),
        "showcase",
    )
    .unwrap_or_else(|| claim("open title", "foreground", t.foreground, "showcase"));
    let info = WidgetInfo::new("Expander")
        .variant("drawn by the showcase")
        .colors(edge)
        .color(title);
    let leading = e.arrow_side == Some(native_theme::next::ArrowSide::Leading);
    let glyph = match (leading, arrow.is_raster()) {
        (true, false) => {
            "ChevronRight, before the title and turned down while open, as Breeze and the HIG draw a leading arrow"
        }
        (false, false) => {
            "ChevronDown, after the title and turned while open, as upstream's Accordion draws it (gpui-component/accordion.rs, AccordionItem::render)"
        }
        (true, true) => {
            "ChevronRight, before the title, as Breeze and the HIG draw a leading arrow; pixels, or an SVG in several colours, are drawn as an image, which does not turn (gpui::img takes no transformation), so an open item shows the set's ChevronDown instead"
        }
        (false, true) => {
            "ChevronDown, after the title, as upstream's Accordion draws it (gpui-component/accordion.rs, AccordionItem::render); pixels, or an SVG in several colours, are drawn as an image, which does not turn (gpui::img takes no transformation), so an open item shows the set's ChevronUp instead"
        }
    };
    let info = info.instance(
        "arrow glyph",
        format!(
            "{glyph}: {}",
            arrow.note("the title row shows its title alone")
        ),
    );
    let info = if arrow.is_raster() {
        info.instance(
            "open arrow glyph",
            turned.note("an open item's title row shows its title alone"),
        )
    } else {
        info
    };
    let info = if arrow.shown() {
        // A raster arrow keeps its own colours (`arrow.note`).
        let info = if arrow.is_raster() {
            info
        } else {
            info.color(
                stated_claim(
                    "arrow",
                    "expander.arrow(NONE).color",
                    icon_color_of(&r.expander.arrow(WidgetState::NONE)),
                    "showcase",
                )
                .unwrap_or_else(|| {
                    claim("arrow", "muted_foreground", t.muted_foreground, "showcase")
                }),
            )
        };
        info.config("arrow", match e.arrow_size {
            Some(size) => format!("expander.arrow(NONE).size, {}px", px_text(size)),
            None => "not stated: upstream's xsmall arrow (gpui-component/accordion.rs, AccordionItem::render)".to_string(),
        })
        .config("arrow side", match e.arrow_side {
            Some(side) => format!("expander.arrow_side, {side:?}"),
            None => "not stated: upstream's Accordion's, after the title (gpui-component/accordion.rs, AccordionItem::render)".to_string(),
        })
        .config("arrow gap", match e.arrow_gap {
            Some(gap) => format!("expander.arrow_gap, {}px", px_text(gap)),
            None => "not stated: upstream's AccordionItem's gap_3 (gpui-component/accordion.rs, AccordionItem::render)".to_string(),
        })
    } else {
        info
    };
    let info = match stated_claim(
        "hovered title",
        "expander.header(HOVER).fill",
        state_fill_of(&r.expander.header(WidgetState::HOVER)),
        "showcase",
    ) {
        Some(hover) => info.color(hover),
        None => info.not_themeable(
            "hover",
            "none: no rule states expander.header(HOVER).fill, and upstream's title row has no hover style of its own (gpui-component/accordion.rs, AccordionItem::hover)",
        ),
    };
    let shown: Vec<&str> = titles
        .iter()
        .zip(open)
        .filter(|(_, open)| *open)
        .map(|(title, _)| *title)
        .collect();
    info.config("title row", expander_title_row(e, &r.typography, t))
    .config("edge", expander_edge(e, t, gap))
    .config(
        "body indent",
        match e.content_indent {
            Some(indent) => format!("expander.content_indent, {}px", px_text(indent)),
            None => {
                "not stated: upstream's AccordionItem's px_3 (gpui-component/accordion.rs, AccordionItem::render)"
                    .to_string()
            }
        },
    )
    .config("padding", expander_padding(e, gap))
    .not_themeable(
        "fill",
        "none: the model states no expander fill, so the window's background shows",
    )
    .instance("items", titles.join(", "))
    .instance(
        "open",
        if shown.is_empty() {
            "none".to_string()
        } else {
            shown.join(", ")
        },
    )
    .instance(
        "click",
        "a title opens or closes its item; the showcase keeps the state",
    )
}

/// The title row line of the theme-drawn expander `e`, lettered over
/// `typography`, on `t`: its height, `expander.header(NONE).min_size[1]`
/// where stated, else upstream's, which sets none; and its title's font,
/// `expander.header_label(NONE)` where it gives one gpui can letter it in,
/// else upstream's.
pub(crate) fn expander_title_row(
    e: &ExpanderLook,
    typography: &native_theme::next::Typography,
    t: &Theme,
) -> String {
    let height = match e.header_height {
        Some(height) => format!("expander.header(NONE).min_size[1], {}px", px_text(height)),
        None => "as tall as its content and padding: the theme states no expander.header(NONE).min_size[1], and upstream's sets no height (gpui-component/accordion.rs, AccordionItem::render)".to_string(),
    };
    let font = match native_theme_gpui::next::label_font_unmapped(
        &e.font,
        "expander.header_label(NONE)",
    ) {
        None => format!(
            "the title in expander.header_label(NONE): {}",
            super::typography::text_style_font(typography, &e.font)
        ),
        Some(unstated) => format!(
            "the title upstream's, font_medium at {}: the theme states no value gpui can take at {} (gpui-component/accordion.rs, AccordionItem::render; spec §10, rule 0)",
            super::rems_text(crate::demo::EXPANDER_TITLE_SIZE, t),
            unstated.home()
        ),
    };
    format!("{height}, {font}")
}

/// What frames the theme-drawn expander `e` and parts its items, the
/// items `gap` (`layout.widget_gap`) apart where each is framed on its own.
pub(crate) fn expander_edge(e: &ExpanderLook, t: &Theme, gap: Option<Pixels>) -> String {
    let colour = match e.line {
        Some(_) => "expander.container(NONE).stroke.color",
        None => "upstream's border, as the theme states no expander.container(NONE).stroke.color",
    };
    let width = match e.line_width {
        Some(width) => format!(
            "expander.container(NONE).stroke.width, {}px",
            px_text(width)
        ),
        None => format!(
            "upstream's border_1, {}px, as the theme states no expander.container(NONE).stroke.width",
            px_text(f32::from(crate::demo::EXPANDER_LINE_WIDTH))
        ),
    };
    let radius = match e.radius {
        Some(radius) => format!(
            "expander.container(NONE).corners, {}px",
            px_text(radius.max(0.0))
        ),
        None => format!(
            "upstream's radius_lg, {}px, as the theme states no expander.container(NONE).corners",
            px_text(t.radius_lg.as_f32())
        ),
    };
    match e.frame_enabled {
        Some(false) => {
            "none: expander.frame_enabled is false, no frame and no line between the items"
                .to_string()
        }
        _ if e.line_width.is_some_and(|width| width <= 0.0) => format!(
            "none: expander.container(NONE).stroke.width is 0; the items are still rounded by {radius}"
        ),
        Some(true) => {
            let apart = match gap {
                Some(gap) => format!("layout.widget_gap, {}px, apart", px_text(gap.as_f32())),
                None => "with no gap between them: layout.widget_gap is unstated".to_string(),
            };
            format!("each item framed on its own by {colour} at {width}, radius {radius}, {apart}")
        }
        None => format!(
            "{colour} at {width}, radius {radius}, round the whole and between the items, as upstream's bordered Accordion frames them: expander.frame_enabled is unstated"
        ),
    }
}

/// How the theme-drawn expander `e` pads its title rows, side by side, and
/// its body: a stated `expander.header(NONE)` padding side as it states it,
/// an unstated left or right by upstream's px_3, an unstated top or bottom
/// by upstream's py_2 (`demo::native_expander`, through
/// `demo::next_padded`); the body right
/// and below as upstream's AccordionItem pads it wherever no `gap`
/// (`layout.widget_gap`) parts it from its title: inside the frame
/// upstream's draws, where `frame_enabled` is unstated, and where the
/// layout states no gap.
pub(crate) fn expander_padding(e: &ExpanderLook, gap: Option<Pixels>) -> String {
    let [top, right, bottom, left] = e.padding;
    let side = |name: &str, value: Option<f32>, across: bool| match (value, across) {
        (Some(v), _) => format!("{name} {}px", px_text(v)),
        (None, true) => format!("{name} upstream's px_3 (EXPANDER_PADDING_X)"),
        (None, false) => format!("{name} upstream's py_2 (EXPANDER_PADDING_Y)"),
    };
    let body = match (e.frame_enabled, gap) {
        (Some(_), Some(_)) => {
            "the body has no padding beyond its indent, a title being layout.widget_gap above its body"
        }
        (Some(_), None) => {
            "the body padded right by px_3 and below by pb_2, upstream's AccordionItem's (gpui-component/accordion.rs, AccordionItem::render), as no layout.widget_gap is stated to part it from its title"
        }
        (None, _) => {
            "the body padded right by px_3 and below by pb_2, upstream's AccordionItem's (gpui-component/accordion.rs, AccordionItem::render), inside the frame upstream's Accordion draws"
        }
    };
    format!(
        "expander.header(NONE).padding on each title row: {}, {}, {}, {}; {body}",
        side("top", top, false),
        side("right", right, true),
        side("bottom", bottom, false),
        side("left", left, true),
    )
}

/// A row of tabs `native_theme_gpui::widgets::TabBar` draws as the
/// installed 0.8.0 theme `r`'s `tab.*` states a tab, as `TabLook::next_of`
/// reads it (`demo::native_tab_strip`), under `variant`: the page tabs, the
/// inspector's and the Basic page's. Each claim names the home the look
/// reads and cites the line of `widgets/tab_bar.rs` that spells it. Where
/// the look or the label's font cannot be built, `widgets::TabBar` draws
/// gpui-component's Tab-variant `TabBar` on `t` in its place
/// (`TabBar::fallback`), and the info is upstream's, with the reason.
#[cfg(feature = "widgets")]
pub fn native_tab_row(t: &Theme, r: &NextTheme, variant: &'static str) -> WidgetInfo {
    let label = r.tab.label(WidgetState::NONE);
    let drawn = TabLook::next_of(r).and_then(|look| {
        match native_theme_gpui::next::label_font_unmapped(&label, "tab.label(NONE)") {
            None => Ok(look),
            Some(unstated) => Err(unstated),
        }
    });
    let look = match drawn {
        Ok(look) => look,
        Err(unstated) => {
            let reason = format!(
                "{unstated}, so widgets::TabBar draws gpui-component's TabBar of the Tab variant in its place (native-theme-gpui/widgets/tab_bar.rs, TabLook::next_of, TabBar::fallback)"
            );
            return upstream_tab_row(t, WidgetInfo::new("TabBar").variant(variant))
                .config("drawn by", reason.clone())
                .config("selection mark", format!("upstream's: {reason}"))
                .config("gap", format!("upstream's: {reason}"))
                .not_themeable("font", "text_sm, the Tab's own: widgets::TabBar draws upstream's TabBar (tab/tab.rs, RenderOnce for Tab)");
        }
    };
    let selected = r.tab.tab(WidgetState::SELECTED);
    // Each value the helper reads at its home, which TabLook::next_of read
    // there too: the look built, every one of them is stated.
    let info = WidgetInfo::new("TabBar")
        .variant(variant)
        .colors(stated_claim(
            "bar",
            "tab.bar(NONE).fill",
            fill_of(&r.tab.bar(WidgetState::NONE)),
            "native-theme-gpui/widgets/tab_bar.rs:282",
        ))
        .colors(stated_claim(
            "unselected tab",
            "tab.tab(NONE).fill",
            fill_of(&r.tab.tab(WidgetState::NONE)),
            "native-theme-gpui/widgets/tab_bar.rs:283",
        ))
        .colors(stated_claim(
            "label",
            "tab.label(NONE).color",
            text_color_of(&r.tab.label(WidgetState::NONE)),
            "native-theme-gpui/widgets/tab_bar.rs:284",
        ))
        .colors(stated_claim(
            "selected tab",
            "tab.tab(SELECTED).fill",
            fill_of(&r.tab.tab(WidgetState::SELECTED)),
            "native-theme-gpui/widgets/tab_bar.rs:287",
        ))
        .colors(stated_claim(
            "selected label",
            "tab.label(SELECTED).color",
            text_color_of(&r.tab.label(WidgetState::SELECTED)),
            "native-theme-gpui/widgets/tab_bar.rs:290",
        ))
        // No line of widgets/tab_bar.rs spells the stroke's colour whole
        // (`selected_outline` reads `tab.tab(SELECTED).stroke`), so the value
        // check alone holds it.
        .colors(edge_claim(
            "selected tab's outline",
            "tab.tab(SELECTED).stroke.color",
            stroke_color_of(&r.tab.tab(WidgetState::SELECTED)),
            Some(f32::from(look.border_width)),
            "showcase",
        ))
        .color(claim(
            "rule under the bar",
            "separator.color",
            super::next_stated(r.separator.color),
            "showcase",
        ))
        .colors(stated_claim(
            "hovered unselected tab, in place of its fill, over the bar",
            "tab.tab(HOVER).fill",
            state_fill_of(&r.tab.tab(WidgetState::HOVER)),
            "native-theme-gpui/widgets/tab_bar.rs:285",
        ))
        .colors(stated_claim(
            "hovered label",
            "tab.label(HOVER).color",
            text_color_of(&r.tab.label(WidgetState::HOVER)),
            "native-theme-gpui/widgets/tab_bar.rs:286",
        ))
        // Only where the look has the line: TabBar draws none otherwise.
        .colors(stated_claim(
            "selected tab's line",
            "tab.indicator(SELECTED).fill",
            look.indicator
                .and(fill_of(&r.tab.indicator(WidgetState::SELECTED))),
            "native-theme-gpui/widgets/tab_bar.rs:306",
        ));
    let [top, right, bottom, left] = padding_of(&r.tab.tab(WidgetState::NONE));
    let across = |stated: Option<f32>, drawn: Pixels| match stated {
        Some(v) => format!("{}px", px_text(v)),
        None => format!(
            "upstream's {}px (tab/tab.rs, TabVariant::inner_paddings)",
            px_text(drawn.as_f32())
        ),
    };
    let up_down = |stated: Option<f32>| match stated {
        Some(v) => format!("{}px", px_text(v)),
        None => "unstated".to_string(),
    };
    let radius = if selected
        .stated
        .contains(native_theme::next::PartStated::CORNERS)
    {
        format!(
            "tab.tab(SELECTED).corners, {}px",
            px_text(look.radius.as_f32())
        )
    } else {
        "square, gpui-component's own for its Tab variant, as the theme states no tab.tab(SELECTED).corners (tab/tab.rs, TabVariant::radius)".to_string()
    };
    let outline = if look.border_width.as_f32() > 0.0 {
        format!(
            "tab.tab(SELECTED).stroke on the selected tab only: {}px, on its top corners {radius}, as Breeze and Windows draw it (docs/platform-facts.md §2.11){}",
            px_text(look.border_width.as_f32()),
            if look.outward_radius.is_some() {
                "; no bottom edge: it runs on down the outward corners instead"
            } else {
                ""
            }
        )
    } else {
        format!(
            "none: the theme states no tab.tab(SELECTED).stroke, or states it has none; the selected tab's top corners {radius}"
        )
    };
    let info = info
        .config(
            "size",
            format!(
                "each tab at least tab.tab(NONE).min_size[0], {}px, by tab.tab(NONE).min_size[1], {}px",
                px_text(look.min_width.as_f32()),
                px_text(look.min_height.as_f32())
            ),
        )
        .config(
            "font",
            format!(
                "tab.label(NONE): {}",
                super::typography::text_style_font(&r.typography, &label)
            ),
        )
        .config(
            "padding",
            format!(
                "tab.tab(NONE).padding: left {}, right {}; top and bottom {} and {}",
                across(left, look.padding_left),
                across(right, look.padding_right),
                up_down(top),
                up_down(bottom)
            ),
        )
        .config("outline", outline)
        .config("outward corners", match look.outward_radius {
            Some(radius) => format!(
                "tab.active_outward_corner_radius, {}px: the selected tab curves out into the bar's bottom edge at both feet, its outline following, with no bottom edge (native-theme-gpui/widgets/tab_bar.rs, TabLook::outward_radius)",
                px_text(radius.as_f32())
            ),
            None => "none: the theme states no outward corners, so the selected tab's feet are square".to_string(),
        })
        .config("selection mark", match look.indicator {
            Some(line) => format!(
                "a line tab.indicator(SELECTED).min_size[1], {}px, thick along the selected tab's {} edge, tab.indicator_side, in tab.indicator(SELECTED).fill, over its outline (native-theme-gpui/widgets/tab_bar.rs, TabIndicator)",
                px_text(line.width.as_f32()),
                match line.side {
                    TabIndicatorSide::Top => "top",
                    TabIndicatorSide::Bottom => "bottom",
                }
            ),
            None => "none: the theme states no indicator -- widgets::TabBar draws one only where tab.indicator(SELECTED).fill, tab.indicator(SELECTED).min_size[1] and tab.indicator_side are all stated -- so the selected tab is marked by its fill, label and outline alone".to_string(),
        })
        .config("gap", match r.tab.item_gap {
            Some(_) => format!("{}px between neighbouring tabs, tab.item_gap", px_text(look.gap.as_f32())),
            None => "none: tab.item_gap is unstated, and widgets::TabBar lays its tabs with none, as gpui-component lays its default variant's (tab/tab_bar.rs, TabBar::render)".to_string(),
        })
        .not_themeable("rule under the bar", "separator.*, the line the theme states for parting content: the model states no tab-bar rule of its own");
    tab_bar_ring(info, r)
}

/// `info` with the focus ring `widgets::TabBar` draws round a tab under the
/// 0.8.0 theme `r`, as `FocusRing::next_of` reads it
/// (native-theme-gpui/widgets/mod.rs): `colors.focus_ring`,
/// `focus_ring.stroke.width` wide, inside the tab.
#[cfg(feature = "widgets")]
fn tab_bar_ring(info: WidgetInfo, r: &NextTheme) -> WidgetInfo {
    let offset = r.focus_ring.offset;
    let width = native_theme_gpui::next::focus_ring_width(r)
        .ok()
        .filter(|width| width.is_finite() && offset.is_finite() && *width > 0.0);
    let Some(width) = width else {
        return info.config(
            "focus ring",
            "none: the theme states no ring, its focus_ring.stroke.width not positive or stated per side, or its focus_ring.offset not finite (native-theme-gpui/widgets/mod.rs, FocusRing::next_of)",
        );
    };
    info.color(claim(
        "focus ring",
        "colors.focus_ring",
        super::next_stated(r.colors.focus_ring),
        "native-theme-gpui/widgets/mod.rs:363",
    ))
    .config(
        "focus ring",
        format!(
            "{} an inset shadow focus_ring.stroke.width, {}px, wide inside the tab (native-theme-gpui/widgets/tab_bar.rs, TabBar). focus_ring.offset, {}px, is not reached: the tabs' row scrolls and clips what lies outside it, so a ring outside the tab would lose a band",
            if r.focus_ring.trigger == Some(native_theme::next::FocusTrigger::KeyboardOnly) {
                "on the focused tab, for a focus that followed a key press only (native-theme-gpui/widgets/mod.rs, focus_followed_key):"
            } else {
                "on the focused tab:"
            },
            px_text(width),
            px_text(offset)
        ),
    )
}

/// Each side of a segment's padding as `demo::segmented` draws it: a side
/// `segmented_control.segment(NONE)` states by its value, an unstated left
/// or right by `SEGMENT_PADDING`, upstream's own, and an unstated top or
/// bottom by none, upstream's too.
pub(crate) fn segment_padding(padding: [Option<f32>; 4]) -> String {
    let [top, right, bottom, left] = padding;
    let side = |name: &str, stated: Option<f32>, across: bool| match (stated, across) {
        (Some(v), _) => format!("{name} {}px", px_text(v)),
        (None, true) => format!("{name} {}px, unstated", px_text(SEGMENT_PADDING.as_f32())),
        (None, false) => format!("{name} none, unstated"),
    };
    format!(
        "segmented_control.segment(NONE).padding per side: {}; {}; {}; {}. An unstated left or right side takes {}px and an unstated top or bottom none, upstream's own for a segment at the default Size (tab/tab.rs, TabVariant::inner_paddings)",
        side("top", top, false),
        side("right", right, true),
        side("bottom", bottom, false),
        side("left", left, true),
        px_text(SEGMENT_PADDING.as_f32()),
    )
}

/// The size, font, border and separator lines of the theme-drawn segmented
/// control `s`, in that order, its labels lettered over `typography`, on
/// `t`: each the stated value where `s` gives one, else upstream's own,
/// named.
pub(crate) fn segmented_lines(
    s: &SegmentedLook,
    typography: &native_theme::next::Typography,
    t: &Theme,
) -> [String; 4] {
    let size = match s.segment_height {
        Some(height) => format!(
            "each segment at least segmented_control.segment(NONE).min_size[1], {}px, tall",
            px_text(height)
        ),
        None => format!(
            "each segment at least {}px tall, upstream's own for a segmented tab at the default Size: the theme states no segmented_control.segment(NONE).min_size[1] (tab/tab.rs, TabVariant::height)",
            px_text(crate::demo::SEGMENT_HEIGHT.as_f32())
        ),
    };
    let font = match native_theme_gpui::next::label_font_unmapped(
        &s.font,
        "segmented_control.label(NONE)",
    ) {
        None => format!(
            "segmented_control.label(NONE): {}",
            super::typography::text_style_font(typography, &s.font)
        ),
        Some(unstated) => format!(
            "upstream's text_sm, {}: the theme states no value gpui can take at {} (tab/tab.rs, RenderOnce for Tab; spec §10, rule 0)",
            super::rems_text(crate::demo::SEGMENT_LABEL_SIZE, t),
            unstated.home()
        ),
    };
    let radius = match s.radius {
        Some(radius) => format!(
            "segmented_control.track(NONE).corners, {}px",
            px_text(radius.max(0.0))
        ),
        None => format!(
            "upstream's radius_lg, {}px, as the theme states no segmented_control.track(NONE).corners (tab/tab.rs, TabVariant::tab_bar_radius)",
            px_text(t.radius_lg.as_f32())
        ),
    };
    let border = match s.frame_width() {
        Some(width) if width > 0.0 => format!(
            "segmented_control.track(NONE).stroke: {}px, radius {radius}",
            px_text(width)
        ),
        Some(_) => format!(
            "none: segmented_control.track(NONE).stroke.width is 0; the track is still rounded by {radius}"
        ),
        None => format!(
            "none, as upstream's segmented TabBar draws none: the theme states no segmented_control.track(NONE).stroke gpui can draw (tab/tab_bar.rs, TabBar::render); the track is rounded by {radius}"
        ),
    };
    let separators = match s.separator_line() {
        None => "none, as upstream's segmented TabBar draws no line between its segments: the theme states no segmented_control.separator(NONE).fill (tab/tab_bar.rs, TabBar::render)".to_string(),
        Some((_, width)) if width > 0.0 => format!(
            "segmented_control.separator_width, {}px, in segmented_control.separator(NONE).fill between segments",
            px_text(width)
        ),
        Some(_) => "none: segmented_control.separator_width is 0".to_string(),
    };
    [size, font, border, separators]
}

/// The segmented control over `labels`, the one at `selected` shown: drawn
/// by the showcase from `native`'s `segmented_control` where a 0.8.0 theme
/// is installed (`demo::SegmentedLook`), each value it leaves unstated
/// upstream's own, read on `t`; upstream's segmented `TabBar` where none is
/// (`demo::segmented`).
pub fn segmented(
    t: &Theme,
    labels: &[&'static str],
    selected: usize,
    native: Option<&NextTheme>,
) -> WidgetInfo {
    let shown = labels.get(selected).copied().unwrap_or("none");
    let info = match native {
        Some(r) => {
            let s = SegmentedLook::next_of(r);
            let track = stated_claim("track", "segmented_control.track(NONE).fill", fill_of(&r.segmented_control.track(WidgetState::NONE)), "showcase")
                .unwrap_or_else(|| claim("track", "tab_bar_segmented", t.tab_bar_segmented, "showcase"));
            let label = stated_claim("label", "segmented_control.label(NONE).color", text_color_of(&r.segmented_control.label(WidgetState::NONE)), "showcase")
                .unwrap_or_else(|| claim("label", "tab_foreground", t.tab_foreground, "showcase"));
            let selected_fill = stated_claim("selected segment", "segmented_control.segment(SELECTED).fill", fill_of(&r.segmented_control.segment(WidgetState::SELECTED)), "showcase")
                .unwrap_or_else(|| claim("selected segment", "background", t.background, "showcase"));
            let selected_label = stated_claim("selected label", "segmented_control.label(SELECTED).color", text_color_of(&r.segmented_control.label(WidgetState::SELECTED)), "showcase")
                .unwrap_or_else(|| claim("selected label", "tab_active_foreground", t.tab_active_foreground, "showcase"));
            let hovered_label = stated_claim("hovered label", "segmented_control.label(HOVER).color", state_text_color_of(&r.segmented_control.label(WidgetState::HOVER)), "showcase")
                .unwrap_or_else(|| claim("hovered label", "tab_active_foreground", t.tab_active_foreground, "showcase"));
            // The line left of every segment but the first, where one is drawn.
            let separators = s.separator_line().map(|(_, width)| {
                let lines = (labels.len() > 1).then_some(width);
                edge_claim("separators", "segmented_control.separator(NONE).fill", fill_of(&r.segmented_control.separator(WidgetState::NONE)), lines, "showcase")
            });
            let info = WidgetInfo::new("Segmented control")
                .variant("drawn by the showcase")
                .color(track)
                .color(label)
                .color(selected_fill)
                .color(selected_label)
                .color(hovered_label)
                .colors(edge_claim("border", "segmented_control.track(NONE).stroke.color", stroke_color_of(&r.segmented_control.track(WidgetState::NONE)), s.frame_width(), "showcase"))
                .colors(separators.flatten());
            let info = match stated_claim("hovered segment", "segmented_control.segment(HOVER).fill", state_fill_of(&r.segmented_control.segment(WidgetState::HOVER)), "showcase") {
                Some(hover) => info.color(hover),
                None => info.not_themeable("hover", "none: no rule states segmented_control.segment(HOVER).fill, and upstream's unselected segment keeps a transparent fill under the pointer (tab/tab.rs, TabVariant::hovered)"),
            };
            let [size, font, border, separators] = segmented_lines(&s, &r.typography, t);
            info.config("why drawn", "upstream's segmented TabBar fills its selected segment with the window's background through a sliding indicator and its hovered one inside render, and labels its segments from the tab tokens (tab/tab_bar.rs, TabBar::render_indicator; tab/tab.rs, TabVariant::hovered), so segmented_control.segment(SELECTED).fill, segment(HOVER).fill and the labels' colours would have no receiver there")
                .config("size", size)
                .config("padding", segment_padding(s.padding))
                .config("font", font)
                .config("border", border)
                .config("separators", separators)
        }
        None => WidgetInfo::new("TabBar")
            .variant("Segmented")
            .color(claim(
                "track",
                "tab_bar_segmented",
                t.tab_bar_segmented,
                "gpui-component/tab/tab_bar.rs:410",
            ))
            .color(claim(
                "selected segment",
                "background",
                t.background,
                "gpui-component/tab/tab_bar.rs:277",
            ))
            .color(claim(
                "label",
                "tab_foreground",
                t.tab_foreground,
                "gpui-component/tab/tab.rs:201",
            ))
            .color(claim(
                "selected label",
                "tab_active_foreground",
                t.tab_active_foreground,
                "gpui-component/tab/tab.rs:300",
            ))
            .not_themeable("font", "text_sm, the Tab's own: no native theme is installed (tab/tab.rs, RenderOnce for Tab)"),
    };
    info.instance("segments", labels.join(", "))
        .instance("selected", shown)
        .instance("click", "selects the segment; the showcase keeps the state")
}

/// The `Collapsible`, `open` or not.
pub fn collapsible(t: &Theme, open: bool) -> WidgetInfo {
    WidgetInfo::new("Collapsible")
        .variant(if open { "open" } else { "closed" })
        .not_themeable("fill and edge", "none: a Collapsible only shows or hides its content and paints nothing of its own (gpui-component/collapsible.rs, Collapsible). The toggle is this demo's ghost Button and the content a Label, which report themselves")
        .not_themeable(
            "content padding",
            format!(
                "{} round the content, COLLAPSIBLE_CONTENT_PADDING, the showcase's own choice rather than a platform's, as the theme states none for it (spec §3.1)",
                super::rems_text(crate::demo::COLLAPSIBLE_CONTENT_PADDING, t)
            ),
        )
        .instance("animation", "none: a Collapsible springs its reveal on the theme's spring_control only when it is given a motion_id (gpui-component/collapsible.rs, Collapsible::render), and this one is not, so its content appears and disappears at once")
}

/// The Collapsible's toggle: an upstream Ghost `Button` reading `label`,
/// with `icon`, of the chosen icon theme: a chevron pointing down while the
/// Collapsible is `open`.
pub fn collapsible_toggle(t: &Theme, open: bool, label: &str, icon: &SampleIcon) -> WidgetInfo {
    let (variant, content) = if icon.shown() {
        ("Ghost, icon", GhostContent::TextAndIcon)
    } else {
        ("Ghost", GhostContent::Text)
    };
    WidgetInfo::new("Button")
        .variant(variant)
        .colors(ghost_colours(t, content))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable(
            "fill",
            "none: a Ghost Button is transparent until hovered (button/button.rs, ButtonVariant::hovered)",
        )
        .instance("variant", "gpui-component's own .ghost(), not native_theme_gpui::variants::ghost_button: it hovers with the item-highlight pair (button/button.rs, ButtonVariant::hovered Ghost arm)")
        .instance("label", label.to_string())
        .instance(
            "icon",
            icon.note(if open {
                "the Button shows its label alone while the content shows"
            } else {
                "the Button shows its label alone while the content is hidden"
            }),
        )
}

/// The `Carousel`, drawn while gpui's `reduce_motion` is as given.
pub fn carousel(t: &Theme, reduce_motion: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Carousel");
    // Carousel::render hands its focus overlay, which has a 1px border
    // (carousel/carousel.rs:198), to focus_ring_style (:201), whose
    // focus_style (styled.rs:183, :250-298) tints the border and draws the
    // ring outside it at FOCUS_RING_OPACITY, 0.5, where the theme turns the
    // ring on (styled.rs:12, :258-262). Where it is off, a bordered element
    // only has its border tinted (:269-271), and the 1px ring line a
    // borderless one gets instead (:273-297) does not apply to this overlay.
    let info = if t.focus_ring {
        info.color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/styled.rs:259",
        ))
        .color(claim(
            "focus ring, ring at 50%",
            "ring",
            t.ring.alpha(0.5),
            "gpui-component/styled.rs:261",
        ))
    } else {
        info.color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/styled.rs:270",
        ))
    };
    info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .config("focus_ring", format!("{}", t.focus_ring))
        .not_themeable("slide fill", "none: a Carousel paints no slide background. The one theme colour it draws is the focus ring, on an overlay the size of the frame while focus is visible (carousel/carousel.rs, Carousel)")
        .not_themeable("snap motion", "Theme::motion's spring_move (carousel/carousel.rs, spring_move); ResolvedTheme has no motion field")
        .not_themeable("reduced motion", "gpui's App::reduce_motion, which native_theme_gpui::next::apply switches on where the installed theme's motion is reduced (the showcase installs every theme through it) -- the spring jumps straight to its target (gpui-base/motion.rs, spring)")
        .not_themeable("slide controls", "outline Buttons the widget builds itself, disabled at the ends (carousel/carousel.rs, carousel_control)")
        .instance("animation", if reduce_motion {
            "none: reduced motion is on, so a change of slide jumps straight to it (gpui-base/motion.rs, spring)"
        } else {
            "a change of slide springs the track to it on spring_move (carousel/carousel.rs, CarouselContent)"
        })
        .not_themeable("own icons", super::own_icons("ChevronLeft and ChevronRight on its Previous and Next Buttons (carousel/carousel.rs, carousel_control)"))
        .not_themeable("side room", format!("{} on either side, CAROUSEL_SIDE_ROOM, the showcase's own choice rather than a platform's: the theme states no distance for it, and the Previous and Next Buttons stand outside the frame, mr_4 and ml_4 past its edges (carousel/carousel.rs, carousel_control), so the page keeps their room free", super::rems_text(crate::pages::CAROUSEL_SIDE_ROOM, t)))
        .instance("controls", "the Previous and Next Buttons place themselves outside the frame, positioned against the Carousel (carousel/carousel.rs, carousel_control), so a wrapper around one would become what it is positioned against: they report through the Carousel. The slides and the page buttons report themselves")
}

/// Slide `index` of the Carousel, titled `title` over `caption`: the
/// showcase's own content in a `CarouselItem`.
pub fn carousel_slide(t: &Theme, index: usize, title: &str, caption: &str) -> WidgetInfo {
    WidgetInfo::new("CarouselItem")
        .variant(format!("slide {}", index.saturating_add(1)))
        .color(claim("slide bg", "muted", t.muted, "showcase"))
        .color(claim(
            "title",
            "foreground",
            t.foreground,
            "gpui-component/label.rs:248",
        ))
        .color(claim(
            "caption",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("item", "a CarouselItem paints nothing of its own and only pads its slide from the one before (carousel/carousel.rs, CarouselItem): the fill, the rounding and the text colours are the showcase's")
        .not_themeable(
            "slide padding",
            format!(
                "{} round the slide's content, CAROUSEL_SLIDE_PADDING, and {} between its title and its caption, CAROUSEL_SLIDE_GAP: the showcase's own choices rather than a platform's, as the theme states no slide (spec §3.1)",
                super::rems_text(crate::demo::CAROUSEL_SLIDE_PADDING, t),
                super::rems_text(crate::demo::CAROUSEL_SLIDE_GAP, t)
            ),
        )
        .instance("title", title.to_string())
        .instance("caption", caption.to_string())
}

/// The page button for slide `index` of the Carousel, `selected` while
/// that slide is shown: a compact XSmall Default `Button` the
/// `CarouselPaginationItem` builds.
pub fn carousel_page(t: &Theme, index: usize, selected: bool) -> WidgetInfo {
    let info = WidgetInfo::new("CarouselPaginationItem");
    // A selected Button takes its selected style and no hover or press
    // style (button/button.rs:730, :755, :823-828).
    let info = if selected {
        info.variant("selected")
            .color(claim(
                "bg",
                "button_active",
                t.button_active,
                "gpui-component/button/button.rs:1343",
            ))
            .color(claim(
                "text",
                "button_foreground",
                t.button_foreground,
                "gpui-component/button/button.rs:1050",
            ))
            .color(claim(
                "border",
                "input",
                t.input,
                "gpui-component/button/button.rs:1102",
            ))
            .not_themeable(
                "hover",
                "none while selected: a selected Button takes no hover or press style (button/button.rs, RenderOnce for Button)",
            )
    } else {
        info.color(claim(
            "bg",
            "button",
            t.button,
            "gpui-component/button/button.rs:1036",
        ))
        .color(claim(
            "text",
            "button_foreground",
            t.button_foreground,
            "gpui-component/button/button.rs:1050",
        ))
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/button/button.rs:1102",
        ))
        .color(claim(
            "hover",
            "button_hover",
            t.button_hover,
            "gpui-component/button/button.rs:1180",
        ))
        .color(claim(
            "active",
            "button_active",
            t.button_active,
            "gpui-component/button/button.rs:1264",
        ))
    };
    info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("size", "a compact Button at Size::XSmall, CarouselPaginationItem's default: h_5, at least w_5, px_1 -- rems, so the platform's font (carousel/carousel.rs, CarouselPaginationItem::new; button/button.rs, RenderOnce for Button)")
        .instance(
            "slide",
            format!("{}: a click shows that slide", index.saturating_add(1)),
        )
}

/// A `Stepper` of `kind` with `count` steps, step `step` the current one
/// (0-based). A horizontal one shows `icons`, one per step as `(label,
/// icon)`, of the chosen icon theme, which take
/// `next::geometry::icon_size_small` where a 0.8.0 theme is installed and
/// states `icons.sizes.small`, whose line is recorded where
/// `demo::stepper` applies it; a step whose icon that theme lacks shows its
/// number.
pub fn stepper(
    t: &Theme,
    kind: StepperKind,
    step: usize,
    count: usize,
    icons: &[(&str, SampleIcon)],
) -> WidgetInfo {
    // Steps up to the current one are checked (stepper/trigger.rs:106), so
    // there is always one; a pending one only before the last step. The
    // separator after step i is passed while i is before the current step
    // (stepper/item.rs:117).
    let pending = step.saturating_add(1) < count;
    let passed = step > 0 && count > 1;
    // (on a checked step, on a pending one): the icon or the number the
    // indicator holds takes the indicator's text colour (icon.rs, Icon).
    let shown = icons.iter().filter(|(_, icon)| icon.shown()).count();
    let (checked_mark, pending_mark) = match kind {
        StepperKind::Icons if shown == icons.len() => (
            claim(
                "icon, completed / current",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/stepper/trigger.rs:136",
            ),
            claim(
                "icon, pending",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/stepper/trigger.rs:133",
            ),
        ),
        StepperKind::Icons if shown > 0 => (
            claim(
                "icon or number, completed / current",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/stepper/trigger.rs:136",
            ),
            claim(
                "icon or number, pending",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/stepper/trigger.rs:133",
            ),
        ),
        StepperKind::Icons | StepperKind::Numbers => (
            claim(
                "number, completed / current",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/stepper/trigger.rs:136",
            ),
            claim(
                "number, pending",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/stepper/trigger.rs:133",
            ),
        ),
    };
    let info = WidgetInfo::new("Stepper")
        .variant(kind.name())
        .color(claim(
            "completed / current",
            "primary",
            t.primary,
            "gpui-component/stepper/trigger.rs:135",
        ))
        .color(checked_mark);
    let info = if pending {
        info.color(claim(
            "pending",
            "secondary",
            t.secondary,
            "gpui-component/stepper/trigger.rs:128",
        ))
        .color(pending_mark)
        .color(claim(
            "pending hover",
            "secondary_hover",
            t.secondary_hover,
            "gpui-component/stepper/trigger.rs:130",
        ))
        .color(claim(
            "pending pressed",
            "secondary_active",
            t.secondary_active,
            "gpui-component/stepper/trigger.rs:131",
        ))
        .color(claim(
            "separator",
            "border",
            t.border,
            "gpui-component/stepper/item.rs:265",
        ))
    } else {
        info
    };
    let info = if passed {
        info.color(claim(
            "passed separator",
            "primary",
            t.primary,
            "gpui-component/stepper/item.rs:266",
        ))
    } else {
        info
    };
    let info = info
        .not_themeable("indicator size", "24px for Size::Medium (stepper/item.rs, StepperItem::render icon_size)")
        .not_themeable("separator", "drawn by the item, absolute (stepper/item.rs: StepperItem::render builds it, StepperSeparator::render positions it)");
    let info = match kind {
        StepperKind::Icons => info
            .instance("indicator", "each step's icon, which keeps its own size inside the circle; a step given no icon shows its number (stepper/trigger.rs, StepperTrigger)")
            .instance(
                "icons",
                icons
                    .iter()
                    .map(|(label, icon)| format!("{label}: {}", icon.note("the step shows its number")))
                    .collect::<Vec<_>>()
                    .join("; "),
            ),
        StepperKind::Numbers => info.instance("indicator", "no icon is given, so each step shows its number (stepper/trigger.rs, StepperTrigger)"),
    };
    info.instance(
        "steps",
        format!(
            "{count}; step {} is current, so {step} completed and {} pending. A click on a step makes it current, in both Steppers",
            step.saturating_add(1),
            count.saturating_sub(step.saturating_add(1))
        ),
    )
    .instance("items", "Stepper::item takes a StepperItem, which the Stepper numbers and renders itself (stepper/stepper.rs, Stepper::item), so a step cannot be wrapped: the Stepper reports for its steps. Each step's label is a Label that reports itself")
}

/// The `Breadcrumb`, whose items but the last show a page.
pub fn breadcrumb(t: &Theme, pages: &[Page], current: Page) -> WidgetInfo {
    let links = pages
        .iter()
        .map(|page| page.label())
        .collect::<Vec<_>>()
        .join(", ");
    let current = current.label();
    WidgetInfo::new("Breadcrumb")
        .color(claim(
            "items",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/breadcrumb.rs:102",
        ))
        .color(claim(
            "last item",
            "foreground",
            t.foreground,
            "gpui-component/breadcrumb.rs:103",
        ))
        .color(claim(
            "separators",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/breadcrumb.rs:150",
        ))
        .not_themeable("own icons", super::own_icons("the ChevronRight between its items, ChevronLeft in a right-to-left layout (breadcrumb.rs, BreadcrumbSeparator)"))
        .not_themeable("separator icon", "a ChevronRight built inline with no setter to replace it, a ChevronLeft in a right-to-left layout (breadcrumb.rs, BreadcrumbSeparator) -- unlike an Alert's icon, which Alert::icon takes")
        .not_themeable("spacing", "gap_1p5 (breadcrumb.rs, Breadcrumb) -- rems again -- and applied before the caller's refinement. native-theme states no breadcrumb widget. Our gap")
        .instance("items", format!("{links} and {current}: a click on one of the first {} shows that page, and {current}, the last, is this page. Breadcrumb::child takes a BreadcrumbItem that the Breadcrumb renders itself (breadcrumb.rs, Breadcrumb::child), so an item cannot be wrapped: the Breadcrumb reports for its items", pages.len()))
}

/// The horizontal `Form` of the Layout page, its labels `label_width` wide,
/// whose Fields' Inputs report themselves.
pub fn form(t: &Theme, label_width: Pixels) -> WidgetInfo {
    WidgetInfo::new("Form")
        .variant("horizontal")
        // The label's box sets a size and a weight but no colour
        // (form/field.rs:302-307), so the label takes the colour the
        // showcase sets on its window.
        .color(claim(
            "label, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .color(claim(
            "description",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/form/field.rs:346",
        ))
        .color(claim(
            "required marker",
            "danger",
            t.danger,
            "gpui-component/form/field.rs:323",
        ))
        .not_themeable("fields", "the Field wrapper takes no geometry -- the model has no form theme. Each field's Input is refined by geometry::input and reports itself")
        .not_themeable("label style", "text_sm and font_medium on a box that sets no colour (form/field.rs, Field)")
        .instance("layout", "horizontal: Form::horizontal puts each label beside its field, and the description under the field (form/field.rs, Field)")
        .instance(
            "label width",
            format!(
                "{}px, the showcase's own through Form::label_width; upstream's default is 140px (form/field.rs, FieldProps)",
                px_text(label_width.as_f32())
            ),
        )
        .instance("fields", "Name, required, and Email, with a description. Form::child takes a Field (form/form.rs, Form::child), which the Form lays out itself, so a field cannot be wrapped: the Form reports for its fields")
}

/// The scrollable area, a column of Labels in a container gpui-component's
/// `overflow_y_scrollbar` scrolls. `styled` is whether a native theme is
/// installed, whose scrollbar the connector writes onto gpui-base; its
/// gutter's geometry line is recorded where `demo::scroll_area` applies the
/// builder.
pub fn scroll_area(t: &Theme, styled: bool, items: usize) -> WidgetInfo {
    let info = WidgetInfo::new("Scrollbar")
        .color(claim(
            "track",
            "scrollbar",
            t.scrollbar,
            "gpui-component/theme/mod.rs:435",
        ))
        .color(claim(
            "thumb",
            "scrollbar_thumb",
            t.scrollbar_thumb,
            "gpui-component/theme/mod.rs:440",
        ))
        .color(claim(
            "thumb hover",
            "scrollbar_thumb_hover",
            t.scrollbar_thumb_hover,
            "gpui-component/theme/mod.rs:451",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .config(
            "show mode",
            format!("scrollbar_mode: {:?}", t.scrollbar_mode),
        );
    let info = if styled {
        info.config("track and thumb", "scrollbar.track(NONE; orientation=Vertical).min_size[0] wide, the thumb scrollbar.thumb(NONE; orientation=Vertical).min_size[0] wide and at least scrollbar.min_thumb_length long, in scrollbar.track(NONE).fill, scrollbar.thumb(UNFOCUSED_VIEW).fill at rest and scrollbar.thumb(HOVER).fill -- the colours ThemeColor holds as scrollbar, scrollbar_thumb and scrollbar_thumb_hover -- written onto gpui-base by the connector after every rebuild (native-theme-gpui/next/mod.rs, scrollbar_geometry; native-theme-gpui/base_layer.rs, scrollbar_styles)")
    } else {
        info
    };
    info.instance(
        "content",
        format!(
            "{items} Labels, each of which reports itself; the strip beside them is the scrollbar's"
        ),
    )
    .not_themeable(
        "content spacing",
        format!(
            "{} between the Labels, SCROLL_SAMPLE_GAP, and {} round them, SCROLL_SAMPLE_PADDING: the showcase's own choices rather than a platform's, as the theme states no such column (spec §3.1)",
            super::rems_text(crate::demo::SCROLL_SAMPLE_GAP, t),
            super::rems_text(crate::demo::SCROLL_SAMPLE_PADDING, t)
        ),
    )
}

/// The Layout page's `Settings` sample. Its geometry line is recorded where
/// `demo::settings` applies the builder to its groups.
pub fn settings(t: &Theme) -> WidgetInfo {
    super::chrome::settings(t, "two pages")
        .not_themeable("own icons", super::own_icons("Search before its sidebar's search field (setting/settings.rs, Settings), Undo2 on the reset button a page shows while one of its settings can be reset (setting/page.rs, SettingPage), the checkbox field's Check while it is checked (setting/fields/bool.rs, BoolField; checkbox.rs, checkbox_check_icon), and the dropdown field's caret, ChevronDown, and Check beside the chosen option in its menu (setting/fields/dropdown.rs, DropdownField; select.rs, Caret; menu/popup_menu.rs, PopupMenu)"))
        .color(claim(
            "group title",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/group_box.rs:158",
        ))
        .not_themeable("fill", "none of its own: nothing under setting/ sets a background, so a Settings page shows the window's (gpui-component setting/)")
        .not_themeable("sidebar width", "250px, upstream's default (setting/settings.rs, Settings::sidebar_width), kept between 160px and 360px (setting/settings.rs, sidebar_size_range), the range a drag of its handle stays in and a width set outside it is clamped to (gpui-base/resizable/panel.rs, ResizablePanel). px literals, and native-theme states no settings page")
        .not_themeable("layout", "the label above the field wherever the page is at most 480px wide, and beside it where it is wider (setting/settings.rs, STACKED_LAYOUT_MAX_WIDTH)")
        .instance("pages", "Appearance, open, with the groups Theme and Editor, and Keyboard, with Shortcuts; the sidebar lists the pages")
        .instance("fields", "switch, checkbox, input, number input, dropdown, or an element of the application's own (setting/fields/mod.rs, SettingFieldType). This sample shows the first three and a dropdown, which upstream builds inside each SettingItem, so they report through the Settings. The Theme group also holds a whole-row item of the showcase's own, a 1px strip built with SettingItem::render (setting/item.rs, SettingItem::render), whose right edge is where a row ends: the self-tests measure it against the page's scrollbar")
}

/// The Layout page's `GridLayout`, gpui-component's `grid_layout` over the
/// showcase's own cards.
pub fn grid_layout(t: &Theme) -> WidgetInfo {
    let info = WidgetInfo::new("GridLayout").color(claim(
        "selected card's ring, 2px",
        "primary",
        t.primary,
        "gpui-component/grid_layout.rs:51",
    ));
    let info = if t.focus_ring {
        info.color(claim(
            "keyboard's card, ring at 50%, 3px",
            "ring",
            t.ring.alpha(0.5),
            "gpui-component/grid_layout.rs:58",
        ))
    } else {
        info.color(claim(
            "keyboard's card, ring, 1px",
            "ring",
            t.ring,
            "gpui-component/grid_layout.rs:61",
        ))
    };
    info.color(claim(
        "drop placeholder, primary at 8%",
        "primary",
        t.primary.opacity(0.08),
        "gpui-component/grid_layout.rs:79",
    ))
    .color(claim(
        "drop placeholder's dashed edge, primary at 50%",
        "primary",
        t.primary.opacity(0.5),
        "gpui-component/grid_layout.rs:82",
    ))
    .color(claim(
        "resize grip, at 35%",
        "muted_foreground",
        t.muted_foreground.opacity(0.35),
        "gpui-component/grid_layout.rs:86",
    ))
    .color(claim(
        "resize grip under the pointer, at 90%",
        "muted_foreground",
        t.muted_foreground.opacity(0.9),
        "gpui-component/grid_layout.rs:86",
    ))
    .config(
        "border-radius",
        format!(
            "radius_lg: {}px, the drop placeholder's corners",
            px_text(t.radius_lg.as_f32())
        ),
    )
    .config("focus_ring", format!("{}", t.focus_ring))
    .not_themeable("gap", format!("{}px, spacing_tokens().md: Theme::spacing_tokens returns SpacingTokens::default() with no field behind it, so no theme reaches it (gpui-component/grid_layout.rs:65; gpui-component/theme/mod.rs:596-598; gpui-base/theme_tokens.rs:146-158)", px_text(t.spacing_tokens().md.as_f32())))
    .not_themeable("motion", "cards settle on Theme::motion's duration_normal and easing_move (gpui-component/grid_layout.rs:66); ResolvedTheme has no motion field, and apply keeps the application's")
    .not_themeable("dragged card", "shadow_lg, two black layers at 10%, gpui's literal (gpui-component/grid_layout.rs:67, dragging_item)")
    .not_themeable("rings", "both are the card's shadow, so the focus ring replaces the selection ring while the keyboard drives the grid, and a read-only grid shows no selection (gpui-component/grid_layout.rs:28-31, :68-73)")
    .not_themeable("placeholder", "a 1px dashed edge round the cells the dragged card will take (gpui-component/grid_layout.rs:74-84)")
    .not_themeable("resize grip", "a 6px corner mark 3px in from the card's corner, two 2px edges (gpui-component/grid_layout.rs:94-99), in a 16px hit corner (gpui-base/grid_layout/grid_layout.rs:22, RESIZE_HANDLE_SIZE): literals")
    .not_themeable("row height", "2.5 rem unless the caller sets one, a literal (gpui-base/grid_layout/grid_layout.rs, GridLayout::row_height)")
    .instance("cards", format!("{} of the showcase's own cards in 12 columns (GRID_LAYOUT_ITEMS), each framed and rounded as the showcase frames a demonstration (demo_frame)", GRID_LAYOUT_ITEMS.len()))
    .instance("editing", "drag a card to move it and its bottom-right corner to resize it; a click selects one; with the grid focused the arrow keys move it, Shift with them resizes it, Tab moves the selection (gpui-base/grid_layout/grid_layout.rs, GridLayout)")
}

/// One of the Layout page's `Sidebar` samples (spec S5), `collapsed` to its
/// icon rail or expanded, its header reading `header`.
pub fn sidebar(t: &Theme, collapsed: bool, header: &str) -> WidgetInfo {
    let info = WidgetInfo::new("Sidebar")
        .variant(if collapsed { "collapsed" } else { "expanded" })
        .color(claim(
            "bg",
            "sidebar",
            t.sidebar,
            "gpui-component/sidebar/mod.rs:435",
        ))
        .color(claim(
            "text",
            "sidebar_foreground",
            t.sidebar_foreground,
            "gpui-component/sidebar/mod.rs:436",
        ))
        .color(claim(
            "border",
            "sidebar_border",
            t.sidebar_border,
            "gpui-component/sidebar/mod.rs:437",
        ))
        .not_themeable(
            "width",
            "SidebarTheme states no width -- our model's gap. The sample sets none, so upstream's own apply: its default width expanded (sidebar/mod.rs, DEFAULT_WIDTH) and its fixed icon rail collapsed (sidebar/mod.rs, COLLAPSED_WIDTH)",
        )
        .not_themeable(
            "padding",
            "px_3 around the items, p_2 while collapsed -- rems, so the platform's font -- and not settable: upstream drops the caller's padding before it lays the Sidebar out (sidebar/mod.rs, RenderOnce for Sidebar)",
        )
        .instance(
            "items",
            "three SidebarMenuItems, the first active, each with its icon from the chosen icon theme where that theme has one: an item whose icon it lacks shows its label alone expanded and nothing in the rail, and no other icon theme's icon stands in",
        )
        .instance(
            "header",
            if collapsed {
                format!("{header}, as plain text truncated with an ellipsis to the rail's width: upstream draws the header in the rail too, padded pt_2 and px_2 (sidebar/mod.rs, RenderOnce for Sidebar)")
            } else {
                format!("{header}, as plain text in the slot upstream pads with pt_3 and px_3 (sidebar/mod.rs, RenderOnce for Sidebar)")
            },
        )
        .instance(
            "height",
            "the sample's own, in rems, so it grows with the text as the rows do: the items are a list that takes the height it is given (sidebar/mod.rs, RenderOnce for Sidebar)",
        )
        .instance(
            "children",
            "must implement SidebarItem, which asks for Collapsible + Clone (sidebar/mod.rs, SidebarItem)",
        );
    if collapsed {
        info.instance(
            "collapsed",
            "to its icons: each item shows its icon alone, its label as a tooltip (sidebar/menu.rs, collapsed_tooltip)",
        )
    } else {
        info
    }
}

/// A Sidebar sample's item labelled `label`, `active` or not, `collapsed`
/// while its Sidebar is, showing `icon` of the icon theme named `set`. Its
/// icon-size line is recorded where the item is built.
pub fn sidebar_item(
    t: &Theme,
    label: &'static str,
    active: bool,
    collapsed: bool,
    icon: &ChromeIcon,
    set: &str,
) -> WidgetInfo {
    let info = WidgetInfo::new("SidebarMenuItem").variant(if active {
        format!("{label}, active")
    } else {
        label.to_string()
    });
    let info = if active {
        info.color(claim(
            "bg",
            "sidebar_accent",
            t.sidebar_accent,
            "gpui-component/sidebar/menu.rs:321",
        ))
        .color(claim(
            "text",
            "sidebar_accent_foreground",
            t.sidebar_accent_foreground,
            "gpui-component/sidebar/menu.rs:322",
        ))
    } else {
        info.color(claim(
            "text (the Sidebar's)",
            "sidebar_foreground",
            t.sidebar_foreground,
            "gpui-component/sidebar/mod.rs:436",
        ))
        .color(claim(
            "hover bg, at 80%",
            "sidebar_accent",
            t.sidebar_accent.opacity(0.8),
            "gpui-component/sidebar/menu.rs:315",
        ))
        .color(claim(
            "hover text",
            "sidebar_accent_foreground",
            t.sidebar_accent_foreground,
            "gpui-component/sidebar/menu.rs:316",
        ))
    };
    let info = info
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable(
            "hover",
            "sidebar_accent at 80%, the selection colour: the model's sidebar.hover_background reaches no slot, because the connector reads it nowhere (sidebar/menu.rs, SidebarMenuItem)",
        )
        .not_themeable(
            "font",
            "text_sm, and font_medium while active -- sidebar.font's size and weight have no route, as no geometry:: builder carries them (sidebar/menu.rs, SidebarMenuItem)",
        )
        .not_themeable(
            "height",
            "h_7 while expanded, set after the caller's refinement, so nothing reaches it -- rems, so the platform's font. SidebarTheme states no row height (sidebar/menu.rs, SidebarMenuItem)",
        )
        .instance(
            "icon",
            chrome_icon_note(icon, set, "the item shows its label alone"),
        );
    match (collapsed, icon) {
        (true, ChromeIcon::Missing(_) | ChromeIcon::Unlisted(_)) => info.instance(
            "collapsed",
            "the label is hidden, and with no icon the item shows nothing, and has no tooltip either: upstream gives a collapsed item its label as a tooltip only when it has an icon (sidebar/menu.rs, collapsed_tooltip)",
        ),
        (true, _) => info.instance(
            "collapsed",
            "only the icon shows, and the label becomes a tooltip at its right (sidebar/menu.rs, collapsed_tooltip)",
        ),
        (false, _) => info,
    }
}
