# Scripts

Visual asset generation and release automation for native-theme.

All scripts are run from the project root. Each asset is generated into the
crate it documents so relative paths in per-crate READMEs resolve everywhere
(offline, on GitHub, and in the published crate on crates.io / docs.rs):

- gpui screenshots + theme-switching GIF → `connectors/native-theme-gpui/docs/assets/`
- iced screenshots + theme-switching GIF → `connectors/native-theme-iced/docs/assets/`
- egui screenshots + theme-switching GIF → `connectors/native-theme-egui/docs/assets/`
- Workspace crate-relations diagram → `docs/assets/` (workspace-level)

## generate_assets_local.sh

Master orchestration script. Runs all four asset generators in sequence:
iced screenshots, gpui screenshots, egui screenshots, and theme-switching
GIFs.

```sh
./scripts/generate_assets_local.sh
```

## generate_gifs_assemble.py

Assembles pre-captured PNG frames (`frame-*.png`, in name order) into an
animated, looping theme-switching GIF, each frame at its captured size (the
size of the screenshots beside it; frames of differing sizes are refused), on
a white background, two seconds per frame. `generate_gifs_theme_switching.sh`
calls it with each connector's output path.

Requires: Python 3, Pillow

```sh
python3 scripts/generate_gifs_assemble.py /path/to/frames \
    connectors/native-theme-iced/docs/assets/theme-switching.gif
```

## generate_screenshots_iced.sh

Captures iced showcase screenshots on Linux (KDE Wayland) using spectacle.
Launches the showcase with each theme/variant/icon-set combination, waits for
it to render, then makes its window the active one and captures it. Every
capture path (these scripts, `generate_gifs_theme_switching.sh` and the
screenshot workflow) passes `--tab basic`: the Basic page, which the three
showcases draw alike, so their captures compare control by control.

On macOS/Windows, use the showcase's built-in `--screenshot` flag instead.

The showcase runs with `--capture`, which opens it at its default 1280 × 720
whatever size the desktop remembers for its window, with an app id of its own,
and each capture passes `capture_window.sh`'s check (below) or the script fails.

Builds and runs the showcase with `--features iced_aw`, so the captures show
the same widget set the coverage check gates.

With a directory argument, `DUMP_DIR`, each run also writes its layout dump
and its capture there for `check_showcase_parity.py` (`capture_window.sh`
below); the screenshots in `docs/assets` are the same either way.

Requires: spectacle (KDE)

```sh
./scripts/generate_screenshots_iced.sh [DUMP_DIR]
```

## generate_screenshots_gpui.sh

Same as `generate_screenshots_iced.sh` but for the gpui showcase.

Requires: spectacle (KDE)

```sh
./scripts/generate_screenshots_gpui.sh [DUMP_DIR]
```

## generate_screenshots_egui.sh

Same as `generate_screenshots_iced.sh` but for the egui showcase, built with
`--all-features`.

Requires: spectacle (KDE)

```sh
./scripts/generate_screenshots_egui.sh [DUMP_DIR]
```

## capture_window.sh

Sourced by the four spectacle scripts, whose one launch-and-capture loop it
holds: `capture_showcases KIND FEATURES OUTPUT DUMPS ENTRY...` builds the showcase
KIND in release mode with the cargo flags FEATURES, runs it once per ENTRY
(`theme:variant:icon-set:icon-theme`) on its Basic page with `--capture`, and
captures its window to OUTPUT, in which `{theme}`, `{variant}` and `{index}`
stand for the entry's; given a directory DUMPS (the screenshot scripts'
`DUMP_DIR` argument, created if missing, a relative one taken from the
directory the script is run from), each run also writes its layout dump there
(`--dump-layout`, as `KIND-THEME-VARIANT.json`) and, once the dump is written,
its capture and the content's offset in it beside it, so both are of one
settled frame: the run directory `check_showcase_parity.py` reads; `stop_showcase`, which each script's EXIT trap calls,
stops the showcase a failed or interrupted run leaves. The features differ by
showcase, as the screenshot workflow builds them: iced with
`--features iced_aw`, egui with `--all-features`, gpui with its defaults. The
three showcases take an entry's icon theme, where it has one, as their
`--icon-set`, its icon set otherwise. `SCREENSHOT_ENTRIES` lists the six captures the three
screenshot scripts take (kde-breeze, material-3 and catppuccin-mocha, dark and
light).

`capture_showcase KIND PID FILE` captures the window of a showcase run with
`--capture`, whose app id is `showcase-KIND-capture-PID`:

- A KWin script, loaded through KWin's scripting D-Bus interface
  (`org.kde.kwin.Scripting.loadScript` at `/Scripting`, `run`, then
  `unloadScript`), finds the one window with that app id and sets
  `workspace.activeWindow` to it; the helper waits until it is the active
  window, and fails when there is no such window, several, or it does not
  become active. `spectacle -a` captures the active window, and a window a
  script opens need not be the active one (KWin's focus stealing prevention
  can keep it behind the window in use). The script reports by a D-Bus call that
  `dbus-monitor` reads.
- With the showcase stopped (SIGSTOP), so its window holds one frame, it
  captures the window with its frame to FILE, and without its decoration and
  shadow (`spectacle -e -S`).
- It fails unless the window is still the active one, the content is exactly
  1280 × 720 times the scale factor of KWin's active output (from
  `kscreen-doctor`), FILE is larger in both directions, and the content appears
  pixel for pixel inside FILE (only the pixels of the window's rounded corners,
  which the two captures blend differently, are not compared). A capture of
  another window, or of the showcase in another state, fails; the message names
  both files and their sizes.

Requires: spectacle, kscreen-doctor, qdbus6 (KDE Plasma 6), dbus-monitor and
dbus-send (D-Bus), Python 3, Pillow

## generate_gifs_theme_switching.sh

Captures four preset/variant pairs (kde-breeze dark, material-3 light,
catppuccin-mocha dark, kde-breeze light) from the iced, gpui and egui showcases, then assembles
each set into a looping theme-switching GIF via `generate_gifs_assemble.py`.
Each frame passes `capture_window.sh`'s check. The frames go to temporary
directories, which the script removes when it ends, fails or is interrupted.

Produces:
- `connectors/native-theme-iced/docs/assets/theme-switching.gif`
- `connectors/native-theme-gpui/docs/assets/theme-switching.gif`
- `connectors/native-theme-egui/docs/assets/theme-switching.gif`

Requires: spectacle (KDE), Python 3, Pillow

```sh
./scripts/generate_gifs_theme_switching.sh
```

## generate_diagrams.sh

Renders the Graphviz `.dot` sources in `docs/assets/` to matching `.svg`
files with Graphviz's `dot` binary.

Run after editing any `.dot` file. The generated `.svg` is checked into
git so contributors don't need Graphviz installed to view diagrams.

Requires: Graphviz (`pacman -S graphviz`, `apt install graphviz` or
`brew install graphviz`). To embed the "Your app" label font: fontconfig
(`fc-match`), python-fonttools and the Fuzzy Bubbles font; without them it
still renders, with a fallback face.

```sh
./scripts/generate_diagrams.sh
```

## generate_assets_release.sh

Full pre-release asset pipeline. Triggers the CI screenshots workflow for
macOS/Windows, generates all local Linux assets while CI runs, then downloads
the CI artifacts into the correct per-connector `docs/assets/` directory
based on artifact name, and finally writes `docs/assets/PROVENANCE.toml`
through `update_provenance.sh`, with the OS versions the CI jobs recorded
beside their captures (`<os>-os-version.txt` in each artifact, copied to a
temporary directory, never into `docs/`). Refuses to start while HEAD is unpushed or a path
the assets depend on has uncommitted changes, because the captures must come
from the commit CI builds.

The Linux screenshots' runs also write their layout dumps and captures into a
temporary directory, and `check_showcase_parity.py` compares the three
showcases there at rest under the six captured presets (sizes, positions,
colours); a difference `docs/showcase-exceptions.toml` lists no reason for
fails the script, which keeps the directory for the comparator's `--merge`;
a screenshot step that fails removes it, naming it.

Its last step runs `update_compatibility.sh run` (below), which needs the network,
runs every connector's tests, clippy and documentation on the newest upstream
releases, runs `check_widget_coverage.py` (Python 3.11+), and rewrites
`docs/COMPATIBILITY.toml` and the Verified line in the connector READMEs. A
connector that fails on the newest set fails the script.

Requires: gh CLI (authenticated), spectacle, kscreen-doctor, qdbus6,
dbus-monitor, dbus-send, Python 3.11+, Pillow, network access

```sh
./scripts/generate_assets_release.sh
```

## update_provenance.sh

Provenance stamp for the visual assets. `write` records the workspace
version, the commit, and a SHA-256 over the git object ids of every path
that feeds the showcases (crate manifests and sources, presets, icon
bundles, the egui connector's `mapping.toml`, the egui widgets crate,
the showcases' shared crate `tools/showcase-support`, `Cargo.lock`, the
capture scripts, the screenshots workflow) into
`docs/assets/PROVENANCE.toml`; `write DIR` also records, as `macos_version`
and `windows_version`, every distinct OS version in DIR's sidecars
(`<toolkit>-macos.txt`, `<toolkit>-windows.txt`: `sw_vers -productVersion`
and `[System.Environment]::OSVersion.Version` on the screenshot runners),
empty without them. No gate checks the versions against a preset's range yet. `check` recomputes the hash at HEAD and exits
non-zero with a message when the stamp is missing or the sources differ;
`scripts/check_release.sh` and the crates.io workflow's release gate call it, so a
release whose assets were captured from other sources is refused. Docs-only
changes do not alter the hash. `verify-clean` fails when a stamped path has
uncommitted changes; `hash` prints the current value.

```sh
./scripts/update_provenance.sh check
./scripts/update_provenance.sh write DIR
```

## update_compatibility.sh

The upstream versions each connector has been verified against, recorded in
`docs/COMPATIBILITY.toml` and stated on each connector README's Verified line.

