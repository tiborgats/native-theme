//! native-theme-iced — comprehensive widget showcase and designer reference.
//!
//! Demonstrates every styled iced widget with live theme switching across all
//! bundled `native-theme` presets (system / light / dark), every `native-theme`
//! metric helper, an icon gallery with icon-theme switching and source
//! tracking, and a theme map showing all palette colors. Layout mirrors the
//! gpui showcase: left sidebar with theme controls and a hover-driven Widget
//! Info inspector; tabbed content area on the right.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-iced --example showcase-iced
//! ```
//!
//! # What to look for
//!
//! - Sidebar switches theme presets, color modes, and icon sets at runtime —
//!   no restart needed. Watch how every widget re-themes together.
//! - Hover the Widget Info inspector to see which `ResolvedTheme` fields
//!   drive the widget currently under the mouse.
//! - The Theme Map tab exposes iced's 6-field `Palette` plus `Extended`
//!   palette values produced by `native-theme-iced`.
//! - The Icons tab demonstrates icon loading across Material, Lucide, and
//!   freedesktop sets, plus animated spinner playback with `Rotation::Floating`.
//!
//! # How this file is organised
//!
//! The source is split into section-divider blocks (`// ───────`) — one per
//! widget category, tab, or view. Search for the dividers to jump between
//! sections.

use iced::widget::{
    button, checkbox, column, combo_box, container, mouse_area, pick_list, progress_bar, radio,
    row, rule, scrollable, slider, space, svg, text, text_editor, text_input, toggler, tooltip,
    vertical_slider,
};
use iced::{Color, Element, Fill, Length, Padding, Theme};

use iced::Subscription;
use native_theme::detect::prefers_reduced_motion;
use native_theme::icons::{
    FreedesktopLoader, IconSetChoice, LucideLoader, MaterialLoader, SegoeIconsLoader,
    SfSymbolsLoader, default_icon_choice, list_freedesktop_themes, load_icon_indicator,
};
use native_theme::theme::{AnimatedIcon, IconData, IconRole, IconSet, TransformAnimation};
use native_theme_iced::icons::{
    AnimatedSvgHandles, animated_frames_to_svg_handles, spin_rotation_radians, to_svg_handle,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// UI spacing scale (local constants, not a theme property)
// ---------------------------------------------------------------------------

/// Fixed spacing scale for showcase UI layout.
///
/// These are UI constants used by this example application for layout spacing.
/// They are NOT theme-driven values -- they are design choices for this showcase.
struct Spacing {
    xxs: f32,
    xs: f32,
    s: f32,
    m: f32,
    l: f32,
    xl: f32,
}

impl Spacing {
    const fn new() -> Self {
        Self {
            xxs: 2.0,
            xs: 4.0,
            s: 8.0,
            m: 12.0,
            l: 16.0,
            xl: 24.0,
        }
    }
}

/// Showcase UI spacing constants.
const SP: Spacing = Spacing::new();

// ---------------------------------------------------------------------------
// CLI argument parsing
// ---------------------------------------------------------------------------

/// Optional CLI arguments for launching the showcase in a specific state.
///
/// Parsed from `std::env::args()` -- no external crate dependency.
/// When no arguments are provided the showcase behaves identically to before.
#[derive(Default)]
struct CliArgs {
    theme: Option<String>,
    variant: Option<String>,
    tab: Option<String>,
    icon_set: Option<String>,
    screenshot: Option<String>,
}

/// Global CLI args, set once in `main()` before the iced application starts.
static CLI_ARGS: OnceLock<CliArgs> = OnceLock::new();

impl CliArgs {
    fn parse() -> Self {
        let mut args = Self::default();
        let argv: Vec<String> = std::env::args().collect();
        let mut i = 1; // skip binary name
        while i < argv.len() {
            match argv[i].as_str() {
                "--theme" => {
                    i += 1;
                    if i < argv.len() {
                        args.theme = Some(argv[i].clone());
                    }
                }
                "--variant" => {
                    i += 1;
                    if i < argv.len() {
                        args.variant = Some(argv[i].to_lowercase());
                    }
                }
                "--tab" => {
                    i += 1;
                    if i < argv.len() {
                        args.tab = Some(argv[i].to_lowercase());
                    }
                }
                "--icon-set" => {
                    i += 1;
                    if i < argv.len() {
                        args.icon_set = Some(argv[i].clone());
                    }
                }
                "--screenshot" => {
                    i += 1;
                    if i < argv.len() {
                        args.screenshot = Some(argv[i].clone());
                    }
                }
                _ => {} // ignore unknown args
            }
            i += 1;
        }
        args
    }

    /// Map a tab name string to the corresponding `Tab` variant.
    fn parse_tab(name: &str) -> Option<Tab> {
        match name {
            "buttons" => Some(Tab::Buttons),
            "text-inputs" | "textinputs" => Some(Tab::TextInputs),
            "selection" => Some(Tab::Selection),
            "range" => Some(Tab::Range),
            "display" => Some(Tab::Display),
            "icons" => Some(Tab::Icons),
            "theme-map" | "thememap" => Some(Tab::ThemeMap),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Tab identifiers (right panel)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Buttons,
    TextInputs,
    Selection,
    Range,
    Display,
    Icons,
    ThemeMap,
}

impl Tab {
    const ALL: &[Tab] = &[
        Tab::Buttons,
        Tab::TextInputs,
        Tab::Selection,
        Tab::Range,
        Tab::Display,
        Tab::Icons,
        Tab::ThemeMap,
    ];

    fn label(self) -> &'static str {
        match self {
            Tab::Buttons => "Buttons",
            Tab::TextInputs => "Text Inputs",
            Tab::Selection => "Selection",
            Tab::Range => "Range",
            Tab::Display => "Display",
            Tab::Icons => "Icons",
            Tab::ThemeMap => "Theme Map",
        }
    }
}

// ---------------------------------------------------------------------------
// ThemeChoice enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum ThemeChoice {
    OsTheme(String),
    Preset(String),
}

impl std::fmt::Display for ThemeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeChoice::OsTheme(label) => write!(f, "{label}"),
            ThemeChoice::Preset(name) => write!(f, "{name}"),
        }
    }
}

impl PartialEq for ThemeChoice {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ThemeChoice::OsTheme(_), ThemeChoice::OsTheme(_)) => true,
            (ThemeChoice::Preset(a), ThemeChoice::Preset(b)) => a == b,
            // Different ThemeChoice variants are never equal
            _ => false,
        }
    }
}

impl Eq for ThemeChoice {}

/// Load the bundled adwaita preset as a last-resort fallback.
///
/// Returns `None` if any step fails (should not happen for bundled data,
/// but we never panic).
fn load_adwaita_fallback(is_dark: bool) -> Option<(native_theme::theme::ResolvedTheme, Theme)> {
    let nt = native_theme::theme::Theme::preset("adwaita").ok()?;
    let variant = nt
        .pick_variant(if is_dark {
            native_theme_iced::ColorMode::Dark
        } else {
            native_theme_iced::ColorMode::Light
        })
        .ok()?
        .clone();
    let r = variant.resolve_system().ok()?;
    let t = native_theme_iced::to_theme(&r, &nt.name);
    Some((r, t))
}

fn theme_choices(default_label: &str) -> Vec<ThemeChoice> {
    let mut choices = vec![ThemeChoice::OsTheme(default_label.to_string())];
    choices.extend(
        native_theme::theme::Theme::list_presets_for_platform()
            .iter()
            .map(|info| ThemeChoice::Preset(info.key.to_string())),
    );
    choices
}

// ---------------------------------------------------------------------------
// Color mode (System / Light / Dark)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppColorMode {
    System,
    Light,
    Dark,
}

impl AppColorMode {
    const ALL: &[AppColorMode] = &[
        AppColorMode::System,
        AppColorMode::Light,
        AppColorMode::Dark,
    ];

    fn is_dark(self) -> bool {
        match self {
            AppColorMode::Light => false,
            AppColorMode::Dark => true,
            AppColorMode::System => native_theme::detect::system_is_dark(),
        }
    }
}

impl std::fmt::Display for AppColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppColorMode::System => {
                let actual = if self.is_dark() { "Dark" } else { "Light" };
                write!(f, "System ({actual})")
            }
            AppColorMode::Light => write!(f, "Light"),
            AppColorMode::Dark => write!(f, "Dark"),
        }
    }
}

// ---------------------------------------------------------------------------
// Icon set choice (uses library type: native_theme::icons::IconSetChoice)
// ---------------------------------------------------------------------------

/// Build the available icon set choices for the dropdown.
///
/// Includes `Default(X)` (if icon_theme is specified), `System`, all installed
/// freedesktop themes, and the bundled sets (Material, Lucide).
fn build_icon_choices(
    icon_set: IconSet,
    icon_theme: Option<&str>,
    installed_themes: &[String],
) -> Vec<IconSetChoice> {
    let mut items = Vec::new();
    if let choice @ IconSetChoice::Default(_) = default_icon_choice(icon_set, icon_theme) {
        items.push(choice);
    }
    items.push(IconSetChoice::System);
    for name in installed_themes {
        items.push(IconSetChoice::Freedesktop(name.clone()));
    }
    items.push(IconSetChoice::Material);
    items.push(IconSetChoice::Lucide);
    items
}

// ---------------------------------------------------------------------------
// Radio demo choice
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Cherry,
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Apple => write!(f, "Apple"),
            Fruit::Banana => write!(f, "Banana"),
            Fruit::Cherry => write!(f, "Cherry"),
        }
    }
}

// ---------------------------------------------------------------------------
// Icon source tracking (matches gpui showcase)
// ---------------------------------------------------------------------------

/// Where an icon was loaded from.
#[derive(Clone, Copy, PartialEq)]
enum IconSource {
    /// Loaded from the OS/desktop icon theme (e.g. Breeze, Adwaita, SF Symbols).
    System,
    /// Bundled icon set (material or lucide) used directly.
    Bundled,
    /// System lookup failed; fell back to bundled Material SVGs.
    Fallback,
    /// No icon data available at all.
    NotFound,
}

impl IconSource {
    fn label(self) -> &'static str {
        match self {
            IconSource::System => "System",
            IconSource::Bundled => "Bundled",
            IconSource::Fallback => "Fallback",
            IconSource::NotFound => "Not Found",
        }
    }
}

struct LoadedIcon {
    role: IconRole,
    data: Option<IconData>,
    name: Option<&'static str>,
    source: IconSource,
}

