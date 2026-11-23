//! Style functions for the `iced_aw` widgets iced core does not have.
//!
//! Requires the `iced_aw` feature.
//!
//! native-theme models a card, a menu, a tab bar, a sidebar, a spinner and a
//! list. iced itself has no card widget, menu bar, tab bar, sidebar, spinner
//! or selection list; its drop-down menu is styled by [`crate::styles::menu`]
//! and a container drawn as a card by [`crate::styles::container_card`].
//! `iced_aw` 0.14 supplies the missing widgets, and its styling is iced's own
//! shape: a `Style` struct of public fields, a `Catalog` whose `Class` is a
//! `StyleFn`, and a `.style(..)` setter on the widget. So these functions
//! follow the same rules as [`crate::styles`] -- every field named, native
//! values captured by value, and a field the model does not carry read from
//! `iced_aw`'s own default class rather than written as a literal, except
//! where the platform draws nothing (a transparent fill, a zero width or
//! radius, `Border::default()`, `Shadow::default()`). Every
//! closure here is `Clone`, as [`crate::styles`]' are, because
//! `SelectionList::new_with` demands it.
//!
//! `iced_aw` states one `Status` enum for every widget
//! (`iced_aw/src/style/status.rs`), of which each widget requests only the
//! values it has a meaning for; each function's doc comment names them and
//! says where the others come from.
//!
//! One of these functions does not style its widget directly: `Spinner` has no
//! `Style`, no `Catalog` and no style setter at all, so `spinner` styles the
//! container a consumer wraps it in. `ContextMenu` gets no function: its
//! `Style` is a one-field backdrop scrim (`style/context_menu.rs:9-14`) rather
//! than the menu styling, and no platform paints one.
//!
//! Upstream citations in this module are read at iced_aw 0.14.1, with
//! iced_widget 0.14.2 and iced_core 0.14.0 beneath it.

use super::{border_of, composite_over};
use crate::palette::to_color;
use iced_aw::style::Status;
use iced_core::border::Radius;
use iced_core::theme::Theme;
use iced_core::{Background, Border, Color, Shadow};
use iced_widget::container;
use native_theme::theme::ResolvedTheme;

// Each function imports its own `Style` locally: `iced_aw`'s style modules
// share their names with these functions, as `iced_widget`'s do with
// `crate::styles`'.

/// The platform's own card, for `Card::style(..)`.
///
/// Replaces `iced_aw::style::card::primary` (`style/card.rs:89`), the class a
/// `Card` gets with no `.style(..)`: it paints the head in the crate's own
/// `colors::PRIMARY` and labels it white, neither of which is a platform
/// value.
///
/// The card is one surface: `card.background_color` fills it, because
/// `CardTheme` states a single fill, and `card.border.*` outlines it, all
/// round. `iced_aw` 0.14.1 paints that fill and the border first, as one
/// quad over the card's bounds, and then fills the head, the body and the
/// foot each across the card's whole width (`widget/card.rs:604-633`,
/// `draw_head` `:883-920`, `draw_body` `:974-988`, `draw_foot`
/// `:1024-1038`), so a section fill hides the border at the card's sides and
/// leaves it showing only where two sections' edges leave a pixel between
/// them. The three section fills are therefore transparent: the one fill,
/// painted once under the border, shows through them, and the border shows
/// where the theme states it. The three section labels are
/// `defaults.text_color` -- `CardTheme` carries no font, so the text on a
/// card is the platform's own, which is what `styles::container_card` leaves
/// inherited for the same reason.
///
/// `close_color` is set to `defaults.text_color` as well, but `iced_aw` 0.14.1
/// does not use it for the close button. `Card::on_close` styles that button
/// with a closure that reads `iced_aw`'s *default* card class rather than the
/// one passed to `.style(..)` (`widget/card.rs:193-206`), and an iced button
/// draws its content in its own style's `text_color`, ignoring the one the
/// card hands it (`button.rs:368`, `:401-407`). The default class is
/// `primary`, whose close color is white (`style/card.rs:78-80`, `:141-149`),
/// so the icon is white on every theme. A card that must be dismissible on a
/// light theme is better given a button of its own in the head or the foot.
///
/// A `Card` asks for `Status::Active` and nothing else (`card.rs:194-197`,
/// `:601`), and `iced_aw`'s own class ignores the status as well, so this
/// closure answers every status with the one appearance the platform states.
#[must_use = "this returns the style function; it does not apply it"]
pub fn card(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, Status) -> iced_aw::style::card::Style + Clone + use<> {
    let c = &resolved.card;
    card_from(
        to_color(c.background_color),
        to_color(resolved.defaults.text_color),
        border_of(&c.border),
    )
}

