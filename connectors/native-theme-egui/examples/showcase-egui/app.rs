//! The application: its settings, its atlas, and the `eframe::App` impl (spec §10.4).

#[cfg(feature = "watch")]
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme::next::{
    Contrast, Direction, MotionPreference, Palette, Preset, Scheme, ThemeContext,
};
use native_theme::{AccessibilityPreferences, SystemTheme, icons::IconSet};
use native_theme_egui::{ColorMode, SystemThemeExt as _, ThemeAtlas, from_preset};
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
    /// The palette the Palette row and `--palette` lay over `theme` (`Palette::BUNDLED_NAMES`),
    /// by its name; `None` for the preset's own colours.
    pub palette: Option<&'static str>,
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
            palette: None,
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
    pub(crate) built: Option<(u64, u64, OldAtlas)>,
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

/// The macOS system menu's actions `logic` runs, `drained` from the menu's channel: every one
/// is taken off the channel, and while the window is shielded from the keyboard
/// (`shields_keyboard`) none is run, so a key equivalent typed while the window is captured or
/// dumped changes nothing it shows, as a key the window gets does not (`raw_input_hook`). Built
/// where the system menu is, and in the tests on every platform.
#[cfg(any(target_os = "macos", test))]
pub(crate) fn system_menu_actions(
    drained: impl Iterator<Item = Action>,
    shields_keyboard: bool,
) -> Vec<Action> {
    drained.filter(|_| !shields_keyboard).collect()
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

/// The text of the error [`OldAtlas::of`]'s `None` gives.
pub(crate) const NOT_OLD: &str = "the atlas holds no 0.7.2 theme, which the showcase's pages read";

/// What the Palette row's hover text says of the pages still on the 0.7.2 model, which has no
/// palettes.
pub(crate) const PALETTE_TOOLTIP: &str = "A palette recolours the preset (0.8.0 model): the \
     installed egui styles and frames -- every widget the connector styles, the companion \
     crate's widgets and the chrome's frames -- are built from the 0.8.0 theme with the palette. \
     The showcase's own refinements on the pages and the chrome (their padding, gaps, row heights, \
     fonts and the colours they draw themselves) still read the 0.7.2 preset, which has no \
     palettes, until their page group moves to the 0.8.0 model: they show the preset alone, in \
     the palette's scheme.";

/// The desktop's settings as the 0.8.0 model reads them (`next::Theme::from_system`), or the
/// error to report where the read failed. No preset stands in for them.
pub(crate) type NextSystem = Result<native_theme::next::Theme, String>;

/// The 0.8.0 model's read of the desktop ([`NextSystem`]), its error worded as the 0.7.2 read's
/// is reported.
pub(crate) fn read_next_system() -> NextSystem {
    native_theme::next::Theme::from_system().map_err(|e| format!("Failed to load OS theme: {e}"))
}

/// Why an install did not build: the 0.7.2 model's error, or the 0.8.0 model's, as the text to
/// report (a failed desktop read, a preset or palette it has none of, a theme the connector
/// cannot install).
#[derive(Debug)]
pub(crate) enum BuildError {
    Old(native_theme::Error),
    Next(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::Old(error) => error.fmt(f),
            BuildError::Next(error) => f.write_str(error),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<native_theme::Error> for BuildError {
    fn from(error: native_theme::Error) -> Self {
        BuildError::Old(error)
    }
}

#[cfg(feature = "watch")]
impl BuildError {
    /// The error the watcher's `rebuild` returns, which takes the 0.7.2 model's type: that
    /// model's error as it is, the 0.8.0 model's text as the error of the showcase's rebuild.
    fn into_native(self) -> native_theme::Error {
        match self {
            BuildError::Old(error) => error,
            BuildError::Next(error) => native_theme::Error::ReaderFailed {
                reader: "showcase-egui rebuild",
                source: error.into(),
            },
        }
    }
}

/// Why the installed 0.8.0 theme is at a lower contrast level than the context asks for
/// (docs/v0.8.0-spec.md §6, resolution step 1): `asked`, `used`, and whether the palette laid
/// over the theme holds `High` back, which the theme without it reaches in the palette's scheme.
/// Otherwise the theme defines the level asked for in none of its sources, its preset's levels
/// or the system's settings, for the scheme resolved. As the iced showcase's `ContrastHeld`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ContrastHeld {
    pub(crate) asked: Contrast,
    pub(crate) used: Contrast,
    pub(crate) by_palette: bool,
}

impl ContrastHeld {
    /// Why `resolved`, `theme` resolved in `ctx`, is at a lower contrast than `ctx` asks for,
    /// where it is; `alone`, the same theme without the palette, where a palette is laid over
    /// it, tells whether the palette holds `High` back.
    fn of(
        ctx: &ThemeContext,
        resolved: &native_theme::next::ResolvedTheme,
        alone: Option<&native_theme::next::Theme>,
    ) -> Option<Self> {
        let (asked, used) = (ctx.contrast, resolved.context.contrast);
        (used != asked).then(|| ContrastHeld {
            asked,
            used,
            by_palette: asked == Contrast::High
                && alone.is_some_and(|alone| {
                    let mut in_scheme = ctx.clone();
                    in_scheme.scheme = resolved.context.scheme;
                    alone.resolve(&in_scheme).context.contrast == Contrast::High
                }),
        })
    }

    /// The sentence the Preferences dialog's High contrast row says it in, as the gpui and
    /// iced showcases' (`support::ContrastHeld::text`, `ContrastHeld::sentence`).
    pub(crate) fn sentence(&self) -> String {
        let Self {
            asked,
            used,
            by_palette,
        } = self;
        let why = if *by_palette {
            "the palette laid over the theme holds High back: a palette stays where High is not \
             taken, and the theme without it has High in the palette's scheme"
        } else {
            "the theme defines no such level for the scheme it is resolved in -- neither its \
             preset nor the system's settings state one -- so the nearest level below it is used"
        };
        format!(
            "the theme is resolved at {used:?} contrast, not the {asked:?} asked for: {why} \
             (docs/v0.8.0-spec.md §6, resolution step 1)"
        )
    }
}

/// What the Palette row offers, as `(palette, row text)`: `none` ([`cli::NO_PALETTE`]), the
/// preset's own colours, then every bundled palette (`Palette::BUNDLED_NAMES`), by the name
/// `--palette` takes (`cli::palette_key`).
pub(crate) fn palette_rows() -> Vec<(Option<&'static str>, String)> {
    std::iter::once((None, cli::NO_PALETTE.to_owned()))
        .chain(
            Palette::BUNDLED_NAMES
                .iter()
                .map(|name| (Some(*name), (*name).to_owned())),
        )
        .collect()
}

/// The context a 0.8.0 theme is installed in (spec §6): `base`, the desktop's as the 0.8.0
/// model read it -- where the read failed, the default context every preset alone is resolved
/// in (`next::Theme::preset`) -- in `scheme`, for an active window, left to right as the
/// showcase lays itself out, with `prefs` as its text scale, motion, contrast and reduced
/// transparency: the accessibility preferences the 0.7.2 theme of the same choice is built
/// with, the OS's with the user's over them ([`effective_preferences`]). A preference set keeps
/// the desktop's own level where that is one of its kind (a contrast `Increased`, a motion
/// `None`), as the gpui and iced showcases' (`Showcase::next_context`, `next_context`).
pub(crate) fn next_context(
    base: Option<&ThemeContext>,
    scheme: Scheme,
    prefs: &AccessibilityPreferences,
) -> ThemeContext {
    let mut ctx = base.cloned().unwrap_or_default();
    ctx.scheme = scheme;
    ctx.text_scale = prefs.text_scaling_factor;
    ctx.motion = match (prefs.reduce_motion, ctx.motion) {
        (false, _) => MotionPreference::Full,
        (true, MotionPreference::Full) => MotionPreference::Reduced,
        (true, reduced) => reduced,
    };
    ctx.contrast = match (prefs.high_contrast, ctx.contrast) {
        (false, _) => Contrast::Normal,
        (true, Contrast::Normal) => Contrast::High,
        (true, raised) => raised,
    };
    ctx.reduced_transparency = prefs.reduce_transparency;
    ctx.direction = Direction::Ltr;
    ctx.window_active = true;
    ctx
}

/// The 0.8.0 themes an install built beside its atlas, per scheme slot (light, then dark):
/// each for an inactive window, and why its contrast is lower than asked, where it is.
struct NextSlots {
    inactive: [native_theme::next::ResolvedTheme; 2],
    contrast_held: [Option<ContrastHeld>; 2],
}

/// The atlas the showcase installs, with the 0.7.2 themes of the same choice beside it, which
/// the showcase's pages read through [`OldAtlas::old_resolved`] until they are ported to the
/// 0.8.0 model, as the gpui showcase keeps its `OldModel`; it derefs to the installed atlas.
///
/// [`App::rebuild_atlas`] builds both from the settings' choice: the installed atlas from the
/// 0.8.0 theme of the choice with its palette (`ThemeAtlas::next_builder`), and, per scheme
/// slot, the 0.7.2 theme of the same choice in the scheme the slot's 0.8.0 theme resolved to:
/// the preset alone under a palette, which the 0.7.2 model has no equivalent of. An atlas of
/// the 0.7.2 model alone ([`OldAtlas::of`]) carries its own themes.
#[derive(Clone)]
pub(crate) struct OldAtlas {
    atlas: ThemeAtlas,
    old: std::sync::Arc<[native_theme_egui::ResolvedTheme; 2]>,
    /// The accessibility preferences the install asked for: the 0.7.2 atlas's, which the 0.8.0
    /// theme's context reads back lowered where the theme holds a level back.
    asked: AccessibilityPreferences,
    next: Option<std::sync::Arc<NextSlots>>,
}

impl std::fmt::Debug for OldAtlas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OldAtlas")
            .field("atlas", &self.atlas)
            .field("next", &self.next.is_some())
            .finish_non_exhaustive()
    }
}

