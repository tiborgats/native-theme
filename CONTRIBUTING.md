# Contributing to native-theme

Thank you for your interest in contributing.

## Quick start

1. Fork and clone the repository
2. Install Rust stable (workspace MSRV **1.88.0**, edition **2024**; the gpui,
   egui and egui-widgets crates need **1.95**, so `cargo test --workspace` needs
   1.95.0 or newer)
3. Verify the workspace builds and tests pass:

   ```bash
   cargo test --workspace
   ```

## Pre-commit gate — `./scripts/check_release.sh`

The script has two modes. Before every commit, run the per-commit subset:

```bash
./scripts/check_release.sh --commit
```

On the last commit of every fix phase and before the release commit, run the
full pre-release check:

```bash
./scripts/check_release.sh
```

This is the canonical quality gate. In full, it runs, in order: a TODO/FIXME scan
of every `.rs` file outside `target/`, the panic-pattern scan
(`scripts/check_panic_patterns.py` and its `--self-test`) of each workspace
member's `src/` outside its test modules and test-only files (not `build.rs`,
`examples/`, `tests/` or `benches/`), per-crate `cargo check --all-targets`,
the feature-combination check (`scripts/check_features.sh`: every crate's
library with no default features, with each feature alone and with all
features — for `native-theme-gpui` all but `gpui-fast` — failing on a
warning), the cross-target checks on `x86_64-pc-windows-gnu` and `x86_64-apple-darwin` where the target is
installed, and for windows-gnu the `x86_64-w64-mingw32-gcc` linker too (a
missing target or linker skips the check with a warning the summary repeats)
(`native-theme` with `system-icons` alone, and with all features
under `RUSTFLAGS="-D warnings"`, and for x86_64-apple-darwin `native-theme-egui`
with all targets and features), `cargo fmt --all` (formats in place; CI uses
`--check`), per-crate `cargo clippy --all-targets -- -D warnings`
(`native-theme` also with its Linux features,
`kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts`;
`native-theme-gpui` with `native,watch`, and with `gpui-fast,native,watch`), the strict
panic lints on library code (`clippy::unwrap_used`, `clippy::indexing_slicing`
and the rest of the type-aware set; `native-theme-gpui` with `native,watch` and
with `gpui-fast,native,watch`),
per-crate `cargo test` (`native-theme-gpui` with `native,watch`, and `native-theme`
also with its Linux features, where the tests that need an installed Breeze or
Adwaita theme run on a host that has them; a test leg that passes no test
fails, except the OS probe's while its test list is empty), `native-theme`'s ignored stem-index test (`--ignored`, ~30-50 s,
on Linux, against the installed Breeze and Adwaita themes; on a host with none
of them it checks nothing and passes), the gpui connector's tests with no
default features and on GPUI Fast (`--features gpui-fast,native,watch --all-targets`; the
`--all-targets` leaves the doctests out, since each is a crate of its own that
names gpui-pre as `gpui`, not GPUI through gpui-kit, and fails on GPUI Fast), the
iced, egui and egui-widgets crates' other configurations (tests with no default features and, for iced, with
`iced_aw,watch,native`, for egui and egui-widgets with all features; clippy with `iced_aw,watch,native`,
for egui and egui-widgets with all features; for egui also that its graph holds
one `skrifa`),
`cargo build --examples` for the crates that have examples, the widget-coverage
check (`scripts/check_widget_coverage.py`, Python 3.11+), the connector-parity
check (`scripts/check_connector_parity.py` and its `--self-test`), the
README-version check (`scripts/check_readme_versions.py` and its `--self-test`), the showcase-parity
comparator's static modes (`scripts/check_showcase_parity.py --check-list` and
`--self-test`, which needs Pillow), per-crate
`cargo doc --no-deps --document-private-items` with `RUSTDOCFLAGS="-D warnings"`,
so the private items' docs are held too (`native-theme` with its Linux
features, plus its public API with no features; and `native-theme-iced`,
`native-theme-egui` and `native-theme-egui-widgets` with all features), the archive-index check (every `docs/archive/*.md` named in its README), the docs.rs
emulation, `cargo package` on every
publishable crate, `cargo semver-checks` against each crate's crates.io release
(`native-theme-gpui` with `--default-features`),
`cargo audit`, `cargo outdated`, the visual assets'
PROVENANCE stamp and the connectors' compatibility claims
(`scripts/update_compatibility.sh check`). The script prints
a single-line status per check, and a failing check's captured output; only the
semver checks are soft (`⚠`), since approved breaks are expected before a
release (a semver warning is resolved by the version number or by undoing the
change); every other check is hard, and a hard failure (`❌`) exits non-zero and
must be fixed. It needs jq and installs nothing: a missing `cargo-audit`,
`cargo-outdated` or `cargo-semver-checks` fails the full run with its install
command.

