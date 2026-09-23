//! What the window's chrome reports about itself (spec §2).

use gpui_component::{Colorize as _, theme::Theme};

use super::{ColorClaim, WidgetInfo, claim};
use crate::demo::{SeparatorKind, Severity};

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
        .color(ghost_hover(t))
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
            "ghost Buttons, so they hover with accent -- at half alpha in dark mode -- rather than the button family, and their label is the Ghost variant's secondary_foreground",
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

/// The window's toolbar row (spec §2.3). Its geometry line is recorded by
/// `native_info` where `demo::toolbar` applies the builder.
pub fn toolbar() -> WidgetInfo {
    WidgetInfo::new("Toolbar")
        .not_themeable(
            "widget",
            "gpui-component has no toolbar widget, so this row is the application's own h_flex, and geometry::toolbar is the whole of its geometry: nothing after it sets a height, padding, gap or fill",
        )
        .not_themeable(
            "edge",
            "the model inherits toolbar.border.color and line_width from defaults.border, but platform-facts §2.13 states neither, so geometry::toolbar draws no edge -- an application that wants a rule draws a Separator",
        )
        .instance(
            "items",
            "the SidebarToggleButton, the preset Combobox, the colour-mode ToggleGroup, the icon-set Select, a vertical Separator, and icon Buttons for the command palette, a theme reload and the inspector",
        )
}

/// The toolbar's `SidebarToggleButton` (spec §2.3, §2.4), drawn `collapsed`
/// while the Sidebar is.
pub fn sidebar_toggle_button(t: &Theme, collapsed: bool) -> WidgetInfo {
    WidgetInfo::new("SidebarToggleButton")
        .color(claim(
            "icon",
            "secondary_foreground",
            t.secondary_foreground,
            "gpui-component/button/button.rs:964",
        ))
        .color(ghost_hover(t))
        .color(claim(
            "icon on hover",
            "accent_foreground",
            t.accent_foreground,
            "gpui-component/button/button.rs:1141",
        ))
        .not_themeable(
            "button",
            "a ghost, small Button the widget builds and keeps to itself: nothing outside it refines, disables or gives a tooltip to it (sidebar/mod.rs, SidebarToggleButton)",
        )
        .not_themeable(
            "fill",
            "none until hovered: a ghost Button is transparent, and it hovers with accent -- at half alpha in dark mode -- rather than the button family (button/button.rs, ButtonVariant::hovered)",
        )
        .not_themeable(
            "icon",
            "PanelLeftClose, or PanelLeftOpen while collapsed, at size_4 -- 1rem, so the platform's font size. toolbar.icon_size does not reach it: the widget gives its Button the icon as it renders (sidebar/mod.rs, SidebarToggleButton)",
        )
        .instance(
            "action",
            "dispatches ToggleSidebar, the action View > Toggle Sidebar and Ctrl+B run: the Sidebar collapses to its icons, or expands again",
        )
        .instance(
            "state",
            if collapsed {
                "the Sidebar is collapsed"
            } else {
                "the Sidebar is expanded"
            },
        )
}

