//! The showcase's state, theme switching, and the view that renders it.

use gpui::{
    Action, App, Context, Entity, FocusHandle, Hsla, ImageSource, IntoElement, KeyBinding, Menu,
    ParentElement, Pixels, Render, SharedString, Styled, Subscription, Task, Window, actions, div,
    prelude::*,
};
use gpui_component::{
    ActiveTheme, GlobalState, ResizableState, Root,
    attachment::AttachmentStatus,
    carousel::CarouselState,
    color_picker::ColorPickerState,
    combobox::{ComboboxEvent, ComboboxState},
    command::CommandState,
    h_flex, h_resizable,
    input::{EditorState, InputState, NumberInputEvent, OtpState, StepAction, TextareaState},
    list::ListState,
    menu::AppMenuBar,
    message_scroller::MessageScrollerState,
    resizable_panel,
    scroll::ScrollableElement,
    select::{SearchableVec, SelectEvent, SelectState},
    slider::{SliderEvent, SliderState},
    table::{Column, TableEvent, TableState},
    theme::Theme,
    tree::{TreeItem, TreeState},
    v_flex,
};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use native_theme::detect::system_is_dark;
use native_theme::icons::{
    FreedesktopLoader, IconSetChoice, default_icon_choice, list_freedesktop_themes,
    load_icon_indicator,
};
use native_theme::pipeline::platform_preset_name;
use native_theme::theme::{
    AnimatedIcon, IconData, IconRole, IconSet, TransformAnimation, system_icon_set,
    system_icon_theme,
};
use native_theme_gpui::icons::{animated_frames_to_image_sources, to_image_source};
use native_theme_gpui::to_theme;
use native_theme_gpui::{AccessibilityPreferences, geometry};

use crate::chrome;
use crate::info::{InfoRegistry, epoch_marker};
use crate::inspector::Inspector;
use crate::support::{
    CAROUSEL_SLIDES, ChatMessage, EDITOR_SAMPLE, IconEntry, IconSource, NativeStyled,
    PresetDelegate, SampleListDelegate, SampleTableDelegate, initial_chat_messages, load_all_icons,
    load_gpui_icons, parse_icon_set_choice, release_sources,
};
use crate::{
    CHROME_HANDLE_INSPECTOR, CHROME_HANDLE_NAV, CONTENT_ALERT, CONTENT_PANEL, CONTENT_SCROLL,
    INSPECTOR_WIDTH, NAV_WIDTH, PAGE_ROOT, Page, demo,
};

/// gpui-component's mode for the showcase's light/dark flag.
fn gpui_theme_mode(is_dark: bool) -> gpui_component::theme::ThemeMode {
    if is_dark {
        gpui_component::theme::ThemeMode::Dark
    } else {
        gpui_component::theme::ThemeMode::Light
    }
}

// ---------------------------------------------------------------------------
// Actions (spec §2.2)
// ---------------------------------------------------------------------------
//
// One action backs a menu item, its key binding, a toolbar button and a
// command-palette entry, where each has one.

actions!(
    showcase,
    [
        Quit,
        ToggleSidebar,
        ToggleInspector,
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

/// Install a colour mode, as the toolbar's colour-mode switch does.
#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = showcase, no_json)]
pub(crate) struct SetColorMode(pub AppColorMode);

/// Install the preset of this key, as the toolbar's preset switch does, and
/// show it chosen there.
#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = showcase, no_json)]
pub(crate) struct SetPreset(pub SharedString);

/// Bind the showcase's keys (spec §2.2) and quit on [`Quit`].
///
/// `Quit` is handled here, for the whole application, so it quits whatever
/// has the focus; the actions that change the showcase are handled on its
/// view (`Showcase::render`).
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("ctrl-q", Quit, None),
        KeyBinding::new("ctrl-b", ToggleSidebar, None),
        KeyBinding::new("ctrl-i", ToggleInspector, None),
        KeyBinding::new("ctrl-k", OpenCommandPalette, None),
        KeyBinding::new("ctrl-,", OpenPreferences, None),
    ]);
    cx.on_action(|_: &Quit, cx| cx.quit());
}