/// Pre-load all 42 icons for the given choice, tracking source.
fn load_all_icons(
    choice: &IconSetChoice,
    resolved: &native_theme::theme::ResolvedTheme,
    theme_icon_set: IconSet,
) -> Vec<LoadedIcon> {
    let set = choice.effective_icon_set(theme_icon_set);
    let theme = choice.freedesktop_theme();
    let is_system_set = matches!(
        set,
        IconSet::Freedesktop | IconSet::SfSymbols | IconSet::SegoeIcons
    );

    // Foreground color for GTK symbolic icon recoloring (Adwaita, Yaru, etc.)
    let tc = resolved.defaults.text_color;
    let fg = Some([tc.r, tc.g, tc.b]);

    // For system icon sets, pre-load the Material set so we can detect fallbacks
    let material_icons: Vec<Option<IconData>> = if is_system_set {
        IconRole::ALL
            .iter()
            .map(|role| MaterialLoader::new(*role).load())
            .collect()
    } else {
        vec![]
    };

    IconRole::ALL
        .iter()
        .enumerate()
        .map(|(i, &role)| {
            let data = match set {
                IconSet::Freedesktop => {
                    let mut l = FreedesktopLoader::new(role).color_opt(fg);
                    if let Some(t) = theme {
                        l = l.theme(t);
                    }
                    l.load()
                }
                IconSet::Material => MaterialLoader::new(role).load(),
                IconSet::Lucide => LucideLoader::new(role).load(),
                IconSet::SfSymbols => SfSymbolsLoader::new(role).load(),
                IconSet::SegoeIcons => SegoeIconsLoader::new(role).load(),
                _ => None,
            };
            let name = native_theme::theme::icon_name(role, set);
            let source = match (&data, is_system_set) {
                (None, _) => IconSource::NotFound,
                (Some(_), false) => IconSource::Bundled,
                (Some(IconData::Svg(loaded)), true) => {
                    // Compare with Material to detect fallback
                    if let Some(Some(IconData::Svg(mat))) = material_icons.get(i) {
                        if loaded == mat {
                            IconSource::Fallback
                        } else {
                            IconSource::System
                        }
                    } else {
                        IconSource::System
                    }
                }
                (Some(_), true) => {
                    // RGBA or other data comes from native APIs, always system
                    IconSource::System
                }
            };
            LoadedIcon {
                role,
                data,
                name,
                source,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Animated icon cache builder
// ---------------------------------------------------------------------------

/// Build animation caches for all known icon sets.
///
/// Returns the full set of animation state fields that go into `State`.
#[allow(clippy::type_complexity)]
fn build_animation_caches(
    icon_set: native_theme::theme::IconSet,
) -> (
    Vec<(String, AnimatedSvgHandles)>,          // animated_frames
    Vec<usize>,                                 // animated_frame_indices
    Vec<Duration>,                              // animated_frame_elapsed
    Vec<(String, iced_core::svg::Handle, u32)>, // animated_spins
    Instant,                                    // animation_start
    bool,                                       // reduced_motion
    Vec<(String, iced_core::svg::Handle)>,      // animated_static
) {
    let mut animated_frames = Vec::new();
    let mut animated_spins = Vec::new();
    let mut animated_static = Vec::new();

    let set_name = icon_set.name().to_string();
    {
        if let Some(anim) = load_icon_indicator(icon_set) {
            // Cache static first-frame for reduced motion
            if let Some(handle) = to_svg_handle(anim.first_frame(), None) {
                animated_static.push((set_name.clone(), handle));
            }

            match &anim {
                AnimatedIcon::Frames(_) => {
                    if let Some(anim_handles) = animated_frames_to_svg_handles(&anim, None) {
                        animated_frames.push((set_name.clone(), anim_handles));
                    }
                }
                AnimatedIcon::Transform(data) => {
                    if let TransformAnimation::Spin { duration_ms } = data.animation()
                        && let Some(handle) = to_svg_handle(data.icon(), None)
                    {
                        animated_spins.push((set_name.clone(), handle, duration_ms.get()));
                    }
                }
                _ => {} // Future AnimatedIcon variants
            }
        }
    }

    let animated_frame_indices = vec![0; animated_frames.len()];
    let animated_frame_elapsed = vec![Duration::ZERO; animated_frames.len()];
    let animation_start = Instant::now();
    let reduced_motion = prefers_reduced_motion();

    (
        animated_frames,
        animated_frame_indices,
        animated_frame_elapsed,
        animated_spins,
        animation_start,
        reduced_motion,
        animated_static,
    )
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

struct State {
    // Theme
    current_choice: ThemeChoice,
    current_theme: Theme,
    color_mode: AppColorMode,
    is_dark: bool,
    current_resolved: native_theme::theme::ResolvedTheme,
    /// Icon set for the current theme (from Theme or SystemTheme, not ResolvedTheme).
    current_icon_set: IconSet,
    /// Icon theme name for the current theme.
    current_icon_theme: String,
    /// Dynamic label for the default theme entry, updated on color mode change.
    default_label: String,

    // Navigation
    active_tab: Tab,

    // Widget Info (hover-driven)
    widget_info: String,

    // Button tab
    button_press_count: u32,

    // Text input tab
    text_input_value: String,
    text_editor_content: text_editor::Content,

    // Selection tab
    checkbox_a: bool,
    checkbox_b: bool,
    checkbox_c: bool,
    selected_fruit: Option<Fruit>,
    toggler_enabled: bool,
    pick_list_selected: Option<String>,
    combo_state: combo_box::State<String>,
    combo_selected: Option<String>,

    // Range tab
    slider_value: f32,
    slider_step: f32,
    vslider_value: f32,
    progress_value: f32,

    // Icons tab
    icon_set_choice: IconSetChoice,
    icon_set_choices: Vec<IconSetChoice>,
    loaded_icons: Vec<LoadedIcon>,
    /// Cached list of installed freedesktop icon themes (populated once at init).
    installed_themes: Vec<String>,

    // Animated Icons state
    /// Cached SVG handles for frame-based animations: (set_name, AnimatedSvgHandles).
    animated_frames: Vec<(String, AnimatedSvgHandles)>,
    /// Current frame index per frame-based animation.
    animated_frame_indices: Vec<usize>,
    /// Elapsed time tracker per frame-based animation (for correct per-animation timing).
    animated_frame_elapsed: Vec<Duration>,
    /// Cached SVG handle + duration for transform (spin) animations: (set_name, handle, duration_ms).
    animated_spins: Vec<(String, iced_core::svg::Handle, u32)>,
    /// Start time for spin animations (used with spin_rotation_radians).
    animation_start: Instant,
    /// Whether reduced motion is active (cached at init).
    reduced_motion: bool,
    /// Static first-frame SVG handles for reduced motion: (set_name, handle).
    animated_static: Vec<(String, iced_core::svg::Handle)>,

    // Screenshot mode
    screenshot_path: Option<String>,
    screenshot_countdown: u8,

    /// Error message from theme loading, displayed as a banner in the UI.
    error_message: Option<String>,

    // Theme watcher (runtime dark/light toggle detection)
    /// Flag set by the ThemeSubscription background thread when the OS theme changes.
    theme_change_flag: Arc<AtomicBool>,
    /// RAII guard keeping the theme watcher background thread alive.
    _theme_watcher: Option<native_theme::watch::ThemeSubscription>,
}

impl Default for State {
    fn default() -> Self {
        let color_mode = AppColorMode::System;
        let is_dark = color_mode.is_dark();
        let (resolved, theme, initial_error, system_preset, init_icon_set, init_icon_theme) =
            match native_theme::SystemTheme::from_system() {
                Ok(system) => {
                    let r = system
                        .pick(if is_dark {
                            native_theme_iced::ColorMode::Dark
                        } else {
                            native_theme_iced::ColorMode::Light
                        })
                        .clone();
                    let t = native_theme_iced::to_theme(&r, &system.name);
                    let preset = system.preset.clone();
                    let is = system.icon_set;
                    let it = system.icon_theme.into_owned();
                    (r, t, None, preset, is, it)
                }
                Err(e) => {
                    // Fallback: load adwaita preset through resolve pipeline
                    match load_adwaita_fallback(is_dark) {
                        Some((r, t)) => (
                            r,
                            t,
                            Some(format!("OS theme failed: {e}. Using adwaita fallback.")),
                            "adwaita".to_string(),
                            IconSet::Freedesktop,
                            "Adwaita".to_string(),
                        ),
                        None => {
                            // This is the only safe fallback when both OS theme
                            // detection and the bundled adwaita preset fail.
                            // With bundled data this case is near-impossible.
                            // process::exit avoids constructing a dummy
                            // ResolvedTheme (30+ required fields).
                            eprintln!(
                                "Fatal: OS theme failed ({e}) and adwaita fallback \
                                 also failed. Cannot start."
                            );
                            std::process::exit(1);
                        }
                    }
                }
            };

        let languages = vec![
            "Rust".to_string(),
            "Python".to_string(),
            "JavaScript".to_string(),
            "TypeScript".to_string(),
            "Go".to_string(),
            "C++".to_string(),
            "Java".to_string(),
            "Swift".to_string(),
            "Kotlin".to_string(),
            "Zig".to_string(),
        ];

        // Populate installed freedesktop themes once at init.
        let installed_themes = list_freedesktop_themes();
        let init_icon_theme_opt: Option<&str> = Some(&init_icon_theme);
        let icon_set_choice = default_icon_choice(init_icon_set, init_icon_theme_opt);
        let icon_set_choices =
            build_icon_choices(init_icon_set, init_icon_theme_opt, &installed_themes);
        let loaded_icons = load_all_icons(&icon_set_choice, &resolved, init_icon_set);

        let anim_set = icon_set_choice.effective_icon_set(init_icon_set);
        let (
            animated_frames,
            animated_frame_indices,
            animated_frame_elapsed,
            animated_spins,
            animation_start,
            reduced_motion,
            animated_static,
        ) = build_animation_caches(anim_set);

        let default_label = format!("default ({})", system_preset);

        // Start theme watcher for runtime dark/light toggle detection.
        // Skip in screenshot mode — the watcher's background thread cleanup
        // races with the Cocoa runtime on macOS CI, causing SIGTRAP on exit.
        let theme_change_flag = Arc::new(AtomicBool::new(false));
        let is_screenshot = CLI_ARGS.get().is_some_and(|cli| cli.screenshot.is_some());
        let _theme_watcher = if is_screenshot {
            None
        } else {
            let flag_clone = theme_change_flag.clone();
            native_theme::watch::on_theme_change(move |_event| {
                flag_clone.store(true, Ordering::Release);
            })
            .ok()
        };

        let mut state = Self {
            current_choice: ThemeChoice::OsTheme(default_label.clone()),
            current_theme: theme,
            color_mode,
            is_dark,
            current_resolved: resolved,
            current_icon_set: init_icon_set,
            current_icon_theme: init_icon_theme,
            default_label,
            active_tab: Tab::Buttons,
            widget_info: String::new(),
            button_press_count: 0,
            text_input_value: String::new(),
            text_editor_content: text_editor::Content::with_text(
                "This is a multi-line text editor.\nEdit this text freely.\n\nIt supports:\n  - Multiple lines\n  - Scrolling\n  - Selection",
            ),
            checkbox_a: true,
            checkbox_b: false,
            checkbox_c: true,
            selected_fruit: Some(Fruit::Apple),
            toggler_enabled: false,
            pick_list_selected: Some("Rust".to_string()),
            combo_state: combo_box::State::new(languages),
            combo_selected: None,
            slider_value: 65.0,
            slider_step: 25.0,
            vslider_value: 50.0,
            progress_value: 72.0,
            icon_set_choice,
            icon_set_choices,
            loaded_icons,
            installed_themes,
            animated_frames,
            animated_frame_indices,
            animated_frame_elapsed,
            animated_spins,
            animation_start,
            reduced_motion,
            animated_static,
            screenshot_path: None,
            screenshot_countdown: 0,
            error_message: initial_error,
            theme_change_flag,
            _theme_watcher,
        };

        // Apply CLI overrides (if any)
        if let Some(cli) = CLI_ARGS.get() {
            // Override color mode first so theme resolution uses the right variant
            if let Some(ref v) = cli.variant {
                state.color_mode = if v == "dark" {
                    AppColorMode::Dark
                } else {
                    AppColorMode::Light
                };
                state.is_dark = state.color_mode.is_dark();
            }

            // Override theme
            if let Some(ref theme_name) = cli.theme {
                state.current_choice = ThemeChoice::Preset(theme_name.clone());
                state.rebuild_theme();
            } else if cli.variant.is_some() {
                // Re-apply the default theme with the new variant
                state.rebuild_theme();
            }

            // Override tab
            if let Some(ref tab_name) = cli.tab
                && let Some(tab) = CliArgs::parse_tab(tab_name)
            {
                state.active_tab = tab;
            }

            // Override icon set
            if let Some(ref set_name) = cli.icon_set {
                let choice = match set_name.as_str() {
                    "material" => IconSetChoice::Material,
                    "lucide" => IconSetChoice::Lucide,
                    "system" => IconSetChoice::System,
                    other => IconSetChoice::Freedesktop(other.to_string()),
                };
                state.icon_set_choice = choice.clone();
                state.loaded_icons =
                    load_all_icons(&choice, &state.current_resolved, state.current_icon_set);
                let anim_set = choice.effective_icon_set(state.current_icon_set);
                let (frames, indices, elapsed, spins, start, rm, statics) =
                    build_animation_caches(anim_set);
                state.animated_frames = frames;
                state.animated_frame_indices = indices;
                state.animated_frame_elapsed = elapsed;
                state.animated_spins = spins;
                state.animation_start = start;
                state.reduced_motion = rm;
                state.animated_static = statics;
            }

            // Apply screenshot settings
            if let Some(ref path) = cli.screenshot {
                state.screenshot_path = Some(path.clone());
                state.screenshot_countdown = 60; // 60 ticks × 50ms = 3s render delay
            }
        }

        state
    }
}

impl State {
    fn rebuild_theme(&mut self) {
        self.is_dark = self.color_mode.is_dark();
        let is_default = matches!(self.current_choice, ThemeChoice::OsTheme(_));
        // Track icon_theme as Option<&str> — None means the TOML didn't specify one.
        let mut icon_theme_opt: Option<String> = None;
        match &self.current_choice {
            ThemeChoice::OsTheme(_) => {
                match native_theme::SystemTheme::from_system() {
                    Ok(system) => {
                        // Platform presets always specify icon_theme.
                        self.current_icon_set = system.icon_set;
                        self.current_icon_theme = system.icon_theme.clone().into_owned();
                        icon_theme_opt = Some(self.current_icon_theme.clone());
                        self.current_resolved = system
                            .pick(if self.is_dark {
                                native_theme_iced::ColorMode::Dark
                            } else {
                                native_theme_iced::ColorMode::Light
                            })
                            .clone();
                        self.current_theme =
                            native_theme_iced::to_theme(&self.current_resolved, &system.name);
                        self.default_label = format!("default ({})", system.preset);
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message =
                            Some(format!("OS theme failed: {e}. Using adwaita fallback."));
                        if let Some((r, t)) = load_adwaita_fallback(self.is_dark) {
                            self.current_icon_set = IconSet::Freedesktop;
                            self.current_icon_theme = "Adwaita".to_string();
                            icon_theme_opt = Some("Adwaita".to_string());
                            self.current_resolved = r;
                            self.current_theme = t;
                        }
                    }
                }
            }
            ThemeChoice::Preset(name) => {
                let name = name.clone();
                let mode = if self.is_dark {
                    native_theme_iced::ColorMode::Dark
                } else {
                    native_theme_iced::ColorMode::Light
                };
                match native_theme::theme::Theme::preset(&name) {
                    Ok(nt) => match nt.resolve(mode) {
                        Ok(r) => {
                            let theme_name = nt.name.clone();
                            let icon_theme_string = r.icon_theme.into_owned();
                            icon_theme_opt =
                                r.icon_theme_explicit.then(|| icon_theme_string.clone());
                            self.current_icon_set = r.icon_set;
                            self.current_icon_theme = icon_theme_string;
                            self.current_resolved = r.variant;
                            self.current_theme =
                                native_theme_iced::to_theme(&self.current_resolved, &theme_name);
                            self.error_message = None;
                        }
                        Err(e) => {
                            self.error_message =
                                Some(format!("Theme '{name}' resolution failed: {e}"));
                        }
                    },
                    Err(e) => {
                        self.error_message = Some(format!("Failed to load preset '{name}': {e}"));
                    }
                }
            }
        }
        if is_default {
            self.current_choice = ThemeChoice::OsTheme(self.default_label.clone());
        }

        // Only re-derive the choice when the user is in "follow preset" mode.
        // All other variants (System, Freedesktop, Material, Lucide) represent
        // an explicit user choice that must be preserved across theme re-applications.
        let it_opt = icon_theme_opt.as_deref();
        if self.icon_set_choice.follows_preset() {
            self.icon_set_choice = default_icon_choice(self.current_icon_set, it_opt);
        }
        // Always rebuild the choices list (theme name in "default (X)" may have changed).
        self.icon_set_choices =
            build_icon_choices(self.current_icon_set, it_opt, &self.installed_themes);

        // Always reload icons — the resolved text_color may have changed (light↔dark)
        // even when the icon set choice is the same.
        {
            self.loaded_icons = load_all_icons(
                &self.icon_set_choice,
                &self.current_resolved,
                self.current_icon_set,
            );
            let anim_set = self
                .icon_set_choice
                .effective_icon_set(self.current_icon_set);
            let (af, afi, afe, asp, astart, rm, ast) = build_animation_caches(anim_set);
            self.animated_frames = af;
            self.animated_frame_indices = afi;
            self.animated_frame_elapsed = afe;
            self.animated_spins = asp;
            self.animation_start = astart;
            self.reduced_motion = rm;
            self.animated_static = ast;
        }
    }
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Message {
    // Navigation
    TabSelected(Tab),

    // Theme
    ThemeSelected(ThemeChoice),
    ColorModeSelected(AppColorMode),

    // Widget Info hover
    WidgetHovered(String),
    WidgetUnhovered,

    // Button tab
    ButtonPressed,

    // Text input tab
    TextInputChanged(String),
    EditorAction(text_editor::Action),

    // Selection tab
    CheckboxAToggled(bool),
    CheckboxBToggled(bool),
    CheckboxCToggled(bool),
    FruitSelected(Fruit),
    TogglerToggled(bool),
    PickListSelected(String),
    ComboBoxSelected(String),

    // Range tab
    SliderChanged(f32),
    StepSliderChanged(f32),
    VSliderChanged(f32),
    ProgressChanged(f32),

    // Icons tab
    IconSetSelected(IconSetChoice),

    // Animated Icons
    AnimationTick,

    // Screenshot
    ScreenshotTick,
    ScreenshotCaptured(Vec<u8>, u32, u32),

    // Theme watcher
    ThemeWatcherTick,
}

// ---------------------------------------------------------------------------
// Self-capture screenshot (macOS only)
// ---------------------------------------------------------------------------

/// Capture the iced window including decorations using macOS `screencapture -l`.
///
/// Gets the CGWindowID via NSApplication -> mainWindow -> windowNumber, then
/// shells out to `screencapture -l <id> -o <path>`.  This is the same approach
/// used by the gpui showcase capture.
#[cfg(target_os = "macos")]
fn capture_own_window_macos(output_path: &str) -> Result<(), String> {
    use std::process::Command;

    let Some(ns_app_class) = objc2::runtime::AnyClass::get(c"NSApplication") else {
        return Err("NSApplication class not found".into());
    };
    let window_id: i64 = unsafe {
        let ns_app: *mut objc2::runtime::AnyObject =
            objc2::msg_send![ns_app_class, sharedApplication];
        // Ensure the app is front-most (the second invocation on CI may
        // launch behind the terminal).
        let _: () = objc2::msg_send![ns_app, activateIgnoringOtherApps: true];

        // Try mainWindow first, then keyWindow, then the first window in
        // the windows array.  On CI the second run may not become main.
        let mut win: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, mainWindow];
        if win.is_null() {
            win = objc2::msg_send![ns_app, keyWindow];
        }
        if win.is_null() {
            let windows: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, windows];
            let count: usize = objc2::msg_send![windows, count];
            if count > 0 {
                win = objc2::msg_send![windows, objectAtIndex: 0usize];
            }
        }
        if win.is_null() {
            return Err("No window found".into());
        }
        objc2::msg_send![win, windowNumber]
    };

    let status = Command::new("screencapture")
        .args(["-l", &format!("{window_id}"), "-o", output_path])
        .status()
        .map_err(|e| format!("Failed to run screencapture: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("screencapture exited with {status}"))
    }
}

// ---------------------------------------------------------------------------
// Self-capture screenshot (Windows only)
// ---------------------------------------------------------------------------

/// Capture the iced window including decorations using Windows BitBlt.
///
/// Uses `FindWindowW` with the known window title to locate the correct HWND
/// (more reliable than `GetForegroundWindow` which may return a console or
/// other window on CI), then `BitBlt` + `GetDIBits` to extract pixel data.
#[cfg(target_os = "windows")]
fn capture_own_window_windows(output_path: &str) -> Result<(), String> {
    use windows::Win32::Foundation::*;
    use windows::Win32::Graphics::Dwm::*;
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::PCWSTR;

    unsafe {
        let title = format!(
            "Native Theme \u{2013} Iced Showcase, v{}",
            env!("CARGO_PKG_VERSION")
        );
        let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = FindWindowW(None, PCWSTR(title_w.as_ptr()))
            .map_err(|e| format!("FindWindowW failed: {e}"))?;

        // DWMWA_EXTENDED_FRAME_BOUNDS gives visible bounds in physical
        // screen pixels (excluding the invisible DWM border), matching
        // the screen DC coordinate space.  Fall back to GetWindowRect.
        let mut rect = RECT::default();
        if DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut _ as *mut std::ffi::c_void,
            std::mem::size_of::<RECT>() as u32,
        )
        .is_err()
        {
            GetWindowRect(hwnd, &mut rect).map_err(|e| format!("GetWindowRect failed: {e}"))?;
        }

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        if width <= 0 || height <= 0 {
            return Err(format!("Invalid window dimensions: {width}x{height}"));
        }
        eprintln!(
            "windows capture: rect=({},{},{},{}), size={}x{}",
            rect.left, rect.top, rect.right, rect.bottom, width, height
        );

        let screen_dc = GetDC(None);
        let mem_dc = CreateCompatibleDC(Some(screen_dc));
        let bitmap = CreateCompatibleBitmap(screen_dc, width, height);
        let old_obj = SelectObject(mem_dc, bitmap.into());

        let blt_result = BitBlt(
            mem_dc,
            0,
            0,
            width,
            height,
            Some(screen_dc),
            rect.left,
            rect.top,
            SRCCOPY | CAPTUREBLT,
        );

        if blt_result.is_err() {
            SelectObject(mem_dc, old_obj);
            let _ = DeleteObject(bitmap.into());
            let _ = DeleteDC(mem_dc);
            ReleaseDC(None, screen_dc);
            return Err("BitBlt failed".into());
        }

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0 as u32,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut pixels = vec![0u8; (width * height * 4) as usize];
        let lines = GetDIBits(
            mem_dc,
            bitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        SelectObject(mem_dc, old_obj);
        let _ = DeleteObject(bitmap.into());
        let _ = DeleteDC(mem_dc);
        ReleaseDC(None, screen_dc);

        if lines == 0 {
            return Err("GetDIBits returned 0 lines".into());
        }

        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2); // BGRA -> RGBA
            chunk[3] = 255; // force opaque
        }

        image::save_buffer(
            output_path,
            &pixels,
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        )
        .map_err(|e| format!("Failed to save PNG: {e}"))
    }
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::ScreenshotTick => {
            if state.screenshot_countdown > 0 {
                state.screenshot_countdown -= 1;
                if state.screenshot_countdown == 0 {
                    // Platform-dispatched self-capture (includes window decorations)
                    // macOS: screencapture -l
                    #[cfg(target_os = "macos")]
                    if let Some(ref path) = state.screenshot_path {
                        match capture_own_window_macos(path) {
                            Ok(()) => eprintln!("Screenshot saved to {path}"),
                            Err(e) => eprintln!("macOS self-capture failed: {e}"),
                        }
                        return iced::exit();
                    }
                    // Windows: BitBlt self-capture
                    #[cfg(target_os = "windows")]
                    if let Some(ref path) = state.screenshot_path {
                        match capture_own_window_windows(path) {
                            Ok(()) => eprintln!("Screenshot saved to {path}"),
                            Err(e) => eprintln!("Windows self-capture failed: {e}"),
                        }
                        return iced::exit();
                    }
                    // Linux (and other platforms): iced internal framebuffer
                    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                    {
                        return iced::window::latest().then(|opt_id| {
                            if let Some(id) = opt_id {
                                iced::window::screenshot(id).map(|s| {
                                    let bytes = s.rgba.to_vec();
                                    let w = s.size.width;
                                    let h = s.size.height;
                                    Message::ScreenshotCaptured(bytes, w, h)
                                })
                            } else {
                                iced::Task::none()
                            }
                        });
                    }
                }
            }
        }
        Message::ScreenshotCaptured(bytes, width, height) => {
            if let Some(ref path) = state.screenshot_path {
                let _ = image::save_buffer(path, &bytes, width, height, image::ColorType::Rgba8);
                eprintln!("Screenshot saved to {path}");
            }
            return iced::exit();
        }
        other => {
            update_inner(state, other);
        }
    }
    iced::Task::none()
}

fn update_inner(state: &mut State, message: Message) {
    match message {
        Message::TabSelected(tab) => {
            state.active_tab = tab;
        }
        Message::ThemeSelected(choice) => {
            state.current_choice = choice;
            state.rebuild_theme();
        }
        Message::ColorModeSelected(mode) => {
            state.color_mode = mode;
            state.rebuild_theme();
        }
        Message::WidgetHovered(info) => {
            state.widget_info = info;
        }
        Message::WidgetUnhovered => {
            // Keep last info visible (like gpui showcase)
        }
        Message::ButtonPressed => {
            state.button_press_count = state.button_press_count.saturating_add(1);
        }
        Message::TextInputChanged(value) => {
            state.text_input_value = value;
        }
        Message::EditorAction(action) => {
            state.text_editor_content.perform(action);
        }
        Message::CheckboxAToggled(v) => state.checkbox_a = v,
        Message::CheckboxBToggled(v) => state.checkbox_b = v,
        Message::CheckboxCToggled(v) => state.checkbox_c = v,
        Message::FruitSelected(fruit) => state.selected_fruit = Some(fruit),
        Message::TogglerToggled(v) => state.toggler_enabled = v,
        Message::PickListSelected(v) => state.pick_list_selected = Some(v),
        Message::ComboBoxSelected(v) => state.combo_selected = Some(v),
        Message::SliderChanged(v) => state.slider_value = v,
        Message::StepSliderChanged(v) => state.slider_step = v,
        Message::VSliderChanged(v) => state.vslider_value = v,
        Message::ProgressChanged(v) => state.progress_value = v,
        Message::IconSetSelected(choice) => {
            state.loaded_icons =
                load_all_icons(&choice, &state.current_resolved, state.current_icon_set);

            // Rebuild animation caches when icon set changes
            let anim_set = choice.effective_icon_set(state.current_icon_set);
            let (af, afi, afe, asp, astart, rm, ast) = build_animation_caches(anim_set);
            state.icon_set_choice = choice;
            state.animated_frames = af;
            state.animated_frame_indices = afi;
            state.animated_frame_elapsed = afe;
            state.animated_spins = asp;
            state.animation_start = astart;
            state.reduced_motion = rm;
            state.animated_static = ast;
        }
        Message::ThemeWatcherTick if state.theme_change_flag.swap(false, Ordering::AcqRel) => {
            native_theme::detect::invalidate_caches();
            state.rebuild_theme();
        }
        Message::AnimationTick => {
            let tick_duration = Duration::from_millis(50);
            for (i, (_, anim_handles)) in state.animated_frames.iter().enumerate() {
                state.animated_frame_elapsed[i] += tick_duration;
                let frame_dur = Duration::from_millis(anim_handles.frame_duration_ms as u64);
                if state.animated_frame_elapsed[i] >= frame_dur {
                    state.animated_frame_elapsed[i] -= frame_dur;
                    state.animated_frame_indices[i] =
                        (state.animated_frame_indices[i] + 1) % anim_handles.handles.len();
                }
            }
        }
        _ => {} // Screenshot messages handled by update()
    }
}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

fn view(state: &State) -> Element<'_, Message> {
    let radius = native_theme_iced::border_radius(&state.current_resolved);
    let sb_width = native_theme_iced::scrollbar_width(&state.current_resolved);
    let btn_pad = native_theme_iced::button_padding(&state.current_resolved);
    let inp_pad = native_theme_iced::input_padding(&state.current_resolved);

    // ---- Left sidebar ----
    let sidebar = {
        let sp = &SP;
        let ts = &state.current_resolved.text_scale;
        let title = text("native-theme").size(ts.dialog_title.size);
        let subtitle =
            text(format!("iced showcase v{}", env!("CARGO_PKG_VERSION"))).size(ts.caption.size);

        // Theme selector
        let theme_section = column![
            text("Theme Selector").size(ts.caption.size),
            pick_list(
                theme_choices(&state.default_label),
                Some(&state.current_choice),
                Message::ThemeSelected,
            )
            .width(Fill),
        ]
        .spacing(sp.xs);

        // Color mode selector (System / Light / Dark)
        let color_mode_section = column![
            text("Color Mode").size(ts.caption.size),
            pick_list(
                AppColorMode::ALL.to_vec(),
                Some(&state.color_mode),
                Message::ColorModeSelected,
            )
            .width(Fill),
        ]
        .spacing(sp.xs);

        // Icon theme selector
        let icon_theme_section = column![
            text("Icon Theme").size(ts.caption.size),
            pick_list(
                state.icon_set_choices.clone(),
                Some(&state.icon_set_choice),
                Message::IconSetSelected,
            )
            .width(Fill),
        ]
        .spacing(sp.xs);

        // Theme config inspector (matches gpui sidebar)
        let fi = format_font_info(&state.current_resolved);
        let metrics_info = {
            let r = format!("radius: {radius:.0}px");
            let rlg = format!(
                "radius_lg: {:.0}px",
                native_theme_iced::border_radius_lg(&state.current_resolved)
            );
            let sw = format!("scrollbar: {sb_width:.0}px");
            let bp = format!("btn pad: {:.0}\u{00d7}{:.0}", btn_pad.left, btn_pad.top);
            let ip = format!("input pad: {:.0}\u{00d7}{:.0}", inp_pad.left, inp_pad.top);
            column![
                text("Theme Config Inspector").size(ts.caption.size),
                text(r).size(ts.caption.size),
                text(rlg).size(ts.caption.size),
                text(sw).size(ts.caption.size),
                text(bp).size(ts.caption.size),
                text(ip).size(ts.caption.size),
                text(fi).size(ts.caption.size),
            ]
            .spacing(sp.xxs)
        };

        // Widget Info panel
        let widget_info_panel = {
            let info_text = if state.widget_info.is_empty() {
                "Hover over any widget to see its theme properties.".to_string()
            } else {
                state.widget_info.clone()
            };
            column![
                text("Widget Info").size(ts.caption.size),
                container(scrollable(text(info_text).size(ts.caption.size)).direction(
                    scrollable::Direction::Vertical(
                        scrollable::Scrollbar::new().width(4).scroller_width(4),
                    )
                ),)
                .padding(Padding::from(sp.s))
                .style(container::rounded_box)
                .width(Fill)
                .height(Fill),
            ]
            .spacing(sp.xs)
            .height(Fill)
        };

        container(
            scrollable(
                column![
                    title,
                    subtitle,
                    rule::horizontal(1),
                    theme_section,
                    color_mode_section,
                    rule::horizontal(1),
                    icon_theme_section,
                    rule::horizontal(1),
                    metrics_info,
                    rule::horizontal(1),
                    widget_info_panel,
                ]
                .spacing(sp.s)
                .padding(Padding::from(sp.m))
                .width(Length::Fixed(210.0)),
            )
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new().width(4).scroller_width(4),
            )),
        )
        .style(container::rounded_box)
        .height(Fill)
    };

    // ---- Tab bar ----
    let tab_bar: Element<'_, Message> = {
        let sp = &SP;
        let ts = &state.current_resolved.text_scale;
        let tabs: Vec<Element<'_, Message>> = Tab::ALL
            .iter()
            .map(|&tab| {
                let label = tab.label();
                let btn = button(text(label).size(ts.caption.size));
                let btn = if tab == state.active_tab {
                    btn.style(button::primary)
                } else {
                    btn.style(button::secondary)
                };
                btn.on_press(Message::TabSelected(tab))
                    .padding(Padding::from([sp.xs, sp.m]))
                    .into()
            })
            .collect();
        row(tabs).spacing(sp.xs).into()
    };

    // ---- Tab content ----
    let tab_content: Element<'_, Message> = match state.active_tab {
        Tab::Buttons => view_buttons(state, btn_pad),
        Tab::TextInputs => view_text_inputs(state, radius, inp_pad),
        Tab::Selection => view_selection(state),
        Tab::Range => view_range(state),
        Tab::Display => view_display(state, radius),
        Tab::Icons => view_icons(state),
        Tab::ThemeMap => view_theme_map(state),
    };

    // ---- Right panel (tabs + content) ----
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let tab_padding = Padding::ZERO.left(sp.l).right(sp.l).top(sp.s);
    let content_padding = Padding::from(sp.l);
    let panel_spacing = sp.xs;
    let mut right_panel = column![].spacing(panel_spacing).width(Fill).height(Fill);

    // Error banner (if any)
    if let Some(ref msg) = state.error_message {
        let danger = state.current_theme.palette().danger;
        right_panel = right_panel.push(
            container(text(msg.as_str()).color(danger).size(ts.caption.size))
                .padding(
                    Padding::ZERO
                        .top(sp.xs)
                        .bottom(sp.xs)
                        .left(sp.s)
                        .right(sp.s),
                )
                .width(Fill),
        );
    }

    let right_panel = right_panel
        .push(
            // Tab bar
            container(tab_bar).padding(tab_padding),
        )
        .push(rule::horizontal(1))
        .push(
            // Scrollable content
            scrollable(container(tab_content).padding(content_padding).width(Fill))
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(sb_width)
                        .scroller_width(sb_width),
                ))
                .height(Fill),
        );

    row![sidebar, right_panel].into()
}

