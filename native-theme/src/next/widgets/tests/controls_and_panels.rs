//! The tests of `popover`, `switch` and `dialog`.

use super::*;
use crate::next::layout::DialogButtonOrder;
use crate::next::motion::DialogEntranceStyle;

#[test]
fn switch_getters_evaluate_their_rule_sets() {
    let track = part_rules(RED, GREEN, BLUE, ());
    let thumb = part_rules(GREEN, BLUE, RED, ());
    let label = text_rules(BLUE, RED);
    let switch = SwitchWidget::new(
        WidgetOrigin::Native,
        track.clone(),
        thumb.clone(),
        label.clone(),
        Some(6.0),
        Some(true),
        Some(1.0),
    );
    assert!(switch.is_native());
    assert_eq!(switch.label_gap, Some(6.0));
    assert_eq!(switch.show_state_glyphs, Some(true));
    assert_eq!(switch.disabled_opacity, Some(1.0));
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(switch.track(query), track.eval(&query));
        assert_eq!(switch.thumb(query), thumb.eval(&query));
        assert_eq!(switch.label(query), label.eval(&query));
        assert_eq!(
            switch.parts(query),
            SwitchParts {
                track: switch.track(query),
                thumb: switch.thumb(query),
                label: switch.label(query),
            }
        );
    }
}

#[test]
fn popover_holds_its_parts() {
    let panel = PartStyle {
        fill: Paint::Solid(RED),
        ..PartStyle::default()
    };
    let arrow = PartStyle {
        fill: Paint::Solid(GREEN),
        ..PartStyle::default()
    };
    let text = TextStyle {
        color: BLUE,
        ..TextStyle::default()
    };
    let popover = PopoverWidget::new(
        WidgetOrigin::Synthesized,
        panel.clone(),
        arrow.clone(),
        text.clone(),
    );
    assert!(!popover.is_native());
    assert_eq!(popover.panel, panel);
    assert_eq!(popover.arrow, arrow);
    assert_eq!(popover.text, text);
}

#[test]
fn dialog_holds_its_parts_and_metrics() {
    let panel = PartStyle {
        fill: Paint::Solid(RED),
        ..PartStyle::default()
    };
    let title = TextStyle {
        color: GREEN,
        ..TextStyle::default()
    };
    let body = TextStyle {
        color: BLUE,
        ..TextStyle::default()
    };
    let icon = IconStyle {
        color: RED,
        ..IconStyle::default()
    };
    let dialog = DialogWidget::new(
        WidgetOrigin::Native,
        panel.clone(),
        title.clone(),
        body.clone(),
        icon.clone(),
        12.0,
        Some(DialogButtonOrder::PrimaryLeft),
        Some(DialogEntranceStyle::FadeRise),
    );
    assert!(dialog.is_native());
    assert_eq!(dialog.panel, panel);
    assert_eq!(dialog.title, title);
    assert_eq!(dialog.body, body);
    assert_eq!(dialog.icon, icon);
    assert_eq!(dialog.button_gap, 12.0);
    assert_eq!(dialog.button_order, Some(DialogButtonOrder::PrimaryLeft));
    assert_eq!(dialog.entrance, Some(DialogEntranceStyle::FadeRise));
}

/// A widget with nothing stated states nothing at rest or in a state.
#[test]
fn the_unstated_controls_and_panels_state_nothing() {
    let theme = nothing_stated(Preset::KdeBreeze, ThemeContext::light());
    for state in states() {
        let q = StyleQuery::new(state);
        let switch = theme.switch.parts(q);
        assert!(switch.track.stated.is_empty() && switch.thumb.stated.is_empty());
        assert!(switch.label.stated.is_empty());
    }
    let popover = &theme.popover;
    assert!(popover.panel.stated.is_empty() && popover.arrow.stated.is_empty());
    assert!(popover.text.stated.is_empty());
    let dialog = &theme.dialog;
    assert!(dialog.panel.stated.is_empty() && dialog.title.stated.is_empty());
    assert!(dialog.body.stated.is_empty() && dialog.icon.stated.is_empty());
    assert!(theme.switch.is_native() && popover.is_native() && dialog.is_native());
    assert_eq!(theme.switch.label_gap, None);
    assert_eq!(theme.switch.show_state_glyphs, None);
    assert_eq!(theme.switch.disabled_opacity, None);
    assert_eq!(dialog.button_order, None);
    assert_eq!(dialog.entrance, None);
}

// Spec §1, "Thread safety & traits".
#[test]
fn controls_and_panels_are_clone_debug_eq_send_sync() {
    assert_send_sync_clone_debug_eq::<SwitchWidget>();
    assert_send_sync_clone_debug_eq::<PopoverWidget>();
    assert_send_sync_clone_debug_eq::<DialogWidget>();
}
