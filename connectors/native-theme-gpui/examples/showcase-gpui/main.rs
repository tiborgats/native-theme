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
//! - The toolbar under the title bar switches theme presets, color modes and icon sets
//!   without restarting the app. Watch how the entire widget tree re-themes
//!   on each change — no manual rewiring per widget.
//! - Hover any widget to see tooltips explaining which `ResolvedTheme` fields
//!   drive its appearance.
//! - The Theme Map page exposes the full 138-field `ThemeColor` palette that
//!   gpui-component exposes, with each field's current value and the
//!   `native-theme` field it was derived from.
//! - The Icons page demonstrates `IconRole` mapping across Material, Lucide,
//!   and freedesktop sets, plus animated spinner playback.
//!
//! # How the source is organised
//!
//! `main.rs` holds the entry point, the command line and the screenshot
//! capture; `app.rs` the `Showcase` view, its state and theme switching;
//! `pages/` one module per page; `chrome.rs` and `demo.rs` the window's
//! chrome and the helpers that build a widget with its info; `info/` what
//! each widget reports; `inspector.rs` the inspector panel; and
//! `support.rs` the sample content, helpers, icon loading and delegates
//! the pages share. Within a file, section-divider blocks (`// ─────`)
//! separate one widget category, page or view from the next.

mod app;
mod chrome;
mod demo;
mod info;
mod inspector;
mod pages;
mod support;

use gpui::{
    App, Bounds, Div, IntoElement, ParentElement, Pixels, SharedString, WindowBounds,
    WindowDecorations, WindowOptions, div, prelude::*, px, size,
};
use gpui_component::{ActiveTheme, IconName, Root, TitleBar, select::SearchableVec};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use {gpui::Window, std::time::Duration};

use native_theme::icons::IconSetChoice;

use crate::app::{AppColorMode, Showcase};
use crate::support::{load_all_icons, load_gpui_icons};

// ---------------------------------------------------------------------------
// Pages
// ---------------------------------------------------------------------------

/// The pages the Sidebar navigates between (spec §2.4).
///
/// `Showcase::render` matches on this, so a new variant cannot be added without
/// the compiler asking what it renders, and the Sidebar's items, the View
/// menu, the `--tab` names and the layout self-test all read [`Page::ALL`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Page {
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

impl Page {
    /// Every page, in the order the Sidebar lists them.
    ///
    /// A new variant forces an arm in [`Page::index`], [`Page::label`],
    /// [`Page::icon`] and [`Page::nav_item`], whose matches are exhaustive,
    /// and the index it is given there has to be its position in this array
    /// — the `const` block below rejects the build otherwise. The one thing
    /// neither the compiler nor that block can see is a variant added to the
    /// enum and to the matches but not to this list: it would take an index
    /// the array does not have, and the assertion fires.
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

    /// The page's position in the Sidebar, which is what `ShowPage` carries.
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

    /// The page at a Sidebar position; `None` past the end.
    fn at(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    /// The label the Sidebar and the View menu show.
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

    /// The icon the page's Sidebar item shows.
    const fn icon(self) -> IconName {
        match self {
            Self::Buttons => IconName::CircleCheck,
            Self::Inputs => IconName::ALargeSmall,
            Self::Data => IconName::Inbox,
            Self::Feedback => IconName::Bell,
            Self::Typography => IconName::CaseSensitive,
            Self::Layout => IconName::LayoutDashboard,
            Self::Overlays => IconName::GalleryVerticalEnd,
            Self::Charts => IconName::ChartPie,
            Self::Icons => IconName::Star,
            Self::ThemeMap => IconName::Palette,
        }
    }

    /// The id and debug selector of the page's Sidebar item, so
    /// `the_sidebar_navigates` can click the item the render code built.
    const fn nav_item(self) -> &'static str {
        match self {
            Self::Buttons => "chrome-nav-buttons",
            Self::Inputs => "chrome-nav-inputs",
            Self::Data => "chrome-nav-data",
            Self::Feedback => "chrome-nav-feedback",
            Self::Typography => "chrome-nav-typography",
            Self::Layout => "chrome-nav-layout",
            Self::Overlays => "chrome-nav-overlays",
            Self::Charts => "chrome-nav-charts",
            Self::Icons => "chrome-nav-icons",
            Self::ThemeMap => "chrome-nav-theme-map",
        }
    }
}

