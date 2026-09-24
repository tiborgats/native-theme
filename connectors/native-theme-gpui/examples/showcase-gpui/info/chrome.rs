//! What the window's chrome reports about itself (spec §2).

use gpui::Pixels;
use gpui_component::{Colorize as _, theme::Theme};
use native_theme::theme::ResolvedPadding;

use super::{ColorClaim, WidgetInfo, claim, px_text};
use crate::demo::{Severity, SheetSide};
use crate::support::ChromeIcon;

/// The window's `TitleBar` (spec §2.1, §3.4, S8), reading `label`: the
/// window's title bar where the window was granted client-side decorations.
/// Its geometry line is recorded by `native_info` where `demo::title_bar`
/// applies the builder.
pub fn title_bar(t: &Theme, label: &str) -> WidgetInfo {
    // macOS draws no window controls of upstream's (title_bar.rs:254-256).
    let info = title_bar_colours(t, cfg!(not(target_os = "macos")));
    let info = if cfg!(target_os = "macos") {
        info
    } else {
        info.not_themeable(
            "own icons",
            super::own_icons("the window controls' WindowMinimize, WindowMaximize or WindowRestore, and WindowClose (title_bar.rs, ControlIcon)"),
        )
    };
    title_bar_notes(info)
    .instance(
        "window",
        "the window asked the window manager to draw its frame, and was granted client-side decorations instead -- a Wayland compositor that offers none, as GNOME's Mutter, leaves the frame to the application -- so this bar is the window's title bar, and Root draws the rest of the frame (window_border.rs, WindowBorder)",
    )
    .instance(
        "label",
        format!("\"{label}\", this crate's name and version: the same string, WINDOW_TITLE, is the title the OS shows and the one the Windows screenshot capture finds the window by. The installed preset and colour mode are in the status bar. Plain text rather than a Label: a Label paints foreground on its own element (label.rs, Label), which would hide the colour geometry::title_bar gives the bar"),
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

/// The Layout page's `TitleBar` sample (spec S8), reading `label`, shown
/// while the window manager draws the window's frame. Its geometry line is
/// recorded by `native_info` where `demo::title_bar_sample` applies the
/// builder.
pub fn title_bar_sample(t: &Theme, label: &str) -> WidgetInfo {
    // Upstream draws its controls on Windows in any window, on Linux only in
    // a client-decorated one, and on macOS never (title_bar.rs:254-270).
    let info = title_bar_colours(t, cfg!(target_os = "windows"));
    let info = if cfg!(target_os = "windows") {
        info.not_themeable(
            "own icons",
            super::own_icons("the window controls' WindowMinimize, WindowMaximize or WindowRestore, and WindowClose (title_bar.rs, ControlIcon)"),
        )
    } else {
        info
    };
    title_bar_notes(info)
        .instance(
            "sample",
            "the window manager draws this window's frame, so the window has no TitleBar of its own: this one is drawn as the window's title bar is where the window manager leaves the frame to the application",
        )
        .instance(
            "label",
            format!("\"{label}\", the window's title, which the window's own TitleBar reads"),
        )
        .instance(
            "pointer",
            "a box laid over the bar takes the pointer and carries this info. A TitleBar acts on the window it is in -- a drag moves it and a double click zooms it (title_bar.rs, RenderOnce for TitleBar) -- and gpui leaves every hitbox under the box out of the hit test (window.rs, HitboxBehavior), so this one moves, zooms and closes nothing",
        )
        .instance(
            "window controls",
            if cfg!(target_os = "windows") {
                "drawn, as upstream draws them in any Windows window (title_bar.rs, WindowControls); the OS would hit-test them as the window's own, and the box keeps them out of that hit test"
            } else if cfg!(target_os = "macos") {
                "none: upstream draws none on macOS (title_bar.rs, WindowControls)"
            } else {
                "none: upstream draws them only in a client-decorated window, and this one is server-decorated (title_bar.rs, WindowControls)"
            },
        )
}

/// What a `TitleBar` paints: its fill and border, and with `controls` drawn,
/// its window controls' colours.
fn title_bar_colours(t: &Theme, controls: bool) -> WidgetInfo {
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
    if !controls {
        return info;
    }
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
}

/// What cannot be given to a `TitleBar`.
fn title_bar_notes(info: WidgetInfo) -> WidgetInfo {
    info.not_themeable(
        "height",
        "TITLE_BAR_HEIGHT, 34px, and settable: TitleBar applies the caller's refinement after it (title_bar.rs, TitleBar). The model states no title-bar height -- our gap. Only the window controls stay 34px wide",
    )
    .not_themeable(
        "fill",
        "a gradient between title_bar and background (title_bar.rs, default_title_bar_background)",
    )
}

/// What an `AppMenuBar` sits in, and so what shows through it.
#[derive(Clone, Copy)]
pub enum MenuHost {
    /// The menu-bar row, under the window manager's frame.
    Row,
    /// The window's own TitleBar, under client-side decorations.
    TitleBar,
}

/// The menu-bar row (spec S8), at the top of a window whose frame the window
/// manager draws. `margin` is the installed layout's
/// `layout.container_margin`, which the row borrows, and `own` the showcase's
/// `MENU_BAR_PADDING`.
pub fn menu_bar(t: &Theme, margin: Option<Pixels>, own: Pixels) -> WidgetInfo {
    let sides = match margin {
        Some(m) => format!(
            "left and right {}px, layout.container_margin, borrowed",
            px_text(m.as_f32())
        ),
        None => format!(
            "left and right {}px, MENU_BAR_PADDING, the showcase's own choice rather than a platform's: the theme states no layout.container_margin to borrow either, and the row is the application's own element, with no toolkit default to keep (spec §3.1)",
            px_text(own.as_f32())
        ),
    };
    WidgetInfo::new("Menu bar")
        .color(claim("bg (the window's)", "background", t.background, "showcase"))
        .not_themeable(
            "widget",
            "gpui-component has no menu-bar row, so this row is the application's own h_flex around the AppMenuBar. The model states no menu bar either: its menu is the popup a menu opens (platform-facts §2.6)",
        )
        .not_themeable(
            "inset",
            "the model states no menu-bar inset. The row borrows layout.container_margin, the padding inside containers, so its menus start where the toolbar's buttons do wherever toolbar.border states no left padding of its own: the showcase's choice",
        )
        .not_themeable(
            "fill",
            "none of its own: the view's background, which the showcase paints under the whole window, shows through",
        )
        .instance(
            "window",
            "the window manager draws this window's frame, so the menus sit in a row of their own at the top of the window, above the toolbar, as a KDE application places them",
        )
        .instance(
            "padding",
            format!("{sides}; top and bottom none: the AppMenuBar's items set the row's height"),
        )
}

/// The `AppMenuBar` (spec §2.2), in `host`.
pub fn app_menu_bar(t: &Theme, host: MenuHost) -> WidgetInfo {
    let info = WidgetInfo::new("AppMenuBar")
        .colors(ghost_rest_and_hover(t, GhostContent::Text))
        .color(claim(
            "item of the open menu",
            "secondary_active",
            t.secondary_active,
            "gpui-component/button/button.rs:1245",
        ));
    popup_menu(info, t)
        .color(claim(
            "menu shortcut text",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/kbd.rs:237",
        ))
        .not_themeable(
            "fill",
            match host {
                MenuHost::Row => "none: an AppMenuBar reads no theme field at all (menu/app_menu_bar.rs) and paints no bar background, so the window's background shows through",
                MenuHost::TitleBar => "none: an AppMenuBar reads no theme field at all (menu/app_menu_bar.rs) and paints no bar background, so the title bar's fill shows through",
            },
        )
        .not_themeable(
            "items",
            "ghost Buttons, so they hover with accent -- at half alpha in dark mode -- rather than the button family, and their label is the Ghost variant's secondary_foreground",
        )
        .not_themeable(
            "press",
            "opens the item's menu at once, and the item of an open menu is selected, which a Ghost paints secondary_active (menu/app_menu_bar.rs, AppMenu::render). A selected Button takes no hover or press style (button/button.rs, RenderOnce for Button), so the Ghost's pressed colour never shows here",
        )
        .instance(
            "source",
            "gpui-base's GlobalState app menus, which only set_app_menus fills -- not gpui's cx.set_menus, which feeds the platform's own menu bar. The showcase gives both the same menus (menu/app_menu_bar.rs, AppMenuBar::reload)",
        )
        .instance(
            "menus",
            "File, View, Theme and Help; each item runs a gpui action, and where the item has a key binding, the binding runs the same action",
        )
}

/// What every `PopupMenu` the showcase opens paints -- the AppMenuBar's, the
/// Overlays page's ContextMenu's and its dropdown menu's -- and what cannot
/// be given to it. Its menus hold plain items and separators, so every row
/// is one `MenuItemElement` of upstream's.
pub(super) fn popup_menu(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.color(claim(
        "menu bg",
        "popover",
        t.popover,
        "gpui-component/styled.rs:197",
    ))
    // POPOVER_RING_INK, a literal 0.1 (styled.rs:26).
    .color(claim(
        "menu edge ring, at 10%",
        "foreground",
        t.foreground.alpha(0.1),
        "gpui-component/styled.rs:35",
    ))
    .color(claim(
        "menu item text",
        "foreground",
        t.foreground,
        "gpui-component/menu/menu_item.rs:107",
    ))
    .color(claim(
        "menu item hover",
        "accent",
        t.accent,
        "gpui-component/menu/menu_item.rs:117",
    ))
    .color(claim(
        "menu item hover text",
        "accent_foreground",
        t.accent_foreground,
        "gpui-component/menu/menu_item.rs:118",
    ))
    .color(claim(
        "menu separator",
        "border",
        t.border,
        "gpui-component/menu/popup_menu.rs:1253",
    ))
    .config(
        "menu border-radius",
        format!("radius: {}px", t.radius.as_f32()),
    )
    .not_themeable(
        "menu text",
        "foreground on every item: the menu sets popover_foreground on itself (menu/popup_menu.rs, PopupMenu::render) and each item sets foreground over it (menu/menu_item.rs, MenuItemElement::render), so popover_foreground reaches none of its text",
    )
    .not_themeable(
        "menu edge",
        "no border: popover_style draws a shadow ring, foreground at 10%, a literal (styled.rs, popover_ring); the border token is the separator between items (menu/popup_menu.rs)",
    )
    .not_themeable(
        "menu separator",
        "2px tall, a literal (menu/popup_menu.rs, PopupMenu::render_item)",
    )
    .not_themeable(
        "menu items",
        "PopupMenu builds its own; geometry::menu_item has no receiver here (geometry.rs, menu/menu_item.rs: MenuItemElement is pub(crate)). An item is 26px tall at the default Size, rounded with the theme radius up to 8px (menu/popup_menu.rs, RenderOptions)",
    )
    .instance(
        "menu shortcut",
        "a Kbd, shown when the item's action has a key binding (menu/popup_menu.rs, binding_for_action_in)",
    )
}

/// Where the toolbar row's gap between items comes from: `stated` is the
/// `toolbar.item_gap` the theme states, `widget_gap` its
/// `layout.widget_gap`, and `own` the showcase's `TOOLBAR_GAP`.
pub struct ToolbarGap {
    pub stated: Option<f32>,
    pub widget_gap: Option<Pixels>,
    pub own: Pixels,
}

impl ToolbarGap {
    /// The gap line: the first of the three that is stated.
    fn text(&self) -> String {
        match (self.stated, self.widget_gap) {
            (Some(g), _) => format!("{}px, toolbar.item_gap", px_text(g)),
            (None, Some(g)) => format!(
                "{}px, layout.widget_gap: the theme states no toolbar.item_gap",
                px_text(g.as_f32())
            ),
            (None, None) => format!(
                "{}px, TOOLBAR_GAP, the showcase's own choice rather than a platform's: the theme states neither toolbar.item_gap nor layout.widget_gap, and the row is the application's own element, with no toolkit default to keep",
                px_text(self.own.as_f32())
            ),
        }
    }
}

/// The window's toolbar row (spec §2.3, §3.1, §3.3). Its geometry line is
/// recorded by `native_info` where `demo::toolbar` applies the builder;
/// `stated` is the `toolbar.border` padding the theme states, `margin` its
/// `layout.container_margin`, `own` the showcase's `TOOLBAR_PADDING`, and
/// `gap` where the gap between the items comes from.
pub fn toolbar(
    stated: ResolvedPadding,
    margin: Option<Pixels>,
    own: Pixels,
    gap: ToolbarGap,
) -> WidgetInfo {
    let side = |name: &str, value: Option<f32>| match (value, margin) {
        (Some(v), _) => format!("{name} {}px, toolbar.border's", px_text(v)),
        (None, Some(m)) => format!("{name} {}px, layout.container_margin", px_text(m.as_f32())),
        (None, None) => format!(
            "{name} {}px, TOOLBAR_PADDING, the showcase's own",
            px_text(own.as_f32())
        ),
    };
    WidgetInfo::new("Toolbar")
        .not_themeable(
            "widget",
            "gpui-component has no toolbar widget, so this row is the application's own h_flex: geometry::toolbar gives it the height, gap, fill and padding sides the theme states, and the showcase pads the sides and spaces the items where the theme leaves them unstated, which the padding and gap lines below name",
        )
        .not_themeable(
            "edge",
            "the model inherits toolbar.border.color and line_width from defaults.border, but platform-facts §2.13 states neither, so geometry::toolbar draws no edge -- an application that wants a rule draws a Separator",
        )
        .instance(
            "padding",
            format!(
                "{}; {}; {}; {}. A side toolbar.border leaves unstated takes layout.container_margin, and where that is unstated too, TOOLBAR_PADDING, {}px, the showcase's own choice rather than a platform's: the row is the application's own element, with no toolkit default to keep (spec §3.1)",
                side("top", stated.top),
                side("right", stated.right),
                side("bottom", stated.bottom),
                side("left", stated.left),
                px_text(own.as_f32()),
            ),
        )
        .instance("gap", gap.text())
        .instance(
            "items",
            "icon Buttons for the command palette, a theme reload and the Preferences sheet",
        )
}

/// The status bar's panel toggle (spec §3.2, S4), running `action` and
/// showing `icon` of the icon theme named `set`, selected while its panel is
/// `open`, which `state` describes. Its icon-size line is recorded where
/// `demo::panel_toggle` applies the builder.
pub fn panel_toggle(
    t: &Theme,
    action: &'static str,
    icon: &ChromeIcon,
    set: &str,
    open: bool,
    state: &'static str,
) -> WidgetInfo {
    let drawn = !matches!(icon, ChromeIcon::Missing(_) | ChromeIcon::Unlisted(_));
    let info = WidgetInfo::new("Button").variant(match (drawn, open) {
        (true, true) => "Ghost, icon, selected",
        (true, false) => "Ghost, icon",
        (false, true) => "Ghost, labelled, selected",
        (false, false) => "Ghost, labelled",
    });
    let info = if open {
        super::buttons::native_ghost_selected(info, t).not_themeable(
            "fill",
            "the ghost variant's active colour while selected, and no hover or press style: upstream paints a selected Button with its variant's selected style and hovers only a Button that is neither disabled nor selected (button/button.rs, ButtonVariant::selected; RenderOnce for Button)",
        )
    } else {
        super::buttons::native_ghost(info, t).not_themeable(
            "fill",
            "none until hovered: the custom variant's fill is transparent (button/button.rs, ButtonCustomVariant::new)",
        )
    };
    let info = info.colors(super::feedback::tooltip_colours(t, true));
    let info = if drawn {
        info.not_themeable(
            "icon",
            "a child of the Button, not its icon: Button::icon resizes whatever it is given to a size derived from the Button's own Size (button/button.rs, RenderOnce for Button), and an Icon's size set last wins over its own style (icon.rs, Icon::into_svg). As a child the icon keeps the size it was built at, and takes the text colour above",
        )
    } else {
        info
    };
    info.not_themeable(
        "size",
        "h_6 with px_2 at Size::Small -- rems, so the platform's font; with its icon a child rather than its icon it is laid out as a labelled Button, not a square icon Button (button/button.rs, RenderOnce for Button)",
    )
    .instance(
        "icon",
        chrome_icon_note(
            icon,
            set,
            "the toggle shows the tooltip's text as its label instead",
        ),
    )
    .not_themeable(
        "tooltip",
        "upstream's Tooltip, which the Button builds as it renders from the text and action tooltip_with_action stored (button/button.rs, RenderOnce for Button), so geometry::tooltip cannot reach it; it shows the action's key binding (tooltip.rs, Tooltip::action)",
    )
    .instance(
        "place",
        "at the end of the status bar on the side of the panel it controls, the pattern Zed's status bar follows; the selection, not a changing icon, shows whether the panel is open",
    )
    .instance("action", action)
    .instance("state", state)
}

/// The side panel (spec S2): the theme settings, a Separator and the
/// inspector. `container_margin` is the installed layout's, which pads the
/// settings. Its geometry line is recorded where `demo::side_panel` applies
/// the margin.
pub fn side_panel(container_margin: Option<Pixels>) -> WidgetInfo {
    WidgetInfo::new("Side panel")
        .not_themeable(
            "widget",
            "the application's own column of plain elements, not upstream's Sidebar: a Sidebar's children must implement SidebarItem (sidebar/mod.rs, SidebarItem), and neither the theme settings nor the inspector is an item",
        )
        .not_themeable(
            "fill",
            "none of its own: the window's background, which the showcase's root paints, shows through",
        )
        .instance(
            "holds",
            "from the top: the theme settings, a Separator, then the inspector -- its TabBar, and below it its content, which fills the rest of the panel's height and scrolls, the bar kept clear of it by geometry::scrollbar_gutter as on the content panel",
        )
        .instance(
            "padding",
            match container_margin {
                Some(margin) => format!(
                    "{}px around the theme settings, layout.container_margin, the padding the inspector's content takes too",
                    px_text(margin.as_f32()),
                ),
                None => "none around the theme settings: layout.container_margin, which pads them and the inspector's content, is unstated".to_string(),
            },
        )
        .instance(
            "width",
            "the first panel of the window's resizable group: LEFT_PANEL_WIDTH, 300px, the showcase's own default -- the model states no such width -- until its handle is dragged",
        )
        .instance(
            "hidden",
            "by the status bar's side-panel toggle, View > Toggle Side Panel or Ctrl+B (Cmd+B on macOS), the whole panel at once: no rail stays. Shown again, it takes the width it had",
        )
}

/// The theme settings (spec §3.1, §3.3, S2), each a label above its control.
/// `widget_gap` is the installed layout's, and `own` the showcase's
/// `THEME_SETTINGS_GAP`.
pub fn theme_settings(widget_gap: Option<Pixels>, own: Pixels) -> WidgetInfo {
    WidgetInfo::new("Theme settings")
        .not_themeable(
            "widget",
            "the application's own column of plain elements, in the side panel",
        )
        .instance(
            "rows",
            "Theme, the preset switch; Mode, the colour mode; Icon theme, the icon theme the chrome and the Icons page take their icons from. Each is a Label above its control, and each control takes the settings' width",
        )
        .instance(
            "gaps",
            match widget_gap {
                Some(gap) => format!(
                    "{}px between the rows and between each label and its control, layout.widget_gap. Where that is unstated, THEME_SETTINGS_GAP, {}px, the showcase's own choice rather than a platform's: the settings are the application's own element, with no toolkit default to keep (spec §3.1)",
                    px_text(gap.as_f32()),
                    px_text(own.as_f32()),
                ),
                None => format!(
                    "{}px between the rows and between each label and its control: THEME_SETTINGS_GAP, the showcase's own choice rather than a platform's, as layout.widget_gap is unstated and the settings are the application's own element, with no toolkit default to keep (spec §3.1)",
                    px_text(own.as_f32()),
                ),
            },
        )
        .instance(
            "padding",
            "none of its own: the side panel pads around them with layout.container_margin",
        )
}

/// What an Underline `TabBar` at `Size::Small` paints and how it is laid
/// out, under `variant`.
fn underline_tab_bar(t: &Theme, variant: &'static str) -> WidgetInfo {
    WidgetInfo::new("TabBar")
        .variant(variant)
        .color(claim(
            "text",
            "tab_foreground",
            t.tab_foreground,
            "gpui-component/tab/tab.rs:159",
        ))
        .color(claim(
            "hover text",
            "tab_active_foreground",
            t.tab_active_foreground,
            "gpui-component/tab/tab.rs:208",
        ))
        .color(claim(
            "active text",
            "tab_active_foreground",
            t.tab_active_foreground,
            "gpui-component/tab/tab.rs:254",
        ))
        .color(claim(
            "active underline",
            "primary",
            t.primary,
            "gpui-component/tab/tab_bar.rs:275",
        ))
        .color(claim(
            "bottom rule",
            "border",
            t.border,
            "gpui-component/tab/tab_bar.rs:512",
        ))
        .not_themeable(
            "tab fill",
            "the tab token has no reader anywhere in gpui-component or gpui-base: an inactive tab is transparent, and the connector writes the slot from tab.background_color for nothing (Tier U)",
        )
        .not_themeable(
            "fill",
            "none, on the bar or on a tab: an Underline bar is transparent, and marks the active tab with its sliding indicator, a 2px primary bar under the tab (tab/tab_bar.rs:268-275). The active tab's own 2px primary bottom border (tab/tab.rs:253-262) is transparent once the indicator is drawn (tab/tab.rs:681-686). tab_active and tab_bar are the Tab variant's (tab/tab_bar.rs, TabBar::render; tab/tab.rs, TabVariant::selected)",
        )
        .not_themeable(
            "corners",
            "square: an Underline bar and its tabs take no radius, whatever the theme's (tab/tab.rs, TabVariant::radius)",
        )
        .not_themeable(
            "spacing",
            "a per-Size gap between the tabs, on an inner row the bar's refinement does not reach; an Underline tab has no horizontal padding at all (tab/tab_bar.rs, TabBar::render; tab/tab.rs, TabVariant::inner_paddings). TabTheme states no spacing either",
        )
        .not_themeable(
            "height",
            "a per-Size literal set with .h() over the caller's style, but a Tab never sets min_h, which leaves tab.min_height a receiver. Nothing applies it: there is no geometry::tab -- our gap (tab/tab.rs, Tab::render)",
        )
}

/// The inspector's TabBar (spec §2.6, §4.4): chrome, so it reports itself,
/// unlike the content below it.
pub fn inspector_tab_bar(t: &Theme) -> WidgetInfo {
    underline_tab_bar(t, "Underline, small").instance(
        "tabs",
        "Widget, the info the pointer settled on, and Theme, what the theme and the window set that no widget carries",
    )
}

/// A TabBar's left and right padding: `container_margin`, the installed
/// layout's, where it states one.
pub fn tab_bar_padding(container_margin: Option<Pixels>) -> String {
    match container_margin {
        Some(margin) => format!(
            "left and right {}px, layout.container_margin, as the side panel's settings and the inspector's content take: upstream's Underline bar pads neither itself nor its tabs (tab/tab.rs, TabVariant::inner_paddings; tab/tab_bar.rs, TabBar::render), leaving the inset to its container. The bottom rule still runs the bar's full width",
            px_text(margin.as_f32()),
        ),
        None => "none: upstream's Underline bar pads neither itself nor its tabs (tab/tab.rs, TabVariant::inner_paddings; tab/tab_bar.rs, TabBar::render), and layout.container_margin, which the showcase would inset it by, is unstated".to_string(),
    }
}

/// The content panel's page TabBar (spec S3), with upstream's menu of every
/// tab.
pub fn page_tab_bar(t: &Theme) -> WidgetInfo {
    underline_tab_bar(t, "Underline, small, menu")
        .instance(
            "tabs",
            "one per page, the shown page's selected; a click dispatches ShowPage, the action the View menu's page items and the command palette's page entries run",
        )
        .not_themeable(
            "menu",
            "menu(true): after the tabs, an extra-small ghost Button with a caret opens a menu of every page, the shown one checked, whose rows show their pages too. Upstream adds that Button whether or not the tabs overflow, and the tabs scroll sideways where they do not fit (tab/tab_bar.rs, TabBar::render)",
        )
        .not_themeable(
            "own icons",
            super::own_icons("the menu Button's caret, ChevronDown (select.rs, Caret), and Check beside the shown page in its menu (menu/popup_menu.rs, PopupMenu)"),
        )
}

/// What an upstream Ghost Button (`.ghost()`) holds, which its content's
/// swatches are named after.
#[derive(Clone, Copy)]
pub(super) enum GhostContent {
    Text,
    Icon,
    TextAndIcon,
}

/// What an upstream Ghost Button paints at rest and hovered: no fill at
/// rest and its content in secondary_foreground; hovered, the fill
/// `ghost_hover` names and the content in accent_foreground.
pub(super) fn ghost_rest_and_hover(t: &Theme, content: GhostContent) -> Vec<ColorClaim> {
    let (rest, hovered) = match content {
        GhostContent::Text => (
            claim(
                "text",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/button/button.rs:964",
            ),
            claim(
                "hover text",
                "accent_foreground",
                t.accent_foreground,
                "gpui-component/button/button.rs:1141",
            ),
        ),
        GhostContent::Icon => (
            claim(
                "icon",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/button/button.rs:964",
            ),
            claim(
                "icon on hover",
                "accent_foreground",
                t.accent_foreground,
                "gpui-component/button/button.rs:1141",
            ),
        ),
        GhostContent::TextAndIcon => (
            claim(
                "text and icon",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/button/button.rs:964",
            ),
            claim(
                "text and icon on hover",
                "accent_foreground",
                t.accent_foreground,
                "gpui-component/button/button.rs:1141",
            ),
        ),
    };
    vec![rest, ghost_hover(t), hovered]
}

/// What an upstream Ghost Button paints at rest, hovered and pressed:
/// `ghost_rest_and_hover`, and a press fills it with button_active, the
/// content keeping its rest colour (button/button.rs, ButtonVariant::active).
pub(super) fn ghost_colours(t: &Theme, content: GhostContent) -> Vec<ColorClaim> {
    let mut claims = ghost_rest_and_hover(t, content);
    claims.push(claim(
        "pressed",
        "button_active",
        t.button_active,
        "gpui-component/button/button.rs:1180",
    ));
    claims
}

/// What a Ghost Button is filled with while hovered: accent, at half alpha
/// in dark mode (button/button.rs:1125-1131).
fn ghost_hover(t: &Theme) -> ColorClaim {
    if t.is_dark() {
        claim(
            "hover, at 50% (dark mode)",
            "accent",
            t.accent.opacity(0.5),
            "gpui-component/button/button.rs:1128",
        )
    } else {
        claim(
            "hover",
            "accent",
            t.accent,
            "gpui-component/button/button.rs:1126",
        )
    }
}

/// What `Theme::input_background()` paints, the fill of an Input and of
/// every widget styled like one: the window background in light mode, and in
/// dark mode an Oklab mix of 30% input and 70% transparent -- `mix_oklab`'s
/// factor is the first colour's share (theme/color.rs:44-49) -- read at
/// theme/mod.rs:379-384. The swatch is that mix, not input at full strength.
pub(super) fn input_background(t: &Theme) -> ColorClaim {
    if t.is_dark() {
        claim(
            "bg, 30% input mixed with 70% transparent",
            "input",
            t.input.mix_oklab(t.transparent, 0.3),
            "gpui-component/theme/mod.rs:381",
        )
    } else {
        claim(
            "bg",
            "background",
            t.background,
            "gpui-component/theme/mod.rs:383",
        )
    }
}

/// The theme settings' preset Combobox (spec §3.3), the showcase's preset
/// switch.
pub fn preset_combobox(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Combobox")
        .variant("searchable")
        .not_themeable(
            "own icons",
            super::own_icons("the caret's ChevronDown (select.rs, Caret), and in its open list Check beside the chosen row (searchable_list/item.rs, SearchableListItemElement) and Inbox while the search matches nothing (combobox.rs, ComboboxState::new)"),
        )
        .color(input_background(t))
        .color(claim(
            "trigger border",
            "input",
            t.input,
            "gpui-component/combobox.rs:992",
        ))
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:105",
        ))
        .color(claim(
            "popup bg",
            "popover",
            t.popover,
            "gpui-component/styled.rs:197",
        ))
        .color(claim(
            "row hover, accent at 70%",
            "accent",
            t.accent.opacity(0.7),
            "gpui-component/searchable_list/item.rs:114",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/combobox.rs:999",
        ))
        .not_themeable(
            "fill",
            "input_background(), as an Input's: the window background in light mode, and input mixed toward transparent in dark -- one accessor, two sources (theme/mod.rs, input_background)",
        )
        .not_themeable(
            "font colour",
            "carried as size and weight only. Upstream's input_style delivers muted_foreground to the trigger when disabled (input/input.rs, input_style) before this refinement lands on it (combobox.rs, render_trigger_container), and the selected-title child sets no colour to re-mute with (combobox.rs, default_trigger_body), so a carried colour would beat the disabled colour instead of yielding to it. Select, whose title child does re-mute, takes it (native-theme-gpui geometry.rs, geometry::combobox)",
        )
        .not_themeable(
            "rows",
            "SearchableList's own, not ListItems: a hovered row is accent at 70% and a selected one accent in full (searchable_list/item.rs, SearchableListItemElement)",
        )
        .not_themeable(
            "caret",
            "its colour is themed -- upstream paints it with muted_foreground (select.rs, Caret) -- and its size is not: Caret maps Size::Size into the same arm as Medium (select.rs, Caret::render), so combo_box.arrow_icon_size has no route at all. Tier U for the size",
        )
        .instance(
            "switch",
            "choosing a row installs that preset. The rows are default -- the desktop's own theme, read from the system and labelled with the preset it builds on -- and the presets meant for this platform, as Theme::list_presets_for_platform lists them; typing filters them by name or key",
        )
        .instance(
            "delegate",
            "SearchableListDelegate, implemented in this showcase (combobox.rs, Combobox)",
        )
}

