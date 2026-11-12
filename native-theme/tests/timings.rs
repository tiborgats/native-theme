//! Wall-clock timings: icon loading, which the code review measures, and the
//! load budget's times, which the budget step (`scripts/check_cost_budget.py`)
//! compares (the 0.8.0 plan's *The load budget*; ADD-A147). Ignored by
//! default; run on the release profile:
//!
//! `cargo test -p native-theme --release --all-features --test timings -- --ignored --nocapture --test-threads=1`
//!
//! No time here is asserted: the figures go to the budget step.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

mod cost_common;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use cost_common::{Body, Case, Change, Measure, PresetFile, REFERENCE_LAYERS, Res, Route, SCHEME};
use native_theme::{
    Accent, ButtonPart, Color, Contrast, Direction, Element, FontMetrics, InputKind, InputName,
    InputValue, Key, Layer, PartRef, PresetId, Px, ResolvedFont, RootInputs, Scheme, SchemeMap,
    Scope, Size, StateFlag, StateFlags, Theme, TypedPart, Value, WindowConditions, WindowState,
};

// Every icon probe loads icons, `system-icons` enabling `material-icons`.
#[cfg(feature = "material-icons")]
mod icons {
    use std::time::{Duration, Instant};

    fn median<T>(runs: usize, mut f: impl FnMut() -> T) -> Duration {
        let mut times: Vec<Duration> = (0..runs)
            .map(|_| {
                let start = Instant::now();
                std::hint::black_box(f());
                start.elapsed()
            })
            .collect();
        times.sort();
        times.get(runs / 2).copied().unwrap_or_default()
    }

    #[test]
    #[ignore = "timing probe; run with --ignored --nocapture"]
    fn bundled_icons() {
        let all = median(11, || {
            native_theme::icons::IconRole::ALL
                .iter()
                .filter_map(|role| native_theme::icons::MaterialLoader::new(*role).load())
                .count()
        });
        println!(
            "timing material, all {} roles: {all:?}",
            native_theme::icons::IconRole::ALL.len()
        );
    }

    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    #[ignore = "timing probe; run with --ignored --nocapture"]
    fn freedesktop_icons() {
        for theme in ["breeze", "breeze-dark", "Adwaita"] {
            let start = Instant::now();
            let cold = native_theme::icons::IconRole::ALL
                .iter()
                .filter_map(|role| {
                    native_theme::icons::FreedesktopLoader::new(*role)
                        .theme(theme)
                        .load()
                })
                .count();
            let cold_time = start.elapsed();
            let warm = median(11, || {
                native_theme::icons::IconRole::ALL
                    .iter()
                    .filter_map(|role| {
                        native_theme::icons::FreedesktopLoader::new(*role)
                            .theme(theme)
                            .load()
                    })
                    .count()
            });
            println!("timing freedesktop {theme}: {cold} found, cold {cold_time:?}, warm {warm:?}");
        }
    }

    /// A name a theme has only as `.xpm`, which freedesktop-icons never returns: each load is a miss.
    /// It needs a theme the process sees from its first lookup, so run it alone, with the data home
    /// it fills: `XDG_DATA_HOME=<CARGO_TARGET_TMPDIR>/xpm-probe cargo test ... -- --ignored --nocapture
    /// --exact icons::freedesktop_xpm_only_name` (the probe prints the path when it is not set).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    #[ignore = "timing probe; run with --ignored --nocapture"]
    fn freedesktop_xpm_only_name() {
        let data_home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("xpm-probe");
        if std::env::var_os("XDG_DATA_HOME").as_deref() != Some(data_home.as_os_str()) {
            println!(
                "timing freedesktop xpm-only name: skipped, run with XDG_DATA_HOME={}",
                data_home.display()
            );
            return;
        }
        let theme = data_home.join("icons/nt-xpm-probe");
        std::fs::create_dir_all(theme.join("actions")).unwrap();
        std::fs::write(
            theme.join("index.theme"),
            "[Icon Theme]\nName=nt-xpm-probe\nDirectories=actions\n\n[actions]\nSize=24\nType=Fixed\n",
        )
        .unwrap();
        std::fs::write(theme.join("actions/nt-xpm-only.xpm"), "/* XPM */\n").unwrap();
        let load = || {
            native_theme::icons::FreedesktopLoader::new("nt-xpm-only")
                .theme("nt-xpm-probe")
                .load()
        };
        let start = Instant::now();
        let found = load().is_some();
        let cold = start.elapsed();
        let warm = median(21, load);
        println!("timing freedesktop xpm-only name: found {found}, cold {cold:?}, warm {warm:?}");
    }
}

