//! Theme Map (spec §10.4): every `mapping.toml` row in a table, filtered by verdict, in
//! `Role::List` — the siblings' Theme Map pages list their toolkit's slots; this one lists the
//! manifest that maps egui's.

use native_theme_egui::{Role, RoleVariant, ThemeAtlas};

use super::DemoState;
use crate::demo::{self, Registry};
use crate::info::{Manifest, Row, Verdict};

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    manifest: &Result<Manifest, String>,
    ui: &mut egui::Ui,
) {
    let theme = ui.ctx().theme();
    let json = crate::info::theme_json(atlas, theme);
    // The filter row is one instance in the base style: selectables, frameless at rest (§10.4).
    demo::base(reg, ui, "verdict filter", |ui| {
        ui.horizontal(|ui| {
            for (label, filter) in [
                ("All", None),
                ("Direct", Some(Verdict::Direct)),
                ("Scoped", Some(Verdict::Scoped)),
                ("Derived", Some(Verdict::Derived)),
                ("Unmappable", Some(Verdict::Unmappable)),
            ] {
                ui.selectable_value(&mut state.theme_map_filter, filter, label);
            }
        })
        .response
    });
    let (Ok(manifest), Ok(json)) = (manifest, &json) else {
        ui.label(
            manifest
                .as_ref()
                .err()
                .or(json.as_ref().err())
                .map_or(String::new(), Clone::clone),
        );
        return;
    };
    let rows: Vec<&Row> = manifest
        .rows
        .iter()
        .filter(|r| state.theme_map_filter.is_none_or(|v| r.verdict == v))
        .collect();
    let row_height = atlas.resolved_for(theme).list.row_height;
    demo::scoped(
        reg,
        ui,
        Role::List,
        RoleVariant::Normal,
        "theme map",
        |ui| {
            let height = row_height.unwrap_or(ui.spacing().interact_size.y);
            let out = ui.scope(|ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::remainder())
                    .header(height, |mut header| {
                        for title in ["Leaf", "Value", "Verdict", "Sinks / upstream"] {
                            header.col(|ui| {
                                ui.strong(title);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(height, rows.len(), |mut row| {
                            if let Some(r) = rows.get(row.index()) {
                                for text in crate::info::row_cells(r, json) {
                                    row.col(|ui| {
                                        ui.label(text);
                                    });
                                }
                            }
                        });
                    });
            });
            out.response
        },
    );
}
