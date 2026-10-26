//! native-theme-gpui — comprehensive widget showcase and designer reference.
//!
//! A full gpui-component widget gallery that documents, in the inspector's
//! Widget tab, every theme-controlled property of the widget under the
//! pointer. Demonstrates all gpui-component widgets,
//! every `ThemeColor` field, every `IconName` variant, and live theme
//! switching across all bundled `native-theme` presets.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-gpui --example showcase-gpui
//! ```
//!
//! To have it follow the OS theme's changes (its accent, fonts and icon
//! theme in every colour mode, and its light or dark under System), add the
//! `watch` feature, which `native_theme_gpui::on_theme_change` needs:
//!
//! ```sh
//! cargo run -p native-theme-gpui --example showcase-gpui --features watch
//! ```
//!
//! # What to look for
//!
//! - The side panel's theme settings switch theme presets, color modes and
//!   icon themes without restarting the app. Watch how the entire widget tree re-themes
//!   on each change — no manual rewiring per widget.
//! - Hover any widget and the inspector's Widget tab, in the side panel,
//!   shows which `ResolvedTheme` fields drive its appearance.
//! - The Theme Map page exposes the full 139-field `ThemeColor` palette that
//!   gpui-component exposes, with each field's current value and the
//!   `native-theme` field it was derived from.
//! - The Icons page demonstrates `IconRole` mapping across Material, Lucide,
//!   and freedesktop sets, plus animated spinner playback.
//!
//! # How the source is organised
//!
//! `main.rs` holds the entry point and what the command line does to the
//! window, its screenshot and its layout dump, whose flags, capture and dump
//! format are the ones the three showcases share
//! (`native_theme_showcase_support`); `app.rs` the `Showcase` view, its
//! state and theme switching; `pages/` one module per page; `chrome.rs` and
//! `demo.rs` the window's chrome and the helpers that build a widget with
//! its info; `info/` what each widget reports; `inspector.rs` the
//! inspector; `elements.rs` the elements the three showcases share; `host.rs`
//! the actions hosted around the whole window; `speech.rs` the synthetic
//! audio input and the silent recognizer the Inputs page's SpeechWaveform
//! runs on; `tests.rs` the self-tests; and `support.rs` the sample content,
//! helpers, icon loading and delegates the pages share. Within a file,
//! section-divider blocks (`// ─────`) separate one widget category, page or
//! view from the next.

// GPUI through gpui-kit, which re-exports the core it is built on: gpui-pre,
// or GPUI Fast under `gpui-kit/gpui-fast` (gpui-kit `src/lib.rs:94, 101-103`).
extern crate gpui_kit as gpui;

mod app;
mod chrome;
mod demo;
mod elements;
mod host;
mod info;
mod inspector;
mod pages;
mod speech;
mod support;

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnyWindowHandle, App, Bounds, Div, IntoElement, Modifiers, MouseButton, MouseDownEvent,
    MouseMoveEvent, ParentElement, Pixels, PlatformInput, WindowBounds, WindowDecorations,
    WindowOptions, div, prelude::*, px, size,
};
use gpui_component::IconName;

use native_theme_showcase_support::cli::{
    self, CliArgs, IconPick, Output, Outputs, ThemeKey, reported,
};
use native_theme_showcase_support::{Kind, capture, dump};

use crate::app::Showcase;

// ---------------------------------------------------------------------------
// Pages
// ---------------------------------------------------------------------------

