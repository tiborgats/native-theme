use super::*;
use crate::next::paint::Paint;
use crate::next::preset::nothing_stated;
use crate::next::query::{
    ButtonKind, ButtonVariant, ListKind, ListVariant, Orientation, WidgetState,
};
use crate::next::rules::RuleSelector;

const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

/// A selector on `states`, every axis open.
fn on<M: Default>(states: WidgetState) -> RuleSelector<M> {
    RuleSelector {
        required_states: states,
        ..RuleSelector::default()
    }
}

/// A part rule set: a fill of `base`, then `HOVER` filling with `hover` and
/// `PRESSED` with `pressed`, the last only for `matcher`.
fn part_rules<M: Copy + Default>(
    base: Color,
    hover: Color,
    pressed: Color,
    matcher: M,
) -> PartRules<M> {
    RuleSet::new(
        PartStyle {
            fill: Paint::Solid(base),
            ..PartStyle::default()
        },
        &[],
        vec![
            (
                on(WidgetState::HOVER),
                vec![PartProp::Fill(Paint::Solid(hover))],
            ),
            (
                RuleSelector {
                    matcher,
                    ..on(WidgetState::PRESSED)
                },
                vec![PartProp::Fill(Paint::Solid(pressed))],
            ),
        ],
    )
}

/// A text rule set: `base` colour, `HOVER` giving `hover`.
fn text_rules<M: Default>(base: Color, hover: Color) -> TextRules<M> {
    RuleSet::new(
        TextStyle {
            color: base,
            ..TextStyle::default()
        },
        &[],
        vec![(on(WidgetState::HOVER), vec![TextProp::Color(hover)])],
    )
}

/// An icon rule set: `base` colour, `CHECKED` giving `checked`.
fn icon_rules<M: Default>(base: Color, checked: Color) -> IconRules<M> {
    RuleSet::new(
        IconStyle {
            color: base,
            ..IconStyle::default()
        },
        &[],
        vec![(on(WidgetState::CHECKED), vec![IconProp::Color(checked)])],
    )
}

fn states() -> [WidgetState; 5] {
    [
        WidgetState::NONE,
        WidgetState::HOVER,
        WidgetState::PRESSED,
        WidgetState::CHECKED,
        WidgetState::HOVER | WidgetState::PRESSED | WidgetState::CHECKED,
    ]
}

#[test]
fn button_getters_evaluate_their_rule_sets() {
    let primary = ButtonMatcher {
        variant: Some(ButtonVariant::Primary),
        shape: None,
    };
    let body = part_rules(RED, GREEN, BLUE, primary);
    let label = text_rules(GREEN, BLUE);
    let icon = icon_rules(BLUE, RED);
    let button = ButtonWidget::new(
        WidgetOrigin::Native,
        body.clone(),
        label.clone(),
        icon.clone(),
        4.0,
        Some(1.0),
    );
    assert!(button.is_native());
    assert_eq!(button.icon_text_gap, 4.0);
    assert_eq!(button.disabled_opacity, Some(1.0));
    for state in states() {
        for variant in [ButtonVariant::Standard, ButtonVariant::Primary] {
            let query = ButtonQuery::new(state).variant(ButtonKind {
                variant,
                ..ButtonKind::default()
            });
            assert_eq!(button.body(query), body.eval(&query));
            assert_eq!(button.label(query), label.eval(&query));
            assert_eq!(button.icon(query), icon.eval(&query));
            assert_eq!(
                button.parts(query),
                ButtonParts {
                    body: button.body(query),
                    label: button.label(query),
                    icon: button.icon(query),
                }
            );
        }
    }
    // The variant reaches the rule: pressed fills blue only on a primary button.
    let pressed = WidgetState::PRESSED;
    assert_eq!(button.body(pressed).fill, Paint::Solid(RED));
    assert_eq!(
        button.body(ButtonQuery::primary().state(pressed)).fill,
        Paint::Solid(BLUE)
    );
    assert_eq!(button.body(ButtonVariant::Primary).fill, Paint::Solid(RED));
}

