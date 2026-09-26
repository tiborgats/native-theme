//! The application: its settings, its atlas, and the `eframe::App` impl (spec §10.4).

use std::sync::Arc;

use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme::{AccessibilityPreferences, SystemTheme, theme::IconSet};
use native_theme_egui::{SystemThemeExt as _, ThemeAtlas, from_preset};

use crate::chrome::{self, Action, InspectorTab, PaletteState};
use crate::{CliArgs, SCREENSHOT_DELAY_S, apply_cli_args, demo, pages};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Page {
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
}

impl Page {
    pub(crate) const ALL: [Page; 10] = [
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
    ];
    /// The `--tab` name (§10.4's palette table).
    pub(crate) fn key(self) -> &'static str {
        match self {
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
        }
    }
    pub(crate) fn label(self) -> &'static str {
        match self {
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
            page: Page::Buttons,
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

/// `--screenshot`: where the capture goes, when the first pass ran, whether the command went out.
struct Screenshot {
    path: String,
    started: Option<f64>,
    sent: bool,
}

pub(crate) struct App {
    /// The one field for theme state (§10.4's third rule).
    pub(crate) atlas: ThemeAtlas,
    pub(crate) settings: Settings,
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
    /// The scheme the icon choice was last derived for; `None` forces a re-derive next pass.
    last_scheme: Option<egui::Theme>,
    screenshot: Option<Screenshot>,
    /// What the watcher's `rebuild` reads: the UI thread writes it on each install.
    #[cfg(feature = "watch")]
    selection: Arc<std::sync::RwLock<Settings>>,
    #[cfg(feature = "watch")]
    watcher: Option<native_theme_egui::ThemeWatcher>,
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
        let (atlas, theme_error) = match Self::rebuild_atlas(&settings) {
            Ok(atlas) => (atlas, None),
            Err(error) => {
                let fallback = native_theme::pipeline::platform_preset_name().name;
                let (atlas, _) =
                    from_preset(fallback, false, &AccessibilityPreferences::from_system())?;
                (atlas, Some(error.to_string()))
            }
        };
        let screenshot = settings.screenshot.clone().map(|path| Screenshot {
            path,
            started: None,
            sent: false,
        });
        #[cfg(feature = "watch")]
        let selection = Arc::new(std::sync::RwLock::new(settings.clone()));
        let mut app = Self {
            atlas,
            settings,
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
            last_scheme: None,
            screenshot,
            #[cfg(feature = "watch")]
            selection,
            #[cfg(feature = "watch")]
            watcher: None,
        };
        app.apply(&cc.egui_ctx);
        #[cfg(feature = "watch")]
        if !cfg!(test) && app.settings.screenshot.is_none() {
            app.start_watcher(&cc.egui_ctx);
        }
        Ok(app)
    }

    /// The atlas `settings` select (§10.4): `default` is `SystemTheme::from_system` through
    /// `to_egui_atlas`, with Preferences' overrides on its public `accessibility`
    /// (`native-theme/src/lib.rs:490`) after `invalidate_caches()`; a preset is `from_preset`,
    /// whose `is_dark` selects only the `ResolvedTheme` this discards — the atlas carries both
    /// variants and egui picks the scheme (§4.6) — so `false` is passed.
    pub(crate) fn rebuild_atlas(settings: &Settings) -> native_theme::Result<ThemeAtlas> {
        match &settings.theme {
            ThemeChoice::Default => {
                native_theme::detect::invalidate_caches();
                let mut sys = SystemTheme::from_system()?;
                if let Some(overrides) = &settings.prefs {
                    sys.accessibility = overrides.clone();
                }
                Ok(sys.to_egui_atlas())
            }
            ThemeChoice::Preset(name) => {
                let prefs = settings
                    .prefs
                    .clone()
                    .unwrap_or_else(AccessibilityPreferences::from_system);
                from_preset(name, false, &prefs).map(|(atlas, _)| atlas)
            }
        }
    }

    /// Rebuild what `settings` select and install it; a failure keeps the installed atlas and
    /// is shown on the page (§10.4, the theme error in the content).
    pub(crate) fn install(&mut self, ctx: &egui::Context) {
        match Self::rebuild_atlas(&self.settings) {
            Ok(atlas) => {
                self.atlas = atlas;
                self.theme_error = None;
            }
            Err(error) => self.theme_error = Some(error.to_string()),
        }
        self.apply(ctx);
    }

    /// Install the held atlas and the mode's theme preference (egui's own call, §10.3).
    fn apply(&mut self, ctx: &egui::Context) {
        self.atlas.install(ctx);
        ctx.set_theme(match self.settings.mode {
            ModeChoice::System => egui::ThemePreference::System,
            ModeChoice::Light => egui::ThemePreference::Light,
            ModeChoice::Dark => egui::ThemePreference::Dark,
        });
        self.last_scheme = None;
        #[cfg(feature = "watch")]
        self.publish_selection();
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
            App::rebuild_atlas(&settings)
        });
        match started {
            Ok(watcher) => self.watcher = Some(watcher),
            Err(error) => self.theme_error = Some(format!("theme watcher: {error}")),
        }
    }
}

