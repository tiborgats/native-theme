//! The Overlays tab.

use gpui::{Context, IntoElement, ParentElement, Styled, Window, div, prelude::*, px};
use gpui_component::{
    ActiveTheme, IconName, Placement, StyledExt, WindowExt,
    button::{Button, ButtonVariant, ButtonVariants, DropdownButton},
    command::{Command, CommandGroup, CommandItem},
    dialog::{
        AlertDialog, DialogButtonProps, DialogClose, DialogDescription, DialogFooter, DialogTitle,
    },
    h_flex,
    hover_card::HoverCard,
    label::Label,
    menu::ContextMenuExt,
    popover::Popover,
    v_flex,
};

use native_theme_gpui::{ActiveNativeTheme, geometry, variants};

use crate::app::Showcase;
use crate::support::{
    NativeStyled, format_font_info, native_geometry, native_icon, refined, section, with_gap,
    with_padding,
};
use crate::{PROBE_ALERT_DIALOG, probe};

impl Showcase {
    // -----------------------------------------------------------------------
    // Tab: Overlays
    // -----------------------------------------------------------------------
    pub(crate) fn render_overlays_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let widget_gap = geometry::widget_gap(&self.layout);
        let container_margin = geometry::container_margin(&self.layout);
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // AppMenuBar
            .child(section("AppMenuBar (File / View / Theme / Help)"))
            .child(
                div()
                    .id("tt-app-menu-bar")
                    .w_full()
                    .demo_frame(cx)
                    .child(self.app_menu_bar.clone())
                    .on_hover(self.hover_info(&fi, "AppMenuBar", &[("item text", "secondary_foreground", t.secondary_foreground, "gpui-component/button/button.rs:964"), ("item hover", "accent", t.accent, "gpui-component/button/button.rs:1126"), ("menu bg", "popover", t.popover, "gpui-component/styled.rs:197")], &[], &[
                            ("fill", "none: an AppMenuBar reads no theme field at all (menu/app_menu_bar.rs) and paints no bar background -- the panel had claimed tab_bar, which nothing here touches"),
                            ("items", "ghost Buttons, so they hover with accent rather than the button family, and their label is the Ghost variant's secondary_foreground"),
                            ("source", "gpui-base's GlobalState app menus, which only set_app_menus fills -- not gpui's cx.set_menus, which feeds the platform's own menu bar. This showcase gives both the same menus (menu/app_menu_bar.rs, AppMenuBar::reload)"),
                        ])),
            )
            // Dialog
            .child(section("Dialog"))
            .child(
                div()
                    .id("tt-dialog")
                    .child(
                        Button::new("open-dialog")
                            .label("Open Dialog")
                            .on_click(cx.listener(|this, _ev, window, cx| {
                                let widget_gap = geometry::widget_gap(&this.layout);
                                window.open_dialog(cx, move |dialog, _w, cx| {
                                    let n = cx.native_theme().and_then(|t| t.native(cx));
                                    let dialog =
                                        dialog
                                            .title(match n {
                                                Some(n) => DialogTitle::new()
                                                    .refine_style(&geometry::dialog_title(n))
                                                    .child("Confirm Action"),
                                                None => DialogTitle::new().child("Confirm Action"),
                                            })
                                            .w(px(400.0))
                                            // The description is where the
                                            // platform's dialog body font and
                                            // its dialog icon size land;
                                            // upstream would paint the text
                                            // with `muted_foreground`
                                            // (`dialog/description.rs:50-51`).
                                            .content(move |content, _w, cx| {
                                                content.child(
                                                    with_gap(h_flex(), widget_gap)
                                                        .items_start()
                                                        .child(native_icon(
                                                            cx,
                                                            IconName::CircleX,
                                                            geometry::icon_size_dialog,
                                                        ))
                                                        .child(refined(
                                                            DialogDescription::new()
                                                                .child("This cannot be undone."),
                                                            native_geometry(
                                                                cx,
                                                                geometry::dialog_description,
                                                            )
                                                            .as_ref(),
                                                        )),
                                                )
                                            })
                                            .footer(
                                                match n {
                                                    Some(n) => DialogFooter::new()
                                                        .refine_style(&geometry::dialog_footer(n)),
                                                    None => DialogFooter::new(),
                                                }
                                                .child(DialogClose::new().child(
                                                    Button::new("dialog-close").label("Close"),
                                                )),
                                            );
                                    match n {
                                        Some(n) => dialog
                                            .refine_style(&geometry::dialog(n))
                                            .max_w(geometry::dialog_max_width(n)),
                                        None => dialog,
                                    }
                                });
                            })),
                    )
                    .on_hover(self.hover_info(&fi, "Dialog", &[("bg", "background", t.background, "gpui-component/dialog/dialog.rs:613"), ("overlay", "overlay", t.overlay, "gpui-component/dialog/dialog.rs:282"), ("border", "border", t.border, "gpui-component/dialog/dialog.rs:615")], &[("geometry", "geometry::dialog on the surface: dialog.border.padding_*, min_height, max_height and border.corner_radius; geometry::dialog_max_width caps the width at dialog.max_width (dialog/dialog.rs, Dialog::max_w)".to_string()), ("title and body", "geometry::dialog_title: dialog.title_font; geometry::dialog_description: dialog.body_font including its colour, which upstream would otherwise paint with muted_foreground (dialog/description.rs, DialogDescription::render)".to_string()), ("footer", "geometry::dialog_footer: dialog.button_gap between the buttons (dialog/footer.rs, DialogFooter::render)".to_string()), ("icon size", "geometry::icon_size_dialog: defaults.icon_sizes.dialog".to_string())], &[
                            ("fill", "the window's own background, not the popover colour: a dialog is a surface, not a popup, and reads background (dialog/dialog.rs, Dialog::render)"),
                            ("text", "none set: the body inherits the window's text colour, and only the description is recoloured (dialog/description.rs, DialogDescription::render)"),
                            ("corner radius", "dialog.border.corner_radius, through geometry::dialog: upstream sets radius_lg, the larger of the theme's two radii, and the refinement replaces it (dialog/dialog.rs, Dialog::render)"),
                            ("animation", "a file-level duration and a literal curve, not the theme's motion tokens the Accordion and Switch read (dialog/dialog.rs, Dialog)"),
                        ])),
            )
            // AlertDialog
            .child(section(match &self.alert_choice {
                Some(choice) => format!("AlertDialog (last answered: {choice})"),
                None => "AlertDialog (not answered yet)".to_string(),
            }))
            .child(
                div()
                    .id("tt-alert-dialog")
                    .child(probe(
                        PROBE_ALERT_DIALOG,
                        Button::new("open-alert-dialog")
                            .native(cx, geometry::button)
                            .danger()
                            .label("Discard changes…")
                            .on_click(cx.listener(|_this, _ev, window, cx| {
                                let this = cx.weak_entity();
                                window.open_alert_dialog(cx, move |alert: AlertDialog, _w, cx| {
                                    let n = cx.native_theme().and_then(|t| t.native(cx));
                                    let ok = this.clone();
                                    let cancel = this.clone();
                                    let alert = alert
                                        .icon(native_icon(
                                            cx,
                                            IconName::TriangleAlert,
                                            geometry::icon_size_dialog,
                                        ))
                                        .title("Discard changes?")
                                        .description(
                                            "The edits made since the last save will be lost.",
                                        )
                                        .button_props(
                                            DialogButtonProps::default()
                                                .ok_text("Discard")
                                                .ok_variant(ButtonVariant::Danger)
                                                .cancel_text("Keep")
                                                .show_cancel(true),
                                        )
                                        .on_ok(move |_ev, _w, cx| {
                                            ok.update(cx, |this, cx| {
                                                this.alert_choice = Some("Discard".into());
                                                cx.notify();
                                            })
                                            .ok();
                                            true
                                        })
                                        .on_cancel(move |_ev, _w, cx| {
                                            cancel
                                                .update(cx, |this, cx| {
                                                    this.alert_choice = Some("Keep".into());
                                                    cx.notify();
                                                })
                                                .ok();
                                            true
                                        });
                                    match n {
                                        Some(n) => alert
                                            .refine_style(&geometry::dialog(n))
                                            .max_w(geometry::dialog_max_width(n)),
                                        None => alert,
                                    }
                                });
                            })),
                    ))
                    .on_hover(self.hover_info(&fi, "AlertDialog", &[("bg", "background", t.background, "gpui-component/dialog/dialog.rs:613"), ("overlay", "overlay", t.overlay, "gpui-component/dialog/dialog.rs:282"), ("border", "border", t.border, "gpui-component/dialog/dialog.rs:615"), ("description", "muted_foreground", t.muted_foreground, "gpui-component/dialog/description.rs:50"), ("confirm button", "button_danger", t.button_danger, "gpui-component/button/button.rs:938")], &[("geometry", "geometry::dialog and geometry::dialog_max_width, as the Dialog above".to_string()), ("icon size", "geometry::icon_size_dialog: defaults.icon_sizes.dialog".to_string())], &[
                            ("surface", "an AlertDialog reads no theme field of its own (dialog/alert_dialog.rs): it is a Dialog, so its fill is background and not the popover colour the panel had claimed"),
                            ("footer", "right-aligned, and built from button_props when none is given (dialog/alert_dialog.rs, AlertDialog::build_surface; dialog/footer.rs, DialogFooter::render justify_end)"),
                            ("dismissal", "no backdrop close by design (dialog/alert_dialog.rs, AlertDialog::overlay_closable, deprecated)"),
                        ])),
            )
            // Sheet
            .child(section("Sheet (slide-in panel)"))
            .child(
                div()
                    .id("tt-sheet")
                    .child(
                        h_flex()
                            .gap_3()
                            .child(
                                Button::new("open-sheet-right")
                                    .native(cx, geometry::button)
                                    .label("Open Sheet (Right)")
                                    .on_click(cx.listener(|_this, _ev, window, cx| {
                                        window.open_sheet(cx, |sheet, _w, _cx| {
                                            sheet.title("Sheet Panel").size(px(320.0))
                                        });
                                    })),
                            )
                            .child(
                                Button::new("open-sheet-bottom")
                                    .native(cx, geometry::button)
                                    .label("Open Sheet (Bottom)")
                                    .on_click(cx.listener(|_this, _ev, window, cx| {
                                        window.open_sheet_at(
                                            Placement::Bottom,
                                            cx,
                                            |sheet, _w, _cx| {
                                                sheet.title("Bottom Sheet").size(px(200.0))
                                            },
                                        );
                                    })),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Sheet", &[("bg", "background", t.background, "gpui-component/sheet.rs:172"), ("overlay", "overlay", t.overlay, "gpui-component/dialog/dialog.rs:282"), ("border", "border", t.border, "gpui-component/sheet.rs:173")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("fill", "the window's own background, not the popover colour, as Dialog above (sheet.rs, Sheet::render)"),
                            ("text", "none set: the panel inherits the window's text colour"),
                            ("overlay", "the same overlay_color a Dialog uses -- sheet.rs imports it from dialog (sheet.rs, use dialog::overlay_color)"),
                            ("top margin", "Theme::sheet.margin_top, a gpui-component setting the connector leaves at its default of TITLE_BAR_HEIGHT, 34px, so the sheet clears a client-side title bar (sheet.rs, SheetSettings). native-theme states no sheet"),
                            ("animation", "a 0.15s literal, not the theme's motion tokens (sheet.rs, Sheet)"),
                            ("placement", "Right / Bottom / Left / Top"),
                        ])),
            )
            // Popover
            .child(section("Popover"))
            .child(
                div()
                    .id("tt-popover")
                    .child(
                        Popover::new("popover-1")
                            .native(cx, geometry::popover)
                            .trigger(
                                Button::new("popover-trigger")
                                    .native(cx, geometry::button)
                                    .label("Click for Popover"),
                            )
                            .content(|_state, _w, cx| {
                                v_flex()
                                    .p_4()
                                    .gap_2()
                                    .w(px(200.0))
                                    .child(Label::new("Popover Content").font_semibold())
                                    .child(
                                        Label::new("This is a popover panel.")
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground),
                                    )
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Popover",
                        &[
                            ("bg", "popover", t.popover, "gpui-component/styled.rs:197"),
                            ("text", "popover_foreground", t.popover_foreground, "gpui-component/styled.rs:198"),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32())), ("geometry", "geometry::popover on the panel: popover.border.padding_* and corner_radius (popover.rs, Popover::render refine_style). The trigger is a Button, shaped by geometry::button".to_string()), ("corner radius", "set by popover_style from the theme radius, before geometry::popover refines it (gpui-component styled.rs, popover_style)".to_string())],
                        &[
                            ("edge", "no border token: the surface is painted by popover_style, whose edge is a 1px shadow ring of foreground at low alpha. That is how the shadow shows through it, and it is why popover.border.color has no receiver here (gpui-component styled.rs, popover_style and popover_ring)"),
                            ("trigger", "any Selectable element"),
                            ("anchor", "Anchor::TopLeft unless Popover::anchor picks another of its eight points (popover.rs, Popover::anchor)"),
                        ],
                    )),
            )
            // HoverCard
            .child(section("HoverCard (hover the trigger, no click)"))
            .child(
                div()
                    .id("tt-hover-card")
                    .child(
                        HoverCard::new("hover-card-1")
                            .native(cx, geometry::popover)
                            .trigger(
                                Button::new("hover-card-trigger")
                                    .native(cx, geometry::button)
                                    .label("KDE Breeze")
                                    .custom(variants::ghost_button(cx)),
                            )
                            // The card's own padding and the gap inside it are
                            // the application's to set, so they take the
                            // platform's container margin and widget gap.
                            .content(move |_state, _w, cx| {
                                with_padding(
                                    with_gap(v_flex(), widget_gap),
                                    container_margin,
                                )
                                    .w(px(260.0))
                                    .child(Label::new("KDE Breeze").font_semibold())
                                    .child(
                                        Label::new(
                                            "The Plasma preset: kdeglobals for the palette, \
                                             Breeze for the icons.",
                                        )
                                        .text_sm()
                                        .text_color(cx.theme().muted_foreground),
                                    )
                            }),
                    )
                    .on_hover(self.hover_info(&fi, "HoverCard", &[("bg", "popover", t.popover, "gpui-component/styled.rs:197"), ("text", "popover_foreground", t.popover_foreground, "gpui-component/styled.rs:198")], &[("border-radius", format!("radius: {}px", t.radius.as_f32())), ("geometry", "geometry::popover, which refines the card surface (hover_card.rs, HoverCard::render refine_style)".to_string()), ("card padding", "geometry::container_margin; gap: geometry::widget_gap".to_string())], &[
                            ("edge", "no border: the card is Popover::render_popover_content, whose popover_style draws a shadow ring instead (gpui-component popover.rs and styled.rs)"),
                            ("secondary text", "muted_foreground, set by this showcase on its own content rather than by the widget"),
                            ("trigger", "variants::ghost_button, the flat button's native state colours"),
                            ("delays", "600ms to open and 300ms to close by default, which HoverCard::open_delay and close_delay replace (hover_card.rs, HoverCard::new)"),
                        ])),
            )
            // ContextMenu
            .child(section("ContextMenu (right-click the area below)"))
            .child(
                div()
                    .id("tt-context-menu")
                    .child(
                        div()
                            .id("ctx-menu-area")
                            .p_6()
                            .w_full()
                            .demo_frame(cx)
                            .bg(t.secondary)
                            .child(
                                Label::new("Right-click anywhere in this area")
                                    .text_sm()
                                    .text_color(t.muted_foreground),
                            )
                            .context_menu(|menu, _w, _cx| {
                                menu.menu("Cut", Box::new(gpui::NoAction))
                                    .menu("Copy", Box::new(gpui::NoAction))
                                    .menu("Paste", Box::new(gpui::NoAction))
                                    .separator()
                                    .menu("Select All", Box::new(gpui::NoAction))
                            }),
                    )
                    .on_hover(self.hover_info(&fi, "ContextMenu", &[("bg", "popover", t.popover, "gpui-component/styled.rs:197"), ("text", "popover_foreground", t.popover_foreground, "gpui-component/menu/popup_menu.rs:1477"), ("row hover", "accent", t.accent, "gpui-component/menu/menu_item.rs:117"), ("row hover text", "accent_foreground", t.accent_foreground, "gpui-component/menu/menu_item.rs:118"), ("separator", "border", t.border, "gpui-component/menu/popup_menu.rs:1253")], &[], &[
                            ("trigger", "right-click (MouseButton::Right)"),
                            ("trait", "ContextMenuExt, on any InteractiveElement + ParentElement + Styled (menu/context_menu.rs, ContextMenuExt)"),
                            ("surface", "a ContextMenu is a PopupMenu wrapper and reads no theme field itself (menu/context_menu.rs)"),
                            ("edge", "no border: popover_style draws a shadow ring; the border token is the separator between items (menu/popup_menu.rs)"),
                        ])),
            )
            // Command palette
            .child(section("Command (a search field over a filtered list)"))
            .child(
                div()
                    .id("tt-command")
                    .w(px(360.0))
                    .child(
                        Command::new(&self.command_state)
                            .placeholder("Type a command or search…")
                            .group(
                                CommandGroup::new().label("Suggestions").item(
                                    CommandItem::new()
                                        .label("Calendar")
                                        .icon(IconName::Calendar),
                                ),
                            )
                            .group(
                                CommandGroup::new().label("Settings").items([
                                    CommandItem::new()
                                        .label("Profile")
                                        .icon(IconName::User),
                                    CommandItem::new()
                                        .label("Search")
                                        .icon(IconName::Search),
                                ]),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Command", &[("surface bg", "popover", t.popover, "gpui-component/command/state.rs:830"), ("surface text", "popover_foreground", t.popover_foreground, "gpui-component/command/state.rs:831"), ("border", "border", t.border, "gpui-component/command/state.rs:835"), ("search divider", "border", t.border, "gpui-component/command/state.rs:847"), ("search icon", "muted_foreground", t.muted_foreground, "gpui-component/command/state.rs:852"), ("group label", "muted_foreground", t.muted_foreground, "gpui-component/command/state.rs:685"), ("separator", "border", t.border, "gpui-component/command/state.rs:695"), ("selected row", "accent", t.accent, "gpui-component/command/state.rs:673"), ("selected row text", "accent_foreground", t.accent_foreground, "gpui-component/command/state.rs:674"), ("empty text", "muted_foreground", t.muted_foreground, "gpui-component/command/state.rs:785")], &[
                        ("border-radius", format!("radius_lg: {}px", t.radius_lg.as_f32())),
                    ], &[
                        ("geometry", "none: no geometry:: builder is applied. The palette sizes itself from its own text sizes and a max height of 18.75rem (command/command.rs, CommandOptions::default)"),
                        ("accent is the menu highlight", "the highlighted row takes the same token a menu row's hover does, so the palette follows the platform's menu selection rather than a list selection (native-theme-gpui colors.rs, assign_core)"),
                        ("query field", "an Input with appearance(false): it draws no background and no border of its own, so the surface shows through (command/state.rs, CommandState::render)"),
                    ])),
            )
            // DropdownMenu
            .child(section("DropdownMenu"))
            .child(
                div()
                    .id("tt-menu")
                    .child(
                        DropdownButton::new("menu-demo")
                            .button(Button::new("menu-trigger").label("Click for Menu"))
                            .dropdown_menu(|menu, _w, _cx| {
                                menu.menu("Cut", Box::new(gpui::NoAction))
                                    .menu("Copy", Box::new(gpui::NoAction))
                                    .menu("Paste", Box::new(gpui::NoAction))
                                    .separator()
                                    .menu("Select All", Box::new(gpui::NoAction))
                            }),
                    )
                    .on_hover(self.hover_info(&fi, "PopupMenu / DropdownMenu", &[("bg", "popover", t.popover, "gpui-component/styled.rs:197"), ("text", "popover_foreground", t.popover_foreground, "gpui-component/menu/popup_menu.rs:1477"), ("row hover", "accent", t.accent, "gpui-component/menu/menu_item.rs:117"), ("row hover text", "accent_foreground", t.accent_foreground, "gpui-component/menu/menu_item.rs:118"), ("separator", "border", t.border, "gpui-component/menu/popup_menu.rs:1253")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("separator", "horizontal line"),
                            ("shortcut", "a Kbd, shown when the item's action has a key binding (menu/popup_menu.rs, binding_for_action_in)"),
                            ("rows", "PopupMenu builds its own; geometry::menu_item has no receiver here (geometry.rs, menu/menu_item.rs: MenuItemElement is pub(crate))"),
                        ])),
            )
            // The menu rows an application draws itself. Upstream's
            // `MenuItemElement` is crate-private and `PopupMenu` builds its
            // own rows, so `geometry::menu_item` has no widget to refine —
            // these rows are the receiver it documents.
            .child(section("Menu rows (drawn by the application)"))
            .child(
                div()
                    .id("tt-menu-rows")
                    .child({
                        let row_style = native_geometry(cx, geometry::menu_item);
                        let row = |id: &'static str, icon: IconName, label: &'static str| {
                            refined(
                                div()
                                    .id(id)
                                    .flex()
                                    .items_center()
                                    .hover(|this| this.bg(t.accent))
                                    .child(native_icon(cx, icon, geometry::icon_size_small))
                                    // No `.text_sm()`: a `Label` refines
                                    // itself last (`label.rs:208`), so the row
                                    // would keep its own size instead of
                                    // `menu.font`'s.
                                    .child(Label::new(label)),
                                row_style.as_ref(),
                            )
                        };
                        v_flex()
                            .w(px(220.0))
                            .bg(t.popover)
                            .text_color(t.popover_foreground)
                            .demo_frame(cx)
                            .child(row("mi-cut", IconName::Delete, "Cut"))
                            .child(row("mi-copy", IconName::Copy, "Copy"))
                            .child(row("mi-paste", IconName::Inbox, "Paste"))
                    })
                    .on_hover(self.hover_info(&fi, "Menu row (application-drawn)", &[("bg", "popover", t.popover, "showcase"), ("row hover", "accent", t.accent, "showcase"), ("text", "popover_foreground", t.popover_foreground, "showcase")], &[("geometry", "geometry::menu_item: menu.row_height (control height), menu.border.padding_*, menu.icon_text_gap, menu.font — the label sets no size of its own, so the font arrives".to_string()), ("icon size", "geometry::icon_size_small: defaults.icon_sizes.small".to_string())], &[
                        ])),
            )
    }
}
