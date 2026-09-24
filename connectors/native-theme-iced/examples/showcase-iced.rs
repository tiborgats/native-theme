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
    AnimatedIcon, IconData, IconRole, IconSet, LayoutTheme, ResolvedTheme, TransformAnimation,
};
use native_theme_iced::icons::{
    AnimatedSvgHandles, animated_frames_to_svg_handles, spin_rotation_radians, to_svg_handle,
};
use native_theme_iced::palette::to_color;
use native_theme_iced::styles;
use std::borrow::Cow;
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

/// The window `main` opens, and the viewport `every_tab_renders` lays out in.
const WINDOW_SIZE: (f32, f32) = (1060.0, 750.0);

/// Tags a widget so the self-tests can find it, and does nothing otherwise.
///
/// `iced_selector` sees only the widgets that implement `Widget::operate`, and
/// `slider`, `radio`, `toggler`, `pick_list`, `combo_box` and `text_editor`
/// implement none, so no selector can reach them. `container` is the one
/// widget in the showcase's set that carries a `widget::Id` and reports it
/// with its own bounds (`container.rs:109`, `:280-283`), which is what
/// `iced_test`'s `id` selector matches.
///
/// Outside `cargo test` the wrapper is not built at all, so the interface the
/// showcase draws -- and the screenshots taken from it -- are exactly what
/// they were.
#[cfg(test)]
fn probe<'a>(
    id: &'static str,
    width: Length,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content).id(id).width(width).into()
}

#[cfg(not(test))]
fn probe<'a>(
    _id: &'static str,
    _width: Length,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    content.into()
}

/// The [`probe`] tags, shared by the render code and by
/// `interactive_controls_respond`.
mod probes {
    pub const THEME: &str = "probe-theme";
    pub const COLOR_MODE: &str = "probe-color-mode";
    pub const TEXT_EDITOR: &str = "probe-text-editor";
    pub const RADIO_APPLE: &str = "probe-radio-apple";
    pub const RADIO_BANANA: &str = "probe-radio-banana";
    pub const RADIO_CHERRY: &str = "probe-radio-cherry";
    pub const TOGGLER: &str = "probe-toggler";
    pub const PICK_LIST: &str = "probe-pick-list";
    pub const COMBO_BOX: &str = "probe-combo-box";
    pub const SLIDER: &str = "probe-slider";
    pub const VERTICAL_SLIDER: &str = "probe-vertical-slider";
    #[cfg(feature = "iced_aw")]
    pub const SELECTION_LIST: &str = "probe-selection-list";
}

/// The `widget::Id` of the single-line `text_input`.
///
/// `text_input` carries an id of its own (`text_input.rs:157`), so it needs no
/// [`probe`]; the two inputs share one value, and this one is the one the
/// self-tests type into.
const TEXT_INPUT_ID: &str = "showcase-text-input";

/// The four layout distances the platform itself states.
///
/// `LayoutTheme` is the one theme struct that is not per-variant, so it lives
/// on the native `Theme` and on `SystemTheme` rather than on `ResolvedTheme`
/// (`native-theme/src/model/widgets/mod.rs:884`). The showcase therefore keeps
/// a copy of it beside the resolved theme and refreshes it whenever the theme
/// is rebuilt.
///
/// Every field of `LayoutTheme` is an `Option`: `None` is the platform saying
/// it states no such distance, and nothing is invented for it -- the
/// showcase's own [`Spacing`] constant stands in, and the Theme Config
/// Inspector says which of the two is on screen.
struct Gaps {
    /// Space between adjacent widgets, `layout.widget_gap`.
    widget: f32,
    /// Padding inside a container, `layout.container_margin`.
    container: f32,
    /// Padding inside the main window, `layout.window_margin`.
    window: f32,
    /// Space between major content sections, `layout.section_gap`.
    section: f32,
}

impl Gaps {
    fn from_layout(layout: &LayoutTheme) -> Self {
        Self {
            widget: layout.widget_gap.unwrap_or(SP.s),
            container: layout.container_margin.unwrap_or(SP.l),
            window: layout.window_margin.unwrap_or(SP.l),
            section: layout.section_gap.unwrap_or(SP.xl),
        }
    }
}

/// How one layout distance reads in the Theme Config Inspector: the platform's
/// value, or the showcase constant that stood in for it.
fn layout_value(stated: Option<f32>, fallback: f32) -> String {
    match stated {
        Some(v) => format!("{v:.0}px"),
        None => format!("{fallback:.0}px (showcase)"),
    }
}

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

    /// The colour mode `--variant` names: `light`, `dark`, or `system`,
    /// which follows the OS.
    fn color_mode(variant: &str) -> Result<AppColorMode, String> {
        match variant {
            "light" => Ok(AppColorMode::Light),
            "dark" => Ok(AppColorMode::Dark),
            "system" => Ok(AppColorMode::System),
            other => Err(format!("--variant {other}: not light, dark or system")),
        }
    }

    /// The theme `--theme` names: `default`, the OS theme, or a preset the
    /// theme picker offers on this platform (`theme_choices`), which offers
    /// only the platform's own presets and the community ones.
    fn theme_choice(name: &str, default_label: &str) -> Result<ThemeChoice, String> {
        if name == "default" {
            return Ok(ThemeChoice::OsTheme(default_label.to_string()));
        }
        let offered = theme_choices(default_label);
        let choice = ThemeChoice::Preset(name.to_string());
        if offered.contains(&choice) {
            return Ok(choice);
        }
        let names: Vec<String> = offered
            .iter()
            .map(|choice| match choice {
                ThemeChoice::OsTheme(_) => "default".to_string(),
                ThemeChoice::Preset(key) => key.clone(),
            })
            .collect();
        let names = names.join(", ");
        if native_theme::theme::Theme::list_presets()
            .iter()
            .any(|info| info.key == name)
        {
            Err(format!(
                "--theme {name}: a preset of another platform; only this platform's \
                 presets run here: {names}"
            ))
        } else {
            Err(format!(
                "--theme {name}: no such preset; the presets are: {names}"
            ))
        }
    }

    /// The icon set `--icon-set` names, which the icon-theme picker offers
    /// (`build_icon_choices`): a bundled set, `system` or `freedesktop` (the
    /// system icon theme), or one of `installed_themes`, the freedesktop
    /// themes the picker lists.
    fn icon_set_choice(name: &str, installed_themes: &[String]) -> Result<IconSetChoice, String> {
        match name {
            "material" => Ok(IconSetChoice::Material),
            "lucide" => Ok(IconSetChoice::Lucide),
            "system" | "freedesktop" => Ok(IconSetChoice::System),
            theme if installed_themes.iter().any(|installed| installed == theme) => {
                Ok(IconSetChoice::Freedesktop(theme.to_string()))
            }
            other => Err(format!(
                "--icon-set {other}: not material, lucide, system, freedesktop or an \
                 installed icon theme; {}",
                if installed_themes.is_empty() {
                    "none are installed".to_string()
                } else {
                    format!(
                        "the installed icon themes are: {}",
                        installed_themes.join(", ")
                    )
                }
            )),
        }
    }

    /// The tab `--tab` names: a tab's [`Tab::flag`], or `textinputs` or
    /// `thememap` for the two tabs whose name has a hyphen.
    fn parse_tab(name: &str) -> Result<Tab, String> {
        let flag = match name {
            "textinputs" => "text-inputs",
            "thememap" => "theme-map",
            other => other,
        };
        Tab::ALL
            .iter()
            .copied()
            .find(|tab| tab.flag() == flag)
            .ok_or_else(|| {
                let tabs: Vec<&str> = Tab::ALL.iter().map(|tab| tab.flag()).collect();
                format!(
                    "--tab {name}: no such tab; the tabs are: {}",
                    tabs.join(", ")
                )
            })
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
    /// Every tab, in declaration order.
    ///
    /// `every_tab_renders` (spec §6.2) iterates this list, so a tab missing
    /// from it is a tab nothing renders under test. The `const` block below
    /// rejects a build where the list is out of order or stops short of the
    /// last declared variant; what no stable construct catches is a variant
    /// appended *after* `ThemeMap` and left out of the list, which `label` and
    /// `view`'s exhaustive matches still force the author to write.
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

    /// The name `--tab` takes for the tab.
    fn flag(self) -> &'static str {
        match self {
            Tab::Buttons => "buttons",
            Tab::TextInputs => "text-inputs",
            Tab::Selection => "selection",
            Tab::Range => "range",
            Tab::Display => "display",
            Tab::Layout => "layout",
            Tab::Graphics => "graphics",
            #[cfg(feature = "iced_aw")]
            Tab::Extra => "extra",
            Tab::Icons => "icons",
            Tab::ThemeMap => "theme-map",
        }
    }

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

