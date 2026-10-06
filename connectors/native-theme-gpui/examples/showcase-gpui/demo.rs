//! Demo helpers the pages share.

use std::{cell::Cell, rc::Rc, time::Duration};

use gpui::DefiniteLength;
use gpui::{
    Action, AnyElement, App, Axis, ClickEvent, ClipboardItem, Context, Div, ElementId, Entity,
    FontWeight, Global, Hsla, ImageSource, KeyDownEvent, Keystroke, MouseButton, Pixels, Rems,
    RenderOnce, SharedString, Stateful, StyleRefinement, Window, anchored, deferred, div,
    prelude::*, px, relative, rems,
};
use gpui_base::{ResizeHandleContext, ResizeHandleRenderer};
use gpui_component::{
    ActiveTheme, ChildElement, Collapsible, Disableable as _, Icon, IconName, Selectable, Sizable,
    Size, StyledExt as _, ThemeStyled as _, TitleBar, WindowExt as _,
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
    color_picker::{ColorPicker, ColorPickerState, ColorSelect},
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
        InputGroupButton, InputGroupText, InputGroupTextarea, InputState, InputToken, NumberInput,
        OtpInput, OtpState, Textarea, TextareaState,
    },
    kbd::Kbd,
    label::Label,
    link::Link,
    list::{List, ListItem, ListState},
    marker::{Marker, MarkerContent, MarkerIcon, MarkerLoadingStyle, MarkerVariant},
    menu::{ContextMenuExt as _, DropdownMenu as _, PopupMenu, PopupMenuItem},
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
    speech::{SpeechState, SpeechWaveform},
    spinner::Spinner,
    status_bar::StatusBar,
    stepper::{Stepper, StepperItem},
    switch::Switch,
    tab::{Tab, TabBar},
    table::{DataTable, Table, TableBody, TableCell, TableHead, TableHeader, TableRow, TableState},
    tag::{Tag, TagVariant},
    text::TextView,
    time_field::{TimeField, TimeFieldState},
    toolbar::{Toolbar, ToolbarGroup},
    tooltip::Tooltip,
    tree::{Tree, TreeState},
    v_flex,
};
use native_theme::theme::{ArrowSide, IconSet, ResolvedFontSpec, ResolvedPadding};
use native_theme_gpui::icons::with_spin_animation;
#[cfg(feature = "widgets")]
use native_theme_gpui::widgets;
use native_theme_gpui::{
    AccessibilityPreferences, ActiveNativeTheme as _, Native, geometry, variants,
};

use crate::app::{Quit, ShowPage};
use crate::elements::{self, ListedExt as _};
use crate::info::{self, InfoExt, InfoRegistry, WidgetInfo, hsla_to_hex, native_info};
use crate::support::{
    CAROUSEL_SLIDES, ChatMessage, ChromeIcon, NativeStyled as _, PresetDelegate, STEPPER_STEPS,
    SampleIcon, SampleListDelegate, SampleTableDelegate, native_color, native_geometry,
    native_value, refined, with_gap, with_padding,
};
use crate::{
    CHROME_APP_MENU_BAR, CHROME_SIDE_PANEL, CHROME_SPLITTER_LINE, CHROME_THEME_SETTINGS,
    DATA_TABLE_HEADER, OVERLAY_ABOUT_LINK, OVERLAY_ABOUT_NAME, OVERLAY_ABOUT_TEXT,
    OVERLAY_ABOUT_TITLE, OVERLAY_PALETTE, OVERLAY_PALETTE_TITLE, OVERLAY_PREFERENCES,
    OVERLAYS_DIALOG_CLOSE, OVERLAYS_DIALOG_FOOTER, PREF_HIGH_CONTRAST, PREF_REDUCE_MOTION,
    PREF_REDUCE_TRANSPARENCY, PROBE_CAROUSEL_LAST, PROBE_SETTINGS_ROW, Page, STATUS_ENVIRONMENT,
    STATUS_HOVERED, STATUS_MIDDLE, TREE_DEMO, probe,
};

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

impl SampleIcon {
    /// The `Icon` a page sample draws, as `chrome_icon` gives it: `None`
    /// where the chosen icon theme has none. A part: the sample that shows
    /// it reports.
    fn icon(&self) -> Option<Icon> {
        chrome_icon(&self.drawn, &self.icon)
    }

    /// The `Icon` at the platform's size for the role the builder names, as
    /// `native_sized` sizes it; `None` where the chosen icon theme has none.
    fn sized(&self, cx: &App, role: fn(Native<'_>) -> Size) -> Option<Icon> {
        self.icon().map(|icon| native_sized(cx, icon, role))
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
/// the application's menus (`app_menus`) where the platform has no menu bar
/// of its own, and quitting
/// the application from its close button: the window's title bar, where the
/// window was granted client-side decorations (spec S8).
pub(crate) fn title_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    label: impl Into<SharedString>,
    menus: &Entity<MenuBar>,
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
        bar.child(app_menus(ui, cx, info::MenuHost::TitleBar, menus))
    });
    bar.info(ui, "chrome-title-bar", bar_info)
}

/// A menu title's side and top/bottom padding where `menu.border.padding`
/// states no side: upstream's for a title of its `AppMenuBar`, a Small
/// compact Button (button/button.rs:672-674, `px_1p5`) given `py_0p5`
/// (menu/app_menu_bar.rs:266).
const MENU_TITLE_PADDING_X: Rems = rems(0.375);
const MENU_TITLE_PADDING_Y: Rems = rems(0.125);

/// A menu row's side padding where `menu.border.padding` states no side,
/// and its height where neither `menu.row_height` nor a vertical padding
/// side is stated: upstream's `PopupMenu` row at the default Size
/// (menu/popup_menu.rs:1216, `INNER_PADDING`; :1221, the item height).
const MENU_ROW_PADDING_X: Pixels = px(8.);
const MENU_ROW_HEIGHT: Pixels = px(26.);

/// What upstream's `PopupMenu` leaves round and between its rows, and
/// between a row's label and its shortcut (menu/popup_menu.rs:1487-1489,
/// `p_1`, `gap_y_0p5`, `min_w(rems(8.))`; :1325, `gap_3`); the model states
/// none of them.
const MENU_POPUP_PADDING: Rems = rems(0.25);
const MENU_POPUP_ROW_GAP: Rems = rems(0.125);
const MENU_POPUP_MIN_WIDTH: Rems = rems(8.);
const MENU_SHORTCUT_GAP: Rems = rems(0.75);

/// A menu separator's thickness without a native theme: upstream's
/// `PopupMenu` separator (menu/popup_menu.rs:1256, `border_b(px(2.))`).
/// With one, it is `separator.line_width`.
const MENU_SEPARATOR: Pixels = px(2.);

/// The two blurred layers of the popup's drop shadow where
/// `popover.border.shadow_enabled` holds, as (y offset, blur radius, spread
/// radius), drawn in `defaults.shadow_color`: the model states no shadow
/// geometry, so these are gpui-component's popup surface shadow's
/// (gpui-component 0.7.1 src/styled.rs:68-73, `popover_shadow`), without its
/// ring (:65-67), whose place the frame in `popover.border` takes.
const MENU_POPUP_SHADOW: [(Pixels, Pixels, Pixels); 2] =
    [(px(4.), px(3.), px(-1.)), (px(2.), px(2.), px(-2.))];

/// The model's menu, as the showcase draws a menu title and a menu row:
/// `menu.font`, `menu.hover_background` and `hover_text_color`,
/// `menu.border`'s padding sides and corner radius (platform-facts §2.6: the
/// items are rectangular where it is 0), `menu.row_height`, and the popup
/// `menu.background_color` framed by the popover's border (§2.6: the popup
/// border is §2.16's).
#[derive(Clone)]
struct MenuLook {
    family: SharedString,
    font_size: Pixels,
    weight: FontWeight,
    /// `defaults.line_height`, the platform's line box, as the connector's
    /// control-height rule lays text out (geometry.rs, `with_height_rule`).
    line_height: DefiniteLength,
    text: Hsla,
    hover: Hsla,
    hover_text: Hsla,
    radius: Pixels,
    padding: ResolvedPadding,
    row_height: Option<Pixels>,
    background: Hsla,
    separator: Hsla,
    separator_width: Pixels,
    frame: Hsla,
    frame_width: Pixels,
    frame_radius: Pixels,
    /// `popover.border.padding`: round the popup's rows.
    popup_padding: ResolvedPadding,
    /// The popup's drop shadow ([`MENU_POPUP_SHADOW`]), none where
    /// `popover.border.shadow_enabled` does not hold.
    popup_shadow: Vec<gpui::BoxShadow>,
}

impl MenuLook {
    fn of(n: &Native<'_>) -> Self {
        let m = &n.resolved.menu;
        let p = &n.resolved.popover.border;
        Self {
            family: native_theme_gpui::font_family(&m.font.family),
            font_size: px(native_theme_gpui::scaled_text_size(
                m.font.size,
                n.accessibility,
            )),
            weight: FontWeight(f32::from(m.font.weight)),
            line_height: relative(n.resolved.defaults.line_height),
            text: info::stated(m.font.color),
            hover: info::stated(m.hover_background),
            hover_text: info::stated(m.hover_text_color),
            radius: px(m.border.corner_radius.max(0.0)),
            padding: m.border.padding,
            row_height: m.row_height.map(px),
            background: info::stated(m.background_color),
            separator: info::stated(m.separator_color),
            separator_width: px(n.resolved.separator.line_width),
            frame: info::stated(p.color),
            frame_width: px(p.line_width),
            frame_radius: px(p.corner_radius.max(0.0)),
            popup_padding: p.padding,
            popup_shadow: if p.shadow_enabled {
                let ink = info::stated(n.resolved.defaults.shadow_color);
                MENU_POPUP_SHADOW
                    .iter()
                    .map(|&(y, blur, spread)| {
                        gpui::BoxShadow::new(px(0.), y, ink)
                            .blur_radius(blur)
                            .spread_radius(spread)
                    })
                    .collect()
            } else {
                Vec::new()
            },
        }
    }
}

/// `el` padded by the stated sides of `padding`, and on a side left
/// unstated by `x` (left and right) or `y` (top and bottom), where given.
fn padded<E: Styled>(
    el: E,
    padding: &ResolvedPadding,
    x: impl Into<DefiniteLength> + Copy,
    y: Option<Rems>,
) -> E {
    let el = match padding.left {
        Some(v) => el.pl(px(v)),
        None => el.pl(x),
    };
    let el = match padding.right {
        Some(v) => el.pr(px(v)),
        None => el.pr(x),
    };
    let el = match (padding.top, y) {
        (Some(v), _) => el.pt(px(v)),
        (None, Some(y)) => el.pt(y),
        (None, None) => el,
    };
    match (padding.bottom, y) {
        (Some(v), _) => el.pb(px(v)),
        (None, Some(y)) => el.pb(y),
        (None, None) => el,
    }
}

/// A menu's title in the showcase's menu bar: the Popover's trigger,
/// selected while its menu is open.
#[derive(IntoElement)]
struct MenuTitle {
    ix: usize,
    name: SharedString,
    look: Option<MenuLook>,
    selected: bool,
}

impl Selectable for MenuTitle {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl RenderOnce for MenuTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let title = div()
            .id(("menu-title", self.ix))
            .flex()
            .items_center()
            .cursor_default()
            .debug_selector({
                let name = self.name.clone();
                move || format!("menu-title-{name}")
            })
            .child(self.name);
        match self.look {
            Some(look) => {
                let (hover, hover_text) = (look.hover, look.hover_text);
                padded(
                    title,
                    &look.padding,
                    MENU_TITLE_PADDING_X,
                    Some(MENU_TITLE_PADDING_Y),
                )
                .font_family(look.family.clone())
                .text_size(look.font_size)
                .line_height(look.line_height)
                .font_weight(look.weight)
                .rounded(look.radius)
                .map(|title| {
                    if self.selected {
                        title.bg(hover).text_color(hover_text)
                    } else {
                        title
                            .text_color(look.text)
                            .hover(move |style| style.bg(hover).text_color(hover_text))
                    }
                })
            }
            None => title
                .px(MENU_TITLE_PADDING_X)
                .py(MENU_TITLE_PADDING_Y)
                .text_sm(),
        }
    }
}

/// One row of a menu the showcase draws.
enum MenuRow {
    Separator,
    Action(SharedString, Box<dyn Action>),
}

/// The application's menus (`chrome::menus`), each as its title and rows.
fn app_menu_rows() -> Vec<(SharedString, Vec<MenuRow>)> {
    crate::chrome::menus()
        .into_iter()
        .map(|menu| {
            let rows = menu
                .items
                .into_iter()
                .filter_map(|item| match item {
                    gpui::MenuItem::Separator => Some(MenuRow::Separator),
                    gpui::MenuItem::Action { name, action, .. } => {
                        Some(MenuRow::Action(name, action))
                    }
                    _ => None,
                })
                .collect();
            (menu.name, rows)
        })
        .collect()
}

/// The next of `menu`'s rows a key moves the highlight to from `from`:
/// forward (`down`) or back, over the separators, wrapping at the ends, and
/// the first (or last) where nothing is highlighted yet.
fn next_row(menu: &[MenuRow], from: Option<usize>, forward: bool) -> Option<usize> {
    let actions: Vec<usize> = menu
        .iter()
        .enumerate()
        .filter(|(_, row)| matches!(row, MenuRow::Action(..)))
        .map(|(ix, _)| ix)
        .collect();
    let at = from.and_then(|from| actions.iter().position(|&ix| ix == from));
    let next = match (at, forward) {
        (None, true) => Some(0),
        (None, false) => actions.len().checked_sub(1),
        (Some(at), true) => (at + 1).checked_rem(actions.len()),
        (Some(at), false) => (at + actions.len() - 1).checked_rem(actions.len()),
    };
    next.and_then(|at| actions.get(at).copied())
}

/// The application's menus (`chrome::menus`) as the showcase draws them: a
/// title per menu, each opening its popup under it, with the keyboard of
/// upstream's `AppMenuBar` and `PopupMenu` (menu/app_menu_bar.rs,
/// `AppMenuBar`: Left and Right move to the neighbouring menu, Escape closes
/// it; menu/popup_menu.rs, `PopupMenu`: Up and Down move between the items,
/// Enter runs one). An open menu holds the keyboard focus and hands it back
/// on closing, as `AppMenuBar` restores its action context, so an item's
/// action is dispatched from the element that had the focus.
///
/// Not upstream's `AppMenuBar`: its titles are Small ghost Buttons and its
/// menus `PopupMenu`s, which set `text_sm`, a rounded hover in `accent` and
/// the `popover` fill on themselves (menu/app_menu_bar.rs,
/// `AppMenu::render`; menu/popup_menu.rs, `PopupMenu::render_item`), so the
/// model's `menu.font`, its rectangular items and `menu.background_color`
/// would not reach them. The row that holds it reports it
/// ([`app_menus`]).
pub(crate) struct MenuBar {
    /// Where the titles and the Theme menu's rows report their layout.
    ui: Entity<InfoRegistry>,
    open: Option<usize>,
    highlighted: Option<usize>,
    focus: gpui::FocusHandle,
    restore: Option<gpui::FocusHandle>,
}

impl MenuBar {
    pub(crate) fn new(ui: Entity<InfoRegistry>, cx: &mut Context<Self>) -> Self {
        Self {
            ui,
            open: None,
            highlighted: None,
            focus: cx.focus_handle(),
            restore: None,
        }
    }

    /// The open menu, by its index in `chrome::menus`.
    #[cfg(test)]
    pub(crate) fn open_menu(&self) -> Option<usize> {
        self.open
    }

    /// Opens the menu titled `name`, compared without case (`--open-menu`);
    /// an error naming the menus where none is titled so.
    pub(crate) fn open_named(
        &mut self,
        name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), String> {
        let menus = app_menu_rows();
        match menus
            .iter()
            .position(|(title, _)| title.eq_ignore_ascii_case(name))
        {
            Some(ix) => {
                self.open(ix, window, cx);
                Ok(())
            }
            None => {
                let names: Vec<String> = menus.iter().map(|(t, _)| t.to_lowercase()).collect();
                Err(format!(
                    "--open-menu {name}: no such menu; the menus are: {}",
                    names.join(", ")
                ))
            }
        }
    }

    fn open(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.open.is_none() {
            self.restore = window.focused(cx);
            self.focus.focus(window, cx);
        }
        self.open = Some(ix);
        self.highlighted = None;
        cx.notify();
    }

    fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.open.take().is_none() {
            return;
        }
        self.highlighted = None;
        match self.restore.take() {
            Some(focus) => focus.focus(window, cx),
            None => window.blur(cx),
        }
        cx.notify();
    }

    /// Close the menu, then dispatch `action` from the focus it handed back.
    fn run(&mut self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        let action = action.boxed_clone();
        self.close(window, cx);
        window.dispatch_action(action, cx);
    }

    fn on_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(open) = self.open else {
            return;
        };
        let menus = app_menu_rows();
        let count = menus.len();
        let rows = menus
            .get(open)
            .map(|(_, rows)| rows.as_slice())
            .unwrap_or(&[]);
        match event.keystroke.key.as_str() {
            "escape" => self.close(window, cx),
            "left" => {
                if let Some(ix) = (open + count.saturating_sub(1)).checked_rem(count) {
                    self.open(ix, window, cx);
                }
            }
            "right" => {
                if let Some(ix) = (open + 1).checked_rem(count) {
                    self.open(ix, window, cx);
                }
            }
            "down" | "up" => {
                self.highlighted = next_row(rows, self.highlighted, event.keystroke.key == "down");
                cx.notify();
            }
            "enter" => {
                let action = self.highlighted.and_then(|ix| match rows.get(ix) {
                    Some(MenuRow::Action(_, action)) => Some(action.boxed_clone()),
                    _ => None,
                });
                if let Some(action) = action {
                    self.run(action.as_ref(), window, cx);
                }
            }
            _ => return,
        }
        cx.stop_propagation();
    }

    /// The popup of the open menu: its `rows`, drawn in `look` where a
    /// native theme is installed; a click, or Enter on the highlighted row,
    /// runs a row's action and closes the popup, and a press outside it
    /// closes it.
    fn popup(
        &self,
        rows: &[MenuRow],
        look: Option<&MenuLook>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let highlighted = self.highlighted;
        let ui = &self.ui;
        let listed = |ix: usize| {
            self.open
                .filter(|&open| open == THEME_MENU)
                .and_then(|_| THEME_MENU_ROWS.get(ix).copied())
        };
        let body = match look {
            Some(look) => padded(
                v_flex(),
                &look.popup_padding,
                MENU_POPUP_PADDING,
                Some(MENU_POPUP_PADDING),
            ),
            None => v_flex().p(MENU_POPUP_PADDING),
        };
        let body = body
            .gap(MENU_POPUP_ROW_GAP)
            .min_w(MENU_POPUP_MIN_WIDTH)
            .children(rows.iter().enumerate().map(|(ix, row)| {
                match row {
                    MenuRow::Separator => div()
                        .h(look.map_or(MENU_SEPARATOR, |l| l.separator_width))
                        .w_full()
                        .bg(look.map_or(cx.theme().border, |l| l.separator))
                        .children(listed(ix).map(|id| elements::record(ui, id)))
                        .into_any_element(),
                    MenuRow::Action(name, action) => {
                        let shortcut = Kbd::global_binding_for_action(action.as_ref(), window)
                            .map(|kbd| kbd.appearance(false))
                            .map(|kbd| match listed(ix) {
                                Some(THEME_MENU_PREFERENCES) => kbd
                                    .listed(ui, THEME_MENU_PREFERENCES_SHORTCUT)
                                    .into_any_element(),
                                _ => kbd.into_any_element(),
                            });
                        let dispatched = action.boxed_clone();
                        let lit = highlighted == Some(ix);
                        let row = h_flex()
                            .id(("menu-row", ix))
                            .w_full()
                            .gap(MENU_SHORTCUT_GAP)
                            .justify_between()
                            .items_center()
                            .cursor_default()
                            .debug_selector({
                                let name = name.clone();
                                move || format!("menu-row-{name}")
                            })
                            .child(name.clone())
                            .children(shortcut)
                            .children(listed(ix).map(|id| elements::record(ui, id)))
                            .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                                if *hovered && this.highlighted != Some(ix) {
                                    this.highlighted = Some(ix);
                                    cx.notify();
                                }
                            }))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.run(dispatched.as_ref(), window, cx);
                            }));
                        let row = match look {
                            Some(look) => {
                                // The shortcut is the plain text of a Kbd
                                // without its appearance (kbd.rs:234-236),
                                // which takes the row's font and line box.
                                let row = padded(row, &look.padding, MENU_ROW_PADDING_X, None)
                                    .font_family(look.family.clone())
                                    .text_size(look.font_size)
                                    .line_height(look.line_height)
                                    .font_weight(look.weight)
                                    .rounded(look.radius)
                                    .map(|row| {
                                        if lit {
                                            row.bg(look.hover).text_color(look.hover_text)
                                        } else {
                                            row.text_color(look.text)
                                        }
                                    });
                                match (look.row_height, look.padding.top, look.padding.bottom) {
                                    (Some(height), _, _) => row.min_h(height),
                                    (None, None, None) => row.h(MENU_ROW_HEIGHT),
                                    (None, _, _) => row,
                                }
                            }
                            None => row
                                .px(MENU_ROW_PADDING_X)
                                .h(MENU_ROW_HEIGHT)
                                .text_sm()
                                .when(lit, |row| {
                                    row.bg(cx.theme().accent)
                                        .text_color(cx.theme().accent_foreground)
                                }),
                        };
                        row.into_any_element()
                    }
                }
            }));
        let frame = div()
            .occlude()
            .popover_style(cx)
            .p_0()
            .on_mouse_down_out(cx.listener(|this, _, window, cx| this.close(window, cx)))
            .child(body);
        let frame = match look {
            Some(look) => frame
                .bg(look.background)
                .border(look.frame_width)
                .border_color(look.frame)
                .rounded(look.frame_radius)
                .shadow(look.popup_shadow.clone()),
            None => frame,
        };
        match self.open {
            Some(THEME_MENU) => frame.listed(ui, THEME_MENU_FRAME).into_any_element(),
            _ => frame.into_any_element(),
        }
    }
}

/// The Theme menu's index in `chrome::menus`: the one menu whose rows the
/// three showcases share (docs/showcase-elements.toml).
const THEME_MENU: usize = 2;

/// The Theme menu's popup and its rows, in `chrome::menus` order, as the
/// elements of docs/showcase-elements.toml they are.
const THEME_MENU_FRAME: &str = "chrome.menu.theme";
const THEME_MENU_ROWS: [&str; 7] = [
    "chrome.menu.theme.reload",
    "chrome.menu.theme.separator_1",
    "chrome.menu.theme.system",
    "chrome.menu.theme.light",
    "chrome.menu.theme.dark",
    "chrome.menu.theme.separator_2",
    THEME_MENU_PREFERENCES,
];
const THEME_MENU_PREFERENCES: &str = "chrome.menu.theme.preferences";
const THEME_MENU_PREFERENCES_SHORTCUT: &str = "chrome.menu.theme.preferences.shortcut";

/// The menu titles, in `chrome::menus` order, as the elements of
/// docs/showcase-elements.toml they are.
const MENU_TITLES: [&str; 4] = [
    "chrome.menu_bar.file",
    "chrome.menu_bar.view",
    "chrome.menu_bar.theme",
    "chrome.menu_bar.help",
];

impl Render for MenuBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let native = cx.native_theme().and_then(|nt| nt.native(cx));
        let look = native.as_ref().map(MenuLook::of);
        let open = self.open;
        let menus = app_menu_rows();
        let mut popup = open.and_then(|ix| menus.get(ix)).map(|(_, rows)| {
            self.popup(rows, look.as_ref(), window, cx)
                .into_any_element()
        });
        let ui = self.ui.clone();
        h_flex()
            .id("app-menus")
            .track_focus(&self.focus)
            .on_key_down(cx.listener(Self::on_key))
            .children(menus.into_iter().enumerate().map(|(ix, (name, _))| {
                let selected = open == Some(ix);
                div()
                    .id(("app-menu", ix))
                    .relative()
                    .child(MenuTitle {
                        ix,
                        name,
                        look: look.clone(),
                        selected,
                    })
                    .children(MENU_TITLES.get(ix).map(|id| elements::record(&ui, id)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            window.prevent_default();
                            cx.stop_propagation();
                            if selected {
                                this.close(window, cx);
                            } else {
                                this.open(ix, window, cx);
                            }
                        }),
                    )
                    .on_hover(cx.listener(move |this, hovered: &bool, window, cx| {
                        if *hovered && this.open.is_some() && this.open != Some(ix) {
                            this.open(ix, window, cx);
                        }
                    }))
                    .when_some(selected.then(|| popup.take()).flatten(), |menu, popup| {
                        menu.child(deferred(
                            anchored()
                                .anchor(gpui::Anchor::TopLeft)
                                .snap_to_window_with_margin(MENU_POPUP_WINDOW_MARGIN)
                                .child(popup),
                        ))
                    })
            }))
    }
}

/// How near the window's edge a menu's popup may come: upstream's
/// `AppMenu` popup's (menu/app_menu_bar.rs, `AppMenu::render`,
/// `snap_to_window_with_margin(px(8.))`).
const MENU_POPUP_WINDOW_MARGIN: Pixels = px(8.);

/// The application's menus, `menus`, reporting themselves as the menus of
/// `host`. A part: the helper that places it reports its host.
fn app_menus(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    host: info::MenuHost,
    menus: &Entity<MenuBar>,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let menus_info = info::app_menus(cx.theme(), host, native.as_ref().map(|n| n.resolved));
    div()
        .child(menus.clone())
        .info(ui, "chrome-app-menu-bar", menus_info)
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
/// manager draws, the application's own row holding its menus (`app_menus`),
/// as a KDE application places its menus. The model states no menu-bar
/// inset, so its sides borrow `container_margin`, the installed layout's
/// `geometry::container_margin`, and where that is unstated take
/// [`MENU_BAR_PADDING`]; the menu titles set its height.
pub(crate) fn menu_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    container_margin: Option<Pixels>,
    menus: &Entity<MenuBar>,
) -> Stateful<Div> {
    // No geometry line: the model states no menu-bar inset, and the row only
    // borrows container_margin, which its info says in its own words.
    let row_info = info::menu_bar(cx.theme(), container_margin, MENU_BAR_PADDING);
    h_flex()
        .px(container_margin.unwrap_or(MENU_BAR_PADDING))
        .child(app_menus(ui, cx, info::MenuHost::Row, menus))
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

/// The gap between the toolbar row's items where the theme leaves it
/// unstated twice over: neither `toolbar.item_gap` nor `layout.widget_gap`
/// states it. The row is the showcase's own element, with no toolkit default
/// to fall back on, so this is the showcase's own choice, not a platform's
/// value, and the row's info says so.
pub(crate) const TOOLBAR_GAP: Pixels = px(4.);

/// The window's toolbar (spec §2.3): the application's own row, refined by
/// `geometry::toolbar`, holding `items`.
///
/// A side `toolbar.border` leaves unstated is padded with `container_margin`,
/// the installed layout's `geometry::container_margin`, and where that is
/// unstated too with [`TOOLBAR_PADDING`] (spec §3.1). Where
/// `toolbar.item_gap` is unstated the items are `widget_gap` apart, the
/// installed layout's `geometry::widget_gap`, and where that is unstated too
/// [`TOOLBAR_GAP`]. Both go on first, so the refinement's stated sides and
/// gap land over them.
pub(crate) fn toolbar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    container_margin: Option<Pixels>,
    widget_gap: Option<Pixels>,
    items: impl IntoIterator<Item = AnyElement>,
) -> Stateful<Div> {
    let stated = native_value(cx, |n| n.resolved.toolbar.border.padding).unwrap_or_default();
    let item_gap = native_value(cx, |n| n.resolved.toolbar.item_gap).flatten();
    let mut row_info = info::toolbar(
        stated,
        container_margin,
        TOOLBAR_PADDING,
        info::ToolbarGap {
            stated: item_gap,
            widget_gap,
            own: TOOLBAR_GAP,
        },
    );
    let unstated = [stated.top, stated.right, stated.bottom, stated.left]
        .iter()
        .any(Option::is_none);
    if unstated && container_margin.is_some() {
        row_info = row_info.geometry("container_margin");
    }
    if item_gap.is_none() && widget_gap.is_some() {
        row_info = row_info.geometry("widget_gap");
    }
    let row = h_flex()
        .p(container_margin.unwrap_or(TOOLBAR_PADDING))
        .gap(widget_gap.unwrap_or(TOOLBAR_GAP));
    // Its bottom edge `toolbar.border.line_width` thick in
    // `toolbar.border.color`, inside the bar, as the status bar's top edge
    // is (`status_bar`).
    let edge = native_value(cx, |n| {
        let b = &n.resolved.toolbar.border;
        (px(b.line_width), info::stated(b.color))
    });
    let row = native_info(row, cx, geometry::toolbar, "toolbar", &mut row_info)
        .when_some(edge, |row, (width, colour)| {
            row.border_b(width).border_color(colour)
        })
        .children(items);
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
    widget_gap: Option<Pixels>,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut bar_info = info::status_bar(cx.theme(), styled);
    let text_line = native_value(cx, |n| line_height_of(&n, &n.resolved.status_bar.font));
    // Its top edge `status_bar.border.line_width` thick, over upstream's
    // `border_t_1` (status_bar.rs, `RenderOnce for StatusBar`), which the
    // bar's style is refined over.
    let edge = native_value(cx, |n| px(n.resolved.status_bar.border.line_width));
    if let Some(edge) = edge {
        bar_info = bar_info.config(
            "top edge",
            format!(
                "status_bar.border.line_width, {}px",
                info::px_text(edge.as_f32())
            ),
        );
    }
    let bar = native_info(
        StatusBar::new(),
        cx,
        geometry::status_bar,
        "status_bar",
        &mut bar_info,
    )
    .when_some(edge, |bar, edge| bar.border_t(edge))
    // Its text one line of `status_bar.font` tall (`line_height_of`).
    .when_some(text_line, |bar, line| bar.line_height(line))
    // The toggle and the environment as one left item, `layout.widget_gap`
    // apart: the left region spaces its items by a `gap_2` of its own
    // (status_bar.rs, `RenderOnce for StatusBar`), which no style reaches.
    // Plain text, not Labels, as in the title bar: a Label would paint
    // foreground over the colour `geometry::status_bar` gives the bar.
    .left(
        with_gap(h_flex(), widget_gap)
            .items_center()
            .child(toggle)
            .child(
                div()
                    .relative()
                    .debug_selector(|| STATUS_ENVIRONMENT.into())
                    .child(environment.into())
                    .child(elements::record(ui, "chrome.status_bar.environment")),
            ),
    )
    // The middle region holds nothing; this empty box fills it, so its
    // edges are where the two ends stop.
    .child(div().flex_1().debug_selector(|| STATUS_MIDDLE.into()))
    .when_some(shown, |bar, title| {
        bar.right(
            div()
                .relative()
                .debug_selector(|| STATUS_HOVERED.into())
                .child(title)
                .child(elements::record(ui, "chrome.status_bar.shown")),
        )
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
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let mut combobox_info = info::preset_combobox(cx.theme(), native.as_ref().map(|n| n.resolved));
    let combobox = native_info(
        Combobox::new(state)
            .placeholder("Pick a preset…")
            .search_placeholder("Filter by name or key…"),
        cx,
        geometry::combobox,
        "combobox",
        &mut combobox_info,
    );
    let combobox = refined(combobox, combo_fill(cx).as_ref()).w_full();
    combo_surface(cx, combobox)
        .w_full()
        .info(ui, "chrome-settings-preset", combobox_info)
}

/// The colour-mode `Select` over `state`, refined by `geometry::select`, as
/// wide as the theme settings it is in: System, Light and Dark, and the
/// choice dispatches `SetColorMode` (`Showcase::new`).
pub(crate) fn color_mode_select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let mut select_info = info::color_mode_select(cx.theme(), native.as_ref().map(|n| n.resolved));
    let select = native_info(
        Select::new(state),
        cx,
        geometry::select,
        "select",
        &mut select_info,
    );
    let select = refined(select, combo_fill(cx).as_ref()).w_full();
    combo_surface(cx, select)
        .w_full()
        .info(ui, "chrome-settings-color-mode", select_info)
}

/// The icon-theme `Select` over `state`, refined by `geometry::select`, as
/// wide as the theme settings it is in.
pub(crate) fn icon_set_select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let mut select_info = info::icon_set_select(cx.theme(), native.as_ref().map(|n| n.resolved));
    let select = native_info(
        Select::new(state),
        cx,
        geometry::select,
        "select",
        &mut select_info,
    );
    let select = refined(select, combo_fill(cx).as_ref()).w_full();
    combo_surface(cx, select)
        .w_full()
        .info(ui, "chrome-settings-icon-theme", select_info)
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
/// (button/button.rs:615-618, 725-729), and the size an `Icon` is given last
/// wins over its style (icon.rs:187-193).
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
    let (tip, styled) = action_tooltip(cx, tooltip, action);
    let mut button_info = info::toolbar_button(cx.theme(), about, &drawn, set, styled);
    if styled {
        button_info = button_info.geometry("tooltip");
    }
    let icon =
        chrome_icon(&drawn, &icon).map(|icon| native_sized(cx, icon, geometry::icon_size_toolbar));
    if icon.is_some() && native_value(cx, geometry::icon_size_toolbar).is_some() {
        button_info = button_info.geometry("icon_size_toolbar");
    }
    let dispatched = action.boxed_clone();
    let button = refined(
        ButtonKind::Ghost.apply(Button::new(id), cx),
        tool_button_box(cx).as_ref(),
    )
    .map(|button| match icon {
        Some(icon) => button.child(listed_icon(ui, icon, elements::icon_of(id))),
        None => labelled(ui, cx, button, id, tooltip),
    })
    .on_click(move |_, window, cx| window.dispatch_action(dispatched.boxed_clone(), cx));
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(
        button,
        ui,
        SharedString::from(format!("chrome-button-{id}")),
        button_info,
    )
    .tooltip(tip)
}

/// One ghost icon Button of the Buttons page's component `Toolbar`.
pub(crate) struct ToolItem {
    pub id: &'static str,
    /// gpui-component's icon of the button, which `drawn` is of.
    pub icon: IconName,
    /// That icon as the chosen icon theme gives it.
    pub drawn: ChromeIcon,
    /// The button's name: its accessible label, and its text where the
    /// chosen icon theme has no icon for it.
    pub label: &'static str,
}

/// The Buttons page's component `Toolbar` (spec §8.2): gpui-component
/// 0.7.1's own, refined by `geometry::toolbar` alone -- the height, fill,
/// font, gap and padding sides the theme states, and no fallback or edge,
/// since the section shows the widget and not the chrome's row -- holding
/// the `edit` and `view` Buttons in two labelled `ToolbarGroup`s with a
/// vertical `Separator` between them. Each group spaces its Buttons by
/// `toolbar.item_gap` where the theme states one, else by the Toolbar's own
/// Small gap, `gap_1` (toolbar.rs, `RenderOnce for Toolbar`): a group spaces
/// nothing itself (gpui-base toolbar.rs, `RenderOnce for ToolbarGroup`).
///
/// Every item goes in through `content()`: `child()` would make a Button a
/// compact ghost and wrap it in an `input_h` box (toolbar.rs, `ToolbarItem`;
/// button/button.rs, `Button::prepare_for_toolbar`).
pub(crate) fn component_toolbar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    set: &str,
    edit: Vec<ToolItem>,
    view: Vec<ToolItem>,
) -> Stateful<Div> {
    let item_gap = native_value(cx, |n| n.resolved.toolbar.item_gap).flatten();
    let group = |id: &'static str, label: &'static str, items: Vec<ToolItem>| {
        let group = ToolbarGroup::new(id).label(label);
        let group = match item_gap {
            Some(gap) => group.gap(px(gap)),
            None => group.gap_1(),
        };
        items.into_iter().fold(group, |group, item| {
            group.content(toolbar_item(ui, cx, set, item))
        })
    };
    let mut toolbar_info = info::buttons::toolbar(item_gap);
    let toolbar = native_info(
        Toolbar::new("buttons-toolbar-row"),
        cx,
        geometry::toolbar,
        "toolbar",
        &mut toolbar_info,
    )
    .content(group("buttons-toolbar-edit", "Edit", edit))
    .content(separator(
        ui,
        cx,
        "buttons-toolbar-separator",
        SeparatorKind::Vertical,
    ))
    .content(group("buttons-toolbar-view", "View", view));
    toolbar.info(ui, "buttons-toolbar", toolbar_info)
}

/// One of the component `Toolbar`'s Buttons: a Ghost `Button` built as
/// [`toolbar_button`] builds the chrome's, without an action or a tooltip,
/// its icon of the chosen icon theme at `geometry::icon_size_toolbar`, and
/// its label as its text where the icon theme has no icon for it, never
/// another icon theme's icon.
fn toolbar_item(ui: &Entity<InfoRegistry>, cx: &App, set: &str, item: ToolItem) -> Stateful<Div> {
    let ToolItem {
        id,
        icon,
        drawn,
        label,
    } = item;
    let mut button_info = info::buttons::toolbar_item(cx.theme(), &drawn, set);
    let icon =
        chrome_icon(&drawn, &icon).map(|icon| native_sized(cx, icon, geometry::icon_size_toolbar));
    if icon.is_some() && native_value(cx, geometry::icon_size_toolbar).is_some() {
        button_info = button_info.geometry("icon_size_toolbar");
    }
    let button = refined(
        ButtonKind::Ghost.apply(Button::new(id), cx),
        tool_button_box(cx).as_ref(),
    )
    .map(|button| match icon {
        Some(icon) => button.accessibility_label(label).child(icon),
        None => labelled(ui, cx, button, id, label),
    });
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into())
}

