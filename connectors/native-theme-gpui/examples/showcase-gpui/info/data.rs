//! What the Data page's widgets report about themselves (spec §3.4).

use gpui_component::{Colorize as _, attachment::AttachmentStatus, theme::Theme};

use native_theme_gpui::ResolvedTheme;

use super::{
    ColorClaim, WidgetInfo,
    chrome::{GhostContent, ghost_colours, input_background},
    claim, stated,
};
use crate::demo::{BubbleKind, DataTableRow, ListRowState};
use crate::support::SampleIcon;

/// A `DescriptionList` of `items` items, `columns` wide, bordered as upstream
/// draws it by default.
pub fn description_list(t: &Theme, items: usize, columns: usize) -> WidgetInfo {
    WidgetInfo::new("DescriptionList")
        .color(claim(
            "label bg",
            "description_list_label",
            t.description_list_label,
            "gpui-component/description_list.rs:333",
        ))
        .color(claim(
            "label text",
            "description_list_label_foreground",
            t.description_list_label_foreground,
            "gpui-component/description_list.rs:317",
        ))
        // A value is a plain div of text (description_list.rs:344-349), so it
        // takes the colour the showcase sets on its window.
        .color(claim(
            "value text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/description_list.rs:288",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("layout spacing", "per Size, in literal px (description_list.rs, DescriptionList), and the label column defaults to 120px -- a width DescriptionList::label_width does take")
        .instance("items", format!("{items}, in {columns} columns"))
}

/// The striped, bordered `DataTable`: its frame and fill. Its rows report
/// themselves (`data_table_row`). `styled` where a native theme is installed,
/// so the table takes `geometry::data_table_size` and its container
/// `geometry::table`.
pub fn data_table(t: &Theme, rows: usize, columns: usize, styled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("DataTable")
        .variant("striped, bordered")
        .color(claim(
            "frame",
            "border",
            t.border,
            "gpui-component/table/data_table.rs:171",
        ))
        .color(claim(
            "bg",
            "table",
            t.table,
            "gpui-component/table/data_table.rs:167",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable(
            "geometry",
            "DataTable is not Styled (table/data_table.rs: DataTable impls \
                         Sizable and RenderOnce, not Styled); \
                         geometry::table goes to the declarative Table below",
        );
    let info = if styled {
        info.config("row height", "list.row_height through geometry::data_table_size as Size::Size, which table_row_height returns verbatim; upstream's Medium 32px where the theme states none (sizing.rs, table_row_height)")
            .config("text", "list.item_font through geometry::table on the table's container; a Size::Size cell sets no size of its own and inherits it (sizing.rs, table_cell_size), where a Medium cell is text_sm")
    } else {
        info.not_themeable("row height", "upstream's Medium 32px: with no native theme installed nothing states list.row_height (sizing.rs, table_row_height)")
            .not_themeable("text", "text_sm, which a Medium cell sets on itself (sizing.rs, table_cell_size)")
    };
    info.config("keyboard focus", "a Tab-focused DataTable takes `ring` and the focus ring (table/data_table.rs:176-179)")
        .instance("rows", format!("{rows} rows of {columns} columns from the delegate, each reporting itself; a click selects a row"))
        .instance("columns", "a click on a header selects its column, whose cells then take table_active (table/state.rs, TableState::render_col_wrap)")
}

/// The `DataTable`'s header row.
pub fn data_table_header(t: &Theme, columns: &str) -> WidgetInfo {
    WidgetInfo::new("DataTable row")
        .variant("header")
        .color(claim(
            "bg",
            "table_head",
            t.table_head,
            "gpui-component/table/state.rs:1819",
        ))
        .color(claim(
            "text",
            "table_head_foreground",
            t.table_head_foreground,
            "gpui-component/table/state.rs:1820",
        ))
        .instance("columns", columns.to_string())
}

/// A row of the `DataTable` as the table paints it, reading `cells` where it
/// is one of the delegate's rows.
pub fn data_table_row(t: &Theme, row: DataTableRow, cells: Option<String>) -> WidgetInfo {
    let body = match row {
        DataTableRow::Filler { striped } => {
            let info = WidgetInfo::new("DataTable row").variant(if striped {
                "empty, striped"
            } else {
                "empty"
            });
            let info = if striped {
                info.color(claim(
                    "stripe",
                    "table_even",
                    t.table_even,
                    "gpui-component/table/state.rs:2290",
                ))
            } else {
                info.color(claim(
                    "bg, the table's",
                    "table",
                    t.table,
                    "gpui-component/table/data_table.rs:167",
                ))
            };
            return info
                .color(claim(
                    "bottom border",
                    "table_row_border",
                    t.table_row_border,
                    "gpui-component/table/state.rs:2289",
                ))
                .not_themeable("purpose", "a row a striped table draws below its data to fill its height: no cells, and no hover (table/state.rs, TableState::render_table_row)");
        }
        DataTableRow::Body(body) => body,
    };
    let shown = body.selected && body.selection_shown;
    let mut states = Vec::new();
    if body.striped {
        states.push("striped");
    }
    if shown {
        states.push("selected");
    } else if body.selected {
        states.push("selected, not shown");
    }
    if body.right_clicked {
        states.push("right-clicked");
    }
    let info = WidgetInfo::new("DataTable row");
    let info = if states.is_empty() {
        info
    } else {
        info.variant(states.join(", "))
    };
    // The selected fill is painted after the stripe, so it covers it. It is
    // table_active because gpui-base's ListSettings::active_highlight defaults
    // to true (list_settings.rs:14) and the connector never sets it.
    let info = if shown {
        info.color(claim(
            "selected bg",
            "table_active",
            t.table_active,
            "gpui-component/table/state.rs:2245",
        ))
    } else if body.striped {
        info.color(claim(
            "stripe",
            "table_even",
            t.table_even,
            "gpui-component/table/state.rs:2032",
        ))
    } else {
        info.color(claim(
            "bg, the table's",
            "table",
            t.table,
            "gpui-component/table/data_table.rs:167",
        ))
    };
    // A cell is the delegate's plain text, and neither the table nor its
    // rows set a body text colour (table/state.rs, TableState::
    // render_table_row), so the text is the colour the showcase sets on its
    // window.
    let info = info.color(claim(
        "text, inherited",
        "foreground",
        t.foreground,
        "showcase",
    ));
    let info = if body.selected || body.right_clicked {
        info.not_themeable("hover", "none: the table gives its selected row and its right-clicked row no hover (table/state.rs, TableState::render_table_row)")
    } else {
        info.color(claim(
            "hover",
            "table_hover",
            t.table_hover,
            "gpui-component/table/state.rs:2038",
        ))
    };
    let info = if body.right_clicked {
        info.color(claim(
            "right-click frame",
            "selection",
            t.selection,
            "gpui-component/table/state.rs:2261",
        ))
        .not_themeable("bottom border", "transparent while the row is right-clicked, under the frame drawn around it (table/state.rs, TableState::render_table_row)")
    } else {
        let info = info.color(claim(
            "bottom border",
            "table_row_border",
            t.table_row_border,
            "gpui-component/table/state.rs:2030",
        ));
        if body.last && !body.selected {
            info.not_themeable("last border", "the last row's bottom border is drawn only while the rows leave room below them or the row is selected (table/state.rs, TableState::render_table_row)")
        } else {
            info
        }
    };
    let info = if body.selected && !body.selection_shown {
        info.instance("selection", "the table's selected row, painted as any other while a column is selected (table/state.rs, TableState::render_table_row)")
    } else {
        info
    };
    match cells {
        Some(cells) => info.instance("cells", cells),
        None => info,
    }
}

/// The declarative `Table`, refined by `geometry::table`, holding `rows` body
/// rows. Its geometry line is recorded where `demo::table` applies the
/// builder.
pub fn table(t: &Theme, rows: usize) -> WidgetInfo {
    WidgetInfo::new("Table")
        .color(claim(
            "bg",
            "table",
            t.table,
            "gpui-component/table/table.rs:113",
        ))
        .not_themeable("cell padding", "inner (Tier U)")
        .instance("rows", format!("a header and {rows} body rows, written out rather than driven by a delegate; each reports itself"))
}

/// The declarative `Table`'s `TableHeader`, reading `columns`.
pub fn table_header(t: &Theme, columns: &str) -> WidgetInfo {
    WidgetInfo::new("TableHeader")
        .color(claim(
            "header bg",
            "table_head",
            t.table_head,
            "gpui-component/table/table.rs:199",
        ))
        .color(claim(
            "header text",
            "table_head_foreground",
            t.table_head_foreground,
            "gpui-component/table/table.rs:200",
        ))
        .color(claim(
            "row border",
            "table_row_border",
            t.table_row_border,
            "gpui-component/table/table.rs:203",
        ))
        .instance("columns", columns.to_string())
}

/// A body `TableRow` of the declarative `Table`, reading `cells`; `first` is
/// whether it is the body's first row.
pub fn table_row(t: &Theme, first: bool, cells: &str) -> WidgetInfo {
    let info = WidgetInfo::new("TableRow")
        .color(claim(
            "bg, the table's",
            "table",
            t.table,
            "gpui-component/table/table.rs:113",
        ))
        // Neither the Table nor the row sets a text colour, and
        // geometry::table carries no colour (native-theme-gpui geometry.rs,
        // table), so the text is the colour the showcase sets on its window.
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ));
    let info = if first {
        info.not_themeable("top border", "none: a TableRow draws a top border from its parent's second row on (table/table.rs, TableRow), and the header's bottom border is above this one")
    } else {
        info.color(claim(
            "top border",
            "table_row_border",
            t.table_row_border,
            "gpui-component/table/table.rs:415",
        ))
    };
    info.instance("cells", cells.to_string())
}

/// The fill of the page Button showing the current page: an outlined
/// Default Button's `input_background()` (button/button.rs:902), the shared
/// claim with a role that says whose fill it is.
fn current_page_fill(t: &Theme) -> ColorClaim {
    ColorClaim {
        role: if t.is_dark() {
            "current page bg, 30% input mixed with 70% transparent"
        } else {
            "current page bg"
        },
        ..input_background(t)
    }
}

/// A `Pagination` on `page` of `pages`, `compact` or not. `gap` is whether
/// the showcase refined its gap with `geometry::widget_gap`, whose line is
/// recorded where `demo::pagination` applies it.
pub fn pagination(t: &Theme, compact: bool, page: usize, pages: usize, gap: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Pagination");
    let info = if compact {
        info.variant("compact")
    } else {
        info
    };
    let info = info.colors(ghost_colours(t, GhostContent::Text));
    let info = if compact {
        info.not_themeable("buttons", "only the previous and next buttons, as icons: a compact Pagination lists no pages (pagination.rs, Pagination::render_nav_button). Both are ghost Buttons the widget builds, and no refinement reaches them")
    } else {
        info.color(current_page_fill(t))
            .color(claim(
                "current page text",
                "button_foreground",
                t.button_foreground,
                "gpui-component/button/button.rs:992",
            ))
            .color(claim(
                "current page edge",
                "input",
                t.input,
                "gpui-component/button/button.rs:1044",
            ))
            .color(claim(
                "current page hover, 50% input mixed with 50% transparent",
                "input",
                t.input.mix_oklab(t.transparent, 0.5),
                "gpui-component/button/button.rs:903-907",
            ))
            .color(claim(
                "current page pressed, 70% input mixed with 30% transparent",
                "input",
                t.input.mix_oklab(t.transparent, 0.7),
                "gpui-component/button/button.rs:908-912",
            ))
            .not_themeable("buttons", "built by the widget as ghost/outline Button (pagination.rs, Pagination::render page items); no refinement reaches them")
            .not_themeable("other pages", "no fill until hovered: a ghost Button is transparent, and it then hovers with accent -- the menu highlight, halved in dark mode (button/button.rs, ButtonVariant::hovered Ghost arm)")
            .not_themeable("current page", "an outlined Default Button, so it fills with input_background() and edges with input, not with the button family (button/button.rs, ButtonVariant::outline_background)")
            .not_themeable("ellipsis", "a ghost Button whose dropdown lists the hidden pages (pagination.rs, PageItem::Ellipsis)")
    };
    let edge = match (page <= 1, page >= pages) {
        (true, true) => Some("both the previous and the next button"),
        (true, false) => Some("the previous button"),
        (false, true) => Some("the next button"),
        (false, false) => None,
    };
    let info = match edge {
        Some(which) => info
            .color(claim(
                "disabled text, at 50%",
                "muted_foreground",
                t.muted_foreground.opacity(0.5),
                "gpui-component/button/button.rs:1327",
            ))
            .instance("disabled", format!("{which}: there is no page beyond it (pagination.rs, Pagination::render_nav_button)")),
        None => info,
    };
    let info = if gap {
        info.not_themeable("gap", "upstream's own is gap_1, which the refinement replaces (pagination.rs, Pagination::render)")
    } else {
        info.not_themeable("gap", "gap_1, upstream's own -- a rem, so the platform's font (pagination.rs, Pagination::render)")
    };
    info.not_themeable("own icons", super::own_icons("ChevronLeft and ChevronRight on its previous and next buttons (pagination.rs, render_nav_button), and Ellipsis where it leaves pages out (pagination.rs, Pagination::render)"))
        .instance("page", format!("{page} of {pages}"))
        .instance("click", "a page or an arrow moves to that page; the showcase keeps it, and both Paginations follow it")
}

/// The fill every List and Tree row lacks.
const NO_LIST_FILL: &str = "none: a ListItem paints no idle background (list/list_item.rs, ListItem::render), so the fill of the box around the rows shows through";

/// The fill of the box around a List's or a Tree's rows: `list.background_color`,
/// which the showcase paints on it, where a native theme is installed.
fn list_fill(info: WidgetInfo, native: Option<&ResolvedTheme>) -> WidgetInfo {
    match native {
        Some(r) => info.color(claim(
            "fill",
            "background_color",
            stated(r.list.background_color),
            "showcase",
        )),
        None => info.not_themeable("fill", "none: no native theme is installed, so the showcase has no list.background_color for the box around the rows, and ThemeColor::list has no reader (list/list_item.rs, ListItem::render)"),
    }
}

/// The List, with the box around it that is its frame and its fill. Its
/// geometry line is recorded where `demo::list` applies the builder; its
/// rows report themselves (`list_row`).
pub fn list(rows: usize, native: Option<&ResolvedTheme>) -> WidgetInfo {
    list_fill(WidgetInfo::new("List").variant("selectable"), native)
        .not_themeable("even rows", "the list_even token has no reader anywhere in gpui-component or gpui-base: a List paints every row the same, and the connector writes the slot for nothing (Tier U)")
        .instance("frame", "List paints no frame of its own (list/list.rs, RenderOnce for List), so the frame is the application's: the box around it")
        .instance("rows", format!("{rows}; a click selects one, and each reports itself"))
}

/// What a `ListItem` row reading `label` paints in `state`, for the List and
/// the Tree. `styled` is whether a native theme is installed, so whether the
/// row took `geometry::list_item`, whose line is recorded where the row
/// applies it.
fn list_item(t: &Theme, label: &str, state: ListRowState, styled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("ListItem").variant(match state {
        ListRowState::Idle => label.to_string(),
        ListRowState::Selected => format!("{label}, selected"),
        ListRowState::RightClicked => format!("{label}, right-clicked"),
        ListRowState::SelectedRightClicked => format!("{label}, selected and right-clicked"),
    });
    // list_active because gpui-base's ListSettings::active_highlight
    // defaults to true (list_settings.rs:14) and the connector never sets it.
    let info = match state {
        ListRowState::Idle => info.color(claim(
            "hover",
            "list_hover",
            t.list_hover,
            "gpui-component/list/list_item.rs:231",
        )),
        ListRowState::Selected => info
            .color(claim(
                "selected bg",
                "list_active",
                t.list_active,
                "gpui-component/list/list_item.rs:259",
            ))
            .not_themeable(
                "hover",
                "none while selected (list/list_item.rs, ListItem::render)",
            ),
        ListRowState::RightClicked => info
            .color(claim(
                "outline, 1px",
                "selection",
                t.selection,
                "gpui-component/list/list_item.rs:272",
            ))
            .not_themeable(
                "fill",
                "none, and no hover while right-clicked (list/list_item.rs, ListItem::render)",
            ),
        ListRowState::SelectedRightClicked => info
            .color(claim(
                "selected bg",
                "list_active",
                t.list_active,
                "gpui-component/list/list_item.rs:259",
            ))
            .color(claim(
                "outline, 1px, over the fill",
                "selection",
                t.selection,
                "gpui-component/list/list_item.rs:272",
            ))
            .not_themeable(
                "hover",
                "none while selected (list/list_item.rs, ListItem::render)",
            ),
    };
    let info = info.not_themeable("fill at rest", NO_LIST_FILL);
    // The label is plain text, so it takes the row's own text style: the
    // foreground and text_base ListItem sets (list/list_item.rs:204-205),
    // which geometry::list_item then refines with list.item_font, colour
    // included -- a colour no ThemeColor field holds.
    if styled {
        info.instance("text", "list.item_font's size and colour, which geometry::list_item refines the row with over the text_base and foreground ListItem sets first (list/list_item.rs, ListItem::render)")
    } else {
        info.color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/list/list_item.rs:205",
        ))
        .not_themeable("text size", "text_base, ListItem's own: no native theme is installed, so geometry::list_item has no font to give the row (list/list_item.rs, ListItem::render)")
    }
}

