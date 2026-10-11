# Whole-codebase review — finding ledger

The plan is `docs/todo_code-review-plan.md`; entry format, lenses, severities and triage
rules are there (*The finding ledger*). Branch `v0.6.2-rc1`, from `main` at `92893ce1`.

## Status

| M | Status |
|---|--------|
| M0 | done (2026-10-11) |
| M1 | decisions done (2026-10-11), fix phase in progress |
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

### M1 (measured 2026-10-11)

The review Workflow, output tokens per agent (the model's own writing and thinking), from
the agents' transcripts; input is almost all prompt-cache reads:

| Stage | Model, effort | Agents | Output tokens each | Cache reads each |
|-------|---------------|-------:|-------------------:|-----------------:|
| Context map | Haiku (`locate`) | 5 | 6k–13k | 74k–419k |
| Design review | Fable 5.1, xhigh | 5 | 45k–73k (D-D, 8 proposals, 73k) | 5.2M–11.2M |
| Critique | Opus 5.5, high | 5 | 22k–26k | 5.5M–7.0M |
| Total | | 15 | 435k output; 9.0M cache writes; 65M cache reads | |

The Workflow reported 1.73M "subagent tokens" for the run (its own accounting, which
counts more than output). The orchestrating session (Fable) read the result (about 60k
tokens of proposals and verdicts) and wrote this section. The maintainer's cap for M1 was
+2000k; the Workflow's budget hook did not pick the cap up (`budget.total` was unset), so
the run was not capped mechanically and stayed under it by its size.

Estimate for the code milestones from these figures: a region reviewer reading 2.5k lines
at Opus `high` is likely 25k–40k output tokens (the critics read about that much code
checking claims), a verifier 15k–25k; a 10-agent review phase about 300k–500k output
tokens, well under the plan's 1.5–2.5M guess, which counted input. The cache-read volume
(5–11M per agent) is the real cost driver and scales with how much of the tree an agent
opens; the Haiku context maps keep it bounded.

## M1 — Design review

**Status:** review, critique and decisions done 2026-10-11 (Steps 1–4); fix plan and fix phase below.

**Decisions (Step 4).** Rounds 1–6 (24 proposals) were put to the maintainer one by one in
AskUserQuestion rounds of four, the critic-amended form first; every answer took that form.
Before round 7 the maintainer wrote: "During this session, I approve all your future
recommendations. Don't ask me until you finished", so round 7 (M1-T10-01…03) took the
recommended form without a question and Steps 5–7 ran without further questions. The two
breaking changes (M1-T4-01, M1-T7-06) were accepted, so under D1 the next release is 0.7.0
and under D0 the branch is renamed `v0.7.0-rc1`.

**How it ran.** One Workflow (`m1-design-review`, run `wf_3d17c6bf-3f8`), three stages
pipelined per reviewer group: a Haiku `locate` agent built a context map (file:line pointers
to the current design, the archived rationale and the `docs/todo.md` entries; 42–73
entries per group, saved in the session scratchpad, not committed), a Fable 5.1 design
reviewer at `xhigh` wrote the proposals (D2), an Opus 5.5 critic at `high`, in fresh
context and seeing only the proposals and the map, argued for the current design, checked
every evidence and cost claim against the tree, and gave the verdict. 15 agents, 523 tool
calls, 19 min 52 s wall clock; no agent failed. Token cost is under *Calibration*.

**Result.** 35 proposals over the ten topics: 2 *adopt*, 25 *adopt with changes*,
8 *keep*. The critics found wrong claims in most proposals (each is marked in the entry's
*Evidence check* and *Cost check*); the corrections that matter beyond their proposal:

- The plan's known-gap sentence (plan :19-24) says the `macos-sonoma` screenshots are
  taken on a macOS 26 runner, citing `screenshots.yml:63`; that line is a comment about
  another repository's run. The workflow uses the floating `macos-latest` label and
  records no runner version anywhere (M1-T9-02). The captures use the static preset
  (`--theme macos-sonoma`), so the reader's version gap does not reach them (M1-T1-01).
- A dark Mac starts light in every connector: `from_system_inner` maps `ReaderOutput::Dual`
  to `ColorMode::Light` unconditionally, and the macOS reader never reads the active
  appearance (M1-T6-01, verdict *adopt*; S1-shaped, verifiable only on the macOS CI leg).
- The four `-live` presets change no value: every key in each `-live` file exists in its
  regular preset with the same value, and the merge order makes the layer a no-op
  (M1-T4-01; D4 names a `macos-tahoe-live` base, so the maintainer must amend D4).
- The reader constants and the live presets differ in three fields the gate does not
  cover (macOS scrollbar groove 15 vs 16, divider 9 vs 6; Windows divider 4 vs 1), all
  already in `docs/todo.md` Table B (M1-T1-02).
- `docs/property-registry.toml` already drifts from the code: `[defaults]` lists four
  accessibility keys that moved to `AccessibilityPreferences` (M1-T3-03).
- `platform-facts.md:1123` gives KDE's text scale as `forceFontDPI / 96`, so text scale
  may already reach resolution through `font_dpi` on KDE, and possibly on GNOME through
  `Xft.dpi`; an M2 question before text scale moves into resolution (M1-T6-03).
- The typed per-set icon loaders replaced a one-builder design on purpose (CHANGELOG
  :330-337, `3132d7f9`: `.set(Freedesktop).theme(..)` silently dropped `.theme()`); a
  union builder would reintroduce that class (M1-T6-06).

Two proposals are breaking under D1 and go to the maintainer as such: M1-T4-01 (remove
the `-live` tier: `Theme::preset("…-live")`, `PlatformPreset::is_live`, `live_name()`) and
M1-T7-06 (one named result struct for the connectors' `from_system`, accessor renames).
Three change rules or documents the maintainer owns (M1-T9-04 the every-commit check
rule, M1-T10-01 deleting closed backlog text, M1-T10-03 the memory index).

### Summary

| ID | Group | Proposal | Reviewer | Critic | API break |
|----|-------|----------|----------|--------|-----------|
| M1-T1-01 | D-A | One generation table picks the preset by OS version and reports the mismatch | change | adopt-with-changes | no |
| M1-T1-02 | D-A | Readers' hardcoded generation geometry moves into the live presets | change | adopt-with-changes | no |
| M1-T1-03 | D-A | Naming rule: one preset per generation, vendor's name, version range in the header | change | adopt-with-changes | no |
| M1-T1-04 | D-A | Pinning an older generation on a newer OS: from_system_with_preset | change | adopt-with-changes | no |
| M1-T2-01 | D-A | README states what ios and material are; ios named by its version after M2c | change | adopt-with-changes | no |
| M1-T3-01 | D-B | Theme::resolve spawns a subprocess per call; expose the pure path | change | adopt-with-changes | no |
| M1-T3-02 | D-B | Theme → ThemeMode → Resolved with Option fields: keep it | keep | keep | no |
| M1-T3-03 | D-B | Generate both TOML registries from the derive's inventory | change | adopt-with-changes | no |
| M1-T4-01 | D-B | The -live tier changes no value: remove it | change | adopt-with-changes | yes |
| M1-T5-01 | D-B | Machine-readable facts for Chapter 2, keyed by platform and OS version | change | adopt-with-changes | no |
| M1-T6-01 | D-C | ReaderOutput::Dual carries no active mode; macOS SystemTheme::mode is Light | change | adopt | no |
| M1-T6-02 | D-C | Sync/async split: keep it; drive both sync entry points with pollster | change | adopt | no |
| M1-T6-03 | D-C | Accessibility flags: text scale belongs in resolution; the others need named receivers | change | adopt-with-changes | yes |
| M1-T6-04 | D-C | Reduced motion has two sources that disagree on KDE | change | adopt-with-changes | no |
| M1-T6-05 | D-C | AccessibilityPreferences::from_system re-runs the whole platform reader | change | adopt-with-changes | no |
| M1-T6-06 | D-C | Icons: keep the five loader types; add one runtime-set builder | change | adopt-with-changes | no |
| M1-T6-07 | D-C | System icons on macOS and Windows: fixed-size, fixed-colour rasters | change | adopt-with-changes | no |
| M1-T6-08 | D-C | Watchers: keep the payload-less event; the gaps are listed | keep | keep | no |
| M1-T7-01 | D-D | Toolkit-free helpers written three times move into core | change | adopt-with-changes | no |
| M1-T7-02 | D-D | Spinner cycle position and arc constants stated once in core | change | keep | no |
| M1-T7-03 | D-D | Converse coverage: every model leaf has a sink, in gpui and iced too | change | adopt-with-changes | no |
| M1-T7-04 | D-D | The verification machinery pays; keep it, with the ledger of catches | keep | keep | no |
| M1-T7-05 | D-D | One unpublished test-support crate for the thrice-copied harnesses | change | keep | no |
| M1-T7-06 | D-D | Entry points and accessors share one shape; retire the parity agent | change | adopt-with-changes | yes |
| M1-T7-07 | D-D | Widget Info: three designs for one panel; generate iced's later | change | adopt-with-changes | no |
| M1-T7-08 | D-D | Widgets: separate crate for egui, in-crate for gpui and iced — keep | keep | keep | no |
| M1-T8-01 | D-E | Keep three full showcases; the shared gallery already exists | keep | keep | no |
| M1-T8-02 | D-E | Parity comparator is a hand tool; make it a release step | change | adopt-with-changes | no |
| M1-T9-01 | D-E | Each release gate traces to a mistake that happened: keep | keep | keep | no |
| M1-T9-02 | D-E | Record and pin the screenshot runner's OS version | change | adopt-with-changes | no |
| M1-T9-03 | D-E | Three gates exist on one side only: panic lints, MSRV, semver, canary | change | adopt-with-changes | no |
| M1-T9-04 | D-E | check_release.sh has one mode: 37 minutes before every commit | change | adopt-with-changes | no |
| M1-T10-01 | D-E | docs/todo.md: closed items stay, nothing marks priority or next | change | adopt-with-changes | no |
| M1-T10-02 | D-E | The archive index lags and the archive rule is only in memory | change | adopt-with-changes | no |
| M1-T10-03 | D-E | The memory index carries superseded project-status entries | change | adopt-with-changes | no |

*Reviewer* is the reviewer's own recommendation (`change` = an alternative to the current
design, `keep` = keep it); *Critic* the critic's verdict. A `keep` verdict closes the
proposal without the maintainer (plan, M1 Step 4); the rest are decided in Step 4 and the
*Decision* line of each entry records the answer.

### Entries

### M1-T1-01 · no API break · One generation table picks the preset by OS version and reports the mismatch

**Current design:** `select_reader()` fixes the live preset by target OS alone: the literals `"macos-sonoma-live"` and `"windows-11-live"` (native-theme/src/pipeline.rs:626-640). `platform_preset_name()` repeats the choice in a second function (pipeline.rs:407-437); its unknown-OS arm returns `adwaita` even on iOS, where `detect_platform()` returns `"ios"` (presets.rs:307-309). No code reads an OS version (grep over native-theme, connectors, docs, scripts, .github: the only hits are the plan's Step 10). `SystemTheme.preset` carries the preset the pipeline settled on (lib.rs:460-461); `DiagnosticEntry` (`#[non_exhaustive]`, pipeline.rs) has no version entry. The showcases label "default (…)" from `SystemTheme.preset`, else `platform_preset_name()` (showcase-gpui/app.rs:243-248; showcase-egui/app.rs:379-381).

**Problem:** Review Focus 6 is unmet: on macOS 26 a user gets Tahoe colours (the reader reads NSColor live, macos.rs:1-4) over Sonoma geometry, and nothing says so; on Windows 10, Windows 11 corner radii (windows.rs:489-492) and WinUI 3 sizes. The CI screenshots labelled `macos-sonoma` are captured on a macOS 26 runner (.github/workflows/screenshots.yml:60-66, :139-142). D4 requires a version switch; the plan's Step 10 (docs/todo_code-review-plan.md:648-660) writes a macOS-only arm, so Windows, macOS 27 and every later generation become further special cases, in two functions that can drift apart.

**Evidence:** pipeline.rs:626-640 and :407-437 (two independent literal choices); presets.rs:307-309 vs pipeline.rs:427-434 (iOS → adwaita); screenshots.yml:63 (macos-26 image); platform-facts.md tags 4 lines Sonoma, 3 Sequoia, 4 Tahoe (grep -c); objc2-foundation 0.3.2 `NSProcessInfo::processInfo()` and `operatingSystemVersion()` are safe `pub fn`s (registry src/generated/NSProcessInfo.rs:78, :135; feature `NSProcessInfo`, its Cargo.toml:167), not enabled in native-theme/Cargo.toml:62-70; `windows-version` 0.1.7 (repository microsoft/windows-rs) wraps `RtlGetVersion` in a safe `OsVersion::current()` (src/lib.rs:38-53); native-theme has no new-`unsafe` budget (plan :92-95).

**Alternatives:** (1) *Keep it*: Preset by target OS; the version gap stays undocumented in code and unreported to the application.; (2) *Plan Step 10 as written*: `macos_preset_for_version(major)` in a macOS-only arm of `platform_preset_name()`; Windows and the iOS arm untouched; `select_reader` updated by hand to agree.; (3) *One generation table per platform*: `&[(OsVersion, PlatformPreset)]` ascending; a pure `preset_for_version(table, running) -> (PlatformPreset, VersionMatch)` returning the newest generation not newer than the running OS, the oldest when the OS predates all (`VersionMatch::{Exact, PresetOlder, PresetNewer}`); used by both `select_reader` and `platform_preset_name`; the match stored on `SystemTheme` and emitted as a `DiagnosticEntry` variant.

**Recommendation:** The table. One function answers macOS 26 and 27 (the newest row not newer than 27 is `macos-tahoe` until a row is added: D4 without a special case), Windows 10 (`windows-11`, reported `PresetNewer`), and every later generation by one row plus two TOML files; the two call sites cannot diverge; the mapping is tested on Linux with literal versions, as Step 10 intends. Reading the version needs no new `unsafe` on either platform (evidence). The key type holds a build number, since Windows 11 is told from 10 by build, not major version: that boundary is an OS constant M2b sources, not stated here.

**Cost:** pipeline.rs ~80 lines (tables, function, `VersionMatch`, one `DiagnosticEntry` variant, 4 pure tests); macos.rs and windows.rs ~15 lines each (`os_version()`); Cargo.toml: the `NSProcessInfo` feature and a `windows-version` dependency on Windows (no_std, by the `windows` crate's authors); lib.rs ~10 (field + doc); presets README, new-os-version-guide, CHANGELOG. Lands before M2a Step 9.

**Unblocks:** M2a Steps 9-12 (D4), M2b Step 6's version-gap triage, the Review Focus 6 pinning tests of M4's reader review, a consistent iOS arm, and P4 (pinning) which validates names against the same table.

**Overlap with docs/todo.md:** docs/todo.md: none (grep for Tahoe, Sequoia, Windows 10, version-aware: no entry). Plan Step 10 (docs/todo_code-review-plan.md:648-660): this proposal generalises it to both platforms, adds the reported mismatch, and keeps its four test cases; the plan allows a different shape at :663-664.

**Verdict:** ADOPT WITH CHANGES — The evidence for a single shared mapping holds, and it removes the drift risk between select_reader and platform_preset_name. It is D4's switch generalised at little extra cost, with no new unsafe. The table as specified cannot report macOS 27's gap, which D4 explicitly requires, and the iOS arm contradicts D5's scope.

**Case for the current design:** Today there is one generation per platform, so a literal per target OS is the simplest correct code, and D4 only asks for a macOS version arm (plan Step 10, :648-660). select_reader ties the reader and the preset together on purpose (pipeline.rs:616-622 doc). A Windows table with one row adds a dependency and a sourced build boundary just to say 'mismatch'.

**Evidence check:** pipeline.rs:626-640 literals: verified (:631, :641). platform_preset_name :407-437: verified. iOS arm returns adwaita: verified (:427-434), but presets.rs's iOS branch is :303-306, not :307-309 (minor). No OS-version read anywhere: verified. SystemTheme.preset lib.rs:460-461: verified. DiagnosticEntry #[non_exhaustive] pipeline.rs:179-180, no version variant: verified. Showcase labels gpui app.rs:243-248 and egui app.rs:379-381: verified. windows.rs:489-492 'Windows 11 defaults' radii: verified. screenshots.yml: half-true. runs-on is macos-latest (:15, :43). :60-66 is a comment about a macos-26 run in another repo, so the repo does not state the runner image. The captures use the static `--theme macos-sonoma` preset (:139-150), so the reader/version gap does not reach them. platform-facts counts 4 Sonoma / 3 Sequoia / 4 Tahoe: verified. objc2-foundation 0.3.2: processInfo() at NSProcessInfo.rs:78 and operatingSystemVersion() at :135 are safe pub fns, feature at Cargo.toml:167, and the feature is not enabled in native-theme/Cargo.toml:61-69: verified. windows-version 0.1.7 has a safe OsVersion::current() over RtlGetVersion (lib.rs:38-53), repository microsoft/windows-rs: verified. It is already in Cargo.lock through tauri-winrt-notification. Plan :92-95 no-new-unsafe: verified.

**Cost check:** Roughly right for the core. It misses: select_reader's own test (pipeline.rs:1554-1583); diagnose_platform_support, which must read the version itself to emit the entry, plus that entry's label/Display arms; with_overlay carrying the match; and a field type for Linux and preset-only paths, where no table exists, so it must be an Option. A plain start-version table cannot express D4's macOS 27 rule (see changes). The Windows boundary build must come from M2b with a source.

**Changes required:** (1) Each row carries its first version and its last verified version. A running OS newer than the last verified version reports PresetOlder, so macOS 27 under macos-tahoe is reported as D4 requires ('documented as such'); without this, 27 reads as an exact match. (2) VersionMatch is #[non_exhaustive]. The SystemTheme field is Option<…> (None on Linux and on preset-only fallbacks) and survives with_overlay. (3) Drop the iOS arm from scope. Under D5 there is no iOS reader and from_system returns PlatformUnsupported there, and a live 'ios' would point at a nonexistent ios-live. At most a label fix with is_live:false. (4) The Windows 10/11 build boundary and the macOS row bounds come from M2a/M2b's version maps with citations, never stated in M1. (5) Correct the ledger's evidence on screenshots as noted. Keep Step 10's four test cases, plus one asserting PresetOlder for 27.

**Decision:** adopt with the critic's changes (maintainer, 2026-10-11, round 1): rows carry first and last-verified version, `VersionMatch` is `#[non_exhaustive]` and `Option` on `SystemTheme`, no iOS arm (D5), version bounds sourced in M2a/M2b; replaces M2a Step 10's macOS-only arm.
**Status:** scheduled: M2a, before Step 9

### M1-T1-02 · no API break · Readers' hardcoded generation geometry moves into the live presets

**Current design:** The macOS reader builds its output on `macos_widget_defaults()` (macos.rs:311-406, 96 lines of HIG/AppKit constants, called from `build_theme` at :430); the Windows reader on `winui3_widget_sizing()` (windows.rs:223-277) plus corner radii labelled "Windows 11 defaults" (windows.rs:489-492). The pipeline merges reader output over the live preset (pipeline.rs:48-52; `with_overlay` repeats the order, lib.rs:566-572), so the Rust copy wins over `macos-sonoma-live.toml`, which states the same values (`:49`, `:58`, `:66` = 22, 22, 14, matching macos.rs:315, 327, 333) and over `windows-11-live.toml` (`:20-21`, `:48`, `:58`, `:74`). `documented_sizes.rs:113-116` exists to keep the copies equal; `docs/new-os-version-guide.md:49-55` and `:89-94` tell a maintainer to update both.

**Problem:** A generation's geometry lives in three places (full preset, live preset, reader function) and the Rust one decides. Under D4, `macos-tahoe-live.toml` cannot change a button height: `macos_widget_defaults()` overrides it. Plan Step 9 (:638-644) says the live base "omits the fields MacosReader provides"; for these fields that premise means Tahoe's values would have to go into Rust, so Step 10's switch must also fork a `*_widget_defaults` function per generation on each platform. docs/todo.md:2580 already records the Windows duplication as a defect, from the TOML side.

**Evidence:** pipeline.rs:51-52 (`merged.merge(&live_preset); merged.merge(&reader_as_theme)`); macos.rs:430 (`let widget_defaults = macos_widget_defaults();` seeds both variants); windows.rs:218-222 ("WinUI3 … constants (not from OS APIs)"); macos-sonoma-live.toml:47-66 and windows-11-live.toml:46-74 state the same widgets' sizes; documented_sizes.rs:46-47, :113-116 (`reader_constants` compares them); baseline: 1104 native-theme tests pass (findings ledger, Cross-target clippy), so the copies agree today.

**Alternatives:** (1) *Keep it*: P1's switch also selects a `*_widget_defaults` function per generation, two Rust forks per new generation, the gate keeping three copies equal.; (2) *Move the constants into the `-live` presets*: Readers return only what they read from the OS (NSColor, NSFont, UISettings, system metrics, accessibility); the generation's design constants live in its `-live` TOML alone; `documented_sizes.rs` checks the live preset against platform-facts; its macOS and Windows `reader_constants` arms go.; (3) *Functions read the live preset*: Keep the functions as thin wrappers over the TOML: an indirection with no reader value.

**Recommendation:** Move. The gate passing today proves the TOML copies already hold the values, so the change is a deletion plus any field the gate then reports missing, added from the function's cited platform-facts line. A new generation becomes two TOML files and a table row, which is what "one preset per generation" means. Not the KDE reader: `populate_widget_sizing` carries Breeze's constants and KDE is outside T1's generations. This touches reader code M4 reviews, so it lands before M4 or M4 reads it after.

**Cost:** macos.rs −96 lines and `build_theme` simplified (~10); windows.rs −60; documented_sizes.rs −15 plus its doc; new-os-version-guide.md: the two "Files to update" lists shrink to the TOML and a table row; live presets: fields the functions set that the TOML lacks (the gate lists them). No dependency. Everything touched is `pub(crate)`.

**Unblocks:** P1's table being sufficient for D4 (Step 9 needs no Rust fork); the docs/todo.md:2580 defect, from the other side; shorter readers for M4's `cfg(target_os)` review (Review Focus 3).

**Overlap with docs/todo.md:** docs/todo.md:2579-2581 (windows-11-live states reader-provided fields): that entry treats the TOML as the duplicate; this proposal makes the Rust the duplicate and removes it. Plan Step 9's premise (:638-644) is noted as false for these fields.

**Verdict:** ADOPT WITH CHANGES — The structural argument holds. Under D4 the reader's Sonoma constants would override macos-tahoe-live, so moving them out is the cheapest way to make one preset per generation true. But the premise that the copies agree today is false in three fields, so the change alters live values and must be ruled field by field from platform-facts.

**Case for the current design:** v0.5.9's rationale (docs/archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-rationale.md §6, :125-155) recorded that reader constants win on the live path. It chose a gate (documented_sizes.rs) checking both paths against platform-facts, not a deletion, and moved the constants out of the OS gates so every run sees them. The readers then stay self-describing, and the WinUI3/AppKit citations sit beside the code that applies them.

**Evidence check:** pipeline.rs:48-52 merge order: verified; lib.rs:566-572: verified. macos_widget_defaults at macos.rs:311-406, called at :430: verified. winui3_widget_sizing at windows.rs:223-277, doc :218-222: verified. The live TOML lines for 22/22/14 (:49, :58, :66): verified, though the reader's lines are :315, :331, :326, not :315/:327/:333. 'documented_sizes.rs exists to keep the copies equal' is wrong: it checks static and live resolutions against platform-facts for gated sizes only (:1-47). 'The gate passing proves the copies agree' is wrong. The macOS reader states groove_width 15 against the live TOML's 16 (macos.rs:341 vs macos-sonoma-live.toml:78), and divider_width 9 against 6 (macos.rs:401 vs :131). The Windows reader states divider 4 against the preset's 1 (windows.rs:276). The gate covers none of these fields, and docs/todo.md Table B already records all three (:2272, :2279), which the proposal did not cite. The claim that this resolves todo.md:2579-2581 'from the other side' is wrong: that entry is about OS-read fields (SM_CXFOCUSBORDER, SM_CXVSCROLL, SM_CYVTHUMB), which the proposal keeps in the reader. 1104 tests pass: verified (ledger :45).

**Cost check:** Line counts are plausible. It misses: macos.rs's spot-check test (:801-823) and build_theme tests; windows.rs tests at :1122 and :1339; the live-output value changes in the three fields above (a CHANGELOG line, plus the Table B rows closed in docs/todo.md); and new-os-version-guide's KDE text, which is unaffected. No connector or showcase code changes. Screenshots on the live path change only through those three fields.

**Changes required:** (1) Before deleting, add a one-off test that diffs every field the reader constants set against the merged full+live preset. Every difference is ruled from platform-facts: today groove 15/16 and divider 9/6 on macOS, divider 4/1 on Windows. The sourced value wins, the matching docs/todo.md Table B cells are updated, and nothing is invented. (2) Do not claim it closes todo.md:2579-2581. (3) Under D3 these are value changes, so schedule the change in M2a/M2b ahead of Step 9, not in M1's fix phase. (4) Coordinate with T4 (D-B): if the -live bases become derived, the constants land in the full preset. Since the full preset is merged first, deleting the reader constants may need no TOML change at all.

**Decision:** adopt, scheduled in M2a/M2b (round 1): a one-off diff test first, each difference ruled from platform-facts (the sourced value wins, Table B updated), then the reader constants go; coordinated with M1-T4-01 (the tier is removed, so the constants live in the full preset, which is merged first).
**Status:** scheduled: M2a (macOS), M2b (Windows)

### M1-T1-03 · no API break · Naming rule: one preset per generation, vendor's name, version range in the header

**Current design:** Platform presets `windows-11`, `macos-sonoma`, `material`, `ios`, `kde-breeze`, `adwaita` (presets.rs:21-30, :69-80), each with a `-live` twin by suffix, which `run_pipeline` (pipeline.rs:36-41), `with_overlay` (lib.rs:560-565), `PlatformPreset::live_name` (pipeline.rs:345-351) and `documented_sizes.rs:84-92` all rely on. The headers name a version only through the name (macos-sonoma.toml:1-3, windows-11.toml:1-3). platform-facts tags rows unevenly (4 lines Sonoma, 3 Sequoia, 4 Tahoe, 6 Windows 10; the macOS column header at :1023 names none). The guide says "create a new preset, e.g., windows-12.toml, if the visual language changes significantly" (new-os-version-guide.md:69-70) and gives no rule.

**Problem:** Review Focus 6 asks that the preset's documentation say which version it is; neither header does, and D4's "`macos-sonoma` covers 14-15" is stated nowhere. Without a rule the next generation's key is improvised per platform, as D6 had to settle Material's by hand. `windows-10` is in the brief: adding it means a facts column, a probe with no Windows 10 runner (plan :613-615: Server images only) and a preset nobody can verify.

**Evidence:** presets.rs:21-30, :69-80; pipeline.rs:36-41, :345-351; lib.rs:560-565; documented_sizes.rs:77, :89; macos-sonoma.toml:1-3; windows-11.toml:1-3; platform-facts.md:173, :1155, :1259, :1278-1279, :1672, :1709 (version tags per row), :1023 (no version in the column header); new-os-version-guide.md:69-70; plan :183-184 (D4's range sentence), :613-615 (runner images); windows.rs:489-492 (the only Windows-11-specific constants are design constants; fonts, colours and metrics are read live, windows.rs:1-8).

**Alternatives:** (1) *Keep it*: Names as they are, no header rule; each new generation named ad hoc.; (2) *The rule*: One preset per visual generation; the key is the vendor's own name for the generation's first OS version (Apple codename: `macos-sonoma`, `macos-tahoe`; number: `windows-11`, `ios-<N>`; design version: `material-3`, D6); the header's first lines state the version range it covers and the versions its values were verified on, matching the facts rows' version tags; `kde-breeze` and `adwaita` keep style names, their pinned style version (Breeze v6.7.5, libadwaita 1.10.0, M2e) in the header, because the reader reads the installed style and the style, not the desktop's version, is the generation.; (3) *Numbered keys everywhere*: `macos-14`, `macos-26`: Apple's own naming is the codename; rejected.

**Recommendation:** The rule, written into `presets/README.md` and the guide, applied to every platform preset. No `windows-10` before 1.0: M2b Step 1 records its support status; on Windows 10 the reader reads fonts, colours and metrics live, so the gap is the WinUI 3 design constants alone, which P1 reports as `PresetNewer`; a user who needs that look has P4 once an older preset exists, which none does.

**Cost:** Documentation only: presets/README.md (~15 lines), new-os-version-guide.md (~20, replacing :69-70 and :93-94 with the rule and the table row), the four platform preset headers (2-3 lines each), CHANGELOG. The `ios` rename is P5's.

**Unblocks:** M2a Step 11 (documentation), M2c's naming decision, M2b Step 6's version-gap triage, and T5's per-row version field if adopted.

**Overlap with docs/todo.md:** docs/todo.md: none. Plan :183-184 (D4's "documents that it covers 14-15") is the first instance of the header rule; this proposal makes it the rule for every platform preset.

**Verdict:** ADOPT WITH CHANGES — The rule is consistent with D4 and D6 and costs only docs. Stating ranges and verified-on versions before M2 has checked them would break the never-invent rule, so the rule lands now and the header contents land with the evidence.

**Case for the current design:** The names already follow the vendors' own naming (Sonoma codename, Windows 11 number), and the guide leaves room for windows-12. A rule written before a second generation exists may not fit the one that comes. Version ranges in headers can go stale, and the facts file already tags versions per row.

**Evidence check:** presets.rs:21-30, :69-80: verified. pipeline.rs:36-41 and :345-351, lib.rs:560-565, documented_sizes.rs:77/:89: verified (suffix convention). Headers macos-sonoma.toml:1-3 and windows-11.toml:1-3 name no range: verified. platform-facts tag counts 4/3/4 and 6 Windows 10/Win10: verified. The macOS column header at :1023 has no version: verified. new-os-version-guide.md:69-70 and :93-94: verified. Plan :183-184 (D4 range) and :612-614 (Server runner images only): verified. Breeze v6.7.5 and libadwaita 1.10.0 are the plan's M2e pins: verified. windows.rs:1-8 says fonts, colours and metrics are read live: verified.

**Cost check:** Documentation only, and correct as scoped. The header text for range and verified-on versions cannot be written in M1 without M2's version maps; it belongs to each M2 part's close (M2a Step 11 for macOS, and the M2b/M2c/M2e closes).

**Changes required:** Adopt the rule in presets/README.md and the guide now. Write each header's 'covers' and 'verified on' lines only from that platform's M2 Step 1 version map and Step 2 checks, at that part's close. macos-sonoma says 14-15 only if M2a confirms D4's condition. Until then a header names the generation and states no range, so no version claim is invented. Keep no windows-10, with the reason recorded once M2b Step 1 records Windows 10's support status.

**Decision:** adopt the rule now, headers at each M2 part's close (round 1): the rule into presets/README.md and new-os-version-guide.md in M1; each header's 'covers' and 'verified on' lines from that part's sourced version map.
**Status:** M1 fix phase (rule); headers in M2a–M2e

### M1-T1-04 · no API break · Pinning an older generation on a newer OS: from_system_with_preset

**Current design:** `SystemTheme::from_system()` (lib.rs:671-693) runs `from_system_inner` → `select_reader`, which fixes the live preset (pipeline.rs:626-640); `run_pipeline` already takes the preset name as a parameter (pipeline.rs:20-24). A user's only way to an older look is `Theme::preset("macos-sonoma")` resolved statically, without the OS's colours, fonts, DPI and accessibility; `with_overlay` (lib.rs:556-572) replays the pipeline on the same live preset. No options struct or builder exists on the system path (grep for `Options|Builder|Config` structs in native-theme/src: none).

**Problem:** Once two macOS generations ship (D4), an application that wants its Sonoma look on a Mac running 26, or a user who prefers it, has no path that keeps the live OS data; the showcases will list both presets (plan Step 12, :667-670) but "default" cannot be steered. The brief asks how a user pins an older generation's look; today the answer is "only without the OS data".

**Evidence:** lib.rs:671-693 (no parameter); pipeline.rs:626-640 (literal live name), :20-24 (`run_pipeline(reader, preset_name, mode)`); lib.rs:556-572 (`with_overlay` reuses `overlay_source.preset_name`); presets.rs:139-181 (`PresetInfo.platforms` gives the platform check a name must pass).

**Alternatives:** (1) *Keep it*: Pin only through the static preset; document that `from_system` always picks the running generation.; (2) *`SystemTheme::from_system_with_preset(name)`*: Plus an async twin. Validates that `name` is a platform preset whose `PresetInfo.platforms` includes the running platform (presets.rs:139-181) and that a `-live` twin exists; runs the platform reader; passes `<name>-live` to `run_pipeline`; `Error::UnknownPreset` otherwise; `VersionMatch` (P1) reports the chosen generation against the running OS.; (3) *An environment variable read by `from_system`*: Hidden control flow inside a library; rejected.

**Recommendation:** The constructor, additive and small, scheduled after P1 lands (it validates against P1's table and reuses `VersionMatch`), and only once a second generation exists on some platform (M2a Step 9); before that it has nothing to choose.

**Cost:** lib.rs ~40 lines (two constructors, docs, a doctest); pipeline.rs ~20 (`from_system_inner` split into a body taking the live preset name); tests ~20; optional showcase flag (M9). No dependency.

**Unblocks:** M2a Step 12's menus showing both generations over live OS data; application-side pinning; Review Focus 6 gains a user action beside the report.

**Overlap with docs/todo.md:** none.

**Verdict:** ADOPT WITH CHANGES — It is the only path that keeps live OS data while choosing a generation, it answers the brief's pinning question, and it is small and additive. But it has no use until a second generation exists, and its result is a hybrid look that must be documented so it does not mislead.

**Case for the current design:** Today there is nothing to pin: one generation per platform. An additive constructor can be added at any time, including after 1.0, so the pre-1.0 argument for doing it now does not apply. No user has asked, and a static Theme::preset already gives the old look.

**Evidence check:** lib.rs:671-693 from_system takes no parameter: verified. pipeline.rs:20-24 run_pipeline takes preset_name: verified. lib.rs:556-572 with_overlay reuses overlay_source.preset_name: verified. presets.rs:139-181 PresetInfo.platforms: verified. No Options/Builder/Config struct in native-theme/src: verified (grep empty). select_reader's literal at :631/:641: verified.

**Cost check:** About right: lib.rs constructors and doctest, from_system_inner split, tests. It misses an accessibility_from_system_inner counterpart (none needed, worth stating), the CHANGELOG, and the showcases' default-label logic if M9 adds a flag. No dependency.

**Changes required:** (1) Record it now as a docs/todo.md entry. Implement only after P0 and macos-tahoe (M2a Step 9) land, not in M1's fix phase. (2) The rustdoc must say what a pin gives: the older generation's geometry over the running OS's colours, fonts and accessibility (on macOS 26, NSColor gives Tahoe colours). That is a hybrid, not the old OS's look, and it is reported as PresetOlder through P0's match. (3) Validate the name against P0's table rows for the running platform, not only PresetInfo.platforms.

**Decision:** adopt after macos-tahoe lands (round 1): recorded on docs/todo.md in M1; implemented after M1-T1-01 and M2a Step 9, the rustdoc stating the hybrid result.
**Status:** scheduled: after M2a Step 9; docs/todo.md entry in M1

### M1-T2-01 · no API break · README states what ios and material are; ios named by its version after M2c

**Current design:** native-theme/README.md:50 lists `material` and `ios` under *Platform* with no word on when to use them; README.md:42 omits them; presets/README.md:14-22 says only that `ios` has no platform-facts column, its table calling them "iOS/iPadOS standalone theme" and "Material Design standalone theme" (:53, :58). `list_presets_for_platform` offers `ios` on macOS and iOS only (`platforms: &["macos", "ios"]`, presets.rs:172-174; doc at :320 and model/mod.rs:670-672) and `material` everywhere (:166-170). `ios.toml:1-5` cites "Apple UIColor system colors, Human Interface Guidelines" in 4 comment lines and names no version; `material.toml:1-7` cites the v0_192 token set at a commit.

**Problem:** D5 says the two presets "stay as looks a desktop app can choose; the README says so": no README says so yet. `ios` is not offered on Windows or Linux, and should not be: its icon set is SF Symbols (`icon_set = "sf-symbols"`, ios.toml:4), loaded only on macOS (sficons.rs cfg arms :152-190), and an icon set is never substituted; the README should say that too. `ios` names no iOS version while iOS 26 changed the design (plan :25-26), so Review Focus 6 cannot be met by documentation either; docs/todo.md:2317-2322 records that every `ios` size is unsourced.

**Evidence:** grep of native-theme/README.md for `ios`: only :50; README.md: none; presets.rs:166-174; model/mod.rs:670-672; ios.toml:1-5 (4 comment lines, `grep -c`); sficons.rs:152, :159, :170, :190 (`cfg(target_os = "macos")`); icons.rs:498, :514 (SF Symbols is the macOS/iOS system set); docs/todo.md:2317-2322; plan :25-26, :188-193 (D5).

**Alternatives:** (1) *Keep it*: The list line and the table rows as they are; `ios` stays unversioned.; (2) *README sentences and a versioned key*: One sentence per preset in native-theme/README.md and presets/README.md: `material` (`material-3` under D6) is a look any desktop app may choose on any platform, with its bundled icons; `ios` is a look for apps on Apple platforms (SF Symbols), offered by `list_presets_for_platform` on macOS and iOS and loadable anywhere by `Theme::preset`, where its icons are absent; neither has a reader (D5). Key `ios-<N>`, N the iOS version M2c Step 2 finds the values match, the header stating the range per P3's rule; `ios` goes without alias, as D6 does for `material`.; (3) *Offer `ios` everywhere*: `platforms: &[]`: a preset whose icon set cannot load off Apple platforms; contradicts the icon rule; rejected.

**Recommendation:** The second. The README sentences now (D5's own requirement); the version in the name only after M2c Step 2 says which iOS version the values match: naming it today would be a guess. If M2c finds the values match no single version, the header says so and the key waits.

**Cost:** Now: three READMEs, ~8 lines. After M2c: rename ios.toml, three entries in presets.rs (:30, :76, :172), documented_sizes.rs (:15, :732), CHANGELOG; the showcases list presets by key and need no change. No dependency.

**Unblocks:** M2c's scope (which version to check) and the README line D5 requires.

**Overlap with docs/todo.md:** docs/todo.md:2317-2322 (every `ios` size unsourced: M2c's work; this proposal adds the name and the README text), :2575-2577 (readers deferred past 1.0: unchanged).

**Verdict:** ADOPT WITH CHANGES — D5 itself requires the README text, so the README part is mandatory and cheap. The versioned key correctly waits for M2c. The proposed wording overstates where SF Symbols load, which would mislead iOS users.

**Case for the current design:** Under D5 neither preset has a reader, and the presets README already calls them standalone themes. list_presets_for_platform already enforces where ios is offered, so more README prose repeats code. Renaming ios is cosmetic until M2c says which version it matches.

**Evidence check:** native-theme/README.md:50 is the only ios/material preset mention: verified. README.md has none: verified. presets/README.md: ios and material rows are at :57 and :62, not :53/:58 (minor). presets.rs:166-174 platforms (material &[], ios macos+ios): verified. model/mod.rs:670-672: verified. ios.toml names no version, and icon_set sf-symbols is at :4: verified. 'cites in 4 comment lines' is wrong: the header has 2 comment lines (:1-2), and 4 is the file's total. sficons.rs cfg arms :152/:159/:170/:190: verified. 'icons.rs:498, :514' is wrong file: it is model/icons.rs:498 (system_icon_set at :510-516). SfSymbolsLoader::load is cfg(target_os = "macos") only (icons.rs:308), so the icons are absent on iOS as well, which the proposal misses. todo.md:2317-2322 (ios sizes unsourced): verified. Plan :25-26 and D5: verified.

**Cost check:** Now: about 8 README lines, correct. After M2c: the rename touches presets.rs (:30, :76, :172), documented_sizes.rs, tests, the CHANGELOG and the showcases' preset lists (they enumerate keys, so no code change), as stated. It also needs the grep sweep the material-3 rename uses (plan Step 7) for '"ios"' preset uses.

**Changes required:** README sentence for ios: it is a look for desktop apps on macOS, and its SF Symbols load only on macOS (SfSymbolsLoader is cfg(target_os="macos")). It is offered on macOS and iOS by list_presets_for_platform, but on iOS and elsewhere its icons are None and are never substituted. For material: its Material Symbols icons load with the material-icons feature. Rename to ios-<N> only after M2c Step 2, with no alias; if no single version matches, the header says so and the key stays. Fix the line citations.

**Decision:** adopt with the critic's wording (round 2): README sentences now (SF Symbols load only on macOS; icons None elsewhere, never substituted; material's icons with the material-icons feature); `ios` → `ios-<N>` only after M2c Step 2, no alias.
**Status:** M1 fix phase (READMEs); rename in M2c

*Reviewer D-A notes:* Read-only review; no build or test run. Verified in the cargo registry, not by running: objc2-foundation 0.3.2's `NSProcessInfo::processInfo()` and `operatingSystemVersion()` are safe methods (src/generated/NSProcessInfo.rs:78, :135) behind its `NSProcessInfo` feature, which native-theme does not enable yet (native-theme/Cargo.toml:62-70); windows-version 0.1.7 (repository microsoft/windows-rs) exposes a safe `OsVersion::current()` over `RtlGetVersion` (src/lib.rs:38-53). The Windows 10/11 build boundary is deliberately not stated: M2b sources it. Counts verified with grep: platform-facts.md lines mentioning Sonoma 4, Sequoia 3, Tahoe 4, Windows 10/Win10 6; `macos-sonoma` appears in 37 files outside target/ and the archive, 13 lines in presets.rs and 9 in pipeline.rs; the presets directory holds 22 entries (20 TOML + README.md + documented_sizes.rs). Two things the plan may want to know: (1) Step 9's premise that the live base omits the fields the reader provides is false for the geometry `macos_widget_defaults()` and `winui3_widget_sizing()` hardcode in Rust, which the pipeline merges over the live preset (pipeline.rs:51-52), so without P2 a `macos-tahoe-live.toml` cannot change those fields; (2) `platform_preset_name()` returns `adwaita` on an iOS target (pipeline.rs:427-434) while `detect_platform()` returns `ios` (presets.rs:307-309), a small inconsistency P1's table removes. P2 changes reader code M4 reviews; per the plan's rule it should land before M4 or M4 reads the readers after it. Question for the maintainer: does the `-live` suffix convention stay the way a generation's live twin is found (four places rely on it), or should P1's table carry both names explicitly?

*Critic D-A notes:* I checked all five proposals against the code, the plan (D0-D7, the Global Constraints, Review Focus 6, M2 Steps 8-12), the ledger's Baseline and docs/todo.md. No proposal contradicts a settled decision, apart from P0's iOS arm, which goes beyond D5's scope.

Main problems found:
- **P1's premise is false.** The reader constants and the live presets differ today. macOS has scrollbar groove 15 against 16 (macos.rs:341, macos-sonoma-live.toml:78) and splitter divider 9 against 6 (macos.rs:401, macos-sonoma-live.toml:131). Windows has divider 4 against 1 (windows.rs:276). docs/todo.md Table B already records all three (:2272, :2279), and P1 did not cite it.
- **P1 does not close todo.md:2579-2581.** That entry is about fields the Windows reader reads from the OS, which P1 keeps in the reader.
- **P0's table cannot report macOS 27.** A table keyed only by start version treats macOS 27 under macos-tahoe as an exact match, so D4's "documented as such" needs a last-verified bound on each row.
- **The CI screenshots claim is overstated.** They use the static macos-sonoma preset, so the reader's version gap does not reach them. The repo shows only `runs-on: macos-latest`; the macos-26 comment is about another repo's run.
- **P4 overstates where SF Symbols load.** They load only on macOS (icons.rs:308), so the ios preset's icons are absent on iOS too.

On ordering: P0's version bounds, P1's value changes (D3) and P2's header contents all belong to the M2 parts, which hold the sourced versions. M1 can adopt P2's rule and P4's README sentences now. P3 waits for P0 and macos-tahoe. P1 must be coordinated with T4 (D-B), since the -live bases may become derived.

### M1-T3-01 · no API break · Theme::resolve spawns a subprocess per call; expose the pure path

**Current design:** `Theme::resolve(mode)` (native-theme/src/model/mod.rs:460-462) builds `ResolutionContext::from_system()` (resolve/context.rs:77-88): `detect::system_font_dpi()` (detect.rs:888, uncached) runs `detect_system_font_dpi` (detect.rs:405-437): kdeglobals/kcmfontsrc file reads, then `xrdb -query` spawned through `run_with_timeout` (detect.rs:256-257, :198-229), which sleeps 1, 2, 4… ms between polls (detect.rs:179-182), then `xrandr`. Icon theme is cached (detect.rs:854-855); button order reads an env var (detect.rs:78-79). The pure entry `resolve_in(mode, &ctx)` is `pub(crate)` (mod.rs:467-471); `ThemeMode::into_resolved(&ctx)` is public but returns `ResolvedTheme` without the icon tiering `Resolved` carries (resolved.rs:258-263). The pipeline calls `from_system()` again after the reader ran the same DPI chain (pipeline.rs:70-73; kde/mod.rs:111-112). egui resolves both modes per preset (connectors/native-theme-egui/src/ext.rs:192-193).

**Problem:** Ledger Baseline: resolve 1.30–1.50 ms per call against ~1 µs to parse, with jumps to 3.4 ms (×12 in run 2) and 5.2 ms once. On this machine (the ledger's) `forceFontDPI` is absent and `Xft.dpi` is set, so every resolve reaches `xrdb`; a Python spawn of `xrdb -query` takes 1.11 ms median (11 runs), matching the floor; a missed 1 ms poll adds 2 ms (3.4), two add 4 (5.2). A function documented as a conversion (mod.rs:442 names `from_system`, not its cost) reads env, files and a process on every call, twice per `SystemTheme::from_system`, twice per egui preset switch, and no public path gives a caller-supplied context plus the icon tiering.

**Evidence:** docs/todo_code-review-findings.md Timings table and the [M3 R2] observation; detect.rs:179-182, :198-229, :256-257, :405-437, :888; context.rs:77-88; model/mod.rs:442, :460-471; pipeline.rs:70-73; kde/mod.rs:111-112; ext.rs:192-193; gpui lib.rs:284 and iced lib.rs:308 use `resolve_system()`; 71 `for_tests()` call sites in 22 files already avoid the I/O in tests; local: `grep forceFontDPI ~/.config/kdeglobals ~/.config/kcmfontsrc` empty, `xrdb -query | grep -c Xft.dpi` = 1.

**Alternatives:** (1) *keep it*: resolve() stays the only Theme-level entry; the subprocess cost is M3 R2's to shrink.; (2) *A: public resolve_with(mode, &ctx)*: Make `resolve_in` public under a clear name; connectors and the pipeline build one `ResolutionContext` per theme-build (the pipeline reusing the reader's DPI instead of re-detecting); `resolve()` stays as the documented I/O convenience.; (3) *B: cache font_dpi process-wide*: Put `font_dpi` into the `DetectionContext` OnceLock beside the icon theme (detect.rs:854); misses a DPI change at runtime unless the watcher invalidates it.; (4) *C: resolve() takes the context*: Change the signature; every caller passes a context. Breaking.

**Recommendation:** A. The G7 rationale introduced `ResolutionContext` so detection happens "once per theme-build" (resolve/mod.rs:180-186), but the Theme-level API never received it, so the convenience became the only path. A is additive, keeps `resolve()` for scripts and examples, and makes the double detection in `run_pipeline` a one-line fix. B is a value question for M3 R2, not a design one.

**Cost:** native-theme/src/model/mod.rs: ~15 lines (pub fn, docs); pipeline.rs:70 and lib.rs:559-572: ~10 lines to pass the reader's DPI into the context builder; egui ext.rs:192-193: ~5 lines. No new dependency. Tests unchanged (they use `for_tests()`).

**Unblocks:** Deterministic resolution for applications and their tests without environment dependence; halves the per-switch cost of egui's preset path and of `SystemTheme::from_system`; gives M3 R2 a boundary to measure.

**Overlap with docs/todo.md:** None in docs/todo.md; the ledger's [M3 R2] note names the cost only. "### Font DPI" (docs/todo.md:2582-2584) is the logical-versus-physical question, untouched. The proposal adds the API shape.

**Verdict:** ADOPT WITH CHANGES — The evidence holds, and G7's own source (doc 2 §J.2) wanted a Theme-level entry that takes a context, so this is additive and in the design's spirit. The cost estimate names one wrong site and misses the egui call sites and the doc.

**Case for the current design:** G7 (docs/archive/v0.5.7_gaps.md:336-400) introduced ResolutionContext with from_system()/for_tests() and no Default so that callers state where the inputs come from. Theme::resolve is the one-line convenience, and its doc (model/mod.rs:442-445) sends custom-context callers to ThemeMode::into_resolved. resolve_in stayed pub(crate) to keep the API small. The cost belongs to M3 R2.

**Evidence check:** Ledger timings 1.30-1.50 ms, 3.36-3.49 ms x12 and 5.18 ms: verified (findings.md:57, :71-73 [M3 R2]). mod.rs:460-471 (resolve calls from_system, resolve_in is pub(crate)): verified. context.rs:77-88: verified. detect.rs poll 1,2,4 ms capped at 50 (:174-182), run_with_timeout (:192-229), xrdb via run_with_timeout (:256-257), DPI chain (:405-437), uncached system_font_dpi (:888-890): verified. 'then xrandr' is wrong for this machine: xrandr runs only when Xft.dpi is missing, so with Xft.dpi set it never runs. pipeline.rs:70-73 runs from_system after the reader's own DPI chain (kde/mod.rs:111-112): verified. egui ext.rs:192-193 resolves twice: verified. gpui lib.rs:284 and iced lib.rs:308 use resolve_system: verified. 71 for_tests() calls in 22 files: verified. The xrdb spawn median of 1.11 ms and the local grep results: unverifiable (not re-run).

**Cost check:** The model/mod.rs change (~15 lines) is right. Two mistakes. First, lib.rs:559-572 is with_overlay, which already reuses the stored OverlaySource.context (lib.rs:609-610) and needs no change. Second, the pipeline's double DPI detection is independent of making resolve_in public: it needs a context constructor that skips DPI detection when the reader supplied font_dpi. The estimate also misses: the other egui double-resolve sites (icons.rs:576-581, plugin.rs:336), the doc at mod.rs:442-445 that points to into_resolved, a CHANGELOG line, and a doctest. No dependency is needed.

**Changes required:** 1) Add a public Theme::resolve_with(mode, &ResolutionContext) (resolve_in renamed or wrapped) and rewrite the doc at mod.rs:442-445 to point to it. 2) Fix the double detection separately: run_pipeline (pipeline.rs:70-73) builds its context without running system_font_dpi when the reader returned a font_dpi. lib.rs:559-572 stays untouched. 3) Build one context per call in egui ext.rs:192-193, icons.rs:576-581 and plugin.rs:336. 4) Correct the problem statement: on the ledger machine the chain is file reads then xrdb; xrandr does not run.

**Decision:** adopt with the critic's changes (round 2): public `Theme::resolve_with(mode, &ResolutionContext)` and the doc rewrite; the pipeline skips DPI detection when the reader supplied `font_dpi`; egui builds one context per call at its three sites; the problem statement corrected (xrandr does not run when Xft.dpi is set).
**Status:** M1 fix phase

### M1-T3-02 · no API break · Theme → ThemeMode → Resolved with Option fields: keep it

**Current design:** `Theme` holds `light`/`dark: Option<ThemeMode>` plus shared `layout`, `icon_set`, `icon_theme` (model/mod.rs:260-300); `ThemeMode` holds `defaults`, `text_scale` and 26 widget structs (mod.rs:107-135); 224 `Option<` fields across the 29 structs of model/widgets/mod.rs; `#[derive(ThemeWidget)]` generates the `Resolved*` twin, merge, is_empty, validate and range checks from field attributes (native-theme-derive/src/lib.rs:21-44; sample ButtonTheme widgets/mod.rs:37-104). Merge is "Some overlays" (gen_merge.rs:40-49; mod.rs:337-360). `Resolved` = `ResolvedTheme` + icon fields (resolved.rs:258-263). Soft options stay `Option` after resolution (theme-contracts rationale C16).

**Problem:** None for the goal. The pipeline is sparse overlays: reader output (`documented_sizes.rs:105-120` builds reader constants as a sparse `ThemeMode`), preset, app TOML, then resolution (v0.5.0_resolution.md:97-131); every field needs an "unstated" state and connectors need plain values. The two-type split delivers both, and the derive keeps the pair in step. Validation names every missing path at once (v0.5.0_resolution.md:186-200).

**Evidence:** widgets/mod.rs: 29 `pub struct`, 224 `Option<`; pipeline.rs:48-51 and :103-110 (three-layer merge, inactive variant from the full preset); gen_merge.rs:40-49; resolved.rs:258-263; docs/archive/v0.5.0_resolution.md:97-205; docs/archive/v0.5.0_theme-variant.md:821-845 (sparse structs are deliberate).

**Alternatives:** (1) *keep it*: Option per field in the spec types, plain values in the Resolved twins, generated by the derive.; (2) *path→value map*: A `HashMap<FieldPath, Value>` model: trivial merge and generic gating, but loses static typing, serde renames, per-field range and inheritance attributes, and connector ergonomics.; (3) *one struct with sentinels*: Default values instead of Option: cannot distinguish an unstated size from a stated 0, which the gates (documented_sizes.rs) rely on.

**Recommendation:** Keep it. The design serves "minimal code in the application": an app reads plain fields from `ResolvedTheme`; the cost of Option lives in the macro, not in hand-written code. No bug, todo entry or measured cost points at the layering.

**Cost:** None.

**Unblocks:** Nothing; it confirms the base M3 reviews against.

**Overlap with docs/todo.md:** none

**Verdict:** KEEP — Every claim checks out, and the alternatives either lose static typing or lose the difference between unstated and 0 that the gates rely on. No todo entry or measurement points at the layering.

**Case for the current design:** v0.5.0_resolution.md:97-205 splits sparse Option specs (merge layers) from non-Option Resolved output, and validation reports every missing path at once. v0.5.0_theme-variant.md:821-845 says sparse structs are deliberate. The derive (v0.5.6_validate-derive-macro.md) generates the twin, merge, validate and ranges so hand code stays small.

**Evidence check:** widgets/mod.rs has 29 pub struct and 224 'Option<': verified. ThemeMode has defaults, text_scale and 26 widget fields (mod.rs:106-135): verified. Theme fields (mod.rs:260-300): verified. gen_merge.rs:40-49 'Some overlays': verified. Resolved = variant + icon fields (resolved.rs:258-263): verified. documented_sizes.rs:105-120 builds reader constants as a sparse ThemeMode: verified.

**Cost check:** None proposed; correct.

**Decision:** keep — the critic's verdict stands; not put to the maintainer (plan, M1 Step 4).
**Status:** closed

### M1-T3-03 · no API break · Generate both TOML registries from the derive's inventory

**Current design:** `docs/property-registry.toml:1-11` says "A Rust test should verify every field in this file exists in the corresponding struct"; no `.rs` file references it (grep over native-theme and connectors: 0 hits). It is kept by hand in lockstep (both it and widgets/mod.rs last changed in 3ee896c3, 2026-09-30; 22 commits versus 58). `docs/inheritance-rules.toml:1-20`: the border and font sections are generated from `#[theme_inherit]` and byte-compared (resolve/inheritance.rs:451-525); `[uniform]` is checked by key presence only, against a third hand-kept list of 67 rule paths inside the test (inheritance.rs:661-720). The derive already registers field names through `inventory` (resolve/mod.rs:23-45) and reads `inherit_from` (gen_inherit.rs:47) but emits no registry with the target or the field type.

**Problem:** Three hand copies of the field list (struct, registry, test list) and of the uniform rules, one unchecked and one checked by presence only (docs/todo.md:1311: the target is never compared). A new field costs four edits. My parse of the registry against the structs finds zero drift today across 27 widget tables, which shows the maintainer pays the cost on every change rather than that the design prevents drift. The archived api-2 review recommended registry-driven codegen (v0.5.7_native-theme-api-2.md:674-680, :819); the derive route was taken instead and the registries were left as prose.

**Evidence:** property-registry.toml:5-8; inheritance-rules.toml:8-19; inheritance.rs:396-399 (`include_str!` of the TOML), :451-525, :661-720 (67 entries); resolve/mod.rs:23-45; gen_inherit.rs:1-47; todo.md:1307-1314; git log counts above; Python comparison of registry keys with struct fields: 0 missing either way.

**Alternatives:** (1) *keep it*: Hand-kept registries, two sections machine-checked.; (2) *A: macro-emitted registries, generated TOML*: The derive emits field type names and `inherit_from` targets into inventory; one test regenerates both TOML files from inventory and diffs them (as border/font already do), printing the new text on mismatch. The TOMLs stay as the review surface; the hand sections of inheritance-rules.toml ([defaults_internal], [per_platform], [text_scale_inheritance]) remain prose about inheritance.rs.; (3) *B: registry-driven codegen*: The archived option D: generate the structs from property-registry.toml through native-theme-build. Reverses the derive decision; heavy.; (4) *C: delete the TOMLs*: Docs point at the structs. Loses the platform-facts traceability column and the review surface.

**Recommendation:** A. inheritance-rules.toml:17-19 already declares the macro authoritative; extending the byte-compare to the whole of both files closes the registry header's own promise and the todo's target gap in about 150 lines, with no new mechanism.

**Cost:** native-theme-derive gen_structs.rs/gen_inherit.rs: ~60 lines (type-name string, inherit target); resolve/mod.rs: ~20 (two registry structs); one generator test ~80 lines; both TOML files regenerated once (a generated-file header). No new dependency; registries stay `pub(crate)`.

**Unblocks:** M2's new fields (Tahoe, two Material presets) cost one edit; M3's model review reads one source; the uniform-target audit in docs/todo.md:1311.

**Overlap with docs/todo.md:** docs/todo.md:1307-1314 (add a target-level assertion for `[uniform]`): the proposal adds generation of the whole registry and the field types, so the assertion becomes a diff. Archived v0.5.7_native-theme-api-2.md B1/B2 wanted a single source; this takes the macro as that source.

**Verdict:** ADOPT WITH CHANGES — The core claim, that hand registries drift unchecked, is stronger than the reviewer found: there is real drift today. The cheaper check-not-generate fix keeps the review surface the archived rationale wanted and closes the existing todo entry.

**Case for the current design:** The registries are a review surface keyed to platform-facts sections. inheritance-rules.toml:13-19 keeps its arrays for review, lint discovery and traceability, and makes the macro authoritative only for border and font. property-registry.toml carries 174 comment lines (section-to-§ mapping, '# defaults only', shorthand notes) that a generator would drop. It is also load-bearing: scripts/check_showcase_parity.py:306, :393-415 reads it as the list of leaf paths.

**Evidence check:** 'No .rs file references it': technically true for src/, but the claim implies the file is only prose, which is wrong: check_showcase_parity.py consumes it, and showcase .rs comments cite it. 'Zero drift across 27 widget tables': wrong. [defaults] lists text_scaling_factor, reduce_motion, high_contrast and reduce_transparency (property-registry.toml:124-127). ThemeDefaults has none of these; they live on AccessibilityPreferences (lib.rs:252). The note about defaults.spacing is stale too (defaults.rs has no 'spacing'). Registry names are serde TOML keys (_px, size_pt/size_px, padding shorthands), not struct field names. git counts 22 vs 58 and last change 3ee896c3 2026-09-30: verified. Byte-compare tests (inheritance.rs:440-525) and include_str (:396-399): verified. The '67-entry list at :661-720' is wrong: the [uniform] test spans five lists at :675-798, about 86 paths. todo.md:1307-1314 (targets never compared): verified. WidgetFieldInfo and FieldInfo inventory with serialized TOML names (resolve/mod.rs:22-45): verified.

**Cost check:** Underestimated. Regenerating property-registry.toml whole needs the macro to emit TOML key expansions (FontSize gives size_pt and size_px; border shorthands are serde-only) and scalar type names (Rgba is 'color'). It also needs a way to keep the 174 comment lines and the § grouping, so ~60 macro lines is too few. check_showcase_parity.py and the platform-fact and add-preset skills read the file and would need checking against any format change.

**Changes required:** 1) Do not regenerate property-registry.toml. Add one bidirectional test: its widget and _structures key sets equal the WidgetFieldInfo/FieldInfo inventory that already exists (resolve/mod.rs:22-45). Types stay hand-kept. This needs no macro change, and the comments stay. 2) Make the test land with the fix it immediately forces: remove the four stale accessibility keys and the stale spacing note from [defaults]. 3) For inheritance-rules.toml, do only what todo.md:1311 asks: emit each inherit_from target into inventory and compare [uniform] targets, replacing the hand-kept lists at inheritance.rs:675-798. 4) Cite the todo entry as the item, not as new work.

