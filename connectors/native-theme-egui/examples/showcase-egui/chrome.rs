//! The chrome: menu bar, toolbar, status bar, side panel, page tabs, overlays (spec §10.4).

use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme::theme::IconRole;
use native_theme_egui::{
    DialogButtonOrder, NativeThemeUiExt as _, PanelSide, Role, RoleVariant, Surface,
    dialog_button_order, window_title_bar_font, window_title_bar_text_color,
};

use crate::{
    LEFT_PANEL_WIDTH,
    app::{App, ModeChoice, Page, ThemeChoice},
    demo::{self, Registry, Seam},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Action {
    ShowPage(Page),
    ToggleSidePanel,
    OpenCommandPalette,
    ReloadTheme,
    SetMode(ModeChoice),
    OpenPreferences,
    OpenAbout,
    Quit,
}

impl Action {
    /// The menus, in order; `None` is a separator (the gpui showcase's menus,
    /// `connectors/native-theme-gpui/examples/showcase-gpui/chrome.rs:28-52`).
    pub(crate) const MENUS: &[(&str, &[Option<Action>])] = &[
        ("File", &[Some(Action::Quit)]),
        (
            "View",
            &[
                Some(Action::ShowPage(Page::Buttons)),
                Some(Action::ShowPage(Page::Selection)),
                Some(Action::ShowPage(Page::Inputs)),
                Some(Action::ShowPage(Page::Range)),
                Some(Action::ShowPage(Page::Text)),
                Some(Action::ShowPage(Page::Colour)),
                Some(Action::ShowPage(Page::Containers)),
                Some(Action::ShowPage(Page::Data)),
                Some(Action::ShowPage(Page::Overlays)),
                Some(Action::ShowPage(Page::Icons)),
                Some(Action::ShowPage(Page::ThemeMap)),
                None,
                Some(Action::ToggleSidePanel),
                Some(Action::OpenCommandPalette),
            ],
        ),
        (
            "Theme",
            &[
                Some(Action::ReloadTheme),
                None,
                Some(Action::SetMode(ModeChoice::System)),
                Some(Action::SetMode(ModeChoice::Light)),
                Some(Action::SetMode(ModeChoice::Dark)),
                None,
                Some(Action::OpenPreferences),
            ],
        ),
        ("Help", &[Some(Action::OpenAbout)]),
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Action::ShowPage(page) => page.label(),
            Action::ToggleSidePanel => "Toggle Side Panel",
            Action::OpenCommandPalette => "Command Palette",
            Action::ReloadTheme => "Reload System Theme",
            Action::SetMode(ModeChoice::System) => "System",
            Action::SetMode(ModeChoice::Light) => "Light",
            Action::SetMode(ModeChoice::Dark) => "Dark",
            Action::OpenPreferences => "Preferences…",
            Action::OpenAbout => "About",
            Action::Quit => "Quit",
        }
    }

    /// `Modifiers::COMMAND` is Ctrl, and Cmd on macOS (`egui/src/data/input/modifiers.rs:115`).
    pub(crate) fn shortcut(self) -> Option<egui::KeyboardShortcut> {
        let key = match self {
            Action::Quit => egui::Key::Q,
            Action::ToggleSidePanel => egui::Key::B,
            Action::OpenCommandPalette => egui::Key::K,
            Action::OpenPreferences => egui::Key::Comma,
            _ => return None,
        };
        Some(egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, key))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct PaletteState {
    pub query: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InspectorTab {
    Widget,
    Theme,
}

/// The chrome bar: `Role::Toolbar` live on the root `Ui` while the panel is
/// shown, and again inside it (§4.4's recipe, §6.18); the panel's frame is the
/// top panel surface fed from `theme.toolbar`.
pub(crate) fn chrome_bar(app: &mut App, ui: &mut egui::Ui) {
    ui.native_set_style(Role::Toolbar, RoleVariant::Normal);
    let frame = ui.native_frame(Surface::Panel(PanelSide::Top));
    let out = egui::Panel::top("chrome-bar").frame(frame).show(ui, |ui| {
        ui.native_set_style(Role::Toolbar, RoleVariant::Normal);
        // On macOS outside `cfg(test)` the menus are the system menu bar's (Task 35).
        #[cfg(not(all(target_os = "macos", not(test))))]
        menu_bar(app, ui);
        toolbar(app, ui);
    });
    app.registry.record(
        &out.response,
        demo::info(
            "chrome bar",
            vec![
                Seam::Surface(Surface::Panel(PanelSide::Top)),
                Seam::Role(Role::Toolbar, RoleVariant::Normal),
            ],
        ),
        true,
    );
}

/// The in-window menu bar (Linux, Windows, and macOS under `cfg(test)`; Task 35
/// gives macOS the system menu bar). `Role::Menu` through `role_modifier` to
/// both `MenuBar::style` and `MenuConfig::style` (§4.2); each open menu's own
/// `Ui` is recorded as `Role::Menu` too.
#[cfg(not(all(target_os = "macos", not(test))))]
fn menu_bar(app: &mut App, ui: &mut egui::Ui) {
    let App {
        registry,
        atlas,
        pending,
        ..
    } = app;
    let theme = ui.ctx().theme();
    let modifier = atlas.role_modifier(theme, Role::Menu, RoleVariant::Normal);
    let bar = egui::MenuBar::new()
        .style(modifier.clone())
        .config(egui::containers::menu::MenuConfig::new().style(modifier));
    let out = bar.ui(ui, |ui| {
        for (menu, items) in Action::MENUS {
            let response = ui.menu_button(*menu, |ui| {
                registry.record(
                    &ui.response(),
                    demo::info("menu", vec![Seam::Role(Role::Menu, RoleVariant::Normal)]),
                    true,
                );
                for item in *items {
                    match item {
                        None => {
                            ui.separator();
                        }
                        Some(action) => {
                            let mut button = egui::Button::new(action.label());
                            if let Some(shortcut) = action.shortcut() {
                                button = button.shortcut_text(ui.ctx().format_shortcut(&shortcut));
                            }
                            let r = ui.add(button);
                            registry.record(
                                &r,
                                demo::info(
                                    "menu item",
                                    vec![Seam::Role(Role::Menu, RoleVariant::Normal)],
                                ),
                                false,
                            );
                            if r.clicked() {
                                pending.push(*action);
                                ui.close();
                            }
                        }
                    }
                }
            });
            registry.record(
                &response.response,
                demo::info(
                    "menu button",
                    vec![Seam::Role(Role::Menu, RoleVariant::Normal)],
                ),
                false,
            );
        }
    });
    registry.record(
        &out.response,
        demo::info(
            "menu bar",
            vec![Seam::Role(Role::Menu, RoleVariant::Normal)],
        ),
        true,
    );
}

/// Three icon buttons with tooltips, at `toolbar.icon_size` (a required size, `f32` on the
/// resolved theme), the row at least `toolbar.bar_height` tall where the theme states a usable
/// one; icons of the chosen set and theme through `demo::role_image`, the Icons page's loader —
/// a freedesktop icon from the chosen theme in the text colour, a bundled key tinted it (§9.2,
/// §10.4).
fn toolbar(app: &mut App, ui: &mut egui::Ui) {
    let theme = ui.ctx().theme();
    let (set, icon_theme) = app.chosen_icons();
    let App {
        registry,
        atlas,
        pending,
        ..
    } = app;
    let t = atlas.resolved_for(theme);
    let bar_height = t.toolbar.bar_height.filter(|h| h.is_finite() && *h >= 0.0);
    let icon_size = t.toolbar.icon_size;
    let out = ui.horizontal(|ui| {
        if let Some(h) = bar_height {
            ui.set_min_height(h);
        }
        for (role, label, action) in [
            (
                IconRole::ActionSearch,
                "Command palette",
                Action::OpenCommandPalette,
            ),
            (IconRole::ActionRefresh, "Reload theme", Action::ReloadTheme),
            (
                IconRole::ActionSettings,
                "Preferences",
                Action::OpenPreferences,
            ),
        ] {
            let image = demo::role_image(ui, role, set, icon_theme.as_deref(), icon_size);
            let response = demo::scoped(
                registry,
                ui,
                Role::Button,
                RoleVariant::Normal,
                "toolbar button",
                |ui| {
                    let r = match image {
                        Some(image) => ui.add(egui::Button::image(image)),
                        None => ui.button(label),
                    };
                    r.widget_info(|| {
                        egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label)
                    });
                    r
                },
            );
            registry.amend_last(|i| i.read.push(("toolbar.icon_size", format!("{icon_size}"))));
            tooltip(registry, ui, &response, label);
            if response.clicked() {
                pending.push(action);
            }
        }
        // The bar spans the window, as a toolbar does: the rest of the row is its own surface.
        ui.allocate_space(egui::Vec2::X * ui.available_width());
    });
    registry.record(
        &out.response,
        demo::info(
            "toolbar",
            vec![Seam::Role(Role::Toolbar, RoleVariant::Normal)],
        ),
        true,
    );
}

/// `Tooltip::for_enabled`, its `popup` given the tooltip surface's frame and the `Role::Tooltip`
/// modifier through `demo::surfaced`, the Overlays page's spelling (§10.4;
/// `Response::on_hover_text` takes neither).
fn tooltip(reg: &mut Registry, ui: &mut egui::Ui, response: &egui::Response, text: &str) {
    let tooltip_role = Some((Role::Tooltip, RoleVariant::Normal));
    demo::surfaced(
        reg,
        ui,
        Surface::Tooltip,
        false,
        tooltip_role,
        "tooltip",
        |_, chrome, _| {
            let mut tip = egui::Tooltip::for_enabled(response);
            tip.popup = tip.popup.frame(chrome.frame);
            if let Some(modifier) = chrome.modifier {
                tip.popup = tip.popup.style(modifier);
            }
            tip.show(|ui| {
                ui.label(text);
            })
            .map(|out| out.response)
        },
    );
}

/// The status bar: the side-panel toggle, the environment, the shown info's
/// title (Task 36 fills the title; here it is empty).
pub(crate) fn status_bar(app: &mut App, ui: &mut egui::Ui) {
    ui.native_set_style(Role::StatusBar, RoleVariant::Normal);
    let frame = ui.native_frame(Surface::Panel(PanelSide::Bottom));
    let out = egui::Panel::bottom("status-bar")
        .frame(frame)
        .show(ui, |ui| {
            ui.native_set_style(Role::StatusBar, RoleVariant::Normal);
            let environment = environment(app, ui.ctx());
            let title = app.status_title();
            ui.horizontal(|ui| {
                let App {
                    registry,
                    side_panel_visible,
                    ..
                } = app;
                demo::scoped(
                    registry,
                    ui,
                    Role::Button,
                    RoleVariant::Normal,
                    "side panel toggle",
                    |ui| ui.toggle_value(side_panel_visible, "Side panel"),
                );
                // The bar's own role, not the base style: its separator line and its text are
                // `status_bar`'s (§10.4).
                let bar = (Role::StatusBar, RoleVariant::Normal);
                let mut item = |ui: &mut egui::Ui, kind: &'static str, text: String| {
                    demo::scoped(registry, ui, bar.0, bar.1, "status separator", |ui| {
                        ui.separator()
                    });
                    demo::scoped(registry, ui, bar.0, bar.1, kind, |ui| ui.label(text));
                };
                for text in environment {
                    item(ui, "status item", text);
                }
                // The shown Widget Info's title, last (§10.4).
                if !title.is_empty() {
                    item(ui, "status title", title);
                }
            });
        });
    app.registry.record(
        &out.response,
        demo::info(
            "status bar",
            vec![
                Seam::Surface(Surface::Panel(PanelSide::Bottom)),
                Seam::Role(Role::StatusBar, RoleVariant::Normal),
            ],
        ),
        true,
    );
}

