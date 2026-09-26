//! Overlays (spec §10.4's palette table): `Window`, `Modal`, `Popup`, `Tooltip`, `Area` and a
//! page `MenuBar`, each container given its surface's frame and its role's modifier as values
//! (§1.5, §3.2), its body styled as the first statement inside.

use egui::containers::menu::{MenuConfig, SubMenuButton};
use native_theme::theme::{IconRole, IconSet};
use native_theme_egui::{
    DialogButtonOrder, Role, RoleVariant, Surface, ThemeAtlas, dialog_button_order,
    window_title_bar_font, window_title_bar_text_color,
};

use super::{DemoState, caption};
use crate::demo::{self, Registry, Seam};

/// The lines the window scrolls: a datum.
const WINDOW_LINES: usize = 30;

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    caption(
        reg,
        ui,
        "Window (Surface::Window, Surface::WindowTitleBar, body Role::Window)",
    );
    let open = demo::scoped(reg, ui, Role::Button, normal, "open window button", |ui| {
        ui.button("Open window")
    });
    if open.clicked() {
        state.window_open = true;
    }
    let title = egui::RichText::new("A window")
        .font(window_title_bar_font(t, atlas.accessibility()))
        .color(window_title_bar_text_color(t, true));
    let window_open = &mut state.window_open;
    demo::surfaced(
        reg,
        ui,
        Surface::Window,
        true,
        None,
        "window",
        |ui, chrome, reg| {
            let mut window = egui::Window::new(title)
                .id(egui::Id::new("overlays/window"))
                .open(window_open)
                .frame(chrome.frame)
                .default_pos(open.rect.left_bottom())
                .collapsible(true)
                .resizable(true)
                .vscroll(true);
            if let Some(title_frame) = chrome.title_frame {
                window = window.title_frame(title_frame);
            }
            window
                .show(ui.ctx(), |ui| {
                    demo::styled(reg, ui, Role::Window, normal, "window body");
                    for line in 0..WINDOW_LINES {
                        demo::base(reg, ui, "window line", |ui| {
                            ui.label(format!("A line of the window's body: {line}"))
                        });
                    }
                })
                .map(|out| out.response)
        },
    );

    caption(
        reg,
        ui,
        "Modal (Surface::Dialog, body Role::Dialog, buttons in dialog_button_order)",
    );
    let open = demo::scoped(reg, ui, Role::Button, normal, "open modal button", |ui| {
        ui.button("Open modal")
    });
    if open.clicked() {
        state.modal_open = true;
    }
    if state.modal_open {
        let order = dialog_button_order(t);
        let modal_open = &mut state.modal_open;
        demo::surfaced(
            reg,
            ui,
            Surface::Dialog,
            false,
            None,
            "modal",
            |ui, chrome, reg| {
                let out = egui::Modal::new(egui::Id::new("overlays/modal"))
                    .frame(chrome.frame)
                    .show(ui.ctx(), |ui| {
                        demo::styled(reg, ui, Role::Dialog, normal, "modal body");
                        demo::base(reg, ui, "modal message", |ui| {
                            ui.label("A modal dialog, its two buttons in the platform's order.")
                        });
                        let labels = match order {
                            DialogButtonOrder::PrimaryRight => ["Cancel", "OK"],
                            DialogButtonOrder::PrimaryLeft => ["OK", "Cancel"],
                        };
                        let mut clicked = false;
                        ui.horizontal(|ui| {
                            for label in labels {
                                clicked |= demo::scoped(
                                    reg,
                                    ui,
                                    Role::Button,
                                    normal,
                                    "modal button",
                                    |ui| ui.button(label),
                                )
                                .clicked();
                            }
                        });
                        clicked
                    });
                if out.inner || out.should_close() {
                    *modal_open = false;
                }
                Some(out.response)
            },
        );
    }

    caption(
        reg,
        ui,
        "Popup from a toggle button (Surface::Popover, Role::Popover), and a context menu (Role::Menu)",
    );
    ui.horizontal(|ui| {
        let toggle = demo::scoped(reg, ui, Role::Button, normal, "open popup button", |ui| {
            ui.button("Open popup")
        });
        demo::surfaced(
            reg,
            ui,
            Surface::Popover,
            false,
            Some((Role::Popover, normal)),
            "popup",
            |_, chrome, reg| {
                let mut popup =
                    egui::Popup::from_toggle_button_response(&toggle).frame(chrome.frame);
                if let Some(modifier) = chrome.modifier {
                    popup = popup.style(modifier);
                }
                popup
                    .show(|ui| {
                        demo::scoped(reg, ui, Role::Button, normal, "popup item", |ui| {
                            ui.button("A button in the popup")
                        });
                    })
                    .map(|out| out.response)
            },
        );
        demo::modifier(
            reg,
            ui,
            Role::Menu,
            normal,
            "context menu",
            |ui, modifier, reg| {
                let target = ui
                    .label("Right-click here for a context menu")
                    .interact(egui::Sense::click());
                let mut popup = egui::Popup::context_menu(&target);
                if let Some(modifier) = modifier {
                    popup = popup.style(modifier);
                }
                popup.show(|ui| {
                    demo::styled(reg, ui, Role::Menu, normal, "context menu (open)");
                    demo::scoped(reg, ui, Role::Menu, normal, "context menu item", |ui| {
                        ui.button("Copy")
                    });
                    demo::scoped(
                        reg,
                        ui,
                        Role::Menu,
                        RoleVariant::Disabled,
                        "context menu item (disabled)",
                        |ui| ui.add_enabled(false, egui::Button::new("Paste")),
                    );
                });
                target
            },
        );
    });

    caption(
        reg,
        ui,
        "Tooltip::for_enabled (Surface::Tooltip, Role::Tooltip), beside Response::on_hover_text",
    );
    ui.horizontal(|ui| {
        let owner = demo::scoped(reg, ui, Role::Button, normal, "tooltip button", |ui| {
            ui.button("Tooltip")
        });
        demo::surfaced(
            reg,
            ui,
            Surface::Tooltip,
            false,
            Some((Role::Tooltip, normal)),
            "tooltip",
            |_, chrome, reg| {
                let mut tip = egui::Tooltip::for_enabled(&owner);
                tip.popup = tip.popup.frame(chrome.frame);
                if let Some(modifier) = chrome.modifier {
                    tip.popup = tip.popup.style(modifier);
                }
                tip.show(|ui| {
                    demo::scoped(reg, ui, Role::Tooltip, normal, "tooltip text", |ui| {
                        ui.label("Tooltip::for_enabled, in the tooltip surface")
                    });
                })
                .map(|out| out.response)
            },
        );
        demo::base(reg, ui, "on_hover_text owner", |ui| {
            ui.button("on_hover_text")
                .on_hover_text("Response::on_hover_text: egui's own tooltip frame")
        });
    });

    caption(reg, ui, "Area (the base style)");
    let anchor = demo::base(reg, ui, "area anchor", |ui| {
        ui.label("An Area floats beside this label:")
    });
    demo::contained(reg, ui, "Area", |ui, reg| {
        egui::Area::new(egui::Id::new("overlays/area"))
            .fixed_pos(anchor.rect.right_top())
            .show(ui.ctx(), |ui| {
                demo::base(reg, ui, "area label", |ui| ui.label("Area"));
            })
            .response
    });

    caption(reg, ui, "Menus in a page MenuBar (Role::Menu)");
    let menu_icon_size = t.menu.icon_size;
    let (set, icon_theme) = chosen;
    demo::modifier(
        reg,
        ui,
        Role::Menu,
        normal,
        "page menu bar",
        |ui, modifier, reg| {
            let mut bar = egui::MenuBar::new();
            if let Some(modifier) = modifier {
                bar = bar
                    .style(modifier.clone())
                    .config(MenuConfig::new().style(modifier));
            }
            bar.ui(ui, |ui| {
                let r = ui.menu_button("Page menu", |ui| {
                    demo::styled(reg, ui, Role::Menu, normal, "page menu (open)");
                    demo::scoped(reg, ui, Role::Menu, normal, "menu item", |ui| {
                        ui.button("An item")
                    });
                    demo::scoped(
                        reg,
                        ui,
                        Role::Menu,
                        RoleVariant::Disabled,
                        "menu item (disabled)",
                        |ui| ui.add_enabled(false, egui::Button::new("A disabled item")),
                    );
                    let (sub, _) = SubMenuButton::new("A submenu").ui(ui, |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "submenu (open)");
                        demo::scoped(reg, ui, Role::Menu, normal, "submenu item", |ui| {
                            ui.button("A submenu item")
                        });
                    });
                    reg.record(
                        &sub,
                        demo::info("SubMenuButton", vec![Seam::Role(Role::Menu, normal)]),
                        false,
                    );
                });
                reg.record(
                    &r.response,
                    demo::info("menu button", vec![Seam::Role(Role::Menu, normal)]),
                    false,
                );

                let image = demo::role_image(
                    ui,
                    IconRole::ActionEdit,
                    *set,
                    icon_theme.as_deref(),
                    menu_icon_size,
                );
                let r = match image {
                    Some(image) => ui.menu_button((image, "With icon"), |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "icon menu (open)");
                        demo::scoped(reg, ui, Role::Menu, normal, "menu item", |ui| {
                            ui.button("An item")
                        });
                    }),
                    None => ui.menu_button("With icon (not in this set)", |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "icon menu (open)");
                    }),
                };
                reg.record(
                    &r.response,
                    demo::info(
                        "menu button (image and text)",
                        vec![Seam::Role(Role::Menu, normal)],
                    ),
                    false,
                );
                reg.amend_last(|i| i.read.push(("menu.icon_size", format!("{menu_icon_size}"))));

                let image = demo::role_image(
                    ui,
                    IconRole::ActionSettings,
                    *set,
                    icon_theme.as_deref(),
                    menu_icon_size,
                );
                if let Some(image) = image {
                    let r = ui.menu_image_button(image, |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "image menu (open)");
                        demo::scoped(reg, ui, Role::Menu, normal, "menu item", |ui| {
                            ui.button("An item")
                        });
                    });
                    reg.record(
                        &r.response,
                        demo::info("ui.menu_image_button", vec![Seam::Role(Role::Menu, normal)]),
                        false,
                    );
                }
                let image = demo::role_image(
                    ui,
                    IconRole::ActionSearch,
                    *set,
                    icon_theme.as_deref(),
                    menu_icon_size,
                );
                if let Some(image) = image {
                    let r = ui.menu_image_text_button(image, "Image and text", |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "image text menu (open)");
                        demo::scoped(reg, ui, Role::Menu, normal, "menu item", |ui| {
                            ui.button("An item")
                        });
                    });
                    reg.record(
                        &r.response,
                        demo::info(
                            "ui.menu_image_text_button",
                            vec![Seam::Role(Role::Menu, normal)],
                        ),
                        false,
                    );
                }
            })
            .response
        },
    );
}
