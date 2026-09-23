//! Demo helpers the pages share.

use std::{cell::Cell, rc::Rc};

use gpui::{
    Action, AnyElement, App, Axis, Div, ElementId, Entity, Pixels, SharedString, Stateful,
    StyleRefinement, Window, div, prelude::*, px,
};
use gpui_base::{ResizeHandleContext, ResizeHandleRenderer};
use gpui_component::{
    ActiveTheme, Collapsible, Disableable as _, IconName, Sizable as _, Size, StyledExt as _,
    TitleBar, WindowExt as _,
    alert::Alert,
    button::{Button, ButtonVariants as _, Toggle, ToggleGroup, ToggleVariants as _},
    combobox::{Combobox, ComboboxState},
    command::{Command, CommandGroup, CommandState},
    dialog::{Dialog, DialogDescription, DialogTitle},
    h_flex,
    link::Link,
    menu::AppMenuBar,
    select::{SearchableVec, Select, SelectState},
    separator::Separator,
    setting::{NumberFieldOptions, SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    sheet::Sheet,
    sidebar::{Sidebar, SidebarItem, SidebarMenuItem, SidebarToggleButton},
    status_bar::StatusBar,
    switch::Switch,
    tab::{Tab, TabBar},
    v_flex,
};
use native_theme_gpui::{AccessibilityPreferences, ActiveNativeTheme as _, geometry};

use crate::app::{AppColorMode, Quit, SetColorMode, ShowPage, ToggleSidebar};
use crate::info::{self, InfoExt, InfoRegistry, WidgetInfo, native_info};
use crate::support::{PresetDelegate, native_icon, native_value, with_gap};
use crate::{
    CHROME_APP_MENU_BAR, OVERLAY_ABOUT_LINK, OVERLAY_PALETTE, OVERLAY_PALETTE_TITLE,
    OVERLAY_PREFERENCES, PREF_HIGH_CONTRAST, PREF_REDUCE_MOTION, PREF_REDUCE_TRANSPARENCY, Page,
    STATUS_HOVERED,
};

/// A `TitleBar` refined by `geometry::title_bar`, reading `label`, holding
/// `app_menu_bar` where the platform has no menu bar of its own, and quitting
/// the application from its close button.
pub(crate) fn title_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    label: impl Into<SharedString>,
    app_menu_bar: Entity<AppMenuBar>,
) -> Stateful<Div> {
    let mut bar_info = info::title_bar(cx.theme());
    let bar = native_info(
        TitleBar::new(),
        cx,
        geometry::title_bar,
        "title_bar",
        &mut bar_info,
    )
    // Linux only: upstream drops the handler on every other platform
    // (title_bar.rs:95-104). On Windows the OS acts on the X itself; macOS
    // draws its own controls.
    .on_close_window(|_, window, cx| window.dispatch_action(Box::new(Quit), cx))
    // Plain text, not a Label: `Label::render` sets `foreground` on its own
    // element (label.rs:211), which would hide the title-bar font's colour
    // `geometry::title_bar` just gave the bar.
    .child(label.into())
    .when(cfg!(not(target_os = "macos")), |bar| {
        bar.child(
            app_menu_bar
                .info(ui, "chrome-app-menu-bar", info::app_menu_bar(cx.theme()))
                .debug_selector(|| CHROME_APP_MENU_BAR.into()),
        )
    });
    bar.info(ui, "chrome-title-bar", bar_info)
}

/// The window's toolbar (spec §2.3): the application's own row, refined by
/// `geometry::toolbar`, holding `items`.
pub(crate) fn toolbar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    items: impl IntoIterator<Item = AnyElement>,
) -> Stateful<Div> {
    let mut row_info = info::toolbar();
    let row =
        native_info(h_flex(), cx, geometry::toolbar, "toolbar", &mut row_info).children(items);
    row.info(ui, "chrome-toolbar", row_info)
}

/// The window's `StatusBar` (spec §2.7), refined by `geometry::status_bar`:
/// `environment` on the left; on the right `shown`, the title of what the
/// inspector shows, where it shows one, then `version`.
pub(crate) fn status_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    environment: impl Into<SharedString>,
    shown: Option<SharedString>,
    version: impl Into<SharedString>,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut bar_info = info::status_bar(cx.theme(), styled);
    let bar = native_info(
        StatusBar::new(),
        cx,
        geometry::status_bar,
        "status_bar",
        &mut bar_info,
    )
    // Plain text, not Labels, as in the title bar: a Label would paint
    // foreground over the colour `geometry::status_bar` gives the bar.
    .left(environment.into())
    .when_some(shown, |bar, title| {
        bar.right(div().debug_selector(|| STATUS_HOVERED.into()).child(title))
    })
    .right(version.into());
    bar.info(ui, "chrome-status-bar", bar_info)
}