/// The pages the content panel's TabBar switches between (spec §2.4).
///
/// `Showcase::render` matches on this, so a new variant cannot be added without
/// the compiler asking what it renders, and the TabBar's tabs, the View menu,
/// the `--tab` names and the layout self-test all read [`Page::ALL`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Page {
    /// The controls all three showcases draw, alike, on one screen: the page
    /// the showcase opens on.
    Basic,
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
    /// Every page, in the order the TabBar lists them.
    ///
    /// A new variant forces an arm in [`Page::index`], [`Page::label`],
    /// [`Page::icon`] and [`Page::tab`], whose matches are exhaustive,
    /// and the index it is given there has to be its position in this array
    /// — the `const` block below rejects the build otherwise. The one thing
    /// neither the compiler nor that block can see is a variant added to the
    /// enum and to the matches but not to this list: it would take an index
    /// the array does not have, and the assertion fires.
    const ALL: [Self; 11] = [
        Self::Basic,
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

    /// The page's position in the TabBar, which is what `ShowPage` carries.
    const fn index(self) -> usize {
        match self {
            Self::Basic => 0,
            Self::Buttons => 1,
            Self::Inputs => 2,
            Self::Data => 3,
            Self::Feedback => 4,
            Self::Typography => 5,
            Self::Layout => 6,
            Self::Overlays => 7,
            Self::Charts => 8,
            Self::Icons => 9,
            Self::ThemeMap => 10,
        }
    }

    /// The page at a TabBar position; `None` past the end.
    fn at(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    /// The label the page's tab, the View menu and the command palette show.
    const fn label(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
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

    /// The name `--tab` takes for the page.
    const fn flag(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Buttons => "buttons",
            Self::Inputs => "inputs",
            Self::Data => "data",
            Self::Feedback => "feedback",
            Self::Typography => "typography",
            Self::Layout => "layout",
            Self::Overlays => "overlays",
            Self::Charts => "charts",
            Self::Icons => "icons",
            Self::ThemeMap => "theme-map",
        }
    }

    /// The page `--tab` names: the one whose [`Page::flag`] it is
    /// (`cli::page_key`, whose error lists every page's).
    fn named(name: &str) -> Result<Self, String> {
        let flags = Self::ALL.map(Self::flag);
        cli::page_key(name, &flags).and_then(|flag| {
            Self::ALL
                .into_iter()
                .find(|page| page.flag() == flag)
                .ok_or_else(|| format!("--tab {flag}: no page has that key"))
        })
    }

    /// The icon the page's command-palette entry shows.
    const fn icon(self) -> IconName {
        match self {
            Self::Basic => IconName::Eye,
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

    /// The debug selector of the page's tab in the content panel's TabBar,
    /// so `the_page_tabs_navigate` can click the tab the render code built.
    const fn tab(self) -> &'static str {
        match self {
            Self::Basic => "chrome-page-tab-basic",
            Self::Buttons => "chrome-page-tab-buttons",
            Self::Inputs => "chrome-page-tab-inputs",
            Self::Data => "chrome-page-tab-data",
            Self::Feedback => "chrome-page-tab-feedback",
            Self::Typography => "chrome-page-tab-typography",
            Self::Layout => "chrome-page-tab-layout",
            Self::Overlays => "chrome-page-tab-overlays",
            Self::Charts => "chrome-page-tab-charts",
            Self::Icons => "chrome-page-tab-icons",
            Self::ThemeMap => "chrome-page-tab-theme-map",
        }
    }
}

/// `Page::ALL` is in TabBar order and holds each page once.
const _: () = {
    let mut pages: &[Page] = &Page::ALL;
    let mut i = 0;
    while let [page, rest @ ..] = pages {
        assert!(
            page.index() == i,
            "Page::ALL is not the pages in TabBar order"
        );
        pages = rest;
        i += 1;
    }
};

/// The window the showcase opens, width × height in logical pixels: the
/// shared crate's `WINDOW_SIZE_PX`, 1280 × 720, the maintainer's default for
/// every showcase. The model states no window size (spec §1.3); a page
/// taller than the window scrolls. `f32`s, because gpui's `Pixels` has no
/// `const` arithmetic; `WINDOW_SIZE` is this size.
const WINDOW_SIZE_PX: (f32, f32) = (
    native_theme_showcase_support::WINDOW_SIZE_PX.0 as f32,
    native_theme_showcase_support::WINDOW_SIZE_PX.1 as f32,
);

/// The width, in logical pixels, the content panel opens at: the window's
/// width less `LEFT_PANEL_WIDTH`.
#[cfg(test)]
pub(crate) const PAGE_WIDTH_PX: f32 = WINDOW_SIZE_PX.0 - LEFT_PANEL_WIDTH_PX;

/// The window the showcase opens, `WINDOW_SIZE_PX`: `LEFT_PANEL_WIDTH` + the
/// pages' width wide. The self-tests lay the interface out at this width, so
/// a measurement they take is a measurement of the real thing.
pub(crate) const WINDOW_SIZE: gpui::Size<Pixels> = size(px(WINDOW_SIZE_PX.0), px(WINDOW_SIZE_PX.1));

/// The window's title: this crate's name and version. The title bar's label,
/// the title the OS shows, and `--screenshot`'s capture, which finds the
/// window by it (`capture::FrameCapture`), all take this one string.
pub(crate) const WINDOW_TITLE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    " showcase"
);

/// Give `window` the title the OS shows, as `main` does once the window is
/// open.
pub(crate) fn name_window(window: &mut gpui::Window) {
    window.set_window_title(WINDOW_TITLE);
}

/// The debug selector the window's title bar carries, drawn where the window
/// was granted client-side decorations, so
/// `under_client_decorations_the_title_bar_holds_the_menus` can see where it
/// was laid out.
pub(crate) const CHROME_TITLE_BAR: &str = "chrome-title-bar";

/// The debug selector the application's menus carry (`demo::app_menus`), in
/// the menu-bar row or in the title bar.
pub(crate) const CHROME_APP_MENU_BAR: &str = "chrome-app-menu-bar";

/// The debug selector of the menu-bar row at the top of a window whose frame
/// the window manager draws, above the toolbar (spec S8).
pub(crate) const CHROME_MENU_BAR: &str = "chrome-menu-bar";

/// The debug selector the window's toolbar carries, so
/// `the_toolbar_is_the_models_toolbar` can measure it.
pub(crate) const CHROME_TOOLBAR: &str = "chrome-toolbar";

/// The id and debug selector of the side panel: the theme settings, a
/// Separator and the inspector, the body's first panel.
pub(crate) const CHROME_SIDE_PANEL: &str = "chrome-side-panel";

/// The debug selector the status bar's side-panel toggle carries, the
/// status bar's first item.
pub(crate) const CHROME_SIDE_PANEL_TOGGLE: &str = "chrome-side-panel-toggle";

/// The id and debug selector of the theme settings at the top of the side
/// panel, one labelled row each.
pub(crate) const CHROME_THEME_SETTINGS: &str = "chrome-theme-settings";

/// The ids and debug selectors of the theme settings' three labels, in
/// their order: Theme, Mode, Icon theme.
pub(crate) const CHROME_LABEL_THEME: &str = "chrome-label-theme";
pub(crate) const CHROME_LABEL_MODE: &str = "chrome-label-mode";
pub(crate) const CHROME_LABEL_ICON_THEME: &str = "chrome-label-icon-theme";

/// The id and debug selector of the Separator between the theme settings
/// and the inspector.
pub(crate) const CHROME_SIDE_PANEL_SEPARATOR: &str = "chrome-side-panel-separator";

/// The initial width of the side panel. The model states no such width
/// (spec §1.3), so this is the showcase's own layout default, and dragging
/// the panel's handle changes it.
///
/// It is 300px, the width the inspector's content was laid out for.
/// `the_side_panel_holds_the_theme_settings_and_the_inspector`
/// checks that the theme settings and the inspector fit it under every
/// bundled preset, a native one at its own platform's DPI, at text scale 1
/// and at 2. `default`, the desktop's own theme, is built on one of them.
pub(crate) const LEFT_PANEL_WIDTH: Pixels = px(LEFT_PANEL_WIDTH_PX);
const LEFT_PANEL_WIDTH_PX: f32 = 300.;

/// The debug selector the active page's root carries, so `every_page_lays_out`
/// can find the page it switched to.
pub(crate) const PAGE_ROOT: &str = "page-root";

/// The debug selector the content pane's scrolled element carries. Its right
/// edge is the scroll area's, which is where a vertical scrollbar's track ends
/// (gpui-base `src/scrollbar.rs:1422-1446`), so
/// `a_non_overlay_scrollbar_keeps_off_the_content` can see whether the page
/// reaches under the bar.
pub(crate) const CONTENT_SCROLL: &str = "content-scroll";

/// The debug selector the content column carries, as wide as its resizable
/// panel, so `dragging_the_handle_resizes_both_panels` can measure it.
pub(crate) const CONTENT_PANEL: &str = "content-panel";

/// The debug selector of the line the side panel's resize handle paints.
pub(crate) const CHROME_SPLITTER_LINE: &str = "chrome-splitter-line";

/// The debug selector of the box a capture lays over the whole window so
/// the pointer hovers nothing (`Showcase::pointer_shield`).
pub(crate) const POINTER_SHIELD: &str = "pointer-shield";

/// The debug selector the content panel's TabBar carries, above the page.
pub(crate) const CHROME_PAGE_TABS: &str = "chrome-page-tabs";

/// The debug selector the inspector's root carries, in the side panel.
pub(crate) const INSPECTOR_PANEL: &str = "inspector-panel";

/// The debug selector the label with the shown info's title carries.
pub(crate) const INSPECTOR_TITLE: &str = "inspector-title";

/// The debug selector the inspector's TabBar carries.
pub(crate) const INSPECTOR_TABS: &str = "inspector-tabs";

/// The debug selector the inspector's Copy button carries.
pub(crate) const INSPECTOR_COPY: &str = "inspector-copy";

/// The debug selector of the inspector's note that, with no native theme
/// installed, the swatches may not be the colours painted.
pub(crate) const INSPECTOR_TOKENS_NOTE: &str = "inspector-tokens-note";

/// The id and debug selector of the resizable group's handle, between the
/// side panel and the content.
pub(crate) const CHROME_HANDLE: &str = "chrome-resize-side-panel-content";

/// The debug selectors the toolbar's three buttons carry, in their order:
/// Command Palette, Reload System Theme and Preferences.
pub(crate) const CHROME_TOOLBAR_PALETTE: &str = "chrome-toolbar-palette";
pub(crate) const CHROME_TOOLBAR_RELOAD: &str = "chrome-toolbar-reload";
pub(crate) const CHROME_TOOLBAR_PREFERENCES: &str = "chrome-toolbar-preferences";

/// The debug selector the window's status bar carries, so
/// `the_status_bar_is_the_bottom_of_the_window` can see where it was laid out.
pub(crate) const CHROME_STATUS_BAR: &str = "chrome-status-bar";

/// The debug selector the status bar's label with the shown info's title
/// carries.
pub(crate) const STATUS_HOVERED: &str = "status-hovered";

/// The debug selector the status bar's environment text carries.
pub(crate) const STATUS_ENVIRONMENT: &str = "status-environment";

/// The debug selector of an empty box that fills the status bar's middle
/// region, which holds nothing: its edges are where the bar's two ends stop,
/// so `the_status_bar_carries_no_version` can account for everything drawn
/// between them.
pub(crate) const STATUS_MIDDLE: &str = "status-middle";

/// The debug selector the Alert that reports a theme error carries, in the
/// content panel under the page TabBar and above the page (spec §2.5).
pub(crate) const CONTENT_ALERT: &str = "content-alert";

/// The debug selectors of the overlays (spec §2.8): the command palette's
/// Command and the title of the Dialog around it, the Preferences sheet's
/// Settings, and the About dialog's link.
pub(crate) const OVERLAY_PALETTE: &str = "overlay-palette";
pub(crate) const OVERLAY_PALETTE_TITLE: &str = "overlay-palette-title";
pub(crate) const OVERLAY_PREFERENCES: &str = "overlay-preferences";
pub(crate) const OVERLAY_ABOUT_LINK: &str = "overlay-about-link";
/// The debug selector of the About dialog's title.
pub(crate) const OVERLAY_ABOUT_TITLE: &str = "overlay-about-title";
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
/// `a_refined_input_is_filled_with_the_platforms_fill` reads.
pub(crate) const INPUTS_FIELD: &str = "inputs-field";
/// The id and debug selector of the Basic page's disabled Input, whose fill
/// `a_refined_input_is_filled_with_the_platforms_fill` reads.
pub(crate) const BASIC_INPUT_DISABLED: &str = "basic-input-disabled";
/// The id and debug selector of the Input below it, sized by
/// `geometry::input_height` alone, which
/// `the_height_only_field_takes_the_height_rule` measures.
pub(crate) const INPUTS_FIELD_HEIGHT_ONLY: &str = "inputs-field-height-only";
/// The id and debug selector of the Inputs page's Textarea, whose own
/// height `the_textarea_keeps_its_own_height` measures.
pub(crate) const INPUTS_TEXTAREA: &str = "inputs-textarea";

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
/// The id and debug selector of the Layout page's Folder TabBar, its tabs'
/// labels and the one of them disabled, which
/// `the_folder_tab_bar_paints_its_bar_and_separators_in_their_claims` and
/// `a_click_selects_a_folder_tab_and_not_the_disabled_one` measure and click.
pub(crate) const LAYOUT_FOLDER_TABS: &str = "layout-folder-tabs";
pub(crate) const LAYOUT_FOLDER_TAB_LABELS: [&str; 4] = ["One", "Two", "Three", "Disabled"];
pub(crate) const LAYOUT_FOLDER_TAB_DISABLED: usize = 3;
/// The id and debug selector of the Layout page's TitleBar sample, drawn
/// while the window manager draws the window's own frame (spec S8).
pub(crate) const LAYOUT_TITLE_BAR: &str = "layout-title-bar";
/// The ids and debug selectors of the Layout page's two Sidebar samples,
/// expanded and collapsed, and of their items, as `(label, icon, id in the
/// expanded sample, id in the collapsed one)`, which
/// `the_sidebar_samples_icons_fit_their_items` measures.
pub(crate) const LAYOUT_SIDEBAR_EXPANDED: &str = "layout-sidebar-expanded";
pub(crate) const LAYOUT_SIDEBAR_COLLAPSED: &str = "layout-sidebar-collapsed";
pub(crate) const LAYOUT_SIDEBAR_ITEMS: [(&str, IconName, &str, &str); 3] = [
    (
        "Dashboard",
        IconName::LayoutDashboard,
        "layout-sidebar-expanded-dashboard",
        "layout-sidebar-collapsed-dashboard",
    ),
    (
        "Inbox",
        IconName::Inbox,
        "layout-sidebar-expanded-inbox",
        "layout-sidebar-collapsed-inbox",
    ),
    (
        "Settings",
        IconName::Settings,
        "layout-sidebar-expanded-settings",
        "layout-sidebar-collapsed-settings",
    ),
];

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

/// The ids and debug selectors of the Charts page's seven charts, which
/// `every_chart_shows_its_own_info` hovers.
pub(crate) const CHARTS_BAR_CHART: &str = "charts-bar-chart";
pub(crate) const CHARTS_LINE_CHART: &str = "charts-line-chart";
pub(crate) const CHARTS_AREA_CHART: &str = "charts-area-chart";
pub(crate) const CHARTS_PIE_CHART: &str = "charts-pie-chart";
pub(crate) const CHARTS_CANDLESTICK_CHART: &str = "charts-candlestick-chart";
pub(crate) const CHARTS_RADAR_CHART: &str = "charts-radar-chart";
pub(crate) const CHARTS_SANKEY_CHART: &str = "charts-sankey-chart";

// ---------------------------------------------------------------------------
// Debug selectors for the interactive controls
// ---------------------------------------------------------------------------
//
// `interactive_controls_respond` clicks each of these but `PROBE_COMBOBOX`
// and asks the model what changed. A control that carries one is a control
// the self-tests drive; the name is shared by the render code and the tests,
// so neither can drift onto an element the other does not mean.
pub(crate) const PROBE_RATING: &str = "probe-rating";
/// The theme settings' preset Combobox, which
/// `the_theme_settings_switch_the_preset` drives and
/// `the_side_panel_holds_the_theme_settings_and_the_inspector` measures.
pub(crate) const PROBE_COMBOBOX: &str = "probe-combobox";
pub(crate) const PROBE_CLIPBOARD: &str = "probe-clipboard";
pub(crate) const PROBE_PAGINATION: &str = "probe-pagination";
pub(crate) const PROBE_ATTACHMENT: &str = "probe-attachment";
pub(crate) const PROBE_CHAT_SEND: &str = "probe-chat-send";
pub(crate) const PROBE_STEPPER: &str = "probe-stepper";
pub(crate) const PROBE_CAROUSEL_LAST: &str = "probe-carousel-last";
pub(crate) const PROBE_ALERT_DIALOG: &str = "probe-alert-dialog";
pub(crate) const PROBE_NOTIFICATION: &str = "probe-notification";
/// The theme settings' colour-mode Select, which
/// `interactive_controls_respond` drives.
pub(crate) const PROBE_COLOR_MODE: &str = "probe-color-mode";
/// The theme settings' icon-theme Select, which
/// `the_theme_settings_switch_the_icon_theme` drives.
pub(crate) const PROBE_ICON_THEME: &str = "probe-icon-theme";

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

/// The window the showcase opens at `bounds`, asking the window manager to
/// draw its frame, so that whatever it draws -- on KDE, Breeze's title bar,
/// controls, corners and shadow -- looks native (spec S8). The self-tests open their windows with it too.
///
/// gpui's default options keep the system's title bar: macOS and Windows
/// hide it only for a titlebar that `appears_transparent` (gpui-pre-macos
/// window.rs, `MacWindow::open`; gpui-pre-windows window.rs,
/// `WindowsWindow::new`), and neither reports anything but server-side
/// decorations (gpui-pre platform.rs, `PlatformWindow::window_decorations`).
/// So nothing of `TitleBar::window_options` applies: its transparent
/// titlebar, traffic-light position and title-bar drag are for a window whose
/// `TitleBar` is its title bar on those two, and the Linux backends read no
/// titlebar option but the title. A Linux compositor that grants no
/// server-side decorations still gets the `TitleBar` (`Showcase::frame`).
pub(crate) fn window_options(bounds: Bounds<Pixels>) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        window_decorations: Some(WindowDecorations::Server),
        ..WindowOptions::default()
    }
}