/// The theme settings' colour-mode Select (spec §3.3): the Inputs page's
/// Select, choosing System, Light or Dark.
pub fn color_mode_select(t: &Theme) -> WidgetInfo {
    super::inputs::select(t)
        .instance(
            "modes",
            "System, Light and Dark; choosing one dispatches SetColorMode, the action the Theme menu's items run. System's row is the mode alone: the mode the desktop is in shows in the status bar's environment text while System is chosen",
        )
        .instance(
            "shows",
            "the mode chosen, whichever way it was chosen: here, from the Theme menu, from the command palette or by the --variant flag (Showcase::show_color_mode)",
        )
        .instance(
            "width",
            "the settings': the trigger takes it and truncates its text (select.rs, SelectState::render), so its text never widens it past the side panel",
        )
}

/// The theme settings' icon-theme Select (spec §3.3, S6): the Inputs page's
/// Select, choosing the icon theme.
pub fn icon_set_select(t: &Theme) -> WidgetInfo {
    super::inputs::select(t)
        .instance(
            "choices",
            "the icon theme the showcase loads its icons from: the preset's own where it names one, the system's, each installed freedesktop theme, gpui-component's built-in Lucide, and the bundled Lucide and Material",
        )
        .instance(
            "on a theme switch",
            "the rows are built again, the default row the installed preset's. A choice that follows the preset -- its default row, or the system's where the preset's own icon theme is not installed, as native_theme::icons::default_icon_choice decides -- becomes the new preset's; any other row the user picked stays chosen",
        )
        .instance(
            "follows the choice",
            "the Icons page's galleries, the chrome's own icons -- the toolbar's Command Palette, Reload System Theme and Preferences buttons, the status bar's side-panel toggle, the command palette's entries, and the icon of the Alert that reports a theme that failed to load -- the items of the Layout page's Sidebar samples, and every icon the pages' samples are given: the Buttons page's icon Buttons, its loading Button's spinner and its Toggles, the input groups' Search icon and Copy button, the Attachments, the Empty state, the Plain Marker, the Collapsible's toggle, the icon Stepper, the Dialog's and the AlertDialog's icons and the menu rows. Each shows the chosen icon theme's icon for its IconName -- gpui-component's own where its built-in icons are chosen -- and none where the icon theme has none: no other icon theme's icon stands in",
        )
        .instance(
            "raster sets",
            "on macOS and Windows the system icon sets come as pixels, not SVG -- native-theme's loaders there return IconData::Rgba (sficons.rs:110 and winicons.rs:173, native-theme 0.5.9) -- and an Icon draws only a path or SVG bytes (icon.rs, Icon::data), so with the system icon theme chosen there the chrome and the pages' samples show none of their icons",
        )
        .not_themeable(
            "upstream's icons",
            "the icons a widget names as it renders, with no setter: this Select's and the preset Combobox's caret (select.rs, Caret), the check mark on their chosen row (searchable_list/item.rs, SearchableListItemElement) and an empty list's Inbox (select.rs, SelectState::new; combobox.rs, ComboboxState::new); the page TabBar's menu Button's caret, a Caret too (button/button.rs, RenderOnce for Button) and the check mark on the shown page's row in its menu (menu/popup_menu.rs, PopupMenu); the window controls (title_bar.rs, ControlIcon); a Dialog's close button (dialog/dialog.rs, Dialog::render) and a Sheet's (sheet.rs, Sheet::render); the command palette's search icon and check mark (command/state.rs, CommandState); and in the Preferences sheet the search field's icon (setting/settings.rs, Settings) and the text scale's minus and plus (input/number_input.rs, NumberInput)",
        )
        .not_themeable(
            "why they stay",
            super::own_icons("those icons, and every icon a sample's widget draws of its own, which that sample's info names"),
        )
        .instance(
            "pages",
            "the samples on the other pages show the chosen icon theme's icons for the icons they are given, as the chrome does, and none where it has none; only the icons their widgets draw of their own stay gpui-component's",
        )
}

