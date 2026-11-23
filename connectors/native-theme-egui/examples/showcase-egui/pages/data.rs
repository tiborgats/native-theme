//! Data (spec §10.4's palette table): an egui_extras table and egui's `Grid` in `Role::List`,
//! and `StripBuilder` in the base style.

use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use native_theme::next::{ResolvedTheme, TextStated, WidgetState};
use native_theme_egui::next::{self, list_header_height};
use native_theme_egui::{Role, RoleVariant};

use super::{DemoState, caption};
use crate::app::OldAtlas;
use crate::demo::{self, Registry};

/// The rows the table and the grids show: a datum.
const ROWS: [(&str, &str); 4] = [
    ("kde-breeze", "KDE"),
    ("adwaita", "GNOME"),
    ("macos-monterey", "macOS"),
    ("windows-11", "Windows"),
];

pub(crate) fn show(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let head = TableHeader::of(ui, r);
    let normal = RoleVariant::Normal;

    caption(
        reg,
        ui,
        "TableBuilder: striped, resizable columns, a selected row (Role::List)",
    );
    demo::scoped_container(reg, ui, Role::List, normal, "Table", |ui, list, reg| {
        // A table's row and header heights are call arguments no `Style` field reaches (§5.4
        // `list.row_height`); a header the theme gives no height is a row.
        let heights = TableRows::of(ui, r, &head.font);
        ui.scope(|ui| {
            one_line_cells(ui);
            TableBuilder::new(ui)
                .id_salt("data/table")
                .striped(true)
                .resizable(true)
                .vscroll(false)
                .sense(egui::Sense::click())
                .cell_layout(centred_cells())
                .column(Column::auto().resizable(true))
                .column(Column::remainder().clip(true))
                .header(heights.header, |mut header| {
                    for title in ["Theme", "Platform"] {
                        header.col(|ui| {
                            list.add(reg, ui, "table header", |ui| {
                                ui.label(heights.title(&head, title))
                            });
                        });
                    }
                })
                .body(|mut body| {
                    for (i, (theme, platform)) in ROWS.into_iter().enumerate() {
                        body.row(heights.body, |mut row| {
                            row.set_selected(state.table_selected == Some(i));
                            for text in [theme, platform] {
                                row.col(|ui| {
                                    list.add(reg, ui, "table cell", |ui| {
                                        ui.label(heights.cell(text))
                                    });
                                });
                            }
                            if row.response().clicked() {
                                state.table_selected = Some(i);
                            }
                        });
                    }
                });
        })
        .response
    });

    caption(
        reg,
        ui,
        "Grid: striped (Role::List), and disabled (RoleVariant::Disabled)",
    );
    ui.horizontal_top(|ui| {
        demo::scoped_container(
            reg,
            ui,
            Role::List,
            normal,
            "Grid (striped)",
            |ui, list, reg| {
                egui::Grid::new("data/grid")
                    .striped(true)
                    .show(ui, |ui| grid_rows(ui, list, reg))
                    .response
            },
        );
        demo::scoped_container(
            reg,
            ui,
            Role::List,
            RoleVariant::Disabled,
            "Grid (disabled)",
            |ui, list, reg| {
                ui.disable();
                egui::Grid::new("data/grid-disabled")
                    .striped(true)
                    .show(ui, |ui| grid_rows(ui, list, reg))
                    .response
            },
        );
    });

    caption(reg, ui, "StripBuilder (the base style)");
    demo::contained(reg, ui, "StripBuilder", |ui, reg| {
        egui::Resize::default()
            .id_salt("data/strip-area")
            .show(ui, |ui| {
                StripBuilder::new(ui)
                    .size(Size::remainder())
                    .size(Size::remainder())
                    .horizontal(|mut strip| {
                        for text in ["First strip cell", "Second strip cell"] {
                            strip.cell(|ui| {
                                demo::base(reg, ui, "strip cell", |ui| ui.label(text));
                            });
                        }
                    })
            })
    });
}

/// The font and colour of an egui_extras table's header, the Data and Theme Map tables' alike:
/// `list.header_label(NONE)` of the installed 0.8.0 theme, its size (`next::list_header_font`)
/// at its weight (`demo::weighted_family` over the body's, `next::font_weight`), as the Basic
/// table's header is, in its colour (§10.4's Data row), not `RichText::strong`, which is the
/// scope's active-state text colour (`egui/src/style.rs:1147-1149`). Each property the theme
/// leaves unstated, and all of them without an installed 0.8.0 theme, egui's own: the scope's
/// `Body` size, the body's face and the scope's text colour (`Visuals::text_color`).
pub(crate) struct TableHeader {
    pub(crate) font: egui::FontId,
    pub(crate) color: egui::Color32,
}

impl TableHeader {
    pub(crate) fn of(ui: &egui::Ui, r: Option<&ResolvedTheme>) -> Self {
        let size = egui::TextStyle::Body.resolve(ui.style()).size;
        let color = ui.visuals().text_color();
        let Some(r) = r else {
            return Self {
                font: egui::FontId::proportional(size),
                color,
            };
        };
        let label = r.list.header_label(WidgetState::NONE);
        let weight = label
            .stated
            .contains(TextStated::WEIGHT)
            .then_some(label.weight);
        let family = match (weight, next::font_weight(r)) {
            (Some(weight), Ok(body)) => demo::weighted_family(ui, weight, body),
            _ => egui::FontFamily::Proportional,
        };
        Self {
            font: egui::FontId::new(
                next::list_header_font(r).map_or(size, |font| font.size),
                family,
            ),
            color: next::text_color_of(&label).unwrap_or(color),
        }
    }
}