/// [`card`] of what it reads: the card's fill, the text on it and its
/// border.
pub(crate) fn card_from(
    panel: Color,
    label: Color,
    border: Border,
) -> impl Fn(&Theme, Status) -> iced_aw::style::card::Style + Clone + use<> {
    use iced_aw::style::card::Style;

    let border_color = border.color;
    let border_width = border.width;
    let border_radius = border.radius.top_left;

    // The sections show the card's own fill through them, under its border.
    let section = Background::Color(Color::TRANSPARENT);

    move |_theme, _status| Style {
        background: Background::Color(panel),
        border_radius,
        border_width,
        border_color,
        head_background: section,
        head_text_color: label,
        body_background: section,
        body_text_color: label,
        foot_background: section,
        foot_text_color: label,
        close_color: label,
    }
}

/// The platform's own menu bar and menus, for `MenuBar::style(..)`.
///
/// Replaces `iced_aw::style::menu_bar::primary` (`style/menu_bar.rs:83`),
/// which paints both surfaces in the palette's base background and the open
/// path in its weak primary.
///
/// The bar is the tools area's fill, `toolbar.background_color`: Breeze fills
/// the menu bar and the toolbars as one area; on every other bundled preset it
/// resolves to `defaults.background_color`, as the window's does. It has no
/// frame (`Border::default()`): `MenuTheme`'s border is no bar's. A menu is
/// the menu's panel, `menu.background_color`,
/// framed by the popup's border, `popover.border` (its colour, width and
/// radius): the platforms frame a menu as they frame every popup
/// (`docs/platform-facts.md` §2.6: the popup border is §2.16's), and `MenuTheme`'s
/// border states no line of its own. `path` -- the highlight under the item
/// the pointer is on -- is `menu.hover_background`, emitted as given: it is
/// a row highlight painted over a panel the widget also paints, so the
/// platform's layering happens by itself.
///
/// Two fields have no native source and come from
/// `menu_bar::primary(theme, status)`: the bar's shadow and the menus'
/// shadow, because the model carries `defaults.shadow_color` but no offset or
/// blur -- a menu casts it only where `popover.border.shadow_enabled` states
/// one.
///
/// `path_border` outlines the highlight where the theme states
/// `menu.hover_border_color`: that colour, composited over the highlight,
/// `defaults.border.line_width` wide, rounded by `menu.border.corner_radius`.
/// iced paints a quad's border in place of its fill, where Breeze lays the
/// line over the fill, so the colour is flattened onto the highlight first
/// (an opaque line, as kde-breeze's, comes back as it is). Where the theme
/// states no outline, `path_border` is `menu_bar::primary`'s.
///
/// `iced_aw` paints `path` and `path_border` only under `DrawPath::Backdrop`
/// (`widget/menu/menu_bar.rs:598-621` under the open title,
/// `widget/menu/menu_tree.rs:783-801` under the active item); under its
/// default, `DrawPath::FakeHovering` (`menu_bar.rs:143`), it draws no path
/// and hands the item a pointer at its centre instead, so the highlight is
/// the item's own button style. Under `Backdrop` it also outlines an open
/// bar title, which Breeze does not.
///
/// `menu.hover_text_color`, `menu.font.color`, `.disabled_text_color`,
/// `.separator_color`, `.row_height`, `.min_row_height`, `.icon_size`,
/// `.icon_text_gap`, `.active_background`, `.active_text_color`,
/// `.panel_padding_*`, `.bar_hover_background`, `.bar_open_background`,
/// `.bar_hover_text_color` and `.bar_padding_*` have no receiver:
/// `menu_bar::Style` states no text color and no geometry, its path has no
/// pressed status (`menu_bar.rs:587` asks for `Active` alone), and an
/// `iced_aw` menu's items, its bar's titles and its panel's padding
/// (`Menu::padding`) are the consumer's own. Padding is per physical side;
/// a right-to-left application swaps it.
#[must_use = "this returns the style function; it does not apply it"]
pub fn menu(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, Status) -> iced_aw::style::menu_bar::Style + Clone + use<> {
    let m = &resolved.menu;
    menu_from(&MenuSource {
        bar: to_color(resolved.toolbar.background_color),
        panel: to_color(m.background_color),
        highlight: to_color(m.hover_background),
        frame: border_of(&resolved.popover.border),
        shadow: resolved.popover.border.shadow_enabled,
        hover_border_color: m.hover_border_color.map(to_color),
        hover_border_width: resolved.defaults.border.line_width,
        item_radius: m.border.corner_radius,
    })
}

