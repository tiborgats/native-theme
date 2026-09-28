//! The chrome: menu bar, toolbar, status bar, side panel, page tabs, overlays (spec §10.4).

use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme::theme::{IconRole, IconSet};
use native_theme_egui::{
    DialogButtonOrder, PanelSide, Role, RoleVariant, Surface, dialog_button_order,
    window_title_bar_font, window_title_bar_text_color,
};

use crate::{
    LEFT_PANEL_WIDTH,
    app::{App, ModeChoice, Page, ThemeChoice},
    demo::{self, PanelSeams, Registry},
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
                Some(Action::ShowPage(Page::Basic)),
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
    let toolbar_role = Some((Role::Toolbar, RoleVariant::Normal));
    let seams = PanelSeams::apply(
        ui,
        Surface::Panel(PanelSide::Top),
        toolbar_role,
        toolbar_role,
    );
    // No line under the bar: the gpui showcase draws none under its toolbar
    // (`showcase-gpui/app.rs:1921-1922`).
    let out = egui::Panel::top("chrome-bar")
        .frame(seams.frame)
        .show_separator_line(false)
        .show(ui, |ui| {
            seams.enter(ui);
            // On macOS outside `cfg(test)` the menus are the system menu bar's.
            #[cfg(not(all(target_os = "macos", not(test))))]
            menu_bar(app, ui);
            toolbar(app, ui);
        });
    seams.record(&mut app.registry, &out.response, "Chrome bar");
}

/// The in-window menu bar (Linux, Windows, and macOS under `cfg(test)`; macOS has the system
/// menu bar otherwise). `Role::Menu` through `role_modifier` to both `MenuBar::style` and
/// `MenuConfig::style` (§4.2); each open menu's own `Ui` is styled and recorded as `Role::Menu`
/// too.
#[cfg(not(all(target_os = "macos", not(test))))]
fn menu_bar(app: &mut App, ui: &mut egui::Ui) {
    let App {
        registry, pending, ..
    } = app;
    let normal = RoleVariant::Normal;
    demo::menu_bar(
        registry,
        ui,
        Role::Menu,
        normal,
        "Menu bar",
        |ui, bar, menu_seam, registry| {
            bar.ui(ui, |ui| {
                for (menu, items) in Action::MENUS {
                    let response = ui.menu_button(*menu, |ui| {
                        let open = demo::styled(registry, ui, Role::Menu, normal, "Menu");
                        for item in *items {
                            match item {
                                None => {
                                    open.add(registry, ui, "Separator · menu", |ui| {
                                        ui.separator()
                                    });
                                }
                                Some(action) => {
                                    let mut button = egui::Button::new(action.label());
                                    if let Some(shortcut) = action.shortcut() {
                                        button = button
                                            .shortcut_text(ui.ctx().format_shortcut(&shortcut));
                                    }
                                    let r =
                                        open.add(registry, ui, "Menu item", |ui| ui.add(button));
                                    if r.clicked() {
                                        pending.push(*action);
                                        ui.close();
                                    }
                                }
                            }
                        }
                    });
                    menu_seam.record(registry, &response.response, "Menu button");
                }
            })
            .response
        },
    );
}

/// What a chrome button shows: gpui-component's `IconName` by its name in the chosen set, or
/// the one an `IconRole` stands for.
#[derive(Clone, Copy)]
enum ChromeButtonIcon {
    Named(demo::ChromeIcon),
    Role(IconRole),
}

/// A chrome button's icon of the chosen set and theme at `size`, or `None` where they lack it.
fn chrome_button_image(
    ui: &egui::Ui,
    icon: ChromeButtonIcon,
    (set, icon_theme): &(IconSet, Option<String>),
    size: f32,
) -> Option<egui::Image<'static>> {
    match icon {
        ChromeButtonIcon::Named(icon) => {
            demo::named_image(ui, icon, *set, icon_theme.as_deref(), size)
        }
        ChromeButtonIcon::Role(role) => {
            demo::role_image(ui, role, *set, icon_theme.as_deref(), size)
        }
    }
}

/// A Ghost button, as the gpui showcase's toolbar and status-bar buttons are (parity rule R4),
/// for a `Ui` made Ghost with `demo::ghost`: transparent at rest and filled on hover; while
/// `selected`, filled with the pressed fill of that `Ui` — in `Role::Button`,
/// `button.active_background` (§6.4) — as native-theme-gpui's `ghost_button` fills with its
/// `active` colour (`connectors/native-theme-gpui/src/variants.rs:51-57`), not with the
/// selected button's accent. Its icon where the set has one, else its tooltip's text as its
/// label, as gpui's (`showcase-gpui/demo.rs:480-483`).
fn ghost_button(
    ui: &egui::Ui,
    image: Option<egui::Image<'static>>,
    label: &'static str,
    selected: bool,
) -> egui::Button<'static> {
    let button = match image {
        Some(image) => egui::Button::image(image),
        None => egui::Button::new(label),
    };
    if selected {
        button.fill(ui.visuals().widgets.active.weak_bg_fill)
    } else {
        button
    }
}

/// The kind a Ghost button records, as the gpui showcase names its toolbar and status-bar
/// buttons (`showcase-gpui/info/chrome.rs:414-419`, `:871-875`).
fn ghost_kind(drawn: bool, selected: bool) -> &'static str {
    match (drawn, selected) {
        (true, true) => "Button · Ghost, icon, selected",
        (true, false) => "Button · Ghost, icon",
        (false, true) => "Button · Ghost, labelled, selected",
        (false, false) => "Button · Ghost, labelled",
    }
}

