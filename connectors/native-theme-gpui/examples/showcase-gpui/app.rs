//! The showcase's state, theme switching, and the view that renders it.

use gpui::{
    Action, App, Context, Decorations, Entity, FocusHandle, Hsla, ImageSource, IntoElement,
    KeyBinding, ParentElement, Pixels, Render, SharedString, Styled, Subscription, Task, Window,
    actions, div, prelude::*,
};
use gpui_component::{
    ActiveTheme, IconName, IconNamed as _, ResizableState,
    attachment::AttachmentStatus,
    carousel::CarouselState,
    color_picker::ColorPickerState,
    combobox::{ComboboxEvent, ComboboxState},
    command::CommandState,
    diff::{DiffFile, DiffState},
    h_flex, h_resizable,
    input::{
        EditorState, InlineToken, InputState, NumberInputEvent, OtpState, StepAction, TextareaState,
    },
    list::ListState,
    message_scroller::MessageScrollerState,
    resizable_panel,
    select::{SearchableVec, SelectEvent, SelectState},
    slider::{SliderEvent, SliderState},
    speech::SpeechState,
    table::{Column, TableEvent, TableState},
    theme::Theme,
    time_field::TimeFieldState,
    tree::{TreeItem, TreeState},
    v_flex,
};
use std::borrow::Cow;
use std::cell::Cell;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Duration;

use native_theme::detect::{system_icon_set, system_icon_theme};
use native_theme::icons::{
    AnimatedIcon, FreedesktopLoader, IconData, IconRole, IconSetChoice, TransformAnimation,
    default_icon_choice, list_freedesktop_themes, load_icon_indicator,
};
use native_theme::pipeline::platform_preset_name;
use native_theme::theme::IconSet;
use native_theme_gpui::icons::{animated_frames_to_image_sources_for, to_image_source};
use native_theme_gpui::to_theme;
use native_theme_gpui::{AccessibilityPreferences, ActiveNativeTheme as _, geometry};
use native_theme_showcase_support::rows;

use crate::chrome;
use crate::elements;
use crate::info::InfoRegistry;
use crate::inspector::{Inspector, PanelLook};
use crate::speech::{SilentRecognizer, SyntheticAudio};
use crate::support::{
    CAROUSEL_SLIDES, ChatMessage, ChromeIcon, DIFF_SAMPLE, DialogIcons, EDITOR_SAMPLE, IconEntry,
    IconSource, PresetDelegate, SampleIcon, SampleListDelegate, SampleTableDelegate,
    SharedDialogIcons, initial_chat_messages, load_all_icons, load_gpui_icons,
    parse_icon_set_choice, release_sources,
};
use crate::{
    CHROME_HANDLE, CONTENT_ALERT, CONTENT_PANEL, CONTENT_SCROLL, LEFT_PANEL_WIDTH, PAGE_ROOT,
    POINTER_SHIELD, Page, demo,
};

/// gpui-component's mode for the showcase's light/dark flag.
fn gpui_theme_mode(is_dark: bool) -> gpui_component::theme::ThemeMode {
    if is_dark {
        gpui_component::theme::ThemeMode::Dark
    } else {
        gpui_component::theme::ThemeMode::Light
    }
}

/// native-theme's colour mode for the showcase's light/dark flag.
pub(crate) fn color_mode_of(is_dark: bool) -> native_theme_gpui::ColorMode {
    if is_dark {
        native_theme_gpui::ColorMode::Dark
    } else {
        native_theme_gpui::ColorMode::Light
    }
}

/// The font the status bar and the inspector name where the OS theme could
/// not be read: no source stated a family, a size or a colour.
fn unread_font() -> native_theme::theme::ResolvedFontSpec {
    native_theme::theme::ResolvedFontSpec {
        family: "(default)".into(),
        size: 0.0,
        defined_size: None,
        weight: 400,
        style: native_theme::theme::FontStyle::Normal,
        color: native_theme::color::Rgba::TRANSPARENT,
    }
}

// ---------------------------------------------------------------------------
// Actions (spec §2.2)
// ---------------------------------------------------------------------------
//
// One action backs a menu item, its key binding, a toolbar button or a
// status-bar toggle, and a command-palette entry, where each has one.

actions!(
    showcase,
    [
        Quit,
        ToggleSidePanel,
        OpenCommandPalette,
        ReloadTheme,
        OpenPreferences,
        OpenAbout
    ]
);

/// Show the page at this position of [`Page::ALL`].
///
/// `no_json`: gpui builds an action from JSON only for a keymap file, which
/// the showcase does not read; its menus and key bindings hold the value
/// itself (gpui-pre action.rs, `Action`).
#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = showcase, no_json)]
pub(crate) struct ShowPage(pub usize);

/// Install a colour mode, as the theme settings' colour-mode Select does.
#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = showcase, no_json)]
pub(crate) struct SetColorMode(pub AppColorMode);

/// Install the preset of this key, as the theme settings' preset switch
/// does, and show it chosen there.
#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = showcase, no_json)]
pub(crate) struct SetPreset(pub SharedString);

/// Bind the showcase's keys (spec §2.2) and quit on [`Quit`].
///
/// The bindings take gpui's `secondary` modifier, Cmd on macOS and Ctrl
/// elsewhere (gpui-pre keystroke.rs:143-150), as gpui-component's own do,
/// so the menus show each platform's own shortcut.
///
/// `Quit` is handled here, for the whole application, so it quits whatever
/// has the focus; the actions that change the showcase are handled around
/// the whole window, view and overlays, by `host::ShowcaseHost`, which this
/// registers before any window opens.
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("secondary-q", Quit, None),
        KeyBinding::new("secondary-b", ToggleSidePanel, None),
        KeyBinding::new("secondary-k", OpenCommandPalette, None),
        KeyBinding::new("secondary-,", OpenPreferences, None),
    ]);
    cx.on_action(|_: &Quit, cx| quit(cx));
    crate::host::register(cx);
}

/// Quit the showcase: take the focus off every field, let the windows paint
/// that, close them, then quit the application.
///
/// A focused text field hands the platform window, at paint, an input handler
/// holding its state (gpui-base's `InputBaseState`), and the platform window
/// keeps the last frame's handler. `App::quit`, or closing the window in the
/// same tick, leaves that handle to outlive the application, which gpui's
/// leak detector (on in the showcase's build: the dev-dependency's
/// `test-support` enables `leak-detection`) reports by panicking — as the
/// Basic page's focused field did on the Windows screenshot runner, whose
/// quit drops the application. Blurring, then letting a frame paint with no
/// focused field, replaces the handler before the windows go; each step
/// waits [`QUIT_SETTLE`].
pub(crate) fn quit(cx: &mut App) {
    for window in cx.windows() {
        let _ = window.update(cx, |_, window, cx| {
            window.blur(cx);
            window.refresh();
        });
    }
    cx.spawn(async move |cx| {
        cx.background_executor().timer(QUIT_SETTLE).await;
        cx.update(|cx| {
            for window in cx.windows() {
                let _ = window.update(cx, |_, window, _| window.remove_window());
            }
        });
        cx.background_executor().timer(QUIT_SETTLE).await;
        cx.update(|cx| cx.quit());
    })
    .detach();
}

/// How long [`quit`] waits between its steps: for the windows to paint a frame
/// with no focused field, then for the platform to take the closed windows
/// down. Six frames at 60 Hz; not a platform value.
const QUIT_SETTLE: std::time::Duration = std::time::Duration::from_millis(100);

// ---------------------------------------------------------------------------
// Color mode (light / dark / system)
// ---------------------------------------------------------------------------

/// The colour-mode Select's rows, the Theme menu's and the command
/// palette's modes: the shared crate's, which `--variant` names
/// (`cli::color_mode`).
pub(crate) use native_theme_showcase_support::cli::ColorModeChoice as AppColorMode;

// ---------------------------------------------------------------------------
// Main view
// ---------------------------------------------------------------------------

pub(crate) struct Showcase {
    /// The theme settings' preset switch (spec §3.3).
    pub(crate) preset_combobox: Entity<ComboboxState<PresetDelegate>>,
    /// The key of the installed theme: `default`, the desktop's own, or a
    /// preset's. Set once a theme installs, so a theme that fails to load
    /// leaves the one still installed named.
    pub(crate) current_theme_name: String,
    /// The platform preset the "default" theme is built on, which the preset
    /// switch's `default` row and the status bar both name
    /// (`Showcase::default_label`): `SystemTheme::preset`, the preset the
    /// pipeline settled on, or `platform_preset_name` where the OS theme
    /// could not be read.
    pub(crate) default_preset: String,
    /// The desktop's own theme as `default` last installed it, both variants
    /// and the OS's preferences as read: a colour-mode switch under
    /// `default` installs its other variant without reading the OS again,
    /// which only a reload (Reload System Theme, the theme watcher) does.
    pub(crate) system: Option<native_theme::SystemTheme>,
    pub(crate) is_dark: bool,
    pub(crate) color_mode: AppColorMode,
    /// Original native-theme font spec, for display purposes.
    pub(crate) original_font: native_theme::theme::ResolvedFontSpec,
    /// Original native-theme mono font spec, for display purposes.
    pub(crate) original_mono_font: native_theme::theme::ResolvedFontSpec,

    pub(crate) active_page: Page,
    /// Whether the side panel -- the theme settings and the inspector -- is
    /// shown (spec S4).
    pub(crate) side_panel_visible: bool,
    /// The window's resizable group's state, owned here so a change of its
    /// arrangement can start it afresh (`Showcase::rearrange`).
    body_layout: Entity<ResizableState>,
    /// The side panel's width, as dragged, which it takes again when it
    /// leaves the group and comes back.
    side_panel_width: Pixels,