// ---------------------------------------------------------------------------
// Hover helper: wraps a widget in mouse_area for Widget Info updates
// ---------------------------------------------------------------------------

fn hoverable<'a>(info: String, content: Element<'a, Message>) -> Element<'a, Message> {
    mouse_area(content)
        .on_enter(Message::WidgetHovered(info))
        .on_exit(Message::WidgetUnhovered)
        .into()
}

// ---------------------------------------------------------------------------
// Widget Info builder (matches gpui's widget_tooltip + widget_tooltip_themed)
// ---------------------------------------------------------------------------

/// Build a multi-line info string for the Widget Info panel.
///
/// Mirrors the gpui showcase's `widget_tooltip` with three sections:
/// - Theme colors: (role, field_name, live hex color)
/// - Theme config: (what, live_value_string)
/// - Not themeable: (what, reason why)
fn widget_tooltip(
    name: &str,
    colors: &[(&str, &str, Color)],
    config: &[(&str, &str)],
    not_themeable: &[(&str, &str)],
) -> String {
    let mut s = format!("{name}\n");

    if !colors.is_empty() {
        s.push_str("\nTheme colors:\n");
        for (role, field, val) in colors {
            s.push_str(&format!("  {role}: {field} {}\n", color_to_hex(*val)));
        }
    }

    if !config.is_empty() {
        s.push_str("\nTheme config:\n");
        for (what, val) in config {
            s.push_str(&format!("  {what}: {val}\n"));
        }
    }

    if !not_themeable.is_empty() {
        s.push_str("\nNot themeable:\n");
        for (what, why) in not_themeable {
            s.push_str(&format!("  {what}: {why}\n"));
        }
    }

    s
}