/// Desktop, preset and mode, the font in its defined unit (§8.7), the
/// text-scaling factor, the flags that are set (the gpui status bar,
/// `connectors/native-theme-gpui/examples/showcase-gpui/chrome.rs:279-302`).
fn environment(app: &App, ctx: &egui::Context) -> Vec<String> {
    let theme = ctx.theme();
    let t = app.atlas.resolved_for(theme);
    let prefs = app.atlas.accessibility();
    let mode = if theme == egui::Theme::Dark {
        "dark"
    } else {
        "light"
    };
    let font = &t.defaults.font;
    let size = match font.defined_size {
        Some(native_theme::theme::FontSize::Pt(v)) => format!("{v}pt"),
        Some(native_theme::theme::FontSize::Px(v)) => format!("{v}px"),
        // Validation records a missing size rather than inventing one (§8.7).
        None => "(size not stated)".to_string(),
    };
    let mut items = vec![
        desktop(),
        format!("{} {mode}", app.atlas.name()),
        format!("{} {size}", font.family),
        format!("text ×{}", prefs.text_scaling_factor),
    ];
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
    items
}

/// The desktop `native_theme::detect` recognises in `XDG_CURRENT_DESKTOP`, as the gpui
/// showcase names it (`connectors/native-theme-gpui/examples/showcase-gpui/chrome.rs:309-318`).
#[cfg(target_os = "linux")]
fn desktop() -> String {
    format!("{:?}", native_theme::detect::detect_linux_desktop())
}

