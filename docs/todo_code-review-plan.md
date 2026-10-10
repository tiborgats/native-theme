# Whole-codebase code review — plan

> **For agentic workers:** each milestone's review phase runs as one Workflow (the maintainer
> opted in, 2026-10-10); its fix phase uses superpowers:subagent-driven-development with
> `implement` agents. Steps use checkbox (`- [ ]`) syntax for tracking. Before starting,
> read this file and `docs/todo_code-review-findings.md`, then resume at the first unchecked
> step (see *Resuming* below).

**Goal:** review native-theme's design decisions, check every platform's theme details
against the OS versions in use today (macOS, Windows, iOS, Android, KDE, GNOME), then read
the whole code once, crate by crate, for bugs, simplification, speed and API design, and
fix what each step proves, in milestones that each end with every gate green, so the work
can stop at any milestone boundary.

**Why the design and the platform data come first:** polishing code that a design change
then replaces is wasted, and a corrected platform fact can add or change model fields that
the code review would otherwise read twice. Known gaps on 2026-10-10:

- No reader asks the OS for its version: `platform_preset_name()`
  (`native-theme/src/pipeline.rs:408-437`) returns `windows-11` on every Windows (10, 11 or
  a later one) and `macos-sonoma` on every macOS, and the CI screenshots of `macos-sonoma`
  are taken on a macOS 26 (Tahoe) runner (`.github/workflows/screenshots.yml:63`), whose
  Liquid Glass design changed control shapes and sizes; `platform-facts.md` records Tahoe
  only for the window corner radius (`:173`, `:1155`).
- The `ios` preset has no `platform-facts.md` column (`native-theme/src/presets/README.md`)
  and names no iOS version; iOS 26 brought Liquid Glass too.
- The `material` preset is Material 3 as material-web implements it (M3 tokens `v0_192`);
  Android's own implementation (Compose Material 3, the Views library), Material You's
  dynamic colour and Material 3 Expressive are not compared against it.
- iOS and Android readers are deferred past 1.0 (`docs/todo.md:2575-2577`).

**Why now:** about 92k lines of production-plus-test Rust in `src/` grew over v0.5.8–v0.6.1,
reviewed only branch by branch; v0.6.1's ad-hoc finds (a 1.9 s Adwaita theme switch from
13–24 ms per icon miss, a fixed 50 ms subprocess poll, docs linking private items) suggest
more of the same; pre-1.0 is the cheapest time to change the API.

**Architecture:** first a design review (M1) and a platform-fidelity audit (M2, one part per
platform); then one milestone per region of the code, core first (every connector depends
on it), connectors after, then cross-cutting work. A design change or a new preset too
large for a milestone's fix phase gets its own rationale, spec and plan documents, as
earlier releases did, and the code milestones that it would rewrite wait for it or skip
the code it replaces. Each milestone: inventory → review →
verify → triage → fix → close. Review agents are read-only; every finding needs evidence
and an independent verifier's verdict before it can be fixed. Fixes land before the next
milestone's review starts, so reviewers always read current code.

**Tech stack:** Rust 2024, workspace MSRV 1.88.0 (gpui and egui connectors 1.95.0),
cargo-nextest 0.9.138, cargo-machete 0.9.2, cargo-bloat 0.12.1, cargo-semver-checks
(installed in M0).

**Spec:** no separate spec. The scope is the design, the platform data and the code; the rules are the project's
standing rules in `CLAUDE.md`, the memory index, `docs/platform-facts.md`,
`docs/property-registry.toml`, `docs/inheritance-rules.toml`, and the design decisions in
`docs/archive/*-rationale.md`.

## Milestones at a glance

| M | Scope | Production lines read | Review agents | Stop points |
|---|-------|----------------------:|--------------:|-------------|
| M0 | Baseline: gates, timings, lint and dependency inventory, semver tool | — | 0 (inline) | end |
| M1 | Design review: the decisions behind the model, presets, OS versions, readers, connectors, showcases, release process, mobile scope | design docs + code sampled | 5 + 5 critics | A, B |
| M2 | Platform fidelity, one part per platform: M2a macOS (with `macos-tahoe`, D4), M2b Windows, M2c iOS (small, D5), M2d `material-3` and `material-3-expressive` (D6), M2e KDE and GNOME currency | `platform-facts.md` (1.8k lines) + 6 platform presets | 2–3 + verifiers per part | A, B per part |
| M3 | Core, part 1: model, resolution, pipeline, presets loading, derive macro | ~11k | 5 + 5 verifiers | A, B |
| M4 | Core, part 2: platform readers, detection, watching, fonts, icons, build crate | ~11k | 5 + 5 verifiers | A, B |
| M5 | iced connector | ~6k | 3 + 3 verifiers | A, B |
| M6 | gpui connector and its `widgets` | ~13k | 5 + 5 verifiers | A, B |
| M7 | egui connector and `native-theme-egui-widgets` | ~13.5k | 5 + 5 verifiers | A, B |
| M8 | Cross-cutting: connector parity, shared code, test quality, scripts and CI, docs | ~8k scripts + sampling | 4 + 4 verifiers | A, B |
| M9 | Optional: the three showcases (72k lines, 14k of them tests), split M9a/b/c | ~58k | 3 per showcase + verifiers | A, B per part |

*Production lines* exclude in-file `#[cfg(test)]` modules and the connectors' test-only
`contract`, `compat`, `mapping_*`, `install_tests`, `style_diff` and `tripwires` modules.

**Stop points.** *A*: findings verified, triaged and committed to the ledger; no fix
started. *B*: the milestone's fixes landed and the close gate is green. Stopping at either
leaves the branch consistent; stopping inside a fix phase is safe too (every fix is its own
commit), but resume then means re-running that fix's gate first.

**Cost.** Token figures are not known before M1 runs; agent counts are above. My guess, to
be replaced by the measured cost in the ledger: a region reviewer reading ~2.5k lines costs
150–300k tokens, a verifier 80–150k, so a 10-agent review phase is roughly 1.5–2.5M tokens.
Suggested caps: M1 **+2000k**; each M2 part **+1000k** (web research costs more per
agent than reading local code, but each part has fewer agents); M3 **+2500k**. Fix phases are sized at triage, once the
findings are known; the maintainer can defer low-severity findings to keep a phase small.
After M1, M2a and M3, the ledger's *Calibration* section records the measured cost of each
kind of milestone and the estimate for the rest.

## Global Constraints

- NEVER invent a value. A finding that changes a preset value, a platform constant or a
  `platform-facts.md` row must cite an upstream source; without one it is dropped.
- No runtime panics, no new `unsafe`. native-theme is `#![deny(unsafe_code)]`; the platform
  FFI already allowed (44 `unsafe` blocks on 2026-10-11, in `macos.rs`, `sficons.rs`,
  `watch/macos.rs`, `watch/windows.rs`, `windows.rs`, `winicons.rs`) is reviewed for
  soundness in M4, not extended. A fix that needs a panic or more `unsafe` stops and asks.
