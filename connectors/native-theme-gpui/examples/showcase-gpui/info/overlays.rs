//! What the Overlays page's widgets report about themselves (spec §3.4).

use gpui_component::theme::Theme;
use native_theme::next::{ResolvedTheme as NextTheme, WidgetState};
use native_theme_gpui::next::{fill_of, state_fill_of, state_text_color_of, text_color_of};

use super::{
    WidgetInfo, buttons,
    chrome::{
        HoverText, ThemedEntrance, dialog_surface, hovered_menu_edge_unmapped, popup_menu,
        pressed_menu_edge, sheet_surface,
    },
    claim, edge_claim, px_text, state_stroke_color_of, stated_claim, stroke_color_of,
    stroke_width_of,
};
use crate::demo::{
    ButtonKind, ButtonState, FrameFrom, MenuRowsLook, MenuRowsParts, Overlay, SAMPLE_MENU,
    SheetSide,
};
use crate::support::SampleIcon;

/// The page's Button that opens `overlay`, refined by `geometry::button`
/// from `native` where given; its geometry line is recorded by
/// `native_info` where `demo::overlay_button` applies the builder.
pub fn trigger(t: &Theme, overlay: Overlay, native: Option<&NextTheme>) -> WidgetInfo {
    buttons::button(
        t,
        overlay.button_kind(),
        ButtonState::Idle,
        false,
        false,
        None,
        native,
    )
    .instance(
        "opens",
        match overlay {
            Overlay::Dialog => {
                "the Dialog, through WindowExt::open_dialog (window_ext.rs, WindowExt::open_dialog), which reports itself once open"
            }
            Overlay::AlertDialog => {
                "the AlertDialog, through WindowExt::open_alert_dialog (window_ext.rs, WindowExt::open_alert_dialog), which reports itself once open; the heading above names the last answer"
            }
            Overlay::Sheet(SheetSide::Right) => {
                "a Sheet at the window's right edge, through WindowExt::open_sheet (window_ext.rs, WindowExt::open_sheet), which reports itself once open"
            }
            Overlay::Sheet(SheetSide::Bottom) => {
                "a Sheet at the window's bottom edge, through WindowExt::open_sheet_at with Placement::Bottom (window_ext.rs, WindowExt::open_sheet_at), which reports itself once open"
            }
        },
    )
}

/// A Dialog's corners: `radius_lg` from upstream, which `geometry::dialog`
/// replaces where `styled`.
fn dialog_corners(info: WidgetInfo, t: &Theme, styled: bool) -> WidgetInfo {
    if styled {
        info.not_themeable(
            "corner radius",
            "dialog.panel.corners, through geometry::dialog: upstream sets radius_lg, and the refinement replaces it (dialog/dialog.rs, Dialog::render)",
        )
    } else {
        info.config(
            "border-radius",
            format!("radius_lg: {}px", t.radius_lg.as_f32()),
        )
    }
}

/// The Dialog the page's Open Dialog Button opens, drawn while gpui's
/// `reduce_motion` is as given. `styled` is whether `geometry::dialog`
/// refined it from the installed 0.8.0 theme, which decides the corners, and
/// `described` whether `geometry::dialog_description` did, which decides the
/// description's colour. Its
/// geometry lines are recorded where `demo::confirm_dialog` applies the
/// builders; `entrance` as `chrome::dialog_surface`'s.
pub fn dialog(
    t: &Theme,
    reduce_motion: bool,
    entrance: ThemedEntrance,
    styled: bool,
    described: bool,
    icon: &SampleIcon,
) -> WidgetInfo {
    let info = dialog_surface(
        WidgetInfo::new("Dialog").variant("Confirm Action"),
        t,
        reduce_motion,
        entrance,
        styled,
        true,
        "the title, the content and the footer",
    );
    // geometry::dialog_description paints the text with dialog.body's
    // colour, which no ThemeColor field holds, over upstream's.
    let info = if described {
        info
    } else {
        info.color(claim(
            "description",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/dialog/description.rs:50",
        ))
    };
    dialog_corners(info, t, styled)
        .not_themeable(
            "text",
            "none set: the body inherits the window's text colour, and only the description is recoloured (dialog/description.rs, DialogDescription::render)",
        )
        .instance(
            "content",
            if icon.shown() {
                "the icon beside a DialogDescription reading This cannot be undone."
            } else {
                "a DialogDescription reading This cannot be undone."
            },
        )
        .instance("icon", icon.note("the content is the description alone"))
        .instance(
            "footer",
            "a DialogClose holding a Default Button reading Close, which reports itself: DialogClose makes it dispatch Cancel, which closes the Dialog (dialog/footer.rs, DialogClose::trigger)",
        )
        .instance(
            "closes",
            "from its Close Button or the close button in its corner, on Escape, or on a click on the backdrop 34px or more below the window's top -- TITLE_BAR_HEIGHT, whether or not a title bar is drawn there (title_bar.rs:16; dialog/dialog.rs:739; gpui-base/dialog.rs:623)",
        )
}