/// The window's `Sidebar` (spec §2.4), which navigates between the pages.
pub fn sidebar(t: &Theme, collapsed: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Sidebar")
        .color(claim(
            "bg",
            "sidebar",
            t.sidebar,
            "gpui-component/sidebar/mod.rs:413",
        ))
        .color(claim(
            "text",
            "sidebar_foreground",
            t.sidebar_foreground,
            "gpui-component/sidebar/mod.rs:414",
        ))
        .color(claim(
            "border",
            "sidebar_border",
            t.sidebar_border,
            "gpui-component/sidebar/mod.rs:415",
        ))
        .not_themeable(
            "width",
            "SidebarTheme states no width -- our model's gap. A width that is not an absolute pixel length leaves the Sidebar without its animated wrapper (sidebar/mod.rs, sidebar_expanded_width), so expanded it fills its resizable panel, and collapsed it is upstream's fixed 48px (sidebar/mod.rs, COLLAPSED_WIDTH)",
        )
        .not_themeable(
            "padding",
            "px_3 around the items, p_2 while collapsed -- rems, so the platform's font -- and not settable: upstream drops the caller's padding before it lays the Sidebar out (sidebar/mod.rs, RenderOnce for Sidebar)",
        )
        .instance(
            "pages",
            "one SidebarMenuItem per page with the page's icon; the shown page's item is active, and a click dispatches ShowPage, the action the View menu's page items run",
        )
        .instance(
            "children",
            "must implement SidebarItem, which asks for Collapsible + Clone (sidebar/mod.rs, SidebarItem)",
        );
    if collapsed {
        info.instance(
            "collapsed",
            "to its icons, by the toolbar's SidebarToggleButton, View > Toggle Sidebar or Ctrl+B. An icon rail is out of the resizable group, so no handle resizes it",
        )
    } else {
        info.instance(
            "width",
            "the first panel of the window's resizable group: NAV_WIDTH, the showcase's own default, until its handle is dragged",
        )
    }
}

/// The Sidebar's item for the page labelled `page`, `active` while that page
/// is shown and `collapsed` while the Sidebar is. Its icon-size line is
/// recorded where the item is built.
pub fn sidebar_item(t: &Theme, page: &'static str, active: bool, collapsed: bool) -> WidgetInfo {
    let info = WidgetInfo::new("SidebarMenuItem").variant(if active {
        format!("{page}, active")
    } else {
        page.to_string()
    });
    let info = if active {
        info.color(claim(
            "bg",
            "sidebar_accent",
            t.sidebar_accent,
            "gpui-component/sidebar/menu.rs:297",
        ))
        .color(claim(
            "text",
            "sidebar_accent_foreground",
            t.sidebar_accent_foreground,
            "gpui-component/sidebar/menu.rs:298",
        ))
    } else {
        info.color(claim(
            "text (the Sidebar's)",
            "sidebar_foreground",
            t.sidebar_foreground,
            "gpui-component/sidebar/mod.rs:414",
        ))
        .color(claim(
            "hover bg, at 80%",
            "sidebar_accent",
            t.sidebar_accent.opacity(0.8),
            "gpui-component/sidebar/menu.rs:291",
        ))
        .color(claim(
            "hover text",
            "sidebar_accent_foreground",
            t.sidebar_accent_foreground,
            "gpui-component/sidebar/menu.rs:292",
        ))
    };
    let info = info
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable(
            "hover",
            "sidebar_accent at 80%, the selection colour: the model's sidebar.hover_background reaches no slot, because the connector reads it nowhere (sidebar/menu.rs, SidebarMenuItem)",
        )
        .not_themeable(
            "font",
            "text_sm, and font_medium while active -- sidebar.font's size and weight have no route, as no geometry:: builder carries them (sidebar/menu.rs, SidebarMenuItem)",
        )
        .not_themeable(
            "height",
            "h_7 while expanded, set after the caller's refinement, so nothing reaches it -- rems, so the platform's font. SidebarTheme states no row height (sidebar/menu.rs, SidebarMenuItem)",
        )
        .instance(
            "page",
            format!("a click dispatches ShowPage for the {page} page, the action View > {page} runs"),
        );
    if collapsed {
        info.instance(
            "collapsed",
            "only the icon shows, and the label becomes a tooltip at its right (sidebar/menu.rs, collapsed_tooltip)",
        )
    } else {
        info
    }
}

/// The inspector's TabBar (spec §2.6, §4.4): chrome, so it reports itself,
/// unlike the content below it.
pub fn inspector_tab_bar(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("TabBar")
        .variant("Underline, small")
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
            "gpui-component/tab/tab.rs:260",
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
            "none, on the bar or on a tab: an Underline bar is transparent and marks the active tab with a primary underline. tab_active and tab_bar are the Tab variant's (tab/tab_bar.rs, TabBar::render; tab/tab.rs, TabVariant::selected)",
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
        .instance(
            "tabs",
            "Widget, the info the pointer settled on, and Theme, what the theme and the window set that no widget carries",
        )
}