/// A tooltip reading `text` and the key binding of `action`, built by the
/// application and refined by `geometry::tooltip`, for the element that
/// wraps a chrome Button; and whether a native theme refined it.
///
/// Not `Button::tooltip_with_action`: the Button builds that Tooltip itself
/// as it renders (button/button.rs, `RenderOnce for Button`), so the
/// platform's tooltip fill, edge, padding, radius and text colour would not
/// reach it. The text is a content element refined by
/// `geometry::tooltip_content`, which is where `tooltip.max_width` makes it
/// wrap: `Tooltip::new(text)` has no element to carry the width.
fn action_tooltip(
    cx: &App,
    text: &'static str,
    action: &dyn Action,
) -> (
    impl Fn(&mut Window, &mut App) -> gpui::AnyView + 'static,
    bool,
) {
    let style = native_geometry(cx, geometry::tooltip);
    let content = native_geometry(cx, geometry::tooltip_content);
    let styled = style.is_some();
    let action = action.boxed_clone();
    let build = move |window: &mut Window, cx: &mut App| {
        let content = content.clone();
        refined(
            Tooltip::element(move |_window, _cx| refined(div().child(text), content.as_ref()))
                .action(action.as_ref(), None),
            style.as_ref(),
        )
        .build(window, cx)
    };
    (build, styled)
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
    let (tip, styled) = action_tooltip(cx, tooltip, action);
    let mut button_info = info::panel_toggle(cx.theme(), about, &drawn, set, open, state, styled);
    if styled {
        button_info = button_info.geometry("tooltip");
    }
    let icon =
        chrome_icon(&drawn, &icon).map(|icon| native_sized(cx, icon, geometry::icon_size_small));
    if icon.is_some() && native_value(cx, geometry::icon_size_small).is_some() {
        button_info = button_info.geometry("icon_size_small");
    }
    let dispatched = action.boxed_clone();
    let button = refined(
        ButtonKind::Ghost.apply(Button::new(id), cx).small(),
        tool_button_box(cx).as_ref(),
    )
    .selected(open)
    .toggled(open)
    .map(|button| match icon {
        Some(icon) => button.child(listed_icon(ui, icon, elements::icon_of(id))),
        None => labelled(ui, cx, button, id, tooltip),
    })
    .on_click(move |_, window, cx| window.dispatch_action(dispatched.boxed_clone(), cx));
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(
        button,
        ui,
        SharedString::from(format!("chrome-button-{id}")),
        button_info,
    )
    .tooltip(tip)
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
                .child(sidebar_label(ui, cx, id, text))
                .child(control)
        }))
        .info(ui, CHROME_THEME_SETTINGS, settings_info)
        .w_full()
        .min_w_0()
        .debug_selector(|| CHROME_THEME_SETTINGS.into())
}

/// A label of the side panel reading `text`, `id` its info's id and debug
/// selector: plain text in `sidebar.font`'s size and weight where a native
/// theme is installed, in the colour the side panel sets. Not a `Label`:
/// `Label::render` paints `foreground` on its own element (label.rs:211),
/// over the sidebar's text colour.
fn sidebar_label(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: &'static str,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let text_div = match &native {
        Some(n) => font_text(n, &n.resolved.sidebar.font, text),
        None => div().text_sm().child(text),
    };
    text_div
        .info(
            ui,
            id,
            info::sidebar_label(cx.theme(), native.as_ref().map(|n| n.resolved)),
        )
        .self_start()
        .debug_selector(move || id.into())
}

/// The side panel (spec S2): `settings`, padded by `container_margin` where
/// the installed layout states one, over `separator`, over `inspector`, which
/// fills the rest of the panel's height and scrolls its own content; filled
/// with the model's sidebar colour and lettered in its font's colour.
///
/// Plain elements, not upstream's `Sidebar`: a Sidebar's children must
/// implement `SidebarItem` (sidebar/mod.rs:211), and neither the settings
/// nor the inspector is an item.
pub(crate) fn side_panel(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    container_margin: Option<Pixels>,
    settings: impl IntoElement,
    separator: impl IntoElement,
    inspector: impl IntoElement,
) -> Stateful<Div> {
    let mut panel_info = info::side_panel(cx.theme(), container_margin);
    if container_margin.is_some() {
        panel_info = panel_info.geometry("container_margin");
    }
    let own = sidebar_padding(cx);
    let side = |stated: Option<f32>| stated.map(px).or(container_margin);
    // The model's sidebar: `sidebar.background_color` and `sidebar.font`'s
    // colour, which the connector installs as the `sidebar` and
    // `sidebar_foreground` tokens.
    v_flex()
        .size_full()
        .bg(cx.theme().sidebar)
        .text_color(cx.theme().sidebar_foreground)
        .when_some(own, |panel, p| {
            panel
                .when_some(side(p.top), |el, v| el.pt(v))
                .when_some(side(p.right), |el, v| el.pr(v))
                .when_some(side(p.bottom), |el, v| el.pb(v))
                .when_some(side(p.left), |el, v| el.pl(v))
        })
        .child(
            with_padding(
                div().w_full(),
                side_panel_content_margin(cx, container_margin),
            )
            .child(settings),
        )
        .child(separator)
        .child(div().w_full().flex_1().min_h_0().child(inspector))
        .info(ui, CHROME_SIDE_PANEL, panel_info)
        .size_full()
}

/// `sidebar.border.padding`, where the theme states any side of it: the
/// side panel's own padding. It then pads the whole panel, a side it leaves
/// unstated by `layout.container_margin`, and the panel's content adds none
/// ([`side_panel_content_margin`]), as the egui showcase's side panel takes
/// it. `None` where it states no side.
pub(crate) fn sidebar_padding(cx: &App) -> Option<ResolvedPadding> {
    native_value(cx, |n| n.resolved.sidebar.border.padding).filter(|p| {
        [p.top, p.right, p.bottom, p.left]
            .iter()
            .any(Option::is_some)
    })
}

/// The padding of the side panel's settings and of the inspector's content:
/// `layout.container_margin`, or none where the theme states the panel's own
/// padding ([`sidebar_padding`]), which pads them already.
pub(crate) fn side_panel_content_margin(
    cx: &App,
    container_margin: Option<Pixels>,
) -> Option<Pixels> {
    match sidebar_padding(cx) {
        Some(_) => None,
        None => container_margin,
    }
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
/// expanded item's `h_7` row (sidebar/menu.rs:323) nor the 48px rail
/// (sidebar/mod.rs:28). `SidebarMenuItem` keeps the icon it is given
/// (sidebar/menu.rs:315). The size goes on the Icon's style, which upstream
/// lays out as it would the same size given through `with_size`
/// (icon.rs:177-188), so a test can read it off the Icon this returns.
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
#[allow(clippy::too_many_arguments)]
pub(crate) fn tab_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    kind: TabBarKind,
    container_margin: Option<Pixels>,
    tabs: impl IntoIterator<Item = (&'static str, &'static str)>,
    selected: usize,
    menu_icon: Option<&SampleIcon>,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let menu_icon = menu_icon.and_then(SampleIcon::icon);
    #[cfg(not(feature = "widgets"))]
    let _ = &menu_icon;
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
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let tabs: Vec<(&'static str, &'static str)> = tabs.into_iter().collect();
    #[cfg(feature = "widgets")]
    if let Some(n) = &native {
        return native_tab_bar(
            ui,
            cx,
            n,
            kind,
            container_margin,
            tabs,
            selected,
            menu_icon,
            on_click,
        );
    }
    let mut bar_info = info::tab_font(bar_info, native.as_ref().map(|n| n.resolved))
        .instance("padding", info::tab_bar_padding(container_margin));
    if container_margin.is_some() {
        bar_info = bar_info.geometry("container_margin");
    }
    let labels: Vec<&'static str> = tabs.iter().map(|&(label, _)| label).collect();
    let on_click = Rc::new(on_click);
    let on_tab = on_click.clone();
    let bar = TabBar::new(id)
        .underline()
        .with_size(Size::Small)
        .when_some(container_margin, |bar, margin| bar.px(margin))
        .children(tabs.into_iter().map(|(label, selector)| {
            // The label as a child in `tab.font`, over the `text_sm` the Tab
            // sets on itself (tab/tab.rs, `RenderOnce for Tab`), and the
            // tab at least `tab.min_width` by `tab.min_height`, through its
            // style, which its own `h` does not clear.
            let tab = match &native {
                Some(n) => {
                    let t = &n.resolved.tab;
                    Tab::new()
                        .child(font_text(n, &t.font, label))
                        .min_w(px(t.min_width))
                        .min_h(px(t.min_height))
                }
                None => Tab::new().label(label),
            };
            tab.debug_selector(move || selector.into())
        }))
        .selected_index(selected)
        .on_click(move |ix, window, cx| on_tab(ix, window, cx));
    // Upstream's menu of every tab names each by its `label` (tab/tab_bar.rs,
    // `RenderOnce for TabBar`), which a tab whose label is a child has not
    // got, so the showcase builds the same Button and menu as the bar's
    // suffix.
    let bar = if menu {
        bar.suffix(
            Button::new("page-tabs-menu")
                .xsmall()
                .ghost()
                .dropdown_caret(true)
                .dropdown_menu(move |menu, _, _| {
                    labels
                        .iter()
                        .enumerate()
                        .fold(menu.scrollable(true), |menu, (ix, &label)| {
                            let on_click = on_click.clone();
                            menu.item(
                                PopupMenuItem::new(label)
                                    .checked(selected == ix)
                                    .on_click(move |_, window, cx| on_click(&ix, window, cx)),
                            )
                        })
                })
                .anchor(gpui::Anchor::TopRight),
        )
    } else {
        bar
    };
    bar.info(ui, info_id, bar_info)
}

/// What a click on the tab at an index runs.
#[cfg(feature = "widgets")]
type TabClick = Rc<dyn Fn(&usize, &mut Window, &mut App)>;

/// What a click on an expander's title runs, with the items' next state.
type ExpanderToggle = Rc<dyn Fn(&[bool; 2], &mut Window, &mut App)>;

/// `tabs`, each `(label, debug selector)`, as `widgets::Tab`s.
#[cfg(feature = "widgets")]
fn native_tabs(tabs: &[(&'static str, &'static str)]) -> Vec<widgets::Tab> {
    tabs.iter()
        .map(|&(label, selector)| widgets::Tab::new(label).debug_selector(|| selector.into()))
        .collect()
}

/// A tab row under a native theme: the connector's `widgets::TabBar`, which
/// draws exactly what `tab.*` states (ISSUES D7). The model states no
/// tab-bar rule; the page row alone is parted from the page by the theme's
/// `separator.*` line, under it (`native_tab_bar`), as the iced and egui
/// showcases part theirs.
#[cfg(feature = "widgets")]
fn native_tab_strip(_n: &Native<'_>, id: &'static str) -> widgets::TabBar {
    widgets::TabBar::new(id)
}

/// [`tab_bar`] under a native theme: a [`native_tab_strip`], inset by
/// `container_margin` as upstream's bar is (the rule runs its full width),
/// the page row's tabs scrolling sideways where they do not fit, with the
/// menu of every page after them, showing `menu_icon`.
#[cfg(feature = "widgets")]
#[allow(clippy::too_many_arguments)]
fn native_tab_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    n: &Native<'_>,
    kind: TabBarKind,
    container_margin: Option<Pixels>,
    tabs: Vec<(&'static str, &'static str)>,
    selected: usize,
    menu_icon: Option<Icon>,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let (id, info_id, menu, variant) = match kind {
        TabBarKind::Inspector => (
            "inspector-tabs",
            "chrome-inspector-tabs",
            false,
            NATIVE_TAB_BAR,
        ),
        TabBarKind::Pages => ("page-tabs", "chrome-page-tabs", true, NATIVE_TAB_BAR_MENU),
    };
    let mut bar_info = info::layout::native_tab_row(n.resolved, variant)
        .instance("padding", info::tab_bar_padding(container_margin));
    if container_margin.is_some() {
        bar_info = bar_info.geometry("container_margin");
    }
    let labels: Vec<&'static str> = tabs.iter().map(|&(label, _)| label).collect();
    let on_click: TabClick = Rc::new(on_click);
    let on_tab = on_click.clone();
    let tool_box = Some(tool_box_of(*n));
    let ghost = variants::ghost_button(cx);
    let menu = menu.then(|| {
        // The chosen set's ChevronDown at `defaults.icon_sizes.small`, where
        // the set has one, in place of upstream's caret, which draws
        // gpui-component's own glyph at a size of its own (button/button.rs,
        // `Caret`); upstream's caret where the set has none, the button
        // otherwise empty.
        refined(
            Button::new("page-tabs-menu").xsmall().custom(ghost),
            tool_box.as_ref(),
        )
        .accessibility_label("Pages")
        .map(|button| match menu_icon {
            Some(icon) => button.child(listed_icon(
                ui,
                icon.with_size(px(n.resolved.defaults.icon_sizes.small)),
                Some("chrome.page_tabs.menu.icon"),
            )),
            None => button.dropdown_caret(true),
        })
        .dropdown_menu(move |menu, _, _| {
            labels
                .iter()
                .enumerate()
                .fold(menu.scrollable(true), |menu, (ix, &label)| {
                    let on_click = on_click.clone();
                    menu.item(
                        PopupMenuItem::new(label)
                            .checked(selected == ix)
                            .on_click(move |_, window, cx| on_click(&ix, window, cx)),
                    )
                })
        })
        .anchor(gpui::Anchor::TopRight)
    });
    native_tab_strip(n, id)
        .w_full()
        .when_some(container_margin, |bar, margin| bar.px(margin))
        .when_some(elements::part_observer(ui, info_id), |bar, observer| {
            bar.on_part_bounds(observer)
        })
        .children(native_tabs(&tabs))
        .selected_index(selected)
        .on_click(move |ix, window, cx| on_tab(ix, window, cx))
        .when_some(menu, |bar, menu| {
            bar.suffix(menu.listed(ui, "chrome.page_tabs.menu"))
        })
        .info(ui, info_id, bar_info)
        // The page row's rule, `separator.line_width` thick in
        // `separator.line_color`, under the strip: the wrapper keeps the room
        // below it, where the rule is laid.
        .when(kind == TabBarKind::Pages, |bar| {
            let s = &n.resolved.separator;
            let width = px(s.line_width);
            bar.mb(width).child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .bottom(-width)
                    .h(width)
                    .bg(info::stated(s.line_color))
                    .child(elements::record(ui, "chrome.page_tabs.rule")),
            )
        })
}

/// The variant a theme-drawn tab row's info names.
#[cfg(feature = "widgets")]
pub(crate) const NATIVE_TAB_BAR: &str = "widgets::TabBar";
/// The same, for the page row with its menu.
#[cfg(feature = "widgets")]
pub(crate) const NATIVE_TAB_BAR_MENU: &str = "widgets::TabBar, menu";

/// `text` in `font`, at the size the text-scaling factor makes of it: a
/// label a widget's own text size would otherwise set, as its child.
fn font_text(n: &Native<'_>, font: &ResolvedFontSpec, text: &'static str) -> Div {
    div()
        .text_size(text_size_of(n, font))
        .line_height(line_height_of(n, font))
        .font_weight(FontWeight(f32::from(font.weight)))
        .child(text)
}

/// `font`'s size, scaled by the text-scaling factor, as the connector's
/// builders scale theirs.
fn text_size_of(n: &Native<'_>, font: &ResolvedFontSpec) -> Pixels {
    px(native_theme_gpui::scaled_text_size(
        font.size,
        n.accessibility,
    ))
}

/// One line of `font`: its scaled size by `defaults.line_height`, the line
/// box the model states for text (docs/property-registry.toml,
/// `defaults.line_height`), in place of gpui's own, which is the golden
/// ratio (gpui-pre style.rs, `phi`).
fn line_height_of(n: &Native<'_>, font: &ResolvedFontSpec) -> Pixels {
    text_size_of(n, font) * n.resolved.defaults.line_height
}

/// A `TabBar` of upstream's default variant, `TabVariant::Tab`, over
/// `labels`, `selected` the one shown; a click hands `on_click` the index of
/// the tab clicked.
///
/// What the theme states reaches it per call where upstream leaves a seam:
/// each label is a child in `tab.font`'s size and weight over the `text_sm`
/// the Tab sets on itself (tab/tab.rs, `RenderOnce for Tab`), and each tab
/// is at least `tab.min_width` wide and `tab.min_height` tall through the
/// Tab's style, which its own `h` does not clear.
pub(crate) fn tab_row(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
    selected: usize,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    #[cfg(feature = "widgets")]
    if let Some(n) = &native {
        let tabs: Vec<(&'static str, &'static str)> =
            labels.iter().map(|&label| (label, label)).collect();
        let shown = labels.get(selected).copied().unwrap_or("none");
        let row_info = info::layout::native_tab_row(n.resolved, NATIVE_TAB_BAR)
            .instance("tabs", labels.join(", "))
            .instance("selected", shown)
            .instance("click", "selects the tab; the showcase keeps the state");
        return native_tab_strip(n, id)
            .wrap()
            .when_some(elements::part_observer(ui, id), |bar, observer| {
                bar.on_part_bounds(observer)
            })
            .children(native_tabs(&tabs))
            .selected_index(selected)
            .on_click(on_click)
            .info(ui, id, row_info)
            .debug_selector(move || id.into());
    }
    let row_info = info::layout::tab_row(
        cx.theme(),
        labels,
        selected,
        native.as_ref().map(|n| n.resolved),
    );
    let tabs = labels.iter().map(|&label| match &native {
        Some(n) => {
            let t = &n.resolved.tab;
            Tab::new()
                .child(font_text(n, &t.font, label))
                .min_w(px(t.min_width))
                .min_h(px(t.min_height))
        }
        None => Tab::new().label(label),
    });
    TabBar::new(id)
        .children(tabs)
        .selected_index(selected)
        .on_click(on_click)
        .info(ui, id, row_info)
        .debug_selector(move || id.into())
}

/// A segment's side padding where `segmented_control.border` states none:
/// upstream's own for a segment at the default Size (tab/tab.rs:73-80,
/// `TabVariant::inner_paddings`), which a `TabBar::segmented` would give it.
const SEGMENT_PADDING: Pixels = px(12.);