/// What the menu bar's style reads from the theme, of either model.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MenuSource {
    pub bar: Color,
    pub panel: Color,
    /// The item under the pointer.
    pub highlight: Color,
    /// The open menu's outline.
    pub frame: Border,
    /// Whether the open menu casts a shadow.
    pub shadow: bool,
    /// The outline of the item under the pointer, as stated, a soft option.
    pub hover_border_color: Option<Color>,
    pub hover_border_width: f32,
    pub item_radius: f32,
}

/// [`menu`] of what it reads.
pub(crate) fn menu_from(
    m: &MenuSource,
) -> impl Fn(&Theme, Status) -> iced_aw::style::menu_bar::Style + Clone + use<> {
    use iced_aw::style::menu_bar::Style;

    let bar = m.bar;
    let panel = m.panel;
    let highlight = m.highlight;
    let frame = m.frame;
    let shadow = m.shadow;
    let outline = m.hover_border_color.map(|c| Border {
        color: composite_over(c, highlight),
        width: m.hover_border_width,
        radius: Radius::new(m.item_radius),
    });

    move |theme, status| {
        let iced = iced_aw::style::menu_bar::primary(theme, status);
        Style {
            bar_background: Background::Color(bar),
            bar_border: Border::default(),
            bar_shadow: iced.bar_shadow,
            menu_background: Background::Color(panel),
            menu_border: frame,
            menu_shadow: if shadow {
                iced.menu_shadow
            } else {
                Shadow::default()
            },
            path: Background::Color(highlight),
            path_border: outline.unwrap_or(iced.path_border),
        }
    }
}