/// The operating system: `native_theme::detect` names a desktop on Linux only.
#[cfg(not(target_os = "linux"))]
fn desktop() -> String {
    std::env::consts::OS.to_string()
}

/// The side panel: `Role::Splitter` live on the root while it is shown, its body
/// in `Role::Sidebar`; not inside a scope (§10.4, the panel-in-scope note).
pub(crate) fn side_panel(app: &mut App, ui: &mut egui::Ui) {
    ui.native_set_style(Role::Splitter, RoleVariant::Normal);
    let frame = ui.native_frame(Surface::Panel(PanelSide::Left));
    let mut visible = app.side_panel_visible;
    app.hold_zone = None; // set again while the inspector is drawn
    let out = egui::Panel::left("side-panel")
        .resizable(true)
        .default_size(LEFT_PANEL_WIDTH)
        .frame(frame)
        .show_collapsible(ui, &mut visible, |ui| {
            ui.native_set_style(Role::Sidebar, RoleVariant::Normal);
            settings_rows(app, ui);
            demo::scoped(
                &mut app.registry,
                ui,
                Role::Separator,
                RoleVariant::Normal,
                "side panel separator",
                |ui| ui.separator(),
            );
            inspector_tabs(app, ui);
            let area = egui::ScrollArea::vertical().show(ui, |ui| inspector_content(app, ui));
            // The content records nothing and is Widget Info's hold zone (§10.4).
            app.hold_zone = Some(area.inner_rect);
        });
    app.side_panel_visible = visible;
    if let Some(out) = out {
        app.registry.record(
            &out.response,
            demo::info(
                "side panel",
                vec![
                    Seam::Surface(Surface::Panel(PanelSide::Left)),
                    Seam::Role(Role::Splitter, RoleVariant::Normal),
                    Seam::Role(Role::Sidebar, RoleVariant::Normal),
                ],
            ),
            true,
        );
    }
}