/// The gpui showcase's toolbar (`showcase-gpui/chrome.rs:103-160`): three Ghost buttons in its
/// order — the command palette, a theme reload, Preferences — each with a tooltip naming it
/// and its shortcut, at `toolbar.icon_size` (a required size, `f32` on the resolved theme), the
/// row at least `toolbar.bar_height` tall where the theme states a usable one. The icons are
/// gpui's: `SquareTerminal` and `RotateCw` by their names in the chosen set, Settings by its
/// role, loaded as the Icons page loads them — a freedesktop icon from the chosen theme in the
/// text colour, a bundled key tinted it (§9.2, §10.4).
fn toolbar(app: &mut App, ui: &mut egui::Ui) {
    let theme = ui.ctx().theme();
    let chosen = app.chosen_icons();
    let App {
        registry,
        atlas,
        pending,
        ..
    } = app;
    let t = atlas.resolved_for(theme);
    let bar_height = t.toolbar.bar_height.filter(|h| h.is_finite() && *h >= 0.0);
    let icon_size = t.toolbar.icon_size;
    let toolbar_row = |ui: &mut egui::Ui, _: demo::Applied, registry: &mut Registry| {
        ui.horizontal(|ui| {
            if let Some(h) = bar_height {
                ui.set_min_height(h);
            }
            for (icon, label, action) in [
                (
                    ChromeButtonIcon::Named(demo::ChromeIcon::SquareTerminal),
                    "Command Palette",
                    Action::OpenCommandPalette,
                ),
                (
                    ChromeButtonIcon::Named(demo::ChromeIcon::RotateCw),
                    "Reload System Theme",
                    Action::ReloadTheme,
                ),
                (
                    ChromeButtonIcon::Role(IconRole::ActionSettings),
                    "Preferences",
                    Action::OpenPreferences,
                ),
            ] {
                let image = chrome_button_image(ui, icon, &chosen, icon_size);
                let kind = ghost_kind(image.is_some(), false);
                let response = demo::scoped(
                    registry,
                    ui,
                    Role::Button,
                    RoleVariant::Normal,
                    kind,
                    |ui| {
                        demo::ghost(ui);
                        let r = ui.add(ghost_button(ui, image, label, false));
                        r.widget_info(|| {
                            egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label)
                        });
                        r
                    },
                );
                registry.ghost_last();
                registry.amend_last(|i| i.read.push(("toolbar.icon_size", format!("{icon_size}"))));
                tooltip(registry, ui, &response, label, action.shortcut());
                if response.clicked() {
                    pending.push(action);
                }
            }
            // The bar spans the window, as a toolbar does: the rest of the row is its own surface.
            ui.allocate_space(egui::Vec2::X * ui.available_width());
        })
        .response
    };
    demo::scoped_container(
        registry,
        ui,
        Role::Toolbar,
        RoleVariant::Normal,
        "Toolbar",
        toolbar_row,
    );
}

/// `Tooltip::for_enabled`, its `popup` given the tooltip surface's frame and the `Role::Tooltip`
/// modifier through `demo::surfaced`, the Overlays page's spelling (§10.4;
/// `Response::on_hover_text` takes neither): `text`, then the action's shortcut in the
/// platform's spelling (`Context::format_shortcut`), weak, as gpui's `tooltip_with_action`
/// shows the key binding (`showcase-gpui/demo.rs:479`).
fn tooltip(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    response: &egui::Response,
    text: &str,
    shortcut: Option<egui::KeyboardShortcut>,
) {
    let tooltip_role = Some((Role::Tooltip, RoleVariant::Normal));
    let shortcut = shortcut.map(|s| ui.ctx().format_shortcut(&s));
    demo::surfaced(
        reg,
        ui,
        Surface::Tooltip,
        false,
        tooltip_role,
        "Tooltip",
        |_, chrome, reg| {
            let mut tip = egui::Tooltip::for_enabled(response);
            tip.popup = tip.popup.frame(chrome.frame);
            if let Some(modifier) = chrome.modifier {
                tip.popup = tip.popup.style(modifier);
            }
            tip.show(|ui| {
                demo::scoped_container(
                    reg,
                    ui,
                    Role::Tooltip,
                    RoleVariant::Normal,
                    "Tooltip row",
                    |ui, label, reg| {
                        ui.horizontal(|ui| {
                            label.add(reg, ui, "Label · tooltip", |ui| ui.label(text));
                            if let Some(shortcut) = shortcut {
                                label.add(reg, ui, "Label · shortcut", |ui| ui.weak(shortcut));
                            }
                        })
                        .response
                    },
                );
            })
            .map(|out| out.response)
        },
    );
}

