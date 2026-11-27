use super::*;
use std::collections::HashSet;
use std::mem::discriminant;

use crate::next::geometry::{StrokeAlign, StrokeSide};
use crate::next::motion::Timing;
use crate::next::paint::PaintLayer;
use crate::next::query::{ButtonQuery, InputQuery, InputVariant, ListQuery};
use crate::next::style::{DecorationKind, FocusKind, FontSize};

const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

fn fill(c: Color) -> PartProp {
    PartProp::Fill(Paint::Solid(c))
}

fn base() -> PartStyle {
    PartStyle {
        fill: Paint::Solid(BLUE),
        stated: PartStated::FILL,
        ..PartStyle::default()
    }
}

/// The selector that matches every query without a variant.
fn any() -> RuleSelector {
    RuleSelector::default()
}

fn on(required: WidgetState) -> RuleSelector {
    RuleSelector {
        required_states: required,
        ..any()
    }
}

fn one_rule<M>(selector: RuleSelector<M>) -> RuleSet<PartStyle, PartProp, M> {
    RuleSet::new(base(), &[], vec![(selector, vec![fill(RED)])])
}

fn fill_of(style: &PartStyle) -> Option<Color> {
    style.fill.fallback_color()
}

#[test]
fn focus_visible_adds_focused() {
    let rules = one_rule(on(WidgetState::FOCUSED));
    let q = StyleQuery::<()>::new(WidgetState::FOCUS_VISIBLE);
    assert_eq!(fill_of(&rules.eval(&q)), Some(RED));
    let forbid = one_rule(RuleSelector {
        forbidden_states: WidgetState::FOCUSED,
        ..any()
    });
    assert_eq!(fill_of(&forbid.eval(&q)), Some(BLUE));
    assert_eq!(
        normalize(WidgetState::FOCUS_VISIBLE, &[]),
        WidgetState::FOCUS_VISIBLE | WidgetState::FOCUSED
    );
}

#[test]
fn exclusive_group_keeps_the_earliest_flag() {
    // Not in bit order (PRESSED is bit 1, HOVER bit 0, DISABLED bit 6): an evaluator keeping
    // the lowest bit fails the first query, one keeping the highest bit the second. In each,
    // the rule of the flag that normalisation clears comes last, so an evaluator that does
    // not normalise fails both.
    const GROUPS: &[&[WidgetState]] = &[&[
        WidgetState::PRESSED,
        WidgetState::HOVER,
        WidgetState::DISABLED,
    ]];
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        base(),
        GROUPS,
        vec![
            (on(WidgetState::PRESSED), vec![fill(GREEN)]),
            (on(WidgetState::HOVER), vec![fill(RED)]),
            (on(WidgetState::DISABLED), vec![fill(Color::BLACK)]),
        ],
    );
    let pressed_hover = StyleQuery::<()>::new(WidgetState::PRESSED | WidgetState::HOVER);
    assert_eq!(fill_of(&rules.eval(&pressed_hover)), Some(GREEN));
    let hover_disabled = StyleQuery::<()>::new(WidgetState::HOVER | WidgetState::DISABLED);
    assert_eq!(fill_of(&rules.eval(&hover_disabled)), Some(RED));
    let disabled = StyleQuery::<()>::new(WidgetState::DISABLED);
    assert_eq!(fill_of(&rules.eval(&disabled)), Some(Color::BLACK));
    assert_eq!(
        normalize(
            WidgetState::DISABLED
                | WidgetState::PRESSED
                | WidgetState::HOVER
                | WidgetState::CHECKED,
            GROUPS
        ),
        WidgetState::PRESSED | WidgetState::CHECKED
    );
    assert_eq!(
        normalize(WidgetState::HOVER | WidgetState::DISABLED, GROUPS),
        WidgetState::HOVER
    );
    assert_eq!(
        normalize(WidgetState::CHECKED, GROUPS),
        WidgetState::CHECKED
    );
}

#[test]
fn several_exclusive_groups_apply_in_order() {
    let all4 = WidgetState::PRESSED
        | WidgetState::HOVER
        | WidgetState::CHECKED
        | WidgetState::INDETERMINATE;
    // Two disjoint groups: both are normalised.
    assert_eq!(
        normalize(
            all4,
            &[
                &[WidgetState::PRESSED, WidgetState::HOVER],
                &[WidgetState::CHECKED, WidgetState::INDETERMINATE],
            ]
        ),
        WidgetState::PRESSED | WidgetState::CHECKED
    );
    // Two groups sharing HOVER: the second sees the first's result.
    let shared = WidgetState::HOVER | WidgetState::PRESSED | WidgetState::DISABLED;
    // [DISABLED, HOVER] clears HOVER; then [HOVER, PRESSED] finds only PRESSED.
    assert_eq!(
        normalize(
            shared,
            &[
                &[WidgetState::DISABLED, WidgetState::HOVER],
                &[WidgetState::HOVER, WidgetState::PRESSED],
            ]
        ),
        WidgetState::DISABLED | WidgetState::PRESSED
    );
    // In the other order: [HOVER, PRESSED] clears PRESSED; then [DISABLED, HOVER] clears HOVER.
    assert_eq!(
        normalize(
            shared,
            &[
                &[WidgetState::HOVER, WidgetState::PRESSED],
                &[WidgetState::DISABLED, WidgetState::HOVER],
            ]
        ),
        WidgetState::DISABLED
    );
    // An empty group changes nothing, and the groups after it still apply.
    assert_eq!(normalize(all4, &[&[]]), all4);
    assert_eq!(
        normalize(all4, &[&[], &[WidgetState::HOVER, WidgetState::PRESSED]]),
        WidgetState::HOVER | WidgetState::CHECKED | WidgetState::INDETERMINATE
    );
    // A group naming a flag twice: the earliest set flag stays.
    const TWICE: &[&[WidgetState]] =
        &[&[WidgetState::HOVER, WidgetState::PRESSED, WidgetState::HOVER]];
    assert_eq!(
        normalize(WidgetState::PRESSED | WidgetState::HOVER, TWICE),
        WidgetState::HOVER
    );
    assert_eq!(normalize(WidgetState::PRESSED, TWICE), WidgetState::PRESSED);
}

#[test]
fn focus_visible_is_added_before_the_groups() {
    // FOCUSED is added first, so the group keeps it and clears HOVER.
    const GROUPS: &[&[WidgetState]] = &[&[WidgetState::FOCUSED, WidgetState::HOVER]];
    let query = WidgetState::FOCUS_VISIBLE | WidgetState::HOVER;
    assert_eq!(
        normalize(query, GROUPS),
        WidgetState::FOCUS_VISIBLE | WidgetState::FOCUSED
    );
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        base(),
        GROUPS,
        vec![
            (on(WidgetState::FOCUSED), vec![fill(GREEN)]),
            (on(WidgetState::HOVER), vec![fill(RED)]),
        ],
    );
    assert_eq!(
        fill_of(&rules.eval(&StyleQuery::<()>::new(query))),
        Some(GREEN)
    );
}

#[test]
fn clearing_focused_clears_focus_visible() {
    const GROUPS: &[&[WidgetState]] = &[&[WidgetState::DISABLED, WidgetState::FOCUSED]];
    let query = WidgetState::DISABLED | WidgetState::FOCUS_VISIBLE;
    assert_eq!(normalize(query, GROUPS), WidgetState::DISABLED);
    let rules = one_rule(on(WidgetState::FOCUS_VISIBLE));
    let rules = RuleSet {
        exclusive: GROUPS,
        ..rules
    };
    assert_eq!(
        fill_of(&rules.eval(&StyleQuery::<()>::new(query))),
        Some(BLUE)
    );
    // Without the higher-precedence state the keyboard-focus rule matches.
    assert_eq!(
        fill_of(&rules.eval(&StyleQuery::<()>::new(WidgetState::FOCUS_VISIBLE))),
        Some(RED)
    );
}

#[test]
fn base_returned_when_no_rule_matches() {
    let rules = one_rule(RuleSelector {
        size: Some(ControlSize::Large),
        ..any()
    });
    assert_eq!(
        rules.eval(&StyleQuery::<()>::new(WidgetState::HOVER)),
        base()
    );
    let buttons: RuleSet<PartStyle, PartProp, ButtonMatcher> = one_rule(RuleSelector {
        matcher: ButtonMatcher {
            variant: Some(ButtonVariant::Primary),
            shape: None,
        },
        ..RuleSelector::default()
    });
    // An axis value no rule names.
    assert_eq!(buttons.eval(&ButtonQuery::glass()), base());
    assert_eq!(fill_of(&buttons.eval(&ButtonQuery::primary())), Some(RED));
    let empty: RuleSet<PartStyle, PartProp> = RuleSet::new(base(), &[], Vec::new());
    assert_eq!(empty.eval(&StyleQuery::<()>::default()), base());
}

#[test]
fn last_match_wins() {
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        base(),
        &[],
        vec![
            (
                on(WidgetState::HOVER),
                vec![fill(RED), PartProp::Opacity(0.5)],
            ),
            (on(WidgetState::HOVER), vec![fill(GREEN)]),
            (on(WidgetState::PRESSED), vec![fill(BLUE)]),
        ],
    );
    let out = rules.eval(&StyleQuery::<()>::new(WidgetState::HOVER));
    assert_eq!(fill_of(&out), Some(GREEN));
    assert_eq!(out.opacity, 0.5);
    let both = rules.eval(&StyleQuery::<()>::new(
        WidgetState::HOVER | WidgetState::PRESSED,
    ));
    assert_eq!(fill_of(&both), Some(BLUE));
}