/// An egui_extras table's body row and header heights, the Data and Theme Map tables' alike,
/// with the line box each row's text is drawn in, of the installed 0.8.0 theme:
/// `list.row(NONE).min_size[1]` and `list.header(NONE).min_size[1]` (else the row's, as a header
/// the theme gives no height is a row), each grown above a text scale of 1 to hold its line, in
/// `list.item_label(NONE)` and `list.header_label(NONE)` (`demo::next_line_box`), as gpui's
/// table rows grow (`demo::next_grown`); where the theme states neither, egui's own
/// `interact_size.y`, or the line box rounded up, as that rule rounds it, where that is taller,
/// as egui's own rows are at least `interact_size.y` (`egui/src/widgets/button.rs:306-309`) and
/// as tall as their line. The cells draw no padding and no line inside the row, and each is one
/// line in its line box, of its font's drawn size ([`one_line_cells`]), so a row is these
/// heights tall, except a stated one shorter than its line box at a text scale of 1 or less,
/// which grows to hold its line (egui_extras `layout.rs:160-168`); in a row taller than its line
/// the text is centred ([`centred_cells`]). Without an installed 0.8.0 theme, egui's own rows
/// and the text's own line.
pub(crate) struct TableRows {
    pub(crate) body: f32,
    pub(crate) header: f32,
    pub(crate) body_line: Option<f32>,
    pub(crate) header_line: Option<f32>,
}

impl TableRows {
    /// The heights in `ui`, the table's `Role::List` scope, whose `TextStyle::Body` is the cells'
    /// font and whose `interact_size.y` egui's own where the theme states no
    /// `list.row(NONE).min_size[1]`; the header drawn in `header_font`.
    pub(crate) fn of(ui: &egui::Ui, r: Option<&ResolvedTheme>, header_font: &egui::FontId) -> Self {
        let own = ui.spacing().interact_size.y;
        let Some(r) = r else {
            return Self {
                body: own,
                header: own,
                body_line: None,
                header_line: None,
            };
        };
        let l = &r.list;
        let (item, label) = (
            l.item_label(WidgetState::NONE),
            l.header_label(WidgetState::NONE),
        );
        let body_line =
            demo::next_line_box(r, &item, egui::TextStyle::Body.resolve(ui.style()).size);
        let header_line = demo::next_line_box(r, &label, header_font.size);
        // a stated row shorter than one line box still grows to its line, as the connector's
        // List rows do (mapping.toml, `list.row_height`): no bundled preset states one
        let height =
            |stated: Option<f32>, style: &native_theme::next::TextStyle, line: f32| match stated {
                Some(h) => {
                    demo::next_grown(r, h, demo::next_line_box(r, style, style.size_px), 0.0, 0.0)
                }
                None => own.max(line.ceil()),
            };
        let [_, row] = l.row(WidgetState::NONE).min_size;
        Self {
            body: height(row, &item, body_line),
            header: height(list_header_height(r).or(row), &label, header_line),
            body_line: Some(body_line),
            header_line: Some(header_line),
        }
    }

    /// A body cell's text, in its line box.
    pub(crate) fn cell(&self, text: impl Into<String>) -> egui::RichText {
        egui::RichText::new(text).line_height(self.body_line)
    }

    /// A header's `title`, in `header`'s font and colour and its line box.
    pub(crate) fn title(&self, header: &TableHeader, title: &str) -> egui::RichText {
        egui::RichText::new(title)
            .font(header.font.clone())
            .color(header.color)
            .line_height(self.header_line)
    }
}

/// The cells of the table `ui` is about to lay out, each one line: text is not wrapped, so an
/// auto-sized column takes its text's width (egui_extras `table.rs:836-846`, the width its
/// cells used) and a clipped column (`Column::clip`: the Data table's last, the Theme Map's
/// Value and Sinks) cuts a text too long for it short with egui's ellipsis
/// (`layout.rs:218-224`) in a cell no taller than its row (`:160-168`), the whole text on
/// hover as egui's truncated `Label` shows it (`egui/src/widgets/label.rs:285-294`); and the
/// rows lie with no spacing between them, as the Basic table's, so each is its height apart
/// (egui_extras `layout.rs:178-183`, `table.rs:1033-1034`).
pub(crate) fn one_line_cells(ui: &mut egui::Ui) {
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
    ui.spacing_mut().item_spacing.y = 0.0;
}

/// The layout of a table's cells, the header's and the body's: left to right, each cell's
/// content centred down the row's height, as the gpui showcase's table centres each cell's text
/// in its row (`showcase-gpui/demo.rs`, `files_table`'s cells, `.h_full().flex().items_center()`)
/// and the Basic table's painted cells are (`pages/basic.rs`, `paint_cell_text`). A horizontal
/// layout centred on its cross axis gives each widget the cell's whole height and centres it
/// there (egui `src/layout.rs:604-623`), where the table's default, the enclosing top-down
/// layout (egui_extras `table.rs:260`), put the text at the row's top and left a stated
/// `list.header_height` or `list.row_height` taller than the line blank below it.
pub(crate) fn centred_cells() -> egui::Layout {
    egui::Layout::left_to_right(egui::Align::Center)
}

/// The grids' cells, each recorded with the grid's seam.
fn grid_rows(ui: &mut egui::Ui, list: demo::Applied, reg: &mut Registry) {
    for (theme, platform) in ROWS {
        list.add(reg, ui, "grid cell", |ui| ui.label(theme));
        list.add(reg, ui, "grid cell", |ui| ui.label(platform));
        ui.end_row();
    }
}