/// Theme (`ComboBox` of `default` and the platform's presets), Mode (a
/// segmented control of `Button::new(..).selected(..)`), Icon theme (`ComboBox`).
fn settings_rows(app: &mut App, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let theme = ctx.theme();
    let current_theme = match &app.settings.theme {
        ThemeChoice::Default => "default".to_string(),
        ThemeChoice::Preset(name) => name.clone(),
    };
    let presets = native_theme::theme::Theme::list_presets_for_platform();
    let current_mode = app.settings.mode;
    // The iced showcase's icon-theme list (`connectors/native-theme-iced/examples/showcase-iced.rs:581-596`).
    let mut icon_rows = Vec::new();
    if let choice @ IconSetChoice::Default(_) =
        default_icon_choice(app.atlas.icon_set(), app.atlas.icon_theme(theme))
    {
        icon_rows.push(choice);
    }
    icon_rows.push(IconSetChoice::System);
    icon_rows.extend(
        app.settings
            .installed_themes
            .iter()
            .map(|name| IconSetChoice::Freedesktop(name.clone())),
    );
    icon_rows.push(IconSetChoice::Material);
    icon_rows.push(IconSetChoice::Lucide);
    let current_icon = app.settings.icon.clone();

    let mut picked_theme: Option<ThemeChoice> = None;
    let mut picked_mode: Option<ModeChoice> = None;
    let mut picked_icon: Option<IconSetChoice> = None;
    let registry = &mut app.registry;
    demo::scoped_popup(
        registry,
        ui,
        Role::ComboBox,
        RoleVariant::Normal,
        "theme picker",
        |ui, modifier| {
            let mut combo =
                egui::ComboBox::from_label("Theme").selected_text(current_theme.as_str());
            if let Some(modifier) = modifier {
                combo = combo.popup_style(modifier);
            }
            combo
                .show_ui(ui, |ui| {
                    for key in std::iter::once("default").chain(presets.iter().map(|info| info.key))
                    {
                        if ui.selectable_label(current_theme == key, key).clicked() {
                            picked_theme = Some(if key == "default" {
                                ThemeChoice::Default
                            } else {
                                ThemeChoice::Preset(key.to_string())
                            });
                        }
                    }
                })
                .response
        },
    );
    demo::scoped(
        registry,
        ui,
        Role::SegmentedControl,
        RoleVariant::Normal,
        "mode",
        |ui| {
            ui.horizontal(|ui| {
                for mode in [ModeChoice::System, ModeChoice::Light, ModeChoice::Dark] {
                    if ui
                        .add(
                            egui::Button::new(Action::SetMode(mode).label())
                                .selected(current_mode == mode),
                        )
                        .clicked()
                    {
                        picked_mode = Some(mode);
                    }
                }
            })
            .response
        },
    );
    demo::scoped_popup(
        registry,
        ui,
        Role::ComboBox,
        RoleVariant::Normal,
        "icon theme picker",
        |ui, modifier| {
            let mut combo =
                egui::ComboBox::from_label("Icon theme").selected_text(current_icon.to_string());
            if let Some(modifier) = modifier {
                combo = combo.popup_style(modifier);
            }
            combo
                .show_ui(ui, |ui| {
                    for choice in icon_rows {
                        if ui
                            .selectable_label(current_icon == choice, choice.to_string())
                            .clicked()
                        {
                            picked_icon = Some(choice);
                        }
                    }
                })
                .response
        },
    );

    if let Some(choice) = picked_theme {
        app.settings.theme = choice;
        app.install(&ctx);
    }
    if let Some(mode) = picked_mode {
        app.settings.mode = mode;
        app.install(&ctx);
    }
    if let Some(choice) = picked_icon {
        app.settings.pick_icon(Some(choice));
        app.settings.theme_installed(&app.atlas, theme);
    }
}