- Connectors follow theme data; a connector ignoring a resolved value is a bug, never a
  question.
- Never mix icon themes or sets; an icon not found returns `None`.
- Read `docs/archive/` spec and rationale documents before proposing a design change to icon
  loading or theme resolution. Design decisions are reviewed in M1; in M3–M9 a decision
  recorded in a rationale, or settled in M1, is not a bug unless the code contradicts it.
- A platform fact is checked against the OS version it claims; a row without a version
  gets one, or a ❓, never a guess. A new preset is built only from genuine sources.
- An item already on `docs/todo.md` is not a new finding; reviewers cite the entry instead.
- Agents updating an existing document read it first and only append.
- Reviewers and verifiers are read-only: no Edit, no Write, no commits.
- Every fix is its own commit, author the maintainer, no AI attribution trailer.
- Run `./scripts/check_release.sh` before every commit that touches code.
- Build into the project's own `target/`; no extra target directory unless a step says so.
- Breaking public-API changes need the maintainer's approval at triage (decision D1 below).
- Nothing is pushed, tagged or published by an agent.

## Review Focus

The inputs and conditions most likely to bite a person using native-theme that no existing
test exercises well. Each line is a mandatory question for the reviewer of the owning region
(named in brackets); a confirmed gap gets a pinning test in that milestone's fix phase.

1. **A desktop with missing or partial configuration** — no `kdeglobals`, a `kdeglobals`
   with malformed lines, GNOME without the settings portal, a headless session, a
   subprocess that hangs or is absent. Expected: a preset fallback or an `Err`, never a
   panic, never a hang past the documented timeout. [M4 readers, M4 detection]
2. **Repeated runtime theme switching** — switching presets and system themes hundreds of
   times in one process. Expected: memory and time per switch stay flat (the icon index is a
   known process-lifetime cache: confirm it is bounded by the number of installed themes).
   [M3 pipeline, M4 icons, M5–M7 connectors]
3. **Code compiled but never run on Linux** — the macOS and Windows readers, watchers and
   icon loaders are only `cargo check`ed by `check_release.sh`; CI runs their tests on the
   Windows and macOS legs. Expected: the reviewer reads every `cfg(target_os = …)` branch
   as carefully as the Linux one. [M4 all regions]
4. **Extreme user settings** — text scale 2× and above, high contrast, reduced motion,
   a very small or very large font size, a scale factor below 1. Expected: no NaN, no
   negative or zero size reaches a toolkit, no layout value overflows. [M3 resolution,
   M5–M7 connectors]
5. **User-supplied TOML overlays with odd values** — NaN, infinity, negative sizes, unknown
   keys, a colour with out-of-range components, an overlay that sets only one variant.
   Expected: a validation `Err` naming the field, or a documented clamp. [M3 model, M3
   resolution]
6. **An OS version the preset was not made for** — Windows 10 or a Windows after 11, macOS
   15 or 26 under `macos-sonoma`, iOS 26 under `ios`. Expected: the values a user sees are
   that version's, or the preset's documentation says which version it is and the library
   reports the mismatch. [M1 topic T1, M2 every part, M4 readers]

## Decisions taken in advance (settled by logic; the maintainer may veto)

- **D0 — branch.** All work happens on local branch `v0.6.2-rc1` from `main` (`92893ce1`);
  the next release after v0.6.1 is v0.6.2 on the ROADMAP. If D1 settles on 0.7.0, the branch
  is renamed `v0.7.0-rc1`.
- **D1 — version (decided at the first stop point A that accepts a breaking API change).** Cargo treats 0.6.x → 0.6.y as compatible,
  so any accepted breaking API change makes the release 0.7.0, whatever the project's pre-1.0
  stance. `cargo semver-checks` against 0.6.1 decides mechanically. The ROADMAP's renumbering
  (iced geometry and upstream PRs move back one version) is written only after the maintainer
  approves it.
- **D2 — models and effort (maintainer, 2026-10-11).** Fable 5.1 at `xhigh` where the work is
  judgment and the volume is low: M1's design reviewers, M3–M9's API reviewers, the final
  whole-branch review at close-out, and the orchestrating session of M1 (a fresh session
  started with `/model`). Opus 5.5 at `high` for everything else: M1's critics (a second
  model arguing against the first), the region reviewers and all verifiers, M2's
  researchers, the orchestrating sessions of M0, M2–M9. Haiku for inventory and search
  stages. Fixes in `implement` agents (Opus), per `CLAUDE.md`.
- **D7 — parallelism (2026-10-11).** Only where it costs no quality; the token total is the
  same, it arrives sooner:
  - M0's cargo steps run one after another (one `target/`, cargo's lock); its non-building
    steps (the `cargo-semver-checks` install, `cargo machete`, `cargo tree`, the ledger) run
    beside them in the background;
  - M2's Steps 1–6 of every part may run alongside M1 (they build nothing and need no M1
    decision); what M2 does with a version gap, and Steps 8–15, wait for M1's stop point A;
  - M5, M6 and M7's review phases may run as one Workflow, after M3's and M4's fixes landed;
  - never in parallel: fix tasks (one `target/`, `check_release.sh` per commit), and a
    review with the previous milestone's fix phase (it would read code about to change).
- **D3 — preset values belong to M2, not to the code milestones.** M2 checks the six
  platform presets (`macos-sonoma`, `windows-11`, `ios`, `material`, `kde-breeze`, `adwaita`,
  and their `-live` bases) against `platform-facts.md` and `platform-facts.md` against the
  platforms; M3–M9 read the code that loads and resolves them. The color-scheme presets
  (catppuccin, dracula, …) copy their upstream palettes and are out of scope.
- **D4 — macOS presets (maintainer, 2026-10-10).** `macos-tahoe` (macOS 26, Liquid Glass) and
  its `-live` base are built in this plan (M2a Steps 8–12), and the macOS reader picks the
  preset by the running macOS version. Sequoia (15) gets no preset of its own unless M2a
  finds that a value the model carries differs from Sonoma's; `macos-sonoma` then documents
  that it covers 14–15. Golden Gate (macOS 27, released 2026-09-14) waits until GitHub offers
  a macOS 27 runner image: until then a Mac on 27 gets `macos-tahoe`, documented as such,
  and M2a's close appends a `docs/todo.md` entry for it.
- **D5 — mobile scope (settled by logic, 2026-10-10).** No iOS or Android readers before 1.0:
  of the three toolkits, only egui's eframe lists Android among its platforms and none lists
  iOS; iced lists Windows, macOS, Linux and the Web; gpui reaches mobile only through a
  third-party platform layer. A mobile app also needs touch-sized controls and mobile
  navigation that a theme does not give. The `ios` and `material` presets stay as looks a
  desktop app can choose; the README says so. M2c shrinks accordingly; M2d does not (D6).
