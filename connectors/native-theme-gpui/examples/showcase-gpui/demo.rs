//! Demo helpers the pages share.

use std::{cell::Cell, rc::Rc};

use gpui::{
    Action, AnyElement, App, Axis, ClickEvent, Context, Div, ElementId, Entity, Pixels, RenderOnce,
    SharedString, Stateful, StyleRefinement, Window, div, prelude::*, px,
};
use gpui_base::{ResizeHandleContext, ResizeHandleRenderer};
use gpui_component::{
    ActiveTheme, ChildElement, Collapsible, Disableable as _, Icon, IconName, Selectable, Sizable,
    Size, StyledExt as _, TitleBar, WindowExt as _,
    alert::Alert,
    attachment::{
        Attachment, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentStatus,
        AttachmentTitle,
    },
    avatar::{Avatar, AvatarGroup},
    bubble::{Bubble, BubbleVariant},
    button::{
        Button, ButtonGroup, ButtonVariants, DropdownButton, Toggle, ToggleGroup,
        ToggleVariants as _,
    },
    calendar::{Calendar, CalendarState},
    checkbox::Checkbox,
    clipboard::Clipboard,
    color_picker::{ColorPicker, ColorPickerState},
    combobox::{Combobox, ComboboxState},
    command::{Command, CommandGroup, CommandState},
    date_picker::{DatePicker, DatePickerState},
    description_list::DescriptionList,
    dialog::{Dialog, DialogDescription, DialogTitle},
    h_flex,
    input::{
        Input, InputGroup, InputGroupAddon, InputGroupAddonAlignment, InputGroupButton,
        InputGroupText, InputGroupTextarea, InputState, NumberInput, OtpInput, OtpState, Textarea,
        TextareaState,
    },
    label::Label,
    link::Link,
    list::{List, ListItem, ListState},
    menu::{AppMenuBar, PopupMenu},
    message::{Message, MessageAlignment, MessageContent},
    message_scroller::{MessageScroller, MessageScrollerState},
    pagination::Pagination,
    radio::{Radio, RadioGroup},
    rating::Rating,
    select::{SearchableVec, Select, SelectState},
    separator::Separator,
    setting::{NumberFieldOptions, SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    sheet::Sheet,
    sidebar::{Sidebar, SidebarItem, SidebarMenuItem, SidebarToggleButton},
    slider::{Slider, SliderState},
    status_bar::StatusBar,
    switch::Switch,
    tab::{Tab, TabBar},
    table::{DataTable, Table, TableBody, TableCell, TableHead, TableHeader, TableRow, TableState},
    tree::{Tree, TreeState},
    v_flex,
};
use native_theme_gpui::{AccessibilityPreferences, ActiveNativeTheme as _, geometry, variants};

use crate::app::{AppColorMode, Quit, SetColorMode, ShowPage, ToggleSidebar};
use crate::info::{self, InfoExt, InfoRegistry, WidgetInfo, native_info};
use crate::support::{
    ChatMessage, NativeStyled as _, PresetDelegate, SampleListDelegate, SampleTableDelegate,
    native_icon, native_value, section, with_gap,
};
use crate::{
    CHROME_APP_MENU_BAR, DATA_TABLE_HEADER, LIST_DEMO, OVERLAY_ABOUT_LINK, OVERLAY_ABOUT_NAME,
    OVERLAY_ABOUT_TEXT, OVERLAY_PALETTE, OVERLAY_PALETTE_TITLE, OVERLAY_PREFERENCES,
    PREF_HIGH_CONTRAST, PREF_REDUCE_MOTION, PREF_REDUCE_TRANSPARENCY, Page, STATUS_HOVERED,
    TREE_DEMO,
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
/// itself on its title and on its content, the link on its own.
pub(crate) fn about(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    dialog: Dialog,
    name_version: &'static str,
    compatibility: &'static str,
    gap: Option<Pixels>,
) -> Dialog {
    let mut dialog_info = info::about_dialog(cx.theme(), gap.is_some());
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
    if gap.is_some() {
        dialog_info = dialog_info.geometry("widget_gap");
    }
    let title = title.info(ui, "overlay-about-title", dialog_info.clone());
    let ui = ui.clone();
    dialog.title(title).content(move |content, _window, cx| {
        let link = Link::new("about-compatibility")
            .href(compatibility)
            .child(concat!(
                "the README's Compatibility table at v",
                env!("CARGO_PKG_VERSION")
            ))
            .info(&ui, "overlay-about-link", info::about_link(cx.theme()))
            .debug_selector(|| OVERLAY_ABOUT_LINK.into());
        content.child(
            with_gap(v_flex(), gap)
                .child(
                    div()
                        .debug_selector(|| OVERLAY_ABOUT_NAME.into())
                        .child(name_version),
                )
                .child(
                    div().debug_selector(|| OVERLAY_ABOUT_TEXT.into()).child(
                        DialogDescription::new()
                            .refine_style(&description_style)
                            .child("The gpui-component, gpui-base and gpui-pre versions it requires, and those it was verified against, are in")
                            .child(link),
                    ),
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

// ---------------------------------------------------------------------------
// Page text (spec §5.3)
// ---------------------------------------------------------------------------

/// A section heading reading `text`. `id` is its info's id and its debug
/// selector.
pub(crate) fn heading(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: impl Into<SharedString>,
) -> Stateful<Div> {
    section(text)
        .info(ui, id, info::text::heading(cx.theme()))
        // As wide as its text, not its column: the space beside a heading
        // is not the heading.
        .self_start()
        .debug_selector(move || id.into())
}

/// A caption reading `text`, in the muted colour. `id` is its info's id and
/// its debug selector.
pub(crate) fn caption(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: impl Into<SharedString>,
) -> Stateful<Div> {
    Label::new(text)
        .text_sm()
        .text_color(cx.theme().muted_foreground)
        .info(ui, id, info::text::caption(cx.theme()))
        .self_start()
        .debug_selector(move || id.into())
}

// ---------------------------------------------------------------------------
// The Buttons page
// ---------------------------------------------------------------------------

/// The Button variants the showcase builds, so the matches over them in
/// `info::buttons` are exhaustive and the compiler rejects a variant without
/// an arm.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ButtonKind {
    Default,
    Primary,
    Secondary,
    Danger,
    Success,
    Warning,
    Info,
    /// `native_theme_gpui::variants::ghost_button`, not upstream's `.ghost()`.
    Ghost,
    Link,
    Text,
    /// Primary, outlined.
    PrimaryOutline,
}

impl ButtonKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Primary => "Primary",
            Self::Secondary => "Secondary",
            Self::Danger => "Danger",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Info => "Info",
            Self::Ghost => "Ghost",
            Self::Link => "Link",
            Self::Text => "Text",
            Self::PrimaryOutline => "Primary, outline",
        }
    }
    /// `button` given this variant.
    fn apply(self, button: Button, cx: &App) -> Button {
        match self {
            Self::Default => button,
            Self::Primary => button.primary(),
            Self::Secondary => button.secondary(),
            Self::Danger => button.danger(),
            Self::Success => button.success(),
            Self::Warning => button.warning(),
            // By path: `InfoExt::info` is in scope too.
            Self::Info => ButtonVariants::info(button),
            Self::Ghost => button.custom(variants::ghost_button(cx)),
            Self::Link => button.link(),
            Self::Text => button.text(),
            Self::PrimaryOutline => button.primary().outline(),
        }
    }
}

/// What a Button of the Buttons page is doing.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ButtonState {
    Idle,
    Disabled,
    Loading,
}

/// One Button of the Buttons page. `id` is its info's id and its debug
/// selector.
pub(crate) struct DemoButton {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: ButtonKind,
    pub state: ButtonState,
    pub icon: Option<IconName>,
}

/// A Button refined by `geometry::button`.
pub(crate) fn button(ui: &Entity<InfoRegistry>, cx: &App, spec: DemoButton) -> Stateful<Div> {
    let DemoButton {
        id,
        label,
        kind,
        state,
        icon,
    } = spec;
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut button_info =
        info::buttons::button(cx.theme(), kind, state, icon.is_some(), None, styled);
    let button = native_info(
        kind.apply(Button::new(id), cx),
        cx,
        geometry::button,
        "button",
        &mut button_info,
    )
    .label(label)
    .when_some(icon, |button, icon| button.icon(icon))
    .disabled(state == ButtonState::Disabled)
    .loading(state == ButtonState::Loading);
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into())
}

/// A Default Button at `size`, left without `geometry::button`: this row
/// shows upstream's own size scale, which the refinement would overrule
/// (button/button.rs:626-641, then :690).
pub(crate) fn sized_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    size: Size,
) -> Stateful<Div> {
    let button = Button::new(id).label(label).with_size(size);
    InfoExt::info(
        button,
        ui,
        id,
        info::buttons::button(
            cx.theme(),
            ButtonKind::Default,
            ButtonState::Idle,
            false,
            Some(size),
            false,
        ),
    )
}

