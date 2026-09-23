//! The Overlays page.

use gpui::{Context, IntoElement, ParentElement, Styled, Window, prelude::*};
use gpui_component::{IconName, Placement, WindowExt, h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, Overlay, SheetSide};
use crate::{
    OVERLAYS_DIALOG_TRIGGER, OVERLAYS_SHEET_BOTTOM, OVERLAYS_SHEET_BOTTOM_TITLE,
    OVERLAYS_SHEET_RIGHT, OVERLAYS_SHEET_RIGHT_TITLE, PROBE_ALERT_DIALOG, probe,
};

/// The menu rows the application draws itself, as `(id, icon, label)`: each
/// label names what its icon shows.
const MENU_ROWS: [(&str, IconName, &str); 3] = [
    ("overlays-menu-row-undo", IconName::Undo, "Undo"),
    ("overlays-menu-row-copy", IconName::Copy, "Copy"),
    ("overlays-menu-row-delete", IconName::Delete, "Delete"),
];

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Overlays
    // -----------------------------------------------------------------------
    pub(crate) fn render_overlays_page(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        let widget_gap = geometry::widget_gap(&self.layout);
        let container_margin = geometry::container_margin(&self.layout);
        let open_dialog = cx.listener(|this, _ev, window, cx| {
            let (ui, gap) = (this.info_ui.clone(), this.overlay_gap.clone());
            window.open_dialog(cx, move |dialog, _window, cx| {
                demo::confirm_dialog(&ui, cx, dialog, gap.get())
            });
        });
        let open_alert_dialog = cx.listener(|this, _ev, window, cx| {
            let (ui, this) = (this.info_ui.clone(), cx.weak_entity());
            window.open_alert_dialog(cx, move |alert, _window, cx| {
                let (ok, cancel) = (this.clone(), this.clone());
                demo::alert_dialog(&ui, cx, alert)
                    .on_ok(move |_ev, _window, cx| {
                        ok.update(cx, |this, cx| {
                            this.alert_choice = Some("Discard".into());
                            cx.notify();
                        })
                        .ok();
                        true
                    })
                    .on_cancel(move |_ev, _window, cx| {
                        cancel
                            .update(cx, |this, cx| {
                                this.alert_choice = Some("Keep".into());
                                cx.notify();
                            })
                            .ok();
                        true
                    })
            });
        });
        let open_sheet_right = cx.listener(|this, _ev, window, cx| {
            let ui = this.info_ui.clone();
            window.open_sheet(cx, move |sheet, _window, cx| {
                demo::sheet(
                    &ui,
                    cx,
                    sheet,
                    SheetSide::Right,
                    OVERLAYS_SHEET_RIGHT_TITLE,
                    "Sheet Panel",
                )
            });
        });
        let open_sheet_bottom = cx.listener(|this, _ev, window, cx| {
            let ui = this.info_ui.clone();
            window.open_sheet_at(Placement::Bottom, cx, move |sheet, _window, cx| {
                demo::sheet(
                    &ui,
                    cx,
                    sheet,
                    SheetSide::Bottom,
                    OVERLAYS_SHEET_BOTTOM_TITLE,
                    "Bottom Sheet",
                )
            });
        });
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Dialog
            .child(demo::heading(ui, cx, "overlays-heading-dialog", "Dialog"))
            .child(
                demo::overlay_button(
                    ui,
                    cx,
                    OVERLAYS_DIALOG_TRIGGER,
                    "Open Dialog",
                    Overlay::Dialog,
                    open_dialog,
                )
                .self_start(),
            )
            // AlertDialog
            .child(demo::heading(
                ui,
                cx,
                "overlays-heading-alert-dialog",
                match &self.alert_choice {
                    Some(choice) => format!("AlertDialog (last answered: {choice})"),
                    None => "AlertDialog (not answered yet)".to_string(),
                },
            ))
            .child(
                probe(
                    PROBE_ALERT_DIALOG,
                    demo::overlay_button(
                        ui,
                        cx,
                        "overlays-alert-dialog-trigger",
                        "Discard changes…",
                        Overlay::AlertDialog,
                        open_alert_dialog,
                    ),
                )
                .self_start(),
            )
            // Sheet
            .child(demo::heading(
                ui,
                cx,
                "overlays-heading-sheet",
                "Sheet (slide-in panel)",
            ))
            .child(
                h_flex()
                    .gap_3()
                    .child(demo::overlay_button(
                        ui,
                        cx,
                        OVERLAYS_SHEET_RIGHT,
                        "Open Sheet (Right)",
                        Overlay::Sheet(SheetSide::Right),
                        open_sheet_right,
                    ))
                    .child(demo::overlay_button(
                        ui,
                        cx,
                        OVERLAYS_SHEET_BOTTOM,
                        "Open Sheet (Bottom)",
                        Overlay::Sheet(SheetSide::Bottom),
                        open_sheet_bottom,
                    )),
            )
            // Popover
            .child(demo::heading(ui, cx, "overlays-heading-popover", "Popover"))
            .child(demo::popover(ui, cx, "overlays-popover", widget_gap).self_start())
            // HoverCard
            .child(demo::heading(
                ui,
                cx,
                "overlays-heading-hover-card",
                "HoverCard (hover the trigger, no click)",
            ))
            .child(
                demo::hover_card(ui, cx, "overlays-hover-card", container_margin, widget_gap)
                    .self_start(),
            )
            // ContextMenu
            .child(demo::heading(
                ui,
                cx,
                "overlays-heading-context-menu",
                "ContextMenu (right-click the area below)",
            ))
            .child(demo::context_menu_area(ui, cx, "overlays-context-menu"))
            // DropdownMenu
            .child(demo::heading(
                ui,
                cx,
                "overlays-heading-dropdown-menu",
                "DropdownMenu",
            ))
            .child(
                demo::dropdown_menu(ui, cx, "overlays-dropdown-menu", "Click for Menu")
                    .self_start(),
            )
            // The menu rows an application draws itself.
            .child(demo::heading(
                ui,
                cx,
                "overlays-heading-menu-rows",
                "Menu rows (drawn by the application)",
            ))
            .child(demo::menu_rows(ui, cx, &MENU_ROWS))
    }
}
