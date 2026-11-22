//! The tests of `menu`, `menu_bar` and `tab`.

use super::*;
use crate::next::layout::TabIndicatorSide;
use crate::next::query::{TabQuery, TabVariant};

#[test]
fn menu_getters_evaluate_their_rule_sets() {
    let panel = part_rules(RED, GREEN, BLUE, ());
    let item = part_rules(GREEN, BLUE, RED, ());
    let item_label = text_rules(BLUE, RED);
    let shortcut = text_rules(RED, GREEN);
    let icon = icon_rules(GREEN, BLUE);
    let submenu_arrow = icon_rules(BLUE, GREEN);
    let separator = part_rules(BLUE, RED, GREEN, ());
    let menu = MenuWidget::new(
        WidgetOrigin::Native,
        panel.clone(),
        item.clone(),
        item_label.clone(),
        shortcut.clone(),
        icon.clone(),
        submenu_arrow.clone(),
        separator.clone(),
        8.0,
    );
    assert!(menu.is_native());
    assert_eq!(menu.icon_text_gap, 8.0);
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(menu.panel(query), panel.eval(&query));
        assert_eq!(menu.item(query), item.eval(&query));
        assert_eq!(menu.item_label(query), item_label.eval(&query));
        assert_eq!(menu.shortcut(query), shortcut.eval(&query));
        assert_eq!(menu.icon(query), icon.eval(&query));
        assert_eq!(menu.submenu_arrow(query), submenu_arrow.eval(&query));
        assert_eq!(menu.separator(query), separator.eval(&query));
        assert_eq!(
            menu.parts(query),
            MenuParts {
                panel: menu.panel(query),
                item: menu.item(query),
                item_label: menu.item_label(query),
                shortcut: menu.shortcut(query),
                icon: menu.icon(query),
                submenu_arrow: menu.submenu_arrow(query),
                separator: menu.separator(query),
            }
        );
    }
}

#[test]
fn menu_bar_getters_evaluate_their_rule_sets() {
    let bar = part_rules(RED, GREEN, BLUE, ());
    let item = part_rules(GREEN, BLUE, RED, ());
    let item_label = text_rules(BLUE, RED);
    let menu_bar = MenuBarWidget::new(
        WidgetOrigin::Synthesized,
        bar.clone(),
        item.clone(),
        item_label.clone(),
        Some(2.0),
    );
    assert!(!menu_bar.is_native());
    assert_eq!(menu_bar.item_gap, Some(2.0));
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(menu_bar.bar(query), bar.eval(&query));
        assert_eq!(menu_bar.item(query), item.eval(&query));
        assert_eq!(menu_bar.item_label(query), item_label.eval(&query));
        assert_eq!(
            menu_bar.parts(query),
            MenuBarParts {
                bar: menu_bar.bar(query),
                item: menu_bar.item(query),
                item_label: menu_bar.item_label(query),
            }
        );
    }
}

#[test]
fn tab_getters_evaluate_their_rule_sets() {
    let document = Some(TabVariant::Document);
    let bar = part_rules(RED, GREEN, BLUE, document);
    let tab = part_rules(GREEN, BLUE, RED, document);
    let label = text_rules(BLUE, RED);
    let indicator = part_rules(BLUE, GREEN, RED, document);
    let close_button = part_rules(RED, BLUE, GREEN, document);
    let close_icon = icon_rules(GREEN, RED);
    let tabs = TabWidget::new(
        WidgetOrigin::Native,
        bar.clone(),
        tab.clone(),
        label.clone(),
        indicator.clone(),
        close_button.clone(),
        close_icon.clone(),
        Some(0.0),
        Some(TabIndicatorSide::Top),
        None,
    );
    assert!(tabs.is_native());
    assert_eq!(tabs.item_gap, Some(0.0));
    assert_eq!(tabs.indicator_side, Some(TabIndicatorSide::Top));
    assert_eq!(tabs.active_outward_corner_radius, None);
    for state in states() {
        for variant in [TabVariant::Standard, TabVariant::Document] {
            let query = TabQuery::new(state).variant(variant);
            assert_eq!(tabs.bar(query), bar.eval(&query));
            assert_eq!(tabs.tab(query), tab.eval(&query));
            assert_eq!(tabs.label(query), label.eval(&query));
            assert_eq!(tabs.indicator(query), indicator.eval(&query));
            assert_eq!(tabs.close_button(query), close_button.eval(&query));
            assert_eq!(tabs.close_icon(query), close_icon.eval(&query));
            assert_eq!(
                tabs.parts(query),
                TabParts {
                    bar: tabs.bar(query),
                    tab: tabs.tab(query),
                    label: tabs.label(query),
                    indicator: tabs.indicator(query),
                    close_button: tabs.close_button(query),
                    close_icon: tabs.close_icon(query),
                }
            );
        }
    }
    // The variant reaches the rule: a pressed tab fills red only in a document's tab bar.
    let pressed = WidgetState::PRESSED;
    assert_eq!(tabs.tab(pressed).fill, Paint::Solid(GREEN));
    assert_eq!(
        tabs.tab(TabQuery::new(pressed).variant(TabVariant::Document))
            .fill,
        Paint::Solid(RED)
    );
}

/// A widget with nothing stated states nothing at rest or in a state.
#[test]
fn the_unstated_widgets_state_nothing() {
    let theme = nothing_stated(Preset::KdeBreeze, ThemeContext::light());
    for state in states() {
        let q = StyleQuery::new(state);
        let menu = theme.menu.parts(q);
        assert!(menu.panel.stated.is_empty() && menu.item.stated.is_empty());
        assert!(menu.item_label.stated.is_empty() && menu.shortcut.stated.is_empty());
        assert!(menu.icon.stated.is_empty() && menu.submenu_arrow.stated.is_empty());
        assert!(menu.separator.stated.is_empty());
        let menu_bar = theme.menu_bar.parts(q);
        assert!(menu_bar.bar.stated.is_empty() && menu_bar.item.stated.is_empty());
        assert!(menu_bar.item_label.stated.is_empty());
        let tab = theme.tab.parts(TabQuery::new(state));
        assert!(tab.bar.stated.is_empty() && tab.tab.stated.is_empty());
        assert!(tab.label.stated.is_empty() && tab.indicator.stated.is_empty());
        assert!(tab.close_button.stated.is_empty() && tab.close_icon.stated.is_empty());
    }
    assert!(theme.menu.is_native() && theme.menu_bar.is_native() && theme.tab.is_native());
    assert_eq!(theme.menu_bar.item_gap, None);
    assert_eq!(theme.tab.item_gap, None);
    assert_eq!(theme.tab.indicator_side, None);
    assert_eq!(theme.tab.active_outward_corner_radius, None);
}

// Spec §1, "Thread safety & traits".
#[test]
fn menus_tabs_and_bars_are_clone_debug_eq_send_sync() {
    assert_send_sync_clone_debug_eq::<MenuWidget>();
    assert_send_sync_clone_debug_eq::<MenuBarWidget>();
    assert_send_sync_clone_debug_eq::<TabWidget>();
}
