//! Demo helpers the pages share.

use std::{cell::Cell, rc::Rc, time::Duration};

use gpui::{
    Action, AnyElement, App, Axis, ClickEvent, ClipboardItem, Context, Div, ElementId, Entity,
    FontWeight, Hsla, ImageSource, Keystroke, Pixels, Rems, RenderOnce, SharedString, Stateful,
    StyleRefinement, Window, div, prelude::*, px, rems,
};
use gpui_base::{ResizeHandleContext, ResizeHandleRenderer};
use gpui_component::{
    ActiveTheme, ChildElement, Collapsible, Disableable as _, Icon, IconName, Selectable, Sizable,
    Size, StyledExt as _, TitleBar, WindowExt as _,
    accordion::Accordion,
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
    calendar::{Calendar, CalendarState},
    carousel::{
        Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPagination,
        CarouselPaginationItem, CarouselPrevious, CarouselState,
    },
    chart::{AreaChart, BarChart, CandlestickChart, LineChart, PieChart},
    checkbox::Checkbox,
    clipboard::Clipboard,
    color_picker::{ColorPicker, ColorPickerState},
    combobox::{Combobox, ComboboxState},
    command::{Command, CommandGroup, CommandItem, CommandState},
    date_picker::{DatePicker, DatePickerState},
    description_list::DescriptionList,
    dialog::{
        AlertDialog, Dialog, DialogButtonProps, DialogClose, DialogDescription, DialogFooter,
        DialogTitle,
    },
    empty::{
        Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant,
        EmptyTitle,
    },
    form::{Field, Form},
    group_box::{GroupBox, GroupBoxVariant, GroupBoxVariants as _},
    h_flex,
    hover_card::HoverCard,
    input::{
        Editor, EditorState, Input, InputGroup, InputGroupAddon, InputGroupAddonAlignment,
        InputGroupButton, InputGroupText, InputGroupTextarea, InputState, NumberInput, OtpInput,
        OtpState, Textarea, TextareaState,
    },
    kbd::Kbd,
    label::Label,
    link::Link,
    list::{List, ListItem, ListState},
    marker::{Marker, MarkerContent, MarkerIcon, MarkerLoadingStyle, MarkerVariant},
    menu::{AppMenuBar, ContextMenuExt as _, DropdownMenu as _, PopupMenu},
    message::{Message, MessageAlignment, MessageContent},
    message_scroller::{MessageScroller, MessageScrollerState},
    notification::Notification,
    pagination::Pagination,
    popover::Popover,
    progress::{Progress, ProgressCircle},
    radio::{Radio, RadioGroup},
    rating::Rating,
    scroll::ScrollableElement as _,
    select::{SearchableVec, Select, SelectState},
    separator::Separator,
    setting::{NumberFieldOptions, SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    sheet::Sheet,
    shimmer::ShimmerText,
    sidebar::{Sidebar, SidebarItem, SidebarMenuItem},
    skeleton::Skeleton,
    slider::{Slider, SliderState},
    spinner::Spinner,
    status_bar::StatusBar,
    stepper::{Stepper, StepperItem},
    switch::Switch,
    tab::{Tab, TabBar},
    table::{DataTable, Table, TableBody, TableCell, TableHead, TableHeader, TableRow, TableState},
    tag::{Tag, TagVariant},
    text::TextView,
    tooltip::Tooltip,
    tree::{Tree, TreeState},
    v_flex,
};
use native_theme_gpui::icons::with_spin_animation;
use native_theme_gpui::{
    AccessibilityPreferences, ActiveNativeTheme as _, Native, geometry, variants,
};

use crate::app::{Quit, ShowPage};
use crate::info::{self, InfoExt, InfoRegistry, WidgetInfo, hsla_to_hex, native_info};
use crate::support::{
    CAROUSEL_SLIDES, ChatMessage, ChromeIcon, NativeStyled as _, PresetDelegate, STEPPER_STEPS,
    SampleListDelegate, SampleTableDelegate, native_geometry, native_value, refined, with_gap,
    with_padding,
};
use crate::{
    CHROME_APP_MENU_BAR, CHROME_SIDE_PANEL, CHROME_THEME_SETTINGS, DATA_TABLE_HEADER, LIST_DEMO,
    OVERLAY_ABOUT_LINK, OVERLAY_ABOUT_NAME, OVERLAY_ABOUT_TEXT, OVERLAY_PALETTE,
    OVERLAY_PALETTE_TITLE, OVERLAY_PREFERENCES, OVERLAYS_DIALOG_CLOSE, OVERLAYS_DIALOG_FOOTER,
    PREF_HIGH_CONTRAST, PREF_REDUCE_MOTION, PREF_REDUCE_TRANSPARENCY, PROBE_CAROUSEL_LAST,
    PROBE_SETTINGS_ROW, Page, STATUS_ENVIRONMENT, STATUS_HOVERED, STATUS_MIDDLE, TREE_DEMO, probe,
};

/// An icon at the platform's size for the role the builder names; upstream's
/// own size before `apply` ran. A part: the helper that places it reports.
fn native_icon(cx: &App, name: IconName, role: fn(Native<'_>) -> Size) -> Icon {
    native_sized(cx, Icon::new(name), role)
}

/// The `Icon` `drawn` is: gpui-component's `icon` where the built-in set is
/// chosen, the chosen set's SVG otherwise, and `None` where that set has
/// none. An SVG is drawn as every `Icon` is, as a mask in the colour its
/// widget gives it (gpui-pre window.rs, `Window::paint_svg`). A part: the
/// helper that places it reports.
fn chrome_icon(drawn: &ChromeIcon, icon: &IconName) -> Option<Icon> {
    match drawn {
        ChromeIcon::Builtin(_) => Some(Icon::new(icon.clone())),
        ChromeIcon::Loaded(_, bytes) => Some(Icon::default().data(bytes)),
        ChromeIcon::Missing(_) | ChromeIcon::Unlisted(_) => None,
    }
}

/// `icon` at the platform's size for the role the builder names; its own
/// size before `apply` ran.
fn native_sized(cx: &App, icon: Icon, role: fn(Native<'_>) -> Size) -> Icon {
    match native_value(cx, role) {
        Some(size) => icon.with_size(size),
        None => icon,
    }
}

/// A `TitleBar` refined by `geometry::title_bar`, reading `label`, holding
/// `app_menu_bar` where the platform has no menu bar of its own, and quitting
/// the application from its close button: the window's title bar, where the
/// window was granted client-side decorations (spec S8).
pub(crate) fn title_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    label: impl Into<SharedString>,
    app_menu_bar: Entity<AppMenuBar>,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    let mut bar_info = info::title_bar(cx.theme(), &label);
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
    .child(label)
    .when(cfg!(not(target_os = "macos")), |bar| {
        bar.child(reported_menus(
            ui,
            cx,
            app_menu_bar,
            info::MenuHost::TitleBar,
        ))
    });
    bar.info(ui, "chrome-title-bar", bar_info)
}

/// `app_menu_bar`, reporting itself as the menus of `host`. A part: the
/// helper that places it reports its host.
fn reported_menus(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    app_menu_bar: Entity<AppMenuBar>,
    host: info::MenuHost,
) -> Stateful<Div> {
    app_menu_bar
        .info(
            ui,
            "chrome-app-menu-bar",
            info::app_menu_bar(cx.theme(), host),
        )
        .debug_selector(|| CHROME_APP_MENU_BAR.into())
}

/// The menu-bar row's side padding where the theme states no
/// `layout.container_margin` (spec §3.1). The model states no menu bar -- its
/// menu is the popup a menu opens (platform-facts §2.6) -- so the row is the
/// showcase's own element, with no toolkit default to fall back on, and this
/// is the showcase's own choice, not a platform's value; the row's info says
/// so.
pub(crate) const MENU_BAR_PADDING: Pixels = px(8.);

/// The menu-bar row (spec S8): at the top of a window whose frame the window
/// manager draws, the application's own row holding `app_menu_bar`, as a KDE
/// application places its menus. The model states no menu-bar inset, so its
/// sides borrow `container_margin`, the installed layout's
/// `geometry::container_margin`, and where that is unstated take
/// [`MENU_BAR_PADDING`]; the AppMenuBar's items set its height.
pub(crate) fn menu_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    container_margin: Option<Pixels>,
    app_menu_bar: Entity<AppMenuBar>,
) -> Stateful<Div> {
    // No geometry line: the model states no menu-bar inset, and the row only
    // borrows container_margin, which its info says in its own words.
    let row_info = info::menu_bar(cx.theme(), container_margin, MENU_BAR_PADDING);
    h_flex()
        .px(container_margin.unwrap_or(MENU_BAR_PADDING))
        .child(reported_menus(ui, cx, app_menu_bar, info::MenuHost::Row))
        .info(ui, "chrome-menu-bar", row_info)
}

/// A `TitleBar` sample, refined by `geometry::title_bar` and reading `label`:
/// the window's own title bar as it is drawn where the window manager leaves
/// the frame to the application (spec S8), shown while it does not.
///
/// A TitleBar acts on the window it is in: a drag moves it and a double
/// click zooms it (title_bar.rs, RenderOnce for TitleBar), and on Windows the
/// OS hit-tests its controls as the window's own (title_bar.rs,
/// ControlIcon::render). So a box laid over the sample takes the pointer,
/// which leaves every hitbox under it out of the hit test (gpui-pre
/// window.rs, `HitboxBehavior::BlockMouse`), and the box carries the bar's
/// info.
pub(crate) fn title_bar_sample(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: impl Into<SharedString>,
) -> Div {
    let label: SharedString = label.into();
    let mut bar_info = info::title_bar_sample(cx.theme(), &label);
    let bar = native_info(
        TitleBar::new(),
        cx,
        geometry::title_bar,
        "title_bar",
        &mut bar_info,
    )
    // Plain text, as in the window's own title bar.
    .child(label);
    div().relative().child(bar).child(
        div()
            .info(ui, id, bar_info)
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude(),
    )
}

/// The toolbar row's padding on a side the theme leaves unstated twice over:
/// neither `toolbar.border` nor `layout.container_margin` states it (spec
/// §3.1). The row is the showcase's own element, with no toolkit default to
/// fall back on, so this is the showcase's own choice, not a platform's
/// value, and the row's info says so.
pub(crate) const TOOLBAR_PADDING: Pixels = px(8.);

/// The window's toolbar (spec §2.3): the application's own row, refined by
/// `geometry::toolbar`, holding `items`.
///
/// A side `toolbar.border` leaves unstated is padded with `container_margin`,
/// the installed layout's `geometry::container_margin`, and where that is
/// unstated too with [`TOOLBAR_PADDING`] (spec §3.1). That padding goes on
/// first, so the refinement's stated sides land over it.
pub(crate) fn toolbar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    container_margin: Option<Pixels>,
    items: impl IntoIterator<Item = AnyElement>,
) -> Stateful<Div> {
    let stated = native_value(cx, |n| n.resolved.toolbar.border.padding).unwrap_or_default();
    let mut row_info = info::toolbar(stated, container_margin, TOOLBAR_PADDING);
    let unstated = [stated.top, stated.right, stated.bottom, stated.left]
        .iter()
        .any(Option::is_none);
    if unstated && container_margin.is_some() {
        row_info = row_info.geometry("container_margin");
    }
    let row = h_flex().p(container_margin.unwrap_or(TOOLBAR_PADDING));
    let row = native_info(row, cx, geometry::toolbar, "toolbar", &mut row_info).children(items);
    row.info(ui, "chrome-toolbar", row_info)
}

/// The window's `StatusBar` (spec §2.7, §3.2), refined by
/// `geometry::status_bar`: on the left `toggle`, then `environment`; on the
/// right `shown`, the title of what the inspector shows, where it shows one.
pub(crate) fn status_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    toggle: impl IntoElement,
    environment: impl Into<SharedString>,
    shown: Option<SharedString>,
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
    .left(toggle)
    // Plain text, not Labels, as in the title bar: a Label would paint
    // foreground over the colour `geometry::status_bar` gives the bar.
    .left(
        div()
            .debug_selector(|| STATUS_ENVIRONMENT.into())
            .child(environment.into()),
    )
    // The middle region holds nothing; this empty box fills it, so its
    // edges are where the two ends stop.
    .child(div().flex_1().debug_selector(|| STATUS_MIDDLE.into()))
    .when_some(shown, |bar, title| {
        bar.right(div().debug_selector(|| STATUS_HOVERED.into()).child(title))
    });
    bar.info(ui, "chrome-status-bar", bar_info)
}

/// The preset switch: a searchable `Combobox` over `state`'s presets,
/// refined by `geometry::combobox`, as wide as the theme settings it is in.
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
    )
    .w_full();
    combobox.info(ui, "chrome-settings-preset", combobox_info)
}

/// The colour-mode `Select` over `state`, refined by `geometry::select`, as
/// wide as the theme settings it is in: System, Light and Dark, and the
/// choice dispatches `SetColorMode` (`Showcase::new`).
pub(crate) fn color_mode_select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
) -> Stateful<Div> {
    let mut select_info = info::color_mode_select(cx.theme());
    let select = native_info(
        Select::new(state),
        cx,
        geometry::select,
        "select",
        &mut select_info,
    )
    .w_full();
    select.info(ui, "chrome-settings-color-mode", select_info)
}

/// The icon-theme `Select` over `state`, refined by `geometry::select`, as
/// wide as the theme settings it is in.
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
    )
    .w_full();
    select.info(ui, "chrome-settings-icon-theme", select_info)
}