/// `Page::ALL` is in Sidebar order and holds each page once.
const _: () = {
    let mut i = 0;
    while i < Page::ALL.len() {
        assert!(
            Page::ALL[i].index() == i,
            "Page::ALL is not the pages in Sidebar order"
        );
        i += 1;
    }
};

/// The width, in logical pixels, the pages were laid out for: the content
/// area the window gave them before the Sidebar and the inspector flanked it.
/// The model states no such value (spec §1.3); it is the showcase's own
/// layout default.
pub(crate) const PAGE_WIDTH_PX: f32 = 880.;

/// The window the showcase opens: `NAV_WIDTH` + the pages' width +
/// `INSPECTOR_WIDTH` wide, so the content panel starts at the width the pages
/// were laid out for. The sum is taken over the `f32`s, because gpui's
/// `Pixels` has no `const` arithmetic. The self-tests lay the interface out
/// at this width, so a measurement they take is a measurement of the real
/// thing.
pub(crate) const WINDOW_SIZE: gpui::Size<Pixels> = size(
    px(NAV_WIDTH_PX + PAGE_WIDTH_PX + INSPECTOR_WIDTH_PX),
    px(850.),
);

/// The debug selector the window's title bar carries, so
/// `the_title_bar_is_the_top_of_the_window` can see where it was laid out.
pub(crate) const CHROME_TITLE_BAR: &str = "chrome-title-bar";

/// The debug selector the AppMenuBar inside the title bar carries.
pub(crate) const CHROME_APP_MENU_BAR: &str = "chrome-app-menu-bar";

/// The debug selector the window's toolbar carries, so
/// `the_toolbar_is_the_models_toolbar` can measure it.
pub(crate) const CHROME_TOOLBAR: &str = "chrome-toolbar";

/// The debug selector the Sidebar carries, so the toggle test can measure it.
pub(crate) const CHROME_SIDEBAR: &str = "chrome-sidebar";

/// The debug selector the toolbar's SidebarToggleButton carries, the
/// toolbar's first item.
pub(crate) const CHROME_SIDEBAR_TOGGLE: &str = "chrome-sidebar-toggle";

/// The initial width of the Sidebar's panel. The model states no such value:
/// `SidebarTheme` has no width (spec §1.3), so this is the showcase's own
/// layout default, and dragging the panel's handle changes it.
pub(crate) const NAV_WIDTH: Pixels = px(NAV_WIDTH_PX);
const NAV_WIDTH_PX: f32 = 200.;

/// The initial width of the inspector's panel. The model states no such
/// value: it has no inspector at all (spec §1.3), so this is the showcase's
/// own layout default, and dragging the panel's handle changes it.
pub(crate) const INSPECTOR_WIDTH: Pixels = px(INSPECTOR_WIDTH_PX);
const INSPECTOR_WIDTH_PX: f32 = 300.;

/// The debug selector the active page's root carries, so `every_page_lays_out`
/// can find the page it switched to.
pub(crate) const PAGE_ROOT: &str = "page-root";

/// The debug selector the content pane's scrolled element carries. Its right
/// edge is the scroll area's, which is where a vertical scrollbar's track ends
/// (gpui-base `src/scrollbar.rs:1408-1432`), so
/// `a_non_overlay_scrollbar_keeps_off_the_content` can see whether the page
/// reaches under the bar.
pub(crate) const CONTENT_SCROLL: &str = "content-scroll";

/// The debug selectors the content column and the inspector carry, each as
/// wide as its resizable panel, so `dragging_a_handle_resizes_its_neighbours`
/// can measure both panels.
pub(crate) const CONTENT_PANEL: &str = "content-panel";
pub(crate) const INSPECTOR_PANEL: &str = "inspector-panel";

