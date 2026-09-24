//! What the Overlays page's widgets report about themselves (spec §3.4).

use gpui_component::theme::Theme;

use super::{
    WidgetInfo, buttons,
    chrome::{dialog_surface, popup_menu, sheet_surface},
    claim,
};
use crate::demo::{ButtonKind, ButtonState, Overlay, SAMPLE_MENU, SheetSide};
use crate::support::SampleIcon;

/// The page's Button that opens `overlay`. `styled` is whether
/// `geometry::button` refined it; its geometry line is recorded by
/// `native_info` where `demo::overlay_button` applies the builder.
pub fn trigger(t: &Theme, overlay: Overlay, styled: bool) -> WidgetInfo {
    buttons::button(
        t,
        overlay.button_kind(),
        ButtonState::Idle,
        false,
        None,
        styled,
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
            "dialog.border.corner_radius, through geometry::dialog: upstream sets radius_lg, and the refinement replaces it (dialog/dialog.rs, Dialog::render)",
        )
    } else {
        info.config(
            "border-radius",
            format!("radius_lg: {}px", t.radius_lg.as_f32()),
        )
    }
}

/// The Dialog the page's Open Dialog Button opens, drawn while gpui's
/// `reduce_motion` is as given. `styled` is whether a native theme's geometry
/// reached it, which decides the description's colour and the corners. Its
/// geometry lines are recorded where `demo::confirm_dialog` applies the
/// builders.
pub fn dialog(t: &Theme, reduce_motion: bool, styled: bool, icon: &SampleIcon) -> WidgetInfo {
    let info = dialog_surface(
        WidgetInfo::new("Dialog").variant("Confirm Action"),
        t,
        reduce_motion,
        true,
        "the title, the content and the footer",
    );
    // geometry::dialog_description paints the text with dialog.body_font's
    // colour, which no ThemeColor field holds, over upstream's.
    let info = if styled {
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
            "from its Close Button or the close button in its corner, on Escape, or on a click on the backdrop 34px or more below the window's top -- TITLE_BAR_HEIGHT, whether or not a title bar is drawn there (title_bar.rs:15; dialog/dialog.rs:586; gpui-base dialog.rs:601)",
        )
}

/// The Button in the footer of the page's Dialog, which closes it. `styled`
/// is whether `geometry::button` refined it; its geometry line is recorded
/// where `demo::confirm_dialog` makes the refinement.
pub fn dialog_close(t: &Theme, styled: bool) -> WidgetInfo {
    buttons::button(
        t,
        ButtonKind::Default,
        ButtonState::Idle,
        false,
        None,
        styled,
    )
    .instance(
        "closes",
        "the Dialog it is in: DialogClose builds it and makes a click dispatch Cancel on the Dialog (dialog/footer.rs, DialogClose::trigger)",
    )
}

/// The AlertDialog the page's Discard changes… Button opens, drawn while
/// gpui's `reduce_motion` is as given, showing `icon` of the chosen icon
/// theme; `styled` as `dialog`'s. Its geometry lines are recorded where
/// `demo::alert_dialog` applies the builders.
pub fn alert_dialog(t: &Theme, reduce_motion: bool, styled: bool, icon: &SampleIcon) -> WidgetInfo {
    dialog_corners(
        dialog_surface(
            WidgetInfo::new("AlertDialog"),
            t,
            reduce_motion,
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
        "gpui-component/button/button.rs:938",
    ))
    .not_themeable(
        "surface",
        "an AlertDialog reads no theme field of its own (dialog/alert_dialog.rs): it is a Dialog, so its fill is background and not the popover colour the panel had claimed",
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
                "upstream's 350px default, as its width (sheet.rs, Sheet::new): the model states no sheet, and the showcase sets no size of its own"
            }
            SheetSide::Bottom => {
                "upstream's 350px default, as its height (sheet.rs, Sheet::new): the model states no sheet, and the showcase sets no size of its own"
            }
        },
    )
    .instance(
        "closes",
        "from its close button, on Escape, or on a click on the backdrop (sheet.rs, Sheet::render)",
    )
}

/// What a surface `popover_style` paints -- a Popover's and a HoverCard's --
/// and the showcase's own muted line on it. `styled` is whether
/// `geometry::popover` refined it, which replaces the corners.
fn popover_surface(info: WidgetInfo, t: &Theme, styled: bool) -> WidgetInfo {
    let info = info
        .color(claim(
            "bg",
            "popover",
            t.popover,
            "gpui-component/styled.rs:197",
        ))
        .color(claim(
            "text",
            "popover_foreground",
            t.popover_foreground,
            "gpui-component/styled.rs:198",
        ))
        // POPOVER_RING_INK, a literal 0.1 (styled.rs:26).
        .color(claim(
            "edge ring, at 10%",
            "foreground",
            t.foreground.alpha(0.1),
            "gpui-component/styled.rs:35",
        ))
        .color(claim(
            "secondary text",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        ));
    if styled {
        info.not_themeable(
            "corner radius",
            "popover.border.corner_radius, through geometry::popover: popover_style sets the theme radius, and the refinement, applied after it, replaces it (gpui-component styled.rs, popover_style)",
        )
    } else {
        info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    }
}