/// The Button in the footer of the page's Dialog, which closes it, refined by
/// `geometry::button` from `native` where given; its geometry line is
/// recorded where `demo::confirm_dialog` makes the refinement.
pub fn dialog_close(t: &Theme, native: Option<&NextTheme>) -> WidgetInfo {
    buttons::button(
        t,
        ButtonKind::Default,
        ButtonState::Idle,
        false,
        false,
        None,
        native,
    )
    .instance(
        "closes",
        "the Dialog it is in: DialogClose builds it and makes a click dispatch Cancel on the Dialog (dialog/footer.rs, DialogClose::trigger)",
    )
}

/// The AlertDialog the page's Discard changes… Button opens, drawn while
/// gpui's `reduce_motion` is as given, showing `icon` of the chosen icon
/// theme; `styled` and `entrance` as `dialog`'s. Its geometry lines are
/// recorded where `demo::alert_dialog` applies the builders.
pub fn alert_dialog(
    t: &Theme,
    reduce_motion: bool,
    entrance: ThemedEntrance,
    styled: bool,
    icon: &SampleIcon,
) -> WidgetInfo {
    dialog_corners(
        dialog_surface(
            WidgetInfo::new("AlertDialog"),
            t,
            reduce_motion,
            entrance,
            styled,
            false,
            if icon.shown() {
                "the icon, the title and the description"
            } else {
                "the title and the description"
            },
        ),
        t,
        styled,
    )
    .color(claim(
        "description",
        "muted_foreground",
        t.muted_foreground,
        "gpui-component/dialog/description.rs:50",
    ))
    .color(claim(
        "confirm button",
        "button_danger",
        t.button_danger,
        "gpui-component/button/button.rs:1039",
    ))
    .not_themeable(
        "surface",
        if styled {
            "an AlertDialog reads no theme field of its own (dialog/alert_dialog.rs): it is a Dialog, which geometry::dialog fills with dialog.panel.fill"
        } else {
            "an AlertDialog reads no theme field of its own (dialog/alert_dialog.rs): it is a Dialog, so its fill is background and not the popover colour the panel had claimed"
        },
    )
    .not_themeable(
        "footer",
        "right-aligned, and built from button_props when none is given (dialog/alert_dialog.rs, AlertDialog::build_surface; dialog/footer.rs, DialogFooter::render justify_end)",
    )
    .not_themeable(
        "dismissal",
        "no backdrop close by design (dialog/alert_dialog.rs, AlertDialog::overlay_closable, deprecated)",
    )
    .instance(
        "title and description",
        "upstream's DialogTitle and DialogDescription, built around the text the showcase passes in (dialog/alert_dialog.rs, AlertDialog::build_surface). The showcase refines neither, so the platform's dialog fonts do not reach them",
    )
    .instance(
        "buttons",
        "Keep, of the Default variant, and Discard, of the Danger variant: upstream builds both from the button props (dialog/dialog.rs, DialogButtonProps::render_ok), so they show no info of their own and report here. Enter confirms and Escape cancels",
    )
    .instance(
        "icon",
        icon.note("the AlertDialog is given none, and shows none: it draws an icon only where it is given one (dialog/alert_dialog.rs, AlertDialog::icon)"),
    )
}