/// `window_options` for a window that is captured (`--screenshot` or
/// `--capture`): it opens at `WINDOW_SIZE` whatever size a desktop stored for
/// the showcase's window. gpui itself stores none; a desktop can, by the
/// window's app id (a KWin script that remembers window geometry, say), so
/// the window takes an app id of this process's own, which nothing stored
/// (`capture::app_id`).
pub(crate) fn capture_window_options(bounds: Bounds<Pixels>) -> WindowOptions {
    WindowOptions {
        app_id: Some(capture::app_id(Kind::Gpui)),
        ..window_options(bounds)
    }
}

// ---------------------------------------------------------------------------
// The command line (`native_theme_showcase_support::cli`)
// ---------------------------------------------------------------------------

/// How often `hold_pointer` hands the window the held pointer again, which
/// the window's own pointer events could otherwise move away. The model
/// states no such period; it is the showcase's.
const HELD_POINTER_PERIOD: Duration = Duration::from_millis(250);

/// How long `hold_pointer` holds the pointer before the press: long enough for
/// the window to open at its size and the page to be laid out under the
/// pointer. No repository script passes `--press`. The capture scripts
/// capture a showcase at least four seconds after starting it
/// (`SHOWCASE_START_DELAY=3`, then `capture_showcase`'s `sleep 1`, in
/// `scripts/capture_window.sh`), so a press three seconds after the first
/// frame lands about a second before such a capture; a capture with `--press`
/// should wait longer.
const HELD_PRESS_AFTER: Duration = Duration::from_secs(3);