/// A row of the List reading `label`, in `state`.
pub fn list_row(t: &Theme, label: &str, state: ListRowState, styled: bool) -> WidgetInfo {
    list_item(t, label, state, styled).instance("click", "selects the row")
}

/// The Tree, with the box around it that is its frame. Its geometry line is
/// recorded where `demo::tree` applies the builder; its rows report
/// themselves (`tree_row`).
pub fn tree(native: Option<&ResolvedTheme>) -> WidgetInfo {
    list_fill(WidgetInfo::new("Tree"), native)
        .not_themeable("indent", "none here, and none upstream: Tree::new takes a render_item closure and tree.rs draws no row content of its own (tree.rs, Tree), so a row's indent is whatever the application's closure applies -- this demo's applies none")
        .not_themeable("disclosure icon", "the same: tree.rs contains no icon, so a chevron would be the closure's to draw. The panel used to claim a hardcoded ChevronRight, which is in neither this demo nor upstream")
        .instance("frame", "a tree is a list view and the model gives it no theme of its own, so its frame is the list's: the box around it")
        .instance("rows", "a file structure with src open; each row reports itself")
}

/// A row of the Tree reading `label`, `selected` or not.
pub fn tree_row(t: &Theme, label: &str, selected: bool, styled: bool) -> WidgetInfo {
    let state = if selected {
        ListRowState::Selected
    } else {
        ListRowState::Idle
    };
    list_item(t, label, state, styled)
        .not_themeable("right-click", "not followed here: Tree tells the row's builder whether the row is selected and nothing else (tree.rs, Tree::new), and marks a right-clicked row after it is built (tree.rs, RenderOnce for Tree). A right-clicked row paints no hover and gains a 1px selection outline, over the selected fill when it is selected (list/list_item.rs, ListItem::render)")
        .instance("click", "selects the row, and opens or closes a folder (gpui-base/tree.rs, TreeState::on_entry_click)")
}