/// One of the toolbar's icon Buttons.
pub(crate) struct ToolbarButton<'a> {
    pub button_id: &'static str,
    /// gpui-component's icon of the button, which `drawn` is of.
    pub icon: IconName,
    /// That icon as the chosen icon theme gives it.
    pub drawn: ChromeIcon,
    /// The chosen icon theme, as the Icons page names it.
    pub set: &'a str,
    /// The tooltip's text; the action's key binding follows it. The button's
    /// label where the chosen set has no icon for it.
    pub tooltip: &'static str,
    pub action: &'a dyn Action,
    /// The info's "action" line: what the button is for.
    pub about: &'static str,
}

/// A Ghost `Button` for the toolbar -- the Buttons page's, built with
/// `variants::ghost_button` -- dispatching its action, its icon of the
/// chosen icon theme at `geometry::icon_size_toolbar` and its tooltip showing
/// the action's key binding. Where the icon theme has no icon for it, it
/// shows its tooltip's text instead, never another icon theme's icon.
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
        drawn,
        set,
        tooltip,
        action,
        about,
    } = spec;
    let mut button_info = info::toolbar_button(cx.theme(), about, &drawn, set);
    let icon =
        chrome_icon(&drawn, &icon).map(|icon| native_sized(cx, icon, geometry::icon_size_toolbar));
    if icon.is_some() && native_value(cx, geometry::icon_size_toolbar).is_some() {
        button_info = button_info.geometry("icon_size_toolbar");
    }
    let dispatched = action.boxed_clone();
    let button = ButtonKind::Ghost
        .apply(Button::new(id), cx)
        .tooltip_with_action(tooltip, action, None)
        .map(|button| match icon {
            Some(icon) => button.child(icon),
            None => button.label(tooltip),
        })
        .on_click(move |_, window, cx| window.dispatch_action(dispatched.boxed_clone(), cx));
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(
        button,
        ui,
        SharedString::from(format!("chrome-button-{id}")),
        button_info,
    )
}

/// The status bar's panel toggle (spec §3.2, S4).
pub(crate) struct PanelToggle<'a> {
    pub button_id: &'static str,
    /// gpui-component's icon of the toggle, which `drawn` is of.
    pub icon: IconName,
    /// That icon as the chosen icon theme gives it.
    pub drawn: ChromeIcon,
    /// The chosen icon theme, as the Icons page names it.
    pub set: &'a str,
    /// The tooltip's text; the action's key binding follows it. The toggle's
    /// label where the chosen set has no icon for it.
    pub tooltip: &'static str,
    pub action: &'a dyn Action,
    /// Whether the panel it toggles is open, which shows it selected.
    pub open: bool,
    /// The info's "action" line: what the toggle does.
    pub about: &'static str,
    /// The info's "state" line: the panel's state, which the selection shows.
    pub state: &'static str,
}

/// A small Ghost `Button` for the status bar -- built with
/// `variants::ghost_button`, as the toolbar's are -- dispatching its action,
/// selected while its panel is open, its icon of the chosen icon theme at
/// `geometry::icon_size_small` and its tooltip showing the action's key
/// binding. Where the icon theme has no icon for it, it shows its tooltip's
/// text instead, never another icon theme's icon. The icon is the Button's
/// child, not its `icon`, for the reason `toolbar_button` gives.
pub(crate) fn panel_toggle(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: PanelToggle<'_>,
) -> Stateful<Div> {
    let PanelToggle {
        button_id: id,
        icon,
        drawn,
        set,
        tooltip,
        action,
        open,
        about,
        state,
    } = spec;
    let mut button_info = info::panel_toggle(cx.theme(), about, &drawn, set, open, state);
    let icon =
        chrome_icon(&drawn, &icon).map(|icon| native_sized(cx, icon, geometry::icon_size_small));
    if icon.is_some() && native_value(cx, geometry::icon_size_small).is_some() {
        button_info = button_info.geometry("icon_size_small");
    }
    let dispatched = action.boxed_clone();
    let button = ButtonKind::Ghost
        .apply(Button::new(id), cx)
        .small()
        .selected(open)
        .toggled(open)
        .tooltip_with_action(tooltip, action, None)
        .map(|button| match icon {
            Some(icon) => button.child(icon),
            None => button.label(tooltip),
        })
        .on_click(move |_, window, cx| window.dispatch_action(dispatched.boxed_clone(), cx));
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(
        button,
        ui,
        SharedString::from(format!("chrome-button-{id}")),
        button_info,
    )
}

/// The gap between the theme settings' rows, and between each row's label
/// and its control, where `layout.widget_gap` is unstated (spec §3.1). The
/// settings are the showcase's own element, with no toolkit default to fall
/// back on, so this is the showcase's own choice, not a platform's value,
/// and the settings' info says so.
pub(crate) const THEME_SETTINGS_GAP: Pixels = px(8.);

/// The theme settings (spec §3.3, S2): `rows`, each `(id, text, control)`, a
/// `label` reading `text` above `control`. The rows, and each row's label and
/// control, are `widget_gap` apart, or [`THEME_SETTINGS_GAP`] where that is
/// unstated. The side panel pads around them (`side_panel`), so they add no
/// margin of their own; they take the width they are given, and so do the
/// controls.
pub(crate) fn theme_settings<const N: usize>(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    widget_gap: Option<Pixels>,
    rows: [(&'static str, &'static str, AnyElement); N],
) -> Stateful<Div> {
    let gap = widget_gap.unwrap_or(THEME_SETTINGS_GAP);
    let mut settings_info = info::theme_settings(widget_gap, THEME_SETTINGS_GAP);
    if widget_gap.is_some() {
        settings_info = settings_info.geometry("widget_gap");
    }
    v_flex()
        .w_full()
        .gap(gap)
        .children(rows.map(|(id, text, control)| {
            v_flex()
                .w_full()
                .gap(gap)
                .child(label(ui, cx, id, text))
                .child(control)
        }))
        .info(ui, CHROME_THEME_SETTINGS, settings_info)
        .w_full()
        .min_w_0()
        .debug_selector(|| CHROME_THEME_SETTINGS.into())
}

/// The side panel (spec S2): `settings`, padded by `container_margin` where
/// the installed layout states one, over `separator`, over `inspector`, which
/// fills the rest of the panel's height and scrolls its own content.
///
/// Plain elements, not upstream's `Sidebar`: a Sidebar's children must
/// implement `SidebarItem` (sidebar/mod.rs:211), and neither the settings
/// nor the inspector is an item.
pub(crate) fn side_panel(
    ui: &Entity<InfoRegistry>,
    container_margin: Option<Pixels>,
    settings: impl IntoElement,
    separator: impl IntoElement,
    inspector: impl IntoElement,
) -> Stateful<Div> {
    let mut panel_info = info::side_panel(container_margin);
    if container_margin.is_some() {
        panel_info = panel_info.geometry("container_margin");
    }
    v_flex()
        .size_full()
        .child(with_padding(div().w_full(), container_margin).child(settings))
        .child(separator)
        .child(div().w_full().flex_1().min_h_0().child(inspector))
        .info(ui, CHROME_SIDE_PANEL, panel_info)
        .size_full()
}

/// One of the Layout page's `Sidebar` samples (spec S5).
pub(crate) struct SidebarSample {
    /// Its info's id and its debug selector.
    pub id: &'static str,
    /// Whether it is collapsed to its icon rail.
    pub collapsed: bool,
    /// The text its header reads.
    pub header: &'static str,
    /// Its items, as `(id, label, icon, drawn)`: `drawn` is `icon` as the
    /// chosen icon theme gives it. The first is active.
    pub items: [(&'static str, &'static str, IconName, ChromeIcon); 3],
    /// The chosen icon theme, as the Icons page names it.
    pub set: SharedString,
    /// Its height, which a Sidebar needs, its items being a list that takes
    /// the height it is given (sidebar/mod.rs, `RenderOnce for Sidebar`).
    pub height: Rems,
}

/// A `Sidebar` sample (spec S5): its header above its items, collapsed to
/// its icon rail where the sample says so. No width is given: upstream's own
/// apply, its default expanded width and its fixed rail
/// (sidebar/mod.rs:27-28).
pub(crate) fn sidebar(ui: &Entity<InfoRegistry>, cx: &App, sample: SidebarSample) -> Stateful<Div> {
    let SidebarSample {
        id,
        collapsed,
        header,
        items,
        set,
        height,
    } = sample;
    let items = items
        .into_iter()
        .enumerate()
        .map(|(ix, (item_id, label, icon, drawn))| SampleItem {
            ui: ui.clone(),
            id: item_id,
            label,
            active: ix == 0,
            collapsed: false,
            icon,
            drawn,
            set: set.clone(),
        });
    // Plain text, not a Label: `Label::render` sets `foreground` on its own
    // element (label.rs:211), which would hide the sidebar_foreground the
    // Sidebar gives its content.
    Sidebar::new(id)
        .collapsed(collapsed)
        .header(div().min_w_0().truncate().child(header))
        .children(items)
        .info(ui, id, info::layout::sidebar(cx.theme(), collapsed, header))
        .h(height)
        .debug_selector(move || id.into())
}

/// An item of a Layout page `Sidebar` sample.
///
/// A `Sidebar` renders its items itself, as it lays out its list
/// (sidebar/mod.rs, `RenderOnce for Sidebar`), so an item's info cannot be
/// wrapped around it from outside; this item builds its `SidebarMenuItem`
/// and wraps the info around what that renders.
#[derive(Clone)]
pub(crate) struct SampleItem {
    ui: Entity<InfoRegistry>,
    /// The item's info id and debug selector.
    id: &'static str,
    label: &'static str,
    active: bool,
    collapsed: bool,
    /// gpui-component's icon of the item, which `drawn` is of.
    icon: IconName,
    /// That icon as the chosen icon theme gives it.
    drawn: ChromeIcon,
    /// The chosen icon theme, as the Icons page names it.
    set: SharedString,
}

impl Collapsible for SampleItem {
    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

impl SidebarItem for SampleItem {
    fn render(
        self,
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let mut item_info = info::layout::sidebar_item(
            cx.theme(),
            self.label,
            self.active,
            self.collapsed,
            &self.drawn,
            &self.set,
        );
        let icon = chrome_icon(&self.drawn, &self.icon).map(|icon| sidebar_icon_sized(cx, icon));
        if icon.is_some() && native_value(cx, geometry::icon_size_small).is_some() {
            item_info = item_info.geometry("icon_size_small");
        }
        let item = SidebarMenuItem::new(self.label);
        let item = match icon {
            Some(icon) => item.icon(icon),
            None => item,
        };
        let item = item.active(self.active).collapsed(self.collapsed);
        let selector = self.id;
        SidebarItem::render(item, id, window, cx)
            .info(&self.ui, self.id, item_info)
            .w_full()
            .debug_selector(move || selector.into())
    }
}

/// `icon` at the size a Sidebar sample's item shows it at: the platform's
/// small icon size.
///
/// Platform-facts §2.1.8 names macOS's small size the sidebar's, and states
/// 16px for Windows, KDE and GNOME. The panel size would not fit: KDE states
/// 48px for its `Panel` group (platform-facts §2.1.8), which fits neither an
/// expanded item's `h_7` row (sidebar/menu.rs:308) nor the 48px rail
/// (sidebar/mod.rs:28). `SidebarMenuItem` keeps the icon it is given
/// (sidebar/menu.rs:300). The size goes on the Icon's style, which upstream
/// lays out as it would the same size given through `with_size`
/// (icon.rs:171-182), so a test can read it off the Icon this returns.
///
/// It takes the Icon built: a public helper that builds one and does not
/// report it fails `every_widget_reports_itself`, and the item it is handed
/// to reports it.
pub(crate) fn sidebar_icon_sized(cx: &App, icon: Icon) -> Icon {
    match native_value(cx, geometry::icon_size_small) {
        Some(Size::Size(size)) => icon.size(size),
        _ => icon,
    }
}

/// The showcase's two `TabBar`s.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TabBarKind {
    /// The inspector's (spec §2.6): Widget and Theme.
    Inspector,
    /// The content panel's (spec S3): a tab per page, with upstream's menu of
    /// every tab (tab/tab_bar.rs, `TabBar::menu`).
    Pages,
}

/// A `TabBar` of `kind`, Underline at `Size::Small`, over `tabs`, each
/// `(label, selector)`, `selected` the one shown; a click hands `on_click`
/// the index of the tab clicked.
///
/// Upstream's Underline bar pads neither itself nor its tabs (tab/tab.rs:
/// 79-81, tab/tab_bar.rs:393-402), leaving the inset to its container, so
/// the bar takes `container_margin`, the installed layout's
/// `geometry::container_margin`, on its left and right, as the side panel's
/// settings and the inspector's content do. Its bottom rule is drawn by an
/// absolute child at the bar's full size (tab_bar.rs:497-507), so it still
/// runs from edge to edge. Where the layout states no margin, upstream's
/// none stands.
pub(crate) fn tab_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    kind: TabBarKind,
    container_margin: Option<Pixels>,
    tabs: impl IntoIterator<Item = (&'static str, &'static str)>,
    selected: usize,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let (id, info_id, menu, bar_info) = match kind {
        TabBarKind::Inspector => (
            "inspector-tabs",
            "chrome-inspector-tabs",
            false,
            info::inspector_tab_bar(cx.theme()),
        ),
        TabBarKind::Pages => (
            "page-tabs",
            "chrome-page-tabs",
            true,
            info::page_tab_bar(cx.theme()),
        ),
    };
    let mut bar_info = bar_info.instance("padding", info::tab_bar_padding(container_margin));
    if container_margin.is_some() {
        bar_info = bar_info.geometry("container_margin");
    }
    TabBar::new(id)
        .underline()
        .with_size(Size::Small)
        .menu(menu)
        .when_some(container_margin, |bar, margin| bar.px(margin))
        .children(tabs.into_iter().map(|(label, selector)| {
            Tab::new()
                .label(label)
                .debug_selector(move || selector.into())
        }))
        .selected_index(selected)
        .on_click(on_click)
        .info(ui, info_id, bar_info)
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

