//! Allocation counts of the 0.8.0 model (spec §11, item 1; plan, rule 6),
//! through the public API: every part getter and `parts(query)` of the
//! first slice's ruled widgets on the resolved `kde-breeze`, light and dark,
//! over a list of states, and the clone of a returned `PartStyle` and of a
//! style holding every `Arc`-carrying property, allocate nothing; and the
//! resolved theme's heap bytes, the live bytes after `resolve()` minus
//! before it with the theme held, stay under 256 KB.
//!
//! dhat's testing mode counts every allocation of this binary through its
//! global allocator, so no `unsafe impl GlobalAlloc` is written here. It
//! allows one profiler at a time, so every measurement runs in the one test
//! below.

use std::hint::black_box;
use std::sync::Arc;

use native_theme::next::{
    BlendMode, ButtonQuery, Color, Corners, FocusIndicator, FocusKind, FocusTrigger, GradientStop,
    IconStated, ListQuery, MaterialKind, MaterialSpec, Orientation, Paint, PaintLayer, PartStated,
    PartStyle, Preset, ResolvedTheme, RippleSpec, ShadowLayer, Stroke, StrokeAlign, StrokeSide,
    StyleQuery, TextStated, Theme, ThemeContext, Timing, Transition, WidgetState,
};
use native_theme::next::{CaptionButtonKind, EdgeFlags, WindowState};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const ACCENT: Color = Color::rgb(0.2, 0.4, 0.8);

/// The resolved theme's heap budget (spec §11, item 1).
const HEAP_BUDGET_BYTES: usize = 256 * 1024;

/// The states every getter is queried with: rest, hover, pressed,
/// focus-visible, disabled, checked, selected, and two combinations.
const STATES: [WidgetState; 9] = [
    WidgetState::NONE,
    WidgetState::HOVER,
    WidgetState::PRESSED,
    WidgetState::FOCUS_VISIBLE,
    WidgetState::DISABLED,
    WidgetState::CHECKED,
    WidgetState::SELECTED,
    WidgetState::HOVER.union(WidgetState::CHECKED),
    WidgetState::DISABLED.union(WidgetState::SELECTED),
];

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