/// The status bar, as the gpui showcase's (`showcase-gpui/chrome.rs:242-302`,
/// `showcase-gpui/demo.rs:342-365`, `:522-563`): the side-panel toggle, a small Ghost button
/// with gpui's `PanelLeft` icon at `defaults.icon_sizes.small`, selected while the panel shows;
/// then the environment as one line joined by " · "; the shown Widget Info's title flush right.
pub(crate) fn status_bar(app: &mut App, ui: &mut egui::Ui) {
    let status_role = Some((Role::StatusBar, RoleVariant::Normal));
    let seams = PanelSeams::apply(
        ui,
        Surface::Panel(PanelSide::Bottom),
        status_role,
        status_role,
    );
    let chosen = app.chosen_icons();
    let out = egui::Panel::bottom("status-bar")
        .frame(seams.frame)
        .show(ui, |ui| {
            let bar = seams.enter(ui);
            let environment = environment(app, ui.ctx()).join(" · ");
            let title = app.status_title();
            let icon_size = app
                .atlas
                .resolved_for(ui.ctx().theme())
                .defaults
                .icon_sizes
                .small;
            ui.horizontal(|ui| {
                let App {
                    registry,
                    side_panel_visible,
                    ..
                } = app;
                let label = "Toggle Side Panel";
                let open = *side_panel_visible;
                let image = chrome_button_image(
                    ui,
                    ChromeButtonIcon::Named(demo::ChromeIcon::PanelLeft),
                    &chosen,
                    icon_size,
                );
                let kind = ghost_kind(image.is_some(), open);
                let toggle = demo::scoped(
                    registry,
                    ui,
                    Role::Button,
                    RoleVariant::Normal,
                    kind,
                    |ui| {
                        demo::ghost(ui);
                        let r = ui.add(ghost_button(ui, image, label, open).small());
                        r.widget_info(|| {
                            egui::WidgetInfo::selected(egui::WidgetType::Button, true, open, label)
                        });
                        r
                    },
                );
                registry.ghost_last();
                registry.amend_last(|i| {
                    i.read
                        .push(("defaults.icon_sizes.small", format!("{icon_size}")));
                    i.notes.push((
                        "fill while the panel shows",
                        "the button role's pressed fill, visuals.widgets.active.weak_bg_fill"
                            .to_string(),
                    ));
                });
                tooltip(
                    registry,
                    ui,
                    &toggle,
                    label,
                    Action::ToggleSidePanel.shortcut(),
                );
                if toggle.clicked() {
                    *side_panel_visible = !open;
                }
                // The bar's own role, not the base style: its separator line and its text are
                // `status_bar`'s (§10.4).
                let Some(bar) = bar else { return };
                bar.add(registry, ui, "Label · environment", |ui| {
                    ui.label(environment)
                });
                // The shown Widget Info's title, flush right (§10.4).
                if !title.is_empty() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        bar.add(registry, ui, "Label · shown info", |ui| ui.label(title));
                    });
                }
            });
        });
    seams.record(&mut app.registry, &out.response, "Status bar");
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
    // The preset by its key, `default` by the preset it builds on, as the gpui showcase's
    // (`showcase-gpui/chrome.rs:90-97`).
    let preset = match &app.settings.theme {
        ThemeChoice::Default => format!("default ({})", app.default_preset),
        ThemeChoice::Preset(key) => key.clone(),
    };
    let mut items = vec![
        desktop(),
        format!("{preset} {mode}"),
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
/// in `Role::Sidebar`; not inside a scope (§10.4, the panel-in-scope note). As the gpui
/// showcase's (`showcase-gpui/demo.rs:606-630`): the settings padded by `container_margin`,
/// a separator and the inspector's tabs from edge to edge, the inspector's content padded too.
pub(crate) fn side_panel(app: &mut App, ui: &mut egui::Ui) {
    let margin = app.atlas.layout().container_margin;
    let seams = PanelSeams::apply(
        ui,
        Surface::Panel(PanelSide::Left),
        Some((Role::Splitter, RoleVariant::Normal)),
        Some((Role::Sidebar, RoleVariant::Normal)),
    )
    .unpadded(margin);
    let mut visible = app.side_panel_visible;
    app.hold_zone = None; // set again while the inspector is drawn
    let out = egui::Panel::left("side-panel")
        .resizable(true)
        .default_size(LEFT_PANEL_WIDTH)
        .frame(seams.frame)
        .show_collapsible(ui, &mut visible, |ui| {
            let Some(body) = seams.enter(ui) else {
                return;
            };
            padded(ui, margin, |ui| settings_rows(app, ui, body));
            demo::scoped(
                &mut app.registry,
                ui,
                Role::Separator,
                RoleVariant::Normal,
                "Separator · side panel",
                |ui| ui.separator(),
            );
            inspector_tabs(app, ui, margin);
            let area = egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| padded(ui, margin, |ui| inspector_content(app, ui)));
            // The content records nothing and is Widget Info's hold zone (§10.4).
            app.hold_zone = Some(area.inner_rect);
        });
    app.side_panel_visible = visible;
    if let Some(out) = out {
        seams.record(&mut app.registry, &out.response, "Side panel");
    }
}

/// `add` padded by `margin`, `layout.container_margin`, where the theme states one, and not at
/// all where it states none, as the gpui showcase's `with_padding`.
fn padded<R>(ui: &mut egui::Ui, margin: Option<f32>, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::NONE
        .inner_margin(margin.unwrap_or_default())
        .show(ui, add)
        .inner
}

/// A theme setting's label, above its control and as wide as its text, in `Small` (the gpui
/// showcase's `demo::label`, `Label::text_sm()`, `showcase-gpui/demo.rs:1420-1432`), recorded
/// with the side panel body's role; the control names it as its AccessKit label.
fn setting_label(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    body: demo::Applied,
    text: &'static str,
) -> egui::Id {
    body.add(reg, ui, "Label · theme setting", |ui| {
        ui.label(egui::RichText::new(text).small())
    })
    .id
}