/// One entry of the command palette.
pub(crate) struct PaletteEntry {
    pub label: SharedString,
    /// What typing finds it by besides its label.
    pub keywords: Vec<SharedString>,
    /// gpui-component's icon of the entry, which `drawn` is of.
    pub icon: IconName,
    /// That icon as the chosen icon set gives it.
    pub drawn: ChromeIcon,
    pub action: Box<dyn Action>,
}

impl Clone for PaletteEntry {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            keywords: self.keywords.clone(),
            icon: self.icon.clone(),
            drawn: self.drawn.clone(),
            action: self.action.boxed_clone(),
        }
    }
}

/// The palette's groups, as `(label, entries)`, each entry with its icon of
/// the chosen set, or with none where the set has none.
fn palette_groups(groups: Vec<(&'static str, Vec<PaletteEntry>)>) -> Vec<CommandGroup> {
    groups
        .into_iter()
        .map(|(label, entries)| {
            CommandGroup::new()
                .label(label)
                .items(entries.into_iter().map(|entry| {
                    let item = CommandItem::new()
                        .label(entry.label)
                        .keywords(entry.keywords)
                        .action(entry.action);
                    match chrome_icon(&entry.drawn, &entry.icon) {
                        Some(icon) => item.icon(icon),
                        None => item,
                    }
                }))
        })
        .collect()
}

/// The command palette (spec §2.8): `dialog`, titled, holding an unbordered
/// `Command` over `state` with `groups`, whose icons are of the icon set
/// named `set`. Running an entry dispatches its action, then closes the
/// palette.
///
/// The Dialog reports itself on its title: the Command fills the rest of
/// its content, and the surface around them is upstream's, out of reach.
pub(crate) fn command_palette(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    dialog: Dialog,
    state: &Entity<CommandState>,
    groups: Vec<(&'static str, Vec<PaletteEntry>)>,
    set: SharedString,
) -> Dialog {
    let groups = palette_groups(groups);
    let mut dialog_info = info::palette_dialog(cx.theme(), cx.reduce_motion());
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
                .info(
                    &ui,
                    "overlay-palette",
                    info::command_palette(cx.theme(), &set),
                )
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
    let mut dialog_info = info::about_dialog(cx.theme(), gap.is_some(), cx.reduce_motion());
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

/// Install `prefs` with `change` made, where a native theme is installed,
/// and tell `ui`: the theme is rebuilt, so an info shown for a target no
/// longer drawn would keep the colours of the theme before.
fn change_preferences(
    ui: &Entity<InfoRegistry>,
    cx: &mut App,
    change: impl FnOnce(&mut AccessibilityPreferences),
) {
    if let Some(mut prefs) = installed_preferences(cx) {
        change(&mut prefs);
        native_theme_gpui::apply_accessibility(&prefs, cx);
        ui.update(cx, |r, _| r.screen_changed());
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
        let clicked = ui.clone();
        Switch::new(selector)
            .checked(checked)
            .disabled(options.is_disabled())
            .with_size(options.size())
            .on_click(move |on: &bool, _window, cx| {
                change_preferences(&clicked, cx, |prefs| pref.set(prefs, *on));
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
    let scaled = ui.clone();
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
            move |scale, cx| {
                change_preferences(&scaled, cx, |prefs| {
                    prefs.text_scaling_factor = scale as f32
                })
            },
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
            info::preferences_sheet(cx.theme(), cx.reduce_motion()),
        ))
        .size(width)
        .child(
            settings
                .info(ui, "overlay-preferences", settings_info)
                .size_full()
                .debug_selector(|| OVERLAY_PREFERENCES.into()),
        )
}

/// An error `Alert` across the content, under the page TabBar (spec §2.5),
/// reading `message`, its icon `drawn`: gpui-component's CircleX as the
/// chosen icon theme named `set` gives it.
///
/// An Alert always holds an `Icon` (alert.rs, `Alert`), so where the icon
/// theme has none it is handed an empty one, sized to nothing and invisible: gpui
/// paints nothing of an invisible element (gpui-pre elements/div.rs,
/// `Interactivity::paint`), so its empty path is never loaded, and no other
/// icon theme's icon stands in.
pub(crate) fn alert(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    message: impl Into<SharedString>,
    drawn: ChromeIcon,
    set: &str,
) -> Stateful<Div> {
    let icon = chrome_icon(&drawn, &IconName::CircleX)
        .unwrap_or_else(|| Icon::empty().size_0().invisible());
    Alert::error(id, message.into())
        .icon(icon)
        .banner()
        .info(ui, id, info::theme_error_alert(cx.theme(), &drawn, set))
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
    Label::new(text)
        .text_base()
        .font_semibold()
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

/// A small Label reading `text`, in the colour the Label paints. `id` is its
/// info's id and its debug selector.
pub(crate) fn label(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: impl Into<SharedString>,
    text: impl Into<SharedString>,
) -> Stateful<Div> {
    let id: SharedString = id.into();
    Label::new(text)
        .text_sm()
        .info(ui, id.clone(), info::text::label(cx.theme()))
        .self_start()
        .debug_selector(move || id.to_string())
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
    /// Default, outlined.
    DefaultOutline,
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
            Self::DefaultOutline => "Default, outline",
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
            Self::DefaultOutline => button.outline(),
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
    /// `geometry::input_height` alone: the height rule `geometry::input`
    /// applies, and nothing else of it.
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
        // height and line height (input/input.rs:699-703, then :719).
        InputField::HeightOnly => native_info(
            input,
            cx,
            geometry::input_height,
            "input_height",
            &mut input_info,
        ),
    };
    input
        .info(ui, id, input_info)
        .debug_selector(move || id.into())
}

/// A `Textarea` over `state`, `width` by `height`, refined by
/// `geometry::input`: it renders as an `Input` (input/textarea.rs:164).
/// `geometry::input`'s height rule and padding are for a single-line field:
/// the Textarea's own `height` goes on after the builder, where it wins, and
/// the refinement's padding sides are cleared, because upstream pads only a
/// single-line root (input/input.rs:700-702).
pub(crate) fn textarea(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TextareaState>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let mut textarea_info = info::inputs::textarea(cx.theme());
    let mut textarea = native_info(
        Textarea::new(state).w(width),
        cx,
        geometry::input,
        "input",
        &mut textarea_info,
    );
    Styled::style(&mut textarea).padding = StyleRefinement::default().padding;
    Styled::h(textarea, height)
        .info(ui, id, textarea_info)
        .debug_selector(move || id.into())
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
/// Search icon, a field with a Copy button after it, which puts the field's
/// text on the clipboard and says so in a notification, and a textarea with
/// a note under it. They report as one.
pub(crate) fn input_groups(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    states: InputGroupStates<'_>,
    width: Pixels,
) -> Stateful<Div> {
    let mut groups_info = info::inputs::input_groups(cx.theme());
    // One refinement, recorded once, for both single-line groups; the
    // textarea group keeps upstream's frame. Without its padding: the frame
    // is not the Input, which sits inside it and keeps its own padding
    // (input/group.rs, render_control), so the frame padded as well would
    // inset the field twice.
    let mut frame = native_info(
        StyleRefinement::default(),
        cx,
        geometry::input,
        "input",
        &mut groups_info,
    );
    frame.padding = StyleRefinement::default().padding;
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
    .on_click({
        let copied = states.copy.clone();
        move |_, window, cx| {
            let value = copied.read(cx).value();
            cx.write_to_clipboard(ClipboardItem::new_string(value.to_string()));
            window.push_notification(
                Notification::success(value).title("Copied").autohide(true),
                cx,
            );
        }
    });
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
    let mut number = native_info(
        NumberInput::new(state),
        cx,
        geometry::input,
        "input",
        &mut number_info,
    );
    // Without geometry::input's padding: the refinement lands on the frame
    // round the buttons (input/number_input.rs, NumberInput::render), not on
    // the Input inside it, which keeps its own padding.
    Styled::style(&mut number).padding = StyleRefinement::default().padding;
    let number = number
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
        // What `native_info` applies the builder under.
        let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
        let mut row_info = info::data::list_row(cx.theme(), &self.label, state, styled);
        let item = native_info(
            ListItem::new(SharedString::from(id.clone())),
            cx,
            geometry::list_item,
            "list_item",
            &mut row_info,
        )
        // Plain text, not a Label: `Label::render` paints foreground on its
        // own element (label.rs:211) over the list font the row carries.
        .child(self.label)
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
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut row_info = info::data::tree_row(cx.theme(), &label, selected, styled);
    let item = native_info(
        ListItem::new(SharedString::from(id.clone())),
        cx,
        geometry::list_item,
        "list_item",
        &mut row_info,
    );
    let ui = ui.clone();
    // Plain text, as the List's rows.
    item.child(label)
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

// ---------------------------------------------------------------------------
// The Feedback page
// ---------------------------------------------------------------------------

/// The four severities an `Alert` and a `Notification` share, so the
/// matches over them in `info::feedback` are exhaustive and the compiler
/// rejects one without an arm.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Severity {
    Info,
    Success,
    Warning,
    Error,
}

impl Severity {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Error => "Error",
        }
    }
}

/// An `Alert` of `severity` reading `message`, titled with the severity's
/// name.
pub(crate) fn severity_alert(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    severity: Severity,
    message: &'static str,
) -> Stateful<Div> {
    let alert = match severity {
        Severity::Info => Alert::info(id, message),
        Severity::Success => Alert::success(id, message),
        Severity::Warning => Alert::warning(id, message),
        Severity::Error => Alert::error(id, message),
    };
    alert
        .title(severity.name())
        .info(
            ui,
            id,
            info::feedback::alert(cx.theme(), severity, false)
                .instance("title", severity.name())
                .instance("message", message),
        )
        .debug_selector(move || id.into())
}

/// A `Progress` bar at `value` percent, refined by `geometry::progress`;
/// `label` names it in its info, as the page's label beside it does.
pub(crate) fn progress(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    value: f32,
) -> Stateful<Div> {
    let mut bar_info = info::feedback::progress(cx.theme(), label, value, cx.reduce_motion());
    native_info(
        Progress::new(id).value(value),
        cx,
        geometry::progress,
        "progress",
        &mut bar_info,
    )
    .info(ui, id, bar_info)
    .debug_selector(move || id.into())
}

/// The ProgressCircles the Feedback page builds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum CircleKind {
    /// At 73%, at the default Size.
    Value,
    /// At 100%, at `Size::Large`.
    ValueLarge,
    /// Loading, at the platform's spinner diameter.
    Indeterminate,
}

impl CircleKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Value => "73%",
            Self::ValueLarge => "100%, Large",
            Self::Indeterminate => "indeterminate",
        }
    }
}

/// A `ProgressCircle` of `kind`. Indeterminate, it is a spinner drawn as an
/// arc, so the platform's spinner diameter is the size it takes.
pub(crate) fn progress_circle(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: CircleKind,
) -> Stateful<Div> {
    let circle = ProgressCircle::new(id);
    let spinner_size = native_value(cx, geometry::spinner_size);
    let (circle, styled) = match kind {
        CircleKind::Value => (circle.value(73.0), false),
        CircleKind::ValueLarge => (circle.value(100.0).with_size(Size::Large), false),
        CircleKind::Indeterminate => match spinner_size {
            Some(size) => (circle.loading(true).with_size(size), true),
            None => (circle.loading(true), false),
        },
    };
    let circle_info = info::feedback::progress_circle(cx.theme(), kind, styled, cx.reduce_motion());
    let circle_info = if styled {
        circle_info.geometry("spinner_size")
    } else {
        circle_info
    };
    circle
        .info(ui, id, circle_info)
        .debug_selector(move || id.into())
}

/// The Spinners the Feedback page builds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpinnerKind {
    Small,
    /// At the platform's spinner diameter where a native theme is
    /// installed, upstream's Medium otherwise.
    Medium,
    Large,
}

impl SpinnerKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        }
    }
}

/// A `Spinner` of `kind`.
pub(crate) fn spinner(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: SpinnerKind,
) -> Stateful<Div> {
    let (size, styled) = match kind {
        SpinnerKind::Small => (Size::Small, false),
        SpinnerKind::Large => (Size::Large, false),
        SpinnerKind::Medium => match native_value(cx, geometry::spinner_size) {
            Some(size) => (size, true),
            None => (Size::Medium, false),
        },
    };
    let spinner_info = info::feedback::spinner(cx.theme(), kind, styled, cx.reduce_motion());
    let spinner_info = if styled {
        spinner_info.geometry("spinner_size")
    } else {
        spinner_info
    };
    Spinner::new()
        .with_size(size)
        .info(ui, id, spinner_info)
        .debug_selector(move || id.into())
}

/// A `Skeleton` placeholder, `secondary` or not, `height` tall and `width`
/// wide, or as wide as its column where `width` is `None`. Its rounding is
/// the theme's, `radius_lg` or `radius`, not a number of its own.
pub(crate) fn skeleton(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    secondary: bool,
    height: f32,
    width: Option<f32>,
    radius_lg: bool,
) -> Stateful<Div> {
    let t = cx.theme();
    let skeleton =
        Skeleton::new()
            .h(px(height))
            .rounded(if radius_lg { t.radius_lg } else { t.radius });
    let skeleton = if secondary {
        skeleton.secondary()
    } else {
        skeleton
    };
    let skeleton = match width {
        Some(width) => skeleton.w(px(width)),
        None => skeleton,
    };
    skeleton
        .info(
            ui,
            id,
            info::feedback::skeleton(t, secondary, height, width, radius_lg, cx.reduce_motion()),
        )
        .debug_selector(move || id.into())
}

