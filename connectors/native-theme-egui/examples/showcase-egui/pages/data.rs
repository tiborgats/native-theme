//! Data (spec §10.4's palette table): an egui_extras table and egui's `Grid` in `Role::List`,
//! and `StripBuilder` in the base style.

use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use native_theme_egui::convert::to_color32;
use native_theme_egui::{Role, RoleVariant, ThemeAtlas, list_header_font, list_header_height};
use native_theme_showcase_support::rows;

use super::{DemoState, caption};
use crate::demo::{self, Registry};

/// The rows the table and the grids show: a datum.
const ROWS: [(&str, &str); 4] = [
    ("kde-breeze", "KDE"),
    ("adwaita", "GNOME"),
    ("macos-monterey", "macOS"),
    ("windows-11", "Windows"),
];

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    // `list.header_font` at its weight, as the Basic table's header is
    // (`demo::weighted_family`), and at its scaled size (`list_header_font`)
    let header_font = egui::FontId::new(
        list_header_font(t, atlas.accessibility()).size,
        demo::weighted_family(ui, t.list.header_font.weight, t.defaults.font.weight),
    );
    let header_color = to_color32(t.list.header_font.color);
    let normal = RoleVariant::Normal;

    caption(
        reg,
        ui,
        "TableBuilder: striped, resizable columns, a selected row (Role::List)",
    );
    demo::scoped_container(reg, ui, Role::List, normal, "Table", |ui, list, reg| {
        // A table's row and header heights are call arguments no `Style` field reaches (§5.4
        // `list.row_height`); a header the theme gives no height is a row.
        let heights = TableRows::of(ui, atlas, t, &header_font);
        ui.scope(|ui| {
            one_line_cells(ui);
            TableBuilder::new(ui)
                .id_salt("data/table")
                .striped(true)
                .resizable(true)
                .vscroll(false)
                .sense(egui::Sense::click())
                .column(Column::auto().resizable(true))
                .column(Column::remainder().clip(true))
                .header(heights.header, |mut header| {
                    for title in ["Theme", "Platform"] {
                        header.col(|ui| {
                            list.add(reg, ui, "table header", |ui| {
                                ui.label(
                                    egui::RichText::new(title)
                                        .font(header_font.clone())
                                        .color(header_color)
                                        .line_height(Some(heights.header_line)),
                                )
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

/// An egui_extras table's body row and header heights, the Data and Theme Map tables' alike,
/// with the line box each row's text is drawn in: `list.row_height` and `list.header_height`
/// (else the row's, as a header the theme gives no height is a row), each grown above a text
/// scale of 1 to hold its line, in `list.item_font` and `list.header_font` at
/// `defaults.line_height`, as gpui's table rows grow and the Basic table's
/// (`rows::scaled_height`); where the theme states neither, egui's own `interact_size.y`, or
/// the line box rounded up, as that rule rounds it, where that is taller, as egui's own rows
/// are at least `interact_size.y` (`egui/src/widgets/button.rs:306-309`) and as tall as their
/// line. The cells draw no padding and no line inside the row, and each is one line in its
/// line box, `defaults.line_height` of its font's drawn size ([`one_line_cells`]), so no row
/// is taller than these heights.
pub(crate) struct TableRows {
    pub(crate) body: f32,
    pub(crate) header: f32,
    pub(crate) body_line: f32,
    pub(crate) header_line: f32,
}

impl TableRows {
    /// The heights in `ui`, the table's `Role::List` scope, whose `TextStyle::Body` is the cells'
    /// font and whose `interact_size.y` egui's own where the theme states no `list.row_height`;
    /// the header drawn in `header_font`.
    pub(crate) fn of(
        ui: &egui::Ui,
        atlas: &ThemeAtlas,
        t: &native_theme_egui::ResolvedTheme,
        header_font: &egui::FontId,
    ) -> Self {
        let l = &t.list;
        let line_height = t.defaults.line_height;
        let own = ui.spacing().interact_size.y;
        let body_line = egui::TextStyle::Body.resolve(ui.style()).size * line_height;
        let header_line = header_font.size * line_height;
        let height = |stated: Option<f32>, size: f32, line: f32| match stated {
            Some(h) => rows::scaled_height(h, atlas.accessibility(), size, line_height, 0.0, 0.0),
            None => own.max(line.ceil()),
        };
        Self {
            body: height(l.row_height, l.item_font.size, body_line),
            header: height(
                list_header_height(t).or(l.row_height),
                l.header_font.size,
                header_line,
            ),
            body_line,
            header_line,
        }
    }

    /// A body cell's text, in its line box.
    pub(crate) fn cell(&self, text: impl Into<String>) -> egui::RichText {
        egui::RichText::new(text).line_height(Some(self.body_line))
    }
}

/// The cells of the table `ui` is about to lay out, each one line: text is not wrapped, so an
/// auto-sized column takes its text's width (egui_extras `table.rs:836-846`, the width its
/// cells used) and the last column, `Column::clip`, cuts a text too long for it short with egui's
/// ellipsis (`layout.rs:218-224`) in a cell no taller than its row (`:160-168`), the whole text
/// on hover as egui's truncated `Label` shows it (`egui/src/widgets/label.rs:285-294`); and the
/// rows lie with no spacing between them, as the Basic table's, so each is its height apart
/// (egui_extras `layout.rs:178-183`, `table.rs:1033-1034`).
pub(crate) fn one_line_cells(ui: &mut egui::Ui) {
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
    ui.spacing_mut().item_spacing.y = 0.0;
}

/// The grids' cells, each recorded with the grid's seam.
fn grid_rows(ui: &mut egui::Ui, list: demo::Applied, reg: &mut Registry) {
    for (theme, platform) in ROWS {
        list.add(reg, ui, "grid cell", |ui| ui.label(theme));
        list.add(reg, ui, "grid cell", |ui| ui.label(platform));
        ui.end_row();
    }
}