/// Whether the rule of `selector` applies to `query`.
fn applies<M: VariantMatcher<V>, V>(selector: RuleSelector<M>, query: &StyleQuery<V>) -> bool {
    fill_of(&one_rule(selector).eval(query)) == Some(RED)
}

#[test]
fn selector_states() {
    let q = StyleQuery::<()>::new(WidgetState::HOVER | WidgetState::CHECKED);
    assert!(applies(on(WidgetState::HOVER | WidgetState::CHECKED), &q));
    assert!(!applies(on(WidgetState::HOVER | WidgetState::PRESSED), &q));
    let forbid = |s| RuleSelector {
        forbidden_states: s,
        ..any()
    };
    assert!(applies(forbid(WidgetState::DISABLED), &q));
    assert!(!applies(
        forbid(WidgetState::DISABLED | WidgetState::CHECKED),
        &q
    ));
}

#[test]
fn selector_axes() {
    let q = StyleQuery::<()>::new(WidgetState::NONE)
        .size(ControlSize::Small)
        .surface(Surface::Sidebar)
        .position(GroupPosition::Middle)
        .orientation(Orientation::Vertical);
    let with = |f: fn(&mut RuleSelector)| {
        let mut s = RuleSelector::default();
        f(&mut s);
        s
    };
    assert!(applies(with(|s| s.size = Some(ControlSize::Small)), &q));
    assert!(!applies(with(|s| s.size = Some(ControlSize::Large)), &q));
    assert!(applies(with(|s| s.surface = Some(Surface::Sidebar)), &q));
    assert!(!applies(with(|s| s.surface = Some(Surface::Card)), &q));
    assert!(applies(
        with(|s| s.position = Some(GroupPosition::Middle)),
        &q
    ));
    assert!(!applies(
        with(|s| s.position = Some(GroupPosition::First)),
        &q
    ));
    assert!(applies(
        with(|s| s.orientation = Some(Orientation::Vertical)),
        &q
    ));
    assert!(!applies(
        with(|s| s.orientation = Some(Orientation::Horizontal)),
        &q
    ));
    assert!(applies(any(), &q));
}

#[test]
fn selector_matchers() {
    let input = InputQuery::from(InputVariant::Search);
    let opt = |m: Option<InputVariant>| RuleSelector {
        matcher: m,
        ..RuleSelector::default()
    };
    assert!(applies(opt(None), &input));
    assert!(applies(opt(Some(InputVariant::Search)), &input));
    assert!(!applies(opt(Some(InputVariant::Filled)), &input));

    let button = ButtonQuery::primary().pill();
    let bm = |variant, shape| RuleSelector {
        matcher: ButtonMatcher { variant, shape },
        ..RuleSelector::default()
    };
    assert!(applies(bm(None, None), &button));
    assert!(applies(
        bm(Some(ButtonVariant::Primary), Some(ButtonShape::Pill)),
        &button
    ));
    assert!(!applies(bm(Some(ButtonVariant::Flat), None), &button));
    assert!(!applies(bm(None, Some(ButtonShape::Circle)), &button));

    let row = ListQuery::from(ListVariant::Table).alternate(true);
    let lm = |variant, alternate| RuleSelector {
        matcher: ListMatcher { variant, alternate },
        ..RuleSelector::default()
    };
    assert!(applies(lm(None, None), &row));
    assert!(applies(lm(Some(ListVariant::Table), Some(true)), &row));
    assert!(!applies(lm(Some(ListVariant::Plain), None), &row));
    assert!(!applies(lm(None, Some(false)), &row));

    assert!(applies(
        RuleSelector::<()>::default(),
        &StyleQuery::<()>::default()
    ));
}

#[test]
fn stated_bits_follow_the_applied_properties() {
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        base(),
        &[],
        vec![(
            on(WidgetState::HOVER),
            vec![PartProp::Stroke(Stroke::uniform(1.0, RED))],
        )],
    );
    let idle = rules.eval(&StyleQuery::<()>::default());
    assert_eq!(idle.stated, PartStated::FILL);
    let hover = rules.eval(&StyleQuery::<()>::new(WidgetState::HOVER));
    assert_eq!(hover.stated, PartStated::FILL | PartStated::STROKE);

    let focus = Arc::new(FocusIndicator::new(
        FocusKind::Ring,
        Stroke::uniform(2.0, BLUE),
        Stroke::None,
        1.0,
        None,
        None,
    ));
    let every_part_prop = vec![
        fill(RED),
        PartProp::Stroke(Stroke::uniform_aligned(1.0, RED, StrokeAlign::Center)),
        PartProp::Corners(Corners::uniform(4.0)),
        PartProp::Shadows(None),
        PartProp::Opacity(0.5),
        PartProp::Transform(Transform::IDENTITY),
        PartProp::Padding(Edges::all(1.0)),
        PartProp::Margin(Edges::all(2.0)),
        PartProp::MinSize([Some(1.0), None]),
        PartProp::MaxSize([None, Some(2.0)]),
        PartProp::Focus(Some(focus)),
        PartProp::Ripple(None),
        PartProp::Transition(None),
        PartProp::Cursor(Cursor::Pointer),
    ];
    let all: RuleSet<PartStyle, PartProp> = RuleSet::new(
        PartStyle::default(),
        &[],
        vec![(RuleSelector::default(), every_part_prop)],
    );
    assert_eq!(
        all.eval(&StyleQuery::<()>::default()).stated,
        PartStated::ALL.difference(PartStated::MIN_SIZE | PartStated::MAX_SIZE)
    );
    let unstated: RuleSet<PartStyle, PartProp> = RuleSet::new(
        PartStyle::default(),
        &[],
        vec![(
            RuleSelector::default(),
            vec![PartProp::MinSize([Some(1.0), Some(1.0)])],
        )],
    );
    assert!(
        unstated
            .eval(&StyleQuery::<()>::default())
            .stated
            .is_empty()
    );

    let text: RuleSet<TextStyle, TextProp> = RuleSet::new(
        TextStyle::default(),
        &[],
        vec![
            (
                on(WidgetState::HOVER),
                vec![
                    TextProp::Family(Arc::from("Noto Sans")),
                    TextProp::Color(RED),
                    TextProp::SizePx(13.0),
                    TextProp::Weight(700),
                    TextProp::Style(FontStyle::Italic),
                    TextProp::Width(FontWidth::new(3)),
                    TextProp::LineHeight(16.0),
                    TextProp::LetterSpacing(0.5),
                    TextProp::OpticalSize(None),
                    TextProp::Decoration(TextDecoration {
                        kind: DecorationKind::Underline,
                        color: None,
                        thickness: None,
                    }),
                ],
            ),
            (
                on(WidgetState::PRESSED),
                vec![TextProp::BlendMode(BlendMode::Multiply)],
            ),
        ],
    );
    let t = text.eval(&StyleQuery::<()>::new(WidgetState::HOVER));
    assert_eq!(t.stated, TextStated::ALL.difference(TextStated::BLEND_MODE));
    assert_eq!(&*t.family, "Noto Sans");
    assert_eq!(t.weight, 700);
    assert!(text.eval(&StyleQuery::<()>::default()).stated.is_empty());

    let icon: RuleSet<IconStyle, IconProp> = RuleSet::new(
        IconStyle::default(),
        &[],
        vec![(
            RuleSelector::default(),
            vec![
                IconProp::Size(16.0),
                IconProp::Color(GREEN),
                IconProp::Opacity(0.5),
                IconProp::Weight(500),
                IconProp::RenderingMode(IconRenderingMode::Palette),
                IconProp::StrokeWidth(Some(1.5)),
            ],
        )],
    );
    let i = icon.eval(&StyleQuery::<()>::default());
    assert_eq!(i.stated, IconStated::ALL.difference(IconStated::BLEND_MODE));
    assert_eq!(i.size, 16.0);
    assert_eq!(i.stroke_width, Some(1.5));
}

#[test]
fn min_and_max_size_merge_per_dimension() {
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        PartStyle {
            min_size: [Some(10.0), Some(20.0)],
            max_size: [Some(100.0), None],
            ..PartStyle::default()
        },
        &[],
        vec![(
            on(WidgetState::HOVER),
            vec![
                PartProp::MinSize([None, Some(24.0)]),
                PartProp::MaxSize([None, Some(48.0)]),
            ],
        )],
    );
    let out = rules.eval(&StyleQuery::<()>::new(WidgetState::HOVER));
    assert_eq!(out.min_size, [Some(10.0), Some(24.0)]);
    assert_eq!(out.max_size, [Some(100.0), Some(48.0)]);
}