/// The platform's own tab bar, for `TabBar::style(..)` and for
/// `Tabs::tab_bar_style(..)` (`tabs.rs:253-257`, which forwards it to the bar
/// it owns).
///
/// Replaces `iced_aw::style::tab_bar::primary` (`style/tab_bar.rs:92`), which
/// paints every tab label in the palette's primary family and leaves the strip
/// behind them transparent.
///
/// **A `TabBar` spells its three tab states in the shared `Status`
/// unusually** (`tab_bar.rs:589-595`): `Hovered` is the tab under the pointer,
/// `Active` is the *selected* tab, and `Disabled` is a tab that is merely not
/// selected -- not a tab that cannot be clicked. The pointer is tested first,
/// so the *selected* tab under the pointer also arrives as `Hovered` and this
/// function is not told it is the selected one: it shows the hover fill and
/// label while the pointer is on it. `iced_aw` offers no way to tell the two
/// apart. So the three are
/// `tab.hover_background` with `.hover_text_color`, `tab.active_background`
/// with `.active_text_color`, and `tab.background_color` with `tab.font.color`.
/// `hover_background` is a soft option: `None` is the platform saying a hovered
/// tab has no fill of its own, so it copies the idle one. All three are
/// emitted as given -- a tab label is painted over the strip this same function
/// fills, so the platform's layering happens by itself.
///
/// The strip is `tab.bar_background`, and `tab.border.*` is the *selected*
/// tab's border: its color and width outline the `Active` label and its
/// corner radius rounds that label's top corners, the bottom ones square.
/// That is the platforms' own (`docs/platform-facts.md` §2.11): Breeze strokes the
/// selected tab only, rounding its top corners, and draws an unselected one
/// with no pen; WinUI's selected tab has a top-only radius and an unselected
/// one a transparent stroke. The model states nothing for an unselected tab's
/// outline, so `Hovered` and `Disabled` get none: no border width and square
/// corners. `iced_aw`'s own default would outline every tab 1px
/// (`style/tab_bar.rs:57-58`). `icon_color` is the label color, which is what a tab's icon is: every
/// one of `iced_aw`'s own tab themes states the two as the same color
/// (`style/tab_bar.rs:115-212`).
///
/// Four fields have no native source and come from
/// `tab_bar::primary(theme, status)`: the strip's own `border_color` and
/// `border_width`, because `TabTheme` states one border and it is the tab's,
/// and the close icon's `icon_background` and `icon_border_radius`. So do the
/// label colors in `Pressed`, `Focused` and `Selected`, three statuses a
/// `TabBar` never asks for and the model describes no tab in.
///
/// Where the theme states `tab.active_outward_corner_radius`, the selected
/// tab curves out into the bar at its feet: wrap the bar, or the `Tabs`, in
/// [`crate::tab_flares()`], which draws the flares and covers the selected
/// tab's bottom edge after the bar, with the room for them in a container's
/// padding outside it. The selected tab under the pointer is `Hovered` here,
/// so it loses its outline and its top corners while the flares stay.
///
/// `tab.min_width` and `.min_height` have no receiver in the `Style`, and no
/// faithful one anywhere: they are **minima**, and every `iced_aw` receiver
/// that could carry them takes a fixed extent -- `TabBar::tab_width(Length)`
/// (`tab_bar.rs:309`), `::width(..)` (`:351`) and `::height(..)` (`:223`),
/// none of which has a minimum form. A consumer who passes the platform's
/// minimum to one of them gets a tab that is *exactly* its minimum rather
/// than *at least* it, which is the closest iced_aw allows.
#[must_use = "this returns the style function; it does not apply it"]
pub fn tab_bar(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, Status) -> iced_aw::style::tab_bar::Style + Clone + use<> {
    let t = &resolved.tab;
    tab_bar_from(&TabBarSource {
        bar_background: to_color(t.bar_background),
        background: to_color(t.background_color),
        active_background: to_color(t.active_background),
        hover_background: t.hover_background.map(to_color),
        label: to_color(t.font.color),
        active_label: to_color(t.active_text_color),
        hover_label: to_color(t.hover_text_color),
        border: border_of(&t.border),
    })
}

/// What the tab bar's style reads from the theme, of either model: each
/// field's 0.7.2 name, soft options as `None` where unset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TabBarSource {
    pub bar_background: Color,
    pub background: Color,
    pub active_background: Color,
    pub hover_background: Option<Color>,
    pub label: Color,
    pub active_label: Color,
    pub hover_label: Color,
    /// The selected tab's outline.
    pub border: Border,
}