/// A segmented control over `labels`, `selected` the one shown; a click
/// hands `on_click` the index of the segment clicked.
///
/// Drawn by the showcase where a native theme is installed: upstream's
/// segmented `TabBar` paints its selected segment with the window's
/// background through a sliding indicator and its hovered one inside
/// render (tab/tab_bar.rs, `TabBar::render_indicator`; tab/tab.rs,
/// `TabVariant::hovered`), and labels its segments from the tab tokens, so
/// `segmented_control.active_background`, `hover_background`, `font.color`
/// and `active_text_color` have no receiver there. Here every segment is
/// `segmented_control.segment_height` tall at least, padded by the stated
/// `border.padding` sides ([`SEGMENT_PADDING`] on a side left unstated), in
/// `font`; the selected one filled with `active_background` and labelled in
/// `active_text_color`; the others filled with `hover_background` under the
/// pointer; neighbours parted by a `separator_width` line in
/// `border.color`; the whole framed by `border` on `background_color`.
/// Without a native theme it is upstream's segmented `TabBar`.
pub(crate) fn segmented(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
    selected: usize,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let control_info = info::layout::segmented(
        cx.theme(),
        labels,
        selected,
        native.as_ref().map(|n| n.resolved),
    );
    let Some(n) = native else {
        return TabBar::new(id)
            .segmented()
            .children(labels.iter().map(|&label| Tab::new().label(label)))
            .selected_index(selected)
            .on_click(on_click)
            .info(ui, id, control_info)
            .debug_selector(move || id.into());
    };
    let s = &n.resolved.segmented_control;
    let b = &s.border;
    let on_click = Rc::new(on_click);
    let colour = info::stated;
    let segments = labels.iter().enumerate().map(|(ix, &label)| {
        let on_click = on_click.clone();
        let active = ix == selected;
        div()
            .id((id, ix))
            .flex()
            .items_center()
            .min_h(px(s.segment_height))
            .pl(b.padding.left.map_or(SEGMENT_PADDING, px))
            .pr(b.padding.right.map_or(SEGMENT_PADDING, px))
            .when_some(b.padding.top, |segment, top| segment.pt(px(top)))
            .when_some(b.padding.bottom, |segment, bottom| segment.pb(px(bottom)))
            .when(ix > 0, |segment| {
                segment
                    .border_l(px(s.separator_width))
                    .border_color(colour(b.color))
            })
            .map(|segment| match (active, s.hover_background) {
                (true, _) => segment
                    .bg(colour(s.active_background))
                    .text_color(colour(s.active_text_color)),
                (false, Some(hover)) => segment
                    .text_color(colour(s.font.color))
                    .hover(move |style| style.bg(colour(hover))),
                (false, None) => segment.text_color(colour(s.font.color)),
            })
            .child(font_text(&n, &s.font, label))
            // The segment inside its dividing line -- an absolute child is
            // laid out inside the border -- and the line, the segment's left
            // border, `separator_width` wide, out over it.
            .children(elements::segment_of(id, ix).map(|(segment, divider)| {
                div()
                    .absolute()
                    .inset_0()
                    .child(elements::record(ui, segment))
                    .children(divider.map(|divider| {
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .left(-px(s.separator_width))
                            .w(px(s.separator_width))
                            .child(elements::record(ui, divider))
                    }))
            }))
            .on_click(move |_, window, cx| on_click(&ix, window, cx))
    });
    // The segments wrap onto a further line where the control's column is
    // too narrow for them, as the Basic page's other rows do.
    h_flex()
        .flex_wrap()
        .bg(colour(s.background_color))
        .border(px(b.line_width))
        .border_color(colour(b.color))
        .rounded(px(b.corner_radius.max(0.0)))
        .overflow_hidden()
        .children(segments)
        .info(ui, id, control_info)
        .debug_selector(move || id.into())
}

/// gpui-base's `HANDLE_PADDING` (resizable/resize_handle.rs:12): the padding
/// a resize handle puts on each side of its line. It is `pub(crate)`
/// upstream, so the showcase names it again to cover the handle's hit area.
const HANDLE_PADDING: Pixels = px(4.);

/// gpui-base's `HANDLE_SIZE` (resizable/resize_handle.rs:13): the width a
/// resize handle is given and its line is drawn at, `pub(crate)` upstream as
/// well.
pub(crate) const HANDLE_SIZE: Pixels = px(1.);