/// Theme (`ComboBox` of `default` and the platform's presets), Mode (`ComboBox` of System,
/// Light and Dark), Icon theme (`ComboBox`), each under its label and as wide as the panel: the
/// gpui showcase's theme settings (`showcase-gpui/demo.rs:578-603`, `chrome.rs:166-196`).
fn settings_rows(app: &mut App, ui: &mut egui::Ui, body: demo::Applied) {
    let ctx = ui.ctx().clone();
    let theme = ctx.theme();
    let theme_rows = app.theme_rows();
    let current_theme = theme_rows
        .iter()
        .find(|(choice, _)| *choice == app.settings.theme)
        .map(|(_, label)| label.clone())
        .unwrap_or_default();
    let current_mode = app.settings.mode;
    // The gpui showcase's icon-theme list (`showcase-gpui/app.rs:428-456`), less its
    // gpui-component row: `default`, `system`, the installed themes, Lucide, Material.
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
    icon_rows.push(IconSetChoice::Lucide);
    icon_rows.push(IconSetChoice::Material);
    let icon_rows: Vec<(IconSetChoice, String)> = icon_rows
        .into_iter()
        .map(|choice| {
            let text = choice.to_string();
            (choice, text)
        })
        .collect();
    let mode_rows: Vec<(ModeChoice, String)> =
        [ModeChoice::System, ModeChoice::Light, ModeChoice::Dark]
            .into_iter()
            .map(|mode| (mode, Action::SetMode(mode).label().to_string()))
            .collect();
    let current_icon = app.settings.icon.clone();

    let registry = &mut app.registry;
    let picked_theme = setting(
        registry,
        ui,
        body,
        Setting {
            label: "Theme",
            kind: "ComboBox · Theme",
            row_kind: "Theme row",
            rows: &theme_rows,
            current: &app.settings.theme,
            current_text: current_theme,
        },
    );
    let picked_mode = setting(
        registry,
        ui,
        body,
        Setting {
            label: "Mode",
            kind: "ComboBox · Mode",
            row_kind: "Mode row",
            current_text: Action::SetMode(current_mode).label().to_string(),
            rows: &mode_rows,
            current: &current_mode,
        },
    );
    let picked_icon = setting(
        registry,
        ui,
        body,
        Setting {
            label: "Icon theme",
            kind: "ComboBox · Icon theme",
            row_kind: "Icon theme row",
            current_text: current_icon.to_string(),
            rows: &icon_rows,
            current: &current_icon,
        },
    );

    if let Some(choice) = picked_theme {
        app.settings.theme = choice;
        app.install(&ctx);
    }
    if let Some(mode) = picked_mode {
        app.set_mode(mode, &ctx);
    }
    if let Some(choice) = picked_icon {
        app.settings.pick_icon(Some(choice));
        app.settings.theme_installed(&app.atlas, theme);
    }
}

/// One theme setting, as `setting` draws it.
struct Setting<'a, T> {
    label: &'static str,
    kind: &'static str,
    row_kind: &'static str,
    rows: &'a [(T, String)],
    current: &'a T,
    current_text: String,
}

/// A theme setting: its label, then a `ComboBox` as wide as the panel in `Role::ComboBox`, its
/// popup through `popup_style` (§10.4), a `selectable_label` per row. Returns the row picked.
fn setting<T: Clone + PartialEq>(
    registry: &mut Registry,
    ui: &mut egui::Ui,
    body: demo::Applied,
    setting: Setting<'_, T>,
) -> Option<T> {
    let label = setting_label(registry, ui, body, setting.label);
    let mut picked = None;
    demo::scoped_popup(
        registry,
        ui,
        Role::ComboBox,
        RoleVariant::Normal,
        setting.kind,
        |ui, modifier, row, registry| {
            let mut combo = egui::ComboBox::from_id_salt(setting.label)
                .width(ui.available_width())
                .selected_text(setting.current_text);
            if let Some(modifier) = modifier {
                combo = combo.popup_style(modifier);
            }
            combo
                .show_ui(ui, |ui| {
                    for (value, text) in setting.rows {
                        let selected = value == setting.current;
                        if row
                            .add(registry, ui, setting.row_kind, |ui| {
                                ui.selectable_label(selected, text)
                            })
                            .clicked()
                        {
                            picked = Some(value.clone());
                        }
                    }
                })
                .response
                .labelled_by(label)
        },
    );
    picked
}

/// The inspector's two tabs, drawn as the page tabs are (`demo::tab_bar`), padded by
/// `container_margin`, their rule from edge to edge: the gpui showcase's inspector `TabBar`
/// (`showcase-gpui/inspector.rs:389-403`).
fn inspector_tabs(app: &mut App, ui: &mut egui::Ui, margin: Option<f32>) {
    let t = app.atlas.resolved_for(ui.ctx().theme());
    let picked = demo::tab_bar(
        &mut app.registry,
        ui,
        t,
        demo::TabBar {
            kind: "TabBar · Inspector",
            tab_kind: "Tab · Inspector",
            tabs: &[
                (InspectorTab::Widget, "Widget"),
                (InspectorTab::Theme, "Theme"),
            ],
            current: app.inspector_tab,
            margin,
            scroll: false,
        },
        |_, _, _| {},
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
            crate::info::widget_tab(
                ui,
                app.atlas.resolved_for(theme),
                app.registry.shown(),
                &app.manifest,
                app.json.get(&app.atlas, theme),
                app.preset_key(),
            );
        }
        InspectorTab::Theme => crate::info::theme_tab(ui, &app.atlas, &app.manifest),
    }
}