/// The preset switch: a searchable `Combobox` over `state`'s presets,
/// refined by `geometry::combobox`.
pub(crate) fn preset_combobox(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    state: &Entity<ComboboxState<PresetDelegate>>,
) -> Stateful<Div> {
    let mut combobox_info = info::preset_combobox(cx.theme());
    let combobox = native_info(
        Combobox::new(state)
            .placeholder("Pick a preset…")
            .search_placeholder("Filter by name or key…"),
        cx,
        geometry::combobox,
        "combobox",
        &mut combobox_info,
    );
    combobox.info(ui, "chrome-toolbar-preset", combobox_info)
}

/// The colour modes, in the order the switch shows them.
const COLOR_MODES: [AppColorMode; 3] = [
    AppColorMode::System,
    AppColorMode::Light,
    AppColorMode::Dark,
];

/// A System / Light / Dark `ToggleGroup` with `mode` checked; a click
/// dispatches `SetColorMode` for the toggle clicked.
pub(crate) fn color_mode_toggle_group(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    mode: AppColorMode,
) -> Stateful<Div> {
    let checked = COLOR_MODES.map(|m| m == mode);
    let group = ToggleGroup::new("color-mode")
        .outline()
        .segmented()
        .children(COLOR_MODES.map(|m| {
            Toggle::new(SharedString::from(format!("color-mode-{m:?}")))
                .label(m.label())
                .checked(m == mode)
        }))
        // Upstream reports every toggle's state with the clicked one flipped
        // (button/toggle.rs, ToggleGroup::render), so the toggle clicked is
        // the one whose state differs from what was drawn.
        .on_click(move |next: &Vec<bool>, window, cx| {
            let clicked = next
                .iter()
                .zip(checked)
                .position(|(now, was)| *now != was)
                .and_then(|ix| COLOR_MODES.get(ix));
            if let Some(&m) = clicked {
                window.dispatch_action(Box::new(SetColorMode(m)), cx);
            }
        });
    group.info(
        ui,
        "chrome-toolbar-color-mode",
        info::color_mode_toggle_group(cx.theme()),
    )
}

/// The icon-set `Select` over `state`, refined by `geometry::select`.
pub(crate) fn icon_set_select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
) -> Stateful<Div> {
    let mut select_info = info::icon_set_select(cx.theme());
    let select = native_info(
        Select::new(state),
        cx,
        geometry::select,
        "select",
        &mut select_info,
    );
    select.info(ui, "chrome-toolbar-icon-set", select_info)
}

/// A vertical `Separator` between toolbar items.
pub(crate) fn toolbar_separator(ui: &Entity<InfoRegistry>, cx: &App) -> Stateful<Div> {
    Separator::vertical()
        .info(
            ui,
            "chrome-toolbar-separator",
            info::toolbar_separator(cx.theme()),
        )
        // A vertical Separator is as tall as its parent (separator.rs,
        // Separator::vertical), so the wrapper takes the row's height.
        .self_stretch()
}

/// One of the toolbar's icon Buttons.
pub(crate) struct ToolbarButton<'a> {
    pub button_id: &'static str,
    pub icon: IconName,
    /// The tooltip's text; the action's key binding follows it.
    pub tooltip: &'static str,
    pub action: &'a dyn Action,
    /// The info's "action" line: what the button is for.
    pub about: &'static str,
}

/// A ghost icon `Button` for the toolbar, dispatching its action, its icon
/// at `geometry::icon_size_toolbar` and its tooltip showing the action's key
/// binding.
///
/// The icon is the Button's child, not its `icon`: `Button::icon` resizes
/// whatever it is given to a size derived from the Button's own
/// (button/button.rs:580-583, 713-717), and the size an `Icon` is given last
/// wins over its style (icon.rs:181-187).
pub(crate) fn toolbar_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: ToolbarButton<'_>,
) -> Stateful<Div> {
    let ToolbarButton {
        button_id: id,
        icon,
        tooltip,
        action,
        about,
    } = spec;
    let mut button_info = info::toolbar_button(cx.theme(), about);
    if native_value(cx, geometry::icon_size_toolbar).is_some() {
        button_info = button_info.geometry("icon_size_toolbar");
    }
    let dispatched = action.boxed_clone();
    let button = Button::new(id)
        .ghost()
        .tooltip_with_action(tooltip, action, None)
        .child(native_icon(cx, icon, geometry::icon_size_toolbar))
        .on_click(move |_, window, cx| window.dispatch_action(dispatched.boxed_clone(), cx));
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(
        button,
        ui,
        SharedString::from(format!("chrome-{id}")),
        button_info,
    )
}