/// `--pointer` and `--press`: hands the window a pointer move to `at` every
/// `HELD_POINTER_PERIOD`, and after `HELD_PRESS_AFTER` a press of the primary
/// button there when `press`, through `Window::dispatch_event`, the entry the
/// platform's own input takes (gpui-pre 0.3.9 `src/window.rs`), until the
/// window closes.
fn hold_pointer(cx: &mut App, window: AnyWindowHandle, at: (u16, u16), press: bool) {
    let position = gpui::point(px(f32::from(at.0)), px(f32::from(at.1)));
    cx.spawn(async move |cx| {
        let mut held = Duration::ZERO;
        let mut pressed = false;
        loop {
            cx.background_executor().timer(HELD_POINTER_PERIOD).await;
            held = held.saturating_add(HELD_POINTER_PERIOD);
            let press_now = press && !pressed && held >= HELD_PRESS_AFTER;
            let delivered = cx.update_window(window, |_view, window, cx| {
                window.dispatch_event(
                    PlatformInput::MouseMove(MouseMoveEvent {
                        position,
                        pressed_button: pressed.then_some(MouseButton::Left),
                        modifiers: Modifiers::default(),
                    }),
                    cx,
                );
                if press_now {
                    window.dispatch_event(
                        PlatformInput::MouseDown(MouseDownEvent {
                            button: MouseButton::Left,
                            position,
                            modifiers: Modifiers::default(),
                            click_count: 1,
                            first_mouse: false,
                        }),
                        cx,
                    );
                }
            });
            if delivered.is_err() {
                break;
            }
            pressed |= press_now;
        }
    })
    .detach();
}

