//! native-theme-gpui Comprehensive Designer Reference Tool
//!
//! A complete widget gallery with tooltip-based documentation for every
//! theme-controlled property. Demonstrates all gpui-component widgets,
//! all ThemeColor fields, all IconName variants, and full theme switching.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-gpui --example showcase-gpui
//! ```

use gpui::{
    Animation, AnimationExt, AnyElement, App, Application, Bounds, Context, Entity, Hsla,
    ImageSource, IntoElement, Keystroke, Menu, MenuItem, ParentElement, Render, SharedString,
    Styled, Task, Timer, Window, WindowBounds, WindowOptions, div, prelude::*, px, rems, size,
};
use gpui_component::{
    ActiveTheme, Disableable, Icon, IconName, PixelsExt, Placement, Root, Sizable, Size, StyledExt,
    WindowExt,
    accordion::Accordion,
    alert::Alert,
    avatar::{Avatar, AvatarGroup},
    badge::Badge,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonGroup, ButtonVariants, DropdownButton, Toggle, ToggleGroup},
    chart::{AreaChart, BarChart, CandlestickChart, LineChart, PieChart},
    checkbox::Checkbox,
    clipboard::Clipboard,
    collapsible::Collapsible,
    color_picker::{ColorPicker, ColorPickerState},
    description_list::DescriptionList,
    divider::Divider,
    form::{self, Field},
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
    input::{Input, InputState, NumberInput, NumberInputEvent, OtpInput, OtpState, StepAction},
    kbd::Kbd,
    label::Label,
    link::Link,
    list::{ListDelegate, ListItem, ListState},
    menu::{AppMenuBar, ContextMenuExt},
    notification::Notification,
    popover::Popover,
    progress::Progress,
    radio::RadioGroup,
    resizable::{h_resizable, resizable_panel, v_resizable},
    scroll::ScrollableElement,
    select::{SearchableVec, Select, SelectEvent, SelectState},
    setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    sidebar::{Sidebar, SidebarMenu, SidebarMenuItem},
    skeleton::Skeleton,
    slider::{Slider, SliderEvent, SliderState},
    spinner::Spinner,
    switch::Switch,
    tab::TabBar,
    table::{Column, Table, TableDelegate, TableState},
    tag::Tag,
    text::{TextView, TextViewStyle},
    theme::Theme,
    tree::{Tree, TreeItem, TreeState},
    v_flex,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[cfg(target_os = "linux")]
use native_theme::detect::parse_linux_desktop;
use native_theme::detect::{prefers_reduced_motion, system_is_dark};
use native_theme::icons::{
    FreedesktopLoader, IconSetChoice, LucideLoader, MaterialLoader, SegoeIconsLoader,
    SfSymbolsLoader, default_icon_choice, list_freedesktop_themes, load_icon, load_icon_indicator,
};
use native_theme::pipeline::platform_preset_name;
use native_theme::theme::{
    AnimatedIcon, IconData, IconRole, IconSet, TransformAnimation, icon_name as native_icon_name,
    system_icon_set, system_icon_theme,
};
#[cfg(target_os = "linux")]
use native_theme_gpui::icons::freedesktop_name_for_gpui_icon;
use native_theme_gpui::icons::{
    animated_frames_to_image_sources, lucide_name_for_gpui_icon, material_name_for_gpui_icon,
    to_image_source,
};
use native_theme_gpui::to_theme;

// ---------------------------------------------------------------------------
// Tab indices
// ---------------------------------------------------------------------------
const TAB_BUTTONS: usize = 0;
const TAB_INPUTS: usize = 1;
const TAB_DATA: usize = 2;
const TAB_FEEDBACK: usize = 3;
const TAB_TYPOGRAPHY: usize = 4;
const TAB_LAYOUT: usize = 5;
const TAB_OVERLAYS: usize = 6;
const TAB_CHARTS: usize = 7;
const TAB_ICONS: usize = 8;
const TAB_THEME_MAP: usize = 9;

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