/// The toolbar's `SidebarToggleButton`, drawn `collapsed` while the Sidebar
/// is; a click dispatches `ToggleSidebar`.
pub(crate) fn sidebar_toggle_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    collapsed: bool,
) -> Stateful<Div> {
    SidebarToggleButton::new()
        .collapsed(collapsed)
        .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleSidebar), cx))
        .info(
            ui,
            "chrome-sidebar-toggle",
            info::sidebar_toggle_button(cx.theme(), collapsed),
        )
}

/// The window's `Sidebar` (spec §2.4): one item per page, the item of
/// `active` marked active, collapsed to icons while `collapsed`.
///
/// Its width is its container's: a width that is not an absolute pixel
/// length keeps upstream from animating it to a width of its own
/// (sidebar/mod.rs, `sidebar_expanded_width`), so expanded it fills the
/// resizable panel it is in, and collapsed it is upstream's icon rail.
pub(crate) fn sidebar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    active: Page,
    collapsed: bool,
) -> Stateful<Div> {
    let items = Page::ALL.map(|page| NavItem {
        ui: ui.clone(),
        page,
        active: page == active,
        collapsed: false,
    });
    Sidebar::new("chrome-sidebar")
        .collapsed(collapsed)
        .w_full()
        .children(items)
        .info(ui, "chrome-sidebar", info::sidebar(cx.theme(), collapsed))
        .h_full()
        .when(!collapsed, |sidebar| sidebar.w_full())
}

/// A page's item in the window's Sidebar.
///
/// A `Sidebar` renders its items itself, as it lays out its list
/// (sidebar/mod.rs, `RenderOnce for Sidebar`), so an item's info cannot be
/// wrapped around it from outside; this item builds its `SidebarMenuItem`
/// and wraps the info around what that renders.
#[derive(Clone)]
pub(crate) struct NavItem {
    ui: Entity<InfoRegistry>,
    page: Page,
    active: bool,
    collapsed: bool,
}

impl Collapsible for NavItem {
    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

impl SidebarItem for NavItem {
    fn render(
        self,
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let page = self.page;
        let mut item_info =
            info::sidebar_item(cx.theme(), page.label(), self.active, self.collapsed);
        // A sidebar is the panel `defaults.icon_sizes.panel` names, and
        // `SidebarMenuItem` keeps the icon it is given (sidebar/menu.rs:300).
        if native_value(cx, geometry::icon_size_panel).is_some() {
            item_info = item_info.geometry("icon_size_panel");
        }
        let item = SidebarMenuItem::new(page.label())
            .icon(native_icon(cx, page.icon(), geometry::icon_size_panel))
            .active(self.active)
            .collapsed(self.collapsed)
            .on_click(move |_, window, cx| {
                window.dispatch_action(Box::new(ShowPage(page.index())), cx)
            });
        SidebarItem::render(item, id, window, cx)
            .info(&self.ui, page.nav_item(), item_info)
            .w_full()
            .debug_selector(move || page.nav_item().into())
    }
}

/// The inspector's `TabBar` (spec §2.6): Underline at `Size::Small`, `labels`
/// its tabs, `selected` the one shown; a click hands `on_click` the index of
/// the tab clicked.
pub(crate) fn tab_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    labels: impl IntoIterator<Item = &'static str>,
    selected: usize,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    TabBar::new("inspector-tabs")
        .underline()
        .with_size(Size::Small)
        .children(labels.into_iter().map(|label| Tab::new().label(label)))
        .selected_index(selected)
        .on_click(on_click)
        .info(
            ui,
            "chrome-inspector-tabs",
            info::inspector_tab_bar(cx.theme()),
        )
}

/// gpui-base's `HANDLE_PADDING` (resizable/resize_handle.rs:11): the padding
/// a resize handle puts on each side of its line. It is `pub(crate)`
/// upstream, so the showcase names it again to cover the handle's hit area.
const HANDLE_PADDING: Pixels = px(4.);