`--commit` runs every one of those checks except the release-tag check, the gpui
connector's tests on GPUI Fast, `native-theme`'s ignored stem-index test and the
release-only sections:
the docs.rs emulation (`scripts/check_docsrs.py`), packaging,
`cargo semver-checks`, `cargo audit`, `cargo outdated`, and the PROVENANCE and
compatibility stamp checks. Each of
them prints `skipped (--commit mode)`, and the summary names them as skipped,
not passed.

The script runs the checks of the CI pipeline on the local platform, plus the
packaging, semver, outdated-dependency and archive-index checks CI does not run.
On Linux it tests `native-theme` with no features and with all its Linux
features at once. CI also runs the Linux features one leg at a time, which
builds code no local leg builds (the reader choice's `kde`-without-`portal`
arm, for one), and the macOS and Windows test legs: the script checks and
lints those targets where they are installed, but cannot run their tests.
Both are left to CI.

A scheduled workflow, `dependency-canary.yml`, runs nightly at 18:10 UTC
(and on demand from the Actions tab) with every dependency updated on a
throwaway lockfile.
Its failure means the newest upstream set broke the crates, not that `main`
is broken; CI on the committed `Cargo.lock` is the gate for merging.

## Individual checks

If you want to run pieces manually instead of the full script:

```bash
# Format
cargo fmt --all --check

# Lint each crate (CI runs clippy per-crate, not workspace-wide)
cargo clippy -p native-theme --all-targets
cargo clippy -p native-theme-build --all-targets
cargo clippy -p native-theme-gpui --all-targets --features native,watch
cargo clippy -p native-theme-iced --all-targets
cargo clippy -p native-theme-egui --all-targets
cargo clippy -p native-theme-egui-widgets --all-targets

# Tests
cargo test --workspace

# Documentation
RUSTDOCFLAGS="-Dwarnings" cargo doc --workspace --no-deps
```

CI sets `RUSTFLAGS=-Dwarnings`, so any clippy warning is treated as an error.

## Project structure

| Crate | Path | Description |
|-------|------|-------------|
| `native-theme` | `native-theme/` | Core theme model, presets, platform readers, icons, animations |
| `native-theme-build` | `native-theme-build/` | Build-time code generation for custom icon roles |
| `native-theme-derive` | `native-theme-derive/` | Proc-macro crate (internal; re-exported via `native-theme`) |
| `native-theme-gpui` | `connectors/native-theme-gpui/` | gpui toolkit connector |
| `native-theme-iced` | `connectors/native-theme-iced/` | iced toolkit connector |
| `native-theme-egui` | `connectors/native-theme-egui/` | egui toolkit connector |
| `native-theme-egui-widgets` | `connectors/native-theme-egui-widgets/` | egui widgets the egui connector cannot give egui's own (switch, slider knob, spinner, segmented control, links, radio dot, progress-bar outline, expander) |

## Feature flags

The core `native-theme` crate has several feature flags:

- **Platform readers:** `kde`, `portal`, `windows`, `macos`, `linux` (meta: kde+portal), `native` (meta: all)
- **Icons:** `system-icons`, `material-icons`, `lucide-icons`
- **Watching:** `watch` (runtime theme change notifications)
- **Rendering:** `svg-rasterize` (resvg-backed SVG→RGBA rasterization)
- **Fonts:** `system-fonts` (face matching against the installed fonts, `fonts::system_face`)

Platform-specific features only compile on their target OS. See
[`native-theme/Cargo.toml`](native-theme/Cargo.toml) for the full list and
dependency gates.

## Commit message conventions

Use conventional-commit-style prefixes. Common forms seen in this project:

- `feat(scope): <summary>` — new user-facing feature or API addition
- `fix(scope): <summary>` — bug fix
- `refactor(scope): <summary>` — internal restructuring with no API change
- `docs(scope): <summary>` — docs-only change
- `test(scope): <summary>` — tests only
- `chore(scope): <summary>` — build, tooling, deps
- `ci(workflow): <summary>` — GitHub Actions changes

The `scope` is usually the crate name (`native-theme`, `gpui`, `iced`,
`readme`, `changelog`), a subsystem (`icons`, `api`, `watch/kde`), or during
active phase-based work a phase identifier (`93-01`, `94-02`). When in doubt,
match the style of recent commits:

```bash
git log --oneline -30
```

Pre-1.0, breaking changes in minor versions are allowed and expected — just
ensure they appear in `CHANGELOG.md` under the `[Unreleased]` section.

## Pull request workflow

1. Fork the repository, branch off `main`
2. Keep commits atomic (one concern per commit)
3. Ensure `./scripts/check_release.sh` passes on the last commit of your branch
4. Open a PR with a clear title matching the conventional-commit style
5. Reference related issues or design docs in the PR description
6. CI runs automatically; results appear on the PR page

## Where decisions live

Design-level choices are captured in files under `docs/`:

- [`docs/platform-facts.md`](docs/platform-facts.md) — canonical platform-native values (macOS Sonoma/Tahoe, Windows 11, KDE Plasma, GNOME, Material 3), cited to official sources
- [`docs/property-registry.toml`](docs/property-registry.toml) — semantic color and property registry
- [`docs/inheritance-rules.toml`](docs/inheritance-rules.toml) — widget-field inheritance rules driving the resolution pipeline
- [`docs/todo.md`](docs/todo.md) — the backlog: the open items and follow-ups, each tagged by milestone; closed ones move to [`docs/archive/todo_closed.md`](docs/archive/todo_closed.md)

A change too large for one entry there gets a plan: `docs/todo_<topic>-plan.md`,
usually with a `-rationale.md` (why each decision was taken) and a `-spec.md`
(what to build) beside it. The `docs/todo_*.md` files in `docs/` are the live
plans. Archiving is every plan's last task: once it is implemented, the plan,
its rationale and its spec move to [`docs/archive/`](docs/archive/), keeping
their names, and the index table in
[`docs/archive/README.md`](docs/archive/README.md) gains their row;
`scripts/check_release.sh` warns about an archived file the table does not name.

Do not invent platform-native values. When the authoritative reference is
silent, flag it explicitly rather than guessing.

## Regenerating visual assets

Screenshots, theme-switching GIFs, spinner GIFs, and the workspace
crate-relations diagram are generated by the scripts in `scripts/`. See
[`scripts/README.md`](scripts/README.md) for the full pipeline.

```bash
./scripts/generate_assets_release.sh        # full screenshots + GIFs pipeline (needs gh, spectacle and KDE Plasma 6's kscreen-doctor and qdbus6, dbus-monitor/dbus-send, Python 3.11+ with Pillow, ImageMagick 7 and network access; see scripts/README.md)
./scripts/generate_diagrams.sh    # workspace crate-relations SVG (needs Graphviz; fontconfig, python-fonttools and the Fuzzy Bubbles font to embed the "Your app" label font)
```

Each asset is generated into the crate it documents, so relative paths in
per-crate READMEs resolve on GitHub, on crates.io, and offline.

Regeneration cannot be forgotten: `generate_assets_release.sh` ends by writing
`docs/assets/PROVENANCE.toml`, which records the workspace version, the
commit, the OS versions of the CI runners that captured the macOS and Windows
screenshots, and a hash over the git object ids of every path that feeds the
showcases (crate sources, presets, icon bundles, `Cargo.lock`, the capture
scripts, the screenshots workflow). `scripts/check_release.sh` recomputes that
hash at HEAD; a mismatch is a warning while the CHANGELOG entry for the
version still says Unreleased and a hard failure once the entry is dated. The
publish workflow's CI gate runs the same check on the pushed tag, so a tag
whose assets were captured from other sources is refused before anything is
uploaded. Docs-only changes leave the hash untouched and need no recapture.

## Release order

1. Bump the workspace version in `Cargo.toml`, commit, push.
2. `./scripts/generate_assets_release.sh`, review the images, commit the assets together
   with `docs/assets/PROVENANCE.toml`.
3. Date the CHANGELOG entry and set its compare link; commit as
   `chore(release): vX.Y.Z`.
4. `./scripts/check_release.sh` on that commit (the asset check is hard now).
5. Fast-forward `main` to it and push; CI runs on the exact commit.
6. `git tag vX.Y.Z` on that commit and push the one tag by name. The publish
   workflow runs its CI gate, then uploads the crates in dependency order.
7. Create the GitHub release page from the tag with the CHANGELOG section as
   notes, then check the docs.rs build pages.

## License

By contributing, you agree your contributions will be triple-licensed under
**MIT OR Apache-2.0 OR 0BSD**.