/// A `ButtonGroup` of Default Buttons reading `labels`, without
/// `geometry::button`: the group joins its Buttons' corners and edges itself
/// (button/button_group.rs:182-229), which a refinement's radius would undo.
pub(crate) fn button_group(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
) -> Stateful<Div> {
    let group =
        ButtonGroup::new(id).children(labels.iter().map(|&label| Button::new(label).label(label)));
    InfoExt::info(group, ui, id, info::buttons::button_group(cx.theme()))
}

/// A `DropdownButton` whose Button, of `kind`, reads `label`, opening
/// `menu`. Neither half takes `geometry::button`: the DropdownButton joins
/// their corners itself (button/dropdown_button.rs:174-207).
pub(crate) fn dropdown_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    kind: ButtonKind,
    menu: impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static,
) -> Stateful<Div> {
    let dropdown = DropdownButton::new(id)
        .button(kind.apply(Button::new("button"), cx).label(label))
        .dropdown_menu(menu);
    InfoExt::info(
        dropdown,
        ui,
        id,
        info::buttons::dropdown_button(cx.theme(), kind),
    )
}

/// A `Toggle` showing `icon`, named `icon_name` in its info, `checked` or
/// not; a click hands `on_click` the state it asks for.
pub(crate) fn toggle(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    icon: IconName,
    icon_name: &'static str,
    checked: bool,
    on_click: impl Fn(&bool, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    Toggle::new(id)
        .icon(icon)
        .checked(checked)
        .on_click(on_click)
        .info(
            ui,
            id,
            info::buttons::toggle(cx.theme(), icon_name, checked),
        )
}

/// A `ToggleGroup` of unchecked Toggles reading `labels`.
pub(crate) fn toggle_group(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
) -> Stateful<Div> {
    ToggleGroup::new(id)
        .children(labels.iter().map(|&label| Toggle::new(label).label(label)))
        .info(ui, id, info::buttons::toggle_group(cx.theme()))
}

/// A `Clipboard` copying `value`.
pub(crate) fn clipboard(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    value: &'static str,
) -> Stateful<Div> {
    Clipboard::new(id)
        .value(value)
        .info(ui, id, info::buttons::clipboard(cx.theme(), value))
}

// ---------------------------------------------------------------------------
// The Inputs page
// ---------------------------------------------------------------------------

/// Which refinement a single-line `Input` of the Inputs page takes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputField {
    /// `geometry::input`, the whole refinement.
    Refined,
    /// `geometry::input_height` alone, as the field's height.
    HeightOnly,
}