- **D6 — two Material presets, `material-3` and `material-3-expressive` (maintainer,
  2026-10-10: "if they are visually different, we must have both", exact to their
  definition in every detail, screenshots measured where the definition is silent).**
  They are visually different. At the pinned sources `platform-facts.md` already cites
  (material-web `cbd34a8`, androidx `1e67768`, 2026-09-28):
  - Expressive adds button sizes XS–XL (the default `button-small`: 40 high, padding 16
    against the baseline's 24, a *round* and a *square* shape, `corner-medium` 12, pressed
    morphing to `corner-small`, selected swapping the two;
    `latest/sass/_md-comp-button-small.scss`), button groups, split buttons, toolbars, a
    shape-morphing loading indicator, wavy progress indicators, 35 decorative shapes, the
    emphasized type scale (`_md-sys-typescale-emphasized.scss`) and spring motion;
  - `MaterialExpressiveTheme` changes the colour scheme (`expressiveLightColorScheme`: the
    four `on-*-container` roles at tone 30 instead of 10, Compose `ColorScheme.kt:1172-1179`)
    and the motion scheme (`MotionScheme.expressive()`, `MaterialTheme.kt:243-262`).
  
  The present `material.toml` matches neither as Android renders them today: it is built from
  material-web's `v0_192` token set, older than the component updates Compose's baseline
  `MaterialTheme` already draws. Its slider has a 4 track and a 20 round handle
  (`material.toml:231-232`); Compose's `SliderTokens` give a 16 track and a 4 × 44 bar handle
  (`tokens/SliderTokens.kt`). Its progress bar has square corners and a
  `surface-container-highest` track; the `latest` tokens give `corner-full`, a
  `secondary-container` track, a 4 gap and a stop indicator
  (`latest/sass/_md-comp-progress-indicator.scss`). So:
  - `material-3` = what Compose Material 3's `MaterialTheme` renders, at a pinned stable
    androidx Material 3 release (M2d Step 1 picks it), from its generated `*Tokens.kt`
    (generated from Google's token database), material-web's `latest` set as a
    cross-check; it replaces `material.toml` (the `v0_192` look is an outdated web
    rendering, not kept as a third preset);
  - `material-3-expressive` = `MaterialExpressiveTheme` plus the Expressive component
    tokens at the same release;
  - a detail with no token takes the Compose implementation constant (**(impl)**, as the
    column already does); with neither, it is measured on screenshots of a Compose reference
    app (M2d Step 3); a detail the model cannot carry (spring motion, shape morphing, wavy
    indicators, the emphasized styles where the model has no field) is listed in
    `platform-facts.md` as not representable, never approximated silently.
  
  The icon set `material` (Material Symbols, the `material-icons` feature,
  `icon_theme = "material"`) is a different thing and keeps its name. No alias for the old
  preset key (pre-1.0); a CHANGELOG line.

---

## The finding ledger

`docs/todo_code-review-findings.md`, created in M0, committed on the branch at every stop
point. One section per milestone; one entry per finding:

```markdown
### M3-07 · bug · S2 · native-theme/src/resolve/inheritance.rs:412
**Claim:** one sentence: what is wrong and for which input.
**Evidence:** the failing test, command and output, or upstream citation that shows it.
**Verdict:** CONFIRMED | PLAUSIBLE | REJECTED — the verifier's reason, one sentence.
**Triage:** fix | maintainer | defer (docs/todo.md) | drop — one sentence why.
**Status:** open | fixed in <sha> | deferred | dropped
```

Lens: `bug`, `simplify`, `perf`, `api`. Severity:

- **S1** — a panic, a hang, wrong output or lost data on input a user can produce.
- **S2** — wrong output on an edge case or one platform.
- **S3** — measured slowness on a path the user hits (theme load, theme switch, per frame).
- **S4** — simplification, naming, docs.

API findings carry `api-break` or `api-add` instead of a severity.

Triage rules (settled by logic, applied by the session, listed for the maintainer):

- CONFIRMED `bug` → fix. PLAUSIBLE `bug` → fix only once the fix's first step (a failing
  test) reproduces it; otherwise defer with the reason.
- `perf` → fix only with a before/after timing from `tests/timings.rs` on the release
  profile; a gain below the run-to-run spread is dropped.
- `simplify` → fix when it deletes code and changes no test except tests of deleted code.
- `api-add` → fix. `api-break` → **maintainer** (batched into one question per milestone).
- Anything touching a preset value or platform-facts row → `preset-validator`, and dropped
  without an upstream source.
- `defer` appends an entry to `docs/todo.md` (append-only).

## Shared prompts

The Workflow for each milestone's review phase is written from these templates; the region
table of the milestone fills `{region}`, `{files}` and `{focus}`.

### Region reviewer (Opus, read-only)

```text
You are reviewing one region of the native-theme Rust workspace at /home/tibi/Rust/native-theme
(branch v0.6.2-rc1). Region: {region}. Files: {files}.

Read every line of production code in those files (skip #[cfg(test)] modules except to learn
what is already tested). Look for, in this order:
1. bugs — panics (indexing, unwrap, arithmetic overflow, float-to-int casts, slicing),
   wrong results, swallowed errors, platform branches (cfg) that differ from their siblings,
   resource leaks (threads, subprocesses, file handles, caches without bound), races in
   watchers;
2. speed — work repeated on a hot path (theme load, theme switch, per-frame connector code):
   repeated I/O or subprocesses, allocations or clones of large structs in loops, caches
   that are rebuilt, O(n^2) lookups;
3. simplification — dead code, duplicated logic (inside this region or with code you can grep
   elsewhere), abstractions with one user, functions doing two jobs;
4. API — only for items in this region: naming, a type that admits invalid states, missing
   #[must_use] / #[non_exhaustive], an Option that should be a default or vice versa,
   inconsistency with the same concept elsewhere in the workspace.
Mandatory questions for this region: {focus}

Before reporting, check docs/todo.md (an item there is not new) and, for a design-level
claim, the relevant docs/archive/*-rationale.md (a recorded decision is not a bug unless the
code contradicts it). Never propose a value without an upstream source.

Report each finding with: lens, severity (S1–S4, or api-break / api-add), file:line, a
one-sentence claim naming the input that triggers it, the evidence (the code path traced
line by line, or a test you would write, written out), and the fix in one or two sentences.
Report nothing you have not traced in the code. An empty list is a valid result.
```

### API reviewer (Fable 5.1 at xhigh, read-only, one per crate; D2)

