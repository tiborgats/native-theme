//! What the Data page's widgets report about themselves (spec §3.4).

use gpui_component::{Colorize as _, attachment::AttachmentStatus, theme::Theme};

use native_theme_gpui::ResolvedTheme;

use super::{
    ColorClaim, WidgetInfo,
    chrome::{GhostContent, ghost_colours, input_background},
    claim, drawn_font, edge_claim, px_text, stated,
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
/// `geometry::table`; `header` where the theme states `list.header_height`,
/// which the header row takes through `geometry::list_header_height`.
pub fn data_table(
    t: &Theme,
    rows: usize,
    columns: usize,
    styled: bool,
    header: bool,
) -> WidgetInfo {
    let info = WidgetInfo::new("DataTable")
        .variant("striped, bordered")
        .color(claim(
            "frame",
            "border",
            t.border,
            "gpui-component/table/data_table.rs:225",
        ))
        .color(claim(
            "bg",
            "table",
            t.table,
            "gpui-component/table/data_table.rs:221",
        ))
        .color(claim(
            "sort button icon",
            "secondary_foreground",
            t.secondary_foreground,
            "gpui-component/table/state.rs:1841",
        ))
        .color(claim(
            "sort button hover",
            "secondary",
            t.secondary,
            "gpui-component/table/state.rs:1833",
        ))
        .color(claim(
            "sort button pressed",
            "secondary_active",
            t.secondary_active,
            "gpui-component/table/state.rs:1834",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .config(
            "sort button radius",
            format!("radius / 2: {}px", t.radius.as_f32() / 2.0),
        )
        .not_themeable("sort button", "dimmed to 50% opacity while its column is unsorted, and given opacity literals of 7 hovered and 1 pressed (table/state.rs, render_sort_icon)")
        .not_themeable("own icons", super::own_icons("the sort button's ChevronsUpDown, SortAscending and SortDescending (table/state.rs, render_sort_icon)"))
        .not_themeable(
            "geometry",
            "DataTable is not Styled (table/data_table.rs: DataTable impls \
                         Sizable and RenderOnce, not Styled); \
                         a theme reaches it only through its Size, its header row \
                         height (table/data_table.rs, DataTable::header_row_height) \
                         and the text its container gives the cells",
        );
    let info = if styled && header {
        info.geometry("data_table_size")
            .geometry("table")
            .geometry("list_header_height")
    } else if styled {
        info.geometry("data_table_size").geometry("table").not_themeable("header height", "the body rows' height: the theme states no list.header_height, and a header row height left unset matches the body rows (table/state.rs, header_row_height)")
    } else {
        info.not_themeable("row height", "upstream's Medium 32px: with no native theme installed nothing states list.row_height (sizing.rs, table_row_height)")
            .not_themeable("text", "text_sm, which a Medium cell sets on itself (sizing.rs, table_cell_size)")
    };
    info.config("keyboard focus", "a focused DataTable takes `ring` while the last input was a key press, so a click alone shows none of it, an arrow key after the click does, and a mouse move takes it away: its edge in `ring`, with the focus ring outside it where the theme's focus_ring is on (gpui-component/styled.rs:250-271, focus_style); DataTable::new turns it on (table/data_table.rs:114, 228-235)")
        .instance("rows", format!("{rows} rows of {columns} columns from the delegate, each reporting itself; a click selects a row"))
        .instance("columns", "a click on a header selects its column, whose cells then take table_active (table/state.rs, TableState::render_col_wrap)")
        .instance("sortable", "the Role column only: its button cycles descending, ascending and unsorted (table/state.rs, TableState::perform_sort), and the showcase sorts its rows to match")
        .instance("right-clicked cell", "none: the outline, table_active_border at 50% (table/state.rs, render_table_row: lines 2311 and 2421), is drawn only round a right-clicked cell of a cell_selectable table, and this one keeps cell_selectable off (table/state.rs, TableState::new: line 329) because selecting cells instead of rows would change the row panels")
}

/// The `DataTable`'s header row.
pub fn data_table_header(t: &Theme, columns: &str) -> WidgetInfo {
    WidgetInfo::new("DataTable row")
        .variant("header")
        .color(claim(
            "bg",
            "table_head",
            t.table_head,
            "gpui-component/table/state.rs:2039",
        ))
        .color(claim(
            "text",
            "table_head_foreground",
            t.table_head_foreground,
            "gpui-component/table/state.rs:2040",
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
                    "gpui-component/table/state.rs:2510",
                ))
            } else {
                info.color(claim(
                    "bg, the table's",
                    "table",
                    t.table,
                    "gpui-component/table/data_table.rs:221",
                ))
            };
            return info
                .color(claim(
                    "bottom border",
                    "table_row_border",
                    t.table_row_border,
                    "gpui-component/table/state.rs:2509",
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
            "gpui-component/table/state.rs:2465",
        ))
    } else if body.striped {
        info.color(claim(
            "stripe",
            "table_even",
            t.table_even,
            "gpui-component/table/state.rs:2243",
        ))
    } else {
        info.color(claim(
            "bg, the table's",
            "table",
            t.table,
            "gpui-component/table/data_table.rs:221",
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
            "gpui-component/table/state.rs:2249",
        ))
    };
    let info = if body.right_clicked {
        info.color(claim(
            "right-click frame",
            "selection",
            t.selection,
            "gpui-component/table/state.rs:2481",
        ))
        .not_themeable("bottom border", "transparent while the row is right-clicked, under the frame drawn around it (table/state.rs, TableState::render_table_row)")
    } else {
        let info = info.color(claim(
            "bottom border",
            "table_row_border",
            t.table_row_border,
            "gpui-component/table/state.rs:2241",
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

/// The Basic page's table frame the showcase draws as `r`'s `list.*`
/// states a list (`demo::files_table`), holding a header and `rows` body
/// rows.
pub fn native_table_frame(r: &ResolvedTheme, rows: usize) -> WidgetInfo {
    let l = &r.list;
    let info = WidgetInfo::new("Table")
        .variant("drawn by the showcase")
        .color(claim(
            "bg",
            "background_color",
            stated(l.background_color),
            "showcase",
        ))
        .colors(edge_claim(
            "edge",
            "border",
            Some(l.border.color),
            Some(l.border.line_width),
            "showcase",
        ));
    let info = if l.border.line_width > 0.0 {
        info.config(
            "edge",
            format!(
                "list.border at {}px, round the header and the rows",
                px_text(l.border.line_width)
            ),
        )
    } else {
        info.config("edge", "none: list.border.line_width is 0")
    };
    // The frame clips the rows to its corners whether or not it draws an
    // edge, at the radius demo::files_table rounds it with.
    let info = info.config(
        "corners",
        format!(
            "list.border.corner_radius, {}px, which the header and the rows are clipped to",
            px_text(l.border.corner_radius.max(0.0))
        ),
    );
    info.config("why drawn", "upstream's Table fills itself with the table token and rules every row after the first with table_row_border (table/table.rs, Table and TableRow), so list.selection_background, selection_text_color, the list.hover_* pair and grid_color would have no receiver there")
        .instance("rows", format!("a header and {rows} body rows, written out rather than driven by a delegate; each reports itself"))
}

/// The header row of the Basic page's table the showcase draws as `r`'s
/// `list.*` states a list (`demo::files_table`), reading `columns`. Its
/// geometry line is recorded where `demo::files_table` applies
/// `geometry::list_header_height`.
pub fn native_table_header(r: &ResolvedTheme, columns: &str) -> WidgetInfo {
    let l = &r.list;
    let info = WidgetInfo::new("Table header")
        .variant("drawn by the showcase")
        .color(claim(
            "bg",
            "header_background",
            stated(l.header_background),
            "showcase",
        ))
        .color(claim(
            "text",
            "header_font",
            stated(l.header_font.color),
            "showcase",
        ));
    grid_lines(info, r, "a line inside its bottom edge, under the header, and one down the first column's right edge")
        .config("font", format!("list.header_font: {}", drawn_font(&l.header_font)))
        .instance("columns", columns.to_string())
}

/// A body row of the Basic page's table the showcase draws as `r`'s
/// `list.*` states a list (`demo::files_table`), reading `cells`;
/// `selected` is whether it is the selected row. The info does not follow
/// the pointer: a row that is not selected names the fill and text it takes
/// under the pointer beside the ones it rests in.
pub fn native_table_row(r: &ResolvedTheme, selected: bool, cells: &str) -> WidgetInfo {
    let l = &r.list;
    let info = WidgetInfo::new("Table row").variant(if selected {
        "drawn by the showcase, selected"
    } else {
        "drawn by the showcase"
    });
    let info = if selected {
        info.color(claim(
            "bg",
            "selection_background",
            stated(l.selection_background),
            "showcase",
        ))
        .color(claim(
            "text",
            "selection_text_color",
            stated(l.selection_text_color),
            "showcase",
        ))
        .config(
            "hover",
            "none: the selected row keeps its fill and text under the pointer",
        )
    } else {
        info.color(claim(
            "bg, the frame's",
            "background_color",
            stated(l.background_color),
            "showcase",
        ))
        .color(claim(
            "text",
            "item_font",
            stated(l.item_font.color),
            "showcase",
        ))
        .color(claim(
            "hovered bg",
            "hover_background",
            stated(l.hover_background),
            "showcase",
        ))
        .color(claim(
            "hovered text",
            "hover_text_color",
            stated(l.hover_text_color),
            "showcase",
        ))
    };
    grid_lines(info, r, "a line down the first column's right edge")
        .config(
            "edges",
            "none between rows: the header's bottom line is the table's one rule across",
        )
        .config(
            "font",
            format!("list.item_font: {}", drawn_font(&l.item_font)),
        )
        .instance("cells", cells.to_string())
}

/// `info` given the claim and the config line of the grid lines a row of the
/// Basic page's table draws, `where_drawn` saying which:
/// `list.grid_color` at `separator.line_width`, or none at a 0 width.
fn grid_lines(info: WidgetInfo, r: &ResolvedTheme, where_drawn: &str) -> WidgetInfo {
    if r.separator.line_width > 0.0 {
        info.color(claim(
            "grid line",
            "grid_color",
            stated(r.list.grid_color),
            "showcase",
        ))
        .config(
            "grid",
            format!(
                "{where_drawn}, list.grid_color at separator.line_width, {}px, one device \
                 pixel at least",
                px_text(r.separator.line_width)
            ),
        )
    } else {
        info.config("grid", "none: separator.line_width is 0")
    }
}

/// The fill of the page Button showing the current page: an outlined
/// Default Button's `input_background()` (button/button.rs:939), the shared
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
                "gpui-component/button/button.rs:1029",
            ))
            .color(claim(
                "current page edge",
                "input",
                t.input,
                "gpui-component/button/button.rs:1081",
            ))
            .color(claim(
                "current page hover, 50% input mixed with 50% transparent",
                "input",
                t.input.mix_oklab(t.transparent, 0.5),
                "gpui-component/button/button.rs:940-944",
            ))
            .color(claim(
                "current page pressed, 70% input mixed with 30% transparent",
                "input",
                t.input.mix_oklab(t.transparent, 0.7),
                "gpui-component/button/button.rs:945-949",
            ))
            .not_themeable("buttons", "built by the widget as ghost/outline Button (gpui-component/pagination.rs, Pagination::render page items); no refinement reaches them")
            .not_themeable("other pages", "no fill until hovered: a ghost Button is transparent, and it then hovers with accent -- the menu highlight, halved in dark mode (button/button.rs, ButtonVariant::hovered Ghost arm)")
            .not_themeable("current page", "an outlined Default Button, so it fills with input_background() and edges with input, not with the button family (button/button.rs, ButtonVariant::outline_background)")
            .not_themeable("ellipsis", "a ghost Button whose dropdown lists the hidden pages (gpui-component/pagination.rs, PageItem::Ellipsis)")
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
                "gpui-component/button/button.rs:1364",
            ))
            .instance("disabled", format!("{which}: there is no page beyond it (pagination.rs, Pagination::render_nav_button)")),
        None => info,
    };
    let info = if gap {
        info.not_themeable("gap", "upstream's own is gap_1, which the refinement replaces (gpui-component/pagination.rs, Pagination::render)")
    } else {
        info.not_themeable("gap", "gap_1, upstream's own -- a rem, so the platform's font (gpui-component/pagination.rs, Pagination::render)")
    };
    info.not_themeable("own icons", super::own_icons("ChevronLeft and ChevronRight on its previous and next buttons (pagination.rs, render_nav_button), and Ellipsis where it leaves pages out (gpui-component/pagination.rs, Pagination::render)"))
        .instance("page", format!("{page} of {pages}"))
        .instance("click", "a page or an arrow moves to that page; the showcase keeps it, and both Paginations follow it")
}