/// [`tab_bar`] of what it reads.
pub(crate) fn tab_bar_from(
    t: &TabBarSource,
) -> impl Fn(&Theme, Status) -> iced_aw::style::tab_bar::Style + Clone + use<> {
    use iced_aw::style::tab_bar::Style;

    let strip = t.bar_background;
    let idle = t.background;
    let selected = t.active_background;
    let hovered = t.hover_background.unwrap_or(t.background);

    let label = t.label;
    let selected_label = t.active_label;
    let hovered_label = t.hover_label;

    let border_color = t.border.color;
    let border_width = t.border.width;
    let selected_radius = Radius::new(0.0).top(t.border.radius.top_left);

    move |theme, status| {
        let iced = iced_aw::style::tab_bar::primary(theme, status);
        let (fill, text, icon) = match status {
            Status::Active => (Background::Color(selected), selected_label, selected_label),
            Status::Hovered => (Background::Color(hovered), hovered_label, hovered_label),
            Status::Disabled => (Background::Color(idle), label, label),
            Status::Pressed | Status::Focused | Status::Selected => {
                (iced.tab_label_background, iced.text_color, iced.icon_color)
            }
        };
        let (width, radius) = match status {
            Status::Active => (border_width, selected_radius),
            _ => (0.0, Radius::new(0.0)),
        };
        Style {
            background: Some(Background::Color(strip)),
            border_color: iced.border_color,
            border_width: iced.border_width,
            tab_border_radius: radius,
            tab_label_background: fill,
            tab_label_border_color: border_color,
            tab_label_border_width: width,
            icon_color: icon,
            icon_background: iced.icon_background,
            icon_border_radius: iced.icon_border_radius,
            text_color: text,
        }
    }
}

/// The platform's own sidebar, for `Sidebar::style(..)`
/// (`sidebar/sidebar.rs:361`) and for `SidebarWithContent::sidebar_style(..)`
/// (`:1308`) -- the compound widget spells the setter differently and has no
/// `style` of its own.
///
/// Replaces `iced_aw::style::sidebar::primary` (`style/sidebar.rs:85`), which
/// paints every item in the palette's primary family and leaves the panel
/// behind them transparent.
///
/// A `Sidebar` reads the shared `Status` the way a `TabBar` does
/// (`sidebar/sidebar.rs:980-986`): `Hovered` is the item under the pointer,
/// `Active` is the selected item, `Disabled` is an unselected one. The pointer
/// is tested first here too, so the *selected* item under the pointer also
/// arrives as `Hovered` and this function cannot tell it apart from an
/// unselected one: it shows the hover fill while the pointer is on it. So the
/// three
/// are `sidebar.hover_background`, `.selection_background` with
/// `.selection_text_color`, and the panel's own `sidebar.background_color` with
/// `sidebar.font.color` -- the platform states no separate fill for an
/// unselected item, so what shows there is the panel. `SidebarTheme` states no
/// hovered label either, so a hovered item keeps `sidebar.font.color`. The
/// fills are emitted as given: an item is painted over the panel this same
/// function fills.
///
/// The panel is `sidebar.background_color` with `sidebar.border.*`, the one
/// border the model states for a sidebar -- its color and its width, because
/// `sidebar.border.corner_radius` has no receiver: `sidebar::Style` carries no
/// radius but the close icon's, and `iced_aw` rounds the panel and the items
/// with a hardcoded `(0.0).into()` (`sidebar/sidebar.rs:616`, `:992`).
/// `icon_color` is the item's label color, as it is on a tab.
///
/// Five fields have no native source and come from
/// `sidebar::primary(theme, status)`: the items' own `tab_label_border_color`
/// and `tab_label_border_width`, the close icon's `icon_background` and
/// `icon_border_radius`, and the label colors in `Pressed`, `Focused` and
/// `Selected` -- statuses a `Sidebar` never asks for.
#[must_use = "this returns the style function; it does not apply it"]
pub fn sidebar(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, Status) -> iced_aw::style::sidebar::Style + Clone + use<> {
    let s = &resolved.sidebar;
    sidebar_from(&ListSource {
        background: to_color(s.background_color),
        selection_background: to_color(s.selection_background),
        hover_background: to_color(s.hover_background),
        label: to_color(s.font.color),
        selection_label: to_color(s.selection_text_color),
        hover_label: to_color(s.font.color),
        disabled_label: to_color(s.font.color),
        border_color: to_color(s.border.color),
        border_width: s.border.line_width,
    })
}