- `run [gpui|iced|egui]` (all when none is named) refuses to start while the
  connector's `Cargo.toml`, `src`, `examples` or `tests` — for egui also
  `mapping.toml` and the egui widgets crate's `Cargo.toml` and `src` — have
  uncommitted changes, then runs `cargo update` on the connector's upstream
  family (gpui-base, gpui-component, gpui-fast, gpui-fast-apple,
  gpui-fast-linux, gpui-fast-macos, gpui-fast-macros, gpui-fast-platform,
  gpui-fast-web, gpui-fast-wgpu, gpui-fast-windows, gpui-kit, gpui-kit-assets,
  gpui-pre; or iced, iced_aw, iced_core, iced_futures, iced_test, iced_widget;
  or the egui release train — ecolor, eframe, egui, egui-wgpu, egui-winit,
  egui_extras, egui_kittest, emath, epaint, epaint_default_fonts, kittest) and
  that connector's gates on the result with `--locked`: tests (for gpui with
  `--features native,watch` and `--features gpui-fast,native,watch
  --all-targets`; for iced with its defaults, `--no-default-features` and
  `--features iced_aw,watch,native`; for egui with its defaults,
  `--no-default-features` and `--all-features`), clippy with `-D warnings` (for
  gpui with `--features native,watch`, for iced and egui with
  `--all-features`), documentation with `RUSTDOCFLAGS="-D warnings"` (for iced
  and egui also `--all-features`), for egui the widgets crate's four gates
  (tests with no features and with all features, clippy and documentation with
  all features), and `check_widget_coverage.py`. Only when every gate passes
  does it write the versions the lockfile resolved, the commit and a sources
  hash into the stamp and rewrite the README's Verified line between its
  `<!-- compat:begin -->` / `<!-- compat:end -->` markers. `Cargo.lock` is
  restored on exit, pass or fail. Needs the network, and Python 3.11+ for the
  coverage script.
- `check` exits 0 when the stamp exists and each connector's sources hash at
  HEAD matches the recorded one, and 1 with a message otherwise. It reads git
  and the stamp only, no network. `scripts/check_release.sh` runs it: a warning
  while the CHANGELOG entry says "Unreleased", a failure once it is dated. The
  crates.io workflow's release gate runs it on the pushed tag, so a tag whose
  connector changed since it was verified is refused before anything is
  uploaded.
- `hash <gpui|iced|egui>` prints a connector's sources hash at HEAD: a SHA-256
  over the git object ids of its `Cargo.toml`, `src`, `examples` and `tests`
  — for egui also `mapping.toml` and the egui widgets crate's `Cargo.toml` and
  `src`.

There is no verb that writes the stamp without the run.

```sh
./scripts/update_compatibility.sh run iced
./scripts/update_compatibility.sh run egui
./scripts/update_compatibility.sh check
```

## check_release.sh

The workspace's quality gate: source hygiene, `cargo check`, the feature
combinations, the cross-target checks and clippy, fmt, clippy, the strict
panic lints, the tests, the connectors' other configurations, the showcases'
tests in CI's runner environment (`run_in_runner_env.sh`, below), the examples,
widget coverage, connector parity, the README versions, the showcase-parity comparator's static modes, the
data documents' checks (the four on the registry, the platform-facts
sections, `docs/todo.md` and the presets' definition comments, the first
three `PLAN_EXCLUDED` until M2.6; then the 0.8.0 plan's: the ledger's
triage, the pinned sources and their cache, the facts and their citations,
the generated widgets, presets, `sf_tracking` and `dynamic_type` tables and
`docs/platform-facts.md`, the function vectors, the trace lines, the
pinned line citations and no file tracked under `docs/evidence/`, whose
evidence lives in `native-theme-evidence`), docs and the archive index, then the docs.rs emulation,
packaging, `cargo semver-checks`, `cargo audit`, `cargo outdated`, and the
visual-asset and compatibility stamp checks, all release-only but `cargo audit`. Prints one line per check and a summary;
exits 1 on a hard failure. A test leg that passes no test fails: a filter or a
cfg that matches nothing still exits 0. `CONTRIBUTING.md` lists every check.

With `--commit` it runs the per-commit subset: the release-tag check, the gpui
connector's tests on GPUI Fast, native-theme's ignored stem-index test and the
release-only sections are skipped, each
printing `skipped (--commit mode)`, and the summary names them. `cargo audit`
runs in both modes, as CI audits every pull request and every push to `main`. It fetches the advisory database
each time; where it cannot (no network), the full run fails, and the
per-commit subset audits against the cached database (`cargo audit
--no-fetch`, which checks no yanks) and warns with that database's date, or,
where no cached database loads, warns that the audit did not run.
The subset runs before every commit; the full run on the last commit of every
fix phase and before the release commit. An unknown argument exits 2;
`--help` prints the usage.

`PLAN_EXCLUDED`, an associative array near the top of the script, names the
steps the 0.8.0 plan's transition breaks, each by its description, with the
milestone that brings it back (the plan's *What cannot build meanwhile*; the
ledger records the list). `--commit` skips each listed step and says so; the
full run still runs each once, in a subshell, and reports a pass (remove it
from the list) or a failure (its milestone) as a warning, never a failure. A
listed description that no step of the full run used is a warning too, and
while the list is not empty the full run's banner says
`not ready: N plan-excluded steps` instead of `ready for release`. The
per-crate loops name one step per crate (`check (<crate>)`, `test (<crate>)`,
`examples (<crate>)`, …), so a crate is excluded alone; the feature
combinations take the crate-level key `feature combinations (<crate>)`,
which runs `check_features.sh --skip <crate>` for the others and
`--only <crate>` as that crate's own step. `ci.yml` marks its matching
exclusions with a YAML comment that is exactly
`# PLAN_EXCLUDED: <description>`, and the first section fails unless the
marks and the list name the same set.

Two speed-ups apply to `--commit` and `--quick` alone; the full run runs
every step through `cargo test`, with no cache. Where `cargo-nextest` is
installed, the test steps run `cargo nextest run` (`cargo_tests` in the
script), and `cargo test --doc` where `cargo test` would have run doctests too.
The step cache (`step_cache.sh`, below) skips a step whose declared inputs are
unchanged since it last passed; the summary lists each as
`unchanged since <commit>`, and `--no-cache` runs every step.

The facts gate (`native-theme/tests/facts_gate.rs`'s `facts_gate`) is a step
of its own; the other test steps skip it (`-- --skip facts_gate --exact`), since no `cfg` reaches it and
one run gates every target and feature set. `cargo nextest` does not act on that libtest form (0.9.138
runs the test all the same), so `cargo_tests` hands it the filter `-E 'not test(=facts_gate)'`, and the
step `nextest leaves the facts gate to its own steps` fails if the filter stops skipping the gate. The full run gates every preset
in one step. `--commit` and `--quick` gate it in parts the cache skips one by
one: `facts gate (the facts, no preset)` (`NT_FACTS_GATE_PRESETS=none`),
cached on every facts directory, then `facts gate (<preset>)` for each
bundled preset, cached on its preset file (the other presets' files left
out), its platform's facts directory (the gate's own `DIRECTORIES` table),
`docs/facts/*.toml` (the evidence pin among them),
`scripts/widget_units.toml` and the crate and test code. Every part also runs
the facts' own checks, so the parts together run every check the whole gate
runs, and a change to any code re-runs every part.

```sh
./scripts/check_release.sh --quick
./scripts/check_release.sh --commit
./scripts/check_release.sh --commit --no-cache
./scripts/check_release.sh
```

## step_cache.sh

The step cache of `check_release.sh --commit` and `--quick`, sourced by it.
A step is skipped only when the SHA-256 key of its run equals the one recorded
when it last passed: the tool versions (`rustc -vV`, `cargo -vV`, clippy's,
rustdoc's, rustfmt's, Python's, nextest's), both scripts' own hashes, the
environment (but `_`, `SHLVL`, `OLDPWD`), the step's description and command,
and the content of its declared inputs (`CACHE_INPUTS` in `check_release.sh`:
files, directories and globs by path and content, a missing one by name;
`__pycache__` folders left out; an input `!<pattern>` leaves out each file the
others name whose path the pattern matches). A cargo step's inputs (`@rust:<crates>`) are
the workspace manifest, lock and tool configuration, every Cargo configuration
file Cargo would read, and the directory of each workspace package the crates
reach on this host with every feature on; where that reach holds a registry
build helper that probes the host (`pkg-config`, `cc`, …) or a workspace build
script or procedural macro not reviewed for what it reads, the step is not
cached. A failed step records nothing, nor does one whose inputs changed while
it ran; a step with no declaration always runs. The records are in
`target/check-release-cache/`, one file per step.

`--self-test` shows that a declared file edited, added, renamed or removed, a
glob's new match, the script edited, the command or the environment changed,
re-run the step, that a file a `!<pattern>` leaves out changes nothing while
its neighbours still do, and that a failure records nothing; `check_release.sh` runs it
in every mode.

```sh
bash scripts/step_cache.sh --self-test
```

## preset-check (tools/preset-check)

A dev-only binary, never published, for the facts lanes, which run no cargo:
it loads a preset as a bundled preset loads (through the synthetic route where
its `[pending]` table lists paths), under each scheme, and prints every
problem in the reader's and validation's own words, grouped by class
(`== Missing: 12`, then one line per problem; one found under a single scheme
names it). Exit 0 when the preset loads, 1 otherwise. A preset whose `id`
names a bundled preset loads as that preset, so its `preset:` conditions hold;
any other loads as a preset no `PresetId` names, and the report says so.
`-` reads the preset from stdin. `--draft <id>` loads what
`python3 scripts/generate_preset.py <id>` prints, run in the current directory:
that draft leaves the hand-written parts' blocks empty, and the reader stops at
the first of them (`preset.generation: missing`) before validation runs. For a
bundled preset, `generate_preset.py <id> --write` in the worktree carries its
hand-written parts into the file, which `preset-check <file>` then loads whole.

Built once in the main checkout, then run by path from any worktree, with no
cargo:

```sh
cargo build -p native-theme-preset-check     # in /home/tibi/Rust/native-theme
cd <worktree>
python3 scripts/generate_preset.py ios-26 --write
/home/tibi/Rust/native-theme/target/debug/preset-check native-theme/src/presets/ios-26.toml
/home/tibi/Rust/native-theme/target/debug/preset-check --draft ios-26
```

## check_panic_patterns.py

A regex tripwire for panic-prone code: scans the `.rs` files under the
directories it is given for the panicking calls and macros its docstring
lists, and for `Instant::now()` followed by `+` or `-`, outside test code
(a `#[cfg(test)]` item, a `tests.rs` file, a module its parent declares under
`#[cfg(test)]`), with comments and literals blanked first. The authoritative
check is the type-aware `check_strict_panic.sh`; this one also sees what that
run's lint list does not name and code behind a cfg the clippy runs do not
build. Exits 0 when nothing is found, 1 on a finding or on an input it cannot
read (a missing directory, one with no `.rs` file, a file that is not UTF-8).
`--self-test` runs its built-in scenarios. `scripts/check_release.sh` (both
modes) runs it over every workspace member's `src/` and runs the self-test.

Requires Python 3.

```sh
python3 scripts/check_panic_patterns.py native-theme/src
python3 scripts/check_panic_patterns.py --self-test
```

## check_gate_cfg.py