/// The inspector's two tabs, drawn as the page tabs are: `Button::new(..).selected(..)` in one
/// `Role::Tab` scope; "Theme tab" so the label differs from the Theme menu and combo box.
fn inspector_tabs(app: &mut App, ui: &mut egui::Ui) {
    let current = app.inspector_tab;
    let mut picked: Option<InspectorTab> = None;
    let registry = &mut app.registry;
    let out = ui.native_scope(Role::Tab, RoleVariant::Normal, |ui| {
        ui.horizontal(|ui| {
            for (tab, label) in [
                (InspectorTab::Widget, "Widget"),
                (InspectorTab::Theme, "Theme tab"),
            ] {
                let r = ui.add(egui::Button::new(label).selected(current == tab));
                registry.record(
                    &r,
                    demo::info(
                        "inspector tab",
                        vec![Seam::Role(Role::Tab, RoleVariant::Normal)],
                    ),
                    false,
                );
                if r.clicked() {
                    picked = Some(tab);
                }
            }
            // The tab bar spans the panel: the rest of the row is its own surface.
            ui.allocate_space(egui::Vec2::X * ui.available_width());
        })
    });
    app.registry.record(
        &out.response,
        demo::info(
            "inspector tabs",
            vec![Seam::Role(Role::Tab, RoleVariant::Normal)],
        ),
        true,
    );
    if let Some(tab) = picked {
        app.inspector_tab = tab;
    }
}

/// The inspector's content below the tabs: the Widget tab or the Theme tab (§10.4).
fn inspector_content(app: &mut App, ui: &mut egui::Ui) {
    let theme = ui.ctx().theme();
    match app.inspector_tab {
        InspectorTab::Widget => {
            let json = crate::info::theme_json(&app.atlas, theme);
            crate::info::widget_tab(
                ui,
                app.registry.shown(),
                &app.manifest,
                &json,
                app.atlas.name(),
            );
        }
        InspectorTab::Theme => crate::info::theme_tab(ui, &app.atlas, &app.manifest),
    }
}

/// The page tabs: one `native_scope(Role::Tab, ..)`, each tab a `Button::new(label).selected(..)` (§10.4).
pub(crate) fn page_tabs(app: &mut App, ui: &mut egui::Ui) {
    let App {
        registry,
        settings,
        pending,
        ..
    } = app;
    let out = ui.native_scope(Role::Tab, RoleVariant::Normal, |ui| {
        ui.horizontal(|ui| {
            for page in Page::ALL {
                let r = ui.add(egui::Button::new(page.label()).selected(settings.page == page));
                registry.record(
                    &r,
                    demo::info("page tab", vec![Seam::Role(Role::Tab, RoleVariant::Normal)]),
                    false,
                );
                if r.clicked() {
                    pending.push(Action::ShowPage(page));
                }
            }
            // The tab bar spans the page: the rest of the row is its own surface.
            ui.allocate_space(egui::Vec2::X * ui.available_width());
        });
    });
    registry.record(
        &out.response,
        demo::info(
            "page tabs",
            vec![Seam::Role(Role::Tab, RoleVariant::Normal)],
        ),
        true,
    );
}

/// The content: the central panel surface fed from `theme.defaults`; the tabs,
/// a theme error in a card, then the page.
pub(crate) fn central_panel(
    app: &mut App,
    ui: &mut egui::Ui,
    add: impl FnOnce(&mut App, &mut egui::Ui),
) {
    let frame = ui.native_frame(Surface::CentralPanel);
    let out = egui::CentralPanel::default().frame(frame).show(ui, |ui| {
        page_tabs(app, ui);
        if let Some(error) = app.theme_error.clone() {
            demo::framed(
                &mut app.registry,
                ui,
                Surface::Card,
                None,
                "theme error",
                |ui, _| {
                    ui.colored_label(ui.visuals().error_fg_color, error);
                },
            );
        }
        add(app, ui);
    });
    app.registry.record(
        &out.response,
        demo::info("central panel", vec![Seam::Surface(Surface::CentralPanel)]),
        true,
    );
}