/// One of the toolbar's icon Buttons (spec §2.3), running `action` and
/// showing `icon` of the icon theme named `set`. Its icon-size line is
/// recorded where `demo::toolbar_button` applies the builder.
pub fn toolbar_button(t: &Theme, action: &'static str, icon: &ChromeIcon, set: &str) -> WidgetInfo {
    let drawn = !matches!(icon, ChromeIcon::Missing(_) | ChromeIcon::Unlisted(_));
    let info = WidgetInfo::new("Button").variant(if drawn {
        "Ghost, icon"
    } else {
        "Ghost, labelled"
    });
    let info = super::buttons::native_ghost(info, t)
        .colors(super::feedback::tooltip_colours(t, true))
        .not_themeable(
            "fill",
            "none until hovered: the custom variant's fill is transparent (button/button.rs, ButtonCustomVariant::new)",
        );
    let info = if drawn {
        info.not_themeable(
            "icon",
            "a child of the Button, not its icon: Button::icon resizes whatever it is given to a size derived from the Button's own Size (button/button.rs, RenderOnce for Button), and an Icon's size set last wins over its own style (icon.rs, Icon::into_svg). As a child the icon keeps the size it was built at, and takes the text colour above",
        )
        .not_themeable(
            "size",
            "h_8 with px_2p5 at the default Size -- rems, so the platform's font; with its icon a child rather than its icon it is laid out as a labelled Button, not a square icon Button (button/button.rs, RenderOnce for Button)",
        )
    } else {
        info.not_themeable(
            "size",
            "h_8 with px_2p5 at the default Size -- rems, so the platform's font (button/button.rs, RenderOnce for Button)",
        )
    };
    info.instance(
        "icon",
        chrome_icon_note(
            icon,
            set,
            "the button shows the tooltip's text as its label instead",
        ),
    )
        .not_themeable(
            "tooltip",
            "upstream's Tooltip, which the Button builds as it renders from the text and action tooltip_with_action stored (button/button.rs, RenderOnce for Button). The only way to hand a Button a Tooltip of one's own is its tooltip_builder, which has no public setter (button/button.rs, Button), so geometry::tooltip cannot reach it; it shows the action's key binding where one is bound (tooltip.rs, Tooltip::action)",
        )
        .instance("action", action)
}