/// The notes every Avatar of the page shares: each has a name and no image.
fn avatar_notes(info: WidgetInfo) -> WidgetInfo {
    info.not_themeable("colours", "none from the theme: a named Avatar takes its fill, initials and edge from three OKLCH literals picked by a hash of its initials -- twelve hues, a light and a dark set -- and only the mode is the theme's (avatar/avatar.rs, IdentityColor::from_hue). secondary, background and border paint an avatar with no name and no image, which this demo does not show")
        .not_themeable("corner radius", "a circle, through radius_full() -- and square where the theme's radius is 0, since radius_full() follows it (theme/mod.rs, Theme::radius_full)")
        .not_themeable("size", "configurable via Size enum")
}

/// An `Avatar` named `name`.
pub fn avatar(name: &str) -> WidgetInfo {
    avatar_notes(WidgetInfo::new("Avatar").variant(name.to_string()))
        .instance("name", name.to_string())
}

/// An `AvatarGroup` of Avatars named `names`, showing `limit` of them, which
/// report through the group.
pub fn avatar_group(names: &[&str], limit: usize) -> WidgetInfo {
    avatar_notes(WidgetInfo::new("AvatarGroup"))
        .not_themeable("limit", match names.len().checked_sub(limit) {
            Some(dropped) if dropped > 0 => format!("{limit} shown and {dropped} dropped with no marker: the overflow marker is an Avatar named ⋯, not a +N count, and only AvatarGroup::ellipsis adds it, which this demo does not call (avatar/avatar_group.rs, AvatarGroup::ellipsis)"),
            _ => format!("all {} shown: the limit, {limit}, drops none (avatar/avatar_group.rs, AvatarGroup::limit)", names.len()),
        })
        .instance("avatars", format!("{}, limited to {limit}. AvatarGroup::child takes an Avatar, not an element a target could wrap, so they report through the group (avatar/avatar_group.rs, AvatarGroup::child)", names.join(", ")))
}

