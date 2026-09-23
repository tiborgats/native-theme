//! The window's chrome: title bar, menus, toolbar, navigation, status bar and overlays.

use gpui::{App, InteractiveElement as _, IntoElement, Menu, MenuItem, SharedString};
use gpui_component::IconName;
use native_theme_gpui::ActiveNativeTheme as _;

use crate::Page;
use crate::app::{
    AppColorMode, OpenAbout, OpenCommandPalette, OpenPreferences, Quit, ReloadTheme, SetColorMode,
    ShowPage, Showcase, ToggleInspector, ToggleSidebar,
};
use crate::support::defined_size;
use crate::{
    CHROME_SIDEBAR, CHROME_SIDEBAR_TOGGLE, CHROME_STATUS_BAR, CHROME_TITLE_BAR, CHROME_TOOLBAR,
    CHROME_TOOLBAR_INSPECTOR, PROBE_COLOR_MODE, PROBE_COMBOBOX, demo,
};

/// The showcase's application menus (spec §2.2), built fresh for each
/// consumer: gpui's `Menu` is handed over by value, and the platform bar and
/// `AppMenuBar` each take their own copy.
///
/// An item whose action nothing handles yet is disabled rather than left to
/// do nothing when chosen; the task that gives the action its handler
/// enables the item.
pub(crate) fn menus() -> Vec<Menu> {
    vec![
        Menu::new("File").items([MenuItem::action("Quit", Quit)]),
        Menu::new("View").items(
            Page::ALL
                .map(|page| MenuItem::action(page.label(), ShowPage(page.index())))
                .into_iter()
                .chain([
                    MenuItem::separator(),
                    MenuItem::action("Toggle Sidebar", ToggleSidebar),
                    MenuItem::action("Toggle Inspector", ToggleInspector),
                    // The command palette is Task 12's.
                    MenuItem::action("Command Palette", OpenCommandPalette).disabled(true),
                ]),
        ),
        Menu::new("Theme").items([
            MenuItem::action("Reload System Theme", ReloadTheme),
            MenuItem::separator(),
            MenuItem::action("System", SetColorMode(AppColorMode::System)),
            MenuItem::action("Light", SetColorMode(AppColorMode::Light)),
            MenuItem::action("Dark", SetColorMode(AppColorMode::Dark)),
            MenuItem::separator(),
            // The preferences dialog is Task 12's.
            MenuItem::action("Preferences…", OpenPreferences).disabled(true),
        ]),
        // The About dialog is Task 12's.
        Menu::new("Help").items([MenuItem::action("About", OpenAbout).disabled(true)]),
    ]
}

/// The window's title bar (spec §2.1): the installed preset and colour mode,
/// and the application's menus where the platform has no menu bar of its own.
pub(crate) fn title_bar(app: &Showcase, cx: &App) -> impl IntoElement {
    let (preset, mode) = preset_and_mode(app);
    demo::title_bar(
        &app.info_ui,
        cx,
        format!("native-theme showcase — {preset} ({mode})"),
        app.menu_bar.clone(),
    )
    .debug_selector(|| CHROME_TITLE_BAR.into())
}

/// The installed preset, the default one by the platform preset it stands
/// for, and the colour mode, as the title bar and the status bar name them.
fn preset_and_mode(app: &Showcase) -> (&str, &str) {
    let preset = if app.current_theme_name == "default" {
        app.default_label.as_str()
    } else {
        app.current_theme_name.as_str()
    };
    (preset, if app.is_dark { "dark" } else { "light" })
}