/// What the sidebar's and the selection list's styles read from the theme,
/// of either model: the panel, its rows at rest, selected, hovered and
/// disabled, and its outline.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ListSource {
    pub background: Color,
    pub selection_background: Color,
    pub hover_background: Color,
    pub label: Color,
    pub selection_label: Color,
    pub hover_label: Color,
    /// A disabled row's label, faded already.
    pub disabled_label: Color,
    pub border_color: Color,
    pub border_width: f32,
}

/// [`sidebar`] of what it reads (the hovered and the disabled label unread).
pub(crate) fn sidebar_from(
    s: &ListSource,
) -> impl Fn(&Theme, Status) -> iced_aw::style::sidebar::Style + Clone + use<> {
    use iced_aw::style::sidebar::Style;

    let panel = s.background;
    let selected = s.selection_background;
    let hovered = s.hover_background;

    let label = s.label;
    let selected_label = s.selection_label;

    let border_color = s.border_color;
    let border_width = s.border_width;

    move |theme, status| {
        let iced = iced_aw::style::sidebar::primary(theme, status);
        let (fill, text) = match status {
            Status::Active => (Background::Color(selected), selected_label),
            Status::Hovered => (Background::Color(hovered), label),
            Status::Disabled => (Background::Color(panel), label),
            Status::Pressed | Status::Focused | Status::Selected => {
                (iced.tab_label_background, iced.text_color)
            }
        };
        let icon = match status {
            Status::Active | Status::Hovered | Status::Disabled => text,
            Status::Pressed | Status::Focused | Status::Selected => iced.icon_color,
        };
        Style {
            background: Some(Background::Color(panel)),
            border_color: Some(border_color),
            border_width,
            tab_label_background: fill,
            tab_label_border_color: iced.tab_label_border_color,
            tab_label_border_width: iced.tab_label_border_width,
            icon_color: icon,
            icon_background: iced.icon_background,
            icon_border_radius: iced.icon_border_radius,
            text_color: text,
        }
    }
}

