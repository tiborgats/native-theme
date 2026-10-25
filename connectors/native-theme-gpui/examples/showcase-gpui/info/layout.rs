//! What the Layout page's widgets report about themselves (spec §3.4).

use gpui::Pixels;
use gpui_component::theme::Theme;
#[cfg(feature = "widgets")]
use native_theme::theme::TabIndicatorSide;
use native_theme::theme::{ArrowSide, ResolvedExpanderTheme, ResolvedPadding};
use native_theme_gpui::ResolvedTheme;
#[cfg(feature = "widgets")]
use native_theme_gpui::widgets::TabLook;

use super::{
    WidgetInfo,
    chrome::{GhostContent, chrome_icon_note, ghost_colours},
    claim, drawn_font, edge_claim, px_text, stated, stated_claim,
};
use crate::Page;
use crate::demo::{
    FOLDER_TAB_HEIGHT, GroupBoxKind, SEGMENT_PADDING, SeparatorKind, SpacingBox, StepperKind,
};
use crate::support::{ChromeIcon, SampleIcon, layout_value};

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
    let info = info.not_themeable("receivers", "none a theme can write: Theme::spacing_tokens() returns SpacingTokens::default() with no field behind it (theme/mod.rs, spacing_tokens), and its one reader, a Dialog's viewport margin, gets that default (dialog/dialog.rs, Dialog). So these four are the application's to apply (geometry.rs §9.5)");
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

/// A plain line drawn by `native_theme_gpui::widgets::Separator` from `r`,
/// with `room` above and below where `demo::separator` gives it some.
#[cfg(feature = "widgets")]
pub fn native_separator(t: &Theme, r: &ResolvedTheme, room: bool) -> WidgetInfo {
    let s = &r.separator;
    let info = WidgetInfo::new("Separator");
    let info = if room {
        info.not_themeable("room", sample_room(t))
    } else {
        info
    };
    info
        .variant(SeparatorKind::Horizontal.name())
        .config("drawn by", "native_theme_gpui::widgets::Separator, a line as thick as the theme states and one device pixel at least, as gpui draws a border: gpui-component's Separator draws its line an absolute child at px(1.), which no caller style reaches (separator.rs, Separator::render_base)")
        .color(claim(
            "line",
            "line_color",
            stated(s.line_color),
            "native-theme-gpui/widgets/separator.rs:30",
        ))
        .config(
            "thickness",
            format!("separator.line_width: {}px", px_text(s.line_width)),
        )
}