#[test]
fn checkbox_getters_evaluate_their_rule_sets() {
    let indicator = part_rules(RED, GREEN, BLUE, ());
    let checkmark = icon_rules(GREEN, BLUE);
    let label = text_rules(BLUE, RED);
    let checkbox = CheckboxWidget::new(
        WidgetOrigin::Synthesized,
        indicator.clone(),
        checkmark.clone(),
        label.clone(),
        6.0,
        None,
    );
    assert!(!checkbox.is_native());
    assert_eq!(checkbox.label_gap, 6.0);
    assert_eq!(checkbox.disabled_opacity, None);
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(checkbox.indicator(state), indicator.eval(&query));
        assert_eq!(checkbox.checkmark(state), checkmark.eval(&query));
        assert_eq!(checkbox.label(state), label.eval(&query));
        assert_eq!(
            checkbox.parts(state),
            CheckboxParts {
                indicator: checkbox.indicator(state),
                checkmark: checkbox.checkmark(state),
                label: checkbox.label(state),
            }
        );
    }
    assert_eq!(checkbox.checkmark(WidgetState::CHECKED).color, BLUE);
}

#[test]
fn radio_getters_evaluate_their_rule_sets() {
    let indicator = part_rules(RED, GREEN, BLUE, ());
    let dot = part_rules(GREEN, BLUE, RED, ());
    let label = text_rules(BLUE, GREEN);
    let radio = RadioWidget::new(
        WidgetOrigin::Native,
        indicator.clone(),
        dot.clone(),
        label.clone(),
        4.0,
        Some(0.5),
    );
    assert!(radio.is_native());
    assert_eq!(radio.label_gap, 4.0);
    assert_eq!(radio.disabled_opacity, Some(0.5));
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(radio.indicator(query), indicator.eval(&query));
        assert_eq!(radio.dot(query), dot.eval(&query));
        assert_eq!(radio.label(query), label.eval(&query));
        assert_eq!(
            radio.parts(query),
            RadioParts {
                indicator: radio.indicator(query),
                dot: radio.dot(query),
                label: radio.label(query),
            }
        );
    }
}

#[test]
fn scrollbar_getters_evaluate_their_rule_sets() {
    let track = part_rules(RED, GREEN, BLUE, ());
    let thumb = part_rules(BLUE, RED, GREEN, ());
    let scrollbar = ScrollbarWidget::new(
        WidgetOrigin::Native,
        track.clone(),
        thumb.clone(),
        20.0,
        false,
    );
    assert!(scrollbar.is_native());
    assert_eq!(scrollbar.min_thumb_length, 20.0);
    assert!(!scrollbar.overlay_mode);
    for state in states() {
        for orientation in [Orientation::Horizontal, Orientation::Vertical] {
            let query = StyleQuery::new(state).orientation(orientation);
            assert_eq!(scrollbar.track(query), track.eval(&query));
            assert_eq!(scrollbar.thumb(query), thumb.eval(&query));
            assert_eq!(
                scrollbar.parts(query),
                ScrollbarParts {
                    track: scrollbar.track(query),
                    thumb: scrollbar.thumb(query),
                }
            );
        }
    }
}

#[test]
fn list_getters_evaluate_their_rule_sets() {
    let alternate = ListMatcher {
        variant: None,
        alternate: Some(true),
    };
    let container = part_rules(RED, GREEN, BLUE, alternate);
    let row = part_rules(GREEN, BLUE, RED, alternate);
    let highlight = part_rules(RED, BLUE, GREEN, alternate);
    let item_label = text_rules(BLUE, RED);
    let header = part_rules(BLUE, GREEN, RED, alternate);
    let header_label = text_rules(RED, GREEN);
    let header_separator = part_rules(GREEN, RED, BLUE, alternate);
    let list = ListWidget::new(
        WidgetOrigin::Native,
        container.clone(),
        row.clone(),
        highlight.clone(),
        item_label.clone(),
        header.clone(),
        header_label.clone(),
        header_separator.clone(),
        RED,
    );
    assert!(list.is_native());
    assert_eq!(list.grid_color, RED);
    for state in states() {
        for alternate in [false, true] {
            let query = ListQuery::new(state).variant(ListKind {
                variant: ListVariant::Plain,
                alternate,
            });
            assert_eq!(list.container(query), container.eval(&query));
            assert_eq!(list.row(query), row.eval(&query));
            assert_eq!(list.highlight(query), highlight.eval(&query));
            assert_eq!(list.item_label(query), item_label.eval(&query));
            assert_eq!(list.header(query), header.eval(&query));
            assert_eq!(list.header_label(query), header_label.eval(&query));
            assert_eq!(list.header_separator(query), header_separator.eval(&query));
            assert_eq!(
                list.parts(query),
                ListParts {
                    container: list.container(query),
                    row: list.row(query),
                    highlight: list.highlight(query),
                    item_label: list.item_label(query),
                    header: list.header(query),
                    header_label: list.header_label(query),
                    header_separator: list.header_separator(query),
                }
            );
        }
    }
    // The alternate flag reaches the rule: a pressed row fills red only where it is alternate.
    let pressed = ListQuery::new(WidgetState::PRESSED);
    assert_eq!(list.row(pressed).fill, Paint::Solid(GREEN));
    assert_eq!(list.row(pressed.alternate(true)).fill, Paint::Solid(RED));
}