const _: () = {
    assert!(
        Tab::ALL.len() == Tab::ThemeMap as usize + 1,
        "Tab::ALL is missing a variant"
    );
    let mut i = 0;
    while i < Tab::ALL.len() {
        assert!(
            Tab::ALL[i] as usize == i,
            "Tab::ALL must list the variants in declaration order"
        );
        i += 1;
    }
};

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
fn load_adwaita_fallback(
    is_dark: bool,
) -> Option<(native_theme::theme::ResolvedTheme, Theme, LayoutTheme)> {
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
    Some((r, t, nt.layout.clone()))
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

/// Build the animation caches for the spinner of `icon_set`: for a
/// freedesktop set, the spinner of `freedesktop_theme` (the system's where
/// `None`), so the page never shows another theme's spinner; none where the
/// theme has none.
///
/// Returns the full set of animation state fields that go into `State`.
#[allow(clippy::type_complexity)]
fn build_animation_caches(
    icon_set: native_theme::theme::IconSet,
    freedesktop_theme: Option<&str>,
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
        let indicator = match icon_set {
            IconSet::Freedesktop => FreedesktopLoader::load_indicator(freedesktop_theme),
            other => load_icon_indicator(other),
        };
        if let Some(anim) = indicator {
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
    /// The current theme's layout spacing. Not part of `ResolvedTheme`: it is
    /// shared between the light and the dark variant, so it sits on the
    /// native `Theme` and on `SystemTheme` instead.
    layout: LayoutTheme,
    /// OS accessibility preferences (from SystemTheme, not ResolvedTheme).
    accessibility: native_theme_iced::AccessibilityPreferences,
    /// Icon set for the current theme (from Theme or SystemTheme, not ResolvedTheme).
    current_icon_set: IconSet,
    /// Icon theme name for the current theme; `None` where the theme names
    /// none and the system's cannot be detected.
    current_icon_theme: Option<String>,
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
    /// Whether `icon_set_choice` is re-derived from each theme installed:
    /// until the user picks an icon theme, and after a pick of the
    /// `default` row. Not `IconSetChoice::follows_preset`: a choice that
    /// followed the preset onto `system`, where the preset's own icon theme
    /// is not installed (`default_icon_choice`), keeps following it.
    icon_choice_follows_preset: bool,
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
    /// Reads the OS theme at startup and when the `default` entry is
    /// installed after it: `SystemTheme::from_system`, which a test replaces
    /// with a read that fails.
    read_system_theme: fn() -> native_theme::Result<native_theme::SystemTheme>,
    /// Detects the system icon theme the Icons page names:
    /// `system_icon_theme`, which a test replaces with a detection that
    /// fails.
    detect_icon_theme: fn() -> native_theme::Result<String>,

    // Theme watcher (runtime dark/light toggle detection)
    /// Flag set by the ThemeSubscription background thread when the OS theme changes.
    theme_change_flag: Arc<AtomicBool>,
    /// RAII guard keeping the theme watcher background thread alive.
    _theme_watcher: Option<native_theme::watch::ThemeSubscription>,
}

impl Default for State {
    fn default() -> Self {
        Self::starting_with(native_theme::SystemTheme::from_system)
    }
}

impl State {
    /// The showcase at startup, with the OS theme `read_system_theme` reads
    /// installed, or adwaita where that read fails.
    fn starting_with(
        read_system_theme: fn() -> native_theme::Result<native_theme::SystemTheme>,
    ) -> Self {
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
            layout,
            fallback_choice,
        ) = match read_system_theme() {
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
                let it = system.icon_theme.map(Cow::into_owned);
                let acc = system.accessibility;
                let lay = system.layout.clone();
                (r, t, None, preset, is, it, acc, lay, None)
            }
            Err(e) => {
                // Fallback: load adwaita preset through resolve pipeline. The
                // theme picker names what is drawn, so a mode change installs
                // adwaita again instead of reading the OS theme.
                match load_adwaita_fallback(is_dark) {
                    Some((r, t, lay)) => (
                        r,
                        t,
                        Some(format!("OS theme failed: {e}. Using adwaita fallback.")),
                        "adwaita".to_string(),
                        IconSet::Freedesktop,
                        Some("Adwaita".to_string()),
                        native_theme_iced::AccessibilityPreferences::default(),
                        lay,
                        Some(ThemeChoice::Preset("adwaita".to_string())),
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
        let init_icon_theme_opt: Option<&str> = init_icon_theme.as_deref();
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
        ) = build_animation_caches(anim_set, icon_set_choice.freedesktop_theme());

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
        // Skip under `cargo test` too: a file watch on the desktop's own
        // configuration is host I/O, and nothing the self-tests assert
        // depends on it.
        let theme_change_flag = Arc::new(AtomicBool::new(false));
        let is_screenshot = CLI_ARGS.get().is_some_and(|cli| cli.screenshot.is_some());
        let _theme_watcher = if is_screenshot || cfg!(test) {
            None
        } else {
            let flag_clone = theme_change_flag.clone();
            native_theme::watch::on_theme_change(move |_event| {
                flag_clone.store(true, Ordering::Release);
            })
            .ok()
        };

        let mut state = Self {
            current_choice: fallback_choice
                .unwrap_or_else(|| ThemeChoice::OsTheme(default_label.clone())),
            current_theme: theme,
            color_mode,
            is_dark,
            current_resolved: resolved,
            layout,
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
            icon_choice_follows_preset: true,
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
            read_system_theme,
            detect_icon_theme: native_theme::theme::system_icon_theme,
            theme_change_flag,
            _theme_watcher,
        };

        if let Some(cli) = CLI_ARGS.get() {
            apply_cli_args(&mut state, cli);
        }

        state
    }
}

/// Put the showcase into the state the command line asks for: its colour
/// mode, theme, tab and icons.
///
/// Each flag takes exactly what the matching picker offers. A value the
/// showcase cannot honour is reported on stderr and ignored: the setting
/// stays what it would have been without the flag. `--variant` installs
/// the theme in the mode it names: the one `--theme` names, or, where
/// `--theme` is absent or rejected here, the one installed. A `--theme`
/// accepted here that then fails to load changes neither the theme nor the
/// mode, `--variant`'s included: the install is all or nothing
/// (`State::install_in_mode`).
fn apply_cli_args(state: &mut State, cli: &CliArgs) {
    let mode = cli
        .variant
        .as_deref()
        .and_then(|variant| reported(CliArgs::color_mode(variant)));
    let choice = cli
        .theme
        .as_deref()
        .and_then(|name| reported(CliArgs::theme_choice(name, &state.default_label)));
    if mode.is_some() || choice.is_some() {
        let choice = choice.unwrap_or_else(|| state.current_choice.clone());
        let mode = mode.unwrap_or(state.color_mode);
        state.install_in_mode(choice, mode);
    }

    // Override tab
    if let Some(tab) = cli
        .tab
        .as_deref()
        .and_then(|name| reported(CliArgs::parse_tab(name)))
    {
        state.active_tab = tab;
    }

    // `--icon-set` names a set, which stays chosen across theme switches as
    // a pick in the icon-theme picker does.
    if let Some(choice) = cli
        .icon_set
        .as_deref()
        .and_then(|name| reported(CliArgs::icon_set_choice(name, &state.installed_themes)))
    {
        state.choose_icon_set(choice);
    }

    // Apply screenshot settings
    if let Some(ref path) = cli.screenshot {
        state.screenshot_path = Some(path.clone());
        state.screenshot_countdown = 60; // 60 ticks × 50ms = 3s render delay
    }
}

/// The value of `result`; its error reported on stderr, and `None`.
fn reported<T>(result: Result<T, String>) -> Option<T> {
    result.map_err(|error| eprintln!("{error}; ignored")).ok()
}

impl State {
    /// Install `choice` in `mode`: every theme and colour-mode switch
    /// installs through here. The pickers then show `choice` and `mode`.
    ///
    /// A theme that fails to load leaves the installed one on screen, and
    /// the pickers showing it and the mode it is drawn in, with the error in
    /// the banner; false is returned.
    fn install_in_mode(&mut self, choice: ThemeChoice, mode: AppColorMode) -> bool {
        let is_dark = mode.is_dark();
        let icon_theme = match self.load_theme(&choice, is_dark) {
            Ok(icon_theme) => icon_theme,
            Err(error) => {
                self.error_message = Some(error);
                return false;
            }
        };
        self.current_choice = match choice {
            ThemeChoice::OsTheme(_) => ThemeChoice::OsTheme(self.default_label.clone()),
            preset @ ThemeChoice::Preset(_) => preset,
        };
        self.color_mode = mode;
        self.is_dark = is_dark;

        // Only a choice that follows the preset is re-derived; the user's
        // pick of an icon theme stays chosen across theme switches.
        let it_opt = icon_theme.as_deref();
        if self.icon_choice_follows_preset {
            self.icon_set_choice = default_icon_choice(self.current_icon_set, it_opt);
        }
        // Always rebuild the choices list (theme name in "default (X)" may have changed).
        self.icon_set_choices =
            build_icon_choices(self.current_icon_set, it_opt, &self.installed_themes);

        // Always reload icons — the resolved text_color may have changed (light↔dark)
        // even when the icon set choice is the same.
        self.reload_icons();
        true
    }

    /// Install the current theme again in the current mode.
    fn rebuild_theme(&mut self) {
        self.install_in_mode(self.current_choice.clone(), self.color_mode);
    }

    /// Load `choice` in the mode `is_dark` names into the theme fields.
    ///
    /// `Ok` holds the icon theme the theme states (`None` where its TOML
    /// states none); `Err` the error, with the theme fields untouched.
    fn load_theme(
        &mut self,
        choice: &ThemeChoice,
        is_dark: bool,
    ) -> Result<Option<String>, String> {
        let mode = if is_dark {
            native_theme_iced::ColorMode::Dark
        } else {
            native_theme_iced::ColorMode::Light
        };
        match choice {
            ThemeChoice::OsTheme(_) => match (self.read_system_theme)() {
                Ok(system) => {
                    // Platform presets always specify icon_theme.
                    self.current_icon_set = system.icon_set;
                    self.accessibility = system.accessibility.clone();
                    self.layout = system.layout.clone();
                    self.current_icon_theme = system.icon_theme.clone().map(Cow::into_owned);
                    self.current_resolved = system.pick(mode).clone();
                    self.current_theme =
                        native_theme_iced::to_theme(&self.current_resolved, &system.name);
                    self.default_label = format!("default ({})", system.preset);
                    self.error_message = None;
                    Ok(self.current_icon_theme.clone())
                }
                // Only startup falls back to adwaita, with nothing installed
                // yet; here the installed theme stays.
                Err(e) => Err(format!("Failed to load OS theme: {e}")),
            },
            ThemeChoice::Preset(name) => {
                let nt = native_theme::theme::Theme::preset(name)
                    .map_err(|e| format!("Failed to load preset '{name}': {e}"))?;
                let r = nt
                    .resolve(mode)
                    .map_err(|e| format!("Theme '{name}' resolution failed: {e}"))?;
                let icon_theme_explicit = r.icon_theme_explicit;
                self.current_icon_theme = r.icon_theme.map(Cow::into_owned);
                let icon_theme = self
                    .current_icon_theme
                    .clone()
                    .filter(|_| icon_theme_explicit);
                self.current_icon_set = r.icon_set;
                self.current_resolved = r.variant;
                self.layout = nt.layout.clone();
                self.current_theme = native_theme_iced::to_theme(&self.current_resolved, &nt.name);
                self.error_message = None;
                Ok(icon_theme)
            }
        }
    }

    /// The Icons page's line for the system icon theme: its name, or why
    /// none was detected -- then no system icon loads, and no other theme
    /// is named in its place.
    fn system_icon_theme_label(&self) -> String {
        match (self.detect_icon_theme)() {
            Ok(theme) => format!("System icon theme: {theme}"),
            Err(e) => format!("System icon theme: unavailable ({e})"),
        }
    }

    /// Load the icons and the spinner of the chosen icon set, recoloured for
    /// the installed theme.
    fn reload_icons(&mut self) {
        self.loaded_icons = load_all_icons(
            &self.icon_set_choice,
            &self.current_resolved,
            self.current_icon_set,
        );
        let anim_set = self
            .icon_set_choice
            .effective_icon_set(self.current_icon_set);
        let (af, afi, afe, asp, astart, rm, ast) =
            build_animation_caches(anim_set, self.icon_set_choice.freedesktop_theme());
        self.animated_frames = af;
        self.animated_frame_indices = afi;
        self.animated_frame_elapsed = afe;
        self.animated_spins = asp;
        self.animation_start = astart;
        self.reduced_motion = rm;
        self.animated_static = ast;
    }

    /// Choose `choice`, the user's pick of an icon theme: only its
    /// `default` row follows the preset from now on.
    fn choose_icon_set(&mut self, choice: IconSetChoice) {
        self.icon_choice_follows_preset = choice.follows_preset();
        self.icon_set_choice = choice;
        self.reload_icons();
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
            let mode = state.color_mode;
            state.install_in_mode(choice, mode);
        }
        Message::ColorModeSelected(mode) => {
            let choice = state.current_choice.clone();
            state.install_in_mode(choice, mode);
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
        Message::IconSetSelected(choice) => state.choose_icon_set(choice),
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
    let gap = Gaps::from_layout(&state.layout);
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
            probe(
                probes::THEME,
                Fill,
                pick_list(
                    theme_choices(&state.default_label),
                    Some(&state.current_choice),
                    Message::ThemeSelected,
                )
                .handle(arrow_handle(resolved))
                .style(styles::pick_list(resolved))
                .menu_style(styles::menu(resolved))
                .width(Fill),
            ),
        ]
        .spacing(sp.xs);

        // Color mode selector (System / Light / Dark)
        let color_mode_section = column![
            text("Color Mode").size(ts.caption.size),
            probe(
                probes::COLOR_MODE,
                Fill,
                pick_list(
                    AppColorMode::ALL.to_vec(),
                    Some(&state.color_mode),
                    Message::ColorModeSelected,
                )
                .handle(arrow_handle(resolved))
                .style(styles::pick_list(resolved))
                .menu_style(styles::menu(resolved))
                .width(Fill),
            ),
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

        // Theme config inspector (the gpui showcase's counterpart: its inspector's Theme tab)
        let fi = format_font_info(&state.current_resolved);
        let metrics_info = {
            let r = format!("radius: {radius:.0}px");
            let rlg = format!(
                "radius_lg: {:.0}px",
                native_theme_iced::border_radius_lg(&state.current_resolved)
            );
            let sw = format!("scrollbar: {sb_width:.0}px");
            // Each side as button_padding / input_padding return it: the
            // theme's where it states the side, iced's own default where not.
            let sides =
                |p: Padding| format!("{:.0} {:.0} {:.0} {:.0}", p.top, p.right, p.bottom, p.left);
            let bp = format!("btn pad (t r b l): {}", sides(btn_pad));
            let ip = format!("input pad (t r b l): {}", sides(inp_pad));
            // The four LayoutTheme distances, and which of them the platform
            // leaves to the showcase's own scale.
            let lay = format!(
                "widget gap: {}\ncontainer margin: {}\nwindow margin: {}\nsection gap: {}",
                layout_value(state.layout.widget_gap, SP.s),
                layout_value(state.layout.container_margin, SP.l),
                layout_value(state.layout.window_margin, SP.l),
                layout_value(state.layout.section_gap, SP.xl),
            );
            column![
                text("Theme Config Inspector").size(ts.caption.size),
                text(r).size(ts.caption.size),
                text(rlg).size(ts.caption.size),
                text(sw).size(ts.caption.size),
                text(bp).size(ts.caption.size),
                text(ip).size(ts.caption.size),
                text(lay).size(ts.caption.size),
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
                .spacing(gap.widget)
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
        let tab_pad =
            native_theme_iced::padding_or(&resolved.tab.border.padding, button::DEFAULT_PADDING);
        let tabs: Vec<Element<'_, Message>> = Tab::ALL
            .iter()
            .map(|&tab| {
                let label = tab.label();
                // A tab is padded like the platform's tabs: the sides
                // tab.border.padding states, a button's own elsewhere.
                let btn = button(text(label).size(ts.caption.size)).padding(tab_pad);
                // The open tab is the call to action; the rest are plain.
                let btn = if tab == state.active_tab {
                    btn.style(styles::button_primary(resolved))
                } else {
                    btn.style(styles::button(resolved))
                };
                btn.on_press(Message::TabSelected(tab)).into()
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
    // The strip's top and right sit against the window's own edges, so those
    // two are the window margin; its left abuts the sidebar
    // (`row![sidebar, right_panel]`), a panel gutter that takes the same value
    // so the strip lines up with the content below it.
    let tab_padding = Padding::ZERO
        .left(gap.window)
        .right(gap.window)
        .top(gap.window);
    let content_padding = Padding::from(gap.window);
    let panel_spacing = sp.xs;
    let mut right_panel = column![].spacing(panel_spacing).width(Fill).height(Fill);

    // Error banner (if any)
    if let Some(ref msg) = state.error_message {
        // The platform's own error colour, not the palette slot iced derives
        // from it.
        let danger = to_color(resolved.defaults.danger_color);
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
// Widget Info builder
// ---------------------------------------------------------------------------

/// Build a multi-line info string for the Widget Info panel, with three of
/// the sections the gpui showcase's `WidgetInfo::to_text` writes (that one
/// adds a "This instance" section and a citation after each colour):
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
    let gap = Gaps::from_layout(&state.layout);
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
                (
                    "padding",
                    "button_padding — button.border.padding's stated sides, \
                     iced's button::DEFAULT_PADDING for the others",
                ),
                ("shadow", "iced's own — the model has no shadow geometry"),
            ],
            &[
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
                    button("Warning")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_warning(resolved))
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
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
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
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
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
    .spacing(gap.widget);

    column![
        header,
        primary_row,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        disabled_row,
        rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
        interactive,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Text Inputs
// ---------------------------------------------------------------------------

fn view_text_inputs<'a>(state: &'a State, inp_pad: Padding) -> Element<'a, Message> {
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
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
            .id(TEXT_INPUT_ID)
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
                &[
                    ("border-radius", &radius_s),
                    (
                        "padding",
                        "input_padding — input.border.padding's stated sides, \
                         iced's text_input::DEFAULT_PADDING for the others",
                    ),
                ],
                &[
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
            .spacing(gap.widget)
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
            .spacing(gap.widget)
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
            probe(
                probes::TEXT_EDITOR,
                Fill,
                text_editor(&state.text_editor_content)
                    .on_action(Message::EditorAction)
                    .style(styles::text_editor(resolved))
                    .height(Length::Fixed(180.0)),
            ),
            text("Supports multi-line editing, selection, and scrolling").size(ts.caption.size),
        ]
        .spacing(gap.widget)
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
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Selection
// ---------------------------------------------------------------------------

fn view_selection(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let c = &resolved.checkbox;
    let sw = &resolved.switch;
    let cb = &resolved.combo_box;
    let checkbox_radius_s = format!("{:.0}px", c.border.corner_radius);
    let combo_radius_s = format!("{:.0}px", cb.border.corner_radius);
    let label_gap_s = format!("{:.0}px", c.label_gap);
    // `checkbox.indicator_width` is the indicator's side length, square for a
    // checkbox and a diameter for a radio (platform-facts.md:980), and both
    // `Checkbox::size` (checkbox.rs:176, laid out at :287) and `Radio::size`
    // (radio.rs:200, :300) take exactly that.
    let indicator_width_s = format!("{:.0}px", c.indicator_width);
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
                ("box size", &indicator_width_s),
            ],
            &[(
                "check mark",
                "Checkbox::icon takes an Icon — a font glyph — so the mark's own \
                 shape is the font's, not the theme's (checkbox.rs:229, :493-504)",
            )],
        ),
        column![
            text("Checkboxes").size(ts.dialog_title.size),
            checkbox(state.checkbox_a)
                .label("Enable notifications")
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .style(styles::checkbox(resolved))
                .on_toggle(Message::CheckboxAToggled),
            checkbox(state.checkbox_b)
                .label("Dark mode auto-detect")
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .style(styles::checkbox(resolved))
                .on_toggle(Message::CheckboxBToggled),
            checkbox(state.checkbox_c)
                .label("Remember preferences")
                .spacing(c.label_gap)
                .size(c.indicator_width)
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
        .spacing(gap.widget)
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
            &[
                ("label gap", &label_gap_s),
                ("indicator diameter", &indicator_width_s),
            ],
            &[
                ("border-radius", "radio::Style carries no corner radius"),
                ("disabled", "radio::Status has no disabled value"),
            ],
        ),
        column![
            text("Radio Buttons").size(ts.dialog_title.size),
            probe(
                probes::RADIO_APPLE,
                Length::Shrink,
                radio(
                    "Apple",
                    Fruit::Apple,
                    state.selected_fruit,
                    Message::FruitSelected
                )
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .style(styles::radio(resolved))
            ),
            probe(
                probes::RADIO_BANANA,
                Length::Shrink,
                radio(
                    "Banana",
                    Fruit::Banana,
                    state.selected_fruit,
                    Message::FruitSelected
                )
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .style(styles::radio(resolved))
            ),
            probe(
                probes::RADIO_CHERRY,
                Length::Shrink,
                radio(
                    "Cherry",
                    Fruit::Cherry,
                    state.selected_fruit,
                    Message::FruitSelected
                )
                .spacing(c.label_gap)
                .size(c.indicator_width)
                .style(styles::radio(resolved))
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
        .spacing(gap.widget)
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
            probe(
                probes::TOGGLER,
                Length::Shrink,
                toggler(state.toggler_enabled)
                    .label("Feature flag enabled")
                    .size(sw.track_height)
                    .style(styles::toggler(resolved))
                    .on_toggle(Message::TogglerToggled)
            ),
            text(format!(
                "State: {}",
                if state.toggler_enabled { "ON" } else { "OFF" }
            ))
            .size(ts.caption.size),
        ]
        .spacing(gap.widget)
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
            probe(
                probes::PICK_LIST,
                Length::Shrink,
                pick_list(
                    languages,
                    state.pick_list_selected.as_ref(),
                    Message::PickListSelected,
                )
                .handle(arrow_handle(resolved))
                .style(styles::pick_list(resolved))
                .menu_style(styles::menu(resolved))
                .width(Length::Fixed(250.0))
            ),
            text(format!(
                "Selected: {}",
                state.pick_list_selected.as_deref().unwrap_or("None")
            ))
            .size(ts.caption.size),
        ]
        .spacing(gap.widget)
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
            probe(
                probes::COMBO_BOX,
                Length::Shrink,
                combo_box(
                    &state.combo_state,
                    "Search a language...",
                    state.combo_selected.as_ref(),
                    Message::ComboBoxSelected,
                )
                .input_style(styles::text_input(resolved))
                .menu_style(styles::menu(resolved))
                .width(Length::Fixed(250.0))
            ),
            text(format!(
                "Selected: {}",
                state.combo_selected.as_deref().unwrap_or("None")
            ))
            .size(ts.caption.size),
        ]
        .spacing(gap.widget)
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
            .spacing(gap.section)
            .width(Fill),
            rule::vertical(resolved.separator.line_width).style(styles::rule(resolved)),
            column![
                radios,
                rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                pickers,
                rule::horizontal(resolved.separator.line_width).style(styles::rule(resolved)),
                combos,
            ]
            .spacing(gap.section)
            .width(Fill),
        ]
        .spacing(gap.section),
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Range
// ---------------------------------------------------------------------------

fn view_range(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
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
                probe(
                    probes::SLIDER,
                    Fill,
                    slider(0.0..=100.0, state.slider_value, Message::SliderChanged)
                        .style(styles::slider(resolved))
                        .width(Fill)
                ),
                text(format!("{:.1}", state.slider_value))
                    .size(ts.section_heading.size)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(sp.m)
            .align_y(iced::Center),
            text("Drag to change value. This slider drives the first progress bar below.")
                .size(ts.caption.size),
        ]
        .spacing(gap.widget)
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
        .spacing(gap.widget)
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
                probe(
                    probes::VERTICAL_SLIDER,
                    Length::Shrink,
                    container(
                        vertical_slider(0.0..=100.0, state.vslider_value, Message::VSliderChanged)
                            .style(styles::slider(resolved))
                            .height(Length::Fixed(200.0))
                    )
                    .center_x(Length::Fixed(60.0))
                ),
                column![
                    text(format!("Value: {:.1}", state.vslider_value))
                        .size(ts.section_heading.size),
                    space().height(Length::Fixed(8.0)),
                    text("Vertical sliders are useful\nfor volume controls,\nequalizers, etc.")
                        .size(ts.caption.size),
                ]
                .spacing(sp.xs),
            ]
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
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
        .spacing(gap.widget)
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
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Display
// ---------------------------------------------------------------------------

fn view_display(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
    let resolved = &state.current_resolved;
    let ts = &resolved.text_scale;
    let card = &resolved.card;
    let tip = &resolved.tooltip;
    let sep = &resolved.separator;
    let card_radius = card.border.corner_radius;
    // A container is unpadded unless given a padding (container.rs:95), so a
    // side the card's theme leaves unstated stays at zero.
    let card_pad = native_theme_iced::padding_or(&card.border.padding, Padding::ZERO);
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
            &[
                ("border-radius", &card_radius_s),
                (
                    "padding",
                    "card.border.padding's stated sides, a container's own \
                     Padding::ZERO for the others",
                ),
            ],
            &[("text", "CardTheme carries no font — the label is inherited")],
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
            .padding(card_pad)
            .style(styles::container_card(resolved))
            .width(Fill),
            container(
                text(
                    "A second container dressed as a card. Containers take their \
                      background, border and padding from the resolved card theme."
                )
                .size(ts.caption.size),
            )
            .padding(card_pad)
            .style(styles::container_card(resolved))
            .width(Fill),
        ]
        .spacing(sp.m)
        .into(),
    );

    let rules = column![
        text("Divider Rules").size(ts.dialog_title.size),
        text(format!(
            "iced takes a rule's thickness as the constructor's argument, and the \
             platform states exactly one: separator.line_width ({line_width_s}). \
             Three rules at that width would be three copies of the same line, so \
             here is the one:"
        ))
        .size(ts.section_heading.size),
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        text(
            "Its colour is separator.line_color; its radius and its fill mode have \
             no native source and are iced's own."
        )
        .size(ts.caption.size),
    ]
    .spacing(gap.widget);

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
                        .style(styles::button_primary(resolved))
                        .padding(native_theme_iced::button_padding(resolved)),
                    text("Tooltip on top!"),
                    tooltip::Position::Top,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button("Hover: Bottom")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button(resolved))
                        .padding(native_theme_iced::button_padding(resolved)),
                    text("Tooltip on bottom!"),
                    tooltip::Position::Bottom,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button("Hover: Left")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_success(resolved))
                        .padding(native_theme_iced::button_padding(resolved)),
                    text("Tooltip on left!"),
                    tooltip::Position::Left,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
                tooltip(
                    button("Hover: Right")
                        .on_press(Message::ButtonPressed)
                        .style(styles::button_danger(resolved))
                        .padding(native_theme_iced::button_padding(resolved)),
                    text("Tooltip on right!"),
                    tooltip::Position::Right,
                )
                .gap(sp.xs)
                .style(styles::tooltip(resolved)),
            ]
            .spacing(sp.m),
        ]
        .spacing(gap.widget)
        .into(),
    );

    let theme_info_text = format!(
        "Active theme: {}  |  Mode: {}",
        state.current_theme,
        if state.is_dark { "Dark" } else { "Light" },
    );

    // The sizes the showcase draws at, which are the theme's own: no text here
    // is scaled by the OS text-scaling factor. That factor is the OS theme's
    // alone -- a preset has no OS reading -- so only there is what
    // `font_size` would scale to shown beside it, and only where it differs.
    let font_info = {
        let r = &state.current_resolved;
        let ff = native_theme_iced::font_family(r);
        let mf = native_theme_iced::mono_font_family(r);
        let drawn = format!(
            "Font: {ff} @ {:.1}px  |  Mono: {mf} @ {:.1}px",
            r.defaults.font.size, r.defaults.mono_font.size
        );
        let scaled = native_theme_iced::font_size(r, &state.accessibility);
        let os_theme = matches!(state.current_choice, ThemeChoice::OsTheme(_));
        if os_theme && scaled != r.defaults.font.size {
            format!(
                "{drawn}  |  OS text scaling: font_size() gives {scaled:.1}px, \
                 which the showcase does not apply"
            )
        } else {
            drawn
        }
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
    .padding(Padding::from(gap.container))
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
        .spacing(gap.widget)
        .align_y(iced::Center),
    ]
    .spacing(gap.widget);

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
    .spacing(gap.section)
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
    let gap = Gaps::from_layout(&state.layout);
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
            .spacing(gap.widget)
            // `Grid::height` sets the height of the whole grid, not of a cell
            // (`grid.rs:78-84`, `Sizing::EvenlyDistribute`): a fixed number
            // here divides that number between the rows, and the swatch plus
            // its caption did not fit in the share. `Shrink` is the one
            // `Sizing` that lets each row be as tall as its content
            // (`grid.rs:207-211`).
            .height(Length::Shrink),
        ]
        .spacing(gap.widget)
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
                .padding(native_theme_iced::button_padding(resolved)),
            button(text("split —").size(ts.caption.size))
                .on_press(Message::PaneSplit(pane_grid::Axis::Horizontal, pane))
                .style(styles::button(resolved))
                .padding(native_theme_iced::button_padding(resolved)),
            button(text("close").size(ts.caption.size))
                .on_press(Message::PaneClosed(pane))
                .style(styles::button_danger(resolved))
                .padding(native_theme_iced::button_padding(resolved)),
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
        .spacing(gap.widget)
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
                "iced's background.strong, read live",
                state
                    .current_theme
                    .extended_palette()
                    .background
                    .strong
                    .color,
            )],
            &[
                ("separator_x / separator_y", &line_width_s),
                ("cell padding", "the showcase's own scale"),
            ],
            &[
                (
                    "Style",
                    "iced_widget 0.14.2 gives Table a Catalog and a Style but no \
                     `.style(..)` or `.class(..)` setter (table.rs:149-196), so its \
                     separator colors stay table::default(theme), which reads \
                     palette.background.strong (table.rs:717-724)",
                ),
                (
                    "separator.line_color",
                    "has no receiver here; the hex above is the one on screen, not \
                     the platform's separator colour",
                ),
            ],
        ),
        column![
            text("Table (columns and rows)").size(ts.dialog_title.size),
            text("The four typographic roles this theme resolves:").size(ts.section_heading.size),
            scale_table,
        ]
        .spacing(gap.widget)
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
    .spacing(gap.section)
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
    let gap = Gaps::from_layout(&state.layout);
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
        .spacing(gap.widget)
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
                "no native source — iced's own DEFAULT_CELL_SIZE (qr_code.rs)",
            )],
        ),
        column![
            text("QRCode").size(ts.dialog_title.size),
            text("Its two-colour Style is the platform's foreground on its background:")
                .size(ts.section_heading.size),
            qr_demo,
        ]
        .spacing(gap.widget)
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
                // Markdown states no colour of its own, so iced paints it with
                // the slot `Base::base` hands every inheriting widget
                // (`iced_core` theme.rs:334); the connector sets that slot from
                // `defaults.text_color`, so reading it live shows what is on
                // screen rather than the palette input behind it.
                (
                    "text",
                    "extended.background.base.text, from defaults.text_color",
                    state.current_theme.extended_palette().background.base.text,
                ),
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
                    "block spacing",
                    "Settings::spacing is iced's own, derived from the native base \
                     size. It is not layout.widget_gap: iced reuses the one number \
                     as the gap between blocks, as 0.6 and 0.75 of itself inside \
                     lists, and as a list's horizontal indent (markdown.rs:1230, \
                     :1364-1374)",
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
            .padding(Padding::from(gap.container))
            .style(styles::container_card(resolved))
            .width(Fill),
            text(link_line).size(ts.caption.size),
        ]
        .spacing(gap.widget)
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
    .spacing(gap.section)
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
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
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
        let card = Card::new(
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
                    .padding(native_theme_iced::button_padding(resolved)),
            ]
            .spacing(sp.xs),
        ))
        .width(Length::Fixed(420.0))
        .style(styles::aw::card(resolved));
        // `Card::padding` sets the head, the body and the foot alike, and
        // iced_aw keeps the default it replaces private (widget/card.rs:21),
        // so a theme that leaves a side unstated leaves the card iced_aw's.
        match native_theme_iced::stated_padding(&card_t.border.padding) {
            Some(padding) => card.padding(padding),
            None => card,
        }
        .into()
    } else {
        button(text("Show the card again").size(ts.caption.size))
            .on_press(Message::AwCardToggled)
            .style(styles::button_primary(resolved))
            .padding(native_theme_iced::button_padding(resolved))
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
                ("radius", "card.border.corner_radius"),
                (
                    "section padding",
                    "card.border.padding, on the head, the body and the foot \
                     alike, where the theme states every side; otherwise \
                     iced_aw's own, which it keeps private (widget/card.rs:21)",
                ),
                ("Dismiss padding", "button_padding"),
            ],
            &[(
                "close icon",
                "not drawn: iced_aw 0.14.1 styles Card::on_close's button \
                     with its default class, whose icon is white whatever \
                     styles::aw::card sets (widget/card.rs:193-206), so the \
                     themed Dismiss button closes the card instead",
            )],
        ),
        column![text("Card").size(ts.dialog_title.size), card_section,]
            .spacing(gap.widget)
            .into(),
    );

    // ---- MenuBar and Menu ----

    // The items are buttons padded like the platform's menu items: the sides
    // menu.border.padding states, a button's own elsewhere. A theme that
    // states no row height (KDE's items size to their font) leaves the item
    // its own height.
    let item_pad = native_theme_iced::padding_or(&menu_t.border.padding, button::DEFAULT_PADDING);
    let menu_entry = move |label: &'static str| -> Element<'_, Message> {
        let entry = button(text(label).size(menu_t.font.size))
            .on_press(Message::AwActionChosen(format!("Menu: {label}")))
            .style(styles::button(resolved))
            .width(Fill)
            .padding(item_pad);
        match menu_t.row_height {
            Some(h) => entry.height(Length::Fixed(h)),
            None => entry,
        }
        .into()
    };
    let menu_root = move |label: &'static str| {
        button(text(label).size(menu_t.font.size))
            .on_press(Message::AwActionChosen(format!("Menu: {label}")))
            .style(styles::button(resolved))
            .padding(item_pad)
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
                (
                    "item height",
                    "menu.row_height where the theme states one, on the item button",
                ),
                (
                    "item padding",
                    "menu.border.padding's stated sides, iced's \
                     button::DEFAULT_PADDING for the others, on the item button",
                ),
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
        .spacing(gap.widget)
        .into(),
    );

    // ---- ContextMenu ----

    let context_underlay = container(
        column![
            text("Right-click inside this panel").size(ts.section_heading.size),
            text(
                "ContextMenu gets no styles::aw function: its own Style is a one-field \
                 backdrop scrim and its default class already emits alpha 0 \
                 (style/context_menu.rs:48-59). The popup below is our own element; \
                 its entries are padded like the menu bar's items, by \
                 menu.border.padding."
            )
            .size(ts.caption.size),
        ]
        .spacing(sp.xs),
    )
    .padding(Padding::from(gap.container))
    .style(styles::container_card(resolved))
    .width(Fill);

    let context_demo = ContextMenu::new(context_underlay, move || {
        let entry = |label: &'static str| -> Element<'_, Message> {
            button(text(label).size(ts.caption.size))
                .on_press(Message::AwActionChosen(format!("Context menu: {label}")))
                .style(styles::button(resolved))
                .width(Fill)
                .padding(item_pad)
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
    // iced_aw keeps a tab's default padding private (widget/tab_bar.rs:41) and
    // takes a padding whole, so only a theme that states every side replaces it.
    let tab_padding = native_theme_iced::stated_padding(&tab_t.border.padding);
    let tab_bar = match tab_padding {
        Some(padding) => tab_bar.padding(padding),
        None => tab_bar,
    };

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
    let tabs = match tab_padding {
        Some(padding) => tabs.tab_label_padding(padding),
        None => tabs,
    };

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
                (
                    "tab padding",
                    "tab.border.padding where the theme states every side; \
                     otherwise iced_aw's own, which it keeps private \
                     (widget/tab_bar.rs:41)",
                ),
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
        .spacing(gap.widget)
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
                .padding(Padding::from(gap.container))
                .style(styles::container_card(resolved))
                .width(Fill),
            ]
            .spacing(gap.widget),
        ]
        .spacing(gap.widget)
        .into(),
    );

    // ---- Spinner ----

    let spinner_demo = hoverable(
        widget_tooltip(
            "Spinner",
            &[(
                "orbiting dot",
                "spinner.fill_color",
                to_color(spin_t.fill_color),
            )],
            &[("orbit diameter", "spinner.diameter")],
            &[
                (
                    "Style",
                    "iced_aw 0.14.1 gives Spinner none at all: it paints in the \
                     inherited text colour, which the wrapping container states",
                ),
                (
                    "spinner.stroke_width",
                    "no receiver: iced_aw paints no ring to stroke — one dot \
                     circling the centre (spinner.rs:151), whose radius is \
                     circle_radius, not a stroke width",
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
        .spacing(gap.widget)
        .into(),
    );

    // ---- SelectionList ----

    // `iced_aw` lays a row out as `text_size + padding.y()`
    // (`selection_list/list.rs:118`, `:209`), so `list.row_height` is reachable
    // after all -- not through a setter of its own, but as the vertical padding
    // that makes the sum come out. `padding.y()` is top plus bottom, so each
    // side takes half of what the label leaves. A theme that states no row
    // height, or one that does not clear its own label size, leaves the
    // showcase's own padding standing, rather than a negative inset.
    let row_height_s = match list_t.row_height {
        Some(h) => format!("{h:.0}px"),
        None => "not stated: the showcase's own padding".to_string(),
    };
    let row_inset = list_t.row_height.map_or(0.0, |h| h - list_t.item_font.size);
    let list_padding = if row_inset > 0.0 {
        // Only the vertical half is the platform's: the label is drawn at the
        // row's own `bounds.x` (`selection_list/list.rs:274`), so horizontal
        // padding never reaches it, and the list's intrinsic width does read
        // `padding.x()` (`selection_list.rs:237`, `:244`).
        Padding::ZERO
            .top(row_inset / 2.0)
            .bottom(row_inset / 2.0)
            .left(sp.xs)
            .right(sp.xs)
    } else {
        Padding::from(sp.xs)
    };

    // `SelectionList::new_with` is the only constructor that gives the rows a
    // class as well as the list, and it demands a `Clone` style function --
    // which every `styles::aw::*` closure is.
    let selection_list = SelectionList::new_with(
        &state.aw_list_options,
        Message::AwListSelected,
        list_t.item_font.size,
        list_padding,
        styles::aw::selection_list(resolved),
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
            &[
                ("row label size", "list.item_font.size"),
                ("row height", &row_height_s),
            ],
            &[(
                "list.row_height",
                "reachable only indirectly: iced_aw has no item_height setter, so \
                 a row is text_size + padding.y() (selection_list/list.rs:118, \
                 :209) and the showcase gives new_with the padding that makes the \
                 sum the platform's height",
            )],
        ),
        column![
            text("SelectionList").size(ts.dialog_title.size),
            probe(probes::SELECTION_LIST, Length::Shrink, selection_list),
        ]
        .spacing(gap.widget)
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
        column![text("ContextMenu").size(ts.dialog_title.size), context_demo,].spacing(gap.widget),
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        tab_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        side_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        spinner_demo,
        rule::horizontal(sep.line_width).style(styles::rule(resolved)),
        list_demo,
    ]
    .spacing(gap.section)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Tab: Icons
// ---------------------------------------------------------------------------

fn view_icons(state: &State) -> Element<'_, Message> {
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
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
        text(state.system_icon_theme_label()).size(ts.caption.size),
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
        grid_rows.push(row(row_icons).spacing(gap.widget).into());
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
    // Header, icon-set summary and animated section are major sections of the
    // tab, the same role the other tab roots give `gap.section`.
    .spacing(gap.section);
    for r in grid_rows {
        content = content.push(r);
    }

    content.width(Fill).into()
}

fn view_animated_icons<'a>(state: &'a State, fg_color: Color) -> Element<'a, Message> {
    let sp = &SP;
    let gap = Gaps::from_layout(&state.layout);
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

    let mut content = column![section_title, divider].spacing(gap.widget);

    if state.reduced_motion {
        content = content
            .push(text("prefers-reduced-motion: showing static frames").size(ts.caption.size));
    }

    if spinners.is_empty() {
        content = content.push(
            text("No animated icons available for this configuration.").size(ts.caption.size),
        );
    } else {
        content = content.push(row(spinners).spacing(gap.widget));
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
    let gap = Gaps::from_layout(&state.layout);
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
        .spacing(gap.widget)
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
        .spacing(gap.widget);
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
    .spacing(gap.section)
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
        .window_size(WINDOW_SIZE)
        .centered()
        .run()
}

