//! The tests of `popover`, `switch`, `dialog`, `combo_box` and
//! `segmented_control`.

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
fn combo_box_getters_evaluate_their_rule_sets() {
    let field = part_rules(RED, GREEN, BLUE, ());
    let label = text_rules(BLUE, RED);
    let chevron = icon_rules(GREEN, BLUE);
    let combo_box = ComboBoxWidget::new(
        WidgetOrigin::Native,
        field.clone(),
        label.clone(),
        chevron.clone(),
        Some(20.0),
        None,
    );
    assert!(combo_box.is_native());
    assert_eq!(combo_box.arrow_area_width, Some(20.0));
    assert_eq!(combo_box.disabled_opacity, None);
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(combo_box.field(query), field.eval(&query));
        assert_eq!(combo_box.label(query), label.eval(&query));
        assert_eq!(combo_box.chevron(query), chevron.eval(&query));
        assert_eq!(
            combo_box.parts(query),
            ComboBoxParts {
                field: combo_box.field(query),
                label: combo_box.label(query),
                chevron: combo_box.chevron(query),
            }
        );
    }
}

#[test]
fn segmented_control_getters_evaluate_their_rule_sets() {
    let track = part_rules(RED, GREEN, BLUE, ());
    let segment = part_rules(GREEN, BLUE, RED, ());
    let label = text_rules(BLUE, RED);
    let separator = part_rules(BLUE, RED, GREEN, ());
    let control = SegmentedControlWidget::new(
        WidgetOrigin::Synthesized,
        track.clone(),
        segment.clone(),
        label.clone(),
        separator.clone(),
        1.0,
        Some(0.5),
    );
    assert!(!control.is_native());
    assert_eq!(control.separator_width, 1.0);
    assert_eq!(control.disabled_opacity, Some(0.5));
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(control.track(query), track.eval(&query));
        assert_eq!(control.segment(query), segment.eval(&query));
        assert_eq!(control.label(query), label.eval(&query));
        assert_eq!(control.separator(query), separator.eval(&query));
        assert_eq!(
            control.parts(query),
            SegmentedControlParts {
                track: control.track(query),
                segment: control.segment(query),
                label: control.label(query),
                separator: control.separator(query),
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
        let combo_box = theme.combo_box.parts(q);
        assert!(combo_box.field.stated.is_empty() && combo_box.label.stated.is_empty());
        assert!(combo_box.chevron.stated.is_empty());
        let control = theme.segmented_control.parts(q);
        assert!(control.track.stated.is_empty() && control.segment.stated.is_empty());
        assert!(control.label.stated.is_empty() && control.separator.stated.is_empty());
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
    assert!(theme.combo_box.is_native() && theme.segmented_control.is_native());
    assert_eq!(theme.combo_box.arrow_area_width, None);
    assert_eq!(theme.combo_box.disabled_opacity, None);
    assert_eq!(theme.segmented_control.disabled_opacity, None);
}

// Spec §1, "Thread safety & traits".
#[test]
fn controls_and_panels_are_clone_debug_eq_send_sync() {
    assert_send_sync_clone_debug_eq::<SwitchWidget>();
    assert_send_sync_clone_debug_eq::<PopoverWidget>();
    assert_send_sync_clone_debug_eq::<DialogWidget>();
    assert_send_sync_clone_debug_eq::<ComboBoxWidget>();
    assert_send_sync_clone_debug_eq::<SegmentedControlWidget>();
}