#[test]
fn padding_and_margin_sides_touch_only_the_flagged_sides() {
    let base = PartStyle {
        padding: Edges::all(1.0),
        margin: Edges::all(1.0),
        stated: PartStated::PADDING_LEFT | PartStated::MARGIN_TOP,
        ..PartStyle::default()
    };
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        base,
        &[],
        vec![(
            RuleSelector::default(),
            vec![
                PartProp::PaddingSides(Edges::all(5.0), EdgeFlags::TOP | EdgeFlags::BOTTOM),
                PartProp::MarginSides(Edges::new(7.0, 8.0, 9.0, 6.0), EdgeFlags::RIGHT),
            ],
        )],
    );
    let out = rules.eval(&StyleQuery::<()>::default());
    assert_eq!(out.padding, Edges::new(5.0, 1.0, 5.0, 1.0));
    assert_eq!(out.margin, Edges::new(1.0, 8.0, 1.0, 1.0));
    assert_eq!(
        out.stated,
        PartStated::PADDING_LEFT
            | PartStated::MARGIN_TOP
            | PartStated::PADDING_TOP
            | PartStated::PADDING_BOTTOM
            | PartStated::MARGIN_RIGHT
    );
    let whole: RuleSet<PartStyle, PartProp> = RuleSet::new(
        PartStyle::default(),
        &[],
        vec![(
            RuleSelector::default(),
            vec![PartProp::Padding(Edges::all(3.0))],
        )],
    );
    let out = whole.eval(&StyleQuery::<()>::default());
    assert_eq!(out.padding, Edges::all(3.0));
    assert_eq!(
        out.stated,
        PartStated::PADDING_TOP
            | PartStated::PADDING_RIGHT
            | PartStated::PADDING_BOTTOM
            | PartStated::PADDING_LEFT
    );
}

#[test]
fn rule_with_out_of_range_props_is_skipped() {
    let entry = |start, len| RuleEntry {
        selector: RuleSelector::default(),
        start,
        len,
    };
    // Props [GREEN, RED]: (0, 1) applies GREEN; (1, 2) reaches past the end. Skipped whole,
    // GREEN stays; clamped to the props in bounds, it would apply RED. The same holds for the
    // ranges at u32::MAX.
    let rules: RuleSet<PartStyle, PartProp> = RuleSet {
        base: base(),
        exclusive: &[],
        rules: Arc::from(vec![
            entry(0, 1),
            entry(1, 2),
            entry(u32::MAX, u32::MAX),
            entry(0, u32::MAX),
        ]),
        props: Arc::from(vec![fill(GREEN), fill(RED)]),
    };
    let out = rules.eval(&StyleQuery::<()>::default());
    assert_eq!(fill_of(&out), Some(GREEN));
    assert_eq!(out.stated, PartStated::FILL);
}

#[test]
fn a_range_beyond_u32_is_stored_empty() -> Result<(), std::num::TryFromIntError> {
    assert_eq!(stored_range(3, 2), (3, 2));
    let max = usize::try_from(u32::MAX)?;
    assert_eq!(stored_range(max, max), (u32::MAX, u32::MAX));
    // One past u32::MAX exists only where usize is wider than 32 bits.
    if let Some(beyond) = max.checked_add(1) {
        assert_eq!(stored_range(beyond, 1), (0, 0));
        assert_eq!(stored_range(1, beyond), (0, 0));
        assert_eq!(stored_range(beyond, beyond), (0, 0));
    }
    // An empty range applies nothing.
    let rules: RuleSet<PartStyle, PartProp> = RuleSet {
        base: base(),
        exclusive: &[],
        rules: Arc::from(vec![RuleEntry {
            selector: RuleSelector::default(),
            start: 0,
            len: 0,
        }]),
        props: Arc::from(vec![fill(RED)]),
    };
    assert_eq!(rules.eval(&StyleQuery::<()>::default()), base());
    Ok(())
}

/// `style` with `prop` applied by a rule that matches every query.
fn applied<T: Clone + Patched<P>, P>(style: T, prop: P) -> T
where
    RuleSet<T, P>: Eval<T>,
{
    RuleSet::new(style, &[], vec![(any(), vec![prop])]).eval_default()
}

/// `eval` on the default query, for the three rule sets alike.
trait Eval<T> {
    fn eval_default(&self) -> T;
}

impl Eval<PartStyle> for RuleSet<PartStyle, PartProp> {
    fn eval_default(&self) -> PartStyle {
        self.eval(&StyleQuery::<()>::default())
    }
}

impl Eval<TextStyle> for RuleSet<TextStyle, TextProp> {
    fn eval_default(&self) -> TextStyle {
        self.eval(&StyleQuery::<()>::default())
    }
}

impl Eval<IconStyle> for RuleSet<IconStyle, IconProp> {
    fn eval_default(&self) -> IconStyle {
        self.eval(&StyleQuery::<()>::default())
    }
}

fn shadow() -> Arc<[ShadowLayer]> {
    Arc::from(vec![ShadowLayer {
        offset_x: 0.0,
        offset_y: 1.0,
        blur: 2.0,
        spread: 0.0,
        color: Color::BLACK,
        inset: false,
    }])
}

fn focus() -> Arc<FocusIndicator> {
    Arc::new(FocusIndicator::new(
        FocusKind::Ring,
        Stroke::uniform(2.0, BLUE),
        Stroke::None,
        1.0,
        None,
        None,
    ))
}

fn ripple() -> Arc<RippleSpec> {
    Arc::new(RippleSpec::new(RED, true, 100, 200))
}

fn transition() -> Transition {
    Transition {
        duration_ms: 150,
        delay_ms: 0,
        timing: Timing::Linear,
    }
}

#[test]
fn each_part_prop_alone_states_exactly_its_property() {
    let d = PartStyle::default;
    let sides = Edges::new(1.0, 2.0, 3.0, 4.0);
    let transform = Transform {
        rotation_deg: 90.0,
        ..Transform::IDENTITY
    };
    let padding_all = PartStated::PADDING_TOP
        | PartStated::PADDING_RIGHT
        | PartStated::PADDING_BOTTOM
        | PartStated::PADDING_LEFT;
    let margin_all = PartStated::MARGIN_TOP
        | PartStated::MARGIN_RIGHT
        | PartStated::MARGIN_BOTTOM
        | PartStated::MARGIN_LEFT;
    let cases = vec![
        (
            fill(RED),
            PartStyle {
                fill: Paint::Solid(RED),
                stated: PartStated::FILL,
                ..d()
            },
        ),
        (
            PartProp::Stroke(Stroke::uniform(1.0, RED)),
            PartStyle {
                stroke: Stroke::uniform(1.0, RED),
                stated: PartStated::STROKE,
                ..d()
            },
        ),
        (
            PartProp::Corners(Corners::uniform(4.0)),
            PartStyle {
                corners: Corners::uniform(4.0),
                stated: PartStated::CORNERS,
                ..d()
            },
        ),
        (
            PartProp::Shadows(Some(shadow())),
            PartStyle {
                shadows: Some(shadow()),
                stated: PartStated::SHADOWS,
                ..d()
            },
        ),
        (
            PartProp::Opacity(0.5),
            PartStyle {
                opacity: 0.5,
                stated: PartStated::OPACITY,
                ..d()
            },
        ),
        (
            PartProp::Transform(transform),
            PartStyle {
                transform,
                stated: PartStated::TRANSFORM,
                ..d()
            },
        ),
        (
            PartProp::Padding(sides),
            PartStyle {
                padding: sides,
                stated: padding_all,
                ..d()
            },
        ),
        (
            PartProp::PaddingSides(sides, EdgeFlags::TOP | EdgeFlags::LEFT),
            PartStyle {
                padding: Edges::new(1.0, 0.0, 0.0, 4.0),
                stated: PartStated::PADDING_TOP | PartStated::PADDING_LEFT,
                ..d()
            },
        ),
        (
            PartProp::Margin(sides),
            PartStyle {
                margin: sides,
                stated: margin_all,
                ..d()
            },
        ),
        (
            PartProp::MarginSides(sides, EdgeFlags::RIGHT | EdgeFlags::BOTTOM),
            PartStyle {
                margin: Edges::new(0.0, 2.0, 3.0, 0.0),
                stated: PartStated::MARGIN_RIGHT | PartStated::MARGIN_BOTTOM,
                ..d()
            },
        ),
        (
            PartProp::MinSize([Some(10.0), None]),
            PartStyle {
                min_size: [Some(10.0), None],
                ..d()
            },
        ),
        (
            PartProp::MaxSize([None, Some(20.0)]),
            PartStyle {
                max_size: [None, Some(20.0)],
                ..d()
            },
        ),
        (
            PartProp::Focus(Some(focus())),
            PartStyle {
                focus: Some(focus()),
                stated: PartStated::FOCUS,
                ..d()
            },
        ),
        (
            PartProp::Ripple(Some(ripple())),
            PartStyle {
                ripple: Some(ripple()),
                stated: PartStated::RIPPLE,
                ..d()
            },
        ),
        (
            PartProp::Transition(Some(transition())),
            PartStyle {
                transition: Some(transition()),
                stated: PartStated::TRANSITION,
                ..d()
            },
        ),
        (
            PartProp::Cursor(Cursor::Pointer),
            PartStyle {
                cursor: Cursor::Pointer,
                stated: PartStated::CURSOR,
                ..d()
            },
        ),
    ];
    // Sixteen variants, each once.
    let variants: HashSet<_> = cases.iter().map(|(p, _)| discriminant(p)).collect();
    assert_eq!(variants.len(), 16);
    assert_eq!(cases.len(), 16);
    for (prop, expected) in cases {
        assert_eq!(applied(d(), prop.clone()), expected, "{prop:?}");
    }
}

