//! Wall-clock timings of the paths the code review measures: preset parsing, resolution,
//! the system reader, the accessibility reader and its cache, and icon loading. Ignored by default; run on
//! the release profile, before and after a perf fix:
//!
//! `cargo test -p native-theme --release --all-features --test timings -- --ignored --nocapture --test-threads=1`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

use std::time::{Duration, Instant};

use native_theme::ResolutionContext;
use native_theme::theme::{ColorMode, Theme};

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
fn presets_parse_and_resolve() {
    for info in Theme::list_presets() {
        let key = info.key;
        let parse = median(21, || Theme::preset(key));
        let theme = Theme::preset(key).expect("bundled preset parses");
        let light = median(21, || theme.resolve(ColorMode::Light));
        let dark = median(21, || theme.resolve(ColorMode::Dark));
        println!("timing preset {key}: parse {parse:?}, resolve light {light:?}, dark {dark:?}");
        let ctx = ResolutionContext::for_tests();
        let light = median(21, || theme.resolve_with(ColorMode::Light, &ctx));
        let dark = median(21, || theme.resolve_with(ColorMode::Dark, &ctx));
        println!("timing preset {key}: resolve_with for_tests() light {light:?}, dark {dark:?}");
    }
}

#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn system_readers() {
    let start = Instant::now();
    let first = native_theme::SystemTheme::from_system();
    println!(
        "timing from_system cold: {:?} (ok: {})",
        start.elapsed(),
        first.is_ok()
    );
    let warm = median(5, native_theme::SystemTheme::from_system);
    println!("timing from_system warm: {warm:?}");
    let prefs = median(5, native_theme::AccessibilityPreferences::from_system);
    println!("timing accessibility: {prefs:?}");
    native_theme::detect::invalidate_caches();
    let _ = native_theme::SystemTheme::from_system();
    let start = Instant::now();
    let _ = native_theme::detect::accessibility_preferences();
    let first = start.elapsed();
    let cached = median(5, native_theme::detect::accessibility_preferences);
    println!("timing accessibility cached, after from_system: first {first:?}, then {cached:?}");
}

#[cfg(feature = "material-icons")]
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn bundled_icons() {
    let all = median(11, || {
        native_theme::theme::IconRole::ALL
            .iter()
            .filter_map(|role| native_theme::icons::MaterialLoader::new(*role).load())
            .count()
    });
    println!(
        "timing material, all {} roles: {all:?}",
        native_theme::theme::IconRole::ALL.len()
    );
}

#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn freedesktop_icons() {
    for theme in ["breeze", "breeze-dark", "Adwaita"] {
        let start = Instant::now();
        let cold = native_theme::theme::IconRole::ALL
            .iter()
            .filter_map(|role| {
                native_theme::icons::FreedesktopLoader::new(*role)
                    .theme(theme)
                    .load()
            })
            .count();
        let cold_time = start.elapsed();
        let warm = median(11, || {
            native_theme::theme::IconRole::ALL
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
