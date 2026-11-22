use super::*;
use crate::next::geometry::StrokeAlign;
use crate::next::query::{ButtonQuery, InputQuery, InputVariant, ListQuery};
use crate::next::style::{DecorationKind, FocusKind};

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
    const GROUPS: &[&[WidgetState]] = &[&[
        WidgetState::DISABLED,
        WidgetState::PRESSED,
        WidgetState::HOVER,
    ]];
    let rules: RuleSet<PartStyle, PartProp> = RuleSet::new(
        base(),
        GROUPS,
        vec![
            (on(WidgetState::HOVER), vec![fill(RED)]),
            (on(WidgetState::PRESSED), vec![fill(GREEN)]),
        ],
    );
    let pressed_hover = StyleQuery::<()>::new(WidgetState::PRESSED | WidgetState::HOVER);
    assert_eq!(fill_of(&rules.eval(&pressed_hover)), Some(GREEN));
    let disabled_hover = StyleQuery::<()>::new(WidgetState::DISABLED | WidgetState::HOVER);
    assert_eq!(fill_of(&rules.eval(&disabled_hover)), Some(BLUE));
    let hover = StyleQuery::<()>::new(WidgetState::HOVER);
    assert_eq!(fill_of(&rules.eval(&hover)), Some(RED));
    assert_eq!(
        normalize(
            WidgetState::DISABLED
                | WidgetState::PRESSED
                | WidgetState::HOVER
                | WidgetState::CHECKED,
            GROUPS
        ),
        WidgetState::DISABLED | WidgetState::CHECKED
    );
    assert_eq!(
        normalize(WidgetState::CHECKED, GROUPS),
        WidgetState::CHECKED
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
        PartStated::ALL
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
    let rules: RuleSet<PartStyle, PartProp> = RuleSet {
        base: base(),
        exclusive: &[],
        rules: Arc::from(vec![
            entry(0, 1),
            entry(1, 1),
            entry(0, 2),
            entry(u32::MAX, u32::MAX),
            entry(0, u32::MAX),
        ]),
        props: Arc::from(vec![fill(RED)]),
    };
    let out = rules.eval(&StyleQuery::<()>::default());
    assert_eq!(fill_of(&out), Some(RED));
    assert_eq!(out.stated, PartStated::FILL);
}