/// The room the side panel keeps clear at its right edge for the splitter's
/// line: `splitter.divider_width` beyond the `HANDLE_SIZE` the handle's own
/// line takes, where the theme states it wider. The handle is absolutely
/// placed (resizable/resize_handle.rs, `ResizeHandle`), so no layout makes
/// room for a wider line; [`resize_handles`] paints it from the boundary
/// back over this strip, so it takes neither panel's content or margin.
pub(crate) fn splitter_reserve(cx: &App) -> Option<Pixels> {
    let width = native_value(cx, |n| px(n.resolved.splitter.divider_width))?;
    (width > HANDLE_SIZE).then(|| width - HANDLE_SIZE)
}

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
            // gpui-base's `handle_color` (resizable/resize_handle.rs:460-469).
            let line = if handle.is_active() {
                base.resizable
                    .active_handle
                    .unwrap_or(base.tokens.colors.ring)
            } else {
                base.resizable.handle.unwrap_or(base.tokens.colors.border)
            };
            // The model's splitter: its line `splitter.divider_width` wide,
            // widened back over the strip the panel before the boundary
            // keeps clear for it (`splitter_reserve`), since the handle
            // lays out no wider than HANDLE_SIZE; `splitter.hover_color`
            // under the pointer, through the group upstream names the
            // handle (resizable/resize_handle.rs, `ResizeHandle`).
            let native = cx.native_theme().and_then(|nt| nt.native(cx));
            let splitter = native.as_ref().map(|n| {
                let s = &n.resolved.splitter;
                (px(s.divider_width), info::stated(s.hover_color))
            });
            let line_info =
                info::resize_handle(&base, between, native.as_ref().map(|n| n.resolved));
            let target = div()
                .size_full()
                .info(&ui, id, line_info)
                .absolute()
                .top_0()
                .bottom_0()
                .left(-HANDLE_PADDING)
                .right(-(HANDLE_PADDING - HANDLE_SIZE))
                .debug_selector(move || id.into());
            let painted = match splitter {
                Some((width, hover)) => div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .right_0()
                    .w(width)
                    .bg(line)
                    .debug_selector(|| CHROME_SPLITTER_LINE.into())
                    .when(!handle.is_active(), |line| {
                        line.group_hover("handle", move |style| style.bg(hover))
                    }),
                None => div().size_full().bg(line),
            }
            .child(elements::record(&ui, "chrome.splitter"));
            Some(
                div()
                    .flex_none()
                    .relative()
                    .h_full()
                    .w(HANDLE_SIZE)
                    .child(painted)
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
    /// That icon as the chosen icon theme gives it.
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
/// the chosen icon theme, or with none where that theme has none.
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
/// `Command` over `state` with `groups`, whose icons are of the icon theme
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
    let title = title
        .info(ui, "overlay-about-title", dialog_info.clone())
        .debug_selector(|| OVERLAY_ABOUT_TITLE.into());
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
    fn set(self, overrides: &mut PreferenceOverrides, on: bool) {
        match self {
            Self::ReduceMotion => overrides.reduce_motion = Some(on),
            Self::HighContrast => overrides.high_contrast = Some(on),
            Self::ReduceTransparency => overrides.reduce_transparency = Some(on),
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

/// The accessibility preferences the user set in the Preferences sheet, each
/// `None` until set there. Every theme install reads the OS's preferences
/// and puts these over them (`PreferenceOverrides::over`), so a preference
/// the user set stays across theme switches and reloads, and one the user
/// did not set follows the OS, as a native application's does.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PreferenceOverrides {
    pub(crate) text_scaling_factor: Option<f32>,
    pub(crate) reduce_motion: Option<bool>,
    pub(crate) high_contrast: Option<bool>,
    pub(crate) reduce_transparency: Option<bool>,
}

impl Global for PreferenceOverrides {}

impl PreferenceOverrides {
    /// `prefs` with every preference the user set replaced by the user's.
    pub(crate) fn over(&self, mut prefs: AccessibilityPreferences) -> AccessibilityPreferences {
        if let Some(scale) = self.text_scaling_factor {
            prefs.text_scaling_factor = scale;
        }
        if let Some(on) = self.reduce_motion {
            prefs.reduce_motion = on;
        }
        if let Some(on) = self.high_contrast {
            prefs.high_contrast = on;
        }
        if let Some(on) = self.reduce_transparency {
            prefs.reduce_transparency = on;
        }
        prefs
    }

    /// The preferences the user set; none before the Preferences sheet set
    /// one.
    pub(crate) fn of(cx: &App) -> Self {
        cx.try_global::<Self>().cloned().unwrap_or_default()
    }
}

/// Record the user's preferences with `change` made, and install them over
/// the installed theme's, where a native theme is installed; tell `ui`: the
/// theme is rebuilt, so an info shown for a target no longer drawn would
/// keep the colours of the theme before.
pub(crate) fn change_preferences(
    ui: &Entity<InfoRegistry>,
    cx: &mut App,
    change: impl FnOnce(&mut PreferenceOverrides),
) {
    if let Some(prefs) = installed_preferences(cx) {
        let mut overrides = PreferenceOverrides::of(cx);
        change(&mut overrides);
        native_theme_gpui::apply_accessibility(&overrides.over(prefs), cx);
        cx.set_global(overrides);
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
        #[cfg(feature = "widgets")]
        if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
            let switch_info =
                info::inputs::native_switch(r, pref.field(), checked, options.is_disabled());
            return widgets::Switch::new(selector)
                .checked(checked)
                .disabled(options.is_disabled())
                .on_change(move |on: &bool, _window, cx| {
                    change_preferences(&clicked, cx, |overrides| pref.set(overrides, *on));
                })
                .info(&ui, selector, switch_info)
                .debug_selector(move || selector.into());
        }
        Switch::new(selector)
            .checked(checked)
            .disabled(options.is_disabled())
            .with_size(options.size())
            .on_click(move |on: &bool, _window, cx| {
                change_preferences(&clicked, cx, |overrides| pref.set(overrides, *on));
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
                change_preferences(&scaled, cx, |overrides| {
                    overrides.text_scaling_factor = Some(scale as f32)
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
        "reduce_transparency: the backdrop behind a dialog or a sheet is not drawn",
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

/// A section heading reading `text`, in the theme's section-heading role
/// (`text_scale.section_heading`, the section divider of
/// `docs/platform-facts.md` §2.19): its size and line height, scaled by the
/// user's text-scaling factor, and its weight, as the iced showcase's section
/// titles are. Without an installed native theme, upstream's `text_base`,
/// semibold. `id` is its info's id and its debug selector.
pub(crate) fn heading(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: impl Into<SharedString>,
) -> Stateful<Div> {
    let role = native_value(cx, |n| {
        let h = &n.resolved.text_scale.section_heading;
        (
            px(native_theme_gpui::scaled_text_size(h.size, n.accessibility)),
            px(native_theme_gpui::scaled_text_size(
                h.line_height,
                n.accessibility,
            )),
            FontWeight(f32::from(h.weight)),
        )
    });
    let label = Label::new(text);
    let label = match role {
        Some((size, line_height, weight)) => label
            .text_size(size)
            .line_height(line_height)
            .font_weight(weight),
        None => label.text_base().font_semibold(),
    };
    label
        .info(ui, id, info::text::heading(cx.theme(), role.is_some()))
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
    /// Its icon, of the chosen icon theme. A loading Button shows its
    /// spinner in place of its icon, turning this one.
    pub icon: Option<SampleIcon>,
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
    let drawn = icon.as_ref().and_then(SampleIcon::icon);
    let mut button_info =
        info::buttons::button(cx.theme(), kind, state, drawn.is_some(), None, styled);
    if let Some(icon) = &icon {
        button_info = match state {
            ButtonState::Loading => button_info.instance(
                "spinner icon",
                icon.note("the Button shows no spinner, only the fade"),
            ),
            ButtonState::Idle | ButtonState::Disabled => {
                button_info.instance("icon", icon.note("the Button shows its label alone"))
            }
        };
    }
    let button = native_info(
        kind.apply(Button::new(id), cx),
        cx,
        geometry::button,
        "button",
        &mut button_info,
    );
    // The platform's disabled pair, which upstream's disabled style replays
    // the caller's refinement over (button/button.rs:797-803); a Button that
    // is not disabled must not take it, since it lands at rest too (:733).
    let disabled_pair = (state == ButtonState::Disabled)
        .then(|| native_geometry(cx, geometry::button_disabled))
        .flatten();
    if disabled_pair.is_some() {
        button_info = button_info.geometry("button_disabled");
    }
    // A disabled Default Button, the variant upstream draws an edge round
    // (button/button.rs, `RenderOnce for Button`: `border_l_1` and its
    // siblings), under a native theme: its fill and its fade go on a surface
    // under it that draws its edge too (`faded_surface`), and the rest of the
    // pair on the Button, which is left without a fill or an edge colour.
    let resolved = cx.native_theme().and_then(|nt| nt.resolved(cx));
    let surface = match (&disabled_pair, resolved) {
        (Some(pair), Some(r)) if kind == ButtonKind::Default => {
            Some((pair.clone(), &r.button.border))
        }
        _ => None,
    };
    let on_button = match &surface {
        Some((pair, _)) => {
            let mut rest = pair.clone();
            rest.background = None;
            rest.opacity = None;
            Some(rest)
        }
        None => disabled_pair,
    };
    if surface.is_some() {
        button_info = button_info.config("surface", "geometry::button_disabled's fill inside button.border.color's line, drawn by the showcase under the Button, which it leaves without a fill or an edge of its own, the whole faded by button.disabled_opacity: gpui fades each quad on its own (gpui-pre window.rs, paint_quad) and draws a quad's border over its own fill (gpui-pre-wgpu shaders.wgsl, fs_quad), so a fill under the line would show through the faded line");
    }
    let button = labelled(ui, cx, refined(button, on_button.as_ref()), id, label)
        .when_some(drawn, |button, icon| match state {
            ButtonState::Loading => button.loading_icon(icon.clone()).icon(icon),
            ButtonState::Idle | ButtonState::Disabled => button.icon(icon),
        })
        .disabled(state == ButtonState::Disabled)
        .loading(state == ButtonState::Loading);
    let none = cx.theme().transparent;
    let button = match surface {
        Some(_) => button.bg(none).border_color(none),
        None => button,
    };
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    let button = InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into());
    match surface {
        Some((fill, border)) => faded_surface(border, &fill, button)
            .flex_none()
            .id(SharedString::from(format!("{id}-surface"))),
        None => button,
    }
}

/// A Default Button at `size`, left without `geometry::button`: this row
/// shows upstream's own size scale, which the refinement would overrule
/// (button/button.rs:669-684, then :733).
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
    let group = ButtonGroup::new(id).children(
        labels
            .iter()
            .map(|&label| labelled(ui, cx, Button::new(label), label, label)),
    );
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
        .button(labelled(
            ui,
            cx,
            kind.apply(Button::new("button"), cx),
            "button",
            label,
        ))
        .dropdown_menu(menu);
    InfoExt::info(
        dropdown,
        ui,
        id,
        info::buttons::dropdown_button(cx.theme(), kind),
    )
}

/// A `Toggle` showing `icon`, of the chosen icon theme, `checked` or not; a
/// click hands `on_click` the state it asks for. Where the icon theme has
/// no such icon the Toggle shows `icon_name` as its label instead. `id` is
/// its info's id and its debug selector.
pub(crate) fn toggle(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    icon: &SampleIcon,
    icon_name: &'static str,
    checked: bool,
    on_click: impl Fn(&bool, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut toggle_info = info::buttons::toggle(
        cx.theme(),
        icon.shown(),
        icon.note("the Toggle shows the icon's name as its label instead"),
        checked,
        styled,
    );
    let toggle = match icon.icon() {
        Some(drawn) => Toggle::new(id).icon(drawn),
        None => Toggle::new(id).label(icon_name),
    };
    native_info(toggle, cx, geometry::toggle, "toggle", &mut toggle_info)
        .checked(checked)
        .on_click(on_click)
        .info(ui, id, toggle_info)
        .debug_selector(move || id.into())
}

/// A `ToggleGroup` of unchecked Toggles reading `labels`, each refined by
/// `geometry::toggle`.
pub(crate) fn toggle_group(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
) -> Stateful<Div> {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut group_info = info::buttons::toggle_group(cx.theme(), styled);
    if styled {
        group_info = group_info.geometry("toggle");
    }
    ToggleGroup::new(id)
        .children(
            labels
                .iter()
                .map(|&label| Toggle::new(label).label(label).native(cx, geometry::toggle)),
        )
        .info(ui, id, group_info)
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
/// `field` names; `disabled` or not.
pub(crate) fn text_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<InputState>,
    field: InputField,
    disabled: bool,
    width: Pixels,
) -> Stateful<Div> {
    text_field(ui, cx, id, state, field, disabled, width, false)
}

/// The Inputs page's `Input` holding an inline token (spec §8.3):
/// [`text_input`]'s field, refined whole and enabled, drawing its state's
/// inline tokens as upstream's `InputToken` chips.
pub(crate) fn token_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<InputState>,
    width: Pixels,
) -> Stateful<Div> {
    text_field(ui, cx, id, state, InputField::Refined, false, width, true)
}

/// [`text_input`], drawing the state's inline tokens as `InputToken` chips
/// where `tokens`.
#[allow(clippy::too_many_arguments)]
fn text_field(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<InputState>,
    field: InputField,
    disabled: bool,
    width: Pixels,
    tokens: bool,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut input_info = info::inputs::input(cx.theme(), field, styled);
    if disabled {
        input_info = input_info.variant("disabled");
    }
    if tokens {
        input_info = info::inputs::with_token(input_info, cx.theme());
    }
    let input = Input::new(state)
        .with_size(Size::Medium)
        .disabled(disabled)
        .w(width)
        .when(tokens, |input| {
            input.token(|ctx, _, _| InputToken::new(ctx))
        });
    let input = match field {
        InputField::Refined => {
            let input = native_info(input, cx, geometry::input, "input", &mut input_info);
            // The platform's fill for the state the field is built in, after
            // upstream's (input/input.rs:785, then :793).
            let fill = native_geometry(cx, |n| geometry::input_fill(n, disabled));
            if fill.is_some() {
                input_info = input_info.geometry("input_fill");
            }
            let resolved = cx.native_theme().and_then(|nt| nt.resolved(cx));
            if let (true, Some(r), Some(fill)) = (disabled, resolved, &fill) {
                input_info = info::inputs::disabled_input_surface(input_info);
                let none = cx.theme().transparent;
                return faded_surface(&r.input.border, fill, input.bg(none).border_color(none))
                    .w(width)
                    .info(ui, id, input_info)
                    .debug_selector(move || id.into());
            }
            let input = refined(input, fill.as_ref());
            if let (false, Some(r)) = (disabled, resolved) {
                input_info = info::inputs::input_surface(input_info, r);
                let none = cx.theme().transparent;
                return input_surface(r, &r.input.border, input.bg(none).border_color(none))
                    .w(width)
                    .info(ui, id, input_info)
                    .debug_selector(move || id.into());
            }
            input
        }
        // Through the caller's style, which Input applies after its own
        // height and line height (input/input.rs:773-777, then :793).
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

/// The group an enabled field's surface takes its hover from.
const INPUT_GROUP: &str = "input-surface";

/// An enabled, refined `field` over its surface, drawn from `r`:
/// `input.background_color` framed by `border` (`input.border`, or a text
/// area's `text_area.border`), and by `input.hover_border_color` under the
/// pointer. The caller leaves the field without a fill or an edge colour of
/// its own, in upstream's `transparent` token (theme/schema.rs).
///
/// `Input` is `Styled` only (input/input.rs:529) and its root's one state is
/// `focused` (:747-754), so it has no hover edge of its own: the field is
/// left without a fill or an edge colour (its caller's style lands after its
/// own, :793) and an absolute box under it, the field's size, paints them.
/// Focused, the field's own edge in `ring` shows over the surface's.
fn input_surface(
    r: &native_theme_gpui::ResolvedTheme,
    border: &native_theme::theme::ResolvedWidgetBorder,
    field: impl IntoElement,
) -> Div {
    let i = &r.input;
    // A soft option the theme leaves unstated keeps the edge it would cover.
    let hover = info::stated(i.hover_border_color.unwrap_or(border.color));
    div()
        .relative()
        .group(INPUT_GROUP)
        .child(
            div()
                .absolute()
                .inset_0()
                .rounded(px(border.corner_radius.max(0.0)))
                .bg(info::stated(i.background_color))
                .border(px(border.line_width))
                .border_color(info::stated(border.color))
                .group_hover(INPUT_GROUP, move |style| style.border_color(hover)),
        )
        .child(field)
}

/// A disabled control over its surface: the fill `fill` gives (a disabled
/// field's `geometry::input_fill`, a disabled Button's
/// `geometry::button_disabled`) inside `border`'s line, the line in
/// `border.color`, and the whole faded by `fill`'s opacity. The caller
/// leaves the control without a fill or an edge colour of its own, and
/// unfaded.
///
/// The fill is a box of its own inside the line, not under it: gpui fades
/// each primitive it paints on its own (gpui-pre src/window.rs:4553-4561,
/// `paint_quad`), and a quad draws its border over its own background
/// (gpui-pre-wgpu src/shaders.wgsl:887, `fs_quad`), so a faded control whose
/// fill lay under its line would show the fill through the faded line,
/// where a platform fades the control as one.
fn faded_surface(
    border: &native_theme::theme::ResolvedWidgetBorder,
    fill: &gpui::StyleRefinement,
    field: impl IntoElement,
) -> Div {
    let line = border.line_width.max(0.0);
    let radius = border.corner_radius.max(0.0);
    let inner = div()
        .absolute()
        .top(px(line))
        .left(px(line))
        .right(px(line))
        .bottom(px(line))
        .rounded(px((radius - line).max(0.0)));
    let inner = match fill.background.clone() {
        Some(background) => inner.bg(background),
        None => inner,
    };
    div()
        .relative()
        .when_some(fill.opacity, |surface, fade| surface.opacity(fade))
        .child(inner)
        .child(
            div()
                .absolute()
                .inset_0()
                .rounded(px(radius))
                .border(px(line))
                .border_color(info::stated(border.color)),
        )
        .child(field)
}

/// A `Textarea` over `state`, `width` by `height`, refined by
/// `geometry::text_area`: it renders as an `Input` (input/textarea.rs:221),
/// padded by the sides `text_area.border.padding` states, the multi-line
/// field's own (docs/platform-facts.md §2.29); its `height` goes on after
/// the builder.
pub(crate) fn textarea(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TextareaState>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let fill = native_geometry(cx, |n| geometry::input_fill(n, false));
    let mut textarea_info = info::inputs::textarea(cx.theme(), fill.is_some());
    let textarea = native_info(
        Textarea::new(state).w(width),
        cx,
        geometry::text_area,
        "text_area",
        &mut textarea_info,
    );
    if fill.is_some() {
        textarea_info = textarea_info.geometry("input_fill");
    }
    let textarea = refined(textarea, fill.as_ref());
    Styled::h(textarea, height)
        .info(ui, id, textarea_info)
        .debug_selector(move || id.into())
}

/// A `Textarea` over `state`, `width` wide and as tall as the rows `state`
/// was built to show, refined by `geometry::text_area` as [`textarea`]
/// refines its own; the refinement sets no height, and the rows the state
/// holds size it (gpui-base input/base/state.rs,
/// `InputBaseState::auto_grow`).
pub(crate) fn rows_textarea(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TextareaState>,
    width: Pixels,
) -> Stateful<Div> {
    let fill = native_geometry(cx, |n| geometry::input_fill(n, false));
    let mut textarea_info = info::inputs::rows_textarea(cx.theme(), fill.is_some());
    let textarea = native_info(
        Textarea::new(state).w(width),
        cx,
        geometry::text_area,
        "text_area",
        &mut textarea_info,
    );
    if fill.is_some() {
        textarea_info = textarea_info.geometry("input_fill");
    }
    let mut textarea = refined(textarea, fill.as_ref());
    let style = Styled::style(&mut textarea);
    style.size.height = None;
    style.min_size.height = None;
    // The frame the theme states for a text area, `text_area.border`, with
    // the text field's hover edge, over a surface as an enabled text input
    // takes it ([`input_surface`]).
    if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
        let none = cx.theme().transparent;
        return input_surface(r, &r.text_area.border, textarea.bg(none).border_color(none))
            .w(width)
            .info(ui, id, textarea_info)
            .debug_selector(move || id.into());
    }
    textarea
        .info(ui, id, textarea_info)
        .debug_selector(move || id.into())
}

/// The states of the three `InputGroup`s, and the icons of the chosen icon
/// theme the first two show.
pub(crate) struct InputGroupStates<'a> {
    /// The field behind the Search icon.
    pub search: &'a Entity<InputState>,
    /// The field the Copy button copies.
    pub copy: &'a Entity<InputState>,
    /// The textarea above the note.
    pub notes: &'a Entity<TextareaState>,
    /// The icon before the first field.
    pub search_icon: SampleIcon,
    /// The Copy button's icon.
    pub copy_icon: SampleIcon,
}

/// Three `InputGroup`s, `width` wide, one above the other: a field behind a
/// Search icon, a field with a Copy button after it, which puts the field's
/// text on the clipboard and says so in a notification, and a textarea with
/// a note under it. Where the chosen icon theme has no Search icon the first
/// field's addon is empty, and where it has no Copy icon the button shows
/// its label alone. They report as one; `id` is their info's id and debug
/// selector.
pub(crate) fn input_groups(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    states: InputGroupStates<'_>,
    width: Pixels,
) -> Stateful<Div> {
    let mut groups_info = info::inputs::input_groups(cx.theme())
        .instance(
            "search icon",
            states.search_icon.note("the first field's addon is empty"),
        )
        .instance(
            "copy icon",
            states
                .copy_icon
                .note("the Copy button shows its label alone"),
        );
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
    .when_some(states.copy_icon.icon(), |button, icon| button.icon(icon))
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
                        .children(states.search_icon.icon()),
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
        .debug_selector(move || id.into())
}

/// A number field over `state`, `width` wide.
///
/// Under a native theme, gpui-component's `NumberInput` rebuilt over the
/// unstyled spin button it composes (`gpui_base::NumberInput`,
/// input/number_input.rs:113-201), with what the theme states for a text
/// field where upstream sets literals no caller reaches: the frame is
/// `input.background_color` edged by `input.border` (colour, width, radius)
/// at `geometry::input`'s height, in `input.hover_border_color` under the
/// pointer and `input.focus_border_color` while focused; the value is an
/// `Input` padded by `input.border.padding` in `input.font`. Upstream's
/// fills its frame with its `input_background` (:113-114) and pads its value
/// by its Size (:158-165). The theme states no spin-box layout: the value
/// starts at the field's padding and the step buttons stack at its right
/// end (gpui-base's `controls_right`), as the desktops' spin boxes lay them
/// out (Qt's QSpinBox, GTK's GtkSpinButton), where upstream's centres the
/// value between − and + (:158-165); the buttons are upstream's otherwise
/// (:116-156). Without a native theme, upstream's `NumberInput`.
pub(crate) fn number_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    window: &Window,
    id: &'static str,
    state: &Entity<InputState>,
    width: Pixels,
) -> Stateful<Div> {
    let mut number_info = info::inputs::number_input(cx.theme());
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let Some(n) = native else {
        return NumberInput::new(state)
            .placeholder("Enter a number")
            .with_size(Size::Medium)
            .w(width)
            .info(ui, id, number_info);
    };
    number_info = number_info.geometry("input");
    let i = &n.resolved.input;
    let focused = gpui::Focusable::focus_handle(state.read(cx), cx).is_focused(window);
    // A soft option the theme leaves unstated keeps the edge it would cover.
    let edge = info::stated(match (focused, i.focus_border_color) {
        (true, Some(focus)) => focus,
        _ => i.border.color,
    });
    let hover = info::stated(i.hover_border_color.unwrap_or(i.border.color));
    let field = geometry::input(n);
    // The frame: the field's height, radius and edge width, no padding --
    // that is the value's, inside the buttons.
    let mut frame = field.clone();
    frame.padding = StyleRefinement::default().padding;
    // The value: the field's padding and text, in the frame's height.
    let mut value = field;
    value.size = StyleRefinement::default().size;
    value.min_size = StyleRefinement::default().min_size;
    value.border_widths = StyleRefinement::default().border_widths;
    value.corner_radii = StyleRefinement::default().corner_radii;
    // Upstream's step buttons (input/number_input.rs:116-121, :137-156,
    // :172-189): transparent, tinted with the frame's edge colour under the
    // pointer and while pressed, a pixel tighter at the outer corners, as
    // wide as upstream's at the field's Size (:147-151, `min_w_8`); stacked,
    // each half the field's height, with its chevron at the Size upstream
    // gives a Small field's icons.
    let foreground = cx.theme().secondary_foreground;
    let tint = cx.theme().input;
    let radius = (px(i.border.corner_radius.max(0.0)) - px(i.border.line_width)).max(px(0.));
    let step = move |button: gpui_base::Button, label: &'static str, icon: IconName| {
        button
            .accessibility_label(label)
            .flex()
            .items_center()
            .justify_center()
            .text_color(foreground)
            .hover(move |this| this.bg(tint.opacity(NUMBER_STEP_HOVER)))
            .active(move |this| this.bg(tint.opacity(NUMBER_STEP_ACTIVE)))
            .min_w_8()
            .child(Icon::new(icon).with_size(Size::Small))
    };
    let spin = gpui_base::NumberInput::new(state)
        .size_full()
        .controls_right()
        .increment_button(move |b| step(b, "Increment", IconName::ChevronUp).rounded_tr(radius))
        .input(
            Input::new(state)
                .appearance(false)
                .with_size(Size::Medium)
                .h_full()
                .gap_0()
                .rounded_none()
                .refine_style(&value),
        )
        .decrement_button(move |b| step(b, "Decrement", IconName::ChevronDown).rounded_br(radius));
    div()
        .id("number-frame")
        .w(width)
        .bg(info::stated(i.background_color))
        .refine_style(&frame)
        .border_color(edge)
        .when(!focused, |el| el.hover(move |s| s.border_color(hover)))
        .child(spin)
        .info(ui, id, number_info)
}

/// The opacity of the frame's edge colour a step button of the number field
/// is tinted with under the pointer and while pressed: upstream's
/// (input/number_input.rs:120-121, `input.opacity(0.4)`, `(0.6)`).
const NUMBER_STEP_HOVER: f32 = 0.4;
const NUMBER_STEP_ACTIVE: f32 = 0.6;

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
    // Under a native theme, the connector's theme-drawn checkbox: every part
    // gpui-component draws from literals of its own is the theme's there.
    #[cfg(feature = "widgets")]
    if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
        let checkbox_info = info::inputs::native_checkbox(r, label, checked, disabled);
        return widgets::Checkbox::new(id)
            .label(label)
            .checked(checked)
            .disabled(disabled)
            .when_some(on_click, |checkbox, on_click| checkbox.on_change(on_click))
            .when_some(elements::part_observer(ui, id), |checkbox, observer| {
                checkbox.on_part_bounds(observer)
            })
            .info(ui, id, checkbox_info)
            .debug_selector(move || id.into());
    }
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

/// The space between the radios of a horizontal group, which the model does
/// not state: gpui-component's own `RadioGroup`'s, `gap_3` (radio.rs,
/// `RenderOnce for RadioGroup`).
#[cfg(feature = "widgets")]
const RADIO_GROUP_GAP: Rems = rems(0.75);

/// A horizontal `RadioGroup` of Radios reading `labels`, each refined by
/// `geometry::radio`, the one at `selected` selected; a click hands
/// `on_click` the index of the Radio clicked.
///
/// `RadioGroup::child` takes `impl Into<Radio>`, so a `&str` child would
/// build a `Radio` with no refinement; built here instead, each row carries
/// the platform's label gap and font. The group overwrites the id
/// (`radio.rs:403`), not the style.
pub(crate) fn radio_group(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
    selected: Option<usize>,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    #[cfg(feature = "widgets")]
    if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
        let group_info =
            info::inputs::native_radio_column(r, labels, selected).variant("horizontal");
        let on_click = Rc::new(on_click);
        return widgets::RadioGroup::new(id)
            .axis(Axis::Horizontal)
            .gap(RADIO_GROUP_GAP)
            .children(labels.iter().enumerate().map(|(ix, &label)| {
                let on_click = on_click.clone();
                widgets::Radio::new((id, ix))
                    .label(label)
                    .checked(selected == Some(ix))
                    .on_change(move |_, window, cx| on_click(&ix, window, cx))
            }))
            .info(ui, id, group_info);
    }
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

/// A column of Radios reading `labels`, `gap` apart, each refined by
/// `geometry::radio`, the one at `selected` selected; a click hands
/// `on_click` the index of the Radio clicked.
///
/// Built Radio by Radio, not as a `RadioGroup`: a group lays its Radios out
/// in a child of its own, `gap_3` apart (radio.rs, `RenderOnce for
/// RadioGroup`), which the group's style does not reach, and the page sets
/// its rows `layout.widget_gap` apart. The column reports as the group does.
pub(crate) fn radio_column(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
    gap: Option<Pixels>,
    selected: Option<usize>,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    #[cfg(feature = "widgets")]
    if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
        let mut group_info = info::inputs::native_radio_column(r, labels, selected);
        if gap.is_some() {
            group_info = group_info.geometry("widget_gap");
        }
        let on_click = Rc::new(on_click);
        let total = labels.len();
        // gpui-base's headless group, as `widgets::RadioGroup` builds its
        // own (the role and the arrow keys), round Radios that each report
        // themselves: a `widgets::RadioGroup` takes Radios alone, no box
        // round one.
        return gpui_base::RadioGroup::new(id)
            .axis(Axis::Vertical)
            .child(
                with_gap(v_flex(), gap)
                    .items_start()
                    .children(labels.iter().enumerate().map(|(ix, &label)| {
                        let on_click = on_click.clone();
                        let checked = selected == Some(ix);
                        let item = format!("{id}-{ix}");
                        widgets::Radio::new((id, ix))
                            .label(label)
                            .checked(checked)
                            .set_position(ix.saturating_add(1), total)
                            .on_change(move |_, window, cx| on_click(&ix, window, cx))
                            .when_some(elements::part_observer(ui, &item), |radio, observer| {
                                radio.on_part_bounds(observer)
                            })
                            .info(
                                ui,
                                SharedString::from(item),
                                info::inputs::native_radio_column(
                                    r,
                                    &[label],
                                    checked.then_some(0),
                                ),
                            )
                    })),
            )
            .info(ui, id, group_info)
            .debug_selector(move || id.into());
    }
    let mut group_info = info::inputs::radio_column(cx.theme(), labels, selected);
    let row = native_info(
        StyleRefinement::default(),
        cx,
        geometry::radio,
        "radio",
        &mut group_info,
    );
    if gap.is_some() {
        group_info = group_info.geometry("widget_gap");
    }
    let on_click = Rc::new(on_click);
    with_gap(v_flex(), gap)
        .items_start()
        .children(labels.iter().enumerate().map(|(ix, &label)| {
            let on_click = on_click.clone();
            Radio::new((id, ix))
                .refine_style(&row)
                .label(label)
                .checked(selected == Some(ix))
                .on_click(move |_, window, cx| on_click(&ix, window, cx))
        }))
        .info(ui, id, group_info)
        .debug_selector(move || id.into())
}

/// A `Switch` reading `label`, `checked` or not, its checked track in the
/// platform's `switch.checked_background` through `Switch::color`. A
/// disabled one takes no `on_click`.
pub(crate) fn switch(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    checked: bool,
    on_click: Option<impl Fn(&bool, &mut Window, &mut App) + 'static>,
) -> Stateful<Div> {
    let disabled = on_click.is_none();
    #[cfg(feature = "widgets")]
    if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
        let switch_info = info::inputs::native_switch(r, label, checked, disabled);
        return widgets::Switch::new(id)
            .label(label)
            .checked(checked)
            .disabled(disabled)
            .when_some(on_click, |switch, on_click| switch.on_change(on_click))
            .when_some(elements::part_observer(ui, id), |switch, observer| {
                switch.on_part_bounds(observer)
            })
            .info(ui, id, switch_info)
            .debug_selector(move || id.into());
    }
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let resolved = native.as_ref().map(|n| n.resolved);
    let checked_background = native_color(cx, |n| n.resolved.switch.checked_background);
    Switch::new(id)
        .label(label)
        .checked(checked)
        .disabled(disabled)
        .when_some(checked_background, |switch, color| switch.color(color))
        .when_some(on_click, |switch, on_click| switch.on_click(on_click))
        .info(
            ui,
            id,
            info::inputs::switch(cx.theme(), label, checked, disabled, resolved),
        )
        .debug_selector(move || id.into())
}

/// A `Slider` over `state`, `width` wide.
pub(crate) fn slider(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<SliderState>,
    width: Pixels,
) -> Stateful<Div> {
    #[cfg(feature = "widgets")]
    if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
        return widgets::Slider::new(state)
            .w(width)
            .when_some(elements::part_observer(ui, id), |slider, observer| {
                slider.on_part_bounds(observer)
            })
            .info(ui, id, info::inputs::native_slider(r))
            .debug_selector(move || id.into());
    }
    Slider::new(state)
        .w(width)
        .info(ui, id, info::inputs::slider(cx.theme()))
        .debug_selector(move || id.into())
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
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let mut select_info = info::inputs::select(cx.theme(), native.as_ref().map(|n| n.resolved));
    let select = native_info(
        Select::new(state).placeholder(placeholder).w(width),
        cx,
        geometry::select,
        "select",
        &mut select_info,
    );
    combo_surface(cx, refined(select, combo_fill(cx).as_ref()))
        .w(width)
        .info(ui, id, select_info)
        .debug_selector(move || id.into())
}

/// A drop-down trigger's edge, `combo_box.border.color`, and no fill of its
/// own, for a `Select` or a `Combobox` that is not disabled, or `None`
/// before `apply` ran: [`combo_surface`] paints the fill under it. The
/// trigger paints `input_background()` and `input` first and refines itself
/// with the caller's style after (select.rs, `Select::render`; combobox.rs,
/// `Combobox::render`), so these land over them.
fn combo_fill(cx: &App) -> Option<StyleRefinement> {
    // Upstream's own `transparent` token, which the trigger starts its edge
    // with (select.rs:542).
    let none = cx.theme().transparent;
    native_value(cx, |n| {
        let c = &n.resolved.combo_box;
        StyleRefinement::default()
            .bg(none)
            .border_color(info::stated(c.border.color))
    })
}

