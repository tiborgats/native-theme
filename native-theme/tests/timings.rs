//! Wall-clock timings of the paths the code review measures: icon loading.
//! Ignored by default; run on the release profile, before and after a perf fix:
//!
//! `cargo test -p native-theme --release --all-features --test timings -- --ignored --nocapture --test-threads=1`

// Every probe loads icons, `system-icons` enabling `material-icons`.
#![cfg(feature = "material-icons")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

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
