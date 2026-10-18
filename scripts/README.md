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
holds: `capture_showcases KIND FEATURES OUTPUT ENTRY...` builds the showcase
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
`Cargo.lock`, the capture
scripts, the screenshots workflow) into
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
panic lints, the tests, the connectors' other configurations, the examples,
widget coverage, connector parity, the README versions, the showcase-parity comparator's static modes, the
data documents' three checks, docs and the archive index, then the release-only sections: the docs.rs emulation,
packaging, `cargo semver-checks`, `cargo audit`, `cargo outdated`, and the
visual-asset and compatibility stamp checks. Prints one line per check and a summary;
exits 1 on a hard failure. A test leg that passes no test fails: a filter or a
cfg that matches nothing still exits 0. `CONTRIBUTING.md` lists every check.

With `--commit` it runs the per-commit subset: the release-tag check, the gpui
connector's tests on GPUI Fast, native-theme's ignored stem-index test and the
release-only sections are skipped, each
printing `skipped (--commit mode)`, and the summary names them.
The subset runs before every commit; the full run on the last commit of every
fix phase and before the release commit. An unknown argument exits 2;
`--help` prints the usage.

```sh
./scripts/check_release.sh --commit
./scripts/check_release.sh
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
failure at the end, and 2 when `jq` is missing. `scripts/check_release.sh` (a
hard failure), `ci.yml` (and through it `publish.yml`'s CI run) and
`dependency-canary.yml` run it.

Requires jq.

```sh
./scripts/check_features.sh
```

## check_strict_panic.sh

Runs `cargo clippy` with the type-aware panic-prone lints denied:
`unwrap_used`, `expect_used`, `indexing_slicing`, `arithmetic_side_effects`
and the rest of the list the script holds, each category explained in its
header. The arguments are clippy's, before the `--`. `scripts/check_release.sh`
runs it on every crate's library and on the gpui showcase example
(`-p native-theme-gpui --example showcase-gpui`), and `ci.yml`'s clippy job
runs the same calls, so the lint list lives in this one file.

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
dependency is in the graph), then requires each one to be either shown in the
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
`toml` block states (`name = "0.6"`, `{ version = .. }`, `{ package = .., version = .. }`),
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