```text
You are reviewing the public API of crate {crate} in /home/tibi/Rust/native-theme as a
user would meet it on docs.rs. Start from the inventory at {inventory} (every pub item with
its signature), then read the crate-level docs and lib.rs. Judge: can a first-time user
find the entry point and do the common task (load the system theme, load a preset, apply it
to the toolkit, react to a change) in a few lines; are names consistent across the crate and
with the sibling crates ({siblings}); are there pub items no user needs (they should be
pub(crate)); do types let invalid states through; are errors actionable; are feature flags
coherent; do doc examples show the real usage. Pre-1.0: breaking changes are allowed but
each one must pay for itself. Report findings in the same format as the region reviewers,
lens api, tagged api-break or api-add, with the before/after signature written out.
```

### Verifier (Opus, read-only, one per reviewer)

```text
You verify findings another reviewer reported for native-theme at /home/tibi/Rust/native-theme.
You do not see their reasoning, only their claims. For each finding: open the cited code,
trace the claimed path yourself, and try to refute it — look for a guard elsewhere, a caller
that cannot produce the input, a test that already covers it, a docs/todo.md entry or a
docs/archive rationale that records it as intended. Verdict per finding: CONFIRMED (you
traced it and it holds), PLAUSIBLE (it may hold but needs a test to show it), REJECTED (you
found why it does not hold — say where). For a perf claim, say what measurement would
confirm it. Be terse: one sentence of reason per verdict.
```

### Workflow shape

`pipeline(regions, reviewer → verifier)` plus the API reviewer and its verifier in
parallel; reviewer output schema: `{findings: [{lens, severity, file, line, claim,
evidence, fix}]}`; verifier schema: `{verdicts: [{index, verdict, reason}]}`. The session
merges the results into the ledger, numbering findings `M<n>-01…`.

---

## M0 — Baseline

No review agents; the session does it inline. Cost: small.

**Files:**
- Create: `docs/todo_code-review-findings.md`
- Create: `native-theme/tests/timings.rs`

- [ ] **Step 1: Branch.** `git switch -c v0.6.2-rc1` from `main` at `92893ce1` (D0).

- [ ] **Step 2: Gate baseline.** Run `./scripts/check_release.sh`. Expected: all pass except
  "visual assets are stale" (the GIF change in `92893ce1`, cleared by the next
  `generate_assets_release.sh`). Record the pass/warn counts in the ledger's *Baseline*.

- [ ] **Step 3: Cross-target clippy.** Run
  `cargo clippy -p native-theme --all-features --target x86_64-apple-darwin -- -D warnings`
  and the same for `x86_64-pc-windows-msvc`. On 2026-09-27 the macOS target had 7 errors
  (`needless_return` in `detect.rs`, `model/icons.rs`, `pipeline.rs`, `presets.rs`;
  `needless_borrow` in `lib.rs`). Fix what remains, one commit,
  `fix(native-theme): clippy is clean on the macOS and Windows targets`; add both commands to
  the cross-target section of `scripts/check_release.sh` in the same commit, so the gate
  holds them.

- [ ] **Step 4: Timing test.** Create `native-theme/tests/timings.rs`:

```rust
//! Wall-clock timings of the paths the code review measures: preset parsing, resolution,
//! the system reader, the accessibility reader and icon loading. Ignored by default; run on
//! the release profile, before and after a perf fix:
//!
//! `cargo test -p native-theme --release --all-features --test timings -- --ignored --nocapture --test-threads=1`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

use std::time::{Duration, Instant};

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
```

  Run it three times; record the medians and the spread between runs in the ledger's
  *Baseline* (the spread is the noise floor for perf triage). If a path does not compile
  (a re-export differs from the names above), correct the path, not the measured
  operation. Commit: `test(native-theme): an ignored timing probe for the paths the code
  review measures`.

- [ ] **Step 5: Lint and dependency inventory.** Reviewer input, not fixed blindly:
  - `cargo clippy --workspace --all-features --message-format=short -- -W clippy::pedantic -W clippy::nursery 2>&1 | grep -oE '\[clippy::[a-z_]+\]|clippy::[a-z_]+' | sort | uniq -c | sort -rn`
    → lint counts per crate in the ledger;
  - `cargo machete` → unused dependencies (each one is an M3/M4/M5–M7 finding for its crate);
  - `cargo tree -d -e normal --workspace` → duplicated dependency versions, per crate.

- [ ] **Step 6: Semver tool.** `cargo install cargo-semver-checks --locked`; run
  `cargo semver-checks -p native-theme --baseline-version 0.6.1` and record "no breaking
  change" as the baseline (it should be: `main` has no API change since 0.6.1).

- [ ] **Step 7: Ledger.** Create `docs/todo_code-review-findings.md` with the sections
  *Baseline*, *Calibration*, *M1* … *M9*, filled from Steps 2–6. Commit:
  `docs: the code review's finding ledger, with the baseline gates, timings and inventories`.

**Close gate:** `./scripts/check_release.sh` as in Step 2; run
`~/.claude/scripts/sweep-target.sh /home/tibi/Rust/native-theme`.

---

## M1 — Design review

Reviews the decisions, not the lines: for each topic, is there a design that would serve
the project's goal (an application looks native on each platform, with minimal code in the
application) better, at a cost worth paying before 1.0? Cost: 5 design reviewers + 5
critics, about +2000k.

### Topics