/// The notes every Bubble but a Ghost one shares.
fn bubble_surface(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.config(
        "border-radius",
        format!("radius_2xl(): {}px", t.radius_2xl().as_f32()),
    )
    .not_themeable("surface padding", "px_3 / py_2 -- rems, so the platform's font -- and settable: Bubble::content takes a BubbleContent, which applies the caller's refinement last (bubble.rs, BubbleContent). native-theme states no bubble")
}

/// A `Bubble` of `kind`, `outgoing` or incoming.
pub fn bubble(t: &Theme, kind: BubbleKind, outgoing: bool) -> WidgetInfo {
    let side = if outgoing { "outgoing" } else { "incoming" };
    let info = WidgetInfo::new("Bubble").variant(format!("{}, {side}", kind.name()));
    let info = match kind {
        BubbleKind::Filled => bubble_surface(info, t)
            .color(claim(
                "bg",
                "primary",
                t.primary,
                "gpui-component/bubble.rs:215",
            ))
            .color(claim(
                "text",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/bubble.rs:216",
            )),
        BubbleKind::Secondary => bubble_surface(info, t)
            .color(claim(
                "bg",
                "muted",
                t.muted,
                "gpui-component/bubble.rs:222",
            ))
            .color(claim(
                "text",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/bubble.rs:223",
            )),
        BubbleKind::Muted => bubble_surface(info, t)
            .color(claim(
                "bg",
                "muted",
                t.muted,
                "gpui-component/bubble.rs:225",
            ))
            .color(claim(
                "text",
                "foreground",
                t.foreground,
                "gpui-component/bubble.rs:226",
            )),
        // `mix_oklab`'s factor is the first colour's share (theme/color.rs:
        // 44-49), and the arm picks it by the mode (bubble.rs:230).
        BubbleKind::Tinted => {
            let fill = if t.is_dark() {
                claim(
                    "bg, 24% primary mixed with 76% background",
                    "primary",
                    t.primary.mix_oklab(t.background, 0.24),
                    "gpui-component/bubble.rs:228-231",
                )
            } else {
                claim(
                    "bg, 12% primary mixed with 88% background",
                    "primary",
                    t.primary.mix_oklab(t.background, 0.12),
                    "gpui-component/bubble.rs:228-231",
                )
            };
            bubble_surface(info, t)
                .color(fill)
                .color(claim(
                    "text",
                    "foreground",
                    t.foreground,
                    "gpui-component/bubble.rs:232",
                ))
                .not_themeable("tint", "a mix of primary into background by a literal share, 12% in light mode and 24% in dark (bubble.rs, BubbleContent)")
        }
        BubbleKind::Outline => bubble_surface(info, t)
            .color(claim(
                "border",
                "border",
                t.border,
                "gpui-component/bubble.rs:234",
            ))
            .color(claim(
                "bg",
                "background",
                t.background,
                "gpui-component/bubble.rs:235",
            ))
            .color(claim(
                "text",
                "foreground",
                t.foreground,
                "gpui-component/bubble.rs:232",
            )),
        BubbleKind::Ghost => info
            .color(claim(
                "bg",
                "transparent",
                t.transparent,
                "gpui-component/bubble.rs:240",
            ))
            .color(claim(
                "text",
                "foreground",
                t.foreground,
                "gpui-component/bubble.rs:241",
            ))
            .not_themeable("surface", "none: the Ghost arm drops the border, the corner radius and the padding -- border_0, the radius tokens' none, p_0 (bubble.rs, BubbleContent)"),
        // The semantic tokens name danger destructive (theme/mod.rs:534); the
        // Destructive arm reads it at a share picked by the mode
        // (bubble.rs:244-249).
        BubbleKind::Destructive => {
            let fill = if t.is_dark() {
                claim(
                    "bg, destructive at 20% (bubble.rs:244)",
                    "danger",
                    t.danger.opacity(0.2),
                    "gpui-component/theme/mod.rs:534",
                )
            } else {
                claim(
                    "bg, destructive at 10% (bubble.rs:244)",
                    "danger",
                    t.danger.opacity(0.1),
                    "gpui-component/theme/mod.rs:534",
                )
            };
            bubble_surface(info, t)
                .color(fill)
                .color(claim(
                    "text, destructive (bubble.rs:249)",
                    "danger",
                    t.danger,
                    "gpui-component/theme/mod.rs:534",
                ))
        }
    };
    info.not_themeable("token path", "a Bubble reads cx.theme().semantic_tokens().colors, not the ThemeColor fields directly (bubble.rs, the content surface). The values are the same; the access path is a third one, beside cx.theme().field and the Theme impl's own self.tokens.field")
        .not_themeable("max width", "80% of the row unless the Bubble's own refinement, applied last, sets another (bubble.rs, Bubble::render)")
}