    /// Where the showcase's widgets report their info (spec §4).
    pub(crate) info_ui: Entity<InfoRegistry>,
    /// The application's menus as the showcase draws them, with their
    /// keyboard (`demo::MenuBar`).
    pub(crate) menus: Entity<demo::MenuBar>,
    /// Where the content panel's page is scrolled (`support::gutter_scroll`).
    pub(crate) content_scroll: gpui::ScrollHandle,
    /// The inspector, in the side panel, which shows the info the registry
    /// settles on.
    pub(crate) inspector: Entity<Inspector>,
    /// What the inspector's Widget tab is drawn in and reads its rows' values
    /// from under the installed native theme (`PanelLook`), built once an
    /// install; `None` where none is installed, and from an install until
    /// the next frame builds it. Every install --
    /// `install_system`, `install_preset`, `demo::change_preferences`, the
    /// connector's own re-apply after a mode switch upstream -- writes the
    /// connector's `NativeTheme`, whose observer (`_native_theme`) drops it.
    pub(crate) panel_look: Option<PanelLook>,
    /// Drops `panel_look` whenever the connector's `NativeTheme` is written.
    _native_theme: Subscription,
    /// The title the status bar's hover label showed in the last frame;
    /// `None` where it showed none.
    pub(crate) status_title_drawn: Option<SharedString>,
    /// The layout of the status bar's environment text in the last frame:
    /// the text it drew, cut short with an ellipsis where the bar left it
    /// too little room (`TextLayout::text`).
    #[cfg(test)]
    pub(crate) status_environment_drawn: gpui::TextLayout,
    /// The decorations the self-tests say the window was granted. gpui's test
    /// platform grants server-side decorations whatever it is asked for (gpui-pre
    /// platform.rs, `PlatformWindow::window_decorations`), so this is how a test
    /// reaches the client-side arm of `Showcase::frame`.
    #[cfg(test)]
    pub(crate) frame_for_test: Option<Decorations>,
    /// The accessibility preferences the self-tests say the OS has, which a
    /// preset install then reads in place of the host's
    /// (`Showcase::os_preferences`): a test cannot change the desktop's.
    /// `install_system` takes the desktop's theme, preferences and
    /// all, from its caller, so it needs none.
    #[cfg(test)]
    pub(crate) os_preferences_for_test: Option<AccessibilityPreferences>,
    /// The system icon theme's detection as the self-tests say it went,
    /// which the Icons page's label then shows in place of the host's
    /// (`Showcase::detected_icon_theme`): a test cannot change the desktop's.
    #[cfg(test)]
    pub(crate) icon_theme_detection_for_test: Option<fn() -> native_theme::Result<String>>,
    /// The command palette's query and highlighted row (spec §2.8).
    pub(crate) palette_state: Entity<CommandState>,
    /// The view's focus, so an action dispatched with nothing else focused
    /// still reaches the handlers `render` puts on the view.
    focus_handle: FocusHandle,
    /// Takes the focus back to the view when the focused element stops being
    /// drawn -- a page's widget whose page was left keeps its handle, and
    /// gpui would dispatch from the window's root instead (gpui-pre
    /// window.rs:6463-6471), out of the view's handlers' reach.
    _refocus: Subscription,

    /// `geometry::widget_gap` of `layout`, set as each frame starts, for the
    /// overlays: gpui-component's `WindowState` root plugin builds them anew
    /// for every frame from builders that cannot borrow the view (root.rs,
    /// `WindowState::dialog_layer`), so this
    /// is how the gap they read is the installed theme's, not the one they
    /// opened under.
    pub(crate) overlay_gap: Rc<Cell<Option<Pixels>>>,

    /// The icons of the Overlays page's dialogs, of the chosen icon theme,
    /// set as each frame starts for the reason `overlay_gap` is.
    pub(crate) dialog_icons: SharedDialogIcons,

    /// Layout spacing of the installed theme. It lives on the model, not on
    /// `ResolvedTheme`, so the geometry accessors take it from here rather
    /// than from `cx.native_theme()`.
    pub(crate) layout: native_theme::theme::LayoutTheme,

    /// Whether a box over the whole window takes the pointer, so what is
    /// drawn does not depend on where the pointer happens to be: set for a
    /// capture (`--capture`, `--screenshot`) and a layout dump
    /// (`--dump-layout`) that hold no pointer of their own (`--pointer`).
    pub(crate) pointer_shield: bool,

    // Basic page
    /// The Basic page's three text fields: empty with a placeholder, filled,
    /// and disabled.
    pub(crate) basic_hint_state: Entity<InputState>,
    pub(crate) basic_text_state: Entity<InputState>,
    pub(crate) basic_disabled_state: Entity<InputState>,
    /// The Basic page's drop-down, Apple chosen.
    pub(crate) basic_select: Entity<SelectState<SearchableVec<SharedString>>>,
    pub(crate) basic_slider_state: Entity<SliderState>,
    pub(crate) basic_radio: Option<usize>,
    /// The Basic page's text area: three rows, three lines of text.
    pub(crate) basic_textarea: Entity<TextareaState>,
    /// The Basic page's List: eight rows, the second selected.
    pub(crate) basic_list: Entity<ListState<SampleListDelegate>>,
    /// The selected tab of the Basic page's tab bar.
    pub(crate) basic_tab: usize,
    /// The selected segment of the Basic page's segmented control.
    pub(crate) basic_segment: usize,
    /// Whether each of the Basic page's two expanders is open.
    pub(crate) basic_expanded: [bool; 2],
    /// The Basic page's number field, at `BASIC_NUMBER`, stepping by one.
    pub(crate) basic_number_state: Entity<InputState>,
    /// The Basic page's text field that holds the keyboard focus at start.
    pub(crate) basic_focused_state: Entity<InputState>,

    // Inputs page
    pub(crate) input_state: Entity<InputState>,
    /// The field sized by `geometry::input_height` alone.
    pub(crate) input_height_state: Entity<InputState>,
    /// The multi-line field of the Textarea demo, beside the single-line
    /// `Input` it shares a surface with.
    pub(crate) textarea_demo: Entity<TextareaState>,
    /// The Inputs page's `Select`: the same trigger as a Combobox, with the
    /// platform's font colour carried as well.
    pub(crate) select_demo: Entity<SelectState<SearchableVec<SharedString>>>,
    pub(crate) input_group_state: Entity<InputState>,
    pub(crate) input_group_button_state: Entity<InputState>,
    pub(crate) input_group_textarea_state: Entity<TextareaState>,
    pub(crate) number_input_state: Entity<InputState>,
    /// The Form section's two fields. Held here, not built in `render`: a
    /// state created per frame loses whatever was typed into it.
    pub(crate) form_name_state: Entity<InputState>,
    pub(crate) form_email_state: Entity<InputState>,
    pub(crate) slider_state: Entity<SliderState>,
    /// The Inputs page's range slider, and its vertical one.
    pub(crate) range_slider_state: Entity<SliderState>,
    pub(crate) vertical_slider_state: Entity<SliderState>,
    pub(crate) otp_state: Entity<OtpState>,
    pub(crate) color_picker_state: Entity<ColorPickerState>,
    /// The Inputs page's ColorSelect, empty until a colour is picked.
    pub(crate) color_select_state: Entity<ColorPickerState>,
    /// The Inputs page's SpeechWaveform: a synthetic input and a recognizer
    /// that hears nothing. It runs until Stop, whichever page is shown.
    pub(crate) speech_state: Entity<SpeechState>,
    pub(crate) date_picker_state: Entity<gpui_component::date_picker::DatePickerState>,
    /// The Inputs page's TimeField (spec §8.3).
    pub(crate) time_field_state: Entity<TimeFieldState>,
    /// The Inputs page's Input holding one inline token (spec §8.3).
    pub(crate) token_input_state: Entity<InputState>,
    pub(crate) calendar_state: Entity<gpui_component::calendar::CalendarState>,
    pub(crate) checkbox_a: bool,
    pub(crate) checkbox_b: bool,
    pub(crate) checkbox_c: bool,
    pub(crate) switch_on: bool,
    pub(crate) radio_index: Option<usize>,
    pub(crate) slider_value: f32,
    /// Stars the `Rating` currently shows; its `on_click` writes here.
    pub(crate) rating_value: usize,

    // Layout page
    pub(crate) collapsible_open: bool,
    pub(crate) carousel_state: Entity<CarouselState>,
    /// The `Stepper`'s current step, written by its `on_click`.
    pub(crate) step: usize,

    // Typography page
    pub(crate) editor_state: Entity<EditorState>,
    /// The Typography page's `Diff`, over `DIFF_SAMPLE`.
    pub(crate) diff_state: Entity<DiffState>,

    // Data page
    pub(crate) table_state: Entity<TableState<SampleTableDelegate>>,
    pub(crate) list_state: Entity<ListState<SampleListDelegate>>,
    pub(crate) tree_state: Entity<TreeState>,
    /// The page the `Pagination` is on, written by its `on_click`.
    pub(crate) page: usize,
    /// The chat thread the `MessageScroller` renders. The data stays with the
    /// caller; the state below owns only the virtual list's bookkeeping.
    pub(crate) chat_messages: Vec<ChatMessage>,
    pub(crate) chat_scroller: Entity<MessageScrollerState>,
    /// The status the third `Attachment` card is in; clicking it advances.
    pub(crate) attachment_status: AttachmentStatus,

    // Buttons page
    pub(crate) toggle_bold: bool,
    pub(crate) toggle_italic: bool,

    // Overlays page
    /// What the last `AlertDialog` was answered with, written by its `on_ok`
    /// and `on_cancel` so the section reports a real outcome.
    pub(crate) alert_choice: Option<SharedString>,

    /// The theme settings' colour-mode Select (spec §3.3): a row per
    /// `AppColorMode::ALL`, reading its `short_label`.
    pub(crate) color_mode_select: Entity<SelectState<SearchableVec<SharedString>>>,

