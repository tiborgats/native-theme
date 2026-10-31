# Scripts

Visual asset generation and release automation for native-theme.

All scripts are run from the project root. Each asset is generated into the
crate it documents so relative paths in per-crate READMEs resolve everywhere
(offline, on GitHub, and in the published crate on crates.io / docs.rs):

- Spinner GIFs → `native-theme/docs/assets/`
- gpui screenshots + theme-switching GIF → `connectors/native-theme-gpui/docs/assets/`
- iced screenshots + theme-switching GIF → `connectors/native-theme-iced/docs/assets/`
- egui screenshots + theme-switching GIF → `connectors/native-theme-egui/docs/assets/`
- Workspace crate-relations diagram → `docs/assets/` (workspace-level)

## generate_assets_local.sh

Master orchestration script. Runs all five asset generators in sequence:
spinner GIFs, iced screenshots, gpui screenshots, egui screenshots, and
theme-switching GIFs.

```sh
./scripts/generate_assets_local.sh
```

## generate_gifs_spinners.py

Generates looping spinner GIF animations from the bundled Material and Lucide
SVG icons into `native-theme/docs/assets/`. Each GIF shows the spinner centered
on a styled card background (24 rotation frames, 42ms/frame).

Also supports `--theme-switching` mode to assemble pre-captured PNG frames into
an animated theme-switching GIF, each frame at its captured size (the size of
the screenshots beside it; frames of differing sizes are refused). Callers pass
the explicit per-connector output path via `--theme-switching-output`.

Requires: Python 3, Pillow, ImageMagick 7

```sh
# Spinner GIFs (default output: native-theme/docs/assets/)
python3 scripts/generate_gifs_spinners.py

# Theme-switching GIF from captured frames
python3 scripts/generate_gifs_spinners.py --theme-switching /path/to/frames \
    --theme-switching-output connectors/native-theme-iced/docs/assets/theme-switching.gif
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
each set into a looping theme-switching GIF via `generate_gifs_spinners.py`.
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

## generate_qt_color_vectors.py

Writes `native-theme/src/kde/qt_color_vectors.rs`, the Qt test vectors the
KDE reader's colour helpers are checked against (`kde::colors`' tests):
`Qt.tint`, run by Qt's own QML engine, and `KColorUtils::mix` and Kirigami's
`ColorUtils::linearInterpolation`, transcribed over Qt's `QColor`. Each table
holds the Breeze inputs, edge cases and seeded random inputs, among them
inputs on which a naive float port gives another colour. The output is
deterministic; run it again only to change the tables.

Requires: Python 3 and PySide6 6.11.2 (Qt 6.11.2), the version the tables were
generated with; it refuses another, and a Qt build whose `QColor::toRgb` does
not fuse `1.0f - (s * f)` into one multiply-add, as the reference build does.

```sh
python3 -I scripts/generate_qt_color_vectors.py
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
dbus-monitor, dbus-send, Python 3.11+, Pillow, ImageMagick 7, network access

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
data documents' checks (the five on the registry, the inheritance rules, the
presets and `docs/todo.md`, then the 0.8.0 plan's: the ledger's triage, the
pinned sources and their cache, the facts and their citations), docs and the archive index, then the docs.rs emulation,
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

```sh
./scripts/check_release.sh --commit
./scripts/check_release.sh
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
and `ci.yml`'s parity job run it and its self-test.

Requires Python 3.

```sh
python3 scripts/check_platform_facts_rows.py
python3 scripts/check_platform_facts_rows.py --self-test
```

## check_wrong_safety_nets.py

Checks `docs/inheritance-rules.toml`'s `[wrong_safety_nets]` against the
file's rules: a `[uniform]` rule marked `# see wrong_safety_nets` has an entry
with the rule's source, a `[uniform]` rule that has an entry carries the mark,
and an entry that is no `[uniform]` rule is inherited by `[font_inheritance]`
or `[border_inheritance]`, or states `removed`; the script's docstring states
the whole rule. Prints one line per finding and exits 0 when there is none, 1
when there are some, 2 on an input it cannot read. `--self-test` runs the
rule on a synthetic rules file. `check_release.sh` (both modes) and
`ci.yml`'s parity job run it and its self-test.

Requires Python 3.11+ (for `tomllib`).