/// A `Message` row, `outgoing` or incoming, from `sender`, reading `text`.
/// Its Avatar reports itself; its Bubble reports through it.
pub fn message(t: &Theme, outgoing: bool, sender: &str, text: &str) -> WidgetInfo {
    let info = WidgetInfo::new("Message").variant(if outgoing { "outgoing" } else { "incoming" });
    let info = if outgoing {
        info.color(claim(
            "bubble bg",
            "primary",
            t.primary,
            "gpui-component/bubble.rs:215",
        ))
        .color(claim(
            "bubble text",
            "primary_foreground",
            t.primary_foreground,
            "gpui-component/bubble.rs:216",
        ))
        .instance("bubble", "Filled, at the end of the row. MessageContent::bubble takes the Bubble itself, so it reports through the Message (message.rs, MessageContent::bubble)")
    } else {
        info.color(claim(
            "bubble bg",
            "muted",
            t.muted,
            "gpui-component/bubble.rs:225",
        ))
        .color(claim(
            "bubble text",
            "foreground",
            t.foreground,
            "gpui-component/bubble.rs:226",
        ))
        .instance("bubble", "Muted, at the start of the row. MessageContent::bubble takes the Bubble itself, so it reports through the Message (message.rs, MessageContent::bubble)")
    };
    info.not_themeable("bubbles", "a Message delegates its bubble to Bubble, so the fill is Bubble's and the citations point there")
        .not_themeable("slot gap", "rems(0.625) -- the platform's font, not a literal -- and settable: Message applies the caller's refinement last, and the stack takes Message::with_stack_style. Only the 0.5rem between avatar and content is out of reach (message.rs, Message::render)")
        .not_themeable("avatar", "a shared size-8 minimum, kept flush with the bubble's bottom edge (message.rs, the avatar slot's RenderOnce: min_w_8, self_end), coloured like the Avatar panel's: OKLCH literals hashed from the sender's initials, not theme colours. The Avatar reports itself")
        .instance("sender", sender.to_string())
        .instance("text", text.to_string())
}

