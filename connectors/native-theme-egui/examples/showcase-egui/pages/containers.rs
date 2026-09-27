//! Containers (spec §10.4's palette table): the card surface beside `ui.group`, the expander
//! role, scroll areas, a nested right panel, and the containers and layout methods
//! native-theme models no counterpart for, in the base style. A container that would take
//! the whole of the page's height — a scroll area, a panel, a `Scene`, a strip — sits in a
//! `Resize` at egui's own default size; the nested panels sit above the page's `ScrollArea`
//! (`nested_panels`).

use egui::emath::TSTransform;
use native_theme_egui::{PanelSide, Role, RoleVariant, Surface, ThemeAtlas, expander_icon};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

/// The rows the scroll areas scroll: a datum.
const SCROLL_ROWS: usize = 100;

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    caption(
        reg,
        ui,
        "Frame in the card surface (Surface::Card, body Role::Card), beside ui.group",
    );
    ui.horizontal_top(|ui| {
        demo::framed(
            reg,
            ui,
            Surface::Card,
            Some((Role::Card, normal)),
            "card",
            |ui, reg| {
                demo::scoped(reg, ui, Role::Card, normal, "card label", |ui| {
                    ui.label("A card")
                });
                let variant = if state.checked {
                    RoleVariant::Selected
                } else {
                    normal
                };
                demo::scoped(reg, ui, Role::Checkbox, variant, "card checkbox", |ui| {
                    ui.checkbox(&mut state.checked, "A check box in the card")
                });
            },
        );
        demo::contained(reg, ui, "ui.group", |ui, reg| {
            ui.group(|ui| {
                demo::base(reg, ui, "group label", |ui| {
                    ui.label("ui.group keeps egui's own Frame::group")
                });
            })
            .response
        });
    });

    caption(reg, ui, "Expanders (Role::Expander)");
    demo::scoped(
        reg,
        ui,
        Role::Expander,
        normal,
        "CollapsingHeader (expander_icon)",
        |ui| {
            egui::CollapsingHeader::new("CollapsingHeader with expander_icon")
                .icon(expander_icon(t))
                .show(ui, |ui| ui.label("The body"))
                .header_response
        },
    );
    demo::scoped(reg, ui, Role::Expander, normal, "ui.collapsing", |ui| {
        ui.collapsing("ui.collapsing, egui's own icon", |ui| ui.label("The body"))
            .header_response
    });
    demo::scoped(reg, ui, Role::Expander, normal, "CollapsingState", |ui| {
        let id = ui.make_persistent_id("containers/collapsing-state");
        let state =
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);
        let header = state.show_header(ui, |ui| ui.label("CollapsingState with a custom header"));
        let (toggle, header, _) = header.body(|ui| ui.label("The HeaderResponse's body"));
        // The toggle and the custom header together: the one instance the helper records.
        toggle.union(header.response)
    });

    caption(
        reg,
        ui,
        "ScrollArea: the base style; show_rows in Role::Scrollbar",
    );
    // One below the other: three `Resize`s side by side are wider than the page, whose
    // vertical `ScrollArea` would then clip its right edge.
    ui.vertical(|ui| {
        demo::contained(reg, ui, "ScrollArea (vertical)", |ui, reg| {
            egui::Resize::default()
                .id_salt("containers/scroll-vertical")
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("containers/vertical")
                        .show(ui, |ui| {
                            for row in 0..SCROLL_ROWS {
                                demo::base(reg, ui, "scrolled label", |ui| {
                                    ui.label(format!("Vertical row {row}"))
                                });
                            }
                        });
                    ui.response()
                })
        });
        demo::contained(reg, ui, "ScrollArea (both)", |ui, reg| {
            egui::Resize::default()
                .id_salt("containers/scroll-both")
                .show(ui, |ui| {
                    egui::ScrollArea::both()
                        .id_salt("containers/both")
                        .show(ui, |ui| {
                            for row in 0..SCROLL_ROWS {
                                demo::base(reg, ui, "scrolled label", |ui| {
                                    ui.add(
                                        egui::Label::new(format!(
                                            "Row {row}, a line wider than the area it scrolls in"
                                        ))
                                        .extend(),
                                    )
                                });
                            }
                        });
                    ui.response()
                })
        });
        demo::scoped_container(
            reg,
            ui,
            Role::Scrollbar,
            normal,
            "ScrollArea (show_rows)",
            |ui, scrollbar, reg| {
                egui::Resize::default()
                    .id_salt("containers/scroll-rows")
                    .show(ui, |ui| {
                        let row_height = ui.text_style_height(&egui::TextStyle::Body);
                        egui::ScrollArea::vertical()
                            .id_salt("containers/rows")
                            .show_rows(ui, row_height, SCROLL_ROWS, |ui, rows| {
                                for row in rows {
                                    scrollbar.add(reg, ui, "show_rows label", |ui| {
                                        ui.label(format!("show_rows row {row}"))
                                    });
                                }
                            });
                        ui.response()
                    })
            },
        );
    });

    caption(reg, ui, "Containers with no counterpart (the base style)");
    // `Sides::show` takes both sides' closures at once, so one helper records the pair: the
    // union of the two labels, a non-container instance whose rect holds both.
    demo::base(reg, ui, "Sides", |ui| {
        let (left, right) =
            egui::Sides::new().show(ui, |ui| ui.label("Sides: left"), |ui| ui.label("right"));
        left.union(right)
    });
    demo::contained(reg, ui, "Resize", |ui, reg| {
        egui::Resize::default()
            .id_salt("containers/resize")
            .show(ui, |ui| {
                demo::base(reg, ui, "Resize label", |ui| {
                    ui.label("Resize: drag the corner")
                });
                ui.response()
            })
    });
    demo::contained(reg, ui, "Scene", |ui, reg| {
        egui::Resize::default()
            .id_salt("containers/scene-area")
            .show(ui, |ui| {
                egui::Scene::new()
                    .show(ui, &mut state.scene_rect, |ui| {
                        demo::scoped(reg, ui, Role::Button, normal, "scene button", |ui| {
                            ui.button("A button in the Scene")
                        });
                    })
                    .response
            })
    });
    ui.columns(2, |columns| {
        for (i, column) in columns.iter_mut().enumerate() {
            demo::base(reg, column, "ui.columns", |ui| {
                ui.label(format!("ui.columns: column {i}"))
            });
        }
    });
    ui.columns_const(|[left, right]| {
        demo::base(reg, left, "ui.columns_const", |ui| {
            ui.label("ui.columns_const: left")
        });
        demo::base(reg, right, "ui.columns_const", |ui| ui.label("right"));
    });
    drag_and_drop(reg, state, ui);
    ui.with_visual_transform(
        TSTransform::from_translation(ui.spacing().item_spacing),
        |ui| {
            demo::base(reg, ui, "ui.with_visual_transform", |ui| {
                ui.label("ui.with_visual_transform, moved by the item spacing")
            });
        },
    );

    caption(
        reg,
        ui,
        "The layout methods: each places other widgets and paints nothing of its own",
    );
    layouts(reg, ui);
}

