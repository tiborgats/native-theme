# Whole-codebase review — finding ledger

The plan is `docs/todo_code-review-plan.md`; entry format, lenses, severities and triage
rules are there (*The finding ledger*). Branch `v0.6.2-rc1`, from `main` at `92893ce1`.

## Status

| M | Status |
|---|--------|
| M0 | done (2026-10-11) |
| M1 | not started |
| M2a–M2e | not started |
| M3–M9 | not started |

## Baseline (M0, 2026-10-11)

### Gates

`./scripts/check_release.sh` on `v0.6.2-rc1` at `10e037b1` plus the uncommitted timing test:
84 checks, 82 ✅, 1 ⚠️ (the tag `v0.6.1` already exists: the version is not bumped yet), 1 ❌
(visual assets stale since `92893ce1`'s GIF change, cleared by the next
`generate_assets_release.sh`); 37 min 22 s. A first run stopped at clippy on the timing
test's `IconRole` import, unused without the icon features; the import moved into the
feature-gated tests (the plan's copy too).

### Cross-target clippy

`cargo clippy -p native-theme --all-targets -- -D warnings` with `--all-features` and with
`--no-default-features`, for `x86_64-apple-darwin` and `x86_64-pc-windows-msvc`: 8 errors on
macOS and 11 on Windows, all fixed in M0:

- `needless_return` (6 sites): `detect.rs` (dark-mode and reduced-motion readers),
  `model/icons.rs` (`detect_icon_theme_outcome`), `pipeline.rs` (`platform_preset_name`),
  `presets.rs` (`detect_platform`), `watch/mod.rs` (both platform arms); the `#[cfg]` blocks
  are one per OS, so each final `return x;` became the block's value. Five
  `#[allow(unreachable_code)]` went with them; the one on `detect_system_font_dpi` stays (its
  blocks are not exclusive: removing it fails on both targets);
- `needless_borrow` (2): `Context::from_waker(&waker)` in both non-Linux `from_system`s,
  where `Waker::noop()` is already a `&Waker`;
- `field_reassign_with_default` (3, Windows): `cbSize` set in the initialiser of
  `NONCLIENTMETRICSW`, `HIGHCONTRASTW` and `SHSTOCKICONINFO`; no `unsafe` line changed;
- one test import (`icons.rs`, `load_system_icon_by_name_tests`) unused on macOS without
  features: explicit `super::` paths instead of `use super::*`.

After: all six runs clean; native-theme's 1104 tests pass on Linux. `check_release.sh`'s
cross-target section now runs the four clippy runs (both targets, both feature sets).

### Timings

`cargo test -p native-theme --release --all-features --test timings -- --ignored --nocapture --test-threads=1`,
three runs on the maintainer's machine (KDE Plasma session), each value a median (21 runs
per preset, 5 for the readers, 11 for icons); release profile.

| Path | Run 1 | Run 2 | Run 3 |
|------|-------|-------|-------|
| `Theme::preset` parse, every preset | 1.02–1.38 µs | 1.03–1.18 µs | 1.02–1.38 µs |
| `Theme::resolve`, per preset and mode | 1.30–1.50 ms; 3.36 ms ×1 | 1.30–1.37 ms; 3.37–3.49 ms ×12 | 1.30–1.46 ms; 3.36–3.42 ms ×6, 5.18 ms ×1 |
| `SystemTheme::from_system` cold / warm | 9.12 / 3.63 ms | 9.66 / 8.30 ms | 9.41 / 5.64 ms |
| `AccessibilityPreferences::from_system` | 11.57 ms | 12.15 ms | 8.35 ms |
| Material, all 42 roles | 431 ns | 431 ns | 431 ns |
| freedesktop `breeze` cold / warm | 39.2 / 1.76 ms | 40.7 / 1.77 ms | 40.2 / 1.86 ms |
| freedesktop `breeze-dark` cold / warm | 37.6–40.9 / 1.82–2.07 ms (three runs) | | |
| freedesktop `Adwaita` cold / warm | 5.0–7.7 / 2.58–2.92 ms (three runs) | | |

(The `from_system` and accessibility rows are from the last three of six runs; the first
three gave warm `from_system` 5.55, 5.61, 24.34 ms and accessibility 7.59, 9.42, 11.86 ms.)

Noise floor: the resting resolve times agree within about 0.2 ms; the readers vary by
about 2×. Observations for M3/M4, not yet findings:

- `resolve` is about 1.3 ms for any preset, against about 1 µs to parse one, and jumps
  by about 2 ms (to 3.4, once 5.2): a fixed-step wait inside resolution is the suspect
  (`ResolutionContext::from_system`, `model/mod.rs:461`, runs on every call) [M3 R2];
- 1 µs is too fast to parse a 500-line TOML, so `Theme::preset` presumably returns a
  cached copy: confirm it is a cache, and what it costs to clone [M3 R4];
- a cold freedesktop lookup took 375 ms on 2026-10-10 (cold disk cache) and 38–41 ms
  here (warm page cache); warm lookups take 1.8–2.9 ms for 42 icons [M4 R4].

### Lint inventory

`cargo clippy --workspace --all-features --all-targets --message-format=json -- -W clippy::pedantic -W clippy::nursery`:
3075 hits (1495 in library code, 1462 in the showcases, 118 in tests). Per crate:
native-theme-gpui 1117, native-theme-egui 728, native-theme 614, native-theme-iced 375,
native-theme-build 112, native-theme-derive 65, native-theme-egui-widgets 61 (and 3 in
generated or third-party files). The most frequent in library code: `doc_markdown` 344,
`missing_const_for_fn` 195, `redundant_pub_crate` 140, `use_self` 90,
`too_long_first_doc_paragraph` 68, `suboptimal_flops` 56, `match_same_arms` 55,
`uninlined_format_args` 52, `option_if_let_else` 48, `too_many_lines` 39. Worth a
reviewer's look rather than a blanket fix: `redundant_clone` 15 (13 in native-theme),
`needless_pass_by_ref_mut` 14 (gpui), the `cast_*` lints 88, `float_cmp` 12 (9 in iced),
`unnecessary_wraps` 8. Each milestone regenerates the list for its own crate (plan,
template Step 1).

### Dependencies

- `cargo machete` (0.9.2): no unused dependency in any workspace crate.
- `cargo tree -d -e normal --all-features`: duplicated versions per crate, counting a
  crate once however many versions it has: native-theme 7, native-theme-iced 10,
  native-theme-egui 10, native-theme-gpui 32. In native-theme the real ones are upstream's:
  `miniz_oxide` 0.8.9 (png) / 0.9.1 (flate2 under png), `roxmltree` 0.20.0 (fontdb's
  fontconfig-parser) / 0.21.1 (usvg), `syn` 2 / 3 (proc macros); the `serde`, `serde_core`
  and `toml_datetime` lines are one version built with two feature sets. None is ours to fix.

### Semver

`cargo semver-checks` 0.51.0, `-p native-theme --baseline-version 0.6.1 --all-features`:
202 checks pass, 58 skipped; no semver update required. The other crates are checked when
their milestone closes.

## Calibration

Measured cost per milestone, and the estimate for the rest.

## M1 — Design review

## M2a — macOS

## M2b — Windows

## M2c — iOS

## M2d — Material 3 and Material 3 Expressive

## M2e — KDE and GNOME currency

## M3 — Core, part 1

## M4 — Core, part 2

## M5 — iced connector

## M6 — gpui connector

## M7 — egui connector and egui widgets

## M8 — Cross-cutting

## M9 — Showcases