/// The debug selector the label with the shown info's title carries.
pub(crate) const INSPECTOR_TITLE: &str = "inspector-title";

/// The debug selector the inspector's TabBar carries.
pub(crate) const INSPECTOR_TABS: &str = "inspector-tabs";

/// The debug selector the inspector's Copy button carries.
pub(crate) const INSPECTOR_COPY: &str = "inspector-copy";

/// The ids and debug selectors of the resizable group's two handles: between
/// the Sidebar and the content, and between the content and the inspector.
pub(crate) const CHROME_HANDLE_NAV: &str = "chrome-resize-sidebar-content";
pub(crate) const CHROME_HANDLE_INSPECTOR: &str = "chrome-resize-content-inspector";

/// The debug selector the toolbar's Toggle Inspector button carries.
pub(crate) const CHROME_TOOLBAR_INSPECTOR: &str = "chrome-toolbar-inspector";

/// The debug selector the toolbar's Command Palette button carries.
pub(crate) const CHROME_TOOLBAR_PALETTE: &str = "chrome-toolbar-palette";

/// The debug selector the window's status bar carries, so
/// `the_status_bar_is_the_bottom_of_the_window` can see where it was laid out.
pub(crate) const CHROME_STATUS_BAR: &str = "chrome-status-bar";

/// The debug selector the status bar's label with the shown info's title
/// carries.
pub(crate) const STATUS_HOVERED: &str = "status-hovered";

/// The debug selector the Alert that reports a theme error carries, at the
/// top of the content panel (spec §2.5).
pub(crate) const CONTENT_ALERT: &str = "content-alert";

/// The debug selectors of the overlays (spec §2.8): the command palette's
/// Command and the title of the Dialog around it, the Preferences sheet's
/// Settings, and the About dialog's link.
pub(crate) const OVERLAY_PALETTE: &str = "overlay-palette";
pub(crate) const OVERLAY_PALETTE_TITLE: &str = "overlay-palette-title";
pub(crate) const OVERLAY_PREFERENCES: &str = "overlay-preferences";
pub(crate) const OVERLAY_ABOUT_LINK: &str = "overlay-about-link";
/// The debug selector of the About dialog's line with this crate's name and
/// version.
pub(crate) const OVERLAY_ABOUT_NAME: &str = "overlay-about-name";
/// The debug selector of the About dialog's description, under that line.
pub(crate) const OVERLAY_ABOUT_TEXT: &str = "overlay-about-text";

/// The debug selectors of the Preferences sheet's three switches, one per
/// `AccessibilityPreferences` flag.
pub(crate) const PREF_REDUCE_MOTION: &str = "pref-reduce-motion";
pub(crate) const PREF_HIGH_CONTRAST: &str = "pref-high-contrast";
pub(crate) const PREF_REDUCE_TRANSPARENCY: &str = "pref-reduce-transparency";

/// The ids and debug selectors of two Buttons of the Buttons page's variant
/// row, which `two_buttons_of_different_variants_show_different_infos`
/// hovers.
pub(crate) const BUTTONS_PRIMARY: &str = "buttons-primary";
pub(crate) const BUTTONS_DANGER: &str = "buttons-danger";
/// The id and debug selector of the Buttons page's Text Button, whose
/// swatches `a_dimmed_swatch_shows_the_painted_colour` reads.
pub(crate) const BUTTONS_TEXT: &str = "buttons-text";
/// The id and debug selector of the Buttons page's disabled Secondary Button.
pub(crate) const BUTTONS_DISABLED_SECONDARY: &str = "buttons-disabled-secondary";
/// The id and debug selector of the Buttons page's first section heading.
pub(crate) const BUTTONS_HEADING_VARIANTS: &str = "buttons-heading-variants";

