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
    button::ButtonCustomVariant,
    list::{ListDelegate, ListState},
    searchable_list::{SearchableListDelegate, SearchableListItem},
    table::{Column, ColumnSort, TableDelegate, TableEvent, TableState},
};

#[cfg(target_os = "linux")]
use native_theme::detect::system_icon_theme;
use native_theme::icons::{
    FreedesktopLoader, IconData, IconRole, IconSetChoice, LucideLoader, MaterialLoader,
    SegoeFluentLoader, SfSymbolsLoader,
};
use native_theme::next::{Contrast, Palette, Preset};
use native_theme::theme::IconSet;
use native_theme_gpui::icons::{IconUse, load_gpui_icon, mask_drawable};
use native_theme_gpui::next::Mapped;
use native_theme_gpui::{AccessibilityPreferences, ActiveNativeTheme, Native};
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

/// One card of the Layout page's `GridLayout`: its id, which is the item's,
/// its area's and its card's selector, its label, and its area in grid cells.
#[derive(Clone, Copy)]
pub(crate) struct GridCard {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) col: usize,
    pub(crate) row: usize,
    pub(crate) col_span: usize,
    pub(crate) row_span: usize,
}

/// The cards of the Layout page's `GridLayout` in its 12 columns: two side by
/// side over one as wide as both.
pub(crate) const GRID_LAYOUT_ITEMS: [GridCard; 3] = [
    GridCard {
        id: "layout-grid-item-one",
        label: "One",
        col: 0,
        row: 0,
        col_span: 6,
        row_span: 2,
    },
    GridCard {
        id: "layout-grid-item-two",
        label: "Two",
        col: 6,
        row: 0,
        col_span: 6,
        row_span: 2,
    },
    GridCard {
        id: "layout-grid-item-three",
        label: "Three",
        col: 0,
        row: 2,
        col_span: 12,
        row_span: 2,
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

/// Who draws the frame of a window granted server-side decorations, on the
/// platform `os` names (`std::env::consts::OS`): the system on macOS and
/// Windows, whose title bar gpui's default window options keep
/// (`window_options`), and the window manager on Linux.
pub(crate) fn frame_drawer(os: &str) -> &'static str {
    match os {
        "macos" | "windows" => "the system",
        _ => "the window manager",
    }
}

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

/// The file of DIFF_SAMPLE as `DiffFile::path` gives it, and the line of its
/// modified side the Typography page's `Diff` decorates: `let system =
/// SystemTheme::from_system()?;`, which both sides keep, so the decoration lies
/// over no change fill, and which the Diff shows at rest.
pub(crate) const DIFF_DECORATED_PATH: &str = "src/install.rs";
pub(crate) const DIFF_DECORATED_LINE: usize = 7;

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

- [x] Resolve the desktop's theme
- [ ] Apply it with `apply_system_theme`
"#;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// The 0.7.2 theme the theme settings' choice resolves to, beside the 0.8.0
/// theme it installed (`Showcase::apply_theme_by_name`), for the pages not
/// yet on the 0.8.0 model: `cx.native()` gives no 0.7.2 theme once a 0.8.0
/// one is installed (`native_theme_gpui::next::apply`). Under a palette, which
/// the 0.7.2 model has no equivalent of, it is the preset alone, in the
/// scheme the palette resolved the 0.8.0 theme to.
pub(crate) struct OldModel {
    pub(crate) resolved: native_theme::theme::ResolvedTheme,
    /// The preferences the theme was installed with.
    pub(crate) accessibility: AccessibilityPreferences,
}

impl gpui::Global for OldModel {}

/// The 0.8.0 theme of the installed choice resolved for an inactive window
/// (`ThemeContext::window_active` false), beside the one installed, for the
/// views of an inactive window (`native_theme_gpui::next::geometry::title_bar_inactive`).
pub(crate) struct NextInactive(
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the page groups read it as they move to the 0.8.0 model"
        )
    )]
    pub(crate) native_theme::next::ResolvedTheme,
);

impl gpui::Global for NextInactive {}