/// A `Separator` of `kind`, its line `native`'s `separator.line_color`
/// where a native theme is installed (`demo::separator`).
pub fn separator(t: &Theme, kind: SeparatorKind, native: Option<&ResolvedTheme>) -> WidgetInfo {
    let info = WidgetInfo::new("Separator").variant(kind.name());
    let info = match native {
        Some(r) => info.color(claim(
            "line",
            "line_color",
            stated(r.separator.line_color),
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
            "Tier U, not an absence: the platform states separator.line_width and the model carries it. Upstream draws the line on an inner absolutely-positioned div at px(1.) and applies the caller's refinement to the outer container instead, so nothing reaches the line (separator.rs, Separator::render_base)",
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
/// the side `card.border.padding` states (`geometry::group_box_content`),
/// else `margin`, the installed layout's `container_margin`, which the
/// showcase pads an unstated side with where a native theme is installed
/// (`demo::padded`), else `own`, the variant's own.
fn content_padding(native: Option<&ResolvedTheme>, margin: Option<Pixels>, own: &str) -> String {
    let side = |name: &str, stated: Option<f32>| match (native, stated, margin) {
        (Some(_), Some(v), _) => format!("{name} {}px, card.border's", px_text(v)),
        (Some(_), None, Some(m)) => {
            format!("{name} {}px, layout.container_margin", px_text(m.as_f32()))
        }
        _ => format!("{name} {own}"),
    };
    let p = native.map(|r| r.card.border.padding).unwrap_or_default();
    format!(
        "{}; {}; {}; {}. A side card.border.padding leaves unstated takes layout.container_margin, the padding inside containers (platform-facts §2.20), where a native theme is installed and its layout states one, else the variant's own (group_box.rs, GroupBox)",
        side("top", p.top),
        side("right", p.right),
        side("bottom", p.bottom),
        side("left", p.left),
    )
}

/// `info` with the edge `geometry::group_box_content` draws on a GroupBox's
/// content, `card.border.color` at `card.border.line_width`, and its line:
/// none where that is 0, which an `outline` box's own edge gives way to.
fn card_edge(info: WidgetInfo, r: &ResolvedTheme, outline: bool) -> WidgetInfo {
    let width = r.card.border.line_width;
    info.colors(edge_claim(
        "edge",
        "border",
        Some(r.card.border.color),
        Some(width),
        "native-theme-gpui/geometry.rs:739",
    ))
    .config("edge", match (width > 0.0, outline) {
        (true, _) => format!("card.border.color at card.border.line_width, {}px, on every variant: geometry::group_box_content sets both on the content, which the GroupBox refines last (group_box.rs, GroupBox) -- over an Outline's own border colour, and on a Normal or Fill box, which draws no edge of its own", px_text(width)),
        (false, true) => "none: card.border.line_width is 0, and geometry::group_box_content sets that width on the content, which the GroupBox refines last, so the Outline box's own border_1 gives way to it (group_box.rs, GroupBox)".to_string(),
        (false, false) => "none: card.border.line_width is 0, so geometry::group_box_content draws no edge on the content, and a Normal or Fill box draws none of its own (group_box.rs, GroupBox)".to_string(),
    })
}

/// A `GroupBox` of `kind` titled `title`. `native` is the theme
/// `geometry::group_box_content` refined its content from, where a native
/// theme is installed, whose line is recorded where `demo::group_box`
/// applies the builder, and `margin` the installed layout's
/// `container_margin`, which pads the sides the theme leaves unstated. The
/// caller adds what the content is.
pub fn group_box(
    t: &Theme,
    kind: GroupBoxKind,
    native: Option<&ResolvedTheme>,
    margin: Option<Pixels>,
    title: &str,
) -> WidgetInfo {
    let styled = native.is_some();
    let info = WidgetInfo::new("GroupBox")
        .variant(kind.name())
        .color(claim(
            "title",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/group_box.rs:158",
        ))
        // A Label would paint foreground over this on its own element
        // (label.rs:211), so the content's text is plain text.
        .color(claim(
            "content text",
            "group_box_foreground",
            t.group_box_foreground,
            "gpui-component/group_box.rs:173",
        ));
    // (fill, edge) per variant (group_box.rs:143-147), under the card's
    // fill and edge where the builder refined the content.
    let info = match (kind, native) {
        (_, Some(r)) => card_edge(
            info.color(claim(
                "fill",
                "background_color",
                stated(r.card.background_color),
                "showcase",
            )),
            r,
            kind == GroupBoxKind::Outline,
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
    let info = match (kind, styled) {
        (_, true) => info,
        (GroupBoxKind::Outline, false) => {
            info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        }
        (GroupBoxKind::Normal | GroupBoxKind::Fill, false) => info
            .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
            .not_themeable(
                "edge",
                "none: only the Outline variant draws a border (group_box.rs, GroupBox)",
            ),
    };
    let info = match (kind, styled) {
        (_, true) => info.config(
            "fill",
            "card.background_color, on every variant: geometry::group_box_content sets it on the content, which the GroupBox refines last (group_box.rs, GroupBox) -- over a Fill box's group_box token, and on a Normal or Outline box, which paints no background of its own",
        ),
        (GroupBoxKind::Normal, false) => info.not_themeable(
            "fill",
            "none: the Normal variant paints no background (group_box.rs, GroupBox)",
        ),
        (GroupBoxKind::Fill | GroupBoxKind::Outline, false) => info,
    };
    let own = match kind {
        GroupBoxKind::Normal => "none, the Normal variant's own",
        GroupBoxKind::Fill => "p_4, the Fill variant's own",
        GroupBoxKind::Outline => "p_4, the Outline variant's own",
    };
    let info = if styled {
        info.config("padding", content_padding(native, margin, own))
    } else {
        info.not_themeable("padding", content_padding(None, None, own))
    };
    info.not_themeable(
        "gaps",
        "gap_3 between the title and the content on a Fill or Outline box and gap_4 on a Normal one -- rems, so the platform's font -- and settable: the GroupBox refines its outer box after them (group_box.rs, GroupBox)",
    )
    .instance("title", title.to_string())
}

/// The `Accordion`, drawn while gpui's `reduce_motion` is as given. `styled`
/// is whether a native theme is installed, so whether `geometry::accordion`
/// coloured its edges and `geometry::accordion_title` its titles, whose
/// geometry lines -- recorded where `demo::accordion` applies the builders --
/// then name the colours.
pub fn accordion(t: &Theme, reduce_motion: bool, items: usize, styled: bool) -> WidgetInfo {
    upstream_accordion(WidgetInfo::new("Accordion"), t, reduce_motion, styled)
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
        false,
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
/// page's and, without a native theme, the Basic page's expanders. `styled`
/// as [`accordion`] takes it.
fn upstream_accordion(
    info: WidgetInfo,
    t: &Theme,
    reduce_motion: bool,
    styled: bool,
) -> WidgetInfo {
    let info = info
        .not_themeable(
            "own icons",
            super::own_icons("ChevronDown on each item's header, turned while the item is open (gpui-component/accordion.rs, AccordionItem::render)"),
        )
        .color(claim(
            "bg",
            "accordion",
            t.accordion,
            "gpui-component/accordion.rs:378",
        ));
    let info = if styled {
        info
    } else {
        info.color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/accordion.rs:110",
        ))
        .color(claim(
            "line between items",
            "border",
            t.border,
            "gpui-component/accordion.rs:381",
        ))
        .color(claim(
            "open item's title",
            "foreground",
            t.foreground,
            "gpui-component/accordion.rs:306",
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
        .not_themeable("hover", "none: no native theme is installed, so the showcase gives the items no hover style (gpui-component/accordion.rs, AccordionItem::hover)")
    };
    info.color(claim(
            "chevron",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/accordion.rs:330",
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
        .not_themeable("chevron", "a ChevronDown built inline at XSmall in muted_foreground, with no setter (gpui-component/accordion.rs, AccordionItem::render), so expander.arrow_icon_size and expander.arrow_color have no receiver -- Tier U")
        .not_themeable("animation", "reads the theme's spring_control (gpui-component/accordion.rs, AccordionItem). Theme::motion is a writable field (theme/mod.rs, MotionTokens) the connector leaves at its default, because native-theme models no motion -- our model's gap, not upstream's")
        .instance("animation", if reduce_motion {
            "none: reduced motion is on, so an item's content opens and closes at once (gpui-base/motion.rs, spring)"
        } else {
            "an item's content springs open and shut on spring_control; the chevron turns at once (gpui-component/accordion.rs, AccordionItem::render)"
        })
}

/// A card: a Fill `GroupBox` with no title, its content filled with
/// `native`'s `card.background_color` where a native theme is installed
/// (`geometry::group_box_content`). Its geometry line is recorded where the
/// helper applies the builder.
pub fn card(
    t: &Theme,
    native: Option<&ResolvedTheme>,
    container_margin: Option<Pixels>,
) -> WidgetInfo {
    // No content text: the content is a Label, which paints foreground on
    // its own element over the group box's colour (label.rs:211) and
    // reports itself.
    let info = WidgetInfo::new("GroupBox").variant("Fill, a card");
    let info = match native {
        Some(r) => card_edge(
            info.color(claim(
                "fill",
                "background_color",
                stated(r.card.background_color),
                "showcase",
            )),
            r,
            false,
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
        Some(_) => info.config("padding", content_padding(native, container_margin, own)),
        None => info.not_themeable("padding", content_padding(None, None, own)),
    };
    info.instance("content", "a body-text Label, which reports itself")
}

/// A `TabBar` of the Tab variant over `labels`, the one at `selected`
/// shown, its per-call seams taken from `native` where a native theme is
/// installed (`demo::tab_row`).
pub fn tab_row(
    t: &Theme,
    labels: &[&'static str],
    selected: usize,
    native: Option<&ResolvedTheme>,
) -> WidgetInfo {
    let info = WidgetInfo::new("TabBar")
        .variant("Tab")
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
                "gpui-component/tab/tab.rs:172",
            ))
            .color(claim(
                "selected label",
                "tab_active_foreground",
                t.tab_active_foreground,
                "gpui-component/tab/tab.rs:276",
            ))
            .color(claim(
                "selected tab",
                "tab_active",
                t.tab_active,
                "gpui-component/tab/tab.rs:277",
            ))
            .color(claim(
                "selected tab's sides",
                "border",
                t.border,
                "gpui-component/tab/tab.rs:283",
            ))
            .color(claim(
                "rule under the bar",
                "border",
                t.border,
                "gpui-component/tab/tab_bar.rs:654",
            ))
            .not_themeable("hover", "transparent: a Tab-variant tab paints no hover fill, and the Tab sets its hover style itself, which replaces one a caller gives (tab/tab.rs, TabVariant::hovered and RenderOnce for Tab), so tab.hover_background has no receiver")
            .not_themeable("edges", "the selected tab's 1px sides and the bar's 1px bottom rule in border, px literals set inside render (tab/tab.rs, TabVariant::selected; tab/tab_bar.rs, RenderOnce for TabBar), while the model's tab.border states the tab's own edge")
            .not_themeable("padding", "the tab's inner padding is a px literal per variant and Size on a child the Tab builds (tab/tab.rs, TabVariant::inner_paddings), so a caller's padding adds to it rather than replacing it, and the model's tab.border.padding is not applied");
    let info = match native {
        Some(r) => info
            .config("font", format!("tab.font: {}, on each label, a child over the text_sm the Tab sets on itself (tab/tab.rs, RenderOnce for Tab)", drawn_font(&r.tab.font)))
            .config("size", format!("at least tab.min_width, {}px, by tab.min_height, {}px, through the Tab's style, which its own h does not clear (tab/tab.rs, RenderOnce for Tab)", px_text(r.tab.min_width), px_text(r.tab.min_height))),
        None => info.not_themeable("font", "text_sm, the Tab's own: no native theme is installed (tab/tab.rs, RenderOnce for Tab)"),
    };
    let shown = labels.get(selected).copied().unwrap_or("none");
    info.instance("tabs", labels.join(", "))
        .instance("selected", shown)
        .instance("click", "selects the tab; the showcase keeps the state. A tab the bar clips is scrolled into view, the next one peeking past it (tab/tab_bar.rs, TabReveal)")
}

/// A `TabBar` of the Folder variant over `labels`, the one at `selected`
/// shown and the one at `disabled` taking no click, above the showcase's
/// panel, its per-call seams taken from `native` where a native theme is
/// installed (`demo::folder_tab_row`).
pub fn folder_tab_row(
    t: &Theme,
    labels: &[&'static str],
    selected: usize,
    disabled: usize,
    native: Option<&ResolvedTheme>,
) -> WidgetInfo {
    let from = |field: &str| match native {
        Some(_) => format!(", from defaults.border.{field}"),
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
            "gpui-component/tab/tab.rs:211",
        ))
        .color(claim(
            "hovered label",
            "tab_active_foreground",
            t.tab_active_foreground,
            "gpui-component/tab/tab.rs:266",
        ))
        .color(claim(
            "selected label",
            "tab_active_foreground",
            t.tab_active_foreground,
            "gpui-component/tab/tab.rs:317",
        ))
        .color(claim(
            "disabled label",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/tab/tab.rs:390",
        ))
        .color(claim(
            "selected tab",
            "tab_active",
            t.tab_active,
            "gpui-component/tab/tab.rs:998",
        ))
        .color(claim(
            "hovered tab",
            "secondary",
            t.secondary,
            "gpui-component/tab/tab.rs:1012",
        ))
        .color(claim(
            "separator",
            "border",
            t.border,
            "gpui-component/tab/tab.rs:1025",
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
        .not_themeable("height", "32px at the default size (20, 24 and 36px at XSmall, Small and Large), its content 24px tall (16, 18 and 28px), as Segmented's: px literals per Size (tab/tab.rs, TabVariant::height and TabVariant::inner_height)");
    let info = match native {
        Some(r) => info
            .config("font", format!("tab.font: {}, on each label, a child over the text_sm the Tab sets on itself (tab/tab.rs, RenderOnce for Tab)", drawn_font(&r.tab.font)))
            .config("size", format!("at least tab.min_width, {}px, by tab.min_height, {}px, through the Tab's style, which its own h does not clear (tab/tab.rs, RenderOnce for Tab)", px_text(r.tab.min_width), px_text(r.tab.min_height))),
        None => info.not_themeable("font", "text_sm, the Tab's own: no native theme is installed (tab/tab.rs, RenderOnce for Tab)"),
    };
    let shown = labels.get(selected).copied().unwrap_or("none");
    let inert = labels.get(disabled).copied().unwrap_or("none");
    info.instance("tabs", labels.join(", "))
        .instance("selected", shown)
        .instance("disabled", inert)
        .instance("click", "selects the tab; the showcase keeps the state. The disabled tab takes none (gpui-base tabs.rs, RenderOnce for Tab)")
        .instance("panel", "the showcase's own, under the bar, filled with tab_active, the colour of the selected tab's shape, so the tab joins it, and labelled in tab_active_foreground, the selected label's colour")
}

/// The expanders the showcase draws as `r`'s `expander.*` states them
/// (`demo::native_expander`), reading `titles`, each `open` or not, their
/// arrow `arrow`, of the chosen icon theme -- `turned`, the chevron an open
/// item shows where the set gives `arrow` as pixels -- and `gap`, the
/// `layout.widget_gap` that parts the items where the theme states the
/// frame.
pub fn native_expander(
    r: &ResolvedTheme,
    titles: [&'static str; 2],
    open: [bool; 2],
    arrow: &SampleIcon,
    turned: &SampleIcon,
    gap: Option<Pixels>,
) -> WidgetInfo {
    let e = &r.expander;
    // The frame is drawn at expander.border.line_width unless
    // frame_enabled is false: KDE's expander draws none (§2.27).
    let frame_width = Some(match e.frame_enabled {
        Some(false) => 0.0,
        _ => e.border.line_width,
    });
    let edge = match e.frame_enabled {
        Some(true) => edge_claim(
            "each item's frame",
            "border",
            Some(e.border.color),
            frame_width,
            "showcase",
        ),
        _ => edge_claim(
            "frame and the line between items",
            "border",
            Some(e.border.color),
            frame_width,
            "showcase",
        ),
    };
    let info = WidgetInfo::new("Expander")
        .variant("drawn by the showcase")
        .colors(edge)
        .color(claim("title", "font", stated(e.font.color), "showcase"));
    let leading = e.arrow_side == Some(ArrowSide::Leading);
    let glyph = match (leading, arrow.is_raster()) {
        (true, false) => {
            "ChevronRight, before the title and turned down while open, as Breeze and the HIG draw a leading arrow"
        }
        (false, false) => {
            "ChevronDown, after the title and turned while open, as upstream's Accordion draws it (gpui-component/accordion.rs, AccordionItem::render)"
        }
        (true, true) => {
            "ChevronRight, before the title, as Breeze and the HIG draw a leading arrow; pixels do not turn (gpui::img takes no transformation), so an open item shows the set's ChevronDown instead"
        }
        (false, true) => {
            "ChevronDown, after the title, as upstream's Accordion draws it (gpui-component/accordion.rs, AccordionItem::render); pixels do not turn (gpui::img takes no transformation), so an open item shows the set's ChevronUp instead"
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
            info.color(claim(
                "arrow",
                "arrow_color",
                stated(e.arrow_color.unwrap_or(e.font.color)),
                "showcase",
            ))
        };
        info.config("arrow", format!("expander.arrow_icon_size, {}px", px_text(e.arrow_icon_size)))
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
        "hover_background",
        e.hover_background,
        "showcase",
    ) {
        Some(hover) => info.color(hover),
        None => info.not_themeable(
            "hover",
            "none: the theme states no expander.hover_background",
        ),
    };
    let shown: Vec<&str> = titles
        .iter()
        .zip(open)
        .filter(|(_, open)| *open)
        .map(|(title, _)| *title)
        .collect();
    info.config(
        "title row",
        format!(
            "expander.header_height, {}px, the title in expander.font: {}",
            px_text(e.header_height),
            drawn_font(&e.font)
        ),
    )
    .config("edge", expander_edge(e, gap))
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

/// What frames the theme-drawn expander `e` and parts its items, the
/// items `gap` (`layout.widget_gap`) apart where each is framed on its own.
fn expander_edge(e: &ResolvedExpanderTheme, gap: Option<Pixels>) -> String {
    let b = &e.border;
    match e.frame_enabled {
        Some(false) => {
            "none: expander.frame_enabled is false, no frame and no line between the items"
                .to_string()
        }
        _ if b.line_width <= 0.0 => format!(
            "none: expander.border.line_width is 0; the items are still rounded by its corner_radius, {}px",
            px_text(b.corner_radius)
        ),
        Some(true) => {
            let apart = match gap {
                Some(gap) => format!("layout.widget_gap, {}px, apart", px_text(gap.as_f32())),
                None => "with no gap between them: layout.widget_gap is unstated".to_string(),
            };
            format!(
                "each item framed on its own by expander.border.color at expander.border.line_width, {}px, radius {}px, {apart}",
                px_text(b.line_width),
                px_text(b.corner_radius)
            )
        }
        None => format!(
            "expander.border.color at expander.border.line_width, {}px, radius {}px, round the whole and between the items, as upstream's bordered Accordion frames them: expander.frame_enabled is unstated",
            px_text(b.line_width),
            px_text(b.corner_radius)
        ),
    }
}

/// How the theme-drawn expander `e` pads its title rows, side by side, and
/// its body: a stated `expander.border.padding` side as it states it, an
/// unstated left or right by upstream's px_3, an unstated top or bottom by
/// none (`demo::native_expander`, through `demo::padded`); the body right
/// and below as upstream's AccordionItem pads it wherever no `gap`
/// (`layout.widget_gap`) parts it from its title: inside the frame
/// upstream's draws, where `frame_enabled` is unstated, and where the
/// layout states no gap.
fn expander_padding(e: &ResolvedExpanderTheme, gap: Option<Pixels>) -> String {
    let p = &e.border.padding;
    let side = |name: &str, value: Option<f32>, across: bool| match (value, across) {
        (Some(v), _) => format!("{name} {}px", px_text(v)),
        (None, true) => format!("{name} upstream's px_3 (EXPANDER_PADDING_X)"),
        (None, false) => format!("{name} none"),
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
        "expander.border.padding on each title row: {}, {}, {}, {}; {body}",
        side("top", p.top, false),
        side("right", p.right, true),
        side("bottom", p.bottom, false),
        side("left", p.left, true),
    )
}

/// A row of tabs `native_theme_gpui::widgets::TabBar` draws as `r`'s
/// `tab.*` states a tab (`demo::native_tab_strip`), under `variant`: the
/// page tabs, the inspector's and the Basic page's, wherever a native theme
/// is installed.
#[cfg(feature = "widgets")]
pub fn native_tab_row(r: &ResolvedTheme, variant: &'static str) -> WidgetInfo {
    let t = &r.tab;
    // What widgets::TabBar draws from: None where a length the theme states
    // is not finite, and the bar is then gpui-component's.
    let look = TabLook::of(r);
    // The line that marks the selected tab, as widgets::TabBar draws it.
    let indicator = look.and_then(|look| look.indicator);
    // The selected tab's outward corners, as widgets::TabBar draws them.
    let outward = look.and_then(|look| look.outward_radius);
    let info = WidgetInfo::new("TabBar")
        .variant(variant)
        .color(claim(
            "bar",
            "bar_background",
            stated(t.bar_background),
            "native-theme-gpui/widgets/tab_bar.rs:217",
        ))
        .color(claim(
            "unselected tab",
            "background_color",
            stated(t.background_color),
            "native-theme-gpui/widgets/tab_bar.rs:218",
        ))
        .color(claim(
            "label",
            "font",
            stated(t.font.color),
            "native-theme-gpui/widgets/tab_bar.rs:223",
        ))
        .color(claim(
            "selected tab",
            "active_background",
            stated(t.active_background),
            "native-theme-gpui/widgets/tab_bar.rs:226",
        ))
        .color(claim(
            "selected label",
            "active_text_color",
            stated(t.active_text_color),
            "native-theme-gpui/widgets/tab_bar.rs:227",
        ))
        .colors(edge_claim(
            "selected tab's outline",
            "border",
            Some(t.border.color),
            Some(t.border.line_width),
            "native-theme-gpui/widgets/tab_bar.rs:228",
        ))
        .color(claim(
            "rule under the bar",
            "line_color",
            stated(r.separator.line_color),
            "showcase",
        ));
    let info = info
        .colors(stated_claim(
            "hovered unselected tab, in place of its fill, over the bar",
            "hover_background",
            t.hover_background,
            "native-theme-gpui/widgets/tab_bar.rs:224",
        ))
        .color(claim(
            "hovered label",
            "hover_text_color",
            stated(t.hover_text_color),
            "native-theme-gpui/widgets/tab_bar.rs:225",
        ))
        // Only where the look has the line: TabBar draws none otherwise.
        .colors(stated_claim(
            "selected tab's line",
            "active_indicator_color",
            indicator.and(t.active_indicator_color),
            "native-theme-gpui/widgets/tab_bar.rs:239",
        ));
    let padding = t.border.padding;
    let side = |v: Option<f32>| {
        v.map_or(
            "upstream's 12px (tab/tab.rs, TabVariant::inner_paddings)".to_string(),
            |v| format!("{}px", px_text(v)),
        )
    };
    info.config("size", format!("each tab at least tab.min_width, {}px, by tab.min_height, {}px", px_text(t.min_width), px_text(t.min_height)))
        .config("font", format!("tab.font: {}", drawn_font(&t.font)))
        .config("padding", format!("tab.border.padding: left {}, right {}; top and bottom {} and {}", side(padding.left), side(padding.right), padding.top.map_or("unstated".to_string(), |v| format!("{}px", px_text(v))), padding.bottom.map_or("unstated".to_string(), |v| format!("{}px", px_text(v)))))
        .config("outline", format!("tab.border on the selected tab only: {}px, radius {}px on its top corners, as Breeze and Windows draw it (docs/platform-facts.md §2.11){}", px_text(t.border.line_width), px_text(t.border.corner_radius), if outward.is_some() { "; no bottom edge: it runs on down the outward corners instead" } else { "" }))
        .config("outward corners", match (look, outward) {
            (_, Some(r)) => format!(
                "tab.active_outward_corner_radius, {}px: the selected tab curves out into the bar's bottom edge at both feet, its outline following, with no bottom edge (native-theme-gpui/widgets/tab_bar.rs, TabLook::outward_radius)",
                px_text(f32::from(r))
            ),
            (Some(_), None) => "none: the theme states no outward corners, so the selected tab's feet are square".to_string(),
            (None, None) => "none: a tab length the theme states is not finite, so widgets::TabBar draws gpui-component's TabBar in its place (native-theme-gpui/widgets/tab_bar.rs, TabLook::of)".to_string(),
        })
        .config("selection mark", match (look, indicator) {
            (_, Some(line)) => format!(
                "a line tab.active_indicator_width, {}px, thick along the selected tab's {} edge, tab.active_indicator_side, in tab.active_indicator_color, over its outline (native-theme-gpui/widgets/tab_bar.rs, TabIndicator)",
                px_text(f32::from(line.width)),
                match line.side {
                    TabIndicatorSide::Top => "top",
                    TabIndicatorSide::Bottom => "bottom",
                }
            ),
            (Some(_), None) => "none: the theme states no indicator -- widgets::TabBar draws one only where tab.active_indicator_color, tab.active_indicator_width and tab.active_indicator_side are all stated -- so the selected tab is marked by its fill, label and outline alone".to_string(),
            (None, None) => "none: a tab length the theme states is not finite, so widgets::TabBar draws gpui-component's TabBar in its place (native-theme-gpui/widgets/tab_bar.rs, TabLook::of)".to_string(),
        })
        .config("gap", match (look, t.item_gap.is_some()) {
            (Some(look), true) => format!("{}px between neighbouring tabs, tab.item_gap", px_text(f32::from(look.gap))),
            (Some(_), false) => "none: tab.item_gap is unstated, and widgets::TabBar lays its tabs with none, as gpui-component lays its default variant's (tab/tab_bar.rs, TabBar::render)".to_string(),
            (None, _) => "not widgets::TabBar's: a tab length the theme states is not finite, so gpui-component's TabBar lays the tabs out".to_string(),
        })
        .not_themeable("rule under the bar", "separator.*, the line the theme states for parting content: the model states no tab-bar rule of its own")
}

/// Each side of a segment's padding as `demo::segmented` draws it: a side
/// `segmented_control.border.padding` states by its value, an unstated left
/// or right by `SEGMENT_PADDING`, upstream's own, and an unstated top or
/// bottom by none.
fn segment_padding(p: &ResolvedPadding) -> String {
    let side = |name: &str, stated: Option<f32>, across: bool| match (stated, across) {
        (Some(v), _) => format!("{name} {}px", px_text(v)),
        (None, true) => format!("{name} {}px, unstated", px_text(SEGMENT_PADDING.as_f32())),
        (None, false) => format!("{name} none, unstated"),
    };
    format!(
        "segmented_control.border.padding per side: {}; {}; {}; {}. An unstated left or right side takes {}px and an unstated top or bottom none, upstream's own for a segment at the default Size (tab/tab.rs, TabVariant::inner_paddings)",
        side("top", p.top, false),
        side("right", p.right, true),
        side("bottom", p.bottom, false),
        side("left", p.left, true),
        px_text(SEGMENT_PADDING.as_f32()),
    )
}

/// The segmented control over `labels`, the one at `selected` shown: drawn
/// by the showcase from `native`'s `segmented_control` where a native theme
/// is installed, upstream's segmented `TabBar` where none is
/// (`demo::segmented`).
pub fn segmented(
    t: &Theme,
    labels: &[&'static str],
    selected: usize,
    native: Option<&ResolvedTheme>,
) -> WidgetInfo {
    let shown = labels.get(selected).copied().unwrap_or("none");
    let info = match native {
        Some(r) => {
            let s = &r.segmented_control;
            let info = WidgetInfo::new("Segmented control")
                .variant("drawn by the showcase")
                .color(claim("track", "background_color", stated(s.background_color), "showcase"))
                .color(claim("label", "color", stated(s.font.color), "showcase"))
                .color(claim("selected segment", "active_background", stated(s.active_background), "showcase"))
                .color(claim("selected label", "active_text_color", stated(s.active_text_color), "showcase"))
                .colors(edge_claim("border", "border", Some(s.border.color), Some(s.border.line_width), "showcase"))
                // The line left of every segment but the first.
                .colors(edge_claim("separators", "border", Some(s.border.color), (labels.len() > 1).then_some(s.separator_width), "showcase"));
            let info = match stated_claim("hovered segment", "hover_background", s.hover_background, "showcase") {
                Some(hover) => info.color(hover),
                None => info.not_themeable("hover", "none: the theme states no segmented_control.hover_background"),
            };
            info.config("why drawn", "upstream's segmented TabBar fills its selected segment with the window's background through a sliding indicator and its hovered one inside render, and labels its segments from the tab tokens (tab/tab_bar.rs, TabBar::render_indicator; tab/tab.rs, TabVariant::hovered), so segmented_control.active_background, hover_background, font.color and active_text_color would have no receiver there")
                .config("size", format!("each segment at least segmented_control.segment_height, {}px, tall", px_text(s.segment_height)))
                .config("padding", segment_padding(&s.border.padding))
                .config("font", format!("segmented_control.font: {}", drawn_font(&s.font)))
                .config("border", if s.border.line_width > 0.0 {
                    format!("segmented_control.border: {}px, radius {}px", px_text(s.border.line_width), px_text(s.border.corner_radius))
                } else {
                    format!("none: segmented_control.border.line_width is 0; the track is still rounded by its corner_radius, {}px", px_text(s.border.corner_radius))
                })
                .config("separators", if s.separator_width > 0.0 {
                    format!("segmented_control.separator_width, {}px, in border.color between segments", px_text(s.separator_width))
                } else {
                    "none: segmented_control.separator_width is 0".to_string()
                })
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
                "gpui-component/tab/tab.rs:200",
            ))
            .color(claim(
                "selected label",
                "tab_active_foreground",
                t.tab_active_foreground,
                "gpui-component/tab/tab.rs:299",
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
        .not_themeable("reduced motion", "gpui's App::reduce_motion, forwarded by apply_system_theme -- the spring jumps straight to its target (gpui-base/motion.rs, spring)")
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
            "gpui-component/label.rs:211",
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
    // style (button/button.rs:716, :741, :807-812).
    let info = if selected {
        info.variant("selected")
            .color(claim(
                "bg",
                "button_active",
                t.button_active,
                "gpui-component/button/button.rs:1308",
            ))
            .color(claim(
                "text",
                "button_foreground",
                t.button_foreground,
                "gpui-component/button/button.rs:1015",
            ))
            .color(claim(
                "border",
                "input",
                t.input,
                "gpui-component/button/button.rs:1067",
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
            "gpui-component/button/button.rs:1001",
        ))
        .color(claim(
            "text",
            "button_foreground",
            t.button_foreground,
            "gpui-component/button/button.rs:1015",
        ))
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/button/button.rs:1067",
        ))
        .color(claim(
            "hover",
            "button_hover",
            t.button_hover,
            "gpui-component/button/button.rs:1145",
        ))
        .color(claim(
            "active",
            "button_active",
            t.button_active,
            "gpui-component/button/button.rs:1229",
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
/// icon)`, of the chosen icon theme, which take `geometry::icon_size_small`
/// where a native theme is installed, whose line is recorded where
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
                "gpui-component/stepper/trigger.rs:134",
            ),
            claim(
                "icon, pending",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/stepper/trigger.rs:131",
            ),
        ),
        StepperKind::Icons if shown > 0 => (
            claim(
                "icon or number, completed / current",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/stepper/trigger.rs:134",
            ),
            claim(
                "icon or number, pending",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/stepper/trigger.rs:131",
            ),
        ),
        StepperKind::Icons | StepperKind::Numbers => (
            claim(
                "number, completed / current",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/stepper/trigger.rs:134",
            ),
            claim(
                "number, pending",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/stepper/trigger.rs:131",
            ),
        ),
    };
    let info = WidgetInfo::new("Stepper")
        .variant(kind.name())
        .color(claim(
            "completed / current",
            "primary",
            t.primary,
            "gpui-component/stepper/trigger.rs:133",
        ))
        .color(checked_mark);
    let info = if pending {
        info.color(claim(
            "pending",
            "secondary",
            t.secondary,
            "gpui-component/stepper/trigger.rs:126",
        ))
        .color(pending_mark)
        .color(claim(
            "pending hover",
            "secondary_hover",
            t.secondary_hover,
            "gpui-component/stepper/trigger.rs:128",
        ))
        .color(claim(
            "pending pressed",
            "secondary_active",
            t.secondary_active,
            "gpui-component/stepper/trigger.rs:129",
        ))
        .color(claim(
            "separator",
            "border",
            t.border,
            "gpui-component/stepper/item.rs:263",
        ))
    } else {
        info
    };
    let info = if passed {
        info.color(claim(
            "passed separator",
            "primary",
            t.primary,
            "gpui-component/stepper/item.rs:264",
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
            "gpui-component/breadcrumb.rs:101",
        ))
        .color(claim(
            "last item",
            "foreground",
            t.foreground,
            "gpui-component/breadcrumb.rs:102",
        ))
        .color(claim(
            "separators",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/breadcrumb.rs:144",
        ))
        .not_themeable("own icons", super::own_icons("the ChevronRight between its items (breadcrumb.rs, Breadcrumb)"))
        .not_themeable("separator icon", "a ChevronRight built inline with no setter to replace it (breadcrumb.rs, Breadcrumb) -- unlike an Alert's icon, which Alert::icon takes")
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
            "gpui-component/form/field.rs:344",
        ))
        .color(claim(
            "required marker",
            "danger",
            t.danger,
            "gpui-component/form/field.rs:321",
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
            "gpui-component/theme/mod.rs:416",
        ))
        .color(claim(
            "thumb",
            "scrollbar_thumb",
            t.scrollbar_thumb,
            "gpui-component/theme/mod.rs:421",
        ))
        .color(claim(
            "thumb hover",
            "scrollbar_thumb_hover",
            t.scrollbar_thumb_hover,
            "gpui-component/theme/mod.rs:432",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .config(
            "show mode",
            format!("scrollbar_mode: {:?}", t.scrollbar_mode),
        );
    let info = if styled {
        info.config("track and thumb", "scrollbar.groove_width wide, the thumb scrollbar.thumb_width wide and at least scrollbar.min_thumb_length long, in scrollbar.track_color, thumb_color and thumb_hover_color -- the colours ThemeColor holds as scrollbar, scrollbar_thumb and scrollbar_thumb_hover -- written onto gpui-base by the connector after every rebuild (native-theme-gpui base_layer.rs, scrollbar_styles)")
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

/// One of the Layout page's `Sidebar` samples (spec S5), `collapsed` to its
/// icon rail or expanded, its header reading `header`.
pub fn sidebar(t: &Theme, collapsed: bool, header: &str) -> WidgetInfo {
    let info = WidgetInfo::new("Sidebar")
        .variant(if collapsed { "collapsed" } else { "expanded" })
        .color(claim(
            "bg",
            "sidebar",
            t.sidebar,
            "gpui-component/sidebar/mod.rs:422",
        ))
        .color(claim(
            "text",
            "sidebar_foreground",
            t.sidebar_foreground,
            "gpui-component/sidebar/mod.rs:423",
        ))
        .color(claim(
            "border",
            "sidebar_border",
            t.sidebar_border,
            "gpui-component/sidebar/mod.rs:424",
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
            "gpui-component/sidebar/menu.rs:318",
        ))
        .color(claim(
            "text",
            "sidebar_accent_foreground",
            t.sidebar_accent_foreground,
            "gpui-component/sidebar/menu.rs:319",
        ))
    } else {
        info.color(claim(
            "text (the Sidebar's)",
            "sidebar_foreground",
            t.sidebar_foreground,
            "gpui-component/sidebar/mod.rs:423",
        ))
        .color(claim(
            "hover bg, at 80%",
            "sidebar_accent",
            t.sidebar_accent.opacity(0.8),
            "gpui-component/sidebar/menu.rs:312",
        ))
        .color(claim(
            "hover text",
            "sidebar_accent_foreground",
            t.sidebar_accent_foreground,
            "gpui-component/sidebar/menu.rs:313",
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