**Decision:** adopt the critic's check-not-generate form (round 2): a bidirectional key-set test of property-registry.toml against the `WidgetFieldInfo`/`FieldInfo` inventory, the four stale `[defaults]` keys and the spacing note removed, `inherit_from` targets emitted and `[uniform]` targets compared (docs/todo.md:1307-1314).
**Status:** M1 fix phase

### M1-T4-01 · api-break · The -live tier changes no value: remove it

**Current design:** README.md:32-44 says live presets omit reader-provided fields and the reader merges onto them. `run_pipeline` (pipeline.rs:34-51) loads the live and the full preset, then `merged = full.clone(); merged.merge(&live); merged.merge(&reader)`; lib.rs:559-572 repeats it; the inactive variant comes from the full preset (pipeline.rs:103-110). Merge is "Some overlays" (gen_merge.rs:40-49). Parsing all four pairs: every key in a -live file exists in its regular preset with the same value (kde 203 of 424 keys, macos 176 of 375, windows 224 of 433, adwaita 218 of 398; 0 live-only keys, 0 differing values); the gate enforces equal sizes (documented_sizes.rs:675-718). 85147c2e (2026-03-27) made the full preset the base because the live one "would fail validation" for the inactive variant.

**Problem:** `merged.merge(&live_preset)` cannot change any field, so 1737 lines of TOML (wc: 439+379+447+472) are kept by hand under a rule (README:34-35) with no observable effect; a drift entry tracks it (docs/todo.md:2579-2580); the tier leaks into `PlatformPreset::is_live`/`live_name()` (pipeline.rs:330-347), `select_reader`'s names (pipeline.rs:631-666), PRESET_INFOS (presets.rs:54-64, :96-102), tests (presets.rs:790-795, :907-930) and connector comments (gpui contract.rs:6, iced contract.rs:6, egui contract.rs:87). The reader's fallback (a `None` from the OS keeps the full preset's value) is identical without the file. Deriving the file mechanically, as the question asks, would keep a no-op in sync.

**Evidence:** pipeline.rs:34-51, :103-110; lib.rs:559-572; gen_merge.rs:40-49; Python key/value comparison of the four pairs (tomllib, flattened keys); documented_sizes.rs:675-718; git show 85147c2e; docs/archive/v0.5.0_inheritance-rules.md:288-296 (the original sparse-TOML intent the commit superseded).

**Alternatives:** (1) *keep it*: Four twin files, the README rule, the todo drift entry.; (2) *A: delete the tier*: Remove the four -live files, their PRESET_INFOS entries, `is_live`/`live_name()`, the `-live` names in `select_reader`; readers merge over the platform preset; README "Two-tier system" becomes one paragraph.; (3) *B: generate the twins*: A test asserting live == regular minus a reader-field list; keeps a file that does nothing.; (4) *C: live-only base (OS-first)*: The v0.5.0 intent: sparse platform TOML plus reader. Single-variant readers (KDE, Windows) then lack the inactive variant's colours, the reason 85147c2e added the full preset.

**Recommendation:** A. The evidence is mechanical and the tier's rationale was overtaken in March; nothing reads it. The fallback semantics the README describes already come from the full preset and the reader's `None`s.

**Cost:** Delete 4 TOML files (1737 lines); presets.rs:54-64, :96-102; pipeline.rs:34-51, :330-347, :616-666 (~60 lines); lib.rs:559-572 (~15); documented_sizes.rs:85-120 and :675-718 (~70); README.md:6-44, :70-89; presets.rs tests :790-795, :907-930; connector comments; showcase-gpui info/icons.rs:280. No dependency change.

**Unblocks:** docs/todo.md:2579-2580 becomes moot and the preset half of :2594-2600 goes (the GNOME reader deriving sizes remains); M2's new presets (macos-tahoe, material-3, -expressive) need no twin, halving M2's preset checking; M3's pipeline review reads one merge order.

**Overlap with docs/todo.md:** docs/todo.md:2579-2580 "Live presets": the proposal shows the entry's premise has no effect. docs/todo.md:2594-2600 adwaita-live sizes: the reader half stays. docs/todo.md:53 and :2262 note the twins carry geometry only. D3 lists "-live bases" among M2's objects; removing them before M2 reduces D3's scope without contradicting its purpose. Breaking: `Theme::preset("kde-breeze-live")` would return UnknownPreset and `PlatformPreset::is_live`, `live_name()` (lib.rs:236 re-export) go, so D1 applies.

**Verdict:** ADOPT WITH CHANGES — The mechanical evidence is airtight: merging the live preset onto its full twin cannot change a single value. But D4 explicitly names a -live base, and the change is breaking, so it needs the maintainer's amendment rather than a silent adopt.

**Case for the current design:** The README (:32-44) and v0.5.0_inheritance-rules.md:279-296 designed the live presets as an OS-first sparse base, recording which fields the reader owns. 4195245a (2026-03-28) kept a 'live preset overrides geometry' layer between the full preset and the reader, in case the live geometry differs. D3 lists the '-live bases' among M2's objects, and D4 has the maintainer ordering macos-tahoe 'and its -live base'.

**Evidence check:** Python/tomllib comparison of the four pairs: verified exactly (kde 203/424, macos 176/375, windows 224/433, adwaita 218/398; 0 live-only keys, 0 differing values). wc of 439+379+447+472 = 1737: verified. pipeline.rs:34-51 merge order and :103-118 (inactive variant from the full preset): verified. lib.rs:559-572 repeats it: verified. The gate enforces equal sizes (documented_sizes.rs:670-718): verified. PlatformPreset::is_live/live_name (pipeline.rs:330-347): verified. PRESET_INFOS live entries (presets.rs:52-65): verified. Tests at presets.rs:790-795 and :907-930: verified. History is partly wrong: 85147c2e made the full preset the base of the inactive variant only; the active variant's full-preset base came in 4195245a.

**Cost check:** Mostly right. Missed: SystemTheme.live_preset (pub(crate), lib.rs:463, :618 and test fixtures :750-836); .claude/skills/add-preset/SKILL.md; a CHANGELOG line; keeping the README's 'Reader-provided fields' list as reader documentation. The break also forces 0.7.0 under D1 (the public is_live field, live_name(), and the '-live' names accepted by Theme::preset) unless it is bundled with another accepted break. No dependency.

**Changes required:** 1) Present it at stop point A as an amendment to D4 (no macos-tahoe-live) and to D3's object list. The maintainer approves it explicitly, and the D1 version consequence is stated. 2) If approved, land it before M2a Steps 8-12 so no new twin is built. 3) Correct the history to 4195245a for the active-variant base. 4) Keep the 'Reader-provided fields' README section as documentation of the readers, add the missing cost items above, and close docs/todo.md:2579-2580 and the preset half of :2594-2600.