/// The Sheet at the window's `side` edge that the page opens, drawn while
/// gpui's `reduce_motion` is as given; its info target is its title.
pub fn sheet(t: &Theme, side: SheetSide, reduce_motion: bool) -> WidgetInfo {
    sheet_surface(
        WidgetInfo::new("Sheet").variant(side.name()),
        t,
        side,
        reduce_motion,
        "the rest of the surface",
    )
    .instance(
        "size",
        match side {
            SheetSide::Right => {
                "upstream's 350px default, as its width (gpui-component/sheet.rs, Sheet::new): the model states no sheet, and the showcase sets no size of its own"
            }
            SheetSide::Bottom => {
                "upstream's 350px default, as its height (gpui-component/sheet.rs, Sheet::new): the model states no sheet, and the showcase sets no size of its own"
            }
        },
    )
    .instance(
        "closes",
        "from its close button, on Escape, or on a click on the backdrop (gpui-component/sheet.rs, Sheet::render)",
    )
}

/// What a surface `popover_style` paints -- a Popover's and a HoverCard's --
/// and the showcase's own muted line on it. `native` is the installed 0.8.0
/// theme `geometry::popover` refined it from, where it applies: the
/// refinement replaces the corners and draws the popover's edge.
fn popover_surface(info: WidgetInfo, t: &Theme, native: Option<&NextTheme>) -> WidgetInfo {
    let info = info
        .color(claim(
            "bg",
            "popover",
            t.popover,
            "gpui-component/styled.rs:190",
        ))
        .color(claim(
            "text",
            "popover_foreground",
            t.popover_foreground,
            "gpui-component/styled.rs:191",
        ))
        // POPOVER_RING_INK, a literal 0.1 (styled.rs:28).
        .color(claim(
            "edge ring, at 10%",
            "foreground",
            t.foreground.alpha(0.1),
            "gpui-component/styled.rs:37",
        ))
        .color(claim(
            "secondary text",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        ));
    match native {
        Some(r) => {
            let width = stroke_width_of(&r.popover.panel);
            let info = info.not_themeable(
                "corner radius",
                "popover.panel.corners, through geometry::popover: popover_style sets the theme radius, and the refinement, applied after it, replaces it (gpui-component/styled.rs, popover_style)",
            ).colors(edge_claim(
                "edge",
                "popover.panel.stroke.color",
                stroke_color_of(&r.popover.panel),
                width,
                "showcase",
            ));
            match width.filter(|width| *width > 0.0) {
                Some(width) => info.config(
                    "edge",
                    format!(
                        "popover.panel.stroke.color at popover.panel.stroke.width, {}px, through geometry::popover: popover_style draws no border, so the refinement's lies inside its edge ring, which stays (gpui-component/styled.rs, popover_style)",
                        px_text(width)
                    ),
                ),
                None => info.config(
                    "edge",
                    "none of the theme's: popover.panel's stroke is stated as none, 0 wide, so geometry::popover draws no border, and the edge is popover_style's ring alone (gpui-component/styled.rs, popover_style)",
                ),
            }
        }
        None => info.config("border-radius", format!("radius: {}px", t.radius.as_f32())),
    }
}