/// What a Ghost Button is filled with while hovered: accent, at half alpha
/// in dark mode (button/button.rs:1125-1131).
pub(super) fn ghost_hover(t: &Theme) -> ColorClaim {
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

/// The toolbar's preset Combobox (spec §2.3), the showcase's preset switch.
pub fn preset_combobox(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Combobox")
        .variant("searchable")
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

/// The toolbar's System / Light / Dark switch (spec §2.3).
pub fn color_mode_toggle_group(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("ToggleGroup")
        .variant("outline, segmented")
        .color(claim(
            "checked bg",
            "accent",
            t.accent,
            "gpui-component/button/toggle.rs:155",
        ))
        .color(claim(
            "checked text",
            "accent_foreground",
            t.accent_foreground,
            "gpui-component/button/toggle.rs:156",
        ))
        .color(claim(
            "unchecked bg",
            "background",
            t.background,
            "gpui-component/button/toggle.rs:198",
        ))
        .color(claim(
            "outline",
            "border",
            t.border,
            "gpui-component/button/toggle.rs:197",
        ))
        .color(claim(
            "hover bg",
            "accent",
            t.accent,
            "gpui-component/button/toggle.rs:202",
        ))
        .color(claim(
            "hover text",
            "accent_foreground",
            t.accent_foreground,
            "gpui-component/button/toggle.rs:203",
        ))
        .not_themeable(
            "checked fill",
            "accent, the menu highlight, by default -- but not out of reach: a Toggle folds the caller's refinement into its checked style too (button/toggle.rs, Toggle::render), so an application that refines a checked Toggle with segmented_control.active_background and active_text_color gets them. Nothing applies them: there is no geometry::toggle -- our gap",
        )
        .not_themeable(
            "size",
            "min_w_8 / h_8 at the default Size -- rems, so the platform's font -- and settable: the refinement comes last, so segmented_control.segment_height, its padding and its font would reach a Toggle through the geometry::toggle nobody has written (button/toggle.rs, Toggle::render)",
        )
        .instance(
            "modes",
            "System, Light and Dark; a click dispatches SetColorMode, the action the Theme menu's items run. System names the mode the desktop is in",
        )
        .instance(
            "segmented",
            "the toggles share one outline, and upstream drops the gap it leaves between separate toggles (button/toggle.rs, ToggleGroup::segmented)",
        )
}

/// The toolbar's icon-set Select (spec §2.3).
pub fn icon_set_select(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Select")
        .color(input_background(t))
        .color(claim(
            "upstream trigger text",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:105",
        ))
        .color(claim(
            "trigger border",
            "input",
            t.input,
            "gpui-component/select.rs:541",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/select.rs:548",
        ))
        .not_themeable(
            "fill",
            "input_background(), as an Input's: the window background in light mode, and input mixed toward transparent in dark -- one accessor, two sources (theme/mod.rs, input_background)",
        )
        .not_themeable(
            "caret",
            "its colour is themed -- upstream paints it with muted_foreground (select.rs, Caret) -- and its size is not: Caret maps Size::Size into the same arm as Medium (select.rs, Caret::render), so combo_box.arrow_icon_size has no route at all. Tier U for the size",
        )
        .instance(
            "choices",
            "the icon set the showcase loads its icons from: the preset's own where it names one, the system's, each installed freedesktop theme, gpui-component's built-in Lucide, and the bundled Lucide and Material",
        )
}

/// The Separator between the toolbar's controls and its icon Buttons.
pub fn toolbar_separator(t: &Theme) -> WidgetInfo {
    super::layout::separator(t, SeparatorKind::Vertical).not_themeable(
        "width",
        "none of its own: a vertical Separator's box sets only its height, and the 1px line overflows it (separator.rs, Separator::vertical), so the toolbar's item_gap is the space on either side",
    )
}

/// One of the toolbar's icon Buttons (spec §2.3), running `action`. Its
/// icon-size line is recorded where `demo::toolbar_button` applies the
/// builder.
pub fn toolbar_button(t: &Theme, action: &'static str) -> WidgetInfo {
    WidgetInfo::new("Button")
        .variant("Ghost, icon")
        .color(claim(
            "icon",
            "secondary_foreground",
            t.secondary_foreground,
            "gpui-component/button/button.rs:964",
        ))
        .color(ghost_hover(t))
        .color(claim(
            "icon on hover",
            "accent_foreground",
            t.accent_foreground,
            "gpui-component/button/button.rs:1141",
        ))
        .color(claim(
            "tooltip bg",
            "popover",
            t.popover,
            "gpui-component/tooltip.rs:114",
        ))
        .color(claim(
            "tooltip text",
            "popover_foreground",
            t.popover_foreground,
            "gpui-component/tooltip.rs:115",
        ))
        .color(claim(
            "tooltip border",
            "border",
            t.border,
            "gpui-component/tooltip.rs:118",
        ))
        .not_themeable(
            "fill",
            "none: a Ghost Button is transparent until hovered (button/button.rs, ButtonVariant::hovered)",
        )
        .not_themeable(
            "icon",
            "a child of the Button, not its icon: Button::icon resizes whatever it is given to a size derived from the Button's own Size (button/button.rs, RenderOnce for Button), and an Icon's size set last wins over its own style (icon.rs, Icon::into_svg). As a child the icon keeps the size it was built at",
        )
        .not_themeable(
            "size",
            "h_8 with px_2p5 at the default Size -- rems, so the platform's font; with its icon a child rather than its icon it is laid out as a labelled Button, not a square icon Button (button/button.rs, RenderOnce for Button)",
        )
        .not_themeable(
            "tooltip",
            "upstream's Tooltip, which the Button builds as it renders from the text and action tooltip_with_action stored (button/button.rs, RenderOnce for Button). The only way to hand a Button a Tooltip of one's own is its tooltip_builder, which has no public setter (button/button.rs, Button), so geometry::tooltip cannot reach it; it shows the action's key binding where one is bound (tooltip.rs, Tooltip::action)",
        )
        .instance("action", action)
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
        "gap_2, 0.5rem, between the regions and between the items of each, on region children the refinement does not reach (status_bar.rs, StatusBar::render); the model states no status-bar item gap",
    )
    .not_themeable(
        "edge",
        "border_t_1, a 1px rule along the top, which geometry::status_bar leaves as it is (status_bar.rs, StatusBar::render)",
    )
    .instance(
        "left",
        if cfg!(target_os = "linux") {
            "the desktop native_theme::detect recognises in XDG_CURRENT_DESKTOP, or Unknown where it recognises none -- SystemTheme::from_system then asks the portal, then kdeglobals, and the preset names what it settled on; the preset and colour mode the title bar names; the font the installed theme states as defaults.font, in the unit its source stated; the installed text-scale factor; and each installed accessibility preference that is set, by its field name"
        } else {
            "the operating system; the preset and colour mode the title bar names; the font the installed theme states as defaults.font, in the unit its source stated; the installed text-scale factor; and each installed accessibility preference that is set, by its field name"
        },
    )
    .instance(
        "right",
        "the title of what the inspector's Widget tab shows, where it shows one, then this crate's name and version",
    )
    .instance(
        "text",
        "plain text rather than Labels: a Label paints foreground on its own element (label.rs, Label), which would hide the colour geometry::status_bar gives the bar",
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
            "moves the boundary to the pointer until a panel reaches PANEL_MIN_SIZE, 100px, the least a panel takes unless it sets a size range of its own (gpui-base/resizable/mod.rs, PANEL_MIN_SIZE; gpui-base/resizable/panel.rs, size_range); the body's panels set none. The panels keep the widths it leaves when the Sidebar collapses or the inspector hides and comes back",
        )
}

/// The Dialog's own colours, which every Dialog the showcase opens paints:
/// the surface, its edge, and the backdrop behind it.
fn dialog_surface(info: WidgetInfo, t: &Theme) -> WidgetInfo {
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
        "a 0.25s slide and fade on a literal curve, not the theme's motion tokens (dialog/dialog.rs, ANIMATION_DURATION)",
    )
    .instance(
        "unreported",
        "the surface's padding and its close button show no info of their own: upstream builds the surface, and the button on it, around what the showcase passes in, with no hook for an element of the caller's (dialog/dialog.rs, Dialog::render, lines 608-725), so only the title and the content report",
    )
}