/// How often `dump_layout` reads where the last frame drew the list's
/// elements. The showcase's own period, as `HELD_POINTER_PERIOD` is.
const DUMP_POLL: Duration = Duration::from_millis(250);

/// How many readings in a row `dump_layout` must find the same before it
/// takes the layout as settled: a second's worth, long enough for the icons
/// and fonts loaded after the first frame to have taken their place.
const DUMP_SETTLED_READINGS: usize = 4;

/// How many readings `dump_layout` takes at most before it gives the dump
/// up as failed, so a layout that never settles fails the run (exit 1)
/// rather than hold it open: 80 readings `DUMP_POLL` apart, the shared
/// crate's `CAPTURE_WAIT`, 20 s, the time `scripts/capture_window.sh` waits
/// for a dump before it fails the capture (`wait_for_dump`), which
/// `the_dump_readings_take_the_capture_wait` holds. The showcase's own
/// bound; the model states none.
const DUMP_MAX_READINGS: usize = 80;

/// What `dump_layout` does after a reading ([`DumpSettle::read`]).
#[derive(Debug, PartialEq)]
pub(crate) enum DumpStep {
    /// The layout has not settled yet: read again after `DUMP_POLL`.
    Wait,
    /// `DUMP_SETTLED_READINGS` readings in a row found this layout: write it.
    Write(serde_json::Value),
    /// `DUMP_MAX_READINGS` readings found no settled layout: the dump fails.
    GiveUp,
}

/// The readings `dump_layout` has taken: the last one, how many in a row
/// found it, and how many there were.
#[derive(Default)]
pub(crate) struct DumpSettle {
    last: Option<serde_json::Value>,
    same: usize,
    readings: usize,
}

