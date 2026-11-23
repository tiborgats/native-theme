//! Demo helpers the pages share.

use std::{cell::Cell, num::NonZeroU32, rc::Rc, time::Duration};

use gpui::DefiniteLength;
use gpui::{
    Action, AnyElement, App, Axis, ClickEvent, ClipboardItem, Context, Div, ElementId, Entity,
    FontWeight, Global, Hsla, ImageSource, KeyDownEvent, Keystroke, Length, MouseButton, Pixels,
    Rems, RenderOnce, SharedString, Stateful, StyleRefinement, StyledText, Window, anchored,
    deferred, div, prelude::*, px, relative, rems,
};
use gpui_base::{ResizeHandleContext, ResizeHandleRenderer};
#[cfg(feature = "widgets")]
use gpui_component::select::Caret;
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
        Button, ButtonCustomVariant, ButtonGroup, ButtonVariant, ButtonVariants, DropdownButton,
        Toggle, ToggleGroup,
    },
    calendar::{Calendar, CalendarState},
    carousel::{
        Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPagination,
        CarouselPaginationItem, CarouselPrevious, CarouselState,
    },
    chart::{AreaChart, BarChart, CandlestickChart, LineChart, PieChart, RadarChart, SankeyChart},
    checkbox::Checkbox,
    clipboard::Clipboard,
    color_picker::{ColorPicker, ColorPickerState, ColorSelect},
    combobox::{Combobox, ComboboxState},
    command::{Command, CommandGroup, CommandItem, CommandState},
    date_picker::{DatePicker, DatePickerState},
    description_list::DescriptionList,
    dialog::{
        AlertDialog, Dialog, DialogButtonProps, DialogClose, DialogDescription, DialogEntrance,
        DialogFooter, DialogTitle,
    },
    diff::{Diff, DiffDecoration, DiffHoverHighlight, DiffLineRange, DiffSide, DiffState},
    empty::{
        Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant,
        EmptyTitle,
    },
    form::{Field, Form},
    grid_layout::{GridItem, GridLayoutState, grid_layout},
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
    plot::shape::SankeyLink,
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
    speech::{SpeechIndicator, SpeechState, SpeechWaveform},
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
use native_theme::next::WidgetState;
use native_theme::theme::{ArrowSide, FontStyle, IconSet, ResolvedFontSpec, ResolvedPadding};
use native_theme_gpui::icons::with_spin_animation;
use native_theme_gpui::next;
#[cfg(feature = "widgets")]
use native_theme_gpui::widgets;
use native_theme_gpui::{
    AccessibilityPreferences, ActiveNativeTheme as _, Native, geometry, variants,
};

use crate::app::{Quit, ShowPage};
use crate::elements::{self, ListedExt as _};
use crate::info::{
    self, InfoExt, InfoRegistry, WidgetInfo, as_rgba, native_info, next_native_info,
};
#[cfg(feature = "widgets")]
use crate::support::old_resolved;
use crate::support::{
    CAROUSEL_SLIDES, ChatMessage, ChromeIcon, DIFF_DECORATED_LINE, DIFF_DECORATED_PATH,
    GRID_LAYOUT_ITEMS, NativeStyled as _, PresetDelegate, STEPPER_STEPS, SampleIcon,
    SampleListDelegate, SampleTableDelegate, contrast_held, native_color, native_geometry,
    native_value, next_geometry, next_resolved, next_value, old_native, refined,
    requested_preferences, toolbar_variant, with_gap, with_padding,
};
use crate::{
    CHROME_APP_MENU_BAR, CHROME_SIDE_PANEL, CHROME_SIDE_PANEL_CONTENT, CHROME_SIDE_PANEL_SCROLL,
    CHROME_SPLITTER_LINE, CHROME_THEME_SETTINGS, DATA_TABLE_HEADER, OVERLAY_ABOUT_LINK,
    OVERLAY_ABOUT_NAME, OVERLAY_ABOUT_TEXT, OVERLAY_ABOUT_TITLE, OVERLAY_PALETTE,
    OVERLAY_PALETTE_TITLE, OVERLAY_PREFERENCES, OVERLAYS_DIALOG_CLOSE, OVERLAYS_DIALOG_FOOTER,
    PREF_HIGH_CONTRAST, PREF_REDUCE_MOTION, PREF_REDUCE_TRANSPARENCY, PROBE_CAROUSEL_LAST,
    PROBE_SETTINGS_ROW, Page, STATUS_ENVIRONMENT, STATUS_HOVERED, STATUS_MIDDLE, TREE_DEMO, probe,
};

/// The `Icon` `drawn` is: gpui-component's `icon` where the built-in set is
/// chosen, the chosen set's SVG otherwise, and `None` where that set has
/// none, or pixels alone, which an `Icon` cannot draw (icon.rs,
/// `IconSource`), or an SVG in several colours, which it would draw as a
/// mask of one colour: for a widget that takes an `Icon` and nothing else
/// (`ChromeIcon::svg_only`). An SVG is drawn as every `Icon` is, as a mask in
/// the colour its widget gives it (gpui-pre window.rs, `Window::paint_svg`).
/// A part: the helper that places it reports.
fn chrome_icon(drawn: &ChromeIcon, icon: &IconName) -> Option<Icon> {
    match drawn {
        ChromeIcon::Builtin(_) => Some(Icon::new(icon.clone())),
        ChromeIcon::Loaded(_, bytes) => Some(Icon::default().data(bytes)),
        ChromeIcon::Raster(..) | ChromeIcon::Missing(_) | ChromeIcon::Unlisted(_) => None,
    }
}

/// What `drawn` is drawn as where its widget takes any element: the `Icon`
/// [`chrome_icon`] gives, or the chosen set's pixels, or its SVG in several
/// colours, as an image, as the Icons page draws them; `None` where the set
/// has none.
fn chrome_glyph(drawn: &ChromeIcon, icon: &IconName) -> Option<Glyph> {
    match drawn {
        ChromeIcon::Raster(name, raster) => Some(Glyph::Raster(name, raster.0.clone(), None)),
        other => chrome_icon(other, icon).map(Glyph::svg),
    }
}

/// The debug selector of a raster glyph of the icon `name`, for the
/// self-tests: `raster-<name>`.
pub(crate) fn raster_selector(name: &str) -> String {
    format!("raster-{name}")
}

/// An icon of the chosen set as a widget that takes any element draws it:
/// an SVG as an `Icon`, or pixels, or an SVG in several colours, as an
/// image (`gpui::img`). An image takes no text colour: a system set's
/// pixels are in the colour its loader drew them in (`app::icon_tint`), an
/// SVG in several colours in its own, and a bundled set has no pixels.
#[derive(Clone, IntoElement)]
pub(crate) enum Glyph {
    Svg(Box<Icon>),
    /// The icon's name, its decoded image, and the size it is drawn at;
    /// `None` draws it at the text size, as an `Icon` given no size is drawn
    /// (icon.rs, `Icon::into_svg`).
    Raster(&'static str, ImageSource, Option<Size>),
}

impl Glyph {
    /// `icon` as a glyph.
    fn svg(icon: Icon) -> Self {
        Self::Svg(Box::new(icon))
    }

    /// The glyph at `size`, as `Sizable::with_size` sizes an `Icon`.
    fn with_size(self, size: Size) -> Self {
        match self {
            Self::Svg(icon) => Self::svg(icon.with_size(size)),
            Self::Raster(name, source, _) => Self::Raster(name, source, Some(size)),
        }
    }

    /// The glyph in `colour`: an `Icon`'s mask; an image keeps its own.
    fn text_color(self, colour: Hsla) -> Self {
        match self {
            Self::Svg(icon) => Self::svg(icon.text_color(colour)),
            raster @ Self::Raster(..) => raster,
        }
    }

    /// The glyph turned by `turn`: an `Icon` through its transformation; an
    /// image as it is, since `gpui::img` takes no transformation.
    fn rotate(self, turn: impl Into<gpui::Radians>) -> Self {
        match self {
            Self::Svg(icon) => Self::svg(icon.rotate(turn)),
            raster @ Self::Raster(..) => raster,
        }
    }

    /// The icon's name where the glyph is pixels.
    #[cfg(test)]
    fn raster_name(&self) -> Option<&'static str> {
        match self {
            Self::Svg(_) => None,
            Self::Raster(name, ..) => Some(name),
        }
    }
}

impl RenderOnce for Glyph {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        match self {
            Self::Svg(icon) => icon.into_any_element(),
            // Square, at the side an `Icon` of the same `Size` takes
            // (icon.rs, `Icon::into_svg`: `size_3`, `size_3p5`, `size_4`,
            // `size_6`, or the text size where it has none).
            Self::Raster(name, source, size) => {
                let side: Length = match size {
                    Some(Size::Size(side)) => side.into(),
                    Some(Size::XSmall) => rems(0.75).into(),
                    Some(Size::Small) => rems(0.875).into(),
                    Some(Size::Medium) => rems(1.).into(),
                    Some(Size::Large) => rems(1.5).into(),
                    None => window
                        .text_style()
                        .font_size
                        .to_pixels(window.rem_size())
                        .into(),
                };
                gpui::img(source)
                    .flex_shrink_0()
                    .size(side)
                    .debug_selector(move || raster_selector(name))
                    .into_any_element()
            }
        }
    }
}

impl SampleIcon {
    /// The `Icon` a page sample draws where its widget takes an `Icon`
    /// alone, as `chrome_icon` gives it: `None` where the chosen icon theme
    /// has none, or pixels alone (`SampleIcon::svg_only`). A part: the
    /// sample that shows it reports.
    fn icon(&self) -> Option<Icon> {
        chrome_icon(&self.drawn, &self.icon)
    }

    /// The `Icon` at `size`, a size of the installed 0.8.0 theme, or at its
    /// own where that is `None`; `None` where the chosen icon theme has none.
    fn next_sized(&self, size: Option<Size>) -> Option<Icon> {
        self.icon().map(|icon| match size {
            Some(size) => icon.with_size(size),
            None => icon,
        })
    }

    /// The glyph a page sample draws where its widget takes any element,
    /// as `chrome_glyph` gives it: `None` where the chosen icon theme has
    /// none.
    fn glyph(&self) -> Option<Glyph> {
        chrome_glyph(&self.drawn, &self.icon)
    }