| Agent | Topic | Current design, where to read it | Questions |
|-------|-------|----------------------------------|-----------|
| D-A | **T1 OS versions and preset naming** | `pipeline.rs:408-437`; `docs/new-os-version-guide.md` (Windows :44-82, macOS :84-120); preset names `windows-11`, `macos-sonoma`, `ios`, `material` | Should the readers detect the OS version and pick the preset made for it (nearest older generation as fallback)? Keep one preset per visual generation, named after its OS version (`windows-11` stays for Windows 11 when a `windows-12` comes; `macos-sonoma` for 14–15 next to a `macos-tahoe`; is `windows-10` worth adding)? How does a user pin an older generation's look on a newer OS? How does the reader map macOS 26 and 27 under D4? Does the naming rule (D6) apply to every platform preset: `ios` (which version, M2c), `kde-breeze`, `adwaita`? |
| D-A | **T2 Mobile scope** — settled by D5 | `docs/todo.md:2575-2577` (readers deferred); `ios.toml`, `material.toml` | Only what D5 leaves open: what the README says about the `ios` and `material` presets, and whether `ios` should name its iOS version. |
| D-B | **T3 Data model and resolution** | `model/`, `resolve/`, `docs/property-registry.toml`, `docs/inheritance-rules.toml`, `native-theme-derive` | Theme → ThemeMode → Resolved layering, per-widget structs, `Option` on every field; `Theme::resolve` reading the system (`model/mod.rs:461`) inside what looks like a pure function; is the derive macro plus the two TOML registries the simplest way to keep them consistent? |
| D-B | **T4 Two preset tiers** | `presets/README.md` (regular vs `-live`); the drift on `docs/todo.md` *Live presets* | Could each `-live` base be derived mechanically from its regular preset minus the reader's fields, instead of a second hand-kept file? |
| D-B | **T5 The source of truth** | `docs/platform-facts.md` (markdown tables, 1.8k lines), `presets/documented_sizes.rs`, `tests/platform_facts_xref.rs` | Would a machine-readable facts file (value, source, OS version per row) let presets be checked, or generated, mechanically, and make a new OS version a data change? |
| D-C | **T6 Readers, runtime, accessibility, icons** | `reader.rs`, `pipeline.rs`, `lib.rs` (`from_system` via `pollster`, `from_system_async`), `watch/`, the five icon loaders in `icons.rs` | The sync/async split; `ReaderOutput::Single`/`Dual`; where text scale and reduced motion are applied (core or each connector); one loader type per icon set vs one loader with a set parameter; system icons as fixed-colour rasters. |
| D-D | **T7 Connector architecture** | the three connectors; their test-only `contract` modules (iced 8.2k lines against 5.9k of production code), compat stamps, Widget Info, the separate `native-theme-egui-widgets` crate vs gpui's `widgets` module | Is each piece of verification machinery paying for itself? Which logic is written three times and belongs in core? Should the three public APIs share one shape by construction (a trait, a macro, or a checked convention)? |
| D-E | **T8 Showcases** | three applications, 72k lines (gpui 41k, iced 17k, egui 14k), `docs/showcase-elements.toml`, `scripts/check_showcase_parity.py` | Are three full applications the right way to show and test native look, or would a smaller shared gallery plus the parity dumps do the same job for less maintenance? |
| D-E | **T9 Release and asset process** | `scripts/check_release.sh` (83 checks), `generate_assets_release.sh`, `PROVENANCE.toml`, `docs/COMPATIBILITY.toml`, `.github/workflows/` | Is each gate catching a class of mistake that happened, and is any missing (OS-version drift: nothing noticed the Tahoe runner)? |
| D-E | **T10 Planning documents** | `docs/todo.md` (2.6k lines), `docs/archive/`, `ROADMAP.md` | Can the backlog be found and prioritised; are closed items cleared? |

### Steps

- [ ] **Step 1: Context (Haiku `locate`).** Per topic, the files and archived rationale
  sections that record the current decision, with `file:line`; saved to the scratchpad.
- [ ] **Step 2: Review + critique (one Workflow).** Each design reviewer (Fable 5.1 at xhigh, D2) writes, per
  topic, one or more *proposals*: current design (cited), the problem (evidence: a bug, a
  measured cost, a user-visible limit, a maintenance cost in lines or steps), at least two
  alternatives including "keep it", a recommendation, its cost (files and lines touched,
  new dependencies), whether it breaks the API, and what it unblocks. Each critic (Opus 5.5 at high,
  fresh context) argues for the current design, checks the evidence, and gives a verdict:
  *adopt*, *adopt with changes* (which), or *keep*.
- [ ] **Step 3: Ledger.** Proposals and verdicts into the ledger's *M1* section, numbered
  `M1-T1-01`….
- [ ] **Step 4: Decisions.** Design choices are the maintainer's: every proposal the critic
  did not reject goes to the maintainer in AskUserQuestion rounds (up to four per round),
  each with the recommendation first. Record the answers.
- [ ] **Step 5: Commit the ledger — stop point A.** `docs: design review findings and
  decisions`. Record the measured cost in *Calibration*.
- [ ] **Step 6: Plan the adopted changes.** A change that fits in a few tasks gets them in
  the ledger, with gates, as in the milestone template's Step 6. A larger one (for example
  version-aware preset selection, or a machine-readable facts file) gets
  `docs/todo_<topic>-{rationale,spec,plan}.md`, written as earlier releases' were, and an
  entry on `docs/todo.md`; the maintainer schedules it, and the plan's later milestones note
  which of their regions it rewrites (those regions are reviewed after it lands).
- [ ] **Step 7: Fix and close — stop point B.** The small changes as in the template's
  Steps 7–8, and the D6 rename:

  **Task: rename the `material` preset to `material-3`.** Files: `native-theme/src/presets/material.toml` →
  `material-3.toml` (`git mv`), its entry in `PRESET_ENTRIES` / `PRESET_NAMES`
  (`native-theme/src/presets.rs`), every preset-name use found by
  `grep -rIn --exclude-dir=target --exclude-dir=.git --exclude-dir=archive -E '"material"|material\.toml|material:(light|dark)' .`
  (189 lines on 2026-10-10 outside this plan, most of them the icon set, which stays): tests, connectors,
  showcases (theme menus, `--theme` arguments), `scripts/generate_screenshots_*.sh`,
  `scripts/generate_gifs_*`, `.github/workflows/screenshots.yml`, READMEs, the presets
  README, `docs/platform-facts.md` prose that names the preset (its *Material* column header
  names the design system and stays). This rename only moves the file; M2d rewrites its
