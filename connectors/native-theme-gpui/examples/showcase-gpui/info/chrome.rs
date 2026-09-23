//! What the window's chrome reports about itself (spec §2).

use gpui_component::theme::Theme;

use super::{ColorClaim, WidgetInfo, claim};

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
            "File, View, Theme and Help; each item runs a gpui action, and where the item has a key binding, the binding runs the same action. An item whose action has no handler yet is disabled",
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

/// What an input-styled trigger is filled with: upstream's input_style
/// (input/input.rs:105) takes `input_background()`, which reads a different
/// field in each mode (theme/mod.rs:379-384).
fn trigger_fill(t: &Theme) -> ColorClaim {
    if t.is_dark() {
        claim(
            "trigger bg (input mixed toward transparent)",
            "input",
            t.input,
            "gpui-component/theme/mod.rs:381",
        )
    } else {
        claim(
            "trigger bg",
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
        .color(trigger_fill(t))
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
            "row hover",
            "accent",
            t.accent,
            "gpui-component/searchable_list/item.rs:114",
        ))
        .color(claim(
            "focus ring",
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
        .color(trigger_fill(t))
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
            "focus ring",
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
    WidgetInfo::new("Separator")
        .variant("vertical")
        .color(claim(
            "line",
            "border",
            t.border,
            "gpui-component/separator.rs:128",
        ))
        .not_themeable(
            "thickness",
            "Tier U, not an absence: the platform states separator.line_width and the model carries it. Upstream draws the line on an inner absolutely-positioned div at px(1.) and applies the caller's refinement to the outer container instead, so nothing reaches the line (separator.rs, Separator::render_base)",
        )
        .not_themeable(
            "width",
            "none of its own: a vertical Separator's box sets only its height, and the 1px line overflows it (separator.rs, Separator::vertical), so the toolbar's item_gap is the space on either side",
        )
}

/// One of the toolbar's icon Buttons (spec §2.3), running `action`.
/// `disabled` is why the button is disabled, where nothing handles its
/// action yet. Its icon-size line is recorded where `demo::toolbar_button`
/// applies the builder.
pub fn toolbar_button(
    t: &Theme,
    action: &'static str,
    disabled: Option<&'static str>,
) -> WidgetInfo {
    let info = WidgetInfo::new("Button").variant("Ghost, icon");
    let info = if disabled.is_some() {
        info.color(claim(
            "disabled icon",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/button/button.rs:1284",
        ))
    } else {
        info.color(claim(
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
    };
    let info = info
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
            "none: a Ghost Button is transparent until hovered, and a disabled one stays transparent while its icon takes muted_foreground at half opacity (button/button.rs, ButtonVariant::disabled)",
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
        .instance("action", action);
    match disabled {
        Some(why) => info.instance("disabled", why),
        None => info,
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
        "gap_2, 0.5rem, between the regions and between the items of each, on region children the refinement does not reach (status_bar.rs, StatusBar::render); the model states no status-bar item gap",
    )
    .not_themeable(
        "edge",
        "border_t_1, a 1px rule along the top, which geometry::status_bar leaves as it is (status_bar.rs, StatusBar::render)",
    )
    .instance(
        "left",
        if cfg!(target_os = "linux") {
            "the desktop native_theme::detect reads from XDG_CURRENT_DESKTOP, as SystemTheme::from_system does to pick its reader; the preset and colour mode the title bar names; the platform font in the unit its source stated; the installed text-scale factor; and each installed accessibility preference that is set, by its field name"
        } else {
            "the operating system; the preset and colour mode the title bar names; the platform font in the unit its source stated; the installed text-scale factor; and each installed accessibility preference that is set, by its field name"
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
            "moves the boundary to the pointer; the panels keep the widths it leaves when the Sidebar collapses or the inspector hides and comes back",
        )
}
