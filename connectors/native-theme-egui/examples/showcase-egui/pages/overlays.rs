//! Overlays (spec §10.4's palette table): `Window`, `Modal`, `Popup`, `Tooltip`, `Area` and a
//! page `MenuBar`, each container given its surface's frame and its role's modifier as values
//! (§1.5, §3.2), its body styled as the first statement inside.

use egui::Button;
use egui::containers::menu::SubMenuButton;
use native_theme::icons::IconRole;
use native_theme::icons::IconSet;
use native_theme::next::{DialogButtonOrder, IconStated, ResolvedTheme, WidgetState};
use native_theme_egui::next::{
    dialog_button_order, window_title_bar_font, window_title_bar_text_color,
};
use native_theme_egui::{Role, RoleVariant, Surface};

use super::{DemoState, button_min_size, caption, inner_extent};
use crate::app::OldAtlas;
use crate::demo::{self, Registry};

/// The lines the window scrolls: a datum.
const WINDOW_LINES: usize = 30;

/// The modal's two buttons in `order`, the platform's (`dialog.button_order`), and its message;
/// where the theme states no order, egui having none of its own, the buttons in the order the
/// page writes them, and the message says so.
pub(crate) fn modal_buttons(order: Option<DialogButtonOrder>) -> ([&'static str; 2], &'static str) {
    match order {
        Some(DialogButtonOrder::PrimaryRight) => (
            ["Cancel", "OK"],
            "A modal dialog, its two buttons in the platform's order.",
        ),
        Some(DialogButtonOrder::PrimaryLeft) => (
            ["OK", "Cancel"],
            "A modal dialog, its two buttons in the platform's order.",
        ),
        None => (
            ["OK", "Cancel"],
            "A modal dialog; the theme states no dialog.button_order, so its two buttons \
             stand in the order the page writes them.",
        ),
    }
}

/// The size of a menu's item icon, `menu.icon(NONE).size` of the installed 0.8.0 theme `r`;
/// `None` where it states none, and the menu's icons are then not drawn and their button says
/// so.
pub(crate) fn menu_icon_size(r: Option<&ResolvedTheme>) -> Option<f32> {
    let icon = r?.menu.icon(WidgetState::NONE);
    icon.stated.contains(IconStated::SIZE).then_some(icon.size)
}

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &OldAtlas,
    ui: &mut egui::Ui,
    chosen: &(IconSet, Option<String>),
) {
    let r = atlas.next_shown(ui.ctx());
    let normal = RoleVariant::Normal;
    let min = button_min_size(r);

    caption(
        reg,
        ui,
        "Window (Surface::Window, Surface::WindowTitleBar, body Role::Window)",
    );
    let open = demo::scoped(reg, ui, Role::Button, normal, "open window button", |ui| {
        ui.add(Button::new("Open window").min_size(min))
    });
    if open.clicked() {
        state.window_open = true;
    }
    // `window.title(normal)`'s size and colour, each egui's own (`TextStyle::Heading`, the
    // window's text colour) where the theme states it not
    let mut title = egui::RichText::new("A window");
    if let Some(font) = r.and_then(|r| window_title_bar_font(r).ok()) {
        title = title.font(font);
    }
    if let Some(color) = r.and_then(|r| window_title_bar_text_color(r).ok()) {
        title = title.color(color);
    }
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
                .id(egui::Id::unique("overlays/window"))
                .open(window_open)
                .frame(chrome.frame)
                // At the page's right edge, clear of the page's own controls, which sit at its left.
                .pivot(egui::Align2::RIGHT_TOP)
                .default_pos(ui.max_rect().right_top())
                .collapsible(true)
                .resizable(true)
                .vscroll(true);
            if let Some(title_frame) = chrome.title_frame {
                window = window.title_frame(title_frame);
            }
            window
                .show(ui.ctx(), |ui| {
                    let body = demo::styled(reg, ui, Role::Window, normal, "window body");
                    for line in 0..WINDOW_LINES {
                        body.add(reg, ui, "window line", |ui| {
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
        ui.add(Button::new("Open modal").min_size(min))
    });
    if open.clicked() {
        state.modal_open = true;
    }
    if state.modal_open {
        let (labels, message) = modal_buttons(r.and_then(|r| dialog_button_order(r).ok()));
        let ([min_w, min_h], [max_w, max_h]) = r.map_or(([None; 2], [None; 2]), |r| {
            (r.dialog.panel.min_size, r.dialog.panel.max_size)
        });
        let modal_open = &mut state.modal_open;
        demo::surfaced(
            reg,
            ui,
            Surface::Dialog,
            false,
            None,
            "modal",
            |ui, chrome, reg| {
                // The four extents egui's `Modal` never reads, per call, as the command palette
                // takes them (`src/mapping.md`: DERIVED), `dialog.panel.min_size` and
                // `max_size`: each the frame's outer size, less its margins and stroke; one the
                // theme states not, egui's own, none.
                let inner = |stated: Option<f32>, horizontal| {
                    stated.and_then(|s| inner_extent(s, &chrome.frame, horizontal))
                };
                let (min_w, max_w) = (inner(min_w, true), inner(max_w, true));
                let (min_h, max_h) = (inner(min_h, false), inner(max_h, false));
                let out = egui::Modal::new(egui::Id::unique("overlays/modal"))
                    .frame(chrome.frame)
                    .show(ui.ctx(), |ui| {
                        let body = demo::styled(reg, ui, Role::Dialog, normal, "modal body");
                        if let Some(w) = min_w {
                            ui.set_min_width(w);
                        }
                        if let Some(w) = max_w {
                            ui.set_max_width(w);
                        }
                        if let Some(h) = min_h {
                            ui.set_min_height(h);
                        }
                        if let Some(h) = max_h {
                            ui.set_max_height(h);
                        }
                        body.add(reg, ui, "modal message", |ui| ui.label(message));
                        let mut clicked = false;
                        ui.horizontal(|ui| {
                            for label in labels {
                                clicked |= demo::scoped(
                                    reg,
                                    ui,
                                    Role::Button,
                                    normal,
                                    "modal button",
                                    |ui| ui.add(Button::new(label).min_size(min)),
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
            ui.add(Button::new("Open popup").min_size(min))
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
                            ui.add(Button::new("A button in the popup").min_size(min))
                        });
                    })
                    .map(|out| out.response)
            },
        );
        // The target is a label on the page, in the base style; only the menu it opens takes the
        // menu role's modifier.
        let target = demo::base(reg, ui, "context menu target", |ui| {
            ui.label("Right-click here for a context menu")
                .interact(egui::Sense::click())
        });
        let menu = (Role::Menu, normal);
        demo::context_menu(reg, ui, &target, menu, "context menu (open)", |ui, reg| {
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
    });

    caption(
        reg,
        ui,
        "Tooltip::for_enabled (Surface::Tooltip, Role::Tooltip), beside Response::on_hover_text",
    );
    ui.horizontal(|ui| {
        let owner = demo::scoped(reg, ui, Role::Button, normal, "tooltip button", |ui| {
            ui.add(Button::new("Tooltip").min_size(min))
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
    // An `Area` clips to its constrain rect, the whole window unless given one
    // (`egui/src/containers/area.rs:439`, `:630`): the page's visible part here, so as the page
    // scrolls the Area follows its anchor out of view instead of over the chrome; not
    // `constrain`ed, which would move it back into that rect, away from its anchor.
    let page = ui.clip_rect();
    demo::contained(reg, ui, "Area", |ui, reg| {
        egui::Area::new(egui::Id::unique("overlays/area"))
            .constrain_to(page)
            .constrain(false)
            .fixed_pos(anchor.rect.right_top())
            .show(ui.ctx(), |ui| {
                demo::base(reg, ui, "area label", |ui| ui.label("Area"));
            })
            .response
    });

    caption(reg, ui, "Menus in a page MenuBar (Role::Menu)");
    let icon_size = menu_icon_size(r);
    let (set, icon_theme) = chosen;
    demo::menu_bar(
        reg,
        ui,
        Role::Menu,
        normal,
        "page menu bar",
        |ui, bar, menu, reg| {
            bar.ui(ui, |ui| {
                let r = ui.menu_button("Page menu", |ui| {
                    let open = demo::styled(reg, ui, Role::Menu, normal, "page menu (open)");
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
                    open.record(reg, &sub, "SubMenuButton");
                    reg.note_last(
                        "the arrow",
                        "egui's triangle, in the button's text colour, in a square \
                         spacing.icon_width on a side, so checkbox.indicator_width \
                         (egui-0.37.0 src/containers/menu.rs:492-505, \
                         SubMenuButton::arrow_atom); menu.icon(NONE).size does not size it",
                    );
                });
                menu.record(reg, &r.response, "menu button");

                let image = icon_size.map(|size| {
                    demo::role_image(ui, IconRole::ActionEdit, *set, icon_theme.as_deref(), size)
                });
                let r = match image {
                    Some(Some(image)) => ui.menu_button((image, "With icon"), |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "icon menu (open)");
                        demo::scoped(reg, ui, Role::Menu, normal, "menu item", |ui| {
                            ui.button("An item")
                        });
                    }),
                    Some(None) => ui.menu_button("With icon (not in this set)", |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "icon menu (open)");
                    }),
                    None => ui.menu_button("With icon (no menu.icon(NONE).size stated)", |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "icon menu (open)");
                    }),
                };
                menu.record(reg, &r.response, "menu button (image and text)");

                let image = icon_size.and_then(|size| {
                    demo::role_image(
                        ui,
                        IconRole::ActionSettings,
                        *set,
                        icon_theme.as_deref(),
                        size,
                    )
                });
                if let Some(image) = image {
                    let r = ui.menu_image_button(image, |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "image menu (open)");
                        demo::scoped(reg, ui, Role::Menu, normal, "menu item", |ui| {
                            ui.button("An item")
                        });
                    });
                    menu.record(reg, &r.response, "ui.menu_image_button");
                }
                let image = icon_size.and_then(|size| {
                    demo::role_image(
                        ui,
                        IconRole::ActionSearch,
                        *set,
                        icon_theme.as_deref(),
                        size,
                    )
                });
                if let Some(image) = image {
                    let r = ui.menu_image_text_button(image, "Image and text", |ui| {
                        demo::styled(reg, ui, Role::Menu, normal, "image text menu (open)");
                        demo::scoped(reg, ui, Role::Menu, normal, "menu item", |ui| {
                            ui.button("An item")
                        });
                    });
                    menu.record(reg, &r.response, "ui.menu_image_text_button");
                }
            })
            .response
        },
    );
}
