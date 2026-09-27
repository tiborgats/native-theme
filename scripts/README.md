# Scripts

Visual asset generation and release automation for native-theme.

All scripts are run from the project root. Each asset is generated into the
crate it documents so relative paths in per-crate READMEs resolve everywhere
(offline, on GitHub, and in the published crate on crates.io / docs.rs):

- Spinner GIFs → `native-theme/docs/assets/`
- gpui screenshots + theme-switching GIF → `connectors/native-theme-gpui/docs/assets/`
- iced screenshots + theme-switching GIF → `connectors/native-theme-iced/docs/assets/`
- Workspace crate-relations diagram → `docs/assets/` (workspace-level)

## generate_assets_local.sh

Master orchestration script. Runs all four asset generators in sequence:
spinner GIFs, iced screenshots, gpui screenshots, and theme-switching GIFs.

```sh
./scripts/generate_assets_local.sh
```

## generate_gifs_spinners.py

Generates looping spinner GIF animations from the bundled Material and Lucide
SVG icons into `native-theme/docs/assets/`. Each GIF shows the spinner centered
on a styled card background (24 rotation frames, 42ms/frame).

Also supports `--theme-switching` mode to assemble pre-captured PNG frames into
an animated theme-switching GIF. Callers pass the explicit per-connector output
path via `--theme-switching-output`.

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
it to render, then captures the active window.

On macOS/Windows, use the showcase's built-in `--screenshot` flag instead.

Builds and runs the showcase with `--features iced_aw`, so the captures show
the same widget set the coverage check gates.

Requires: spectacle (KDE)

```sh
./scripts/generate_screenshots_iced.sh
```

## generate_screenshots_gpui.sh

Same as `generate_screenshots_iced.sh` but for the gpui showcase. Includes
`--icon-theme` for freedesktop themes that need an explicit icon theme name.

Requires: spectacle (KDE)

```sh
./scripts/generate_screenshots_gpui.sh
```

## generate_gifs_theme_switching.sh

Captures 4 theme presets from both iced and gpui showcases, then assembles
each set into a looping theme-switching GIF via `generate_gifs_spinners.py`.

Produces:
- `connectors/native-theme-iced/docs/assets/theme-switching.gif`
- `connectors/native-theme-gpui/docs/assets/theme-switching.gif`

Requires: spectacle (KDE), Python 3, Pillow

```sh
./scripts/generate_gifs_theme_switching.sh
```

## generate_diagrams.sh

Regenerates SVG diagrams from Mermaid `.mmd` sources in `docs/assets/`.
Uses `mermaid-cli` via `npx` — no global install required.

Run after editing any `.mmd` file. The generated `.svg` is checked into
git so contributors don't need Node installed to view diagrams.

Requires: Node.js ≥ 18, network access (first run downloads `mmdc` on demand)

```sh
./scripts/generate_diagrams.sh
```

## generate_assets_release.sh

Full pre-release asset pipeline. Triggers the CI screenshots workflow for
macOS/Windows, generates all local Linux assets while CI runs, then downloads
the CI artifacts into the correct per-connector `docs/assets/` directory
based on artifact name, and finally writes `docs/assets/PROVENANCE.toml`
through `update_provenance.sh`. Refuses to start while HEAD is unpushed or a path
the assets depend on has uncommitted changes, because the captures must come
from the commit CI builds.

Its last step runs `update_compatibility.sh run` (below), which needs the network,
runs both connectors' tests, clippy and documentation on the newest upstream
releases, runs `check_widget_coverage.py` (Python 3.11+), and rewrites
`docs/COMPATIBILITY.toml` and the Verified line in both connector READMEs. A
connector that fails on the newest set fails the script.

Requires: gh CLI (authenticated), spectacle, Python 3.11+, Pillow,
ImageMagick 7, network access

```sh
./scripts/generate_assets_release.sh
```

## update_provenance.sh

