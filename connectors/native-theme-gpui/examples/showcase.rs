//! native-theme-gpui Comprehensive Designer Reference Tool
//!
//! A complete widget gallery with tooltip-based documentation for every
//! theme-controlled property. Demonstrates all gpui-component widgets,
//! all ThemeColor fields, all IconName variants, and full theme switching.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-gpui --example showcase
//! ```

use gpui::{
    div, prelude::*, px, rems, size, App, Application, Bounds, Context, Entity, Hsla,
    ImageSource, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    WindowBounds, WindowOptions,
};
use gpui_component::{
    accordion::Accordion,
    alert::Alert,
    badge::Badge,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonGroup, ButtonVariants},
    checkbox::Checkbox,
    collapsible::Collapsible,
    description_list::DescriptionList,
    divider::Divider,
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
    input::{Input, InputState, NumberInput, NumberInputEvent, StepAction},
    label::Label,
    link::Link,
    progress::Progress,
    radio::RadioGroup,
    resizable::{h_resizable, resizable_panel, v_resizable},
    scroll::ScrollableElement,
    select::{SearchableVec, Select, SelectEvent, SelectState},
    skeleton::Skeleton,
    slider::{Slider, SliderEvent, SliderState},
    spinner::Spinner,
    switch::Switch,
    tab::TabBar,
    tag::Tag,
    text::{TextView, TextViewStyle},
    theme::Theme,
    v_flex, ActiveTheme, Disableable, Icon, IconName, PixelsExt, Root, Sizable, Size, StyledExt,
};

use native_theme::{icon_name as native_icon_name, load_icon, IconData, IconRole, IconSet, NativeTheme, system_icon_set, system_icon_theme, bundled_icon_by_name};
#[cfg(target_os = "linux")]
use native_theme::{detect_linux_de, load_freedesktop_icon_by_name, system_is_dark};
use native_theme_gpui::icons::{to_image_source, to_image_source_colored, lucide_name_for_gpui_icon, material_name_for_gpui_icon};
#[cfg(target_os = "linux")]
use native_theme_gpui::icons::freedesktop_name_for_gpui_icon;
use native_theme_gpui::{pick_variant, to_theme};

// ---------------------------------------------------------------------------
// Tab indices
// ---------------------------------------------------------------------------
const TAB_BUTTONS: usize = 0;
const TAB_INPUTS: usize = 1;
const TAB_DATA: usize = 2;
const TAB_FEEDBACK: usize = 3;
const TAB_TYPOGRAPHY: usize = 4;
const TAB_LAYOUT: usize = 5;
const TAB_ICONS: usize = 6;
const TAB_THEME_MAP: usize = 7;

// ---------------------------------------------------------------------------
// Tooltip helpers
// ---------------------------------------------------------------------------