/// gpui-base's `HANDLE_SIZE` (resizable/resize_handle.rs:12): the width a
/// resize handle is given and its line is drawn at, `pub(crate)` upstream as
/// well.
const HANDLE_SIZE: Pixels = px(1.);

/// What each handle of a horizontal resizable group paints: upstream's line
/// in upstream's colour, with an info target over the handle's whole hit
/// area.
///
/// That area is not the line and `HANDLE_PADDING` on either side. The handle
/// is `HANDLE_SIZE` wide with `HANDLE_PADDING` on each side, and layout
/// widens a box to at least its padding (taffy 0.13 compute/flexbox.rs:
/// 2294-2295), so the handle spans `HANDLE_PADDING` either side of the
/// panel boundary: 4px left of the line, which starts at the boundary, and
/// 3px right of it.
///
/// `handles` names the group's handles in order, as `(id, between)`. The
/// renderer is not told which handle it draws (resizable/resize_handle.rs,
/// `ResizeHandleContext`), and the group lays its handles out in panel
/// order, once each per frame, so the renderer counts them; build it anew
/// for every frame. A handle past the end of `handles` keeps upstream's own
/// line. The target is the handle's child, so a press on it is still the
/// handle's and starts its drag.
pub(crate) fn resize_handles(
    ui: &Entity<InfoRegistry>,
    handles: Vec<(&'static str, &'static str)>,
) -> ResizeHandleRenderer {
    let ui = ui.clone();
    let next = Cell::new(0usize);
    Rc::new(
        move |handle: &ResizeHandleContext, _window: &mut Window, cx: &mut App| {
            // One call per handle, in panel order, every frame: each panel
            // after the first builds the handle on its left edge as it
            // renders (resizable/panel.rs:363-368), and each handle calls
            // this once as it lays itself out (resizable/resize_handle.rs:
            // 194-203). A panel given `visible(false)` returns before it
            // builds its handle (resizable/panel.rs:302) and would shift the
            // count, so an absent panel is left out of the group instead.
            let ix = next.get();
            next.set(ix + 1);
            if handle.axis() != Axis::Horizontal {
                return None;
            }
            let (id, between) = handles.get(ix).copied()?;
            let base = gpui_base::Theme::global(cx);
            // gpui-base's `handle_color` (resizable/resize_handle.rs:286-295).
            let line = if handle.is_active() {
                base.resizable
                    .active_handle
                    .unwrap_or(base.tokens.colors.ring)
            } else {
                base.resizable.handle.unwrap_or(base.tokens.colors.border)
            };
            let target = div()
                .size_full()
                .info(&ui, id, info::resize_handle(&base, between))
                .absolute()
                .top_0()
                .bottom_0()
                .left(-HANDLE_PADDING)
                .right(-(HANDLE_PADDING - HANDLE_SIZE))
                .debug_selector(move || id.into());
            Some(
                div()
                    .flex_none()
                    .relative()
                    .h_full()
                    .w(HANDLE_SIZE)
                    .bg(line)
                    .child(target)
                    .into_any_element(),
            )
        },
    )
}

// ---------------------------------------------------------------------------
// Overlays (spec §2.8) and the theme-error Alert (spec §2.5)
// ---------------------------------------------------------------------------

/// `dialog` refined by `geometry::dialog` and capped at
/// `geometry::dialog_max_width`, each recorded in `info` where it applies.
fn dialog_frame(dialog: Dialog, cx: &App, info: &mut WidgetInfo) -> Dialog {
    let dialog = native_info(dialog, cx, geometry::dialog, "dialog", info);
    match native_value(cx, geometry::dialog_max_width) {
        Some(width) => {
            *info = std::mem::take(info).geometry("dialog_max_width");
            dialog.max_w(width)
        }
        None => dialog,
    }
}

/// A `DialogTitle` reading `text`, refined by `geometry::dialog_title`,
/// recorded in `info`.
fn dialog_title(cx: &App, text: &'static str, info: &mut WidgetInfo) -> DialogTitle {
    native_info(
        DialogTitle::new().child(text),
        cx,
        geometry::dialog_title,
        "dialog_title",
        info,
    )
}

