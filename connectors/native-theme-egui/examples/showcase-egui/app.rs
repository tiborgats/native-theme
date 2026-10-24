//! The application: its settings, its atlas, and the `eframe::App` impl (spec §10.4).

#[cfg(feature = "watch")]
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme::{AccessibilityPreferences, SystemTheme, theme::IconSet};
use native_theme_egui::{SystemThemeExt as _, ThemeAtlas, from_preset};
use native_theme_showcase_support::capture::{Content, FrameCapture};
use native_theme_showcase_support::cli::{self, Output, Outputs};
use native_theme_showcase_support::{CAPTURE_WAIT, Kind, dump, rows};

use crate::chrome::{self, Action, InspectorTab, PaletteState};
use crate::{CliArgs, SCREENSHOT_DELAY_S, apply_cli_args, demo, elements, info, pages};

/// The colour mode the Mode row, the Theme menu and `--variant` choose (`cli::color_mode`), and
/// the theme the Theme row and `--theme` choose (`cli::theme_key`): the shared crate's, which
/// the gpui and iced showcases choose from too.
pub(crate) use native_theme_showcase_support::cli::{
    ColorModeChoice as ModeChoice, ThemeKey as ThemeChoice,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Page {
    Basic,
    Buttons,
    Inputs,
    Selection,
    Range,
    Data,
    Text,
    Containers,
    Overlays,
    Colour,
    Icons,
    ThemeMap,
}

impl Page {
    pub(crate) const ALL: [Page; 12] = [
        Page::Basic,
        Page::Buttons,
        Page::Inputs,
        Page::Selection,
        Page::Range,
        Page::Data,
        Page::Text,
        Page::Containers,
        Page::Overlays,
        Page::Colour,
        Page::Icons,
        Page::ThemeMap,
    ];
    /// The `--tab` name (§10.4's palette table): the page's label in lower case with each space
    /// a `-`, the key the three showcases give the pages that correspond (the counterpart table
    /// in `docs/showcase-elements.toml`'s header).
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
    /// The page `--tab` names (`cli::page_key`, whose error lists every page's key).
    pub(crate) fn from_key(key: &str) -> Result<Page, String> {
        let keys = Page::ALL.map(Page::key);
        let key = cli::page_key(key, &keys)?;
        Page::ALL
            .into_iter()
            .find(|page| page.key() == key)
            .ok_or_else(|| format!("--tab {key}: no such page"))
    }
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
    /// Preferences' overrides, the user's requests; `None` is the OS's own. An install builds
    /// with their [`effective_preferences`] over the OS's.
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

/// What the watcher's `rebuild` reads: the settings the UI thread last installed or tried to
/// install, so a hand-off it builds is a rebuild of the choice the Theme row names, and their
/// generation, which each publish bumps. A mode switch publishes nothing, since the rebuild
/// reads only `theme` and `prefs` (`App::rebuild_atlas`), so `settings.mode` here can be stale.
#[cfg(feature = "watch")]
#[derive(Debug)]
pub(crate) struct Selection {
    pub(crate) generation: u64,
    pub(crate) settings: Settings,
}

/// The watcher's rebuilds, as its `rebuild` records them under one lock, before the connector's
/// hand-off stores the atlas it returns (`Handoff::on_change`, `src/watch.rs`), so the atlas
/// `ThemeWatcher::take` signals is recorded by then: how many have returned, `Ok` or `Err`, and
/// the last that returned an atlas, as its number, the generation of the selection it was built
/// for, and the atlas.
#[cfg(feature = "watch")]
#[derive(Debug, Default)]
pub(crate) struct Rebuilt {
    pub(crate) rebuilds: u64,
    pub(crate) built: Option<(u64, u64, ThemeAtlas)>,
}

/// `--screenshot`: where the capture goes, when the first pass ran, the capture while under
/// way (the shared crate's `FrameCapture`), whether it was written.
struct Screenshot {
    path: String,
    started: Option<f64>,
    capture: Option<FrameCapture>,
    written: bool,
}

/// `--dump-layout`: where the dump goes, the input time of its first pass, the layout of the last
/// pass and the input time it was first seen at, and whether it was written.
struct LayoutDump {
    path: String,
    started: Option<f64>,
    last: Option<(std::collections::BTreeMap<String, egui::Rect>, f64)>,
    written: bool,
}

/// The layout dump's JSON (`dump::layout_json`, the object `docs/showcase-elements.toml`'s
/// "The layout dump" describes): each placed element's rectangle in logical pixels,
/// window-content coordinates.
pub(crate) fn layout_json(
    places: &std::collections::BTreeMap<String, egui::Rect>,
    preset: &str,
    theme: egui::Theme,
    scale: f32,
) -> serde_json::Value {
    let variant = match theme {
        egui::Theme::Light => "light",
        egui::Theme::Dark => "dark",
    };
    let rects = places.iter().map(|(id, r)| {
        let rect = dump::Rect {
            x: r.min.x,
            y: r.min.y,
            w: r.width(),
            h: r.height(),
        };
        (id.clone(), rect)
    });
    dump::layout_json(Kind::Egui, preset, variant, scale, rects)
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
    /// four seconds after starting it (`SHOWCASE_START_DELAY=3`, then `capture_showcase`'s
    /// `sleep 1`, in `scripts/capture_window.sh`), so a press three seconds
    /// after the first frame lands about a second before such a capture; a capture with
    /// `--press` should wait longer.
    const PRESS_AFTER: f64 = 3.0;
}

/// Whether `event` is the keyboard's: a key, typed or composed text, or the clipboard keys'
/// cut, copy and paste.
fn keyboard_event(event: &egui::Event) -> bool {
    matches!(
        event,
        egui::Event::Key { .. }
            | egui::Event::Text(_)
            | egui::Event::Ime(_)
            | egui::Event::Paste(_)
            | egui::Event::Copy
            | egui::Event::Cut
    )
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

/// The preferences an install builds with, from the OS's reading `os` and Preferences'
/// `overrides` (`None` until the user changes one): every preference the override's where one
/// is set, except Reduce motion, whose switch off is no request, so an OS request still stops
/// the animations (the Step 4 ruling: the gpui showcase's switch through native-theme-gpui's
/// `forward_reduce_motion`, spec v0.5.9 §5, and the iced showcase's `motion_reduced`). High
/// contrast and Reduce transparency keep the override's value, since the atlas builds no
/// differently for them.
pub(crate) fn effective_preferences(
    os: &AccessibilityPreferences,
    overrides: Option<&AccessibilityPreferences>,
) -> AccessibilityPreferences {
    match overrides {
        Some(overrides) => {
            let mut prefs = overrides.clone();
            prefs.reduce_motion |= os.reduce_motion;
            prefs
        }
        None => os.clone(),
    }
}

pub(crate) struct App {
    /// The one field for theme state (§10.4's third rule).
    pub(crate) atlas: ThemeAtlas,
    pub(crate) settings: Settings,
    /// The platform preset `default` builds on, `SystemTheme::preset` since the last read of
    /// the OS: the name the Theme row, the command palette and the status bar give `default`,
    /// "default (kde-breeze)", as the gpui showcase's `default_label`
    /// (`showcase-gpui/app.rs:1808-1812`).
    pub(crate) default_preset: String,
    /// The OS's semibold face of the theme's family, registered after each install.
    pub(crate) semibold: demo::Semibold,
    pub(crate) theme_error: Option<String>,
    pub(crate) registry: demo::Registry,
    pub(crate) demo_state: pages::DemoState,
    /// The icons the showcase loads itself, kept until the next install or icon choice.
    pub(crate) icon_cache: demo::IconCache,
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
    pub(crate) elements: Result<Vec<elements::ShowcaseElement>, String>,
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
    /// The files the run writes, and whether it quits once they are written (`cli::Outputs`, the
    /// three showcases' exit rule).
    outputs: Outputs,
    /// `--pointer` and `--press`: the pointer held at a point, and the primary button held down.
    held_pointer: Option<HeldPointer>,
    /// `--capture`, `--screenshot` or `--dump-layout`: the window is captured or its layout
    /// dumped, so the real pointer draws nothing (`raw_input_hook`, `CliArgs::shields_pointer`).
    shields_pointer: bool,
    /// `--capture`, `--screenshot` or `--dump-layout`: the window takes no keyboard input, so a
    /// key typed while it is captured or dumped changes nothing it shows (`raw_input_hook`,
    /// `CliArgs::shields_keyboard`).
    shields_keyboard: bool,
    /// `--open-menu <title>`: the menu bar's menu of that title (lower case) still to be opened;
    /// the menu bar opens it at its first pass and clears this. On macOS the menus are the
    /// system menu bar's, which the flag does not reach.
    #[cfg_attr(all(target_os = "macos", not(test)), allow(dead_code))]
    pub(crate) open_menu: Option<String>,
    /// What the watcher's `rebuild` reads: the UI thread publishes it on each install, failed
    /// or not.
    #[cfg(feature = "watch")]
    pub(crate) selection: Arc<RwLock<Selection>>,
    /// What the watcher's `rebuild` has returned.
    #[cfg(feature = "watch")]
    pub(crate) rebuilt: Arc<Mutex<Rebuilt>>,
    /// The watcher's rebuilds an install has covered: the record's `rebuilds` when an `install`
    /// read it, or the number of the rebuilt atlas the hand-off installed.
    #[cfg(feature = "watch")]
    installed_rebuild: u64,
    #[cfg(feature = "watch")]
    watcher: Option<native_theme_egui::ThemeWatcher>,
    /// Why the watcher did not start: shown as a note, which no install clears, since the OS's
    /// changes stay unfollowed whatever is installed.
    #[cfg(feature = "watch")]
    pub(crate) watcher_error: Option<String>,
    /// The tests' stand-in for the running watcher's `ThemeWatcher::last_error`: the tests start
    /// no watcher. Unlike the watcher's, it is never reset by a later rebuild, so the tests prove
    /// the record's count gates the banner's line.
    #[cfg(all(test, feature = "watch"))]
    pub(crate) watcher_last_error: Option<String>,
    /// The macOS system menu bar, owned here: a muda `Menu` holds `Rc`s and is not `Send`.
    #[cfg(all(target_os = "macos", not(test)))]
    system_menu: Option<crate::chrome::system_menu::SystemMenu>,
    /// The system menu's clicks, drained into `pending` by `logic`.
    #[cfg(all(target_os = "macos", not(test)))]
    menu_rx: Option<std::sync::mpsc::Receiver<Action>>,
    /// The system icon theme's detection, `native_theme::detect::system_icon_theme`, which the
    /// Icon theme drop-down's hover text and the Icons page's label read; a test replaces it
    /// with one that fails or names a theme, as it cannot change the desktop's.
    pub(crate) detect_icon_theme: fn() -> native_theme::Result<String>,
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
        let selection = Arc::new(RwLock::new(Selection {
            generation: 0,
            settings: settings.clone(),
        }));
        let mut app = Self {
            atlas: atlas.clone(),
            settings,
            default_preset,
            semibold: demo::Semibold::default(),
            theme_error: None,
            registry: demo::Registry::default(),
            demo_state: pages::DemoState::default(),
            icon_cache: demo::IconCache::default(),
            side_panel_visible: true,
            palette: None,
            preferences_open: false,
            about_open: false,
            inspector_tab: InspectorTab::Widget,
            quit_requested: false,
            pending: Vec::new(),
            manifest: info::Manifest::parse(include_str!("../../mapping.toml")),
            elements: elements::showcase_elements(),
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
                started: None,
                last: None,
                written: false,
            }),
            outputs: Outputs::new(cli),
            held_pointer: cli.pointer.map(|(x, y)| HeldPointer {
                at: egui::pos2(f32::from(x), f32::from(y)),
                press: cli.press,
                moved: false,
                pressed: false,
            }),
            shields_pointer: cli.shields_pointer(),
            shields_keyboard: cli.shields_keyboard(),
            // On macOS outside `cfg(test)` the menus are the system menu bar's, which no flag
            // holds open, so every name is reported and none kept.
            open_menu: cli.open_menu.as_deref().and_then(|menu| {
                let titles: Vec<&str> = crate::chrome::Action::MENUS
                    .iter()
                    .map(|(title, _)| *title)
                    .collect();
                cli::reported(cli::open_menu(
                    menu,
                    &titles,
                    cfg!(all(target_os = "macos", not(test))),
                ))
            }),
            #[cfg(feature = "watch")]
            selection,
            #[cfg(feature = "watch")]
            rebuilt: Arc::default(),
            #[cfg(feature = "watch")]
            installed_rebuild: 0,
            #[cfg(feature = "watch")]
            watcher: None,
            #[cfg(feature = "watch")]
            watcher_error: None,
            #[cfg(all(test, feature = "watch"))]
            watcher_last_error: None,
            #[cfg(all(target_os = "macos", not(test)))]
            system_menu: None,
            #[cfg(all(target_os = "macos", not(test)))]
            menu_rx: None,
            detect_icon_theme: native_theme::detect::system_icon_theme,
        };
        app.apply(&cc.egui_ctx, atlas);
        // After the install, which clears it: the platform's preset stands in for the selection.
        app.theme_error = theme_error;
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
        if !cfg!(test) && cli.follows_os() {
            app.start_watcher(&cc.egui_ctx);
        }
        Ok(app)
    }

    /// The atlas `settings` select (§10.4): `default` is `SystemTheme::from_system` through
    /// `to_egui_atlas` after `invalidate_caches()`, its public `accessibility`
    /// (`native-theme/src/lib.rs:500`) the [`effective_preferences`] of the OS's reading it
    /// holds and Preferences' overrides; a preset is `from_preset` with the effective
    /// preferences of the cached OS reading (`detect::accessibility_preferences`, which the
    /// watcher and `ReloadTheme` clear before they rebuild) and the overrides. `from_preset`'s
    /// `is_dark` selects only the `ResolvedTheme` this discards — the atlas carries both
    /// variants and egui picks the scheme (§4.6) — so `false` is passed. With the atlas comes,
    /// for `default`, the platform preset it builds on (`SystemTheme::preset`). It reads
    /// `theme` and `prefs` alone, not `mode`, which the watcher's selection relies on: a mode
    /// switch publishes nothing to it.
    pub(crate) fn rebuild_atlas(
        settings: &Settings,
    ) -> native_theme::Result<(ThemeAtlas, Option<String>)> {
        match &settings.theme {
            ThemeChoice::Default => {
                native_theme::detect::invalidate_caches();
                let mut sys = SystemTheme::from_system()?;
                sys.accessibility =
                    effective_preferences(&sys.accessibility, settings.prefs.as_ref());
                let preset = sys.preset.clone();
                Ok((sys.to_egui_atlas(), Some(preset)))
            }
            ThemeChoice::Preset(name) => {
                let prefs = effective_preferences(
                    &native_theme::detect::accessibility_preferences(),
                    settings.prefs.as_ref(),
                );
                from_preset(name, false, &prefs).map(|(atlas, _)| (atlas, None))
            }
        }
    }

    /// Rebuild what `settings` select and install it; a failure keeps the installed atlas and
    /// is shown on the page (§10.4, the theme error in the content). A success covers the
    /// watcher's rebuilds returned before it read the record: a rebuild still running counts as
    /// newer, since it may carry an OS change this rebuild did not read.
    pub(crate) fn install(&mut self, ctx: &egui::Context) {
        #[cfg(feature = "watch")]
        let rebuilds = self.rebuilds();
        match Self::rebuild_atlas(&self.settings) {
            Ok((atlas, preset)) => {
                if let Some(preset) = preset {
                    self.default_preset = preset;
                }
                self.apply(ctx, atlas);
                #[cfg(feature = "watch")]
                {
                    self.installed_rebuild = rebuilds;
                }
            }
            Err(error) => {
                // The installed atlas, its fonts and the semibold faces stay as they are; the
                // failed choice is published, so a rebuild of the theme still drawn is stale
                // and keeps this error.
                self.theme_error = Some(error.to_string());
                #[cfg(feature = "watch")]
                self.publish_selection();
            }
        }
    }

    /// Install `atlas` and the mode's theme preference (egui's own call, §10.3).
    fn apply(&mut self, ctx: &egui::Context, atlas: ThemeAtlas) {
        self.adopt(ctx, atlas);
        ctx.set_theme(self.theme_preference());
        #[cfg(feature = "watch")]
        self.publish_selection();
    }

    /// The steps every install of an atlas takes, and nothing else: `atlas` installed into
    /// `ctx` and held, the OS's semibold faces of its family registered, the JSON cache, the
    /// icons the showcase keeps (the install forgets the connector's icon URIs they name) and
    /// the scheme the icon choice was derived for forgotten, the shown Widget Info released, and
    /// the theme error cleared, since a theme now loaded. `apply` runs it, so does the watcher's
    /// hand-off, which leaves the mode as it is, and so do the tests that install an atlas of
    /// their own; it sets no theme preference, so a test that holds a scheme keeps it.
    pub(crate) fn adopt(&mut self, ctx: &egui::Context, atlas: ThemeAtlas) {
        atlas.install(ctx);
        self.semibold.register(ctx, &atlas);
        self.atlas = atlas;
        self.json = info::JsonCache::default();
        self.icon_cache.clear();
        self.last_scheme = None;
        self.registry.screen_changed();
        self.theme_error = None;
    }

    /// Switch the mode: egui's own theme call and nothing else (§10.4's side-panel row). The
    /// atlas carries both schemes and egui picks one (§4.6), so nothing is rebuilt, the OS is
    /// not read again, and a mode switch cannot fail. Nothing is published to the watcher, whose
    /// rebuild reads no mode, so a rebuild under way stays current.
    pub(crate) fn set_mode(&mut self, mode: ModeChoice, ctx: &egui::Context) {
        self.settings.mode = mode;
        ctx.set_theme(self.theme_preference());
    }

    fn theme_preference(&self) -> egui::ThemePreference {
        match self.settings.mode {
            ModeChoice::System => egui::ThemePreference::System,
            ModeChoice::Light => egui::ThemePreference::Light,
            ModeChoice::Dark => egui::ThemePreference::Dark,
        }
    }

    /// What the Theme row and the command palette offer, as `(choice, display name)`: the
    /// shared crate's preset rows (`rows::preset_rows`), `default` named by the preset it builds
    /// on, then this platform's presets by their display names, as the gpui and iced showcases
    /// offer them.
    pub(crate) fn theme_rows(&self) -> Vec<(ThemeChoice, String)> {
        rows::preset_rows(&self.default_preset)
            .into_iter()
            .filter_map(|row| Some((cli::theme_key(&row.key).ok()?, row.display_name)))
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
            Some(element) => element.title(),
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

    /// The icon choice was `before`: where it changed, the icons the showcase keeps are of a
    /// set or icon theme no longer drawn, so they are forgotten; where it stayed, as at a
    /// scheme switch under a chosen theme, they are kept, the other scheme's included.
    pub(crate) fn icons_chosen(&mut self, before: &IconSetChoice) {
        if self.settings.icon != *before {
            self.icon_cache.clear();
        }
    }

    #[cfg(feature = "watch")]
    fn publish_selection(&self) {
        let mut guard = self
            .selection
            .write()
            .unwrap_or_else(PoisonError::into_inner);
        guard.generation = guard.generation.wrapping_add(1);
        guard.settings = self.settings.clone();
    }

    /// The generation of the selection last published.
    #[cfg(feature = "watch")]
    pub(crate) fn generation(&self) -> u64 {
        self.selection
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .generation
    }

    /// How many of the watcher's rebuilds have returned.
    #[cfg(feature = "watch")]
    fn rebuilds(&self) -> u64 {
        self.rebuilt
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .rebuilds
    }

    /// One watcher for every selection, never dropped and restarted (§10.2, §10.4).
    #[cfg(feature = "watch")]
    fn start_watcher(&mut self, ctx: &egui::Context) {
        let (selection, rebuilt) = (Arc::clone(&self.selection), Arc::clone(&self.rebuilt));
        let started = native_theme_egui::ThemeWatcher::start(ctx, move || {
            App::rebuild_selected(&selection, &rebuilt)
        });
        self.watcher_started(started);
    }

    /// The watcher held, or why it did not start kept for the note.
    #[cfg(feature = "watch")]
    pub(crate) fn watcher_started(
        &mut self,
        started: native_theme::Result<native_theme_egui::ThemeWatcher>,
    ) {
        match started {
            Ok(watcher) => self.watcher = Some(watcher),
            Err(error) => self.watcher_error = Some(error.to_string()),
        }
    }

    /// The watcher's `rebuild`, on the watcher thread: the selection read with its generation,
    /// the detection caches cleared, as `ThemeWatcher::system_rebuild` does, so a preset's
    /// rebuild reads the OS accessibility preferences again, then the atlas it selects built and
    /// recorded.
    #[cfg(feature = "watch")]
    pub(crate) fn rebuild_selected(
        selection: &RwLock<Selection>,
        rebuilt: &Mutex<Rebuilt>,
    ) -> native_theme::Result<ThemeAtlas> {
        let (generation, settings) = {
            let guard = selection.read().unwrap_or_else(PoisonError::into_inner);
            (guard.generation, guard.settings.clone())
        };
        native_theme::detect::invalidate_caches();
        let result = App::rebuild_atlas(&settings).map(|(atlas, _)| atlas);
        App::record_rebuild(rebuilt, generation, result)
    }

    /// A rebuild for the selection of `generation` returned `result`: counted, and an atlas
    /// recorded as the last built, before the connector's hand-off holds it.
    #[cfg(feature = "watch")]
    pub(crate) fn record_rebuild(
        rebuilt: &Mutex<Rebuilt>,
        generation: u64,
        result: native_theme::Result<ThemeAtlas>,
    ) -> native_theme::Result<ThemeAtlas> {
        let mut record = rebuilt.lock().unwrap_or_else(PoisonError::into_inner);
        record.rebuilds = record.rebuilds.saturating_add(1);
        if let Ok(atlas) = &result {
            record.built = Some((record.rebuilds, generation, atlas.clone()));
        }
        result
    }

    /// The watcher's hand-off, which `ThemeWatcher::take` signals; the atlas is read from the
    /// record, with the generation it was built for. A rebuild no install has covered is
    /// installed where it was built for the current selection; built for an earlier one, it is
    /// dropped and the current selection rebuilt, which also reads the OS change it announced
    /// (M9-115). A rebuild an install has covered installs nothing.
    #[cfg(feature = "watch")]
    pub(crate) fn take_rebuilt(&mut self, ctx: &egui::Context) {
        let built = self
            .rebuilt
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .built
            .clone();
        let Some((number, generation, atlas)) =
            built.filter(|(number, ..)| *number > self.installed_rebuild)
        else {
            return;
        };
        if generation == self.generation() {
            self.adopt(ctx, atlas);
            self.installed_rebuild = number;
        } else {
            self.install(ctx);
        }
    }

    /// The watcher's last rebuild error, `None` again once a later rebuild succeeds.
    #[cfg(feature = "watch")]
    fn watcher_last_error(&self) -> Option<String> {
        #[cfg(test)]
        if let Some(error) = &self.watcher_last_error {
            return Some(error.clone());
        }
        self.watcher
            .as_ref()
            .and_then(native_theme_egui::ThemeWatcher::last_error)
    }

    /// The theme errors the banner shows: the last install's, then the watcher's last failed
    /// rebuild as `theme watcher: …` while no install has covered it. The record's count decides,
    /// not the error's text, which a repeat failure keeps.
    pub(crate) fn theme_errors(&self) -> Vec<String> {
        let errors = self.theme_error.iter().cloned();
        #[cfg(feature = "watch")]
        let errors = errors.chain(
            self.watcher_last_error()
                .filter(|_| self.rebuilds() > self.installed_rebuild)
                .map(|error| format!("theme watcher: {error}")),
        );
        errors.collect()
    }

    /// The note the banner shows under the errors: why the watcher did not start.
    pub(crate) fn watcher_note(&self) -> Option<String> {
        #[cfg(feature = "watch")]
        if let Some(error) = &self.watcher_error {
            return Some(format!("OS theme changes are not followed: {error}"));
        }
        None
    }
}

impl App {
    /// Whether the system menu bar AppKit holds is the showcase's (`system_menu::menus`), on
    /// macOS outside `cfg(test)`: the showcase's installed, and the five titles read back are
    /// the ones the shared crate gives, in order (`menu::check_main_menu`). The error says
    /// which does not hold.
    #[cfg(all(target_os = "macos", not(test)))]
    pub(crate) fn menu_installed(&self) -> Result<(), String> {
        if self.system_menu.is_none() {
            return Err("the showcase's macOS menu bar did not install".to_string());
        }
        native_theme_showcase_support::menu::check_main_menu(
            crate::chrome::system_menu::APP_NAME,
            native_theme_showcase_support::menu::main_menu_titles(),
        )
    }
    /// `Ok` wherever there is no system menu bar to read back.
    #[cfg(not(all(target_os = "macos", not(test))))]
    pub(crate) fn menu_installed(&self) -> Result<(), String> {
        Ok(())
    }

    /// `--dump-layout`: once this pass's layout has been the same for `DUMP_SETTLE_S`, write it
    /// (`dump::write_layout`), then close the window once every file the run asked for is
    /// written, unless `--capture` keeps it for the tool that captures it (`cli::Outputs`). A
    /// failed write is reported and ends the run with 1, as a failed `--screenshot` does
    /// (`cli::refuse`), and so does a layout that has not settled `CAPTURE_WAIT` after the first
    /// pass (`dump_given_up`).
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
        let started = *dump.started.get_or_insert(now);
        ctx.request_repaint();
        if now - since < crate::DUMP_SETTLE_S {
            if let Some(failure) = dump_given_up(&self.outputs, &dump.path, now - started) {
                cli::refuse(&failure);
            }
            return;
        }
        let json = layout_json(places, &preset, ctx.theme(), ctx.pixels_per_point());
        if let Err(error) = dump::write_layout(&dump.path, &json) {
            cli::refuse(&error);
        }
        eprintln!("Layout dumped to {}", dump.path);
        dump.written = true;
        if self.outputs.written(Output::Dump) {
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
            Action::ReloadTheme => {
                native_theme::detect::invalidate_caches();
                self.install(ctx);
            }
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

/// Why `--dump-layout` to `path` failed, where `outputs` still waits for the dump `waited`
/// seconds of input time after its first pass, once that reaches the shared crate's
/// `CAPTURE_WAIT`, the bound the gpui and iced showcases give up theirs at: a layout that never
/// stays the same for `DUMP_SETTLE_S` fails the run (exit 1) rather than keep it open,
/// `--screenshot` written or not. `None` before then, once the dump is written, or where the
/// run asked for none.
pub(crate) fn dump_given_up(outputs: &Outputs, path: &str, waited: f64) -> Option<String> {
    (outputs.pending(Output::Dump) && waited >= CAPTURE_WAIT.as_secs_f64()).then(|| {
        Output::Dump.failure(
            path,
            &format!(
                "no layout was written within {} s: the layout did not stay the same for {} s",
                CAPTURE_WAIT.as_secs(),
                crate::DUMP_SETTLE_S
            ),
        )
    })
}

/// The window's content in physical pixels and the display's scale factor, as egui has them
/// when the capture asks (`FrameCapture::poll`); a scale egui does not know is refused here, so
/// the frame check (`capture::check_frame_capture`) always has one to measure against.
pub(crate) fn content_px(ctx: &egui::Context) -> Result<Content, String> {
    let (inner, scale) = ctx.input(|i| {
        (
            i.viewport().inner_rect,
            i.viewport().native_pixels_per_point,
        )
    });
    let inner = inner.ok_or("the window's content rect is unknown")?;
    let scale = scale.ok_or("the display's scale factor is unknown")?;
    let px = |points: f32| (points * scale).round() as i64;
    Ok(((px(inner.width()), px(inner.height())), f64::from(scale)))
}

impl eframe::App for App {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        // A captured or dumped window shows the run as it started whatever is typed while it
        // runs: a key the desktop hands it types into no field and runs no shortcut.
        if self.shields_keyboard {
            raw_input.events.retain(|e| !keyboard_event(e));
        }
        let Some(held) = &mut self.held_pointer else {
            // A captured or dumped window shows the page at rest wherever the real pointer is:
            // no control hovered or pressed under it, no Widget Info chosen by it, nothing
            // scrolled by it.
            if self.shields_pointer {
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
        if self
            .watcher
            .as_ref()
            .and_then(native_theme_egui::ThemeWatcher::take)
            .is_some()
        {
            self.take_rebuilt(ctx);
        }
        // A new theme or a scheme switch re-derives a following icon choice (§10.4:
        // `breeze` to `breeze-dark` on `kde-breeze`); a pass on the same scheme does nothing.
        let scheme = ctx.theme();
        if self.last_scheme != Some(scheme) {
            let before = self.settings.icon.clone();
            self.settings.theme_installed(&self.atlas, scheme);
            self.icons_chosen(&before);
            self.last_scheme = Some(scheme);
        }
        let now = ctx.input(|i| i.time);
        let due = self.screenshot.as_mut().is_some_and(|shot| {
            let started = *shot.started.get_or_insert(now);
            !shot.written && shot.capture.is_none() && now - started >= SCREENSHOT_DELAY_S
        });
        // The macOS runner's check (§13), read on the pass the capture is due alone, and before
        // the screenshot's `&mut` borrow of `self` (§10.4's fourth rule): a capture step fails
        // where the menu bar AppKit holds is not the showcase's.
        let menu_installed = due.then(|| self.menu_installed());
        if let Some(shot) = &mut self.screenshot {
            if due {
                if let Some(Err(error)) = &menu_installed {
                    cli::refuse(&Output::Screenshot.failure(&shot.path, error));
                }
                match FrameCapture::start(crate::WINDOW_TITLE, &shot.path) {
                    Ok(capture) => shot.capture = Some(capture),
                    Err(error) => cli::refuse(&Output::Screenshot.failure(&shot.path, &error)),
                }
            }
            match shot
                .capture
                .as_mut()
                .and_then(|c| c.poll(|| content_px(ctx)))
            {
                None => {}
                Some(Ok(())) => {
                    eprintln!("Screenshot saved to {}", shot.path);
                    shot.capture = None;
                    shot.written = true;
                    if self.outputs.written(Output::Screenshot) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                Some(Err(error)) => {
                    cli::refuse(&Output::Screenshot.failure(&shot.path, &error));
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
        chrome::central_panel(self, ui, |app, ui, margin| {
            if pages::has_fixed(app) {
                egui::Frame::NONE
                    .inner_margin(margin)
                    .show(ui, |ui| pages::show_fixed(app, ui));
            }
            // The page scrolls vertically in the panel, padded by the panel's margin inside the
            // scroll area, which spans the panel: its bar at the panel's edge, as the gpui and
            // iced content panels draw theirs. At rest it is at the top.
            let _page = egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    egui::Frame::NONE
                        .inner_margin(margin)
                        .show(ui, |ui| pages::show(app, ui));
                });
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