/// The page tabs, the gpui showcase's page `TabBar` (`showcase-gpui/chrome.rs:215-228`): the
/// underline tabs of `demo::tab_bar` in one `Role::Tab` scope (§10.4), padded by
/// `container_margin`, scrolling sideways where the content is too narrow for them, and at the
/// row's right end a Ghost menu button listing every page, the current one selected, as
/// gpui-component's tab-bar menu does (`GC/tab/tab_bar.rs:554-584`). Its caret is the chosen
/// icon theme's `NavDown` at `defaults.icon_sizes.small`; where the theme has none, the button
/// reads "Pages" (§10.4's icon rule).
pub(crate) fn page_tabs(app: &mut App, ui: &mut egui::Ui) {
    let theme = ui.ctx().theme();
    let (set, icon_theme) = app.chosen_icons();
    let margin = app.atlas.layout().container_margin;
    let App {
        registry,
        settings,
        pending,
        atlas,
        ..
    } = app;
    let t = atlas.resolved_for(theme);
    let caret_size = t.defaults.icon_sizes.small;
    let tabs: Vec<(Page, &'static str)> = Page::ALL.map(|page| (page, page.label())).to_vec();
    let current = settings.page;
    let mut from_menu = None;
    let picked = demo::tab_bar(
        registry,
        ui,
        t,
        demo::TabBar {
            kind: "TabBar · Pages",
            tab_kind: "Tab · Page",
            tabs: &tabs,
            current,
            margin,
            scroll: true,
        },
        |ui, tab, registry| {
            let caret = demo::role_image(
                ui,
                IconRole::NavDown,
                set,
                icon_theme.as_deref(),
                caret_size,
            );
            let button = match caret {
                Some(image) => egui::Button::image(image),
                None => egui::Button::new("Pages"),
            }
            .small();
            let response = demo::menu_button(
                registry,
                ui,
                tab,
                "Menu button · Pages",
                button,
                (Role::Menu, RoleVariant::Normal),
                |ui, item, registry| {
                    for page in Page::ALL {
                        let button = egui::Button::new(page.label()).selected(page == current);
                        if item
                            .add(registry, ui, "Menu item · Pages", |ui| ui.add(button))
                            .clicked()
                        {
                            from_menu = Some(page);
                            ui.close();
                        }
                    }
                },
            );
            registry.ghost_last();
            response
                .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Pages"));
        },
    );
    if let Some(page) = picked.or(from_menu) {
        pending.push(Action::ShowPage(page));
    }
}

/// The content: the central panel surface fed from `theme.defaults`; the page tabs flush at its
/// top, then — padded by the surface's own inner margin — a theme error in a card, and the page.
pub(crate) fn central_panel(
    app: &mut App,
    ui: &mut egui::Ui,
    add: impl FnOnce(&mut App, &mut egui::Ui),
) {
    let mut seams = PanelSeams::apply(ui, Surface::CentralPanel, None, None);
    let page_margin = seams.lift_margin();
    let out = egui::CentralPanel::default()
        .frame(seams.frame)
        .show(ui, |ui| {
            page_tabs(app, ui);
            egui::Frame::NONE.inner_margin(page_margin).show(ui, |ui| {
                if let Some(error) = app.theme_error.clone() {
                    // As the gpui showcase's error banner (`showcase-gpui/demo.rs:1342-1357`):
                    // across the content, the chosen icon theme's error icon first, where it
                    // has one — `IconRole::DialogError`, the role gpui's `CircleX` maps to
                    // (`showcase-gpui/support.rs:435`) — at `defaults.icon_sizes.small`.
                    let (set, icon_theme) = app.chosen_icons();
                    let size = app
                        .atlas
                        .resolved_for(ui.ctx().theme())
                        .defaults
                        .icon_sizes
                        .small;
                    demo::framed(
                        &mut app.registry,
                        ui,
                        Surface::Card,
                        None,
                        "theme error",
                        |ui, reg| {
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                let icon = demo::role_image(
                                    ui,
                                    IconRole::DialogError,
                                    set,
                                    icon_theme.as_deref(),
                                    size,
                                );
                                if let Some(icon) = icon {
                                    demo::base(reg, ui, "Image · theme error", |ui| ui.add(icon));
                                }
                                demo::base(reg, ui, "theme error text", |ui| {
                                    ui.colored_label(ui.visuals().error_fg_color, error)
                                });
                            });
                        },
                    );
                }
                add(app, ui);
            });
        });
    seams.record(&mut app.registry, &out.response, "Central panel");
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