/// The ShimmerTexts the Feedback page builds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShimmerKind {
    /// Upstream's defaults.
    Default,
    /// A 3s sweep, in the muted text colour.
    Slow,
    /// Swept right to left, half the text wide.
    Reverse,
}

impl ShimmerKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Slow => "slow, muted",
            Self::Reverse => "reversed",
        }
    }
}

/// A `ShimmerText` of `kind` reading `text`. The highlight is mixed from the
/// text colour, so each shimmers in whatever colour the native theme gave its
/// own text.
pub(crate) fn shimmer_text(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: ShimmerKind,
    text: &'static str,
) -> Stateful<Div> {
    let t = cx.theme();
    let shimmer = ShimmerText::new(text).id(id);
    let shimmer = match kind {
        ShimmerKind::Default => shimmer,
        ShimmerKind::Slow => shimmer
            .duration(Duration::from_secs(3))
            .text_color(t.muted_foreground),
        ShimmerKind::Reverse => shimmer.reverse(true).spread(0.5),
    };
    shimmer
        .info(
            ui,
            id,
            info::feedback::shimmer_text(t, kind, text, cx.reduce_motion()),
        )
        .debug_selector(move || id.into())
}

/// The `Empty` state, titled `title` over `description`, its media the
/// Inbox icon at the platform's large icon size and its action an outlined
/// Refresh Button, which reports itself as `refresh`.
pub(crate) fn empty(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    refresh: &'static str,
    title: &'static str,
    description: &'static str,
) -> Stateful<Div> {
    let empty_info = info::feedback::empty(cx.theme(), title, description);
    let empty_info = if native_value(cx, geometry::icon_size_large).is_some() {
        empty_info.geometry("icon_size_large")
    } else {
        empty_info
    };
    Empty::new()
        .header(
            EmptyHeader::new()
                .media(
                    EmptyMedia::new()
                        .with_variant(EmptyMediaVariant::Icon)
                        // An empty state's icon is the large one; `EmptyMedia`
                        // takes it as a plain child, so the size survives.
                        .child(native_icon(cx, IconName::Inbox, geometry::icon_size_large)),
                )
                .title(EmptyTitle::new().child(title))
                .description(EmptyDescription::new().child(description)),
        )
        .content(EmptyContent::new().child(button(
            ui,
            cx,
            DemoButton {
                id: refresh,
                label: "Refresh",
                kind: ButtonKind::DefaultOutline,
                state: ButtonState::Idle,
                icon: None,
            },
        )))
        .info(ui, id, empty_info)
        .debug_selector(move || id.into())
}

/// The Tag variants the showcase builds, so the matches over them in
/// `info::feedback` are exhaustive and the compiler rejects a variant
/// without an arm.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TagKind {
    Primary,
    Secondary,
    Danger,
    Success,
    Warning,
    Info,
}

impl TagKind {
    fn variant(self) -> TagVariant {
        match self {
            Self::Primary => TagVariant::Primary,
            Self::Secondary => TagVariant::Secondary,
            Self::Danger => TagVariant::Danger,
            Self::Success => TagVariant::Success,
            Self::Warning => TagVariant::Warning,
            Self::Info => TagVariant::Info,
        }
    }
}

/// A `Tag` of `kind`, `outline` or not, reading `label`.
pub(crate) fn tag(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: TagKind,
    outline: bool,
    label: &'static str,
) -> Stateful<Div> {
    let tag = Tag::new().with_variant(kind.variant());
    let tag = if outline { tag.outline() } else { tag };
    tag.child(label)
        .info(
            ui,
            id,
            info::feedback::tag(cx.theme(), kind, outline, label),
        )
        .debug_selector(move || id.into())
}

/// A `Badge` showing `count`, or a dot where `count` is `None`, on a
/// Default Button reading `label` and refined by `geometry::button`. The
/// Badge is exactly as large as the Button, so the Button reports through
/// it.
pub(crate) fn badge(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    count: Option<usize>,
    label: &'static str,
) -> Stateful<Div> {
    let mut badge_info = info::feedback::badge(cx.theme(), count, label);
    let target = native_info(
        Button::new(id),
        cx,
        geometry::button,
        "button",
        &mut badge_info,
    )
    .label(label);
    let badge = match count {
        Some(count) => Badge::new().count(count),
        None => Badge::new().dot(),
    };
    badge
        .child(target)
        .info(ui, id, badge_info)
        .debug_selector(move || id.into())
}

/// The Markers the Feedback page builds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum MarkerKind {
    /// Plain, with an icon slot.
    Plain,
    Separator,
    Border,
    /// Loading, with the spinner the Marker adds for itself when no icon
    /// slot is set.
    Spinner,
    /// Loading, shimmering.
    Shimmer,
}

impl MarkerKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Plain => "Plain, with an icon",
            Self::Separator => "Separator",
            Self::Border => "Border",
            Self::Spinner => "loading, Spinner",
            Self::Shimmer => "loading, Shimmer",
        }
    }
}

/// A `Marker` of `kind` reading `text`; a Plain one shows the CircleCheck
/// icon at the platform's small icon size.
pub(crate) fn marker(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: MarkerKind,
    text: &'static str,
) -> Stateful<Div> {
    let mut marker_info = info::feedback::marker(cx.theme(), kind, text, cx.reduce_motion());
    let marker = Marker::new();
    let marker = match kind {
        MarkerKind::Plain => {
            if native_value(cx, geometry::icon_size_small).is_some() {
                marker_info = marker_info.geometry("icon_size_small");
            }
            marker.icon(MarkerIcon::new().child(native_icon(
                cx,
                IconName::CircleCheck,
                geometry::icon_size_small,
            )))
        }
        MarkerKind::Separator => marker.with_variant(MarkerVariant::Separator),
        MarkerKind::Border => marker.with_variant(MarkerVariant::Border),
        MarkerKind::Spinner => marker.id(id).loading(true),
        MarkerKind::Shimmer => marker
            .id(id)
            .loading(true)
            .with_loading_style(MarkerLoadingStyle::Shimmer),
    };
    marker
        .content(MarkerContent::new().text(text))
        .info(ui, id, marker_info)
        .debug_selector(move || id.into())
}

/// A Default Button reading `label`, refined by `geometry::button`, whose
/// tooltip reads `text`. The Button reports through the tooltip: its popup
/// is drawn on a layer above the page, where nothing can wrap it.
pub(crate) fn tooltip_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    text: &'static str,
) -> Stateful<Div> {
    let mut tooltip_info = info::feedback::tooltip(cx.theme(), false, false, label, text);
    let button = native_info(
        Button::new(id),
        cx,
        geometry::button,
        "button",
        &mut tooltip_info,
    )
    .label(label)
    .tooltip(text);
    InfoExt::info(button, ui, id, tooltip_info).debug_selector(move || id.into())
}

/// A Default Button reading `label`, refined by `geometry::button`, under a
/// tooltip the application builds itself, reading `text`.
///
/// `Button::tooltip` takes a string and builds the tooltip itself
/// (`button/button.rs:389`), so the only way to a refined one is to build
/// it: that is what `geometry::tooltip` documents, and the one place the
/// platform's tooltip padding, radius and text colour reach the popup. The
/// width is the content element's, through `Tooltip::element`: on the
/// popup it would clamp the popup and not the text, which then runs out of
/// it.
pub(crate) fn built_tooltip_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    text: &'static str,
) -> Stateful<Div> {
    let style = native_geometry(cx, geometry::tooltip);
    let content = native_geometry(cx, geometry::tooltip_content);
    let mut tooltip_info = info::feedback::tooltip(cx.theme(), true, style.is_some(), label, text);
    let button = native_info(
        Button::new(id),
        cx,
        geometry::button,
        "button",
        &mut tooltip_info,
    )
    .label(label);
    if style.is_some() {
        tooltip_info = tooltip_info.geometry("tooltip");
    }
    if content.is_some() {
        tooltip_info = tooltip_info.geometry("tooltip_content");
    }
    InfoExt::info(button, ui, id, tooltip_info)
        .debug_selector(move || id.into())
        .tooltip(move |window, cx| {
            let content = content.clone();
            refined(
                Tooltip::element(move |_window, _cx| refined(div().child(text), content.as_ref())),
                style.as_ref(),
            )
            .build(window, cx)
        })
}

/// A Default Button reading `label`, refined by `geometry::button`, whose
/// click pushes a `Notification` of `severity` reading `message`, titled
/// with the severity's name. The notification reports through the Button:
/// upstream draws it on the Root's layer, where nothing can wrap it.
pub(crate) fn notification_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    severity: Severity,
    label: &'static str,
    message: &'static str,
) -> Stateful<Div> {
    let mut button_info = info::feedback::notification(cx.theme(), severity, label, message);
    let button = native_info(
        Button::new(id),
        cx,
        geometry::button,
        "button",
        &mut button_info,
    )
    .label(label)
    .on_click(move |_ev, window, cx| {
        let notification = match severity {
            Severity::Info => Notification::info(message),
            Severity::Success => Notification::success(message),
            Severity::Warning => Notification::warning(message),
            Severity::Error => Notification::error(message),
        };
        window.push_notification(notification.title(severity.name()).autohide(true), cx);
    });
    InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into())
}

// ---------------------------------------------------------------------------
// The Typography page
// ---------------------------------------------------------------------------

/// The Labels of the Typography page's Label gallery, so the match over
/// them in `info::text` is exhaustive and the compiler rejects one without
/// an arm.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum LabelKind {
    Plain,
    /// With the given secondary text after it.
    Secondary(&'static str),
    Masked,
}

impl LabelKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Secondary(_) => "with secondary",
            Self::Masked => "masked",
        }
    }
}

/// A `Label` of `kind` reading `text`.
pub(crate) fn gallery_label(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: LabelKind,
    text: &'static str,
) -> Stateful<Div> {
    let label = Label::new(text);
    let label = match kind {
        LabelKind::Plain => label,
        LabelKind::Secondary(secondary) => label.secondary(secondary),
        LabelKind::Masked => label.masked(true),
    };
    label
        .info(ui, id, info::text::gallery_label(cx.theme(), kind, text))
        .self_start()
        .debug_selector(move || id.into())
}

/// The text sizes of the Font Sizes gallery, so the match over them in
/// `info::text` is exhaustive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextSize {
    Xs,
    Sm,
    /// text_base, which is gpui's default size too.
    Base,
    Lg,
    Xl,
}

impl TextSize {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Xs => "text_xs",
            Self::Sm => "text_sm",
            Self::Base => "text_base",
            Self::Lg => "text_lg",
            Self::Xl => "text_xl",
        }
    }
    /// The size in rems each sets (gpui-pre styled.rs:545-576).
    pub(crate) fn rems(self) -> f32 {
        match self {
            Self::Xs => 0.75,
            Self::Sm => 0.875,
            Self::Base => 1.0,
            Self::Lg => 1.125,
            Self::Xl => 1.25,
        }
    }
}

/// A `Label` reading `text` at `size`.
pub(crate) fn sized_label(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    size: TextSize,
    text: &'static str,
) -> Stateful<Div> {
    let label = Label::new(text);
    let label = match size {
        TextSize::Xs => label.text_xs(),
        TextSize::Sm => label.text_sm(),
        TextSize::Base => label.text_base(),
        TextSize::Lg => label.text_lg(),
        TextSize::Xl => label.text_xl(),
    };
    label
        .info(ui, id, info::text::sized_label(cx.theme(), size))
        .self_start()
        .debug_selector(move || id.into())
}

/// The showcase's own heading ladder, H1 to H6.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::H1 => "H1",
            Self::H2 => "H2",
            Self::H3 => "H3",
            Self::H4 => "H4",
            Self::H5 => "H5",
            Self::H6 => "H6",
        }
    }
    pub(crate) fn rems(self) -> f32 {
        match self {
            Self::H1 => 1.875,
            Self::H2 => 1.5,
            Self::H3 => 1.25,
            Self::H4 => 1.125,
            Self::H5 => 1.0,
            Self::H6 => 0.875,
        }
    }
    /// The weight, and the name of its `FontWeight` constant.
    pub(crate) fn weight(self) -> (FontWeight, &'static str) {
        match self {
            Self::H1 | Self::H2 => (FontWeight::BOLD, "BOLD"),
            Self::H3 | Self::H4 => (FontWeight::SEMIBOLD, "SEMIBOLD"),
            Self::H5 | Self::H6 => (FontWeight::MEDIUM, "MEDIUM"),
        }
    }
}

/// Plain text reading `text` at heading `level`.
pub(crate) fn heading_level(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    level: HeadingLevel,
    text: &'static str,
) -> Stateful<Div> {
    div()
        .text_size(rems(level.rems()))
        .font_weight(level.weight().0)
        .child(text)
        .info(ui, id, info::typography::heading_level(cx.theme(), level))
        .self_start()
        .debug_selector(move || id.into())
}

/// The weights of the Font Weights list, Thin to Black.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum WeightKind {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