impl DumpSettle {
    /// Takes `reading`, one more of the last frame's layout dumps: a dump
    /// with no element, or one unlike the last, starts the count again.
    pub(crate) fn read(&mut self, reading: serde_json::Value) -> DumpStep {
        self.readings = self.readings.saturating_add(1);
        let empty = reading
            .get("elements")
            .and_then(serde_json::Value::as_object)
            .is_none_or(serde_json::Map::is_empty);
        if empty || self.last.as_ref() != Some(&reading) {
            self.last = Some(reading);
            self.same = 1;
        } else {
            self.same = self.same.saturating_add(1);
        }
        if !empty
            && self.same >= DUMP_SETTLED_READINGS
            && let Some(settled) = self.last.take()
        {
            return DumpStep::Write(settled);
        }
        if self.readings >= DUMP_MAX_READINGS {
            DumpStep::GiveUp
        } else {
            DumpStep::Wait
        }
    }
}

/// The layout dump of the last frame `window` drew (`dump::layout_json`):
/// where each element of docs/showcase-elements.toml was drawn, the theme,
/// its variant and the scale factor.
fn layout_dump(s: &Showcase, window: &gpui::Window, cx: &App) -> serde_json::Value {
    let mut drawn = s.info_ui.read(cx).layout_drawn();
    // The fields' text, which their states report
    // (`Showcase::field_text_bounds`), on the Basic page.
    if s.active_page == Page::Basic {
        drawn.extend(s.field_text_bounds(cx));
    }
    let variant = if s.is_dark { "dark" } else { "light" };
    let rects = drawn.into_iter().map(|(id, b)| {
        let rect = dump::Rect {
            x: b.origin.x.as_f32(),
            y: b.origin.y.as_f32(),
            w: b.size.width.as_f32(),
            h: b.size.height.as_f32(),
        };
        (id.to_string(), rect)
    });
    dump::layout_json(
        Kind::Gpui,
        &s.current_theme_name,
        variant,
        window.scale_factor(),
        rects,
    )
}

/// The files the run asked for and which are written yet (`cli::Outputs`),
/// shared by the tasks that write them.
pub(crate) type RunOutputs = Rc<Cell<Outputs>>;

/// Records `output` as written in `outputs`, and quits once every file the
/// run asked for is written and no `--capture` keeps the window
/// (`cli::Outputs::written`): the showcases' exit rule, so the last of
/// `--dump-layout F` and `--screenshot P` to be written quits.
fn output_written(outputs: &RunOutputs, output: Output, cx: &mut App) {
    let mut now = outputs.get();
    let quit = now.written(output);
    outputs.set(now);
    if quit {
        app::quit(cx);
    }
}

/// `--dump-layout`: once `DUMP_SETTLED_READINGS` readings in a row find the
/// last frame's layout of the list's elements unchanged, writes it to
/// `path` as docs/showcase-elements.toml's "The layout dump" describes, then
/// records it in `outputs` (`output_written`). A write that fails, or a
/// layout that has not settled after `DUMP_MAX_READINGS` readings, is
/// reported naming `--dump-layout` and the path (`cli::Output::failure`)
/// and exits with 1 (`cli::refuse`), as a failed `--screenshot` does.
pub(crate) fn dump_layout(
    cx: &mut App,
    window: AnyWindowHandle,
    showcase: gpui::Entity<Showcase>,
    path: String,
    outputs: RunOutputs,
) {
    cx.spawn(async move |cx| {
        let mut settle = DumpSettle::default();
        loop {
            cx.background_executor().timer(DUMP_POLL).await;
            let reading = cx.update_window(window, |_view, window, cx| {
                layout_dump(showcase.read(cx), window, cx)
            });
            let Ok(reading) = reading else { break };
            match settle.read(reading) {
                DumpStep::Wait => {}
                DumpStep::Write(settled) => {
                    if let Err(error) = dump::write_layout(&path, &settled) {
                        cli::refuse(&error);
                    }
                    cx.update(|cx| output_written(&outputs, Output::Dump, cx));
                    break;
                }
                DumpStep::GiveUp => cli::refuse(&dump_unsettled(&path)),
            }
        }
    })
    .detach();
}

/// Why `--dump-layout` to `path` failed when its layout never settled.
pub(crate) fn dump_unsettled(path: &str) -> String {
    let waited = DUMP_POLL.saturating_mul(u32::try_from(DUMP_MAX_READINGS).unwrap_or(u32::MAX));
    Output::Dump.failure(
        path,
        &format!(
            "the layout did not settle: no {DUMP_SETTLED_READINGS} readings in a row agreed \
             within {} s",
            waited.as_secs()
        ),
    )
}

/// How often `screenshot` asks whether the capture is done: on Windows the
/// second run finding the window takes a moment. The showcase's own
/// period, as `DUMP_POLL` is.
const SCREENSHOT_POLL: Duration = Duration::from_millis(50);

/// How long `screenshot` waits after the window opened before it captures
/// it: on macOS a 200 ms nudge of the content size and 1300 ms after it
/// (`capture::nudge_content_size`), on Windows 1500 ms, as the macOS and
/// Windows runners' captures were timed. The showcase's own delays.
#[cfg(target_os = "macos")]
const SCREENSHOT_NUDGE: Duration = Duration::from_millis(200);
#[cfg(target_os = "macos")]
const SCREENSHOT_SETTLE: Duration = Duration::from_millis(1300);
#[cfg(not(target_os = "macos"))]
const SCREENSHOT_SETTLE: Duration = Duration::from_millis(1500);