impl OldAtlas {
    /// `atlas` with its 0.7.2 themes, light then dark; `None` for an atlas of the 0.8.0 model
    /// (`native_theme_egui::next::to_theme`).
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "the tests install 0.7.2 atlases of their own")
    )]
    pub(crate) fn of(atlas: ThemeAtlas) -> Option<Self> {
        let light = atlas.resolved_for(egui::Theme::Light)?.clone();
        let dark = atlas.resolved_for(egui::Theme::Dark)?.clone();
        Some(Self {
            asked: atlas.accessibility().clone(),
            atlas,
            old: std::sync::Arc::new([light, dark]),
            next: None,
        })
    }

    /// The 0.7.2 theme of `theme`'s scheme.
    pub(crate) fn old_resolved(&self, theme: egui::Theme) -> &native_theme_egui::ResolvedTheme {
        let [light, dark] = &*self.old;
        match theme {
            egui::Theme::Light => light,
            egui::Theme::Dark => dark,
        }
    }

    /// The accessibility preferences the install asked for, which the Preferences dialog and
    /// the status bar show: the user's over the OS's.
    pub(crate) fn asked(&self) -> &AccessibilityPreferences {
        &self.asked
    }

    /// The installed 0.8.0 theme of `theme`'s slot for an active window, `None` for an atlas
    /// of the 0.7.2 model.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the page groups call it as they move to the 0.8.0 model"
        )
    )]
    pub(crate) fn next_resolved(
        &self,
        theme: egui::Theme,
    ) -> Option<&native_theme::next::ResolvedTheme> {
        self.atlas.next_resolved_for(theme)
    }

    /// The installed 0.8.0 theme of `theme`'s slot for an inactive window.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the page groups call it as they move to the 0.8.0 model"
        )
    )]
    pub(crate) fn next_inactive(
        &self,
        theme: egui::Theme,
    ) -> Option<&native_theme::next::ResolvedTheme> {
        let [light, dark] = &self.next.as_deref()?.inactive;
        Some(match theme {
            egui::Theme::Light => light,
            egui::Theme::Dark => dark,
        })
    }

    /// The installed 0.8.0 theme a page on the 0.8.0 model reads: the scheme egui draws,
    /// resolved for an inactive window while the viewport reports itself unfocused, else for
    /// an active one.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the page groups call it as they move to the 0.8.0 model"
        )
    )]
    pub(crate) fn next_shown(
        &self,
        ctx: &egui::Context,
    ) -> Option<&native_theme::next::ResolvedTheme> {
        let theme = ctx.theme();
        if ctx.input(|i| i.viewport().focused) == Some(false) {
            self.next_inactive(theme)
        } else {
            self.next_resolved(theme)
        }
    }

    /// Why the 0.8.0 theme of `theme`'s slot is at a lower contrast than asked, where it is.
    pub(crate) fn contrast_held(&self, theme: egui::Theme) -> Option<&ContrastHeld> {
        let [light, dark] = &self.next.as_deref()?.contrast_held;
        match theme {
            egui::Theme::Light => light.as_ref(),
            egui::Theme::Dark => dark.as_ref(),
        }
    }

    /// The atlas [`App::adopt_pair`] last installed into `ctx` and kept there, for the code that is handed
    /// the `Context` alone, as `ThemeAtlas::from_ctx` gives the installed atlas.
    pub(crate) fn from_ctx(ctx: &egui::Context) -> Option<Self> {
        ctx.data(|d| d.get_temp::<Self>(egui::Id::NULL))
    }
}

