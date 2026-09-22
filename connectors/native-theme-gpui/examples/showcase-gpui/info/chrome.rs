//! What the window's chrome reports about itself (spec §2).

use gpui_component::theme::Theme;

use super::{WidgetInfo, claim};

/// The window's `TitleBar` (spec §2.1). Its geometry line is recorded by
/// `native_info` where `demo::title_bar` applies the builder.
pub fn title_bar(t: &Theme) -> WidgetInfo {
    let info = WidgetInfo::new("TitleBar")
        .color(claim(
            "bg",
            "title_bar",
            t.title_bar,
            "gpui-component/title_bar.rs:340",
        ))
        .color(claim(
            "border",
            "title_bar_border",
            t.title_bar_border,
            "gpui-component/title_bar.rs:338",
        ));
    // macOS draws no window controls of upstream's (title_bar.rs:254-256).
    let info = if cfg!(target_os = "macos") {
        info
    } else {
        info.color(claim(
            "window control text",
            "foreground",
            t.foreground,
            "gpui-component/title_bar.rs:217",
        ))
        .color(claim(
            "control hover",
            "secondary_hover",
            t.secondary_hover,
            "gpui-component/title_bar.rs:181",
        ))
        .color(claim(
            "control hover icon",
            "secondary_foreground",
            t.secondary_foreground,
            "gpui-component/title_bar.rs:172",
        ))
        .color(claim(
            "control pressed",
            "secondary_active",
            t.secondary_active,
            "gpui-component/title_bar.rs:190",
        ))
        .color(claim(
            "close hover",
            "danger",
            t.danger,
            "gpui-component/title_bar.rs:179",
        ))
        .color(claim(
            "close hover icon",
            "danger_foreground",
            t.danger_foreground,
            "gpui-component/title_bar.rs:170",
        ))
        .color(claim(
            "close pressed",
            "danger_active",
            t.danger_active,
            "gpui-component/title_bar.rs:188",
        ))
    };
    info.not_themeable(
        "height",
        "TITLE_BAR_HEIGHT, 34px, and settable: TitleBar applies the caller's refinement after it (title_bar.rs, TitleBar). The model states no title-bar height -- our gap. Only the window controls stay 34px wide",
    )
    .not_themeable(
        "fill",
        "a gradient between title_bar and background (title_bar.rs, default_title_bar_background)",
    )
    .instance(
        "window",
        "the window asks to draw its own decorations, on top of TitleBar::window_options (title_bar.rs, TitleBar::window_options), so this bar is the window's title bar",
    )
    .instance(
        "label",
        "the installed preset and colour mode, as plain text rather than a Label: a Label paints foreground on its own element (label.rs, Label), which would hide the colour geometry::title_bar gives the bar",
    )
    .instance(
        "window controls",
        if cfg!(target_os = "windows") {
            "the OS hit-tests these buttons and upstream discards the close handler (title_bar.rs, TitleBar::on_close_window), so the X closes the window itself instead of dispatching Quit"
        } else if cfg!(target_os = "macos") {
            "none of upstream's: the system draws the traffic lights, which the bar leaves room for (title_bar.rs, TITLE_BAR_LEFT_PADDING)"
        } else {
            "the close button dispatches Quit, which quits the application (title_bar.rs, TitleBar::on_close_window); dragging the bar moves the window and a double click zooms it. They are drawn only while the window is client-decorated: under server-side decorations the window manager draws its own (title_bar.rs, WindowControls)"
        },
    )
    .instance(
        "menus",
        if cfg!(target_os = "macos") {
            "in the system's menu bar, through cx.set_menus; the bar holds no AppMenuBar"
        } else {
            "the AppMenuBar is the bar's second child, after the label"
        },
    )
}

/// The `AppMenuBar` inside the window's title bar (spec §2.2).
pub fn app_menu_bar(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("AppMenuBar")
        .color(claim(
            "item text",
            "secondary_foreground",
            t.secondary_foreground,
            "gpui-component/button/button.rs:964",
        ))
        .color(claim(
            "item hover",
            "accent",
            t.accent,
            "gpui-component/button/button.rs:1126",
        ))
        .color(claim(
            "menu bg",
            "popover",
            t.popover,
            "gpui-component/styled.rs:197",
        ))
        .not_themeable(
            "fill",
            "none: an AppMenuBar reads no theme field at all (menu/app_menu_bar.rs) and paints no bar background, so the title bar's fill shows through",
        )
        .not_themeable(
            "items",
            "ghost Buttons, so they hover with accent rather than the button family, and their label is the Ghost variant's secondary_foreground",
        )
        .instance(
            "source",
            "gpui-base's GlobalState app menus, which only set_app_menus fills -- not gpui's cx.set_menus, which feeds the platform's own menu bar. The showcase gives both the same menus (menu/app_menu_bar.rs, AppMenuBar::reload)",
        )
        .instance(
            "menus",
            "File, View, Theme and Help; each item runs a gpui action, the one its key binding runs too. An item whose action has no handler yet is disabled",
        )
}