/// `Panel::right` beside a `CentralPanel`, nested in the page above its `ScrollArea`: a panel
/// replaces its `Ui`'s clip rect with its own rect instead of narrowing it
/// (`egui/src/containers/panel.rs:824`, `:1241`), so where the page scrolls it out of view it
/// would paint over the chrome. The `Resize` holding them grows no taller than the room left.
pub(crate) fn nested_panels(reg: &mut Registry, ui: &mut egui::Ui) {
    caption(
        reg,
        ui,
        "Panel::right with a CentralPanel, nested (Surface::Panel(PanelSide::Right))",
    );
    let room = ui.available_size();
    demo::contained(reg, ui, "Resize (panels)", |ui, reg| {
        egui::Resize::default()
            .id_salt("containers/panels")
            .max_size(room)
            .show(ui, |ui| {
                demo::surfaced(
                    reg,
                    ui,
                    Surface::Panel(PanelSide::Right),
                    false,
                    None,
                    "right panel",
                    |ui, chrome, reg| {
                        Some(
                            egui::Panel::right("containers/right-panel")
                                .frame(chrome.frame)
                                .show(ui, |ui| {
                                    demo::base(reg, ui, "right panel label", |ui| {
                                        ui.label("Panel::right")
                                    });
                                })
                                .response,
                        )
                    },
                );
                demo::contained(reg, ui, "CentralPanel (nested)", |ui, reg| {
                    egui::CentralPanel::default()
                        .show(ui, |ui| {
                            demo::base(reg, ui, "central panel label", |ui| {
                                ui.label("CentralPanel")
                            });
                        })
                        .response
                });
                ui.response()
            })
    });
}

/// `ui.dnd_drag_source` and `ui.dnd_drop_zone`: two columns whose items move between them.
fn drag_and_drop(reg: &mut Registry, state: &mut DemoState, ui: &mut egui::Ui) {
    let mut dropped: Option<(usize, &'static str)> = None;
    ui.horizontal_top(|ui| {
        for (column, items) in state.dnd_columns.iter().enumerate() {
            demo::contained(reg, ui, "ui.dnd_drop_zone", |ui, reg| {
                let (zone, payload) =
                    ui.dnd_drop_zone::<&'static str, ()>(egui::Frame::default(), |ui| {
                        ui.set_min_size(ui.spacing().interact_size);
                        for item in items {
                            let id = egui::Id::new(("containers/dnd", *item));
                            demo::base(reg, ui, "ui.dnd_drag_source", |ui| {
                                ui.dnd_drag_source(id, *item, |ui| ui.label(*item)).response
                            });
                        }
                    });
                if let Some(item) = payload {
                    dropped = Some((column, *item));
                }
                zone.response
            });
        }
    });
    if let Some((column, item)) = dropped {
        for items in &mut state.dnd_columns {
            items.retain(|i| *i != item);
        }
        if let Some(items) = state.dnd_columns.get_mut(column) {
            items.push(item);
        }
    }
}