#[test]
fn each_text_prop_alone_states_exactly_its_property() {
    let d = TextStyle::default;
    let underline = TextDecoration {
        kind: DecorationKind::Underline,
        color: None,
        thickness: None,
    };
    let cases = vec![
        (
            TextProp::Family(Arc::from("Noto Sans")),
            TextStyle {
                family: Arc::from("Noto Sans"),
                stated: TextStated::FAMILY,
                ..d()
            },
        ),
        (
            TextProp::Color(RED),
            TextStyle {
                color: RED,
                stated: TextStated::COLOR,
                ..d()
            },
        ),
        (
            TextProp::SizePx(13.0),
            TextStyle {
                size_px: 13.0,
                stated: TextStated::SIZE_PX,
                ..d()
            },
        ),
        (
            TextProp::Weight(700),
            TextStyle {
                weight: 700,
                stated: TextStated::WEIGHT,
                ..d()
            },
        ),
        (
            TextProp::Style(FontStyle::Italic),
            TextStyle {
                style: FontStyle::Italic,
                stated: TextStated::STYLE,
                ..d()
            },
        ),
        (
            TextProp::Width(FontWidth::new(3)),
            TextStyle {
                width: FontWidth::new(3),
                stated: TextStated::WIDTH,
                ..d()
            },
        ),
        (
            TextProp::LineHeight(16.0),
            TextStyle {
                line_height: 16.0,
                stated: TextStated::LINE_HEIGHT,
                ..d()
            },
        ),
        (
            TextProp::LetterSpacing(0.5),
            TextStyle {
                letter_spacing: 0.5,
                stated: TextStated::LETTER_SPACING,
                ..d()
            },
        ),
        (
            TextProp::OpticalSize(Some(12.0)),
            TextStyle {
                optical_size: Some(12.0),
                stated: TextStated::OPTICAL_SIZE,
                ..d()
            },
        ),
        (
            TextProp::Decoration(underline),
            TextStyle {
                decoration: underline,
                stated: TextStated::DECORATION,
                ..d()
            },
        ),
        (
            TextProp::BlendMode(BlendMode::Multiply),
            TextStyle {
                blend_mode: BlendMode::Multiply,
                stated: TextStated::BLEND_MODE,
                ..d()
            },
        ),
    ];
    // Eleven variants, each once.
    let variants: HashSet<_> = cases.iter().map(|(p, _)| discriminant(p)).collect();
    assert_eq!(variants.len(), 11);
    assert_eq!(cases.len(), 11);
    for (prop, expected) in cases {
        assert_eq!(applied(d(), prop.clone()), expected, "{prop:?}");
    }
}

#[test]
fn each_icon_prop_alone_states_exactly_its_property() {
    let d = IconStyle::default;
    let cases = vec![
        (
            IconProp::Size(16.0),
            IconStyle {
                size: 16.0,
                stated: IconStated::SIZE,
                ..d()
            },
        ),
        (
            IconProp::Color(GREEN),
            IconStyle {
                color: GREEN,
                stated: IconStated::COLOR,
                ..d()
            },
        ),
        (
            IconProp::Opacity(0.5),
            IconStyle {
                opacity: 0.5,
                stated: IconStated::OPACITY,
                ..d()
            },
        ),
        (
            IconProp::Weight(500),
            IconStyle {
                weight: 500,
                stated: IconStated::WEIGHT,
                ..d()
            },
        ),
        (
            IconProp::RenderingMode(IconRenderingMode::Palette),
            IconStyle {
                rendering_mode: IconRenderingMode::Palette,
                stated: IconStated::RENDERING_MODE,
                ..d()
            },
        ),
        (
            IconProp::StrokeWidth(Some(1.5)),
            IconStyle {
                stroke_width: Some(1.5),
                stated: IconStated::STROKE_WIDTH,
                ..d()
            },
        ),
        (
            IconProp::BlendMode(BlendMode::Screen),
            IconStyle {
                blend_mode: BlendMode::Screen,
                stated: IconStated::BLEND_MODE,
                ..d()
            },
        ),
    ];
    // Seven variants, each once.
    let variants: HashSet<_> = cases.iter().map(|(p, _)| discriminant(p)).collect();
    assert_eq!(variants.len(), 7);
    assert_eq!(cases.len(), 7);
    for (prop, expected) in cases {
        assert_eq!(applied(d(), prop.clone()), expected, "{prop:?}");
    }
}

#[test]
fn a_rules_none_replaces_a_base_some() {
    let part = PartStyle {
        shadows: Some(shadow()),
        focus: Some(focus()),
        ripple: Some(ripple()),
        transition: Some(transition()),
        ..PartStyle::default()
    };
    let part_cases = [
        (
            PartProp::Shadows(None),
            PartStyle {
                shadows: None,
                stated: PartStated::SHADOWS,
                ..part.clone()
            },
        ),
        (
            PartProp::Focus(None),
            PartStyle {
                focus: None,
                stated: PartStated::FOCUS,
                ..part.clone()
            },
        ),
        (
            PartProp::Ripple(None),
            PartStyle {
                ripple: None,
                stated: PartStated::RIPPLE,
                ..part.clone()
            },
        ),
        (
            PartProp::Transition(None),
            PartStyle {
                transition: None,
                stated: PartStated::TRANSITION,
                ..part.clone()
            },
        ),
    ];
    for (prop, expected) in part_cases {
        assert_eq!(applied(part.clone(), prop.clone()), expected, "{prop:?}");
    }
    let text = TextStyle {
        optical_size: Some(12.0),
        ..TextStyle::default()
    };
    assert_eq!(
        applied(text.clone(), TextProp::OpticalSize(None)),
        TextStyle {
            optical_size: None,
            stated: TextStated::OPTICAL_SIZE,
            ..text
        }
    );
    let icon = IconStyle {
        stroke_width: Some(1.5),
        ..IconStyle::default()
    };
    assert_eq!(
        applied(icon.clone(), IconProp::StrokeWidth(None)),
        IconStyle {
            stroke_width: None,
            stated: IconStated::STROKE_WIDTH,
            ..icon
        }
    );
}

#[test]
fn size_px_clears_defined_size() {
    let text = TextStyle {
        size_px: 13.0,
        defined_size: Some(FontSize::Pt(10.0)),
        stated: TextStated::SIZE_PX,
        ..TextStyle::default()
    };
    assert_eq!(
        applied(text.clone(), TextProp::SizePx(16.0)),
        TextStyle {
            size_px: 16.0,
            defined_size: None,
            ..text.clone()
        }
    );
    // Every other property leaves it.
    let bold = applied(text.clone(), TextProp::Weight(700));
    assert_eq!(bold.defined_size, Some(FontSize::Pt(10.0)));
    let tall = applied(text, TextProp::LineHeight(18.0));
    assert_eq!(tall.defined_size, Some(FontSize::Pt(10.0)));
}

// ---------------------------------------------------------------------------
// The builder.
// ---------------------------------------------------------------------------

const PRIMARY: ButtonMatcher = ButtonMatcher {
    variant: Some(ButtonVariant::Primary),
    shape: None,
};

/// A button body as a preset writes it: a base of five stated properties,
/// one exclusive group, rules on states, on a variant, a size and an
/// orientation, and one property list shared by two selectors.
fn built_body() -> RuleSet<PartStyle, PartProp, ButtonMatcher> {
    const GROUPS: &[&[WidgetState]] = &[&[
        WidgetState::DISABLED,
        WidgetState::PRESSED,
        WidgetState::HOVER,
    ]];
    RuleBuilder::with_exclusive(PartStyle::default(), GROUPS)
        .base([
            fill(BLUE),
            PartProp::Stroke(Stroke::uniform(1.0, RED)),
            PartProp::Corners(Corners::uniform(4.0)),
            PartProp::Padding(Edges::all(6.0)),
            PartProp::MinSize([Some(80.0), None]),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [fill(GREEN), PartProp::Stroke(Stroke::uniform(1.0, GREEN))],
        )
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::FOCUS_VISIBLE),
            [PartProp::Stroke(Stroke::uniform(2.0, BLUE))],
        )
        .rule(
            RuleSelector::on(WidgetState::NONE).matcher(PRIMARY),
            [fill(RED)],
        )
        .rule(
            RuleSelector::on(WidgetState::NONE).size(ControlSize::Small),
            [PartProp::MinSize([None, Some(20.0)])],
        )
        .rule(
            RuleSelector::on(WidgetState::NONE).orientation(Orientation::Vertical),
            [PartProp::MinSize([Some(30.0), None])],
        )
        .rules(
            [
                RuleSelector::on(WidgetState::PRESSED),
                RuleSelector::on(WidgetState::CHECKED).except(WidgetState::DISABLED),
            ],
            [fill(Color::BLACK), PartProp::Opacity(0.5)],
        )
        .build()
}