impl WeightKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Thin => "Thin (100)",
            Self::ExtraLight => "Extra Light (200)",
            Self::Light => "Light (300)",
            Self::Normal => "Normal (400)",
            Self::Medium => "Medium (500)",
            Self::Semibold => "Semibold (600)",
            Self::Bold => "Bold (700)",
            Self::ExtraBold => "Extra Bold (800)",
            Self::Black => "Black (900)",
        }
    }
    /// The weight, and the name of its `FontWeight` constant.
    pub(crate) fn weight(self) -> (FontWeight, &'static str) {
        match self {
            Self::Thin => (FontWeight::THIN, "THIN"),
            Self::ExtraLight => (FontWeight::EXTRA_LIGHT, "EXTRA_LIGHT"),
            Self::Light => (FontWeight::LIGHT, "LIGHT"),
            Self::Normal => (FontWeight::NORMAL, "NORMAL"),
            Self::Medium => (FontWeight::MEDIUM, "MEDIUM"),
            Self::Semibold => (FontWeight::SEMIBOLD, "SEMIBOLD"),
            Self::Bold => (FontWeight::BOLD, "BOLD"),
            Self::ExtraBold => (FontWeight::EXTRA_BOLD, "EXTRA_BOLD"),
            Self::Black => (FontWeight::BLACK, "BLACK"),
        }
    }
}

/// Plain text at `weight`, reading its name.
pub(crate) fn weight_sample(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    weight: WeightKind,
) -> Stateful<Div> {
    div()
        .font_weight(weight.weight().0)
        .child(weight.name())
        .info(ui, id, info::typography::weight(cx.theme(), weight))
        .self_start()
        .debug_selector(move || id.into())
}

/// The styles of the Text Decorations list.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum DecorationKind {
    Bold,
    Underline,
    Strikethrough,
    Italic,
}

impl DecorationKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Bold => "bold",
            Self::Underline => "underlined",
            Self::Strikethrough => "struck through",
            Self::Italic => "italic",
        }
    }
}

/// Plain text reading `text`, styled `decoration`.
pub(crate) fn decoration_sample(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    decoration: DecorationKind,
    text: &'static str,
) -> Stateful<Div> {
    let sample = div();
    let sample = match decoration {
        DecorationKind::Bold => sample.font_weight(FontWeight::BOLD),
        DecorationKind::Underline => sample.underline().text_decoration_1(),
        DecorationKind::Strikethrough => sample.line_through(),
        DecorationKind::Italic => sample.italic(),
    };
    sample
        .child(text)
        .info(ui, id, info::typography::decoration(cx.theme(), decoration))
        .self_start()
        .debug_selector(move || id.into())
}

/// Plain text reading `text` in the muted colour.
pub(crate) fn muted_text(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: &'static str,
) -> Stateful<Div> {
    div()
        .text_color(cx.theme().muted_foreground)
        .child(text)
        .info(ui, id, info::typography::muted_text(cx.theme()))
        .self_start()
        .debug_selector(move || id.into())
}

/// Plain text reading `text` in the mono family.
pub(crate) fn mono_text(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: &'static str,
) -> Stateful<Div> {
    div()
        .font_family(cx.theme().mono_font_family.clone())
        .child(text)
        .info(ui, id, info::typography::mono_text(cx.theme()))
        .self_start()
        .debug_selector(move || id.into())
}

/// A `Link` reading `text` that opens `href`.
pub(crate) fn link(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: &'static str,
    href: &'static str,
) -> Stateful<Div> {
    Link::new(id)
        .child(text)
        .href(href)
        .info(
            ui,
            id,
            info::typography::link(cx.theme())
                .instance("text", text)
                .instance("target", href),
        )
        .debug_selector(move || id.into())
}

/// A `Kbd` for `keys`, in gpui's keystroke syntax; `None` where they do not
/// parse.
pub(crate) fn kbd(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    keys: &'static str,
) -> Option<Stateful<Div>> {
    let stroke = Keystroke::parse(keys).ok()?;
    let drawn = Kbd::format(&stroke);
    Some(
        Kbd::new(stroke)
            .info(ui, id, info::typography::kbd(cx.theme(), keys, &drawn))
            .debug_selector(move || id.into()),
    )
}

/// The Rust code `Editor` over `state`, `height` tall.
pub(crate) fn editor(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<EditorState>,
    height: Pixels,
) -> Stateful<Div> {
    Editor::new(state)
        .h(height)
        .info(ui, id, info::typography::editor(cx.theme()))
        .debug_selector(move || id.into())
}

/// A Markdown `TextView` of `source`, selectable.
pub(crate) fn markdown(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    source: &'static str,
) -> Stateful<Div> {
    TextView::markdown(id, source)
        .selectable(true)
        .info(ui, id, info::typography::markdown(cx.theme()))
        .debug_selector(move || id.into())
}

// ---------------------------------------------------------------------------
// The Layout page
// ---------------------------------------------------------------------------

/// The two boxes the Layout page applies the four layout accessors to, so
/// the match over them in `info::layout` is exhaustive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpacingBox {
    /// A column padded by the window margin, its children the section gap
    /// apart.
    Window,
    /// A row padded by the container margin, its children the widget gap
    /// apart.
    Container,
}

/// A spacing box of `kind` in the showcase's frame, holding `children`,
/// padded by `padding` and spacing them by `gap`. Where either is `None`
/// the platform states none and gpui's own spacing stands.
pub(crate) fn spacing_box(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: SpacingBox,
    padding: Option<Pixels>,
    gap: Option<Pixels>,
    children: impl IntoIterator<Item = AnyElement>,
) -> Stateful<Div> {
    let box_info = info::layout::spacing_box(cx.theme(), kind, padding, gap);
    let (frame, box_info) = match kind {
        SpacingBox::Window => {
            let box_info = if padding.is_some() {
                box_info.geometry("window_margin")
            } else {
                box_info
            };
            let box_info = if gap.is_some() {
                box_info.geometry("section_gap")
            } else {
                box_info
            };
            (v_flex(), box_info)
        }
        SpacingBox::Container => {
            let box_info = if padding.is_some() {
                box_info.geometry("container_margin")
            } else {
                box_info
            };
            let box_info = if gap.is_some() {
                box_info.geometry("widget_gap")
            } else {
                box_info
            };
            (h_flex(), box_info)
        }
    };
    with_padding(with_gap(frame.demo_frame(cx), gap), padding)
        .children(children)
        .info(ui, id, box_info)
        .debug_selector(move || id.into())
}

/// The Separators the Layout page builds, so the match over them in
/// `info::layout` is exhaustive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SeparatorKind {
    Horizontal,
    /// Horizontal, reading the given label.
    Labelled(&'static str),
    /// Horizontal, dashed.
    Dashed,
}

impl SeparatorKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Labelled(_) => "horizontal, labelled",
            Self::Dashed => "horizontal, dashed",
        }
    }
}

/// A `Separator` of `kind`.
pub(crate) fn separator(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: SeparatorKind,
) -> Stateful<Div> {
    let separator = match kind {
        SeparatorKind::Horizontal => Separator::horizontal(),
        SeparatorKind::Labelled(label) => Separator::horizontal().label(label),
        SeparatorKind::Dashed => Separator::horizontal_dashed(),
    };
    separator
        .info(ui, id, info::layout::separator(cx.theme(), kind))
        // A horizontal Separator's box is as tall as its label and no
        // taller: the line is an absolute child (separator.rs:79-84). The
        // padding leaves something to point at.
        .py_1()
        .debug_selector(move || id.into())
}

/// The GroupBox variants, so the matches over them in `info::layout` are
/// exhaustive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum GroupBoxKind {
    Normal,
    Fill,
    Outline,
}

impl GroupBoxKind {
    /// The name upstream gives the variant (group_box.rs, GroupBoxVariant).
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Fill => "Fill",
            Self::Outline => "Outline",
        }
    }
    fn variant(self) -> GroupBoxVariant {
        match self {
            Self::Normal => GroupBoxVariant::Normal,
            Self::Fill => GroupBoxVariant::Fill,
            Self::Outline => GroupBoxVariant::Outline,
        }
    }
}

/// A `GroupBox` of `kind` titled `title` around `content`, which `about`
/// describes in its info; its content refined by
/// `geometry::group_box_content`.
pub(crate) fn group_box(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: GroupBoxKind,
    title: &'static str,
    content: impl IntoElement,
    about: &'static str,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut box_info =
        info::layout::group_box(cx.theme(), kind, styled, title).instance("content", about);
    // `GroupBox::content_style` takes a refinement rather than being one.
    let content_style = native_info(
        StyleRefinement::default(),
        cx,
        geometry::group_box_content,
        "group_box_content",
        &mut box_info,
    );
    GroupBox::new()
        .with_variant(kind.variant())
        .content_style(content_style)
        .title(title)
        .child(content)
        .info(ui, id, box_info)
        .debug_selector(move || id.into())
}

/// A column of `items` small Labels, each reporting itself, `height` tall in
/// the showcase's frame, scrolled by gpui-component's scrollbar and kept
/// clear of it by `geometry::scrollbar_gutter`.
pub(crate) fn scroll_area(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    height: Pixels,
    items: usize,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut area_info = info::layout::scroll_area(cx.theme(), styled, items);
    native_info(
        div()
            .id(id)
            .h(height)
            .w_full()
            .demo_frame(cx)
            .overflow_y_scrollbar(),
        cx,
        geometry::scrollbar_gutter,
        "scrollbar_gutter",
        &mut area_info,
    )
    .child(v_flex().gap_2().p_3().children((1..=items).map(|n| {
        label(
            ui,
            cx,
            format!("{id}-item-{n}"),
            format!("Scrollable item #{n} - demonstrates scrollbar theming"),
        )
    })))
    .info(ui, id, area_info)
    // A scroller inside the page's own takes the pointer out of the page's
    // hit test, or a wheel turned here would scroll both (gpui-pre
    // window.rs, HitboxBehavior::BlockMouse). On the wrapper, not the
    // scroller: an occluding scroller would hide its wrapper's hover.
    .occlude()
    .debug_selector(move || id.into())
}

/// The Accordion of the Layout page: an item per `(title, answer id,
/// answer)`, the first open, their title rows refined by
/// `geometry::accordion_title`. Each answer is a small Label that reports
/// itself.
pub(crate) fn accordion(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    items: [(&'static str, &'static str, SharedString); 3],
) -> Stateful<Div> {
    let mut accordion_info = info::layout::accordion(cx.theme(), cx.reduce_motion(), items.len());
    // `AccordionItem::title_style` takes a refinement rather than being one.
    let title_style = native_info(
        StyleRefinement::default(),
        cx,
        geometry::accordion_title,
        "accordion_title",
        &mut accordion_info,
    );
    items
        .into_iter()
        .enumerate()
        .fold(
            Accordion::new(id),
            |accordion, (ix, (title, answer_id, answer))| {
                accordion.item(|item| {
                    item.title_style(title_style.clone())
                        .title(title)
                        .open(ix == 0)
                        .child(label(ui, cx, answer_id, answer))
                })
            },
        )
        .info(ui, id, accordion_info)
        .debug_selector(move || id.into())
}

/// A `Collapsible`, `open` or not, toggled by an upstream Ghost Button
/// `toggle` that runs `on_toggle`, over a small Label `content`. The
/// Collapsible, the Button and the Label each report themselves.
pub(crate) fn collapsible(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    toggle: &'static str,
    content: &'static str,
    open: bool,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let text = if open {
        "Click to collapse"
    } else {
        "Click to expand"
    };
    let button = Button::new(toggle)
        .label(text)
        .ghost()
        .icon(if open {
            IconName::ChevronDown
        } else {
            IconName::ChevronRight
        })
        .on_click(on_toggle);
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    let button = InfoExt::info(
        button,
        ui,
        toggle,
        info::layout::collapsible_toggle(cx.theme(), open, text),
    )
    .debug_selector(move || toggle.into());
    gpui_component::collapsible::Collapsible::new()
        .open(open)
        .child(button)
        .content(v_flex().p_3().child(label(
            ui,
            cx,
            content,
            "This content is shown when collapsible is open.",
        )))
        .info(ui, id, info::layout::collapsible(open))
        .debug_selector(move || id.into())
}

/// The Carousel over `state`, `width` wide with slides `height` tall: a
/// slide per `CAROUSEL_SLIDES` entry, a page button for each, and its
/// previous and next controls. The last page button carries
/// `PROBE_CAROUSEL_LAST`.
pub(crate) fn carousel(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<CarouselState>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let t = cx.theme();
    let selected = state.read(cx).selected_index();
    let slides = CAROUSEL_SLIDES
        .iter()
        .enumerate()
        .map(|(ix, (title, caption))| {
            let slide_id = SharedString::from(format!("{id}-slide-{}", ix + 1));
            let selector = slide_id.clone();
            CarouselItem::new((id, ix), ix, state).child(
                v_flex()
                    .size_full()
                    .justify_center()
                    .gap_1()
                    .p_4()
                    .rounded(t.radius)
                    .bg(t.muted)
                    .child(Label::new(*title).font_semibold())
                    .child(
                        Label::new(*caption)
                            .text_sm()
                            .text_color(t.muted_foreground),
                    )
                    .info(
                        ui,
                        slide_id,
                        info::layout::carousel_slide(t, ix, title, caption),
                    )
                    .size_full()
                    .debug_selector(move || selector.to_string()),
            )
        });
    let last = CAROUSEL_SLIDES.len().saturating_sub(1);
    let pages = (0..CAROUSEL_SLIDES.len()).map(|ix| {
        let page_id = SharedString::from(format!("{id}-page-{}", ix + 1));
        let selector = page_id.clone();
        let page = CarouselPaginationItem::new(page_id.clone(), ix, state)
            .child(SharedString::from((ix + 1).to_string()))
            .info(
                ui,
                page_id,
                info::layout::carousel_page(t, ix, selected == Some(ix)),
            )
            .debug_selector(move || selector.to_string());
        // The last page button is the self-test's way into the carousel:
        // the previous and next controls place themselves absolutely
        // outside the frame, so a wrapper around one of those would take
        // it out of the flow.
        if ix == last {
            probe(PROBE_CAROUSEL_LAST, page).into_any_element()
        } else {
            page.into_any_element()
        }
    });
    Carousel::new(id, state)
        .w(width)
        .child(CarouselContent::new(state).h(height).children(slides))
        .child(CarouselPagination::new().children(pages))
        // The carousel's own slide controls. They take the same state as
        // the viewport and position themselves outside the frame
        // (`carousel/carousel.rs:757-768`), so they belong to the carousel
        // rather than beside it.
        .child(CarouselPrevious::new(state))
        .child(CarouselNext::new(state))
        .info(ui, id, info::layout::carousel(t, cx.reduce_motion()))
        .self_start()
        .debug_selector(move || id.into())
}