/// The surface under a drop-down `trigger` refined by [`combo_fill`]:
/// `combo_box.background_color`, and under the pointer
/// `combo_box.hover_background` over it, rounded as the trigger is
/// (`combo_box.border.corner_radius`, `geometry::select`). Upstream's trigger
/// sets no hover (select.rs:535-553; combobox.rs:981-998) and neither
/// `Select` nor `Combobox` is an `InteractiveElement` (select.rs:795), so the
/// hover fill is this element's. Before `apply` ran it is a plain box.
fn combo_surface(cx: &App, trigger: impl IntoElement) -> Div {
    let look = native_value(cx, |n| {
        let c = &n.resolved.combo_box;
        let fill = info::stated(c.background_color);
        (
            fill,
            c.hover_background
                .map(|hover| fill.blend(info::stated(hover))),
            px(c.border.corner_radius.max(0.0)),
        )
    });
    match look {
        Some((fill, hover, radius)) => div()
            .rounded(radius)
            .bg(fill)
            .when_some(hover, |surface, hover| {
                surface.hover(move |style| style.bg(hover))
            })
            .child(trigger),
        None => div().child(trigger),
    }
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

/// A `ColorSelect` over `state`, upstream's own: its framed field is an
/// inner element that sizes itself from the Input ladder, so no geometry
/// builder reaches it. gpui redraws the window when `state` notifies, so the
/// panel follows a pick without a subscription.
pub(crate) fn color_select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<ColorPickerState>,
) -> Stateful<Div> {
    ColorSelect::new(state).placeholder("Pick a color").info(
        ui,
        id,
        info::inputs::color_select(cx.theme(), state.read(cx).value().is_some()),
    )
}

/// A `SpeechWaveform` over `state`, upstream's own: its size and colours are
/// literals and theme tokens it reads itself, so no geometry builder reaches
/// it. gpui redraws the window when `state` notifies, once per new level.
pub(crate) fn speech_waveform(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<SpeechState>,
) -> Stateful<Div> {
    SpeechWaveform::new(state).info(ui, id, info::inputs::speech_waveform(cx.theme()))
}

/// A `TimeField` over `state` (spec §8.3), upstream's own: the theme states
/// no time-field geometry, so its size and radius are upstream's.
pub(crate) fn time_field(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TimeFieldState>,
) -> Stateful<Div> {
    TimeField::new(state)
        .info(ui, id, info::inputs::time_field(cx.theme()))
        .debug_selector(move || id.into())
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
    // What `native_value` and `.native` apply the builders under.
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    DataTable::new(state)
        .with_size(native_value(cx, geometry::data_table_size).unwrap_or_default())
        .stripe(true)
        .bordered(true)
        .info(
            ui,
            id,
            info::data::data_table(cx.theme(), rows, columns, styled),
        )
        .native(cx, geometry::table)
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
    /// An odd row of a striped table (table/state.rs:2009).
    pub striped: bool,
    /// The last of the delegate's rows.
    pub last: bool,
    /// The table's selected row.
    pub selected: bool,
    /// The table is selecting rows, so it paints its selected row as one
    /// (table/state.rs:497, 2243); a column selection keeps the selected row but
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

/// A box `width` by `height`, refined by `geometry::list` and filled with
/// `list.background_color`, around the List over `state`; `selector` is its
/// debug selector. A List paints no frame and no fill of its own
/// (list/list.rs, `RenderOnce for List`), so the box is its frame and its
/// fill; the rows report themselves (`ListRow`).
pub(crate) fn list(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    selector: &'static str,
    state: &Entity<ListState<SampleListDelegate>>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let mut list_info = info::data::list(
        state.read(cx).delegate().items.len(),
        native.as_ref().map(|n| n.resolved),
    );
    let fill = native_color(cx, |n| n.resolved.list.background_color);
    native_info(
        div()
            .w(width)
            .h(height)
            .when_some(fill, |list, fill| list.bg(fill)),
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
    .debug_selector(move || selector.into())
}

/// What a `ListItem` row is marked as.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ListRowState {
    Idle,
    Selected,
    /// Right-clicked and not selected: the `selection` outline and no fill
    /// (list/list_item.rs:266-275).
    RightClicked,
    /// Selected and right-clicked: the selected fill with the outline over
    /// it (list/list_item.rs:257-275).
    SelectedRightClicked,
}

impl ListRowState {
    fn of(selected: bool, right_clicked: bool) -> Self {
        match (selected, right_clicked) {
            (true, true) => Self::SelectedRightClicked,
            (false, true) => Self::RightClicked,
            (true, false) => Self::Selected,
            (false, false) => Self::Idle,
        }
    }
}

/// Row `ix` of a List, reading `label`: a `ListItem` refined by
/// `geometry::list_item`, its info id and debug selector `{prefix}-{ix}`.
///
/// The List marks a row selected or right-clicked after the delegate built
/// it (list/list.rs, `ListState::render_list_item`), so the row builds its
/// `ListItem` and its info only as it renders, from what it was marked.
#[derive(IntoElement)]
pub(crate) struct ListRow {
    ui: Entity<InfoRegistry>,
    prefix: &'static str,
    ix: usize,
    label: SharedString,
    selected: bool,
    right_clicked: bool,
}

impl ListRow {
    pub(crate) fn new(
        ui: &Entity<InfoRegistry>,
        prefix: &'static str,
        ix: usize,
        label: SharedString,
    ) -> Self {
        Self {
            ui: ui.clone(),
            prefix,
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

/// The group a list row's label takes its hover colour from.
const LIST_ROW_GROUP: &str = "list-row";

/// The room a List's row leaves for the scrollbar: its groove, where the
/// theme's scrollbar is not an overlay (`geometry::scrollbar_gutter`'s
/// width); `None` for an overlay, or before `apply` ran.
fn list_gutter(cx: &App) -> Option<Pixels> {
    native_value(cx, |n| {
        (!n.resolved.scrollbar.overlay_mode)
            .then(|| native_theme_gpui::base_layer::scrollbar_geometry(n.resolved).track_width)
    })
    .flatten()
}

impl RenderOnce for ListRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = format!("{}-{}", self.prefix, self.ix);
        let state = ListRowState::of(self.selected, self.right_clicked);
        // What `native_info` applies the builder under.
        let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
        let mut row_info = info::data::list_row(cx.theme(), &self.label, state, styled);
        // A selected row's text in `list.selection_text_color`, over the
        // list font's colour `geometry::list_item` gives every row: the row
        // refines itself with its style after its own `foreground`
        // (list/list_item.rs, `RenderOnce for ListItem`).
        let selected_text = self
            .selected
            .then(|| native_color(cx, |n| n.resolved.list.selection_text_color))
            .flatten();
        if selected_text.is_some() {
            row_info = row_info.config(
                "selected text",
                "list.selection_text_color, which the showcase sets on the selected row over the list font's colour",
            );
        }
        // A row under the pointer lettered in `list.hover_text_color`: the
        // ListItem's own hover sets its fill alone, inside render
        // (list/list_item.rs:231), so the label takes the colour from the
        // row's group hover. Not on a selected row, which does not hover
        // (:228-229).
        let hover_text = (!self.selected && !self.right_clicked)
            .then(|| native_color(cx, |n| n.resolved.list.hover_text_color))
            .flatten();
        if hover_text.is_some() {
            row_info = row_info.config(
                "hovered text",
                "list.hover_text_color, which the showcase sets on the label while the row is hovered",
            );
        }
        let item = native_info(
            ListItem::new(SharedString::from(id.clone())),
            cx,
            geometry::list_item,
            "list_item",
            &mut row_info,
        )
        .when_some(selected_text, |item, text| item.text_color(text))
        // Clear of the List's scrollbar where the theme's is not an overlay:
        // the List draws its bar over its rows (list/list.rs, `List::render`),
        // where a scrollbar that is not an overlay takes the room beside them
        // (`scrollbar.overlay_mode`, its `groove_width`).
        .when_some(list_gutter(cx), |item, gutter| item.mr(gutter))
        .group(LIST_ROW_GROUP)
        // Plain text, not a Label: `Label::render` paints foreground on its
        // own element (label.rs:211) over the list font the row carries.
        .child(
            div()
                .child(self.label)
                .when_some(hover_text, |label, text| {
                    label.group_hover(LIST_ROW_GROUP, move |style| style.text_color(text))
                }),
        )
        .selected(self.selected)
        .secondary_selected(self.right_clicked);
        item.info(&self.ui, SharedString::from(id.clone()), row_info)
            .w_full()
            .debug_selector(move || id)
    }
}

/// A box `width` by `height`, refined by `geometry::list` and filled with
/// `list.background_color`, around the Tree over `state`. A tree is a list
/// view and the model gives it no theme of its own, so its frame and fill
/// are the list's; the rows report themselves (`tree_row`).
pub(crate) fn tree(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TreeState>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let mut tree_info = info::data::tree(native.as_ref().map(|n| n.resolved));
    let fill = native_color(cx, |n| n.resolved.list.background_color);
    let rows = ui.clone();
    native_info(
        div()
            .w(width)
            .h(height)
            .when_some(fill, |tree, fill| tree.bg(fill)),
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
/// (list/list_item.rs:199, 233), so an absolute box there covers the
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
    // As a List row's: a selected row's text in `list.selection_text_color`.
    let selected_text = selected
        .then(|| native_color(cx, |n| n.resolved.list.selection_text_color))
        .flatten();
    if selected_text.is_some() {
        row_info = row_info.config(
            "selected text",
            "list.selection_text_color, which the showcase sets on the selected row over the list font's colour",
        );
    }
    let item = native_info(
        ListItem::new(SharedString::from(id.clone())),
        cx,
        geometry::list_item,
        "list_item",
        &mut row_info,
    )
    .when_some(selected_text, |item, text| item.text_color(text));
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
    let button = labelled(
        ui,
        cx,
        native_info(
            Button::new(id),
            cx,
            geometry::button,
            "button",
            &mut button_info,
        ),
        id,
        label,
    )
    .on_click(on_click);
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into())
}

/// One `Attachment` card of the Data page: `file`, described by
/// `description`, in `status`, its media `icon` of the chosen icon theme.
/// `id` is its info's id and its debug selector.
pub(crate) struct DemoAttachment {
    pub id: &'static str,
    pub status: AttachmentStatus,
    pub icon: SampleIcon,
    pub file: &'static str,
    pub description: SharedString,
}

/// An `Attachment` card, its media icon at `geometry::icon_size_small`, or
/// an empty media frame where the chosen icon theme has none. A card given
/// `on_click` is clickable as a whole.
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
    let mut card_info = info::data::attachment(cx.theme(), status, file, on_click.is_some(), &icon);
    if icon.shown() && native_value(cx, geometry::icon_size_small).is_some() {
        card_info = card_info.geometry("icon_size_small");
    }
    let card = Attachment::new()
        .status(status)
        .media(AttachmentMedia::new().children(icon.sized(cx, geometry::icon_size_small)))
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
            info::feedback::severity_alert(cx.theme(), severity)
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
    #[cfg(feature = "widgets")]
    if let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx)) {
        return widgets::ProgressBar::new(id)
            .value(value)
            .accessibility_label(label)
            .when_some(elements::part_observer(ui, id), |bar, observer| {
                bar.on_part_bounds(observer)
            })
            .info(ui, id, info::feedback::native_progress(r, label, value))
            .debug_selector(move || id.into());
    }
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

/// The icon set a theme-drawn Spinner draws its indicator from, as the
/// page's icons come: their set (`None` for gpui-component's own icons), the
/// freedesktop theme they load from (`None`: the system's), and whether the
/// set has an indicator (the Icons page's animated icons, loaded the same
/// way).
#[derive(Clone, Default)]
#[cfg_attr(not(feature = "widgets"), allow(dead_code))]
pub(crate) struct SpinnerIcons {
    pub(crate) set: Option<IconSet>,
    pub(crate) theme: Option<SharedString>,
    pub(crate) indicator: bool,
}

/// A `Spinner` of `kind`; the theme-drawn one draws `icons`' indicator.
pub(crate) fn spinner(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: SpinnerKind,
    icons: &SpinnerIcons,
) -> Stateful<Div> {
    // The theme states one spinner, `spinner.*`: the Medium one is drawn
    // from it; Small and Large stay gpui-component's, sizes of its own.
    #[cfg(feature = "widgets")]
    if kind == SpinnerKind::Medium
        && let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx))
    {
        return widgets::Spinner::new(id)
            .icon_set(icons.set)
            .icon_theme(icons.theme.clone())
            .info(
                ui,
                id,
                info::feedback::native_spinner(r, cx.reduce_motion(), icons),
            )
            .debug_selector(move || id.into());
    }
    #[cfg(not(feature = "widgets"))]
    let _ = icons;
    let (size, styled) = match kind {
        SpinnerKind::Small => (Size::Small, false),
        SpinnerKind::Large => (Size::Large, false),
        SpinnerKind::Medium => match native_value(cx, geometry::spinner_size) {
            Some(size) => (size, true),
            None => (Size::Medium, false),
        },
    };
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let spinner_info = info::feedback::spinner(
        cx.theme(),
        kind,
        styled,
        cx.reduce_motion(),
        native.as_ref().map(|n| n.resolved),
    );
    let spinner_info = if styled {
        spinner_info.geometry("spinner_size")
    } else {
        spinner_info
    };
    let fill = native_color(cx, |n| n.resolved.spinner.fill_color);
    Spinner::new()
        .with_size(size)
        .when_some(fill, |spinner, fill| spinner.color(fill))
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

/// The `Empty` state, titled `title` over `description`, its media `icon`,
/// of the chosen icon theme, at the platform's large icon size -- an empty
/// media frame where that theme has none -- and its action an outlined
/// Refresh Button, which reports itself as `refresh`.
pub(crate) fn empty(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    refresh: &'static str,
    title: &'static str,
    description: &'static str,
    icon: &SampleIcon,
) -> Stateful<Div> {
    let empty_info = info::feedback::empty(cx.theme(), title, description, icon);
    let empty_info = if icon.shown() && native_value(cx, geometry::icon_size_large).is_some() {
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
                        .children(icon.sized(cx, geometry::icon_size_large)),
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
    let target = labelled(
        ui,
        cx,
        native_info(
            Button::new(id),
            cx,
            geometry::button,
            "button",
            &mut badge_info,
        ),
        id,
        label,
    );
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

/// A `Marker` of `kind` reading `text`; a Plain one shows `icon`, of the
/// chosen icon theme, at the platform's small icon size, and its text alone
/// where that theme has none.
pub(crate) fn marker(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: MarkerKind,
    text: &'static str,
    icon: &SampleIcon,
) -> Stateful<Div> {
    let plain_icon = (kind == MarkerKind::Plain).then_some(icon);
    let mut marker_info =
        info::feedback::marker(cx.theme(), kind, text, cx.reduce_motion(), plain_icon);
    let marker = Marker::new();
    let marker = match kind {
        MarkerKind::Plain => match icon.sized(cx, geometry::icon_size_small) {
            Some(drawn) => {
                if native_value(cx, geometry::icon_size_small).is_some() {
                    marker_info = marker_info.geometry("icon_size_small");
                }
                marker.icon(MarkerIcon::new().child(drawn))
            }
            None => marker,
        },
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
    let button = labelled(
        ui,
        cx,
        native_info(
            Button::new(id),
            cx,
            geometry::button,
            "button",
            &mut tooltip_info,
        ),
        id,
        label,
    )
    .tooltip(text);
    InfoExt::info(button, ui, id, tooltip_info).debug_selector(move || id.into())
}

/// A Default Button reading `label`, refined by `geometry::button`, under a
/// tooltip the application builds itself, reading `text`.
///
/// `Button::tooltip` takes a string and builds the tooltip itself
/// (`button/button.rs:403`), so the only way to a refined one is to build
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
    );
    let button = labelled(ui, cx, button, id, label);
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
    let button = labelled(
        ui,
        cx,
        native_info(
            Button::new(id),
            cx,
            geometry::button,
            "button",
            &mut button_info,
        ),
        id,
        label,
    )
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

/// Body text reading `text`: a plain `Label` in the theme's line box,
/// `defaults.line_height` times the text size, where upstream lays a Label out
/// at `rems(1.25)` (label.rs, `Label::render`, before the caller's
/// refinement). Without an installed native theme, upstream's.
pub(crate) fn body_label(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: &'static str,
) -> Stateful<Div> {
    let line_height = native_value(cx, |n| n.resolved.defaults.line_height);
    let mut label_info = info::text::gallery_label(cx.theme(), LabelKind::Plain, text);
    let label = match line_height {
        Some(ratio) => {
            label_info = label_info.config(
                "line height",
                format!("defaults.line_height, {ratio} times the text size"),
            );
            Label::new(text).line_height(relative(ratio))
        }
        None => Label::new(text),
    };
    label
        .info(ui, id, label_info)
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
    let mut link_info = info::typography::link(cx.theme())
        .instance("text", text)
        .instance("target", href);
    // One line of `link.font` tall (`line_height_of`).
    let line = native_value(cx, |n| line_height_of(&n, &n.resolved.link.font));
    native_info(
        Link::new(id).child(text).href(href),
        cx,
        geometry::link,
        "link",
        &mut link_info,
    )
    .when_some(line, |link, line| link.line_height(line))
    .info(ui, id, link_info)
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
    /// Vertical, between the groups of a row.
    Vertical,
}

impl SeparatorKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Labelled(_) => "horizontal, labelled",
            Self::Dashed => "horizontal, dashed",
            Self::Vertical => "vertical",
        }
    }
}

