//! Wall-clock timings of the paths the code review measures: icon loading,
//! and what a bundled preset costs an application at run time.
//! Ignored by default; run on the release profile, before and after a perf fix:
//!
//! `cargo test -p native-theme --release --all-features --test timings -- --ignored --nocapture --test-threads=1`

// Every probe loads icons, `system-icons` enabling `material-icons`.
#![cfg(feature = "material-icons")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

use std::time::{Duration, Instant};

use native_theme::{
    ButtonPart, Direction, Element, FontMetrics, InputKind, Layer, PartRef, PresetId, Px,
    ResolvedFont, RootInputs, Scheme, Size, StateFlag, StateFlags, Styled, Theme, TypedPart,
    WindowConditions, WindowState,
};

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
/// --exact freedesktop_xpm_only_name` (the probe prints the path when it is not set).
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

/// The median of `f`'s runs after one warm-up run, and their count: 11
/// where the warm-up took under 100 ms, 5 under a second, else 3.
fn median_scaled<T>(mut f: impl FnMut() -> T) -> (Duration, usize) {
    let start = Instant::now();
    std::hint::black_box(f());
    let runs = match start.elapsed() {
        warm if warm < Duration::from_millis(100) => 11,
        warm if warm < Duration::from_secs(1) => 5,
        _ => 3,
    };
    (median(runs, f), runs)
}

fn ms(time: Duration) -> f64 {
    time.as_secs_f64() * 1e3
}

/// The process's peak resident set (`VmHWM`), in MiB; `None` off Linux.
fn peak_mib() -> Option<f64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let kib: f64 = status
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))?
        .trim()
        .strip_suffix("kB")?
        .trim()
        .parse()
        .ok()?;
    Some(kib / 1024.0)
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

/// A bundled preset's file as the synthetic route needs it from outside
/// the crate: its `[pending]` table, its own layer (the file without its
/// `[preset]`, `[declare]` and `[pending]` tables, read as a layer: the
/// bases and rules `synthetic_layers` looks a keyed entry up in), the
/// schemes it declares (both where it declares none) and its size.
struct PresetFile {
    pending: Vec<(String, Vec<String>)>,
    own: Layer,
    schemes: Vec<Scheme>,
    bytes: u64,
}

fn preset_file(id: PresetId) -> PresetFile {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/presets")
        .join(format!("{}.toml", id.key()));
    let text = std::fs::read_to_string(&path).unwrap();
    let mut table: toml::Table = text.parse().unwrap();
    let pending = table
        .get("pending")
        .and_then(toml::Value::as_table)
        .map(|entries| {
            entries
                .iter()
                .map(|(path, ids)| {
                    let ids = match ids {
                        toml::Value::String(id) => vec![id.clone()],
                        toml::Value::Array(ids) => ids
                            .iter()
                            .map(|id| id.as_str().unwrap().to_owned())
                            .collect(),
                        other => panic!("{}: pending {path} = {other}", id.key()),
                    };
                    (path.clone(), ids)
                })
                .collect()
        })
        .unwrap_or_default();
    let declared: Vec<Scheme> = table
        .get("declare")
        .and_then(|declare| declare.get("schemes"))
        .and_then(toml::Value::as_array)
        .map(|names| {
            names
                .iter()
                .map(|name| match name.as_str() {
                    Some("light") => Scheme::Light,
                    Some("dark") => Scheme::Dark,
                    other => panic!("{}: scheme {other:?}", id.key()),
                })
                .collect()
        })
        .unwrap_or_default();
    for header in ["preset", "declare", "pending"] {
        table.remove(header);
    }
    let own = Layer::from_toml(&toml::to_string(&table).unwrap())
        .unwrap_or_else(|e| panic!("{}: {e}", id.key()));
    PresetFile {
        pending,
        own,
        schemes: if declared.is_empty() {
            vec![Scheme::Light, Scheme::Dark]
        } else {
            declared
        },
        bytes: std::fs::metadata(&path).unwrap().len(),
    }
}

