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
            "the preset Combobox, the colour-mode ToggleGroup, the icon-set Select, a vertical Separator, and icon Buttons for the command palette, a theme reload and the inspector",
        )
}

/// The toolbar's preset Combobox (spec §2.3), the showcase's preset switch.
pub fn preset_combobox(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Combobox")
        .variant("searchable")
        .color(claim(
            "trigger bg",
            "background",
            t.background,
            "gpui-component/theme/mod.rs:383",
        ))
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
        .color(claim(
            "trigger bg",
            "background",
            t.background,
            "gpui-component/theme/mod.rs:383",
        ))
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
        .color(claim(
            "hover",
            "accent",
            t.accent,
            "gpui-component/button/button.rs:1126",
        ))
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
            "upstream's Tooltip, built inside the Button (button/button.rs, Button::tooltip_with_action), so geometry::tooltip cannot reach it; it shows the action's key binding where one is bound (tooltip.rs, Tooltip::action)",
        )
        .instance("action", action);
    match disabled {
        Some(why) => info.instance("disabled", why),
        None => info,
    }
}