/// The ids and debug selectors of the Inputs page's first two Checkboxes,
/// one checked and one not when the showcase starts, which
/// `two_checkboxes_in_different_states_show_different_infos` hovers.
pub(crate) const INPUTS_CHECKBOX_NOTIFICATIONS: &str = "inputs-checkbox-notifications";
pub(crate) const INPUTS_CHECKBOX_AUTOSAVE: &str = "inputs-checkbox-autosave";
/// The id and debug selector of the Inputs page's first Input, whose fill
/// `an_input_fill_is_what_input_background_paints` reads.
pub(crate) const INPUTS_FIELD: &str = "inputs-field";
/// The id and debug selector of the Input below it, sized by
/// `geometry::input_height` alone, which
/// `the_height_only_field_takes_the_control_height` measures.
pub(crate) const INPUTS_FIELD_HEIGHT_ONLY: &str = "inputs-field-height-only";

/// The debug selectors the List and the Tree demo boxes carry, so
/// `a_nested_scroller_keeps_the_wheel_to_itself` can put a wheel event inside
/// one and `the_three_list_frames_agree` can measure their frames.
pub(crate) const LIST_DEMO: &str = "list-demo";
pub(crate) const TREE_DEMO: &str = "tree-demo";

/// The ids and debug selectors of the Data page's two Paginations, which
/// `two_paginations_show_different_infos` hovers.
pub(crate) const DATA_PAGINATION: &str = "data-pagination";
pub(crate) const DATA_PAGINATION_COMPACT: &str = "data-pagination-compact";
/// The id and debug selector of the Data page's DataTable header row. Its
/// body rows are `data-table-row-{ix}`, the List's rows `data-list-row-{ix}`
/// and the Tree's `data-tree-row-{ix}`, each its own debug selector too.
pub(crate) const DATA_TABLE_HEADER: &str = "data-table-header";

/// The ids and debug selectors of two of the Feedback page's Tags, which
/// `two_tags_show_different_infos` hovers.
pub(crate) const FEEDBACK_TAG_PRIMARY: &str = "feedback-tag-primary";
pub(crate) const FEEDBACK_TAG_DANGER: &str = "feedback-tag-danger";
/// The ids and debug selectors of the Feedback page's first count Badge and
/// its dot Badge, which `a_count_badge_and_a_dot_badge_show_different_infos`
/// hovers.
pub(crate) const FEEDBACK_BADGE_COUNT: &str = "feedback-badge-messages";
pub(crate) const FEEDBACK_BADGE_DOT: &str = "feedback-badge-updates";
/// The id and debug selector of the Feedback page's Info Alert, whose fill
/// `an_alerts_fill_swatch_is_the_painted_tint` reads.
pub(crate) const FEEDBACK_ALERT_INFO: &str = "feedback-alert-info";
/// The ids and debug selectors of the Feedback page's small Spinner and its
/// indeterminate ProgressCircle, whose infos
/// `an_animations_info_follows_reduced_motion` reads with motion on and off.
pub(crate) const FEEDBACK_SPINNER_SMALL: &str = "feedback-spinner-small";
pub(crate) const FEEDBACK_CIRCLE_LOADING: &str = "feedback-circle-loading";

/// The ids and debug selectors of the Typography page's plain Label and its
/// Label with secondary text, which
/// `a_plain_label_and_a_secondary_one_show_different_infos` hovers.
pub(crate) const TYPOGRAPHY_LABEL_PLAIN: &str = "typography-label-plain";
pub(crate) const TYPOGRAPHY_LABEL_SECONDARY: &str = "typography-label-secondary";
/// The ids and debug selectors of the Typography page's first two heading
/// levels, which `two_heading_levels_show_different_infos` hovers.
pub(crate) const TYPOGRAPHY_H1: &str = "typography-h1";
pub(crate) const TYPOGRAPHY_H2: &str = "typography-h2";