/// What a chrome icon's info says it is: `icon` as the icon theme named
/// `set` gives it, and `absent` what its widget shows where the icon theme
/// has none.
pub(crate) fn chrome_icon_note(icon: &ChromeIcon, set: &str, absent: &str) -> String {
    match icon {
        ChromeIcon::Builtin(name) => format!(
            "gpui-component's own {name}: its built-in icons are the icon theme the theme settings' Icon theme Select chose"
        ),
        ChromeIcon::Loaded(name, _) => format!(
            "{set}'s icon for {name}, the icon theme the theme settings' Icon theme Select chose. It is drawn as every Icon is, as a mask in its widget's text colour (gpui-pre/window.rs, Window::paint_svg), so a coloured icon shows its shape only"
        ),
        ChromeIcon::Missing(name) => format!(
            "none: {set} holds no SVG for {name}, and no other icon theme's icon stands in -- {absent}"
        ),
        ChromeIcon::Unlisted(path) => format!(
            "none: {path} is not in the Icons page's gallery, so nothing of {set} was loaded for it, and no other icon theme's icon stands in -- {absent}"
        ),
    }
}

/// The window's `StatusBar` (spec §2.7). Its geometry line is recorded by
/// `native_info` where `demo::status_bar` applies the builder; `styled` is
/// whether it does, which decides the colour of the text.
pub fn status_bar(t: &Theme, styled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("StatusBar")
        .color(claim(
            "bg",
            "status_bar",
            t.status_bar,
            "gpui-component/status_bar.rs:93",
        ))
        .color(claim(
            "border",
            "status_bar_border",
            t.status_bar_border,
            "gpui-component/status_bar.rs:92",
        ));
    // geometry::status_bar paints the text with status_bar.font's colour,
    // which no ThemeColor field holds, over upstream's.
    let info = if styled {
        info
    } else {
        info.color(claim(
            "text",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/status_bar.rs:95",
        ))
    };
    info.not_themeable(
        "region gap",
        "gap_2, 0.5rem, between the regions: upstream sets it on the bar before refine_style (status_bar.rs:88, :96), so a refinement could replace it, but the model states no status-bar item gap and geometry::status_bar sets none",
    )
    .not_themeable(
        "item gap",
        "gap_2, 0.5rem, between the items of each region, on the region elements the refinement does not reach (status_bar.rs:84)",
    )
    .not_themeable(
        "edge",
        "border_t_1, a 1px rule along the top, which geometry::status_bar leaves as it is (status_bar.rs, StatusBar::render)",
    )
    .instance(
        "left",
        if cfg!(target_os = "linux") {
            "the side-panel toggle at the bar's left end, then the desktop native_theme::detect recognises in XDG_CURRENT_DESKTOP, or Unknown where it recognises none -- SystemTheme::from_system then asks the portal, then kdeglobals, and the preset names what it settled on; the installed preset and colour mode; the font the installed theme states as defaults.font, in the unit its source stated; the installed text-scale factor; and each installed accessibility preference that is set, by its field name"
        } else {
            "the side-panel toggle at the bar's left end, then the operating system; the installed preset and colour mode; the font the installed theme states as defaults.font, in the unit its source stated; the installed text-scale factor; and each installed accessibility preference that is set, by its field name"
        },
    )
    .instance(
        "right",
        "the title of what the inspector's Widget tab shows, where it shows one, at the bar's right end",
    )
    .instance(
        "text",
        "the environment and the shown title are plain text rather than Labels: a Label paints foreground on its own element (label.rs, Label), which would hide the colour geometry::status_bar gives the bar",
    )
}