/// The page's Popover, whose trigger and content both report it. `native`
/// is the installed 0.8.0 theme `geometry::popover` refined it from, where
/// it applies; its geometry lines are recorded where `demo::popover`
/// applies the builders.
pub fn popover(t: &Theme, native: Option<&NextTheme>) -> WidgetInfo {
    let info = popover_surface(WidgetInfo::new("Popover"), t, native);
    let info = match native {
        Some(_) => info,
        None => info.not_themeable(
            "edge",
            "no border token: the surface is painted by popover_style, whose edge is a 1px shadow ring of foreground at low alpha. That is how the shadow shows through it; popover.panel.stroke.color reaches the surface only through geometry::popover (gpui-component/styled.rs, popover_style and popover_ring)",
        ),
    };
    info.instance(
            "trigger",
            "any Selectable element: here a Default Button reading Click for Popover, which reports the Popover and is drawn selected while it is open (gpui-component/popover.rs, Popover::trigger). Its own colours are a Default Button's, which the Buttons page's Default Button states",
        )
        .instance(
            "anchor",
            "Anchor::TopLeft unless Popover::anchor picks another of its eight points (gpui-component/popover.rs, Popover::anchor)",
        )
        .instance(
            "opens",
            "on a click on the trigger; a click outside closes it (gpui-component/popover.rs, Popover::overlay_closable)",
        )
        .instance(
            "content",
            "a title in the surface's text colour over a line in muted_foreground, set by this showcase on its own content rather than by the widget",
        )
        .instance(
            "unreported",
            "the surface's padding shows no info of its own: upstream builds the surface inline around the content the showcase passes in (gpui-component/popover.rs, Popover::render: popover_style and p_3 on the content's own v_flex), so only the trigger and the content report",
        )
}

/// The page's HoverCard, whose trigger and card both report it. `native`
/// is the installed 0.8.0 theme `geometry::popover` refined it from, where
/// it applies; its geometry lines are recorded where `demo::hover_card`
/// applies the builders and the spacing.
pub fn hover_card(t: &Theme, native: Option<&NextTheme>) -> WidgetInfo {
    let info = popover_surface(WidgetInfo::new("HoverCard"), t, native);
    let info = match native {
        Some(_) => info,
        None => info.not_themeable(
            "edge",
            "no border: the card is Popover::render_popover_content, whose popover_style draws a shadow ring instead (gpui-component/popover.rs and styled.rs)",
        ),
    };
    info.not_themeable(
            "delays",
            "600ms to open and 300ms to close by default, which HoverCard::open_delay and close_delay replace (gpui-component/hover_card.rs, HoverCard::new)",
        )
        .instance(
            "trigger",
            "variants::ghost_button, the flat button's native state colours, on a Button that reports the HoverCard. Its own colours are the Buttons page's Ghost Button's",
        )
        .instance(
            "anchor",
            "Anchor::TopCenter unless HoverCard::anchor picks another (gpui-component/hover_card.rs, HoverCard::new)",
        )
        .instance(
            "secondary text",
            "muted_foreground, set by this showcase on its own content rather than by the widget",
        )
        .instance(
            "unreported",
            "the card's own padding shows no info of its own: upstream builds the card around the content the showcase passes in (gpui-component/hover_card.rs, HoverCard::render), so only the trigger and the content report",
        )
}

/// What the page's two menus hold, as `SAMPLE_MENU` lists it.
fn menu_items() -> String {
    let (first, last) = SAMPLE_MENU;
    format!(
        "{}, a separator, then {last}; each runs gpui::NoAction, so choosing one only closes the menu",
        first.join(", ")
    )
}

/// The page's ContextMenu, which its area reports.
pub fn context_menu(t: &Theme) -> WidgetInfo {
    let info = WidgetInfo::new("ContextMenu")
        .color(claim("area bg", "secondary", t.secondary, "showcase"))
        .color(claim(
            "area text",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        ))
        .color(claim("area frame", "border", t.border, "showcase"));
    popup_menu(info, t)
        .not_themeable(
            "surface",
            "a ContextMenu is a PopupMenu wrapper and reads no theme field itself (menu/context_menu.rs)",
        )
        .instance(
            "trigger",
            "right-click (MouseButton::Right), or a long press on a touch screen (menu/context_menu.rs, open_menu)",
        )
        .instance(
            "trait",
            "ContextMenuExt, on any InteractiveElement + ParentElement + Styled (menu/context_menu.rs, ContextMenuExt)",
        )
        .instance(
            "area",
            "the showcase's own frame, not a widget of gpui-component's: secondary, edged with Theme::border at the platform's border line width and rounded with Theme::radius",
        )
        .not_themeable(
            "area padding",
            format!(
                "{} inside the area, CONTEXT_AREA_PADDING, the showcase's own choice rather than a platform's, as the theme states no such area (spec §3.1)",
                super::rems_text(crate::demo::CONTEXT_AREA_PADDING, t)
            ),
        )
        .instance(
            "opens",
            "with its top-left corner at the pointer where the area was right-clicked (menu/context_menu.rs, ContextMenu)",
        )
        .instance("items", menu_items())
        .instance(
            "unreported",
            "the menu shows no info of its own: PopupMenu builds its items itself (menu/popup_menu.rs, PopupMenu::render_item) on a layer above the page, where nothing of the showcase's can wrap them, so the area reports the menu",
        )
}