/// The `MessageScroller` over a thread of `messages`, in the showcase's
/// frame.
pub fn message_scroller(t: &Theme, messages: usize) -> WidgetInfo {
    WidgetInfo::new("MessageScroller")
        .color(claim(
            "bottom fade",
            "background",
            t.background,
            "showcase",
        ))
        .color(claim(
            "scrollbar",
            "scrollbar_thumb",
            t.scrollbar_thumb,
            "gpui-component/theme/mod.rs:421",
        ))
        .color(claim(
            "jump button",
            "background",
            t.background,
            "gpui-component/message_scroller.rs:433",
        ))
        .color(claim(
            "jump button edge",
            "border",
            t.border,
            "gpui-component/message_scroller.rs:432",
        ))
        .color(claim(
            "jump button icon",
            "foreground",
            t.foreground,
            "gpui-component/message_scroller.rs:434",
        ))
        .color(claim("frame", "border", t.border, "showcase"))
        .not_themeable("jump button", "appears once the user scrolls away from the tail, fading over a 200ms module const rather than the theme's motion tokens (message_scroller.rs, JUMP_BUTTON_TRANSITION)")
        .not_themeable("own icons", super::own_icons("ArrowDown on the jump button it shows once the thread is scrolled away from its end (message_scroller.rs, MessageScroller)"))
        .instance("rows", format!("{messages} Message rows, rendered on demand; each reports itself"))
        .instance("follow", "FollowMode::Tail: Send scrolls the thread to the new row (message_scroller.rs, MessageScrollerState::new)")
        .instance("frame", "the showcase's own frame around the scroller, not the widget's")
}