values and adds `material-3-expressive`. Gate, a test in `native-theme/tests/preset_loading.rs`:

  ```rust
  #[test]
  fn material_preset_is_named_by_its_generation() {
      assert!(Theme::preset("material-3").is_ok());
      assert!(Theme::preset("material").is_err());
      assert!(Theme::list_presets().iter().any(|p| p.key == "material-3"));
  }
  ```

  fails before the rename and passes after; then every remaining `"material"` hit of the
  grep above is an icon-set use (listed in the commit message's body), all tests pass, and
  `./scripts/check_release.sh` passes except the stale-assets warning (screenshot file
  names change with the next `generate_assets_release.sh`). CHANGELOG `[Unreleased]`:
  "The `material` preset is now `material-3`." Commit:
  `refactor(presets): the material preset is material-3, named by its generation`.

## M2 — Platform fidelity

Checks every platform's theme details against the platform, per OS version. Each part runs
on its own and can stop on its own; the order is by how much is unverified: M2a, M2b, M2c,
M2d, M2e. The decisions of M1's T1 and T2 (version-aware presets, mobile scope) decide what
a part does with a version gap it finds: a new preset, a documented limit, or nothing.

| Part | Platform | Versions to map | Primary sources |
|------|----------|-----------------|-----------------|
| M2a | macOS | 14 Sonoma, 15 Sequoia (only: does it differ from 14?), 26 Tahoe (the `macos-tahoe` preset of D4), 27 Golden Gate (does it differ from 26?) | Apple HIG (the JSON API at `developer.apple.com/tutorials/data/design/human-interface-guidelines/*.json`, already used in `platform-facts.md`), AppKit documentation, WebKit `RenderThemeMac.mm` and its Cocoa siblings at a pinned commit, the probe (Step 3) |
| M2b | Windows | 10 22H2, 11 (23H2–25H2), and any release after 11 | `microsoft-ui-xaml` theme resources at a pinned commit, Win32 documentation (`GetSystemMetricsForDpi`, `GetSysColor`, `NONCLIENTMETRICSW`), the Fluent 2 specification, the probe |
| M2c | iOS (small, under D5) | which iOS version `ios.toml` matches; its colours and fonts against that version | Apple HIG (iOS), UIKit documentation (`UIColor` system colours, Dynamic Type sizes) |
| M2d | Material 3 and Material 3 Expressive (D6) | the latest stable androidx Material 3 release: `MaterialTheme` and `MaterialExpressiveTheme` | Compose Material 3's generated `*Tokens.kt` and its component sources at that release's tag; material-web's `latest` token set as a cross-check; m3.material.io's spec pages; the reference app's screenshots (Step 3) where no token or constant exists |
| M2e | KDE and GNOME (currency only) | Breeze pinned at v6.7.5, libadwaita at 1.10.0 | the latest released Breeze and libadwaita: diff each cited file between the pinned and the latest tag; only rows whose cited lines changed are re-verified |

### Steps (per part)

- [ ] **Step 1: Version map.** Which versions are in support and in use (the vendor's own
  lifecycle pages), which visual generation each belongs to, and which version each
  `platform-facts.md` row of this column claims (a row with no version is listed as such).
  Into the ledger's *M2<part>* section.
- [ ] **Step 2: Row-by-row check (Workflow, Opus, web access).** The column's rows in
  chapter 1 and its column in chapter 2, split by chapter section across 2–3 researchers
  (fonts and colours; geometry, metrics and spacing; the widget sections 2.2–2.29). Each
  row: the value at the newest version of each generation, with a primary-source citation
  at a pinned revision or a dated documentation page; *same*, *differs* (old and new value,
  both cited), or *unverifiable* (no primary source found: the row keeps or gets a ❓, and
  no value is proposed). For M2c, the rows the `ios` preset states, since iOS has no column.
- [ ] **Step 3, M2d: Compose reference app (the maintainer decides).** Where Step 2 finds
  no token and no implementation constant: a dev-only Android app (Kotlin, Jetpack Compose,
  the pinned Material 3 release) that draws every widget the model has, resting and in each
  state, under `MaterialTheme` and under `MaterialExpressiveTheme`, light and dark; run on an
  Android emulator in CI (an `ubuntu-latest` runner with KVM) at 160 dpi, so 1 dp = 1 px;
  screenshots measured as earlier rounds measured desktop captures, each measurement
  recorded with the app's commit, the Material 3 version and the emulator image. A new
  toolchain (JDK, Android SDK) and a CI dispatch: both the maintainer's.
- [ ] **Step 3: Probe (M2a, M2b; the maintainer decides).** A dev-only example that prints
  what the OS reports (macOS: control intrinsic sizes, system colours and fonts in both
  appearances; Windows: system metrics at 96 DPI, `UISettings` colours, the
  `NONCLIENTMETRICSW` fonts), run in CI on each runner image GitHub offers
  (`macos-14`, `macos-15`, `macos-26`, and a macOS 27 image if GitHub offers one yet;
  `windows-2022`, `windows-2025`, which are Windows
  Server builds and are recorded as such, not as Windows 10 or 11). Two conditions: if the
  probe needs `unsafe` FFI it stops and asks (strict rule), and the CI dispatch is the
  maintainer's.
- [ ] **Step 4: Preset check.** The `preset-validator` agent on the part's presets
  (`macos-sonoma*`; `windows-11*`; `ios`; `material`; `kde-breeze*`, `adwaita*`) against the
  checked facts.
- [ ] **Step 5: Verify.** A fresh verifier re-opens every cited source behind a *differs*
  and confirms the value; a citation it cannot reach is downgraded to *unverifiable*.
- [ ] **Step 6: Triage and commit — stop point A.** A *differs* row inside the version the
  preset targets → fix. A version gap (a generation with no preset) → a maintainer decision
  under M1's T1: a new preset (its own spec and plan: facts column, preset, `-live` base,
  reader selection, screenshots), a documented limit, or nothing. Commit the ledger.
- [ ] **Step 7: Fix and close — stop point B.** Rows corrected in place, each change citing
  its source, nothing else in the file rewritten; presets updated from the corrected rows;
  gates: `cargo test -p native-theme --test platform_facts_xref`, the
  `documented_sizes.rs` checks, a clean `preset-validator` report, then the template's
  Step 8. Screenshots of a changed preset wait for the next `generate_assets_release.sh`.
  For M2a, Step 7 comes after Steps 8–12; for M2d, after Steps 13–15.

### M2a only: build `macos-tahoe` (D4)

Runs after M2a's Steps 1–6, so the Tahoe value of every macOS row is known and cited, and
after M1, whose T1 decision says how the reader selects by version and T5 decision says how
`platform-facts.md` holds a second macOS version (if M1 changed neither: the Tahoe value goes
beside the Sonoma value in the same cell, tagged with its source and "macOS 26").

- [ ] **Step 8: Tahoe facts.** Every macOS row of `platform-facts.md` gets its macOS 26
  value from Step 2, or a ❓ where no primary source gives one. A value unchanged since
  Sonoma is recorded as unchanged, with the source that shows it.
- [ ] **Step 9: Presets.** `native-theme/src/presets/macos-tahoe.toml` and
  `macos-tahoe-live.toml`, written from Step 8's rows only: a value Step 8 left ❓ stays
  unstated, as the presets README's gated-size rule already requires; the `-live` base omits
  the fields `MacosReader` provides, as `macos-sonoma-live.toml` does. Registered in
  `PRESET_ENTRIES` / `PRESET_NAMES`. Gate: `preset-validator` reports no value without a
  citation; `cargo test -p native-theme` passes (the preset tests iterate every preset).
- [ ] **Step 10: Version-aware selection.** `platform_preset_name()` on macOS returns
  `macos-tahoe` on 26 and later and `macos-sonoma` before 26, from the running version
  (`NSProcessInfo::operatingSystemVersion`; if reading it needs `unsafe`, stop and ask). The
  mapping is a pure function of the version, so it is tested on Linux:

  ```rust
  #[test]
  fn macos_preset_follows_the_os_generation() {
      assert_eq!(macos_preset_for_version(14), "macos-sonoma");
      assert_eq!(macos_preset_for_version(15), "macos-sonoma");
      assert_eq!(macos_preset_for_version(26), "macos-tahoe");
      // macOS 27 until a macos-golden-gate preset exists (D4)
      assert_eq!(macos_preset_for_version(27), "macos-tahoe");
  }
  ```

  (`macos_preset_for_version` is the name unless M1 T1 designed a different shape; the
  test's four cases stand either way.) CI's macOS legs (`macos-latest`) run it on a real Mac.
- [ ] **Step 11: Documentation.** The presets README (listing, live presets), the
  `native-theme` README's preset list, `docs/new-os-version-guide.md` (macOS: a new
  generation is a new preset plus a version arm), CHANGELOG `[Unreleased]`, and the
  `docs/todo.md` entry for macOS 27 (D4).