Provenance stamp for the visual assets. `write` records the workspace
version, the commit, and a SHA-256 over the git object ids of every path
that feeds the showcases (crate manifests and sources, presets, icon
bundles, `Cargo.lock`, the capture scripts, the screenshots workflow) into
`docs/assets/PROVENANCE.toml`. `check` recomputes the hash at HEAD and exits
non-zero with a message when the stamp is missing or the sources differ;
`scripts/check_release.sh` and the crates.io workflow's CI gate call it, so a
release whose assets were captured from other sources is refused. Docs-only
changes do not alter the hash. `verify-clean` fails when a stamped path has
uncommitted changes; `hash` prints the current value.

```sh
./scripts/update_provenance.sh check
```

## update_compatibility.sh

The upstream versions each connector has been verified against, recorded in
`docs/COMPATIBILITY.toml` and stated on each connector README's Verified line.

- `run [gpui|iced]` (both when none is named) refuses to start while the
  connector's `Cargo.toml`, `src`, `examples` or `tests` have uncommitted
  changes, then runs `cargo update` on the connector's upstream family
  (gpui-base, gpui-component, gpui-kit, gpui-kit-assets, gpui-pre; or iced,
  iced_aw, iced_core, iced_test, iced_widget) and that connector's gates on
  the result with `--locked`: tests (for iced also `--no-default-features`
  and `--features iced_aw`), clippy with `-D warnings` (for iced with
  `--all-features`), documentation with `RUSTDOCFLAGS="-D warnings"` (for
  iced also `--all-features`) and `check_widget_coverage.py`. Only when every gate passes does it write the
  versions the lockfile resolved, the commit and a sources hash into the
  stamp and rewrite the README's Verified line between its
  `<!-- compat:begin -->` / `<!-- compat:end -->` markers. `Cargo.lock` is
  restored on exit, pass or fail. Needs the network, and Python 3.11+ for the
  coverage script.
- `check` exits 0 when the stamp exists and each connector's sources hash at
  HEAD matches the recorded one, and 1 with a message otherwise. It reads git
  and the stamp only, no network. `scripts/check_release.sh` runs it: a warning
  while the CHANGELOG entry says "Unreleased", a failure once it is dated.
- `hash <gpui|iced>` prints a connector's sources hash at HEAD: a SHA-256
  over the git object ids of its `Cargo.toml`, `src`, `examples` and `tests`.

There is no verb that writes the stamp without the run.

```sh
./scripts/update_compatibility.sh run iced
./scripts/update_compatibility.sh check
```

## check_features.sh

Checks that every workspace crate builds, without warnings, in every feature
combination the gate names: for each member of `cargo metadata --no-deps`
with a library target, `cargo check -p <crate> --lib` with
`--no-default-features`, with `--no-default-features --features <F>` for each
feature in its `[features]` table except `default`, and with
`--all-features` — the coverage of `cargo hack check --each-feature`, without
the extra tool. A combination fails when it does not build or when cargo
prints a `warning:` line; cargo caps lints for dependencies, so those come
from the workspace's own crates, and no `RUSTFLAGS` is set, so the
dependencies are not rebuilt. Colour is turned off inside the script, and so
is cargo's future-incompatibility notice, a `warning:` line about a
dependency rather than a workspace crate. Prints one line per combination
with an excerpt under each failure; exits 1 when any fails, naming each
failure at the end, and 2 when `jq` is missing. `scripts/check_release.sh` (a
hard failure), `ci.yml`, `publish.yml` and `dependency-canary.yml` run it.

Requires jq.

```sh
./scripts/check_features.sh
```

## check_widget_coverage.py

Checks that every widget the toolkits offer is rendered by the matching
showcase. Discovers the widgets from the dependencies' own sources through
`cargo metadata` (the iced manifest with `--features iced_aw`, so the optional
dependency is in the graph), then requires each one to be either shown in the
showcase or listed with a reason in `docs/showcase-exceptions.toml`. Exits 0
when clean, 1 on a missing widget or a stale exception, 2 when a toolkit's
package is absent from the metadata. `scripts/check_release.sh`, `ci.yml`,
`publish.yml`, `dependency-canary.yml` and `update_compatibility.sh run` run it, so an
upstream release that adds a widget is reported the evening it appears.

Requires Python 3.11+ (for `tomllib`); no network of its own once the registry
is populated.

```sh
python3 scripts/check_widget_coverage.py
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