The facts gate runs on one configuration only (CI's Linux `facts-gate`
jobs, `check_release.sh`'s single gate step), which holds while no code on
its path depends on the target or the features. This check finds the path
from the sources, never from a list kept by hand: the gate's test files
(`facts_gate.rs` and its `#[path]` modules), the crate root, then every
module the `use` trees, paths and macro invocations of a module on the path
reach through the `mod` declarations and re-exports, and every module with
an `impl` of a type on the path; the derive crate and the build script are
always on it. Along the path it fails on every `cfg`, `cfg!` and `cfg_attr`
but a test-only one, a lint-level `cfg_attr` and a `cfg` on the declaration
of a module off the path, each finding saying why. One edge is cut by name
(`CUTS`: `icons::model`'s `icon_svg`, the bundled SVGs), and the cut fails
where that function is named elsewhere on the path or the edge is gone.
`--list` prints each module on the path and what reached it; `--self-test`
runs its built-in scenarios. Every mode of `check_release.sh` and CI's parity
job run both. Exits 0, 1 on a finding, 2 on an unreadable source.

```sh
python3 scripts/check_gate_cfg.py --list
python3 scripts/check_gate_cfg.py --self-test
```

## check_features.sh

Checks that every workspace crate builds, without warnings, in every feature
combination the gate names: for each member of `cargo metadata --no-deps`
with a library target, `cargo check -p <crate> --lib` with
`--no-default-features`, with `--no-default-features --features <F>` for each
feature in its `[features]` table except `default`, and with
`--all-features` — the coverage of `cargo hack check --each-feature`, without
the extra tool. A feature that moves a crate to other dependencies instead of
adding to them is checked alone but left out of the all-features run, which
then names every other feature: native-theme-gpui's `gpui-fast`, which
switches the connector to another GPUI core. A combination fails when it does not build or when cargo
prints a `warning:` line; cargo caps lints for dependencies, so those come
from the workspace's own crates, and no `RUSTFLAGS` is set, so the
dependencies are not rebuilt. Colour is turned off inside the script, and so
is cargo's future-incompatibility notice, a `warning:` line about a
dependency rather than a workspace crate. Prints one line per combination
with an excerpt under each failure; exits 1 when any fails, naming each
failure at the end, and 2 when `jq` is missing. `--skip CRATE` leaves a
crate out and `--only CRATE` checks the named crates alone (both repeatable;
a name that is not a workspace crate with a library exits 2), for a
crate-level `PLAN_EXCLUDED` key of `check_release.sh`.
`scripts/check_release.sh` (a hard failure), `ci.yml` (and through it
`publish.yml`'s CI run) and `dependency-canary.yml` run it.

Requires jq.

```sh
./scripts/check_features.sh
./scripts/check_features.sh --skip native-theme-gpui
```

## check_strict_panic.sh

Runs `cargo clippy` with the type-aware panic-prone lints denied:
`unwrap_used`, `expect_used`, `indexing_slicing`, `arithmetic_side_effects`
and the rest of the list the script holds, each category explained in its
header. The arguments are clippy's, before the `--`. `scripts/check_release.sh`
runs it on every crate's library and on the three showcase examples
(`-p native-theme-gpui --example showcase-gpui`,
`-p native-theme-iced --example showcase-iced`,
`-p native-theme-egui --example showcase-egui`), and `ci.yml`'s clippy job
runs the same calls, so the lint list lives in this one file.

Its results are kept apart from the ordinary clippy's (`-- -D warnings`):
cargo tracks the lints, which clippy gets in `CLIPPY_ARGS`, in a unit's
fingerprint but not in the hash that names its files, so the two lint sets
shared one result per workspace crate and each run replaced the other's,
compiling `native-theme-derive` and native-theme's build script again and
checking every workspace crate again, changed or not. The script sets
`debug = false` on each workspace crate (`--config
profile.dev.package.<crate>.debug=false`, the crates read from `cargo
metadata`), which is in that hash: their units get files of their own in the
same `target/debug` and `target/<triple>/debug`, no other directory, while
the dependencies keep their one build. Debug info is code generation's, so
no lint result changes. A named profile or a target directory of its own
would check every dependency a second time, locally and in CI. Exits 2 when
`jq` is missing.

Requires jq.

```sh
./scripts/check_strict_panic.sh -p native-theme-iced --lib
```

## check_docsrs.py

Documents one workspace crate the way docs.rs will, before it is published:
reads the crate's `[package.metadata.docs.rs]` through `cargo metadata` and
runs `cargo +nightly rustdoc -p <crate> --lib --locked` with its features
(`all-features`, `features`, `no-default-features`), its `cargo-args`, its
`rustc-args` as `RUSTFLAGS`, `DOCS_RS=1`, and `--cfg docsrs -D warnings` plus
its `rustdoc-args`, once for each target: `default-target` (else the first of
`targets`, else x86_64-unknown-linux-gnu) and then every other listed one,
into `target/docsrs`. A metadata key it does not know, a missing nightly
toolchain or a target whose nightly standard library is not installed fails
the check. Exits 0 when every target documented cleanly, 1 otherwise.
`scripts/check_release.sh` runs it for every workspace crate, `publish.yml`
runs it for every crate before it uploads anything, and `docs.yml` builds the
GitHub Pages documentation with it, so the site shows each crate as docs.rs
does. It does not reproduce the
dependency versions docs.rs resolves for the published package, nor docs.rs's
build limits.

Requires Python 3 and the nightly toolchain with the crates' docs.rs targets
(`rustup +nightly target add x86_64-apple-darwin x86_64-pc-windows-msvc`).

```sh
python3 scripts/check_docsrs.py native-theme
```

## check_widget_coverage.py

Checks that every widget the toolkits offer is rendered by the matching
showcase. Discovers the widgets from the dependencies' own sources through
`cargo metadata` (the iced manifest with `--features iced_aw`, so the optional
dependency is in the graph; gpui-component's widgets by their `RenderOnce` /
`IntoElement` impls and their `IntoElement` or `IntoPlot` derives, the charts
deriving `IntoPlot`), then requires each one to be either shown in the
showcase or listed with a reason in `docs/showcase-exceptions.toml`. Exits 0
when clean, 1 on a missing widget or a stale exception, 2 when a toolkit's
package is absent from the metadata, the discovery finds no widget in it, or
a file the rules read has moved upstream. `--self-test` runs those guards on
synthetic crate roots, without cargo; `scripts/check_release.sh` and `ci.yml`
run it beside the check. `scripts/check_release.sh`, `ci.yml` (and through
it `publish.yml`'s CI run), `dependency-canary.yml` and
`update_compatibility.sh run` run it, so an
upstream release that adds a widget is reported the evening it appears.

Requires Python 3.11+ (for `tomllib`); no network of its own once the registry
is populated.

```sh
python3 scripts/check_widget_coverage.py
python3 scripts/check_widget_coverage.py --self-test
```

## check_connector_parity.py

Checks that the iced, gpui and egui connectors offer the same functions under
the model's names. Lists each connector's crate-root `pub fn`s (its `lib.rs`'s
own and those a `pub use` there re-exports from one of its modules) and those
of each public module a sibling also declares (`icons`, compared as
`icons::name`), with their parameters and the `ResolvedTheme` leaves each
reads (the leaf and group types come from the core crate's model sources).
It requires every one-field accessor to be named after its leaf's path,
without the leading `defaults.` and a unit suffix
(`defaults.border.corner_radius` is `border_corner_radius`), and every
function of every class to be in every connector, under one signature (the
parameter types, a toolkit's own type read as `_`), unless
`scripts/connector_parity_exceptions.toml` lists it with its reason (`absent`
where a connector has no function of that name, `unlike` where it has one
under another signature or meaning; an entry may list several connectors and
names). A function it cannot classify needs an exception too. It prints how
many functions it put in each class and the names each class holds, per
connector, and the public modules no sibling declares, which it lists and does
not compare (the toolkits' own style builders and conversions). Exits 0 when clean, 1 on a
violation or a stale exception, 2 on an input it cannot read (a field the
model lacks, a `Resolved*` type no model source declares, a connector with
no one-field accessor found, a malformed exception file). `--self-test` runs its built-in
scenarios, a planted violation of each rule among them and an absent
function of each class; `--root DIR` reads a scratch copy of the tree
instead, to try a change; `-v` lists every function with its class and
signature. `scripts/check_release.sh` (in the `--commit` subset too) runs
the check and the self-test; `ci.yml` (and through it `publish.yml`'s CI
run) runs the check and the self-test, and `dependency-canary.yml` the check.

Requires Python 3.11+ (for `tomllib`); reads no network and runs no cargo.

```sh
python3 scripts/check_connector_parity.py
python3 scripts/check_connector_parity.py --self-test
```

## check_readme_versions.py

Checks that every README depends on this workspace's release, and on every
crate its code names. Reads the
workspace manifest's `[workspace.package] version` and, in the root README and
every workspace member's, each `native-theme` or `native-theme-*` dependency a
`toml` block states (`name = "0.7"`, `{ version = .. }`, `{ package = .., version = .. }`),
and requires it to name the version's `major.minor`. The rule compares against
the version the workspace states, so until the release commit bumps it the
READMEs name the current minor; the bump fails the check until the same
commit moves every README to the new one, and the check holds them there.
Each crate an application depends on (core, build, the three connectors,
egui-widgets) must state its own dependency, in its Quick start where it has
one, so a README that lost its block cannot pass with nothing to compare.
Exits 0 when clean, 1 on a README naming another minor or code naming a crate
the README does not depend on (below), 2 on an input it
cannot read (a version not of the form major.minor.patch, a missing root or
crate README, a crate README without its own dependency).

It also reads every README's Rust blocks (an empty or `rust…` info string, the
doctest's hidden `# ` lines included, comments and string literals left out)
and fails on a crate the code names first, the root of a `use` tree or of any
other path the block does not bind itself, that no `[dependencies]`,
`[build-dependencies]` or `[dev-dependencies]` table of a `toml` block above
that code states, nor a `cargo add` line of a shell block above it (`-` read
as `_`; `std`, `core`, `alloc`, `crate`, `self`,
`super` and the primitives need none). A README's doctests compile against
its crate's own dependencies, so a block naming `gpui::` passes there through
the connector's `gpui` while a reader who copied the Quick start has none.
The same rule reads the doc examples: every Rust block in the `///` and `//!`
doc comments under `src/` of core, build, the three connectors and
egui-widgets may name only a crate that crate's README states (in any block:
item docs have no reading order) and the crate itself (its own name, `crate`,
`self`, `super`), hidden lines included. They compile against the crate's own
dependencies as the README's blocks do, and are copied as they are.
The rule reads the code as text, not as a parse: a lowercase module that a glob
import brings in or the block declares itself reads as a crate, and a path in
a macro body as any other path; none occurs in a README today.
`--self-test` runs its built-in scenarios, a README naming another minor,
code naming an unstated crate and a doc example naming one among them; `--root DIR` reads a
scratch copy of the tree. `scripts/check_release.sh` (in the `--commit`
subset too) and `ci.yml`'s documentation job (and through it `publish.yml`'s
CI run) run the check and the self-test.

Requires Python 3.11+ (for `tomllib`); reads no network and runs no cargo.

```sh
python3 scripts/check_readme_versions.py
python3 scripts/check_readme_versions.py --self-test
```

## check_showcase_parity.py

Compares the gpui, iced and egui showcases element by element, for the
elements `docs/showcase-elements.toml` lists. Reads a directory of layout dumps
(`<kind>-<preset>-<variant>.json`, written by a showcase's `--dump-layout`) and,
beside them, the captures (`<kind>-<preset>-<variant>.png`, the content alone or
framed with a `.offset` file or `--content-offset X,Y`), and compares sizes,
positions relative to each element's parent, and the colours at the list's
sample points. Three rules, documented with their sources in the script's
docstring, cover what the renderers alone make differ: edges are compared,
and samples taken, rounded to the pixel grid (R-snap), a text run's width within max(1 px, 2 %)
(R-shape), a glyph sample within 8 and a fill or border within 1 per channel
(R-glyph). Every other difference must be listed with a reason in the
`[parity]` section of `docs/showcase-exceptions.toml`, in `[[parity.group]]`
entries that each state a `reason` once for their `keys`; a key may be scoped
to a preset (`material-3:<id>.<property>`), a preset in one variant
(`adwaita/dark:...`) or a run (`hover@...`, `menu@...`). A dump directory
holds the dumps directly or one subdirectory down; `hover=DIR` or `menu=DIR`
names the run it is (a bare `DIR` is the `rest` run). Exits 0 when clean, 1 on
an unexcepted difference, 2 on an input it cannot read. `--check-list`
validates the list against `docs/property-registry.toml` and the exceptions
against the list and the presets; `--self-test` runs its built-in scenarios.
`--merge OUT --proposal FILE...` writes to OUT the minimal scoped `[parity]`
groups the dumps need from proposed exceptions (flat `key = reason` pairs or
groups) and the current groups, one group per set of reasons, listing the
proposals it dropped and the differences none covers. With `--write` it also
rewrites the exceptions file's `[parity]` section, from its `[parity]` line to
the end, with those groups, every byte above that line unchanged; a second run
over the same inputs changes nothing. `scripts/check_release.sh` (in the
`--commit` subset too) and `ci.yml` run `--check-list` and `--self-test`,
which need no display. `generate_assets_release.sh` runs the comparison
itself over the rest run of the six captured presets: each Linux screenshot's
run writes its layout dump, its capture and the capture's `.offset` into one
directory, which the showcases record at rest whatever the pointer does, and
an unexcepted difference fails the asset run.

Requires Python 3.11+ (for `tomllib`) and Pillow.

```sh
python3 scripts/check_showcase_parity.py DUMP_DIR [hover=HOVER_DIR] [menu=MENU_DIR]
python3 scripts/check_showcase_parity.py --check-list
python3 scripts/check_showcase_parity.py --self-test
python3 scripts/check_showcase_parity.py --merge OUT.toml --proposal A.toml --proposal B.toml DUMP_DIR
python3 scripts/check_showcase_parity.py --merge OUT.toml --write --proposal A.toml DUMP_DIR [hover=HOVER_DIR] [menu=MENU_DIR]
```

## check_platform_facts_rows.py

Lists every field of `docs/property-registry.toml` that has no row in
`docs/platform-facts.md`, so no preset value is left without a facts row to
cite. A registry table is looked for in the section its `# §2.x` comment
names (`[defaults]` in all of §2.1), a field under its name without the unit
suffix, and a `border`, `font` or `icon_sizes` field part by part, the
per-side paddings under their axis rows; the script's docstring states the
whole rule. Prints one line per missing field (field, section, row name) and
exits 0 when there is none, 1 when there are some, 2 on an input it cannot
read. `--self-test` runs the rule on a synthetic registry and facts file. `check_release.sh` (both modes)
and `ci.yml`'s parity job run its self-test; the check itself is
`PLAN_EXCLUDED` until M2.6, which rewrites it to read the facts: the
registry names today's (0.7) fields, and the generated
`docs/platform-facts.md` has none of today's `§2` sections.

Requires Python 3.

```sh
python3 scripts/check_platform_facts_rows.py
python3 scripts/check_platform_facts_rows.py --self-test
```

## check_section_refs.py

Lists every numbered `§` reference in `docs/property-registry.toml`,
`docs/inheritance-rules.toml`, `docs/showcase-elements.toml` and
`docs/new-os-version-guide.md` that names no section of
`docs/platform-facts.md` (`§2` a chapter, `§2.21` a `###`, `§2.1.9` a `####`
heading); a lettered `§` or one after another `.md` file's name is another
document's and is skipped, as the script's docstring states. Prints one line
per dangling reference (file:line, reference) and exits 0 when there is none,
1 when there are some, 2 on an input it cannot read. `--self-test` runs the
rule on a synthetic document and facts file. `check_release.sh` (both modes)
and `ci.yml`'s parity job run its self-test; the check itself is
`PLAN_EXCLUDED` until M2.6, which rewrites it: `docs/inheritance-rules.toml`
is deleted, and the generated `docs/platform-facts.md` has no numbered
sections.

Requires Python 3.

```sh
python3 scripts/check_section_refs.py
python3 scripts/check_section_refs.py --self-test
```

## check_todo_citations.py

Checks the preset citations in `docs/todo.md`, which cites a preset value by
its TOML key path (`adwaita.toml` `[light.button] active_background`): each
backticked `[light…]`, `[dark…]` or `[layout…]` path, with its keys, must
name a table and keys of the presets the entry names before it, at the tree
it runs on; a preset line citation (`adwaita.toml:122`) is a finding unless
its line dates it to a commit (`at <sha>`), since a line number moves with
every edit above it. The script's docstring states the whole rule. Prints
one line per finding (file:line, citation, what is wrong) and exits 0 when
there is none, 1 when there are some, 2 on an input it cannot read.
`--self-test` runs the rule on a synthetic document and presets.
`check_release.sh` (both modes) and `ci.yml`'s parity job run its
self-test; the check itself is `PLAN_EXCLUDED` until M2.6, which rewrites
it to read the facts, as the presets it reads went at the switch.

Requires Python 3.11+ (for `tomllib`).

```sh
python3 scripts/check_todo_citations.py
python3 scripts/check_todo_citations.py --self-test
```

## check_preset_citations.py

Checks the bundled presets' definition comments. A value the specification
defines, not a fact, carries its section as its comment in place of a
citation (`font_dpi = 72  # S9.4` on `macos-*` and `ios-*`,
`animated_visuals = {}  # S4.5` on every preset but `windows-11`, ADD-A125,
ADD-A131): the comment is accepted for that key, value and presets alone. The
presets are the bundled ones, `native-theme/src/presets/`. Its citation rule
over the 0.7 presets went with them at the switch; M2.6 rewrites it to read the facts. The
script's docstring states the whole rule. Prints one line per finding
(preset:line, key path, what is wrong) and exits 0 when there is none, 1 when
there are some, 2 on an input it cannot read (a folder without presets, or a
file that is not UTF-8 or TOML). `--self-test` runs the rule on synthetic
presets. `check_release.sh` (both modes) and `ci.yml`'s parity job run it and
its self-test.

Requires Python 3.11+ (for `tomllib`).

```sh
python3 scripts/check_preset_citations.py
python3 scripts/check_preset_citations.py --self-test
```

## check_pinned_citations.py

Lists every citation of today's `docs/platform-facts.md` or the deleted
`docs/inheritance-rules.toml` by line that names no commit
(`platform-facts.md:N` where `platform-facts.md@fc6109d1:N` is meant,
the 0.8.0 plan's *Citations of today's platform-facts.md by line*): since
the switch the document is generated and the rules file is gone, so a line
number without its commit names a line of another file, or of none. It
reads every file git tracks or would track, but `docs/archive/` and the
scripts that define and convert the format
(`citations.py`, `repin_citations.py`, `import_platform_facts.py`,
`convert_inheritance_rules.py` and itself). A `:N` continuation after a
pinned citation inherits its commit; a mention with no line is no line
citation. Prints one line per finding (file:line, citation) and exits 0 when
there is none, 1 when there are some, 2 when git cannot list the files.
`--self-test` runs the rule on synthetic files. `check_release.sh` (both
modes) and `ci.yml`'s parity job run it and its self-test.

Requires Python 3.

```sh
python3 scripts/check_pinned_citations.py
python3 scripts/check_pinned_citations.py --self-test
```

## spec_catalogue.py

Reads the v0.7.1 specification's widget catalogue,
`docs/todo_v0.7.1_model-completeness-spec-widgets.md`, into its entries: the
86 typed widgets (`### <name>` under the typed sections) and the 28 keyed
components (`#### component.<key> (<platform>)`), each with its heading line,
its items (the bold labels, `**Parts:**`, `**Widget values:**`, several to a
line in many components), its parts (names, the parent a part nests in, the
*(element)* mark and note, the field list), its widget values (name and
the type the catalogue states) and its recency sets (each `<name> = [<flag>,
…]` its **Exclusive groups** item names after "Recency sets", ADD-A124, and
per platform, with the exclusive groups, `state_groups`), with
the source line of each. Variants, sizes
and flags are prose in the catalogue, so their items' text and backticked
names are given as they stand; `shared_types` reads the *Shared value types*
table. A module, imported by `check_spec_names.py`, `check_spec_entries.py`,
`generate_widgets.py` and the facts checker; its docstring states the API and
the reading rules. Run alone
it prints one line per entry (line, kind, name, parts, values); `--self-test`
runs the rules on a synthetic catalogue.

Requires Python 3.

```sh
python3 scripts/spec_catalogue.py
python3 scripts/spec_catalogue.py --self-test
```

## check_spec_names.py

Checks every name the v0.7.1 specification's catalogue and traceability use:
builds the vocabulary the core (`docs/todo_v0.7.1_model-completeness-spec.md`)
defines (state and shared flags, conditions, colour roles, tokens, element
data, part-style fields) and, through `spec_catalogue.py`, the parts, widget
values, variants and flags each catalogue entry lists, then reports each name
in a trace target or a catalogue backtick span that neither defines, notation
the specification forbids, names the core's checks removed, and platform
values written into the specification. An `input.<name>` path is one of S9's
inputs (`S9_INPUTS`, the macOS accent inputs among them) or a part of the
`input` widget. ADD-A124's names are read from the core's sketches: a
`latest <set>:<flag>` term names a recency set a targeted widget declares and
a member of it, the catalogue's recency sets hold two or more state flags,
none twice, no two of an entry ordering two flags oppositely, `PartStyle.`
paths and the `interaction` block's fields are part-style fields, and
`Radius::`, `PropertyGroup::` (and a `transitions (…)` note's groups, so
`StateLayer` and `StrokeWidth`), `RippleEffect::` and a corner's or effect's
TOML spelling name variants of their enums; a transition's `from {…}` is not
empty and names no flag twice. The motion and icon tokens are the fields of
the core's `MotionTokens` and `IconTokens` (`token.motion.color_space`,
`token.icons.animated_visuals.<visual>`; ADD-A125, ADD-A126), read from the
sketches as every other token group is. Prints one line per problem
(tab-separated: file, line, kind, name, context) and exits 0 when there is
none, 1 when there are some. `--only <key>` checks one file (`catalogue`,
`trace-macos`, ...), `--cat` and
`--traces` read copies, `--index` also writes `index.json` and `vocab.json`
beside the script; the docstring lists the options. `--self-test` runs each
rule on a synthetic core, catalogue and trace.

Requires Python 3.

```sh
python3 scripts/check_spec_names.py
python3 scripts/check_spec_names.py --self-test
```

## check_spec_entries.py

Checks the form of the v0.7.1 catalogue's entries, read through
`spec_catalogue.py`: every typed entry has every item of the entry format, in
order, one part per bullet, a root that is neither an *(element)* nor a
may-be-absent part, and a `PartStyle` field on every part bullet; every
`may_be_absent` item (typed entries and keyed components) gives each name its
own preset list and no "the preset states none" reason; the role-record and
token tables grant absence per name with presets or a criterion. Prints the
counts and one line per problem, and exits 0 when there is none, 1 when there
are some. `--self-test` runs each rule on a synthetic entry, component and
table.

Requires Python 3.

```sh
python3 scripts/check_spec_entries.py
python3 scripts/check_spec_entries.py --self-test
```

## check_spec_trace.py

Checks the v0.7.1 traceability: every row of the gap map's Appendix A
(`docs/todo_v0.7.1_model-completeness-gap-map.md`) that its platform file does
not withdraw has a line in that platform's
`docs/todo_v0.7.1_model-completeness-spec-trace-<platform>.md`, and every line
names a row of the map and a known kind or verdict, with a target where its
kind needs one and none under a verdict but `readers (R18)`; the script's
docstring states the whole rule. Prints one
line per problem (platform, row, what is wrong) and exits 0 when there is
none, 1 when there are some, 2 on a malformed input. `--self-test` runs the
rule on a synthetic map and trace.

The three spec checkers and `spec_catalogue.py` are run by hand while the
specification changes; `check_release.sh` and CI do not run them.

Requires Python 3.

```sh
python3 scripts/check_spec_trace.py
python3 scripts/check_spec_trace.py --self-test
```

## check_ledger.py

Checks the 0.8.0 ledger's triage (`## Triage` in
`docs/todo_v0.7.1_model-completeness-ledger.md`): it extracts every open item
of the five sources the plan's M0 names (`docs/todo.md`'s `- [ ]` items, the
catalogue's *Open items*, ROADMAP's *Later — the connectors* and *Also in the
0.7.x releases*, and a fixed list of the two connector plans' items) and
requires exactly one row per item, matched by source, heading and opening
words, and exactly one item per row, so a closed or moved item's row is a
finding; each destination must be one the plan names, and a row going
`after 0.8.0`, `not this plan's work` or `settled by the model` needs a
reason. On every run it also checks the *Unmeasured register*'s Ids (`U` and
a number, none shared by two rows) and that every pending fact
(`pending = "U<n>"` in `docs/facts/<platform>/`) and every pending default
size of a state model (`default_size = { pending = "U<n>" }`) names one of
its rows.
The script's docstring states the whole rule. `--final` adds the
plan's M4 gate: `docs/facts/imported/` empty, S19.2's staged file (named by
`--staged-file PATH`, which `--final` requires) gone, the `LEGACY_EVIDENCE`
list of `check_fact_citations.py` empty, no unmeasured-register row
without a decision and no S17 pending fact or declaration; the drift comparison of every measured fact has no ledger
table to read yet, so `--final` reports it as a finding until it has one.
Prints one line per finding (file:line: what) and a summary with the counts,
and exits 0 when there is none, 1 when there are some, 2 on an input it
cannot read or a wrong command line. `--ledger PATH` checks a copy of the
ledger. `--self-test` runs each rule on synthetic sources and a ledger.
`check_release.sh` runs it in the full run (`--commit` skips it) and its
self-test in both modes; the plan adds `--final` from M4.

Requires Python 3.11+ (for `tomllib`).

```sh
python3 scripts/check_ledger.py
python3 scripts/check_ledger.py --self-test
python3 scripts/check_ledger.py --final --staged-file <path>
```

## citations.py

Finds every citation in the 0.8.0 planning documents (the core, the
catalogue, the six traces, the gap maps and `docs/todo.md`) and resolves each to a project and a ref where the documents
let it: Markdown links and bare URLs to code hosts and documentation sites,
`name@sha path:L`, the abbreviations the gap maps define (`st:5327`,
`A/w/_buttons.scss:328`, `kwin src/x.cpp:140`, each with the ref its
definition in the documents states now, read from the document line that
defines it, so a re-pinned definition is read as it stands), bare `file:L`
names, `:L` continuations, `platform-facts.md`
line references (today's hand-written document, cited at `fc6109d1`), Apple documentation paths and pages named by title. Each
record gives the document, line, syntax, project, ref, path as written, the
repository path where the rules give one, the lines and a status (resolved,
heuristic, ambiguous, unresolved); the seeds file's excerpts of other
documents' cells are marked as quotes. A module, imported by
`check_sources.py` and meant for the re-pin of the citations; its docstring
states the rules in the order they apply and the API, `evidence_root` among
it (the evidence root every reader of evidence resolves, under
`check_fact_citations.py`). Existence checks (a path
carried from the citation before it, the Windows gap map's two bases, a bare
name) use this repository's files, and with `--cache` also the source cache's
checkouts. Run alone it prints the records as TSV, or with `--summary` the
citations per project; `--self-test` runs one case per syntax family.

Requires Python 3.11.

```sh
python3 scripts/citations.py --summary gap-map-kde
python3 scripts/citations.py --cache spec-widgets
python3 scripts/citations.py --self-test
```

## check_sources.py

Checks `docs/facts/sources.toml`, the pinned platform sources of the 0.8.0
model (plan D1): every `[[source]]` has the fields of its kind (a git
repository at a commit, an SDK on a runner image, a dated distribution
snapshot), no blank field, a unique `<name>@<version>` id (a commit pin's
version is its commit's first eight hex digits), full SHAs, preset ids as
generations and one pin per name and generation; a git entry's repository is
an https URL, its tag the id's version as the project tags it (`v6.8.2` for
`breeze@6.8.2`), its `sparse` folders relative paths without spaces, `..` or
a leading `-`; every `[[site]]` names its pinning method (its git source
repositories, each naming the site back as its `site`, or snapshots), and a
source's `site` is a listed site; every `[[not_pinned]]` gives its reason.
The one git entry with `evidence = true` pins the evidence repository
(specification S17 *The evidence root*): its id is `native-theme-evidence`,
it is a commit pin with `generations = []` and a `note`, and it has no
`project`, `release`, `researched` or `proof`, which it alone is waived (its
commit is this project's own, moved by its facts commits).
`update_sources.sh` refuses a file with any of these problems. Then, through `citations.py`, every project the core, the catalogue,
the traces and the gap maps cite must be a source, a site or not pinned (the
generated `docs/platform-facts.md` is not read: its citations are the facts',
whose source ids `check_facts.py` checks against `sources.toml`). Offline: it reads the documents and the repository's file
list, not the source cache. Prints the counts and one line per problem, and
exits 0 when there is none, 1 when there are some, 2 when the file cannot be
read. `--self-test` runs each rule on a synthetic file.

Requires Python 3.11.

```sh
python3 scripts/check_sources.py
python3 scripts/check_sources.py --self-test
```

## check_facts.py

Checks the form of the 0.8.0 facts (specification S17): every
`docs/facts/<platform>/*.toml` (gnome, kde, macos, ios, windows, material),
each platform's `state-models.toml` and the palette facts in
`docs/facts/palettes/`; `docs/facts/imported/` (lane input) and
`docs/facts/staged/` (S19.2's staged bases) are skipped.
A fact has a key the catalogue (through `spec_catalogue.py`) or the core (its
Rust sketches, as `check_spec_names.py` reads them) defines, without TOML's
unit suffix (`button.body.min_width`, not `min_width_px`), a value spelled
as a preset spells it (of the key's dimension where the sketches type it, an
expression in S2.2's grammar), gap-map row IDs, a `when` as S11's, citations
with the fields of their kind and a pinned source, `scheme_file` and `accent`
both or neither in `kde/` only, and the generation rule: without `preset:`,
its citations together serve each preset of its directory that exists (the
bundled `native-theme/src/presets/<id>.toml`; with none the rule asks
nothing; a measured citation serves the preset its run's `run.toml` maps its
image to, read under the evidence root, below `check_fact_citations.py`). A
state-model entry's declarations are checked against the catalogue's entry,
a palette fact's key against its file, and a bundled palette's
`[palette.sources]` and `[palette.picks]` against the palette facts and the
colours it states. ADD-A122's forms: a pending fact (`pending = "U<n>"` in
place of `value`, no citation; one a later fact of its key shadows is a
finding), a state model's `variant_products` (expanded and trimmed by the
exclusive groups and `requires_variants`; a set also listed in
`variant_sets` is a finding; the number of sets per entry is printed on
stderr), S4.1's motion keys (no dropped word or toolkit prefix, and on a
platform whose catalogue row lists every duration key, one of them), the
accent ramp's `role.global.accent_ramp.<key>`, and no key inside an enum
block. ADD-A123's forms: a text-keyed map's entry in a key as TOML writes it
(`token.text.fallbacks.per_family."Segoe Fluent Icons"`, a quoted segment one
segment; an unterminated quote is a finding), each function called with the
arity its S2.2 signature gives (`with_alpha(c, a)` beside `alpha(c, a)`), a
platform curve one of the core's `QtEasing` or `AdwEasing` members, a
`token.motion.timings.<key>` a `Timing` (S4.1's sketch), and
`requires_variants` as any one of its forms (a listed set holding none of
them is trimmed from the count). ADD-A124's forms: a state model's `recency`
sets (two or more flags of `axes.states`, none twice, no two sets ordering
two flags oppositely), with the gate's runs per order of the present flags
counted per entry on stderr and summed in the summary (at most n! per state
combination); a fact's `when.latest` (a set every state model it can hold
under declares, a flag that is a member of it and that `not_states` does not
refuse), which fact applies made order-aware (two latest flags of one set, or
latest flags no one order allows, never hold together); a transition's
`from` (stated in a single transition, never empty, no flag twice, no keyed
flag the widget lacks) and a `state_layer` transition only on a part with
`interaction`; the `interaction` block's keys; `"ellipse"` on all four
corners of one `when` or none, never with a `continuous` curve; and each
state model's `exclusive` groups and `recency` sets equal to the catalogue's
for its widget and preset, members and order (S10, S17). A `when` that can
never hold is a finding (S7's `SetMatch`, S12's exclusions): a name in both a
list and its `not_` list, two or more sizes, densities or size classes on one
axis (an element has one, and `when` takes all of them), two values of a
one-valued condition (two schemes, two presets, both contrast conditions),
two flags of one `exclusive` group or two variants of one
`exclusive_variants` group in every state model it can hold under, or
`latest` flags no one order allows. So is a map (`Vec<(K, V)>`) written as a
list of pairs, `[["k", v]]` or `[{ key = …, value = … }]`, or an empty one
written `[]`, where S11 spells a TOML table (`{}` when it has no entry); a
whole map's table is read with the map's keys and value type, so a map keyed
by text (`per_family`, `per_language`) takes its text as keys
(`{ "Segoe Fluent Icons" = ["Segoe MDL2 Assets"] }`). So is a compound field
(`Sides`, `Axes`) stated whole as `"absent"`, `"auto"` or `"none"`: it is no
`Value`, S6.1 patches it per member and S6.3 lets each member be absent, so
its members are stated (`{ top_px = "absent", … }`). ADD-A125 to ADD-A130's
forms: a state model's `axes.contexts` entry may be a nested context, an
ordered chain of two or more selectors, outermost first; a whole part takes
`"present"` too, after a fact of its key making it absent or drawn as
another; every transition states each field of the core's `Transition` (its
`engine` among them, `suspended_by_theme_change` gone), its `timing` absent
exactly on a stated zero or under `win_ui_animated_icon`, its duration absent
only on a spring or under that engine, a stated timing one its engine's code
computes (S12's per-engine curves), no `Keyed` kind under a value engine and
no `focus_move`; an AnimatedIcon transition names a visual the platform's
facts state, whose markers hold its state; a Breeze test states its three
lists, empty on a stated zero, with window conditions only (and
`focus_visible` on a focus ring); `gtk_switch` is the thumb's geometry run;
`shown` is `true` or `false`; a plain block (`TextStyle.font`, a `Tokens`
group) is stated per leaf and a plain enum block by its variant; an
`Option` of a block (`Behavior.key_repeat`) is `"none"` or a full table, and
a generation states it whole or per leaf, never both; a part's `text` is
never a reference; a drawn-text leaf (`fill`, `elision`, `wrap`, `max_lines`)
is absent only in a keyed text style, a slot or a part the catalogue's
may_be_absent marks; no fact states `token.text.font_dpi` on the Apple
presets (S9.4's own 72); and the types the catalogue's prose defines
(`ContentWidth`, `InputPlacement`, ADD-A127) type their widget values.
ADD-A131's forms: no fact states `token.icons.animated_visuals` off
`windows/` (S4.5's own `{}`); `round_pt(x)` only in an expression on a
`Font.size`, its operand a reference to a font size, and a font size rounded
in points by `round(div(<size>, 1pt))` a finding; Core Animation's
`kCAMediaTimingFunction` among the motion prefixes; a state model's
`default_size` (a size or `{ pending = "U<n>" }`) wherever `sizes` is not
empty, `default_size_by_variant` removed, a `sizes_by_variant` entry's own
`default_size` where it leaves out the widget's, a `sizes_by_context`
entry's `context` one S11 selector of the keys S10 lists (read from the
specification) matching some `axes.contexts` chain, every element taking a
size in each context where it takes one outside it and the default in force
there among them, and that default drawn in some combination of each variant
set and context the entry draws (`not_drawn` read). Offline. The
docstring states every rule and list. Prints one line per finding (file:line: what) and a summary with the counts, and
exits 0 when there is none, 1 when there are some, 2 on an input it cannot
read or a wrong command line. `--self-test` walks the specification's
example facts, each with a broken variant.

Requires Python 3.11.

```sh
python3 scripts/check_facts.py
python3 scripts/check_facts.py --self-test
```

## check_fact_citations.py

Checks what the 0.8.0 facts' citations say, against the source cache
(`update_sources.sh`; `--cache DIR` or `NATIVE_THEME_SOURCES` overrides it)
and the evidence: a path `docs/evidence/<rest>` names
`<root>/docs/evidence/<rest>`, the root being the evidence repository
([`native-theme-evidence`](https://github.com/tiborgats/native-theme-evidence))
checked out where `NATIVE_THEME_EVIDENCE` names, else in the source cache at
`<cache>/native-theme-evidence/` (`update_sources.sh --only
native-theme-evidence`), and accepted only clean (no changed, untracked or
ignored file) at the commit `sources.toml`'s `evidence = true` entry pins:
otherwise the run fails, naming the root and the pin (specification S17 *The
evidence root*; `citations.evidence_root`, which `check_facts.py`,
`check_function_vectors.py`, `generate_dynamic_type.py` and the facts gate
apply alike; a self-test's fixture holds its own `docs/evidence/`). Each
quote occurs, whitespace-normalised, in the cited
lines at the pin (a JSON page: in the string at the pointer, in the snapshot
file its `sources.toml` entry names with `file`); no `absent` name occurs
there as an identifier; per generation, every literal of the value equals a
literal of its citations' quotes, read by the cited file's notation (on
Apple's sources a point is a logical point, px) and converted as the
citation says (an absence citation holds a zero stated by omission; on a
LinearGradientBrush an angle matches only the angle its points give; C++'s
defaulted parameter is read from the quote's declaration; `ns_font_weight`
converts an AppKit `NSFontWeight` to S2.8's CSS weight by WebKit's
thresholds; XAML's character reference `&#xE974;` is its code point, equal
only to the same integer, and each entry of a XAML `<FontFamily>` list is
a whole string; a platform curve's name is the verifier's, and so are
ADD-A124's names, a ripple effect's, a transition's `from` flags); a
`GtkCss` transition list naming several curves follows its quoted CSS
`transition` list item by item, in its order (duration, delay, curve;
ADD-A131); a value the specification defines (`# S9.4`, `# S4.5`) is asked
no fact; a measured value
has the committed evidence's shape and
equals it within its tolerance, leaf by leaf, and the run's `run.toml` maps
the image to a preset its source serves; every evidence file is cited (an
image by the analysis output that names it in `input_image`), except
READMEs, `run.toml` and the legacy runs (`LEGACY_EVIDENCE`, which
`check_ledger.py --final` reads). An SDK-header citation is checked like a
cached file against its Xcode's local header copy, `<cache>/xcode@<v>/<F>/`
(the copy of the SDK its `xcode.txt` records for that framework); with no
local copy it is listed (`# sdk:`), not checked. The
docstring holds the closed lists (notations, units, conversions, evidence
path forms, `run.toml`'s fields). Prints one line per finding and a summary,
and exits 0, 1, or 2 when the cache is missing or an input cannot be read;
`check_release.sh --commit` skips it only when the cache folder is absent.
Each evidence file and cited source file is read once per run.
`--self-test` runs every notation and the example facts on a fixture cache.

Requires Python 3.11 and git.

```sh
python3 scripts/check_fact_citations.py
python3 scripts/check_fact_citations.py --self-test
```

## verify_facts.py

The verifiers' tooling (the 0.8.0 plan's "Facts, independently"), which
makes the lane-against-verifier comparison mechanical. It reads the facts
and state models as `check_facts.py` does, at a commit, through `git show`.
A fact's identity is its key (as a TOML dotted key), its `when` (empty lists
left out, name lists as sets), its `scheme_file` and its `accent`.

Each unit goes through two steps, as the plan asks: the verifier derives the
state model first, and states values only for the combinations both models
draw.

```sh
python3 scripts/verify_facts.py units --out units.toml
python3 scripts/verify_facts.py brief KD-C1 /tmp/KD-C1-model --units units.toml \
    --lane-brief lane-brief.md --no-combinations
#   the verifier writes /tmp/KD-C1-model/state-models.toml
python3 scripts/verify_facts.py compare-model KD-C1 /tmp/KD-C1-model/state-models.toml --units units.toml
python3 scripts/verify_facts.py brief KD-C1 /tmp/KD-C1-facts --units units.toml \
    --lane-brief lane-brief.md --combinations-from /tmp/KD-C1-model/model-compare.toml
#   the verifier writes /tmp/KD-C1-facts/facts.toml
python3 scripts/verify_facts.py compare KD-C1 /tmp/KD-C1-facts/facts.toml --units units.toml \
    --combinations-from /tmp/KD-C1-model/model-compare.toml
python3 scripts/verify_facts.py sample KD-C1 --compared /tmp/KD-C1-facts --seed 1
python3 scripts/verify_facts.py --self-test
```

- `units`: the facts to verify since each platform's baseline (the M1a
  tie-break commits, built in; `--since kde=<commit>` or `--since <commit>`
  overrides them). A fact is to verify when it is new, or differs in any
  field but `rows`, and is not pending. Per platform and kind (`cited`,
  `measured`) they are split by key prefix into units of near equal size,
  at most 1,000 facts each; a unit under 300 joins a neighbour where the two
  fit in 1,000 (a kind with fewer facts stays one small unit). Unit ids look
  like `KD-C1` and `MC-M4`. It writes a units file (TOML, `--out`) holding
  each unit's scope and fact identities, and prints the counts per platform
  and kind. The same commits give the same file.
- `brief <unit> <out-dir> --lane-brief <lane-brief.md>`: the verifier's
  folder. It holds `README.md`, `keys.toml`, `combinations.toml`, `pins.toml`
  (the platform's `sources.toml` entries with their source-cache paths),
  `catalogue.md` (the catalogue's entry format, shared value types, shared
  flags and selector notation, and each widget's whole entry), `spec.md`
  (the specification's normative sections S1 to S21, each found by its
  heading; not S22, the additions register, nor the traces, gap maps or
  `platform-facts.md`, which state platform values) and `facts-format.md`
  (the lane brief's format sections). `--no-combinations` leaves the combinations out
  (the state-model step); `--combinations-from` writes only those a
  `compare-model` result agrees on (the facts step). A measured unit also
  gets copies of its cited evidence files, with each run's `run.toml` and
  `analysis/` scripts, taken from the evidence root (a clean checkout at the
  pin). The folder holds no lane value or citation. The leak check proves
  it: every value and citation string of the unit's facts is looked for in
  the folder (the evidence copies left out). Each hit is printed with its
  class: `identity`, `specification`, `sources` or `template` (the file's
  own input holds the string too, so the hit is unavoidable), `trivial`
  (under four characters, or a keyword), `normative-value` (a platform
  value, such as a citation, a literal or a font or icon name, that the
  specification's or catalogue's own text holds: a leak to report), or
  `leak`. A leak or a normative value exits 1. So that the brief passes,
  it elides each of the unit's platform values and citations in its copies
  of the normative text (`spec.md`, `catalogue.md`, `facts-format.md`) with
  a visible marker, `⟦elided: a value of this unit⟧` or `⟦elided: a
  citation of this unit⟧`, keeping the text around it; its `README.md`
  lists each elision by file, line and class, never the string. Trivial
  strings and those of the unit's identities stay.
- `compare-model <unit> <verifier-state-models.toml>`: the lane's state
  models of the unit's widgets against the verifier's, entry by entry. It
  lists every declaration that differs (axes, flags, variants, sizes,
  `not_drawn`, exclusive groups, variant sets and products, recency) with
  the members only one side states. It also lists the state combinations
  and variant sets that `check_facts.py`'s expansion gives one side only.
  Each combination of the unit is then agreed, or refused where the
  verifier's model cannot draw it and the lane's can. A combination the
  lane's own model cannot draw either is kept, marked `lane_model_refuses`.
  It writes `model-compare.toml`.
- `compare <unit> <verifier-facts.toml>`: the unit's facts (with
  `--combinations-from`, those of the agreed combinations) against the
  verifier's, by identity. It normalises colour spellings, number
  formatting, expression spacing and equivalent selectors and key spellings,
  and lists each normalisation with the facts it changed and the pairs that
  agree only through it. A pair that still differs, where one side holds an
  expression or a reference, is resolved: both values are evaluated in the
  combination, under each scheme (and preset) it leaves open, the references
  read from the lane's own facts, S2.2's arithmetic, `alpha`, `with_alpha`,
  `mix` and `pick` evaluated. A pair equal in every case is **equivalent by
  resolution**, its own class, listed with the resolved values and never
  counted as agreed. `surface.`, `element.` and the platform functions are
  not evaluated, and `absent` never equals a transparent colour. It writes `tiebreak.toml` (the counts, a summary
  line in the ledger's *Verification rates* form, and every disagreement and
  one-sided fact with both values and citations) and `agreed.toml` into
  `--out` (default: the verifier file's folder).
- `sample <unit> --compared <dir> --seed <n>`: the audit sample of the
  agreed facts: `max(--min 10, ceil(--rate 0.05 × agreed))` facts, picked
  by a SHA-256 order of the seed and the identity, so one seed always gives
  the same sample. It writes `sample.toml`, which records the seed.

`--self-test` runs every subcommand on fixtures in a temporary git
repository. It covers the units' counts and splitting, the brief's files,
the specification's sections (S17's heading among them, S22 left out) and
its leak check (a planted citation is caught, a platform value in the
catalogue elided and listed, an unelided one reported), the model comparison and the two-step flow, the
resolution (equivalent, equal under one scheme only, `absent` against a
transparent colour, an unevaluated function), each normalisation, an identity mismatch, one-sided facts and the sample's
determinism. It runs in `check_release.sh` and in CI's parity job.

Requires Python 3.11 and git.

## import_platform_facts.py

Imports today's `docs/platform-facts.md` rows (as `fc6109d1` holds the
document, read through `git show`, so the switch's generated document does
not change them) into
`docs/facts/imported/<platform>.toml`, the 0.8.0 plan's lane input: one
`[[row]]` per row and platform cell of Chapter 2, and per row of Chapter 1's
section of that platform, with the property, the line, the cell's text as
written (its values and their cited sources) and the evidence files it links.
Where a gap-map row of that platform names the cell (a
`platform-facts.md@fc6109d1:N` citation, a section and property or lines,
or the map's table of the platform's "(none)" cells), `rows` lists the rows
and `targets` their trace lines' targets; elsewhere `targets = []` and a
note says why. It writes no key of its own. Not facts: the facts gate and
`check_fact_citations.py` do not read `imported/`, and `check_facts.py`
skips it; a lane removes each entry it converts, and the folder is empty at
M4, when the script is deleted. Prints the counts per platform. `--check`
exits 1 when the files differ from a fresh run (until a lane removes its
first entry); `--self-test` runs every rule on a fixture document, gap maps
and traces. Not in `check_release.sh` (the plan's table of scripts).

Requires Python 3.11 and a git checkout holding `fc6109d1`.

```sh
python3 scripts/import_platform_facts.py
python3 scripts/import_platform_facts.py --check
python3 scripts/import_platform_facts.py --self-test
```

## convert_inheritance_rules.py

Converts the `[uniform]` and `[per_platform]` rules of
`docs/inheritance-rules.toml` into S19.2's staged file,
`docs/facts/staged/inheritance-bases.toml`, the rules and today's presets
read as `fc6109d1` holds them (through `git show`, so their removal at the
switch does not change the file): per preset that exists today
(`ios-26` has none; the community presets become palettes), each field
today's preset leaves to a rule under a scheme, with the S2.2 reference the
rule implies over the new model's paths (`= role.window.background`,
`= widget.view.fill`), its rule and line. The paths come only from the widget
catalogue's *Today's fields* (a widget's item, the `defaults.` table) and are
checked against its parts and fields; where a clause names no one path, the
source is another widget's or the rule restates the target's own field,
`target` or `value` is `""` with a note, and `mapping` quotes the clauses
used. `[wrong_safety_nets]` is not converted (an entry it names says so,
quoting its text, whose `platform-facts.md:N` it writes at `fc6109d1`).
Each unit's preset step uses the file and the gate checks it; it is removed
at M4 (`check_ledger.py --final --staged-file`), with the script. Prints the
counts per preset. `--check` exits 1 when the file differs from a fresh run;
`--self-test` runs every rule on fixture rules, catalogue and presets. Not
in `check_release.sh`.

Requires Python 3.11 and a git checkout holding `fc6109d1`.

```sh
python3 scripts/convert_inheritance_rules.py
python3 scripts/convert_inheritance_rules.py --check
python3 scripts/convert_inheritance_rules.py --self-test
```

## check_function_vectors.py

Holds, documents and checks the vector-table format of the 0.8.0 model's
platform functions, placement strategies and colour interpolations
(specification S2.2, S13, S2.7): `native-theme/tests/vectors/<function>.tsv`,
`place.<strategy>.tsv` and `interpolate_color.<member>.tsv`,
tab-separated, with `# <name>: <value>` header lines (the route, which is
`public_api`, `observable_behaviour`, `compiled_source`, `measured` or
`published`; the `sources.toml` entries, the generations, the generator,
`# inputs:` and `# outputs:` with each column's type, a `# tolerance:`
per measured output), a column header row, and rows whose cells are
spelled by their column's type, with `cite` and `quote` (published) or
`evidence` (measured) where the route needs them. Checks that every `pub`
or `pub(crate)` function of the `platform` module
(`native-theme/src/platform/`), every
platform function of S2.2's table that is implemented and, once the
`placement` module exists, every strategy, and, once
`transitions::interpolate_color` exists, every `ColorInterpolation` member
(its table `from`, `to`, `t` with an undershoot and an overshoot row, and the
colour; ADD-A131) has a table and a `#[test] fn vectors_<fn>` in
`native-theme/tests/platform_vectors.rs`; that
every platform function a preset calls is implemented (one only facts call
is reported as not yet implemented); and the generations rule: the rows'
generator pins, citations or evidence serve every generation that calls the
function with the rows' enum operands. The model's own arithmetic
(`round_pt` among it) has unit tests, not tables. Offline; the docstring states the format and every rule.
Prints one line per finding, `# ` lines for what is informational, and a
summary; exits 0, 1, or 2 on an input it cannot read. `--self-test` runs
each rule on fixture tables, Rust snippets, facts and presets. Enters
`check_release.sh` at M1a's code tasks (the plan's table of scripts).

Requires Python 3.11.

```sh
python3 scripts/check_function_vectors.py
python3 scripts/check_function_vectors.py --self-test
```

## generate_widgets.py

Generates the 0.8.0 model's typed widgets from the widget catalogue
(specification S5, S6.1): for each widget of the units
`scripts/widget_units.toml` enables, `native-theme/src/widgets/gen_<widget>.rs`
(the `ThemeParts` derive's input, a `#[part(...)]` per part with its fields
and marks; the widget's values struct; the entry's own enums), and
`gen_index.rs` (`WidgetKind`, `Variant`, `TypedPart`, `WidgetsPatch`,
`WidgetValuesPatch`, the shared value types the units use, the widgets' slots
and the registry's list), each through `rustfmt`. It reads the catalogue
through `spec_catalogue.py` only, never touches the module's hand-written
`mod.rs` and `tests.rs`, and leaves out the values a unit defers to a later
unit (the button's `Placement` values, until M3's placement unit). The units
file lists the enabled units, the deferrals with their units, and the
hand-written units, enabled or not; a deferral to an enabled unit is an
input error. `--check` writes nothing and exits 1 when a generated file
differs or is stale (in `check_release.sh`); a construct the generator does
not handle exits 2 naming it. `--self-test` runs the reading rules and the
refusals on a synthetic catalogue. Deleted once the catalogue is archived,
after which the generated code is the source.

Requires Python 3.11 and rustfmt.

```sh
python3 scripts/generate_widgets.py
python3 scripts/generate_widgets.py --check
python3 scripts/generate_widgets.py --self-test
```

## check_facts_trace.py

Checks that every live trace line of the checked units (the specification's
traceability, S20) is carried by a fact or a state-model entry of its
platform's `docs/facts/` directory that lists its row in `rows` and whose key
lies on the line's target (each line of a row separately), or carries a
verdict, on the platforms
whose presets exist (the bundled `native-theme/src/presets/`).
`--enabled` checks the units of `scripts/widget_units.toml` (the slice:
`button`, the colour roles and the slice's tokens, with button's `Placement`
values deferred and `token.behavior.scroll` staged; the same slice while the
file does not exist),
`--unit <unit>` one unit, and no option every trace line. A target lies in
a unit by its leading paths (braces expanded), and a rule naming a context
widget waits until that widget is enabled. Prints one line per finding
(platform, row, target, what), per platform how many of the checked lines
and rows the facts already carry, and a summary; exits 0, 1, or 2 on an
input it cannot read or an unknown unit. `--self-test` runs each rule on
fixture gap maps, traces, facts and units files. `--enabled` enters
`check_release.sh` at M1a; the script is deleted at M6.

Requires Python 3.11.

```sh
python3 scripts/check_facts_trace.py --enabled
python3 scripts/check_facts_trace.py --unit slice
python3 scripts/check_facts_trace.py
python3 scripts/check_facts_trace.py --self-test
```

## generate_platform_facts.py

Generates the platform-facts document from the verified facts
(`docs/facts/<platform>/*.toml` and `docs/facts/palettes/`; `imported/`
and `staged/` are no facts): one section per unit (the colour roles, the
tokens, the inputs, then the widget catalogue's sections), one table per
widget, colour-role set or token group, a row per key and combination, and
per preset the value of the fact that applies there (the last in file
order) with its citations. Writes `--out PATH`, by default
`docs/platform-facts.md`, which its output has replaced since the switch:
the document is generated and never edited (the hand-written one, which the
planning documents cite by line, is the one `fc6109d1` holds). `--check`
exits 1 when PATH differs from a fresh run; `check_release.sh` (both modes
and `--quick`) runs it and the self-test. Offline. `--self-test` compares a
fixture's document with the expected one and runs `--check`.

Requires Python 3.11.

```sh
python3 scripts/generate_platform_facts.py
python3 scripts/generate_platform_facts.py --check
python3 scripts/generate_platform_facts.py --self-test
```

## generate_preset.py

Generates a draft of a preset's TOML (specification S11) from its verified
facts, as far as they transcribe: `[declare]` from the state models (S10),
the facts at rest as bases, every other fact as a rule, facts with an equal
`when` and target grouped so that the rules keep the facts' order and the
last matching rule is the last matching fact (S17). It leaves out, and
counts by reason, pending facts, keys the enabled units do not hold, `read`
facts, and what S11 cannot spell from a fact alone; S19.2's staged bases
and the keyed namespaces of `[declare]` are left for hand-writing. Prints
on stderr the transcription share per widget and per unit (the 0.8.0
plan's checkpoint D6), the rule count, the pending facts' register ids and
a round trip: the draft read back gives every combination the value of
the fact that applies there, a map compared as a map (a `Value`'s too,
`Font.axes`), a fact's list of pairs as its table, each `when`'s minimal
combination at the size its element holds by S10's rule (ADD-A131). A
pending default size of a state model is listed in `[pending]` by its
declaration's path, never written in `[declare]`. `--write` points each
stale citation of the hand-written blocks (a `[declare]` entry's, a `read`
value's, the generation's source) at the fact naming its subject and lists
one it cannot find; `--check` fails on either. Exits 1 on a round-trip
mismatch or such a citation, 2 on an input it cannot read. `--self-test`
compares a fixture's draft with the expected one and runs each rule.
Offline; not in `check_release.sh`; about four minutes for an iOS preset.

Requires Python 3.11.

```sh
python3 scripts/generate_preset.py adwaita --out adwaita.toml
python3 scripts/generate_preset.py --self-test
```

## update_sources.sh

Fills the source cache, `/run/media/tibi/work/cargo-build/native-theme-sources/`
(`NATIVE_THEME_SOURCES` overrides it): every git entry of
`docs/facts/sources.toml` is checked out at its commit into `<cache>/<id>/`,
by a shallow fetch of that commit (or of its tag, where a server refuses a
fetch by SHA), as a partial clone of the entry's `sparse` folders only where
it lists them (androidx, AOSP, WebKit, the documentation repositories). A
checkout already at its commit is skipped, so a run after an edit of
`sources.toml` fetches only what changed; a new checkout is built beside its
final folder and renamed once at its commit. A checkout counts as current
when it is at its commit with exactly the entry's `sparse` folders (a changed
list fetches it again); a changed tracked file in it is reset to the commit.
`--seed DIR` fetches a complete entry from the checkout `DIR/<repository
name>` instead, when that checkout's HEAD is the commit (its tracked files
only), `--jobs N` sets how many fetches run at once (4, at least 1), `--only
ID` (repeatable) fetches or verifies the named git entries alone.
`--verify` runs offline and exits 1 unless every entry's checkout is at its
commit with its sparse folders and no tracked file changed; both modes report
cache folders no entry names (not with `--only`). The evidence entry's
checkout, `<cache>/native-theme-evidence/`, is what this, the main checkout's
readers of evidence read: a moved pin is fetched into it (only the new runs
download), and both modes count an untracked or ignored file in it as a
problem, since the readers refuse it. The entry is skipped, `--verify`
included, where `NATIVE_THEME_EVIDENCE` is set and in any worktree of this
repository (`git rev-parse --git-dir` differs from `--git-common-dir`), so a
worktree, whose pin may differ, never moves the checkout under a gate running
in the main checkout: it sets `NATIVE_THEME_EVIDENCE` to a clone of its own. Before anything else both modes check
`sources.toml` with `check_sources.py`'s form rules and exit 2 on a problem,
and every path the script writes or removes must resolve to a folder directly
inside the cache. The SDK and distribution entries are not fetched; an SDK
entry's folder, `<cache>/xcode@<v>/`, holds the headers `os-probe.yml`'s macOS
job copied (put there by hand from its artifact), which `--verify` checks
against the SHA-256 lines of its `xcode.txt` and does not report as stale.

Requires git and Python 3.11; network access unless `--verify`.

```sh
./scripts/update_sources.sh
./scripts/update_sources.sh --verify
./scripts/update_sources.sh --only native-theme-evidence
```

## repin_citations.py

Re-pins the 0.8.0 planning documents' citations of pinned platform sources
(the core, the catalogue, the six traces and the gap maps; plan M0, *Re-pin*)
to the release set of `docs/facts/sources.toml`. For each citation
`citations.py` finds there, it takes the commit the citation was read at (the
ref the citation or its abbreviation states; for a bare file name, the one
source its gap-map row's platform file cites it from), reads the cited file at
that commit (a checkout under `--from`) and at the pin (the source cache), and
maps the cited lines by a line diff of the whole file: **unchanged** (the same
commit), **moved** (every line found with the same text), **changed**,
**vanished**, **present** (a file mention found at the pin), **unmappable**
(no commit, several projects, no path) or **error** (no checkout of the
commit). `--report PATH.tsv` writes one row per citation with its old and new
lines, and stdout has the counts per status and document. `--apply` rewrites
the moved, unchanged and present citations in place, line numbers and ref in
the document's own syntax (an abbreviation's definition line gets the pin's
commit and tag), and lists the lines that still name an old commit; changed,
vanished and unmappable citations stay as they are for a reader to correct.
A citation already at its pin is unchanged, and the abbreviations' refs are
read from the documents' definitions, so a second run over re-pinned
documents changes nothing; a bare citation whose ref comes from another
citation is rewritten only with it. `--docs DIR` works on a copy of the
documents. Git runs offline. Exits 0 when
no changed, vanished or error citation remains, 1 when some do, 2 on
unreadable input. `--self-test` runs every status and rewrite on two commits
of a fixture repository.

Requires git and Python 3.11.

```sh
python3 scripts/repin_citations.py --from <checkouts> --report repin.tsv
python3 scripts/repin_citations.py --from <checkouts> --apply
python3 scripts/repin_citations.py --self-test
```

## update_icons.sh

Re-downloads every bundled SVG under `native-theme/icons/` from the
provenance manifest `native-theme/icons/SOURCES.toml` (one rule per set, one
per-file exception per set), so a refresh or an addition is reproducible.

```sh
./scripts/update_icons.sh                      # refresh every file to the manifest refs
./scripts/update_icons.sh add lucide battery   # add a Lucide icon by its upstream name
./scripts/update_icons.sh add material lan     # add a Material Symbols icon
```

Requires Python 3.11+ (for `tomllib`) and network access. The exit code only
says every file was reachable; byte-identity with upstream is established by
running the script and confirming `git diff --stat native-theme/icons` is
empty. After a run,
`cargo test -p native-theme --test icon_sources` checks the manifest and
`cargo build -p native-theme` regenerates the by-name tables from the
directories (`native-theme/build.rs`).

## run_in_runner_env.sh

Runs `cargo <arguments>` with each test binary in the environment of CI's
Linux runner, so a test that holds only on this host's desktop, icon themes or
fonts fails here before it fails on CI. Cargo builds as usual; its target
runner for the host's target starts each test binary through the script,
which clears the environment and gives the binary what an `ubuntu-latest` job
has:

- no desktop: no `XDG_CURRENT_DESKTOP`, display or session bus;
- an empty home under `target/runner-env/home`;
- an empty `target/runner-env/config` as the one `XDG_CONFIG_DIRS` entry, so
  a host's `/etc/xdg/kdeglobals` does not reach the KDE reader (the egui
  showcase's `native` feature among its users), as the runner has none;
- a `PATH` of `fc-match`, `sh`, `sleep`, `true` and the toolchain's `cargo`
  and `rustc` only, none of the desktop readers `gsettings`, `xrdb`, `xrandr`
  or `xfconf-query`;
- the runner's icon themes alone, Adwaita, hicolor, Humanity and ubuntu-mono
  (the packages firefox's GTK 3 pulls in; no breeze), as the one
  `XDG_DATA_DIRS` entry;
- the runner image's fonts, the Ubuntu 24.04 font packages and fontconfig
  configuration of image 20261012.334, named by `FONTCONFIG_FILE`, which both
  fontdb and `fc-match` read.

The packages are pinned by SHA-256 and downloaded from `snapshot.ubuntu.com`
(else the archive's pool) into `target/runner-env/debs` once; the environment
is built from them again when the list changes. The script's header gives the
evidence for each part. `scripts/check_release.sh` runs the three showcases'
tests through it on Linux, in every feature set `ci.yml`'s test-connectors
job tests them with: the egui showcase with its defaults, with none and with
all (whose `native` reads the desktop, which the environment hides), the iced
showcase with its defaults and with `iced_aw,watch,native`, the gpui showcase
with its defaults, with `native,watch` and on GPUI Fast with
`gpui-fast,native,watch`. A test leg already builds each but the gpui
showcase's defaults, which that leg compiles again. Both modes run them all
but the GPUI Fast leg, which `--commit` skips, as it skips the gpui
connector's other GPUI Fast tests. It runs `run_in_runner_env.sh --prepare`
first, which builds the environment and runs nothing: where a package is not
cached with its SHA-256 and neither server delivers it, one or both out of
reach (no connection, a timeout, a transfer closed early) or answering
transiently (5xx, 408, 429), the script exits 75, and check_release skips the
legs with a warning that the showcase tests were not run in CI's
environment (CI runs them on the runner itself). Anything else fails, in
both, naming its cause: a 4xx answer from both servers (a pin that has
rotted; the message names the package and both answers; one server's 4xx
with the other out of reach is still 75, its message saying the pin may have
rotted), a file that cannot be
written or extracted, a downloaded package whose SHA-256 differs. curl gives
up on a connection after 10 s and on a stalled transfer after 30 s.

Requires curl, sha256sum, bsdtar (libarchive) and fontconfig's `fc-match`;
network access on the first run.

```sh
./scripts/run_in_runner_env.sh test -p native-theme-egui --example showcase-egui
./scripts/run_in_runner_env.sh --prepare
```