    /// The glyph at the platform's size for the role the builder names, as
    /// `native_sized_glyph` sizes it; `None` where the chosen icon theme has
    /// none.
    fn glyph_sized(&self, cx: &App, role: fn(Native<'_>) -> Size) -> Option<Glyph> {
        self.glyph()
            .map(|glyph| native_sized_glyph(cx, glyph, role))
    }

    /// This sample for a widget that takes an `Icon` alone
    /// (`ChromeIcon::svg_only`), whose info then says what it shows.
    fn svg_only(&self) -> Self {
        Self {
            drawn: self.drawn.clone().svg_only(),
            ..self.clone()
        }
    }
}

/// `glyph` at the platform's size for the role the builder names; its own
/// size before `apply` ran.
fn native_sized_glyph(cx: &App, glyph: Glyph, role: fn(Native<'_>) -> Size) -> Glyph {
    match native_value(cx, role) {
        Some(size) => glyph.with_size(size),
        None => glyph,
    }
}

/// A `TitleBar` refined by `geometry::title_bar`, reading `label`, holding
/// the application's menus (`app_menus`) where the platform has no menu bar
/// of its own, and quitting
/// the application from its close button: the window's title bar, where the
/// window was granted client-side decorations (spec S8). While the window is
/// not `active` it is refined by `geometry::title_bar_inactive` as well.
pub(crate) fn title_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    label: impl Into<SharedString>,
    menus: &Entity<MenuBar>,
    active: bool,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    // After `geometry::title_bar`: the inactive window's fill and text,
    // over upstream's gradient (title_bar.rs:345-349).
    let inactive = (!active)
        .then(|| native_geometry(cx, geometry::title_bar_inactive))
        .flatten();
    let mut bar_info = info::title_bar(cx.theme(), &label, inactive.is_some());
    let bar = native_info(
        TitleBar::new(),
        cx,
        geometry::title_bar,
        "title_bar",
        &mut bar_info,
    );
    if inactive.is_some() {
        bar_info = bar_info.geometry("title_bar_inactive");
    }
    let bar = refined(bar, inactive.as_ref())
        // Linux only: upstream drops the handler on every other platform
        // (title_bar.rs:96-105). On Windows the OS acts on the X itself; macOS
        // draws its own controls.
        .on_close_window(|_, window, cx| window.dispatch_action(Box::new(Quit), cx))
        // Plain text, not a Label: `Label::render` sets `foreground` on its own
        // element (label.rs:213), which would hide the title-bar font's colour
        // `geometry::title_bar` just gave the bar.
        .child(label)
        .when(cfg!(not(target_os = "macos")), |bar| {
            bar.child(app_menus(ui, cx, info::MenuHost::TitleBar, menus))
        });
    bar.info(ui, "chrome-title-bar", bar_info)
}

/// A menu title's side and top/bottom padding where `menu.border.padding`
/// states no side: upstream's for a title of its `AppMenuBar`, a Small
/// compact Button (button/button.rs:697-699, `px_1p5`) given `py_0p5`
/// (menu/app_menu_bar.rs:276).
const MENU_TITLE_PADDING_X: Rems = rems(0.375);
const MENU_TITLE_PADDING_Y: Rems = rems(0.125);

/// A menu row's side padding where `menu.border.padding` states no side,
/// and its height where neither `menu.row_height` nor a vertical padding
/// side is stated: upstream's `PopupMenu` row at the default Size, in rems
/// since gpui-component 0.9.0 (gpui-component 0.9.1
/// menu/popup_menu.rs:1328, `INNER_PADDING`; :1334, the item height), so
/// both follow the window's rem, the theme's font size. The height takes
/// the connector's control-height rule (geometry.rs, `height_rule`): it
/// is the row's height at a text scale of 1 or less and its minimum above.
pub(crate) const MENU_ROW_PADDING_X: Rems = rems(0.5);
pub(crate) const MENU_ROW_HEIGHT: Rems = rems(1.625);

/// What upstream's `PopupMenu` leaves round and between its rows, and
/// between a row's label and its shortcut (menu/popup_menu.rs:1617-1619,
/// `p_1`, `gap_y_0p5`, `min_w(rems(8.))`; :1449, `gap_3`); the model states
/// none of them.
const MENU_POPUP_PADDING: Rems = rems(0.25);
pub(crate) const MENU_POPUP_ROW_GAP: Rems = rems(0.125);
const MENU_POPUP_MIN_WIDTH: Rems = rems(8.);
const MENU_SHORTCUT_GAP: Rems = rems(0.75);

/// A menu separator's thickness without a native theme: upstream's
/// `PopupMenu` separator (menu/popup_menu.rs:1380, `border_b(px(2.))`).
/// With one, it is `separator.line_width`.
const MENU_SEPARATOR: Pixels = px(2.);

/// The two blurred layers of the popup's drop shadow where
/// `popover.border.shadow_enabled` holds, as (y offset, blur radius, spread
/// radius), drawn in `defaults.shadow_color`: the model states no shadow
/// geometry, so these are gpui-component's popup surface shadow's
/// (gpui-component 0.9.1 src/styled.rs:68-73, `popover_shadow`), without its
/// ring (:65-67), whose place the frame in `popover.border` takes.
const MENU_POPUP_SHADOW: [(Pixels, Pixels, Pixels); 2] =
    [(px(4.), px(3.), px(-1.)), (px(2.), px(2.), px(-2.))];

/// The model's menu, as the showcase draws a menu title and a menu row:
/// `menu.font` and a row's size as `geometry::menu_item` refines a menu item,
/// `menu.hover_background` and `hover_text_color`, a lit row's outline in
/// `menu.hover_border_color`, a pressed row's `menu.active_background` and
/// `active_text_color`, a title's `menu.bar_*` fills, text and padding,
/// `menu.border`'s padding sides and corner radius (platform-facts §2.6: the
/// items are rectangular where it is 0), and the popup `menu.background_color`
/// framed by the popover's border (§2.6: the popup border is §2.16's), its
/// rows inset by `menu.panel_padding_*` where stated.
#[derive(Clone)]
pub(crate) struct MenuLook {
    /// `geometry::menu_item`: a row's font, stated padding sides and height.
    item: StyleRefinement,
    text: Hsla,
    hover: Hsla,
    hover_text: Hsla,
    /// `menu.hover_border_color`, a lit row's outline: `None` where the
    /// theme leaves it unstated or `defaults.border.line_width` is 0.
    edge: Option<Hsla>,
    /// `defaults.border.line_width`, the outline's width.
    edge_width: Pixels,
    /// A pressed row's fill and text: `menu.active_background` and
    /// `active_text_color`, each `None` where unstated, the row then keeping
    /// its hover look (the fill's outline included) or its hover text.
    active: Option<Hsla>,
    active_text: Option<Hsla>,
    /// A title's fill under the pointer and while its menu is open, and its
    /// text in both: `menu.bar_hover_background`, `bar_open_background` and
    /// `bar_hover_text_color`, each falling back as its doc says, to the
    /// item's hover colours where all are unstated.
    bar_hover: Hsla,
    bar_open: Hsla,
    bar_text: Hsla,
    radius: Pixels,
    padding: ResolvedPadding,
    /// A title's padding: `menu.border.padding`, with
    /// `menu.bar_padding_horizontal` over its left and right and
    /// `bar_padding_vertical` over its top and bottom where stated.
    title_padding: ResolvedPadding,
    row_height: Option<Pixels>,
    /// `menu.min_row_height`, which `geometry::menu_item` sets as the row's
    /// least height where `row_height` is unstated.
    min_row_height: Option<Pixels>,
    /// Whether the text scale is above 1, where a fixed height becomes a
    /// minimum (geometry.rs, `height_rule`).
    text_scaled_up: bool,
    background: Hsla,
    separator: Hsla,
    frame: Hsla,
    frame_width: Pixels,
    frame_radius: Pixels,
    /// `popover.border.padding`, with `menu.panel_padding_horizontal` over
    /// its left and right and `panel_padding_vertical` over its top and
    /// bottom where stated: round the popup's rows.
    popup_padding: ResolvedPadding,
    /// `menu.panel_padding_horizontal` and `panel_padding_vertical` alone,
    /// each side `None` where unstated: round the Overlays page's rows
    /// ([`menu_rows`]), which pad nothing on an unstated side.
    panel_padding: ResolvedPadding,
    /// The gap between the popup's rows: none where
    /// `menu.panel_padding_vertical` is stated (Breeze's rows touch), else
    /// upstream's [`MENU_POPUP_ROW_GAP`].
    popup_row_gap: Option<Rems>,
    /// The popup's drop shadow ([`MENU_POPUP_SHADOW`]), none where
    /// `popover.border.shadow_enabled` does not hold.
    popup_shadow: Vec<gpui::BoxShadow>,
}

impl MenuLook {
    pub(crate) fn of(n: &Native<'_>) -> Self {
        let m = &n.resolved.menu;
        let p = &n.resolved.popover.border;
        let edge_width = px(n.resolved.defaults.border.line_width.max(0.0));
        let pad = m.border.padding;
        let (x, y) = (m.bar_padding_horizontal, m.bar_padding_vertical);
        let (panel_x, panel_y) = (m.panel_padding_horizontal, m.panel_padding_vertical);
        Self {
            item: geometry::menu_item(*n),
            text: info::stated(m.font.color),
            hover: info::stated(m.hover_background),
            hover_text: info::stated(m.hover_text_color),
            edge: m
                .hover_border_color
                .map(info::stated)
                .filter(|_| edge_width > px(0.0)),
            edge_width,
            active: m.active_background.map(info::stated),
            active_text: m.active_text_color.map(info::stated),
            bar_hover: info::stated(m.bar_hover_background.unwrap_or(m.hover_background)),
            bar_open: info::stated(
                m.bar_open_background
                    .or(m.bar_hover_background)
                    .unwrap_or(m.hover_background),
            ),
            bar_text: info::stated(m.bar_hover_text_color.unwrap_or(m.hover_text_color)),
            radius: px(m.border.corner_radius.max(0.0)),
            padding: pad,
            title_padding: ResolvedPadding {
                top: y.or(pad.top),
                right: x.or(pad.right),
                bottom: y.or(pad.bottom),
                left: x.or(pad.left),
            },
            row_height: m.row_height.map(px),
            min_row_height: m.min_row_height.map(px),
            text_scaled_up: n.accessibility.text_scale_factor() > 1.0,
            background: info::stated(m.background_color),
            separator: info::stated(m.separator_color),
            frame: info::stated(p.color),
            frame_width: px(p.line_width),
            frame_radius: px(p.corner_radius.max(0.0)),
            popup_padding: ResolvedPadding {
                top: panel_y.or(p.padding.top),
                right: panel_x.or(p.padding.right),
                bottom: panel_y.or(p.padding.bottom),
                left: panel_x.or(p.padding.left),
            },
            panel_padding: ResolvedPadding {
                top: panel_y,
                right: panel_x,
                bottom: panel_y,
                left: panel_x,
            },
            popup_row_gap: panel_y.is_none().then_some(MENU_POPUP_ROW_GAP),
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

    /// `title` as a menu title, in the font `geometry::menu_item` sets,
    /// padded by `title_padding`, and filled `bar_open` where `selected` (its
    /// menu is open, under the pointer too) and `bar_hover` under the
    /// pointer, lettered `bar_text` in both.
    pub(crate) fn title(&self, mut title: Stateful<Div>, selected: bool) -> Stateful<Div> {
        let (hover, open, text) = (self.bar_hover, self.bar_open, self.bar_text);
        title.text_style().refine(&self.item.text);
        padded(
            title,
            &self.title_padding,
            Some(MENU_TITLE_PADDING_X),
            Some(MENU_TITLE_PADDING_Y),
        )
        .rounded(self.radius)
        .map(|title| {
            if selected {
                title.bg(open).text_color(text)
            } else {
                title
                    .text_color(self.text)
                    .hover(move |style| style.bg(hover).text_color(text))
            }
        })
    }

    /// A popup row, refined by `geometry::menu_item` before the caller's own
    /// style (its label-to-shortcut gap replaces the builder's icon gap),
    /// with the fallbacks on what the theme leaves unstated: an unstated left
    /// or right is padded by `MENU_ROW_PADDING_X` ([`padded`], over the
    /// stated sides the builder set already), an unstated top or bottom by
    /// nothing.
    pub(crate) fn row(&self, lit: bool) -> Div {
        // The shortcut is the plain text of a Kbd without its appearance
        // (kbd.rs:236-238), which takes the row's font and line box.
        let row = padded(
            h_flex().refine_style(&self.item),
            &self.padding,
            Some(MENU_ROW_PADDING_X),
            None,
        )
        .rounded(self.radius)
        .relative()
        .group(MENU_ROW_GROUP);
        let no_height = self.row_height.is_none() && self.min_row_height.is_none();
        let row = match (no_height, self.padding.top, self.padding.bottom) {
            (true, None, None) if self.text_scaled_up => row.min_h(MENU_ROW_HEIGHT).h_auto(),
            (true, None, None) => row.h(MENU_ROW_HEIGHT),
            _ => row,
        };
        if lit {
            row.bg(self.hover).text_color(self.hover_text)
        } else {
            row.text_color(self.text)
        }
    }

    /// The outline of a row in `MENU_ROW_GROUP`, for the row's last child:
    /// a box laid over the whole row, its padding included, so nothing
    /// moves, rounded as the row's fill, `edge_width` wide in `edge`. Lit by
    /// the row's state where `lit` is `Some` (`MenuBar::popup`'s highlighted
    /// row), by the pointer on the row where it is `None` (`menu_rows`);
    /// `None` where the theme draws no outline. `none` is upstream's own
    /// `transparent` token, the box's colour where it is not lit.
    ///
    /// Its group-hover style, a no-op where `lit` is `Some`, gives the box a
    /// `Normal` hitbox, which leaves the row's own mouse handling alone, as
    /// `combo_surface`'s hover edge does (gpui-pre 0.3.12
    /// `elements/div.rs:2460-2486`, `should_insert_hitbox`); its id gives it
    /// the state a group-active style needs (`:3570-3579`): where `active`
    /// is stated the box is `none` while the row is pressed, as Breeze draws
    /// the pressed fill with no outline of another colour.
    pub(crate) fn edge(&self, lit: Option<bool>, none: Hsla) -> Option<Stateful<Div>> {
        let edge = self.edge?;
        let rest = if lit == Some(true) { edge } else { none };
        let hovered = if lit.is_none() { edge } else { rest };
        Some(
            div()
                .id("menu-row-edge")
                .absolute()
                .inset_0()
                .rounded(self.radius)
                .border(self.edge_width)
                .border_color(rest)
                .group_hover(MENU_ROW_GROUP, move |style| style.border_color(hovered))
                .when(self.active.is_some(), |edge| {
                    edge.group_active(MENU_ROW_GROUP, move |style| style.border_color(none))
                }),
        )
    }

    /// `row` while the mouse button is held on it: filled `active` and
    /// lettered `active_text` where each is stated, over its hover look; a
    /// row of `MENU_ROW_GROUP` whose [`edge`](Self::edge) hides while it is
    /// pressed where `active` is stated. One `active` style, as gpui keeps
    /// one (gpui-pre 0.3.12 `elements/div.rs`, `active`): where neither is
    /// stated the row is left as it is.
    pub(crate) fn pressed(&self, row: Stateful<Div>) -> Stateful<Div> {
        let (fill, text) = (self.active, self.active_text);
        if fill.is_none() && text.is_none() {
            return row;
        }
        row.active(move |style| {
            let style = match fill {
                Some(fill) => style.bg(fill),
                None => style,
            };
            match text {
                Some(text) => style.text_color(text),
                None => style,
            }
        })
    }
}

/// `el` padded by the stated sides of `padding`, and on a side it leaves
/// unstated by `x` (left and right) or `y` (top and bottom), where given: a
/// side neither states is left as it is.
fn padded<E: Styled, L: Into<DefiniteLength> + Copy>(
    el: E,
    padding: &ResolvedPadding,
    x: Option<L>,
    y: Option<L>,
) -> E {
    let side = |stated: Option<f32>, unstated: Option<L>| -> Option<DefiniteLength> {
        stated.map(|v| px(v).into()).or(unstated.map(Into::into))
    };
    let el = match side(padding.top, y) {
        Some(v) => el.pt(v),
        None => el,
    };
    let el = match side(padding.right, x) {
        Some(v) => el.pr(v),
        None => el,
    };
    let el = match side(padding.bottom, y) {
        Some(v) => el.pb(v),
        None => el,
    };
    match side(padding.left, x) {
        Some(v) => el.pl(v),
        None => el,
    }
}

/// Whether `padding` leaves a side unstated: the side a fallback, such as
/// `layout.container_margin`, pads ([`padded`]). Where it states all four,
/// the fallback pads nothing, and its info line is not recorded.
fn leaves_a_side_unstated(padding: &ResolvedPadding) -> bool {
    [padding.top, padding.right, padding.bottom, padding.left]
        .iter()
        .any(Option::is_none)
}

/// [`padded`] on the 0.8.0 model: `el` padded by the sides of `sides`, a
/// part's padding as `next::padding_of` reads it (top, right, bottom, left,
/// each `None` where unstated), and on a side it leaves unstated by `x`
/// (left and right) or `y` (top and bottom), where given.
fn next_padded<E: Styled, L: Into<DefiniteLength> + Copy>(
    el: E,
    sides: [Option<f32>; 4],
    x: Option<L>,
    y: Option<L>,
) -> E {
    let [top, right, bottom, left] = sides;
    let side = |stated: Option<f32>, unstated: Option<L>| -> Option<DefiniteLength> {
        stated.map(|v| px(v).into()).or(unstated.map(Into::into))
    };
    let el = match side(top, y) {
        Some(v) => el.pt(v),
        None => el,
    };
    let el = match side(right, x) {
        Some(v) => el.pr(v),
        None => el,
    };
    let el = match side(bottom, y) {
        Some(v) => el.pb(v),
        None => el,
    };
    match side(left, x) {
        Some(v) => el.pl(v),
        None => el,
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
    /// Where the title records the colour its text is laid out in, for the
    /// self-tests (`elements::label_probe`).
    #[cfg(test)]
    ui: Entity<InfoRegistry>,
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
            .child(self.name.clone());
        #[cfg(test)]
        let title = title.child(crate::elements::label_probe(
            &self.ui,
            format!("menu-title-{}", self.name),
        ));
        match self.look {
            Some(look) => look.title(title, self.selected),
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
        (Some(at), true) => at.saturating_add(1).checked_rem(actions.len()),
        (Some(at), false) => at
            .saturating_add(actions.len().saturating_sub(1))
            .checked_rem(actions.len()),
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
    /// an error naming the menus where none is titled so, and for every name
    /// where `menus_are_the_systems`: the macOS system menu bar holds the
    /// menus there, and the window draws none
    /// (`native_theme_showcase_support::cli::open_menu`).
    pub(crate) fn open_named(
        &mut self,
        name: &str,
        menus_are_the_systems: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), String> {
        let menus = app_menu_rows();
        let titles: Vec<&str> = menus.iter().map(|(title, _)| title.as_ref()).collect();
        let name =
            native_theme_showcase_support::cli::open_menu(name, &titles, menus_are_the_systems)?;
        if let Some(ix) = titles.iter().position(|title| title.to_lowercase() == name) {
            self.open(ix, window, cx);
        }
        Ok(())
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
                if let Some(ix) = open
                    .saturating_add(count.saturating_sub(1))
                    .checked_rem(count)
                {
                    self.open(ix, window, cx);
                }
            }
            "right" => {
                if let Some(ix) = open.saturating_add(1).checked_rem(count) {
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
                Some(MENU_POPUP_PADDING),
                Some(MENU_POPUP_PADDING),
            )
            .when_some(look.popup_row_gap, |body, gap| body.gap(gap)),
            None => v_flex().p(MENU_POPUP_PADDING).gap(MENU_POPUP_ROW_GAP),
        };
        let body = body
            .min_w(MENU_POPUP_MIN_WIDTH)
            .children(rows.iter().enumerate().map(|(ix, row)| {
                match row {
                    // Under a native theme the line is `widgets::Separator`'s
                    // in the menu's separator colour, one device pixel at
                    // least, where a box `separator.line_width` 0.5 thick
                    // (the macOS presets) rounds to nothing at scale 1
                    // (M9-165).
                    MenuRow::Separator => match look {
                        Some(l) => div()
                            .relative()
                            .w_full()
                            .child(widgets::Separator::horizontal().bg(l.separator))
                            .children(listed(ix).map(|id| elements::record(ui, id)))
                            .into_any_element(),
                        None => div()
                            .h(MENU_SEPARATOR)
                            .w_full()
                            .bg(cx.theme().border)
                            .children(listed(ix).map(|id| elements::record(ui, id)))
                            .into_any_element(),
                    },
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
                        let row = match look {
                            Some(look) => look.row(lit),
                            None => h_flex()
                                .px(MENU_ROW_PADDING_X)
                                .h(MENU_ROW_HEIGHT)
                                .text_sm()
                                .when(lit, |row| {
                                    row.bg(cx.theme().accent)
                                        .text_color(cx.theme().accent_foreground)
                                }),
                        };
                        let row = row.id(("menu-row", ix));
                        let row = match look {
                            Some(look) => look.pressed(row),
                            None => row,
                        };
                        let label = div()
                            .debug_selector({
                                let name = name.clone();
                                move || format!("menu-row-label-{name}")
                            })
                            .child(name.clone());
                        #[cfg(test)]
                        let label = label.child(crate::elements::label_probe(
                            ui,
                            format!("menu-row-label-{name}"),
                        ));
                        // The shortcut is plain text in the row's own text
                        // style, which a probe beside it records.
                        #[cfg(test)]
                        let row = row.when(shortcut.is_some(), |row| {
                            row.child(crate::elements::label_probe(
                                ui,
                                format!("menu-row-shortcut-{name}"),
                            ))
                        });
                        let row = row
                            .w_full()
                            .gap(MENU_SHORTCUT_GAP)
                            .justify_between()
                            .items_center()
                            .cursor_default()
                            .debug_selector({
                                let name = name.clone();
                                move || format!("menu-row-{name}")
                            })
                            .child(label)
                            .children(shortcut)
                            .children(listed(ix).map(|id| elements::record(ui, id)))
                            .children(
                                look.and_then(|look| look.edge(Some(lit), cx.theme().transparent)),
                            )
                            .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                                if *hovered && this.highlighted != Some(ix) {
                                    this.highlighted = Some(ix);
                                    cx.notify();
                                }
                            }))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.run(dispatched.as_ref(), window, cx);
                            }));
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
        let native = old_native(cx);
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
                        #[cfg(test)]
                        ui: ui.clone(),
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
    let native = old_native(cx);
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
/// as a KDE application places its menus, filled with
/// `toolbar.background_color` where a native theme is installed (the model's
/// `MenuTheme`: a menu bar takes the toolbar's fill). The model states no
/// menu-bar inset, so its sides borrow `container_margin`, the installed
/// layout's `geometry::container_margin`, and where that is unstated take
/// [`MENU_BAR_PADDING`]; the menu titles set its height.
pub(crate) fn menu_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    container_margin: Option<Pixels>,
    menus: &Entity<MenuBar>,
) -> Stateful<Div> {
    let native = old_native(cx);
    let resolved = native.as_ref().map(|n| n.resolved);
    // No geometry line: the model states no menu-bar inset, and the row only
    // borrows container_margin, which its info says in its own words.
    let row_info = info::menu_bar(cx.theme(), resolved, container_margin, MENU_BAR_PADDING);
    h_flex()
        .px(container_margin.unwrap_or(MENU_BAR_PADDING))
        .when_some(resolved, |row, r| {
            row.bg(info::stated(r.toolbar.background_color))
        })
        .child(app_menus(ui, cx, info::MenuHost::Row, menus))
        .info(ui, "chrome-menu-bar", row_info)
}

/// A `TitleBar` sample, refined by `next::geometry::title_bar` and reading
/// `label`:
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
    let bar = next_native_info(
        TitleBar::new(),
        cx,
        next::geometry::title_bar,
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
    if leaves_a_side_unstated(&stated) && container_margin.is_some() {
        row_info = row_info.geometry("container_margin");
    }
    if item_gap.is_none() && widget_gap.is_some() {
        row_info = row_info.geometry("widget_gap");
    }
    let row = h_flex()
        .p(container_margin.unwrap_or(TOOLBAR_PADDING))
        .gap(widget_gap.unwrap_or(TOOLBAR_GAP));
    let row = native_info(row, cx, geometry::toolbar, "toolbar", &mut row_info).children(items);
    row.info(ui, "chrome-toolbar", row_info)
}

/// The window's `StatusBar` (spec §2.7, §3.2), refined by
/// `geometry::status_bar`: on the left `toggle`, then `environment_text`
/// drawing `environment`; on the right `shown`, the title of what the
/// inspector shows, where it shows one.
pub(crate) fn status_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    toggle: impl IntoElement,
    environment: &str,
    environment_text: StyledText,
    shown: Option<SharedString>,
    widget_gap: Option<Pixels>,
) -> Stateful<Div> {
    // What `native_info` applies the builder under.
    let styled = old_native(cx).is_some();
    let mut bar_info = info::status_bar(cx.theme(), styled, environment);
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
    // The toggle and the environment as one item, `layout.widget_gap`
    // apart: a region spaces its items by a `gap_2` of its own (status_bar.rs,
    // `RenderOnce for StatusBar`), which no style reaches. Plain text, not
    // Labels, as in the title bar: a Label would paint foreground over the
    // colour `geometry::status_bar` gives the bar.
    //
    // The item opens the middle region, before an empty box that fills the
    // rest of it, so its edges are where the two ends stop. Upstream's left
    // and right regions both start at their content's width and both shrink,
    // `overflow_hidden` (status_bar.rs:84) with gpui's flex-shrink 1
    // (gpui-pre style.rs:802), each by its share of what the bar lacks, so an
    // environment wider than the bar would cut the shown title short too; the
    // middle region, `flex_1` (status_bar.rs:100), starts at none and takes
    // what the right one leaves it. There the environment shrinks alone: it
    // is cut short with an ellipsis (`truncate`), the toggle keeps its width,
    // and the bar's info carries the whole text. With no left region the
    // middle one starts at the bar's left padding, where the left one did.
    .child(
        with_gap(h_flex(), widget_gap)
            .min_w_0()
            .items_center()
            .child(div().flex_shrink_0().child(toggle))
            .child(
                div()
                    .relative()
                    .min_w_0()
                    .truncate()
                    .debug_selector(|| STATUS_ENVIRONMENT.into())
                    .child(environment_text)
                    .child(elements::record(ui, "chrome.status_bar.environment")),
            ),
    )
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
    let mut combobox_info = info::preset_combobox(cx.theme(), next_resolved(cx));
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
    let mut select_info = info::color_mode_select(cx.theme(), next_resolved(cx));
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
/// wide as the theme settings it is in; where the system icon theme cannot be
/// detected, `unavailable` gives the reason, which the Select shows in a
/// tooltip built as [`built_tooltip_button`]'s is, and its info in a `system`
/// line: the `system` row's label is short, `system (unavailable)`
/// (`IconSetChoice`'s `Display`).
pub(crate) fn icon_set_select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
    unavailable: Option<String>,
) -> Stateful<Div> {
    let mut select_info = info::icon_set_select(cx.theme(), next_resolved(cx));
    if let Some(reason) = &unavailable {
        select_info = select_info.instance("system", reason.clone());
    }
    let select = native_info(
        Select::new(state),
        cx,
        geometry::select,
        "select",
        &mut select_info,
    );
    let select = refined(select, combo_fill(cx).as_ref()).w_full();
    let select =
        combo_surface(cx, select)
            .w_full()
            .info(ui, "chrome-settings-icon-theme", select_info);
    match unavailable {
        Some(reason) => settings_tooltip(cx, select, reason.into()),
        None => select,
    }
}

/// `control` of the theme settings with a tooltip reading `text`, built as
/// [`built_tooltip_button`]'s is.
fn settings_tooltip(cx: &App, control: Stateful<Div>, text: SharedString) -> Stateful<Div> {
    let style = native_geometry(cx, geometry::tooltip);
    let content = native_geometry(cx, geometry::tooltip_content);
    control
        .tooltip(move |window, cx| {
            let (text, content) = (text.clone(), content.clone());
            refined(
                Tooltip::element(move |_window, _cx| {
                    refined(div().child(text.clone()), content.as_ref())
                }),
                style.as_ref(),
            )
            .build(window, cx)
        })
        .when_some(tooltip_delay(cx), |control, delay| {
            control.tooltip_show_delay(delay)
        })
}

/// What the palette Select's tooltip says: what is drawn from which model
/// under a palette, which has no 0.7.2 equivalent.
pub(crate) const PALETTE_TOOLTIP: &str = "A palette recolours the preset (0.8.0 model): \
     gpui-component's theme colours and the connector's own widgets are drawn from the 0.8.0 \
     theme with the palette. Still read from the 0.7.2 preset, which has no palettes, until \
     their pages move to the 0.8.0 model: the button variant and rest fill the showcase gives \
     the Basic page's default-button sample, the checked track of the Preferences Switches \
     built without the widgets feature, and the showcase's own \
     refinements -- its colours, geometry, menus and the side panel's padding and label font.";

/// The palette Select over `state`, refined by `geometry::select`, as wide
/// as the theme settings it is in: `none`, the theme's own colours, and the
/// bundled palettes (`support::palette_rows`); the choice installs the theme
/// again with the palette laid over it (`Showcase::choose_palette`). Its
/// tooltip says what the pages still on the 0.7.2 model show under a
/// palette ([`PALETTE_TOOLTIP`]); its info says where High contrast set the
/// palette chosen aside (`support::palette_set_aside`).
pub(crate) fn palette_select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
) -> Stateful<Div> {
    let mut select_info = info::palette_select(cx.theme(), next_resolved(cx));
    if let Some(palette) = crate::support::palette_set_aside(cx) {
        select_info =
            select_info.instance("set aside", crate::support::palette_set_aside_text(palette));
    }
    let select = native_info(
        Select::new(state),
        cx,
        geometry::select,
        "select",
        &mut select_info,
    );
    let select = refined(select, combo_fill(cx).as_ref()).w_full();
    let select =
        combo_surface(cx, select)
            .w_full()
            .info(ui, "chrome-settings-palette", select_info);
    settings_tooltip(cx, select, PALETTE_TOOLTIP.into())
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

/// A flat `Button` for the toolbar, built with `variants::toolbar_button` and
/// refined by `geometry::toolbar_button_rest` after its box -- dispatching
/// its action, its icon of the chosen icon theme at
/// `geometry::icon_size_toolbar` and its tooltip showing the action's key
/// binding. Where the icon theme has no icon for it, it
/// shows its tooltip's text instead, never another icon theme's icon.
///
/// The icon is the Button's child, not its `icon`: `Button::icon` resizes
/// whatever it is given to a size derived from the Button's own
/// (button/button.rs:638-641, 789-794), and the size an `Icon` is given last
/// wins over its style (icon.rs:225-231).
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
    let delay = tooltip_delay(cx);
    let native = old_native(cx);
    let mut button_info = info::toolbar_button(
        cx.theme(),
        about,
        &drawn,
        set,
        styled,
        delay,
        native.as_ref().map(|n| n.resolved),
    );
    if styled {
        button_info = button_info.geometry("tooltip");
    }
    let icon = chrome_glyph(&drawn, &icon)
        .map(|icon| native_sized_glyph(cx, icon, geometry::icon_size_toolbar));
    if icon.is_some() && native_value(cx, geometry::icon_size_toolbar).is_some() {
        button_info = button_info.geometry("icon_size_toolbar");
    }
    let dispatched = action.boxed_clone();
    let button = native_info(
        refined(
            Button::new(id).custom(toolbar_variant(cx)),
            tool_button_box(cx).as_ref(),
        ),
        cx,
        geometry::toolbar_button_rest,
        "toolbar_button_rest",
        &mut button_info,
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
    .when_some(delay, |button, delay| button.tooltip_show_delay(delay))
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
/// 0.9.1's own, refined by `geometry::toolbar` alone -- the height, fill,
/// font, gap, padding sides and bottom edge the theme states, and no
/// fallback, since the section shows the widget and not the chrome's row -- holding
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
    let native = old_native(cx);
    let mut toolbar_info = info::buttons::toolbar(item_gap, native.as_ref().map(|n| n.resolved));
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

/// One of the component `Toolbar`'s Buttons: a flat `Button` built as
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
    let native = old_native(cx);
    let mut button_info =
        info::buttons::toolbar_item(cx.theme(), &drawn, set, native.as_ref().map(|n| n.resolved));
    let icon = chrome_glyph(&drawn, &icon)
        .map(|icon| native_sized_glyph(cx, icon, geometry::icon_size_toolbar));
    if icon.is_some() && native_value(cx, geometry::icon_size_toolbar).is_some() {
        button_info = button_info.geometry("icon_size_toolbar");
    }
    let button = native_info(
        refined(
            Button::new(id).custom(toolbar_variant(cx)),
            tool_button_box(cx).as_ref(),
        ),
        cx,
        geometry::toolbar_button_rest,
        "toolbar_button_rest",
        &mut button_info,
    )
    .map(|button| match icon {
        Some(icon) => button.accessibility_label(label).child(icon),
        None => labelled(ui, cx, button, id, label),
    });
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(button, ui, id, button_info).debug_selector(move || id.into())
}

/// The installed theme's `tooltip.show_delay_ms`, for GPUI's own
/// `.tooltip()` on an element, which waits GPUI's 500 ms unless the element
/// is given another (gpui-pre `elements/div.rs:53, 1755`); `None` where the
/// theme states none or no native theme is installed.
pub(crate) fn tooltip_delay(cx: &App) -> Option<Duration> {
    native_value(cx, |n| native_theme_gpui::tooltip_show_delay(n.resolved)).flatten()
}

/// A tooltip reading `text` and the key binding of `action`, built by the
/// application and refined by `geometry::tooltip`, for the element that
/// wraps a chrome Button; and whether a native theme refined it.
///
/// Not `Button::tooltip_with_action`: the Button builds that Tooltip itself
/// as it renders (button/button.rs, `RenderOnce for Button`), so the
/// platform's tooltip fill, edge, padding, radius and text colour would not
/// reach it. It is `Tooltip::new(text)` with `tooltip.max_width` on the
/// bubble, which keeps the bubble, key binding included, within the
/// platform's outer width and wraps the text beside the key. The text as an
/// element refined by `geometry::tooltip_content` would not: that width
/// counts the text alone, and with a key binding the bubble grows past
/// `tooltip.max_width` by the key and the gap before it; without one the two
/// are the same bubble (`tests/seams.rs`, `geometry::tooltip`).
fn action_tooltip(
    cx: &App,
    text: &'static str,
    action: &dyn Action,
) -> (
    impl Fn(&mut Window, &mut App) -> gpui::AnyView + 'static,
    bool,
) {
    let style = native_geometry(cx, geometry::tooltip);
    let max_width = native_value(cx, |n| px(n.resolved.tooltip.max_width));
    let styled = style.is_some();
    let action = action.boxed_clone();
    let build = move |window: &mut Window, cx: &mut App| {
        refined(
            Tooltip::new(text).action(action.as_ref(), None),
            style.as_ref(),
        )
        .when_some(max_width, |tooltip, max| tooltip.max_w(max))
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
/// `variants::ghost_button`, not the toolbar's variant -- dispatching its action,
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
    let (tip, styled) = action_tooltip(cx, spec.tooltip, spec.action);
    let delay = tooltip_delay(cx);
    let native = old_native(cx);
    let mut button_info = info::panel_toggle(
        cx.theme(),
        &spec,
        styled,
        delay,
        native.as_ref().map(|n| &n.resolved.button),
    );
    let PanelToggle {
        button_id: id,
        icon,
        drawn,
        tooltip,
        action,
        open,
        ..
    } = spec;
    if styled {
        button_info = button_info.geometry("tooltip");
    }
    let icon = chrome_glyph(&drawn, &icon)
        .map(|icon| native_sized_glyph(cx, icon, geometry::icon_size_small));
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
    .when_some(delay, |button, delay| button.tooltip_show_delay(delay))
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
/// selector: plain text in the installed 0.8.0 theme's
/// `sidebar.item_label(NONE)` ([`next_text_in`]) where it gives a font gpui
/// can letter it in, else `text_sm`, in the colour the side panel sets. Not
/// a `Label`: `Label::render` paints `foreground` on its own element
/// (label.rs:213), over the sidebar's text colour.
fn sidebar_label(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: &'static str,
) -> Stateful<Div> {
    let native = next_resolved(cx);
    let text_div = native
        .and_then(|r| {
            next_text_in(
                &r.typography,
                &r.sidebar.item_label(WidgetState::NONE),
                "sidebar.item_label(NONE)",
                text,
            )
        })
        .unwrap_or_else(|| div().text_sm().child(text));
    text_div
        .info(ui, id, info::sidebar_label(cx.theme(), native))
        .self_start()
        .debug_selector(move || id.into())
}

/// The side panel (spec S2): `settings` over `separator`, over `inspector`,
/// which fills the rest of the panel's height and scrolls its own content;
/// filled with the model's sidebar colour and lettered in its font's colour.
/// The three are one content, scrolled on `scroll` where it is taller than
/// the panel ([`crate::support::gutter_scroll`], `relayout` its frame's
/// relayout): the inspector then keeps its own height, its TabBar's and its
/// info's down to the first row (`inspector::PanelLook::body_min_height`),
/// so every control stays reachable at every text scale in the fixed
/// window, and the panel never runs past its bounds.
/// Where the theme states any side of `sidebar.border.padding`
/// ([`sidebar_padding`]), the whole panel is padded by it, a side it leaves
/// unstated by `container_margin`, and the content adds none; where it
/// states no side, `container_margin` pads the settings
/// ([`side_panel_content_margin`]). Its `container_margin` line is recorded
/// only where the margin pads something.
///
/// Plain elements, not upstream's `Sidebar`: a Sidebar's children must
/// implement `SidebarItem` (sidebar/mod.rs:211), and neither the settings
/// nor the inspector is an item.
#[allow(clippy::too_many_arguments)]
pub(crate) fn side_panel(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    container_margin: Option<Pixels>,
    settings: impl IntoElement,
    separator: impl IntoElement,
    inspector: impl IntoElement,
    scroll: &gpui::ScrollHandle,
    relayout: impl Fn(&mut App) + 'static,
) -> Stateful<Div> {
    let own = sidebar_padding(cx);
    let mut panel_info = info::side_panel(cx.theme(), container_margin, own);
    if container_margin.is_some() && own.as_ref().is_none_or(leaves_a_side_unstated) {
        panel_info = panel_info.geometry("container_margin");
    }
    // The model's sidebar: `sidebar.background_color` and `sidebar.font`'s
    // colour, which the connector installs as the `sidebar` and
    // `sidebar_foreground` tokens.
    v_flex()
        .size_full()
        .bg(cx.theme().sidebar)
        .text_color(cx.theme().sidebar_foreground)
        .when_some(own, |panel, p| {
            padded(panel, &p, container_margin, container_margin)
        })
        .child(
            crate::support::gutter_scroll(
                CHROME_SIDE_PANEL_SCROLL,
                scroll,
                v_flex()
                    .w_full()
                    .debug_selector(|| CHROME_SIDE_PANEL_CONTENT.into())
                    .child(
                        with_padding(
                            div().w_full(),
                            side_panel_content_margin(cx, container_margin),
                        )
                        .child(settings),
                    )
                    .child(separator)
                    // Grown from nothing over the room the settings leave, so
                    // it is never the inspector's own content that makes the
                    // panel's content taller than the panel.
                    .child(
                        div()
                            .w_full()
                            .flex_grow_1()
                            .flex_basis(px(0.))
                            .child(inspector),
                    ),
                relayout,
                cx,
            )
            .w_full()
            .flex_1()
            .min_h_0(),
        )
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
            // `SidebarMenuItem::icon` takes an `Icon` alone (sidebar/menu.rs).
            drawn: drawn.svg_only(),
            set: set.clone(),
        });
    // Plain text, not a Label: `Label::render` sets `foreground` on its own
    // element (label.rs:213), which would hide the sidebar_foreground the
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
/// expanded item's `h_7` row (sidebar/menu.rs:329) nor the 48px rail
/// (sidebar/mod.rs:28). `SidebarMenuItem` keeps the icon it is given
/// (sidebar/menu.rs:321). The size goes on the Icon's style, which upstream
/// lays out as it would the same size given through `with_size`
/// (icon.rs:215-226), so a test can read it off the Icon this returns.
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
/// 121-122, tab/tab_bar.rs:412-421), leaving the inset to its container, so
/// the bar takes `container_margin`, the installed layout's
/// `geometry::container_margin`, on its left and right, as the side panel's
/// settings and the inspector's content do. Its bottom rule is drawn by an
/// absolute child at the bar's full size (tab_bar.rs:639-649), so it still
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
    let menu_icon = menu_icon.and_then(SampleIcon::glyph);
    #[cfg(not(feature = "widgets"))]
    let _ = &menu_icon;
    let native = old_native(cx);
    let resolved = native.as_ref().map(|n| n.resolved);
    let (id, info_id, menu, bar_info) = match kind {
        TabBarKind::Inspector => (
            "inspector-tabs",
            "chrome-inspector-tabs",
            false,
            info::inspector_tab_bar(cx.theme(), resolved),
        ),
        TabBarKind::Pages => (
            "page-tabs",
            "chrome-page-tabs",
            true,
            info::page_tab_bar(cx.theme(), resolved),
        ),
    };
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
    let mut bar_info = info::tab_font(bar_info, resolved)
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
                        .child(text_in(n, &t.font, label))
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
    menu_icon: Option<Glyph>,
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
        .instance("padding", info::tab_bar_padding(container_margin))
        .instance("click", "selects the tab; the showcase keeps the state. A tab the bar clips is scrolled just into view (widgets::TabBar, ScrollHandle::scroll_to_item)");
    if container_margin.is_some() {
        bar_info = bar_info.geometry("container_margin");
    }
    if menu && menu_icon.is_some() {
        bar_info = bar_info.geometry("icon_size_small");
    }
    let labels: Vec<&'static str> = tabs.iter().map(|&(label, _)| label).collect();
    let on_click: TabClick = Rc::new(on_click);
    let on_tab = on_click.clone();
    let tool_box = Some(tool_box_of(*n));
    let ghost = variants::ghost_button(cx);
    let menu = menu.then(|| {
        // The chosen set's ChevronDown at `geometry::icon_size_small`, where
        // the set has one, in place of upstream's caret, which draws
        // gpui-component's own glyph at a size of its own (button/button.rs,
        // `Caret`); upstream's caret where the set has none, the button
        // otherwise empty. The caret is the Button's child, not its
        // `dropdown_caret`: a Button with neither a label nor a child is laid
        // out as an icon Button of a fixed side (button/button.rs,
        // `RenderOnce for Button`, `size_5` at XSmall), whose stated side
        // padding (`tool_box`; 11 px a side under windows-11) leaves the
        // caret no room. It is upstream's caret in upstream's colour, the
        // foreground at 75% (the same, `dropdown_caret`).
        let caret =
            Caret::new(Size::XSmall).text_color(cx.theme().secondary_foreground.opacity(0.75));
        refined(
            Button::new("page-tabs-menu").xsmall().custom(ghost),
            tool_box.as_ref(),
        )
        .accessibility_label("Pages")
        .map(|button| match menu_icon {
            Some(icon) => button.child(listed_icon(
                ui,
                native_sized_glyph(cx, icon, geometry::icon_size_small),
                Some("chrome.page_tabs.menu.icon"),
            )),
            None => button.child(caret),
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
    let bar = native_tab_strip(n, id)
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
        .info(ui, info_id, bar_info);
    if kind != TabBarKind::Pages {
        return bar;
    }
    // The page row's rule, `separator.line_width` thick in
    // `separator.line_color`, under the strip, laid out after it: the rule
    // takes the room it is drawn in, `widgets::Separator`'s line, one device
    // pixel at least, so it covers none of the strip's last row of pixels.
    div()
        .id("page-tabs-row")
        .w_full()
        .flex()
        .flex_col()
        .child(bar.flex_none())
        .child(
            div()
                .relative()
                .w_full()
                .child(widgets::Separator::horizontal())
                .child(elements::record(ui, "chrome.page_tabs.rule")),
        )
}

/// The variant a theme-drawn tab row's info names.
#[cfg(feature = "widgets")]
pub(crate) const NATIVE_TAB_BAR: &str = "widgets::TabBar";
/// The same, for the page row with its menu.
#[cfg(feature = "widgets")]
pub(crate) const NATIVE_TAB_BAR_MENU: &str = "widgets::TabBar, menu";

/// `text` in `font` as the connector's `with_text` sets a font (geometry.rs):
/// its family through `native_theme_gpui::font_family`, its size scaled by
/// the text-scaling factor, its weight and style, one [`line_height_of`]
/// tall. A widget's label the showcase draws as the widget's child.
pub(crate) fn text_in(n: &Native<'_>, font: &ResolvedFontSpec, text: &'static str) -> Div {
    let mut label = div()
        .font_family(native_theme_gpui::font_family(&font.family))
        .text_size(text_size_of(n, font))
        .line_height(line_height_of(n, font))
        .font_weight(FontWeight(f32::from(font.weight)))
        .child(text);
    // The connector's own mapping, `font_style` (native-theme-gpui
    // src/lib.rs:834-840), is private to its crate.
    label.text_style().font_style = Some(match font.style {
        FontStyle::Normal => gpui::FontStyle::Normal,
        FontStyle::Italic => gpui::FontStyle::Italic,
        FontStyle::Oblique => gpui::FontStyle::Oblique,
    });
    label
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
    px(f32::from(text_size_of(n, font)) * n.resolved.defaults.line_height)
}

/// A `TabBar` of upstream's default variant, `TabVariant::Tab`, over
/// `labels`, `selected` the one shown; a click hands `on_click` the index of
/// the tab clicked.
///
/// What the theme states reaches it per call where upstream leaves a seam:
/// each label is a child in `tab.font` ([`text_in`]) over the `text_sm`
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
    let native = old_native(cx);
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
                .child(text_in(n, &t.font, label))
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

/// The height of the panel under the Folder TabBar. The model states none,
/// so this is the showcase's own: room for its one line, in rems, so it
/// grows with the text.
const FOLDER_PANEL_HEIGHT: Rems = rems(4.);

/// A Folder tab's height at the default Size, upstream's px literal, which
/// it keeps private (tab/tab.rs, `TabVariant::height`, the arm every
/// variant but Underline takes): its corner radius is capped at a third of
/// it (tab/tab.rs, `FolderTabMetrics::new`).
pub(crate) const FOLDER_TAB_HEIGHT: Pixels = px(32.);

/// A `TabBar` of upstream's `TabVariant::Folder` over `labels`, `selected`
/// the one shown and `disabled` the one that takes no click, above a panel
/// filled with `tab_active`, the colour the selected tab's shape is painted
/// in (tab/tab.rs, `folder_tab_shape`), so the tab joins it as the variant
/// is meant to sit; a click hands `on_click` the index of the tab clicked.
/// Each tab is tagged `{id}-tab-{ix}` for the tests.
///
/// What the installed 0.8.0 theme states reaches it per call: each label a
/// child in `tab.label(NONE)` ([`next_text_in`]) where the theme gives a
/// font gpui can letter it in, else upstream's own label; each tab at least
/// `tab.tab(NONE)`'s stated least width and height. Upstream's Folder has no
/// seam for the theme's tab shape; `widgets::TabBar` draws that.
pub(crate) fn folder_tab_row(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
    selected: usize,
    disabled: usize,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let native = next_resolved(cx);
    let row_info =
        info::layout::next_folder_tab_row(cx.theme(), labels, selected, disabled, native);
    let tabs = labels.iter().enumerate().map(|(ix, &label)| {
        let tab = match native {
            Some(r) => {
                let [min_w, min_h] = r.tab.tab(WidgetState::NONE).min_size;
                let tab = match next_text_in(
                    &r.typography,
                    &r.tab.label(WidgetState::NONE),
                    "tab.label(NONE)",
                    label,
                ) {
                    Some(text) => Tab::new().child(text),
                    None => Tab::new().label(label),
                };
                tab.when_some(min_w, |tab, w| tab.min_w(px(w)))
                    .when_some(min_h, |tab, h| tab.min_h(px(h)))
            }
            None => Tab::new().label(label),
        };
        tab.disabled(ix == disabled)
            .debug_selector(move || format!("{id}-tab-{ix}"))
    });
    let shown = labels.get(selected).copied().unwrap_or("none");
    v_flex()
        .child(
            TabBar::new(id)
                .folder()
                .children(tabs)
                .selected_index(selected)
                .on_click(on_click),
        )
        .child(
            h_flex()
                .h(FOLDER_PANEL_HEIGHT)
                .justify_center()
                .bg(cx.theme().tab_active)
                .text_color(cx.theme().tab_active_foreground)
                .child(format!("{shown}: the selected tab's panel")),
        )
        .info(ui, id, row_info)
        .debug_selector(move || id.into())
}

/// A segment's side padding where `segmented_control.border` states none:
/// upstream's own for a segment at the default Size (tab/tab.rs:113-118,
/// `TabVariant::inner_paddings`), which a `TabBar::segmented` would give it.
pub(crate) const SEGMENT_PADDING: Pixels = px(12.);

/// The width [`segmented`] lays its control over `labels` out at under `n`:
/// each label as gpui's text system shapes it in `segmented_control.font`
/// (its family, weight, style and scaled size, as [`text_in`] sets them),
/// its width rounded up, as gpui's text element rounds a line's width up
/// (gpui-pre 0.3.12 `src/elements/text.rs:772`), inside the segment's stated
/// side padding ([`SEGMENT_PADDING`] on a side left unstated), the
/// `separator_width` line before each segment after the first, and the
/// frame's `border.line_width` at both ends. The Basic page's
/// column count holds it (M9-166): the segments stay on one row.
pub(crate) fn segmented_width(window: &Window, n: &Native<'_>, labels: &[&str]) -> f32 {
    let s = &n.resolved.segmented_control;
    let b = &s.border;
    let font = gpui::Font {
        weight: FontWeight(f32::from(s.font.weight)),
        style: match s.font.style {
            FontStyle::Normal => gpui::FontStyle::Normal,
            FontStyle::Italic => gpui::FontStyle::Italic,
            FontStyle::Oblique => gpui::FontStyle::Oblique,
        },
        ..gpui::font(native_theme_gpui::font_family(&s.font.family))
    };
    let size = text_size_of(n, &s.font);
    let side = |stated: Option<f32>| stated.unwrap_or(f32::from(SEGMENT_PADDING));
    let padding = side(b.padding.left) + side(b.padding.right);
    #[allow(
        clippy::disallowed_methods,
        reason = "shape_line's panic is a debug_assert! on a newline (gpui-pre 0.3.12 \
                  src/text_system.rs:679-682): the one non-test caller passes BASIC_SEGMENTS \
                  (pages/basic.rs), constants without one, and the showcase tests call it in \
                  debug builds, where a newline would fire the assert; its one run is the \
                  label's whole length, so no run splits a character or passes the text"
    )]
    let shaped = |label: &str| {
        let run = gpui::TextRun {
            len: label.len(),
            font: font.clone(),
            color: Hsla::default(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        f32::from(
            window
                .text_system()
                .shape_line(SharedString::from(label.to_owned()), size, &[run], None)
                .width
                .ceil(),
        )
    };
    let segments: f32 = labels.iter().map(|label| shaped(label) + padding).sum();
    let lines = labels.len().saturating_sub(1) as f32 * s.separator_width;
    segments + lines + b.line_width + b.line_width
}

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
    let native = old_native(cx);
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
        padded(div().id((id, ix)), &b.padding, Some(SEGMENT_PADDING), None)
            .flex()
            .flex_none()
            .items_center()
            .min_h(px(s.segment_height))
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
            .child(text_in(&n, &s.font, label))
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
                            .left(px(-s.separator_width))
                            .w(px(s.separator_width))
                            .child(elements::record(ui, divider))
                    }))
            }))
            .on_click(move |_, window, cx| on_click(&ix, window, cx))
    });
    // The segments stay on one row at their own widths, as the platforms'
    // segmented controls do; the Basic page's column count gives the control
    // a column it overruns by at most the section gap less
    // `layout.widget_gap` (`pages::basic::basic_column_count`, M9-166).
    h_flex()
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

