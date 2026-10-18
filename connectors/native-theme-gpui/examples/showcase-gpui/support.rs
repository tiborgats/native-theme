//! What the pages share: sample content, layout helpers, icon loading, and the list and table delegates.

use std::borrow::Cow;
use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    App, Context, Div, Entity, ImageSource, IntoElement, Pixels, SharedString, Stateful,
    StyleRefinement, Styled, Task, Window, px,
};
use gpui_component::{
    ActiveTheme, IconName, IconNamed as _, IndexPath, Selectable as _, StyledExt,
    attachment::AttachmentStatus,
    list::{ListDelegate, ListState},
    searchable_list::{SearchableListChange, SearchableListDelegate, SearchableListItem},
    table::{Column, ColumnSort, TableDelegate, TableEvent, TableState},
};

#[cfg(target_os = "linux")]
use native_theme::detect::system_icon_theme;
use native_theme::icons::{
    FreedesktopLoader, IconData, IconRole, IconSetChoice, LucideLoader, MaterialLoader,
    SegoeFluentLoader, SfSymbolsLoader, load_icon,
};
use native_theme::theme::IconSet;
#[cfg(target_os = "linux")]
use native_theme_gpui::icons::freedesktop_name_candidates;
use native_theme_gpui::icons::{lucide_name_for_gpui_icon, material_name_for_gpui_icon};
use native_theme_gpui::{ActiveNativeTheme, Native};
use native_theme_showcase_support::rows;

use crate::demo::{self, BodyRow, DataTableRow};
use crate::info::InfoRegistry;

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
        "IconRole maps to the desktop icon theme; icon themes are never mixed.",
    ),
];

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