#[test]
fn the_builder_states_the_base_and_orders_the_rules() {
    let body = built_body();
    let rest = body.eval(&ButtonQuery::new(WidgetState::NONE));
    assert_eq!(
        rest,
        PartStyle {
            fill: Paint::Solid(BLUE),
            stroke: Stroke::uniform(1.0, RED),
            corners: Corners::uniform(4.0),
            padding: Edges::all(6.0),
            min_size: [Some(80.0), None],
            stated: PartStated::FILL
                | PartStated::STROKE
                | PartStated::CORNERS
                | PartStated::PADDING_TOP
                | PartStated::PADDING_RIGHT
                | PartStated::PADDING_BOTTOM
                | PartStated::PADDING_LEFT,
            ..PartStyle::default()
        }
    );
    let hover = body.eval(&ButtonQuery::new(WidgetState::HOVER));
    assert_eq!(hover.fill, Paint::Solid(GREEN));
    assert_eq!(hover.stroke, Stroke::uniform(1.0, GREEN));
    let keyboard = body.eval(&ButtonQuery::new(
        WidgetState::HOVER | WidgetState::FOCUS_VISIBLE,
    ));
    assert_eq!(keyboard.fill, Paint::Solid(GREEN));
    assert_eq!(keyboard.stroke, Stroke::uniform(2.0, BLUE));
    assert_eq!(body.eval(&ButtonQuery::primary()).fill, Paint::Solid(RED));
    let small = body.eval(&ButtonQuery::new(WidgetState::NONE).size(ControlSize::Small));
    assert_eq!(small.min_size, [Some(80.0), Some(20.0)]);
    let vertical = body.eval(
        &ButtonQuery::new(WidgetState::NONE)
            .size(ControlSize::Small)
            .orientation(Orientation::Vertical),
    );
    assert_eq!(vertical.min_size, [Some(30.0), Some(20.0)]);
    // The exclusive group: pressed wins over hover, so the hover rules do not apply.
    let pressed = body.eval(&ButtonQuery::new(WidgetState::PRESSED | WidgetState::HOVER));
    assert_eq!(pressed.fill, Paint::Solid(Color::BLACK));
    assert_eq!(pressed.stroke, Stroke::uniform(1.0, RED));
    assert_eq!(pressed.opacity, 0.5);
}

#[test]
fn a_shared_property_list_is_stored_once() -> Result<(), &'static str> {
    let body = built_body();
    // Base 5 stated properties are not in the list; the rules hold 2 + 1 + 1 + 1 + 1 + 2.
    assert_eq!(body.props.len(), 8);
    assert_eq!(body.rules.len(), 7);
    // Each rule's range follows the lists before it: hover 2, keyboard 1, primary 1, small 1,
    // vertical 1, then pressed and checked sharing one list of 2.
    let ranges: Vec<(u32, u32)> = body.rules.iter().map(|r| (r.start, r.len)).collect();
    assert_eq!(
        ranges,
        vec![(0, 2), (2, 1), (3, 1), (4, 1), (5, 1), (6, 2), (6, 2)]
    );
    let pressed = body.rules.get(5).ok_or("a sixth rule")?;
    let checked = body.rules.get(6).ok_or("a seventh rule")?;
    assert_eq!((pressed.start, pressed.len), (checked.start, checked.len));
    let checked = body.eval(&ButtonQuery::new(WidgetState::CHECKED));
    assert_eq!(checked.fill, Paint::Solid(Color::BLACK));
    assert_eq!(checked.opacity, 0.5);
    let disabled = body.eval(&ButtonQuery::new(
        WidgetState::CHECKED | WidgetState::DISABLED,
    ));
    assert_eq!(disabled.fill, Paint::Solid(BLUE));
    Ok(())
}

#[test]
fn selector_shorthands_set_their_fields() {
    let built = RuleSelector::on(WidgetState::HOVER)
        .except(WidgetState::DISABLED)
        .size(ControlSize::Large)
        .surface(Surface::Sidebar)
        .position(GroupPosition::First)
        .orientation(Orientation::Vertical)
        .matcher(PRIMARY);
    assert_eq!(
        built,
        RuleSelector {
            required_states: WidgetState::HOVER,
            forbidden_states: WidgetState::DISABLED,
            size: Some(ControlSize::Large),
            surface: Some(Surface::Sidebar),
            position: Some(GroupPosition::First),
            orientation: Some(Orientation::Vertical),
            matcher: PRIMARY,
        }
    );
}

#[test]
fn new_is_the_builder() {
    let selector = on(WidgetState::HOVER);
    let by_new = RuleSet::new(base(), &[], vec![(selector, vec![fill(RED), fill(GREEN)])]);
    let by_builder = RuleBuilder::new(base())
        .rule(selector, [fill(RED), fill(GREEN)])
        .build();
    assert_eq!(by_new, by_builder);
}

#[test]
fn dead_rules_lists_the_rules_that_cannot_match() {
    const GROUPS: &[&[WidgetState]] = &[
        &[
            WidgetState::DISABLED,
            WidgetState::PRESSED,
            WidgetState::HOVER,
        ],
        &[WidgetState::CHECKED, WidgetState::INDETERMINATE],
    ];
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::with_exclusive(base(), GROUPS)
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(RED)])
        // Two flags of the first group: never both after normalisation.
        .rule(
            RuleSelector::on(WidgetState::PRESSED | WidgetState::HOVER),
            [fill(RED)],
        )
        // One flag of each group: can match.
        .rule(
            RuleSelector::on(WidgetState::HOVER | WidgetState::CHECKED),
            [fill(RED)],
        )
        // Required and forbidden intersect.
        .rule(
            RuleSelector::on(WidgetState::FOCUSED | WidgetState::SELECTED)
                .except(WidgetState::SELECTED),
            [fill(RED)],
        )
        // Two flags of the second group.
        .rule(
            RuleSelector::on(WidgetState::CHECKED | WidgetState::INDETERMINATE),
            [fill(RED)],
        )
        // Forbidden flags of one group are no conflict.
        .rule(
            RuleSelector::on(WidgetState::CHECKED)
                .except(WidgetState::PRESSED | WidgetState::HOVER),
            [fill(RED)],
        )
        .build();
    assert_eq!(rules.dead_rules(), vec![1, 3, 4]);
}

#[test]
fn a_clean_rule_set_has_no_dead_rules() {
    assert!(built_body().dead_rules().is_empty());
    // Without exclusive groups, two required flags are no conflict.
    let free: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(
            RuleSelector::on(WidgetState::PRESSED | WidgetState::HOVER),
            [fill(RED)],
        )
        .build();
    assert!(free.dead_rules().is_empty());
}

/// Every state flag, for the tests that evaluate a rule set against all
/// combinations of them.
fn every_state() -> impl Iterator<Item = WidgetState> {
    (0..=u16::from(u8::MAX)).map(|bits| {
        [
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::FOCUSED,
            WidgetState::DISABLED,
            WidgetState::FOCUS_VISIBLE,
            WidgetState::CHECKED,
            WidgetState::SELECTED,
            WidgetState::UNFOCUSED_VIEW,
        ]
        .into_iter()
        .enumerate()
        .filter(|(i, _)| bits & (1 << i) != 0)
        .fold(WidgetState::NONE, |acc, (_, flag)| acc | flag)
    })
}

#[test]
fn requiring_focus_visible_and_forbidding_focused_is_dead() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(
            RuleSelector::on(WidgetState::FOCUS_VISIBLE).except(WidgetState::FOCUSED),
            [fill(RED)],
        )
        .build();
    assert_eq!(rules.dead_rules(), vec![0]);
    // Step 0 adds FOCUSED to every query holding FOCUS_VISIBLE: no query matches.
    assert!(
        every_state()
            .all(|state| fill_of(&rules.eval(&StyleQuery::<()>::new(state))) == Some(BLUE))
    );
}

#[test]
fn requiring_focus_visible_with_a_group_rival_of_focused_is_dead() {
    const GROUPS: &[&[WidgetState]] = &[&[WidgetState::DISABLED, WidgetState::FOCUSED]];
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::with_exclusive(base(), GROUPS)
        .rule(
            RuleSelector::on(WidgetState::DISABLED | WidgetState::FOCUS_VISIBLE),
            [fill(RED)],
        )
        // FOCUS_VISIBLE alone is no conflict.
        .rule(RuleSelector::on(WidgetState::FOCUS_VISIBLE), [fill(GREEN)])
        .build();
    assert_eq!(rules.dead_rules(), vec![0]);
    // Step 1 keeps DISABLED, clears FOCUSED and with it FOCUS_VISIBLE: the first rule never
    // applies.
    assert!(
        every_state().all(|state| fill_of(&rules.eval(&StyleQuery::<()>::new(state))) != Some(RED))
    );
}

#[test]
fn the_groups_given_at_creation_are_the_rule_sets_groups() {
    const BOTH: &[&[WidgetState]] = &[
        &[WidgetState::DISABLED, WidgetState::HOVER],
        &[WidgetState::CHECKED, WidgetState::SELECTED],
    ];
    let rules = |builder: RuleBuilder<PartStyle, PartProp>| {
        builder
            .rule(RuleSelector::on(WidgetState::HOVER), [fill(RED)])
            .rule(RuleSelector::on(WidgetState::SELECTED), [fill(GREEN)])
            .build()
    };
    let grouped = rules(RuleBuilder::with_exclusive(base(), BOTH));
    assert!(std::ptr::eq(grouped.exclusive, BOTH));
    assert!(rules(RuleBuilder::new(base())).exclusive.is_empty());
    // Both groups normalise: DISABLED drops HOVER, CHECKED drops SELECTED.
    let query =
        WidgetState::DISABLED | WidgetState::HOVER | WidgetState::CHECKED | WidgetState::SELECTED;
    assert_eq!(
        fill_of(&grouped.eval(&StyleQuery::<()>::new(query))),
        Some(BLUE)
    );
}