/// A style holding every `Arc`-carrying property, for the clone.
fn full_style() -> PartStyle {
    let mut style = PartStyle::default();
    style.fill = complex_paint();
    style.stroke = complex_stroke();
    style.shadows = Some(Arc::from(vec![ShadowLayer {
        offset_y: 1.0,
        blur: 3.0,
        color: Color::BLACK.with_alpha(0.25),
        ..ShadowLayer::default()
    }]));
    style.focus = Some(Arc::new(FocusIndicator::new(
        FocusKind::Ring,
        complex_stroke(),
        Stroke::uniform(1.0, Color::WHITE),
        2.0,
        Some(Corners::uniform(4.0)),
        Some(FocusTrigger::KeyboardOnly),
    )));
    style.ripple = Some(Arc::new(RippleSpec::new(
        ACCENT.with_alpha(0.1),
        true,
        150,
        300,
    )));
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

/// Every getter and `parts` of the six widgets' ruled parts for `state`, and
/// the clone of a returned part style, each result kept from the optimiser.
fn query_every_part(theme: &ResolvedTheme, state: WidgetState) {
    let q = StyleQuery::<()>::new(state);
    let button = ButtonQuery::new(state);
    let list = ListQuery::new(state);
    let body = black_box(theme.button.body(button));
    black_box(body.clone());
    black_box(theme.button.label(button));
    black_box(theme.button.icon(button));
    black_box(theme.button.parts(button));
    black_box(theme.checkbox.indicator(q));
    black_box(theme.checkbox.checkmark(q));
    black_box(theme.checkbox.label(q));
    black_box(theme.checkbox.parts(q));
    black_box(theme.radio.indicator(q));
    black_box(theme.radio.dot(q));
    black_box(theme.radio.label(q));
    black_box(theme.radio.parts(q));
    black_box(theme.scrollbar.track(q));
    black_box(theme.scrollbar.thumb(q));
    black_box(theme.scrollbar.parts(q));
    black_box(theme.list.container(list));
    black_box(theme.list.row(list));
    black_box(theme.list.item_label(list));
    black_box(theme.list.header(list));
    black_box(theme.list.header_label(list));
    black_box(theme.list.header_separator(list));
    black_box(theme.list.parts(list));
    // The axes the builder has rules for: the primary variant, an alternate
    // row and both orientations of the scroll bar.
    let primary = ButtonQuery::primary().state(state);
    black_box(theme.button.body(primary));
    black_box(theme.button.label(primary));
    black_box(theme.button.parts(primary));
    let alternate = ListQuery::new(state).alternate(true);
    black_box(theme.list.row(alternate));
    black_box(theme.list.item_label(alternate));
    black_box(theme.list.parts(alternate));
    for orientation in [Orientation::Horizontal, Orientation::Vertical] {
        let bar = q.orientation(orientation);
        black_box(theme.scrollbar.track(bar));
        black_box(theme.scrollbar.thumb(bar));
        black_box(theme.scrollbar.parts(bar));
    }
    // The window chrome: a normal window and a maximized, tiled one.
    let mut tiled = WindowState::default();
    tiled.maximized = true;
    tiled.tiled = EdgeFlags::LEFT | EdgeFlags::TOP;
    for window in [WindowState::default(), tiled] {
        black_box(theme.window.frame(window));
        black_box(theme.window.title_bar(window));
        black_box(theme.window.title_bar_height(window));
        black_box(theme.window.title(window));
    }
    for kind in [CaptionButtonKind::Close, CaptionButtonKind::Minimize] {
        black_box(theme.window.caption_button(kind, state));
        black_box(theme.window.caption_icon(kind, state));
    }
}

#[test]
fn queries_and_clones_allocate_nothing_and_the_theme_fits_its_budget() {
    let _profiler = dhat::Profiler::builder().testing().build();

    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let source = Theme::preset(Preset::KdeBreeze);
        let before = dhat::HeapStats::get();
        let theme = source.resolve(black_box(&ctx));
        let after = dhat::HeapStats::get();
        let heap_bytes = after.curr_bytes - before.curr_bytes;
        println!(
            "kde-breeze {:?}: the resolved theme holds {heap_bytes} heap bytes in {} blocks",
            ctx.scheme,
            after.curr_blocks - before.curr_blocks
        );
        dhat::assert!(heap_bytes < HEAP_BUDGET_BYTES);

        for state in STATES {
            let before = dhat::HeapStats::get();
            query_every_part(black_box(&theme), state);
            let after = dhat::HeapStats::get();
            dhat::assert_eq!(after.total_blocks, before.total_blocks);
            dhat::assert_eq!(after.curr_bytes, before.curr_bytes);
        }

        // The queries did their work: the builder's rules apply.
        let hover = theme.button.body(ButtonQuery::new(WidgetState::HOVER));
        dhat::assert!(hover.stated.contains(PartStated::FILL | PartStated::STROKE));
        dhat::assert!(hover.stroke != theme.button.body(WidgetState::NONE).stroke);
        let checked = theme.checkbox.checkmark(WidgetState::CHECKED);
        dhat::assert!(
            checked
                .stated
                .contains(IconStated::COLOR | IconStated::STROKE_WIDTH)
        );
        let selected = theme.list.item_label(ListQuery::new(WidgetState::SELECTED));
        dhat::assert!(selected.stated.contains(TextStated::COLOR));
        dhat::assert_eq!(selected.color, theme.colors.selection.foreground);
        dhat::assert!(
            theme.scrollbar.thumb(WidgetState::PRESSED).fill
                != theme.scrollbar.thumb(WidgetState::NONE).fill
        );
        dhat::assert_eq!(
            theme.button.body(ButtonQuery::primary()).fill,
            Paint::Solid(theme.colors.accent.fill)
        );
        dhat::assert!(
            theme
                .list
                .row(ListQuery::new(WidgetState::NONE).alternate(true))
                .stated
                .contains(PartStated::FILL)
        );
        let vertical = StyleQuery::<()>::new(WidgetState::NONE).orientation(Orientation::Vertical);
        let horizontal = vertical.orientation(Orientation::Horizontal);
        dhat::assert!(theme.scrollbar.thumb(vertical).min_size[0].is_some());
        dhat::assert!(theme.scrollbar.thumb(horizontal).min_size[1].is_some());
        drop(black_box(theme));
    }

    // The clone of a style holding every `Arc`-carrying property.
    let style = full_style();
    let before = dhat::HeapStats::get();
    let cloned = black_box(&style).clone();
    let after = dhat::HeapStats::get();
    dhat::assert_eq!(after.total_blocks, before.total_blocks);
    dhat::assert_eq!(after.curr_bytes, before.curr_bytes);
    dhat::assert_eq!(&cloned, &style);

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
