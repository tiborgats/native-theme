//! What the pages share: sample content, tooltip and layout helpers, icon loading, and the list and table delegates.

use gpui::{
    App, Axis, Context, Div, Hsla, ImageSource, IntoElement, ParentElement, Pixels, SharedString,
    StyleRefinement, Styled, Task, Window, div, px,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, IndexPath, Sizable, Size, StyledExt,
    accordion::AccordionItem,
    attachment::AttachmentStatus,
    avatar::Avatar,
    bubble::{Bubble, BubbleVariant},
    group_box::GroupBox,
    h_flex,
    label::Label,
    list::{ListDelegate, ListItem, ListState},
    message::{Message, MessageAlignment, MessageContent},
    searchable_list::{SearchableListChange, SearchableListDelegate, SearchableListItem},
    table::{Column, TableDelegate, TableState},
};
use std::collections::HashMap;

#[cfg(target_os = "linux")]
use native_theme::detect::parse_linux_desktop;
use native_theme::icons::{
    FreedesktopLoader, IconSetChoice, LucideLoader, MaterialLoader, SegoeIconsLoader,
    SfSymbolsLoader, load_icon,
};
use native_theme::pipeline::platform_preset_name;
use native_theme::theme::{IconData, IconRole, IconSet};
#[cfg(target_os = "linux")]
use native_theme::theme::{icon_name as native_icon_name, system_icon_theme};
#[cfg(target_os = "linux")]
use native_theme_gpui::icons::freedesktop_name_for_gpui_icon;
use native_theme_gpui::icons::{lucide_name_for_gpui_icon, material_name_for_gpui_icon};
use native_theme_gpui::{ActiveNativeTheme, Native, geometry};

pub use crate::info::hsla_to_hex;

// ---------------------------------------------------------------------------
// Sample content (Carousel slides, code editor, Markdown)
// ---------------------------------------------------------------------------

/// The three Carousel slides of the Layout page, as (title, caption).
pub(crate) const CAROUSEL_SLIDES: &[(&str, &str)] = &[
    (
        "Native geometry",
        "Control heights, corner radii and line widths come from the platform theme.",
    ),
    (
        "Native colors",
        "Every ThemeColor field is derived from the resolved native palette.",
    ),
    (
        "Native icons",
        "IconRole maps to the desktop icon theme; sets are never mixed.",
    ),
];

/// What the demo `TitleBar`'s window controls do on this platform.
///
/// `on_close_window` is kept only on Linux (`title_bar.rs:99-101`); on Windows
/// the controls are hit-tested by the OS through `window_control_area`
/// (`:220-222`), so no handler can stand between them and the real window.
/// macOS draws none (`:254-256`).
pub(crate) const TITLE_BAR_CONTROLS_NOTE: &str = if cfg!(target_os = "windows") {
    "On Windows these buttons act on the real window: the OS hit-tests them \
     and upstream discards the close handler, so the X really does close the \
     showcase."
} else if cfg!(target_os = "macos") {
    "On macOS upstream draws no window controls in its own title bar; the \
     system provides them."
} else {
    "The close button is intercepted and inert; dragging the bar moves the \
     window and a double click zooms it."
};

/// How many pages the Data page's `Pagination` navigates, at ten rows each.
pub(crate) const PAGE_COUNT: usize = 12;

/// One row of the Data page's chat thread: who sent it and what it says.
#[derive(Clone)]
pub(crate) struct ChatMessage {
    pub(crate) outgoing: bool,
    pub(crate) sender: SharedString,
    pub(crate) text: SharedString,
}

/// The thread the `MessageScroller` starts with; the Send button appends.
pub(crate) fn initial_chat_messages() -> Vec<ChatMessage> {
    [
        (false, "Dana", "Does the palette follow the desktop?"),
        (true, "You", "It does — kdeglobals is read on every change."),
        (false, "Dana", "And the icons?"),
        (
            true,
            "You",
            "The icon theme the preset names, never a mixture of sets.",
        ),
        (false, "Dana", "Good. What about dark mode?"),
    ]
    .into_iter()
    .map(|(outgoing, sender, text)| ChatMessage {
        outgoing,
        sender: sender.into(),
        text: text.into(),
    })
    .collect()
}

