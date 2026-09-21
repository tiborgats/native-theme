//! native-theme-gpui — comprehensive widget showcase and designer reference.
//!
//! A full gpui-component widget gallery with tooltip-based documentation for
//! every theme-controlled property. Demonstrates all gpui-component widgets,
//! every `ThemeColor` field, every `IconName` variant, and live theme
//! switching across all bundled `native-theme` presets.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-gpui --example showcase-gpui
//! ```
//!
//! # What to look for
//!
//! - Sidebar on the left switches theme presets, color modes, and icon sets
//!   without restarting the app. Watch how the entire widget tree re-themes
//!   on each change — no manual rewiring per widget.
//! - Hover any widget to see tooltips explaining which `ResolvedTheme` fields
//!   drive its appearance.
//! - The Color Map tab exposes the full 138-field `ThemeColor` palette that
//!   gpui-component exposes, with each field's current value and the
//!   `native-theme` field it was derived from.
//! - The Icons tab demonstrates `IconRole` mapping across Material, Lucide,
//!   and freedesktop sets, plus animated spinner playback.
//!
//! # How this file is organised
//!
//! The source is split into section-divider blocks (`// ─────`) — one per
//! widget category, tab, or view. Search for the dividers to jump between
//! sections.

use gpui::{
    Animation, AnimationExt, AnyElement, App, Axis, Bounds, ClipboardItem, Context, Div, Entity,
    Hsla, ImageSource, IntoElement, Keystroke, Menu, MenuItem, ParentElement, Pixels, Render,
    SharedString, StyleRefinement, Styled, Task, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rems, size,
};
use gpui_component::{
    ActiveTheme, Disableable, Icon, IconName, IndexPath, Placement, Root, Sizable, Size, StyledExt,
    TitleBar, WindowExt,
    accordion::{Accordion, AccordionItem},
    alert::Alert,
    attachment::{
        Attachment, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentStatus,
        AttachmentTitle,
    },
    avatar::{Avatar, AvatarGroup},
    badge::Badge,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    bubble::{Bubble, BubbleVariant},
    button::{
        Button, ButtonGroup, ButtonVariant, ButtonVariants, DropdownButton, Toggle, ToggleGroup,
    },
    carousel::{
        Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPagination,
        CarouselPaginationItem, CarouselPrevious, CarouselState,
    },
    chart::{AreaChart, BarChart, CandlestickChart, LineChart, PieChart},
    checkbox::Checkbox,
    clipboard::Clipboard,
    collapsible::Collapsible,
    color_picker::{ColorPicker, ColorPickerState},
    combobox::{Combobox, ComboboxState},
    description_list::DescriptionList,
    dialog::{
        AlertDialog, DialogButtonProps, DialogClose, DialogDescription, DialogFooter, DialogTitle,
    },
    empty::{
        Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant,
        EmptyTitle,
    },
    form::{self, Field},
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
    hover_card::HoverCard,
    input::{
        Editor, EditorState, Input, InputGroup, InputGroupAddon, InputGroupAddonAlignment,
        InputGroupButton, InputGroupText, InputGroupTextarea, InputState, NumberInput,
        NumberInputEvent, OtpInput, OtpState, StepAction, Textarea, TextareaState,
    },
    kbd::Kbd,
    label::Label,
    link::Link,
    list::{ListDelegate, ListItem, ListState},
    marker::{Marker, MarkerContent, MarkerIcon, MarkerLoadingStyle, MarkerVariant},
    menu::{AppMenuBar, ContextMenuExt},
    message::{Message, MessageAlignment, MessageContent},
    message_scroller::{MessageScroller, MessageScrollerState},
    notification::Notification,
    pagination::Pagination,
    popover::Popover,
    progress::{Progress, ProgressCircle},
    radio::{Radio, RadioGroup},
    rating::Rating,
    resizable::{h_resizable, resizable_panel, v_resizable},
    scroll::ScrollableElement,
    searchable_list::{SearchableListDelegate, SearchableListItem},
    select::{SearchableVec, Select, SelectEvent, SelectState},
    separator::Separator,
    setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    shimmer::ShimmerText,
    sidebar::{Sidebar, SidebarMenu, SidebarMenuItem, SidebarToggleButton},
    skeleton::Skeleton,
    slider::{Slider, SliderEvent, SliderState},
    spinner::Spinner,
    status_bar::StatusBar,
    stepper::{Stepper, StepperItem},
    switch::Switch,
    tab::TabBar,
    table::{
        Column, DataTable, Table, TableBody, TableCell, TableDelegate, TableHead, TableHeader,
        TableRow, TableState,
    },
    tag::Tag,
    text::{TextView, TextViewStyle},
    theme::Theme,
    tooltip::Tooltip,
    tree::{Tree, TreeItem, TreeState},
    v_flex, window_paddings,
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
use native_theme_gpui::{AccessibilityPreferences, ActiveNativeTheme, Native, geometry, variants};

/// gpui-component's mode for the showcase's light/dark flag.
fn gpui_theme_mode(is_dark: bool) -> gpui_component::theme::ThemeMode {
    if is_dark {
        gpui_component::theme::ThemeMode::Dark
    } else {
        gpui_component::theme::ThemeMode::Light
    }
}

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------

/// The content area's tabs.
///
/// `Showcase::render` matches on this, so a new variant cannot be added without
/// the compiler asking what it renders, and the bar's labels, the `--tab` names
/// and the layout self-test all read [`Tab::ALL`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tab {
    Buttons,
    Inputs,
    Data,
    Feedback,
    Typography,
    Layout,
    Overlays,
    Charts,
    Icons,
    ThemeMap,
}

impl Tab {
    /// Every tab, in the order the bar shows them.
    ///
    /// A new variant forces an arm in [`Tab::index`] and [`Tab::label`], whose
    /// matches are exhaustive, and the index it is given there has to be its
    /// position in this array — the `const` block below rejects the build
    /// otherwise. The one thing neither the compiler nor that block can see is
    /// a variant added to the enum and to both matches but not to this list:
    /// it would take an index the array does not have, and the assertion fires.
    const ALL: [Self; 10] = [
        Self::Buttons,
        Self::Inputs,
        Self::Data,
        Self::Feedback,
        Self::Typography,
        Self::Layout,
        Self::Overlays,
        Self::Charts,
        Self::Icons,
        Self::ThemeMap,
    ];

    /// The tab's position in the bar, which is what `TabBar` counts in.
    const fn index(self) -> usize {
        match self {
            Self::Buttons => 0,
            Self::Inputs => 1,
            Self::Data => 2,
            Self::Feedback => 3,
            Self::Typography => 4,
            Self::Layout => 5,
            Self::Overlays => 6,
            Self::Charts => 7,
            Self::Icons => 8,
            Self::ThemeMap => 9,
        }
    }

    /// The tab at a bar position; `None` past the end.
    fn at(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    /// The label the bar shows.
    const fn label(self) -> &'static str {
        match self {
            Self::Buttons => "Buttons",
            Self::Inputs => "Inputs",
            Self::Data => "Data",
            Self::Feedback => "Feedback",
            Self::Typography => "Typography",
            Self::Layout => "Layout",
            Self::Overlays => "Overlays",
            Self::Charts => "Charts",
            Self::Icons => "Icons",
            Self::ThemeMap => "Theme Map",
        }
    }
}

/// `Tab::ALL` is in bar order and holds each tab once.
const _: () = {
    let mut i = 0;
    while i < Tab::ALL.len() {
        assert!(
            Tab::ALL[i].index() == i,
            "Tab::ALL is not the tabs in bar order"
        );
        i += 1;
    }
};

/// The window the showcase opens. The self-tests lay the interface out at this
/// width, so a measurement they take is a measurement of the real thing.
const WINDOW_SIZE: gpui::Size<Pixels> = size(px(1100.), px(850.));

/// The debug selector the active tab's root carries, so `every_tab_lays_out`
/// can find the tab it switched to.
const TAB_ROOT: &str = "tab-root";

// ---------------------------------------------------------------------------
// Debug selectors for the interactive controls
// ---------------------------------------------------------------------------
//
// `interactive_controls_respond` clicks each of these and asks the model what
// changed. A control that carries one is a control the self-test drives; the
// name is shared by the render code and the test, so neither can drift onto an
// element the other does not mean.
const PROBE_RATING: &str = "probe-rating";
const PROBE_COMBOBOX: &str = "probe-combobox";
const PROBE_CLIPBOARD: &str = "probe-clipboard";
const PROBE_PAGINATION: &str = "probe-pagination";
const PROBE_ATTACHMENT: &str = "probe-attachment";
const PROBE_CHAT_SEND: &str = "probe-chat-send";
const PROBE_STEPPER: &str = "probe-stepper";
const PROBE_SIDEBAR_TOGGLE: &str = "probe-sidebar-toggle";
const PROBE_CAROUSEL_LAST: &str = "probe-carousel-last";
const PROBE_ALERT_DIALOG: &str = "probe-alert-dialog";
const PROBE_NOTIFICATION: &str = "probe-notification";
const PROBE_COLOR_MODE: &str = "probe-color-mode";

/// Tag a control with a debug selector, so the self-test can find what it has
/// to click. The wrapper is a plain box around the control and leaves the
/// layout to it.
fn probe(selector: &'static str, control: impl IntoElement) -> Div {
    div().debug_selector(move || selector.into()).child(control)
}

// ---------------------------------------------------------------------------
// Sample content (Carousel slides, code editor, Markdown)
// ---------------------------------------------------------------------------