/// What the installed 0.8.0 theme was asked for (`Showcase::try_install`):
/// the preferences it was installed with -- the OS's with the user's over
/// them (`demo::PreferenceOverrides`) -- which the Preferences sheet and the
/// status bar show, why the theme's contrast is lower than they ask, where
/// it is, and the palette chosen that High contrast set aside, where it set
/// one aside. The connector reads back the context the theme was resolved
/// to (`NativeTheme::accessibility`), whose contrast level the theme may have
/// lowered (docs/v0.8.0-spec.md §6, resolution step 1).
pub(crate) struct Requested {
    pub(crate) preferences: AccessibilityPreferences,
    pub(crate) contrast_held: Option<ContrastHeld>,
    pub(crate) palette_set_aside: Option<&'static str>,
}

impl gpui::Global for Requested {}

/// A contrast level the installed 0.8.0 theme was asked for and not resolved
/// at (docs/v0.8.0-spec.md §6, resolution step 1): the level asked, the one
/// used, and whether the palette laid over the theme holds `High` back,
/// which the theme without it reaches in the palette's scheme. Otherwise the
/// theme defines the level asked for in none of its sources, its preset's
/// levels or the system's settings, for the scheme resolved.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ContrastHeld {
    pub(crate) asked: Contrast,
    pub(crate) used: Contrast,
    pub(crate) by_palette: bool,
}

impl ContrastHeld {
    /// The sentence the Preferences sheet's High contrast switch says it in.
    pub(crate) fn text(&self) -> String {
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

/// The accessibility preferences the installed theme was asked for: the
/// 0.7.2 theme's own, where one is installed (`native_theme_gpui::apply`
/// keeps them as given), else the installed 0.8.0 theme's ([`Requested`]);
/// `None` before a theme was installed.
pub(crate) fn requested_preferences(cx: &App) -> Option<AccessibilityPreferences> {
    cx.native().map(|n| n.accessibility.clone()).or_else(|| {
        cx.native_theme()?.next_resolved(cx)?;
        cx.try_global::<Requested>()
            .map(|requested| requested.preferences.clone())
    })
}

/// Why the installed 0.8.0 theme's contrast is lower than its preferences
/// ask ([`Requested`]); `None` where it is not, or no 0.8.0 theme is
/// installed.
pub(crate) fn contrast_held(cx: &App) -> Option<&ContrastHeld> {
    cx.native_theme()?.next_resolved(cx)?;
    cx.try_global::<Requested>()?.contrast_held.as_ref()
}

/// The palette chosen that High contrast set aside in the installed 0.8.0
/// theme ([`Requested`]): `High` was asked for and the theme has it for the
/// scheme asked, so the theme is resolved without the palette, which comes
/// back once High contrast is off (docs/v0.8.0-spec.md §6, resolution step
/// 1). `None` where no palette is set aside, or no 0.8.0 theme is installed.
pub(crate) fn palette_set_aside(cx: &App) -> Option<&'static str> {
    cx.native_theme()?.next_resolved(cx)?;
    cx.try_global::<Requested>()?.palette_set_aside
}

/// The sentence the palette Select and the High contrast switch say that
/// High contrast set `palette` aside in ([`palette_set_aside`]).
pub(crate) fn palette_set_aside_text(palette: &str) -> String {
    format!(
        "the palette {palette} is set aside while High contrast is on: the theme has a High \
         level for the scheme asked, and a palette is not laid over it there, so the theme is \
         drawn in its own High contrast colours; {palette} is laid over it again once High \
         contrast is off (docs/v0.8.0-spec.md §6, resolution step 1)"
    )
}

/// What the pages still on the 0.7.2 model read where they read
/// `cx.native()`: the 0.7.2 theme installed, where one is
/// (`native_theme_gpui::apply`), else the 0.7.2 theme of the choice the
/// installed 0.8.0 theme was built from ([`OldModel`]); `None` before a
/// theme was installed.
pub(crate) fn old_native(cx: &App) -> Option<Native<'_>> {
    cx.native().or_else(|| {
        let installed = cx.native_theme()?;
        installed.next_resolved(cx)?;
        cx.try_global::<OldModel>().map(|old| Native {
            resolved: &old.resolved,
            accessibility: &old.accessibility,
        })
    })
}

