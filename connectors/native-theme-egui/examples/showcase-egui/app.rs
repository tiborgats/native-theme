//! The application: its settings, its atlas, and the `eframe::App` impl (spec §10.4).

#[cfg(feature = "watch")]
use std::sync::Arc;

use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme::{AccessibilityPreferences, SystemTheme, theme::IconSet};
use native_theme_egui::{SystemThemeExt as _, ThemeAtlas, from_preset};

use crate::chrome::{self, Action, InspectorTab, PaletteState};
use crate::{CliArgs, SCREENSHOT_DELAY_S, apply_cli_args, demo, info, pages};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Page {
    Basic,
    Buttons,
    Selection,
    Inputs,
    Range,
    Text,
    Colour,
    Containers,
    Data,
    Overlays,
    Icons,
    ThemeMap,
}

impl Page {
    pub(crate) const ALL: [Page; 12] = [
        Page::Basic,
        Page::Buttons,
        Page::Selection,
        Page::Inputs,
        Page::Range,
        Page::Text,
        Page::Colour,
        Page::Containers,
        Page::Data,
        Page::Overlays,
        Page::Icons,
        Page::ThemeMap,
    ];
    /// The `--tab` name (§10.4's palette table).
    pub(crate) fn key(self) -> &'static str {
        match self {
            Page::Basic => "basic",
            Page::Buttons => "buttons",
            Page::Selection => "selection",
            Page::Inputs => "inputs",
            Page::Range => "range",
            Page::Text => "text",
            Page::Colour => "colour",
            Page::Containers => "containers",
            Page::Data => "data",
            Page::Overlays => "overlays",
            Page::Icons => "icons",
            Page::ThemeMap => "theme-map",
        }
    }
    pub(crate) fn label(self) -> &'static str {
        match self {
            Page::Basic => "Basic",
            Page::Buttons => "Buttons",
            Page::Selection => "Selection",
            Page::Inputs => "Inputs",
            Page::Range => "Range",
            Page::Text => "Text",
            Page::Colour => "Colour",
            Page::Containers => "Containers",
            Page::Data => "Data",
            Page::Overlays => "Overlays",
            Page::Icons => "Icons",
            Page::ThemeMap => "Theme Map",
        }
    }
    pub(crate) fn from_key(key: &str) -> Result<Page, String> {
        Page::ALL
            .into_iter()
            .find(|p| p.key() == key)
            .ok_or_else(|| {
                let keys: Vec<&str> = Page::ALL.iter().map(|p| p.key()).collect();
                format!(
                    "--tab {key}: no such page; the pages are: {}",
                    keys.join(", ")
                )
            })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ModeChoice {
    System,
    Light,
    Dark,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ThemeChoice {
    Default,
    Preset(String),
}

/// What the pickers, the command line and Preferences select. Pure: T11 (c)
/// tests it with no `Context`, and the watcher's `rebuild` reads a copy.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Settings {
    pub theme: ThemeChoice,
    pub mode: ModeChoice,
    /// The icon theme the pages and the toolbar draw from.
    pub icon: IconSetChoice,
    /// Whether `icon` is re-derived from each theme installed: until the user picks an icon
    /// theme, and again after a pick of the `default` row (§10.4).
    pub icon_follows_theme: bool,
    /// Preferences' overrides; `None` is the OS's own.
    pub prefs: Option<AccessibilityPreferences>,
    pub page: Page,
    pub screenshot: Option<String>,
    /// `native_theme::icons::list_freedesktop_themes()`, read once.
    pub installed_themes: Vec<String>,
}

impl Settings {
    pub(crate) fn initial() -> Self {
        Self {
            theme: ThemeChoice::Default,
            mode: ModeChoice::System,
            // Re-derived by the first `theme_installed`, since it follows the theme.
            icon: IconSetChoice::System,
            icon_follows_theme: true,
            prefs: None,
            page: Page::Basic,
            screenshot: None,
            installed_themes: native_theme::icons::list_freedesktop_themes(),
        }
    }
    #[cfg(test)]
    pub(crate) fn for_tests() -> Self {
        Self {
            installed_themes: Vec::new(),
            ..Self::initial()
        }
    }
    pub(crate) fn mode_choice(variant: &str) -> Result<ModeChoice, String> {
        match variant {
            "light" => Ok(ModeChoice::Light),
            "dark" => Ok(ModeChoice::Dark),
            "system" => Ok(ModeChoice::System),
            other => Err(format!("--variant {other}: not light, dark or system")),
        }
    }
    /// `default`, or a preset the Theme row offers on this platform.
    pub(crate) fn theme_choice(name: &str) -> Result<ThemeChoice, String> {
        if name == "default" {
            return Ok(ThemeChoice::Default);
        }
        let offered = native_theme::theme::Theme::list_presets_for_platform();
        if offered.iter().any(|info| info.key == name) {
            return Ok(ThemeChoice::Preset(name.to_string()));
        }
        let names: Vec<&str> = std::iter::once("default")
            .chain(offered.iter().map(|info| info.key))
            .collect();
        let names = names.join(", ");
        if native_theme::theme::Theme::list_presets()
            .iter()
            .any(|info| info.key == name)
        {
            Err(format!(
                "--theme {name}: a preset of another platform; only this platform's presets run here: {names}"
            ))
        } else {
            Err(format!(
                "--theme {name}: no such preset; the presets are: {names}"
            ))
        }
    }
    /// What the icon-theme picker offers: `default` (`None`: follow the theme), a bundled set,
    /// `system` or `freedesktop` (the OS's icon theme), or an installed freedesktop theme.
    pub(crate) fn icon_choice(&self, name: &str) -> Result<Option<IconSetChoice>, String> {
        match name {
            "default" => Ok(None),
            "material" => Ok(Some(IconSetChoice::Material)),
            "lucide" => Ok(Some(IconSetChoice::Lucide)),
            "system" | "freedesktop" => Ok(Some(IconSetChoice::System)),
            theme if self.installed_themes.iter().any(|t| t == theme) => {
                Ok(Some(IconSetChoice::Freedesktop(theme.to_string())))
            }
            other => Err(format!(
                "--icon-set {other}: not default, material, lucide, system, freedesktop or an installed icon theme; installed: {}",
                if self.installed_themes.is_empty() {
                    "none".to_string()
                } else {
                    self.installed_themes.join(", ")
                }
            )),
        }
    }
    /// A pick: `None` is the `default` row, which follows the theme again (re-derived at the
    /// next `theme_installed`); a `Default(_)` row picked from the list follows too
    /// (`IconSetChoice::follows_preset`); anything else stays chosen.
    pub(crate) fn pick_icon(&mut self, pick: Option<IconSetChoice>) {
        match pick {
            Some(choice) => {
                self.icon_follows_theme = choice.follows_preset();
                self.icon = choice;
            }
            None => self.icon_follows_theme = true,
        }
    }
    /// A theme was installed, or the scheme switched: re-derive the icon choice while it
    /// follows the theme (§10.4) — the preset's icon theme for `theme` where it is installed,
    /// else `system` (`default_icon_choice`).
    pub(crate) fn theme_installed(&mut self, atlas: &ThemeAtlas, theme: egui::Theme) {
        if self.icon_follows_theme {
            self.icon = default_icon_choice(atlas.icon_set(), atlas.icon_theme(theme));
        }
    }
}

/// `--screenshot`: where the capture goes, when the first pass ran, the capture while under
/// way, whether it was written.
struct Screenshot {
    path: String,
    started: Option<f64>,
    capture: Option<crate::capture::FrameCapture>,
    written: bool,
}

/// `--dump-layout`: where the dump goes, whether the showcase keeps running after it, the layout
/// of the last pass and the input time it was first seen at, and whether it was written.
struct LayoutDump {
    path: String,
    keep_running: bool,
    last: Option<(std::collections::BTreeMap<String, egui::Rect>, f64)>,
    written: bool,
}

/// The layout dump's JSON (`docs/showcase-elements.toml`, "The layout dump"): each placed
/// element's rectangle in logical pixels, window-content coordinates.
pub(crate) fn layout_json(
    places: &std::collections::BTreeMap<String, egui::Rect>,
    preset: &str,
    theme: egui::Theme,
    scale: f32,
) -> serde_json::Value {
    let elements: serde_json::Map<String, serde_json::Value> = places
        .iter()
        .map(|(id, r)| {
            (
                id.clone(),
                serde_json::json!({"x": r.min.x, "y": r.min.y, "w": r.width(), "h": r.height()}),
            )
        })
        .collect();
    serde_json::json!({
        "kind": "egui",
        "preset": preset,
        "variant": match theme {
            egui::Theme::Light => "light",
            egui::Theme::Dark => "dark",
        },
        "scale": scale,
        "elements": elements,
    })
}

/// The pointer `--pointer` holds at a point of the window, and whether `--press` holds the
/// primary button down there: for a capture of a control hovered or pressed where nothing can
/// move the real pointer, as in a nested compositor. egui reads the pointer from the input it is
/// handed each pass, so the move to the point and the press are handed to it in a pass's input
/// (`eframe::App::raw_input_hook`), in place of the window's own pointer events.
struct HeldPointer {
    at: egui::Pos2,
    press: bool,
    moved: bool,
    pressed: bool,
}

impl HeldPointer {
    /// How long, in seconds of egui's input time, the pointer is held before the press: long
    /// enough for the window to open at its size and the page to be laid out under the pointer.
    /// No repository script passes `--press`. The capture scripts capture a showcase at least
    /// four seconds after starting it (`DELAY=3` in `scripts/generate_screenshots_*.sh`, then
    /// `capture_showcase`'s `sleep 1` in `scripts/capture_window.sh`), so a press three seconds
    /// after the first frame lands about a second before such a capture; a capture with
    /// `--press` should wait longer.
    const PRESS_AFTER: f64 = 3.0;
}

/// Whether `event` moves, presses, touches or leaves with the window's own pointer.
fn pointer_event(event: &egui::Event) -> bool {
    matches!(
        event,
        egui::Event::PointerMoved(_)
            | egui::Event::MouseMoved(_)
            | egui::Event::PointerButton { .. }
            | egui::Event::PointerGone
            | egui::Event::Touch { .. }
    )
}

pub(crate) struct App {
    /// The one field for theme state (§10.4's third rule).
    pub(crate) atlas: ThemeAtlas,
    pub(crate) settings: Settings,
    /// The platform preset `default` builds on, `SystemTheme::preset` since the last read of
    /// the OS: the name the Theme row, the command palette and the status bar give `default`,
    /// "default (kde-breeze)", as the gpui showcase's (`showcase-gpui/support.rs:1031-1036`).
    pub(crate) default_preset: String,
    /// The OS's semibold face of the theme's family, registered after each install.
    pub(crate) semibold: demo::Semibold,
    pub(crate) theme_error: Option<String>,
    pub(crate) registry: demo::Registry,
    pub(crate) demo_state: pages::DemoState,
    pub(crate) side_panel_visible: bool,
    /// The command palette, while it is open.
    pub(crate) palette: Option<PaletteState>,
    pub(crate) preferences_open: bool,
    pub(crate) about_open: bool,
    pub(crate) inspector_tab: InspectorTab,
    /// Quit was run; the close itself is `ViewportCommand::Close`.
    pub(crate) quit_requested: bool,
    /// Actions the chrome's widgets asked for this pass, run after the pass's input is read.
    pub(crate) pending: Vec<Action>,
    /// The embedded `mapping.toml`, parsed once; an error is shown in the inspector (§10.4).
    pub(crate) manifest: Result<info::Manifest, String>,
    /// `docs/showcase-elements.toml`, parsed once; an error is shown in the inspector.
    pub(crate) elements: Result<Vec<crate::elements::ShowcaseElement>, String>,
    /// The installed atlas's JSON per scheme, emptied on each install.
    pub(crate) json: info::JsonCache,
    /// The inspector's content rect this pass: Widget Info's hold zone (§10.4).
    pub(crate) hold_zone: Option<egui::Rect>,
    /// Whether the inspector's content was taller than its area this pass.
    #[cfg(test)]
    pub(crate) inspector_scrolls: bool,
    /// Whether the page was taller than the room it is shown in this pass.
    #[cfg(test)]
    pub(crate) page_scrolls: bool,
    /// The scheme the icon choice was last derived for; `None` forces a re-derive next pass.
    last_scheme: Option<egui::Theme>,
    screenshot: Option<Screenshot>,
    /// `--dump-layout`, until the dump is written.
    dump: Option<LayoutDump>,
    /// `--pointer` and `--press`: the pointer held at a point, and the primary button held down.
    held_pointer: Option<HeldPointer>,
    /// `--capture` or `--screenshot`: the window is captured, so the real pointer draws nothing
    /// (`raw_input_hook`).
    capturing: bool,
    /// What the watcher's `rebuild` reads: the UI thread writes it on each install.
    #[cfg(feature = "watch")]
    selection: Arc<std::sync::RwLock<Settings>>,
    #[cfg(feature = "watch")]
    watcher: Option<native_theme_egui::ThemeWatcher>,
    /// The macOS system menu bar, owned here: a muda `Menu` holds `Rc`s and is not `Send`.
    #[cfg(all(target_os = "macos", not(test)))]
    system_menu: Option<crate::chrome::system_menu::SystemMenu>,
    /// The system menu's clicks, drained into `pending` by `logic`.
    #[cfg(all(target_os = "macos", not(test)))]
    menu_rx: Option<std::sync::mpsc::Receiver<Action>>,
}

impl App {
    /// Build the selected atlas once and install it (§10.4, "The atlas the showcase installs").
    /// Its error is shown on the page and the platform's own preset stands in; if that fails
    /// too, the error ends the start-up (eframe's `AppCreator` is fallible).
    pub(crate) fn new(
        cc: &eframe::CreationContext<'_>,
        cli: &CliArgs,
    ) -> native_theme::Result<Self> {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut settings = Settings::initial();
        apply_cli_args(&mut settings, cli);
        let mut default_preset = native_theme::pipeline::platform_preset_name()
            .name
            .to_string();
        let (atlas, theme_error) = match Self::rebuild_atlas(&settings) {
            Ok((atlas, preset)) => {
                if let Some(preset) = preset {
                    default_preset = preset;
                }
                (atlas, None)
            }
            Err(error) => {
                let (atlas, _) = from_preset(
                    &default_preset,
                    false,
                    &AccessibilityPreferences::from_system(),
                )?;
                (atlas, Some(error.to_string()))
            }
        };
        let screenshot = settings.screenshot.clone().map(|path| Screenshot {
            path,
            started: None,
            capture: None,
            written: false,
        });
        #[cfg(feature = "watch")]
        let selection = Arc::new(std::sync::RwLock::new(settings.clone()));
        let mut app = Self {
            atlas,
            settings,
            default_preset,
            semibold: demo::Semibold::default(),
            theme_error,
            registry: demo::Registry::default(),
            demo_state: pages::DemoState::default(),
            side_panel_visible: true,
            palette: None,
            preferences_open: false,
            about_open: false,
            inspector_tab: InspectorTab::Widget,
            quit_requested: false,
            pending: Vec::new(),
            manifest: info::Manifest::parse(include_str!("../../mapping.toml")),
            elements: crate::elements::showcase_elements(),
            json: info::JsonCache::default(),
            hold_zone: None,
            #[cfg(test)]
            inspector_scrolls: false,
            #[cfg(test)]
            page_scrolls: false,
            last_scheme: None,
            screenshot,
            dump: cli.dump_layout.clone().map(|path| LayoutDump {
                path,
                keep_running: cli.capturing(),
                last: None,
                written: false,
            }),
            held_pointer: cli.pointer.map(|(x, y)| HeldPointer {
                at: egui::pos2(f32::from(x), f32::from(y)),
                press: cli.press,
                moved: false,
                pressed: false,
            }),
            capturing: cli.capturing(),
            #[cfg(feature = "watch")]
            selection,
            #[cfg(feature = "watch")]
            watcher: None,
            #[cfg(all(target_os = "macos", not(test)))]
            system_menu: None,
            #[cfg(all(target_os = "macos", not(test)))]
            menu_rx: None,
        };
        app.apply(&cc.egui_ctx);
        #[cfg(all(target_os = "macos", not(test)))]
        {
            let (tx, rx) = std::sync::mpsc::channel();
            match crate::chrome::system_menu::build(&cc.egui_ctx, tx) {
                Ok(menu) => {
                    app.system_menu = Some(menu);
                    app.menu_rx = Some(rx);
                }
                Err(error) => eprintln!("ERROR: the macOS menu bar did not install: {error}"),
            }
        }
        #[cfg(feature = "watch")]
        if !cfg!(test) && app.settings.screenshot.is_none() {
            app.start_watcher(&cc.egui_ctx);
        }
        Ok(app)
    }

    /// The atlas `settings` select (§10.4): `default` is `SystemTheme::from_system` through
    /// `to_egui_atlas`, with Preferences' overrides on its public `accessibility`
    /// (`native-theme/src/lib.rs:500`) after `invalidate_caches()`; a preset is `from_preset`,
    /// whose `is_dark` selects only the `ResolvedTheme` this discards — the atlas carries both
    /// variants and egui picks the scheme (§4.6) — so `false` is passed. With the atlas comes,
    /// for `default`, the platform preset it builds on (`SystemTheme::preset`).
    pub(crate) fn rebuild_atlas(
        settings: &Settings,
    ) -> native_theme::Result<(ThemeAtlas, Option<String>)> {
        match &settings.theme {
            ThemeChoice::Default => {
                native_theme::detect::invalidate_caches();
                let mut sys = SystemTheme::from_system()?;
                if let Some(overrides) = &settings.prefs {
                    sys.accessibility = overrides.clone();
                }
                let preset = sys.preset.clone();
                Ok((sys.to_egui_atlas(), Some(preset)))
            }
            ThemeChoice::Preset(name) => {
                let prefs = settings
                    .prefs
                    .clone()
                    .unwrap_or_else(AccessibilityPreferences::from_system);
                from_preset(name, false, &prefs).map(|(atlas, _)| (atlas, None))
            }
        }
    }

    /// Rebuild what `settings` select and install it; a failure keeps the installed atlas and
    /// is shown on the page (§10.4, the theme error in the content).
    pub(crate) fn install(&mut self, ctx: &egui::Context) {
        match Self::rebuild_atlas(&self.settings) {
            Ok((atlas, preset)) => {
                self.atlas = atlas;
                if let Some(preset) = preset {
                    self.default_preset = preset;
                }
                self.theme_error = None;
            }
            Err(error) => self.theme_error = Some(error.to_string()),
        }
        self.apply(ctx);
    }

    /// Install the held atlas and the mode's theme preference (egui's own call, §10.3).
    fn apply(&mut self, ctx: &egui::Context) {
        self.atlas.install(ctx);
        self.semibold.register(ctx, &self.atlas);
        ctx.set_theme(self.theme_preference());
        self.json = info::JsonCache::default();
        self.last_scheme = None;
        self.registry.screen_changed();
        #[cfg(feature = "watch")]
        self.publish_selection();
    }

    /// Switch the mode: egui's own theme call and nothing else (§10.4's side-panel row). The
    /// atlas carries both schemes and egui picks one (§4.6), so nothing is rebuilt, the OS is
    /// not read again, and a mode switch cannot fail.
    pub(crate) fn set_mode(&mut self, mode: ModeChoice, ctx: &egui::Context) {
        self.settings.mode = mode;
        ctx.set_theme(self.theme_preference());
        #[cfg(feature = "watch")]
        self.publish_selection();
    }

    fn theme_preference(&self) -> egui::ThemePreference {
        match self.settings.mode {
            ModeChoice::System => egui::ThemePreference::System,
            ModeChoice::Light => egui::ThemePreference::Light,
            ModeChoice::Dark => egui::ThemePreference::Dark,
        }
    }

    /// What the Theme row and the command palette offer, as `(choice, display name)`: `default`
    /// named by the preset it builds on, then this platform's presets by their display names,
    /// as the gpui showcase's `preset_items` (`showcase-gpui/support.rs:1038-1056`).
    pub(crate) fn theme_rows(&self) -> Vec<(ThemeChoice, String)> {
        std::iter::once((
            ThemeChoice::Default,
            format!("default ({})", self.default_preset),
        ))
        .chain(
            native_theme::theme::Theme::list_presets_for_platform()
                .into_iter()
                .map(|info| {
                    (
                        ThemeChoice::Preset(info.key.to_string()),
                        info.display_name.to_string(),
                    )
                }),
        )
        .collect()
    }

    /// The key of the preset the atlas is built from — `default_preset` for `default` — the key
    /// `mapping.toml`'s per-preset `exceptions` name (not the atlas's display name).
    pub(crate) fn preset_key(&self) -> &str {
        match &self.settings.theme {
            ThemeChoice::Default => &self.default_preset,
            ThemeChoice::Preset(key) => key,
        }
    }

    /// The status bar's title: the shown Widget Info's title, or nothing.
    pub(crate) fn status_title(&self) -> String {
        let Some(shown) = self.registry.shown() else {
            return String::new();
        };
        let element = shown.info.element.as_deref().and_then(|id| {
            self.elements
                .as_ref()
                .ok()
                .and_then(|all| all.iter().find(|e| e.id == id))
        });
        match element {
            Some(element) => match element.states.first() {
                Some(state) => format!("{} · {state}", element.name),
                None => element.name.clone(),
            },
            None => shown.info.kind.to_string(),
        }
    }

    /// The icon set and freedesktop theme the pages load from (§10.4's icon rule).
    pub(crate) fn chosen_icons(&self) -> (IconSet, Option<String>) {
        let icon = &self.settings.icon;
        (
            icon.effective_icon_set(self.atlas.icon_set()),
            icon.freedesktop_theme().map(str::to_string),
        )
    }

    #[cfg(feature = "watch")]
    fn publish_selection(&self) {
        let mut guard = self
            .selection
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        *guard = self.settings.clone();
    }

    /// One watcher for every selection, never dropped and restarted (§10.2, §10.4).
    #[cfg(feature = "watch")]
    fn start_watcher(&mut self, ctx: &egui::Context) {
        let selection = Arc::clone(&self.selection);
        let started = native_theme_egui::ThemeWatcher::start(ctx, move || {
            let settings = selection
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .clone();
            App::rebuild_atlas(&settings).map(|(atlas, _)| atlas)
        });
        match started {
            Ok(watcher) => self.watcher = Some(watcher),
            Err(error) => self.theme_error = Some(format!("theme watcher: {error}")),
        }
    }
}

impl App {
    /// Whether the system menu bar holds exactly the menus the showcase built:
    /// `Some(true)` or `Some(false)` on macOS outside `cfg(test)`.
    #[cfg(all(target_os = "macos", not(test)))]
    pub(crate) fn menu_installed(&self) -> Option<bool> {
        let built = self.system_menu.as_ref().map(|m| m.top_level())?;
        let installed = crate::chrome::system_menu::main_menu_item_count()?;
        Some(usize::try_from(installed).ok() == Some(built))
    }
    /// `None` wherever there is no system menu bar to read back.
    #[cfg(not(all(target_os = "macos", not(test))))]
    pub(crate) fn menu_installed(&self) -> Option<bool> {
        None
    }

    /// `--dump-layout`: once this pass's layout has been the same for `DUMP_SETTLE_S`, write it,
    /// then close the window unless it is being captured. A failed write is reported and ends
    /// the run with 1, as a failed `--screenshot` does.
    fn dump_layout(&mut self, ctx: &egui::Context) {
        let preset = self.preset_key().to_string();
        let Some(dump) = self.dump.as_mut().filter(|d| !d.written) else {
            return;
        };
        let now = ctx.input(|i| i.time);
        let places = self.registry.places();
        let since = match &dump.last {
            Some((last, since)) if last == places => *since,
            _ => {
                dump.last = Some((places.clone(), now));
                now
            }
        };
        ctx.request_repaint();
        if now - since < crate::DUMP_SETTLE_S {
            return;
        }
        let json = layout_json(places, &preset, ctx.theme(), ctx.pixels_per_point());
        let written = serde_json::to_string_pretty(&json)
            .map_err(|e| e.to_string())
            .and_then(|text| std::fs::write(&dump.path, text).map_err(|e| e.to_string()));
        if let Err(error) = written {
            eprintln!("ERROR: --dump-layout {}: {error}", dump.path);
            std::process::exit(1);
        }
        eprintln!("Layout dumped to {}", dump.path);
        dump.written = true;
        if !dump.keep_running {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    pub(crate) fn run_action(&mut self, action: Action, ctx: &egui::Context) {
        match action {
            Action::ShowPage(page) => {
                self.settings.page = page;
                self.registry.screen_changed();
            }
            Action::ToggleSidePanel => self.side_panel_visible = !self.side_panel_visible,
            Action::OpenCommandPalette => self.palette = Some(PaletteState::default()),
            Action::ReloadTheme => self.install(ctx),
            Action::SetMode(mode) => self.set_mode(mode, ctx),
            Action::OpenPreferences => self.preferences_open = true,
            Action::OpenAbout => self.about_open = true,
            Action::Quit => {
                self.quit_requested = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        ctx.request_repaint();
    }
}

impl eframe::App for App {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        let Some(held) = &mut self.held_pointer else {
            // A captured window shows the page at rest wherever the real pointer is: no control
            // hovered or pressed under it, no Widget Info chosen by it, nothing scrolled by it.
            if self.capturing {
                raw_input
                    .events
                    .retain(|e| !pointer_event(e) && !matches!(e, egui::Event::MouseWheel { .. }));
            }
            return;
        };
        // The held pointer replaces the window's own: a real pointer entering or leaving the
        // window would move it away and back, which egui reads as a drag that cancels a press.
        raw_input.events.retain(|e| !pointer_event(e));
        // Moved there once: egui keeps the pointer where it last moved, and a move every pass
        // would keep it from ever being still, which a tooltip waits for.
        if !held.moved {
            raw_input.events.push(egui::Event::PointerMoved(held.at));
            held.moved = true;
        }
        // The press lands once the page has been laid out under the pointer: a press before it
        // hits no widget, and egui then highlights none while the button is down.
        let (down, time) = ctx.input(|i| (i.pointer.primary_down(), i.time));
        if held.press && (!held.pressed || !down) && time > HeldPointer::PRESS_AFTER {
            // egui stops treating a press held longer than `max_click_duration` as a click, and
            // stops drawing the control pressed (`InputState::could_any_button_be_click`,
            // `egui/src/interaction.rs`), so a held press stays one until the capture.
            ctx.options_mut(|o| o.input_options.max_click_duration = f64::INFINITY);
            raw_input.events.push(egui::Event::PointerButton {
                pos: held.at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            });
            held.pressed = true;
        }
        // A widget's state is read from the pass before (`Context::read_response`), so the
        // passes go on until the capture is taken.
        ctx.request_repaint_after(crate::INFO_SETTLE);
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // The system menu's clicks, as the in-window items' (§10.4).
        #[cfg(all(target_os = "macos", not(test)))]
        if let Some(rx) = &self.menu_rx {
            self.pending.extend(rx.try_iter());
        }
        // Before anything reads input (§10.4: the palette's Ctrl+K beats a focused field's own
        // Ctrl+K); compiled out on macOS outside `cfg(test)`, where the system menu's
        // accelerators own the keys, so one route owns each key.
        #[cfg(all(target_os = "macos", not(test)))]
        let shortcuts: Vec<Action> = Vec::new();
        #[cfg(not(all(target_os = "macos", not(test))))]
        let shortcuts: Vec<Action> = ctx.input_mut(|i| {
            Action::MENUS
                .iter()
                .flat_map(|(_, items)| items.iter().flatten())
                .filter(|a| a.shortcut().is_some_and(|s| i.consume_shortcut(&s)))
                .copied()
                .collect()
        });
        for action in shortcuts
            .into_iter()
            .chain(std::mem::take(&mut self.pending))
        {
            self.run_action(action, ctx);
        }
        #[cfg(feature = "watch")]
        if let Some(atlas) = self.watcher.as_ref().and_then(|w| w.take()) {
            atlas.install(ctx);
            self.semibold.register(ctx, &atlas);
            self.atlas = atlas;
            self.json = info::JsonCache::default();
            self.last_scheme = None;
            self.registry.screen_changed();
        }
        // A new theme or a scheme switch re-derives a following icon choice (§10.4:
        // `breeze` to `breeze-dark` on `kde-breeze`); a pass on the same scheme does nothing.
        let scheme = ctx.theme();
        if self.last_scheme != Some(scheme) {
            self.settings.theme_installed(&self.atlas, scheme);
            self.last_scheme = Some(scheme);
        }
        // Read before the screenshot's `&mut` borrow of `self` (§10.4's fourth rule).
        let menu_installed = self.menu_installed();
        if let Some(shot) = &mut self.screenshot {
            let now = ctx.input(|i| i.time);
            let started = *shot.started.get_or_insert(now);
            if !shot.written && shot.capture.is_none() && now - started >= SCREENSHOT_DELAY_S {
                // The macOS runner's check (§13): a capture step fails when the menu did not install.
                if menu_installed == Some(false) {
                    eprintln!(
                        "ERROR: the macOS main menu holds a different number of menus than the showcase built"
                    );
                    std::process::exit(1);
                }
                match crate::capture::FrameCapture::start(&shot.path) {
                    Ok(capture) => shot.capture = Some(capture),
                    Err(error) => {
                        eprintln!(
                            "ERROR: screenshot capture failed for {}: {error}",
                            shot.path
                        );
                        std::process::exit(1);
                    }
                }
            }
            match shot.capture.as_mut().and_then(|c| c.poll(ctx)) {
                None => {}
                Some(Ok(())) => {
                    eprintln!("Screenshot saved to {}", shot.path);
                    shot.capture = None;
                    shot.written = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                Some(Err(error)) => {
                    eprintln!(
                        "ERROR: screenshot capture failed for {}: {error}",
                        shot.path
                    );
                    std::process::exit(1);
                }
            }
            // egui repaints only on demand (`egui/src/memory/mod.rs:341`): keep the passes
            // coming until the capture, on a page that does not animate as on one that does.
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.registry.begin_pass();
        self.registry.place(ui, "chrome.window", ui.max_rect());
        chrome::chrome_bar(self, ui);
        chrome::status_bar(self, ui);
        chrome::side_panel(self, ui);
        ui.reset_style();
        chrome::central_panel(self, ui, |app, ui| {
            pages::show_fixed(app, ui);
            let _page = egui::ScrollArea::vertical().show(ui, |ui| pages::show(app, ui));
            #[cfg(test)]
            {
                app.page_scrolls = _page.content_size.y > _page.inner_rect.height();
            }
        });
        chrome::command_palette(self, ui);
        chrome::preferences(self, ui);
        chrome::about(self, ui);
        let pending = std::mem::take(&mut self.pending);
        let ctx = ui.ctx().clone();
        for action in pending {
            self.run_action(action, &ctx);
        }
        self.registry.end_pass(&ctx, self.hold_zone);
        self.dump_layout(&ctx);
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        visuals.panel_fill.to_normalized_gamma_f32()
    }
}