/// A single-line `Input` over `state`, `width` wide, taking the refinement
/// `field` names.
pub(crate) fn text_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<InputState>,
    field: InputField,
    width: Pixels,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut input_info = info::inputs::input(cx.theme(), field, styled);
    let input = Input::new(state).with_size(Size::Medium).w(width);
    let input = match field {
        InputField::Refined => native_info(input, cx, geometry::input, "input", &mut input_info),
        // Through the caller's style, which Input applies after its own
        // height (input/input.rs:703, then :719). `Input::h` would not do:
        // upstream reads it for a multi-line input only (:706-709).
        InputField::HeightOnly => match native_value(cx, geometry::input_height) {
            Some(height) => {
                input_info = input_info.geometry("input_height");
                Styled::h(input, height)
            }
            None => input,
        },
    };
    input
        .info(ui, id, input_info)
        .debug_selector(move || id.into())
}

/// A `Textarea` over `state`, `width` by `height`, refined by
/// `geometry::input`: it renders as an `Input` (input/textarea.rs:164).
pub(crate) fn textarea(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TextareaState>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let mut textarea_info = info::inputs::textarea(cx.theme());
    let textarea = native_info(
        Textarea::new(state).h(height).w(width),
        cx,
        geometry::input,
        "input",
        &mut textarea_info,
    );
    textarea.info(ui, id, textarea_info)
}

/// The states of the three `InputGroup`s.
pub(crate) struct InputGroupStates<'a> {
    /// The field behind the Search icon.
    pub search: &'a Entity<InputState>,
    /// The field the Copy button copies.
    pub copy: &'a Entity<InputState>,
    /// The textarea above the note.
    pub notes: &'a Entity<TextareaState>,
}

/// Three `InputGroup`s, `width` wide, one above the other: a field behind a
/// Search icon, a field with a Copy button after it, which hands `on_copy`
/// its click, and a textarea with a note under it. They report as one.
pub(crate) fn input_groups(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    states: InputGroupStates<'_>,
    width: Pixels,
    on_copy: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let mut groups_info = info::inputs::input_groups(cx.theme());
    // One refinement, recorded once, for both single-line groups; the
    // textarea group keeps upstream's frame.
    let frame = native_info(
        StyleRefinement::default(),
        cx,
        geometry::input,
        "input",
        &mut groups_info,
    );
    // `InputGroupButton::new` is a ghost button that upstream repaints,
    // inside a group, with a hover of `theme.muted` (`input/group.rs:544-583`):
    // a grey that under Breeze is barely distinguishable from the field,
    // while every button around it hovers blue. A custom variant makes
    // upstream skip that repaint.
    let copy = native_info(
        InputGroupButton::new("input-group-copy"),
        cx,
        geometry::input_group_button,
        "input_group_button",
        &mut groups_info,
    )
    .custom(variants::ghost_button(cx))
    .icon(IconName::Copy)
    .label("Copy")
    .tooltip("Copy the field to the clipboard")
    .on_click(on_copy);
    v_flex()
        .gap_3()
        .w(width)
        .child(
            InputGroup::new("input-group-inline")
                .input(Input::new(states.search))
                .addon(
                    InputGroupAddon::new("input-group-inline-addon")
                        .child(Icon::new(IconName::Search)),
                )
                .refine_style(&frame),
        )
        .child(
            InputGroup::new("input-group-trailing")
                .input(Input::new(states.copy))
                .addon(
                    InputGroupAddon::new("input-group-trailing-addon")
                        .align(InputGroupAddonAlignment::InlineEnd)
                        .child(copy),
                )
                .refine_style(&frame),
        )
        .child(
            InputGroup::new("input-group-textarea")
                .input(InputGroupTextarea::new(states.notes))
                .addon(
                    InputGroupAddon::new("input-group-textarea-addon")
                        .align(InputGroupAddonAlignment::BlockEnd)
                        .child(InputGroupText::new().child("Markdown supported")),
                ),
        )
        .info(ui, id, groups_info)
}