/// The window's content in physical pixels and the display's scale factor,
/// which `capture::FrameCapture` checks a capture of the window against.
fn content_size(window: &gpui::Window) -> capture::Content {
    let scale = f64::from(window.scale_factor());
    let content = window.viewport_size();
    (
        (
            (f64::from(content.width) * scale).round() as i64,
            (f64::from(content.height) * scale).round() as i64,
        ),
        scale,
    )
}

/// `--screenshot`: captures `window` with its frame to `path` once it has
/// settled (`capture::FrameCapture`), then records it in `outputs`
/// (`output_written`). A capture that fails, or fails the frame check, is
/// reported and exits with 1 (`cli::refuse`). `main` refuses the flag
/// before the window opens where the frame cannot be captured
/// (`cli::screenshot_support`).
fn screenshot(cx: &mut App, window: AnyWindowHandle, path: String, outputs: RunOutputs) {
    cx.spawn(async move |cx| {
        // The Retina nudge, again once the window has been laid out: calls
        // `-[NSWindow setContentSize:]` directly (synchronous) rather than
        // gpui's `window.resize()`, which is async and may not run before
        // the capture.
        #[cfg(target_os = "macos")]
        {
            capture::nudge_content_size(-1.0, 0.0);
            cx.background_executor().timer(SCREENSHOT_NUDGE).await;
            capture::nudge_content_size(1.0, 0.0);
        }
        cx.background_executor().timer(SCREENSHOT_SETTLE).await;
        let failed = |error: &str| Output::Screenshot.failure(&path, error);
        // The macOS runner's check: the menu bar AppKit holds is the
        // showcase's (`chrome::macos_menus`), by its titles in order, or the
        // capture fails. On the main thread, where this task runs.
        #[cfg(target_os = "macos")]
        if let Err(error) = native_theme_showcase_support::menu::check_main_menu(
            crate::chrome::APP_NAME,
            native_theme_showcase_support::menu::main_menu_titles(),
        ) {
            cli::refuse(&failed(&error));
        }
        let mut capture = match capture::FrameCapture::start(WINDOW_TITLE, &path) {
            Ok(capture) => capture,
            Err(error) => cli::refuse(&failed(&error)),
        };
        loop {
            let polled = cx.update_window(window, |_view, window, _cx| {
                capture.poll(|| Ok(content_size(window)))
            });
            match polled {
                Ok(None) => cx.background_executor().timer(SCREENSHOT_POLL).await,
                Ok(Some(Ok(()))) => break,
                Ok(Some(Err(error))) => cli::refuse(&failed(&error)),
                Err(error) => cli::refuse(&failed(&error.to_string())),
            }
        }
        eprintln!("Screenshot saved to {path}");
        cx.update(|cx| output_written(&outputs, Output::Screenshot, cx));
    })
    .detach();
}

/// Put the showcase `main` opened into the state the command line asks for:
/// its colour mode, theme, page, icons and open menu.
///
/// Each flag takes exactly what its Select offers, through the shared
/// crate's one validator per flag (`cli`): `--variant` a row of the
/// colour-mode Select, `--theme` one of the preset switch, `--icon-set` one
/// of the icon-theme Select. A value it does not offer is reported on stderr
/// and ignored (`cli::reported`), so the setting stays what it would have
/// been without the flag: a rejected `--theme` leaves `--variant` to install
/// the theme `Showcase::new` installed, in its mode.
///
/// `--variant` installs the theme in the mode it names: the one `--theme`
/// names, or, without `--theme`, the one `Showcase::new` installed. A theme
/// that fails to load leaves the one installed, in the mode it was drawn in
/// and with the colour-mode Select showing that mode, as the colour-mode
/// Select's own switch does (`Showcase::install_in_mode`).
fn apply_cli_args(
    s: &mut Showcase,
    cli_args: &CliArgs,
    window: &mut gpui::Window,
    cx: &mut gpui::Context<Showcase>,
) {
    s.pointer_shield = cli_args.shields_pointer();
    s.keyboard_shield = cli_args
        .shields_keyboard()
        .then(|| cx.intercept_keystrokes(|_event, _window, cx| cx.stop_propagation()));
    let mode = cli_args
        .variant
        .as_deref()
        .and_then(|variant| reported(cli::color_mode(variant)));
    let theme = cli_args
        .theme
        .as_deref()
        .and_then(|name| reported(cli::theme_key(name)))
        .map(|key| match key {
            ThemeKey::Default => "default".to_string(),
            ThemeKey::Preset(preset) => preset,
        });
    match (theme, mode) {
        (theme, Some(mode)) => {
            let theme = theme.unwrap_or_else(|| s.current_theme_name.clone());
            s.install_in_mode(&theme, mode, window, cx);
        }
        (Some(theme), None) => {
            s.apply_theme_by_name(&theme, window, cx);
        }
        (None, None) => {}
    }

    if let Some(page) = cli_args
        .tab
        .as_deref()
        .and_then(|name| reported(Page::named(name)))
    {
        s.show_page(page, cx);
    }

    // `--icon-set` names a row of the icon-theme Select: `default`, which
    // follows the theme from now on, or a set, which stays chosen across
    // theme switches as a pick in the Select does.
    if let Some(pick) = cli_args.icon_set.as_deref().and_then(|name| {
        reported(cli::icon_set_choice(
            name,
            &s.installed_themes,
            &[GPUI_BUILTIN_FLAG],
        ))
    }) {
        match pick {
            IconPick::FollowTheme => s.icon_choice_follows_preset = true,
            IconPick::Choice(choice) => s.choose_icon_row(&choice.to_string()),
            IconPick::Toolkit(_) => s.choose_icon_row(app::GPUI_BUILTIN_ROW),
        }
        s.show_icon_choice(window, cx);
        s.reload_icons(window, cx);
    }

    // On macOS the menus are the system menu bar's, which the window does
    // not draw (`chrome::window_top`, `demo::title_bar`).
    if let Some(name) = cli_args.open_menu.as_deref() {
        let opened = s.menus.update(cx, |menus, cx| {
            menus.open_named(name, cfg!(target_os = "macos"), window, cx)
        });
        reported(opened);
    }
}

