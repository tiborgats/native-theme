//! Wall-clock timings of the connector's theme switch, `from_preset(key, is_dark)`, for every
//! bundled preset in both modes. Ignored by default; run on the release profile, before and
//! after a perf fix:
//!
//! `cargo test -p native-theme-iced --release --all-features --test timings -- --ignored --nocapture --test-threads=1`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

use std::time::{Duration, Instant};

use native_theme::theme::Theme;

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
    for info in Theme::list_presets() {
        let key = info.key;
        let start = Instant::now();
        let first = native_theme_iced::from_preset(key, false);
        let cold = start.elapsed();
        let light = median(21, || native_theme_iced::from_preset(key, false));
        let dark = median(21, || native_theme_iced::from_preset(key, true));
        println!(
            "timing from_preset {key}: first {cold:?} (ok: {}), light {light:?}, dark {dark:?}",
            first.is_ok()
        );
    }
}