/// The platform's own selection list, for `SelectionList::style(..)` -- and
/// for the `style` argument of `SelectionList::new_with(..)`, which is the
/// one that reaches the rows.
///
/// `SelectionList::new` gives the outer list *and* the inner `List` that
/// draws the rows `Catalog::default()`, and `.style(..)` replaces only the
/// outer one (`selection_list.rs:72-97`, `:159-167`). A consumer who wants
/// the rows themed too must pass this function as the fifth argument of
/// `new_with` (`selection_list.rs:100-110`), which sets both. That argument
/// is `impl Fn(&Theme, Status) -> Style + 'a + Clone`, which is why this
/// function's return type says `+ Clone`.
///
/// Replaces `iced_aw::style::selection_list::primary`
/// (`style/selection_list.rs:47`), which paints the hovered and the selected
/// row in the palette's primary family.
///
/// A `SelectionList` asks for `Active` for the list itself and for an
/// unselected row, `Hovered` for the row under the pointer and `Selected` for
/// the selected one (`selection_list.rs:302`,
/// `selection_list/list.rs:241-258`), which are `list.background_color` with
/// `list.item_font.color`, `list.hover_background` with `.hover_text_color`,
/// and `.selection_background` with `.selection_text_color`. Selection wins
/// over the pointer here (`list.rs:237-247`, `:251-256`), so a selected row
/// keeps its selected look while hovered -- unlike a tab or a sidebar item,
/// whose widgets test the pointer first. `Disabled` is
/// `list.disabled_text_color` on the list's own fill -- a mapping the model
/// does describe but that nothing paints today, because `iced_aw` 0.14.1 asks
/// a row only for `Selected`, `Hovered` and `Active`
/// (`selection_list/list.rs:241-258`). The row fills are emitted as given: a
/// row is painted over the list this same function fills.
///
/// The outline is `list.border.*` -- its color and its width, because
/// `list.border.corner_radius` has no receiver: `selection_list::Style` carries
/// no radius at all, and `iced_aw` outlines the list with a hardcoded
/// `(0.0).into()` (`selection_list.rs:309`). `Pressed` and `Focused`, which a
/// `SelectionList` never asks for and the model describes no row in, come from
/// `selection_list::primary(theme, status)`.
///
/// `list.row_height` is reachable **only indirectly**: `iced_aw` 0.14.1 has no
/// `item_height` setter anywhere, and a row's height is the sum
/// `text_size + padding.y()` of two of the arguments `new_with` takes
/// (`selection_list/list.rs:118`, `:209`, `:223`, `:294`). So a consumer
/// passes `list.item_font.size` as `text_size` and, as the padding, the
/// vertical remainder the label leaves -- `row_height - item_font.size`, split
/// between top and bottom -- guarding the case where a theme states a row no
/// taller than its own label, and the case where it states none (KDE's rows
/// size to their content), where the consumer's own padding stands. Nothing
/// horizontal is implied: the label is
/// drawn at the row's own `bounds.x` (`selection_list/list.rs:274`), while the
/// list's intrinsic width does read `padding.x()` (`selection_list.rs:237`,
/// `:244`).
///
/// `list.alternate_row_background`, `.hover_border_color`,
/// `.header_background`, `.header_font` and `.grid_color` have no receiver at
/// all -- a selection list has no striping, fills a hovered row with a quad
/// whose border is hardcoded 0 wide (`selection_list/list.rs:226-249`), and
/// has no column header and no grid.
#[must_use = "this returns the style function; it does not apply it"]
pub fn selection_list(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, Status) -> iced_aw::style::selection_list::Style + Clone + use<> {
    let l = &resolved.list;
    selection_list_from(&ListSource {
        background: to_color(l.background_color),
        selection_background: to_color(l.selection_background),
        hover_background: to_color(l.hover_background),
        label: to_color(l.item_font.color),
        selection_label: to_color(l.selection_text_color),
        hover_label: to_color(l.hover_text_color),
        // `ListTheme` states no `disabled_opacity`: the defaults' fades a
        // disabled row (see `styles::faded`).
        disabled_label: super::faded(
            to_color(l.disabled_text_color),
            resolved.defaults.disabled_opacity,
        ),
        border_color: to_color(l.border.color),
        border_width: l.border.line_width,
    })
}

/// [`selection_list`] of what it reads.
pub(crate) fn selection_list_from(
    l: &ListSource,
) -> impl Fn(&Theme, Status) -> iced_aw::style::selection_list::Style + Clone + use<> {
    use iced_aw::style::selection_list::Style;

    let panel = l.background;
    let selected = l.selection_background;
    let hovered = l.hover_background;

    let label = l.label;
    let selected_label = l.selection_label;
    let hovered_label = l.hover_label;
    let disabled_label = l.disabled_label;

    let border_color = l.border_color;
    let border_width = l.border_width;

    move |theme, status| {
        let iced = iced_aw::style::selection_list::primary(theme, status);
        let (fill, text) = match status {
            Status::Active => (Background::Color(panel), label),
            Status::Hovered => (Background::Color(hovered), hovered_label),
            Status::Selected => (Background::Color(selected), selected_label),
            Status::Disabled => (Background::Color(panel), disabled_label),
            Status::Pressed | Status::Focused => (iced.background, iced.text_color),
        };
        Style {
            text_color: text,
            background: fill,
            border_width,
            border_color,
        }
    }
}

