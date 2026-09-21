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
    button, canvas, checkbox, column, combo_box, container, grid, markdown, mouse_area, pane_grid,
    pick_list, progress_bar, qr_code, radio, row, rule, scrollable, slider, space, svg, table,
    text, text_editor, text_input, toggler, tooltip, vertical_slider,
};
use iced::{Color, Element, Fill, Length, Padding, Theme};
#[cfg(feature = "iced_aw")]
use iced_aw::menu::{Item, Menu, MenuBar};
#[cfg(feature = "iced_aw")]
use iced_aw::sidebar::Sidebar;
#[cfg(feature = "iced_aw")]
use iced_aw::{Card, ContextMenu, SelectionList, Spinner, TabBar, TabLabel, Tabs};

use iced::Subscription;
use native_theme::detect::prefers_reduced_motion;
use native_theme::icons::{
    FreedesktopLoader, IconSetChoice, LucideLoader, MaterialLoader, SegoeIconsLoader,
    SfSymbolsLoader, default_icon_choice, list_freedesktop_themes, load_icon_indicator,
};
use native_theme::theme::{
    AnimatedIcon, IconData, IconRole, IconSet, ResolvedTheme, TransformAnimation,
};
use native_theme_iced::icons::{
    AnimatedSvgHandles, animated_frames_to_svg_handles, spin_rotation_radians, to_svg_handle,
};
use native_theme_iced::palette::to_color;
use native_theme_iced::styles;
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
            "layout" => Some(Tab::Layout),
            "graphics" => Some(Tab::Graphics),
            #[cfg(feature = "iced_aw")]
            "extra" => Some(Tab::Extra),
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
    Layout,
    Graphics,
    /// The `iced_aw` widgets. Only when the connector's `iced_aw` feature is
    /// on, because `styles::aw` and the widgets themselves are behind it.
    #[cfg(feature = "iced_aw")]
    Extra,
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
        Tab::Layout,
        Tab::Graphics,
        #[cfg(feature = "iced_aw")]
        Tab::Extra,
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
            Tab::Layout => "Layout",
            Tab::Graphics => "Graphics",
            #[cfg(feature = "iced_aw")]
            Tab::Extra => "Extra widgets (iced_aw)",
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
    /// OS accessibility preferences (from SystemTheme, not ResolvedTheme).
    accessibility: native_theme_iced::AccessibilityPreferences,
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

    // Layout tab
    /// The `pane_grid`'s own layout state: splits, ratios and pane order.
    panes: pane_grid::State<usize>,
    /// The number the next pane created by a split is labelled with.
    pane_count: usize,
    /// The pane last clicked, dragged or created.
    focused_pane: Option<pane_grid::Pane>,

    // Graphics tab
    /// The parsed Markdown document the `markdown` section renders.
    markdown_content: markdown::Content,
    /// The last Markdown link clicked, echoed under the document.
    markdown_link: Option<String>,
    /// The encoded QR payload. `None` when the encoder rejects the data,
    /// which the section then says instead of showing a code.
    qr_data: Option<qr_code::Data>,

    // Extra widgets tab (iced_aw)
    /// The selected tab of the stand-alone `TabBar`.
    #[cfg(feature = "iced_aw")]
    aw_tab_bar_active: usize,
    /// The selected tab of the `Tabs` container, which owns its content.
    #[cfg(feature = "iced_aw")]
    aw_tabs_active: usize,
    /// The selected item of the `Sidebar`.
    #[cfg(feature = "iced_aw")]
    aw_sidebar_active: usize,
    /// The `SelectionList`'s options. It borrows them, so they live here.
    #[cfg(feature = "iced_aw")]
    aw_list_options: Vec<String>,
    /// The index the `SelectionList` has selected, if any.
    #[cfg(feature = "iced_aw")]
    aw_list_selected: Option<usize>,
    /// Whether the `Card` is on screen; its close button clears this.
    #[cfg(feature = "iced_aw")]
    aw_card_open: bool,
    /// The last menu or context-menu entry chosen, echoed in the tab.
    #[cfg(feature = "iced_aw")]
    aw_last_action: String,

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
        let (
            resolved,
            theme,
            initial_error,
            system_preset,
            init_icon_set,
            init_icon_theme,
            accessibility,
        ) = match native_theme::SystemTheme::from_system() {
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
                let acc = system.accessibility;
                (r, t, None, preset, is, it, acc)
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
                        native_theme_iced::AccessibilityPreferences::default(),
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

        // Two panes to start with, so the vertical split between them is
        // there to be dragged before anything is clicked.
        let (mut panes, first_pane) = pane_grid::State::new(1);
        let pane_count = if panes
            .split(pane_grid::Axis::Vertical, first_pane, 2)
            .is_some()
        {
            2
        } else {
            1
        };

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
            accessibility,
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
            panes,
            pane_count,
            focused_pane: Some(first_pane),
            markdown_content: markdown::Content::parse(MARKDOWN_SAMPLE),
            markdown_link: None,
            qr_data: qr_code::Data::new(QR_PAYLOAD).ok(),
            #[cfg(feature = "iced_aw")]
            aw_tab_bar_active: 0,
            #[cfg(feature = "iced_aw")]
            aw_tabs_active: 0,
            #[cfg(feature = "iced_aw")]
            aw_sidebar_active: 0,
            #[cfg(feature = "iced_aw")]
            aw_list_options: vec![
                "Adwaita".to_string(),
                "Breeze".to_string(),
                "Catppuccin".to_string(),
                "Dracula".to_string(),
                "Gruvbox".to_string(),
                "Nord".to_string(),
                "Solarized".to_string(),
                "Tokyo Night".to_string(),
            ],
            #[cfg(feature = "iced_aw")]
            aw_list_selected: Some(0),
            #[cfg(feature = "iced_aw")]
            aw_card_open: true,
            #[cfg(feature = "iced_aw")]
            aw_last_action: String::new(),
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
                        self.accessibility = system.accessibility.clone();
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

    // Layout tab
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    PaneSplit(pane_grid::Axis, pane_grid::Pane),
    PaneClosed(pane_grid::Pane),

    // Graphics tab
    MarkdownLinkClicked(markdown::Uri),

    // Extra widgets tab (iced_aw)
    #[cfg(feature = "iced_aw")]
    AwTabBarSelected(usize),
    #[cfg(feature = "iced_aw")]
    AwTabsSelected(usize),
    #[cfg(feature = "iced_aw")]
    AwSidebarSelected(usize),
    #[cfg(feature = "iced_aw")]
    AwListSelected(usize, String),
    #[cfg(feature = "iced_aw")]
    AwCardToggled,
    #[cfg(feature = "iced_aw")]
    AwActionChosen(String),

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

        for chunk in pixels.as_chunks_mut::<4>().0 {
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
        Message::PaneClicked(pane) => state.focused_pane = Some(pane),
        Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            state.panes.drop(pane, target);
            state.focused_pane = Some(pane);
        }
        // Picked and Canceled leave the layout as it was.
        Message::PaneDragged(_) => {}
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            state.panes.resize(split, ratio);
        }
        Message::PaneSplit(axis, pane) => {
            let label = state.pane_count.saturating_add(1);
            if let Some((new_pane, _)) = state.panes.split(axis, pane, label) {
                state.pane_count = label;
                state.focused_pane = Some(new_pane);
            }
        }
        Message::PaneClosed(pane) => {
            if let Some((_, sibling)) = state.panes.close(pane) {
                state.focused_pane = Some(sibling);
            }
        }
        Message::MarkdownLinkClicked(uri) => state.markdown_link = Some(uri),
        #[cfg(feature = "iced_aw")]
        Message::AwTabBarSelected(i) => state.aw_tab_bar_active = i,
        #[cfg(feature = "iced_aw")]
        Message::AwTabsSelected(i) => state.aw_tabs_active = i,
        #[cfg(feature = "iced_aw")]
        Message::AwSidebarSelected(i) => state.aw_sidebar_active = i,
        #[cfg(feature = "iced_aw")]
        Message::AwListSelected(index, value) => {
            state.aw_list_selected = Some(index);
            state.aw_last_action = format!("SelectionList: {value}");
        }
        #[cfg(feature = "iced_aw")]
        Message::AwCardToggled => state.aw_card_open = !state.aw_card_open,
        #[cfg(feature = "iced_aw")]
        Message::AwActionChosen(what) => state.aw_last_action = what,
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
    let resolved = &state.current_resolved;
    let radius = native_theme_iced::border_radius(resolved);
    let sb_width = native_theme_iced::scrollbar_width(resolved);
    let btn_pad = native_theme_iced::button_padding(resolved);
    let inp_pad = native_theme_iced::input_padding(resolved);

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
            .handle(arrow_handle(resolved))
            .style(styles::pick_list(resolved))
            .menu_style(styles::menu(resolved))
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
            .handle(arrow_handle(resolved))
            .style(styles::pick_list(resolved))
            .menu_style(styles::menu(resolved))
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
            .handle(arrow_handle(resolved))
            .style(styles::pick_list(resolved))
            .menu_style(styles::menu(resolved))
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
                container(
                    scrollable(text(info_text).size(ts.caption.size))
                        .direction(scrollable::Direction::Vertical(styles::scrollbar(resolved)))
                        .style(styles::scrollable(resolved)),
                )
                .padding(Padding::from(sp.s))
                .style(styles::container_card(resolved))
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
                    rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                    theme_section,
                    color_mode_section,
                    rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                    icon_theme_section,
                    rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                    metrics_info,
                    rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                    widget_info_panel,
                ]
                .spacing(sp.s)
                .padding(Padding::from(sp.m))
                .width(Length::Fixed(210.0)),
            )
            .direction(scrollable::Direction::Vertical(styles::scrollbar(resolved)))
            .style(styles::scrollable(resolved)),
        )
        .style(styles::container_card(resolved))
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
                // The open tab is the call to action; the rest are plain.
                let btn = if tab == state.active_tab {
                    btn.style(styles::button_primary(resolved))
                } else {
                    btn.style(styles::button(resolved))
                };
                btn.on_press(Message::TabSelected(tab))
                    .padding(Padding::from([sp.xs, sp.m]))
                    .into()
            })
            .collect();
        // More tabs than the window is wide: the strip scrolls sideways rather
        // than clipping the last ones.
        scrollable(row(tabs).spacing(sp.xs))
            .direction(scrollable::Direction::Horizontal(styles::scrollbar(
                resolved,
            )))
            .style(styles::scrollable(resolved))
            .into()
    };

    // ---- Tab content ----
    let tab_content: Element<'_, Message> = match state.active_tab {
        Tab::Buttons => view_buttons(state, btn_pad),
        Tab::TextInputs => view_text_inputs(state, inp_pad),
        Tab::Selection => view_selection(state),
        Tab::Range => view_range(state),
        Tab::Display => view_display(state),
        Tab::Layout => view_layout(state),
        Tab::Graphics => view_graphics(state),
        #[cfg(feature = "iced_aw")]
        Tab::Extra => view_extra(state),
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
        .push(rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)))
        .push(
            // Scrollable content
            scrollable(container(tab_content).padding(content_padding).width(Fill))
                .direction(scrollable::Direction::Vertical(styles::scrollbar(resolved)))
                .style(styles::scrollable(resolved))
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
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let ext = state.current_theme.extended_palette();
    let radius_s = format!("{:.0}px", resolved.button.border.corner_radius);

    let apply_pad =
        |b: button::Button<'a, Message>| -> button::Button<'a, Message> { b.padding(btn_pad) };

    let header = section_header(
        "Buttons",
        "Interactive button styles from the resolved theme",
        resolved,
        ts,
        sp,
    );

    let primary_row = hoverable(
        widget_tooltip_themed(
            state,
            "Button (Primary)",
            &[
                (
                    "bg",
                    "button.primary_background",
                    to_color(resolved.button.primary_background),
                ),
                (
                    "text",
                    "button.primary_text_color",
                    to_color(resolved.button.primary_text_color),
                ),
                (
                    "hover bg",
                    "iced's primary.strong",
                    ext.primary.strong.color,
                ),
            ],
            &[
                ("border-radius", &radius_s),
                ("shadow", "iced's own — the model has no shadow geometry"),
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
                        .style(styles::button_primary(resolved))
                ),
                apply_pad(
                    button("Secondary")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button(resolved))
                ),
                apply_pad(
                    button("Success")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_success(resolved))
                ),
                apply_pad(
                    button("Danger")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_danger(resolved))
                ),
                apply_pad(
                    button("Text Style")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_link(resolved))
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
            &[
                (
                    "bg",
                    "button.disabled_background",
                    to_color(
                        resolved
                            .button
                            .disabled_background
                            .unwrap_or(resolved.button.background_color),
                    ),
                ),
                (
                    "text",
                    "button.disabled_text_color",
                    to_color(resolved.button.disabled_text_color),
                ),
            ],
            &[],
            &[
                ("cursor", "not interactive"),
                ("theme", "one disabled pair for every button class"),
            ],
        ),
        column![
            text("Disabled State").size(ts.dialog_title.size),
            text("Buttons without on_press are rendered as disabled:")
                .size(ts.section_heading.size),
            row![
                apply_pad(button("Disabled Primary").style(styles::button_primary(resolved))),
                apply_pad(button("Disabled Secondary").style(styles::button(resolved))),
                apply_pad(button("Disabled Danger").style(styles::button_danger(resolved))),
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
                    .style(styles::button_primary(resolved))
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
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        disabled_row,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        interactive,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Text Inputs
// ---------------------------------------------------------------------------

fn view_text_inputs<'a>(state: &'a State, inp_pad: Padding) -> Element<'a, Message> {
    let sp = &SP;
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let i = &resolved.input;
    let radius = i.border.corner_radius;
    let radius_s = format!("{radius:.0}px");

    let header = section_header(
        "Text Inputs",
        "Single-line TextInput and multi-line TextEditor",
        resolved,
        ts,
        sp,
    );

    let single_line = {
        let mut input = text_input("Type something here...", &state.text_input_value)
            .on_input(Message::TextInputChanged)
            .style(styles::text_input(resolved));
        {
            input = input.padding(inp_pad);
        }

        hoverable(
            widget_tooltip_themed(
                state,
                "TextInput",
                &[
                    ("border", "input.border.color", to_color(i.border.color)),
                    ("bg", "input.background_color", to_color(i.background_color)),
                    ("text", "input.font.color", to_color(i.font.color)),
                    (
                        "placeholder",
                        "input.placeholder_color",
                        to_color(i.placeholder_color),
                    ),
                    (
                        "selection",
                        "input.selection_background",
                        to_color(i.selection_background),
                    ),
                ],
                &[("border-radius", &radius_s)],
                &[
                    ("padding", "set per widget instance"),
                    ("height", "set by iced"),
                    ("icon color", "no native source — iced's own"),
                ],
            ),
            column![
                text("TextInput (single line)").size(ts.dialog_title.size),
                input,
                text(format!(
                    "Characters: {}  |  input.border.corner_radius: {radius:.0}px",
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
            .secure(true)
            .style(styles::text_input(resolved));
        {
            input = input.padding(inp_pad);
        }

        hoverable(
            widget_tooltip(
                "TextInput (secure)",
                &[
                    ("border", "input.border.color", to_color(i.border.color)),
                    ("bg", "input.background_color", to_color(i.background_color)),
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
                ("bg", "input.background_color", to_color(i.background_color)),
                ("text", "input.font.color", to_color(i.font.color)),
                (
                    "selection",
                    "input.selection_background",
                    to_color(i.selection_background),
                ),
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
                .style(styles::text_editor(resolved))
                .height(Length::Fixed(180.0)),
            text("Supports multi-line editing, selection, and scrolling").size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    column![
        header,
        single_line,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        secure_input,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
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
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let c = &resolved.checkbox;
    let sw = &resolved.switch;
    let cb = &resolved.combo_box;
    let checkbox_radius_s = format!("{:.0}px", c.border.corner_radius);
    let combo_radius_s = format!("{:.0}px", cb.border.corner_radius);
    let label_gap_s = format!("{:.0}px", c.label_gap);
    let track_radius_s = format!("{:.0}px", sw.track_radius);
    let track_height_s = format!("{:.0}px", sw.track_height);
    let thumb_diameter_s = format!("{:.0}px", sw.thumb_diameter);
    let arrow_size_s = format!("{:.0}px", cb.arrow_icon_size);
    let input_radius_s = format!("{:.0}px", resolved.input.border.corner_radius);

    let header = section_header(
        "Selection Widgets",
        "Checkbox, Radio, Toggler, PickList, and ComboBox",
        resolved,
        ts,
        sp,
    );

    let checkboxes = hoverable(
        widget_tooltip(
            "Checkbox",
            &[
                (
                    "checked bg",
                    "checkbox.checked_background",
                    to_color(c.checked_background),
                ),
                (
                    "checkmark",
                    "checkbox.indicator_color",
                    to_color(c.indicator_color),
                ),
                (
                    "unchecked border",
                    "checkbox.unchecked_border_color",
                    to_color(c.unchecked_border_color.unwrap_or(c.border.color)),
                ),
                (
                    "bg",
                    "checkbox.unchecked_background",
                    to_color(c.unchecked_background.unwrap_or(c.background_color)),
                ),
            ],
            &[
                ("border-radius", &checkbox_radius_s),
                ("label gap", &label_gap_s),
            ],
            &[
                ("size", "hardcoded by iced"),
                ("indicator size", "checkbox.indicator_width has no receiver"),
            ],
        ),
        column![
            text("Checkboxes").size(ts.dialog_title.size),
            checkbox(state.checkbox_a)
                .label("Enable notifications")
                .spacing(c.label_gap)
                .style(styles::checkbox(resolved))
                .on_toggle(Message::CheckboxAToggled),
            checkbox(state.checkbox_b)
                .label("Dark mode auto-detect")
                .spacing(c.label_gap)
                .style(styles::checkbox(resolved))
                .on_toggle(Message::CheckboxBToggled),
            checkbox(state.checkbox_c)
                .label("Remember preferences")
                .spacing(c.label_gap)
                .style(styles::checkbox(resolved))
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
                (
                    "selected",
                    "checkbox.checked_background",
                    to_color(c.checked_background),
                ),
                (
                    "dot",
                    "checkbox.indicator_color",
                    to_color(c.indicator_color),
                ),
                (
                    "unselected border",
                    "checkbox.unchecked_border_color",
                    to_color(c.unchecked_border_color.unwrap_or(c.border.color)),
                ),
                (
                    "bg",
                    "checkbox.unchecked_background",
                    to_color(c.unchecked_background.unwrap_or(c.background_color)),
                ),
            ],
            &[("label gap", &label_gap_s)],
            &[
                ("size", "hardcoded"),
                ("border-radius", "radio::Style carries no corner radius"),
                ("disabled", "radio::Status has no disabled value"),
            ],
        ),
        column![
            text("Radio Buttons").size(ts.dialog_title.size),
            radio(
                "Apple",
                Fruit::Apple,
                state.selected_fruit,
                Message::FruitSelected
            )
            .spacing(c.label_gap)
            .style(styles::radio(resolved)),
            radio(
                "Banana",
                Fruit::Banana,
                state.selected_fruit,
                Message::FruitSelected
            )
            .spacing(c.label_gap)
            .style(styles::radio(resolved)),
            radio(
                "Cherry",
                Fruit::Cherry,
                state.selected_fruit,
                Message::FruitSelected
            )
            .spacing(c.label_gap)
            .style(styles::radio(resolved)),
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
                (
                    "active track",
                    "switch.checked_background",
                    to_color(sw.checked_background),
                ),
                (
                    "inactive track",
                    "switch.unchecked_background",
                    to_color(sw.unchecked_background),
                ),
                (
                    "thumb",
                    "switch.thumb_background",
                    to_color(sw.thumb_background),
                ),
            ],
            &[
                ("border-radius", &track_radius_s),
                ("track height", &track_height_s),
                ("thumb diameter", &thumb_diameter_s),
            ],
            &[
                ("track width", "iced lays the track out as 2 x its height"),
                ("border", "SwitchTheme carries none — iced's own"),
                ("animation timing", "hardcoded"),
            ],
        ),
        column![
            text("Toggler (Switch)").size(ts.dialog_title.size),
            toggler(state.toggler_enabled)
                .label("Feature flag enabled")
                .size(sw.track_height)
                .style(styles::toggler(resolved))
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
                (
                    "bg",
                    "combo_box.background_color",
                    to_color(cb.background_color),
                ),
                ("text", "combo_box.font.color", to_color(cb.font.color)),
                (
                    "border",
                    "combo_box.border.color",
                    to_color(cb.border.color),
                ),
                (
                    "menu bg",
                    "menu.background_color",
                    to_color(resolved.menu.background_color),
                ),
                (
                    "menu selected",
                    "menu.hover_background",
                    to_color(resolved.menu.hover_background),
                ),
            ],
            &[
                ("border-radius", &combo_radius_s),
                ("arrow size", &arrow_size_s),
            ],
            &[
                ("dropdown arrow", "iced's own chevron glyph"),
                ("arrow color", "ComboBoxTheme carries no arrow color"),
                ("arrow area width", "no receiver in iced"),
            ],
        ),
        column![
            text("PickList (dropdown)").size(ts.dialog_title.size),
            pick_list(
                languages,
                state.pick_list_selected.as_ref(),
                Message::PickListSelected,
            )
            .handle(arrow_handle(resolved))
            .style(styles::pick_list(resolved))
            .menu_style(styles::menu(resolved))
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
                (
                    "bg",
                    "input.background_color",
                    to_color(resolved.input.background_color),
                ),
                (
                    "text",
                    "input.font.color",
                    to_color(resolved.input.font.color),
                ),
                (
                    "border",
                    "input.border.color",
                    to_color(resolved.input.border.color),
                ),
                (
                    "menu bg",
                    "menu.background_color",
                    to_color(resolved.menu.background_color),
                ),
            ],
            &[("border-radius", &input_radius_s)],
            &[
                ("search", "built-in text filter"),
                ("field style", "a ComboBox takes a text_input style"),
            ],
        ),
        column![
            text("ComboBox (searchable dropdown)").size(ts.dialog_title.size),
            combo_box(
                &state.combo_state,
                "Search a language...",
                state.combo_selected.as_ref(),
                Message::ComboBoxSelected,
            )
            .input_style(styles::text_input(resolved))
            .menu_style(styles::menu(resolved))
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
            column![
                checkboxes,
                rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                togglers,
            ]
            .spacing(sp.xl)
            .width(Fill),
            rule::vertical(resolved.separator.line_width).style(styles::rule(resolved)),
            column![
                radios,
                rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                pickers,
                rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
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
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let sl = &resolved.slider;
    let pb = &resolved.progress_bar;
    let rail_s = format!("{:.0}px", sl.track_height);
    let thumb_s = format!("{:.0}px", sl.thumb_diameter);
    let bar_girth_s = format!("{:.0}px", pb.track_height);

    let header = section_header(
        "Range Widgets",
        "Slider, VerticalSlider, and ProgressBar",
        resolved,
        ts,
        sp,
    );

    let horiz_slider = hoverable(
        widget_tooltip(
            "Horizontal Slider",
            &[
                ("active track", "slider.fill_color", to_color(sl.fill_color)),
                (
                    "inactive track",
                    "slider.track_color",
                    to_color(sl.track_color),
                ),
                ("handle", "slider.thumb_color", to_color(sl.thumb_color)),
                (
                    "hovered handle",
                    "slider.thumb_hover_color",
                    to_color(sl.thumb_hover_color.unwrap_or(sl.thumb_color)),
                ),
            ],
            &[("rail width", &rail_s), ("thumb diameter", &thumb_s)],
            &[
                ("widget height", "no native source — iced's own"),
                ("dragged handle fill", "no native source — iced's own"),
            ],
        ),
        column![
            text("Horizontal Slider").size(ts.dialog_title.size),
            row![
                slider(0.0..=100.0, state.slider_value, Message::SliderChanged)
                    .style(styles::slider(resolved))
                    .width(Fill),
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
            &[("track", "slider.fill_color", to_color(sl.fill_color))],
            &[("step", "5.0")],
            &[("snap behavior", "hardcoded step increments")],
        ),
        column![
            text("Slider with Step (5-unit increments)").size(ts.dialog_title.size),
            row![
                slider(0.0..=100.0, state.slider_step, Message::StepSliderChanged)
                    .step(5.0_f32)
                    .style(styles::slider(resolved))
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
            &[("track", "slider.fill_color", to_color(sl.fill_color))],
            &[
                ("orientation", "vertical"),
                ("rail width", &rail_s),
                ("thumb diameter", &thumb_s),
            ],
            &[("widget width", "no native source — iced's own")],
        ),
        column![
            text("Vertical Slider").size(ts.dialog_title.size),
            row![
                container(
                    vertical_slider(0.0..=100.0, state.vslider_value, Message::VSliderChanged)
                        .style(styles::slider(resolved))
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
                ("fill", "progress_bar.fill_color", to_color(pb.fill_color)),
                (
                    "track bg",
                    "progress_bar.track_color",
                    to_color(pb.track_color),
                ),
                (
                    "border",
                    "progress_bar.border.color",
                    to_color(pb.border.color),
                ),
            ],
            &[("girth", &bar_girth_s)],
            &[
                ("min width", "progress_bar.min_width has no receiver"),
                ("animation", "none — immediate"),
            ],
        ),
        column![
            text("Progress Bars").size(ts.dialog_title.size),
            text("Driven by horizontal slider value:").size(ts.section_heading.size),
            progress_bar(0.0..=100.0, state.slider_value)
                .girth(Length::Fixed(pb.track_height))
                .style(styles::progress_bar(resolved)),
            space().height(Length::Fixed(4.0)),
            text("Separate progress control:").size(ts.section_heading.size),
            row![
                slider(0.0..=100.0, state.progress_value, Message::ProgressChanged)
                    .style(styles::slider(resolved))
                    .width(Fill),
                text(format!("{:.0}%", state.progress_value))
                    .size(ts.section_heading.size)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(sp.m)
            .align_y(iced::Center),
            progress_bar(0.0..=100.0, state.progress_value)
                .girth(Length::Fixed(pb.track_height))
                .style(styles::progress_bar(resolved)),
        ]
        .spacing(sp.s)
        .into(),
    );

    column![
        header,
        horiz_slider,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        step_slider,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        vert_slider,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        progress,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Display
// ---------------------------------------------------------------------------

fn view_display(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let card = &resolved.card;
    let tip = &resolved.tooltip;
    let sep = &resolved.separator;
    let card_radius = card.border.corner_radius;
    let card_radius_s = format!("{card_radius:.0}px");
    let tip_radius_s = format!("{:.0}px", tip.border.corner_radius);
    let line_width_s = format!("{:.0}px", sep.line_width);

    let header = section_header(
        "Display Widgets",
        "Container, Rule, Tooltip, and layout helpers",
        resolved,
        ts,
        sp,
    );

    let containers = hoverable(
        widget_tooltip(
            "Styled Containers (card)",
            &[
                (
                    "bg",
                    "card.background_color",
                    to_color(card.background_color),
                ),
                ("border", "card.border.color", to_color(card.border.color)),
            ],
            &[("border-radius", &card_radius_s)],
            &[
                ("padding", "set per widget instance"),
                ("text", "CardTheme carries no font — the label is inherited"),
            ],
        ),
        column![
            text("Styled Containers").size(ts.dialog_title.size),
            container(
                column![
                    text("Container (card fill)").size(ts.section_heading.size),
                    text(format!(
                        "This container uses styles::container_card. \
                         card.border.corner_radius: {card_radius:.0}px."
                    ))
                    .size(ts.caption.size),
                ]
                .spacing(sp.xs),
            )
            .padding(Padding::from(sp.l))
            .style(styles::container_card(resolved))
            .width(Fill),
            container(
                text(
                    "A secondary container with different padding. Containers take their \
                      background and border from the resolved card theme."
                )
                .size(ts.caption.size),
            )
            .padding(Padding::from([sp.m, sp.xl]))
            .style(styles::container_card(resolved))
            .width(Fill),
        ]
        .spacing(sp.m)
        .into(),
    );

    let rules = column![
        text("Divider Rules").size(ts.dialog_title.size),
        text(format!(
            "iced takes a rule's thickness as the constructor's argument; \
             the platform states one, separator.line_width ({line_width_s}):"
        ))
        .size(ts.section_heading.size),
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        text("The line color is separator.line_color.").size(ts.caption.size),
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        text("Its radius and fill mode have no native source — they are iced's.")
            .size(ts.caption.size),
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
    ]
    .spacing(sp.s);

    let tooltips = hoverable(
        widget_tooltip(
            "Tooltip",
            &[
                (
                    "bg",
                    "tooltip.background_color",
                    to_color(tip.background_color),
                ),
                ("text", "tooltip.font.color", to_color(tip.font.color)),
                ("border", "tooltip.border.color", to_color(tip.border.color)),
            ],
            &[
                ("positions", "Top / Bottom / Left / Right"),
                ("border-radius", &tip_radius_s),
            ],
            &[
                ("gap", "set per widget instance"),
                ("delay", "none"),
                ("max width", "iced wraps the tip's own content element"),
            ],
        ),
        column![
            text("Tooltips").size(ts.dialog_title.size),
            row![
                tooltip(
                    button("Hover: Top")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_primary(resolved)),
                    text("Tooltip on top!"),
                    tooltip::Position::Top,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button("Hover: Bottom")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button(resolved)),
                    text("Tooltip on bottom!"),
                    tooltip::Position::Bottom,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button("Hover: Left")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_success(resolved)),
                    text("Tooltip on left!"),
                    tooltip::Position::Left,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button("Hover: Right")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_danger(resolved)),
                    text("Tooltip on right!"),
                    tooltip::Position::Right,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
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
            native_theme_iced::font_size(&state.current_resolved, &state.accessibility)
        );
        let mf = native_theme_iced::mono_font_family(&state.current_resolved);
        let ms = format!(
            "{:.1}px",
            native_theme_iced::mono_font_size(&state.current_resolved, &state.accessibility)
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
    .style(styles::container_card(resolved))
    .width(Fill);

    let spacing_demo = column![
        text("Spacing & Layout").size(ts.dialog_title.size),
        row![
            container(text("A").size(ts.section_heading.size))
                .padding(Padding::from(sp.m))
                .style(styles::container_card(resolved))
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            container(text("B").size(ts.section_heading.size))
                .padding(Padding::from(sp.m))
                .style(styles::container_card(resolved))
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            container(text("C").size(ts.section_heading.size))
                .padding(Padding::from(sp.m))
                .style(styles::container_card(resolved))
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            space().width(Fill),
            container(text("Right-aligned").size(ts.caption.size))
                .padding(Padding::from(sp.m))
                .style(styles::container_card(resolved)),
        ]
        .spacing(sp.s)
        .align_y(iced::Center),
    ]
    .spacing(sp.s);

    column![
        header,
        containers,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        rules,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        tooltips,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        spacing_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        info_box,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Layout
// ---------------------------------------------------------------------------

/// One row of the text-scale `table`: role, size, weight, line height.
///
/// `table::column`'s view function takes its row by value, so the row type is
/// `Clone` and carries the formatted strings rather than a borrow.
type ScaleRow = (&'static str, String, String, String);

/// Grid, PaneGrid and Table: the three `iced_widget` modules that arrange
/// other widgets rather than paint a control.
fn view_layout(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let sep = &resolved.separator;
    let sp_th = &resolved.splitter;
    let line_width_s = format!("{:.0}px", sep.line_width);
    let divider_width_s = format!("{:.0}px", sp_th.divider_width);

    let header = section_header(
        "Layout Widgets",
        "Grid, PaneGrid and Table: the modules that arrange other widgets",
        resolved,
        ts,
        sp,
    );

    // ---- Grid: the platform's own colors, four to a row ----

    let swatch_border = native_theme_iced::border_color(resolved);
    let swatch_bw = resolved.defaults.border.line_width;
    let swatch_r = native_theme_iced::border_radius(resolved);
    let caption_sz = ts.caption.size;
    let xxs_sp = sp.xxs;
    let cell = |label: &'static str, color: Color| -> Element<'_, Message> {
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
    let d = &resolved.defaults;

    let grid_demo = hoverable(
        widget_tooltip(
            "Grid",
            &[
                (
                    "cell fill",
                    "each cell's own color",
                    to_color(d.accent_color),
                ),
                (
                    "cell border",
                    "defaults.border.color",
                    to_color(d.border.color),
                ),
            ],
            &[
                ("columns", "4"),
                ("cell radius", "defaults.border.corner_radius"),
            ],
            &[
                (
                    "Style",
                    "grid has no Catalog and no Style — it only places its children \
                     (grid.rs:13-19)",
                ),
                ("spacing", "the showcase's own scale"),
            ],
        ),
        column![
            text("Grid (cells in columns)").size(ts.dialog_title.size),
            text(
                "iced_widget::grid distributes its children over a fixed number of \
                 columns. Here: the eight colors ResolvedDefaults names."
            )
            .size(ts.section_heading.size),
            grid([
                cell("accent_color", to_color(d.accent_color)),
                cell("danger_color", to_color(d.danger_color)),
                cell("warning_color", to_color(d.warning_color)),
                cell("success_color", to_color(d.success_color)),
                cell("info_color", to_color(d.info_color)),
                cell("link_color", to_color(d.link_color)),
                cell("muted_color", to_color(d.muted_color)),
                cell("surface_color", to_color(d.surface_color)),
            ])
            .columns(4)
            .spacing(sp.s)
            .height(Length::Fixed(64.0)),
        ]
        .spacing(sp.s)
        .into(),
    );

    // ---- PaneGrid: splits the platform's splitter theme paints ----

    let pane_style = {
        let hovered = to_color(sp_th.hover_color);
        let divider_width = sp_th.divider_width;
        move |theme: &Theme| {
            let iced = pane_grid::default(theme);
            pane_grid::Style {
                // The model states no drop-target highlight, so the region
                // iced paints under a dragged pane stays iced's own.
                hovered_region: iced.hovered_region,
                // A picked split is a split being dragged; SplitterTheme
                // states an idle and a hovered color and no dragged one, so
                // that one line is iced's (spec §3.2).
                picked_split: pane_grid::Line {
                    color: iced.picked_split.color,
                    width: divider_width,
                },
                hovered_split: pane_grid::Line {
                    color: hovered,
                    width: divider_width,
                },
            }
        }
    };

    let panes = pane_grid(&state.panes, |pane, label, _is_maximized| {
        let focused = state.focused_pane == Some(pane);
        let title = format!("Pane {label}");
        let controls = row![
            button(text("split |").size(ts.caption.size))
                .on_press(Message::PaneSplit(pane_grid::Axis::Vertical, pane))
                .style(styles::button(resolved))
                .padding(Padding::from([sp.xxs, sp.xs])),
            button(text("split —").size(ts.caption.size))
                .on_press(Message::PaneSplit(pane_grid::Axis::Horizontal, pane))
                .style(styles::button(resolved))
                .padding(Padding::from([sp.xxs, sp.xs])),
            button(text("close").size(ts.caption.size))
                .on_press(Message::PaneClosed(pane))
                .style(styles::button_danger(resolved))
                .padding(Padding::from([sp.xxs, sp.xs])),
        ]
        .spacing(sp.xs);

        let title_bar = pane_grid::TitleBar::new(text(title).size(ts.section_heading.size))
            .controls(Element::from(controls))
            .always_show_controls()
            .padding(Padding::from([sp.xs, sp.s]))
            .style(styles::container_card(resolved));

        let body = column![
            text(if focused {
                "Focused. Drag the title bar onto another pane to move it."
            } else {
                "Drag the split between the panes to resize."
            })
            .size(ts.caption.size),
        ]
        .spacing(sp.xs)
        .padding(Padding::from(sp.s));

        pane_grid::Content::new(body)
            .title_bar(title_bar)
            .style(styles::container_card(resolved))
    })
    .width(Fill)
    .height(Length::Fixed(220.0))
    .spacing(sp_th.divider_width)
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(sp_th.divider_width, Message::PaneResized)
    .style(pane_style);

    let pane_demo = hoverable(
        widget_tooltip(
            "PaneGrid",
            &[(
                "hovered split",
                "splitter.hover_color",
                to_color(sp_th.hover_color),
            )],
            &[
                ("split width", &divider_width_s),
                ("pane surface", "styles::container_card"),
                ("title bar", "styles::container_card"),
            ],
            &[
                (
                    "drop region",
                    "the model states no drop-target highlight — iced's own",
                ),
                (
                    "picked split",
                    "SplitterTheme states no dragged color — iced's own, at the \
                     platform's width",
                ),
                (
                    "splitter.divider_color",
                    "no receiver: iced leaves an idle split unpainted",
                ),
            ],
        ),
        column![
            text("PaneGrid (split, drag and resize)").size(ts.dialog_title.size),
            text(
                "Split a pane, drag its title bar onto another one, or drag the \
                 divider between two panes."
            )
            .size(ts.section_heading.size),
            panes,
        ]
        .spacing(sp.s)
        .into(),
    );

    // ---- Table: the resolved text scale, as data ----

    let scale_rows: Vec<ScaleRow> = vec![
        (
            "caption",
            format!("{:.0}px", ts.caption.size),
            format!("{}", ts.caption.weight),
            format!("{:.1}px", ts.caption.line_height),
        ),
        (
            "section_heading",
            format!("{:.0}px", ts.section_heading.size),
            format!("{}", ts.section_heading.weight),
            format!("{:.1}px", ts.section_heading.line_height),
        ),
        (
            "dialog_title",
            format!("{:.0}px", ts.dialog_title.size),
            format!("{}", ts.dialog_title.weight),
            format!("{:.1}px", ts.dialog_title.line_height),
        ),
        (
            "display",
            format!("{:.0}px", ts.display.size),
            format!("{}", ts.display.weight),
            format!("{:.1}px", ts.display.line_height),
        ),
    ];

    let scale_table = table(
        [
            table::column(text("text_scale role").size(ts.section_heading.size), {
                let size = ts.caption.size;
                move |r: ScaleRow| text(r.0).size(size)
            })
            .width(Length::Fixed(160.0)),
            table::column(text("size").size(ts.section_heading.size), {
                let size = ts.caption.size;
                move |r: ScaleRow| text(r.1).size(size)
            })
            .width(Length::Fixed(90.0)),
            table::column(text("weight").size(ts.section_heading.size), {
                let size = ts.caption.size;
                move |r: ScaleRow| text(r.2).size(size)
            })
            .width(Length::Fixed(90.0)),
            table::column(text("line height").size(ts.section_heading.size), {
                let size = ts.caption.size;
                move |r: ScaleRow| text(r.3).size(size)
            })
            .width(Length::Fixed(110.0)),
        ],
        scale_rows,
    )
    .padding_x(sp.s)
    .padding_y(sp.xs)
    .separator_x(sep.line_width)
    .separator_y(sep.line_width)
    .width(Fill);

    let table_demo = hoverable(
        widget_tooltip(
            "Table",
            &[(
                "separators",
                "iced's own — see below",
                to_color(sep.line_color),
            )],
            &[
                ("separator_x / separator_y", &line_width_s),
                ("cell padding", "the showcase's own scale"),
            ],
            &[(
                "Style",
                "iced_widget 0.14.2 gives Table a Catalog and a Style but no \
                 `.style(..)` or `.class(..)` setter (table.rs:149-196), so its \
                 separator colors stay table::default(theme)",
            )],
        ),
        column![
            text("Table (columns and rows)").size(ts.dialog_title.size),
            text("The four typographic roles this theme resolves:").size(ts.section_heading.size),
            scale_table,
        ]
        .spacing(sp.s)
        .into(),
    );

    column![
        header,
        grid_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        pane_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        table_demo,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Graphics
// ---------------------------------------------------------------------------

/// The document the `markdown` section parses once, at start-up.
const MARKDOWN_SAMPLE: &str = "\
# Markdown under a native theme

`iced_widget::markdown` parses its input once and renders the items every
frame with a `Settings` value. The showcase builds that value from the
resolved theme: heading sizes from `text_scale`, the base size from
`defaults.font`, code size from `defaults.mono_font` and the link colour
below from `link.font.color`.

- headings, lists and rules come from the parser
- [a link, in the platform's own link colour](https://github.com/tiborgats/native-theme)
- inline `code` and code blocks share the monospace weight

```rust
let settings = markdown::Settings::with_text_size(size, style);
```
";

/// What the QR code encodes.
const QR_PAYLOAD: &str = "https://github.com/tiborgats/native-theme";

/// A small drawing whose every colour and width is the platform's.
///
/// `canvas` has no `Catalog` and no `Style` of its own: a [`canvas::Program`]
/// is handed the theme and paints whatever it likes (`canvas/program.rs:49`).
/// So the values are captured from the resolved theme when the program is
/// built, which `view` does once per frame.
struct ThemeSketch {
    surface: Color,
    outline: Color,
    outline_width: f32,
    corner_radius: f32,
    accent: Color,
    ink: Color,
    ink_width: f32,
    dot_radius: f32,
}

impl<Message> canvas::Program<Message> for ThemeSketch {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let panel = canvas::Path::rounded_rectangle(
            iced::Point::ORIGIN,
            frame.size(),
            self.corner_radius.into(),
        );
        frame.fill(&panel, self.surface);
        frame.stroke(
            &panel,
            canvas::Stroke::default()
                .with_color(self.outline)
                .with_width(self.outline_width),
        );

        let center = frame.center();
        let baseline = canvas::Path::line(
            iced::Point::new(0.0, center.y),
            iced::Point::new(frame.width(), center.y),
        );
        frame.stroke(
            &baseline,
            canvas::Stroke::default()
                .with_color(self.ink)
                .with_width(self.ink_width),
        );

        frame.fill(&canvas::Path::circle(center, self.dot_radius), self.accent);

        vec![frame.into_geometry()]
    }
}

/// Canvas, QRCode and Markdown: the three modules that paint content the
/// application supplies rather than a control.
fn view_graphics(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let sep = &resolved.separator;
    let d = &resolved.defaults;
    let line_width_s = format!("{:.0}px", sep.line_width);
    let dot_s = format!("{:.0}px", d.icon_sizes.large);

    let header = section_header(
        "Graphics Widgets",
        "Canvas, QRCode and Markdown: content the application paints itself",
        resolved,
        ts,
        sp,
    );

    // ---- Canvas ----

    let sketch = ThemeSketch {
        surface: to_color(resolved.card.background_color),
        outline: to_color(sep.line_color),
        outline_width: sep.line_width,
        corner_radius: resolved.card.border.corner_radius,
        accent: to_color(d.accent_color),
        ink: to_color(d.text_color),
        ink_width: sep.line_width,
        // iced states a circle by its radius, the model by its diameter.
        dot_radius: d.icon_sizes.large / 2.0,
    };

    let canvas_demo = hoverable(
        widget_tooltip(
            "Canvas",
            &[
                (
                    "panel fill",
                    "card.background_color",
                    to_color(resolved.card.background_color),
                ),
                ("outline", "separator.line_color", to_color(sep.line_color)),
                ("baseline", "defaults.text_color", to_color(d.text_color)),
                ("dot", "defaults.accent_color", to_color(d.accent_color)),
            ],
            &[
                ("stroke width", &line_width_s),
                ("panel radius", "card.border.corner_radius"),
                ("dot diameter", &dot_s),
            ],
            &[(
                "Style",
                "canvas has no Catalog and no Style: a Program paints with \
                 whatever it is given (canvas/program.rs:49-56)",
            )],
        ),
        column![
            text("Canvas (a drawing from the theme)").size(ts.dialog_title.size),
            text(
                "Every colour and every width below is a ResolvedTheme field, \
                 captured when the program is built."
            )
            .size(ts.section_heading.size),
            canvas(sketch)
                .width(Length::Fixed(280.0))
                .height(Length::Fixed(120.0)),
        ]
        .spacing(sp.s)
        .into(),
    );

    // ---- QR code ----

    let qr_cell = to_color(d.text_color);
    let qr_background = to_color(d.background_color);
    let qr_demo: Element<'_, Message> = match &state.qr_data {
        Some(data) => qr_code(data)
            .style(move |_theme: &Theme| qr_code::Style {
                cell: qr_cell,
                background: qr_background,
            })
            .into(),
        None => text("The payload could not be encoded as a QR code.")
            .size(ts.caption.size)
            .into(),
    };

    let qr_section = hoverable(
        widget_tooltip(
            "QRCode",
            &[
                ("cell", "defaults.text_color", qr_cell),
                ("background", "defaults.background_color", qr_background),
            ],
            &[("payload", QR_PAYLOAD)],
            &[(
                "cell size",
                "no native source — iced's own DEFAULT_CELL_SIZE (qr_code.rs:35)",
            )],
        ),
        column![
            text("QRCode").size(ts.dialog_title.size),
            text("Its two-colour Style is the platform's foreground on its background:")
                .size(ts.section_heading.size),
            qr_demo,
        ]
        .spacing(sp.s)
        .into(),
    );

    // ---- Markdown ----

    let md_settings = {
        // Every field the model does not carry is iced's own, read at run
        // time from the theme the connector produced.
        let iced_style = markdown::Style::from(&state.current_theme);
        let style = markdown::Style {
            font: iced::Font {
                weight: native_theme_iced::to_iced_weight(native_theme_iced::font_weight(resolved)),
                ..iced::Font::DEFAULT
            },
            inline_code_highlight: iced_style.inline_code_highlight,
            inline_code_padding: iced_style.inline_code_padding,
            inline_code_color: iced_style.inline_code_color,
            inline_code_font: iced::Font {
                weight: native_theme_iced::to_iced_weight(native_theme_iced::mono_font_weight(
                    resolved,
                )),
                ..iced::Font::MONOSPACE
            },
            code_block_font: iced::Font {
                weight: native_theme_iced::to_iced_weight(native_theme_iced::mono_font_weight(
                    resolved,
                )),
                ..iced::Font::MONOSPACE
            },
            link_color: to_color(resolved.link.font.color),
        };
        let mut settings = markdown::Settings::with_text_size(d.font.size, style);
        settings.h1_size = ts.display.size.into();
        settings.h2_size = ts.dialog_title.size.into();
        settings.h3_size = ts.section_heading.size.into();
        settings.code_size = d.mono_font.size.into();
        settings
    };

    let link_line = match &state.markdown_link {
        Some(uri) => format!("Last link clicked: {uri}"),
        None => "Click the link above and it is echoed here.".to_string(),
    };

    let markdown_demo = hoverable(
        widget_tooltip(
            "Markdown",
            &[
                (
                    "link",
                    "link.font.color",
                    to_color(resolved.link.font.color),
                ),
                ("text", "inherited from the surrounding container", {
                    to_color(d.text_color)
                }),
            ],
            &[
                ("base size", "defaults.font.size"),
                (
                    "h1 / h2 / h3",
                    "text_scale display / dialog_title / section_heading",
                ),
                ("code size", "defaults.mono_font.size"),
            ],
            &[
                (
                    "h4 / h5 / h6",
                    "the model names four typographic roles, not six — iced's own",
                ),
                (
                    "inline code chip",
                    "the model states no inline-code surface; its fill and its \
                     foreground are iced's, as a pair",
                ),
                (
                    "font family",
                    "iced's Family::Name takes a &'static str; a platform family \
                     name is an Arc<str> and cannot become one. Only the weight \
                     reaches the Font",
                ),
            ],
        ),
        column![
            text("Markdown").size(ts.dialog_title.size),
            container(
                markdown::view(state.markdown_content.items(), md_settings)
                    .map(Message::MarkdownLinkClicked)
            )
            .padding(Padding::from(sp.l))
            .style(styles::container_card(resolved))
            .width(Fill),
            text(link_line).size(ts.caption.size),
        ]
        .spacing(sp.s)
        .into(),
    );

    column![
        header,
        canvas_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        qr_section,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        markdown_demo,
    ]
    .spacing(sp.xl)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Extra widgets (iced_aw)
// ---------------------------------------------------------------------------

/// How wide a drop-down menu of the `MenuBar` is allowed to grow.
///
/// `MenuTheme` states no menu width, so this is the showcase's own layout
/// number, like the sidebar's 210px.
#[cfg(feature = "iced_aw")]
const AW_MENU_WIDTH: f32 = 220.0;

/// A drop-down of the `MenuBar`, with the gap `iced_aw` leaves around it.
#[cfg(feature = "iced_aw")]
fn aw_menu<'a>(
    items: Vec<Item<'a, Message, Theme, iced::Renderer>>,
    gap: f32,
) -> Menu<'a, Message, Theme, iced::Renderer> {
    Menu::new(items)
        .max_width(AW_MENU_WIDTH)
        .offset(gap)
        .spacing(gap)
}

/// The six `styles::aw::*` functions, on the eight `iced_aw` widgets the
/// connector's feature enables.
#[cfg(feature = "iced_aw")]
fn view_extra(state: &State) -> Element<'_, Message> {
    use iced_aw::style::Status;
    use std::rc::Rc;

    let sp = &SP;
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let sep = &resolved.separator;
    let card_t = &resolved.card;
    let menu_t = &resolved.menu;
    let tab_t = &resolved.tab;
    let side_t = &resolved.sidebar;
    let list_t = &resolved.list;
    let spin_t = &resolved.spinner;

    let header = section_header(
        "Extra widgets (iced_aw)",
        "The six styles::aw functions, each through the setter its widget offers",
        resolved,
        ts,
        sp,
    );

    // ---- Card ----

    let card_section: Element<'_, Message> = if state.aw_card_open {
        Card::new(
            text("Card").size(ts.dialog_title.size),
            column![
                text(
                    "CardTheme states one fill and one border, so the head, the body \
                     and the foot are the same surface, and the three labels are \
                     defaults.text_color."
                )
                .size(ts.caption.size),
            ]
            .spacing(sp.xs),
        )
        .foot(Element::from(
            row![
                button(text("Dismiss").size(ts.caption.size))
                    .on_press(Message::AwCardToggled)
                    .style(styles::button(resolved))
                    .padding(Padding::from([sp.xxs, sp.s])),
            ]
            .spacing(sp.xs),
        ))
        .on_close(Message::AwCardToggled)
        .close_size(resolved.defaults.icon_sizes.small)
        .padding_head(Padding::from(sp.s))
        .padding_body(Padding::from(sp.s))
        .padding_foot(Padding::from(sp.s))
        .width(Length::Fixed(420.0))
        .style(styles::aw::card(resolved))
        .into()
    } else {
        button(text("Show the card again").size(ts.caption.size))
            .on_press(Message::AwCardToggled)
            .style(styles::button_primary(resolved))
            .padding(Padding::from([sp.xs, sp.s]))
            .into()
    };

    let card_demo = hoverable(
        widget_tooltip(
            "Card",
            &[
                (
                    "surface",
                    "card.background_color",
                    to_color(card_t.background_color),
                ),
                ("border", "card.border.color", to_color(card_t.border.color)),
                (
                    "labels",
                    "defaults.text_color",
                    to_color(resolved.defaults.text_color),
                ),
            ],
            &[
                ("close icon", "defaults.icon_sizes.small"),
                ("radius", "card.border.corner_radius"),
            ],
            &[(
                "section padding",
                "the showcase's own scale — CardTheme states none",
            )],
        ),
        column![text("Card").size(ts.dialog_title.size), card_section,]
            .spacing(sp.s)
            .into(),
    );

    // ---- MenuBar and Menu ----

    let menu_entry = |label: &'static str| -> Element<'_, Message> {
        button(text(label).size(menu_t.font.size))
            .on_press(Message::AwActionChosen(format!("Menu: {label}")))
            .style(styles::button(resolved))
            .width(Fill)
            .height(Length::Fixed(menu_t.row_height))
            .padding(Padding::from([0.0, sp.s]))
            .into()
    };
    let menu_root = |label: &'static str| {
        button(text(label).size(menu_t.font.size))
            .on_press(Message::AwActionChosen(format!("Menu: {label}")))
            .style(styles::button(resolved))
            .padding(Padding::from([sp.xxs, sp.s]))
    };
    let drop = |items| aw_menu(items, sp.xxs);

    let menu_bar = MenuBar::new(vec![
        Item::with_menu(
            menu_root("File"),
            drop(vec![
                Item::new(menu_entry("New window")),
                Item::new(menu_entry("Open preset")),
                Item::with_menu(
                    menu_entry("Recent"),
                    drop(vec![
                        Item::new(menu_entry("adwaita.toml")),
                        Item::new(menu_entry("kde-breeze.toml")),
                    ]),
                ),
            ]),
        ),
        Item::with_menu(
            menu_root("View"),
            drop(vec![
                Item::new(menu_entry("Light")),
                Item::new(menu_entry("Dark")),
                Item::new(menu_entry("Follow the system")),
            ]),
        ),
    ])
    .padding(Padding::from(sp.xxs))
    .spacing(sp.xs)
    .style(styles::aw::menu(resolved));

    let menu_demo = hoverable(
        widget_tooltip(
            "MenuBar / Menu",
            &[
                (
                    "bar and panel",
                    "menu.background_color",
                    to_color(menu_t.background_color),
                ),
                (
                    "open path",
                    "menu.hover_background",
                    to_color(menu_t.hover_background),
                ),
                ("border", "menu.border.color", to_color(menu_t.border.color)),
            ],
            &[
                ("item height", "menu.row_height, on the item button"),
                ("item label size", "menu.font.size"),
            ],
            &[
                (
                    "label colour",
                    "menu_bar::Style has no text colour; the items are our own \
                     widgets, so they take styles::button",
                ),
                (
                    "shadows",
                    "the model has no shadow geometry — iced_aw's own",
                ),
                ("menu width", "MenuTheme states none — the showcase's own"),
            ],
        ),
        column![
            text("MenuBar and its Menus").size(ts.dialog_title.size),
            menu_bar,
        ]
        .spacing(sp.s)
        .into(),
    );

    // ---- ContextMenu ----

    let context_underlay = container(
        column![
            text("Right-click inside this panel").size(ts.section_heading.size),
            text(
                "ContextMenu gets no styles::aw function: its own Style is a one-field \
                 backdrop scrim and its default class already emits alpha 0 \
                 (style/context_menu.rs:48-59). The popup below is our own element."
            )
            .size(ts.caption.size),
        ]
        .spacing(sp.xs),
    )
    .padding(Padding::from(sp.l))
    .style(styles::container_card(resolved))
    .width(Fill);

    let context_demo = ContextMenu::new(context_underlay, move || {
        let entry = |label: &'static str| -> Element<'_, Message> {
            button(text(label).size(ts.caption.size))
                .on_press(Message::AwActionChosen(format!("Context menu: {label}")))
                .style(styles::button(resolved))
                .width(Fill)
                .padding(Padding::from([sp.xxs, sp.s]))
                .into()
        };
        container(column![entry("Copy"), entry("Paste"), entry("Select all")].spacing(sp.xxs))
            .padding(Padding::from(sp.xs))
            .style(styles::container_card(resolved))
            .width(Length::Fixed(180.0))
            .into()
    });

    // ---- TabBar and Tabs ----

    let tab_bar = TabBar::new(Message::AwTabBarSelected)
        .push(0usize, TabLabel::Text("Overview".to_string()))
        .push(1usize, TabLabel::Text("Details".to_string()))
        .push(2usize, TabLabel::Text("About".to_string()))
        .set_active_tab(&state.aw_tab_bar_active)
        .text_size(tab_t.font.size)
        .tab_width(Length::Fixed(tab_t.min_width))
        .height(Length::Fixed(tab_t.min_height))
        .spacing(sp.xxs);

    let tab_bar_body = text(match state.aw_tab_bar_active {
        0 => "A stand-alone TabBar reports the selection and shows nothing itself.",
        1 => "The second tab. Its label colour is tab.active_text_color.",
        _ => "An unselected tab is Status::Disabled to iced_aw, not a dead one.",
    })
    .size(ts.caption.size);

    let tabs = Tabs::new(Message::AwTabsSelected)
        .push(
            0usize,
            TabLabel::Text("Colours".to_string()),
            container(
                text("Tabs owns its content and forwards the bar's style to the TabBar it holds.")
                    .size(ts.caption.size),
            )
            .padding(Padding::from(sp.s)),
        )
        .push(
            1usize,
            TabLabel::Text("Sizes".to_string()),
            container(
                text(format!(
                    "tab.min_width {:.0}px · tab.min_height {:.0}px",
                    tab_t.min_width, tab_t.min_height
                ))
                .size(ts.caption.size),
            )
            .padding(Padding::from(sp.s)),
        )
        .set_active_tab(&state.aw_tabs_active)
        .text_size(tab_t.font.size)
        .tab_bar_height(Length::Fixed(tab_t.min_height))
        .tab_bar_style(styles::aw::tab_bar(resolved))
        .height(Length::Shrink);

    let tab_demo = hoverable(
        widget_tooltip(
            "TabBar / Tabs",
            &[
                (
                    "strip",
                    "tab.bar_background",
                    to_color(tab_t.bar_background),
                ),
                (
                    "selected tab",
                    "tab.active_background",
                    to_color(tab_t.active_background),
                ),
                (
                    "selected label",
                    "tab.active_text_color",
                    to_color(tab_t.active_text_color),
                ),
                (
                    "hovered tab",
                    "tab.hover_background",
                    to_color(tab_t.hover_background.unwrap_or(tab_t.background_color)),
                ),
            ],
            &[
                ("label size", "tab.font.size"),
                ("tab width", "tab.min_width"),
                ("bar height", "tab.min_height"),
            ],
            &[(
                "min_width / min_height",
                "the platform states minima and iced_aw takes fixed lengths, so a \
                 tab here is exactly its minimum",
            )],
        ),
        column![
            text("TabBar (stand-alone) and Tabs (with content)").size(ts.dialog_title.size),
            tab_bar.style(styles::aw::tab_bar(resolved)),
            tab_bar_body,
            tabs,
        ]
        .spacing(sp.s)
        .into(),
    );

    // ---- Sidebar ----

    let side_bar = Sidebar::new(Message::AwSidebarSelected)
        // A `Sidebar` declares a `TabLabel` of its own (sidebar/sidebar.rs:51).
        .push(
            0usize,
            iced_aw::sidebar::TabLabel::Text("General".to_string()),
        )
        .push(
            1usize,
            iced_aw::sidebar::TabLabel::Text("Appearance".to_string()),
        )
        .push(
            2usize,
            iced_aw::sidebar::TabLabel::Text("Icons".to_string()),
        )
        .set_active_tab(&state.aw_sidebar_active)
        .text_size(side_t.font.size)
        .width(Length::Fixed(200.0))
        .height(Length::Shrink)
        .style(styles::aw::sidebar(resolved));

    let side_demo = hoverable(
        widget_tooltip(
            "Sidebar",
            &[
                (
                    "panel",
                    "sidebar.background_color",
                    to_color(side_t.background_color),
                ),
                (
                    "selected item",
                    "sidebar.selection_background",
                    to_color(side_t.selection_background),
                ),
                (
                    "selected label",
                    "sidebar.selection_text_color",
                    to_color(side_t.selection_text_color),
                ),
                (
                    "hovered item",
                    "sidebar.hover_background",
                    to_color(side_t.hover_background),
                ),
            ],
            &[("label size", "sidebar.font.size")],
            &[(
                "corner radius",
                "sidebar::Style carries none but the close icon's; iced_aw rounds \
                 the panel with a hardcoded 0",
            )],
        ),
        column![
            text("Sidebar").size(ts.dialog_title.size),
            row![
                side_bar,
                container(
                    text(match state.aw_sidebar_active {
                        0 =>
                            "General: an unselected item shows the panel itself — the \
                              platform states no fill of its own for one.",
                        1 => "Appearance: the selected item is sidebar.selection_background.",
                        _ => "Icons: hovering an item paints sidebar.hover_background.",
                    })
                    .size(ts.caption.size),
                )
                .padding(Padding::from(sp.l))
                .style(styles::container_card(resolved))
                .width(Fill),
            ]
            .spacing(sp.s),
        ]
        .spacing(sp.s)
        .into(),
    );

    // ---- Spinner ----

    let spinner_demo = hoverable(
        widget_tooltip(
            "Spinner",
            &[("arc", "spinner.fill_color", to_color(spin_t.fill_color))],
            &[("diameter", "spinner.diameter")],
            &[
                (
                    "Style",
                    "iced_aw 0.14.1 gives Spinner none at all: it paints in the \
                     inherited text colour, which the wrapping container states",
                ),
                (
                    "stroke width",
                    "spinner.stroke_width has no receiver; circle_radius sizes the \
                     orbiting dot, not an arc",
                ),
            ],
        ),
        column![
            text("Spinner (styled through its container)").size(ts.dialog_title.size),
            container(
                Spinner::new()
                    .width(Length::Fixed(spin_t.diameter))
                    .height(Length::Fixed(spin_t.diameter)),
            )
            .style(styles::aw::spinner(resolved)),
        ]
        .spacing(sp.s)
        .into(),
    );

    // ---- SelectionList ----

    // `SelectionList::new_with` is the only constructor that gives the rows a
    // class as well as the list, and it demands a `Clone` style function; the
    // connector's returns an opaque type that is not known to be `Clone`, so
    // an `Rc` carries it.
    let list_style = Rc::new(styles::aw::selection_list(resolved));
    let selection_list = SelectionList::new_with(
        &state.aw_list_options,
        Message::AwListSelected,
        list_t.item_font.size,
        Padding::from(sp.xs),
        move |theme: &Theme, status: Status| list_style(theme, status),
        state.aw_list_selected,
        iced::Font::DEFAULT,
    )
    .width(Length::Fixed(260.0))
    .height(Length::Fixed(180.0));

    let list_demo = hoverable(
        widget_tooltip(
            "SelectionList",
            &[
                (
                    "list",
                    "list.background_color",
                    to_color(list_t.background_color),
                ),
                (
                    "selected row",
                    "list.selection_background",
                    to_color(list_t.selection_background),
                ),
                (
                    "hovered row",
                    "list.hover_background",
                    to_color(list_t.hover_background),
                ),
                (
                    "row label",
                    "list.item_font.color",
                    to_color(list_t.item_font.color),
                ),
            ],
            &[("row label size", "list.item_font.size")],
            &[(
                "list.row_height",
                "no receiver in iced_aw 0.14.1: a row is text_size + padding \
                 (selection_list/list.rs:209), and there is no item_height setter",
            )],
        ),
        column![
            text("SelectionList").size(ts.dialog_title.size),
            selection_list,
        ]
        .spacing(sp.s)
        .into(),
    );

    let action_line = if state.aw_last_action.is_empty() {
        "Nothing chosen yet. Use the menu bar, the context menu or the list.".to_string()
    } else {
        state.aw_last_action.clone()
    };

    column![
        header,
        text(action_line).size(ts.caption.size),
        card_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        menu_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        column![text("ContextMenu").size(ts.dialog_title.size), context_demo,].spacing(sp.s),
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        tab_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        side_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        spinner_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        list_demo,
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
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
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
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
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
                    resolved,
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
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        animated_section,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved))
    ]
    .spacing(sp.l);
    for r in grid_rows {
        content = content.push(r);
    }

    content.width(Fill).into()
}

fn view_animated_icons<'a>(state: &'a State, fg_color: Color) -> Element<'a, Message> {
    let sp = &SP;
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let icon_px = resolved.defaults.icon_sizes.large;
    let section_title = text("Animated Icons").size(ts.display.size);
    let divider = rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved));

    // Collect spinner columns into a row
    let mut spinners: Vec<Element<'a, Message>> = Vec::new();

    if state.reduced_motion {
        // Reduced motion: show static first-frame for each animated icon
        for (set_name, handle) in &state.animated_static {
            let icon = svg(handle.clone())
                .width(Length::Fixed(icon_px))
                .height(Length::Fixed(icon_px))
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
                .width(Length::Fixed(icon_px))
                .height(Length::Fixed(icon_px))
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
                .width(Length::Fixed(icon_px))
                .height(Length::Fixed(icon_px))
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
    resolved: &ResolvedTheme,
    fg_color: Color,
    caption_size: f32,
    heading_size: f32,
    xxs_spacing: f32,
) -> Element<'a, Message> {
    let role_name = format!("{:?}", loaded.role);
    let icon_name_str = loaded.name.unwrap_or("(unmapped)");
    let source_label = loaded.source.label();
    let icon_px = resolved.defaults.icon_sizes.toolbar;
    let cell_px = resolved.defaults.icon_sizes.large;

    let icon_element: Element<'a, Message> = match &loaded.data {
        Some(data @ IconData::Svg(_)) => {
            if loaded.source == IconSource::System {
                // System icons: render as-is without colorization
                match native_theme_iced::icons::to_svg_handle(data, None) {
                    Some(handle) => svg(handle)
                        .width(Length::Fixed(icon_px))
                        .height(Length::Fixed(icon_px))
                        .into(),
                    None => placeholder_icon(heading_size, icon_px),
                }
            } else {
                // Bundled/fallback: colorize with theme foreground
                match native_theme_iced::icons::to_svg_handle(data, Some(fg_color)) {
                    Some(handle) => svg(handle)
                        .width(Length::Fixed(icon_px))
                        .height(Length::Fixed(icon_px))
                        .into(),
                    None => placeholder_icon(heading_size, icon_px),
                }
            }
        }
        Some(data @ IconData::Rgba { .. }) => {
            match native_theme_iced::icons::to_image_handle(data) {
                Some(handle) => iced::widget::image(handle)
                    .width(Length::Fixed(icon_px))
                    .height(Length::Fixed(icon_px))
                    .into(),
                None => placeholder_icon(heading_size, icon_px),
            }
        }
        _ => placeholder_icon(heading_size, icon_px),
    };

    let info = format!("{role_name}\nicon: {icon_name_str}\nsource: {source_label}");

    // Wrap in mouse_area for Widget Info hover
    mouse_area(
        container(
            column![
                container(icon_element)
                    .center_x(Length::Fixed(cell_px))
                    .center_y(Length::Fixed(cell_px)),
                text(role_name.clone()).size(caption_size),
                text(source_label).size(caption_size),
            ]
            .spacing(xxs_spacing)
            .align_x(iced::Center),
        )
        .padding(Padding::from(SP.xs))
        .style(styles::container_card(resolved))
        .width(Length::Fixed(100.0)),
    )
    .on_enter(Message::WidgetHovered(info))
    .on_exit(Message::WidgetUnhovered)
    .into()
}

fn placeholder_icon<'a>(size: f32, box_size: f32) -> Element<'a, Message> {
    container(text("?").size(size))
        .center_x(Length::Fixed(box_size))
        .center_y(Length::Fixed(box_size))
        .into()
}

// ---------------------------------------------------------------------------
// Tab: Theme Map
// ---------------------------------------------------------------------------

fn view_theme_map(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let header = section_header(
        "Theme Map",
        "All palette and extended palette colors from the current theme",
        resolved,
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
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_background,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_primary,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_secondary,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_success,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_warning,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        ext_danger,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
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

/// The drop-down arrow at the platform's own icon size.
///
/// `combo_box.arrow_icon_size` is not a `pick_list::Style` field: iced carries
/// the arrow in the `Handle` the widget is built with
/// (`pick_list.rs:794-801`), so it is set here rather than in
/// `styles::pick_list`.
fn arrow_handle(resolved: &ResolvedTheme) -> pick_list::Handle<iced::Font> {
    pick_list::Handle::Arrow {
        size: Some(resolved.combo_box.arrow_icon_size.into()),
    }
}

fn section_header<'a>(
    title: &'a str,
    description: &'a str,
    resolved: &ResolvedTheme,
    ts: &native_theme::theme::ResolvedTextScale,
    sp: &Spacing,
) -> Element<'a, Message> {
    column![
        text(title).size(ts.display.size),
        text(description).size(ts.section_heading.size),
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
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
