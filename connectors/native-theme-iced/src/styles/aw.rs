//! Style functions for the `iced_aw` widgets iced core does not have.
//!
//! Requires the `iced_aw` feature.
//!
//! native-theme models a card, a menu, a tab bar, a sidebar, a spinner and a
//! list. iced itself has no card widget, menu bar, tab bar, sidebar, spinner
//! or selection list; its drop-down menu is styled by [`crate::styles::menu`]
//! and a container drawn as a card by [`crate::styles::container_card`].
//! `iced_aw` 0.14 supplies the missing widgets, and its styling is iced's own shape: a `Style` struct
//! of public fields, a `Catalog` whose `Class` is a `StyleFn`, and a
//! `.style(..)` setter on the widget. So these functions follow the same rules
//! as [`crate::styles`] -- every field named, native values captured by value,
//! and a field the model does not carry read from `iced_aw`'s own default class
//! rather than written as a literal. Every closure here is
//! `Clone`, as [`crate::styles`]' are, because `SelectionList::new_with`
//! demands it.
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

use crate::palette::to_color;
use iced_aw::style::Status;
use iced_core::border::Radius;
use iced_core::theme::Theme;
use iced_core::{Background, Border};
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
/// The card is one surface: `card.background_color` fills it and its head,
/// body and foot alike, because `CardTheme` states a single fill, and
/// `card.border.*` outlines it. The three section labels are
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
    use iced_aw::style::card::Style;

    let c = &resolved.card;

    let panel = to_color(c.background_color);
    let label = to_color(resolved.defaults.text_color);

    let border_color = to_color(c.border.color);
    let border_width = c.border.line_width;
    let border_radius = c.border.corner_radius;

    move |_theme, _status| Style {
        background: Background::Color(panel),
        border_radius,
        border_width,
        border_color,
        head_background: Background::Color(panel),
        head_text_color: label,
        body_background: Background::Color(panel),
        body_text_color: label,
        foot_background: Background::Color(panel),
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
/// The bar and the menus it opens are the same surface here:
/// `menu.background_color` fills both and `menu.border.*` outlines both,
/// because `MenuTheme` states one menu panel. `path` -- the highlight under the
/// item the pointer is on -- is `menu.hover_background`, emitted as given: it
/// is a row highlight painted over a panel the widget also paints, so the
/// platform's layering happens by itself.
///
/// Three fields have no native source and come from
/// `menu_bar::primary(theme, status)`: the two shadows, because the model
/// carries `defaults.shadow_color` but no offset or blur, and `path_border`,
/// because the model's only menu border is the panel's and drawing it around a
/// highlighted row would outline something the platform does not.
///
/// `menu.hover_text_color`, `menu.font.color`, `.disabled_text_color`,
/// `.separator_color`, `.row_height`, `.icon_size` and `.icon_text_gap` have
/// no receiver: `menu_bar::Style` states no text color and no geometry, and an
/// `iced_aw` menu's items are the consumer's own widgets.
#[must_use = "this returns the style function; it does not apply it"]
pub fn menu(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, Status) -> iced_aw::style::menu_bar::Style + Clone + use<> {
    use iced_aw::style::menu_bar::Style;

    let m = &resolved.menu;

    let panel = to_color(m.background_color);
    let highlight = to_color(m.hover_background);

    let border = Border {
        color: to_color(m.border.color),
        width: m.border.line_width,
        radius: Radius::new(m.border.corner_radius),
    };

    move |theme, status| {
        let iced = iced_aw::style::menu_bar::primary(theme, status);
        Style {
            bar_background: Background::Color(panel),
            bar_border: border,
            bar_shadow: iced.bar_shadow,
            menu_background: Background::Color(panel),
            menu_border: border,
            menu_shadow: iced.menu_shadow,
            path: Background::Color(highlight),
            path_border: iced.path_border,
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
/// The strip is `tab.bar_background`, and `tab.border.*` is the tab's own
/// border: its color and width outline the labels and its corner radius rounds
/// them. `icon_color` is the label color, which is what a tab's icon is: every
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
    use iced_aw::style::tab_bar::Style;

    let t = &resolved.tab;

    let strip = to_color(t.bar_background);
    let idle = to_color(t.background_color);
    let selected = to_color(t.active_background);
    let hovered = to_color(t.hover_background.unwrap_or(t.background_color));

    let label = to_color(t.font.color);
    let selected_label = to_color(t.active_text_color);
    let hovered_label = to_color(t.hover_text_color);

    let border_color = to_color(t.border.color);
    let border_width = t.border.line_width;
    let border_radius = Radius::new(t.border.corner_radius);

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
        Style {
            background: Some(Background::Color(strip)),
            border_color: iced.border_color,
            border_width: iced.border_width,
            tab_border_radius: border_radius,
            tab_label_background: fill,
            tab_label_border_color: border_color,
            tab_label_border_width: border_width,
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
    use iced_aw::style::sidebar::Style;

    let s = &resolved.sidebar;

    let panel = to_color(s.background_color);
    let selected = to_color(s.selection_background);
    let hovered = to_color(s.hover_background);

    let label = to_color(s.font.color);
    let selected_label = to_color(s.selection_text_color);

    let border_color = to_color(s.border.color);
    let border_width = s.border.line_width;

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
/// `list.alternate_row_background`, `.header_background`, `.header_font` and
/// `.grid_color` have no receiver at all -- a selection list has no striping,
/// no column header and no grid.
#[must_use = "this returns the style function; it does not apply it"]
pub fn selection_list(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, Status) -> iced_aw::style::selection_list::Style + Clone + use<> {
    use iced_aw::style::selection_list::Style;

    let l = &resolved.list;

    let panel = to_color(l.background_color);
    let selected = to_color(l.selection_background);
    let hovered = to_color(l.hover_background);

    let label = to_color(l.item_font.color);
    let selected_label = to_color(l.selection_text_color);
    let hovered_label = to_color(l.hover_text_color);
    let disabled_label = to_color(l.disabled_text_color);

    let border_color = to_color(l.border.color);
    let border_width = l.border.line_width;

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
/// `container(Spinner::new()).style(styles::aw::spinner(&resolved))`.
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
    use iced_widget::container::Style;

    let arc = to_color(resolved.spinner.fill_color);

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