/// A dialog extent the theme states, less the frame's own margins and stroke,
/// floored at `0.0` and passed only when finite: `Ui::set_min_*` debug-assert a
/// size of at least `0.0` (`egui/src/ui.rs:739-743`, `:749-753`), and
/// `set_max_*` reach the same assert through `Placer` and `Layout`
/// (`egui/src/placer.rs:233-234`, `egui/src/layout.rs:584-587`).
pub(crate) fn inner_extent(stated: f32, frame: &egui::Frame, horizontal: bool) -> Option<f32> {
    let (a, b, c, d) = if horizontal {
        (
            frame.inner_margin.left,
            frame.inner_margin.right,
            frame.outer_margin.left,
            frame.outer_margin.right,
        )
    } else {
        (
            frame.inner_margin.top,
            frame.inner_margin.bottom,
            frame.outer_margin.top,
            frame.outer_margin.bottom,
        )
    };
    let margins = f32::from(a) + f32::from(b) + f32::from(c) + f32::from(d);
    let inner = stated - margins - 2.0 * frame.stroke.width;
    if inner.is_finite() {
        Some(inner.max(0.0))
    } else {
        None
    }
}

/// The command palette (§10.4): a `Modal` in the dialog surface, its body in
/// `Role::Dialog`, a focused field, then rows in `Role::List` for every page,
/// preset and mode. Escape clears, then closes.
pub(crate) fn command_palette(app: &mut App, ui: &mut egui::Ui) {
    let Some(palette) = app.palette.clone() else {
        return;
    };
    let theme = ui.ctx().theme();
    let frame = ui.native_frame(Surface::Dialog);
    let t = app.atlas.resolved_for(theme);
    // The four dialog extents are required sizes (`f32` on the resolved theme).
    let min_w = inner_extent(t.dialog.min_width, &frame, true);
    let max_w = inner_extent(t.dialog.max_width, &frame, true);
    let min_h = inner_extent(t.dialog.min_height, &frame, false);
    let max_h = inner_extent(t.dialog.max_height, &frame, false);
    let mut query = palette.query.clone();
    let mut chosen: Option<Action> = None;
    let mut chosen_theme: Option<ThemeChoice> = None;
    let App {
        registry, settings, ..
    } = app;
    let response = egui::Modal::new(egui::Id::new("command-palette"))
        .frame(frame)
        .show(ui.ctx(), |ui| {
            demo::styled(
                registry,
                ui,
                Role::Dialog,
                RoleVariant::Normal,
                "command palette",
            );
            if let Some(w) = min_w {
                ui.set_min_width(w);
            }
            if let Some(w) = max_w {
                ui.set_max_width(w);
            }
            if let Some(h) = min_h {
                ui.set_min_height(h);
            }
            if let Some(h) = max_h {
                ui.set_max_height(h);
            }
            let field = ui.add(egui::TextEdit::singleline(&mut query).hint_text("Type to filter"));
            if !field.has_focus() && palette.query.is_empty() {
                field.request_focus();
            }
            registry.record(
                &field,
                demo::info(
                    "palette query",
                    vec![Seam::Role(Role::Dialog, RoleVariant::Normal)],
                ),
                false,
            );
            let needle = query.to_lowercase();
            ui.native_scope(Role::List, RoleVariant::Normal, |ui| {
                for page in Page::ALL {
                    let label = format!("Page: {}", page.label());
                    if label.to_lowercase().contains(&needle) {
                        let r = ui.selectable_label(false, &label);
                        registry.record(
                            &r,
                            demo::info(
                                "palette row",
                                vec![Seam::Role(Role::List, RoleVariant::Normal)],
                            ),
                            false,
                        );
                        if r.clicked() {
                            chosen = Some(Action::ShowPage(page));
                        }
                    }
                }
                let presets = std::iter::once("default".to_string()).chain(
                    native_theme::theme::Theme::list_presets_for_platform()
                        .into_iter()
                        .map(|i| i.key.to_string()),
                );
                for key in presets {
                    let label = format!("Preset: {key}");
                    if label.to_lowercase().contains(&needle) {
                        let r = ui.selectable_label(false, &label);
                        registry.record(
                            &r,
                            demo::info(
                                "palette row",
                                vec![Seam::Role(Role::List, RoleVariant::Normal)],
                            ),
                            false,
                        );
                        if r.clicked() {
                            chosen_theme = Some(if key == "default" {
                                ThemeChoice::Default
                            } else {
                                ThemeChoice::Preset(key)
                            });
                        }
                    }
                }
                for mode in [ModeChoice::System, ModeChoice::Light, ModeChoice::Dark] {
                    let label = format!("Mode: {}", Action::SetMode(mode).label());
                    if label.to_lowercase().contains(&needle) {
                        let r = ui.selectable_label(false, &label);
                        registry.record(
                            &r,
                            demo::info(
                                "palette row",
                                vec![Seam::Role(Role::List, RoleVariant::Normal)],
                            ),
                            false,
                        );
                        if r.clicked() {
                            chosen = Some(Action::SetMode(mode));
                        }
                    }
                }
            });
        });
    registry.record(
        &response.backdrop_response,
        demo::info(
            "command palette backdrop",
            vec![Seam::Surface(Surface::Dialog)],
        ),
        true,
    );
    // Escape: clear the query first; only an empty query lets the modal close (§10.4).
    let escape = ui.ctx().input_mut(|i| {
        !query.is_empty() && i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)
    });
    if escape {
        query.clear();
    }
    let close =
        chosen.is_some() || chosen_theme.is_some() || (query.is_empty() && response.should_close());
    app.palette = if close {
        None
    } else {
        Some(PaletteState { query })
    };
    if let Some(choice) = chosen_theme {
        settings.theme = choice;
        let ctx = ui.ctx().clone();
        app.install(&ctx);
    }
    if let Some(action) = chosen {
        app.pending.push(action);
    }
}