/// The 0.7.2 resolved theme of [`old_native`].
#[cfg_attr(
    not(any(test, feature = "widgets")),
    expect(
        dead_code,
        reason = "only the theme-drawn widgets still read the 0.7.2 theme so, until their Looks move to the 0.8.0 model"
    )
)]
pub(crate) fn old_resolved(cx: &App) -> Option<&native_theme::theme::ResolvedTheme> {
    old_native(cx).map(|n| n.resolved)
}

/// The geometry refinement a builder produces for the installed native theme,
/// or `None` before `apply` ran (spec §9.1). Returns an owned value so the
/// borrow of `cx` ends at once.
pub(crate) fn native_geometry<F: FnOnce(Native<'_>) -> StyleRefinement>(
    cx: &App,
    build: F,
) -> Option<StyleRefinement> {
    old_native(cx).map(build)
}

/// The installed 0.8.0 theme for the styled theme's mode, or `None` before
/// `native_theme_gpui::next::apply` ran.
pub(crate) fn next_resolved(cx: &App) -> Option<&native_theme::next::ResolvedTheme> {
    cx.native_theme()?.next_resolved(cx)
}

/// `variants::toolbar_button` for the installed theme. That builder reads
/// the theme through `cx.native()`, which gives no 0.7.2 theme once a 0.8.0
/// one is installed (`native_theme_gpui::next::apply`), so under a 0.8.0
/// theme the variant is built from it (`next::variants::toolbar_button`,
/// which the connector parity holds to the 0.7.2 builder's); where that
/// theme lacks a source the builder reads, or a 0.7.2 theme is installed,
/// the 0.7.2 builder's.
pub(crate) fn toolbar_variant(cx: &App) -> ButtonCustomVariant {
    next_resolved(cx)
        .and_then(|r| native_theme_gpui::next::variants::toolbar_button(r, cx).ok())
        .unwrap_or_else(|| native_theme_gpui::variants::toolbar_button(cx))
}

/// The installed 0.8.0 theme resolved for an inactive window
/// ([`NextInactive`]), or `None` before `native_theme_gpui::next::apply` ran.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the page groups call it as they move to the 0.8.0 model"
    )
)]
pub(crate) fn next_inactive(cx: &App) -> Option<&native_theme::next::ResolvedTheme> {
    next_resolved(cx)?;
    cx.try_global::<NextInactive>().map(|inactive| &inactive.0)
}

/// [`native_value`] on the 0.8.0 model: the value `build` gives for the
/// installed 0.8.0 theme, or `None` before `native_theme_gpui::next::apply`
/// ran.
pub(crate) fn next_value<T, F: FnOnce(&native_theme::next::ResolvedTheme) -> T>(
    cx: &App,
    build: F,
) -> Option<T> {
    next_resolved(cx).map(build)
}

/// [`native_geometry`] on the 0.8.0 model: the refinement a builder of
/// `native_theme_gpui::next::geometry` produces for the installed 0.8.0
/// theme, or the source it lacks (`Unmapped`); `None` before
/// `native_theme_gpui::next::apply` ran.
pub(crate) fn next_geometry<
    F: FnOnce(&native_theme::next::ResolvedTheme) -> Mapped<StyleRefinement>,
>(
    cx: &App,
    build: F,
) -> Option<Mapped<StyleRefinement>> {
    next_resolved(cx).map(build)
}

/// [`native_color`] on the 0.8.0 model: the colour `pick` gives for the
/// installed 0.8.0 theme, as gpui paints it; `None` before
/// `native_theme_gpui::next::apply` ran, or where the theme states no such
/// colour (`pick` gives `None`).
#[cfg_attr(
    not(all(test, feature = "gpui-fast")),
    expect(
        dead_code,
        reason = "the page groups call it as they move to the 0.8.0 model"
    )
)]
pub(crate) fn next_color<
    F: FnOnce(&native_theme::next::ResolvedTheme) -> Option<native_theme::next::Color>,