/// A `NumberInput` over `state`, `width` wide, refined by `geometry::input`.
pub(crate) fn number_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<InputState>,
    width: Pixels,
) -> Stateful<Div> {
    let mut number_info = info::inputs::number_input(cx.theme());
    let number = native_info(
        NumberInput::new(state),
        cx,
        geometry::input,
        "input",
        &mut number_info,
    )
    .placeholder("Enter a number")
    .with_size(Size::Medium)
    .w(width);
    number.info(ui, id, number_info)
}

/// A `Checkbox` reading `label`, `checked` or not, refined by
/// `geometry::checkbox`. A disabled one takes no `on_click`.
pub(crate) fn checkbox(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    checked: bool,
    on_click: Option<impl Fn(&bool, &mut Window, &mut App) + 'static>,
) -> Stateful<Div> {
    let disabled = on_click.is_none();
    let mut checkbox_info = info::inputs::checkbox(cx.theme(), label, checked, disabled);
    let checkbox = native_info(
        Checkbox::new(id),
        cx,
        geometry::checkbox,
        "checkbox",
        &mut checkbox_info,
    )
    .label(label)
    .checked(checked)
    .disabled(disabled)
    .when_some(on_click, |checkbox, on_click| checkbox.on_click(on_click));
    checkbox
        .info(ui, id, checkbox_info)
        .debug_selector(move || id.into())
}

/// A horizontal `RadioGroup` of Radios reading `labels`, each refined by
/// `geometry::radio`, the one at `selected` selected; a click hands
/// `on_click` the index of the Radio clicked.
///
/// `RadioGroup::child` takes `impl Into<Radio>`, so a `&str` child would
/// build a `Radio` with no refinement; built here instead, each row carries
/// the platform's label gap and font. The group overwrites the id
/// (`radio.rs:406`), not the style.
pub(crate) fn radio_group(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
    selected: Option<usize>,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let mut group_info = info::inputs::radio_group(cx.theme(), labels, selected);
    // One refinement, recorded once, for every Radio of the group.
    let row = native_info(
        StyleRefinement::default(),
        cx,
        geometry::radio,
        "radio",
        &mut group_info,
    );
    RadioGroup::horizontal(id)
        .children(
            labels
                .iter()
                .map(|&label| Radio::new(label).refine_style(&row).label(label)),
        )
        .selected_index(selected)
        .on_click(on_click)
        .info(ui, id, group_info)
}

/// A `Switch` reading `label`, `checked` or not. A disabled one takes no
/// `on_click`.
pub(crate) fn switch(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    checked: bool,
    on_click: Option<impl Fn(&bool, &mut Window, &mut App) + 'static>,
) -> Stateful<Div> {
    let disabled = on_click.is_none();
    Switch::new(id)
        .label(label)
        .checked(checked)
        .disabled(disabled)
        .when_some(on_click, |switch, on_click| switch.on_click(on_click))
        .info(
            ui,
            id,
            info::inputs::switch(cx.theme(), label, checked, disabled),
        )
}

/// A `Slider` over `state`, `width` wide.
pub(crate) fn slider(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<SliderState>,
    width: Pixels,
) -> Stateful<Div> {
    Slider::new(state)
        .w(width)
        .info(ui, id, info::inputs::slider(cx.theme()))
}

/// A `Rating` at `value`, its stars at `geometry::icon_size_small`: they are
/// inline icons, and `Rating` has no geometry builder of its own. A disabled
/// one takes no `on_click`.
pub(crate) fn rating(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    value: usize,
    on_click: Option<impl Fn(&usize, &mut Window, &mut App) + 'static>,
) -> Stateful<Div> {
    let disabled = on_click.is_none();
    let mut rating_info = info::inputs::rating(cx.theme(), value, disabled);
    let rating = Rating::new(id)
        .value(value)
        .disabled(disabled)
        .when_some(on_click, |rating, on_click| rating.on_click(on_click));
    let rating = match native_value(cx, geometry::icon_size_small) {
        Some(size) => {
            rating_info = rating_info.geometry("icon_size_small");
            rating.with_size(size)
        }
        None => rating,
    };
    rating.info(ui, id, rating_info)
}

/// An `OtpInput` over `state`, its boxes in `groups` groups. It is not
/// `Styled`, so it takes no refinement.
pub(crate) fn otp_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<OtpState>,
    groups: usize,
) -> Stateful<Div> {
    OtpInput::new(state)
        .groups(groups)
        .info(ui, id, info::inputs::otp_input(cx.theme()))
}

