//! The window's chrome: title bar, menus, toolbar, navigation, status bar and overlays.

use gpui::{App, InteractiveElement as _, IntoElement, Menu, MenuItem};

use crate::Tab;
use crate::app::{
    AppColorMode, OpenAbout, OpenCommandPalette, OpenPreferences, Quit, ReloadTheme, SetColorMode,
    ShowPage, Showcase, ToggleInspector, ToggleSidebar,
};
use crate::{CHROME_TITLE_BAR, demo};

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
            Tab::ALL
                .map(|tab| MenuItem::action(tab.label(), ShowPage(tab.index())))
                .into_iter()
                .chain([
                    MenuItem::separator(),
                    // The sidebar and the inspector are Task 10's, the
                    // command palette Task 12's.
                    MenuItem::action("Toggle Sidebar", ToggleSidebar).disabled(true),
                    MenuItem::action("Toggle Inspector", ToggleInspector).disabled(true),
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
    let preset = if app.current_theme_name == "default" {
        app.default_label.as_str()
    } else {
        app.current_theme_name.as_str()
    };
    let mode = if app.is_dark { "dark" } else { "light" };
    demo::title_bar(
        &app.info_ui,
        cx,
        format!("native-theme showcase — {preset} ({mode})"),
        app.menu_bar.clone(),
    )
    .debug_selector(|| CHROME_TITLE_BAR.into())
}
