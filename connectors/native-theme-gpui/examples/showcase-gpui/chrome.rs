//! The window's chrome: title bar or menu-bar row, menus, toolbar, side panel, page tabs, status bar and overlays.

use gpui::{
    Action, AnyElement, App, Decorations, InteractiveElement as _, IntoElement, Menu, MenuItem,
    Pixels, SharedString, Window, px,
};
use gpui_component::{IconName, WindowExt as _};
use native_theme_gpui::{ActiveNativeTheme as _, geometry};

use crate::Page;
use crate::app::{
    AppColorMode, OpenAbout, OpenCommandPalette, OpenPreferences, Quit, ReloadTheme, SetColorMode,
    SetPreset, ShowPage, Showcase, ToggleSidePanel,
};
use crate::demo::{SeparatorKind, TabBarKind};
use crate::support::{defined_size, preset_items};
use crate::{
    CHROME_LABEL_ICON_THEME, CHROME_LABEL_MODE, CHROME_LABEL_THEME, CHROME_MENU_BAR,
    CHROME_PAGE_TABS, CHROME_SIDE_PANEL, CHROME_SIDE_PANEL_SEPARATOR, CHROME_SIDE_PANEL_TOGGLE,
    CHROME_STATUS_BAR, CHROME_TITLE_BAR, CHROME_TOOLBAR, CHROME_TOOLBAR_PALETTE,
    CHROME_TOOLBAR_PREFERENCES, CHROME_TOOLBAR_RELOAD, PROBE_COLOR_MODE, PROBE_COMBOBOX,
    PROBE_ICON_THEME, WINDOW_TITLE, demo, demo::PaletteEntry,
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
                    MenuItem::action("Toggle Side Panel", ToggleSidePanel),
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

/// The top of the window, by the decorations it was granted, `frame` (spec
/// S8). Where the window manager draws the frame, the menu-bar row -- none on
/// macOS, whose menus are in the system's menu bar. Where it leaves the
/// frame to the application, the window's TitleBar.
pub(crate) fn window_top(app: &Showcase, frame: Decorations, cx: &App) -> Option<AnyElement> {
    match frame {
        Decorations::Client { .. } => Some(title_bar(app, cx).into_any_element()),
        Decorations::Server if cfg!(target_os = "macos") => None,
        Decorations::Server => Some(menu_bar(app, cx).into_any_element()),
    }
}

/// The window's title bar under client-side decorations (spec §2.1, §3.4,
/// S8): this crate's name and version, the window's title, and the
/// application's menus where the platform has no menu bar of its own.
fn title_bar(app: &Showcase, cx: &App) -> impl IntoElement {
    demo::title_bar(&app.info_ui, cx, WINDOW_TITLE, app.menu_bar.clone())
        .debug_selector(|| CHROME_TITLE_BAR.into())
}

/// The menu-bar row under the window manager's frame (spec S8): the
/// application's menus at the top of the window, padded by the installed
/// layout's `container_margin`, as the toolbar under it is.
fn menu_bar(app: &Showcase, cx: &App) -> impl IntoElement {
    demo::menu_bar(
        &app.info_ui,
        cx,
        geometry::container_margin(&app.layout),
        app.menu_bar.clone(),
    )
    .debug_selector(|| CHROME_MENU_BAR.into())
}

/// The installed preset, the default one by the platform preset it stands
/// for, and the colour mode, as the status bar names them.
fn preset_and_mode(app: &Showcase) -> (&str, &str) {
    let preset = if app.current_theme_name == "default" {
        app.default_label.as_str()
    } else {
        app.current_theme_name.as_str()
    };
    (preset, if app.is_dark { "dark" } else { "light" })
}

/// The window's toolbar (spec §2.3, §3.3, S8), under the title bar or the
/// menu-bar row: buttons for three of the actions -- the command palette, a
/// theme reload and the Preferences sheet -- their icons of the chosen icon
/// theme.
pub(crate) fn toolbar(app: &Showcase, cx: &App) -> impl IntoElement {
    let ui = &app.info_ui;
    let set = app.icon_set_label();
    demo::toolbar(
        ui,
        cx,
        geometry::container_margin(&app.layout),
        [
            demo::toolbar_button(
                ui,
                cx,
                demo::ToolbarButton {
                    button_id: "toolbar-command-palette",
                    icon: IconName::SquareTerminal,
                    drawn: app.chrome_icon(&IconName::SquareTerminal),
                    set: &set,
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
                    drawn: app.chrome_icon(&IconName::RotateCw),
                    set: &set,
                    tooltip: "Reload System Theme",
                    action: &ReloadTheme,
                    about: "dispatches ReloadTheme, the action Theme > Reload System Theme runs: the desktop's settings are read again and the current theme is installed from them",
                },
            )
            .debug_selector(|| CHROME_TOOLBAR_RELOAD.into())
            .into_any_element(),
            demo::toolbar_button(
                ui,
                cx,
                demo::ToolbarButton {
                    button_id: "toolbar-preferences",
                    icon: IconName::Settings,
                    drawn: app.chrome_icon(&IconName::Settings),
                    set: &set,
                    tooltip: "Preferences",
                    action: &OpenPreferences,
                    about: "dispatches OpenPreferences, the action Theme > Preferences… and Ctrl+, run: the Preferences sheet opens",
                },
            )
            .debug_selector(|| CHROME_TOOLBAR_PREFERENCES.into())
            .into_any_element(),
        ],
    )
    .debug_selector(|| CHROME_TOOLBAR.into())
}

/// The side panel (spec S2): the theme settings -- the preset switch, the
/// colour mode and the icon theme, each labelled -- padded by the installed
/// layout's `container_margin`, as the inspector's content is; a Separator;
/// then the inspector, which fills the rest of the panel's height.
pub(crate) fn side_panel(app: &Showcase, cx: &App) -> impl IntoElement {
    let ui = &app.info_ui;
    let margin = geometry::container_margin(&app.layout);
    let settings = demo::theme_settings(
        ui,
        cx,
        geometry::widget_gap(&app.layout),
        [
            (
                CHROME_LABEL_THEME,
                "Theme",
                demo::preset_combobox(ui, cx, &app.preset_combobox)
                    .debug_selector(|| PROBE_COMBOBOX.into())
                    .into_any_element(),
            ),
            (
                CHROME_LABEL_MODE,
                "Mode",
                demo::color_mode_select(ui, cx, &app.color_mode_select)
                    .debug_selector(|| PROBE_COLOR_MODE.into())
                    .into_any_element(),
            ),
            (
                CHROME_LABEL_ICON_THEME,
                "Icon theme",
                demo::icon_set_select(ui, cx, &app.icon_set_select)
                    .debug_selector(|| PROBE_ICON_THEME.into())
                    .into_any_element(),
            ),
        ],
    );
    demo::side_panel(
        ui,
        margin,
        settings,
        demo::separator(
            ui,
            cx,
            CHROME_SIDE_PANEL_SEPARATOR,
            SeparatorKind::Horizontal,
        ),
        app.inspector.clone(),
    )
    .debug_selector(|| CHROME_SIDE_PANEL.into())
}

/// The content panel's TabBar (spec S3): a tab per page, the shown one
/// selected, and a click shows its page.
pub(crate) fn page_tabs(app: &Showcase, cx: &App) -> impl IntoElement {
    demo::tab_bar(
        &app.info_ui,
        cx,
        TabBarKind::Pages,
        Page::ALL.map(|page| (page.label(), page.tab())),
        app.active_page.index(),
        |ix: &usize, window: &mut Window, cx: &mut App| {
            window.dispatch_action(Box::new(ShowPage(*ix)), cx)
        },
    )
    .debug_selector(|| CHROME_PAGE_TABS.into())
}

/// The window's status bar (spec §2.7, §3.2, S4), below the body: at its
/// left end the side-panel toggle, then the environment; at its right end
/// `shown`, the title of what the inspector shows. The toggle sits at the
/// edge of the panel it controls, and is selected while that panel is shown.
pub(crate) fn status_bar(
    app: &Showcase,
    cx: &App,
    shown: Option<SharedString>,
) -> impl IntoElement {
    let ui = &app.info_ui;
    let set = app.icon_set_label();
    let visible = app.side_panel_visible;
    let toggle = demo::panel_toggle(
        ui,
        cx,
        demo::PanelToggle {
            button_id: "status-toggle-side-panel",
            icon: IconName::PanelLeft,
            drawn: app.chrome_icon(&IconName::PanelLeft),
            set: &set,
            tooltip: "Toggle Side Panel",
            action: &ToggleSidePanel,
            open: visible,
            about: "dispatches ToggleSidePanel, the action View > Toggle Side Panel and Ctrl+B run: the side panel -- the theme settings and the inspector -- is hidden, or shown again at the width it had",
            state: if visible {
                "selected: the side panel is shown"
            } else {
                "not selected: the side panel is hidden"
            },
        },
    )
    .debug_selector(|| CHROME_SIDE_PANEL_TOGGLE.into());
    demo::status_bar(
        ui,
        cx,
        toggle,
        status_environment(app, cx).join(" · "),
        shown,
    )
    .debug_selector(|| CHROME_STATUS_BAR.into())
}

/// The status bar's environment text (spec §2.7), one item each: the desktop, the
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
/// starts its sidebar at 250px (setting/settings.rs:54): a 350px sheet, less
/// its 1px left border (sheet.rs:186) and the body's 16px padding on each
/// side (sheet.rs:147, 216-217), holds 317px, which would leave the
/// preferences at most 67px beside the sidebar.
const PREFERENCES_WIDTH: Pixels = px(600.);

/// The presets the command palette offers, as `(key, display name)`: the
/// preset switch's, from the same list (`support::preset_items`).
pub(crate) fn palette_presets() -> Vec<(SharedString, SharedString)> {
    preset_items()
        .into_iter()
        .map(|item| (item.key, item.display_name))
        .collect()
}

/// One entry of the command palette, its icon of `app`'s chosen icon theme.
fn palette_entry(
    app: &Showcase,
    label: impl Into<SharedString>,
    keywords: Vec<SharedString>,
    icon: IconName,
    action: Box<dyn Action>,
) -> PaletteEntry {
    PaletteEntry {
        label: label.into(),
        keywords,
        drawn: app.chrome_icon(&icon),
        icon,
        action,
    }
}

/// The command palette's entries (spec §2.8), as `(group, entries)`: every
/// page, every preset the preset switch offers, and the three colour modes,
/// each running the action its menu item or theme setting runs, their icons
/// of `app`'s chosen icon theme.
fn palette_groups(app: &Showcase) -> Vec<(&'static str, Vec<PaletteEntry>)> {
    let pages = Page::ALL
        .map(|page| {
            palette_entry(
                app,
                page.label(),
                Vec::new(),
                page.icon(),
                Box::new(ShowPage(page.index())),
            )
        })
        .into_iter()
        .collect();
    let presets = palette_presets()
        .into_iter()
        .map(|(key, name)| {
            palette_entry(
                app,
                name,
                vec![key.clone()],
                IconName::Palette,
                Box::new(SetPreset(key)),
            )
        })
        .collect();
    let modes = [
        AppColorMode::System,
        AppColorMode::Light,
        AppColorMode::Dark,
    ]
    .map(|mode| {
        let icon = match mode {
            AppColorMode::System => IconName::Settings,
            AppColorMode::Light => IconName::Sun,
            AppColorMode::Dark => IconName::Moon,
        };
        palette_entry(
            app,
            mode.label(),
            Vec::new(),
            icon,
            Box::new(SetColorMode(mode)),
        )
    })
    .into_iter()
    .collect();
    vec![
        ("Pages", pages),
        ("Presets", presets),
        ("Colour mode", modes),
    ]
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
    let groups = palette_groups(app);
    let set = SharedString::from(app.icon_set_label());
    window.open_dialog(cx, move |dialog, _window, cx| {
        demo::command_palette(&ui, cx, dialog, &state, groups.clone(), set.clone())
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
const ABOUT_NAME_VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

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