/// A dialog's title row, as gpui-component's `Dialog` draws one (`GC/dialog/dialog.rs:182`) and
/// the gpui showcase titles its palette and About (`showcase-gpui/demo.rs:1013`, `:1048`): the
/// title in `dialog.title_font` — its size is the dialog role's `Heading` slot (§5), its colour
/// read from the theme — and flush right a Ghost close button, the chosen icon theme's
/// `WindowClose` at `defaults.icon_sizes.small`, or "Close" where the theme has none (§10.4's
/// icon rule). Added through the dialog body's seam; returns whether the button was clicked.
fn dialog_title(
    reg: &mut Registry,
    ui: &mut egui::Ui,
    body: demo::Applied,
    t: &native_theme::theme::ResolvedTheme,
    (set, icon_theme): &(IconSet, Option<String>),
    title: &'static str,
) -> bool {
    let colour = native_theme_egui::convert::to_color32(t.dialog.title_font.color);
    let image = demo::role_image(
        ui,
        IconRole::WindowClose,
        *set,
        icon_theme.as_deref(),
        t.defaults.icon_sizes.small,
    );
    let mut clicked = false;
    ui.horizontal(|ui| {
        body.add(reg, ui, "Label · dialog title", |ui| {
            ui.label(egui::RichText::new(title).heading().color(colour))
        });
        reg.amend_last(|i| {
            i.read.push((
                "dialog.title_font.color",
                t.dialog.title_font.color.to_string(),
            ));
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            demo::ghost(ui);
            let button = match image {
                Some(image) => egui::Button::image(image),
                None => egui::Button::new("Close"),
            };
            let r = body.add(reg, ui, "Button · dialog close", |ui| ui.add(button));
            reg.ghost_last();
            r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Close"));
            clicked = r.clicked();
        });
    });
    clicked
}

/// What a palette row runs: an action, or a preset to install.
enum Pick {
    Action(Action),
    Theme(ThemeChoice),
}

/// A palette row: its label, what the query matches, what it runs.
type PaletteRow = (String, String, Pick);