/// A `Select` over `state`, `width` wide, reading `placeholder` until a
/// choice is made, refined by `geometry::select`.
pub(crate) fn select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
    placeholder: &'static str,
    width: Pixels,
) -> Stateful<Div> {
    let mut select_info = info::inputs::select(cx.theme());
    let select = native_info(
        Select::new(state).placeholder(placeholder).w(width),
        cx,
        geometry::select,
        "select",
        &mut select_info,
    );
    select.info(ui, id, select_info)
}

/// A `ColorPicker` over `state`, reading `label`.
pub(crate) fn color_picker(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<ColorPickerState>,
    label: &'static str,
) -> Stateful<Div> {
    ColorPicker::new(state)
        .label(label)
        .info(ui, id, info::inputs::color_picker(cx.theme()))
}

/// A `DatePicker` over `state`, reading `placeholder` until a date is
/// picked.
pub(crate) fn date_picker(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<DatePickerState>,
    placeholder: &'static str,
) -> Stateful<Div> {
    DatePicker::new(state).placeholder(placeholder).info(
        ui,
        id,
        info::inputs::date_picker(cx.theme()),
    )
}

/// A `Calendar` over `state`.
pub(crate) fn calendar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<CalendarState>,
) -> Stateful<Div> {
    Calendar::new(state).info(ui, id, info::inputs::calendar(cx.theme()))
}

// ---------------------------------------------------------------------------
// The Data page
// ---------------------------------------------------------------------------

/// A `DescriptionList` of `items`, as (label, value, span), in `columns`
/// columns.
pub(crate) fn description_list(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    columns: usize,
    items: &[(&'static str, &'static str, usize)],
) -> Stateful<Div> {
    items
        .iter()
        .fold(
            DescriptionList::new().columns(columns),
            |list, &(label, value, span)| list.item(label, value, span),
        )
        .info(
            ui,
            id,
            info::data::description_list(cx.theme(), items.len(), columns),
        )
}

/// The striped, bordered `DataTable` over `state`, `height` tall. Its rows
/// report themselves through the delegate, which builds them with
/// `data_table_header` and `data_table_row`.
pub(crate) fn data_table(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TableState<SampleTableDelegate>>,
    height: Pixels,
) -> Stateful<Div> {
    let (rows, columns) = {
        let delegate = state.read(cx).delegate();
        (delegate.rows.len(), delegate.columns.len())
    };
    DataTable::new(state)
        .stripe(true)
        .bordered(true)
        .info(ui, id, info::data::data_table(cx.theme(), rows, columns))
        .h(height)
        // gpui's scroll listeners run in the bubble phase and stop at no one,
        // so without this the page under the table scrolls by the same delta
        // (div.rs, paint_scroll_listener; window.rs, HitboxBehavior::BlockMouse).
        .occlude()
}

/// What a row of the Data page's `DataTable` is, as the table paints it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DataTableRow {
    /// One of the delegate's rows.
    Body(BodyRow),
    /// A row the table draws below the data to fill its height.
    Filler { striped: bool },
}

/// The state of one of the `DataTable`'s own rows, as the table's events
/// reported it to the delegate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BodyRow {
    /// An odd row of a striped table (table/state.rs:1959).
    pub striped: bool,
    /// The last of the delegate's rows.
    pub last: bool,
    /// The table's selected row.
    pub selected: bool,
    /// The table is selecting rows, so it paints its selected row as one
    /// (table/state.rs:2194); a column selection keeps the selected row but
    /// not its fill.
    pub selection_shown: bool,
    pub right_clicked: bool,
}

/// The `DataTable`'s header row, reading `columns`, as `render_header` hands
/// it to the table: an empty row the table fills with the header cells.
pub(crate) fn data_table_header(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    columns: &str,
) -> Stateful<Div> {
    div()
        .info(
            ui,
            DATA_TABLE_HEADER,
            info::data::data_table_header(cx.theme(), columns),
        )
        .debug_selector(|| DATA_TABLE_HEADER.into())
}

/// The `DataTable`'s row `ix`, reading `cells` where it is one of the
/// delegate's rows, as `render_tr` hands it to the table: an empty row the
/// table fills with its cells.
pub(crate) fn data_table_row(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    ix: usize,
    row: DataTableRow,
    cells: Option<String>,
) -> Stateful<Div> {
    let id = format!("data-table-row-{ix}");
    div()
        .info(
            ui,
            SharedString::from(id.clone()),
            info::data::data_table_row(cx.theme(), row, cells),
        )
        .debug_selector(move || id)
}

/// A cell of the `DataTable`, reading `text`.
pub(crate) fn data_table_cell(text: SharedString) -> Label {
    Label::new(text).text_sm()
}

/// A child of a declarative `Table` wrapped in its info: the `Table`, its
/// header and its body take their children as `ChildElement`s, so the
/// wrapper passes on the index and the size the parent gives it.
#[derive(IntoElement)]
pub(crate) struct Reported<E: ChildElement + 'static> {
    inner: E,
    ui: Entity<InfoRegistry>,
    id: SharedString,
    info: WidgetInfo,
}

