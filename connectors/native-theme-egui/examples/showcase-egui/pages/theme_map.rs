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
    // (`showcase-gpui/pages/theme_map.rs:271-276`).
    caption(reg, ui, "All mapping.toml Rows");
    // The filter row is one instance in the base style: selectables, frameless at rest (§10.4).
    // It wraps at the page's width: a row wider than the page would widen the central panel past
    // the window, and the table's view across with it, so its last column could not be
    // scrolled into view.
    demo::contained(reg, ui, "verdict filter", |ui, reg| {
        ui.horizontal_wrapped(|ui| {
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
    // The header as the Data page's table has it: `list.header_font` at its weight
    // (`demo::weighted_family`) and scaled size (`list_header_font`), in
    // `list.header_font.color` (§10.4's Data row), not `RichText::strong`, which is the scope's
    // active-state text colour (`egui/src/style.rs:1147-1149`).
    let header_font = egui::FontId::new(
        list_header_font(t, atlas.accessibility()).size,
        demo::weighted_family(ui, t.list.header_font.weight, t.defaults.font.weight),
    );
    let header_color = to_color32(t.list.header_font.color);
    demo::scoped_container(
        reg,
        ui,
        Role::List,
        RoleVariant::Normal,
        "theme map",
        |ui, list, reg| {
            let heights = super::data::TableRows::of(ui, atlas, t, &header_font);
            const TITLES: [&str; 4] = ["Leaf", "Value", "Verdict", "Sinks / upstream"];
            // A header's text as drawn, its width rounded up to a whole point.
            let title_width = |ui: &egui::Ui, title: &str| {
                ui.fonts_mut(|f| {
                    f.layout_no_wrap(title.to_string(), header_font.clone(), header_color)
                })
                .size()
                .x
                .ceil()
            };
            let (value_floor, sinks_floor) =
                (title_width(ui, TITLES[1]), title_width(ui, TITLES[3]));
            // The leaf and the verdict as wide as their text, each frame's (egui_extras
            // `table.rs:844-846`); the value and the sinks share what is left equally
            // (`sizing.rs:141-166`), each at least its header's text (`Column::at_least`, a
            // remainder's `range.min`, `:146-153`) and clipped, so a text too long for it is cut
            // short and shown whole on hover (`super::data::one_line_cells`). A clipped
            // remainder is sized from the room each frame, never from what its cells drew
            // (`table.rs:617-646`), so a filter or a scroll never leaves it narrower than the
            // room, as a clipped auto column would be (`:844-846`, its cells' widths). Where the
            // columns are wider than the page, the rest is reached by scrolling across: the page
            // itself scrolls only down, and the content of a horizontal `ScrollArea` is first
            // given its width (`egui/src/containers/scroll_area.rs:784-798`).
            let out = ui.scope(|ui| {
                egui::ScrollArea::horizontal()
                    .id_salt("theme map/across")
                    .show(ui, |ui| {
                        super::data::one_line_cells(ui);
                        egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .column(egui_extras::Column::auto())
                            .column(
                                egui_extras::Column::remainder()
                                    .clip(true)
                                    .at_least(value_floor),
                            )
                            .column(egui_extras::Column::auto())
                            .column(
                                egui_extras::Column::remainder()
                                    .clip(true)
                                    .at_least(sinks_floor),
                            )
                            .header(heights.header, |mut header| {
                                for title in TITLES {
                                    header.col(|ui| {
                                        list.add(reg, ui, "theme map header", |ui| {
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
                            .body(|body| {
                                body.rows(heights.body, rows.len(), |mut row| {
                                    if let Some(r) = rows.get(row.index()) {
                                        // A colour value beside its swatch, as the gpui Theme
                                        // Map shows every colour
                                        // (`showcase-gpui/demo.rs:8449-8473`).
                                        let colour = crate::info::value_at(json, &r.leaf)
                                            .as_ref()
                                            .and_then(crate::info::swatch_colour);
                                        for (ix, text) in
                                            crate::info::row_cells(r, json).into_iter().enumerate()
                                        {
                                            let swatch = colour.filter(|_| ix == 1);
                                            row.col(|ui| {
                                                list.add(reg, ui, "theme map cell", |ui| {
                                                    match swatch {
                                                        Some(colour) => crate::info::swatch_row(
                                                            ui,
                                                            t,
                                                            colour,
                                                            (&text, egui::TextStyle::Body),
                                                            Some(heights.body_line),
                                                            &[],
                                                        ),
                                                        None => ui.label(heights.cell(text)),
                                                    }
                                                });
                                            });
                                        }
                                    }
                                });
                            });
                    })
            });
            out.response
        },
    );
}
