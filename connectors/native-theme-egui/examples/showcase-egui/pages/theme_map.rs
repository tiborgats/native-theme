//! Theme Map (spec §10.4): every `mapping.toml` row in a table, filtered by verdict, in
//! `Role::List` — the siblings' Theme Map pages list their toolkit's slots; this one lists the
//! manifest that maps egui's.

use native_theme_egui::convert::to_color32;
use native_theme_egui::{Role, RoleVariant, ThemeAtlas, list_header_font};

use super::{DemoState, caption};
use crate::demo::{self, Registry};
use crate::info::{Manifest, Row, Verdict};

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    manifest: &Result<Manifest, String>,
    json: &crate::info::JsonCache,
    ui: &mut egui::Ui,
) {
    let theme = ui.ctx().theme();
    let json = json.get(atlas, theme);
    // The page's heading, as the gpui Theme Map's "All ThemeColor Fields"
    // (`showcase-gpui/pages/theme_map.rs:276-280`).
    caption(reg, ui, "All mapping.toml Rows");
    // The filter row is one instance in the base style: selectables, frameless at rest (§10.4).
    demo::contained(reg, ui, "verdict filter", |ui, reg| {
        ui.horizontal(|ui| {
            for (label, filter) in [
                ("All", None),
                ("Direct", Some(Verdict::Direct)),
                ("Scoped", Some(Verdict::Scoped)),
                ("Derived", Some(Verdict::Derived)),
                ("Unmappable", Some(Verdict::Unmappable)),
            ] {
                demo::base(reg, ui, "verdict", |ui| {
                    ui.selectable_value(&mut state.theme_map_filter, filter, label)
                });
            }
        })
        .response
    });
    let (Ok(manifest), Ok(json)) = (manifest, json) else {
        let error = manifest
            .as_ref()
            .err()
            .or(json.as_ref().err())
            .map_or(String::new(), Clone::clone);
        demo::base(reg, ui, "theme map error", |ui| ui.label(error));
        return;
    };
    let rows: Vec<&Row> = manifest
        .rows
        .iter()
        .filter(|r| state.theme_map_filter.is_none_or(|v| r.verdict == v))
        .collect();
    let t = atlas.resolved_for(theme);
    let row_height = t.list.row_height;
    // The header as the Data page's table has it: `list_header_font` in `list.header_font.color`
    // (§10.4's Data row), not `RichText::strong`, which is the scope's active-state text colour
    // (`egui/src/style.rs:1147-1149`).
    let header_font = list_header_font(t, atlas.accessibility());
    let header_color = to_color32(t.list.header_font.color);
    demo::scoped_container(
        reg,
        ui,
        Role::List,
        RoleVariant::Normal,
        "theme map",
        |ui, list, reg| {
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
                                list.add(reg, ui, "theme map header", |ui| {
                                    ui.label(
                                        egui::RichText::new(title)
                                            .font(header_font.clone())
                                            .color(header_color),
                                    )
                                });
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(height, rows.len(), |mut row| {
                            if let Some(r) = rows.get(row.index()) {
                                // A colour value beside its swatch, as the gpui Theme Map shows
                                // every colour (`showcase-gpui/demo.rs:5407-5428`).
                                let colour = crate::info::value_at(json, &r.leaf)
                                    .as_ref()
                                    .and_then(crate::info::swatch_colour);
                                for (ix, text) in
                                    crate::info::row_cells(r, json).into_iter().enumerate()
                                {
                                    let swatch = colour.filter(|_| ix == 1);
                                    row.col(|ui| {
                                        list.add(reg, ui, "theme map cell", |ui| match swatch {
                                            Some(colour) => crate::info::swatch_row(
                                                ui,
                                                t,
                                                colour,
                                                (&text, egui::TextStyle::Body),
                                                &[],
                                            ),
                                            None => ui.label(text),
                                        });
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