/// Preferences: a `Window` in the window surfaces, its body `Role::Window`, its
/// title in the title-bar font and colour (§4.7, §14 item 2b); a change
/// rebuilds with `Builder::accessibility` through `App::install` (§4.3).
pub(crate) fn preferences(app: &mut App, ui: &mut egui::Ui) {
    if !app.preferences_open {
        return;
    }
    let ctx = ui.ctx().clone();
    let t = app.atlas.resolved_for(ctx.theme());
    let title = egui::RichText::new("Preferences")
        .font(window_title_bar_font(t, app.atlas.accessibility()))
        .color(window_title_bar_text_color(t, true));
    let mut prefs = app
        .settings
        .prefs
        .clone()
        .unwrap_or_else(|| app.atlas.accessibility().clone());
    let before = prefs.clone();
    let mut open = app.preferences_open;
    let window = egui::Window::new(title)
        .id(egui::Id::new("preferences"))
        .open(&mut open)
        .frame(ui.native_frame(Surface::Window))
        .title_frame(ui.native_frame(Surface::WindowTitleBar));
    let registry = &mut app.registry;
    let out = window.show(&ctx, |ui| {
        demo::styled(
            registry,
            ui,
            Role::Window,
            RoleVariant::Normal,
            "preferences",
        );
        ui.horizontal(|ui| {
            ui.label("Text scaling");
            demo::scoped(
                registry,
                ui,
                Role::Input,
                RoleVariant::Normal,
                "text scaling factor",
                |ui| ui.add(egui::DragValue::new(&mut prefs.text_scaling_factor)),
            );
        });
        demo::scoped(
            registry,
            ui,
            Role::Checkbox,
            RoleVariant::Normal,
            "reduce motion",
            |ui| ui.checkbox(&mut prefs.reduce_motion, "Reduce motion"),
        );
        demo::scoped(
            registry,
            ui,
            Role::Checkbox,
            RoleVariant::Normal,
            "high contrast",
            |ui| ui.checkbox(&mut prefs.high_contrast, "High contrast"),
        );
        demo::scoped(
            registry,
            ui,
            Role::Checkbox,
            RoleVariant::Normal,
            "reduce transparency",
            |ui| ui.checkbox(&mut prefs.reduce_transparency, "Reduce transparency"),
        );
    });
    if let Some(out) = out {
        app.registry.record(
            &out.response,
            demo::info(
                "preferences window",
                vec![
                    Seam::Surface(Surface::Window),
                    Seam::Surface(Surface::WindowTitleBar),
                ],
            ),
            true,
        );
    }
    app.preferences_open = open;
    if prefs != before {
        app.settings.prefs = Some(prefs);
        app.install(&ctx);
    }
}

