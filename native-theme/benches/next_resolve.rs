//! The speed of the 0.8.0 model (spec §11, item 2): `Theme::resolve` and part
//! queries for every bundled preset whose builder is written (`kde-breeze`,
//! light and dark). Each measurement times many batches of calls, a batch
//! long enough for the clock's resolution, and prints the median time per
//! call with the 10th and the 90th percentile, in nanoseconds. Nothing is
//! asserted: a timing fails on a loaded machine. The resolved themes are
//! dropped outside the timed region, and their drop is reported on its own
//! line.
//!
//! `cargo bench -p native-theme --bench next_resolve`

use std::hint::black_box;
use std::time::{Duration, Instant};

use native_theme::next::{
    ButtonQuery, ListQuery, Orientation, Preset, ResolvedTheme, StyleQuery, Theme, ThemeContext,
    WidgetState,
};

/// The batches each measurement times.
const BATCHES: usize = 201;

/// The least wall time of a batch: the clock's resolution is nanoseconds, so
/// its error is below a millionth of a batch.
const MIN_BATCH: Duration = Duration::from_millis(1);

fn main() {
    for (scheme, ctx) in [
        ("light", ThemeContext::light()),
        ("dark", ThemeContext::dark()),
    ] {
        let preset = Preset::KdeBreeze;
        let name = |what: &str| format!("{} {scheme}: {what}", preset.as_str());
        measure_resolve(
            &name("resolve"),
            &name("drop of the resolved theme"),
            preset,
            &ctx,
        );
        let theme = Theme::preset(preset).resolve(&ctx);
        measure_queries(&theme, &name);
    }
}

/// The part queries on `theme`, each named by `name`.
fn measure_queries(theme: &ResolvedTheme, name: &dyn Fn(&str) -> String) {
    let hover = WidgetState::HOVER;
    let rest = ButtonQuery::new(WidgetState::NONE);
    measure(&name("button.body at rest"), || {
        black_box(theme.button.body(black_box(rest)));
    });
    let hovered = ButtonQuery::new(hover);
    measure(&name("button.body(HOVER)"), || {
        black_box(theme.button.body(black_box(hovered)));
    });
    let primary = ButtonQuery::primary().state(WidgetState::PRESSED);
    measure(&name("button.body(primary, PRESSED)"), || {
        black_box(theme.button.body(black_box(primary)));
    });
    let checked = StyleQuery::new(WidgetState::CHECKED.union(hover));
    measure(&name("checkbox.indicator(CHECKED | HOVER)"), || {
        black_box(theme.checkbox.indicator(black_box(checked)));
    });
    let row = ListQuery::new(WidgetState::SELECTED.union(hover)).alternate(true);
    measure(&name("list.row(alternate, SELECTED | HOVER)"), || {
        black_box(theme.list.row(black_box(row)));
    });
    let thumb = StyleQuery::new(hover).orientation(Orientation::Vertical);
    measure(&name("scrollbar.thumb(vertical, HOVER)"), || {
        black_box(theme.scrollbar.thumb(black_box(thumb)));
    });
    measure(&name("button.parts(HOVER)"), || {
        black_box(theme.button.parts(black_box(hovered)));
    });
}

/// `Theme::preset(preset).resolve(ctx)` timed with the resolved themes held,
/// reported as `name`; their drop after each batch timed apart, reported as
/// `drop_name`.
fn measure_resolve(name: &str, drop_name: &str, preset: Preset, ctx: &ThemeContext) {
    let calls = calls_per_batch(|| {
        black_box(Theme::preset(black_box(preset)).resolve(black_box(ctx)));
    });
    let mut resolve = Vec::with_capacity(BATCHES);
    let mut drop = Vec::with_capacity(BATCHES);
    for _ in 0..BATCHES {
        let mut held: Vec<ResolvedTheme> = Vec::with_capacity(calls);
        let start = Instant::now();
        for _ in 0..calls {
            held.push(black_box(
                Theme::preset(black_box(preset)).resolve(black_box(ctx)),
            ));
        }
        resolve.push(per_call(start.elapsed(), calls));
        let start = Instant::now();
        std::mem::drop(black_box(held));
        drop.push(per_call(start.elapsed(), calls));
    }
    report(name, resolve);
    report(drop_name, drop);
}

/// `call` timed in batches, reported as `name`.
fn measure(name: &str, mut call: impl FnMut()) {
    let calls = calls_per_batch(&mut call);
    let times = (0..BATCHES)
        .map(|_| {
            let start = Instant::now();
            for _ in 0..calls {
                call();
            }
            per_call(start.elapsed(), calls)
        })
        .collect();
    report(name, times);
}

/// The number of calls of `call` that last at least [`MIN_BATCH`], doubled
/// from one.
fn calls_per_batch(mut call: impl FnMut()) -> usize {
    let mut calls: usize = 1;
    loop {
        let start = Instant::now();
        for _ in 0..calls {
            call();
        }
        if start.elapsed() >= MIN_BATCH || calls == usize::MAX {
            return calls;
        }
        calls = calls.saturating_mul(2);
    }
}

/// The time per call of `calls` calls that took `elapsed`, in nanoseconds.
fn per_call(elapsed: Duration, calls: usize) -> f64 {
    elapsed.as_nanos() as f64 / calls as f64
}

/// One line for `name`: the median of `times` and their 10th and 90th
/// percentile, each the nearest rank.
fn report(name: &str, mut times: Vec<f64>) {
    times.sort_by(f64::total_cmp);
    let last = times.len().saturating_sub(1);
    let rank = |percent: usize| times.get(last.saturating_mul(percent) / 100).copied();
    match (rank(50), rank(10), rank(90)) {
        (Some(median), Some(p10), Some(p90)) => {
            println!("{name:<56} median {median:>9.1} ns  p10 {p10:>9.1}  p90 {p90:>9.1}");
        }
        _ => println!("{name}: no batch timed"),
    }
}