/// The command palette's groups, as the gpui showcase's (`showcase-gpui/chrome.rs:374-425`):
/// the pages, the presets the Theme row offers by their display names, found by their keys too,
/// and the colour modes, System named by the scheme it follows now.
fn palette_groups(app: &App, ctx: &egui::Context) -> [(&'static str, Vec<PaletteRow>); 3] {
    let pages = Page::ALL
        .into_iter()
        .map(|page| {
            let label = page.label().to_string();
            (label.clone(), label, Pick::Action(Action::ShowPage(page)))
        })
        .collect();
    let presets = app
        .theme_rows()
        .into_iter()
        .map(|(choice, name)| {
            let key = match &choice {
                ThemeChoice::Default => "default".to_string(),
                ThemeChoice::Preset(key) => key.clone(),
            };
            (name.clone(), format!("{name} {key}"), Pick::Theme(choice))
        })
        .collect();
    // The scheme `System` follows: what the integration reports, else the atlas's own reading
    // of the OS (`ThemeAtlas::os_mode`, §10.3).
    let system = ctx
        .system_theme()
        .map(|theme| theme == egui::Theme::Dark)
        .or_else(|| app.atlas.os_mode().map(|mode| mode.is_dark()));
    let modes = [ModeChoice::System, ModeChoice::Light, ModeChoice::Dark]
        .into_iter()
        .map(|mode| {
            let label = match (mode, system) {
                (ModeChoice::System, Some(true)) => "System (Dark)".to_string(),
                (ModeChoice::System, Some(false)) => "System (Light)".to_string(),
                _ => Action::SetMode(mode).label().to_string(),
            };
            (label.clone(), label, Pick::Action(Action::SetMode(mode)))
        })
        .collect();
    [
        ("Pages", pages),
        ("Presets", presets),
        ("Colour mode", modes),
    ]
}

/// The command palette (§10.4), as the gpui showcase's (`showcase-gpui/demo.rs:1002-1033`): a
/// `Modal` in the dialog surface, its body in `Role::Dialog`, titled "Command Palette" with a
/// close button, a focused field, then rows in `Role::List` grouped under small weak headers.
/// Escape clears, then closes.
pub(crate) fn command_palette(app: &mut App, ui: &mut egui::Ui) {
    let Some(palette) = app.palette.clone() else {
        return;
    };
    let t = app.atlas.resolved_for(ui.ctx().theme());
    // The four dialog extents are required sizes (`f32` on the resolved theme).
    let extents = [
        t.dialog.min_width,
        t.dialog.max_width,
        t.dialog.min_height,
        t.dialog.max_height,
    ];
    let groups = palette_groups(app, ui.ctx());
    let chosen_icons = app.chosen_icons();
    let mut query = palette.query.clone();
    let mut chosen: Option<Action> = None;
    let mut chosen_theme: Option<ThemeChoice> = None;
    let mut should_close = false;
    let mut close_clicked = false;
    let App {
        registry,
        settings,
        atlas,
        ..
    } = app;
    let t = atlas.resolved_for(ui.ctx().theme());
    demo::surfaced(
        registry,
        ui,
        Surface::Dialog,
        false,
        None,
        "command palette backdrop",
        |ui, chrome, registry| {
            let [min_w, max_w, min_h, max_h] = extents;
            let min_w = inner_extent(min_w, &chrome.frame, true);
            let max_w = inner_extent(max_w, &chrome.frame, true);
            let min_h = inner_extent(min_h, &chrome.frame, false);
            let max_h = inner_extent(max_h, &chrome.frame, false);
            let out = egui::Modal::new(egui::Id::new("command-palette"))
                .frame(chrome.frame)
                .show(ui.ctx(), |ui| {
                    let dialog = demo::styled(
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
                    close_clicked =
                        dialog_title(registry, ui, dialog, t, &chosen_icons, "Command Palette");
                    let field = dialog.add(registry, ui, "palette query", |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut query)
                                .hint_text("A page, a preset or a colour mode…")
                                .desired_width(ui.available_width()),
                        )
                    });
                    if !field.has_focus() && palette.query.is_empty() {
                        field.request_focus();
                    }
                    let needle = query.to_lowercase();
                    demo::scoped_container(
                        registry,
                        ui,
                        Role::List,
                        RoleVariant::Normal,
                        "palette rows",
                        |ui, row, registry| {
                            // The rows scroll within the dialog's height and span its width, as
                            // gpui's `Command` list's do.
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, true])
                                .show(ui, |ui| {
                                    ui.with_layout(
                                        egui::Layout::top_down_justified(egui::Align::Min),
                                        |ui| {
                                            for (header, rows) in groups {
                                                let rows: Vec<_> = rows
                                                    .into_iter()
                                                    .filter(|(_, matched, _)| {
                                                        matched.to_lowercase().contains(&needle)
                                                    })
                                                    .collect();
                                                if rows.is_empty() {
                                                    continue;
                                                }
                                                // A command group's header, small and muted.
                                                row.add(registry, ui, "palette group", |ui| {
                                                    ui.label(
                                                        egui::RichText::new(header).small().weak(),
                                                    )
                                                });
                                                for (label, _, pick) in rows {
                                                    let r = row.add(
                                                        registry,
                                                        ui,
                                                        "palette row",
                                                        |ui| ui.selectable_label(false, &label),
                                                    );
                                                    if r.clicked() {
                                                        match pick {
                                                            Pick::Action(action) => {
                                                                chosen = Some(action)
                                                            }
                                                            Pick::Theme(choice) => {
                                                                chosen_theme = Some(choice);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                    )
                                    .response
                                })
                                .inner
                        },
                    );
                });
            // Escape: clear the query first; only an empty query lets it close the modal
            // (§10.4). It is consumed here, so `should_close` below sees it only when the query
            // was already empty, and still answers a click on the backdrop, which closes
            // whatever the query holds.
            let escape = ui.ctx().input_mut(|i| {
                !query.is_empty() && i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)
            });
            if escape {
                query.clear();
            }
            should_close = out.should_close();
            Some(out.backdrop_response)
        },
    );
    let close = chosen.is_some() || chosen_theme.is_some() || should_close || close_clicked;
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

/// The text-scaling field's range and step: the gpui showcase's Preferences number field
/// (`connectors/native-theme-gpui/examples/showcase-gpui/demo.rs:1199-1206`), whose range is
/// Windows' `UISettings.TextScaleFactor` (platform-facts §1.2.7) and whose step divides it
/// evenly. The model states none; they are not style values.
const TEXT_SCALE_MIN: f32 = 1.0;
const TEXT_SCALE_MAX: f32 = 2.25;
const TEXT_SCALE_STEP: f32 = 0.25;

/// A Preferences row, as a gpui `SettingItem` lays one out: `control` flush right, the title and
/// its weak description on the left, wrapping in the room left of it. Returns the control's
/// response.
fn preference_row(
    registry: &mut Registry,
    ui: &mut egui::Ui,
    body: demo::Applied,
    (title, description): (&str, &str),
    control: impl FnOnce(&mut egui::Ui, &mut Registry) -> egui::Response,
) -> egui::Response {
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let response = control(ui, registry);
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                body.add(registry, ui, "Label · preference", |ui| ui.label(title));
                body.add(registry, ui, "Label · preference description", |ui| {
                    ui.label(egui::RichText::new(description).small().weak())
                });
            });
            response
        })
        .inner
    })
    .inner
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
    // `dialog.max_width`, a required size (`f32` on the resolved theme).
    let dialog_width = t.dialog.max_width;
    let mut prefs = app
        .settings
        .prefs
        .clone()
        .unwrap_or_else(|| app.atlas.accessibility().clone());
    let before = prefs.clone();
    // Whether the text-scaling field is still being dragged, or was just let go of.
    let mut scale_dragged = false;
    let mut scale_settled = false;
    let mut open = app.preferences_open;
    demo::surfaced(
        &mut app.registry,
        ui,
        Surface::Window,
        true,
        None,
        "preferences window",
        |_, chrome, registry| {
            // As wide as the theme lets a dialog be: the gpui showcase's sheet is its own 600px
            // (`PREFERENCES_WIDTH`), which the model states nothing of.
            let mut window = egui::Window::new(title)
                .id(egui::Id::new("preferences"))
                .open(&mut open)
                .default_width(dialog_width)
                .frame(chrome.frame);
            if let Some(title_frame) = chrome.title_frame {
                window = window.title_frame(title_frame);
            }
            window
                .show(&ctx, |ui| {
                    let body = demo::styled(
                        registry,
                        ui,
                        Role::Window,
                        RoleVariant::Normal,
                        "preferences",
                    );
                    // The gpui showcase's Settings page (`showcase-gpui/demo.rs:1249-1331`): its
                    // title and description, then a row per preference — its title and a weak
                    // description on the left, its control on the right.
                    body.add(registry, ui, "heading", |ui| {
                        let heading = demo::heading_text(ui, "Accessibility");
                        ui.label(heading)
                    });
                    body.add(registry, ui, "Label · description", |ui| {
                        ui.label(
                            egui::RichText::new(
                                "Installed with native_theme_egui's Builder::accessibility",
                            )
                            .small()
                            .weak(),
                        )
                    });
                    let scale = preference_row(
                        registry,
                        ui,
                        body,
                        (
                            "Text scale",
                            "text_scaling_factor: every text size the atlas writes is multiplied by it",
                        ),
                        |ui, registry| {
                            demo::scoped(
                                registry,
                                ui,
                                Role::Input,
                                RoleVariant::Normal,
                                "text scaling factor",
                                |ui| {
                                    ui.add(
                                        egui::DragValue::new(&mut prefs.text_scaling_factor)
                                            .range(TEXT_SCALE_MIN..=TEXT_SCALE_MAX)
                                            // The OS's own factor is shown as it is until the user edits it.
                                            .clamp_existing_to_range(false)
                                            .speed(TEXT_SCALE_STEP),
                                    )
                                },
                            )
                        },
                    );
                    if scale.changed() {
                        // The field's steps: a drag moves the value continuously (`speed` is per point).
                        let steps = (prefs.text_scaling_factor / TEXT_SCALE_STEP).round();
                        prefs.text_scaling_factor =
                            (steps * TEXT_SCALE_STEP).clamp(TEXT_SCALE_MIN, TEXT_SCALE_MAX);
                    }
                    scale_dragged = scale.dragged();
                    scale_settled = scale.drag_stopped() || scale.lost_focus();
                    // Switches, as gpui's Settings rows have (§5.3's spelling: a
                    // `Button::new(..).selected(on)` in `Role::Switch`, whose own flag picks the
                    // checked look, §6.2).
                    for (kind, title, description, flag) in [
                                (
                                    "switch · reduce motion",
                                    "Reduce motion",
                                    "reduce_motion: every style's animation time is zero and its scroll animation is off",
                                    &mut prefs.reduce_motion,
                                ),
                                (
                                    "switch · high contrast",
                                    "High contrast",
                                    "high_contrast: stored with the atlas, which builds no differently for it",
                                    &mut prefs.high_contrast,
                                ),
                                (
                                    "switch · reduce transparency",
                                    "Reduce transparency",
                                    "reduce_transparency: stored with the atlas, which builds no differently for it",
                                    &mut prefs.reduce_transparency,
                                ),
                            ] {
                                let on = *flag;
                                let r = preference_row(registry, ui, body, (title, description), |ui, registry| {
                                    demo::scoped(
                                        registry,
                                        ui,
                                        Role::Switch,
                                        RoleVariant::Normal,
                                        kind,
                                        |ui| {
                                            let r = ui.add(
                                                egui::Button::new(if on { "On" } else { "Off" })
                                                    .selected(on),
                                            );
                                            r.widget_info(|| {
                                                egui::WidgetInfo::selected(
                                                    egui::WidgetType::Button,
                                                    true,
                                                    on,
                                                    title,
                                                )
                                            });
                                            r
                                        },
                                    )
                                });
                                if r.clicked() {
                                    *flag = !on;
                                }
                            }
                })
                .map(|out| out.response)
        },
    );
    app.preferences_open = open;
    let changed = prefs != before;
    if changed {
        app.settings.prefs = Some(prefs);
    }
    // A rebuild per pass of a drag would re-read the OS for `default` and rebuild the font
    // atlas on every frame: the drag installs once, when it ends.
    let differs = app
        .settings
        .prefs
        .as_ref()
        .is_some_and(|p| p != app.atlas.accessibility());
    if differs && !scale_dragged && (changed || scale_settled) {
        app.install(&ctx);
    }
}