/// The command palette (spec §2.8): `dialog`, titled, holding an unbordered
/// `Command` over `state` with `groups`. Running an entry dispatches its
/// action, then closes the palette.
///
/// The Dialog reports itself on its title: the Command fills the rest of
/// its content, and the surface around them is upstream's, out of reach.
pub(crate) fn command_palette(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    dialog: Dialog,
    state: &Entity<CommandState>,
    groups: Vec<CommandGroup>,
) -> Dialog {
    let mut dialog_info = info::palette_dialog(cx.theme());
    let dialog = dialog_frame(dialog, cx, &mut dialog_info);
    let title = dialog_title(cx, "Command Palette", &mut dialog_info)
        .info(ui, "overlay-palette-dialog", dialog_info)
        .debug_selector(|| OVERLAY_PALETTE_TITLE.into());
    let (ui, state) = (ui.clone(), state.clone());
    dialog.title(title).content(move |content, _window, cx| {
        let command = Command::new(&state)
            .bordered(false)
            .placeholder("A page, a preset or a colour mode…")
            .on_confirm(|_, window, cx| window.close_dialog(cx));
        let command = groups.iter().cloned().fold(command, Command::group);
        content.child(
            command
                .info(&ui, "overlay-palette", info::command_palette(cx.theme()))
                .debug_selector(|| OVERLAY_PALETTE.into()),
        )
    })
}

/// The About dialog (spec §2.8): `dialog`, titled, stating `name_version`
/// and linking to `compatibility`, its lines `gap` apart. The Dialog reports
/// itself on its content, the link on its own.
pub(crate) fn about(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    dialog: Dialog,
    name_version: &'static str,
    compatibility: &'static str,
    gap: Option<Pixels>,
) -> Dialog {
    let mut dialog_info = info::about_dialog(cx.theme());
    let dialog = dialog_frame(dialog, cx, &mut dialog_info);
    let title = dialog_title(cx, "About", &mut dialog_info);
    // A `DialogDescription` is built anew for every frame, so what the
    // refinement records is recorded once, here, and applied there.
    let description_style = native_info(
        StyleRefinement::default(),
        cx,
        geometry::dialog_description,
        "dialog_description",
        &mut dialog_info,
    );
    let ui = ui.clone();
    dialog.title(title).content(move |content, _window, cx| {
        let link = Link::new("about-compatibility")
            .href(compatibility)
            .child("the README's Compatibility table")
            .info(&ui, "overlay-about-link", info::about_link(cx.theme()))
            .debug_selector(|| OVERLAY_ABOUT_LINK.into());
        content.child(
            with_gap(v_flex(), gap)
                .child(name_version)
                .child(
                    DialogDescription::new()
                        .refine_style(&description_style)
                        .child("The gpui-component, gpui-base and gpui-pre versions it requires, and those it was verified against, are in")
                        .child(link),
                )
                .info(&ui, "overlay-about", dialog_info.clone()),
        )
    })
}

/// The preferences a switch flips.
#[derive(Clone, Copy)]
enum Preference {
    ReduceMotion,
    HighContrast,
    ReduceTransparency,
}

impl Preference {
    fn get(self, prefs: &AccessibilityPreferences) -> bool {
        match self {
            Self::ReduceMotion => prefs.reduce_motion,
            Self::HighContrast => prefs.high_contrast,
            Self::ReduceTransparency => prefs.reduce_transparency,
        }
    }
    fn set(self, prefs: &mut AccessibilityPreferences, on: bool) {
        match self {
            Self::ReduceMotion => prefs.reduce_motion = on,
            Self::HighContrast => prefs.high_contrast = on,
            Self::ReduceTransparency => prefs.reduce_transparency = on,
        }
    }
    /// The field's name, which its info names.
    fn field(self) -> &'static str {
        match self {
            Self::ReduceMotion => "reduce_motion",
            Self::HighContrast => "high_contrast",
            Self::ReduceTransparency => "reduce_transparency",
        }
    }
    /// The id and debug selector of the preference's switch.
    fn selector(self) -> &'static str {
        match self {
            Self::ReduceMotion => PREF_REDUCE_MOTION,
            Self::HighContrast => PREF_HIGH_CONTRAST,
            Self::ReduceTransparency => PREF_REDUCE_TRANSPARENCY,
        }
    }
}

/// The accessibility preferences the installed native theme carries; `None`
/// before one is installed.
fn installed_preferences(cx: &App) -> Option<AccessibilityPreferences> {
    cx.native_theme().map(|nt| nt.accessibility().clone())
}

/// Install `prefs` with `change` made, where a native theme is installed.
fn change_preferences(cx: &mut App, change: impl FnOnce(&mut AccessibilityPreferences)) {
    if let Some(mut prefs) = installed_preferences(cx) {
        change(&mut prefs);
        native_theme_gpui::apply_accessibility(&prefs, cx);
    }
}

