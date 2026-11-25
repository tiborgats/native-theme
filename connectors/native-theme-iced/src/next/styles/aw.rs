//! [`crate::styles::aw`] on the 0.8.0 model: each function reads what its
//! 0.7.2 namesake reads, at its home in the table of homes, and hands it to
//! the function that namesake hands its own reads to. Where the theme states
//! nothing for a value `iced_aw` has one of its own for, `iced_aw`'s stays
//! (spec §10, rule 0): its default class's for that status.
//!
//! Requires the `iced_aw` feature.

use iced_aw::style::Status;
use iced_core::theme::Theme;
use native_theme::next::{PartStated, PartStyle, ResolvedTheme, WidgetState};

use super::super::borders::{line_or_none, state_line, stroke_width};
use super::super::{Mapped, fill_of, state_fill_of, state_text_color, text_color_of, to_color};
use super::{radius_or_iced, shadow};
use crate::styles::Line;
use crate::styles::aw::{self as old, CardSource, ListSource, MenuSource, TabBarSource};

/// A part's stroke and its width where the theme states the stroke, a
/// stated `Stroke::None` being no stroke ([`Line::none`]) 0 wide; `None`
/// for each where it states none, `iced_aw`'s own then.
fn edge_or_own(
    part: &PartStyle,
    color_home: &'static str,
    width_home: &'static str,
) -> Mapped<(Option<Line>, Option<f32>)> {
    if !part.stated.contains(PartStated::STROKE) {
        return Ok((None, None));
    }
    let width = stroke_width(part, width_home)?;
    Ok((Some(line_or_none(part, color_home)?), Some(width)))
}