/// The Steppers of the Layout page.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum StepperKind {
    /// Horizontal, each step showing its icon at the platform's small icon
    /// size.
    Icons,
    /// Vertical, each step showing its number.
    Numbers,
}

impl StepperKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Icons => "horizontal, icons",
            Self::Numbers => "vertical, numbered",
        }
    }
}

/// A `Stepper` of `kind` through `STEPPER_STEPS`, `step` the current one; a
/// click on a step hands `on_click` its index. Each step's label is a small
/// Label that reports itself.
pub(crate) fn stepper(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: StepperKind,
    step: usize,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let mut stepper_info = info::layout::stepper(cx.theme(), kind, step, STEPPER_STEPS.len());
    // The indicator is a circle (stepper/trigger.rs:118-123) around the
    // icon it is given, which keeps its size (`:138-139`).
    if kind == StepperKind::Icons && native_value(cx, geometry::icon_size_small).is_some() {
        stepper_info = stepper_info.geometry("icon_size_small");
    }
    let stepper = match kind {
        StepperKind::Icons => Stepper::new(id),
        StepperKind::Numbers => Stepper::new(id).vertical(),
    };
    stepper
        .selected_index(step)
        .items(STEPPER_STEPS.iter().enumerate().map(|(ix, (text, icon))| {
            let item =
                StepperItem::new().child(label(ui, cx, format!("{id}-step-{}", ix + 1), *text));
            match kind {
                StepperKind::Icons => {
                    item.icon(native_icon(cx, icon.clone(), geometry::icon_size_small))
                }
                StepperKind::Numbers => item,
            }
        }))
        .on_click(on_click)
        .info(ui, id, stepper_info)
        .debug_selector(move || id.into())
}

/// A `Breadcrumb` through `pages`, each showing its page when clicked, to
/// `current`, the last, which takes no click.
pub(crate) fn breadcrumb(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    pages: &[Page],
    current: Page,
) -> Stateful<Div> {
    Breadcrumb::new()
        .children(pages.iter().map(|&page| {
            BreadcrumbItem::new(page.label()).on_click(move |_, window, cx| {
                window.dispatch_action(Box::new(ShowPage(page.index())), cx)
            })
        }))
        .child(BreadcrumbItem::new(current.label()))
        .info(ui, id, info::layout::breadcrumb(cx.theme(), pages, current))
        .self_start()
        .debug_selector(move || id.into())
}

/// The width the Layout page's Form gives its labels: the showcase's own,
/// narrower than upstream's 140px default (form/field.rs:27). The model
/// states no form.
const FORM_LABEL_WIDTH: Pixels = px(100.);

/// An `Input` over `state` in a Form's field, refined by `geometry::input`.
fn field_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<InputState>,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut input_info = info::inputs::input(cx.theme(), InputField::Refined, styled);
    native_info(
        Input::new(state),
        cx,
        geometry::input,
        "input",
        &mut input_info,
    )
    .info(ui, id, input_info)
    .debug_selector(move || id.into())
}

/// The horizontal `Form` of the Layout page: a required Name field over
/// `name` and an Email field over `email` with a description. The Form
/// reports for its Fields, and each field's Input reports itself.
pub(crate) fn form(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    (name_id, name): (&'static str, &Entity<InputState>),
    (email_id, email): (&'static str, &Entity<InputState>),
) -> Stateful<Div> {
    Form::horizontal()
        .label_width(FORM_LABEL_WIDTH)
        .child(
            Field::new()
                .label("Name")
                .required(true)
                .child(field_input(ui, cx, name_id, name)),
        )
        .child(
            Field::new()
                .label("Email")
                .description("We will never share your email.")
                .child(field_input(ui, cx, email_id, email)),
        )
        .info(ui, id, info::layout::form(cx.theme(), FORM_LABEL_WIDTH))
        .debug_selector(move || id.into())
}

/// The Layout page's `Settings`: two pages of fields upstream builds, each
/// group kept clear of the page's scrollbar by `geometry::scrollbar_gutter`,
/// and a whole-row item carrying `PROBE_SETTINGS_ROW`. A `SettingGroup` is
/// not an element the showcase can wrap, so the Settings reports for them.
pub(crate) fn settings(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str) -> Stateful<Div> {
    let mut settings_info = info::layout::settings(cx.theme());
    // One refinement for the three groups, so its line is recorded once.
    let gutter = native_info(
        StyleRefinement::default(),
        cx,
        geometry::scrollbar_gutter,
        "scrollbar_gutter",
        &mut settings_info,
    );
    let group = |title: &'static str| refined(SettingGroup::new(), Some(&gutter)).title(title);
    Settings::new(id)
        .page(
            SettingPage::new("Appearance")
                .description("Customize the look and feel")
                .default_open(true)
                .group(
                    group("Theme")
                        .item(
                            SettingItem::new(
                                "Dark Mode",
                                SettingField::switch(|_cx| false, |_val, _cx| {}),
                            )
                            .description("Toggle dark appearance"),
                        )
                        // The row probe is a whole-row item
                        // (setting/item.rs, SettingItem::render): a field's
                        // slot is only as wide as its content, so it would
                        // not span the row.
                        .item(SettingItem::render(|_, _, _| {
                            div()
                                .w_full()
                                .h(px(1.))
                                .debug_selector(|| PROBE_SETTINGS_ROW.into())
                        }))
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
                    group("Editor")
                        .item(SettingItem::new(
                            "Font Size",
                            SettingField::input(|_cx| "14".into(), |_val, _cx| {}),
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
                .group(group("Shortcuts").item(SettingItem::new(
                    "Vim Mode",
                    SettingField::switch(|_cx| false, |_val, _cx| {}),
                ))),
        )
        .info(ui, id, settings_info)
        .size_full()
        .debug_selector(move || id.into())
}

// ---------------------------------------------------------------------------
// The Overlays page
// ---------------------------------------------------------------------------

/// The edges of the window a Sheet of the showcase slides in at, so the
/// matches over them in `info` are exhaustive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SheetSide {
    Right,
    Bottom,
}

impl SheetSide {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Right => "Right",
            Self::Bottom => "Bottom",
        }
    }
}

/// The overlays the Overlays page's Buttons open.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Overlay {
    Dialog,
    AlertDialog,
    Sheet(SheetSide),
}

impl Overlay {
    /// The variant of the Button that opens it: the AlertDialog asks to
    /// discard, so its Button is a Danger one.
    pub(crate) fn button_kind(self) -> ButtonKind {
        match self {
            Self::AlertDialog => ButtonKind::Danger,
            Self::Dialog | Self::Sheet(_) => ButtonKind::Default,
        }
    }
}

/// The items of the Overlays page's two menus: a group, then, after a
/// separator, one more.
pub(crate) const SAMPLE_MENU: ([&str; 3], &str) = (["Cut", "Copy", "Paste"], "Select All");

/// `menu` holding `SAMPLE_MENU`, each item running `gpui::NoAction`.
fn sample_menu(menu: PopupMenu) -> PopupMenu {
    let (first, last) = SAMPLE_MENU;
    first
        .iter()
        .fold(menu, |menu, &label| {
            menu.menu(label, Box::new(gpui::NoAction))
        })
        .separator()
        .menu(last, Box::new(gpui::NoAction))
}

/// A Button of the variant `overlay` asks for, refined by `geometry::button`,
/// reading `label`; a click runs `on_click`, which opens `overlay`.
pub(crate) fn overlay_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    overlay: Overlay,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut button_info = info::overlays::trigger(cx.theme(), overlay, styled);
    let button = native_info(
        overlay.button_kind().apply(Button::new(id), cx),
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

/// The Overlays page's Dialog: `dialog`, titled, holding a dialog icon beside
/// a description, `gap` apart, over a footer whose Close Button closes it.
///
/// The Dialog reports itself on its title, its content and its footer; the
/// Button in the footer reports itself (spec §4.3.1).
pub(crate) fn confirm_dialog(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    dialog: Dialog,
    gap: Option<Pixels>,
) -> Dialog {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut dialog_info = info::overlays::dialog(cx.theme(), cx.reduce_motion(), styled);
    let dialog = dialog_frame(dialog, cx, &mut dialog_info);
    let title = dialog_title(cx, "Confirm Action", &mut dialog_info);
    // A `DialogDescription` is built anew for every frame, so what the
    // refinement records is recorded once, here, and applied there.
    let description_style = native_info(
        StyleRefinement::default(),
        cx,
        geometry::dialog_description,
        "dialog_description",
        &mut dialog_info,
    );
    if native_value(cx, geometry::icon_size_dialog).is_some() {
        dialog_info = dialog_info.geometry("icon_size_dialog");
    }
    if gap.is_some() {
        dialog_info = dialog_info.geometry("widget_gap");
    }
    let footer = native_info(
        DialogFooter::new(),
        cx,
        geometry::dialog_footer,
        "dialog_footer",
        &mut dialog_info,
    );
    let mut close_info = info::overlays::dialog_close(cx.theme(), styled);
    // DialogClose builds the Button and hands it over
    // (dialog/footer.rs:83-88), so the refinement is made here and applied
    // there.
    let close_style = native_info(
        StyleRefinement::default(),
        cx,
        geometry::button,
        "button",
        &mut close_info,
    );
    let close = DialogClose::new().trigger({
        let ui = ui.clone();
        move |button| {
            // In a row, so the Button's info is as wide as the Button:
            // DialogClose puts its trigger in a block, which would stretch
            // it across the footer (gpui-base dialog.rs, DialogClose).
            h_flex().justify_end().child(
                InfoExt::info(
                    button.refine_style(&close_style).label("Close"),
                    &ui,
                    OVERLAYS_DIALOG_CLOSE,
                    close_info,
                )
                .debug_selector(|| OVERLAYS_DIALOG_CLOSE.into()),
            )
        }
    });
    let title = title
        .info(ui, "overlays-dialog-title", dialog_info.clone())
        .debug_selector(|| "overlays-dialog-title".into());
    let footer = footer
        .child(close)
        .info(ui, OVERLAYS_DIALOG_FOOTER, dialog_info.clone())
        .debug_selector(|| OVERLAYS_DIALOG_FOOTER.into());
    let ui = ui.clone();
    dialog
        .title(title)
        .footer(footer)
        .content(move |content, _window, cx| {
            content.child(
                with_gap(h_flex(), gap)
                    .items_start()
                    .child(native_icon(
                        cx,
                        IconName::CircleX,
                        geometry::icon_size_dialog,
                    ))
                    .child(
                        DialogDescription::new()
                            .refine_style(&description_style)
                            .child("This cannot be undone."),
                    )
                    .info(&ui, "overlays-dialog-content", dialog_info.clone())
                    .debug_selector(|| "overlays-dialog-content".into()),
            )
        })
}

/// The Overlays page's AlertDialog: `alert`, refined by `geometry::dialog`
/// and capped at `geometry::dialog_max_width`, with a dialog icon, a title,
/// a description, and Keep and Discard buttons. It reports itself on the
/// icon, the title and the description, the parts the showcase builds.
pub(crate) fn alert_dialog(ui: &Entity<InfoRegistry>, cx: &App, alert: AlertDialog) -> AlertDialog {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut alert_info = info::overlays::alert_dialog(cx.theme(), cx.reduce_motion(), styled);
    let alert = native_info(alert, cx, geometry::dialog, "dialog", &mut alert_info);
    // An AlertDialog has no `max_w` of its own; `Styled::max_w` lands on the
    // surface through the refinement (dialog/dialog.rs:621).
    let alert = match native_value(cx, geometry::dialog_max_width) {
        Some(width) => {
            alert_info = alert_info.geometry("dialog_max_width");
            alert.max_w(width)
        }
        None => alert,
    };
    if native_value(cx, geometry::icon_size_dialog).is_some() {
        alert_info = alert_info.geometry("icon_size_dialog");
    }
    alert
        .icon(
            native_icon(cx, IconName::TriangleAlert, geometry::icon_size_dialog)
                .info(ui, "overlays-alert-dialog-icon", alert_info.clone())
                .debug_selector(|| "overlays-alert-dialog-icon".into()),
        )
        .title(
            "Discard changes?"
                .info(ui, "overlays-alert-dialog-title", alert_info.clone())
                .debug_selector(|| "overlays-alert-dialog-title".into()),
        )
        .description(
            "The edits made since the last save will be lost."
                .info(ui, "overlays-alert-dialog-description", alert_info)
                .debug_selector(|| "overlays-alert-dialog-description".into()),
        )
        .button_props(
            DialogButtonProps::default()
                .ok_text("Discard")
                .ok_variant(ButtonVariant::Danger)
                .cancel_text("Keep")
                .show_cancel(true),
        )
}

/// `sheet`, at the window's `side` edge, titled `title`: the one part of it
/// the showcase builds, so the Sheet reports itself there, as `id`.
pub(crate) fn sheet(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    sheet: Sheet,
    side: SheetSide,
    id: &'static str,
    title: &'static str,
) -> Sheet {
    sheet.title(
        title
            .info(
                ui,
                id,
                info::overlays::sheet(cx.theme(), side, cx.reduce_motion()),
            )
            .debug_selector(move || id.into()),
    )
}

/// The width the Overlays page's Popover content takes: the showcase's own,
/// as the model states no popover width.
const POPOVER_WIDTH: Pixels = px(200.);

/// A `Popover` refined by `geometry::popover`, opened by a Default Button
/// refined by `geometry::button`, holding a title over a muted line, `gap`
/// apart. The trigger and the content report the Popover: the surface
/// around the content is upstream's.
pub(crate) fn popover(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    gap: Option<Pixels>,
) -> Stateful<Div> {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut popover_info = info::overlays::popover(cx.theme(), styled);
    let trigger = native_info(
        Button::new("overlays-popover-trigger"),
        cx,
        geometry::button,
        "button",
        &mut popover_info,
    )
    .label("Click for Popover");
    let popover = native_info(
        Popover::new(id),
        cx,
        geometry::popover,
        "popover",
        &mut popover_info,
    )
    .trigger(trigger);
    if gap.is_some() {
        popover_info = popover_info.geometry("widget_gap");
    }
    let (content_ui, content_info) = (ui.clone(), popover_info.clone());
    popover
        .content(move |_state, _window, cx| {
            with_gap(v_flex(), gap)
                .w(POPOVER_WIDTH)
                .child(div().font_semibold().child("Popover Content"))
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child("This is a popover panel."),
                )
                .info(
                    &content_ui,
                    "overlays-popover-content",
                    content_info.clone(),
                )
                .debug_selector(|| "overlays-popover-content".into())
        })
        .info(ui, id, popover_info)
        .debug_selector(move || id.into())
}

/// The width the Overlays page's HoverCard content takes: the showcase's
/// own, as the model states no popover width.
const HOVER_CARD_WIDTH: Pixels = px(260.);

/// A `HoverCard` refined by `geometry::popover`, over a Button styled with
/// `variants::ghost_button` and refined by `geometry::button`, holding a
/// title over a muted line, `gap` apart. The card pads itself, with the
/// platform's popover padding where `geometry::popover` applies and upstream's
/// otherwise (popover.rs:284), so the content adds none. The trigger and the
/// content report the HoverCard.
pub(crate) fn hover_card(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    gap: Option<Pixels>,
) -> Stateful<Div> {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut card_info = info::overlays::hover_card(cx.theme(), styled);
    let trigger = native_info(
        ButtonKind::Ghost.apply(Button::new("overlays-hover-card-trigger"), cx),
        cx,
        geometry::button,
        "button",
        &mut card_info,
    )
    .label("KDE Breeze");
    let card = native_info(
        HoverCard::new(id),
        cx,
        geometry::popover,
        "popover",
        &mut card_info,
    )
    .trigger(trigger);
    if gap.is_some() {
        card_info = card_info.geometry("widget_gap");
    }
    let (content_ui, content_info) = (ui.clone(), card_info.clone());
    card.content(move |_state, _window, cx| {
        with_gap(v_flex(), gap)
            .w(HOVER_CARD_WIDTH)
            .child(div().font_semibold().child("KDE Breeze"))
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("The Plasma preset: the palette of Breeze's own colour schemes, and the Breeze icon theme."),
            )
            .info(&content_ui, "overlays-hover-card-content", content_info.clone())
            .debug_selector(|| "overlays-hover-card-content".into())
    })
    .info(ui, id, card_info)
    .debug_selector(move || id.into())
}