/// One row of the thread, built the same way for the `Message` section and
/// for every row the `MessageScroller` renders: the sender's avatar beside a
/// bubble whose variant and alignment say which side sent it.
pub(crate) fn chat_message(msg: &ChatMessage) -> Message {
    let (alignment, variant) = if msg.outgoing {
        (MessageAlignment::End, BubbleVariant::Filled)
    } else {
        (MessageAlignment::Start, BubbleVariant::Muted)
    };
    Message::new()
        .alignment(alignment)
        .avatar(Avatar::new().name(msg.sender.clone()))
        .content(
            MessageContent::new()
                .bubble(Bubble::new().with_variant(variant).child(msg.text.clone())),
        )
}

/// The next lifecycle state the Attachment card steps to when it is clicked.
pub(crate) fn next_attachment_status(status: AttachmentStatus) -> AttachmentStatus {
    match status {
        AttachmentStatus::Pending => AttachmentStatus::Uploading,
        AttachmentStatus::Uploading => AttachmentStatus::Processing,
        AttachmentStatus::Processing => AttachmentStatus::Complete,
        AttachmentStatus::Complete => AttachmentStatus::Failed,
        AttachmentStatus::Failed => AttachmentStatus::Pending,
    }
}

/// The steps the Layout page's `Stepper` walks through.
pub(crate) const STEPPER_STEPS: &[(&str, IconName)] = &[
    ("Read the OS", IconName::Search),
    ("Resolve the theme", IconName::Settings),
    ("Apply to gpui", IconName::CircleCheck),
];

/// One panel of a resizable group: the title it carries, the line under that
/// title if it has one, and the size it asks the group for — `None` for the
/// panel that takes whatever the others leave.
pub(crate) struct ResizablePanelSpec {
    pub(crate) title: &'static str,
    pub(crate) caption: Option<&'static str>,
    pub(crate) size: Option<f32>,
}

/// One of the Layout page's resizable groups: the element id and debug selector
/// of the fixed-height box it sits in, its heading, the axis its divider
/// travels on, the box's height, and its panels.
pub(crate) struct ResizableGroup {
    pub(crate) id: &'static str,
    pub(crate) group_id: &'static str,
    pub(crate) heading: &'static str,
    pub(crate) axis: Axis,
    pub(crate) height: f32,
    pub(crate) panels: &'static [ResizablePanelSpec],
}

// The line width of the box a resizable group sits in is the platform's
// (`demo_frame`), so `resizable_groups_have_room_to_drag` asks
// `demo_border_width` for it rather than naming a number: it is inside the
// box's measured size and has to come off both edges before what is left is
// compared with `PANEL_MIN_SIZE`.

/// The Layout page's resizable groups.
///
/// The box's size along the divider's axis is what makes a group draggable:
/// gpui-base clamps every panel to `PANEL_MIN_SIZE` (gpui-base
/// resizable/mod.rs, `PANEL_MIN_SIZE`), so a two-panel group needs more than
/// twice that plus its border before the divider has anywhere to go. The
/// vertical group stood at 200px once, with both panels clamped to 99px and the
/// divider unable to move at all. `resizable_groups_have_room_to_drag` is that
/// finding as a rule, and it measures the boxes this list builds.
pub(crate) const RESIZABLE_GROUPS: &[ResizableGroup] = &[
    ResizableGroup {
        id: "tt-resizable-h",
        group_id: "resize-h",
        heading: "Resizable Panels (horizontal)",
        axis: Axis::Horizontal,
        // Cross-axis here: the width the divider travels on comes from the
        // content area, so only the laid-out bounds can report it.
        height: 160.0,
        panels: &[
            ResizablePanelSpec {
                title: "Left Panel",
                caption: Some("Drag the divider to resize"),
                size: Some(250.0),
            },
            ResizablePanelSpec {
                title: "Right Panel",
                caption: Some("This panel fills remaining space"),
                size: None,
            },
        ],
    },
    ResizableGroup {
        id: "tt-resizable-v",
        group_id: "resize-v",
        heading: "Resizable Panels (vertical)",
        axis: Axis::Vertical,
        height: 300.0,
        panels: &[
            ResizablePanelSpec {
                title: "Top Panel",
                caption: None,
                // Above PANEL_MIN_SIZE, so the request survives the clamp;
                // travel is 100px..198px.
                size: Some(130.0),
            },
            ResizablePanelSpec {
                title: "Bottom Panel",
                caption: None,
                size: None,
            },
        ],
    },
];