/// This probe's test, which a pass in a process of its own runs again.
const TEST: &str = "bundled_preset_costs";

/// How often the sampler reads the runnable count (*The load budget*, rule 2).
const SAMPLE_EVERY: Duration = Duration::from_millis(100);

/// The runnable count's samples, each its instant and the count (`None`:
/// `/proc/loadavg`'s fourth field could not be read).
#[derive(Default)]
struct Samples {
    taken: Vec<(Instant, Option<u32>)>,
}

/// The runnable count, sampled every 100 ms in a thread of its own while a
/// sweep runs (*The load budget*, rule 2).
struct Sampler {
    samples: Arc<(Mutex<Samples>, Condvar)>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

/// The first number of `/proc/loadavg`'s fourth field: the threads runnable
/// at that instant; `None` where it cannot be read, off Linux among them.
fn runnable() -> Option<u32> {
    let text = std::fs::read_to_string("/proc/loadavg").ok()?;
    text.split_whitespace()
        .nth(3)?
        .split('/')
        .next()?
        .parse()
        .ok()
}

impl Sampler {
    /// Starts sampling, and waits for the first sample.
    fn start() -> Sampler {
        let samples = Arc::new((Mutex::new(Samples::default()), Condvar::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread = {
            let samples = Arc::clone(&samples);
            let stop = Arc::clone(&stop);
            std::thread::spawn(move || {
                loop {
                    let sample = (Instant::now(), runnable());
                    let (taken, added) = &*samples;
                    taken
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .taken
                        .push(sample);
                    added.notify_all();
                    if stop.load(Ordering::Relaxed) {
                        break;
                    }
                    std::thread::sleep(SAMPLE_EVERY);
                }
            })
        };
        let sampler = Sampler {
            samples,
            stop,
            thread: Some(thread),
        };
        drop(sampler.wait_after(Instant::now()));
        sampler
    }

    /// Waits for a sample taken at `at` or after it, and returns the samples.
    fn wait_after(&self, at: Instant) -> std::sync::MutexGuard<'_, Samples> {
        let (taken, added) = &*self.samples;
        let guard = taken.lock().unwrap_or_else(PoisonError::into_inner);
        added
            .wait_while(guard, |samples| {
                samples.taken.last().is_none_or(|(when, _)| *when < at)
            })
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// The mean runnable count of a pass from `start` to `end`: of the
    /// samples from the last one before the pass to the first one after it
    /// (rule 2), and their number; the mean `None` where one could not be
    /// read.
    fn mean(&self, start: Instant, end: Instant) -> (Option<f64>, usize) {
        let samples = self.wait_after(end);
        let first = samples
            .taken
            .iter()
            .rposition(|(when, _)| *when <= start)
            .unwrap_or(0);
        let last = samples
            .taken
            .iter()
            .position(|(when, _)| *when >= end)
            .unwrap_or(samples.taken.len().saturating_sub(1));
        let over = samples.taken.get(first..=last).unwrap_or(&[]);
        let counts: Option<Vec<u32>> = over.iter().map(|(_, count)| *count).collect();
        let mean = counts.filter(|counts| !counts.is_empty()).map(|counts| {
            counts.iter().map(|&count| f64::from(count)).sum::<f64>() / counts.len() as f64
        });
        (mean, over.len())
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// A pass's figure: the median of its samples, in nanoseconds, and their
/// count.
type Figure = (f64, usize);

fn median_ns(mut times: Vec<Duration>, per: u32) -> Figure {
    times.sort();
    let middle = times.get(times.len() / 2).copied().unwrap_or_default();
    (middle.as_secs_f64() * 1e9 / f64::from(per), times.len())
}

/// The median of `sample`'s times after one warm-up sample: 11 samples
/// where the warm-up took under 100 ms, 5 under a second, else 3.
fn median_of(mut sample: impl FnMut() -> Res<Duration>) -> Res<Figure> {
    let runs = match sample()? {
        warm if warm < Duration::from_millis(100) => 11,
        warm if warm < Duration::from_secs(1) => 5,
        _ => 3,
    };
    let times = (0..runs).map(|_| sample()).collect::<Res<Vec<_>>>()?;
    Ok(median_ns(times, 1))
}

/// The time of one call of `call`: under 1 µs a call (100 warm-up calls
/// tell), the median of 201 samples of 100 calls each, divided by 100
/// (rule 2); else [`median_of`] its calls.
fn median_calls<T>(mut call: impl FnMut() -> T) -> Res<Figure> {
    let mut batch = || {
        let start = Instant::now();
        for _ in 0..100 {
            std::hint::black_box(call());
        }
        start.elapsed()
    };
    if batch() < Duration::from_micros(100) {
        return Ok(median_ns((0..201).map(|_| batch()).collect(), 100));
    }
    median_of(|| {
        let start = Instant::now();
        std::hint::black_box(call());
        Ok(start.elapsed())
    })
}

/// Times `change`, then undoes it with `undo`, untimed.
fn timed_change(
    theme: &mut Theme,
    change: impl Fn(&mut Theme) -> Res<()>,
    undo: impl Fn(&mut Theme) -> Res<()>,
) -> Res<Figure> {
    median_of(|| {
        let start = Instant::now();
        change(theme)?;
        let time = start.elapsed();
        undo(theme)?;
        Ok(time)
    })
}

/// A connector's measurement in a probe: every metric zero.
struct NoMetrics;

impl FontMetrics for NoMetrics {
    fn char_width(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn max_char_width(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn x_height(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn height(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn line_spacing(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn underline_offset(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn underline_thickness(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn strikeout_offset(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn strikeout_thickness(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
}

/// The window a query is asked in: 800 × 600 on a 1920 × 1080 screen, left
/// to right (the probe's own fixture).
fn root() -> RootInputs {
    RootInputs::new(
        Size {
            width: Px(800.0),
            height: Px(600.0),
        },
        Size {
            width: Px(1920.0),
            height: Px(1080.0),
        },
        Direction::Ltr,
    )
}

/// A synthetic accent colour, the probe's own fixture (line 4's
/// `set_accent`): no platform's.
const SYNTHETIC_ACCENT: &str = "#7f3fbf";

/// The accent line 4's `set_accent` sets on the preset `file` states: the
/// synthetic colour at both schemes for the accent and for each other
/// accent input the preset declares (`[declare] accent_inputs`).
fn synthetic_accent(file: &PresetFile) -> Res<Accent> {
    let color = Color::parse(SYNTHETIC_ACCENT).ok_or("the synthetic accent is no colour")?;
    let value = || InputValue::new(SchemeMap::new(Value::Stated(color), Value::Stated(color)));
    let mut accent = Accent::new(value());
    for name in file.declared("accent_inputs")? {
        if name != "accent" {
            accent.derived.push((InputName::new(&name)?, value()));
        }
    }
    Ok(accent)
}

/// The first two flavours the preset `file` states declares.
fn two_flavors(file: &PresetFile) -> Res<(Key, Key)> {
    match file.declared("flavors")?.as_slice() {
        [first, second, ..] => Ok((Key::new(first)?, Key::new(second)?)),
        _ => Err("fewer than two flavours declared".into()),
    }
}

/// The button's body in `scope`, at rest or hovered.
fn element(body: Body, scope: &Scope) -> Element<'_> {
    let part = PartRef::Typed(TypedPart::Button(ButtonPart::Body));
    let element = Element::new(part, scope, &NoMetrics);
    match body {
        Body::Rest => element,
        Body::Hover => element.with_states(StateFlags::new().with(StateFlag::Hover), &[]),
    }
}

/// The theme a sweep measures one preset's cases on, kept while the sweep's
/// cases stay on that preset and route.
struct Subject {
    id: PresetId,
    route: Route,
    file: PresetFile,
    theme: Theme,
    /// A second live theme of the preset, for line 4's `set_preset`.
    holder: Option<Theme>,
}

impl Subject {
    fn new(id: PresetId, route: Route) -> Res<Subject> {
        let file = PresetFile::read(id)?;
        let own = file.own()?;
        let theme = cost_common::theme(id, route, &file, &own)?;
        Ok(Subject {
            id,
            route,
            file,
            theme,
            holder: None,
        })
    }

    fn measure(&mut self, measure: Measure) -> Res<Figure> {
        let conditions = WindowConditions::new(InputKind::Mouse);
        let theme = &mut self.theme;
        match measure {
            Measure::Change(_, Change::Pick) => timed_change(
                theme,
                |theme| Ok(theme.pick(Some(Scheme::Dark))?),
                |theme| Ok(theme.pick(None)?),
            ),
            Measure::Change(_, Change::Contrast) => timed_change(
                theme,
                |theme| Ok(theme.set_contrast(Some(Contrast::High))?),
                |theme| Ok(theme.set_contrast(None)?),
            ),
            Measure::Change(_, Change::Accent) => {
                let accent = synthetic_accent(&self.file)?;
                timed_change(
                    theme,
                    |theme| Ok(theme.set_accent(Some(accent.clone()))?),
                    |theme| Ok(theme.set_accent(None)?),
                )
            }
            Measure::Change(_, Change::Flavor) => {
                let (first, second) = two_flavors(&self.file)?;
                timed_change(
                    theme,
                    |theme| Ok(theme.set_flavor(second.clone())?),
                    |theme| Ok(theme.set_flavor(first.clone())?),
                )
            }
            Measure::Change(_, Change::SetPreset) => {
                if self.holder.is_none() {
                    self.holder = Some(Theme::from_preset(self.id, SCHEME)?);
                }
                let id = self.id;
                timed_change(theme, |theme| Ok(theme.set_preset(id)?), |_| Ok(()))
            }
            // A resolve's time includes dropping the resolution it returns,
            // as the probe's D6 measurement (cost-probe d7adb68c) did (lines 5
            // and 6).
            Measure::ResolveChanged(_) => {
                median_calls(|| theme.resolve(&mut WindowState::default(), &conditions))
            }
            Measure::ResolveUnchanged(_) => {
                let mut window = WindowState::default();
                let _first = theme.resolve(&mut window, &conditions);
                median_calls(|| theme.resolve(&mut window, &conditions))
            }
            Measure::QueryComputed(_, body) => median_of(|| {
                let resolved = theme
                    .resolve(&mut WindowState::default(), &conditions)
                    .theme;
                let scope = resolved.root_scope(&root());
                let element = element(body, &scope);
                let start = Instant::now();
                std::hint::black_box(resolved.style(&element));
                Ok(start.elapsed())
            }),
            Measure::QueryRepeated(_, body) => {
                let resolved = theme
                    .resolve(&mut WindowState::default(), &conditions)
                    .theme;
                let scope = resolved.root_scope(&root());
                let element = element(body, &scope);
                std::hint::black_box(resolved.style(&element));
                median_calls(|| resolved.style(&element))
            }
            Measure::AddLayer(_, at) => {
                let (_, text) = REFERENCE_LAYERS.get(at).ok_or("no such reference layer")?;
                let layer = Layer::from_toml(text)?;
                median_of(|| {
                    let layer = layer.clone();
                    let start = Instant::now();
                    let id = theme.add_layer(layer)?;
                    let time = start.elapsed();
                    theme.remove_layer(id)?;
                    Ok(time)
                })
            }
            other => Err(format!("{other:?} is no time this binary measures on a theme").into()),
        }
    }
}

/// The route of the theme `measure` is taken on, `None` for a load.
fn theme_route(measure: Measure) -> Option<Route> {
    match measure {
        Measure::Change(route, _)
        | Measure::ResolveChanged(route)
        | Measure::ResolveUnchanged(route)
        | Measure::QueryComputed(route, _)
        | Measure::QueryRepeated(route, _)
        | Measure::AddLayer(route, _) => Some(route),
        _ => None,
    }
}

fn print_pass(name: &str, (ns, samples): Figure, (load, load_samples): (Option<f64>, usize)) {
    let load = load.map_or_else(|| "absent".to_owned(), |load| format!("{load:.2}"));
    println!(
        "cost-pass {name} median_ns={ns:.1} samples={samples} load={load} load_samples={load_samples}"
    );
}

/// Line 1's pass, in a process of its own: the case `1.<preset>.<route>`'s
/// first load in the process, the read alone (`read`) or
/// `Theme::from_preset` whole (`plain`). Its name is parsed, not looked up
/// among the cases, so that no preset file is read before the load.
fn first_load(name: &str) -> Res<()> {
    let (key, route) = name
        .strip_prefix("1.")
        .and_then(|rest| rest.rsplit_once('.'))
        .ok_or_else(|| format!("{name} is no case of line 1"))?;
    let id = PresetId::from_key(key).ok_or_else(|| format!("no preset {key}"))?;
    let sampler = Sampler::start();
    let start = Instant::now();
    let time = match route {
        "read" => native_theme::gate::read_cost(id, None)?.time,
        "plain" => {
            let theme = Theme::from_preset(id, SCHEME)?;
            let time = start.elapsed();
            drop(theme);
            time
        }
        other => return Err(format!("{name}: no route {other} for line 1").into()),
    };
    let end = Instant::now();
    print_pass(name, median_ns(vec![time], 1), sampler.mean(start, end));
    Ok(())
}

/// One sweep over the cases `names` asks for, in its order: each case's
/// pass, its figure beside the mean runnable count over it, then the
/// sweep's duration. Line 1's passes each run in a process of their own.
fn sweep(names: &str) -> Res<()> {
    let all: Vec<Case> = cost_common::cases()?
        .into_iter()
        .filter(|case| case.kind() == cost_common::Kind::Time)
        .collect();
    let (cases, mut failed) = cost_common::asked(names, &all);
    let began = Instant::now();
    let sampler = Sampler::start();
    let mut subject: Option<Subject> = None;
    for case in &cases {
        let result = match (case.measure, theme_route(case.measure)) {
            (Measure::Load(_), _) => cost_common::run_child(TEST, &case.name, &[&case.name]),
            (measure, Some(route)) => (|| -> Res<()> {
                let held = subject
                    .as_ref()
                    .is_some_and(|held| held.id == case.preset && held.route == route);
                if !held {
                    subject = None;
                    subject = Some(Subject::new(case.preset, route)?);
                }
                let subject = subject.as_mut().ok_or("no theme")?;
                let start = Instant::now();
                let figure = subject.measure(measure)?;
                print_pass(&case.name, figure, sampler.mean(start, Instant::now()));
                Ok(())
            })(),
            (measure, None) => Err(format!("{measure:?}: not a time").into()),
        };
        if let Err(error) = result {
            failed = failed.saturating_add(1);
            println!("cost-error {} {error}", case.name);
        }
    }
    drop(subject);
    println!(
        "cost-sweep seconds={:.3} cases={}",
        began.elapsed().as_secs_f64(),
        cases.len()
    );
    if failed > 0 {
        return Err(format!("{failed} cases not measured").into());
    }
    Ok(())
}

/// The load budget's times (*The load budget*, lines 1, 4 to 8 and 12), in
/// a release build, for the budget step, which compares them; nothing is
/// asserted here.
///
/// `NT_COST_CASES=<case>,<case>,…` runs one sweep over those cases, by
/// name, in that order, and prints for each its pass (`cost-pass <case>
/// median_ns=… samples=… load=… load_samples=…`: the median of the pass's
/// samples, and the mean runnable count over the pass with the number of
/// samples it took), or `cost-error <case> <why>`, then `cost-sweep
/// seconds=…`. Without it, it lists every time case it can measure
/// (`cost-case <case> line=… kind=time`). Line 1 runs each pass in a
/// process of its own (`NT_COST_CHILD`).
///
/// `NT_COST_CASES=1.adwaita.read,5.adwaita.synthetic <the timings binary>
/// --ignored --nocapture --test-threads=1 --exact bundled_preset_costs`
#[test]
#[ignore = "the load budget's probe; the budget step runs it"]
fn bundled_preset_costs() -> Res<()> {
    if let Ok(name) = std::env::var("NT_COST_CHILD") {
        return first_load(&name);
    }
    match std::env::var("NT_COST_CASES") {
        Ok(names) => sweep(&names),
        Err(_) => {
            for case in cost_common::cases()? {
                if case.kind() == cost_common::Kind::Time {
                    println!("{}", case.listed());
                }
            }
            Ok(())
        }
    }
}