impl<E: ChildElement + 'static> Reported<E> {
    fn new(
        inner: E,
        ui: &Entity<InfoRegistry>,
        id: impl Into<SharedString>,
        info: WidgetInfo,
    ) -> Self {
        Self {
            inner,
            ui: ui.clone(),
            id: id.into(),
            info,
        }
    }
}

impl<E: ChildElement + 'static> Sizable for Reported<E> {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.inner = self.inner.with_size(size);
        self
    }
}

impl<E: ChildElement + 'static> ChildElement for Reported<E> {
    fn with_ix(mut self, ix: usize) -> Self {
        self.inner = self.inner.with_ix(ix);
        self
    }
}

impl<E: ChildElement + 'static> RenderOnce for Reported<E> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let id = self.id.clone();
        self.inner
            .info(&self.ui, self.id, self.info)
            .w_full()
            .debug_selector(move || id.to_string())
    }
}

/// A declarative `Table` named `label`, refined by `geometry::table`: a
/// header reading `head` above a body row for each of `rows`. The header
/// and every body row report themselves.
pub(crate) fn table(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    head: [&'static str; 2],
    rows: &[[&'static str; 2]],
) -> Stateful<Div> {
    let t = cx.theme();
    let mut table_info = info::data::table(t, rows.len());
    let table = native_info(Table::new(), cx, geometry::table, "table", &mut table_info)
        .accessibility_label(label);
    let header = Reported::new(
        TableHeader::new().child(TableRow::new().children(head.map(|h| TableHead::new().child(h)))),
        ui,
        format!("{id}-header"),
        info::data::table_header(t, &head.join(", ")),
    );
    let body = rows
        .iter()
        .enumerate()
        .fold(TableBody::new(), |body, (ix, cells)| {
            body.child(Reported::new(
                TableRow::new().children(cells.map(|c| TableCell::new().child(c))),
                ui,
                format!("{id}-row-{ix}"),
                info::data::table_row(t, ix == 0, &cells.join(", ")),
            ))
        });
    table.child(header).child(body).info(ui, id, table_info)
}

/// One `Pagination` of the Data page. `id` is its info's id and its debug
/// selector.
pub(crate) struct DemoPagination {
    pub id: &'static str,
    pub page: usize,
    pub pages: usize,
    pub compact: bool,
    /// The gap `geometry::widget_gap` gives, where the Pagination takes it.
    pub gap: Option<Pixels>,
}

/// A `Pagination` on `page` of `pages`, `compact` or not, `gap` apart where
/// it is given one; a click hands `on_click` the page asked for.
pub(crate) fn pagination(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: DemoPagination,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let DemoPagination {
        id,
        page,
        pages,
        compact,
        gap,
    } = spec;
    let mut pagination_info =
        info::data::pagination(cx.theme(), compact, page, pages, gap.is_some());
    let pagination = Pagination::new(id)
        .current_page(page)
        .total_pages(pages)
        .on_click(on_click);
    let pagination = if compact {
        pagination.compact()
    } else {
        pagination
    };
    let pagination = match gap {
        Some(gap) => {
            pagination_info = pagination_info.geometry("widget_gap");
            pagination.gap(gap)
        }
        None => pagination,
    };
    pagination
        .info(ui, id, pagination_info)
        .debug_selector(move || id.into())
}

/// A box `width` by `height`, refined by `geometry::list`, around the List
/// over `state`. A List paints no frame of its own (list/list.rs,
/// `RenderOnce for List`), so the box is its frame; the rows report
/// themselves (`ListRow`).
pub(crate) fn list(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<ListState<SampleListDelegate>>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let mut list_info = info::data::list(state.read(cx).delegate().items.len());
    native_info(
        div().w(width).h(height),
        cx,
        geometry::list,
        "list",
        &mut list_info,
    )
    .child(List::new(state))
    .info(ui, id, list_info)
    .self_start()
    // gpui's scroll listeners run in the bubble phase and stop at no one, so
    // without this the page under the List scrolls by the same delta (div.rs,
    // paint_scroll_listener; window.rs, HitboxBehavior::BlockMouse). Every
    // demo box that holds a scroller of its own carries it.
    .occlude()
    .debug_selector(|| LIST_DEMO.into())
}

/// What a `ListItem` row is marked as.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ListRowState {
    Idle,
    Selected,
    /// Right-clicked, selected or not: it paints the same either way
    /// (list/list_item.rs:235-242).
    RightClicked,
}

impl ListRowState {
    fn of(selected: bool, right_clicked: bool) -> Self {
        match (selected, right_clicked) {
            (_, true) => Self::RightClicked,
            (true, false) => Self::Selected,
            (false, false) => Self::Idle,
        }
    }
}

/// Row `ix` of the Data page's List, reading `label`: a `ListItem` refined
/// by `geometry::list_item`.
///
/// The List marks a row selected or right-clicked after the delegate built
/// it (list/list.rs, `ListState::render_list_item`), so the row builds its
/// `ListItem` and its info only as it renders, from what it was marked.
#[derive(IntoElement)]
pub(crate) struct ListRow {
    ui: Entity<InfoRegistry>,
    ix: usize,
    label: SharedString,
    selected: bool,
    right_clicked: bool,
}

impl ListRow {
    pub(crate) fn new(ui: &Entity<InfoRegistry>, ix: usize, label: SharedString) -> Self {
        Self {
            ui: ui.clone(),
            ix,
            label,
            selected: false,
            right_clicked: false,
        }
    }
}

impl Selectable for ListRow {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    fn is_selected(&self) -> bool {
        self.selected
    }
    fn secondary_selected(mut self, selected: bool) -> Self {
        self.right_clicked = selected;
        self
    }
}

impl RenderOnce for ListRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = format!("data-list-row-{}", self.ix);
        let state = ListRowState::of(self.selected, self.right_clicked);
        let mut row_info = info::data::list_row(cx.theme(), &self.label, state);
        let item = native_info(
            ListItem::new(SharedString::from(id.clone())),
            cx,
            geometry::list_item,
            "list_item",
            &mut row_info,
        )
        .child(Label::new(self.label).text_sm())
        .selected(self.selected)
        .secondary_selected(self.right_clicked);
        item.info(&self.ui, SharedString::from(id.clone()), row_info)
            .w_full()
            .debug_selector(move || id)
    }
}