/// The source the code editor holds — the connector's own install sequence.
pub(crate) const EDITOR_SAMPLE: &str = r#"use gpui::App;
use native_theme::SystemTheme;
use native_theme_gpui::{ColorMode, apply_system_theme};

/// Install the desktop's colors, fonts and geometry into gpui-component.
fn install(cx: &mut App) -> native_theme::Result<()> {
    let system = SystemTheme::from_system()?;
    let mode = match system.mode.is_dark() {
        true => ColorMode::Dark,
        false => ColorMode::Light,
    };
    let resolved = system.pick(mode);
    println!("{} {}px", resolved.defaults.font.family, resolved.defaults.font.size);
    apply_system_theme(&system, cx);
    Ok(())
}
"#;

/// The Markdown source rendered by the `TextView` of the Typography page.
pub(crate) const MARKDOWN_SAMPLE: &str = r#"## What native-theme maps

The connector copies the resolved desktop theme into gpui-component's `Theme`;
see the [project README](https://github.com/tiborgats/native-theme) for the
full list.

```rust
apply_system_theme(&system, cx);
```

> A preset is never mixed with another platform's: a Linux desktop gets a
> Linux preset, with the matching icon set.

| ResolvedTheme | gpui-component Theme |
| --- | --- |
| `defaults.border.corner_radius` | `radius` |
| `defaults.mono_font.family` | `mono_font_family` |
| `input.min_height` | `geometry::input` height |
"#;

// ---------------------------------------------------------------------------
// Tooltip helpers
// ---------------------------------------------------------------------------

/// Build a multi-line tooltip string for a widget.
///
/// - `name`: widget display name
/// - `colors`: slice of (role, field_name, live Hsla value)
/// - `config`: slice of (what, live_value_string)
/// - `not_themeable`: slice of (what, why)
fn widget_tooltip(
    name: &str,
    colors: &[(&str, &str, Hsla, &str)],
    config: &[(&str, String)],
    not_themeable: &[(&str, &str)],
) -> String {
    let mut s = format!("{}\n", name);

    if !colors.is_empty() {
        s.push_str("\nTheme colors:\n");
        for (role, field, val, cited_at) in colors {
            s.push_str(&format!("  {}: {} {}", role, field, hsla_to_hex(*val)));
            if !cited_at.is_empty() {
                s.push_str(&format!(" ({cited_at})"));
            }
            s.push('\n');
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
pub(crate) fn format_font_info(
    font: &native_theme::theme::ResolvedFontSpec,
    mono_font: &native_theme::theme::ResolvedFontSpec,
) -> String {
    format!(
        "\nTheme fonts:\n  Font: {} {}\n  Mono: {} {}",
        font.family,
        defined_size(font),
        mono_font.family,
        defined_size(mono_font),
    )
}

/// A font size in the unit its source stated, never converted.
///
/// A platform that reports points (KDE, GNOME, Windows) and a preset written
/// in pixels both resolve to logical pixels, and 10.5pt at 96 DPI is exactly
/// 14px -- so dividing the resolved size by the DPI would print "10.5pt" for
/// a preset whose author wrote `size_px = 14`. `ResolvedFontSpec::defined_size`
/// carries what the source actually said, so this converts nothing.
pub(crate) fn defined_size(font: &native_theme::theme::ResolvedFontSpec) -> String {
    match font.defined_size {
        Some(native_theme::theme::FontSize::Pt(v)) => format!("{v}pt"),
        Some(native_theme::theme::FontSize::Px(v)) => format!("{v}px"),
        // Validation records a missing size rather than inventing one; say so
        // instead of presenting the resolved fallback as a definition.
        None => "(size not stated)".to_string(),
    }
}

/// Like [`widget_tooltip`] but appends the active theme font settings.
pub(crate) fn widget_tooltip_themed(
    font_info: &str,
    name: &str,
    colors: &[(&str, &str, Hsla, &str)],
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

/// What a Ghost Button is filled with while hovered: accent, at half alpha in
/// dark mode (gpui-component button/button.rs:1125-1131).
pub(crate) fn ghost_hover_fill(t: &gpui_component::theme::Theme) -> Hsla {
    if t.is_dark() {
        t.accent.opacity(0.5)
    } else {
        t.accent
    }
}

pub(crate) fn section(title: impl Into<SharedString>) -> Label {
    Label::new(title).text_size(px(13.0)).font_semibold()
}

/// A color swatch: small rounded square + label.
/// The geometry refinement a builder produces for the installed native theme,
/// or `None` before `apply` ran (spec §9.1). Returns an owned value so the
/// borrow of `cx` ends at once.
pub(crate) fn native_geometry<F: FnOnce(Native<'_>) -> StyleRefinement>(
    cx: &App,
    build: F,
) -> Option<StyleRefinement> {
    cx.native_theme().and_then(|nt| nt.native(cx)).map(build)
}

/// Applies a geometry refinement when one is available; the widget keeps
/// upstream's geometry otherwise.
/// `.native(cx, geometry::button)`: the builder's refinement when the native
/// theme is installed, nothing otherwise. Called straight after the
/// constructor, so any style the call site sets afterwards still wins.
///
/// Every widget in this showcase that has a `geometry` builder takes it, with
/// three kinds of exception, each marked where it occurs: the "Button Sizes"
/// row, which exists to show upstream's own size scale; `ButtonGroup` and
/// `DropdownButton` children, whose joined corners the group manages; and
/// widgets that are not `Styled` (`OtpInput`).
pub(crate) trait NativeStyled: Styled + Sized {
    fn native(self, cx: &App, build: fn(Native<'_>) -> StyleRefinement) -> Self {
        refined(self, native_geometry(cx, build).as_ref())
    }

    /// `.demo_frame(cx)`: the one frame this showcase draws around a
    /// demonstration.
    ///
    /// Every card, box and bordered row the showcase draws for its own
    /// purposes goes through this, so they agree with one another and follow
    /// the selected theme instead of a number someone typed once: the colour
    /// is `Theme::border` (the connector fills it from
    /// `defaults.border.color`), the radius is `Theme::radius` (from
    /// `defaults.border.corner_radius`), and the line width is the platform's
    /// `defaults.border.line_width` — 0.5 px on macOS and iOS, 1 px
    /// elsewhere. It clips, because a child that paints its own background to
    /// the edge would otherwise show through the rounded corners.
    ///
    /// A box that stands for a widget takes that widget's border instead —
    /// the List and the Tree take `geometry::list`.
    fn demo_frame(self, cx: &App) -> Self {
        let theme = cx.theme();
        let (colour, radius) = (theme.border, theme.radius);
        self.border(demo_border_width(cx))
            .border_color(colour)
            .rounded(radius)
            .overflow_hidden()
    }
}
impl<W: Styled> NativeStyled for W {}

/// The line width of a frame the showcase draws: the platform's own, or the
/// 1 px `border_1()` asks for where no native theme is installed yet. The
/// fallback is gpui's own width, never a theme value.
pub(crate) fn demo_border_width(cx: &App) -> Pixels {
    native_value(cx, |n| px(n.resolved.defaults.border.line_width)).unwrap_or(px(1.0))
}

/// Hand the images a cache is about to replace back to gpui.
///
/// The connector returns decoded icons (`ImageSource::Render`), which is what
/// keeps an animated icon from blinking through its first pass; the price is
/// that each one holds a tile in the window's sprite atlas until it is dropped
/// there, and nothing releases it on its own (`App::drop_image`, gpui-pre
/// `src/app.rs:2782-2792`). The showcase rebuilds these caches on every
/// icon-set change and on every colour change that re-colorizes the icons, so
/// without this the atlas would grow for the life of the window. The other
/// `ImageSource` variants are released through their own `remove_asset`, so a
/// future source shape is handled too.
pub(crate) fn release_sources(
    sources: impl IntoIterator<Item = ImageSource>,
    window: &mut Window,
    cx: &mut App,
) {
    for source in sources {
        match source {
            ImageSource::Render(image) => cx.drop_image(image, Some(window)),
            other => other.remove_asset(cx),
        }
    }
}

pub(crate) fn refined<W: Styled>(widget: W, style: Option<&StyleRefinement>) -> W {
    match style {
        Some(s) => widget.refine_style(s),
        None => widget,
    }
}

/// The value a size or length builder gives for the installed native theme,
/// or `None` before `apply` ran (spec §9.3, §9.4).
pub(crate) fn native_value<T, F: FnOnce(Native<'_>) -> T>(cx: &App, build: F) -> Option<T> {
    cx.native_theme().and_then(|nt| nt.native(cx)).map(build)
}

/// An icon at the platform's size for the role the builder names; upstream's
/// own size before `apply` ran.
pub(crate) fn native_icon(cx: &App, name: IconName, role: fn(Native<'_>) -> Size) -> Icon {
    let icon = Icon::new(name);
    match native_value(cx, role) {
        Some(size) => icon.with_size(size),
        None => icon,
    }
}

/// `v_flex`/`h_flex` sized by one of the layout accessors, which are `None`
/// wherever the platform specifies nothing (platform-facts §2.20). All 16
/// bundled presets state `layout.widget_gap`, so the `None` arm is reached
/// only on the `from_system()` error branch; there the widget keeps gpui's own
/// default gap, because nothing native is known to set.
pub(crate) fn with_gap<W: Styled>(widget: W, gap: Option<Pixels>) -> W {
    match gap {
        Some(g) => widget.gap(g),
        None => widget,
    }
}

pub(crate) fn with_padding<W: Styled>(widget: W, padding: Option<Pixels>) -> W {
    match padding {
        Some(p) => widget.p(p),
        None => widget,
    }
}

/// How a layout accessor's value reads in the showcase's own labels.
pub(crate) fn layout_value(value: Option<Pixels>) -> String {
    match value {
        Some(v) => format!("{}px", v.as_f32()),
        None => "unspecified by the platform".into(),
    }
}

/// `AccordionItem::title_style` with the native expander header height, when
/// the native theme is installed.
pub(crate) fn with_accordion_title_style(
    item: AccordionItem,
    style: &Option<StyleRefinement>,
) -> AccordionItem {
    match style {
        Some(s) => item.title_style(s.clone()),
        None => item,
    }
}

/// A `GroupBox` whose content carries the native card geometry when the native
/// theme is installed (`GroupBox::content_style`, spec §9.3).
pub(crate) fn native_group_box(cx: &App) -> GroupBox {
    match native_geometry(cx, geometry::group_box_content) {
        Some(s) => GroupBox::new().content_style(s),
        None => GroupBox::new(),
    }
}

/// `frame` is the showcase's own `demo_frame`, built once by the caller: the
/// fill is the datum this swatch exists to show, and everything around it is
/// the same box every other demonstration in the showcase sits in.
pub(crate) fn color_swatch(name: &str, color: Hsla, frame: &StyleRefinement) -> Div {
    let hex = hsla_to_hex(color);
    let label_text: SharedString = format!("{} {}", name, hex).into();
    h_flex()
        .gap_2()
        .items_center()
        .child(refined(div().size(px(16.0)).bg(color), Some(frame)))
        .child(Label::new(label_text).text_sm())
}

// ---------------------------------------------------------------------------
// Icon loading helper
// ---------------------------------------------------------------------------

/// Where an icon was loaded from.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum IconSource {
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
pub(crate) fn parse_icon_set_choice(display: &str) -> IconSetChoice {
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
pub(crate) fn load_all_icons(
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
pub(crate) fn is_native_icon_set(name: &str) -> bool {
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

/// The 101 gpui-component 0.6.4 IconName variants shown in the gallery.
const GPUI_ICONS: &[(&str, IconName)] = &[
    ("ALargeSmall", IconName::ALargeSmall),
    ("ArrowDown", IconName::ArrowDown),
    ("ArrowLeft", IconName::ArrowLeft),
    ("ArrowRight", IconName::ArrowRight),
    ("ArrowUp", IconName::ArrowUp),
    ("Asterisk", IconName::Asterisk),
    ("Battery", IconName::Battery),
    ("BatteryCharging", IconName::BatteryCharging),
    ("BatteryFull", IconName::BatteryFull),
    ("BatteryLow", IconName::BatteryLow),
    ("BatteryMedium", IconName::BatteryMedium),
    ("BatteryWarning", IconName::BatteryWarning),
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
    ("Cpu", IconName::Cpu),
    ("Dash", IconName::Dash),
    ("Delete", IconName::Delete),
    ("Ellipsis", IconName::Ellipsis),
    ("EllipsisVertical", IconName::EllipsisVertical),
    ("ExternalLink", IconName::ExternalLink),
    ("Eye", IconName::Eye),
    ("EyeOff", IconName::EyeOff),
    ("File", IconName::File),
    ("FileText", IconName::FileText),
    ("Folder", IconName::Folder),
    ("FolderClosed", IconName::FolderClosed),
    ("FolderOpen", IconName::FolderOpen),
    ("Frame", IconName::Frame),
    ("GalleryVerticalEnd", IconName::GalleryVerticalEnd),
    ("Github", IconName::Github),
    ("Globe", IconName::Globe),
    ("HardDrive", IconName::HardDrive),
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
    ("MemoryStick", IconName::MemoryStick),
    ("Menu", IconName::Menu),
    ("Minimize", IconName::Minimize),
    ("Minus", IconName::Minus),
    ("Moon", IconName::Moon),
    ("Network", IconName::Network),
    ("Palette", IconName::Palette),
    ("PanelBottom", IconName::PanelBottom),
    ("PanelBottomOpen", IconName::PanelBottomOpen),
    ("PanelLeft", IconName::PanelLeft),
    ("PanelLeftClose", IconName::PanelLeftClose),
    ("PanelLeftOpen", IconName::PanelLeftOpen),
    ("PanelRight", IconName::PanelRight),
    ("PanelRightClose", IconName::PanelRightClose),
    ("PanelRightOpen", IconName::PanelRightOpen),
    ("Pause", IconName::Pause),
    ("Play", IconName::Play),
    ("Plus", IconName::Plus),
    ("Redo", IconName::Redo),
    ("Redo2", IconName::Redo2),
    ("Replace", IconName::Replace),
    ("ResizeCorner", IconName::ResizeCorner),
    ("RotateCw", IconName::RotateCw),
    ("Search", IconName::Search),
    ("Settings", IconName::Settings),
    ("Settings2", IconName::Settings2),
    ("SortAscending", IconName::SortAscending),
    ("SortDescending", IconName::SortDescending),
    ("SquareTerminal", IconName::SquareTerminal),
    ("Star", IconName::Star),
    ("StarFill", IconName::StarFill),
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
pub(crate) type IconEntry = (
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
pub(crate) fn load_gpui_icons(
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
                    if let Some(fd_name) = freedesktop_name_for_gpui_icon(icon.clone(), *de)
                        && let Some(fd_data) = FreedesktopLoader::new(fd_name)
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
                if let Some(fd_name) = freedesktop_name_for_gpui_icon(icon.clone(), *de)
                    && let Some(data) = FreedesktopLoader::new(fd_name)
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
                    IconSet::Lucide => lucide_name_for_gpui_icon(icon.clone()),
                    IconSet::Material => material_name_for_gpui_icon(icon.clone()),
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
// Sample Table Delegate (for Data page)
// ---------------------------------------------------------------------------

pub(crate) struct SampleTableDelegate {
    pub(crate) columns: Vec<Column>,
    pub(crate) rows: Vec<[SharedString; 3]>,
}

impl TableDelegate for SampleTableDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _cx: &App) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> Column {
        self.columns[col_ix].clone()
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
// Sample List Delegate (for Data page)
// ---------------------------------------------------------------------------

pub(crate) struct SampleListDelegate {
    pub(crate) items: Vec<SharedString>,
    pub(crate) selected: Option<usize>,
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
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let label = self.items.get(ix.row)?.clone();
        Some(
            ListItem::new(("list-item", ix.row))
                .native(cx, geometry::list_item)
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
// The toolbar's preset Combobox
// ---------------------------------------------------------------------------

/// One row of the preset Combobox: a preset, by key and display name.
#[derive(Clone)]
pub(crate) struct PresetItem {
    key: SharedString,
    display_name: SharedString,
}

impl SearchableListItem for PresetItem {
    type Value = SharedString;

    fn title(&self) -> SharedString {
        self.display_name.clone()
    }

    fn value(&self) -> &Self::Value {
        &self.key
    }

    /// The key is searchable too, so typing `kde` finds "KDE Breeze".
    fn matches(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.display_name.to_lowercase().contains(&query)
            || self.key.to_lowercase().contains(&query)
    }
}

/// `Combobox` is generic over a `SearchableListDelegate` (`combobox.rs:749`),
/// so the toolbar's preset switch takes a delegate: the desktop's own theme,
/// keyed `default` and labelled with the preset it builds on, then the
/// presets meant for this platform, filtered as the user types.
pub(crate) struct PresetDelegate {
    items: Vec<PresetItem>,
    matched: Vec<PresetItem>,
}

impl PresetDelegate {
    pub(crate) fn new() -> Self {
        let default = PresetItem {
            key: "default".into(),
            display_name: format!("default ({})", platform_preset_name().name).into(),
        };
        let items: Vec<PresetItem> = std::iter::once(default)
            .chain(
                native_theme::theme::Theme::list_presets_for_platform()
                    .iter()
                    .map(|info| PresetItem {
                        key: info.key.into(),
                        display_name: info.display_name.into(),
                    }),
            )
            .collect();
        Self {
            matched: items.clone(),
            items,
        }
    }
}

impl SearchableListDelegate for PresetDelegate {
    type Item = PresetItem;

    fn items_count(&self, _section: usize) -> usize {
        self.matched.len()
    }

    fn item(&self, ix: IndexPath) -> Option<&Self::Item> {
        self.matched.get(ix.row)
    }

    fn position<V>(&self, value: &V) -> Option<IndexPath>
    where
        Self::Item: SearchableListItem<Value = V>,
        V: PartialEq,
    {
        self.matched
            .iter()
            .position(|item| item.value() == value)
            .map(|row| IndexPath::default().row(row))
    }

    fn perform_search(&mut self, query: &str, _window: &mut Window, _cx: &mut App) -> Task<()> {
        self.matched = self
            .items
            .iter()
            .filter(|item| item.matches(query))
            .cloned()
            .collect();
        Task::ready(())
    }

    /// Records the chosen preset at its row in the unfiltered list.
    ///
    /// The Combobox tells a changed selection from an unchanged one by the
    /// selection's row indices alone (combobox.rs, ComboboxState::new), and
    /// the default hook records the row the item has in the list as filtered
    /// (searchable_list/delegate.rs, SearchableListDelegate::on_will_change).
    /// Typing "nord" puts Nord in row 0, where `default` already is, so the
    /// choice would change nothing that upstream compares: no `Change`, and
    /// the popup stays open. A preset's row in the full list is its own.
    /// Single selection only, which is how the toolbar builds it.
    fn on_will_change(
        &mut self,
        selection: &mut Vec<(IndexPath, Self::Item)>,
        changes: &[SearchableListChange],
    ) {
        for change in changes {
            match change {
                SearchableListChange::Select { index } => {
                    let Some(item) = self.matched.get(index.row).cloned() else {
                        continue;
                    };
                    let Some(row) = self.items.iter().position(|i| i.key == item.key) else {
                        continue;
                    };
                    if !selection.iter().any(|(_, s)| s.key == item.key) {
                        selection.push((IndexPath::default().row(row), item));
                    }
                }
                // Upstream deselects the current choice by the index this
                // hook recorded for it (combobox.rs, selection_changes).
                SearchableListChange::Deselect { index } => {
                    selection.retain(|(at, _)| at != index);
                }
            }
        }
    }
}