/// Convert Hsla to a #rrggbb hex string.
fn hsla_to_hex(c: Hsla) -> String {
    // Convert HSL to RGB through gpui's Rgba
    let rgba: gpui::Rgba = c.into();
    let r = (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (rgba.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (rgba.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Build a multi-line tooltip string for a widget.
///
/// - `name`: widget display name
/// - `colors`: slice of (role, field_name, live Hsla value)
/// - `config`: slice of (what, live_value_string)
/// - `not_themeable`: slice of (what, why)
fn widget_tooltip(
    name: &str,
    colors: &[(&str, &str, Hsla)],
    config: &[(&str, String)],
    not_themeable: &[(&str, &str)],
) -> String {
    let mut s = format!("{}\n", name);

    if !colors.is_empty() {
        s.push_str("\nTheme colors:\n");
        for (role, field, val) in colors {
            s.push_str(&format!("  {}: {} {}\n", role, field, hsla_to_hex(*val)));
        }
    }

    if !config.is_empty() {
        s.push_str("\nTheme config:\n");
        for (what, val) in config {
            s.push_str(&format!("  {}: {}\n", what, val));
        }
    }

    if !not_themeable.is_empty() {
        s.push_str("\nNot themeable:\n");
        for (what, why) in not_themeable {
            s.push_str(&format!("  {}: {}\n", what, why));
        }
    }

    s
}

/// Format original native-theme font settings (in points) for display.
fn format_font_info(fonts: &native_theme::ThemeFonts) -> String {
    let family = fonts.family.as_deref().unwrap_or("(default)");
    let size = fonts.size.map(|s| format!("{}pt", s)).unwrap_or("(default)".into());
    let mono = fonts.mono_family.as_deref().unwrap_or("(default)");
    let mono_size = fonts.mono_size.map(|s| format!("{}pt", s)).unwrap_or("(default)".into());
    format!(
        "\nTheme fonts:\n  Font: {} {}\n  Mono: {} {}",
        family, size, mono, mono_size,
    )
}

/// Like [`widget_tooltip`] but appends the active theme font settings.
fn widget_tooltip_themed(
    font_info: &str,
    name: &str,
    colors: &[(&str, &str, Hsla)],
    config: &[(&str, String)],
    not_themeable: &[(&str, &str)],
) -> String {
    let mut s = widget_tooltip(name, colors, config, not_themeable);
    s.push_str(font_info);
    s
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Return the preset name that best matches the current platform.
fn platform_preset_name() -> &'static str {
    #[cfg(target_os = "macos")]
    { "macos-sonoma" }
    #[cfg(target_os = "windows")]
    { "windows-11" }
    #[cfg(target_os = "linux")]
    {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        if desktop.split(':').any(|c| c == "KDE") {
            "kde-breeze"
        } else {
            "adwaita"
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    { "default" }
}

fn theme_names() -> Vec<SharedString> {
    let platform = platform_preset_name();
    let mut names: Vec<SharedString> = NativeTheme::list_presets()
        .iter()
        .map(|s| {
            if *s == "default" {
                SharedString::from(format!("default ({})", platform))
            } else {
                SharedString::from(s.to_string())
            }
        })
        .collect();
    names.push("OS Theme".into());
    names
}

fn section(title: impl Into<SharedString>) -> Label {
    Label::new(title).text_size(px(13.0)).font_semibold()
}

/// A color swatch: small rounded square + label.
fn color_swatch(name: &str, color: Hsla) -> impl IntoElement {
    let hex = hsla_to_hex(color);
    let label_text: SharedString = format!("{} {}", name, hex).into();
    h_flex()
        .gap_2()
        .items_center()
        .child(
            div()
                .size(px(16.0))
                .rounded(px(2.0))
                .bg(color)
                .border_1()
                .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3)),
        )
        .child(Label::new(label_text).text_sm())
}

// ---------------------------------------------------------------------------
// Icon loading helper
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

/// Pre-load all 42 icons for the given icon set name, tracking source.
fn load_all_icons(icon_set: &str) -> Vec<(IconRole, Option<IconData>, IconSource)> {
    // For system icon sets, pre-load the Material set so we can detect fallbacks
    // by comparing SVG bytes.
    let is_system_set = matches!(icon_set, "freedesktop" | "sf-symbols" | "segoe-fluent");
    let material_icons: Vec<Option<IconData>> = if is_system_set {
        IconRole::ALL
            .iter()
            .map(|role| load_icon(*role, "material"))
            .collect()
    } else {
        vec![]
    };

    IconRole::ALL
        .iter()
        .enumerate()
        .map(|(i, role)| {
            let data = load_icon(*role, icon_set);
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
                        // Material has no icon for this role, so it must be system
                        IconSource::System
                    }
                }
                (Some(_), true) => {
                    // RGBA or other data comes from native APIs, always system
                    IconSource::System
                }
            };
            (*role, data, source)
        })
        .collect()
}

/// Check if the given icon set name matches the current platform.
fn is_native_icon_set(name: &str) -> bool {
    match name {
        "freedesktop" => cfg!(target_os = "linux"),
        "sf-symbols" => cfg!(any(target_os = "macos", target_os = "ios")),
        "segoe-fluent" => cfg!(target_os = "windows"),
        "material" | "lucide" | "gpui-builtin" => true, // bundled, always available
        _ => false,
    }
}

/// Reverse lookup: find the IconRole for a gpui-component icon name string.
///
/// We match by Lucide icon name string since `IconName` doesn't implement `PartialEq`.
fn role_for_gpui_icon(gpui_name: &str) -> Option<IconRole> {
    // Static table mapping gpui-component icon names to IconRole, derived from
    // the connector's icon_name() mapping.
    match gpui_name {
        "TriangleAlert" => Some(IconRole::DialogWarning),
        "CircleX" => Some(IconRole::DialogError),
        "Info" => Some(IconRole::DialogInfo),
        "CircleCheck" => Some(IconRole::DialogSuccess),
        "WindowClose" => Some(IconRole::WindowClose),
        "WindowMinimize" => Some(IconRole::WindowMinimize),
        "WindowMaximize" => Some(IconRole::WindowMaximize),
        "WindowRestore" => Some(IconRole::WindowRestore),
        "Delete" => Some(IconRole::ActionDelete),
        "Copy" => Some(IconRole::ActionCopy),
        "Undo2" => Some(IconRole::ActionUndo),
        "Redo2" => Some(IconRole::ActionRedo),
        "Search" => Some(IconRole::ActionSearch),
        "Settings" => Some(IconRole::ActionSettings),
        "Plus" => Some(IconRole::ActionAdd),
        "Minus" => Some(IconRole::ActionRemove),
        "ChevronLeft" => Some(IconRole::NavBack),
        "ChevronRight" => Some(IconRole::NavForward),
        "ChevronUp" => Some(IconRole::NavUp),
        "ChevronDown" => Some(IconRole::NavDown),
        "Menu" => Some(IconRole::NavMenu),
        "File" => Some(IconRole::FileGeneric),
        "FolderClosed" => Some(IconRole::FolderClosed),
        "FolderOpen" => Some(IconRole::FolderOpen),
        "Loader" => Some(IconRole::StatusLoading),
        "Check" => Some(IconRole::StatusCheck),
        "User" => Some(IconRole::UserAccount),
        "Bell" => Some(IconRole::Notification),
        _ => None,
    }
}

/// The 86 gpui-component IconName variants shown in the gallery.
const GPUI_ICONS: &[(&str, IconName)] = &[
    ("ALargeSmall", IconName::ALargeSmall),
    ("ArrowDown", IconName::ArrowDown),
    ("ArrowLeft", IconName::ArrowLeft),
    ("ArrowRight", IconName::ArrowRight),
    ("ArrowUp", IconName::ArrowUp),
    ("Asterisk", IconName::Asterisk),
    ("Bell", IconName::Bell),
    ("BookOpen", IconName::BookOpen),
    ("Bot", IconName::Bot),
    ("Building2", IconName::Building2),
    ("Calendar", IconName::Calendar),
    ("CaseSensitive", IconName::CaseSensitive),
    ("ChartPie", IconName::ChartPie),
    ("Check", IconName::Check),
    ("ChevronDown", IconName::ChevronDown),
    ("ChevronLeft", IconName::ChevronLeft),
    ("ChevronRight", IconName::ChevronRight),
    ("ChevronsUpDown", IconName::ChevronsUpDown),
    ("ChevronUp", IconName::ChevronUp),
    ("CircleCheck", IconName::CircleCheck),
    ("CircleUser", IconName::CircleUser),
    ("CircleX", IconName::CircleX),
    ("Close", IconName::Close),
    ("Copy", IconName::Copy),
    ("Dash", IconName::Dash),
    ("Delete", IconName::Delete),
    ("Ellipsis", IconName::Ellipsis),
    ("EllipsisVertical", IconName::EllipsisVertical),
    ("ExternalLink", IconName::ExternalLink),
    ("Eye", IconName::Eye),
    ("EyeOff", IconName::EyeOff),
    ("File", IconName::File),
    ("Folder", IconName::Folder),
    ("FolderClosed", IconName::FolderClosed),
    ("FolderOpen", IconName::FolderOpen),
    ("Frame", IconName::Frame),
    ("GalleryVerticalEnd", IconName::GalleryVerticalEnd),
    ("GitHub", IconName::GitHub),
    ("Globe", IconName::Globe),
    ("Heart", IconName::Heart),
    ("HeartOff", IconName::HeartOff),
    ("Inbox", IconName::Inbox),
    ("Info", IconName::Info),
    ("Inspector", IconName::Inspector),
    ("LayoutDashboard", IconName::LayoutDashboard),
    ("Loader", IconName::Loader),
    ("LoaderCircle", IconName::LoaderCircle),
    ("Map", IconName::Map),
    ("Maximize", IconName::Maximize),
    ("Menu", IconName::Menu),
    ("Minimize", IconName::Minimize),
    ("Minus", IconName::Minus),
    ("Moon", IconName::Moon),
    ("Palette", IconName::Palette),
    ("PanelBottom", IconName::PanelBottom),
    ("PanelBottomOpen", IconName::PanelBottomOpen),
    ("PanelLeft", IconName::PanelLeft),
    ("PanelLeftClose", IconName::PanelLeftClose),
    ("PanelLeftOpen", IconName::PanelLeftOpen),
    ("PanelRight", IconName::PanelRight),
    ("PanelRightClose", IconName::PanelRightClose),
    ("PanelRightOpen", IconName::PanelRightOpen),
    ("Plus", IconName::Plus),
    ("Redo", IconName::Redo),
    ("Redo2", IconName::Redo2),
    ("Replace", IconName::Replace),
    ("ResizeCorner", IconName::ResizeCorner),
    ("Search", IconName::Search),
    ("Settings", IconName::Settings),
    ("Settings2", IconName::Settings2),
    ("SortAscending", IconName::SortAscending),
    ("SortDescending", IconName::SortDescending),
    ("SquareTerminal", IconName::SquareTerminal),
    ("Star", IconName::Star),
    ("StarOff", IconName::StarOff),
    ("Sun", IconName::Sun),
    ("ThumbsDown", IconName::ThumbsDown),
    ("ThumbsUp", IconName::ThumbsUp),
    ("TriangleAlert", IconName::TriangleAlert),
    ("Undo", IconName::Undo),
    ("Undo2", IconName::Undo2),
    ("User", IconName::User),
    ("WindowClose", IconName::WindowClose),
    ("WindowMaximize", IconName::WindowMaximize),
    ("WindowMinimize", IconName::WindowMinimize),
    ("WindowRestore", IconName::WindowRestore),
];

/// Pre-load native-theme icons for gpui-component IconName variants that have a
/// corresponding IconRole. Returns (name, IconName, Option<role>, Option<data>, source).
fn load_gpui_icons(
    icon_set: &str,
) -> Vec<(&'static str, IconName, Option<IconRole>, Option<IconData>, IconSource)> {
    if icon_set == "gpui-builtin" {
        // All icons rendered from gpui-component built-in; no native-theme data loaded
        return GPUI_ICONS
            .iter()
            .map(|(name, icon)| {
                let role = role_for_gpui_icon(name);
                (*name, icon.clone(), role, None, IconSource::Bundled)
            })
            .collect();
    }

    let is_system_set = matches!(icon_set, "freedesktop" | "sf-symbols" | "segoe-fluent");
    let icon_set_enum = IconSet::from_name(icon_set);

    // On Linux with freedesktop, detect DE + theme once for the whole batch
    #[cfg(target_os = "linux")]
    let (linux_de, fd_theme) = if is_system_set && matches!(icon_set_enum, Some(IconSet::Freedesktop)) {
        let de_str = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        (Some(detect_linux_de(&de_str)), Some(system_icon_theme()))
    } else {
        (None, None)
    };

    GPUI_ICONS
        .iter()
        .map(|(name, icon)| {
            let role = role_for_gpui_icon(name);

            // Try loading by IconRole first (existing path)
            if let Some(r) = role {
                let data = load_icon(r, icon_set);
                let source = match &data {
                    None => IconSource::NotFound,
                    Some(_) if !is_system_set => IconSource::Bundled,
                    Some(IconData::Svg(loaded)) => {
                        let mat = load_icon(r, "material");
                        if let Some(IconData::Svg(mat_bytes)) = &mat {
                            if loaded == mat_bytes {
                                IconSource::Fallback
                            } else {
                                IconSource::System
                            }
                        } else {
                            IconSource::System
                        }
                    }
                    Some(_) => IconSource::System,
                };
                // If system set returned a bundled fallback or not found, try
                // freedesktop_name_for_gpui_icon before giving up (no theme mixing)
                #[cfg(target_os = "linux")]
                if matches!(source, IconSource::Fallback | IconSource::NotFound) {
                    if let (Some(de), Some(theme)) = (&linux_de, &fd_theme) {
                        if let Some(fd_name) = freedesktop_name_for_gpui_icon(name, *de) {
                            if let Some(fd_data) = load_freedesktop_icon_by_name(fd_name, theme) {
                                return (*name, icon.clone(), Some(r), Some(fd_data), IconSource::System);
                            }
                        }
                        // System set but no system icon — mark not found (no bundled fallback)
                        return (*name, icon.clone(), Some(r), None, IconSource::NotFound);
                    }
                }
                return (*name, icon.clone(), Some(r), data, source);
            }

            // No IconRole mapping — try by-name lookup for the active icon set
            #[cfg(target_os = "linux")]
            if let (Some(de), Some(theme)) = (&linux_de, &fd_theme) {
                if let Some(fd_name) = freedesktop_name_for_gpui_icon(name, *de) {
                    if let Some(data) = load_freedesktop_icon_by_name(fd_name, theme) {
                        return (*name, icon.clone(), None, Some(data), IconSource::System);
                    }
                }
                // System set but no system icon — do NOT fall back to bundled
                return (*name, icon.clone(), None, None, IconSource::NotFound);
            }

            if let Some(set) = icon_set_enum {
                let lookup_name = match set {
                    IconSet::Lucide => lucide_name_for_gpui_icon(name),
                    IconSet::Material => material_name_for_gpui_icon(name),
                    _ => None,
                };
                if let Some(lname) = lookup_name {
                    if let Some(svg_bytes) = bundled_icon_by_name(set, lname) {
                        let data = Some(IconData::Svg(svg_bytes.to_vec()));
                        return (*name, icon.clone(), None, data, IconSource::Bundled);
                    }
                }
            }

            // Fallback: no icon data
            (*name, icon.clone(), None, None, IconSource::NotFound)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Widget Info panel – separate Entity so hover updates only re-render this
// small panel instead of the entire Showcase.
// ---------------------------------------------------------------------------

struct WidgetInfoPanel {
    text: String,
    input_state: Entity<InputState>,
    /// True when `text` changed and `input_state` needs syncing on next render.
    needs_sync: bool,
}

impl WidgetInfoPanel {
    fn set_text(&mut self, text: String, cx: &mut Context<Self>) {
        if self.text != text {
            self.text = text;
            self.needs_sync = true;
            cx.notify();
        }
    }
}

impl Render for WidgetInfoPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.needs_sync {
            self.needs_sync = false;
            let val = if self.text.is_empty() {
                SharedString::from("Hover over any widget to see its theme properties.")
            } else {
                SharedString::from(self.text.clone())
            };
            self.input_state.update(cx, |state, cx| {
                state.set_value(val, window, cx);
            });
        }

        v_flex()
            .p_3()
            .w_full()
            .child(Label::new("Widget Info").text_size(px(13.0)).font_semibold())
            .child(
                Input::new(&self.input_state)
                    .appearance(false)
                    .text_size(px(11.0)),
            )
    }
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// Color mode (light / dark / system)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
enum ColorMode {
    System,
    Light,
    Dark,
}

impl ColorMode {
    /// Resolve to a concrete is_dark bool.
    fn is_dark(self) -> bool {
        match self {
            ColorMode::Light => false,
            ColorMode::Dark => true,
            ColorMode::System => Self::system_prefers_dark(),
        }
    }

    /// Synchronously detect whether the OS prefers dark mode.
    fn system_prefers_dark() -> bool {
        #[cfg(target_os = "linux")]
        {
            system_is_dark()
        }
        #[cfg(not(target_os = "linux"))]
        {
            false // TODO: macOS / Windows detection
        }
    }

    /// Display label for the combobox, with system preference in parentheses.
    fn label(self) -> String {
        match self {
            ColorMode::System => {
                let actual = if Self::system_prefers_dark() { "Dark" } else { "Light" };
                format!("System ({actual})")
            }
            ColorMode::Light => "Light".into(),
            ColorMode::Dark => "Dark".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main view
// ---------------------------------------------------------------------------

struct Showcase {
    theme_select: Entity<SelectState<SearchableVec<SharedString>>>,
    current_theme_name: String,
    is_dark: bool,
    color_mode: ColorMode,
    dark_mode_select: Entity<SelectState<SearchableVec<SharedString>>>,
    /// Original native-theme fonts (in points), for display purposes.
    original_fonts: native_theme::ThemeFonts,

    active_tab: usize,

    input_state: Entity<InputState>,
    number_input_state: Entity<InputState>,
    slider_state: Entity<SliderState>,
    checkbox_a: bool,
    checkbox_b: bool,
    checkbox_c: bool,
    switch_on: bool,
    radio_index: Option<usize>,
    slider_value: f32,
    collapsible_open: bool,

    // Icon set selector state
    icon_set_select: Entity<SelectState<SearchableVec<SharedString>>>,
    icon_set_name: String,
    loaded_icons: Vec<(IconRole, Option<IconData>, IconSource)>,
    gpui_icons: Vec<(&'static str, IconName, Option<IconRole>, Option<IconData>, IconSource)>,
    /// Cached ImageSource per native icon (same indexing as loaded_icons).
    loaded_icon_sources: Vec<Option<ImageSource>>,
    /// Cached ImageSource per gpui icon (same indexing as gpui_icons).
    gpui_icon_sources: Vec<Option<ImageSource>>,
    /// Foreground color used when building the image source caches.
    icon_cache_fg: Hsla,
    /// Whether the icon set follows the theme's default.
    use_default_icon_set: bool,
    /// The current theme's variant icon_set (for reading default).
    current_variant_icon_set: Option<String>,
    /// Widget Info sidebar panel (separate Entity for independent re-render).
    widget_info_panel: Entity<WidgetInfoPanel>,
}

impl Showcase {
    /// Rebuild cached `ImageSource` objects for all loaded icons.
    ///
    /// Called when icons are loaded or the theme foreground color changes,
    /// so that `render_icons_tab` can reuse the cached sources instead of
    /// re-creating `Image` + `Arc` allocations and re-colorizing SVGs on
    /// every frame.
    fn rebuild_icon_caches(&mut self, fg: Hsla) {
        self.icon_cache_fg = fg;
        self.loaded_icon_sources = self
            .loaded_icons
            .iter()
            .map(|(_, data, source)| {
                data.as_ref().map(|d| {
                    if *source == IconSource::System {
                        to_image_source(d)
                    } else {
                        to_image_source_colored(d, fg)
                    }
                })
            })
            .collect();
        self.gpui_icon_sources = self
            .gpui_icons
            .iter()
            .map(|(_, _, _, data, source)| {
                data.as_ref().map(|d| {
                    if *source == IconSource::System {
                        to_image_source(d)
                    } else {
                        to_image_source_colored(d, fg)
                    }
                })
            })
            .collect();
    }

    /// Resolve the effective icon set name for the "default" selection.
    ///
    /// Uses the theme variant's icon_set if specified, otherwise falls back
    /// to the platform's system icon set.
    fn resolve_default_icon_set(&self) -> String {
        self.current_variant_icon_set
            .as_deref()
            .unwrap_or(system_icon_set().name())
            .to_string()
    }

    /// Convert a display name from the theme selector to the internal theme name.
    fn theme_internal_name(display: &str) -> String {
        if display.starts_with("default (") {
            "default".to_string()
        } else {
            display.to_string()
        }
    }

    /// Convert a display name from the icon theme selector to the internal icon set name.
    fn icon_set_internal_name(display: &str) -> String {
        if display.starts_with("default (") {
            "default".to_string()
        } else if display == "gpui-component built-in (Lucide)" {
            "gpui-builtin".to_string()
        } else if display == "Lucide (bundled)" {
            "lucide".to_string()
        } else if display == "Material (bundled)" {
            "material".to_string()
        } else if display.ends_with(" (system)") {
            // "<ThemeName> (system)" → platform icon set
            system_icon_set().name().to_string()
        } else {
            display.to_string()
        }
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let names = theme_names();
        let delegate = SearchableVec::new(names);

        let theme_select = cx.new(|cx| {
            SelectState::new(
                delegate,
                Some(gpui_component::IndexPath::default().row(0)),
                window,
                cx,
            )
        });

        cx.subscribe_in(
            &theme_select,
            window,
            |this: &mut Self, _entity, event: &SelectEvent<SearchableVec<SharedString>>, window, cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    let name = Self::theme_internal_name(&value.to_string());
                    this.current_theme_name = name.clone();
                    this.apply_theme_by_name(&name, window, cx);
                }
            },
        )
        .detach();

        // Color mode selector (System / Light / Dark)
        let color_mode = ColorMode::System;
        let color_mode_labels: Vec<SharedString> = [
            ColorMode::System,
            ColorMode::Light,
            ColorMode::Dark,
        ]
        .iter()
        .map(|m| SharedString::from(m.label()))
        .collect();
        let dark_mode_delegate = SearchableVec::new(color_mode_labels);
        let dark_mode_select = cx.new(|cx| {
            SelectState::new(
                dark_mode_delegate,
                Some(gpui_component::IndexPath::default().row(0)),
                window,
                cx,
            )
        });

        cx.subscribe_in(
            &dark_mode_select,
            window,
            |this: &mut Self, _entity, event: &SelectEvent<SearchableVec<SharedString>>, window, cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    let val = value.to_string();
                    let mode = if val.starts_with("System") {
                        ColorMode::System
                    } else if val == "Light" {
                        ColorMode::Light
                    } else {
                        ColorMode::Dark
                    };
                    this.set_color_mode(mode, window, cx);
                }
            },
        )
        .detach();

        let input_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("Type something here...", window, cx);
            state
        });

        let number_input_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("0", window, cx);
            state
        });

        cx.subscribe_in(
            &number_input_state,
            window,
            |_this: &mut Self, input, event: &NumberInputEvent, window, cx| {
                let NumberInputEvent::Step(action) = event;
                input.update(cx, |input, cx| {
                    let value = input.value();
                    let num: f64 = value.parse().unwrap_or(0.0);
                    let new_value = if *action == StepAction::Increment {
                        num + 1.0
                    } else {
                        num - 1.0
                    };
                    input.set_value(
                        SharedString::from(new_value.to_string()),
                        window,
                        cx,
                    );
                });
            },
        )
        .detach();

        let slider_state = cx.new(|_cx| SliderState::new().default_value(65.0));

        cx.subscribe_in(
            &slider_state,
            window,
            |this: &mut Self, _entity, event: &SliderEvent, _window, _cx| {
                let SliderEvent::Change(val) = event;
                this.slider_value = val.start();
            },
        )
        .detach();

        // Apply the initial "default" preset theme.
        let is_dark = color_mode.is_dark();
        let nt = NativeTheme::preset("default").expect("default preset must exist");
        let original_fonts = pick_variant(&nt, is_dark)
            .map(|v| v.fonts.clone())
            .unwrap_or_default();
        let current_variant_icon_set = pick_variant(&nt, is_dark)
            .and_then(|v| v.icon_set.clone());
        if let Some(variant) = pick_variant(&nt, is_dark) {
            let theme = to_theme(variant, "default", is_dark);
            *Theme::global_mut(cx) = theme;
            window.refresh();
        }

        // Resolve initial icon set from theme's default
        let initial_resolved = current_variant_icon_set
            .as_deref()
            .unwrap_or(system_icon_set().name())
            .to_string();
        let loaded_icons = load_all_icons(&initial_resolved);
        let gpui_icons = load_gpui_icons(&initial_resolved);

        // Icon theme selector – list detected system theme + bundled options
        let detected_theme = system_icon_theme();
        let default_label = format!("default ({})", detected_theme);
        let system_label = format!("{} (system)", detected_theme);
        let mut icon_theme_names: Vec<SharedString> = vec![
            default_label.into(),
            "gpui-component built-in (Lucide)".into(),
            "Lucide (bundled)".into(),
            "Material (bundled)".into(),
        ];
        icon_theme_names.push(system_label.into());
        let icon_set_delegate = SearchableVec::new(icon_theme_names);
        let icon_set_select = cx.new(|cx| {
            SelectState::new(
                icon_set_delegate,
                Some(gpui_component::IndexPath::default().row(0)),
                window,
                cx,
            )
        });

        cx.subscribe_in(
            &icon_set_select,
            window,
            |this: &mut Self, _entity, event: &SelectEvent<SearchableVec<SharedString>>, _window, cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    let display = value.to_string();
                    let internal = Self::icon_set_internal_name(&display);
                    this.use_default_icon_set = internal == "default";
                    let effective = if this.use_default_icon_set {
                        this.resolve_default_icon_set()
                    } else {
                        internal
                    };
                    this.icon_set_name = effective.clone();
                    this.loaded_icons = load_all_icons(&effective);
                    this.gpui_icons = load_gpui_icons(&effective);
                    let fg = cx.theme().foreground;
                    this.rebuild_icon_caches(fg);
                    cx.notify();
                }
            },
        )
        .detach();

        let fg = cx.theme().foreground;
        let mut showcase = Self {
            theme_select,
            current_theme_name: "default".into(),
            is_dark,
            color_mode,
            dark_mode_select,
            original_fonts,
            active_tab: TAB_BUTTONS,
            input_state,
            number_input_state,
            slider_state,
            checkbox_a: true,
            checkbox_b: false,
            checkbox_c: false,
            switch_on: false,
            radio_index: Some(0),
            slider_value: 65.0,
            collapsible_open: true,
            icon_set_select,
            icon_set_name: initial_resolved,
            loaded_icons,
            gpui_icons,
            loaded_icon_sources: Vec::new(),
            gpui_icon_sources: Vec::new(),
            icon_cache_fg: fg,
            use_default_icon_set: true,
            current_variant_icon_set,
            widget_info_panel: {
                let info_input = cx.new(|cx| {
                    let mut state = InputState::new(window, cx).auto_grow(4, 30);
                    state.set_placeholder("Hover over any widget…", window, cx);
                    state
                });
                cx.new(|_cx| WidgetInfoPanel {
                    text: String::new(),
                    input_state: info_input,
                    needs_sync: false,
                })
            },
        };
        showcase.rebuild_icon_caches(fg);
        showcase
    }

    fn apply_theme_by_name(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) {
        if name == "OS Theme" {
            Theme::sync_system_appearance(Some(window), cx);
            self.is_dark = cx.theme().is_dark();
            self.original_fonts = native_theme::ThemeFonts::default();
            let fg = cx.theme().foreground;
            self.rebuild_icon_caches(fg);
            return;
        }

        let nt = match NativeTheme::preset(name) {
            Ok(t) => t,
            Err(_) => return,
        };

        if let Some(variant) = pick_variant(&nt, self.is_dark) {
            self.original_fonts = variant.fonts.clone();
            self.current_variant_icon_set = variant.icon_set.clone();
            let theme = to_theme(variant, name, self.is_dark);
            *Theme::global_mut(cx) = theme;
            window.refresh();

            // If using default icon set, reload icons for the new theme's default
            if self.use_default_icon_set {
                let effective = self.resolve_default_icon_set();
                self.icon_set_name = effective.clone();
                self.loaded_icons = load_all_icons(&effective);
                self.gpui_icons = load_gpui_icons(&effective);
            }
            // Rebuild image caches for the new theme's foreground color
            let fg = cx.theme().foreground;
            self.rebuild_icon_caches(fg);
        }
    }

    fn set_color_mode(&mut self, mode: ColorMode, window: &mut Window, cx: &mut Context<Self>) {
        self.color_mode = mode;
        self.is_dark = mode.is_dark();
        let name = self.current_theme_name.clone();
        self.apply_theme_by_name(&name, window, cx);
    }

    /// Create a hover handler that updates the Widget Info panel.
    ///
    /// Captures a clone of the `WidgetInfoPanel` entity handle and updates it
    /// directly — the Showcase entity is never entered so it does **not**
    /// re-render, keeping hover updates cheap.
    fn set_info(&self, info: String) -> impl Fn(&bool, &mut Window, &mut App) + 'static {
        let panel = self.widget_info_panel.clone();
        move |hovered: &bool, _window: &mut Window, cx: &mut App| {
            if *hovered {
                panel.update(cx, |p, cx| {
                    p.set_text(info.clone(), cx);
                });
            }
        }
    }

    /// Create a hover handler using the standard widget_tooltip_themed format.
    fn hover_info(
        &self,
        fi: &str,
        name: &str,
        colors: &[(&str, &str, Hsla)],
        config: &[(&str, String)],
        not_themeable: &[(&str, &str)],
    ) -> impl Fn(&bool, &mut Window, &mut App) + 'static {
        let info = widget_tooltip_themed(fi, name, colors, config, not_themeable);
        self.set_info(info)
    }

    // -----------------------------------------------------------------------
    // Left sidebar: theme config inspector
    // -----------------------------------------------------------------------
    fn render_sidebar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let radius_str = format!("{}px", theme.radius.as_f32());
        let radius_lg_str = format!("{}px", theme.radius_lg.as_f32());
        let font_family_str = self.original_fonts.family.as_deref().unwrap_or("(default)").to_string();
        let font_size_str = self.original_fonts.size.map(|s| format!("{}pt", s)).unwrap_or("(default)".into());
        let mono_family_str = self.original_fonts.mono_family.as_deref().unwrap_or("(default)").to_string();
        let mono_size_str = self.original_fonts.mono_size.map(|s| format!("{}pt", s)).unwrap_or("(default)".into());
        let shadow_str = if theme.shadow { "true" } else { "false" };
        let scrollbar_str = format!("{:?}", theme.scrollbar_show);

        let md = format!(
            "### Theme Config Inspector\n\n\
             **radius:** {}\n\
             **radius_lg:** {}\n\
             **font_family:** {}\n\
             **font_size:** {}\n\
             **mono_font_family:** {}\n\
             **mono_font_size:** {}\n\
             **shadow:** {}\n\
             **scrollbar_show:** {}",
            radius_str, radius_lg_str, font_family_str, font_size_str,
            mono_family_str, mono_size_str, shadow_str, scrollbar_str,
        );

        let style = TextViewStyle::default()
            .paragraph_gap(rems(0.3))
            .heading_font_size(|_level, _base| px(13.0));

        v_flex()
            .p_3()
            .w_full()
            .child(
                TextView::markdown("config-inspector", SharedString::from(md), window, cx)
                    .selectable(true)
                    .style(style)
                    .text_xs(),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Buttons
    // -----------------------------------------------------------------------
    fn render_buttons_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_fonts);
        let t = cx.theme().clone();
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Button variants
            .child(section("Button Variants (all 10)"))
            .child(
                h_flex()
                    .gap_2()
                    .flex_wrap()
                    .child(
                        div()
                            .id("tt-btn-primary")
                            .child(Button::new("b-primary").label("Primary").primary())
                            .on_hover(self.hover_info(&fi,
                                "Button (Primary)",
                                &[
                                    ("bg", "primary", t.primary),
                                    ("text", "primary_foreground", t.primary_foreground),
                                    ("hover", "primary_hover", t.primary_hover),
                                    ("active", "primary_active", t.primary_active),
                                ],
                                &[
                                    ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                    ("shadow", format!("{}", t.shadow)),
                                ],
                                &[
                                    ("padding", "set per Size enum (XS/S/M/L)"),
                                    ("font-weight", "hardcoded"),
                                    ("min-height", "hardcoded"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-secondary")
                            .child(Button::new("b-secondary").label("Secondary"))
                            .on_hover(self.hover_info(&fi,
                                    "Button (Secondary)",
                                    &[
                                        ("bg", "secondary", t.secondary),
                                        ("text", "secondary_foreground", t.secondary_foreground),
                                        ("hover", "secondary_hover", t.secondary_hover),
                                        ("active", "secondary_active", t.secondary_active),
                                    ],
                                    &[
                                        ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                        ("shadow", format!("{}", t.shadow)),
                                    ],
                                    &[
                                        ("padding", "set per Size enum"),
                                        ("font-weight", "hardcoded"),
                                        ("min-height", "hardcoded"),
                                    ],
                                        )),
                    )
                    .child(
                        div()
                            .id("tt-btn-danger")
                            .child(Button::new("b-danger").label("Danger").danger())
                            .on_hover(self.hover_info(&fi,
                                    "Button (Danger)",
                                    &[
                                        ("bg", "danger", t.danger),
                                        ("text", "danger_foreground", t.danger_foreground),
                                        ("hover", "danger_hover", t.danger_hover),
                                        ("active", "danger_active", t.danger_active),
                                    ],
                                    &[
                                        ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                        ("shadow", format!("{}", t.shadow)),
                                    ],
                                    &[
                                        ("padding", "set per Size enum"),
                                        ("font-weight", "hardcoded"),
                                        ("min-height", "hardcoded"),
                                    ],
                                        )),
                    )
                    .child(
                        div()
                            .id("tt-btn-success")
                            .child(Button::new("b-success").label("Success").success())
                            .on_hover(self.hover_info(&fi,
                                    "Button (Success)",
                                    &[
                                        ("bg", "success", t.success),
                                        ("text", "success_foreground", t.success_foreground),
                                        ("hover", "success_hover", t.success_hover),
                                        ("active", "success_active", t.success_active),
                                    ],
                                    &[
                                        ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                        ("shadow", format!("{}", t.shadow)),
                                    ],
                                    &[
                                        ("padding", "set per Size enum"),
                                        ("font-weight", "hardcoded"),
                                        ("min-height", "hardcoded"),
                                    ],
                                        )),
                    )
                    .child(
                        div()
                            .id("tt-btn-warning")
                            .child(Button::new("b-warning").label("Warning").warning())
                            .on_hover(self.hover_info(&fi,
                                    "Button (Warning)",
                                    &[
                                        ("bg", "warning", t.warning),
                                        ("text", "warning_foreground", t.warning_foreground),
                                        ("hover", "warning_hover", t.warning_hover),
                                        ("active", "warning_active", t.warning_active),
                                    ],
                                    &[
                                        ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                        ("shadow", format!("{}", t.shadow)),
                                    ],
                                    &[
                                        ("padding", "set per Size enum"),
                                        ("font-weight", "hardcoded"),
                                        ("min-height", "hardcoded"),
                                    ],
                                        )),
                    )
                    .child(
                        div()
                            .id("tt-btn-info")
                            .child(Button::new("b-info").label("Info").info())
                            .on_hover(self.hover_info(&fi,
                                    "Button (Info)",
                                    &[
                                        ("bg", "info", t.info),
                                        ("text", "info_foreground", t.info_foreground),
                                        ("hover", "info_hover", t.info_hover),
                                        ("active", "info_active", t.info_active),
                                    ],
                                    &[
                                        ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                        ("shadow", format!("{}", t.shadow)),
                                    ],
                                    &[
                                        ("padding", "set per Size enum"),
                                        ("font-weight", "hardcoded"),
                                        ("min-height", "hardcoded"),
                                    ],
                                        )),
                    )
                    .child(
                        div()
                            .id("tt-btn-ghost")
                            .child(Button::new("b-ghost").label("Ghost").ghost())
                            .on_hover(self.hover_info(&fi,
                                    "Button (Ghost)",
                                    &[
                                        ("text", "foreground", t.foreground),
                                        ("hover-text", "muted_foreground", t.muted_foreground),
                                    ],
                                    &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                    &[("padding", "set per Size enum"), ("font-weight", "hardcoded")],
                                        )),
                    )
                    .child(
                        div()
                            .id("tt-btn-link")
                            .child(Button::new("b-link").label("Link").link())
                            .on_hover(self.hover_info(&fi,
                                    "Button (Link)",
                                    &[
                                        ("text", "foreground", t.foreground),
                                        ("hover-text", "muted_foreground", t.muted_foreground),
                                    ],
                                    &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                    &[("padding", "set per Size enum"), ("font-weight", "hardcoded")],
                                        )),
                    )
                    .child(
                        div()
                            .id("tt-btn-text")
                            .child(Button::new("b-text").label("Text").text())
                            .on_hover(self.hover_info(&fi,
                                    "Button (Text)",
                                    &[
                                        ("text", "foreground", t.foreground),
                                        ("hover-text", "muted_foreground", t.muted_foreground),
                                    ],
                                    &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                    &[("padding", "set per Size enum"), ("font-weight", "hardcoded")],
                                        )),
                    )
                    .child(
                        div()
                            .id("tt-btn-outline")
                            .child(Button::new("b-outline").label("Outline").primary().outline())
                            .on_hover(self.hover_info(&fi,
                                    "Button (Primary Outline)",
                                    &[
                                        ("border", "primary", t.primary),
                                        ("text", "primary", t.primary),
                                        ("hover bg", "primary_hover", t.primary_hover),
                                    ],
                                    &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                    &[("padding", "set per Size enum"), ("font-weight", "hardcoded")],
                                        )),
                    ),
            )
            // Button sizes (using secondary for readability)
            .child(section("Button Sizes (Secondary for readability)"))
            .child(
                div()
                    .id("tt-btn-sizes")
                    .child(
                        h_flex()
                            .gap_2()
                            .items_end()
                            .child(Button::new("s-xs").label("XSmall").with_size(Size::XSmall))
                            .child(Button::new("s-sm").label("Small").with_size(Size::Small))
                            .child(Button::new("s-md").label("Medium").with_size(Size::Medium))
                            .child(Button::new("s-lg").label("Large").with_size(Size::Large)),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Button Sizes",
                            &[
                                ("bg", "secondary", t.secondary),
                                ("text", "secondary_foreground", t.secondary_foreground),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[
                                ("size", "XSmall/Small/Medium/Large via Size enum"),
                                ("padding", "varies per Size"),
                                ("min-height", "varies per Size"),
                            ],
                                )),
            )
            // Button group
            .child(section("ButtonGroup"))
            .child(
                div()
                    .id("tt-btn-group")
                    .child(
                        ButtonGroup::new("bg-1")
                            .child(Button::new("bg-a").label("Left"))
                            .child(Button::new("bg-b").label("Center"))
                            .child(Button::new("bg-c").label("Right")),
                    )
                    .on_hover(self.hover_info(&fi,
                            "ButtonGroup",
                            &[
                                ("bg", "secondary", t.secondary),
                                ("text", "secondary_foreground", t.secondary_foreground),
                                ("border", "border", t.border),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[("gap", "hardcoded (0px, merged borders)")],
                                )),
            )
            // Disabled + loading
            .child(section("Disabled State"))
            .child(
                div()
                    .id("tt-btn-disabled")
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Button::new("d-pri").label("Disabled Primary").primary().disabled(true))
                            .child(Button::new("d-sec").label("Disabled Secondary").disabled(true))
                            .child(Button::new("d-dng").label("Disabled Danger").danger().disabled(true)),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Disabled Buttons",
                            &[],
                            &[],
                            &[
                                ("opacity", "hardcoded 0.5 when disabled"),
                                ("cursor", "not-allowed"),
                                ("theme", "same variant colors at reduced opacity"),
                            ],
                                )),
            )
            .child(section("Loading State"))
            .child(
                div()
                    .id("tt-btn-loading")
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Button::new("l-pri").label("Loading...").primary().loading(true)),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Loading Button",
                            &[],
                            &[],
                            &[
                                ("spinner", "replaces icon when loading"),
                                ("interaction", "disabled while loading"),
                            ],
                                )),
            )
            // Buttons with icons
            .child(section("Buttons with Icons"))
            .child(
                div()
                    .id("tt-btn-icons")
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Button::new("bi-save").label("Save").primary().icon(IconName::Check))
                            .child(Button::new("bi-search").label("Search").icon(IconName::Search))
                            .child(Button::new("bi-del").label("Delete").danger().icon(IconName::Delete)),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Buttons with Icons",
                            &[],
                            &[],
                            &[
                                ("icon color", "inherits button text color"),
                                ("icon position", "leading (before label)"),
                                ("icon size", "matches button Size enum"),
                            ],
                                )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Inputs
    // -----------------------------------------------------------------------
    fn render_inputs_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_fonts);
        let t = cx.theme().clone();
        let checkbox_a = self.checkbox_a;
        let checkbox_b = self.checkbox_b;
        let checkbox_c = self.checkbox_c;
        let switch_on = self.switch_on;
        let radio_index = self.radio_index;
        let slider_value = self.slider_value;

        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Text Input
            .child(section("Text Input"))
            .child(
                div()
                    .id("tt-input")
                    .child(
                        Input::new(&self.input_state)
                            .with_size(Size::Medium)
                            .w(px(360.0)),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Input",
                            &[
                                ("border", "input", t.input),
                                ("bg", "background", t.background),
                                ("text", "foreground", t.foreground),
                                ("placeholder", "muted_foreground", t.muted_foreground),
                                ("disabled bg", "muted", t.muted),
                            ],
                            &[
                                ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                ("shadow", format!("{}", t.shadow)),
                            ],
                            &[("padding", "set per Size enum"), ("height", "set per Size enum")],
                                )),
            )
            // Number Input
            .child(section("Number Input"))
            .child(
                div()
                    .id("tt-number-input")
                    .child(
                        NumberInput::new(&self.number_input_state)
                            .placeholder("Enter a number")
                            .with_size(Size::Medium)
                            .w(px(200.0)),
                    )
                    .on_hover(self.hover_info(&fi,
                            "NumberInput",
                            &[
                                ("border", "input", t.input),
                                ("bg", "background", t.background),
                                ("text", "foreground", t.foreground),
                                ("placeholder", "muted_foreground", t.muted_foreground),
                                ("disabled bg", "muted", t.muted),
                            ],
                            &[
                                ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                ("shadow", format!("{}", t.shadow)),
                            ],
                            &[
                                ("padding", "set per Size enum"),
                                ("height", "set per Size enum"),
                                ("step buttons", "hardcoded +/- icons"),
                            ],
                                )),
            )
            // Checkboxes
            .child(section("Checkboxes"))
            .child(
                div()
                    .id("tt-checkbox")
                    .child(
                        v_flex()
                            .gap_3()
                            .child(
                                Checkbox::new("cb-a")
                                    .label("Enable notifications")
                                    .checked(checkbox_a)
                                    .on_click(cx.listener(|this, val: &bool, _w, _cx| {
                                        this.checkbox_a = *val;
                                    })),
                            )
                            .child(
                                Checkbox::new("cb-b")
                                    .label("Auto-save drafts")
                                    .checked(checkbox_b)
                                    .on_click(cx.listener(|this, val: &bool, _w, _cx| {
                                        this.checkbox_b = *val;
                                    })),
                            )
                            .child(
                                Checkbox::new("cb-c")
                                    .label("Disabled checkbox")
                                    .checked(checkbox_c)
                                    .disabled(true),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Checkbox",
                            &[
                                ("checked bg", "primary", t.primary),
                                ("checkmark", "primary_foreground", t.primary_foreground),
                                ("unchecked border", "input", t.input),
                                ("bg", "background", t.background),
                            ],
                            &[
                                ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                ("shadow", format!("{}", t.shadow)),
                            ],
                            &[("size", "set per Size enum"), ("indicator size", "hardcoded")],
                                )),
            )
            // Radio group
            .child(section("Radio Group"))
            .child(
                div()
                    .id("tt-radio")
                    .child(
                        RadioGroup::horizontal("rg-1")
                            .child("Option A")
                            .child("Option B")
                            .child("Option C")
                            .selected_index(radio_index)
                            .on_click(cx.listener(|this, ix: &usize, _w, _cx| {
                                this.radio_index = Some(*ix);
                            })),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Radio",
                            &[
                                ("selected", "primary", t.primary),
                                ("unselected border", "input", t.input),
                                ("bg", "background", t.background),
                            ],
                            &[
                                ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                ("shadow", format!("{}", t.shadow)),
                            ],
                            &[("size", "hardcoded"), ("indicator size", "hardcoded")],
                                )),
            )
            // Switch
            .child(section("Switch"))
            .child(
                div()
                    .id("tt-switch")
                    .child(
                        h_flex()
                            .gap_6()
                            .child(
                                Switch::new("sw-feature")
                                    .label("Feature toggle")
                                    .checked(switch_on)
                                    .on_click(cx.listener(|this, val: &bool, _w, _cx| {
                                        this.switch_on = *val;
                                    })),
                            )
                            .child(
                                Switch::new("sw-disabled")
                                    .label("Disabled")
                                    .checked(true)
                                    .disabled(true),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Switch",
                            &[
                                ("on track", "primary", t.primary),
                                ("off track", "switch", t.switch),
                                ("thumb", "switch_thumb", t.switch_thumb),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[("size", "hardcoded"), ("animation timing", "hardcoded")],
                                )),
            )
            // Slider
            .child(section(format!("Slider (value: {:.0})", slider_value)))
            .child(
                div()
                    .id("tt-slider")
                    .child(Slider::new(&self.slider_state).w(px(360.0)))
                    .on_hover(self.hover_info(&fi,
                            "Slider",
                            &[
                                ("track", "slider_bar", t.slider_bar),
                                ("thumb", "slider_thumb", t.slider_thumb),
                                ("text", "foreground", t.foreground),
                            ],
                            &[("shadow", format!("{}", t.shadow))],
                            &[("track height", "hardcoded"), ("thumb size", "hardcoded")],
                                )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Data
    // -----------------------------------------------------------------------
    fn render_data_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_fonts);
        let t = cx.theme().clone();
        let collapsible_open = self.collapsible_open;

        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Description list
            .child(section("DescriptionList"))
            .child(
                div()
                    .id("tt-desclist")
                    .child(
                        DescriptionList::new()
                            .columns(2)
                            .item("Name", "native-theme", 1)
                            .item("Version", "0.1.0", 1)
                            .item("License", "MIT OR Apache-2.0 OR BSD-0", 1)
                            .item("Platforms", "Linux, macOS, Windows", 1)
                            .item("Description", "Universal theme abstraction layer", 2),
                    )
                    .on_hover(self.hover_info(&fi,
                            "DescriptionList",
                            &[
                                ("label bg", "description_list_label", t.description_list_label),
                                (
                                    "label text",
                                    "description_list_label_foreground",
                                    t.description_list_label_foreground,
                                ),
                                ("border", "border", t.border),
                            ],
                            &[],
                            &[("layout spacing", "hardcoded")],
                                )),
            )
            // Accordion
            .child(section("Accordion"))
            .child(
                div()
                    .id("tt-accordion")
                    .child(
                        Accordion::new("acc-1")
                            .item(|item| {
                                item.title("What is native-theme?")
                                    .open(true)
                                    .child(
                                        Label::new(
                                            "A cross-platform theme abstraction that reads OS \
                                             settings and maps them to toolkit-specific themes.",
                                        )
                                        .text_sm(),
                                    )
                            })
                            .item(|item| {
                                item.title("Supported toolkits").child(
                                    Label::new("gpui-component, iced, egui, and more planned.")
                                        .text_sm(),
                                )
                            })
                            .item(|item| {
                                item.title("How many presets?").child(
                                    Label::new("17 built-in theme presets covering major OS styles.")
                                        .text_sm(),
                                )
                            }),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Accordion",
                            &[
                                ("bg", "accordion", t.accordion),
                                ("hover", "accordion_hover", t.accordion_hover),
                                ("border", "border", t.border),
                                ("text", "foreground", t.foreground),
                                ("secondary text", "muted_foreground", t.muted_foreground),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[("padding", "hardcoded"), ("animation", "hardcoded")],
                                )),
            )
            // Collapsible
            .child(section("Collapsible"))
            .child(
                div()
                    .id("tt-collapsible")
                    .child(
                        Collapsible::new()
                            .open(collapsible_open)
                            .child(
                                Button::new("coll-toggle")
                                    .label(if collapsible_open {
                                        "Click to collapse"
                                    } else {
                                        "Click to expand"
                                    })
                                    .ghost()
                                    .icon(if collapsible_open {
                                        IconName::ChevronDown
                                    } else {
                                        IconName::ChevronRight
                                    })
                                    .on_click(cx.listener(|this, _ev, _w, _cx| {
                                        this.collapsible_open = !this.collapsible_open;
                                    })),
                            )
                            .content(
                                v_flex()
                                    .p_3()
                                    .child(Label::new("This content is shown when collapsible is open.").text_sm()),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Collapsible",
                            &[
                                ("bg", "accordion", t.accordion),
                                ("hover", "accordion_hover", t.accordion_hover),
                                ("border", "border", t.border),
                            ],
                            &[],
                            &[("animation", "hardcoded slide")],
                                )),
            )
            // GroupBox variants
            .child(section("GroupBox (3 variants)"))
            .child(
                div()
                    .id("tt-groupbox")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                GroupBox::new()
                                    .title("Default")
                                    .w(px(180.0))
                                    .child(Label::new("Default style").text_sm()),
                            )
                            .child(
                                GroupBox::new()
                                    .title("Filled")
                                    .fill()
                                    .w(px(180.0))
                                    .child(Label::new("Filled background").text_sm()),
                            )
                            .child(
                                GroupBox::new()
                                    .title("Outline")
                                    .outline()
                                    .w(px(180.0))
                                    .child(Label::new("Outlined border").text_sm()),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "GroupBox",
                            &[
                                ("fill bg", "group_box", t.group_box),
                                ("text", "group_box_foreground", t.group_box_foreground),
                                ("border", "border", t.border),
                                ("title", "muted_foreground", t.muted_foreground),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[("padding", "hardcoded")],
                                )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Feedback
    // -----------------------------------------------------------------------
    fn render_feedback_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_fonts);
        let t = cx.theme().clone();
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Alerts with icons
            .child(section("Alerts (all 4 variants with icons)"))
            .child(
                div()
                    .id("tt-alert-info")
                    .child(
                        Alert::info("alert-info", "This is an informational message.")
                            .title("Info"),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Alert (Info)",
                            &[
                                ("color", "info", t.info),
                                ("text", "info", t.info),
                                ("border", "info", t.info),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[
                                ("padding", "hardcoded per Size"),
                                ("icon", "Info (hardcoded for variant)"),
                                ("icon size", "hardcoded"),
                            ],
                                )),
            )
            .child(
                div()
                    .id("tt-alert-success")
                    .child(
                        Alert::success("alert-ok", "Operation completed successfully.")
                            .title("Success"),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Alert (Success)",
                            &[
                                ("color", "success", t.success),
                                ("text", "success", t.success),
                                ("border", "success", t.success),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[
                                ("padding", "hardcoded per Size"),
                                ("icon", "CircleCheck (hardcoded)"),
                            ],
                                )),
            )
            .child(
                div()
                    .id("tt-alert-warning")
                    .child(
                        Alert::warning("alert-warn", "Please review before proceeding.")
                            .title("Warning"),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Alert (Warning)",
                            &[
                                ("color", "warning", t.warning),
                                ("text", "warning", t.warning),
                                ("border", "warning", t.warning),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[
                                ("padding", "hardcoded per Size"),
                                ("icon", "TriangleAlert (hardcoded)"),
                            ],
                                )),
            )
            .child(
                div()
                    .id("tt-alert-error")
                    .child(
                        Alert::error("alert-err", "Something went wrong. Please try again.")
                            .title("Error"),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Alert (Error)",
                            &[
                                ("color", "danger", t.danger),
                                ("text", "danger", t.danger),
                                ("border", "danger", t.danger),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[
                                ("padding", "hardcoded per Size"),
                                ("icon", "CircleX (hardcoded)"),
                            ],
                                )),
            )
            // Progress
            .child(section("Progress Bars"))
            .child(
                div()
                    .id("tt-progress")
                    .child(
                        v_flex()
                            .gap_3()
                            .w(px(360.0))
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Upload").text_sm())
                                    .child(Label::new("73%").text_sm()),
                            )
                            .child(Progress::new().value(73.0))
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Processing").text_sm())
                                    .child(Label::new("45%").text_sm()),
                            )
                            .child(Progress::new().value(45.0))
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Complete").text_sm())
                                    .child(Label::new("100%").text_sm()),
                            )
                            .child(Progress::new().value(100.0)),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Progress",
                            &[("bar", "progress_bar", t.progress_bar)],
                            &[],
                            &[("height", "hardcoded"), ("animation", "hardcoded")],
                                )),
            )
            // Spinners
            .child(section("Spinner (3 sizes)"))
            .child(
                div()
                    .id("tt-spinner")
                    .child(
                        h_flex()
                            .gap_6()
                            .items_center()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Spinner::new().with_size(Size::Small))
                                    .child(Label::new("Small").text_sm()),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Spinner::new().with_size(Size::Medium))
                                    .child(Label::new("Medium").text_sm()),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Spinner::new().with_size(Size::Large))
                                    .child(Label::new("Large").text_sm()),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Spinner",
                            &[],
                            &[],
                            &[("animation speed", "hardcoded"), ("size", "hardcoded per Size enum")],
                                )),
            )
            // Skeleton
            .child(section("Skeleton Placeholders"))
            .child(
                div()
                    .id("tt-skeleton")
                    .child(
                        v_flex()
                            .gap_2()
                            .w(px(360.0))
                            .child(Skeleton::new().h(px(12.0)).w(px(200.0)).rounded(px(4.0)))
                            .child(Skeleton::new().h(px(8.0)).w(px(300.0)).rounded(px(4.0)))
                            .child(Skeleton::new().h(px(8.0)).w(px(250.0)).rounded(px(4.0)))
                            .child(Skeleton::new().secondary().h(px(60.0)).rounded(px(6.0))),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Skeleton",
                            &[("bg", "skeleton", t.skeleton)],
                            &[],
                            &[("animation", "hardcoded pulse")],
                                )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Typography
    // -----------------------------------------------------------------------
    fn render_typography_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_fonts);
        let t = cx.theme().clone();
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Labels
            .child(section("Label"))
            .child(
                div()
                    .id("tt-label")
                    .child(
                        v_flex()
                            .gap_2()
                            .child(Label::new("Regular label"))
                            .child(Label::new("Label with secondary").secondary("(secondary text)"))
                            .child(Label::new("Masked label: secret123").masked(true)),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Label",
                            &[
                                ("text", "foreground", t.foreground),
                                ("secondary", "muted_foreground", t.muted_foreground),
                                ("highlights", "blue", t.blue),
                            ],
                            &[
                                ("font", format!("font_family: {}", t.font_family)),
                                ("size", format!("font_size: {}px (renders)", t.font_size.as_f32())),
                            ],
                            &[("font weights", "hardcoded")],
                                )),
            )
            // Tags
            .child(section("Tags (7 colors + outline)"))
            .child(
                div()
                    .id("tt-tags")
                    .child(
                        h_flex()
                            .gap_2()
                            .flex_wrap()
                            .child(Tag::primary().child("Primary"))
                            .child(Tag::secondary().child("Secondary"))
                            .child(Tag::danger().child("Danger"))
                            .child(Tag::success().child("Success"))
                            .child(Tag::warning().child("Warning"))
                            .child(Tag::info().child("Info"))
                            .child(Tag::primary().outline().child("Primary Outline"))
                            .child(Tag::danger().outline().child("Danger Outline")),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Tag (per variant)",
                            &[
                                ("bg (primary)", "primary", t.primary),
                                ("text (primary)", "primary_foreground", t.primary_foreground),
                                ("border (outline)", "border", t.border),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[("padding", "hardcoded per Size")],
                                )),
            )
            // Badges
            .child(section("Badge"))
            .child(
                div()
                    .id("tt-badge")
                    .child(
                        h_flex()
                            .gap_8()
                            .child(Badge::new().count(5).child(Button::new("badge-1").label("Messages")))
                            .child(
                                Badge::new()
                                    .count(99)
                                    .child(Button::new("badge-2").label("Notifications")),
                            )
                            .child(Badge::new().dot().child(Button::new("badge-3").label("Updates"))),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Badge",
                            &[
                                ("bg", "red", t.red),
                                ("text", "background", t.background),
                            ],
                            &[],
                            &[("size", "hardcoded"), ("padding", "hardcoded")],
                                )),
            )
            // Link
            .child(section("Link"))
            .child(
                div()
                    .id("tt-link")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                Link::new("link-1")
                                    .child("Visit Documentation")
                                    .href("https://github.com"),
                            )
                            .child(
                                Link::new("link-2")
                                    .child("Another Link")
                                    .href("https://gpui.rs"),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Link",
                            &[("text+decoration", "link", t.link)],
                            &[],
                            &[
                                ("underline style", "hardcoded"),
                                ("hover opacity", "0.8"),
                                ("active opacity", "0.6"),
                            ],
                                )),
            )
            // Breadcrumb (with tab navigation)
            .child(section("Breadcrumb (click to navigate tabs)"))
            .child(
                div()
                    .id("tt-breadcrumb")
                    .child(
                        Breadcrumb::new()
                            .child(
                                BreadcrumbItem::new("Buttons")
                                    .on_click(cx.listener(|this, _ev, _w, _cx| {
                                        this.active_tab = TAB_BUTTONS;
                                    })),
                            )
                            .child(
                                BreadcrumbItem::new("Inputs")
                                    .on_click(cx.listener(|this, _ev, _w, _cx| {
                                        this.active_tab = TAB_INPUTS;
                                    })),
                            )
                            .child(
                                BreadcrumbItem::new("Data")
                                    .on_click(cx.listener(|this, _ev, _w, _cx| {
                                        this.active_tab = TAB_DATA;
                                    })),
                            )
                            .child(
                                BreadcrumbItem::new("Feedback")
                                    .on_click(cx.listener(|this, _ev, _w, _cx| {
                                        this.active_tab = TAB_FEEDBACK;
                                    })),
                            )
                            .child(BreadcrumbItem::new("Typography")),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Breadcrumb",
                            &[
                                ("last item", "foreground", t.foreground),
                                ("non-last + separators", "muted_foreground", t.muted_foreground),
                            ],
                            &[],
                            &[("separator icon", "hardcoded ChevronRight"), ("spacing", "hardcoded")],
                                )),
            )
            // Divider styles
            .child(section("Divider Styles"))
            .child(
                div()
                    .id("tt-divider")
                    .child(
                        v_flex()
                            .gap_3()
                            .child(Divider::horizontal())
                            .child(Divider::horizontal().label("OR"))
                            .child(Divider::horizontal_dashed())
                            .child(Divider::horizontal_dashed().label("END")),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Divider",
                            &[
                                ("line", "border", t.border),
                                ("label bg", "background", t.background),
                                ("label text", "muted_foreground", t.muted_foreground),
                            ],
                            &[],
                            &[("thickness", "1px hardcoded")],
                                )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Layout
    // -----------------------------------------------------------------------
    fn render_layout_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_fonts);
        let t = cx.theme().clone();
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Horizontal resizable
            .child(section("Resizable Panels (horizontal)"))
            .child(
                div()
                    .id("tt-resizable-h")
                    .h(px(160.0))
                    .border_1()
                    .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
                    .child(
                        h_resizable("resize-h")
                            .child(
                                resizable_panel()
                                    .size(px(250.0))
                                    .child(
                                        v_flex()
                                            .p_3()
                                            .size_full()
                                            .child(Label::new("Left Panel").font_semibold())
                                            .child(Label::new("Drag the divider to resize").text_sm()),
                                    ),
                            )
                            .child(
                                resizable_panel().child(
                                    v_flex()
                                        .p_3()
                                        .size_full()
                                        .child(Label::new("Right Panel").font_semibold())
                                        .child(Label::new("This panel fills remaining space").text_sm()),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Resizable",
                            &[
                                ("dragging border", "drag_border", t.drag_border),
                                ("idle border", "border", t.border),
                            ],
                            &[],
                            &[("min panel size", "100px hardcoded")],
                                )),
            )
            // Vertical resizable
            .child(section("Resizable Panels (vertical)"))
            .child(
                div()
                    .id("tt-resizable-v")
                    .h(px(200.0))
                    .border_1()
                    .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
                    .child(
                        v_resizable("resize-v")
                            .child(
                                resizable_panel()
                                    .size(px(80.0))
                                    .child(
                                        v_flex()
                                            .p_3()
                                            .size_full()
                                            .child(Label::new("Top Panel").font_semibold()),
                                    ),
                            )
                            .child(
                                resizable_panel().child(
                                    v_flex()
                                        .p_3()
                                        .size_full()
                                        .child(Label::new("Bottom Panel").font_semibold()),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Resizable (vertical)",
                            &[
                                ("dragging border", "drag_border", t.drag_border),
                                ("idle border", "border", t.border),
                            ],
                            &[],
                            &[("min panel size", "100px hardcoded")],
                                )),
            )
            // Dividers
            .child(section("Divider (solid / dashed / labeled)"))
            .child(
                div()
                    .id("tt-layout-divider")
                    .child(
                        v_flex()
                            .gap_3()
                            .child(Divider::horizontal())
                            .child(Divider::horizontal().label("Section Break"))
                            .child(Divider::horizontal_dashed()),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Divider",
                            &[
                                ("line", "border", t.border),
                                ("label bg", "background", t.background),
                                ("label text", "muted_foreground", t.muted_foreground),
                            ],
                            &[],
                            &[("thickness", "1px hardcoded")],
                                )),
            )
            // GroupBox as container
            .child(section("GroupBox as Layout Container"))
            .child(
                div()
                    .id("tt-layout-groupbox")
                    .child(
                        GroupBox::new()
                            .title("Contained Content")
                            .fill()
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(Label::new("GroupBox can wrap any content as a visual container.").text_sm())
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Button::new("gb-1").label("Action A"))
                                            .child(Button::new("gb-2").label("Action B").primary()),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "GroupBox (layout)",
                            &[
                                ("fill bg", "group_box", t.group_box),
                                ("text", "group_box_foreground", t.group_box_foreground),
                                ("border", "border", t.border),
                                ("title", "muted_foreground", t.muted_foreground),
                            ],
                            &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                            &[("padding", "hardcoded")],
                                )),
            )
            // Scrollable area demo
            .child(section("Scrollable Area (visible scrollbar)"))
            .child(
                div()
                    .id("tt-scrollbar")
                    .child(
                        div()
                            .id("scroll-demo-outer")
                            .h(px(150.0))
                            .w_full()
                            .border_1()
                            .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
                            .overflow_y_scrollbar()
                            .child(
                                v_flex().gap_2().p_3().children(
                                    (0..20).map(|i| {
                                        Label::new(SharedString::from(format!(
                                            "Scrollable item #{} - demonstrates scrollbar theming",
                                            i + 1
                                        )))
                                        .text_sm()
                                    }),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Scrollbar",
                            &[
                                ("track", "scrollbar", t.scrollbar),
                                ("thumb", "scrollbar_thumb", t.scrollbar_thumb),
                                ("thumb hover", "scrollbar_thumb_hover", t.scrollbar_thumb_hover),
                                ("border", "border", t.border),
                            ],
                            &[
                                ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                                ("show mode", format!("scrollbar_show: {:?}", t.scrollbar_show)),
                            ],
                            &[("width", "16px hardcoded"), ("min thumb length", "48px hardcoded")],
                                )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Icons
    // -----------------------------------------------------------------------
    fn render_icons_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_fonts);

        // --- Native Theme Icons section ---
        let fallback_label = if !is_native_icon_set(&self.icon_set_name) {
            " (fallback)"
        } else {
            ""
        };
        let loaded_count = self.loaded_icons.iter().filter(|(_, d, _)| d.is_some()).count();
        let system_count = self.loaded_icons.iter().filter(|(_, _, s)| *s == IconSource::System).count();
        let fallback_count = self.loaded_icons.iter().filter(|(_, _, s)| *s == IconSource::Fallback).count();
        let is_system_set = matches!(self.icon_set_name.as_str(), "freedesktop" | "sf-symbols" | "segoe-fluent");
        let native_section_title = if is_system_set {
            let detected_theme = system_icon_theme();
            format!(
                "Native Theme Icons: {} [{}/{} loaded, {} system, {} fallback]{}",
                detected_theme,
                loaded_count,
                self.loaded_icons.len(),
                system_count,
                fallback_count,
                fallback_label,
            )
        } else {
            format!(
                "Native Theme Icons: {} [{}/{} loaded]{}",
                self.icon_set_name,
                loaded_count,
                self.loaded_icons.len(),
                fallback_label,
            )
        };

        // Resolve the IconSet for tooltip icon-name lookups
        let icon_set_enum = match self.icon_set_name.as_str() {
            "freedesktop" => Some(IconSet::Freedesktop),
            "material" => Some(IconSet::Material),
            "lucide" => Some(IconSet::Lucide),
            "sf-symbols" => Some(IconSet::SfSymbols),
            "segoe-fluent" => Some(IconSet::SegoeIcons),
            _ => None,
        };
        let icon_set_label = self.icon_set_name.clone();

        // Build icon cells for loaded native icons
        let native_icon_cells: Vec<_> = self
            .loaded_icons
            .iter()
            .enumerate()
            .map(|(i, (role, _data, source))| {
                let role_name: SharedString = format!("{:?}", role).into();
                let cell_id = SharedString::from(format!("native-icon-{}-{}", self.icon_set_name, i));

                let is_gpui_builtin = self.icon_set_name == "gpui-builtin";
                let icon_element = if is_gpui_builtin {
                    if let Some(icon_name) = native_theme_gpui::icons::icon_name(*role) {
                        div().child(Icon::new(icon_name).with_size(Size::Medium))
                    } else {
                        div()
                            .w(px(20.0))
                            .h(px(20.0))
                            .bg(gpui::hsla(0.0, 0.0, 0.5, 0.2))
                            .rounded(px(2.0))
                    }
                } else if let Some(img_source) = self.loaded_icon_sources.get(i).and_then(|s| s.clone()) {
                    div().child(
                        gpui::img(img_source)
                            .w(px(20.0))
                            .h(px(20.0)),
                    )
                } else {
                    // No icon data -- gray placeholder
                    div()
                        .w(px(20.0))
                        .h(px(20.0))
                        .bg(gpui::hsla(0.0, 0.0, 0.5, 0.2))
                        .rounded(px(2.0))
                };

                // Build tooltip text with origin info
                let tooltip_role = format!("{:?}", role);
                let tooltip_set = icon_set_label.clone();
                let tooltip_icon_name = icon_set_enum
                    .and_then(|set| native_icon_name(set, *role))
                    .unwrap_or("(unmapped)");
                let tooltip_icon_name = tooltip_icon_name.to_string();
                let source = *source;

                div()
                    .id(cell_id)
                    .flex()
                    .flex_col()
                    .items_center()
                    .py_2()
                    .px_2()
                    .gap_1()
                    .child(icon_element)
                    .child(Label::new(role_name).text_xs())
                    .on_hover(self.set_info({
                        let mut lines = format!(
                            "Role: {}\nIcon set: {}\nIcon name: {}",
                            tooltip_role, tooltip_set, tooltip_icon_name,
                        );
                        match source {
                            IconSource::System => {
                                lines.push_str("\nOrigin: OS theme");
                            }
                            IconSource::Fallback => {
                                lines.push_str(
                                    "\nOrigin: Bundled Material fallback.\n\
                                     The OS icon theme did not provide this icon,\n\
                                     so the bundled Material SVG is used instead.",
                                );
                            }
                            IconSource::Bundled => {
                                lines.push_str("\nOrigin: Bundled with native-theme");
                            }
                            IconSource::NotFound => {
                                lines.push_str(
                                    "\nOrigin: Not found.\n\
                                     No icon is available for this role in this set\n\
                                     and no bundled fallback is configured.",
                                );
                            }
                        }
                        lines
                    }))
            })
            .collect();

        // --- gpui-component IconName gallery ---
        let gpui_icon_set_label = self.icon_set_name.clone();
        let gpui_icon_cells: Vec<_> = self
            .gpui_icons
            .iter()
            .enumerate()
            .map(|(i, (name, icon, role, _data, source))| {
                let name_s: SharedString = (*name).into();
                let cell_id = SharedString::from(format!("gpui-icon-{}", i));

                // Render from cached image sources. Bundled sets (material, lucide)
                // cover all 86 icons via by-name lookup — no mixing of sets.
                let is_gpui_builtin = self.icon_set_name == "gpui-builtin";
                let icon_element = if is_gpui_builtin {
                    div().child(Icon::new(icon.clone()).with_size(Size::Medium))
                } else if let Some(img_source) = self.gpui_icon_sources.get(i).and_then(|s| s.clone()) {
                    div().child(
                        gpui::img(img_source)
                            .w(px(20.0))
                            .h(px(20.0)),
                    )
                } else {
                    // Gray placeholder — no fallback to a different icon set
                    div()
                        .w(px(20.0))
                        .h(px(20.0))
                        .bg(gpui::hsla(0.0, 0.0, 0.5, 0.2))
                        .rounded(px(2.0))
                };

                let tooltip_name = name.to_string();
                let tooltip_set = gpui_icon_set_label.clone();
                let tooltip_role = role.map(|r| format!("{:?}", r));
                let source = *source;

                div()
                    .id(cell_id)
                    .flex()
                    .flex_col()
                    .items_center()
                    .py_2()
                    .px_2()
                    .gap_1()
                    .child(icon_element)
                    .child(Label::new(name_s).text_xs())
                    .on_hover(self.set_info({
                        let mut lines = format!("Icon: {}", tooltip_name);
                        if let Some(ref role_str) = tooltip_role {
                            lines.push_str(&format!(
                                "\nMapped to IconRole: {}\nIcon set: {}",
                                role_str, tooltip_set,
                            ));
                        } else {
                            lines.push_str(
                                "\nNo native-theme IconRole mapping.\n\
                                 Loaded via by-name lookup from bundled icon set.",
                            );
                        }
                        match source {
                            IconSource::System => {
                                lines.push_str("\nOrigin: OS theme");
                            }
                            IconSource::Fallback => {
                                lines.push_str(
                                    "\nOrigin: Bundled Material fallback.\n\
                                     The OS icon theme did not provide this icon,\n\
                                     so the bundled Material SVG is used instead.",
                                );
                            }
                            IconSource::Bundled => {
                                lines.push_str(&format!(
                                    "\nOrigin: Bundled {} SVG",
                                    tooltip_set,
                                ));
                            }
                            IconSource::NotFound => {
                                lines.push_str(
                                    "\nOrigin: Not found in selected set.\n\
                                     No icon available for this variant in the selected set.",
                                );
                            }
                        }
                        lines
                    }))
            })
            .collect();

        let mapped_count = self.gpui_icons.iter().filter(|(_, _, r, _, _)| r.is_some()).count();

        v_flex()
            .gap_3()
            .p_4()
            // Native Theme Icons section
            .child(section(native_section_title))
            .child(
                div()
                    .id("native-icons-grid")
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .children(native_icon_cells),
                    ),
            )
            .child(Divider::horizontal())
            // gpui-component IconName gallery
            .child(section(format!(
                "gpui-component Icons ({} variants, {} mapped to {})",
                self.gpui_icons.len(),
                mapped_count,
                self.icon_set_name,
            )))
            .child(
                div()
                    .id("tt-icons-grid")
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .children(gpui_icon_cells),
                    )
                    .on_hover(self.hover_info(&fi,
                            "Icon",
                            &[],
                            &[],
                            &[
                                ("color", "inherited from parent foreground, customizable via text_color()"),
                                ("SVG shapes", "86 built-in Lucide icons from gpui-component"),
                            ],
                                )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Theme Map
    // -----------------------------------------------------------------------
    fn render_theme_map_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let _fi = format_font_info(&self.original_fonts);
        let t = cx.theme().clone();

        v_flex()
            .gap_4()
            .p_4()
            .flex_1()
            .child(section("All ThemeColor Fields"))
            // Core
            .child(section("Core"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("background", t.background))
                    .child(color_swatch("foreground", t.foreground))
                    .child(color_swatch("accent", t.accent))
                    .child(color_swatch("accent_foreground", t.accent_foreground))
                    .child(color_swatch("border", t.border))
                    .child(color_swatch("muted", t.muted))
                    .child(color_swatch("muted_foreground", t.muted_foreground))
                    .child(color_swatch("input", t.input))
                    .child(color_swatch("ring", t.ring))
                    .child(color_swatch("selection", t.selection))
                    .child(color_swatch("caret", t.caret))
                    .child(color_swatch("link", t.link))
                    .child(color_swatch("link_hover", t.link_hover))
                    .child(color_swatch("link_active", t.link_active))
                    .child(color_swatch("overlay", t.overlay)),
            )
            // Primary
            .child(section("Primary"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("primary", t.primary))
                    .child(color_swatch("primary_foreground", t.primary_foreground))
                    .child(color_swatch("primary_hover", t.primary_hover))
                    .child(color_swatch("primary_active", t.primary_active)),
            )
            // Secondary
            .child(section("Secondary"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("secondary", t.secondary))
                    .child(color_swatch("secondary_foreground", t.secondary_foreground))
                    .child(color_swatch("secondary_hover", t.secondary_hover))
                    .child(color_swatch("secondary_active", t.secondary_active)),
            )
            // Danger
            .child(section("Danger"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("danger", t.danger))
                    .child(color_swatch("danger_foreground", t.danger_foreground))
                    .child(color_swatch("danger_hover", t.danger_hover))
                    .child(color_swatch("danger_active", t.danger_active))
                    .child(color_swatch("red", t.red))
                    .child(color_swatch("red_light", t.red_light)),
            )
            // Success
            .child(section("Success"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("success", t.success))
                    .child(color_swatch("success_foreground", t.success_foreground))
                    .child(color_swatch("success_hover", t.success_hover))
                    .child(color_swatch("success_active", t.success_active))
                    .child(color_swatch("green", t.green))
                    .child(color_swatch("green_light", t.green_light)),
            )
            // Warning
            .child(section("Warning"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("warning", t.warning))
                    .child(color_swatch("warning_foreground", t.warning_foreground))
                    .child(color_swatch("warning_hover", t.warning_hover))
                    .child(color_swatch("warning_active", t.warning_active))
                    .child(color_swatch("yellow", t.yellow))
                    .child(color_swatch("yellow_light", t.yellow_light)),
            )
            // Info
            .child(section("Info"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("info", t.info))
                    .child(color_swatch("info_foreground", t.info_foreground))
                    .child(color_swatch("info_hover", t.info_hover))
                    .child(color_swatch("info_active", t.info_active))
                    .child(color_swatch("blue", t.blue))
                    .child(color_swatch("blue_light", t.blue_light)),
            )
            // List
            .child(section("List"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("list", t.list))
                    .child(color_swatch("list_active", t.list_active))
                    .child(color_swatch("list_active_border", t.list_active_border))
                    .child(color_swatch("list_even", t.list_even))
                    .child(color_swatch("list_head", t.list_head))
                    .child(color_swatch("list_hover", t.list_hover)),
            )
            // Table
            .child(section("Table"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("table", t.table))
                    .child(color_swatch("table_active", t.table_active))
                    .child(color_swatch("table_active_border", t.table_active_border))
                    .child(color_swatch("table_even", t.table_even))
                    .child(color_swatch("table_head", t.table_head))
                    .child(color_swatch("table_head_foreground", t.table_head_foreground))
                    .child(color_swatch("table_hover", t.table_hover))
                    .child(color_swatch("table_row_border", t.table_row_border)),
            )
            // Tab
            .child(section("Tab"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("tab", t.tab))
                    .child(color_swatch("tab_active", t.tab_active))
                    .child(color_swatch("tab_active_foreground", t.tab_active_foreground))
                    .child(color_swatch("tab_bar", t.tab_bar))
                    .child(color_swatch("tab_bar_segmented", t.tab_bar_segmented))
                    .child(color_swatch("tab_foreground", t.tab_foreground)),
            )
            // Sidebar
            .child(section("Sidebar"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("sidebar", t.sidebar))
                    .child(color_swatch("sidebar_foreground", t.sidebar_foreground))
                    .child(color_swatch("sidebar_accent", t.sidebar_accent))
                    .child(color_swatch(
                        "sidebar_accent_foreground",
                        t.sidebar_accent_foreground,
                    ))
                    .child(color_swatch("sidebar_border", t.sidebar_border))
                    .child(color_swatch("sidebar_primary", t.sidebar_primary))
                    .child(color_swatch(
                        "sidebar_primary_foreground",
                        t.sidebar_primary_foreground,
                    )),
            )
            // Scrollbar
            .child(section("Scrollbar"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("scrollbar", t.scrollbar))
                    .child(color_swatch("scrollbar_thumb", t.scrollbar_thumb))
                    .child(color_swatch("scrollbar_thumb_hover", t.scrollbar_thumb_hover)),
            )
            // Accordion
            .child(section("Accordion"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("accordion", t.accordion))
                    .child(color_swatch("accordion_hover", t.accordion_hover)),
            )
            // GroupBox
            .child(section("GroupBox"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("group_box", t.group_box))
                    .child(color_swatch("group_box_foreground", t.group_box_foreground)),
            )
            // Chart
            .child(section("Chart"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("chart_1", t.chart_1))
                    .child(color_swatch("chart_2", t.chart_2))
                    .child(color_swatch("chart_3", t.chart_3))
                    .child(color_swatch("chart_4", t.chart_4))
                    .child(color_swatch("chart_5", t.chart_5))
                    .child(color_swatch("bullish", t.bullish))
                    .child(color_swatch("bearish", t.bearish)),
            )
            // Misc
            .child(section("Misc"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("description_list_label", t.description_list_label))
                    .child(color_swatch(
                        "description_list_label_foreground",
                        t.description_list_label_foreground,
                    ))
                    .child(color_swatch("drag_border", t.drag_border))
                    .child(color_swatch("drop_target", t.drop_target))
                    .child(color_swatch("popover", t.popover))
                    .child(color_swatch("popover_foreground", t.popover_foreground))
                    .child(color_swatch("progress_bar", t.progress_bar))
                    .child(color_swatch("skeleton", t.skeleton))
                    .child(color_swatch("slider_bar", t.slider_bar))
                    .child(color_swatch("slider_thumb", t.slider_thumb))
                    .child(color_swatch("switch", t.switch))
                    .child(color_swatch("switch_thumb", t.switch_thumb))
                    .child(color_swatch("title_bar", t.title_bar))
                    .child(color_swatch("title_bar_border", t.title_bar_border))
                    .child(color_swatch("window_border", t.window_border))
                    .child(color_swatch("tiles", t.tiles)),
            )
            // Base colors
            .child(section("Base"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("magenta", t.magenta))
                    .child(color_swatch("magenta_light", t.magenta_light))
                    .child(color_swatch("cyan", t.cyan))
                    .child(color_swatch("cyan_light", t.cyan_light)),
            )
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for Showcase {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_fonts);
        let theme = cx.theme().clone();

        // Ensure icon image caches match the current foreground color
        if theme.foreground != self.icon_cache_fg {
            self.rebuild_icon_caches(theme.foreground);
        }

        let active_tab = self.active_tab;
        let is_dark = self.is_dark;
        let theme_name_display: SharedString = if self.current_theme_name == "OS Theme" {
            "OS Theme".into()
        } else {
            format!(
                "{} ({})",
                self.current_theme_name,
                if is_dark { "dark" } else { "light" }
            )
            .into()
        };

        // Build the sidebar content
        let sidebar = v_flex()
            .id("sidebar")
            .w(px(220.0))
            .min_w(px(220.0))
            .h_full()
            .bg(theme.sidebar)
            .border_r_1()
            .border_color(theme.sidebar_border)
            .overflow_y_scroll()
            .child(
                v_flex()
                    .p_3()
                    .gap_3()
                    .child(Label::new("Theme Selector").text_size(px(13.0)).font_semibold())
                    .child(
                        Select::new(&self.theme_select)
                            .with_size(Size::Small)
                            .w_full(),
                    )
                    .child(
                        Select::new(&self.dark_mode_select)
                            .with_size(Size::Small)
                            .w_full(),
                    )
                    .child(Divider::horizontal()),
            )
            .child(
                v_flex()
                    .p_3()
                    .gap_3()
                    .child(Label::new("Icon Theme").text_size(px(13.0)).font_semibold())
                    .child(
                        Select::new(&self.icon_set_select)
                            .with_size(Size::Small)
                            .w_full(),
                    )
                    .child(Divider::horizontal()),
            )
            .child(self.render_sidebar(window, cx))
            .child(Divider::horizontal())
            .child(self.widget_info_panel.clone());

        // Build the content area
        let content = v_flex()
            .flex_1()
            .h_full()
            .overflow_hidden()
            // Header
            .child(
                v_flex()
                    .px_4()
                    .pt_3()
                    .pb_2()
                    .gap_2()
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(
                                v_flex()
                                    .child(
                                        Label::new("native-theme-gpui Reference")
                                            .text_size(px(18.0))
                                            .font_semibold(),
                                    )
                                    .child(
                                        Label::new(theme_name_display)
                                            .text_size(px(11.0))
                                            .text_color(theme.muted_foreground),
                                    ),
                            ),
                    )
                    // Tab bar
                    .child(
                        div()
                            .id("tt-tabbar")
                            .child(
                                TabBar::new("nav")
                                    .underline()
                                    .with_size(Size::Small)
                                    .child("Buttons")
                                    .child("Inputs")
                                    .child("Data")
                                    .child("Feedback")
                                    .child("Typography")
                                    .child("Layout")
                                    .child("Icons")
                                    .child("Theme Map")
                                    .selected_index(active_tab)
                                    .on_click(cx.listener(|this, ix: &usize, _window, _cx| {
                                        this.active_tab = *ix;
                                    })),
                            )
                            .on_hover(self.hover_info(&fi,
                                    "TabBar",
                                    &[
                                        ("bg", "tab", theme.tab),
                                        ("active bg", "tab_active", theme.tab_active),
                                        ("active text", "tab_active_foreground", theme.tab_active_foreground),
                                        ("bar bg", "tab_bar", theme.tab_bar),
                                        ("text", "tab_foreground", theme.tab_foreground),
                                        ("border", "border", theme.border),
                                        ("hover", "secondary_hover", theme.secondary_hover),
                                    ],
                                    &[("border-radius", format!("radius: {}px", theme.radius.as_f32()))],
                                    &[("padding", "set per Size enum")],
                                        )),
                    ),
            )
            // Content with scrollbar
            .child(
                div()
                    .id("content-scroll-outer")
                    .flex_1()
                    .overflow_y_scroll()
                    .child(match active_tab {
                        TAB_BUTTONS => self.render_buttons_tab(cx).into_any_element(),
                        TAB_INPUTS => self.render_inputs_tab(cx).into_any_element(),
                        TAB_DATA => self.render_data_tab(cx).into_any_element(),
                        TAB_FEEDBACK => self.render_feedback_tab(cx).into_any_element(),
                        TAB_TYPOGRAPHY => self.render_typography_tab(cx).into_any_element(),
                        TAB_LAYOUT => self.render_layout_tab(cx).into_any_element(),
                        TAB_ICONS => self.render_icons_tab(cx).into_any_element(),
                        TAB_THEME_MAP => self.render_theme_map_tab(cx).into_any_element(),
                        _ => self.render_buttons_tab(cx).into_any_element(),
                    }),
            );

        // Main layout: horizontal split with sidebar + content
        h_flex()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(sidebar)
            .child(content)
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn main() {
    Application::new()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(1100.), px(850.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let showcase = cx.new(|cx| Showcase::new(window, cx));
                cx.new(|cx| Root::new(showcase, window, cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn main() {
    eprintln!("gpui showcase requires macOS or Linux");
}