/// The command palette's Dialog (spec §2.8). Its geometry lines are recorded
/// where `demo::command_palette` applies the builders; its info target is the
/// title, because the Command fills the rest of its content.
pub fn palette_dialog(t: &Theme) -> WidgetInfo {
    dialog_surface(WidgetInfo::new("Dialog").variant("Command Palette"), t)
        .instance(
            "opens",
            "on OpenCommandPalette: View > Command Palette, Ctrl+K, or the toolbar's Command Palette button",
        )
        .instance(
            "closes",
            "when an entry runs, on Escape with an empty query, from its close button, or on a click on the backdrop below the title bar (dialog/dialog.rs, Dialog::render)",
        )
}

/// The command palette's `Command` (spec §2.8), unbordered inside its Dialog.
pub fn command_palette(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Command")
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
            "every page, then the presets the toolbar's preset switch offers -- default, and those Theme::list_presets_for_platform lists -- then the three colour modes. Each runs an action: ShowPage, SetPreset or SetColorMode",
        )
        .instance(
            "keys",
            "typing filters the entries by label and keyword, Up and Down move the highlight, Enter runs the highlighted entry and closes the palette, and Escape clears the query, then closes it (command/state.rs, on_action_cancel)",
        )
}

/// The Preferences `Sheet` (spec §2.8); its info target is its title.
pub fn preferences_sheet(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Sheet")
        .variant("Preferences")
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
            "top margin",
            "Theme::sheet.margin_top, a gpui-component setting the connector leaves at its default of TITLE_BAR_HEIGHT, 34px (sheet.rs, SheetSettings). native-theme states no sheet",
        )
        .not_themeable(
            "scrollbar",
            "the body scrolls on an element the Sheet builds itself, and a Sheet's refinement lands on its surface, not there -- Tier U (sheet.rs, Sheet)",
        )
        .not_themeable(
            "animation",
            "a 0.15s literal slide, not the theme's motion tokens (sheet.rs, Sheet)",
        )
        .instance(
            "unreported",
            "only the title reports the Sheet: upstream builds the surface, its title row, padding and close button around what the showcase passes in, with no hook for an element of the caller's (sheet.rs, Sheet::render, lines 167-245), so the rest of the surface around the Settings shows no info",
        )
        .instance(
            "width",
            "PREFERENCES_WIDTH, the showcase's own: the model states no sheet",
        )
        .instance(
            "opens",
            "on OpenPreferences: Theme > Preferences… or Ctrl+,",
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
/// `gapped` is whether the installed theme states the gap between its lines.
pub fn about_dialog(t: &Theme, gapped: bool) -> WidgetInfo {
    dialog_surface(WidgetInfo::new("Dialog").variant("About"), t)
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
/// Feedback page's Error Alert, as a banner.
pub fn theme_error_alert(t: &Theme) -> WidgetInfo {
    super::feedback::alert(t, Severity::Error, true).instance(
        "message",
        "why the theme failed to load, as the loader reported it. The theme installed before stays, in the colour mode asked for, and the next theme that loads clears the Alert",
    )
}