#[test]
fn tooltip_holds_its_parts() {
    let panel = PartStyle {
        fill: Paint::Solid(RED),
        ..PartStyle::default()
    };
    let text = TextStyle {
        color: BLUE,
        ..TextStyle::default()
    };
    let tooltip = TooltipWidget::new(WidgetOrigin::Synthesized, panel.clone(), text.clone());
    assert!(!tooltip.is_native());
    assert_eq!(tooltip.panel, panel);
    assert_eq!(tooltip.text, text);
}

fn assert_send_sync_clone_debug_eq<T: Clone + std::fmt::Debug + PartialEq + Send + Sync>() {}

// Spec §1, "Thread safety & traits".
#[test]
fn widgets_and_resolved_theme_are_clone_debug_eq_send_sync() {
    assert_send_sync_clone_debug_eq::<ButtonWidget>();
    assert_send_sync_clone_debug_eq::<CheckboxWidget>();
    assert_send_sync_clone_debug_eq::<RadioWidget>();
    assert_send_sync_clone_debug_eq::<ScrollbarWidget>();
    assert_send_sync_clone_debug_eq::<ListWidget>();
    assert_send_sync_clone_debug_eq::<TooltipWidget>();
    assert_send_sync_clone_debug_eq::<ResolvedTheme>();
}

#[test]
fn scheme_and_is_dark_follow_the_context() {
    let light = nothing_stated(Preset::KdeBreeze, ThemeContext::light());
    assert_eq!(light.scheme(), Scheme::Light);
    assert!(!light.is_dark());
    let dark = nothing_stated(Preset::KdeBreeze, ThemeContext::dark());
    assert_eq!(dark.scheme(), Scheme::Dark);
    assert!(dark.is_dark());
}

#[test]
fn elevation_reads_each_level_and_is_empty_beyond_the_last() {
    let layer = |blur: f32| ShadowLayer {
        offset_x: 0.0,
        offset_y: 1.0,
        blur,
        spread: 0.0,
        color: RED,
        inset: false,
    };
    let none: Arc<[ShadowLayer]> = Arc::from([]);
    let one: Arc<[ShadowLayer]> = Arc::from([layer(2.0)]);
    let two: Arc<[ShadowLayer]> = Arc::from([layer(4.0), layer(8.0)]);
    let theme = ResolvedTheme {
        elevations: Arc::from([none, one, two]),
        ..nothing_stated(Preset::KdeBreeze, ThemeContext::light())
    };
    assert!(theme.elevation(0).is_empty());
    assert_eq!(theme.elevation(1), &[layer(2.0)]);
    assert_eq!(theme.elevation(2), &[layer(4.0), layer(8.0)]);
    assert!(theme.elevation(3).is_empty());
    assert!(theme.elevation(usize::MAX).is_empty());
    let flat = nothing_stated(Preset::KdeBreeze, ThemeContext::light());
    assert!(flat.elevation(0).is_empty());
}

#[test]
fn input_getters_evaluate_their_rule_sets() {
    use crate::next::query::{InputQuery, InputVariant};
    let field = part_rules(RED, GREEN, BLUE, Some(InputVariant::Search));
    let text = text_rules(GREEN, BLUE);
    let placeholder = text_rules(BLUE, RED);
    let helper_text = text_rules(RED, BLUE);
    let input = InputWidget::new(
        WidgetOrigin::Native,
        field.clone(),
        text.clone(),
        placeholder.clone(),
        helper_text.clone(),
        Some(1.0),
    );
    assert!(input.is_native());
    assert_eq!(input.disabled_opacity, Some(1.0));
    for state in states() {
        for variant in [InputVariant::Standard, InputVariant::Search] {
            let query = InputQuery::new(state).variant(variant);
            assert_eq!(input.field(query), field.eval(&query));
            assert_eq!(input.text(query), text.eval(&query));
            assert_eq!(input.placeholder(query), placeholder.eval(&query));
            assert_eq!(input.helper_text(query), helper_text.eval(&query));
            assert_eq!(
                input.parts(query),
                InputParts {
                    field: input.field(query),
                    text: input.text(query),
                    placeholder: input.placeholder(query),
                    helper_text: input.helper_text(query),
                }
            );
        }
    }
    // The variant reaches the rule: pressed fills blue only on a search field.
    let pressed = WidgetState::PRESSED;
    assert_eq!(input.field(pressed).fill, Paint::Solid(RED));
    assert_eq!(
        input
            .field(InputQuery::from(InputVariant::Search).state(pressed))
            .fill,
        Paint::Solid(BLUE)
    );
}