/// The three Carousel slides of the Layout tab, as (title, caption).
const CAROUSEL_SLIDES: &[(&str, &str)] = &[
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
const TITLE_BAR_CONTROLS_NOTE: &str = if cfg!(target_os = "windows") {
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

/// How many pages the Data tab's `Pagination` navigates, at ten rows each.
const PAGE_COUNT: usize = 12;

/// One row of the Data tab's chat thread: who sent it and what it says.
#[derive(Clone)]
struct ChatMessage {
    outgoing: bool,
    sender: SharedString,
    text: SharedString,
}

/// The thread the `MessageScroller` starts with; the Send button appends.
fn initial_chat_messages() -> Vec<ChatMessage> {
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
fn chat_message(msg: &ChatMessage) -> Message {
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
fn next_attachment_status(status: AttachmentStatus) -> AttachmentStatus {
    match status {
        AttachmentStatus::Pending => AttachmentStatus::Uploading,
        AttachmentStatus::Uploading => AttachmentStatus::Processing,
        AttachmentStatus::Processing => AttachmentStatus::Complete,
        AttachmentStatus::Complete => AttachmentStatus::Failed,
        AttachmentStatus::Failed => AttachmentStatus::Pending,
    }
}

/// The steps the Layout tab's `Stepper` walks through.
const STEPPER_STEPS: &[(&str, IconName)] = &[
    ("Read the OS", IconName::Search),
    ("Resolve the theme", IconName::Settings),
    ("Apply to gpui", IconName::CircleCheck),
];

/// One panel of a resizable group: the title it carries, the line under that
/// title if it has one, and the size it asks the group for — `None` for the
/// panel that takes whatever the others leave.
struct ResizablePanelSpec {
    title: &'static str,
    caption: Option<&'static str>,
    size: Option<f32>,
}

/// One of the Layout tab's resizable groups: the element id and debug selector
/// of the fixed-height box it sits in, its heading, the axis its divider
/// travels on, the box's height, and its panels.
struct ResizableGroup {
    id: &'static str,
    group_id: &'static str,
    heading: &'static str,
    axis: Axis,
    height: f32,
    panels: &'static [ResizablePanelSpec],
}

/// The line width of the box a resizable group sits in. It is inside the box's
/// measured size, so `resizable_groups_have_room_to_drag` takes it off both
/// edges before it compares what is left with `PANEL_MIN_SIZE`.
const RESIZABLE_BORDER: f32 = 1.0;

/// The Layout tab's resizable groups.
///
/// The box's size along the divider's axis is what makes a group draggable:
/// gpui-base clamps every panel to `PANEL_MIN_SIZE` (gpui-base
/// resizable/mod.rs, `PANEL_MIN_SIZE`), so a two-panel group needs more than
/// twice that plus its border before the divider has anywhere to go. The
/// vertical group stood at 200px once, with both panels clamped to 99px and the
/// divider unable to move at all. `resizable_groups_have_room_to_drag` is that
/// finding as a rule, and it measures the boxes this list builds.
const RESIZABLE_GROUPS: &[ResizableGroup] = &[
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
const EDITOR_SAMPLE: &str = r#"use gpui::App;
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

/// The Markdown source rendered by the `TextView` of the Typography tab.
const MARKDOWN_SAMPLE: &str = r#"## What native-theme maps

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
/// The geometry refinement a builder produces for the installed native theme,
/// or `None` before `apply` ran (spec §9.1). Returns an owned value so the
/// borrow of `cx` ends at once.
fn native_geometry<F: FnOnce(Native<'_>) -> StyleRefinement>(
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
trait NativeStyled: Styled + Sized {
    fn native(self, cx: &App, build: fn(Native<'_>) -> StyleRefinement) -> Self {
        refined(self, native_geometry(cx, build).as_ref())
    }
}
impl<W: Styled> NativeStyled for W {}

fn refined<W: Styled>(widget: W, style: Option<&StyleRefinement>) -> W {
    match style {
        Some(s) => widget.refine_style(s),
        None => widget,
    }
}

/// The value a size or length builder gives for the installed native theme,
/// or `None` before `apply` ran (spec §9.3, §9.4).
fn native_value<T, F: FnOnce(Native<'_>) -> T>(cx: &App, build: F) -> Option<T> {
    cx.native_theme().and_then(|nt| nt.native(cx)).map(build)
}

/// An icon at the platform's size for the role the builder names; upstream's
/// own size before `apply` ran.
fn native_icon(cx: &App, name: IconName, role: fn(Native<'_>) -> Size) -> Icon {
    let icon = Icon::new(name);
    match native_value(cx, role) {
        Some(size) => icon.with_size(size),
        None => icon,
    }
}

/// The height a platform button occupies, for the rows the showcase draws with
/// its own elements — a toolbar has no gpui-component widget to take a
/// refinement, so [`geometry::control_height`] is called directly.
fn native_control_height(cx: &App) -> Option<Pixels> {
    native_value(cx, |n| {
        let b = &n.resolved.button;
        geometry::control_height(b.min_height, &b.font, &b.border, n)
    })
}

/// `v_flex`/`h_flex` sized by one of the layout accessors, which are `None`
/// wherever the platform specifies nothing (platform-facts §2.20). All 16
/// bundled presets state `layout.widget_gap`, so the `None` arm is reached
/// only on the `from_system()` error branch; there the widget keeps gpui's own
/// default gap, because nothing native is known to set.
fn with_gap<W: Styled>(widget: W, gap: Option<Pixels>) -> W {
    match gap {
        Some(g) => widget.gap(g),
        None => widget,
    }
}

fn with_padding<W: Styled>(widget: W, padding: Option<Pixels>) -> W {
    match padding {
        Some(p) => widget.p(p),
        None => widget,
    }
}

/// How a layout accessor's value reads in the showcase's own labels.
fn layout_value(value: Option<Pixels>) -> String {
    match value {
        Some(v) => format!("{}px", v.as_f32()),
        None => "unspecified by the platform".into(),
    }
}

/// `AccordionItem::title_style` with the native expander header height, when
/// the native theme is installed.
fn with_accordion_title_style(
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
fn native_group_box(cx: &App) -> GroupBox {
    match native_geometry(cx, geometry::group_box_content) {
        Some(s) => GroupBox::new().content_style(s),
        None => GroupBox::new(),
    }
}

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
// Widget Info panel – separate Entity so hover updates only re-render this
// small panel instead of the entire Showcase.
// ---------------------------------------------------------------------------

struct WidgetInfoPanel {
    text: String,
    input_state: Entity<TextareaState>,
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
                Textarea::new(&self.input_state)
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
// Sample Combobox Delegate (for Inputs tab)
// ---------------------------------------------------------------------------

/// One row of the Combobox: a bundled preset, by key and display name.
#[derive(Clone)]
struct PresetItem {
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
/// so showing one takes a delegate: the bundled presets, filtered as the user
/// types. It selects nothing — the sidebar's `Select` is what switches the
/// theme; this one demonstrates the widget and its geometry builder.
struct PresetDelegate {
    items: Vec<PresetItem>,
    matched: Vec<PresetItem>,
}

impl PresetDelegate {
    fn new() -> Self {
        let items: Vec<PresetItem> = native_theme::theme::Theme::list_presets()
            .iter()
            .map(|info| PresetItem {
                key: info.key.into(),
                display_name: info.display_name.into(),
            })
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

    active_tab: Tab,

    /// Layout spacing of the installed theme. It lives on the model, not on
    /// `ResolvedTheme`, so the geometry accessors take it from here rather
    /// than from `cx.native_theme()`.
    layout: native_theme::theme::LayoutTheme,

    // Inputs tab
    input_state: Entity<InputState>,
    /// The field sized by `geometry::input_height` alone.
    input_height_state: Entity<InputState>,
    combobox_state: Entity<ComboboxState<PresetDelegate>>,
    input_group_state: Entity<InputState>,
    input_group_button_state: Entity<InputState>,
    input_group_textarea_state: Entity<TextareaState>,
    number_input_state: Entity<InputState>,
    /// The Form section's two fields. Held here, not built in `render`: a
    /// state created per frame loses whatever was typed into it.
    form_name_state: Entity<InputState>,
    form_email_state: Entity<InputState>,
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
    /// Stars the `Rating` currently shows; its `on_click` writes here.
    rating_value: usize,

    // Layout tab
    collapsible_open: bool,
    carousel_state: Entity<CarouselState>,
    /// The `Stepper`'s current step, written by its `on_click`.
    step: usize,
    /// Whether the Layout tab's `Sidebar` is collapsed; the
    /// `SidebarToggleButton` flips it.
    sidebar_collapsed: bool,

    // Typography tab
    editor_state: Entity<EditorState>,

    // Data tab
    table_state: Entity<TableState<SampleTableDelegate>>,
    list_state: Entity<ListState<SampleListDelegate>>,
    tree_state: Entity<TreeState>,
    /// The page the `Pagination` is on, written by its `on_click`.
    page: usize,
    /// The chat thread the `MessageScroller` renders. The data stays with the
    /// caller; the state below owns only the virtual list's bookkeeping.
    chat_messages: Vec<ChatMessage>,
    chat_scroller: Entity<MessageScrollerState>,
    /// The status the third `Attachment` card is in; clicking it advances.
    attachment_status: AttachmentStatus,

    // Buttons tab
    toggle_bold: bool,
    toggle_italic: bool,

    // Overlays tab
    app_menu_bar: Entity<AppMenuBar>,
    /// What the last `AlertDialog` was answered with, written by its `on_ok`
    /// and `on_cancel` so the section reports a real outcome.
    alert_choice: Option<SharedString>,

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
                cx.background_executor()
                    .timer(Duration::from_millis(min_duration))
                    .await;
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

        let input_height_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("Height only", window, cx);
            state
        });

        let combobox_state = cx.new(|cx| {
            ComboboxState::new(PresetDelegate::new(), Vec::new(), window, cx).searchable(true)
        });

        let input_group_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("Search the palette…", window, cx);
            state
        });

        let input_group_button_state =
            cx.new(|cx| InputState::new(window, cx).default_value("native-theme-gpui"));

        let input_group_textarea_state = cx.new(|cx| {
            let mut state = TextareaState::new(window, cx).auto_grow(3, 8);
            state.set_placeholder("Describe what the theme should look like…", window, cx);
            state
        });

        let form_name_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("Enter your name", window, cx);
            state
        });
        let form_email_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("you@example.com", window, cx);
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
                // 0.6.0 added `SliderEvent::Release`; only value changes matter here.
                if let SliderEvent::Change(val) = event {
                    this.slider_value = val.start();
                }
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
            initial_layout,
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
                let layout = system.layout.clone();
                // Install the OS theme with both variants stored; the showcase's own
                // light/dark choice then goes through upstream's mode switch, which
                // reproduces the native palette from the installed configs (D34).
                native_theme_gpui::apply_system_theme(&system, cx);
                if is_dark != system.mode.is_dark() {
                    Theme::change(gpui_theme_mode(is_dark), Some(window), cx);
                }
                let label = format!("default ({})", system.preset);
                // Platform presets always specify icon_theme
                (
                    font, mono_font, label, icon_theme, icon_set, true, layout, None,
                )
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
                    // Nothing was read, so nothing is claimed: every key stays
                    // `None` and the layout accessors report it as such.
                    native_theme::theme::LayoutTheme::default(),
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

        let carousel_state = cx.new(|_cx| CarouselState::new(CAROUSEL_SLIDES.len()));

        let initial_chat = initial_chat_messages();
        let chat_scroller = cx.new(|cx| MessageScrollerState::new(initial_chat.len(), cx));

        let editor_state = cx.new(|cx| {
            EditorState::new(window, cx)
                .language("rust")
                .default_value(EDITOR_SAMPLE)
        });

        // Set up application menus for AppMenuBar
        cx.set_menus(vec![
            Menu {
                name: "File".into(),
                disabled: false,
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
                disabled: false,
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
                disabled: false,
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
                disabled: false,
                items: vec![
                    MenuItem::action("Documentation", gpui::NoAction),
                    MenuItem::action("About", gpui::NoAction),
                ],
            },
        ]);
        let app_menu_bar = AppMenuBar::new(cx);

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
            active_tab: Tab::Buttons,
            layout: initial_layout,
            input_state,
            input_height_state,
            combobox_state,
            input_group_state,
            input_group_button_state,
            input_group_textarea_state,
            number_input_state,
            form_name_state,
            form_email_state,
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
            rating_value: 3,
            collapsible_open: true,
            carousel_state,
            step: 1,
            sidebar_collapsed: false,
            editor_state,
            table_state,
            list_state,
            tree_state,
            page: 5,
            chat_messages: initial_chat,
            chat_scroller,
            attachment_status: AttachmentStatus::Uploading,
            toggle_bold: false,
            toggle_italic: false,
            app_menu_bar,
            alert_choice: None,
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
                    let mut state = TextareaState::new(window, cx).auto_grow(4, 30);
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
                    self.layout = system.layout.clone();
                    // Platform presets always specify icon_theme
                    self.has_toml_icon_theme = true;
                    native_theme_gpui::apply_system_theme(&system, cx);
                    if self.is_dark != system.mode.is_dark() {
                        Theme::change(gpui_theme_mode(self.is_dark), Some(window), cx);
                    }
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
            self.layout = nt.layout.clone();

            let mode = if self.is_dark {
                native_theme_gpui::ColorMode::Dark
            } else {
                native_theme_gpui::ColorMode::Light
            };
            let r = match nt.resolve(mode) {
                Ok(r) => r,
                Err(e) => {
                    self.show_theme_error(&format!("Theme '{name}' resolution failed: {e}"));
                    return;
                }
            };
            self.has_toml_icon_theme = r.icon_theme_explicit;
            self.current_icon_set = r.icon_set;
            self.current_icon_theme = r.icon_theme.into_owned();
            self.original_font = r.variant.defaults.font.clone();
            self.original_mono_font = r.variant.defaults.mono_font.clone();
            // Preset path: accessibility is orthogonal to the theme choice, so the
            // OS preferences are honoured under a preset too (spec §7.1).
            let prefs = AccessibilityPreferences::from_system();
            let theme = to_theme(&r.variant, name, self.is_dark, &prefs);
            native_theme_gpui::apply(theme, &r.variant, &prefs, cx);
            self.error_message = None;
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
                cx.background_executor()
                    .timer(Duration::from_millis(500))
                    .await;
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
    fn render_sidebar(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let radius_str = format!("{}px", theme.radius.as_f32());
        let radius_lg_str = format!("{}px", theme.radius_lg.as_f32());
        let font_family_str = self.original_font.family.clone();
        let font_size_str = format!("{}px", self.original_font.size);
        let mono_family_str = self.original_mono_font.family.clone();
        let mono_size_str = format!("{}px", self.original_mono_font.size);
        let shadow_str = if theme.shadow { "true" } else { "false" };
        let scrollbar_str = format!("{:?}", theme.scrollbar_mode);

        let md = format!(
            "### Theme Config Inspector\n\n\
             **radius:** {}\n\
             **radius_lg:** {}\n\
             **font_family:** {}\n\
             **font_size:** {}\n\
             **mono_font_family:** {}\n\
             **mono_font_size:** {}\n\
             **shadow:** {}\n\
             **scrollbar_mode:** {}",
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
            TextView::markdown("config-inspector", SharedString::from(md))
                .selectable(true)
                .style(style)
                .text_xs(),
        )
    }

    // -----------------------------------------------------------------------
    // Tab: Buttons
    // -----------------------------------------------------------------------
    fn render_buttons_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        // One refinement per section (spec §9.1); `None` before `apply` ran.
        let button_style = native_geometry(cx, geometry::button);
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
                            .child(refined(
                                Button::new("b-primary").label("Primary").primary(),
                                button_style.as_ref(),
                            ))
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
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("font-weight", "hardcoded"),
                                    ("label size", "inner element (Tier U)"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-secondary")
                            .child(refined(
                                Button::new("b-secondary").label("Secondary"),
                                button_style.as_ref(),
                            ))
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
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("font-weight", "hardcoded"),
                                    ("label size", "inner element (Tier U)"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-danger")
                            .child(refined(
                                Button::new("b-danger").label("Danger").danger(),
                                button_style.as_ref(),
                            ))
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
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("font-weight", "hardcoded"),
                                    ("label size", "inner element (Tier U)"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-success")
                            .child(refined(
                                Button::new("b-success").label("Success").success(),
                                button_style.as_ref(),
                            ))
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
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("font-weight", "hardcoded"),
                                    ("label size", "inner element (Tier U)"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-warning")
                            .child(refined(
                                Button::new("b-warning").label("Warning").warning(),
                                button_style.as_ref(),
                            ))
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
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("font-weight", "hardcoded"),
                                    ("label size", "inner element (Tier U)"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-info")
                            .child(refined(
                                Button::new("b-info").label("Info").info(),
                                button_style.as_ref(),
                            ))
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
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("font-weight", "hardcoded"),
                                    ("label size", "inner element (Tier U)"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-ghost")
                            .child(refined(
                                Button::new("b-ghost")
                                    .label("Ghost")
                                    .custom(variants::ghost_button(cx)),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(
                                &fi,
                                "Button (Ghost)",
                                &[
                                    ("text", "secondary_foreground", t.secondary_foreground),
                                    ("hover bg", "secondary_hover", t.secondary_hover),
                                    ("active bg", "secondary_active", t.secondary_active),
                                ],
                                &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("variant", "native_theme_gpui::variants::ghost_button: flat like gpui-component's .ghost(), but with the platform's button.hover_background / active_background. Upstream's own .ghost() would hover with the item-highlight pair (button/button.rs, ButtonVariant::hovered Ghost arm), which is the menu selection colour, not a button hover"),
                                    ("font-weight", "hardcoded"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-link")
                            .child(refined(
                                Button::new("b-link").label("Link").link(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(
                                &fi,
                                "Button (Link)",
                                &[
                                    ("text", "foreground", t.foreground),
                                    ("hover-text", "muted_foreground", t.muted_foreground),
                                ],
                                &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("font-weight", "hardcoded"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-text")
                            .child(refined(
                                Button::new("b-text").label("Text").text(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(
                                &fi,
                                "Button (Text)",
                                &[
                                    ("text", "foreground", t.foreground),
                                    ("hover-text", "muted_foreground", t.muted_foreground),
                                ],
                                &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                                &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
                                    ("font-weight", "hardcoded"),
                                ],
                            )),
                    )
                    .child(
                        div()
                            .id("tt-btn-outline")
                            .child(refined(
                                Button::new("b-outline")
                                    .label("Outline")
                                    .primary()
                                    .outline(),
                                button_style.as_ref(),
                            ))
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
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)"),
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
                                Button::new("d-pri").native(cx, geometry::button)
                                    .label("Disabled Primary")
                                    .primary()
                                    .disabled(true),
                            )
                            .child(
                                Button::new("d-sec").native(cx, geometry::button)
                                    .label("Disabled Secondary")
                                    .disabled(true),
                            )
                            .child(
                                Button::new("d-dng").native(cx, geometry::button)
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
                            Button::new("l-pri").native(cx, geometry::button)
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
                                Button::new("bi-save").native(cx, geometry::button)
                                    .label("Save")
                                    .primary()
                                    .icon(IconName::Check),
                            )
                            .child(
                                Button::new("bi-search").native(cx, geometry::button)
                                    .label("Search")
                                    .icon(IconName::Search),
                            )
                            .child(
                                Button::new("bi-del").native(cx, geometry::button)
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
                            .child(probe(
                                PROBE_CLIPBOARD,
                                Clipboard::new("clip-1").value("cargo add native-theme"),
                            ))
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
    fn render_inputs_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let widget_gap = geometry::widget_gap(&self.layout);
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
                        with_gap(v_flex(), widget_gap)
                            .child(refined(
                                Input::new(&self.input_state)
                                    .with_size(Size::Medium)
                                    .w(px(360.0)),
                                native_geometry(cx, geometry::input).as_ref(),
                            ))
                            // The same control height without the rest of the
                            // refinement: what `geometry::input_height` is for
                            // (`Input::h`, input/input.rs:257), and a field
                            // that must line up with the one above without
                            // taking its border or text size.
                            .child({
                                let input = Input::new(&self.input_height_state)
                                    .with_size(Size::Medium)
                                    .w(px(360.0));
                                match native_value(cx, geometry::input_height) {
                                    Some(height) => input.h(height),
                                    None => input,
                                }
                            }),
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
                            ("geometry", "geometry::input: input.min_height (control height), border.corner_radius, line_width, input.font"),
                            ("second field", "geometry::input_height alone: the same control height, nothing else"),
                            ("padding", "inner editor (Tier U)"),
                        ],
                    )),
            )
            // InputGroup
            .child(section("InputGroup"))
            .child(
                div()
                    .id("tt-input-group")
                    .child(
                        v_flex()
                            .gap_3()
                            .w(px(360.0))
                            .child(refined(
                                InputGroup::new("input-group-inline")
                                    .input(Input::new(&self.input_group_state))
                                    .addon(
                                        InputGroupAddon::new("input-group-inline-addon")
                                            .child(Icon::new(IconName::Search)),
                                    ),
                                native_geometry(cx, geometry::input).as_ref(),
                            ))
                            .child(refined(
                                InputGroup::new("input-group-trailing")
                                    .input(Input::new(&self.input_group_button_state))
                                    .addon(
                                        InputGroupAddon::new("input-group-trailing-addon")
                                            .align(InputGroupAddonAlignment::InlineEnd)
                                            .child(
                                                // `InputGroupButton::new` is a ghost
                                                // button that upstream repaints, inside
                                                // a group, with a hover of `theme.muted`
                                                // (`input/group.rs:544-583`): a grey that
                                                // under Breeze is barely distinguishable
                                                // from the field, while every button
                                                // around it hovers blue. A custom variant
                                                // makes upstream skip that repaint.
                                                InputGroupButton::new("input-group-copy")
                                                    .native(cx, geometry::input_group_button)
                                                    .custom(variants::ghost_button(cx))
                                                    .icon(IconName::Copy)
                                                    .label("Copy")
                                                    .tooltip("Copy the field to the clipboard")
                                                    .on_click(cx.listener(
                                                        |this, _, window, cx| {
                                                            let value = this
                                                                .input_group_button_state
                                                                .read(cx)
                                                                .value();
                                                            cx.write_to_clipboard(
                                                                ClipboardItem::new_string(
                                                                    value.to_string(),
                                                                ),
                                                            );
                                                            window.push_notification(
                                                                Notification::success(value)
                                                                    .title("Copied")
                                                                    .autohide(true),
                                                                cx,
                                                            );
                                                        },
                                                    )),
                                            ),
                                    ),
                                native_geometry(cx, geometry::input).as_ref(),
                            ))
                            .child(
                                InputGroup::new("input-group-textarea")
                                    .input(InputGroupTextarea::new(
                                        &self.input_group_textarea_state,
                                    ))
                                    .addon(
                                        InputGroupAddon::new("input-group-textarea-addon")
                                            .align(InputGroupAddonAlignment::BlockEnd)
                                            .child(
                                                InputGroupText::new().child("Markdown supported"),
                                            ),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "InputGroup",
                        &[
                            ("border", "input", t.input),
                            ("focus ring", "ring", t.ring),
                            ("addon text", "muted_foreground", t.muted_foreground),
                            ("addon button hover", "secondary_hover", t.secondary_hover),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("geometry", "geometry::input on the frame: input.min_height (single-line groups only), border.corner_radius, line_width, input.font"),
                            ("addon padding", "inner (Tier U)"),
                            ("addon button", "native_theme_gpui::variants::ghost_button: flat idle, hover = secondary_hover (the platform's button.hover_background). Upstream's own in-group ghost would hover with muted (input/group.rs, InputGroupButton::render_in_group)"),
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
                            .native(cx, geometry::input)
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
                                refined(
                                    Checkbox::new("cb-a"),
                                    native_geometry(cx, geometry::checkbox).as_ref(),
                                )
                                .label("Enable notifications")
                                .checked(checkbox_a)
                                .on_click(cx.listener(
                                    |this, val: &bool, _w, _cx| {
                                        this.checkbox_a = *val;
                                    },
                                )),
                            )
                            .child(
                                refined(
                                    Checkbox::new("cb-b"),
                                    native_geometry(cx, geometry::checkbox).as_ref(),
                                )
                                .label("Auto-save drafts")
                                .checked(checkbox_b)
                                .on_click(cx.listener(
                                    |this, val: &bool, _w, _cx| {
                                        this.checkbox_b = *val;
                                    },
                                )),
                            )
                            .child(
                                refined(
                                    Checkbox::new("cb-c"),
                                    native_geometry(cx, geometry::checkbox).as_ref(),
                                )
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
                            ("geometry", "geometry::checkbox: checkbox.label_gap, checkbox.font"),
                            ("font colour", "carried as size and weight only. Upstream wraps a Checkbox label in a div that sets foreground itself and re-sets muted_foreground there when disabled (checkbox.rs, Checkbox::render), and the disabled hook applies muted_foreground before this refinement, so a carried colour would never reach the label and would displace the disabled colour of custom children (native-theme-gpui geometry.rs, geometry::checkbox)"),
                            ("indicator size", "inner element (Tier U)"),
                        ],
                    )),
            )
            // Radio group
            .child(section("Radio Group"))
            .child(
                div()
                    .id("tt-radio")
                    .child(
                        // `RadioGroup` takes `impl Into<Radio>`, so a `&str`
                        // child would build a `Radio` with no refinement;
                        // built here instead, each row carries the platform's
                        // label gap and font. The group overwrites the id
                        // (`radio.rs:406`), not the style.
                        RadioGroup::horizontal("rg-1")
                            .child(
                                Radio::new("rg-a")
                                    .native(cx, geometry::radio)
                                    .label("Option A"),
                            )
                            .child(
                                Radio::new("rg-b")
                                    .native(cx, geometry::radio)
                                    .label("Option B"),
                            )
                            .child(
                                Radio::new("rg-c")
                                    .native(cx, geometry::radio)
                                    .label("Option C"),
                            )
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
                        &[
                            ("geometry", "geometry::radio: checkbox.label_gap, checkbox.font (platform-facts §2.5: radio metrics are the checkbox's)"),
                            ("indicator size", "hardcoded"),
                        ],
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
            // Rating
            .child(section(format!(
                "Rating ({} of 5 stars)",
                self.rating_value
            )))
            .child(
                div()
                    .id("tt-rating")
                    .child(
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            .child(probe(PROBE_RATING, {
                                // The stars are inline icons, so the platform's
                                // small icon size is what they take; `Rating`
                                // has no geometry builder of its own.
                                let rating = Rating::new("rating-1")
                                    .value(self.rating_value)
                                    .on_click(cx.listener(
                                        |this, value: &usize, _w, cx| {
                                            this.rating_value = *value;
                                            cx.notify();
                                        },
                                    ));
                                match native_value(cx, geometry::icon_size_small) {
                                    Some(size) => rating.with_size(size),
                                    None => rating,
                                }
                            }))
                            .child(
                                Label::new(SharedString::from(format!(
                                    "value: {}",
                                    self.rating_value
                                )))
                                .text_sm()
                                .text_color(t.muted_foreground),
                            )
                            .child({
                                let disabled = Rating::new("rating-disabled").value(2).disabled(true);
                                match native_value(cx, geometry::icon_size_small) {
                                    Some(size) => disabled.with_size(size),
                                    None => disabled,
                                }
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Rating",
                        &[
                            ("active star", "yellow", t.yellow),
                            ("inactive star", "muted_foreground", t.muted_foreground),
                        ],
                        &[],
                        &[
                            ("star size", "geometry::icon_size_small: defaults.icon_sizes.small"),
                            ("active colour", "cx.theme().yellow unless Rating::color overrides it (rating.rs, Rating::render active_color)"),
                            ("hover preview", "upstream keeps its own hovered value (rating.rs, RaitingState::hovered_value)"),
                        ],
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
            // Combobox
            .child(section("Combobox (searchable, over the bundled presets)"))
            .child(
                div()
                    .id("tt-combobox")
                    .child(probe(
                        PROBE_COMBOBOX,
                        Combobox::new(&self.combobox_state)
                            .native(cx, geometry::combobox)
                            .placeholder("Pick a preset…")
                            .search_placeholder("Filter by name or key…")
                            .menu_width(px(260.0))
                            .w(px(260.0)),
                    ))
                    .on_hover(self.hover_info(
                        &fi,
                        "Combobox",
                        &[
                            ("trigger bg", "background", t.background),
                            ("trigger border", "input", t.input),
                            ("text", "foreground", t.foreground),
                            ("popup bg", "popover", t.popover),
                            ("row hover", "list_hover", t.list_hover),
                            ("focus ring", "ring", t.ring),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("geometry", "geometry::combobox: combo_box.min_height (control height), min_width, border.corner_radius, combo_box.font"),
                            ("font colour", "carried as size and weight only. Upstream's input_style delivers muted_foreground to the trigger when disabled (input/input.rs, input_style) before this refinement lands on it (combobox.rs, render_trigger_container), and the selected-title child sets no colour to re-mute with (combobox.rs, ComboboxState::default_trigger_body), so a carried colour would beat the disabled colour instead of yielding to it. Select, whose title child does re-mute, takes it (native-theme-gpui geometry.rs, geometry::combobox)"),
                            ("delegate", "SearchableListDelegate, implemented in this showcase (combobox.rs, Combobox<D>)"),
                            ("caret", "inner element (Tier U)"),
                        ],
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
    fn render_data_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let widget_gap = geometry::widget_gap(&self.layout);
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
                    .child(
                        DataTable::new(&self.table_state)
                            .stripe(true)
                            .bordered(true),
                    )
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
                        &[
                            ("row height", "hardcoded per Size"),
                            (
                                "geometry",
                                "DataTable is not Styled (table/data_table.rs: DataTable impls \
                                 Sizable and RenderOnce, not Styled); \
                                 geometry::table goes to the declarative Table below",
                            ),
                        ],
                    )),
            )
            // The other table: rows written out instead of driven by a
            // delegate. This one is `Styled` (`table/table.rs:84`), so it is
            // the receiver `geometry::table` documents.
            .child(section("Table (declarative)"))
            .child(
                div()
                    .id("tt-table-declarative")
                    .child(
                        Table::new()
                            .native(cx, geometry::table)
                            .accessibility_label("Theme sources")
                            .child(
                                TableHeader::new().child(
                                    TableRow::new()
                                        .child(TableHead::new().child("Field"))
                                        .child(TableHead::new().child("Source")),
                                ),
                            )
                            .child(
                                TableBody::new()
                                    .child(
                                        TableRow::new()
                                            .child(TableCell::new().child("radius"))
                                            .child(TableCell::new().child(
                                                "defaults.border.corner_radius",
                                            )),
                                    )
                                    .child(
                                        TableRow::new()
                                            .child(TableCell::new().child("font"))
                                            .child(
                                                TableCell::new().child("defaults.font.family"),
                                            ),
                                    )
                                    .child(
                                        TableRow::new()
                                            .child(TableCell::new().child("row text"))
                                            .child(TableCell::new().child("list.item_font")),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Table (declarative)",
                        &[
                            ("bg", "table", t.table),
                            ("header bg", "table_head", t.table_head),
                            (
                                "header text",
                                "table_head_foreground",
                                t.table_head_foreground,
                            ),
                            ("row border", "table_row_border", t.table_row_border),
                        ],
                        &[],
                        &[
                            ("geometry", "geometry::table: list.item_font on the table root (table/table.rs, Table::render: text_sm then refine_style)"),
                            ("cell padding", "inner (Tier U)"),
                        ],
                    )),
            )
            // Pagination
            .child(section(format!(
                "Pagination (page {} of {})",
                self.page, PAGE_COUNT
            )))
            .child(
                div()
                    .id("tt-pagination")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .child(probe(
                                PROBE_PAGINATION,
                                with_gap(
                                    Pagination::new("pagination-1")
                                    .current_page(self.page)
                                    .total_pages(PAGE_COUNT)
                                    .on_click(cx.listener(|this, page: &usize, _w, cx| {
                                        this.page = *page;
                                        cx.notify();
                                    })),
                                    widget_gap,
                                ),
                            ))
                            .child(
                                Label::new(SharedString::from(format!(
                                    "Rows {}–{} of {}",
                                    (self.page - 1) * 10 + 1,
                                    self.page * 10,
                                    PAGE_COUNT * 10
                                )))
                                .text_sm()
                                .text_color(t.muted_foreground),
                            )
                            .child(
                                Pagination::new("pagination-compact")
                                    .compact()
                                    .current_page(self.page)
                                    .total_pages(PAGE_COUNT)
                                    .on_click(cx.listener(|this, page: &usize, _w, cx| {
                                        this.page = *page;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Pagination",
                        &[
                            ("current page", "background", t.background),
                            ("current border", "border", t.border),
                            ("other pages", "transparent until hover", t.transparent),
                            ("hover", "secondary_hover", t.secondary_hover),
                            ("text", "foreground", t.foreground),
                        ],
                        &[],
                        &[
                            ("gap", "geometry::widget_gap on the row; upstream's own is gap_1 (pagination.rs, Pagination::render)"),
                            ("buttons", "built by the widget as ghost/outline Button (pagination.rs, Pagination::render page items); no refinement reaches them"),
                            ("ellipsis", "a dropdown over the hidden pages"),
                        ],
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
                            ("bg", "list", t.colors.list),
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
                        |ix, entry, selected, _w, cx| {
                            ListItem::new(("tree-item", ix))
                                .native(cx, geometry::list_item)
                                .child(Label::new(entry.item().label.clone()).text_sm())
                                .selected(selected)
                        },
                    ))
                    .on_hover(self.hover_info(
                        &fi,
                        "Tree",
                        &[
                            ("bg", "list", t.colors.list),
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
            // Bubble
            .child(section("Bubble (all 7 variants, incoming and outgoing)"))
            .child(
                div()
                    .id("tt-bubble")
                    .child(
                        with_gap(v_flex().w(px(420.0)), widget_gap)
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Muted)
                                    .child("Incoming, Muted"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::End)
                                    .with_variant(BubbleVariant::Filled)
                                    .child("Outgoing, Filled"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Secondary)
                                    .child("Secondary"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::End)
                                    .with_variant(BubbleVariant::Tinted)
                                    .child("Tinted"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Outline)
                                    .child("Outline"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Ghost)
                                    .child("Ghost: no surface, no padding"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::End)
                                    .with_variant(BubbleVariant::Destructive)
                                    .child("Destructive: this one failed to send"),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Bubble",
                        &[
                            ("filled bg", "primary", t.primary),
                            ("filled text", "primary_foreground", t.primary_foreground),
                            ("muted / secondary bg", "muted", t.muted),
                            ("outline border", "border", t.border),
                            ("destructive", "danger", t.danger),
                        ],
                        &[(
                            "border-radius",
                            format!("radius_2xl(): {}px", t.radius_2xl().as_f32()),
                        )],
                        &[
                            ("stack gap", "geometry::widget_gap between the bubbles"),
                            ("surface padding", "hardcoded px_3/py_2 (bubble.rs, the content surface's RenderOnce)"),
                            ("max width", "80% of the row: max_w(relative(0.8)) (bubble.rs, Bubble::render)"),
                        ],
                    )),
            )
            // Message
            .child(section("Message (avatar, bubble, both alignments)"))
            .child(
                div()
                    .id("tt-message")
                    .child(
                        with_gap(v_flex().w(px(420.0)), widget_gap).children(
                            self.chat_messages.iter().take(2).map(chat_message),
                        ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Message",
                        &[
                            ("incoming bubble", "muted", t.muted),
                            ("outgoing bubble", "primary", t.primary),
                            ("avatar fallback", "secondary", t.secondary),
                        ],
                        &[],
                        &[
                            ("row gap", "geometry::widget_gap between the rows"),
                            ("slot gap", "hardcoded rems(0.625) (message.rs, Message::render)"),
                            ("avatar baseline", "a shared size-8 minimum, kept flush with the bubble's bottom edge (message.rs, the avatar slot's RenderOnce: min_w_8, self_end)"),
                        ],
                    )),
            )
            // MessageScroller
            .child(section(format!(
                "MessageScroller (virtualised thread, {} messages)",
                self.chat_messages.len()
            )))
            .child(
                div()
                    .id("tt-message-scroller")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(460.0))
                            .child(
                                div()
                                    .h(px(240.0))
                                    .border_1()
                                    .border_color(t.border)
                                    .rounded(t.radius)
                                    .child({
                                        // The data stays with the caller; the
                                        // state owns only the virtual list's
                                        // bookkeeping (message_scroller.rs:22-25).
                                        let messages = self.chat_messages.clone();
                                        MessageScroller::new(
                                            "chat-scroller",
                                            self.chat_scroller.clone(),
                                            move |ix, _w, _cx| match messages.get(ix) {
                                                Some(msg) => {
                                                    chat_message(msg).into_any_element()
                                                }
                                                None => div().into_any_element(),
                                            },
                                        )
                                        .with_bottom_fade(t.background)
                                        .size_full()
                                    }),
                            )
                            .child(probe(
                                PROBE_CHAT_SEND,
                                Button::new("chat-send")
                                    .native(cx, geometry::button)
                                    .label("Send a reply")
                                    .on_click(cx.listener(|this, _ev, _w, cx| {
                                        this.chat_messages.push(ChatMessage {
                                            outgoing: true,
                                            sender: "You".into(),
                                            text: "Dark mode follows the desktop too.".into(),
                                        });
                                        this.chat_scroller.update(cx, |state, cx| {
                                            state.append(1, cx);
                                        });
                                        cx.notify();
                                    })),
                            )),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "MessageScroller",
                        &[
                            ("bottom fade", "background", t.background),
                            ("scrollbar", "scrollbar_thumb", t.scrollbar_thumb),
                            ("jump button", "secondary", t.secondary),
                        ],
                        &[],
                        &[
                            ("rows", "the Message rows above, rendered on demand"),
                            ("follow", "FollowMode::Tail: Send scrolls the thread to the new row (message_scroller.rs, MessageScrollerState::new)"),
                            ("jump button", "appears once the user scrolls away from the tail"),
                        ],
                    )),
            )
            // Attachment
            .child(section(format!(
                "Attachment (complete, uploading, and one at {:?} — click it)",
                self.attachment_status
            )))
            .child(
                div()
                    .id("tt-attachment")
                    .child(
                        with_gap(h_flex().flex_wrap(), widget_gap)
                            .child(
                                Attachment::new()
                                    .media(AttachmentMedia::new().child(native_icon(
                                        cx,
                                        IconName::Inbox,
                                        geometry::icon_size_small,
                                    )))
                                    .content(
                                        AttachmentContent::new()
                                            .title(AttachmentTitle::new("platform-facts.md"))
                                            .description(AttachmentDescription::new("48 KB")),
                                    ),
                            )
                            .child(
                                Attachment::new()
                                    .status(AttachmentStatus::Uploading)
                                    .media(AttachmentMedia::new().child(native_icon(
                                        cx,
                                        IconName::Copy,
                                        geometry::icon_size_small,
                                    )))
                                    .content(
                                        AttachmentContent::new()
                                            .title(
                                                AttachmentTitle::new("breeze-palette.png")
                                                    .status(AttachmentStatus::Uploading),
                                            )
                                            .description(
                                                AttachmentDescription::new("uploading…")
                                                    .status(AttachmentStatus::Uploading),
                                            ),
                                    ),
                            )
                            // The whole card is the click target, so the
                            // status it is in is the status a click advances.
                            .child(probe(
                                PROBE_ATTACHMENT,
                                Attachment::new()
                                    .id("attachment-cycle")
                                    .status(self.attachment_status)
                                    .media(AttachmentMedia::new().child(native_icon(
                                        cx,
                                        IconName::Settings,
                                        geometry::icon_size_small,
                                    )))
                                    .content(
                                        AttachmentContent::new()
                                            .title(
                                                AttachmentTitle::new("kdeglobals")
                                                    .status(self.attachment_status),
                                            )
                                            .description(
                                                AttachmentDescription::new(SharedString::from(
                                                    format!("{:?}", self.attachment_status),
                                                ))
                                                .status(self.attachment_status),
                                            ),
                                    )
                                    .on_click(cx.listener(|this, _ev, _w, cx| {
                                        this.attachment_status =
                                            next_attachment_status(this.attachment_status);
                                        cx.notify();
                                    })),
                            )),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Attachment",
                        &[
                            ("bg", "background", t.background),
                            ("border", "border", t.border),
                            ("media bg", "muted", t.muted),
                            ("description", "muted_foreground", t.muted_foreground),
                            ("failed", "danger", t.danger),
                        ],
                        &[(
                            "border-radius",
                            format!("radius_2xl(): {}px", t.radius_2xl().as_f32()),
                        )],
                        &[
                            ("card gap", "geometry::widget_gap between the cards"),
                            ("icon size", "geometry::icon_size_small: defaults.icon_sizes.small"),
                            ("in-progress title", "the ShimmerText highlight, driven by the status (attachment.rs, AttachmentTitle::render)"),
                            ("pending", "a dashed border; failed tints the border with destructive (attachment.rs, Attachment::render: border_dashed, destructive.opacity(0.3))"),
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
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let widget_gap = geometry::widget_gap(&self.layout);
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
                            .child(refined(
                                Progress::new("progress-upload").value(73.0),
                                native_geometry(cx, geometry::progress).as_ref(),
                            ))
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Processing").text_sm())
                                    .child(Label::new("45%").text_sm()),
                            )
                            .child(refined(
                                Progress::new("progress-processing").value(45.0),
                                native_geometry(cx, geometry::progress).as_ref(),
                            ))
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Complete").text_sm())
                                    .child(Label::new("100%").text_sm()),
                            )
                            .child(refined(
                                Progress::new("progress-complete").value(100.0),
                                native_geometry(cx, geometry::progress).as_ref(),
                            )),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Progress",
                        &[("bar", "progress_bar", t.progress_bar)],
                        &[],
                        &[
                            ("geometry", "geometry::progress: progress_bar.track_height, border.corner_radius, min_width"),
                            ("animation", "hardcoded"),
                        ],
                    )),
            )
            // ProgressCircle
            .child(section("ProgressCircle (determinate and indeterminate)"))
            .child(
                div()
                    .id("tt-progress-circle")
                    .child(
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            .child(
                                with_gap(h_flex(), widget_gap)
                                    .items_center()
                                    .child(ProgressCircle::new("progress-circle-73").value(73.0))
                                    .child(Label::new("73%").text_sm()),
                            )
                            .child(
                                with_gap(h_flex(), widget_gap)
                                    .items_center()
                                    .child(
                                        ProgressCircle::new("progress-circle-100")
                                            .value(100.0)
                                            .with_size(Size::Large),
                                    )
                                    .child(Label::new("100%").text_sm()),
                            )
                            .child({
                                // Indeterminate, it is a spinner drawn as an
                                // arc, so the platform's spinner diameter is
                                // the size it should take.
                                let circle =
                                    ProgressCircle::new("progress-circle-loading").loading(true);
                                match native_value(cx, geometry::spinner_size) {
                                    Some(size) => circle.with_size(size),
                                    None => circle,
                                }
                            })
                            .child(
                                Label::new("indeterminate")
                                    .text_sm()
                                    .text_color(t.muted_foreground),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "ProgressCircle",
                        &[
                            ("arc", "progress_bar", t.progress_bar),
                            ("track", "progress_bar at 20%: color.opacity(0.2) (progress/progress_circle.rs, ProgressCircle::render_circle)", t.progress_bar),
                        ],
                        &[],
                        &[
                            ("indeterminate size", "geometry::spinner_size: spinner.diameter"),
                            ("determinate size", "per Size enum; the model carries no circular-progress diameter"),
                            ("stroke width", "15% of the diameter, capped at 5px (progress/progress_circle.rs, ProgressCircle::render_circle stroke_width)"),
                        ],
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
                                    .child(Spinner::new().with_size(
                                        cx.native_theme()
                                            .and_then(|nt| nt.native(cx))
                                            .map_or(Size::Medium, geometry::spinner_size),
                                    ))
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
                            ("size", "Small/Large per Size enum; Medium via geometry::spinner_size (spinner.diameter)"),
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
            // ShimmerText
            .child(section("ShimmerText (a highlight sweeping across the label)"))
            .child(
                div()
                    .id("tt-shimmer-text")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(360.0))
                            // The highlight is mixed from the text colour, so
                            // each of these shimmers in whatever colour the
                            // native theme gave its own text.
                            .child(ShimmerText::new("Reading the desktop configuration…"))
                            .child(
                                ShimmerText::new("Resolving the palette…")
                                    .id("shimmer-slow")
                                    .duration(Duration::from_secs(3))
                                    .text_color(t.muted_foreground),
                            )
                            .child(
                                ShimmerText::new("Applying to gpui…")
                                    .id("shimmer-reverse")
                                    .reverse(true)
                                    .spread(0.5),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "ShimmerText",
                        &[
                            ("text", "foreground", t.foreground),
                            ("second line", "muted_foreground", t.muted_foreground),
                        ],
                        &[],
                        &[
                            ("highlight", "the text colour mixed with background (light) or foreground (dark), at 75%/60% peak (shimmer.rs, shimmer_highlight_color)"),
                            ("reduced motion", "gpui's App::reduce_motion: the text renders once, unanimated (shimmer.rs, ShimmerText::render)"),
                            ("sweep", "2s by default; 3s and a reversed 0.5 spread here"),
                        ],
                    )),
            )
            // Empty state
            .child(section("Empty"))
            .child(
                div()
                    .id("tt-empty")
                    .child(
                        Empty::new()
                            .w(px(360.0))
                            .header(
                                EmptyHeader::new()
                                    .media(
                                        EmptyMedia::new()
                                            .with_variant(EmptyMediaVariant::Icon)
                                            // An empty state's icon is the
                                            // large one; `EmptyMedia` takes it
                                            // as a plain child, so the size
                                            // survives.
                                            .child(native_icon(
                                                cx,
                                                IconName::Inbox,
                                                geometry::icon_size_large,
                                            )),
                                    )
                                    .title(EmptyTitle::new().child("No notifications"))
                                    .description(EmptyDescription::new().child(
                                        "Anything the application reports shows up here.",
                                    )),
                            )
                            .content(
                                EmptyContent::new().child(
                                    Button::new("empty-refresh").native(cx, geometry::button).label("Refresh").outline(),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Empty",
                        &[
                            ("border", "border", t.border),
                            ("media bg", "muted", t.muted),
                            ("title", "foreground", t.foreground),
                            ("description", "muted_foreground", t.muted_foreground),
                        ],
                        &[(
                            "border-radius",
                            format!("radius_tokens().xl: {}px", t.radius_tokens().xl.as_f32()),
                        )],
                        &[
                            ("icon size", "geometry::icon_size_large: defaults.icon_sizes.large"),
                            ("border style", "hardcoded dashed"),
                            ("media frame", "hardcoded 2rem square"),
                        ],
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
                                    .child(Button::new("badge-1").native(cx, geometry::button).label("Messages")),
                            )
                            .child(
                                Badge::new()
                                    .count(99)
                                    .child(Button::new("badge-2").native(cx, geometry::button).label("Notifications")),
                            )
                            .child(
                                Badge::new()
                                    .dot()
                                    .child(Button::new("badge-3").native(cx, geometry::button).label("Updates")),
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
            // Marker
            .child(section("Marker (3 variants, 2 loading styles)"))
            .child(
                div()
                    .id("tt-marker")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(360.0))
                            .child(
                                Marker::new()
                                    .icon(MarkerIcon::new().child(native_icon(
                                        cx,
                                        IconName::CircleCheck,
                                        geometry::icon_size_small,
                                    )))
                                    .content(MarkerContent::new().text("Theme applied")),
                            )
                            .child(
                                Marker::new()
                                    .with_variant(MarkerVariant::Separator)
                                    .content(MarkerContent::new().text("Today")),
                            )
                            .child(
                                Marker::new()
                                    .with_variant(MarkerVariant::Border)
                                    .content(MarkerContent::new().text("Unread from here")),
                            )
                            // Loading, with the spinner the marker adds for
                            // itself when no icon slot is set.
                            .child(
                                Marker::new()
                                    .id("marker-spinner")
                                    .loading(true)
                                    .content(MarkerContent::new().text("Reading the OS theme…")),
                            )
                            .child(
                                Marker::new()
                                    .id("marker-shimmer")
                                    .loading(true)
                                    .with_loading_style(MarkerLoadingStyle::Shimmer)
                                    .content(MarkerContent::new().text("Resolving the palette…")),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Marker",
                        &[
                            ("text", "muted_foreground", t.muted_foreground),
                            ("separator line", "border", t.border),
                            ("bottom border", "border", t.border),
                        ],
                        &[],
                        &[
                            ("icon size", "geometry::icon_size_small: defaults.icon_sizes.small"),
                            ("row gap", "hardcoded gap_2 (marker.rs, Marker::render)"),
                            ("shimmer", "the loading highlight ShimmerText paints, on the content slot only (marker.rs, Marker::render MarkerChild::Content)"),
                        ],
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
                                Button::new("tooltip-1").native(cx, geometry::button)
                                    .label("Hover me")
                                    .tooltip("This is a tooltip"),
                            )
                            .child(
                                Button::new("tooltip-2").native(cx, geometry::button)
                                    .label("With tooltip")
                                    .tooltip("Save file (Cmd+S)"),
                            )
                            // `Button::tooltip` takes a string and builds the
                            // tooltip itself (`button/button.rs:389`), so the
                            // only way to a refined one is to build it: that
                            // is what `geometry::tooltip` documents, and the
                            // one place the platform's tooltip padding, radius
                            // and text colour reach the popup.
                            .child({
                                let style = native_geometry(cx, geometry::tooltip);
                                div()
                                    .id("tooltip-built")
                                    .child(
                                        Button::new("tooltip-3")
                                            .native(cx, geometry::button)
                                            .label("Built by the application"),
                                    )
                                    .tooltip(move |window, cx| {
                                        refined(
                                            Tooltip::new(
                                                "This popup carries geometry::tooltip: the \
                                                 platform's max width, padding, radius, text \
                                                 size and text colour.",
                                            ),
                                            style.as_ref(),
                                        )
                                        .build(window, cx)
                                    })
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Tooltip",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("geometry", "geometry::tooltip on an application-built Tooltip: tooltip.max_width, border.padding_*, corner_radius, tooltip.font — including its colour, which upstream would otherwise paint with popover_foreground (tooltip.rs, Tooltip::render: text_color then refine_style)"),
                            ("delay", "hardcoded"),
                            ("position", "auto"),
                        ],
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
                            .child(probe(
                                PROBE_NOTIFICATION,
                                Button::new("notify-info").native(cx, geometry::button)
                                    .label("Info")
                                    .on_click(cx.listener(|_this, _ev, window, cx| {
                                        window.push_notification(
                                            Notification::info("This is an info notification.")
                                                .title("Info")
                                                .autohide(true),
                                            cx,
                                        );
                                    })),
                            ))
                            .child(Button::new("notify-success").native(cx, geometry::button).label("Success").on_click(
                                cx.listener(|_this, _ev, window, cx| {
                                    window.push_notification(
                                        Notification::success("Operation completed.")
                                            .title("Success")
                                            .autohide(true),
                                        cx,
                                    );
                                }),
                            ))
                            .child(Button::new("notify-warning").native(cx, geometry::button).label("Warning").on_click(
                                cx.listener(|_this, _ev, window, cx| {
                                    window.push_notification(
                                        Notification::warning("Careful with this action.")
                                            .title("Warning")
                                            .autohide(true),
                                        cx,
                                    );
                                }),
                            ))
                            .child(Button::new("notify-error").native(cx, geometry::button).label("Error").on_click(
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
    fn render_typography_tab(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
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
            // Code editor
            .child(section("Code editor (Rust)"))
            .child(
                div()
                    .id("tt-code-editor")
                    .child(Editor::new(&self.editor_state).h(px(240.0)))
                    .on_hover(self.hover_info(
                        &fi,
                        "Editor",
                        &[
                            ("border", "input", t.input),
                            ("text", "foreground", t.foreground),
                            ("caret", "caret", t.caret),
                            ("selection", "selection", t.selection),
                            ("line numbers", "muted_foreground", t.muted_foreground),
                        ],
                        &[
                            (
                                "mono font",
                                format!("mono_font_family: {}", t.mono_font_family),
                            ),
                            (
                                "mono size",
                                format!("mono_font_size: {}px", t.mono_font_size.as_f32()),
                            ),
                        ],
                        &[
                            ("bg", "highlight_theme's editor_background, else input_background()"),
                            ("syntax colors", "highlight_theme: default_light / default_dark per color mode; the grammar comes from the tree-sitter-rust dev feature"),
                            ("line height", "hardcoded 1.5 × mono_font_size"),
                            ("line numbers / search", "on by default"),
                        ],
                    )),
            )
            // Markdown
            .child(section("Markdown"))
            .child(
                div()
                    .id("tt-markdown")
                    .child(TextView::markdown("markdown-sample", MARKDOWN_SAMPLE).selectable(true))
                    .on_hover(self.hover_info(
                        &fi,
                        "Markdown (TextView)",
                        &[
                            ("text", "foreground", t.foreground),
                            ("link", "link", t.link),
                            ("code block bg", "muted", t.muted),
                            ("inline code bg", "accent", t.accent),
                            ("table border", "border", t.border),
                            ("table head", "table_head", t.table_head),
                        ],
                        &[
                            (
                                "border-radius",
                                format!("radius: {}px", t.radius.as_f32()),
                            ),
                            (
                                "mono font",
                                format!("mono_font_family: {}", t.mono_font_family),
                            ),
                        ],
                        &[("heading sizes", "derived from the base font size")],
                    )),
            )
    }

    /// One resizable group of the Layout tab, from its [`RESIZABLE_GROUPS`]
    /// entry. The box carries its own id as a debug selector so
    /// `resizable_groups_have_room_to_drag` can measure what was laid out.
    fn render_resizable_group(
        &self,
        group: &'static ResizableGroup,
        fi: &str,
        t: &Theme,
    ) -> impl IntoElement {
        let panels = group.panels.iter().map(|panel| {
            let body = v_flex()
                .p_3()
                .size_full()
                .child(Label::new(panel.title).font_semibold())
                .children(panel.caption.map(|c| Label::new(c).text_sm()));
            match panel.size {
                Some(size) => resizable_panel().size(px(size)).child(body),
                None => resizable_panel().child(body),
            }
        });
        let panel_group = match group.axis {
            Axis::Horizontal => h_resizable(group.group_id),
            Axis::Vertical => v_resizable(group.group_id),
        };
        div()
            .id(group.id)
            .debug_selector(|| group.id.into())
            .h(px(group.height))
            .border(px(RESIZABLE_BORDER))
            .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.3))
            .child(panel_group.children(panels))
            .on_hover(self.hover_info(
                fi,
                "Resizable",
                &[
                    ("dragging border", "drag_border", t.drag_border),
                    ("idle border", "border", t.border),
                ],
                &[],
                &[(
                    "min panel size",
                    "PANEL_MIN_SIZE = 100px (gpui-base resizable/mod.rs, PANEL_MIN_SIZE)",
                )],
            ))
    }

    // -----------------------------------------------------------------------
    // Tab: Layout
    // -----------------------------------------------------------------------
    fn render_layout_tab(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let accordion_title_style = native_geometry(cx, geometry::accordion_title);
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let collapsible_open = self.collapsible_open;
        // The four layout accessors. `None` where the platform specifies
        // nothing (platform-facts §2.20), and then the showcase's own spacing
        // stands — nothing is invented to fill the gap.
        let widget_gap = geometry::widget_gap(&self.layout);
        let container_margin = geometry::container_margin(&self.layout);
        let window_margin = geometry::window_margin(&self.layout);
        let section_gap = geometry::section_gap(&self.layout);
        let spacing_summary = format!(
            "widget_gap {} · container_margin {} · window_margin {} · section_gap {}",
            layout_value(widget_gap),
            layout_value(container_margin),
            layout_value(window_margin),
            layout_value(section_gap),
        );
        // The frame this very window is inside. `Root::new` sets
        // `bordered: true` (`root.rs:117`) and `Root::render` wraps everything
        // it holds in `window_border()` (`root.rs:605`), so the showcase's
        // `WindowBorder` is the window's own edge; a second, nested one would
        // set the client inset and lay down resize hit zones over the window a
        // second time.
        //
        // # shown-by: read, not constructed. `window_paddings` is
        // gpui-component's own `window_border::window_paddings`, and it returns
        // `window.client_inset()` (`window_border.rs:88-94`); the only writer of
        // that inset is `WindowBorder::render` (`window_border.rs:147-149`). The
        // `Some` the summary below reports is therefore live evidence that the
        // widget rendered around this window -- and under server-side
        // decorations, where upstream draws nothing, there is none.
        let decorations = window.window_decorations();
        let frame_insets = window_paddings(window);
        let client_inset = window.client_inset();
        let window_border_summary = format!(
            "{} · insets: top {}px, right {}px, bottom {}px, left {}px · {}",
            match decorations {
                gpui::Decorations::Server => "server-side decorations: a pass-through".to_string(),
                gpui::Decorations::Client { tiling } =>
                    format!("client-side decorations, tiled {tiling:?}"),
            },
            frame_insets.top.as_f32(),
            frame_insets.right.as_f32(),
            frame_insets.bottom.as_f32(),
            frame_insets.left.as_f32(),
            match client_inset {
                Some(inset) => format!(
                    "client inset {}px: the WindowBorder that Root renders around this window \
                     set it",
                    inset.as_f32()
                ),
                // Under server-side decorations the widget did render; it just
                // rendered as a pass-through, and only its client-side arm
                // calls set_client_inset (`window_border.rs:147-148`).
                None => match decorations {
                    gpui::Decorations::Server =>
                        "no client inset: the WindowBorder around this window rendered as a \
                         pass-through, and only its client-side arm sets one"
                            .to_string(),
                    gpui::Decorations::Client { .. } =>
                        "no client inset: nothing has called set_client_inset".to_string(),
                },
            },
        );
        // The resize band is the client-side arm's alone: the Server arm hands
        // back the bare backdrop div (`window_border.rs:172`) and the
        // compositor owns the edges.
        let window_border_hint = match decorations {
            gpui::Decorations::Server => {
                "The compositor draws this window's frame and resizes it; the widget lays no \
                 resize band of its own here."
            }
            gpui::Decorations::Client { .. } => {
                "Drag an edge of the window: the resize band is this widget's."
            }
        };
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            .child(section("WindowBorder (this window's own frame)"))
            .child(
                div()
                    .id("tt-window-border")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .child(
                                Label::new(SharedString::from(window_border_summary)).text_sm(),
                            )
                            .child(
                                Label::new(window_border_hint)
                                    .text_sm()
                                    .text_color(t.muted_foreground),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "WindowBorder",
                        &[("window bg", "background", t.background)],
                        &[],
                        &[
                            ("receiver", "Root::new sets bordered and Root::render wraps the window in window_border() (root.rs); nesting a second one would call set_client_inset again (window_border.rs, WindowBorder::render)"),
                            ("frame colour", "hardcoded grey, l=0.2 dark / l=0.8 light (window_border.rs, WindowBorder::render border_color)"),
                            ("shadow", "hardcoded two-layer box shadow (window_border.rs, WindowBorder::render shadow(vec![..]))"),
                            ("server-side decorations", "nothing is drawn: the compositor owns the frame (window_border.rs, WindowBorder::render Decorations::Server arm)"),
                        ],
                    )),
            )
            // Window chrome, stacked the way a window stacks it: the title bar
            // above, a toolbar the application draws itself, the status bar
            // below. Each is bounded in its own section like every other
            // widget here — and the title bar is a real one, so it carries
            // upstream's window handlers with it.
            .child(section("TitleBar (a real one: dragging it moves the window)"))
            .child(
                div()
                    .id("tt-title-bar")
                    .border_1()
                    .border_color(t.border)
                    .child(
                        TitleBar::new()
                            .native(cx, geometry::title_bar)
                            // Linux only (`title_bar.rs:99-101` drops the
                            // handler on every other platform), and the reason
                            // this bar can be shown at all there: without a
                            // handler the X calls `window.remove_window()`
                            // (`:237`), which would close the showcase.
                            // Upstream's other handlers — drag to move, double
                            // click to zoom — are the widget, and stay.
                            //
                            // On Windows the handler is discarded and the
                            // controls are hit-tested by the OS instead
                            // (`window_control_area`, `:220-222`), so this
                            // demo bar's minimise, maximise and close act on
                            // the real window. Nothing can intercept them
                            // short of not drawing the widget, so the note
                            // below says so on that platform. On macOS
                            // upstream draws no controls at all (`:254-256`).
                            .on_close_window(cx.listener(|_this, _ev, window, cx| {
                                window.push_notification(
                                    Notification::info(
                                        "A nested title bar for the geometry builder; \
                                         its close button is deliberately inert.",
                                    )
                                    .title("TitleBar")
                                    .autohide(true),
                                    cx,
                                );
                            }))
                            // No `.text_sm()`: `Label::render` applies its own
                            // refinement last (`label.rs:208`), so a size set
                            // here would cancel the one `geometry::title_bar`
                            // just supplied.
                            .child(Label::new("native-theme showcase")),
                    )
                    .child(Label::new(TITLE_BAR_CONTROLS_NOTE).text_sm().text_color(
                        if cfg!(target_os = "windows") {
                            t.danger
                        } else {
                            t.muted_foreground
                        },
                    ))
                    .on_hover(self.hover_info(
                        &fi,
                        "TitleBar",
                        &[
                            ("bg", "title_bar", t.title_bar),
                            ("border", "title_bar_border", t.title_bar_border),
                            ("text", "foreground", t.foreground),
                            ("control hover", "secondary_hover", t.secondary_hover),
                            ("close hover", "danger", t.danger),
                        ],
                        &[],
                        &[
                            ("geometry", "geometry::title_bar: window.title_bar_font size and weight, carried by the label because nothing overrides it afterwards; the colour is the inherited foreground, which every preset states as the title bar's own"),
                            ("height", "TITLE_BAR_HEIGHT = 34px (title_bar.rs, TITLE_BAR_HEIGHT)"),
                            ("fill", "a gradient between title_bar and background (title_bar.rs, default_title_bar_background)"),
                            ("window controls", TITLE_BAR_CONTROLS_NOTE),
                        ],
                    )),
            )
            .child(section("Toolbar (no widget upstream: the application draws it)"))
            .child(
                div()
                    .id("tt-toolbar")
                    .child({
                        // The row is drawn with the application's own
                        // elements, so its spacing is the application's to
                        // set: the platform's container padding inside it and
                        // its widget gap between the icons.
                        let row = with_padding(
                            with_gap(h_flex(), widget_gap),
                            container_margin,
                        )
                        .items_center()
                        .bg(t.tab_bar)
                        .border_1()
                        .border_color(t.border)
                        .child(native_icon(cx, IconName::Search, geometry::icon_size_toolbar))
                        .child(native_icon(cx, IconName::Copy, geometry::icon_size_toolbar))
                        .child(native_icon(cx, IconName::Settings, geometry::icon_size_toolbar))
                        .child(Separator::vertical())
                        .child(Label::new(
                            "Toolbar icons at the platform's toolbar size",
                        ));
                        // gpui-component has no toolbar widget, so there is no
                        // refinement to apply: the row's own height is
                        // `geometry::control_height` of the platform's button,
                        // the same derivation `geometry::button` uses.
                        match native_control_height(cx) {
                            Some(height) => row.h(height),
                            None => row,
                        }
                    })
                    .on_hover(self.hover_info(
                        &fi,
                        "Toolbar (application-drawn)",
                        &[("bg", "tab_bar", t.tab_bar), ("border", "border", t.border)],
                        &[],
                        &[
                            ("height", "geometry::control_height(button.min_height, button.font, button.border)"),
                            ("padding", "geometry::container_margin; gap: geometry::widget_gap"),
                            ("icon size", "geometry::icon_size_toolbar: defaults.icon_sizes.toolbar"),
                        ],
                    )),
            )
            .child(section("StatusBar"))
            .child(
                div()
                    .id("tt-status-bar")
                    .border_1()
                    .border_color(t.border)
                    .child(
                        StatusBar::new()
                            .native(cx, geometry::status_bar)
                            .left(native_icon(cx, IconName::Inbox, geometry::icon_size_small))
                            .left("6 items")
                            .child("native-theme showcase")
                            .right("UTF-8"),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "StatusBar",
                        &[
                            ("bg", "status_bar", t.status_bar),
                            ("border", "status_bar_border", t.status_bar_border),
                            ("upstream text", "muted_foreground", t.muted_foreground),
                        ],
                        &[],
                        &[
                            ("geometry", "geometry::status_bar: status_bar.border.padding_*, status_bar.font — including its colour, which upstream would otherwise paint with muted_foreground (status_bar.rs, StatusBar::render: text_color then refine_style)"),
                            ("icon size", "geometry::icon_size_small: defaults.icon_sizes.small"),
                            ("region gap", "hardcoded gap_2 (status_bar.rs, StatusBar::render region)"),
                        ],
                    )),
            )
            // The four layout accessors, applied rather than printed: the outer
            // box takes the platform's window margin, the row inside it the
            // container margin and the widget gap, and the two rows are
            // separated by the section gap.
            .child(section("Layout spacing (the four LayoutTheme accessors)"))
            .child(
                div()
                    .id("tt-layout-spacing")
                    .child(
                        with_padding(
                            with_gap(
                                v_flex().border_1().border_color(t.border),
                                section_gap,
                            ),
                            window_margin,
                        )
                        .child(
                            with_padding(
                                with_gap(
                                    h_flex().border_1().border_color(t.border),
                                    widget_gap,
                                ),
                                container_margin,
                            )
                            .child(Button::new("ls-a").native(cx, geometry::button).label("One"))
                            .child(Button::new("ls-b").native(cx, geometry::button).label("Two"))
                            .child(Button::new("ls-c").native(cx, geometry::button).label("Three")),
                        )
                        .child(Label::new(SharedString::from(spacing_summary)).text_sm()),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Layout spacing",
                        &[("border", "border", t.border)],
                        &[],
                        &[(
                            "receivers",
                            "no gpui-component widget reads the gpui-base spacing tokens, so these four are the application's to apply (geometry.rs §9.5)",
                        )],
                    )),
            )
            // The resizable groups, each from its entry in RESIZABLE_GROUPS.
            .children(RESIZABLE_GROUPS.iter().map(|group| {
                v_flex()
                    .gap_5()
                    .child(section(group.heading))
                    .child(self.render_resizable_group(group, &fi, &t))
            }))
            // Dividers
            .child(section("Separator (solid / dashed / labeled)"))
            .child(
                div()
                    .id("tt-layout-divider")
                    .child(
                        v_flex()
                            .gap_3()
                            .child(Separator::horizontal())
                            .child(Separator::horizontal().label("Section Break"))
                            .child(Separator::horizontal_dashed()),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Separator",
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
                        native_group_box(cx)
                        .title("Contained Content")
                        .fill()
                        .child(
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
                                        .child(Button::new("gb-1").native(cx, geometry::button).label("Action A"))
                                        .child(Button::new("gb-2").native(cx, geometry::button).label("Action B").primary()),
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
                        &[("geometry", "geometry::group_box_content: card.border.padding_*, corner_radius, line_width, color")],
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
                                format!("scrollbar_mode: {:?}", t.scrollbar_mode),
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
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("What is native-theme?")
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
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("Supported toolkits")
                                    .child(
                                        Label::new("gpui-component, iced, egui, and more planned.")
                                            .text_sm(),
                                    )
                            })
                            .item(|item| {
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("How many presets?")
                                    .child(
                                        // Counted from the list itself, so the
                                        // answer cannot go stale again.
                                        Label::new(SharedString::from(format!(
                                            "{} built-in theme presets covering major OS styles.",
                                            native_theme::theme::Theme::list_presets().len()
                                        )))
                                        .text_sm(),
                                    )
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Accordion",
                        &[
                            ("bg", "accordion", t.accordion),
                            ("border", "border", t.border),
                            ("text", "foreground", t.foreground),
                            ("secondary text", "muted_foreground", t.muted_foreground),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("header height", "geometry::accordion_title: expander.header_height"),
                            ("padding", "inner (Tier U)"),
                            ("animation", "hardcoded"),
                        ],
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
                            ("border", "border", t.border),
                        ],
                        &[],
                        &[("animation", "hardcoded slide")],
                    )),
            )
            // Carousel
            .child(section("Carousel"))
            .child(
                div()
                    .id("tt-carousel")
                    // The slide controls are positioned outside the frame, so
                    // the section leaves a button's width on either side.
                    .px_16()
                    .child(
                        Carousel::new("carousel", &self.carousel_state)
                            .w(px(360.0))
                            .child(
                                CarouselContent::new(&self.carousel_state)
                                    .h(px(120.0))
                                    .children(CAROUSEL_SLIDES.iter().enumerate().map(
                                        |(ix, (title, body))| {
                                            CarouselItem::new(
                                                ("carousel-slide", ix),
                                                ix,
                                                &self.carousel_state,
                                            )
                                            .child(
                                                v_flex()
                                                    .size_full()
                                                    .justify_center()
                                                    .gap_1()
                                                    .p_4()
                                                    .rounded(t.radius)
                                                    .bg(t.muted)
                                                    .child(Label::new(*title).font_semibold())
                                                    .child(
                                                        Label::new(*body)
                                                            .text_sm()
                                                            .text_color(t.muted_foreground),
                                                    ),
                                            )
                                        },
                                    )),
                            )
                            .child(CarouselPagination::new().children(
                                (0..CAROUSEL_SLIDES.len()).map(|ix| {
                                    let dot = CarouselPaginationItem::new(
                                        ("carousel-page", ix),
                                        ix,
                                        &self.carousel_state,
                                    )
                                    .child(SharedString::from((ix + 1).to_string()));
                                    // The last dot is the self-test's way into
                                    // the carousel: the prev/next controls
                                    // place themselves absolutely outside the
                                    // frame, so a wrapper around one of those
                                    // would take it out of the flow.
                                    match ix == CAROUSEL_SLIDES.len() - 1 {
                                        true => probe(PROBE_CAROUSEL_LAST, dot).into_any_element(),
                                        false => dot.into_any_element(),
                                    }
                                }),
                            ))
                            // The carousel's own slide controls. They take the
                            // same state as the viewport and position
                            // themselves outside the frame
                            // (`carousel/carousel.rs:757-768`), so they belong
                            // to the carousel rather than beside it.
                            .child(CarouselPrevious::new(&self.carousel_state))
                            .child(CarouselNext::new(&self.carousel_state)),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Carousel",
                        &[
                            ("slide bg", "muted", t.muted),
                            ("slide text", "foreground", t.foreground),
                            ("slide caption", "muted_foreground", t.muted_foreground),
                            ("focus ring", "ring", t.ring),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("snap motion", "Theme::motion spring_move; ResolvedTheme has no motion field"),
                            ("reduced motion", "gpui's App::reduce_motion, forwarded by apply_system_theme — the snap becomes instant"),
                            ("slide controls", "outline Buttons the widget builds itself, disabled at the ends (carousel/carousel.rs, carousel_control)"),
                        ],
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
                                native_group_box(cx)
                                    .title("Default")
                                    .w(px(180.0))
                                    .child(Label::new("Default style").text_sm()),
                            )
                            .child(
                                native_group_box(cx)
                                    .title("Filled")
                                    .fill()
                                    .w(px(180.0))
                                    .child(Label::new("Filled background").text_sm()),
                            )
                            .child(
                                native_group_box(cx)
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
                        &[("geometry", "geometry::group_box_content: card.border.padding_*, corner_radius, line_width, color")],
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
                                    this.active_tab = Tab::Buttons;
                                },
                            )))
                            .child(BreadcrumbItem::new("Inputs").on_click(cx.listener(
                                |this, _ev, _w, _cx| {
                                    this.active_tab = Tab::Inputs;
                                },
                            )))
                            .child(BreadcrumbItem::new("Data").on_click(cx.listener(
                                |this, _ev, _w, _cx| {
                                    this.active_tab = Tab::Data;
                                },
                            )))
                            .child(BreadcrumbItem::new("Feedback").on_click(cx.listener(
                                |this, _ev, _w, _cx| {
                                    this.active_tab = Tab::Feedback;
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
            // Stepper
            .child(section(format!(
                "Stepper (step {} of {}: one completed, one current, one pending)",
                self.step + 1,
                STEPPER_STEPS.len()
            )))
            .child(
                div()
                    .id("tt-stepper")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(480.0))
                            .child(probe(
                                PROBE_STEPPER,
                                Stepper::new("stepper-1")
                                    .selected_index(self.step)
                                    .items(STEPPER_STEPS.iter().map(|(label, icon)| {
                                        // The indicator is a circle
                                        // (stepper/trigger.rs:118-123) around
                                        // the icon it is given, which keeps its
                                        // size (`:138-139`).
                                        StepperItem::new()
                                            .icon(native_icon(
                                                cx,
                                                icon.clone(),
                                                geometry::icon_size_small,
                                            ))
                                            .child(Label::new(*label).text_sm())
                                    }))
                                    .on_click(cx.listener(|this, step: &usize, _w, cx| {
                                        this.step = *step;
                                        cx.notify();
                                    })),
                            ))
                            .child(
                                Stepper::new("stepper-vertical")
                                    .vertical()
                                    .selected_index(self.step)
                                    .items(STEPPER_STEPS.iter().map(|(label, _)| {
                                        StepperItem::new().child(Label::new(*label).text_sm())
                                    }))
                                    .on_click(cx.listener(|this, step: &usize, _w, cx| {
                                        this.step = *step;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Stepper",
                        &[
                            ("completed / current", "primary", t.primary),
                            ("completed text", "primary_foreground", t.primary_foreground),
                            ("pending", "secondary", t.secondary),
                            ("pending text", "secondary_foreground", t.secondary_foreground),
                            ("pending hover", "secondary_hover", t.secondary_hover),
                        ],
                        &[],
                        &[
                            ("icon size", "geometry::icon_size_small: defaults.icon_sizes.small"),
                            ("indicator size", "24px for Size::Medium (stepper/item.rs, StepperItem::render icon_size)"),
                            ("separator", "drawn by the item, absolute (stepper/item.rs: StepperItem::render builds it, StepperSeparator::render positions it)"),
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
                            .child(
                                Field::new().label("Name").required(true).child(
                                    Input::new(&self.form_name_state)
                                        .native(cx, geometry::input),
                                ),
                            )
                            .child(
                                Field::new()
                                    .label("Email")
                                    .description("We will never share your email.")
                                    .child(
                                        Input::new(&self.form_email_state)
                                            .native(cx, geometry::input),
                                    ),
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
            .child(section(
                "Sidebar (mini navigation; the toggle button collapses it)",
            ))
            .child(
                div()
                    .id("tt-sidebar-toggle")
                    .child(
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            // `SidebarToggleButton` owns no collapsed state of
                            // its own (`sidebar/mod.rs:302-307`): the flag it
                            // draws and the flag the sidebar reads are the
                            // same one, here.
                            .child(probe(
                                PROBE_SIDEBAR_TOGGLE,
                                SidebarToggleButton::new()
                                    .collapsed(self.sidebar_collapsed)
                                    .on_click(cx.listener(|this, _ev, _w, cx| {
                                        this.sidebar_collapsed = !this.sidebar_collapsed;
                                        cx.notify();
                                    })),
                            ))
                            .child(
                                Label::new(if self.sidebar_collapsed {
                                    "collapsed"
                                } else {
                                    "expanded"
                                })
                                .text_sm()
                                .text_color(t.muted_foreground),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "SidebarToggleButton",
                        &[
                            ("bg", "transparent until hover", t.transparent),
                            ("hover", "secondary_hover", t.secondary_hover),
                            ("icon", "foreground", t.foreground),
                        ],
                        &[],
                        &[
                            ("button", "a ghost, small Button built by the widget (sidebar/mod.rs, SidebarToggleButton::new)"),
                            ("icon", "PanelLeftOpen / PanelLeftClose, at a hardcoded size_4 (sidebar/mod.rs, SidebarToggleButton::render)"),
                        ],
                    )),
            )
            .child(
                div()
                    .id("tt-sidebar")
                    .h(px(240.0))
                    .w(px(280.0))
                    .border_1()
                    .border_color(t.border)
                    .overflow_hidden()
                    .child(
                        // A sidebar is the panel `defaults.icon_sizes.panel`
                        // names, and `SidebarMenuItem` keeps the icon it is
                        // given (`sidebar/menu.rs:300`), so the size arrives.
                        Sidebar::new("layout-sidebar")
                            .collapsed(self.sidebar_collapsed)
                            .child(
                            SidebarMenu::new()
                                .child(
                                    SidebarMenuItem::new("Dashboard")
                                        .icon(native_icon(
                                            cx,
                                            IconName::LayoutDashboard,
                                            geometry::icon_size_panel,
                                        ))
                                        .active(true),
                                )
                                .child(SidebarMenuItem::new("Settings").icon(native_icon(
                                    cx,
                                    IconName::Settings,
                                    geometry::icon_size_panel,
                                )))
                                .child(SidebarMenuItem::new("Inbox").icon(native_icon(
                                    cx,
                                    IconName::Inbox,
                                    geometry::icon_size_panel,
                                )))
                                .child(SidebarMenuItem::new("Calendar").icon(native_icon(
                                    cx,
                                    IconName::Calendar,
                                    geometry::icon_size_panel,
                                ))),
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
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let widget_gap = geometry::widget_gap(&self.layout);
        let container_margin = geometry::container_margin(&self.layout);
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
            .child(section("Dialog"))
            .child(
                div()
                    .id("tt-dialog")
                    .child(
                        Button::new("open-dialog")
                            .label("Open Dialog")
                            .on_click(cx.listener(|this, _ev, window, cx| {
                                let widget_gap = geometry::widget_gap(&this.layout);
                                window.open_dialog(cx, move |dialog, _w, cx| {
                                    let n = cx.native_theme().and_then(|t| t.native(cx));
                                    let dialog =
                                        dialog
                                            .title(match n {
                                                Some(n) => DialogTitle::new()
                                                    .refine_style(&geometry::dialog_title(n))
                                                    .child("Confirm Action"),
                                                None => DialogTitle::new().child("Confirm Action"),
                                            })
                                            .w(px(400.0))
                                            // The description is where the
                                            // platform's dialog body font and
                                            // its dialog icon size land;
                                            // upstream would paint the text
                                            // with `muted_foreground`
                                            // (`dialog/description.rs:50-51`).
                                            .content(move |content, _w, cx| {
                                                content.child(
                                                    with_gap(h_flex(), widget_gap)
                                                        .items_start()
                                                        .child(native_icon(
                                                            cx,
                                                            IconName::CircleX,
                                                            geometry::icon_size_dialog,
                                                        ))
                                                        .child(refined(
                                                            DialogDescription::new()
                                                                .child("This cannot be undone."),
                                                            native_geometry(
                                                                cx,
                                                                geometry::dialog_description,
                                                            )
                                                            .as_ref(),
                                                        )),
                                                )
                                            })
                                            .footer(
                                                match n {
                                                    Some(n) => DialogFooter::new()
                                                        .refine_style(&geometry::dialog_footer(n)),
                                                    None => DialogFooter::new(),
                                                }
                                                .child(DialogClose::new().child(
                                                    Button::new("dialog-close").label("Close"),
                                                )),
                                            );
                                    match n {
                                        Some(n) => dialog
                                            .refine_style(&geometry::dialog(n))
                                            .max_w(geometry::dialog_max_width(n)),
                                        None => dialog,
                                    }
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
            // AlertDialog
            .child(section(match &self.alert_choice {
                Some(choice) => format!("AlertDialog (last answered: {choice})"),
                None => "AlertDialog (not answered yet)".to_string(),
            }))
            .child(
                div()
                    .id("tt-alert-dialog")
                    .child(probe(
                        PROBE_ALERT_DIALOG,
                        Button::new("open-alert-dialog")
                            .native(cx, geometry::button)
                            .danger()
                            .label("Discard changes…")
                            .on_click(cx.listener(|_this, _ev, window, cx| {
                                let this = cx.weak_entity();
                                window.open_alert_dialog(cx, move |alert: AlertDialog, _w, cx| {
                                    let n = cx.native_theme().and_then(|t| t.native(cx));
                                    let ok = this.clone();
                                    let cancel = this.clone();
                                    let alert = alert
                                        .icon(native_icon(
                                            cx,
                                            IconName::TriangleAlert,
                                            geometry::icon_size_dialog,
                                        ))
                                        .title("Discard changes?")
                                        .description(
                                            "The edits made since the last save will be lost.",
                                        )
                                        .button_props(
                                            DialogButtonProps::default()
                                                .ok_text("Discard")
                                                .ok_variant(ButtonVariant::Danger)
                                                .cancel_text("Keep")
                                                .show_cancel(true),
                                        )
                                        .on_ok(move |_ev, _w, cx| {
                                            ok.update(cx, |this, cx| {
                                                this.alert_choice = Some("Discard".into());
                                                cx.notify();
                                            })
                                            .ok();
                                            true
                                        })
                                        .on_cancel(move |_ev, _w, cx| {
                                            cancel
                                                .update(cx, |this, cx| {
                                                    this.alert_choice = Some("Keep".into());
                                                    cx.notify();
                                                })
                                                .ok();
                                            true
                                        });
                                    match n {
                                        Some(n) => alert
                                            .refine_style(&geometry::dialog(n))
                                            .max_w(geometry::dialog_max_width(n)),
                                        None => alert,
                                    }
                                });
                            })),
                    ))
                    .on_hover(self.hover_info(
                        &fi,
                        "AlertDialog",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                            ("description", "muted_foreground", t.muted_foreground),
                            ("overlay", "overlay", t.overlay),
                            ("confirm button", "danger", t.danger),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("geometry", "geometry::dialog and geometry::dialog_max_width, as the Dialog above"),
                            ("icon size", "geometry::icon_size_dialog: defaults.icon_sizes.dialog"),
                            ("footer", "right-aligned, and built from button_props when none is given (dialog/alert_dialog.rs, AlertDialog::build_surface; dialog/footer.rs, DialogFooter::render justify_end)"),
                            ("dismissal", "no backdrop close by design (dialog/alert_dialog.rs, AlertDialog::overlay_closable, deprecated)"),
                        ],
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
                                    .native(cx, geometry::button)
                                    .label("Open Sheet (Right)")
                                    .on_click(cx.listener(|_this, _ev, window, cx| {
                                        window.open_sheet(cx, |sheet, _w, _cx| {
                                            sheet.title("Sheet Panel").size(px(320.0))
                                        });
                                    })),
                            )
                            .child(
                                Button::new("open-sheet-bottom")
                                    .native(cx, geometry::button)
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
                            .native(cx, geometry::popover)
                            .trigger(
                                Button::new("popover-trigger")
                                    .native(cx, geometry::button)
                                    .label("Click for Popover"),
                            )
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
            // HoverCard
            .child(section("HoverCard (hover the trigger, no click)"))
            .child(
                div()
                    .id("tt-hover-card")
                    .child(
                        HoverCard::new("hover-card-1")
                            .native(cx, geometry::popover)
                            .trigger(
                                Button::new("hover-card-trigger")
                                    .native(cx, geometry::button)
                                    .label("KDE Breeze")
                                    .custom(variants::ghost_button(cx)),
                            )
                            // The card's own padding and the gap inside it are
                            // the application's to set, so they take the
                            // platform's container margin and widget gap.
                            .content(move |_state, _w, cx| {
                                with_padding(
                                    with_gap(v_flex(), widget_gap),
                                    container_margin,
                                )
                                    .w(px(260.0))
                                    .child(Label::new("KDE Breeze").font_semibold())
                                    .child(
                                        Label::new(
                                            "The Plasma preset: kdeglobals for the palette, \
                                             Breeze for the icons.",
                                        )
                                        .text_sm()
                                        .text_color(cx.theme().muted_foreground),
                                    )
                            }),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "HoverCard",
                        &[
                            ("bg", "popover", t.popover),
                            ("text", "popover_foreground", t.popover_foreground),
                            ("secondary text", "muted_foreground", t.muted_foreground),
                            ("border", "border", t.border),
                        ],
                        &[("border-radius", format!("radius: {}px", t.radius.as_f32()))],
                        &[
                            ("geometry", "geometry::popover, which refines the card surface (hover_card.rs, HoverCard::render refine_style)"),
                            ("card padding", "geometry::container_margin; gap: geometry::widget_gap"),
                            ("trigger", "variants::ghost_button, the flat button's native state colours"),
                            ("delays", "600ms to open, 300ms to close (hover_card.rs, HoverCard::new)"),
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
                            ("rows", "PopupMenu builds its own; geometry::menu_item has no receiver here (geometry.rs, menu/menu_item.rs: MenuItemElement is pub(crate))"),
                        ],
                    )),
            )
            // The menu rows an application draws itself. Upstream's
            // `MenuItemElement` is crate-private and `PopupMenu` builds its
            // own rows, so `geometry::menu_item` has no widget to refine —
            // these rows are the receiver it documents.
            .child(section("Menu rows (drawn by the application)"))
            .child(
                div()
                    .id("tt-menu-rows")
                    .child({
                        let row_style = native_geometry(cx, geometry::menu_item);
                        let row = |id: &'static str, icon: IconName, label: &'static str| {
                            refined(
                                div()
                                    .id(id)
                                    .flex()
                                    .items_center()
                                    .hover(|this| this.bg(t.accent))
                                    .child(native_icon(cx, icon, geometry::icon_size_small))
                                    // No `.text_sm()`: a `Label` refines
                                    // itself last (`label.rs:208`), so the row
                                    // would keep its own size instead of
                                    // `menu.font`'s.
                                    .child(Label::new(label)),
                                row_style.as_ref(),
                            )
                        };
                        v_flex()
                            .w(px(220.0))
                            .bg(t.popover)
                            .text_color(t.popover_foreground)
                            .border_1()
                            .border_color(t.border)
                            .rounded(t.radius)
                            .child(row("mi-cut", IconName::Delete, "Cut"))
                            .child(row("mi-copy", IconName::Copy, "Copy"))
                            .child(row("mi-paste", IconName::Inbox, "Paste"))
                    })
                    .on_hover(self.hover_info(
                        &fi,
                        "Menu row (application-drawn)",
                        &[
                            ("bg", "popover", t.popover),
                            ("hover", "accent", t.accent),
                            ("text", "popover_foreground", t.popover_foreground),
                            ("border", "border", t.border),
                        ],
                        &[],
                        &[
                            ("geometry", "geometry::menu_item: menu.row_height (control height), menu.border.padding_*, menu.icon_text_gap, menu.font — the label sets no size of its own, so the font arrives"),
                            ("icon size", "geometry::icon_size_small: defaults.icon_sizes.small"),
                        ],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Charts
    // -----------------------------------------------------------------------
    fn render_charts_tab(&self, cx: &mut Context<Self>) -> impl IntoElement + InteractiveElement {
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
                            .band(|d: &MonthData| d.month.clone())
                            .value(|d: &MonthData| d.value)
                            .fill(move |_: &MonthData, _, _, _| bar_fill),
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
                            ("bullish", "chart_bullish", t.chart_bullish),
                            ("bearish", "chart_bearish", t.chart_bearish),
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
    fn render_icons_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement + InteractiveElement {
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
                // cover all 101 icons via by-name lookup (a set without an equivalent
                // shows NotFound) — no mixing of sets.
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
            .child(Separator::horizontal())
            // Native Theme Icons section
            .child(section(native_section_title))
            .child(
                div()
                    .id("native-icons-grid")
                    .child(div().flex().flex_wrap().gap_2().children(native_icon_cells)),
            )
            .child(Separator::horizontal())
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
                            ("SVG shapes", "101 built-in Lucide icons from gpui-kit"),
                        ],
                    )),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Theme Map
    // -----------------------------------------------------------------------
    fn render_theme_map_tab(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
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
            // Button (0.6.0): button*/button_secondary* ← secondary*, button_primary*
            // ← primary*, button_{danger,info,success,warning}* ← the status
            // fields (spec §6.2) — solid native surfaces, not upstream's tint.
            .child(section(
                "Button (28 fields, copies of secondary/primary/status)",
            ))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("button", t.button))
                    .child(color_swatch("button_hover", t.button_hover))
                    .child(color_swatch("button_active", t.button_active))
                    .child(color_swatch("button_foreground", t.button_foreground))
                    .child(color_swatch("button_secondary", t.button_secondary))
                    .child(color_swatch(
                        "button_secondary_hover",
                        t.button_secondary_hover,
                    ))
                    .child(color_swatch(
                        "button_secondary_active",
                        t.button_secondary_active,
                    ))
                    .child(color_swatch(
                        "button_secondary_foreground",
                        t.button_secondary_foreground,
                    ))
                    .child(color_swatch("button_primary", t.button_primary))
                    .child(color_swatch("button_primary_hover", t.button_primary_hover))
                    .child(color_swatch(
                        "button_primary_active",
                        t.button_primary_active,
                    ))
                    .child(color_swatch(
                        "button_primary_foreground",
                        t.button_primary_foreground,
                    ))
                    .child(color_swatch("button_danger", t.button_danger))
                    .child(color_swatch("button_danger_hover", t.button_danger_hover))
                    .child(color_swatch("button_danger_active", t.button_danger_active))
                    .child(color_swatch(
                        "button_danger_foreground",
                        t.button_danger_foreground,
                    ))
                    .child(color_swatch("button_info", t.button_info))
                    .child(color_swatch("button_info_hover", t.button_info_hover))
                    .child(color_swatch("button_info_active", t.button_info_active))
                    .child(color_swatch(
                        "button_info_foreground",
                        t.button_info_foreground,
                    ))
                    .child(color_swatch("button_success", t.button_success))
                    .child(color_swatch("button_success_hover", t.button_success_hover))
                    .child(color_swatch(
                        "button_success_active",
                        t.button_success_active,
                    ))
                    .child(color_swatch(
                        "button_success_foreground",
                        t.button_success_foreground,
                    ))
                    .child(color_swatch("button_warning", t.button_warning))
                    .child(color_swatch("button_warning_hover", t.button_warning_hover))
                    .child(color_swatch(
                        "button_warning_active",
                        t.button_warning_active,
                    ))
                    .child(color_swatch(
                        "button_warning_foreground",
                        t.button_warning_foreground,
                    )),
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
                    .child(color_swatch("list", t.colors.list))
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
                    // table_foot* mirror table_head* (spec §6.2 derivation)
                    .child(color_swatch("table_foot", t.table_foot))
                    .child(color_swatch(
                        "table_foot_foreground",
                        t.table_foot_foreground,
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
                    .child(color_swatch("accordion", t.accordion)),
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
                    .child(color_swatch("chart_bullish", t.chart_bullish))
                    .child(color_swatch("chart_bearish", t.chart_bearish)),
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
                    // status_bar* ← status_bar.background_color / .border.color (spec §6.2)
                    .child(color_swatch("status_bar", t.status_bar))
                    .child(color_swatch("status_bar_border", t.status_bar_border))
                    .child(color_swatch("title_bar", t.title_bar))
                    .child(color_swatch("title_bar_border", t.title_bar_border))
                    .child(color_swatch("window_border", t.window_border)),
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
        // After the deferred system-theme change above, so the selects and the
        // rest of the frame read the same installed theme.
        let select_style = native_geometry(cx, geometry::select);

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
                        refined(Select::new(&self.theme_select), select_style.as_ref())
                            .with_size(Size::Small)
                            .w_full(),
                    )
                    .child(probe(
                        PROBE_COLOR_MODE,
                        refined(Select::new(&self.dark_mode_select), select_style.as_ref())
                            .with_size(Size::Small)
                            .w_full(),
                    ))
                    .child(Separator::horizontal()),
            )
            .child(
                v_flex()
                    .p_3()
                    .gap_3()
                    .child(Label::new("Icon Theme").text_size(px(13.0)).font_semibold())
                    .child(
                        refined(Select::new(&self.icon_set_select), select_style.as_ref())
                            .with_size(Size::Small)
                            .w_full(),
                    )
                    .child(Separator::horizontal()),
            )
            .child(self.render_sidebar(window, cx))
            .child(Separator::horizontal())
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
                                .children(Tab::ALL.map(Tab::label))
                                .selected_index(active_tab.index())
                                .on_click(cx.listener(|this, ix: &usize, _window, _cx| {
                                    if let Some(tab) = Tab::at(*ix) {
                                        this.active_tab = tab;
                                    }
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
                    // TAB_ROOT is what `every_tab_lays_out` looks the tab up
                    // by, so every arm carries it.
                    .child(match active_tab {
                        Tab::Buttons => self
                            .render_buttons_tab(window, cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::Inputs => self
                            .render_inputs_tab(window, cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::Data => self
                            .render_data_tab(window, cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::Feedback => self
                            .render_feedback_tab(window, cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::Typography => self
                            .render_typography_tab(cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::Layout => self
                            .render_layout_tab(window, cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::Overlays => self
                            .render_overlays_tab(window, cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::Charts => self
                            .render_charts_tab(cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::Icons => self
                            .render_icons_tab(cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                        Tab::ThemeMap => self
                            .render_theme_map_tab(cx)
                            .debug_selector(|| TAB_ROOT.into())
                            .into_any_element(),
                    }),
            );

        // Main layout: horizontal split with sidebar + content, and above it
        // the three layers `Root` keeps but does not draw. `Root::render`
        // renders only the view it was given (root.rs, Root::render), so a
        // dialog, a sheet or a notification the showcase pushes reaches the
        // screen only because these three are here -- upstream's own dialog
        // test builds its host the same way (dialog/dialog.rs, DialogHost).
        div()
            .relative()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(h_flex().size_full().child(sidebar).child(content))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
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

    /// Map a `--tab` name to the tab it names.
    fn tab(name: &str) -> Option<Tab> {
        match name {
            "buttons" => Some(Tab::Buttons),
            "inputs" | "text-inputs" => Some(Tab::Inputs),
            "data" => Some(Tab::Data),
            "feedback" => Some(Tab::Feedback),
            "typography" => Some(Tab::Typography),
            "layout" => Some(Tab::Layout),
            "overlays" => Some(Tab::Overlays),
            "charts" => Some(Tab::Charts),
            "icons" => Some(Tab::Icons),
            "theme-map" => Some(Tab::ThemeMap),
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

        for chunk in pixels.as_chunks_mut::<4>().0 {
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

    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx: &mut App| {
            gpui_kit::init(cx);

            // Apply CLI variant override before window opens so the initial
            // theme is resolved with the correct light/dark setting.
            let variant_override = cli_args.variant.as_deref().map(|v| v == "dark");

            let bounds = Bounds::centered(None, WINDOW_SIZE, cx);
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
                            && let Some(tab) = CliArgs::tab(tab_name)
                        {
                            s.active_tab = tab;
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
                        cx.background_executor()
                            .timer(Duration::from_millis(200))
                            .await;
                        nudge_content_size(1.0, 0.0);
                        cx.background_executor()
                            .timer(Duration::from_millis(1300))
                            .await;
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
                        cx.background_executor()
                            .timer(Duration::from_millis(1500))
                            .await;
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

// ---------------------------------------------------------------------------
// Showcase self-tests (spec v0.5.9 §6.1)
// ---------------------------------------------------------------------------
//
// `test = true` on the example target (Cargo.toml) puts these under a plain
// `cargo test`, so CI and the nightly dependency canary run them with no
// workflow change. They build the real `Showcase` on GPUI's headless test
// platform — the same view, the same `Root`, the same window width `main`
// opens — and drive it with real input.

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, Point, TestAppContext, VisualTestContext, point};
    use gpui_base::PANEL_MIN_SIZE;
    use std::cell::RefCell;
    use std::ops::Deref as _;
    use std::rc::Rc;

    /// The window the self-tests lay the showcase out in.
    ///
    /// The width is the showcase's own, so the horizontal resizable group is
    /// measured at the width the application gives it. The height is not: a tab
    /// is one long scrolling column, and an element scrolled out of the
    /// viewport is clipped out of the frame and cannot be clicked, so the
    /// window is made tall enough to hold the longest tab whole.
    const TEST_WINDOW: gpui::Size<Pixels> = size(WINDOW_SIZE.width, px(9000.));

    /// Build the showcase the way `main` does — `gpui_kit::init`, the view, and
    /// the `Root` that owns the dialog and notification layers — in a test
    /// window.
    fn open(cx: &mut TestAppContext) -> (Entity<Showcase>, Entity<Root>, VisualTestContext) {
        cx.update(gpui_kit::init);
        let view: Rc<RefCell<Option<Entity<Showcase>>>> = Rc::new(RefCell::new(None));
        let handle = cx.open_window(TEST_WINDOW, {
            let view = view.clone();
            move |window, cx| {
                let showcase = cx.new(|cx| Showcase::new(window, cx));
                *view.borrow_mut() = Some(showcase.clone());
                Root::new(showcase, window, cx)
            }
        });
        let root = handle.root(cx).expect("the root view was built");
        let showcase = view.borrow_mut().take().expect("the showcase was built");
        let mut cx = VisualTestContext::from_window(*handle.deref(), cx);
        cx.run_until_parked();
        draw(&mut cx);
        (showcase, root, cx)
    }

    /// Lay the window out and paint it, which is what fills `debug_bounds`.
    fn draw(cx: &mut VisualTestContext) {
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    /// Switch to `tab` and draw the frame that shows it.
    fn show(cx: &mut VisualTestContext, showcase: &Entity<Showcase>, tab: Tab) {
        cx.update(|_window, cx| {
            showcase.update(cx, |this, cx| {
                this.active_tab = tab;
                cx.notify();
            });
        });
        cx.run_until_parked();
        draw(cx);
    }

    /// Where the element tagged `selector` was laid out.
    fn bounds_of(cx: &mut VisualTestContext, selector: &'static str) -> Bounds<Pixels> {
        cx.debug_bounds(selector)
            .unwrap_or_else(|| panic!("{selector} was not laid out"))
    }

    /// Click `at`, let the click's work finish and draw the frame it produced.
    fn click_at(cx: &mut VisualTestContext, at: Point<Pixels>) {
        cx.simulate_click(at, Modifiers::default());
        cx.run_until_parked();
        draw(cx);
    }

    /// Click the leading edge of the element tagged `selector`.
    ///
    /// The leading edge, not the middle: a probe around a control in a block
    /// container is as wide as the column, and the control it holds sits at the
    /// left of it, so the middle of the probe can be empty space. Upstream's own
    /// interaction tests click the same way (`tests/controlled_change_callbacks.rs`).
    fn click(cx: &mut VisualTestContext, selector: &'static str) {
        let bounds = bounds_of(cx, selector);
        click_at(cx, point(bounds.left() + px(8.), bounds.center().y));
    }

    /// Read something off the showcase's model.
    fn read<R>(
        cx: &mut VisualTestContext,
        showcase: &Entity<Showcase>,
        f: impl FnOnce(&Showcase, &App) -> R,
    ) -> R {
        cx.update(|_window, cx| f(showcase.read(cx), cx))
    }

    /// Every tab lays out: the bar's ten tabs each render on the test platform,
    /// each leaves a tab root behind, and that root has a size.
    #[gpui::test]
    fn every_tab_lays_out(cx: &mut TestAppContext) {
        let (showcase, _root, mut cx) = open(cx);
        assert_eq!(Tab::ALL.len(), 10, "the bar no longer has ten tabs");
        for tab in Tab::ALL {
            show(&mut cx, &showcase, tab);
            assert_eq!(read(&mut cx, &showcase, |this, _| this.active_tab), tab);
            let bounds = cx
                .debug_bounds(TAB_ROOT)
                .unwrap_or_else(|| panic!("{tab:?}: nothing was laid out under the tab bar"));
            assert!(
                bounds.size.width > px(0.) && bounds.size.height > px(0.),
                "{tab:?}: the tab root laid out at {:?}",
                bounds.size
            );
        }
    }

    /// Every resizable group has room to drag: rationale §1.1 as a rule.
    ///
    /// gpui-base clamps each panel to `PANEL_MIN_SIZE`, so a box narrower —
    /// or shorter — than its panels' minimums put together holds a divider
    /// that cannot move. The sizes come from the frame the showcase just drew
    /// and the panel counts from the same `RESIZABLE_GROUPS` entries the render
    /// code builds from, so neither is a number this test types out again.
    #[gpui::test]
    fn resizable_groups_have_room_to_drag(cx: &mut TestAppContext) {
        let (showcase, _root, mut cx) = open(cx);
        show(&mut cx, &showcase, Tab::Layout);
        assert!(!RESIZABLE_GROUPS.is_empty());
        for group in RESIZABLE_GROUPS {
            let bounds = bounds_of(&mut cx, group.id);
            let outer = match group.axis {
                Axis::Horizontal => bounds.size.width,
                Axis::Vertical => bounds.size.height,
            };
            let room = outer - px(2.0 * RESIZABLE_BORDER);
            let needed = PANEL_MIN_SIZE * group.panels.len() as f32;
            assert!(
                room > needed,
                "{}: {} panels need more than {needed:?} between the borders, the box leaves {room:?} — the divider cannot move",
                group.id,
                group.panels.len(),
            );
        }
    }

    /// Every control the showcase advertises as interactive answers a click.
    ///
    /// Each step drives the real widget through the test platform's mouse and
    /// keyboard and then asks the model what changed, so a handler that stops
    /// being wired up fails the step that names it.
    #[gpui::test]
    fn interactive_controls_respond(cx: &mut TestAppContext) {
        let (showcase, root, mut cx) = open(cx);

        // --- Buttons tab --------------------------------------------------
        show(&mut cx, &showcase, Tab::Buttons);

        // Clipboard: the card is one icon button, and the test platform holds
        // a real in-memory clipboard.
        click(&mut cx, PROBE_CLIPBOARD);
        assert_eq!(
            cx.read_from_clipboard().and_then(|item| item.text()),
            Some("cargo add native-theme".to_string()),
            "Clipboard: the Copy button wrote nothing"
        );

        // --- Inputs tab ---------------------------------------------------
        show(&mut cx, &showcase, Tab::Inputs);

        // Rating: clicking a star at or below the current value clears down to
        // the one before it (rating.rs, Rating::render on_click), so the first
        // star takes the three the showcase starts with to none.
        click(&mut cx, PROBE_RATING);
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.rating_value),
            0,
            "Rating: clicking the first star left the value alone"
        );

        // Combobox: the trigger opens the list, and Enter takes the row the
        // list has under the cursor.
        click(&mut cx, PROBE_COMBOBOX);
        cx.simulate_keystrokes("down enter");
        draw(&mut cx);
        assert!(
            read(&mut cx, &showcase, |this, cx| !this
                .combobox_state
                .read(cx)
                .selection()
                .is_empty()),
            "Combobox: opening the list and confirming a row selected nothing"
        );

        // --- Data tab -----------------------------------------------------
        show(&mut cx, &showcase, Tab::Data);

        // Pagination: the leading end of the strip is the previous-page control.
        let before = read(&mut cx, &showcase, |this, _| this.page);
        click(&mut cx, PROBE_PAGINATION);
        assert_ne!(
            read(&mut cx, &showcase, |this, _| this.page),
            before,
            "Pagination: the page did not move"
        );

        // Attachment: the whole card advances its status.
        let before = read(&mut cx, &showcase, |this, _| this.attachment_status);
        click(&mut cx, PROBE_ATTACHMENT);
        assert_ne!(
            read(&mut cx, &showcase, |this, _| this.attachment_status),
            before,
            "Attachment: the status did not advance"
        );

        // MessageScroller: Send appends to the thread the scroller renders.
        let before = read(&mut cx, &showcase, |this, _| this.chat_messages.len());
        click(&mut cx, PROBE_CHAT_SEND);
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.chat_messages.len()),
            before + 1,
            "MessageScroller: Send added no message"
        );

        // --- Layout tab ---------------------------------------------------
        show(&mut cx, &showcase, Tab::Layout);

        // Stepper: the steps run left to right, so the leading one is the first.
        click(&mut cx, PROBE_STEPPER);
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.step),
            0,
            "Stepper: clicking the first step left the selection alone"
        );

        // SidebarToggleButton: the flag it draws is the flag it flips.
        click(&mut cx, PROBE_SIDEBAR_TOGGLE);
        assert!(
            read(&mut cx, &showcase, |this, _| this.sidebar_collapsed),
            "SidebarToggleButton: the sidebar did not collapse"
        );

        // Carousel: the last pagination dot goes to the last slide.
        click(&mut cx, PROBE_CAROUSEL_LAST);
        assert_eq!(
            read(&mut cx, &showcase, |this, cx| this
                .carousel_state
                .read(cx)
                .selected_index()),
            Some(CAROUSEL_SLIDES.len() - 1),
            "Carousel: the last pagination dot did not select the last slide"
        );

        // --- Overlays tab -------------------------------------------------
        show(&mut cx, &showcase, Tab::Overlays);

        // AlertDialog: opened by a click, then answered from the keyboard the
        // dialog binds — Enter confirms, Escape cancels.
        click(&mut cx, PROBE_ALERT_DIALOG);
        assert!(
            cx.debug_bounds("dialog-layer").is_some(),
            "AlertDialog: the dialog layer never reached the screen"
        );
        cx.simulate_keystrokes("enter");
        draw(&mut cx);
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.alert_choice.clone()),
            Some("Discard".into()),
            "AlertDialog: confirming did not report a choice"
        );
        click(&mut cx, PROBE_ALERT_DIALOG);
        cx.simulate_keystrokes("escape");
        draw(&mut cx);
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.alert_choice.clone()),
            Some("Keep".into()),
            "AlertDialog: cancelling did not report a choice"
        );

        // --- Feedback tab -------------------------------------------------
        show(&mut cx, &showcase, Tab::Feedback);

        // Notification: the button pushes one onto the Root's own layer.
        let before = cx.update(|_w, cx| root.read(cx).notification.read(cx).notifications().len());
        click(&mut cx, PROBE_NOTIFICATION);
        assert_eq!(
            cx.update(|_w, cx| root.read(cx).notification.read(cx).notifications().len()),
            before + 1,
            "Notification: nothing was pushed"
        );

        // --- The sidebar's colour mode switch -----------------------------
        //
        // The list opens under the trigger; the third row is Dark, and the
        // mode the showcase installs is what the whole interface re-themes on.
        click(&mut cx, PROBE_COLOR_MODE);
        cx.simulate_keystrokes("down down enter");
        draw(&mut cx);
        assert_eq!(
            read(&mut cx, &showcase, |this, _| this.color_mode),
            AppColorMode::Dark,
            "the colour mode switch did not reach Dark"
        );
        assert!(
            cx.update(|_w, cx| Theme::global(cx).mode.is_dark()),
            "the colour mode switch did not reach Theme::mode"
        );
    }
}