/// One preset's costs per scheme it declares, one `cost` line each, through
/// the synthetic route (`Theme::from_preset_synthetic` with
/// `native_theme::gate::synthetic_layers`), as no bundled preset loads
/// through `Theme::from_preset` while its `[pending]` table lists paths:
/// - `load_ms`: the synthetic layers and `Theme::from_preset_synthetic`
///   (the read of the bundled text, the layers added, validation), of which
///   `syn_ms` the synthetic layers alone; the preset's own layer they look
///   entries up in is read before, untimed;
/// - `refused_ms`: the plain `Theme::from_preset` up to its refusal, the
///   parse and validation an application pays today (`plain=built` where
///   it builds the theme instead);
/// - `resolve_ms`: `Theme::resolve` for one new window under mouse input;
/// - `query_rest_us`, `query_hover_us`: one `ResolvedTheme::style` of the
///   button's body at rest and hovered, the median of 201 samples of 100
///   queries each, divided by 100 (`rest=`, `hover=`: whether each answered
///   a style);
/// - `bytes`: the preset file's size, which `include_str!` puts in a binary.
///
/// Each time is the median of the runs `runs_*` counts, after a warm-up run.
fn preset_costs(id: PresetId) {
    let file = preset_file(id);
    let root = RootInputs::new(
        Size {
            width: Px(800.0),
            height: Px(600.0),
        },
        Size {
            width: Px(1920.0),
            height: Px(1080.0),
        },
        Direction::Ltr,
    );
    let conditions = WindowConditions::new(InputKind::Mouse);
    for &scheme in &file.schemes {
        let layers = || native_theme::gate::synthetic_layers(&file.pending, &file.own).unwrap();
        let (syn, _) = median_scaled(layers);
        let load = || Theme::from_preset_synthetic(id, scheme, layers()).unwrap();
        let (load_time, load_runs) = median_scaled(load);
        let plain = if Theme::from_preset(id, scheme).is_ok() {
            "built"
        } else {
            "refused"
        };
        let (refused, refused_runs) = median_scaled(|| Theme::from_preset(id, scheme).is_err());
        let theme = load();
        let (resolve, resolve_runs) =
            median_scaled(|| theme.resolve(&mut WindowState::default(), &conditions));
        let resolved = theme
            .resolve(&mut WindowState::default(), &conditions)
            .theme;
        let scope = resolved.root_scope(&root);
        let body = || PartRef::Typed(TypedPart::Button(ButtonPart::Body));
        let rest = Element::new(body(), &scope, &NoMetrics);
        let hover = Element::new(body(), &scope, &NoMetrics)
            .with_states(StateFlags::new().with(StateFlag::Hover), &[]);
        let answered = |element: &Element<'_>| match resolved.style(element) {
            Styled::Style(_) => "style",
            _ => "none",
        };
        let query = |element: &Element<'_>| {
            median(201, || {
                for _ in 0..100 {
                    std::hint::black_box(resolved.style(element));
                }
            })
            .as_secs_f64()
                * 1e6
                / 100.0
        };
        println!(
            "cost preset={} scheme={} runs_load={load_runs} load_ms={:.3} syn_ms={:.3} \
             runs_refused={refused_runs} refused_ms={:.3} plain={plain} \
             runs_resolve={resolve_runs} resolve_ms={:.3} query_rest_us={:.3} \
             query_hover_us={:.3} rest={} hover={} bytes={}",
            id.key(),
            scheme.name(),
            ms(load_time),
            ms(syn),
            ms(refused),
            ms(resolve),
            query(&rest),
            query(&hover),
            answered(&rest),
            answered(&hover),
            file.bytes,
        );
    }
}

/// What each bundled preset costs an application at run time (D6), in a
/// release build, through the synthetic route ([`preset_costs`]). A peak is
/// per process, so with `NT_COST_PRESET=<key>` it measures that preset
/// alone and prints the process's peak resident set after it (`VmHWM`, in
/// MiB however `peak_mb` names it, `start_mb` the peak before it); unset,
/// every preset and no peak.
/// Run one process per preset for the peaks: `NT_COST_PRESET=adwaita
/// <the timings binary> --ignored --nocapture --test-threads=1 --exact
/// bundled_preset_costs`.
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn bundled_preset_costs() {
    match std::env::var("NT_COST_PRESET") {
        Ok(key) => {
            let id = PresetId::from_key(&key).unwrap_or_else(|| panic!("no preset {key}"));
            let start = peak_mib();
            preset_costs(id);
            println!(
                "cost preset={key} peak_mb={:.1} start_mb={:.1}",
                peak_mib().unwrap_or(f64::NAN),
                start.unwrap_or(f64::NAN)
            );
        }
        Err(_) => PresetId::all().iter().copied().for_each(preset_costs),
    }
}