/// A `Separator` of `kind`, its line in `separator.line_color` through
/// `Separator::color` where a native theme is installed.
pub(crate) fn separator(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: SeparatorKind,
) -> Stateful<Div> {
    // The plain line under a native theme is the connector's: its thickness
    // is `separator.line_width`, which gpui-component's literal 1px line does
    // not take (separator.rs:78-82).
    #[cfg(feature = "widgets")]
    if kind == SeparatorKind::Horizontal
        && let Some(r) = cx.native_theme().and_then(|nt| nt.resolved(cx))
    {
        // A line of docs/showcase-elements.toml takes no room beyond its
        // own, as the iced and egui showcases lay it out; the others keep
        // some to point at.
        let listed = elements::listed_for(id).is_some();
        return widgets::Separator::horizontal()
            .info(ui, id, info::layout::native_separator(r))
            .when(!listed, |separator| separator.py_1())
            .debug_selector(move || id.into());
    }
    let separator = match kind {
        SeparatorKind::Horizontal => Separator::horizontal(),
        SeparatorKind::Labelled(label) => Separator::horizontal().label(label),
        SeparatorKind::Dashed => Separator::horizontal_dashed(),
        SeparatorKind::Vertical => Separator::vertical(),
    };
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let line = native_color(cx, |n| n.resolved.separator.line_color);
    separator
        .when_some(line, |separator, line| separator.color(line))
        .info(
            ui,
            id,
            info::layout::separator(cx.theme(), kind, native.as_ref().map(|n| n.resolved)),
        )
        // A horizontal Separator's box is as tall as its label and no
        // taller: the line is an absolute child (separator.rs:79-84). The
        // padding leaves something to point at. A vertical one is as tall
        // as its row: its box is `h_full` of the box that reports it
        // (separator.rs:28), which stretches to the row.
        .map(|separator| match kind {
            SeparatorKind::Vertical => separator.self_stretch(),
            _ => separator.py_1(),
        })
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

/// A card: a Fill `GroupBox` with no title, `width` wide, around the text
/// `text`, whose Label reports itself as `text_id`. Its content is refined
/// by `geometry::group_box_content` and filled with `card.background_color`,
/// which the content style lays over the Fill variant's `group_box` token
/// (group_box.rs, `RenderOnce for GroupBox`).
pub(crate) fn card(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text_id: &'static str,
    text: &'static str,
    width: Pixels,
    container_margin: Option<Pixels>,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    let mut card_info = info::layout::card(
        cx.theme(),
        native.as_ref().map(|n| n.resolved),
        container_margin,
    );
    let content_style = native_info(
        StyleRefinement::default(),
        cx,
        geometry::group_box_content,
        "group_box_content",
        &mut card_info,
    );
    // A card is a container, and `layout.container_margin` is the padding
    // inside containers (platform-facts §2.20): it pads a side
    // `card.border.padding` leaves unstated, in place of the Fill
    // GroupBox's own `p_4` (group_box.rs, `GroupBox`).
    let content_style = match (&native, container_margin) {
        (Some(n), Some(margin)) => {
            let p = &n.resolved.card.border.padding;
            card_info = card_info.geometry("container_margin");
            let style = content_style;
            let style = if p.top.is_none() {
                style.pt(margin)
            } else {
                style
            };
            let style = if p.right.is_none() {
                style.pr(margin)
            } else {
                style
            };
            let style = if p.bottom.is_none() {
                style.pb(margin)
            } else {
                style
            };
            if p.left.is_none() {
                style.pl(margin)
            } else {
                style
            }
        }
        _ => content_style,
    };
    let fill = native_color(cx, |n| n.resolved.card.background_color);
    let content_style = match fill {
        Some(fill) => content_style.bg(fill),
        None => content_style,
    };
    GroupBox::new()
        .with_variant(GroupBoxVariant::Fill)
        .content_style(content_style)
        .w(width)
        .child(body_label(ui, cx, text_id, text))
        .info(ui, id, card_info)
        .debug_selector(move || id.into())
}

/// A column of `items` small Labels, each reporting itself, `height` tall in
/// the showcase's frame, scrolled by gpui-component's scrollbar and kept
/// clear of it by `geometry::scrollbar_gutter`.
///
/// The frame is a box of its own around the scrolling viewport: framed
/// itself, the viewport would scroll its border with the content and draw
/// the scrollbar over the border's right edge.
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
    let viewport = native_info(
        div()
            .id(SharedString::from(format!("{id}-viewport")))
            .size_full()
            .debug_selector(move || format!("{id}-viewport"))
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
    })));
    div()
        .h(height)
        .w_full()
        .demo_frame(cx)
        .debug_selector(move || format!("{id}-frame"))
        .child(viewport)
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