/// An `Attachment` card for `file` in `status`, its media `icon` of the
/// chosen icon theme; `clickable` is whether a click steps it to the next
/// status.
pub fn attachment(
    t: &Theme,
    status: AttachmentStatus,
    file: &str,
    clickable: bool,
    icon: &SampleIcon,
) -> WidgetInfo {
    let info = WidgetInfo::new("Attachment")
        .variant(format!("{status:?}"))
        .color(claim(
            "bg",
            "background",
            t.background,
            "gpui-component/attachment.rs:431",
        ))
        .color(claim(
            "title",
            "foreground",
            t.foreground,
            "gpui-component/attachment.rs:432",
        ));
    // The semantic tokens name danger destructive (theme/mod.rs:534); a
    // failed card reads it at full strength and at 10%.
    let info = if status == AttachmentStatus::Failed {
        info.color(claim(
            "border, destructive (attachment.rs:425-426)",
            "danger",
            t.danger,
            "gpui-component/theme/mod.rs:534",
        ))
        .color(claim(
            "media bg, destructive at 10% (attachment.rs:840-841)",
            "danger",
            t.danger.opacity(0.1),
            "gpui-component/theme/mod.rs:534",
        ))
        .color(claim(
            "status glyph, upstream's Ban, destructive (attachment.rs:800, 845-846)",
            "danger",
            t.danger,
            "gpui-component/theme/mod.rs:534",
        ))
        .color(claim(
            "description, destructive (attachment.rs:1141-1142)",
            "danger",
            t.danger,
            "gpui-component/theme/mod.rs:534",
        ))
        .not_themeable("failed tint", "the semantic layer renames danger to destructive, and a failed card draws its border, status glyph and description in it (theme/mod.rs, color_tokens; attachment.rs, Attachment::render)")
    } else {
        info.color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/attachment.rs:428",
        ))
        .color(claim(
            "media bg",
            "muted",
            t.muted,
            "gpui-component/attachment.rs:843",
        ))
        .colors(
            if matches!(
                status,
                AttachmentStatus::Uploading | AttachmentStatus::Processing
            ) {
                Some(claim(
                    "status spinner, in place of the media icon",
                    "primary",
                    t.primary,
                    "gpui-component/attachment.rs:796",
                ))
            } else {
                icon.shown().then(|| {
                    claim(
                        "media icon",
                        "foreground",
                        t.foreground,
                        "gpui-component/attachment.rs:848",
                    )
                })
            },
        )
        .color(claim(
            "description",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/attachment.rs:1143",
        ))
    };
    let info = match status {
        AttachmentStatus::Pending => info.not_themeable("pending", "a dashed border (attachment.rs, Attachment::render: border_dashed)"),
        AttachmentStatus::Uploading | AttachmentStatus::Processing => info
            .not_themeable("in-progress title", "the ShimmerText highlight, driven by the status (attachment.rs, AttachmentTitle::render)")
            .not_themeable("own icons", super::own_icons("the status Spinner's Loader (attachment.rs, AttachmentMedia::render; spinner.rs, Spinner)")),
        AttachmentStatus::Failed => info.not_themeable("own icons", super::own_icons("the failed status glyph, Ban (attachment.rs, AttachmentMedia::render)")),
        AttachmentStatus::Complete => info,
    };
    let info = if clickable {
        info.color(claim(
            "hover bg, muted at 50%",
            "muted",
            t.muted.opacity(0.5),
            "gpui-component/attachment.rs:437",
        ))
        .instance("click", "steps it to the next status: Pending, Uploading, Processing, Complete, Failed, then Pending again")
    } else {
        info
    };
    let media = if matches!(
        status,
        AttachmentStatus::Pending | AttachmentStatus::Complete
    ) {
        icon.note("the media frame is empty")
    } else {
        "replaced by the status glyph while the card has no image (attachment.rs, AttachmentMedia::render)".to_string()
    };
    info.config(
        "border-radius",
        format!("radius_lg: {}px", t.radius_lg.as_f32()),
    )
    .not_themeable(
        "size",
        "a fixed chip: 3.5 rem tall and 14.5 rem wide at Medium (attachment.rs, CardMetrics)",
    )
    .instance("file", file.to_string())
    .instance("media icon", media)
}
