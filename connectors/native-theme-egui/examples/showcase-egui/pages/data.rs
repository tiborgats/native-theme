//! Data (spec §10.4's palette table): an egui_extras table and egui's `Grid` in `Role::List`,
//! and `StripBuilder` in the base style.

use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use native_theme_egui::convert::to_color32;
use native_theme_egui::{Role, RoleVariant, ThemeAtlas, list_header_font};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

/// The rows the table and the grids show: a datum.
const ROWS: [(&str, &str); 4] = [
    ("kde-breeze", "KDE"),
    ("adwaita", "GNOME"),
    ("macos-sonoma", "macOS"),
    ("windows-11", "Windows"),
];

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let header_font = list_header_font(t, atlas.accessibility());
    let header_color = to_color32(t.list.header_font.color);
    let row_height = t.list.row_height;
    let normal = RoleVariant::Normal;

    caption(
        reg,
        ui,
        "TableBuilder: striped, resizable columns, a selected row (Role::List)",
    );
    demo::scoped_container(reg, ui, Role::List, normal, "Table", |ui, list, reg| {
        // A table's row height is a call argument no `Style` field reaches (§5.4 `list.row_height`).
        let height = row_height.unwrap_or(ui.spacing().interact_size.y);
        ui.scope(|ui| {
            TableBuilder::new(ui)
                .id_salt("data/table")
                .striped(true)
                .resizable(true)
                .vscroll(false)
                .sense(egui::Sense::click())
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(height, |mut header| {
                    for title in ["Theme", "Platform"] {
                        header.col(|ui| {
                            list.add(reg, ui, "table header", |ui| {
                                ui.label(
                                    egui::RichText::new(title)
                                        .font(header_font.clone())
                                        .color(header_color),
                                )
                            });
                        });
                    }
                })
                .body(|mut body| {
                    for (i, (theme, platform)) in ROWS.into_iter().enumerate() {
                        body.row(height, |mut row| {
                            row.set_selected(state.table_selected == Some(i));
                            for text in [theme, platform] {
                                row.col(|ui| {
                                    list.add(reg, ui, "table cell", |ui| ui.label(text));
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
    reg.amend_last(|i| {
        i.read.push(("list.row_height", format!("{row_height:?}")));
        i.read
            .push(("list_header_font", format!("{header_font:?}")));
        i.read
            .push(("list.header_font.color", format!("{header_color:?}")));
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

/// The grids' cells, each recorded with the grid's seam.
fn grid_rows(ui: &mut egui::Ui, list: demo::Applied, reg: &mut Registry) {
    for (theme, platform) in ROWS {
        list.add(reg, ui, "grid cell", |ui| ui.label(theme));
        list.add(reg, ui, "grid cell", |ui| ui.label(platform));
        ui.end_row();
    }
}