/// The Overlays page's ContextMenu: the showcase's own framed area, which a
/// right-click opens `SAMPLE_MENU` over. The menu's items are upstream's, on
/// a layer above the page, so the area reports the menu.
pub(crate) fn context_menu_area(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
) -> Stateful<Div> {
    let t = cx.theme();
    div()
        .id("overlays-context-menu-area")
        .p_6()
        .w_full()
        .demo_frame(cx)
        .bg(t.secondary)
        .child(
            div()
                .text_sm()
                .text_color(t.muted_foreground)
                .child("Right-click anywhere in this area"),
        )
        .context_menu(|menu, _window, _cx| sample_menu(menu))
        .info(ui, id, info::overlays::context_menu(t))
        .debug_selector(move || id.into())
}

/// A Default Button refined by `geometry::button`, reading `label`, whose
/// click opens `SAMPLE_MENU` as a dropdown menu. The menu's items are
/// upstream's, on a layer above the page, so the Button reports the menu.
pub(crate) fn dropdown_menu(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
) -> Stateful<Div> {
    let mut menu_info = info::overlays::dropdown_menu(cx.theme());
    native_info(
        Button::new(id),
        cx,
        geometry::button,
        "button",
        &mut menu_info,
    )
    .label(label)
    .dropdown_menu(|menu, _window, _cx| sample_menu(menu))
    .info(ui, id, menu_info)
    .debug_selector(move || id.into())
}

/// The width of the Overlays page's frame of application-drawn menu rows:
/// the showcase's own, as the model states no menu width.
const MENU_ROWS_WIDTH: Pixels = px(220.);

/// The menu rows an application draws itself, one per `(id, icon, label)`
/// of `rows`, in a frame painted as a menu is. Upstream's `MenuItemElement`
/// is crate-private and `PopupMenu` builds its own rows, so
/// `geometry::menu_item` has no widget to refine: these rows are the
/// receiver it documents.
pub(crate) fn menu_rows(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    rows: &[(&'static str, IconName, &'static str)],
) -> Div {
    let t = cx.theme();
    let (hover_bg, hover_text) = (t.accent, t.accent_foreground);
    v_flex()
        .w(MENU_ROWS_WIDTH)
        .bg(t.popover)
        .text_color(t.popover_foreground)
        .demo_frame(cx)
        .children(rows.iter().map(|(id, icon, label)| {
            let (id, label) = (*id, *label);
            let mut row_info = info::overlays::menu_row(t, label);
            if native_value(cx, geometry::icon_size_small).is_some() {
                row_info = row_info.geometry("icon_size_small");
            }
            // Plain text, not a Label: a Label paints foreground on its
            // own element (label.rs:211), over the frame's colour.
            let row = div()
                .flex()
                .items_center()
                .hover(move |this| this.bg(hover_bg).text_color(hover_text))
                .child(native_icon(cx, icon.clone(), geometry::icon_size_small))
                .child(label);
            native_info(row, cx, geometry::menu_item, "menu_item", &mut row_info)
                .info(ui, id, row_info)
                .debug_selector(move || id.into())
        }))
}

// ---------------------------------------------------------------------------
// The Charts page
// ---------------------------------------------------------------------------

/// The months the BarChart and the LineChart plot, with their values.
pub(crate) const SAMPLE_MONTHS: [(&str, f64); 6] = [
    ("Jan", 40.),
    ("Feb", 65.),
    ("Mar", 55.),
    ("Apr", 80.),
    ("May", 72.),
    ("Jun", 90.),
];

/// The months the AreaChart plots, with their values.
pub(crate) const SAMPLE_MONTHS_AREA: [(&str, f64); 6] = [
    ("Jan", 30.),
    ("Feb", 50.),
    ("Mar", 45.),
    ("Apr", 70.),
    ("May", 60.),
    ("Jun", 85.),
];

/// The days the CandlestickChart plots, as `(day, open, high, low, close)`.
pub(crate) const SAMPLE_OHLC: [(&str, f64, f64, f64, f64); 5] = [
    ("Mon", 100., 115., 95., 110.),
    ("Tue", 110., 120., 105., 108.),
    ("Wed", 108., 118., 100., 115.),
    ("Thu", 115., 125., 110., 112.),
    ("Fri", 112., 122., 108., 120.),
];

/// The PieChart's slices, as `(label, value)`, in the order they go round
/// the ring and take `chart_1`, `chart_2` and `chart_3`.
pub(crate) const PIE_SLICES: [(&str, f32); 3] =
    [("Desktop", 55.), ("Mobile", 30.), ("Tablet", 15.)];

/// The height of every chart on the Charts page but the PieChart: the
/// showcase's own, as the model states no chart size.
const CHART_HEIGHT: Pixels = px(220.);

/// The PieChart's square box: the showcase's own, as the model states no
/// chart size.
const PIE_SIZE: Pixels = px(250.);

/// The PieChart's inner and outer radii and the angle between its slices,
/// in pixels and radians: the showcase's own, as the model states no chart.
pub(crate) const PIE_INNER_RADIUS: f32 = 40.;
pub(crate) const PIE_OUTER_RADIUS: f32 = 100.;
pub(crate) const PIE_PAD_ANGLE: f32 = 0.03;

/// The strength the AreaChart's fill takes its series colour at: the
/// showcase's own, as the model states no chart.
pub(crate) const AREA_FILL_OPACITY: f32 = 0.3;