/// gpui-base's `HANDLE_SIZE` (resizable/resize_handle.rs:13): the width of a
/// resize handle's line, between its paddings; `pub(crate)` upstream as well.
pub(crate) const HANDLE_SIZE: Pixels = px(1.);

/// The room the side panel keeps clear at its right edge for the splitter's
/// line: `splitter.divider_width` beyond the `HANDLE_SIZE` the handle's own
/// line takes, where the theme states it wider. The handle is absolutely
/// placed (resizable/resize_handle.rs, `ResizeHandle`), so no layout makes
/// room for a wider line; [`resize_handles`] paints it from the boundary
/// back over this strip, so it takes neither panel's content or margin.
pub(crate) fn splitter_reserve(cx: &App) -> Option<Pixels> {
    let width = native_value(cx, |n| px(n.resolved.splitter.divider_width))?;
    (width > HANDLE_SIZE).then(|| px(f32::from(width) - f32::from(HANDLE_SIZE)))
}

/// What each handle of a horizontal resizable group paints: upstream's line
/// in upstream's colour, with an info target over the handle's whole hit
/// area.
///
/// That area is the line, which starts at the boundary, and `HANDLE_PADDING`
/// on either side of it: the handle is `straddle_extent` wide, its line padded
/// by `HANDLE_PADDING` each side, absolutely placed `HANDLE_PADDING` left of
/// the boundary (gpui-base 0.9.1 resizable/resize_handle.rs:283, :298,
/// :340-346). gpui lays boxes out border-box (taffy 0.15.1 style/mod.rs:882;
/// gpui-pre's taffy.rs never sets it), so taffy's clamp of an absolute box's
/// min size to its padding (compute/oof.rs:515-516) does not bite: 9px over 8.
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
            // renders (resizable/panel.rs:396-401), and each handle calls
            // this once as it lays itself out (resizable/resize_handle.rs:
            // 238-247). A panel given `visible(false)` returns before it
            // builds its handle (resizable/panel.rs:321) and would shift the
            // count, so an absent panel is left out of the group instead.
            let ix = next.get();
            next.set(ix.saturating_add(1));
            if handle.axis() != Axis::Horizontal {
                return None;
            }
            let (id, between) = handles.get(ix).copied()?;
            let base = gpui_base::Theme::global(cx);
            // gpui-base's `handle_color` (resizable/resize_handle.rs:508-517).
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
            // holds its line in HANDLE_SIZE; `splitter.hover_color`
            // under the pointer, through the group upstream names the
            // handle (resizable/resize_handle.rs, `ResizeHandle`).
            let native = old_native(cx);
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
                .right(-HANDLE_PADDING)
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

/// The entrance `geometry::dialog_entrance` gives the installed theme's
/// `dialog.entrance`, for every Dialog and AlertDialog the showcase opens;
/// `None` where the theme states none, and the dialog keeps upstream's.
/// Counted in the self-tests (`THEMED_ENTRANCES`).
fn themed_entrance(cx: &App) -> Option<DialogEntrance> {
    #[cfg(test)]
    THEMED_ENTRANCES.with(|reads| reads.set(reads.get().saturating_add(1)));
    native_value(cx, geometry::dialog_entrance).flatten()
}

#[cfg(test)]
thread_local! {
    /// How often a dialog's build read the theme's entrance
    /// ([`themed_entrance`]) on this thread; a test reads how much a build
    /// adds to it.
    pub(crate) static THEMED_ENTRANCES: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

/// `dialog` refined by `geometry::dialog`, capped at
/// `geometry::dialog_max_width` and entering as `entrance`, the theme's
/// ([`themed_entrance`]), each recorded in `info` where it applies.
fn dialog_frame(
    dialog: Dialog,
    entrance: Option<DialogEntrance>,
    cx: &App,
    info: &mut WidgetInfo,
) -> Dialog {
    let dialog = native_info(dialog, cx, geometry::dialog, "dialog", info);
    let dialog = match entrance {
        Some(entrance) => {
            *info = std::mem::take(info).geometry("dialog_entrance");
            dialog.entrance(entrance)
        }
        None => dialog,
    };
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
    let entrance = themed_entrance(cx);
    let mut dialog_info = info::palette_dialog(
        cx.theme(),
        cx.reduce_motion(),
        entrance.is_some(),
        old_native(cx).is_some(),
    );
    let dialog = dialog_frame(dialog, entrance, cx, &mut dialog_info);
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
    let entrance = themed_entrance(cx);
    let mut dialog_info = info::about_dialog(
        cx.theme(),
        gap.is_some(),
        cx.reduce_motion(),
        entrance.is_some(),
        old_native(cx).is_some(),
    );
    let dialog = dialog_frame(dialog, entrance, cx, &mut dialog_info);
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
        // Refined as `demo::link` refines every Link: `geometry::link`, and
        // one line of `link.font` tall.
        let underline = native_value(cx, |n| n.resolved.link.underline_enabled);
        let mut link_info = info::about_link(cx.theme(), underline);
        let line = native_value(cx, |n| line_height_of(&n, &n.resolved.link.font));
        let link = native_info(
            Link::new("about-compatibility")
                .href(compatibility)
                .child(concat!(
                    "the README's Compatibility table at v",
                    env!("CARGO_PKG_VERSION")
                )),
            cx,
            geometry::link,
            "link",
            &mut link_info,
        )
        .when_some(line, |link, line| link.line_height(line))
        .info(&ui, "overlay-about-link", link_info)
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

/// Record the user's preferences with `change` made, where a native theme
/// is installed, which the showcase then installs again, resolved for the
/// context they make (`Showcase::new` observes them); tell `ui`: the theme
/// is rebuilt, so an info shown for a target no longer drawn would keep the
/// colours of the theme before.
pub(crate) fn change_preferences(
    ui: &Entity<InfoRegistry>,
    cx: &mut App,
    change: impl FnOnce(&mut PreferenceOverrides),
) {
    if cx.native_theme().is_some() {
        let mut overrides = PreferenceOverrides::of(cx);
        change(&mut overrides);
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

/// The Reduce motion row's description in the Preferences sheet.
pub(crate) const REDUCE_MOTION_DESCRIPTION: &str = "reduce_motion: forwarded to gpui's own reduce-motion flag; off is no request, so an OS request keeps motion reduced whatever the switch shows";

/// `info` with the reason the installed theme's contrast is lower than the
/// preferences ask, where `pref` is High contrast and it is
/// (`support::contrast_held`): the switch shows what was asked for. Where
/// High contrast set the palette chosen aside
/// (`support::palette_set_aside`), the High contrast switch's info says so.
fn with_contrast_held(info: WidgetInfo, pref: Preference, cx: &App) -> WidgetInfo {
    if !matches!(pref, Preference::HighContrast) {
        return info;
    }
    let info = match contrast_held(cx) {
        Some(held) => info.instance("resolved", held.text()),
        None => info,
    };
    match crate::support::palette_set_aside(cx) {
        Some(palette) => info.instance("palette", crate::support::palette_set_aside_text(palette)),
        None => info,
    }
}

/// A row whose field is one of the showcase's own Switches, built as
/// upstream's switch field builds one (setting/fields/bool.rs, `BoolField`)
/// but wrapped in its info, which upstream's field has no room for. The
/// switch shows the preference the installed theme was asked for
/// (`support::requested_preferences`), not the level the theme resolved to,
/// which its info gives where it is lower.
fn preference_item(
    ui: &Entity<InfoRegistry>,
    title: &'static str,
    description: &'static str,
    pref: Preference,
    disabled: bool,
) -> SettingItem {
    let ui = ui.clone();
    let field = SettingField::render(move |options, _window, cx| {
        let checked = requested_preferences(cx).is_some_and(|p| pref.get(&p));
        let selector = pref.selector();
        let clicked = ui.clone();
        #[cfg(feature = "widgets")]
        if let Some(r) = next_resolved(cx) {
            let switch_info = with_contrast_held(
                info::inputs::native_switch(r, pref.field(), checked, options.is_disabled()),
                pref,
                cx,
            );
            return widgets::Switch::new(selector)
                .checked(checked)
                .disabled(options.is_disabled())
                .on_change(move |on: &bool, _window, cx| {
                    change_preferences(&clicked, cx, |overrides| pref.set(overrides, *on));
                })
                .info(&ui, selector, switch_info)
                .debug_selector(move || selector.into());
        }
        let native = old_native(cx);
        let resolved = native.as_ref().map(|n| n.resolved);
        Switch::new(selector)
            .checked(checked)
            .disabled(options.is_disabled())
            .with_size(options.size())
            .when_some(native.map(variants::switch_color), |switch, color| {
                switch.color(color)
            })
            .on_click(move |on: &bool, _window, cx| {
                change_preferences(&clicked, cx, |overrides| pref.set(overrides, *on));
            })
            .info(
                &ui,
                selector,
                with_contrast_held(
                    info::preference_switch(
                        cx.theme(),
                        pref.field(),
                        checked,
                        options.is_disabled(),
                        resolved,
                    ),
                    pref,
                    cx,
                ),
            )
            .debug_selector(move || selector.into())
    });
    SettingItem::new(title, field)
        .description(description)
        .disabled(disabled)
}

/// The Preferences sheet (spec §2.8): `sheet`, `width` wide, holding a
/// Settings page with the four accessibility preferences. A change is
/// recorded (`change_preferences`), and the showcase's `_preferences`
/// observer installs the theme again, resolved for the context it makes
/// (`Showcase::install`, through `native_theme_gpui::next::apply`). Without
/// a native theme there is nothing to re-install, and the rows are disabled.
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
                    requested_preferences(cx)
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
        REDUCE_MOTION_DESCRIPTION,
        Preference::ReduceMotion,
        disabled,
    ))
    .item(preference_item(
        ui,
        "High contrast",
        "high_contrast: the theme is resolved at High contrast where it defines that level; the switch's info says where it does not",
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
            .description(
                "The theme is resolved again for them and installed with native_theme_gpui::next::apply",
            )
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

/// A section heading reading `text`, in the installed 0.8.0 theme's
/// section-heading role ([`heading_role`]): its size, line box and weight,
/// as the iced showcase's section titles are. Without an installed theme,
/// or where its role states no size or weight, upstream's `text_base`,
/// semibold. `id` is its info's id and its debug selector.
pub(crate) fn heading(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: impl Into<SharedString>,
) -> Stateful<Div> {
    let role = next_value(cx, |r| heading_role(&r.typography));
    let label = Label::new(text);
    let label = match role {
        Some(Ok(role)) => label
            .text_size(role.size)
            .line_height(role.line_height)
            .font_weight(role.weight),
        _ => label.text_base().font_semibold(),
    };
    label
        .info(ui, id, info::text::heading(cx.theme(), role))
        // As wide as its text, not its column: the space beside a heading
        // is not the heading.
        .self_start()
        .debug_selector(move || id.into())
}

/// What a section heading is lettered in ([`heading_role`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HeadingRole {
    /// The text style read: `typography.headline`, or `typography.body`
    /// where the theme states no headline.
    pub(crate) home: &'static str,
    /// Its `size_px`, which the theme's text scale has already scaled.
    pub(crate) size: Pixels,
    /// Its line box ([`line_box`]).
    pub(crate) line_height: Pixels,
    /// Its `weight`.
    pub(crate) weight: FontWeight,
}

/// The section heading's role in `typography`: `headline`, the platform's
/// section heading (`docs/platform-facts.md` §2.19, 0.7.2's
/// `text_scale.section_heading`), or `body` where the theme states no
/// headline (spec §4, `Typography`: a caller falls back to `body`). `Err`
/// with the home where that style states no size or no weight: the heading
/// keeps the showcase's own style (spec §10, rule 0).
pub(crate) fn heading_role(
    typography: &native_theme::next::Typography,
) -> Result<HeadingRole, &'static str> {
    use native_theme::next::TextStated;
    let (style, home, size_home, weight_home) = match &typography.headline {
        Some(headline) => (
            headline,
            "typography.headline",
            "typography.headline.size_px",
            "typography.headline.weight",
        ),
        None => (
            &typography.body,
            "typography.body",
            "typography.body.size_px",
            "typography.body.weight",
        ),
    };
    if !style.stated.contains(TextStated::SIZE_PX) {
        return Err(size_home);
    }
    if !style.stated.contains(TextStated::WEIGHT) {
        return Err(weight_home);
    }
    Ok(HeadingRole {
        home,
        size: px(style.size_px),
        line_height: px(line_box(typography, style)),
        weight: FontWeight(f32::from(style.weight)),
    })
}

/// The line box of `style`, one of `typography`'s text styles: its
/// `line_height` where stated, else its size by
/// `typography.line_height_multiplier` (spec §4, `Typography`; §5, "A text
/// part's `line_height`").
pub(crate) fn line_box(
    typography: &native_theme::next::Typography,
    style: &native_theme::next::TextStyle,
) -> f32 {
    if style
        .stated
        .contains(native_theme::next::TextStated::LINE_HEIGHT)
    {
        style.line_height
    } else {
        style.size_px * typography.line_height_multiplier
    }
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

/// A [`label`] whose info also holds the text it prints, for a label that
/// states values the page reads (the Layout page's spacing summary).
pub(crate) fn printed_label(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: impl Into<SharedString>,
) -> Stateful<Div> {
    let text: SharedString = text.into();
    Label::new(text.clone())
        .text_sm()
        .info(
            ui,
            id,
            info::text::label(cx.theme()).instance("text", text.to_string()),
        )
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
    /// `native_theme_gpui::variants::bordered_button`, filled with `ThemeColor`
    /// `button` at rest; built only where that returns `Some`
    /// (`ButtonKind::built`), and never selected: a selected Button takes no
    /// rest fill.
    Bordered,
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
            Self::Bordered => "Bordered",
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
            Self::Bordered => match bordered(cx) {
                Some(v) => button.custom(v).bg(cx.theme().button),
                None => button,
            },
            Self::Link => button.link(),
            Self::Text => button.text(),
            Self::DefaultOutline => button.outline(),
            Self::PrimaryOutline => button.primary().outline(),
        }
    }
    /// The kind a Button asked for as `self` is built as: `Default` for
    /// `Bordered` where the theme states no hover border, `self` otherwise.
    pub(crate) fn built(self, cx: &App) -> Self {
        match self {
            Self::Bordered if bordered(cx).is_none() => Self::Default,
            kind => kind,
        }
    }
}

/// `variants::bordered_button` under the theme a Button's info is given
/// ([`button_theme`]), where there is one and a rule of it states a hovered
/// button's stroke in a form gpui can take.
fn bordered(cx: &App) -> Option<ButtonCustomVariant> {
    button_theme(cx).and_then(|r| next::variants::bordered_button(r, cx).ok().flatten())
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
    /// spinner in place of its icon, turning this one, or, where the widget
    /// icons serve the chosen set's Loader, that ([`spinner_default`]).
    pub icon: Option<SampleIcon>,
}

/// A Button refined by `geometry::button`.
pub(crate) fn button(ui: &Entity<InfoRegistry>, cx: &App, spec: DemoButton) -> Stateful<Div> {
    button_turning(ui, cx, spec, None)
}

/// A loading Button refined by `geometry::button`, without an icon of its
/// own, given `spinner`, of the chosen icon theme, as its loading icon:
/// since gpui-component 0.10.0 a loading Button without an icon shows its
/// spinner where an icon would sit (button/button.rs:777-786). Where the
/// chosen icon theme has none, it is given none, as a loading Button with
/// an icon is given none, and the spinner turns its own default; so it is
/// where the widget icons serve the chosen set's Loader, which that default
/// then is ([`spinner_default`]).
pub(crate) fn loading_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: DemoButton,
    spinner: SampleIcon,
) -> Stateful<Div> {
    button_turning(ui, cx, spec, Some(spinner))
}

/// What the Spinner's own default icon, gpui-component's Loader, is in this
/// window: upstream's glyph, the chosen set's (the widget icons serve it),
/// or nothing (the widget icons serve the set, and it has no Loader).
/// Without `svg-rasterize` there are no widget icons, and it is always
/// upstream's glyph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(feature = "svg-rasterize"), allow(dead_code))]
pub(crate) enum SpinnerDefault {
    Upstream,
    Set,
    Empty,
}

/// [`SpinnerDefault`] in this window: what the showcase's widget icons
/// answer for the Loader's path (`WidgetIcons::lookup`), `Upstream` where
/// none are installed or the build has none.
pub(crate) fn spinner_default(cx: &App) -> SpinnerDefault {
    #[cfg(feature = "svg-rasterize")]
    {
        use gpui_component::IconNamed as _;
        use native_theme_gpui::icons::IconAnswer;
        let Some(icons) = cx.try_global::<crate::app::ShowcaseWidgetIcons>() else {
            return SpinnerDefault::Upstream;
        };
        match icons.0.lookup(&IconName::Loader.path(), cx) {
            IconAnswer::Draw(_) => SpinnerDefault::Set,
            IconAnswer::Empty(_) => SpinnerDefault::Empty,
            _ => SpinnerDefault::Upstream,
        }
    }
    #[cfg(not(feature = "svg-rasterize"))]
    {
        let _ = cx;
        SpinnerDefault::Upstream
    }
}