/// A box `width` by `height`, refined by `geometry::list`, around the Tree
/// over `state`. A tree is a list view and the model gives it no theme of
/// its own, so its frame is the list's; the rows report themselves
/// (`tree_row`).
pub(crate) fn tree(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TreeState>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let mut tree_info = info::data::tree();
    let rows = ui.clone();
    native_info(
        div().w(width).h(height),
        cx,
        geometry::list,
        "list",
        &mut tree_info,
    )
    .child(Tree::new(state, move |ix, entry, selected, _window, cx| {
        tree_row(&rows, cx, ix, entry.item().label.clone(), selected)
    }))
    .info(ui, id, tree_info)
    .self_start()
    // As the List's box: the Tree scrolls on its own.
    .occlude()
    .debug_selector(|| TREE_DEMO.into())
}

/// Row `ix` of the Tree, reading `label`, `selected` or not: a `ListItem`
/// refined by `geometry::list_item`.
///
/// `Tree::new` takes a closure that returns the `ListItem` itself and wraps
/// it in a box of its own (tree.rs:41-44, 87-93), so nothing can be put
/// around the row. Its info target is its suffix instead: `ListItem` puts
/// the suffix straight into its root, which is `relative()`
/// (list/list_item.rs:183, 233), so an absolute box there covers the
/// whole row. It takes no pointer from the row: an ordinary hitbox blocks
/// nothing under it.
fn tree_row(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    ix: usize,
    label: SharedString,
    selected: bool,
) -> ListItem {
    let id = format!("data-tree-row-{ix}");
    let mut row_info = info::data::tree_row(cx.theme(), &label, selected);
    let item = native_info(
        ListItem::new(SharedString::from(id.clone())),
        cx,
        geometry::list_item,
        "list_item",
        &mut row_info,
    );
    let ui = ui.clone();
    item.child(Label::new(label).text_sm())
        .selected(selected)
        .suffix(move |_window, _cx| {
            let id = id.clone();
            div()
                .info(&ui, SharedString::from(id.clone()), row_info.clone())
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .debug_selector(move || id)
        })
}

/// An `Avatar` named `name`.
pub(crate) fn avatar(
    ui: &Entity<InfoRegistry>,
    id: impl Into<SharedString>,
    name: impl Into<SharedString>,
) -> Stateful<Div> {
    let name = name.into();
    let avatar_info = info::data::avatar(&name);
    Avatar::new().name(name).info(ui, id.into(), avatar_info)
}

/// An `AvatarGroup` of Avatars named `names`, showing `limit` of them.
pub(crate) fn avatar_group(
    ui: &Entity<InfoRegistry>,
    id: &'static str,
    names: &[&'static str],
    limit: usize,
) -> Stateful<Div> {
    names
        .iter()
        .fold(AvatarGroup::new(), |group, &name| {
            group.child(Avatar::new().name(name))
        })
        .limit(limit)
        .info(ui, id, info::data::avatar_group(names, limit))
}

/// The Bubble variants the Data page builds, so the match over them in
/// `info::data::bubble` is exhaustive and the compiler rejects a variant
/// without an arm.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum BubbleKind {
    Filled,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

impl BubbleKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Filled => "Filled",
            Self::Secondary => "Secondary",
            Self::Muted => "Muted",
            Self::Tinted => "Tinted",
            Self::Outline => "Outline",
            Self::Ghost => "Ghost",
            Self::Destructive => "Destructive",
        }
    }
    fn variant(self) -> BubbleVariant {
        match self {
            Self::Filled => BubbleVariant::Filled,
            Self::Secondary => BubbleVariant::Secondary,
            Self::Muted => BubbleVariant::Muted,
            Self::Tinted => BubbleVariant::Tinted,
            Self::Outline => BubbleVariant::Outline,
            Self::Ghost => BubbleVariant::Ghost,
            Self::Destructive => BubbleVariant::Destructive,
        }
    }
}