#[test]
fn rules_with_no_selector_store_nothing() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(RED)])
        .rules([], [fill(GREEN), PartProp::Opacity(0.5)])
        .build();
    assert_eq!(rules.props.len(), 1);
    assert_eq!(rules.rules.len(), 1);
    assert_eq!(
        rules,
        RuleBuilder::new(base())
            .rule(RuleSelector::on(WidgetState::HOVER), [fill(RED)])
            .build()
    );
}

#[test]
fn a_rule_without_properties_after_two_others_is_an_empty_range_at_the_end()
-> Result<(), &'static str> {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(RuleSelector::on(WidgetState::HOVER), [fill(RED)])
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [fill(GREEN), PartProp::Opacity(0.5)],
        )
        .rule(RuleSelector::on(WidgetState::CHECKED), [])
        .build();
    assert_eq!(rules.props.len(), 3);
    let empty = rules.rules.get(2).ok_or("a third rule")?;
    assert_eq!((empty.start, empty.len), (3, 0));
    assert_eq!(
        rules.eval(&StyleQuery::<()>::new(WidgetState::CHECKED)),
        base()
    );
    Ok(())
}

#[test]
fn eval_hands_out_the_rules_own_arcs() {
    let side = StrokeSide {
        width: 1.0,
        paint: Paint::layers(vec![PaintLayer::Solid {
            color: RED,
            blend: BlendMode::Normal,
        }]),
    };
    let paint = Paint::layers(vec![PaintLayer::Solid {
        color: GREEN,
        blend: BlendMode::Normal,
    }]);
    let stroke = Stroke::sides(
        side.clone(),
        side.clone(),
        side.clone(),
        side,
        StrokeAlign::Inside,
    );
    let (shadows, focus, ripple) = (shadow(), focus(), ripple());
    let family: Arc<str> = Arc::from("Noto Sans");
    let part: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(
            on(WidgetState::HOVER),
            [
                PartProp::Fill(paint.clone()),
                PartProp::Stroke(stroke.clone()),
                PartProp::Shadows(Some(shadows.clone())),
            ],
        )
        // Matched by a `FOCUS_VISIBLE` query only through step 0 of the evaluation contract.
        .rule(
            on(WidgetState::FOCUSED),
            [
                PartProp::Focus(Some(focus.clone())),
                PartProp::Ripple(Some(ripple.clone())),
            ],
        )
        .build();
    let text: RuleSet<TextStyle, TextProp> = RuleBuilder::new(TextStyle::default())
        .rule(on(WidgetState::HOVER), [TextProp::Family(family.clone())])
        .build();
    let query = StyleQuery::<()>::new(WidgetState::HOVER | WidgetState::FOCUS_VISIBLE);
    let out = part.eval(&query);
    assert!(
        matches!((&out.fill, &paint), (Paint::Complex(a), Paint::Complex(b)) if Arc::ptr_eq(a, b))
    );
    assert!(
        matches!((&out.stroke, &stroke), (Stroke::Complex(a), Stroke::Complex(b)) if Arc::ptr_eq(a, b))
    );
    assert!(
        out.shadows
            .as_ref()
            .is_some_and(|s| Arc::ptr_eq(s, &shadows))
    );
    assert!(out.focus.as_ref().is_some_and(|f| Arc::ptr_eq(f, &focus)));
    assert!(out.ripple.as_ref().is_some_and(|r| Arc::ptr_eq(r, &ripple)));
    assert!(Arc::ptr_eq(&text.eval(&query).family, &family));
}

#[test]
fn build_with_no_rules_is_the_base() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base()).build();
    assert!(rules.rules.is_empty());
    assert!(rules.props.is_empty());
    assert!(rules.exclusive.is_empty());
    assert!(every_state().all(|state| rules.eval(&StyleQuery::<()>::new(state)) == base()));
}

// ---------------------------------------------------------------------------
// `from_state`.
// ---------------------------------------------------------------------------

#[test]
fn the_base_has_no_from_state_bits() {
    let body = built_body();
    assert!(body.base.from_state.is_empty());
    let rest = body.eval(&ButtonQuery::new(WidgetState::NONE));
    assert!(!rest.stated.is_empty());
    assert!(rest.from_state.is_empty());
}

#[test]
fn a_state_rule_sets_the_bits_of_exactly_its_properties() {
    let body = built_body();
    let hover = body.eval(&ButtonQuery::new(WidgetState::HOVER));
    assert_eq!(hover.from_state, PartStated::FILL | PartStated::STROKE);
    let pressed = body.eval(&ButtonQuery::new(WidgetState::PRESSED));
    assert_eq!(pressed.from_state, PartStated::FILL | PartStated::OPACITY);
    let padded: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(
            on(WidgetState::HOVER),
            [
                PartProp::PaddingSides(Edges::all(2.0), EdgeFlags::TOP | EdgeFlags::LEFT),
                PartProp::MinSize([Some(10.0), None]),
            ],
        )
        .build();
    assert_eq!(
        padded
            .eval(&StyleQuery::<()>::new(WidgetState::HOVER))
            .from_state,
        PartStated::PADDING_TOP | PartStated::PADDING_LEFT | PartStated::MIN_SIZE
    );
}

#[test]
fn a_size_written_for_a_state_sets_its_bit_in_from_state_alone() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(PartStyle::default())
        .base([PartProp::MinSize([Some(8.0), Some(8.0)])])
        .rule(
            on(WidgetState::HOVER),
            [PartProp::MinSize([Some(12.0), None])],
        )
        .rule(
            on(WidgetState::PRESSED),
            [PartProp::MaxSize([None, Some(20.0)])],
        )
        .rule(
            any().size(ControlSize::Small),
            [PartProp::MaxSize([Some(30.0), None])],
        )
        .build();
    let rest = rules.eval(&StyleQuery::<()>::default());
    assert!(rest.from_state.is_empty());
    let hover = rules.eval(&StyleQuery::<()>::new(WidgetState::HOVER));
    assert_eq!(hover.min_size, [Some(12.0), Some(8.0)]);
    assert_eq!(hover.from_state, PartStated::MIN_SIZE);
    assert!(
        !hover
            .stated
            .intersects(PartStated::MIN_SIZE | PartStated::MAX_SIZE)
    );
    let pressed = rules.eval(&StyleQuery::<()>::new(WidgetState::PRESSED));
    assert_eq!(pressed.from_state, PartStated::MAX_SIZE);
    // A later stateless rule writing the size clears its bit.
    let small = rules.eval(&StyleQuery::<()>::new(WidgetState::PRESSED).size(ControlSize::Small));
    assert_eq!(small.max_size, [Some(30.0), Some(20.0)]);
    assert!(small.from_state.is_empty());
    assert!(small.stated.is_empty());
}

#[test]
fn a_stateless_side_clears_only_its_own_bit() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(PartStyle::default())
        .rule(on(WidgetState::HOVER), [PartProp::Padding(Edges::all(4.0))])
        .rule(
            any(),
            [PartProp::PaddingSides(Edges::all(1.0), EdgeFlags::TOP)],
        )
        .rule(
            on(WidgetState::HOVER),
            [PartProp::MarginSides(
                Edges::all(2.0),
                EdgeFlags::BOTTOM | EdgeFlags::RIGHT,
            )],
        )
        .build();
    let hover = rules.eval(&StyleQuery::<()>::new(WidgetState::HOVER));
    assert_eq!(
        hover.from_state,
        PartStated::PADDING_RIGHT
            | PartStated::PADDING_BOTTOM
            | PartStated::PADDING_LEFT
            | PartStated::MARGIN_RIGHT
            | PartStated::MARGIN_BOTTOM
    );
    assert!(hover.stated.contains(PartStated::PADDING_TOP));
}

#[test]
fn from_state_follows_the_normalised_state() {
    let body = built_body();
    let pressed = body.eval(&ButtonQuery::new(WidgetState::PRESSED));
    let both = body.eval(&ButtonQuery::new(WidgetState::PRESSED | WidgetState::HOVER));
    assert_eq!(both.from_state, pressed.from_state);
    assert_eq!(both.from_state, PartStated::FILL | PartStated::OPACITY);
    // DISABLED ranks first and clears FOCUSED, and FOCUS_VISIBLE with it: the
    // keyboard-focus rule does not match, and nothing comes from a state.
    const GROUPS: &[&[WidgetState]] = &[&[WidgetState::DISABLED, WidgetState::FOCUSED]];
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        base(),
        GROUPS,
        vec![(on(WidgetState::FOCUS_VISIBLE), vec![fill(RED)])],
    );
    let out = rules.eval(&StyleQuery::<()>::new(
        WidgetState::DISABLED | WidgetState::FOCUS_VISIBLE,
    ));
    assert_eq!(fill_of(&out), Some(BLUE));
    assert!(out.from_state.is_empty());
}

#[test]
fn size_px_from_a_state_sets_its_bit() {
    let rules: RuleSet<TextStyle, TextProp> = RuleBuilder::new(TextStyle::default())
        .base([TextProp::SizePx(10.0)])
        .rule(on(WidgetState::SELECTED), [TextProp::SizePx(12.0)])
        .build();
    let selected = rules.eval(&StyleQuery::<()>::new(WidgetState::SELECTED));
    assert_eq!(selected.size_px, 12.0);
    assert_eq!(selected.from_state, TextStated::SIZE_PX);
    assert!(
        rules
            .eval(&StyleQuery::<()>::default())
            .from_state
            .is_empty()
    );
}