/// The platform's own spinner, for the container a `Spinner` is wrapped in:
/// `container(iced_aw::Spinner::new()).style(styles::aw::spinner(&resolved))`.
///
/// It styles a container because `iced_aw` 0.14.1 gives `Spinner` no `Style`,
/// no `Catalog` and no style setter at all (`spinner.rs:16-66`): it paints its
/// circle in the **inherited** color, `fill_circle(.., style.text_color)`
/// where `style: &renderer::Style` (`spinner.rs:151`). A container is what
/// states that inherited color for its children -- it draws them with
/// `renderer::Style { text_color: style.text_color.unwrap_or(renderer_style
/// .text_color) }` (`container.rs:354-362`) -- so a container carrying
/// `spinner.fill_color` is how the platform's spinner color reaches the
/// widget. Shape B: a spinner has no `Status`, so the closure takes the theme
/// alone.
///
/// `text_color` is the only field with a native source. The rest come from
/// `container::Style::default()`, which paints nothing: the wrapper exists to
/// state a color, not to draw a panel behind the spinner.
///
/// The spinner's three other native fields are the consumer's, like any other
/// builder geometry: `spinner.diameter` is `Spinner::width(..)` and
/// `::height(..)` (`spinner.rs:47-58`), and `spinner.stroke_width` and
/// `.min_diameter` have no receiver at all -- `Spinner::circle_radius(..)`
/// (`:62-65`) sizes the orbiting dot rather than an arc's stroke, and iced_aw
/// states no minimum anywhere.
#[must_use = "this returns the style function; it does not apply it"]
pub fn spinner(resolved: &ResolvedTheme) -> impl Fn(&Theme) -> container::Style + Clone + use<> {
    spinner_from(to_color(resolved.spinner.fill_color))
}

/// [`spinner`] of what it reads: the arc's colour.
pub(crate) fn spinner_from(arc: Color) -> impl Fn(&Theme) -> container::Style + Clone + use<> {
    use iced_widget::container::Style;

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: Some(arc),
            background: iced.background,
            border: iced.border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::theme::ColorMode;

    /// The card's border is not overdrawn by a section fill: `iced_aw`
    /// 0.14.1 paints the head, the body and the foot after the card's fill
    /// and border, each across the card's whole width (`widget/card.rs`,
    /// `draw`, `draw_head`, `draw_body`, `draw_foot`), so under every
    /// bundled preset in both modes the three section fills paint nothing,
    /// while the card's own fill is `card.background_color` and its border
    /// `card.border.color` at `card.border.line_width`.
    #[test]
    fn no_section_fill_overdraws_the_cards_border() {
        let mut checked = 0;
        for info in native_theme::theme::Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = native_theme::theme::Theme::preset(info.key)
                    .unwrap()
                    .resolve(mode)
                    .unwrap()
                    .variant;
                let style = card(&r)(&Theme::Light, Status::Active);
                for (section, fill) in [
                    ("head", style.head_background),
                    ("body", style.body_background),
                    ("foot", style.foot_background),
                ] {
                    let Background::Color(color) = fill else {
                        panic!("{} {mode:?}: the {section} fill is not a colour", info.key);
                    };
                    assert!(
                        color.a == 0.0,
                        "{} {mode:?}: the {section} fill {color:?} overdraws the border",
                        info.key
                    );
                }
                assert_eq!(
                    style.background,
                    Background::Color(to_color(r.card.background_color)),
                    "{} {mode:?}: the card's fill",
                    info.key
                );
                assert_eq!(style.border_color, to_color(r.card.border.color));
                assert_eq!(style.border_width, r.card.border.line_width);
                checked += 1;
            }
        }
        assert!(checked > 0, "no preset was checked");
    }
}