/// The page's Popover, whose trigger and content both report it. `styled`
/// is whether `geometry::popover` refined it; its geometry lines are
/// recorded where `demo::popover` applies the builders.
pub fn popover(t: &Theme, styled: bool) -> WidgetInfo {
    popover_surface(WidgetInfo::new("Popover"), t, styled)
        .not_themeable(
            "edge",
            "no border token: the surface is painted by popover_style, whose edge is a 1px shadow ring of foreground at low alpha. That is how the shadow shows through it, and it is why popover.border.color has no receiver here (gpui-component styled.rs, popover_style and popover_ring)",
        )
        .instance(
            "trigger",
            "any Selectable element: here a Default Button reading Click for Popover, which reports the Popover and is drawn selected while it is open (popover.rs, Popover::trigger). Its own colours are a Default Button's, which the Buttons page's Default Button states",
        )
        .instance(
            "anchor",
            "Anchor::TopLeft unless Popover::anchor picks another of its eight points (popover.rs, Popover::anchor)",
        )
        .instance(
            "opens",
            "on a click on the trigger; a click outside closes it (popover.rs, Popover::overlay_closable)",
        )
        .instance(
            "content",
            "a title in the surface's text colour over a line in muted_foreground, set by this showcase on its own content rather than by the widget",
        )
        .instance(
            "unreported",
            "the surface's padding shows no info of its own: upstream builds the surface around the content the showcase passes in (popover.rs, Popover::render_popover_content), so only the trigger and the content report",
        )
}

/// The page's HoverCard, whose trigger and card both report it. `styled` is
/// whether `geometry::popover` refined it; its geometry lines are recorded
/// where `demo::hover_card` applies the builders and the spacing.
pub fn hover_card(t: &Theme, styled: bool) -> WidgetInfo {
    popover_surface(WidgetInfo::new("HoverCard"), t, styled)
        .not_themeable(
            "edge",
            "no border: the card is Popover::render_popover_content, whose popover_style draws a shadow ring instead (gpui-component popover.rs and styled.rs)",
        )
        .not_themeable(
            "delays",
            "600ms to open and 300ms to close by default, which HoverCard::open_delay and close_delay replace (hover_card.rs, HoverCard::new)",
        )
        .instance(
            "trigger",
            "variants::ghost_button, the flat button's native state colours, on a Button that reports the HoverCard. Its own colours are the Buttons page's Ghost Button's",
        )
        .instance(
            "anchor",
            "Anchor::TopCenter unless HoverCard::anchor picks another (hover_card.rs, HoverCard::new)",
        )
        .instance(
            "secondary text",
            "muted_foreground, set by this showcase on its own content rather than by the widget",
        )
        .instance(
            "unreported",
            "the card's own padding shows no info of its own: upstream builds the card around the content the showcase passes in (hover_card.rs, HoverCard::render), so only the trigger and the content report",
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
        .instance("trigger", "right-click (MouseButton::Right)")
        .instance(
            "trait",
            "ContextMenuExt, on any InteractiveElement + ParentElement + Styled (menu/context_menu.rs, ContextMenuExt)",
        )
        .instance(
            "area",
            "the showcase's own frame, not a widget of gpui-component's: secondary, edged with Theme::border at the platform's border line width and rounded with Theme::radius",
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
        "a Default Button reading Click for Menu, which reports the menu: a click opens it, anchored at its top-left corner (menu/dropdown_menu.rs, DropdownMenu::dropdown_menu), and the Button is drawn selected while it is open (popover.rs, Popover::trigger). Its own colours are a Default Button's, which the Buttons page's Default Button states",
    )
    .instance("items", menu_items())
    .instance(
        "unreported",
        "the menu shows no info of its own: PopupMenu builds its items itself (menu/popup_menu.rs, PopupMenu::render_item) on a layer above the page, where nothing of the showcase's can wrap them, so the Button reports the menu",
    )
}

/// One of the menu rows the application draws itself, reading `label`,
/// with `icon` of the chosen icon theme. Its geometry lines are recorded
/// where `demo::menu_rows` applies the builders.
pub fn menu_row(t: &Theme, label: &'static str, icon: &SampleIcon) -> WidgetInfo {
    WidgetInfo::new("Menu row")
        .variant(label)
        .color(claim("bg (the frame's)", "popover", t.popover, "showcase"))
        .color(claim(
            "text",
            "popover_foreground",
            t.popover_foreground,
            "showcase",
        ))
        .color(claim("hover", "accent", t.accent, "showcase"))
        .color(claim(
            "hover text",
            "accent_foreground",
            t.accent_foreground,
            "showcase",
        ))
        .instance(
            "drawn by",
            "the application: upstream's MenuItemElement is crate-private and PopupMenu builds its own rows (menu/menu_item.rs, MenuItemElement), so these rows are the receiver geometry::menu_item documents. They hover as a PopupMenu's item does, with accent and accent_foreground",
        )
        .instance(
            "label",
            "plain text rather than a Label: a Label paints foreground on its own element and sets its own line height (label.rs, Label::render), which would hide the frame's text colour; plain text takes menu.font from the row",
        )
        .instance("icon", icon.note("the row shows its label alone"))
}