#[test]
fn build_clears_the_base_from_state() {
    let mut part = base();
    part.from_state = PartStated::FILL | PartStated::MIN_SIZE;
    let parts: RuleSet<PartStyle, PartProp> = RuleBuilder::new(part).build();
    assert!(parts.base.from_state.is_empty());
    let text = TextStyle {
        from_state: TextStated::COLOR,
        ..TextStyle::default()
    };
    let texts: RuleSet<TextStyle, TextProp> =
        RuleBuilder::with_exclusive(text, &[&[WidgetState::HOVER]]).build();
    assert!(texts.base.from_state.is_empty());
    let icon = IconStyle {
        from_state: IconStated::SIZE,
        ..IconStyle::default()
    };
    let icons: RuleSet<IconStyle, IconProp> = RuleBuilder::new(icon).build();
    assert!(icons.base.from_state.is_empty());
}

#[test]
fn style_sizes() {
    use std::mem::size_of;
    println!(
        "PartStyle {} B, TextStyle {} B, IconStyle {} B",
        size_of::<PartStyle>(),
        size_of::<TextStyle>(),
        size_of::<IconStyle>()
    );
    assert!(size_of::<PartStyle>() <= 256);
}

#[test]
fn a_later_stateless_rule_clears_the_bit_of_what_it_overwrites() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(on(WidgetState::HOVER), [fill(RED), PartProp::Opacity(0.5)])
        .rule(any(), [fill(GREEN)])
        .build();
    let out = rules.eval(&StyleQuery::<()>::new(WidgetState::HOVER));
    assert_eq!(fill_of(&out), Some(GREEN));
    assert_eq!(out.from_state, PartStated::OPACITY);
    assert_eq!(out.stated, PartStated::FILL | PartStated::OPACITY);
}

#[test]
fn a_focused_rule_matched_through_focus_visible_sets_the_bit() {
    let rules = one_rule(on(WidgetState::FOCUSED));
    let out = rules.eval(&StyleQuery::<()>::new(WidgetState::FOCUS_VISIBLE));
    assert_eq!(fill_of(&out), Some(RED));
    assert_eq!(out.from_state, PartStated::FILL);
}

#[test]
fn a_rule_on_axes_alone_sets_no_bit() {
    let body = built_body();
    let primary = body.eval(&ButtonQuery::primary());
    assert_eq!(primary.fill, Paint::Solid(RED));
    assert!(primary.from_state.is_empty());
    let forbidding = one_rule(RuleSelector {
        forbidden_states: WidgetState::DISABLED,
        size: Some(ControlSize::Small),
        ..any()
    });
    let small =
        forbidding.eval(&StyleQuery::<()>::new(WidgetState::HOVER).size(ControlSize::Small));
    assert_eq!(fill_of(&small), Some(RED));
    assert!(small.from_state.is_empty());
}

#[test]
fn text_from_state_follows_the_rules() {
    let rules: RuleSet<TextStyle, TextProp> = RuleBuilder::new(TextStyle::default())
        .base([TextProp::Color(BLUE), TextProp::Weight(400)])
        .rule(
            on(WidgetState::HOVER),
            [TextProp::Color(RED), TextProp::Weight(700)],
        )
        .rule(any(), [TextProp::Weight(600)])
        .build();
    assert!(
        rules
            .eval(&StyleQuery::<()>::default())
            .from_state
            .is_empty()
    );
    let hover = rules.eval(&StyleQuery::<()>::new(WidgetState::HOVER));
    assert_eq!(hover.color, RED);
    assert_eq!(hover.weight, 600);
    assert_eq!(hover.from_state, TextStated::COLOR);
}

#[test]
fn icon_from_state_follows_the_rules() {
    let rules: RuleSet<IconStyle, IconProp> = RuleBuilder::new(IconStyle::default())
        .base([IconProp::Color(BLUE), IconProp::Size(16.0)])
        .rule(
            on(WidgetState::DISABLED),
            [IconProp::Color(RED), IconProp::Opacity(0.5)],
        )
        .rule(any(), [IconProp::Opacity(0.25)])
        .build();
    assert!(
        rules
            .eval(&StyleQuery::<()>::default())
            .from_state
            .is_empty()
    );
    let disabled = rules.eval(&StyleQuery::<()>::new(WidgetState::DISABLED));
    assert_eq!(disabled.color, RED);
    assert_eq!(disabled.opacity, 0.25);
    assert_eq!(disabled.from_state, IconStated::COLOR);
}

// ---------------------------------------------------------------------------
// `from_state` of a query that asks about flags.
// ---------------------------------------------------------------------------

const DISABLED_CHECKED_HOVER: WidgetState = WidgetState::DISABLED
    .union(WidgetState::CHECKED)
    .union(WidgetState::HOVER);

#[test]
fn asked_a_rule_requiring_the_flag_sets_another_states_rule_leaves_a_stateless_one_clears() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(
            on(WidgetState::DISABLED),
            [fill(RED), PartProp::Opacity(0.5)],
        )
        .rule(
            on(WidgetState::DISABLED | WidgetState::CHECKED),
            [PartProp::Corners(Corners::uniform(2.0))],
        )
        .rule(
            on(WidgetState::CHECKED | WidgetState::HOVER),
            [fill(GREEN), PartProp::Stroke(Stroke::uniform(1.0, GREEN))],
        )
        .rule(any(), [PartProp::Opacity(1.0)])
        .build();
    let plain = StyleQuery::<()>::new(DISABLED_CHECKED_HOVER);
    let unasked = rules.eval(&plain);
    assert_eq!(
        unasked.from_state,
        PartStated::FILL | PartStated::CORNERS | PartStated::STROKE
    );
    let asking = rules.eval(&plain.asking(WidgetState::DISABLED));
    // The `DISABLED` rule's fill bit stays under the `CHECKED | HOVER` rule's value; that rule
    // sets no bit of its own (the stroke); the stateless rule clears the opacity's.
    assert_eq!(fill_of(&asking), Some(GREEN));
    assert_eq!(asking.from_state, PartStated::FILL | PartStated::CORNERS);
    // A flag the matching rules require only together with others: the two-flag rules set it.
    let checked = rules.eval(&plain.asking(WidgetState::CHECKED));
    assert_eq!(
        checked.from_state,
        PartStated::FILL | PartStated::CORNERS | PartStated::STROKE
    );
    // An asked flag the query does not set is required by no matching rule.
    let selected = rules.eval(&plain.asking(WidgetState::SELECTED));
    assert!(selected.from_state.is_empty());
    // Two asked flags: only a rule requiring both sets a bit.
    let both = rules.eval(&plain.asking(WidgetState::DISABLED | WidgetState::CHECKED));
    assert_eq!(both.from_state, PartStated::CORNERS);
}

#[test]
fn asked_text_and_icon_bits_are_left_by_another_states_rule() {
    let text: RuleSet<TextStyle, TextProp> = RuleBuilder::new(TextStyle::default())
        .rule(on(WidgetState::DISABLED), [TextProp::Color(RED)])
        .rule(
            on(WidgetState::CHECKED | WidgetState::HOVER),
            [TextProp::Color(GREEN), TextProp::Weight(700)],
        )
        .build();
    let icon: RuleSet<IconStyle, IconProp> = RuleBuilder::new(IconStyle::default())
        .rule(on(WidgetState::DISABLED), [IconProp::Color(RED)])
        .rule(
            on(WidgetState::CHECKED | WidgetState::HOVER),
            [IconProp::Color(GREEN), IconProp::Opacity(0.5)],
        )
        .build();
    let query = StyleQuery::<()>::new(DISABLED_CHECKED_HOVER).asking(WidgetState::DISABLED);
    let label = text.eval(&query);
    assert_eq!(label.color, GREEN);
    assert_eq!(label.from_state, TextStated::COLOR);
    let glyph = icon.eval(&query);
    assert_eq!(glyph.color, GREEN);
    assert_eq!(glyph.from_state, IconStated::COLOR);
}

#[test]
fn asked_focus_visible_counts_a_focused_rule_and_back() {
    let focus = [WidgetState::FOCUSED, WidgetState::FOCUS_VISIBLE];
    for required in focus {
        let rules = one_rule(on(required));
        for asked in focus {
            let query = StyleQuery::<()>::new(WidgetState::FOCUS_VISIBLE);
            let out = rules.eval(&query.asking(asked));
            assert_eq!(fill_of(&out), Some(RED), "{required:?} {asked:?}");
            assert_eq!(out.from_state, PartStated::FILL, "{required:?} {asked:?}");
            // Asked together with another flag, the rule requires only one of them.
            let pair = rules.eval(
                &query
                    .with_state(WidgetState::HOVER)
                    .asking(asked | WidgetState::HOVER),
            );
            assert!(pair.from_state.is_empty(), "{required:?} {asked:?}");
        }
    }
    // A rule requiring both reads as requiring `FOCUSED` alone.
    let both = one_rule(on(WidgetState::FOCUSED | WidgetState::FOCUS_VISIBLE));
    let out =
        both.eval(&StyleQuery::<()>::new(WidgetState::FOCUS_VISIBLE).asking(WidgetState::FOCUSED));
    assert_eq!(out.from_state, PartStated::FILL);
}