/// About: a `Modal` in the dialog surface with the crate's name and version, a
/// `Hyperlink` to the README's Compatibility section, and its buttons in
/// `dialog_button_order` (§4.7).
pub(crate) fn about(app: &mut App, ui: &mut egui::Ui) {
    if !app.about_open {
        return;
    }
    let ctx = ui.ctx().clone();
    let t = app.atlas.resolved_for(ctx.theme());
    // The primary button is Close: at the trailing end on PrimaryRight, the leading on PrimaryLeft.
    let labels = match dialog_button_order(t) {
        DialogButtonOrder::PrimaryRight => ["Copy version", "Close"],
        DialogButtonOrder::PrimaryLeft => ["Close", "Copy version"],
    };
    let version = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));
    let frame = ui.native_frame(Surface::Dialog);
    let registry = &mut app.registry;
    let mut close = false;
    let response = egui::Modal::new(egui::Id::new("about")).frame(frame).show(&ctx, |ui| {
        demo::styled(registry, ui, Role::Dialog, RoleVariant::Normal, "about");
        demo::scoped(registry, ui, Role::Dialog, RoleVariant::Normal, "about version", |ui| ui.label(version));
        demo::scoped(registry, ui, Role::Link, RoleVariant::Normal, "compatibility link", |ui| {
            ui.hyperlink_to(
                "Compatibility",
                "https://github.com/tiborgats/native-theme/blob/main/connectors/native-theme-egui/README.md#compatibility",
            )
        });
        ui.horizontal(|ui| {
            for label in labels {
                if demo::scoped(registry, ui, Role::Button, RoleVariant::Normal, "about button", |ui| ui.button(label)).clicked() {
                    if label == "Close" {
                        close = true;
                    } else {
                        ui.ctx().copy_text(version.to_string());
                    }
                }
            }
        });
    });
    app.registry.record(
        &response.response,
        demo::info("about", vec![Seam::Surface(Surface::Dialog)]),
        true,
    );
    if close || response.should_close() {
        app.about_open = false;
    }
}

#[cfg(all(target_os = "macos", not(test)))]
pub(crate) mod system_menu {
    //! The showcase's menus in the macOS system menu bar (spec §10.4).

    use muda::{
        IsMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem, Submenu,
        accelerator::{Accelerator, Code, Modifiers},
    };
    use objc2::MainThreadMarker;
    use objc2_app_kit::NSApplication;

    use super::Action;

    pub(crate) struct SystemMenu {
        menu: Menu,
    }

    impl SystemMenu {
        /// The number of top-level menus, which the readback compares with `mainMenu`'s count.
        pub(crate) fn top_level(&self) -> usize {
            self.menu.items().len()
        }
    }

    /// `action`'s shortcut as muda's accelerator: egui's `Modifiers::COMMAND` is Cmd on macOS,
    /// muda's `Modifiers::META`; a key this match does not map gets no accelerator.
    fn accelerator(action: Action) -> Option<Accelerator> {
        let code = match action.shortcut()?.logical_key {
            egui::Key::Q => Code::KeyQ,
            egui::Key::B => Code::KeyB,
            egui::Key::K => Code::KeyK,
            egui::Key::Comma => Code::Comma,
            _ => return None,
        };
        Some(Accelerator::new(Modifiers::META, code))
    }

    /// Build File, View, Theme and Help from `Action::MENUS`, install them as the
    /// application's main menu, and route every click to `tx` with a repaint,
    /// so `logic` runs the action on the next pass.
    pub(crate) fn build(
        ctx: &egui::Context,
        tx: std::sync::mpsc::Sender<Action>,
    ) -> muda::Result<SystemMenu> {
        let menu = Menu::new();
        let mut actions = Vec::new();
        for (title, items) in Action::MENUS {
            let mut owned: Vec<Box<dyn IsMenuItem>> = Vec::new();
            for item in *items {
                match item {
                    None => owned.push(Box::new(PredefinedMenuItem::separator())),
                    Some(action) => {
                        let id = MenuId::new(actions.len().to_string());
                        actions.push(*action);
                        owned.push(Box::new(MenuItem::with_id(
                            id,
                            action.label(),
                            true,
                            accelerator(*action),
                        )));
                    }
                }
            }
            let refs: Vec<&dyn IsMenuItem> = owned.iter().map(|b| b.as_ref()).collect();
            let submenu = Submenu::with_items(*title, true, &refs)?;
            menu.append(&submenu)?;
        }
        menu.init_for_nsapp();
        let ctx = ctx.clone();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            if let Some(action) = event
                .id()
                .0
                .parse::<usize>()
                .ok()
                .and_then(|i| actions.get(i).copied())
            {
                let _ = tx.send(action);
                ctx.request_repaint();
            }
        }));
        Ok(SystemMenu { menu })
    }

    /// `NSApplication.mainMenu.numberOfItems`, on the main thread; `None` off it
    /// or when no main menu is installed (§13's runner check).
    pub(crate) fn main_menu_item_count() -> Option<isize> {
        let mtm = MainThreadMarker::new()?;
        let app = NSApplication::sharedApplication(mtm);
        app.mainMenu().map(|menu| menu.numberOfItems())
    }
}