    /// The theme settings' icon-theme Select, its rows rebuilt on every
    /// theme install (`Showcase::show_icon_choice`).
    pub(crate) icon_theme_select: Entity<SelectState<SearchableVec<SharedString>>>,
    /// The chosen icon theme's name as the Icons page gives it: its
    /// `IconSet`'s name, or "gpui-builtin" for gpui-component's own icons.
    pub(crate) icon_theme_name: String,
    /// Parsed `IconSet` for the current selection (`None` for "gpui-builtin").
    pub(crate) icon_set_enum: Option<IconSet>,
    pub(crate) loaded_icons: Vec<(IconRole, Option<IconData>, IconSource)>,
    pub(crate) gpui_icons: Vec<IconEntry>,
    /// Cached ImageSource per native icon (same indexing as loaded_icons).
    pub(crate) loaded_icon_sources: Vec<Option<ImageSource>>,
    /// Cached ImageSource per gpui icon (same indexing as gpui_icons).
    pub(crate) gpui_icon_sources: Vec<Option<ImageSource>>,
    /// Foreground color used when building the image source caches.
    pub(crate) icon_cache_fg: Hsla,
    /// What the loaded icons were loaded as; `None` before the first load
    /// and, for a system set's, after a reload of the desktop's settings, so
    /// the next loads.
    pub(crate) icon_load: Option<IconLoad>,
    /// The icon-theme choice (library type).
    pub(crate) icon_set_choice: IconSetChoice,
    /// Whether the choice follows the preset: it is `default_icon_choice`'s
    /// for the installed theme -- its `default`, or `system` where the
    /// preset's own icon theme is not installed -- and is taken again on
    /// every theme install. False once the user picks any other row of the
    /// icon-theme Select, or `--icon-set` names a set: that choice stays;
    /// true again after `--icon-set default`.
    pub(crate) icon_choice_follows_preset: bool,
    /// Cached list of installed freedesktop icon themes (populated once at init).
    pub(crate) installed_themes: Vec<String>,
    /// The current resolved theme's preferred icon theme (e.g. "breeze", "Lucide");
    /// `None` where the theme names none and the system's cannot be detected.
    pub(crate) current_icon_theme: Option<String>,
    /// The current resolved theme's icon set (loading mechanism).
    pub(crate) current_icon_set: IconSet,
    /// Whether the current theme's TOML specified `icon_theme` (before resolution).
    pub(crate) has_toml_icon_theme: bool,

    // Animated Icons state
    /// Cached frame ImageSources for frame-based animations (set name, frames).
    pub(crate) animated_frame_sources: Vec<(String, Vec<ImageSource>)>,
    /// Frame duration in ms for each frame-based animation (parallel to animated_frame_sources).
    pub(crate) animated_frame_durations: Vec<u32>,
    /// Current frame index for each frame-based animation.
    pub(crate) animated_frame_indices: Vec<usize>,
    /// The SVG of each transform-based (spin) animation, which the page
    /// turns with `with_spin_animation` (set name, SVG bytes, duration_ms).
    pub(crate) animated_spin_sources: Vec<(String, Vec<u8>, NonZeroU32)>,
    /// Timer task handle for frame cycling (dropped to cancel).
    pub(crate) animation_timer: Option<Task<()>>,

    /// Why the last theme failed to load, which an Alert at the top of the
    /// content reports (spec §2.5); `None` once a theme loads.
    pub(crate) error_message: Option<String>,

    /// The OS theme watcher (`native_theme_gpui::on_theme_change`), which
    /// stops when this is dropped.
    #[cfg(feature = "watch")]
    pub(crate) _theme_watcher: Option<native_theme_gpui::ThemeSubscription>,
}

/// The icon-theme Select's row for gpui-component's own icons, which no
/// `IconSetChoice` names.
pub(crate) const GPUI_BUILTIN_ROW: &str = "gpui-component built-in (Lucide)";

/// The Basic page's slider value, on 0 to 100: the datum on display, as the
/// page's `BASIC_PROGRESS` is.
const BASIC_SLIDER: f32 = 40.0;

/// The Basic page's number field: the value it opens at and its step, the
/// iced and egui showcases' too.
const BASIC_NUMBER: i64 = 42;
const BASIC_NUMBER_STEP: i64 = 1;

/// The rows the Basic page's text area is tall, and the text it holds: one
/// line per row.
const BASIC_TEXTAREA_ROWS: usize = 3;
const BASIC_TEXTAREA_TEXT: &str = "Line one\nLine two\nLine three";

/// The Basic page's List: its rows, `Item 1` to `Item 8`, and the one
/// selected, `Item 2`.
const BASIC_LIST_ROWS: usize = 8;
const BASIC_LIST_SELECTED: usize = 1;

/// The colour the system icon sets' loaders recolour a symbolic icon in
/// (`load_all_icons`, `load_gpui_icons`): the installed native theme's
/// `defaults.font.color`, and none without a native theme, so an icon keeps
/// its own colours. The placeholder font shown where the OS theme could not
/// be read is transparent black (`unread_font`), which the loaders would
/// take as black, as they keep only its red, green and blue.
pub(crate) fn icon_tint(
    native_installed: bool,
    font_colour: native_theme::Rgba,
) -> Option<native_theme::Rgba> {
    native_installed.then_some(font_colour)
}

#[cfg(test)]
thread_local! {
    /// How often a tick of the animation timer (`start_animation_timer`)
    /// stepped the frames and asked for a redraw on this thread; a test reads
    /// how much an action adds to it.
    pub(crate) static ANIMATION_STEPS: Cell<usize> = const { Cell::new(0) };
}

/// What the Icons page's icons were loaded as (`Showcase::reload_icons`),
/// so a theme install loads them again only where this changed.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IconLoad {
    /// The set chosen; `None` for gpui-component's own icons.
    set: Option<IconSet>,
    /// The set the choice loads from.
    effective: IconSet,
    /// The freedesktop theme a freedesktop set loads from; `None` is the
    /// system's own.
    theme: Option<String>,
    /// The colour a system set's loaders recoloured a symbolic icon in
    /// (`icon_tint`); `None` for a bundled set, whose icons are coloured
    /// where they are drawn.
    tint: Option<native_theme::Rgba>,
}

/// The icon-theme Select's rows for a theme of `icon_set` naming
/// `icon_theme`: the shared crate's (`rows::icon_rows`) -- its `default` row,
/// where the theme names an icon theme that is available
/// (`default_icon_choice`), then `system`, the installed freedesktop themes
/// and the bundled sets -- with gpui-component's own icons after the
/// installed themes.
fn icon_theme_rows(
    icon_set: IconSet,
    icon_theme: Option<&str>,
    installed: &[String],
) -> Vec<SharedString> {
    rows::icon_rows(icon_set, icon_theme, installed)
        .into_iter()
        .flat_map(|choice| {
            let builtin = (choice == IconSetChoice::Lucide).then(|| GPUI_BUILTIN_ROW.into());
            builtin
                .into_iter()
                .chain([SharedString::from(choice.to_string())])
        })
        .collect()
}