/// An Accordion of two expanders, `width` wide: an item per `(title, body
/// id, body)`, open as `open` says; a click on a title hands `on_toggle`
/// which are open after it. Each body is a body-text Label that reports
/// itself.
///
/// What the theme states reaches it per call: each title row is
/// `expander.header_height` tall (`geometry::accordion_title`), and fills
/// with `expander.hover_background` under the pointer through the item's
/// hover style (accordion.rs, `AccordionItem::hover`); each item takes
/// `expander.font`'s size through its own style, which upstream refines it
/// with after its `text_size` (accordion.rs, `RenderOnce for AccordionItem`),
/// and so do the lines between items their colour; the Accordion takes
/// `expander.border`, which it refines its bordered card with last
/// (accordion.rs, `RenderOnce for Accordion`).
#[allow(clippy::too_many_arguments)]
pub(crate) fn expander(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    items: [(&'static str, &'static str, &'static str); 2],
    open: [bool; 2],
    width: Pixels,
    gap: Option<Pixels>,
    on_toggle: impl Fn(&[bool; 2], &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let native = cx.native_theme().and_then(|nt| nt.native(cx));
    if let Some(n) = &native {
        let spec = ExpanderSpec {
            id,
            items,
            open,
            width,
            gap,
        };
        return native_expander(ui, cx, n, spec, Rc::new(on_toggle));
    }
    let mut expander_info = info::layout::expander(
        cx.theme(),
        cx.reduce_motion(),
        items.map(|(title, _, _)| title),
        open,
        native.as_ref().map(|n| n.resolved),
    );
    // `AccordionItem::title_style` takes a refinement rather than being one.
    let title_style = native_info(
        StyleRefinement::default(),
        cx,
        geometry::accordion_title,
        "accordion_title",
        &mut expander_info,
    );
    let accordion = Accordion::new(id)
        .w(width)
        .on_toggle_click(move |open, window, cx| {
            on_toggle(&[open.contains(&0), open.contains(&1)], window, cx)
        });
    let accordion = match &native {
        Some(n) => {
            let b = &n.resolved.expander.border;
            accordion
                .border(px(b.line_width))
                .border_color(info::stated(b.color))
                .rounded(px(b.corner_radius.max(0.0)))
        }
        None => accordion,
    };
    items
        .into_iter()
        .zip(open)
        .fold(accordion, |accordion, ((title, body_id, body), open)| {
            let title_style = title_style.clone();
            let native = native.as_ref().map(|n| {
                let e = &n.resolved.expander;
                (
                    px(native_theme_gpui::scaled_text_size(
                        e.font.size,
                        n.accessibility,
                    )),
                    e.hover_background.map(info::stated),
                    info::stated(e.border.color),
                )
            });
            accordion.item(move |item| {
                let item = item
                    .title_style(title_style)
                    .title(title)
                    .open(open)
                    .child(body_label(ui, cx, body_id, body));
                match native {
                    Some((size, hover, line)) => item
                        .text_size(size)
                        .border_color(line)
                        .when_some(hover, |item, hover| {
                            item.hover(move |style| style.bg(hover))
                        }),
                    None => item,
                }
            })
        })
        .info(ui, id, expander_info)
        .debug_selector(move || id.into())
}

/// What [`native_expander`] draws: the expander `id`, its two items as
/// (title, body id, body), whether each is open, its width, and
/// `layout.widget_gap`, which parts an unframed expander's items and a
/// title from its body.
struct ExpanderSpec {
    id: &'static str,
    items: [(&'static str, &'static str, &'static str); 2],
    open: [bool; 2],
    width: Pixels,
    gap: Option<Pixels>,
}

/// An expander row's side padding where `expander.border.padding` states no
/// side, the space between its title and its arrow where `arrow_gap` is
/// unstated, and its body's side and bottom padding in a frame the theme
/// does not state: upstream's `AccordionItem`'s at the default Size
/// (accordion.rs, `RenderOnce for AccordionItem`: the trigger's `px_3` and
/// `gap_3`, the panel's `pb_2` and `px_3`).
const EXPANDER_PADDING_X: Rems = rems(0.75);
const EXPANDER_ARROW_GAP: Rems = rems(0.75);
const EXPANDER_BODY_BOTTOM: Rems = rems(0.5);

/// [`expander`] under a native theme, drawn by the showcase as `expander.*`
/// states it (ISSUES D8): where `frame_enabled` is true, each item framed by
/// `expander.border` on its own, `layout.widget_gap` from the next; where it
/// is unstated, the whole framed and the items parted by its line, as
/// upstream's bordered `Accordion` frames them; and none of either where it
/// is false (KDE's `KCollapsibleGroupBox` draws neither, docs/platform-facts.md
/// §2.27); each title row `header_height` tall (`geometry::accordion_title`)
/// in `expander.font`, `hover_background` under the pointer; its arrow
/// `arrow_icon_size` in `arrow_color` (the title's colour where that is
/// unstated), on the `arrow_side` the theme states, `arrow_gap` from the
/// title; the body `content_indent` in from the leading edge. Where the
/// theme states no side the arrow is upstream's, after the title; a
/// trailing arrow is upstream's ChevronDown, turned while the item is open,
/// and a leading one ChevronRight turned down while it is open, as Breeze
/// and the HIG draw a leading arrow (§2.27). Upstream builds its arrow's
/// size and colour inline, where no caller reaches them
/// (accordion.rs, `RenderOnce for AccordionItem`).
fn native_expander(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    n: &Native<'_>,
    spec: ExpanderSpec,
    on_toggle: ExpanderToggle,
) -> Stateful<Div> {
    let ExpanderSpec {
        id,
        items,
        open,
        width,
        gap,
    } = spec;
    let e = &n.resolved.expander;
    let colour = info::stated;
    let line = colour(e.border.color);
    let line_width = px(e.border.line_width);
    let text = colour(e.font.color);
    let arrow = colour(e.arrow_color.unwrap_or(e.font.color));
    let hover = e.hover_background.map(colour);
    let title_style = geometry::accordion_title(*n);
    let text_size = px(native_theme_gpui::scaled_text_size(
        e.font.size,
        n.accessibility,
    ));
    let expander_info =
        info::layout::native_expander(n.resolved, items.map(|(title, _, _)| title), open)
            .geometry("accordion_title");
    // KDE's expander has no frame and no line between items (§2.27); where
    // the theme states nothing the frame stays, as upstream's.
    let framed = e.frame_enabled != Some(false);
    let leading = e.arrow_side == Some(ArrowSide::Leading);
    let arrow_gap: DefiniteLength = match e.arrow_gap {
        Some(gap) => px(gap).into(),
        None => EXPANDER_ARROW_GAP.into(),
    };
    let body_indent: DefiniteLength = match e.content_indent {
        Some(indent) => px(indent).into(),
        None => EXPANDER_PADDING_X.into(),
    };
    let radius = px(e.border.corner_radius.max(0.0));
    let last = items.len().saturating_sub(1);
    // Framed where the theme states the frame, each item is an expander of
    // its own, framed on its own and `layout.widget_gap` from the next, as
    // the rows of a layout stand. Framed where it states nothing, the frame
    // is upstream's Accordion's, one for all items, drawn by the items: each
    // its sides and its bottom edge, the first its top, the first's top
    // corners and the last's bottom ones rounded -- so an item's box is its
    // part of the frame, its line included, and the line between two items
    // the one above's bottom edge.
    let own_frames = e.frame_enabled == Some(true);
    let shared_frame = framed && !own_frames;
    with_gap(v_flex(), if shared_frame { None } else { gap })
        .w(width)
        .when(shared_frame, |frame| frame.rounded(radius))
        .overflow_hidden()
        .children(items.into_iter().zip(open).enumerate().map(
            |(ix, ((title, body_id, body), is_open))| {
                let on_toggle = on_toggle.clone();
                // A leading arrow points at the title while closed and down
                // while open; a trailing one down while closed and up while
                // open (§2.27, `arrow_side`).
                let (glyph, turn) = if leading {
                    (IconName::ChevronRight, if is_open { 0.25 } else { 0. })
                } else {
                    (IconName::ChevronDown, if is_open { 0.5 } else { 0. })
                };
                let listed = elements::expander_item(id, ix);
                let arrow = div()
                    .relative()
                    .flex_none()
                    .debug_selector(move || format!("{id}-arrow-{ix}"))
                    .child(
                        Icon::new(glyph)
                            .with_size(px(e.arrow_icon_size))
                            .text_color(arrow)
                            .rotate(gpui::percentage(turn)),
                    )
                    .children(listed.map(|[_, _, arrow, _]| elements::record(ui, arrow)));
                let title = div()
                    .relative()
                    .child(title)
                    .children(listed.map(|[_, _, _, title]| elements::record(ui, title)));
                let header = padded(
                    h_flex().id((id, ix)).w_full().items_center().gap(arrow_gap),
                    &e.border.padding,
                    EXPANDER_PADDING_X,
                    None,
                )
                .refine_style(&title_style)
                .text_size(text_size)
                .line_height(line_height_of(n, &e.font))
                .font_weight(FontWeight(f32::from(e.font.weight)))
                .text_color(text)
                .when_some(hover, |header, hover| {
                    header.hover(move |style| style.bg(hover))
                })
                .debug_selector(move || format!("{id}-header-{ix}"))
                .children(listed.map(|[_, header, _, _]| elements::record(ui, header)))
                .map(|header| {
                    if leading {
                        header.child(arrow).child(title)
                    } else {
                        header.justify_between().child(title).child(arrow)
                    }
                })
                .on_click(move |_, window, cx| {
                    let mut next = open;
                    if let Some(item) = next.get_mut(ix) {
                        *item = !*item;
                    }
                    on_toggle(&next, window, cx);
                });
                // Unframed, the items stand apart as a layout's rows do,
                // `layout.widget_gap` from each other and a title from its
                // body, with no padding of their own. Framed where the theme
                // states the frame, they fill it, a title `layout.widget_gap`
                // above its body, as a layout spaces rows; framed where it
                // states nothing, they pad their body as upstream's
                // AccordionItem does.
                let spaced = match e.frame_enabled {
                    Some(true) | Some(false) => gap,
                    None => None,
                };
                with_gap(v_flex(), spaced)
                    .w_full()
                    .when(own_frames, |item| {
                        item.border(line_width)
                            .border_color(line)
                            .rounded(radius)
                            .overflow_hidden()
                    })
                    .when(shared_frame, |item| {
                        item.border_l(line_width)
                            .border_r(line_width)
                            .border_b(line_width)
                            .border_color(line)
                            .when(ix == 0, |item| item.border_t(line_width).rounded_t(radius))
                            .when(ix == last, |item| item.rounded_b(radius))
                    })
                    .child(header)
                    .when(is_open, |item| {
                        let body = h_flex()
                            .pl(body_indent)
                            .debug_selector(move || format!("{id}-body-{ix}"))
                            .child(body_label(ui, cx, body_id, body));
                        item.child(match spaced {
                            Some(_) => body,
                            None => body.pr(EXPANDER_PADDING_X).pb(EXPANDER_BODY_BOTTOM),
                        })
                    })
                    // An item the list names reports itself as that
                    // element, as the whole expander reports itself.
                    .map(|item| match listed {
                        Some([element, ..]) => item
                            .info(
                                ui,
                                SharedString::from(format!("{id}-item-{ix}")),
                                expander_info.clone().listed(element),
                            )
                            .w_full()
                            .into_any_element(),
                        None => item.into_any_element(),
                    })
            },
        ))
        .info(ui, id, expander_info)
        .debug_selector(move || id.into())
}

/// The Layout page's `Collapsible`, `open` or not: `id` is its info's id
/// and debug selector, `toggle` its toggle Button's and `content` its
/// Label's. The Button shows `icon`, of the chosen icon theme: ChevronDown
/// while `open`, ChevronRight while not.
pub(crate) struct DemoCollapsible {
    pub id: &'static str,
    pub toggle: &'static str,
    pub content: &'static str,
    pub open: bool,
    pub icon: SampleIcon,
}

/// A `Collapsible` toggled by an upstream Ghost Button that runs
/// `on_toggle`, over a small Label, as `spec` describes it. The Collapsible,
/// the Button and the Label each report themselves.
pub(crate) fn collapsible(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: DemoCollapsible,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let DemoCollapsible {
        id,
        toggle,
        content,
        open,
        icon,
    } = spec;
    let text = if open {
        "Click to collapse"
    } else {
        "Click to expand"
    };
    let button = labelled(ui, cx, Button::new(toggle), toggle, text)
        .ghost()
        .when_some(icon.icon(), |button, icon| button.icon(icon))
        .on_click(on_toggle);
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    let button = InfoExt::info(
        button,
        ui,
        toggle,
        info::layout::collapsible_toggle(cx.theme(), open, text, &icon),
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
        // (`carousel/carousel.rs:780-791`), so they belong to the carousel
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
/// click on a step hands `on_click` its index. An Icons Stepper's steps show
/// `icons`, one per step, of the chosen icon theme, and their number where
/// that theme has none. Each step's label is a small Label that reports
/// itself.
pub(crate) fn stepper(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: StepperKind,
    step: usize,
    icons: &[SampleIcon],
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let labelled: Vec<(&str, SampleIcon)> = match kind {
        StepperKind::Icons => STEPPER_STEPS
            .iter()
            .map(|(text, _)| *text)
            .zip(icons.iter().cloned())
            .collect(),
        StepperKind::Numbers => Vec::new(),
    };
    let mut stepper_info =
        info::layout::stepper(cx.theme(), kind, step, STEPPER_STEPS.len(), &labelled);
    // The indicator is a circle (stepper/trigger.rs:118-123) around the
    // icon it is given, which keeps its size (`:138-139`).
    if labelled.iter().any(|(_, icon)| icon.shown())
        && native_value(cx, geometry::icon_size_small).is_some()
    {
        stepper_info = stepper_info.geometry("icon_size_small");
    }
    let stepper = match kind {
        StepperKind::Icons => Stepper::new(id),
        StepperKind::Numbers => Stepper::new(id).vertical(),
    };
    stepper
        .selected_index(step)
        .items(STEPPER_STEPS.iter().enumerate().map(|(ix, (text, _))| {
            let item =
                StepperItem::new().child(label(ui, cx, format!("{id}-step-{}", ix + 1), *text));
            let icon = labelled
                .get(ix)
                .and_then(|(_, icon)| icon.sized(cx, geometry::icon_size_small));
            match icon {
                Some(icon) => item.icon(icon),
                None => item,
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
    let button = labelled(
        ui,
        cx,
        native_info(
            overlay.button_kind().apply(Button::new(id), cx),
            cx,
            geometry::button,
            "button",
            &mut button_info,
        ),
        id,
        label,
    )
    .on_click(on_click);
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into())
}

/// The Overlays page's Dialog: `dialog`, titled, holding `icon`, of the
/// chosen icon theme, beside a description, `gap` apart -- the description
/// alone where that theme has none -- over a footer whose Close Button
/// closes it.
///
/// The Dialog reports itself on its title, its content and its footer; the
/// Button in the footer reports itself (spec §4.3.1).
pub(crate) fn confirm_dialog(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    dialog: Dialog,
    gap: Option<Pixels>,
    icon: &SampleIcon,
) -> Dialog {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut dialog_info = info::overlays::dialog(cx.theme(), cx.reduce_motion(), styled, icon);
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
    if icon.shown() && native_value(cx, geometry::icon_size_dialog).is_some() {
        dialog_info = dialog_info.geometry("icon_size_dialog");
    }
    if icon.shown() && gap.is_some() {
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
                    labelled(&ui, cx, button.refine_style(&close_style), "close", "Close"),
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
    let icon = icon.clone();
    dialog
        .title(title)
        .footer(footer)
        .content(move |content, _window, cx| {
            content.child(
                with_gap(h_flex(), gap)
                    .items_start()
                    .children(icon.sized(cx, geometry::icon_size_dialog))
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
/// and capped at `geometry::dialog_max_width`, with `icon`, of the chosen
/// icon theme -- none where that theme has none -- a title, a description,
/// and Keep and Discard buttons. It reports itself on the icon, the title
/// and the description, the parts the showcase builds.
pub(crate) fn alert_dialog(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    alert: AlertDialog,
    icon: &SampleIcon,
) -> AlertDialog {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut alert_info = info::overlays::alert_dialog(cx.theme(), cx.reduce_motion(), styled, icon);
    let alert = native_info(alert, cx, geometry::dialog, "dialog", &mut alert_info);
    // An AlertDialog has no `max_w` of its own; `Styled::max_w` lands on the
    // surface through the refinement (dialog/dialog.rs:677).
    let alert = match native_value(cx, geometry::dialog_max_width) {
        Some(width) => {
            alert_info = alert_info.geometry("dialog_max_width");
            alert.max_w(width)
        }
        None => alert,
    };
    let drawn = icon.sized(cx, geometry::icon_size_dialog);
    if drawn.is_some() && native_value(cx, geometry::icon_size_dialog).is_some() {
        alert_info = alert_info.geometry("icon_size_dialog");
    }
    let alert = match drawn {
        Some(drawn) => alert.icon(
            drawn
                .info(ui, "overlays-alert-dialog-icon", alert_info.clone())
                .debug_selector(|| "overlays-alert-dialog-icon".into()),
        ),
        None => alert,
    };
    alert
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
    let trigger = labelled(
        ui,
        cx,
        native_info(
            Button::new("overlays-popover-trigger"),
            cx,
            geometry::button,
            "button",
            &mut popover_info,
        ),
        "overlays-popover-trigger",
        "Click for Popover",
    );
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
/// otherwise (popover.rs:308), so the content adds none. The trigger and the
/// content report the HoverCard.
pub(crate) fn hover_card(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    gap: Option<Pixels>,
) -> Stateful<Div> {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut card_info = info::overlays::hover_card(cx.theme(), styled);
    let trigger = labelled(
        ui,
        cx,
        native_info(
            ButtonKind::Ghost.apply(Button::new("overlays-hover-card-trigger"), cx),
            cx,
            geometry::button,
            "button",
            &mut card_info,
        ),
        "overlays-hover-card-trigger",
        "KDE Breeze",
    );
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
    labelled(
        ui,
        cx,
        native_info(
            Button::new(id),
            cx,
            geometry::button,
            "button",
            &mut menu_info,
        ),
        id,
        label,
    )
    .dropdown_menu(|menu, _window, _cx| sample_menu(menu))
    .info(ui, id, menu_info)
    .debug_selector(move || id.into())
}

/// The width of the Overlays page's frame of application-drawn menu rows:
/// the showcase's own, as the model states no menu width.
const MENU_ROWS_WIDTH: Pixels = px(220.);

/// The menu rows an application draws itself, one per `(id, icon, label)`
/// of `rows`, in a frame painted as a menu is. Each icon is the chosen icon
/// theme's, and a row whose icon that theme lacks shows its label alone.
/// Upstream's `MenuItemElement` is crate-private and `PopupMenu` builds its
/// own rows, so `geometry::menu_item` has no widget to refine: these rows
/// are the receiver it documents.
pub(crate) fn menu_rows(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    rows: &[(&'static str, SampleIcon, &'static str)],
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
            let mut row_info = info::overlays::menu_row(t, label, icon);
            if icon.shown() && native_value(cx, geometry::icon_size_small).is_some() {
                row_info = row_info.geometry("icon_size_small");
            }
            // Plain text, not a Label: a Label paints foreground on its
            // own element (label.rs:211), over the frame's colour.
            let row = div()
                .flex()
                .items_center()
                .hover(move |this| this.bg(hover_bg).text_color(hover_text))
                .children(icon.sized(cx, geometry::icon_size_small))
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

/// One cell of an Icons page gallery: the icon the chosen icon theme gives
/// for `label`, labelled with it.
pub(crate) struct IconCell<'a> {
    /// The role's or the IconName's name.
    pub(crate) label: &'a str,
    /// The chosen icon theme, as the page names it.
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
    ChartGrid,
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

// ---------------------------------------------------------------------------
// The Basic page's own samples (Basic v3)
// ---------------------------------------------------------------------------

/// `button` labelled `label`. Where a native theme is installed, or the label
/// is an element of docs/showcase-elements.toml, the label is the Button's
/// child, in the box upstream puts its own `label` in (button/button.rs,
/// `RenderOnce for Button`: `min_w_0`, `whitespace_nowrap`, `text_ellipsis`):
/// in `button.font` through `geometry::button_label`, because upstream sizes
/// a `.label()` from the `Size` enum on an element no style reaches, and
/// recording its bounds where it is listed. The Button is named by the label
/// as `Button::label` would name it. Otherwise `Button::label`.
/// `id` is also the label key of docs/showcase-elements.toml: an `id` that
/// is not listed there gets no bounds record, by design.
pub(crate) fn labelled(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    button: Button,
    id: &str,
    label: impl Into<SharedString>,
) -> Button {
    let label = label.into();
    let text = native_geometry(cx, geometry::button_label);
    let listed = elements::label_of(id);
    if text.is_none() && listed.is_none() {
        return button.label(label);
    }
    let boxed = div()
        .relative()
        .min_w_0()
        .whitespace_nowrap()
        .text_ellipsis();
    button.accessibility_label(label.clone()).child(
        refined(boxed, text.as_ref())
            .child(label)
            .when_some(listed, |this, listed| {
                this.child(elements::record(ui, listed))
            }),
    )
}

/// `icon` in a box of its own size that records its bounds as the element
/// `listed`, where it is one.
fn listed_icon(ui: &Entity<InfoRegistry>, icon: Icon, listed: Option<&'static str>) -> AnyElement {
    match listed {
        Some(listed) => div()
            .relative()
            .flex_none()
            .child(icon)
            .child(elements::record(ui, listed))
            .into_any_element(),
        None => icon.into_any_element(),
    }
}

/// A tool button's box where the theme states the button's padding: its
/// icon and the `button.border.padding` sides the theme states round it, in
/// place of the height and the side padding upstream gives a Button of its
/// Size (button/button.rs, `RenderOnce for Button`: `h_6`/`h_8`,
/// `px_2`/`px_2p5`); a side the theme leaves unstated keeps upstream's. It
/// is rounded by `button.border.corner_radius` in place of the Theme's
/// radius the Button takes (the same, `rounding`). A
/// Ghost Button draws no border (the same, `border_l_1` and its siblings go
/// to the Default and outline variants alone). `None` before `apply` ran.
fn tool_button_box(cx: &App) -> Option<StyleRefinement> {
    native_value(cx, tool_box_of)
}

/// [`tool_button_box`] under the native theme `n`.
fn tool_box_of(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button.border;
    let p = &b.padding;
    let style = StyleRefinement::default().rounded(px(b.corner_radius.max(0.)));
    let style = if p.top.is_some() || p.bottom.is_some() {
        style.h_auto()
    } else {
        style
    };
    let side = |style: StyleRefinement,
                v: Option<f32>,
                f: fn(StyleRefinement, Pixels) -> StyleRefinement| {
        match v {
            Some(v) => f(style, px(v)),
            None => style,
        }
    };
    let style = side(style, p.top, |s, v| s.pt(v));
    let style = side(style, p.right, |s, v| s.pr(v));
    let style = side(style, p.bottom, |s, v| s.pb(v));
    side(style, p.left, |s, v| s.pl(v))
}

/// A toggle button that is on: a Default Button reading `label`, refined by
/// `geometry::button`, shown selected, and refined by
/// `geometry::button_checked` -- filled with `button.checked_background` and
/// lettered in `button.checked_text_color`, the pressed pair where the theme
/// states none. Upstream's selected style would fill it with its
/// `button_active` token and letter it in `button_foreground`
/// (button/button.rs, `ButtonVariant::selected`); the Button replays the
/// caller's style over that state (button/button.rs, `RenderOnce for
/// Button`, `styles.selected`), so these win.
pub(crate) fn toggle_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
) -> Stateful<Div> {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let mut button_info = info::buttons::button(
        cx.theme(),
        ButtonKind::Default,
        ButtonState::Idle,
        false,
        None,
        styled,
    )
    .instance(
        "state",
        "selected: on, in button.checked_background and button.checked_text_color (the pressed pair where unstated)",
    );
    let button = native_info(
        Button::new(id),
        cx,
        geometry::button,
        "button",
        &mut button_info,
    );
    let button = native_info(
        button.selected(true).toggled(true),
        cx,
        geometry::button_checked,
        "button_checked",
        &mut button_info,
    );
    InfoExt::info(labelled(ui, cx, button, id, label), ui, id, button_info)
        .debug_selector(move || id.into())
}

/// An icon-only button as the toolbar's (`toolbar_button`): the Ghost
/// variant, its icon native-theme's for a role as the chosen set gives it
/// (`Showcase::role_chrome_icon`), at `toolbar.icon_size`, the icon
/// recording its bounds as the element `listed_icon`. Where the set has no
/// icon for the role the button shows `name` instead -- never another set's
/// icon.
pub(crate) fn icon_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    name: &'static str,
    drawn: &(ChromeIcon, Option<IconName>),
    listed: &'static str,
) -> Stateful<Div> {
    let styled = cx.native_theme().and_then(|nt| nt.native(cx)).is_some();
    let icon = role_icon_of(drawn);
    let mut button_info = info::buttons::button(
        cx.theme(),
        ButtonKind::Ghost,
        ButtonState::Idle,
        icon.is_some(),
        None,
        styled,
    )
    .instance("icon", format!("{name}, at toolbar.icon_size"));
    let icon = icon.map(|icon| native_sized(cx, icon, geometry::icon_size_toolbar));
    if icon.is_some() && native_value(cx, geometry::icon_size_toolbar).is_some() {
        button_info = button_info.geometry("icon_size_toolbar");
    }
    let button = refined(
        ButtonKind::Ghost.apply(Button::new(id), cx),
        tool_button_box(cx).as_ref(),
    );
    let button = match icon {
        Some(icon) => button
            .accessibility_label(name)
            .child(listed_icon(ui, icon, Some(listed))),
        None => labelled(ui, cx, button, id, name),
    };
    InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into())
}

/// A line of text of the Basic page's Typography group.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TypeRole {
    Caption,
    Body,
    SectionHeading,
    DialogTitle,
    Monospace,
}

/// `text` in `role`'s type: a `text_scale` entry's size, weight and line
/// height in `defaults.font`'s family and colour; `defaults.font` itself at
/// `defaults.line_height`; or `defaults.mono_font` at `defaults.line_height`
/// -- each size and line height scaled by the text-scaling factor, as the
/// section headings are (`heading`). Without a native theme, the Label's own
/// type.
pub(crate) fn type_line(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    role: TypeRole,
    text: &'static str,
) -> Stateful<Div> {
    let look = native_value(cx, |n| {
        let r = n.resolved;
        let scaled = |v: f32| px(native_theme_gpui::scaled_text_size(v, n.accessibility));
        let entry = |e: &native_theme::theme::ResolvedTextScaleEntry| {
            (scaled(e.size), scaled(e.line_height), e.weight)
        };
        let font = &r.defaults.font;
        let lined = |f: &ResolvedFontSpec| {
            (
                scaled(f.size),
                scaled(f.size * r.defaults.line_height),
                f.weight,
            )
        };
        let ((size, line_height, weight), family, colour) = match role {
            TypeRole::Caption => (entry(&r.text_scale.caption), &font.family, font.color),
            TypeRole::Body => (lined(font), &font.family, font.color),
            TypeRole::SectionHeading => (
                entry(&r.text_scale.section_heading),
                &font.family,
                font.color,
            ),
            TypeRole::DialogTitle => (entry(&r.text_scale.dialog_title), &font.family, font.color),
            TypeRole::Monospace => (
                lined(&r.defaults.mono_font),
                &r.defaults.mono_font.family,
                r.defaults.mono_font.color,
            ),
        };
        (
            size,
            line_height,
            FontWeight(f32::from(weight)),
            native_theme_gpui::font_family(family),
            info::stated(colour),
        )
    });
    let label = Label::new(text);
    let label = match look {
        Some((size, line_height, weight, family, colour)) => label
            .text_size(size)
            .line_height(line_height)
            .font_weight(weight)
            .font_family(family)
            .text_color(colour),
        None => label,
    };
    label
        .info(ui, id, info::text::label(cx.theme()).variant(text))
        .self_start()
        .debug_selector(move || id.into())
}

/// The `Icon` a role's `drawn` icon is (`Showcase::role_chrome_icon`):
/// gpui-component's own where its built-in set is chosen, the chosen set's
/// SVG otherwise, drawn as `chrome_icon` draws one; `None` where the set has
/// none for the role.
fn role_icon_of(drawn: &(ChromeIcon, Option<IconName>)) -> Option<Icon> {
    match drawn {
        (ChromeIcon::Builtin(_), Some(icon)) => Some(Icon::new(icon.clone())),
        (ChromeIcon::Loaded(_, bytes), _) => Some(Icon::default().data(bytes)),
        _ => None,
    }
}

/// native-theme's icon for a role, `drawn`, of the chosen set, at the size
/// `role` gives it (`geometry::icon_size_small`, `_toolbar`, `_large`),
/// reporting itself as `variant`. Where the set has no icon for the role,
/// nothing is drawn -- never another set's.
pub(crate) fn sized_icon(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    drawn: &(ChromeIcon, Option<IconName>),
    role: fn(Native<'_>) -> Size,
    variant: &'static str,
) -> Stateful<Div> {
    let shown = role_icon_of(drawn).map(|icon| native_sized(cx, icon, role));
    let info = WidgetInfo::new("Icon")
        .variant(variant)
        .instance("icon", "IconRole::FolderOpen, of the chosen set");
    div()
        .children(shown)
        .info(ui, id, info)
        .debug_selector(move || id.into())
}

/// One row of [`files_table`]: its two cells' texts, the elements of
/// docs/showcase-elements.toml its cells are, where they are ones, its font
/// as (size, weight, line height, colour), its fill, and whether the
/// header's line closes it.
struct TableLine {
    cells: [&'static str; 2],
    listed: [Option<&'static str>; 2],
    font: (Pixels, FontWeight, Pixels, Hsla),
    fill: Option<Hsla>,
    closed: bool,
}

/// The Basic page's table, `width` wide across its outer border, laid out
/// as the iced and egui showcases lay theirs out, a table none of the three
/// toolkits' own tables can style so (gpui-component's `TableRow` draws a
/// line of its own between rows, after the caller's style: table/table.rs,
/// `TableRow::render`): two columns, each half the width inside the frame;
/// a header row reading `head` over a row for each of `rows`; every cell
/// padded by the four `list.border.padding` sides (none on a side the theme
/// leaves unstated) and every row `list.row_height` tall where the theme
/// states it, else its line -- its font's size by `defaults.line_height` --
/// and that padding; the header in `list.header_background` and
/// `list.header_font`, the rows in `list.item_font`, the row at `selected`
/// in `list.selection_background` and `selection_text_color`; a
/// `list.grid_color` line `separator.line_width`
/// wide inside the header's bottom edge and one down the first column's
/// right edge through every row; the whole framed by `list.border` on
/// `list.background_color`. The header, its two cells and every row report
/// themselves. Without a native theme, the Data page's declarative `Table`.
pub(crate) fn files_table(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    head: [(&'static str, &'static str); 2],
    rows: &[[&'static str; 2]],
    selected: usize,
    width: Pixels,
) -> Stateful<Div> {
    let Some(n) = cx.native_theme().and_then(|nt| nt.native(cx)) else {
        let [(first, _), (second, _)] = head;
        return table(ui, cx, id, "Files", [first, second], rows);
    };
    let t = cx.theme();
    let r = n.resolved;
    let l = &r.list;
    let colour = info::stated;
    let grid = colour(l.grid_color);
    let line = px(r.separator.line_width);
    let pad = &l.border.padding;
    let side = |v: Option<f32>| v.map_or(px(0.), px);
    let font = |f: &ResolvedFontSpec, colour: Hsla| {
        let size = native_theme_gpui::scaled_text_size(f.size, n.accessibility);
        (
            px(size),
            FontWeight(f32::from(f.weight)),
            px(size * r.defaults.line_height),
            colour,
        )
    };
    let height = |line_height: Pixels| {
        l.row_height
            .map_or(side(pad.top) + line_height + side(pad.bottom), px)
    };
    let row =
        |spec: TableLine| {
            let (size, weight, line_height, text) = spec.font;
            let cells = spec.cells.into_iter().zip(spec.listed).enumerate().map(
                |(ix, (content, listed))| {
                    div()
                        .relative()
                        .w(relative(0.5))
                        .h_full()
                        .flex()
                        .items_center()
                        .pt(side(pad.top))
                        .pr(side(pad.right))
                        .pb(side(pad.bottom))
                        .pl(side(pad.left))
                        .child(content)
                        .when(ix == 0, |cell| {
                            cell.child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .bottom_0()
                                    .right_0()
                                    .w(line)
                                    .bg(grid),
                            )
                        })
                        .children(listed.map(|listed| elements::record(ui, listed)))
                },
            );
            h_flex()
                .relative()
                .w_full()
                .h(height(line_height))
                .text_size(size)
                .font_weight(weight)
                .line_height(line_height)
                .text_color(text)
                .when_some(spec.fill, |row, fill| row.bg(fill))
                .children(cells)
                .when(spec.closed, |row| {
                    row.child(
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .bottom_0()
                            .h(line)
                            .bg(grid),
                    )
                })
        };
    let [first, second] = head;
    let header = row(TableLine {
        cells: [first.0, second.0],
        listed: [Some(first.1), Some(second.1)],
        font: font(&l.header_font, colour(l.header_font.color)),
        fill: Some(colour(l.header_background)),
        closed: true,
    })
    .info(
        ui,
        SharedString::from(format!("{id}-header")),
        info::data::table_header(t, &format!("{}, {}", first.0, second.0)),
    )
    .w_full();
    let body = rows.iter().enumerate().map(|(ix, cells)| {
        let (fill, text) = if ix == selected {
            (
                Some(colour(l.selection_background)),
                colour(l.selection_text_color),
            )
        } else {
            (None, colour(l.item_font.color))
        };
        // Under the pointer, a row that is not the selected one takes the
        // list's hover fill and text colour.
        let (hover, hover_text) = (colour(l.hover_background), colour(l.hover_text_color));
        row(TableLine {
            cells: *cells,
            listed: [None, None],
            font: font(&l.item_font, text),
            fill,
            closed: false,
        })
        .id(("table-row", ix))
        .when(ix != selected, |row| {
            row.hover(move |s| s.bg(hover).text_color(hover_text))
        })
        .info(
            ui,
            SharedString::from(format!("{id}-row-{ix}")),
            info::data::table_row(t, ix == 0, &cells.join(", ")),
        )
        .w_full()
    });
    v_flex()
        .w(width)
        .bg(colour(l.background_color))
        .border(px(l.border.line_width))
        .border_color(colour(l.border.color))
        .rounded(px(l.border.corner_radius.max(0.0)))
        .overflow_hidden()
        .child(header)
        .children(body)
        .info(ui, id, info::data::table(t, rows.len()))
        .self_start()
        .debug_selector(move || id.into())
}