/// The `PopupMenu` the page's Click for Menu Button opens, which the Button
/// reports. Its geometry line is recorded where `demo::dropdown_menu`
/// applies `geometry::button` to the Button.
pub fn dropdown_menu(t: &Theme) -> WidgetInfo {
    popup_menu(
        WidgetInfo::new("PopupMenu").variant("Button::dropdown_menu"),
        t,
    )
    .instance(
        "trigger",
        "a Default Button reading Click for Menu, which reports the menu: a click opens it, anchored at its top-left corner (menu/dropdown_menu.rs, DropdownMenu::dropdown_menu), and the Button is drawn selected while it is open (gpui-component/popover.rs, Popover::trigger). Its own colours are a Default Button's, which the Buttons page's Default Button states",
    )
    .instance("items", menu_items())
    .instance(
        "unreported",
        "the menu shows no info of its own: PopupMenu builds its items itself (menu/popup_menu.rs, PopupMenu::render_item) on a layer above the page, where nothing of the showcase's can wrap them, so the Button reports the menu",
    )
}

/// One of the menu rows the application draws itself, reading `label`,
/// with `icon` of the chosen icon theme, painted as `look`
/// (`demo::MenuRowsLook`) paints it from the menu's `parts` of the
/// installed 0.8.0 theme `native`: each property the theme states claimed
/// at its home, each it leaves unstated at the showcase's own token for it
/// alone (spec §10, rule 0). Its geometry lines are recorded where
/// `demo::menu_rows` applies the builders.
pub fn menu_row(
    t: &Theme,
    native: Option<&NextTheme>,
    parts: &MenuRowsParts,
    look: &MenuRowsLook,
    label: &'static str,
    icon: &SampleIcon,
) -> WidgetInfo {
    let installed = native.is_some();
    let info = menu_row_look(
        WidgetInfo::new("Menu row").variant(label),
        t,
        native,
        parts,
        look,
    );
    let unstated: Vec<&str> = [
        ("menu.panel(NONE).fill", fill_of(&parts.panel).is_none()),
        (
            "menu.item_label(NONE).color",
            text_color_of(&parts.label).is_none(),
        ),
        (
            "menu.item(HOVER).fill",
            state_fill_of(&parts.hovered).is_none(),
        ),
        (
            "menu.item_label(HOVER).color",
            matches!(hover_text_from(parts), HoverText::Accent),
        ),
        ("popover.panel.stroke.width", look.frame_width.1.is_none()),
        ("popover.panel.corners", look.frame_radius.1.is_none()),
    ]
    .into_iter()
    .filter_map(|(home, unstated)| unstated.then_some(home))
    .collect();
    let drawn_by = "the application: upstream's MenuItemElement is crate-private and PopupMenu builds its own rows (menu/menu_item.rs, MenuItemElement), so these rows are the receiver geometry::menu_item documents.";
    let info = if !installed {
        info.instance(
            "drawn by",
            format!("{drawn_by} With no native theme, they take the popover's tokens and hover as a PopupMenu's item does, with accent and accent_foreground, in the showcase's own frame"),
        )
    } else if unstated.is_empty() {
        info.instance(
            "drawn by",
            format!("{drawn_by} They are painted as the showcase's own menus are, in menu.panel(NONE).fill, menu.item_label(NONE).color and the menu's hover colours"),
        )
    } else {
        info.instance(
            "drawn by",
            format!("{drawn_by} They are painted as the showcase's own menus are, but where the theme states no value gpui can take, at {}, each of those alone takes the showcase's own (popover, popover_foreground, accent, accent_foreground, the showcase's frame)", unstated.join(", ")),
        )
    };
    info.instance(
        "label",
        "plain text rather than a Label: a Label paints foreground on its own element and sets its own line height (gpui-component/label.rs, Label::render), which would hide the frame's text colour; plain text takes menu.item_label(NONE)'s font from the row",
    )
    .instance("icon", icon.note("the row shows its label alone"))
}