/// A handle of the window's resizable group (spec §1.1, §4.3.5), between the
/// panels `between` names. `base` is gpui-base's theme, where the handle's
/// colours live.
pub fn resize_handle(base: &gpui_base::Theme, between: &'static str) -> WidgetInfo {
    WidgetInfo::new("ResizeHandle")
        .variant(between)
        .color(claim(
            "line",
            "handle",
            base.resizable.handle.unwrap_or(base.tokens.colors.border),
            "gpui-base/resizable/resize_handle.rs:293",
        ))
        .color(claim(
            "line while pressed",
            "active_handle",
            base.resizable
                .active_handle
                .unwrap_or(base.tokens.colors.ring),
            "gpui-base/resizable/resize_handle.rs:290",
        ))
        .instance(
            "colour source",
            "handle and active_handle are gpui-base's fields, always filled: upstream projects border and drag_border into them (theme/mod.rs, Theme::base_theme), and the connector then writes the platform's splitter.divider_color and splitter.hover_color over both (native-theme-gpui/base_layer.rs, resizable_theme). Only where the installed theme has no variant for the current colour mode are border and drag_border written back (native-theme-gpui/lib.rs, base_overrides_for). So the fallbacks gpui-base reads for an unset field, border at rest and ring while pressed, never apply here (gpui-base/resizable/resize_handle.rs, handle_color)",
        )
        .not_themeable(
            "hover",
            "none: a hovered handle keeps its resting colour, and the model's splitter.hover_color reaches it only while it is pressed -- a press inside the handle sets it and any release clears it (resizable/resize_handle.rs, ResizeHandleState)",
        )
        .not_themeable(
            "width",
            "a 1px line with a hit area of 4px on its left and 3px on its right: the handle is 1px wide with 4px of padding on each side, which layout widens to the 8px of its padding. Upstream's constants; the model's splitter.divider_width does not reach it (resizable/resize_handle.rs, HANDLE_SIZE)",
        )
        .instance(
            "drawn by",
            "the showcase, which with_handle_appearance hands the painted part of each handle to (resizable/panel.rs, with_handle_appearance): it paints the colour upstream would (resizable/resize_handle.rs, handle_color) and lays this info over the hit area. The drag and the cursor stay upstream's",
        )
        .instance(
            "drag",
            "moves the boundary to the pointer until a panel reaches PANEL_MIN_SIZE, 100px, the least a panel takes unless it sets a size range of its own (gpui-base/resizable/mod.rs, PANEL_MIN_SIZE; gpui-base/resizable/panel.rs, size_range); the body's panels set none. The side panel keeps the width it leaves when it hides and comes back",
        )
}