- [ ] **Step 12: Screenshots.** `macos-tahoe:light:system` and `macos-tahoe:dark:system` in
  `.github/workflows/screenshots.yml` next to the `macos-sonoma` entries (:141-142), and the
  showcases' theme menus list the new preset. The CI run is the maintainer's (the next
  `generate_assets_release.sh`); until then the stale-assets warning stands.

### M2d only: the two Material presets (D6)

Runs after M2d's Steps 1–6 and after M1's rename task (the file is `material-3.toml`).

- [ ] **Step 13: Facts.** The *Material* column of `platform-facts.md` is re-based on the
  pinned Compose release: every cell cites its `*Tokens.kt` token, its Compose constant, or
  its Step 3 measurement; cells whose Expressive value differs carry both, the Expressive one
  tagged *Expressive*; the not-representable details are listed under the column's
  introduction (`platform-facts.md:865`).
- [ ] **Step 14: Presets.** `material-3.toml` rewritten from the baseline values and
  `material-3-expressive.toml` written from the Expressive ones, both sourced row by row as
  `material.toml`'s comments are now; registered in `PRESET_ENTRIES` / `PRESET_NAMES`.
- [ ] **Step 15: A mechanical token gate.** `scripts/check_material_tokens.py`: reads the
  pinned `*Tokens.kt` files (downloaded once into `target/material-tokens/<tag>/`) and
  checks every preset value whose comment cites a token against that token's value, for
  both presets; added to `scripts/check_release.sh`. It fails on the present `material.toml`
  values for the slider (`track_height_px = 4.0` against `SliderTokens.ActiveTrackHeight`
  16) before Step 14 and passes after. Screenshots of both presets are added to the
  showcases' theme menus and `screenshots.yml`, with the run left to the maintainer.

---

## Milestone template (M3–M9)

Every milestone runs these steps; its own section lists only the regions, focus questions and
anything extra.