// ---------------------------------------------------------------------------
// Self-tests (spec §6.2)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use iced::mouse;
    use iced::{Event, Point, Rectangle, Settings, Size};
    use iced_test::Simulator;
    use iced_test::selector::{self, Candidate};

    /// The viewport the interaction test lays the interface out in.
    ///
    /// Not the window `main` opens: an element scrolled out of the content
    /// `scrollable`'s viewport has no visible bounds and cannot be clicked,
    /// and the tab strip scrolls sideways once the tabs are wider than the
    /// panel. `every_tab_renders` uses the real [`WINDOW_SIZE`] instead, which
    /// is what makes the scroll containers part of what it renders.
    const TALL_WINDOW: Size = Size::new(1400.0, 6000.0);

    /// The real interface, in [`TALL_WINDOW`].
    ///
    /// `Settings::default()` leaves the font as `Font::DEFAULT`, which the
    /// simulator resolves to the Fira Sans it bundles
    /// (`iced_test` `simulator.rs:67-70`), so text measures the same on every
    /// host.
    fn interface(state: &State) -> Simulator<'_, Message> {
        Simulator::with_size(Settings::default(), TALL_WINDOW, view(state))
    }

    /// Builds the real interface, runs `interaction` on it, then feeds every
    /// message the interaction produced through the real `update`.
    ///
    /// Every message reaches `update`; the ones handed back leave the Widget
    /// Info panel's out. Pointing the cursor at a widget is what that panel
    /// listens for, so any click no widget captures — a disabled button's,
    /// say — also reports the hover, which says nothing about the control
    /// under test.
    fn drive(
        state: &mut State,
        interaction: impl FnOnce(&mut Simulator<'_, Message>),
    ) -> Vec<Message> {
        let messages: Vec<Message> = {
            let mut ui = interface(state);
            interaction(&mut ui);
            ui.into_messages().collect()
        };
        for message in messages.iter().cloned() {
            let _ = update(state, message);
        }
        messages
            .into_iter()
            .filter(|message| {
                !matches!(
                    message,
                    Message::WidgetHovered(_) | Message::WidgetUnhovered
                )
            })
            .collect()
    }

    /// Clicks the widget whose own text is `label`.
    fn click_text(ui: &mut Simulator<'_, Message>, label: &str) {
        if let Err(error) = ui.click(label) {
            panic!("{label}: {error}");
        }
    }

    /// Clicks the widget tagged with the given [`probe`] id.
    fn click_probe(ui: &mut Simulator<'_, Message>, id: &'static str) {
        if let Err(error) = ui.click(selector::id(id)) {
            panic!("{id}: {error}");
        }
    }

    /// Opens the `pick_list` tagged `id` and chooses its `nth` option.
    ///
    /// A `pick_list` pads its menu with `button::DEFAULT_PADDING`
    /// (`pick_list.rs:204`).
    fn pick_list_option(ui: &mut Simulator<'_, Message>, id: &'static str, nth: usize) {
        pick_option(ui, id, nth, iced::widget::button::DEFAULT_PADDING);
    }

    /// Opens the `combo_box` tagged `id` and chooses its `nth` option.
    ///
    /// A `combo_box` pads its menu with `text_input::DEFAULT_PADDING`
    /// (`combo_box.rs:190`), which is as tall as a `pick_list`'s today but is
    /// not the same constant and need not stay equal.
    fn combo_box_option(ui: &mut Simulator<'_, Message>, id: &'static str, nth: usize) {
        pick_option(ui, id, nth, iced::widget::text_input::DEFAULT_PADDING);
    }

    /// Opens the picker tagged `id` and chooses its `nth` option, counting
    /// from the top of the menu.
    ///
    /// The menu is an overlay whose list implements no `Widget::operate`
    /// (`overlay/menu.rs`), so no selector reaches its rows. What iced does
    /// state publicly is where they are: the menu is laid out directly under
    /// the widget that opened it (`overlay/menu.rs:241-262`), one row per
    /// option, each `line_height + padding.y()` tall
    /// (`overlay/menu.rs:375-397`), where the padding is the one the picker
    /// gives it. Both terms are read from iced's own defaults, which the
    /// showcase overrides for none of its pickers, rather than copied as
    /// numbers.
    fn pick_option(
        ui: &mut Simulator<'_, Message>,
        id: &'static str,
        nth: usize,
        menu_padding: Padding,
    ) {
        let text_size = Settings::default().default_text_size;
        let row = f32::from(iced_core::text::LineHeight::default().to_absolute(text_size))
            + menu_padding.y();
        let field = probe_bounds(ui, id);
        let at = Point::new(
            field.center().x,
            field.y + field.height + row * (nth as f32 + 0.5),
        );

        click_probe(ui, id);
        ui.point_at(at);
        let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
        let _ = ui.simulate(iced_test::simulator::click());
    }

    /// The bounds of the widget tagged with the given [`probe`] id.
    fn probe_bounds(ui: &mut Simulator<'_, Message>, id: &'static str) -> Rectangle {
        match ui.find(selector::id(id)) {
            Ok(target) => match target.visible_bounds() {
                Some(bounds) => bounds,
                None => panic!("{id}: found, but not visible"),
            },
            Err(error) => panic!("{id}: {error}"),
        }
    }

    /// Clicks `label`, and asserts the one message it produced.
    fn press(state: &mut State, label: &'static str, expected: &str) {
        let messages = drive(state, |ui| click_text(ui, label));
        let printed: Vec<String> = messages.iter().map(|m| format!("{m:?}")).collect();
        assert_eq!(
            printed,
            vec![expected.to_string()],
            "{label}: the click produced the wrong messages"
        );
    }

    // -----------------------------------------------------------------------
    // every_tab_renders
    // -----------------------------------------------------------------------

    /// Every tab lays out and draws, at the size the application opens.
    ///
    /// What taking the snapshot catches is a panic in `view`, in layout or in
    /// drawing. It has no failure path of its own: `Simulator::snapshot` in
    /// `iced_test` 0.14.0 always returns `Ok` (`simulator.rs:199-241`), so the
    /// `Err` arm below is unreachable today and is written out rather than
    /// discarded so that the day it stops being so does not pass unnoticed.
    ///
    /// The snapshot is deliberately not compared with a baseline — that is
    /// Layer 4 and out of scope (spec §6.2). What is compared is that no text
    /// the tab lays out was squeezed to nothing: a widget with no room left is
    /// a widget nobody sees, which a drawing pass that merely returns `Ok`
    /// would not report.
    #[test]
    fn every_tab_renders() {
        for tab in Tab::ALL {
            let state = State {
                active_tab: *tab,
                ..State::default()
            };
            let theme = theme(&state);

            let mut ui: Simulator<'_, Message> =
                Simulator::with_size(Settings::default(), WINDOW_SIZE, view(&state));

            match ui.snapshot(&theme) {
                Ok(_) => {}
                Err(error) => panic!("{}: the interface did not draw: {error}", tab.label()),
            }

            let mut starved: Vec<String> = Vec::new();
            {
                let sink = &mut starved;
                let _ = ui.find(move |candidate: Candidate<'_>| -> Option<()> {
                    if let Candidate::Text {
                        content, bounds, ..
                    } = candidate
                        && (bounds.width <= 0.0 || bounds.height <= 0.0)
                    {
                        sink.push(format!("{content:?} at {bounds:?}"));
                    }
                    None
                });
            }
            assert!(
                starved.is_empty(),
                "{}: laid out with no room to draw: {starved:?}",
                tab.label()
            );
        }
    }

    // -----------------------------------------------------------------------
    // interactive_controls_respond
    // -----------------------------------------------------------------------

    /// Every control the showcase advertises answers a click, and the answer
    /// reaches the state through the real `update`.
    #[test]
    fn interactive_controls_respond() {
        let mut state = State::default();

        // ---- the tab strip ----
        assert_eq!(
            state.active_tab,
            Tab::Buttons,
            "the showcase opens on Buttons"
        );

        // ---- Buttons tab: one message per class, and none from a disabled one ----
        for (label, count) in [
            ("Primary", 1),
            ("Secondary", 2),
            ("Success", 3),
            ("Warning", 4),
            ("Danger", 5),
            ("Text Style", 6),
            ("Click me!", 7),
        ] {
            press(&mut state, label, "ButtonPressed");
            assert_eq!(
                state.button_press_count, count,
                "{label}: the press did not reach the counter"
            );
        }
        // Spec §6.2 words this as "its click fails". That is not what
        // `iced_test` does: `click` errs only when the selector finds nothing
        // (`simulator.rs:121-128`) or the target has no visible bounds
        // (`:151-154`), and a disabled button is both found and visible. The
        // click resolves; what says the button is disabled is that it
        // published nothing and the counter did not move, which is the
        // stronger claim.
        for label in ["Disabled Primary", "Disabled Secondary", "Disabled Danger"] {
            let messages = drive(&mut state, |ui| click_text(ui, label));
            assert!(
                messages.is_empty(),
                "{label}: a button with no on_press must stay silent, got {messages:?}"
            );
            assert_eq!(state.button_press_count, 7, "{label}: the counter moved");
        }

        // ---- the tab strip carries the view to the next tab ----
        let messages = drive(&mut state, |ui| click_text(ui, "Text Inputs"));
        assert!(
            matches!(messages.as_slice(), [Message::TabSelected(Tab::TextInputs)]),
            "tab strip: {messages:?}"
        );
        assert_eq!(
            state.active_tab,
            Tab::TextInputs,
            "the tab strip did not switch tabs"
        );

        // ---- TextInput: click to focus, then type ----
        let messages = drive(&mut state, |ui| {
            click_probe(ui, TEXT_INPUT_ID);
            let _ = ui.typewrite("q");
        });
        assert!(
            matches!(messages.as_slice(), [Message::TextInputChanged(value)] if value == "q"),
            "text_input: {messages:?}"
        );
        assert_eq!(
            state.text_input_value, "q",
            "text_input: the value did not reach the state"
        );

        // ---- TextEditor: the same, through its own Content ----
        let before = state.text_editor_content.text();
        let messages = drive(&mut state, |ui| {
            click_probe(ui, probes::TEXT_EDITOR);
            let _ = ui.typewrite("q");
        });
        assert!(
            messages
                .iter()
                .any(|m| matches!(m, Message::EditorAction(_))),
            "text_editor: {messages:?}"
        );
        assert_ne!(
            state.text_editor_content.text(),
            before,
            "text_editor: the action did not reach the document"
        );

        // ---- Selection tab ----
        let _ = drive(&mut state, |ui| click_text(ui, "Selection"));
        assert_eq!(state.active_tab, Tab::Selection);

        assert!(state.checkbox_a, "the first checkbox starts checked");
        let messages = drive(&mut state, |ui| click_text(ui, "Enable notifications"));
        assert!(
            matches!(messages.as_slice(), [Message::CheckboxAToggled(false)]),
            "checkbox: {messages:?}"
        );
        assert!(
            !state.checkbox_a,
            "checkbox: the toggle did not reach the state"
        );

        assert_eq!(
            state.selected_fruit,
            Some(Fruit::Apple),
            "the radios start on Apple"
        );
        let messages = drive(&mut state, |ui| click_probe(ui, probes::RADIO_BANANA));
        assert!(
            matches!(messages.as_slice(), [Message::FruitSelected(Fruit::Banana)]),
            "radio: {messages:?}"
        );
        assert_eq!(
            state.selected_fruit,
            Some(Fruit::Banana),
            "radio: the selection did not reach the state"
        );

        assert!(!state.toggler_enabled, "the toggler starts off");
        let messages = drive(&mut state, |ui| click_probe(ui, probes::TOGGLER));
        assert!(
            matches!(messages.as_slice(), [Message::TogglerToggled(true)]),
            "toggler: {messages:?}"
        );
        assert!(
            state.toggler_enabled,
            "toggler: the toggle did not reach the state"
        );

        // The menu is an overlay: it exists only while the same interface is
        // open, so the click that opens it and the click that picks an entry
        // share one simulator.
        assert_eq!(state.pick_list_selected.as_deref(), Some("Rust"));
        let messages = drive(&mut state, |ui| pick_list_option(ui, probes::PICK_LIST, 1));
        let picked = match messages.as_slice() {
            [Message::PickListSelected(value)] => value.clone(),
            other => panic!("pick_list: {other:?}"),
        };
        assert_ne!(picked, "Rust", "pick_list: the second row is the first one");
        assert_eq!(
            state.pick_list_selected.as_deref(),
            Some(picked.as_str()),
            "pick_list: the choice did not reach the state"
        );

        assert_eq!(state.combo_selected, None);
        let messages = drive(&mut state, |ui| combo_box_option(ui, probes::COMBO_BOX, 1));
        let picked = match messages.as_slice() {
            [Message::ComboBoxSelected(value)] => value.clone(),
            other => panic!("combo_box: {other:?}"),
        };
        assert_eq!(
            state.combo_selected.as_deref(),
            Some(picked.as_str()),
            "combo_box: the choice did not reach the state"
        );

        // ---- Range tab ----
        let _ = drive(&mut state, |ui| click_text(ui, "Range"));
        assert_eq!(state.active_tab, Tab::Range);

        let before = state.slider_value;
        let messages = drive(&mut state, |ui| click_probe(ui, probes::SLIDER));
        let picked = match messages.as_slice() {
            [Message::SliderChanged(value)] => *value,
            other => panic!("slider: {other:?}"),
        };
        assert_ne!(
            picked, before,
            "slider: the click landed on the current value"
        );
        assert_eq!(
            state.slider_value, picked,
            "slider: the value did not reach the state"
        );

        // The middle of the vertical rail is the value the slider already
        // holds, and a slider publishes nothing when the value does not move
        // (`vertical_slider.rs`, `change`), so the click lands a quarter of
        // the way down the rail instead of at its centre.
        let before = state.vslider_value;
        let messages = drive(&mut state, |ui| {
            let rail = probe_bounds(ui, probes::VERTICAL_SLIDER);
            let quarter = rail.y + rail.height / 4.0;
            ui.point_at(Point::new(rail.center().x, quarter));
            let _ = ui.simulate(iced_test::simulator::click());
        });
        let picked = match messages.as_slice() {
            [Message::VSliderChanged(value)] => *value,
            other => panic!("vertical_slider: {other:?}"),
        };
        assert_ne!(
            picked, before,
            "vertical_slider: the click landed on the current value"
        );
        assert_eq!(
            state.vslider_value, picked,
            "vertical_slider: the value did not reach the state"
        );

        // ---- Layout tab: the pane grid splits and closes ----
        let _ = drive(&mut state, |ui| click_text(ui, "Layout"));
        assert_eq!(state.active_tab, Tab::Layout);

        // A click inside a pane focuses it as well as pressing the button
        // under the cursor, so both messages are expected here.
        let panes_before = state.panes.iter().count();
        let messages = drive(&mut state, |ui| click_text(ui, "split |"));
        assert!(
            messages
                .iter()
                .any(|message| matches!(message, Message::PaneSplit(pane_grid::Axis::Vertical, _))),
            "pane_grid split: {messages:?}"
        );
        assert_eq!(
            state.panes.iter().count(),
            panes_before + 1,
            "pane_grid: the split did not add a pane"
        );

        let panes_before = state.panes.iter().count();
        let messages = drive(&mut state, |ui| click_text(ui, "close"));
        assert!(
            messages
                .iter()
                .any(|message| matches!(message, Message::PaneClosed(_))),
            "pane_grid close: {messages:?}"
        );
        assert_eq!(
            state.panes.iter().count(),
            panes_before - 1,
            "pane_grid: the close did not remove a pane"
        );

        // ---- Graphics tab: the Markdown link ----
        let _ = drive(&mut state, |ui| click_text(ui, "Graphics"));
        assert_eq!(state.active_tab, Tab::Graphics);

        let (nth_item, url) = markdown_link();
        assert_eq!(state.markdown_link, None, "nothing has been clicked yet");
        let messages = drive(&mut state, |ui| {
            let bounds = markdown_item_bounds(ui, nth_item);
            ui.point_at(bounds.center());
            let _ = ui.simulate(iced_test::simulator::click());
        });
        assert!(
            matches!(messages.as_slice(), [Message::MarkdownLinkClicked(uri)] if uri == url),
            "markdown link: {messages:?}"
        );
        assert_eq!(
            state.markdown_link.as_deref(),
            Some(url),
            "markdown link: the uri did not reach the state"
        );

        #[cfg(feature = "iced_aw")]
        aw_controls_respond(&mut state);

        // ---- The sidebar selectors, last: they replace the theme ----
        theme_selectors_respond(&mut state);
    }

    /// The `iced_aw` widgets of the Extra tab.
    #[cfg(feature = "iced_aw")]
    fn aw_controls_respond(state: &mut State) {
        let _ = drive(state, |ui| click_text(ui, "Extra widgets (iced_aw)"));
        assert_eq!(state.active_tab, Tab::Extra);

        // TabBar (stand-alone).
        assert_eq!(state.aw_tab_bar_active, 0);
        let messages = drive(state, |ui| click_text(ui, "Details"));
        assert!(
            matches!(messages.as_slice(), [Message::AwTabBarSelected(1)]),
            "TabBar: {messages:?}"
        );
        assert_eq!(
            state.aw_tab_bar_active, 1,
            "TabBar: the selection did not reach the state"
        );

        // Sidebar.
        assert_eq!(state.aw_sidebar_active, 0);
        let messages = drive(state, |ui| click_text(ui, "Appearance"));
        assert!(
            matches!(messages.as_slice(), [Message::AwSidebarSelected(1)]),
            "Sidebar: {messages:?}"
        );
        assert_eq!(
            state.aw_sidebar_active, 1,
            "Sidebar: the selection did not reach the state"
        );

        // MenuBar: a root item is one of our own buttons.
        let messages = drive(state, |ui| click_text(ui, "File"));
        assert!(
            matches!(messages.as_slice(), [Message::AwActionChosen(what)] if what == "Menu: File"),
            "MenuBar: {messages:?}"
        );
        assert_eq!(
            state.aw_last_action, "Menu: File",
            "MenuBar: the choice did not reach the state"
        );

        // ContextMenu: it opens on the right button only, and its popup is
        // placed at the cursor (`overlay/context_menu.rs:84-105`). The entry
        // is then clicked without looking it up: while the menu is open,
        // `ContextMenu::operate` hands the popup the *underlay's* layout
        // (`widget/context_menu.rs:186-192`), and every button in it panics on
        // a layout node with no children (`button.rs:265-268`), so any
        // selector run over that tab would take the whole test down. Clicking
        // needs no selector -- `simulate` never calls `operate` -- and the
        // point is the popup's own padding plus its first entry's.
        let messages = drive(state, |ui| {
            let underlay = match ui.find("Right-click inside this panel") {
                Ok(target) => match target.visible_bounds() {
                    Some(bounds) => bounds,
                    None => panic!("ContextMenu: the underlay is not visible"),
                },
                Err(error) => panic!("ContextMenu: {error}"),
            };
            let at = underlay.center();
            ui.point_at(at);
            let _ = ui.simulate([
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)),
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)),
            ]);
            let entry = Point::new(at.x + SP.xs + SP.s, at.y + SP.xs + SP.xxs);
            ui.point_at(entry);
            let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: entry })]);
            let _ = ui.simulate(iced_test::simulator::click());
        });
        assert!(
            matches!(messages.as_slice(), [Message::AwActionChosen(what)] if what == "Context menu: Copy"),
            "ContextMenu: {messages:?}"
        );
        assert_eq!(
            state.aw_last_action, "Context menu: Copy",
            "ContextMenu: the choice did not reach the state"
        );

        // SelectionList: `iced_aw` reports its rows in the list's own
        // coordinates and with no visible bounds
        // (`selection_list/list.rs:281-310`), so the click point is the
        // list's own origin plus the row's offset inside it.
        assert_eq!(
            state.aw_list_selected,
            Some(0),
            "the list starts on its first row"
        );
        let messages = drive(state, |ui| {
            let list = probe_bounds(ui, probes::SELECTION_LIST);
            let row = match ui.find("Breeze") {
                Ok(target) => target.bounds(),
                Err(error) => panic!("SelectionList: {error}"),
            };
            ui.point_at(Point::new(list.x + row.center().x, list.y + row.center().y));
            let _ = ui.simulate(iced_test::simulator::click());
        });
        assert!(
            matches!(messages.as_slice(), [Message::AwListSelected(1, value)] if value == "Breeze"),
            "SelectionList: {messages:?}"
        );
        assert_eq!(
            state.aw_list_selected,
            Some(1),
            "SelectionList: the row did not reach the state"
        );

        // Card: its foot button closes it, and the button that replaces it
        // brings it back.
        assert!(state.aw_card_open, "the card starts open");
        let messages = drive(state, |ui| click_text(ui, "Dismiss"));
        assert!(
            matches!(messages.as_slice(), [Message::AwCardToggled]),
            "Card close: {messages:?}"
        );
        assert!(
            !state.aw_card_open,
            "Card: the close did not reach the state"
        );

        let messages = drive(state, |ui| click_text(ui, "Show the card again"));
        assert!(
            matches!(messages.as_slice(), [Message::AwCardToggled]),
            "Card reopen: {messages:?}"
        );
        assert!(
            state.aw_card_open,
            "Card: the reopen did not reach the state"
        );
    }

    /// The two sidebar selectors, asserted against what the state held before
    /// the click rather than against this host's desktop.
    fn theme_selectors_respond(state: &mut State) {
        // Colour mode. `AppColorMode::System` resolves to whichever mode this
        // desktop is in, so the test asks for the opposite of what is on
        // screen and checks that both the flag and the installed palette
        // followed.
        let was_dark = state.is_dark;
        let before = theme(state).extended_palette().background.base.color;
        let wanted = if was_dark {
            AppColorMode::Light
        } else {
            AppColorMode::Dark
        };
        let nth = match AppColorMode::ALL.iter().position(|mode| *mode == wanted) {
            Some(index) => index,
            None => panic!("{wanted} is not offered by the colour mode picker"),
        };
        let messages = drive(state, |ui| pick_list_option(ui, probes::COLOR_MODE, nth));
        assert!(
            matches!(messages.as_slice(), [Message::ColorModeSelected(mode)] if *mode == wanted),
            "colour mode: {messages:?}"
        );
        assert_eq!(
            state.is_dark, !was_dark,
            "colour mode: the mode did not change"
        );
        assert_ne!(
            theme(state).extended_palette().background.base.color,
            before,
            "colour mode: the installed theme did not follow the mode"
        );

        // Theme preset. The entry is the first preset this platform offers
        // that would put a different background on screen, so the assertion
        // is about the installed theme moving and not about which desktop
        // this runs on.
        let mode = if state.is_dark {
            native_theme_iced::ColorMode::Dark
        } else {
            native_theme_iced::ColorMode::Light
        };
        let before = theme(state).extended_palette().background.base.color;
        let chosen = native_theme::theme::Theme::list_presets_for_platform()
            .iter()
            .enumerate()
            .find_map(|(index, info)| {
                let native = native_theme::theme::Theme::preset(info.key).ok()?;
                let resolved = native.resolve(mode).ok()?;
                let built = native_theme_iced::to_theme(&resolved.variant, &native.name);
                (built.extended_palette().background.base.color != before)
                    .then(|| (index + 1, info.key.to_string(), built))
            });
        let (nth, preset, expected) = match chosen {
            Some(chosen) => chosen,
            None => panic!("no preset of this platform resolves to another background"),
        };

        let messages = drive(state, |ui| pick_list_option(ui, probes::THEME, nth));
        assert!(
            matches!(messages.as_slice(), [Message::ThemeSelected(ThemeChoice::Preset(name))] if *name == preset),
            "theme preset: {messages:?}"
        );
        assert_eq!(
            state.current_choice,
            ThemeChoice::Preset(preset.clone()),
            "theme preset: the choice did not reach the state"
        );
        assert!(
            state.error_message.is_none(),
            "theme preset: {:?}",
            state.error_message
        );
        assert_eq!(
            theme(state).to_string(),
            expected.to_string(),
            "theme preset: another theme is installed"
        );
        assert_eq!(
            theme(state).extended_palette().background.base.color,
            expected.extended_palette().background.base.color,
            "theme preset: the installed theme did not follow the choice"
        );
    }

    // -----------------------------------------------------------------------
    // Theme installs and the command line
    // -----------------------------------------------------------------------

    /// The colour mode the state is not drawn in: installing in it changes
    /// what is on screen, so an install that failed and kept it is caught.
    fn other_mode(state: &State) -> AppColorMode {
        if state.is_dark {
            AppColorMode::Light
        } else {
            AppColorMode::Dark
        }
    }

    /// What the theme and colour-mode pickers show, and what is drawn.
    fn shown(state: &State) -> (ThemeChoice, AppColorMode, bool, String, Color) {
        let drawn = theme(state);
        (
            state.current_choice.clone(),
            state.color_mode,
            state.is_dark,
            drawn.to_string(),
            drawn.extended_palette().background.base.color,
        )
    }

    /// The command line of `args`, each flag with the value `main` would
    /// have parsed for it.
    fn command_line(args: &[(&str, &str)]) -> CliArgs {
        let mut cli = CliArgs::default();
        for (flag, value) in args {
            let value = Some(value.to_string());
            match *flag {
                "--theme" => cli.theme = value,
                "--variant" => cli.variant = value,
                "--icon-set" => cli.icon_set = value,
                other => panic!("the tests do not pass {other}"),
            }
        }
        cli
    }

    /// The icon choice a preset's install derives where the choice follows
    /// the preset (`default_icon_choice`).
    fn preset_icon_choice(key: &str, is_dark: bool) -> Option<IconSetChoice> {
        let mode = if is_dark {
            native_theme_iced::ColorMode::Dark
        } else {
            native_theme_iced::ColorMode::Light
        };
        let resolved = native_theme::theme::Theme::preset(key)
            .ok()?
            .resolve(mode)
            .ok()?;
        let icon_theme = resolved.icon_theme.map(Cow::into_owned);
        Some(default_icon_choice(
            resolved.icon_set,
            icon_theme
                .as_deref()
                .filter(|_| resolved.icon_theme_explicit),
        ))
    }

    /// A theme that fails to load leaves the installed one on screen, and
    /// the theme and colour-mode pickers showing it and the mode it is drawn
    /// in, never a choice nothing on screen follows.
    #[test]
    fn a_failed_install_keeps_the_theme_and_mode_shown() {
        let mut state = State::default();
        let before = shown(&state);
        let failing = ThemeChoice::Preset("no-such-preset".to_string());

        let mode = other_mode(&state);
        assert!(
            !state.install_in_mode(failing.clone(), mode),
            "no-such-preset installed"
        );
        assert_eq!(
            shown(&state),
            before,
            "a failed install in {mode} changed what the pickers show or what is drawn"
        );
        assert!(
            state.error_message.is_some(),
            "a failed install reported nothing"
        );

        let _ = update(&mut state, Message::ThemeSelected(failing));
        assert_eq!(
            shown(&state),
            before,
            "the theme picker shows a theme that failed to load"
        );
    }

    /// A system icon theme that cannot be detected is shown as such: the
    /// Icons page gives the reason where it would name the theme, never a
    /// theme nothing detected.
    #[test]
    fn a_failed_icon_theme_detection_is_shown_as_a_failure() {
        let state = State {
            detect_icon_theme: || {
                Err(native_theme::error::Error::PlatformUnsupported {
                    platform: "the test's failing detection",
                })
            },
            ..State::default()
        };
        assert_eq!(
            state.system_icon_theme_label(),
            "System icon theme: unavailable (platform not supported: the test's failing detection)"
        );
    }

    /// After startup, an OS theme that fails to read is a failed install
    /// like any other: the installed theme stays drawn and named, never
    /// adwaita under the `default` entry. Only at startup, with nothing
    /// installed yet, is adwaita the fallback.
    #[test]
    fn a_failed_os_theme_read_keeps_the_installed_theme() {
        let mut state = State::default();
        let preset = match native_theme::theme::Theme::list_presets_for_platform().first() {
            Some(info) => info.key,
            None => panic!("this platform offers no preset"),
        };
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset(preset.to_string())),
        );
        state.read_system_theme = || {
            Err(native_theme::error::Error::PlatformUnsupported {
                platform: "the test's failing read",
            })
        };
        let before = shown(&state);

        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::OsTheme(String::new())),
        );
        assert_eq!(
            shown(&state),
            before,
            "a failed OS-theme read replaced {preset}"
        );
        assert!(
            state.error_message.is_some(),
            "a failed OS-theme read reported nothing"
        );
    }

    /// An OS theme that fails to read at startup leaves adwaita drawn, and
    /// the theme picker naming adwaita, with the failed read in the banner:
    /// a colour-mode change, from the picker or `--variant`, and a rebuild
    /// by the theme watcher install adwaita again, in the mode chosen.
    #[test]
    fn the_startup_fallback_is_adwaita_in_every_mode() {
        fn failing_read() -> native_theme::Result<native_theme::SystemTheme> {
            Err(native_theme::error::Error::PlatformUnsupported {
                platform: "the test's failing read",
            })
        }
        fn installed(state: &State) -> (ThemeChoice, AppColorMode, Option<String>) {
            (
                state.current_choice.clone(),
                state.color_mode,
                state.error_message.clone(),
            )
        }
        let adwaita = ThemeChoice::Preset("adwaita".to_string());
        let mut state = State::starting_with(failing_read);
        assert_eq!(state.current_choice, adwaita, "the picker at startup");
        assert!(
            state.error_message.is_some(),
            "the failed OS-theme read at startup reported nothing"
        );

        let (other, other_name) = if state.is_dark {
            (AppColorMode::Light, "light")
        } else {
            (AppColorMode::Dark, "dark")
        };
        let _ = update(&mut state, Message::ColorModeSelected(other));
        assert_eq!(
            installed(&state),
            (adwaita.clone(), other, None),
            "a colour-mode change after the startup fallback"
        );

        let mut state = State::starting_with(failing_read);
        apply_cli_args(&mut state, &command_line(&[("--variant", other_name)]));
        assert_eq!(
            installed(&state),
            (adwaita.clone(), other, None),
            "--variant {other_name} after the startup fallback"
        );

        state.error_message = Some("the banner before the rebuild".to_string());
        state.rebuild_theme();
        assert_eq!(
            installed(&state),
            (adwaita, other, None),
            "the theme watcher's rebuild after the startup fallback"
        );
    }

    /// `--icon-set` takes exactly what the icon-theme picker offers: a
    /// bundled set, `system` or `freedesktop` (the system icon theme), or an
    /// installed theme the picker lists (`installed_themes`). Anything else
    /// is reported and ignored, also a theme with an `index.theme` the
    /// picker does not list, such as `hicolor`. The picker's list is set
    /// here, so the test does not depend on the themes of the host.
    #[test]
    fn the_icon_set_flag_takes_what_the_picker_offers() {
        let mut state = State::default();
        let listed = "a-listed-icon-theme";
        state.installed_themes = vec![listed.to_string()];

        apply_cli_args(&mut state, &command_line(&[("--icon-set", "freedesktop")]));
        assert_eq!(
            state.icon_set_choice,
            IconSetChoice::System,
            "--icon-set freedesktop is not the system icon theme"
        );

        apply_cli_args(&mut state, &command_line(&[("--icon-set", "material")]));
        assert_eq!(state.icon_set_choice, IconSetChoice::Material);
        for unlisted in ["no-such-icon-theme", "hicolor"] {
            apply_cli_args(&mut state, &command_line(&[("--icon-set", unlisted)]));
            assert_eq!(
                state.icon_set_choice,
                IconSetChoice::Material,
                "--icon-set {unlisted}, which the picker does not offer, replaced the icon set"
            );
        }

        apply_cli_args(&mut state, &command_line(&[("--icon-set", listed)]));
        assert_eq!(
            state.icon_set_choice,
            IconSetChoice::Freedesktop(listed.to_string()),
            "--icon-set {listed}, which the picker offers, is not chosen"
        );
    }

    /// `--tab` takes each tab's name, and `textinputs` and `thememap` for
    /// two of them; another name is reported with the tabs it could have
    /// been.
    #[test]
    fn the_tab_flag_names_every_tab() {
        for &tab in Tab::ALL {
            assert_eq!(CliArgs::parse_tab(tab.flag()), Ok(tab));
        }
        assert_eq!(CliArgs::parse_tab("textinputs"), Ok(Tab::TextInputs));
        assert_eq!(CliArgs::parse_tab("thememap"), Ok(Tab::ThemeMap));
        match CliArgs::parse_tab("no-such-tab") {
            Ok(tab) => panic!("--tab no-such-tab opened {tab:?}"),
            Err(error) => {
                for tab in Tab::ALL {
                    assert!(error.contains(tab.flag()), "{error}");
                }
            }
        }
    }

    /// Where no freedesktop icon theme is installed -- off Linux, always --
    /// `--icon-set` says so, rather than end its report on an empty list.
    #[test]
    fn the_icon_set_flag_says_when_no_icon_theme_is_installed() {
        match CliArgs::icon_set_choice("no-such-icon-theme", &[]) {
            Ok(choice) => panic!("--icon-set no-such-icon-theme chose {choice:?}"),
            Err(error) => assert!(error.ends_with("; none are installed"), "{error}"),
        }
    }

    /// A freedesktop icon theme's spinner is that theme's own, and a theme
    /// that has none shows none: never the system theme's.
    #[test]
    fn the_spinner_comes_from_the_chosen_icon_theme() {
        let mut state = State::default();
        if state.installed_themes.is_empty() {
            eprintln!(
                "the_spinner_comes_from_the_chosen_icon_theme: this host has no freedesktop \
                 icon theme, so no theme's spinner was checked"
            );
        }
        for name in state.installed_themes.clone() {
            let _ = update(
                &mut state,
                Message::IconSetSelected(IconSetChoice::Freedesktop(name.clone())),
            );
            let expected = FreedesktopLoader::load_indicator(Some(&name))
                .and_then(|anim| to_svg_handle(anim.first_frame(), None));
            let spinners = state.animated_frames.len() + state.animated_spins.len();
            assert_eq!(
                state.animated_static.first().map(|(_, handle)| handle),
                expected.as_ref(),
                "{name}: the spinner is not the one {name} has"
            );
            assert_eq!(
                spinners,
                usize::from(expected.is_some()),
                "{name}: {spinners} spinners animate"
            );
        }
    }

    /// A freedesktop theme without a spinner gets none, never the system
    /// theme's: checked with a theme that does not exist, so on every host.
    #[test]
    fn a_freedesktop_theme_without_a_spinner_gets_none() {
        let theme = "no-such-icon-theme";
        let (frames, _, _, spins, _, _, statics) =
            build_animation_caches(IconSet::Freedesktop, Some(theme));
        assert!(
            frames.is_empty() && spins.is_empty() && statics.is_empty(),
            "{theme} has a spinner: {} frame animations, {} spins, {} static frames",
            frames.len(),
            spins.len(),
            statics.len()
        );
    }

    /// A choice that followed the preset keeps following it, also where it
    /// followed it onto `system` (`default_icon_choice`, where the preset's
    /// own icon theme is not installed); the user's pick, `system` too,
    /// stays chosen across a preset change.
    #[test]
    fn an_icon_choice_that_followed_the_preset_keeps_following_it() {
        let mut state = State::default();
        let is_dark = state.is_dark;
        let followed = native_theme::theme::Theme::list_presets_for_platform()
            .iter()
            .find_map(|info| match preset_icon_choice(info.key, is_dark) {
                Some(choice @ IconSetChoice::Default(_)) => Some((info.key.to_string(), choice)),
                _ => None,
            });
        let (preset, expected) = match followed {
            Some(followed) => followed,
            None => panic!("no preset of this platform names an available icon theme"),
        };

        state.icon_set_choice = IconSetChoice::System;
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset(preset.clone())),
        );
        assert_eq!(
            state.icon_set_choice, expected,
            "a choice that followed the preset onto system did not follow {preset}"
        );

        let _ = update(&mut state, Message::IconSetSelected(IconSetChoice::System));
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::OsTheme(String::new())),
        );
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset(preset.clone())),
        );
        assert_eq!(
            state.icon_set_choice,
            IconSetChoice::System,
            "the user's pick of system did not stay chosen across a preset change"
        );
    }

    /// `--variant` and `--theme` values the showcase cannot honour are
    /// reported and ignored: the state stays what it would have been
    /// without the flag. `--variant system` follows the OS, `--theme
    /// default` is the OS theme, and a preset of another platform is not
    /// installed.
    #[test]
    fn the_command_line_rejects_what_it_cannot_honour() {
        let mut state = State::default();
        let before = shown(&state);
        apply_cli_args(&mut state, &command_line(&[("--variant", "sideways")]));
        assert_eq!(
            shown(&state),
            before,
            "--variant sideways changed the state"
        );

        let other = other_mode(&state);
        let other_name = if other == AppColorMode::Dark {
            "dark"
        } else {
            "light"
        };
        apply_cli_args(&mut state, &command_line(&[("--variant", other_name)]));
        assert_eq!(state.color_mode, other, "--variant {other_name}");
        apply_cli_args(&mut state, &command_line(&[("--variant", "system")]));
        assert_eq!(state.color_mode, AppColorMode::System, "--variant system");

        let offered = native_theme::theme::Theme::list_presets_for_platform();
        let foreign = native_theme::theme::Theme::list_presets()
            .iter()
            .find(|info| !offered.iter().any(|o| o.key == info.key))
            .map(|info| info.key);
        let foreign = match foreign {
            Some(key) => key,
            None => panic!("every bundled preset is offered on this platform"),
        };
        let before = shown(&state);
        apply_cli_args(&mut state, &command_line(&[("--theme", foreign)]));
        assert_eq!(
            shown(&state),
            before,
            "--theme {foreign} installed a preset of another platform"
        );
        apply_cli_args(&mut state, &command_line(&[("--theme", "no-such-preset")]));
        assert_eq!(
            shown(&state),
            before,
            "--theme no-such-preset changed the state"
        );

        apply_cli_args(
            &mut state,
            &command_line(&[("--theme", foreign), ("--variant", other_name)]),
        );
        assert_eq!(
            (&state.current_choice, state.color_mode),
            (&before.0, other),
            "--theme {foreign} --variant {other_name}: not the variant alone"
        );

        let preset = match offered.first() {
            Some(info) => info.key,
            None => panic!("this platform offers no preset"),
        };
        let _ = update(
            &mut state,
            Message::ThemeSelected(ThemeChoice::Preset(preset.to_string())),
        );
        apply_cli_args(&mut state, &command_line(&[("--theme", "default")]));
        assert!(
            matches!(state.current_choice, ThemeChoice::OsTheme(_)),
            "--theme default: {:?}",
            state.current_choice
        );
        assert!(
            state.error_message.is_none(),
            "--theme default: {:?}",
            state.error_message
        );
    }

    /// Which list item of [`MARKDOWN_SAMPLE`] holds the link, and where it
    /// points — read from the document itself rather than copied out of it.
    fn markdown_link() -> (usize, &'static str) {
        let items: Vec<&'static str> = MARKDOWN_SAMPLE
            .lines()
            .filter(|line| line.starts_with("- "))
            .collect();
        let nth = match items.iter().position(|line| line.contains("](")) {
            Some(index) => index + 1,
            None => panic!("MARKDOWN_SAMPLE has no link in a list item"),
        };
        let line = items[nth - 1];
        let (start, end) = match (line.find("]("), line.rfind(')')) {
            (Some(start), Some(end)) => (start + 2, end),
            _ => panic!("MARKDOWN_SAMPLE's link is not an inline link"),
        };
        (nth, &line[start..end])
    }

    /// The bounds of the `nth` list item `markdown::view` lays out.
    ///
    /// A link lives in a `rich_text` span, and `rich_text` implements no
    /// `Widget::operate`, so no selector can see it. What `markdown` does
    /// expose is the row it puts each list item in: a bullet `text` followed
    /// by the item's own container. Counting the bullets picks the item out.
    fn markdown_item_bounds(ui: &mut Simulator<'_, Message>, nth: usize) -> Rectangle {
        let mut bullets = 0usize;
        let found = ui.find(move |candidate: Candidate<'_>| -> Option<Rectangle> {
            match candidate {
                Candidate::Text {
                    content: "\u{2022}",
                    ..
                } => {
                    bullets += 1;
                    None
                }
                Candidate::Container {
                    visible_bounds: Some(bounds),
                    ..
                } if bullets == nth => Some(bounds),
                _ => None,
            }
        });
        match found {
            Ok(bounds) => bounds,
            Err(error) => panic!("markdown list item {nth}: {error}"),
        }
    }

    // -----------------------------------------------------------------------
    // styles_cover_every_widget_shown
    // -----------------------------------------------------------------------

    /// The showcase's own source, comments and string literals removed.
    const SHOWCASE: &str = include_str!("showcase-iced.rs");

    /// One row of the coverage table: a widget constructor, and the
    /// `styles::*` functions that dress what it builds.
    struct Dressed {
        ctor: &'static str,
        /// One entry per style the constructor has to be given; a site clears
        /// an entry by carrying any one of its alternatives. A widget with
        /// two dressable parts — a picker and the menu it drops — therefore
        /// has two entries, and losing either is a failure.
        styles: &'static [&'static [&'static str]],
    }

    /// Every constructor in the showcase that has a `styles::*` function.
    ///
    /// `container(` is deliberately absent (spec §6.2): most of them are
    /// layout and carry no class at all. `styles::container_card`'s own call
    /// sites are covered by the second half of the test, which asks every
    /// public style function for at least one caller.
    const DRESSED: &[Dressed] = &[
        Dressed {
            ctor: "button",
            styles: &[&[
                "styles::button",
                "styles::button_primary",
                "styles::button_danger",
                "styles::button_success",
                "styles::button_warning",
                "styles::button_link",
            ]],
        },
        Dressed {
            ctor: "text_input",
            styles: &[&["styles::text_input"]],
        },
        Dressed {
            ctor: "text_editor",
            styles: &[&["styles::text_editor"]],
        },
        Dressed {
            ctor: "checkbox",
            styles: &[&["styles::checkbox"]],
        },
        Dressed {
            ctor: "radio",
            styles: &[&["styles::radio"]],
        },
        Dressed {
            ctor: "toggler",
            styles: &[&["styles::toggler"]],
        },
        Dressed {
            ctor: "pick_list",
            styles: &[&["styles::pick_list"], &["styles::menu"]],
        },
        Dressed {
            ctor: "combo_box",
            styles: &[&["styles::text_input"], &["styles::menu"]],
        },
        Dressed {
            ctor: "scrollable",
            styles: &[&["styles::scrollable"], &["styles::scrollbar"]],
        },
        Dressed {
            ctor: "slider",
            styles: &[&["styles::slider"]],
        },
        Dressed {
            ctor: "vertical_slider",
            styles: &[&["styles::slider"]],
        },
        Dressed {
            ctor: "progress_bar",
            styles: &[&["styles::progress_bar"]],
        },
        Dressed {
            ctor: "tooltip",
            styles: &[&["styles::tooltip"]],
        },
        Dressed {
            ctor: "rule::horizontal",
            styles: &[&["styles::rule"]],
        },
        Dressed {
            ctor: "rule::vertical",
            styles: &[&["styles::rule"]],
        },
    ];

    /// The iced classes the connector replaces. A widget wearing one of these
    /// is a widget the platform's theme never reached.
    const REPLACED: &[&str] = &[
        "button::primary",
        "button::secondary",
        "button::danger",
        "button::success",
        "button::text",
        "container::rounded_box",
    ];

    /// Every widget the showcase renders wears the platform's own style.
    #[test]
    fn styles_cover_every_widget_shown() {
        let source = strip_comments_and_strings(SHOWCASE);

        for replaced in REPLACED {
            if let Some(at) = source.find(replaced) {
                panic!(
                    "{replaced} is still used at showcase-iced.rs:{}",
                    line_at(&source, at)
                );
            }
        }

        let mut total = 0usize;
        for row in DRESSED {
            let sites = call_sites(&source, row.ctor);
            assert!(
                !sites.is_empty(),
                "{}( has no call site left in the showcase",
                row.ctor
            );
            // The invariant `call_sites` rests on: the showcase imports its
            // constructors and calls them bare.
            let qualified = format!("widget::{}(", row.ctor);
            assert!(
                !source.contains(&qualified),
                "{qualified} is a call site this test cannot see; the showcase \
                 calls its constructors by their imported names only"
            );
            total += sites.len();

            for required in row.styles {
                let bare: Vec<String> = sites
                    .iter()
                    .filter(|site| !is_dressed(&source, **site, required))
                    .map(|site| format!("showcase-iced.rs:{}", line_at(&source, *site)))
                    .collect();
                assert!(
                    bare.is_empty(),
                    "{}( is built with no {} at {}",
                    row.ctor,
                    required.join(" / "),
                    bare.join(", ")
                );
            }
        }
        assert!(total > 0, "the coverage table found nothing");

        // Every public style function the connector ships is demonstrated.
        for function in public_functions(include_str!("../src/styles.rs")) {
            let call = format!("styles::{function}");
            assert!(
                !call_sites(&source, &call).is_empty(),
                "styles::{function} has no call site in the showcase"
            );
        }
        if cfg!(feature = "iced_aw") {
            for function in public_functions(include_str!("../src/styles/aw.rs")) {
                let call = format!("styles::aw::{function}");
                assert!(
                    !call_sites(&source, &call).is_empty(),
                    "styles::aw::{function} has no call site in the showcase"
                );
            }
        }
    }

    /// The connector's padding helpers: each returns a padding whose stated
    /// sides are the theme's.
    const PADDING_HELPERS: &[&str] = &["button_padding(", "padding_or(", "stated_padding("];

    /// Every button dressed in a `styles::button*` class takes the theme's
    /// padding through one of the connector's padding helpers, rather than a
    /// spacing of the showcase's own or iced's default -- the push buttons
    /// `button.border.padding`, and the buttons that stand in for a menu item
    /// or a tab that widget's padding.
    #[test]
    fn themed_buttons_take_the_themes_padding() {
        let source = strip_comments_and_strings(SHOWCASE);
        let classes = [
            "styles::button",
            "styles::button_primary",
            "styles::button_danger",
            "styles::button_success",
            "styles::button_warning",
            "styles::button_link",
        ];
        let derived = theme_padding_bindings(&source);
        let from_theme = |args: &str| {
            PADDING_HELPERS.iter().any(|helper| args.contains(helper))
                || derived.iter().any(|name| {
                    args.match_indices(name).any(|(at, _)| {
                        let joined = |c: char| c.is_alphanumeric() || c == '_';
                        !args[..at].chars().next_back().is_some_and(joined)
                            && !args[at + name.len()..].chars().next().is_some_and(joined)
                    })
                })
        };

        let mut checked = 0usize;
        let mut unpadded = Vec::new();
        for site in call_sites(&source, "button") {
            if !is_dressed(&source, site, &classes) {
                continue;
            }
            checked += 1;
            if source[..site].trim_end().ends_with("apply_pad(") {
                continue;
            }
            let padded = close_of_call(&source, site).is_some_and(|end| {
                method_calls(&source, end)
                    .iter()
                    .any(|(name, args)| *name == "padding" && from_theme(args))
            });
            if !padded {
                unpadded.push(format!("showcase-iced.rs:{}", line_at(&source, site)));
            }
        }
        assert!(checked > 0, "no themed button was found");
        assert!(
            unpadded.is_empty(),
            "themed buttons not padded from the theme: {}",
            unpadded.join(", ")
        );
    }

    /// The card surfaces and the `iced_aw` tabs take the padding the theme
    /// states for them, not a spacing of the showcase's own: the `Card` and
    /// the `TabBar`/`Tabs` where every side is stated (`iced_aw` keeps their
    /// defaults private), the containers dressed as a card side by side.
    #[test]
    fn card_and_tab_padding_come_from_the_theme() {
        let source = strip_comments_and_strings(SHOWCASE);
        for per_section in [".padding_head(", ".padding_body(", ".padding_foot("] {
            if let Some(at) = source.find(per_section) {
                panic!(
                    "the card's padding is set section by section at \
                     showcase-iced.rs:{}, not from card.border.padding",
                    line_at(&source, at)
                );
            }
        }
        for call in [
            "stated_padding(&card_t.border.padding)",
            "stated_padding(&tab_t.border.padding)",
            "padding_or(&card.border.padding, Padding::ZERO)",
        ] {
            assert!(
                source.contains(call),
                "{call} has no call site in the showcase"
            );
        }
    }

    /// The `Card` is dismissed by a themed button of its own, never by
    /// `Card::on_close`: `iced_aw` 0.14.1 styles that close button with its
    /// default class (`widget/card.rs:193-206`), so the icon is white whatever
    /// `styles::aw::card` says.
    #[test]
    fn the_card_has_no_iced_aw_close_button() {
        let source = strip_comments_and_strings(SHOWCASE);
        if let Some(at) = source.find(".on_close(") {
            panic!(
                "Card::on_close is called at showcase-iced.rs:{}",
                line_at(&source, at)
            );
        }
    }

    /// The names `let` binds to a padding one of `PADDING_HELPERS` returns.
    fn theme_padding_bindings(source: &str) -> Vec<&str> {
        let mut names = Vec::new();
        for (at, _) in source.match_indices("let ") {
            let joined_before = source[..at]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
            if joined_before {
                continue;
            }
            let rest = &source[at + 4..];
            let rest = rest.strip_prefix("mut ").unwrap_or(rest);
            let name_len = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .count();
            if name_len == 0 {
                continue;
            }
            let name_at = source.len() - rest.len();
            let mut depth = 0i32;
            let mut end = source.len();
            for (offset, c) in source[name_at..].char_indices() {
                match c {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => {
                        depth -= 1;
                        if depth < 0 {
                            end = name_at + offset;
                            break;
                        }
                    }
                    ';' if depth == 0 => {
                        end = name_at + offset;
                        break;
                    }
                    _ => {}
                }
            }
            let statement = &source[name_at..end];
            if PADDING_HELPERS
                .iter()
                .any(|helper| statement.contains(helper))
            {
                names.push(&rest[..name_len]);
            }
        }
        names
    }

    /// The names of the `pub fn` items a module declares.
    fn public_functions(source: &str) -> Vec<String> {
        strip_comments_and_strings(source)
            .lines()
            .filter_map(|line| line.strip_prefix("pub fn "))
            .filter_map(|rest| rest.split(['(', '<']).next())
            .map(str::to_string)
            .collect()
    }

    /// Rust source with `//` comments, `/* */` comments and string, raw
    /// string and character literals removed, so that a widget named in prose
    /// is not counted as one built.
    ///
    /// A `'` opens a literal only when what follows it closes one; otherwise
    /// it is a lifetime or a loop label (`'a`, `'_`, `'static`), which is code
    /// and stays. Byte literals need no state of their own: `b` is an
    /// identifier character, and what follows it is lexed like any other
    /// string or character. Every removed character becomes a space, which
    /// keeps line numbers and offsets the same as the original's.
    fn strip_comments_and_strings(source: &str) -> String {
        #[derive(Clone, Copy, PartialEq)]
        enum Mode {
            Code,
            Line,
            Block(usize),
            Text,
            Raw(usize),
        }

        let bytes: Vec<char> = source.chars().collect();
        let mut out = String::with_capacity(source.len());
        let mut mode = Mode::Code;
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i];
            let next = bytes.get(i + 1).copied();
            match mode {
                Mode::Code => {
                    if c == '/' && next == Some('/') {
                        mode = Mode::Line;
                        out.push(' ');
                    } else if c == '/' && next == Some('*') {
                        mode = Mode::Block(1);
                        out.push(' ');
                    } else if let Some(hashes) = raw_string_open(&bytes, i) {
                        mode = Mode::Raw(hashes);
                        let opener = hashes + 2;
                        out.extend(std::iter::repeat_n(' ', opener));
                        i += opener;
                        continue;
                    } else if c == '"' {
                        mode = Mode::Text;
                        out.push(' ');
                    } else if let Some(len) = char_literal_len(&bytes, i) {
                        out.extend(std::iter::repeat_n(' ', len));
                        i += len;
                        continue;
                    } else {
                        out.push(c);
                    }
                }
                Mode::Line => {
                    if c == '\n' {
                        mode = Mode::Code;
                        out.push(c);
                    } else {
                        out.push(' ');
                    }
                }
                Mode::Block(depth) => {
                    if c == '/' && next == Some('*') {
                        mode = Mode::Block(depth + 1);
                        out.push(' ');
                        out.push(' ');
                        i += 2;
                        continue;
                    } else if c == '*' && next == Some('/') {
                        mode = if depth == 1 {
                            Mode::Code
                        } else {
                            Mode::Block(depth - 1)
                        };
                        out.push(' ');
                        out.push(' ');
                        i += 2;
                        continue;
                    }
                    out.push(if c == '\n' { c } else { ' ' });
                }
                Mode::Text => {
                    if c == '\\' {
                        out.push(' ');
                        out.push(' ');
                        i += 2;
                        continue;
                    } else if c == '"' {
                        mode = Mode::Code;
                    }
                    out.push(if c == '\n' { c } else { ' ' });
                }
                Mode::Raw(hashes) => {
                    if c == '"' && (1..=hashes).all(|k| bytes.get(i + k) == Some(&'#')) {
                        mode = Mode::Code;
                        let closer = hashes + 1;
                        out.extend(std::iter::repeat_n(' ', closer));
                        i += closer;
                        continue;
                    }
                    out.push(if c == '\n' { c } else { ' ' });
                }
            }
            i += 1;
        }
        out
    }

    /// The number of `#`s of the raw string opener at `at`, if one starts
    /// there.
    ///
    /// `at` is the `r`; a `b` in front of it is a prefix of the same literal
    /// and not the tail of an identifier, so `br"..."` is a raw string while
    /// `str"` — which no Rust ever writes — is not.
    fn raw_string_open(source: &[char], at: usize) -> Option<usize> {
        if source.get(at) != Some(&'r') {
            return None;
        }
        let before = at.checked_sub(1).and_then(|k| source.get(k)).copied();
        let head = if before == Some('b') {
            at.checked_sub(2).and_then(|k| source.get(k)).copied()
        } else {
            before
        };
        if head.is_some_and(|c| c.is_alphanumeric() || c == '_') {
            return None;
        }
        let mut hashes = 0;
        while source.get(at + 1 + hashes) == Some(&'#') {
            hashes += 1;
        }
        (source.get(at + 1 + hashes) == Some(&'"')).then_some(hashes)
    }

    /// The length of the character literal at `at`, if one starts there.
    ///
    /// A `'` that opens no literal is a lifetime or a loop label, which the
    /// stripper leaves alone. The escaped form is scanned to its closing
    /// quote, which is at most twelve characters away — `'\u{10FFFF}'` is the
    /// longest a character literal gets.
    fn char_literal_len(source: &[char], at: usize) -> Option<usize> {
        if source.get(at) != Some(&'\'') {
            return None;
        }
        match source.get(at + 1)? {
            '\\' => {
                let end = (at + 3..(at + 12).min(source.len()))
                    .find(|k| source.get(*k) == Some(&'\''))?;
                Some(end + 1 - at)
            }
            _ => (source.get(at + 2) == Some(&'\'')).then_some(3),
        }
    }

    /// The stripper, on a fixture of everything that has ever confused one.
    ///
    /// The fixture is a raw string, so the stripper's own source carries the
    /// forms it is asked about; that a `thing(` inside it is *not* counted is
    /// half of what this asserts.
    #[test]
    fn the_stripper_blanks_every_literal_and_keeps_every_lifetime() {
        let fixture = r##"
let quote = '"';
let escaped = '\'';
let slash = '\\';
let newline = '\n';
fn f<'a>(s: &'a str) -> &'a str { s }   // a " inside a line comment
let text = "a // b /* c */ d";
/* outer /* inner */ still a comment */
'outer: loop { break 'outer; }
let raw = r"first";
let hashed = r#"a "quoted" thing(  "#;
thing(0);
"##;
        let stripped = strip_comments_and_strings(fixture);

        assert_eq!(
            stripped.chars().count(),
            fixture.chars().count(),
            "a removed character must become a space, or offsets move"
        );
        assert_eq!(
            stripped.lines().count(),
            fixture.lines().count(),
            "a removed newline moves every line number after it"
        );

        for code in [
            "let quote =",
            "let escaped =",
            "let slash =",
            "let newline =",
            "fn f<'a>(s: &'a str) -> &'a str { s }",
            "'outer: loop { break 'outer; }",
            "let hashed =",
        ] {
            assert!(stripped.contains(code), "the stripper ate code: {code}");
        }
        assert_eq!(
            stripped.matches('\'').count(),
            5,
            "the only quotes left are the three lifetimes and the two labels"
        );

        for gone in [
            "\"",
            "//",
            "/*",
            "*/",
            "quoted",
            "inside a line comment",
            "still a comment",
        ] {
            assert!(
                !stripped.contains(gone),
                "the stripper left a literal or a comment behind: {gone}"
            );
        }

        let sites = call_sites(&stripped, "thing");
        assert_eq!(
            sites.len(),
            1,
            "one `thing(` is code and one is inside a raw string, got {sites:?}"
        );
    }

    /// Every place `name(` is called, as an offset into `source`.
    ///
    /// A call is only a call when nothing joins it to what precedes it, so
    /// `styles::slider(` is not a `slider(` site and `vertical_slider(` is not
    /// one either.
    ///
    /// That rule is what the coverage count rests on: a constructor is seen
    /// only under its bare imported name, and a site written
    /// `iced::widget::button(` would be counted by nothing.
    /// `styles_cover_every_widget_shown` asserts that the showcase contains no
    /// such qualified form, so the invariant fails loudly instead of quietly
    /// shrinking the count.
    fn call_sites(source: &str, name: &str) -> Vec<usize> {
        let mut sites = Vec::new();
        for (at, _) in source.match_indices(name) {
            let joined_before = source[..at]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == ':' || c == '.');
            if joined_before {
                continue;
            }
            let rest = &source[at + name.len()..];
            if rest.starts_with('(') {
                sites.push(at);
            }
        }
        sites
    }

    /// Does the widget built at `site` get one of `styles` put on it?
    ///
    /// The style normally sits in the method chain that follows the
    /// constructor. The tab strip builds its button once and dresses it in the
    /// two arms of an `if`, so a constructor bound to a name is followed
    /// through that name as well — which is why a file-wide count of
    /// `button(` against `styles::button*` does not balance (14 against 15)
    /// while every one of the 14 is dressed.
    fn is_dressed(source: &str, site: usize, styles: &[&str]) -> bool {
        let after = match close_of_call(source, site) {
            Some(end) => end,
            None => return false,
        };
        if styles.iter().any(|style| {
            method_chain(source, after)
                .iter()
                .any(|call| !call_sites(call, style).is_empty())
        }) {
            return true;
        }
        match binding_of(source, site) {
            Some(name) => {
                let block = &source[site..block_end(source, site)];
                let handle = format!("{name}.");
                block.match_indices(&handle).any(|(at, _)| {
                    let window = &block[at..block.len().min(at + 200)];
                    styles
                        .iter()
                        .any(|style| !call_sites(window, style).is_empty())
                })
            }
            None => false,
        }
    }

    /// The offset just past the `)` that closes the call starting at `site`.
    fn close_of_call(source: &str, site: usize) -> Option<usize> {
        let open = source[site..].find('(')? + site;
        let mut depth = 0usize;
        for (offset, c) in source[open..].char_indices() {
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(open + offset + 1);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// The arguments of every `.method(..)` chained onto the value that ends
    /// at `from`.
    fn method_chain(source: &str, from: usize) -> Vec<&str> {
        method_calls(source, from)
            .into_iter()
            .map(|(_, args)| args)
            .collect()
    }

    /// Every `.method(..)` chained onto the value that ends at `from`: its
    /// name, and its arguments with their parentheses.
    fn method_calls(source: &str, from: usize) -> Vec<(&str, &str)> {
        let mut calls = Vec::new();
        let mut at = from;
        loop {
            let rest = &source[at..];
            let skipped = rest.len() - rest.trim_start().len();
            at += skipped;
            if !source[at..].starts_with('.') {
                return calls;
            }
            let name_start = at + 1;
            let name_len = source[name_start..]
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .count();
            let open = name_start + name_len;
            if name_len == 0 || !source[open..].starts_with('(') {
                return calls;
            }
            match close_of_call(source, open) {
                Some(end) => {
                    calls.push((&source[name_start..open], &source[open..end]));
                    at = end;
                }
                None => return calls,
            }
        }
    }

    /// The name a `let` binds the call at `site` to, if it binds one.
    fn binding_of(source: &str, site: usize) -> Option<&str> {
        let head = &source[..site];
        let start = head.rfind(['{', '}', ';']).map(|at| at + 1).unwrap_or(0);
        let statement = head[start..].trim_start();
        let rest = statement.strip_prefix("let ")?;
        let rest = rest.strip_prefix("mut ").unwrap_or(rest);
        let name_len = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .count();
        let name = &rest[..name_len];
        let tail = rest[name_len..].trim_start();
        if name.is_empty() || !tail.starts_with('=') {
            return None;
        }
        Some(name)
    }

    /// Where the block that encloses `site` ends.
    fn block_end(source: &str, site: usize) -> usize {
        let mut depth = 0i32;
        for (offset, c) in source[site..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    if depth == 0 {
                        return site + offset;
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }
        source.len()
    }

    /// The 1-based line the given offset sits on.
    fn line_at(source: &str, at: usize) -> usize {
        source[..at].lines().count().max(1)
    }
}