// ---------------------------------------------------------------------------
// Color mode (light / dark / system)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AppColorMode {
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

    /// Display label for the colour-mode switch, with system preference in parentheses.
    pub(crate) fn label(self) -> String {
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
// Main view
// ---------------------------------------------------------------------------

pub(crate) struct Showcase {
    /// The toolbar's preset switch (spec §2.3).
    pub(crate) preset_combobox: Entity<ComboboxState<PresetDelegate>>,
    pub(crate) current_theme_name: String,
    /// Dynamic label for the "default" theme entry, updated on color mode change.
    pub(crate) default_label: String,
    pub(crate) is_dark: bool,
    pub(crate) color_mode: AppColorMode,
    /// Original native-theme font spec, for display purposes.
    pub(crate) original_font: native_theme::theme::ResolvedFontSpec,
    /// Original native-theme mono font spec, for display purposes.
    pub(crate) original_mono_font: native_theme::theme::ResolvedFontSpec,

    pub(crate) active_page: Page,
    /// Whether the Sidebar is collapsed to its icons (spec §2.4).
    pub(crate) nav_collapsed: bool,
    /// Whether the inspector's panel is shown (spec §1.1, §2.6).
    pub(crate) inspector_visible: bool,
    /// The window's resizable group's state, owned here so a change of its
    /// arrangement can start it afresh (`Showcase::rearrange`).
    body_layout: Entity<ResizableState>,
    /// The Sidebar's and the inspector's widths, as dragged, which a panel
    /// that leaves the group and comes back takes again.
    nav_width: Pixels,
    inspector_width: Pixels,

    /// Where the showcase's widgets report their info (spec §4).
    pub(crate) info_ui: Entity<InfoRegistry>,
    /// The inspector panel, which shows the info the registry settles on.
    pub(crate) inspector: Entity<Inspector>,
    /// The title the status bar's hover label showed in the last frame;
    /// `None` where it showed none.
    pub(crate) status_title_drawn: Option<SharedString>,
    /// The title bar's menus.
    pub(crate) menu_bar: Entity<AppMenuBar>,
    /// The command palette's query and highlighted row (spec §2.8).
    pub(crate) palette_state: Entity<CommandState>,
    /// The view's focus, so an action dispatched with nothing else focused
    /// still reaches the handlers `render` puts on the view.
    focus_handle: FocusHandle,
    /// Takes the focus back to the view when the focused element stops being
    /// drawn -- a page's widget whose page was left keeps its handle, and
    /// gpui would dispatch from the window's root instead (gpui-pre
    /// window.rs:6244-6252), out of the view's handlers' reach.
    _refocus: Subscription,

    /// `geometry::widget_gap` of `layout`, set as each frame starts, for the
    /// overlays: `Root` builds them anew for every frame from builders that
    /// cannot borrow the view (root.rs, `Root::render_dialog_layer`), so this
    /// is how the gap they read is the installed theme's, not the one they
    /// opened under.
    pub(crate) overlay_gap: Rc<Cell<Option<Pixels>>>,

    /// Layout spacing of the installed theme. It lives on the model, not on
    /// `ResolvedTheme`, so the geometry accessors take it from here rather
    /// than from `cx.native_theme()`.
    pub(crate) layout: native_theme::theme::LayoutTheme,

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
    pub(crate) otp_state: Entity<OtpState>,
    pub(crate) color_picker_state: Entity<ColorPickerState>,
    pub(crate) date_picker_state: Entity<gpui_component::date_picker::DatePickerState>,
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

    // Icon set selector state
    pub(crate) icon_set_select: Entity<SelectState<SearchableVec<SharedString>>>,
    pub(crate) icon_set_name: String,
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
    /// The user's icon set selection intent (library type).
    pub(crate) icon_set_choice: IconSetChoice,
    /// Cached list of installed freedesktop icon themes (populated once at init).
    pub(crate) installed_themes: Vec<String>,
    /// The current resolved theme's preferred icon theme (e.g. "breeze", "Lucide").
    pub(crate) current_icon_theme: String,
    /// The current resolved theme's icon set (loading mechanism).
    pub(crate) current_icon_set: IconSet,
    /// Whether the current theme's TOML specified `icon_theme` (before resolution).
    pub(crate) has_toml_icon_theme: bool,
    /// CLI override for the freedesktop icon theme (e.g. "breeze", "breeze-dark", "adwaita").
    pub(crate) icon_theme_override: Option<String>,

    // Animated Icons state
    /// Cached frame ImageSources for frame-based animations (set name, frames).
    pub(crate) animated_frame_sources: Vec<(String, Vec<ImageSource>)>,
    /// Frame duration in ms for each frame-based animation (parallel to animated_frame_sources).
    pub(crate) animated_frame_durations: Vec<u32>,
    /// Current frame index for each frame-based animation.
    pub(crate) animated_frame_indices: Vec<usize>,
    /// The SVG of each transform-based (spin) animation, which the page
    /// turns with `with_spin_animation` (set name, SVG bytes, duration_ms).
    pub(crate) animated_spin_sources: Vec<(String, Vec<u8>, u32)>,
    /// Timer task handle for frame cycling (dropped to cancel).
    pub(crate) animation_timer: Option<Task<()>>,

    /// Why the last theme failed to load, which an Alert at the top of the
    /// content reports (spec §2.5); `None` once a theme loads.
    pub(crate) error_message: Option<String>,

    // Theme watcher (runtime dark/light toggle detection)
    /// Flag set by the ThemeSubscription background thread when the OS theme changes.
    pub(crate) theme_change_flag: Arc<AtomicBool>,
    /// RAII guard keeping the theme watcher background thread alive.
    pub(crate) _theme_watcher: Option<native_theme::watch::ThemeSubscription>,
    /// Set by the watcher polling task; checked in render() where window access is available.
    pub(crate) pending_system_theme_change: bool,
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
    /// Called at init and whenever the icon set changes so that animated icon
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
            }
            Some(icon_set) => load_icon_indicator(icon_set),
            None => None,
        };
        let set_name = &self.icon_set_name;
        let fg = self.icon_cache_fg;
        if let Some(anim) = anim {
            match &anim {
                AnimatedIcon::Frames(data) => {
                    if let Some(anim_sources) =
                        animated_frames_to_image_sources(&anim, Some(fg), None)
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
                            duration_ms.get(),
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
    /// animations cached, and a tick does nothing while gpui's
    /// `reduce_motion` is on: the page then shows each animation's first
    /// frame, as gpui holds its own animations at their start.
    pub(crate) fn start_animation_timer(&mut self, cx: &mut Context<Self>) {
        // Drop old timer (cancels the task)
        self.animation_timer = None;

        if self.animated_frame_sources.is_empty() {
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

    /// The freedesktop theme the page's icons load from where the set is
    /// freedesktop: the `--icon-theme` override, else the theme the choice
    /// names; `None` is the system's own.
    pub(crate) fn freedesktop_theme(&self) -> Option<&str> {
        self.icon_theme_override
            .as_deref()
            .or(self.icon_set_choice.freedesktop_theme())
    }

    /// The icon set as the Icons page names it: a freedesktop set with the
    /// theme its icons load from.
    pub(crate) fn icon_set_label(&self) -> String {
        match self.icon_set_enum {
            Some(IconSet::Freedesktop) => format!(
                "freedesktop ({})",
                self.freedesktop_theme()
                    .map_or_else(system_icon_theme, str::to_string)
            ),
            _ => self.icon_set_name.clone(),
        }
    }

    /// Load the freedesktop icons from `theme` from now on: what
    /// `--icon-theme` asks for.
    pub(crate) fn set_icon_theme_override(
        &mut self,
        theme: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.icon_theme_override = Some(theme);
        // The icons on show were loaded before the override, so the page
        // would name this theme over another theme's icons.
        self.reload_icons(window, cx);
    }

    /// Load the icon set the icon-set Select names `display`, as the Select
    /// does when it is confirmed.
    pub(crate) fn select_icon_set(
        &mut self,
        display: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let is_gpui_builtin = display == "gpui-component built-in (Lucide)";
        self.icon_set_choice = parse_icon_set_choice(display);
        let effective = self
            .icon_set_choice
            .effective_icon_set(self.current_icon_set);
        // The page tells gpui-component's own icons by this name, which
        // `effective` (Lucide, for the built-in entry) would not give.
        self.icon_set_name = if is_gpui_builtin {
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
        self.reload_icons(window, cx);
    }

    /// Load the icons of the chosen set -- from the `--icon-theme` override
    /// where one is set -- and rebuild what the Icons page draws from them.
    pub(crate) fn reload_icons(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
        // gpui-builtin draws gpui-component's own icons: nothing to load.
        if self.icon_set_enum.is_some() {
            self.loaded_icons =
                load_all_icons(effective, default_theme.as_deref(), cli_ref, fg_rgb);
        }
        self.gpui_icons = load_gpui_icons(
            self.icon_set_enum,
            default_theme.as_deref(),
            cli_ref,
            fg_rgb,
        );
        let fg = cx.theme().foreground;
        self.rebuild_icon_caches(fg, window, cx);
        self.rebuild_animation_caches(window, cx);
        self.start_animation_timer(cx);
        cx.notify();
    }

    /// Build the list of icon set dropdown names.
    pub(crate) fn icon_set_dropdown_names(&self) -> Vec<SharedString> {
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

    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let preset_combobox = cx.new(|cx| {
            ComboboxState::new(
                PresetDelegate::new(),
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
                    this.current_theme_name = name.to_string();
                    this.apply_theme_by_name(name, window, cx);
                    cx.notify();
                }
            },
        )
        .detach();

        let color_mode = AppColorMode::System;

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
                    // No theme could be read, so no source stated a size.
                    defined_size: None,
                    weight: 400,
                    style: native_theme::theme::FontStyle::Normal,
                    color: native_theme::color::Rgba::TRANSPARENT,
                };
                let mono_font = native_theme::theme::ResolvedFontSpec {
                    family: "(default)".into(),
                    size: 0.0,
                    // No theme could be read, so no source stated a size.
                    defined_size: None,
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

        // Date picker state
        let date_picker_state =
            cx.new(|cx| gpui_component::date_picker::DatePickerState::new(window, cx));

        // Calendar state
        let calendar_state = cx.new(|cx| gpui_component::calendar::CalendarState::new(window, cx));

        // What the inspector shows. Created before the Data page's states:
        // their delegates build rows that report themselves to it.
        let info_ui = cx.new(|_| InfoRegistry::new());

        // Table state with sample data
        let table_state = cx.new(|cx| {
            let delegate = SampleTableDelegate {
                ui: info_ui.clone(),
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

        // Set up application menus. `cx.set_menus` hands them to the
        // platform's own menu bar; `AppMenuBar` does not read those, it
        // reads gpui-base's `GlobalState` list, which only
        // `set_app_menus` fills (`menu/app_menu_bar.rs:49-50`), so the
        // same menus go there too, before the bar is built and reads them.
        cx.set_menus(chrome::menus());
        if cx.has_global::<GlobalState>() {
            GlobalState::global_mut(cx)
                .set_app_menus(chrome::menus().into_iter().map(Menu::owned).collect());
        }
        let menu_bar = AppMenuBar::new(cx);
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        let _refocus = cx.on_focus_lost(window, |this: &mut Self, window, cx| {
            this.focus_handle.focus(window, cx)
        });

        // Start theme watcher for runtime dark/light toggle detection.
        // Skip in screenshot mode — the watcher's background thread cleanup
        // races with the Cocoa runtime on macOS CI, causing SIGTRAP on exit.
        // Skip under `cargo test` too: the watcher puts a file watch on the
        // desktop's own configuration, and the self-tests below build this
        // view for real. Nothing they assert depends on it.
        let theme_change_flag = Arc::new(AtomicBool::new(false));
        let no_watcher = cfg!(test) || std::env::args().any(|a| a == "--screenshot");
        let _theme_watcher = if no_watcher {
            None
        } else {
            let flag_clone = theme_change_flag.clone();
            native_theme::watch::on_theme_change(move |_event| {
                flag_clone.store(true, Ordering::Release);
            })
            .ok()
        };

        let inspector = {
            let (ui, showcase) = (info_ui.clone(), cx.weak_entity());
            cx.new(|cx| Inspector::new(ui, showcase, cx))
        };
        let body_layout = cx.new(|_| ResizableState::default());

        let fg = cx.theme().foreground;
        let mut showcase = Self {
            preset_combobox,
            current_theme_name: "default".into(),
            default_label: initial_default_label,
            is_dark,
            color_mode,
            original_font,
            original_mono_font,
            active_page: Page::Buttons,
            nav_collapsed: false,
            inspector_visible: true,
            body_layout,
            nav_width: NAV_WIDTH,
            inspector_width: INSPECTOR_WIDTH,
            info_ui,
            inspector,
            status_title_drawn: None,
            menu_bar,
            palette_state,
            focus_handle,
            _refocus,
            overlay_gap: Rc::new(Cell::new(None)),
            layout: initial_layout,
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
            error_message: initial_error,
            theme_change_flag,
            _theme_watcher,
            pending_system_theme_change: false,
        };
        showcase.rebuild_icon_caches(fg, window, cx);
        showcase.rebuild_animation_caches(window, cx);
        showcase.start_animation_timer(cx);
        showcase.start_theme_watcher(cx);
        showcase
    }

    fn show_theme_error(&mut self, msg: &str) {
        self.error_message = Some(msg.to_string());
    }

    /// Install the light/dark choice into whatever theme is currently up.
    ///
    /// The three paths below that cannot read a theme reach this: they leave
    /// the installed theme alone, which is right, but the user's light/dark
    /// choice still has to land. `Showcase::new` falls back to
    /// gpui-component's built-in theme when the OS read fails, and that theme
    /// has both variants; without this the selector moved `is_dark` and
    /// nothing else, and the interface stayed in the mode it started in.
    fn apply_color_mode(&self, window: &mut Window, cx: &mut Context<Self>) {
        Theme::change(gpui_theme_mode(self.is_dark), Some(window), cx);
    }

    pub(crate) fn apply_theme_by_name(
        &mut self,
        name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
                    self.apply_color_mode(window, cx);
                }
            }
        } else {
            let nt = match native_theme::theme::Theme::preset(name) {
                Ok(t) => t,
                Err(e) => {
                    self.show_theme_error(&format!("Failed to load preset '{name}': {e}"));
                    self.apply_color_mode(window, cx);
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
                    self.apply_color_mode(window, cx);
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
        // Always, whichever set is chosen: a recoloured icon takes the new
        // theme's text colour.
        self.reload_icons(window, cx);
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

    /// Read the desktop's settings again and re-install the current theme
    /// from them: what the theme watcher does when the OS theme changes, and
    /// what `ReloadTheme` asks for.
    fn reload_system_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        native_theme::detect::invalidate_caches();
        self.is_dark = self.color_mode.is_dark();
        let name = self.current_theme_name.clone();
        self.apply_theme_by_name(&name, window, cx);
    }

    /// Show `page`. Every way to a page -- the Sidebar, the View menu, the
    /// Layout page's Breadcrumb -- goes through here, so the info of what
    /// the page change takes off the screen does not stay on show (spec
    /// §4.3.4).
    pub(crate) fn show_page(&mut self, page: Page, cx: &mut Context<Self>) {
        if self.active_page != page {
            self.active_page = page;
            self.info_ui.update(cx, |r, _| r.page_changed());
        }
        cx.notify();
    }

    fn on_show_page(&mut self, action: &ShowPage, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(page) = Page::at(action.0) {
            self.show_page(page, cx);
        }
    }

    fn on_toggle_sidebar(&mut self, _: &ToggleSidebar, _: &mut Window, cx: &mut Context<Self>) {
        self.rearrange(cx, |this| this.nav_collapsed = !this.nav_collapsed);
    }

    fn on_toggle_inspector(&mut self, _: &ToggleInspector, _: &mut Window, cx: &mut Context<Self>) {
        self.rearrange(cx, |this| this.inspector_visible = !this.inspector_visible);
    }

    /// Add or take away the Sidebar's or the inspector's panel.
    ///
    /// The group's state keeps its sizes by panel position (gpui-base
    /// resizable/mod.rs, `ResizableState::sync_panels_count`), so a panel
    /// leaving the group would hand its width to its neighbour; and a panel
    /// kept but hidden (`ResizablePanel::visible`) keeps bounds that a drag of
    /// another handle still counts (`ResizableState::resize_panel_at_handle`).
    /// So the widths the two side panels have now are kept here, and a new
    /// state takes over, the panels taking those widths again as their first
    /// sizes. A new state, not the old one cleared: `ResizableState::clear`
    /// leaves the index of a handle being dragged (gpui-base
    /// resizable/mod.rs:242-245), and the next pointer move would look that
    /// panel up in a state that no longer has it (resizable/panel.rs:452). The
    /// old state stays with the listeners the last frame painted, whole.
    fn rearrange(&mut self, cx: &mut Context<Self>, change: impl FnOnce(&mut Self)) {
        let sizes = self.body_layout.read(cx).sizes().clone();
        let has_nav = !self.nav_collapsed;
        let panels = usize::from(has_nav) + 1 + usize::from(self.inspector_visible);
        if sizes.len() == panels {
            if has_nav && let Some(&width) = sizes.first() {
                self.nav_width = width;
            }
            if self.inspector_visible
                && let Some(&width) = sizes.last()
            {
                self.inspector_width = width;
            }
        }
        change(self);
        self.body_layout = cx.new(|_| ResizableState::default());
        cx.notify();
    }

    fn on_set_color_mode(
        &mut self,
        action: &SetColorMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_color_mode(action.0, window, cx);
        cx.notify();
    }

    fn on_reload_theme(&mut self, _: &ReloadTheme, window: &mut Window, cx: &mut Context<Self>) {
        self.reload_system_theme(window, cx);
        cx.notify();
    }

    fn on_set_preset(&mut self, action: &SetPreset, window: &mut Window, cx: &mut Context<Self>) {
        let key = action.0.clone();
        self.current_theme_name = key.to_string();
        self.apply_theme_by_name(&key, window, cx);
        self.preset_combobox.update(cx, |combobox, cx| {
            combobox.set_selected_values(&[key], window, cx)
        });
        cx.notify();
    }

    fn on_open_command_palette(
        &mut self,
        _: &OpenCommandPalette,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        chrome::open_command_palette(self, window, cx);
    }

    fn on_open_preferences(
        &mut self,
        _: &OpenPreferences,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        chrome::open_preferences(self, window, cx);
    }

    fn on_open_about(&mut self, _: &OpenAbout, window: &mut Window, cx: &mut Context<Self>) {
        chrome::open_about(self, window, cx);
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
            self.reload_system_theme(window, cx);
        }

        self.overlay_gap.set(geometry::widget_gap(&self.layout));
        let theme = cx.theme().clone();
        // Ensure icon image caches match the current foreground color
        if theme.foreground != self.icon_cache_fg {
            self.rebuild_icon_caches(theme.foreground, window, cx);
        }

        let active_page = self.active_page;

        // The content panel: the Alert of a theme that failed to load, then
        // the page, scrolling.
        let content = v_flex()
            .size_full()
            .overflow_hidden()
            .debug_selector(|| CONTENT_PANEL.into())
            .children(self.error_message.clone().map(|message| {
                demo::alert(&self.info_ui, cx, "content-alert", message)
                    .debug_selector(|| CONTENT_ALERT.into())
            }))
            .child(
                div()
                    .id("content-scroll-outer")
                    .flex_1()
                    .overflow_y_scrollbar()
                    // The bar is drawn over the right edge of the scroll area,
                    // so the page keeps that width free; the page roots' own
                    // padding is untouched.
                    .native(cx, geometry::scrollbar_gutter)
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
            );

        // The body: Sidebar | content | inspector, one resizable group
        // (spec §1.1). A collapsed Sidebar is an icon rail beside the group
        // rather than a panel in it: upstream's rail has a fixed width
        // (sidebar/mod.rs, `COLLAPSED_WIDTH`) that no panel size can name.
        let nav = chrome::sidebar(self, cx).into_any_element();
        let (rail, nav_panel) = if self.nav_collapsed {
            (Some(nav), None)
        } else {
            (
                None,
                Some(
                    resizable_panel()
                        .size(self.nav_width)
                        .flex_none()
                        .child(nav),
                ),
            )
        };
        let inspector_panel = self.inspector_visible.then(|| {
            resizable_panel()
                .size(self.inspector_width)
                .flex_none()
                .child(self.inspector.clone())
        });
        // The handles in panel order: each panel after the first carries the
        // one on its left edge (gpui-base resizable/panel.rs,
        // `ResizablePanel::render`).
        let handles = [
            nav_panel
                .is_some()
                .then_some((CHROME_HANDLE_NAV, "Sidebar | content")),
            inspector_panel
                .is_some()
                .then_some((CHROME_HANDLE_INSPECTOR, "content | inspector")),
        ]
        .into_iter()
        .flatten()
        .collect();
        let panels: Vec<_> = nav_panel
            .into_iter()
            .chain([resizable_panel().child(content)])
            .chain(inspector_panel)
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

        // Main layout: the title bar, the toolbar, the body and the status
        // bar, and above them the three layers `Root` keeps but does not draw.
        // `Root::render` renders only the view it was given (root.rs,
        // Root::render), so a dialog, a sheet or a notification the showcase
        // pushes reaches the screen only because these three are here --
        // upstream's own dialog test builds its host the same way
        // (dialog/dialog.rs, DialogHost).
        div()
            .relative()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_show_page))
            .on_action(cx.listener(Self::on_set_color_mode))
            .on_action(cx.listener(Self::on_reload_theme))
            .on_action(cx.listener(Self::on_toggle_sidebar))
            .on_action(cx.listener(Self::on_toggle_inspector))
            .on_action(cx.listener(Self::on_set_preset))
            .on_action(cx.listener(Self::on_open_command_palette))
            .on_action(cx.listener(Self::on_open_preferences))
            .on_action(cx.listener(Self::on_open_about))
            // First, so its prepaint opens the frame for every target
            // (info/registry.rs, epoch_marker).
            .child(epoch_marker(&self.info_ui))
            .child(
                v_flex()
                    .size_full()
                    .child(chrome::title_bar(self, cx))
                    .child(chrome::toolbar(self, cx))
                    .child(
                        h_flex()
                            .w_full()
                            .flex_1()
                            // A flex item's minimum height is its content's
                            // unless it clips; the body has to fit under the
                            // title bar, not push the window taller.
                            .overflow_hidden()
                            .children(rail)
                            .child(div().flex_1().min_w_0().h_full().child(body)),
                    )
                    .child(chrome::status_bar(self, cx, shown)),
            )
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}