/// Every layout method §10.4's Containers row lists, each around a base-style label.
fn layouts(reg: &mut Registry, ui: &mut egui::Ui) {
    demo::base(reg, ui, "ui.add", |ui| ui.add(egui::Label::new("ui.add")));
    let size = ui.spacing().interact_size;
    demo::base(reg, ui, "ui.add_sized", |ui| {
        ui.add_sized(size, egui::Label::new("ui.add_sized"))
    });
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    demo::base(reg, ui, "ui.place", |ui| {
        ui.place(rect, egui::Label::new("ui.place"))
    });
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    demo::base(reg, ui, "ui.put", |ui| {
        ui.put(rect, egui::Label::new("ui.put"))
    });
    demo::base(reg, ui, "ui.add_enabled", |ui| {
        ui.add_enabled(true, egui::Label::new("ui.add_enabled"))
    });
    demo::base(reg, ui, "ui.add_enabled_ui", |ui| {
        ui.add_enabled_ui(true, |ui| ui.label("ui.add_enabled_ui"))
            .inner
    });
    demo::base(reg, ui, "ui.add_visible", |ui| {
        ui.add_visible(true, egui::Label::new("ui.add_visible"))
    });
    ui.add_space(ui.spacing().item_spacing.y);
    demo::base(reg, ui, "ui.push_id", |ui| {
        ui.push_id("containers/push-id", |ui| ui.label("ui.push_id"))
            .inner
    });
    demo::base(reg, ui, "ui.scope", |ui| {
        ui.scope(|ui| ui.label("ui.scope")).inner
    });
    demo::base(reg, ui, "ui.scope_builder", |ui| {
        ui.scope_builder(egui::UiBuilder::new(), |ui| ui.label("ui.scope_builder"))
            .inner
    });
    demo::base(reg, ui, "ui.scope_dyn", |ui| {
        ui.scope_dyn(
            egui::UiBuilder::new(),
            Box::new(|ui: &mut egui::Ui| ui.label("ui.scope_dyn")),
        )
        .inner
    });
    demo::base(reg, ui, "ui.indent", |ui| {
        ui.indent("containers/indent", |ui| ui.label("ui.indent"))
            .inner
    });
    demo::base(reg, ui, "ui.horizontal", |ui| {
        ui.horizontal(|ui| ui.label("ui.horizontal")).inner
    });
    demo::base(reg, ui, "ui.horizontal_centered", |ui| {
        ui.horizontal_centered(|ui| ui.label("ui.horizontal_centered"))
            .inner
    });
    demo::base(reg, ui, "ui.horizontal_top", |ui| {
        ui.horizontal_top(|ui| ui.label("ui.horizontal_top")).inner
    });
    demo::base(reg, ui, "ui.horizontal_wrapped", |ui| {
        ui.horizontal_wrapped(|ui| ui.label("ui.horizontal_wrapped"))
            .inner
    });
    demo::base(reg, ui, "ui.vertical", |ui| {
        ui.vertical(|ui| ui.label("ui.vertical")).inner
    });
    demo::base(reg, ui, "ui.vertical_centered", |ui| {
        ui.vertical_centered(|ui| ui.label("ui.vertical_centered"))
            .inner
    });
    demo::base(reg, ui, "ui.vertical_centered_justified", |ui| {
        ui.vertical_centered_justified(|ui| ui.label("ui.vertical_centered_justified"))
            .inner
    });
    demo::base(reg, ui, "ui.with_layout", |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label("ui.with_layout")
        })
        .inner
    });
    ui.horizontal(|ui| {
        ui.set_row_height(ui.spacing().interact_size.y);
        demo::base(reg, ui, "ui.set_row_height", |ui| {
            ui.label("ui.set_row_height")
        });
    });
    egui::Grid::new("containers/end-row").show(ui, |ui| {
        demo::base(reg, ui, "ui.end_row", |ui| ui.label("ui.end_row"));
        ui.end_row();
        demo::base(reg, ui, "ui.end_row", |ui| ui.label("the next row"));
        ui.end_row();
    });
    demo::contained(reg, ui, "Resize (centered_and_justified)", |ui, reg| {
        egui::Resize::default()
            .id_salt("containers/centered")
            .show(ui, |ui| {
                demo::base(reg, ui, "ui.centered_and_justified", |ui| {
                    ui.centered_and_justified(|ui| ui.label("ui.centered_and_justified"))
                        .inner
                });
                ui.response()
            })
    });
}
