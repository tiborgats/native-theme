//! Allocation counts of the 0.8.0 model (spec §11, item 1; plan, rule 6):
//! `RuleSet::eval` of parts, texts and icons and `PartStyle::clone` allocate
//! nothing — the clone on a style holding every `Arc`-carrying property, each
//! `eval` on a rule set whose rules apply them — and dhat reports the live
//! heap bytes the resolved-theme budget will be measured with.
//!
//! dhat's testing mode counts every allocation of this binary through its
//! global allocator, so no `unsafe impl GlobalAlloc` is written here. It
//! allows one profiler at a time, so every measurement runs in the one test
//! below.

use std::hint::black_box;
use std::sync::Arc;

use native_theme::next::{
    __alloc_test_fixtures, BlendMode, Color, Corners, FocusIndicator, FocusKind, FocusTrigger,
    GradientStop, IconProp, IconStated, IconStyle, MaterialKind, MaterialSpec, Paint, PaintLayer,
    PartProp, PartStated, PartStyle, RippleSpec, RuleSet, ShadowLayer, Stroke, StrokeAlign,
    StrokeSide, StyleQuery, TextProp, TextStated, TextStyle, Timing, Transition, WidgetState,
};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const ACCENT: Color = Color::rgb(0.2, 0.4, 0.8);