#[test]
fn link_getters_evaluate_their_rule_sets() {
    let container = part_rules(RED, GREEN, BLUE, ());
    let label = text_rules(BLUE, RED);
    let link = LinkWidget::new(WidgetOrigin::Native, container.clone(), label.clone());
    assert!(link.is_native());
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(link.container(query), container.eval(&query));
        assert_eq!(link.label(query), label.eval(&query));
        assert_eq!(
            link.parts(query),
            LinkParts {
                container: link.container(query),
                label: link.label(query),
            }
        );
    }
}

#[test]
fn slider_getters_evaluate_their_rule_sets() {
    let track = part_rules(RED, GREEN, BLUE, ());
    let active_track = part_rules(GREEN, BLUE, RED, ());
    let thumb = part_rules(BLUE, RED, GREEN, ());
    let tick = part_rules(RED, BLUE, GREEN, ());
    let slider = SliderWidget::new(
        WidgetOrigin::Native,
        track.clone(),
        active_track.clone(),
        thumb.clone(),
        tick.clone(),
        6.0,
        None,
    );
    assert!(slider.is_native());
    assert_eq!(slider.tick_length, 6.0);
    assert_eq!(slider.disabled_opacity, None);
    for state in states() {
        for orientation in [Orientation::Horizontal, Orientation::Vertical] {
            let query = StyleQuery::new(state).orientation(orientation);
            assert_eq!(slider.track(query), track.eval(&query));
            assert_eq!(slider.active_track(query), active_track.eval(&query));
            assert_eq!(slider.thumb(query), thumb.eval(&query));
            assert_eq!(slider.tick(query), tick.eval(&query));
            assert_eq!(
                slider.parts(query),
                SliderParts {
                    track: slider.track(query),
                    active_track: slider.active_track(query),
                    thumb: slider.thumb(query),
                    tick: slider.tick(query),
                }
            );
        }
    }
}

#[test]
fn progress_bar_and_splitter_getters_evaluate_their_rule_sets() {
    let track = part_rules(RED, GREEN, BLUE, ());
    let fill = part_rules(GREEN, BLUE, RED, ());
    let handle = part_rules(BLUE, RED, GREEN, ());
    let bar = ProgressBarWidget::new(WidgetOrigin::Native, track.clone(), fill.clone(), 1000);
    let splitter = SplitterWidget::new(WidgetOrigin::Native, handle.clone(), 4.0);
    assert!(bar.is_native() && splitter.is_native());
    assert_eq!(bar.indeterminate_period_ms, 1000);
    assert_eq!(splitter.divider_width, 4.0);
    for state in states() {
        let query = StyleQuery::new(state);
        assert_eq!(bar.track(query), track.eval(&query));
        assert_eq!(bar.fill(query), fill.eval(&query));
        assert_eq!(
            bar.parts(query),
            ProgressBarParts {
                track: bar.track(query),
                fill: bar.fill(query),
            }
        );
        assert_eq!(splitter.handle(query), handle.eval(&query));
        assert_eq!(
            splitter.parts(query),
            SplitterParts {
                handle: splitter.handle(query),
            }
        );
    }
}

#[test]
fn the_widgets_added_in_m1_are_clone_debug_eq_send_sync() {
    assert_send_sync_clone_debug_eq::<crate::next::window::WindowChrome>();
    assert_send_sync_clone_debug_eq::<InputWidget>();
    assert_send_sync_clone_debug_eq::<LinkWidget>();
    assert_send_sync_clone_debug_eq::<SliderWidget>();
    assert_send_sync_clone_debug_eq::<ProgressBarWidget>();
    assert_send_sync_clone_debug_eq::<SplitterWidget>();
}
