//! Allocation counts of the 0.8.0 model (spec §11, item 1; plan, rule 6):
//! `RuleSet::eval` and `PartStyle::clone` allocate nothing, on a style and a
//! rule set holding every `Arc`-carrying property, and dhat reports the live
//! heap bytes the resolved-theme budget will be measured with.
//!
//! dhat's testing mode counts every allocation of this binary through its
//! global allocator, so no `unsafe impl GlobalAlloc` is written here. It
//! allows one profiler at a time, so every measurement runs in the one test
//! below.

use std::hint::black_box;
use std::sync::Arc;

use native_theme::next::{
    BlendMode, Color, Corners, FocusIndicator, FocusKind, FocusTrigger, GradientStop, MaterialKind,
    MaterialSpec, Paint, PaintLayer, PartProp, PartStated, PartStyle, RippleSpec, RuleSelector,
    RuleSet, ShadowLayer, Stroke, StrokeAlign, StrokeSide, StyleQuery, Timing, Transition,
    WidgetState,
};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const ACCENT: Color = Color::rgb(0.2, 0.4, 0.8);

fn complex_paint() -> Paint {
    Paint::layers(vec![
        PaintLayer::Material(MaterialSpec::new(MaterialKind::Acrylic, Color::WHITE)),
        PaintLayer::LinearGradient {
            angle_deg: 90.0,
            stops: Arc::from(vec![
                GradientStop {
                    offset: 0.0,
                    color: Color::WHITE,
                },
                GradientStop {
                    offset: 1.0,
                    color: ACCENT,
                },
            ]),
            blend: BlendMode::Normal,
        },
    ])
}

fn complex_stroke() -> Stroke {
    let side = StrokeSide {
        width: 1.0,
        paint: complex_paint(),
    };
    Stroke::sides(
        side.clone(),
        side.clone(),
        side.clone(),
        side,
        StrokeAlign::Inside,
    )
}

fn shadows() -> Option<Arc<[ShadowLayer]>> {
    Some(Arc::from(vec![ShadowLayer {
        offset_y: 1.0,
        blur: 3.0,
        color: Color::BLACK.with_alpha(0.25),
        ..ShadowLayer::default()
    }]))
}

fn focus() -> Option<Arc<FocusIndicator>> {
    Some(Arc::new(FocusIndicator::new(
        FocusKind::Ring,
        complex_stroke(),
        Stroke::uniform(1.0, Color::WHITE),
        2.0,
        Some(Corners::uniform(4.0)),
        Some(FocusTrigger::KeyboardOnly),
    )))
}

fn ripple() -> Option<Arc<RippleSpec>> {
    Some(Arc::new(RippleSpec::new(
        ACCENT.with_alpha(0.1),
        true,
        150,
        300,
    )))
}

fn full_style() -> PartStyle {
    let mut style = PartStyle::default();
    style.fill = complex_paint();
    style.stroke = complex_stroke();
    style.shadows = shadows();
    style.focus = focus();
    style.ripple = ripple();
    style.transition = Some(Transition {
        duration_ms: 150,
        delay_ms: 0,
        timing: Timing::CubicBezier(0.4, 0.0, 0.2, 1.0),
    });
    style.stated = PartStated::FILL
        | PartStated::STROKE
        | PartStated::SHADOWS
        | PartStated::FOCUS
        | PartStated::RIPPLE
        | PartStated::TRANSITION;
    style
}

fn selector(required: WidgetState) -> RuleSelector {
    let mut s = RuleSelector::default();
    s.required_states = required;
    s
}

fn full_rule_set() -> RuleSet<PartStyle, PartProp> {
    const GROUPS: &[&[WidgetState]] = &[&[
        WidgetState::DISABLED,
        WidgetState::PRESSED,
        WidgetState::HOVER,
    ]];
    RuleSet::new(
        full_style(),
        GROUPS,
        vec![
            (
                selector(WidgetState::HOVER),
                vec![
                    PartProp::Fill(complex_paint()),
                    PartProp::Stroke(complex_stroke()),
                    PartProp::Shadows(shadows()),
                ],
            ),
            (
                selector(WidgetState::FOCUSED),
                vec![PartProp::Focus(focus()), PartProp::Ripple(ripple())],
            ),
            (
                selector(WidgetState::PRESSED),
                vec![PartProp::Fill(Paint::solid(ACCENT))],
            ),
        ],
    )
}

#[test]
fn eval_and_clone_allocate_nothing_and_live_bytes_are_reported() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let style = full_style();
    let rules = full_rule_set();
    let query = StyleQuery::<()>::new(WidgetState::HOVER | WidgetState::FOCUS_VISIBLE);

    let before = dhat::HeapStats::get();
    let cloned = black_box(&style).clone();
    let evaluated = black_box(&rules).eval(black_box(&query));
    let after = dhat::HeapStats::get();
    dhat::assert_eq!(after.total_blocks, before.total_blocks);
    dhat::assert_eq!(after.curr_bytes, before.curr_bytes);

    // The evaluation did its work: the hover and focus rules applied.
    dhat::assert_eq!(&cloned, &style);
    dhat::assert!(evaluated.focus.is_some() && evaluated.ripple.is_some());
    dhat::assert!(matches!(evaluated.fill, Paint::Complex(_)));
    drop(black_box(cloned));
    drop(black_box(evaluated));

    // A known allocation is counted: one block, its bytes live until dropped.
    let before = dhat::HeapStats::get();
    let block: Vec<u8> = black_box(Vec::with_capacity(4096));
    let during = dhat::HeapStats::get();
    dhat::assert_eq!(during.total_blocks, before.total_blocks + 1);
    dhat::assert_eq!(during.curr_blocks, before.curr_blocks + 1);
    dhat::assert_eq!(during.curr_bytes, before.curr_bytes + 4096);
    drop(block);
    let after = dhat::HeapStats::get();
    dhat::assert_eq!(after.curr_bytes, before.curr_bytes);
}