/// The Dialog's own colours, which every Dialog the showcase opens paints:
/// the surface, its edge, and the backdrop behind it; and what upstream
/// builds around the parts it is given. `close_button` is whether the Dialog
/// draws its close button, `reported` the parts that report it, and
/// `reduce_motion` gpui's reduced motion while it is drawn.
pub(super) fn dialog_surface(
    info: WidgetInfo,
    t: &Theme,
    reduce_motion: bool,
    close_button: bool,
    reported: &str,
) -> WidgetInfo {
    let info = if close_button {
        info.not_themeable(
            "own icons",
            super::own_icons("the close button's Close (dialog/dialog.rs, Dialog::render)"),
        )
    } else {
        info
    };
    info.color(claim(
        "bg",
        "background",
        t.background,
        "gpui-component/dialog/dialog.rs:613",
    ))
    .color(claim(
        "border",
        "border",
        t.border,
        "gpui-component/dialog/dialog.rs:615",
    ))
    .color(claim(
        "backdrop",
        "overlay",
        t.overlay,
        "gpui-component/dialog/dialog.rs:282",
    ))
    .not_themeable(
        "fill",
        "the window's own background, not the popover colour: a dialog is a surface, not a popup (dialog/dialog.rs, Dialog)",
    )
    .not_themeable(
        "width",
        "upstream's 448px default, the showcase setting none of its own (dialog/dialog.rs, DialogProps); where a native theme is installed geometry::dialog_max_width caps it",
    )
    .not_themeable(
        "animation",
        if reduce_motion {
            "none: reduced motion is on, so the Dialog appears at rest at once, where it would slide and fade in over 0.25s on a literal curve, not the theme's motion tokens (dialog/dialog.rs, ANIMATION_DURATION; gpui-pre/elements/animation.rs, AnimationExt)"
        } else {
            "a 0.25s slide and fade on a literal curve, not the theme's motion tokens (dialog/dialog.rs, ANIMATION_DURATION)"
        },
    )
    .instance(
        "unreported",
        if close_button {
            format!(
                "the surface's padding and its close button show no info of their own: upstream builds the surface, and the button on it, around what the showcase passes in, with no hook for an element of the caller's (dialog/dialog.rs, Dialog::render, lines 608-725), so only {reported} report"
            )
        } else {
            format!(
                "the surface's padding shows no info of its own: upstream builds the surface around what the showcase passes in, with no hook for an element of the caller's (dialog/dialog.rs, Dialog::render, lines 608-725), so only {reported} report"
            )
        },
    )
}