/// The ids and debug selectors of the Layout page's Normal and Outline
/// GroupBoxes, which `two_group_boxes_of_different_variants_show_different_infos`
/// hovers.
pub(crate) const LAYOUT_GROUP_BOX_NORMAL: &str = "layout-group-box-normal";
pub(crate) const LAYOUT_GROUP_BOX_OUTLINE: &str = "layout-group-box-outline";
/// The ids and debug selectors of the Layout page's solid and dashed
/// Separators, which `a_solid_separator_and_a_dashed_one_show_different_infos`
/// hovers.
pub(crate) const LAYOUT_SEPARATOR_SOLID: &str = "layout-separator-solid";
pub(crate) const LAYOUT_SEPARATOR_DASHED: &str = "layout-separator-dashed";
/// The ids and debug selectors of the Layout page's Breadcrumb, whose
/// first link `a_breadcrumb_link_shows_its_page` clicks, and of its
/// Collapsible's toggle and content, which
/// `toggling_the_collapsible_redraws_it` clicks and looks for.
pub(crate) const LAYOUT_BREADCRUMB: &str = "layout-breadcrumb";
pub(crate) const LAYOUT_COLLAPSIBLE: &str = "layout-collapsible";
pub(crate) const LAYOUT_COLLAPSIBLE_TOGGLE: &str = "layout-collapsible-toggle";
pub(crate) const LAYOUT_COLLAPSIBLE_CONTENT: &str = "layout-collapsible-content";

/// The ids and debug selectors of the Overlays page's Dialog trigger, and of
/// the Button and the footer inside the Dialog it opens, which
/// `a_button_inside_the_dialog_shows_the_button_and_its_surface_the_dialog`
/// hovers.
pub(crate) const OVERLAYS_DIALOG_TRIGGER: &str = "overlays-dialog-trigger";
pub(crate) const OVERLAYS_DIALOG_CLOSE: &str = "overlays-dialog-close";
pub(crate) const OVERLAYS_DIALOG_FOOTER: &str = "overlays-dialog-footer";
/// The ids and debug selectors of the Overlays page's two Sheet triggers and
/// of the titles of the Sheets they open, which
/// `a_right_sheet_and_a_bottom_sheet_show_different_infos` hovers.
pub(crate) const OVERLAYS_SHEET_RIGHT: &str = "overlays-sheet-right";
pub(crate) const OVERLAYS_SHEET_BOTTOM: &str = "overlays-sheet-bottom";
pub(crate) const OVERLAYS_SHEET_RIGHT_TITLE: &str = "overlays-sheet-right-title";
pub(crate) const OVERLAYS_SHEET_BOTTOM_TITLE: &str = "overlays-sheet-bottom-title";

/// The ids and debug selectors of the Charts page's five charts, which
/// `every_chart_shows_its_own_info` hovers.
pub(crate) const CHARTS_BAR_CHART: &str = "charts-bar-chart";
pub(crate) const CHARTS_LINE_CHART: &str = "charts-line-chart";
pub(crate) const CHARTS_AREA_CHART: &str = "charts-area-chart";
pub(crate) const CHARTS_PIE_CHART: &str = "charts-pie-chart";
pub(crate) const CHARTS_CANDLESTICK_CHART: &str = "charts-candlestick-chart";

// ---------------------------------------------------------------------------
// Debug selectors for the interactive controls
// ---------------------------------------------------------------------------
//
// `interactive_controls_respond` clicks each of these but `PROBE_COMBOBOX`
// and asks the model what changed. A control that carries one is a control
// the self-tests drive; the name is shared by the render code and the tests,
// so neither can drift onto an element the other does not mean.
pub(crate) const PROBE_RATING: &str = "probe-rating";
/// The toolbar's preset Combobox, which `the_toolbar_switches_the_preset`
/// drives and `the_toolbar_is_the_models_toolbar` measures (the latter with
/// `PROBE_COLOR_MODE`).
pub(crate) const PROBE_COMBOBOX: &str = "probe-combobox";
pub(crate) const PROBE_CLIPBOARD: &str = "probe-clipboard";
pub(crate) const PROBE_PAGINATION: &str = "probe-pagination";
pub(crate) const PROBE_ATTACHMENT: &str = "probe-attachment";
pub(crate) const PROBE_CHAT_SEND: &str = "probe-chat-send";
pub(crate) const PROBE_STEPPER: &str = "probe-stepper";
pub(crate) const PROBE_CAROUSEL_LAST: &str = "probe-carousel-last";
pub(crate) const PROBE_ALERT_DIALOG: &str = "probe-alert-dialog";
pub(crate) const PROBE_NOTIFICATION: &str = "probe-notification";
pub(crate) const PROBE_COLOR_MODE: &str = "probe-color-mode";