/// The name `--icon-set` takes for gpui-component's own icons, the
/// icon-theme Select's `app::GPUI_BUILTIN_ROW`, the row only this
/// showcase's Select has.
const GPUI_BUILTIN_FLAG: &str = "gpui-builtin";

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // The second run `--screenshot` starts on Windows to find the window
    // (`capture::run_finder`), before any window opens.
    if let Some(code) = capture::run_finder(&args, WINDOW_TITLE) {
        std::process::exit(code);
    }
    let cli_args = CliArgs::parse(args);
    if let Some(value) = &cli_args.bad_pointer {
        cli::refuse(&format!(
            "--pointer {value:?}: not X,Y in whole logical pixels"
        ));
    }
    if cli_args.screenshot.is_some()
        && let Err(error) = cli::screenshot_support(Kind::Gpui)
    {
        cli::refuse(&error);
    }

    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx: &mut App| {
            gpui_kit::init(cx);
            app::init(cx);

            // Centred on whole pixels: `Bounds::centered` centres on the display's visible
            // bounds (gpui-pre 0.3.9 `src/geometry.rs:740-749`), which exclude the macOS menu
            // bar, so the origin can fall on a half pixel, and AppKit then grows the window to
            // whole pixels -- the 720 px content came out 721 px on the macOS runner
            // (screenshots run 36352884582).
            let mut bounds = Bounds::centered(None, WINDOW_SIZE, cx);
            bounds.origin = gpui::point(bounds.origin.x.round(), bounds.origin.y.round());
            let options = if cli_args.capturing() {
                capture_window_options(bounds)
            } else {
                window_options(bounds)
            };
            // gpui-kit's one window entry point: the showcase is the content
            // of a gpui-base `Root` (gpui-kit lib.rs, `open_window`), whose
            // plugins host the overlays and the showcase's actions (host.rs).
            let opened = gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| {
                    let mut s = Showcase::new(window, cx);
                    apply_cli_args(&mut s, &cli_args, window, cx);
                    s
                })
            });
            let (window_handle, showcase_entity) = match opened {
                Ok(opened) => opened,
                Err(error) => cli::refuse(&format!("the main window did not open: {error}")),
            };
            window_handle
                .update(cx, |_, window, _| name_window(window))
                .ok();
            let outputs = Rc::new(Cell::new(Outputs::new(&cli_args)));
            if let Some(path) = &cli_args.dump_layout {
                dump_layout(
                    cx,
                    window_handle,
                    showcase_entity.clone(),
                    path.clone(),
                    outputs.clone(),
                );
            }
            if let Some(at) = cli_args.pointer {
                hold_pointer(cx, window_handle, at, cli_args.press);
            }

            // Force Metal drawable to adopt the Retina scale factor by
            // nudging the content size synchronously.  Must happen outside
            // update() to avoid deadlocking the window-state mutex.
            #[cfg(target_os = "macos")]
            {
                capture::nudge_content_size(-1.0, 0.0);
                capture::nudge_content_size(1.0, 0.0);
            }
            cx.activate(true);

            if let Some(path) = &cli_args.screenshot {
                // Whether gpui's font names hold the stated monospace family,
                // "SF Mono" under macos-monterey: UNVERIFIED (egui spec §15),
                // and the family gpui draws it in, as the Theme tab and Widget
                // Info name it (`inspector::DrawnFamilies`). Printed for the
                // capture's log, never asserted (M9-167). In the
                // application because a `#[gpui::test]` cannot ask:
                // `TestAppContext::build` gives gpui's `NoopTextSystem`
                // (gpui-pre 0.3.9 src/app/test_context.rs:136,
                // src/platform/test/platform.rs:127-134).
                #[cfg(target_os = "macos")]
                {
                    let mono = showcase_entity.read(cx).original_mono_font.family.clone();
                    let names = cx.text_system().all_font_names();
                    let held = names.iter().any(|name| name.as_str() == mono.as_ref());
                    let asked = gpui_component::ActiveTheme::theme(cx).mono_font_family.clone();
                    let drawn = crate::inspector::DrawnFamilies::of([asked.clone()], cx)
                        .label(&mono, &asked);
                    println!(
                        "font-names: stated mono family {mono:?} in gpui's font names: {held}; drawn: {drawn}"
                    );
                }
                screenshot(cx, window_handle, path.clone(), outputs);
            }
        });
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn main() {
    eprintln!("gpui showcase is not supported on this platform");
}

#[cfg(test)]
mod tests;