/// The command palette's Dialog (spec §2.8), drawn while gpui's
/// `reduce_motion` is as given. Its geometry lines are recorded where
/// `demo::command_palette` applies the builders; its info target is the
/// title, because the Command fills the rest of its content.
pub fn palette_dialog(t: &Theme, reduce_motion: bool) -> WidgetInfo {
    dialog_surface(
        WidgetInfo::new("Dialog").variant("Command Palette"),
        t,
        reduce_motion,
        true,
        "the title and the content",
    )
    .instance(
        "opens",
        "on OpenCommandPalette: View > Command Palette, Ctrl+K (Cmd+K on macOS), or the toolbar's Command Palette button",
    )
    .instance(
        "closes",
        "when an entry runs, on Escape with an empty query, from its close button, or on a click on the backdrop 34px or more below the window's top -- TITLE_BAR_HEIGHT, whether or not a title bar is drawn there (title_bar.rs:15; dialog/dialog.rs:586; gpui-base dialog.rs:601)",
    )
}

/// The command palette's `Command` (spec §2.8), unbordered inside its
/// Dialog, its entries' icons from the icon theme named `set`.
pub fn command_palette(t: &Theme, set: &str) -> WidgetInfo {
    WidgetInfo::new("Command")
        .not_themeable(
            "own icons",
            super::own_icons("Search before its query field, and Check beside a checked entry (command/state.rs, CommandState)"),
        )
        .color(claim(
            "surface bg",
            "popover",
            t.popover,
            "gpui-component/command/state.rs:830",
        ))
        .color(claim(
            "surface text",
            "popover_foreground",
            t.popover_foreground,
            "gpui-component/command/state.rs:831",
        ))
        .color(claim(
            "search divider",
            "border",
            t.border,
            "gpui-component/command/state.rs:847",
        ))
        .color(claim(
            "search icon",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/command/state.rs:852",
        ))
        .color(claim(
            "group label",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/command/state.rs:685",
        ))
        .color(claim(
            "row icon",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/command/state.rs:716",
        ))
        .color(claim(
            "highlighted row",
            "accent",
            t.accent,
            "gpui-component/command/state.rs:673",
        ))
        .color(claim(
            "highlighted row text",
            "accent_foreground",
            t.accent_foreground,
            "gpui-component/command/state.rs:674",
        ))
        .color(claim(
            "empty text",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/command/state.rs:785",
        ))
        .not_themeable(
            "geometry",
            "none: no geometry:: builder reaches it. Rows are text_sm with rem paddings, and the list is at most 18.75rem tall (command/command.rs, CommandOptions)",
        )
        .not_themeable(
            "edge",
            "none: bordered(false), as upstream advises for a palette inside a frame of its own such as a Dialog (command/command.rs, Command::bordered)",
        )
        .not_themeable(
            "query field",
            "an Input with appearance(false): it draws no background and no border of its own, so the surface shows through (command/state.rs, CommandState)",
        )
        .instance(
            "accent is the menu highlight",
            "the highlighted row takes the token a menu row's hover takes, which the connector fills from the platform's menu hover colour, so the palette follows the platform's menu selection rather than a list selection (native-theme-gpui/colors.rs, assign_core)",
        )
        .not_themeable(
            "scrollbar",
            "the list scrolls inside the Command, with a Scrollbar laid over it, on an element no refinement reaches, so geometry::scrollbar_gutter cannot keep the rows clear of the bar -- Tier U (command/state.rs, CommandState)",
        )
        .instance(
            "entries",
            "every page, then the presets the theme settings' preset switch offers -- default, and those Theme::list_presets_for_platform lists -- then the three colour modes. Each runs an action: ShowPage, SetPreset or SetColorMode",
        )
        .instance(
            "entry icons",
            format!("{set}'s, the icon theme the theme settings' Icon theme Select chose, in the row icon colour above; an entry whose icon that icon theme holds no SVG for shows its label alone, and no other icon theme's icon stands in. The search icon is upstream's own (command/state.rs, CommandState)"),
        )
        .instance(
            "keys",
            "typing filters the entries by label and keyword, Up and Down move the highlight, Enter runs the highlighted entry and closes the palette, and Escape clears the query, then closes it (command/state.rs, on_action_cancel)",
        )
}

/// What every `Sheet` the showcase opens paints -- at the `side` edge of the
/// window, drawn while gpui's `reduce_motion` is as given -- and what upstream
/// builds around its title, which alone reports it; `rest` names the part of
/// the surface that shows no info.
pub(super) fn sheet_surface(
    info: WidgetInfo,
    t: &Theme,
    side: SheetSide,
    reduce_motion: bool,
    rest: &str,
) -> WidgetInfo {
    let info = info
        .color(claim(
            "bg",
            "background",
            t.background,
            "gpui-component/sheet.rs:172",
        ))
        .color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/sheet.rs:173",
        ))
        .color(claim(
            "backdrop",
            "overlay",
            t.overlay,
            "gpui-component/dialog/dialog.rs:282",
        ))
        .not_themeable(
            "fill",
            "the window's own background, not the popover colour, as a Dialog's (sheet.rs, Sheet)",
        )
        .not_themeable(
            "own icons",
            super::own_icons("the close button's Close (sheet.rs, Sheet::render)"),
        );
    let info = match side {
        SheetSide::Right => info
            .not_themeable(
                "edge",
                "a 1px rule of border along its left edge, the one facing the window, a literal (sheet.rs, Sheet::render)",
            )
            .not_themeable(
                "top margin",
                "Theme::sheet.margin_top, a gpui-component setting the connector leaves at its default of TITLE_BAR_HEIGHT, 34px (sheet.rs, SheetSettings). native-theme states no sheet",
            ),
        SheetSide::Bottom => info
            .not_themeable(
                "edge",
                "a 1px rule of border along its top edge, the one facing the window, a literal (sheet.rs, Sheet::render)",
            )
            .not_themeable(
                "top margin",
                "none: a bottom Sheet sits on the window's bottom edge and spans its width, so Theme::sheet.margin_top does not reach it (sheet.rs, Sheet::render)",
            ),
    };
    info.not_themeable(
        "scrollbar",
        "the body scrolls on an element the Sheet builds itself, and a Sheet's refinement lands on its surface, not there -- Tier U (sheet.rs, Sheet)",
    )
    .not_themeable(
        "animation",
        if reduce_motion {
            "none: reduced motion is on, so the Sheet appears in place at once, where it would slide in over 0.15s, a literal, not the theme's motion tokens (sheet.rs, Sheet::render; gpui-pre/elements/animation.rs, AnimationExt)"
        } else {
            "a 0.15s literal slide, not the theme's motion tokens (sheet.rs, Sheet)"
        },
    )
    .instance(
        "unreported",
        format!(
            "only the title reports the Sheet: upstream builds the surface, its title row, padding and close button around what the showcase passes in, with no hook for an element of the caller's (sheet.rs, Sheet::render, lines 167-245), so {rest} shows no info"
        ),
    )
}