impl std::ops::Deref for OldAtlas {
    type Target = ThemeAtlas;

    fn deref(&self) -> &ThemeAtlas {
        &self.atlas
    }
}

pub(crate) struct App {
    /// The one field for theme state (§10.4's third rule).
    pub(crate) atlas: OldAtlas,
    pub(crate) settings: Settings,
    /// The platform preset `default` builds on, `SystemTheme::preset` since the last read of
    /// the OS: the name the Theme row, the command palette and the status bar give `default`,
    /// "default (kde-breeze)", as the gpui showcase's `default_label`
    /// (`showcase-gpui/app.rs:1972-1976`).
    pub(crate) default_preset: String,
    /// The desktop's settings as the 0.8.0 model last read them ([`read_next_system`]): read
    /// again at each install of `default`, as the 0.7.2 model's are, and where none is kept;
    /// Reload System Theme and a rebuild of the watcher's let it go.
    pub(crate) next_system: Option<NextSystem>,
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
    /// `CliArgs::shields_keyboard`); on macOS the system menu's own items run nothing either
    /// ([`system_menu_actions`]). Outside the shield are AppKit's predefined items, which AppKit
    /// runs itself and the showcase never sees: Close ⌘W, Quit ⌘Q, Hide ⌘H, Hide Others ⌥⌘H
    /// and Minimize ⌘M.
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
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut settings = Settings::initial();
        apply_cli_args(&mut settings, cli);
        let mut default_preset = native_theme::pipeline::platform_preset_name()
            .name
            .to_string();
        let next_system = read_next_system();
        // Where the choice does not build, the platform's preset in its place, which the Theme
        // row then shows: a fallback is never labelled the desktop's own theme.
        let (atlas, theme_error) = match Self::rebuild_atlas(&settings, &next_system) {
            Ok((atlas, preset)) => {
                if let Some(preset) = preset {
                    default_preset = preset;
                }
                (atlas, None)
            }
            Err(error) => {
                settings.theme = ThemeChoice::Preset(default_preset.clone());
                let (atlas, _) = Self::rebuild_atlas(&settings, &next_system)?;
                (
                    atlas,
                    Some(format!("{error}. Using {default_preset} fallback.")),
                )
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
            next_system: Some(next_system),
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
        // After the install, which clears it: the platform's preset stands in for the choice.
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

    /// The 0.7.2 atlas `settings` select (§10.4): `default` is `SystemTheme::from_system`
    /// through `to_egui_atlas` after `invalidate_caches()`, its public `accessibility`
    /// (`native-theme/src/lib.rs:643`) the [`effective_preferences`] of the OS's reading it
    /// holds and Preferences' overrides; a preset is `from_preset` with the effective
    /// preferences of the cached OS reading (`detect::accessibility_preferences`, which the
    /// watcher and `ReloadTheme` clear before they rebuild) and the overrides. `from_preset`'s
    /// `is_dark` selects only the `ResolvedTheme` this discards — the atlas carries both
    /// variants and egui picks the scheme (§4.6) — so `false` is passed. With the atlas comes,
    /// for `default`, the platform preset it builds on (`SystemTheme::preset`).
    fn rebuild_old(settings: &Settings) -> native_theme::Result<(ThemeAtlas, Option<String>)> {
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

    /// The atlas `settings` select, the one place that builds both models from the theme
    /// settings' choice ([`OldAtlas`]). The 0.8.0 theme of the choice -- `default`, the
    /// desktop's own as the 0.8.0 model read it (`system`), or a preset alone -- with the
    /// palette chosen laid over it (`next::Theme::with_palette`, which locks the scheme to the
    /// palette's), is resolved per scheme slot in [`next_context`] (on the desktop's context
    /// where it was read) with the preferences the 0.7.2 atlas of the choice is built with
    /// ([`App::rebuild_old`]), and for an inactive window beside it; the atlas installed is
    /// built from those four themes by `ThemeAtlas::next_builder`, with the OS's faces for the
    /// light theme's text where `system-fonts` is on (`next::font_plan_from_system`), as the
    /// 0.7.2 atlas takes them, and the OS colour mode: the desktop's scheme where the 0.8.0
    /// model read it, else `detect::system_is_dark`, as `from_preset` reads it. Each slot keeps
    /// the 0.7.2 theme of the choice in the scheme its 0.8.0 theme resolved to. With the atlas
    /// comes, for `default`, the platform preset it builds on (`SystemTheme::preset`). It reads
    /// `theme`, `palette` and `prefs` alone, not `mode`, which the watcher's selection relies
    /// on: a mode switch publishes nothing to it, as the atlas holds a theme for each scheme.
    pub(crate) fn rebuild_atlas(
        settings: &Settings,
        system: &NextSystem,
    ) -> Result<(OldAtlas, Option<String>), BuildError> {
        let (old, preset) = Self::rebuild_old(settings)?;
        let alone = match &settings.theme {
            ThemeChoice::Default => system.clone().map_err(BuildError::Next)?,
            ThemeChoice::Preset(name) => {
                native_theme::next::Theme::preset(Preset::from_name(name).ok_or_else(|| {
                    BuildError::Next(format!(
                        "Failed to load preset '{name}': the 0.8.0 model has no preset of that name"
                    ))
                })?)
            }
        };
        let (theme, under) = match settings.palette {
            None => (alone, None),
            Some(name) => {
                let palette = Palette::from_name(name).ok_or_else(|| {
                    BuildError::Next(format!(
                        "Failed to load palette '{name}': no bundled palette"
                    ))
                })?;
                (alone.clone().with_palette(palette), Some(alone))
            }
        };
        let base = system.as_ref().ok().map(|system| system.context().clone());
        let asked = old.accessibility().clone();
        let slot = |scheme| {
            let ctx = next_context(base.as_ref(), scheme, &asked);
            let resolved = theme.resolve(&ctx);
            let mut inactive_ctx = ctx.clone();
            inactive_ctx.window_active = false;
            let inactive = theme.resolve(&inactive_ctx);
            let held = ContrastHeld::of(&ctx, &resolved, under.as_ref());
            (resolved, inactive, held)
        };
        let (light, light_inactive, light_held) = slot(Scheme::Light);
        let (dark, dark_inactive, dark_held) = slot(Scheme::Dark);
        let os_dark = match &base {
            Some(base) => base.scheme == Scheme::Dark,
            None => native_theme::detect::system_is_dark(),
        };
        let builder = ThemeAtlas::next_builder(&light, &light_inactive, &dark, &dark_inactive)
            .os_mode(if os_dark {
                ColorMode::Dark
            } else {
                ColorMode::Light
            });
        let unmapped = |e: native_theme_egui::next::Unmapped| {
            BuildError::Next(format!("Theme '{}' cannot be installed: {e}", light.name))
        };
        #[cfg(feature = "system-fonts")]
        let builder = builder
            .fonts(native_theme_egui::next::font_plan_from_system(&light).map_err(unmapped)?);
        let atlas = builder.build().map_err(unmapped)?;
        let old_of = |next: &native_theme::next::ResolvedTheme| {
            let scheme = if next.is_dark() {
                egui::Theme::Dark
            } else {
                egui::Theme::Light
            };
            old.resolved_for(scheme).cloned()
        };
        let (Some(old_light), Some(old_dark)) = (old_of(&light), old_of(&dark)) else {
            return Err(BuildError::Next(NOT_OLD.to_owned()));
        };
        let pair = OldAtlas {
            atlas,
            old: std::sync::Arc::new([old_light, old_dark]),
            asked,
            next: Some(std::sync::Arc::new(NextSlots {
                inactive: [light_inactive, dark_inactive],
                contrast_held: [light_held, dark_held],
            })),
        };
        Ok((pair, preset))
    }

    /// Rebuild what `settings` select and install it; a failure keeps the installed atlas and
    /// is shown on the page (§10.4, the theme error in the content). An install of `default`
    /// reads the desktop again in both models; a preset's reads the 0.8.0 model's desktop
    /// settings, its context's source, only where none are kept. A success covers the
    /// watcher's rebuilds returned before it read the record: a rebuild still running counts as
    /// newer, since it may carry an OS change this rebuild did not read.
    pub(crate) fn install(&mut self, ctx: &egui::Context) {
        #[cfg(feature = "watch")]
        let rebuilds = self.rebuilds();
        if self.settings.theme == ThemeChoice::Default {
            self.next_system = None;
        }
        let system = self.next_system.get_or_insert_with(read_next_system);
        match Self::rebuild_atlas(&self.settings, system) {
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
    fn apply(&mut self, ctx: &egui::Context, atlas: OldAtlas) {
        self.adopt_pair(ctx, atlas);
        ctx.set_theme(self.theme_preference());
        #[cfg(feature = "watch")]
        self.publish_selection();
    }

    /// [`App::adopt_pair`] of an atlas of the 0.7.2 model alone, with its own themes, as the
    /// tests install one. An atlas of the 0.8.0 model alone is not installed, as it carries no
    /// 0.7.2 theme for the pages: the theme error says so.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "the tests install 0.7.2 atlases of their own")
    )]
    pub(crate) fn adopt(&mut self, ctx: &egui::Context, atlas: ThemeAtlas) {
        let Some(atlas) = OldAtlas::of(atlas) else {
            self.theme_error = Some(NOT_OLD.to_owned());
            return;
        };
        self.adopt_pair(ctx, atlas);
    }

    /// The steps every install of an atlas takes, and nothing else: `atlas` installed into
    /// `ctx` and held, the OS's semibold faces of its family registered, the JSON cache, the
    /// icons the showcase keeps (the install forgets the connector's icon URIs they name) and
    /// the scheme the icon choice was derived for forgotten, the shown Widget Info released, and
    /// the theme error cleared, since a theme now loaded. `apply` runs it, so does the watcher's
    /// hand-off, which leaves the mode as it is, and so do the tests that install an atlas of
    /// their own; it sets no theme preference, so a test that holds a scheme keeps it.
    pub(crate) fn adopt_pair(&mut self, ctx: &egui::Context, atlas: OldAtlas) {
        atlas.atlas.install(ctx);
        // Kept in `ctx` for `OldAtlas::from_ctx`, beside the installed atlas.
        ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, atlas.clone()));
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
    /// on, then this platform's presets the 0.8.0 model has a `Preset` of, by their display
    /// names, as the gpui and iced showcases offer them (`cli::next_theme_key`). The community
    /// presets of the 0.7.2 model are not offered: their colours are palettes in the 0.8.0
    /// model, chosen in the Palette row ([`palette_rows`]) over any preset.
    pub(crate) fn theme_rows(&self) -> Vec<(ThemeChoice, String)> {
        rows::preset_rows(&self.default_preset)
            .into_iter()
            .filter_map(|row| Some((cli::next_theme_key(&row.key).ok()?, row.display_name)))
            .collect()
    }

    /// Lay `palette` over the theme chosen (`None`: its own colours) and install it again, as a
    /// pick of the Palette row does.
    pub(crate) fn choose_palette(&mut self, palette: Option<&'static str>, ctx: &egui::Context) {
        self.settings.palette = palette;
        self.install(ctx);
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
    /// rebuild reads the OS accessibility preferences again, and the desktop read again by the
    /// 0.8.0 model, then the atlas it selects built and recorded.
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
        let result = App::rebuild_atlas(&settings, &read_next_system()).map(|(atlas, _)| atlas);
        App::record_rebuild(rebuilt, generation, result)
    }

    /// A rebuild for the selection of `generation` returned `result`: counted, and an atlas
    /// recorded as the last built, before the connector's hand-off holds the installed one.
    #[cfg(feature = "watch")]
    pub(crate) fn record_rebuild(
        rebuilt: &Mutex<Rebuilt>,
        generation: u64,
        result: Result<OldAtlas, BuildError>,
    ) -> native_theme::Result<ThemeAtlas> {
        let mut record = rebuilt.lock().unwrap_or_else(PoisonError::into_inner);
        record.rebuilds = record.rebuilds.saturating_add(1);
        match result {
            Ok(atlas) => {
                let installed = atlas.atlas.clone();
                record.built = Some((record.rebuilds, generation, atlas));
                Ok(installed)
            }
            Err(error) => Err(error.into_native()),
        }
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
        // The desktop the rebuild read may have changed since the read kept.
        self.next_system = None;
        if generation == self.generation() {
            self.adopt_pair(ctx, atlas);
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
                self.next_system = None;
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
        // runs: a key the desktop hands it types into no field and runs no shortcut, and on
        // macOS the system menu's key equivalents, which never pass through here, are dropped
        // where `logic` drains them (`system_menu_actions`); AppKit's predefined items (⌘W ⌘Q
        // ⌘H ⌥⌘H ⌘M) run in AppKit, outside the shield.
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
            let actions = system_menu_actions(rx.try_iter(), self.shields_keyboard);
            self.pending.extend(actions);
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