/// A `Bubble` of `kind` reading `text`, at the end of its row where
/// `outgoing`, at the start otherwise.
pub(crate) fn bubble(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: BubbleKind,
    outgoing: bool,
    text: &'static str,
) -> Stateful<Div> {
    let alignment = if outgoing {
        MessageAlignment::End
    } else {
        MessageAlignment::Start
    };
    Bubble::new()
        .alignment(alignment)
        .with_variant(kind.variant())
        .child(text)
        .info(ui, id, info::data::bubble(cx.theme(), kind, outgoing))
}

/// A row of the chat thread, built the same way for the Message section and
/// for every row the `MessageScroller` renders: the sender's Avatar beside a
/// bubble whose variant and alignment say which side sent it. The Avatar
/// reports itself as `{id}-avatar`; the Bubble, which `MessageContent::bubble`
/// takes as it is, reports through the row.
pub(crate) fn message(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: impl Into<SharedString>,
    msg: &ChatMessage,
) -> Stateful<Div> {
    let id = id.into();
    let (alignment, variant) = if msg.outgoing {
        (MessageAlignment::End, BubbleVariant::Filled)
    } else {
        (MessageAlignment::Start, BubbleVariant::Muted)
    };
    Message::new()
        .alignment(alignment)
        .avatar(avatar(ui, format!("{id}-avatar"), msg.sender.clone()))
        .content(
            MessageContent::new()
                .bubble(Bubble::new().with_variant(variant).child(msg.text.clone())),
        )
        .info(
            ui,
            id.clone(),
            info::data::message(cx.theme(), msg.outgoing, &msg.sender, &msg.text),
        )
        .debug_selector(move || id.to_string())
}

/// The `MessageScroller` over `messages`, following `state`, in the
/// showcase's frame `height` tall, its bottom faded into the window's
/// background.
pub(crate) fn message_scroller(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<MessageScrollerState>,
    messages: &[ChatMessage],
    height: Pixels,
) -> Stateful<Div> {
    let t = cx.theme();
    // The data stays with the caller; the state owns only the virtual
    // list's bookkeeping (message_scroller.rs:23-25).
    let rows = messages.to_vec();
    let row_ui = ui.clone();
    let scroller = MessageScroller::new(id, state.clone(), move |ix, _window, cx| {
        match rows.get(ix) {
            Some(msg) => {
                message(&row_ui, cx, format!("data-scroller-message-{ix}"), msg).into_any_element()
            }
            None => div().into_any_element(),
        }
    })
    .with_bottom_fade(t.background)
    .size_full();
    div()
        .h(height)
        .demo_frame(cx)
        .child(scroller)
        .info(ui, id, info::data::message_scroller(t, messages.len()))
        // As the List's box: the thread scrolls on its own.
        .occlude()
}

/// The Default Button reading `label`, refined by `geometry::button`, that
/// hands `on_click` its click. `about` is its info's "click" line.
pub(crate) fn action_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    about: &'static str,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut button_info = info::buttons::button(
        cx.theme(),
        ButtonKind::Default,
        ButtonState::Idle,
        false,
        None,
        styled,
    )
    .instance("click", about);
    let button = native_info(
        Button::new(id),
        cx,
        geometry::button,
        "button",
        &mut button_info,
    )
    .label(label)
    .on_click(on_click);
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into())
}

/// One `Attachment` card of the Data page: `file`, described by
/// `description`, in `status`, its media `icon`. `id` is its info's id and
/// its debug selector.
pub(crate) struct DemoAttachment {
    pub id: &'static str,
    pub status: AttachmentStatus,
    pub icon: IconName,
    pub file: &'static str,
    pub description: SharedString,
}

/// An `Attachment` card, its media icon at `geometry::icon_size_small`. A
/// card given `on_click` is clickable as a whole.
pub(crate) fn attachment(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: DemoAttachment,
    on_click: Option<impl Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
) -> Stateful<Div> {
    let DemoAttachment {
        id,
        status,
        icon,
        file,
        description,
    } = spec;
    let mut card_info = info::data::attachment(cx.theme(), status, file, on_click.is_some());
    if native_value(cx, geometry::icon_size_small).is_some() {
        card_info = card_info.geometry("icon_size_small");
    }
    let card = Attachment::new()
        .status(status)
        .media(AttachmentMedia::new().child(native_icon(cx, icon, geometry::icon_size_small)))
        .content(
            AttachmentContent::new()
                .title(AttachmentTitle::new(file).status(status))
                .description(AttachmentDescription::new(description).status(status)),
        );
    // The whole card is the click target, so the status it is in is the
    // status a click advances.
    let card = match on_click {
        Some(on_click) => card.id(id).on_click(on_click),
        None => card,
    };
    card.info(ui, id, card_info)
        .debug_selector(move || id.into())
}