impl Showcase {
    /// Rebuild cached `ImageSource` objects for all loaded icons.
    ///
    /// Called when icons are loaded or the theme foreground color changes,
    /// so that `render_icons_tab` can reuse the cached sources instead of
    /// re-creating `Image` + `Arc` allocations and re-colorizing SVGs on
    /// every frame.
    pub(crate) fn rebuild_icon_caches(&mut self, fg: Hsla, window: &mut Window, cx: &mut App) {
        release_sources(
            std::mem::take(&mut self.loaded_icon_sources)
                .into_iter()
                .chain(std::mem::take(&mut self.gpui_icon_sources))
                .flatten(),
            window,
            cx,
        );
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
    /// Called at init and whenever the icon theme changes so that animated icon
    /// rendering can use pre-built `ImageSource` objects without re-rasterizing
    /// SVGs on every frame tick.
    pub(crate) fn rebuild_animation_caches(&mut self, window: &mut Window, cx: &mut App) {
        release_sources(
            std::mem::take(&mut self.animated_frame_sources)
                .into_iter()
                .flat_map(|(_name, frames)| frames),
            window,
            cx,
        );
        self.animated_spin_sources.clear();
        self.animated_frame_durations.clear();

        // gpui-builtin is not a native-theme icon set; load_indicator would
        // fall back to the system set, showing the wrong spinner. A
        // freedesktop spinner comes from the theme the icons come from, not
        // the system's, so the page never mixes two themes.
        let anim = match self.icon_set_enum {
            Some(IconSet::Freedesktop) => {
                FreedesktopLoader::load_indicator(self.freedesktop_theme())
                    .map(|anim| (IconSet::Freedesktop, anim))
            }
            Some(icon_set) => load_icon_indicator(icon_set, None).map(|anim| (icon_set, anim)),
            None => None,
        };
        let set_name = &self.icon_theme_name;
        let fg = self.icon_cache_fg;
        if let Some((icon_set, anim)) = anim {
            match &anim {
                // Coloured as the set colours it: a freedesktop theme's
                // frames keep the colour their stylesheet sets, as the
                // connector's Spinner draws them.
                AnimatedIcon::Frames(data) => {
                    if let Some(anim_sources) =
                        animated_frames_to_image_sources_for(icon_set, &anim, Some(fg), None)
                    {
                        self.animated_frame_durations
                            .push(data.frame_duration_ms().get());
                        self.animated_frame_sources
                            .push((set_name.to_string(), anim_sources.sources));
                    }
                }
                // gpui turns an SVG element but not an image, so a spin is
                // kept as the SVG it is (gpui-pre elements/svg.rs, Svg).
                AnimatedIcon::Transform(data) => {
                    if let (IconData::Svg(bytes), TransformAnimation::Spin { duration_ms }) =
                        (data.icon(), data.animation())
                    {
                        self.animated_spin_sources.push((
                            set_name.to_string(),
                            bytes.to_vec(),
                            *duration_ms,
                        ));
                    }
                }
                _ => {}
            }
        }

        self.animated_frame_indices = vec![0; self.animated_frame_sources.len()];
    }

    /// Start (or restart) the frame-cycling timer for animated icons.
    ///
    /// Cancels any previous timer. Does nothing when there are no frame-based
    /// animations cached, or while a page other than Icons, the one page that
    /// draws the frames, is shown: `show_page` starts it as the Icons page
    /// opens and drops it as the page closes, the page showing a paused
    /// frame from where it stopped. A tick does nothing while gpui's
    /// `reduce_motion` is on: the page then shows each animation's first
    /// frame, as gpui holds its own animations at their start.
    pub(crate) fn start_animation_timer(&mut self, cx: &mut Context<Self>) {
        // Drop old timer (cancels the task)
        self.animation_timer = None;

        if self.animated_frame_sources.is_empty() || self.active_page != Page::Icons {
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
                    if cx.reduce_motion() {
                        return;
                    }
                    for (i, (_name, frames)) in this.animated_frame_sources.iter().enumerate() {
                        if let Some(idx) = this.animated_frame_indices.get_mut(i)
                            && let Some(next) = idx.saturating_add(1).checked_rem(frames.len())
                        {
                            *idx = next;
                        }
                    }
                    #[cfg(test)]
                    ANIMATION_STEPS.with(|steps| steps.set(steps.get().saturating_add(1)));
                    cx.notify();
                }) else {
                    break;
                };
            }
        });

        self.animation_timer = Some(task);
    }

    /// The freedesktop theme the page's icons load from where the set is
    /// freedesktop: the theme the choice names; `None` is the system's own.
    pub(crate) fn freedesktop_theme(&self) -> Option<&str> {
        self.icon_set_choice.freedesktop_theme()
    }

    /// The content panel's width: the page's scroll area's, as the last frame
    /// laid it out (the area spans the panel; the scrollbar is drawn over
    /// it). Before a first layout, the window's width less the side panel
    /// where it is shown -- its width, the strip it keeps clear for the
    /// splitter's line (`demo::splitter_reserve`) and the resize handle
    /// (`demo::HANDLE_SIZE`) -- as the body lays a newly opened window out.
    pub(crate) fn content_width(&self, window: &Window, cx: &App) -> Pixels {
        let laid = self.content_scroll.bounds().size.width;
        if laid > gpui::px(0.) {
            return laid;
        }
        let viewport = window.viewport_size().width;
        if !self.side_panel_visible {
            return viewport;
        }
        gpui::px(
            f32::from(viewport)
                - f32::from(self.side_panel_width)
                - f32::from(crate::demo::splitter_reserve(cx).unwrap_or_default())
                - f32::from(crate::demo::HANDLE_SIZE),
        )
    }

    /// The icon set the theme-drawn Spinner draws its indicator from: the
    /// page's own, with the freedesktop theme its icons load from.
    pub(crate) fn spinner_icons(&self) -> crate::demo::SpinnerIcons {
        crate::demo::SpinnerIcons {
            set: self.icon_set_enum,
            theme: self
                .freedesktop_theme()
                .map(|t| SharedString::from(t.to_owned())),
            indicator: !self.animated_frame_sources.is_empty()
                || !self.animated_spin_sources.is_empty(),
            spins: !self.animated_spin_sources.is_empty(),
        }
    }

    /// The icon theme as the Icons page names it: a freedesktop set with the
    /// theme its icons load from -- where that is the system's and it cannot
    /// be detected, why, as no theme's icons load then.
    pub(crate) fn icon_set_label(&self) -> String {
        match self.icon_set_enum {
            Some(IconSet::Freedesktop) => match self.freedesktop_theme() {
                Some(theme) => format!("freedesktop ({theme})"),
                None => match self.detected_icon_theme() {
                    Ok(theme) => format!("freedesktop ({theme})"),
                    Err(e) => format!("freedesktop (unavailable: {e})"),
                },
            },
            _ => self.icon_theme_name.clone(),
        }
    }

    /// The system icon theme, or why none was detected; in the self-tests,
    /// what `icon_theme_detection_for_test` gives, where it is set.
    fn detected_icon_theme(&self) -> native_theme::Result<String> {
        #[cfg(test)]
        if let Some(detect) = self.icon_theme_detection_for_test {
            return detect();
        }
        system_icon_theme()
    }

    /// The decorations the window was granted (spec S8), which decide who
    /// draws its frame: the window manager, or the application with `Root`'s
    /// client frame and a `TitleBar`.
    pub(crate) fn frame(&self, window: &Window) -> Decorations {
        #[cfg(test)]
        if let Some(frame) = self.frame_for_test {
            return frame;
        }
        window.window_decorations()
    }

    /// The chrome's icon for gpui-component's `icon`, from the chosen icon
    /// set: its own icons where the built-in set is chosen, never another
    /// set's.
    pub(crate) fn chrome_icon(&self, icon: &IconName) -> ChromeIcon {
        ChromeIcon::of(&self.gpui_icons, self.icon_set_enum.is_none(), icon)
    }

    /// Where the Basic page's fields laid their text out, as the elements of
    /// docs/showcase-elements.toml the text is: each value's box as the
    /// field's state reports it (`range_to_bounds`, gpui-base
    /// input/base/state.rs), one line tall; the text area's as the box round
    /// its lines. A field whose text is not laid out, or empty, reports
    /// nothing: the placeholder is drawn by the field's element, which
    /// reports no bounds of it.
    pub(crate) fn field_text_bounds(
        &self,
        cx: &gpui::App,
    ) -> Vec<(&'static str, gpui::Bounds<Pixels>)> {
        let line = |state: &Entity<InputState>| {
            let state = state.read(cx);
            let len = state.value().len();
            (len > 0)
                .then(|| state.range_to_bounds(&(0..len)))
                .flatten()
        };
        let fields = [
            ("basic.text_inputs.filled.text", &self.basic_text_state),
            (
                "basic.text_inputs.disabled.text",
                &self.basic_disabled_state,
            ),
            ("basic.text_inputs.focused.text", &self.basic_focused_state),
            ("basic.number_input.field.text", &self.basic_number_state),
        ];
        let mut found: Vec<(&'static str, gpui::Bounds<Pixels>)> = fields
            .into_iter()
            .filter_map(|(id, state)| line(state).map(|b| (id, b)))
            .collect();
        let area = self.basic_textarea.read(cx);
        let text = area.value();
        let mut start: usize = 0;
        let mut lines: Option<gpui::Bounds<Pixels>> = None;
        for piece in text.split('\n') {
            let end = start.saturating_add(piece.len());
            if let Some(b) = area.range_to_bounds(&(start..end)) {
                lines = Some(lines.map_or(b, |so_far| so_far.union(&b)));
            }
            start = end.saturating_add(1);
        }
        found.extend(lines.map(|b| ("basic.text_area.field.text", b)));
        found
    }

    /// The chrome's icon for native-theme's `role`, from the chosen icon set
    /// as the Icons page loaded it (`loaded_icons`): gpui-component's own
    /// icon for the role where its built-in set is chosen, with the IconName
    /// to draw; the set's SVG otherwise; `Missing` where the set has none
    /// for the role, or only pixels -- never another set's.
    pub(crate) fn role_chrome_icon(&self, role: IconRole) -> (ChromeIcon, Option<IconName>) {
        if self.icon_set_enum.is_none() {
            return match native_theme_gpui::icons::icon_name(role) {
                Some(icon) => (ChromeIcon::Builtin(icon.clone().path()), Some(icon)),
                None => (ChromeIcon::Missing(role.name()), None),
            };
        }
        let svg = self
            .loaded_icons
            .iter()
            .find(|(loaded, ..)| *loaded == role)
            .and_then(|(_, data, _)| match data {
                Some(IconData::Svg(bytes)) => Some(bytes.clone()),
                _ => None,
            });
        match svg {
            Some(bytes) => (ChromeIcon::Loaded(role.name(), bytes), None),
            None => (ChromeIcon::Missing(role.name()), None),
        }
    }

    /// A page sample's icon for gpui-component's `icon`: the chosen icon
    /// theme's, found as the chrome's are (`chrome_icon`), with the name the
    /// sample's info gives that theme.
    pub(crate) fn sample_icon(&self, icon: IconName) -> SampleIcon {
        SampleIcon {
            drawn: self.chrome_icon(&icon),
            icon,
            set: self.icon_set_label().into(),
        }
    }

    /// Load the icon theme the icon-theme Select names `display`, as the
    /// Select does when it is confirmed. The user's pick: only its `default`
    /// row follows the preset from now on.
    pub(crate) fn select_icon_set(
        &mut self,
        display: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.choose_icon_row(display);
        self.reload_icons(window, cx);
    }

    /// Choose the icon theme of the icon-theme Select's row `display`,
    /// without loading its icons: only its `default` row follows the preset
    /// from now on.
    pub(crate) fn choose_icon_row(&mut self, display: &str) {
        let is_gpui_builtin = display == GPUI_BUILTIN_ROW;
        self.icon_set_choice = parse_icon_set_choice(display);
        self.icon_choice_follows_preset = matches!(self.icon_set_choice, IconSetChoice::Default(_));
        let effective = self
            .icon_set_choice
            .effective_icon_set(self.current_icon_set);
        // The page tells gpui-component's own icons by this name, which
        // `effective` (Lucide, for the built-in entry) would not give.
        self.icon_theme_name = if is_gpui_builtin {
            "gpui-builtin".to_string()
        } else {
            effective.name().to_string()
        };
        // For gpui-builtin, icon_set_enum is None (uses gpui-component's
        // built-in icons rather than native-theme's loader).
        self.icon_set_enum = if is_gpui_builtin {
            None
        } else {
            Some(effective)
        };
    }

    /// Load the icons of the chosen set and rebuild what the Icons page
    /// draws from them, where what they are loaded as (`IconLoad`) changed
    /// since the last load: the set, its icon theme, or a system set's
    /// colour. Otherwise the loaded icons stay, and what is drawn from them
    /// is recoloured with the theme's foreground, as a bundled set's icons
    /// are coloured where they are drawn.
    pub(crate) fn reload_icons(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let effective = self
            .icon_set_choice
            .effective_icon_set(self.current_icon_set);
        let theme = self.freedesktop_theme().map(str::to_string);
        let fg_color = icon_tint(cx.native_theme().is_some(), self.original_font.color);
        let load = IconLoad {
            set: self.icon_set_enum,
            effective,
            theme,
            tint: fg_color.filter(|_| crate::support::is_system_set(effective)),
        };
        let fg = cx.theme().foreground;
        if self.icon_load.as_ref() == Some(&load) {
            if fg != self.icon_cache_fg {
                self.rebuild_icon_caches(fg, window, cx);
                self.rebuild_animation_caches(window, cx);
                self.start_animation_timer(cx);
                cx.notify();
            }
            return;
        }
        // gpui-builtin draws gpui-component's own icons: nothing to load.
        if self.icon_set_enum.is_some() {
            self.loaded_icons = load_all_icons(effective, load.theme.as_deref(), fg_color);
        }
        self.gpui_icons = load_gpui_icons(self.icon_set_enum, load.theme.as_deref(), fg_color);
        self.icon_load = Some(load);
        self.rebuild_icon_caches(fg, window, cx);
        self.rebuild_animation_caches(window, cx);
        self.start_animation_timer(cx);
        cx.notify();
    }

    /// The icon-theme Select's rows for the installed theme.
    pub(crate) fn icon_set_dropdown_names(&self) -> Vec<SharedString> {
        icon_theme_rows(
            self.current_icon_set,
            self.preset_icon_theme(),
            &self.installed_themes,
        )
    }

    /// The icon theme the installed theme names, where it names one.
    fn preset_icon_theme(&self) -> Option<&str> {
        self.current_icon_theme
            .as_deref()
            .filter(|_| self.has_toml_icon_theme)
    }

    /// The icon-theme Select's row for the icons drawn: the current
    /// choice's, or gpui-component's own icons' row.
    fn icon_choice_row(&self) -> SharedString {
        match self.icon_set_enum {
            None => GPUI_BUILTIN_ROW.into(),
            Some(_) => self.icon_set_choice.to_string().into(),
        }
    }

    /// Take the installed theme's icon choice where the choice follows the
    /// preset, then rebuild the icon-theme Select's rows for the installed
    /// theme -- its `default` row is the installed preset's, never an
    /// earlier one's -- and show the current choice chosen in it.
    pub(crate) fn show_icon_choice(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.icon_choice_follows_preset {
            self.icon_set_choice =
                default_icon_choice(self.current_icon_set, self.preset_icon_theme());
            let effective = self
                .icon_set_choice
                .effective_icon_set(self.current_icon_set);
            self.icon_theme_name = effective.name().to_string();
            self.icon_set_enum = Some(effective);
        }
        let rows = SearchableVec::new(self.icon_set_dropdown_names());
        let chosen = self.icon_choice_row();
        self.icon_theme_select.update(cx, |select, cx| {
            select.set_items(rows, window, cx);
            select.set_selected_value(&chosen, window, cx);
        });
    }

    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let color_mode = AppColorMode::System;
        let color_mode_select = cx.new(|cx| {
            SelectState::new(
                SearchableVec::new(
                    AppColorMode::ALL
                        .map(|m| SharedString::from(m.short_label()))
                        .to_vec(),
                ),
                AppColorMode::ALL
                    .iter()
                    .position(|&m| m == color_mode)
                    .map(|ix| gpui_component::IndexPath::default().row(ix)),
                window,
                cx,
            )
        });
        cx.subscribe_in(
            &color_mode_select,
            window,
            |_this: &mut Self,
             _entity,
             event: &SelectEvent<SearchableVec<SharedString>>,
             window,
             cx| {
                if let SelectEvent::Confirm(Some(value)) = event
                    && let Some(mode) = AppColorMode::ALL
                        .into_iter()
                        .find(|mode| mode.short_label() == value.as_ref())
                {
                    window.dispatch_action(Box::new(SetColorMode(mode)), cx);
                }
            },
        )
        .detach();

        let basic_hint_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("Placeholder", window, cx);
            state
        });
        let basic_text_state = cx.new(|cx| InputState::new(window, cx).default_value("Text"));
        let basic_disabled_state =
            cx.new(|cx| InputState::new(window, cx).default_value("Disabled"));
        let basic_select = cx.new(|cx| {
            SelectState::new(
                SearchableVec::new(vec![
                    SharedString::from("Apple"),
                    SharedString::from("Banana"),
                    SharedString::from("Cherry"),
                ]),
                Some(gpui_component::IndexPath::default().row(0)),
                window,
                cx,
            )
        });
        let basic_slider_state = cx.new(|_cx| SliderState::new().default_value(BASIC_SLIDER));
        let basic_textarea = cx.new(|cx| {
            TextareaState::new(window, cx)
                .auto_grow(BASIC_TEXTAREA_ROWS, BASIC_TEXTAREA_ROWS)
                .default_value(BASIC_TEXTAREA_TEXT)
        });
        let basic_number_state =
            cx.new(|cx| InputState::new(window, cx).default_value(BASIC_NUMBER.to_string()));
        cx.subscribe_in(
            &basic_number_state,
            window,
            |_this: &mut Self, input, event: &NumberInputEvent, window, cx| {
                let NumberInputEvent::Step(action) = event;
                input.update(cx, |input, cx| {
                    let value = input.value().parse().unwrap_or(BASIC_NUMBER);
                    let value = match action {
                        StepAction::Increment => value.saturating_add(BASIC_NUMBER_STEP),
                        StepAction::Decrement => value.saturating_sub(BASIC_NUMBER_STEP),
                    };
                    input.set_value(SharedString::from(value.to_string()), window, cx);
                });
            },
        )
        .detach();
        let basic_focused_state = cx.new(|cx| InputState::new(window, cx).default_value("Focused"));

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

        let textarea_demo = cx.new(|cx| TextareaState::new(window, cx));

        let select_demo = cx.new(|cx| {
            SelectState::new(
                SearchableVec::new(vec![
                    SharedString::from("Adwaita"),
                    SharedString::from("Breeze"),
                    SharedString::from("Yaru"),
                ]),
                None,
                window,
                cx,
            )
        });

        let palette_state = cx.new(|cx| CommandState::new(window, cx));

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

        let range_slider_state = cx.new(|_cx| SliderState::new().default_value(20.0..80.0));
        let vertical_slider_state = cx.new(|_cx| SliderState::new().default_value(40.0));

        // The desktop's own theme, which `install_system` installs once the
        // view is built; where it cannot be read, gpui-component's built-in
        // theme, so the window still renders.
        let is_dark = color_mode.is_dark();
        let system = crate::support::read_system_theme();
        let initial_default_preset = match &system {
            Ok(system) => system.preset.clone(),
            Err(_) => platform_preset_name().name.to_string(),
        };
        let preset_combobox = cx.new(|cx| {
            ComboboxState::new(
                PresetDelegate::new(&initial_default_preset),
                vec![gpui_component::IndexPath::default().row(0)],
                window,
                cx,
            )
            .searchable(true)
        });

        // `Change`, not `Confirm`: the Combobox confirms whenever its popup
        // closes, chosen or not (combobox.rs, ComboboxState::toggle_menu).
        cx.subscribe_in(
            &preset_combobox,
            window,
            |this: &mut Self, _entity, event: &ComboboxEvent<PresetDelegate>, window, cx| {
                if let ComboboxEvent::Change(values) = event
                    && let Some(name) = values.first()
                {
                    this.apply_theme_by_name(name, window, cx);
                    cx.notify();
                }
            },
        )
        .detach();

        // Its rows and the row chosen are the installed theme's, which
        // `show_icon_choice` sets once the theme is installed.
        let installed_themes = list_freedesktop_themes();
        let icon_theme_select =
            cx.new(|cx| SelectState::new(SearchableVec::new(Vec::new()), None, window, cx));

        cx.subscribe_in(
            &icon_theme_select,
            window,
            |this: &mut Self,
             _entity,
             event: &SelectEvent<SearchableVec<SharedString>>,
             window,
             cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    this.select_icon_set(value.as_ref(), window, cx);
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
        let color_select_state = cx.new(|cx| ColorPickerState::new(window, cx));
        let speech_state = cx.new(|cx| {
            SpeechState::new(cx)
                .recognizer(SilentRecognizer)
                .input(SyntheticAudio::default())
                .system_fallback(false)
        });

        // Date picker state
        let date_picker_state =
            cx.new(|cx| gpui_component::date_picker::DatePickerState::new(window, cx));

        // Time field state, and an input holding one token: inserting it is
        // fallible, so a refused token leaves plain text and says why.
        let time_field_state = cx.new(|cx| TimeFieldState::new(window, cx));
        let token_input_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            if let Err(error) =
                state.replace_with_token(InlineToken::new("mention", "@native-theme"), window, cx)
            {
                eprintln!("showcase: the sample token was refused: {error:?}");
            }
            state
        });

        // Calendar state
        let calendar_state = cx.new(|cx| gpui_component::calendar::CalendarState::new(window, cx));

        // What the inspector shows. Created before the Data page's states:
        // their delegates build rows that report themselves to it.
        let info_ui = cx.new(|_| InfoRegistry::new());
        let menus = cx.new(|cx| demo::MenuBar::new(info_ui.clone(), cx));

        // Table state with sample data
        let table_state = cx.new(|cx| {
            let rows: Vec<[SharedString; 3]> = vec![
                ["Alice".into(), "Engineer".into(), "Active".into()],
                ["Bob".into(), "Designer".into(), "Away".into()],
                ["Carol".into(), "Manager".into(), "Active".into()],
                ["Dave".into(), "Intern".into(), "Offline".into()],
                ["Eve".into(), "DevOps".into(), "Active".into()],
            ];
            let delegate = SampleTableDelegate {
                ui: info_ui.clone(),
                columns: vec![
                    Column::new("name", "Name"),
                    // One sortable column shows the table's sort button.
                    Column::new("role", "Role").sortable(),
                    Column::new("status", "Status"),
                ],
                unsorted: rows.clone(),
                rows,
                selected_row: None,
                selection_shown: true,
                right_clicked_row: None,
            };
            TableState::new(delegate, window, cx)
        });
        // The rows' infos follow what the table selects (support.rs,
        // `SampleTableDelegate::follow`).
        cx.subscribe(&table_state, |_this, table, event: &TableEvent, cx| {
            table.update(cx, |table, cx| {
                table.delegate_mut().follow(event);
                cx.notify();
            });
        })
        .detach();

        // List state with sample items
        let list_state = cx.new(|cx| {
            let delegate = SampleListDelegate {
                ui: info_ui.clone(),
                id_prefix: "data-list-row",
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
        let basic_list = cx.new(|cx| {
            let delegate = SampleListDelegate {
                ui: info_ui.clone(),
                id_prefix: "basic-list-row",
                items: (1..=BASIC_LIST_ROWS)
                    .map(|n| SharedString::from(format!("Item {n}")))
                    .collect(),
                selected: Some(BASIC_LIST_SELECTED),
            };
            let mut state = ListState::new(delegate, window, cx);
            state.set_selected_index(
                Some(gpui_component::IndexPath::default().row(BASIC_LIST_SELECTED)),
                window,
                cx,
            );
            state
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
        let diff_state =
            cx.new(|cx| DiffState::new(DiffFile::parse(DIFF_SAMPLE).unwrap_or_default(), cx));

        // Set up application menus: `cx.set_menus` hands them to the
        // platform's own menu bar; the showcase draws them itself where the
        // platform has none (`demo::menu_bar`, `demo::title_bar`).
        cx.set_menus(chrome::menus());
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        // The Basic page's focused field holds the keyboard focus at start,
        // so its focus border shows; it is inside the view, whose actions
        // it still reaches.
        basic_focused_state.update(cx, |state, cx| state.focus(window, cx));
        let _refocus = cx.on_focus_lost(window, |this: &mut Self, window, cx| {
            this.focus_handle.focus(window, cx)
        });

        // Follow the OS theme's changes (feature `watch`, `on_os_theme_change`),
        // in an interactive run only (`CliArgs::follows_os`): a
        // capture and a layout dump show the theme the run started with, and
        // under `--screenshot` the watcher's background thread cleanup races
        // with the Cocoa runtime on macOS CI, causing SIGTRAP on exit, so it
        // is never started there. Skip under `cargo test` too: the watcher
        // puts a file watch on the desktop's own configuration, and the
        // self-tests below build this view for real. Nothing they assert
        // depends on it.
        #[cfg(feature = "watch")]
        let _theme_watcher =
            if cfg!(test) || !crate::CliArgs::parse(std::env::args().skip(1)).follows_os() {
                None
            } else {
                let showcase = cx.weak_entity();
                // The showcase's one window, so it watches as long as it runs.
                native_theme_gpui::on_theme_change(window, cx, move |_event, window, cx| {
                    // Gone once the window closed; nothing to follow then.
                    let _ = showcase.update(cx, |this, cx| this.on_os_theme_change(window, cx));
                })
                .ok()
            };

        // The install below is made before this observer is active (gpui
        // activates it at the end of this update), so the first frame builds
        // the panel's look from `None` too.
        let _native_theme =
            cx.observe_global::<native_theme_gpui::NativeTheme>(|this: &mut Self, _cx| {
                this.panel_look = None
            });

        let inspector = {
            let (ui, showcase) = (info_ui.clone(), cx.weak_entity());
            cx.new(|cx| Inspector::new(ui, showcase, cx))
        };
        let body_layout = cx.new(|_| ResizableState::default());

        let fg = cx.theme().foreground;
        let mut showcase = Self {
            preset_combobox,
            current_theme_name: "default".into(),
            default_preset: initial_default_preset,
            system: None,
            is_dark,
            color_mode,
            original_font: unread_font(),
            original_mono_font: unread_font(),
            active_page: Page::Basic,
            side_panel_visible: true,
            body_layout,
            side_panel_width: LEFT_PANEL_WIDTH,
            info_ui,
            menus,
            content_scroll: gpui::ScrollHandle::new(),
            inspector,
            panel_look: None,
            _native_theme,
            status_title_drawn: None,
            #[cfg(test)]
            status_environment_drawn: gpui::TextLayout::default(),
            #[cfg(test)]
            frame_for_test: None,
            #[cfg(test)]
            os_preferences_for_test: None,
            #[cfg(test)]
            icon_theme_detection_for_test: None,
            palette_state,
            focus_handle,
            _refocus,
            overlay_gap: Rc::new(Cell::new(None)),
            dialog_icons: SharedDialogIcons::default(),
            // Nothing was read yet, so nothing is claimed: every key stays
            // `None` and the layout accessors report it as such.
            layout: native_theme::theme::LayoutTheme::default(),
            pointer_shield: false,
            basic_hint_state,
            basic_text_state,
            basic_disabled_state,
            basic_select,
            basic_slider_state,
            basic_radio: Some(0),
            basic_textarea,
            basic_list,
            basic_tab: 0,
            basic_segment: 1,
            basic_expanded: [true, false],
            basic_number_state,
            basic_focused_state,
            input_state,
            input_height_state,
            textarea_demo,
            select_demo,
            input_group_state,
            input_group_button_state,
            input_group_textarea_state,
            number_input_state,
            form_name_state,
            form_email_state,
            slider_state,
            range_slider_state,
            vertical_slider_state,
            otp_state,
            color_picker_state,
            color_select_state,
            speech_state,
            date_picker_state,
            time_field_state,
            token_input_state,
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
            editor_state,
            diff_state,
            table_state,
            list_state,
            tree_state,
            page: 5,
            chat_messages: initial_chat,
            chat_scroller,
            attachment_status: AttachmentStatus::Uploading,
            toggle_bold: false,
            toggle_italic: false,
            alert_choice: None,
            color_mode_select,
            icon_theme_select,
            icon_theme_name: String::new(),
            icon_set_enum: None,
            loaded_icons: Vec::new(),
            gpui_icons: Vec::new(),
            loaded_icon_sources: Vec::new(),
            gpui_icon_sources: Vec::new(),
            icon_cache_fg: fg,
            icon_load: None,
            icon_set_choice: IconSetChoice::System,
            icon_choice_follows_preset: true,
            installed_themes,
            current_icon_theme: None,
            current_icon_set: system_icon_set(),
            has_toml_icon_theme: false,
            animated_frame_sources: Vec::new(),
            animated_frame_durations: Vec::new(),
            animated_frame_indices: Vec::new(),
            animated_spin_sources: Vec::new(),
            animation_timer: None,
            error_message: None,
            #[cfg(feature = "watch")]
            _theme_watcher,
        };
        match system {
            Ok(system) => showcase.install_system(system, window, cx),
            Err(e) => {
                Theme::sync_system_appearance(Some(window), cx);
                showcase.current_icon_theme = system_icon_theme().ok();
                showcase.error_message = Some(format!("Failed to load OS theme: {e}"));
            }
        }
        showcase.show_icon_choice(window, cx);
        showcase.reload_icons(window, cx);
        showcase
    }

    fn show_theme_error(&mut self, msg: &str) {
        self.error_message = Some(msg.to_string());
    }

    /// What a theme that failed to load -- the OS theme unread, a preset
    /// unread or unresolved -- leaves on screen.
    ///
    /// With a native theme installed, the window stays exactly as that
    /// theme drew it: its mode too, so `is_dark` goes back to the mode drawn.
    /// Upstream's mode switch would rebuild the palette from its stored
    /// configs, which recomputes upstream's derived colours and, where the
    /// native theme was installed with one variant (`apply`), shows
    /// upstream's own colours for the other mode.
    ///
    /// With none installed -- `Showcase::new` falls back to gpui-component's
    /// built-in theme when the OS read fails, and that theme has both
    /// variants -- the user's light/dark choice still lands through
    /// upstream's mode switch; without it the selector moved `is_dark` and
    /// nothing else, and the interface stayed in the mode it started in.
    fn keep_installed_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if cx.native_theme().is_some() {
            self.is_dark = cx.theme().mode.is_dark();
        } else {
            Theme::change(gpui_theme_mode(self.is_dark), Some(window), cx);
        }
    }

    /// The accessibility preferences a theme is installed with: `os`, the
    /// OS's as read for this install, with the ones the user set in the
    /// Preferences sheet put over them (`PreferenceOverrides`). A preference
    /// the user set stays; one the user did not follows the OS.
    fn install_preferences(cx: &App, os: AccessibilityPreferences) -> AccessibilityPreferences {
        demo::PreferenceOverrides::of(cx).over(os)
    }

    /// The OS's accessibility preferences as last read, from the detection
    /// cache that the last system read filled and a reload clears
    /// (`reload_system_theme`), so a preset switch runs no platform reader;
    /// in the self-tests, the ones `os_preferences_for_test` holds, where it
    /// holds any.
    fn os_preferences(&self) -> AccessibilityPreferences {
        #[cfg(test)]
        if let Some(prefs) = &self.os_preferences_for_test {
            return prefs.clone();
        }
        native_theme::detect::accessibility_preferences()
    }

    /// Install `system`, the desktop's own theme as `SystemTheme::from_system`
    /// read it, in the mode shown, and keep it (`Showcase::system`): what
    /// `default` installs, at startup too. Its accessibility preferences are
    /// the OS's it read, with the user's over them.
    ///
    /// The preset the pipeline settled on is what the preset switch's
    /// `default` row and the status bar name, so a change of it relabels
    /// that row.
    pub(crate) fn install_system(
        &mut self,
        mut system: native_theme::SystemTheme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mode = color_mode_of(self.is_dark);
        let resolved = system.pick(mode);
        self.original_font = resolved.defaults.font.clone();
        self.original_mono_font = resolved.defaults.mono_font.clone();
        // The mode shown's own variant's: `system.icon_theme` is the one of
        // the mode the OS is in.
        self.current_icon_theme = system.icon_theme_for(mode).map(str::to_owned);
        self.current_icon_set = system.icon_set;
        self.layout = system.layout.clone();
        // Platform presets always specify icon_theme
        self.has_toml_icon_theme = true;
        // Installed with both variants stored, so the showcase's own
        // light/dark choice goes through upstream's mode switch, which
        // reproduces the native palette from the installed configs (D34).
        // Kept with the OS's preferences as read.
        let prefs = Self::install_preferences(cx, system.accessibility.clone());
        let os = std::mem::replace(&mut system.accessibility, prefs);
        native_theme_gpui::apply_system_theme(&system, cx);
        system.accessibility = os;
        if self.is_dark != system.mode.is_dark() {
            Theme::change(gpui_theme_mode(self.is_dark), Some(window), cx);
        }
        if self.default_preset != system.preset {
            self.default_preset = system.preset.clone();
            let rows = PresetDelegate::new(&self.default_preset);
            self.preset_combobox.update(cx, |combobox, cx| {
                let chosen = combobox.selected_values();
                combobox.set_items(rows, window, cx);
                combobox.set_selected_values(&chosen, window, cx);
            });
        }
        self.error_message = None;
        self.system = Some(system);
    }

    /// Install the preset of `name`. False, with the error reported and the
    /// installed theme left as it is, where it cannot be read or resolved.
    fn install_preset(&mut self, name: &str, cx: &mut Context<Self>) -> bool {
        let nt = match native_theme::theme::Theme::preset(name) {
            Ok(t) => t,
            Err(e) => {
                self.show_theme_error(&format!("Failed to load preset '{name}': {e}"));
                return false;
            }
        };
        let r = match nt.resolve(color_mode_of(self.is_dark)) {
            Ok(r) => r,
            Err(e) => {
                self.show_theme_error(&format!("Theme '{name}' resolution failed: {e}"));
                return false;
            }
        };
        self.layout = nt.layout.clone();
        self.has_toml_icon_theme = r.icon_theme_explicit;
        self.current_icon_set = r.icon_set;
        self.current_icon_theme = r.icon_theme.map(Cow::into_owned);
        self.original_font = r.variant.defaults.font.clone();
        self.original_mono_font = r.variant.defaults.mono_font.clone();
        // Accessibility is orthogonal to the theme choice, so a preset is
        // installed with the OS's preferences and the user's over them, as
        // the desktop's own theme is.
        let prefs = Self::install_preferences(cx, self.os_preferences());
        let theme = to_theme(&r.variant, &nt.name, r.mode.is_dark(), &prefs);
        native_theme_gpui::apply(theme, &r.variant, &prefs, cx);
        self.error_message = None;
        true
    }

    /// Install the theme of `name`: `default`, the desktop's own, or a
    /// preset's. Every preset, colour-mode, reload and palette switch
    /// installs through here. A theme that fails to load leaves the
    /// installed one -- its name, layout, mode and every colour
    /// (`Showcase::keep_installed_theme`) -- and false is returned.
    ///
    /// `default` installs the desktop's theme kept from its last install
    /// (`Showcase::system`), in the mode now shown, and reads the OS only
    /// where none is kept: at the first install, where the last read
    /// failed, and after a reload (`reload_system_theme`).
    pub(crate) fn apply_theme_by_name(
        &mut self,
        name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.info_ui.update(cx, |r, _| r.screen_changed());
        let installed = if name == "default" {
            let kept = self.system.take();
            match kept.map_or_else(crate::support::read_system_theme, Ok) {
                Ok(system) => {
                    self.install_system(system, window, cx);
                    true
                }
                Err(e) => {
                    self.show_theme_error(&format!("Failed to load OS theme: {e}"));
                    false
                }
            }
        } else {
            self.install_preset(name, cx)
        };
        if installed {
            self.current_theme_name = name.to_string();
        } else {
            self.keep_installed_theme(window, cx);
        }
        self.show_installed_theme(window, cx);
        self.show_icon_choice(window, cx);
        self.reload_icons(window, cx);
        installed
    }

    /// Show the installed theme chosen in the preset switch, whichever way
    /// it was installed -- and the one still installed where another failed
    /// to load.
    fn show_installed_theme(&self, window: &mut Window, cx: &mut Context<Self>) {
        let key = SharedString::from(self.current_theme_name.clone());
        if self.preset_combobox.read(cx).selected_value().as_ref() == Some(&key) {
            return;
        }
        self.preset_combobox.update(cx, |combobox, cx| {
            combobox.set_selected_values(&[key], window, cx)
        });
    }

    /// The preset switch's and the status bar's name for `default`: the
    /// platform preset it is built on.
    pub(crate) fn default_label(&self) -> String {
        rows::default_label(&self.default_preset)
    }

    /// Install `mode`: the current theme again, in that mode. Where that
    /// fails with a native theme installed, the window keeps the mode it
    /// was drawn in, and so does the colour-mode Select.
    fn set_color_mode(&mut self, mode: AppColorMode, window: &mut Window, cx: &mut Context<Self>) {
        let name = self.current_theme_name.clone();
        self.install_in_mode(&name, mode, window, cx);
    }

    /// Install the theme of `name` in `mode`. Where that fails with a
    /// native theme installed, the window keeps the mode it was drawn in,
    /// and so does the colour-mode Select.
    pub(crate) fn install_in_mode(
        &mut self,
        name: &str,
        mode: AppColorMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let before = self.color_mode;
        self.color_mode = mode;
        self.is_dark = mode.is_dark();
        if !self.apply_theme_by_name(name, window, cx) && cx.native_theme().is_some() {
            self.color_mode = before;
        }
        self.show_color_mode(window, cx);
    }

    /// The key of the preset the installed theme is: the current theme's,
    /// or, under "default", the platform preset it is built on.
    pub(crate) fn platform_preset(&self) -> &str {
        if self.current_theme_name == "default" {
            &self.default_preset
        } else {
            &self.current_theme_name
        }
    }

    /// Show the current colour mode chosen in the theme settings' colour-mode
    /// Select, whichever way it was chosen.
    pub(crate) fn show_color_mode(&self, window: &mut Window, cx: &mut Context<Self>) {
        let label = SharedString::from(self.color_mode.short_label());
        self.color_mode_select.update(cx, |select, cx| {
            select.set_selected_value(&label, window, cx);
        });
    }

    /// What the theme watcher does when the OS theme changes, in every colour
    /// mode: read the desktop's settings again and re-install the current
    /// theme in the mode chosen (`reload_system_theme`). A chosen Light or
    /// Dark picks only the variant; the OS still owns the accent, the fonts
    /// and the icon theme `default` shows.
    #[cfg(any(test, feature = "watch"))]
    pub(crate) fn on_os_theme_change(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.reload_system_theme(window, cx);
        cx.notify();
    }

    /// Read the desktop's settings again and re-install the current theme
    /// from them, in the mode chosen: what the theme watcher does when the
    /// OS theme changes, and what `ReloadTheme` asks for.
    fn reload_system_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        native_theme::detect::invalidate_caches();
        // The desktop's theme is read again, and a system set's icons
        // loaded again: the system icon theme may have changed with it. A
        // bundled set's cannot, and stay.
        self.system = None;
        self.icon_load = self
            .icon_load
            .take()
            .filter(|load| !load.set.is_some_and(crate::support::is_system_set));
        self.is_dark = self.color_mode.is_dark();
        let name = self.current_theme_name.clone();
        self.apply_theme_by_name(&name, window, cx);
    }

    /// Show `page`. Every way to a page -- the page TabBar, the View menu,
    /// the command palette, the Layout page's Breadcrumb, `--tab` -- goes
    /// through here, so the info of what the page change takes off the
    /// screen does not stay on show (spec §4.3.4), and the animation timer
    /// runs while the Icons page is shown, and only then.
    pub(crate) fn show_page(&mut self, page: Page, cx: &mut Context<Self>) {
        if self.active_page != page {
            self.active_page = page;
            self.info_ui.update(cx, |r, _| r.screen_changed());
            // Running on the Icons page alone, which draws its frames.
            self.start_animation_timer(cx);
        }
        cx.notify();
    }

    pub(crate) fn on_show_page(
        &mut self,
        action: &ShowPage,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(page) = Page::at(action.0) {
            self.show_page(page, cx);
        }
    }

    /// Hide the side panel or show it again. Hidden, its widgets leave the
    /// screen, so the info of one of them does not stay on show (spec
    /// §4.3.4), as on a page change.
    pub(crate) fn on_toggle_side_panel(
        &mut self,
        _: &ToggleSidePanel,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.rearrange(cx, |this| {
            this.side_panel_visible = !this.side_panel_visible
        });
        self.info_ui.update(cx, |r, _| r.screen_changed());
    }

    /// Add or take away the side panel.
    ///
    /// The group's state keeps its sizes by panel position (gpui-base
    /// resizable/mod.rs, `ResizableState::sync_panels_count`), so a panel
    /// leaving the group would hand its width to its neighbour; and a panel
    /// kept but hidden (`ResizablePanel::visible`) keeps bounds that a drag of
    /// another handle still counts (`ResizableState::resize_panel_at_handle`).
    /// So the width the side panel has now is kept here, and a new state
    /// takes over, the panel taking that width again as its first size. A new
    /// state, not the old one cleared: `ResizableState::clear` leaves the
    /// index of a handle being dragged (gpui-base resizable/mod.rs:266-269),
    /// and the next pointer move would look that panel up in a state that no
    /// longer has it (resizable/panel.rs:485). The old state stays with the
    /// listeners the last frame painted, whole.
    fn rearrange(&mut self, cx: &mut Context<Self>, change: impl FnOnce(&mut Self)) {
        let sizes = self.body_layout.read(cx).sizes().clone();
        let panels = usize::from(self.side_panel_visible).saturating_add(1);
        if self.side_panel_visible
            && sizes.len() == panels
            && let Some(&width) = sizes.first()
        {
            self.side_panel_width = width;
        }
        change(self);
        self.body_layout = cx.new(|_| ResizableState::default());
        cx.notify();
    }

    pub(crate) fn on_set_color_mode(
        &mut self,
        action: &SetColorMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_color_mode(action.0, window, cx);
        cx.notify();
    }

    pub(crate) fn on_reload_theme(
        &mut self,
        _: &ReloadTheme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.reload_system_theme(window, cx);
        cx.notify();
    }

    pub(crate) fn on_set_preset(
        &mut self,
        action: &SetPreset,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.apply_theme_by_name(&action.0, window, cx);
        cx.notify();
    }

    pub(crate) fn on_open_command_palette(
        &mut self,
        _: &OpenCommandPalette,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        chrome::open_command_palette(self, window, cx);
    }

    pub(crate) fn on_open_preferences(
        &mut self,
        _: &OpenPreferences,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        chrome::open_preferences(self, window, cx);
    }

    pub(crate) fn on_open_about(
        &mut self,
        _: &OpenAbout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        chrome::open_about(self, window, cx);
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for Showcase {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.overlay_gap.set(geometry::widget_gap(&self.layout));
        // Before the inspector, a child view, renders and borrows it.
        if self.panel_look.is_none() {
            self.panel_look = PanelLook::of(self, cx);
        }
        self.dialog_icons.set(DialogIcons {
            dialog: self.sample_icon(IconName::CircleX),
            alert: self.sample_icon(IconName::TriangleAlert),
        });
        let (foreground, background) = (cx.theme().foreground, cx.theme().background);
        // Ensure icon image caches match the current foreground color
        if foreground != self.icon_cache_fg {
            self.rebuild_icon_caches(foreground, window, cx);
        }

        let active_page = self.active_page;

        // The content panel: the page TabBar, then the Alert of a theme that
        // failed to load, then the page, scrolling. The TabBar comes first so
        // that an Alert appearing never moves the page navigation.
        let content = v_flex()
            .size_full()
            .overflow_hidden()
            .debug_selector(|| CONTENT_PANEL.into())
            .child(elements::record(&self.info_ui, "chrome.content"))
            .child(chrome::page_tabs(self, cx))
            .children(self.error_message.clone().map(|message| {
                demo::alert(
                    &self.info_ui,
                    cx,
                    "content-alert",
                    message,
                    self.chrome_icon(&IconName::CircleX),
                    &self.icon_set_label(),
                )
                .debug_selector(|| CONTENT_ALERT.into())
            }))
            .child(
                // The bar is drawn over the right edge of the scroll area, so
                // while the page overflows it the page keeps that width free
                // (`gutter_scroll`); the page roots' own padding is untouched.
                crate::support::gutter_scroll(
                    "content-scroll-outer",
                    &self.content_scroll,
                    div()
                        .debug_selector(|| CONTENT_SCROLL.into())
                        // PAGE_ROOT is what `every_page_lays_out` looks the page up
                        // by, and it goes on each arm rather than on one wrapper
                        // around the match: the test asserts the page's own root
                        // has a size, and a wrapper would report this scroll
                        // container's size for every page -- including a page that
                        // rendered nothing. That is what makes the ten
                        // `impl IntoElement + InteractiveElement` signatures worth
                        // their noise.
                        .child(match active_page {
                            Page::Basic => self
                                .render_basic_page(window, cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Buttons => self
                                .render_buttons_page(window, cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Inputs => self
                                .render_inputs_page(window, cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Data => self
                                .render_data_page(window, cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Feedback => self
                                .render_feedback_page(window, cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Typography => self
                                .render_typography_page(cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Layout => self
                                .render_layout_page(window, cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Overlays => self
                                .render_overlays_page(window, cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Charts => self
                                .render_charts_page(cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::Icons => self
                                .render_icons_page(cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                            Page::ThemeMap => self
                                .render_theme_map_page(cx)
                                .debug_selector(|| PAGE_ROOT.into())
                                .into_any_element(),
                        }),
                    {
                        let this = cx.entity().downgrade();
                        move |cx: &mut gpui::App| {
                            this.update(cx, |_, cx| cx.notify()).ok();
                        }
                    },
                    cx,
                )
                .flex_1(),
            );

        // The body: the side panel | content, one resizable group (spec S1).
        // A hidden side panel leaves the group, the content taking its room.
        let side_panel = self.side_panel_visible.then(|| {
            // The splitter's line is painted back from the boundary over the
            // strip the panel keeps clear for it (`demo::splitter_reserve`).
            // As wide as the side panel and the strip it keeps clear for the
            // splitter's line, so the side panel keeps its width whatever
            // the line's.
            let reserve = demo::splitter_reserve(cx).unwrap_or_default();
            resizable_panel()
                .size(gpui::px(
                    f32::from(self.side_panel_width) + f32::from(reserve),
                ))
                .flex_none()
                .child(
                    div()
                        .size_full()
                        .when_some(demo::splitter_reserve(cx), |panel, reserve| {
                            panel.pr(reserve)
                        })
                        .child(chrome::side_panel(self, cx)),
                )
        });
        // The content panel carries the handle on its left edge (gpui-base
        // resizable/panel.rs, `ResizablePanel::render`).
        let handles = side_panel
            .is_some()
            .then_some((CHROME_HANDLE, "side panel | content"))
            .into_iter()
            .collect();
        // The content starts after the splitter's line, which the handle
        // draws over the content panel's leading edge (`demo::HANDLE_SIZE`
        // wide; gpui-base resizable/panel.rs, `ResizablePanel::render`).
        let after_line = side_panel.is_some();
        let panels: Vec<_> = side_panel
            .into_iter()
            .chain([resizable_panel().child(
                div()
                    .size_full()
                    .when(after_line, |panel| panel.pl(demo::HANDLE_SIZE))
                    .child(content),
            )])
            .collect();
        let body = h_resizable("body")
            .with_state(&self.body_layout)
            .with_handle_appearance(demo::resize_handles(&self.info_ui, handles))
            .children(panels);

        let shown = self
            .inspector
            .read(cx)
            .shown_title(cx)
            .map(SharedString::from);
        self.status_title_drawn = shown.clone();
        let environment = SharedString::from(chrome::status_environment(self, cx).join(" · "));
        let environment_text = gpui::StyledText::new(environment.clone());
        #[cfg(test)]
        {
            self.status_environment_drawn = environment_text.layout().clone();
        }

        // Main layout: the title bar or the menu-bar row (spec S8), the
        // toolbar, the body and the status bar. Dialogs, sheets and
        // notifications are drawn beside this view, not inside it, by
        // gpui-component's `WindowState` plugin (gpui-base root.rs,
        // `Root::render`), so the showcase's actions are handled around both
        // by `host::ShowcaseHost`, where an action dispatched from inside an
        // overlay still finds them.
        div()
            .relative()
            .size_full()
            .bg(background)
            .text_color(foreground)
            .track_focus(&self.focus_handle)
            .child(elements::record(&self.info_ui, "chrome.window"))
            .child(
                v_flex()
                    .size_full()
                    .children(chrome::window_top(
                        self,
                        self.frame(window),
                        window.is_window_active(),
                        cx,
                    ))
                    .child(chrome::toolbar(self, cx))
                    .child(
                        h_flex()
                            .w_full()
                            .flex_1()
                            // A flex item's minimum height is its content's
                            // unless it clips; the body has to fit under the
                            // toolbar, not push the window taller.
                            .overflow_hidden()
                            .child(div().flex_1().min_w_0().h_full().child(body)),
                    )
                    .child(chrome::status_bar(
                        self,
                        cx,
                        &environment,
                        environment_text,
                        shown,
                    )),
            )
            // Deferred at the highest priority, so it is the topmost hitbox:
            // dialogs, menus and popovers paint deferred themselves (gpui-base
            // dialog.rs, popup.rs), above any ordinary element, and a deferred
            // element is painted by priority wherever it sits in the tree; an
            // occluding box takes the pointer out of the hit test of
            // everything under it (gpui-pre window.rs,
            // `HitboxBehavior::BlockMouse`), so no widget is hovered and the
            // inspector is handed no hover. A popup nested inside a deferred
            // overlay (a submenu) is prepainted later and still lands above.
            .when(self.pointer_shield, |root| {
                root.child(
                    gpui::deferred(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .occlude()
                            .debug_selector(|| POINTER_SHIELD.into()),
                    )
                    .with_priority(usize::MAX),
                )
            })
    }
}