/// The smallest and largest text scale the Preferences sheet offers:
/// Windows' `UISettings.TextScaleFactor` range (platform-facts §1.2.7), the
/// one range the platform facts state. The model states none.
const TEXT_SCALE_MIN: f64 = 1.0;
const TEXT_SCALE_MAX: f64 = 2.25;
/// The step the text scale field moves by: the showcase's own choice, which
/// divides the range above evenly. The model states no step.
const TEXT_SCALE_STEP: f64 = 0.25;

/// A row whose field is one of the showcase's own Switches, built as
/// upstream's switch field builds one (setting/fields/bool.rs, `BoolField`)
/// but wrapped in its info, which upstream's field has no room for.
fn preference_item(
    ui: &Entity<InfoRegistry>,
    title: &'static str,
    description: &'static str,
    pref: Preference,
    disabled: bool,
) -> SettingItem {
    let ui = ui.clone();
    let field = SettingField::render(move |options, _window, cx| {
        let checked = installed_preferences(cx).is_some_and(|p| pref.get(&p));
        let selector = pref.selector();
        Switch::new(selector)
            .checked(checked)
            .disabled(options.is_disabled())
            .with_size(options.size())
            .on_click(move |on: &bool, _window, cx| {
                change_preferences(cx, |prefs| pref.set(prefs, *on));
            })
            .info(
                &ui,
                selector,
                info::preference_switch(cx.theme(), pref.field(), checked, options.is_disabled()),
            )
            .debug_selector(move || selector.into())
    });
    SettingItem::new(title, field)
        .description(description)
        .disabled(disabled)
}

/// The Preferences sheet (spec §2.8): `sheet`, `width` wide, holding a
/// Settings page with the four accessibility preferences. A change goes
/// through `native_theme_gpui::apply_accessibility`. Without a native theme
/// there is nothing to re-install, and the rows are disabled.
///
/// The Sheet reports itself on its title, the one part of it the showcase
/// builds; the Settings below report theirs.
pub(crate) fn preferences(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    sheet: Sheet,
    width: Pixels,
) -> Sheet {
    let disabled = cx.native_theme().is_none();
    let mut settings_info = info::preferences_settings(cx.theme());
    let text_scale = SettingItem::new(
        "Text scale",
        SettingField::number_input(
            NumberFieldOptions {
                min: TEXT_SCALE_MIN,
                max: TEXT_SCALE_MAX,
                step: TEXT_SCALE_STEP,
            },
            |cx| {
                f64::from(
                    installed_preferences(cx)
                        .unwrap_or_default()
                        .text_scaling_factor,
                )
            },
            |scale, cx| change_preferences(cx, |prefs| prefs.text_scaling_factor = scale as f32),
        ),
    )
    .description("text_scaling_factor: the theme's font sizes are multiplied by it")
    .disabled(disabled);
    let group = native_info(
        SettingGroup::new(),
        cx,
        geometry::scrollbar_gutter,
        "scrollbar_gutter",
        &mut settings_info,
    )
    .item(text_scale)
    .item(preference_item(
        ui,
        "Reduce motion",
        "reduce_motion: forwarded to gpui's own reduce-motion flag",
        Preference::ReduceMotion,
        disabled,
    ))
    .item(preference_item(
        ui,
        "High contrast",
        "high_contrast: stored with the theme, which the connector builds no differently for it",
        Preference::HighContrast,
        disabled,
    ))
    .item(preference_item(
        ui,
        "Reduce transparency",
        "reduce_transparency: the backdrop behind a dialog or a sheet is made opaque",
        Preference::ReduceTransparency,
        disabled,
    ));
    let settings = Settings::new("preferences").page(
        SettingPage::new("Accessibility")
            .description("Installed with native_theme_gpui::apply_accessibility")
            .resettable(false)
            .default_open(true)
            .group(group),
    );
    sheet
        .title(div().child("Preferences").info(
            ui,
            "overlay-preferences-sheet",
            info::preferences_sheet(cx.theme()),
        ))
        .size(width)
        .child(
            settings
                .info(ui, "overlay-preferences", settings_info)
                .size_full()
                .debug_selector(|| OVERLAY_PREFERENCES.into()),
        )
}

/// An error `Alert` across the top of the content (spec §2.5), reading
/// `message`.
pub(crate) fn alert(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    message: impl Into<SharedString>,
) -> Stateful<Div> {
    Alert::error(id, message.into())
        .banner()
        .info(ui, id, info::theme_error_alert(cx.theme()))
        .w_full()
}