/// Where the menu rows' hovered text is from, for `parts`
/// (`demo::menu_hover_of`).
fn hover_text_from(parts: &MenuRowsParts) -> HoverText {
    match (
        state_text_color_of(&parts.hovered_label).is_some(),
        state_fill_of(&parts.hovered).is_some(),
    ) {
        (true, _) => HoverText::Stated,
        (false, true) => HoverText::AtRest,
        (false, false) => HoverText::Accent,
    }
}

/// What a menu row painted with `look`, the menu rows' look the `parts` of
/// `r` give (`demo::MenuRowsLook`), claims and says, property by property:
/// a colour `r` states at its home, read there, else the showcase's own
/// token's.
fn menu_row_look(
    info: WidgetInfo,
    t: &Theme,
    r: Option<&NextTheme>,
    parts: &MenuRowsParts,
    look: &MenuRowsLook,
) -> WidgetInfo {
    let (frame_width, frame_from) = (look.frame_width.0.as_f32(), look.frame_width.1);
    let info = info
        .color(
            r.and_then(|r| {
                stated_claim(
                    "bg (the frame's)",
                    "menu.panel(NONE).fill",
                    fill_of(&r.menu.panel(WidgetState::NONE)),
                    "showcase",
                )
            })
            .unwrap_or_else(|| claim("bg (the frame's)", "popover", t.popover, "showcase")),
        )
        .color(
            r.and_then(|r| {
                stated_claim(
                    "text",
                    "menu.item_label(NONE).color",
                    text_color_of(&r.menu.item_label(WidgetState::NONE)),
                    "showcase",
                )
            })
            .unwrap_or_else(|| {
                claim(
                    "text",
                    "popover_foreground",
                    t.popover_foreground,
                    "showcase",
                )
            }),
        )
        .color(
            r.and_then(|r| {
                stated_claim(
                    "hover",
                    "menu.item(HOVER).fill",
                    state_fill_of(&r.menu.item(WidgetState::HOVER)),
                    "showcase",
                )
            })
            .unwrap_or_else(|| claim("hover", "accent", t.accent, "showcase")),
        )
        .color(
            match (r, hover_text_from(parts)) {
                (Some(r), HoverText::Stated) => stated_claim(
                    "hover text",
                    "menu.item_label(HOVER).color",
                    state_text_color_of(&r.menu.item_label(WidgetState::HOVER)),
                    "showcase",
                ),
                (Some(r), HoverText::AtRest) => stated_claim(
                    "hover text",
                    "menu.item_label(NONE).color",
                    text_color_of(&r.menu.item_label(WidgetState::NONE)),
                    "showcase",
                ),
                _ => None,
            }
            .unwrap_or_else(|| {
                claim(
                    "hover text",
                    "accent_foreground",
                    t.accent_foreground,
                    "showcase",
                )
            }),
        )
        .colors(match (frame_from, r) {
            (Some(FrameFrom::Menu), Some(r)) => edge_claim(
                "frame edge",
                "menu.panel(NONE).stroke.color",
                stroke_color_of(&r.menu.panel(WidgetState::NONE)),
                Some(frame_width),
                "showcase",
            ),
            (Some(FrameFrom::Popover), Some(r)) => edge_claim(
                "frame edge",
                "popover.panel.stroke.color",
                stroke_color_of(&r.popover.panel),
                Some(frame_width),
                "showcase",
            ),
            _ => edge_claim(
                "frame edge",
                "border",
                Some(t.border),
                Some(frame_width),
                "showcase",
            ),
        })
        .colors(r.and_then(|r| {
            edge_claim(
                "hovered item edge",
                "menu.item(HOVER).stroke.color",
                state_stroke_color_of(&r.menu.item(WidgetState::HOVER)),
                look.edge.map(|(_, width)| width.as_f32()),
                "showcase",
            )
        }))
        .colors(r.and_then(|r| {
            stated_claim(
                "pressed item",
                "menu.item(PRESSED).fill",
                state_fill_of(&r.menu.item(WidgetState::PRESSED)),
                "showcase",
            )
        }))
        .colors(r.and_then(|r| {
            stated_claim(
                "pressed item text",
                "menu.item_label(PRESSED).color",
                state_text_color_of(&r.menu.item_label(WidgetState::PRESSED)),
                "showcase",
            )
        }));
    let info = match look.edge {
        Some((_, width)) => info.config(
            "hovered item edge",
            format!(
                "the stroke a rule states for menu.item(HOVER), {}px, a box over the hovered row, its padding included, so nothing moves, rounded as the highlight",
                px_text(width.as_f32())
            ),
        ),
        None if state_stroke_color_of(&parts.hovered).is_some() => info.config(
            "hovered item edge",
            "none: the stroke a rule states for menu.item(HOVER) is 0 wide",
        ),
        None => hovered_menu_edge_unmapped(info, &parts.hovered),
    };
    let info = info.colors(r.and_then(|r| {
        edge_claim(
            "pressed item edge",
            "menu.item(PRESSED).stroke.color",
            state_stroke_color_of(&r.menu.item(WidgetState::PRESSED)),
            look.pressed_edge.map(|(_, width)| width.as_f32()),
            "showcase",
        )
    }));
    let info = pressed_menu_edge(info, look.pressed_edge, look.edge.is_some(), &parts.pressed);
    let corners = match look.frame_radius.1 {
        Some(FrameFrom::Menu) => "menu.panel(NONE).corners",
        Some(FrameFrom::Popover) => "popover.panel.corners",
        None => {
            "the showcase's own radius token: neither menu.panel(NONE) nor popover.panel states corners gpui can take"
        }
    };
    let radius = px_text(look.frame_radius.0.as_f32());
    let info = match frame_from {
        Some(from) => {
            let stroke = match from {
                FrameFrom::Menu => "menu.panel(NONE).stroke",
                FrameFrom::Popover => "popover.panel.stroke",
            };
            if frame_width > 0.0 {
                info.config(
                    "frame edge",
                    format!(
                        "{stroke}: {}px, radius {radius}px ({corners}), as the showcase's menus frame their popup: the menu's own panel's where it states one, else popover.panel's, the popup frame a connector draws where the menu states none (docs/v0.8.0-spec.md §5, notes to the table, the menu's popup frame)",
                        px_text(frame_width)
                    ),
                )
            } else {
                info.config(
                    "frame edge",
                    format!(
                        "none: {stroke} is stated as none, 0 wide; the frame is still rounded by {corners}, {radius}px"
                    ),
                )
            }
        }
        None => info.config(
            "frame edge",
            format!(
                "the showcase's own frame, {}px in the border token, radius {radius}px ({corners}): neither menu.panel(NONE) nor popover.panel states a stroke width gpui can take",
                px_text(frame_width)
            ),
        ),
    };
    match look.radius {
        Some(radius) => info.config(
            "highlight",
            format!(
                "menu.item(NONE).corners on each row, {}px (platform-facts §2.6: the items are rectangular where it is 0)",
                px_text(radius.as_f32())
            ),
        ),
        None => info.config(
            "highlight",
            "square: the theme states no value gpui can take at menu.item(NONE).corners, and the rows are drawn square, as with no native theme",
        ),
    }
}
