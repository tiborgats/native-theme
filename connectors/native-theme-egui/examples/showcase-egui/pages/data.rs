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
    demo::scoped(reg, ui, Role::List, normal, "Table", |ui| {
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
                            ui.label(
                                egui::RichText::new(title)
                                    .font(header_font.clone())
                                    .color(header_color),
                            );
                        });
                    }
                })
                .body(|mut body| {
                    for (i, (theme, platform)) in ROWS.into_iter().enumerate() {
                        body.row(height, |mut row| {
                            row.set_selected(state.table_selected == Some(i));
                            row.col(|ui| {
                                ui.label(theme);
                            });
                            row.col(|ui| {
                                ui.label(platform);
                            });
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
        demo::scoped(reg, ui, Role::List, normal, "Grid (striped)", |ui| {
            egui::Grid::new("data/grid")
                .striped(true)
                .show(ui, grid_rows)
                .response
        });
        demo::scoped(
            reg,
            ui,
            Role::List,
            RoleVariant::Disabled,
            "Grid (disabled)",
            |ui| {
                ui.disable();
                egui::Grid::new("data/grid-disabled")
                    .striped(true)
                    .show(ui, grid_rows)
                    .response
            },
        );
    });

    caption(reg, ui, "StripBuilder (the base style)");
    demo::base(reg, ui, "StripBuilder", |ui| {
        egui::Resize::default()
            .id_salt("data/strip-area")
            .show(ui, |ui| {
                StripBuilder::new(ui)
                    .size(Size::remainder())
                    .size(Size::remainder())
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.label("First strip cell");
                        });
                        strip.cell(|ui| {
                            ui.label("Second strip cell");
                        });
                    })
            })
    });
}

fn grid_rows(ui: &mut egui::Ui) {
    for (theme, platform) in ROWS {
        ui.label(theme);
        ui.label(platform);
        ui.end_row();
    }
}