/// The patch the Typography page's `Diff` shows: one Rust file, with
/// removed, added and context lines.
pub(crate) const DIFF_SAMPLE: &str = r#"diff --git a/src/install.rs b/src/install.rs
--- a/src/install.rs
+++ b/src/install.rs
@@ -5,12 +5,12 @@
 /// Install the desktop's colors, fonts and geometry into gpui-component.
 fn install(cx: &mut App) -> native_theme::Result<()> {
     let system = SystemTheme::from_system()?;
-    let mode = match system.mode.is_dark() {
-        true => ColorMode::Dark,
-        false => ColorMode::Light,
+    let mode = if system.mode.is_dark() {
+        ColorMode::Dark
+    } else {
+        ColorMode::Light
     };
     let resolved = system.pick(mode);
-    println!("{} {}px", resolved.defaults.font.family, resolved.defaults.font.size);
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
// Helpers
// ---------------------------------------------------------------------------

/// The geometry refinement a builder produces for the installed native theme,
/// or `None` before `apply` ran (spec §9.1). Returns an owned value so the
/// borrow of `cx` ends at once.
pub(crate) fn native_geometry<F: FnOnce(Native<'_>) -> StyleRefinement>(
    cx: &App,
    build: F,
) -> Option<StyleRefinement> {
    cx.native().map(build)
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
/// widgets that are not `Styled` (`OtpInput`). `ButtonGroup` and
/// `DropdownButton` children still take `geometry::button_label` on their
/// label; it is their frame the group manages.
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
/// `src/app.rs:3103-3113`). The showcase rebuilds these caches on every
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
    cx.native().map(build)
}

/// A colour of the installed native theme, as gpui paints it, or `None`
/// before `apply` ran: for a widget whose own setter or refinement takes a
/// colour the connector's tokens do not carry.
pub(crate) fn native_color<F: FnOnce(Native<'_>) -> native_theme::color::Rgba>(
    cx: &App,
    pick: F,
) -> Option<gpui::Hsla> {
    native_value(cx, pick).map(|color| {
        let [r, g, b, a] = color.to_f32_array();
        gpui::Hsla::from(gpui::Rgba { r, g, b, a })
    })
}

/// `v_flex`/`h_flex` sized by one of the layout accessors, which are `None`
/// wherever the platform specifies nothing (platform-facts §2.20). Where it
/// is `None` no gap is set, so none is drawn: nothing native is known to
/// set, and the showcase invents none. Every bundled preset states
/// `widget_gap` and `window_margin`; macos-tahoe states no `section_gap` (nor
/// `container_margin`), so its pages' sections touch.
pub(crate) fn with_gap<W: Styled>(widget: W, gap: Option<Pixels>) -> W {
    match gap {
        Some(g) => widget.gap(g),
        None => widget,
    }
}

/// A box `id` that scrolls `content` vertically: the structure
/// gpui-component's `Scrollable` builds (scroll/scrollable.rs:147-196, a
/// root the caller sizes, a scroll area filling it, the content at least the
/// area's height, the scrollbar laid over the root), on the application's
/// own `handle`, so the content keeps `geometry::scrollbar_gutter`'s strip
/// free only while it overflows the area. gpui-base hides the bar while the
/// content fits (gpui-base src/scrollbar.rs:1402-1406), and the platform's
/// bar then takes no room either. The strip follows the last layout's
/// overflow; a frame whose layout changes it calls `relayout`, which asks
/// the view for another frame.
pub(crate) fn gutter_scroll(
    id: &'static str,
    handle: &gpui::ScrollHandle,
    content: Div,
    relayout: impl Fn(&mut App) + 'static,
    cx: &App,
) -> Stateful<Div> {
    use gpui::{InteractiveElement as _, ParentElement as _, StatefulInteractiveElement as _};
    use gpui_component::{InteractiveElementExt as _, scroll::ScrollableElement as _};
    let overflows = |h: &gpui::ScrollHandle| h.max_offset().y > px(0.);
    let reserved = overflows(handle);
    let content = if reserved {
        content.native(cx, native_theme_gpui::geometry::scrollbar_gutter)
    } else {
        content
    };
    let watched = handle.clone();
    // Clipped on the scrolled axis, as upstream's root (scrollable.rs:227-262):
    // a flex item that is not keeps its content's height as its minimum and
    // grows to it instead of scrolling.
    gpui::div()
        .id(id)
        .relative()
        .overflow_y_hidden()
        .child(
            gpui::div()
                .id("scroll-area")
                .size_full()
                .flex()
                .flex_col()
                .track_scroll(handle)
                .overflow_y_scroll()
                .lock_scroll_axis()
                .child(content.flex_none().h_auto().min_h_full()),
        )
        .vertical_scrollbar(handle)
        .child(
            gpui::canvas(
                move |_, _, cx| {
                    if overflows(&watched) != reserved {
                        relayout(cx);
                    }
                },
                |_, _, _, _| {},
            )
            .absolute()
            .size_0(),
        )
}

/// `widget` padded on every side by one of the layout accessors, and not at
/// all where it is `None`, as [`with_gap`] spaces.
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
    /// The set has no icon for it. No other set's icon stands in: the
    /// loaders return `None` rather than substitute one.
    NotFound,
}

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

/// Whether `set` is one of the platforms' own icon sets -- freedesktop, SF
/// Symbols, Segoe Fluent -- whose loaders colour a symbolic icon as they
/// load it (`color_opt`), where a bundled set's SVGs are coloured as they
/// are drawn (`Showcase::rebuild_icon_caches`).
pub(crate) fn is_system_set(set: IconSet) -> bool {
    matches!(
        set,
        IconSet::Freedesktop | IconSet::SfSymbols | IconSet::SegoeFluent
    )
}

/// The desktop's own theme, read: `SystemTheme::from_system`, which
/// `default` installs. Counted in the self-tests (`SYSTEM_READS`).
pub(crate) fn read_system_theme() -> native_theme::Result<native_theme::SystemTheme> {
    #[cfg(test)]
    SYSTEM_READS.with(|reads| reads.set(reads.get().saturating_add(1)));
    native_theme::SystemTheme::from_system()
}

#[cfg(test)]
thread_local! {
    /// How often the showcase read the desktop's theme
    /// (`read_system_theme`) on this thread; a test reads how much an action
    /// adds to it.
    pub(crate) static SYSTEM_READS: Cell<usize> = const { Cell::new(0) };
    /// How often the showcase loaded a set's icons (`load_all_icons`) on
    /// this thread; a test reads how much an action adds to it.
    pub(crate) static ICON_LOADS: Cell<usize> = const { Cell::new(0) };
}

/// Pre-load the icon of each of the 42 `IconRole`s from the given icon set,
/// tracking its source.
///
/// `theme`: the freedesktop theme a freedesktop set loads from
/// (`Showcase::freedesktop_theme`), the system's own where `None`; the
/// loader looks each role's name up itself (native-theme
/// `FreedesktopLoader::load`), which loads nothing off Linux.
pub(crate) fn load_all_icons(
    icon_set: IconSet,
    theme: Option<&str>,
    fg_color: Option<native_theme::Rgba>,
) -> Vec<(IconRole, Option<IconData>, IconSource)> {
    #[cfg(test)]
    ICON_LOADS.with(|loads| loads.set(loads.get().saturating_add(1)));
    let is_system_set = is_system_set(icon_set);

    IconRole::ALL
        .iter()
        .map(|role| {
            let data = match icon_set {
                IconSet::Freedesktop => {
                    let loader = FreedesktopLoader::new(*role).color_opt(fg_color);
                    match theme {
                        Some(theme) => loader.theme(theme).load(),
                        None => loader.load(),
                    }
                }
                IconSet::Material => MaterialLoader::new(*role).load(),
                IconSet::Lucide => LucideLoader::new(*role).load(),
                IconSet::SfSymbols => SfSymbolsLoader::new(*role).color_opt(fg_color).load(),
                IconSet::SegoeFluent => SegoeFluentLoader::new(*role).color_opt(fg_color).load(),
                _ => None,
            };

            let source = match (&data, is_system_set) {
                (None, _) => IconSource::NotFound,
                (Some(_), false) => IconSource::Bundled,
                (Some(_), true) => IconSource::System,
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

/// The `IconRole` the gallery shows `icon` under, and loads it by: the
/// first of `IconRole::ALL` whose `native_theme_gpui::icons::icon_name` is
/// `icon`, so where two roles map to one icon (`DialogError` and
/// `StatusError` to `CircleX`) the first listed wins; `None` where no role
/// maps to it. `IconName` has no `PartialEq`, so the two are compared by
/// their asset paths.
pub(crate) fn role_for_gpui_icon(icon: &IconName) -> Option<IconRole> {
    let path = icon.clone().path();
    IconRole::ALL.iter().copied().find(|role| {
        native_theme_gpui::icons::icon_name(*role).is_some_and(|named| named.path() == path)
    })
}

/// The 106 gpui-component 0.7.3 IconName variants shown in the gallery.
pub(crate) const GPUI_ICONS: &[(&str, IconName)] = &[
    ("ALargeSmall", IconName::ALargeSmall),
    ("ArrowDown", IconName::ArrowDown),
    ("ArrowLeft", IconName::ArrowLeft),
    ("ArrowRight", IconName::ArrowRight),
    ("ArrowUp", IconName::ArrowUp),
    ("Asterisk", IconName::Asterisk),
    ("Ban", IconName::Ban),
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
    ("CircleAlert", IconName::CircleAlert),
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
    ("Mic", IconName::Mic),
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
    ("RefreshCw", IconName::RefreshCw),
    ("Replace", IconName::Replace),
    ("ResizeCorner", IconName::ResizeCorner),
    ("RotateCw", IconName::RotateCw),
    ("Search", IconName::Search),
    ("Settings", IconName::Settings),
    ("Settings2", IconName::Settings2),
    ("SortAscending", IconName::SortAscending),
    ("SortDescending", IconName::SortDescending),
    ("Square", IconName::Square),
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
/// `theme`: the freedesktop theme a freedesktop set loads from
/// (`Showcase::freedesktop_theme`), the system's own where `None`, as
/// `load_all_icons` takes it.
pub(crate) fn load_gpui_icons(
    icon_set: Option<IconSet>,
    theme: Option<&str>,
    fg_color: Option<native_theme::Rgba>,
) -> Vec<IconEntry> {
    let icon_set = match icon_set {
        Some(set) => set,
        None => {
            // "gpui-builtin" — no native-theme data loaded
            return GPUI_ICONS
                .iter()
                .map(|(name, icon)| {
                    let role = role_for_gpui_icon(icon);
                    (*name, icon.clone(), role, None, IconSource::Bundled)
                })
                .collect();
        }
    };

    let is_system_set = is_system_set(icon_set);

    // On Linux with freedesktop, pick the theme once for the whole batch:
    // `theme`, else the system's. No theme where the system's cannot be
    // detected: then no icon loads.
    #[cfg(target_os = "linux")]
    let fd_theme = if icon_set == IconSet::Freedesktop {
        theme
            .map(|s| s.to_string())
            .or_else(|| system_icon_theme().ok())
    } else {
        None
    };
    // An icon's freedesktop names, each tried in the one theme: the theme's
    // convention decides which it has, not the desktop's (no theme mixing).
    #[cfg(target_os = "linux")]
    let load_by_name = |icon: &IconName, theme: &str| {
        FreedesktopLoader::any(freedesktop_name_candidates(icon.clone()))
            .theme(theme)
            .color_opt(fg_color)
            .load()
    };

    GPUI_ICONS
        .iter()
        .map(|(name, icon)| {
            let role = role_for_gpui_icon(icon);

            // Try loading by IconRole first (existing path)
            if let Some(r) = role {
                let data = match icon_set {
                    IconSet::Freedesktop => {
                        let loader = FreedesktopLoader::new(r).color_opt(fg_color);
                        match theme {
                            Some(theme) => loader.theme(theme).load(),
                            None => loader.load(),
                        }
                    }
                    IconSet::Material => MaterialLoader::new(r).load(),
                    IconSet::Lucide => LucideLoader::new(r).load(),
                    IconSet::SfSymbols => SfSymbolsLoader::new(r).color_opt(fg_color).load(),
                    IconSet::SegoeFluent => SegoeFluentLoader::new(r).color_opt(fg_color).load(),
                    _ => None,
                };
                let source = match &data {
                    None => IconSource::NotFound,
                    Some(_) if !is_system_set => IconSource::Bundled,
                    Some(_) => IconSource::System,
                };
                // If the system set has no icon for the role, try the icon's
                // freedesktop names in the same theme before giving up
                #[cfg(target_os = "linux")]
                if source == IconSource::NotFound
                    && let Some(theme) = &fd_theme
                {
                    if let Some(fd_data) = load_by_name(icon, theme) {
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
            if let Some(theme) = &fd_theme {
                if let Some(data) = load_by_name(icon, theme) {
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

/// One of the chrome's own icons -- a toolbar button's, the status bar's
/// side-panel toggle's, a command-palette entry's, a Layout page Sidebar
/// sample item's -- as the chosen icon theme gives it. Icon themes are never
/// mixed: where the chosen one has no icon for it, nothing is drawn.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ChromeIcon {
    /// gpui-component's own icon of this name, its built-in set being the
    /// one chosen: its IconName's name, or its asset path where the gallery
    /// does not list it.
    Builtin(SharedString),
    /// The chosen set's SVG for the IconName of this name, as the Icons
    /// page loads it (`load_gpui_icons`).
    Loaded(&'static str, Cow<'static, [u8]>),
    /// The chosen set holds no SVG for the IconName of this name: none at
    /// all, or only pixels, which an `Icon` cannot draw (icon.rs, `Icon`
    /// takes a path or SVG bytes).
    Missing(&'static str),
    /// An IconName, by its asset path, that the Icons page's gallery does
    /// not list, so no set other than gpui-component's own was asked for it.
    Unlisted(SharedString),
}

impl ChromeIcon {
    /// The chrome's icon for `icon`: `icons` are the chosen set's, as
    /// `load_gpui_icons` loaded them, and `builtin` is whether that set is
    /// gpui-component's own. With another set chosen, an IconName the
    /// gallery does not list is `Unlisted`: nothing was loaded for it.
    pub(crate) fn of(icons: &[IconEntry], builtin: bool, icon: &IconName) -> Self {
        let path = icon.clone().path();
        let listed = icons.iter().find(|(_, i, ..)| i.clone().path() == path);
        match listed {
            Some((name, ..)) if builtin => Self::Builtin((*name).into()),
            None if builtin => Self::Builtin(path),
            Some((name, _, _, Some(IconData::Svg(bytes)), _)) => Self::Loaded(name, bytes.clone()),
            Some((name, ..)) => Self::Missing(name),
            None => Self::Unlisted(path),
        }
    }

    /// Whether the chosen set gives an icon to draw.
    pub(crate) fn shown(&self) -> bool {
        !matches!(self, Self::Missing(_) | Self::Unlisted(_))
    }
}

/// An icon a page sample shows: gpui-component's `icon` as the chosen icon
/// theme, named `set`, gives it (`drawn`), looked up as the chrome's icons
/// are (`Showcase::sample_icon`). Where that theme has none, the sample
/// draws none and its info says so -- never another icon theme's.
#[derive(Clone)]
pub(crate) struct SampleIcon {
    pub(crate) icon: IconName,
    pub(crate) drawn: ChromeIcon,
    pub(crate) set: SharedString,
}

impl SampleIcon {
    /// Whether the chosen icon theme gives the sample an icon to draw.
    pub(crate) fn shown(&self) -> bool {
        self.drawn.shown()
    }

    /// What the sample's info says its icon is, `absent` being what the
    /// sample shows where the chosen icon theme has none.
    pub(crate) fn note(&self, absent: &str) -> String {
        crate::info::chrome::chrome_icon_note(&self.drawn, &self.set, absent)
    }
}

/// The icons of the Overlays page's dialogs, of the chosen icon theme: the
/// Dialog's beside its description and the AlertDialog's.
pub(crate) struct DialogIcons {
    pub(crate) dialog: SampleIcon,
    pub(crate) alert: SampleIcon,
}

/// The dialogs' icons as the view last set them, which it does as each
/// frame starts: gpui-component's `WindowState` root plugin builds a dialog
/// anew for every frame, from a builder that cannot borrow the view (root.rs,
/// `WindowState::dialog_layer`), so
/// this is how a dialog shows the icon theme installed now, not the one it
/// opened under. `Showcase::overlay_gap` does the same for the gap.
#[derive(Clone, Default)]
pub(crate) struct SharedDialogIcons(Rc<Cell<Option<Rc<DialogIcons>>>>);

impl SharedDialogIcons {
    pub(crate) fn set(&self, icons: DialogIcons) {
        self.0.set(Some(Rc::new(icons)));
    }

    /// The icons last set; `None` before the first frame.
    pub(crate) fn get(&self) -> Option<Rc<DialogIcons>> {
        let icons = self.0.take();
        self.0.set(icons.clone());
        icons
    }
}

// ---------------------------------------------------------------------------
// Sample Table Delegate (for Data page)
// ---------------------------------------------------------------------------

/// The Data page's `DataTable` rows, and what the table's events said about
/// them.
///
/// A row's info has to say what the table paints on it, and the table
/// renders its rows from inside its own update, so the delegate cannot ask
/// it (table/state.rs, `TableState::render_table_row`). The showcase keeps
/// these three fields in step with the table's events instead
/// (`SampleTableDelegate::follow`).
pub(crate) struct SampleTableDelegate {
    pub(crate) ui: Entity<InfoRegistry>,
    pub(crate) columns: Vec<Column>,
    /// The rows in the order the table shows them.
    pub(crate) rows: Vec<[SharedString; 3]>,
    /// The rows in the order they were given, which an unsorted column
    /// restores.
    pub(crate) unsorted: Vec<[SharedString; 3]>,
    /// The table's selected row (`TableState::selected_row`).
    pub(crate) selected_row: Option<usize>,
    /// Whether the table selects rows rather than a column, so paints its
    /// selected row as one.
    pub(crate) selection_shown: bool,
    pub(crate) right_clicked_row: Option<usize>,
}

impl SampleTableDelegate {
    /// Keep the delegate's copy of the table's selection in step with
    /// `event`: every change to it is emitted (table/state.rs,
    /// `TableState::set_selected_row`, `set_selected_col`,
    /// `set_selected_cell`, `clear_selection`, `on_row_right_click`,
    /// `on_cell_right_click`).
    pub(crate) fn follow(&mut self, event: &TableEvent) {
        match event {
            TableEvent::SelectRow(ix) => {
                self.selected_row = Some(*ix);
                self.selection_shown = true;
            }
            TableEvent::SelectColumn(_) | TableEvent::SelectCell(..) => {
                self.selection_shown = false;
            }
            TableEvent::ClearSelection => {
                self.selected_row = None;
                self.selection_shown = true;
            }
            TableEvent::RightClickedRow(row) => self.right_clicked_row = *row,
            // A right-clicked cell clears the right-clicked row (table/
            // state.rs:820). Only a `cell_selectable` table emits it.
            TableEvent::RightClickedCell(..) => self.right_clicked_row = None,
            _ => {}
        }
    }
}

impl TableDelegate for SampleTableDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _cx: &App) -> usize {
        self.rows.len()
    }

    /// Upstream asks only for the columns `columns_count` gives it
    /// (table/state.rs, `TableState::prepare_col_groups`); an index past them
    /// gets upstream's own `Column::default()` rather than a panic.
    fn column(&self, col_ix: usize, _cx: &App) -> Column {
        self.columns.get(col_ix).cloned().unwrap_or_default()
    }

    fn render_header(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> Stateful<Div> {
        let names: Vec<&str> = self.columns.iter().map(|c| c.name.as_ref()).collect();
        demo::data_table_header(&self.ui, cx, &names.join(", "))
    }

    /// Sort the rows as the sort button of column `col_ix` asks; upstream
    /// only cycles the button's state (table/state.rs,
    /// `TableState::perform_sort`).
    fn perform_sort(
        &mut self,
        col_ix: usize,
        sort: ColumnSort,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) {
        match sort {
            ColumnSort::Ascending => self.rows.sort_by(|a, b| a.get(col_ix).cmp(&b.get(col_ix))),
            ColumnSort::Descending => self.rows.sort_by(|a, b| b.get(col_ix).cmp(&a.get(col_ix))),
            ColumnSort::Default => self.rows.clone_from(&self.unsorted),
        }
    }

    /// Move column `col_ix` to `to_ix` with its cells, in `rows` and in
    /// `unsorted`, as upstream asks (table/delegate.rs, `move_column`): the
    /// table moves only its own header (table/state.rs,
    /// `TableState::move_column`), and asks the delegate for each cell and
    /// each sort by the column's new index.
    fn move_column(
        &mut self,
        col_ix: usize,
        to_ix: usize,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) {
        move_item(&mut self.columns, col_ix, to_ix);
        for row in self.rows.iter_mut().chain(self.unsorted.iter_mut()) {
            move_item(row, col_ix, to_ix);
        }
    }

    fn render_tr(
        &mut self,
        row_ix: usize,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> Stateful<Div> {
        // `demo::data_table` stripes the table, which shades its odd rows
        // (table/state.rs:2042).
        let striped = !row_ix.is_multiple_of(2);
        let (row, cells) = match self.rows.get(row_ix) {
            Some(cells) => (
                DataTableRow::Body(BodyRow {
                    striped,
                    last: row_ix + 1 == self.rows.len(),
                    selected: self.selected_row == Some(row_ix),
                    selection_shown: self.selection_shown,
                    right_clicked: self.right_clicked_row == Some(row_ix),
                }),
                Some(cells.join(", ")),
            ),
            None => (DataTableRow::Filler { striped }, None),
        };
        demo::data_table_row(&self.ui, cx, row_ix, row, cells)
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let text = self
            .rows
            .get(row_ix)
            .and_then(|row| row.get(col_ix))
            .cloned()
            .unwrap_or_default();
        // Plain text, not a Label: `Label::render` paints foreground and
        // this showcase's text_sm on its own element (label.rs:211), over what
        // the table gives the row.
        text
    }
}

/// Move the item at `from` to `to`, the items between shifting over by one;
/// nothing where either index is past `items`.
fn move_item<T>(items: &mut [T], from: usize, to: usize) {
    if from < to {
        if let Some(span) = items.get_mut(from..=to) {
            span.rotate_left(1);
        }
    } else if let Some(span) = items.get_mut(to..=from) {
        span.rotate_right(1);
    }
}

// ---------------------------------------------------------------------------
// Sample List Delegate (for Data page)
// ---------------------------------------------------------------------------

pub(crate) struct SampleListDelegate {
    pub(crate) ui: Entity<InfoRegistry>,
    /// Each row's info id and debug selector is this and its index,
    /// `{id_prefix}-{ix}`.
    pub(crate) id_prefix: &'static str,
    pub(crate) items: Vec<SharedString>,
    pub(crate) selected: Option<usize>,
}

impl ListDelegate for SampleListDelegate {
    type Item = demo::ListRow;

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
            demo::ListRow::new(&self.ui, self.id_prefix, ix.row, label)
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
// The theme settings' preset Combobox
// ---------------------------------------------------------------------------

/// One row of the preset Combobox: a preset, by key and display name.
#[derive(Clone)]
pub(crate) struct PresetItem {
    pub(crate) key: SharedString,
    pub(crate) display_name: SharedString,
}

/// The presets the showcase offers, in the order it offers them: the shared
/// crate's rows (`rows::preset_rows`), the desktop's own theme first, keyed
/// `default` and labelled with `default_preset`, the preset it builds on
/// (`Showcase::default_preset`), then the presets meant for this platform.
/// The theme settings' preset switch and the command palette both list
/// these, so neither can offer a preset the other does not.
pub(crate) fn preset_items(default_preset: &str) -> Vec<PresetItem> {
    rows::preset_rows(default_preset)
        .into_iter()
        .map(|row| PresetItem {
            key: row.key.into(),
            display_name: row.display_name.into(),
        })
        .collect()
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

/// `Combobox` is generic over a `SearchableListDelegate` (`combobox.rs:750`),
/// so the theme settings' preset switch takes a delegate: [`preset_items`],
/// filtered as the user types.
pub(crate) struct PresetDelegate {
    items: Vec<PresetItem>,
    matched: Vec<PresetItem>,
}

impl PresetDelegate {
    /// The rows of [`preset_items`], `default` labelled with `default_preset`.
    pub(crate) fn new(default_preset: &str) -> Self {
        let items = preset_items(default_preset);
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
    /// Single selection only, which is how the theme settings build it.
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