/// The connector README's Compatibility table at this version's tag, as the gpui showcase links
/// its own (`showcase-gpui/chrome.rs:328-333`, `COMPATIBILITY_URL`).
const COMPATIBILITY_URL: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    "/blob/v",
    env!("CARGO_PKG_VERSION"),
    "/connectors/native-theme-egui/README.md#compatibility"
);

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
    let chosen_icons = app.chosen_icons();
    let mut close = false;
    demo::surfaced(
        &mut app.registry,
        ui,
        Surface::Dialog,
        false,
        None,
        "about",
        |_, chrome, registry| {
            let max_w = inner_extent(t.dialog.max_width, &chrome.frame, true);
            let response = egui::Modal::new(egui::Id::new("about")).frame(chrome.frame).show(&ctx, |ui| {
        let body = demo::styled(registry, ui, Role::Dialog, RoleVariant::Normal, "about body");
        if let Some(w) = max_w {
            ui.set_max_width(w);
        }
        // The gpui showcase's About (`showcase-gpui/demo.rs:1038-1092`): its title, the name
        // and version, then the description in `dialog.body_font` — the dialog role's `Body`
        // slot and text colour (§5) — ending in the link to the Compatibility table at this
        // version's tag.
        close |= dialog_title(registry, ui, body, t, &chosen_icons, "About");
        body.add(registry, ui, "about version", |ui| ui.label(version));
        // The link on a line of its own: a widget in a role scope of its own sits in a child
        // `Ui` a wrapping row cannot carry on to its next line (`demo::Applied`).
        body.add(registry, ui, "Label · about description", |ui| {
            ui.label("The egui and egui_extras versions it requires, and those it was verified against, are in")
        });
        demo::scoped(registry, ui, Role::Link, RoleVariant::Normal, "compatibility link", |ui| {
            ui.hyperlink_to(
                concat!("the README's Compatibility table at v", env!("CARGO_PKG_VERSION")),
                COMPATIBILITY_URL,
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
            close |= response.should_close();
            Some(response.response)
        },
    );
    if close {
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