/// The fill every List and Tree row lacks.
const NO_LIST_FILL: &str = "none: a ListItem paints no idle background (list/list_item.rs, ListItem::render), so the fill of the box around the rows shows through";

/// The fill of the box around a List's or a Tree's rows: `list.background_color`,
/// which `geometry::list` paints on it, where a native theme is installed.
fn list_fill(info: WidgetInfo, native: Option<&ResolvedTheme>) -> WidgetInfo {
    match native {
        Some(r) => info.color(claim(
            "fill",
            "background_color",
            stated(r.list.background_color),
            "native-theme-gpui/geometry.rs:737",
        )),
        None => info.not_themeable("fill", "none: no native theme is installed, so geometry::list, which paints list.background_color on the box around the rows, does not apply, and ThemeColor::list has no reader (list/list_item.rs, ListItem::render)"),
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

/// The room a List's or a Tree's row keeps at its right end for the
/// scrollbar its view lays over the rows: `gutter`, the groove's width
/// (`demo::list_gutter`), where a native theme's scrollbar is no overlay.
fn scrollbar_room(styled: bool, gutter: Option<gpui::Pixels>) -> String {
    match (styled, gutter) {
        (_, Some(gutter)) => format!(
            "{}px, scrollbar.groove_width: the List and the Tree lay their scrollbar over the rows' right end (gpui-component/list/list.rs, List::render; gpui-component/tree.rs, Tree), and the theme's is no overlay (scrollbar.overlay_mode false), so the row ends that groove short of it (demo::list_gutter)",
            px_text(gutter.as_f32())
        ),
        (true, None) => "none: the theme's scrollbar is an overlay (scrollbar.overlay_mode), which the List and the Tree lay over the rows' right end".to_string(),
        (false, None) => "none: no native theme is installed, and the List and the Tree lay their scrollbar over the rows' right end".to_string(),
    }
}

/// What a `ListItem` row reading `label` paints in `state`, for the List and
/// the Tree, keeping `gutter` clear for the scrollbar where the theme's is
/// no overlay. `styled` is whether a native theme is installed, so whether
/// the row took `geometry::list_item`, whose line is recorded where the row
/// applies it.
fn list_item(
    t: &Theme,
    label: &str,
    state: ListRowState,
    styled: bool,
    gutter: Option<gpui::Pixels>,
) -> WidgetInfo {
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
    let info = info
        .not_themeable("fill at rest", NO_LIST_FILL)
        .instance("right margin", scrollbar_room(styled, gutter));
    // The label is plain text, so it takes the row's own text style: the
    // foreground and text_base ListItem sets (list/list_item.rs:204-205),
    // which geometry::list_item then refines with list.item_font, colour
    // included -- a colour no ThemeColor field holds. A selected row is
    // refined by geometry::list_item_selected after it, over that colour.
    let selected = matches!(
        state,
        ListRowState::Selected | ListRowState::SelectedRightClicked
    );
    if styled && selected {
        info.instance("text", "list.selection_text_color, which geometry::list_item_selected sets on the selected row over list.item_font's colour; the size is list.item_font's, from geometry::list_item, over the text_base and foreground ListItem sets first (list/list_item.rs, ListItem::render)")
    } else if styled {
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

/// A row of the List reading `label`, in `state`, keeping `gutter` clear
/// for the scrollbar.
pub fn list_row(
    t: &Theme,
    label: &str,
    state: ListRowState,
    styled: bool,
    gutter: Option<gpui::Pixels>,
) -> WidgetInfo {
    list_item(t, label, state, styled, gutter).instance("click", "selects the row")
}

/// The Tree, with the box around it that is its frame. Its geometry line is
/// recorded where `demo::tree` applies the builder; its rows report
/// themselves (`tree_row`).
pub fn tree(native: Option<&ResolvedTheme>) -> WidgetInfo {
    list_fill(WidgetInfo::new("Tree"), native)
        .not_themeable("indent", "none here, and none upstream: Tree::new takes a render_item closure and tree.rs draws no row content of its own (gpui-component/tree.rs, Tree), so a row's indent is whatever the application's closure applies -- this demo's applies none")
        .not_themeable("disclosure icon", "the same: tree.rs contains no icon, so a chevron would be the closure's to draw. The panel used to claim a hardcoded ChevronRight, which is in neither this demo nor upstream")
        .instance("frame", "a tree is a list view and the model gives it no theme of its own, so its frame is the list's: the box around it")
        .instance("rows", "a file structure with src open; each row reports itself")
}

/// A row of the Tree reading `label`, `selected` or not, keeping `gutter`
/// clear for the scrollbar.
pub fn tree_row(
    t: &Theme,
    label: &str,
    selected: bool,
    styled: bool,
    gutter: Option<gpui::Pixels>,
) -> WidgetInfo {
    let state = if selected {
        ListRowState::Selected
    } else {
        ListRowState::Idle
    };
    list_item(t, label, state, styled, gutter)
        .not_themeable("right-click", "not followed here: Tree tells the row's builder whether the row is selected and nothing else (gpui-component/tree.rs, Tree::new), and marks a right-clicked row after it is built (gpui-component/tree.rs, RenderOnce for Tree). A right-clicked row paints no hover and gains a 1px selection outline, over the selected fill when it is selected (list/list_item.rs, ListItem::render)")
        .instance("click", "a full click, the press released on the row, selects it and opens or closes a folder; a press dragged off the row does neither (gpui-base/tree.rs, TreeState::on_entry_click)")
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

/// What a `Bubble` of `kind` paints: its arm of upstream's content surface
/// (bubble.rs, BubbleContent), the fill and the text, and an Outline's edge.
fn bubble_colours(t: &Theme, kind: BubbleKind) -> Vec<ColorClaim> {
    match kind {
        BubbleKind::Filled => vec![
            claim("bg", "primary", t.primary, "gpui-component/bubble.rs:215"),
            claim(
                "text",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/bubble.rs:216",
            ),
        ],
        BubbleKind::Secondary => vec![
            claim("bg", "muted", t.muted, "gpui-component/bubble.rs:222"),
            claim(
                "text",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/bubble.rs:223",
            ),
        ],
        BubbleKind::Muted => vec![
            claim("bg", "muted", t.muted, "gpui-component/bubble.rs:225"),
            claim(
                "text",
                "foreground",
                t.foreground,
                "gpui-component/bubble.rs:226",
            ),
        ],
        // `mix_oklab`'s factor is the first colour's share (theme/color.rs:
        // 44-49), and the arm picks it by the mode (bubble.rs:230).
        BubbleKind::Tinted => vec![
            if t.is_dark() {
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
            },
            claim(
                "text",
                "foreground",
                t.foreground,
                "gpui-component/bubble.rs:232",
            ),
        ],
        BubbleKind::Outline => vec![
            claim("border", "border", t.border, "gpui-component/bubble.rs:234"),
            claim(
                "bg",
                "background",
                t.background,
                "gpui-component/bubble.rs:235",
            ),
            claim(
                "text",
                "foreground",
                t.foreground,
                "gpui-component/bubble.rs:232",
            ),
        ],
        BubbleKind::Ghost => vec![
            claim(
                "bg",
                "transparent",
                t.transparent,
                "gpui-component/bubble.rs:240",
            ),
            claim(
                "text",
                "foreground",
                t.foreground,
                "gpui-component/bubble.rs:241",
            ),
        ],
        // The semantic tokens name danger destructive (theme/mod.rs:534); the
        // Destructive arm reads it at a share picked by the mode
        // (bubble.rs:244-249).
        BubbleKind::Destructive => vec![
            if t.is_dark() {
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
            },
            claim(
                "text, destructive (bubble.rs:249)",
                "danger",
                t.danger,
                "gpui-component/theme/mod.rs:534",
            ),
        ],
    }
}

/// A `Bubble` of `kind`, `outgoing` or incoming.
pub fn bubble(t: &Theme, kind: BubbleKind, outgoing: bool) -> WidgetInfo {
    let side = if outgoing { "outgoing" } else { "incoming" };
    let info = WidgetInfo::new("Bubble")
        .variant(format!("{}, {side}", kind.name()))
        .colors(bubble_colours(t, kind));
    let info = match kind {
        BubbleKind::Ghost => info.not_themeable("surface", "none: the Ghost arm drops the border, the corner radius and the padding -- border_0, the radius tokens' none, p_0 (bubble.rs, BubbleContent)"),
        BubbleKind::Tinted => bubble_surface(info, t).not_themeable("tint", "a mix of primary into background by a literal share, 12% in light mode and 24% in dark (bubble.rs, BubbleContent)"),
        _ => bubble_surface(info, t),
    };
    info.not_themeable("token path", "a Bubble reads cx.theme().semantic_tokens().colors, not the ThemeColor fields directly (bubble.rs, the content surface). The values are the same; the access path is a third one, beside cx.theme().field and the Theme impl's own self.tokens.field")
        .not_themeable("max width", "80% of the row unless the Bubble's own refinement, applied last, sets another (bubble.rs, Bubble::render)")
}

/// A `Message` row, `outgoing` or incoming, from `sender`, reading `text`.
/// Its Avatar reports itself; its Bubble -- Filled for an outgoing message,
/// Muted for an incoming one -- reports through it.
pub fn message(t: &Theme, outgoing: bool, sender: &str, text: &str) -> WidgetInfo {
    let (side, kind, place) = if outgoing {
        ("outgoing", BubbleKind::Filled, "end")
    } else {
        ("incoming", BubbleKind::Muted, "start")
    };
    WidgetInfo::new("Message")
        .variant(side)
        // The Message panel names its colours as the bubble's.
        .colors(bubble_colours(t, kind).into_iter().map(|c| ColorClaim {
            role: match c.role {
                "bg" => "bubble bg",
                "text" => "bubble text",
                role => role,
            },
            ..c
        }))
        .instance("bubble", format!("{}, at the {place} of the row. MessageContent::bubble takes the Bubble itself, so it reports through the Message (message.rs, MessageContent::bubble)", kind.name()))
        .not_themeable("bubbles", "a Message delegates its bubble to Bubble, so the fill is Bubble's and the citations point there")
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
            "gpui-component/message_scroller.rs:446",
        ))
        .color(claim(
            "jump button edge",
            "border",
            t.border,
            "gpui-component/message_scroller.rs:445",
        ))
        .color(claim(
            "jump button icon",
            "foreground",
            t.foreground,
            "gpui-component/message_scroller.rs:447",
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
            "gpui-component/attachment.rs:432",
        ))
        .color(claim(
            "title",
            "foreground",
            t.foreground,
            "gpui-component/attachment.rs:433",
        ));
    // The semantic tokens name danger destructive (theme/mod.rs:534); a
    // failed card reads it at full strength and at 10%.
    let info = if status == AttachmentStatus::Failed {
        info.color(claim(
            "border, destructive (attachment.rs:426-427)",
            "danger",
            t.danger,
            "gpui-component/theme/mod.rs:534",
        ))
        .color(claim(
            "media bg, destructive at 10% (attachment.rs:841-842)",
            "danger",
            t.danger.opacity(0.1),
            "gpui-component/theme/mod.rs:534",
        ))
        .color(claim(
            "status glyph, upstream's Ban, destructive (attachment.rs:801, 846-847)",
            "danger",
            t.danger,
            "gpui-component/theme/mod.rs:534",
        ))
        .color(claim(
            "description, destructive (attachment.rs:1142-1143)",
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
            "gpui-component/attachment.rs:429",
        ))
        .color(claim(
            "media bg",
            "muted",
            t.muted,
            "gpui-component/attachment.rs:844",
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
                    "gpui-component/attachment.rs:797",
                ))
            } else {
                // A raster icon keeps its own colours (`icon.note`).
                (icon.shown() && !icon.is_raster()).then(|| {
                    claim(
                        "media icon",
                        "foreground",
                        t.foreground,
                        "gpui-component/attachment.rs:849",
                    )
                })
            },
        )
        .color(claim(
            "description",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/attachment.rs:1144",
        ))
    };
    let info = match status {
        AttachmentStatus::Pending => info.not_themeable("pending", "a dashed border (attachment.rs, Attachment::render: border_dashed)"),
        AttachmentStatus::Uploading | AttachmentStatus::Processing => info
            .not_themeable("in-progress title", "the ShimmerText highlight, driven by the status (attachment.rs, AttachmentTitle::render)")
            .not_themeable("own icons", super::own_icons("the status Spinner's Loader (attachment.rs, AttachmentMedia::render; gpui-component/spinner.rs, Spinner)")),
        AttachmentStatus::Failed => info.not_themeable("own icons", super::own_icons("the failed status glyph, Ban (attachment.rs, AttachmentMedia::render)")),
        AttachmentStatus::Complete => info,
    };
    let info = if clickable {
        info.color(claim(
            "hover bg, muted at 50%",
            "muted",
            t.muted.opacity(0.5),
            "gpui-component/attachment.rs:438",
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
