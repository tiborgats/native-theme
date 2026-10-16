//! Wall-clock timings of the connector's theme switch, `from_preset(key, is_dark, prefs)`, and
//! of its conversion step alone, `to_theme`, for every bundled preset in both modes, and of the
//! icon conversions that hand gpui a decoded image. Ignored by default; run on the release
//! profile, before and after a perf fix:
//!
//! `cargo test -p native-theme-gpui --release --test timings -- --ignored --nocapture --test-threads=1`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

use std::time::{Duration, Instant};

use native_theme::icons::{IconData, load_icon};
use native_theme::theme::{ColorMode, IconSet, Theme};
use native_theme_gpui::AccessibilityPreferences;
use native_theme_gpui::icons::{into_image_source, to_image_source};

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
fn from_preset_every_bundled_preset() {
    let prefs = AccessibilityPreferences::default();
    for info in Theme::list_presets() {
        let key = info.key;
        let start = Instant::now();
        let first = native_theme_gpui::from_preset(key, false, &prefs);
        let cold = start.elapsed();
        let light = median(21, || native_theme_gpui::from_preset(key, false, &prefs));
        let dark = median(21, || native_theme_gpui::from_preset(key, true, &prefs));
        println!(
            "timing from_preset {key}: first {cold:?} (ok: {}), light {light:?}, dark {dark:?}",
            first.is_ok()
        );
    }
}

#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn to_theme_every_bundled_preset() {
    let prefs = AccessibilityPreferences::default();
    for info in Theme::list_presets() {
        let key = info.key;
        let spec = Theme::preset(key).unwrap();
        let [light, dark] = [ColorMode::Light, ColorMode::Dark].map(|mode| {
            let resolved = spec.resolve(mode).unwrap();
            let is_dark = resolved.mode.is_dark();
            median(21, || {
                native_theme_gpui::to_theme(&resolved.variant, &spec.name, is_dark, &prefs)
            })
        });
        println!("timing to_theme {key}: light {light:?}, dark {dark:?}");
    }
}

/// An owned `IconData::Rgba` of `side` × `side` pixels handed to `into_image_source`, each run
/// with a buffer of its own built before the clock starts.
fn into_image_source_rgba(side: u32) -> Duration {
    const RUNS: usize = 21;
    let len = (side * side * 4) as usize;
    let pixels: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
    let icon = IconData::rgba(side, side, pixels).unwrap();
    let mut inputs: Vec<IconData> = (0..RUNS).map(|_| icon.clone()).collect();
    median(RUNS, || {
        into_image_source(inputs.pop().unwrap(), None, None).unwrap()
    })
}

#[test]
#[ignore = "timing probe; run with --ignored --nocapture"]
fn icon_conversions() {
    for side in [512, 48] {
        let time = into_image_source_rgba(side);
        println!("timing into_image_source rgba {side}x{side}: {time:?}");
    }
    let svg = load_icon("search", IconSet::Lucide).unwrap();
    let time = median(21, || to_image_source(&svg, None, Some(48)).unwrap());
    println!("timing to_image_source svg 48: {time:?}");
}