**Decision:** adopt (round 2): remove the `-live` tier before M2a Steps 8–12; D3's object list and D4 amended (no `macos-tahoe-live`); the README's reader-provided-fields list kept as reader documentation; docs/todo.md:2579-2580 and the preset half of :2594-2600 closed. Breaking change accepted: under D1 the release is 0.7.0 and under D0 the branch becomes `v0.7.0-rc1`.
**Status:** M1 fix phase

### M1-T5-01 · no API break · Machine-readable facts for Chapter 2, keyed by platform and OS version

**Current design:** docs/platform-facts.md (1842 lines): Chapter 1 (:46-856, 435 table rows) is prose per OS; Chapter 2 (:857-1647) has 356 property rows with one free-text cell per platform carrying a value, a source tag (:7-29) and a symbol (:31-45). OS versions sit in a few cells, not on rows (grep: Sonoma 4, Tahoe 4, Sequoia 3, "Windows 11" 1, "libadwaita 1.7" 1); section headings name no version (:51, :274, :548, :719). Consumers: documented_sizes.rs copies 95 padding rows and 90 field rows by hand with a line number (:185-, :823-) and checks the cited line is the right row, not that its number matches (:635-672); platform_facts_xref.rs checks substrings (`contains("Noto Sans")`, :67-74); the preset-validator agent reads the three documents. check_widget_coverage.py is about showcases, not facts (:1-6).

**Problem:** A new OS version is a prose change. The plan's fallback, Tahoe beside Sonoma in one cell, makes the cell per-context, which the unstated-sizes spec reads as "not stated" unless a ruling names the context (archive spec :67-80), while the gate keys rows by `Platform::Macos` and one row per (platform, widget) (documented_sizes.rs:57-63, :618-630), so it can hold one of the two values. Inserting a row shifts up to 872 `platform-facts.md:N` citations (src and docs, excluding archive; 21 in todo.md). The gate covers about 185 of 1780 Chapter 2 cells; every other native value is unchecked mechanically (todo.md:2343-2350 says so).

**Evidence:** platform-facts.md counts above (grep, awk over :857-1647: 356 data rows); documented_sizes.rs:53, :185-348 (95 rows), :823-925 (90 rows), :635-672; platform_facts_xref.rs:22, :67-74; .claude/agents/preset-validator.md:11-16; git: 50 commits to platform-facts.md, 25 to documented_sizes.rs; docs/todo.md:2343-2350; docs/archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md:67-80 (cell-reading rules, the de-facto schema).

**Alternatives:** (1) *keep it*: Markdown cells, Tahoe beside Sonoma, hand-copied gate rows with line numbers.; (2) *A: facts TOML, Chapter 2 generated, gates by key*: `docs/platform-facts/<platform>.toml` with `[[fact]] property, platform, os_version, value|none|range|derived|pointer, qualifier, source, confidence, note` (the spec's cell rules as fields); a script renders Chapter 2; one generic test replaces the hand tables: every native preset field with a fact for its (platform, version) equals it and every fact value is stated. Migrate section by section as M2 touches it, macOS first; unmigrated sections keep their cells.; (3) *B: version columns in the Markdown*: macOS 14 | macOS 26 columns: keeps line coupling, doubles the width, still hand-copied into the gate.; (4) *C: generate native presets from the facts*: A preset as a projection of the facts for (platform, version) plus rulings for (none) cells; only sensible after A covers every preset field, since colours and rulings have no row yet.

**Recommendation:** A, tied to M2's path: M2a migrates the macOS cells it versions and the gate gains a (platform, version) key; D4's `macos-tahoe` and D6's two Material presets then differ by key, not by cell convention. Checking stays; generation (C) waits until a preset is a pure projection. It is the todo.md:2343 item with a version axis and a schedule.

**Cost:** A schema and a render script (~200 lines Python, stdlib tomllib; check_release.sh:674-679 already requires 3.11); the gate's two hand tables (~600 of documented_sizes.rs's 1253 lines) become a loop; migration of 356 rows × 5 cells done per section as M2 verifies them; citations re-pointed to keys over time, archive untouched. No Rust dependency.

**Unblocks:** D4 and D6 get a version key; Review Focus 6 ("the preset's documentation says which version it is") becomes data; the preset-validator agent's check becomes a test; todo.md:325, :345, :371, :533, :545, :657 become rows with no value instead of prose gaps.

**Overlap with docs/todo.md:** docs/todo.md:2343-2350 "A machine-readable platform-facts" (already there: structured facts, every native value gated, tables generated). The proposal adds the OS-version axis, the per-section migration tied to M2a, and the (platform, version) key in the gate; it also records that the same-cell fallback and the one-number cell rule conflict.

**Verdict:** ADOPT WITH CHANGES — The conflict between a Tahoe value in the same cell and the one-number cell rule and per-platform gate key is real and must be settled before M2a builds macos-tahoe. The full facts-TOML migration duplicates a tracked todo entry and costs far more than estimated, so only the minimal version key and ruling go in now.

**Case for the current design:** platform-facts.md is written for a human to read: sources, qualifiers, derivations and pointers live in prose cells, and the unstated-sizes spec (archive spec :67-80) defines how to read seven kinds of cell. The theme-contracts rationale (§2.3, :201-222) automated only what it could justify. The gate copies values by hand with a line citation and checks the citation lands on the right row (documented_sizes.rs:630-672). docs/todo.md:2343-2350 already records the structured-facts idea as deferred.

**Evidence check:** 1842 lines and chapter bounds :46/:857/:1648: verified. 356 Chapter 2 data rows (:857-1647): verified. Chapter 1 435 rows: verified. Version mentions Sonoma 4, Tahoe 4, Sequoia 3, 'Windows 11' 1, 'libadwaita 1.7' 1: verified. Gate keyed by Platform with Macos mapped to macos-sonoma (documented_sizes.rs:57-81): verified. '95 padding rows': wrong, there are 90 row() entries at :185-371; 90 field() entries at :823+: verified. The line check tests the row, not the value (:630-672): verified. xref uses substring contains (platform_facts_xref.rs:67-74): verified. check_widget_coverage.py is about showcases (:1-6): verified. 50 and 25 commits: verified. check_release.sh requires Python 3.11 (:672-680): verified. 872 citations: wrong, grep finds 776 'platform-facts.md:N' outside archive and target; 21 in todo.md: verified. The per-context rule matches spec :67-80: verified.

**Cost check:** Badly underestimated. A schema for seven cell kinds (number with qualifier, range, per-context, derivation, pointer, none, unmodelled structure), migrating up to 1780 cells, re-pointing 776 citations, and a Chapter 2 that is half generated and half hand-written while M2 edits it is a design project. Under the plan's Architecture rule that needs its own rationale, spec and plan, not ~200 lines of Python. What M2 needs now is much smaller: preset-keyed gate rows and a version rule for cells.

**Changes required:** 1) Now, before M2a Step 8: add a ruling to platform-facts.md Conventions that a version-tagged value in a cell (e.g. 'macOS 14: X · macOS 26: Y') belongs to the preset of that version, not to a usage context, so the one-number rule reads per preset. 2) Make the gate's Platform enum preset-keyed (MacosSonoma, MacosTahoe, Material3, Material3Expressive) with hand rows per preset. 3) Do not migrate to a facts TOML in M1 or M2. Append the OS-version axis and these findings to the existing docs/todo.md:2343-2350 entry, as its own rationale, spec and plan after M2. 4) Correct the counts to 90 padding rows and 776 citations.

**Decision:** adopt the critic's minimal form (round 3): a version ruling in platform-facts.md Conventions (a version-tagged value belongs to that version's preset) and a preset-keyed gate in documented_sizes.rs, both before M2a Step 8; the facts-TOML migration stays on docs/todo.md:2343 with the OS-version axis and these findings appended, for its own rationale/spec/plan after M2.
**Status:** M1 fix phase

*Reviewer D-B notes:* Verified counts: live preset lines 439+379+447+472 = 1737 (wc); the key/value comparison of each -live file against its regular preset used Python tomllib over flattened keys (comments ignored); property-registry versus struct comparison used a regex over widgets/mod.rs with serde renames applied, widget tables only (the defaults, text_scale and theme_variant tables were not compared). The xrdb timing is a Python subprocess spawn on the maintainer's machine, 11 runs, not a trace of resolve itself; the 1 ms poll explanation of the 3.4 and 5.2 ms jumps is inferred from detect.rs:179-229 and the ledger, not measured with strace. Context-map corrections: `ResolutionContext::from_system` is at resolve/context.rs:77; `ThemeMode` at model/mod.rs:107; a second full+live merge site exists at lib.rs:559-572; scripts/check_widget_coverage.py checks showcase widget coverage, not platform facts. T4's recommendation is breaking (preset names and two public items), so it needs D1 triage; a question for the maintainer: does any external consumer call `Theme::preset("…-live")` or read `PlatformPreset::is_live`? No cargo was run; no files were edited.

*Critic D-B notes:* I checked the proposals read-only, with greps, wc, git log and two throwaway Python comparisons in the scratchpad. I did not re-run the xrdb timing. Two corrections matter for triage. First, property-registry.toml already disagrees with the code: [defaults] still lists text_scaling_factor, reduce_motion, high_contrast and reduce_transparency (lines 124-127), which moved to AccessibilityPreferences, so the reviewer's 'zero drift' is wrong. Second, the -live presets do nothing, verified exactly, but D4 explicitly orders a macos-tahoe -live base and removing the tier is a breaking change (D1), so proposal 3 needs the maintainer to amend D4 at stop point A. Proposal 4's facts-TOML is the existing docs/todo.md:2343 entry. Only the version ruling and the preset-keyed gate are needed before M2a.

### M1-T6-01 · no API break · ReaderOutput::Dual carries no active mode; macOS SystemTheme::mode is Light

**Current design:** `ReaderOutput` is `Single { mode, is_dark } | Dual { light, dark }` (native-theme/src/lib.rs:337-355). `from_system_inner` maps `Dual` to `ColorMode::Light` unconditionally (pipeline.rs:737-746) and `run_pipeline` stores that as `SystemTheme::mode` (pipeline.rs:139-141). The macOS reader resolves both appearances by name (macos.rs:488-491) and returns `Dual` (macos.rs:599-602); nothing in macos.rs asks which appearance is active (no `is_dark`/`AppleInterfaceStyle` hit), although detect.rs:545-560 already reads it.

**Problem:** On a Mac in dark mode `SystemTheme::from_system()` reports `mode: Light`, and all three connectors pick the variant by it (gpui lib.rs:319, iced lib.rs:332, egui ext.rs:238), so the application starts light. The archived decision chose a struct with `is_dark` for every reader to remove exactly this inference (docs/archive/v0.5.7_native-theme-api-2.md:915-925); the enum shipped is its option E and dropped the flag for `Dual`. The one `mode` assertion covers a `Single` reader (pipeline.rs:1208); CI screenshots pass `--variant dark` to the showcase (.github/workflows/screenshots.yml:142-150), which sets the app's own mode and hides the defect.

**Evidence:** pipeline.rs:745 `ReaderOutput::Dual { .. } => crate::ColorMode::Light`; macos.rs has no dark-mode read (grep); the arm dates from a3cb768d/66423ae7 (git log -S). Connector call sites above. No macOS test asserts `from_system().mode`.

**Alternatives:** (1) *keep it*: macOS apps start light whenever the OS is dark; each app would have to call `detect::system_is_dark()` itself.; (2) *is_dark on Dual*: Add `is_dark: bool` to `Dual`; the macOS reader fills it from `detect::detect_is_dark()` (detect.rs:166, 545-560); the pipeline derives `mode` from the flag for both variants and the Light arm goes.; (3) *archived F struct*: `ReaderOutput { active, inactive: Option<ThemeMode>, is_dark }` as api-2.md:915-925 recommended; same information, replaces the enum and its 8 match sites.; (4) *pipeline detects*: For `Dual` the pipeline calls `system_is_dark()`; keeps the reader ignorant of the mode and adds a second OS read.