>(
    cx: &App,
    pick: F,
) -> Option<gpui::Hsla> {
    next_value(cx, pick).flatten().map(|color| {
        gpui::Hsla::from(gpui::Rgba {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        })
    })
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

    /// `.next_native(cx, next::geometry::button)`: [`native`](Self::native)
    /// on the 0.8.0 model, the builder's refinement when a 0.8.0 theme is
    /// installed and states what the builder needs; nothing otherwise, so
    /// the widget keeps upstream's geometry where the theme states none
    /// (spec §10, rule 0).
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "every widget on the 0.8.0 model so far records the builder in its info, through next_native_value; the tests hold this one"
        )
    )]
    fn next_native(
        self,
        cx: &App,
        build: fn(&native_theme::next::ResolvedTheme) -> Mapped<StyleRefinement>,
    ) -> Self {
        refined(self, next_geometry(cx, build).and_then(Result::ok).as_ref())
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
    /// elsewhere. It clips a child that paints its own background to the
    /// edge: on GPUI Fast to the rounded inside (gpui-fast 0.1.15
    /// `src/fast/rounded_clip.rs:68-104`), on gpui-pre to the box's rectangle
    /// (gpui-pre 0.3.12 `src/window.rs:2453-2461`, `ContentMask`), where such a
    /// child's square corner shows past the rounded ones.
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
/// `src/app.rs:3217-3227`). The showcase rebuilds these caches on every
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
    old_native(cx).map(build)
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
/// gpui-component's `Scrollable` builds (scroll/scrollable.rs:153-227, a
/// root the caller sizes, a scroll area filling it, the content at least the
/// area's height, the scrollbar laid over the root; this one leaves out the root's flex
/// column, :203-207, and the `gpui_base::ScrollBounce` 0.9.1 wraps the area in, :209-215),
/// on the application's own `handle`, so the content keeps `geometry::scrollbar_gutter`'s strip
/// free only while it overflows the area. gpui-base hides the bar while the
/// content fits (gpui-base src/scrollbar.rs:1421-1424), and the platform's
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
    // Clipped on the scrolled axis, as upstream's root (scrollable.rs:269-304):
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

/// The 106 gpui-component 0.10.0 IconName variants shown in the gallery.
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

/// The gallery's entries as `load_gpui_icons` loads them, and, in the same
/// order, whether a mask of one colour carries each one's icon
/// (`native_theme_gpui::icons::mask_drawable`; `None` where nothing was
/// loaded or judged).
pub(crate) type GpuiIconsLoad = (Vec<IconEntry>, Vec<Option<bool>>);

/// Pre-load native-theme icons for gpui-component IconName variants, and
/// judge each.
///
/// The lookup is the connector's (`native_theme_gpui::icons::load_gpui_icon`):
/// each name by the connector's table, at the page's size (the loader's
/// own), any icon the set has; under SF Symbols and Segoe Fluent, which the
/// connector has no table for, by the role the showcase maps the name to.
/// `theme`: the freedesktop theme a freedesktop set loads from
/// (`Showcase::freedesktop_theme`), the system's own where `None`, as
/// `load_all_icons` takes it.
pub(crate) fn load_gpui_icons(
    icon_set: Option<IconSet>,
    theme: Option<&str>,
    fg_color: Option<native_theme::Rgba>,
) -> GpuiIconsLoad {
    let Some(icon_set) = icon_set else {
        // "gpui-builtin" — no native-theme data loaded
        let entries = GPUI_ICONS
            .iter()
            .map(|(name, icon)| {
                let role = role_for_gpui_icon(icon);
                (*name, icon.clone(), role, None, IconSource::Bundled)
            })
            .collect();
        return (entries, vec![None; GPUI_ICONS.len()]);
    };

    let is_system_set = is_system_set(icon_set);
    let source_of = |data: &Option<IconData>| match data {
        None => IconSource::NotFound,
        Some(_) if is_system_set => IconSource::System,
        Some(_) => IconSource::Bundled,
    };

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
    #[cfg(not(target_os = "linux"))]
    let fd_theme: Option<String> = {
        let _ = theme;
        None
    };
    let no_theme = icon_set == IconSet::Freedesktop && fd_theme.is_none();

    GPUI_ICONS
        .iter()
        .map(|(name, icon)| {
            let role = role_for_gpui_icon(icon);
            let (data, mask) = match icon_set {
                // Known by role only: the connector has no table from a
                // name to an SF Symbol or a Segoe glyph.
                IconSet::SfSymbols | IconSet::SegoeFluent => {
                    let data = role.and_then(|r| match icon_set {
                        IconSet::SfSymbols => SfSymbolsLoader::new(r).color_opt(fg_color).load(),
                        _ => SegoeFluentLoader::new(r).color_opt(fg_color).load(),
                    });
                    let mask = data.as_ref().and_then(mask_drawable);
                    (data, mask)
                }
                _ if no_theme => (None, None),
                _ => load_gpui_icon(
                    icon.clone(),
                    icon_set,
                    fd_theme.as_deref(),
                    None,
                    fg_color,
                    IconUse::Any,
                )
                .map_or((None, None), |found| (Some(found.data), found.mask)),
            };
            let source = source_of(&data);
            ((*name, icon.clone(), role, data, source), mask)
        })
        .unzip()
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
    /// The chosen set's pixels for the IconName of this name: an SF Symbol
    /// or a Segoe Fluent glyph, which the system loaders give as pixels
    /// only (`IconData::Rgba`), or a freedesktop theme's raster icon; or an
    /// SVG in several colours, which a mask would show as a silhouette
    /// (native_theme_gpui::icons::mask_drawable). It is
    /// the Icons page's decoded image of it (`Showcase::gpui_icon_sources`,
    /// `loaded_icon_sources`), drawn as that page draws one, with
    /// `gpui::img`: an `Icon` takes a path or SVG bytes alone (icon.rs,
    /// `IconSource`).
    Raster(&'static str, RasterIcon),
    /// The chosen set holds no icon for the IconName of this name: none at
    /// all, or pixels that did not decode into an image.
    Missing(&'static str),
    /// An IconName, by its asset path, that the Icons page's gallery does
    /// not list, so no set other than gpui-component's own was asked for it.
    Unlisted(SharedString),
}

/// A raster icon as gpui draws it: the decoded image the Icons page keeps
/// for it (`Showcase::rebuild_icon_caches`), which a rebuild of that cache
/// releases. A chrome icon is looked up again every frame, so it never
/// holds an image past that rebuild.
#[derive(Clone)]
pub(crate) struct RasterIcon(pub(crate) ImageSource);

impl PartialEq for RasterIcon {
    /// The same decoded image: gpui's sources compare by nothing else.
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (ImageSource::Render(a), ImageSource::Render(b)) => std::sync::Arc::ptr_eq(a, b),
            (ImageSource::Image(a), ImageSource::Image(b)) => std::sync::Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl std::fmt::Debug for RasterIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RasterIcon")
    }
}

impl ChromeIcon {
    /// The chrome's icon for `icon`: `icons` are the chosen set's, as
    /// `load_gpui_icons` loaded them, `sources` their decoded images and
    /// `masks` their judgements in the same order
    /// (`Showcase::gpui_icon_sources`, `Showcase::gpui_icon_masks`), and
    /// `builtin` is whether that set is gpui-component's own. An SVG judged
    /// not to be one colour is drawn as an image, as pixels are; a judgement
    /// missing counts as a mask. With another set chosen, an IconName the
    /// gallery does not list is `Unlisted`: nothing was loaded for it.
    pub(crate) fn of(
        icons: &[IconEntry],
        sources: &[Option<ImageSource>],
        masks: &[Option<bool>],
        builtin: bool,
        icon: &IconName,
    ) -> Self {
        let path = icon.clone().path();
        let listed = icons
            .iter()
            .enumerate()
            .find(|(_, (_, i, ..))| i.clone().path() == path);
        match listed {
            Some((_, (name, ..))) if builtin => Self::Builtin((*name).into()),
            None if builtin => Self::Builtin(path),
            Some((ix, (name, _, _, Some(IconData::Svg(_)), _)))
                if masks.get(ix).copied().flatten() == Some(false) =>
            {
                Self::raster(name, sources, ix)
            }
            Some((_, (name, _, _, Some(IconData::Svg(bytes)), _))) => {
                Self::Loaded(name, bytes.clone())
            }
            Some((ix, (name, _, _, Some(IconData::Rgba(_)), _))) => Self::raster(name, sources, ix),
            Some((_, (name, ..))) => Self::Missing(name),
            None => Self::Unlisted(path),
        }
    }

    /// The raster icon `name` at `ix` of the decoded images `sources`, or
    /// `Missing` where it did not decode.
    pub(crate) fn raster(name: &'static str, sources: &[Option<ImageSource>], ix: usize) -> Self {
        match sources.get(ix) {
            Some(Some(source)) => Self::Raster(name, RasterIcon(source.clone())),
            _ => Self::Missing(name),
        }
    }

    /// Whether the chosen set gives an icon to draw.
    pub(crate) fn shown(&self) -> bool {
        !matches!(self, Self::Missing(_) | Self::Unlisted(_))
    }

    /// Whether the icon is drawn as an image, in its own colours, not as a
    /// mask in its widget's text colour.
    pub(crate) fn is_raster(&self) -> bool {
        matches!(self, Self::Raster(..))
    }

    /// This icon for a widget that takes an `Icon` and nothing else -- a
    /// `Toggle`'s, a `CommandItem`'s, an `Alert`'s, a `StepperItem`'s, a
    /// `SidebarMenuItem`'s, a `Button`'s icon beside its label
    /// (`ButtonIcon`): a raster icon is `Missing` there, since an `Icon`
    /// draws a path or SVG bytes alone (icon.rs, `IconSource`), and the
    /// widget shows what it shows where the set has none. An SVG in several
    /// colours is `Missing` there too: an `Icon` would draw it as a mask of
    /// one colour.
    pub(crate) fn svg_only(self) -> Self {
        match self {
            Self::Raster(name, _) => Self::Missing(name),
            other => other,
        }
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

    /// Whether the sample's icon is drawn as an image, in its own colours
    /// (`ChromeIcon::is_raster`).
    pub(crate) fn is_raster(&self) -> bool {
        self.drawn.is_raster()
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
            // state.rs:885). Only a `cell_selectable` table emits it.
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
        // (table/state.rs:2339).
        let striped = !row_ix.is_multiple_of(2);
        let (row, cells) = match self.rows.get(row_ix) {
            Some(cells) => (
                DataTableRow::Body(BodyRow {
                    striped,
                    last: row_ix.saturating_add(1) == self.rows.len(),
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
        // this showcase's text_sm on its own element (label.rs:213), over what
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
                .at_edges(ix.row == 0, Some(ix.row) == self.items.len().checked_sub(1))
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
/// (`Showcase::default_preset`), then the platform presets meant for this
/// platform: those of the shared rows the 0.8.0 model has a [`Preset`] of.
/// The ten community presets of the 0.7.2 model are not offered: their
/// colours are palettes in the 0.8.0 model, chosen in the palette Select
/// ([`palette_rows`]) over any preset. The theme settings' preset switch and
/// the command palette both list these, so neither can offer a preset the
/// other does not.
pub(crate) fn preset_items(default_preset: &str) -> Vec<PresetItem> {
    rows::preset_rows(default_preset)
        .into_iter()
        .filter(|row| row.key == "default" || Preset::from_name(&row.key).is_some())
        .map(|row| PresetItem {
            key: row.key.into(),
            display_name: row.display_name.into(),
        })
        .collect()
}

/// The palette Select's row for no palette: the preset's own colours, the
/// value `--palette` takes for it.
pub(crate) use native_theme_showcase_support::cli::NO_PALETTE;

/// The palette Select's rows: [`NO_PALETTE`], then every bundled palette
/// (`Palette::BUNDLED_NAMES`), by its name.
pub(crate) fn palette_rows() -> Vec<SharedString> {
    std::iter::once(NO_PALETTE)
        .chain(Palette::BUNDLED_NAMES.iter().copied())
        .map(SharedString::from)
        .collect()
}

/// The bundled palette of the palette Select's row `row`, as `--palette`
/// takes it (`cli::palette_key`): `None` for [`NO_PALETTE`] and for a name
/// `Palette::BUNDLED_NAMES` does not hold.
pub(crate) fn palette_of_row(row: &str) -> Option<&'static str> {
    native_theme_showcase_support::cli::palette_key(row)
        .ok()
        .flatten()
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

/// `Combobox` is generic over a `SearchableListDelegate` (`combobox.rs:768`),
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
}