```sh
python3 scripts/check_wrong_safety_nets.py
python3 scripts/check_wrong_safety_nets.py --self-test
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
and `ci.yml`'s parity job run it and its self-test.

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
`check_release.sh` (both modes) and `ci.yml`'s parity job run it and its
self-test.

Requires Python 3.11+ (for `tomllib`).

```sh
python3 scripts/check_todo_citations.py
python3 scripts/check_todo_citations.py --self-test
```

## check_preset_citations.py

Checks the `docs/platform-facts.md` citations in the presets' comments. A
citation (`platform-facts.md:N`, a range `N-M` or further `, :K` lines, each
with its `(§x)`) cites the key its comment names (`no K:`, `K not stated:`,
`K ← X`, ...), or else the value straight below the comment, and its row
must name that key or value: the row's backticked name must be the key path
inside its widget, `<sub-table>.<key>` without `_px`, `_ms` or `_pt`
(`[light.button.border] color` is `border.color`), and the row must lie in
the section the comment states; a citation of several rows for a value also
needs each row of the value's section to name a value it covers. A line
number drifts with every edit above it, which is how 52 of material-3's
comments in each variant came to cite a neighbouring row. The stated
cross-row citations pass by rules (a disabled colour citing
`disabled_opacity`, an unchecked or hovered fill citing the widget's
`background_color`, ..., each only where the section has no row of the
value's own), and the rest by the script's `EXCEPTIONS` list, keyed by
preset, key path and cited row, each with its reason; an exception no
citation uses is a finding. The script's docstring states the whole rule.
Prints one line per finding (preset:line, key path, cited line, what is
wrong) and the counts of every preset, and exits 0 when there is none, 1
when there are some, 2 on an input it cannot read (not UTF-8 or TOML, or a
preset citing `platform-facts.md:N` where the rule reads no citation).
`--self-test` runs the rule on a synthetic facts file and preset.
`check_release.sh` (both modes) and `ci.yml`'s parity job run it and its
self-test.

Requires Python 3.11+ (for `tomllib`).

```sh
python3 scripts/check_preset_citations.py
python3 scripts/check_preset_citations.py --self-test
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
names are given as they stand. A module, imported by `check_spec_names.py` and
`check_spec_entries.py` and meant for the widget-declaration generator and the
facts checker; its docstring states the API and the reading rules. Run alone
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
empty and names no flag twice. Prints one line per problem
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
(`pending = "U<n>"` in `docs/facts/<platform>/`) names one of its rows.
The script's docstring states the whole rule. `--final` adds the
plan's M4 gate: `docs/facts/imported/` empty, S19.2's staged file (named by
`--staged-file PATH`, which `--final` requires) gone, the `LEGACY_EVIDENCE`
list of `check_fact_citations.py` empty, no unmeasured-register row
without a decision and no S17 pending fact; the drift comparison of every measured fact has no ledger
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
catalogue, the six traces, the gap maps, `docs/platform-facts.md` and
`docs/todo.md`) and resolves each to a project and a ref where the documents
let it: Markdown links and bare URLs to code hosts and documentation sites,
`name@sha path:L`, the abbreviations the gap maps define (`st:5327`,
`A/w/_buttons.scss:328`, `kwin src/x.cpp:140`, each with the ref its
definition in the documents states now, read from the document line that
defines it, so a re-pinned definition is read as it stands), bare `file:L`
names, `:L` continuations, `platform-facts.md`
line references, Apple documentation paths and pages named by title. Each
record gives the document, line, syntax, project, ref, path as written, the
repository path where the rules give one, the lines and a status (resolved,
heuristic, ambiguous, unresolved); the seeds file's excerpts of other
documents' cells are marked as quotes. A module, imported by
`check_sources.py` and meant for the re-pin of the citations; its docstring
states the rules in the order they apply and the API. Existence checks (a path
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
`update_sources.sh` refuses a file with any of these problems. Then, through `citations.py`, every project the core, the catalogue,
the traces, the gap maps and `docs/platform-facts.md` cite must be a source, a
site or not pinned. Offline: it reads the documents and the repository's file
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
new model's, `native-theme/src/next/presets/<id>.toml`; today's presets do
not count, and with none the rule asks nothing). A
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
list of pairs, `[["k", v]]` or `[{ key = …, value = … }]`, where S11 spells a
TOML table. Offline. The docstring states every rule and list. Prints
one line per finding (file:line: what) and a summary with the counts, and
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
and `docs/evidence/`: each quote occurs, whitespace-normalised, in the cited
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
measured value
has the committed evidence's shape and
equals it within its tolerance, leaf by leaf, and the run's `run.toml` maps
the image to a preset its source serves; every evidence file is cited (an
image by the analysis output that names it in `input_image`), except
READMEs, `run.toml` and the legacy runs (`LEGACY_EVIDENCE`, which
`check_ledger.py --final` reads). SDK-header citations are listed, not
checked: `os-probe.yml`'s macOS job checks them at their Xcode. The
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

## import_platform_facts.py

Imports today's `docs/platform-facts.md` rows (at `fc6109d1`) into
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

Requires Python 3.11.

```sh
python3 scripts/import_platform_facts.py
python3 scripts/import_platform_facts.py --check
python3 scripts/import_platform_facts.py --self-test
```

## convert_inheritance_rules.py