/// What a loading Button's info says its spinner turns: `icon`, of the
/// chosen icon theme, where that theme gives it; where it does not, the
/// Button is given no icon, and the spinner turns its own default,
/// gpui-component's Loader, as `default` says it is drawn.
pub(crate) fn spinner_note(icon: &SampleIcon, default: SpinnerDefault) -> String {
    if icon.shown() {
        return icon.note("");
    }
    let set = &icon.set;
    match default {
        SpinnerDefault::Empty => format!(
            "none: {set} holds no SVG of one colour for it, so the Button is given no icon to turn, and the Spinner's own default, gpui-component's Loader, is answered by the connector's widget icons, which serve {set} and find no Loader in it: the Spinner draws nothing (native-theme-gpui/icons/widget.rs, WidgetIcons)"
        ),
        SpinnerDefault::Set => format!(
            "the widget icons' Loader of {set}: the Button is given no icon to turn, so the Spinner turns its own default, gpui-component's Loader (gpui-component/spinner.rs, Spinner::new), which the connector's widget icons serve from {set} (native-theme-gpui/icons/widget.rs, WidgetIcons)"
        ),
        SpinnerDefault::Upstream => format!(
            "gpui-component's own Loader, the Spinner's default (gpui-component/spinner.rs, Spinner::new): {set} holds no SVG of one colour for it, so the Button is given no icon to turn, and a loading Button given none turns the Spinner's own (button/button.rs:777-786, loading_icon)"
        ),
    }
}

/// [`button`], and where `spinner` is given, a loading Button without an
/// icon given it as its loading icon ([`loading_button`]).
fn button_turning(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: DemoButton,
    spinner: Option<SampleIcon>,
) -> Stateful<Div> {
    let DemoButton {
        id,
        label,
        kind,
        state,
        icon,
    } = spec;
    let asked = kind;
    let kind = kind.built(cx);
    // `Button::icon` and `loading_icon` take an `Icon` (`ButtonIcon`) alone
    // (button/button.rs, button/button_icon.rs).
    let icon = icon.as_ref().map(SampleIcon::svg_only);
    let spinner = spinner.as_ref().map(SampleIcon::svg_only);
    // Where the widget icons draw the chosen set's Loader, the Spinner's own
    // default is already that set's, so no loading icon is given: every
    // Spinner of the window then turns the same icon.
    let default = match state {
        ButtonState::Loading => spinner_default(cx),
        ButtonState::Idle | ButtonState::Disabled => SpinnerDefault::Upstream,
    };
    let served = default == SpinnerDefault::Set;
    let spun = spinner
        .as_ref()
        .and_then(SampleIcon::icon)
        .filter(|_| !served);
    let drawn = icon.as_ref().and_then(SampleIcon::icon);
    let mut button_info = info::buttons::button(
        cx.theme(),
        kind,
        state,
        drawn.is_some(),
        served,
        None,
        button_theme(cx),
    );
    if kind != asked {
        let built_as = format!(
            "{} asked for, built as {}: variants::bordered_button gives no variant, the installed theme having no rule that states a hovered button's stroke, button.body(HOVER).stroke.color, in a form gpui can take, or no value gpui can take at a home geometry::button reads (native-theme-gpui/next/variants.rs, bordered_button)",
            asked.name(),
            kind.name()
        );
        match button_info
            .instance
            .iter_mut()
            .find(|n| n.what == "variant")
        {
            Some(note) => note.text = format!("{built_as}. {}", note.text),
            None => button_info = button_info.instance("variant", built_as),
        }
    }
    if let Some(icon) = &icon {
        button_info = match state {
            ButtonState::Loading if served => button_info.instance(
                "icon",
                icon.note("the Button shows its Spinner and its label"),
            ),
            ButtonState::Loading => {
                button_info.instance("spinner icon", spinner_note(icon, default))
            }
            ButtonState::Idle | ButtonState::Disabled => {
                button_info.instance("icon", icon.note("the Button shows its label alone"))
            }
        };
    } else if let Some(spinner) = spinner
        .as_ref()
        .filter(|_| state == ButtonState::Loading && !served)
    {
        button_info = button_info.instance("spinner icon", spinner_note(spinner, default));
    }
    let button = next_native_info(
        kind.apply(Button::new(id), cx),
        cx,
        next::geometry::button,
        "button",
        &mut button_info,
    );
    // The platform's disabled pair, which upstream's disabled style replays
    // the caller's refinement over (button/button.rs:830-836); a Button that
    // is not disabled must not take it, since it lands at rest too (:764).
    let theme = button_theme(cx);
    let disabled_pair = theme
        .filter(|_| state == ButtonState::Disabled)
        .and_then(|r| next::geometry::button_disabled(r).ok());
    if disabled_pair.is_some() {
        button_info = button_info.next_geometry("button_disabled");
    }
    // A disabled Default Button, the variant upstream draws an edge round
    // (button/button.rs, `RenderOnce for Button`: `border_l_1` and its
    // siblings), under a native theme: its fill and its fade go on a surface
    // under it that draws its edge too (`faded_surface`), and the rest of the
    // pair on the Button, which is left without a fill or an edge colour.
    let surface = match (&disabled_pair, theme) {
        (Some(pair), Some(r)) if kind == ButtonKind::Default => {
            SurfaceLine::of(&r.button.body(WidgetState::NONE)).map(|line| (pair.clone(), line))
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
    if let Some((_, line)) = &surface {
        button_info = button_info.config("surface", line.disabled_button_note());
    }
    let button = labelled(ui, cx, refined(button, on_button.as_ref()), id, label)
        .when_some(drawn, |button, icon| match state {
            ButtonState::Loading if served => button.icon(icon),
            ButtonState::Loading => button.loading_icon(icon.clone()).icon(icon),
            ButtonState::Idle | ButtonState::Disabled => button.icon(icon),
        })
        .when_some(spun, |button, spun| button.loading_icon(spun))
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
        Some((fill, border)) => faded_surface(&border, &fill, button)
            .flex_none()
            .id(SharedString::from(format!("{id}-surface"))),
        None => button,
    }
}

/// A Default Button at `size`, left without `geometry::button`: this row
/// shows upstream's own size scale, which the refinement would overrule
/// (button/button.rs:694-709, then :764).
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
            false,
            Some(size),
            None,
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
/// their corners itself (button/dropdown_button.rs:187-224).
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
    // `Toggle::icon` takes an `Icon` alone (button/toggle.rs).
    let icon = &icon.svg_only();
    let mut toggle_info = info::buttons::toggle(
        cx.theme(),
        icon.shown(),
        icon.note("the Toggle shows the icon's name as its label instead"),
        checked,
    );
    let toggle = match icon.icon() {
        Some(drawn) => Toggle::new(id).icon(drawn),
        None => Toggle::new(id).label(icon_name),
    };
    let style = info::next_native_value(cx, next::geometry::toggle, "toggle", &mut toggle_info);
    if style.is_some() {
        toggle_info = info::buttons::styled_toggle(toggle_info);
    }
    refined(toggle, style.as_ref())
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
    let mut group_info = info::buttons::toggle_group(cx.theme());
    // The group's info takes the builder's line, each Toggle its refinement.
    let style = info::next_native_value(cx, next::geometry::toggle, "toggle", &mut group_info);
    if style.is_some() {
        group_info = info::buttons::styled_toggle(group_info);
    }
    ToggleGroup::new(id)
        .children(
            labels
                .iter()
                .map(|&label| refined(Toggle::new(label).label(label), style.as_ref())),
        )
        .info(ui, id, group_info)
        .debug_selector(move || id.into())
}

/// A `Clipboard` copying `value`, reading `label` beside its icon where one
/// is given.
pub(crate) fn clipboard(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    value: &'static str,
    label: Option<&'static str>,
) -> Stateful<Div> {
    Clipboard::new(id)
        .value(value)
        .when_some(label, Clipboard::label)
        .info(ui, id, info::buttons::clipboard(cx.theme(), value, label))
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
    text_field(ui, cx, id, state, field, disabled, width.into(), false)
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
    text_field(
        ui,
        cx,
        id,
        state,
        InputField::Refined,
        false,
        width.into(),
        true,
    )
}

/// [`text_input`], `width` wide, drawing the state's inline tokens as
/// `InputToken` chips where `tokens`.
#[allow(clippy::too_many_arguments)]
fn text_field(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<InputState>,
    field: InputField,
    disabled: bool,
    width: Length,
    tokens: bool,
) -> Stateful<Div> {
    let theme = next_resolved(cx);
    let built = match field {
        InputField::Refined => theme.map(|r| RefinedField::of(r, disabled)),
        InputField::HeightOnly => None,
    };
    let taken = match (field, &built) {
        (InputField::Refined, built) => info::inputs::FieldTaken {
            builder: built.as_ref().is_some_and(|built| built.applied),
            fill: built.as_ref().is_some_and(|built| built.fill.is_ok()),
            surface: built.as_ref().is_some_and(|built| built.ground.is_some()),
        },
        (InputField::HeightOnly, _) => info::inputs::FieldTaken {
            builder: theme.is_some_and(|r| next::geometry::input_height(r).is_ok()),
            fill: false,
            surface: false,
        },
    };
    let mut input_info = info::inputs::input(cx.theme(), field, taken);
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
            let input =
                next_native_info(input, cx, next::geometry::input, "input", &mut input_info);
            let (fill, ground) = match built {
                Some(RefinedField { fill, ground, .. }) => (Some(fill), ground),
                None => (None, None),
            };
            match &fill {
                Some(Err(unstated)) => {
                    input_info = input_info.next_unmapped("input_fill", *unstated);
                }
                Some(Ok(_)) => {
                    input_info = input_info.next_geometry("input_fill");
                    if ground.is_none() {
                        input_info = input_info.config("surface", NO_INPUT_SURFACE);
                    }
                }
                None => {}
            }
            let none = cx.theme().transparent;
            match (ground, theme) {
                (Some(FieldGround::Faded(fill, line)), Some(r)) => {
                    input_info = info::inputs::disabled_input_surface(input_info, r, fill.opacity);
                    // The fill goes under the field; the disabled text
                    // colour stays on it, where the editor reads it.
                    let input = match fill.text.color {
                        Some(ink) => input.text_color(ink),
                        None => input,
                    };
                    return faded_surface(&line, &fill, input.bg(none).border_color(none))
                        .w(width)
                        .info(ui, id, input_info)
                        .debug_selector(move || id.into());
                }
                (Some(FieldGround::Surface(surface)), Some(r)) => {
                    input_info = info::inputs::input_surface(input_info, r, surface.line.width);
                    return input_surface(&surface, input.bg(none).border_color(none))
                        .w(width)
                        .info(ui, id, input_info)
                        .debug_selector(move || id.into());
                }
                // The platform's fill for the state the field is built in,
                // after upstream's (input/input.rs:800, then :808), on the
                // field itself where no surface is drawn under it.
                _ => refined(input, fill.and_then(Result::ok).as_ref()),
            }
        }
        // Through the caller's style, which Input applies after its own
        // height and line height (input/input.rs:788-792, then :808).
        InputField::HeightOnly => next_native_info(
            input,
            cx,
            next::geometry::input_height,
            "input_height",
            &mut input_info,
        ),
    };
    input
        .info(ui, id, input_info)
        .debug_selector(move || id.into())
}

/// The "surface" line of a refined field whose surface the showcase does
/// not draw, though `geometry::input_fill` applies.
const NO_INPUT_SURFACE: &str = "none: the showcase draws the field's surface only where geometry::input and geometry::input_fill apply and input.field(NONE) states its fill, its stroke and its corners in a form gpui can take; the field takes geometry::input_fill's refinement over upstream's fill, after it (input/input.rs, Input::render), and keeps upstream's edge colour";

/// The "surface" line of a text area whose surface the showcase does not
/// draw under an installed 0.8.0 theme.
const NO_TEXT_AREA_SURFACE: &str = "none: the showcase draws the text area's surface only where geometry::text_area applies and text_area.field(NONE) states its fill, its stroke and its corners in a form gpui can take; the text area keeps upstream's fill and edge colour";

/// What a refined field built `disabled` or not takes of a 0.8.0 theme
/// ([`RefinedField::of`]).
#[derive(Debug)]
pub(crate) struct RefinedField {
    /// Whether `geometry::input` applies.
    pub(crate) applied: bool,
    /// `geometry::input_fill`'s refinement for the field's state, or the
    /// home it lacks.
    pub(crate) fill: next::Mapped<StyleRefinement>,
    /// What the field is drawn over, where both builders apply and
    /// `input.field(NONE)` states its surface; `None` where the field is
    /// drawn without one, taking `fill` itself where it applies.
    pub(crate) ground: Option<FieldGround>,
}

impl RefinedField {
    /// What a refined field built `disabled` or not takes of `r`.
    pub(crate) fn of(r: &native_theme::next::ResolvedTheme, disabled: bool) -> Self {
        let applied = next::geometry::input(r).is_ok();
        let fill = next::geometry::input_fill(r, disabled);
        let ground = match &fill {
            Ok(fill) if applied => {
                let rest = r.input.field(WidgetState::NONE);
                if disabled {
                    SurfaceLine::of(&rest)
                        .map(|line| FieldGround::Faded(Box::new(fill.clone()), line))
                } else {
                    FieldSurface::of(&rest, &r.input.field(WidgetState::HOVER))
                        .map(FieldGround::Surface)
                }
            }
            _ => None,
        };
        Self {
            applied,
            fill,
            ground,
        }
    }
}

/// What a refined field is drawn over: a disabled one, its
/// `geometry::input_fill` inside the line of its part at rest
/// ([`faded_surface`]); an enabled one, its surface ([`input_surface`]).
#[derive(Debug)]
pub(crate) enum FieldGround {
    Faded(Box<gpui::StyleRefinement>, SurfaceLine),
    Surface(FieldSurface),
}

/// The surface an enabled, refined field is drawn over ([`input_surface`]).
#[derive(Debug)]
pub(crate) struct FieldSurface {
    /// The fill of the field's part at rest.
    pub(crate) fill: Hsla,
    /// The line of the field's part at rest.
    pub(crate) line: SurfaceLine,
    /// The edge colour under the pointer, `None` where the line has none.
    pub(crate) hover: Option<Hsla>,
}

impl FieldSurface {
    /// The surface of a field of the 0.8.0 model, `rest` its part at rest
    /// and `hovered` its part under the pointer, where the theme states the
    /// fill at rest as one colour and the line as [`SurfaceLine::of`] reads
    /// it. Under the pointer the edge is the hovered part's where a rule
    /// states it for the state, transparent where the rule states no
    /// stroke; a soft option the theme leaves unstated keeps the edge it
    /// would cover.
    pub(crate) fn of(
        rest: &native_theme::next::PartStyle,
        hovered: &native_theme::next::PartStyle,
    ) -> Option<Self> {
        let line = SurfaceLine::of(rest)?;
        Some(Self {
            fill: next::fill_of(rest)?,
            hover: next::state_stroke_color_of(hovered).or(line.colour),
            line,
        })
    }
}

/// The group an enabled field's surface takes its hover from.
const INPUT_GROUP: &str = "input-surface";

/// An enabled, refined `field` over `surface`: the fill of the field's part
/// at rest framed by its line, and under the pointer by the hovered part's
/// edge colour. The caller leaves the field without a fill or an edge
/// colour of its own, in upstream's `transparent` token (theme/schema.rs).
///
/// `Input` is `Styled` only (input/input.rs:535) and its root's one state is
/// `focused` (:774-781), so it has no hover edge of its own: the field is
/// left without a fill or an edge colour (its caller's style lands after its
/// own, :808) and an absolute box under it, the field's size, paints them.
/// Focused, the field's own edge in `ring` shows over the surface's.
fn input_surface(surface: &FieldSurface, field: impl IntoElement) -> Div {
    let line = &surface.line;
    let hover = surface.hover;
    div()
        .relative()
        .group(INPUT_GROUP)
        .child(
            div()
                .absolute()
                .inset_0()
                .rounded(px(line.radius.max(0.0)))
                .bg(surface.fill)
                .border(px(line.width))
                .when_some(line.colour, |edge, colour| edge.border_color(colour))
                .when_some(hover, |edge, hover| {
                    edge.group_hover(INPUT_GROUP, move |style| style.border_color(hover))
                }),
        )
        .child(field)
}

/// The installed 0.8.0 theme where `next::geometry::button` applies to it:
/// one that states every value the builder reads. What a Button's info is
/// given as the theme that refines it; under any other theme the Button
/// keeps upstream's geometry (spec §10, rule 0).
fn button_theme(cx: &App) -> Option<&native_theme::next::ResolvedTheme> {
    next_resolved(cx).filter(|r| next::geometry::button(r).is_ok())
}

/// The line [`faded_surface`] draws round a disabled control: the control's
/// edge width, corner radius and edge colour.
#[derive(Debug)]
pub(crate) struct SurfaceLine {
    pub(crate) width: f32,
    pub(crate) radius: f32,
    /// `None` where the stroke is stated as none, 0 wide.
    pub(crate) colour: Option<Hsla>,
}

impl SurfaceLine {
    /// The "surface" line of a disabled Default Button's info whose surface
    /// is drawn round by this line, `button.body(NONE)`'s: its fill inside
    /// the line where one is drawn, and with no line where the stroke is
    /// none or 0 wide.
    pub(crate) fn disabled_button_note(&self) -> &'static str {
        if self.colour.is_some() && self.width > 0.0 {
            "geometry::button_disabled's fill inside button.body(NONE).stroke.color's line, drawn by the showcase under the Button, which it leaves without a fill or an edge of its own, the whole faded by button.disabled_opacity: gpui fades each quad on its own (gpui-pre window.rs, paint_quad) and draws a quad's border over its own fill (gpui-pre-wgpu shaders.wgsl, fs_quad), so a fill under the line would show through the faded line"
        } else {
            "geometry::button_disabled's fill with no line round it, button.body(NONE).stroke being none or 0 wide, drawn by the showcase under the Button, which it leaves without a fill or an edge of its own, the whole faded by button.disabled_opacity (gpui-pre window.rs, paint_quad)"
        }
    }

    /// The line of a part of the 0.8.0 model, where the theme states its
    /// stroke as one width and colour, or as none, which is no line, 0 wide
    /// and of no colour (spec §10, rule 2), and its corners as one radius.
    pub(crate) fn of(part: &native_theme::next::PartStyle) -> Option<Self> {
        Some(Self {
            width: info::stroke_width_of(part)?,
            radius: info::radius_of(part)?,
            colour: info::stroke_color_of(part),
        })
    }
}

/// A disabled control over its surface: the fill `fill` gives (a disabled
/// field's `geometry::input_fill`, a disabled Button's
/// `geometry::button_disabled`) inside `border`'s line, the line in
/// `border.colour`, and the whole faded by `fill`'s opacity. The caller
/// leaves the control without a fill or an edge colour of its own, and
/// unfaded.
///
/// The fill is a box of its own inside the line, not under it: gpui fades
/// each primitive it paints on its own (gpui-pre src/window.rs:4937-4945,
/// `paint_quad`), and a quad draws its border over its own background
/// (gpui-pre-wgpu src/shaders.wgsl:887, `fs_quad`), so a faded control whose
/// fill lay under its line would show the fill through the faded line,
/// where a platform fades the control as one.
fn faded_surface(
    border: &SurfaceLine,
    fill: &gpui::StyleRefinement,
    field: impl IntoElement,
) -> Div {
    let line = border.width.max(0.0);
    let radius = border.radius.max(0.0);
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
                .when_some(border.colour, |edge, colour| edge.border_color(colour)),
        )
        .child(field)
}

/// A `Textarea` over `state`, `width` by `height`, refined by
/// `geometry::text_area`: it renders as an `Input` (input/textarea.rs:221),
/// padded by the sides `text_area.field(NONE)` states, the multi-line
/// field's own (docs/platform-facts.md §2.29); its `height` goes on after
/// the builder, and under a 0.8.0 theme on the surface it is drawn over
/// ([`surfaced_textarea`]).
pub(crate) fn textarea(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TextareaState>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let info = info::inputs::textarea(cx.theme(), text_area_taken(cx));
    surfaced_textarea(ui, cx, id, state, width, Some(height), info)
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
    let info = info::inputs::rows_textarea(cx.theme(), text_area_taken(cx));
    surfaced_textarea(ui, cx, id, state, width, None, info)
}

/// What a text area takes of the installed 0.8.0 theme: `geometry::text_area`
/// where it applies, and the surface it is drawn over
/// ([`text_area_surface`]), whose fill is the one it takes.
fn text_area_taken(cx: &App) -> info::inputs::FieldTaken {
    let surface = text_area_surface(cx).is_some();
    info::inputs::FieldTaken {
        builder: next_resolved(cx).is_some_and(|r| next::geometry::text_area(r).is_ok()),
        fill: surface,
        surface,
    }
}

/// The surface a text area is drawn over under the installed 0.8.0 theme
/// ([`input_surface`]): its own field's, `text_area.field(NONE)` and
/// `text_area.field(HOVER)`, where `geometry::text_area` applies and the
/// field at rest states its fill and its line.
fn text_area_surface(cx: &App) -> Option<FieldSurface> {
    next_resolved(cx).and_then(text_area_surface_of)
}

/// [`text_area_surface`] of `r`.
pub(crate) fn text_area_surface_of(r: &native_theme::next::ResolvedTheme) -> Option<FieldSurface> {
    next::geometry::text_area(r).ok()?;
    FieldSurface::of(
        &r.text_area.field(WidgetState::NONE),
        &r.text_area.field(WidgetState::HOVER),
    )
}