/// Format the resolved theme font settings for display.
fn format_font_info(resolved: &native_theme::theme::ResolvedTheme) -> String {
    let ff = &resolved.defaults.font.family;
    let fs = format!("{:.0}px", resolved.defaults.font.size);
    let mf = &resolved.defaults.mono_font.family;
    let ms = format!("{:.0}px", resolved.defaults.mono_font.size);
    format!("Font: {ff} {fs}  Mono: {mf} {ms}")
}

/// Like [`widget_tooltip`] but appends the active theme font settings.
fn widget_tooltip_themed(
    state: &State,
    name: &str,
    colors: &[(&str, &str, Color)],
    config: &[(&str, &str)],
    not_themeable: &[(&str, &str)],
) -> String {
    let mut s = widget_tooltip(name, colors, config, not_themeable);
    let ff = &state.current_resolved.defaults.font.family;
    let fs = format!("{:.0}px", state.current_resolved.defaults.font.size);
    let mf = &state.current_resolved.defaults.mono_font.family;
    let ms = format!("{:.0}px", state.current_resolved.defaults.mono_font.size);
    s.push_str(&format!(
        "\nTheme fonts:\n  Font: {ff} {fs}\n  Mono: {mf} {ms}\n"
    ));
    s
}

// ---------------------------------------------------------------------------
// Tab: Buttons
// ---------------------------------------------------------------------------