/// The Preferences `Sheet` (spec §2.8), drawn while gpui's `reduce_motion`
/// is as given; its info target is its title.
pub fn preferences_sheet(t: &Theme, reduce_motion: bool) -> WidgetInfo {
    sheet_surface(
        WidgetInfo::new("Sheet").variant("Preferences"),
        t,
        SheetSide::Right,
        reduce_motion,
        "the rest of the surface around the Settings",
    )
    .instance(
        "width",
        "PREFERENCES_WIDTH, the showcase's own: the model states no sheet",
    )
    .instance(
        "opens",
        "on OpenPreferences: Theme > Preferences…, Ctrl+, (Cmd+, on macOS) or the toolbar's Preferences button",
    )
}

/// What every `Settings` the showcase builds paints and cannot be given:
/// the Preferences sheet's and the Layout page's, named `variant`.
pub(super) fn settings(t: &Theme, variant: &'static str) -> WidgetInfo {
    WidgetInfo::new("Settings")
        .variant(variant)
        .color(claim(
            "sidebar bg",
            "sidebar",
            t.sidebar,
            "gpui-component/sidebar/mod.rs:413",
        ))
        .color(claim(
            "sidebar text",
            "sidebar_foreground",
            t.sidebar_foreground,
            "gpui-component/sidebar/mod.rs:414",
        ))
        .color(claim(
            "page item",
            "sidebar_accent",
            t.sidebar_accent,
            "gpui-component/sidebar/menu.rs:297",
        ))
        .color(claim(
            "header rule",
            "border",
            t.border,
            "gpui-component/setting/page.rs:185",
        ))
        .color(claim(
            "page description",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/setting/page.rs:219",
        ))
        .color(claim(
            "item description",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/setting/item.rs:299",
        ))
        .not_themeable(
            "group",
            "no fill and no edge: a Settings uses GroupBoxVariant::Normal unless with_group_variant picks another (setting/settings.rs, with_group_variant). A SettingGroup is not an element the showcase can wrap -- the page renders it (setting/group.rs, SettingGroup) -- so it reports here",
        )
        .not_themeable(
            "scrollbar",
            "the page lays its scrollbar over the body's right edge and takes no refinement -- Tier U (setting/page.rs, SettingPage); the group is padded by the platform's groove width instead (setting/group.rs, SettingGroup)",
        )
}

/// The Settings inside the Preferences sheet (spec §2.8). Its geometry line
/// is recorded where `demo::preferences` applies the builder to its group.
pub fn preferences_settings(t: &Theme) -> WidgetInfo {
    settings(t, "Preferences")
        .not_themeable(
            "own icons",
            super::own_icons("Search before its sidebar's search field (setting/settings.rs, Settings), Undo2 on the reset button a page shows while one of its settings can be reset (setting/page.rs, SettingPage), and Minus and Plus on the text scale's step buttons (setting/fields/number.rs, NumberField; input/number_input.rs, NumberInput)"),
        )
        .not_themeable(
            "layout",
            "the label above the field wherever the page is at most 480px wide, as it is in this sheet (setting/settings.rs, STACKED_LAYOUT_MAX_WIDTH)",
        )
        .instance(
            "preferences",
            "the four AccessibilityPreferences fields of the installed native theme; a change is installed with native_theme_gpui::apply_accessibility, which rebuilds the theme from its stored variant (native-theme-gpui lib.rs, apply_accessibility). The desktop's own settings are not changed, and switching or reloading the theme installs the desktop's preferences again",
        )
        .instance(
            "text scale",
            "1.0 to 2.25 in steps of 0.25: Windows' TextScaleFactor range (platform-facts §1.2.7) in the showcase's own steps; the model states no range",
        )
}

/// One of the Preferences sheet's switches, `checked` or not, `disabled`
/// while no native theme is installed.
pub fn preference_switch(
    t: &Theme,
    field: &'static str,
    checked: bool,
    disabled: bool,
) -> WidgetInfo {
    let info = WidgetInfo::new("Switch").variant(match (checked, disabled) {
        (_, true) => "disabled",
        (true, false) => "on",
        (false, false) => "off",
    });
    let info = match (checked, disabled) {
        (_, true) => info.not_themeable(
            "disabled",
            "the row at half opacity (setting/item.rs, SettingItem), and the track at half alpha within it (switch.rs, Switch)",
        ),
        (true, false) => info.color(claim(
            "track",
            "primary",
            t.primary,
            "gpui-component/switch.rs:139",
        )),
        (false, false) => info.color(claim(
            "track",
            "switch",
            t.switch,
            "gpui-component/switch.rs:140",
        )),
    };
    info.color(claim(
        "thumb",
        "switch_thumb",
        t.switch_thumb,
        "gpui-component/switch.rs:146",
    ))
    .not_themeable(
        "size",
        "Tier U: track and thumb are px literals per Size, on children of the wrapper a refinement lands on (switch.rs, Switch), while the model states switch.track_width, track_height and thumb_diameter",
    )
    .not_themeable(
        "on track",
        "primary by default; Switch::color would replace it, a receiver for the model's switch.checked_background that nothing in the connector feeds (switch.rs, Switch)",
    )
    .instance(
        "preference",
        match field {
            "reduce_motion" => "reduce_motion: a click installs the preferences with it flipped, and apply_accessibility forwards it to gpui's reduced motion",
            "high_contrast" => "high_contrast: a click installs the preferences with it flipped",
            _ => "reduce_transparency: a click installs the preferences with it flipped",
        },
    )
    .instance(
        "field",
        "the showcase's own Switch, built as upstream's switch field builds one (setting/fields/bool.rs, BoolField) but in a SettingField::render slot, so it can report itself",
    )
}

/// The About dialog (spec §2.8). Its geometry lines are recorded where
/// `demo::about` applies the builders; its title and its content report it.
/// `gapped` is whether the installed theme states the gap between its lines,
/// `reduce_motion` gpui's reduced motion while it is drawn.
pub fn about_dialog(t: &Theme, gapped: bool, reduce_motion: bool) -> WidgetInfo {
    dialog_surface(
        WidgetInfo::new("Dialog").variant("About"),
        t,
        reduce_motion,
        true,
        "the title and the content",
    )
    .instance(
            "content",
            "this crate's name and the version Cargo built it at. Upstream's versions are not repeated: the link opens the README's Compatibility table, which states them",
        )
        .instance(
            "line gap",
            if gapped {
                "layout.widget_gap, as the installed theme states it now"
            } else {
                "none: no installed theme states layout.widget_gap, and the showcase sets no gap of its own"
            },
        )
        .instance("opens", "on OpenAbout: Help > About")
}

/// The About dialog's link to the README's Compatibility table: the
/// Typography page's Link, pointing somewhere else.
pub fn about_link(t: &Theme) -> WidgetInfo {
    super::typography::link(t).instance(
        "target",
        "the connector README at this version's tag, v-prefixed as the text says, at its Compatibility heading. The tag exists once the version is released; before that the page is missing",
    )
}

/// The Alert that reports a theme that failed to load (spec §2.5): the
/// Feedback page's Error Alert, as a banner, showing `icon` of the icon theme
/// named `set`.
pub fn theme_error_alert(t: &Theme, icon: &ChromeIcon, set: &str) -> WidgetInfo {
    super::feedback::alert(t, Severity::Error, true)
        .instance(
            "message",
            "why the theme failed to load, as the loader reported it. The theme installed before stays, in the colour mode asked for, and the next theme that loads clears the Alert",
        )
        .instance(
            "icon",
            chrome_icon_note(
                icon,
                set,
                "an Alert always holds an Icon (alert.rs, Alert), so the showcase hands it an empty one, sized to nothing and invisible, which gpui never paints (gpui-pre/elements/div.rs, Interactivity::paint); the message keeps the gap an icon would have beside it",
            ),
        )
}