**Recommendation:** `is_dark` on `Dual` (or the F struct; same data, more churn). The reader that read both appearances is the place that knows which is current, and the pipeline then has one rule for every reader. Add a macOS-leg test that `from_system().mode == detect_is_dark()`. D4's version-aware macOS reader touches the same function and can land together.

**Cost:** lib.rs enum + `to_theme` (~5 lines); pipeline.rs:737-746 (−8); macos.rs:599 (+3); `Dual` constructions and match arms in tests and readers (lib.rs:740, 783, 826, 930; pipeline.rs:1032, 1225; gnome/kde/windows `Dual { light, .. }` arms) ~10 sites. ~40 lines, no dependency.

**Unblocks:** Correct dark-mode start on macOS in every connector; D4 (preset by macOS version) in the same reader change.

**Overlap with docs/todo.md:** none (grep for `ColorMode::Light`, `Dual`, `SystemTheme::mode`, macOS+mode in docs/todo.md finds no entry).

**Verdict:** ADOPT — A real user-visible bug: a dark Mac starts light in all three connectors and gets the light icon theme. The fix is small, additive inside the crate (ReaderOutput is pub(crate)) and restores the archived intent. No D0-D7 conflict, and it fits D4's reader change.

**Case for the current design:** Nothing argues for it. The enum is api-2.md option E, chosen over the recommended F (api-2.md:915-925), which carried is_dark for every reader. The Light arm came in with a3cb768d, and it carried over the bug it replaced: the old `reader_theme.dark.is_some() && light.is_none()` heuristic also gave Light for macOS, which fills both variants. That is exactly the fragility api-2.md:891-893 names. The only possible defence is that Dual readers 'know both', but SystemTheme::mode (lib.rs:452, 'The OS color mode preference') and the icon_theme pick at pipeline.rs:95 both need the active one.

**Evidence check:** pipeline.rs:745 Dual => ColorMode::Light: verified. macos.rs has no is_dark/AppleInterfaceStyle read: verified (grep), and detect.rs:545-560 reads AppleInterfaceStyle through NSUserDefaults: verified. Connector picks by sys.mode at gpui lib.rs:319, iced lib.rs:332, egui ext.rs:238: verified. pipeline.rs:1208 asserts mode for a Single reader only: verified. screenshots.yml:142-150 passes --variant: verified. No macOS test of from_system().mode: verified (grep of native-theme/tests and macos.rs). Origin a3cb768d/66423ae7: verified with git log -S. api-2.md:915-925 recommends F with is_dark: verified.

**Cost check:** About right. It misses three things. Two exhaustive patterns, lib.rs:374 and gnome/mod.rs:537 (`Dual { light, dark }`), need `..`. The pipeline.rs:18 and lib.rs:330-335 docs need updating. The new test runs only on the macOS CI leg. The `Dual { light, .. }` arms in kde/windows/gnome are unaffected. The reader should call the uncached detect_is_dark(), not system_is_dark(), so a re-read after a watch event is fresh.

**Decision:** adopt (round 3): `is_dark` on `ReaderOutput::Dual`, filled by the macOS reader from the uncached `detect_is_dark()`, the pipeline deriving `mode` from it for both variants; the two exhaustive patterns and the docs updated; a macOS-leg test.
**Status:** M1 fix phase

### M1-T6-02 · no API break · Sync/async split: keep it; drive both sync entry points with pollster

**Current design:** One async implementation, `from_system_inner` (pipeline.rs:734). On Linux `from_system` is `pollster::block_on` (lib.rs:671-673); on macOS/Windows a hand-rolled noop-waker single poll that answers `PlatformUnsupported { platform: "unexpected async suspension" }` if the future is `Pending` (lib.rs:676-688), duplicated in `AccessibilityPreferences::from_system` (lib.rs:294-305). `from_system_async` is `from_system_inner().await` everywhere (lib.rs:700-703). `pollster` is a Linux-only dependency (Cargo.toml:114) and a dev-dependency (:132). Readers are `async_trait` objects behind `Box<dyn ThemeReader>` (reader.rs:43-47, pipeline.rs:626).

**Problem:** The split serves the goal: one line in a sync app, a non-blocking future on Linux where the portal is awaited. Two things do not: the second executor (24 lines) exists only because pollster is Linux-only, and its impossible-today error would misreport a future `.await` in the macOS or Windows reader as an unsupported platform. The archived decision G chose `pollster::block_on` for the sync path on every target (docs/archive/v0.5.7_native-theme-api.md:557-636). Dependency weight is not an argument for a redesign: `portal` adds 77 crates (111 vs 34 unique, `cargo tree --offline`), 17 of them async-runtime crates, but zbus, which `watch` needs anyway, pulls async-io/async-executor itself (`cargo tree -i async-io`), so replacing ashpd with `zbus::blocking` would not remove them.

**Evidence:** lib.rs:676-688 and :294-305 (two pollers); Cargo.toml:114, :132; `cargo tree --offline -p native-theme --features portal -e normal` 111 crates vs 34 without; `-i async-executor` shows zbus as the puller. Ledger: `from_system` 9 ms cold, 4-8 ms warm.

**Alternatives:** (1) *keep it*: Two executors, one with a misleading error path, stay.; (2) *pollster everywhere*: Move Cargo.toml:114 to `[dependencies]`; both non-Linux `from_system`s become `pollster::block_on(..)`; delete the noop pollers and the error arm.; (3) *drop from_system_async*: Archived option B; loses the non-blocking Linux path async apps use today.

**Recommendation:** Keep the split; pollster everywhere. It is the archived decision, applied uniformly, and removes the only code that can lie about why a read failed.

**Cost:** Cargo.toml: 1 line moved; lib.rs: ~24 lines deleted, 2 added; +1 crate (pollster, no transitive deps) on macOS/Windows builds.

**Unblocks:** Readers may gain `.await` points (an async Windows `UISettings` read, a future portal call on another platform) without the sync path silently breaking.

**Overlap with docs/todo.md:** none.

**Verdict:** ADOPT — Applies the archived decision uniformly, removes a misleading error path and duplicated code, and costs one zero-dependency crate off Linux. The dependency-count numbers are off, but they are not load-bearing.

**Case for the current design:** The noop-waker poll avoids a dependency on macOS/Windows and is correct today, since those readers have no .await points (lib.rs:676-678 doc). Decision G's text arguably concerned only the Linux portal path, where pollster was introduced (CHANGELOG.md:328).

**Evidence check:** Two noop pollers at lib.rs:676-688 and :294-305: verified. The 'unexpected async suspension' PlatformUnsupported arm: verified, and the accessibility poller silently returns defaults on Pending. pollster is Linux-only at Cargo.toml:114 plus a dev-dep at :132: verified. Archived G says 'sync from_system() is pollster::block_on(from_system_async_inner())' with no platform restriction (api.md:557-560): verified. Crate counts: wrong. `cargo tree --offline -p native-theme --features portal -e normal --prefix none | sort -u` gives 111 lines only because '(*)' duplicates are counted; with them stripped it is 89 unique crates against 29 without portal (+60, not +77). zbus as the async-executor puller: verified (cargo tree -i). Ledger 9 ms cold: verified (9.12-9.66).

**Cost check:** About right. The two cfg-split pairs of functions also collapse to one each, so their doc comments merge. pipeline.rs:725-727 needs a doc update. pollster has no transitive deps. A future Pending would then block rather than error, the same behaviour as on Linux.

**Decision:** adopt (round 3): pollster on every target; the two noop pollers and the 'unexpected async suspension' arm deleted.
**Status:** M1 fix phase

### M1-T6-03 · api-break · Accessibility flags: text scale belongs in resolution; the others need named receivers

**Current design:** `AccessibilityPreferences` lives on `SystemTheme` (lib.rs:247-266, :493) and is deliberately absent from `ResolutionContext` (resolve/context.rs:20-23; docs/archive/v0.5.7_gaps.md:393-396: "render-time concern"). Text scale: each connector carries its own `text_scale_factor` + `scaled_text_size` (egui accessors.rs:25-33, gpui lib.rs:463-473, iced lib.rs:1050-1059). Reduced motion: egui zeroes `animation_time` (style/mod.rs:33), gpui forwards into `App::reduce_motion` (lib.rs:846-857), iced only in its spinner (spinner.rs:131-175); gpui and iced icon docs send callers to `detect::prefers_reduced_motion()` (gpui icons.rs:1159, 1219; iced icons.rs:149, 206). High contrast is applied by no connector (egui atlas.rs:566; todo:1108); reduced transparency only by gpui's modal scrim (atlas.rs:567-568).

**Problem:** For "looks native with minimal application code" the four flags have no owner. A GNOME user at `text-scaling-factor` 1.5 (docs/platform-facts.md:850) gets scaled text only where a connector or the app remembered `scaled_text_size`; an iced app gets reduced motion for the spinner alone; a Windows high-contrast user gets the normal palette in all three toolkits. The archived premise holds for motion and transparency (they need a toolkit receiver) but not for text scale: it is a multiplier on font sizes, the same kind of input as `font_dpi`, which is resolve-time (context.rs:39-51) and applied at one point, `validate_with_dpi(ctx.font_dpi)` (resolve/mod.rs:236).

**Evidence:** Three identical guards (files above); grep counts: text_scale uses egui 8 / gpui 21 / iced 8 lines; reduce_motion iced 2 files, high_contrast applied in 0 files. Ledger: the preset path pays 8-12 ms to read the block (see the cache proposal). platform-facts.md:1123 records text scaling as an OS setting on Windows, GNOME, limited on macOS, none on KDE.

**Alternatives:** (1) *keep it*: Each connector and application applies text scale itself; three copies of the guard stay.; (2) *core method only*: `AccessibilityPreferences::text_scale_factor()` and `scaled(size)` in core (todo:2087); application stays per connector.; (3) *text scale in resolution*: `ResolutionContext` gains `text_scale: f32` (sanitised), applied beside `font_dpi` at resolve/mod.rs:236; `ResolvedTheme` font sizes arrive scaled; the preset path reads the factor from the detection cache. Motion and transparency stay connector forwards, each connector documenting its receiver; high contrast decided in M2 (Windows high-contrast palette, GNOME HighContrast) as applied-or-carried.; (4) *model motion*: A `MotionSpec` on the theme (todo:1639); separate, larger.

**Recommendation:** Text scale in resolution plus the core method; this contradicts gaps.md §G7 for one field, and the premise is false for that field (a font-size multiplier is not render-time state). Keep the other three as connector forwards with a receiver table per connector; send high contrast to M2.

**Cost:** context.rs +1 field and `from_system`/`for_tests` (~10 lines); resolve/mod.rs one multiplication (~5); pipeline.rs passes the reader's factor (~3); connectors delete or thin 3 × ~12 lines; tests for 2× and 0.5×. No dependency.

**Unblocks:** Every toolkit and application gets scaled text with no call; review focus 4 (2× text) becomes a core test instead of three connector tests.

**Overlap with docs/todo.md:** todo:2087 (one method in core; this goes on to apply it at resolve); todo:1173 (iced has no receiver for motion/transparency, confirmed); todo:1108 (gpui high contrast unapplied; answered by the M2 decision); todo:1639 (motion model, not taken here).

**Verdict:** ADOPT WITH CHANGES — The premise 'a multiplier like font_dpi' is exactly why the move is risky: on KDE, and possibly on GNOME, the scale already arrives through font_dpi, and the reviewer misread platform-facts for KDE. The core-method half is cheap and already wanted (todo:2087).

**Case for the current design:** The archived G7 / J.2-B4 decision (v0.5.7_gaps.md:393-396, context.rs:20-23) keeps ResolvedTheme as the platform's stated values and accessibility as render-time. The connectors already apply the factor to what they draw (gpui to_theme and geometry builders, egui and iced font_size accessors), and each exposes scaled_text_size publicly for the rest. More importantly, on KDE the text scale already is font_dpi: platform-facts.md:1123 gives KDE's text_scaling_factor as `forceFontDPI / 96`, which ResolutionContext.font_dpi already applies. On GNOME the reader's font_dpi comes from xrdb Xft.dpi (gnome/mod.rs:140-153), which GNOME's xsettings daemon may already derive from text-scaling-factor (not verified here). If it does, a resolve-time multiply double-scales, and today's connector scaling may already do so.