/// [`crate::styles::aw::card`]: the card's fill and outline, the text on it
/// the window's foreground. An outline the theme leaves unstated (the table
/// of homes, `card.border.*`: `unstated_if_unset`), a fill or a foreground
/// it leaves unstated, is `iced_aw`'s own (spec §10, rule 0).
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states an outline iced cannot
/// take.
pub fn card(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::card::Style + Clone + use<>> {
    Ok(old::card_from(&card_source(r)?))
}

/// [`CardSource`] of the 0.8.0 model.
pub(super) fn card_source(r: &ResolvedTheme) -> Mapped<CardSource> {
    let c = r.card.container(WidgetState::NONE);
    let (line, width) = edge_or_own(
        &c,
        "card.container(NONE).stroke.color",
        "card.container(NONE).stroke.width",
    )?;
    Ok(CardSource {
        background: fill_of(&c, "card.container(NONE).fill")?,
        label: r.colors.window.foreground.map(to_color),
        line,
        width,
        radius: radius_or_iced(&c, "card.container(NONE).corners")?,
    })
}

/// [`crate::styles::aw::menu`]: the menu bar's fill, the menu's panel framed
/// and rounded by its own stroke and corners where the theme states them,
/// else by the popover's (spec §5, notes to the table, the menu's popup
/// frame), its shadow the first outset layer of the menu's or else the
/// popover's shadows (`shadow`); the highlight under the pointer and its
/// outline where a `HOVER` rule states them. Each value the theme leaves
/// unstated is `iced_aw`'s own (spec §10, rule 0).
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states a frame, corners or
/// highlight iced cannot take.
pub fn menu(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::menu_bar::Style + Clone + use<>> {
    Ok(old::menu_from(&menu_source(r)?))
}

/// [`MenuSource`] of the 0.8.0 model.
pub(super) fn menu_source(r: &ResolvedTheme) -> Mapped<MenuSource> {
    let m = &r.menu;
    let panel = m.panel(WidgetState::NONE);
    let popover = &r.popover.panel;
    let (frame_line, frame_width) = if panel.stated.contains(PartStated::STROKE) {
        edge_or_own(
            &panel,
            "menu.panel(NONE).stroke.color",
            "menu.panel(NONE).stroke.width",
        )?
    } else {
        edge_or_own(
            popover,
            "popover.panel.stroke.color",
            "popover.panel.stroke.width",
        )?
    };
    let frame_radius = if panel.stated.contains(PartStated::CORNERS) {
        radius_or_iced(&panel, "menu.panel(NONE).corners")?
    } else {
        radius_or_iced(popover, "popover.panel.corners")?
    };
    let hover = m.item(WidgetState::HOVER);
    let hover_line = state_line(&hover, "menu.item(HOVER).stroke.color")?;
    let hover_width = match hover_line {
        Some(_) => Some(stroke_width(&hover, "menu.item(HOVER).stroke.width")?),
        None => None,
    };
    Ok(MenuSource {
        bar: fill_of(
            &r.menu_bar.bar(WidgetState::NONE),
            "menu_bar.bar(NONE).fill",
        )?,
        panel: fill_of(&panel, "menu.panel(NONE).fill")?,
        highlight: state_fill_of(&hover, "menu.item(HOVER).fill")?,
        frame_line,
        frame_width,
        frame_radius,
        shadow: shadow(&panel).or_else(|| shadow(popover)),
        hover_line,
        hover_width,
        item_radius: radius_or_iced(&m.item(WidgetState::NONE), "menu.item(NONE).corners")?,
    })
}

/// [`crate::styles::aw::tab_bar`]: the strip, the tabs at rest, selected
/// and hovered, the selected tab's outline (spec §5, `tab.border`: the
/// selected tab's stroke and corners) and a tab's not selected
/// (`tab.tab(NONE)`). A hovered fill or label no `HOVER` rule states is as
/// `crate::styles::in_state` gives it, and an outline, a fill or a label the
/// theme leaves unstated is `iced_aw`'s own (spec §10, rule 0).
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states a value in a form iced
/// cannot draw (a layered fill with no fallback colour, a per-side stroke,
/// a concentric radius).
pub fn tab_bar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::tab_bar::Style + Clone + use<>> {
    Ok(old::tab_bar_from(&tab_bar_source(r)?))
}

/// [`TabBarSource`] of the 0.8.0 model.
pub(super) fn tab_bar_source(r: &ResolvedTheme) -> Mapped<TabBarSource> {
    let t = &r.tab;
    let selected = t.tab(WidgetState::SELECTED);
    let idle = t.tab(WidgetState::NONE);
    let (line, width) = edge_or_own(
        &selected,
        "tab.tab(SELECTED).stroke.color",
        "tab.tab(SELECTED).stroke.width",
    )?;
    let (idle_line, idle_width) = edge_or_own(
        &idle,
        "tab.tab(NONE).stroke.color",
        "tab.tab(NONE).stroke.width",
    )?;
    Ok(TabBarSource {
        bar_background: fill_of(&t.bar(WidgetState::NONE), "tab.bar(NONE).fill")?,
        background: fill_of(&idle, "tab.tab(NONE).fill")?,
        idle_line,
        idle_width,
        active_background: fill_of(&selected, "tab.tab(SELECTED).fill")?,
        hover_background: state_fill_of(&t.tab(WidgetState::HOVER), "tab.tab(HOVER).fill")?,
        label: text_color_of(&t.label(WidgetState::NONE)),
        active_label: text_color_of(&t.label(WidgetState::SELECTED)),
        hover_label: state_text_color(&t.label(WidgetState::HOVER), "tab.label(HOVER).color")?,
        line,
        width,
        radius: radius_or_iced(&selected, "tab.tab(SELECTED).corners")?,
    })
}

/// [`crate::styles::aw::sidebar`]: the panel and its outline, the items
/// selected and hovered, a hovered item lettered in
/// `sidebar.item_label(HOVER).color` where a rule states it. A hovered fill
/// or label no `HOVER` rule states, and an outline, a fill or a label the
/// theme leaves unstated, is `iced_aw`'s own (spec §10, rule 0).
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states a value in a form iced
/// cannot draw (a layered fill with no fallback colour, a per-side stroke).
pub fn sidebar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::sidebar::Style + Clone + use<>> {
    Ok(old::sidebar_from(&sidebar_source(r)?))
}

/// [`ListSource`] of the 0.8.0 model's sidebar.
pub(super) fn sidebar_source(r: &ResolvedTheme) -> Mapped<ListSource> {
    let s = &r.sidebar;
    let panel = s.container(WidgetState::NONE);
    let (line, width) = edge_or_own(
        &panel,
        "sidebar.container(NONE).stroke.color",
        "sidebar.container(NONE).stroke.width",
    )?;
    let label = text_color_of(&s.item_label(WidgetState::NONE));
    Ok(ListSource {
        background: fill_of(&panel, "sidebar.container(NONE).fill")?,
        selection_background: fill_of(
            &s.item(WidgetState::SELECTED),
            "sidebar.item(SELECTED).fill",
        )?,
        hover_background: state_fill_of(&s.item(WidgetState::HOVER), "sidebar.item(HOVER).fill")?,
        label,
        selection_label: text_color_of(&s.item_label(WidgetState::SELECTED)),
        hover_label: state_text_color(
            &s.item_label(WidgetState::HOVER),
            "sidebar.item_label(HOVER).color",
        )?,
        disabled_label: label,
        line,
        width,
    })
}

/// [`crate::styles::aw::selection_list`]: the list's fill and outline, its
/// rows hovered, selected and disabled (the row's label faded by the
/// theme's `colors.disabled_opacity`, the list stating none of its own). A
/// hovered fill or label no `HOVER` rule states, and an outline, a fill or
/// a label the theme leaves unstated, is `iced_aw`'s own (spec §10, rule
/// 0).
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states a value in a form iced
/// cannot draw (a layered fill with no fallback colour, a per-side stroke).
pub fn selection_list(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::selection_list::Style + Clone + use<>> {
    Ok(old::selection_list_from(&list_source(r)?))
}

/// [`ListSource`] of the 0.8.0 model's list.
pub(super) fn list_source(r: &ResolvedTheme) -> Mapped<ListSource> {
    let l = &r.list;
    let panel = l.container(WidgetState::NONE);
    let (line, width) = edge_or_own(
        &panel,
        "list.container(NONE).stroke.color",
        "list.container(NONE).stroke.width",
    )?;
    Ok(ListSource {
        background: fill_of(&panel, "list.container(NONE).fill")?,
        selection_background: fill_of(
            &l.highlight(WidgetState::SELECTED),
            "list.highlight(SELECTED).fill",
        )?,
        hover_background: state_fill_of(
            &l.highlight(WidgetState::HOVER),
            "list.highlight(HOVER).fill",
        )?,
        label: text_color_of(&l.item_label(WidgetState::NONE)),
        selection_label: text_color_of(&l.item_label(WidgetState::SELECTED)),
        hover_label: state_text_color(
            &l.item_label(WidgetState::HOVER),
            "list.item_label(HOVER).color",
        )?,
        disabled_label: text_color_of(&l.item_label(WidgetState::DISABLED))
            .map(|label| crate::styles::faded(label, r.colors.disabled_opacity)),
        line,
        width,
    })
}

/// [`crate::styles::aw::spinner`]: the spinner's colour.
#[must_use = "this returns the style function; it does not apply it"]
pub fn spinner(
    r: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    old::spinner_from(to_color(r.spinner.color))
}