- [ ] **Step 1: Inventory (Haiku `locate`).** For each crate in scope, list every `pub` item
  reachable from the crate root with its signature (rustdoc JSON on the docs.rs emulation
  toolchain from `scripts/check_docsrs.py`, or grep of `pub` items); save to the session's
  scratchpad as `inventory-<crate>.md`; and the crate's pedantic and nursery lint hits
  (M0 Step 5's command with `-p <crate>`), each reviewer given the lines for its files as
  hints, never as findings by themselves. For a connector, add a timing step to its
  `tests/timings.rs` (create the file) that times `from_preset(key, is_dark …)` for every
  bundled preset in both modes, and record it in the ledger's *Baseline*.
- [ ] **Step 2: Review + verify (one Workflow).** Regions × region reviewer → verifier, plus
  API reviewer → verifier, from the templates above.
- [ ] **Step 3: Ledger.** Merge the results into the ledger; drop REJECTED findings with
  their reason kept on one line each.
- [ ] **Step 4: Triage.** Apply the triage rules. Batch every `maintainer` item into one
  AskUserQuestion round (API breaks with before/after signatures). Record the answers.
- [ ] **Step 5: Commit the ledger — stop point A.** `docs: code review M<n> findings, verified
  and triaged`. Record the review phase's measured cost in *Calibration*.
- [ ] **Step 6: Fix plan.** Append to this milestone's ledger section a task list, one task
  per fix (or per group of fixes in one function), each with its mechanical gate:
  - bug → a failing test that reproduces the finding, then the fix, then it passes;
  - perf → the timing test before and after on the release profile, the gain above the
    noise floor recorded;
  - simplify → the crate's tests pass unchanged except deleted tests of deleted code, and
    `git diff --stat` shows a net deletion;
  - api → the semver report changes as the triage approved, every caller in the workspace
    (connectors, showcases, doc examples) compiles, a CHANGELOG `[Unreleased]` line.
  Order: S1 first, then API changes (they move code the others touch), then S2, S3, S4.
- [ ] **Step 7: Fix (subagent-driven-development).** One `implement` agent per task with
  the task's gate as its done-criterion; a fresh reviewer per task. After two failed
  attempts at the same gate, the session takes the task over or re-specifies it.
- [ ] **Step 8: Close — stop point B.** `./scripts/check_release.sh` (only the known stale
  assets warning), both cross-target clippy commands from M0 Step 3, `cargo semver-checks`
  for every crate touched (result recorded; then `rm -rf target/semver-checks`, the tool's
  own build directory), the timing test re-run (no path slower than its
  baseline beyond the noise floor), ledger statuses updated and committed, the project
  memory updated, `~/.claude/scripts/sweep-target.sh /home/tibi/Rust/native-theme`.

---

## M3 — Core, part 1: model, resolution, pipeline, presets loading, derive macro

| Region | Files | Focus questions |
|--------|-------|-----------------|
| R1 model types | `native-theme/src/model/{mod,defaults,border,font,resolved,dialog_order,arrow_side,tab_indicator_side}.rs`, `model/widgets/mod.rs` | Review Focus 5: what does each type accept from TOML, and what reaches `Resolved` unvalidated? |
| R2 resolution | `native-theme/src/resolve/{mod,context,inheritance,validate,validate_helpers}.rs` | Review Focus 4 and 5: every arithmetic on sizes (scale, sums, differences) with zero, negative, NaN and very large inputs; does `Theme::resolve` re-read the system on every call (`ResolutionContext::from_system`, `model/mod.rs:461`), and what does that cost per theme switch (measured 2026-10-10 with the M0 probe, tests in parallel: 1.3–3.5 ms per `resolve`, in two clusters, against ~1 µs to parse a preset)? |
| R3 pipeline and entry points | `native-theme/src/{lib,pipeline,reader,prelude,error,color}.rs` | Review Focus 2: what work does a second `SystemTheme::from_system` repeat; is every reader error kept or reported, not dropped? |
| R4 presets loading and derive | `native-theme/src/presets.rs`, `presets/documented_sizes.rs`, `native-theme-derive/src/*.rs` | Is preset parsing done once per process or per call; is generated code (merge, inherit, validate, ranges) the same as a hand-written version would be, and does the macro report a span on misuse instead of panicking? |
| API | crate `native-theme` (whole public surface; siblings: the three connectors) | The common tasks in a few lines; `theme` vs `prelude` vs crate-root re-exports overlapping. |

## M4 — Core, part 2: platform readers, detection, watching, fonts, icons, build crate

| Region | Files | Focus questions |
|--------|-------|-----------------|
| R1 Linux readers | `native-theme/src/kde/{mod,colors,fonts,metrics}.rs`, `gnome/mod.rs` | Review Focus 1: malformed and missing `kdeglobals`, no portal, no gsettings. |
| R2 detection and watching | `native-theme/src/detect.rs`, `watch/{mod,kde,gnome,macos,windows}.rs` | Review Focus 1 and 3: every subprocess has a timeout and is reaped; watcher threads stop when the handle drops; the macOS and Windows watchers read as carefully as the Linux ones. |
| R3 macOS and Windows readers | `native-theme/src/{macos,windows}.rs` | Review Focus 3: every `cfg` branch; FFI results checked; each of the `unsafe` blocks in `macos.rs`, `sficons.rs`, `windows.rs`, `winicons.rs` and the two platform watchers: its safety invariant stated and upheld (buffer sizes, `cbSize`, handle cleanup on every path). |
| R4 icons and fonts | `native-theme/src/{icons,freedesktop,winicons,sficons,rasterize,spinners,fonts}.rs`, `model/{icons,bundled,icon_sizes,animated}.rs`, `native-theme-build/src/*.rs` | Review Focus 2: every cache's bound and lifetime; icon misses cost; no icon-set mixing on any fallback path; `native-theme-build` errors name the offending TOML line. |
| API | crates `native-theme` (icons, fonts, watch, detect surfaces), `native-theme-build` | One loader pattern across five icon loaders; watch API's shutdown semantics. |

## M5 — iced connector

Reviewed before v0.6.2's iced geometry work, so that work builds on reviewed code.

| Region | Files | Focus questions |
|--------|-------|-----------------|
| R1 theme and palette | `connectors/native-theme-iced/src/{lib,palette,extended,icons,spinner}.rs` | Review Focus 4: every padding helper with zero, negative and unstated sides; every resolved value reaches iced (theme-data rule). |
| R2 styles | `connectors/native-theme-iced/src/styles.rs`, `styles/aw.rs` | Review Focus 2: what each style closure captures and clones per call (they run per widget per frame). |
| API | crate `native-theme-iced` (siblings: gpui, egui) | Names and shapes shared with the siblings (`to_theme`, `from_preset`, `from_system`). |

## M6 — gpui connector

| Region | Files | Focus questions |
|--------|-------|-----------------|
| R1 theme build | `connectors/native-theme-gpui/src/{lib,colors,config,derive,variants}.rs` | Review Focus 4; the base-palette repair; per-switch cost of `to_theme`. |
| R2 geometry and base layer | `connectors/native-theme-gpui/src/{geometry,base_layer}.rs` | Every geometry builder with unstated and extreme sizes; the base-layer observer's re-application on every theme change, not more. |
| R3 icons | `connectors/native-theme-gpui/src/icons.rs` | The `_ => None` tables against `all_icon_names!`; no mixing of sets; per-switch icon cost. |
| R4 widgets | `connectors/native-theme-gpui/src/widgets/{mod,checkbox,spinner,tab_bar,switch,slider,progress,separator}.rs` | Render-path allocations; every widget reads its size from the theme. |
| API | crate `native-theme-gpui` (siblings: iced, egui); `showcase.rs` read as a consumer of the API only | |

## M7 — egui connector and egui widgets

| Region | Files | Focus questions |
|--------|-------|-----------------|
| R1 atlas, install, plugin | `connectors/native-theme-egui/src/{lib,atlas,ext,plugin,watch,roles,accessors}.rs` | Review Focus 2: what runs per frame vs per install; `install` called every frame by a naive app. |
| R2 style builders | `connectors/native-theme-egui/src/style/{mod,base,states,roles}.rs` | Review Focus 4: every float reaching epaint (NaN, ∞, ≤ 0 sanitised as the spec requires). |
| R3 derived, frames, fonts, icons | `connectors/native-theme-egui/src/style/{derived,variants,frames}.rs`, `src/{fonts,icons,convert}.rs` | Font database load once per process; icon texture lifetime and `forget_icons`. |
| R4 egui widgets | `connectors/native-theme-egui-widgets/src/*.rs` (not `tests.rs`) | Each widget's size from the theme; animation state cleaned up. |
| API | crates `native-theme-egui`, `native-theme-egui-widgets` (siblings: iced, gpui) | |

## M8 — Cross-cutting

| Region | Scope | Focus questions |
|--------|-------|-----------------|
| R1 connector parity and shared code | the three connectors' public surfaces (the `connector-parity-checker` agent for gpui/iced, the same check extended to egui) | Concepts with different names or shapes across connectors; logic written three times that belongs in core (padding fallbacks, text scaling, icon recolouring). |
| R2 test quality | every crate's tests (Haiku inventory first: tests without an assertion, `#[ignore]`d tests no script runs, tests asserting a constant against itself) | Tests that cannot fail; gates that pass when their input is missing. |
| R3 scripts and CI | `scripts/*.sh`, `scripts/*.py`, `.github/workflows/*.yml` | A gate that silently skips (missing tool, empty glob, `|| true`); release steps run in the wrong order; the `docsrs` job in `publish.yml` (untested until the next tag). |
| R4 documentation | `README.md`, crate-level docs, `docs/new-os-version-guide.md`, `CHANGELOG.md` `[Unreleased]` | A documented behaviour the code no longer has; an example that would not compile. |

## M9 — Optional: the showcases

Lens: theme-data violations (a hard-coded colour, size or spacing where a resolved value
exists), dead code, duplication between the three showcases, and slow paths on theme switch.
Split so each part can stop on its own:

- **M9a** gpui showcase, `connectors/native-theme-gpui/examples/showcase-gpui/` (~32k lines
  without `tests.rs`): 3 reviewers by directory (`demo.rs`; `app.rs` + `main.rs`; `info/`).
- **M9b** iced showcase, `connectors/native-theme-iced/examples/showcase-iced.rs`
  (16.9k lines): 2 reviewers by line range at a function boundary near the middle.
- **M9c** egui showcase, `connectors/native-theme-egui/examples/showcase-egui/` (~10k lines
  without `tests.rs`): 2 reviewers (`pages/`; the rest).

## Close-out (after the last milestone the maintainer runs)

- [ ] CHANGELOG `[Unreleased]` complete; ROADMAP entry and renumbering per D1, approved.
- [ ] Move this plan and the ledger to `docs/archive/` (`todo_v<version>_code-review-plan.md`,
  `…-findings.md`) with an *As built* note at the top of each; commit.
- [ ] Project memory updated; `~/.claude/scripts/sweep-target.sh` run.
- [ ] The maintainer decides push, PR and release.

## Resuming

A new session: read this file, then the ledger. The current milestone is the first whose
*Status* line in the ledger is not `done`; inside it, the first unchecked step of the
template. If the last step started was Step 2 and its Workflow did not finish, re-run it
(Workflow resume works only inside the session that started it). If it was Step 7, re-run
the gate of the last task without a commit before starting the next.