fn view_buttons<'a>(state: &'a State, btn_pad: Padding) -> Element<'a, Message> {
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let palette = state.current_theme.palette();
    let ext = state.current_theme.extended_palette();
    let radius = native_theme_iced::border_radius(&state.current_resolved);
    let radius_s = format!("{radius:.0}px");

    let apply_pad =
        |b: button::Button<'a, Message>| -> button::Button<'a, Message> { b.padding(btn_pad) };

    let header = section_header(
        "Buttons",
        "Interactive button styles from the theme palette",
        ts,
        sp,
    );

    let primary_row = hoverable(
        widget_tooltip_themed(
            state,
            "Button (Primary)",
            &[
                ("bg", "primary", palette.primary),
                ("text", "primary.base.text", ext.primary.base.text),
                ("hover bg", "primary.strong", ext.primary.strong.color),
            ],
            &[
                ("border-radius", &radius_s),
                ("shadow", if state.is_dark { "none" } else { "subtle" }),
            ],
            &[
                ("padding", "set by iced per widget instance"),
                ("font-weight", "hardcoded"),
                ("min-height", "hardcoded by iced"),
            ],
        ),
        column![
            text("Primary Actions").size(ts.dialog_title.size),
            row![
                apply_pad(
                    button("Primary")
                        .on_press(Message::ButtonPressed)
                        .style(button::primary)
                ),
                apply_pad(
                    button("Secondary")
                        .on_press(Message::ButtonPressed)
                        .style(button::secondary)
                ),
                apply_pad(
                    button("Success")
                        .on_press(Message::ButtonPressed)
                        .style(button::success)
                ),
                apply_pad(
                    button("Danger")
                        .on_press(Message::ButtonPressed)
                        .style(button::danger)
                ),
                apply_pad(
                    button("Text Style")
                        .on_press(Message::ButtonPressed)
                        .style(button::text)
                ),
            ]
            .spacing(sp.s),
        ]
        .spacing(sp.s)
        .into(),
    );

    let disabled_row = hoverable(
        widget_tooltip(
            "Disabled Buttons",
            &[],
            &[],
            &[
                ("opacity", "reduced when disabled (no on_press)"),
                ("cursor", "not interactive"),
                ("theme", "same variant colors at reduced opacity"),
            ],
        ),
        column![
            text("Disabled State").size(ts.dialog_title.size),
            text("Buttons without on_press are rendered as disabled:")
                .size(ts.section_heading.size),
            row![
                apply_pad(button("Disabled Primary").style(button::primary)),
                apply_pad(button("Disabled Secondary").style(button::secondary)),
                apply_pad(button("Disabled Danger").style(button::danger)),
            ]
            .spacing(sp.s),
        ]
        .spacing(sp.s)
        .into(),
    );

    let counter_text = format!("Button presses this session: {}", state.button_press_count);

    let interactive = column![
        text("Interactive Demo").size(ts.dialog_title.size),
        row![
            apply_pad(
                button(text("Click me!").size(ts.section_heading.size))
                    .on_press(Message::ButtonPressed)
                    .style(button::primary)
            ),
            text(counter_text).size(ts.section_heading.size),
        ]
        .spacing(sp.m)
        .align_y(iced::Center),
    ]
    .spacing(sp.s);

    column![
        header,
        primary_row,
        rule::horizontal(1),
        disabled_row,
        rule::horizontal(1),
        interactive,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Text Inputs
// ---------------------------------------------------------------------------

fn view_text_inputs<'a>(state: &'a State, radius: f32, inp_pad: Padding) -> Element<'a, Message> {
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let palette = state.current_theme.palette();
    let ext = state.current_theme.extended_palette();
    let radius_s = format!("{radius:.0}px");

    let header = section_header(
        "Text Inputs",
        "Single-line TextInput and multi-line TextEditor",
        ts,
        sp,
    );

    let single_line = {
        let mut input = text_input("Type something here...", &state.text_input_value)
            .on_input(Message::TextInputChanged);
        {
            input = input.padding(inp_pad);
        }

        hoverable(
            widget_tooltip_themed(
                state,
                "TextInput",
                &[
                    ("border", "background.strong", ext.background.strong.color),
                    ("bg", "background", palette.background),
                    ("text", "text", palette.text),
                    (
                        "placeholder",
                        "background.strong.text",
                        ext.background.strong.text,
                    ),
                ],
                &[("border-radius", &radius_s)],
                &[
                    ("padding", "set per widget instance"),
                    ("height", "set by iced"),
                ],
            ),
            column![
                text("TextInput (single line)").size(ts.dialog_title.size),
                input,
                text(format!(
                    "Characters: {}  |  Border radius from theme: {radius:.0}px",
                    state.text_input_value.len()
                ))
                .size(ts.caption.size),
            ]
            .spacing(sp.s)
            .into(),
        )
    };

    let secure_input = {
        let mut input = text_input("Password field...", &state.text_input_value)
            .on_input(Message::TextInputChanged)
            .secure(true);
        {
            input = input.padding(inp_pad);
        }

        hoverable(
            widget_tooltip(
                "TextInput (secure)",
                &[
                    ("border", "background.strong", ext.background.strong.color),
                    ("bg", "background", palette.background),
                ],
                &[("border-radius", &radius_s)],
                &[("mode", "password / secure — dots replace chars")],
            ),
            column![
                text("TextInput (secure / password)").size(ts.dialog_title.size),
                input,
            ]
            .spacing(sp.s)
            .into(),
        )
    };

    let multi_line = hoverable(
        widget_tooltip_themed(
            state,
            "TextEditor (multi-line)",
            &[
                ("bg", "background", palette.background),
                ("text", "text", palette.text),
                ("selection", "primary.weak", ext.primary.weak.color),
            ],
            &[("border-radius", &radius_s)],
            &[
                ("line numbers", "not built-in"),
                ("syntax highlighting", "requires iced_highlighter"),
            ],
        ),
        column![
            text("TextEditor (multi-line)").size(ts.dialog_title.size),
            text_editor(&state.text_editor_content)
                .on_action(Message::EditorAction)
                .height(Length::Fixed(180.0)),
            text("Supports multi-line editing, selection, and scrolling").size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    column![
        header,
        single_line,
        rule::horizontal(1),
        secure_input,
        rule::horizontal(1),
        multi_line,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Selection
// ---------------------------------------------------------------------------

fn view_selection(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let palette = state.current_theme.palette();
    let ext = state.current_theme.extended_palette();
    let radius = native_theme_iced::border_radius(&state.current_resolved);
    let radius_s = format!("{radius:.0}px");

    let header = section_header(
        "Selection Widgets",
        "Checkbox, Radio, Toggler, PickList, and ComboBox",
        ts,
        sp,
    );

    let checkboxes = hoverable(
        widget_tooltip(
            "Checkbox",
            &[
                ("checked bg", "primary", palette.primary),
                ("checkmark", "primary.base.text", ext.primary.base.text),
                (
                    "unchecked border",
                    "background.strong",
                    ext.background.strong.color,
                ),
                ("bg", "background", palette.background),
            ],
            &[("border-radius", &radius_s)],
            &[
                ("size", "hardcoded by iced"),
                ("indicator size", "hardcoded"),
            ],
        ),
        column![
            text("Checkboxes").size(ts.dialog_title.size),
            checkbox(state.checkbox_a)
                .label("Enable notifications")
                .on_toggle(Message::CheckboxAToggled),
            checkbox(state.checkbox_b)
                .label("Dark mode auto-detect")
                .on_toggle(Message::CheckboxBToggled),
            checkbox(state.checkbox_c)
                .label("Remember preferences")
                .on_toggle(Message::CheckboxCToggled),
            text(format!(
                "Checked: {}",
                [
                    state.checkbox_a.then_some("notifications"),
                    state.checkbox_b.then_some("auto-detect"),
                    state.checkbox_c.then_some("remember"),
                ]
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>()
                .join(", ")
            ))
            .size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    let radios = hoverable(
        widget_tooltip(
            "Radio",
            &[
                ("selected", "primary", palette.primary),
                (
                    "unselected border",
                    "background.strong",
                    ext.background.strong.color,
                ),
                ("bg", "background", palette.background),
            ],
            &[("border-radius", &radius_s)],
            &[("size", "hardcoded"), ("indicator size", "hardcoded")],
        ),
        column![
            text("Radio Buttons").size(ts.dialog_title.size),
            radio(
                "Apple",
                Fruit::Apple,
                state.selected_fruit,
                Message::FruitSelected
            ),
            radio(
                "Banana",
                Fruit::Banana,
                state.selected_fruit,
                Message::FruitSelected
            ),
            radio(
                "Cherry",
                Fruit::Cherry,
                state.selected_fruit,
                Message::FruitSelected
            ),
            text(format!(
                "Selected: {}",
                state
                    .selected_fruit
                    .map(|f| f.to_string())
                    .unwrap_or_else(|| "None".to_string())
            ))
            .size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    let togglers = hoverable(
        widget_tooltip(
            "Toggler (Switch)",
            &[
                ("active track", "primary", palette.primary),
                (
                    "inactive track",
                    "background.strong",
                    ext.background.strong.color,
                ),
                ("thumb", "background.base", ext.background.base.color),
            ],
            &[("border-radius", "pill (fully rounded)")],
            &[("size", "hardcoded"), ("animation timing", "hardcoded")],
        ),
        column![
            text("Toggler (Switch)").size(ts.dialog_title.size),
            toggler(state.toggler_enabled)
                .label("Feature flag enabled")
                .on_toggle(Message::TogglerToggled),
            text(format!(
                "State: {}",
                if state.toggler_enabled { "ON" } else { "OFF" }
            ))
            .size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    let languages: Vec<String> = vec![
        "Rust",
        "Python",
        "JavaScript",
        "TypeScript",
        "Go",
        "C++",
        "Java",
        "Swift",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let pickers = hoverable(
        widget_tooltip(
            "PickList (dropdown)",
            &[
                ("bg", "background", palette.background),
                ("text", "text", palette.text),
                ("border", "background.strong", ext.background.strong.color),
                ("selected", "primary", palette.primary),
            ],
            &[("border-radius", &radius_s)],
            &[("dropdown arrow", "hardcoded chevron")],
        ),
        column![
            text("PickList (dropdown)").size(ts.dialog_title.size),
            pick_list(
                languages,
                state.pick_list_selected.as_ref(),
                Message::PickListSelected,
            )
            .width(Length::Fixed(250.0)),
            text(format!(
                "Selected: {}",
                state.pick_list_selected.as_deref().unwrap_or("None")
            ))
            .size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    let combos = hoverable(
        widget_tooltip(
            "ComboBox (searchable dropdown)",
            &[
                ("bg", "background", palette.background),
                ("text", "text", palette.text),
                ("border", "background.strong", ext.background.strong.color),
            ],
            &[("border-radius", &radius_s)],
            &[("search", "built-in text filter")],
        ),
        column![
            text("ComboBox (searchable dropdown)").size(ts.dialog_title.size),
            combo_box(
                &state.combo_state,
                "Search a language...",
                state.combo_selected.as_ref(),
                Message::ComboBoxSelected,
            )
            .width(Length::Fixed(250.0)),
            text(format!(
                "Selected: {}",
                state.combo_selected.as_deref().unwrap_or("None")
            ))
            .size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    column![
        header,
        row![
            column![checkboxes, rule::horizontal(1), togglers,]
                .spacing(sp.xl)
                .width(Fill),
            rule::vertical(1),
            column![
                radios,
                rule::horizontal(1),
                pickers,
                rule::horizontal(1),
                combos,
            ]
            .spacing(sp.xl)
            .width(Fill),
        ]
        .spacing(sp.xl),
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Range
// ---------------------------------------------------------------------------

fn view_range(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let palette = state.current_theme.palette();
    let ext = state.current_theme.extended_palette();

    let header = section_header(
        "Range Widgets",
        "Slider, VerticalSlider, and ProgressBar",
        ts,
        sp,
    );

    let horiz_slider = hoverable(
        widget_tooltip(
            "Horizontal Slider",
            &[
                ("active track", "primary", palette.primary),
                (
                    "inactive track",
                    "background.strong",
                    ext.background.strong.color,
                ),
                ("handle", "primary.strong", ext.primary.strong.color),
            ],
            &[],
            &[("track height", "hardcoded"), ("thumb size", "hardcoded")],
        ),
        column![
            text("Horizontal Slider").size(ts.dialog_title.size),
            row![
                slider(0.0..=100.0, state.slider_value, Message::SliderChanged).width(Fill),
                text(format!("{:.1}", state.slider_value))
                    .size(ts.section_heading.size)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(sp.m)
            .align_y(iced::Center),
            text("Drag to change value. This slider drives the first progress bar below.")
                .size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    let step_slider = hoverable(
        widget_tooltip(
            "Slider (stepped)",
            &[("track", "primary", palette.primary)],
            &[("step", "5.0")],
            &[("snap behavior", "hardcoded step increments")],
        ),
        column![
            text("Slider with Step (5-unit increments)").size(ts.dialog_title.size),
            row![
                slider(0.0..=100.0, state.slider_step, Message::StepSliderChanged)
                    .step(5.0)
                    .width(Fill),
                text(format!("{:.0}", state.slider_step))
                    .size(ts.section_heading.size)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(sp.m)
            .align_y(iced::Center),
        ]
        .spacing(sp.s)
        .into(),
    );

    let vert_slider = hoverable(
        widget_tooltip(
            "Vertical Slider",
            &[("track", "primary", palette.primary)],
            &[("orientation", "vertical")],
            &[("track width", "hardcoded"), ("thumb size", "hardcoded")],
        ),
        column![
            text("Vertical Slider").size(ts.dialog_title.size),
            row![
                container(
                    vertical_slider(0.0..=100.0, state.vslider_value, Message::VSliderChanged)
                        .height(Length::Fixed(200.0))
                )
                .center_x(Length::Fixed(60.0)),
                column![
                    text(format!("Value: {:.1}", state.vslider_value))
                        .size(ts.section_heading.size),
                    space().height(Length::Fixed(8.0)),
                    text("Vertical sliders are useful\nfor volume controls,\nequalizers, etc.")
                        .size(ts.caption.size),
                ]
                .spacing(sp.xs),
            ]
            .spacing(sp.l),
        ]
        .spacing(sp.s)
        .into(),
    );

    let progress = hoverable(
        widget_tooltip(
            "Progress Bar",
            &[
                ("fill", "primary", palette.primary),
                ("track bg", "background.strong", ext.background.strong.color),
            ],
            &[],
            &[("height", "hardcoded"), ("animation", "none — immediate")],
        ),
        column![
            text("Progress Bars").size(ts.dialog_title.size),
            text("Driven by horizontal slider value:").size(ts.section_heading.size),
            progress_bar(0.0..=100.0, state.slider_value),
            space().height(Length::Fixed(4.0)),
            text("Separate progress control:").size(ts.section_heading.size),
            row![
                slider(0.0..=100.0, state.progress_value, Message::ProgressChanged).width(Fill),
                text(format!("{:.0}%", state.progress_value))
                    .size(ts.section_heading.size)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(sp.m)
            .align_y(iced::Center),
            progress_bar(0.0..=100.0, state.progress_value),
        ]
        .spacing(sp.s)
        .into(),
    );

    column![
        header,
        horiz_slider,
        rule::horizontal(1),
        step_slider,
        rule::horizontal(1),
        vert_slider,
        rule::horizontal(1),
        progress,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Display
// ---------------------------------------------------------------------------

fn view_display<'a>(state: &'a State, radius: f32) -> Element<'a, Message> {
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let ext = state.current_theme.extended_palette();
    let radius_s = format!("{radius:.0}px");

    let header = section_header(
        "Display Widgets",
        "Container, Rule, Tooltip, and layout helpers",
        ts,
        sp,
    );

    let containers = hoverable(
        widget_tooltip(
            "Styled Containers (rounded_box)",
            &[
                ("bg", "background.weak", ext.background.weak.color),
                ("text", "background.weak.text", ext.background.weak.text),
                ("border", "background.strong", ext.background.strong.color),
            ],
            &[("border-radius", &radius_s)],
            &[("padding", "set per widget instance")],
        ),
        column![
            text("Styled Containers").size(ts.dialog_title.size),
            container(
                column![
                    text("Rounded Box Container").size(ts.section_heading.size),
                    text(format!(
                        "This container uses the theme's rounded_box style. \
                         Border radius from theme metrics: {radius:.0}px."
                    ))
                    .size(ts.caption.size),
                ]
                .spacing(sp.xs),
            )
            .padding(Padding::from(sp.l))
            .style(container::rounded_box)
            .width(Fill),
            container(
                text(
                    "A secondary container with different padding. Containers adapt their \
                      background and border colors from the active theme palette."
                )
                .size(ts.caption.size),
            )
            .padding(Padding::from([sp.m, sp.xl]))
            .style(container::rounded_box)
            .width(Fill),
        ]
        .spacing(sp.m)
        .into(),
    );

    let rules = column![
        text("Divider Rules").size(ts.dialog_title.size),
        text("Horizontal rules at various thicknesses:").size(ts.section_heading.size),
        rule::horizontal(1),
        text("1px above, 2px below").size(ts.caption.size),
        rule::horizontal(2),
        text("2px above, 4px below").size(ts.caption.size),
        rule::horizontal(4),
    ]
    .spacing(sp.s);

    let tooltips = hoverable(
        widget_tooltip(
            "Tooltip",
            &[
                ("bg", "background.weak", ext.background.weak.color),
                ("text", "background.weak.text", ext.background.weak.text),
                ("border", "background.strong", ext.background.strong.color),
            ],
            &[("positions", "Top / Bottom / Left / Right")],
            &[("gap", "set per widget instance"), ("delay", "none")],
        ),
        column![
            text("Tooltips").size(ts.dialog_title.size),
            row![
                tooltip(
                    button("Hover: Top")
                        .on_press(Message::ButtonPressed)
                        .style(button::primary),
                    text("Tooltip on top!"),
                    tooltip::Position::Top,
                )
                .gap(sp.xs)
                .style(container::rounded_box),
                tooltip(
                    button("Hover: Bottom")
                        .on_press(Message::ButtonPressed)
                        .style(button::secondary),
                    text("Tooltip on bottom!"),
                    tooltip::Position::Bottom,
                )
                .gap(sp.xs)
                .style(container::rounded_box),
                tooltip(
                    button("Hover: Left")
                        .on_press(Message::ButtonPressed)
                        .style(button::success),
                    text("Tooltip on left!"),
                    tooltip::Position::Left,
                )
                .gap(sp.xs)
                .style(container::rounded_box),
                tooltip(
                    button("Hover: Right")
                        .on_press(Message::ButtonPressed)
                        .style(button::danger),
                    text("Tooltip on right!"),
                    tooltip::Position::Right,
                )
                .gap(sp.xs)
                .style(container::rounded_box),
            ]
            .spacing(sp.m),
        ]
        .spacing(sp.s)
        .into(),
    );

    let theme_info_text = format!(
        "Active theme: {}  |  Mode: {}",
        state.current_theme,
        if state.is_dark { "Dark" } else { "Light" },
    );

    let font_info = {
        let ff = native_theme_iced::font_family(&state.current_resolved);
        let fs = format!(
            "{:.1}px",
            native_theme_iced::font_size(&state.current_resolved)
        );
        let mf = native_theme_iced::mono_font_family(&state.current_resolved);
        let ms = format!(
            "{:.1}px",
            native_theme_iced::mono_font_size(&state.current_resolved)
        );
        format!("Font: {ff} @ {fs}  |  Mono: {mf} @ {ms}")
    };

    let info_box = container(
        column![
            text("Theme Information").size(ts.section_heading.size),
            text(theme_info_text).size(ts.caption.size),
            text(font_info).size(ts.caption.size),
            text(format!(
                "Available presets: {} | All presets have both light and dark variants.",
                native_theme::theme::Theme::list_presets().len(),
            ))
            .size(ts.caption.size),
        ]
        .spacing(sp.xs),
    )
    .padding(Padding::from(sp.l))
    .style(container::rounded_box)
    .width(Fill);

    let spacing_demo = column![
        text("Spacing & Layout").size(ts.dialog_title.size),
        row![
            container(text("A").size(ts.section_heading.size))
                .padding(Padding::from(sp.m))
                .style(container::rounded_box)
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            container(text("B").size(ts.section_heading.size))
                .padding(Padding::from(sp.m))
                .style(container::rounded_box)
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            container(text("C").size(ts.section_heading.size))
                .padding(Padding::from(sp.m))
                .style(container::rounded_box)
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            space().width(Fill),
            container(text("Right-aligned").size(ts.caption.size))
                .padding(Padding::from(sp.m))
                .style(container::rounded_box),
        ]
        .spacing(sp.s)
        .align_y(iced::Center),
    ]
    .spacing(sp.s);

    column![
        header,
        containers,
        rule::horizontal(1),
        rules,
        rule::horizontal(1),
        tooltips,
        rule::horizontal(1),
        spacing_demo,
        rule::horizontal(1),
        info_box,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Icons
// ---------------------------------------------------------------------------

fn view_icons(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let loaded_count = state
        .loaded_icons
        .iter()
        .filter(|i| i.data.is_some())
        .count();
    let system_count = state
        .loaded_icons
        .iter()
        .filter(|i| i.source == IconSource::System)
        .count();
    let fallback_count = state
        .loaded_icons
        .iter()
        .filter(|i| i.source == IconSource::Fallback)
        .count();
    let total_count = state.loaded_icons.len();

    let header = column![
        text("Icons").size(ts.display.size),
        text(format!(
            "All {total_count} IconRole variants — \
             {loaded_count} loaded, {system_count} system, {fallback_count} fallback"
        ))
        .size(ts.section_heading.size),
        rule::horizontal(2),
    ]
    .spacing(sp.xs);

    let icon_set_info = column![
        text(format!("Active icon set: {}", state.icon_set_choice)).size(ts.section_heading.size),
        text(format!(
            "System icon theme: {}",
            native_theme::theme::system_icon_theme()
        ))
        .size(ts.caption.size),
    ]
    .spacing(sp.xs);

    // Use the theme's foreground color for colorizing icons
    let fg_color = state.current_theme.palette().text;

    // Build grid rows of 6 icons each
    let icons_per_row = 6;
    let mut grid_rows: Vec<Element<'_, Message>> = Vec::new();
    let mut idx = 0;
    while idx < state.loaded_icons.len() {
        let end = (idx + icons_per_row).min(state.loaded_icons.len());
        let row_icons: Vec<Element<'_, Message>> = state.loaded_icons[idx..end]
            .iter()
            .map(|loaded| {
                build_icon_cell(
                    loaded,
                    fg_color,
                    ts.caption.size,
                    ts.section_heading.size,
                    sp.xxs,
                )
            })
            .collect();
        grid_rows.push(row(row_icons).spacing(sp.s).into());
        idx = end;
    }

    let animated_section = view_animated_icons(state, fg_color);
    let mut content = column![
        header,
        icon_set_info,
        rule::horizontal(1),
        animated_section,
        rule::horizontal(1)
    ]
    .spacing(sp.l);
    for r in grid_rows {
        content = content.push(r);
    }

    content.width(Fill).into()
}

fn view_animated_icons<'a>(state: &'a State, fg_color: Color) -> Element<'a, Message> {
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let section_title = text("Animated Icons").size(ts.display.size);
    let divider = rule::horizontal(2);

    // Collect spinner columns into a row
    let mut spinners: Vec<Element<'a, Message>> = Vec::new();

    if state.reduced_motion {
        // Reduced motion: show static first-frame for each animated icon
        for (set_name, handle) in &state.animated_static {
            let icon = svg(handle.clone())
                .width(Length::Fixed(32.0))
                .height(Length::Fixed(32.0))
                .style(move |_theme, _status| iced::widget::svg::Style {
                    color: Some(fg_color),
                });
            let label =
                text(format!("{} - Static (reduced motion)", set_name)).size(ts.caption.size);
            spinners.push(
                column![icon, label]
                    .spacing(sp.xs)
                    .align_x(iced::Center)
                    .into(),
            );
        }
    } else {
        // Frame-based animations
        for (i, (set_name, anim_handles)) in state.animated_frames.iter().enumerate() {
            let frame_idx = state.animated_frame_indices[i];
            let icon = svg(anim_handles.handles[frame_idx].clone())
                .width(Length::Fixed(32.0))
                .height(Length::Fixed(32.0))
                .style(move |_theme, _status| iced::widget::svg::Style {
                    color: Some(fg_color),
                });
            let label = text(format!(
                "{} - Frames: {} ({}ms)",
                set_name,
                anim_handles.handles.len(),
                anim_handles.frame_duration_ms,
            ))
            .size(ts.caption.size);
            spinners.push(
                column![icon, label]
                    .spacing(sp.xs)
                    .align_x(iced::Center)
                    .into(),
            );
        }

        // Spin-based animations
        for (set_name, handle, duration_ms) in &state.animated_spins {
            let angle = spin_rotation_radians(state.animation_start.elapsed(), *duration_ms);
            let icon = svg(handle.clone())
                .width(Length::Fixed(32.0))
                .height(Length::Fixed(32.0))
                .rotation(iced::Rotation::Floating(angle))
                .style(move |_theme, _status| iced::widget::svg::Style {
                    color: Some(fg_color),
                });
            let label =
                text(format!("{} - Spin ({}ms)", set_name, duration_ms)).size(ts.caption.size);
            spinners.push(
                column![icon, label]
                    .spacing(sp.xs)
                    .align_x(iced::Center)
                    .into(),
            );
        }
    }

    let mut content = column![section_title, divider].spacing(sp.s);

    if state.reduced_motion {
        content = content
            .push(text("prefers-reduced-motion: showing static frames").size(ts.caption.size));
    }

    if spinners.is_empty() {
        content = content.push(
            text("No animated icons available for this configuration.").size(ts.caption.size),
        );
    } else {
        content = content.push(row(spinners).spacing(sp.xl));
    }

    content.into()
}

fn build_icon_cell<'a>(
    loaded: &LoadedIcon,
    fg_color: Color,
    caption_size: f32,
    heading_size: f32,
    xxs_spacing: f32,
) -> Element<'a, Message> {
    let role_name = format!("{:?}", loaded.role);
    let icon_name_str = loaded.name.unwrap_or("(unmapped)");
    let source_label = loaded.source.label();

    let icon_element: Element<'a, Message> = match &loaded.data {
        Some(data @ IconData::Svg(_)) => {
            if loaded.source == IconSource::System {
                // System icons: render as-is without colorization
                match native_theme_iced::icons::to_svg_handle(data, None) {
                    Some(handle) => svg(handle)
                        .width(Length::Fixed(24.0))
                        .height(Length::Fixed(24.0))
                        .into(),
                    None => placeholder_icon(heading_size),
                }
            } else {
                // Bundled/fallback: colorize with theme foreground
                match native_theme_iced::icons::to_svg_handle(data, Some(fg_color)) {
                    Some(handle) => svg(handle)
                        .width(Length::Fixed(24.0))
                        .height(Length::Fixed(24.0))
                        .into(),
                    None => placeholder_icon(heading_size),
                }
            }
        }
        Some(data @ IconData::Rgba { .. }) => {
            match native_theme_iced::icons::to_image_handle(data) {
                Some(handle) => iced::widget::image(handle)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
                    .into(),
                None => placeholder_icon(heading_size),
            }
        }
        _ => placeholder_icon(heading_size),
    };

    let info = format!("{role_name}\nicon: {icon_name_str}\nsource: {source_label}");

    // Wrap in mouse_area for Widget Info hover
    mouse_area(
        container(
            column![
                container(icon_element)
                    .center_x(Length::Fixed(32.0))
                    .center_y(Length::Fixed(32.0)),
                text(role_name.clone()).size(caption_size),
                text(source_label).size(caption_size),
            ]
            .spacing(xxs_spacing)
            .align_x(iced::Center),
        )
        .padding(Padding::from(6))
        .style(container::rounded_box)
        .width(Length::Fixed(100.0)),
    )
    .on_enter(Message::WidgetHovered(info))
    .on_exit(Message::WidgetUnhovered)
    .into()
}

fn placeholder_icon<'a>(size: f32) -> Element<'a, Message> {
    container(text("?").size(size))
        .center_x(Length::Fixed(24.0))
        .center_y(Length::Fixed(24.0))
        .into()
}

// ---------------------------------------------------------------------------
// Tab: Theme Map
// ---------------------------------------------------------------------------

fn view_theme_map(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let ts = &state.current_resolved.text_scale;
    let header = section_header(
        "Theme Map",
        "All palette and extended palette colors from the current theme",
        ts,
        sp,
    );

    let palette = state.current_theme.palette();
    let extended = state.current_theme.extended_palette();
    let swatch_border = native_theme_iced::border_color(&state.current_resolved);
    let swatch_bw = state.current_resolved.defaults.border.line_width;
    let swatch_r = native_theme_iced::border_radius(&state.current_resolved);
    // Local closure for concise swatch calls
    let caption_sz = ts.caption.size;
    let xxs_sp = sp.xxs;
    let cs = |label: &'static str, color: Color| -> Element<'_, Message> {
        color_swatch(
            label,
            color,
            swatch_border,
            swatch_bw,
            swatch_r,
            caption_sz,
            xxs_sp,
        )
    };

    let swatch_style = SwatchStyle {
        border_color: swatch_border,
        border_width: swatch_bw,
        radius: swatch_r,
        heading_size: ts.dialog_title.size,
        caption_size: ts.caption.size,
        xxs_spacing: sp.xxs,
        swatch_spacing: sp.m,
        column_spacing: sp.s,
    };

    // Base palette (6 colors)
    let base_palette = hoverable(
        widget_tooltip(
            "Base Palette (6 fields)",
            &[
                ("background", "background", palette.background),
                ("text", "text", palette.text),
                ("primary", "primary", palette.primary),
                ("success", "success", palette.success),
                ("warning", "warning", palette.warning),
                ("danger", "danger", palette.danger),
            ],
            &[],
            &[],
        ),
        column![
            text("Base Palette (6 fields)").size(ts.dialog_title.size),
            row![
                cs("background", palette.background),
                cs("text", palette.text),
                cs("primary", palette.primary),
                cs("success", palette.success),
                cs("warning", palette.warning),
                cs("danger", palette.danger),
            ]
            .spacing(sp.m),
        ]
        .spacing(sp.s)
        .into(),
    );

    // Extended palette sections (all 6 families use hoverable_ext_section)
    let ext_background = hoverable_ext_section(
        "Background (Extended)",
        extended.background.base,
        extended.background.weak,
        extended.background.strong,
        &swatch_style,
    );
    let ext_primary = hoverable_ext_section(
        "Primary (Extended)",
        extended.primary.base,
        extended.primary.weak,
        extended.primary.strong,
        &swatch_style,
    );
    let ext_secondary = hoverable_ext_section(
        "Secondary (Extended)",
        extended.secondary.base,
        extended.secondary.weak,
        extended.secondary.strong,
        &swatch_style,
    );
    let ext_success = hoverable_ext_section(
        "Success (Extended)",
        extended.success.base,
        extended.success.weak,
        extended.success.strong,
        &swatch_style,
    );
    let ext_warning = hoverable_ext_section(
        "Warning (Extended)",
        extended.warning.base,
        extended.warning.weak,
        extended.warning.strong,
        &swatch_style,
    );
    let ext_danger = hoverable_ext_section(
        "Danger (Extended)",
        extended.danger.base,
        extended.danger.weak,
        extended.danger.strong,
        &swatch_style,
    );

    // Resolved theme colors (defaults + selected per-widget)
    let native_colors = {
        let d = &state.current_resolved.defaults;
        let r = &state.current_resolved;
        let pairs: Vec<(&str, native_theme::color::Rgba)> = vec![
            ("accent", d.accent_color),
            ("background", d.background_color),
            ("text", d.text_color),
            ("surface", d.surface_color),
            ("border", d.border.color),
            ("muted", d.muted_color),
            ("shadow", d.shadow_color),
            ("accent_fg", d.accent_text_color),
            ("btn_bg", r.button.background_color),
            ("btn_fg", r.button.font.color),
            ("btn_primary", r.button.primary_background),
            ("danger", d.danger_color),
            ("danger_fg", d.danger_text_color),
            ("warning", d.warning_color),
            ("warning_fg", d.warning_text_color),
            ("success", d.success_color),
            ("success_fg", d.success_text_color),
            ("info", d.info_color),
            ("info_fg", d.info_text_color),
            ("selection", d.selection_background),
            ("selection_fg", d.selection_text_color),
            ("link", d.link_color),
            ("focus_ring", d.focus_ring_color),
            ("sidebar_bg", r.sidebar.background_color),
            ("sidebar_fg", r.sidebar.font.color),
            ("tooltip_bg", r.tooltip.background_color),
            ("tooltip_fg", r.tooltip.font.color),
            ("popover_bg", r.popover.background_color),
            ("popover_fg", r.popover.font.color),
            ("input_bg", r.input.background_color),
            ("input_fg", r.input.font.color),
            ("disabled_fg", d.disabled_text_color),
            ("separator", r.separator.line_color),
            ("alt_row", r.list.alternate_row_background),
            ("sel_inactive", d.selection_inactive_background),
            ("card_bg", r.card.background_color),
        ];

        // Wrap into rows of 6
        let mut rows: Vec<Element<'_, Message>> = Vec::new();
        let mut idx = 0;
        while idx < pairs.len() {
            let end = (idx + 6).min(pairs.len());
            let row_items: Vec<Element<'_, Message>> = pairs[idx..end]
                .iter()
                .map(|(name, rgba)| {
                    let [cr, cg, cb, ca] = rgba.to_f32_array();
                    color_swatch(
                        name,
                        Color::from_rgba(cr, cg, cb, ca),
                        swatch_border,
                        swatch_bw,
                        swatch_r,
                        ts.caption.size,
                        sp.xxs,
                    )
                })
                .collect();
            rows.push(row(row_items).spacing(sp.m).into());
            idx = end;
        }

        let mut col = column![
            text("Resolved Theme Colors (defaults + per-widget)").size(ts.dialog_title.size),
        ]
        .spacing(sp.s);
        for r in rows {
            col = col.push(r);
        }
        col
    };

    column![
        header,
        base_palette,
        rule::horizontal(1),
        ext_background,
        rule::horizontal(1),
        ext_primary,
        rule::horizontal(1),
        ext_secondary,
        rule::horizontal(1),
        ext_success,
        rule::horizontal(1),
        ext_warning,
        rule::horizontal(1),
        ext_danger,
        rule::horizontal(1),
        native_colors,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

fn color_to_hex(c: Color) -> String {
    let r = (c.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (c.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (c.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = c.a.clamp(0.0, 1.0);
    if (a - 1.0).abs() < f32::EPSILON {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        let a8 = (a * 255.0).round() as u8;
        format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a8)
    }
}

fn color_swatch<'a>(
    label: &'a str,
    color: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
    caption_size: f32,
    xxs_spacing: f32,
) -> Element<'a, Message> {
    let hex = color_to_hex(color);
    // Determine if text should be light or dark for contrast
    let luminance = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
    let text_color = if luminance > 0.5 {
        Color::BLACK
    } else {
        Color::WHITE
    };

    // Derive swatch padding from the theme spacing scale (xxs ~= 2px).
    // Vertical uses 3×xxs, horizontal uses 2×xxs — keeps the swatch compact
    // while scaling proportionally with the theme.
    let swatch_pad = Padding::new(0.0)
        .top(xxs_spacing * 3.0)
        .bottom(xxs_spacing * 3.0)
        .left(xxs_spacing * 2.0)
        .right(xxs_spacing * 2.0);

    column![
        container(text(hex.clone()).size(caption_size).color(text_color))
            .padding(swatch_pad)
            .style(move |_theme: &Theme| container::Style {
                background: Some(color.into()),
                border: iced::Border {
                    color: border_color,
                    width: border_width,
                    radius: radius.into(),
                },
                ..Default::default()
            })
            .center_x(Length::Fixed(80.0))
            .center_y(Length::Fixed(32.0)),
        text(label).size(caption_size),
    ]
    .spacing(xxs_spacing)
    .align_x(iced::Center)
    .into()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Swatch rendering parameters shared across extended palette sections.
struct SwatchStyle {
    border_color: Color,
    border_width: f32,
    radius: f32,
    heading_size: f32,
    caption_size: f32,
    xxs_spacing: f32,
    swatch_spacing: f32,
    column_spacing: f32,
}

/// Build an extended palette section wrapped in a hoverable tooltip.
///
/// Combines `widget_tooltip` + `ext_palette_section` + `hoverable` to eliminate
/// repetition in `view_theme_map`. Each call produces a complete hoverable section
/// with all 6 swatches (base/weak/strong x color/text) and a Widget Info tooltip.
fn hoverable_ext_section<'a>(
    label: &'a str,
    base: iced_core::theme::palette::Pair,
    weak: iced_core::theme::palette::Pair,
    strong: iced_core::theme::palette::Pair,
    style: &SwatchStyle,
) -> Element<'a, Message> {
    let SwatchStyle {
        border_color,
        border_width,
        radius,
        heading_size,
        caption_size,
        xxs_spacing,
        swatch_spacing,
        column_spacing,
    } = *style;
    let cs = |field: &'static str, color: Color| -> Element<'_, Message> {
        color_swatch(
            field,
            color,
            border_color,
            border_width,
            radius,
            caption_size,
            xxs_spacing,
        )
    };
    let info = widget_tooltip(
        label,
        &[
            ("base.color", "base.color", base.color),
            ("base.text", "base.text", base.text),
            ("weak.color", "weak.color", weak.color),
            ("weak.text", "weak.text", weak.text),
            ("strong.color", "strong.color", strong.color),
            ("strong.text", "strong.text", strong.text),
        ],
        &[],
        &[],
    );
    let content: Element<'a, Message> = column![
        text(label).size(heading_size),
        row![
            cs("base.color", base.color),
            cs("base.text", base.text),
            cs("weak.color", weak.color),
            cs("weak.text", weak.text),
            cs("strong.color", strong.color),
            cs("strong.text", strong.text),
        ]
        .spacing(swatch_spacing),
    ]
    .spacing(column_spacing)
    .into();
    hoverable(info, content)
}

fn section_header<'a>(
    title: &'a str,
    description: &'a str,
    ts: &native_theme::theme::ResolvedTextScale,
    sp: &Spacing,
) -> Element<'a, Message> {
    column![
        text(title).size(ts.display.size),
        text(description).size(ts.section_heading.size),
        rule::horizontal(2),
    ]
    .spacing(sp.xs)
    .into()
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

fn theme(state: &State) -> Theme {
    state.current_theme.clone()
}

// ---------------------------------------------------------------------------
// Subscription (animation tick)
// ---------------------------------------------------------------------------

fn subscription(state: &State) -> Subscription<Message> {
    let mut subs = vec![];

    // Animation tick (existing logic)
    if state.active_tab == Tab::Icons
        && !state.reduced_motion
        && (!state.animated_frames.is_empty() || !state.animated_spins.is_empty())
    {
        subs.push(iced::time::every(Duration::from_millis(50)).map(|_| Message::AnimationTick));
    }

    // Theme watcher: poll the atomic flag set by on_theme_change() callback.
    // Only active when color mode is System and the watcher started successfully.
    if matches!(state.color_mode, AppColorMode::System) && state._theme_watcher.is_some() {
        subs.push(iced::time::every(Duration::from_millis(500)).map(|_| Message::ThemeWatcherTick));
    }

    // Screenshot countdown timer
    if state.screenshot_path.is_some() && state.screenshot_countdown > 0 {
        subs.push(iced::time::every(Duration::from_millis(50)).map(|_| Message::ScreenshotTick));
    }

    Subscription::batch(subs)
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> iced::Result {
    // Parse CLI args and store globally before the iced application starts.
    // State::default() reads from CLI_ARGS to apply overrides.
    let _ = CLI_ARGS.set(CliArgs::parse());

    iced::application(State::default, update, view)
        .title(|_: &State| {
            format!(
                "Native Theme – Iced Showcase, v{}",
                env!("CARGO_PKG_VERSION")
            )
        })
        .theme(theme)
        .subscription(subscription)
        .window_size((1060.0, 750.0))
        .centered()
        .run()
}