impl App {
    pub(crate) fn run_action(&mut self, action: Action, ctx: &egui::Context) {
        match action {
            Action::ShowPage(page) => self.settings.page = page,
            Action::ToggleSidePanel => self.side_panel_visible = !self.side_panel_visible,
            Action::OpenCommandPalette => self.palette = Some(PaletteState::default()),
            Action::ReloadTheme => self.install(ctx),
            Action::SetMode(mode) => {
                self.settings.mode = mode;
                self.install(ctx);
            }
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
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Before anything reads input (§10.4: the palette's Ctrl+K beats a focused field's own
        // Ctrl+K). Task 35 compiles this block out on macOS outside `cfg(test)`, where the system
        // menu owns the keys.
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
            self.atlas = atlas;
            self.last_scheme = None;
        }
        // A new theme or a scheme switch re-derives a following icon choice (§10.4:
        // `breeze` to `breeze-dark` on `kde-breeze`); a pass on the same scheme does nothing.
        let scheme = ctx.theme();
        if self.last_scheme != Some(scheme) {
            self.settings.theme_installed(&self.atlas, scheme);
            self.last_scheme = Some(scheme);
        }
        if let Some(shot) = &mut self.screenshot {
            let now = ctx.input(|i| i.time);
            let started = *shot.started.get_or_insert(now);
            if !shot.sent && now - started >= SCREENSHOT_DELAY_S {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
                shot.sent = true;
            }
            let image = ctx.input(|i| {
                i.events.iter().find_map(|e| match e {
                    egui::Event::Screenshot { image, .. } => Some(Arc::clone(image)),
                    _ => None,
                })
            });
            if let Some(image) = image {
                match write_png(&image, &shot.path) {
                    Ok(()) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                    Err(error) => {
                        eprintln!(
                            "ERROR: screenshot capture failed for {}: {error}",
                            shot.path
                        );
                        std::process::exit(1);
                    }
                }
            }
            // egui repaints only on demand (`egui/src/memory/mod.rs:341`): keep the passes
            // coming until the capture, on a page that does not animate as on one that does.
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.registry.begin_pass();
        chrome::chrome_bar(self, ui);
        chrome::status_bar(self, ui);
        chrome::side_panel(self, ui);
        ui.reset_style();
        chrome::central_panel(self, ui, |app, ui| {
            egui::ScrollArea::vertical().show(ui, |ui| pages::show(app, ui));
        });
        chrome::command_palette(self, ui);
        chrome::preferences(self, ui);
        chrome::about(self, ui);
        let pending = std::mem::take(&mut self.pending);
        let ctx = ui.ctx().clone();
        for action in pending {
            self.run_action(action, &ctx);
        }
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        visuals.panel_fill.to_normalized_gamma_f32()
    }
}

/// The captured frame as a PNG through the `image` dev-dependency (§11);
/// `ColorImage::as_raw` is RGBA, `epaint/src/image.rs:177`.
fn write_png(image: &egui::ColorImage, path: &str) -> Result<(), String> {
    let (w, h) = (image.width() as u32, image.height() as u32);
    let rgba = image::RgbaImage::from_raw(w, h, image.as_raw().to_vec())
        .ok_or_else(|| "the image's buffer does not match its size".to_string())?;
    rgba.save(path).map_err(|e| e.to_string())
}
