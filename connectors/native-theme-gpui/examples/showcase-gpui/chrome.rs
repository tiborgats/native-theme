//! The window's chrome: title bar, menus, toolbar, navigation, status bar and overlays.

use gpui::{
    App, InteractiveElement as _, IntoElement, Menu, MenuItem, Pixels, SharedString, Window, px,
};
use gpui_component::{
    IconName, WindowExt as _,
    command::{CommandGroup, CommandItem},
};
use native_theme_gpui::ActiveNativeTheme as _;

use crate::Page;
use crate::app::{
    AppColorMode, OpenAbout, OpenCommandPalette, OpenPreferences, Quit, ReloadTheme, SetColorMode,
    SetPreset, ShowPage, Showcase, ToggleInspector, ToggleSidebar,
};
use crate::support::{defined_size, preset_items};
use crate::{
    CHROME_SIDEBAR, CHROME_SIDEBAR_TOGGLE, CHROME_STATUS_BAR, CHROME_TITLE_BAR, CHROME_TOOLBAR,
    CHROME_TOOLBAR_INSPECTOR, CHROME_TOOLBAR_PALETTE, PROBE_COLOR_MODE, PROBE_COMBOBOX, demo,
};

/// The showcase's application menus (spec §2.2), built fresh for each
/// consumer: gpui's `Menu` is handed over by value, and the platform bar and
/// `AppMenuBar` each take their own copy.
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
                    MenuItem::action("Command Palette", OpenCommandPalette),
                ]),
        ),
        Menu::new("Theme").items([
            MenuItem::action("Reload System Theme", ReloadTheme),
            MenuItem::separator(),
            MenuItem::action("System", SetColorMode(AppColorMode::System)),
            MenuItem::action("Light", SetColorMode(AppColorMode::Light)),
            MenuItem::action("Dark", SetColorMode(AppColorMode::Dark)),
            MenuItem::separator(),
            MenuItem::action("Preferences…", OpenPreferences),
        ]),
        Menu::new("Help").items([MenuItem::action("About", OpenAbout)]),
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
                    about: "dispatches OpenCommandPalette, the action View > Command Palette and Ctrl+K run: the command palette opens",
                },
            )
            .debug_selector(|| CHROME_TOOLBAR_PALETTE.into())
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

// ---------------------------------------------------------------------------
// Overlays (spec §2.8)
// ---------------------------------------------------------------------------

/// The connector README's Compatibility table, at the tag of this version:
/// the upstream versions the About dialog does not repeat. `#compatibility`
/// is the anchor of the README's `## Compatibility` heading, which the
/// README's own opening lines link to.
pub(crate) const COMPATIBILITY_URL: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    "/blob/v",
    env!("CARGO_PKG_VERSION"),
    "/connectors/native-theme-gpui/README.md#compatibility"
);

/// The width of the Preferences sheet. The model states no sheet, so no
/// such width either (spec §1.3): this is the showcase's own layout default.
/// It overrides upstream's 350px (sheet.rs:63) because the Settings inside
/// starts its sidebar at 250px (setting/settings.rs:54), which would leave
/// the preferences 100px.
const PREFERENCES_WIDTH: Pixels = px(600.);

/// The presets the command palette offers, as `(key, display name)`: the
/// toolbar's preset switch's, from the same list (`support::preset_items`).
pub(crate) fn palette_presets() -> Vec<(SharedString, SharedString)> {
    preset_items()
        .into_iter()
        .map(|item| (item.key, item.display_name))
        .collect()
}

/// The command palette's entries (spec §2.8): every page, every preset the
/// toolbar offers, and the three colour modes, each running the action its
/// menu item or toolbar control runs.
fn palette_groups() -> Vec<CommandGroup> {
    let pages = CommandGroup::new()
        .label("Pages")
        .items(Page::ALL.map(|page| {
            CommandItem::new()
                .label(page.label())
                .icon(page.icon())
                .action(Box::new(ShowPage(page.index())))
        }));
    let presets = CommandGroup::new()
        .label("Presets")
        .items(palette_presets().into_iter().map(|(key, name)| {
            CommandItem::new()
                .label(name)
                .keywords([key.clone()])
                .icon(IconName::Palette)
                .action(Box::new(SetPreset(key)))
        }));
    let modes = CommandGroup::new().label("Colour mode").items(
        [
            AppColorMode::System,
            AppColorMode::Light,
            AppColorMode::Dark,
        ]
        .map(|mode| {
            CommandItem::new()
                .label(mode.label())
                .icon(match mode {
                    AppColorMode::System => IconName::Settings,
                    AppColorMode::Light => IconName::Sun,
                    AppColorMode::Dark => IconName::Moon,
                })
                .action(Box::new(SetColorMode(mode)))
        }),
    );
    vec![pages, presets, modes]
}

/// Whether a Dialog or a Sheet is open. Upstream stacks a new Dialog over any
/// open one (root.rs, Root::open_dialog pushes a layer each time), so the
/// three overlays open only while none is: a second Ctrl+K would otherwise
/// lay a second palette over the first, on the same state. A modal keeps the
/// application's other windows out until it is dismissed, as a desktop
/// application's does.
fn an_overlay_is_open(window: &mut Window, cx: &mut App) -> bool {
    window.has_active_dialog(cx) || window.has_active_sheet(cx)
}

/// Open the command palette, its query cleared and focused, unless an
/// overlay is open.
pub(crate) fn open_command_palette(app: &Showcase, window: &mut Window, cx: &mut App) {
    if an_overlay_is_open(window, cx) {
        return;
    }
    let (ui, state) = (app.info_ui.clone(), app.palette_state.clone());
    state.update(cx, |state, cx| state.set_query("", window, cx));
    let groups = palette_groups();
    window.open_dialog(cx, move |dialog, _window, cx| {
        demo::command_palette(&ui, cx, dialog, &state, groups.clone())
    });
    // After the Dialog took the focus for itself (root.rs, Root::open_dialog),
    // so typing reaches the query at once.
    app.palette_state
        .clone()
        .update(cx, |state, cx| state.focus(window, cx));
}

/// Open the Preferences sheet, unless an overlay is open.
pub(crate) fn open_preferences(app: &Showcase, window: &mut Window, cx: &mut App) {
    if an_overlay_is_open(window, cx) {
        return;
    }
    let ui = app.info_ui.clone();
    window.open_sheet(cx, move |sheet, _window, cx| {
        demo::preferences(&ui, cx, sheet, PREFERENCES_WIDTH)
    });
}

/// This crate's name and version, as the About dialog states them.
pub(crate) const ABOUT_NAME_VERSION: &str =
    concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

/// Open the About dialog, unless an overlay is open. Its lines are
/// `app.overlay_gap` apart, read as each frame draws it, so a theme switched
/// while it is open reaches it.
pub(crate) fn open_about(app: &Showcase, window: &mut Window, cx: &mut App) {
    if an_overlay_is_open(window, cx) {
        return;
    }
    let (ui, gap) = (app.info_ui.clone(), app.overlay_gap.clone());
    window.open_dialog(cx, move |dialog, _window, cx| {
        demo::about(
            &ui,
            cx,
            dialog,
            ABOUT_NAME_VERSION,
            COMPATIBILITY_URL,
            gap.get(),
        )
    });
}