/// A `Textarea` over `state`, `width` wide and `height` tall, or as tall as
/// its rows where `height` is `None`, refined by `geometry::text_area`,
/// reporting `textarea_info`. Under a 0.8.0 theme
/// that states its surface ([`text_area_surface`]) it is drawn as an
/// enabled text input is ([`input_surface`]), over its own field's surface,
/// its own fill and edge colour left transparent.
fn surfaced_textarea(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TextareaState>,
    width: Pixels,
    height: Option<Pixels>,
    mut textarea_info: WidgetInfo,
) -> Stateful<Div> {
    let mut textarea = next_native_info(
        Textarea::new(state).w(width),
        cx,
        next::geometry::text_area,
        "text_area",
        &mut textarea_info,
    );
    let surface = text_area_surface(cx);
    if surface.is_none() && next_resolved(cx).is_some() {
        textarea_info = textarea_info.config("surface", NO_TEXT_AREA_SURFACE);
    }
    match height {
        Some(height) => textarea = Styled::h(textarea, height),
        None => {
            let style = Styled::style(&mut textarea);
            style.size.height = None;
            style.min_size.height = None;
        }
    }
    if let (Some(surface), Some(r)) = (surface, next_resolved(cx)) {
        textarea_info = info::inputs::text_area_surface(textarea_info, r, surface.line.width);
        let none = cx.theme().transparent;
        return input_surface(&surface, textarea.bg(none).border_color(none))
            .w(width)
            .when_some(height, |surface, height| surface.h(height))
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

/// Three `InputGroup`s, `width` wide, one above the other `gap` apart (the
/// page's `layout.widget_gap`, none where it is unstated): a field behind a
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
    gap: Option<Pixels>,
) -> Stateful<Div> {
    // `Button::icon` takes an `Icon` (`ButtonIcon`) alone (button/button.rs).
    let copy_icon = states.copy_icon.svg_only();
    let mut groups_info = info::inputs::input_groups(cx.theme())
        .instance(
            "search icon",
            states.search_icon.note("the first field's addon is empty"),
        )
        .instance(
            "copy icon",
            copy_icon.note("the Copy button shows its label alone"),
        );
    // One refinement, recorded once, for both single-line groups; the
    // textarea group keeps upstream's frame. Without its padding: the frame
    // is not the Input, which sits inside it and keeps its own padding
    // (input/group.rs, render_control), so the frame padded as well would
    // inset the field twice.
    let mut frame = next_native_info(
        StyleRefinement::default(),
        cx,
        next::geometry::input,
        "input",
        &mut groups_info,
    );
    frame.padding = StyleRefinement::default().padding;
    // `InputGroupButton::new` is a ghost button that upstream repaints,
    // inside a group, with a hover of `theme.muted` (`input/group.rs:556-595`):
    // a grey that under Breeze is barely distinguishable from the field,
    // while every button around it hovers blue. A custom variant makes
    // upstream skip that repaint.
    let copy = next_native_info(
        InputGroupButton::new("input-group-copy"),
        cx,
        next::geometry::input_group_button,
        "input_group_button",
        &mut groups_info,
    )
    .custom(variants::ghost_button(cx))
    .when_some(copy_icon.icon(), |button, icon| button.icon(icon))
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
    with_gap(v_flex(), gap)
        .w(width)
        .child(
            InputGroup::new("input-group-inline")
                .input(Input::new(states.search))
                .addon(
                    InputGroupAddon::new("input-group-inline-addon")
                        .children(states.search_icon.glyph()),
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

/// The chevrons a theme-drawn number field's step buttons show, of the
/// chosen icon theme: ChevronUp on the increment, ChevronDown on the
/// decrement.
pub(crate) struct StepIcons {
    pub up: SampleIcon,
    pub down: SampleIcon,
}

/// A number field over `state`, `width` wide, which a screen reader names
/// `label`: it draws no text of its own to be named by.
///
/// Under a 0.8.0 theme that states what it reads ([`NumberFrame`]),
/// gpui-component's `NumberInput` rebuilt over the unstyled spin button it
/// composes (`gpui_base::NumberInput`, input/number_input.rs:125-216), with
/// what the theme states for a text field where upstream sets literals no
/// caller reaches: the frame is `input.field(NONE)`'s fill edged by its
/// stroke and corners at `geometry::input`'s height, in
/// `input.field(HOVER).stroke.color` under the pointer and
/// `input.field(FOCUSED).stroke.color` while focused, upstream's `ring`
/// where no rule states it ([`number_edge`]); the value is an
/// `Input` padded by the sides `input.field(NONE)` states, in
/// `input.text(NONE)`. Upstream's
/// fills its frame with its `input_background` (:125-126) and pads its value
/// by its Size (:173-180). The theme states no spin-box layout: the value
/// starts at the field's padding and the step buttons stack at its right
/// end (gpui-base's `controls_right`), as the desktops' spin boxes lay them
/// out (Qt's QSpinBox, GTK's GtkSpinButton), where upstream's centres the
/// value between − and + (:173-180); the buttons are upstream's otherwise
/// (:128-171), each showing its chevron of `steps`, the chosen icon theme's
/// at `geometry::icon_size_small` or, where a step is shorter, the step's
/// height (`info::inputs::StepGlyph`), or none where that theme has none.
/// Without such a theme, upstream's `NumberInput`, which draws its own
/// Minus and Plus.
#[allow(clippy::too_many_arguments)]
pub(crate) fn number_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    window: &Window,
    id: &'static str,
    label: &'static str,
    state: &Entity<InputState>,
    width: Pixels,
    steps: &StepIcons,
) -> Stateful<Div> {
    let theme = next_resolved(cx);
    let frame = theme.map(NumberFrame::of);
    let (r, number) = match (theme, frame) {
        (Some(r), Some(Ok(number))) => (r, number),
        (_, frame) => {
            let number_info = info::inputs::number_input(cx.theme(), frame.and_then(Result::err));
            return NumberInput::new(state)
                .accessibility_label(label)
                .placeholder("Enter a number")
                .with_size(Size::Medium)
                .w(width)
                .info(ui, id, number_info);
        }
    };
    let NumberFrame {
        field,
        surface,
        focus,
        foreground,
        glyph,
    } = number;
    let line = &surface.line;
    let focused = gpui::Focusable::focus_handle(state.read(cx), cx).is_focused(window);
    let mut number_info = info::inputs::native_number_input(
        r,
        cx.theme(),
        focused,
        &steps.up,
        &steps.down,
        glyph,
        line.width,
        line.radius,
    )
    .next_geometry("input");
    if (steps.up.shown() || steps.down.shown()) && glyph.at_icon_size_small() {
        number_info = number_info.next_geometry("icon_size_small");
    }
    let edge = number_edge(focused, focus, line.colour, cx.theme().ring);
    let hover = surface.hover;
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
    // Upstream's step buttons (input/number_input.rs:128-133, :152-171,
    // :187-204): transparent, tinted with the frame's edge colour under the
    // pointer and while pressed, a pixel tighter at the outer corners, as
    // wide as upstream's at the field's Size (:162-166, `min_w_8`); stacked,
    // each half the field's height, with its chevron of the chosen icon
    // theme. Their colours are the homes upstream's tokens carry under a
    // 0.8.0 theme: secondary_foreground is `button.label(NONE)`'s colour,
    // input the field's edge colour.
    let tint = line.colour;
    let radius = px(line.radius.max(0.0) - line.width).max(px(0.));
    // Each step's glyph at `icons.sizes.small`, or where a step is shorter,
    // the step's height (`info::inputs::StepGlyph`, M9-177).
    let side = Size::Size(px(glyph.side()));
    let (up, down) = (
        steps.up.glyph().map(|glyph| glyph.with_size(side)),
        steps.down.glyph().map(|glyph| glyph.with_size(side)),
    );
    let step = move |button: gpui_base::Button, label: &'static str, icon: Option<Glyph>| {
        button
            .accessibility_label(label)
            .flex()
            .items_center()
            .justify_center()
            .text_color(foreground)
            .when_some(tint, |button, tint| {
                button
                    .hover(move |this| this.bg(tint.opacity(NUMBER_STEP_HOVER)))
                    .active(move |this| this.bg(tint.opacity(NUMBER_STEP_ACTIVE)))
            })
            .min_w_8()
            .debug_selector(move || format!("{id}-step-{label}"))
            .children(icon)
    };
    let spin = gpui_base::NumberInput::new(state)
        .accessibility_label(label)
        .size_full()
        .controls_right()
        .increment_button(move |b| step(b, "Increment", up).rounded_tr(radius))
        .input(
            Input::new(state)
                .appearance(false)
                .with_size(Size::Medium)
                .h_full()
                .gap_0()
                .rounded_none()
                .refine_style(&value),
        )
        .decrement_button(move |b| step(b, "Decrement", down).rounded_br(radius));
    div()
        .id("number-frame")
        .w(width)
        .bg(surface.fill)
        .refine_style(&frame)
        .when_some(edge, |el, edge| el.border_color(edge))
        .when_some(hover.filter(|_| !focused), |el, hover| {
            el.hover(move |s| s.border_color(hover))
        })
        .child(spin)
        .info(ui, id, number_info)
        .debug_selector(move || id.into())
}

/// The edge colour of the number field's frame ([`number_input`]),
/// `focused` or not: at rest `rest`, the line's, none where the stroke is
/// stated as none; focused, `focus`, the `input.field(FOCUSED).stroke.color`
/// a rule states, else `ring`, upstream's: gpui-component's `NumberInput`
/// edges its frame in `input` at rest and in `ring` while focused
/// (input/number_input.rs:138-142, :230-231), so the toolkit changes the
/// edge colour between rest and focus, and where the theme states none for
/// focus its own focused value stands (spec §10, rule 0).
pub(crate) fn number_edge(
    focused: bool,
    focus: Option<Hsla>,
    rest: Option<Hsla>,
    ring: Hsla,
) -> Option<Hsla> {
    if focused { focus.or(Some(ring)) } else { rest }
}

/// What the number field the showcase draws ([`number_input`]) reads of the
/// installed 0.8.0 theme.
pub(crate) struct NumberFrame {
    /// `geometry::input`'s refinement.
    field: StyleRefinement,
    /// The field's surface, `input.field(NONE)` and `input.field(HOVER)`.
    surface: FieldSurface,
    /// `input.field(FOCUSED).stroke.color`, where a rule states it;
    /// transparent where it states no stroke.
    pub(crate) focus: Option<Hsla>,
    /// The step glyphs' colour, `button.label(NONE).color`.
    foreground: Hsla,
    /// The step glyphs' size.
    glyph: info::inputs::StepGlyph,
}

impl NumberFrame {
    /// What `r` states of the field, or the home it leaves without a value
    /// gpui can take.
    pub(crate) fn of(r: &native_theme::next::ResolvedTheme) -> Result<Self, &'static str> {
        let field = next::geometry::input(r).map_err(|unstated| unstated.home())?;
        let surface = FieldSurface::of(
            &r.input.field(WidgetState::NONE),
            &r.input.field(WidgetState::HOVER),
        )
        .ok_or("input.field(NONE)'s fill, stroke or corners")?;
        Ok(Self {
            field,
            surface,
            focus: next::state_stroke_color_of(&r.input.field(WidgetState::FOCUSED)),
            foreground: next::text_color_of(&r.button.label(WidgetState::NONE))
                .ok_or("button.label(NONE).color")?,
            glyph: info::inputs::StepGlyph::of(r)?,
        })
    }
}

/// The opacity of the frame's edge colour a step button of the number field
/// is tinted with under the pointer and while pressed: upstream's
/// (input/number_input.rs:132-133, `input.opacity(0.4)`, `(0.6)`).
pub(crate) const NUMBER_STEP_HOVER: f32 = 0.4;
pub(crate) const NUMBER_STEP_ACTIVE: f32 = 0.6;

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
    if let Some(r) = next_resolved(cx) {
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
    let checkbox = next_native_info(
        Checkbox::new(id),
        cx,
        next::geometry::checkbox,
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

/// The theme [`mixed_checkbox`] draws a theme-drawn box from: the installed
/// native theme's, with the `widgets` feature. `None` where it draws
/// gpui-component's own in its place.
#[cfg(feature = "widgets")]
pub(crate) fn theme_checkbox(cx: &App) -> Option<&native_theme::next::ResolvedTheme> {
    next_resolved(cx)
}

/// The theme [`mixed_checkbox`] draws a theme-drawn box from: none without
/// the `widgets` feature, where it draws gpui-component's own.
#[cfg(not(feature = "widgets"))]
pub(crate) fn theme_checkbox(_: &App) -> Option<&native_theme::next::ResolvedTheme> {
    None
}

/// The Inputs page's label of its theme-drawn mixed checkbox: theme-drawn
/// where [`theme_checkbox`] gives a theme whose look the widget draws, a
/// mixed box being drawn as a checked one (`info::inputs::checkbox_drawn`:
/// the look, the window's colour and the label font stated), else
/// gpui-component's own, which the widget draws in its place.
pub(crate) fn mixed_checkbox_label(cx: &App) -> &'static str {
    #[cfg(feature = "widgets")]
    let drawn =
        theme_checkbox(cx).is_some_and(|r| info::inputs::checkbox_drawn(r, true, false).is_ok());
    #[cfg(not(feature = "widgets"))]
    let drawn = theme_checkbox(cx).is_some();
    mixed_label(drawn)
}

/// The label [`mixed_checkbox_label`] gives: theme-drawn where the widget
/// draws the theme's look (`drawn`), else gpui-component's own.
pub(crate) fn mixed_label(drawn: bool) -> &'static str {
    if drawn {
        "Mixed, theme-drawn"
    } else {
        "Mixed, gpui-component's"
    }
}

/// A `Checkbox` reading `label` in the mixed state, `indeterminate(true)`,
/// with no handler: gpui-component's own, refined by `geometry::checkbox`,
/// or where `theme_drawn` and [`theme_checkbox`] gives a theme, the
/// connector's theme-drawn `widgets::Checkbox`.
pub(crate) fn mixed_checkbox(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    theme_drawn: bool,
) -> Stateful<Div> {
    #[cfg(feature = "widgets")]
    if let Some(r) = theme_checkbox(cx).filter(|_| theme_drawn) {
        let checkbox_info = info::inputs::native_mixed_checkbox(r, label);
        return widgets::Checkbox::new(id)
            .label(label)
            .indeterminate(true)
            .when_some(elements::part_observer(ui, id), |checkbox, observer| {
                checkbox.on_part_bounds(observer)
            })
            .info(ui, id, checkbox_info)
            .debug_selector(move || id.into());
    }
    #[cfg(not(feature = "widgets"))]
    let _ = theme_drawn;
    let mut checkbox_info = info::inputs::mixed_checkbox(cx.theme(), label);
    next_native_info(
        Checkbox::new(id),
        cx,
        next::geometry::checkbox,
        "checkbox",
        &mut checkbox_info,
    )
    .label(label)
    .indeterminate(true)
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
/// (`radio.rs:411`), not the style.
pub(crate) fn radio_group(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    labels: &[&'static str],
    selected: Option<usize>,
    on_click: impl Fn(&usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    #[cfg(feature = "widgets")]
    if let Some(r) = next_resolved(cx) {
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
    let row = next_native_info(
        StyleRefinement::default(),
        cx,
        next::geometry::radio,
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
    if let Some(r) = next_resolved(cx) {
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
    let row = next_native_info(
        StyleRefinement::default(),
        cx,
        next::geometry::radio,
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
/// installed 0.8.0 theme's `switch.track(CHECKED).fill` through
/// `Switch::color` and `next::variants::switch_color`. A disabled one takes
/// no `on_click`.
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
    if let Some(r) = next_resolved(cx) {
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
    let theme = next_resolved(cx);
    let checked_background = theme.and_then(|r| next::variants::switch_color(r).ok());
    Switch::new(id)
        .label(label)
        .checked(checked)
        .disabled(disabled)
        .when_some(checked_background, |switch, color| switch.color(color))
        .when_some(on_click, |switch, on_click| switch.on_click(on_click))
        .info(
            ui,
            id,
            info::inputs::switch(cx.theme(), label, checked, disabled, theme),
        )
        .debug_selector(move || id.into())
}

/// A `Slider` over `state` along `axis`, `length` long (its width, or a
/// vertical one's height), which a screen reader names `label`: it draws no
/// text of its own to be named by. A range state draws a thumb at each end.
pub(crate) fn slider(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    state: &Entity<SliderState>,
    axis: Axis,
    length: Pixels,
) -> Stateful<Div> {
    let vertical = axis == Axis::Vertical;
    let range = state.read(cx).value().is_range();
    #[cfg(feature = "widgets")]
    if let Some(r) = next_resolved(cx) {
        return widgets::Slider::new(state)
            .accessibility_label(label)
            .axis(axis)
            .map(|slider| {
                if vertical {
                    slider.h(length)
                } else {
                    slider.w(length)
                }
            })
            .when_some(elements::part_observer(ui, id), |slider, observer| {
                slider.on_part_bounds(observer)
            })
            .info(ui, id, info::inputs::native_slider(r, vertical, range))
            .debug_selector(move || id.into());
    }
    Slider::new(state)
        .accessibility_label(label)
        .when(vertical, |slider| slider.vertical())
        .map(|slider| {
            if vertical {
                slider.h(length)
            } else {
                slider.w(length)
            }
        })
        .info(ui, id, info::inputs::slider(cx.theme(), vertical, range))
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
    let rating = match info::next_native_value(
        cx,
        next::geometry::icon_size_small,
        "icon_size_small",
        &mut rating_info,
    ) {
        Some(size) => rating.with_size(size),
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
    select_drawn(ui, cx, id, state, placeholder, width, false)
}

/// The rows of the Inputs page's searchable `Select`.
pub(crate) const HIGHLIGHT_SELECT_ROWS: [&str; 5] =
    ["Adwaita", "Breeze", "Breeze Dark", "Papirus", "Yaru"];

/// [`select`] over a searchable `state`, with `highlight_matches(true)`: the
/// part of each row that matches what is typed into its search field is
/// drawn in `list_highlight_foreground` (gpui-component label.rs,
/// `highlight_matches`).
pub(crate) fn highlighted_select(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
    placeholder: &'static str,
    width: Pixels,
) -> Stateful<Div> {
    select_drawn(ui, cx, id, state, placeholder, width, true)
}

/// [`select`], with `highlight_matches` set to `highlight`.
fn select_drawn(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<SelectState<SearchableVec<SharedString>>>,
    placeholder: &'static str,
    width: Pixels,
    highlight: bool,
) -> Stateful<Div> {
    let theme = next_resolved(cx);
    let select_info = info::inputs::select(cx.theme(), theme);
    let mut select_info = if highlight {
        info::inputs::match_highlight(select_info, cx.theme(), theme.is_some())
    } else {
        select_info
    };
    let select = next_native_info(
        Select::new(state)
            .placeholder(placeholder)
            .highlight_matches(highlight)
            .w(width),
        cx,
        next::geometry::select,
        "select",
        &mut select_info,
    );
    combo_surface(cx, refined(select, combo_fill(cx).as_ref()))
        .w(width)
        .info(ui, id, select_info)
        .debug_selector(move || id.into())
}

/// What a drop-down trigger that is not disabled is drawn with under the
/// installed 0.8.0 theme ([`combo_fill`], [`combo_surface`]).
pub(crate) struct ComboLook {
    /// `geometry::combo_box_fill`'s fill, `combo_box.field(NONE).fill`.
    background: Hsla,
    /// `combo_box.field(NONE).stroke.color`, `None` where the stroke is
    /// stated as none.
    edge: Option<Hsla>,
    /// `combo_box.field(NONE).stroke.width`.
    width: f32,
    /// `combo_box.field(NONE)`'s corner radius.
    radius: f32,
    /// `combo_box.field(HOVER).fill` laid over the fill at rest, where a
    /// rule states it.
    hover: Option<Hsla>,
    /// `combo_box.field(HOVER).stroke.color`, where a rule states it;
    /// transparent where it states no stroke.
    pub(crate) hover_edge: Option<Hsla>,
}

impl ComboLook {
    /// The edge [`combo_surface`] draws under the trigger, which then has
    /// none of its own ([`combo_fill`]): at rest `combo_box.field(NONE)`'s,
    /// under the pointer [`hover_edge`](Self::hover_edge), where a rule
    /// states it and the trigger is edged at all.
    fn hover_edges(&self) -> Option<(Option<Hsla>, Hsla)> {
        self.hover_edge
            .filter(|_| self.width > 0.0)
            .map(|hover| (self.edge, hover))
    }
}

/// The [`ComboLook`] `r` states, or the home it leaves without a value gpui
/// can take, which the trigger then keeps upstream's fill and edge for.
pub(crate) fn combo_look(r: &native_theme::next::ResolvedTheme) -> Result<ComboLook, &'static str> {
    // The builder sets an enabled trigger's fill alone, the one read below
    // (geometry::field_fill); it applies where the theme states every home
    // it reads.
    next::geometry::combo_box_fill(r, false).map_err(|unstated| unstated.home())?;
    let rest = r.combo_box.field(WidgetState::NONE);
    let hovered = r.combo_box.field(WidgetState::HOVER);
    let background = next::fill_of(&rest).ok_or("combo_box.field(NONE).fill")?;
    Ok(ComboLook {
        background,
        edge: next::stroke_color_of(&rest),
        width: next::stroke_width_of(&rest).ok_or("combo_box.field(NONE).stroke")?,
        radius: next::radius_of(&rest).ok_or("combo_box.field(NONE).corners")?,
        hover: next::state_fill_of(&hovered)
            .map(|hover| native_theme_gpui::layer_over(background, hover)),
        hover_edge: next::state_stroke_color_of(&hovered),
    })
}

/// A drop-down trigger's edge, `combo_box.field(NONE).stroke.color` (none
/// where the stroke is stated as none, which `geometry::select` and
/// `geometry::combobox` draw 0 wide; transparent where [`combo_surface`]
/// draws the edge under the trigger, [`ComboLook::hover_edges`]), and no
/// fill of its own, for a `Select` or a `Combobox` that is not disabled,
/// under a 0.8.0 theme that states its [`ComboLook`]; `None` otherwise:
/// [`combo_surface`] paints `geometry::combo_box_fill`'s fill under it. The
/// trigger paints `input_background()` and `input` first and refines itself
/// with the caller's style after (select.rs, `Select::render`; combobox.rs,
/// `Combobox::render`), so these land over them.
fn combo_fill(cx: &App) -> Option<StyleRefinement> {
    // Upstream's own `transparent` token, which the trigger starts its edge
    // with (select.rs:570).
    let none = cx.theme().transparent;
    let look = next_resolved(cx).and_then(|r| combo_look(r).ok())?;
    let fill = StyleRefinement::default().bg(none);
    Some(match (look.hover_edges(), look.edge) {
        (Some(_), _) => fill.border_color(none),
        (None, Some(edge)) => fill.border_color(edge),
        (None, None) => fill,
    })
}

/// The surface under a drop-down `trigger` refined by [`combo_fill`]:
/// `geometry::combo_box_fill`'s fill, `combo_box.field(NONE).fill`, moved
/// under the trigger so that under the pointer `combo_box.field(HOVER).fill`
/// can be laid over it as the connector lays its layers
/// (`native_theme_gpui::layer_over`), rounded as the trigger is
/// (`combo_box.field(NONE)`'s corners, `geometry::select`). Upstream's
/// trigger sets no hover (select.rs:562-580; combobox.rs:999-1016) and
/// neither `Select` nor `Combobox` is an `InteractiveElement`
/// (select.rs:832), so the hover fill is this element's. Without a 0.8.0
/// theme that states the [`ComboLook`] it is a plain box.
///
/// Where a rule states `combo_box.field(HOVER).stroke.color`, the edge is
/// drawn under the trigger, which [`combo_fill`] leaves edged in
/// transparent: `combo_box.field(NONE)`'s at rest and the hovered one under
/// the pointer, which takes the place of the edge at rest (none where the
/// rule states no stroke); the trigger is no `InteractiveElement` either.
/// The edge is a child before the trigger, laid absolutely under it, as
/// wide as `combo_box.field(NONE)`'s stroke, with only a hover style of the
/// surface's group: gpui paints a later child over an earlier one, so the
/// focused trigger's own edge in upstream's `ring` (select.rs:596-598)
/// stays over it, and an element that only has a group-hover style gets a
/// `Normal` hitbox, which leaves the trigger's mouse handling as it is
/// (gpui-pre 0.3.12 `window.rs:891-894`; `elements/div.rs:2460-2468`).
fn combo_surface(cx: &App, trigger: impl IntoElement) -> Div {
    // Upstream's own `transparent` token, as `combo_fill` takes it.
    let none = cx.theme().transparent;
    let look = next_resolved(cx).and_then(|r| combo_look(r).ok());
    match look {
        Some(look) => {
            let width = px(look.width.max(0.0));
            let radius = px(look.radius.max(0.0));
            let (fill, hover) = (look.background, look.hover);
            let edges = look.hover_edges();
            div()
                .rounded(radius)
                .bg(fill)
                .when_some(hover, |surface, hover| {
                    surface.hover(move |style| style.bg(hover))
                })
                .when(edges.is_some(), |surface| surface.group(COMBO_HOVER_GROUP))
                .when_some(edges, |surface, (rest, hovered)| {
                    surface.child(
                        div()
                            .absolute()
                            .inset_0()
                            .rounded(radius)
                            .border(width)
                            .border_color(rest.unwrap_or(none))
                            .group_hover(COMBO_HOVER_GROUP, move |style| {
                                style.border_color(hovered)
                            })
                            .debug_selector(|| "combo-hover-edge".into()),
                    )
                })
                .child(trigger)
        }
        None => div().child(trigger),
    }
}

/// The group [`combo_surface`] names its surface by, whose hover the edge
/// drawn under the trigger listens to.
const COMBO_HOVER_GROUP: &str = "showcase-combo-hover";

/// The group a menu row names itself by, whose hover and press its outline
/// ([`MenuLook::edge`]) follows.
const MENU_ROW_GROUP: &str = "showcase-menu-row";

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

/// How a `SpeechWaveform` draws the input: upstream's bars, or the water
/// surface `SpeechWaveform::liquid` draws across a row (gpui-component
/// speech/waveform.rs:85-101).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum WaveformKind {
    Bars,
    Liquid,
}

/// A `SpeechWaveform` of `kind` over `state`, upstream's own: its size and
/// colours are literals and theme tokens it reads itself, so no geometry
/// builder reaches it. The liquid one spans its row. gpui redraws the window
/// when `state` notifies, once per new level.
pub(crate) fn speech_waveform(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<SpeechState>,
    kind: WaveformKind,
) -> Stateful<Div> {
    let waveform = match kind {
        WaveformKind::Bars => SpeechWaveform::new(state),
        WaveformKind::Liquid => SpeechWaveform::new(state).liquid().w_full(),
    };
    waveform
        .info(ui, id, info::inputs::speech_waveform(cx.theme(), kind))
        .debug_selector(move || id.into())
}

/// The stop button for `state` while it captures, as `SpeechIndicator`'s doc
/// asks a caller to build one (speech/indicator.rs:52-58): a selected Ghost
/// Button -- built with `variants::ghost_button`, as the toolbar's are, so
/// it letters in the platform's button text colour, which the indicator
/// paints its bars in -- whose only child is the indicator, its padding
/// removed so the indicator fills it. The Button is a square of
/// `button.body(NONE).min_size[1]`, rounded by `button.body(NONE).corners`,
/// where the installed 0.8.0 theme states both, and upstream's icon-Button
/// square otherwise;
/// the indicator's stop glyph takes `geometry::icon_size_toolbar`, the size
/// of the icon the Button stands for. `keyboard_focused` is whether `focus`,
/// the Button's, has keyboard focus: no pointer reveals the stop glyph then,
/// so the Button shows its name instead -- the chosen icon theme has no stop
/// icon, and another theme's is never shown. A click stops the session.
pub(crate) fn speech_indicator(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<SpeechState>,
    focus: &gpui::FocusHandle,
    keyboard_focused: bool,
) -> Stateful<Div> {
    let stated = next_resolved(cx).and_then(info::inputs::StopSquare::of);
    let mut indicator_info = info::inputs::speech_indicator(cx.theme(), stated, keyboard_focused);
    let square = stated.map(|s| (px(s.side), px(s.radius.max(0.))));
    let glyph = if keyboard_focused {
        None
    } else {
        info::next_native_value(
            cx,
            next::geometry::icon_size_toolbar,
            "icon_size_toolbar",
            &mut indicator_info,
        )
    };
    let button = ButtonKind::Ghost
        .apply(Button::new(id), cx)
        .selected(true)
        .track_focus(focus)
        .accessibility_label("Stop");
    // The name keeps upstream's side padding and may widen the square; the
    // indicator fills it, with none.
    let button = match (square, keyboard_focused) {
        (Some((side, radius)), false) => button.p_0().size(side).rounded(radius),
        (Some((side, radius)), true) => button.h(side).min_w(side).rounded(radius),
        (None, false) => button.p_0().size_8(),
        (None, true) => button.min_w_8(),
    };
    let button = if keyboard_focused {
        labelled(ui, cx, button, id, "Stop")
    } else {
        let indicator = SpeechIndicator::new(state);
        button.child(match glyph {
            Some(size) => indicator.with_size(size),
            None => indicator,
        })
    };
    let state = state.clone();
    let button = button.on_click(move |_, _, cx| state.update(cx, |s, cx| s.stop(cx)));
    // `InfoExt::info` by path: `ButtonVariants::info` picks the Info variant.
    InfoExt::info(button, ui, id, indicator_info).debug_selector(move || id.into())
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

/// The striped, bordered `DataTable` over `state`, `height` tall, its header
/// row `geometry::list_header_height` tall where the theme states one and
/// its body rows' height otherwise. Its rows report themselves through the
/// delegate, which builds them with `data_table_header` and `data_table_row`.
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
    let styled = old_native(cx).is_some();
    let header = native_value(cx, geometry::list_header_height).flatten();
    DataTable::new(state)
        .with_size(native_value(cx, geometry::data_table_size).unwrap_or_default())
        .when_some(header, DataTable::header_row_height)
        .stripe(true)
        .bordered(true)
        .info(
            ui,
            id,
            info::data::data_table(cx.theme(), rows, columns, styled, header.is_some()),
        )
        .native(cx, geometry::table)
        .h(height)
        .debug_selector(move || id.into())
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
    /// An odd row of a striped table (table/state.rs:2339).
    pub striped: bool,
    /// The last of the delegate's rows.
    pub last: bool,
    /// The table's selected row.
    pub selected: bool,
    /// The table is selecting rows, so it paints its selected row as one
    /// (table/state.rs:540, 2536); a column selection keeps the selected row but
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

/// A box `width` by `height` around the List over `state`, refined by
/// `geometry::list`, which frames it and fills it with
/// `list.background_color`; `selector` is its debug selector. A List paints
/// no frame and no fill of its own (list/list.rs, `RenderOnce for List`), so
/// the box is its frame and its fill; the rows report themselves (`ListRow`).
pub(crate) fn list(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    selector: &'static str,
    state: &Entity<ListState<SampleListDelegate>>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let native = old_native(cx);
    let mut list_info = info::data::list(
        state.read(cx).delegate().items.len(),
        native.as_ref().map(|n| n.resolved),
    );
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
    .debug_selector(move || selector.into())
}

/// What a `ListItem` row is marked as.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ListRowState {
    Idle,
    Selected,
    /// Right-clicked and not selected: the `selection` outline and no fill
    /// (list/list_item.rs:265-274).
    RightClicked,
    /// Selected and right-clicked: the selected fill with the outline over
    /// it (list/list_item.rs:257-274).
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
    /// Whether the row is the List's first and its last: there its fill meets
    /// the frame (`at_edges`).
    edges: [bool; 2],
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
            edges: [false; 2],
        }
    }

    /// The row as the List's `first` and its `last`: its fill is rounded by
    /// the frame's inner radius (`list.border.corner_radius` less
    /// `list.border.line_width`) on its top corners where it is the first and
    /// on its bottom ones where it is the last, where it meets
    /// `geometry::list`'s frame, as the egui and iced Basic lists round
    /// theirs. GPUI Fast clips the frame's children to its curved padding
    /// edge (gpui-fast 0.1.15 src/fast/rounded_clip.rs:68-104), so a square
    /// fill would be cut at the curve; gpui-pre clips them to its rectangle
    /// (gpui-pre 0.3.12 src/window.rs:2453-2461, ContentMask), so a square
    /// fill would show past the curve there. The List does not tell a row
    /// where its view is scrolled to, so a row cut by the view's edge
    /// mid-scroll keeps its square corners.
    pub(crate) fn at_edges(mut self, first: bool, last: bool) -> Self {
        self.edges = [first, last];
        self
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

/// The colour a List or Tree row under the pointer letters its label in:
/// `list.hover_text_color`, with its config line recorded in `row_info`.
/// The ListItem's own hover sets its fill alone, inside render
/// (list/list_item.rs:231), so the label takes the colour from the row's
/// group hover (`row_label`). `None` on a selected row, which does not hover
/// (:228-229), and where no native theme is installed.
fn list_hover_text(cx: &App, selected: bool, row_info: &mut WidgetInfo) -> Option<Hsla> {
    let hover_text = (!selected)
        .then(|| native_color(cx, |n| n.resolved.list.hover_text_color))
        .flatten();
    if hover_text.is_some() {
        *row_info = std::mem::take(row_info).config(
            "hovered text",
            "list.hover_text_color, which the showcase sets on the label while the row is hovered",
        );
    }
    hover_text
}

/// The outline a List or Tree row under the pointer is edged in:
/// `list.hover_border_color` at `defaults.border.line_width`, where `shown`,
/// the theme states it and the width is above 0, with its colour claim and
/// config line recorded in `row_info`. A ListItem's own hover sets its fill
/// alone (list/list_item.rs:229-235), so the outline is a box of the
/// showcase's over the row ([`hover_edge`]). `None` where no native theme
/// is installed.
fn list_hover_edge(cx: &App, shown: bool, row_info: &mut WidgetInfo) -> Option<(Hsla, Pixels)> {
    let n = old_native(cx)?;
    let width = n.resolved.defaults.border.line_width;
    let claim = info::edge_claim(
        "hovered edge",
        "hover_border_color",
        n.resolved.list.hover_border_color.filter(|_| shown),
        Some(width),
        "showcase",
    )?;
    let edge = claim.value;
    *row_info = std::mem::take(row_info)
        .color(claim)
        .config(
            "hovered edge",
            format!(
                "list.hover_border_color at defaults.border.line_width, {}px, a box the showcase lays over the row while it is hovered, on the shape of its hover fill",
                info::px_text(width)
            ),
        );
    Some((edge, px(width)))
}

/// A box laid over the whole of a row in `group`, for the row's last child:
/// `width` wide, transparent at rest (upstream's own `transparent` token, as
/// `combo_surface` takes it) and edged `edge` while the group is under the
/// pointer. Its group-hover style gives it a `Normal` hitbox, which leaves
/// the row's own mouse handling alone (`combo_surface`).
fn hover_edge(cx: &App, edge: Hsla, width: Pixels, group: &'static str) -> Div {
    div()
        .absolute()
        .inset_0()
        .border(width)
        .border_color(cx.theme().transparent)
        .group_hover(group, move |style| style.border_color(edge))
}

/// A List or Tree row's `ListItem` after `geometry::list_item`: lettered by
/// `selected_text` where the row is selected, clear of the scrollbar its view
/// lays over the rows, and the `LIST_ROW_GROUP` its label's hover colour
/// follows.
///
/// Clear of the scrollbar where the theme's is not an overlay: the List and
/// the Tree draw their bar over their rows (list/list.rs, `List::render`;
/// tree.rs, `RenderOnce for Tree`), where a scrollbar that is not an overlay
/// takes the room beside them (`scrollbar.overlay_mode`, its `groove_width`).
fn list_row_item(item: ListItem, cx: &App, selected_text: Option<&StyleRefinement>) -> ListItem {
    refined(item, selected_text)
        .when_some(list_gutter(cx), |item, gutter| item.mr(gutter))
        .group(LIST_ROW_GROUP)
}

/// A List or Tree row's label, reading `label`, lettered in `hover_text`
/// while its row (`LIST_ROW_GROUP`) is under the pointer. The self-tests
/// read the colour it is laid out in through [`probed_label`].
///
/// Plain text, not a Label: `Label::render` paints foreground on its own
/// element (label.rs:213) over the list font the row carries. It has an id
/// because a text takes its colour as it is laid out, and in layout the
/// group hover is known only from the hover state an element with an id
/// keeps between frames: the group's hitbox is made in paint (gpui-pre
/// src/elements/div.rs, `Interactivity::compute_style_internal` and
/// `GroupHitboxes`), so a label without an id is laid out unhovered.
fn row_label(label: impl IntoElement, hover_text: Option<Hsla>) -> Stateful<Div> {
    div()
        .id("label")
        .child(label)
        .when_some(hover_text, |label, text| {
            label.group_hover(LIST_ROW_GROUP, move |style| style.text_color(text))
        })
}

/// `label`, the row `id`'s, recording the colour its text is laid out in
/// for the self-tests (`elements::label_probe`).
#[cfg(test)]
fn probed_label(label: Stateful<Div>, ui: &Entity<InfoRegistry>, id: &str) -> Stateful<Div> {
    label.child(crate::elements::label_probe(ui, id.to_string()))
}

/// The room a List's or a Tree's row leaves for the scrollbar: its groove,
/// where the theme's scrollbar is not an overlay
/// (`geometry::scrollbar_gutter`'s width); `None` for an overlay, or before
/// `apply` ran.
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
        let styled = old_native(cx).is_some();
        let mut row_info =
            info::data::list_row(cx.theme(), &self.label, state, styled, list_gutter(cx));
        // A selected row's text in `list.selection_text_color`, over the
        // list font's colour `geometry::list_item` gives every row: the row
        // refines itself with its style after its own `foreground`
        // (list/list_item.rs, `RenderOnce for ListItem`).
        let selected_text = self
            .selected
            .then(|| native_geometry(cx, geometry::list_item_selected))
            .flatten();
        if selected_text.is_some() {
            row_info = row_info.geometry("list_item_selected");
        }
        // Not on a right-clicked row, which does not hover either
        // (list/list_item.rs, `ListItem::render`).
        let hover_text = (!self.right_clicked)
            .then(|| list_hover_text(cx, self.selected, &mut row_info))
            .flatten();
        let edge = list_hover_edge(cx, !self.selected && !self.right_clicked, &mut row_info);
        let inner = native_value(cx, |n| list_inner_radius(&n.resolved.list.border));
        let [first, last] = self.edges;
        let item = native_info(
            ListItem::new(SharedString::from(id.clone())),
            cx,
            geometry::list_item,
            "list_item",
            &mut row_info,
        )
        .map(|item| list_row_item(item, cx, selected_text.as_ref()))
        .when_some(inner.filter(|_| first), |item, r| {
            item.rounded_tl(r).rounded_tr(r)
        })
        .when_some(inner.filter(|_| last), |item, r| {
            item.rounded_bl(r).rounded_br(r)
        })
        // The outline on the row's corners: `ListItem` puts the suffix into
        // its `relative()` root (list/list_item.rs, `RenderOnce for
        // ListItem`), as upstream lays a right-clicked row's outline.
        .when_some(edge, |item, (edge, width)| {
            item.suffix(move |_window, cx| {
                hover_edge(cx, edge, width, LIST_ROW_GROUP)
                    .when_some(inner.filter(|_| first), |b, r| {
                        b.rounded_tl(r).rounded_tr(r)
                    })
                    .when_some(inner.filter(|_| last), |b, r| b.rounded_bl(r).rounded_br(r))
            })
        })
        .child({
            // The text in a box of its own extent, which the layout dump
            // records where the row is one of the list's
            // (`elements::list_row_text`): in a flex row, so it is as wide
            // as its text, not the row.
            let text = div().relative().child(self.label).children(
                elements::list_row_text(self.prefix, self.ix)
                    .map(|listed| elements::record(&self.ui, listed)),
            );
            let label = row_label(h_flex().child(text), hover_text);
            #[cfg(test)]
            let label = probed_label(label, &self.ui, &id);
            label
        })
        .selected(self.selected)
        .secondary_selected(self.right_clicked);
        item.info(&self.ui, SharedString::from(id.clone()), row_info)
            .w_full()
            .debug_selector(move || id)
    }
}

/// A box `width` by `height` around the Tree over `state`, refined by
/// `geometry::list`, which frames it and fills it with
/// `list.background_color`. A tree is a list view and the model gives it no
/// theme of its own, so its frame and fill are the list's; the rows report
/// themselves (`tree_row`).
pub(crate) fn tree(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<TreeState>,
    width: Pixels,
    height: Pixels,
) -> Stateful<Div> {
    let native = old_native(cx);
    let mut tree_info = info::data::tree(native.as_ref().map(|n| n.resolved));
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
    .debug_selector(|| TREE_DEMO.into())
}

/// Row `ix` of the Tree, reading `label`, `selected` or not: a `ListItem`
/// refined by `geometry::list_item` and laid out as a List's row is
/// (`list_row_item`, `row_label`).
///
/// `Tree::new` takes a closure that returns the `ListItem` itself and wraps
/// it in a box of its own (tree.rs:42-45, 98-104), so nothing can be put
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
    let styled = old_native(cx).is_some();
    let mut row_info = info::data::tree_row(cx.theme(), &label, selected, styled, list_gutter(cx));
    // As a List row's: a selected row's text in `list.selection_text_color`,
    // a hovered one's in `list.hover_text_color`.
    let selected_text = selected
        .then(|| native_geometry(cx, geometry::list_item_selected))
        .flatten();
    if selected_text.is_some() {
        row_info = row_info.geometry("list_item_selected");
    }
    // The Tree marks a row right-clicked only after this closure built it
    // (tree.rs, `RenderOnce for Tree`), and gives the closure no way to
    // learn it, so unlike a List row's this label and outline cannot leave
    // out a right-clicked row, which does not hover.
    let hover_text = list_hover_text(cx, selected, &mut row_info);
    let edge = list_hover_edge(cx, !selected, &mut row_info);
    let item = native_info(
        ListItem::new(SharedString::from(id.clone())),
        cx,
        geometry::list_item,
        "list_item",
        &mut row_info,
    )
    .map(|item| list_row_item(item, cx, selected_text.as_ref()));
    let ui = ui.clone();
    let label = row_label(label, hover_text);
    #[cfg(test)]
    let label = probed_label(label, &ui, &id);
    item.child(label)
        .selected(selected)
        .suffix(move |_window, cx| {
            let id = id.clone();
            // The outline is a child of the wrapper `info` makes, which
            // the size is set on: the bare `div()` inside it has no height.
            div()
                .info(&ui, SharedString::from(id.clone()), row_info.clone())
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .when_some(edge, |target, (edge, width)| {
                    target.child(hover_edge(cx, edge, width, LIST_ROW_GROUP))
                })
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
    // list's bookkeeping (message_scroller.rs:20-22).
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
        .debug_selector(move || id.into())
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
    let mut button_info = info::buttons::button(
        cx.theme(),
        ButtonKind::Default,
        ButtonState::Idle,
        false,
        false,
        None,
        button_theme(cx),
    )
    .instance("click", about);
    let button = labelled(
        ui,
        cx,
        next_native_info(
            Button::new(id),
            cx,
            next::geometry::button,
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
        .media(AttachmentMedia::new().children(icon.glyph_sized(cx, geometry::icon_size_small)))
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
    /// The icon upstream gives the severity, an Alert's and a
    /// Notification's alike (alert.rs, Alert::info and the other three;
    /// notification.rs, NotificationType::icon).
    pub(crate) fn upstream_icon(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Success => "CircleCheck",
            Self::Warning => "TriangleAlert",
            Self::Error => "CircleX",
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

/// A `Progress` bar at `value` percent, or loading (indeterminate) where
/// `value` is `None`, refined by `geometry::progress`; `label` names it in
/// its info, as the page's label beside it does.
pub(crate) fn progress(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    value: Option<f32>,
) -> Stateful<Div> {
    #[cfg(feature = "widgets")]
    if let Some(r) = old_resolved(cx) {
        return widgets::ProgressBar::new(id)
            .value(value.unwrap_or_default())
            .loading(value.is_none())
            .accessibility_label(label)
            .when_some(elements::part_observer(ui, id), |bar, observer| {
                bar.on_part_bounds(observer)
            })
            .info(
                ui,
                id,
                info::feedback::native_progress(r, label, value, cx.reduce_motion()),
            )
            .debug_selector(move || id.into());
    }
    let mut bar_info = info::feedback::progress(cx.theme(), label, value, cx.reduce_motion());
    native_info(
        Progress::new(id)
            .value(value.unwrap_or_default())
            .loading(value.is_none()),
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

    /// The `Size` this kind is drawn at.
    pub(crate) fn size(self) -> Size {
        match self {
            Self::Small => Size::Small,
            Self::Medium => Size::Medium,
            Self::Large => Size::Large,
        }
    }
}

/// The icon set a theme-drawn Spinner draws its indicator from, as the
/// page's icons come: their set (`None` for gpui-component's own icons), the
/// freedesktop theme they load from (`None`: the system's), and whether the
/// set has an indicator (the Icons page's animated icons, loaded the same
/// way) and whether it is one icon that spins rather than frames.
#[derive(Clone, Default)]
#[cfg_attr(not(feature = "widgets"), allow(dead_code))]
pub(crate) struct SpinnerIcons {
    pub(crate) set: Option<IconSet>,
    pub(crate) theme: Option<SharedString>,
    pub(crate) indicator: bool,
    pub(crate) spins: bool,
}

/// A `Spinner` of `kind`; the theme-drawn one draws `icons`' indicator.
pub(crate) fn spinner(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: SpinnerKind,
    icons: &SpinnerIcons,
) -> Stateful<Div> {
    // The theme states one spinner, `spinner.*`: every size is drawn from
    // it, Medium at its diameter and Small and Large at gpui-component's own
    // ratios to that (`widgets::SpinnerLook::of_size`).
    #[cfg(feature = "widgets")]
    if let Some(r) = old_resolved(cx) {
        return widgets::Spinner::new(id)
            .with_size(kind.size())
            .icon_set(icons.set)
            .icon_theme(icons.theme.clone())
            .info(
                ui,
                id,
                info::feedback::native_spinner(r, kind, cx.reduce_motion(), icons),
            )
            .debug_selector(move || id.into());
    }
    #[cfg(not(feature = "widgets"))]
    let _ = icons;
    let (size, styled) = match kind {
        SpinnerKind::Small | SpinnerKind::Large => (kind.size(), false),
        SpinnerKind::Medium => match native_value(cx, geometry::spinner_size) {
            Some(size) => (size, true),
            None => (kind.size(), false),
        },
    };
    let native = old_native(cx);
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
    // The size `icon.sized` draws the icon at where a native theme is
    // installed.
    let large = native_value(cx, geometry::icon_size_large);
    let empty_info = info::feedback::empty(cx.theme(), title, description, icon, large);
    let empty_info = if icon.shown() && large.is_some() {
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
                        .children(icon.glyph_sized(cx, geometry::icon_size_large)),
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
/// it. Where `next` is given, the next Badge of the row follows its count
/// that far (the page's `layout.widget_gap`, none where it is unstated):
/// the Badge keeps that and the count's overhang
/// (`pages::badge_count_overhang`) free after it.
pub(crate) fn badge(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    count: Option<usize>,
    label: &'static str,
    next: Option<Option<Pixels>>,
) -> Stateful<Div> {
    let overhang = crate::pages::badge_count_overhang(count);
    let mut badge_info = info::feedback::badge(cx.theme(), count, label, overhang, next);
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
        .when_some(next, |badge, gap| {
            badge.mr(px(f32::from(overhang) + f32::from(gap.unwrap_or_default())))
        })
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
    /// Whether the Marker shows loading, a style of upstream's loading
    /// Marker rather than a variant of its own.
    pub(crate) const fn is_loading(self) -> bool {
        matches!(self, Self::Spinner | Self::Shimmer)
    }

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
        MarkerKind::Plain => match icon.glyph_sized(cx, geometry::icon_size_small) {
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
    text: impl Into<SharedString>,
) -> Stateful<Div> {
    let text = text.into();
    let mut tooltip_info = info::feedback::tooltip(
        cx.theme(),
        None,
        false,
        tooltip_delay(cx),
        gpui_base::TooltipDefaults::global(cx),
        label,
        &text,
    );
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

/// A Button of `kind` (as `ButtonKind::built` gives it) reading `label`,
/// refined by `geometry::button`, under a tooltip the application builds
/// itself, reading `text`.
///
/// `Button::tooltip` takes a string and builds the tooltip itself
/// (`button/button.rs:407`), so the only way to a refined one is to build
/// it: that is what `geometry::tooltip` documents, and the one place the
/// platform's tooltip padding, radius and text colour reach the popup. The
/// width is the content element's, through `Tooltip::element`; without a
/// key binding, as here, `Tooltip::new(text)` with the width on the popup is
/// the same bubble since gpui-component 0.10.0 (`geometry::tooltip`).
pub(crate) fn built_tooltip_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
    text: &'static str,
    kind: ButtonKind,
) -> Stateful<Div> {
    let kind = kind.built(cx);
    let style = native_geometry(cx, geometry::tooltip);
    let content = native_geometry(cx, geometry::tooltip_content);
    let delay = tooltip_delay(cx);
    let mut tooltip_info = info::feedback::tooltip(
        cx.theme(),
        Some(kind),
        style.is_some(),
        delay,
        gpui_base::TooltipDefaults::global(cx),
        label,
        text,
    );
    let button = native_info(
        kind.apply(Button::new(id), cx),
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
        .when_some(delay, |button, delay| button.tooltip_show_delay(delay))
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

/// Plain text reading `text` in the installed 0.8.0 theme's
/// `typography.monospace` ([`mono_sample`]).
pub(crate) fn mono_text(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: &'static str,
) -> Stateful<Div> {
    mono_sample(cx, text)
        .info(
            ui,
            id,
            info::typography::mono_text(cx.theme(), next_resolved(cx)),
        )
        .self_start()
        .debug_selector(move || id.into())
}

/// [`mono_text`]'s text: in the installed 0.8.0 theme's
/// `typography.monospace` ([`next_text_in`]) where one is installed and
/// states a font gpui can letter it in, else in the mono family alone,
/// gpui's own (spec §10, rule 0).
pub(crate) fn mono_sample(cx: &App, text: &'static str) -> Div {
    next_resolved(cx)
        .and_then(|r| {
            next_text_in(
                &r.typography,
                &r.typography.monospace,
                "typography.monospace",
                text,
            )
        })
        .unwrap_or_else(|| {
            div()
                .font_family(cx.theme().mono_font_family.clone())
                .child(text)
        })
}

/// [`text_in`] on the 0.8.0 model: plain text reading `text` in `style`, one
/// of `typography`'s text styles, which the theme's text scale has already
/// scaled: its family, size, weight and slant, and its line box, the style's
/// `line_height` where stated, else its size by
/// `typography.line_height_multiplier` (spec §4, `Typography`; §5, "A text
/// part's `line_height`"). `None` where `style`, read at its home `home`,
/// gives no font gpui can letter it in (the connector's check,
/// `next::label_font_unmapped`).
fn next_text_in(
    typography: &native_theme::next::Typography,
    style: &native_theme::next::TextStyle,
    home: &'static str,
    text: &'static str,
) -> Option<Div> {
    use native_theme::next::FontStyle as NextFontStyle;
    if next::label_font_unmapped(style, home).is_some() {
        return None;
    }
    let slant = match style.style {
        NextFontStyle::Normal => gpui::FontStyle::Normal,
        NextFontStyle::Italic => gpui::FontStyle::Italic,
        NextFontStyle::Oblique => gpui::FontStyle::Oblique,
        _ => return None,
    };
    let line = line_box(typography, style);
    let mut label = div()
        .font_family(native_theme_gpui::font_family(&style.family))
        .text_size(px(style.size_px))
        .line_height(px(line))
        .font_weight(FontWeight(f32::from(style.weight)))
        .child(text);
    label.text_style().font_style = Some(slant);
    Some(label)
}

/// A `Link` reading `text` that opens `href`.
pub(crate) fn link(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    text: &'static str,
    href: &'static str,
) -> Stateful<Div> {
    // Where `next::geometry::link` refines it: the underline it leaves or
    // removes at rest, and one line of `link.label(NONE)` tall.
    let refined = next_value(cx, |r| {
        let label = r.link.label(WidgetState::NONE);
        next::geometry::link(r).ok().map(|_| {
            (
                label.decoration.kind == native_theme::next::DecorationKind::Underline,
                px(label.size_px * r.typography.line_height_multiplier),
            )
        })
    })
    .flatten();
    let mut link_info = info::typography::next_link(cx.theme(), refined.map(|(u, _)| u))
        .instance("text", text)
        .instance("target", href);
    next_native_info(
        Link::new(id).child(text).href(href),
        cx,
        next::geometry::link,
        "link",
        &mut link_info,
    )
    .when_some(refined, |link, (_, line)| link.line_height(line))
    .info(ui, id, link_info)
    .debug_selector(move || id.into())
}

/// A `Kbd` for `keys`, in gpui's keystroke syntax, `secondary` resolved
/// with gpui's runtime OS (gpui-pre keystroke.rs:124-130); `None`, with one
/// stderr line, where they do not parse.
pub(crate) fn kbd(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    keys: &'static str,
) -> Option<Stateful<Div>> {
    let stroke = match Keystroke::parse_with_os(keys, cx.platform_info().os) {
        Ok(stroke) => stroke,
        Err(error) => {
            eprintln!("showcase: the keys {keys:?} did not parse: {error:?}");
            return None;
        }
    };
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
    // Refined by `next::geometry::editor`, the platform's line height, where
    // a 0.8.0 theme is installed, the info then given the builder's line.
    let mut editor_info = info::typography::editor(cx.theme());
    next_native_info(
        Editor::new(state),
        cx,
        next::geometry::editor,
        "editor",
        &mut editor_info,
    )
    .h(height)
    .info(ui, id, editor_info)
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

/// A `Diff` over `state`, `height` tall, its hovered line and line number
/// highlighted, and one line decorated in upstream's default background.
/// `Diff` is `Styled` and refines its frame last, so the height stands.
pub(crate) fn diff(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<DiffState>,
    height: Pixels,
) -> Stateful<Div> {
    Diff::new(state)
        .hover_highlight(DiffHoverHighlight::Both)
        .decorations([DiffDecoration::new(DiffLineRange::new(
            DIFF_DECORATED_PATH,
            DiffSide::Modified,
            DIFF_DECORATED_LINE,
            DIFF_DECORATED_LINE,
        ))])
        .h(height)
        .info(ui, id, info::typography::diff(cx.theme()))
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
                box_info.next_geometry("window_margin")
            } else {
                box_info
            };
            let box_info = if gap.is_some() {
                box_info.next_geometry("section_gap")
            } else {
                box_info
            };
            (v_flex(), box_info)
        }
        SpacingBox::Container => {
            let box_info = if padding.is_some() {
                box_info.next_geometry("container_margin")
            } else {
                box_info
            };
            let box_info = if gap.is_some() {
                box_info.next_geometry("widget_gap")
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

/// Without the connector's `widgets` feature, the one connector widget the
/// showcase draws its own lines with, the menu separator and the table's grid
/// lines (M9-165): a stand-in for `native_theme_gpui::widgets::Separator`
/// drawn as that one draws (connectors/native-theme-gpui/src/widgets/separator.rs,
/// `RenderOnce for Separator`), a box filled in `separator.line_color` (or
/// the colour a caller's `bg` gives), `separator.line_width` thick and at
/// least one device pixel, which every gpui renderer draws; gpui-component's
/// `Separator` without a native theme or with a width that is not finite.
#[cfg(not(feature = "widgets"))]
mod widgets {
    use gpui::{
        AnyElement, App, Axis, InteractiveElement as _, IntoElement, RenderOnce, StyleRefinement,
        Styled, Window, div, px,
    };
    use gpui_component::StyledExt as _;

    /// A line across its container, as the connector's `Separator`.
    #[derive(IntoElement)]
    pub(crate) struct Separator {
        axis: Axis,
        style: StyleRefinement,
    }

    impl Separator {
        /// A line across the container's width.
        pub(crate) fn horizontal() -> Self {
            Self {
                axis: Axis::Horizontal,
                style: StyleRefinement::default(),
            }
        }

        /// A line down the container's height.
        pub(crate) fn vertical() -> Self {
            Self {
                axis: Axis::Vertical,
                style: StyleRefinement::default(),
            }
        }
    }

    impl Styled for Separator {
        fn style(&mut self) -> &mut StyleRefinement {
            &mut self.style
        }
    }

    impl RenderOnce for Separator {
        fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
            let width = crate::support::native_value(cx, |n| n.resolved.separator.line_width)
                .filter(|w| w.is_finite() && *w >= 0.0);
            let color = crate::support::native_color(cx, |n| n.resolved.separator.line_color);
            let look = color.zip(width.map(px));
            let Some((color, width)) = look else {
                return match self.axis {
                    Axis::Horizontal => gpui_component::separator::Separator::horizontal(),
                    Axis::Vertical => gpui_component::separator::Separator::vertical(),
                }
                .refine_style(&self.style)
                .into_any_element();
            };
            let thickness = width.max(px(1.0 / window.scale_factor()));
            let line = div().flex_none().bg(color);
            let line: AnyElement = match self.axis {
                Axis::Horizontal => line.w_full().h(thickness),
                Axis::Vertical => line.h_full().w(thickness),
            }
            .refine_style(&self.style)
            .debug_selector(|| "native-separator".into())
            .into_any_element();
            line
        }
    }
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

/// The room above and below a horizontal Separator sample, which leaves
/// something to point at: the showcase's own, gpui's `py_1`, as the theme
/// states none (spec §3.1). The connector's plain line, under a native theme
/// with `widgets`, takes it only where it is no element of
/// docs/showcase-elements.toml; upstream's line -- a labelled or dashed
/// one, or any without a native theme or `widgets` -- takes it always. The
/// Separator's info says so.
pub(crate) const SEPARATOR_SAMPLE_ROOM: Rems = rems(0.25);

/// A `Separator` of `kind`, its line in the installed 0.8.0 theme's
/// `separator.color` through `Separator::color`.
pub(crate) fn separator(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: SeparatorKind,
) -> Stateful<Div> {
    // The plain line under a 0.8.0 theme is the connector's: its thickness
    // is `separator.thickness`, which gpui-component's literal 1px line does
    // not take (separator.rs:78-82).
    #[cfg(feature = "widgets")]
    if kind == SeparatorKind::Horizontal
        && let Some(look) = next_resolved(cx).and_then(|r| widgets::SeparatorLook::next_of(r).ok())
    {
        // A line of docs/showcase-elements.toml takes no room beyond its
        // own, as the iced and egui showcases lay it out; the others keep
        // some to point at.
        let listed = elements::listed_for(id).is_some();
        return widgets::Separator::horizontal()
            .info(
                ui,
                id,
                info::layout::native_separator(cx.theme(), &look, !listed),
            )
            .when(!listed, |separator| separator.py(SEPARATOR_SAMPLE_ROOM))
            .debug_selector(move || id.into());
    }
    let separator = match kind {
        SeparatorKind::Horizontal => Separator::horizontal(),
        SeparatorKind::Labelled(label) => Separator::horizontal().label(label),
        SeparatorKind::Dashed => Separator::horizontal_dashed(),
        SeparatorKind::Vertical => Separator::vertical(),
    };
    let native = next_resolved(cx);
    let line = native.map(|r| info::next_stated(r.separator.color));
    separator
        .when_some(line, |separator, line| separator.color(line))
        .info(ui, id, info::layout::separator(cx.theme(), kind, native))
        // A horizontal Separator's box is as tall as its label and no
        // taller: the line is an absolute child (separator.rs:79-84). The
        // padding leaves something to point at. A vertical one is as tall
        // as its row: its box is `h_full` of the box that reports it
        // (separator.rs:28), which stretches to the row.
        .map(|separator| match kind {
            SeparatorKind::Vertical => separator.self_stretch(),
            _ => separator.py(SEPARATOR_SAMPLE_ROOM),
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
/// `next::geometry::group_box_content`, which sets the sides
/// `card.container(NONE)` states, and padded with `container_margin`, the
/// installed 0.8.0 theme's `layout.container_margin`, on a side it leaves
/// unstated ([`next_padded`]). A group box is a container, and
/// `layout.container_margin` is the padding inside containers
/// (platform-facts §2.20): it stands in for a Fill or Outline GroupBox's own
/// `p_4` and a Normal one's none (group_box.rs, `GroupBox`), as it does in a
/// card. Where the theme states no value gpui can take at a home the builder
/// reads, the content keeps upstream's look and padding (spec §10, rule 0).
#[allow(clippy::too_many_arguments)]
pub(crate) fn group_box(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    kind: GroupBoxKind,
    title: &'static str,
    content: impl IntoElement,
    about: &'static str,
    container_margin: Option<Pixels>,
) -> Stateful<Div> {
    // The theme the builder refines the content from, where it does.
    let native = next_resolved(cx).filter(|r| next::geometry::group_box_content(r).is_ok());
    let mut box_info =
        info::layout::next_group_box(cx.theme(), kind, native, container_margin, title)
            .instance("content", about);
    // `GroupBox::content_style` takes a refinement rather than being one.
    let content_style = next_native_info(
        StyleRefinement::default(),
        cx,
        next::geometry::group_box_content,
        "group_box_content",
        &mut box_info,
    );
    let content_style = match (native, container_margin) {
        (Some(r), Some(margin)) => {
            let sides = next::padding_of(&r.card.container(WidgetState::NONE));
            if sides.iter().any(Option::is_none) {
                box_info = box_info.next_geometry("container_margin");
            }
            next_padded(content_style, sides, Some(margin), Some(margin))
        }
        _ => content_style,
    };
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
/// by `geometry::group_box_content`, which fills it with
/// `card.background_color` over the Fill variant's `group_box` token
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
    let native = old_native(cx);
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
    let content_style = match (&native, container_margin) {
        (Some(n), Some(margin)) => {
            let padding = &n.resolved.card.border.padding;
            if leaves_a_side_unstated(padding) {
                card_info = card_info.geometry("container_margin");
            }
            padded(content_style, padding, Some(margin), Some(margin))
        }
        _ => content_style,
    };
    GroupBox::new()
        .with_variant(GroupBoxVariant::Fill)
        .content_style(content_style)
        .w(width)
        .child(body_label(ui, cx, text_id, text))
        .info(ui, id, card_info)
        .debug_selector(move || id.into())
}

/// The space between the scroll area's sample labels, and the padding
/// round them: the showcase's own, gpui's `gap_2` and `p_3`, as the theme
/// states no such column (spec §3.1). The area's info says so.
pub(crate) const SCROLL_SAMPLE_GAP: Rems = rems(0.5);
pub(crate) const SCROLL_SAMPLE_PADDING: Rems = rems(0.75);

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
    // Whether the connector wrote the installed 0.8.0 theme's scroll bar onto
    // gpui-base (`next::base_layer::scrollbar_geometry`).
    let styled =
        next_value(cx, |r| next::base_layer::scrollbar_geometry(r).is_ok()).unwrap_or(false);
    let mut area_info = info::layout::scroll_area(cx.theme(), styled, items);
    let viewport = next_native_info(
        div()
            .id(SharedString::from(format!("{id}-viewport")))
            .size_full()
            .debug_selector(move || format!("{id}-viewport"))
            .overflow_y_scrollbar(),
        cx,
        next::geometry::scrollbar_gutter,
        "scrollbar_gutter",
        &mut area_info,
    )
    .child(
        v_flex()
            .gap(SCROLL_SAMPLE_GAP)
            .p(SCROLL_SAMPLE_PADDING)
            .children((1..=items).map(|n| {
                label(
                    ui,
                    cx,
                    format!("{id}-item-{n}"),
                    format!("Scrollable item #{n} - demonstrates scrollbar theming"),
                )
            })),
    );
    div()
        .h(height)
        .w_full()
        .demo_frame(cx)
        .debug_selector(move || format!("{id}-frame"))
        .child(viewport)
        .info(ui, id, area_info)
        .debug_selector(move || id.into())
}

/// The Accordion of the Layout page: an item per `(title, answer id,
/// answer)`, the first open, their title rows refined by
/// `next::geometry::accordion_title`. Each answer is a small Label that
/// reports itself.
pub(crate) fn accordion(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    items: [(&'static str, &'static str, SharedString); 3],
) -> Stateful<Div> {
    // Which of the title rows and the edges the builders refine, where a
    // 0.8.0 theme is installed.
    let parts = next_value(cx, |r| {
        (
            next::geometry::accordion_title(r).is_ok(),
            next::geometry::accordion(r).is_ok(),
        )
    });
    let mut accordion_info =
        info::layout::next_accordion(cx.theme(), cx.reduce_motion(), items.len(), parts);
    // `AccordionItem::title_style` takes a refinement rather than being one.
    let title_style = next_native_info(
        StyleRefinement::default(),
        cx,
        next::geometry::accordion_title,
        "accordion_title",
        &mut accordion_info,
    );
    let edge = next_geometry(cx, next::geometry::accordion).and_then(Result::ok);
    let hover = info::next_native_value(
        cx,
        next::geometry::accordion_hover,
        "accordion_hover",
        &mut accordion_info,
    );
    items
        .into_iter()
        .enumerate()
        .fold(
            // The edge colour lands on the Accordion's card and on each
            // item's line (accordion.rs:108-114, :380-384).
            next_native_info(
                Accordion::new(id),
                cx,
                next::geometry::accordion,
                "accordion",
                &mut accordion_info,
            ),
            |accordion, (ix, (title, answer_id, answer))| {
                accordion.item(|item| {
                    let item = refined(item, edge.as_ref())
                        .title_style(title_style.clone())
                        .title(title)
                        .open(ix == 0)
                        .child(label(ui, cx, answer_id, answer));
                    match hover.clone() {
                        Some(hover) => item.hover(move |_| hover),
                        None => item,
                    }
                })
            },
        )
        .info(ui, id, accordion_info)
        .debug_selector(move || id.into())
}

/// The Basic page's two expanders, as `spec` describes them: an item per
/// `(title, body id, body)`, open as `open` says; a click on a title hands
/// `on_toggle` which are open after it. Each body is a body-text Label that
/// reports itself. Under a native theme the showcase draws them as
/// `expander.*` states them ([`native_expander`]); without one they are
/// upstream's Accordion, an item each.
pub(crate) fn expander(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: ExpanderSpec,
    on_toggle: impl Fn(&[bool; 2], &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    if let Some(n) = old_native(cx) {
        return native_expander(ui, cx, &n, spec, Rc::new(on_toggle));
    }
    let ExpanderSpec {
        id,
        items,
        open,
        width,
        ..
    } = spec;
    let expander_info = info::layout::expander(
        cx.theme(),
        cx.reduce_motion(),
        items.map(|(title, _, _)| title),
        open,
    );
    let accordion = Accordion::new(id)
        .w(width)
        .on_toggle_click(move |open, window, cx| {
            on_toggle(&[open.contains(&0), open.contains(&1)], window, cx)
        });
    items
        .into_iter()
        .zip(open)
        .fold(accordion, |accordion, ((title, body_id, body), open)| {
            accordion.item(move |item| {
                item.title(title)
                    .open(open)
                    .child(body_label(ui, cx, body_id, body))
            })
        })
        .info(ui, id, expander_info)
        .debug_selector(move || id.into())
}

/// What [`expander`] draws: the expander `id`, its two items as (title,
/// body id, body), whether each is open, its width, `layout.widget_gap`,
/// which parts a theme-drawn expander's items and a title from its body
/// where the theme states its frame, and the chevrons of the chosen icon
/// theme its arrows are drawn with.
pub(crate) struct ExpanderSpec {
    pub id: &'static str,
    pub items: [(&'static str, &'static str, &'static str); 2],
    pub open: [bool; 2],
    pub width: Pixels,
    pub gap: Option<Pixels>,
    pub arrows: ExpanderArrows,
}

/// The chevrons a theme-drawn expander's arrow is drawn with, of the chosen
/// icon theme: ChevronRight for an arrow before the title, ChevronDown for
/// one after it.
pub(crate) struct ExpanderArrows {
    pub leading: SampleIcon,
    pub trailing: SampleIcon,
    /// The arrows of an open item where the set gives its chevrons as
    /// pixels, or SVGs in several colours, drawn as images, which do not
    /// turn (`gpui::img` takes no transformation):
    /// ChevronDown for a leading arrow, ChevronUp for a trailing one, the
    /// directions a turned chevron points in.
    pub leading_open: SampleIcon,
    pub trailing_open: SampleIcon,
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

/// How far an expander item's arrow is turned, of a whole turn: an open
/// item's `open_turn` where its arrow is the closed one's chevron turned, none
/// where it is closed or where the closed chevron is pixels, which do not
/// turn: there an open item shows the set's chevron of the direction a turned
/// one points in, already pointing so, whether that is pixels or an SVG (a set
/// that gives one chevron as pixels and the other as an SVG).
pub(crate) fn expander_arrow_turn(closed_is_raster: bool, is_open: bool, open_turn: f32) -> f32 {
    if is_open && !closed_is_raster {
        open_turn
    } else {
        0.
    }
}

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
/// theme states no side the arrow is upstream's, after the title. The arrow
/// is a chevron of the chosen icon theme, none where that theme has none: a
/// trailing arrow ChevronDown, turned while the item is open, as upstream's,
/// and a leading one ChevronRight turned down while it is open, as Breeze
/// and the HIG draw a leading arrow (§2.27). Upstream builds its arrow, its
/// glyph, size and colour, inline, where no caller reaches them
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
        arrows,
    } = spec;
    let e = &n.resolved.expander;
    let colour = info::stated;
    let line = colour(e.border.color);
    let line_width = px(e.border.line_width);
    let text = colour(e.font.color);
    let arrow = colour(e.arrow_color.unwrap_or(e.font.color));
    let hover = e.hover_background.map(colour);
    let title_style = geometry::accordion_title(*n);
    let leading = e.arrow_side == Some(ArrowSide::Leading);
    // A leading arrow points at the title while closed and down while open;
    // a trailing one down while closed and up while open (§2.27,
    // `arrow_side`).
    let (chevron, turned, open_turn) = if leading {
        (&arrows.leading, &arrows.leading_open, 0.25)
    } else {
        (&arrows.trailing, &arrows.trailing_open, 0.5)
    };
    let sized = |icon: &SampleIcon| {
        icon.glyph().map(|glyph| {
            glyph
                .with_size(Size::Size(px(e.arrow_icon_size)))
                .text_color(arrow)
        })
    };
    let glyph = sized(chevron);
    // Pixels do not turn: an open item's arrow is then the set's chevron of
    // the direction a turned one points in.
    let open_glyph = if chevron.is_raster() {
        sized(turned)
    } else {
        glyph.clone()
    };
    let expander_info = info::layout::native_expander(
        n.resolved,
        items.map(|(title, _, _)| title),
        open,
        chevron,
        turned,
        gap,
    )
    .geometry("accordion_title");
    // KDE's expander has no frame and no line between items (§2.27); where
    // the theme states nothing the frame stays, as upstream's.
    let framed = e.frame_enabled != Some(false);
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
                let turn = expander_arrow_turn(chevron.is_raster(), is_open, open_turn);
                let listed = elements::expander_item(id, ix);
                let shown = if is_open { &open_glyph } else { &glyph };
                let arrow = shown.clone().map(|glyph| {
                    // Which set's image the arrow's glyph is, for the
                    // self-tests alone: an image's own selector names its
                    // icon alone, which the Number input's steps show too.
                    #[cfg(test)]
                    let shows = glyph.raster_name().map(|name| {
                        div()
                            .absolute()
                            .inset_0()
                            .debug_selector(move || format!("{id}-arrow-{ix}-{name}"))
                    });
                    #[cfg(not(test))]
                    let shows: Option<Div> = None;
                    div()
                        .relative()
                        .flex_none()
                        .debug_selector(move || format!("{id}-arrow-{ix}"))
                        .child(glyph.rotate(gpui::percentage(turn)))
                        .children(shows)
                        .children(listed.map(|[_, _, arrow, _]| elements::record(ui, arrow)))
                });
                let title = text_in(n, &e.font, title)
                    .relative()
                    .children(listed.map(|[_, _, _, title]| elements::record(ui, title)));
                let header = padded(
                    h_flex().id((id, ix)).w_full().items_center().gap(arrow_gap),
                    &e.border.padding,
                    Some(EXPANDER_PADDING_X),
                    None,
                )
                .refine_style(&title_style)
                .text_color(text)
                .when_some(hover, |header, hover| {
                    header.hover(move |style| style.bg(hover))
                })
                .debug_selector(move || format!("{id}-header-{ix}"))
                .children(listed.map(|[_, header, _, _]| elements::record(ui, header)))
                .map(|header| {
                    if leading {
                        header.children(arrow).child(title)
                    } else {
                        header.justify_between().child(title).children(arrow)
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

/// The padding round the Collapsible sample's content: the showcase's own,
/// gpui's `p_3`, as the theme states none for it (spec §3.1). The
/// Collapsible's info says so.
pub(crate) const COLLAPSIBLE_CONTENT_PADDING: Rems = rems(0.75);

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
    // `Button::icon` takes an `Icon` (`ButtonIcon`) alone (button/button.rs).
    let icon = icon.svg_only();
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
        .content(v_flex().p(COLLAPSIBLE_CONTENT_PADDING).child(label(
            ui,
            cx,
            content,
            "This content is shown when collapsible is open.",
        )))
        .info(ui, id, info::layout::collapsible(cx.theme(), open))
        .debug_selector(move || id.into())
}

/// The padding of a Carousel slide, and between its title and its caption:
/// the showcase's own, gpui's `p_4` and `gap_1`, as the theme states no
/// slide (spec §3.1). Each slide's info says so.
pub(crate) const CAROUSEL_SLIDE_PADDING: Rems = rems(1.);
pub(crate) const CAROUSEL_SLIDE_GAP: Rems = rems(0.25);

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
            let slide_id = SharedString::from(format!("{id}-slide-{}", ix.saturating_add(1)));
            let selector = slide_id.clone();
            CarouselItem::new((id, ix), ix, state).child(
                v_flex()
                    .size_full()
                    .justify_center()
                    .gap(CAROUSEL_SLIDE_GAP)
                    .p(CAROUSEL_SLIDE_PADDING)
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
        let page_id = SharedString::from(format!("{id}-page-{}", ix.saturating_add(1)));
        let selector = page_id.clone();
        let page = CarouselPaginationItem::new(page_id.clone(), ix, state)
            .child(SharedString::from(ix.saturating_add(1).to_string()))
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
        // `StepperItem::icon` takes an `Icon` alone (stepper/item.rs).
        StepperKind::Icons => STEPPER_STEPS
            .iter()
            .map(|(text, _)| *text)
            .zip(icons.iter().map(SampleIcon::svg_only))
            .collect(),
        StepperKind::Numbers => Vec::new(),
    };
    let mut stepper_info =
        info::layout::stepper(cx.theme(), kind, step, STEPPER_STEPS.len(), &labelled);
    // The indicator is a circle (stepper/trigger.rs:118-123) around the
    // icon it is given, which keeps its size (`:138-139`): the installed
    // 0.8.0 theme's `icons.sizes.small`, where it states one.
    let size = if labelled.iter().any(|(_, icon)| icon.shown()) {
        info::next_native_value(
            cx,
            next::geometry::icon_size_small,
            "icon_size_small",
            &mut stepper_info,
        )
    } else {
        None
    };
    let stepper = match kind {
        StepperKind::Icons => Stepper::new(id),
        StepperKind::Numbers => Stepper::new(id).vertical(),
    };
    stepper
        .selected_index(step)
        .items(STEPPER_STEPS.iter().enumerate().map(|(ix, (text, _))| {
            let item = StepperItem::new().child(label(
                ui,
                cx,
                format!("{id}-step-{}", ix.saturating_add(1)),
                *text,
            ));
            let icon = labelled.get(ix).and_then(|(_, icon)| icon.next_sized(size));
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

/// An `Input` over `state` in a Form's field, as wide as the field: the
/// Inputs page's refined, enabled field ([`text_field`]).
fn field_input(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<InputState>,
) -> Stateful<Div> {
    text_field(
        ui,
        cx,
        id,
        state,
        InputField::Refined,
        false,
        relative(1.).into(),
        false,
    )
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
    let gutter = next_native_info(
        StyleRefinement::default(),
        cx,
        next::geometry::scrollbar_gutter,
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

/// The Layout page's `GridLayout` over `state`, `width` wide: gpui-component's
/// `grid_layout`, its gap, motion, rings, placeholder and grip upstream's
/// (gpui-component grid_layout.rs:40-105), holding GRID_LAYOUT_ITEMS as cards
/// framed as the showcase frames every demonstration.
pub(crate) fn grid_layout_sample(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    state: &Entity<GridLayoutState>,
    width: Pixels,
) -> Stateful<Div> {
    grid_layout(id, state, cx)
        .w(width)
        .items(GRID_LAYOUT_ITEMS.map(|card| {
            GridItem::new(card.id).rounded(cx.theme().radius).child(
                div()
                    .size_full()
                    .demo_frame(cx)
                    .bg(cx.theme().background)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .child(card.label)
                    .debug_selector(move || card.id.into()),
            )
        }))
        .info(ui, id, info::layout::grid_layout(cx.theme()))
        .self_start()
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
    let mut button_info = info::overlays::trigger(cx.theme(), overlay, button_theme(cx));
    let button = labelled(
        ui,
        cx,
        next_native_info(
            overlay.button_kind().apply(Button::new(id), cx),
            cx,
            next::geometry::button,
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
    let styled = old_native(cx).is_some();
    let entrance = themed_entrance(cx);
    let mut dialog_info = info::overlays::dialog(
        cx.theme(),
        cx.reduce_motion(),
        entrance.is_some(),
        styled,
        icon,
    );
    let dialog = dialog_frame(dialog, entrance, cx, &mut dialog_info);
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
    let mut close_info = info::overlays::dialog_close(cx.theme(), button_theme(cx));
    // DialogClose builds the Button and hands it over
    // (dialog/footer.rs:83-88), so the refinement is made here and applied
    // there.
    let close_style = next_native_info(
        StyleRefinement::default(),
        cx,
        next::geometry::button,
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
                    .children(icon.glyph_sized(cx, geometry::icon_size_dialog))
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

/// The Overlays page's AlertDialog: `alert`, refined by `geometry::dialog`,
/// capped at `geometry::dialog_max_width` and entering as
/// `geometry::dialog_entrance` gives, with `icon`, of the chosen
/// icon theme -- none where that theme has none -- a title, a description,
/// and Keep and Discard buttons. It reports itself on the icon, the title
/// and the description, the parts the showcase builds.
pub(crate) fn alert_dialog(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    alert: AlertDialog,
    icon: &SampleIcon,
) -> AlertDialog {
    let styled = old_native(cx).is_some();
    let entrance = themed_entrance(cx);
    let mut alert_info = info::overlays::alert_dialog(
        cx.theme(),
        cx.reduce_motion(),
        entrance.is_some(),
        styled,
        icon,
    );
    let alert = native_info(alert, cx, geometry::dialog, "dialog", &mut alert_info);
    let alert = match entrance {
        Some(entrance) => {
            alert_info = alert_info.geometry("dialog_entrance");
            alert.entrance(entrance)
        }
        None => alert,
    };
    // An AlertDialog has no `max_w` of its own; `Styled::max_w` lands on the
    // surface through the refinement (dialog/dialog.rs:787).
    let alert = match native_value(cx, geometry::dialog_max_width) {
        Some(width) => {
            alert_info = alert_info.geometry("dialog_max_width");
            alert.max_w(width)
        }
        None => alert,
    };
    let drawn = icon.glyph_sized(cx, geometry::icon_size_dialog);
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
    let native = old_native(cx);
    let mut popover_info = info::overlays::popover(cx.theme(), native.as_ref().map(|n| n.resolved));
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
    let native = old_native(cx);
    let mut card_info = info::overlays::hover_card(cx.theme(), native.as_ref().map(|n| n.resolved));
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

/// The padding inside the Overlays page's ContextMenu area: the showcase's
/// own, gpui's `p_6`, as the theme states no such area (spec §3.1). The
/// area's info says so.
pub(crate) const CONTEXT_AREA_PADDING: Rems = rems(1.5);

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
        .p(CONTEXT_AREA_PADDING)
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

/// The debug selector of the Overlays page's frame of menu rows.
pub(crate) const MENU_ROWS_FRAME: &str = "overlays-menu-rows";

/// The menu rows an application draws itself, one per `(id, icon, label)`
/// of `rows`, in a frame painted as the showcase's menus are, from
/// [`MenuLook`]: `menu.background_color` edged by `popover.border` and
/// padded by the `menu.panel_padding_*` sides the theme states, the rows in
/// `menu.font.color`, hovered in `menu.hover_background` and
/// `hover_text_color` and outlined in `menu.hover_border_color`
/// ([`MenuLook::edge`]), pressed in `menu.active_background` and
/// `active_text_color` ([`MenuLook::pressed`]), rounded by
/// `menu.border.corner_radius`; without a native theme, the popover's tokens
/// and the accent. Each icon is the chosen icon theme's, and a row whose
/// icon that theme lacks shows its label alone. Upstream's `MenuItemElement` is crate-private and
/// `PopupMenu` builds its own rows, so `geometry::menu_item` has no widget
/// to refine: these rows are the receiver it documents.
pub(crate) fn menu_rows(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    rows: &[(&'static str, SampleIcon, &'static str)],
) -> Div {
    let t = cx.theme();
    let native = old_native(cx);
    let look = native.as_ref().map(MenuLook::of);
    let frame = v_flex().w(MENU_ROWS_WIDTH);
    let (frame, hover_bg, hover_text, radius) = match &look {
        Some(look) => (
            padded(
                frame
                    .bg(look.background)
                    .text_color(look.text)
                    .border(look.frame_width)
                    .border_color(look.frame)
                    .rounded(look.frame_radius)
                    .overflow_hidden(),
                &look.panel_padding,
                None::<Pixels>,
                None,
            ),
            look.hover,
            look.hover_text,
            Some(look.radius),
        ),
        None => (
            frame
                .bg(t.popover)
                .text_color(t.popover_foreground)
                .demo_frame(cx),
            t.accent,
            t.accent_foreground,
            None,
        ),
    };
    frame
        .debug_selector(|| MENU_ROWS_FRAME.into())
        .children(rows.iter().map(|(id, icon, label)| {
            let (id, label) = (*id, *label);
            let mut row_info =
                info::overlays::menu_row(t, native.as_ref().map(|n| n.resolved), label, icon);
            if icon.shown() && native_value(cx, geometry::icon_size_small).is_some() {
                row_info = row_info.geometry("icon_size_small");
            }
            // Plain text, not a Label: a Label paints foreground on its
            // own element (label.rs:213), over the frame's colour.
            let row = div()
                .id("row")
                .relative()
                .group(MENU_ROW_GROUP)
                .flex()
                .items_center()
                .when_some(radius, |row, radius| row.rounded(radius))
                .hover(move |this| this.bg(hover_bg).text_color(hover_text))
                .children(icon.glyph_sized(cx, geometry::icon_size_small))
                .child(label)
                .children(
                    look.as_ref()
                        .and_then(|look| look.edge(None, t.transparent)),
                );
            #[cfg(test)]
            let row = row.child(crate::elements::label_probe(ui, format!("{id}-label")));
            // Pressed before `info`, which wraps the row in an element of
            // its own: a style set after it would land on the wrapper,
            // under the row's own hover fill and text.
            native_info(row, cx, geometry::menu_item, "menu_item", &mut row_info)
                .map(|row| match &look {
                    Some(look) => look.pressed(row),
                    None => row,
                })
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

/// The name of the BarChart's series, its tooltip row's label.
pub(crate) const BAR_SERIES: &str = "Visitors";

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

/// The dimensions the RadarChart plots, as `(dimension, first series,
/// second series)`, clockwise from 12 o'clock.
pub(crate) const RADAR_DIMENSIONS: [(&str, f64, f64); 6] = [
    ("Speed", 80., 60.),
    ("Power", 65., 85.),
    ("Range", 90., 70.),
    ("Comfort", 55., 75.),
    ("Safety", 85., 80.),
    ("Price", 70., 50.),
];

/// The names of the RadarChart's two series, in the order they are added.
pub(crate) const RADAR_SERIES: [&str; 2] = ["Model A", "Model B"];

/// The value at the RadarChart's outer ring: the showcase's own, as the
/// model states no chart.
pub(crate) const RADAR_MAX_VALUE: f64 = 100.;

/// The strength the RadarChart's fills take their series colours at: the
/// showcase's own, as the model states no chart.
pub(crate) const RADAR_FILL_OPACITY: f32 = 0.3;

/// The SankeyChart's nodes, which take `chart_1` to `chart_5` in this
/// order.
pub(crate) const SANKEY_NODES: [&str; 5] = ["Solar", "Wind", "Grid", "Homes", "Industry"];

/// The SankeyChart's links, as `(source, target, value)`, the nodes by
/// their place in `SANKEY_NODES`.
pub(crate) const SANKEY_LINKS: [(usize, usize, f64); 4] =
    [(0, 2, 40.), (1, 2, 60.), (2, 3, 45.), (2, 4, 55.)];

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
        .name(BAR_SERIES)
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

/// The AreaChart's line and fill: `chart_3`, and `chart_3` at
/// `AREA_FILL_OPACITY`.
pub(crate) fn area_series(t: &gpui_component::theme::Theme) -> (Hsla, Hsla) {
    (t.chart_3, t.chart_3.opacity(AREA_FILL_OPACITY))
}

/// An `AreaChart` of `SAMPLE_MONTHS_AREA`: one series, its line and fill
/// [`area_series`].
pub(crate) fn area_chart(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str) -> Stateful<Div> {
    let t = cx.theme();
    let (stroke, fill) = area_series(t);
    AreaChart::new(SAMPLE_MONTHS_AREA)
        .x(|d: &(&'static str, f64)| d.0)
        .y(|d: &(&'static str, f64)| d.1)
        .stroke(stroke)
        .fill(fill)
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

/// The RadarChart's two series as `(line, fill)`: `chart_1` and `chart_2`,
/// each over a fill of itself at `RADAR_FILL_OPACITY`.
pub(crate) fn radar_series(t: &gpui_component::theme::Theme) -> [(Hsla, Hsla); 2] {
    [t.chart_1, t.chart_2].map(|line| (line, line.opacity(RADAR_FILL_OPACITY)))
}

/// A `RadarChart` of `RADAR_DIMENSIONS`: two series, their lines and fills
/// [`radar_series`].
pub(crate) fn radar_chart(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str) -> Stateful<Div> {
    type Dimension = (&'static str, f64, f64);
    let t = cx.theme();
    let [first, second] = RADAR_SERIES;
    let [(first_line, first_fill), (second_line, second_fill)] = radar_series(t);
    RadarChart::new(RADAR_DIMENSIONS)
        .label(|d: &Dimension| d.0)
        .value(|d: &Dimension| d.1)
        .stroke(first_line)
        .fill(first_fill)
        .name(first)
        .value(|d: &Dimension| d.2)
        .stroke(second_line)
        .fill(second_fill)
        .name(second)
        .max_value(RADAR_MAX_VALUE)
        .info(ui, id, info::charts::radar_chart(t))
        .h(CHART_HEIGHT)
        .w_full()
        .debug_selector(move || id.into())
}

/// A `SankeyChart` of `SANKEY_NODES` and `SANKEY_LINKS`, its nodes in
/// `chart_1` to `chart_5`.
pub(crate) fn sankey_chart(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str) -> Stateful<Div> {
    let t = cx.theme();
    let nodes: Vec<(&'static str, Hsla)> = SANKEY_NODES
        .into_iter()
        .zip([t.chart_1, t.chart_2, t.chart_3, t.chart_4, t.chart_5])
        .collect();
    let links = SANKEY_LINKS.map(|(source, target, value)| SankeyLink::new(source, target, value));
    SankeyChart::new(nodes, links)
        .node_color(|d: &(&'static str, Hsla)| d.1)
        .node_label(|d: &(&'static str, Hsla)| d.0.into())
        .tooltip_name(|d: &(&'static str, Hsla)| d.0.into())
        .info(ui, id, info::charts::sankey_chart(t))
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

/// The padding round an Icons page gallery cell, and between its icon and
/// its name: the showcase's own, gpui's `p_2` and `gap_1`, as the theme
/// states no icon grid, and a cell is a gallery's, not a widget (spec
/// §3.1). The cells' infos say so.
pub(crate) const ICON_CELL_PADDING: Rems = rems(0.5);
pub(crate) const ICON_CELL_GAP: Rems = rems(0.25);

/// The padding of an animated icon's card on the Icons page, and between
/// its icon and its label: the showcase's own, gpui's `p_4` and `gap_2`, as
/// the theme states no such card (spec §3.1). The card's info says so.
pub(crate) const ANIMATED_CARD_PADDING: Rems = rems(1.);
pub(crate) const ANIMATED_CARD_GAP: Rems = rems(0.5);

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
        .p(ICON_CELL_PADDING)
        .gap(ICON_CELL_GAP)
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
/// IconName `cell.label` names, found by the connector's table, or, where
/// `by_role` (SF Symbols and Segoe Fluent), by `role` where one maps to it.
/// A recoloured icon is painted in `fg`.
pub(crate) fn gpui_icon(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: impl Into<SharedString>,
    cell: IconCell<'_>,
    role: Option<&str>,
    by_role: bool,
    fg: Hsla,
) -> Stateful<Div> {
    let id: SharedString = id.into();
    let drawn = cell.art.drawn();
    let info = info::icons::gpui_icon(cx.theme(), cell.label, role, by_role, cell.set, drawn, fg);
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
    let icon =
        chrome_glyph(drawn, icon).map(|icon| native_sized_glyph(cx, icon, context.builder()));
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
        .p(ICON_CELL_PADDING)
        .gap(ICON_CELL_GAP)
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
    Spin { duration_ms: NonZeroU32 },
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
    Spin {
        svg: &'a [u8],
        duration_ms: NonZeroU32,
    },
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
        .gap(ANIMATED_CARD_GAP)
        .p(ANIMATED_CARD_PADDING)
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
    ListHighlightForeground,
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
    let native = old_native(cx);
    let value = info::theme_map::row(t, token).value;
    let id = SharedString::from(format!("theme-map-swatch-{}", value.field));
    let label = SharedString::from(format!("{} {}", value.field, as_rgba(value.value)));
    // The fill is the datum this swatch exists to show; everything around it
    // is `frame`, the box every other demonstration sits in.
    h_flex()
        .gap(crate::pages::SWATCH_LABEL_GAP)
        .items_center()
        .child(refined(
            div().size(crate::pages::SWATCH_SIZE).bg(value.value),
            Some(frame),
        ))
        .child(Label::new(label).text_sm())
        .info(ui, id.clone(), info::theme_map::swatch(t, token, native))
        .debug_selector(move || id.to_string())
}

// ---------------------------------------------------------------------------
// The Basic page's own samples (Basic v3)
// ---------------------------------------------------------------------------

/// `button` labelled `label`. Where the installed 0.8.0 theme states the
/// font `geometry::button_label` reads, or the label is an element of
/// docs/showcase-elements.toml, the label is the Button's child, in the box
/// upstream puts its own `label` in (button/button.rs, `RenderOnce for
/// Button`: `min_w_0`, `whitespace_nowrap`, `text_ellipsis`): in
/// `button.label(NONE)`'s font through `geometry::button_label`, because upstream sizes
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
    let text = next_geometry(cx, next::geometry::button_label).and_then(Result::ok);
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
fn listed_icon(
    ui: &Entity<InfoRegistry>,
    icon: impl IntoElement,
    listed: Option<&'static str>,
) -> AnyElement {
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
/// Ghost Button draws no border at rest (the same, `border_l_1` and its
/// siblings go to the Default and outline variants alone; under a theme that
/// states the toolbar's outline, `geometry::toolbar_button_rest`'s border is
/// transparent at rest). `None` before `apply` ran.
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
    padded(style, p, None::<Pixels>, None)
}

/// A toggle button that is on: a Default Button reading `label`, refined by
/// `geometry::button`, shown selected, and refined by
/// `geometry::button_checked` -- filled with `button.body(CHECKED).fill` and
/// lettered in `button.label(CHECKED).color`, the pressed pair where no rule
/// states them for the state. Upstream's selected style would fill it with its
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
    let mut button_info = info::buttons::button(
        cx.theme(),
        ButtonKind::Default,
        ButtonState::Idle,
        false,
        false,
        None,
        button_theme(cx),
    );
    let button = next_native_info(
        Button::new(id),
        cx,
        next::geometry::button,
        "button",
        &mut button_info,
    );
    let checked = info::next_native_value(
        cx,
        next::geometry::button_checked,
        "button_checked",
        &mut button_info,
    );
    if let (Some(_), Some(r)) = (&checked, next_resolved(cx)) {
        let b = &r.button;
        button_info = button_info.instance(
            "state",
            toggle_state_line(
                next::state_fill_of(&b.body(WidgetState::CHECKED)).is_some(),
                next::state_fill_of(&b.body(WidgetState::PRESSED)).is_some(),
                next::state_text_color_of(&b.label(WidgetState::CHECKED)).is_some(),
            ),
        );
    }
    let button = refined(button.selected(true).toggled(true), checked.as_ref());
    InfoExt::info(labelled(ui, cx, button, id, label), ui, id, button_info)
        .debug_selector(move || id.into())
}

/// The toggle button's "state" line: what its checked fill and label are
/// painted with, from which of `button.body(CHECKED).fill`,
/// `button.body(PRESSED).fill` and `button.label(CHECKED).color` the theme
/// states.
pub(crate) fn toggle_state_line(
    checked_fill: bool,
    pressed_fill: bool,
    checked_label: bool,
) -> String {
    let fill = if checked_fill {
        "button.body(CHECKED).fill"
    } else if pressed_fill {
        "button.body(PRESSED).fill, where no rule states button.body(CHECKED).fill"
    } else {
        "gpui-component's own button_active token: the theme states neither button.body(CHECKED).fill nor button.body(PRESSED).fill, so geometry::button_checked sets no fill and upstream's selected fill stands (button/button.rs, ButtonVariant::selected)"
    };
    let text = if checked_label {
        "button.label(CHECKED).color"
    } else {
        "button.label(PRESSED).color, where no rule states button.label(CHECKED).color"
    };
    format!("selected: on, filled with {fill} and lettered in {text}")
}

/// A dialog's default button, the one Enter activates, reading `label`.
/// Where the theme gives it a look of its own (`variants::default_button`),
/// a Button on that custom variant, refined by `geometry::button` and then
/// by `geometry::default_button_rest`, its rest fill and frame, which the
/// variant does not give (its own rest fill is a fifth of its border
/// colour, button/button.rs, `ButtonVariant::bg_color`); elsewhere the
/// primary Button, refined by `geometry::button` alone, as macOS and
/// Windows draw it.
pub(crate) fn dialog_default_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    label: &'static str,
) -> Stateful<Div> {
    let native = old_native(cx);
    let own = native.and_then(|n| {
        let variant = variants::default_button(n, cx)?;
        let rest = geometry::default_button_rest(n)?;
        Some((n, variant, rest))
    });
    let (button, button_info) = match own {
        Some((n, variant, rest)) => {
            let mut button_info = info::buttons::dialog_default(cx.theme(), &n.resolved.button);
            let button = native_info(
                Button::new(id).custom(variant),
                cx,
                geometry::button,
                "button",
                &mut button_info,
            );
            (
                refined(button, Some(&rest)),
                button_info.geometry("default_button_rest"),
            )
        }
        None => {
            let mut button_info = info::buttons::button(
                cx.theme(),
                ButtonKind::Primary,
                ButtonState::Idle,
                false,
                false,
                None,
                button_theme(cx),
            );
            let button = next_native_info(
                Button::new(id).primary(),
                cx,
                next::geometry::button,
                "button",
                &mut button_info,
            );
            (button, button_info)
        }
    };
    InfoExt::info(labelled(ui, cx, button, id, label), ui, id, button_info)
        .debug_selector(move || id.into())
}

/// One of the Basic page's icon buttons.
pub(crate) struct IconButton<'a> {
    pub id: &'static str,
    /// The button's label where the chosen set has no icon for its role.
    pub name: &'static str,
    /// Its role's icon as the chosen set gives it
    /// (`Showcase::role_chrome_icon`).
    pub drawn: &'a (ChromeIcon, Option<IconName>),
    /// The element of docs/showcase-elements.toml its icon is.
    pub listed: &'static str,
    /// Whether it is a switch that is on: selected, and refined by
    /// `geometry::toolbar_button_checked`.
    pub selected: bool,
}

/// An icon-only button as the toolbar's (`toolbar_button`): the toolbar's
/// variant, its icon native-theme's for a role as the chosen set gives it
/// (`Showcase::role_chrome_icon`), at `toolbar.icon_size`, the icon
/// recording its bounds as the element `listed_icon`. Where the set has no
/// icon for the role the button shows `name` instead -- never another set's
/// icon. A button given `on_click` is a switch (the edit-mode one,
/// `Showcase::basic_edit`): while `selected` it is shown selected and refined
/// by `geometry::toolbar_button_checked` after its other refinements.
pub(crate) fn icon_button(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    spec: IconButton<'_>,
    on_click: Option<impl Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
) -> Stateful<Div> {
    let IconButton {
        id,
        name,
        drawn,
        listed,
        selected,
    } = spec;
    let native = old_native(cx);
    let icon = role_glyph_of(drawn);
    let mut button_info = info::buttons::icon_button(
        cx.theme(),
        icon.is_some(),
        on_click.as_ref().map(|_| selected),
        native.as_ref().map(|n| n.resolved),
    )
    .instance("icon", format!("{name}, at toolbar.icon_size"));
    if drawn.0.is_raster() {
        button_info = button_info.instance(
            "icon colour",
            format!(
                "pixels, or an SVG in several colours, drawn as an image (gpui::img), {}",
                info::chrome::RASTER_ICON_COLOUR
            ),
        );
    }
    let icon = icon.map(|icon| native_sized_glyph(cx, icon, geometry::icon_size_toolbar));
    if icon.is_some() && native_value(cx, geometry::icon_size_toolbar).is_some() {
        button_info = button_info.geometry("icon_size_toolbar");
    }
    let button = native_info(
        refined(
            Button::new(id).custom(toolbar_variant(cx)),
            tool_button_box(cx).as_ref(),
        ),
        cx,
        geometry::toolbar_button_rest,
        "toolbar_button_rest",
        &mut button_info,
    );
    let button = if selected {
        native_info(
            button.selected(true),
            cx,
            geometry::toolbar_button_checked,
            "toolbar_button_checked",
            &mut button_info,
        )
    } else {
        button
    };
    let button = match on_click {
        Some(on_click) => button.on_click(on_click),
        None => button,
    };
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

/// The glyph a role's `drawn` icon is (`Showcase::role_chrome_icon`):
/// gpui-component's own where its built-in set is chosen, the chosen set's
/// SVG or pixels otherwise, drawn as `chrome_glyph` draws one; `None` where
/// the set has none for the role.
fn role_glyph_of(drawn: &(ChromeIcon, Option<IconName>)) -> Option<Glyph> {
    match drawn {
        (ChromeIcon::Builtin(_), Some(icon)) => Some(Glyph::svg(Icon::new(icon.clone()))),
        (ChromeIcon::Loaded(_, bytes), _) => Some(Glyph::svg(Icon::default().data(bytes))),
        (ChromeIcon::Raster(name, raster), _) => Some(Glyph::Raster(name, raster.0.clone(), None)),
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
    let shown = role_glyph_of(drawn).map(|icon| native_sized_glyph(cx, icon, role));
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
/// and text colour, its fill, the colour of its lines, and whether the
/// header's line closes it.
struct TableLine<'a> {
    cells: [&'static str; 2],
    listed: [Option<&'static str>; 2],
    /// The first part of each cell's element id, its column the second.
    cell_id: &'static str,
    font: &'a ResolvedFontSpec,
    text: Hsla,
    fill: Option<Hsla>,
    /// The line down the first cell's right edge and, where `closed`, the
    /// one along the bottom: `list.grid_color` in a body row, the header's
    /// `list.header_separator_color`, else that.
    line: Hsla,
    closed: bool,
    /// The row's height where it is not a body row's.
    height: Option<Pixels>,
    /// The header's sections' fills; `None` for a body row.
    sections: Option<SectionFills>,
}

/// The fills a header section of [`files_table`] takes on its own, each
/// where the theme states one: under the pointer `list.header_hover_background`;
/// pressed `list.header_active_background`, else the hovered fill.
#[derive(Clone, Copy)]
struct SectionFills {
    hover: Option<Hsla>,
    active: Option<Hsla>,
}

/// Which row of [`files_table`] is selected, and whether its window is
/// active, which decides the selected row's pair (`selection_pair`).
#[derive(Clone, Copy)]
pub(crate) struct TableSelection {
    pub row: usize,
    pub window_active: bool,
}

/// Whether `r` gives the selection of a view without the focus a fill of its
/// own: `defaults.selection_inactive_background` other than
/// `defaults.selection_background`. Only then does a selected row change
/// with the window's state (`selection_pair`).
pub(crate) fn inactive_selection_differs(r: &native_theme_gpui::ResolvedTheme) -> bool {
    r.defaults.selection_inactive_background != r.defaults.selection_background
}

/// The fill and text colour of a selected row of [`files_table`] under `r`:
/// while the window is not active and the theme's inactive fill differs
/// from its selection's ([`inactive_selection_differs`]),
/// `defaults.selection_inactive_background` lettered
/// `defaults.selection_inactive_text_color`; otherwise the list's own pair,
/// `list.selection_background` and `list.selection_text_color`.
pub(crate) fn selection_pair(
    r: &native_theme_gpui::ResolvedTheme,
    window_active: bool,
) -> (Hsla, Hsla) {
    if !window_active && inactive_selection_differs(r) {
        (
            info::stated(r.defaults.selection_inactive_background),
            info::stated(r.defaults.selection_inactive_text_color),
        )
    } else {
        (
            info::stated(r.list.selection_background),
            info::stated(r.list.selection_text_color),
        )
    }
}

/// The group an unselected row of the Basic table names itself by, whose
/// hover its outline follows (`files_table`).
const TABLE_ROW_GROUP: &str = "showcase-table-row";

/// The inner radius of `geometry::list`'s frame, whose list border is
/// `border`: its `corner_radius` less its `line_width`, none below zero. A
/// fill where a row meets the frame is rounded by it on the corners it meets
/// (`ListRow::at_edges`, `files_table`).
pub(crate) fn list_inner_radius(border: &native_theme::theme::ResolvedWidgetBorder) -> Pixels {
    px((border.corner_radius - border.line_width).max(0.0))
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
/// and that padding, the header `list.header_height` tall where the theme
/// states it (`geometry::list_header_height`), else as a row is; the header
/// in `list.header_background` and
/// `list.header_font`, each of its two sections filled on its own under the
/// pointer and pressed where the theme states those fills (`SectionFills`),
/// which gpui-component's own table header has no style for
/// (table/state.rs, `render_th`); the rows in `list.item_font`, the row at
/// `selected` in `selection_pair`'s fill and text colour, which follow the
/// window's state, every other row outlined in `list.hover_border_color`
/// under the pointer; a line `separator.line_width` wide inside the
/// header's bottom edge and one down the first column's right edge through
/// every row, each a `widgets::Separator` recoloured, so one device pixel at
/// least (M9-165), laid over the cell's and a section's fill: in
/// `list.grid_color`, the header's in `list.header_separator_color` where
/// the theme states it (platform-facts §2.15: the last section has none of
/// its own, and the two half-width columns leave no space after it); the
/// whole framed by `list.border` on `list.background_color`. The header, its two cells and every row report
/// themselves. Without a native theme, the Data page's declarative `Table`.
pub(crate) fn files_table(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    id: &'static str,
    head: [(&'static str, &'static str); 2],
    rows: &[[&'static str; 2]],
    selected: TableSelection,
    width: Pixels,
) -> Stateful<Div> {
    let Some(n) = old_native(cx) else {
        let [(first, _), (second, _)] = head;
        return table(ui, cx, id, "Files", [first, second], rows);
    };
    let r = n.resolved;
    let l = &r.list;
    let colour = info::stated;
    let grid = colour(l.grid_color);
    let pad = &l.border.padding;
    let side = |v: Option<f32>| v.map_or(px(0.), px);
    let height = |line_height: Pixels| {
        l.row_height.map_or(
            px(f32::from(side(pad.top)) + f32::from(line_height) + f32::from(side(pad.bottom))),
            px,
        )
    };
    // A fill where a row meets the frame is rounded by the frame's inner
    // radius (`list_inner_radius`) on the corners it meets: the header's top
    // ones and the last row's bottom ones, as the egui and iced Basic tables
    // round theirs. GPUI Fast clips the frame's children to its curved
    // padding edge (gpui-fast 0.1.15 src/fast/rounded_clip.rs:68-104), so a
    // square fill would be cut at the curve; gpui-pre clips them to its
    // rectangle (gpui-pre 0.3.12 src/window.rs:2453-2461, ContentMask), so a
    // square fill would show past the curve there.
    let inner = list_inner_radius(&l.border);
    let row =
        |spec: TableLine<'_>| {
            let (cell_id, sections) = (spec.cell_id, spec.sections);
            let cells = spec.cells.into_iter().zip(spec.listed).enumerate().map(
                |(ix, (content, listed))| {
                    div()
                        .id((cell_id, ix))
                        .relative()
                        .w(relative(0.5))
                        .h_full()
                        .flex()
                        .items_center()
                        .pt(side(pad.top))
                        .pr(side(pad.right))
                        .pb(side(pad.bottom))
                        .pl(side(pad.left))
                        .child(text_in(&n, spec.font, content))
                        .when(ix == 0, |cell| {
                            cell.child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .bottom_0()
                                    .right_0()
                                    .child(widgets::Separator::vertical().bg(spec.line)),
                            )
                        })
                        .children(listed.map(|listed| elements::record(ui, listed)))
                        // A header section is filled on its own, as Breeze
                        // fills one; where it meets the frame its fill is
                        // rounded as the header's is.
                        .when_some(sections, |cell, fills| {
                            cell.when(ix == 0, |cell| cell.rounded_tl(inner))
                                .when(ix == 1, |cell| cell.rounded_tr(inner))
                                .when_some(fills.hover, |cell, fill| {
                                    cell.hover(move |s| s.bg(fill))
                                })
                                .when_some(fills.active, |cell, fill| {
                                    cell.active(move |s| s.bg(fill))
                                })
                                .debug_selector(move || format!("{id}-header-{ix}"))
                        })
                },
            );
            h_flex()
                .relative()
                .w_full()
                .h(spec
                    .height
                    .unwrap_or_else(|| height(line_height_of(&n, spec.font))))
                .text_color(spec.text)
                .when_some(spec.fill, |row, fill| row.bg(fill))
                .children(cells)
                .when(spec.closed, |row| {
                    row.child(
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .bottom_0()
                            .child(widgets::Separator::horizontal().bg(spec.line)),
                    )
                })
        };
    let [first, second] = head;
    let header_height = geometry::list_header_height(n);
    let header_info = info::data::native_table_header(r, &format!("{}, {}", first.0, second.0));
    let header_info = if header_height.is_some() {
        header_info.geometry("list_header_height")
    } else {
        header_info
    };
    let section_hover = l.header_hover_background.map(colour);
    let header = row(TableLine {
        cells: [first.0, second.0],
        listed: [Some(first.1), Some(second.1)],
        cell_id: "table-head",
        font: &l.header_font,
        text: colour(l.header_font.color),
        fill: Some(colour(l.header_background)),
        line: l.header_separator_color.map(colour).unwrap_or(grid),
        closed: true,
        height: header_height,
        sections: Some(SectionFills {
            hover: section_hover,
            active: l.header_active_background.map(colour).or(section_hover),
        }),
    })
    .rounded_tl(inner)
    .rounded_tr(inner)
    .info(ui, SharedString::from(format!("{id}-header")), header_info)
    .debug_selector(move || format!("{id}-header"))
    .w_full();
    let last = rows.len().checked_sub(1);
    // An unselected row's outline under the pointer, a box over the row
    // (`hover_edge`): square but on the last row's bottom corners, which
    // meet the frame.
    let edge_width = r.defaults.border.line_width;
    let edge = l
        .hover_border_color
        .filter(|_| edge_width > 0.0)
        .map(colour);
    let body = rows.iter().enumerate().map(|(ix, cells)| {
        let is_selected = ix == selected.row;
        let (fill, text) = if is_selected {
            let (fill, text) = selection_pair(r, selected.window_active);
            (Some(fill), text)
        } else {
            (None, colour(l.item_font.color))
        };
        // Under the pointer, a row that is not the selected one takes the
        // list's hover fill and text colour.
        let (hover, hover_text) = (colour(l.hover_background), colour(l.hover_text_color));
        row(TableLine {
            cells: *cells,
            listed: [None, None],
            cell_id: "table-cell",
            font: &l.item_font,
            text,
            fill,
            line: grid,
            closed: false,
            height: None,
            sections: None,
        })
        .id(("table-row", ix))
        .when(Some(ix) == last, |row| {
            row.rounded_bl(inner).rounded_br(inner)
        })
        .when(!is_selected, |row| {
            row.hover(move |s| s.bg(hover).text_color(hover_text))
        })
        .when_some(edge.filter(|_| !is_selected), |row, edge| {
            row.group(TABLE_ROW_GROUP).child(
                hover_edge(cx, edge, px(edge_width), TABLE_ROW_GROUP)
                    .when(Some(ix) == last, |b| b.rounded_bl(inner).rounded_br(inner)),
            )
        })
        .info(
            ui,
            SharedString::from(format!("{id}-row-{ix}")),
            info::data::native_table_row(r, is_selected, &cells.join(", ")),
        )
        .debug_selector(move || format!("{id}-row-{ix}"))
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
        .info(ui, id, info::data::native_table_frame(r, rows.len()))
        .self_start()
        .debug_selector(move || id.into())
}