fn complex_paint() -> Paint {
    Paint::layers(vec![
        PaintLayer::Material(MaterialSpec::new(
            MaterialKind::Acrylic,
            30.0,
            Color::WHITE.with_alpha(0.5),
            1.0,
            0.5,
            0.02,
            Color::WHITE,
        )),
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

fn shadows() -> Arc<[ShadowLayer]> {
    Arc::from(vec![ShadowLayer {
        offset_y: 1.0,
        blur: 3.0,
        color: Color::BLACK.with_alpha(0.25),
        ..ShadowLayer::default()
    }])
}

fn focus() -> Arc<FocusIndicator> {
    Arc::new(FocusIndicator::new(
        FocusKind::Ring,
        complex_stroke(),
        Stroke::uniform(1.0, Color::WHITE),
        2.0,
        Some(Corners::uniform(4.0)),
        Some(FocusTrigger::KeyboardOnly),
    ))
}

fn ripple() -> Arc<RippleSpec> {
    Arc::new(RippleSpec::new(ACCENT.with_alpha(0.1), true, 150, 300))
}

fn transition() -> Transition {
    Transition {
        duration_ms: 150,
        delay_ms: 0,
        timing: Timing::CubicBezier(0.4, 0.0, 0.2, 1.0),
    }
}

/// A style holding every `Arc`-carrying property, for the clone.
fn full_style() -> PartStyle {
    let mut style = PartStyle::default();
    style.fill = complex_paint();
    style.stroke = complex_stroke();
    style.shadows = Some(shadows());
    style.focus = Some(focus());
    style.ripple = Some(ripple());
    style.transition = Some(transition());
    style.stated = PartStated::FILL
        | PartStated::STROKE
        | PartStated::SHADOWS
        | PartStated::FOCUS
        | PartStated::RIPPLE
        | PartStated::TRANSITION;
    style
}

/// A base with a solid fill and nothing else, so that what `eval` returns
/// beyond it can only come from the rules.
fn plain_base() -> PartStyle {
    let mut style = PartStyle::default();
    style.fill = Paint::solid(ACCENT);
    style.stated = PartStated::FILL;
    style
}

/// The `Arc`s the part rules apply, kept to compare by pointer.
struct PartArcs {
    fill: Paint,
    stroke: Stroke,
    shadows: Arc<[ShadowLayer]>,
    focus: Arc<FocusIndicator>,
    ripple: Arc<RippleSpec>,
}

/// The three rule sets, built by the crate's rule builder through its test
/// hook: the part rules apply `arcs` on `HOVER` and, on `FOCUSED`, the focus
/// and the ripple; the text and icon rules apply their properties on `HOVER`.
fn rule_sets(
    arcs: &PartArcs,
    family: &Arc<str>,
) -> (
    RuleSet<PartStyle, PartProp>,
    RuleSet<TextStyle, TextProp>,
    RuleSet<IconStyle, IconProp>,
) {
    __alloc_test_fixtures(
        plain_base(),
        vec![
            PartProp::Fill(arcs.fill.clone()),
            PartProp::Stroke(arcs.stroke.clone()),
            PartProp::Shadows(Some(arcs.shadows.clone())),
            PartProp::Transition(Some(transition())),
        ],
        // Matches a `FOCUS_VISIBLE` query only through step 0 of the
        // evaluation contract.
        vec![
            PartProp::Focus(Some(arcs.focus.clone())),
            PartProp::Ripple(Some(arcs.ripple.clone())),
        ],
        vec![PartProp::Fill(Paint::solid(Color::BLACK))],
        vec![TextProp::Family(family.clone()), TextProp::SizePx(13.0)],
        vec![IconProp::Size(16.0), IconProp::Color(ACCENT)],
    )
}

fn same_paint(a: &Paint, b: &Paint) -> bool {
    match (a, b) {
        (Paint::Complex(x), Paint::Complex(y)) => Arc::ptr_eq(x, y),
        _ => false,
    }
}

fn same_stroke(a: &Stroke, b: &Stroke) -> bool {
    match (a, b) {
        (Stroke::Complex(x), Stroke::Complex(y)) => Arc::ptr_eq(x, y),
        _ => false,
    }
}

#[test]
fn eval_and_clone_allocate_nothing_and_live_bytes_are_reported() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let style = full_style();
    let arcs = PartArcs {
        fill: complex_paint(),
        stroke: complex_stroke(),
        shadows: shadows(),
        focus: focus(),
        ripple: ripple(),
    };
    let family: Arc<str> = Arc::from("Noto Sans");
    let (rules, text_rules, icon_rules) = rule_sets(&arcs, &family);
    let query = StyleQuery::<()>::new(WidgetState::HOVER | WidgetState::FOCUS_VISIBLE);

    let before = dhat::HeapStats::get();
    let cloned = black_box(&style).clone();
    let evaluated = black_box(&rules).eval(black_box(&query));
    let text = black_box(&text_rules).eval(black_box(&query));
    let icon = black_box(&icon_rules).eval(black_box(&query));
    let after = dhat::HeapStats::get();
    dhat::assert_eq!(after.total_blocks, before.total_blocks);
    dhat::assert_eq!(after.curr_bytes, before.curr_bytes);

    // The evaluations did their work: every rule that the query matches
    // applied, the focus rule through `FOCUS_VISIBLE` → `FOCUSED`, and each
    // `Arc` is the rule's own.
    dhat::assert_eq!(&cloned, &style);
    dhat::assert!(evaluated.stated.contains(
        PartStated::FILL
            | PartStated::STROKE
            | PartStated::SHADOWS
            | PartStated::TRANSITION
            | PartStated::FOCUS
            | PartStated::RIPPLE
    ));
    dhat::assert!(same_paint(&evaluated.fill, &arcs.fill));
    dhat::assert!(same_stroke(&evaluated.stroke, &arcs.stroke));
    dhat::assert!(
        evaluated
            .shadows
            .as_ref()
            .is_some_and(|s| Arc::ptr_eq(s, &arcs.shadows))
    );
    dhat::assert!(
        evaluated
            .focus
            .as_ref()
            .is_some_and(|f| Arc::ptr_eq(f, &arcs.focus))
    );
    dhat::assert!(
        evaluated
            .ripple
            .as_ref()
            .is_some_and(|r| Arc::ptr_eq(r, &arcs.ripple))
    );
    dhat::assert!(
        text.stated
            .contains(TextStated::FAMILY | TextStated::SIZE_PX)
    );
    dhat::assert!(Arc::ptr_eq(&text.family, &family));
    dhat::assert!(icon.stated.contains(IconStated::SIZE | IconStated::COLOR));
    dhat::assert_eq!(icon.color, ACCENT);
    drop(black_box(cloned));
    drop(black_box(evaluated));
    drop(black_box(text));
    black_box(icon);

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