/// The window's toolbar (spec §2.3), under the title bar: the Sidebar's
/// toggle, the preset switch, the colour mode, the icon set, and buttons for
/// three of the actions.
///
/// A button whose action nothing handles yet is disabled, as its menu item
/// is, and says why.
pub(crate) fn toolbar(app: &Showcase, cx: &App) -> impl IntoElement {
    let ui = &app.info_ui;
    demo::toolbar(
        ui,
        cx,
        [
            demo::sidebar_toggle_button(ui, cx, app.nav_collapsed)
                .debug_selector(|| CHROME_SIDEBAR_TOGGLE.into())
                .into_any_element(),
            demo::preset_combobox(ui, cx, &app.preset_combobox)
                .debug_selector(|| PROBE_COMBOBOX.into())
                .into_any_element(),
            demo::color_mode_toggle_group(ui, cx, app.color_mode)
                .debug_selector(|| PROBE_COLOR_MODE.into())
                .into_any_element(),
            demo::icon_set_select(ui, cx, &app.icon_set_select).into_any_element(),
            demo::toolbar_separator(ui, cx).into_any_element(),
            demo::toolbar_button(
                ui,
                cx,
                demo::ToolbarButton {
                    button_id: "toolbar-command-palette",
                    icon: IconName::SquareTerminal,
                    tooltip: "Command Palette",
                    action: &OpenCommandPalette,
                    about: "OpenCommandPalette's button: the action behind View > Command Palette and Ctrl+K",
                    disabled: Some(
                        "disabled in this build: nothing handles OpenCommandPalette yet, and a button that did nothing when pressed would misstate what the showcase does",
                    ),
                },
            )
            .into_any_element(),
            demo::toolbar_button(
                ui,
                cx,
                demo::ToolbarButton {
                    button_id: "toolbar-reload-theme",
                    icon: IconName::RotateCw,
                    tooltip: "Reload System Theme",
                    action: &ReloadTheme,
                    about: "dispatches ReloadTheme, the action Theme > Reload System Theme runs: the desktop's settings are read again and the current theme is installed from them",
                    disabled: None,
                },
            )
            .into_any_element(),
            demo::toolbar_button(
                ui,
                cx,
                demo::ToolbarButton {
                    button_id: "toolbar-inspector",
                    icon: IconName::Inspector,
                    tooltip: "Toggle Inspector",
                    action: &ToggleInspector,
                    about: "dispatches ToggleInspector, the action View > Toggle Inspector and Ctrl+I run: the inspector's panel is hidden, or shown again",
                    disabled: None,
                },
            )
            .debug_selector(|| CHROME_TOOLBAR_INSPECTOR.into())
            .into_any_element(),
        ],
    )
    .debug_selector(|| CHROME_TOOLBAR.into())
}

/// The window's Sidebar (spec §2.4): the pages, the shown one active.
pub(crate) fn sidebar(app: &Showcase, cx: &App) -> impl IntoElement {
    demo::sidebar(&app.info_ui, cx, app.active_page, app.nav_collapsed)
        .debug_selector(|| CHROME_SIDEBAR.into())
}

/// The window's status bar (spec §2.7), below the body: the environment on
/// the left; on the right `shown`, the title of what the inspector shows, and
/// the version of the crate this example belongs to.
pub(crate) fn status_bar(
    app: &Showcase,
    cx: &App,
    shown: Option<SharedString>,
) -> impl IntoElement {
    demo::status_bar(
        &app.info_ui,
        cx,
        status_environment(app, cx).join(" · "),
        shown,
        concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION")),
    )
    .debug_selector(|| CHROME_STATUS_BAR.into())
}

/// The status bar's left side (spec §2.7), one item each: the desktop, the
/// preset and colour mode, the installed theme's `defaults.font` in the unit
/// its source stated (`Showcase::original_font`, a placeholder where no
/// theme could be read), and, once a native theme is installed, its
/// text-scale factor and each of its accessibility preferences that is set,
/// by its field name. Before that no preferences are installed, and none are
/// named.
pub(crate) fn status_environment(app: &Showcase, cx: &App) -> Vec<String> {
    let (preset, mode) = preset_and_mode(app);
    let font = &app.original_font;
    let mut items = vec![
        desktop(),
        format!("{preset} {mode}"),
        format!("{} {}", font.family, defined_size(font)),
    ];
    if let Some(prefs) = cx.native_theme().map(|nt| nt.accessibility()) {
        items.push(format!("text ×{}", prefs.text_scaling_factor));
        let flags = [
            ("reduce_motion", prefs.reduce_motion),
            ("high_contrast", prefs.high_contrast),
            ("reduce_transparency", prefs.reduce_transparency),
        ];
        items.extend(
            flags
                .into_iter()
                .filter(|(_, set)| *set)
                .map(|(name, _)| name.to_string()),
        );
    }
    items
}

/// The desktop `native_theme::detect` recognises in `XDG_CURRENT_DESKTOP`, or
/// `Unknown` where it recognises none. `SystemTheme::from_system` then asks
/// the portal, then `kdeglobals` (native-theme pipeline.rs, `select_reader`),
/// and the preset names what it settled on.
#[cfg(target_os = "linux")]
fn desktop() -> String {
    format!("{:?}", native_theme::detect::detect_linux_desktop())
}

/// The operating system: `native_theme::detect` names a desktop on Linux
/// only.
#[cfg(not(target_os = "linux"))]
fn desktop() -> String {
    std::env::consts::OS.to_string()
}