**Evidence check:** Three identical finite-positive guards (egui accessors.rs:31-34, gpui lib.rs:471-474, iced lib.rs:1056-1059): verified. Grep counts egui 8 / gpui 21 / iced 8: wrong ('text_scale' gives 45 / 18 / 7 lines; the reviewer's pattern is unknown). egui animation_time at style/mod.rs:33: verified. gpui forward_reduce_motion at lib.rs:846-857: verified. iced reduce_motion only in spinner.rs: verified. high_contrast applied by no connector (only the is_high_contrast accessors): verified. platform-facts.md:850 GNOME text-scaling-factor: verified. platform-facts.md:1123 'none on KDE': wrong, the row says KDE `forceFontDPI / 96`. todo:2087 (one core method), :1108, :1173 and :1639 exist: verified.

**Cost check:** Underestimated. It is a public API break in core semantics and in three connectors' public scaled_text_size / font_size behaviour, which falls under D1 and makes the release 0.7.0. The preset path needs a cheap factor source, which does not exist until proposal 4 lands, because ResolutionContext::from_system already runs on every resolve (ledger M3 R2). It also touches with_overlay replay, validate ranges, the showcases' Widget Info size claims and the screenshot gates. The double-scaling question has to be settled first.

**Changes required:** Adopt only the core method now: AccessibilityPreferences::text_scale_factor()/scaled(size) per todo:2087 (an existing entry, so this is not a new finding), with the connectors calling it. Put the motion/transparency receiver table in the connector docs. Send high contrast to M2 as proposed. Do not move text scale into ResolutionContext in M1. Instead add an M2 check: does GNOME's xrdb Xft.dpi already include text-scaling-factor, and does KDE's forceFontDPI already cover it? Revisit at stop point A only if M2 shows the factor is not already in font_dpi.

**Decision:** adopt the critic's form (round 3): only the core method `AccessibilityPreferences::text_scale_factor()` / `scaled()` now (docs/todo.md:2087, with M1-T7-01), connectors calling it; a motion/transparency receiver table in the connector docs; high contrast decided in M2 (M2b Windows, M2e GNOME); M2e checks whether KDE's `forceFontDPI` and GNOME's `Xft.dpi` already carry the text scale before resolve-time scaling is revisited.
**Status:** M1 fix phase (method, docs); M2 questions appended to the plan

### M1-T6-04 · no API break · Reduced motion has two sources that disagree on KDE

**Current design:** `detect::prefers_reduced_motion()` (cached, detect.rs:629-631, 756-761) runs `gsettings get org.gnome.desktop.interface enable-animations` on every Linux desktop (detect.rs:648-658), `NSWorkspace` on macOS (:665-667), `UISettings.AnimationsEnabled` on Windows (:678-686). The readers read it a second way: KDE `[KDE] AnimationDurationFactor == 0` (kde/mod.rs:41-47), GNOME portal then gsettings (gnome/mod.rs:333), macOS `NSWorkspace` (macos.rs:150-157), Windows `SPI_GETCLIENTAREAANIMATION` (windows.rs:395-411). `accessibility_from_system_inner` ORs the reader's value with the detect path (pipeline.rs:800-802); `from_system_inner` does not (:734-748); the consistency test tolerates the difference (lib.rs:905-906).

**Problem:** On KDE `prefers_reduced_motion()` answers from a GNOME schema that may be absent (then `false`) while `sys.accessibility.reduce_motion` answers from kdeglobals. Connector docs send icon callers to the detect path (gpui icons.rs:1159, 1219; iced icons.rs:149, 206), so an app following them animates for a KDE user who set animations to instant while the same app's theme stops them. On Windows the two APIs may or may not reflect one setting; M2b should confirm. `SystemTheme::from_system().accessibility.reduce_motion` and `AccessibilityPreferences::from_system().reduce_motion` can differ on the same machine because only one path ORs.

**Evidence:** detect.rs:648-658 has no desktop dispatch; kde/mod.rs:41-47; pipeline.rs:800-802 vs :734-748; lib.rs:905-906 `assert!(prefs.reduce_motion || !sys.accessibility.reduce_motion)`. todo:1874 fixed one symptom in the gpui showcase.

**Alternatives:** (1) *keep it*: Two answers per platform; docs keep pointing at the weaker one.; (2) *one per-platform read*: A single `read_reduce_motion()` in detect.rs dispatching on the Linux desktop (KDE kdeglobals, GNOME gsettings/portal), used by the readers and by `prefers_reduced_motion()`; drop the OR; connector docs point at `sys.accessibility` or the toolkit switch.; (3) *remove the detect functions*: Delete `prefers_reduced_motion`/`detect_reduced_motion` in favour of `AccessibilityPreferences`; API break, and the cache precedent (`system_is_dark`) argues for keeping them.

**Recommendation:** One per-platform read shared by both paths; the OR in pipeline.rs:800 then goes. Which Windows API is right is M2b's; the design question is that there must be one.

**Cost:** detect.rs ~30 lines; kde/mod.rs factor the parse into a `pub(crate)` fn (~10); pipeline.rs −3; doc lines in gpui and iced icons. No dependency.

**Unblocks:** The receiver table of the accessibility proposal; a `MotionSpec` (todo:1639) would read one source.

**Overlap with docs/todo.md:** todo:1874 (gpui showcase read detect vs `cx.reduce_motion()`, fixed; this removes the root); todo:1337, :1426, :1637 name `AnimationDurationFactor` for a motion model, not this mismatch.

**Verdict:** ADOPT WITH CHANGES — Two inconsistent sources for one flag is a design defect, and fixing it is cheap and non-breaking. The estimate just has to account for the async portal source.

**Case for the current design:** The detect functions mirror the cached/uncached system_is_dark pattern (v0.4.0-animated-icons.md:599-619) and are meant to be cheap and sync. On KDE, Plasma's kde-gtk-config kded module appears to mirror AnimationDurationFactor into the GNOME enable-animations key: both strings occur in /usr/lib/qt6/plugins/kf6/kded/gtkconfig.so on this machine. So the two paths may often agree where kde-gtk-config is installed (unverified from source).

**Evidence check:** detect.rs:648-658 runs gsettings only, with no desktop dispatch: verified. kde/mod.rs:41-47 AnimationDurationFactor==0: verified. GNOME portal then gsettings (gnome/mod.rs:333): verified. Windows SPI_GETCLIENTAREAANIMATION (windows.rs:395-411) against detect's UISettings.AnimationsEnabled (detect.rs:678-686): verified. OR only in accessibility_from_system_inner (pipeline.rs:800-802), not in from_system_inner: verified. lib.rs:905-906 tolerance: verified. The KDE disagreement as a live symptom: unverifiable. On this KDE host gsettings says enable-animations true with AnimationDurationFactor=0.25, which is consistent.

**Cost check:** It misses that GNOME's portal `reduced-motion` value is async (ashpd) while prefers_reduced_motion() is sync. Either the shared read uses pollster::block_on on Linux (already a dependency there) or the portal stays a reader-only input, which reintroduces a second source. detect_linux_desktop() exists and is cached. The KDE path re-reads kdeglobals (cheap). Connector icon docs in gpui and iced also change.

**Changes required:** Make the shared read cover the portal too (pollster::block_on on Linux, same order as the GNOME reader: portal, then gsettings), so there really is one source. Choose the Windows API in M2b before coding. Keep prefers_reduced_motion/detect_reduced_motion as the public face of that one read.

**Decision:** adopt with the critic's changes (round 4): one read per platform shared by the readers and `prefers_reduced_motion()` (KDE kdeglobals; GNOME portal then gsettings via pollster; macOS NSWorkspace; Windows the API M2b chooses), the OR removed; lands in M4's fix phase after M2b's choice.
**Status:** scheduled: M4 (detect and readers)

### M1-T6-05 · no API break · AccessibilityPreferences::from_system re-runs the whole platform reader

**Current design:** `accessibility_from_system_inner` calls `select_reader()` and `reader.read()` and keeps only the accessibility block (pipeline.rs:792-803). The preset path needs it: gpui's quick start calls it before `from_preset` (connectors/native-theme-gpui/src/lib.rs:10-11, :33); the showcases call it on the preset path (egui app.rs:393, :504; gpui app.rs:1692). `DetectionContext` already caches `is_dark`, `reduced_motion` and `icon_theme` with `invalidate_caches` (detect.rs:714-719, :154-156).

**Problem:** The ledger measured it at 8.35-12.15 ms, as much as `SystemTheme::from_system` (9.1-9.7 ms cold, 3.6-8.3 ms warm): a full kdeglobals parse plus portal D-Bus on KDE+portal, for four fields. An app that reads the system theme and then switches to a preset pays the OS read twice; review focus 2 (repeated switching) makes it per switch where the app does not cache (the egui showcase does only when `settings.prefs` is set, app.rs:501-504).

**Evidence:** pipeline.rs:793-797; ledger Timings table rows `SystemTheme::from_system` and `AccessibilityPreferences::from_system`; callers above.

**Alternatives:** (1) *keep it*: Every preset-path application pays a reader run for the block.; (2) *cache in DetectionContext*: `from_system_inner` stores the reader's block in `DetectionContext`; `AccessibilityPreferences::from_system` reads the cache first; `invalidate_caches` clears it (watch consumers call it already, detect.rs:144-150).; (3) *split the reader*: `ThemeReader::read_accessibility()` with cheap per-platform reads (KDE: the `[KDE]` group; macOS: three `NSWorkspace` flags; Windows: `UISettings` + SPI; GNOME: four gsettings keys).

**Recommendation:** The cache: consistent with the existing `DetectionContext`, a few lines, and it makes the preset path free after any OS read. The split reader is the better long-term shape but is M4 code work and unnecessary once the cache exists.

**Cost:** detect.rs ~25 lines (field, getter, invalidate); pipeline.rs ~8. No dependency.

**Unblocks:** Text scale in resolution on the preset path without a reader run (accessibility proposal).

**Overlap with docs/todo.md:** none.

**Verdict:** ADOPT WITH CHANGES — The saving is real (8-12 ms per preset switch). But silently caching a function documented as a fresh read, while watchers miss accessibility changes, would trade a cost for stale accessibility state. The existing cached/uncached pair pattern solves both.

**Case for the current design:** AccessibilityPreferences::from_system is documented as a fresh read ('Read the OS accessibility preferences', lib.rs:271; gpui showcase os_preferences 'read now', app.rs:1685). The watchers do not report most accessibility changes: GNOME misses a11y/text-scaling keys, macOS misses the accessibility flags, Windows misses TextScaleFactorChanged (todo:247-283). A cache would therefore go stale with nothing to call invalidate_caches.

**Evidence check:** pipeline.rs:792-803 runs select_reader + read for the block: verified. Ledger timings 8.35-12.15 ms against from_system 9.12-9.66 cold / 3.63-8.30 warm: verified. Callers gpui lib.rs:10, :33, egui app.rs:393, :504, gpui app.rs:1692: verified. DetectionContext fields (detect.rs:714-719) and invalidate_caches (:154): verified. 'Watch consumers call it already': verified for egui watch.rs:99, the egui/iced/gpui showcases (app.rs:492, showcase-iced.rs:2621, :2727, app.rs:1911), but only on watcher events, which do not cover accessibility.

**Cost check:** About right for the code. It misses the semantic change and the docs/README snippets (gpui README:114, :146, :178; egui ext.rs:176) that describe from_system as a read.

**Changes required:** Do not cache AccessibilityPreferances::from_system itself. Add a separate cached accessor (e.g. detect::accessibility(), a DetectionContext field filled by from_system_inner and on first call, cleared by invalidate_caches), as prefers_reduced_motion sits beside detect_reduced_motion. Point the preset-switch paths in the connectors' docs and the showcases at it, and leave from_system fresh.

**Decision:** adopt the critic's form (round 4): a separate cached accessor filled by `from_system_inner` and on first call, cleared by `invalidate_caches`; `AccessibilityPreferences::from_system` stays a fresh read; connector docs and the showcases use the cached one on preset switches.
**Status:** M1 fix phase

### M1-T6-06 · no API break · Icons: keep the five loader types; add one runtime-set builder

**Current design:** One builder struct per set (icons.rs:121-535): Freedesktop with `size`/`color`/`theme` (:135-168), SfSymbols and Segoe with `color` (:282-300, :352-370), Material and Lucide with none (:417-419, :476-478). The free `load_icon(id, set)` takes no option (:530-546; its doc says to build the specific loader for colour or theme), `load_icon_indicator(set)` likewise (:556-563). The archived decision was one builder with `.set()` (docs/archive/v0.5.7_native-theme-api.md:1030-1046); the per-set split came with the docs overhaul (v0.5.7_docs-overhaul-plan.md:647), no recorded reason found.

**Problem:** A runtime `sys.icon_set` with a colour, the normal case since every system set is monochrome, forces callers to write the dispatch. It is written 19 times outside core (`IconSet::Freedesktop =>` arms: 8 in connector src — gpui icons.rs:1046, iced icons.rs:116, egui icons.rs:241 and :422, the three spinners — and 11 in the showcases), and `load_custom_via_builder` is a verbatim copy in gpui (icons.rs:1037-1059) and iced (icons.rs:107-129). An application wanting "this role, this colour, under the theme I have" must know five types plus `SystemTheme::icon_set` and `icon_theme_for`.

**Evidence:** grep counts above; `load_icon` signature icons.rs:537; `SystemTheme` has `icon_theme_for` (lib.rs:525) but no icon entry point.

**Alternatives:** (1) *keep it*: Typed per-set options, dispatch duplicated in every consumer.; (2) *runtime-set builder*: `IconLoader::new(id).set(set).size().color().theme().load()` / `.load_indicator()` carrying the union of options (ignored where the set has none, documented), dispatching to the five types, which stay.; (3) *SystemTheme entry*: `SystemTheme::icon(id) -> IconLoader` pre-filled with `icon_set` and `icon_theme_for(mode)`: the one-line application path.; (4) *fold into one builder*: The archived C: replace the five types; API break for no gain over the additive form.

**Recommendation:** The additive builder plus `SystemTheme::icon`. It is where every connector and showcase already is, and it leaves the typed path for callers that know their set.

**Cost:** icons.rs ~90 lines + tests; lib.rs ~15; connectors drop ~3 × 25 lines; showcases shrink. No dependency.

**Unblocks:** todo:475's one colouriser has one caller; a connector-neutral icon cache key (set, theme, size, colour) for the egui atlas and the gpui/iced caches.

**Overlap with docs/todo.md:** todo:475-491 (one monochrome colouriser; this adds the one dispatcher next to it); todo:2607 (`system_icon_set` fallback, unchanged by this).

**Verdict:** ADOPT WITH CHANGES — The duplication is real, and a core dispatcher fixes it. But the proposed form contradicts a recorded decision (CHANGELOG.md:330-337) by reintroducing silently ignored options. Limit the dispatcher to options all sets honour.

**Case for the current design:** The reviewer's 'no recorded reason found' is wrong. CHANGELOG.md:330-337 and commit 3132d7f9 record that the one-builder IconLoader with .set() was replaced by the typed per-set loaders because `.set(Freedesktop).theme(..)` silently dropped .theme(). The aim was to 'make that class of bug impossible by construction'. A builder 'carrying the union of options (ignored where the set has none)' reintroduces exactly that silent-ignore class.

**Evidence check:** Per-set loader fields and options (icons.rs:121-535): verified. load_icon/load_icon_indicator take no options (icons.rs:530-563): verified. 19 `IconSet::Freedesktop =>` arms outside core, 8 in connector src and 11 in showcases: verified as a count, but not all are loader dispatch. egui icons.rs:241 and gpui spinner.rs:154 are SVG recolouring (todo:475's subject), and egui pages/icons.rs:211 is a cfg check. load_custom_via_builder copied in gpui icons.rs:1037-1059 and iced icons.rs:107-129: verified (bodies identical, docs differ). Archived C (api.md:1030-1046): verified. 'No recorded reason': wrong (CHANGELOG.md:330-337, 3132d7f9).

**Cost check:** Reasonable, but the estimate assumes a design the project already removed. Showcase shrinkage is overstated, because several arms are recolouring rather than dispatch.

**Changes required:** No union builder with ignored options. Move load_custom_via_builder into core as a public runtime-set function whose every option is honoured by every set: colour applied in the loader for the system sets, and via colorize_monochrome_svg for Material/Lucide, tied to todo:475's single colouriser. Add SystemTheme::icon(id, color), which fills the freedesktop theme internally from icon_theme_for(mode), so no caller-set option can be dropped. Keep the five typed loaders.

**Decision:** adopt the critic's form (round 4): `load_custom_via_builder` moves into core as a runtime-set function whose every option every set honours (colour via the single colouriser for Material/Lucide, docs/todo.md:475), plus `SystemTheme::icon(id, color)`; the five typed loaders stay; no union builder.
**Status:** scheduled: M4 (core), M5–M7 (connectors drop their copies)

### M1-T6-07 · no API break · System icons on macOS and Windows: fixed-size, fixed-colour rasters

**Current design:** SF Symbols are rasterised at `DEFAULT_ICON_SIZE` 24 points (sficons.rs:23, :107) with the colour tinted into the pixels (:115-116); `SfSymbolsLoader` has no size (icons.rs:272-277). Segoe glyphs come from `GetGlyphOutlineW` GGO_GRAY8_BITMAP at 32 px (winicons.rs:34, :289-293, :416-420), white or `fg_color`. Both return `IconData::Rgba` at that pixel size, and the connectors draw a raster at its own size (egui icons.rs:360-372; gpui icons.rs:1305-1317); no display scale is asked for. Freedesktop returns SVG with the colour written into the bytes, or a decoded PNG (freedesktop.rs:330-367).

**Problem:** On a 2× Mac a 24-pt symbol is a 24-px bitmap upscaled; an icon wanted at `icon_sizes.toolbar` cannot be requested at that size; each colour (hover, disabled, the other scheme) is a new NSImage/GDI rasterisation; iced's SVG path excludes RGBA frames (iced icons.rs:133-136). The bundled sets stayed SVG for the same reasons (docs/archive/v0.4.0-animated-icons.md:624). The user-visible result is blurry system icons on the two platforms whose users expect the sharpest ones.

**Evidence:** File:line above; `SfSymbolsLoader`/`SegoeIconsLoader` fields (icons.rs:272-277, :342-347) have no size; `IconData` is `#[non_exhaustive]` (model/icons.rs:292). No HiDPI entry on docs/todo.md (grep Retina/HiDPI/pixels_per_point).

**Alternatives:** (1) *keep it*: Fixed 24/32 px rasters, colour baked in.; (2) *size and scale*: `.size(px)` and `.scale(f32)` on both loaders: rasterise at size × scale, document the logical size; connectors pass egui's `pixels_per_point` / gpui's window scale. Additive.; (3) *Segoe as SVG*: Glyph outlines (`GGO_NATIVE` or DirectWrite `GetGlyphRunOutline`) to SVG paths, so Segoe joins the SVG sets: resolution-independent, coloured by `colorize_monochrome_svg`. SF Symbols have no public vector export, so a raster stays there.; (4) *coverage mask*: An `IconData::Mask` tinted at draw time; additive on the enum but every connector match gains an arm.

**Recommendation:** Size and scale now (small, additive, verifiable on the macOS and Windows CI legs); Segoe-as-SVG as an M4/M6 item once M2b confirms the outline API. No platform value is involved: the sizes come from the theme's `icon_sizes`.

**Cost:** icons.rs ~30 lines; sficons.rs ~15 (point size × scale); winicons.rs ~15; ~10 lines per connector to pass the scale. No dependency.

**Unblocks:** Crisp system icons on HiDPI; the egui atlas already keys freedesktop icons by size (egui icons.rs:422-430) and can key the system sets the same way.

**Overlap with docs/todo.md:** none.

**Verdict:** ADOPT WITH CHANGES — Not being able to request a size is verified and is cheap to fix. The blurriness claim that motivates .scale() is unverified and contradicted by the code's own comment, so measure it before building it.

**Case for the current design:** Fixed sizes keep the loaders simple and keep the platform raster calls cheap. sficons.rs:42-45 states that CGImageForProposedRect with a null rect lets AppKit pick 'the best representation, which on Retina displays will be at the full pixel resolution', so the current code claims HiDPI is already handled on macOS.

**Evidence check:** DEFAULT_ICON_SIZE 24 (sficons.rs:23, :107) with the tint baked in (:115-116): verified. Segoe at 32 with GGO_GRAY8_BITMAP (winicons.rs:34, :289-293, :416-420): verified. Loaders have no size field (icons.rs:272-277, :342-347): verified. IconData #[non_exhaustive] (model/icons.rs:292): verified. No HiDPI todo entry: verified (grep). 'A 24-pt symbol is a 24-px bitmap upscaled on a 2x Mac': unverifiable, and contradicted by the code comment at sficons.rs:42-45; needs a macOS run. iced icons.rs:133-136 excludes RGBA frames: verified, but that concerns animated frames, which SF/Segoe do not supply, so it is not relevant.

**Cost check:** Plausible for .size(). .scale() also needs each connector's window-scale plumbing and cache keys (gpui/iced caches, egui atlas), plus macOS/Windows CI verification, because neither can be built or seen on the Linux host.

**Changes required:** Adopt .size(px) on SfSymbolsLoader and SegoeIconsLoader now (additive, sized from the theme's icon_sizes). Make .scale() conditional on an M2b/M4 measurement on the macOS leg of what CGImageForProposedRect(null) returns at 2x, and on Windows at 150%/200%. Keep Segoe-as-SVG as an M4/M6 item, as proposed.

**Decision:** adopt the critic's form (round 4): `.size(px)` on `SfSymbolsLoader` and `SegoeIconsLoader`; `.scale()` only after an M2b/M4 measurement on the macOS and Windows legs; Segoe outlines as SVG an M4/M6 item.
**Status:** scheduled: M4 (icons)

### M1-T6-08 · no API break · Watchers: keep the payload-less event; the gaps are listed

**Current design:** `on_theme_change` returns a `ThemeSubscription` joined on drop (watch/mod.rs:127-131, :176-191) and fires `ThemeChangeEvent::Changed`, `#[non_exhaustive]`, no payload (:97-102), from one backend per platform: KDE `notify` on the config directory with a 300 ms throttle (watch/kde.rs:29-51, :59-60), GNOME portal `SettingChanged` for `org.freedesktop.appearance` (watch/gnome.rs:40-56), macOS `AppleInterfaceThemeChangedNotification` (watch/macos.rs:88-89), Windows `ColorValuesChanged` (watch/windows.rs:77). Separate `watch` feature (Cargo.toml:31; docs/archive/v0.5.6_runtime-theme-change.md:267-286). 867 lines.

**Problem:** The design is right: the consumer re-reads through `from_system` (4-9 ms warm, ledger), which is cheaper than a diff protocol and keeps the readers the one source. What the watchers miss is already on docs/todo.md:247-283 per platform, and the KDE burst at :294-308. The structural weakness is that watcher and reader list their sources separately, so the gap recurs whenever a reader gains a source.

**Evidence:** File:line above; todo entries; the GNOME drop hang is fixed (todo:285-293, checked).

**Alternatives:** (1) *keep it*: Plus the todo:247 and :294 fixes.; (2) *re-read and compare*: The watcher thread runs `from_system` after a trailing debounce and fires once per changed result; fixes KDE bursts too, at a reader run per burst on the watcher thread.; (3) *event payload*: `Changed(Box<SystemTheme>)`: changes the variant's shape, an API break, and the consumer still rebuilds its toolkit theme.

**Recommendation:** Keep. When the todo:247 fix lands (M4), each reader module lists its sources in its docs and watch/ gets a test-only table asserting every listed source has a watched notification, so the drift is caught mechanically.

**Cost:** none now; the table is part of the todo:247 fix.

**Unblocks:** nothing new.

**Overlap with docs/todo.md:** todo:247-283 (per-platform gaps), :285-293 (done), :294-308 (KDE trailing debounce); this adds only the source-to-notification table.

**Verdict:** KEEP — I agree with keeping it. The gaps are already on docs/todo.md, and the only new idea, a test-only table tying reader sources to watched notifications, belongs with the todo:247 fix in M4.

**Case for the current design:** Archived v0.5.6_runtime-theme-change.md chose a payload-less callback with drop-to-stop and a separate watch feature (:49-58, :267-286). Re-reading through from_system keeps the readers as the single source, and a non-exhaustive event leaves room for growth.

**Evidence check:** ThemeChangeEvent::Changed #[non_exhaustive] (watch/mod.rs:97-102): verified. ThemeSubscription with Drop join (:127-131, :176-191): verified. GNOME portal appearance namespace (watch/gnome.rs:44-56): verified. macOS AppleInterfaceThemeChangedNotification (watch/macos.rs:88-89): verified. Windows ColorValuesChanged (watch/windows.rs:77): verified. 867 lines: verified (wc). Gaps listed at todo:247-283 and :294-308, GNOME drop hang fixed at :285-293: verified. Warm from_system 4-9 ms: roughly verified (3.63-8.30 ms, one 24.34 ms outlier).

**Cost check:** No cost now. The source-to-notification test table is a small addition to the todo:247 fix.

**Decision:** keep — the critic's verdict stands; not put to the maintainer (plan, M1 Step 4).
**Status:** closed

*Reviewer D-C notes:* Read-only; no build run. Timings cited are the ledger's. Two items for other milestones, not proposals: (1) M2e/M4 question — the GNOME reader takes `font_dpi` from `Xft.dpi` (native-theme/src/gnome/mod.rs:143-152) and `text_scaling_factor` from gsettings (:343); if GNOME's settings daemon derives `Xft.dpi` from `text-scaling-factor` (not verified here, gnome-settings-daemon is not installed on this KDE machine), a GNOME user at 1.5 is scaled twice today (DPI in resolution, factor in the connectors); the text-scale-in-resolution proposal would make that one place to check. (2) M2b question — whether `UISettings.AnimationsEnabled` (detect.rs:678-686) and `SPI_GETCLIENTAREAANIMATION` (windows.rs:395-411) reflect the same Windows setting. The macOS `mode` finding (first proposal) is S1-shaped and verifiable only on the macOS CI leg; I did not run anything on macOS. The archive gives no reason for the per-set loader split replacing the single builder of v0.5.7 §8 (only docs-overhaul-plan.md:647 mentions it); if the maintainer remembers one, the icon proposal should be weighed against it.

*Critic D-C notes:* Cross-cutting items for the orchestrator:
(1) Proposal 2's evidence misreads platform-facts.md:1123, which gives KDE text_scaling_factor as `forceFontDPI / 96`, not 'none'. That suggests a possible existing defect for M2/M3. On GNOME the reader's font_dpi is xrdb Xft.dpi (native-theme/src/gnome/mod.rs:140-153). If GNOME's xsettings daemon derives Xft.dpi from text-scaling-factor (not verified here; needs an upstream source), today's connector-side scaled_text_size double-scales on GNOME. Treat this as an M2 question, not a fact.
(2) Proposal 5 contradicts a recorded decision the reviewer reported as missing: CHANGELOG.md:330-337 and commit 3132d7f9 (typed per-set loaders close the silent-ignore bug class).
(3) Proposal 4's cache would go stale, because the watchers miss most accessibility changes (docs/todo.md:247-283).
(4) Proposal 1's crate counts are inflated by '(*)' duplicate lines: 89 unique crates with portal against 29 without.
(5) The ledger's needless_borrow note (docs/todo_code-review-findings.md:38) cites `from_waker(&waker)`; the current code at lib.rs:299 and :682 passes `waker` without a borrow, which looks fixed by a6bcdcfe. Proposal 1 removes the code either way.
Only read-only commands were run: grep, sed, git log/show, cargo tree --offline, gsettings get, strings. No builds.

### M1-T7-01 · no API break · Toolkit-free helpers written three times move into core

**Current design:** `text_scale_factor` (finite and positive, else 1) exists at iced `connectors/native-theme-iced/src/lib.rs:1056`, gpui `connectors/native-theme-gpui/src/lib.rs:471` and egui `connectors/native-theme-egui/src/accessors.rs:31`, whose doc cites iced's lines as its source (`:30`). The monochrome-SVG colouriser lives in core (`native-theme/src/icons.rs:587`, commit 8545159a "one colouriser for every connector") yet is still copied at iced `icons.rs:300` and gpui `icons.rs:1356`; egui `icons.rs:253` and gpui `widgets/spinner.rs:153` call core's. Source-over compositing of a hover layer (C17) is written at iced `styles.rs:79` (f32 straight alpha), egui `convert.rs:333` (premultiplied u8 via `Color32::blend`), gpui `widgets/mod.rs:161` (`Hsla::blend`, gpui-pre 0.3.8 `src/color.rs:58-71`), and core keeps a private `over` at `native-theme/src/kde/colors.rs:28`. egui's toolkit-free `padding_with_border` (`convert.rs:270`) returns a `ResolvedPadding`; iced's `padding_inside_border` (`lib.rs:415`) does the same sum inline.

**Problem:** One rule in three crates drifts. iced's colouriser returns after replacing `currentColor` (`icons.rs:313-315`) and never reaches the black-fill step that core and gpui run, so the three connectors colour the same icon differently. egui's composite doc admits a rounding difference from iced's (`convert.rs:322-331`), absorbed only by the parity script's 1-per-channel tolerance (04a39d3c). The text-scaling rule was fixed in two connectors in one commit (038695d6), and `docs/todo.md:2086` still counts two copies where there are three. Core's `AccessibilityPreferences` has no method for it (`native-theme/src/lib.rs:247-258`) and `Rgba` no blend (`native-theme/src/color.rs:61-137`).

**Evidence:** Lines cited above. Use counts: `composite_over` 14× in iced `styles.rs`, 15× in egui `style/*.rs` plus 6× in egui-widgets; `over(` 16× across gpui `widgets/*.rs`. Core's `over` (`kde/colors.rs:28`) and iced's (`styles.rs:79-96`) are the same straight-alpha formula; gpui's `Rgba::blend` keeps the base alpha (`color.rs:68`), a fourth variant.

**Alternatives:** (1) *keep it*: Each connector owns its arithmetic; accept the drift and the parity tolerance.; (2) *core methods*: `AccessibilityPreferences::text_scale_factor()`, `Rgba::over(self, layer)` (straight alpha in sRGB, iced's argument `styles.rs:74-78`), `ResolvedWidgetBorder::padding_outside()` (egui's `padding_with_border` moved), and the iced and gpui colourisers deleted for core's; each connector converts the result to its colour type, as gpui's spinner already does (`widgets/spinner.rs:153`). egui's `pub` `composite_over` stays as a wrapper.; (3) *a shared connector-common crate*: A fourth published crate for about sixty lines; more surface than it removes.

**Recommendation:** Core methods. They are toolkit-free, already exist once in core in some form, and remove three places where one bug must be fixed three times (038695d6 is the precedent). Per-toolkit colour conversion (`palette::to_color`, `rgba_to_hsla`, `to_color32`) stays: those are target-type conversions, not shared logic.

**Cost:** Core +~80 lines with tests (`lib.rs`, `color.rs`, `model/border.rs`); iced −~90 (`icons.rs:260-330`, `lib.rs:1056-1060`, `styles.rs:79-100`), gpui −~70 (`icons.rs:1356-1400`, `lib.rs:471-474`, `widgets/mod.rs:161`), egui −~40. No dependency. Four commits.

**Unblocks:** Review Focus 4 (extreme text scale) is checked once, in core; M5–M7 read one composite and one colouriser; the parity tolerance can tighten.

**Overlap with docs/todo.md:** `docs/todo.md:2086-2088` (text_scale_factor written twice; a core method): the proposal adds the third copy, the two leftover colourisers, the compositing and the border-padding sum.

**Verdict:** ADOPT WITH CHANGES — The text-scale and colouriser parts are real duplication with a measured drift, and both are already on docs/todo.md. Moving the composite into core would replace three deliberate per-toolkit blends with a u8 one and change iced's exact contract values.

**Case for the current design:** Each composite is deliberately the toolkit's own blend. gpui's colors.rs:340-354 explains that it keeps `Hsla::blend` so the widget states match the button states the connector already composites with it. egui's convert.rs:322-331 uses ecolor's `Color32::blend`, which is what egui itself paints with. iced's styles.rs:74-78 gives the sRGB argument for its own exact f32 composite, which the contract rows compare exactly (styles.rs:87-92). For the colouriser and the text-scale factor there is no recorded reason to keep copies: core's colouriser was written to replace them (8545159a).

**Evidence check:** text_scale_factor in three places (iced lib.rs:1056, gpui :471, egui accessors.rs:31, which cites iced): verified. Core colouriser at icons.rs:587 and the iced and gpui copies: verified. iced returns early after replacing currentColor (icons.rs:313-315): verified. egui and gpui's spinner call core's: verified. 038695d6 fixed two connectors at once: verified. Use counts: iced styles.rs composite_over 14×: verified; egui style/*.rs 15×: verified; egui-widgets 6×: wrong, it is 8; gpui `over(` 16×: wrong, it is 19. 'Core's `over` is the same formula as iced's': wrong. kde/colors.rs:35-40 takes the bottom as opaque and returns alpha 1, while iced handles a translucent base. gpui keeps the base's alpha (gpui-pre color.rs:58-70): verified.

**Cost check:** Removing iced's and gpui's colourisers also removes about 200 lines of their unit tests (iced icons.rs:640-850, gpui icons.rs:2150-2310). Their cases must move into core's tests, as todo:486-490 asks; the estimate leaves this out. A core `Rgba::over` works in u8 `Rgba`, so it would round iced's exact f32 composite to u8 and change contract rows that compare exactly. That is a behaviour change, not a move. `padding_outside` serves only egui and iced: gpui lays out content inside the border and adds no line width. It also changes iced's handling of non-finite values: egui's version filters them out, iced's (lib.rs:415-429) does not.

**Changes required:** Adopt the `AccessibilityPreferences::text_scale_factor()` method, citing todo:2086-2088 and correcting it to three copies. Adopt the switch of iced and gpui to core's colouriser as the existing entry docs/todo.md:476-490, citing that entry as the item instead of presenting it as a new finding, and move their test cases into core. `ResolvedWidgetBorder::padding_outside()` is optional: if adopted, document that iced gains egui's non-finite filtering. Drop `Rgba::over`: each connector keeps its own composite, and egui's parity tolerance stays.

**Decision:** adopt the critic's form (round 5): the text-scale method (docs/todo.md:2086, corrected to three copies) in M1 with M1-T6-03; iced and gpui switch to core's colouriser with their test cases moved into core (docs/todo.md:476-490) in M5 and M6; composites stay per toolkit; `padding_outside()` optional; no `Rgba::over`.
**Status:** M1 fix phase (method); M5, M6 (colouriser)

### M1-T7-02 · no API break · Spinner cycle position and arc constants stated once in core

**Current design:** Core builds the spinner frames (`native-theme/src/spinners.rs:14-20`, 24 frames × 42 ms) and `AnimatedIcon` exposes `frame_list()`, `frame_duration_ms()`, `animation()` (`native-theme/src/model/animated.rs:315-342`) but nothing answers "which frame at time t". Each connector derives it: iced `frame_index` (`connectors/native-theme-iced/src/spinner.rs:213-224`) and `spin_rotation_radians` (`icons.rs:219-225`); gpui `frame_at` and `arc_at` (`connectors/native-theme-gpui/src/widgets/spinner.rs:116-125`); egui `animated_frame_index` and `spin_angle` (`connectors/native-theme-egui/src/icons.rs:482-520`). The fallback arc's 240° sweep is a constant in iced `spinner.rs:20`, gpui `widgets/spinner.rs:112` and egui-widgets `spinner.rs:17`; its one-turn-per-second period in gpui `:104`, iced `spinner.rs:178` and egui-widgets `spinner.rs:163`.

**Problem:** Three implementations of the same modular arithmetic on the same core data, with three overflow strategies (iced checked u128, gpui f32 floor, egui u128 with `?`), and the arc's look defined thrice by constants whose gpui doc already cites the egui connector and "the iced connector's arc keeps too" (`widgets/spinner.rs:106-112`). The reduced-motion rule (first frame, still) is restated per connector (iced `spinner.rs:31`, gpui `:333-335`, egui-widgets `:30-43`). A change to the cycle is three edits.

**Evidence:** Lines above. `docs/todo.md:1742-1744` (spinner size applied twice in gpui) shows how a per-connector spinner path drifts from the model.

**Alternatives:** (1) *keep it*: Three timing helpers, three copies of the arc constants.; (2) *core methods and constants*: `AnimatedIcon::frame_index_at(&self, elapsed: Duration) -> Option<usize>`, `TransformAnimation::turn_at(elapsed) -> f32` (fraction of a turn), plus `spinners::ARC_SWEEP_DEGREES` and `ARC_PERIOD` carrying the citation gpui's doc holds today; connectors keep only repaint scheduling (egui `icons.rs:498`) and drawing. egui's two functions stay `pub` as wrappers because egui-widgets calls them (`egui-widgets/src/spinner.rs:140`).; (3) *a core SpinnerClock type*: Core owning elapsed time; over-reach, the toolkits own their clocks.

**Recommendation:** The core methods and constants. Not a platform value: the sweep and period are the connectors' existing fallback, moved with their citation; M2 decides nothing here.

**Cost:** Core +~60 lines with tests in `model/animated.rs` and `spinners.rs`; iced −~25, gpui −~15, egui −~20 (wrappers remain). Two commits.

**Unblocks:** M5–M7 review one timing rule; a reduced-motion change lands once; the egui-widgets spinner and gpui's read the same constants.

**Overlap with docs/todo.md:** none; `docs/todo.md:345-368` (spinner stroke-width source) is M2's and untouched.

**Verdict:** KEEP — The shared arithmetic is a few lines of modular division, tested in each connector, and gpui's clock does not fit an elapsed-time API. The constants are toolkit look values that the model deliberately does not carry, so moving them into core gains little and puts egui's citations in the wrong crate.

**Case for the current design:** Each connector computes the frame position on its toolkit's own clock. gpui gets a 0-1 `delta` from gpui's `Animation::new(..).repeat()` (widgets/spinner.rs:460-463, 494-498), not an elapsed `Duration`. egui's `animated_frame_index` uses the same arithmetic to schedule the next repaint (`request_repaint_after(until_next)`, icons.rs:497-498). iced's `spin_rotation_radians` is public API. The arc's 240° sweep and one-second period are egui's (and gpui-component's) toolkit constants, cited to egui 0.36.2 spinner.rs:48-49 and progress_circle.rs:201. The model deliberately does not carry them ('The model states the arc's diameter, stroke and colour, not its sweep').

**Evidence check:** The frame data in core (spinners.rs, animated.rs:315-342) has no time-to-frame method: verified. iced frame_index (spinner.rs:213-224) and spin_rotation_radians (icons.rs:219-225): verified. gpui frame_at and arc_at (:116-125): verified. egui animated_frame_index and spin_angle (icons.rs:482-520): verified. SWEEP in three places (iced :20, gpui :112, egui-widgets :17) and the period: verified. Three overflow strategies: verified. todo:1742-1744 is about the spinner's size, not its timing, so it does not support this proposal.

**Cost check:** gpui would have to rebuild an elapsed Duration from gpui's delta, or keep `frame_at`, so its −15 does not happen. egui keeps its repaint arithmetic, so its −20 shrinks to a few lines. Core would gain public API, two methods and two constants, whose citations name egui and gpui-component source lines: toolkit facts sitting in the toolkit-free crate.

**Decision:** keep — the critic's verdict stands; not put to the maintainer (plan, M1 Step 4).
**Status:** closed

### M1-T7-03 · no API break · Converse coverage: every model leaf has a sink, in gpui and iced too

**Current design:** The gpui and iced contracts partition the toolkit's slots: gpui every `ThemeColor` field across rows, `COMPUTED_ROWS` and `DERIVED` (`connectors/native-theme-gpui/src/contract.rs:11-15`, `:1271`, enumerated by serde over `ThemeColor` at `:1277`); iced every palette slot and every `styles::*` field across rows, `DERIVED` and `UNREACHABLE` (`connectors/native-theme-iced/src/contract.rs:12-15`, `:1702`). Neither enumerates the model (no `serde_json::to_value(resolved)` in either crate). egui's design does: `mapping.toml` has one row per native leaf and `every_leaf_has_exactly_one_row_and_every_row_is_well_formed` walks 501 `ResolvedTheme` leaves plus 4 layout leaves by serde (`connectors/native-theme-egui/src/mapping_tests.rs:156-174`, `:234-240`); rationale §3.20 chose it because "a native-theme field addition is caught by nothing else" (`docs/archive/todo_v0.6.0_egui-connector-rationale.md:2110-2121`).

**Problem:** For gpui and iced a model leaf can map nowhere, silently — the gap the egui design was built to close. The todo already holds instances of exactly that class: iced "reads nothing of the model's toolbar" (`docs/todo.md:2239`); iced "maps no `DialogTheme` field, and its contract lists none as unreachable" (`:2589`). The §2.4 "one table" decision (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:224-229`) took the toolkit as the domain for two connectors; the egui design strengthened the property on the model side and nobody ported it back. Decisions C6/C7 are not contradicted: this adds a test to the table they chose.

**Evidence:** Lines cited. Cost of the hand-written form today: iced contract 8,208 lines against 4,111 production (14,572 total − 8,634 test-only files − 1,827 inline tests), 47 commits, +13,374/−5,166 lines since 2026-09-21; gpui contract 2,092 lines, 26 commits, +2,470/−378.

**Alternatives:** (1) *keep it*: Rely on todo entries and reviewers to notice an unread leaf.; (2) *port egui's leaf walk and add one test per connector*: Copy `leaf_paths` (`mapping_tests.rs:156-174`, ~20 lines, or host it in the testkit crate of the compat proposal) into the gpui and iced contracts; rows gain a `leaf: &'static str` beside `native`; one test asserts every leaf path is named by a row, a `DERIVED` entry or an `UNREACHABLE` entry with its reason.; (3) *migrate gpui and iced to a mapping.toml manifest*: The full egui design with generated docs and Widget Info; a rewrite of ~10.3k lines of contract.

**Recommendation:** The port. It adds the one property the two older contracts lack at a fraction of a rewrite; the first run lists the unmapped leaves (the toolbar and dialog among them) to classify. The manifest migration is a rationale-document question for after this plan, not a fix-phase task.

**Cost:** gpui `contract.rs` ~+150 lines (leaf names on ~130 rows, the walker, one test); iced `contract/rows.rs` ~+300 and `contract.rs` ~+60; then classification commits for whatever the first run lists. No new dependency (`serde_json` is already a dev-dependency where the walker runs; confirm for iced).

**Unblocks:** iced's toolbar and dialog gaps become failing tests to classify instead of todo prose; iced Widget Info generation (the showcase proposal, `docs/todo.md:2224`) gets a leaf→slot table to read; a model field M2 adds cannot map nowhere.

**Overlap with docs/todo.md:** `docs/todo.md:2239`, `:2589` (iced model gaps) and `:2027-2035` (gpui write-only slots): the proposal adds the mechanism that finds this class, instead of the instances.

**Verdict:** ADOPT WITH CHANGES — The gap is real and already shows as todo instances. As proposed, though, it adds an unchecked hand-written leaf string and leaves out the geometry leaves the contracts never covered, so the cost is far above ~500 lines.

**Case for the current design:** Rationale §2.4 (theme-contracts-rationale.md:222-238) chose one hand-written table over the toolkit's slots, plus a tripwire over every slot. It rejected generating the table from the mapping code. The gpui and iced contracts were built to state the toolkit side honestly. egui's model-side walk (§3.20, rationale:2110-2121) works only because a differential test, T2, splices a leaf and checks the declared sinks. That is what keeps egui's leaf names honest.

**Evidence check:** The gpui and iced contract headers and tripwires (gpui :11-15, :1271-1277; iced :12-15, :1702): verified. Neither serialises ResolvedTheme: verified; gpui's only serde walk is over ThemeColor. egui's leaf_paths (mapping_tests.rs:156-174) and the 505-leaf assertion (:234-240): verified. todo:2239 (iced toolbar) and :2589 (iced dialog): verified. Contract size of 8,208 lines and 14,572 in total: verified. '47 commits, +13,374/−5,166 since 2026-09-21': wrong. It is 38 commits, and +13,374/−5,166 is all-time; since 2026-09-21 it is 37 commits and +12,946/−5,166. gpui: 26 commits: verified. serde_json is already a dev-dependency in iced (Cargo.toml:105) and gpui (:83): verified.

**Cost check:** The estimate badly understates the work. gpui's contract has 100 colour rows over `ThemeColor`, and gpui's geometry leaves (fonts, padding, sizes, about 400 of the 505) reach the toolkit through `geometry::*` builders that no contract row covers. So the first run would list most of the model as unmapped, not 'the toolbar and dialog among them'. A `leaf: &'static str` written beside a `native: fn(&ResolvedTheme)` closure is a hand-written claim that nothing checks. Many closures read several leaves (iced rows.rs:139 `as_one`, the composites), which is the drift class egui's T2 exists to stop.

**Changes required:** Scope the first step to the colour leaves, the domain the two contracts already own. Derive each row's leaves mechanically, not as a hand-written string: splice one leaf from another preset into the resolved theme and record which rows' `native` values change, egui's T2 method. Then assert that every colour leaf reaches a row, a `DERIVED` entry or an `UNREACHABLE` entry. Leave coverage of geometry leaves for a separate, later decision, with its own cost estimate. Re-estimate the cost after a dry run that counts the unmapped leaves.

**Decision:** adopt the critic's scoped form (round 5): colour leaves only, each row's leaves derived by splicing a leaf from another preset (egui's T2 method), one coverage test per connector; a dry run counts the unmapped leaves and re-estimates the cost first; geometry leaves a later decision.
**Status:** scheduled: M5 (iced), M6 (gpui)

### M1-T7-04 · no API break · The verification machinery pays; keep it, with the ledger of catches

**Current design:** Layer 1 contracts (gpui `src/contract.rs`; iced `src/contract.rs` + `contract/`; egui `contract.rs`, `mapping_tests.rs`, `install_tests.rs`, `style_diff.rs`); egui compile-time tripwires (`connectors/native-theme-egui/src/tripwires.rs:1-3`, two tests, 260 lines); compat stamps (`compat.rs` in each connector reading `docs/COMPATIBILITY.toml`, `scripts/update_compatibility.sh`, gate at `scripts/check_release.sh:843-857`); showcase self-tests, the shared element list and `scripts/check_showcase_parity.py` (1,944 lines) with `docs/showcase-exceptions.toml` (1,112 lines); the nightly canary (`.github/workflows/dependency-canary.yml:22`).

**Problem:** The question is whether each piece earns its lines. Measured test share today: iced 10,461 test lines to 4,111 production; gpui 12,114 to 7,647; egui 12,563 to 6,404 (test-only files plus `#[cfg(test)]` modules, counted by script).

**Evidence:** Contracts at creation: seven iced slot defects (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:72-104`), gpui `accent` (`:56-60`), a claimed-but-unchecked row made to fail (c6336a7b), unreachable `tab.min_width` and card `close_color` (927d2433, ddf97b4a), two slots with no reader (231d2fc0), the omission report's eight defects (27f1039c); `docs/todo.md:1120-1135`: all nine closed in v0.5.9. After creation: the gpui-component 0.7.1 move (2a52c29f, 064f8c9d, 29ec6869) changed no contract row; stale prose citations were found by reading (67c90455), which nothing checks. Tripwires: one commit, egui still 0.36.2 (`connectors/native-theme-egui/Cargo.toml:36`); no catch, no upkeep. Compat: its catch is its header (`compat.rs:4-7`); cost: the stamp hashes sources (`docs/COMPATIBILITY.toml:10-14`), so four re-stamp commits on 2026-10-07/08 (ed41d7fd, db143886, 4e27a81f, cb2633dc). Canary: found the `tiles` removal (`docs/archive/todo_v0.5.9_gpui-kit-0.6.4-rationale.md:71-73`). Parity script: CHANGELOG:84, :120; 569 exception keys (f99889ea).

**Alternatives:** (1) *keep it*: All pieces stay; the stale parity agent is handled in the API-shape proposal.; (2) *drop the source hash from the compat stamp*: Stamp per upstream set only; saves the re-stamp commits but makes the Verified line a claim about older sources, which `update_compatibility.sh:20-24` was written to refuse.; (3) *drop the egui tripwires*: Saves 260 lines that cost nothing and would fire on the first egui bump.

**Recommendation:** Keep it. Every piece but the parity agent has a recorded catch or no upkeep. The re-stamp commits are the stamp's stated rule, and batching them with the release's asset run is already the practice (36904a59). The one known blind spot — prose citations drifting under an upstream bump — is M9/M8 material, not a design change.

**Cost:** none

**Unblocks:** Records the evidence so M5–M9 reviewers do not re-litigate the contracts' existence, only their rows.

**Overlap with docs/todo.md:** `docs/todo.md:1370-1381` (Widget Info follow-ups): unchanged.

**Verdict:** KEEP — Each piece either has a recorded catch or costs nothing to keep. The re-stamp churn is the stamp's stated rule, and those commits are already batched into the release asset run.

**Case for the current design:** Rationale §1 (theme-contracts-rationale.md:36-124) and C6-C8 recorded the defects that justified the contracts. The compat header (gpui compat.rs:4-7) records the README/manifest drift it closes. update_compatibility.sh refuses to stamp stale sources by design. The canary found the `tiles` removal.

**Evidence check:** Tripwires: one commit (0e8e9523), two tests, egui 0.36.2 at Cargo.toml:36: verified. check_release.sh:843-857: verified. Parity script 1,944 lines and exceptions 1,112: verified. f99889ea, 569 keys: verified. Re-stamp commits ed41d7fd, db143886, 4e27a81f, cb2633dc (2026-10-07 and 08) and 36904a59: verified. c6336a7b, 927d2433, ddf97b4a, 231d2fc0, 27f1039c and 67c90455 say what the proposal claims: verified. The canary schedule (dependency-canary.yml:22): verified. todo:1120-1135, all nine closed: verified. Test-share counts: unverifiable without the reviewer's script, but consistent with the file sizes.

**Cost check:** No cost. Correct as stated.

**Decision:** keep — the critic's verdict stands; not put to the maintainer (plan, M1 Step 4).
**Status:** closed

### M1-T7-05 · no API break · One unpublished test-support crate for the thrice-copied harnesses

**Current design:** `compat.rs` is copied into each connector: iced 426 lines, gpui 518, egui 433, the same thirteen functions (`connectors/native-theme-iced/src/compat.rs:44-236`, gpui `:45-266`, egui `:50-243`); iced and egui differ in 29 lines, iced and gpui in 130 (gpui adds the Quick-start check, `:211`, `:315`). Each contract re-implements the walk over `Theme::list_presets()` in both modes (iced `contract.rs:823`, gpui `contract.rs:1006`, egui `contract.rs:92`, `install_tests.rs:1382`, `:2050`). The three showcases each parse `docs/showcase-elements.toml` with their own `ShowcaseElement` (gpui `examples/showcase-gpui/elements.rs:29-52`, egui `examples/showcase-egui/elements.rs:8-21`, iced `examples/showcase-iced.rs:240-260`, "walked by hand").

**Problem:** A fix lands once and must be copied twice: bf39afc1 corrected egui's compat readers alone, a023cdef touched the two older READMEs together; 1,377 near-identical test lines; adding the egui-widgets crate to the compat set meant editing the script and every stamp (e2b87875). Three element parsers can disagree on the list's schema while the parity script assumes one.

**Evidence:** Counts above; `git log` on compat.rs: iced 3, gpui 6, egui 3 commits in three weeks.

**Alternatives:** (1) *keep it*: Three copies, each changing three to six times in three weeks.; (2) *native-theme-testkit, a workspace member never published*: Used as a path-only `[dev-dependency]`: `compat::check(Inputs { readme, manifest, workspace, stamp, crate_name, required })` called from a ~20-line `compat.rs` per connector (the `include_str!`s stay in the crate, their paths being crate-relative); `presets::for_each_preset_mode(..)`; `showcase_elements()` for the three examples. Precedent: the egui connector already takes a sibling by path as a dev-dependency (`connectors/native-theme-egui/Cargo.toml:80`).; (3) *a test-support feature on native-theme*: Ships test code in the published crate; rejected.

**Recommendation:** The testkit crate, starting with `compat` and the element list; the preset walk follows when the converse-coverage proposal touches the contracts. Cargo documents that a dev-dependency with only a path is stripped at package time; the gate's `cargo package` (`scripts/check_release.sh:773`) confirms it in the first commit before more moves.

**Cost:** One crate (~400 lines moved, ~1,000 deleted), one workspace member line, three `[dev-dependencies]` lines; `scripts/check_features.sh:36-37` reads members from `cargo metadata`, so it is picked up; `.github/workflows/publish.yml:54-79` names crates and needs no entry. Three commits.

**Unblocks:** A shared leaf walk for the converse-coverage proposal; one element-list schema for the showcases; fewer files for M8's cross-cutting review.

**Overlap with docs/todo.md:** none

**Verdict:** KEEP — The main evidence that a fix had to be copied (e2b87875) never touched compat.rs. The saving is test-only lines that change a few times a month, while the shared crate adds stamp-hash and packaging complications the estimate misses.

**Case for the current design:** Each connector's compat.rs is self-contained beside the README and manifest it `include_str!`s. It sits inside that connector's own source hash (COMPATIBILITY.toml:10-14), so the stamp proves the checks for exactly those sources. gpui's extra 130 lines are its genuine Quick-start toolkit floor (gpui README:105 names gpui-kit; the iced and egui Quick starts name only native-theme crates).

**Evidence check:** Line counts 426, 518 and 433 (1,377 in total): verified. Diffs of 29 lines (iced against egui) and 130 (iced against gpui): verified. 3, 6 and 3 commits: verified. bf39afc1 (egui doc fix only) and a023cdef: verified. e2b87875 is cited as 'editing the script and every stamp': wrong. It touched only scripts/update_compatibility.sh and update_provenance.sh, no compat.rs, and a shared crate would not have avoided it. The egui path-only dev-dependency at Cargo.toml:80: verified. cargo package at check_release.sh:773: verified. The element parsers: not opened.

**Cost check:** The estimate leaves out the stamp's source hash. If the testkit crate is not hashed, a change to the checks no longer invalidates the stamp. If it is hashed for all three connectors, every testkit edit re-stamps three connectors, the churn proposal 3 counts as the stamp's main cost. A path dev-dependency is stripped at package time, so the published crates' `#[cfg(test)] mod compat` would no longer compile from the tarball. The new workspace member also needs `publish = false` and the clippy, machete and check_features handling.

**Decision:** keep — the critic's verdict stands; not put to the maintainer (plan, M1 Step 4).
**Status:** closed

### M1-T7-06 · api-break · Entry points and accessors share one shape; retire the parity agent

**Current design:** `from_system` returns a 4-tuple in iced (`connectors/native-theme-iced/src/lib.rs:325-330`, with `AccessibilityPreferences`), 3-tuples in gpui (`connectors/native-theme-gpui/src/lib.rs:317`) and egui (`connectors/native-theme-egui/src/ext.rs:235`); `from_preset` takes `&prefs` in gpui (`lib.rs:272-276`) and egui (`ext.rs:184-188`), not iced (`lib.rs:297-300`); `to_theme` is `(resolved, name)` in iced and egui, `(resolved, name, is_dark, prefs)` in gpui (`lib.rs:179-184`); `SystemThemeExt` yields three return shapes (iced `:347`, gpui `:343`, egui `ext.rs:273`). Of the free accessors (42 in iced `lib.rs`, 31 in gpui `lib.rs`, 42 in egui `accessors.rs`+`ext.rs`) ten names are shared; one field carries three names: `defaults.info_text_color` is `info_foreground_color` (iced `:1132`) and `info_text_color` (egui `:65`); `disabled_text_color` is `disabled_foreground` (gpui `:611`); `selection_inactive_background` is `selection_inactive` (gpui `:605`). The only parity instrument, `.claude/agents/connector-parity-checker.md`, is gitignored (`.gitignore:23`), dated 2026-04-19, gpui+iced only (`:3`, `:9`), cites the deleted planning's phases (`:9`), and no gate runs it.

**Problem:** An application author reads three READMEs destructuring three tuple shapes (iced `README.md:89`, gpui `:115`, egui `:146`), and a positional tuple cannot grow without a break — iced's fourth element was one (CHANGELOG:161). The accessor vocabulary is each connector's, not the model's, so a value documented once in `platform-facts.md` is found under three names. The deviations have reasons (C4, `docs/archive/todo_v0.5.9_theme-contracts-rationale.md:522`; egui `ext.rs:248-262`) but are recorded per crate, and nothing checks the convention they imply.

**Evidence:** Lines cited; 79 `from_system()`/`from_preset(` sites in READMEs, examples and docs would change; the agent's two recorded findings are at `docs/todo.md:1128`.

**Alternatives:** (1) *keep it*: Document the three shapes in one table in the workspace README; delete the stale agent.; (2) *a core Connector trait*: `type Theme; fn to_theme(..)`: the inputs genuinely differ (egui's atlas needs both variants, iced's palette has no use for prefs), so a trait forces a lowest common signature; rejected.; (3) *by construction where shared, by checked convention where not*: Core `#[non_exhaustive] pub struct Loaded<T> { theme: T, resolved: ResolvedTheme, mode: ColorMode, accessibility: AccessibilityPreferences }` returned by every `from_system` and `from_preset`; `to_theme` and `SystemThemeExt` keep their documented toolkit-specific signatures; every one-field accessor takes the model field's name; `scripts/check_connector_parity.py` (sibling of `check_widget_coverage.py`) lists each connector's `pub fn` names and the field each reads, failing on a name unlike its field or absent from a sibling without a reasoned exception entry; the agent file is deleted.

**Recommendation:** The third. One generic struct replaces three tuples without a trait; renaming accessors to field names needs no design, only `docs/property-registry.toml`; the script does what the agent was for, versioned and gated. Before 1.0 is the cheap time (plan line 35).

**Cost:** Core +~30 lines; three `lib.rs`/`ext.rs` signature changes and ~8 renames; 79 call sites in READMEs, examples and doctests; the script ~200 lines plus one `run_check` in `check_release.sh`. One commit per connector plus the script.

**Unblocks:** A fifth field (the OS version D4 introduces) joins `Loaded` without a break; one quick-start shape in three READMEs; the agent's findings become a gate; the connectors' accessor layer reads as the model's own vocabulary.

**Overlap with docs/todo.md:** `docs/todo.md:1128` (what the agent reported): the proposal turns the agent into a gate; no entry proposes the struct or the renames.

**Verdict:** ADOPT WITH CHANGES — The positional tuple genuinely cannot grow without a break (the D4 OS-version field is next), and the accessor names diverge from the model's vocabulary. But `Loaded` on from_preset only echoes inputs, and the change is breaking, so it must go through D1.

**Case for the current design:** C4 (theme-contracts-rationale.md:522) made iced's from_system return the preferences and left from_preset and to_theme unchanged, 'because a palette has nothing for the preferences to act on'. egui's ext.rs:248-262 records why its SystemThemeExt returns both variants. The shapes differ for documented toolkit reasons.

**Evidence check:** from_system tuple arities (iced 4, gpui 3, egui 3): verified. from_preset takes &prefs in gpui (:272-276) and egui (ext.rs:184-188), not in iced: verified. gpui to_theme's four parameters (:179-184): verified. Accessor names info_foreground_color (iced :1132), info_text_color (egui :65), disabled_foreground (gpui :611) and selection_inactive (gpui :605): verified. The agent is ignored through `.claude/` (.gitignore:23), dated 2026-04-19, covers gpui and iced only and cites phase 92 work: verified. CHANGELOG:161 records iced's fourth element as a break: verified. '79 call sites': unverifiable, since greps return 58 to 142 depending on whether core's SystemTheme::from_system is counted.

**Cost check:** For from_preset, `Loaded`'s mode and accessibility would only echo the caller's arguments, and egui's ThemeAtlas already carries both (atlas.rs:292, :311), so a core generic struct adds little there. The estimate leaves out the CHANGELOG Breaking Changes entries, the contracts and showcases that destructure the tuples, and the D1 consequence: any accepted break makes the release 0.7.0 and needs the maintainer's approval at triage. The agent is an untracked local file, so deleting it is no commit.

**Changes required:** (1) Delete the stale agent file locally, with no commit since it is untracked. (2) Rename the accessors to the model's field names, gated by scripts/check_connector_parity.py with a reasoned exceptions list. (3) Give a named `#[non_exhaustive]` struct only to `from_system` and `SystemThemeExt`, where the fields are detected outputs. Keep `from_preset` returning (theme, resolved), since its mode and preferences are inputs. Whether that struct is core's generic or one per connector is decided at triage. (4) Present the whole proposal as a D1 item for the maintainer's approval, bundled with the other breaking changes.

**Decision:** adopt the critic's form (round 5): a named `#[non_exhaustive]` result struct for `from_system` and `SystemThemeExt` only (core generic or per connector: decided at M5's triage), accessors renamed to the model's field names, `scripts/check_connector_parity.py` in check_release.sh with a reasoned exception list; `from_preset` keeps `(theme, resolved)`; the stale agent file deleted (untracked, in M1). Breaking, bundled with M1-T4-01 into 0.7.0. Lands across all three connectors in M5's fix phase, before M6's review.
**Status:** scheduled: M5 (all three connectors); agent file deleted in M1

### M1-T7-07 · no API break · Widget Info: three designs for one panel; generate iced's later

**Current design:** gpui: hand-written panels in `connectors/native-theme-gpui/examples/showcase-gpui/info/` (13,135 lines, 11 files), every claim cited and checked by the test-only `src/showcase.rs` (3,249 lines, `:1-20`); egui: generated from `mapping.toml` (`connectors/native-theme-egui/examples/showcase-egui/info.rs:1`, 1,154 lines); iced: `widget_tooltip` strings per demo block, three of the four sections and no citations (`connectors/native-theme-iced/examples/showcase-iced.rs:6569-6575`). The shared chrome and Basic-page elements are already one format from `docs/showcase-elements.toml` (CHANGELOG:84).

**Problem:** The per-widget half is hand-written twice in two formats and generated once. `docs/todo.md:2224-2238` already asks whether iced's can be generated from its contract rows. The hand-written form's upkeep shows in the gpui 0.7.1 move: `info/*.rs` edits in each font-related commit (064f8c9d touches three info files, 29ec6869 two).

**Evidence:** Lines above; widget-info rationale §2 (`docs/archive/todo_v0.5.9_widget-info-rationale.md:182-200`) names the drift of hand-written panels as the reason for gating them.

**Alternatives:** (1) *keep it*: Three designs; iced's panel stays uncited.; (2) *generate iced's and gpui's from a leaf→slot table*: The egui design everywhere; loses gpui's per-instance citations and the gates built for them.; (3) *generate iced's only*: From the iced contract rows once they carry a leaf per row (the converse-coverage proposal); gpui keeps its gated panels.

**Recommendation:** Generate iced's only, in M9, after the converse-coverage rows exist. gpui's 369 cited claims are machine-checked already (memory: Widget Info gates) and the open question at `docs/todo.md:1370-1380` is about those citations, which a regeneration would discard.

**Cost:** M9: iced showcase ~−400/+150 lines, one commit; depends on the converse-coverage proposal landing first.

**Unblocks:** `docs/todo.md:2224` answered; the three showcases show the same per-widget rows with the same provenance.

**Overlap with docs/todo.md:** `docs/todo.md:2224-2238` (iced per-instance Widget Info): the proposal adds the decision (generate) and its dependency (leaf names on the iced rows); `:1370-1380` unchanged.

**Verdict:** ADOPT WITH CHANGES — Generating iced's panel from its contract is the cheapest way to give it provenance, and it answers part of an open todo entry. It must rest on the checked table, and the entry's per-instance question remains.

**Case for the current design:** Rationale D13 left iced's per-instance info out of scope. gpui's panels are hand-written because the widget-info rationale §2 wanted per-instance claims with citations and gates (369 claims). egui's info is generated because egui has a manifest. Each form follows from what its connector has.

**Evidence check:** gpui info/ totals 13,135 lines: verified, but the directory has 16 entries, not 11 files. showcase.rs 3,249: verified. egui info.rs 1,154, generated from mapping.toml: verified. iced widget_tooltip with three sections and no citations (showcase-iced.rs:6569-6575): verified. 064f8c9d touches three info files and 29ec6869 two: verified. todo:2224-2238: verified.

**Cost check:** The ~−400/+150 is plausible for the content. todo:2224 also asks how iced can find the innermost hovered instance, and whether the citation gates can be shared; generating the text answers neither. The dependency is on whatever leaf-to-row table proposal 2 actually produces.

**Changes required:** Make the dependency the leaf-to-row table from proposal 2 in its mechanically derived form (see that verdict), not hand-written leaf strings. Record that the per-instance and innermost-hover half of todo:2224 stays open. Keep the work in M9.

**Decision:** adopt, in M9b after M1-T7-03 (round 5): iced's per-widget panel generated from the derived leaf-to-row table; the per-instance and innermost-hover half of docs/todo.md:2224 recorded as still open.
**Status:** scheduled: M9b

### M1-T7-08 · no API break · Widgets: separate crate for egui, in-crate for gpui and iced — keep

**Current design:** egui's theme-drawn widgets are `native-theme-egui-widgets` (3,767 lines), a separate published crate, because the connector's no-widgets charter forbids them (`docs/archive/todo_v0.6.0_egui-connector-rationale.md:2205`; `docs/archive/todo_egui-widgets-rationale.md:68-76`, "two failure domains under one version number"). gpui's are `src/widgets/` under feature `widgets`, on by default (`connectors/native-theme-gpui/src/widgets/mod.rs:1-2`), built on gpui-base's headless primitives (`:11-13`). iced's are free functions under feature `widgets` (`connectors/native-theme-iced/src/lib.rs:599-900`: `switch`, `radio`, `pick_list_handle`, `tab_indicator`).

**Problem:** One question, three answers: an application pays one dependency on gpui and two on egui, and the gates had to be extended for the extra crate (e2b87875; CHANGELOG:135, "a change there left both stamps fresh").

**Evidence:** Lines cited; the extra crate's own tests are `egui-widgets/src/tests.rs` (1,392 lines) with 18 inline test lines elsewhere.

**Alternatives:** (1) *keep it*: The asymmetry stays, with its two recorded reasons.; (2) *fold egui-widgets into the connector under feature widgets*: Mirrors gpui; one crate fewer to stamp and publish; breaks the egui-widgets paths and the charter.; (3) *split gpui's widgets into a crate*: Mirrors egui; breaks gpui paths; one more crate to publish and stamp.

**Recommendation:** Keep it. The egui split's reason holds for egui alone: its widgets paint every pixel themselves (`connectors/native-theme-egui-widgets/src/spinner.rs:155`), while gpui's delegate interaction and painting to gpui-base (`widgets/mod.rs:11-13`) and iced's are style functions. The application's extra line is one dependency; the gate cost is already paid. A fold would be a rename with no user-visible gain.

**Cost:** none

**Unblocks:** nothing

**Overlap with docs/todo.md:** none

**Verdict:** KEEP — The asymmetry has a recorded reason, and changing it breaks paths for no gain. The reviewer's 'painting is delegated' argument is inaccurate; the real reason is the egui connector's charter.

**Case for the current design:** egui-widgets-rationale.md:68-76 rejected putting the widgets in the connector under the egui connector's no-widgets charter (§3.22): 'two failure domains under one version number'. gpui's widgets predate that charter and gpui's design has none.

**Evidence check:** egui-widgets has 3,767 lines and tests.rs 1,392: verified. Rationale citations: verified. gpui `widgets` is on by default (mod.rs:1-2): verified. iced's switch, radio, pick_list_handle and tab_indicator (lib.rs:599-877): verified. CHANGELOG:135 and e2b87875: verified. 'gpui's delegate interaction and painting to gpui-base': partly wrong. mod.rs:11-14 says gpui-base owns interaction, while the widgets 'paint every part from the ResolvedTheme leaf' themselves.

**Cost check:** No cost. Folding or splitting would break public paths and the gates for no user-visible gain.

**Decision:** keep — the critic's verdict stands; not put to the maintainer (plan, M1 Step 4).
**Status:** closed

*Reviewer D-D notes:* Limits of this reading: line counts come from wc and a brace-matching script over `#[cfg(test)]` modules (test-only files were added by their `mod` declarations in each lib.rs); contract and archive documents were read only at the cited ranges, never end to end; no cargo command was run. Two claims in the testkit proposal are unverified in-session and should be checked at the first fix step: that cargo strips a path-only dev-dependency at package time, and whether iced already has `serde_json` as a dev-dependency for the leaf walk. The parity agent file is outside git, so its age is its mtime (2026-04-19). The context map's "5886 production lines" for iced did not subtract inline tests; my 4,111 does. For the maintainer: the entry-point proposal is an API break (tuples to one `Loaded<T>` struct plus ~8 accessor renames) and under D1 makes the release 0.7.0; the other proposals are additive or internal.

*Critic D-D notes:* No settled decision (D0-D7) is contradicted. Proposal 5 is breaking and needs the maintainer's approval under D1. Two proposals overlap docs/todo.md entries the reviewer did not cite: the colouriser switch is already docs/todo.md:476-490, and the text-scale method is :2086-2088, which counts two copies where there are three. Count errors found: egui-widgets uses composite_over 8 times (not 6); gpui widgets use `over(` 19 times (not 16); iced contract churn is 38 commits all-time (not 47 since 2026-09-21); gpui info/ has 16 entries (not 11 files); e2b87875 touched no compat.rs. Core's kde `over` assumes an opaque bottom, so it is not iced's formula. Checks ran through the shell, which behaves as zsh: `--include` globs failed silently in two greps, and those counts were redone without them.

### M1-T8-01 · no API break · Keep three full showcases; the shared gallery already exists

**Current design:** Three applications: gpui 41,435 lines (demo.rs 8,414, info/ about 14k, tests.rs 8,991), iced showcase-iced.rs 16,881 (tests from :12875), egui 14,189 (tests.rs 4,063). All three draw one shared chrome and Basic page named once in docs/showcase-elements.toml (196 `[[element]]` entries, header :1-9), embedded by each showcase (include_str in showcase-gpui/elements.rs, showcase-egui/elements.rs, showcase-iced.rs). Each is a test target (`test = true`, connectors/native-theme-gpui/Cargo.toml:102-107, -iced:64-69, -egui:82-88). Archived decision: docs/archive/todo_v0.5.9_showcase-app-rationale.md §3-§4 (D1, D11-D13).

**Problem:** The question is whether 72k lines of example code earn their keep. Counted uses per showcase: 10 PNGs + 1 GIF embedded in its README (connectors/native-theme-gpui/README.md:609-631, -iced:526-541, -egui:366-385; root README.md:14-18); the widget-coverage gate, which requires every upstream widget rendered or excepted (check_release.sh:672-683, ci.yml:195-203, canary :117-121); 181/61/60 tests run by plain `cargo test` (gpui tests.rs, iced, egui); Widget Info claims with line citations (gpui info/ 782 cited lines, iced 101; gate `every_colour_claim_is_read_at_the_line_it_cites`, native-theme-gpui/src/showcase.rs:2247); macOS runner checks only reachable there (screenshots.yml:112-121); parity dumps (`--dump-layout`). The showcases found real defects: docs/todo.md:2612 (font seen only in the CI capture), memory feedback_showcase_parity_visual (tests alone missed a chaotic egui showcase).

**Evidence:** scripts/check_widget_coverage.py run read-only today: gpui discovered 150, shown 114, excepted 36; iced_widget 38/30/8; iced_aw 9/9; egui 35/35, egui_ui 69/69, egui_extras 6/6. Those widgets are toolkit-specific, so no shared gallery can render them. What is shareable is already shared once: docs/showcase-elements.toml (1,897 lines) and docs/showcase-exceptions.toml (1,112). The chrome is still written three times: gpui chrome.rs 493 + app.rs 2,273 + inspector.rs 728; egui chrome.rs 2,015 + app.rs 887; iced menu_bar/toolbar/status_bar/inspector at showcase-iced.rs:4203, :4421, :5006, :6089. Coverage gate introduced with 33 missing widgets (commit 806c173b, 2026-09-21).

**Alternatives:** (1) *Keep it*: Three applications, one element list, coverage gate per toolkit.; (2) *One toolkit-neutral gallery crate*: A fourth UI layer over three toolkits; cannot satisfy the coverage gate (toolkit widgets differ) and adds the code the project goal says an application should not need.; (3) *Shrink to the Basic page and drop the coverage gate*: About 8k lines of gpui demo and the upstream-widget detector in the nightly canary go; README screenshots and parity are unaffected.

**Recommendation:** Keep it. The per-toolkit size is driven by the coverage gate (114 + 39 + 110 widgets rendered), which is the only mechanical detector of an upstream release adding a widget (dependency-canary.yml:117-121) and which no shared gallery could replace. The duplicated part (chrome, Basic page) is already pinned by one list and one comparator, so the triplication is bounded to about 6k lines of chrome and is itself the demonstration (rationale D1: a chrome widget is demonstrated by being the chrome).

**Cost:** None (no change).

**Unblocks:** Nothing; it keeps M9's review scope as three showcases plus the shared list.

**Overlap with docs/todo.md:** docs/todo.md:2621 and :2639 (shared-chrome follow-ups in all three showcases) show the triplication cost: each chrome change is three edits. No entry proposes a shared gallery.

**Verdict:** KEEP — I agree with the reviewer's own 'keep'. The size comes from the per-toolkit coverage gate, which no shared gallery can satisfy, and the shared part is already a single list. Nothing settled is contradicted.

**Case for the current design:** Rationale todo_v0.5.9_showcase-app-rationale.md §3.4 and D1: 'a chrome widget is demonstrated by being the chrome'. A sample copy beside the real chrome was rejected as two things to keep in sync. The widget-coverage gate (806c173b, started with 33 widgets missing) can only be met by an application per toolkit, because the widgets it finds are specific to each toolkit.

**Evidence check:** Sizes verified (gpui 41,435; iced 16,881; egui 14,189; gpui tests.rs 8,991). 196 [[element]] entries and include_str in all three showcases verified. test = true in all three manifests verified. Test counts verified: gpui 181 (162 #[gpui::test] + 19 #[test]), iced 61, egui 60. every_colour_claim_is_read_at_the_line_it_cites at gpui src/showcase.rs:2247 verified. Coverage script rerun today matches, except iced_aw, which shows 11 excepted, not 9/9 (shown 9). 806c173b with 33 missing verified. docs/todo.md:2612 (SF Pro) verified. The 782/101 cited-line counts were not recounted (unverifiable here).

**Cost check:** No change, no cost. The reviewer's figure of about 6k lines of triplicated chrome is fair.

**Decision:** keep — the critic's verdict stands; not put to the maintainer (plan, M1 Step 4).
**Status:** closed

### M1-T8-02 · no API break · Parity comparator is a hand tool; make it a release step

**Current design:** scripts/check_showcase_parity.py (1,944 lines; docstring :1-60) compares the three showcases' layout dumps and captures against docs/showcase-elements.toml, with 86 `[[parity.group]]` exceptions (docs/showcase-exceptions.toml:126-1112). Each showcase has a test that its dump holds every listed element (showcase-gpui/tests.rs:8704, showcase-egui/tests.rs:3906 and :4047, showcase-iced.rs tests `the_layout_dump_records_the_listed_elements`). The cross-showcase comparison itself is documented only in scripts/README.md:308-345.

**Problem:** No script or workflow runs the comparator: grep over scripts/, .github/ and Cargo manifests finds it only in scripts/README.md and two manifest comments (native-theme-iced/Cargo.toml:103, -gpui:82). check_release.sh (84 checks) has no parity section; generate_assets_release.sh (steps 1-6, :107-131) opens every showcase under six presets on the maintainer's KDE session but takes no dump. So the one gate that holds the three showcases to the same look (memory: the chaotic egui showcase was caught by side-by-side screenshots, not tests) runs only when someone remembers, against the memory rule that release gates must be mechanical (feedback_release_gates_mechanical).

**Evidence:** grep 'check_showcase_parity|dump-layout' across scripts, workflows, Cargo.toml, CONTRIBUTING.md: hits only scripts/README.md:308-345, Cargo comments and docs/todo.md:1363. Comparator landed 2026-09-29 (7f40285a) with four follow-up fixes to 2026-09-30 (04a39d3c, 47973625, 5095fbca), none wiring it in. docs/todo.md:1363 records that the dump is non-deterministic under the pointer, which a scripted run with the pointer parked would remove.

**Alternatives:** (1) *Keep it*: Run by hand during native-look passes, as in v0.6.0-rc1.; (2) *A step in generate_assets_release.sh*: Each Linux capture also writes `--dump-layout` beside its PNG into a scratch directory; one comparator call after step 5 fails the pipeline on an unexcepted difference. Dumps are not committed.; (3) *A CI job*: Needs a compositor for the captures on ubuntu runners; dumps alone (no captures) would lose the colour samples.

**Recommendation:** The release-pipeline step. The pipeline already runs the showcases under the six Linux presets with a settled frame (generate_screenshots_*.sh, DELAY=3), so the marginal cost is one flag per run and one comparator call; the comparator's exceptions file already carries the data. Fix docs/todo.md:1363 first or park the pointer outside the window in capture_window.sh.

**Cost:** generate_screenshots_{gpui,iced,egui}.sh about 5 lines each (add `--dump-layout <scratch>/<kind>-<preset>-<variant>.json`), generate_assets_release.sh about 15 lines (scratch dir, comparator call, `--content-offset` from capture_window.sh), scripts/README.md 5 lines. No new dependency (Pillow is already required, :54). One extra step per release run.

**Unblocks:** A mechanical parity verdict for every release's screenshots, and for M9's showcase review a baseline it can cite instead of re-running by hand.

**Overlap with docs/todo.md:** docs/todo.md:1363 (make the layout dump deterministic) is a prerequisite; this proposal adds the runner that makes the gate run.

**Verdict:** ADOPT WITH CHANGES — The gap is real and runs against the mechanical-gates rule. The static modes are the cheapest part of it, and they are ungated too.

**Case for the current design:** The comparator was built for native-look passes (v0.6.0-rc1) as a working tool with --merge and --write modes. Its exceptions cover adwaita and the hover and menu runs, which the release pipeline never captures. Its first run after the gpui-kit 0.7.1 changes is likely to need an exceptions refresh, so a hard gate could block a release on curation work.

**Evidence check:** Verified: nothing outside scripts/README.md:308-345, the two manifest comments and docs/todo.md:1363 calls the comparator, check_release.sh has no parity section, and generate_assets_release.sh steps 2-6 take no dump. Commit history 7f40285a to d4855c8d verified. Wrong: 83 [[parity.group]] entries, not 86. The pipeline covers 3 presets x 2 variants, not 'six presets', and no adwaita, hover or menu run. capture_window.sh does not park the pointer (todo:1363 still open).

**Cost check:** Roughly right for the runner, but it misses three things. The first run will probably need --merge to refresh the exceptions. todo:1363 must be fixed first. And --check-list and --self-test, which need no display and pass in seconds today ('196 elements ... 83 group(s): valid'; 'self-test passed'), are wired into no gate either.

**Changes required:** (1) Add `check_showcase_parity.py --check-list` and `--self-test` to check_release.sh and ci.yml now: no display needed, seconds to run. (2) Add the dump-and-compare step to generate_assets_release.sh only after todo:1363 is fixed. Scope it to the rest run of the three captured presets. The comparator already only reports exceptions that go unused, so a partial run cannot fail on adwaita, hover or menu keys. (3) Budget one exceptions refresh for its first run.

**Decision:** adopt the critic's form (round 6): `--check-list` and `--self-test` gated in check_release.sh and ci.yml now; the dump-and-compare step in generate_assets_release.sh after docs/todo.md:1363 is fixed, scoped to the rest run of the three captured presets, one exceptions refresh budgeted.
**Status:** M1 fix phase (static modes); M8 R3 (dump step)

### M1-T9-01 · no API break · Each release gate traces to a mistake that happened: keep

**Current design:** scripts/check_release.sh (924 lines; 84 checks, 37 min 22 s per the ledger Baseline) in 17 sections :253-849; .github/workflows/{ci,publish,screenshots,dependency-canary,docs}.yml; provenance stamp (scripts/update_provenance.sh, docs/assets/PROVENANCE.toml:8-15); compatibility stamp (scripts/update_compatibility.sh, docs/COMPATIBILITY.toml); docs.rs emulation (scripts/check_docsrs.py); CONTRIBUTING.md:17-22 and :184-195 name the process.

**Problem:** The question was whether each gate catches a class of mistake that occurred. Traced: provenance stamp (a4ebe239: 'the v0.5.8 screenshots were not regenerated because nothing enforced it'); publish job packages (4dcc39e9: run 34127726983 failed after four uploads); already-uploaded detection (17d2f377: run 34132076031); canary (27adf7cf; gpui-kit 0.6.4 removed `ThemeColor::tiles` in a patch, memory project_gpui_kit_064_v059); compatibility stamp (0704de54; CHANGELOG.md:171); feature combinations (b35432a9; de7ec5d8: docs failed without iced_aw); docs.rs emulation (6f2fa303; CHANGELOG.md:12,16: private-item links seen on docs.rs); package target dir (72ce91e7, 47a5e6cc); widget coverage (806c173b: 33 widgets missing at introduction); cross-target clippy (a6bcdcfe: 8 + 11 lints); artifact overwrite (e2fcb468); macOS display (ed45903d: every capture 1024 wide). Untraced: MSRV job (0072a095, no body), TODO scan, outdated-deps.

**Evidence:** git log on scripts/check_release.sh and .github/workflows/ (62 commits since 2026-03-14) and the commit bodies cited; CHANGELOG.md:12, :114, :135, :187, :244. The ledger's Baseline: 82 pass, 1 warn, 1 fail, all explained.

**Alternatives:** (1) *Keep it*: All gates stay; the three untraced ones are cheap hygiene.; (2) *Drop the untraced gates*: Saves seconds; MSRV is a published claim (README badges) that only the job verifies.

**Recommendation:** Keep it. Every gate that costs real time answers an incident recorded in git; the untraced three are seconds each and the MSRV job verifies a claim the READMEs make.

**Cost:** None.

**Unblocks:** Nothing; it records the traceability so M8 need not redo it.

**Overlap with docs/todo.md:** none

**Verdict:** KEEP — I agree with the reviewer's 'keep': the traceability holds on the commits I opened. The missing gate the question asks about (OS-version drift) is proposal 3.

**Case for the current design:** Every costly gate answers a recorded incident, and the cheap untraced ones verify published claims such as the MSRV badges.

**Evidence check:** Spot-checked a4ebe239, 4dcc39e9, 17d2f377, 27adf7cf (weekly at the time, nightly now), 0704de54, b35432a9, de7ec5d8, 6f2fa303, 72ce91e7, 47a5e6cc, e2fcb468, ed45903d and a6bcdcfe: all verified as described. 0072a095 has no body: verified. CHANGELOG lines 12, 16 and 171 verified. Ledger Baseline (84 checks, 82 pass, 1 warn, 1 fail, 37 min 22 s) verified. The map's '23 run_check calls' is static text: the loops make 84 at run time.

**Cost check:** No change, no cost.

**Decision:** keep — the critic's verdict stands; not put to the maintainer (plan, M1 Step 4).
**Status:** closed

### M1-T9-02 · no API break · Record and pin the screenshot runner's OS version

**Current design:** screenshots.yml runs the macOS captures on the floating label `macos-latest` and Windows on `windows-latest` (:15-40), names the files by preset (`macos-sonoma`, :141-142; `windows-11`, :158-159), and records nothing about the runner: no `sw_vers`, `systeminfo` or image-version step (grep over .github/, scripts/ and the capture code: none). PROVENANCE.toml holds version, commit, date and a source hash (:8-15). native-theme/src/presets/README.md:60-61 names no macOS version range for `macos-sonoma`. The READMEs label the result 'macOS Sonoma' (connectors/native-theme-gpui/README.md:623-625 and the iced and egui equivalents).

**Problem:** Nothing notices OS-version drift: when GitHub moves `macos-latest`, the 'Sonoma' screenshots silently show another macOS's frame, system font and SF Symbols (`--icon-set system`, :137). The plan's known gap (todo_code-review-plan.md:19-24) says the runner is macOS 26 citing screenshots.yml:63, but that line is a comment about an external repository's run; the repository holds no evidence of which version captured the committed PNGs either way. docs/todo.md:2612 shows a CI capture already revealed a runner-only font fact. Under D4 the macOS reader will choose the preset by running version, so a Tahoe runner capturing a file named `macos-sonoma` would be a contradiction the gate must see. `windows-latest` is a Server image, not Windows 11 (verify from the image, not assumed).

**Evidence:** screenshots.yml:15-40, :63, :137-146, :154-163; PROVENANCE.toml:8-15; docs/todo.md:2612 ('captured at 33fea0f7' on `macos-latest`); plan D4 lines 181-187; presets/README.md:60-61.

**Alternatives:** (1) *Keep it*: Floating labels, no record; the README's OS name is a belief.; (2) *Record only*: A step writes `sw_vers -productVersion` / Windows build into a sidecar uploaded with the artifact; update_provenance.sh writes it per platform into PROVENANCE.toml. Humans read it.; (3) *Pin and record*: Matrix entries name a versioned label per preset (whichever GitHub offers for the version the preset documents, checked at the time, never assumed); the recorded version is compared by `update_provenance.sh check` against the preset's documented range (presets/README.md), warning as the stamp does.

**Recommendation:** Pin and record. It is the only alternative under which D4's `macos-tahoe` screenshots (M2a Steps 8-12) and the existing Sonoma ones can coexist with the label telling the truth; the comparison is the same warning class the stamp already uses (check_release.sh:823-840). No platform value is proposed: the versions come from the runner and the preset's own documentation.

**Cost:** screenshots.yml about 25 lines (versioned labels, a version step, sidecar upload); update_provenance.sh about 30 lines (two fields, a range check); generate_assets_release.sh about 10 (copy the sidecars); PROVENANCE.toml two fields; presets/README.md one version range per platform preset (M2 supplies it). No dependency.

**Unblocks:** Trustworthy platform screenshots for M2a/M2b, the macos-tahoe preset's assets, and the README label.

**Overlap with docs/todo.md:** none in docs/todo.md; the plan's known gap at todo_code-review-plan.md:19-24 states the premise this proposal corrects (runner version unrecorded, not known).

**Verdict:** ADOPT WITH CHANGES — The drift is real and nothing records it, and D4's tahoe/sonoma split needs truthful labels. Windows cannot be pinned to the version its label names.

**Case for the current design:** macos-latest keeps the captures on a runner GitHub supports without upkeep. The screenshot content is drawn by the toolkit from preset values, so drift reaches only the window frame, the system font and SF Symbols.

**Evidence check:** Verified: screenshots.yml:15-40 uses macos-latest and windows-latest. Line 63 is a comment citing an external repository's run, so the plan's premise (todo_code-review-plan.md:19-24, 'taken on a macOS 26 runner') has no evidence in the repository. Verified: grep finds no sw_vers or systeminfo step. PROVENANCE.toml holds only version, commit, generated date and sources. presets/README.md names no version range. todo:2612 verified. 'windows-latest is a Server image' cannot be checked from the repository (the reviewer also says to verify it from the image).

**Cost check:** Plausible for macOS. It misses Windows: no hosted runner is Windows 11, so pinning cannot make the label true, and a range check against 'Windows 11' would always fail. It also does not say who reads the sidecar: generate_assets_release.sh would have to copy it out of the gh run download it already does.

**Changes required:** (1) Record the OS version for both platforms (sw_vers / Windows build into a sidecar, then into PROVENANCE.toml). (2) Pin only the macOS entries to versioned labels, chosen from what GitHub lists when the change is made. (3) Windows stays record-only. M2b decides what the windows-11 captures may claim, never assumed. (4) The range check is a soft warning and lands only once M2 supplies the preset ranges (D4). (5) Correct the plan's known-gap sentence at :19-24 in the same commit.

**Decision:** adopt the critic's form (round 6): record the runner OS version on both platforms (a sidecar, then PROVENANCE.toml fields); pin only the macOS matrix entries to the versioned labels GitHub lists at the time; Windows record-only, M2b decides what the captures may claim; the range check a soft warning once M2 supplies preset ranges; the plan's known-gap sentence corrected.
**Status:** M1 fix phase (files); the CI run is the maintainer's

### M1-T9-03 · no API break · Three gates exist on one side only: panic lints, MSRV, semver, canary

**Current design:** Strict panic lints run only locally (check_release.sh:548-563); no workflow names `unwrap_used` or strict-panic (grep over .github/: none). The MSRV job exists only in CI (ci.yml:129-139); check_release.sh never names 1.88 or MSRV. `cargo semver-checks` is in no gate although D1 says it 'decides mechanically' (plan :149-153; the ledger ran it by hand, Semver section). The nightly canary tests native-theme-egui (dependency-canary.yml:91-101) but never native-theme-egui-widgets (0 mentions, against 10 in ci.yml and 11 in publish.yml), though that crate forwards the egui features one to one (check_release.sh:629-630) and takes every egui 0.36.x patch (canary header :15-16).

**Problem:** A PR from a contributor (CONTRIBUTING.md:140-146; CI runs on pull_request, ci.yml:6) can merge a panic path CI never lints; the local gate cannot tell when a change raises the floor the README badges claim; the version decision D1 depends on a tool nobody runs automatically; and an egui patch release that breaks the widgets crate is first seen at release time instead of the evening it appears.

**Evidence:** Grep results above; dependency-canary.yml:91-101 and :80-88 list the crates; ledger Baseline 'Semver': 202 checks pass with 0.51.0 installed in M0; plan :107-108 'every fix is its own commit' with semver decided at stop point A.

**Alternatives:** (1) *Keep it*: Local and CI gates stay asymmetric; the sole maintainer runs check_release.sh before every commit anyway.; (2) *Mirror the four*: ci.yml clippy matrix gains the strict-panic lint list for library targets; the canary gains the egui-widgets clippy and test lines; check_release.sh gains `cargo semver-checks` as a soft check (tool installed) and a soft MSRV `cargo check --locked` on the 1.88/1.95 toolchains if present.; (3) *Mirror the cheap three only*: Skip the local MSRV check (two extra toolchains).

**Recommendation:** Mirror the cheap three: strict-panic into CI (the lint list is already a shell variable), egui-widgets into the canary, semver-checks soft in check_release.sh so D1's decision is printed on every run. Leave MSRV to CI; a second toolchain locally is not worth the download.

**Cost:** ci.yml about 15 lines; dependency-canary.yml 4 lines; check_release.sh about 10 lines (soft check, skipped with a message if the tool is absent). No dependency beyond cargo-semver-checks, already installed.

**Unblocks:** D1's version decision becomes a line in every check run; outside contributions are held to the no-panic rule mechanically.

**Overlap with docs/todo.md:** none

**Verdict:** ADOPT WITH CHANGES — The canary gap is real and costs four lines. The CI panic gap is narrower than claimed, but real. semver-checks per commit needs its cost measured first.

**Case for the current design:** Crate-level #![deny(clippy::unwrap_used, expect_used)] in native-theme and all four connectors, plus egui's wider deny set (indexing_slicing, panic, unreachable, todo, unimplemented), already fail CI's clippy under RUSTFLAGS=-Dwarnings. The sole maintainer runs the full local list before every commit.

**Evidence check:** Verified: .github has no strict-panic list, check_release.sh never names MSRV, semver-checks appears in no script or workflow, and the canary has 0 egui-widgets mentions (ci 10, publish 11). Wrong: 'a panic path CI never lints'. unwrap and expect are already denied crate-wide; the gap is the other lints (indexing_slicing, arithmetic_side_effects, panic, etc. outside egui). The title says three gates but lists four. Ledger semver result (202 pass) verified.

**Cost check:** The CI and canary line counts are fine, but CI must keep gpui soft as check_release does (continue-on-error). The estimate misses semver-checks' run time: it builds rustdoc JSON for the baseline and the current tree of every published crate. Nobody has measured that, and it adds to the 37 minutes.

**Changes required:** (1) egui-widgets clippy and test in the canary: adopt as proposed. (2) The strict list in CI's clippy job, gpui with continue-on-error. Correct the problem statement: the gap is the lints beyond unwrap and expect. (3) semver-checks as a soft check: time it on the first run, then put it in the release-only part if proposal 5 is adopted. (4) MSRV stays CI-only.

**Decision:** adopt the critic's form (round 6): egui-widgets in the canary; the strict-panic list in CI's clippy job (gpui with continue-on-error); `cargo semver-checks` as a soft check in check_release.sh, timed on its first run and placed in the release-only part under M1-T9-04; MSRV stays CI-only.
**Status:** M1 fix phase

### M1-T9-04 · no API break · check_release.sh has one mode: 37 minutes before every commit

**Current design:** CONTRIBUTING.md:17-22 and the plan (:108) require the full script before every commit; the memory rule says the same (feedback_run_prerelease_check). The script takes no arguments (no `$1`, `$#`, getopts or `case` on a flag: grep) and always runs all 17 sections, including release-only work: cross-target clippy and checks (:449-487), docs.rs emulation on nightly for 7 crates (:707-716), `cargo package` five times in a separate target directory (:736-760), outdated-deps, the two stamp checks.

**Problem:** The ledger measured 37 min 22 s for one run. The plan makes every fix its own commit (:107) and forbids parallel fix tasks (D7, :174), so a fix phase of N commits costs N x 37 min of serialised machine time before anything else can run; the review phases cannot overlap it either (:175). Nothing in the script distinguishes the checks a commit needs (fmt, clippy, strict-panic, tests, features) from those a release needs (packaging, docs.rs, stamps), so the only way to go faster is to skip the script, which the rule forbids.

**Evidence:** docs/todo_code-review-findings.md Baseline: '84 checks ... 37 min 22 s'; scripts/check_release.sh:153-154 (runners), :253-849 (sections), no argument handling; CONTRIBUTING.md:17-30; plan :107-108, :173-175.

**Alternatives:** (1) *Keep it*: One mode; no risk that a commit skipped a release check; 37 min per commit.; (2) *A `--commit` subset*: A flag that runs hygiene, check, features, fmt, clippy, strict-panic, tests and docs, and prints which sections it skipped; the full run stays the default and is required at the release steps CONTRIBUTING.md:184-195 already lists and at every milestone close (plan :12-13).; (3) *Rely on CI for the rest*: Push after the subset and wait for CI; slower feedback, and CI lacks the panic lints and packaging.

**Recommendation:** The subset flag, with the full run still the default and mandatory before the release commit and at each milestone's close. The maintainer set the every-commit rule and may veto; the counted cost is what changes the trade: the plan's fix phases alone will run the script dozens of times. Per-section timings are already printed, so the first subset run shows what it saves.

**Cost:** scripts/check_release.sh about 40 lines (a flag, a guard around the release-only sections, a summary line naming the skipped sections); CONTRIBUTING.md 5 lines; plan :108 one sentence; the memory rule one line. No dependency.

**Unblocks:** The wall-clock of M3-M9's fix phases; a review can start sooner after a fix phase.

**Overlap with docs/todo.md:** none

**Verdict:** ADOPT WITH CHANGES — The serialised cost across M3-M9's fix phases is real, but the proposed split drops the very gate that just found 19 lints, and its saving is unmeasured.

**Case for the current design:** The maintainer's rule (memory feedback_run_prerelease_check, the plan's Global Constraints line) names cargo package as part of the per-commit gate, after a 2026-04-15 packaging incident. a6bcdcfe shows cross-target lints pile up silently when they are not run on every change (8 + 11 lints).

**Evidence check:** Verified: 37 min 22 s (ledger), no argument handling, CONTRIBUTING.md:17-22. Minor errors: the script has 19 print_section sections, not 17, and docs.rs is at :725-745 and packaging at :746-778, not :707-716 and :736-760. 'Per-section timings are already printed' is true per check, but the ledger records no per-section breakdown, so the saving is not measured.

**Cost check:** About 40 lines is plausible. It misses changing the memory rule and the plan constraint (the maintainer's to change), and it moves cross-target clippy into the release-only part, which is exactly the class that slipped before M0.

**Changes required:** (1) The subset keeps cross-target clippy and checks, docs and widget coverage. It skips only packaging, docs.rs emulation, the security audit, outdated-deps and the two stamp checks. (2) The full run stays mandatory on the last commit of every fix phase and before the release commit. (3) Before deciding, take the per-check times from one existing run's output to show the saving. (4) The maintainer approves changing the rule at stop point A; agents do not.

**Decision:** adopt the critic's subset (round 6; the maintainer approved changing the every-commit rule): `--commit` keeps every lint, check, test, docs and widget-coverage gate and skips only packaging, docs.rs emulation, the security audit, outdated-deps and the two stamp checks; the full run stays mandatory on the last commit of every fix phase and before the release commit; per-check times from the next full run recorded; CONTRIBUTING, the plan's constraint and the memory rule updated.
**Status:** M1 fix phase, first task

### M1-T10-01 · no API break · docs/todo.md: closed items stay, nothing marks priority or next

**Current design:** One file, 2,646 lines, 24,823 words, four `##` sections (:5 Core API, :816 Toolkit Connectors, :2558 Publishing Prep, :2564 Post-1.0 / Deferred), 57 `###` and 6 `####` headings; 164 open and 42 done checkboxes. README.md:79 points readers to ROADMAP.md for 'what's planned next'; ROADMAP.md names v0.6.2 and v0.6.3 and links docs/todo_iced-full-theme-geometry.md and docs/todo_gpui-full-theme.md, never docs/todo.md. CONTRIBUTING.md does not mention docs/todo.md or the archive (grep: none).

**Problem:** Closed items are not cleared: 10 of 57 `###` sections have every checkbox done and remain (214 lines; :7, :153, :439, :545, :605, :715, :787, :2566, :2591, :2600), plus 42 done boxes elsewhere. No entry carries a priority or milestone; grep for priority, next release, v0.6.2 finds none ('Tier U' is a verdict). The largest unit is a `####`, 'Follow-ups from the v0.5.9 showcase and contract work' (:1370), 1,024 lines with 39 open items, inside the gpui connector section. 'Post-1.0 / Deferred' (:2564) mixes genuinely post-1.0 items (mobile readers :2575) with small pre-1.0 fixes (:2621 reload glyph, :2636, :2642, :2645). 381 `file:NNN` citations drift (the archive README's own warning). The plan's triage rule 'defer (docs/todo.md)' (:72 of the ledger template) and 'an item already there is not a new finding' (:104) need a file a reviewer can search by crate and milestone.

**Evidence:** Per-section counts computed over the file (python, checkbox markers); git log: 92 commits touch the file, 18 on 2026-09-24 alone; ROADMAP.md:1-123; README.md:79; docs/todo_gpui-full-theme.md:3 'Status: Superseded by v0.5.8'.

**Alternatives:** (1) *Keep it*: The file keeps growing; readers use ROADMAP for next and grep for the rest.; (2) *Prune, tag, link*: Delete the closed sections (their content is in CHANGELOG and the archive); move done boxes out; prefix every open item with a milestone tag (`[v0.6.2]`, `[1.0]`, `[post-1.0]`, `[upstream]`); split 'Post-1.0 / Deferred' into the two it is; a 10-line header saying how the file is ordered; ROADMAP and CONTRIBUTING link it.; (3) *One todo per crate*: Five files; the connector sections are already per crate, so this only moves the problem.; (4) *GitHub issues*: Loses offline reading and the file:line prose the maintainer writes; a sole maintainer gains little.

**Recommendation:** Prune, tag, link, in one docs commit before M3's triage starts, so every 'defer (docs/todo.md)' lands under a tag and reviewers can honour the 'already there' rule by search. Keep one file.

**Cost:** Docs only: about 214 lines removed, 164 items tagged, a 10-line header, ROADMAP.md and CONTRIBUTING.md one line each. One session's work; no dependency.

**Unblocks:** M3-M9 triage ('defer' has a tagged destination); M1's stop point A (the maintainer sees the pre-1.0 backlog as a list, not 2,646 lines).

**Overlap with docs/todo.md:** none; the file has no entry about itself.

**Verdict:** ADOPT WITH CHANGES — The evidence holds, and M3-M9's 'defer' and 'already there' rules need a searchable, tagged backlog. Deleting text and assigning priority are the maintainer's calls.

**Case for the current design:** Closed sections such as 'Icon theme detection falls back to hicolor' (:2600) carry the maintainer's decision text ('Decided 2026-09-24 ... neither; the library reports the failure'). The append-only rule (memory feedback_agents_append_only) forbids agents from deleting document content.

**Evidence check:** Verified: 2,646 lines and 24,823 words; 164 open and 42 done; 57 ### and 6 ####; four ## headings at :5, :816, :2558 and :2564; 10 fully done ### sections at the listed lines (about 214-224 lines depending on how they are counted); the #### at :1370 spans 1,025 lines with 39 open; no priority or v0.6.2 markers; ROADMAP, CONTRIBUTING and README never link todo.md; README.md:79 points to ROADMAP. 'Post-1.0 / Deferred' does hold pre-1.0 items (:2621 glyph, popover arrow, and others): verified. The citation count depends on the regex (I found 318, not 381). That the closed content is already in the CHANGELOG or archive is unverifiable.

**Cost check:** Fine for the mechanical part. It misses that tagging 164 items with a milestone is a priority call only the maintainer can make, and that deleting closed sections loses decision records unless they are moved somewhere.

**Changes required:** (1) Move the closed sections and done boxes to a docs/archive file instead of deleting them, so the append-only rule holds. (2) Agents propose the milestone tags; the maintainer approves them at stop point A. (3) Keep the rest as proposed: split Post-1.0/Deferred, add a header, add links from ROADMAP and CONTRIBUTING.

**Decision:** adopt with the critic's changes (round 7, under the maintainer's blanket approval): closed sections and done boxes move to a docs/archive file (not deleted); every open item gets a milestone tag proposed by the session; Post-1.0/Deferred split into the two it is; a header on how the file is ordered; ROADMAP and CONTRIBUTING link it.
**Status:** M1 fix phase

### M1-T10-02 · no API break · The archive index lags and the archive rule is only in memory

**Current design:** docs/archive/ holds 81 files, 121,482 lines; docs/archive/README.md indexes them in a table by version (:24-46) whose last row is v0.6.0 (:45). The rule that implemented plans move to docs/archive/ with archiving as the last task of every plan exists only in the agent memory (MEMORY.md:54, feedback_archive_implemented_plans.md); CONTRIBUTING.md's 'Where decisions live' (:149-158) names platform-facts, the registry and the inheritance rules, not the archive or the todo. Live plans keep the `todo_` prefix in docs/ (docs/todo_iced-full-theme-geometry.md 472 lines, docs/todo_gpui-full-theme.md 671) and archived ones keep it too (docs/archive/todo_v0.5.9_*), so the prefix no longer says which is live.

**Problem:** Four archived sets are absent from the index: todo_v0.6.0-rc1_gpui-kit-0.7-{rationale,spec,plan}, todo_v0.6.1_gpui-kit-0.7.1-{rationale,spec,plan}, todo_egui-widgets-{rationale,spec}, todo_gpui-widgets-spec (grep in README.md: 0 hits each). The plan (:39-41) says M1 design changes get their own rationale/spec/plan, which will be archived by a rule no repository file states; a contributor following CONTRIBUTING cannot know it. The rule is applied in practice (docs/todo.md cites 20 `docs/archive/todo_v0.` paths and 0 stale `docs/todo_v0.` ones), so the gap is discoverability, not discipline.

**Evidence:** ls docs/archive | wc -l = 81; wc -l = 121,482; grep counts above; docs/archive/README.md:45; CONTRIBUTING.md:149-158; MEMORY.md:54.

**Alternatives:** (1) *Keep it*: The index is a courtesy; git log finds the rest.; (2) *State the rule and add the rows*: CONTRIBUTING 'Where decisions live' gains two sentences (live plans in docs/todo_*.md, archived with their rationale under docs/archive/, index table updated as the plan's last task); the four rows are added.; (3) *Mechanical index*: The same, plus a soft check in check_release.sh listing docs/archive/*.md not named in docs/archive/README.md, in the spirit of the mechanical-gates rule.

**Recommendation:** Mechanical index. The maintainer's rule that gates must be mechanical (memory feedback_release_gates_mechanical) applies: a 6-line loop keeps the table honest, and the four missing rows show the human step already slipped twice.

**Cost:** CONTRIBUTING.md about 6 lines; docs/archive/README.md 4 rows; scripts/check_release.sh about 8 lines (soft). No dependency.

**Unblocks:** M1's close-out and every later milestone's archiving step have a stated rule and a gate.

**Overlap with docs/todo.md:** none

**Verdict:** ADOPT WITH CHANGES — The rule is stated nowhere a contributor reads, and the index has slipped. A soft check is cheap, provided it handles the index's own notation.

**Case for the current design:** The archive README is a courtesy index, and ROADMAP.md already links docs/archive/ and each release's design docs, so the files can be found.

**Evidence check:** Verified: 81 files and 121,482 lines; the last table row is v0.6.0 (:45); 0 index hits for the v0.6.0-rc1 gpui-kit-0.7, v0.6.1 gpui-kit-0.7.1, egui-widgets and gpui-widgets-spec sets; CONTRIBUTING.md:149-158 names neither the archive nor the rule; no repo CLAUDE.md states it, so the rule lives only in memory.

**Cost check:** About 8 lines for the check is too low. The index writes brace sets ('todo_v0.5.9_...-{rationale,spec,plan}.md'), so a plain grep loop would flag every v0.5.9 file. The check must expand the braces, or the rows must be rewritten flat.

**Changes required:** The soft check expands the brace notation, or the index rows are written one file each. Everything else as proposed.

**Decision:** adopt with the critic's change (round 7, blanket approval): CONTRIBUTING states where live and archived plans live and the archiving rule; the four missing index rows added; a soft check in check_release.sh that expands the index's brace notation.
**Status:** M1 fix phase

### M1-T10-03 · no API break · The memory index carries superseded project-status entries

**Current design:** ~/.claude/projects/-home-tibi-Rust-native-theme/memory/MEMORY.md, 60 lines, 45 topic files (1,430 lines); its 'Project Status' section (:45-60) holds 15 entries, one per piece of work, each with a status sentence. The plan names the memory index among the standing rules every review session reads (todo_code-review-plan.md:51-54).

**Problem:** Eight of the fifteen status lines are contradicted by later ones in the same section: :46 'version not yet bumped', :47 'await the maintainer's veto', :48 'not yet released', :49 'nothing pushed; screenshots + branch decision pending', :50 'screenshots pending, not pushed', :51 'nothing pushed', :52 'next: version bump', :57 'v0.6.0 not ready yet' — against :55 (v0.5.9 released 2026-09-25), :58 (v0.6.0 released 2026-10-01) and :59 (v0.6.1 released 2026-10-09; next v0.6.2). An agent reading the index at the start of M2-M9 gets 'nothing pushed' for work released two versions ago and must reconcile by itself; the append-only rule (:19) was written for project documents, and applied to the index it guarantees this accumulation.

**Evidence:** MEMORY.md:45-60 read today; the contradictions are within the file. This file is outside the repository, so no commit touches it.

**Alternatives:** (1) *Keep it*: Agents reconcile the entries by date each time.; (2) *One current-state line*: Collapse the 15 status entries into one 'Current state' line (released version, branch, next version, link to the plan) plus one line per still-open thread; the topic files stay as history.; (3) *Move status out of memory*: ROADMAP.md and CHANGELOG.md already carry what shipped; the index keeps only rules and references.

**Recommendation:** One current-state line, kept current at each stop point, with the topic files untouched. The maintainer owns the memory and the append rule, so this is a request, not a change an agent makes on its own.

**Cost:** One file outside the repository, about 15 lines replaced by about 4; the 45 topic files unchanged.

**Unblocks:** Every M2-M9 session starts from a true state without re-deriving it from CHANGELOG.

**Overlap with docs/todo.md:** none

**Verdict:** ADOPT WITH CHANGES — The contradictions are real and every later milestone session reads them. The proposal correctly treats this as a request to the maintainer.

**Case for the current design:** The status entries are dated history, and an agent can reconcile them by date. The memory belongs to the maintainer, and the append-only habit keeps agents from rewriting it.

**Evidence check:** Verified: 60 lines, 15 Project Status entries, and the eight stale phrases at the cited lines, all contradicted by :55, :58 and :59. Minor: the 45 files include MEMORY.md itself. Missed: :60 says the review plan is 'uncommitted, awaiting review', but it was committed in 10e037b1. :45 still points to docs/todo_v0.5.8_... (archived). :54 is a rule filed under Project Status.

**Cost check:** Right: one file outside the repository, with the topic files unchanged.

**Changes required:** Also fix :60 and :45's stale path, and move the :54 archive rule to Workflow. The maintainer approves or makes the edit; it is not an agent edit.

**Decision:** adopt (round 7, blanket approval; the maintainer approved the edit): the memory index's Project Status collapses to one current-state line plus one line per open thread; :60 and :45 corrected; the archive rule moved to Workflow; topic files untouched.
**Status:** M1 (memory file, outside the repository)

*Reviewer D-E notes:* Limits of this reading: (1) The 84-check count is the ledger's; the script has 23 run_check call sites, the rest come from per-crate loops (7 crates), so the counts agree. (2) The Widget Info claim counts are proxies (lines in the info modules citing `.rs:NNN`): gpui 782, iced 101, egui 7; egui's claims may live in connectors/native-theme-egui/mapping.toml, which I did not read, so the egui figure is not a measure of its gate depth. (3) I ran scripts/check_widget_coverage.py read-only (it calls `cargo metadata`, no build) to get the shown/excepted counts. (4) I did not read the archive beyond the showcase-app rationale sections the map named and the archive README; I did not read the whole todo, only its headings, counts and the entries the map cited. (5) One premise in the plan's own text deserves the maintainer's eye: todo_code-review-plan.md:21-22 says the Sonoma screenshots are taken on a macOS 26 runner citing screenshots.yml:63, but that line is a comment about another repository's run; the repository records no runner version at all. The T9 pinning proposal treats the version as unknown, not as 26. No D0-D7 decision is contradicted; the subset-mode proposal (T9) touches the maintainer's every-commit rule and is offered with its counted cost for a veto. Question for the maintainer: is docs/todo.md meant to be read by contributors (then CONTRIBUTING should link it) or only by the maintainer and agents?

*Critic D-E notes:* I checked all nine proposals read-only. The only things run were check_widget_coverage.py, check_showcase_parity.py --check-list and --self-test, greps, wc and git show. Three findings go beyond the reviewer's proposals. (1) The parity comparator's static modes (--check-list, --self-test) pass in seconds and need no display, yet no gate runs them. This is the cheapest part of proposal 1. (2) The plan's known gap at todo_code-review-plan.md:19-24 says the macos-sonoma screenshots are taken on a macOS 26 runner, citing screenshots.yml:63. That line describes an external repository's run, so the claim has no evidence in the repository. (3) Crate-level #![deny(clippy::unwrap_used, expect_used)] already holds in native-theme and all four connectors under CI's -Dwarnings, so the CI panic gap in proposal 4 is narrower than stated. Proposals 5, 6 and 8 change rules or documents the maintainer owns (the per-commit check rule, the append-only rule, the memory index), so they belong at stop point A for approval, not to agents.


### Fix plan (Step 6)

Tasks for M1's fix phase, in order: F1 first because it shortens every later gate; the two
structural changes next (F2, F3); then the small code changes; docs last. Every task is its
own commit (several for F12), each code commit gated by `./scripts/check_release.sh
--commit` once F1 exists, the last commit of the phase by the full run. Items decided for
later milestones are listed in the plan's milestone sections (*Scheduled from M1*). The
`docs/todo.md:N` citations in the entries above refer to the file at `a7b3a4cb`, before
F13 reorders it.

- [ ] **F1 · M1-T9-04 · `check_release.sh --commit`.** `scripts/check_release.sh` gains a
  `--commit` flag that skips the packaging, docs.rs-emulation, security-audit,
  outdated-deps and stamp-check sections and prints the skipped sections by name;
  `CONTRIBUTING.md:17-30`, the plan's Global Constraints line and the memory rule say when
  each mode is required. Gate: `./scripts/check_release.sh --commit` passes on the tree
  before any other fix and names what it skipped; the full run is unchanged in what it
  runs; the subset's wall-clock recorded here. Commit: `chore(release-check): a --commit
  subset for per-commit runs; the full run stays for fix-phase ends and releases`.
- [ ] **F2 · D6 · `material` → `material-3`.** As the plan's M1 Step 7 task, with its test.
- [ ] **F3 · M1-T4-01 · remove the `-live` tier.** Delete the four `-live` TOML files;
  `presets.rs` (PRESET_INFOS live entries, their tests), `pipeline.rs` (the live merge,
  `is_live`/`live_name`, `select_reader`'s names), `lib.rs` (the re-export, the
  `live_preset` field, `with_overlay`, the test fixtures), `documented_sizes.rs` (the live
  rows and `reader_constants`' live half), `presets/README.md` (the two-tier section
  becomes one paragraph; the reader-provided-fields list stays as reader documentation),
  connector contract comments, `showcase-gpui/info/icons.rs`, CHANGELOG `[Unreleased]`
  *Breaking*, `docs/todo.md:2579-2580` closed and the preset half of `:2594-2600`. Gate: a
  new test `Theme::preset("kde-breeze-live")` is `Err`; `cargo test -p native-theme` and the
  connectors' tests pass; `cargo semver-checks -p native-theme --baseline-version 0.6.1`
  reports the removal (recorded as D1's evidence; then `rm -rf target/semver-checks`).
  Commit: `refactor(presets)!: the -live preset tier is gone; readers merge over the
  platform preset`.
- [ ] **F4 · M1-T5-01 · version ruling and preset-keyed gate.** A ruling paragraph in
  `platform-facts.md`'s Conventions; `documented_sizes.rs`'s `Platform` enum keyed by preset
  (variants named by preset, rows unchanged); `docs/todo.md:2343` appended with the
  OS-version axis. Gate: `cargo test -p native-theme` and `--test platform_facts_xref`
  pass. Commit: `docs(platform-facts): a version-tagged cell belongs to that version's
  preset; the size gate is keyed by preset`.
- [ ] **F5 · M1-T6-01 · `is_dark` on `Dual`.** `lib.rs` enum and its two exhaustive
  patterns (`lib.rs`, `gnome/mod.rs`), `pipeline.rs` deriving `mode` from the flag,
  `macos.rs` filling it from the uncached `detect_is_dark()`, docs, the test constructions.
  Gate: a pure test with a fake `Dual` reader asserting `mode` follows `is_dark` fails
  before and passes after; a `cfg(target_os = "macos")` test `from_system().mode ==
  detect_is_dark()`; both cross-target clippy runs clean. Commit: `fix(native-theme): a
  dual-variant reader reports the active mode; a dark Mac no longer starts light`.
- [ ] **F6 · M1-T6-02 · pollster everywhere.** `Cargo.toml` (pollster to `[dependencies]`),
  `lib.rs` (the two noop pollers and the error arm deleted), `pipeline.rs` doc. Gate:
  cross-target clippy clean on both targets; `cargo tree -p native-theme --target
  x86_64-apple-darwin -e normal | grep pollster`; tests pass. Commit: `refactor(native-theme):
  the sync entry points block on pollster on every platform`.
- [ ] **F7 · M1-T3-01 · `Theme::resolve_with`.** `model/mod.rs` (public `resolve_with`,
  the doc at :442-445 rewritten), `resolve/context.rs` (a constructor taking a known
  `font_dpi`), `pipeline.rs:70-73` (reuses the reader's DPI), egui `ext.rs`, `icons.rs`,
  `plugin.rs` (one context per call), `tests/timings.rs` (a `resolve_with` row), CHANGELOG.
  Gate: a doctest on `resolve_with`; a unit test that the pipeline's context constructor
  does not run DPI detection when given a DPI; the timing probe shows `resolve_with` on a
  `for_tests()` context far below `resolve()` (recorded, not asserted). Commit:
  `feat(native-theme): Theme::resolve_with takes the caller's context; the pipeline reuses
  the reader's DPI`.
- [ ] **F8 · M1-T3-03 · registry check.** A test comparing `docs/property-registry.toml`'s
  widget and `_structures` key sets with the `WidgetFieldInfo`/`FieldInfo` inventory both
  ways; the four stale `[defaults]` keys and the spacing note removed; the derive emits
  each `inherit_from` target and the `[uniform]` test compares targets instead of the
  hand-kept lists (`inheritance.rs:675-798`). Gate: the key-set test fails on the current
  registry and passes after the fix; `cargo test -p native-theme` passes; `docs/todo.md`'s
  `[uniform]` target entry closed. Commit: `test(native-theme): the property registry's keys
  are checked against the derive inventory, and uniform-inheritance targets against the
  macro`.
- [ ] **F9 · M1-T6-03 + M1-T7-01 · the text-scale guard in core.**
  `AccessibilityPreferences::text_scale_factor()` and `scaled(size)` in `lib.rs`; iced,
  gpui and egui call them; a receiver table (reduced motion, reduced transparency, high
  contrast: what the connector applies, what the application forwards) in each connector's
  crate docs; `docs/todo.md:2086-2088` closed. Gate: core unit tests (NaN, 0, negative,
  infinity give 1.0; 2.0 gives 2.0); the connectors' existing text-scale tests pass
  unchanged; net deletion in the connectors. Commit: `refactor: the text-scale guard lives
  once in core; the connectors call it`.
- [ ] **F10 · M1-T6-05 · a cached accessibility accessor.** `detect.rs` (a
  `DetectionContext` field, a public cached getter, `invalidate_caches` clears it),
  `pipeline.rs` fills it from `from_system_inner`; connector README snippets and the
  showcases' preset-switch paths use it. Gate: a test that a second call does not run the
  reader; `tests/timings.rs` gains the cached call; tests pass. Commit: `feat(native-theme):
  a cached accessibility accessor for preset switches; from_system stays a fresh read`.
- [ ] **F11 · docs · M1-T1-03, M1-T2-01, M1-T1-04, D1.** The naming rule in
  `presets/README.md` and `new-os-version-guide.md`; the `ios` and `material` sentences in
  `native-theme/README.md` and `presets/README.md`; a `docs/todo.md` entry for
  `from_system_with_preset`; ROADMAP renumbered for 0.7.0; CHANGELOG. Gate: docs only;
  `cargo test --doc -p native-theme`. Commit: `docs: platform presets are named by
  generation; what the ios and material presets are; the roadmap renumbered for 0.7.0`.
- [ ] **F12 · gates · M1-T8-02, M1-T9-03, M1-T10-02, M1-T9-02.** `check_release.sh`
  (`check_showcase_parity.py --check-list` and `--self-test`; `cargo semver-checks` soft,
  timed; the archive-index check expanding brace notation), `ci.yml` (the parity static
  modes, the strict-panic lint list with gpui `continue-on-error`), `dependency-canary.yml`
  (egui-widgets), `screenshots.yml` (versioned macOS labels from GitHub's current list, an
  OS-version sidecar on both platforms), `update_provenance.sh` and `PROVENANCE.toml` (the
  version fields), `generate_assets_release.sh` (copies the sidecars), `CONTRIBUTING.md`
  (where plans live, the archiving rule), `docs/archive/README.md` (four rows). Gate:
  `./scripts/check_release.sh --commit` passes with the new checks; the YAML parses; the
  archive check reports nothing missing. Two or three commits.
- [ ] **F13 · M1-T10-01 · `docs/todo.md`.** Closed sections and done boxes move to a new
  `docs/archive/todo_closed.md` with their decision text; every open item gets a milestone
  tag (`[v0.7.0]`, `[1.0]`, `[post-1.0]`, `[upstream]`); *Post-1.0 / Deferred* split;
  a header on the ordering; ROADMAP and CONTRIBUTING link the file. Gate: no `- [x]` left
  in `docs/todo.md`, every one present in the archive file; every open item tagged
  (a grep count equals the open-item count). Commit: `docs(todo): closed items archived,
  open items tagged by milestone, the deferred section split`.
- [ ] **F14 · M1-T10-03 · the memory index** (outside the repository, no commit).
- [ ] **Close (stop point B).** The full `./scripts/check_release.sh`, both cross-target
  clippy commands, `cargo semver-checks` for `native-theme` (recorded), the timing probe
  re-run, the ledger's statuses, the project memory, `~/.claude/scripts/sweep-target.sh`.

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