Converts the `[uniform]` and `[per_platform]` rules of
`docs/inheritance-rules.toml` (at `fc6109d1`) into S19.2's staged file,
`docs/facts/staged/inheritance-bases.toml`: per preset that exists today
(`ios-26` has none; the community presets become palettes), each field
today's preset leaves to a rule under a scheme, with the S2.2 reference the
rule implies over the new model's paths (`= role.window.background`,
`= widget.view.fill`), its rule and line. The paths come only from the widget
catalogue's *Today's fields* (a widget's item, the `defaults.` table) and are
checked against its parts and fields; where a clause names no one path, the
source is another widget's or the rule restates the target's own field,
`target` or `value` is `""` with a note, and `mapping` quotes the clauses
used. `[wrong_safety_nets]` is not converted (an entry it names says so).
Each unit's preset step uses the file and the gate checks it; it is removed
at M4 (`check_ledger.py --final --staged-file`), with the script. Prints the
counts per preset. `--check` exits 1 when the file differs from a fresh run;
`--self-test` runs every rule on fixture rules, catalogue and presets. Not
in `check_release.sh`.

Requires Python 3.11.

```sh
python3 scripts/convert_inheritance_rules.py
python3 scripts/convert_inheritance_rules.py --check
python3 scripts/convert_inheritance_rules.py --self-test
```

## check_function_vectors.py

Holds, documents and checks the vector-table format of the 0.8.0 model's
platform functions and placement strategies (specification S2.2, S13):
`native-theme/tests/vectors/<function>.tsv` and `place.<strategy>.tsv`,
tab-separated, with `# <name>: <value>` header lines (the route, which is
`public_api`, `observable_behaviour`, `compiled_source`, `measured` or
`published`; the `sources.toml` entries, the generations, the generator,
`# inputs:` and `# outputs:` with each column's type, a `# tolerance:`
per measured output), a column header row, and rows whose cells are
spelled by their column's type, with `cite` and `quote` (published) or
`evidence` (measured) where the route needs them. Checks that every `pub`
or `pub(crate)` function of the `platform` module
(`native-theme/src/next/platform/`, `src/platform/` after the switch), every
platform function of S2.2's table that is implemented and, once the
`placement` module exists, every strategy has a table and a
`#[test] fn vectors_<fn>` in `native-theme/tests/platform_vectors.rs`; that
every platform function a preset calls is implemented (one only facts call
is reported as not yet implemented); and the generations rule: the rows'
generator pins, citations or evidence serve every generation that calls the
function with the rows' enum operands. The model's own arithmetic has unit
tests, not tables. Offline; the docstring states the format and every rule.
Prints one line per finding, `# ` lines for what is informational, and a
summary; exits 0, 1, or 2 on an input it cannot read. `--self-test` runs
each rule on fixture tables, Rust snippets, facts and presets. Enters
`check_release.sh` at M1a's code tasks (the plan's table of scripts).

Requires Python 3.11.

```sh
python3 scripts/check_function_vectors.py
python3 scripts/check_function_vectors.py --self-test
```

## check_facts_trace.py

Checks that every live trace line of the checked units (the specification's
traceability, S20) is carried by a fact or a state-model entry of its
platform's `docs/facts/` directory that lists its row in `rows` and whose key
lies on the line's target (each line of a row separately), or carries a
verdict, on the platforms
whose presets exist (the new model's `native-theme/src/next/presets/`).
`--enabled` checks the units of `scripts/widget_units.toml` (until it
exists, the slice: `button`, the colour roles and the slice's tokens, with
button's `Placement` values deferred and `token.behavior.scroll` staged),
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
`docs/platform-facts.generated.md`, and never today's hand-written
`docs/platform-facts.md`, which its output replaces at the switch;
`--check` exits 1 when PATH differs from a fresh run (in `check_release.sh`
from the switch). Offline. `--self-test` compares a fixture's document with
the expected one and runs `--check`.

Requires Python 3.11.

```sh
python3 scripts/generate_platform_facts.py --out docs/platform-facts.generated.md
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
the fact that applies there. Exits 1 on a round-trip mismatch, 2 on an
input it cannot read. `--self-test` compares a fixture's draft with the
expected one and runs each rule. Offline; not in `check_release.sh`.

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
only), `--jobs N` sets how many fetches run at once (4, at least 1).
`--verify` runs offline and exits 1 unless every entry's checkout is at its
commit with its sparse folders and no tracked file changed; both modes report
cache folders no entry names. Before anything else both modes check
`sources.toml` with `check_sources.py`'s form rules and exit 2 on a problem,
and every path the script writes or removes must resolve to a folder directly
inside the cache. The SDK and distribution entries are not fetched.

Requires git and Python 3.11; network access unless `--verify`.

```sh
./scripts/update_sources.sh
./scripts/update_sources.sh --verify
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
