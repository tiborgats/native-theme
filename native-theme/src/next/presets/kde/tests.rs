use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, ListQuery, StyleQuery};

/// The resolved `kde-breeze` of both schemes, from its constants.
fn both_schemes() -> [ResolvedTheme; 2] {
    [ThemeContext::light(), ThemeContext::dark()]
        .map(|ctx| Theme::preset(Preset::KdeBreeze).resolve(&ctx))
}

#[test]
fn a_primary_button_keeps_the_accent_fill_when_pressed_checked_or_default() {
    for theme in both_schemes() {
        for state in [
            WidgetState::PRESSED,
            WidgetState::CHECKED,
            WidgetState::DEFAULT,
        ] {
            let query = ButtonQuery::primary().state(state);
            assert_eq!(
                theme.button.body(query).fill,
                Paint::Solid(theme.colors.accent.fill),
                "{:?} {state:?}",
                theme.context.scheme
            );
            assert_eq!(
                theme.button.label(query).color,
                theme.colors.accent.foreground,
                "{:?} {state:?}",
                theme.context.scheme
            );
            // A button of the default variant keeps its own fill in that state.
            assert_ne!(
                theme.button.body(ButtonQuery::new(state)).fill,
                Paint::Solid(theme.colors.accent.fill),
                "{:?} {state:?}",
                theme.context.scheme
            );
        }
        // Disabled, the primary button takes the disabled fill.
        let disabled = ButtonQuery::primary().state(WidgetState::DISABLED);
        assert_eq!(
            theme.button.body(disabled).fill,
            theme
                .button
                .body(ButtonQuery::new(WidgetState::DISABLED))
                .fill
        );
    }
}

#[test]
fn a_selected_row_has_no_hover_outline() {
    for theme in both_schemes() {
        let hover = theme.list.row(ListQuery::new(WidgetState::HOVER));
        let rest = theme.list.row(ListQuery::new(WidgetState::NONE));
        assert_ne!(hover.stroke, rest.stroke, "{:?}", theme.context.scheme);
        let selected = WidgetState::SELECTED;
        let hovered_selected = theme
            .list
            .row(ListQuery::new(selected.union(WidgetState::HOVER)));
        assert_eq!(
            hovered_selected.stroke,
            theme.list.row(ListQuery::new(selected)).stroke,
            "{:?}",
            theme.context.scheme
        );
        assert_eq!(
            hovered_selected.fill,
            Paint::Solid(theme.colors.selection.background)
        );
    }
}

#[test]
fn pressed_changes_nothing_on_a_part_without_a_pressed_rule() {
    let pressed_hover = WidgetState::PRESSED.union(WidgetState::HOVER);
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        for checked in [WidgetState::NONE, WidgetState::CHECKED] {
            let hover = StyleQuery::<()>::new(WidgetState::HOVER.union(checked));
            let both = StyleQuery::<()>::new(pressed_hover.union(checked));
            assert_eq!(
                theme.checkbox.indicator(both),
                theme.checkbox.indicator(hover),
                "{scheme:?} {checked:?}"
            );
            assert_eq!(
                theme.radio.indicator(both),
                theme.radio.indicator(hover),
                "{scheme:?} {checked:?}"
            );
        }
        for alternate in [false, true] {
            assert_eq!(
                theme
                    .list
                    .row(ListQuery::new(pressed_hover).alternate(alternate)),
                theme
                    .list
                    .row(ListQuery::new(WidgetState::HOVER).alternate(alternate)),
                "{scheme:?} alternate {alternate}"
            );
        }
        // Parts with a pressed rule keep pressed over hover.
        assert_eq!(
            theme.list.header(ListQuery::new(pressed_hover)),
            theme.list.header(ListQuery::new(WidgetState::PRESSED))
        );
        assert_eq!(
            theme.scrollbar.thumb(pressed_hover),
            theme.scrollbar.thumb(WidgetState::PRESSED)
        );
    }
}