#[test]
fn asked_a_flag_an_exclusive_group_cleared_has_no_bit() {
    const GROUPS: &[&[WidgetState]] = &[&[WidgetState::DISABLED, WidgetState::HOVER]];
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        base(),
        GROUPS,
        vec![
            (on(WidgetState::HOVER), vec![fill(RED)]),
            (on(WidgetState::DISABLED), vec![fill(GREEN)]),
        ],
    );
    let query = StyleQuery::<()>::new(WidgetState::DISABLED | WidgetState::HOVER);
    let out = rules.eval(&query.asking(WidgetState::HOVER));
    assert_eq!(fill_of(&out), Some(GREEN));
    assert!(out.from_state.is_empty());
    assert_eq!(rules.eval(&query).from_state, PartStated::FILL);
}

/// A rule set with one-flag, two-flag, focus, exclusive and stateless rules.
fn asking_rules() -> RuleSet<PartStyle, PartProp> {
    const GROUPS: &[&[WidgetState]] = &[&[
        WidgetState::DISABLED,
        WidgetState::PRESSED,
        WidgetState::HOVER,
    ]];
    RuleBuilder::with_exclusive(base(), GROUPS)
        .rule(
            on(WidgetState::HOVER),
            [fill(GREEN), PartProp::Opacity(0.5)],
        )
        .rule(
            on(WidgetState::FOCUSED),
            [PartProp::Corners(Corners::uniform(2.0))],
        )
        .rule(
            on(WidgetState::FOCUS_VISIBLE),
            [PartProp::Stroke(Stroke::uniform(2.0, BLUE))],
        )
        .rule(
            on(WidgetState::CHECKED | WidgetState::HOVER),
            [fill(RED), PartProp::Stroke(Stroke::uniform(1.0, RED))],
        )
        .rule(
            on(WidgetState::DISABLED | WidgetState::CHECKED),
            [PartProp::Opacity(0.25)],
        )
        .rule(
            on(WidgetState::SELECTED).except(WidgetState::CHECKED),
            [fill(Color::BLACK)],
        )
        .rule(any().size(ControlSize::Small), [PartProp::Opacity(1.0)])
        .build()
}

#[test]
fn asking_changes_no_value_and_no_stated_bit() {
    let rules = asking_rules();
    let asked = [
        WidgetState::HOVER,
        WidgetState::PRESSED,
        WidgetState::FOCUSED,
        WidgetState::DISABLED,
        WidgetState::FOCUS_VISIBLE,
        WidgetState::CHECKED,
        WidgetState::SELECTED,
        WidgetState::CHECKED | WidgetState::HOVER,
    ];
    for state in every_state() {
        for size in [ControlSize::default(), ControlSize::Small] {
            let query = StyleQuery::<()>::new(state).size(size);
            let plain = rules.eval(&query);
            for flags in asked {
                let out = rules.eval(&query.asking(flags));
                assert_eq!(
                    PartStyle {
                        from_state: plain.from_state,
                        ..out.clone()
                    },
                    plain,
                    "{state:?} {size:?} {flags:?}"
                );
                assert!(
                    plain.from_state.contains(out.from_state),
                    "{state:?} {flags:?}"
                );
            }
        }
    }
}

/// Each flag of [`WidgetState::ALL`], one at a time.
fn every_flag() -> impl Iterator<Item = WidgetState> {
    (0..u32::BITS)
        .filter_map(|i| 1u32.checked_shl(i))
        .map(WidgetState::from_bits_truncate)
        .filter(|flag| !flag.is_empty())
}

#[test]
fn asking_a_querys_only_flag_changes_nothing() {
    let rules = asking_rules();
    let body = built_body();
    assert_eq!(
        every_flag().fold(WidgetState::NONE, |acc, flag| acc | flag),
        WidgetState::ALL
    );
    for flag in every_flag() {
        let query = StyleQuery::<()>::new(flag);
        assert_eq!(
            rules.eval(&query.asking(flag)),
            rules.eval(&query),
            "{flag:?}"
        );
        let button = ButtonQuery::new(flag);
        assert_eq!(
            body.eval(&button.asking(flag)),
            body.eval(&button),
            "{flag:?}"
        );
        // A rule for each flag, so that every flag has a rule that matches its own query.
        for required in every_flag() {
            let one = one_rule(on(required));
            assert_eq!(
                one.eval(&query.asking(flag)),
                one.eval(&query),
                "{flag:?} {required:?}"
            );
        }
    }
}

#[test]
fn asked_focus_visible_at_a_focused_state_counts_a_focused_rule() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(on(WidgetState::FOCUSED), [fill(RED)])
        .rule(
            on(WidgetState::FOCUS_VISIBLE),
            [PartProp::Stroke(Stroke::uniform(2.0, BLUE))],
        )
        .rule(
            on(WidgetState::CHECKED),
            [PartProp::Corners(Corners::uniform(2.0))],
        )
        .rule(
            on(WidgetState::FOCUSED | WidgetState::CHECKED),
            [PartProp::Opacity(0.5)],
        )
        .build();
    let focused = StyleQuery::<()>::new(WidgetState::FOCUSED).asking(WidgetState::FOCUS_VISIBLE);
    let out = rules.eval(&focused);
    assert_eq!(fill_of(&out), Some(RED));
    assert_eq!(out.stroke, Stroke::default());
    assert_eq!(out.from_state, PartStated::FILL);
    // At a combined state the two-flag `FOCUSED` rule sets its bit too; the `CHECKED` rule
    // leaves its own.
    let checked = rules.eval(&focused.with_state(WidgetState::CHECKED));
    assert_eq!(checked.from_state, PartStated::FILL | PartStated::OPACITY);
}

#[test]
fn asked_a_size_bit_is_left_by_another_states_rule() {
    let rules: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(
            on(WidgetState::DISABLED),
            [PartProp::MinSize([Some(10.0), None])],
        )
        .rule(
            on(WidgetState::CHECKED | WidgetState::HOVER),
            [
                PartProp::MinSize([None, Some(20.0)]),
                PartProp::MaxSize([Some(30.0), None]),
            ],
        )
        .build();
    let plain = StyleQuery::<()>::new(DISABLED_CHECKED_HOVER);
    let unasked = rules.eval(&plain);
    assert_eq!(
        unasked.from_state,
        PartStated::MIN_SIZE | PartStated::MAX_SIZE
    );
    let asking = rules.eval(&plain.asking(WidgetState::DISABLED));
    assert_eq!(asking.min_size, [Some(10.0), Some(20.0)]);
    assert_eq!(asking.max_size, [Some(30.0), None]);
    // The `DISABLED` rule's width sets the minimum's bit, which the `CHECKED | HOVER` rule's
    // height leaves; the maximum's was never set.
    assert_eq!(asking.from_state, PartStated::MIN_SIZE);
    // A later rule without required states clears it.
    let cleared: RuleSet<PartStyle, PartProp> = RuleBuilder::new(base())
        .rule(
            on(WidgetState::DISABLED),
            [PartProp::MinSize([Some(10.0), None])],
        )
        .rule(any(), [PartProp::MinSize([None, Some(20.0)])])
        .build();
    assert!(
        cleared
            .eval(&plain.asking(WidgetState::DISABLED))
            .from_state
            .is_empty()
    );
}

#[test]
fn an_appended_rule_wins_over_the_builders() {
    let mut rules = built_body();
    let hover = ButtonQuery::new(WidgetState::HOVER);
    let before = rules.eval(&hover);
    assert_eq!(before.fill, Paint::Solid(GREEN));
    rules.append([
        (
            RuleSelector::on(WidgetState::HOVER),
            vec![fill(RED), PartProp::Opacity(0.5)],
        ),
        (
            RuleSelector::on(WidgetState::HOVER),
            vec![PartProp::Opacity(0.75)],
        ),
    ]);
    let after = rules.eval(&hover);
    assert_eq!(after.fill, Paint::Solid(RED));
    assert_eq!(after.opacity, 0.75);
    assert_eq!(after.from_state, before.from_state | PartStated::OPACITY);
    assert_eq!(after.stroke, before.stroke);
    // A query the appended rules do not match is what it was.
    let rest = ButtonQuery::default();
    assert_eq!(rules.eval(&rest), built_body().eval(&rest));
}

#[test]
fn appending_nothing_keeps_the_buffers() {
    let mut rules = built_body();
    let (own_rules, own_props) = (Arc::clone(&rules.rules), Arc::clone(&rules.props));
    rules.append(std::iter::empty::<(
        RuleSelector<ButtonMatcher>,
        Vec<PartProp>,
    )>());
    assert!(Arc::ptr_eq(&rules.rules, &own_rules));
    assert!(Arc::ptr_eq(&rules.props, &own_props));
}

#[test]
fn dead_rules_cover_the_appended_rules() {
    let mut rules = built_body();
    assert!(rules.dead_rules().is_empty());
    let count = rules.rules.len();
    rules.append([
        (RuleSelector::on(WidgetState::HOVER), vec![fill(RED)]),
        (
            RuleSelector::on(WidgetState::PRESSED | WidgetState::HOVER),
            vec![fill(RED)],
        ),
    ]);
    assert_eq!(rules.dead_rules(), vec![count + 1]);
}