/// Format original native-theme font settings (in logical pixels) for display.
fn format_font_info(
    font: &native_theme::theme::ResolvedFontSpec,
    mono_font: &native_theme::theme::ResolvedFontSpec,
) -> String {
    format!(
        "\nTheme fonts:\n  Font: {} {}px\n  Mono: {} {}px",
        font.family, font.size, mono_font.family, mono_font.size,
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

fn theme_names() -> Vec<SharedString> {
    let preset = platform_preset_name();
    let default_label = format!("default ({})", preset.name);
    let mut names: Vec<SharedString> = vec![default_label.into()];
    names.extend(
        native_theme::theme::Theme::list_presets_for_platform()
            .iter()
            .map(|s| SharedString::from(s.key)),
    );
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

/// Pre-load all 42 icons for the given icon set, tracking source.
/// Parse a dropdown display string back into an `IconSetChoice`.
///
/// The GPUI dropdown gives us a display string, and we need to reconstruct
/// the corresponding `IconSetChoice`.  The "gpui-component built-in (Lucide)"
/// entry is GPUI-specific and maps to `Lucide` (the caller handles the
/// gpui-builtin distinction separately via display string check).
fn parse_icon_set_choice(display: &str) -> IconSetChoice {
    if let Some(inner) = display
        .strip_prefix("default (")
        .and_then(|s| s.strip_suffix(')'))
    {
        IconSetChoice::Default(inner.to_string())
    } else if display.starts_with("system (") {
        IconSetChoice::System
    } else if display == "gpui-component built-in (Lucide)" || display == "Lucide (bundled)" {
        IconSetChoice::Lucide
    } else if display == "Material (bundled)" {
        IconSetChoice::Material
    } else {
        // Bare name = installed freedesktop theme (e.g. "breeze", "Papirus")
        IconSetChoice::Freedesktop(display.to_string())
    }
}

///
/// `default_theme`: when this is `Some(theme_name)` and `icon_set` is
/// `Freedesktop`, icons are loaded via `FreedesktopLoader` with `.theme()` so they come
/// from the specific theme rather than the system default.  This is used for
/// the "default" dropdown selection.  `None` means use the plain `load_icon(role, icon_set)`.
///
/// `cli_override`: CLI `--icon-theme` override, takes priority when the user
/// explicitly selects the system icon set entry.
fn load_all_icons(
    icon_set: IconSet,
    default_theme: Option<&str>,
    cli_override: Option<&str>,
    fg_color: Option<[u8; 3]>,
) -> Vec<(IconRole, Option<IconData>, IconSource)> {
    // For system icon sets, pre-load the Material set so we can detect fallbacks
    // by comparing SVG bytes.
    let is_system_set = matches!(
        icon_set,
        IconSet::Freedesktop | IconSet::SfSymbols | IconSet::SegoeIcons
    );
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
        .map(|(i, role)| {
            // When a CLI override is specified and we're using freedesktop,
            // load from that specific theme via FreedesktopLoader with .theme().
            // When a default theme is specified (from the TOML), use
            // FreedesktopLoader with .theme() which handles freedesktop themes properly.
            #[cfg(target_os = "linux")]
            let data = match icon_set {
                IconSet::Freedesktop => {
                    let mut l = FreedesktopLoader::new(*role).color_opt(fg_color);
                    if let Some(t) = cli_override.or(default_theme) {
                        // cli_override takes priority; name-based lookup via native_icon_name
                        // is only needed when overriding explicitly
                        if cli_override.is_some() {
                            if let Some(name) = native_icon_name(*role, IconSet::Freedesktop) {
                                l = FreedesktopLoader::new(name).theme(t).color_opt(fg_color);
                            }
                        } else {
                            l = l.theme(t);
                        }
                    }
                    l.load()
                }
                IconSet::Material => MaterialLoader::new(*role).load(),
                IconSet::Lucide => LucideLoader::new(*role).load(),
                IconSet::SfSymbols => SfSymbolsLoader::new(*role).load(),
                IconSet::SegoeIcons => SegoeIconsLoader::new(*role).load(),
                _ => None,
            };
            #[cfg(not(target_os = "linux"))]
            let data = match icon_set {
                IconSet::Freedesktop => {
                    let mut l = FreedesktopLoader::new(*role).color_opt(fg_color);
                    if let Some(t) = default_theme {
                        l = l.theme(t);
                    }
                    l.load()
                }
                IconSet::Material => MaterialLoader::new(*role).load(),
                IconSet::Lucide => LucideLoader::new(*role).load(),
                IconSet::SfSymbols => SfSymbolsLoader::new(*role).load(),
                IconSet::SegoeIcons => SegoeIconsLoader::new(*role).load(),
                _ => None,
            };

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
/// Maps gpui icon display names back to IconRole for showcase. When multiple roles
/// map to the same icon (e.g., DialogError and StatusError both -> CircleX), the
/// first listed role is used.
///
/// We match by Lucide icon name string since `IconName` doesn't implement `PartialEq`.
fn role_for_gpui_icon(gpui_name: &str) -> Option<IconRole> {
    // Static table mapping gpui-component icon names to IconRole, derived from
    // the connector's icon_name() mapping.
    match gpui_name {
        "TriangleAlert" => Some(IconRole::DialogWarning),
        "CircleX" => Some(IconRole::DialogError), // also maps StatusError (issue 46/55)
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
        "Loader" => Some(IconRole::StatusBusy),
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
type IconEntry = (
    &'static str,
    IconName,
    Option<IconRole>,
    Option<IconData>,
    IconSource,
);

/// Pre-load native-theme icons for gpui-component IconName variants.
///
/// `default_theme`: when `Some(theme)` and icon_set is Freedesktop, uses
/// `FreedesktopLoader` with `.theme()` to load from the specified theme (for the "default"
/// dropdown selection).
///
/// `cli_override`: CLI `--icon-theme` override that takes priority (for explicit
/// user overrides via `--icon-theme` flag).
fn load_gpui_icons(
    icon_set: Option<IconSet>,
    default_theme: Option<&str>,
    cli_override: Option<&str>,
    fg_color: Option<[u8; 3]>,
) -> Vec<IconEntry> {
    let icon_set = match icon_set {
        Some(set) => set,
        None => {
            // "gpui-builtin" — no native-theme data loaded
            return GPUI_ICONS
                .iter()
                .map(|(name, icon)| {
                    let role = role_for_gpui_icon(name);
                    (*name, icon.clone(), role, None, IconSource::Bundled)
                })
                .collect();
        }
    };

    let is_system_set = matches!(
        icon_set,
        IconSet::Freedesktop | IconSet::SfSymbols | IconSet::SegoeIcons
    );

    // On Linux with freedesktop, detect DE + theme once for the whole batch.
    // Use the CLI override first, then the default_theme, then system fallback.
    #[cfg(target_os = "linux")]
    let (linux_de, fd_theme) = if is_system_set && icon_set == IconSet::Freedesktop {
        let theme = cli_override
            .or(default_theme)
            .map(|s| s.to_string())
            .unwrap_or_else(|| system_icon_theme().to_string());
        (
            Some(parse_linux_desktop(
                &std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default(),
            )),
            Some(theme),
        )
    } else {
        (None, None)
    };

    // Pre-load Material icons once for all roles that appear in GPUI_ICONS,
    // so we can detect system-vs-fallback without redundant per-icon loads.
    // Issue 56: this duplicates the Material pre-load in load_all_icons().
    // A future refactor could share the Material cache between both call sites.
    let material_cache: HashMap<IconRole, Option<IconData>> = if is_system_set {
        GPUI_ICONS
            .iter()
            .filter_map(|(name, _)| role_for_gpui_icon(name))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .map(|r| (r, MaterialLoader::new(r).load()))
            .collect()
    } else {
        HashMap::new()
    };

    GPUI_ICONS
        .iter()
        .map(|(name, icon)| {
            let role = role_for_gpui_icon(name);

            // Try loading by IconRole first (existing path)
            if let Some(r) = role {
                // When a CLI override is specified and we're using freedesktop,
                // load from that specific theme via FreedesktopLoader with .theme().
                // When a default theme is specified (from the TOML), use
                // FreedesktopLoader with .theme().
                #[cfg(target_os = "linux")]
                let data = match icon_set {
                    IconSet::Freedesktop => {
                        if let Some(theme) = cli_override {
                            native_icon_name(r, IconSet::Freedesktop).and_then(|n| {
                                FreedesktopLoader::new(n)
                                    .theme(theme)
                                    .color_opt(fg_color)
                                    .load()
                            })
                        } else {
                            let mut l = FreedesktopLoader::new(r).color_opt(fg_color);
                            if let Some(theme) = default_theme {
                                l = l.theme(theme);
                            }
                            l.load()
                        }
                    }
                    IconSet::Material => MaterialLoader::new(r).load(),
                    IconSet::Lucide => LucideLoader::new(r).load(),
                    IconSet::SfSymbols => SfSymbolsLoader::new(r).load(),
                    IconSet::SegoeIcons => SegoeIconsLoader::new(r).load(),
                    _ => None,
                };
                #[cfg(not(target_os = "linux"))]
                let data = match icon_set {
                    IconSet::Freedesktop => {
                        let mut l = FreedesktopLoader::new(r).color_opt(fg_color);
                        if let Some(theme) = default_theme {
                            l = l.theme(theme);
                        }
                        l.load()
                    }
                    IconSet::Material => MaterialLoader::new(r).load(),
                    IconSet::Lucide => LucideLoader::new(r).load(),
                    IconSet::SfSymbols => SfSymbolsLoader::new(r).load(),
                    IconSet::SegoeIcons => SegoeIconsLoader::new(r).load(),
                    _ => None,
                };
                let source = match &data {
                    None => IconSource::NotFound,
                    Some(_) if !is_system_set => IconSource::Bundled,
                    Some(IconData::Svg(loaded)) => {
                        // Compare against pre-loaded Material icon to detect fallback
                        if let Some(Some(IconData::Svg(mat_bytes))) = material_cache.get(&r) {
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
                if matches!(source, IconSource::Fallback | IconSource::NotFound)
                    && let (Some(de), Some(theme)) = (&linux_de, &fd_theme)
                {
                    let fd_name = freedesktop_name_for_gpui_icon(icon.clone(), *de);
                    if let Some(fd_data) = FreedesktopLoader::new(fd_name)
                        .theme(theme)
                        .color_opt(fg_color)
                        .load()
                    {
                        return (
                            *name,
                            icon.clone(),
                            Some(r),
                            Some(fd_data),
                            IconSource::System,
                        );
                    }
                    // System set but no system icon — mark not found (no bundled fallback)
                    return (*name, icon.clone(), Some(r), None, IconSource::NotFound);
                }
                return (*name, icon.clone(), Some(r), data, source);
            }

            // No IconRole mapping — try by-name lookup for the active icon set
            #[cfg(target_os = "linux")]
            if let (Some(de), Some(theme)) = (&linux_de, &fd_theme) {
                let fd_name = freedesktop_name_for_gpui_icon(icon.clone(), *de);
                if let Some(data) = FreedesktopLoader::new(fd_name)
                    .theme(theme)
                    .color_opt(fg_color)
                    .load()
                {
                    return (*name, icon.clone(), None, Some(data), IconSource::System);
                }
                // System set but no system icon — do NOT fall back to bundled
                return (*name, icon.clone(), None, None, IconSource::NotFound);
            }

            {
                let lookup_name = match icon_set {
                    IconSet::Lucide => Some(lucide_name_for_gpui_icon(icon.clone())),
                    IconSet::Material => Some(material_name_for_gpui_icon(icon.clone())),
                    _ => None,
                };
                if let Some(lname) = lookup_name
                    && let Some(data) = load_icon(lname, icon_set)
                {
                    return (*name, icon.clone(), None, Some(data), IconSource::Bundled);
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
            .child(
                Label::new("Widget Info")
                    .text_size(px(13.0))
                    .font_semibold(),
            )
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
enum AppColorMode {
    System,
    Light,
    Dark,
}

impl AppColorMode {
    /// Resolve to a concrete is_dark bool.
    fn is_dark(self) -> bool {
        match self {
            AppColorMode::Light => false,
            AppColorMode::Dark => true,
            AppColorMode::System => system_is_dark(),
        }
    }

    /// Display label for the combobox, with system preference in parentheses.
    fn label(self) -> String {
        match self {
            AppColorMode::System => {
                let actual = if system_is_dark() { "Dark" } else { "Light" };
                format!("System ({actual})")
            }
            AppColorMode::Light => "Light".into(),
            AppColorMode::Dark => "Dark".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Sample Table Delegate (for Data tab)
// ---------------------------------------------------------------------------

struct SampleTableDelegate {
    columns: Vec<Column>,
    rows: Vec<[SharedString; 3]>,
}

impl TableDelegate for SampleTableDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _cx: &App) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        Label::new(self.rows[row_ix][col_ix].clone()).text_sm()
    }
}

// ---------------------------------------------------------------------------
// Sample List Delegate (for Data tab)
// ---------------------------------------------------------------------------

struct SampleListDelegate {
    items: Vec<SharedString>,
    selected: Option<usize>,
}

impl ListDelegate for SampleListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.items.len()
    }

    fn render_item(
        &mut self,
        ix: gpui_component::IndexPath,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let label = self.items.get(ix.row)?.clone();
        Some(
            ListItem::new(("list-item", ix.row))
                .child(Label::new(label).text_sm())
                .selected(self.selected == Some(ix.row)),
        )
    }

    fn set_selected_index(
        &mut self,
        ix: Option<gpui_component::IndexPath>,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) {
        self.selected = ix.map(|i| i.row);
    }

    fn perform_search(
        &mut self,
        _query: &str,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        Task::ready(())
    }
}

// ---------------------------------------------------------------------------
// Main view
// ---------------------------------------------------------------------------

struct Showcase {
    theme_select: Entity<SelectState<SearchableVec<SharedString>>>,
    current_theme_name: String,
    /// Dynamic label for the "default" theme entry, updated on color mode change.
    default_label: String,
    is_dark: bool,
    color_mode: AppColorMode,
    dark_mode_select: Entity<SelectState<SearchableVec<SharedString>>>,
    /// Original native-theme font spec, for display purposes.
    original_font: native_theme::theme::ResolvedFontSpec,
    /// Original native-theme mono font spec, for display purposes.
    original_mono_font: native_theme::theme::ResolvedFontSpec,

    active_tab: usize,

    // Inputs tab
    input_state: Entity<InputState>,
    number_input_state: Entity<InputState>,
    slider_state: Entity<SliderState>,
    otp_state: Entity<OtpState>,
    color_picker_state: Entity<ColorPickerState>,
    date_picker_state: Entity<gpui_component::date_picker::DatePickerState>,
    calendar_state: Entity<gpui_component::calendar::CalendarState>,
    checkbox_a: bool,
    checkbox_b: bool,
    checkbox_c: bool,
    switch_on: bool,
    radio_index: Option<usize>,
    slider_value: f32,

    // Layout tab
    collapsible_open: bool,

    // Data tab
    table_state: Entity<TableState<SampleTableDelegate>>,
    list_state: Entity<ListState<SampleListDelegate>>,
    tree_state: Entity<TreeState>,

    // Buttons tab
    toggle_bold: bool,
    toggle_italic: bool,

    // Overlays tab
    app_menu_bar: Entity<AppMenuBar>,

    // Icon set selector state
    icon_set_select: Entity<SelectState<SearchableVec<SharedString>>>,
    icon_set_name: String,
    /// Parsed `IconSet` for the current selection (`None` for "gpui-builtin").
    icon_set_enum: Option<IconSet>,
    loaded_icons: Vec<(IconRole, Option<IconData>, IconSource)>,
    gpui_icons: Vec<IconEntry>,
    /// Cached ImageSource per native icon (same indexing as loaded_icons).
    loaded_icon_sources: Vec<Option<ImageSource>>,
    /// Cached ImageSource per gpui icon (same indexing as gpui_icons).
    gpui_icon_sources: Vec<Option<ImageSource>>,
    /// Foreground color used when building the image source caches.
    icon_cache_fg: Hsla,
    /// The user's icon set selection intent (library type).
    icon_set_choice: IconSetChoice,
    /// Cached list of installed freedesktop icon themes (populated once at init).
    installed_themes: Vec<String>,
    /// The current resolved theme's preferred icon theme (e.g. "breeze", "Lucide").
    current_icon_theme: String,
    /// The current resolved theme's icon set (loading mechanism).
    current_icon_set: IconSet,
    /// Whether the current theme's TOML specified `icon_theme` (before resolution).
    has_toml_icon_theme: bool,
    /// CLI override for the freedesktop icon theme (e.g. "breeze", "breeze-dark", "adwaita").
    icon_theme_override: Option<String>,

    // Animated Icons state
    /// Cached frame ImageSources for frame-based animations (set name, frames).
    animated_frame_sources: Vec<(String, Vec<ImageSource>)>,
    /// Frame duration in ms for each frame-based animation (parallel to animated_frame_sources).
    animated_frame_durations: Vec<u32>,
    /// Current frame index for each frame-based animation.
    animated_frame_indices: Vec<usize>,
    /// Cached ImageSource for transform-based (spin) animations (set name, source, duration_ms).
    animated_spin_sources: Vec<(String, ImageSource, u32)>,
    /// Timer task handle for frame cycling (dropped to cancel).
    animation_timer: Option<Task<()>>,
    /// Whether reduced motion is active.
    reduced_motion: bool,
    /// Static first-frame ImageSources for reduced motion display (set name, source, anim type label).
    animated_static_sources: Vec<(String, ImageSource, &'static str)>,

    /// Widget Info sidebar panel (separate Entity for independent re-render).
    widget_info_panel: Entity<WidgetInfoPanel>,

    /// Error message from theme loading, displayed as a banner in the UI.
    error_message: Option<String>,

    // Theme watcher (runtime dark/light toggle detection)
    /// Flag set by the ThemeSubscription background thread when the OS theme changes.
    theme_change_flag: Arc<AtomicBool>,
    /// RAII guard keeping the theme watcher background thread alive.
    _theme_watcher: Option<native_theme::watch::ThemeSubscription>,
    /// Set by the watcher polling task; checked in render() where window access is available.
    pending_system_theme_change: bool,
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
                data.as_ref().and_then(|d| {
                    if *source == IconSource::System {
                        to_image_source(d, None, None)
                    } else {
                        to_image_source(d, Some(fg), None)
                    }
                })
            })
            .collect();
        self.gpui_icon_sources = self
            .gpui_icons
            .iter()
            .map(|(_, _, _, data, source)| {
                data.as_ref().and_then(|d| {
                    if *source == IconSource::System {
                        to_image_source(d, None, None)
                    } else {
                        to_image_source(d, Some(fg), None)
                    }
                })
            })
            .collect();
    }

    /// Rebuild cached animated icon data from `load_icon_indicator()`.
    ///
    /// Called at init and whenever the icon set changes so that animated icon
    /// rendering can use pre-built `ImageSource` objects without re-rasterizing
    /// SVGs on every frame tick.
    fn rebuild_animation_caches(&mut self) {
        self.animated_frame_sources.clear();
        self.animated_frame_durations.clear();
        self.animated_spin_sources.clear();
        self.animated_static_sources.clear();

        let set_name = &self.icon_set_name;
        let fg = self.icon_cache_fg;
        // gpui-builtin is not a native-theme icon set; load_indicator would
        // fall back to the system set, showing the wrong spinner.
        if let Some(icon_set) = self.icon_set_enum
            && let Some(anim) = load_icon_indicator(icon_set)
        {
            match &anim {
                AnimatedIcon::Frames(data) => {
                    if let Some(anim_sources) =
                        animated_frames_to_image_sources(&anim, Some(fg), None)
                    {
                        if let Some(first_source) =
                            to_image_source(anim.first_frame(), Some(fg), None)
                        {
                            self.animated_static_sources.push((
                                set_name.to_string(),
                                first_source,
                                "Frames",
                            ));
                        }
                        self.animated_frame_durations
                            .push(data.frame_duration_ms().get());
                        self.animated_frame_sources
                            .push((set_name.to_string(), anim_sources.sources));
                    }
                }
                AnimatedIcon::Transform(data) => {
                    if let Some(source) = to_image_source(data.icon(), None, None) {
                        self.animated_static_sources.push((
                            set_name.to_string(),
                            source.clone(),
                            "Transform",
                        ));
                        if let TransformAnimation::Spin { duration_ms } = data.animation() {
                            self.animated_spin_sources.push((
                                set_name.to_string(),
                                source,
                                duration_ms.get(),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        self.animated_frame_indices = vec![0; self.animated_frame_sources.len()];
        self.reduced_motion = prefers_reduced_motion();
    }

    /// Start (or restart) the frame-cycling timer for animated icons.
    ///
    /// Cancels any previous timer. Does nothing when `reduced_motion` is true
    /// or there are no frame-based animations cached.
    fn start_animation_timer(&mut self, cx: &mut Context<Self>) {
        // Drop old timer (cancels the task)
        self.animation_timer = None;

        if self.reduced_motion || self.animated_frame_sources.is_empty() {
            return;
        }

        // Invariant: animated_frame_durations is pushed in lockstep with
        // animated_frame_sources (see rebuild_animation_caches). If sources
        // is non-empty, durations is non-empty, so min() returns Some.
        let Some(min_duration) = self
            .animated_frame_durations
            .iter()
            .copied()
            .min()
            .map(u64::from)
        else {
            return;
        };

        let task = cx.spawn(async move |this, cx| {
            loop {
                Timer::after(Duration::from_millis(min_duration)).await;
                let Ok(()) = this.update(cx, |this, cx| {
                    for (i, (_name, frames)) in this.animated_frame_sources.iter().enumerate() {
                        if let Some(idx) = this.animated_frame_indices.get_mut(i) {
                            *idx = (*idx + 1) % frames.len();
                        }
                    }
                    cx.notify();
                }) else {
                    break;
                };
            }
        });

        self.animation_timer = Some(task);
    }

    /// Build the list of icon set dropdown names.
    fn icon_set_dropdown_names(&self) -> Vec<SharedString> {
        let icon_theme_opt = if self.has_toml_icon_theme {
            Some(self.current_icon_theme.as_str())
        } else {
            None
        };
        let mut names: Vec<SharedString> = Vec::new();
        // "default (X)" -- only when TOML specifies icon_theme and it's available
        if let choice @ IconSetChoice::Default(_) =
            default_icon_choice(self.current_icon_set, icon_theme_opt)
        {
            names.push(choice.to_string().into());
        }
        // "system (Y)" -- always
        names.push(IconSetChoice::System.to_string().into());
        // Installed freedesktop themes
        for name in &self.installed_themes {
            names.push(IconSetChoice::Freedesktop(name.clone()).to_string().into());
        }
        // GPUI-specific built-in
        names.push("gpui-component built-in (Lucide)".into());
        // Bundled
        names.push(IconSetChoice::Lucide.to_string().into());
        names.push(IconSetChoice::Material.to_string().into());
        names
    }

    /// Convert a display name from the theme selector to the internal theme name.
    fn theme_internal_name(display: &str) -> String {
        if display.starts_with("default (") {
            "default".to_string()
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
            |this: &mut Self,
             _entity,
             event: &SelectEvent<SearchableVec<SharedString>>,
             window,
             cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    let name = Self::theme_internal_name(value.as_ref());
                    this.current_theme_name = name.clone();
                    this.apply_theme_by_name(&name, window, cx);
                }
            },
        )
        .detach();

        // Color mode selector (System / Light / Dark)
        let color_mode = AppColorMode::System;
        let color_mode_labels: Vec<SharedString> = [
            AppColorMode::System,
            AppColorMode::Light,
            AppColorMode::Dark,
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
            |this: &mut Self,
             _entity,
             event: &SelectEvent<SearchableVec<SharedString>>,
             window,
             cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    let val = value.to_string();
                    let mode = if val.starts_with("System") {
                        AppColorMode::System
                    } else if val == "Light" {
                        AppColorMode::Light
                    } else {
                        AppColorMode::Dark
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
                    // Issue 57: guard against NaN/Inf from malformed input
                    let num = if num.is_finite() { num } else { 0.0 };
                    let new_value = if *action == StepAction::Increment {
                        num + 1.0
                    } else {
                        num - 1.0
                    };
                    input.set_value(SharedString::from(new_value.to_string()), window, cx);
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

        // Apply the initial OS Theme via native-theme pipeline.
        let is_dark = color_mode.is_dark();
        let (
            original_font,
            original_mono_font,
            initial_default_label,
            initial_icon_theme,
            initial_icon_set,
            initial_has_toml_icon_theme,
            initial_error,
        ) = match native_theme::SystemTheme::from_system() {
            Ok(system) => {
                let resolved = system.pick(if is_dark {
                    native_theme_gpui::ColorMode::Dark
                } else {
                    native_theme_gpui::ColorMode::Light
                });
                let font = resolved.defaults.font.clone();
                let mono_font = resolved.defaults.mono_font.clone();
                let icon_theme = system.icon_theme.clone().into_owned();
                let icon_set = system.icon_set;
                let theme = to_theme(
                    resolved,
                    &system.name,
                    is_dark,
                    system.accessibility.reduce_transparency,
                );
                *Theme::global_mut(cx) = theme;
                window.refresh();
                let label = format!("default ({})", system.preset);
                // Platform presets always specify icon_theme
                (font, mono_font, label, icon_theme, icon_set, true, None)
            }
            Err(e) => {
                // Fall back to gpui-component built-in theme so the window still renders
                Theme::sync_system_appearance(Some(window), cx);
                let font = native_theme::theme::ResolvedFontSpec {
                    family: "(default)".into(),
                    size: 0.0,
                    weight: 400,
                    style: native_theme::theme::FontStyle::Normal,
                    color: native_theme::color::Rgba::TRANSPARENT,
                };
                let mono_font = native_theme::theme::ResolvedFontSpec {
                    family: "(default)".into(),
                    size: 0.0,
                    weight: 400,
                    style: native_theme::theme::FontStyle::Normal,
                    color: native_theme::color::Rgba::TRANSPARENT,
                };
                let preset = platform_preset_name();
                let label = format!("default ({})", preset.name);
                let icon_theme = system_icon_theme().to_string();
                let icon_set = system_icon_set();
                (
                    font,
                    mono_font,
                    label,
                    icon_theme,
                    icon_set,
                    false,
                    Some(format!("Failed to load OS theme: {e}")),
                )
            }
        };
        // Use the library's IconSetChoice to compute the initial icon selection.
        let icon_theme_opt = if initial_has_toml_icon_theme {
            Some(initial_icon_theme.as_str())
        } else {
            None
        };
        let initial_icon_set_choice = default_icon_choice(initial_icon_set, icon_theme_opt);
        let initial_effective_set = initial_icon_set_choice.effective_icon_set(initial_icon_set);
        let initial_default_theme = initial_icon_set_choice
            .freedesktop_theme()
            .map(|s| s.to_string());
        let initial_resolved_name = initial_effective_set.name().to_string();
        let installed_themes = list_freedesktop_themes();
        let fc = original_font.color;
        let fg = Some([fc.r, fc.g, fc.b]);
        let loaded_icons = load_all_icons(
            initial_effective_set,
            initial_default_theme.as_deref(),
            None,
            fg,
        );
        let gpui_icons = load_gpui_icons(
            Some(initial_effective_set),
            initial_default_theme.as_deref(),
            None,
            fg,
        );

        // Icon theme selector -- build dropdown list using IconSetChoice
        let initial_icon_label = initial_icon_set_choice.to_string();
        let mut icon_theme_names: Vec<SharedString> = Vec::new();
        // "default (X)" -- only when TOML specifies icon_theme and it's available
        if let choice @ IconSetChoice::Default(_) =
            default_icon_choice(initial_icon_set, icon_theme_opt)
        {
            icon_theme_names.push(choice.to_string().into());
        }
        // "system (Y)" -- always
        icon_theme_names.push(IconSetChoice::System.to_string().into());
        // Installed freedesktop themes
        for name in &installed_themes {
            icon_theme_names.push(IconSetChoice::Freedesktop(name.clone()).to_string().into());
        }
        // GPUI-specific built-in
        icon_theme_names.push("gpui-component built-in (Lucide)".into());
        // Bundled
        icon_theme_names.push(IconSetChoice::Lucide.to_string().into());
        icon_theme_names.push(IconSetChoice::Material.to_string().into());

        // Find the index of the initial selection label
        let initial_icon_idx = icon_theme_names
            .iter()
            .position(|n| n.as_ref() == initial_icon_label)
            .unwrap_or(0);
        let icon_set_delegate = SearchableVec::new(icon_theme_names);
        let icon_set_select = cx.new(|cx| {
            SelectState::new(
                icon_set_delegate,
                Some(gpui_component::IndexPath::default().row(initial_icon_idx)),
                window,
                cx,
            )
        });

        cx.subscribe_in(
            &icon_set_select,
            window,
            |this: &mut Self,
             _entity,
             event: &SelectEvent<SearchableVec<SharedString>>,
             _window,
             cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    let display = value.to_string();
                    let is_gpui_builtin = display == "gpui-component built-in (Lucide)";
                    this.icon_set_choice = parse_icon_set_choice(&display);
                    let effective = this
                        .icon_set_choice
                        .effective_icon_set(this.current_icon_set);
                    let default_theme = this
                        .icon_set_choice
                        .freedesktop_theme()
                        .map(|s| s.to_string());
                    this.icon_set_name = effective.name().to_string();
                    // For gpui-builtin, icon_set_enum is None (uses gpui-component's
                    // built-in icons rather than native-theme's loader).
                    this.icon_set_enum = if is_gpui_builtin {
                        None
                    } else {
                        Some(effective)
                    };
                    let cli_ref = this.icon_theme_override.as_deref();
                    let fc = this.original_font.color;
                    let fg_rgb = Some([fc.r, fc.g, fc.b]);
                    if !is_gpui_builtin {
                        this.loaded_icons =
                            load_all_icons(effective, default_theme.as_deref(), cli_ref, fg_rgb);
                    }
                    this.gpui_icons = load_gpui_icons(
                        this.icon_set_enum,
                        default_theme.as_deref(),
                        cli_ref,
                        fg_rgb,
                    );
                    let fg = cx.theme().foreground;
                    this.rebuild_icon_caches(fg);
                    this.rebuild_animation_caches();
                    this.start_animation_timer(cx);
                    cx.notify();
                }
            },
        )
        .detach();

        // OTP input state (6 digits)
        let otp_state = cx.new(|cx| OtpState::new(6, window, cx));

        // Color picker state
        let color_picker_state = cx.new(|cx| {
            ColorPickerState::new(window, cx).default_value(gpui::hsla(0.6, 0.8, 0.5, 1.0))
        });

        // Date picker state
        let date_picker_state =
            cx.new(|cx| gpui_component::date_picker::DatePickerState::new(window, cx));

        // Calendar state
        let calendar_state = cx.new(|cx| gpui_component::calendar::CalendarState::new(window, cx));

        // Table state with sample data
        let table_state = cx.new(|cx| {
            let delegate = SampleTableDelegate {
                columns: vec![
                    Column::new("name", "Name"),
                    Column::new("role", "Role"),
                    Column::new("status", "Status"),
                ],
                rows: vec![
                    ["Alice".into(), "Engineer".into(), "Active".into()],
                    ["Bob".into(), "Designer".into(), "Away".into()],
                    ["Carol".into(), "Manager".into(), "Active".into()],
                    ["Dave".into(), "Intern".into(), "Offline".into()],
                    ["Eve".into(), "DevOps".into(), "Active".into()],
                ],
            };
            TableState::new(delegate, window, cx)
        });

        // List state with sample items
        let list_state = cx.new(|cx| {
            let delegate = SampleListDelegate {
                items: vec![
                    "Inbox".into(),
                    "Starred".into(),
                    "Sent".into(),
                    "Drafts".into(),
                    "Trash".into(),
                    "Archive".into(),
                ],
                selected: None,
            };
            ListState::new(delegate, window, cx)
        });

        // Tree state with sample file structure
        let tree_state = cx.new(|cx| {
            TreeState::new(cx).items(vec![
                TreeItem::new("src", "src")
                    .expanded(true)
                    .child(TreeItem::new("lib", "lib.rs"))
                    .child(TreeItem::new("main", "main.rs"))
                    .child(
                        TreeItem::new("utils", "utils")
                            .child(TreeItem::new("helpers", "helpers.rs")),
                    ),
                TreeItem::new("cargo", "Cargo.toml"),
                TreeItem::new("readme", "README.md"),
            ])
        });

        // Set up application menus for AppMenuBar
        cx.set_menus(vec![
            Menu {
                name: "File".into(),
                items: vec![
                    MenuItem::action("New", gpui::NoAction),
                    MenuItem::action("Open", gpui::NoAction),
                    MenuItem::separator(),
                    MenuItem::action("Save", gpui::NoAction),
                    MenuItem::action("Save As…", gpui::NoAction),
                    MenuItem::separator(),
                    MenuItem::action("Quit", gpui::NoAction),
                ],
            },
            Menu {
                name: "Edit".into(),
                items: vec![
                    MenuItem::action("Undo", gpui::NoAction),
                    MenuItem::action("Redo", gpui::NoAction),
                    MenuItem::separator(),
                    MenuItem::action("Cut", gpui::NoAction),
                    MenuItem::action("Copy", gpui::NoAction),
                    MenuItem::action("Paste", gpui::NoAction),
                    MenuItem::separator(),
                    MenuItem::action("Select All", gpui::NoAction),
                ],
            },
            Menu {
                name: "View".into(),
                items: vec![
                    MenuItem::action("Zoom In", gpui::NoAction),
                    MenuItem::action("Zoom Out", gpui::NoAction),
                    MenuItem::separator(),
                    MenuItem::action("Toggle Sidebar", gpui::NoAction),
                    MenuItem::action("Toggle Full Screen", gpui::NoAction),
                ],
            },
            Menu {
                name: "Help".into(),
                items: vec![
                    MenuItem::action("Documentation", gpui::NoAction),
                    MenuItem::action("About", gpui::NoAction),
                ],
            },
        ]);
        let app_menu_bar = AppMenuBar::new(window, cx);

        // Start theme watcher for runtime dark/light toggle detection.
        // Skip in screenshot mode — the watcher's background thread cleanup
        // races with the Cocoa runtime on macOS CI, causing SIGTRAP on exit.
        let theme_change_flag = Arc::new(AtomicBool::new(false));
        let is_screenshot = std::env::args().any(|a| a == "--screenshot");
        let _theme_watcher = if is_screenshot {
            None
        } else {
            let flag_clone = theme_change_flag.clone();
            native_theme::watch::on_theme_change(move |_event| {
                flag_clone.store(true, Ordering::Release);
            })
            .ok()
        };

        let fg = cx.theme().foreground;
        let mut showcase = Self {
            theme_select,
            current_theme_name: "default".into(),
            default_label: initial_default_label,
            is_dark,
            color_mode,
            dark_mode_select,
            original_font,
            original_mono_font,
            active_tab: TAB_BUTTONS,
            input_state,
            number_input_state,
            slider_state,
            otp_state,
            color_picker_state,
            date_picker_state,
            calendar_state,
            checkbox_a: true,
            checkbox_b: false,
            checkbox_c: false,
            switch_on: false,
            radio_index: Some(0),
            slider_value: 65.0,
            collapsible_open: true,
            table_state,
            list_state,
            tree_state,
            toggle_bold: false,
            toggle_italic: false,
            app_menu_bar,
            icon_set_select,
            icon_set_name: initial_resolved_name,
            icon_set_enum: Some(initial_effective_set),
            loaded_icons,
            gpui_icons,
            loaded_icon_sources: Vec::new(),
            gpui_icon_sources: Vec::new(),
            icon_cache_fg: fg,
            icon_set_choice: initial_icon_set_choice,
            installed_themes,
            current_icon_theme: initial_icon_theme,
            current_icon_set: initial_icon_set,
            has_toml_icon_theme: initial_has_toml_icon_theme,
            icon_theme_override: None,
            animated_frame_sources: Vec::new(),
            animated_frame_durations: Vec::new(),
            animated_frame_indices: Vec::new(),
            animated_spin_sources: Vec::new(),
            animation_timer: None,
            reduced_motion: false,
            animated_static_sources: Vec::new(),
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
            error_message: initial_error,
            theme_change_flag,
            _theme_watcher,
            pending_system_theme_change: false,
        };
        showcase.rebuild_icon_caches(fg);
        showcase.rebuild_animation_caches();
        showcase.start_animation_timer(cx);
        showcase.start_theme_watcher(cx);
        showcase
    }

    fn show_theme_error(&mut self, msg: &str) {
        self.error_message = Some(msg.to_string());
    }

    fn apply_theme_by_name(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) {
        if name == "default" {
            match native_theme::SystemTheme::from_system() {
                Ok(system) => {
                    let resolved = system.pick(if self.is_dark {
                        native_theme_gpui::ColorMode::Dark
                    } else {
                        native_theme_gpui::ColorMode::Light
                    });
                    self.original_font = resolved.defaults.font.clone();
                    self.original_mono_font = resolved.defaults.mono_font.clone();
                    self.current_icon_theme = system.icon_theme.clone().into_owned();
                    self.current_icon_set = system.icon_set;
                    // Platform presets always specify icon_theme
                    self.has_toml_icon_theme = true;
                    let theme = to_theme(
                        resolved,
                        &system.name,
                        self.is_dark,
                        system.accessibility.reduce_transparency,
                    );
                    *Theme::global_mut(cx) = theme;
                    window.refresh();
                    self.default_label = format!("default ({})", system.preset);
                    self.error_message = None;
                }
                Err(e) => {
                    self.show_theme_error(&format!("Failed to load OS theme: {e}"));
                }
            }
        } else {
            let nt = match native_theme::theme::Theme::preset(name) {
                Ok(t) => t,
                Err(e) => {
                    self.show_theme_error(&format!("Failed to load preset '{name}': {e}"));
                    return;
                }
            };

            if let Ok(variant) = nt.pick_variant(if self.is_dark {
                native_theme_gpui::ColorMode::Dark
            } else {
                native_theme_gpui::ColorMode::Light
            }) {
                // icon_theme is per-variant on defaults
                self.has_toml_icon_theme = variant.defaults.icon_theme.is_some();
                let icon_set = nt
                    .icon_set
                    .unwrap_or_else(native_theme::theme::system_icon_set);
                let icon_theme = variant
                    .defaults
                    .icon_theme
                    .as_deref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(native_theme::theme::system_icon_theme);
                let v = variant.clone();
                let resolved = match v.into_resolved(None) {
                    Ok(r) => r,
                    Err(e) => {
                        self.show_theme_error(&format!("Theme '{name}' validation failed: {e}"));
                        return;
                    }
                };
                self.original_font = resolved.defaults.font.clone();
                self.original_mono_font = resolved.defaults.mono_font.clone();
                self.current_icon_theme = icon_theme;
                self.current_icon_set = icon_set;
                let theme = to_theme(&resolved, name, self.is_dark, false);
                *Theme::global_mut(cx) = theme;
                window.refresh();
                self.error_message = None;
            }
        }

        // Only re-derive icon choice when user is in "follow preset" mode
        if self.icon_set_choice.follows_preset() {
            let icon_theme_opt = if self.has_toml_icon_theme {
                Some(self.current_icon_theme.as_str())
            } else {
                None
            };
            self.icon_set_choice = default_icon_choice(self.current_icon_set, icon_theme_opt);
            let effective = self
                .icon_set_choice
                .effective_icon_set(self.current_icon_set);
            self.icon_set_name = effective.name().to_string();
            self.icon_set_enum = Some(effective);

            // Update the icon theme dropdown to reflect the new effective icon theme
            let selected_label: SharedString = self.icon_set_choice.to_string().into();
            let icon_names = self.icon_set_dropdown_names();
            let new_delegate = SearchableVec::new(icon_names);
            self.icon_set_select.update(cx, |select, cx| {
                select.set_items(new_delegate, window, cx);
                select.set_selected_value(&selected_label, window, cx);
            });
        }
        // ALWAYS reload icons regardless of choice (text color changes on dark/light)
        {
            let effective = self
                .icon_set_choice
                .effective_icon_set(self.current_icon_set);
            let default_theme = self
                .icon_set_choice
                .freedesktop_theme()
                .map(|s| s.to_string());
            let cli_ref = self.icon_theme_override.as_deref();
            let fc = self.original_font.color;
            let fg_rgb = Some([fc.r, fc.g, fc.b]);
            self.loaded_icons =
                load_all_icons(effective, default_theme.as_deref(), cli_ref, fg_rgb);
            self.gpui_icons =
                load_gpui_icons(Some(effective), default_theme.as_deref(), cli_ref, fg_rgb);
        }
        let fg = cx.theme().foreground;
        self.rebuild_icon_caches(fg);
        self.rebuild_animation_caches();
        self.start_animation_timer(cx);
    }

    /// Spawn a background task that polls the theme change flag and triggers
    /// a theme rebuild when the OS color scheme changes at runtime.
    fn start_theme_watcher(&self, cx: &mut Context<Self>) {
        if self._theme_watcher.is_none() {
            return;
        }
        let flag = self.theme_change_flag.clone();
        cx.spawn(async move |this, cx| {
            loop {
                Timer::after(Duration::from_millis(500)).await;
                if flag.swap(false, Ordering::AcqRel) {
                    let Ok(()) = this.update(cx, |this, cx| {
                        if matches!(this.color_mode, AppColorMode::System) {
                            this.pending_system_theme_change = true;
                            cx.notify();
                        }
                    }) else {
                        break;
                    };
                }
            }
        })
        .detach();
    }

    fn set_color_mode(&mut self, mode: AppColorMode, window: &mut Window, cx: &mut Context<Self>) {
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
        let font_family_str = self.original_font.family.clone();
        let font_size_str = format!("{}px", self.original_font.size);
        let mono_family_str = self.original_mono_font.family.clone();
        let mono_size_str = format!("{}px", self.original_mono_font.size);
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
            radius_str,
            radius_lg_str,
            font_family_str,
            font_size_str,
            mono_family_str,
            mono_size_str,
            shadow_str,
            scrollbar_str,
        );

        let style = TextViewStyle::default()
            .paragraph_gap(rems(0.3))
            .heading_font_size(|_level, _base| px(13.0));

        v_flex().p_3().w_full().child(
            TextView::markdown("config-inspector", SharedString::from(md), window, cx)
                .selectable(true)
                .style(style)
                .text_xs(),
        )
    }

    // -----------------------------------------------------------------------
    // Tab: Buttons
    // -----------------------------------------------------------------------
    fn render_buttons_tab(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
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
                            .on_hover(self.hover_info(
                                &fi,
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
                            .on_hover(self.hover_info(
                                &fi,
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
                            .on_hover(self.hover_info(
                                &fi,
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
                            .on_hover(self.hover_info(
                                &fi,
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
                            .on_hover(self.hover_info(
                                &fi,
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
                            .on_hover(self.hover_info(
                                &fi,
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
                            .on_hover(self.hover_info(
                                &fi,
                                "Button (Ghost)",
                                &[
                                    ("text", "foreground", t.foreground),
                                    ("hover-text", "muted_foreground", t.muted_foreground),
                                ],
                                &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                &[
                                    ("padding", "set per Size enum"),
                                    ("font-weight", "hardcoded"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-link")
                            .child(Button::new("b-link").label("Link").link())
                            .on_hover(self.hover_info(
                                &fi,
                                "Button (Link)",
                                &[
                                    ("text", "foreground", t.foreground),
                                    ("hover-text", "muted_foreground", t.muted_foreground),
                                ],
                                &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                &[
                                    ("padding", "set per Size enum"),
                                    ("font-weight", "hardcoded"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-text")
                            .child(Button::new("b-text").label("Text").text())
                            .on_hover(self.hover_info(
                                &fi,
                                "Button (Text)",
                                &[
                                    ("text", "foreground", t.foreground),
                                    ("hover-text", "muted_foreground", t.muted_foreground),
                                ],
                                &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                &[
                                    ("padding", "set per Size enum"),
                                    ("font-weight", "hardcoded"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-outline")
                            .child(
                                Button::new("b-outline")
                                    .label("Outline")
                                    .primary()
                                    .outline(),
                            )
                            .on_hover(self.hover_info(
                                &fi,
                                "Button (Primary Outline)",
                                &[
                                    ("border", "primary", t.primary),
                                    ("text", "primary", t.primary),
                                    ("hover bg", "primary_hover", t.primary_hover),
                                ],
                                &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                &[
                                    ("padding", "set per Size enum"),
                                    ("font-weight", "hardcoded"),
                                ],
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
                            .child(
                                Button::new("d-pri")
                                    .label("Disabled Primary")
                                    .primary()
                                    .disabled(true),
                            )
                            .child(
                                Button::new("d-sec")
                                    .label("Disabled Secondary")
                                    .disabled(true),
                            )
                            .child(
                                Button::new("d-dng")
                                    .label("Disabled Danger")
                                    .danger()
                                    .disabled(true),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
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
                        h_flex().gap_2().child(
                            Button::new("l-pri")
                                .label("Loading...")
                                .primary()
                                .loading(true),
                        ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
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
                            .child(
                                Button::new("bi-save")
                                    .label("Save")
                                    .primary()
                                    .icon(IconName::Check),
                            )
                            .child(
                                Button::new("bi-search")
                                    .label("Search")
                                    .icon(IconName::Search),
                            )
                            .child(
                                Button::new("bi-del")
                                    .label("Delete")
                                    .danger()
                                    .icon(IconName::Delete),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
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
            // DropdownButton
            .child(section("DropdownButton"))
            .child(
                div()
                    .id("tt-dropdown-btn")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                DropdownButton::new("dropdown-1")
                                    .button(Button::new("dropdown-main").label("Save").primary())
                                    .dropdown_menu(|menu, _w, _cx| {
                                        menu.menu("Save as Draft", Box::new(gpui::NoAction))
                                            .separator()
                                            .menu("Export as PDF", Box::new(gpui::NoAction))
                                    }),
                            )
                            .child(
                                DropdownButton::new("dropdown-2")
                                    .button(Button::new("dropdown-sec").label("Actions"))
                                    .dropdown_menu(|menu, _w, _cx| {
                                        menu.menu("Cut", Box::new(gpui::NoAction))
                                            .menu("Copy", Box::new(gpui::NoAction))
                                            .menu("Paste", Box::new(gpui::NoAction))
                                    }),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "DropdownButton",
                        &[
                            ("bg", "primary/secondary", t.primary),
                            ("dropdown border", "border", t.border),
                            ("menu bg", "popover", t.popover),
                        ],
                        &[],
                        &[("dropdown arrow", "hardcoded ChevronDown")],
                    )),
            )
            // Toggle & ToggleGroup
            .child(section("Toggle & ToggleGroup"))
            .child(
                div()
                    .id("tt-toggle")
                    .child(
                        h_flex()
                            .gap_6()
                            .items_center()
                            .child(
                                Toggle::new("tog-bold")
                                    .icon(IconName::Star)
                                    .checked(self.toggle_bold)
                                    .on_click(cx.listener(|this, checked: &bool, _w, _cx| {
                                        this.toggle_bold = *checked;
                                    })),
                            )
                            .child(
                                Toggle::new("tog-italic")
                                    .icon(IconName::Heart)
                                    .checked(self.toggle_italic)
                                    .on_click(cx.listener(|this, checked: &bool, _w, _cx| {
                                        this.toggle_italic = *checked;
                                    })),
                            )
                            .child(
                                ToggleGroup::new("tog-group-1")
                                    .child(Toggle::new("tg-left").label("Left"))
                                    .child(Toggle::new("tg-center").label("Center"))
                                    .child(Toggle::new("tg-right").label("Right")),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Toggle / ToggleGroup",
                        &[
                            ("checked bg", "secondary_active", t.secondary_active),
                            (
                                "checked text",
                                "secondary_foreground",
                                t.secondary_foreground,
                            ),
                            ("unchecked bg", "secondary", t.secondary),
                            ("hover", "secondary_hover", t.secondary_hover),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[],
                    )),
            )
            // Clipboard
            .child(section("Clipboard"))
            .child(
                div()
                    .id("tt-clipboard")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(Clipboard::new("clip-1").value("cargo add native-theme"))
                            .child(Clipboard::new("clip-2").value("npm install native-theme")),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Clipboard",
                        &[
                            ("bg", "secondary", t.secondary),
                            ("text", "foreground", t.foreground),
                            ("icon", "muted_foreground", t.muted_foreground),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[("copy icon", "hardcoded Clipboard/ClipboardCheck")],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Inputs
    // -----------------------------------------------------------------------
    fn render_inputs_tab(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
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
                    .on_hover(self.hover_info(
                        &fi,
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
                        &[
                            ("padding", "set per Size enum"),
                            ("height", "set per Size enum"),
                        ],
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
                        &[
                            ("size", "set per Size enum"),
                            ("indicator size", "hardcoded"),
                        ],
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
            // OTP Input
            .child(section("OTP Input (6 digits)"))
            .child(
                div()
                    .id("tt-otp")
                    .child(OtpInput::new(&self.otp_state).groups(2))
                    .on_hover(self.hover_info(
                        &fi,
                        "OtpInput",
                        &[
                            ("bg", "input", t.input),
                            ("border", "border", t.border),
                            ("focus", "ring", t.ring),
                            ("text", "foreground", t.foreground),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[("digit count", "configurable"), ("groups", "2")],
                    )),
            )
            // Color Picker
            .child(section("ColorPicker"))
            .child(
                div()
                    .id("tt-colorpicker")
                    .child(ColorPicker::new(&self.color_picker_state).label("Pick a color"))
                    .on_hover(self.hover_info(
                        &fi,
                        "ColorPicker",
                        &[
                            ("bg", "input", t.input),
                            ("border", "border", t.border),
                            ("popover", "popover", t.popover),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[("palette", "hardcoded HSL picker")],
                    )),
            )
            // Date Picker
            .child(section("DatePicker"))
            .child(
                div()
                    .id("tt-datepicker")
                    .child(
                        gpui_component::date_picker::DatePicker::new(&self.date_picker_state)
                            .placeholder("Select a date"),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "DatePicker",
                        &[
                            ("bg", "input", t.input),
                            ("border", "border", t.border),
                            ("popover", "popover", t.popover),
                            ("selected", "primary", t.primary),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("calendar icon", "hardcoded"),
                            ("format", "default YYYY-MM-DD"),
                        ],
                    )),
            )
            // Calendar
            .child(section("Calendar"))
            .child(
                div()
                    .id("tt-calendar")
                    .child(gpui_component::calendar::Calendar::new(
                        &self.calendar_state,
                    ))
                    .on_hover(self.hover_info(
                        &fi,
                        "Calendar",
                        &[
                            ("bg", "popover", t.popover),
                            ("selected day", "primary", t.primary),
                            ("today", "secondary", t.secondary),
                            ("text", "foreground", t.foreground),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[("month navigation", "hardcoded arrows")],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Data
    // -----------------------------------------------------------------------
    fn render_data_tab(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
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
                    .on_hover(self.hover_info(
                        &fi,
                        "DescriptionList",
                        &[
                            (
                                "label bg",
                                "description_list_label",
                                t.description_list_label,
                            ),
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
            // Table
            .child(section("Table (striped, 3 cols × 5 rows)"))
            .child(
                div()
                    .id("tt-table")
                    .h(px(220.0))
                    .child(Table::new(&self.table_state).stripe(true).bordered(true))
                    .on_hover(self.hover_info(
                        &fi,
                        "Table",
                        &[
                            ("header bg", "table_head", t.table_head),
                            (
                                "header text",
                                "table_head_foreground",
                                t.table_head_foreground,
                            ),
                            ("row bg", "table", t.table),
                            ("stripe", "table_even", t.table_even),
                            ("active row", "table_active", t.table_active),
                            ("hover", "table_hover", t.table_hover),
                            ("border", "table_row_border", t.table_row_border),
                        ],
                        &[],
                        &[("row height", "hardcoded per Size")],
                    )),
            )
            // List
            .child(section("List (selectable)"))
            .child(
                div()
                    .id("tt-list")
                    .h(px(200.0))
                    .w(px(260.0))
                    .border_1()
                    .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
                    .child(gpui_component::list::List::new(&self.list_state))
                    .on_hover(self.hover_info(
                        &fi,
                        "List",
                        &[
                            ("bg", "list", t.list),
                            ("active", "list_active", t.list_active),
                            ("hover", "list_hover", t.list_hover),
                            ("even", "list_even", t.list_even),
                        ],
                        &[],
                        &[("item height", "hardcoded per Size")],
                    )),
            )
            // Tree
            .child(section("Tree (file structure)"))
            .child(
                div()
                    .id("tt-tree")
                    .h(px(200.0))
                    .w(px(260.0))
                    .border_1()
                    .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
                    .child(Tree::new(
                        &self.tree_state,
                        |ix, entry, selected, _w, _cx| {
                            ListItem::new(("tree-item", ix))
                                .child(Label::new(entry.item().label.clone()).text_sm())
                                .selected(selected)
                        },
                    ))
                    .on_hover(self.hover_info(
                        &fi,
                        "Tree",
                        &[
                            ("bg", "list", t.list),
                            ("active", "list_active", t.list_active),
                            ("hover", "list_hover", t.list_hover),
                        ],
                        &[],
                        &[
                            ("indent", "per depth level"),
                            ("expand icon", "hardcoded ChevronRight"),
                        ],
                    )),
            )
            // Avatar & AvatarGroup
            .child(section("Avatar & AvatarGroup"))
            .child(
                div()
                    .id("tt-avatar")
                    .child(
                        h_flex()
                            .gap_6()
                            .items_center()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Avatar::new().name("Alice"))
                                    .child(Avatar::new().name("Bob"))
                                    .child(Avatar::new().name("Carol")),
                            )
                            .child(
                                AvatarGroup::new()
                                    .child(Avatar::new().name("D"))
                                    .child(Avatar::new().name("E"))
                                    .child(Avatar::new().name("F"))
                                    .child(Avatar::new().name("G"))
                                    .limit(3),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Avatar / AvatarGroup",
                        &[
                            ("fallback bg", "secondary", t.secondary),
                            (
                                "fallback text",
                                "secondary_foreground",
                                t.secondary_foreground,
                            ),
                            ("border", "background", t.background),
                        ],
                        &[],
                        &[
                            ("size", "configurable via Size enum"),
                            ("limit overflow", "+N indicator"),
                        ],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Feedback
    // -----------------------------------------------------------------------
    fn render_feedback_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
                        "Spinner",
                        &[],
                        &[],
                        &[
                            ("animation speed", "hardcoded"),
                            ("size", "hardcoded per Size enum"),
                        ],
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
                    .on_hover(self.hover_info(
                        &fi,
                        "Skeleton",
                        &[("bg", "skeleton", t.skeleton)],
                        &[],
                        &[("animation", "hardcoded pulse")],
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
                    .on_hover(self.hover_info(
                        &fi,
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
                            .child(
                                Badge::new()
                                    .count(5)
                                    .child(Button::new("badge-1").label("Messages")),
                            )
                            .child(
                                Badge::new()
                                    .count(99)
                                    .child(Button::new("badge-2").label("Notifications")),
                            )
                            .child(
                                Badge::new()
                                    .dot()
                                    .child(Button::new("badge-3").label("Updates")),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Badge",
                        &[("bg", "red", t.red), ("text", "background", t.background)],
                        &[],
                        &[("size", "hardcoded"), ("padding", "hardcoded")],
                    )),
            )
            // Tooltip
            .child(section("Tooltip"))
            .child(
                div()
                    .id("tt-tooltip")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                Button::new("tooltip-1")
                                    .label("Hover me")
                                    .tooltip("This is a tooltip"),
                            )
                            .child(
                                Button::new("tooltip-2")
                                    .label("With tooltip")
                                    .tooltip("Save file (Cmd+S)"),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Tooltip",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[("delay", "hardcoded"), ("position", "auto")],
                    )),
            )
            // Notification
            .child(section("Notification (push via WindowExt)"))
            .child(
                div()
                    .id("tt-notification")
                    .child(
                        h_flex()
                            .gap_3()
                            .child(
                                Button::new("notify-info")
                                    .label("Info")
                                    .on_click(cx.listener(|_this, _ev, window, cx| {
                                        window.push_notification(
                                            Notification::info("This is an info notification.")
                                                .title("Info")
                                                .autohide(true),
                                            cx,
                                        );
                                    })),
                            )
                            .child(Button::new("notify-success").label("Success").on_click(
                                cx.listener(|_this, _ev, window, cx| {
                                    window.push_notification(
                                        Notification::success("Operation completed.")
                                            .title("Success")
                                            .autohide(true),
                                        cx,
                                    );
                                }),
                            ))
                            .child(Button::new("notify-warning").label("Warning").on_click(
                                cx.listener(|_this, _ev, window, cx| {
                                    window.push_notification(
                                        Notification::warning("Careful with this action.")
                                            .title("Warning")
                                            .autohide(true),
                                        cx,
                                    );
                                }),
                            ))
                            .child(Button::new("notify-error").label("Error").on_click(
                                cx.listener(|_this, _ev, window, cx| {
                                    window.push_notification(
                                        Notification::error("Something went wrong.")
                                            .title("Error")
                                            .autohide(true),
                                        cx,
                                    );
                                }),
                            )),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Notification",
                        &[
                            ("bg", "popover", t.popover),
                            ("border", "border", t.border),
                            ("info icon", "info", t.info),
                            ("success icon", "success", t.success),
                            ("warning icon", "warning", t.warning),
                            ("error icon", "danger", t.danger),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[("animation", "slide in/out"), ("autohide", "configurable")],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Typography
    // -----------------------------------------------------------------------
    fn render_typography_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
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
                    .on_hover(self.hover_info(
                        &fi,
                        "Label",
                        &[
                            ("text", "foreground", t.foreground),
                            ("secondary", "muted_foreground", t.muted_foreground),
                            ("highlights", "blue", t.blue),
                        ],
                        &[
                            ("font", format!("font_family: {}", t.font_family)),
                            (
                                "size",
                                format!("font_size: {}px (renders)", t.font_size.as_f32()),
                            ),
                        ],
                        &[("font weights", "hardcoded")],
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
                    .on_hover(self.hover_info(
                        &fi,
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
            // Headings
            .child(section("Headings (H1–H6)"))
            .child(
                div()
                    .id("tt-headings")
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(rems(1.875))
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("H1 — Page Title"),
                            )
                            .child(
                                div()
                                    .text_size(rems(1.5))
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("H2 — Section"),
                            )
                            .child(
                                div()
                                    .text_size(rems(1.25))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("H3 — Subsection"),
                            )
                            .child(
                                div()
                                    .text_size(rems(1.125))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("H4 — Group"),
                            )
                            .child(
                                div()
                                    .text_size(rems(1.0))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child("H5 — Detail"),
                            )
                            .child(
                                div()
                                    .text_size(rems(0.875))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child("H6 — Fine Print"),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Headings",
                        &[("text", "foreground", t.foreground)],
                        &[("font", format!("font_family: {}", t.font_family))],
                        &[("sizes", "30px / 24px / 20px / 18px / 16px / 14px")],
                    )),
            )
            // Font weights
            .child(section("Font Weights (Thin → Black)"))
            .child(
                div()
                    .id("tt-weights")
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::THIN)
                                    .child("Thin (100)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::EXTRA_LIGHT)
                                    .child("Extra Light (200)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::LIGHT)
                                    .child("Light (300)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::NORMAL)
                                    .child("Normal (400)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child("Medium (500)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("Semibold (600)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("Bold (700)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::EXTRA_BOLD)
                                    .child("Extra Bold (800)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::BLACK)
                                    .child("Black (900)"),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Font Weights",
                        &[("text", "foreground", t.foreground)],
                        &[("font", format!("font_family: {}", t.font_family))],
                        &[("weights", "gpui::FontWeight constants")],
                    )),
            )
            // Font sizes
            .child(section("Font Sizes (XS → XL)"))
            .child(
                div()
                    .id("tt-sizes")
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("text_xs — Extra Small").text_xs())
                            .child(Label::new("text_sm — Small").text_sm())
                            .child(Label::new("text_base — Base (default)"))
                            .child(Label::new("text_lg — Large").text_lg())
                            .child(Label::new("text_xl — Extra Large").text_xl()),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Font Sizes",
                        &[("text", "foreground", t.foreground)],
                        &[("base", format!("font_size: {}px", t.font_size.as_f32()))],
                        &[
                            ("xs", "0.75rem"),
                            ("sm", "0.875rem"),
                            ("base", "1rem"),
                            ("lg", "1.125rem"),
                            ("xl", "1.25rem"),
                        ],
                    )),
            )
            // Text decorations
            .child(section("Text Decorations"))
            .child(
                div()
                    .id("tt-decorations")
                    .child(
                        v_flex()
                            .gap_1()
                            .child(div().font_weight(gpui::FontWeight::BOLD).child("Bold text"))
                            .child(
                                div()
                                    .underline()
                                    .text_decoration_1()
                                    .child("Underlined text"),
                            )
                            .child(
                                div()
                                    .line_through()
                                    .text_decoration_1()
                                    .child("Strikethrough text"),
                            )
                            .child(div().italic().child("Italic text")),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Text Decorations",
                        &[("text", "foreground", t.foreground)],
                        &[],
                        &[("styles", "bold / underline / strikethrough / italic")],
                    )),
            )
            // Kbd
            .child(section("Kbd (keyboard shortcuts)"))
            .child(
                div()
                    .id("tt-kbd")
                    .child(
                        h_flex().gap_4().items_center().children(
                            ["cmd-c", "cmd-v", "cmd-shift-p", "ctrl-z"]
                                .iter()
                                .filter_map(|k| Keystroke::parse(k).ok())
                                .map(Kbd::new),
                        ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Kbd",
                        &[
                            ("bg", "muted", t.muted),
                            ("text", "muted_foreground", t.muted_foreground),
                            ("border", "border", t.border),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[("padding", "hardcoded"), ("font", "monospace")],
                    )),
            )
            // Muted / mono text
            .child(section("Muted & Monospace Text"))
            .child(
                div()
                    .id("tt-muted-mono")
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(t.muted_foreground)
                                    .child("Muted text (secondary content)"),
                            )
                            .child(
                                div()
                                    .font_family(t.mono_font_family.clone())
                                    .child("Monospace text (code / technical content)"),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Muted & Mono",
                        &[
                            ("muted", "muted_foreground", t.muted_foreground),
                            ("text", "foreground", t.foreground),
                        ],
                        &[(
                            "mono font",
                            format!("mono_font_family: {}", t.mono_font_family),
                        )],
                        &[],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Layout
    // -----------------------------------------------------------------------
    fn render_layout_tab(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let collapsible_open = self.collapsible_open;
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
                                resizable_panel().size(px(250.0)).child(
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
                                        .child(
                                            Label::new("This panel fills remaining space")
                                                .text_sm(),
                                        ),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
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
                                resizable_panel().size(px(80.0)).child(
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
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
                        GroupBox::new().title("Contained Content").fill().child(
                            v_flex()
                                .gap_2()
                                .child(
                                    Label::new(
                                        "GroupBox can wrap any content as a visual container.",
                                    )
                                    .text_sm(),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(Button::new("gb-1").label("Action A"))
                                        .child(Button::new("gb-2").label("Action B").primary()),
                                ),
                        ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
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
                            .child(v_flex().gap_2().p_3().children((0..20).map(|i| {
                                Label::new(SharedString::from(format!(
                                    "Scrollable item #{} - demonstrates scrollbar theming",
                                    i + 1
                                )))
                                .text_sm()
                            }))),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Scrollbar",
                        &[
                            ("track", "scrollbar", t.scrollbar),
                            ("thumb", "scrollbar_thumb", t.scrollbar_thumb),
                            (
                                "thumb hover",
                                "scrollbar_thumb_hover",
                                t.scrollbar_thumb_hover,
                            ),
                            ("border", "border", t.border),
                        ],
                        &[
                            ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                            (
                                "show mode",
                                format!("scrollbar_show: {:?}", t.scrollbar_show),
                            ),
                        ],
                        &[
                            ("width", "16px hardcoded"),
                            ("min thumb length", "48px hardcoded"),
                        ],
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
                                item.title("What is native-theme?").open(true).child(
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
                                    Label::new(
                                        "17 built-in theme presets covering major OS styles.",
                                    )
                                    .text_sm(),
                                )
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
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
                                v_flex().p_3().child(
                                    Label::new("This content is shown when collapsible is open.")
                                        .text_sm(),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
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
                    .on_hover(self.hover_info(
                        &fi,
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
            // Breadcrumb (with tab navigation)
            .child(section("Breadcrumb (click to navigate tabs)"))
            .child(
                div()
                    .id("tt-breadcrumb")
                    .child(
                        Breadcrumb::new()
                            .child(BreadcrumbItem::new("Buttons").on_click(cx.listener(
                                |this, _ev, _w, _cx| {
                                    this.active_tab = TAB_BUTTONS;
                                },
                            )))
                            .child(BreadcrumbItem::new("Inputs").on_click(cx.listener(
                                |this, _ev, _w, _cx| {
                                    this.active_tab = TAB_INPUTS;
                                },
                            )))
                            .child(BreadcrumbItem::new("Data").on_click(cx.listener(
                                |this, _ev, _w, _cx| {
                                    this.active_tab = TAB_DATA;
                                },
                            )))
                            .child(BreadcrumbItem::new("Feedback").on_click(cx.listener(
                                |this, _ev, _w, _cx| {
                                    this.active_tab = TAB_FEEDBACK;
                                },
                            )))
                            .child(BreadcrumbItem::new("Layout")),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Breadcrumb",
                        &[
                            ("last item", "foreground", t.foreground),
                            (
                                "non-last + separators",
                                "muted_foreground",
                                t.muted_foreground,
                            ),
                        ],
                        &[],
                        &[
                            ("separator icon", "hardcoded ChevronRight"),
                            ("spacing", "hardcoded"),
                        ],
                    )),
            )
            // Form / Field
            .child(section("Form / Field (horizontal layout)"))
            .child(
                div()
                    .id("tt-form")
                    .child(
                        form::Form::horizontal()
                            .label_width(px(100.0))
                            .child(Field::new().label("Name").required(true).child(Input::new(
                                &cx.new(|cx| {
                                    let mut s = InputState::new(_window, cx);
                                    s.set_placeholder("Enter your name", _window, cx);
                                    s
                                }),
                            )))
                            .child(
                                Field::new()
                                    .label("Email")
                                    .description("We will never share your email.")
                                    .child(Input::new(&cx.new(|cx| {
                                        let mut s = InputState::new(_window, cx);
                                        s.set_placeholder("you@example.com", _window, cx);
                                        s
                                    }))),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Form / Field",
                        &[
                            ("label", "foreground", t.foreground),
                            ("description", "muted_foreground", t.muted_foreground),
                            ("required marker", "danger", t.danger),
                        ],
                        &[],
                        &[
                            ("layout", "horizontal/vertical"),
                            ("label width", "configurable"),
                        ],
                    )),
            )
            // Sidebar
            .child(section("Sidebar (mini navigation)"))
            .child(
                div()
                    .id("tt-sidebar")
                    .h(px(240.0))
                    .w(px(280.0))
                    .border_1()
                    .border_color(t.border)
                    .overflow_hidden()
                    .child(
                        Sidebar::left().collapsible(false).child(
                            SidebarMenu::new()
                                .child(
                                    SidebarMenuItem::new("Dashboard")
                                        .icon(IconName::LayoutDashboard)
                                        .active(true),
                                )
                                .child(SidebarMenuItem::new("Settings").icon(IconName::Settings))
                                .child(SidebarMenuItem::new("Inbox").icon(IconName::Inbox))
                                .child(SidebarMenuItem::new("Calendar").icon(IconName::Calendar)),
                        ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Sidebar",
                        &[
                            ("bg", "sidebar", t.sidebar),
                            ("text", "sidebar_foreground", t.sidebar_foreground),
                            ("accent", "sidebar_accent", t.sidebar_accent),
                            ("border", "sidebar_border", t.sidebar_border),
                        ],
                        &[],
                        &[
                            ("width", "255px default, 48px collapsed"),
                            ("children", "must impl Collapsible + IntoElement"),
                        ],
                    )),
            )
            // Settings
            .child(section("Settings (page with field types)"))
            .child(
                div()
                    .id("tt-settings")
                    .h(px(320.0))
                    .w_full()
                    .border_1()
                    .border_color(t.border)
                    .overflow_y_scroll()
                    .child(
                        Settings::new("settings-demo")
                            .sidebar_width(px(140.0))
                            .page(
                                SettingPage::new("Appearance")
                                    .description("Customize the look and feel")
                                    .default_open(true)
                                    .group(
                                        SettingGroup::new()
                                            .title("Theme")
                                            .item(
                                                SettingItem::new(
                                                    "Dark Mode",
                                                    SettingField::switch(
                                                        |_cx| false,
                                                        |_val, _cx| {},
                                                    ),
                                                )
                                                .description("Toggle dark appearance"),
                                            )
                                            .item(SettingItem::new(
                                                "Accent Color",
                                                SettingField::dropdown(
                                                    vec![
                                                        ("blue".into(), "Blue".into()),
                                                        ("green".into(), "Green".into()),
                                                        ("red".into(), "Red".into()),
                                                    ],
                                                    |_cx| "blue".into(),
                                                    |_val, _cx| {},
                                                ),
                                            )),
                                    )
                                    .group(
                                        SettingGroup::new()
                                            .title("Editor")
                                            .item(SettingItem::new(
                                                "Font Size",
                                                SettingField::input(
                                                    |_cx| "14".into(),
                                                    |_val, _cx| {},
                                                ),
                                            ))
                                            .item(SettingItem::new(
                                                "Word Wrap",
                                                SettingField::checkbox(|_cx| true, |_val, _cx| {}),
                                            )),
                                    ),
                            )
                            .page(
                                SettingPage::new("Keyboard")
                                    .description("Keyboard shortcuts and input")
                                    .group(SettingGroup::new().title("Shortcuts").item(
                                        SettingItem::new(
                                            "Vim Mode",
                                            SettingField::switch(|_cx| false, |_val, _cx| {}),
                                        ),
                                    )),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Settings",
                        &[
                            ("bg", "background", t.background),
                            ("sidebar", "sidebar", t.sidebar),
                            ("group", "group_box", t.group_box),
                            ("border", "border", t.border),
                        ],
                        &[],
                        &[
                            ("fields", "switch/checkbox/input/dropdown/number_input"),
                            ("layout", "sidebar + pages"),
                        ],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Overlays
    // -----------------------------------------------------------------------
    fn render_overlays_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // AppMenuBar
            .child(section("AppMenuBar (File / Edit / View / Help)"))
            .child(
                div()
                    .id("tt-app-menu-bar")
                    .w_full()
                    .border_1()
                    .border_color(t.border)
                    .child(self.app_menu_bar.clone())
                    .on_hover(self.hover_info(
                        &fi,
                        "AppMenuBar",
                        &[
                            ("bg", "tab_bar", t.tab_bar),
                            ("text", "foreground", t.foreground),
                            ("hover", "secondary_hover", t.secondary_hover),
                            ("menu bg", "popover", t.popover),
                        ],
                        &[],
                        &[
                            ("source", "cx.set_menus(Vec<Menu>)"),
                            ("reads", "cx.get_menus()"),
                        ],
                    )),
            )
            // Dialog
            .child(section("Dialog (confirm)"))
            .child(
                div()
                    .id("tt-dialog")
                    .child(
                        Button::new("open-dialog")
                            .label("Open Dialog")
                            .on_click(cx.listener(|_this, _ev, window, cx| {
                                window.open_dialog(cx, |dialog, _w, _cx| {
                                    dialog.title("Confirm Action").confirm().width(px(400.0))
                                });
                            })),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Dialog",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                            ("overlay", "overlay", t.overlay),
                            ("border", "border", t.border),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[("animation", "hardcoded scale+fade")],
                    )),
            )
            // Sheet
            .child(section("Sheet (slide-in panel)"))
            .child(
                div()
                    .id("tt-sheet")
                    .child(
                        h_flex()
                            .gap_3()
                            .child(
                                Button::new("open-sheet-right")
                                    .label("Open Sheet (Right)")
                                    .on_click(cx.listener(|_this, _ev, window, cx| {
                                        window.open_sheet(cx, |sheet, _w, _cx| {
                                            sheet.title("Sheet Panel").size(px(320.0))
                                        });
                                    })),
                            )
                            .child(
                                Button::new("open-sheet-bottom")
                                    .label("Open Sheet (Bottom)")
                                    .on_click(cx.listener(|_this, _ev, window, cx| {
                                        window.open_sheet_at(
                                            Placement::Bottom,
                                            cx,
                                            |sheet, _w, _cx| {
                                                sheet.title("Bottom Sheet").size(px(200.0))
                                            },
                                        );
                                    })),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Sheet",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                            ("overlay", "overlay", t.overlay),
                            ("border", "border", t.border),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("animation", "slide in/out"),
                            ("placement", "Right / Bottom / Left / Top"),
                        ],
                    )),
            )
            // Popover
            .child(section("Popover"))
            .child(
                div()
                    .id("tt-popover")
                    .child(
                        Popover::new("popover-1")
                            .trigger(Button::new("popover-trigger").label("Click for Popover"))
                            .content(|_state, _w, cx| {
                                v_flex()
                                    .p_4()
                                    .gap_2()
                                    .w(px(200.0))
                                    .child(Label::new("Popover Content").font_semibold())
                                    .child(
                                        Label::new("This is a popover panel.")
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground),
                                    )
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Popover",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                            ("border", "border", t.border),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("trigger", "any Selectable element"),
                            ("anchor", "configurable Corner"),
                        ],
                    )),
            )
            // ContextMenu
            .child(section("ContextMenu (right-click the area below)"))
            .child(
                div()
                    .id("tt-context-menu")
                    .child(
                        div()
                            .id("ctx-menu-area")
                            .p_6()
                            .w_full()
                            .rounded(px(6.0))
                            .border_1()
                            .border_color(t.border)
                            .bg(t.secondary)
                            .child(
                                Label::new("Right-click anywhere in this area")
                                    .text_sm()
                                    .text_color(t.muted_foreground),
                            )
                            .context_menu(|menu, _w, _cx| {
                                menu.menu("Cut", Box::new(gpui::NoAction))
                                    .menu("Copy", Box::new(gpui::NoAction))
                                    .menu("Paste", Box::new(gpui::NoAction))
                                    .separator()
                                    .menu("Select All", Box::new(gpui::NoAction))
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "ContextMenu",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                            ("hover", "list_hover", t.list_hover),
                            ("border", "border", t.border),
                        ],
                        &[],
                        &[
                            ("trigger", "right-click (MouseButton::Right)"),
                            ("trait", "ContextMenuExt on any ParentElement+Styled"),
                        ],
                    )),
            )
            // DropdownMenu
            .child(section("DropdownMenu"))
            .child(
                div()
                    .id("tt-menu")
                    .child(
                        DropdownButton::new("menu-demo")
                            .button(Button::new("menu-trigger").label("Click for Menu"))
                            .dropdown_menu(|menu, _w, _cx| {
                                menu.menu("Cut", Box::new(gpui::NoAction))
                                    .menu("Copy", Box::new(gpui::NoAction))
                                    .menu("Paste", Box::new(gpui::NoAction))
                                    .separator()
                                    .menu("Select All", Box::new(gpui::NoAction))
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "PopupMenu / DropdownMenu",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                            ("hover", "list_hover", t.list_hover),
                            ("border", "border", t.border),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("separator", "horizontal line"),
                            ("shortcut", "optional Kbd"),
                        ],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Charts
    // -----------------------------------------------------------------------
    fn render_charts_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();

        // Sample data structs for charts
        #[derive(Clone)]
        struct MonthData {
            month: SharedString,
            value: f64,
        }

        #[derive(Clone)]
        struct OhlcData {
            date: SharedString,
            open: f64,
            high: f64,
            low: f64,
            close: f64,
        }

        #[derive(Clone)]
        struct PieSlice {
            _label: SharedString,
            amount: f32,
            color: Hsla,
        }

        let months: Vec<MonthData> = vec![
            MonthData {
                month: "Jan".into(),
                value: 40.0,
            },
            MonthData {
                month: "Feb".into(),
                value: 65.0,
            },
            MonthData {
                month: "Mar".into(),
                value: 55.0,
            },
            MonthData {
                month: "Apr".into(),
                value: 80.0,
            },
            MonthData {
                month: "May".into(),
                value: 72.0,
            },
            MonthData {
                month: "Jun".into(),
                value: 90.0,
            },
        ];

        let months2: Vec<MonthData> = vec![
            MonthData {
                month: "Jan".into(),
                value: 30.0,
            },
            MonthData {
                month: "Feb".into(),
                value: 50.0,
            },
            MonthData {
                month: "Mar".into(),
                value: 45.0,
            },
            MonthData {
                month: "Apr".into(),
                value: 70.0,
            },
            MonthData {
                month: "May".into(),
                value: 60.0,
            },
            MonthData {
                month: "Jun".into(),
                value: 85.0,
            },
        ];

        let ohlc_data = vec![
            OhlcData {
                date: "Mon".into(),
                open: 100.0,
                high: 115.0,
                low: 95.0,
                close: 110.0,
            },
            OhlcData {
                date: "Tue".into(),
                open: 110.0,
                high: 120.0,
                low: 105.0,
                close: 108.0,
            },
            OhlcData {
                date: "Wed".into(),
                open: 108.0,
                high: 118.0,
                low: 100.0,
                close: 115.0,
            },
            OhlcData {
                date: "Thu".into(),
                open: 115.0,
                high: 125.0,
                low: 110.0,
                close: 112.0,
            },
            OhlcData {
                date: "Fri".into(),
                open: 112.0,
                high: 122.0,
                low: 108.0,
                close: 120.0,
            },
        ];

        let pie_data = vec![
            PieSlice {
                _label: "Desktop".into(),
                amount: 55.0,
                color: t.chart_1,
            },
            PieSlice {
                _label: "Mobile".into(),
                amount: 30.0,
                color: t.chart_2,
            },
            PieSlice {
                _label: "Tablet".into(),
                amount: 15.0,
                color: t.chart_3,
            },
        ];

        let bar_fill = t.chart_1;
        let line_stroke = t.chart_2;
        let area_stroke = t.chart_3;
        let area_fill = t.chart_3.opacity(0.3);

        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Bar chart
            .child(section("BarChart"))
            .child(
                div()
                    .id("tt-bar-chart")
                    .h(px(220.0))
                    .w_full()
                    .child(
                        BarChart::new(months.clone())
                            .x(|d: &MonthData| d.month.clone())
                            .y(|d: &MonthData| d.value)
                            .fill(move |_: &MonthData| bar_fill),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "BarChart",
                        &[
                            ("fill", "chart_1", t.chart_1),
                            ("axis", "muted_foreground", t.muted_foreground),
                            ("grid", "border", t.border),
                        ],
                        &[],
                        &[
                            ("bar width", "auto-scaled"),
                            ("tick_margin", "configurable"),
                        ],
                    )),
            )
            // Line chart
            .child(section("LineChart"))
            .child(
                div()
                    .id("tt-line-chart")
                    .h(px(220.0))
                    .w_full()
                    .child(
                        LineChart::new(months.clone())
                            .x(|d: &MonthData| d.month.clone())
                            .y(|d: &MonthData| d.value)
                            .stroke(line_stroke)
                            .dot(),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "LineChart",
                        &[
                            ("stroke", "chart_2", t.chart_2),
                            ("axis", "muted_foreground", t.muted_foreground),
                            ("grid", "border", t.border),
                        ],
                        &[],
                        &[("style", "natural/linear/step_after"), ("dot", "optional")],
                    )),
            )
            // Area chart
            .child(section("AreaChart"))
            .child(
                div()
                    .id("tt-area-chart")
                    .h(px(220.0))
                    .w_full()
                    .child(
                        AreaChart::new(months2)
                            .x(|d: &MonthData| d.month.clone())
                            .y(|d: &MonthData| d.value)
                            .stroke(area_stroke)
                            .fill(area_fill),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "AreaChart",
                        &[
                            ("stroke", "chart_3", t.chart_3),
                            ("fill", "chart_3 (0.3 opacity)", t.chart_3),
                            ("axis", "muted_foreground", t.muted_foreground),
                        ],
                        &[],
                        &[("multiple series", "chain .y()/.stroke()/.fill()")],
                    )),
            )
            // Pie chart
            .child(section("PieChart (donut)"))
            .child(
                div()
                    .id("tt-pie-chart")
                    .h(px(250.0))
                    .w(px(250.0))
                    .child(
                        PieChart::new(pie_data)
                            .value(|d: &PieSlice| d.amount)
                            .color(|d: &PieSlice| d.color)
                            .inner_radius(40.0)
                            .outer_radius(100.0)
                            .pad_angle(0.03),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "PieChart",
                        &[
                            ("slice 1", "chart_1", t.chart_1),
                            ("slice 2", "chart_2", t.chart_2),
                            ("slice 3", "chart_3", t.chart_3),
                        ],
                        &[],
                        &[
                            ("inner_radius", "0=filled, >0=donut"),
                            ("pad_angle", "gap between slices"),
                        ],
                    )),
            )
            // Candlestick chart
            .child(section("CandlestickChart"))
            .child(
                div()
                    .id("tt-candlestick-chart")
                    .h(px(220.0))
                    .w_full()
                    .child(
                        CandlestickChart::new(ohlc_data)
                            .x(|d: &OhlcData| d.date.clone())
                            .open(|d: &OhlcData| d.open)
                            .high(|d: &OhlcData| d.high)
                            .low(|d: &OhlcData| d.low)
                            .close(|d: &OhlcData| d.close),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "CandlestickChart",
                        &[
                            ("bullish", "bullish", t.bullish),
                            ("bearish", "bearish", t.bearish),
                            ("axis", "muted_foreground", t.muted_foreground),
                        ],
                        &[],
                        &[
                            ("body_width_ratio", "default 0.8"),
                            ("auto-colored", "green=up, red=down"),
                        ],
                    )),
            )
    }

    /// Build the "Animated Icons" section for the Icons tab.
    fn render_animated_icons_section(&self) -> impl IntoElement {
        let mut cards: Vec<AnyElement> = Vec::new();

        if self.reduced_motion {
            // Show static first frames with reduced-motion label
            for (set_name, source, anim_type) in &self.animated_static_sources {
                let label_text: SharedString =
                    format!("{} - {} (reduced motion)", set_name, anim_type).into();
                cards.push(
                    v_flex()
                        .items_center()
                        .gap_2()
                        .p_4()
                        .rounded_md()
                        .border_1()
                        .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
                        .child(gpui::img(source.clone()).size(px(32.)))
                        .child(Label::new(label_text).text_xs())
                        .into_any_element(),
                );
            }
        } else {
            // Frame-based animations
            for (i, (set_name, frames)) in self.animated_frame_sources.iter().enumerate() {
                let frame_idx = self.animated_frame_indices.get(i).copied().unwrap_or(0);
                let total = frames.len();
                let duration = self.animated_frame_durations.get(i).copied().unwrap_or(83);
                if let Some(source) = frames.get(frame_idx) {
                    let label_text: SharedString =
                        format!("{} - Frames: {} ({}ms)", set_name, total, duration).into();
                    cards.push(
                        v_flex()
                            .items_center()
                            .gap_2()
                            .p_4()
                            .rounded_md()
                            .border_1()
                            .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
                            .child(gpui::img(source.clone()).size(px(32.)))
                            .child(Label::new(label_text).text_xs())
                            .into_any_element(),
                    );
                }
            }

            // Transform (spin) animations -- shown with opacity pulse since gpui
            // Div lacks with_transformation (only Svg has it). In real usage,
            // callers use with_spin_animation() on an Svg element.
            for (set_name, source, duration_ms) in &self.animated_spin_sources {
                let label_text: SharedString =
                    format!("{} - Spin ({}ms)", set_name, duration_ms).into();
                let spin_id = SharedString::from(format!("spinner-{}", set_name));
                let dur = Duration::from_millis(*duration_ms as u64);
                cards.push(
                    v_flex()
                        .items_center()
                        .gap_2()
                        .p_4()
                        .rounded_md()
                        .border_1()
                        .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
                        .child(
                            div()
                                .size(px(32.))
                                .child(gpui::img(source.clone()).size(px(32.)))
                                .with_animation(
                                    spin_id,
                                    Animation::new(dur).repeat(),
                                    |el: gpui::Div, delta| {
                                        // Pulse opacity 0.3..1.0 to indicate animation
                                        let opacity = 0.3 + 0.7 * (1.0 - (delta * 2.0 - 1.0).abs());
                                        el.opacity(opacity)
                                    },
                                ),
                        )
                        .child(Label::new(label_text).text_xs())
                        .into_any_element(),
                );
            }
        }

        let has_items = !cards.is_empty();

        let mut section_el = v_flex().gap_2();
        section_el = section_el.child(section("Animated Icons"));

        if self.reduced_motion {
            section_el = section_el.child(
                Label::new("(prefers-reduced-motion: showing static frames)")
                    .text_xs()
                    .text_color(gpui::hsla(0.0, 0.0, 0.5, 1.0)),
            );
        }

        if has_items {
            section_el = section_el.child(h_flex().gap_6().flex_wrap().children(cards));
        } else {
            section_el = section_el.child(
                Label::new("No animated icons available for current icon sets")
                    .text_xs()
                    .text_color(gpui::hsla(0.0, 0.0, 0.5, 1.0)),
            );
        }

        section_el
    }

    // -----------------------------------------------------------------------
    // Tab: Icons
    // -----------------------------------------------------------------------
    fn render_icons_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);

        // --- Native Theme Icons section ---
        let fallback_label = if !is_native_icon_set(&self.icon_set_name) {
            " (fallback)"
        } else {
            ""
        };
        let loaded_count = self
            .loaded_icons
            .iter()
            .filter(|(_, d, _)| d.is_some())
            .count();
        let system_count = self
            .loaded_icons
            .iter()
            .filter(|(_, _, s)| *s == IconSource::System)
            .count();
        let fallback_count = self
            .loaded_icons
            .iter()
            .filter(|(_, _, s)| *s == IconSource::Fallback)
            .count();
        let is_system_set = matches!(
            self.icon_set_name.as_str(),
            "freedesktop" | "sf-symbols" | "segoe-fluent"
        );
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

        // Use the stored IconSet for tooltip icon-name lookups
        let icon_set_enum = self.icon_set_enum;
        let icon_set_label = self.icon_set_name.clone();

        // Build icon cells for loaded native icons
        let native_icon_cells: Vec<_> = self
            .loaded_icons
            .iter()
            .enumerate()
            .map(|(i, (role, _data, source))| {
                let role_name: SharedString = format!("{:?}", role).into();
                let cell_id =
                    SharedString::from(format!("native-icon-{}-{}", self.icon_set_name, i));

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
                } else if let Some(img_source) =
                    self.loaded_icon_sources.get(i).and_then(|s| s.clone())
                {
                    div().child(gpui::img(img_source).w(px(20.0)).h(px(20.0)))
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
                    .and_then(|set| native_icon_name(*role, set))
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
                } else if let Some(img_source) =
                    self.gpui_icon_sources.get(i).and_then(|s| s.clone())
                {
                    div().child(gpui::img(img_source).w(px(20.0)).h(px(20.0)))
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
                                lines.push_str(&format!("\nOrigin: Bundled {} SVG", tooltip_set,));
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

        let mapped_count = self
            .gpui_icons
            .iter()
            .filter(|(_, _, r, _, _)| r.is_some())
            .count();

        v_flex()
            .gap_3()
            .p_4()
            // Animated Icons section
            .child(self.render_animated_icons_section())
            .child(Divider::horizontal())
            // Native Theme Icons section
            .child(section(native_section_title))
            .child(
                div()
                    .id("native-icons-grid")
                    .child(div().flex().flex_wrap().gap_2().children(native_icon_cells)),
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
                    .child(div().flex().flex_wrap().gap_2().children(gpui_icon_cells))
                    .on_hover(self.hover_info(
                        &fi,
                        "Icon",
                        &[],
                        &[],
                        &[
                            (
                                "color",
                                "inherited from parent foreground, customizable via text_color()",
                            ),
                            ("SVG shapes", "86 built-in Lucide icons from gpui-component"),
                        ],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Theme Map
    // -----------------------------------------------------------------------
    fn render_theme_map_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let _fi = format_font_info(&self.original_font, &self.original_mono_font);
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
                    .child(color_swatch(
                        "table_head_foreground",
                        t.table_head_foreground,
                    ))
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
                    .child(color_swatch(
                        "tab_active_foreground",
                        t.tab_active_foreground,
                    ))
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
                    .child(color_swatch(
                        "scrollbar_thumb_hover",
                        t.scrollbar_thumb_hover,
                    )),
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
                    .child(color_swatch(
                        "description_list_label",
                        t.description_list_label,
                    ))
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
        // Apply deferred system theme change (set by the watcher polling task).
        // Done here because apply_theme_by_name needs window access.
        if self.pending_system_theme_change {
            self.pending_system_theme_change = false;
            native_theme::detect::invalidate_caches();
            self.is_dark = AppColorMode::System.is_dark();
            let name = self.current_theme_name.clone();
            self.apply_theme_by_name(&name, window, cx);
            // Rebuild the color mode dropdown items and selected value to
            // reflect the new state (e.g. "System (Dark)" → "System (Light)").
            let labels: Vec<SharedString> = [
                AppColorMode::System,
                AppColorMode::Light,
                AppColorMode::Dark,
            ]
            .iter()
            .map(|m| SharedString::from(m.label()))
            .collect();
            let selected: SharedString = self.color_mode.label().into();
            let delegate = SearchableVec::new(labels);
            self.dark_mode_select.update(cx, |select, cx| {
                select.set_items(delegate, window, cx);
                select.set_selected_value(&selected, window, cx);
            });
        }

        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let theme = cx.theme().clone();

        // Ensure icon image caches match the current foreground color
        if theme.foreground != self.icon_cache_fg {
            self.rebuild_icon_caches(theme.foreground);
        }

        let active_tab = self.active_tab;

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
                    .child(
                        Label::new("Theme Selector")
                            .text_size(px(13.0))
                            .font_semibold(),
                    )
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
        let mut content = v_flex().flex_1().h_full().overflow_hidden();

        // Error banner (if any)
        if let Some(ref msg) = self.error_message {
            content = content.child(
                div()
                    .id("error-banner")
                    .px_4()
                    .py_2()
                    .bg(gpui::hsla(0.0, 0.7, 0.2, 1.0))
                    .text_color(gpui::hsla(0.0, 0.0, 1.0, 1.0))
                    .child(Label::new(msg.clone()).text_size(px(12.0))),
            );
        }

        // Tab bar
        let content = content
            .child(
                v_flex().px_4().pt_3().pb_2().child(
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
                                .child("Overlays")
                                .child("Charts")
                                .child("Icons")
                                .child("Theme Map")
                                .selected_index(active_tab)
                                .on_click(cx.listener(|this, ix: &usize, _window, _cx| {
                                    this.active_tab = *ix;
                                })),
                        )
                        .on_hover(self.hover_info(
                            &fi,
                            "TabBar",
                            &[
                                ("bg", "tab", theme.tab),
                                ("active bg", "tab_active", theme.tab_active),
                                (
                                    "active text",
                                    "tab_active_foreground",
                                    theme.tab_active_foreground,
                                ),
                                ("bar bg", "tab_bar", theme.tab_bar),
                                ("text", "tab_foreground", theme.tab_foreground),
                                ("border", "border", theme.border),
                                ("hover", "secondary_hover", theme.secondary_hover),
                            ],
                            &[(
                                "border-radius",
                                format!("radius: {}px", theme.radius.as_f32()),
                            )],
                            &[("padding", "set per Size enum")],
                        )),
                ),
            )
            // Content with scrollbar
            .child(
                div()
                    .id("content-scroll-outer")
                    .flex_1()
                    .overflow_y_scrollbar()
                    .child(match active_tab {
                        TAB_BUTTONS => self.render_buttons_tab(window, cx).into_any_element(),
                        TAB_INPUTS => self.render_inputs_tab(window, cx).into_any_element(),
                        TAB_DATA => self.render_data_tab(window, cx).into_any_element(),
                        TAB_FEEDBACK => self.render_feedback_tab(window, cx).into_any_element(),
                        TAB_TYPOGRAPHY => self.render_typography_tab(cx).into_any_element(),
                        TAB_LAYOUT => self.render_layout_tab(window, cx).into_any_element(),
                        TAB_OVERLAYS => self.render_overlays_tab(window, cx).into_any_element(),
                        TAB_CHARTS => self.render_charts_tab(cx).into_any_element(),
                        TAB_ICONS => self.render_icons_tab(cx).into_any_element(),
                        TAB_THEME_MAP => self.render_theme_map_tab(cx).into_any_element(),
                        _ => self.render_buttons_tab(window, cx).into_any_element(),
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
// CLI argument parsing
// ---------------------------------------------------------------------------

/// Optional CLI arguments for launching the showcase in a specific state.
///
/// Parsed from `std::env::args()` — no external crate dependency.
/// When no arguments are provided the showcase behaves identically to before.
#[derive(Default)]
struct CliArgs {
    theme: Option<String>,
    variant: Option<String>,
    tab: Option<String>,
    icon_set: Option<String>,
    icon_theme: Option<String>,
    screenshot: Option<String>,
}

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
                "--icon-theme" => {
                    i += 1;
                    if i < argv.len() {
                        args.icon_theme = Some(argv[i].clone());
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

    /// Map a tab name string to the corresponding tab index constant.
    fn tab_index(name: &str) -> Option<usize> {
        match name {
            "buttons" => Some(TAB_BUTTONS),
            "inputs" | "text-inputs" => Some(TAB_INPUTS),
            "data" => Some(TAB_DATA),
            "feedback" => Some(TAB_FEEDBACK),
            "typography" => Some(TAB_TYPOGRAPHY),
            "layout" => Some(TAB_LAYOUT),
            "overlays" => Some(TAB_OVERLAYS),
            "charts" => Some(TAB_CHARTS),
            "icons" => Some(TAB_ICONS),
            "theme-map" => Some(TAB_THEME_MAP),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Self-capture screenshot (macOS only)
// ---------------------------------------------------------------------------

/// Get the NSWindow pointer for the main window via NSApplication.
#[cfg(target_os = "macos")]
fn get_main_window_ptr() -> Option<*mut objc2::runtime::AnyObject> {
    let ns_app_class = objc2::runtime::AnyClass::get(c"NSApplication")?;
    unsafe {
        let ns_app: *mut objc2::runtime::AnyObject =
            objc2::msg_send![ns_app_class, sharedApplication];
        // Try mainWindow first, then keyWindow, then first element of the
        // windows array.  On CI runners the second GUI process launched in
        // sequence may not get mainWindow promoted even after
        // cx.activate(true) and a 1.5 s delay.
        let main: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, mainWindow];
        if !main.is_null() {
            return Some(main);
        }
        let key: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, keyWindow];
        if !key.is_null() {
            return Some(key);
        }
        let windows: *mut objc2::runtime::AnyObject = objc2::msg_send![ns_app, windows];
        let count: usize = objc2::msg_send![windows, count];
        if count > 0 {
            let first: *mut objc2::runtime::AnyObject =
                objc2::msg_send![windows, objectAtIndex: 0usize];
            if !first.is_null() {
                return Some(first);
            }
        }
        None
    }
}

/// Force the Metal drawable to update by nudging the window content size.
///
/// gpui initialises the Metal drawable at logical-pixel dimensions, ignoring
/// the Retina backing scale factor.  The correct device-pixel size is only
/// set inside the `setFrameSize:` callback, which early-returns when the
/// old size equals the new size.  A 1 px nudge-and-restore forces two real
/// resize events so `update_drawable_size` runs with the correct scale.
///
/// IMPORTANT: calls `[NSWindow setContentSize:]` directly via ObjC because
/// gpui's `window.resize()` spawns an async task that may not execute before
/// the screenshot capture.  Must be called **outside** `cx.update_window` to
/// avoid deadlocking the window-state mutex (since `setFrameSize:` acquires
/// it internally).
/// Minimal Core Graphics types for ObjC interop.
/// Based on objc2's encode_core_graphics example.
#[cfg(target_os = "macos")]
mod cg_types {
    use objc2::encode::{Encode, Encoding};

    #[repr(C)]
    pub struct CGPoint {
        pub x: f64,
        pub y: f64,
    }
    // SAFETY: repr(C) struct with correct encoding.
    unsafe impl Encode for CGPoint {
        const ENCODING: Encoding = Encoding::Struct("CGPoint", &[f64::ENCODING, f64::ENCODING]);
    }

    #[repr(C)]
    pub struct CGSize {
        pub width: f64,
        pub height: f64,
    }
    // SAFETY: repr(C) struct with correct encoding.
    unsafe impl Encode for CGSize {
        const ENCODING: Encoding = Encoding::Struct("CGSize", &[f64::ENCODING, f64::ENCODING]);
    }

    #[repr(C)]
    pub struct CGRect {
        pub origin: CGPoint,
        pub size: CGSize,
    }
    // SAFETY: repr(C) struct with correct encoding.
    unsafe impl Encode for CGRect {
        const ENCODING: Encoding =
            Encoding::Struct("CGRect", &[CGPoint::ENCODING, CGSize::ENCODING]);
    }
}

#[cfg(target_os = "macos")]
fn nudge_content_size(delta_w: f64, delta_h: f64) {
    if let Some(main_window) = get_main_window_ptr() {
        unsafe {
            let content_view: *mut objc2::runtime::AnyObject =
                objc2::msg_send![main_window, contentView];
            let frame: cg_types::CGRect = objc2::msg_send![content_view, frame];
            let new_size = cg_types::CGSize {
                width: frame.size.width + delta_w,
                height: frame.size.height + delta_h,
            };
            let _: () = objc2::msg_send![main_window, setContentSize: new_size];
        }
    }
}

/// Capture the gpui window including decorations using macOS `screencapture -l`.
///
/// Gets the CGWindowID via NSApplication -> mainWindow -> windowNumber, then
/// shells out to `screencapture -l <id> -o <path>`. This avoids the deprecated
/// `CGWindowListCreateImage` API and produces a PNG with full title bar and
/// window chrome.
#[cfg(target_os = "macos")]
fn capture_own_window_macos(_window: &mut Window, output_path: &str) -> bool {
    let Some(window_ptr) = get_main_window_ptr() else {
        eprintln!("No main window found");
        return false;
    };
    let window_id: i64 = unsafe { objc2::msg_send![window_ptr, windowNumber] };
    let status = std::process::Command::new("screencapture")
        .args(["-l", &format!("{}", window_id), "-o", output_path])
        .status();
    match status {
        Ok(s) if s.success() => {
            eprintln!("Screenshot saved to {output_path}");
            true
        }
        Ok(s) => {
            eprintln!("screencapture exited with {s}");
            false
        }
        Err(e) => {
            eprintln!("Failed to run screencapture: {e}");
            false
        }
    }
}

// ---------------------------------------------------------------------------
// Self-capture screenshot (Windows only)
// ---------------------------------------------------------------------------

/// Capture the gpui window including decorations using Windows BitBlt.
///
/// Uses `FindWindowW` with the known window title to locate the correct HWND
/// (more reliable than `GetForegroundWindow` which may return a console or
/// other window on CI), then `BitBlt` + `GetDIBits` to extract pixel data.
#[cfg(target_os = "windows")]
fn capture_own_window_windows(_window: &mut Window, output_path: &str) -> bool {
    use windows::Win32::Foundation::*;
    use windows::Win32::Graphics::Dwm::*;
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::PCWSTR;

    unsafe {
        let title = format!(
            "Native Theme \u{2013} GPUI Showcase, v{}",
            env!("CARGO_PKG_VERSION")
        );
        let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = match FindWindowW(None, PCWSTR(title_w.as_ptr())) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("FindWindowW failed: {e}");
                return false;
            }
        };

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
            if let Err(e) = GetWindowRect(hwnd, &mut rect) {
                eprintln!("GetWindowRect failed: {e}");
                return false;
            }
        }

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        if width <= 0 || height <= 0 {
            eprintln!("Invalid window dimensions: {width}x{height}");
            return false;
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
            eprintln!("BitBlt failed");
            return false;
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
            eprintln!("GetDIBits returned 0 lines");
            return false;
        }

        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2); // BGRA -> RGBA
            chunk[3] = 255; // force opaque
        }

        match image::save_buffer(
            output_path,
            &pixels,
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        ) {
            Ok(()) => {
                eprintln!("Screenshot saved to {output_path}");
                true
            }
            Err(e) => {
                eprintln!("Failed to save PNG: {e}");
                false
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
fn main() {
    let cli_args = CliArgs::parse();

    Application::new()
        .with_assets(gpui_component_assets::Assets)
        .run(move |cx: &mut App| {
            gpui_component::init(cx);

            // Apply CLI variant override before window opens so the initial
            // theme is resolved with the correct light/dark setting.
            let variant_override = cli_args.variant.as_deref().map(|v| v == "dark");

            let bounds = Bounds::centered(None, size(px(1100.), px(850.)), cx);
            let window_handle = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let showcase = cx.new(|cx| {
                        let mut s = Showcase::new(window, cx);

                        // Override color mode if --variant was specified
                        if let Some(is_dark) = variant_override {
                            let mode = if is_dark {
                                AppColorMode::Dark
                            } else {
                                AppColorMode::Light
                            };
                            s.color_mode = mode;
                            s.is_dark = is_dark;
                            // Update the color mode selector dropdown
                            let label = SharedString::from(mode.label());
                            s.dark_mode_select.update(cx, |select, cx| {
                                select.set_selected_value(&label, window, cx);
                            });
                        }

                        // Override theme if --theme was specified
                        if let Some(ref theme_name) = cli_args.theme {
                            s.current_theme_name = theme_name.clone();
                            s.apply_theme_by_name(theme_name, window, cx);
                            // Update the theme selector dropdown to show the overridden theme
                            let display = SharedString::from(theme_name.clone());
                            s.theme_select.update(cx, |select, cx| {
                                select.set_selected_value(&display, window, cx);
                            });
                        }

                        // Override tab if --tab was specified
                        if let Some(ref tab_name) = cli_args.tab
                            && let Some(idx) = CliArgs::tab_index(tab_name)
                        {
                            s.active_tab = idx;
                        }

                        // Override icon theme if --icon-theme was specified
                        if let Some(ref theme_name) = cli_args.icon_theme {
                            s.icon_theme_override = Some(theme_name.clone());
                        }

                        // Override icon set if --icon-set was specified
                        if let Some(ref set_name) = cli_args.icon_set {
                            // Map CLI set name to an IconSetChoice
                            s.icon_set_choice = match set_name.as_str() {
                                "material" => IconSetChoice::Material,
                                "lucide" => IconSetChoice::Lucide,
                                "freedesktop" => IconSetChoice::System,
                                _ => IconSetChoice::System,
                            };
                            let effective =
                                s.icon_set_choice.effective_icon_set(s.current_icon_set);
                            let default_theme =
                                s.icon_set_choice.freedesktop_theme().map(|t| t.to_string());
                            s.icon_set_name = effective.name().to_string();
                            s.icon_set_enum = Some(effective);
                            let cli_ref = s.icon_theme_override.as_deref();
                            let fc = s.original_font.color;
                            let fg_rgb = Some([fc.r, fc.g, fc.b]);
                            s.loaded_icons = load_all_icons(
                                effective,
                                default_theme.as_deref(),
                                cli_ref,
                                fg_rgb,
                            );
                            s.gpui_icons = load_gpui_icons(
                                Some(effective),
                                default_theme.as_deref(),
                                cli_ref,
                                fg_rgb,
                            );
                            let fg = cx.theme().foreground;
                            s.rebuild_icon_caches(fg);
                            s.rebuild_animation_caches();
                            s.start_animation_timer(cx);

                            // Update the icon theme selector dropdown
                            let icon_display: SharedString = s.icon_set_choice.to_string().into();
                            let mut icon_names = s.icon_set_dropdown_names();
                            // Add the override display name if not already in list
                            if !icon_names.contains(&icon_display) {
                                icon_names.push(icon_display.clone());
                            }
                            let new_delegate = SearchableVec::new(icon_names);
                            s.icon_set_select.update(cx, |select, cx| {
                                select.set_items(new_delegate, window, cx);
                                select.set_selected_value(&icon_display, window, cx);
                            });
                        }

                        s
                    });
                    cx.new(|cx| Root::new(showcase, window, cx))
                },
            );
            let Ok(window_handle) = window_handle else {
                eprintln!("Fatal: failed to open main application window");
                cx.quit();
                return;
            };
            window_handle
                .update(cx, |_, window, _| {
                    window.set_window_title(&format!(
                        "Native Theme – GPUI Showcase, v{}",
                        env!("CARGO_PKG_VERSION")
                    ));
                })
                .ok();

            // Force Metal drawable to adopt the Retina scale factor by
            // nudging the content size synchronously via ObjC.  Must happen
            // outside update() to avoid deadlocking the window-state mutex.
            #[cfg(target_os = "macos")]
            {
                nudge_content_size(-1.0, 0.0);
                nudge_content_size(1.0, 0.0);
            }
            cx.activate(true);

            // Schedule delayed self-capture if --screenshot was provided
            if let Some(screenshot_path) = cli_args.screenshot.as_ref() {
                #[cfg(target_os = "macos")]
                {
                    let path = screenshot_path.clone();
                    let any_handle = *window_handle;
                    cx.spawn(async move |cx| {
                        // Force Metal drawable to update on Retina displays.
                        // Calls [NSWindow setContentSize:] directly (synchronous)
                        // rather than gpui's window.resize() which is async and
                        // may not execute before the capture.
                        nudge_content_size(-1.0, 0.0);
                        Timer::after(Duration::from_millis(200)).await;
                        nudge_content_size(1.0, 0.0);
                        Timer::after(Duration::from_millis(1300)).await;
                        let captured = cx
                            .update_window(any_handle, |_view, window, _cx| {
                                capture_own_window_macos(window, &path)
                            })
                            .unwrap_or(false);
                        if !captured {
                            eprintln!("ERROR: screenshot capture failed for {path}");
                            std::process::exit(1);
                        }
                        let _ = cx.update(|cx| cx.quit());
                    })
                    .detach();
                }
                #[cfg(target_os = "windows")]
                {
                    let path = screenshot_path.clone();
                    let any_handle = *window_handle;
                    cx.spawn(async move |cx| {
                        Timer::after(Duration::from_millis(1500)).await;
                        let captured = cx
                            .update_window(any_handle, |_view, window, _cx| {
                                capture_own_window_windows(window, &path)
                            })
                            .unwrap_or(false);
                        if !captured {
                            eprintln!("ERROR: screenshot capture failed for {path}");
                            std::process::exit(1);
                        }
                        let _ = cx.update(|cx| cx.quit());
                    })
                    .detach();
                }
                #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                {
                    let _ = &screenshot_path;
                    eprintln!(
                        "Self-capture not supported on this platform. \
                         Use spectacle or generate_gpui_screenshots.sh instead."
                    );
                    // Continue running -- let the user capture manually
                }
            }
            let _ = &window_handle; // suppress unused warning when not used for capture
        });
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn main() {
    eprintln!("gpui showcase is not supported on this platform");
}
