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
use super::super::{Mapped, fill, state_fill, state_text_color, stated, text_color, to_color};
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
/// of homes, `card.border.*`: `unstated_if_unset`) is `iced_aw`'s own.
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states no card fill or window
/// foreground, or an outline iced cannot take.
pub fn card(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::card::Style + Clone + use<>> {
    let c = r.card.container(WidgetState::NONE);
    let (line, width) = edge_or_own(
        &c,
        "card.container(NONE).stroke.color",
        "card.container(NONE).stroke.width",
    )?;
    Ok(old::card_from(&CardSource {
        background: fill(&c, "card.container(NONE).fill")?,
        label: to_color(stated(
            r.colors.window.foreground,
            "colors.window.foreground",
        )?),
        line,
        width,
        radius: radius_or_iced(&c, "card.container(NONE).corners")?,
    }))
}

/// [`crate::styles::aw::menu`]: the menu bar's fill, the menu's panel framed
/// and rounded by its own stroke and corners where the theme states them,
/// else by the popover's (spec §5, notes to the table, the menu's popup
/// frame), its shadow the first outset layer of the menu's or else the
/// popover's shadows (`shadow`); the highlight under the pointer and its
/// outline where a `HOVER` rule states them. Each value the theme leaves
/// unstated is `iced_aw`'s own.
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states no bar or panel fill,
/// or a frame, corners or highlight iced cannot take.
pub fn menu(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::menu_bar::Style + Clone + use<>> {
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
    Ok(old::menu_from(&MenuSource {
        bar: fill(
            &r.menu_bar.bar(WidgetState::NONE),
            "menu_bar.bar(NONE).fill",
        )?,
        panel: fill(&panel, "menu.panel(NONE).fill")?,
        highlight: state_fill(&hover, "menu.item(HOVER).fill")?,
        frame_line,
        frame_width,
        frame_radius,
        shadow: shadow(&panel).or_else(|| shadow(popover)),
        hover_line,
        hover_width,
        item_radius: radius_or_iced(&m.item(WidgetState::NONE), "menu.item(NONE).corners")?,
    }))
}

/// [`crate::styles::aw::tab_bar`]: the strip, the tabs at rest, selected
/// and hovered, the selected tab's outline (spec §5, `tab.border`: the
/// selected tab's stroke and corners) and a tab's not selected
/// (`tab.tab(NONE)`). A hovered fill or label no `HOVER` rule states is as
/// `crate::styles::in_state` gives it, and an outline the theme leaves
/// unstated is `iced_aw`'s own.
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states no value the bar needs.
pub fn tab_bar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::tab_bar::Style + Clone + use<>> {
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
    Ok(old::tab_bar_from(&TabBarSource {
        bar_background: fill(&t.bar(WidgetState::NONE), "tab.bar(NONE).fill")?,
        background: fill(&idle, "tab.tab(NONE).fill")?,
        idle_line,
        idle_width,
        active_background: fill(&selected, "tab.tab(SELECTED).fill")?,
        hover_background: state_fill(&t.tab(WidgetState::HOVER), "tab.tab(HOVER).fill")?,
        label: text_color(&t.label(WidgetState::NONE), "tab.label(NONE).color")?,
        active_label: text_color(&t.label(WidgetState::SELECTED), "tab.label(SELECTED).color")?,
        hover_label: state_text_color(&t.label(WidgetState::HOVER), "tab.label(HOVER).color")?,
        line,
        width,
        radius: radius_or_iced(&selected, "tab.tab(SELECTED).corners")?,
    }))
}

/// [`crate::styles::aw::sidebar`]: the panel and its outline, the items
/// selected and hovered. A hovered fill no `HOVER` rule states, and an
/// outline the theme leaves unstated, is `iced_aw`'s own.
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states no value the sidebar
/// needs.
pub fn sidebar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::sidebar::Style + Clone + use<>> {
    let s = &r.sidebar;
    let panel = s.container(WidgetState::NONE);
    let (line, width) = edge_or_own(
        &panel,
        "sidebar.container(NONE).stroke.color",
        "sidebar.container(NONE).stroke.width",
    )?;
    let label = text_color(
        &s.item_label(WidgetState::NONE),
        "sidebar.item_label(NONE).color",
    )?;
    Ok(old::sidebar_from(&ListSource {
        background: fill(&panel, "sidebar.container(NONE).fill")?,
        selection_background: fill(
            &s.item(WidgetState::SELECTED),
            "sidebar.item(SELECTED).fill",
        )?,
        hover_background: state_fill(&s.item(WidgetState::HOVER), "sidebar.item(HOVER).fill")?,
        label,
        selection_label: text_color(
            &s.item_label(WidgetState::SELECTED),
            "sidebar.item_label(SELECTED).color",
        )?,
        hover_label: Some(label),
        disabled_label: label,
        line,
        width,
    }))
}

/// [`crate::styles::aw::selection_list`]: the list's fill and outline, its
/// rows hovered, selected and disabled (the row's label faded by the
/// theme's `colors.disabled_opacity`, the list stating none of its own). A
/// hovered fill or label no `HOVER` rule states, and an outline the theme
/// leaves unstated, is `iced_aw`'s own.
///
/// # Errors
///
/// [`super::super::Unmapped`] where the theme states no value the list
/// needs.
pub fn selection_list(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, Status) -> iced_aw::style::selection_list::Style + Clone + use<>> {
    let l = &r.list;
    let panel = l.container(WidgetState::NONE);
    let (line, width) = edge_or_own(
        &panel,
        "list.container(NONE).stroke.color",
        "list.container(NONE).stroke.width",
    )?;
    Ok(old::selection_list_from(&ListSource {
        background: fill(&panel, "list.container(NONE).fill")?,
        selection_background: fill(
            &l.highlight(WidgetState::SELECTED),
            "list.highlight(SELECTED).fill",
        )?,
        hover_background: state_fill(
            &l.highlight(WidgetState::HOVER),
            "list.highlight(HOVER).fill",
        )?,
        label: text_color(
            &l.item_label(WidgetState::NONE),
            "list.item_label(NONE).color",
        )?,
        selection_label: text_color(
            &l.item_label(WidgetState::SELECTED),
            "list.item_label(SELECTED).color",
        )?,
        hover_label: state_text_color(
            &l.item_label(WidgetState::HOVER),
            "list.item_label(HOVER).color",
        )?,
        disabled_label: crate::styles::faded(
            text_color(
                &l.item_label(WidgetState::DISABLED),
                "list.item_label(DISABLED).color",
            )?,
            r.colors.disabled_opacity,
        ),
        line,
        width,
    }))
}

/// [`crate::styles::aw::spinner`]: the spinner's colour.
#[must_use = "this returns the style function; it does not apply it"]
pub fn spinner(
    r: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    old::spinner_from(to_color(r.spinner.color))
}
