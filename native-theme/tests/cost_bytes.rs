//! The load budget's bytes (the 0.8.0 plan's *The load budget*, lines 2, 3,
//! 9 and 11; ADD-A147), which the budget step (`scripts/check_cost_budget.py`)
//! compares: counted by `peak_alloc`'s counting allocator, in a test binary
//! of its own (its atomics would slow the times' binary), run with one test
//! thread, since the allocator counts every thread's allocations; and the
//! preset files' sizes. Ignored by default; run on the release profile:
//!
//! `cargo test -p native-theme --release --all-features --test cost_bytes -- --ignored --nocapture --test-threads=1`
//!
//! No byte count here is asserted: the figures go to the budget step.

#![allow(clippy::print_stdout)]

mod cost_common;

use cost_common::{Case, Kind, Measure, PresetFile, Res, Route, SCHEME};
use native_theme::{InputKind, PresetId, Theme, WindowConditions, WindowState};
use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK: PeakAlloc = PeakAlloc;

/// This probe's test, which each preset's cases run again in a process of
/// their own.
const TEST: &str = "bundled_preset_bytes";

fn in_use() -> usize {
    PEAK.current_usage()
}

/// The highest count of bytes in use while `f` runs, above the count
/// before it, and `f`'s result.
fn peak_of<T>(f: impl FnOnce() -> Res<T>) -> Res<(usize, T)> {
    PEAK.reset_peak_usage();
    let before = in_use();
    let value = f()?;
    Ok((PEAK.peak_usage().saturating_sub(before), value))
}

fn print_bytes(name: &str, bytes: usize, extra: &str) {
    println!("cost-bytes {name} bytes={bytes}{extra}");
}

/// The asked cases of one preset, in a process of its own, so that line 2's
/// load is the first in the process (its `plain` case is asked alone): the read (lines 2 and 3, `read`), the
/// plain load (line 2, `plain`), the theme with one window's resolution
/// (line 3) and a resolve's peak over it (line 11), the file's size (line 9).
fn one_preset(cases: &[Case]) -> Res<()> {
    let named = |test: &dyn Fn(Measure) -> bool| cases.iter().find(|case| test(case.measure));
    let Some(id) = cases.first().map(|case| case.preset) else {
        return Ok(());
    };
    let peak_read = named(&|m| m == Measure::LoadPeak(Route::Read));
    let kept_read = named(&|m| m == Measure::Kept(Route::Read));
    if peak_read.is_some() || kept_read.is_some() {
        let (peak, read) = peak_of(|| {
            Ok(native_theme::gate::read_cost(
                id,
                Some(&in_use as &dyn Fn() -> usize),
            )?)
        })?;
        if let Some(case) = peak_read {
            print_bytes(&case.name, peak, "");
        }
        if let Some(case) = kept_read {
            let held = read.held.ok_or("the read counted no bytes held")?;
            let per_rule = if read.rules == 0 {
                "absent".to_owned()
            } else {
                format!("{:.1}", held as f64 / read.rules as f64)
            };
            print_bytes(
                &case.name,
                held,
                &format!(" rules={} held_per_rule={per_rule}", read.rules),
            );
        }
    }
    if let Some(case) = named(&|m| m == Measure::LoadPeak(Route::Plain)) {
        let (peak, theme) = peak_of(|| Ok(Theme::from_preset(id, SCHEME)?))?;
        drop(theme);
        print_bytes(&case.name, peak, "");
    }
    let kept = named(&|m| matches!(m, Measure::Kept(Route::Synthetic | Route::Plain)));
    let resolve_peak = named(&|m| matches!(m, Measure::ResolvePeak(_)));
    let route = kept.or(resolve_peak).and_then(|case| match case.measure {
        Measure::Kept(route) | Measure::ResolvePeak(route) => Some(route),
        _ => None,
    });
    if let Some(route) = route {
        let conditions = WindowConditions::new(InputKind::Mouse);
        let file = PresetFile::read(id)?;
        let own = file.own()?;
        let before = in_use();
        let theme = cost_common::theme(id, route, &file, &own)?;
        let resolution = theme.resolve(&mut WindowState::default(), &conditions);
        let held = in_use().saturating_sub(before);
        if let Some(case) = kept {
            print_bytes(&case.name, held, "");
        }
        if let Some(case) = resolve_peak {
            let (peak, again) =
                peak_of(|| Ok(theme.resolve(&mut WindowState::default(), &conditions)))?;
            drop(again);
            print_bytes(&case.name, peak, "");
        }
        drop(resolution);
    }
    if let Some(case) = named(&|m| m == Measure::File) {
        let bytes = usize::try_from(std::fs::metadata(cost_common::preset_path(id))?.len())?;
        print_bytes(&case.name, bytes, "");
    }
    Ok(())
}

/// The load budget's bytes (*The load budget*, lines 2, 3, 9 and 11), for
/// the budget step, which compares them; nothing is asserted here.
///
/// `NT_COST_CASES=<case>,<case>,…` measures those cases, by name, each
/// preset's in a process of its own (`NT_COST_CHILD`; line 2's `plain` in
/// one more, so that each load of line 2 is its process's first), and prints each
/// case's figure (`cost-bytes <case> bytes=…`; the read's held bytes also
/// with its `[[rules]]` and the bytes held per rule), or `cost-error <case>
/// <why>`. Without it, it lists every bytes case it can measure (`cost-case
/// <case> line=… kind=bytes|exact`).
///
/// `NT_COST_CASES=2.adwaita.read,9.adwaita.file <the cost_bytes binary>
/// --ignored --nocapture --test-threads=1 --exact bundled_preset_bytes`
#[test]
#[ignore = "the load budget's probe; the budget step runs it"]
fn bundled_preset_bytes() -> Res<()> {
    let all: Vec<Case> = cost_common::cases()?
        .into_iter()
        .filter(|case| case.kind() != Kind::Time)
        .collect();
    let Ok(names) = std::env::var("NT_COST_CASES") else {
        for case in &all {
            println!("{}", case.listed());
        }
        return Ok(());
    };
    let cases = cost_common::asked(&names, &all)
        .map_err(|name| format!("{name}: not a case this binary can measure"))?;
    if std::env::var("NT_COST_CHILD").is_ok() {
        return one_preset(&cases);
    }
    // Line 2's plain load has a process of its own beside the read's, each
    // the first load in its process.
    let alone = |case: &Case| case.measure == Measure::LoadPeak(Route::Plain);
    let mut groups: Vec<(PresetId, bool)> = Vec::new();
    for case in &cases {
        let group = (case.preset, alone(case));
        if !groups.contains(&group) {
            groups.push(group);
        }
    }
    for (id, plain) in groups {
        let names: Vec<&str> = cases
            .iter()
            .filter(|case| case.preset == id && alone(case) == plain)
            .map(|case| case.name.as_str())
            .collect();
        cost_common::run_child(TEST, id.key(), &names)?;
    }
    Ok(())
}