/// The debug selector a full-width item in the Settings demo's first group
/// carries. Nothing clicks it: its right edge is where a Settings row ends,
/// which `a_settings_row_keeps_off_the_page_scrollbar` measures against the
/// page's scrollbar.
pub(crate) const PROBE_SETTINGS_ROW: &str = "probe-settings-row";

/// Tag a control with a debug selector, so the self-test can find what it has
/// to click. The wrapper is a plain box around the control and leaves the
/// layout to it.
pub(crate) fn probe(selector: &'static str, control: impl IntoElement) -> Div {
    div().debug_selector(move || selector.into()).child(control)
}

/// The window the showcase opens at `bounds`: upstream's options for a
/// window that renders a `TitleBar` (title_bar.rs, `TitleBar::window_options`),
/// asking to draw its own decorations, so the `TitleBar` is the window's title
/// bar (spec §1.2). The self-tests open their windows with it too.
pub(crate) fn window_options(bounds: Bounds<Pixels>) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        window_decorations: Some(WindowDecorations::Client),
        ..TitleBar::window_options()
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

    /// Map a `--tab` name to the page it names. The flag keeps its name:
    /// the screenshot scripts pass it.
    fn page(name: &str) -> Option<Page> {
        match name {
            "buttons" => Some(Page::Buttons),
            "inputs" | "text-inputs" => Some(Page::Inputs),
            "data" => Some(Page::Data),
            "feedback" => Some(Page::Feedback),
            "typography" => Some(Page::Typography),
            "layout" => Some(Page::Layout),
            "overlays" => Some(Page::Overlays),
            "charts" => Some(Page::Charts),
            "icons" => Some(Page::Icons),
            "theme-map" => Some(Page::ThemeMap),
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
            app::init(cx);

            // Apply CLI variant override before window opens so the initial
            // theme is resolved with the correct light/dark setting.
            let variant_override = cli_args.variant.as_deref().map(|v| v == "dark");

            let bounds = Bounds::centered(None, WINDOW_SIZE, cx);
            let window_handle = cx.open_window(window_options(bounds), |window, cx| {
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
                    }

                    // Override theme if --theme was specified
                    if let Some(ref theme_name) = cli_args.theme {
                        s.current_theme_name = theme_name.clone();
                        s.apply_theme_by_name(theme_name, window, cx);
                        // Show the overridden theme in the toolbar's preset switch
                        let key = SharedString::from(theme_name.clone());
                        s.preset_combobox.update(cx, |combobox, cx| {
                            combobox.set_selected_values(&[key], window, cx);
                        });
                    }

                    // Override the page if --tab was specified
                    if let Some(ref page_name) = cli_args.tab
                        && let Some(page) = CliArgs::page(page_name)
                    {
                        s.active_page = page;
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
                        let effective = s.icon_set_choice.effective_icon_set(s.current_icon_set);
                        let default_theme =
                            s.icon_set_choice.freedesktop_theme().map(|t| t.to_string());
                        s.icon_set_name = effective.name().to_string();
                        s.icon_set_enum = Some(effective);
                        let cli_ref = s.icon_theme_override.as_deref();
                        let fc = s.original_font.color;
                        let fg_rgb = Some([fc.r, fc.g, fc.b]);
                        s.loaded_icons =
                            load_all_icons(effective, default_theme.as_deref(), cli_ref, fg_rgb);
                        s.gpui_icons = load_gpui_icons(
                            Some(effective),
                            default_theme.as_deref(),
                            cli_ref,
                            fg_rgb,
                        );
                        let fg = cx.theme().foreground;
                        s.rebuild_icon_caches(fg, window, cx);
                        s.rebuild_animation_caches(window, cx);
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
            });
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

#[cfg(test)]
mod tests;