/// A `BarChart` of `SAMPLE_MONTHS`, its bars in `chart_1`.
pub(crate) fn bar_chart(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str) -> Stateful<Div> {
    let fill = cx.theme().chart_1;
    BarChart::new(SAMPLE_MONTHS)
        .band(|d: &(&'static str, f64)| d.0)
        .value(|d: &(&'static str, f64)| d.1)
        .fill(move |_: &(&'static str, f64), _, _, _| fill)
        .info(ui, id, info::charts::bar_chart(cx.theme()))
        .h(CHART_HEIGHT)
        .w_full()
        .debug_selector(move || id.into())
}

/// A `LineChart` of `SAMPLE_MONTHS`, its line and dots in `chart_2`.
pub(crate) fn line_chart(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str) -> Stateful<Div> {
    LineChart::new(SAMPLE_MONTHS)
        .x(|d: &(&'static str, f64)| d.0)
        .y(|d: &(&'static str, f64)| d.1)
        .stroke(cx.theme().chart_2)
        .dot()
        .info(ui, id, info::charts::line_chart(cx.theme()))
        .h(CHART_HEIGHT)
        .w_full()
        .debug_selector(move || id.into())
}

/// An `AreaChart` of `SAMPLE_MONTHS_AREA`: one series, its line in
/// `chart_3` over a fill of `chart_3` at `AREA_FILL_OPACITY`.
pub(crate) fn area_chart(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str) -> Stateful<Div> {
    let t = cx.theme();
    AreaChart::new(SAMPLE_MONTHS_AREA)
        .x(|d: &(&'static str, f64)| d.0)
        .y(|d: &(&'static str, f64)| d.1)
        .stroke(t.chart_3)
        .fill(t.chart_3.opacity(AREA_FILL_OPACITY))
        .info(ui, id, info::charts::area_chart(t))
        .h(CHART_HEIGHT)
        .w_full()
        .debug_selector(move || id.into())
}

/// A donut `PieChart` of `PIE_SLICES`, its slices in `chart_1`, `chart_2`
/// and `chart_3`.
pub(crate) fn pie_chart(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str) -> Stateful<Div> {
    let t = cx.theme();
    let slices: Vec<(f32, Hsla)> = PIE_SLICES
        .iter()
        .zip([t.chart_1, t.chart_2, t.chart_3])
        .map(|(&(_, value), color)| (value, color))
        .collect();
    PieChart::new(slices)
        .value(|d: &(f32, Hsla)| d.0)
        .color(|d: &(f32, Hsla)| d.1)
        .inner_radius(PIE_INNER_RADIUS)
        .outer_radius(PIE_OUTER_RADIUS)
        .pad_angle(PIE_PAD_ANGLE)
        .info(ui, id, info::charts::pie_chart(t))
        .size(PIE_SIZE)
        .debug_selector(move || id.into())
}

/// A `CandlestickChart` of `SAMPLE_OHLC`, in the theme's candle colours.
pub(crate) fn candlestick_chart(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
) -> Stateful<Div> {
    type Day = (&'static str, f64, f64, f64, f64);
    CandlestickChart::new(SAMPLE_OHLC)
        .x(|d: &Day| d.0)
        .open(|d: &Day| d.1)
        .high(|d: &Day| d.2)
        .low(|d: &Day| d.3)
        .close(|d: &Day| d.4)
        .info(ui, id, info::charts::candlestick_chart(cx.theme()))
        .h(CHART_HEIGHT)
        .w_full()
        .debug_selector(move || id.into())
}

// ---------------------------------------------------------------------------
// The Icons page
// ---------------------------------------------------------------------------

/// The size the Icons page draws an icon image at: the showcase's own. The
/// model's icon sizes each belong to a role -- small, large, toolbar,
/// panel, dialog -- and a gallery of every icon belongs to none of them.
pub(crate) const ICON_CELL_SIZE: Pixels = px(20.);

/// The size the Icons page draws an animated icon at: the showcase's own,
/// for the same reason.
pub(crate) const ANIMATED_ICON_SIZE: Pixels = px(32.);

/// What a cell of the Icons page's galleries draws, so the matches over it
/// in `info::icons` are exhaustive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum IconDrawn {
    /// gpui-component's own Icon.
    Builtin,
    /// A set bundled with native-theme, recoloured by the showcase.
    Bundled,
    /// The OS icon theme's own file.
    System,
    /// The set has no icon for it: the placeholder.
    Missing,
    /// The set has one that did not convert to an image: the placeholder.
    Unreadable,
}

/// What a cell of the Icons page's galleries draws, and from what.
pub(crate) enum IconArt {
    Builtin(IconName),
    Bundled(ImageSource),
    System(ImageSource),
    Missing,
    Unreadable,
}

impl IconArt {
    fn drawn(&self) -> IconDrawn {
        match self {
            Self::Builtin(_) => IconDrawn::Builtin,
            Self::Bundled(_) => IconDrawn::Bundled,
            Self::System(_) => IconDrawn::System,
            Self::Missing => IconDrawn::Missing,
            Self::Unreadable => IconDrawn::Unreadable,
        }
    }
}

/// One cell of an Icons page gallery: the icon a set gives for `label`,
/// labelled with it.
pub(crate) struct IconCell<'a> {
    /// The role's or the IconName's name.
    pub(crate) label: &'a str,
    /// The icon set, as the page names it.
    pub(crate) set: &'a str,
    pub(crate) art: IconArt,
}

/// The icon `art` draws above its `label`. A set that has no icon leaves
/// the placeholder the theme names for one, at the theme's own rounding --
/// never another set's icon.
fn icon_cell(cx: &App, art: IconArt, label: &str) -> Div {
    let t = cx.theme();
    let icon = match art {
        IconArt::Builtin(name) => div().child(Icon::new(name).with_size(Size::Medium)),
        IconArt::Bundled(source) | IconArt::System(source) => {
            div().child(gpui::img(source).size(ICON_CELL_SIZE))
        }
        IconArt::Missing | IconArt::Unreadable => {
            div().size(ICON_CELL_SIZE).bg(t.skeleton).rounded(t.radius)
        }
    };
    div()
        .flex()
        .flex_col()
        .items_center()
        .py_2()
        .px_2()
        .gap_1()
        .child(icon)
        .child(Label::new(label.to_string()).text_xs())
}

/// A cell of the Native Theme Icons grid: native-theme's icon for the role
/// `cell.label` names, which the set calls `name` where the showcase knows
/// it; `builtin` where the set is gpui-component's own. A recoloured icon
/// is painted in `fg`.
pub(crate) fn role_icon(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: impl Into<SharedString>,
    cell: IconCell<'_>,
    builtin: bool,
    name: Option<&str>,
    fg: Hsla,
) -> Stateful<Div> {
    let id: SharedString = id.into();
    let drawn = cell.art.drawn();
    let info = info::icons::role_icon(cx.theme(), cell.label, cell.set, builtin, name, drawn, fg);
    icon_cell(cx, cell.art, cell.label)
        .info(ui, id.clone(), info)
        .debug_selector(move || id.to_string())
}

/// A cell of the gpui-component Icons grid: the icon the set gives the
/// IconName `cell.label` names, found by `role` where one maps to it. A
/// recoloured icon is painted in `fg`.
pub(crate) fn gpui_icon(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: impl Into<SharedString>,
    cell: IconCell<'_>,
    role: Option<&str>,
    fg: Hsla,
) -> Stateful<Div> {
    let id: SharedString = id.into();
    let drawn = cell.art.drawn();
    let info = info::icons::gpui_icon(cx.theme(), cell.label, role, cell.set, drawn, fg);
    icon_cell(cx, cell.art, cell.label)
        .info(ui, id.clone(), info)
        .debug_selector(move || id.to_string())
}

/// The icon contexts `defaults.icon_sizes` names, in the model's order, so
/// the matches over them are exhaustive.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum IconSizeContext {
    Toolbar,
    Small,
    Large,
    Dialog,
    Panel,
}

impl IconSizeContext {
    pub(crate) const ALL: [Self; 5] = [
        Self::Toolbar,
        Self::Small,
        Self::Large,
        Self::Dialog,
        Self::Panel,
    ];

    /// The context's name, as `defaults.icon_sizes` spells its field.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Toolbar => "toolbar",
            Self::Small => "small",
            Self::Large => "large",
            Self::Dialog => "dialog",
            Self::Panel => "panel",
        }
    }

    /// The builder that sizes an icon for the context.
    pub(crate) fn builder(self) -> fn(Native<'_>) -> Size {
        match self {
            Self::Toolbar => geometry::icon_size_toolbar,
            Self::Small => geometry::icon_size_small,
            Self::Large => geometry::icon_size_large,
            Self::Dialog => geometry::icon_size_dialog,
            Self::Panel => geometry::icon_size_panel,
        }
    }

    /// The id and debug selector of the context's cell in the Icon Sizes
    /// section.
    pub(crate) const fn cell(self) -> &'static str {
        match self {
            Self::Toolbar => "icons-size-toolbar",
            Self::Small => "icons-size-small",
            Self::Large => "icons-size-large",
            Self::Dialog => "icons-size-dialog",
            Self::Panel => "icons-size-panel",
        }
    }

    /// The debug selector of the box the cell's icon is laid out in, which
    /// is as large as the icon.
    pub(crate) const fn icon_box(self) -> &'static str {
        match self {
            Self::Toolbar => "icons-size-toolbar-icon",
            Self::Small => "icons-size-small-icon",
            Self::Large => "icons-size-large-icon",
            Self::Dialog => "icons-size-dialog-icon",
            Self::Panel => "icons-size-panel-icon",
        }
    }
}

/// A cell of the Icon Sizes section: `drawn`'s icon for `icon`, of the icon
/// set named `set`, at the size `context` names, through that context's
/// `geometry::icon_size_*` builder, above the context's name, under the
/// theme installed as `preset`. Where the set
/// has no such icon the cell shows the name alone, never another set's
/// icon.
pub(crate) fn icon_size_cell(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    context: IconSizeContext,
    drawn: &ChromeIcon,
    icon: &IconName,
    set: &str,
    preset: &str,
) -> Stateful<Div> {
    let size = match native_value(cx, context.builder()) {
        Some(Size::Size(size)) => Some(size.as_f32()),
        _ => None,
    };
    let mut cell_info = info::icons::icon_size(cx.theme(), context, drawn, set, size, preset);
    let icon = chrome_icon(drawn, icon).map(|icon| native_sized(cx, icon, context.builder()));
    if icon.is_some() && size.is_some() {
        cell_info = match context {
            IconSizeContext::Toolbar => cell_info.geometry("icon_size_toolbar"),
            IconSizeContext::Small => cell_info.geometry("icon_size_small"),
            IconSizeContext::Large => cell_info.geometry("icon_size_large"),
            IconSizeContext::Dialog => cell_info.geometry("icon_size_dialog"),
            IconSizeContext::Panel => cell_info.geometry("icon_size_panel"),
        };
    }
    let id = context.cell();
    div()
        .flex()
        .flex_col()
        .items_center()
        .py_2()
        .px_2()
        .gap_1()
        .when_some(icon, |cell, icon| {
            cell.child(
                div()
                    .debug_selector(move || context.icon_box().into())
                    .child(icon),
            )
        })
        .child(Label::new(context.name()).text_xs())
        .info(ui, id, cell_info)
        .debug_selector(move || id.into())
}

/// The animations the Icons page shows, so the matches over them in
/// `info::icons` are exhaustive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum AnimatedKind {
    /// Frame by frame: `count` frames, `duration_ms` each.
    Frames { count: usize, duration_ms: u32 },
    /// One SVG turned a full circle every `duration_ms`.
    Spin { duration_ms: u32 },
}

/// An animated icon, with what the page draws it from.
pub(crate) enum AnimatedArt<'a> {
    /// The frame to show now, of `count`.
    Frames {
        frame: ImageSource,
        count: usize,
        duration_ms: u32,
    },
    /// The SVG to turn.
    Spin { svg: &'a [u8], duration_ms: u32 },
}

/// A card holding the animated icon `set` ships as `art`, in the
/// showcase's frame, labelled with what it is. A bundled set's frames are
/// recoloured with `fg`.
pub(crate) fn animated_icon(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: impl Into<SharedString>,
    set: &str,
    art: AnimatedArt<'_>,
    bundled: bool,
    fg: Hsla,
) -> Stateful<Div> {
    let id: SharedString = id.into();
    let t = cx.theme();
    let reduce_motion = cx.reduce_motion();
    let (kind, icon, what) = match art {
        AnimatedArt::Frames {
            frame,
            count,
            duration_ms,
        } => (
            AnimatedKind::Frames { count, duration_ms },
            gpui::img(frame).size(ANIMATED_ICON_SIZE).into_any_element(),
            format!("Frames: {count} ({duration_ms}ms)"),
        ),
        // An SVG element paints every shape in its text colour, and gpui
        // turns it where it would not turn an image (gpui-pre
        // elements/svg.rs, Svg::with_transformation).
        AnimatedArt::Spin { svg, duration_ms } => (
            AnimatedKind::Spin { duration_ms },
            with_spin_animation(
                gpui::svg()
                    .data(svg)
                    .size(ANIMATED_ICON_SIZE)
                    .text_color(t.foreground),
                SharedString::from(format!("{id}-spin")),
                duration_ms,
            )
            .into_any_element(),
            format!("Spin ({duration_ms}ms)"),
        ),
    };
    let label = if reduce_motion {
        format!("{set} - {what} (reduced motion)")
    } else {
        format!("{set} - {what}")
    };
    v_flex()
        .items_center()
        .gap_2()
        .p_4()
        .demo_frame(cx)
        .child(icon)
        .child(Label::new(label).text_xs())
        .info(
            ui,
            id.clone(),
            info::icons::animated_icon(t, set, kind, bundled, fg, reduce_motion),
        )
        .debug_selector(move || id.to_string())
}

// ---------------------------------------------------------------------------
// The Theme Map page
// ---------------------------------------------------------------------------

/// The ThemeColor fields the Theme Map shows, a swatch each, so the match
/// over them in `info::theme_map` is exhaustive and the compiler rejects a
/// field without its row.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ThemeToken {
    Background,
    Foreground,
    Accent,
    AccentForeground,
    Border,
    Muted,
    MutedForeground,
    Input,
    Ring,
    Selection,
    Caret,
    Link,
    LinkHover,
    LinkActive,
    Overlay,
    Primary,
    PrimaryForeground,
    PrimaryHover,
    PrimaryActive,
    Secondary,
    SecondaryForeground,
    SecondaryHover,
    SecondaryActive,
    Button,
    ButtonHover,
    ButtonActive,
    ButtonForeground,
    ButtonSecondary,
    ButtonSecondaryHover,
    ButtonSecondaryActive,
    ButtonSecondaryForeground,
    ButtonPrimary,
    ButtonPrimaryHover,
    ButtonPrimaryActive,
    ButtonPrimaryForeground,
    ButtonDanger,
    ButtonDangerHover,
    ButtonDangerActive,
    ButtonDangerForeground,
    ButtonInfo,
    ButtonInfoHover,
    ButtonInfoActive,
    ButtonInfoForeground,
    ButtonSuccess,
    ButtonSuccessHover,
    ButtonSuccessActive,
    ButtonSuccessForeground,
    ButtonWarning,
    ButtonWarningHover,
    ButtonWarningActive,
    ButtonWarningForeground,
    Danger,
    DangerForeground,
    DangerHover,
    DangerActive,
    Red,
    RedLight,
    Success,
    SuccessForeground,
    SuccessHover,
    SuccessActive,
    Green,
    GreenLight,
    Warning,
    WarningForeground,
    WarningHover,
    WarningActive,
    Yellow,
    YellowLight,
    Info,
    InfoForeground,
    InfoHover,
    InfoActive,
    Blue,
    BlueLight,
    List,
    ListActive,
    ListActiveBorder,
    ListEven,
    ListHead,
    ListHover,
    Table,
    TableActive,
    TableActiveBorder,
    TableEven,
    TableHead,
    TableHeadForeground,
    TableFoot,
    TableFootForeground,
    TableHover,
    TableRowBorder,
    Tab,
    TabActive,
    TabActiveForeground,
    TabBar,
    TabBarSegmented,
    TabForeground,
    Sidebar,
    SidebarForeground,
    SidebarAccent,
    SidebarAccentForeground,
    SidebarBorder,
    SidebarPrimary,
    SidebarPrimaryForeground,
    Scrollbar,
    ScrollbarThumb,
    ScrollbarThumbHover,
    Accordion,
    GroupBox,
    GroupBoxForeground,
    Chart1,
    Chart2,
    Chart3,
    Chart4,
    Chart5,
    ChartBullish,
    ChartBearish,
    DescriptionListLabel,
    DescriptionListLabelForeground,
    DragBorder,
    DropTarget,
    Popover,
    PopoverForeground,
    ProgressBar,
    Skeleton,
    SliderBar,
    SliderThumb,
    Switch,
    SwitchThumb,
    StatusBar,
    StatusBarBorder,
    TitleBar,
    TitleBarBorder,
    WindowBorder,
    Magenta,
    MagentaLight,
    Cyan,
    CyanLight,
}

/// A Theme Map swatch of `token`: the installed value of its field in
/// `frame`, the showcase's frame, which the page builds once, labelled with
/// the field's name and the value. Its id is `theme-map-swatch-<field>`.
pub(crate) fn swatch(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    frame: &StyleRefinement,
    token: ThemeToken,
) -> Stateful<Div> {
    let t = cx.theme();
    // What the connector writes the colours under.
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let value = info::theme_map::row(t, token).value;
    let id = SharedString::from(format!("theme-map-swatch-{}", value.field));
    let label = SharedString::from(format!("{} {}", value.field, hsla_to_hex(value.value)));
    // The fill is the datum this swatch exists to show; everything around it
    // is `frame`, the box every other demonstration sits in.
    h_flex()
        .gap_2()
        .items_center()
        .child(refined(div().size(px(16.0)).bg(value.value), Some(frame)))
        .child(Label::new(label).text_sm())
        .info(ui, id.clone(), info::theme_map::swatch(t, token, native))
        .debug_selector(move || id.to_string())
}
